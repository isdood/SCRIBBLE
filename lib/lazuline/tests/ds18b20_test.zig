const std = @import("std");
const testing = std.testing;
const hardware = @import("hardware");

test "DS18B20 sensor resolution configuration" {
    if (!@import("builtin").is_test) return error.SkipZigTest;

    var arena = std.heap.ArenaAllocator.init(std.testing.allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    var sensor = try hardware.sensors.DS18B20.init(allocator, .{
        .device_id = "28-xxxxxxxxxxxx",
    });
    defer sensor.deinit();

    try sensor.setResolution(.Bits12);
    try testing.expectEqual(sensor.config.conversion_delay, 750_000_000);

    try sensor.setResolution(.Bits9);
    try testing.expectEqual(sensor.config.conversion_delay, 94_000_000);
}

test "DS18B20 CRC calculation" {
    const test_data = "28 01 4B 46 7F FF 0C 10 5C : crc=5C YES\n28 01 4B 46 7F FF 0C 10 5C t=23125";
    try testing.expect(hardware.onewire.OneWireBus.checkCrc(test_data));
}

test "DS18B20 temperature reading simulation" {
    if (!@import("builtin").is_test) return error.SkipZigTest;

    var arena = std.heap.ArenaAllocator.init(std.testing.allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    var sensor = try hardware.sensors.TemperatureSensor.init(allocator, .{
        .sensor_type = .DS18B20,
        .onewire_id = "28-xxxxxxxxxxxx",
        .update_interval = 100_000_000, // 100ms
    });
    defer sensor.deinit();

    const temp = try sensor.readTemperature();
    try testing.expect(temp >= -55.0 and temp <= 125.0); // DS18B20 range
}
