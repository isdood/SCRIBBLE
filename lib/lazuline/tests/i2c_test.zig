const std = @import("std");
const testing = std.testing;
const hardware = @import("hardware");

test "I2C bus device addressing" {
    if (!@import("builtin").is_test) return error.SkipZigTest;

    const bus = try hardware.i2c.I2CBus.init(1);
    defer bus.deinit();

    try testing.expect(bus.getDeviceAddr() == null);

    // These operations should work in test mode without actual hardware
    try bus.setDeviceAddr(0x48);
    try testing.expect(bus.getDeviceAddr().? == 0x48);

    try bus.setDeviceAddr(0x49);
    try testing.expect(bus.getDeviceAddr().? == 0x49);
}

test "I2C bus error handling" {
    if (!@import("builtin").is_test) return error.SkipZigTest;

    const bus = try hardware.i2c.I2CBus.init(1);
    defer bus.deinit();

    // Try to write to device before setting address
    try testing.expectError(
        hardware.i2c.I2CBus.Error.AddressError,
        bus.writeReg(0x48, 0x00, 0x00)
    );
}
