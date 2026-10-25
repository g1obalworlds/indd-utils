//! Tests of the `indd` command line.

use std::process::Command;

/// A failed conversion leaves an existing output file as it was.
#[test]
fn failed_convert_keeps_existing_output() {
    let dir = std::env::temp_dir().join(format!("indd-cli-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let (input, output) = (dir.join("bad.indd"), dir.join("out.idml"));
    std::fs::write(&input, b"not an indd file").unwrap();
    std::fs::write(&output, b"KEEP").unwrap();

    let status = Command::new(env!("CARGO_BIN_EXE_indd"))
        .arg("convert")
        .arg(&input)
        .arg(&output)
        .status()
        .unwrap();

    assert!(!status.success());
    assert_eq!(std::fs::read(&output).unwrap(), b"KEEP");
    std::fs::remove_dir_all(&dir).unwrap();
}
