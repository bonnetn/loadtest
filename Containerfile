FROM rust:1-bookworm

WORKDIR /app

COPY Cargo.toml Cargo.lock build.rs ./
COPY proto ./proto
COPY src ./src
COPY tests ./tests

RUN cargo build --locked

CMD ["cargo", "test", "--locked"]
