# voice-of-hell

Soundboard project in Rust with a small GUI using Tauri crate and React as frontend framework (Mantine framework).

## Quickstart

### Install (Ubuntu)

```bash
$NODE_VERSION = 24;

## Install Rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh;

## Install toolchain
rustup toolchain install stable --component rustfmt,clippy;

## Install dependencies
apt update && apt-get install -y libwebkit2gtk-4.1-dev libasound2-dev libpango1.0-dev libgtk-3-dev libglib2.0-dev;

## Install pnpm
wget -qO- https://get.pnpm.io/install.sh | ENV="$HOME/.bashrc" SHELL="$(which bash)" bash -;

## Download and install nvm + node
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.3/install.sh | bash && source $HOME/nvm.sh && nvm install $NODE_VERSION && nvm alias default $NODE_VERSION;

## Use Package.json commands
## Ex: launch dev mode (live reload)
pnpm dev;
```

### Docker (WIP)

```bash
## TODO: be able to expose audio devices to docker
docker run -it -v "${PWD}:/home/voice-of-hell" ghcr.io/wasta-geek/voice_of_hell:latest
```

## Formatting

This project use rustfmt as formatting software, default style applied.

## Linting

Clippy is used for linting purpose, default settings applied.

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
  - ~~Lint (+ force every classes / function commented)~~
  - ~~Format~~
- [CI/CD]
  - ~~Binary build~~
  - ~~Testing~~
- [R&D] Allow to select a keyboard that will entirely be managed by the app / ignored by OS
- [R&D] Create a virtual audio device ? (would replace VCable)

## Errors

- When quitting dev: "ELIFECYCLE  Command failed with exit code 4294967295."
  - Can be safely ignored: https://github.com/tauri-apps/tauri/issues/5243#issuecomment-1252001710