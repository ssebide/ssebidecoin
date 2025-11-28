# Build Stage
FROM rust:1.75-slim-bookworm as builder

WORKDIR /usr/src/ssebidecoin

# Install build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock ./
COPY lib/Cargo.toml lib/
COPY node/Cargo.toml node/
COPY wallet/Cargo.toml wallet/
COPY miner/Cargo.toml miner/

# Create dummy source files to build dependencies
RUN mkdir -p lib/src node/src wallet/src miner/src && \
    echo "fn main() {}" > lib/src/lib.rs && \
    echo "fn main() {}" > node/src/main.rs && \
    echo "fn main() {}" > wallet/src/main.rs && \
    echo "fn main() {}" > miner/src/main.rs

# Build dependencies
RUN cargo build --release

# Remove dummy source files
RUN rm -rf lib/src node/src wallet/src miner/src

# Copy actual source code
COPY . .

# Touch main files to force rebuild of the project crates
RUN touch lib/src/lib.rs node/src/main.rs wallet/src/main.rs miner/src/main.rs

# Build the actual application
RUN cargo build --release

# Runtime Stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

# Copy binaries from builder
COPY --from=builder /usr/src/ssebidecoin/target/release/node /usr/local/bin/
COPY --from=builder /usr/src/ssebidecoin/target/release/wallet /usr/local/bin/
COPY --from=builder /usr/src/ssebidecoin/target/release/miner /usr/local/bin/

# Expose the node port
EXPOSE 9000

# Create a directory for data
RUN mkdir -p /data
VOLUME /data

# Default command runs the node
ENTRYPOINT ["node"]
