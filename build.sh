#!/bin/zsh

if [[ -z "$1" ]] ; then
  echo "missing directory to run"
  exit 1;
  else directory=$1
fi

echo "start compiling"
docker run --rm -v "$PWD/$directory":/home rustpython_compiler
echo "finished compiling"

project_root=$PWD
rm -rf run_directory/*
cp "$project_root/$directory/target/debug/$directory" run_directory/run
cd "$project_root/$directory/src" && rsync -R **/*.py $project_root/run_directory || true && cp *.py $project_root/run_directory/ || true && cd $project_root
