const std = @import("std");
const Wand = @import("wand.zig").Wand;

pub const Spellbook = struct {
    allocator: std.mem.Allocator,
    enchantments: std.ArrayList([]const u8),

    pub fn init(allocator: std.mem.Allocator) !Spellbook {
        return Spellbook{
            .allocator = allocator,
            .enchantments = std.ArrayList([]const u8).init(allocator),
        };
    }

    pub fn deinit(self: *Spellbook) void {
        for (self.enchantments.items) |enchantment| {
            self.allocator.free(enchantment);
        }
        self.enchantments.deinit();
    }

    pub fn inscribe(self: *Spellbook, tokens: []const Wand.Token) !void {
        for (tokens) |token| {
            switch (token) {
                .star_path => |path| try self.translateStarPath(path),
                .enchant => |func| try self.translateEnchantment(func),
                .potion => |var_decl| try self.translatePotion(var_decl),
                .scroll => |import| try self.translateScroll(import),
            }
        }
    }

    fn translateStarPath(self: *Spellbook, path: []const u8) !void {
        const translated = try std.fmt.allocPrint(
            self.allocator,
            "// Magical path: {s}\n",
            .{path}
        );
        try self.enchantments.append(translated);
    }

    fn translateEnchantment(self: *Spellbook, func: []const u8) !void {
        const translated = try std.fmt.allocPrint(
            self.allocator,
            "// Enchantment: {s}\n",
            .{func}
        );
        try self.enchantments.append(translated);
    }

    fn translatePotion(self: *Spellbook, var_decl: []const u8) !void {
        const translated = try std.fmt.allocPrint(
            self.allocator,
            "// Potion: {s}\n",
            .{var_decl}
        );
        try self.enchantments.append(translated);
    }

    fn translateScroll(self: *Spellbook, import: []const u8) !void {
        const translated = try std.fmt.allocPrint(
            self.allocator,
            "// Scroll: {s}\n",
            .{import}
        );
        try self.enchantments.append(translated);
    }
};
