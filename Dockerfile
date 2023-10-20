FROM lukemathwalker/cargo-chef:0.1.54-rust-1.68-buster AS chef
RUN apt update && apt install -y curl unzip cmake capnproto libsasl2-dev
ENV PROTOC_ZIP=protoc-3.20.1-linux-x86_64.zip
RUN curl -LO "https://github.com/protocolbuffers/protobuf/releases/download/v3.20.1/protoc-3.20.1-linux-x86_64.zip" \
    && unzip -o protoc-3.20.1-linux-x86_64.zip -d /usr/local bin/protoc \
    && unzip -o protoc-3.20.1-linux-x86_64.zip -d /usr/local 'include/*' \
    && rm -f protoc-3.20.1-linux-x86_64.zip
WORKDIR app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
COPY ./deps ./deps
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release

# We do not need the Rust toolchain to run the binary!
FROM debian:bullseye-slim AS runtime
RUN apt update && apt install -y curl unzip cmake capnproto libsasl2-dev
ENV PROTOC_ZIP=protoc-3.20.1-linux-x86_64.zip
RUN curl -LO "https://github.com/protocolbuffers/protobuf/releases/download/v3.20.1/protoc-3.20.1-linux-x86_64.zip" \
    && unzip -o protoc-3.20.1-linux-x86_64.zip -d /usr/local bin/protoc \
    && unzip -o protoc-3.20.1-linux-x86_64.zip -d /usr/local 'include/*' \
    && rm -f protoc-3.20.1-linux-x86_64.zip
WORKDIR app

COPY . .

COPY --from=builder /app/target/release/blumer-ms-profile /usr/local/bin/app
RUN ls -l /usr/local/bin
ENTRYPOINT ["/usr/local/bin/app"]