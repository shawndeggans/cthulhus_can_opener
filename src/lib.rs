pub mod commands;
pub mod errors;
pub mod events;
pub mod store;
pub mod views;

// Test modules
#[cfg(test)]
pub mod integration_tests;

// Legacy module - to be removed
pub mod game;

// Re-exports for convenience
pub use commands::*;
pub use errors::*;
pub use events::*;
pub use store::*;
pub use views::*;

// Legacy export for backwards compatibility
pub use game::Game;