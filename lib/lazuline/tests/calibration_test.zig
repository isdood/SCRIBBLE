const std = @import("std");
const testing = std.testing;
const hardware = @import("hardware");

test "temperature calibration curve" {
    var arena = std.heap.ArenaAllocator.init(std.testing.allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    var curve = hardware.calibration.CalibrationCurve.init(allocator);
    defer curve.deinit();

    // Add calibration points
    try curve.addPoint(0.0, -0.5);    // Sensor reads -0.5°C at 0°C
    try curve.addPoint(100.0, 99.0);  // Sensor reads 99.0°C at 100°C

    // Test calibration
    try testing.expectApprox(0.0, curve.calibrate(-0.5));
    try testing.expectApprox(100.0, curve.calibrate(99.0));
    try testing.expectApprox(50.0, curve.calibrate(49.25));
}

test "temperature sensor with calibration" {
    var arena = std.heap.ArenaAllocator.init(std.testing.allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    var sensor = try hardware.sensors.TemperatureSensor.init(allocator, .{
        .sensor_type = .Simulated,
        .update_interval = 100_000_000, // 100ms
    });
    defer sensor.deinit();

    // Add calibration points
    try sensor.addCalibrationPoint(25.0);  // Current simulated temp is ~25°C
    try sensor.addCalibrationPoint(30.0);  // Wait and add another point

    const info = sensor.getCalibrationInfo();
    try testing.expectEqual(@as(usize, 2), info.point_count);
    try testing.expect(info.max_error < 1.0);
}
