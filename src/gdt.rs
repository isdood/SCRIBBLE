// src/gdt.rs
use unstable_matter::arch::x86_64::structures::gdt::{GlobalDescriptorTable, Descriptor};
use unstable_matter::arch::x86_64::structures::tss::TaskStateSegment;
use unstable_matter::arch::x86_64::instructions::segmentation::{Segment, CS};
use unstable_matter::arch::x86_64::instructions::tables;
use unstable_matter::VirtAddr;
use lazy_static::lazy_static;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

/// Holds the selectors for code and TSS
#[derive(Debug)]
pub struct Selectors {
    pub code_selector: SegmentSelector,
    pub tss_selector: SegmentSelector,
}

lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(&raw const STACK as *const u8);
            let stack_end = stack_start + STACK_SIZE;
            stack_end
        };
        tss
    };
}

lazy_static! {
    pub static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
        (
            gdt,
         Selectors {
             code_selector,
             tss_selector,
         },
        )
    };
}

/// Initialize the Global Descriptor Table
pub fn init() {
    // Load the GDT
    GDT.0.load();

    // Update segment registers
    unsafe {
        // Must use Segment trait for CS
        CS::set_reg(GDT.1.code_selector);
        x86_64::instructions::tables::load_tss(GDT.1.tss_selector);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_case]
    fn test_gdt_init() {
        init();
        // If we reach here, the GDT was loaded successfully
    }
}
