const std = @import("std");

pub const PerformanceThresholds = struct {
    // Core operation thresholds (in nanoseconds)
    pub const MAX_CORE_OP_TIME: u64 = 50;  // Max 50ns per operation
    pub const TARGET_CORE_OP_TIME: u64 = 20;  // Target 20ns per operation
    pub const MIN_CORE_OPS_PER_SEC: u64 = 1_000_000;  // Min 1M ops/sec

    // Memory usage thresholds (in bytes)
    pub const MAX_MEMORY_PER_OP: usize = 16;  // Max 16 bytes per operation
    pub const TARGET_MEMORY_PER_OP: usize = 8;  // Target 8 bytes per operation
    pub const MAX_TOTAL_MEMORY: usize = 1024 * 1024;  // Max 1MB total

    // Concurrent operation thresholds
    pub const MAX_CONCURRENT_OP_TIME: u64 = 40;  // Max 40ns per operation
    pub const TARGET_CONCURRENT_OP_TIME: u64 = 25;  // Target 25ns per operation
    pub const MIN_CONCURRENT_OPS_PER_SEC: u64 = 4_000_000;  // Min 4M ops/sec
};
