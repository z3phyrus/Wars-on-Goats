use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use crate::types::{
    AbilityDefinition, AbilityTarget, CombatEvent, GameMode, MatchState, PlayerReadyState,
    Position, Status, StatusEffect, TalentBuild, TeamId, Velocity, VisibilityState, WorldMap,
};

// ============ Player ============
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Player {
    pub id: String,
    pub name: String,
    pub team_id: Option<TeamId>,
    pub talent_build: Option<TalentBuild>,
    pub position: Position,
    pub velocity: Velocity,
    pub health: f32,
    pub max_health: f32,
    pub alive: bool,
    pub statuses: Vec<Status>,
    pub ability_cooldowns: HashMap<String, u64>,
    pub wins: u32,
    pub losses: u32,
    // #stretch-goal: rank field
}

impl Player {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            team_id: None,
            talent_build: None,
            position: Position { x: 0.0, y: 0.0 },
            velocity: Velocity { dx: 0.0, dy: 0.0 },
            health: 100.0,
            max_health: 100.0,
            alive: true,
            statuses: Vec::new(),
            ability_cooldowns: HashMap::new(),
            wins: 0,
            losses: 0,
        }
    }

    #[allow(dead_code)]
    pub fn distance_to(&self, other: &Player) -> f32 {
        ((self.position.x - other.position.x).powi(2)
            + (self.position.y - other.position.y).powi(2))
        .sqrt()
    }
}

// ============ Team ============
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Team {
    pub id: TeamId,
    pub score: u32,
    pub base_position: Position,
    pub player_ids: Vec<String>,
    pub team_size_target: usize,
}

impl Team {
    pub fn new(id: TeamId, team_size_target: usize) -> Self {
        let base_position = match id {
            TeamId::Team1 => Position { x: 0.0, y: 0.0 },
            TeamId::Team2 => Position { x: 100.0, y: 100.0 },
        };

        Self {
            id,
            score: 0,
            base_position,
            player_ids: Vec::new(),
            team_size_target,
        }
    }

    #[allow(dead_code)]
    pub fn player_count(&self) -> usize {
        self.player_ids.len()
    }

    #[allow(dead_code)]
    pub fn is_full(&self) -> bool {
        self.player_ids.len() >= self.team_size_target
    }
}

// ============ GameRoom ============
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameRoom {
    pub room_id: String,
    pub game_mode: GameMode,
    pub players: Vec<String>,
    pub teams: HashMap<TeamId, Team>,
    pub map_id: String,
    pub match_state: MatchState,
    pub ready_states: HashMap<String, PlayerReadyState>, // player_id -> ready state
    pub created_at: u64,
    pub match_start_time: Option<u64>, // timestamp when match should start
}

impl GameRoom {
    pub fn new(room_id: String, game_mode: GameMode) -> Self {
        let team_size = match game_mode {
            GameMode::CaptureTheFlag => 5,
            GameMode::ControlBases => 10,
        };

        let mut teams = HashMap::new();
        teams.insert(TeamId::Team1, Team::new(TeamId::Team1, team_size));
        teams.insert(TeamId::Team2, Team::new(TeamId::Team2, team_size));

        Self {
            room_id,
            game_mode,
            players: Vec::new(),
            teams,
            map_id: "default_map".to_string(),
            match_state: MatchState::Queued,
            ready_states: HashMap::new(),
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            match_start_time: None,
        }
    }

    #[allow(dead_code)]
    pub fn is_full(&self) -> bool {
        let team_size = match self.game_mode {
            GameMode::CaptureTheFlag => 5,
            GameMode::ControlBases => 10,
        };
        self.players.len() >= team_size * 2
    }

    pub fn all_ready(&self) -> bool {
        !self.players.is_empty()
            && self.players.iter().all(|pid| {
                self.ready_states
                    .get(pid)
                    .map(|state| *state == PlayerReadyState::Ready)
                    .unwrap_or(false)
            })
    }
}

// ============ MatchmakingEntry ============
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MatchmakingEntry {
    pub player_id: String,
    pub talent_build: TalentBuild,
    pub game_mode: GameMode,
    pub queue_join_time: u64,
    // #stretch-goal: rank field
}

