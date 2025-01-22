const std = @import("std");
const lazuline = @import("lazuline");

fn benchmarkChannelThroughput(
    allocator: std.mem.Allocator,
    message_size: usize,
    iterations: usize
) !struct { throughput: f64, latency: f64 } {
    var channel = lazuline.crystal.channels.CrystalChannel.init(allocator, .{
        .buffer_size = 1024,
        .resonance_frequency = 440.0,
        .damping_factor = 0.01,
    });
    defer channel.deinit();

    var message = try allocator.alloc(u8, message_size);
    defer allocator.free(message);
    @memset(message, 'A');

    var timer = try std.time.Timer.start();
    const start = timer.lap();

    // Measure send throughput
    for (0..iterations) |_| {
        try channel.send(message);
    }

    // Measure receive throughput
    for (0..iterations) |_| {
        const received = try channel.receive();
        defer allocator.free(received);
    }

    const elapsed = timer.lap() - start;
    const seconds = @as(f64, @floatFromInt(elapsed)) / 1_000_000_000.0;
    const bytes_transferred = @as(f64, @floatFromInt(message_size * iterations * 2));
    const throughput = bytes_transferred / seconds;
    const latency = seconds / @as(f64, @floatFromInt(iterations * 2));

    return .{
        .throughput = throughput,
        .latency = latency,
    };
}

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    const sizes = [_]usize{ 64, 256, 1024, 4096 };
    const iterations: usize = 10_000;

    std.debug.print("\nCrystal Channel Benchmarks\n", .{});
    std.debug.print("=======================\n\n", .{});

    for (sizes) |size| {
        const result = try benchmarkChannelThroughput(allocator, size, iterations);

        std.debug.print("Message Size: {d} bytes\n", .{size});
        std.debug.print("  Throughput: {d:.2} MB/s\n", .{result.throughput / 1_000_000.0});
        std.debug.print("  Latency: {d:.3} µs\n\n", .{result.latency * 1_000_000.0});
    }
}
