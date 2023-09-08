use oxigraph::io::read::TripleReader;
use oxigraph::io::{GraphFormat, GraphParser, GraphSerializer};
use std::io::{self, BufRead};

// Call the appropriate parser based on the input RDF format.
pub fn parse_any_rdf<R: BufRead>(
    src: R,
    format: GraphFormat,
) -> Result<TripleReader<R>, oxigraph::io::read::ParseError> {
    GraphParser::from_format(format).read_triples(src)
}

pub fn serialize_any_rdf<W: std::io::Write>(
    dest: W,
    format: GraphFormat,
) -> Result<oxigraph::io::write::TripleWriter<W>, io::Error> {
    GraphSerializer::from_format(format).triple_writer(dest)
}
