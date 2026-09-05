use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: T,
}

// ============ Status Effects ============
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum StatusEffect {
    Slow,
    Stun,
    Poison,
    Burn,
    Heal,
    // Add more as needed
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Status {
    pub effect: StatusEffect,
    pub duration_ms: u64,
    pub intensity: f32, // 0.0 to 1.0 for most effects
}

// ============ Game Classes and Talents ============
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Class {
    Hunter,
    Mage,
    Rogue,
    Warlock,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TalentBuild {
    pub class: Class,
    pub specialization: String, // e.g., "Beast Hunter", "Frost Mage"
    pub selected_talents: Vec<String>, // names of selected talents
}

// ============ Position and Movement ============
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Velocity {
    pub dx: f32,
    pub dy: f32,
}

// ============ Teams ============
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Copy)]
pub enum TeamId {
    Team1,
    Team2,
}

// ============ Match and Game Mode ============
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Copy)]
pub enum GameMode {
    CaptureTheFlag, // 5v5
    ControlBases,   // 10v10
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Copy)]
pub enum MatchState {
    Queued,
    Loading,
    InProgress,
    Finished,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Copy)]
pub enum PlayerReadyState {
    NotReady,
    Ready,
}

// ============ Request/Response DTOs ============
#[derive(Clone, Debug, Deserialize)]
pub struct QueueRequest {
    pub player_id: String,
    pub talent_build: TalentBuild,
    pub game_mode: GameMode,
}

#[derive(Clone, Debug, Serialize)]
pub struct QueueResponse {
    pub queued: bool,
    pub message: String,
    pub player_id: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ReadyRequest {
    pub player_id: String,
    pub ready: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct ReadyResponse {
    pub success: bool,
    pub message: String,
}
