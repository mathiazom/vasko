FROM rust:1.100 AS builder

WORKDIR /app

COPY ./Cargo.toml ./Cargo.lock ./

# Build dependencies alone so this layer survives source changes
RUN mkdir src && echo 'fn main(){}' > src/main.rs && cargo build --release && rm -rf src

COPY ./src ./src

# touch: the dummy main.rs build is newer than the copied source, so cargo would skip it
RUN touch src/main.rs && cargo build --release

FROM debian:stable-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/vasko /usr/local/bin/vasko

CMD ["vasko"]
