use std::error::Error;
use assert_cmd::Command;
use std::fs::File;
use std::io::Write;
use tempfile::tempdir;

#[test]
fn test_conversion() -> Result<(), Box<dyn Error>> {
    let dir = tempdir()?;
    let input_file = dir.path().join("input.ttl");

    let mut input = File::create(&input_file)?;
    writeln!(
        input,
        "<http://example.org> <http://example.org/predicate> \"object\" ."
    )?;

    let mut cmd = Command::cargo_bin("rdfpipe-rs").unwrap();
    let status = cmd
        .arg("-i")
        .arg("turtle")
        .arg("-o")
        .arg("rdf-xml")
        .arg(&input_file)
        .assert();

    status.success();

    Ok(())
}
