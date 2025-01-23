#[derive(Debug)]
pub struct DNASequence {
    sequence: String,
}

impl DNASequence {
    pub fn new(sequence: &str) -> Result<Self, &'static str> {
        if !sequence.chars().all(|c| "ATCG".contains(c)) {
            return Err("Invalid DNA sequence");
        }
        Ok(DNASequence { sequence: sequence.to_string() })
    }

    pub fn get_sequence(&self) -> &str {
        &self.sequence
    }
}
