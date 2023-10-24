// RDF serialization based on file extension using sophia api
use crate::cli::GraphFormat;
use sophia_api::serializer::TripleSerializer;
use sophia_jsonld::serializer::JsonLdSerializer;
use sophia_turtle::serializer::{nt, turtle};
use sophia_xml::serializer::RdfXmlSerializer;
use std::error::Error;

// RDF serialization based on file extension using sophia api
pub fn serialize_any_rdf<W: std::io::Write>(
    writer: W,
    format: GraphFormat,
) -> Result<Box<dyn sophia_api::serializer::TripleSerializer<W>>, Box<dyn Error>> {
    match format {
        GraphFormat::NTriples => Ok(Box::new(nt::NtSerializer::new(writer))),
        GraphFormat::Turtle => Ok(Box::new(turtle::TurtleSerializer::new(writer))),
        GraphFormat::RdfXml => Ok(Box::new(RdfXmlSerializer::new(writer))),
        GraphFormat::JsonLd => Ok(Box::new(JsonLdSerializer::new(writer))),
    }
}
