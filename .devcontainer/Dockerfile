# x86_64
FROM rust:slim

# Non-Rust tooling
ENV TZ=America/New_York
RUN apt-get update -y
RUN DEBIAN_FRONTEND="noninteractive" apt-get update && apt-get install -y \
    build-essential \
    libssl-dev \
    pkg-config \
    rr \
    tree \
    xxd \
    git \
    vim \
    curl \
    libpcap-dev

# Rust tooling
RUN rustup toolchain install nightly
RUN rustup component add llvm-tools-preview
RUN rustup component add rustfmt
RUN cargo install mdbook
RUN cargo install cargo-fuzz
RUN cargo install cargo-binutils
RUN cargo install cargo-modules
RUN cargo install cargo-audit
