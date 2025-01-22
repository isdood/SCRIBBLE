const std = @import("std");
const testing = std.testing;
const math = std.math;

pub const Vector3D = struct {
    x: f64,
    y: f64,
    z: f64,
    quantum_coherence: f64,

    pub fn init(x: f64, y: f64, z: f64) Vector3D {
        return Vector3D{
            .x = x,
            .y = y,
            .z = z,
            .quantum_coherence = 1.0,
        };
    }

    pub fn dot(self: Vector3D, other: Vector3D) f64 {
        const classical_dot = self.x * other.x + self.y * other.y + self.z * other.z;
        const coherence = @min(self.quantum_coherence, other.quantum_coherence);
        return classical_dot * coherence;
    }

    pub fn magnitude(self: Vector3D) f64 {
        return @sqrt(self.x * self.x + self.y * self.y + self.z * self.z);
    }
};

test "vector creation" {
    const v = Vector3D.init(1.0, 2.0, 3.0);
    try testing.expectEqual(v.x, 1.0);
    try testing.expectEqual(v.y, 2.0);
    try testing.expectEqual(v.z, 3.0);
}

test "dot product" {
    const v1 = Vector3D.init(1.0, 2.0, 3.0);
    const v2 = Vector3D.init(4.0, 5.0, 6.0);
    try testing.expectEqual(v1.dot(v2), 32.0);
}
