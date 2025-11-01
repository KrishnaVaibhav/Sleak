# Sleak Stealth Client — Windows

This directory houses the Windows implementation of the Sleak stealth overlay, built with [Tauri](https://tauri.app/). The goal is to keep the runtime lightweight while giving us direct access to the Win32 APIs we need for:

- An always-on-top, transparent overlay that can be excluded from common screen-capture pipelines.
- WASAPI microphone and loopback capture for simultaneous local speech-to-text (STT).
- Desktop Duplication API access for low-latency screen snapshots without leaking the overlay.

> **Why Tauri?**  
> We get a modern web-based UI surface while the control plane remains Rust, making it much easier to toggle the native flags (`SetWindowDisplayAffinity`, layered window hints, taskbar suppression, etc.) without juggling Node.js native bindings.

## Repository Layout

- `public/` — Static UI assets served by Tauri (no heavy framework by default). Replace with Vite/React/etc. if needed.
- `src-tauri/` — Rust sources, Cargo manifest, and Tauri configuration.
- `src-tauri/icons/` — Shipping icon placeholders (`icon.ico`) required by the Windows bundler; swap with the branded assets later.
- `src-tauri/src/windows_overlay.rs` — All Win32-specific window manipulation lives here; this keeps the Tauri entry point clean.
- `src-tauri/src/main.rs` — Tauri bootstrap plus commands (hotkeys, overlay toggles, capture stubs, …).

## Getting Started

1. **Install prerequisites**
   - [Rust toolchain](https://www.rust-lang.org/tools/install) (stable 1.74+ recommended).
   - [Node.js 18+](https://nodejs.org/en/) — only for the dev workflow and bundling static assets.
   - Visual Studio Build Tools with the “Desktop development with C++” workload (required for compiling Win32 crates).

2. **Install dependencies**
   ```bash
   npm install
   npx tauri info
   ```
   > `npx tauri info` verifies your Rust toolchain and WebView2 install. If it fails because `rustc` is missing, install Rust via [rustup.rs](https://rustup.rs/) and retry.

3. **Run the dev build**
   ```bash
   npm run tauri:dev
   ```

   This launches the overlay. In dev builds it starts visible for easier debugging; use the global shortcut (default: `Ctrl+Shift+Space`) to dim/restore it while staying on your screen.

4. **Build the distributable**
   ```bash
   npm run tauri:build
   ```

## Feature Hooks

| Area | Stub / Entry Point | Notes |
| ---- | ------------------ | ----- |
| Overlay stealth | `windows_overlay::apply_overlay_hints` | Applies layered window flags, capture affinity, and removes taskbar/alt-tab presence. |
| Overlay toggle | `toggle_overlay` command | Wire this to a shortcut or backend event to show inspectable UI when needed. |
| Screen capture | `capture::start_desktop_duplication` (to be added) | Add a Rust module wrapping the Desktop Duplication API. |
| Audio capture | `audio::start_loopback` / `start_microphone` (to be added) | Use WASAPI loopback + capture; feed audio buffers into local STT. |
| Local STT | Future `stt` module | Integrate `whisper.cpp` or similar via FFI. |

## Next Steps

- Flesh out modules for screen/audio capture and stitch them into the command layer.
- Replace the static HTML shell with our actual React/Svelte/etc. UI once designs settle.
- Add tests around the window-affinity helpers using `tauri::test::mock_builder`.
- Harden error handling (e.g., fallback when `SetWindowDisplayAffinity` fails because Teams is in protected capture mode).

Feel free to update this README as we implement each module—think of it as the living spec for the Windows client.
