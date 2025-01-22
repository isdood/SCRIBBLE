const std = @import("std");
const testing = std.testing;
const hardware = @import("hardware");

test "TMP102 sensor configuration" {
    if (!@import("builtin").is_test) return error.SkipZigTest;

    const config = hardware.sensors.TMP102.Config{
        .bus_number = 1,
        .address = 0x48,
        .conversion_rate = .Four_Hz,
    };

    var sensor = try hardware.sensors.TMP102.init(config);
    defer sensor.deinit();

    try sensor.configure(.Four_Hz);
}

test "TMP102 temperature reading simulation" {
    if (!@import("builtin").is_test) return error.SkipZigTest;

    var arena = std.heap.ArenaAllocator.init(std.testing.allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    var sensor = try hardware.sensors.TemperatureSensor.init(allocator, .{
        .sensor_type = .TMP102,
        .i2c_bus = 1,
        .i2c_address = 0x48,
        .update_interval = 100_000_000, // 100ms
    });
    defer sensor.deinit();

    const temp = try sensor.readTemperature();
    try testing.expect(temp >= -55.0 and temp <= 150.0); // TMP102 range
}
