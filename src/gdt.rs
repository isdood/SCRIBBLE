use x86_64::structures::tss::TaskStateSegment;
use x86_64::structures::gdt::{GlobalDescriptorTable, Descriptor, SegmentSelector};
use lazy_static::lazy_static;
use x86_64::instructions::segmentation::Segment;

lazy_static! {
    static ref TSS: TaskStateSegment = {
        let tss = TaskStateSegment::new();
        // Initialize TSS here...
        tss
    };
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
        (gdt, Selectors { code_selector, tss_selector })
    };
}

struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

pub fn init() {
    GDT.0.load();
    unsafe {
        x86_64::instructions::segmentation::CS::set_reg(GDT.1.code_selector);
        x86_64::instructions::tables::load_tss(GDT.1.tss_selector);
    }
}
