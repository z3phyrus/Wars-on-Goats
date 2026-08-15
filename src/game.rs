use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::{Arc, Mutex}};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Player {
    pub id: String,
    pub name: String,
}

#[derive(Clone, Debug)]
pub struct GameState {
    pub players: Arc<Mutex<HashMap<String, Player>>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct JoinRequest {
    pub name: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct JoinResponse {
    pub player_id: String,
    pub welcome: String,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            players: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn register_player(&self, name: String) -> Player {
        let player = Player {
            id: uuid::Uuid::new_v4().to_string(),
            name,
        };

        self.players.lock().unwrap().insert(player.id.clone(), player.clone());
        player
    }
}
