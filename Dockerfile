FROM rust:latest AS build
WORKDIR /app
COPY . .
RUN cargo build

FROM alpine:latest
COPY --from=build /app/target /app
WORKDIR /app

ENV LEPTOS_SITE_ADDRESS "0.0.0.0:3000"
EXPOSE 3000
CMD ["./server/release/leptos_start"]
