use crate::converter::RdfIO;
use crate::io::Output;
use sophia::turtle::parser::nt::NTriplesParser;
use sophia::turtle::parser::turtle::TurtleParser;
use sophia::turtle::serializer::nt::NtSerializer;
use sophia::turtle::serializer::turtle::TurtleSerializer;
use sophia::xml::parser::RdfXmlParser;
use sophia::xml::serializer::RdfXmlSerializer;

pub(crate) struct NTriples;
pub(crate) struct Turtle;
pub(crate) struct RdfXml;

impl<'a> RdfIO<'a, NTriplesParser, NtSerializer<Output>> for NTriples {
    fn parser(&self) -> NTriplesParser {
        NTriplesParser {}
    }

    fn serializer(&self, writer: Output) -> NtSerializer<Output> {
        NtSerializer::new(writer)
    }
}

impl<'a> RdfIO<'a, TurtleParser, TurtleSerializer<Output>> for Turtle {
    fn parser(&self) -> TurtleParser {
        TurtleParser { base: None }
    }

    fn serializer(&self, writer: Output) -> TurtleSerializer<Output> {
        TurtleSerializer::new(writer)
    }
}

impl<'a> RdfIO<'a, RdfXmlParser, RdfXmlSerializer<Output>> for RdfXml {
    fn parser(&self) -> RdfXmlParser {
        RdfXmlParser { base: None }
    }

    fn serializer(&self, writer: Output) -> RdfXmlSerializer<Output> {
        RdfXmlSerializer::new(writer)
    }
}
