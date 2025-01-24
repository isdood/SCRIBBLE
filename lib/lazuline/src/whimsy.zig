const std = @import("std");

pub const Whimsy = struct {
    level: u8,

    pub fn init() Whimsy {
        return Whimsy{ .level = 1 };
    }
};
