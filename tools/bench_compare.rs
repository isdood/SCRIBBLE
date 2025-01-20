//! Scribble Benchmark Comparison Tool
//! ================================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-20
//! Last Updated: 2025-01-20 17:32:54 UTC
//! Version: 0.1.0
//! License: MIT

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, BufReader};
use std::path::Path;
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize)]
struct BenchmarkResult {
    name: String,
    median: f64,
    min: f64,
    max: f64,
    outliers: usize,
    unit: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CriterionData {
    results: Vec<BenchmarkResult>,
}

fn load_benchmark_data(path: &Path) -> io::Result<Vec<BenchmarkResult>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let data: CriterionData = serde_json::from_reader(reader)?;
    Ok(data.results)
}

fn format_duration(ns: f64) -> String {
    if ns < 1.0 {
        format!("{:.2} ps", ns * 1000.0)
    } else {
        format!("{:.2} ns", ns)
    }
}

fn compare_results(ziggy: &[BenchmarkResult], magicmath: &[BenchmarkResult]) -> String {
    let mut output = String::new();

    output.push_str(&format!("Scribble Vector Operations Benchmark Comparison\n"));
    output.push_str(&format!("==========================================\n"));
    output.push_str(&format!("Generated: 2025-01-20 17:32:54 UTC\n"));
    output.push_str(&format!("User: isdood\n\n"));

    let mut ops: HashMap<&str, (&BenchmarkResult, &BenchmarkResult)> = HashMap::new();

    // Group matching operations
    for z in ziggy {
        if let Some(m) = magicmath.iter().find(|m| m.name == z.name) {
            ops.insert(&z.name, (z, m));
        }
    }

    // Dot Product comparisons
    output.push_str("Dot Product Operations\n");
    output.push_str("---------------------\n");
    for (name, (z, m)) in ops.iter().filter(|(n, _)| n.contains("dot")) {
        let diff_percent = ((m.median - z.median) / z.median) * 100.0;
        output.push_str(&format!(
            "{}\n  Ziggy:     {} (±{:.1}%)\n  MagicMath: {} (±{:.1}%)\n  Difference: {:+.1}%\n\n",
                                 name,
                                 format_duration(z.median),
                                     (z.max - z.min) / z.median * 50.0,
                                 format_duration(m.median),
                                     (m.max - m.min) / m.median * 50.0,
                                 diff_percent
        ));
    }

    // Magnitude comparisons
    output.push_str("\nMagnitude Operations\n");
    output.push_str("-------------------\n");
    for (name, (z, m)) in ops.iter().filter(|(n, _)| n.contains("magnitude")) {
        let diff_percent = ((m.median - z.median) / z.median) * 100.0;
        output.push_str(&format!(
            "{}\n  Ziggy:     {} (±{:.1}%)\n  MagicMath: {} (±{:.1}%)\n  Difference: {:+.1}%\n\n",
                                 name,
                                 format_duration(z.median),
                                     (z.max - z.min) / z.median * 50.0,
                                 format_duration(m.median),
                                     (m.max - m.min) / m.median * 50.0,
                                 diff_percent
        ));
    }

    output
}

fn main() -> io::Result<()> {
    let ziggy_results = Path::new("lib/ziggy/target/criterion/report/data.json");
    let magicmath_results = Path::new("lib/magicmath/target/criterion/report/data.json");

    println!("Loading benchmark results...");
    let ziggy_data = load_benchmark_data(ziggy_results)?;
    let magicmath_data = load_benchmark_data(magicmath_results)?;

    println!("{}", compare_results(&ziggy_data, &magicmath_data));

    Ok(())
}
