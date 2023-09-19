use crate::utils::GraphFormat;
use clap::Parser;

// This lets clap automate validation of
// RDF formats from the command line

#[derive(Parser, Debug)]
#[command(author, about = "RDF conversion tool")]
pub(crate) struct Args {
    #[arg(long, help = "Don't guess format based on file suffix.")]
    pub(crate) no_guess: bool,
    #[arg(
        long,
        help = "Don't output the resulting graph (useful for checking validity of input)."
    )]
    pub(crate) no_out: bool,
    #[arg(short, long, help = "Input RDF serialization format")]
    pub(crate) input_format: Option<GraphFormat>,
    #[arg(
        short,
        long,
        default_value = "turtle",
        help = "Output RDF serialization format"
    )]
    pub(crate) output_format: Option<GraphFormat>,
    #[arg(default_value = "-", help = "Input file. Omit or use - for stdin.")]
    pub(crate) input_file: Option<String>,
}
