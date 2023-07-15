FROM rust:1.71-slim-buster as dev
WORKDIR /src 
COPY website website

RUN apt update && apt install -y libssl-dev musl-dev pkg-config

WORKDIR /src/website
RUN cargo build
CMD ["cargo", "run"]
