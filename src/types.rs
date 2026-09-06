use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    Silence,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Status {
    pub effect: StatusEffect,
    pub duration_ms: u64,
    pub intensity: f32, // 0.0 to 1.0 for most effects
}

#[allow(dead_code)]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AbilityKind {
    SingleTarget,
    AreaOfEffect,
    SelfCast,
    Passive,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AbilityTarget {
    Enemy,
    Ally,
    SelfCast,
    Any,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AbilityDefinition {
    pub id: String,
    pub name: String,
    pub kind: AbilityKind,
    pub target: AbilityTarget,
    pub range: f32,
    pub cooldown_ms: u64,
    pub cast_time_ms: u64,
    pub effects: Vec<StatusEffect>,
    pub min_damage: f32,
    pub max_damage: f32,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CombatEvent {
    pub source_player: String,
    pub target_player: Option<String>,
    pub target_zone: Option<String>,
    pub damage: f32,
    pub healing: f32,
    pub status_effects: Vec<StatusEffect>,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Copy)]
pub enum ZoneKind {
    Spawn,
    Base,
    Objective,
    Hazard,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorldBounds {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorldObstacle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorldZone {
    pub id: String,
    pub label: String,
    pub kind: ZoneKind,
    pub bounds: WorldBounds,
    pub team: Option<TeamId>,
    pub visibility_range: f32,
}

#[allow(dead_code)]
impl WorldZone {
    pub fn center(&self) -> Position {
        Position {
            x: self.bounds.x + self.bounds.width / 2.0,
            y: self.bounds.y + self.bounds.height / 2.0,
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct VisibilityState {
    pub visible_players: Vec<String>,
    pub hidden_players: Vec<String>,
    pub visible_zones: Vec<String>,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct WorldMap {
    pub width: f32,
    pub height: f32,
    pub spawn_points: HashMap<TeamId, Position>,
    pub base_zones: HashMap<TeamId, WorldZone>,
    pub obstacles: Vec<WorldObstacle>,
    pub platforms: Vec<WorldBounds>,
    pub zones: Vec<WorldZone>,
}

#[allow(dead_code)]
impl WorldMap {
    pub fn default() -> Self {
        let mut spawn_points = HashMap::new();
        spawn_points.insert(TeamId::Team1, Position { x: 60.0, y: 80.0 });
        spawn_points.insert(TeamId::Team2, Position { x: 940.0, y: 80.0 });

        let mut base_zones = HashMap::new();
        base_zones.insert(
            TeamId::Team1,
            WorldZone {
                id: "team1-base".to_string(),
                label: "Team 1 base".to_string(),
                kind: ZoneKind::Base,
                bounds: WorldBounds {
                    x: 25.0,
                    y: 25.0,
                    width: 200.0,
                    height: 160.0,
                },
                team: Some(TeamId::Team1),
                visibility_range: 220.0,
            },
        );
        base_zones.insert(
            TeamId::Team2,
            WorldZone {
                id: "team2-base".to_string(),
                label: "Team 2 base".to_string(),
                kind: ZoneKind::Base,
                bounds: WorldBounds {
                    x: 775.0,
                    y: 25.0,
                    width: 200.0,
                    height: 160.0,
                },
                team: Some(TeamId::Team2),
                visibility_range: 220.0,
            },
        );

        let zones = vec![
            base_zones[&TeamId::Team1].clone(),
            base_zones[&TeamId::Team2].clone(),
            WorldZone {
                id: "mid-field".to_string(),
                label: "Mid field".to_string(),
                kind: ZoneKind::Objective,
                bounds: WorldBounds {
                    x: 425.0,
                    y: 20.0,
                    width: 150.0,
                    height: 180.0,
                },
                team: None,
                visibility_range: 180.0,
            },
        ];

        Self {
            width: 1024.0,
            height: 240.0,
            spawn_points,
            base_zones,
            obstacles: vec![
                WorldObstacle {
                    x: 350.0,
                    y: 80.0,
                    width: 40.0,
                    height: 120.0,
                },
                WorldObstacle {
                    x: 635.0,
                    y: 80.0,
                    width: 40.0,
                    height: 120.0,
                },
            ],
            platforms: vec![
                WorldBounds {
                    x: 0.0,
                    y: 0.0,
                    width: 1024.0,
                    height: 12.0,
                },
                WorldBounds {
                    x: 0.0,
                    y: 160.0,
                    width: 1024.0,
                    height: 12.0,
                },
            ],
            zones,
        }
    }

    pub fn line_of_sight_blocked(&self, a: &Position, b: &Position) -> bool {
        let dx = b.x - a.x;
        let dy = b.y - a.y;
        let steps = (dx.abs().max(dy.abs()) * 2.0).ceil() as i32;

        for i in 1..steps {
            let t = i as f32 / steps as f32;
            let x = a.x + dx * t;
            let y = a.y + dy * t;
            if self.obstacles.iter().any(|obstacle| {
                x >= obstacle.x
                    && x <= obstacle.x + obstacle.width
                    && y >= obstacle.y
                    && y <= obstacle.y + obstacle.height
            }) {
                return true;
            }
        }
        false
    }
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
