FROM rust:1.60

ENV CARGO_TARGET_DIR=/tmp/target \
    DEBIAN_FRONTEND=noninteractive \
    LC_CTYPE=ja_JP.utf8 \
    LANG=ja_JP.utf8

RUN apt update \
    && apt-get upgrade -y \
    && apt-get install -y \
    git \
    curl \
    vim \
    && rustup component add rustfmt

WORKDIR /home/backend

RUN cargo install diesel_cli --no-default-features --features mysql
