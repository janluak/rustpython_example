#!/bin/bash

rustpython -m pip download --only-binary :all: --dest . --no-cache -r requirements.txt