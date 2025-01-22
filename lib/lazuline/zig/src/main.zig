const std = @import("std");
const harmony = @import("harmony.zig");
const executor = @import("executor.zig");

pub const CrystalCore = struct {
    harmony_state: harmony.HarmonyState,
    executor: executor.Executor,

    pub fn init() CrystalCore {
        return .{
            .harmony_state = harmony.HarmonyState.init(),
            .executor = executor.Executor.init(),
        };
    }

    pub fn processTask(self: *CrystalCore, task: []const u8) void {
        if (task.len > 0) {
            self.harmony_state.process();
            self.executor.execute(task);
        }
    }
};
