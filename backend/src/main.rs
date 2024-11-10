#[macro_use]
extern crate rocket;

use std::collections::HashMap;
use std::sync::mpsc;

use rand::prelude::IteratorRandom;
use rand::prelude::SliceRandom;
use rand::Rng;
use rocket::serde::json::Json;
use rocket::State;
use rocket::tokio::sync::Mutex;
use rocket_ws::Message;
use rocket_ws as ws;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
pub struct PlayerData {
    pid: Uuid,
    name: String,
    host: bool,
}

pub struct Player {
    data: PlayerData,
    stream_outbox: Option<Box<mpsc::Sender<String>>>,
}

pub struct Game {
    code: String,
    players: Vec<Player>,
}

impl Game {
    fn get_player_data(&self) -> Vec<PlayerData> {
        let mut res = Vec::new();
        for i in &self.players {
            res.push(i.data.clone());
        }
        res
    }
}

const RANDOM_WORDS: [&str; 11] = [
    "defenestration",
    "tribulation",
    "romania",
    "tubular",
    "tibia",
    "teeming",
    "blissful",
    "ream",
    "showcase",
    "reticle",
    "curtail",
];

pub struct Games(Mutex<HashMap<String, Game>>);

#[derive(Serialize, Deserialize)]
struct ResponseGame {
    code: String,
    pid: Uuid,
    players: Vec<PlayerData>,
}

#[post("/games?<name>")]
async fn create_game(name: &str, games: &State<Games>) -> Json<ResponseGame> {
    // Make game code using RANDOM_WORDS plus random number, use 4 random words and 2 random
    // numbers at the end of 2 of the words randomly
    let words = RANDOM_WORDS.choose_multiple(&mut rand::thread_rng(), 4);
    let rand_indices = (0..4).choose_multiple(&mut rand::thread_rng(), 2);
    let mut code = String::new();
    for (i, word) in words.enumerate() {
        if i > 0 {
            code.push_str("-");
        }
        code.push_str(word);
        if rand_indices.contains(&i) {
            code.push_str(&rand::thread_rng().gen_range(1..=9).to_string());
        }
    }

    let pid = Uuid::new_v4();
    // TODO: Setup websocket & add to Player struct
    let game = Game {
        code: code.clone(),
        players: vec![Player {
            data: PlayerData {
                pid,
                name: name.to_string(),
                host: true,
            },
            stream_outbox: None,
        }],
    };

    let resp = ResponseGame {
        code: code.clone(),
        pid,
        players: game.get_player_data(),
    };

    games.0.lock().await.insert(code.clone(), game);

    Json(resp)
}

#[put("/games/<code>?<name>")]
async fn join_game(code: &str, name: &str, games: &State<Games>) -> Option<Json<ResponseGame>> {
    let mut games = games.0.lock().await;
    let game = games.get_mut(code)?;

    let pid = Uuid::new_v4();
    game.players.push(Player {
        data: PlayerData {
            pid,
            name: name.to_owned(),
            host: false,
        },
        stream_outbox: None,
    });

    Some(Json(ResponseGame {
        code: game.code.to_string(),
        pid,
        players: game.get_player_data(),
    }))
}

#[post("/games/<code>/start")]
async fn start_game(code: &str, games: &State<Games>) {
    let mut games = games.0.lock().await;
    let game = games.get_mut(code).unwrap();

    broadcast(game, "start".to_owned());
}

/*async fn ws_take(mut stream: rocket_ws::stream::DuplexStream) -> String {
    use rocket::futures::StreamExt;
    stream
        .next()
        .await
        .unwrap()
        .unwrap()
        .into_text()
        .unwrap()
}*/

async fn ws_push(stream: &mut rocket_ws::stream::DuplexStream, msg: String) {
    use rocket::futures::SinkExt;
    let _ = stream.send(Message::Text(msg)).await.unwrap();
}

#[get("/games/<code>/<pid>/ws")]
async fn ws_channel(ws: ws::WebSocket, code: &str, pid: &str, games: &State<Games>) -> ws::Channel<'static> {
    let mut games_unlocked = games.0.lock().await;
    let game = games_unlocked.get_mut(code).unwrap();
    let (outbox_tx, outbox_rx) = mpsc::channel::<String>();
    let actual_pid = Uuid::parse_str(pid).unwrap();
    let player_index = game
        .get_player_data()
        .iter()
        .position(|data| data.pid == actual_pid)
        .unwrap();
    game.players[player_index].stream_outbox = Some(Box::new(outbox_tx));
    ws.channel(move |mut stream| Box::pin(async move {
        for outbox_msg in outbox_rx {
            ws_push(&mut stream, outbox_msg).await;
        }
        Ok(())
    }))
}

fn broadcast(game: &Game, msg: String) {
    for player in &game.players {
        if let Some(outbox) = &player.stream_outbox {
            outbox.send(msg.clone()).unwrap();
        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(Games(HashMap::new().into()))
        .mount("/api", routes![
            create_game, join_game,
            start_game,
            ws_channel,
        ])
}
