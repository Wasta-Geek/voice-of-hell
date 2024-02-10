# voice-of-hell
Soundboard project in the same line of my previous project 'voice-changer' in C++, but in Rust.

## Formatting

This project use rustfmt as formatting software, default style applied.

## Linting

Clippy is used for linting purpose, default settings applied.

## Install on WSL

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

## TODO (for personal progress)

- Handle keyboard input globally without focus
- System tray app
- GUI:
  - device selection
  - Sound set up (key / sound file managed / volume for each sound)
- Audio processing:
  - Play file (wav)
  - Add effects:
    - Echo
    - Record && replay (both input and output)
  - Stop played files / effects
- [Windows] app installer
- [R&D] Create a virtual audio device ? (would replace VCable)