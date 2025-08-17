# Event Modeling with Claude Code - Cthulhu's Can Opener Project

## Project Overview

This Rust learning project implements a text-based game involving Cthulhu's can opener. The project follows Adam Dymitruk's Event Modeling methodology to design, develop, and maintain the system through visual storytelling and independent development slices.

**Core Philosophy:** Simplicity scales, complexity kills. Event Modeling provides a visual blueprint that serves as the single source of truth for system development.

## Event Modeling Setup

### Environment Configuration
- **Draw.io Integration:** Use `.drawio` files in `/docs/` directory for visual models
- **Color Palette:** Orange (Events), Blue (Commands), Green (Views), White (Wireframes)
- **File Organization:** Store event models in `/docs/event-models/` when created
- **Version Control:** All `.drawio` files are tracked with code

### Project Structure
```
/workspaces/cthulhus_can_opener/
├── src/                    # Rust source code
├── docs/                   # Documentation and event models
│   ├── event-modeling.drawio         # Current game model
│   └── event-models/                 # Future models
├── slices/                 # Development slice implementations
└── CLAUDE.md              # This file
```

## Custom Slash Commands

### Core Event Modeling Commands

**`/em-start`** - Initialize new Event Modeling session
- Create new draw.io file with proper template
- Set up modeling canvas with swimlanes
- Define business scenario and scope

**`/em-brainstorm`** - Guide event discovery process
- Identify all business events (orange rectangles)
- Focus on past-tense business facts
- Avoid technical implementation details

**`/em-plot`** - Arrange events chronologically
- Create left-to-right timeline
- Ensure logical flow of business events
- Remove any branching logic

**`/em-storyboard`** - Add wireframes and UI flows
- Place white wireframe boxes above timeline
- Show realistic UI layouts
- Connect to business events below

**`/em-commands`** - Identify input commands
- Add blue command rectangles
- Name as imperative intentions
- Connect wireframes to commands to events

**`/em-views`** - Identify output views
- Add green view rectangles
- Name as data projections/queries
- Connect events to views to wireframes

**`/em-swimlanes`** - Apply Conway's Law organization
- Group related elements into swimlanes
- Represent autonomous system boundaries
- Minimize cross-lane dependencies

**`/em-specs`** - Create Given-When-Then specifications
- Generate test scenarios from model
- Cover all command and view patterns
- Ensure traceability to visual model

### Implementation Commands

**`/em-slice`** - Define development slice from model
- Identify vertical feature slices
- Generate slice specifications
- Create implementation task breakdown

**`/em-validate`** - Review model against principles
- Check adherence to 4 building blocks
- Validate 4 patterns usage
- Ensure model quality standards

## The 4 Building Blocks

1. **Events (Orange)** - Business facts that changed system state
   - Named in past tense (e.g., "Game Started", "Score Updated")
   - Represent immutable facts
   - Drive all other elements

2. **Commands (Blue)** - Intentions to change system state
   - Named as imperative verbs (e.g., "Start Game", "Take Turn")
   - Triggered by user actions
   - Result in one or more events

3. **Views (Green)** - Queries that read and present data
   - Named as data projections (e.g., "Current Score", "Game Status")
   - Built from event data
   - Feed wireframe displays

4. **Wireframes (White)** - UI mockups showing user interactions
   - Show realistic interface layouts
   - Connect user actions to commands
   - Display data from views

## The 4 Patterns

1. **Command Pattern:** Wireframe → Command → Event(s)
   - User interaction triggers business change
   - Example: [Start Game Button] → [Start Game] → [Game Started]

2. **View Pattern:** Event(s) → View → Wireframe
   - Business changes update displays
   - Example: [Score Updated] → [Current Score] → [Score Display]

3. **Automation Pattern:** Event(s) → View → Command → Event(s)
   - System reactions to business events
   - Example: [Game Started] → [Turn Counter] → [Generate Event] → [Random Event Occurred]

4. **Translation Pattern:** Event(s) → View → Command → Event(s)
   - Integration between system boundaries
   - Example: [Game Ended] → [Final Score] → [Save High Score] → [High Score Saved]

## Development Slice Methodology

### Slice Definition
- **Smallest vertical work unit** from UI to persistence
- **Independent implementation** with no dependencies
- **Complete functionality** deployable in isolation
- **1-3 day development cycle** for single developer

### Slice Identification Process
1. Identify each pattern instance in event model
2. Group related patterns into coherent features
3. Define explicit contracts between slices
4. Ensure true independence (no shared state)

### Rust Implementation Guidelines

**Event Store:**
```rust
// Events as data structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameEvent {
    GameStarted { player_name: String, timestamp: DateTime<Utc> },
    RandomEventOccurred { event_type: EventType, score_change: i32 },
    ScoreUpdated { new_score: u32 },
    GameEnded { final_score: u32 },
}
```

