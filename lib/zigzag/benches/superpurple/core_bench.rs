use criterion::Criterion;
use zigzag::superpurple::core::*;

pub fn bench_core_ops(c: &mut Criterion) {
    let group = c.benchmark_group("core_operations");
    // TODO: Implement core benchmarks
    group.finish();
}
