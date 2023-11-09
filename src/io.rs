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
//! # Input and Output helpers
//!
//! This module contains the `Input` and `Output` helper structs.
//! These structs simplify the handling of various sources and sinks.
use std::fs::File;
use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Read, Stdin, Stdout, Write};

pub enum Input {
    Stdin(BufReader<Stdin>),
    File(BufReader<File>),
}

pub enum Output {
    Stdout(BufWriter<Stdout>),
    File(BufWriter<File>),
}

impl Output {
    pub fn new(path: Option<String>) -> Self {
        match path.as_deref() {
            Some(path) => {
                let file = File::create(path).expect("Can not create file");
                Self::File(BufWriter::new(file))
            }
            None => Self::Stdout(BufWriter::new(stdout())),
        }
    }
}

impl Write for Output {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            Output::Stdout(b) => b.write(buf),
            Output::File(b) => b.write(buf),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            Output::Stdout(b) => b.flush(),
            Output::File(b) => b.flush(),
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_new_stdin() {
        let input = Input::new(None);
        match input {
            Input::Stdin(_) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_input_new_file() {
        let input = Input::new(Some("README.md".to_string()));
        match input {
            Input::File(_) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    #[should_panic(expected = "Can not open file")]
    fn test_input_new_file_panic() {
        let _input = Input::new(Some("nonexistent.txt".to_string()));
    }

    #[test]
    fn test_output_new_stdout() {
        let output = Output::new(None);
        match output {
            Output::Stdout(_) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_output_new_file() {
        let output = Output::new(Some("test.txt".to_string()));
        match output {
            Output::File(_) => assert!(true),
            _ => assert!(false),
        }
    }
}
