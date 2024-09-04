# rdfpipe-rs

> [!WARNING]
> This is a WIP repository, it is not yet stable.

`rdfpipe-rs` is meant to replicate the command line interface of rdfpipe, a python-based command line tool bundled with [rdflib](https://github.com/RDFLib) that allows conversion between RDF exchange file formats (turtle, json-ld, rdf/xlm, ntriples, ...). The goal of `rdfpipe-rs` is to be a drop in replacement which provides better performance to handle larger files.

## Current status

rdfpipe-rs is still missing the following features:

* json-ld support
* quad support (named graphs)
* `--ns` option for explicit namespace binding


## Installation

The package must be compiled from source using [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html):

```sh
git clone https://github.com/sdsc-ordes/rdfpipe-rs
cd rdfpipe-rs
cargo build --release
# executable binary located in ./target/release/rdfpipe-rs
```


## Usage

The command line interface follows [rdfpipe](https://manpages.ubuntu.com/manpages/impish/man1/rdfpipe.1.html)'s as closely as possible:


```
$ rdfpipe-rs --help

RDF conversion tool

Usage: rdfpipe-rs [OPTIONS] [INPUT_FILE]

Arguments:
  [INPUT_FILE]  Input file. Omit or use - for stdin. [default: -]

Options:
      --no-guess                       Don't guess format based on file suffix.
      --no-out                         Don't output the resulting graph (useful for checking validity of input).
  -i, --input-format <INPUT_FORMAT>    Input RDF serialization format [possible values: turtle, n-triples, rdf-xml]
  -o, --output-format <OUTPUT_FORMAT>  Output RDF serialization format [default: turtle] [possible values: turtle, n-triples, rdf-xml]
  -h, --help                           Print help
```

## Development

See [CONTRIBUTING.md](CONTRIBUTING.md) to learn about the different ways you can contribute to rdfpipe-rs.

```sh
# Install for development
git clone https://github.com/sdsc-ordes/rdfpipe-rs
cd rdfpipe-rs

# Run unit and integration tests
cargo test

# Build documentation
cargo doc

# Build debug binary
cargo build

# Build optimized binary
cargo build --release

# Install
cargo install --path .
```
