# Event Modeling for Claude Code

## Overview

This guide enables Claude Code to facilitate Event Modeling - Adam Dymitruk's methodology for designing information systems - in a single-developer GitHub Codespace environment using Draw.io. Event Modeling creates visual blueprints that serve as the single source of truth for system development.

**Core Philosophy:** Simplicity scales, complexity kills. Event Modeling takes 15 minutes to learn and eliminates traditional software methodology overhead through visual storytelling and independent work slices.

## Environment Setup

**Prerequisites for Codespace:**
- Draw.io extension installed and configured
- Claude Code running in terminal
- Git repository initialized
- This CLAUDE.md file present in project root

**Draw.io Configuration:**
- Use landscape orientation
- Enable snap-to-grid for alignment
- Set up color palette: Orange (Events), Blue (Commands), Green (Views), White (Wireframes)
- Create template with basic shapes pre-configured

## Event Modeling Process

### The 4 Building Blocks
1. **Events (Orange rectangles)** - Business facts that changed system state
2. **Commands (Blue rectangles)** - Intentions to change system state  
3. **Views (Green rectangles)** - Queries that read and present data
4. **Wireframes (White boxes)** - UI mockups showing user interactions

### The 4 Patterns
1. **Command Pattern:** Wireframe → Command → Event(s)
2. **View Pattern:** Event(s) → View → Wireframe
3. **Automation Pattern:** Event(s) → View → Command → Event(s) 
4. **Translation Pattern:** Event(s) → View → Command → Event(s) (between systems)

### The 7-Step Process
1. **Brainstorm** - Identify all business events
2. **Plot** - Arrange events chronologically  
3. **Storyboard** - Add wireframes above timeline
4. **Inputs** - Add blue command boxes
5. **Outputs** - Add green view boxes
6. **Conway's Law** - Organize into swimlanes
7. **Elaborate** - Create Given-When-Then specs

## Claude Code Integration

### Custom Slash Commands

**Setup Commands Directory:**
```bash
mkdir -p .claude/commands
```

**Available Commands:**
- `/em-start` - Initialize new Event Model
- `/em-brainstorm` - Guide through event brainstorming
- `/em-plot` - Help arrange events chronologically
- `/em-storyboard` - Add wireframes and UI flows
- `/em-commands` - Identify input commands
- `/em-views` - Identify output views  
- `/em-swimlanes` - Apply Conway's Law organization
- `/em-specs` - Generate Given-When-Then specifications
- `/em-slice` - Define development slice from model
- `/em-validate` - Review model against principles

### Development Workflow

**Phase 1: Event Model Creation**
```
/em-start
# Claude guides through Draw.io setup and initial model creation
```

**Phase 2: Iterative Refinement**
```
/em-brainstorm
/em-plot  
/em-storyboard
# Continue through 7 steps with Claude guidance
```

**Phase 3: Implementation Planning**
```
/em-slice
# Convert visual model into development slices
# Each slice = one vertical feature from UI to persistence
```

**Phase 4: Code Generation**
```
# Claude uses Event Model as blueprint for:
# - API endpoint definitions
# - Database schema
# - Frontend components
# - Test specifications
```

## Working with Draw.io Files

**File Organization:**
- Store `.drawio` files in `/docs/event-models/` directory
- Use descriptive names: `user-registration.drawio`, `order-processing.drawio`
- Version control all model files with code

**Claude Code Integration:**
- Claude can read and interpret Draw.io XML format
- Models serve as input for code generation
- Changes to models trigger code updates
- Models remain single source of truth

## Development Slice Concepts

**What is a Slice:**
- Smallest possible work unit for one developer
- Complete vertical functionality from UI to database
- Independent and isolated implementation
- Defined by explicit contracts between components

**Slice Identification:**
- Each pattern (Command/View/Automation/Translation) = one slice
- Slices map directly to development tasks
- No dependencies between slices
- Can be implemented in any order

**Slice Implementation:**
1. Generate API contract from Event Model
2. Implement backend slice (command/view handlers)
3. Create frontend slice (UI components)
4. Write Given-When-Then tests
5. Deploy and validate

## Anti-Patterns to Avoid

**Do NOT add:**
- Technical implementation details to models
- Branching logic or decision trees
- Multiple methodologies or processes
- Framework-specific artifacts
- Complex inheritance hierarchies

**Stay minimal:**
- Use only the 4 building blocks
- Follow only the 4 patterns
- Stick to the 7-step process
- Resist gold-plating and over-engineering

## Code Generation Guidelines

When generating code from Event Models:

**API Layer:**
- Create RESTful endpoints matching command/view patterns
- Use event names for resource identification
- Implement CQRS separation (commands vs queries)
- Generate OpenAPI specifications from models

**Database Layer:**
- Create event store tables matching event definitions
- Generate read model tables from view requirements
- Implement event sourcing if specified
- Create projection handlers for views

**Frontend Layer:**
- Generate React components from wireframe specifications
- Create forms matching command parameters
- Build displays matching view outputs
- Implement navigation flows from storyboard

**Testing Layer:**
- Generate Given-When-Then test scenarios
- Create integration tests for each slice
- Build UI tests from wireframe interactions
- Validate API contracts match model

## Quality Assurance

**Model Validation Checklist:**
- [ ] Events are named as past-tense business facts
- [ ] Commands are named as intentions (imperative verbs)
- [ ] Views are named as data projections/queries
- [ ] Wireframes show realistic UI layouts
- [ ] Timeline flows logically from left to right
- [ ] Swimlanes represent autonomous system parts
- [ ] No branching logic in main flow
- [ ] All UI fields have data sources identified

**Implementation Validation:**
- [ ] Each slice can be deployed independently
- [ ] API matches model specifications exactly
- [ ] Database schema supports event storage
- [ ] Frontend implements wireframe layouts
- [ ] Tests cover all Given-When-Then scenarios
- [ ] Model remains synchronized with code

## Troubleshooting

**Common Issues:**
- **Model too complex:** Return to core 4 patterns, eliminate branching
- **Unclear data flow:** Trace from UI fields back to event sources
- **Technical details creeping in:** Focus on business events only
- **Slice dependencies:** Redesign for true independence
- **Implementation diverging:** Use model as single source of truth

**Claude Code Support:**
- Use `/em-validate` to check model against principles
- Ask Claude to trace data flows through patterns
- Request slice independence verification
- Get code generation aligned with model specifications

## Success Metrics

**Event Model Quality:**
- Can explain system behavior to any stakeholder in 15 minutes
- Visual model serves as complete implementation blueprint
- No external documentation needed beyond the model
- Changes to requirements reflected in model first

**Development Efficiency:**
- Slices deliverable in 1-3 days each
- No work dependencies between team members
- Fixed velocity based on empirical slice completion
- Zero rework due to misunderstood requirements

## Templates and Examples

**Basic Command Pattern:**
```
[User Registration Wireframe] → [Register User Command] → [User Registered Event]
```

**Basic View Pattern:**
```
[User Registered Event] → [User Profile View] → [Profile Display Wireframe]
```

**Automation Pattern:**
```
[Order Placed Event] → [Pending Orders View] → [Send Confirmation Email Command] → [Email Sent Event]
```

Remember: Event Modeling is about visual storytelling that everyone can understand. Keep it simple, keep it visual, and let the model be your single source of truth.