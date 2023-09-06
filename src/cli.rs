use clap::Parser;
use clap::ValueEnum;
use oxigraph::io::GraphFormat;

#[derive(Parser, Debug, ValueEnum)]
#[command(author, about = "RDF conversion tool")]
pub(crate) struct Args {
    #[arg(
        short,
        long,
        default_value = "turtle",
        help = "Input RDF serialization format"
    )]
    pub(crate) input_format: Option<GraphFormat>,
    #[arg(
        short,
        long,
        default_value = "turtle",
        help = "Output RDF serialization format"
    )]
    pub(crate) output_format: Option<GraphFormat>,
    #[arg(default_value = "-", help = "Input file. Use - for stdin.")]
    pub(crate) input_file: Option<String>,
}
