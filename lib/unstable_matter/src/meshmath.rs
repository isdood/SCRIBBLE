/// MeshSpace Mathematics Implementation
/// Last Updated: 2025-01-18 15:03:45 UTC
/// Author: isdood
/// Current User: isdood

// Operation traits
pub trait MeshAdd<Rhs = Self> {
    type Output;
    fn mesh_add(self, rhs: Rhs) -> Self::Output;
}

pub trait MeshSub<Rhs = Self> {
    type Output;
    fn mesh_sub(self, rhs: Rhs) -> Self::Output;
}

pub trait MeshMul<Rhs = Self> {
    type Output;
    fn mesh_mul(self, rhs: Rhs) -> Self::Output;
}

pub trait MeshDiv<Rhs = Self> {
    type Output;
    fn mesh_div(self, rhs: Rhs) -> Self::Output;
}

#[derive(Debug, Clone, PartialEq)]
pub struct MeshMath;

impl MeshMath {
    /// Normalizes an angle to be within the range [-π, π]
    pub fn normalize_angle(angle: f64) -> f64 {
        let two_pi = 2.0 * std::f64::consts::PI;
        let mut normalized = angle % two_pi;
        if normalized > std::f64::consts::PI {
            normalized -= two_pi;
        } else if normalized < -std::f64::consts::PI {
            normalized += two_pi;
        }
        normalized
    }

    /// Returns the absolute value of a number
    pub fn abs(x: f64) -> f64 {
        if x < 0.0 {
            -x
        } else {
            x
        }
    }

    /// Custom square root implementation for mesh calculations
    pub fn sqrt(x: f64) -> f64 {
        if x < 0.0 {
            panic!("Cannot calculate square root of negative number in meshspace");
        }
        if x == 0.0 {
            return 0.0;
        }

        let mut guess = x / 2.0;
        let mut prev_guess = 0.0;
        let epsilon: f64 = 1e-15;

        while (guess - prev_guess).abs() > epsilon {
            prev_guess = guess;
            guess = (guess + x / guess) / 2.0;
        }

        guess
    }

    /// Custom sine implementation using Taylor series
    pub fn sin(x: f64) -> f64 {
        // Normalize angle to -2π to 2π range
        let x = x % (2.0 * std::f64::consts::PI);

        let mut result = 0.0;
        let mut term = x;
        let mut n = 1;

        for i in 0..12 { // 12 terms for good precision
            if i > 0 {
                term = -term * x * x / ((2 * n) * (2 * n + 1)) as f64;
                n += 1;
            }
            result += term;
        }

        result
    }

    /// Custom cosine implementation using sin(x + π/2)
    pub fn cos(x: f64) -> f64 {
        Self::sin(x + std::f64::consts::PI / 2.0)
    }

    /// Custom exponential function implementation
    pub fn exp(x: f64) -> f64 {
        let mut result = 1.0;
        let mut term = 1.0;
        let mut n = 1;
        let epsilon: f64 = 1e-15;

        while term.abs() > epsilon && n < 100 {
            term *= x / n as f64;
            result += term;
            n += 1;
        }

        result
    }

    /// Custom natural logarithm implementation
    pub fn ln(x: f64) -> f64 {
        if x <= 0.0 {
            panic!("Cannot calculate natural logarithm of non-positive number in meshspace");
        }

        let mut result = 0.0;
        let y = (x - 1.0) / (x + 1.0);
        let mut power = y;
        let mut n = 1;
        let epsilon: f64 = 1e-15;

        while power.abs() > epsilon && n < 100 {
            result += power / n as f64;
            power *= y * y;
            n += 2;
        }

        2.0 * result
    }

    /// Custom power function implementation
    pub fn pow(x: f64, y: f64) -> f64 {
        Self::exp(y * Self::ln(x))
    }

    /// Calculate hyperbolic sine
    pub fn sinh(x: f64) -> f64 {
        (Self::exp(x) - Self::exp(-x)) / 2.0
    }

    /// Calculate hyperbolic cosine
    pub fn cosh(x: f64) -> f64 {
        (Self::exp(x) + Self::exp(-x)) / 2.0
    }

    /// Convert isize to f64
    pub fn to_f64(x: isize) -> f64 {
        x as f64
    }
}

// Implement mesh operations for f64
impl MeshAdd<f64> for f64 {
    type Output = f64;
    fn mesh_add(self, rhs: f64) -> f64 {
        self + rhs
    }
}

impl MeshSub<f64> for f64 {
    type Output = f64;
    fn mesh_sub(self, rhs: f64) -> f64 {
        self - rhs
    }
}

impl MeshMul<f64> for f64 {
    type Output = f64;
    fn mesh_mul(self, rhs: f64) -> f64 {
        self * rhs
    }
}

impl MeshDiv<f64> for f64 {
    type Output = f64;
    fn mesh_div(self, rhs: f64) -> f64 {
        self / rhs
    }
}

// Implement mesh operations for isize
impl MeshAdd<isize> for isize {
    type Output = isize;
    fn mesh_add(self, rhs: isize) -> isize {
        self + rhs
    }
}

impl MeshSub<isize> for isize {
    type Output = isize;
    fn mesh_sub(self, rhs: isize) -> isize {
        self - rhs
    }
}

impl MeshMul<isize> for isize {
    type Output = isize;
    fn mesh_mul(self, rhs: isize) -> isize {
        self * rhs
    }
}

impl MeshDiv<isize> for isize {
    type Output = isize;
    fn mesh_div(self, rhs: isize) -> isize {
        self / rhs
    }
}
