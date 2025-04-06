# Lappi

![lappi6](https://github.com/user-attachments/assets/feabf859-c47a-462e-ba0d-fb00677d9104)

Lappi is a research project dedicated to help organize and explore a music collection.

## Structure

At the moment the project consists of two modules:
- lappi_lab: Executable application.
- lappi_core: Most of the code is located here. Expected to be included in builds for other platforms (mobile, etc.) in the future.

## Quick start

The easiest way to build Lappi Lab is to use development Docker container.

### Prerequisites

- [Docker](https://www.docker.com/get-started/)
- [Docker Compose](https://docs.docker.com/compose/)

### Prepare environment

Clone repository

```bash
git clone --recursive https://github.com/9lives-lab/lappi.git
cd lappi
```

Build container

```bash
cd lappi_lab/docker/dev
docker build -t lappi-lab-dev .
```

Run container

```bash
docker compose up
```

Attach to container

```bash
docker exec -it lappi-lab-dev /bin/bash
```

### Run backend (inside container)

Build Rust project

```bash
cargo build
```

To launch the application, workspace should be set.
A workspace is a working directory where the application stores launch configuration files, settings, music collection, etc. 
This way, it is possible to have multiple independent workspaces for different work scenarios.

Base directory for workspaces is:
```
lappi_lab/debug_workspace/
```

Subdirectory inside base directory is configured by 'LAPPI_WORKSPACE' environment variable.

```bash
cargo run -- --env LAPPI_WORKSPACE=default
```

### Run frontend (inside container)

Change directory

```bash
cd lappi_lab/lappi_lab_ui
```

Install dependencies

```bash
yarn install
```

Run frontend

```bash
yarn quasar dev
```

Frontend is available at http://localhost:9000

## License
[MIT License](LICENSE-MIT), [Apache License](LICENSE-APACHE)
<p><em>Screenshots included in this repository may contain copyrighted material such as album covers, photos, and lyrics, which are the property of their respective owners. 
These are provided solely for the purpose of demonstrating the application's functionality. No copyright infringement is intended.</em></p>
