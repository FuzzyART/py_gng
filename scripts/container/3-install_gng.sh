#!/bin/bash

docker exec -it pytorch_project_cont fish -c "pip install /workDir/target/wheels/* --force-reinstall"

