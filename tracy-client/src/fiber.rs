use std::ffi::CStr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// Handle for entering fiber contexts
pub struct FiberHandle {
    name: Arc<CStr>,
    entered: AtomicBool,
    leaked: AtomicBool,
}

impl FiberHandle {
    pub(crate) fn new(name: Arc<CStr>) -> Self {
        Self {
            name,
            entered: false.into(),
            leaked: false.into(),
        }
    }

    /// Enter the fiber
    pub fn enter(&self) {
        if !self.entered.swap(true, Ordering::Relaxed) {
            if !self.leaked.swap(true, Ordering::Relaxed) {
                unsafe {
                    Arc::increment_strong_count(self.name.as_ptr());
                }
            }

            unsafe {
                crate::sys::___tracy_fiber_enter(self.name.as_ptr());
            }
        }
    }

    /// Leave the fiber
    pub fn leave(&self) {
        if self.entered.swap(false, Ordering::Relaxed) {
            unsafe {
                crate::sys::___tracy_fiber_leave();
            }
        }
    }
}

impl Drop for FiberHandle {
    fn drop(&mut self) {
        self.leave();
    }
}
