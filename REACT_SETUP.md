# React Integration Guide

This guide explains how React is integrated into your Tauri app and how to work with it.

## Project Structure

```
tauri-app/
├── src/                    # React source files
│   ├── main.jsx          # React entry point
│   ├── App.jsx            # Main App component
│   ├── App.css            # App styles
│   ├── index.css          # Global styles
│   └── components/         # React components
│       ├── MatchCard.jsx  # Match card component
│       └── MatchCard.css  # Match card styles
├── src-tauri/             # Rust/Tauri backend
│   └── src/
│       └── main.rs        # Rust code with Tauri commands
├── package.json           # Node dependencies
├── vite.config.js         # Vite configuration
└── index.html            # HTML entry point
```

## How It Works

1. **React Frontend** (`src/`):
   - Uses Vite for fast development and building
   - Calls Rust commands via `@tauri-apps/api`
   - Displays matches in a beautiful, modern UI

2. **Rust Backend** (`src-tauri/src/main.rs`):
   - Exposes `get_live_matches()` command
   - Handles tray icon clicks to show/hide window
   - Fetches data from vlrggapi

3. **Window Behavior**:
   - Click tray icon → Opens React window
   - Click again → Hides window
   - Window is transparent, always-on-top, no decorations

## Development

### Install Dependencies

```bash
npm install
```

### Run Development Server

```bash
# Terminal 1: Start Vite dev server
npm run dev

# Terminal 2: Start Tauri app (in src-tauri/)
cd src-tauri
cargo tauri dev
```

The Tauri config is set to use `http://localhost:5173` in dev mode.

### Build for Production

```bash
npm run build
cd src-tauri
cargo tauri build
```

## Customizing the UI

### Modify Styles

- **Global styles**: `src/index.css`
- **App styles**: `src/App.css`
- **Component styles**: `src/components/MatchCard.css`

### Add Components

1. Create component file: `src/components/YourComponent.jsx`
2. Import and use in `App.jsx`:

```jsx
import YourComponent from './components/YourComponent'

function App() {
  return (
    <div>
      <YourComponent />
    </div>
  )
}
```

### Call Rust Commands

```jsx
import { invoke } from '@tauri-apps/api/core'

// Call a Rust command
const result = await invoke('get_live_matches')
```

To add new commands, add them to `src-tauri/src/main.rs`:

```rust
#[tauri::command]
async fn your_command() -> Result<String, String> {
    Ok("Hello from Rust!".to_string())
}

// Register in main():
.invoke_handler(tauri::generate_handler![get_live_matches, your_command])
```

## Features

- ✅ Modern React 18 with hooks
- ✅ Vite for fast HMR (Hot Module Replacement)
- ✅ Beautiful dark theme matching Valorant aesthetic
- ✅ Responsive design
- ✅ Auto-refresh every 30 seconds
- ✅ Loading and error states
- ✅ Smooth animations and transitions

## Troubleshooting

### Window doesn't open
- Check console for errors
- Make sure Vite dev server is running (`npm run dev`)
- Verify `devUrl` in `tauri.conf.json` points to `http://localhost:5173`

### Styles not loading
- Check browser console for CSS errors
- Verify `index.css` is imported in `main.jsx`
- Clear browser cache

### Rust command not found
- Make sure command is registered in `invoke_handler`
- Check command name matches exactly (case-sensitive)
- Verify return type matches TypeScript expectations
