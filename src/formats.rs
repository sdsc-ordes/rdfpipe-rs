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
//! # Implementation of concrete RDF formats
//!
//! This module implements `RdfIO` trait for each RDF serialization format.
use crate::cli::GraphFormat;
use crate::converter::RdfIO;
use crate::io::{Input, Output};
use sophia::inmem::graph::FastGraph;
use sophia::turtle::parser::nt::NTriplesParser;
use sophia::turtle::parser::turtle::TurtleParser;
use sophia::turtle::serializer::nt::NtSerializer;
use sophia::turtle::serializer::turtle::TurtleSerializer;
use sophia::xml::parser::RdfXmlParser;
use sophia::xml::serializer::RdfXmlSerializer;

pub(crate) struct NTriples;
pub(crate) struct Turtle;
pub(crate) struct RdfXml;

/// The `RdfParser` struct provides a generic interface to parse RDF graphs
/// from different formats.
pub struct RdfParser {
    pub graph: FastGraph,
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

/// The `RdfSerializer` struct provides a generic interface to serialize
/// RDF graphs to different formats.
pub struct RdfSerializer;

impl RdfSerializer {
    pub fn serialize(dest: Output, format: GraphFormat, graph: FastGraph) -> Result<(), String> {
        match format {
            GraphFormat::NTriples => NTriples.serialize(dest, graph),
            GraphFormat::Turtle => Turtle.serialize(dest, graph),
            GraphFormat::RdfXml => RdfXml.serialize(dest, graph),
        }
    }
}
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
