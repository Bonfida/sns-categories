FROM rust:1.65.0

ENV HOME="/root"
ENV PATH="${HOME}/.local/share/solana/install/active_release/bin:${PATH}"

# Install Solana tools.
RUN sh -c "$(curl -sSfL https://release.solana.com/v1.14.20/install)"

RUN cargo new temp && cd temp && cargo build-bpf && cargo build-sbf

WORKDIR /workdir