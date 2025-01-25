#!/bin/bash

# Spark Comp Fix Script
# Author: isdood
# Created: 2025-01-25 18:56:12 UTC
# Repository: isdood/scribble
# Description: Fixes comp module implementation issues

PURPLE='\033[0;35m'
NC='\033[0m'

print_purple() {
    echo -e "${PURPLE}$1${NC}"
}

fix_comp_module() {
    cd forge/std || exit 1

    # Fix comp module implementation
    cat > src/comp/mod.rs << 'EOL'
//! Crystal-optimized comparison system
//!
//! Similar to std::cmp but with crystal-space optimizations and SIMD support.

use std::cmp::Ordering;

/// Result type for crystal-optimized comparisons
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompResult {
    /// Less than
    Less,
    /// Equal to
    Equal,
    /// Greater than
    Greater,
    /// Incomparable values
    Incomparable,
}

impl From<Ordering> for CompResult {
    fn from(ord: Ordering) -> Self {
        match ord {
            Ordering::Less => CompResult::Less,
            Ordering::Equal => CompResult::Equal,
            Ordering::Greater => CompResult::Greater,
        }
    }
}

impl CompResult {
    /// Converts to Option<Ordering>
    pub fn as_ordering(&self) -> Option<Ordering> {
        match self {
            CompResult::Less => Some(Ordering::Less),
            CompResult::Equal => Some(Ordering::Equal),
            CompResult::Greater => Some(Ordering::Greater),
            CompResult::Incomparable => None,
        }
    }

    /// Returns true if this is Less
    pub fn is_less(&self) -> bool {
        matches!(self, CompResult::Less)
    }

    /// Returns true if this is Equal
    pub fn is_equal(&self) -> bool {
        matches!(self, CompResult::Equal)
    }

    /// Returns true if this is Greater
    pub fn is_greater(&self) -> bool {
        matches!(self, CompResult::Greater)
    }

    /// Returns true if this is Incomparable
    pub fn is_incomparable(&self) -> bool {
        matches!(self, CompResult::Incomparable)
    }
}

/// Ordering with crystal-space optimizations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompOrd {
    /// Ascending order
    Ascending,
    /// Descending order
    Descending,
}

impl CompOrd {
    /// Reverses the ordering
    pub fn reverse(&self) -> Self {
        match self {
            CompOrd::Ascending => CompOrd::Descending,
            CompOrd::Descending => CompOrd::Ascending,
        }
    }

    /// Applies the ordering to a comparison result
    pub fn apply(&self, result: CompResult) -> CompResult {
        match self {
            CompOrd::Ascending => result,
            CompOrd::Descending => match result {
                CompResult::Less => CompResult::Greater,
                CompResult::Greater => CompResult::Less,
                other => other,
            },
        }
    }
}

/// Trait for crystal-optimized comparison
pub trait CrystalComp {
    /// Compares two values
    fn crystal_cmp(&self, other: &Self) -> CompResult;

    /// Compares two values with a specific ordering
    fn crystal_cmp_by(&self, other: &Self, ord: CompOrd) -> CompResult {
        ord.apply(self.crystal_cmp(other))
    }

    /// Returns true if self is less than other
    fn crystal_lt(&self, other: &Self) -> bool {
        self.crystal_cmp(other).is_less()
    }

    /// Returns true if self is greater than other
    fn crystal_gt(&self, other: &Self) -> bool {
        self.crystal_cmp(other).is_greater()
    }

    /// Returns true if self is equal to other
    fn crystal_eq(&self, other: &Self) -> bool {
        self.crystal_cmp(other).is_equal()
    }

    /// Returns true if self is less than or equal to other
    fn crystal_le(&self, other: &Self) -> bool {
        matches!(self.crystal_cmp(other), CompResult::Less | CompResult::Equal)
    }

    /// Returns true if self is greater than or equal to other
    fn crystal_ge(&self, other: &Self) -> bool {
        matches!(self.crystal_cmp(other), CompResult::Greater | CompResult::Equal)
    }
}

/// Helper for SIMD-accelerated comparisons
#[cfg(target_feature = "avx2")]
pub(crate) mod simd {
    use std::arch::x86_64::*;

