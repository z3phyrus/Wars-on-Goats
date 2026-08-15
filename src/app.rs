use axum::{extract::ws::{Message, WebSocket, WebSocketUpgrade}, extract::Extension, routing::{get, post}, Json, Router};
use futures_util::stream::StreamExt;
use futures_util::SinkExt;
use std::sync::Arc;

use crate::{db::SpaceTimeClient, game::{GameState, JoinRequest, JoinResponse}, types::ApiResponse};

pub type SharedState = Arc<AppState>;

#[derive(Clone)]
pub struct AppState {
    pub game_state: GameState,
    pub db_client: SpaceTimeClient,
}

pub fn create_router(state: SharedState) -> Router {
    Router::new()
        .route("/", get(root_handler))
        .route("/join", post(join_handler))
        .route("/ws", get(ws_handler))
        .layer(Extension(state))
}

async fn root_handler() -> Json<ApiResponse<&'static str>> {
    Json(ApiResponse { success: true, message: "Wars on Goats server is running." })
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

async fn ws_handler(
    Extension(state): Extension<SharedState>,
    ws: WebSocketUpgrade,
) -> impl axum::response::IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(stream: WebSocket, state: SharedState) {
    let (mut sender, mut receiver) = stream.split();
    if let Some(Ok(message)) = receiver.next().await {
        if let Message::Text(text) = message {
            let _ = sender.send(Message::Text(format!("Echo: {}", text))).await;
        }
    }
}
