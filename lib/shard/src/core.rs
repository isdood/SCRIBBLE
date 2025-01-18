// libs/shard/src/core.rs

use crate::hashbrown::{QuantumHashMap, HashBrownConfig};
use crate::vector4d::{Vector4D, HyperRotation, QuatTransform};

#[derive(Debug, Clone)]
pub struct ShardRegisterFile {
    v_regs: [Vector4D; 8],          // V0-V7
    qs_regs: [Vec<f64>; 4],         // QS0-QS3
    cr_regs: [Vec<f64>; 2],         // CR0-CR1
    rp_regs: [[f64; 7]; 2],         // RP0-RP1
    pc4d: Vector4D,                 // 4D Program Counter
    qf: u64,                        // Quantum Flags
}

#[derive(Debug, Clone)]
pub struct ShardMemory {
    l1q: QuantumHashMap<Vector4D, f64>,    // L1 Quantum Cache
    l2c: QuantumHashMap<Vector4D, f64>,    // L2 Crystal Cache
    l3h: QuantumHashMap<Vector4D, f64>,    // L3 Hyperspace Cache
}

impl ShardMemory {
    pub fn new() -> Self {
        let config = HashBrownConfig {
            quantum_threshold: 0.87,
            max_entries: 1024,
            creator: b"isdood".to_vec(),
        };

        Self {
            l1q: QuantumHashMap::new(config.clone()),
            l2c: QuantumHashMap::new(config.clone()),
            l3h: QuantumHashMap::new(config),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ShardOpcode {
    VADD4D,
    VMUL4D,
    VROT4D,
    VPROJ4D,
    QENT,
    QCOH,
    QPHASE,
    QBRIDGE,
    CGROW,
    CLATT,
    CRES,
    CFACET,
    LOAD4D,
    STORE4D,
    LOADQ,
    STOREQ,
    // ... add other opcodes
}

#[derive(Debug, Clone)]
pub struct ShardInstruction {
    pub opcode: ShardOpcode,
    pub dest: usize,
    pub src1: usize,
    pub src2: Option<usize>,
    pub imm: Option<f64>,
    pub addr: Option<Vector4D>,
}
