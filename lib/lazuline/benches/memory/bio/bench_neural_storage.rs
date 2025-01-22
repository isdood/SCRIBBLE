//! Neural Storage Benchmark
//! Created: 2025-01-22
//! Author: isdood

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lazuline::memory::bio::neural_storage::{StorageNetwork, LearningSystem, PatternRecognizer};

pub fn bench_storage_network(c: &mut Criterion) {
    let mut network = StorageNetwork::new();
    c.bench_function("storage_network_add_node", |b| {
        b.iter(|| {
            for i in 0..100 {
                network.add_node(format!("node{}", i));
            }
        })
    });
}

pub fn bench_learning_system(c: &mut Criterion) {
    let mut learning = LearningSystem::new();
    c.bench_function("learning_system_learn", |b| {
        b.iter(|| {
            for i in 0..100 {
                learning.learn(format!("data{}", i));
            }
        })
    });
}

pub fn bench_pattern_recognizer(c: &mut Criterion) {
    let mut recognizer = PatternRecognizer::new();
    c.bench_function("pattern_recognizer_recognize", |b| {
        b.iter(|| {
            for i in 0..100 {
                recognizer.recognize(format!("pattern{}", i));
            }
        })
    });
}

criterion_group!(benches, bench_storage_network, bench_learning_system, bench_pattern_recognizer);
criterion_main!(benches);
