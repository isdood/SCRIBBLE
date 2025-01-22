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
