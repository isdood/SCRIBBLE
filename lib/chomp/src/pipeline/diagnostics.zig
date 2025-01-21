///! Diagnostic System (Continued)
///! ==============
///! Author: Caleb J.D. Terkovics <isdood>
///! Created: 2025-01-21 02:57:39 UTC
///! License: MIT

fn emitError(self: *Diagnostics, error: Error) !void {
    const writer = std.io.getStdErr().writer();

    // Print error header in red
    try writer.print("\x1b[31mError[{s}]\x1b[0m: {s}\n", .{
        error.code,
        error.message,
    });

    // Print location information
    try writer.print("  --> {s}:{}:{}\n", .{
        error.location.file,
        error.location.line,
        error.location.column,
    });

    // Print code snippet with error highlighting
    try self.printCodeSnippet(error.location);

    // Print help message if available
    if (error.help) |help| {
        try writer.print("\nHelp: {s}\n", .{help});
    }

    // Print suggestions if available
    if (error.suggestions.len > 0) {
        try writer.writeAll("\nSuggestions:\n");
        for (error.suggestions) |suggestion| {
            try writer.print("  - {s}\n", .{suggestion});
        }
    }
}

fn emitWarning(self: *Diagnostics, warning: Warning) !void {
    const writer = std.io.getStdErr().writer();

    // Print warning header in yellow
    try writer.print("\x1b[33mWarning[{s}]\x1b[0m: {s}\n", .{
        warning.code,
        warning.message,
    });

    // Print location information
    try writer.print("  --> {s}:{}:{}\n", .{
        warning.location.file,
        warning.location.line,
        warning.location.column,
    });

    // Print code snippet with warning highlighting
    try self.printCodeSnippet(warning.location);

    // Print help message if available
    if (warning.help) |help| {
        try writer.print("\nHelp: {s}\n", .{help});
    }
}

fn printCodeSnippet(self: *Diagnostics, location: Location) !void {
    const writer = std.io.getStdErr().writer();
    const file = try std.fs.cwd().openFile(location.file, .{});
    defer file.close();

    var line_number: usize = 1;
    var buffer: [1024]u8 = undefined;
    var line_start: usize = 0;
    var reader = file.reader();

    while (try reader.readUntilDelimiterOrEof(&buffer, '\n')) |line| {
        if (line_number >= location.line - 2 and line_number <= location.line + 2) {
            if (line_number == location.line) {
                // Print the error line with highlighting
                try writer.print("\x1b[34m{:>4} |\x1b[0m {s}\n", .{line_number, line});
                try writer.writeAll("     | ");

                // Print the error indicator
                var i: usize = 0;
                while (i < location.column - 1) : (i += 1) {
                    try writer.writeAll(" ");
                }
                try writer.print("\x1b[31m{s}\x1b[0m\n", .{"^"});
            } else {
                // Print context lines
                try writer.print("\x1b[34m{:>4} |\x1b[0m {s}\n", .{line_number, line});
            }
        }
        line_number += 1;
    }
}

pub fn addNote(self: *Diagnostics, note: Note) !void {
    try self.notes.append(note);
    try self.emitNote(note);
}

fn emitNote(self: *Diagnostics, note: Note) !void {
    const writer = std.io.getStdErr().writer();

    // Print note in blue
    try writer.print("\x1b[36mNote\x1b[0m: {s}\n", .{note.message});

    if (note.location) |location| {
        try writer.print("  --> {s}:{}:{}\n", .{
            location.file,
            location.line,
            location.column,
        });
        try self.printCodeSnippet(location);
    }
}

pub fn hasErrors(self: Diagnostics) bool {
    return self.errors.items.len > 0;
}

pub fn errorCount(self: Diagnostics) usize {
    return self.errors.items.len;
}

pub fn warningCount(self: Diagnostics) usize {
    return self.warnings.items.len;
}

pub const DiagnosticFormatter = struct {
    pub fn formatError(
        allocator: std.mem.Allocator,
        comptime format: []const u8,
        args: anytype,
        location: Location,
    ) !Error {
        return Error{
            .code = "E0001",
            .message = try std.fmt.allocPrint(allocator, format, args),
            .location = location,
            .suggestions = &[_][]const u8{},
        };
    }

    pub fn formatWarning(
        allocator: std.mem.Allocator,
        comptime format: []const u8,
        args: anytype,
        location: Location,
    ) !Warning {
        return Warning{
            .code = "W0001",
            .message = try std.fmt.allocPrint(allocator, format, args),
            .location = location,
        };
    }

    pub fn formatNote(
        allocator: std.mem.Allocator,
        comptime format: []const u8,
        args: anytype,
    ) !Note {
        return Note{
            .message = try std.fmt.allocPrint(allocator, format, args),
            .location = null,
            .related_error = null,
        };
    }
};

pub fn deinit(self: *Diagnostics) void {
    for (self.errors.items) |error| {
        self.allocator.free(error.message);
        if (error.help) |help| {
            self.allocator.free(help);
        }
        for (error.suggestions) |suggestion| {
            self.allocator.free(suggestion);
        }
    }
    self.errors.deinit();

    for (self.warnings.items) |warning| {
        self.allocator.free(warning.message);
        if (warning.help) |help| {
            self.allocator.free(help);
        }
    }
    self.warnings.deinit();

    for (self.notes.items) |note| {
        self.allocator.free(note.message);
    }
    self.notes.deinit();

    self.allocator.destroy(self);
}
};
