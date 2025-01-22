pub const sensors = struct {
    pub const TemperatureSensor = @import("sensors/temperature.zig").TemperatureSensor;
    pub const TMP102 = @import("sensors/tmp102.zig").TMP102;
    pub const DS18B20 = @import("sensors/ds18b20.zig").DS18B20;
};
pub const crystal = @import("crystal/frequency.zig");
pub const i2c = @import("i2c/bus.zig");
pub const onewire = @import("onewire/bus.zig");
pub const calibration = @import("calibration/temperature.zig");
