#!/bin/bash
# Check if the Docker image exists
set -e
docker build -t sns_categories .


solana program dump namesLPneVptA9Z5rqUDD9tMTWEJwofgaYwp8cawRkX target/deploy/spl_name_service.so


if [[ ${1} == "build-only" ]]; then
    echo "Only building..."
    docker run -it --net=host --mount type=bind,source=$(pwd),target=/workdir sns_categories:latest /bin/bash -c "cargo build-sbf"
elif [[ ${1} == "test" ]]; then
    echo "Running tests..."
    docker run -it --net=host --mount type=bind,source=$(pwd),target=/workdir sns_categories:latest /bin/bash -c "cargo test-sbf --features no-signer"
else
    echo "Running tests + building..."
    docker run -it --net=host --mount type=bind,source=$(pwd),target=/workdir sns_categories:latest /bin/bash -c "cargo test-sbf --features no-signer && cargo build-sbf"
fi