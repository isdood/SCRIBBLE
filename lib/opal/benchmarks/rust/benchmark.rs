extern crate test;
use test::Bencher;
use opal::harmony::HarmonyCore;

#[bench]
fn benchmark_harmony_core(b: &mut Bencher) {
    b.iter(|| {
        let mut core = HarmonyCore::new();
        core.optimize();
    });
}
