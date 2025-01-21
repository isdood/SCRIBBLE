//! Shattered Cache Implementation
//! Created: 2025-01-21 04:18:35 UTC
//! Author: isdood
//! 
//! A quantum-coherent predictive caching system for the Scribble framework.

pub const cache = @import("cache.zig");
pub const shard = @import("shard.zig");
pub const resonator = @import("resonator.zig");
pub const patterns = @import("patterns.zig");
pub const quantum = @import("quantum.zig");

pub const ShatteredCache = cache.ShatteredCache;
pub const Shard = shard.Shard;
pub const Resonator = resonator.Resonator;
pub const AccessPattern = patterns.AccessPattern;
pub const QuantumState = quantum.QuantumState;

test {
    _ = @import("cache.zig");
    _ = @import("shard.zig");
    _ = @import("resonator.zig");
    _ = @import("patterns.zig");
    _ = @import("quantum.zig");
}