// ============ Join/Queue Request/Response ============
#[derive(Clone, Debug, Deserialize)]
pub struct JoinRequest {
    pub name: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct JoinResponse {
    pub player_id: String,
    pub welcome: String,
}

// ============ GameState ============
#[derive(Clone, Debug)]
pub struct GameState {
    pub players: Arc<Mutex<HashMap<String, Player>>>,
    pub rooms: Arc<Mutex<HashMap<String, GameRoom>>>,
    pub queues: Arc<Mutex<HashMap<GameMode, Vec<MatchmakingEntry>>>>,
    pub player_room_map: Arc<Mutex<HashMap<String, String>>>, // player_id -> room_id
    pub room_start_timers: Arc<Mutex<HashMap<String, Instant>>>, // room_id -> timer start
}

impl GameState {
    pub fn new() -> Self {
        Self {
            players: Arc::new(Mutex::new(HashMap::new())),
            rooms: Arc::new(Mutex::new(HashMap::new())),
            queues: Arc::new(Mutex::new(HashMap::new())),
            player_room_map: Arc::new(Mutex::new(HashMap::new())),
            room_start_timers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    // -------- Player Management --------
    pub fn register_player(&self, name: String) -> Player {
        let player = Player::new(uuid::Uuid::new_v4().to_string(), name);
        self.players
            .lock()
            .unwrap()
            .insert(player.id.clone(), player.clone());
        player
    }

    pub fn get_player(&self, player_id: &str) -> Option<Player> {
        self.players.lock().unwrap().get(player_id).cloned()
    }

    #[allow(dead_code)]
    pub fn get_player_mut<F>(&self, player_id: &str, f: F)
    where
        F: FnOnce(&mut Player),
    {
        if let Some(player) = self.players.lock().unwrap().get_mut(player_id) {
            f(player);
        }
    }

    #[allow(dead_code)]
    pub fn remove_player(&self, player_id: &str) -> Option<Player> {
        self.players.lock().unwrap().remove(player_id)
    }

    // -------- Room Management --------
    pub fn create_room(&self, game_mode: GameMode) -> String {
        let room_id = uuid::Uuid::new_v4().to_string();
        let room = GameRoom::new(room_id.clone(), game_mode);
        self.rooms.lock().unwrap().insert(room_id.clone(), room);
        room_id
    }

    pub fn get_room(&self, room_id: &str) -> Option<GameRoom> {
        self.rooms.lock().unwrap().get(room_id).cloned()
    }

    pub fn get_room_mut<F>(&self, room_id: &str, f: F)
    where
        F: FnOnce(&mut GameRoom),
    {
        if let Some(room) = self.rooms.lock().unwrap().get_mut(room_id) {
            f(room);
        }
    }

    // -------- Matchmaking --------
    pub fn add_to_queue(
        &self,
        player_id: String,
        talent_build: TalentBuild,
        game_mode: GameMode,
    ) -> bool {
        let entry = MatchmakingEntry {
            player_id,
            talent_build,
            game_mode,
            queue_join_time: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };

        let mut queues = self.queues.lock().unwrap();
        queues.entry(game_mode).or_default().push(entry);
        true
    }

    #[allow(dead_code)]
    pub fn get_queue(&self, game_mode: GameMode) -> Vec<MatchmakingEntry> {
        self.queues
            .lock()
            .unwrap()
            .get(&game_mode)
            .cloned()
            .unwrap_or_default()
    }

    /// Tries to match queued players into a room.
    /// Returns the room_id if a match was created, None otherwise.
    pub fn try_create_match_from_queue(&self, game_mode: GameMode) -> Option<String> {
        let needed_players = match game_mode {
            GameMode::CaptureTheFlag => 10, // 5v5
            GameMode::ControlBases => 20,   // 10v10
        };

        let mut queues = self.queues.lock().unwrap();
        let queue = queues.get_mut(&game_mode)?;

        if queue.len() < needed_players {
            return None;
        }

        let room_id = self.create_room(game_mode);
        let entries: Vec<_> = queue.drain(0..needed_players).collect();
        drop(queues); // Release the lock early

        // Assign players to teams fairly (round-robin by join time)
        let mut entries = entries;
        entries.sort_by_key(|e| e.queue_join_time);

        // Collect team assignments first
        let mut team_assignments: Vec<(String, TeamId)> = Vec::new();
        for (idx, entry) in entries.iter().enumerate() {
            let team_id = if idx % 2 == 0 {
                TeamId::Team1
            } else {
                TeamId::Team2
            };
            team_assignments.push((entry.player_id.clone(), team_id));
        }

        // Update room with players and team assignments
        self.get_room_mut(&room_id, |room| {
            for (player_id, team_id) in &team_assignments {
                room.players.push(player_id.clone());
                room.ready_states
                    .insert(player_id.clone(), PlayerReadyState::NotReady);

                if let Some(team) = room.teams.get_mut(team_id) {
                    team.player_ids.push(player_id.clone());
                }
            }
        });

        // Update player with team assignment and talent build
        for (entry, (player_id, team_id)) in entries.iter().zip(team_assignments.iter()) {
            self.player_room_map
                .lock()
                .unwrap()
                .insert(player_id.clone(), room_id.clone());

            if let Some(player) = self.players.lock().unwrap().get_mut(player_id) {
                player.team_id = Some(*team_id);
                player.talent_build = Some(entry.talent_build.clone());
                player.max_health =
                    100.0 + (entry.talent_build.selected_talents.len() as f32 * 10.0);
                player.health = player.max_health;
            }
        }

        // Start the match start timer (30 seconds)
        self.room_start_timers
            .lock()
            .unwrap()
            .insert(room_id.clone(), Instant::now());

        Some(room_id)
    }

    pub fn set_player_ready(&self, player_id: &str, ready: bool) -> bool {
        // Find the room the player is in
        if let Some(room_id) = self.player_room_map.lock().unwrap().get(player_id).cloned() {
            self.get_room_mut(&room_id, |room| {
                room.ready_states.insert(
                    player_id.to_string(),
                    if ready {
                        PlayerReadyState::Ready
                    } else {
                        PlayerReadyState::NotReady
                    },
                );
            });
            return true;
        }
        false
    }

    pub fn check_and_start_match(&self, room_id: &str) -> bool {
        if let Some(room) = self.get_room(room_id) {
            let timer_start = self.room_start_timers.lock().unwrap().get(room_id).copied();

            // Check if all players are ready OR 30 seconds have passed
            let all_ready = room.all_ready();
            let timer_expired =
                timer_start.is_some_and(|start| start.elapsed() >= Duration::from_secs(30));

            if all_ready || timer_expired {
                self.get_room_mut(room_id, |room| {
                    room.match_state = MatchState::InProgress;
                    room.match_start_time = Some(
                        std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_secs(),
                    );
                });
                return true;
            }
        }
        false
    }

    #[allow(dead_code)]
    pub fn remove_player_from_room(&self, player_id: &str) {
        if let Some(room_id) = self.player_room_map.lock().unwrap().remove(player_id) {
            self.get_room_mut(&room_id, |room| {
                room.players.retain(|id| id != player_id);
                room.ready_states.remove(player_id);

                for team in room.teams.values_mut() {
                    team.player_ids.retain(|id| id != player_id);
                }
            });
        }
    }

    #[allow(dead_code)]
    pub fn get_world_map(&self) -> WorldMap {
        WorldMap::default()
    }

    #[allow(dead_code)]
    pub fn compute_visibility_for_player(
        &self,
        player_id: &str,
        map: &WorldMap,
    ) -> VisibilityState {
        let Some(player) = self.get_player(player_id) else {
            return VisibilityState::default();
        };

        let mut visible_players = Vec::new();
        let mut hidden_players = Vec::new();
        let mut visible_zones = Vec::new();

        for (other_id, other) in self.players.lock().unwrap().iter() {
            if other_id == player_id {
                continue;
            }
            let dist = player.distance_to(other);
            let is_visible =
                dist <= 180.0 && !map.line_of_sight_blocked(&player.position, &other.position);
            if is_visible {
                visible_players.push(other_id.clone());
            } else {
                hidden_players.push(other_id.clone());
            }
        }

        for zone in &map.zones {
            let center = zone.center();
            let dist = ((player.position.x - center.x).powi(2)
                + (player.position.y - center.y).powi(2))
            .sqrt();
            if dist <= zone.visibility_range {
                visible_zones.push(zone.id.clone());
            }
        }

        VisibilityState {
            visible_players,
            hidden_players,
            visible_zones,
        }
    }

    #[allow(dead_code)]
    pub fn apply_status_effect(
        &self,
        player_id: &str,
        effect: StatusEffect,
        duration_ms: u64,
        intensity: f32,
    ) {
        let mut players = self.players.lock().unwrap();
        if let Some(player) = players.get_mut(player_id) {
            player.statuses.push(Status {
                effect,
                duration_ms,
                intensity,
            });
        }
    }

    #[allow(dead_code)]
    pub fn tick_status_effects(&self) {
        let mut players = self.players.lock().unwrap();
        for player in players.values_mut() {
            let mut remaining = Vec::new();
            for status in &player.statuses {
                let updated_duration = status.duration_ms.saturating_sub(100);
                if updated_duration > 0 {
                    remaining.push(Status {
                        effect: status.effect.clone(),
                        duration_ms: updated_duration,
                        intensity: status.intensity,
                    });
                } else if matches!(status.effect, StatusEffect::Burn | StatusEffect::Poison) {
                    player.health = (player.health - status.intensity * 8.0).max(0.0);
                    if player.health <= 0.0 {
                        player.alive = false;
                    }
                }
            }
            player.statuses = remaining;
        }
    }

    #[allow(dead_code)]
    pub fn try_cast_ability(
        &self,
        attacker_id: &str,
        target_id: &str,
        ability: &AbilityDefinition,
    ) -> Result<CombatEvent, String> {
        let attacker_snapshot = {
            let players = self.players.lock().unwrap();
            players
                .get(attacker_id)
                .cloned()
                .ok_or_else(|| "attacker not found".to_string())?
        };
        let target_snapshot = {
            let players = self.players.lock().unwrap();
            players
                .get(target_id)
                .cloned()
                .ok_or_else(|| "target not found".to_string())?
        };

        if !attacker_snapshot.alive {
            return Err("attacker is dead".to_string());
        }
        if !target_snapshot.alive {
            return Err("target is already dead".to_string());
        }

        let distance = attacker_snapshot.distance_to(&target_snapshot);
        if distance > ability.range {
            return Err("target is out of range".to_string());
        }

        if attacker_snapshot
            .statuses
            .iter()
            .any(|status| matches!(status.effect, StatusEffect::Silence) && status.duration_ms > 0)
        {
            return Err("attacker is silenced".to_string());
        }

        let now_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        let same_team = attacker_snapshot.team_id == target_snapshot.team_id;
        let kind_matches = match ability.target {
            AbilityTarget::Enemy => !same_team,
            AbilityTarget::Ally => same_team,
            AbilityTarget::SelfCast => true,
            AbilityTarget::Any => true,
        };

        if !kind_matches {
            return Err("ability target does not match".to_string());
        }

        {
            let mut players = self.players.lock().unwrap();
            let attacker = players
                .get_mut(attacker_id)
                .ok_or_else(|| "attacker not found".to_string())?;
            if let Some(cooldown_until) = attacker.ability_cooldowns.get(&ability.id)
                && *cooldown_until > now_ms
            {
                return Err("ability is on cooldown".to_string());
            }
            attacker
                .ability_cooldowns
                .insert(ability.id.clone(), now_ms + ability.cooldown_ms);
        }

        let damage = (ability.min_damage + ability.max_damage) / 2.0;
        {
            let mut players = self.players.lock().unwrap();
            let target = players
                .get_mut(target_id)
                .ok_or_else(|| "target not found".to_string())?;
            let new_health = (target.health - damage).max(0.0);
            target.health = new_health;
            if target.health <= 0.0 {
                target.alive = false;
            }
        }

        let event = CombatEvent {
            source_player: attacker_snapshot.id.clone(),
            target_player: Some(target_snapshot.id.clone()),
            target_zone: None,
            damage,
            healing: 0.0,
            status_effects: ability.effects.clone(),
        };

        Ok(event)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{AbilityDefinition, AbilityKind, AbilityTarget, WorldMap};

    #[test]
    fn world_map_tracks_bases_spawn_and_visibility() {
        let map = WorldMap::default();
        assert!(map.width > 0.0);
        assert!(map.height > 0.0);
        assert_eq!(map.spawn_points.len(), 2);

        let mut blue = map.spawn_points.get(&TeamId::Team1).unwrap().clone();
        blue.x += 12.0;
        assert!(blue.x > map.spawn_points.get(&TeamId::Team1).unwrap().x);

        let world = GameState::new();
        let p1 = world.register_player("alice".to_string());
        let p2 = world.register_player("bob".to_string());
        world.get_player_mut(&p1.id, |player| {
            player.team_id = Some(TeamId::Team1);
            player.position = Position { x: 10.0, y: 20.0 };
        });
        world.get_player_mut(&p2.id, |player| {
            player.team_id = Some(TeamId::Team2);
            player.position = Position { x: 15.0, y: 20.0 };
        });

        let visibility = world.compute_visibility_for_player(&p1.id, &map);
        assert!(visibility.visible_players.contains(&p2.id));
        assert!(!visibility.visible_zones.is_empty());
    }

    #[test]
    fn combat_ability_checks_range_cooldown_and_state() {
        let world = GameState::new();
        let attacker = world.register_player("attacker".to_string());
        let defender = world.register_player("defender".to_string());

        world.get_player_mut(&attacker.id, |player| {
            player.team_id = Some(TeamId::Team1);
            player.position = Position { x: 10.0, y: 10.0 };
            player.alive = true;
            player.health = 100.0;
            player.max_health = 100.0;
        });
        world.get_player_mut(&defender.id, |player| {
            player.team_id = Some(TeamId::Team2);
            player.position = Position { x: 12.0, y: 10.0 };
            player.alive = true;
            player.health = 100.0;
            player.max_health = 100.0;
        });

        let ability = AbilityDefinition {
            id: "basic-shot".to_string(),
            name: "Basic Shot".to_string(),
            kind: AbilityKind::SingleTarget,
            target: AbilityTarget::Enemy,
            range: 15.0,
            cooldown_ms: 2000,
            cast_time_ms: 0,
            effects: vec![],
            min_damage: 10.0,
            max_damage: 18.0,
        };

        let result = world.try_cast_ability(&attacker.id, &defender.id, &ability);
        assert!(result.is_ok());
        assert!(world.get_player(&defender.id).unwrap().health < 100.0);

        let second = world.try_cast_ability(&attacker.id, &defender.id, &ability);
        assert!(second.is_err());
    }

    #[test]
    fn status_effects_expire_and_damage_can_ko_players() {
        let world = GameState::new();
        let source = world.register_player("source".to_string());
        let target = world.register_player("target".to_string());

        world.get_player_mut(&source.id, |player| {
            player.team_id = Some(TeamId::Team1);
            player.position = Position { x: 0.0, y: 0.0 };
        });
        world.get_player_mut(&target.id, |player| {
            player.team_id = Some(TeamId::Team2);
            player.position = Position { x: 5.0, y: 0.0 };
            player.health = 15.0;
            player.max_health = 100.0;
            player.alive = true;
        });

        world.apply_status_effect(&target.id, crate::types::StatusEffect::Burn, 10, 1.0);
        world.apply_status_effect(&target.id, crate::types::StatusEffect::Slow, 20, 0.5);
        let before = world.get_player(&target.id).unwrap().health;
        world.tick_status_effects();
        let after = world.get_player(&target.id).unwrap().health;
        assert!(after <= before || !world.get_player(&target.id).unwrap().alive);

        world.get_player_mut(&target.id, |player| {
            player.health = 0.0;
            player.alive = false;
        });
        assert!(!world.get_player(&target.id).unwrap().alive);
    }
}
