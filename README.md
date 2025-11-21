# voice-of-hell

Soundboard project in Rust with a small GUI using Tauri crate and React as frontend framework (Mantine framework).

## Formatting

This project use rustfmt as formatting software, default style applied.

## Linting

Clippy is used for linting purpose, default settings applied.

## Install on WSL

```bash
## Install Rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
## Install project
$TODO
```

## TODO (for personal progress)

- ~~Handle keyboard input globally without focus~~
- System tray app
- GUI:
  - ~~device selection~~
  - ~~Sound set up (key / sound file managed)~~
- Audio processing:
  - Play file (wav)
  - Add effects:
    - Echo
    - Record && replay input
    - Record && replay output (what's emitted in output device -> replay external voices)
    - High pitch voice
    - Low pitch voice
    - Simulate "lag" effect (drop some frames ?)
  - Clear played files / effects
- [precommit]
  - Lint (+ force every classes / function commented)
  - Format
- [CI/CD]
  - Binary build
  - Testing
- [R&D] Allow to select a keyboard that will entirely be managed by the app / ignored by OS
- [R&D] Create a virtual audio device ? (would replace VCable)

## Errors

- When quitting dev: "ELIFECYCLE  Command failed with exit code 4294967295."
  - Can be safely ignored: https://github.com/tauri-apps/tauri/issues/5243#issuecomment-1252001710