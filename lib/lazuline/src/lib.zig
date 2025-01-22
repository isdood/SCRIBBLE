//! Crystal Wave Runtime Library
//! Created: 2025-01-22 01:33:22 UTC
//! Author: isdood

pub const lattice = @import("lattice/core/mod.zig");
pub const harmony = @import("harmony/io/mod.zig");
pub const resonance = @import("resonance/scheduler/mod.zig");
pub const wave = @import("wave/core/mod.zig");
pub const runtime = @import("wave_runtime.zig");

test {
    _ = @import("std").testing.refAllDecls(@This());
}

// Harmonic channels and timers
pub const crystal = struct {
    pub const channels = @import("crystal/channels/mod.zig");
    pub const timers = @import("crystal/timers/mod.zig");
};

pub const harmonic = struct {
    pub const sync = @import("harmonic/sync/mod.zig");
    pub const async_primitives = @import("harmonic/async/mod.zig");
};
