mod cli;
mod utils;

use crate::cli::Args;
use clap::Parser;
use oxigraph::io::GraphFormat;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

// Should replicate rdfpipe's command line interface
// rdfpipe [-i {ttl,nt}] [-o {ttl,nt}] [input]
// if input is - (default) or missing, read from stdin and infer format from file contents
// if an input file is specified, infer format from file extension

fn get_input_buf(input_path: Option<&str>) -> io::Result<Box<dyn BufRead>> {
    match input_path {
        Some("-") | None => {
            // Read from stdin
            let stdin = io::stdin();
            let handle = stdin.lock();
            Ok(Box::new(handle))
        }
        Some(file_path) => {
            // Attempt to open and read from file
            let file = File::open(file_path)?;
            let reader = BufReader::new(file);
            Ok(Box::new(reader))
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let input_buf = get_input_buf(args.input_file.as_deref())?;
    let input_format = GraphFormat::from(args.input_format.clone().unwrap());
    let output_format = GraphFormat::from(args.output_format.clone().unwrap());
    let parser = utils::parse_any_rdf(input_buf, input_format)?;
    let mut writer = utils::serialize_any_rdf(std::io::stdout(), output_format)?;
    for triple in parser {
        writer.write(triple?.as_ref())?;
    }
    writer.finish()?;

    println!("{:?}", args);
    Ok(())
}