**Command Handlers:**
```rust
// Commands as functions
pub fn start_game(player_name: String) -> Result<Vec<GameEvent>, GameError> {
    // Business logic here
    Ok(vec![GameEvent::GameStarted { player_name, timestamp: Utc::now() }])
}
```

**View Projections:**
```rust
// Views as state projections
#[derive(Debug, Clone)]
pub struct GameView {
    pub current_score: u32,
    pub turn_count: u32,
    pub is_active: bool,
}

impl GameView {
    pub fn apply_event(&mut self, event: &GameEvent) {
        // Update view state based on events
    }
}
```

## Code Generation from Event Models

### API Layer Generation
- Create RESTful endpoints matching command/view patterns
- Use event names for resource identification
- Implement CQRS separation (commands vs queries)
- Generate OpenAPI specifications

### Database Schema Generation
- Create event store tables for event persistence
- Generate read model tables from view requirements
- Implement event sourcing patterns
- Create projection handlers for views

### Frontend Component Generation
- Generate CLI components from wireframe specifications
- Create input forms matching command parameters
- Build displays matching view outputs
- Implement navigation flows from storyboard

### Test Generation
- Generate Given-When-Then test scenarios
- Create integration tests for each slice
- Build CLI tests from wireframe interactions
- Validate implementations match model

## Current Game Model Analysis

**Existing Events:**
- Game Created (player with score=0)
- Random Event Triggered (opener/artifact/nothing)
- Score Updated (+10/+5/+0)
- Game Ended (after 5 turns)

**Current Commands:**
- Start Game
- Take Turn

**Current Views:**
- Welcome Message
- Event Description
- Score Display
- Final Score

**Current Patterns:**
- Command: Start Game → Game Created
- Automation: Take Turn → Random Event → Score Update
- View: Score Updated → Score Display

## Quality Assurance Checklist

### Model Validation
- [ ] Events named as past-tense business facts
- [ ] Commands named as imperative intentions
- [ ] Views named as data projections
- [ ] Wireframes show realistic UI layouts
- [ ] Timeline flows logically left to right
- [ ] Swimlanes represent autonomous boundaries
- [ ] No branching logic in main flow
- [ ] All UI fields have data sources identified

### Implementation Validation
- [ ] Each slice deployable independently
- [ ] API matches model specifications exactly
- [ ] Database schema supports event storage
- [ ] Frontend implements wireframe layouts
- [ ] Tests cover all Given-When-Then scenarios
- [ ] Model remains synchronized with code

## Anti-Patterns to Avoid

**Do NOT add to models:**
- Technical implementation details
- Branching logic or decision trees
- Multiple methodologies or processes
- Framework-specific artifacts
- Complex inheritance hierarchies

**Stay minimal:**
- Use only the 4 building blocks
- Follow only the 4 patterns
- Stick to the 7-step process
- Resist gold-plating and over-engineering

## Success Metrics

### Event Model Quality
- Can explain system behavior to anyone in 15 minutes
- Visual model serves as complete implementation blueprint
- No external documentation needed beyond the model
- Changes to requirements reflected in model first

### Development Efficiency
- Slices deliverable in 1-3 days each
- No dependencies between development tasks
- Fixed velocity based on empirical slice completion
- Zero rework due to misunderstood requirements

## Working with Draw.io Files

### File Organization
- Store all `.drawio` files in version control
- Use descriptive names: `user-management.drawio`, `scoring-system.drawio`
- Keep models synchronized with implementation
- Update models before code changes

### Claude Code Integration
- Claude reads and interprets draw.io XML format
- Models serve as input for code generation
- Changes to models trigger code updates
- Models remain single source of truth for system design

## Troubleshooting

### Common Issues
- **Model too complex:** Return to core 4 patterns, eliminate branching
- **Unclear data flow:** Trace from UI fields back to event sources
- **Technical details creeping in:** Focus on business events only
- **Slice dependencies:** Redesign for true independence
- **Implementation diverging:** Use model as single source of truth

### Claude Code Support
- Use `/em-validate` to check model against principles
- Ask Claude to trace data flows through patterns
- Request slice independence verification
- Get code generation aligned with model specifications

## Project-Specific Conventions

### Rust Code Style
- Follow existing project formatting (rustfmt.toml)
- Use existing linting rules (clippy.toml)
- Maintain MSRV compatibility (1.70.0)
- Follow existing naming conventions

### Game-Specific Events
- All game events should be business-focused
- Avoid technical randomization details in models
- Focus on player experience and scoring
- Keep cosmic horror theme in event descriptions

### Testing Strategy
- Use existing test framework (assert_cmd, predicates)
- Generate CLI interaction tests from wireframes
- Create integration tests for each slice
- Validate game logic through event replay

---

*Remember: Event Modeling is about visual storytelling that everyone can understand. Keep it simple, keep it visual, and let the model be your single source of truth.*