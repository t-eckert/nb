# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**NotaBene (nb)** is a personal command-line tool for managing a markdown notebook. The user maintains their notebook in `~/Notebook` using Obsidian and Neovim. This is a personal tool built for one user's specific workflow.

## Development Commands

```bash
# Build the project
cargo build

# Run the tool
cargo run -- <command>

# Build and install locally
cargo install --path .

# Common test commands
cargo run -- log edit
cargo run -- log edit --yesterday
cargo run -- log edit --tomorrow
cargo run -- log rollover
cargo run -- log weather
cargo run -- log weather --days 5
```

## Architecture

### Module Structure

The codebase follows a strict separation of concerns:

```
src/
├── main.rs      # Minimal entry point (24 lines) - routing only
├── cli.rs       # Clap CLI definitions
└── notebook.rs  # All notebook operations and business logic
```

**Keep `main.rs` minimal** - it should only parse CLI args and route to the appropriate `notebook::` function.

### CLI Design Philosophy

Commands follow a strict **`<noun> <verb>`** pattern:
- ✅ `nb log edit`
- ✅ `nb log rollover`
- ❌ `nb edit-log`

Currently only the `log` subcommand is implemented. Other planned subcommands (`schedule`, `server`, `agent`) should NOT be added to `cli.rs` until they're ready to implement.

### Notebook Integration

The `notebook` module interacts with the user's notebook directory:

**Default Location:** `~/Notebook` (override with `$NOTEBOOK_PATH`)

**Directory Structure:**
- `~/Notebook/Log/` - Daily logs in format `YYYY-MM-DD.md`
- `~/Notebook/+Templates/Daily Note.md` - Template for new logs

**Template System:**
- Uses Obsidian-style date placeholders: `{{date:ddd D MMMM YYYY}}`
- Creates logs with proper section headers: `## Meetings and Events`, `## Work`, `## Personal`, `## Notes`
- TODO format: `- [ ]` for unchecked, `- [x]` for checked

**Environment Variables:**
- `$NOTEBOOK_PATH` - Custom notebook location
- `$EDITOR` - User's preferred editor (defaults to `vim`)

**Weather Integration:**
- Uses OpenWeather API One Call 3.0 for forecasts
- API key retrieved from 1Password: `op://Private/Open Weather Map/Notebook API Key`
- Location: Ottawa, Ontario (lat: 45.4215, lon: -75.6972)
- Weather format: `Weather: High [max]°C, Low [min]°C. [Description].`
- Automatic alerts for cold weather (< -10°C) and precipitation
- Weather line inserted after the date heading, before first section

### Adding New `log` Subcommands

When adding new actions to the `log` subcommand:

1. Add variant to `LogAction` enum in `src/cli.rs`
2. Implement public function in `src/notebook.rs`
3. Add match arm in `src/main.rs` routing to the new function
4. Keep main.rs minimal - all logic stays in `notebook.rs`

## Git Workflow

- Use **conventional commits** format: `<type>(<scope>): <subject>`
- Common types: `feat`, `fix`, `refactor`, `docs`, `test`, `chore`
- Do NOT include Claude attribution footers in commits
- Example: `feat(log): add list command to show recent logs`

## Current Implementation Status

### Implemented
- `nb log edit` - Edit today's, yesterday's, or tomorrow's log
- `nb log view` - View a daily log to stdout
- `nb log rollover` - Roll over unchecked TODOs from today to tomorrow
- `nb log list` - List recent logs with TODO completion stats
- `nb log weather` - Add weather forecasts to upcoming logs (default: 7 days)

### Planned (not yet in code)
- Additional `log` subcommands (search, stats, etc.)
- `schedule` subcommand (task planning)
- `server` subcommand (HTML/JSON export)
- `agent` subcommand (AI integration)

**Important:** Only implement features one subcommand at a time. Finish all `log` features before moving to other subcommands.
