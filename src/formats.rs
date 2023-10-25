use crate::io::RdfIO;
use sophia::turtle::parser::nt::NTriplesParser;
use sophia::turtle::parser::turtle::TurtleParser;
use sophia::turtle::serializer::nt::NtSerializer;
use sophia::turtle::serializer::turtle::TurtleSerializer;
use sophia::xml::parser::RdfXmlParser;
use sophia::xml::serializer::RdfXmlSerializer;
use std::fs::File;
use std::io::{BufWriter, Write};

pub(crate) struct NTriples;
pub(crate) struct Turtle;
pub(crate) struct RdfXml;

impl<'a, W: Write> RdfIO<'a, W, NTriplesParser, NtSerializer<W>> for NTriples {
    fn parser(&self) -> NTriplesParser {
        NTriplesParser {}
    }

    fn serializer(&self, writer: W) -> NtSerializer<W> {
        NtSerializer::new(writer)
    }
}

impl<'a, W: Write> RdfIO<'a, W, TurtleParser, TurtleSerializer<W>> for Turtle {
    fn parser(&self) -> TurtleParser {
        TurtleParser { base: None }
    }

    fn serializer(&self, writer: W) -> TurtleSerializer<W> {
        TurtleSerializer::new(writer)
    }
}

impl<'a, W: Write> RdfIO<'a, W, RdfXmlParser, RdfXmlSerializer<W>> for RdfXml {
    fn parser(&self) -> RdfXmlParser {
        RdfXmlParser { base: None }
    }

    fn serializer(&self, writer: W) -> RdfXmlSerializer<W> {
        RdfXmlSerializer::new(writer)
    }
}
