FROM rust:1-bookworm

RUN apt-get update \
    && apt-get install --no-install-recommends -y protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock build.rs ./
COPY proto ./proto
COPY src ./src
COPY tests ./tests

RUN cargo build --locked

CMD ["cargo", "test", "--locked"]
