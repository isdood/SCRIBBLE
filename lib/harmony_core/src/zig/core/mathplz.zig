pub const std = @import("std");

pub fn abs(x: f64) f64 {
    return if (x < 0) -x else x;
}

pub fn log2(x: f64) f64 {
    return @log2(x);
}
