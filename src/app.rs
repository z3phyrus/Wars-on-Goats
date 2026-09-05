use axum::{
    Json, Router,
    extract::Extension,
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    routing::{get, post},
};
use futures_util::SinkExt;
use futures_util::stream::StreamExt;
use std::sync::Arc;

use crate::{
    db::SpaceTimeClient,
    game::{GameState, JoinRequest, JoinResponse},
    types::{ApiResponse, QueueRequest, QueueResponse, ReadyRequest, ReadyResponse},
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
        .route("/ws", get(ws_handler))
        .layer(Extension(state))
}

async fn root_handler() -> Json<ApiResponse<&'static str>> {
    Json(ApiResponse {
        success: true,
        message: "Wars on Goats server is running.",
    })
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

async fn ws_handler(
    Extension(state): Extension<SharedState>,
    ws: WebSocketUpgrade,
) -> impl axum::response::IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(stream: WebSocket, _state: SharedState) {
    let (mut sender, mut receiver) = stream.split();
    if let Some(Ok(Message::Text(text))) = receiver.next().await {
        let _ = sender.send(Message::Text(format!("Echo: {}", text))).await;
    }
}
