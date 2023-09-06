mod cli;
mod utils;

use crate::cli::Args;
use clap::Parser;
use oxigraph::io::GraphFormat;
use std::io;
use std::fs::File;

// Should replicate rdfpipe's command line interface
// rdfpipe [-i {ttl,nt}] [-o {ttl,nt}] [input]
// if input is - (default) or missing, read from stdin and infer format from file contents
// if an input file is specified, infer format from file extension

fn main() {
    let args = Args::parse();
    if let Some("-") = args.input_file.as_deref() {
        println!("Reading from stdin");
    }
    
    let input = match args.input_file {
        Some(x) => File::open(x),
        Some("-") | None => io::stdin(),
        _ => panic!("no valid input")
    };

    let input_buffer = io::BufReader::new(input);

        
    utils::parse_any_rdf(input_buffer, GraphFormat::ttl);

    println!("{:?}", args);
}
