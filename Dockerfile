FROM rust:latest

COPY . /app
WORKDIR /app

RUN cargo build --release