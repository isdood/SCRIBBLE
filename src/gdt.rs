use x86_64::VirtAddr;
use x86_64::structures::tss::TaskStateSegment;
use x86_64::structures::gdt::{GlobalDescriptorTable, Descriptor};
use x86_64::registers::segmentation::CS;
use x86_64::instructions::segmentation::Segment;  // Add back the Segment trait
use lazy_static::lazy_static;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;
pub const KEYBOARD_IST_INDEX: u16 = 1;

lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            // Remove unnecessary unsafe block
            let stack_start = VirtAddr::from_ptr(&raw const STACK);
            stack_start + STACK_SIZE
        };
        tss.interrupt_stack_table[KEYBOARD_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            // Remove unnecessary unsafe block
            let stack_start = VirtAddr::from_ptr(&raw const STACK);
            stack_start + STACK_SIZE
        };
        tss
    };

    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
        (gdt, Selectors {
            code_selector,
         tss_selector,
        })
    };
}

#[derive(Debug)]
struct Selectors {
    code_selector: x86_64::structures::gdt::SegmentSelector,
    tss_selector: x86_64::structures::gdt::SegmentSelector,
}

pub fn init() {
    GDT.0.load();
    unsafe {
        CS::set_reg(GDT.1.code_selector);
        x86_64::instructions::tables::load_tss(GDT.1.tss_selector);
    }
}

#[test_case]
fn test_gdt_load() {
    init();
}
