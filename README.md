# PLATZHALTER: Projektname

PLATZHALTER: was das Projekt macht, in zwei Sätzen.

Aus der Vorlage [`template-rust`](https://github.com/waldonso2/template-rust) des
[quality-kit](https://github.com/waldonso2/quality-kit): Sicherheit und Leistung werden in der CI geprüft, die
Kennzahlen stehen in [`KENNZAHLEN.md`](KENNZAHLEN.md).

## Nach „Use this template“ ersetzen

- [ ] `PLATZHALTER` in `README.md`, `CLAUDE.md`, `KENNZAHLEN.md`; `name`/`description` in `Cargo.toml` und der
      Crate-Name `tool` in `src/main.rs`, `tests/`
- [ ] die Beispiel-Logik (`src/lib.rs`)
- [ ] die Einstellungen aus [`SETUP.md`](https://github.com/waldonso2/quality-kit/blob/main/SETUP.md) setzen

## Entwickeln

```bash
cargo fmt && cargo clippy --all-targets -- -D warnings && cargo test
cargo install --locked cargo-llvm-cov cargo-deny        # einmal
cargo llvm-cov --ignore-filename-regex 'src/main\.rs' --fail-under-lines 90
cargo deny check
cargo test --release --test perf_report -- --ignored --nocapture   # Performance-Bericht (perf-report.md)
echo "ein text ein" | cargo run
```

## Security baseline

This template enforces the shared security-exception policy in CI. Keep `security-exceptions.json` as `[]` unless an explicitly approved, time-limited exception is required. See the [quality-kit security baseline](https://github.com/waldonso2/quality-kit/blob/main/SECURITY_BASELINE.md), [project checklist](https://github.com/waldonso2/quality-kit/blob/main/SECURITY_CHECKLIST.md), and [setup requirements](https://github.com/waldonso2/quality-kit/blob/main/SETUP.md). Configure `security-policy` as a required status check in the repository ruleset.
