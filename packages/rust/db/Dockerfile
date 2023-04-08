FROM rust:latest

RUN cargo install cargo-quickinstall
RUN cargo quickinstall sqlx-cli

COPY ./sqlx.sh /sqlx.sh
COPY ./migrations /migrations

ENTRYPOINT ["/sqlx.sh"]