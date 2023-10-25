mod cli;
mod formats;
mod input;
mod io;

use crate::cli::{Args, GraphFormat};
use crate::input::Input;
use clap::Parser;
use io::{RdfIO, RdfParser, RdfSerializer};
use std::error::Error;
use std::io::{stdin, stdout, BufWriter, Write};
use std::path::Path;

// Infer RDF serialization format from file extension
// If the extension is missing or unknown, None is returned.
fn format_from_path<'a>(path: &'a str) -> Option<GraphFormat> {
    let ext = Path::new(path).extension()?.to_str()?;
    GraphFormat::from_extension(ext)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    //
    // If input specified in CLI, use that format
    // Otherwise, infer format from file extension
    // unless --no-guess was provided
    let input_format = match (args.input_format, args.no_guess) {
        (Some(format), _) => format,
        (None, true) => Err("Could not infer input format.")?,
        (None, false) => args
            .input_file
            .as_ref()
            .and_then(|path| format_from_path(path))
            .ok_or_else(|| "Could not infer input format")?,
    };
    let output_format = args.output_format;

    let input = Input::new(args.input_file);
    let parser = RdfParser::new(input, input_format)?;
    RdfSerializer::serialize(BufWriter::new(stdout()), output_format, parser.graph)?;
    Ok(())
}
