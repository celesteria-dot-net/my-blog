FROM ekidd/rust-musl-builder:1.57.0 AS builder

USER root
WORKDIR /app

COPY Cargo.lock .
COPY Cargo.toml .
RUN mkdir src
RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs
RUN cargo build --release
RUN rm -rf /src

COPY ./src ./src
RUN cargo build --release

FROM gcr.io/distroless/cc

COPY --from=builder --chown=nonroot:nonroot /app/target/release/my-blog /
COPY ./static /static
USER nonroot

EXPOSE 8080

ENTRYPOINT ["/my-blog"]
