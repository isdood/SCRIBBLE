use crate::harmony::HarmonyWeaver;
use crate::crystals::CrystalField;

pub struct ResonanceCore {
    harmony_weaver: HarmonyWeaver,
    crystal_field: CrystalField,
    whimsy_factor: f64,
}

impl ResonanceCore {
    pub fn new() -> Self {
        Self {
            harmony_weaver: HarmonyWeaver::new(),
            crystal_field: CrystalField::new(),
            whimsy_factor: 0.618033988749895, // Golden ratio for maximum whimsy
        }
    }

    pub async fn weave_harmony(&self) -> f64 {
        self.harmony_weaver.weave(self.whimsy_factor)
    }
}
