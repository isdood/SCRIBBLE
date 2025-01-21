//! Access Pattern Implementations
//! Last Updated: 2025-01-21 04:18:35 UTC
//! Author: isdood

const std = @import("std");

/// Memory access patterns for optimization
pub const AccessPattern = enum {
    /// Sequential access (e.g., array traversal)
    Sequential,
    /// Strided access (e.g., matrix operations)
    Strided,
    /// Random access
    Random,
    /// Clustered access (e.g., spatial locality)
    Clustered,
    /// Hybrid patterns
    Hybrid,

    /// Get the base resonance frequency for this pattern
    pub fn baseResonance(self: AccessPattern) f64 {
        return switch (self) {
            .Sequential => 1.0,
            .Strided => 0.9,
            .Random => 0.7,
            .Clustered => 0.85,
            .Hybrid => 0.8,
        };
    }

    /// Get the coherence decay rate for this pattern
    pub fn coherenceDecayRate(self: AccessPattern) f64 {
        return switch (self) {
            .Sequential => 0.01,
            .Strided => 0.02,
            .Random => 0.05,
            .Clustered => 0.03,
            .Hybrid => 0.04,
        };
    }
};
