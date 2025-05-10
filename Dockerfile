FROM rust:1.70-slim as builder

WORKDIR /usr/src/qitops
COPY . .

RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    cargo build --release

FROM debian:bullseye-slim

RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/qitops/target/release/qitops /usr/local/bin/qitops

# Create a directory for test configurations
RUN mkdir -p /etc/qitops/configs

# Set the working directory
WORKDIR /workspace

ENTRYPOINT ["qitops"]
CMD ["--help"]
