# Use a generic base image
FROM ubuntu:latest

# Set environment variables for non-interactive installation
ENV DEBIAN_FRONTEND=noninteractive

# Install required packages and tools
RUN apt-get update && apt-get install -y \
    curl \
    git \
    mc \
    glances \
    build-essential \
    pkg-config \
    libssl-dev \
    libasound2-dev \
    protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*

# Install Node.js v22
RUN curl -fsSL https://deb.nodesource.com/setup_22.x | bash - && \
    apt-get install -y nodejs && \
    rm -rf /var/lib/apt/lists/*

# Enable corepack
RUN corepack enable

# Set the default user to ubuntu
USER ubuntu

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Set the working directory
WORKDIR /workspace/lappi

# Keep the container running
ENTRYPOINT ["tail", "-f", "/dev/null"]

