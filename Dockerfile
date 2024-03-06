FROM rustlang/rust:nightly-bullseye AS builder
RUN apt-get update && apt-get upgrade -y
RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN cp cargo-binstall /usr/local/cargo/bin
RUN cargo binstall cargo-leptos -y
ARG SQLX_OFFLINE=true
RUN cargo binstall trunk -y
RUN mkdir -p /app
RUN wget https://apt.llvm.org/llvm.sh
RUN apt-get install -y lsb-release gnupg software-properties-common wget
RUN apt-add-repository "deb [trusted=yes] http://apt.llvm.org/bullseye/ llvm-toolchain-bullseye-17 main"
RUN wget -O - https://apt.llvm.org/llvm-snapshot.gpg.key | apt-key add -
RUN apt-get update && apt-get upgrade -y
RUN chmod +x llvm.sh
RUN ./llvm.sh 17
RUN ln -s /usr/bin/clang-17 /usr/bin/clang
WORKDIR /app
COPY . .
RUN rustup target add wasm32-unknown-unknown
ENV LEPTOS_OUTPUT_NAME="dev-asm"
RUN cargo leptos build -r -vv


FROM node:lts-bullseye as style-builder
RUN mkdir -p /app
WORKDIR /app
COPY . .
RUN npx tailwindcss -i style/tailwind.css -o tailwind-build.css


FROM rustlang/rust:nightly-bullseye
COPY --from=style-builder /app/tailwind-build.css /app/site/tailwind.css
COPY --from=builder /app/target/release/dev-asm /app/
COPY --from=builder /app/target/site /app/site
COPY --from=builder /app/Cargo.toml /app/
WORKDIR /app

ENV LEPTOS_SITE_ROOT="site"
ENV LEPTOS_SITE_PKG_DIR="pkg"
ENV LEPTOS_SITE_ADDRESS="0.0.0.0:3000"

EXPOSE 3000
CMD ["/app/dev-asm"]
