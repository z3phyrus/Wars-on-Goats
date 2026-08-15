mod app;
mod db;
mod game;
mod types;

use std::sync::Arc;

use crate::{app::{create_router, AppState}, db::SpaceTimeClient, game::GameState};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let state = Arc::new(AppState {
        game_state: GameState::new(),
        db_client: SpaceTimeClient::new("http://localhost:8000"),
    });

    let app = create_router(state.clone());
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("starting server on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
