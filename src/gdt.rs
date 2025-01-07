// src/gdt.rs

//IMPORTS
use x86_64::VirtAddr;
use x86_64::structures::tss::TaskStateSegment;
use x86_64::structures::gdt::{GlobalDescriptorTable, Descriptor, SegmentSelector};
use x86_64::PrivilegeLevel;
use x86_64::instructions::{segmentation::CS, tables};
use core::sync::atomic::{AtomicBool, Ordering};
use lazy_static::lazy_static;
use alloc::string::String;
use alloc::format;
use crate::splat::{self, SplatLevel};

// Rest of the code remains the same until Selectors impl...

impl Selectors {
    fn verify(&self) -> Result<(), GDTError> {
        if !self.verify_privilege_levels() || !self.verify_tss() {
            return Err(GDTError::VerificationFailed);
        }
        Ok(())
    }

    fn verify_privilege_levels(&self) -> bool {
        // Convert raw RPL to PrivilegeLevel for comparison
        self.kernel_code_selector.rpl() == PrivilegeLevel::Ring0 &&
        self.kernel_data_selector.rpl() == PrivilegeLevel::Ring0 &&
        self.user_code_selector.rpl() == PrivilegeLevel::Ring3 &&
        self.user_data_selector.rpl() == PrivilegeLevel::Ring3
    }

    fn verify_tss(&self) -> bool {
        // Check Table Indicator bit directly
        (self.tss_selector.0 & 0x04) == 0
    }

    fn log_status(&self) {
        splat::log(
            SplatLevel::BitsNBytes,
            &format!(
                "GDT Configuration:\n\
└─ Kernel Segments:\n\
│  └─ Code: {:#x} (RPL {})\n\
│  └─ Data: {:#x} (RPL {})\n\
└─ User Segments:\n\
│  └─ Code: {:#x} (RPL {})\n\
│  └─ Data: {:#x} (RPL {})\n\
└─ TSS: {:#x}",
self.kernel_code_selector.0,
self.kernel_code_selector.rpl() as u8,
                     self.kernel_data_selector.0,
                     self.kernel_data_selector.rpl() as u8,
                     self.user_code_selector.0,
                     self.user_code_selector.rpl() as u8,
                     self.user_data_selector.0,
                     self.user_data_selector.rpl() as u8,
                     self.tss_selector.0
            )
        );
    }
}

// Update get_current_privilege_level to return u8 instead of PrivilegeLevel
pub fn get_current_privilege_level() -> u8 {
    unsafe { CS::get_reg().rpl() as u8 }
}

// Rest of the code remains the same...

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selector_verification() {
        let selectors = &GDT.1;
        assert!(selectors.verify().is_ok());
    }

    #[test]
    fn test_privilege_levels() {
        let selectors = &GDT.1;
        assert_eq!(selectors.kernel_code_selector.rpl(), PrivilegeLevel::Ring0);
        assert_eq!(selectors.user_code_selector.rpl(), PrivilegeLevel::Ring3);
    }
}
