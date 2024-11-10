#[macro_use]
extern crate rocket;

use std::collections::HashMap;

use rand::prelude::IteratorRandom;
use rand::prelude::SliceRandom;
use rand::Rng;
use rocket::request::{FromParam, FromRequest};
use rocket::serde::json::Json;
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

pub struct Games {
    games: HashMap<String, Game>,
}

#[derive(Serialize, Deserialize)]
struct ResponseGame {
    code: String,
    pid: Uuid,
}

#[post("/games")]
fn create_game(games: &State<Games>) -> Json<ResponseGame> {
    // Make game code using RANDOM_WORDS plus random number, use 4 random words and 2 random
    // numbers at the end of 2 of the words randomly
    let mut rng = rand::thread_rng();
    let mut words = RANDOM_WORDS.choose_multiple(&mut rng, 4);
    let rand_indices = (0..4).choose_multiple(&mut rng, 2);
    let mut code = String::new();
    for (i, word) in words.enumerate() {
        if i > 0 {
            code.push_str("-");
        }
        code.push_str(word);
        if rand_indices.contains(&i) {
            code.push_str(&rng.gen_range(1..=9).to_string());
        }
    }

    let pid = Uuid::new_v4();
    let game = Game {
        code,
        pid,
    };

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
        .manage(Games {
            games: HashMap::new(),
        })
        .mount("/api", routes![create_game, get_game])
}
