FROM rust:1-buster AS build
LABEL image=hpcsc/pair-matching-bot

WORKDIR /build

COPY . .

RUN rustup override set nightly && cargo build --release

FROM debian:buster-slim

WORKDIR /app

COPY --from=build /build/target/release/pair-matching-bot .

ENTRYPOINT ["/app/pair-matching-bot"]
