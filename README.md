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

### Dev container (recommended)

Open the repository in VS Code and run **Dev Containers: Reopen in Container**. The [dev container](.devcontainer/devcontainer.json) builds the [Dockerfile](Dockerfile), forwards the GUI and audio devices (speakers + microphone) from the host, caches cargo / pnpm builds in volumes and installs `tauri-cli` + node dependencies.

```bash
## Then, inside the container
pnpm dev;
```

It works out of the box with WSLg (Windows 11 / WSL2 + Docker Desktop). On a native Linux host, point the container to the host X11 / PulseAudio socket directories (e.g. in `~/.profile`, then restart VS Code):

```bash
export VOH_X11_SOCKET=/tmp/.X11-unix
export VOH_PULSE_DIR="$XDG_RUNTIME_DIR/pulse"
```

Local X11 clients are allowed automatically (`xhost +local:` on container start) and the socket names (display number, PulseAudio socket) are detected inside the container.

Notes:

- `node_modules` lives in a container volume: run git commits (husky hooks) from inside the container.
- First build takes a while (`cargo install tauri-cli` + full Rust build), next ones reuse the cached volumes.

### Docker

The Dockerfile has two targets: `ci` (build / test dependencies, published as `ghcr.io/wasta-geek/voice_of_hell` for the GitHub workflows) and `dev` (adds the GUI / audio runtime packages, used by the dev container).

```bash
## WSLg (from a WSL shell)
docker build --target dev -t voice_of_hell:dev .
docker run -it --shm-size=1g \
  -v "${PWD}:/workspaces/voice-of-hell" -w /workspaces/voice-of-hell \
  -v /mnt/wslg/.X11-unix:/run/host-x11 -v /mnt/wslg:/run/host-pulse \
  voice_of_hell:dev

## Native Linux
## Build with your own uid / gid: files written in the mounted repo stay yours, and PulseAudio accepts the connection
docker build --target dev --build-arg USER_UID="$(id -u)" --build-arg USER_GID="$(id -g)" -t voice_of_hell:dev .
xhost +local:
docker run -it --shm-size=1g \
  -v "${PWD}:/workspaces/voice-of-hell" -w /workspaces/voice-of-hell \
  -v /tmp/.X11-unix:/run/host-x11 -v "${XDG_RUNTIME_DIR}/pulse:/run/host-pulse" \
  voice_of_hell:dev

## Then, inside the container
cargo install tauri-cli --version '^2' --locked && pnpm install && pnpm dev;
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