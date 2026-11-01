use axum::{
    Json, Router,
    extract::Extension,
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::Html,
    routing::{get, post},
};
use futures_util::SinkExt;
use futures_util::stream::StreamExt;
use std::{convert::Infallible, sync::Arc};

use crate::{
    db::SpaceTimeClient,
    game::{GameState, JoinRequest, JoinResponse},
    types::{QueueRequest, QueueResponse, ReadyRequest, ReadyResponse},
};

pub type SharedState = Arc<AppState>;

#[derive(Clone)]
pub struct AppState {
    pub game_state: GameState,
    #[allow(dead_code)]
    pub db_client: SpaceTimeClient,
}

pub fn create_router(state: SharedState) -> Router {
    Router::new()
        .route("/", get(root_handler))
        .route("/join", post(join_handler))
        .route("/queue", post(queue_handler))
        .route("/ready", post(ready_handler))
        .route("/solo/session", post(solo_session_handler))
        .route("/solo/input", post(solo_input_handler))
        .route("/solo/reset", post(solo_reset_handler))
        .route("/ws", get(ws_handler))
        .layer(Extension(state))
}

async fn root_handler() -> Result<Html<String>, Infallible> {
    let html = std::fs::read_to_string("static/index.html").unwrap_or_else(|_| {
        "<html><body><h1>Wars on Goats</h1><p>Static client failed to load.</p></body></html>"
            .to_string()
    });
    Ok(Html(html))
}

async fn join_handler(
    Extension(state): Extension<SharedState>,
    Json(payload): Json<JoinRequest>,
) -> Json<JoinResponse> {
    let player = state.game_state.register_player(payload.name.clone());
    let response = JoinResponse {
        player_id: player.id.clone(),
        welcome: format!("Welcome, {}!", player.name),
    };

    Json(response)
}

async fn queue_handler(
    Extension(state): Extension<SharedState>,
    Json(payload): Json<QueueRequest>,
) -> Json<QueueResponse> {
    // Verify player exists
    if state.game_state.get_player(&payload.player_id).is_none() {
        return Json(QueueResponse {
            queued: false,
            message: "Player not found".to_string(),
            player_id: payload.player_id,
        });
    }

    // Add to queue
    state.game_state.add_to_queue(
        payload.player_id.clone(),
        payload.talent_build.clone(),
        payload.game_mode,
    );

    // Try to create a match from the queue
    if let Some(room_id) = state
        .game_state
        .try_create_match_from_queue(payload.game_mode)
    {
        tracing::info!(
            "Match created: room_id={}, mode={:?}",
            room_id,
            payload.game_mode
        );
    }

    Json(QueueResponse {
        queued: true,
        message: "Successfully queued for match".to_string(),
        player_id: payload.player_id,
    })
}

async fn ready_handler(
    Extension(state): Extension<SharedState>,
    Json(payload): Json<ReadyRequest>,
) -> Json<ReadyResponse> {
    let success = state
        .game_state
        .set_player_ready(&payload.player_id, payload.ready);

    if success {
        // Check if the match should start
        if let Some(room_id) = state
            .game_state
            .player_room_map
            .lock()
            .unwrap()
            .get(&payload.player_id)
            .cloned()
            && state.game_state.check_and_start_match(&room_id)
        {
            tracing::info!("Match started: room_id={}", room_id);
        }

        Json(ReadyResponse {
            success: true,
            message: format!(
                "Ready state updated to: {}",
                if payload.ready { "ready" } else { "not ready" }
            ),
        })
    } else {
        Json(ReadyResponse {
            success: false,
            message: "Player not found in any room".to_string(),
        })
    }
}

async fn solo_session_handler(
    Extension(state): Extension<SharedState>,
    Json(payload): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    let name = payload
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("Hero");
    let player = state.game_state.start_local_solo_session(name);

    Json(serde_json::json!({
        "player_id": player.id,
        "name": player.name,
        "spawn": { "x": player.position.x, "y": player.position.y },
        "targets": training_targets_json(&state.game_state, &player.id)
    }))
}

