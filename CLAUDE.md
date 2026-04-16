# HomeWizard Unofficial Desktop

macOS tray app for monitoring HomeWizard Energy devices on the local network.

## Quick Start

```bash
mise run setup     # Install hooks + dependencies
mise run dev       # Desktop app with hot reload
```

## Project Structure

- `crates/homewizard-api/` — Types + HTTP client for HomeWizard local API
- `crates/homewizard-core/` — mDNS discovery, polling, config persistence
- `crates/homewizard-desktop/` — Tauri app, tray, IPC commands, polling loop
- `src/` — Vue 3 frontend (Composition API, Pinia, Tailwind CSS v4)

## Commands

```bash
mise run dev           # Desktop app with hot reload
mise run test          # All tests (Rust + frontend)
mise run lint          # ESLint + Clippy
mise run lint:fix      # Auto-fix + fmt
mise run build         # Production build
mise run ci            # Full CI checks
```

## Architecture Rules

- Views compose components, never use Tailwind directly
- Components are atomic, use Tailwind
- `useHomeWizard` composable centralizes all device logic
- `ProductType` constants in `src/types/products.ts` — no hardcoded strings
- `fmt()` lives in `src/utils/format.ts` — not in composables
- Tauri commands expose minimal API — backend processes, frontend renders
- Telemetry flows via Tauri events (`telemetry_update`, `device_offline`, `device_online`)
- Rust stable toolchain
- ESLint: `no-explicit-any` is error, `curly: all` enforced
- Prettier for formatting, ESLint for logic rules
- Tests colocated with source files (`Component.test.ts` next to `Component.vue`)

## Config

- App config: `~/Library/Application Support/com.homewizard-unofficial/config.json`
- Telemetry cache: `~/Library/Application Support/com.homewizard-unofficial/telemetry.json`
