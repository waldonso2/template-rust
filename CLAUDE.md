# PLACEHOLDER: project name

PLACEHOLDER: what it does. Rust crate (`src/lib.rs` the logic, `src/main.rs` only input/output), from
`waldonso2/template-rust` (quality-kit). Docs German, CLAUDE.md, code and comments English. Toolchain in
`rust-toolchain.toml` (stable, clippy, rustfmt, llvm-tools).

## Commands

```bash
cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test
cargo llvm-cov --ignore-filename-regex 'src/main\.rs' --fail-under-lines 90
cargo deny check
cargo test --release --test perf_report -- --ignored --nocapture   # performance report, never blocking
```

## Quality rules (quality-kit - keep them)

- **Blocking:** `cargo fmt --check`, clippy `all` + `pedantic` with `-D warnings` (`unwrap`/`expect`/`panic` warn too:
  return errors instead), `unsafe_code = "forbid"`, tests with line coverage ≥ 90 % (cargo-llvm-cov, `main.rs`
  excluded - keep it to input/output), cargo-deny (RustSec advisories, unmaintained crates, licences, sources),
  dependency review, gitleaks, zizmor (≥ medium), CodeQL. Everything with `--locked`.
- **Performance is only reported, never blocking** (owner's rule): `tests/perf_report.rs` (`#[ignore]`) runs in the
  CI job `performance` with `continue-on-error` and writes its table to the run's summary; new measurements of the
  project's own main operations go there with a reference value. `KENNZAHLEN.md` lists every KPI.
- A deliberate exception carries its reason: `#[allow(clippy::<lint>)] // why`.
- **GitHub Actions and quality-kit workflows pinned to commit SHAs** with the version as a comment; Dependabot waits
  7 days (`cooldown`).
- **Secrets never in the repo**; test data is fabricated (allowed test values in `.gitleaks.toml`).
- Questions to the owner go as a clickable list (the question tool), never as free text.
