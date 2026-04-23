#! /bin/bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
docker buildx build --platform linux/amd64 -t pytorch_project_cont --load $SCRIPT_DIR
