mod cli;
mod utils;

use crate::cli::Args;
use crate::utils::GraphFormat;
use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

// Gets a BufReader from a file or standard input. If input_path is None or "-",
// return a reader from stin, if it points to a path, open from a file.
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
    //
    // If input specified in CLI, use that format
    // Otherwise, infer format from file extension
    // unless --no-guess was provided
    let input_format = match (args.input_format, args.no_guess) {
        (Some(format), _) => GraphFormat::from(&format),
        (None, true) => Err("Could not infer input format.")?,
        (None, false) => args
            .input_file
            .as_ref()
            .and_then(|path| format_from_path(path))
            .ok_or_else(|| "Could not infer input format")?,
    };
    let output_format = args.output_format;

    let input_buf = get_input_buf(args.input_file.as_deref())?;
    let parser = utils::parse_any_rdf(input_buf, input_format)?;
    let mut writer = utils::serialize_any_rdf(std::io::stdout(), output_format)?;

    // Skip output if --no-out enabled
    if let true = args.no_out {
        for triple in parser {
            _ = triple?.as_ref();
        }
    } else {
        for triple in parser {
            writer.write(triple?.as_ref())?;
        }
        writer.finish()?;
    }

    Ok(())
}
