//! Memory pool implementation for SIMD operations
//! Created: 2025-01-21 23:43:42 UTC
//! Author: isdood

use std::alloc::{alloc, dealloc, Layout};
use std::ptr::NonNull;
use std::sync::Arc;
use parking_lot::RwLock;
use crate::superpurple::core::LatticeSymmetry;

/// Memory alignment for different symmetry types
#[derive(Debug, Clone, Copy)]
pub enum SymmetryAlignment {
    Cubic = 32,      // AVX-512 alignment
    Tetragonal = 16, // AVX2 alignment
    Hexagonal = 32,  // Custom alignment
    Custom = 16,     // Default alignment
}

/// SIMD-optimized memory pool
pub struct CLSIMDMemoryPool {
    /// Memory blocks organized by symmetry type
    blocks: Arc<RwLock<Vec<MemoryBlock>>>,
    /// Alignment settings
    alignment: SymmetryAlignment,
    /// Total allocated size
    total_size: usize,
    /// Maximum pool size
    max_size: usize,
}

/// Memory block structure
#[derive(Debug)]
struct MemoryBlock {
    /// Raw pointer to aligned memory
    ptr: NonNull<u8>,
    /// Block size
    size: usize,
    /// Memory alignment
    alignment: SymmetryAlignment,
    /// Whether block is in use
    in_use: bool,
}

impl CLSIMDMemoryPool {
    /// Create new memory pool with specified maximum size
    pub fn new(max_size: usize) -> Self {
        Self {
            blocks: Arc::new(RwLock::new(Vec::new())),
            alignment: SymmetryAlignment::Cubic,
            total_size: 0,
            max_size,
        }
    }

    /// Allocate memory with specific symmetry alignment
    pub fn allocate(&mut self, size: usize, symmetry: LatticeSymmetry) -> Option<NonNull<u8>> {
        let alignment = match symmetry {
            LatticeSymmetry::Cubic => SymmetryAlignment::Cubic,
            LatticeSymmetry::Tetragonal => SymmetryAlignment::Tetragonal,
            LatticeSymmetry::Hexagonal => SymmetryAlignment::Hexagonal,
            LatticeSymmetry::Custom(_) => SymmetryAlignment::Custom,
        };

        // Check if we can reuse an existing block
        if let Some(block) = self.find_free_block(size, alignment) {
            return Some(block.ptr);
        }

        // Allocate new block if within size limits
        if self.total_size + size <= self.max_size {
            self.allocate_new_block(size, alignment)
        } else {
            None
        }
    }

    /// Find existing free block with appropriate size and alignment
    fn find_free_block(&mut self, size: usize, alignment: SymmetryAlignment) -> Option<&MemoryBlock> {
        let blocks = self.blocks.read();
        blocks.iter().find(|block| {
            !block.in_use &&
            block.size >= size &&
            block.alignment as usize >= alignment as usize
        })
    }

    /// Allocate new memory block
    fn allocate_new_block(&mut self, size: usize, alignment: SymmetryAlignment) -> Option<NonNull<u8>> {
        let layout = Layout::from_size_align(
            size,
            alignment as usize
        ).ok()?;

        unsafe {
            let ptr = alloc(layout);
            if ptr.is_null() {
                return None;
            }

            let ptr = NonNull::new(ptr)?;
            let block = MemoryBlock {
                ptr,
                size,
                alignment,
                in_use: true,
            };

            self.blocks.write().push(block);
            self.total_size += size;
            Some(ptr)
        }
    }

    /// Deallocate memory block
    pub unsafe fn deallocate(&mut self, ptr: NonNull<u8>) {
        let mut blocks = self.blocks.write();
        if let Some(block) = blocks.iter_mut().find(|b| b.ptr == ptr) {
            block.in_use = false;
        }
    }
}

impl Drop for CLSIMDMemoryPool {
    fn drop(&mut self) {
        let blocks = self.blocks.read();
        for block in blocks.iter() {
            unsafe {
                let layout = Layout::from_size_align_unchecked(
                    block.size,
                    block.alignment as usize
                );
                dealloc(block.ptr.as_ptr(), layout);
            }
        }
    }
}
