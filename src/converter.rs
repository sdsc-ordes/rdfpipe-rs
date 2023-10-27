use crate::cli::GraphFormat;
use crate::formats::{NTriples, RdfXml, Turtle};
use crate::io::{Input, Output};
use sophia::api::prelude::TripleParser;
use sophia::api::serializer::TripleSerializer;
use sophia::api::source::TripleSource;
use sophia::inmem::graph::FastGraph;

pub struct RdfParser {
    pub graph: FastGraph,
}

pub struct RdfSerializer;

pub trait RdfIO<'a, P: TripleParser<Input>, F: TripleSerializer> {
    fn parse(&self, input: Input) -> Result<FastGraph, String> {
        let mut graph = FastGraph::new();
        match self.parser().parse(input).add_to_graph(&mut graph) {
            Ok(_) => Ok(graph),
            Err(_) => Err(String::from("Could not parse graph")),
        }
    }
    fn serialize(&self, writer: Output, graph: FastGraph) -> Result<(), String> {
        let mut formatter = self.serializer(writer);
        match formatter.serialize_graph(&graph) {
            Ok(_) => Ok(()),
            Err(_) => Err(String::from("Could not serialize graph")),
        }
    }
    fn parser(&self) -> P;
    fn serializer(&self, writer: Output) -> F;
}

impl RdfParser {
    pub fn new(input: Input, format: GraphFormat) -> Result<Self, String> {
        let graph = match format {
            GraphFormat::NTriples => NTriples.parse(input),
            GraphFormat::Turtle => Turtle.parse(input),
            GraphFormat::RdfXml => RdfXml.parse(input),
        }?;
        Ok(RdfParser { graph })
    }
}

impl RdfSerializer {
    pub fn serialize(dest: Output, format: GraphFormat, graph: FastGraph) -> Result<(), String> {
        match format {
            GraphFormat::NTriples => NTriples.serialize(dest, graph),
            GraphFormat::Turtle => Turtle.serialize(dest, graph),
            GraphFormat::RdfXml => RdfXml.serialize(dest, graph),
        }
    }
}
