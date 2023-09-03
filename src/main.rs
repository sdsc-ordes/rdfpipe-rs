mod cli;
mod utils;

use crate::cli::Args;
use clap::Parser;

// Should replicate rdfpipe's command line interface
// rdfpipe [-i {ttl,nt}] [-o {ttl,nt}] [input]
// if input is - (default), read from stdin and infer format from file contents
// if input is specified, infer format from file extension

fn main() {
    let args = Args::parse();
    if let Some("-") = args.input_file.as_deref() {
        println!("Reading from stdin");
    }

    println!("{:?}", args);
}
