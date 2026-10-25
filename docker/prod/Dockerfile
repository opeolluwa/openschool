# Stage 1: Rust build
FROM rust:1.97.0-slim-bookworm AS builder

RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY macros/Cargo.toml macros/Cargo.toml
COPY migration/Cargo.toml migration/Cargo.toml
COPY seeds/ seeds/

# Dependency cache layer: build deps with stub sources so they survive
# source-only changes. The `|| true` swallows the expected link failure.
RUN mkdir -p src macros/src migration/src \
    && echo 'fn main() {}' > src/main.rs \
    && touch src/lib.rs macros/src/lib.rs migration/src/lib.rs \
    && cargo build --release 2>/dev/null || true \
    && rm -rf src macros/src migration/src

COPY macros/ macros/
COPY migration/ migration/
COPY src/ src/
COPY templates/ templates/
COPY seeds/ seeds/
COPY assets/ assets/

RUN cargo build --release --bin backoffice --bin backoffice-cli \
    && strip target/release/backoffice \
    && strip target/release/backoffice-cli


# Stage 2: Runtime
FROM debian:bookworm-slim

RUN groupadd --system nonroot \
    && useradd --system --gid nonroot --no-create-home nonroot

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*
    
WORKDIR /app

COPY --from=builder /app/target/release/backoffice ./backoffice
COPY --from=builder /app/target/release/backoffice-cli ./backoffice-cli
COPY --from=builder /app/templates ./templates
COPY --from=builder /app/assets ./assets

RUN chown -R nonroot:nonroot /app

USER nonroot:nonroot

EXPOSE 8000

STOPSIGNAL SIGTERM

ENTRYPOINT ["./backoffice"]
