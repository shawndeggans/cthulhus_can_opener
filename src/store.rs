use crate::events::GameEvent;
use crate::errors::GameResult;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct EventStore {
    events: Vec<GameEvent>,
}

impl EventStore {
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
        }
    }

    pub fn append(&mut self, event: GameEvent) -> GameResult<()> {
        self.events.push(event);
        Ok(())
    }

    pub fn append_events(&mut self, events: Vec<GameEvent>) -> GameResult<()> {
        for event in events {
            self.append(event)?;
        }
        Ok(())
    }

    pub fn get_events(&self) -> &[GameEvent] {
        &self.events
    }

    pub fn get_events_for_game(&self, game_id: Uuid) -> Vec<&GameEvent> {
        self.events
            .iter()
            .filter(|event| event.game_id() == game_id)
            .collect()
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }

    pub fn replay_events_for_game(&self, game_id: Uuid) -> Vec<GameEvent> {
        self.get_events_for_game(game_id)
            .into_iter()
            .cloned()
            .collect()
    }
}

impl Default for EventStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::GameEvent;

    #[test]
    fn test_event_store_creation() {
        let store = EventStore::new();
        assert!(store.is_empty());
        assert_eq!(store.len(), 0);
    }

    #[test]
    fn test_append_event() {
        let mut store = EventStore::new();
        let game_id = Uuid::new_v4();
        let event = GameEvent::new_game_started(game_id, "Player".to_string());
        
        assert!(store.append(event).is_ok());
        assert_eq!(store.len(), 1);
        assert!(!store.is_empty());
    }

    #[test]
    fn test_get_events_for_game() {
        let mut store = EventStore::new();
        let game_id1 = Uuid::new_v4();
        let game_id2 = Uuid::new_v4();
        
        let event1 = GameEvent::new_game_started(game_id1, "Player1".to_string());
        let event2 = GameEvent::new_game_started(game_id2, "Player2".to_string());
        
        store.append(event1).unwrap();
        store.append(event2).unwrap();
        
        let game1_events = store.get_events_for_game(game_id1);
        assert_eq!(game1_events.len(), 1);
        
        let game2_events = store.get_events_for_game(game_id2);
        assert_eq!(game2_events.len(), 1);
    }

    #[test]
    fn test_replay_events() {
        let mut store = EventStore::new();
        let game_id = Uuid::new_v4();
        
        let event1 = GameEvent::new_game_started(game_id, "Player".to_string());
        let event2 = GameEvent::new_score_updated(game_id, 10, 0);
        
        store.append(event1).unwrap();
        store.append(event2).unwrap();
        
        let replayed = store.replay_events_for_game(game_id);
        assert_eq!(replayed.len(), 2);
    }

    #[test]
    fn test_clear_store() {
        let mut store = EventStore::new();
        let game_id = Uuid::new_v4();
        let event = GameEvent::new_game_started(game_id, "Player".to_string());
        
        store.append(event).unwrap();
        assert!(!store.is_empty());
        
        store.clear();
        assert!(store.is_empty());
    }
}