mod app;
mod db;
mod game;
mod spells;
mod types;

use std::net::SocketAddr;
use std::sync::Arc;

use crate::{
    app::{AppState, create_router},
    db::SpaceTimeClient,
    game::GameState,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
        .with_graceful_shutdown(async {
            match tokio::signal::ctrl_c().await {
                Ok(()) => tracing::info!("shutdown signal received; stopping server"),
                Err(error) => {
                    tracing::error!(%error, "failed to listen for shutdown signal; stopping server");
                }
            }
        })
        .await?;

    Ok(())
}
