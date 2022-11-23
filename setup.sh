#!/bin/zsh

docker build -t rustpython_compiler -f Dockerfile_build .
docker build -t rustpython_runner -f Dockerfile_run .
mkdir run_directory