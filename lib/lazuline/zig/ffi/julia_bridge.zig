const std = @import("std");
const crystal = @import("crystal");

export fn julia_harmony_init() ?*crystal.harmony.HarmonyState {
    const state = std.heap.c_allocator.create(crystal.harmony.HarmonyState) catch return null;
    state.* = crystal.harmony.HarmonyState.init();
    return state;
}

export fn julia_harmony_process(state: ?*crystal.harmony.HarmonyState) void {
    if (state) |s| {
        s.process();
    }
}