    pub unsafe fn compare_bytes_simd(a: &[u8], b: &[u8]) -> crate::comp::CompResult {
        if a.len() != b.len() {
            return if a.len() < b.len() {
                crate::comp::CompResult::Less
            } else {
                crate::comp::CompResult::Greater
            };
        }

        let chunks = a.len() / 32;
        let (prefix, aligned_a, suffix_a) = a.align_to::<__m256i>();
        let (_, aligned_b, suffix_b) = b.align_to::<__m256i>();

        if !prefix.is_empty() {
            for (x, y) in prefix.iter().zip(b.iter()) {
                match x.cmp(y) {
                    std::cmp::Ordering::Less => return crate::comp::CompResult::Less,
                    std::cmp::Ordering::Greater => return crate::comp::CompResult::Greater,
                    std::cmp::Ordering::Equal => continue,
                }
            }
        }

        for i in 0..chunks {
            let a_chunk = _mm256_load_si256(aligned_a.get_unchecked(i));
            let b_chunk = _mm256_load_si256(aligned_b.get_unchecked(i));

            let eq = _mm256_cmpeq_epi8(a_chunk, b_chunk);
            let lt = _mm256_cmpgt_epi8(b_chunk, a_chunk);

            let eq_mask = _mm256_movemask_epi8(eq) as u32;
            let lt_mask = _mm256_movemask_epi8(lt) as u32;

            if eq_mask != 0xFFFFFFFF {
                let diff_pos = eq_mask.trailing_ones();
                let is_less = (lt_mask >> diff_pos) & 1 != 0;
                return if is_less {
                    crate::comp::CompResult::Less
                } else {
                    crate::comp::CompResult::Greater
                };
            }
        }

        for (x, y) in suffix_a.iter().zip(suffix_b.iter()) {
            match x.cmp(y) {
                std::cmp::Ordering::Less => return crate::comp::CompResult::Less,
                std::cmp::Ordering::Greater => return crate::comp::CompResult::Greater,
                std::cmp::Ordering::Equal => continue,
            }
        }

        crate::comp::CompResult::Equal
    }
}

// Implement for integer primitives
macro_rules! impl_crystal_comp_integer {
    ($($t:ty),*) => {
        $(
            impl CrystalComp for $t {
                fn crystal_cmp(&self, other: &Self) -> CompResult {
                    CompResult::from(self.cmp(other))
                }
            }
        )*
    }
}

// Implement for floating point types
macro_rules! impl_crystal_comp_float {
    ($($t:ty),*) => {
        $(
            impl CrystalComp for $t {
                fn crystal_cmp(&self, other: &Self) -> CompResult {
                    if self.is_nan() || other.is_nan() {
                        CompResult::Incomparable
                    } else if self < other {
                        CompResult::Less
                    } else if self > other {
                        CompResult::Greater
                    } else {
                        CompResult::Equal
                    }
                }
            }
        )*
    }
}

impl_crystal_comp_integer!(
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize
);

impl_crystal_comp_float!(f32, f64);

// Implement for bool and char
impl CrystalComp for bool {
    fn crystal_cmp(&self, other: &Self) -> CompResult {
        CompResult::from(self.cmp(other))
    }
}

impl CrystalComp for char {
    fn crystal_cmp(&self, other: &Self) -> CompResult {
        CompResult::from(self.cmp(other))
    }
}

// Implement for slices of comparable types
impl<T: CrystalComp> CrystalComp for &[T] {
    fn crystal_cmp(&self, other: &Self) -> CompResult {
        let len_cmp = self.len().cmp(&other.len());
        if len_cmp != Ordering::Equal {
            return CompResult::from(len_cmp);
        }

        for (a, b) in self.iter().zip(other.iter()) {
            let cmp = a.crystal_cmp(b);
            if !cmp.is_equal() {
                return cmp;
            }
        }

        CompResult::Equal
    }
}

// Implement for string slices
impl CrystalComp for &str {
    fn crystal_cmp(&self, other: &Self) -> CompResult {
        #[cfg(target_feature = "avx2")]
        unsafe {
            simd::compare_bytes_simd(self.as_bytes(), other.as_bytes())
        }

        #[cfg(not(target_feature = "avx2"))]
        CompResult::from(self.cmp(other))
    }
}
EOL

    print_purple "✓ Fixed comp module implementation"
}

main() {
    print_purple "🔮 Fixing Spark Comp Module..."
    fix_comp_module
    print_purple "✨ Comp module fixes applied!

Fixed Issues:
- Removed Sized bound from CrystalComp trait
- Fixed floating-point comparisons
- Added separate macros for integer and float types
- Improved slice and str implementations
- Removed unused imports
- Added better NaN handling
- Fixed compilation errors

Run 'cargo test' to verify the fixes!"
}

main
