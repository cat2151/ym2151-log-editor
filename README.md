# ym2151-log-editor

YM2151 Event Log Editor (TUI Interface). Written in Rust.

<p align="left">
  <a href="README.ja.md"><img src="https://img.shields.io/badge/🇯🇵-Japanese-red.svg" alt="Japanese"></a>
  <a href="README.md"><img src="https://img.shields.io/badge/🇺🇸-English-blue.svg" alt="English"></a>
</p>

## Status

- Untested (as far as I know)

### Installation

```
cargo install --force --git https://github.com/cat2151/ym2151-log-editor
```

## Overview

This is a terminal-based JSON editor for YM2151 synthesizer event logs. It assists with visualizing and editing YM2151 event data, with a particular focus on timing adjustment and event inspection.

## Features

- **JSON Visualization**: Displays YM2151 event logs in a readable format
- **KeyON Display**: Shows events for register 0x08 as "KeyON" for better readability
- **Time Display Modes**: Toggle between cumulative time (wait) and absolute timestamps
  - Cumulative Mode: Displays delta time between events (convenient for editing)
  - Timestamp Mode: Displays absolute time from start (internal storage format)
- **Navigation**: Browse events using arrow keys
- **File Operations**: Load and save JSON files

## Quick Start

### Installation

Rust 1.70 or later is required.

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
# Launch the editor with a file
./target/release/ym2151-log-editor your_log.json

# Check for updates
./target/release/ym2151-log-editor check

# Update this application via cargo install
./target/release/ym2151-log-editor update

# If the filename is 'check' or 'update', specify it as a path to launch
./target/release/ym2151-log-editor ./check
```

### Keyboard Shortcuts

| Key | Action |
|-----|-----|
| ↑/↓ or k/j | Move between events |
| PageUp / PageDown | Jump 10 lines |
| Ctrl+U / Ctrl+D | Jump 10 lines |
| 9k / 9j | Jump 9 lines (vim numeric prefix) |
| / or ENTER | Insert a new event before the current line |
| DELETE | Delete the current event |
| 0-9 | Set wait time (0-9 milliseconds, cumulative mode only) |
| P | Preview playback (play the entire JSON) |
| L | Toggle loop playback (interactive mode) |
| T | Toggle time display mode (Cumulative ↔ Timestamp) |
| S | Save file |
| Q or ESC | Exit application |

## JSON Format

The editor handles YM2151 event logs in the following JSON format:

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
0.000000  20  4F    ← First event (time 0)
0.010000  40  16    ← 10ms after the previous event
0.010000  KeyON  78 ← 10ms after the previous event
```

### Timestamp Mode
Displays the absolute time from the start. This is the internal format used when saving files.

Example:
```
0.000000  20  4F    ← 0ms from start
0.010000  40  16    ← 10ms from start
0.020000  KeyON  78 ← 20ms from start
```

Press the **T** key to switch between these modes.

## KeyON Display

Events for register 0x08 (KeyON/KeyOFF register) are displayed as "KeyON" instead of "08" to improve readability:

```
0.010000  KeyON  78  ← Easily identify KeyON/KeyOFF events
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

Sample test data is available in `test_data/sample.json`:

```bash
cargo run -- test_data/sample.json
```

## Documentation

Refer to [IMPLEMENTATION_PLAN.md](IMPLEMENTATION_PLAN.md) for detailed implementation plans and future roadmap.

## Dependencies

- [ratatui](https://ratatui.rs/) 0.29 - Terminal UI framework
- [crossterm](https://github.com/crossterm-rs/crossterm) 0.29 - Terminal backend
- [serde](https://serde.rs/) 1.0 - Serialization framework
- [serde_json](https://github.com/serde-rs/json) 1.0 - JSON support

## Related Projects

- [ym2151-tone-editor](https://github.com/cat2151/ym2151-tone-editor) - YM2151 Tone Editor (reference implementation)
- [ym2151-log-play-server](https://github.com/cat2151/ym2151-log-play-server) - YM2151 Log Playback Server

## License

Refer to the [LICENSE](LICENSE) file for details.

## Future Enhancements

- Editing register addresses and data values for inserted events
- Undo/Redo functionality
- Audio preview integration
- Batch time scaling
- Event filtering and searching

For a complete roadmap, refer to [IMPLEMENTATION_PLAN.md](IMPLEMENTATION_PLAN.md).

## Project Goals

- Minimal `wait` editing
- Minimal event editing
- Minimal event visualization

## Out of Scope (Non-Goals)

- Advanced features.
  - Event insertion via MML input.
  - Advanced visualization. Parallel display of 8 channels. Achieving all features found in DAW event editors and beyond.
  - DAW features. Piano roll display and editing. Staff notation display and editing. Data automation display and editing. Event insertion via MIDI IN.
  - Querying. Advanced editing achievable by querying events. Filtering and editing only notes, specific events, etc., and highly intelligent automatic event dependency recognition that preserves event relationships even with event additions/deletions.
  - Decompilation. Highly intelligent inverse transformation of event content into SMF or MML by analyzing event data. Calculating BPM, measure, beat, and tick from time with 100% success rate.
