//! Crystal tunneling module for Signal type

/// Crystal tunnel state
#[derive(Debug, Clone)]
pub struct Tunnel {
    /// Tunnel width
    width: f64,
    /// Barrier height
    height: f64,
    /// Transmission probability
    probability: f64,
}

impl Tunnel {
    /// Creates a new tunnel state
    pub fn new(width: f64, height: f64, probability: f64) -> Self {
        Self {
            width: width.abs(),
            height: height.abs(),
            probability: probability.clamp(0.0, 1.0),
        }
    }

    /// Creates a tunnel from bytes
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let width = bytes.iter().fold(0.0, |acc, &b| acc + b as f64) / bytes.len() as f64;
        let height = bytes.iter().fold(0.0, |acc, &b| acc + (b as f64 / 255.0)) / bytes.len() as f64;
        let probability = bytes.windows(2).fold(0.0, |acc, w| {
            acc + (w[0] as f64 - w[1] as f64).abs() / 255.0
        }) / (bytes.len() - 1) as f64;

        Self::new(width, height, probability)
    }

    /// Gets the tunnel width
    pub fn width(&self) -> f64 {
        self.width
    }

    /// Gets the barrier height
    pub fn height(&self) -> f64 {
        self.height
    }

    /// Gets the transmission probability
    pub fn probability(&self) -> f64 {
        self.probability
    }

    /// Tunnels through bytes
    pub fn tunnel(&mut self, bytes: &[u8]) {
        let new = Self::from_bytes(bytes);
        self.width = (self.width + new.width) / 2.0;
        self.height = (self.height + new.height) / 2.0;
        self.probability = (self.probability * new.probability).sqrt();
    }

    /// Verifies transmission
    pub fn verify_transmission(&self, bytes: &[u8]) -> Result<(), String> {
        let transmitted = bytes.iter().fold(0.0, |acc, &b| acc + b as f64) / bytes.len() as f64;
        if (transmitted - self.width).abs() > self.height {
            Err("Transmission verification failed".to_string())
        } else {
            Ok(())
        }
    }

    /// Synchronizes with another tunnel
    pub fn synchronize(&mut self, other: &Self) -> Result<(), String> {
        if (self.probability - other.probability).abs() > 0.5 {
            Err("Tunnel synchronization failed".to_string())
        } else {
            self.width = (self.width + other.width) / 2.0;
            self.height = (self.height + other.height) / 2.0;
            self.probability = (self.probability * other.probability).sqrt();
            Ok(())
        }
    }

    /// Amplifies the tunnel
    pub fn amplify(&mut self, gain: f64) -> Result<(), String> {
        if gain <= 0.0 {
            return Err("Invalid gain factor".to_string());
        }
        self.width *= gain;
        self.height *= gain;
        self.probability = self.probability.powf(1.0 / gain).min(1.0);
        Ok(())
    }
}

impl Default for Tunnel {
    fn default() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }
}

impl std::ops::BitAnd for Tunnel {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self::new(
            (self.width + rhs.width) / 2.0,
            self.height.min(rhs.height),
            self.probability * rhs.probability,
        )
    }
}

impl std::ops::BitOr for Tunnel {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self::new(
            self.width.max(rhs.width),
            self.height.max(rhs.height),
            (self.probability + rhs.probability) / 2.0,
        )
    }
}

impl std::ops::BitXor for Tunnel {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self::new(
            (self.width * rhs.width).sqrt(),
            (self.height * rhs.height).sqrt(),
            (self.probability + rhs.probability) / 2.0,
        )
    }
}

impl std::ops::Not for Tunnel {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self::new(
            1.0 / self.width,
            1.0 / self.height,
            1.0 - self.probability,
        )
    }
}
