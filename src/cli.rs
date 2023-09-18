use std::fmt::Error;
use std::str::FromStr;

use clap::Parser;
use clap::ValueEnum;
use oxigraph::io::GraphFormat;

// This lets clap automate validation of
// RDF formats from the command line
#[derive(Clone, Debug, ValueEnum)]
pub(crate) enum ArgGraphFormat {
    #[clap(alias = "ttl")]
    Turtle,
    #[clap(alias = "nt", alias = "ntriples")]
    NTriples,
    #[clap(alias = "xml", alias = "rdf/xml")]
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

impl FromStr for ArgGraphFormat {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ntriples" | "nt" | "n-triples" => Ok(ArgGraphFormat::NTriples),
            "xml" | "rdf/xml" | "rdf-xml" => Ok(ArgGraphFormat::RdfXml),
            "ttl" | "turtle" => Ok(ArgGraphFormat::Turtle),
            _ => Err(Error),
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, about = "RDF conversion tool")]
pub(crate) struct Args {
    #[arg(short, long, help = "Input RDF serialization format")]
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
