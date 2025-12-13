# ym2151-log-editor

YM2151 event log editor with TUI interface. Written in Rust.

## Overview

A terminal-based JSON editor for YM2151 synthesizer event logs. This tool helps visualize and edit YM2151 event data, with special focus on timing adjustments and event inspection.

## Features

- **JSON Visualization**: Display YM2151 event logs in a readable format
- **KeyON Display**: Events on register 0x08 are displayed as "KeyON" for clarity
- **Time Display Modes**: Toggle between cumulative time (wait) and absolute timestamp
  - Cumulative mode: Shows delta time between events (useful for editing)
  - Timestamp mode: Shows absolute time from start (internal storage format)
- **Navigation**: Browse through events with arrow keys
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

# Run with a JSON file
cargo run -- path/to/your/file.json
```

### Usage

```bash
# Run the editor with a file
./target/release/ym2151-log-editor your_log.json
```

### Keyboard Controls

| Key | Action |
|-----|--------|
| ↑/↓ | Navigate through events |
| T | Toggle time display mode (Cumulative ↔ Timestamp) |
| S | Save file |
| Q or ESC | Quit application |

## JSON Format

The editor works with YM2151 event logs in JSON format:

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

- `time`: Absolute timestamp in seconds
- `addr`: YM2151 register address (hexadecimal)
- `data`: Register data value (hexadecimal)

## Time Display Modes

### Cumulative Mode (Default)
Shows the wait time (delta) from the previous event. This is useful when editing timing, as you can see and adjust the delay between events.

Example:
```
0.000000  20  4F    ← First event at time 0
0.010000  40  16    ← 10ms after previous
0.010000  KeyON  78 ← 10ms after previous
```

### Timestamp Mode
Shows the absolute time from the start. This is the internal format used when saving files.

Example:
```
0.000000  20  4F    ← At 0ms from start
0.010000  40  16    ← At 10ms from start
0.020000  KeyON  78 ← At 20ms from start
```

Press **T** to toggle between these modes.

## KeyON Display

Events on register 0x08 (KeyON/KeyOFF register) are displayed as "KeyON" instead of "08" for better readability:

```
0.010000  KeyON  78  ← Easy to identify key on/off events
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

### Building

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

See [IMPLEMENTATION_PLAN.md](IMPLEMENTATION_PLAN.md) for detailed implementation plan and future roadmap.

## Dependencies

- [ratatui](https://ratatui.rs/) 0.29 - Terminal UI framework
- [crossterm](https://github.com/crossterm-rs/crossterm) 0.29 - Terminal backend
- [serde](https://serde.rs/) 1.0 - Serialization framework
- [serde_json](https://github.com/serde-rs/json) 1.0 - JSON support

## Related Projects

- [ym2151-tone-editor](https://github.com/cat2151/ym2151-tone-editor) - YM2151 tone editor (reference implementation)
- [ym2151-log-play-server](https://github.com/cat2151/ym2151-log-play-server) - YM2151 log playback server

## License

See [LICENSE](LICENSE) file for details.

## Future Enhancements

- Edit time values (increase/decrease)
- Edit register addresses and data values
- Insert/delete events
- Undo/redo functionality
- Audio preview integration
- Batch time scaling
- Event filtering and search

See [IMPLEMENTATION_PLAN.md](IMPLEMENTATION_PLAN.md) for the complete roadmap.