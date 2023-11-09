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
//! Command line interface for the RDF conversion tool.
use clap;
use clap::{Parser, ValueEnum};
use std::fmt::Error;
use std::str::FromStr;

// This lets clap automate validation of
// RDF formats from the command line
#[derive(Clone, Debug, PartialEq, ValueEnum)]
pub enum GraphFormat {
    #[clap(alias = "ttl")]
    Turtle,
    #[clap(alias = "nt", alias = "ntriples")]
    NTriples,
    #[clap(alias = "xml", alias = "rdf/xml")]
    RdfXml,
}

impl FromStr for GraphFormat {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ntriples" | "nt" | "n-triples" => Ok(GraphFormat::NTriples),
            "xml" | "rdf/xml" | "rdf-xml" => Ok(GraphFormat::RdfXml),
            "ttl" | "turtle" => Ok(GraphFormat::Turtle),
            _ => Err(Error),
        }
    }
}

impl GraphFormat {
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext {
            "nt" | "ntriples" => Some(GraphFormat::NTriples),
            "xml" | "rdf" | "owl" => Some(GraphFormat::RdfXml),
            "ttl" | "turtle" => Some(GraphFormat::Turtle),
            _ => None,
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, about = "RDF conversion tool")]
pub(crate) struct Args {
    #[arg(long, help = "Don't guess format based on file suffix.")]
    pub(crate) no_guess: bool,
    #[arg(
        long,
        help = "Don't output the resulting graph (useful for checking validity of input)."
    )]
    pub(crate) no_out: bool,
    #[arg(short, long, help = "Input RDF serialization format")]
    pub(crate) input_format: Option<GraphFormat>,
    #[arg(
        short,
        long,
        default_value = "turtle",
        help = "Output RDF serialization format"
    )]
    pub(crate) output_format: GraphFormat,
    #[arg(default_value = "-", help = "Input file. Omit or use - for stdin.")]
    pub(crate) input_file: Option<String>,
}
