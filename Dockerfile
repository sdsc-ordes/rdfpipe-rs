ARG BASE_IMAGE=ekidd/rust-musl-builder:latest

# Build environment
FROM ${BASE_IMAGE} AS builder

# Add source code under new user
ADD --chown=rust:rust . ./

RUN cargo build --release

# Runner image
FROM alpine:latest
RUN apk --no-cache add ca-certificates
COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/rdfpipe-rs \
    /usr/local/bin/

CMD /usr/local/rdfpipe-rs

