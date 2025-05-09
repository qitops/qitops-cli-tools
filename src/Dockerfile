FROM rust:1.70-slim as builder

WORKDIR /app

# Copy the Cargo files for dependency caching
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir -p src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy the actual source code
COPY . .

# Build the application
RUN cargo build --release

# Create a smaller runtime image
FROM debian:bullseye-slim

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/qitops /app/qitops
COPY --from=builder /app/tests/configs /app/configs

# Set the entrypoint
ENTRYPOINT ["/app/qitops"]

# Default command
CMD ["--help"]
