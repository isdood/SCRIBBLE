// lib/unstable_matter/src/arch/mod.rs
pub mod x86_64;

// lib/unstable_matter/src/arch/x86_64/mod.rs
/// x86_64 Quantum Architecture Support
/// Last Updated: 2025-01-14 21:38:36 UTC
/// Author: isdood
/// Current User: isdood

use crate::{
    helium::Helium,
    phantom::QuantumCell,
    unstable::UnstableDescriptor,
    zeronaut::Zeronaut,
};

pub mod structures {
    use super::*;

    pub mod gdt {
        use super::*;

        #[derive(Debug)]
        pub struct GlobalDescriptorTable {
            table: QuantumCell<[u64; 8]>,
            next_selector: Helium<u16>,
            state: UnstableDescriptor,
        }

        impl GlobalDescriptorTable {
            pub fn new() -> Self {
                Self {
                    table: QuantumCell::new([0; 8]),
                    next_selector: Helium::new(8),
                    state: UnstableDescriptor::new(),
                }
            }

            pub fn add_entry(&mut self, entry: Descriptor) -> SegmentSelector {
                let selector = self.next_selector.quantum_load();
                let mut table = *self.table.get();
                table[(selector / 8) as usize] = entry.0;
                self.table.set(table);
                self.next_selector.quantum_store(selector + 8);
                SegmentSelector::new(selector)
            }
        }

        #[derive(Debug, Clone, Copy)]
        pub struct Descriptor(pub u64);

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

            pub const fn quantum_segment() -> Self {
                Self(0x00AFFF000000FFFF)
            }
        }

        #[derive(Debug, Clone, Copy)]
        pub struct SegmentSelector(pub Helium<u16>);

        impl SegmentSelector {
            pub const fn new(index: u16) -> Self {
                Self(Helium::new(index))
            }
        }
    }

    pub mod tss {
        use super::*;

        #[derive(Debug)]
        pub struct TaskStateSegment {
            pub rsp: QuantumCell<[u64; 3]>,
            reserved1: Helium<u64>,
            pub ist: QuantumCell<[u64; 7]>,
            reserved2: Helium<u64>,
            reserved3: Helium<u16>,
            pub iomap_base: Helium<u16>,
            state: UnstableDescriptor,
        }

        impl TaskStateSegment {
            pub const fn new() -> Self {
                Self {
                    rsp: QuantumCell::new([0; 3]),
                    reserved1: Helium::new(0),
                    ist: QuantumCell::new([0; 7]),
                    reserved2: Helium::new(0),
                    reserved3: Helium::new(0),
                    iomap_base: Helium::new(0),
                    state: UnstableDescriptor::new(),
                }
            }
        }
    }

    pub mod idt {
        use super::*;

        #[derive(Debug, Clone, Copy)]
        #[repr(C)]
        pub struct InterruptStackFrame {
            pub instruction_pointer: Helium<u64>,
            pub code_segment: Helium<u64>,
            pub cpu_flags: Helium<u64>,
            pub stack_pointer: Helium<u64>,
            pub stack_segment: Helium<u64>,
        }
    }
}

pub mod instructions {
    use super::*;

    pub mod port {
        use super::*;
        use core::marker::PhantomData;

        pub struct Port<T> {
            port: Helium<u16>,
            phantom: PhantomData<T>,
            state: UnstableDescriptor,
        }

        impl<T> Port<T> {
            pub const fn new(port: u16) -> Port<T> {
                Port {
                    port: Helium::new(port),
                    phantom: PhantomData,
                    state: UnstableDescriptor::new(),
                }
            }
        }

        impl Port<u8> {
            pub unsafe fn read(&self) -> u8 {
                let port = self.port.quantum_load();
                let value: u8;
                core::arch::asm!("in al, dx", out("al") value, in("dx") port, options(nomem, nostack, preserves_flags));
                value
            }

            pub unsafe fn write(&mut self, value: u8) {
                let port = self.port.quantum_load();
                core::arch::asm!("out dx, al", in("dx") port, in("al") value, options(nomem, nostack, preserves_flags));
            }
        }
    }

    pub mod interrupts {
        use super::*;

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
        use super::*;

        pub trait Segment {
            fn get_reg() -> Helium<u16>;
        }

        pub struct CS;
        impl Segment for CS {
            fn get_reg() -> Helium<u16> {
                let segment: u16;
                unsafe {
                    core::arch::asm!("mov {0:x}, cs", out(reg) segment, options(nomem, nostack, preserves_flags));
                }
                Helium::new(segment)
            }
        }
    }

    pub mod tables {
        use super::*;

        pub unsafe fn load_tss(selector: Helium<u16>) {
            core::arch::asm!("ltr {0:x}", in(reg) selector.quantum_load(), options(nomem, nostack, preserves_flags));
        }
    }
}

pub mod registers {
    use super::*;

    pub mod rflags {
        use super::*;

        pub fn read() -> Helium<u64> {
            let flags: u64;
            unsafe {
                core::arch::asm!("pushfq; pop {}", out(reg) flags, options(nomem, preserves_flags));
            }
            Helium::new(flags)
        }
    }
}
