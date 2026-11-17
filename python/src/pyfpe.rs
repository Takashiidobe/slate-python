//! These variables used to be used when Python was built with --with-fpectl,
//! but support for that was dropped in 3.7. We continue to define them,
//! though, because they may be referenced by extensions using the stable ABI.
#[unsafe(no_mangle)]
static mut PyFPE_counter: i32 = unsafe { core::mem::zeroed() };

#[unsafe(no_mangle)]
extern "C-unwind" fn PyFPE_dummy(dummy: *mut ()) -> f64 {
    1.0f64
}
