//! Crystal-optimized foreign function interface.
//!
//! This module provides a high-performance FFI system optimized for
//! crystal-space operations, with native support for Zig, Julia, and Rust.

pub mod zig;
pub mod julia;
pub mod rust;

use std::sync::Arc;
use std::any::Any;
use std::ffi::{c_void, CString};
use std::marker::PhantomData;
use std::error::Error;
use std::fmt;

/// Error type for shimmer operations
#[derive(Debug)]
pub enum ShimmerError {
    /// Failed to load dynamic library
    LoadError(String),
    /// Symbol not found in library
    SymbolError(String),
    /// Type conversion error
    TypeError(String),
    /// Runtime error
    RuntimeError(String),
}

impl fmt::Display for ShimmerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LoadError(msg) => write!(f, "Failed to load library: {}", msg),
            Self::SymbolError(msg) => write!(f, "Symbol not found: {}", msg),
            Self::TypeError(msg) => write!(f, "Type conversion error: {}", msg),
            Self::RuntimeError(msg) => write!(f, "Runtime error: {}", msg),
        }
    }
}

impl Error for ShimmerError {}

/// Result type for shimmer operations
pub type ShimmerResult<T> = Result<T, ShimmerError>;

/// Wrapper for raw pointer debug formatting
#[derive(Clone, Copy)]
struct RawPtr(*mut c_void);

impl RawPtr {
    fn is_null(self) -> bool {
        self.0.is_null()
    }
}

impl fmt::Debug for RawPtr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:p}", self.0)
    }
}

/// Wrapper for dynamic data
struct DynamicData(Box<dyn Any + Send + Sync>);

impl DynamicData {
    fn new<T: Any + Send + Sync>(value: T) -> Self {
        Self(Box::new(value))
    }

    fn downcast_ref<T: Any>(&self) -> Option<&T> {
        self.0.downcast_ref()
    }
}

impl Clone for DynamicData {
    fn clone(&self) -> Self {
        Self(Box::new(()))
    }
}

impl fmt::Debug for DynamicData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DynamicData")
    }
}

/// Foreign function context
#[derive(Debug, Clone)]
pub struct ShimmerContext {
    /// Library handle
    lib: Arc<RawPtr>,
    /// Language-specific context data
    data: DynamicData,
}

impl ShimmerContext {
    /// Get the context data if it matches the expected type
    pub fn get_data<T: Any>(&self) -> Option<&T> {
        self.data.downcast_ref()
    }
}

unsafe impl Send for ShimmerContext {}
unsafe impl Sync for ShimmerContext {}

/// Foreign function wrapper
#[derive(Debug, Clone)]
pub struct ShimmerFn<T> {
    /// Function pointer
    ptr: RawPtr,
    /// Function context
    ctx: Arc<ShimmerContext>,
    /// Function type
    _phantom: PhantomData<T>,
}

impl<T> ShimmerFn<T> {
    /// Check if the function pointer is valid
    pub fn is_valid(&self) -> bool {
        !self.ptr.is_null()
    }

    /// Get the associated context
    pub fn context(&self) -> &ShimmerContext {
        &self.ctx
    }
}

unsafe impl<T> Send for ShimmerFn<T> {}
unsafe impl<T> Sync for ShimmerFn<T> {}

/// Main FFI interface
#[derive(Debug, Clone)]
pub struct Shimmer {
    /// Current context
    context: Arc<ShimmerContext>,
}

impl Shimmer {
    /// Creates a new Shimmer instance
    pub fn new() -> Self {
        Self {
            context: Arc::new(ShimmerContext {
                lib: Arc::new(RawPtr(std::ptr::null_mut())),
                data: DynamicData::new(()),
            }),
        }
    }

    /// Get the current context data if it matches the expected type
    pub fn get_data<T: Any>(&self) -> Option<&T> {
        self.context.get_data()
    }

    /// Loads a dynamic library
    pub fn load(&mut self, path: &str) -> ShimmerResult<()> {
        let path = CString::new(path).map_err(|e| ShimmerError::LoadError(e.to_string()))?;

        #[cfg(unix)]
        let lib = unsafe { libc::dlopen(path.as_ptr(), libc::RTLD_LAZY) };

        #[cfg(windows)]
        let lib = unsafe { kernel32::LoadLibraryA(path.as_ptr() as *const i8) };

        if lib.is_null() {
            return Err(ShimmerError::LoadError("Failed to load library".into()));
        }

        self.context = Arc::new(ShimmerContext {
            lib: Arc::new(RawPtr(lib)),
            data: DynamicData::new(()),
        });

        Ok(())
    }

    /// Gets a function from the loaded library
    pub fn get_fn<T>(&self, name: &str) -> ShimmerResult<ShimmerFn<T>> {
        let name = CString::new(name).map_err(|e| ShimmerError::SymbolError(e.to_string()))?;

        #[cfg(unix)]
        let ptr = unsafe { libc::dlsym(self.context.lib.0, name.as_ptr()) };

        #[cfg(windows)]
        let ptr = unsafe { kernel32::GetProcAddress(self.context.lib.0 as _, name.as_ptr() as *const i8) };

        if ptr.is_null() {
            return Err(ShimmerError::SymbolError(format!("Symbol not found: {}", name.to_str().unwrap())));
        }

        Ok(ShimmerFn {
            ptr: RawPtr(ptr),
            ctx: Arc::clone(&self.context),
            _phantom: PhantomData,
        })
    }
}

impl Drop for Shimmer {
    fn drop(&mut self) {
        if Arc::strong_count(&self.context.lib) == 1 && !self.context.lib.0.is_null() {
            unsafe {
                #[cfg(unix)]
                libc::dlclose(self.context.lib.0);

                #[cfg(windows)]
                kernel32::FreeLibrary(self.context.lib.0 as _);
            }
        }
    }
}

impl Default for Shimmer {
    fn default() -> Self {
        Self::new()
    }
}