async fn solo_input_handler(
    Extension(state): Extension<SharedState>,
    Json(payload): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    let player_id = payload
        .get("player_id")
        .and_then(|v| v.as_str())
        .unwrap_or_default();
    let player_x = payload
        .get("player_x")
        .and_then(|v| v.as_f64())
        .unwrap_or(120.0) as f32;
    let player_y = payload
        .get("player_y")
        .and_then(|v| v.as_f64())
        .unwrap_or(154.0) as f32;
    let aim_x = payload.get("aim_x").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
    let aim_y = payload.get("aim_y").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
    let ability_id = payload
        .get("ability_id")
        .and_then(|v| v.as_str())
        .map(str::to_string);
    let cast_attempted = ability_id.is_some();
    let requested_ability_id = ability_id.clone();

    let event = state.game_state.set_player_input(
        player_id,
        crate::types::Position {
            x: player_x,
            y: player_y,
        },
        crate::types::Position { x: aim_x, y: aim_y },
        ability_id,
    );

    let cast_succeeded = cast_attempted && event.is_some();
    let now_ms = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64;
    let cast_cooldown_remaining_ms = requested_ability_id
        .as_ref()
        .and_then(|ability_id| {
            state
                .game_state
                .get_player(player_id)
                .and_then(|player| player.ability_cooldowns.get(ability_id).copied())
        })
        .map(|cooldown_until| cooldown_until.saturating_sub(now_ms))
        .unwrap_or(0);
    let targets = training_targets_json(&state.game_state, player_id);

    Json(serde_json::json!({
        "ok": event.is_some() || !player_id.is_empty(),
        "cast_succeeded": cast_succeeded,
        "cast_cooldown_remaining_ms": cast_cooldown_remaining_ms,
        "targets": targets,
        "event": event.map(|ev| {
            serde_json::json!({
                "source_player": ev.source_player,
                "affected_players": ev.affected_players,
                "damage": ev.damage,
                "healing": ev.healing,
                "status_effects": ev.status_effects.iter().map(|effect| format!("{:?}", effect)).collect::<Vec<_>>()
            })
        })
    }))
}

fn training_targets_json(game_state: &GameState, player_id: &str) -> Vec<serde_json::Value> {
    game_state
        .training_targets_for_player(player_id)
        .iter()
        .map(|target| {
            serde_json::json!({
                "id": target.id,
                "name": target.name,
                "health": target.health,
                "max_health": target.max_health,
                "position": { "x": target.position.x, "y": target.position.y },
                "alive": target.alive
            })
        })
        .collect()
}

async fn solo_reset_handler(
    Extension(state): Extension<SharedState>,
    Json(payload): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    let player_id = payload
        .get("player_id")
        .and_then(|v| v.as_str())
        .unwrap_or_default();
    let reset = state.game_state.reset_local_session(player_id);
    Json(serde_json::json!({
        "ok": reset.is_some(),
        "player_id": player_id,
        "spawn": reset.map(|player| { serde_json::json!({ "x": player.position.x, "y": player.position.y }) }),
        "targets": training_targets_json(&state.game_state, player_id)
    }))
}

async fn ws_handler(
    Extension(state): Extension<SharedState>,
    ws: WebSocketUpgrade,
) -> impl axum::response::IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(stream: WebSocket, state: SharedState) {
    let (mut sender, mut receiver) = stream.split();
    while let Some(message) = receiver.next().await {
        match message {
            Ok(Message::Text(text)) => {
                let payload: serde_json::Value = match serde_json::from_str(&text) {
                    Ok(value) => value,
                    Err(_) => {
                        let _ = sender
                            .send(Message::Text(
                                "{\"type\":\"error\",\"message\":\"invalid-json\"}".to_string(),
                            ))
                            .await;
                        continue;
                    }
                };

                let player_id = payload
                    .get("player_id")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();
                let snapshot = if !player_id.is_empty() {
                    state.game_state.snapshot_for_player(player_id)
                } else {
                    serde_json::json!({ "type": "snapshot", "players": [] })
                };

                let _ = sender.send(Message::Text(snapshot.to_string())).await;
            }
            Ok(Message::Close(_)) => break,
            _ => {}
        }
    }
}
