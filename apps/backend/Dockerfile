# syntax=docker/dockerfile:1
# FROM public.ecr.aws/docker/library/rust:1.84.1 AS base
FROM rust:1.86-slim AS base
RUN apt-get update && apt-get upgrade -y && apt-get clean
WORKDIR /workspace/morrow/apps/backend

FROM base AS dev
EXPOSE 3000
RUN <<EOF
rustup component add rustfmt clippy
# cargo install bacon sea-orm-cli
cargo install bacon
EOF

FROM base AS builder
WORKDIR /workspace/morrow/apps/backend
COPY . .
RUN cargo build --release

# FROM busybox AS runtime
# FROM busybox AS runtime
FROM public.ecr.aws/debian/debian:stable-slim AS runtime
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && update-ca-certificates
# RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && update-ca-certificates
COPY --from=builder /workspace/morrow/apps/backend/target/release/backend /usr/local/bin
EXPOSE 3000
CMD ["backend"]
