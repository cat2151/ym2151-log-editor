# ym2151-log-editor

YM2151 event log editor (TUI interface). Written in Rust.

<p align="left">
  <a href="README.ja.md"><img src="https://img.shields.io/badge/🇯🇵-Japanese-red.svg" alt="Japanese"></a>
  <a href="README.md"><img src="https://img.shields.io/badge/🇺🇸-English-blue.svg" alt="English"></a>
</p>

## Overview

A terminal-based JSON editor for YM2151 synthesizer event logs. It assists in visualizing and editing YM2151 event data, with a particular focus on timing adjustment and event inspection.

## Features

- **JSON Visualization**: Displays YM2151 event logs in an easy-to-read format.
- **KeyON Display**: Events for register 0x08 are displayed as "KeyON" for improved clarity.
- **Time Display Modes**: Toggle between cumulative time (wait) and absolute timestamps.
  - Cumulative mode: Shows delta time between events (convenient for editing).
  - Timestamp mode: Shows absolute time from the start (internal storage format).
- **Navigation**: Browse events using arrow keys.
- **File Operations**: Load and save JSON files.

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
# Start the editor with a specified file
./target/release/ym2151-log-editor your_log.json
```

### Keyboard Controls

| Key     | Action                          |
|---------|---------------------------------|
| ↑/↓     | Move between events             |
| / or ENTER | Insert a new event before the current line |
| DELETE  | Delete the current event        |
| 0-9     | Set wait time (0-9 milliseconds, cumulative mode only) |
| P       | Preview playback (plays the entire JSON) |
| L       | Toggle loop playback (interactive mode) |
| T       | Toggle time display mode (Cumulative ↔ Timestamp) |
| S       | Save file                       |
| Q or ESC | Exit application                |

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

- `time`: Absolute timestamp (in seconds).
- `addr`: YM2151 register address (hexadecimal).
- `data`: Register data value (hexadecimal).

## Time Display Modes

### Cumulative Mode (Default)
Displays the wait time (delta) from the previous event. This is useful for editing timings, allowing you to see and adjust delays between events.

Example:
```
0.000000  20  4F    ← First event (time 0)
0.010000  40  16    ← 10ms after the previous event
0.010000  KeyON  78 ← 10ms after the previous event
```

### Timestamp Mode
Displays the absolute time from the start. This is the internal format used when saving the file.

Example:
```
0.000000  20  4F    ← 0ms from start
0.010000  40  16    ← 10ms from start
0.020000  KeyON  78 ← 20ms from start
```

Press the **T** key to switch between these modes.

## KeyON Display

Events for register 0x08 (KeyON/KeyOFF register) are displayed as "KeyON" instead of "08" for improved readability:

```
0.010000  KeyON  78  ← Easily identify KeyON/OFF events
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

Refer to [IMPLEMENTATION_PLAN.md](IMPLEMENTATION_PLAN.md) for detailed implementation plans and the future roadmap.

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

- Editing register address and data values for inserted events.
- Undo/redo functionality.
- Audio preview integration.
- Batch time scaling.
- Event filtering and searching.

See [IMPLEMENTATION_PLAN.md](IMPLEMENTATION_PLAN.md) for the complete roadmap.

## Project Goals
- Minimal editing of waits.
- Minimal editing of events.
- Minimal visualization of events.

## Out of Scope (What the project does not aim for)
- High-level functionality.
  - Event insertion via MML input.
  - Advanced visualization. Parallel display of 8 channels. All features beyond what's found in a DAW's event editor.
  - DAW features. Piano roll display and editing. Staff notation display and editing. Data automation display and editing. Event insertion via MIDI IN.
  - Querying. Advanced editing enabled by querying events. Highly intelligent automatic event dependency recognition that filters and edits only notes, specific events, etc., and maintains event dependencies even when events are added or removed.
  - Decompilation. Highly intelligent reverse conversion of event content into SMF or MML. 100% success rate in calculating BPM, measure, beat, and tick from time.