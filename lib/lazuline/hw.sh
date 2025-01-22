cat > fix_i2c_bus.sh << 'EOF'
#!/bin/bash

# Timestamp: 2025-01-22 02:14:48
# Author: isdood

echo "[FIX] Updating I2C bus interface with proper device addressing..."

cat > src/hardware/i2c/bus.zig << 'EOI'
const std = @import("std");
const os = std.os;
const fs = std.fs;

pub const I2CBus = struct {
    const Self = @This();

    pub const Error = error{
        BusError,
        DeviceNotFound,
        AccessDenied,
        Timeout,
        AddressError,
    };

    fd: fs.File,
    bus_number: u8,
    current_addr: ?u8,

    pub fn init(bus_number: u8) !Self {
        const path = try std.fmt.allocPrint(
            std.heap.page_allocator,
            "/dev/i2c-{d}",
            .{bus_number}
        );
        defer std.heap.page_allocator.free(path);

        const file = try fs.cwd().openFile(path, .{ .mode = .read_write });

        return Self{
            .fd = file,
            .bus_number = bus_number,
            .current_addr = null,
        };
    }

    pub fn deinit(self: *Self) void {
        self.fd.close();
    }

    fn setDeviceAddr(self: *Self, device_addr: u8) !void {
        if (self.current_addr != device_addr) {
            // I2C_SLAVE constant from Linux i2c-dev.h
            const I2C_SLAVE: u64 = 0x0703;
            const err = os.linux.ioctl(self.fd.handle, I2C_SLAVE, @as(u64, device_addr));
            if (err != 0) return Error.AddressError;
            self.current_addr = device_addr;
        }
    }

    pub fn writeReg(self: *Self, device_addr: u8, reg: u8, value: u8) !void {
        try self.setDeviceAddr(device_addr);
        const bytes = [_]u8{ reg, value };
        const written = try self.fd.write(&bytes);
        if (written != 2) return Error.BusError;
    }

    pub fn readReg(self: *Self, device_addr: u8, reg: u8) !u8 {
        try self.setDeviceAddr(device_addr);
        _ = try self.fd.write(&[_]u8{reg});
        var buf: [1]u8 = undefined;
        const read = try self.fd.read(&buf);
        if (read != 1) return Error.BusError;
        return buf[0];
    }

    pub fn read16(self: *Self, device_addr: u8, reg: u8) !u16 {
        try self.setDeviceAddr(device_addr);
        _ = try self.fd.write(&[_]u8{reg});
        var buf: [2]u8 = undefined;
        const read = try self.fd.read(&buf);
        if (read != 2) return Error.BusError;
        return @as(u16, buf[0]) << 8 | buf[1];
    }

    pub fn getDeviceAddr(self: Self) ?u8 {
        return self.current_addr;
    }
};
EOI

# Add test for I2C bus operations
cat > tests/i2c_test.zig << 'EOT'
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
EOT

echo "[FIX] Added proper I2C device addressing"
echo "[FIX] Added error checking for I/O operations"
echo "[FIX] Added I2C bus tests"
echo "[FIX] Updated timestamp to: 2025-01-22 02:14:48"
echo "[INFO] Try running 'zig build test' again"

EOF

chmod +x fix_i2c_bus.sh

echo "[DONE] Created I2C bus fix script"
echo "[INFO] Run './fix_i2c_bus.sh' to apply the fixes"
