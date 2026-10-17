//! The binary end to end.

use std::io::Write;
use std::process::{Command, Stdio};

#[test]
fn prints_the_top_words() -> Result<(), Box<dyn std::error::Error>> {
    let mut child = Command::new(env!("CARGO_BIN_EXE_tool"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    child.stdin.take().ok_or("no stdin")?.write_all(b"x y x")?;
    let out = child.wait_with_output()?;
    assert!(out.status.success());
    assert_eq!(String::from_utf8(out.stdout)?, "2\tx\n1\ty\n");
    Ok(())
}
