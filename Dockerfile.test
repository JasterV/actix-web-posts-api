FROM rust:latest

RUN apt-get -y update && \
     apt-get -y upgrade  && \
     apt -y install ca-certificates libssl-dev libpq-dev libpq-dev

RUN cargo install cargo-watch
RUN cargo install diesel_cli --no-default-features --features postgres

RUN mkdir -p /app

WORKDIR /app

