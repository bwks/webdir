# Build stage
FROM rust:latest AS builder

WORKDIR /usr/src/app

# Copy manifests
COPY . .

# Build release binary
RUN cargo build --release

# Runtime stage  
FROM debian:bookworm-slim

# Install Dependencies
RUN apt-get update && apt-get install -y \
  libc6 \
  && rm -rf /var/lib/apt/lists/*

# Copy binary from build stage
COPY --from=builder /usr/src/app/target/release/webdir /usr/local/bin/webdir

# Expose port
EXPOSE 13337

# Run the web service
CMD ["webdir", "-4", "0.0.0.0", "-l", "debug", "/opt/ztp"]