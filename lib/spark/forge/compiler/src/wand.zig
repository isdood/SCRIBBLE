const std = @import("std");
const Spellbook = @import("spellbook.zig").Spellbook;

pub const Wand = struct {
    allocator: std.mem.Allocator,
    sparkles: std.ArrayList(Token),

    pub const Token = union(enum) {
        star_path: []const u8,
        enchant: []const u8,
        potion: []const u8,
        scroll: []const u8,
    };

    pub fn init(allocator: std.mem.Allocator) !Wand {
        return Wand{
            .allocator = allocator,
            .sparkles = std.ArrayList(Token).init(allocator),
        };
    }

    pub fn deinit(self: *Wand) void {
        self.sparkles.deinit();
    }

    pub const SpellOptions = struct {
        spellbook: *Spellbook,
        incantation: []const u8,
    };

    pub fn castSpell(self: *Wand, options: SpellOptions) !void {
        try self.sparkles.append(.{ .star_path = "std**math" });
        try options.spellbook.inscribe(self.sparkles.items);
    }
};
