# ym2151-log-editor

YM2151 Event Log Editor (TUI Interface). Written in Rust.

<p align="left">
  <a href="README.ja.md"><img src="https://img.shields.io/badge/🇯🇵-Japanese-red.svg" alt="Japanese"></a>
  <a href="README.md"><img src="https://img.shields.io/badge/🇺🇸-English-blue.svg" alt="English"></a>
</p>

## Status

- Untested (to my knowledge)
- Installation

```
cargo install --force --git https://github.com/cat2151/ym2151-log-editor
```

## Overview

This is a terminal-based JSON editor for YM2151 synthesizer event logs. It assists in visualizing and editing YM2151 event data, with a particular focus on timing adjustment and event inspection.

## Features

- **JSON Visualization**: Displays YM2151 event logs in an easy-to-read format
- **KeyON Display**: Events for register 0x08 are displayed as "KeyON" for improved clarity
- **Time Display Mode**: Toggle between cumulative time (wait) and absolute timestamps
  - Cumulative Mode: Shows delta time between events (useful for editing)
  - Timestamp Mode: Shows absolute time from the start (internal storage format)
- **Navigation**: Browse events using arrow keys
- **File Operations**: Load and save JSON files

## Quick Start

### Installation

Requires Rust 1.70 or later.

```bash
# Clone the repository
git clone https://github.com/cat2151/ym2151-log-editor.git
cd ym2151-log-editor

# Build
cargo build --release

# Run with a specified JSON file
cargo run -- path/to/your/file.json
```

### Usage

```bash
# Launch editor with specified file
./target/release/ym2151-log-editor your_log.json

# Check for updates
./target/release/ym2151-log-editor check

# Update this app via cargo install
./target/release/ym2151-log-editor update

# If filename is 'check' / 'update', launch by explicitly providing path
./target/release/ym2151-log-editor ./check
```

### Keyboard Controls

| Key | Action |
|-----|-----|
| ↑/↓ | Move between events |
| / or ENTER | Insert a new event before the current line |
| DELETE | Delete the current event |
| 0-9 | Set wait time (0-9 milliseconds, cumulative mode only) |
| P | Preview playback (play entire JSON) |
| L | Toggle loop playback (interactive mode) |
| I | Import JSON from clipboard |
| T | Toggle time display mode (Cumulative ↔ Timestamp) |
| S | Save file |
| Q or ESC | Exit application |

## JSON Format

The editor handles YM2151 event logs in JSON format:

```json
{
  "events": [
    {
      "time": 0.0,
      "addr": "20",
      "data": "4F"
    },
    {
      "time": 0.01,
      "addr": "08",
      "data": "78"
    }
  ]
}
```

- `time`: Absolute timestamp (in seconds)
- `addr`: YM2151 register address (hexadecimal)
- `data`: Register data value (hexadecimal)

## Time Display Modes

### Cumulative Mode (Default)
Displays the wait time (delta) from the previous event. This is useful for editing timing, allowing you to check and adjust delays between events.

Example:
```
0.000000  20  4F    # First event (time 0)
0.010000  40  16    # 10ms after previous
0.010000  KeyON  78 # 10ms after previous
```

### Timestamp Mode
Displays the absolute time from the start. This is the internal format used when saving files.

Example:
```
0.000000  20  4F    # 0ms from start
0.010000  40  16    # 10ms from start
0.020000  KeyON  78 # 20ms from start
```

Press the **T** key to toggle between these modes.

## KeyON Display

Events for register 0x08 (KeyON/KeyOFF register) are displayed as "KeyON" instead of "08" to improve readability:

```
0.010000  KeyON  78  # Easily identify KeyON/OFF events
0.500000  KeyON  00
```

## Development

### Project Structure

```
src/
├── main.rs       - Entry point and event loop
├── app.rs        - Application state and logic
├── models.rs     - Data structures (Ym2151Event, Ym2151Log)
└── ui.rs         - UI rendering
```

### Build

```bash
cargo build          # Development build
cargo build --release # Optimized build
```

### Testing

Sample test data is provided in `test_data/sample.json`:

```bash
cargo run -- test_data/sample.json
```

## Documentation

For detailed implementation plans and future roadmap, please refer to [IMPLEMENTATION_PLAN.md](IMPLEMENTATION_PLAN.md).

## Dependencies

- [ratatui](https://ratatui.rs/) 0.29 - Terminal UI framework
- [crossterm](https://github.com/crossterm-rs/crossterm) 0.29 - Terminal backend
- [serde](https://serde.rs/) 1.0 - Serialization framework
- [serde_json](https://github.com/serde-rs/json) 1.0 - JSON support

## Related Projects

- [ym2151-tone-editor](https://github.com/cat2151/ym2151-tone-editor) - YM2151 Tone Editor (reference implementation)
- [ym2151-log-play-server](https://github.com/cat2151/ym2151-log-play-server) - YM2151 Log Play Server

## License

See the [LICENSE](LICENSE) file for details.

## Future Enhancements

- Editing register address and data values of inserted events
- Undo/Redo functionality
- Audio preview integration
- Bulk time scaling
- Event filtering and search

For the complete roadmap, refer to [IMPLEMENTATION_PLAN.md](IMPLEMENTATION_PLAN.md).

## Project Goals

- Minimum wait time editing
- Minimum event editing
- Minimum event visualization

## Out of Scope (What the Project Does NOT Aim For)

- High functionality.
  - Event insertion via MML input.
  - Advanced visualization. Parallel display of 8 channels. Achieving more than all features found in DAW event editors.
  - DAW features. Piano roll display and editing. Staff notation display and editing. Data automation display and editing. Event insertion via MIDI IN.
  - Querying. Advanced editing achievable by querying events. Filtering and editing only notes, specific events, etc., and highly intelligent automatic event dependency recognition that prevents breaking dependencies between events even with event additions or deletions.
  - Decompilation. Highly intelligent reverse conversion of event content to SMF or MML. Calculating BPM, measure, beat, and tick from time with 100% success rate.