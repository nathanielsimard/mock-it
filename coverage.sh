#!/bin/sh

mkdir -p target/cov
cargo install cargo-kcov
docker run -it --rm --security-opt seccomp=unconfined \
	-v "$HOME"/.cargo:/root/.cargo \
	-v "$HOME"/.rustup:/root/.rustup \
	-v "$PWD":/root/app kcov/kcov \
	sh -c "export PATH=$PATH:/root/.cargo/bin; \
		apt update; \
		apt install gcc -y; \
		cargo kcov"
		
