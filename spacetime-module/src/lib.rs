use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PlayerRecord {
    pub id: String,
    pub name: String,
    pub room_id: Option<String>,
    pub team: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct RoomRecord {
    pub id: String,
    pub name: String,
    pub is_open: bool,
    pub max_players: u32,
    pub player_ids: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct MatchRecord {
    pub id: String,
    pub room_id: String,
    pub status: MatchStatus,
    pub blue_team_score: u32,
    pub red_team_score: u32,
    pub started_at: Option<String>,
    pub winner: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum MatchStatus {
    Lobby,
    InProgress,
    Completed,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct JoinRoomRequest {
    pub room_id: String,
    pub player_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct StartMatchRequest {
    pub room_id: String,
    pub organizer_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct UpdateMatchStateRequest {
    pub match_id: String,
    pub blue_team_score: u32,
    pub red_team_score: u32,
    pub status: MatchStatus,
}

pub struct ModuleState {
    pub players: Vec<PlayerRecord>,
    pub rooms: Vec<RoomRecord>,
    pub matches: Vec<MatchRecord>,
}

impl ModuleState {
    pub fn new() -> Self {
        Self {
            players: Vec::new(),
            rooms: Vec::new(),
            matches: Vec::new(),
        }
    }

    pub fn join_room(&mut self, request: JoinRoomRequest) -> Result<(), String> {
        let room = self
            .rooms
            .iter_mut()
            .find(|room| room.id == request.room_id)
            .ok_or_else(|| format!("room {} not found", request.room_id))?;

        if room.player_ids.len() as u32 >= room.max_players {
            return Err("room is full".to_string());
        }

        if !room.player_ids.iter().any(|id| id == &request.player_id) {
            room.player_ids.push(request.player_id.clone());
        }

        if let Some(player) = self
            .players
            .iter_mut()
            .find(|player| player.id == request.player_id)
        {
            player.room_id = Some(request.room_id.clone());
            player.team = Some(if room.player_ids.len() % 2 == 0 {
                "red".to_string()
            } else {
                "blue".to_string()
            });
        }

        Ok(())
    }

    pub fn start_match(&mut self, request: StartMatchRequest) -> Result<String, String> {
        let room = self
            .rooms
            .iter()
            .find(|room| room.id == request.room_id)
            .ok_or_else(|| format!("room {} not found", request.room_id))?;

        if room.player_ids.is_empty() {
            return Err("room must have at least one player before starting a match".to_string());
        }

        let match_id = format!("match-{}", self.matches.len() + 1);
        let record = MatchRecord {
            id: match_id.clone(),
            room_id: request.room_id,
            status: MatchStatus::InProgress,
            blue_team_score: 0,
            red_team_score: 0,
            started_at: Some(chrono::Utc::now().to_rfc3339()),
            winner: None,
        };

        self.matches.push(record);
        Ok(match_id)
    }

    pub fn update_match_state(&mut self, request: UpdateMatchStateRequest) -> Result<(), String> {
        let match_record = self
            .matches
            .iter_mut()
            .find(|match_record| match_record.id == request.match_id)
            .ok_or_else(|| format!("match {} not found", request.match_id))?;

        let new_status = request.status.clone();

        match_record.blue_team_score = request.blue_team_score;
        match_record.red_team_score = request.red_team_score;
        match_record.status = new_status.clone();

        if matches!(new_status, MatchStatus::Completed) {
            match_record.winner = Some(if request.blue_team_score >= request.red_team_score {
                "blue".to_string()
            } else {
                "red".to_string()
            });
        }

        Ok(())
    }
}

pub fn create_default_state() -> ModuleState {
    let mut state = ModuleState::new();

    state.players.push(PlayerRecord {
        id: "player-1".to_string(),
        name: "Alpha".to_string(),
        room_id: None,
        team: None,
    });

    state.rooms.push(RoomRecord {
        id: "room-1".to_string(),
        name: "Alpha Arena".to_string(),
        is_open: true,
        max_players: 8,
        player_ids: vec!["player-1".to_string()],
    });

    state
}
