#!/bin/zsh

if [[ -z "$1" ]] ; then
  echo "missing directory to run"
  exit 1;
  else directory=$1
fi

echo "start compiling"
docker run --rm -v "$PWD/$directory":/home rustpython_compiler
echo "finished compiling"

rm run_directory/*
cp "$PWD/$directory/target/debug/$directory" run_directory/run
cp $PWD/$directory/src/*.py run_directory
