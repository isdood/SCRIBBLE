const std = @import("std");
const testing = std.testing;
const hardware = @import("hardware");
const fs = std.fs;

test "calibration storage save and load" {
    var arena = std.heap.ArenaAllocator.init(std.testing.allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    // Create a temporary file
    const temp_path = "test_calibration.dat";
    defer fs.cwd().deleteFile(temp_path) catch {};

    // Create and save calibration data
    var curve1 = try hardware.calibration.CalibrationCurve.initWithStorage(allocator, temp_path);
    defer curve1.deinit();

    try curve1.addPoint(0.0, -0.5);
    try curve1.addPoint(100.0, 99.0);

    // Load calibration data in a new curve
    var curve2 = try hardware.calibration.CalibrationCurve.initWithStorage(allocator, temp_path);
    defer curve2.deinit();

    try testing.expectEqual(@as(usize, 2), curve2.points.items.len);
    try testing.expectApprox(0.0, curve2.calibrate(-0.5));
    try testing.expectApprox(100.0, curve2.calibrate(99.0));
}

test "calibration storage error handling" {
    var arena = std.heap.ArenaAllocator.init(std.testing.allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    const invalid_path = "/nonexistent/path/calibration.dat";
    var storage = try hardware.calibration.persistence.CalibrationStorage.init(allocator, invalid_path);
    defer storage.deinit();

    try testing.expectError(error.FileNotFound, storage.load());
}
