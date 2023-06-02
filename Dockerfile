ARG CARGO_BUILD_TARGET="x86_64-unknown-linux-musl"

FROM rust:1.66.1-alpine3.17 AS config
WORKDIR /app
RUN addgroup -S rust && adduser -S rust -G rust
RUN apk update
RUN apk add pkgconfig
RUN apk add musl-dev
RUN apk add libressl-dev
RUN apk add --no-cache ca-certificates

FROM config AS chef
RUN cargo install cargo-chef --locked

FROM chef AS prepare
COPY . .
RUN cargo chef prepare --recipe-path recipe.json


FROM chef AS cacher
ARG CARGO_BUILD_TARGET
COPY --from=prepare /app/recipe.json recipe.json
RUN cargo chef cook --release --target $CARGO_BUILD_TARGET --recipe-path recipe.json


FROM config AS builder
ARG CARGO_BUILD_TARGET
COPY . .
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo/ /usr/local/cargo/
RUN cargo build --release --target $CARGO_BUILD_TARGET --bin api
RUN strip target/$CARGO_BUILD_TARGET/release/api


FROM scratch AS runtime
ARG CARGO_BUILD_TARGET
WORKDIR /usr/local/bin
COPY --from=0 /etc/passwd /etc/passwd
COPY --from=0 /etc/group /etc/group
ADD ca-certificates /usr/local/bin/ca-certificates
USER rust:rust
COPY --from=builder /app/target/$CARGO_BUILD_TARGET/release/api /usr/local/bin
ENTRYPOINT ["api"]