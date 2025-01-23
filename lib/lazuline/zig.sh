#!/bin/bash

echo "[INFO] Starting Lazuline build configuration fix..."
echo "[INFO] Current time: 2025-01-23 00:13:22 UTC"
echo "[INFO] User: isdood"
echo "[INFO] Zig version: 0.13.0"

# Update lib.zig with fixed CacheConfig
cat > src/lib.zig << 'EOF'
const std = @import("std");
const builtin = @import("builtin");

pub const version = "0.1.0";

pub const WavePattern = struct {
    amplitude: f64,
    frequency: f64,
    phase: f64,
    omega: f64,

    pub fn new(amplitude: f64, frequency: f64, phase: f64) WavePattern {
        return WavePattern{
            .amplitude = amplitude,
            .frequency = frequency,
            .phase = phase,
            .omega = 2.0 * std.math.pi * frequency,
        };
    }

    pub fn compute(self: WavePattern, time: f64) f64 {
        return self.amplitude * @sin(self.omega * time + self.phase);
    }
};

pub const CrystalLattice = struct {
    dimensions: [3]usize,
    data: []align(32) f64,
    allocator: std.mem.Allocator,
    size: usize,

    // Fixed cache line size to 64 bytes for modern CPUs
    const CACHE_LINE_SIZE: usize = 64;
    const VECTOR_SIZE: usize = 4;

    pub fn init(allocator: std.mem.Allocator, dimensions: [3]usize) !*CrystalLattice {
        const self = try allocator.create(CrystalLattice);
        const size = dimensions[0] * dimensions[1] * dimensions[2];
        const data = try allocator.alignedAlloc(f64, CACHE_LINE_SIZE, size);

        self.* = .{
            .dimensions = dimensions,
            .data = data,
            .allocator = allocator,
            .size = size,
        };

        @memset(data, 0);
        return self;
    }

    pub fn deinit(self: *CrystalLattice) void {
        self.allocator.free(self.data);
        self.allocator.destroy(self);
    }

    pub fn batchSet(self: *CrystalLattice, value: f64) void {
        const aligned_size = self.size & ~@as(usize, VECTOR_SIZE - 1);

        var i: usize = 0;
        // Process 4 elements at a time
        while (i < aligned_size) : (i += VECTOR_SIZE) {
            self.data[i] = value;
            self.data[i + 1] = value;
            self.data[i + 2] = value;
            self.data[i + 3] = value;
        }
        // Handle remaining elements
        while (i < self.size) : (i += 1) {
            self.data[i] = value;
        }
    }

    pub fn clone(self: *const CrystalLattice) !*CrystalLattice {
        const new_lattice = try self.allocator.create(CrystalLattice);
        const new_data = try self.allocator.alignedAlloc(f64, CACHE_LINE_SIZE, self.size);
        @memcpy(new_data, self.data);

        new_lattice.* = .{
            .dimensions = self.dimensions,
            .data = new_data,
            .allocator = self.allocator,
            .size = self.size,
        };

        return new_lattice;
    }
};

pub const QuantumResonance = struct {
    frequency_inv: f64,
    coherence: f64,

    pub fn new(frequency: f64, coherence: f64) QuantumResonance {
        return QuantumResonance{
            .frequency_inv = 1.0 / frequency,
            .coherence = coherence,
        };
    }

    pub fn calculate(self: QuantumResonance, time: f64) f64 {
        return self.coherence * @exp(-time * self.frequency_inv);
    }
};
EOF

# Update bench/main.zig with fixed cache line size reference
cat > bench/main.zig << 'EOF'
const std = @import("std");
const lazuline = @import("lazuline");
const time = std.time;
const print = std.debug.print;

const BenchmarkConfig = struct {
    iterations: usize,
    warmup_iterations: usize,
    thread_count: usize,

    pub fn default() BenchmarkConfig {
        return .{
            .iterations = 1_000_000,
            .warmup_iterations = 10_000,
            .thread_count = 4,
        };
    }
};

fn runWavePatternBenchmark(config: BenchmarkConfig) !void {
    print("\nWave Pattern Benchmark:\n", .{});
    var prng = std.rand.DefaultPrng.init(0);
    const random = prng.random();

    var timer = try time.Timer.start();
    const warmup_start = timer.lap();

    var i: usize = 0;
    var warmup_acc: f64 = 0;
    while (i < config.warmup_iterations) : (i += 1) {
        const amp = random.float(f64) * 10.0;
        const freq = random.float(f64) * 1000.0;
        const phase = random.float(f64) * std.math.pi * 2.0;
        const wave = lazuline.WavePattern.new(amp, freq, phase);
        warmup_acc += wave.compute(1.0);
    }

    const warmup_end = timer.lap();
    const warmup_ns = @as(f64, @floatFromInt(warmup_end - warmup_start)) / @as(f64, @floatFromInt(config.warmup_iterations));

    const start = timer.lap();

    i = 0;
    var acc: f64 = 0;
    while (i < config.iterations) : (i += 1) {
        const amp = random.float(f64) * 10.0;
        const freq = random.float(f64) * 1000.0;
        const phase = random.float(f64) * std.math.pi * 2.0;
        const wave = lazuline.WavePattern.new(amp, freq, phase);
        acc += wave.compute(1.0);
    }

    const end = timer.lap();
    const elapsed_ns = end - start;
    const ns_per_op = @as(f64, @floatFromInt(elapsed_ns)) / @as(f64, @floatFromInt(config.iterations));

    print("  Creation + Computation: {d:.2} ns/op\n", .{ns_per_op});
    print("  Warmup: {d:.2} ns/op\n", .{warmup_ns});
    print("  Throughput: {d:.2} MOps/s\n", .{1000.0 / ns_per_op});
    print("  (Control sum: {d})\n", .{acc});
}

