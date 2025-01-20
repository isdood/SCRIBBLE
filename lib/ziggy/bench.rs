//! Ziggy Benchmark Configuration
//! ===========================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-20
//! Last Updated: 2025-01-20 17:08:00 UTC
//! Version: 0.1.0
//! License: MIT

#![feature(test)]

extern crate test;

use test::Bencher;
use ziggy::Vector3D;

#[bench]
fn bench_dot_product(b: &mut Bencher) {
    let v1 = Vector3D::new(1.0, 2.0, 3.0);
    let v2 = Vector3D::new(4.0, 5.0, 6.0);
    b.iter(|| v1.dot(&v2));
}

#[bench]
fn bench_magnitude(b: &mut Bencher) {
    let v = Vector3D::new(3.0, 4.0, 0.0);
    b.iter(|| v.magnitude());
}
