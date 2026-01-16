# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/claude-code) when working with code in this repository.

## Project Overview

Superlectura Games is a Rust-based desktop application built with egui (Immediate Mode GUI) for creating educational games based on Tony Buzan's speed reading and memory techniques.

## Build and Run Commands

```bash
# Build the project
cargo build

# Build for release
cargo build --release

# Run the application
cargo run

# Run in release mode
cargo run --release

# Run tests
cargo test

# Check for compilation errors without building
cargo check

# Format code
cargo fmt

# Lint code
cargo clippy
```

## Architecture

The project follows **MVC (Model-View-Controller)** architecture:

```
src/
├── main.rs              # Entry point, SuperlecturaApp struct
├── models/              # Data structures and business logic
│   ├── game_types.rs    # AppState, GameType, Difficulty, GameState enums
│   ├── game_config.rs   # GameConfig struct
│   ├── game_result.rs   # GameResult, GameDetails
│   └── storage.rs       # GameStorage for JSON persistence
├── views/               # UI rendering
│   ├── menu_view.rs     # Main menu and game cards
│   ├── results_view.rs  # Post-game results display
│   ├── history_view.rs  # Game history view
│   └── components.rs    # Reusable UI components
├── controllers/         # State management
│   └── app_controller.rs # AppController (central state manager)
├── games/               # Game implementations
│   ├── mod.rs           # Game trait definition
│   ├── reading_speed.rs # Numeric memory game
│   ├── word_memory.rs   # Word recall game
│   ├── text_comprehension.rs # Reading comprehension
│   └── inumbs.rs        # Number sequence memory
└── utils/               # Utility functions
    └── time_format.rs   # Date/time formatting
```

## Key Patterns

- **State Machine**: `AppState` enum drives UI (`GameSelection`, `Playing(GameType)`, `Results`, `History`)
- **Game Trait**: All games implement the `Game` trait with `update()`, `is_finished()`, `get_result()`, `get_state()`
- **Immediate Mode GUI**: egui renders UI each frame based on current state
- **JSON Persistence**: Results saved to `~/.config/superlectura_games/save_data.json`

## Game Types

1. **ReadingSpeed**: Numeric memory (recall random numbers)
2. **WordMemory**: Vocabulary recall
3. **TextComprehension**: Reading comprehension with questions
4. **INumbs**: Number sequence memory (00-99 pairs)

## Key Dependencies

- **eframe/egui 0.24**: GUI framework
- **serde/serde_json**: JSON serialization
- **chrono**: Date/time handling
- **rand**: Random generation

## Code Conventions

- Spanish language in comments and user-facing text
- Module exports via `mod.rs` files
- `button_with_enter()` helper for accessibility (Enter key support)
- Configuration objects passed to game constructors
- Difficulty levels: Easy, Medium, Hard (consistent across games)

## Working with Games

To add a new game:
1. Create file in `src/games/`
2. Implement the `Game` trait
3. Add variant to `GameType` enum in `models/game_types.rs`
4. Add `GameDetails` variant for result data
5. Update `AppController` to handle the new game type
6. Add game card in `menu_view.rs`
