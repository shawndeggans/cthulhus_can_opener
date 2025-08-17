// Integration tests following Given-When-Then pattern from Event Model

#[cfg(test)]
mod given_when_then_tests {
    use crate::{commands, EventStore, GameView, GameEvent, EventType};
    use uuid::Uuid;

    #[test]
    fn given_no_game_when_start_game_command_then_game_started_event() {
        // Given: No game exists
        let mut event_store = EventStore::new();
        let mut game_view = GameView::new();
        
        // When: Start Game command is executed
        let events = commands::start_game("Alice".to_string()).unwrap();
        event_store.append_events(events.clone()).unwrap();
        
        // Then: Game Started event is created and game view is updated
        assert_eq!(events.len(), 1);
        match &events[0] {
            GameEvent::GameStarted { player_name, .. } => {
                assert_eq!(player_name, "Alice");
            }
            _ => panic!("Expected GameStarted event"),
        }
        
        // And: Game view reflects the started state
        for event in &events {
            game_view.apply_event(event);
        }
        assert!(game_view.is_active);
        assert_eq!(game_view.player_name, Some("Alice".to_string()));
        assert_eq!(game_view.current_score, 0);
        assert_eq!(game_view.turn_count, 0);
    }

    #[test]
    fn given_active_game_when_take_turn_command_then_random_event_occurs() {
        // Given: An active game
        let mut event_store = EventStore::new();
        let mut game_view = GameView::new();
        
        let start_events = commands::start_game("Bob".to_string()).unwrap();
        event_store.append_events(start_events.clone()).unwrap();
        for event in &start_events {
            game_view.apply_event(event);
        }
        
        // When: Take Turn command is executed
        let turn_events = commands::take_turn(&game_view).unwrap();
        event_store.append_events(turn_events.clone()).unwrap();
        
        // Then: Random Event Occurred event is created
        assert!(!turn_events.is_empty());
        match &turn_events[0] {
            GameEvent::RandomEventOccurred { event_type, score_change, description, .. } => {
                // Verify the event type and score change are consistent
                match event_type {
                    EventType::CanOpenerFound => {
                        assert_eq!(*score_change, 10);
                        assert!(description.contains("can opener"));
                    }
                    EventType::ArtifactDiscovered => {
                        assert_eq!(*score_change, 5);
                        assert!(description.contains("artifact"));
                    }
                    EventType::NothingHappened => {
                        assert_eq!(*score_change, 0);
                        assert!(description.contains("nothing happens"));
                    }
                }
            }
            _ => panic!("Expected RandomEventOccurred event"),
        }
    }

    #[test]
    fn given_random_event_with_score_when_score_updated_then_view_shows_new_score() {
        // Given: A game with a random event that changes score
        let mut event_store = EventStore::new();
        let mut game_view = GameView::new();
        let game_id = Uuid::new_v4();
        
        // Start the game
        let start_event = GameEvent::new_game_started(game_id, "Charlie".to_string());
        event_store.append(start_event.clone()).unwrap();
        game_view.apply_event(&start_event);
        
        // When: A score-changing random event occurs followed by score update
        let random_event = GameEvent::new_random_event_occurred(
            game_id,
            EventType::CanOpenerFound,
            10,
            "You found Cthulhu's can opener! +10 points".to_string(),
        );
        let score_event = GameEvent::new_score_updated(game_id, 10, 0);
        
        event_store.append(random_event.clone()).unwrap();
        event_store.append(score_event.clone()).unwrap();
        
        // Then: Game view is updated with new score
        game_view.apply_event(&random_event);
        game_view.apply_event(&score_event);
        
        assert_eq!(game_view.current_score, 10);
        assert_eq!(game_view.turn_count, 1);
        assert_eq!(game_view.get_score_display(), "Current score: 10");
    }

    #[test]
    fn given_fifth_turn_when_take_turn_then_game_ends() {
        // Given: A game on the 4th turn (next turn will be 5th and final)
        let mut event_store = EventStore::new();
        let mut game_view = GameView::new();
        let game_id = Uuid::new_v4();
        
        // Setup: Game started and played 4 turns
        let start_event = GameEvent::new_game_started(game_id, "David".to_string());
        game_view.apply_event(&start_event);
        event_store.append(start_event).unwrap();
        
        // Simulate 4 turns
        for turn in 1..=4 {
            let random_event = GameEvent::new_random_event_occurred(
                game_id,
                EventType::ArtifactDiscovered,
                5,
                "You discovered an ancient artifact. +5 points".to_string(),
            );
            let score_event = GameEvent::new_score_updated(game_id, turn * 5, (turn - 1) * 5);
            
            game_view.apply_event(&random_event);
            game_view.apply_event(&score_event);
            event_store.append(random_event).unwrap();
            event_store.append(score_event).unwrap();
        }
        
        assert_eq!(game_view.turn_count, 4);
        assert_eq!(game_view.current_score, 20);
        assert!(game_view.is_active);
        
        // When: Take the 5th turn
        let final_turn_events = commands::take_turn(&game_view).unwrap();
        event_store.append_events(final_turn_events.clone()).unwrap();
        
        // Then: Game ends
        let has_game_ended = final_turn_events.iter().any(|event| {
            matches!(event, GameEvent::GameEnded { .. })
        });
        assert!(has_game_ended, "Game should end after 5th turn");
        
        // And: Apply events to view to verify final state
        for event in &final_turn_events {
            game_view.apply_event(event);
        }
        
        assert!(!game_view.is_active);
        assert_eq!(game_view.turn_count, 5);
        assert!(game_view.is_game_over());
    }

