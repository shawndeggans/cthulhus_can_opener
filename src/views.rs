use crate::events::{EventType, GameEvent};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct GameView {
    pub game_id: Option<Uuid>,
    pub player_name: Option<String>,
    pub current_score: u32,
    pub turn_count: u32,
    pub is_active: bool,
    pub last_event_description: Option<String>,
}

impl GameView {
    pub fn new() -> Self {
        Self {
            game_id: None,
            player_name: None,
            current_score: 0,
            turn_count: 0,
            is_active: false,
            last_event_description: None,
        }
    }

    pub fn from_events(events: &[GameEvent]) -> Self {
        let mut view = Self::new();
        for event in events {
            view.apply_event(event);
        }
        view
    }

    pub fn apply_event(&mut self, event: &GameEvent) {
        match event {
            GameEvent::GameStarted { metadata, player_name } => {
                self.game_id = Some(metadata.game_id);
                self.player_name = Some(player_name.clone());
                self.current_score = 0;
                self.turn_count = 0;
                self.is_active = true;
                self.last_event_description = Some(format!("Welcome to Cthulhu's Can Opener, {}!", player_name));
            }
            GameEvent::RandomEventOccurred { description, .. } => {
                self.turn_count += 1;
                self.last_event_description = Some(description.clone());
            }
            GameEvent::ScoreUpdated { new_score, .. } => {
                self.current_score = *new_score;
            }
            GameEvent::GameEnded { final_score, turn_count, .. } => {
                self.current_score = *final_score;
                self.turn_count = *turn_count;
                self.is_active = false;
                self.last_event_description = Some(format!("Game over! Final score: {}", final_score));
            }
        }
    }

    pub fn get_welcome_message(&self) -> String {
        match &self.player_name {
            Some(name) => format!("Welcome to Cthulhu's Can Opener, {}!", name),
            None => "Welcome to Cthulhu's Can Opener!".to_string(),
        }
    }

    pub fn get_turn_display(&self) -> String {
        format!("--- Turn {} ---", self.turn_count + 1)
    }

    pub fn get_event_description(&self) -> String {
        self.last_event_description
            .as_ref()
            .cloned()
            .unwrap_or_else(|| "Nothing has happened yet.".to_string())
    }

    pub fn get_score_display(&self) -> String {
        format!("Current score: {}", self.current_score)
    }

    pub fn get_final_score_display(&self) -> String {
        format!("Game over! Final score: {}", self.current_score)
    }

    pub fn is_game_over(&self) -> bool {
        !self.is_active || self.turn_count >= 5
    }

    pub fn max_turns_reached(&self) -> bool {
        self.turn_count >= 5
    }
}

impl Default for GameView {
    fn default() -> Self {
        Self::new()
    }
}

pub fn generate_event_description(event_type: &EventType, score_change: i32) -> String {
    match event_type {
        EventType::CanOpenerFound => {
            format!("You found Cthulhu's can opener! +{} points", score_change)
        }
        EventType::ArtifactDiscovered => {
            format!("You discovered an ancient artifact. +{} points", score_change)
        }
        EventType::NothingHappened => {
            "The cosmic horror grows... but nothing happens.".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::EventType;

    #[test]
    fn test_game_view_creation() {
        let view = GameView::new();
        assert_eq!(view.current_score, 0);
        assert_eq!(view.turn_count, 0);
        assert!(!view.is_active);
        assert!(view.game_id.is_none());
        assert!(view.player_name.is_none());
    }

    #[test]
    fn test_apply_game_started_event() {
        let mut view = GameView::new();
        let game_id = Uuid::new_v4();
        let event = GameEvent::new_game_started(game_id, "Alice".to_string());
        
        view.apply_event(&event);
        
        assert_eq!(view.game_id, Some(game_id));
        assert_eq!(view.player_name, Some("Alice".to_string()));
        assert_eq!(view.current_score, 0);
        assert_eq!(view.turn_count, 0);
        assert!(view.is_active);
    }

    #[test]
    fn test_apply_score_updated_event() {
        let mut view = GameView::new();
        let game_id = Uuid::new_v4();
        let event = GameEvent::new_score_updated(game_id, 10, 0);
        
        view.apply_event(&event);
        
        assert_eq!(view.current_score, 10);
    }

    #[test]
    fn test_apply_game_ended_event() {
        let mut view = GameView::new();
        view.is_active = true;
        let game_id = Uuid::new_v4();
        let event = GameEvent::new_game_ended(game_id, 25, 5);
        
        view.apply_event(&event);
        
        assert_eq!(view.current_score, 25);
        assert_eq!(view.turn_count, 5);
        assert!(!view.is_active);
    }

    #[test]
    fn test_from_events() {
        let game_id = Uuid::new_v4();
        let events = vec![
            GameEvent::new_game_started(game_id, "Bob".to_string()),
            GameEvent::new_score_updated(game_id, 15, 0),
            GameEvent::new_game_ended(game_id, 15, 3),
        ];
        
        let view = GameView::from_events(&events);
        
        assert_eq!(view.game_id, Some(game_id));
        assert_eq!(view.player_name, Some("Bob".to_string()));
        assert_eq!(view.current_score, 15);
        assert_eq!(view.turn_count, 3);
        assert!(!view.is_active);
    }

    #[test]
    fn test_display_methods() {
        let mut view = GameView::new();
        view.player_name = Some("Charlie".to_string());
        view.current_score = 20;
        view.turn_count = 2;
        
        assert_eq!(view.get_welcome_message(), "Welcome to Cthulhu's Can Opener, Charlie!");
        assert_eq!(view.get_turn_display(), "--- Turn 3 ---");
        assert_eq!(view.get_score_display(), "Current score: 20");
        assert_eq!(view.get_final_score_display(), "Game over! Final score: 20");
    }

    #[test]
    fn test_game_state_checks() {
        let mut view = GameView::new();
        
        view.is_active = true;
        view.turn_count = 3;
        assert!(!view.is_game_over());
        assert!(!view.max_turns_reached());
        
        view.turn_count = 5;
        assert!(view.is_game_over());
        assert!(view.max_turns_reached());
        
        view.is_active = false;
        view.turn_count = 2;
        assert!(view.is_game_over());
        assert!(!view.max_turns_reached());
    }

    #[test]
    fn test_event_description_generation() {
        assert_eq!(
            generate_event_description(&EventType::CanOpenerFound, 10),
            "You found Cthulhu's can opener! +10 points"
        );
        
        assert_eq!(
            generate_event_description(&EventType::ArtifactDiscovered, 5),
            "You discovered an ancient artifact. +5 points"
        );
        
        assert_eq!(
            generate_event_description(&EventType::NothingHappened, 0),
            "The cosmic horror grows... but nothing happens."
        );
    }
}