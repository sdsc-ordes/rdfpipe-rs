use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(ValueEnum, Copy, Clone, Debug)]
enum Format {
    Turtle,
    NTriples,
}

#[derive(Parser, Debug)]
#[command(author, about = "RDF conversion tool")]
pub struct Args {
    #[arg(
        short,
        long,
        default_value = "turtle",
        help = "Input RDF serialization format"
    )]
    input_format: Option<Format>,
    #[arg(
        short,
        long,
        default_value = "turtle",
        help = "Output RDF serialization format"
    )]
    output_format: Option<Format>,
    #[arg(default_value = "-", help = "Input file. Use - for stdin.")]
    input_file: Option<String>,
}
