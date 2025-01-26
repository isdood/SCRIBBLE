//! ASCII character set implementation

/// A set of ASCII characters
#[derive(Clone, Debug, Default)]
pub struct AsciiSet {
    bits: [u128; 2], // 256 bits to cover all ASCII characters
}

impl AsciiSet {
    /// Creates a new empty ASCII set
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new ASCII set from a string of characters
    pub fn from_str(s: &str) -> Option<Self> {
        if !s.is_ascii() {
            return None;
        }

        let mut set = Self::new();
        for b in s.bytes() {
            set.insert(b);
        }
        Some(set)
    }

    /// Adds an ASCII character to the set
    pub fn insert(&mut self, byte: u8) {
        let idx = (byte >> 7) as usize;
        let bit = byte & 0x7F;
        self.bits[idx] |= 1u128 << bit;
    }

    /// Removes an ASCII character from the set
    pub fn remove(&mut self, byte: u8) {
        let idx = (byte >> 7) as usize;
        let bit = byte & 0x7F;
        self.bits[idx] &= !(1u128 << bit);
    }

    /// Returns true if the set contains the given ASCII character
    pub fn contains(&self, byte: u8) -> bool {
        let idx = (byte >> 7) as usize;
        let bit = byte & 0x7F;
        (self.bits[idx] & (1u128 << bit)) != 0
    }

    /// Returns true if the set is empty
    pub fn is_empty(&self) -> bool {
        self.bits.iter().all(|&x| x == 0)
    }

    /// Clears the set
    pub fn clear(&mut self) {
        self.bits = [0; 2];
    }

    /// Returns the number of characters in the set
    pub fn len(&self) -> usize {
        self.bits.iter().map(|x| x.count_ones() as usize).sum()
    }

    /// Returns an iterator over the characters in the set
    pub fn iter(&self) -> impl Iterator<Item = u8> + '_ {
        (0..=255u8).filter(move |&b| self.contains(b))
    }
}
