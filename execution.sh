#!/bin/zsh

docker run --rm -it -v "$PWD/$directory/run_directory":/home rustpython_runner
