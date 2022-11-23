# Examples for making rustpython run actual python code
These examples are for helping understand the issues and (once solved) hopefully helpful for future users of `rustpython`

## Repo overview
The repository contains various examples with their own issues (see `Readme` in each directory).

Additionally, the repo ships with two dockerfiles and three scripts.
The dockerfiles are for ensuring encapsulation and making issues reproducible.

Each build process creates newly a `run` binary in a directory called `run_directory` (created with first step).
This directory is mounted to the execution container (step 3) where the `./run` can be executed after each build.
Unfortunately this is necessary since any python printing is not passed through the docker stdout pipe.

## Steps for running

1. run the `setup.sh` for having the required docker images for the second step and have shared runtime_directory created
2. run the `build.sh` with directory name as first argument for building the rust project in the directory. The compiled program will be copied into the `run_directory`
3. run the `execution.sh` (best in own terminal as it is reusable after each build process). Run `./run` for showing the output.

