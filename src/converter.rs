//! # Conversion logic
//!
//! This module contains the `RdfIO` trait which is used to parse and serialize RDF graphs.
//! Each RDF serialization format should implement this trait.
//!

use crate::io::{Input, Output};
use sophia::api::prelude::TripleParser;
use sophia::api::serializer::TripleSerializer;
use sophia::api::source::TripleSource;
use sophia::inmem::graph::FastGraph;

/// The `RdfIO` trait is used to parse and serialize RDF graphs.

pub trait RdfIO<'a, P: TripleParser<Input>, F: TripleSerializer> {
    /// Parse an RDF graph from an input source to an in-memory graph.
    fn parse(&self, input: Input) -> Result<FastGraph, String> {
        let mut graph = FastGraph::new();
        match self.parser().parse(input).add_to_graph(&mut graph) {
            Ok(_) => Ok(graph),
            Err(_) => Err(String::from("Could not parse graph")),
        }
    }

    /// Serialize an in-memory RDF graph to an output source.
    fn serialize(&self, writer: Output, graph: FastGraph) -> Result<(), String> {
        let mut formatter = self.serializer(writer);
        match formatter.serialize_graph(&graph) {
            Ok(_) => Ok(()),
            Err(_) => Err(String::from("Could not serialize graph")),
        }
    }

    /// Create a new parser for this format.
    fn parser(&self) -> P;

    /// Create a new serializer for this format.
    fn serializer(&self, writer: Output) -> F;
}
