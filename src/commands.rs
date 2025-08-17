use crate::events::{EventType, GameEvent};
use crate::errors::{GameError, GameResult};
use crate::views::{generate_event_description, GameView};
use rand::Rng;
use uuid::Uuid;

pub fn start_game(player_name: String) -> GameResult<Vec<GameEvent>> {
    if player_name.trim().is_empty() {
        return Err(GameError::InvalidPlayerName(player_name));
    }

    let game_id = Uuid::new_v4();
    let events = vec![GameEvent::new_game_started(game_id, player_name.trim().to_string())];
    
    Ok(events)
}

pub fn take_turn(game_view: &GameView) -> GameResult<Vec<GameEvent>> {
    let game_id = game_view
        .game_id
        .ok_or(GameError::GameNotStarted)?;

    if !game_view.is_active {
        return Err(GameError::GameAlreadyEnded);
    }

    if game_view.max_turns_reached() {
        return Err(GameError::MaxTurnsReached);
    }

    let mut rng = rand::thread_rng();
    let random_value = rng.gen_range(1..=3);
    
    let (event_type, score_change) = match random_value {
        1 => (EventType::CanOpenerFound, 10),
        2 => (EventType::ArtifactDiscovered, 5), 
        _ => (EventType::NothingHappened, 0),
    };
    
    let description = generate_event_description(&event_type, score_change);
    let new_score = game_view.current_score + score_change as u32;
    let new_turn_count = game_view.turn_count + 1;
    
    let mut events = vec![
        GameEvent::new_random_event_occurred(game_id, event_type, score_change, description),
    ];
    
    if score_change > 0 {
        events.push(GameEvent::new_score_updated(game_id, new_score, game_view.current_score));
    }
    
    if new_turn_count >= 5 {
        events.push(GameEvent::new_game_ended(game_id, new_score, new_turn_count));
    }
    
    Ok(events)
}

pub fn end_game(game_view: &GameView) -> GameResult<Vec<GameEvent>> {
    let game_id = game_view
        .game_id
        .ok_or(GameError::GameNotStarted)?;

    if !game_view.is_active {
        return Err(GameError::GameAlreadyEnded);
    }

    let events = vec![GameEvent::new_game_ended(
        game_id,
        game_view.current_score,
        game_view.turn_count,
    )];
    
    Ok(events)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::GameEvent;

    #[test]
    fn test_start_game_success() {
        let result = start_game("Alice".to_string());
        assert!(result.is_ok());
        
        let events = result.unwrap();
        assert_eq!(events.len(), 1);
        
        if let GameEvent::GameStarted { player_name, .. } = &events[0] {
            assert_eq!(player_name, "Alice");
        } else {
            panic!("Expected GameStarted event");
        }
    }

    #[test]
    fn test_start_game_with_whitespace() {
        let result = start_game("  Bob  ".to_string());
        assert!(result.is_ok());
        
        let events = result.unwrap();
        if let GameEvent::GameStarted { player_name, .. } = &events[0] {
            assert_eq!(player_name, "Bob");
        } else {
            panic!("Expected GameStarted event");
        }
    }

    #[test]
    fn test_start_game_empty_name() {
        let result = start_game("".to_string());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), GameError::InvalidPlayerName("".to_string()));
    }

    #[test]
    fn test_start_game_whitespace_only_name() {
        let result = start_game("   ".to_string());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), GameError::InvalidPlayerName("   ".to_string()));
    }

    #[test]
    fn test_take_turn_game_not_started() {
        let view = GameView::new();
        let result = take_turn(&view);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), GameError::GameNotStarted);
    }

    #[test]
    fn test_take_turn_game_ended() {
        let mut view = GameView::new();
        view.game_id = Some(Uuid::new_v4());
        view.is_active = false;
        
        let result = take_turn(&view);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), GameError::GameAlreadyEnded);
    }

    #[test]
    fn test_take_turn_max_turns_reached() {
        let mut view = GameView::new();
        view.game_id = Some(Uuid::new_v4());
        view.is_active = true;
        view.turn_count = 5;
        
        let result = take_turn(&view);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), GameError::MaxTurnsReached);
    }

    #[test]
    fn test_take_turn_success() {
        let mut view = GameView::new();
        view.game_id = Some(Uuid::new_v4());
        view.is_active = true;
        view.current_score = 5;
        view.turn_count = 2;
        
        let result = take_turn(&view);
        assert!(result.is_ok());
        
        let events = result.unwrap();
        assert!(!events.is_empty());
        
        if let GameEvent::RandomEventOccurred { event_type, score_change, .. } = &events[0] {
            match event_type {
                EventType::CanOpenerFound => assert_eq!(*score_change, 10),
                EventType::ArtifactDiscovered => assert_eq!(*score_change, 5),
                EventType::NothingHappened => assert_eq!(*score_change, 0),
            }
        } else {
            panic!("Expected RandomEventOccurred event");
        }
    }

    #[test]
    fn test_take_turn_final_turn() {
        let mut view = GameView::new();
        view.game_id = Some(Uuid::new_v4());
        view.is_active = true;
        view.current_score = 20;
        view.turn_count = 4; // This will be the 5th turn
        
        let result = take_turn(&view);
        assert!(result.is_ok());
        
        let events = result.unwrap();
        
        let has_game_ended = events.iter().any(|event| {
            matches!(event, GameEvent::GameEnded { .. })
        });
        assert!(has_game_ended, "Game should end after 5th turn");
    }

    #[test]
    fn test_end_game_success() {
        let mut view = GameView::new();
        view.game_id = Some(Uuid::new_v4());
        view.is_active = true;
        view.current_score = 15;
        view.turn_count = 3;
        
        let result = end_game(&view);
        assert!(result.is_ok());
        
        let events = result.unwrap();
        assert_eq!(events.len(), 1);
        
        if let GameEvent::GameEnded { final_score, turn_count, .. } = &events[0] {
            assert_eq!(*final_score, 15);
            assert_eq!(*turn_count, 3);
        } else {
            panic!("Expected GameEnded event");
        }
    }

    #[test]
    fn test_end_game_not_started() {
        let view = GameView::new();
        let result = end_game(&view);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), GameError::GameNotStarted);
    }

    #[test]
    fn test_end_game_already_ended() {
        let mut view = GameView::new();
        view.game_id = Some(Uuid::new_v4());
        view.is_active = false;
        
        let result = end_game(&view);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), GameError::GameAlreadyEnded);
    }
}