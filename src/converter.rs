// rdfpipe-rs
// Copyright (C) 2023 - Swiss Data Science Center (SDSC)
// A partnership between École Polytechnique Fédérale de Lausanne (EPFL) and
// Eidgenössische Technische Hochschule Zürich (ETHZ).
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
//! # Conversion logic
//!
//! This module contains the `RdfIO` trait which is used to parse and serialize RDF graphs.
//! Each RDF serialization format should implement this trait.
//!
use std::io::{BufRead, Write};
use std::boxed::Box;

use sophia::api::prelude::TripleParser;
use sophia::api::serializer::TripleSerializer;
use sophia::api::source::TripleSource;
use sophia::inmem::graph::FastGraph;

/// The `RdfIO` trait is used to parse and serialize RDF graphs.

pub trait RdfIO<'a, P: TripleParser<Box<dyn BufRead>>, F: TripleSerializer> {
    /// Parse an RDF graph from an input source to an in-memory graph.
    fn parse(&self, input: Box<dyn BufRead>) -> Result<FastGraph, String> {
        let mut graph = FastGraph::new();
        match self.parser().parse(input).add_to_graph(&mut graph) {
            Ok(_) => Ok(graph),
            Err(_) => Err(String::from("Could not parse graph")),
        }
    }

    /// Serialize an in-memory RDF graph to an output source.
    fn serialize(&self, writer: Box<dyn Write>, graph: FastGraph) -> Result<(), String> {
        let mut formatter = self.serializer(writer);
        match formatter.serialize_graph(&graph) {
            Ok(_) => Ok(()),
            Err(_) => Err(String::from("Could not serialize graph")),
        }
    }

    /// Create a new parser for this format.
    fn parser(&self) -> P;

    /// Create a new serializer for this format.
    fn serializer(&self, writer: Box<dyn Write>) -> F;
}
