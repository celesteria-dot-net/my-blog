# syntax=docker/dockerfile:1.4

FROM lukemathwalker/cargo-chef:latest-rust-1.61.0 AS chef
WORKDIR /app

FROM chef AS planner
COPY --link . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS build-env
COPY --link --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY --link . .
RUN cargo build --release

FROM gcr.io/distroless/cc
COPY --link --from=build-env /app/target/release/my-blog /
COPY --link ./static /static

USER nonroot
ENV PORT 8080
EXPOSE $PORT

ENTRYPOINT ["/my-blog"]
