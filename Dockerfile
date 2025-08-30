# Copyright 2025 Mehmet T. AKALIN
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

# php2ir Dockerfile
# Multi-stage build for efficient containerization

# Build stage
FROM rust:1.78-slim as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    cmake \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy Cargo files
COPY Cargo.toml Cargo.lock ./

# Create dummy source to build dependencies
RUN mkdir -p src && \
    echo "fn main() {}" > src/main.rs && \
    echo "pub fn dummy() {}" > src/lib.rs && \
    cargo build --release && \
    rm -rf src

# Copy source code
COPY src/ src/

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    llvm-17 \
    lld-17 \
    clang-17 \
    libc6 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create symlinks for LLVM tools
RUN ln -sf /usr/bin/llvm-config-17 /usr/bin/llvm-config && \
    ln -sf /usr/bin/llc-17 /usr/bin/llc && \
    ln -sf /usr/bin/ld.lld-17 /usr/bin/ld.lld && \
    ln -sf /usr/bin/clang-17 /usr/bin/clang

# Create non-root user
RUN useradd -m -u 1000 php2ir && \
    mkdir -p /home/php2ir/work && \
    chown -R php2ir:php2ir /home/php2ir

# Set working directory
WORKDIR /home/php2ir/work

# Copy binary from builder stage
COPY --from=builder /app/target/release/php2ir /usr/local/bin/

# Set environment variables
ENV PATH="/usr/lib/llvm-17/bin:${PATH}"
ENV LLVM_PREFIX="/usr/lib/llvm-17"

# Switch to non-root user
USER php2ir

# Set default command
ENTRYPOINT ["php2ir"]

# Default command (can be overridden)
CMD ["--help"]
