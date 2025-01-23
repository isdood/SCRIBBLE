const std = @import("std");

pub fn abs(x: anytype) @TypeOf(x) {
    if (x < 0) return -x;
    return x;
}

pub fn sin(x: f64) f64 {
    return std.math.sin(x);
}

pub fn asin(x: f64) f64 {
    return std.math.asin(x);
}

pub const Crystal = struct {
    pub fn calculateDSpacing(h: i32, k: i32, l: i32, a: f64) f64 {
        const d = @sqrt(
            @as(f64, @floatFromInt(h * h + k * k + l * l))
        );
        return a / d;
    }

    pub fn calculateBraggAngle(d_spacing: f64, wavelength: f64) f64 {
        return asin(wavelength / (2.0 * d_spacing));
    }

    pub fn calculateStructureFactor(h: i32, k: i32, l: i32) f64 {
        return @as(f64, @floatFromInt(h + k + l)) * 0.5;
    }
};
