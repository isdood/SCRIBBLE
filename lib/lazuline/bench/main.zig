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
