use cthulhus_can_opener::{commands, EventStore, GameView};

fn main() {
    // Initialize Event Store and View
    let mut event_store = EventStore::new();
    let mut game_view = GameView::new();
    
    // Command Pattern: Start Game
    match commands::start_game("Player".to_string()) {
        Ok(events) => {
            event_store.append_events(events.clone()).unwrap();
            for event in &events {
                game_view.apply_event(event);
            }
            println!("{}", game_view.get_welcome_message());
        }
        Err(e) => {
            eprintln!("Failed to start game: {}", e);
            return;
        }
    }
    
    // Game Loop: Take Turns
    while !game_view.is_game_over() {
        println!("\n{}", game_view.get_turn_display());
        
        // Command Pattern: Take Turn -> Events
        match commands::take_turn(&game_view) {
            Ok(events) => {
                event_store.append_events(events.clone()).unwrap();
                
                // View Pattern: Events -> View -> Display
                for event in &events {
                    game_view.apply_event(event);
                }
                
                println!("{}", game_view.get_event_description());
                println!("{}", game_view.get_score_display());
            }
            Err(e) => {
                eprintln!("Error during turn: {}", e);
                break;
            }
        }
    }
    
    // View Pattern: Final Score Display
    println!("\n{}", game_view.get_final_score_display());
    
    // Optional: Display event history for debugging
    if std::env::var("DEBUG").is_ok() {
        println!("\nEvent History:");
        for (i, event) in event_store.get_events().iter().enumerate() {
            println!("{}. {:?}", i + 1, event);
        }
    }
}
