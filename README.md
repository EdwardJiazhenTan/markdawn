# Real-time Markdown Renderer

A real-time markdown renderer built with Rust, featuring live preview updates similar to React development environment hot reloading.

## Features

- **Live Preview**: Browser updates immediately when markdown files are saved
- **WebSocket Communication**: Real-time bidirectional communication between server and client
- **File Monitoring**: Automatic detection of markdown file changes with debouncing
- **Multi-client Support**: Multiple browser windows update synchronously
- **Auto-reconnection**: Automatic WebSocket reconnection on network issues

## Technology Stack

- **Backend**: Rust + Axum + WebSocket + notify crate
- **Frontend**: HTML + JavaScript + WebSocket API
- **Parser**: Custom markdown parser supporting headers, paragraphs, bold, and italic text

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Cargo package manager

### Installation and Usage

1. Clone the repository:
```bash
git clone https://github.com/EdwardJiazhenTan/markdawn.git
cd markdawn
```

2. Start the server:
```bash
cargo run
```

3. Open your browser and navigate to:
```
http://localhost:5000
```

4. Test the live updates:
   - Edit any `.md` file in the project directory
   - Save the file
   - Watch the browser update automatically

## Project Structure

```
src/
├── main.rs          # Main server and HTTP routes
├── websocket.rs     # WebSocket connection management
├── watcher.rs       # File system monitoring
├── parser.rs        # Markdown parser implementation
├── renderer.rs      # HTML rendering from parsed markdown
├── events.rs        # Event type definitions
└── data.rs          # Data structures for markdown elements

static/
├── index.html       # Frontend interface
└── style.css        # Styling

test.md              # Sample markdown file for testing
README.md            # This file
```

## Architecture

The system follows an event-driven architecture:

```
File Change -> File Watcher -> Markdown Parser -> HTML Renderer -> WebSocket Broadcast -> Browser Update
```

### Key Components

1. **File Watcher**: Monitors markdown files for changes using the `notify` crate
2. **WebSocket Manager**: Handles multiple client connections and broadcasts updates
3. **Markdown Parser**: Custom parser supporting basic markdown syntax
4. **Event System**: Type-safe event handling for file changes and client updates

## Supported Markdown Syntax

Currently supports:
- Headers (H1-H6): `# Header`
- Paragraphs
- Bold text: `**bold**`
- Italic text: `*italic*`

## Development

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Development Mode

For development, you can watch a specific file:

```bash
# Edit the watcher.rs to use watch_single_file() method
# Then run cargo run and edit your target markdown file
```

## Configuration

The server runs on `localhost:5000` by default. File watching includes:
- Debounce duration: 300ms
- Recursive directory monitoring
- Markdown file filtering (`.md` extension)

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

This project is open source. Feel free to use and modify as needed.

## Acknowledgments

Built as a Rust learning project exploring:
- Async programming with Tokio
- WebSocket implementation with Axum
- File system monitoring
- Custom parser development
- Real-time web applications