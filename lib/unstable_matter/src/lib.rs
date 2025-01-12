#![no_std]

extern crate alloc;

mod unstable_matter;
pub mod unstable_vectrix;

pub use unstable_matter::UnstableMatter;
pub use unstable_vectrix::UnstableVectrix;

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec::Vec;
    use alloc::vec; // Import the `vec!` macro

    #[test]
    fn test_wrapper_read_write() {
        let size = 10;
        let offset = 0;

        // Use a vector to simulate memory
        let mut mem = vec![0u8; size];
        let base_addr = mem.as_mut_ptr() as usize;

        // Create a new Wrapper instance
        let mut vector = unsafe { Wrapper::new(base_addr, size, offset) };

        // Write some values
        for i in 0..size {
            vector.write(i, i as u8);
        }

        // Read back the values
        for i in 0..size {
            let value = vector.read(i);
            assert_eq!(value, i as u8);
        }
    }

    #[test]
    fn test_wrapper_move_to() {
        let size = 10;
        let offset = 0;

        // Use a vector to simulate memory
        let mut mem = vec![0u8; size];
        let base_addr = mem.as_mut_ptr() as usize;

        // Create a new Wrapper instance
        let mut vector = unsafe { Wrapper::new(base_addr, size, offset) };

        // Write some values
        for i in 0..size {
            vector.write(i, i as u8);
        }

        // Use another vector to simulate new memory
        let mut new_mem = vec![0u8; size];
        let new_addr = new_mem.as_mut_ptr() as usize;

        // Move to a new address
        vector.move_to(new_addr);

        // Write new values at the new address
        for i in 0..size {
            vector.write(i, (i + 10) as u8);
        }

        // Read back the new values
        for i in 0..size {
            let value = vector.read(i);
            assert_eq!(value, (i + 10) as u8);
        }
    }
}
