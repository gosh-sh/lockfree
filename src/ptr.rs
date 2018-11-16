use std::ptr::NonNull;

static _NON_NULL: u8 = /* dummy value */ 1;

// don't use with bit flags
#[inline(always)]
pub fn non_zero_null<T>() -> NonNull<T> {
    NonNull::from(&_NON_NULL).cast()
}

#[inline(always)]
pub unsafe fn bypass_null<T>(ptr: *mut T) -> NonNull<T> {
    debug_assert!(!ptr.is_null());
    NonNull::new_unchecked(ptr)
}
