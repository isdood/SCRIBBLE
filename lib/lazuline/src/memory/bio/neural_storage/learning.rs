//! Learning System Implementation
//! Created: 2025-01-22
//! Author: isdood

pub struct LearningSystem {
    knowledge: Vec<String>,
}

impl LearningSystem {
    pub fn new() -> Self {
        LearningSystem {
            knowledge: Vec::new(),
        }
    }

    pub fn learn(&mut self, data: String) {
        self.knowledge.push(data);
    }

    pub fn get_knowledge(&self) -> &Vec<String> {
        &self.knowledge
    }
}
