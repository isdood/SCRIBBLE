// src/gdt.rs
use x86_64::instructions::segmentation::{CS, Segment};
use x86_64::structures::gdt::{GlobalDescriptorTable, Descriptor, SegmentSelector};
use x86_64::structures::tss::TaskStateSegment;
use x86_64::{VirtAddr, PrivilegeLevel};
use lazy_static::lazy_static;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

#[derive(Debug)]
pub enum GDTError {
    VerificationFailed,
    InvalidSelector,
}

pub struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

impl Selectors {
    fn verify(&self) -> Result<(), GDTError> {
        if self.code_selector.rpl() != PrivilegeLevel::Ring0 {
            return Err(GDTError::VerificationFailed);
        }
        Ok(())
    }

    pub fn code_selector(&self) -> SegmentSelector {
        self.code_selector
    }

    pub fn tss_selector(&self) -> SegmentSelector {
        self.tss_selector
    }
}

lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
            let stack_end = stack_start + STACK_SIZE;
            stack_end
        };
        tss
    };
}

lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(unsafe { &raw const STACK });
            let stack_end = stack_start + STACK_SIZE;
            stack_end
        };
        tss
    };
}

pub fn init() {
    use x86_64::instructions::tables;
    GDT.0.load();
    unsafe {
        CS::set_reg(GDT.1.code_selector);
        tables::load_tss(GDT.1.tss_selector);
    }
}

pub fn get_current_privilege_level() -> u8 {
    CS::get_reg().rpl() as u8
}
