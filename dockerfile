FROM rust:latest

RUN apt update && \
    apt install vim -y
RUN rustup component add rls rust-analysis rust-src rustfmt clippy && \
    rustup install nightly
WORKDIR /workspace
ENV USER=root
ENV RUST_BACKTRACE=1