#[macro_use]
extern crate rocket;

use std::collections::HashMap;
use rand::prelude::IteratorRandom;
use rand::prelude::SliceRandom;
use rand::Rng;
use rocket::http::Status;
use rocket::outcome::IntoOutcome;
use rocket::request::{FromParam, FromRequest};
use rocket::serde::json::Json;
use rocket::tokio::sync::Mutex;
use rocket::State;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct Player {
    pid: Uuid,
    name: String,
}

pub struct Game {
    code: String,
    players: Vec<Player>,
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

// impl<'r> FromParam<'r> for Game {
//     type Error = ();
//
//     fn from_param(param: &'r str) -> Result<Self, Self::Error> {
//         let id = Uuid::parse_str(param).map_err(|_| ())?;
//         Ok(Game {
//             id,
//             code: "Hello, world!".to_string(),
//             players: Vec::new(),
//         })
//     }
// }

impl<'r> FromRequest<'r> for Game {
    type Error = ();

    async fn from_request(request: &'r rocket::Request<'_>) -> rocket::request::Outcome<Self, Self::Error> {
        // Lookup from state Games
        let code = request.param::<String>(0).unwrap().unwrap();

        request.rocket().state::<Games>()
            .map(|&games| -> impl Future<Game> { async {
                games.0.lock().await.get(&code)
            } })
            .or_forward(Status::InternalServerError)
    }
}

pub struct Games(Mutex<HashMap<String, Game>>);

#[derive(Serialize, Deserialize)]
struct ResponseGame {
    code: String,
    pid: Uuid,
}

#[post("/games/<name>")]
async fn create_game(name: &str, games: &State<Games>) -> Json<ResponseGame> {
    // Make game code using RANDOM_WORDS plus random number, use 4 random words and 2 random
    // numbers at the end of 2 of the words randomly
    let mut words = RANDOM_WORDS.choose_multiple(&mut rand::thread_rng(), 4);
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
            pid,
            name: name.to_string(),
        }],
    };

    games.0.lock().await.insert(code.clone(), game);

    Json(ResponseGame { code, pid })
}

// #[get("/games/<code>")]
#[get("/games")]
fn get_game(/*game: &Game*/) -> Json<ResponseGame> {
    Json(ResponseGame {
        // code: game.code,
        code: "Hello, world!".to_string(),
        pid: Uuid::new_v4(),
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(Games(HashMap::new().into()))
        .mount("/api", routes![create_game, get_game])
}
