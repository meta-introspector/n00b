# Stage 1: Build the Rust application
FROM rust:latest AS builder

# Set working directory
WORKDIR /app

# Copy Cargo.toml and Cargo.lock first to leverage Docker cache
COPY mytool/Cargo.toml mytool/Cargo.lock ./mytool/
COPY Cargo.toml Cargo.lock .
COPY .cargo .cargo/

# This step is crucial for caching dependencies
RUN mkdir -p mytool/src && touch mytool/src/main.rs && \
    cargo build --manifest-path mytool/Cargo.toml --release && \
    rm mytool/src/main.rs

# Copy the rest of the source code
COPY mytool/src ./mytool/src
COPY src src/

# Build the release binary
RUN cargo build --manifest-path mytool/Cargo.toml --release

# Stage 2: Create a minimal runtime image
FROM debian:stable-slim

# Install necessary runtime dependencies (e.g., for RocksDB)
# This might need adjustment based on specific RocksDB dependencies
RUN apt-get update && apt-get install -y libssl-dev libz-dev && \
    rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/mytool ./mytool

# Expose the port Actix-web server listens on (default 8080)
EXPOSE 8080

# Set environment variable for GitHub Token (expected during runtime)
ENV GITHUB_TOKEN=

# Command to run the application
CMD ["./mytool"]
