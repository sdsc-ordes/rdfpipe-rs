use clap::Parser;
use clap::ValueEnum;
use oxigraph::io::GraphFormat;

// This lets clap automate validation of
// RDF formats from the command line
#[derive(ValueEnum, Clone, Debug)]
pub(crate) enum ArgGraphFormat {
    Turtle,
    NTriples,
    RdfXml,
}

// Helper mappings to convert from helper CLI enum
// to corresponding values in oxigraph's enum
impl From<&ArgGraphFormat> for GraphFormat {
    fn from(other: &ArgGraphFormat) -> GraphFormat {
        match other {
            ArgGraphFormat::Turtle => GraphFormat::Turtle,
            ArgGraphFormat::NTriples => GraphFormat::NTriples,
            ArgGraphFormat::RdfXml => GraphFormat::RdfXml,
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, about = "RDF conversion tool")]
pub(crate) struct Args {
    #[arg(
        short,
        long,
        default_value = "turtle",
        help = "Input RDF serialization format"
    )]
    pub(crate) input_format: Option<ArgGraphFormat>,
    #[arg(
        short,
        long,
        default_value = "turtle",
        help = "Output RDF serialization format"
    )]
    pub(crate) output_format: Option<ArgGraphFormat>,
    #[arg(default_value = "-", help = "Input file. Use - for stdin.")]
    pub(crate) input_file: Option<String>,
}
