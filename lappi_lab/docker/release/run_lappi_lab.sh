#!/bin/bash

export LAPPI_WORKSPACE_DIR=/usr/src/lappi/workspace

npx http-server ./lappi_lab_ui/spa -s -p 8080 & ./lappi_lab

