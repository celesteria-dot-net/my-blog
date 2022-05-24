FROM lukemathwalker/cargo-chef:latest-rust-1.61.0 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS build-env
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc
COPY --from=build-env /app/target/release/my-blog /
COPY ./static /static

USER nonroot
ENV PORT 8080
EXPOSE $PORT

ENTRYPOINT ["/my-blog"]
