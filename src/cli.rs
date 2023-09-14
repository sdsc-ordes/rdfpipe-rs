use clap::builder::PossibleValue;
use clap::Parser;
use clap::ValueEnum;
use oxigraph::io::GraphFormat;

// This lets clap automate validation of
// RDF formats from the command line
#[derive(Clone, Debug)]
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

impl ValueEnum for ArgGraphFormat {
    fn from_str(input: &str, ignore_case: bool) -> Result<Self, String> {
        let fmt_str: String;
        if ignore_case {
            fmt_str = input.to_lowercase();
        } else {
            fmt_str = input.to_string();
        }
        match fmt_str.as_str() {
            "ntriples" | "nt" | "n-triples" => Ok(ArgGraphFormat::NTriples),
            "xml" | "rdf/xml" | "rdf-xml" => Ok(ArgGraphFormat::RdfXml),
            "ttl" | "turtle" => Ok(ArgGraphFormat::Turtle),
            _ => Err(format!("Unknown format: {}", input)),
        }
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        match self {
            ArgGraphFormat::NTriples => Some(PossibleValue::new("ntriples")),
            ArgGraphFormat::RdfXml => Some(PossibleValue::new("rdf-xml")),
            ArgGraphFormat::Turtle => Some(PossibleValue::new("turtle")),
        }
    }

    fn value_variants<'a>() -> &'a [Self] {
        &[
            ArgGraphFormat::Turtle,
            ArgGraphFormat::NTriples,
            ArgGraphFormat::RdfXml,
        ]
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
