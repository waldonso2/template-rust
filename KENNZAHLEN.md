# Kennzahlen

Woran dieses Projekt Sicherheit, Bedienbarkeit und Leistung misst, gegliedert nach **ISO/IEC 25010**.
**Blockierend:** Die CI scheitert, wenn der Wert nicht erreicht wird. Diese Jobs sind die Pflicht-Checks des
Branch-Schutzes ([`SETUP.md`](https://github.com/waldonso2/quality-kit/blob/main/SETUP.md)). **Bericht:** Der Wert
wird gemessen und auf der Zusammenfassungsseite des CI-Laufs gezeigt (Job `performance`), hält die CI aber nie an.

## Sicherheit

| Kennzahl | Ziel | Referenz | Wo gemessen | Art |
|---|---|---|---|---|
| Anforderungen an die Anwendung | Stufe 2 | OWASP ASVS 5.0 | Reviews, Tests | Leitlinie |
| Häufigste Risiken abgedeckt | alle 10 | OWASP Top 10 (2025) | Reviews, CodeQL | Leitlinie |
| Funde der Code-Analyse | 0 offen | CodeQL (`codeql.yml`, security-extended) | Security → Code scanning | Bericht |
| Funde in den Workflows | 0 ab „medium“ | zizmor | Job `workflows` | blockierend |
| Bekannte Lücken, nicht gepflegte Crates | 0 | cargo-deny (RustSec), Dependency Review | Jobs `deny`, `dependency-review` | blockierend |
| Lizenzen und Herkunft der Crates | nur erlaubte (`deny.toml`), nur crates.io | cargo-deny | Job `deny` | blockierend |
| Abhängigkeiten | nur aus `Cargo.lock` (`--locked`) | eigene Regel | alle Jobs | blockierend |
| Kein `unsafe` | verboten (`unsafe_code = "forbid"`) | eigene Regel | Compiler | blockierend |
| Sicherheits- und Robustheitsregeln | 0 Funde (`unwrap`, `expect`, `panic` als Warnung) | clippy, `-D warnings` | Job `test` | blockierend |
| Geheimnisse im Repository | 0 | gitleaks | Job `secrets` | blockierend |
| Sicherheit des Repositorys | ≥ 7 von 10 | OpenSSF Scorecard | `scorecard.yml` | Bericht |
| Zeit bis zur Behebung einer Lücke | kritisch ≤ 7 Tage, hoch ≤ 30 Tage | übliche Fristen | Dependabot, Advisories | Leitlinie |

## Bedienbarkeit

| Kennzahl | Ziel | Referenz | Wo gemessen | Art |
|---|---|---|---|---|
| Fehler | verständliche Meldung, Exit-Code ≠ 0, kein Traceback | eigene Regel | Tests der CLI | Leitlinie |
| Rückmeldung | ohne Unterbrechung ≤ 1 s, ab 10 s Fortschritt | Nielsen | Gestaltung | Leitlinie |

## Leistung

Gemessen in `tests/perf_report.rs` (`cargo test --release --test perf_report -- --ignored`) auf dem CI-Runner; entscheidend ist die Entwicklung von Lauf zu Lauf.

| Kennzahl | Ziel | Referenz | Art |
|---|---|---|---|
| Kernfunktion (`word_counts` über 1 Mio. Wörter) | ≤ 1 s | Nielsen 1 s | Bericht |
| PLATZHALTER: die Kernoperation des Projekts | … | … | Bericht |

## Wartbarkeit

| Kennzahl | Ziel | Wo gemessen | Art |
|---|---|---|---|
| Testabdeckung (Zeilen, ohne `main.rs`) | ≥ 90 % | cargo-llvm-cov, Job `test` | blockierend |
| Lint | 0 Warnungen (clippy `all` + `pedantic`) | Job `test` | blockierend |
| Formatierung | `cargo fmt --check` | Job `test` | blockierend |
