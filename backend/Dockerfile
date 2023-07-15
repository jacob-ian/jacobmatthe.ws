FROM rust:1.71-slim-buster as dev
WORKDIR /src 
COPY backend backend

RUN apt update && apt install -y libssl-dev musl-dev pkg-config
RUN cargo install sqlx-cli

WORKDIR /src/backend
RUN cargo build
CMD ["cargo", "run"]
