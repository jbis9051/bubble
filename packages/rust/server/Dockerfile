FROM rust:latest

WORKDIR /rust

RUN mkdir -p /rust/common/src
RUN mkdir -p /rust/server/src
RUN mkdir -p /rust/frontend/src
RUN mkdir -p /rust/bridge_macro/src
RUN mkdir -p /rust/bridge_gen/src

RUN echo "fn main() {}" > /rust/server/src/main.rs
RUN echo "" > /rust/common/src/lib.rs
RUN echo "" > /rust/frontend/src/lib.rs
RUN echo "" > /rust/bridge_macro/src/lib.rs
RUN echo "fn main() {}" > /rust/bridge_gen/src/main.rs

COPY Cargo.toml .
COPY common/Cargo.toml ./common/Cargo.toml
COPY server/Cargo.toml ./server/Cargo.toml
COPY frontend/Cargo.toml ./frontend/Cargo.toml
COPY bridge_macro/Cargo.toml ./bridge_macro/Cargo.toml
COPY bridge_gen/Cargo.toml ./bridge_gen/Cargo.toml

RUN cargo build --package server --release

COPY common ./common
COPY server ./server
COPY frontend ./frontend
COPY bridge_macro ./bridge_macro

RUN touch /rust/server/src/main.rs
RUN touch /rust/common/src/lib.rs
RUN touch /rust/frontend/src/lib.rs
RUN touch /rust/bridge_macro/src/lib.rs

RUN cargo build --package server --release

ENTRYPOINT ["/rust/target/release/server"]