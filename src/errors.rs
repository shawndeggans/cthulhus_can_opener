use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum GameError {
    GameAlreadyStarted,
    GameNotStarted,
    GameAlreadyEnded,
    InvalidPlayerName(String),
    MaxTurnsReached,
    InvalidTurnCount(u32),
    EventStoreError(String),
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameError::GameAlreadyStarted => write!(f, "Game has already been started"),
            GameError::GameNotStarted => write!(f, "Game has not been started yet"),
            GameError::GameAlreadyEnded => write!(f, "Game has already ended"),
            GameError::InvalidPlayerName(name) => write!(f, "Invalid player name: '{}'", name),
            GameError::MaxTurnsReached => write!(f, "Maximum number of turns (5) has been reached"),
            GameError::InvalidTurnCount(count) => write!(f, "Invalid turn count: {}", count),
            GameError::EventStoreError(msg) => write!(f, "Event store error: {}", msg),
        }
    }
}

impl std::error::Error for GameError {}

pub type GameResult<T> = Result<T, GameError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let error = GameError::InvalidPlayerName("".to_string());
        assert_eq!(error.to_string(), "Invalid player name: ''");
    }

    #[test]
    fn test_game_result_type() {
        let result: GameResult<u32> = Ok(42);
        assert_eq!(result.unwrap(), 42);
    }
}