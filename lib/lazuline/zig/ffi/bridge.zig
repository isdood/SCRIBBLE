const std = @import("std");
const crystal = @import("crystal");
const CrystalCore = crystal.CrystalCore;

export fn crystal_core_init() ?*CrystalCore {
    const core = std.heap.c_allocator.create(CrystalCore) catch return null;
    core.* = CrystalCore.init();
    return core;
}

export fn crystal_core_process_task(core: ?*CrystalCore, task: [*]const u8, len: usize) void {
    if (core) |c| {
        c.processTask(task[0..len]);
    }
}
