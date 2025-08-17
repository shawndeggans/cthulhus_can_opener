use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EventType {
    CanOpenerFound,
    ArtifactDiscovered,
    NothingHappened,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMetadata {
    pub event_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub game_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameEvent {
    GameStarted {
        metadata: EventMetadata,
        player_name: String,
    },
    RandomEventOccurred {
        metadata: EventMetadata,
        event_type: EventType,
        score_change: i32,
        description: String,
    },
    ScoreUpdated {
        metadata: EventMetadata,
        new_score: u32,
        previous_score: u32,
    },
    GameEnded {
        metadata: EventMetadata,
        final_score: u32,
        turn_count: u32,
    },
}

impl GameEvent {
    pub fn new_game_started(game_id: Uuid, player_name: String) -> Self {
        Self::GameStarted {
            metadata: EventMetadata {
                event_id: Uuid::new_v4(),
                timestamp: Utc::now(),
                game_id,
            },
            player_name,
        }
    }

    pub fn new_random_event_occurred(
        game_id: Uuid,
        event_type: EventType,
        score_change: i32,
        description: String,
    ) -> Self {
        Self::RandomEventOccurred {
            metadata: EventMetadata {
                event_id: Uuid::new_v4(),
                timestamp: Utc::now(),
                game_id,
            },
            event_type,
            score_change,
            description,
        }
    }

    pub fn new_score_updated(game_id: Uuid, new_score: u32, previous_score: u32) -> Self {
        Self::ScoreUpdated {
            metadata: EventMetadata {
                event_id: Uuid::new_v4(),
                timestamp: Utc::now(),
                game_id,
            },
            new_score,
            previous_score,
        }
    }

    pub fn new_game_ended(game_id: Uuid, final_score: u32, turn_count: u32) -> Self {
        Self::GameEnded {
            metadata: EventMetadata {
                event_id: Uuid::new_v4(),
                timestamp: Utc::now(),
                game_id,
            },
            final_score,
            turn_count,
        }
    }

    pub fn game_id(&self) -> Uuid {
        match self {
            GameEvent::GameStarted { metadata, .. } => metadata.game_id,
            GameEvent::RandomEventOccurred { metadata, .. } => metadata.game_id,
            GameEvent::ScoreUpdated { metadata, .. } => metadata.game_id,
            GameEvent::GameEnded { metadata, .. } => metadata.game_id,
        }
    }

    pub fn timestamp(&self) -> DateTime<Utc> {
        match self {
            GameEvent::GameStarted { metadata, .. } => metadata.timestamp,
            GameEvent::RandomEventOccurred { metadata, .. } => metadata.timestamp,
            GameEvent::ScoreUpdated { metadata, .. } => metadata.timestamp,
            GameEvent::GameEnded { metadata, .. } => metadata.timestamp,
        }
    }

    pub fn event_id(&self) -> Uuid {
        match self {
            GameEvent::GameStarted { metadata, .. } => metadata.event_id,
            GameEvent::RandomEventOccurred { metadata, .. } => metadata.event_id,
            GameEvent::ScoreUpdated { metadata, .. } => metadata.event_id,
            GameEvent::GameEnded { metadata, .. } => metadata.event_id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_started_event_creation() {
        let game_id = Uuid::new_v4();
        let event = GameEvent::new_game_started(game_id, "Player".to_string());
        
        if let GameEvent::GameStarted { metadata, player_name } = event {
            assert_eq!(metadata.game_id, game_id);
            assert_eq!(player_name, "Player");
        } else {
            panic!("Expected GameStarted event");
        }
    }

    #[test]
    fn test_event_metadata_consistency() {
        let game_id = Uuid::new_v4();
        let event = GameEvent::new_score_updated(game_id, 10, 0);
        
        assert_eq!(event.game_id(), game_id);
        assert!(event.timestamp() <= Utc::now());
    }
}