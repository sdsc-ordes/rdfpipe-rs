mod cli;

use crate::cli::Args;
use clap::Parser;
use sophia::parser::{nt::NTriplesParser, turtle::TurtleParser};
use sophia::serializer::{nt::NtSerializer, turtle::TurtleSerializer};
use sophia::triple::stream::TripleSource;

// Should replicate rdfpipe's command line interface
// rdfpipe [-i {ttl,nt}] [-o {ttl,nt}] [input]
// if input is - (default), read from stdin and infer format from file contents
// if input is specified, infer format from file extension

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
}
