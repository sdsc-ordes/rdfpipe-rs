use std::fs::File;
use std::io::{stdin, BufRead, BufReader, Read, Stdin};

pub enum Input {
    Stdin(BufReader<Stdin>),
    File(BufReader<File>),
}

impl Input {
    pub fn new(path: Option<String>) -> Self {
        match path.as_deref() {
            Some("-") | None => Self::Stdin(BufReader::new(stdin())),
            Some(path) => {
                let file = File::open(path).expect("Can not open file");
                Self::File(BufReader::new(file))
            }
        }
    }
}

impl Read for Input {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            Input::Stdin(b) => b.read(buf),
            Input::File(b) => b.read(buf),
        }
    }
}

impl BufRead for Input {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        match self {
            Input::Stdin(b) => b.fill_buf(),
            Input::File(b) => b.fill_buf(),
        }
    }

    fn consume(&mut self, amt: usize) {
        match self {
            Input::Stdin(b) => b.consume(amt),
            Input::File(b) => b.consume(amt),
        }
    }
}
