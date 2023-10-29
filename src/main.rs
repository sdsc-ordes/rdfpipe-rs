//! # rdfpipe-rs
//!
//! A command-line tool for converting between RDF serialization formats.
//!
//! ## Usage
//!
//! ```bash
//! Usage: rdfpipe-rs [OPTIONS] [INPUT_FILE]
//!
//! Arguments:
//!   [INPUT_FILE]  Input file. Omit or use - for stdin. [default: -]
//!
//! Options:
//!       --no-guess                       Don't guess format based on file suffix.
//!       --no-out                         Don't output the resulting graph (useful for checking validity of input).
//!   -i, --input-format <INPUT_FORMAT>    Input RDF serialization format [possible values: turtle, n-triples, rdf-xml]
//!   -o, --output-format <OUTPUT_FORMAT>  Output RDF serialization format [default: turtle] [possible values: turtle, n-triples, rdf-xml]
//!   -h, --help                           Print help
//! ```
//!
//! ## Examples
//!
//! ```bash
//! # Convert from Turtle to RDF/XML
//! rdfpipe-rs -i turtle -o rdf-xml input.ttl > output.rdf
//!
//! # Input format can be inferred from file extension
//! rdfpipe-rs -o xml input.ttl > output.rdf
//!
//! # Shortcut notations are also supported
//! head -n 1000 input.ttl \
//! | rdfpipe-rs -i ttl -o nt \
//! | grep 'example.org' \
//! > output.nt
//! ```

mod cli;
mod converter;
mod formats;
mod io;

use crate::cli::{Args, GraphFormat};
use crate::io::{Input, Output};
use clap::Parser;
use formats::{RdfParser, RdfSerializer};
use std::error::Error;
use std::path::Path;

/// Infer RDF serialization format from file extension
/// If the extension is missing or unknown, None is returned.
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
    let output = Output::new(None);
    let parser = RdfParser::new(input, input_format)?;
    RdfSerializer::serialize(output, output_format, parser.graph)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_cmd::Command;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_format_from_path() {
        assert_eq!(format_from_path("file.ttl"), Some(GraphFormat::Turtle));
        assert_eq!(format_from_path("file.nt"), Some(GraphFormat::NTriples));
        assert_eq!(format_from_path("file.rdf"), Some(GraphFormat::RdfXml));
        assert_eq!(format_from_path("file.unknown"), None);
        assert_eq!(format_from_path("file"), None);
    }

    #[test]
    fn test_main() -> Result<(), Box<dyn Error>> {
        let dir = tempdir()?;
        let input_file = dir.path().join("input.ttl");

        let mut input = File::create(&input_file)?;
        writeln!(
            input,
            "<http://example.org> <http://example.org/predicate> \"object\" ."
        )?;

        let mut cmd = Command::cargo_bin("rdfpipe-rs").unwrap();
        let status = cmd
            .arg("-i")
            .arg("turtle")
            .arg("-o")
            .arg("rdf-xml")
            .arg(&input_file)
            .assert();

        status.success();

        Ok(())
    }
}
