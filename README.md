# Valorant Match Menubar App

A beautiful macOS menu bar app built with Tauri and React that displays live Valorant esports match results.

## Features

- 🎮 Live match results from vlrggapi
- 🎨 Modern React UI with dark theme
- 📱 Menu bar tray icon integration
- ⚡ Fast development with Vite
- 🔄 Auto-refresh every 30 seconds
- 🧪 Test mode for development

## Quick Start

### Prerequisites

- Node.js 18+ and npm
- Rust (install from https://rustup.rs/)
- Tauri CLI: `cargo install tauri-cli --locked`

### Installation

```bash
# Install Node dependencies
npm install

# Install Rust dependencies (automatic on first build)
cd src-tauri
cargo build
```

### Development

```bash
# Start Vite dev server (Terminal 1)
npm run dev

# Start Tauri app (Terminal 2)
cd src-tauri
cargo tauri dev
```

### Build

```bash
# Build React app
npm run build

# Build Tauri app
cd src-tauri
cargo tauri build
```

## Project Structure

- `src/` - React frontend
- `src-tauri/` - Rust backend
- `src/components/` - React components

## Configuration

### Test Mode

Edit `src-tauri/src/main.rs`:

```rust
const USE_TEST_DATA: bool = true;  // Use mock data
const USE_TEST_DATA: bool = false; // Use real API
```

## Documentation

- [React Setup Guide](./REACT_SETUP.md)
- [UI/UX Customization Guide](./UI_UX_GUIDE.md)
- [Tray Menu Guide](./TRAY_MENU_GUIDE.md)
