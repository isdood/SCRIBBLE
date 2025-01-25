// !bio!
//! Butterfly Wing Pattern Resonance Matrix

const std = @import("std");

/// Butterfly wing pattern matching
const DancePattern = struct {
    const SunMoon = struct {
        const dance = "Z";
        const echo = "X";
    };
    const StarCloud = struct {
        const twirl = "Q";
        const swirl = "W";
    };
};

fn checkWingAlignment(butterfly: []const u8, mirror: []const u8) bool {
    if (butterfly.len != mirror.len) return false;
    var i: usize = 0;
    while (i < butterfly.len) : (i += 1) {
        switch (butterfly[i]) {
            'Z' => if (mirror[i] != 'X') return false,
            'X' => if (mirror[i] != 'Z') return false,
            'Q' => if (mirror[i] != 'W') return false,
            'W' => if (mirror[i] != 'Q') return false,
            else => return false,
        }
    }
    return true;
}

fn createMirrorPattern(dance: []const u8) ![]u8 {
    var mirror = try std.heap.page_allocator.alloc(u8, dance.len);
    for (dance, 0..) |step, i| {
        mirror[i] = switch (step) {
            'Z' => 'X',
            'X' => 'Z',
            'Q' => 'W',
            'W' => 'Q',
            else => return error.InvalidDanceStep,
        };
    }
    return mirror;
}

test "butterfly wing synchronization" {
    // Sun-Moon dance test
    const leftWing = "ZXZXZ";
    const rightWing = "XZXZX";
    try std.testing.expect(checkWingAlignment(leftWing, rightWing));

    // Star-Cloud twirl test
    const dawnPattern = "QWQWQ";
    const duskPattern = "WQWQW";
    try std.testing.expect(checkWingAlignment(dawnPattern, duskPattern));

    // Moonlight waltz test
    const dreamDance = "ZQXWZQ";
    const dreamEcho = try createMirrorPattern(dreamDance);
    defer std.heap.page_allocator.free(dreamEcho);
    try std.testing.expect(checkWingAlignment(dreamDance, dreamEcho));
}

test "moonlight pattern validation" {
    try std.testing.expectEqualStrings(DancePattern.SunMoon.dance, "Z");
    try std.testing.expectEqualStrings(DancePattern.SunMoon.echo, "X");
    try std.testing.expectEqualStrings(DancePattern.StarCloud.twirl, "Q");
    try std.testing.expectEqualStrings(DancePattern.StarCloud.swirl, "W");
}