    #[test]
    fn given_completed_game_when_view_final_score_then_shows_correct_display() {
        // Given: A completed game
        let mut game_view = GameView::new();
        let game_id = Uuid::new_v4();
        
        // When: Game ended event is applied
        let end_event = GameEvent::new_game_ended(game_id, 25, 5);
        game_view.apply_event(&end_event);
        
        // Then: View shows correct final score display
        assert_eq!(game_view.get_final_score_display(), "Game over! Final score: 25");
        assert!(!game_view.is_active);
        assert!(game_view.is_game_over());
        assert_eq!(game_view.current_score, 25);
        assert_eq!(game_view.turn_count, 5);
    }

    #[test]
    fn given_event_store_when_replay_events_then_reconstruct_game_state() {
        // Given: Event store with complete game history
        let mut event_store = EventStore::new();
        let game_id = Uuid::new_v4();
        
        let events = vec![
            GameEvent::new_game_started(game_id, "Eve".to_string()),
            GameEvent::new_random_event_occurred(
                game_id,
                EventType::CanOpenerFound,
                10,
                "You found Cthulhu's can opener! +10 points".to_string(),
            ),
            GameEvent::new_score_updated(game_id, 10, 0),
            GameEvent::new_random_event_occurred(
                game_id,
                EventType::NothingHappened,
                0,
                "The cosmic horror grows... but nothing happens.".to_string(),
            ),
            GameEvent::new_game_ended(game_id, 10, 2),
        ];
        
        for event in &events {
            event_store.append(event.clone()).unwrap();
        }
        
        // When: Replaying events to reconstruct game state
        let replayed_events = event_store.replay_events_for_game(game_id);
        let reconstructed_view = GameView::from_events(&replayed_events);
        
        // Then: Game state is correctly reconstructed
        assert_eq!(reconstructed_view.game_id, Some(game_id));
        assert_eq!(reconstructed_view.player_name, Some("Eve".to_string()));
        assert_eq!(reconstructed_view.current_score, 10);
        assert_eq!(reconstructed_view.turn_count, 2);
        assert!(!reconstructed_view.is_active); // Game ended
        assert!(reconstructed_view.is_game_over());
    }

    #[test]
    fn given_invalid_player_name_when_start_game_then_error_returned() {
        // Given: Invalid player name (empty)
        let empty_name = "".to_string();
        
        // When: Attempting to start game with invalid name
        let result = commands::start_game(empty_name.clone());
        
        // Then: Error is returned
        assert!(result.is_err());
        match result.unwrap_err() {
            crate::GameError::InvalidPlayerName(name) => {
                assert_eq!(name, empty_name);
            }
            _ => panic!("Expected InvalidPlayerName error"),
        }
    }

    #[test]
    fn given_game_not_started_when_take_turn_then_error_returned() {
        // Given: Game not started (empty view)
        let game_view = GameView::new();
        
        // When: Attempting to take turn without starting game
        let result = commands::take_turn(&game_view);
        
        // Then: Error is returned
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), crate::GameError::GameNotStarted);
    }

    #[test]
    fn given_complete_game_flow_when_events_applied_then_matches_original_implementation() {
        // Given: A complete game flow using Event Modeling
        let mut event_store = EventStore::new();
        let mut game_view = GameView::new();
        
        // When: Playing a complete game
        // Start game
        let start_events = commands::start_game("Player".to_string()).unwrap();
        event_store.append_events(start_events.clone()).unwrap();
        for event in &start_events {
            game_view.apply_event(event);
        }
        
        let mut final_score = 0;
        
        // Play 5 turns
        for _turn in 1..=5 {
            if !game_view.is_game_over() {
                let turn_events = commands::take_turn(&game_view).unwrap();
                event_store.append_events(turn_events.clone()).unwrap();
                
                for event in &turn_events {
                    game_view.apply_event(event);
                }
                
                final_score = game_view.current_score;
            }
        }
        
        // Then: Game behavior matches expected patterns
        assert!(game_view.is_game_over());
        assert_eq!(game_view.turn_count, 5);
        assert_eq!(game_view.current_score, final_score);
        
        // And: Event store contains complete history
        let all_events = event_store.get_events();
        assert!(!all_events.is_empty());
        
        // Verify first event is GameStarted
        match &all_events[0] {
            GameEvent::GameStarted { player_name, .. } => {
                assert_eq!(player_name, "Player");
            }
            _ => panic!("First event should be GameStarted"),
        }
        
        // Verify last event is GameEnded
        let last_event = all_events.last().unwrap();
        match last_event {
            GameEvent::GameEnded { final_score: end_score, turn_count, .. } => {
                assert_eq!(*end_score, final_score);
                assert_eq!(*turn_count, 5);
            }
            _ => panic!("Last event should be GameEnded"),
        }
    }
}