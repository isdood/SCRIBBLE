//! Error integration benchmarks
//! Created: 2025-01-20 22:14:29
//! Author: isdood

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use error_core::Diagnose;
use error_derive::Diagnose;
use error_integration::{DefaultErrorReporter, ErrorContext, ErrorReporter, ReporterConfig};
use std::time::Duration;
use thiserror::Error;

#[derive(Debug, Error, Diagnose)]
enum BenchError {
    #[error("simple benchmark error")]
    #[diagnose(detect = "basic benchmark condition", suggestion = "optimize the code")]
    Basic,

    #[error("complex benchmark error: {msg}")]
    #[diagnose(detect = "complex benchmark condition", suggestion = "review and optimize")]
    Complex {
        msg: String,
        #[diagnose(field_info = "error code")]
        code: i32,
    },
}

fn create_reporter() -> DefaultErrorReporter {
    let config = ReporterConfig {
        include_backtraces: false, // Disable for consistent benchmarking
        collect_stats: true,
        max_errors_per_type: 1000,
        ..Default::default()
    };
    DefaultErrorReporter::new(config)
}

fn error_reporting_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("error_reporting");
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(100);

    let reporter = create_reporter();

    // Benchmark basic error reporting
    let basic_error = BenchError::Basic;
    group.bench_function("basic_error", |b| {
        b.iter(|| {
            let context = ErrorContext::current();
            reporter.report_error(black_box(&basic_error), black_box(&context));
        })
    });

    // Benchmark complex error reporting
    let complex_error = BenchError::Complex {
        msg: "benchmark test".to_string(),
        code: 42,
    };
    group.bench_function("complex_error", |b| {
        b.iter(|| {
            let context = ErrorContext::current().with_extra("benchmark context");
            reporter.report_error(black_box(&complex_error), black_box(&context));
        })
    });

    group.finish();
}

fn context_creation_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("context_creation");

    // Benchmark basic context creation
    group.bench_function("basic", |b| {
        b.iter(|| {
            black_box(ErrorContext::current());
        })
    });

    // Benchmark context with extra information
    group.bench_function("with_extra", |b| {
        b.iter(|| {
            black_box(ErrorContext::current().with_extra("benchmark extra info"));
        })
    });

    group.finish();
}

fn error_stats_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("error_stats");
    let reporter = create_reporter();

    // Pre-fill with different numbers of errors
    for size in [10, 100, 1000].iter() {
        let error = BenchError::Basic;
        let context = ErrorContext::current();

        for _ in 0..*size {
            reporter.report_error(&error, &context);
        }

        group.bench_with_input(BenchmarkId::new("stats_retrieval", size), size, |b, _| {
            b.iter(|| {
                black_box(reporter.get_stats());
            })
        });

        reporter.reset_stats();
    }

    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default()
    .with_plots()
    .sample_size(50)
    .measurement_time(Duration::from_secs(5));
    targets = error_reporting_benchmark, context_creation_benchmark, error_stats_benchmark
}
criterion_main!(benches);
