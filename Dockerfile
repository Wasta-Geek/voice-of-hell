FROM rust:1.91-trixie

WORKDIR /home

ENV SERVICE_NAME="voice-of-hell"

## Install tauri + audio dependencies
RUN apt update && \
    apt-get install -y libwebkit2gtk-4.1-dev libasound2-dev libpango1.0-dev libgtk-3-dev libglib2.0-dev && \
    rm -rf /var/lib/apt/lists/*

## Install latest Rust toolchain
RUN rustup toolchain install stable --component rustfmt,clippy

## Install pnpm
ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"
RUN wget -qO- https://get.pnpm.io/install.sh | ENV="$HOME/.bashrc" SHELL="$(which bash)" bash -

# Download and install n + node
ENV NODE_VERSION=24
RUN curl -fsSL https://raw.githubusercontent.com/tj/n/master/bin/n | bash -s install lts && \
    pnpm install -g n && \
    n install $NODE_VERSION

## Add && setup user
## Note: https://github.com/actions/checkout/issues/1014#issuecomment-2899102017
RUN groupadd -g 1001 ${SERVICE_NAME} && useradd -u 1001 -g ${SERVICE_NAME} -ms /bin/bash ${SERVICE_NAME}

USER ${SERVICE_NAME}
ENV HOME=/home/${SERVICE_NAME}
WORKDIR $HOME

ENTRYPOINT [ "/bin/bash" ]