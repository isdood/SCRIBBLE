// lib/ziggy/src/vector3d.zig
const std = @import("std");

pub const Vector3D = struct {
    x: f64,
    y: f64,
    z: f64,

    const Self = @This();

    // Basic constructor
    pub fn init(x: f64, y: f64, z: f64) Self {
        return Self{
            .x = x,
            .y = y,
            .z = z,
        };
    }

    // Dot product - commonly used in quantum calculations
    pub fn dot(self: Self, other: Self) f64 {
        return self.x * other.x +
        self.y * other.y +
        self.z * other.z;
    }

    // Magnitude calculation
    pub fn magnitude(self: Self) f64 {
        return @sqrt(self.dot(self));
    }
};

// Export these functions with C ABI for Rust FFI
export fn vector3d_dot(v1: Vector3D, v2: Vector3D) f64 {
    return v1.dot(v2);
}

export fn vector3d_magnitude(v: Vector3D) f64 {
    return v.magnitude();
}
