FROM rust:latest

WORKDIR /server

RUN mkdir -p /server/src
RUN echo "fn main() {}" > /server/src/main.rs
COPY Cargo.toml .
RUN cargo build --release

COPY . .
RUN cargo build --release
ENTRYPOINT ["/server/target/release/bubble"]