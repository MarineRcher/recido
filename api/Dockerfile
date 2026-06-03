FROM rust:latest

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*

RUN cargo install cargo-watch

WORKDIR /app

# Layer dépendances — ne se rebuilde que si Cargo.toml ou Cargo.lock change
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build
RUN rm -rf src

# Layer code source — se rebuilde à chaque changement dans src/
COPY src ./src

EXPOSE 8000

CMD ["cargo", "watch", "-x", "run"]

