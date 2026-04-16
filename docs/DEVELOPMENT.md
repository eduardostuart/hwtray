# Development

## Quick Start

```bash
mise run setup     # Install hooks and dependencies
mise run dev       # Desktop app with hot reload
```

## Commands

```bash
mise run dev       # Desktop app with hot reload
mise run test      # Run all tests (Rust + frontend)
mise run lint      # ESLint + Clippy
mise run lint:fix  # Auto-fix + fmt
mise run build     # Production build
mise run ci        # Full CI checks
```

## Project Structure

- `crates/homewizard-api/` — Types + HTTP client for HomeWizard local API
- `crates/homewizard-core/` — mDNS discovery, polling, config persistence
- `crates/homewizard-desktop/` — Tauri app, tray, IPC commands, polling loop
- `src/` — Vue 3 frontend (Composition API, Pinia, Tailwind CSS v4)
