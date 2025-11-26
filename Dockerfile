FROM rust:1.91-trixie

WORKDIR /home

ENV SERVICE_NAME="voice-of-hell"

## Install tauri + audio dependencies
RUN apt update && \
    apt-get install -y libwebkit2gtk-4.1-dev libasound2-dev libpango1.0-dev libgtk-3-dev libglib2.0-dev && \
    rm -rf /var/lib/apt/lists/*

## Install latest Rust toolchain
RUN rustup toolchain install stable --component rustfmt,clippy

## Add && Change user
RUN useradd -ms /bin/bash ${SERVICE_NAME}
USER ${SERVICE_NAME}
ENV HOME=/home/${SERVICE_NAME}
WORKDIR $HOME

## Node / Nvm variables
ENV NODE_VERSION=24
ENV NVM_DIR=$HOME/.nvm

## Install pnpm
RUN wget -qO- https://get.pnpm.io/install.sh | ENV="$HOME/.bashrc" SHELL="$(which bash)" bash -

# Download and install nvm + node
RUN bash -c " \
    curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.3/install.sh | bash && \
    source $NVM_DIR/nvm.sh && \
    nvm install $NODE_VERSION && \
    nvm alias default $NODE_VERSION"

ENTRYPOINT [ "/bin/bash" ]