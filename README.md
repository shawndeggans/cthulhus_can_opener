# Cthulhu's Can Opener - Event Modeling in Rust

> **A reference implementation of Adam Dymitruk's Event Modeling methodology in Rust**

This project demonstrates how to implement Event Modeling from the ground up, transforming a simple text-based game into a fully event-sourced system with CQRS architecture. It serves as both a working example and a practical reference for implementing Event Modeling in your own projects.

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)]()
[![Rust Version](https://img.shields.io/badge/rust-1.70.0+-blue.svg)]()
[![Event Modeling](https://img.shields.io/badge/methodology-Event%20Modeling-orange.svg)]()

## Table of Contents

- [What is Event Modeling?](#what-is-event-modeling)
- [Why Event Modeling?](#why-event-modeling)
- [The Game](#the-game)
- [Event Modeling Implementation](#event-modeling-implementation)
- [Architecture Overview](#architecture-overview)
- [Code Structure](#code-structure)
- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Testing Strategy](#testing-strategy)
- [Extending the System](#extending-the-system)
- [Reference for Your Next Project](#reference-for-your-next-project)
- [Files and Documentation](#files-and-documentation)

## What is Event Modeling?

Event Modeling is a methodology created by [Adam Dymitruk](https://eventmodeling.org/) for designing information systems through visual storytelling. It uses just **4 building blocks** arranged in **4 patterns** to create a complete system blueprint that serves as the single source of truth.

### The 4 Building Blocks

1. **🟠 Events (Orange)** - Business facts that changed system state
   - Named in past tense: `Game Started`, `Score Updated`, `Game Ended`
   - Immutable facts that drive all other elements
   - Example: [`GameEvent` enum](src/events.rs)

2. **🔵 Commands (Blue)** - Intentions to change system state  
   - Named as imperative verbs: `Start Game`, `Take Turn`
   - Triggered by user actions, result in events
   - Example: [`commands.rs`](src/commands.rs)

3. **🟢 Views (Green)** - Queries that read and present data
   - Named as data projections: `Current Score`, `Game Status`
   - Built from event data, feed display logic
   - Example: [`GameView` struct](src/views.rs)

4. **⚪ Wireframes (White)** - UI mockups showing user interactions
   - Show realistic interface layouts
   - Connect user actions to commands
   - Example: CLI prompts and displays

### The 4 Patterns

1. **Command Pattern:** `Wireframe → Command → Event(s)`
   - User interaction triggers business change
   - Example: Welcome Screen → Start Game → Game Started

2. **View Pattern:** `Event(s) → View → Wireframe`  
   - Business changes update displays
   - Example: Score Updated → Current Score → Score Display

3. **Automation Pattern:** `Event(s) → View → Command → Event(s)`
   - System reactions to business events
   - Example: Take Turn → Random Event → Score Update

4. **Translation Pattern:** `Event(s) → View → Command → Event(s)`
   - Integration between system boundaries
   - Example: Game Ended → Final Score → Save High Score

## Why Event Modeling?

**Simplicity scales, complexity kills.** Event Modeling provides:

- **📊 Visual Blueprint**: Single source of truth that anyone can understand
- **🔄 Event Sourcing**: Complete audit trail and state reconstruction
- **⚡ CQRS**: Clean separation of commands and queries
- **🧪 Testability**: Every pattern becomes a Given-When-Then test
- **🎯 Focus**: Business logic separated from technical concerns
- **🚀 Velocity**: Independent development slices, no dependencies

## The Game

Cthulhu's Can Opener is a simple text-based adventure where players:
1. Start a game with their name
2. Take 5 turns, each revealing a random cosmic event
3. Score points by finding artifacts or the legendary can opener
4. View their final score

While simple, it demonstrates all Event Modeling patterns and serves as a perfect learning vehicle for the methodology.

## Event Modeling Implementation

### Visual Model

Our complete Event Model is visualized in [`docs/event-modeling.drawio`](docs/event-modeling.drawio):

```
Timeline: Game Started → Random Event → Score Updated → Game Ended

Patterns:
┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│ Welcome     │───▶│ Start Game  │───▶│ Game        │
│ Screen      │    │ (Command)   │    │ Started     │
│ (Wireframe) │    │             │    │ (Event)     │
└─────────────┘    └─────────────┘    └─────────────┘

┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│ Score       │◀───│ Current     │◀───│ Score       │
│ Display     │    │ Score       │    │ Updated     │
│ (Wireframe) │    │ (View)      │    │ (Event)     │
└─────────────┘    └─────────────┘    └─────────────┘
```

### From Visual Model to Code

Every element in our visual model maps directly to code:

**Events (Orange)** → [`events.rs`](src/events.rs)
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameEvent {
    GameStarted { metadata: EventMetadata, player_name: String },
    RandomEventOccurred { metadata: EventMetadata, event_type: EventType, score_change: i32, description: String },
    ScoreUpdated { metadata: EventMetadata, new_score: u32, previous_score: u32 },
    GameEnded { metadata: EventMetadata, final_score: u32, turn_count: u32 },
}
```

**Commands (Blue)** → [`commands.rs`](src/commands.rs)
```rust
pub fn start_game(player_name: String) -> GameResult<Vec<GameEvent>>
pub fn take_turn(game_view: &GameView) -> GameResult<Vec<GameEvent>>
```

**Views (Green)** → [`views.rs`](src/views.rs)
```rust
pub struct GameView {
    pub current_score: u32,
    pub turn_count: u32,
    pub is_active: bool,
    // ... projection methods
}
```

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                        main.rs                              │
│                   (Application Layer)                       │
├─────────────────────────────────────────────────────────────┤
│  commands.rs     │  views.rs        │  events.rs           │
│  (Write Side)    │  (Read Side)     │  (Event Definitions) │
│                  │                  │                      │
│  • start_game    │  • GameView      │  • GameEvent enum    │
│  • take_turn     │  • apply_event   │  • EventMetadata     │
│  • end_game      │  • display       │  • Factory methods   │
├─────────────────────────────────────────────────────────────┤
│                       store.rs                              │
│                   (Event Store)                             │
│                                                             │
│  • append          • replay         • get_events          │
│  • event_history   • projections    • persistence         │
└─────────────────────────────────────────────────────────────┘
```

### Event Sourcing

Events are the source of truth. Game state is reconstructed by replaying events:

```rust
// Event sourcing in action
let events = event_store.replay_events_for_game(game_id);
let current_state = GameView::from_events(&events);
```

### CQRS (Command Query Responsibility Segregation)

- **Commands** change state by generating events
- **Views** read state by projecting from events
- Complete separation of read/write concerns

## Code Structure

```
src/
├── main.rs              # Application entry point, Event Modeling patterns
├── lib.rs               # Module organization and exports
├── events.rs            # Event definitions and metadata
├── commands.rs          # Command handlers (write side)
├── views.rs             # View projections (read side) 
├── store.rs             # Event store implementation
├── errors.rs            # Error types and handling
└── integration_tests.rs # Given-When-Then test scenarios

docs/
├── event-modeling.drawio      # Visual Event Model
├── event-modeling-for-claude.md   # Comprehensive guide
└── event-modeling-setup.md   # Quick setup instructions

CLAUDE.md                # Event Modeling methodology documentation
```

## Getting Started

### Prerequisites

- Rust 1.70.0+ 
- Optional: Draw.io for viewing/editing the Event Model

### Build and Run

```bash
# Clone and build
git clone <repository-url>
cd cthulhus_can_opener
cargo build

# Run the game
cargo run

# Run with event history debug output
DEBUG=1 cargo run

# Run all tests
cargo test
```

### Example Output

```
Welcome to Cthulhu's Can Opener, Player!

--- Turn 1 ---
You found Cthulhu's can opener! +10 points
Current score: 10

--- Turn 2 ---
You discovered an ancient artifact. +5 points
Current score: 15

...

Game over! Final score: 25
```

## Development Workflow

### 1. Visual-First Design

Start with the Event Model in [`docs/event-modeling.drawio`](docs/event-modeling.drawio):
1. Identify business events (orange)
2. Add commands and views (blue/green)
3. Connect with wireframes (white)
4. Arrange in timeline

### 2. Test-Driven Implementation

Generate Given-When-Then tests from the Event Model:

```rust
#[test]
fn given_no_game_when_start_game_command_then_game_started_event() {
    // Given: No game exists
    let mut event_store = EventStore::new();
    
    // When: Start Game command
    let events = commands::start_game("Alice".to_string()).unwrap();
    
    // Then: Game Started event
    assert!(matches!(events[0], GameEvent::GameStarted { .. }));
}
```

### 3. Slice-Based Development

Implement features as independent vertical slices:
- Each pattern = one slice
- No dependencies between slices
- Complete from UI to persistence

### 4. Continuous Verification

Ensure code matches the Event Model:
- Events map to visual events
- Commands implement visual commands  
- Views project visual views
- Tests verify visual patterns

## Testing Strategy

### Test Pyramid

1. **Unit Tests** (22 tests)
   - Individual component behavior
   - Error handling and edge cases
   - Pure function validation

2. **Integration Tests** (16 tests)  
   - Given-When-Then scenarios from Event Model
   - End-to-end pattern verification
   - Event sourcing and projection testing

3. **Property Tests**
   - Event replay consistency
   - State reconstruction accuracy
   - Business rule invariants

### Example Test Patterns

```rust
// Command Pattern Test
given_welcome_screen_when_start_game_then_game_started_event()

// View Pattern Test  
given_score_updated_when_view_projected_then_display_updated()

// Automation Pattern Test
given_take_turn_when_random_event_then_score_updated()

// Event Sourcing Test
given_event_history_when_replayed_then_state_reconstructed()
```

## Extending the System

### Adding New Features

1. **Update Event Model** first in Draw.io
2. **Add new events** to `events.rs`
3. **Create command handlers** in `commands.rs`
4. **Build view projections** in `views.rs`
5. **Write Given-When-Then tests**
6. **Update main application flow**

### Example: Adding Player Levels

```rust
// 1. New events
LevelUp { player_level: u32, threshold_reached: u32 }

// 2. New command
pub fn check_level_up(current_score: u32) -> GameResult<Vec<GameEvent>>

// 3. New view projection
pub struct PlayerView {
    pub level: u32,
    pub experience_points: u32,
}

// 4. Integration into automation pattern
Score Updated → Check Level → Level Up (if threshold reached)
```

## Reference for Your Next Project

### Reusable Patterns

**Event Store Pattern**
```rust
// Copy and adapt for any domain
pub struct EventStore<T> {
    events: Vec<T>,
}

impl<T> EventStore<T> {
    pub fn append(&mut self, event: T) -> Result<(), StoreError>
    pub fn replay_events(&self) -> Vec<&T>
}
```

**Command Handler Pattern**
```rust
// Pure functions that return events
pub fn handle_command(state: &View, params: Params) -> Result<Vec<Event>, Error>
```

**View Projection Pattern**
```rust
// State projections from events
impl View {
    pub fn from_events(events: &[Event]) -> Self
    pub fn apply_event(&mut self, event: &Event)
}
```

### Scaling Considerations

- **Multiple Aggregates**: Each gets its own event stream
- **Snapshots**: For performance with large event histories  
- **Sagas**: For cross-aggregate coordination
- **Event Versioning**: For schema evolution
- **Persistence**: Replace in-memory store with database

### Best Practices Learned

✅ **Do:**
- Start with the visual Event Model
- Keep events as immutable business facts
- Separate commands from queries completely
- Test every pattern with Given-When-Then
- Use events as the single source of truth

❌ **Don't:**
- Put technical details in the Event Model
- Mix business logic with presentation
- Create dependencies between slices
- Skip the visual modeling step
- Let code diverge from the model

## Files and Documentation

| File | Purpose | Event Modeling Concept |
|------|---------|----------------------|
| [`docs/event-modeling.drawio`](docs/event-modeling.drawio) | Visual Event Model | Single source of truth |
| [`CLAUDE.md`](CLAUDE.md) | Comprehensive methodology guide | Implementation patterns |
| [`src/events.rs`](src/events.rs) | Event definitions | Business facts (Orange) |
| [`src/commands.rs`](src/commands.rs) | Command handlers | User intentions (Blue) |
| [`src/views.rs`](src/views.rs) | View projections | Data queries (Green) |
| [`src/store.rs`](src/store.rs) | Event persistence | Event sourcing |
| [`src/main.rs`](src/main.rs) | Application flow | UI wireframes (White) |
| [`src/integration_tests.rs`](src/integration_tests.rs) | Given-When-Then tests | Pattern verification |

## Dependencies

| Crate | Purpose | Event Modeling Usage |
|-------|---------|---------------------|
| `chrono` | Date/time handling | Event timestamps |
| `serde` | Serialization | Event persistence |
| `uuid` | Unique identifiers | Event/aggregate IDs |
| `rand` | Random number generation | Game mechanics |

## Contributing

This project serves as a reference implementation. To contribute:

1. Ensure changes align with Event Modeling principles
2. Update the visual model first
3. Maintain test coverage for all patterns
4. Keep documentation synchronized with code

## License

MIT OR Apache-2.0

---

**🎯 Key Takeaway**: This project demonstrates that Event Modeling scales from simple games to complex systems. The visual model remains your single source of truth, the code stays aligned with business requirements, and development proceeds in predictable, independent slices.

Use this as a template for your next Event Modeling project! 🚀
