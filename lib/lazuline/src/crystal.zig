const std = @import("std");

pub const Crystal = struct {
    value: f64,

    pub fn init() Crystal {
        return Crystal{ .value = 1.0 };
    }
};
