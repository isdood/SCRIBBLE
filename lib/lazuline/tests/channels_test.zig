const std = @import("std");
const testing = std.testing;
const lazuline = @import("lazuline");

test "crystal channel basic operations" {
    var arena = std.heap.ArenaAllocator.init(std.testing.allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    var channel = lazuline.crystal.channels.CrystalChannel.init(allocator, .{});
    defer channel.deinit();

    // Test sending data
    const test_data = "Hello, Crystal!";
    try channel.send(test_data);

    // Test receiving data
    const received = try channel.receive();
    defer allocator.free(received);

    try testing.expectEqualStrings(test_data, received);
}

test "crystal timer operations" {
    var timer = try lazuline.crystal.timers.CrystalTimer.init(.{});

    // Test basic timing
    try timer.sleep(1_000_000); // 1ms
    const elapsed = try timer.tick();

    try testing.expect(elapsed >= 1_000_000);
}
