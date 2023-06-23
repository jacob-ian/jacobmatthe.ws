FROM rust:alpine3.18 as dev
WORKDIR /src 
COPY backend backend

RUN apk add musl-dev openssl-dev
RUN cargo install cargo-watch
RUN cargo install sqlx-cli

WORKDIR /src/backend
CMD ["cargo", "watch", "-x", "run"]
