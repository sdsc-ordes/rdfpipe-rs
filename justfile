set positional-arguments
set shell := ["bash", "-cue"]
comp_dir := justfile_directory()
root_dir := `git rev-parse --show-toplevel`
temp_dir := `mktemp -du`

# General Variables:
# You can chose either "podman" or "docker"
container_mgr := "podman"

# Build the executable.
build *args:
    cd "{{root_dir}}" && cargo build "${@:1}"

build-main *args:
    git clone https://github.com/sdsc-ordes/rdfpipe-rs.git "{{temp_dir}}" \
    && cd  "{{temp_dir}}" && cargo build "${@:1}" \
    && mv target/release/rdfpipe-rs {{root_dir}}/target/release/rdfpipe-rs-main

# Watch source and continuously build the executable.
watch:
    cd "{{root_dir}}" && cargo watch -x 'build'

# Run the executable.
run:
    cd "{{root_dir}}" && cargo run "${@:1}"

benchmark DATASET: (build "--release") (build-main "--release")
    cd "{{root_dir}}" \
    && bash ./scripts/run_bench.sh ./target/release/rdfpipe-rs-main ./target/release/rdfpipe-rs "{{DATASET}}"

format:
    cd "{{root_dir}}" && \
        {{container_mgr}} run -v "{{root_dir}}:/repo" -v "$(pwd):/workspace" -w "/workspace" \
    	instrumentisto/rust:nightly-alpine cargo fmt -- --config-path /repo
