// TODO: It is possible to make this crate `no_std`, but we need to figure out how to work around
//  doc tests. Maybe just wait for `cfg(doctest)` to be stabilized.
// #![no_std]

extern crate alloc;

use alloc::alloc::{alloc as alloc_raw, handle_alloc_error, Layout};
use alloc::boxed::Box;
use core::ptr;

pub use default_boxed_derive::DefaultBoxed;

/// Helper trait to create a boxed instance of the given type with a default value for each field.
///
/// This trait can be derived for structs.
///
/// To derive this trait, each field needs to also implement this trait, but all types which
/// implements `Default` implements this trait via the blanket `impl` already.
///
/// In addition, if a field is an array, only the item type needs to implement this trait, and each
/// item would be initialized separately.
pub trait DefaultBoxed {
    /// Create a boxed instance with default value for each field.
    fn default_boxed() -> Box<Self>
    where
        Self: Sized,
    {
        let layout = Layout::new::<Self>();
        unsafe {
            if layout.size() == 0 {
                return Box::from_raw(ptr::NonNull::<Self>::dangling().as_ptr());
            }
            let raw = alloc_raw(layout) as *mut Self;
            if raw.is_null() {
                handle_alloc_error(layout)
            } else {
                Self::default_in_place(raw);
                Box::from_raw(raw)
            }
        }
    }

    /// Fill the given memory location with default value.
    ///
    /// # Safety
    ///
    /// For callers, behavior is undefined if `ptr` is not valid for writes, or it is not properly
    /// aligned.
    ///
    /// For impls, behavior is undefined if this method reads from `ptr`.
    unsafe fn default_in_place(ptr: *mut Self);
}

impl<T: Default> DefaultBoxed for T {
    unsafe fn default_in_place(ptr: *mut Self) {
        ptr::write(ptr, Default::default());
    }
}
