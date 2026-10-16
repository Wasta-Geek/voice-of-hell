## CI stage: build / test dependencies only (published image, used by the GitHub workflows)
FROM rust:1.91-trixie AS ci

WORKDIR /home

ENV SERVICE_NAME="voice-of-hell"

## Install tauri + audio dependencies
RUN apt update && \
    apt-get install -y libwebkit2gtk-4.1-dev libasound2-dev libpango1.0-dev libgtk-3-dev libglib2.0-dev && \
    rm -rf /var/lib/apt/lists/*

## Install latest Rust toolchain
RUN rustup toolchain install stable --component rustfmt,clippy

# Download and install n + node
ENV NODE_VERSION=24
RUN curl -fsSL https://raw.githubusercontent.com/tj/n/master/bin/n | bash -s install lts && \
    npm install -g n && \
    n install $NODE_VERSION && \
    npm install -g pnpm@12.11.2

## Add && setup user
## Note: https://github.com/actions/checkout/issues/1014#issuecomment-2899102017
## Override with `--build-arg USER_UID=$(id -u) --build-arg USER_GID=$(id -g)` to match a Linux host user (plain docker run)
ARG USER_UID=1001
ARG USER_GID=1001
RUN groupadd -g ${USER_GID} ${SERVICE_NAME} && useradd -u ${USER_UID} -g ${SERVICE_NAME} -ms /bin/bash ${SERVICE_NAME}

USER ${SERVICE_NAME}
ENV HOME=/home/${SERVICE_NAME}
WORKDIR $HOME

ENTRYPOINT [ "/bin/bash" ]

## Dev stage: CI image + runtime packages to display the Tauri window and use the host audio devices
## Host sockets are mounted by .devcontainer/devcontainer.json or the docker run flags in README:
##   X11 socket directory -> /run/host-x11, PulseAudio socket directory -> /run/host-pulse
FROM ci AS dev

USER root
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
      libgl1-mesa-dri \
      fonts-dejavu-core \
      libasound2-plugins \
      alsa-utils \
      pulseaudio-utils \
      libayatana-appindicator3-1 && \
    rm -rf /var/lib/apt/lists/*

## cpal talks to ALSA: route the default ALSA devices to the host PulseAudio server
RUN printf 'pcm.!default { type pulse }\nctl.!default { type pulse }\n' > /etc/asound.conf

## Host socket names differ between WSLg and native Linux: linked at startup to these fixed paths
COPY --chmod=755 .devcontainer/link-host-sockets.sh /usr/local/bin/link-host-sockets
RUN mkdir -p /tmp/.X11-unix && chmod 1777 /tmp/.X11-unix
ENV DISPLAY=:0
ENV PULSE_SERVER=unix:/tmp/pulse-native
## Force X11: the WSLg wayland socket lives in a runtime dir owned by the host uid
ENV GDK_BACKEND=x11
## DMA-BUF renderer leaves a blank window in containers / WSLg
ENV WEBKIT_DISABLE_DMABUF_RENDERER=1

USER ${SERVICE_NAME}
ENTRYPOINT [ "/bin/sh", "-c", "link-host-sockets && exec /bin/bash \"$@\"", "--" ]