fn runCrystalLatticeBenchmark(config: BenchmarkConfig, allocator: std.mem.Allocator) !void {
    print("\nCrystal Lattice Benchmark:\n", .{});
    const dims = [3]usize{ 16, 16, 16 };

    const template_lattice = try lazuline.CrystalLattice.init(allocator, dims);
    defer template_lattice.deinit();

    var timer = try time.Timer.start();
    const warmup_start = timer.lap();

    var i: usize = 0;
    while (i < config.warmup_iterations) : (i += 1) {
        const lattice = try template_lattice.clone();
        lattice.batchSet(@as(f64, @floatFromInt(i)));
        lattice.deinit();
    }

    const warmup_end = timer.lap();
    const warmup_ns = @as(f64, @floatFromInt(warmup_end - warmup_start)) / @as(f64, @floatFromInt(config.warmup_iterations));

    const start = timer.lap();

    i = 0;
    var acc: f64 = 0;
    while (i < config.iterations) : (i += 1) {
        const lattice = try template_lattice.clone();
        lattice.batchSet(@as(f64, @floatFromInt(i)));
        acc += lattice.data[0];
        lattice.deinit();
    }

    const end = timer.lap();
    const elapsed_ns = end - start;
    const ns_per_op = @as(f64, @floatFromInt(elapsed_ns)) / @as(f64, @floatFromInt(config.iterations));
    const memory_size = dims[0] * dims[1] * dims[2] * @sizeOf(f64);

    print("  Allocation + Init: {d:.2} ns/op\n", .{ns_per_op});
    print("  Warmup: {d:.2} ns/op\n", .{warmup_ns});
    print("  Throughput: {d:.2} MOps/s\n", .{1000.0 / ns_per_op});
    print("  Memory: {d} bytes per lattice\n", .{memory_size});
    print("  Cache line size: {d} bytes\n", .{lazuline.CrystalLattice.CACHE_LINE_SIZE});
    print("  (Control sum: {d})\n", .{acc});
}

fn runQuantumResonanceBenchmark(config: BenchmarkConfig) !void {
    print("\nQuantum Resonance Benchmark:\n", .{});
    var prng = std.rand.DefaultPrng.init(0);
    const random = prng.random();

    var timer = try time.Timer.start();
    const warmup_start = timer.lap();

    var i: usize = 0;
    var warmup_acc: f64 = 0;
    while (i < config.warmup_iterations) : (i += 1) {
        const freq = random.float(f64) * 1000.0;
        const coherence = random.float(f64);
        const resonance = lazuline.QuantumResonance.new(freq, coherence);
        warmup_acc += resonance.calculate(1.0);
    }

    const warmup_end = timer.lap();
    const warmup_ns = @as(f64, @floatFromInt(warmup_end - warmup_start)) / @as(f64, @floatFromInt(config.warmup_iterations));

    const start = timer.lap();

    i = 0;
    var acc: f64 = 0;
    while (i < config.iterations) : (i += 1) {
        const freq = random.float(f64) * 1000.0;
        const coherence = random.float(f64);
        const resonance = lazuline.QuantumResonance.new(freq, coherence);
        acc += resonance.calculate(1.0);
    }

    const end = timer.lap();
    const elapsed_ns = end - start;
    const ns_per_op = @as(f64, @floatFromInt(elapsed_ns)) / @as(f64, @floatFromInt(config.iterations));

    print("  Resonance Calculation: {d:.2} ns/op\n", .{ns_per_op});
    print("  Warmup: {d:.2} ns/op\n", .{warmup_ns});
    print("  Throughput: {d:.2} MOps/s\n", .{1000.0 / ns_per_op});
    print("  (Control sum: {d})\n", .{acc});
}

pub fn main() !void {
    const config = BenchmarkConfig.default();

    print("\n=== Lazuline Benchmark Suite ===\n", .{});
    print("Date: 2025-01-23 00:13:22 UTC\n", .{});
    print("Configuration:\n", .{});
    print("  Iterations: {d}\n", .{config.iterations});
    print("  Warmup iterations: {d}\n", .{config.warmup_iterations});
    print("  Thread count: {d}\n\n", .{config.thread_count});

    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    try runWavePatternBenchmark(config);
    try runCrystalLatticeBenchmark(config, allocator);
    try runQuantumResonanceBenchmark(config);
}
EOF

# Clean any existing build artifacts
echo "[INFO] Cleaning existing build artifacts..."
rm -rf zig-cache
rm -rf zig-out

echo "[SUCCESS] Build configuration has been updated"
echo "[INFO] Try running 'zig build bench' now"
echo ""
echo "Performance Optimizations:"
echo "1. Fixed cache line size constants"
echo "2. Simplified SIMD vector operations"
echo "3. Improved memory alignment"
echo "4. Enhanced benchmark metrics"
echo "5. Updated timestamp"
