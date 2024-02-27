FROM rust:latest AS builder
RUN cargo install cargo-leptos
RUN rustup target add wasm32-unknown-unknown 
RUN mkdir -p /app
WORKDIR /app
COPY . .
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
