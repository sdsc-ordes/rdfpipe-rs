mod cli;
mod utils;

use crate::cli::Args;
use clap::Parser;
use oxigraph::io::read::ParseError;
use oxigraph::io::GraphFormat;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

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
// Infer RDF serialization format from file extension
// If the extension is missing or unknown, None is returned.
fn format_from_path<'a>(path: &'a str) -> Option<GraphFormat> {
    let ext = Path::new(path).extension()?.to_str()?;
    GraphFormat::from_extension(ext)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    // If input specified in CLI, use that format
    // Otherwise, infer format from file extension
    let input_format = match args.input_format {
        Some(format) => GraphFormat::from(&format),
        None => args
            .input_file
            .as_ref()
            .and_then(|path| format_from_path(path))
            .ok_or_else(|| "Could not infer input format")?,
    };

    let input_buf = get_input_buf(args.input_file.as_deref())?;

    let output_format = GraphFormat::from(&args.output_format.unwrap());
    let parser = utils::parse_any_rdf(input_buf, input_format)?;
    let mut writer = utils::serialize_any_rdf(std::io::stdout(), output_format)?;
    for triple in parser {
        writer.write(triple?.as_ref())?;
    }
    writer.finish()?;

    Ok(())
}
