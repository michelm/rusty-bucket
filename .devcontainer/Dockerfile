ARG VARIANT=bookworm
FROM mcr.microsoft.com/vscode/devcontainers/rust:${VARIANT}

# Improve lld linker to improve build times either by setting the following
# environment variable:
#   export RUSTFLAGS="-C link-arg=-fuse-ld=lld"
# or by adding the following line to your .cargo/config file:
#   [build]
#   rustflags = ["-C", "link-arg=-fuse-ld=lld"]
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install clang lld lldb \
    && apt-get autoremove -y && apt-get clean -y

RUN rustup install nightly
RUN rustup component add rustfmt --toolchain nightly
RUN rustup component add clippy --toolchain nightly
RUN cargo install cargo-expand cargo-edit
