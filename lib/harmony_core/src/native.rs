//! Native Types and Macros
//! =======================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-19 17:52:37 UTC
//! Version: 0.1.0
//! License: MIT

pub struct String {
    inner: Vec<u8>,
}

impl String {
    pub fn new() -> Self {
        String { inner: Vec::new() }
    }

    pub fn push_str(&mut self, s: &str) {
        self.inner.extend_from_slice(s.as_bytes());
    }
}

impl core::fmt::Debug for String {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", core::str::from_utf8(&self.inner).unwrap())
    }
}

pub struct Box<T> {
    value: *mut T,
}

impl<T> Box<T> {
    pub fn new(value: T) -> Self {
        // Allocate memory for the value
        let mut uninit = core::mem::MaybeUninit::uninit();
        unsafe {
            // Write the value into the uninitialized memory
            uninit.as_mut_ptr().write(value);
            Self {
                value: uninit.as_mut_ptr(),
            }
        }
    }

    pub fn into_inner(self) -> T {
        unsafe { *self.value }
    }
}

#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = $crate::native::Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

pub struct Vec<T> {
    data: std::vec::Vec<T>,
}

impl<T> Vec<T> {
    pub fn new() -> Self {
        Vec { data: std::vec::Vec::new() }
    }

    pub fn push(&mut self, value: T) {
        self.data.push(value);
    }

    pub fn into_boxed_slice(self) -> Box<[T]> {
        Box::new(self.data.into_boxed_slice())
    }
}

impl<T> core::ops::Deref for Box<[T]> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.value }
    }
}
