const std = @import("std");
const testing = std.testing;
const CrystalCore = @import("../core/crystal_core.zig").CrystalCore;

test "crystal core formation measurement" {
    var core = CrystalCore.init();
    try testing.expectEqual(core.formation_speed, 0);
    try testing.expectApproxEqAbs(core.lattice_alignment, 0.93, 0.001);

    core.measureFormation();
    try testing.expect(core.formation_speed == 1);
    try testing.expect(core.last_measurement > 0);

    const latency = core.getLatencyNs();
    try testing.expect(latency >= 0);
}
