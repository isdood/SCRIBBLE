const std = @import("std");
const constants = @import("constants.zig");
const harmonic = @import("harmonic.zig");

pub const version = "0.1.0";

const VectorType = @Vector(8, f64);

// ... (previous WavePattern and QuantumResonance remain the same)

pub const CrystalLattice = struct {
    data: []align(64) f64,
    size: usize,
    buffer_index: ?usize,
    dimensions: [3]usize,
    allocator: std.mem.Allocator,
    harmonic_state: harmonic.HarmonicState,

    pub const CACHE_LINE_SIZE: usize = 64;
    pub const VECTOR_SIZE: usize = 8;
    pub const MAX_SIZE: usize = 65536;
    pub const CRYSTAL_ALIGNMENT: usize = 64;

    const Buffer = struct {
        data: []align(64) f64,
        index: usize,
    };

    var buffer_pool = [_]?[]align(64) f64{null} ** 16;

    fn acquireBuffer(allocator: std.mem.Allocator, size: usize) ?Buffer {
        for (buffer_pool, 0..) |*slot, i| {
            if (slot.* == null) {
                const data = allocator.alignedAlloc(f64, 64, size) catch return null;
                slot.* = data;
                return Buffer{ .data = data, .index = i };
            }
        }
        return null;
    }

    fn releaseBuffer(allocator: std.mem.Allocator, index: usize) void {
        if (index < buffer_pool.len) {
            if (buffer_pool[index]) |data| {
                allocator.free(data);
            }
            buffer_pool[index] = null;
        }
    }

    pub fn init(allocator: std.mem.Allocator, dimensions: [3]usize) !*CrystalLattice {
        const size = dimensions[0] * dimensions[1] * dimensions[2];
        if (size > MAX_SIZE) return error.SizeTooLarge;

        const self = try allocator.create(CrystalLattice);
        errdefer allocator.destroy(self);

        if (acquireBuffer(allocator, size)) |buffer| {
            self.* = .{
                .dimensions = dimensions,
                .data = buffer.data,
                .allocator = allocator,
                .size = size,
                .buffer_index = buffer.index,
                .harmonic_state = harmonic.HarmonicState.new(),
            };
            return self;
        }

        const data = try allocator.alignedAlloc(f64, CRYSTAL_ALIGNMENT, size);
        errdefer allocator.free(data);

        self.* = .{
            .dimensions = dimensions,
            .data = data,
            .allocator = allocator,
            .size = size,
            .buffer_index = null,
            .harmonic_state = harmonic.HarmonicState.new(),
        };

        return self;
    }

    pub fn check_stability(self: *const CrystalLattice) bool {
        return self.harmonic_state.get_coherence() >= constants.QUANTUM_STABILITY_THRESHOLD;
    }

    pub inline fn batchSet(self: *CrystalLattice, value: f64) void {
        const value_vector: VectorType = @splat(value);

        if (self.size <= VECTOR_SIZE) {
            @memset(self.data, value);
            return;
        }

        var i: usize = 0;
        while (i < self.size - VECTOR_SIZE) : (i += VECTOR_SIZE) {
            const ptr_aligned: *align(64) f64 = @alignCast(&self.data[i]);
            const vec_ptr: *VectorType = @ptrCast(ptr_aligned);
            vec_ptr.* = value_vector;
        }

        while (i < self.size) : (i += 1) {
            self.data[i] = value;
        }

        self.harmonic_state.apply_field(value);
    }

    pub fn deinit(self: *CrystalLattice) void {
        if (self.buffer_index) |index| {
            releaseBuffer(self.allocator, index);
        } else {
            self.allocator.free(self.data);
        }
        self.allocator.destroy(self);
    }
};
