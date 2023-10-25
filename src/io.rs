use crate::cli::GraphFormat;
use crate::formats::{NTriples, RdfXml, Turtle};
use crate::input::Input;
use sophia::api::prelude::{Graph, TripleParser};
use sophia::api::serializer::TripleSerializer;
use sophia::api::source::TripleSource;
use sophia::inmem::graph::FastGraph;
use sophia::turtle::parser::nt::NTriplesParser;
use sophia::turtle::parser::turtle::TurtleParser;
use sophia::turtle::serializer::nt::NtSerializer;
use sophia::turtle::serializer::turtle::TurtleSerializer;
use sophia::xml::parser::RdfXmlParser;
use sophia::xml::serializer::RdfXmlSerializer;
use std::io::{stdin, BufRead, BufReader, BufWriter, Read, Stdin, Stdout, Write};

pub struct RdfParser {
    pub graph: FastGraph,
}

pub struct RdfSerializer;

pub trait RdfIO<'a, W: Write, P: TripleParser<Input>, F: TripleSerializer> {
    fn parse(&self, input: Input) -> Result<FastGraph, String> {
        let mut graph = FastGraph::new();
        match self.parser().parse(input).add_to_graph(&mut graph) {
            Ok(_) => Ok(graph),
            Err(_) => Err(String::from("Could not parse graph")),
        }
    }
    fn serialize(&self, writer: W, graph: FastGraph) -> Result<(), String> {
        let mut formatter = self.serializer(writer);
        match formatter.serialize_graph(&graph) {
            Ok(_) => Ok(()),
            Err(_) => Err(String::from("Could not serialize graph")),
        }
    }
    fn parser(&self) -> P;
    fn serializer(&self, writer: W) -> F;
}

impl RdfParser {
    pub fn new(input: Input, format: GraphFormat) -> Result<Self, String> {
        Ok(RdfParser {
            graph: match format {
                GraphFormat::NTriples => {
                    match <NTriples as RdfIO<
                        '_,
                        BufWriter<Stdout>,
                        NTriplesParser,
                        NtSerializer<BufWriter<Stdout>>,
                    >>::parse(&NTriples, input)
                    {
                        Ok(graph) => graph,
                        Err(_) => Err(String::from("Could not load NTriples"))?,
                    }
                }
                GraphFormat::Turtle => {
                    match <Turtle as RdfIO<
                        '_,
                        BufWriter<Stdout>,
                        TurtleParser,
                        TurtleSerializer<BufWriter<Stdout>>,
                    >>::parse(&Turtle, input)
                    {
                        Ok(graph) => graph,
                        Err(_) => Err(String::from("Could not load Turtle"))?,
                    }
                }
                GraphFormat::RdfXml => {
                    match <RdfXml as RdfIO<
                        '_,
                        BufWriter<Stdout>,
                        RdfXmlParser,
                        RdfXmlSerializer<BufWriter<Stdout>>,
                    >>::parse(&RdfXml, input)
                    {
                        Ok(graph) => graph,
                        Err(_) => Err(String::from("Could not load RDF/XML"))?,
                    }
                }
                _ => Err(String::from("Unsupported file format"))?,
            },
        })
    }
}

impl RdfSerializer {
    pub fn serialize<W: Write>(
        dest: W,
        format: GraphFormat,
        graph: FastGraph,
    ) -> Result<(), String> {
        match format {
            GraphFormat::NTriples => NTriples.serialize(dest, graph),
            GraphFormat::Turtle => Turtle.serialize(dest, graph),
            GraphFormat::RdfXml => RdfXml.serialize(dest, graph),
            _ => Err(String::from("Unsupported file format")),
        }
    }
}
