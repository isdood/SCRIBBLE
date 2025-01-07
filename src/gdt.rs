//IMPORTS
use x86_64::structures::tss::TaskStateSegment;
use x86_64::structures::gdt::{GlobalDescriptorTable, Descriptor, SegmentSelector};
use x86_64::instructions::{segmentation::{CS, Segment}, tables};
use x86_64::VirtAddr;
use lazy_static::lazy_static;
use crate::splat::{self, SplatLevel};
use core::sync::atomic::{AtomicBool, Ordering};

// GDT Configuration Constants
pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;
pub const STACK_SIZE: usize = 4096 * 5;  // 20KiB
const EXPECTED_GDT_ENTRIES: usize = 5;   // Expected number of GDT entries

// System Status Tracking
static GDT_INITIALIZED: AtomicBool = AtomicBool::new(false);
static TSS_LOADED: AtomicBool = AtomicBool::new(false);

#[derive(Debug)]
pub enum GDTError {
    InvalidSelector,
    LoadError,
    TSSError,
    VerificationFailed,
}

lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();

        // Configure emergency stack for double faults
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
            let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
            let stack_end = stack_start + STACK_SIZE;

            splat::log(
                SplatLevel::BitsNBytes,
                &format!(
                    "Emergency Stack Configuration:\n\
└─ Start: {:#x}\n\
└─ End: {:#x}\n\
└─ Size: {} KiB",
stack_start.as_u64(),
                         stack_end.as_u64(),
                         STACK_SIZE / 1024
                )
            );

            stack_end
        };

        tss
    };

    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();

        // Configure GDT entries
        let kernel_code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let kernel_data_selector = gdt.add_entry(Descriptor::kernel_data_segment());
        let user_code_selector = gdt.add_entry(Descriptor::user_code_segment());
        let user_data_selector = gdt.add_entry(Descriptor::user_data_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));

        (
            gdt,
         Selectors {
             kernel_code_selector,
             kernel_data_selector,
             user_code_selector,
             user_data_selector,
             tss_selector,
         }
        )
    };
}

#[derive(Debug)]
struct Selectors {
    kernel_code_selector: SegmentSelector,
    kernel_data_selector: SegmentSelector,
    user_code_selector: SegmentSelector,
    user_data_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

impl Selectors {
    fn verify(&self) -> Result<(), GDTError> {
        if !self.verify_privilege_levels() || !self.verify_tss() {
            return Err(GDTError::VerificationFailed);
        }
        Ok(())
    }

    fn verify_privilege_levels(&self) -> bool {
        self.kernel_code_selector.rpl() == 0 &&
        self.kernel_data_selector.rpl() == 0 &&
        self.user_code_selector.rpl() == 3 &&
        self.user_data_selector.rpl() == 3
    }

    fn verify_tss(&self) -> bool {
        !self.tss_selector.ti()
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
self.kernel_code_selector.rpl(),
                     self.kernel_data_selector.0,
                     self.kernel_data_selector.rpl(),
                     self.user_code_selector.0,
                     self.user_code_selector.rpl(),
                     self.user_data_selector.0,
                     self.user_data_selector.rpl(),
                     self.tss_selector.0
            )
        );
    }
}

pub fn init() -> Result<(), GDTError> {
    if GDT_INITIALIZED.load(Ordering::Relaxed) {
        splat::log(SplatLevel::Warning, "GDT already initialized");
        return Ok(());
    }

    splat::log(SplatLevel::BitsNBytes, "Starting GDT initialization");

    // Verify GDT configuration
    GDT.1.verify()?;

    // Load GDT and configure segments
    unsafe {
        GDT.0.load();
        configure_segments()?;
    }

    GDT.1.log_status();
    GDT_INITIALIZED.store(true, Ordering::Relaxed);

    splat::log(
        SplatLevel::BitsNBytes,
        "GDT initialization completed successfully"
    );

    Ok(())
}

unsafe fn configure_segments() -> Result<(), GDTError> {
    // Set code segment
    CS::set_reg(GDT.1.kernel_code_selector);

    // Load TSS
    tables::load_tss(GDT.1.tss_selector);
    TSS_LOADED.store(true, Ordering::Relaxed);

    verify_segment_configuration()
}

fn verify_segment_configuration() -> Result<(), GDTError> {
    unsafe {
        if CS::get_reg() != GDT.1.kernel_code_selector {
            return Err(GDTError::LoadError);
        }

        if !tables::is_tss_loaded() {
            return Err(GDTError::TSSError);
        }
    }

    Ok(())
}

pub fn get_gdt_status() -> String {
    format!(
        "GDT Status:\n\
└─ Initialized: {}\n\
└─ TSS Loaded: {}\n\
└─ Current CPL: {}\n\
└─ Emergency Stack: Available",
GDT_INITIALIZED.load(Ordering::Relaxed),
            TSS_LOADED.load(Ordering::Relaxed),
            get_current_privilege_level()
    )
}

pub fn get_current_privilege_level() -> u8 {
    unsafe { CS::get_reg().rpl() }
}

pub fn log_gdt_status() {
    splat::log(SplatLevel::BitsNBytes, &get_gdt_status());
}

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
        assert_eq!(selectors.kernel_code_selector.rpl(), 0);
        assert_eq!(selectors.user_code_selector.rpl(), 3);
    }
}
