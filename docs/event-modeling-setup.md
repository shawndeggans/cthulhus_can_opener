# Event Modeling with Claude Code - Quick Setup

## Installation Steps

### 1. Set Up Your Codespace
```bash
# Open your GitHub repository in a Codespace
# Install Draw.io extension from VS Code marketplace
# Ensure Claude Code is available (should be pre-installed)
```

### 2. Initialize Claude Code for Event Modeling
```bash
# In your codespace terminal
cd your-project-directory
claude

# In Claude Code REPL
/init
```

### 3. Create Event Modeling Directory Structure
```bash
mkdir -p docs/event-models
mkdir -p .claude/commands
mkdir -p slices
```

### 4. Add the CLAUDE.md File
Copy the "Event Modeling for Claude Code - Complete Implementation Guide" content into your project's `CLAUDE.md` file.

### 5. Install Custom Commands
Create these files in `.claude/commands/`:
- `em-start.md` - Initialize Event Modeling session
- `em-brainstorm.md` - Guide event discovery
- `em-slice.md` - Generate development slices  
- `em-validate.md` - Validate model quality

(Use the command content provided in the artifacts above)

### 6. Configure Draw.io
In your codespace:
1. Install Draw.io integration extension
2. Create template with Event Modeling shapes
3. Set up color palette: Orange (Events), Blue (Commands), Green (Views), White (Wireframes)

## Quick Start Workflow

### 1. Start Event Modeling Session
```bash
# In Claude Code
/em-start
```
Claude will guide you through:
- Creating a new Draw.io file
- Setting up the modeling canvas
- Defining the business scenario

### 2. Model Your System
Follow the 7-step process with Claude's guidance:
```bash
/em-brainstorm    # Step 1: Discover business events
/em-plot          # Step 2: Arrange chronologically  
/em-storyboard    # Step 3: Add wireframes
/em-commands      # Step 4: Identify inputs
/em-views         # Step 5: Identify outputs
/em-swimlanes     # Step 6: Apply Conway's Law
/em-specs         # Step 7: Create Given-When-Then specs
```

### 3. Generate Implementation Slices
```bash
/em-slice
```
Claude will convert your visual model into:
- Independent development slices
- API specifications
- Database schemas
- Frontend component definitions
- Test scenarios

### 4. Validate and Implement
```bash
/em-validate      # Check model quality
```
Then begin implementing slices in priority order.

## Key Benefits

**For Single Developer with Claude Code:**
- Visual model serves as complete implementation blueprint
- Claude Code generates scaffolding from Event Models
- Independent slices enable focused development
- No ceremony overhead - just visual modeling and coding
- Model stays synchronized with implementation

**Integration Points:**
- Draw.io files version controlled with code
- Claude Code reads models for code generation
- Given-When-Then specs become executable tests
- Slices map directly to development tasks

## Success Indicators

You'll know it's working when:
- You can explain your system to anyone using the visual model
- Claude Code generates accurate code from your models
- Development proceeds in independent, predictable slices
- Requirements changes are reflected in the model first
- The model remains your single source of truth

Start with `/em-start` and let Claude Code guide you through the Adam Dymitruk minimalist approach to system design!