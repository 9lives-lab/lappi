# Lappi Lab Release Image

Build container

```bash
docker build -t lappi-lab-release -f lappi_lab/docker/release/Dockerfile .
```

Run container

```bash
docker run -i -v /path/to/collection:/usr/src/lappi/workspace/collection lappi-lab-release
```

