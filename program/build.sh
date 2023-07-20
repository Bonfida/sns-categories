#!/bin/bash
docker build -t solana_build .
docker run -it --net=host --mount type=bind,source=$(pwd),target=/workdir --mount type=bind,source=$SSH_AUTH_SOCK,target=/ssh-agent --env SSH_AUTH_SOCK=/ssh-agent solana_build:latest cargo build-sbf