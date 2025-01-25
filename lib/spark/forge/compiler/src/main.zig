const std = @import("std");
const Wand = @import("wand.zig").Wand;
const Spellbook = @import("spellbook.zig").Spellbook;

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var wand = try Wand.init(allocator);
    defer wand.deinit();

    var spellbook = try Spellbook.init(allocator);
    defer spellbook.deinit();

    try std.io.getStdOut().writer().print("🔮 Sparkle Compiler v0.1.0\n", .{});

    try wand.castSpell(.{
        .spellbook = &spellbook,
        .incantation = "sparkle build",
    });
}
