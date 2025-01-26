const std = @import("std");
const testing = std.testing;
const Wand = @import("wand.zig").Wand;
const Spellbook = @import("spellbook.zig").Spellbook;

test "basic star path parsing" {
    var allocator = testing.allocator;

    var wand = try Wand.init(allocator);
    defer wand.deinit();

    var spellbook = try Spellbook.init(allocator);
    defer spellbook.deinit();

    try wand.castSpell(.{
        .spellbook = &spellbook,
        .incantation = "std**math**add",
    });

    try testing.expect(spellbook.enchantments.items.len > 0);
}
