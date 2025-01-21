const std = @import("std");

pub export fn lazuline_init() c_int {
    return 0;
}

pub export fn lazuline_version() [*:0]const u8 {
    return "0.1.0";
}

test "basic initialization" {
    try std.testing.expectEqual(@as(c_int, 0), lazuline_init());
}

test "version string" {
    const version = lazuline_version();
    try std.testing.expect(std.mem.eql(u8, std.mem.span(version), "0.1.0"));
}
