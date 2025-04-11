# Stage 1: Build the Rust Backend
FROM rust:1.85 AS backend-builder

RUN apt-get update && apt-get install -y \
    libssl-dev \
    libasound2-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app/lappi

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./amina ./amina
COPY ./lappi_core ./lappi_core
COPY ./lappi_lab/lappi_lab ./lappi_lab/lappi_lab

RUN cargo build --release


# Stage 2: Build the Vue Frontend
FROM node:22 AS frontend-builder

WORKDIR /usr/src/app/lappi

COPY ./lappi_lab/lappi_lab_ui ./lappi_lab_ui

WORKDIR /usr/src/app/lappi/lappi_lab_ui

RUN yarn install
RUN yarn quasar build


# Stage 3: Combine the Builds into a Minimal Runtime Container
FROM ubuntu:latest

RUN apt-get update && apt-get install -y \
    libssl-dev \
    libasound2-dev \
    && rm -rf /var/lib/apt/lists/*

# Set the default user to ubuntu
USER ubuntu

WORKDIR /usr/src/lappi

# Frontend
COPY --chown=ubuntu:ubuntu --from=frontend-builder /usr/src/app/lappi/lappi_lab_ui/dist/spa ./lappi_lab_ui

# Backend binary
COPY --chown=ubuntu:ubuntu --from=backend-builder /usr/src/app/lappi/target/release/lappi_lab ./lappi_lab
COPY --chown=ubuntu:ubuntu ./lappi_lab/docker/release/app_config.yaml ./workspace/app_config.yaml
COPY --chown=ubuntu:ubuntu ./lappi_lab/docker/release/run_lappi_lab.sh ./run_lappi_lab.sh

# Default port
EXPOSE 8080

# Start the backend and serve the frontend
ENTRYPOINT ["/usr/src/lappi/run_lappi_lab.sh"]

