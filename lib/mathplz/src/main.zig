const std = @import("std");
pub const bio = @import("bio");
pub const crystal = @import("crystal");
pub const quantum = @import("quantum");

pub const MathConfig = struct {
    crystal_coherence: f64,
    quantum_stability: f64,
    bio_precision: u8,
};

pub const Engine = struct {
    config: MathConfig,

    pub fn init(config: MathConfig) Engine {
        return .{
            .config = config,
        };
    }
};

test "basic config" {
    const config = MathConfig{
        .crystal_coherence = 0.93,
        .quantum_stability = 0.87,
        .bio_precision = 4,
    };
    const engine = Engine.init(config);
    try std.testing.expect(engine.config.crystal_coherence >= 0.93);
}
