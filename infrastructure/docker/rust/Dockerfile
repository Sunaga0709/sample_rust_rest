FROM rust:1.67.1-alpine

WORKDIR /app

RUN apk update && apk upgrade && apk add git alpine-sdk make libffi-dev openssl-dev pkgconfig
RUN rustup component add rls rust-analysis rustfmt clippy
RUN cargo install --version="0.6.2" sqlx-cli

CMD ["cargo", "add", "tokio", "clap", "env_logger", "tokio_openssl"]