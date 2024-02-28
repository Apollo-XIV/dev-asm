FROM rust:latest AS builder
RUN apt-get update && apt-get upgrade -y
RUN cargo install cargo-leptos
ARG SQLX_OFFLINE=true
RUN cargo install trunk
RUN mkdir -p /app
RUN wget https://apt.llvm.org/llvm.sh
RUN apt-get install -y lsb-release gnupg software-properties-common wget
RUN apt-add-repository "deb http://apt.llvm.org/bionic/ llvm-toolchain-bionic-11 main"
RUN apt-get update
RUN chmod +x llvm.sh
RUN ./llvm.sh 17
RUN ln -s /usr/bin/clang-17 /usr/bin/clang
WORKDIR /app
COPY . .
RUN rustup target add wasm32-unknown-unknown
RUN cargo leptos build -r -vv

FROM alpine:latest
WORKDIR /app

COPY --from=builder /app/target /app
COPY --from=builder /app/target/site /app/site

ENV LEPTOS_OUTPUT_NAME="dev-asm"
ENV LEPTOS_SITE_ROOT="site"
ENV LEPTOS_SITE_PKG_DIR="pkg"
ENV LEPTOS_SITE_ADDRESS="0.0.0.0:3000"

EXPOSE 3000
CMD [""]
