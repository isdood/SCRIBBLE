// lib/unstable_matter/src/arch/mod.rs
// This is the only place we declare the x86_64 module

// lib/unstable_matter/src/arch/x86_64/mod.rs
/// x86_64 Architecture Support
/// Last Updated: 2025-01-12 20:42:52 UTC
/// Author: Caleb J.D. Terkovics (isdood)

pub mod structures {
    pub mod gdt {
        #[derive(Debug)]
        pub struct GlobalDescriptorTable {
            table: [u64; 8],
            next_selector: u16,
        }

        impl GlobalDescriptorTable {
            pub fn new() -> Self {
                Self {
                    table: [0; 8],
                    next_selector: 8,
                }
            }

            pub fn add_entry(&mut self, entry: Descriptor) -> SegmentSelector {
                let selector = self.next_selector;
                self.table[(selector / 8) as usize] = entry.0;
                self.next_selector += 8;
                SegmentSelector::new(selector)
            }
        }

        #[derive(Debug, Clone, Copy)]
        pub struct Descriptor(u64);

        impl Descriptor {
            pub const fn kernel_code() -> Self {
                Self(0x00AF9B000000FFFF)
            }

            pub const fn kernel_data() -> Self {
                Self(0x00CF93000000FFFF)
            }

            pub const fn user_code() -> Self {
                Self(0x00AFFA000000FFFF)
            }

            pub const fn user_data() -> Self {
                Self(0x00CFF2000000FFFF)
            }
        }

        #[derive(Debug, Clone, Copy)]
        pub struct SegmentSelector(pub u16);

        impl SegmentSelector {
            pub const fn new(index: u16) -> Self {
                Self(index)
            }
        }
    }

    pub mod tss {
        #[derive(Debug)]
        pub struct TaskStateSegment {
            pub rsp: [u64; 3],
            reserved1: u64,
            pub ist: [u64; 7],
            reserved2: u64,
            reserved3: u16,
            pub iomap_base: u16,
        }

        impl TaskStateSegment {
            pub const fn new() -> Self {
                Self {
                    rsp: [0; 3],
                    reserved1: 0,
                    ist: [0; 7],
                    reserved2: 0,
                    reserved3: 0,
                    iomap_base: 0,
                }
            }
        }
    }

    pub mod idt {
        #[derive(Debug, Clone, Copy)]
        #[repr(C)]
        pub struct InterruptStackFrame {
            pub instruction_pointer: u64,
            pub code_segment: u64,
            pub cpu_flags: u64,
            pub stack_pointer: u64,
            pub stack_segment: u64,
        }
    }
}

pub mod instructions {
    pub mod port {
        use core::marker::PhantomData;

        pub struct Port<T> {
            port: u16,
            phantom: PhantomData<T>,
        }

        impl<T> Port<T> {
            pub const fn new(port: u16) -> Port<T> {
                Port {
                    port,
                    phantom: PhantomData,
                }
            }
        }

        impl Port<u8> {
            pub unsafe fn read(&self) -> u8 {
                let value: u8;
                core::arch::asm!("in al, dx", out("al") value, in("dx") self.port, options(nomem, nostack, preserves_flags));
                value
            }

            pub unsafe fn write(&mut self, value: u8) {
                core::arch::asm!("out dx, al", in("dx") self.port, in("al") value, options(nomem, nostack, preserves_flags));
            }
        }
    }

    pub mod interrupts {
        pub fn without_interrupts<F, R>(f: F) -> R
        where
        F: FnOnce() -> R
        {
            let flags = super::super::registers::rflags::read();
            unsafe {
                core::arch::asm!("cli", options(nomem, nostack));
            }
            let result = f();
            if flags & (1 << 9) != 0 {
                unsafe {
                    core::arch::asm!("sti", options(nomem, nostack));
                }
            }
            result
        }
    }

    pub mod segmentation {
        pub trait Segment {
            fn get_reg() -> u16;
        }

        pub struct CS;
        impl Segment for CS {
            fn get_reg() -> u16 {
                let segment: u16;
                unsafe {
                    core::arch::asm!("mov {0:x}, cs", out(reg) segment, options(nomem, nostack, preserves_flags));
                }
                segment
            }
        }
    }

    pub mod tables {
        pub unsafe fn load_tss(selector: u16) {
            core::arch::asm!("ltr {0:x}", in(reg) selector, options(nomem, nostack, preserves_flags));
        }
    }
}

pub mod registers {
    pub mod rflags {
        pub fn read() -> u64 {
            let flags: u64;
            unsafe {
                core::arch::asm!("pushfq; pop {}", out(reg) flags, options(nomem, preserves_flags));
            }
            flags
        }
    }
}
