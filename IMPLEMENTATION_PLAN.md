# YM2151 Log Editor - Implementation Plan

## Project Overview
A TUI-based JSON editor for YM2151 event logs, written in Rust for Windows. The application provides visualization and editing capabilities for YM2151 synthesizer event data.

## Goals (Jumpstart)
1. ✅ Create foundational project structure (reference: ym2151-tone-editor)
2. ✅ Create implementation plan document (this document)
3. ✅ Implement simple initial Rust implementation

## Features Implemented (v0.1.0)

### Core Functionality
- ✅ JSON file loading from command line argument
- ✅ Display YM2151 events in TUI format
- ✅ Convert register 0x08 events to "KeyON" display format
- ✅ Time display toggle (T key) between cumulative time and timestamp
- ✅ JSON saving functionality (saves with internal timestamp format)
- ✅ Basic navigation (Up/Down arrow keys)

### UI Components
- ✅ Header showing filename and current time display mode
- ✅ Content area showing event list with scrolling
- ✅ Footer showing keyboard shortcuts
- ✅ Selection highlighting

## Technology Stack
- **Language**: Rust (edition 2021)
- **TUI Framework**: ratatui 0.29
- **Terminal Backend**: crossterm 0.29
- **Serialization**: serde 1.0 + serde_json 1.0
- **Platform**: Cross-platform (with Windows focus)

## Architecture

### Module Structure
```
src/
├── main.rs          - Entry point, terminal setup, event loop
├── app.rs           - Application state and business logic
├── models.rs        - Data structures (Ym2151Event, Ym2151Log)
└── ui.rs            - UI rendering logic
```

### Data Models

#### Ym2151Event
- `time`: f64 - Absolute timestamp in seconds
- `addr`: String - Register address (hex)
- `data`: String - Register data value (hex)

#### Ym2151Log
- `events`: Vec<Ym2151Event> - List of YM2151 events

#### TimeDisplayMode
- `Cumulative` - Shows delta time from previous event (for editing)
- `Timestamp` - Shows absolute time from start (internal format for saving)

## Key Features Detail

### 1. KeyON Display Conversion
Events with address `0x08` are displayed as "KeyON" instead of the register address to improve readability. This register controls note on/off operations in YM2151.

Format: `{time}  KeyON  {data}`

### 2. Time Display Toggle (T Key)
Two modes for time visualization:

**Cumulative Mode** (default):
- Shows wait time (delta) from previous event
- Easier for editing wait times between events
- Format: `0.010000  40  16` (10ms after previous event)

**Timestamp Mode**:
- Shows absolute time from start
- Internal format used for JSON storage
- Format: `0.010000  40  16` (10ms from start)

Toggle with `T` key during runtime.

### 3. JSON File Operations

**Loading**:
```bash
ym2151-log-editor path/to/file.json
```

**Saving**:
- Press `S` key to save
- Always saves with timestamp format (absolute time)
- Maintains JSON structure and formatting

## User Interface

### Keyboard Controls
| Key | Action |
|-----|--------|
| ↑/↓ | Navigate through events |
| T | Toggle time display mode |
| S | Save current file |
| Q/ESC | Quit application |

### Display Layout
```
┌─────────────────────────────────────┐
│ YM2151 Log Editor - filename        │
│ Time: Cumulative/Timestamp          │
├─────────────────────────────────────┤
│ Event list (scrollable)             │
│ > 0.000000  20  4F                  │
│   0.010000  KeyON  78               │
│   ...                               │
├─────────────────────────────────────┤
│ ↑/↓: Navigate | T: Toggle | S: Save│
└─────────────────────────────────────┘
```

## Future Enhancements (Not Yet Implemented)

### Phase 2: Editing Capabilities
- [ ] Edit time values (increase/decrease with +/- keys)
- [ ] Edit register address and data values
- [ ] Insert new events
- [ ] Delete events
- [ ] Undo/Redo functionality

### Phase 3: Advanced Features
- [ ] Batch time adjustment (scale all times)
- [ ] Event filtering and search
- [ ] Copy/paste events
- [ ] Event templates
- [ ] Multiple file support / tabs

### Phase 4: Quality of Life
- [ ] Configuration file support
- [ ] Recent files list
- [ ] Keyboard shortcuts customization
- [ ] Color themes
- [ ] Help screen (F1)

### Phase 5: Integration
- [ ] Integration with ym2151-log-play-server for audio preview
- [ ] Import from other formats
- [ ] Export to different formats
- [ ] Conversion utilities

## Build and Run

### Prerequisites
- Rust 1.70 or later
- Windows (or any platform supporting crossterm)

### Build
```bash
cargo build --release
```

### Run
```bash
# With file argument
cargo run -- path/to/file.json

# Or run compiled binary
./target/release/ym2151-log-editor path/to/file.json
```

### Development
```bash
# Run in development mode
cargo run -- test_data/sample.json

# Run tests (when implemented)
cargo test

# Check code
cargo clippy
cargo fmt
```

## Testing Strategy

### Current Test Data
- `test_data/sample.json` - Sample YM2151 log with various events

### Future Test Coverage
- [ ] Unit tests for time conversion logic
- [ ] Unit tests for KeyON detection
- [ ] Integration tests for file loading/saving
- [ ] UI interaction tests
- [ ] Edge cases (empty files, malformed JSON, etc.)

## Known Limitations

1. **No Time Editing**: Current version only displays, cannot edit time values
2. **Read-only Data**: Cannot modify register addresses or data values
3. **No Undo**: No undo/redo functionality
4. **Single File**: Can only work with one file at a time
5. **No Audio Preview**: No integration with YM2151 playback server yet

## Dependencies

### Core Dependencies
```toml
ratatui = "0.29"      # TUI framework
crossterm = "0.29"    # Terminal backend
serde = "1.0"         # Serialization framework
serde_json = "1.0"    # JSON support
```

## References
- [ym2151-tone-editor](https://github.com/cat2151/ym2151-tone-editor) - Reference implementation
- [YM2151 (OPM) Datasheet](https://github.com/cat2151/ym2151-tone-editor/) - For register specifications
- [ratatui Documentation](https://ratatui.rs/) - TUI framework docs

## Version History

### v0.1.0 (Current - Jumpstart)
- Initial project structure
- Basic JSON loading and display
- KeyON conversion display
- Time mode toggle (Cumulative/Timestamp)
- Basic navigation
- File saving functionality

## Conclusion

This jumpstart implementation provides a solid foundation for the YM2151 Log Editor. The core visualization and file handling features are working, and the architecture is designed to easily accommodate future enhancements for editing capabilities.

The main use case of "JSON visualization and easier time field editing" has a strong foundation with the time toggle feature, though actual editing of time values will be added in Phase 2.
