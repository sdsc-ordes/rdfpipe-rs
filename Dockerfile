# This Dockerfile uses a multi-stage build with
# cargo-chef to cache compiled dependencies and
# minimize the size of the final image.

# 1: Read code and write recipe file
FROM rust AS planner
WORKDIR /app
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# 2: Cache compiled dependencies for faster builds
FROM rust AS cacher
WORKDIR /app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json


# 3: Build project, but reuse cached dependencies
FROM rust AS builder

# Create unprivileged user
ENV USER=rust
ENV UID=1001
 
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

COPY . /app
WORKDIR /app

# Copy pre-built dependencies
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo

RUN cargo build --release


# 4: Copy required binaries into distroless runner
FROM gcr.io/distroless/cc-debian12 AS runner

WORKDIR /app

# Import user files
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

# Import binary
COPY --from=builder \
    /app/target/release/rdfpipe-rs \
    /app/rdfpipe-rs

# Use unprivileged user
USER rust:rust

ENV PATH="/app/:${PATH}"
ENTRYPOINT ["rdfpipe-rs"]

