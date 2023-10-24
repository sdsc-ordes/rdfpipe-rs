// Parse any RDF format using sophia

use crate::cli::GraphFormat;
use sophia::{
    // jsonld::JsonLdParser,
    turtle::parser::{nt::NTriplesParser, turtle::TurtleParser},
    xml::parser::RdfXmlParser,
};
// requires sophia 0.8.0
use std::error::Error;

// Parse any RDF format using sophia
pub fn parse_any_rdf<R: std::io::Read>(
    reader: R,
    format: GraphFormat,
) -> Result<Box<dyn TripleParser<R, Source = R>>, Box<dyn Error>> {
    match format {
        GraphFormat::NTriples => Ok(Box::new(NTriplesParser::new(reader))),
        GraphFormat::Turtle => Ok(Box::new(TurtleParser::new(reader))),
        GraphFormat::RdfXml => Ok(Box::new(RdfXmlParser::new(reader))),
        // GraphFormat::JsonLd => Ok(Box::new(JsonLdParser::new(reader))),
    }
}
