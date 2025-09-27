# Multi-stage build for cross-architecture support (AMD64 + ARM64)
FROM rust:1.90-alpine AS builder

# Install build dependencies
RUN apk add --no-cache \
    musl-dev \
    nodejs \
    npm \
    make \
    pkgconfig \
    openssl-dev \
    perl

WORKDIR /app

# Copy source code
COPY . .

# Build CSS and Rust binary
RUN npm install && \
    npm run build-css && \
    cargo build --release

# Runtime stage - minimal Alpine image  
FROM alpine:latest

# Install runtime dependencies and upgrade
RUN apk add --no-cache ca-certificates && \
    apk upgrade --no-cache

# Add data dir (mount volume with your recipes and config dir here)
RUN mkdir /data --mode=755

# Add non-root user
RUN addgroup -g 1000 cookcli_user && \
    adduser -u 1000 -G cookcli_user -s /bin/sh -D cookcli_user

# Copy binary from builder stage
COPY --from=builder /app/target/release/cook /bin/cook
RUN chmod 755 /bin/cook

# Set ownership and switch to non-root user
RUN chown -R cookcli_user:cookcli_user /data
USER cookcli_user

WORKDIR /data
EXPOSE 9080

# Run server (using JSON array format for better signal handling)
ENTRYPOINT ["cook", "server", ".", "--host", "--port", "9080"]
