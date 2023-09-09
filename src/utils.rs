use oxigraph::io::read::{ParseError, TripleReader};
use oxigraph::io::write::TripleWriter;
use oxigraph::io::{GraphFormat, GraphParser, GraphSerializer};
use std::io::{self, BufRead, Write};

// Call the appropriate parser based on the input RDF format.
pub fn parse_any_rdf<R: BufRead>(
    src: R,
    format: GraphFormat,
) -> Result<TripleReader<R>, ParseError> {
    GraphParser::from_format(format).read_triples(src)
}

pub fn serialize_any_rdf<W: Write>(
    dest: W,
    format: GraphFormat,
) -> Result<TripleWriter<W>, io::Error> {
    GraphSerializer::from_format(format).triple_writer(dest)
}
