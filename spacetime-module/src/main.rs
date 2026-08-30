use spacetime_module::{MatchStatus, create_default_state};

fn main() {
    let mut state = create_default_state();

    println!("Spacetime module booted.");
    println!("Players: {}", state.players.len());
    println!("Rooms: {}", state.rooms.len());

    let room_id = state
        .rooms
        .first()
        .map(|room| room.id.clone())
        .unwrap_or_default();
    let _ = state.start_match(spacetime_module::StartMatchRequest {
        room_id,
        organizer_id: "player-1".to_string(),
    });

    if let Some(match_record) = state.matches.first() {
        println!(
            "Match {} status: {:?}",
            match_record.id, match_record.status
        );
    }

    println!("Default match state is ready for SpacetimeDB migration.");
    println!("Current phase: {:?}", MatchStatus::Lobby);
}
