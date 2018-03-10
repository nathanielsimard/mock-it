#!/bin/bash

mkdir -p target/cov
docker run -it --rm --security-opt seccomp=unconfined --volume "$(pwd):/volume" elmtai/docker-rust-kcov