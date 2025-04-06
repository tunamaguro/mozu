FROM rust:1.85.1-slim-bookworm

# sqlc
COPY --from=sqlc/sqlc:1.28.0 /workspace/sqlc /usr/bin/sqlc

RUN apt-get update -y && \
    apt-get install -y \
    git \
    build-essential \
    curl

# golang-migrate
RUN curl -L https://github.com/golang-migrate/migrate/releases/download/v4.18.2/migrate.linux-amd64.tar.gz | tar xvz -C /usr/bin/

ARG USERNAME=vscode
ARG GROUPNAME=vscode
ARG UID=1000
ARG GID=1000
RUN groupadd -g $GID $GROUPNAME && \
    useradd -m -s /bin/bash -u $UID -g $GID $USERNAME

USER ${USERNAME}

RUN rustup component add rustfmt clippy

RUN cargo install just