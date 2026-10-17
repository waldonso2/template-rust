//! Command line: reads standard input, prints the most frequent words.

use std::io::{self, Read, Write};
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut text = String::new();
    if let Err(e) = io::stdin().read_to_string(&mut text) {
        eprintln!("tool: cannot read the input: {e}");
        return ExitCode::FAILURE;
    }
    let mut out = io::stdout().lock();
    for (word, n) in tool::word_counts(&text).into_iter().take(10) {
        if writeln!(out, "{n}\t{word}").is_err() {
            return ExitCode::FAILURE;
        }
    }
    ExitCode::SUCCESS
}
