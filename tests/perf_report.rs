//! Performance report (KENNZAHLEN.md): measured, compared with the reference values, never failing.
//!
//! Ignored by `cargo test`; the CI step runs it with continue-on-error:
//! `cargo test --release --test perf_report -- --ignored --nocapture`. The table goes to `$GITHUB_STEP_SUMMARY` (the
//! run's summary page) and to `$PERF_REPORT` (default perf-report.md). Measure the project's own main operations here.

use std::fmt::Write as _;
use std::io::Write as _;
use std::time::Instant;

struct Row {
    area: &'static str,
    measure: &'static str,
    value: f64,
    unit: &'static str,
    good: f64,
    source: &'static str,
}

fn median_ms(runs: usize, mut f: impl FnMut()) -> f64 {
    let mut times: Vec<f64> = (0..runs)
        .map(|_| {
            let t = Instant::now();
            f();
            t.elapsed().as_secs_f64() * 1000.0
        })
        .collect();
    times.sort_by(f64::total_cmp);
    times[times.len() / 2]
}

#[test]
#[ignore = "performance report: run on its own"]
fn report() -> std::io::Result<()> {
    let text = (0..1_000_000)
        .map(|i| format!("wort{}", i % 5000))
        .collect::<Vec<_>>()
        .join(" ");
    let rows = [Row {
        area: "Kern",
        measure: "word_counts über 1 Mio. Wörter",
        value: median_ms(3, || {
            std::hint::black_box(tool::word_counts(&text));
        }),
        unit: "ms",
        good: 1000.0, // Nielsen: up to 1 s the user's flow of thought stays uninterrupted.
        source: "Nielsen 1 s",
    }];
    let mut md = String::from(
        "## Performance (nur Bericht, hält die CI nicht an)\n\n| Bereich | Messung | Wert | Referenz | |\n|---|---|---|---|---|\n",
    );
    for r in &rows {
        let verdict = if r.value <= r.good { "✅" } else { "⚠️" };
        let _ = writeln!(
            md,
            "| {} | {} | {:.1} {} | ≤ {} {} ({}) | {} |",
            r.area, r.measure, r.value, r.unit, r.good, r.unit, r.source, verdict
        );
    }
    println!("{md}");
    std::fs::write(
        std::env::var("PERF_REPORT").unwrap_or_else(|_| "perf-report.md".into()),
        &md,
    )?;
    if let Ok(summary) = std::env::var("GITHUB_STEP_SUMMARY") {
        std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(summary)?
            .write_all(md.as_bytes())?;
    }
    Ok(())
}
