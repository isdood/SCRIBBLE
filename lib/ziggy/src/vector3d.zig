const std = @import("std");
const testing = std.testing;

// Mark the struct as extern and packed for C ABI compatibility
pub const Vector3D = extern struct {
    x: f64,
    y: f64,
    z: f64,

    const Self = @This();

    pub fn init(x: f64, y: f64, z: f64) Self {
        return Self{
            .x = x,
            .y = y,
            .z = z,
        };
    }

    pub fn dot(self: Self, other: Self) f64 {
        return self.x * other.x +
        self.y * other.y +
        self.z * other.z;
    }

    pub fn magnitude(self: Self) f64 {
        return @sqrt(self.dot(self));
    }
};

// Export these functions with explicit parameter types for C ABI
export fn vector3d_dot(v1: extern struct { x: f64, y: f64, z: f64 },
                       v2: extern struct { x: f64, y: f64, z: f64 }) f64 {
                           const vec1 = Vector3D{ .x = v1.x, .y = v1.y, .z = v1.z };
                           const vec2 = Vector3D{ .x = v2.x, .y = v2.y, .z = v2.z };
                           return vec1.dot(vec2);
                       }

                       export fn vector3d_magnitude(v: extern struct { x: f64, y: f64, z: f64 }) f64 {
                           const vec = Vector3D{ .x = v.x, .y = v.y, .z = v.z };
                           return vec.magnitude();
                       }

                       // Tests
                       test "vector3d basic operations" {
                           const v1 = Vector3D.init(1.0, 2.0, 3.0);
                           const v2 = Vector3D.init(4.0, 5.0, 6.0);

                           try testing.expectApproxEqAbs(v1.dot(v2), 32.0, 0.0001);
                           try testing.expectApproxEqAbs(v1.magnitude(), 3.7416573867739413, 0.0001);
                       }

                       test "C ABI functions" {
                           const v1 = .{ .x = 1.0, .y = 2.0, .z = 3.0 };
                           const v2 = .{ .x = 4.0, .y = 5.0, .z = 6.0 };

                           try testing.expectApproxEqAbs(vector3d_dot(v1, v2), 32.0, 0.0001);
                           try testing.expectApproxEqAbs(vector3d_magnitude(v1), 3.7416573867739413, 0.0001);
                       }
