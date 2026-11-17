//! Memoryview object implementation
//!
//!   This implementation is a complete rewrite contributed by Stefan Krah in
//!   Python 3.3.  Substantial credit goes to Antoine Pitrou (who had already
//!   fortified and rewritten the previous implementation) and Nick Coghlan
//!   (who came up with the idea of the ManagedBuffer) for analyzing the complex
//!   ownership rules.
//! The size of 'void *', as computed by sizeof.
//! PyObject_HEAD defines the initial segment of every PyObject.
//! In 64+ bit systems, any object whose 32 bit reference count is >= 2**31
//! will be treated as immortal.
//!
//! Using the lower 32 bits makes the value backwards compatible by allowing
//! C-Extensions without the updated checks in Py_INCREF and Py_DECREF to safely
//! increase and decrease the objects reference count.
//!
//! In order to offer sufficient resilience to C extensions using the stable ABI
//! compiled against 3.11 or earlier, we set the initial value near the
//! middle of the range (2**31, 2**32). That way the refcount can be
//! off by ~1 billion without affecting immortality.
//!
//! Reference count increases will use saturated arithmetic, taking advantage of
//! having all the lower 32 bits set, which will avoid the reference count to go
//! beyond the refcount limit. Immortality checks for reference count decreases will
//! be done by checking the bit sign flag in the lower 32 bits.
//!
//! To ensure that once an object becomes immortal, it remains immortal, the threshold
//! for omitting increfs is much higher than for omitting decrefs. Consequently, once
//! the refcount for an object exceeds _Py_IMMORTAL_MINIMUM_REFCNT it will gradually
//! increase over time until it reaches _Py_IMMORTAL_INITIAL_REFCNT.
//! Macros and static inline functions, trading safety for speed
//! PyException_HEAD defines the initial segment of every exception class.
//! The structs are declared here so that macros can work, but they shouldn't
//! be considered public. Don't access their fields directly, use the macros
//! and functions instead!
//! Cast a function to the PyCFunction type to use it with PyMethodDef.
//!
//!
//!
//! This macro can be used to prevent compiler warnings if the first parameter
//!
//! uses a different pointer type than PyObject* (ex: METH_VARARGS and METH_O
//!
//! calling conventions).
//!
//!
//!
//! The macro can also be used for METH_FASTCALL and METH_VARARGS|METH_KEYWORDS
//!
//! calling conventions to avoid compiler warnings because the function has more
//!
//! than 2 parameters. The macro first casts the function to the
//!
//! "void func(void)" type to prevent compiler warnings.
//!
//!
//!
//! If a function is declared with the METH_NOARGS calling convention, it must
//!
//! have 2 parameters. Since the second parameter is unused, Py_UNUSED() can be
//!
//! used to prevent a compiler warning. If the function has a single parameter,
//!
//! it triggers an undefined behavior when Python calls it with 2 parameters
//!
//! (bpo-33012).
//! A code object, the __code__ attribute
//! This limitation is for performance and simplicity. If needed it can be
//! removed (with effort).
//! Remote debugger support
//! PY_TIMEOUT_T is the integral type used to specify timeouts when waiting
//!
//! on a lock (see PyThread_acquire_lock_timed() below).
//! Chosen so that _PyObjectStackChunk is a power-of-two size.
//! Largest tuple to save on freelist
//! leads to warnings... __attribute__((cdecl))
//! Various defines for all used memory orders in mimalloc
//! Main tuning parameters for segment and page sizes
//!
//! Sizes for 64-bit (usually divide by two for 32-bit)
//! Inlined definitions
//! Total tool ids available
//! Fast inlined version of PyType_IS_GC()
// Cast a function to another function type T.
//
//
//
// The macro first casts the function to the "void func(void)" type
//
// to prevent compiler warnings.
//
//
//
// Note that using this cast only prevents the compiler from emitting
//
// warnings, but does not prevent an undefined behavior at runtime if
//
// the original function signature is not respected.
// Signed variants of the above
// Convert the argument to a string. For example, Py_STRINGIFY(123) is replaced
// with "123" by the preprocessor. Defines are also replaced by their value.
// For example Py_STRINGIFY(__LINE__) is replaced by the line number, not
// by "__LINE__".
// Define macros for inline documentation.
// Round up size "n" to be a multiple of "a".
// Prevent using an expression as a l-value.
//
// For example, "int x; _Py_RVALUE(x) = 1;" fails with a compiler error.
// 64KiB  (32KiB on 32-bit)
// 1024
// Used as a special value to encode block sizes in 32 bits.
// 35 == mi_segment_bin(MI_SLICES_PER_SEGMENT)
// initialize a local variable to zero; use memset as compilers optimize constant sized memset's
/// Adjust ptr if suboffsets are present.
macro_rules! ADJUST_PTR__2 {
    ($ptr:expr, $suboffsets:expr, $dim:expr) => {
        if HAVE_PTR!($suboffsets, $dim) {
            unsafe { *(($ptr) as *mut *mut i8) }.wrapping_offset(
                (unsafe { *$suboffsets.wrapping_offset(($dim) as isize) }) as isize,
            )
        } else {
            $ptr
        }
    };
}

/// Adjust ptr if suboffsets are present.
macro_rules! ADJUST_PTR__3 {
    ($ptr:expr, $suboffsets:expr, $dim:expr) => {
        if HAVE_PTR__2!($suboffsets, $dim) {
            unsafe { *(($ptr) as *mut *mut i8) }.wrapping_offset(
                (unsafe { *unsafe { $suboffsets }.wrapping_offset(($dim) as isize) }) as isize,
            )
        } else {
            $ptr
        }
    };
}

/// Adjust ptr if suboffsets are present.
macro_rules! ADJUST_PTR {
    ($ptr:expr, $suboffsets:expr, $dim:expr) => {
        if HAVE_PTR!($suboffsets, $dim) {
            unsafe { *(($ptr) as *mut *mut i8) }.wrapping_offset(
                (unsafe { *$suboffsets.wrapping_offset(($dim) as isize) }) as isize,
            ) as *const i8
        } else {
            $ptr
        }
    };
}

/// In the process of breaking reference cycles mbuf_release() can be
/// called before memory_release().
macro_rules! BASE_INACCESSIBLE {
    ($mv:expr) => {
        (unsafe { (*$mv).flags }) & 1 != 0 || (unsafe { (*(*$mv).mbuf).flags }) & 1 != 0
    };
}

/// In the process of breaking reference cycles mbuf_release() can be
/// called before memory_release().
macro_rules! BASE_INACCESSIBLE__2 {
    ($mv:expr) => {
        (unsafe { (*(($mv) as *mut __SlateRecord_memoryobject_h_36)).flags }) & 1 != 0
            || (unsafe { (*(*(($mv) as *mut __SlateRecord_memoryobject_h_36)).mbuf).flags }) & 1
                != 0
    };
}

/// See gh-92888. These macros signal that we need to check the memoryview
/// again due to possible read after frees.
macro_rules! CHECK_RELEASED_AGAIN {
    ($mv:expr) => {{
        CHECK_RELEASED!($mv);
    }};
}

macro_rules! CHECK_RELEASED_INT_AGAIN {
    ($mv:expr) => {{
        CHECK_RELEASED_INT!($mv);
    }};
}

macro_rules! CHECK_RELEASED_INT__2 {
    ($mv:expr) => {{
        if BASE_INACCESSIBLE!($mv) {
            unsafe {
                PyErr_SetString(
                    PyExc_ValueError,
                    c"operation forbidden on released memoryview object".as_ptr(),
                )
            };
            return 1i32.wrapping_neg() as i64;
        }
    }};
}

macro_rules! CHECK_RELEASED_INT {
    ($mv:expr) => {{
        if BASE_INACCESSIBLE!($mv) {
            unsafe {
                PyErr_SetString(
                    PyExc_ValueError,
                    c"operation forbidden on released memoryview object".as_ptr(),
                )
            };
            return 1i32.wrapping_neg();
        }
    }};
}

macro_rules! CHECK_RELEASED__2 {
    ($mv:expr) => {{
        if BASE_INACCESSIBLE__2!($mv) {
            unsafe {
                PyErr_SetString(
                    PyExc_ValueError,
                    c"operation forbidden on released memoryview object".as_ptr(),
                )
            };
            return core::ptr::null_mut::<_object>();
        }
    }};
}

macro_rules! CHECK_RELEASED {
    ($mv:expr) => {{
        if BASE_INACCESSIBLE!($mv) {
            unsafe {
                PyErr_SetString(
                    PyExc_ValueError,
                    c"operation forbidden on released memoryview object".as_ptr(),
                )
            };
            return core::ptr::null_mut::<_object>();
        }
    }};
}

macro_rules! CHECK_RESTRICTED_INT {
    ($mv:expr) => {{
        if (unsafe { (*$mv).flags }) & 32 != 0 {
            unsafe {
                PyErr_SetString(
                    PyExc_ValueError,
                    c"cannot create new view on restricted memoryview".as_ptr(),
                )
            };
            return 1i32.wrapping_neg();
        }
    }};
}

macro_rules! CHECK_RESTRICTED {
    ($mv:expr) => {{
        if (unsafe { (*$mv).flags }) & 32 != 0 {
            unsafe {
                PyErr_SetString(
                    PyExc_ValueError,
                    c"cannot create new view on restricted memoryview".as_ptr(),
                )
            };
            return core::ptr::null_mut::<_object>();
        }
    }};
}

/// Unpack and compare single items of p and q. If both p and q have the same
/// single element native format, the comparison uses a fast path (gcc creates
/// a jump table and converts memcpy into simple assignments on x86/x64).
///
/// Otherwise, the comparison is delegated to the struct module, which is
/// 30-60x slower.
macro_rules! CMP_SINGLE__3 {
    ($p:expr, $q:expr, $type:ty, $equal:expr) => {
        loop {
            let mut x: $type = 0;
            let mut y: $type = 0;
            unsafe {
                core::ptr::copy_nonoverlapping(
                    ((($p) as *const ()) as *const core::ffi::c_void) as *const u8,
                    (((core::ptr::addr_of_mut!(x) as *mut i8) as *mut ()) as *mut core::ffi::c_void)
                        as *mut u8,
                    ((core::mem::size_of::<$type>() as u64) as usize) as usize,
                )
            };
            unsafe {
                core::ptr::copy_nonoverlapping(
                    ((($q) as *const ()) as *const core::ffi::c_void) as *const u8,
                    (((core::ptr::addr_of_mut!(y) as *mut i8) as *mut ()) as *mut core::ffi::c_void)
                        as *mut u8,
                    ((core::mem::size_of::<$type>() as u64) as usize) as usize,
                )
            };
            $equal = ((x as i32) == (y as i32)) as i32;
            if true {
                break;
            }
        }
    };
}

/// Unpack and compare single items of p and q. If both p and q have the same
/// single element native format, the comparison uses a fast path (gcc creates
/// a jump table and converts memcpy into simple assignments on x86/x64).
///
/// Otherwise, the comparison is delegated to the struct module, which is
/// 30-60x slower.
macro_rules! CMP_SINGLE__4 {
    ($p:expr, $q:expr, $type:ty, $equal:expr) => {
        loop {
            let mut x: $type = 0 as f32;
            let mut y: $type = 0 as f32;
            unsafe {
                core::ptr::copy_nonoverlapping(
                    ((($p) as *const ()) as *const core::ffi::c_void) as *const u8,
                    (((core::ptr::addr_of_mut!(x) as *mut i8) as *mut ()) as *mut core::ffi::c_void)
                        as *mut u8,
                    ((core::mem::size_of::<$type>() as u64) as usize) as usize,
                )
            };
            unsafe {
                core::ptr::copy_nonoverlapping(
                    ((($q) as *const ()) as *const core::ffi::c_void) as *const u8,
                    (((core::ptr::addr_of_mut!(y) as *mut i8) as *mut ()) as *mut core::ffi::c_void)
                        as *mut u8,
                    ((core::mem::size_of::<$type>() as u64) as usize) as usize,
                )
            };
            $equal = (x == y) as i32;
            if true {
                break;
            }
        }
    };
}

/// Unpack and compare single items of p and q. If both p and q have the same
/// single element native format, the comparison uses a fast path (gcc creates
/// a jump table and converts memcpy into simple assignments on x86/x64).
///
/// Otherwise, the comparison is delegated to the struct module, which is
/// 30-60x slower.
macro_rules! CMP_SINGLE {
    ($p:expr, $q:expr, $type:ty, $equal:expr) => {
        loop {
            let mut x: $type = 0;
            let mut y: $type = 0;
            unsafe {
                core::ptr::copy_nonoverlapping(
                    ((($p) as *const ()) as *const core::ffi::c_void) as *const u8,
                    (((core::ptr::addr_of_mut!(x) as *mut i8) as *mut ()) as *mut core::ffi::c_void)
                        as *mut u8,
                    ((core::mem::size_of::<$type>() as u64) as usize) as usize,
                )
            };
            unsafe {
                core::ptr::copy_nonoverlapping(
                    ((($q) as *const ()) as *const core::ffi::c_void) as *const u8,
                    (((core::ptr::addr_of_mut!(y) as *mut i8) as *mut ()) as *mut core::ffi::c_void)
                        as *mut u8,
                    ((core::mem::size_of::<$type>() as u64) as usize) as usize,
                )
            };
            $equal = (x == y) as i32;
            if true {
                break;
            }
        }
    };
}

/// Unpack and compare single items of p and q. If both p and q have the same
/// single element native format, the comparison uses a fast path (gcc creates
/// a jump table and converts memcpy into simple assignments on x86/x64).
///
/// Otherwise, the comparison is delegated to the struct module, which is
/// 30-60x slower.
macro_rules! CMP_SINGLE__5 {
    ($p:expr, $q:expr, $type:ty, $equal:expr) => {
        loop {
            let mut x: $type = 0;
            let mut y: $type = 0;
            unsafe {
                core::ptr::copy_nonoverlapping(
                    ((($p) as *const ()) as *const core::ffi::c_void) as *const u8,
                    (((core::ptr::addr_of_mut!(x) as *mut i8) as *mut ()) as *mut core::ffi::c_void)
                        as *mut u8,
                    ((core::mem::size_of::<$type>() as u64) as usize) as usize,
                )
            };
            unsafe {
                core::ptr::copy_nonoverlapping(
                    ((($q) as *const ()) as *const core::ffi::c_void) as *const u8,
                    (((core::ptr::addr_of_mut!(y) as *mut i8) as *mut ()) as *mut core::ffi::c_void)
                        as *mut u8,
                    ((core::mem::size_of::<$type>() as u64) as usize) as usize,
                )
            };
            $equal = ((x as i32) == (y as i32)) as i32;
            if true {
                break;
            }
        }
    };
}

/// Unpack and compare single items of p and q. If both p and q have the same
/// single element native format, the comparison uses a fast path (gcc creates
/// a jump table and converts memcpy into simple assignments on x86/x64).
///
/// Otherwise, the comparison is delegated to the struct module, which is
/// 30-60x slower.
macro_rules! CMP_SINGLE__6 {
    ($p:expr, $q:expr, $type:ty, $equal:expr) => {
        loop {
            let mut x: $type = 0;
            let mut y: $type = 0;
            unsafe {
                core::ptr::copy_nonoverlapping(
                    ((($p) as *const ()) as *const core::ffi::c_void) as *const u8,
                    (((core::ptr::addr_of_mut!(x) as *mut i8) as *mut ()) as *mut core::ffi::c_void)
                        as *mut u8,
                    ((core::mem::size_of::<$type>() as u64) as usize) as usize,
                )
            };
            unsafe {
                core::ptr::copy_nonoverlapping(
                    ((($q) as *const ()) as *const core::ffi::c_void) as *const u8,
                    (((core::ptr::addr_of_mut!(y) as *mut i8) as *mut ()) as *mut core::ffi::c_void)
                        as *mut u8,
                    ((core::mem::size_of::<$type>() as u64) as usize) as usize,
                )
            };
            $equal = (x == y) as $type;
            if true {
                break;
            }
        }
    };
}

/// Unpack and compare single items of p and q. If both p and q have the same
/// single element native format, the comparison uses a fast path (gcc creates
/// a jump table and converts memcpy into simple assignments on x86/x64).
///
/// Otherwise, the comparison is delegated to the struct module, which is
/// 30-60x slower.
macro_rules! CMP_SINGLE__2 {
    ($p:expr, $q:expr, $type:ty, $equal:expr) => {
        loop {
            let mut x: $type = 0;
            let mut y: $type = 0;
            unsafe {
                core::ptr::copy_nonoverlapping(
                    ((($p) as *const ()) as *const core::ffi::c_void) as *const u8,
                    (((core::ptr::addr_of_mut!(x) as *mut i8) as *mut ()) as *mut core::ffi::c_void)
                        as *mut u8,
                    ((core::mem::size_of::<$type>() as u64) as usize) as usize,
                )
            };
            unsafe {
                core::ptr::copy_nonoverlapping(
                    ((($q) as *const ()) as *const core::ffi::c_void) as *const u8,
                    (((core::ptr::addr_of_mut!(y) as *mut i8) as *mut ()) as *mut core::ffi::c_void)
                        as *mut u8,
                    ((core::mem::size_of::<$type>() as u64) as usize) as usize,
                )
            };
            $equal = (x == y) as i32;
            if true {
                break;
            }
        }
    };
}

/// Unpack and compare single items of p and q. If both p and q have the same
/// single element native format, the comparison uses a fast path (gcc creates
/// a jump table and converts memcpy into simple assignments on x86/x64).
///
/// Otherwise, the comparison is delegated to the struct module, which is
/// 30-60x slower.
macro_rules! CMP_SINGLE__7 {
    ($p:expr, $q:expr, $type:ty, $equal:expr) => {
        loop {
            let mut x: $type = 0;
            let mut y: $type = 0;
            unsafe {
                core::ptr::copy_nonoverlapping(
                    ((($p) as *const ()) as *const core::ffi::c_void) as *const u8,
                    (((core::ptr::addr_of_mut!(x) as *mut i8) as *mut ()) as *mut core::ffi::c_void)
                        as *mut u8,
                    ((core::mem::size_of::<$type>() as u64) as usize) as usize,
                )
            };
            unsafe {
                core::ptr::copy_nonoverlapping(
                    ((($q) as *const ()) as *const core::ffi::c_void) as *const u8,
                    (((core::ptr::addr_of_mut!(y) as *mut i8) as *mut ()) as *mut core::ffi::c_void)
                        as *mut u8,
                    ((core::mem::size_of::<$type>() as u64) as usize) as usize,
                )
            };
            $equal = (x == y) as i32;
            if true {
                break;
            }
        }
    };
}

/// Unpack and compare single items of p and q. If both p and q have the same
/// single element native format, the comparison uses a fast path (gcc creates
/// a jump table and converts memcpy into simple assignments on x86/x64).
///
/// Otherwise, the comparison is delegated to the struct module, which is
/// 30-60x slower.
macro_rules! CMP_SINGLE__8 {
    ($p:expr, $q:expr, $type:ty, $equal:expr) => {
        loop {
            let mut x: $type = 0 as f64;
            let mut y: $type = 0 as f64;
            unsafe {
                core::ptr::copy_nonoverlapping(
                    ((($p) as *const ()) as *const core::ffi::c_void) as *const u8,
                    (((core::ptr::addr_of_mut!(x) as *mut i8) as *mut ()) as *mut core::ffi::c_void)
                        as *mut u8,
                    ((core::mem::size_of::<$type>() as u64) as usize) as usize,
                )
            };
            unsafe {
                core::ptr::copy_nonoverlapping(
                    ((($q) as *const ()) as *const core::ffi::c_void) as *const u8,
                    (((core::ptr::addr_of_mut!(y) as *mut i8) as *mut ()) as *mut core::ffi::c_void)
                        as *mut u8,
                    ((core::mem::size_of::<$type>() as u64) as usize) as usize,
                )
            };
            $equal = (x == y) as i32;
            if true {
                break;
            }
        }
    };
}

/// Check for the presence of suboffsets in the first dimension.
macro_rules! HAVE_PTR {
    ($suboffsets:expr, $dim:expr) => {
        !$suboffsets.is_null() && (unsafe { *$suboffsets.wrapping_offset(($dim) as isize) }) >= 0
    };
}

/// Check for the presence of suboffsets in the first dimension.
macro_rules! HAVE_PTR__2 {
    ($suboffsets:expr, $dim:expr) => {
        !unsafe { $suboffsets }.is_null()
            && (unsafe { *unsafe { $suboffsets }.wrapping_offset(($dim) as isize) }) >= 0
    };
}

/// Assumptions: ndim >= 1. The macro tests for a corner case that should
/// perhaps be explicitly forbidden in the PEP.
macro_rules! HAVE_SUBOFFSETS_IN_LAST_DIM {
    ($view:expr) => {
        !unsafe { (*$view).suboffsets }.is_null()
            && (unsafe {
                *unsafe { (*$view).suboffsets }
                    .wrapping_offset(unsafe { (*$view).ndim }.wrapping_sub(1) as isize)
            }) >= 0
    };
}

macro_rules! IS_BYTE_FORMAT {
    ($f:expr) => {
        (($f) as i32) == 98 || (($f) as i32) == 66 || (($f) as i32) == 99
    };
}

macro_rules! MV_ANY_CONTIGUOUS {
    ($flags:expr) => {
        ($flags) & (8 | 2 | 4)
    };
}

/// Fast contiguity test. Caller must ensure suboffsets==NULL and ndim==1.
macro_rules! MV_CONTIGUOUS_NDIM1 {
    ($view:expr) => {
        (unsafe { *unsafe { (*$view).shape }.wrapping_offset(0 as isize) }) == 1
            || (unsafe { *unsafe { (*$view).strides }.wrapping_offset(0 as isize) })
                == unsafe { (*$view).itemsize }
    };
}

/// Memoryview buffer properties
macro_rules! MV_C_CONTIGUOUS {
    ($flags:expr) => {
        ($flags) & (8 | 2)
    };
}

macro_rules! MV_F_CONTIGUOUS {
    ($flags:expr) => {
        ($flags) & (8 | 4)
    };
}

macro_rules! PACK_SINGLE__3 {
    ($ptr:expr, $src:expr, $type:ty) => {
        loop {
            let mut x: $type = ($src) as $type;
            unsafe {
                core::ptr::copy_nonoverlapping(
                    (((core::ptr::addr_of_mut!(x) as *mut i8) as *const ())
                        as *const core::ffi::c_void) as *const u8,
                    ((($ptr) as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                    ((core::mem::size_of::<$type>() as u64) as usize) as usize,
                )
            };
            if true {
                break;
            }
        }
    };
}

macro_rules! PACK_SINGLE {
    ($ptr:expr, $src:expr, $type:ty) => {
        loop {
            let mut x: $type = ($src) as $type;
            unsafe {
                core::ptr::copy_nonoverlapping(
                    (((core::ptr::addr_of_mut!(x) as *mut i8) as *const ())
                        as *const core::ffi::c_void) as *const u8,
                    ((($ptr) as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                    ((core::mem::size_of::<$type>() as u64) as usize) as usize,
                )
            };
            if true {
                break;
            }
        }
    };
}

macro_rules! PACK_SINGLE__4 {
    ($ptr:expr, $src:expr, $type:ty) => {
        loop {
            let mut x: $type = ($src) as $type;
            unsafe {
                core::ptr::copy_nonoverlapping(
                    (((core::ptr::addr_of_mut!(x) as *mut i8) as *const ())
                        as *const core::ffi::c_void) as *const u8,
                    ((($ptr) as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                    ((core::mem::size_of::<$type>() as u64) as usize) as usize,
                )
            };
            if true {
                break;
            }
        }
    };
}

macro_rules! PACK_SINGLE__5 {
    ($ptr:expr, $src:expr, $type:ty) => {
        loop {
            let mut x: $type = $src;
            unsafe {
                core::ptr::copy_nonoverlapping(
                    (((core::ptr::addr_of_mut!(x) as *mut i8) as *const ())
                        as *const core::ffi::c_void) as *const u8,
                    ((($ptr) as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                    ((core::mem::size_of::<$type>() as u64) as usize) as usize,
                )
            };
            if true {
                break;
            }
        }
    };
}

macro_rules! PACK_SINGLE__6 {
    ($ptr:expr, $src:expr, $type:ty) => {
        loop {
            let mut x: $type = false;
            x = ($src) != 0;
            unsafe {
                core::ptr::copy_nonoverlapping(
                    (((core::ptr::addr_of_mut!(x) as *mut i8) as *const ())
                        as *const core::ffi::c_void) as *const u8,
                    ((($ptr) as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                    ((core::mem::size_of::<$type>() as u64) as usize) as usize,
                )
            };
            if true {
                break;
            }
        }
    };
}

macro_rules! PACK_SINGLE__2 {
    ($ptr:expr, $src:expr, $type:ty) => {
        loop {
            let mut x: $type = ($src) as $type;
            unsafe {
                core::ptr::copy_nonoverlapping(
                    (((core::ptr::addr_of_mut!(x) as *mut i8) as *const ())
                        as *const core::ffi::c_void) as *const u8,
                    ((($ptr) as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                    ((core::mem::size_of::<$type>() as u64) as usize) as usize,
                )
            };
            if true {
                break;
            }
        }
    };
}

macro_rules! PACK_SINGLE__7 {
    ($ptr:expr, $src:expr, $type:ty) => {
        loop {
            let mut x: $type = ($src) as $type;
            unsafe {
                core::ptr::copy_nonoverlapping(
                    (((core::ptr::addr_of_mut!(x) as *mut i8) as *const ())
                        as *const core::ffi::c_void) as *const u8,
                    ((($ptr) as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                    ((core::mem::size_of::<$type>() as u64) as usize) as usize,
                )
            };
            if true {
                break;
            }
        }
    };
}

macro_rules! PACK_SINGLE__8 {
    ($ptr:expr, $src:expr, $type:ty) => {
        loop {
            let mut x: $type = ($src) as $type;
            unsafe {
                core::ptr::copy_nonoverlapping(
                    (((core::ptr::addr_of_mut!(x) as *mut i8) as *const ())
                        as *const core::ffi::c_void) as *const u8,
                    ((($ptr) as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                    ((core::mem::size_of::<$type>() as u64) as usize) as usize,
                )
            };
            if true {
                break;
            }
        }
    };
}

macro_rules! PACK_SINGLE__9 {
    ($ptr:expr, $src:expr, $type:ty) => {
        loop {
            let mut x: $type = ($src) as $type;
            unsafe {
                core::ptr::copy_nonoverlapping(
                    (((core::ptr::addr_of_mut!(x) as *mut i8) as *const ())
                        as *const core::ffi::c_void) as *const u8,
                    ((($ptr) as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                    ((core::mem::size_of::<$type>() as u64) as usize) as usize,
                )
            };
            if true {
                break;
            }
        }
    };
}

macro_rules! PyBytes_AS_STRING {
    ($op:expr) => {
        PyBytes_AS_STRING(_PyObject_CAST!($op))
    };
}

macro_rules! PyBytes_Check {
    ($op:expr) => {
        PyType_FastSubclass!(Py_TYPE!($op), 134217728u64)
    };
}

macro_rules! PyBytes_GET_SIZE {
    ($self:expr) => {
        PyBytes_GET_SIZE(_PyObject_CAST!($self))
    };
}

/// Mask the old API with a call to the new API for code compiled under
/// Python 2.0:
macro_rules! PyErr_BadInternalCall__56 {
    () => {
        unsafe {
            _PyErr_BadInternalCall(
                c"/home/takashi/c-corpus/cpython/Objects/memoryobject.c".as_ptr(),
                3523i32,
            )
        };
    };
}

macro_rules! PyList_Check {
    ($op:expr) => {
        PyType_FastSubclass!(Py_TYPE!($op), 33554432u64)
    };
}

macro_rules! PyList_Check__2 {
    ($op:expr) => {
        PyType_FastSubclass!(Py_TYPE__2!($op), 33554432u64)
    };
}

macro_rules! PyList_GET_ITEM {
    ($op:expr, $index:expr) => {
        unsafe { *unsafe { (*_PyList_CAST!($op)).ob_item }.wrapping_offset(($index) as isize) }
    };
}

macro_rules! PyList_GET_SIZE {
    ($op:expr) => {
        PyList_GET_SIZE(_PyObject_CAST!($op))
    };
}

macro_rules! PyList_SET_ITEM {
    ($op:expr, $index:expr, $value:expr) => {
        PyList_SET_ITEM(_PyObject_CAST!($op), $index, _PyObject_CAST!($value));
    };
}

macro_rules! PyLong_Check {
    ($op:expr) => {
        PyType_FastSubclass!(Py_TYPE!($op), 16777216u64)
    };
}

macro_rules! PyMemoryView_Check {
    ($op:expr) => {
        Py_IS_TYPE!($op, unsafe { core::ptr::addr_of_mut!(PyMemoryView_Type) })
    };
}

macro_rules! PyObject_GC_NewVar__2 {
    ($type:ty, $typeobj:expr, $n:expr) => {
        _Py_CAST!(*mut $type, unsafe {
            _PyObject_GC_NewVar($typeobj, ($n) as i64)
        })
    };
}

macro_rules! PyObject_GC_New {
    ($type:ty, $typeobj:expr) => {
        _Py_CAST!(*mut $type, unsafe { _PyObject_GC_New($typeobj) })
    };
}

/// Return the 'i'-th element of the sequence 'o', assuming that o was returned
/// by PySequence_Fast, and that i is within bounds.
macro_rules! PySequence_Fast_GET_ITEM__2 {
    ($o:expr, $i:expr) => {{
        let __v: *mut _object = if PyList_Check__2!($o) != 0 {
            PyList_GET_ITEM!($o, $i)
        } else {
            PyTuple_GET_ITEM!($o, $i)
        };
        __v
    }};
}

/// Return the size of the sequence 'o', assuming that 'o' was returned by
/// PySequence_Fast and is not NULL.
macro_rules! PySequence_Fast_GET_SIZE {
    ($o:expr) => {{
        let __v: i64 = if PyList_Check!($o) != 0 {
            PyList_GET_SIZE!($o)
        } else {
            PyTuple_GET_SIZE!($o)
        };
        __v
    }};
}

macro_rules! PySlice_Check {
    ($op:expr) => {
        Py_IS_TYPE!($op, unsafe { core::ptr::addr_of_mut!(PySlice_Type) })
    };
}

macro_rules! PyTuple_Check {
    ($op:expr) => {
        PyType_FastSubclass!(Py_TYPE!($op), 67108864u64)
    };
}

macro_rules! PyTuple_GET_ITEM {
    ($op:expr, $index:expr) => {
        unsafe {
            *((*_PyTuple_CAST!($op)).ob_item.as_mut_ptr() as *mut *mut _object)
                .wrapping_offset(($index) as isize)
        }
    };
}

macro_rules! PyTuple_GET_SIZE {
    ($op:expr) => {
        PyTuple_GET_SIZE(_PyObject_CAST!($op))
    };
}

macro_rules! PyTuple_SET_ITEM {
    ($op:expr, $index:expr, $value:expr) => {
        PyTuple_SET_ITEM(
            _PyObject_CAST!($op),
            ($index) as i64,
            _PyObject_CAST!($value),
        );
    };
}

macro_rules! PyType_FastSubclass {
    ($type:expr, $flag:expr) => {
        PyType_HasFeature($type, $flag)
    };
}

macro_rules! PyUnicode_Check {
    ($op:expr) => {
        PyType_FastSubclass!(Py_TYPE!($op), 268435456u64)
    };
}

macro_rules! Py_CLEAR__14 {
    ($op:expr) => {
        loop {
            let _tmp_op_ptr: *mut *mut __SlateRecord_memoryobject_h_16 =
                unsafe { core::ptr::addr_of_mut!($op) };
            let _tmp_old_op: *mut __SlateRecord_memoryobject_h_16 = unsafe { *_tmp_op_ptr };
            if !_tmp_old_op.is_null() {
                unsafe {
                    *_tmp_op_ptr = core::ptr::null_mut::<__SlateRecord_memoryobject_h_16>();
                }
                Py_DECREF__2!(_tmp_old_op);
            }
            if true {
                break;
            }
        }
    };
}

macro_rules! Py_DECREF__2 {
    ($op:expr) => {
        Py_DECREF(_PyObject_CAST__2!($op));
    };
}

macro_rules! Py_DECREF {
    ($op:expr) => {
        Py_DECREF(_PyObject_CAST!($op));
    };
}

macro_rules! Py_INCREF {
    ($op:expr) => {
        Py_INCREF(_PyObject_CAST!($op));
    };
}

macro_rules! Py_IS_TYPE {
    ($ob:expr, $type:expr) => {
        Py_IS_TYPE(_PyObject_CAST!($ob), $type)
    };
}

/// Maximum value between x and y
macro_rules! Py_MAX__2 {
    ($x:expr, $y:expr) => {
        if ($x) > (($y) as i64) {
            $x
        } else {
            ($y) as i64
        }
    };
}

/// Minimum value between x and y
macro_rules! Py_MIN {
    ($x:expr, $y:expr) => {
        if ($x) > $y { $y } else { $x }
    };
}

macro_rules! Py_NewRef__2 {
    ($obj:expr) => {
        _Py_NewRef(_PyObject_CAST__2!($obj))
    };
}

macro_rules! Py_NewRef {
    ($obj:expr) => {
        _Py_NewRef(_PyObject_CAST!($obj))
    };
}

macro_rules! Py_SIZE {
    ($ob:expr) => {
        Py_SIZE(_PyObject_CAST__2!($ob))
    };
}

macro_rules! Py_TYPE__2 {
    ($ob:expr) => {
        _Py_TYPE(_PyObject_CAST__2!($ob))
    };
}

macro_rules! Py_TYPE {
    ($ob:expr) => {
        _Py_TYPE(_PyObject_CAST!($ob))
    };
}

/// Utility macro to help write tp_traverse functions.
/// To use this macro, the tp_traverse function must name its arguments
/// "visit" and "arg".  This is intended to keep tp_traverse functions
/// looking as much alike as possible.
macro_rules! Py_VISIT__16 {
    ($op:expr, $visit:expr, $arg:expr) => {{
        loop {
            if !$op.is_null() {
                let vret: i32 = unsafe { $visit.unwrap()(_PyObject_CAST__2!($op), $arg) };
                if vret != 0 {
                    return vret;
                }
            }
            if true {
                break;
            }
        }
    }};
}

/// Utility macro to help write tp_traverse functions.
/// To use this macro, the tp_traverse function must name its arguments
/// "visit" and "arg".  This is intended to keep tp_traverse functions
/// looking as much alike as possible.
macro_rules! Py_VISIT__17 {
    ($op:expr, $visit:expr, $arg:expr) => {{
        loop {
            if !$op.is_null() {
                let vret: i32 = unsafe { $visit.unwrap()(_PyObject_CAST__2!($op), $arg) };
                if vret != 0 {
                    return vret;
                }
            }
            if true {
                break;
            }
        }
    }};
}

/// Utility macro to help write tp_traverse functions.
/// To use this macro, the tp_traverse function must name its arguments
/// "visit" and "arg".  This is intended to keep tp_traverse functions
/// looking as much alike as possible.
macro_rules! Py_VISIT {
    ($op:expr, $visit:expr, $arg:expr) => {{
        loop {
            if !$op.is_null() {
                let vret: i32 = unsafe { $visit.unwrap()(_PyObject_CAST!($op), $arg) };
                if vret != 0 {
                    return vret;
                }
            }
            if true {
                break;
            }
        }
    }};
}

macro_rules! Py_XDECREF__2 {
    ($op:expr) => {
        Py_XDECREF(_PyObject_CAST__2!($op));
    };
}

macro_rules! Py_XDECREF {
    ($op:expr) => {
        Py_XDECREF(_PyObject_CAST!($op));
    };
}

macro_rules! Py_XINCREF {
    ($op:expr) => {
        Py_XINCREF(_PyObject_CAST!($op));
    };
}

macro_rules! Py_XNewRef {
    ($obj:expr) => {
        _Py_XNewRef(_PyObject_CAST!($obj))
    };
}

macro_rules! REQ_ANY_CONTIGUOUS {
    ($flags:expr) => {
        ($flags) & (128 | (16 | 8)) == 128 | (16 | 8)
    };
}

macro_rules! REQ_C_CONTIGUOUS {
    ($flags:expr) => {
        ($flags) & (32 | (16 | 8)) == 32 | (16 | 8)
    };
}

macro_rules! REQ_FORMAT {
    ($flags:expr) => {
        ($flags) & 4
    };
}

macro_rules! REQ_F_CONTIGUOUS {
    ($flags:expr) => {
        ($flags) & (64 | (16 | 8)) == 64 | (16 | 8)
    };
}

/// getbuffer() requests
macro_rules! REQ_INDIRECT {
    ($flags:expr) => {
        ($flags) & (256 | (16 | 8)) == 256 | (16 | 8)
    };
}

macro_rules! REQ_SHAPE {
    ($flags:expr) => {
        ($flags) & 8 == 8
    };
}

macro_rules! REQ_STRIDES {
    ($flags:expr) => {
        ($flags) & (16 | 8) == 16 | 8
    };
}

macro_rules! REQ_WRITABLE {
    ($flags:expr) => {
        ($flags) & 1
    };
}

macro_rules! UNPACK_SINGLE__3 {
    ($dest:expr, $ptr:expr, $type:ty) => {
        loop {
            let mut x: $type = 0;
            unsafe {
                core::ptr::copy_nonoverlapping(
                    ((($ptr) as *const ()) as *const core::ffi::c_void) as *const u8,
                    (((core::ptr::addr_of_mut!(x) as *mut i8) as *mut ()) as *mut core::ffi::c_void)
                        as *mut u8,
                    ((core::mem::size_of::<$type>() as u64) as usize) as usize,
                )
            };
            $dest = x as i64;
            if true {
                break;
            }
        }
    };
}

macro_rules! UNPACK_SINGLE__4 {
    ($dest:expr, $ptr:expr, $type:ty) => {
        loop {
            let mut x: $type = 0 as f32;
            unsafe {
                core::ptr::copy_nonoverlapping(
                    ((($ptr) as *const ()) as *const core::ffi::c_void) as *const u8,
                    (((core::ptr::addr_of_mut!(x) as *mut i8) as *mut ()) as *mut core::ffi::c_void)
                        as *mut u8,
                    ((core::mem::size_of::<$type>() as u64) as usize) as usize,
                )
            };
            $dest = x as f64;
            if true {
                break;
            }
        }
    };
}

macro_rules! UNPACK_SINGLE__5 {
    ($dest:expr, $ptr:expr, $type:ty) => {
        loop {
            let mut x: $type = 0 as f64;
            unsafe {
                core::ptr::copy_nonoverlapping(
                    ((($ptr) as *const ()) as *const core::ffi::c_void) as *const u8,
                    (((core::ptr::addr_of_mut!(x) as *mut i8) as *mut ()) as *mut core::ffi::c_void)
                        as *mut u8,
                    ((core::mem::size_of::<$type>() as u64) as usize) as usize,
                )
            };
            $dest = x;
            if true {
                break;
            }
        }
    };
}

macro_rules! UNPACK_SINGLE {
    ($dest:expr, $ptr:expr, $type:ty) => {
        loop {
            let mut x: $type = 0;
            unsafe {
                core::ptr::copy_nonoverlapping(
                    ((($ptr) as *const ()) as *const core::ffi::c_void) as *const u8,
                    (((core::ptr::addr_of_mut!(x) as *mut i8) as *mut ()) as *mut core::ffi::c_void)
                        as *mut u8,
                    ((core::mem::size_of::<$type>() as u64) as usize) as usize,
                )
            };
            $dest = x;
            if true {
                break;
            }
        }
    };
}

macro_rules! UNPACK_SINGLE__6 {
    ($dest:expr, $ptr:expr, $type:ty) => {
        loop {
            let mut x: $type = 0;
            unsafe {
                core::ptr::copy_nonoverlapping(
                    ((($ptr) as *const ()) as *const core::ffi::c_void) as *const u8,
                    (((core::ptr::addr_of_mut!(x) as *mut i8) as *mut ()) as *mut core::ffi::c_void)
                        as *mut u8,
                    ((core::mem::size_of::<$type>() as u64) as usize) as usize,
                )
            };
            $dest = x as u64;
            if true {
                break;
            }
        }
    };
}

macro_rules! UNPACK_SINGLE__7 {
    ($dest:expr, $ptr:expr, $type:ty) => {
        loop {
            let mut x: $type = 0;
            unsafe {
                core::ptr::copy_nonoverlapping(
                    ((($ptr) as *const ()) as *const core::ffi::c_void) as *const u8,
                    (((core::ptr::addr_of_mut!(x) as *mut i8) as *mut ()) as *mut core::ffi::c_void)
                        as *mut u8,
                    ((core::mem::size_of::<$type>() as u64) as usize) as usize,
                )
            };
            $dest = x as u64;
            if true {
                break;
            }
        }
    };
}

macro_rules! UNPACK_SINGLE__8 {
    ($dest:expr, $ptr:expr, $type:ty) => {
        loop {
            let mut x: $type = 0;
            unsafe {
                core::ptr::copy_nonoverlapping(
                    ((($ptr) as *const ()) as *const core::ffi::c_void) as *const u8,
                    (((core::ptr::addr_of_mut!(x) as *mut i8) as *mut ()) as *mut core::ffi::c_void)
                        as *mut u8,
                    ((core::mem::size_of::<$type>() as u64) as usize) as usize,
                )
            };
            $dest = x as i64;
            if true {
                break;
            }
        }
    };
}

macro_rules! UNPACK_SINGLE__2 {
    ($dest:expr, $ptr:expr, $type:ty) => {
        loop {
            let mut x: $type = 0;
            unsafe {
                core::ptr::copy_nonoverlapping(
                    ((($ptr) as *const ()) as *const core::ffi::c_void) as *const u8,
                    (((core::ptr::addr_of_mut!(x) as *mut i8) as *mut ()) as *mut core::ffi::c_void)
                        as *mut u8,
                    ((core::mem::size_of::<$type>() as u64) as usize) as usize,
                )
            };
            $dest = x;
            if true {
                break;
            }
        }
    };
}

macro_rules! UNPACK_TO_BOOL {
    ($PTR:expr) => {
        (unsafe {
            memcmp(
                (($PTR) as *const ()) as *const core::ffi::c_void,
                (core::ptr::addr_of!(bool_false) as *const ()) as *const core::ffi::c_void,
                (core::mem::size_of::<bool>() as u64) as usize,
            ) as i32
        }) != 0
    };
}

macro_rules! VIEW_ADDR {
    ($mv:expr) => {
        unsafe { core::ptr::addr_of_mut!((*$mv).view) }
    };
}

macro_rules! VIEW_ADDR__2 {
    ($mv:expr) => {
        unsafe { core::ptr::addr_of_mut!((*(($mv) as *mut __SlateRecord_memoryobject_h_36)).view) }
    };
}

macro_rules! _PyArg_CheckPositional {
    ($funcname:expr, $nargs:expr, $min:expr, $max:expr) => {{
        let __v: bool =
            !_Py_ANY_VARARGS!($max) && (($min) as i64) <= $nargs && ($nargs) <= (($max) as i64)
                || (unsafe {
                    _PyArg_CheckPositional(
                        ($funcname) as *const i8,
                        $nargs,
                        ($min) as i64,
                        ($max) as i64,
                    )
                }) != 0;
        __v
    }};
}

macro_rules! _PyArg_UnpackKeywords__2 {
    ($args:expr, $nargs:expr, $kwargs:expr, $kwnames:expr, $parser:expr, $minpos:expr, $maxpos:expr, $minkw:expr, $varpos:expr, $buf:expr) => {{
        let __v: bool = ($minkw) == 0
            && $kwargs.is_null()
            && $kwnames.is_null()
            && (($minpos) as i64) <= $nargs
            && (($varpos) != 0 || ($nargs) <= (($maxpos) as i64))
            && !$args.is_null();
        let __v2: *const *mut _object = if __v {
            ($args) as *const *mut _object
        } else {
            unsafe {
                _PyArg_UnpackKeywords(
                    ($args) as *const *mut _object,
                    $nargs,
                    $kwargs,
                    core::ptr::null_mut::<_object>(),
                    $parser,
                    $minpos,
                    $maxpos,
                    $minkw,
                    $varpos,
                    $buf,
                )
            }
        };
        __v2
    }};
}

macro_rules! _PyArg_UnpackKeywords {
    ($args:expr, $nargs:expr, $kwargs:expr, $kwnames:expr, $parser:expr, $minpos:expr, $maxpos:expr, $minkw:expr, $varpos:expr, $buf:expr) => {{
        let __v: *const *mut _object = if ($minkw) == 0
            && $kwargs.is_null()
            && $kwnames.is_null()
            && (($minpos) as i64) <= $nargs
            && (($varpos) != 0 || ($nargs) <= (($maxpos) as i64))
            && !$args.is_null()
        {
            $args
        } else {
            unsafe {
                _PyArg_UnpackKeywords(
                    $args,
                    $nargs,
                    core::ptr::null_mut::<_object>(),
                    $kwnames,
                    $parser,
                    $minpos,
                    $maxpos,
                    $minkw,
                    $varpos,
                    $buf,
                )
            }
        };
        __v
    }};
}

/// Macros and static inline functions, trading safety for speed
macro_rules! _PyBytes_CAST {
    ($op:expr) => {
        _Py_CAST!(*mut __SlateRecord_bytesobject_h_15, $op)
    };
}

/// Cast argument to PyListObject* type.
macro_rules! _PyList_CAST {
    ($op:expr) => {
        _Py_CAST!(*mut __SlateRecord_listobject_h_22, $op)
    };
}

/// Define a pair of assertion macros:
/// _PyObject_ASSERT_FROM(), _PyObject_ASSERT_WITH_MSG() and _PyObject_ASSERT().
///
/// These work like the regular C assert(), in that they will abort the
/// process with a message on stderr if the given condition fails to hold,
/// but compile away to nothing if NDEBUG is defined.
///
/// However, before aborting, Python will also try to call _PyObject_Dump() on
/// the given object.  This may be of use when investigating bugs in which a
/// particular object is corrupt (e.g. buggy a tp_visit method in an extension
/// module breaking the garbage collector), to help locate the broken objects.
///
/// The WITH_MSG variant allows you to supply an additional message that Python
/// will attempt to print to stderr, after the object dump.
///
/// No debugging: compile away the assertions:
macro_rules! _PyObject_ASSERT_FROM {
    () => {};
}

/// Cast argument to PyObject* type.
macro_rules! _PyObject_CAST {
    ($op:expr) => {
        _Py_CAST__2!($op)
    };
}

/// Cast argument to PyObject* type.
macro_rules! _PyObject_CAST__2 {
    ($op:expr) => {
        _Py_CAST!(*mut _object, $op)
    };
}

macro_rules! _PyObject_GC_TRACK {
    ($op:expr) => {
        _PyObject_GC_TRACK(_PyObject_CAST__2!($op));
    };
}

macro_rules! _PyObject_GC_UNTRACK__2 {
    ($op:expr) => {
        _PyObject_GC_UNTRACK(_PyObject_CAST__2!($op));
    };
}

/// Cast argument to PyTupleObject* type.
macro_rules! _PyTuple_CAST {
    ($op:expr) => {
        _Py_CAST!(*mut __SlateRecord_tupleobject_h_13, $op)
    };
}

/// Cast argument to PyVarObject* type.
macro_rules! _PyVarObject_CAST {
    ($op:expr) => {
        _Py_CAST!(*mut __SlateRecord_object_h_172, $op)
    };
}

macro_rules! _Py_ANY_VARARGS {
    ($n:expr) => {
        (($n) as i64) == 9223372036854775807
    };
}

/// Macro to use the more powerful/dangerous C-style cast even in C++.
macro_rules! _Py_CAST__2 {
    ($expr:expr) => {
        $expr
    };
}

/// Macro to use the more powerful/dangerous C-style cast even in C++.
macro_rules! _Py_CAST {
    ($type:ty, $expr:expr) => {
        ($expr) as $type
    };
}

macro_rules! _Py_DECREF_IMMORTAL_STAT_INC {
    () => {};
}

macro_rules! _Py_DECREF_STAT_INC {
    () => {};
}

macro_rules! _Py_INCREF_IMMORTAL_STAT_INC {
    () => {};
}

macro_rules! _Py_INCREF_STAT_INC {
    () => {};
}

macro_rules! _Py_IsImmortal {
    ($op:expr) => {
        _Py_IsImmortal(_PyObject_CAST!($op))
    };
}

unsafe extern "C" {
    static mut PyType_Type: _typeobject;
    static mut _Py_NoneStruct: _object;
    static mut _Py_NotImplementedStruct: _object;
    static mut PyExc_BufferError: *mut _object;
    static mut PyExc_ImportError: *mut _object;
    static mut PyExc_IndexError: *mut _object;
    static mut PyExc_MemoryError: *mut _object;
    static mut PyExc_OverflowError: *mut _object;
    static mut PyExc_RuntimeError: *mut _object;
    static mut PyExc_NotImplementedError: *mut _object;
    static mut PyExc_SystemError: *mut _object;
    static mut PyExc_TypeError: *mut _object;
    static mut PyExc_ValueError: *mut _object;
    static mut _Py_FalseStruct: _longobject;
    static mut _Py_TrueStruct: _longobject;
    static mut PyTuple_Type: _typeobject;
    static mut _Py_EllipsisObject: _object;
    static mut PySlice_Type: _typeobject;
    #[thread_local]
    static mut _Py_tss_tstate: *mut _ts;
    static mut _PyRuntime: pyruntimestate;
    fn index(arg0: *const i8, arg1: i32) -> *mut i8;
    fn PyMem_Malloc(arg0: u64) -> *mut ();
    fn PyMem_Free(arg0: *mut ());
    fn PyObject_CheckBuffer(arg0: *mut _object) -> i32;
    fn PyObject_GetBuffer(
        arg0: *mut _object,
        arg1: *mut __SlateRecord_pybuffer_h_33,
        arg2: i32,
    ) -> i32;
    fn PyBuffer_IsContiguous(arg0: *const __SlateRecord_pybuffer_h_33, arg1: i8) -> i32;
    fn PyBuffer_FillInfo(
        arg0: *mut __SlateRecord_pybuffer_h_33,
        arg1: *mut _object,
        arg2: *mut (),
        arg3: i64,
        arg4: i32,
        arg5: i32,
    ) -> i32;
    fn PyBuffer_Release(arg0: *mut __SlateRecord_pybuffer_h_33);
    fn PyObject_RichCompareBool(arg0: *mut _object, arg1: *mut _object, arg2: i32) -> i32;
    fn PyObject_GetAttrString(arg0: *mut _object, arg1: *const i8) -> *mut _object;
    fn PyObject_SelfIter(arg0: *mut _object) -> *mut _object;
    fn PyObject_GenericGetAttr(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn PyObject_Hash(arg0: *mut _object) -> i64;
    fn PyObject_IsTrue(arg0: *mut _object) -> i32;
    fn PyObject_ClearWeakRefs(arg0: *mut _object);
    fn _Py_Dealloc(arg0: *mut _object);
    fn _PyObject_GC_New(arg0: *mut _typeobject) -> *mut _object;
    fn _PyObject_GC_NewVar(arg0: *mut _typeobject, arg1: i64) -> *mut __SlateRecord_object_h_172;
    fn PyObject_GC_Del(arg0: *mut ());
    fn Py_HashBuffer(arg0: *const (), arg1: i64) -> i64;
    fn PyBytes_FromStringAndSize(arg0: *const i8, arg1: i64) -> *mut _object;
    fn PyBytes_FromString(arg0: *const i8) -> *mut _object;
    fn PyUnicode_FromString(arg0: *const i8) -> *mut _object;
    fn PyUnicode_FromFormat(arg0: *const i8, ...) -> *mut _object;
    fn PyUnicode_AsUTF8AndSize(arg0: *mut _object, arg1: *mut i64) -> *const i8;
    fn PyUnicode_AsASCIIString(arg0: *mut _object) -> *mut _object;
    fn PyErr_SetString(arg0: *mut _object, arg1: *const i8);
    fn PyErr_Occurred() -> *mut _object;
    fn PyErr_Clear();
    fn PyErr_ExceptionMatches(arg0: *mut _object) -> i32;
    fn PyErr_NoMemory() -> *mut _object;
    fn PyErr_Format(arg0: *mut _object, arg1: *const i8, ...) -> *mut _object;
    fn _PyErr_BadInternalCall(arg0: *const i8, arg1: i32);
    fn PyLong_FromLong(arg0: i64) -> *mut _object;
    fn PyLong_FromUnsignedLong(arg0: u64) -> *mut _object;
    fn PyLong_FromSize_t(arg0: u64) -> *mut _object;
    fn PyLong_FromSsize_t(arg0: i64) -> *mut _object;
    fn PyLong_AsLong(arg0: *mut _object) -> i64;
    fn PyLong_AsSsize_t(arg0: *mut _object) -> i64;
    fn PyLong_AsSize_t(arg0: *mut _object) -> u64;
    fn PyLong_AsUnsignedLong(arg0: *mut _object) -> u64;
    fn PyLong_AsInt(arg0: *mut _object) -> i32;
    fn PyLong_FromVoidPtr(arg0: *mut ()) -> *mut _object;
    fn PyLong_AsVoidPtr(arg0: *mut _object) -> *mut ();
    fn PyLong_FromLongLong(arg0: i64) -> *mut _object;
    fn PyLong_FromUnsignedLongLong(arg0: u64) -> *mut _object;
    fn PyLong_AsLongLong(arg0: *mut _object) -> i64;
    fn PyLong_AsUnsignedLongLong(arg0: *mut _object) -> u64;
    fn PyBool_FromLong(arg0: i64) -> *mut _object;
    fn PyFloat_FromDouble(arg0: f64) -> *mut _object;
    fn PyFloat_AsDouble(arg0: *mut _object) -> f64;
    fn PyFloat_Pack2(arg0: f64, arg1: *mut i8, arg2: i32) -> i32;
    fn PyFloat_Unpack2(arg0: *const i8, arg1: i32) -> f64;
    fn PyTuple_New(arg0: i64) -> *mut _object;
    fn PyList_New(arg0: i64) -> *mut _object;
    fn PySlice_Unpack(arg0: *mut _object, arg1: *mut i64, arg2: *mut i64, arg3: *mut i64) -> i32;
    fn PySlice_AdjustIndices(arg0: i64, arg1: *mut i64, arg2: *mut i64, arg3: i64) -> i64;
    fn Py_GenericAlias(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn _PyEval_SliceIndexNotNone(arg0: *mut _object, arg1: *mut i64) -> i32;
    fn PyImport_ImportModuleAttrString(arg0: *const i8, arg1: *const i8) -> *mut _object;
    fn PyObject_GetIter(arg0: *mut _object) -> *mut _object;
    fn PyIter_NextItem(arg0: *mut _object, arg1: *mut *mut _object) -> i32;
    fn PyNumber_AsSsize_t(arg0: *mut _object, arg1: *mut _object) -> i64;
    fn PyObject_CallOneArg(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn _PyNumber_Index(arg0: *mut _object) -> *mut _object;
    fn _Py_strhex_with_sep(
        arg0: *const i8,
        arg1: i64,
        arg2: *mut _object,
        arg3: i32,
    ) -> *mut _object;
    fn _PyArg_CheckPositional(arg0: *const i8, arg1: i64, arg2: i64, arg3: i64) -> i32;
    fn _PyArg_BadArgument(arg0: *const i8, arg1: *const i8, arg2: *const i8, arg3: *mut _object);
    fn _PyArg_UnpackKeywords(
        arg0: *const *mut _object,
        arg1: i64,
        arg2: *mut _object,
        arg3: *mut _object,
        arg4: *mut _PyArg_Parser,
        arg5: i32,
        arg6: i32,
        arg7: i32,
        arg8: i32,
        arg9: *mut *mut _object,
    ) -> *const *mut _object;
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord___sigset_t_h_8 {
    __val: [u64; 16],
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord_atomic_wide_counter_h_33 {
    __value64: u64,
    __value32: __SlateRecord_atomic_wide_counter_h_33_2,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_atomic_wide_counter_h_33_2 {
    __low: u32,
    __high: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __pthread_internal_list {
    __prev: *mut __pthread_internal_list,
    __next: *mut __pthread_internal_list,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __pthread_mutex_s {
    __lock: i32,
    __count: u32,
    __owner: i32,
    __nusers: u32,
    __kind: i32,
    __spins: i16,
    __glibc_reserved: i16,
    __list: __pthread_internal_list,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __pthread_cond_s {
    __wseq: __SlateRecord_atomic_wide_counter_h_33,
    __g1_start: __SlateRecord_atomic_wide_counter_h_33,
    __g_size: [u32; 2],
    __g1_orig_size: u32,
    __wrefs: u32,
    __g_signals: [u32; 2],
    __unused_initialized_1: u32,
    __unused_initialized_2: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord_pthreadtypes_h_45 {
    __size: [i8; 4],
    __align: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord_pthreadtypes_h_72 {
    __data: __pthread_mutex_s,
    __size: [i8; 40],
    __align: i64,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord_pthreadtypes_h_80 {
    __data: __pthread_cond_s,
    __size: [i8; 48],
    __align: i64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pymem_h_48 {
    ctx: *mut (),
    malloc: Option<unsafe extern "C-unwind" fn(*mut (), u64) -> *mut ()>,
    calloc: Option<unsafe extern "C-unwind" fn(*mut (), u64, u64) -> *mut ()>,
    realloc: Option<unsafe extern "C-unwind" fn(*mut (), *mut (), u64) -> *mut ()>,
    free: Option<unsafe extern "C-unwind" fn(*mut (), *mut ())>,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pybuffer_h_33 {
    buf: *mut (),
    obj: *mut _object,
    len: i64,
    itemsize: i64,
    readonly: i32,
    ndim: i32,
    format: *mut i8,
    shape: *mut i64,
    strides: *mut i64,
    suboffsets: *mut i64,
    internal: *mut (),
}

// Maximum number of dimensions
#[repr(C)]
#[derive(Clone, Copy)]
struct PyMutex {
    _bits: u8,
}

// PyObject_VAR_HEAD defines the initial segment of all variable-size
// container objects.  These end with a declaration of an array with 1
// element, but enough space is malloc'ed so that the array actually
// has room for ob_size elements.  Note that ob_size is an element count,
// not necessarily a byte count.
#[repr(C)]
#[derive(Clone, Copy)]
struct _object {
    __slate_anon_0: __SlateRecord_object_h_122,
    ob_type: *mut _typeobject,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_object_h_172 {
    ob_base: _object,
    ob_size: i64,
}

fn _Py_TYPE(ob: *mut _object) -> *mut _typeobject {
    unsafe { (*ob).ob_type }
}

fn Py_SIZE(ob: *mut _object) -> i64 {
    unsafe { (*_PyVarObject_CAST!(ob)).ob_size }
}

fn Py_IS_TYPE(ob: *mut _object, r#type: *mut _typeobject) -> i32 {
    (Py_TYPE!(ob) == r#type) as i32
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _frame {}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_object_h_106 {
    nb_add: Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
    nb_subtract: Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
    nb_multiply: Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
    nb_remainder: Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
    nb_divmod: Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
    nb_power: Option<
        unsafe extern "C-unwind" fn(*mut _object, *mut _object, *mut _object) -> *mut _object,
    >,
    nb_negative: Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
    nb_positive: Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
    nb_absolute: Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
    nb_bool: Option<unsafe extern "C-unwind" fn(*mut _object) -> i32>,
    nb_invert: Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
    nb_lshift: Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
    nb_rshift: Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
    nb_and: Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
    nb_xor: Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
    nb_or: Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
    nb_int: Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
    nb_reserved: *mut (),
    nb_float: Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
    nb_inplace_add: Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
    nb_inplace_subtract:
        Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
    nb_inplace_multiply:
        Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
    nb_inplace_remainder:
        Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
    nb_inplace_power: Option<
        unsafe extern "C-unwind" fn(*mut _object, *mut _object, *mut _object) -> *mut _object,
    >,
    nb_inplace_lshift:
        Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
    nb_inplace_rshift:
        Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
    nb_inplace_and: Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
    nb_inplace_xor: Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
    nb_inplace_or: Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
    nb_floor_divide:
        Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
    nb_true_divide: Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
    nb_inplace_floor_divide:
        Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
    nb_inplace_true_divide:
        Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
    nb_index: Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
    nb_matrix_multiply:
        Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
    nb_inplace_matrix_multiply:
        Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_object_h_120 {
    sq_length: Option<unsafe extern "C-unwind" fn(*mut _object) -> i64>,
    sq_concat: Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
    sq_repeat: Option<unsafe extern "C-unwind" fn(*mut _object, i64) -> *mut _object>,
    sq_item: Option<unsafe extern "C-unwind" fn(*mut _object, i64) -> *mut _object>,
    was_sq_slice: *mut (),
    sq_ass_item: Option<unsafe extern "C-unwind" fn(*mut _object, i64, *mut _object) -> i32>,
    was_sq_ass_slice: *mut (),
    sq_contains: Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> i32>,
    sq_inplace_concat:
        Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
    sq_inplace_repeat: Option<unsafe extern "C-unwind" fn(*mut _object, i64) -> *mut _object>,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_object_h_126 {
    mp_length: Option<unsafe extern "C-unwind" fn(*mut _object) -> i64>,
    mp_subscript: Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
    mp_ass_subscript:
        Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object, *mut _object) -> i32>,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_object_h_135 {
    am_await: Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
    am_aiter: Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
    am_anext: Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
    am_send:
        Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object, *mut *mut _object) -> i32>,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord_object_h_122 {
    ob_refcnt_full: i64,
    __slate_anon_1: __SlateRecord_object_h_125,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_object_h_125 {
    ob_refcnt: u32,
    ob_overflow: u16,
    ob_flags: u16,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_object_h_140 {
    bf_getbuffer: Option<
        unsafe extern "C-unwind" fn(*mut _object, *mut __SlateRecord_pybuffer_h_33, i32) -> i32,
    >,
    bf_releasebuffer:
        Option<unsafe extern "C-unwind" fn(*mut _object, *mut __SlateRecord_pybuffer_h_33)>,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _typeobject {
    ob_base: __SlateRecord_object_h_172,
    tp_name: *const i8,
    tp_basicsize: i64,
    tp_itemsize: i64,
    tp_dealloc: Option<unsafe extern "C-unwind" fn(*mut _object)>,
    tp_vectorcall_offset: i64,
    tp_getattr: Option<unsafe extern "C-unwind" fn(*mut _object, *mut i8) -> *mut _object>,
    tp_setattr: Option<unsafe extern "C-unwind" fn(*mut _object, *mut i8, *mut _object) -> i32>,
    tp_as_async: *mut __SlateRecord_object_h_135,
    tp_repr: Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
    tp_as_number: *mut __SlateRecord_object_h_106,
    tp_as_sequence: *mut __SlateRecord_object_h_120,
    tp_as_mapping: *mut __SlateRecord_object_h_126,
    tp_hash: Option<unsafe extern "C-unwind" fn(*mut _object) -> i64>,
    tp_call: Option<
        unsafe extern "C-unwind" fn(*mut _object, *mut _object, *mut _object) -> *mut _object,
    >,
    tp_str: Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
    tp_getattro: Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
    tp_setattro:
        Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object, *mut _object) -> i32>,
    tp_as_buffer: *mut __SlateRecord_object_h_140,
    tp_flags: u64,
    tp_doc: *const i8,
    tp_traverse: Option<
        unsafe extern "C-unwind" fn(
            *mut _object,
            Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
            *mut (),
        ) -> i32,
    >,
    tp_clear: Option<unsafe extern "C-unwind" fn(*mut _object) -> i32>,
    tp_richcompare:
        Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object, i32) -> *mut _object>,
    tp_weaklistoffset: i64,
    tp_iter: Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
    tp_iternext: Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
    tp_methods: *mut PyMethodDef,
    tp_members: *mut PyMemberDef,
    tp_getset: *mut PyGetSetDef,
    tp_base: *mut _typeobject,
    tp_dict: *mut _object,
    tp_descr_get: Option<
        unsafe extern "C-unwind" fn(*mut _object, *mut _object, *mut _object) -> *mut _object,
    >,
    tp_descr_set:
        Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object, *mut _object) -> i32>,
    tp_dictoffset: i64,
    tp_init: Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object, *mut _object) -> i32>,
    tp_alloc: Option<unsafe extern "C-unwind" fn(*mut _typeobject, i64) -> *mut _object>,
    tp_new: Option<
        unsafe extern "C-unwind" fn(*mut _typeobject, *mut _object, *mut _object) -> *mut _object,
    >,
    tp_free: Option<unsafe extern "C-unwind" fn(*mut ())>,
    tp_is_gc: Option<unsafe extern "C-unwind" fn(*mut _object) -> i32>,
    tp_bases: *mut _object,
    tp_mro: *mut _object,
    tp_cache: *mut _object,
    tp_subclasses: *mut (),
    tp_weaklist: *mut _object,
    tp_del: Option<unsafe extern "C-unwind" fn(*mut _object)>,
    tp_version_tag: u32,
    tp_finalize: Option<unsafe extern "C-unwind" fn(*mut _object)>,
    tp_vectorcall: Option<
        unsafe extern "C-unwind" fn(
            *mut _object,
            *const *mut _object,
            u64,
            *mut _object,
        ) -> *mut _object,
    >,
    tp_watched: u8,
    tp_versions_used: u16,
}

// Track types initialized using _PyStaticType_InitBuiltin().
// Set if instances of the type object are treated as sequences for pattern matching
// Objects support garbage collection (see objimpl.h)
fn PyType_HasFeature(r#type: *mut _typeobject, feature: u64) -> i32 {
    let flags: u64 = unsafe { (*r#type).tp_flags };
    (flags & feature != 0) as i32
}

#[inline(always)]
fn _Py_IsImmortal(op: *mut _object) -> i32 {
    (_Py_CAST!(i32, unsafe {
        (*op).__slate_anon_0.__slate_anon_1.ob_refcnt
    }) < 0) as i32
}

#[inline(always)]
fn Py_INCREF(op: *mut _object) {
    // Non-limited C API and limited C API for Python 3.9 and older access
    //
    // directly PyObject.ob_refcnt.
    let cur_refcnt: u32 = unsafe { (*op).__slate_anon_0.__slate_anon_1.ob_refcnt };
    if (cur_refcnt as u64) >= 3221225472u64 {
        // the object is immortal
        _Py_INCREF_IMMORTAL_STAT_INC!();
        return;
    }
    unsafe {
        (*op).__slate_anon_0.__slate_anon_1.ob_refcnt = cur_refcnt.wrapping_add(1);
    }
    _Py_INCREF_STAT_INC!();
}

#[inline(always)]
fn Py_DECREF(op: *mut _object) {
    // Non-limited C API and limited C API for Python 3.9 and older access
    //
    // directly PyObject.ob_refcnt.
    if _Py_IsImmortal!(op) != 0 {
        _Py_DECREF_IMMORTAL_STAT_INC!();
        return;
    }
    _Py_DECREF_STAT_INC!();
    let __v1: u32 = unsafe { (*op).__slate_anon_0.__slate_anon_1.ob_refcnt };
    let __v2: u32 = __v1.wrapping_sub(1);
    unsafe {
        (*op).__slate_anon_0.__slate_anon_1.ob_refcnt = __v2;
    }
    if __v2 == 0 {
        unsafe { _Py_Dealloc(op) };
    }
}

fn Py_XINCREF(op: *mut _object) {
    if !op.is_null() {
        Py_INCREF!(op);
    }
}

fn Py_XDECREF(op: *mut _object) {
    if !op.is_null() {
        Py_DECREF!(op);
    }
}

fn _Py_NewRef(obj: *mut _object) -> *mut _object {
    Py_INCREF!(obj);
    obj
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_unicodeobject_h_163_2 {
    __slate_bits_0: __slate_bits::__SlateBits135U0,
}

fn _Py_XNewRef(obj: *mut _object) -> *mut _object {
    Py_XINCREF!(obj);
    obj
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_objimpl_h_68 {
    ctx: *mut (),
    alloc: Option<unsafe extern "C-unwind" fn(*mut (), u64) -> *mut ()>,
    free: Option<unsafe extern "C-unwind" fn(*mut (), *mut (), u64)>,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_bytesobject_h_15 {
    ob_base: __SlateRecord_object_h_172,
    ob_shash: i64,
    ob_sval: [i8; 1],
}

fn PyBytes_AS_STRING(op: *mut _object) -> *mut i8 {
    unsafe { (*_PyBytes_CAST!(op)).ob_sval.as_mut_ptr() as *mut i8 }
}

fn PyBytes_GET_SIZE(op: *mut _object) -> i64 {
    let self_: *mut __SlateRecord_bytesobject_h_15 = _PyBytes_CAST!(op);
    Py_SIZE!(self_)
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_unicodeobject_h_163 {
    ob_base: _object,
    length: i64,
    hash: i64,
    state: __SlateRecord_unicodeobject_h_163_2,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_unicodeobject_h_173 {
    _base: __SlateRecord_unicodeobject_h_163,
    utf8_length: i64,
    utf8: *mut i8,
}

// PyUnicode_KIND(): Return one of the PyUnicode_*_KIND values defined above.
//
//
//
// gh-89653: Converting this macro to a static inline function would introduce
//
// new compiler warnings on "kind < PyUnicode_KIND(str)" (compare signed and
//
// unsigned numbers) where kind type is an int or on
//
// "unsigned int kind = PyUnicode_KIND(str)" (cast signed to unsigned).
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pyerrors_h_15 {
    ob_base: _object,
    dict: *mut _object,
    args: *mut _object,
    notes: *mut _object,
    traceback: *mut _object,
    context: *mut _object,
    cause: *mut _object,
    suppress_context: i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _PyLongValue {
    lv_tag: u64,
    ob_digit: [u32; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _longobject {
    ob_base: _object,
    long_value: _PyLongValue,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _PyExecutorObject {}

#[unsafe(no_mangle)]
static mut PyMemoryView_Type: _typeobject = {
    let mut __t0: _typeobject = unsafe { core::mem::zeroed() };
    __t0.ob_base = __SlateRecord_object_h_172 {
        ob_base: _object {
            __slate_anon_0: __SlateRecord_object_h_122 {
                ob_refcnt_full: 1407378104778752i64,
            },
            ob_type: unsafe { core::ptr::addr_of_mut!(PyType_Type) },
        },
        ob_size: 0i64,
    };
    __t0.tp_name = (b"memoryview\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize =
        (core::mem::offset_of!(__SlateRecord_memoryobject_h_36, ob_array) as u64) as i64;
    __t0.tp_itemsize = (core::mem::size_of::<i64>() as u64) as i64;
    __t0.tp_dealloc = Some(memory_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = Some(memory_repr);
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = unsafe { core::ptr::addr_of_mut!(memory_as_sequence) };
    __t0.tp_as_mapping = unsafe { core::ptr::addr_of_mut!(memory_as_mapping) };
    __t0.tp_hash = Some(memory_hash);
    __t0.tp_call = None;
    __t0.tp_str = None;
    __t0.tp_getattro = unsafe {
        core::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
        >(PyObject_GenericGetAttr as *const ())
    };
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = unsafe { core::ptr::addr_of_mut!(memory_as_buffer) };
    __t0.tp_flags = 16416u64;
    __t0.tp_doc = unsafe { core::ptr::addr_of!(memoryview__doc__.0) as *const i8 };
    __t0.tp_traverse = Some(memory_traverse);
    __t0.tp_clear = Some(memory_clear);
    __t0.tp_richcompare = Some(memory_richcompare);
    __t0.tp_weaklistoffset =
        (core::mem::offset_of!(__SlateRecord_memoryobject_h_36, weakreflist) as u64) as i64;
    __t0.tp_iter = Some(memory_iter);
    __t0.tp_iternext = None;
    __t0.tp_methods = unsafe { core::ptr::addr_of_mut!(memory_methods.0) as *mut PyMethodDef };
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = unsafe { core::ptr::addr_of_mut!(memory_getsetlist.0) as *mut PyGetSetDef };
    __t0.tp_base = core::ptr::null_mut::<_typeobject>();
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = 0i64;
    __t0.tp_init = None;
    __t0.tp_alloc = None;
    __t0.tp_new = Some(memoryview);
    __t0
};

// access to exporter blocked
// free format
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_memoryobject_h_16 {
    ob_base: _object,
    flags: i32,
    exports: i64,
    master: __SlateRecord_pybuffer_h_33,
}

// memoryview state flags
// access to master buffer blocked
// C-contiguous layout
// Fortran contiguous layout
// scalar: ndim = 0
// PIL-style layout
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_memoryobject_h_36 {
    ob_base: __SlateRecord_object_h_172,
    mbuf: *mut __SlateRecord_memoryobject_h_16,
    hash: i64,
    flags: i32,
    exports: i64,
    view: __SlateRecord_pybuffer_h_33,
    weakreflist: *mut _object,
    ob_array: [i64; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_tupleobject_h_13 {
    ob_base: __SlateRecord_object_h_172,
    ob_hash: i64,
    ob_item: [*mut _object; 1],
}

fn PyTuple_GET_SIZE(op: *mut _object) -> i64 {
    let tuple: *mut __SlateRecord_tupleobject_h_13 = _PyTuple_CAST!(op);
    Py_SIZE!(tuple)
}

fn PyTuple_SET_ITEM(op: *mut _object, index: i64, value: *mut _object) {
    let tuple: *mut __SlateRecord_tupleobject_h_13 = _PyTuple_CAST!(op);
    unsafe {
        *((*tuple).ob_item.as_mut_ptr() as *mut *mut _object).wrapping_offset(index as isize) =
            value;
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_listobject_h_22 {
    ob_base: __SlateRecord_object_h_172,
    ob_item: *mut *mut _object,
    allocated: i64,
}

fn PyList_GET_SIZE(op: *mut _object) -> i64 {
    let list: *mut __SlateRecord_listobject_h_22 = _PyList_CAST!(op);
    Py_SIZE!(list)
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pystate_h_78 {
    __slate_bits_0: __slate_bits::__SlateBits185U0,
}

fn PyList_SET_ITEM(op: *mut _object, index: i64, value: *mut _object) {
    let list: *mut __SlateRecord_listobject_h_22 = _PyList_CAST!(op);
    unsafe {
        *unsafe { (*list).ob_item }.wrapping_offset(index as isize) = value;
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct PyMethodDef {
    ml_name: *const i8,
    ml_meth: Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
    ml_flags: i32,
    ml_doc: *const i8,
}

// Flag passed to newmethodobject
//
// #define METH_OLDARGS  0x0000   -- unsupported now
// METH_NOARGS and METH_O must not be combined with the flags above.
// METH_CLASS and METH_STATIC are a little different; these control
// the construction of methods for a class.  These cannot be used for
// functions in modules.
// NULL or a tuple
// NULL or a dict
// NULL or a tuple of cell objects
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_funcobject_h_62 {
    ob_base: _object,
    func_globals: *mut _object,
    func_builtins: *mut _object,
    func_name: *mut _object,
    func_qualname: *mut _object,
    func_code: *mut _object,
    func_defaults: *mut _object,
    func_kwdefaults: *mut _object,
    func_closure: *mut _object,
    func_doc: *mut _object,
    func_dict: *mut _object,
    func_weakreflist: *mut _object,
    func_module: *mut _object,
    func_annotations: *mut _object,
    func_annotate: *mut _object,
    func_typeparams: *mut _object,
    vectorcall: Option<
        unsafe extern "C-unwind" fn(
            *mut _object,
            *const *mut _object,
            u64,
            *mut _object,
        ) -> *mut _object,
    >,
    func_version: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_code_h_16 {
    _co_code: *mut _object,
    _co_varnames: *mut _object,
    _co_cellvars: *mut _object,
    _co_freevars: *mut _object,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_code_h_22 {
    size: i32,
    capacity: i32,
    executors: [*mut _PyExecutorObject; 1],
}

// To avoid repeating ourselves in deepfreeze.py, all PyCodeObject members are
//
// defined in this macro:
// Note only the following fields are used in hash and/or comparisons           *                                                                              * - co_name                                                                    * - co_argcount                                                                * - co_posonlyargcount                                                         * - co_kwonlyargcount                                                          * - co_nlocals                                                                 * - co_stacksize                                                               * - co_flags                                                                   * - co_firstlineno                                                             * - co_consts                                                                  * - co_names                                                                   * - co_localsplusnames                                                         * This is done to preserve the name and line number for tracebacks             * and debuggers; otherwise, constant de-duplication would collapse             * identical functions/lambdas defined on different lines.
// These fields are set with provided values on new code objects.
// The hottest fields (in the eval loop) are grouped here at the top.
// list (constants used)
// list of strings (names used)
// Byte string encoding exception handling                                        table
// CO_..., see below
// The rest are not so impactful on performance.
// #arguments, except *args
// #positional only arguments
// #keyword only arguments
// #entries needed for evaluation stack
// first source line number
// redundant values (derived from co_localsplusnames and                          co_localspluskinds)
// number of spaces for holding local, cell,                                      and free variables
// Size of frame in words
// number of local variables
// total number of cell variables
// number of free variables
// version number
// tuple mapping offsets to names
// Bytes mapping to local kinds (one byte                                         per variable)
// unicode (where it was loaded from)
// unicode (name, for reference)
// unicode (qualname, for reference)
// bytes object that holds location info
// to support weakrefs to code objects
// executors from optimizer
// cached co_* attributes
// current instrumentation version
// Monitoring data
// ID used for per-thread refcounting
// index of first traceable instruction
// Scratch space for extra data relating to the code object.                      Type is a void* to keep the format private in codeobject.c to force            people to go through the proper APIs.
#[repr(C)]
#[derive(Clone, Copy)]
struct PyCodeObject {
    ob_base: __SlateRecord_object_h_172,
    co_consts: *mut _object,
    co_names: *mut _object,
    co_exceptiontable: *mut _object,
    co_flags: i32,
    co_argcount: i32,
    co_posonlyargcount: i32,
    co_kwonlyargcount: i32,
    co_stacksize: i32,
    co_firstlineno: i32,
    co_nlocalsplus: i32,
    co_framesize: i32,
    co_nlocals: i32,
    co_ncellvars: i32,
    co_nfreevars: i32,
    co_version: u32,
    co_localsplusnames: *mut _object,
    co_localspluskinds: *mut _object,
    co_filename: *mut _object,
    co_name: *mut _object,
    co_qualname: *mut _object,
    co_linetable: *mut _object,
    co_weakreflist: *mut _object,
    co_executors: *mut __SlateRecord_code_h_22,
    _co_cached: *mut __SlateRecord_code_h_16,
    _co_instrumentation_version: u64,
    _co_monitoring: *mut _PyCoMonitoringData,
    _co_unique_id: i64,
    _co_firsttraceable: i32,
    co_extra: *mut (),
    co_code_adaptive: [i8; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_initconfig_h_36 {
    length: i64,
    items: *mut *mut i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct PyPreConfig {
    _config_init: i32,
    parse_argv: i32,
    isolated: i32,
    use_environment: i32,
    configure_locale: i32,
    coerce_c_locale: i32,
    coerce_c_locale_warn: i32,
    utf8_mode: i32,
    dev_mode: i32,
    allocator: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct PyConfig {
    _config_init: i32,
    isolated: i32,
    use_environment: i32,
    dev_mode: i32,
    install_signal_handlers: i32,
    use_hash_seed: i32,
    hash_seed: u64,
    faulthandler: i32,
    tracemalloc: i32,
    perf_profiling: i32,
    remote_debug: i32,
    import_time: i32,
    code_debug_ranges: i32,
    show_ref_count: i32,
    dump_refs: i32,
    dump_refs_file: *mut i32,
    malloc_stats: i32,
    filesystem_encoding: *mut i32,
    filesystem_errors: *mut i32,
    pycache_prefix: *mut i32,
    parse_argv: i32,
    orig_argv: __SlateRecord_initconfig_h_36,
    argv: __SlateRecord_initconfig_h_36,
    xoptions: __SlateRecord_initconfig_h_36,
    warnoptions: __SlateRecord_initconfig_h_36,
    site_import: i32,
    bytes_warning: i32,
    warn_default_encoding: i32,
    inspect: i32,
    interactive: i32,
    optimization_level: i32,
    parser_debug: i32,
    write_bytecode: i32,
    verbose: i32,
    quiet: i32,
    user_site_directory: i32,
    configure_c_stdio: i32,
    buffered_stdio: i32,
    stdio_encoding: *mut i32,
    stdio_errors: *mut i32,
    check_hash_pycs_mode: *mut i32,
    use_frozen_modules: i32,
    safe_path: i32,
    int_max_str_digits: i32,
    thread_inherit_context: i32,
    context_aware_warnings: i32,
    cpu_count: i32,
    pathconfig_warnings: i32,
    program_name: *mut i32,
    pythonpath_env: *mut i32,
    home: *mut i32,
    platlibdir: *mut i32,
    module_search_paths_set: i32,
    module_search_paths: __SlateRecord_initconfig_h_36,
    stdlib_dir: *mut i32,
    executable: *mut i32,
    base_executable: *mut i32,
    prefix: *mut i32,
    base_prefix: *mut i32,
    exec_prefix: *mut i32,
    base_exec_prefix: *mut i32,
    skip_source_first_line: i32,
    run_command: *mut i32,
    run_module: *mut i32,
    run_filename: *mut i32,
    sys_path_0: *mut i32,
    _install_importlib: i32,
    _init_main: i32,
    _is_python_build: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pystate_h_35 {
    debugger_pending_call: i32,
    debugger_script_path: [i8; 512],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _err_stackitem {
    exc_value: *mut _object,
    previous_item: *mut _err_stackitem,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _stack_chunk {
    previous: *mut _stack_chunk,
    size: u64,
    top: u64,
    data: [*mut _object; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _ts {
    prev: *mut _ts,
    next: *mut _ts,
    interp: *mut _is,
    eval_breaker: u64,
    _status: __SlateRecord_pystate_h_78,
    holds_gil: i32,
    _whence: i32,
    state: i32,
    py_recursion_remaining: i32,
    py_recursion_limit: i32,
    recursion_headroom: i32,
    tracing: i32,
    what_event: i32,
    current_frame: *mut _PyInterpreterFrame,
    c_profilefunc:
        Option<unsafe extern "C-unwind" fn(*mut _object, *mut _frame, i32, *mut _object) -> i32>,
    c_tracefunc:
        Option<unsafe extern "C-unwind" fn(*mut _object, *mut _frame, i32, *mut _object) -> i32>,
    c_profileobj: *mut _object,
    c_traceobj: *mut _object,
    current_exception: *mut _object,
    exc_info: *mut _err_stackitem,
    dict: *mut _object,
    gilstate_counter: i32,
    async_exc: *mut _object,
    thread_id: u64,
    native_thread_id: u64,
    delete_later: *mut _object,
    critical_section: u64,
    coroutine_origin_tracking_depth: i32,
    async_gen_firstiter: *mut _object,
    async_gen_finalizer: *mut _object,
    context: *mut _object,
    context_ver: u64,
    id: u64,
    datastack_chunk: *mut _stack_chunk,
    datastack_top: *mut *mut _object,
    datastack_limit: *mut *mut _object,
    exc_state: _err_stackitem,
    current_executor: *mut _object,
    dict_global_version: u64,
    threading_local_key: *mut _object,
    threading_local_sentinel: *mut _object,
    remote_debugger_support: __SlateRecord_pystate_h_35,
    datastack_cached_chunk: *mut _stack_chunk,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_structs_h_32_2 {
    code: u8,
    arg: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct PyGetSetDef {
    name: *const i8,
    get: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> *mut _object>,
    set: Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object, *mut ()) -> i32>,
    doc: *const i8,
    closure: *mut (),
}

#[repr(C)]
#[derive(Clone, Copy)]
struct PyMemberDef {
    name: *const i8,
    r#type: i32,
    offset: i64,
    flags: i32,
    doc: *const i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct wrapperbase {
    name: *const i8,
    offset: i32,
    function: *mut (),
    wrapper:
        Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object, *mut ()) -> *mut _object>,
    doc: *const i8,
    flags: i32,
    name_strobj: *mut _object,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _Py_tss_t {
    _is_initialized: i32,
    _key: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_modsupport_h_9 {
    v: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _PyArg_Parser {
    format: *const i8,
    keywords: *const *const i8,
    fname: *const i8,
    custom_msg: *const i8,
    once: __SlateRecord_modsupport_h_9,
    is_kwtuple_owned: i32,
    pos: i32,
    min: i32,
    max: i32,
    kwtuple: *mut _object,
    next: *mut _PyArg_Parser,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _inittab {
    name: *const i8,
    initfunc: Option<unsafe extern "C-unwind" fn() -> *mut _object>,
}

fn _PyIndex_Check(obj: *mut _object) -> i32 {
    let tp_as_number: *mut __SlateRecord_object_h_106 = unsafe { (*Py_TYPE!(obj)).tp_as_number };
    (!tp_as_number.is_null() && (unsafe { (*tp_as_number).nb_index }) != None) as i32
}

#[unsafe(no_mangle)]
static mut _PyManagedBuffer_Type: _typeobject = {
    let mut __t0: _typeobject = unsafe { core::mem::zeroed() };
    __t0.ob_base = __SlateRecord_object_h_172 {
        ob_base: _object {
            __slate_anon_0: __SlateRecord_object_h_122 {
                ob_refcnt_full: 1407378104778752i64,
            },
            ob_type: unsafe { core::ptr::addr_of_mut!(PyType_Type) },
        },
        ob_size: 0i64,
    };
    __t0.tp_name = (b"managedbuffer\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_memoryobject_h_16>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(mbuf_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = None;
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = None;
    __t0.tp_getattro = unsafe {
        core::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
        >(PyObject_GenericGetAttr as *const ())
    };
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 16384u64;
    __t0.tp_doc = core::ptr::null::<i8>();
    __t0.tp_traverse = Some(mbuf_traverse);
    __t0.tp_clear = Some(mbuf_clear);
    __t0
};

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_lock_h_69 {
    v: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_lock_h_153 {
    mutex: PyMutex,
    thread: u64,
    level: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_lock_h_189 {
    bits: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct ast_state {
    once: __SlateRecord_modsupport_h_9,
    finalized: i32,
    AST_type: *mut _object,
    Add_singleton: *mut _object,
    Add_type: *mut _object,
    And_singleton: *mut _object,
    And_type: *mut _object,
    AnnAssign_type: *mut _object,
    Assert_type: *mut _object,
    Assign_type: *mut _object,
    AsyncFor_type: *mut _object,
    AsyncFunctionDef_type: *mut _object,
    AsyncWith_type: *mut _object,
    Attribute_type: *mut _object,
    AugAssign_type: *mut _object,
    Await_type: *mut _object,
    BinOp_type: *mut _object,
    BitAnd_singleton: *mut _object,
    BitAnd_type: *mut _object,
    BitOr_singleton: *mut _object,
    BitOr_type: *mut _object,
    BitXor_singleton: *mut _object,
    BitXor_type: *mut _object,
    BoolOp_type: *mut _object,
    Break_type: *mut _object,
    Call_type: *mut _object,
    ClassDef_type: *mut _object,
    Compare_type: *mut _object,
    Constant_type: *mut _object,
    Continue_type: *mut _object,
    Del_singleton: *mut _object,
    Del_type: *mut _object,
    Delete_type: *mut _object,
    DictComp_type: *mut _object,
    Dict_type: *mut _object,
    Div_singleton: *mut _object,
    Div_type: *mut _object,
    Eq_singleton: *mut _object,
    Eq_type: *mut _object,
    ExceptHandler_type: *mut _object,
    Expr_type: *mut _object,
    Expression_type: *mut _object,
    FloorDiv_singleton: *mut _object,
    FloorDiv_type: *mut _object,
    For_type: *mut _object,
    FormattedValue_type: *mut _object,
    FunctionDef_type: *mut _object,
    FunctionType_type: *mut _object,
    GeneratorExp_type: *mut _object,
    Global_type: *mut _object,
    GtE_singleton: *mut _object,
    GtE_type: *mut _object,
    Gt_singleton: *mut _object,
    Gt_type: *mut _object,
    IfExp_type: *mut _object,
    If_type: *mut _object,
    ImportFrom_type: *mut _object,
    Import_type: *mut _object,
    In_singleton: *mut _object,
    In_type: *mut _object,
    Interactive_type: *mut _object,
    Interpolation_type: *mut _object,
    Invert_singleton: *mut _object,
    Invert_type: *mut _object,
    IsNot_singleton: *mut _object,
    IsNot_type: *mut _object,
    Is_singleton: *mut _object,
    Is_type: *mut _object,
    JoinedStr_type: *mut _object,
    LShift_singleton: *mut _object,
    LShift_type: *mut _object,
    Lambda_type: *mut _object,
    ListComp_type: *mut _object,
    List_type: *mut _object,
    Load_singleton: *mut _object,
    Load_type: *mut _object,
    LtE_singleton: *mut _object,
    LtE_type: *mut _object,
    Lt_singleton: *mut _object,
    Lt_type: *mut _object,
    MatMult_singleton: *mut _object,
    MatMult_type: *mut _object,
    MatchAs_type: *mut _object,
    MatchClass_type: *mut _object,
    MatchMapping_type: *mut _object,
    MatchOr_type: *mut _object,
    MatchSequence_type: *mut _object,
    MatchSingleton_type: *mut _object,
    MatchStar_type: *mut _object,
    MatchValue_type: *mut _object,
    Match_type: *mut _object,
    Mod_singleton: *mut _object,
    Mod_type: *mut _object,
    Module_type: *mut _object,
    Mult_singleton: *mut _object,
    Mult_type: *mut _object,
    Name_type: *mut _object,
    NamedExpr_type: *mut _object,
    Nonlocal_type: *mut _object,
    NotEq_singleton: *mut _object,
    NotEq_type: *mut _object,
    NotIn_singleton: *mut _object,
    NotIn_type: *mut _object,
    Not_singleton: *mut _object,
    Not_type: *mut _object,
    Or_singleton: *mut _object,
    Or_type: *mut _object,
    ParamSpec_type: *mut _object,
    Pass_type: *mut _object,
    Pow_singleton: *mut _object,
    Pow_type: *mut _object,
    RShift_singleton: *mut _object,
    RShift_type: *mut _object,
    Raise_type: *mut _object,
    Return_type: *mut _object,
    SetComp_type: *mut _object,
    Set_type: *mut _object,
    Slice_type: *mut _object,
    Starred_type: *mut _object,
    Store_singleton: *mut _object,
    Store_type: *mut _object,
    Sub_singleton: *mut _object,
    Sub_type: *mut _object,
    Subscript_type: *mut _object,
    TemplateStr_type: *mut _object,
    TryStar_type: *mut _object,
    Try_type: *mut _object,
    Tuple_type: *mut _object,
    TypeAlias_type: *mut _object,
    TypeIgnore_type: *mut _object,
    TypeVarTuple_type: *mut _object,
    TypeVar_type: *mut _object,
    UAdd_singleton: *mut _object,
    UAdd_type: *mut _object,
    USub_singleton: *mut _object,
    USub_type: *mut _object,
    UnaryOp_type: *mut _object,
    While_type: *mut _object,
    With_type: *mut _object,
    YieldFrom_type: *mut _object,
    Yield_type: *mut _object,
    __dict__: *mut _object,
    __doc__: *mut _object,
    __match_args__: *mut _object,
    __module__: *mut _object,
    _attributes: *mut _object,
    _fields: *mut _object,
    alias_type: *mut _object,
    annotation: *mut _object,
    arg: *mut _object,
    arg_type: *mut _object,
    args: *mut _object,
    argtypes: *mut _object,
    arguments_type: *mut _object,
    asname: *mut _object,
    ast: *mut _object,
    attr: *mut _object,
    bases: *mut _object,
    body: *mut _object,
    boolop_type: *mut _object,
    bound: *mut _object,
    cases: *mut _object,
    cause: *mut _object,
    cls: *mut _object,
    cmpop_type: *mut _object,
    col_offset: *mut _object,
    comparators: *mut _object,
    comprehension_type: *mut _object,
    context_expr: *mut _object,
    conversion: *mut _object,
    ctx: *mut _object,
    decorator_list: *mut _object,
    default_value: *mut _object,
    defaults: *mut _object,
    elt: *mut _object,
    elts: *mut _object,
    end_col_offset: *mut _object,
    end_lineno: *mut _object,
    exc: *mut _object,
    excepthandler_type: *mut _object,
    expr_context_type: *mut _object,
    expr_type: *mut _object,
    finalbody: *mut _object,
    format_spec: *mut _object,
    func: *mut _object,
    generators: *mut _object,
    guard: *mut _object,
    handlers: *mut _object,
    id: *mut _object,
    ifs: *mut _object,
    is_async: *mut _object,
    items: *mut _object,
    iter: *mut _object,
    key: *mut _object,
    keys: *mut _object,
    keyword_type: *mut _object,
    keywords: *mut _object,
    kind: *mut _object,
    kw_defaults: *mut _object,
    kwarg: *mut _object,
    kwd_attrs: *mut _object,
    kwd_patterns: *mut _object,
    kwonlyargs: *mut _object,
    left: *mut _object,
    level: *mut _object,
    lineno: *mut _object,
    lower: *mut _object,
    match_case_type: *mut _object,
    mod_type: *mut _object,
    module: *mut _object,
    msg: *mut _object,
    name: *mut _object,
    names: *mut _object,
    op: *mut _object,
    operand: *mut _object,
    operator_type: *mut _object,
    ops: *mut _object,
    optional_vars: *mut _object,
    orelse: *mut _object,
    pattern: *mut _object,
    pattern_type: *mut _object,
    patterns: *mut _object,
    posonlyargs: *mut _object,
    rest: *mut _object,
    returns: *mut _object,
    right: *mut _object,
    simple: *mut _object,
    slice: *mut _object,
    step: *mut _object,
    stmt_type: *mut _object,
    str: *mut _object,
    subject: *mut _object,
    tag: *mut _object,
    target: *mut _object,
    targets: *mut _object,
    test: *mut _object,
    r#type: *mut _object,
    type_comment: *mut _object,
    type_ignore_type: *mut _object,
    type_ignores: *mut _object,
    type_param_type: *mut _object,
    type_params: *mut _object,
    unaryop_type: *mut _object,
    upper: *mut _object,
    value: *mut _object,
    values: *mut _object,
    vararg: *mut _object,
    withitem_type: *mut _object,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct code_arena_st {}

#[repr(C)]
#[derive(Clone, Copy)]
struct llist_node {
    next: *mut llist_node,
    prev: *mut llist_node,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_structs_h_15 {
    value_and_backoff: u16,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_interp_structs_h_81 {
    status: i32,
    perf_trampoline_type: i32,
    extra_code_index: i64,
    code_arena: *mut code_arena_st,
    trampoline_api: trampoline_api_st,
    map_file: *mut slate_support::FILE,
    persist_after_fork: i64,
    prev_eval_frame: Option<
        unsafe extern "C-unwind" fn(*mut _ts, *mut _PyInterpreterFrame, i32) -> *mut _object,
    >,
    trampoline_refcount: i64,
    code_watcher_id: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord_pycore_structs_h_32 {
    cache: u16,
    op: __SlateRecord_pycore_structs_h_32_2,
    counter: __SlateRecord_pycore_structs_h_15,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_structs_h_38 {
    ob_base: _object,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_structs_h_47 {
    ob_base: _object,
    h_root: *mut __SlateRecord_pycore_structs_h_38,
    h_weakreflist: *mut _object,
    h_count: i64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_structs_h_53 {
    ob_base: __SlateRecord_object_h_172,
    b_bitmap: u32,
    b_array: [*mut _object; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_context_h_21 {
    ob_base: _object,
}

#[repr(C)]
#[derive(Clone, Copy)]
union _PyStackRef {
    bits: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _Py_freelist {
    freelist: *mut (),
    size: i64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _Py_freelists {
    floats: _Py_freelist,
    ints: _Py_freelist,
    tuples: [_Py_freelist; 20],
    lists: _Py_freelist,
    list_iters: _Py_freelist,
    tuple_iters: _Py_freelist,
    dicts: _Py_freelist,
    dictkeys: _Py_freelist,
    slices: _Py_freelist,
    ranges: _Py_freelist,
    range_iters: _Py_freelist,
    contexts: _Py_freelist,
    async_gens: _Py_freelist,
    async_gen_asends: _Py_freelist,
    futureiters: _Py_freelist,
    object_stack_chunks: _Py_freelist,
    unicode_writers: _Py_freelist,
    pycfunctionobject: _Py_freelist,
    pycmethodobject: _Py_freelist,
    pymethodobjects: _Py_freelist,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _qsbr_thread_state {
    seq: u64,
    shared: *mut _qsbr_shared,
    tstate: *mut _ts,
    deferred_count: i32,
    deferred_memory: u64,
    deferred_page_memory: u64,
    should_process: bool,
    allocated: bool,
    freelist_next: *mut _qsbr_thread_state,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _qsbr_pad {
    qsbr: _qsbr_thread_state,
    __padding: [i8; 0],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_pythread_h_64 {
    ptr: *mut __SlateRecord_pthreadtypes_h_45,
    val: __SlateRecord_pthreadtypes_h_45,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _qsbr_shared {
    wr_seq: u64,
    rd_seq: u64,
    array: *mut _qsbr_pad,
    array_raw: *mut (),
    size: i64,
    mutex: PyMutex,
    freelist: *mut _qsbr_thread_state,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_interp_structs_h_297 {
    mutex: PyMutex,
    hashtable: *mut _Py_hashtable_t,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _PyThreadStateImpl {
    base: _ts,
    refcount: i64,
    c_stack_top: u64,
    c_stack_soft_limit: u64,
    c_stack_hard_limit: u64,
    asyncio_running_loop: *mut _object,
    asyncio_running_task: *mut _object,
    asyncio_tasks_head: llist_node,
    qsbr: *mut _qsbr_thread_state,
    mem_free_queue: llist_node,
    c_stack_init_base: u64,
    c_stack_init_top: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _pending_call {
    func: Option<unsafe extern "C-unwind" fn(*mut ()) -> i32>,
    arg: *mut (),
    flags: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_interp_structs_h_348 {
    import_level: i32,
    accumulated: i64,
    header: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _pending_calls {
    handling_thread: *mut _ts,
    mutex: PyMutex,
    npending: i32,
    max: i32,
    maxloop: i32,
    calls: [_pending_call; 300],
    first: i32,
    next: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct trampoline_api_st {
    init_state: Option<unsafe extern "C-unwind" fn() -> *mut ()>,
    write_state: Option<unsafe extern "C-unwind" fn(*mut (), *const (), u32, *mut PyCodeObject)>,
    free_state: Option<unsafe extern "C-unwind" fn(*mut ()) -> i32>,
    state: *mut (),
    code_padding: i64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _ceval_runtime_state {
    perf: __SlateRecord_pycore_interp_structs_h_81,
    pending_mainthread: _pending_calls,
    unused_sys_trace_profile_mutex: PyMutex,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _ceval_state {
    instrumentation_version: u64,
    recursion_limit: i32,
    gil: *mut _gil_runtime_state,
    own_gil: i32,
    pending: _pending_calls,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _atexit_runtime_state {
    mutex: PyMutex,
    callbacks: [Option<unsafe extern "C-unwind" fn()>; 32],
    ncallbacks: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct atexit_callback {
    func: Option<unsafe extern "C-unwind" fn(*mut ())>,
    data: *mut (),
    next: *mut atexit_callback,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct atexit_state {
    ll_callbacks: *mut atexit_callback,
    callbacks: *mut _object,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_interp_structs_h_170 {
    _gc_next: u64,
    _gc_prev: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct gc_generation {
    head: __SlateRecord_pycore_interp_structs_h_170,
    threshold: i32,
    count: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct gc_generation_stats {
    collections: i64,
    collected: i64,
    uncollectable: i64,
}

// If we change this, we need to change the default value in the
// signature of gc.collect.
#[repr(C)]
#[derive(Clone, Copy)]
struct _gc_runtime_state {
    trash_delete_later: *mut _object,
    trash_delete_nesting: i32,
    enabled: i32,
    debug: i32,
    generations: [gc_generation; 3],
    permanent_generation: gc_generation,
    generation_stats: [gc_generation_stats; 3],
    collecting: i32,
    garbage: *mut _object,
    callbacks: *mut _object,
    heap_size: i64,
    dummy1: i64,
    dummy2: i32,
    dummy3: i32,
    long_lived_total: i64,
    long_lived_pending: i64,
    generation0: *mut __SlateRecord_pycore_interp_structs_h_170,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _pythread_runtime_state {
    initialized: i32,
    _condattr_monotonic: __SlateRecord_pycore_pythread_h_64,
    handles: llist_node,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _gil_runtime_state {
    interval: u64,
    last_holder: *mut _ts,
    locked: i32,
    switch_number: u64,
    cond: __SlateRecord_pthreadtypes_h_80,
    mutex: __SlateRecord_pthreadtypes_h_72,
    switch_cond: __SlateRecord_pthreadtypes_h_80,
    switch_mutex: __SlateRecord_pthreadtypes_h_72,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _import_runtime_state {
    inittab: *mut _inittab,
    last_module_index: i64,
    extensions: __SlateRecord_pycore_interp_structs_h_297,
    pkgcontext: *const i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct xi_exceptions {
    PyExc_InterpreterError: *mut _object,
    PyExc_InterpreterNotFoundError: *mut _object,
    PyExc_NotShareableError: *mut _object,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _import_state {
    modules: *mut _object,
    modules_by_index: *mut _object,
    importlib: *mut _object,
    override_frozen_modules: i32,
    override_multi_interp_extensions_check: i32,
    dlopenflags: i32,
    import_func: *mut _object,
    lock: __SlateRecord_pycore_lock_h_153,
    find_and_load: __SlateRecord_pycore_interp_structs_h_348,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _Py_slist_item_s {
    next: *mut _Py_slist_item_s,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_hashtable_h_19 {
    head: *mut _Py_slist_item_s,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_hashtable_h_35 {
    _Py_slist_item: _Py_slist_item_s,
    key_hash: u64,
    key: *mut (),
    value: *mut (),
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_hashtable_h_56 {
    malloc: Option<unsafe extern "C-unwind" fn(u64) -> *mut ()>,
    free: Option<unsafe extern "C-unwind" fn(*mut ())>,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _Py_hashtable_t {
    nentries: u64,
    nbuckets: u64,
    buckets: *mut __SlateRecord_pycore_hashtable_h_19,
    get_entry_func: Option<
        unsafe extern "C-unwind" fn(
            *mut _Py_hashtable_t,
            *const (),
        ) -> *mut __SlateRecord_pycore_hashtable_h_35,
    >,
    hash_func: Option<unsafe extern "C-unwind" fn(*const ()) -> u64>,
    compare_func: Option<unsafe extern "C-unwind" fn(*const (), *const ()) -> i32>,
    key_destroy_func: Option<unsafe extern "C-unwind" fn(*mut ())>,
    value_destroy_func: Option<unsafe extern "C-unwind" fn(*mut ())>,
    alloc: __SlateRecord_pycore_hashtable_h_56,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _reftracer_runtime_state {
    tracer_func: Option<unsafe extern "C-unwind" fn(*mut _object, u32, *mut ()) -> i32>,
    tracer_data: *mut (),
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _py_object_runtime_state {
    _not_used: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _py_object_state {
    freelists: _Py_freelists,
    _not_used: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _xidata {
    data: *mut (),
    obj: *mut _object,
    interpid: i64,
    new_object: Option<unsafe extern "C-unwind" fn(*mut _xidata) -> *mut _object>,
    free: Option<unsafe extern "C-unwind" fn(*mut ())>,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_crossinterp_h_150 {
    basic: Option<unsafe extern "C-unwind" fn(*mut _ts, *mut _object, *mut _xidata) -> i32>,
    fallback: Option<unsafe extern "C-unwind" fn(*mut _ts, *mut _object, i32, *mut _xidata) -> i32>,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _xid_regitem {
    prev: *mut _xid_regitem,
    next: *mut _xid_regitem,
    cls: *mut _typeobject,
    weakref: *mut _object,
    refcount: u64,
    getdata: __SlateRecord_pycore_crossinterp_h_150,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_interp_structs_h_504 {
    types: [__SlateRecord_pycore_interp_structs_h_505; 210],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_interp_structs_h_505 {
    r#type: *mut _typeobject,
    interp_count: i64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_crossinterp_data_registry_h_28 {
    global: i32,
    initialized: i32,
    mutex: PyMutex,
    head: *mut _xid_regitem,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _xid_lookup_state {
    registry: __SlateRecord_pycore_crossinterp_data_registry_h_28,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_crossinterp_h_255 {
    data_lookup: _xid_lookup_state,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_crossinterp_h_268 {
    data_lookup: _xid_lookup_state,
    exceptions: xi_exceptions,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_interp_structs_h_592 {
    num_initialized: u64,
    initialized: [__SlateRecord_pycore_interp_structs_h_544; 200],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_interp_structs_h_597 {
    num_initialized: u64,
    next_index: u64,
    initialized: [__SlateRecord_pycore_interp_structs_h_544; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _Py_long_state {
    max_str_digits: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct codecs_state {
    search_path: *mut _object,
    search_cache: *mut _object,
    error_registry: *mut _object,
    initialized: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _stoptheworld_state {
    mutex: PyMutex,
    requested: bool,
    world_stopped: bool,
    is_global: bool,
    stop_event: __SlateRecord_pycore_lock_h_69,
    thread_countdown: i64,
    requester: *mut _ts,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _rare_events {
    set_class: u8,
    set_bases: u8,
    set_eval_frame_func: u8,
    builtin_dict: u8,
    func_modification: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Bigint {
    next: *mut Bigint,
    k: i32,
    maxwds: i32,
    sign: i32,
    wds: i32,
    x: [u32; 1],
}

// The size of the Bigint freelist
// The size of the cached powers of 5 array
#[repr(C)]
#[derive(Clone, Copy)]
struct _dtoa_state {
    p5s: [*mut Bigint; 8],
    freelist: [*mut Bigint; 8],
    preallocated: [f64; 288],
    preallocated_next: *mut f64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _py_code_state {
    mutex: PyMutex,
    constants: *mut _Py_hashtable_t,
}

// Must be a power of 2
#[repr(C)]
#[derive(Clone, Copy)]
struct _func_version_cache_item {
    func: *mut __SlateRecord_funcobject_h_62,
    code: *mut _object,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _py_func_state {
    next_version: u32,
    func_version_cache: [_func_version_cache_item; 4096],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _Py_dict_state {
    next_keys_version: u32,
    watcher_mutex: PyMutex,
    watcher_setup_once: __SlateRecord_modsupport_h_9,
    watchers: [Option<
        unsafe extern "C-unwind" fn(u32, *mut _object, *mut _object, *mut _object) -> i32,
    >; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _Py_exc_state {
    errnomap: *mut _object,
    memerrors_freelist: *mut __SlateRecord_pyerrors_h_15,
    memerrors_numfree: i32,
    PyExc_ExceptionGroup: *mut _object,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_interp_structs_h_718 {
    _not_used: i32,
    _hamt_empty_gc_not_used: __SlateRecord_pycore_interp_structs_h_170,
    hamt_empty: __SlateRecord_pycore_structs_h_47,
    last_resort_memory_error: __SlateRecord_pyerrors_h_15,
}

// For now we hard-code this to a value for which we are confident
// all the static builtin types will fit (for all builds).
#[repr(C)]
#[derive(Clone, Copy)]
struct _types_runtime_state {
    next_version_tag: u32,
    managed_static: __SlateRecord_pycore_interp_structs_h_504,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct type_cache_entry {
    version: u32,
    name: *mut _object,
    value: *mut _object,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct type_cache {
    hashtable: [type_cache_entry; 4096],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct pythreads {
    next_unique_id: u64,
    head: *mut _ts,
    preallocated: *mut _PyThreadStateImpl,
    main: *mut _ts,
    count: i64,
    stacksize: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_interp_structs_h_544 {
    r#type: *mut _typeobject,
    isbuiltin: i32,
    readying: i32,
    ready: i32,
    tp_dict: *mut _object,
    tp_subclasses: *mut _object,
    tp_weaklist: *mut _object,
}

// Must be a power of 2
#[repr(C)]
#[derive(Clone, Copy)]
struct types_state {
    next_version_tag: u32,
    type_cache: type_cache,
    builtins: __SlateRecord_pycore_interp_structs_h_592,
    for_extensions: __SlateRecord_pycore_interp_structs_h_597,
    mutex: PyMutex,
    type_version_cache: [*mut _typeobject; 4096],
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord_pycore_obmalloc_h_255 {
    _padding: *mut u8,
    count: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _warnings_runtime_state {
    filters: *mut _object,
    once_registry: *mut _object,
    default_action: *mut _object,
    lock: __SlateRecord_pycore_lock_h_153,
    filters_version: i64,
    context: *mut _object,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _Py_mem_interp_free_queue {
    has_work: i32,
    mutex: PyMutex,
    head: llist_node,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _Py_unicode_runtime_ids {
    mutex: PyMutex,
    next_index: i64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _Py_unicode_runtime_state {
    ids: _Py_unicode_runtime_ids,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _Py_unicode_fs_codec {
    encoding: *mut i8,
    utf8: i32,
    errors: *mut i8,
    error_handler: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _Py_unicode_ids {
    size: i64,
    array: *mut *mut _object,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ucnhash_h_29 {
    getname: Option<unsafe extern "C-unwind" fn(u32, *mut i8, i32, i32) -> i32>,
    getcode: Option<unsafe extern "C-unwind" fn(*const i8, i32, *mut u32, i32) -> i32>,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _Py_unicode_state {
    fs_codec: _Py_unicode_fs_codec,
    ucnhash_capi: *mut __SlateRecord_pycore_ucnhash_h_29,
    ids: _Py_unicode_ids,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct callable_cache {
    isinstance: *mut _object,
    len: *mut _object,
    list_append: *mut _object,
    object__getattribute__: *mut _object,
}

// Length of array of slotdef pointers used to store slots with the
// same __name__.  There should be at most MAX_EQUIV-1 slotdef entries with
// the same __name__, for any __name__. Since that's a static property, it is
// appropriate to declare fixed-size arrays for this.
#[repr(C)]
#[derive(Clone, Copy)]
struct _Py_interp_cached_objects {
    interned_strings: *mut _object,
    objreduce: *mut _object,
    type_slots_pname: *mut _object,
    type_slots_ptrs: [*mut wrapperbase; 10],
    generic_type: *mut _typeobject,
    typevar_type: *mut _typeobject,
    typevartuple_type: *mut _typeobject,
    paramspec_type: *mut _typeobject,
    paramspecargs_type: *mut _typeobject,
    paramspeckwargs_type: *mut _typeobject,
    constevaluator_type: *mut _typeobject,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _Py_interp_static_objects {
    singletons: __SlateRecord_pycore_interp_structs_h_718,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_runtime_structs_h_23 {
    raw: __SlateRecord_pymem_h_48,
    mem: __SlateRecord_pymem_h_48,
    obj: __SlateRecord_pymem_h_48,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_runtime_structs_h_28 {
    raw: __SlateRecord_pycore_runtime_structs_h_19,
    mem: __SlateRecord_pycore_runtime_structs_h_19,
    obj: __SlateRecord_pycore_runtime_structs_h_19,
}

// Count of all local monitoring events
// Count of all "real" monitoring events (not derived from other events)
// Count of all  monitoring events
#[repr(C)]
#[derive(Clone, Copy)]
struct _Py_LocalMonitors {
    tools: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _Py_GlobalMonitors {
    tools: [u8; 16],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_runtime_structs_h_49 {
    fd: i32,
    st_dev: u64,
    st_ino: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_instruments_h_94 {
    bytes_per_entry: u8,
    data: [u8; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _PyCoMonitoringData {
    local_monitors: _Py_LocalMonitors,
    active_monitors: _Py_LocalMonitors,
    tools: *mut u8,
    tool_versions: [u64; 8],
    lines: *mut __SlateRecord_pycore_instruments_h_94,
    line_tools: *mut u8,
    per_instruction_opcodes: *mut u8,
    per_instruction_tools: *mut u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _is {
    ceval: _ceval_state,
    _malloced: *mut (),
    next: *mut _is,
    id: i64,
    id_refcount: i64,
    requires_idref: i32,
    _whence: i64,
    _initialized: i32,
    _ready: i32,
    finalizing: i32,
    last_restart_version: u64,
    threads: pythreads,
    runtime: *mut pyruntimestate,
    _finalizing: *mut _ts,
    _finalizing_id: u64,
    gc: _gc_runtime_state,
    sysdict: *mut _object,
    builtins: *mut _object,
    imports: _import_state,
    _gil: _gil_runtime_state,
    _code_object_generation: u64,
    codecs: codecs_state,
    config: PyConfig,
    feature_flags: u64,
    dict: *mut _object,
    sysdict_copy: *mut _object,
    builtins_copy: *mut _object,
    eval_frame: Option<
        unsafe extern "C-unwind" fn(*mut _ts, *mut _PyInterpreterFrame, i32) -> *mut _object,
    >,
    func_watchers: [Option<
        unsafe extern "C-unwind" fn(u32, *mut __SlateRecord_funcobject_h_62, *mut _object) -> i32,
    >; 8],
    active_func_watchers: u8,
    co_extra_user_count: i64,
    co_extra_freefuncs: [Option<unsafe extern "C-unwind" fn(*mut ())>; 255],
    xi: __SlateRecord_pycore_crossinterp_h_268,
    before_forkers: *mut _object,
    after_forkers_parent: *mut _object,
    after_forkers_child: *mut _object,
    warnings: _warnings_runtime_state,
    atexit: atexit_state,
    stoptheworld: _stoptheworld_state,
    qsbr: _qsbr_shared,
    asyncio_tasks_head: llist_node,
    asyncio_tasks_lock: PyMutex,
    obmalloc: *mut _obmalloc_state,
    audit_hooks: *mut _object,
    type_watchers: [Option<unsafe extern "C-unwind" fn(*mut _typeobject) -> i32>; 8],
    code_watchers: [Option<unsafe extern "C-unwind" fn(u32, *mut PyCodeObject) -> i32>; 8],
    context_watchers: [Option<unsafe extern "C-unwind" fn(u32, *mut _object) -> i32>; 8],
    active_code_watchers: u8,
    active_context_watchers: u8,
    object_state: _py_object_state,
    unicode: _Py_unicode_state,
    long_state: _Py_long_state,
    dtoa: _dtoa_state,
    func_state: _py_func_state,
    code_state: _py_code_state,
    dict_state: _Py_dict_state,
    exc_state: _Py_exc_state,
    mem_free_queue: _Py_mem_interp_free_queue,
    ast: ast_state,
    types: types_state,
    callable_cache: callable_cache,
    common_consts: [*mut _object; 5],
    jit: bool,
    executor_list_head: *mut _PyExecutorObject,
    executor_deletion_list_head: *mut _PyExecutorObject,
    executor_deletion_list_remaining_capacity: i32,
    trace_run_counter: u64,
    rare_events: _rare_events,
    builtins_dict_watcher:
        Option<unsafe extern "C-unwind" fn(u32, *mut _object, *mut _object, *mut _object) -> i32>,
    monitors: _Py_GlobalMonitors,
    sys_profile_once_flag: __SlateRecord_modsupport_h_9,
    sys_trace_once_flag: __SlateRecord_modsupport_h_9,
    sys_profiling_threads: i64,
    sys_tracing_threads: i64,
    monitoring_callables: [[*mut _object; 19]; 8],
    monitoring_tool_names: [*mut _object; 8],
    monitoring_tool_versions: [u64; 8],
    cached_objects: _Py_interp_cached_objects,
    static_objects: _Py_interp_static_objects,
    _interactive_src_count: i64,
    _initial_thread: _PyThreadStateImpl,
}

fn _PyThreadState_GET() -> *mut _ts {
    unsafe { _Py_tss_tstate }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_tracemalloc_h_70 {
    mem: __SlateRecord_pymem_h_48,
    raw: __SlateRecord_pymem_h_48,
    obj: __SlateRecord_pymem_h_48,
}

fn _PyInterpreterState_GET() -> *mut _is {
    let tstate: *mut _ts = _PyThreadState_GET();
    unsafe { (*tstate).interp }
}

fn _Py_AS_GC(op: *mut _object) -> *mut __SlateRecord_pycore_interp_structs_h_170 {
    let gc: *mut i8 = (op as *mut i8).wrapping_offset(
        -(core::mem::size_of::<__SlateRecord_pycore_interp_structs_h_170>() as isize),
    );
    gc as *mut __SlateRecord_pycore_interp_structs_h_170
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _runtime_state {
    size: u64,
    finalizing: u64,
    interpreters_head: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _interpreter_state {
    size: u64,
    id: u64,
    next: u64,
    threads_head: u64,
    threads_main: u64,
    gc: u64,
    imports_modules: u64,
    sysdict: u64,
    builtins: u64,
    ceval_gil: u64,
    gil_runtime_state: u64,
    gil_runtime_state_enabled: u64,
    gil_runtime_state_locked: u64,
    gil_runtime_state_holder: u64,
    code_object_generation: u64,
    tlbc_generation: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _thread_state {
    size: u64,
    prev: u64,
    next: u64,
    interp: u64,
    current_frame: u64,
    thread_id: u64,
    native_thread_id: u64,
    datastack_chunk: u64,
    status: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _interpreter_frame {
    size: u64,
    previous: u64,
    executable: u64,
    instr_ptr: u64,
    localsplus: u64,
    owner: u64,
    stackpointer: u64,
    tlbc_index: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _code_object {
    size: u64,
    filename: u64,
    name: u64,
    qualname: u64,
    linetable: u64,
    firstlineno: u64,
    argcount: u64,
    localsplusnames: u64,
    localspluskinds: u64,
    co_code_adaptive: u64,
    co_tlbc: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _pyobject {
    size: u64,
    ob_type: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _type_object {
    size: u64,
    tp_name: u64,
    tp_repr: u64,
    tp_flags: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _tuple_object {
    size: u64,
    ob_item: u64,
    ob_size: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _list_object {
    size: u64,
    ob_item: u64,
    ob_size: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _set_object {
    size: u64,
    used: u64,
    table: u64,
    mask: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _dict_object {
    size: u64,
    ma_keys: u64,
    ma_values: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _float_object {
    size: u64,
    ob_fval: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _long_object {
    size: u64,
    lv_tag: u64,
    ob_digit: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _bytes_object {
    size: u64,
    ob_size: u64,
    ob_sval: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _unicode_object {
    size: u64,
    state: u64,
    length: u64,
    asciiobject_size: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _gc {
    size: u64,
    collecting: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _gen_object {
    size: u64,
    gi_name: u64,
    gi_iframe: u64,
    gi_frame_state: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _llist_node {
    next: u64,
    prev: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _debugger_support {
    eval_breaker: u64,
    remote_debugger_support: u64,
    remote_debugging_enabled: u64,
    debugger_pending_call: u64,
    debugger_script_path: u64,
    debugger_script_path_size: u64,
}

// Bit flags for _gc_prev
//
// Bit 0 is set when tp_finalize is called
fn _PyGCHead_NEXT(
    gc: *mut __SlateRecord_pycore_interp_structs_h_170,
) -> *mut __SlateRecord_pycore_interp_structs_h_170 {
    let next: u64 = unsafe { (*gc)._gc_next };
    next as *mut __SlateRecord_pycore_interp_structs_h_170
}

fn _PyGCHead_SET_NEXT(
    gc: *mut __SlateRecord_pycore_interp_structs_h_170,
    next: *mut __SlateRecord_pycore_interp_structs_h_170,
) {
    unsafe {
        (*gc)._gc_next = next as u64;
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord_siginfo_t_h_124_2 {
    _pad: [i32; 28],
    _kill: __SlateRecord_siginfo_t_h_124_3,
    _timer: __SlateRecord_siginfo_t_h_124_4,
    _rt: __SlateRecord_siginfo_t_h_124_5,
    _sigchld: __SlateRecord_siginfo_t_h_124_6,
    _sigfault: __SlateRecord_siginfo_t_h_124_7,
    _sigpoll: __SlateRecord_siginfo_t_h_124_10,
    _sigsys: __SlateRecord_siginfo_t_h_124_11,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_siginfo_t_h_124_3 {
    si_pid: i32,
    si_uid: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_siginfo_t_h_124_4 {
    si_tid: i32,
    si_overrun: i32,
    si_sigval: sigval,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_siginfo_t_h_124_5 {
    si_pid: i32,
    si_uid: u32,
    si_sigval: sigval,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_siginfo_t_h_124_6 {
    si_pid: i32,
    si_uid: u32,
    si_status: i32,
    si_utime: i64,
    si_stime: i64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_siginfo_t_h_124_7 {
    si_addr: *mut (),
    si_addr_lsb: i16,
    _bounds: __SlateRecord_siginfo_t_h_124_8,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord_siginfo_t_h_124_8 {
    _addr_bnd: __SlateRecord_siginfo_t_h_124_9,
    _pkey: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_siginfo_t_h_124_9 {
    _lower: *mut (),
    _upper: *mut (),
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_siginfo_t_h_124_10 {
    si_band: i64,
    si_fd: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_siginfo_t_h_124_11 {
    _call_addr: *mut (),
    _syscall: i32,
    _arch: u32,
}

fn _PyGCHead_PREV(
    gc: *mut __SlateRecord_pycore_interp_structs_h_170,
) -> *mut __SlateRecord_pycore_interp_structs_h_170 {
    let prev: u64 = (unsafe { (*gc)._gc_prev }) & (1i32.wrapping_neg() as u64) << 2i32;
    prev as *mut __SlateRecord_pycore_interp_structs_h_170
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord_sigaction_h_31 {
    sa_handler: Option<unsafe extern "C-unwind" fn(i32)>,
    sa_sigaction:
        Option<unsafe extern "C-unwind" fn(i32, *mut __SlateRecord_siginfo_t_h_124, *mut ())>,
}

fn _PyGCHead_SET_PREV(
    gc: *mut __SlateRecord_pycore_interp_structs_h_170,
    prev: *mut __SlateRecord_pycore_interp_structs_h_170,
) {
    let uprev: u64 = prev as u64;
    unsafe {
        (*gc)._gc_prev = (*gc)._gc_prev & !((1i32.wrapping_neg() as u64) << 2i32) | uprev;
    }
}

/// # Arguments
///
/// * `op` - The preprocessor removes _PyObject_ASSERT_FROM() calls if NDEBUG is defined
fn _PyObject_GC_TRACK(op: *mut _object) {
    _PyObject_ASSERT_FROM!();
    let gc: *mut __SlateRecord_pycore_interp_structs_h_170 = _Py_AS_GC(op);
    _PyObject_ASSERT_FROM!();
    let gcstate: *mut _gc_runtime_state =
        unsafe { core::ptr::addr_of_mut!((*_PyInterpreterState_GET()).gc) };
    let generation0: *mut __SlateRecord_pycore_interp_structs_h_170 =
        unsafe { (*gcstate).generation0 };
    let last: *mut __SlateRecord_pycore_interp_structs_h_170 =
        (unsafe { (*generation0)._gc_prev }) as *mut __SlateRecord_pycore_interp_structs_h_170;
    _PyGCHead_SET_NEXT(last, gc);
    _PyGCHead_SET_PREV(gc, last);
    _PyGCHead_SET_NEXT(gc, generation0);
    unsafe {
        (*generation0)._gc_prev = gc as u64;
    }
    let __v1: i64 = unsafe { (*gcstate).heap_size };
    unsafe {
        (*gcstate).heap_size = __v1.wrapping_add(1);
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_signal_h_40 {
    tripped: i32,
    func: *mut _object,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_signal_h_46 {
    fd: i32,
    warn_on_full_buffer: i32,
}

/// # Arguments
///
/// * `op` - The preprocessor removes _PyObject_ASSERT_FROM() calls if NDEBUG is defined
fn _PyObject_GC_UNTRACK(op: *mut _object) {
    _PyObject_ASSERT_FROM!();
    let gc: *mut __SlateRecord_pycore_interp_structs_h_170 = _Py_AS_GC(op);
    let prev: *mut __SlateRecord_pycore_interp_structs_h_170 = _PyGCHead_PREV(gc);
    let next: *mut __SlateRecord_pycore_interp_structs_h_170 = _PyGCHead_NEXT(gc);
    _PyGCHead_SET_NEXT(prev, next);
    _PyGCHead_SET_PREV(next, prev);
    unsafe {
        (*gc)._gc_next = 0;
    }
    unsafe {
        (*gc)._gc_prev = (*gc)._gc_prev & 1;
    }
    let gcstate: *mut _gc_runtime_state =
        unsafe { core::ptr::addr_of_mut!((*_PyInterpreterState_GET()).gc) };
    let __v4: i64 = unsafe { (*gcstate).heap_size };
    unsafe {
        (*gcstate).heap_size = __v4.wrapping_sub(1);
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct pool_header {
    r#ref: __SlateRecord_pycore_obmalloc_h_255,
    freeblock: *mut u8,
    nextpool: *mut pool_header,
    prevpool: *mut pool_header,
    arenaindex: u32,
    szidx: u32,
    nextoffset: u32,
    maxnextoffset: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_faulthandler_h_50 {
    enabled: i32,
    file: *mut _object,
    fd: i32,
    all_threads: i32,
    interp: *mut _is,
    c_stack: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_faulthandler_h_62 {
    file: *mut _object,
    fd: i32,
    timeout_us: i64,
    repeat: i32,
    interp: *mut _is,
    exit: i32,
    header: *mut i8,
    header_len: u64,
    cancel_event: *mut (),
    running: *mut (),
}

#[repr(C)]
#[derive(Clone, Copy)]
struct arena_object {
    address: u64,
    pool_address: *mut u8,
    nfreepools: u32,
    ntotalpools: u32,
    freepools: *mut pool_header,
    nextarena: *mut arena_object,
    prevarena: *mut arena_object,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _obmalloc_pools {
    used: [*mut pool_header; 64],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _obmalloc_mgmt {
    arenas: *mut arena_object,
    maxarenas: u32,
    unused_arena_objects: *mut arena_object,
    usable_arenas: *mut arena_object,
    nfp2lasta: [*mut arena_object; 65],
    narenas_currently_allocated: u64,
    ntimes_arena_allocated: u64,
    narenas_highwater: u64,
    raw_allocated_blocks: i64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_obmalloc_h_622 {
    tail_hi: i32,
    tail_lo: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct arena_map_bot {
    arenas: [__SlateRecord_pycore_obmalloc_h_622; 16384],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct arena_map_mid {
    ptrs: [*mut arena_map_bot; 32768],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct arena_map_top {
    ptrs: [*mut arena_map_mid; 32768],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _obmalloc_usage {
    arena_map_root: arena_map_top,
    arena_map_mid_count: i32,
    arena_map_bot_count: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _obmalloc_global_state {
    dump_debug_stats: i32,
    interpreter_leaks: i64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _obmalloc_state {
    pools: _obmalloc_pools,
    mgmt: _obmalloc_mgmt,
    usage: _obmalloc_usage,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord_pycore_ast_h_370 {
    BoolOp: __SlateRecord_pycore_ast_h_371,
    NamedExpr: __SlateRecord_pycore_ast_h_376,
    BinOp: __SlateRecord_pycore_ast_h_381,
    UnaryOp: __SlateRecord_pycore_ast_h_387,
    Lambda: __SlateRecord_pycore_ast_h_392,
    IfExp: __SlateRecord_pycore_ast_h_397,
    Dict: __SlateRecord_pycore_ast_h_403,
    Set: __SlateRecord_pycore_ast_h_408,
    ListComp: __SlateRecord_pycore_ast_h_412,
    SetComp: __SlateRecord_pycore_ast_h_417,
    DictComp: __SlateRecord_pycore_ast_h_422,
    GeneratorExp: __SlateRecord_pycore_ast_h_428,
    Await: __SlateRecord_pycore_ast_h_433,
    Yield: __SlateRecord_pycore_ast_h_437,
    YieldFrom: __SlateRecord_pycore_ast_h_441,
    Compare: __SlateRecord_pycore_ast_h_445,
    Call: __SlateRecord_pycore_ast_h_451,
    FormattedValue: __SlateRecord_pycore_ast_h_457,
    Interpolation: __SlateRecord_pycore_ast_h_463,
    JoinedStr: __SlateRecord_pycore_ast_h_470,
    TemplateStr: __SlateRecord_pycore_ast_h_474,
    Constant: __SlateRecord_pycore_ast_h_478,
    Attribute: __SlateRecord_pycore_ast_h_483,
    Subscript: __SlateRecord_pycore_ast_h_489,
    Starred: __SlateRecord_pycore_ast_h_495,
    Name: __SlateRecord_pycore_ast_h_500,
    List: __SlateRecord_pycore_ast_h_505,
    Tuple: __SlateRecord_pycore_ast_h_510,
    Slice: __SlateRecord_pycore_ast_h_515,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_371 {
    op: u32,
    values: *mut __SlateRecord_pycore_ast_h_74,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_376 {
    target: *mut _expr,
    value: *mut _expr,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_381 {
    left: *mut _expr,
    op: u32,
    right: *mut _expr,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_387 {
    op: u32,
    operand: *mut _expr,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_392 {
    args: *mut _arguments,
    body: *mut _expr,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_397 {
    test: *mut _expr,
    body: *mut _expr,
    orelse: *mut _expr,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_403 {
    keys: *mut __SlateRecord_pycore_ast_h_74,
    values: *mut __SlateRecord_pycore_ast_h_74,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_408 {
    elts: *mut __SlateRecord_pycore_ast_h_74,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_412 {
    elt: *mut _expr,
    generators: *mut __SlateRecord_pycore_ast_h_81,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_417 {
    elt: *mut _expr,
    generators: *mut __SlateRecord_pycore_ast_h_81,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_422 {
    key: *mut _expr,
    value: *mut _expr,
    generators: *mut __SlateRecord_pycore_ast_h_81,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_428 {
    elt: *mut _expr,
    generators: *mut __SlateRecord_pycore_ast_h_81,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_433 {
    value: *mut _expr,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_437 {
    value: *mut _expr,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_441 {
    value: *mut _expr,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_445 {
    left: *mut _expr,
    ops: *mut __SlateRecord_pycore_asdl_h_45,
    comparators: *mut __SlateRecord_pycore_ast_h_74,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_451 {
    func: *mut _expr,
    args: *mut __SlateRecord_pycore_ast_h_74,
    keywords: *mut __SlateRecord_pycore_ast_h_111,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_457 {
    value: *mut _expr,
    conversion: i32,
    format_spec: *mut _expr,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_463 {
    value: *mut _expr,
    str: *mut _object,
    conversion: i32,
    format_spec: *mut _expr,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_470 {
    values: *mut __SlateRecord_pycore_ast_h_74,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_474 {
    values: *mut __SlateRecord_pycore_ast_h_74,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_478 {
    value: *mut _object,
    kind: *mut _object,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_483 {
    value: *mut _expr,
    attr: *mut _object,
    ctx: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_489 {
    value: *mut _expr,
    slice: *mut _expr,
    ctx: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_495 {
    value: *mut _expr,
    ctx: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_500 {
    id: *mut _object,
    ctx: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_505 {
    elts: *mut __SlateRecord_pycore_ast_h_74,
    ctx: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_510 {
    elts: *mut __SlateRecord_pycore_ast_h_74,
    ctx: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_515 {
    lower: *mut _expr,
    upper: *mut _expr,
    step: *mut _expr,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_runtime_structs_h_19 {
    api_id: i8,
    alloc: __SlateRecord_pymem_h_48,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _pymem_allocators {
    mutex: PyMutex,
    standard: __SlateRecord_pycore_runtime_structs_h_23,
    debug: __SlateRecord_pycore_runtime_structs_h_28,
    is_debug_enabled: i32,
    obj_arena: __SlateRecord_objimpl_h_68,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _Py_float_runtime_state {
    float_format: u32,
    double_format: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct pyhash_runtime_state {
    urandom_cache: __SlateRecord_pycore_runtime_structs_h_49,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_32 {
    _py_anon_dictcomp: __SlateRecord_pycore_global_strings_h_33,
    _py_anon_genexpr: __SlateRecord_pycore_global_strings_h_34,
    _py_anon_lambda: __SlateRecord_pycore_global_strings_h_35,
    _py_anon_listcomp: __SlateRecord_pycore_global_strings_h_36,
    _py_anon_module: __SlateRecord_pycore_global_strings_h_37,
    _py_anon_null: __SlateRecord_pycore_global_strings_h_38,
    _py_anon_setcomp: __SlateRecord_pycore_global_strings_h_39,
    _py_anon_string: __SlateRecord_pycore_global_strings_h_40,
    _py_anon_unknown: __SlateRecord_pycore_global_strings_h_41,
    _py_dbl_close_br: __SlateRecord_pycore_global_strings_h_42,
    _py_dbl_open_br: __SlateRecord_pycore_global_strings_h_43,
    _py_dbl_percent: __SlateRecord_pycore_global_strings_h_44,
    _py_defaults: __SlateRecord_pycore_global_strings_h_45,
    _py_dot_locals: __SlateRecord_pycore_global_strings_h_46,
    _py_empty: __SlateRecord_pycore_global_strings_h_47,
    _py_format: __SlateRecord_pycore_global_strings_h_48,
    _py_generic_base: __SlateRecord_pycore_global_strings_h_49,
    _py_json_decoder: __SlateRecord_pycore_global_strings_h_50,
    _py_kwdefaults: __SlateRecord_pycore_global_strings_h_51,
    _py_list_err: __SlateRecord_pycore_global_strings_h_52,
    _py_str_replace_inf: __SlateRecord_pycore_global_strings_h_53,
    _py_type_params: __SlateRecord_pycore_global_strings_h_54,
    _py_utf_8: __SlateRecord_pycore_global_strings_h_55,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_33 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_34 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_35 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_36 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_37 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_38 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_39 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_40 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_41 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_42 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 3],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_43 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 3],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_44 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 3],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_45 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_46 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_47 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_48 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_49 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 14],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_50 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 13],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_51 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_52 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 24],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_53 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_54 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 13],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_55 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_58 {
    _py_CANCELLED: __SlateRecord_pycore_global_strings_h_59,
    _py_FINISHED: __SlateRecord_pycore_global_strings_h_60,
    _py_False: __SlateRecord_pycore_global_strings_h_61,
    _py_JSONDecodeError: __SlateRecord_pycore_global_strings_h_62,
    _py_PENDING: __SlateRecord_pycore_global_strings_h_63,
    _py_Py_Repr: __SlateRecord_pycore_global_strings_h_64,
    _py_TextIOWrapper: __SlateRecord_pycore_global_strings_h_65,
    _py_True: __SlateRecord_pycore_global_strings_h_66,
    _py_WarningMessage: __SlateRecord_pycore_global_strings_h_67,
    _py__WindowsConsoleIO: __SlateRecord_pycore_global_strings_h_68,
    _py___IOBase_closed: __SlateRecord_pycore_global_strings_h_69,
    _py___abc_tpflags__: __SlateRecord_pycore_global_strings_h_70,
    _py___abs__: __SlateRecord_pycore_global_strings_h_71,
    _py___abstractmethods__: __SlateRecord_pycore_global_strings_h_72,
    _py___add__: __SlateRecord_pycore_global_strings_h_73,
    _py___aenter__: __SlateRecord_pycore_global_strings_h_74,
    _py___aexit__: __SlateRecord_pycore_global_strings_h_75,
    _py___aiter__: __SlateRecord_pycore_global_strings_h_76,
    _py___all__: __SlateRecord_pycore_global_strings_h_77,
    _py___and__: __SlateRecord_pycore_global_strings_h_78,
    _py___anext__: __SlateRecord_pycore_global_strings_h_79,
    _py___annotate__: __SlateRecord_pycore_global_strings_h_80,
    _py___annotate_func__: __SlateRecord_pycore_global_strings_h_81,
    _py___annotations__: __SlateRecord_pycore_global_strings_h_82,
    _py___annotations_cache__: __SlateRecord_pycore_global_strings_h_83,
    _py___args__: __SlateRecord_pycore_global_strings_h_84,
    _py___await__: __SlateRecord_pycore_global_strings_h_85,
    _py___bases__: __SlateRecord_pycore_global_strings_h_86,
    _py___bool__: __SlateRecord_pycore_global_strings_h_87,
    _py___buffer__: __SlateRecord_pycore_global_strings_h_88,
    _py___build_class__: __SlateRecord_pycore_global_strings_h_89,
    _py___builtins__: __SlateRecord_pycore_global_strings_h_90,
    _py___bytes__: __SlateRecord_pycore_global_strings_h_91,
    _py___call__: __SlateRecord_pycore_global_strings_h_92,
    _py___cantrace__: __SlateRecord_pycore_global_strings_h_93,
    _py___ceil__: __SlateRecord_pycore_global_strings_h_94,
    _py___class__: __SlateRecord_pycore_global_strings_h_95,
    _py___class_getitem__: __SlateRecord_pycore_global_strings_h_96,
    _py___classcell__: __SlateRecord_pycore_global_strings_h_97,
    _py___classdict__: __SlateRecord_pycore_global_strings_h_98,
    _py___classdictcell__: __SlateRecord_pycore_global_strings_h_99,
    _py___complex__: __SlateRecord_pycore_global_strings_h_100,
    _py___conditional_annotations__: __SlateRecord_pycore_global_strings_h_101,
    _py___contains__: __SlateRecord_pycore_global_strings_h_102,
    _py___ctypes_from_outparam__: __SlateRecord_pycore_global_strings_h_103,
    _py___del__: __SlateRecord_pycore_global_strings_h_104,
    _py___delattr__: __SlateRecord_pycore_global_strings_h_105,
    _py___delete__: __SlateRecord_pycore_global_strings_h_106,
    _py___delitem__: __SlateRecord_pycore_global_strings_h_107,
    _py___dict__: __SlateRecord_pycore_global_strings_h_108,
    _py___dictoffset__: __SlateRecord_pycore_global_strings_h_109,
    _py___dir__: __SlateRecord_pycore_global_strings_h_110,
    _py___divmod__: __SlateRecord_pycore_global_strings_h_111,
    _py___doc__: __SlateRecord_pycore_global_strings_h_112,
    _py___enter__: __SlateRecord_pycore_global_strings_h_113,
    _py___eq__: __SlateRecord_pycore_global_strings_h_114,
    _py___exit__: __SlateRecord_pycore_global_strings_h_115,
    _py___file__: __SlateRecord_pycore_global_strings_h_116,
    _py___firstlineno__: __SlateRecord_pycore_global_strings_h_117,
    _py___float__: __SlateRecord_pycore_global_strings_h_118,
    _py___floor__: __SlateRecord_pycore_global_strings_h_119,
    _py___floordiv__: __SlateRecord_pycore_global_strings_h_120,
    _py___format__: __SlateRecord_pycore_global_strings_h_121,
    _py___fspath__: __SlateRecord_pycore_global_strings_h_122,
    _py___ge__: __SlateRecord_pycore_global_strings_h_123,
    _py___get__: __SlateRecord_pycore_global_strings_h_124,
    _py___getattr__: __SlateRecord_pycore_global_strings_h_125,
    _py___getattribute__: __SlateRecord_pycore_global_strings_h_126,
    _py___getinitargs__: __SlateRecord_pycore_global_strings_h_127,
    _py___getitem__: __SlateRecord_pycore_global_strings_h_128,
    _py___getnewargs__: __SlateRecord_pycore_global_strings_h_129,
    _py___getnewargs_ex__: __SlateRecord_pycore_global_strings_h_130,
    _py___getstate__: __SlateRecord_pycore_global_strings_h_131,
    _py___gt__: __SlateRecord_pycore_global_strings_h_132,
    _py___hash__: __SlateRecord_pycore_global_strings_h_133,
    _py___iadd__: __SlateRecord_pycore_global_strings_h_134,
    _py___iand__: __SlateRecord_pycore_global_strings_h_135,
    _py___ifloordiv__: __SlateRecord_pycore_global_strings_h_136,
    _py___ilshift__: __SlateRecord_pycore_global_strings_h_137,
    _py___imatmul__: __SlateRecord_pycore_global_strings_h_138,
    _py___imod__: __SlateRecord_pycore_global_strings_h_139,
    _py___import__: __SlateRecord_pycore_global_strings_h_140,
    _py___imul__: __SlateRecord_pycore_global_strings_h_141,
    _py___index__: __SlateRecord_pycore_global_strings_h_142,
    _py___init__: __SlateRecord_pycore_global_strings_h_143,
    _py___init_subclass__: __SlateRecord_pycore_global_strings_h_144,
    _py___instancecheck__: __SlateRecord_pycore_global_strings_h_145,
    _py___int__: __SlateRecord_pycore_global_strings_h_146,
    _py___invert__: __SlateRecord_pycore_global_strings_h_147,
    _py___ior__: __SlateRecord_pycore_global_strings_h_148,
    _py___ipow__: __SlateRecord_pycore_global_strings_h_149,
    _py___irshift__: __SlateRecord_pycore_global_strings_h_150,
    _py___isabstractmethod__: __SlateRecord_pycore_global_strings_h_151,
    _py___isub__: __SlateRecord_pycore_global_strings_h_152,
    _py___iter__: __SlateRecord_pycore_global_strings_h_153,
    _py___itruediv__: __SlateRecord_pycore_global_strings_h_154,
    _py___ixor__: __SlateRecord_pycore_global_strings_h_155,
    _py___le__: __SlateRecord_pycore_global_strings_h_156,
    _py___len__: __SlateRecord_pycore_global_strings_h_157,
    _py___length_hint__: __SlateRecord_pycore_global_strings_h_158,
    _py___lltrace__: __SlateRecord_pycore_global_strings_h_159,
    _py___loader__: __SlateRecord_pycore_global_strings_h_160,
    _py___lshift__: __SlateRecord_pycore_global_strings_h_161,
    _py___lt__: __SlateRecord_pycore_global_strings_h_162,
    _py___main__: __SlateRecord_pycore_global_strings_h_163,
    _py___match_args__: __SlateRecord_pycore_global_strings_h_164,
    _py___matmul__: __SlateRecord_pycore_global_strings_h_165,
    _py___missing__: __SlateRecord_pycore_global_strings_h_166,
    _py___mod__: __SlateRecord_pycore_global_strings_h_167,
    _py___module__: __SlateRecord_pycore_global_strings_h_168,
    _py___mro_entries__: __SlateRecord_pycore_global_strings_h_169,
    _py___mul__: __SlateRecord_pycore_global_strings_h_170,
    _py___name__: __SlateRecord_pycore_global_strings_h_171,
    _py___ne__: __SlateRecord_pycore_global_strings_h_172,
    _py___neg__: __SlateRecord_pycore_global_strings_h_173,
    _py___new__: __SlateRecord_pycore_global_strings_h_174,
    _py___newobj__: __SlateRecord_pycore_global_strings_h_175,
    _py___newobj_ex__: __SlateRecord_pycore_global_strings_h_176,
    _py___next__: __SlateRecord_pycore_global_strings_h_177,
    _py___notes__: __SlateRecord_pycore_global_strings_h_178,
    _py___or__: __SlateRecord_pycore_global_strings_h_179,
    _py___orig_class__: __SlateRecord_pycore_global_strings_h_180,
    _py___origin__: __SlateRecord_pycore_global_strings_h_181,
    _py___package__: __SlateRecord_pycore_global_strings_h_182,
    _py___parameters__: __SlateRecord_pycore_global_strings_h_183,
    _py___path__: __SlateRecord_pycore_global_strings_h_184,
    _py___pos__: __SlateRecord_pycore_global_strings_h_185,
    _py___pow__: __SlateRecord_pycore_global_strings_h_186,
    _py___prepare__: __SlateRecord_pycore_global_strings_h_187,
    _py___qualname__: __SlateRecord_pycore_global_strings_h_188,
    _py___radd__: __SlateRecord_pycore_global_strings_h_189,
    _py___rand__: __SlateRecord_pycore_global_strings_h_190,
    _py___rdivmod__: __SlateRecord_pycore_global_strings_h_191,
    _py___reduce__: __SlateRecord_pycore_global_strings_h_192,
    _py___reduce_ex__: __SlateRecord_pycore_global_strings_h_193,
    _py___release_buffer__: __SlateRecord_pycore_global_strings_h_194,
    _py___repr__: __SlateRecord_pycore_global_strings_h_195,
    _py___reversed__: __SlateRecord_pycore_global_strings_h_196,
    _py___rfloordiv__: __SlateRecord_pycore_global_strings_h_197,
    _py___rlshift__: __SlateRecord_pycore_global_strings_h_198,
    _py___rmatmul__: __SlateRecord_pycore_global_strings_h_199,
    _py___rmod__: __SlateRecord_pycore_global_strings_h_200,
    _py___rmul__: __SlateRecord_pycore_global_strings_h_201,
    _py___ror__: __SlateRecord_pycore_global_strings_h_202,
    _py___round__: __SlateRecord_pycore_global_strings_h_203,
    _py___rpow__: __SlateRecord_pycore_global_strings_h_204,
    _py___rrshift__: __SlateRecord_pycore_global_strings_h_205,
    _py___rshift__: __SlateRecord_pycore_global_strings_h_206,
    _py___rsub__: __SlateRecord_pycore_global_strings_h_207,
    _py___rtruediv__: __SlateRecord_pycore_global_strings_h_208,
    _py___rxor__: __SlateRecord_pycore_global_strings_h_209,
    _py___set__: __SlateRecord_pycore_global_strings_h_210,
    _py___set_name__: __SlateRecord_pycore_global_strings_h_211,
    _py___setattr__: __SlateRecord_pycore_global_strings_h_212,
    _py___setitem__: __SlateRecord_pycore_global_strings_h_213,
    _py___setstate__: __SlateRecord_pycore_global_strings_h_214,
    _py___sizeof__: __SlateRecord_pycore_global_strings_h_215,
    _py___slotnames__: __SlateRecord_pycore_global_strings_h_216,
    _py___slots__: __SlateRecord_pycore_global_strings_h_217,
    _py___spec__: __SlateRecord_pycore_global_strings_h_218,
    _py___static_attributes__: __SlateRecord_pycore_global_strings_h_219,
    _py___str__: __SlateRecord_pycore_global_strings_h_220,
    _py___sub__: __SlateRecord_pycore_global_strings_h_221,
    _py___subclasscheck__: __SlateRecord_pycore_global_strings_h_222,
    _py___subclasshook__: __SlateRecord_pycore_global_strings_h_223,
    _py___truediv__: __SlateRecord_pycore_global_strings_h_224,
    _py___trunc__: __SlateRecord_pycore_global_strings_h_225,
    _py___type_params__: __SlateRecord_pycore_global_strings_h_226,
    _py___typing_is_unpacked_typevartuple__: __SlateRecord_pycore_global_strings_h_227,
    _py___typing_prepare_subst__: __SlateRecord_pycore_global_strings_h_228,
    _py___typing_subst__: __SlateRecord_pycore_global_strings_h_229,
    _py___typing_unpacked_tuple_args__: __SlateRecord_pycore_global_strings_h_230,
    _py___warningregistry__: __SlateRecord_pycore_global_strings_h_231,
    _py___weaklistoffset__: __SlateRecord_pycore_global_strings_h_232,
    _py___weakref__: __SlateRecord_pycore_global_strings_h_233,
    _py___xor__: __SlateRecord_pycore_global_strings_h_234,
    _py__abc_impl: __SlateRecord_pycore_global_strings_h_235,
    _py__abstract_: __SlateRecord_pycore_global_strings_h_236,
    _py__active: __SlateRecord_pycore_global_strings_h_237,
    _py__anonymous_: __SlateRecord_pycore_global_strings_h_238,
    _py__argtypes_: __SlateRecord_pycore_global_strings_h_239,
    _py__as_parameter_: __SlateRecord_pycore_global_strings_h_240,
    _py__asyncio_future_blocking: __SlateRecord_pycore_global_strings_h_241,
    _py__blksize: __SlateRecord_pycore_global_strings_h_242,
    _py__bootstrap: __SlateRecord_pycore_global_strings_h_243,
    _py__check_retval_: __SlateRecord_pycore_global_strings_h_244,
    _py__dealloc_warn: __SlateRecord_pycore_global_strings_h_245,
    _py__feature_version: __SlateRecord_pycore_global_strings_h_246,
    _py__field_types: __SlateRecord_pycore_global_strings_h_247,
    _py__fields_: __SlateRecord_pycore_global_strings_h_248,
    _py__filters: __SlateRecord_pycore_global_strings_h_249,
    _py__finalizing: __SlateRecord_pycore_global_strings_h_250,
    _py__find_and_load: __SlateRecord_pycore_global_strings_h_251,
    _py__fix_up_module: __SlateRecord_pycore_global_strings_h_252,
    _py__flags_: __SlateRecord_pycore_global_strings_h_253,
    _py__get_sourcefile: __SlateRecord_pycore_global_strings_h_254,
    _py__handle_fromlist: __SlateRecord_pycore_global_strings_h_255,
    _py__initializing: __SlateRecord_pycore_global_strings_h_256,
    _py__internal_use: __SlateRecord_pycore_global_strings_h_257,
    _py__io: __SlateRecord_pycore_global_strings_h_258,
    _py__is_text_encoding: __SlateRecord_pycore_global_strings_h_259,
    _py__isatty_open_only: __SlateRecord_pycore_global_strings_h_260,
    _py__length_: __SlateRecord_pycore_global_strings_h_261,
    _py__limbo: __SlateRecord_pycore_global_strings_h_262,
    _py__lock_unlock_module: __SlateRecord_pycore_global_strings_h_263,
    _py__loop: __SlateRecord_pycore_global_strings_h_264,
    _py__needs_com_addref_: __SlateRecord_pycore_global_strings_h_265,
    _py__only_immortal: __SlateRecord_pycore_global_strings_h_266,
    _py__restype_: __SlateRecord_pycore_global_strings_h_267,
    _py__showwarnmsg: __SlateRecord_pycore_global_strings_h_268,
    _py__shutdown: __SlateRecord_pycore_global_strings_h_269,
    _py__slotnames: __SlateRecord_pycore_global_strings_h_270,
    _py__strptime: __SlateRecord_pycore_global_strings_h_271,
    _py__strptime_datetime_date: __SlateRecord_pycore_global_strings_h_272,
    _py__strptime_datetime_datetime: __SlateRecord_pycore_global_strings_h_273,
    _py__strptime_datetime_time: __SlateRecord_pycore_global_strings_h_274,
    _py__type_: __SlateRecord_pycore_global_strings_h_275,
    _py__uninitialized_submodules: __SlateRecord_pycore_global_strings_h_276,
    _py__warn_unawaited_coroutine: __SlateRecord_pycore_global_strings_h_277,
    _py__xoptions: __SlateRecord_pycore_global_strings_h_278,
    _py_abs_tol: __SlateRecord_pycore_global_strings_h_279,
    _py_access: __SlateRecord_pycore_global_strings_h_280,
    _py_aclose: __SlateRecord_pycore_global_strings_h_281,
    _py_add: __SlateRecord_pycore_global_strings_h_282,
    _py_add_done_callback: __SlateRecord_pycore_global_strings_h_283,
    _py_after_in_child: __SlateRecord_pycore_global_strings_h_284,
    _py_after_in_parent: __SlateRecord_pycore_global_strings_h_285,
    _py_aggregate_class: __SlateRecord_pycore_global_strings_h_286,
    _py_alias: __SlateRecord_pycore_global_strings_h_287,
    _py_align: __SlateRecord_pycore_global_strings_h_288,
    _py_all: __SlateRecord_pycore_global_strings_h_289,
    _py_all_threads: __SlateRecord_pycore_global_strings_h_290,
    _py_allow_code: __SlateRecord_pycore_global_strings_h_291,
    _py_any: __SlateRecord_pycore_global_strings_h_292,
    _py_append: __SlateRecord_pycore_global_strings_h_293,
    _py_arg: __SlateRecord_pycore_global_strings_h_294,
    _py_argdefs: __SlateRecord_pycore_global_strings_h_295,
    _py_args: __SlateRecord_pycore_global_strings_h_296,
    _py_arguments: __SlateRecord_pycore_global_strings_h_297,
    _py_argv: __SlateRecord_pycore_global_strings_h_298,
    _py_as_integer_ratio: __SlateRecord_pycore_global_strings_h_299,
    _py_asend: __SlateRecord_pycore_global_strings_h_300,
    _py_ast: __SlateRecord_pycore_global_strings_h_301,
    _py_athrow: __SlateRecord_pycore_global_strings_h_302,
    _py_attribute: __SlateRecord_pycore_global_strings_h_303,
    _py_authorizer_callback: __SlateRecord_pycore_global_strings_h_304,
    _py_autocommit: __SlateRecord_pycore_global_strings_h_305,
    _py_backtick: __SlateRecord_pycore_global_strings_h_306,
    _py_base: __SlateRecord_pycore_global_strings_h_307,
    _py_before: __SlateRecord_pycore_global_strings_h_308,
    _py_big: __SlateRecord_pycore_global_strings_h_309,
    _py_binary_form: __SlateRecord_pycore_global_strings_h_310,
    _py_bit_offset: __SlateRecord_pycore_global_strings_h_311,
    _py_bit_size: __SlateRecord_pycore_global_strings_h_312,
    _py_block: __SlateRecord_pycore_global_strings_h_313,
    _py_bound: __SlateRecord_pycore_global_strings_h_314,
    _py_buffer: __SlateRecord_pycore_global_strings_h_315,
    _py_buffer_callback: __SlateRecord_pycore_global_strings_h_316,
    _py_buffer_size: __SlateRecord_pycore_global_strings_h_317,
    _py_buffering: __SlateRecord_pycore_global_strings_h_318,
    _py_buffers: __SlateRecord_pycore_global_strings_h_319,
    _py_bufsize: __SlateRecord_pycore_global_strings_h_320,
    _py_builtins: __SlateRecord_pycore_global_strings_h_321,
    _py_byte_offset: __SlateRecord_pycore_global_strings_h_322,
    _py_byte_size: __SlateRecord_pycore_global_strings_h_323,
    _py_byteorder: __SlateRecord_pycore_global_strings_h_324,
    _py_bytes: __SlateRecord_pycore_global_strings_h_325,
    _py_bytes_per_sep: __SlateRecord_pycore_global_strings_h_326,
    _py_c_call: __SlateRecord_pycore_global_strings_h_327,
    _py_c_exception: __SlateRecord_pycore_global_strings_h_328,
    _py_c_parameter_type: __SlateRecord_pycore_global_strings_h_329,
    _py_c_return: __SlateRecord_pycore_global_strings_h_330,
    _py_cached_datetime_module: __SlateRecord_pycore_global_strings_h_331,
    _py_cached_statements: __SlateRecord_pycore_global_strings_h_332,
    _py_cadata: __SlateRecord_pycore_global_strings_h_333,
    _py_cafile: __SlateRecord_pycore_global_strings_h_334,
    _py_call: __SlateRecord_pycore_global_strings_h_335,
    _py_call_exception_handler: __SlateRecord_pycore_global_strings_h_336,
    _py_call_soon: __SlateRecord_pycore_global_strings_h_337,
    _py_callback: __SlateRecord_pycore_global_strings_h_338,
    _py_cancel: __SlateRecord_pycore_global_strings_h_339,
    _py_capath: __SlateRecord_pycore_global_strings_h_340,
    _py_category: __SlateRecord_pycore_global_strings_h_341,
    _py_cb_type: __SlateRecord_pycore_global_strings_h_342,
    _py_certfile: __SlateRecord_pycore_global_strings_h_343,
    _py_check_same_thread: __SlateRecord_pycore_global_strings_h_344,
    _py_clear: __SlateRecord_pycore_global_strings_h_345,
    _py_close: __SlateRecord_pycore_global_strings_h_346,
    _py_closed: __SlateRecord_pycore_global_strings_h_347,
    _py_closefd: __SlateRecord_pycore_global_strings_h_348,
    _py_closure: __SlateRecord_pycore_global_strings_h_349,
    _py_co_argcount: __SlateRecord_pycore_global_strings_h_350,
    _py_co_cellvars: __SlateRecord_pycore_global_strings_h_351,
    _py_co_code: __SlateRecord_pycore_global_strings_h_352,
    _py_co_consts: __SlateRecord_pycore_global_strings_h_353,
    _py_co_exceptiontable: __SlateRecord_pycore_global_strings_h_354,
    _py_co_filename: __SlateRecord_pycore_global_strings_h_355,
    _py_co_firstlineno: __SlateRecord_pycore_global_strings_h_356,
    _py_co_flags: __SlateRecord_pycore_global_strings_h_357,
    _py_co_freevars: __SlateRecord_pycore_global_strings_h_358,
    _py_co_kwonlyargcount: __SlateRecord_pycore_global_strings_h_359,
    _py_co_linetable: __SlateRecord_pycore_global_strings_h_360,
    _py_co_name: __SlateRecord_pycore_global_strings_h_361,
    _py_co_names: __SlateRecord_pycore_global_strings_h_362,
    _py_co_nlocals: __SlateRecord_pycore_global_strings_h_363,
    _py_co_posonlyargcount: __SlateRecord_pycore_global_strings_h_364,
    _py_co_qualname: __SlateRecord_pycore_global_strings_h_365,
    _py_co_stacksize: __SlateRecord_pycore_global_strings_h_366,
    _py_co_varnames: __SlateRecord_pycore_global_strings_h_367,
    _py_code: __SlateRecord_pycore_global_strings_h_368,
    _py_col_offset: __SlateRecord_pycore_global_strings_h_369,
    _py_command: __SlateRecord_pycore_global_strings_h_370,
    _py_comment_factory: __SlateRecord_pycore_global_strings_h_371,
    _py_compile_mode: __SlateRecord_pycore_global_strings_h_372,
    _py_consts: __SlateRecord_pycore_global_strings_h_373,
    _py_context: __SlateRecord_pycore_global_strings_h_374,
    _py_contravariant: __SlateRecord_pycore_global_strings_h_375,
    _py_conversion: __SlateRecord_pycore_global_strings_h_376,
    _py_cookie: __SlateRecord_pycore_global_strings_h_377,
    _py_copy: __SlateRecord_pycore_global_strings_h_378,
    _py_copyreg: __SlateRecord_pycore_global_strings_h_379,
    _py_coro: __SlateRecord_pycore_global_strings_h_380,
    _py_count: __SlateRecord_pycore_global_strings_h_381,
    _py_covariant: __SlateRecord_pycore_global_strings_h_382,
    _py_cwd: __SlateRecord_pycore_global_strings_h_383,
    _py_d_parameter_type: __SlateRecord_pycore_global_strings_h_384,
    _py_data: __SlateRecord_pycore_global_strings_h_385,
    _py_database: __SlateRecord_pycore_global_strings_h_386,
    _py_day: __SlateRecord_pycore_global_strings_h_387,
    _py_debug: __SlateRecord_pycore_global_strings_h_388,
    _py_decode: __SlateRecord_pycore_global_strings_h_389,
    _py_decoder: __SlateRecord_pycore_global_strings_h_390,
    _py_default: __SlateRecord_pycore_global_strings_h_391,
    _py_defaultaction: __SlateRecord_pycore_global_strings_h_392,
    _py_delete: __SlateRecord_pycore_global_strings_h_393,
    _py_depth: __SlateRecord_pycore_global_strings_h_394,
    _py_desired_access: __SlateRecord_pycore_global_strings_h_395,
    _py_detect_types: __SlateRecord_pycore_global_strings_h_396,
    _py_deterministic: __SlateRecord_pycore_global_strings_h_397,
    _py_device: __SlateRecord_pycore_global_strings_h_398,
    _py_dict: __SlateRecord_pycore_global_strings_h_399,
    _py_dictcomp: __SlateRecord_pycore_global_strings_h_400,
    _py_difference_update: __SlateRecord_pycore_global_strings_h_401,
    _py_digest: __SlateRecord_pycore_global_strings_h_402,
    _py_digest_size: __SlateRecord_pycore_global_strings_h_403,
    _py_digestmod: __SlateRecord_pycore_global_strings_h_404,
    _py_dir_fd: __SlateRecord_pycore_global_strings_h_405,
    _py_discard: __SlateRecord_pycore_global_strings_h_406,
    _py_dispatch_table: __SlateRecord_pycore_global_strings_h_407,
    _py_displayhook: __SlateRecord_pycore_global_strings_h_408,
    _py_dklen: __SlateRecord_pycore_global_strings_h_409,
    _py_doc: __SlateRecord_pycore_global_strings_h_410,
    _py_done: __SlateRecord_pycore_global_strings_h_411,
    _py_dont_inherit: __SlateRecord_pycore_global_strings_h_412,
    _py_dst: __SlateRecord_pycore_global_strings_h_413,
    _py_dst_dir_fd: __SlateRecord_pycore_global_strings_h_414,
    _py_eager_start: __SlateRecord_pycore_global_strings_h_415,
    _py_effective_ids: __SlateRecord_pycore_global_strings_h_416,
    _py_element_factory: __SlateRecord_pycore_global_strings_h_417,
    _py_encode: __SlateRecord_pycore_global_strings_h_418,
    _py_encoding: __SlateRecord_pycore_global_strings_h_419,
    _py_end: __SlateRecord_pycore_global_strings_h_420,
    _py_end_col_offset: __SlateRecord_pycore_global_strings_h_421,
    _py_end_lineno: __SlateRecord_pycore_global_strings_h_422,
    _py_end_offset: __SlateRecord_pycore_global_strings_h_423,
    _py_endpos: __SlateRecord_pycore_global_strings_h_424,
    _py_entrypoint: __SlateRecord_pycore_global_strings_h_425,
    _py_env: __SlateRecord_pycore_global_strings_h_426,
    _py_errors: __SlateRecord_pycore_global_strings_h_427,
    _py_event: __SlateRecord_pycore_global_strings_h_428,
    _py_eventmask: __SlateRecord_pycore_global_strings_h_429,
    _py_exc_type: __SlateRecord_pycore_global_strings_h_430,
    _py_exc_value: __SlateRecord_pycore_global_strings_h_431,
    _py_excepthook: __SlateRecord_pycore_global_strings_h_432,
    _py_exception: __SlateRecord_pycore_global_strings_h_433,
    _py_existing_file_name: __SlateRecord_pycore_global_strings_h_434,
    _py_exp: __SlateRecord_pycore_global_strings_h_435,
    _py_expression: __SlateRecord_pycore_global_strings_h_436,
    _py_extend: __SlateRecord_pycore_global_strings_h_437,
    _py_extra_tokens: __SlateRecord_pycore_global_strings_h_438,
    _py_facility: __SlateRecord_pycore_global_strings_h_439,
    _py_factory: __SlateRecord_pycore_global_strings_h_440,
    _py_false: __SlateRecord_pycore_global_strings_h_441,
    _py_family: __SlateRecord_pycore_global_strings_h_442,
    _py_fanout: __SlateRecord_pycore_global_strings_h_443,
    _py_fd: __SlateRecord_pycore_global_strings_h_444,
    _py_fd2: __SlateRecord_pycore_global_strings_h_445,
    _py_fdel: __SlateRecord_pycore_global_strings_h_446,
    _py_fget: __SlateRecord_pycore_global_strings_h_447,
    _py_fields: __SlateRecord_pycore_global_strings_h_448,
    _py_file: __SlateRecord_pycore_global_strings_h_449,
    _py_file_actions: __SlateRecord_pycore_global_strings_h_450,
    _py_filename: __SlateRecord_pycore_global_strings_h_451,
    _py_fileno: __SlateRecord_pycore_global_strings_h_452,
    _py_filepath: __SlateRecord_pycore_global_strings_h_453,
    _py_fillvalue: __SlateRecord_pycore_global_strings_h_454,
    _py_filter: __SlateRecord_pycore_global_strings_h_455,
    _py_filters: __SlateRecord_pycore_global_strings_h_456,
    _py_final: __SlateRecord_pycore_global_strings_h_457,
    _py_find_class: __SlateRecord_pycore_global_strings_h_458,
    _py_fix_imports: __SlateRecord_pycore_global_strings_h_459,
    _py_flags: __SlateRecord_pycore_global_strings_h_460,
    _py_flush: __SlateRecord_pycore_global_strings_h_461,
    _py_fold: __SlateRecord_pycore_global_strings_h_462,
    _py_follow_symlinks: __SlateRecord_pycore_global_strings_h_463,
    _py_format: __SlateRecord_pycore_global_strings_h_464,
    _py_format_spec: __SlateRecord_pycore_global_strings_h_465,
    _py_frame_buffer: __SlateRecord_pycore_global_strings_h_466,
    _py_from_param: __SlateRecord_pycore_global_strings_h_467,
    _py_fromlist: __SlateRecord_pycore_global_strings_h_468,
    _py_fromtimestamp: __SlateRecord_pycore_global_strings_h_469,
    _py_fromutc: __SlateRecord_pycore_global_strings_h_470,
    _py_fset: __SlateRecord_pycore_global_strings_h_471,
    _py_func: __SlateRecord_pycore_global_strings_h_472,
    _py_future: __SlateRecord_pycore_global_strings_h_473,
    _py_generation: __SlateRecord_pycore_global_strings_h_474,
    _py_genexpr: __SlateRecord_pycore_global_strings_h_475,
    _py_get: __SlateRecord_pycore_global_strings_h_476,
    _py_get_debug: __SlateRecord_pycore_global_strings_h_477,
    _py_get_event_loop: __SlateRecord_pycore_global_strings_h_478,
    _py_get_loop: __SlateRecord_pycore_global_strings_h_479,
    _py_get_source: __SlateRecord_pycore_global_strings_h_480,
    _py_getattr: __SlateRecord_pycore_global_strings_h_481,
    _py_getstate: __SlateRecord_pycore_global_strings_h_482,
    _py_gid: __SlateRecord_pycore_global_strings_h_483,
    _py_globals: __SlateRecord_pycore_global_strings_h_484,
    _py_groupindex: __SlateRecord_pycore_global_strings_h_485,
    _py_groups: __SlateRecord_pycore_global_strings_h_486,
    _py_handle: __SlateRecord_pycore_global_strings_h_487,
    _py_handle_seq: __SlateRecord_pycore_global_strings_h_488,
    _py_has_location: __SlateRecord_pycore_global_strings_h_489,
    _py_hash_name: __SlateRecord_pycore_global_strings_h_490,
    _py_header: __SlateRecord_pycore_global_strings_h_491,
    _py_headers: __SlateRecord_pycore_global_strings_h_492,
    _py_hi: __SlateRecord_pycore_global_strings_h_493,
    _py_hook: __SlateRecord_pycore_global_strings_h_494,
    _py_hour: __SlateRecord_pycore_global_strings_h_495,
    _py_id: __SlateRecord_pycore_global_strings_h_496,
    _py_ident: __SlateRecord_pycore_global_strings_h_497,
    _py_identity_hint: __SlateRecord_pycore_global_strings_h_498,
    _py_ignore: __SlateRecord_pycore_global_strings_h_499,
    _py_imag: __SlateRecord_pycore_global_strings_h_500,
    _py_importlib: __SlateRecord_pycore_global_strings_h_501,
    _py_in_fd: __SlateRecord_pycore_global_strings_h_502,
    _py_incoming: __SlateRecord_pycore_global_strings_h_503,
    _py_index: __SlateRecord_pycore_global_strings_h_504,
    _py_indexgroup: __SlateRecord_pycore_global_strings_h_505,
    _py_inf: __SlateRecord_pycore_global_strings_h_506,
    _py_infer_variance: __SlateRecord_pycore_global_strings_h_507,
    _py_inherit_handle: __SlateRecord_pycore_global_strings_h_508,
    _py_inheritable: __SlateRecord_pycore_global_strings_h_509,
    _py_initial: __SlateRecord_pycore_global_strings_h_510,
    _py_initial_bytes: __SlateRecord_pycore_global_strings_h_511,
    _py_initial_owner: __SlateRecord_pycore_global_strings_h_512,
    _py_initial_state: __SlateRecord_pycore_global_strings_h_513,
    _py_initial_value: __SlateRecord_pycore_global_strings_h_514,
    _py_initval: __SlateRecord_pycore_global_strings_h_515,
    _py_inner_size: __SlateRecord_pycore_global_strings_h_516,
    _py_input: __SlateRecord_pycore_global_strings_h_517,
    _py_insert_comments: __SlateRecord_pycore_global_strings_h_518,
    _py_insert_pis: __SlateRecord_pycore_global_strings_h_519,
    _py_instructions: __SlateRecord_pycore_global_strings_h_520,
    _py_intern: __SlateRecord_pycore_global_strings_h_521,
    _py_intersection: __SlateRecord_pycore_global_strings_h_522,
    _py_interval: __SlateRecord_pycore_global_strings_h_523,
    _py_io: __SlateRecord_pycore_global_strings_h_524,
    _py_is_compress: __SlateRecord_pycore_global_strings_h_525,
    _py_is_raw: __SlateRecord_pycore_global_strings_h_526,
    _py_is_running: __SlateRecord_pycore_global_strings_h_527,
    _py_is_struct: __SlateRecord_pycore_global_strings_h_528,
    _py_isatty: __SlateRecord_pycore_global_strings_h_529,
    _py_isinstance: __SlateRecord_pycore_global_strings_h_530,
    _py_isoformat: __SlateRecord_pycore_global_strings_h_531,
    _py_isolation_level: __SlateRecord_pycore_global_strings_h_532,
    _py_istext: __SlateRecord_pycore_global_strings_h_533,
    _py_item: __SlateRecord_pycore_global_strings_h_534,
    _py_items: __SlateRecord_pycore_global_strings_h_535,
    _py_iter: __SlateRecord_pycore_global_strings_h_536,
    _py_iterable: __SlateRecord_pycore_global_strings_h_537,
    _py_iterations: __SlateRecord_pycore_global_strings_h_538,
    _py_join: __SlateRecord_pycore_global_strings_h_539,
    _py_jump: __SlateRecord_pycore_global_strings_h_540,
    _py_keepends: __SlateRecord_pycore_global_strings_h_541,
    _py_key: __SlateRecord_pycore_global_strings_h_542,
    _py_keyfile: __SlateRecord_pycore_global_strings_h_543,
    _py_keys: __SlateRecord_pycore_global_strings_h_544,
    _py_kind: __SlateRecord_pycore_global_strings_h_545,
    _py_kw: __SlateRecord_pycore_global_strings_h_546,
    _py_kw1: __SlateRecord_pycore_global_strings_h_547,
    _py_kw2: __SlateRecord_pycore_global_strings_h_548,
    _py_kwdefaults: __SlateRecord_pycore_global_strings_h_549,
    _py_label: __SlateRecord_pycore_global_strings_h_550,
    _py_lambda: __SlateRecord_pycore_global_strings_h_551,
    _py_last: __SlateRecord_pycore_global_strings_h_552,
    _py_last_exc: __SlateRecord_pycore_global_strings_h_553,
    _py_last_node: __SlateRecord_pycore_global_strings_h_554,
    _py_last_traceback: __SlateRecord_pycore_global_strings_h_555,
    _py_last_type: __SlateRecord_pycore_global_strings_h_556,
    _py_last_value: __SlateRecord_pycore_global_strings_h_557,
    _py_latin1: __SlateRecord_pycore_global_strings_h_558,
    _py_leaf_size: __SlateRecord_pycore_global_strings_h_559,
    _py_len: __SlateRecord_pycore_global_strings_h_560,
    _py_length: __SlateRecord_pycore_global_strings_h_561,
    _py_level: __SlateRecord_pycore_global_strings_h_562,
    _py_limit: __SlateRecord_pycore_global_strings_h_563,
    _py_line: __SlateRecord_pycore_global_strings_h_564,
    _py_line_buffering: __SlateRecord_pycore_global_strings_h_565,
    _py_lineno: __SlateRecord_pycore_global_strings_h_566,
    _py_listcomp: __SlateRecord_pycore_global_strings_h_567,
    _py_little: __SlateRecord_pycore_global_strings_h_568,
    _py_lo: __SlateRecord_pycore_global_strings_h_569,
    _py_locale: __SlateRecord_pycore_global_strings_h_570,
    _py_locals: __SlateRecord_pycore_global_strings_h_571,
    _py_logoption: __SlateRecord_pycore_global_strings_h_572,
    _py_loop: __SlateRecord_pycore_global_strings_h_573,
    _py_manual_reset: __SlateRecord_pycore_global_strings_h_574,
    _py_mapping: __SlateRecord_pycore_global_strings_h_575,
    _py_match: __SlateRecord_pycore_global_strings_h_576,
    _py_max_length: __SlateRecord_pycore_global_strings_h_577,
    _py_maxdigits: __SlateRecord_pycore_global_strings_h_578,
    _py_maxevents: __SlateRecord_pycore_global_strings_h_579,
    _py_maxlen: __SlateRecord_pycore_global_strings_h_580,
    _py_maxmem: __SlateRecord_pycore_global_strings_h_581,
    _py_maxsplit: __SlateRecord_pycore_global_strings_h_582,
    _py_maxvalue: __SlateRecord_pycore_global_strings_h_583,
    _py_memLevel: __SlateRecord_pycore_global_strings_h_584,
    _py_memlimit: __SlateRecord_pycore_global_strings_h_585,
    _py_message: __SlateRecord_pycore_global_strings_h_586,
    _py_metaclass: __SlateRecord_pycore_global_strings_h_587,
    _py_metadata: __SlateRecord_pycore_global_strings_h_588,
    _py_method: __SlateRecord_pycore_global_strings_h_589,
    _py_microsecond: __SlateRecord_pycore_global_strings_h_590,
    _py_milliseconds: __SlateRecord_pycore_global_strings_h_591,
    _py_minute: __SlateRecord_pycore_global_strings_h_592,
    _py_mod: __SlateRecord_pycore_global_strings_h_593,
    _py_mode: __SlateRecord_pycore_global_strings_h_594,
    _py_module: __SlateRecord_pycore_global_strings_h_595,
    _py_module_globals: __SlateRecord_pycore_global_strings_h_596,
    _py_modules: __SlateRecord_pycore_global_strings_h_597,
    _py_month: __SlateRecord_pycore_global_strings_h_598,
    _py_mro: __SlateRecord_pycore_global_strings_h_599,
    _py_msg: __SlateRecord_pycore_global_strings_h_600,
    _py_mutex: __SlateRecord_pycore_global_strings_h_601,
    _py_mycmp: __SlateRecord_pycore_global_strings_h_602,
    _py_n_arg: __SlateRecord_pycore_global_strings_h_603,
    _py_n_fields: __SlateRecord_pycore_global_strings_h_604,
    _py_n_sequence_fields: __SlateRecord_pycore_global_strings_h_605,
    _py_n_unnamed_fields: __SlateRecord_pycore_global_strings_h_606,
    _py_name: __SlateRecord_pycore_global_strings_h_607,
    _py_name_from: __SlateRecord_pycore_global_strings_h_608,
    _py_namespace_separator: __SlateRecord_pycore_global_strings_h_609,
    _py_namespaces: __SlateRecord_pycore_global_strings_h_610,
    _py_narg: __SlateRecord_pycore_global_strings_h_611,
    _py_ndigits: __SlateRecord_pycore_global_strings_h_612,
    _py_nested: __SlateRecord_pycore_global_strings_h_613,
    _py_new_file_name: __SlateRecord_pycore_global_strings_h_614,
    _py_new_limit: __SlateRecord_pycore_global_strings_h_615,
    _py_newline: __SlateRecord_pycore_global_strings_h_616,
    _py_newlines: __SlateRecord_pycore_global_strings_h_617,
    _py_next: __SlateRecord_pycore_global_strings_h_618,
    _py_nlocals: __SlateRecord_pycore_global_strings_h_619,
    _py_node_depth: __SlateRecord_pycore_global_strings_h_620,
    _py_node_offset: __SlateRecord_pycore_global_strings_h_621,
    _py_ns: __SlateRecord_pycore_global_strings_h_622,
    _py_nstype: __SlateRecord_pycore_global_strings_h_623,
    _py_nt: __SlateRecord_pycore_global_strings_h_624,
    _py_null: __SlateRecord_pycore_global_strings_h_625,
    _py_number: __SlateRecord_pycore_global_strings_h_626,
    _py_obj: __SlateRecord_pycore_global_strings_h_627,
    _py_object: __SlateRecord_pycore_global_strings_h_628,
    _py_offset: __SlateRecord_pycore_global_strings_h_629,
    _py_offset_dst: __SlateRecord_pycore_global_strings_h_630,
    _py_offset_src: __SlateRecord_pycore_global_strings_h_631,
    _py_on_type_read: __SlateRecord_pycore_global_strings_h_632,
    _py_onceregistry: __SlateRecord_pycore_global_strings_h_633,
    _py_only_active_thread: __SlateRecord_pycore_global_strings_h_634,
    _py_only_keys: __SlateRecord_pycore_global_strings_h_635,
    _py_oparg: __SlateRecord_pycore_global_strings_h_636,
    _py_opcode: __SlateRecord_pycore_global_strings_h_637,
    _py_open: __SlateRecord_pycore_global_strings_h_638,
    _py_opener: __SlateRecord_pycore_global_strings_h_639,
    _py_operation: __SlateRecord_pycore_global_strings_h_640,
    _py_optimize: __SlateRecord_pycore_global_strings_h_641,
    _py_options: __SlateRecord_pycore_global_strings_h_642,
    _py_order: __SlateRecord_pycore_global_strings_h_643,
    _py_origin: __SlateRecord_pycore_global_strings_h_644,
    _py_out_fd: __SlateRecord_pycore_global_strings_h_645,
    _py_outgoing: __SlateRecord_pycore_global_strings_h_646,
    _py_outpath: __SlateRecord_pycore_global_strings_h_647,
    _py_overlapped: __SlateRecord_pycore_global_strings_h_648,
    _py_owner: __SlateRecord_pycore_global_strings_h_649,
    _py_pages: __SlateRecord_pycore_global_strings_h_650,
    _py_parameter: __SlateRecord_pycore_global_strings_h_651,
    _py_parent: __SlateRecord_pycore_global_strings_h_652,
    _py_password: __SlateRecord_pycore_global_strings_h_653,
    _py_path: __SlateRecord_pycore_global_strings_h_654,
    _py_pattern: __SlateRecord_pycore_global_strings_h_655,
    _py_peek: __SlateRecord_pycore_global_strings_h_656,
    _py_persistent_id: __SlateRecord_pycore_global_strings_h_657,
    _py_persistent_load: __SlateRecord_pycore_global_strings_h_658,
    _py_person: __SlateRecord_pycore_global_strings_h_659,
    _py_pi_factory: __SlateRecord_pycore_global_strings_h_660,
    _py_pid: __SlateRecord_pycore_global_strings_h_661,
    _py_policy: __SlateRecord_pycore_global_strings_h_662,
    _py_pos: __SlateRecord_pycore_global_strings_h_663,
    _py_pos1: __SlateRecord_pycore_global_strings_h_664,
    _py_pos2: __SlateRecord_pycore_global_strings_h_665,
    _py_posix: __SlateRecord_pycore_global_strings_h_666,
    _py_print_file_and_line: __SlateRecord_pycore_global_strings_h_667,
    _py_priority: __SlateRecord_pycore_global_strings_h_668,
    _py_progress: __SlateRecord_pycore_global_strings_h_669,
    _py_progress_handler: __SlateRecord_pycore_global_strings_h_670,
    _py_progress_routine: __SlateRecord_pycore_global_strings_h_671,
    _py_proto: __SlateRecord_pycore_global_strings_h_672,
    _py_protocol: __SlateRecord_pycore_global_strings_h_673,
    _py_ps1: __SlateRecord_pycore_global_strings_h_674,
    _py_ps2: __SlateRecord_pycore_global_strings_h_675,
    _py_query: __SlateRecord_pycore_global_strings_h_676,
    _py_quotetabs: __SlateRecord_pycore_global_strings_h_677,
    _py_raw: __SlateRecord_pycore_global_strings_h_678,
    _py_read: __SlateRecord_pycore_global_strings_h_679,
    _py_read1: __SlateRecord_pycore_global_strings_h_680,
    _py_readable: __SlateRecord_pycore_global_strings_h_681,
    _py_readall: __SlateRecord_pycore_global_strings_h_682,
    _py_readinto: __SlateRecord_pycore_global_strings_h_683,
    _py_readinto1: __SlateRecord_pycore_global_strings_h_684,
    _py_readline: __SlateRecord_pycore_global_strings_h_685,
    _py_readonly: __SlateRecord_pycore_global_strings_h_686,
    _py_real: __SlateRecord_pycore_global_strings_h_687,
    _py_reducer_override: __SlateRecord_pycore_global_strings_h_688,
    _py_registry: __SlateRecord_pycore_global_strings_h_689,
    _py_rel_tol: __SlateRecord_pycore_global_strings_h_690,
    _py_release: __SlateRecord_pycore_global_strings_h_691,
    _py_reload: __SlateRecord_pycore_global_strings_h_692,
    _py_repl: __SlateRecord_pycore_global_strings_h_693,
    _py_replace: __SlateRecord_pycore_global_strings_h_694,
    _py_reserved: __SlateRecord_pycore_global_strings_h_695,
    _py_reset: __SlateRecord_pycore_global_strings_h_696,
    _py_resetids: __SlateRecord_pycore_global_strings_h_697,
    _py_return: __SlateRecord_pycore_global_strings_h_698,
    _py_reverse: __SlateRecord_pycore_global_strings_h_699,
    _py_reversed: __SlateRecord_pycore_global_strings_h_700,
    _py_salt: __SlateRecord_pycore_global_strings_h_701,
    _py_sched_priority: __SlateRecord_pycore_global_strings_h_702,
    _py_scheduler: __SlateRecord_pycore_global_strings_h_703,
    _py_script: __SlateRecord_pycore_global_strings_h_704,
    _py_second: __SlateRecord_pycore_global_strings_h_705,
    _py_security_attributes: __SlateRecord_pycore_global_strings_h_706,
    _py_seek: __SlateRecord_pycore_global_strings_h_707,
    _py_seekable: __SlateRecord_pycore_global_strings_h_708,
    _py_selectors: __SlateRecord_pycore_global_strings_h_709,
    _py_self: __SlateRecord_pycore_global_strings_h_710,
    _py_send: __SlateRecord_pycore_global_strings_h_711,
    _py_sep: __SlateRecord_pycore_global_strings_h_712,
    _py_sequence: __SlateRecord_pycore_global_strings_h_713,
    _py_server_hostname: __SlateRecord_pycore_global_strings_h_714,
    _py_server_side: __SlateRecord_pycore_global_strings_h_715,
    _py_session: __SlateRecord_pycore_global_strings_h_716,
    _py_setcomp: __SlateRecord_pycore_global_strings_h_717,
    _py_setpgroup: __SlateRecord_pycore_global_strings_h_718,
    _py_setsid: __SlateRecord_pycore_global_strings_h_719,
    _py_setsigdef: __SlateRecord_pycore_global_strings_h_720,
    _py_setsigmask: __SlateRecord_pycore_global_strings_h_721,
    _py_setstate: __SlateRecord_pycore_global_strings_h_722,
    _py_shape: __SlateRecord_pycore_global_strings_h_723,
    _py_show_cmd: __SlateRecord_pycore_global_strings_h_724,
    _py_signed: __SlateRecord_pycore_global_strings_h_725,
    _py_size: __SlateRecord_pycore_global_strings_h_726,
    _py_sizehint: __SlateRecord_pycore_global_strings_h_727,
    _py_skip_file_prefixes: __SlateRecord_pycore_global_strings_h_728,
    _py_sleep: __SlateRecord_pycore_global_strings_h_729,
    _py_sock: __SlateRecord_pycore_global_strings_h_730,
    _py_sort: __SlateRecord_pycore_global_strings_h_731,
    _py_source: __SlateRecord_pycore_global_strings_h_732,
    _py_source_traceback: __SlateRecord_pycore_global_strings_h_733,
    _py_spam: __SlateRecord_pycore_global_strings_h_734,
    _py_src: __SlateRecord_pycore_global_strings_h_735,
    _py_src_dir_fd: __SlateRecord_pycore_global_strings_h_736,
    _py_stacklevel: __SlateRecord_pycore_global_strings_h_737,
    _py_start: __SlateRecord_pycore_global_strings_h_738,
    _py_statement: __SlateRecord_pycore_global_strings_h_739,
    _py_status: __SlateRecord_pycore_global_strings_h_740,
    _py_stderr: __SlateRecord_pycore_global_strings_h_741,
    _py_stdin: __SlateRecord_pycore_global_strings_h_742,
    _py_stdout: __SlateRecord_pycore_global_strings_h_743,
    _py_step: __SlateRecord_pycore_global_strings_h_744,
    _py_steps: __SlateRecord_pycore_global_strings_h_745,
    _py_store_name: __SlateRecord_pycore_global_strings_h_746,
    _py_strategy: __SlateRecord_pycore_global_strings_h_747,
    _py_strftime: __SlateRecord_pycore_global_strings_h_748,
    _py_strict: __SlateRecord_pycore_global_strings_h_749,
    _py_strict_mode: __SlateRecord_pycore_global_strings_h_750,
    _py_string: __SlateRecord_pycore_global_strings_h_751,
    _py_sub_key: __SlateRecord_pycore_global_strings_h_752,
    _py_subcalls: __SlateRecord_pycore_global_strings_h_753,
    _py_symmetric_difference_update: __SlateRecord_pycore_global_strings_h_754,
    _py_tabsize: __SlateRecord_pycore_global_strings_h_755,
    _py_tag: __SlateRecord_pycore_global_strings_h_756,
    _py_target: __SlateRecord_pycore_global_strings_h_757,
    _py_target_is_directory: __SlateRecord_pycore_global_strings_h_758,
    _py_task: __SlateRecord_pycore_global_strings_h_759,
    _py_tb_frame: __SlateRecord_pycore_global_strings_h_760,
    _py_tb_lasti: __SlateRecord_pycore_global_strings_h_761,
    _py_tb_lineno: __SlateRecord_pycore_global_strings_h_762,
    _py_tb_next: __SlateRecord_pycore_global_strings_h_763,
    _py_tell: __SlateRecord_pycore_global_strings_h_764,
    _py_template: __SlateRecord_pycore_global_strings_h_765,
    _py_term: __SlateRecord_pycore_global_strings_h_766,
    _py_text: __SlateRecord_pycore_global_strings_h_767,
    _py_threading: __SlateRecord_pycore_global_strings_h_768,
    _py_throw: __SlateRecord_pycore_global_strings_h_769,
    _py_timeout: __SlateRecord_pycore_global_strings_h_770,
    _py_timer: __SlateRecord_pycore_global_strings_h_771,
    _py_times: __SlateRecord_pycore_global_strings_h_772,
    _py_timetuple: __SlateRecord_pycore_global_strings_h_773,
    _py_timeunit: __SlateRecord_pycore_global_strings_h_774,
    _py_top: __SlateRecord_pycore_global_strings_h_775,
    _py_trace_callback: __SlateRecord_pycore_global_strings_h_776,
    _py_traceback: __SlateRecord_pycore_global_strings_h_777,
    _py_trailers: __SlateRecord_pycore_global_strings_h_778,
    _py_translate: __SlateRecord_pycore_global_strings_h_779,
    _py_true: __SlateRecord_pycore_global_strings_h_780,
    _py_truncate: __SlateRecord_pycore_global_strings_h_781,
    _py_twice: __SlateRecord_pycore_global_strings_h_782,
    _py_txt: __SlateRecord_pycore_global_strings_h_783,
    _py_type: __SlateRecord_pycore_global_strings_h_784,
    _py_type_params: __SlateRecord_pycore_global_strings_h_785,
    _py_tz: __SlateRecord_pycore_global_strings_h_786,
    _py_tzinfo: __SlateRecord_pycore_global_strings_h_787,
    _py_tzname: __SlateRecord_pycore_global_strings_h_788,
    _py_uid: __SlateRecord_pycore_global_strings_h_789,
    _py_unlink: __SlateRecord_pycore_global_strings_h_790,
    _py_unraisablehook: __SlateRecord_pycore_global_strings_h_791,
    _py_uri: __SlateRecord_pycore_global_strings_h_792,
    _py_usedforsecurity: __SlateRecord_pycore_global_strings_h_793,
    _py_value: __SlateRecord_pycore_global_strings_h_794,
    _py_values: __SlateRecord_pycore_global_strings_h_795,
    _py_version: __SlateRecord_pycore_global_strings_h_796,
    _py_volume: __SlateRecord_pycore_global_strings_h_797,
    _py_wait_all: __SlateRecord_pycore_global_strings_h_798,
    _py_warn_on_full_buffer: __SlateRecord_pycore_global_strings_h_799,
    _py_warnings: __SlateRecord_pycore_global_strings_h_800,
    _py_warnoptions: __SlateRecord_pycore_global_strings_h_801,
    _py_wbits: __SlateRecord_pycore_global_strings_h_802,
    _py_week: __SlateRecord_pycore_global_strings_h_803,
    _py_weekday: __SlateRecord_pycore_global_strings_h_804,
    _py_which: __SlateRecord_pycore_global_strings_h_805,
    _py_who: __SlateRecord_pycore_global_strings_h_806,
    _py_withdata: __SlateRecord_pycore_global_strings_h_807,
    _py_writable: __SlateRecord_pycore_global_strings_h_808,
    _py_write: __SlateRecord_pycore_global_strings_h_809,
    _py_write_through: __SlateRecord_pycore_global_strings_h_810,
    _py_year: __SlateRecord_pycore_global_strings_h_811,
    _py_zdict: __SlateRecord_pycore_global_strings_h_812,
    _py_zstd_dict: __SlateRecord_pycore_global_strings_h_813,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_59 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_60 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_61 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_62 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 16],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_63 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_64 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_65 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 14],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_66 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_67 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 15],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_68 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 18],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_69 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 16],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_70 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 16],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_71 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_72 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 20],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_73 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_74 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_75 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_76 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_77 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_78 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_79 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_80 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 13],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_81 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 18],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_82 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 16],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_83 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 22],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_84 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_85 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_86 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_87 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_88 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_89 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 16],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_90 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 13],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_91 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_92 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_93 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 13],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_94 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_95 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_96 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 18],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_97 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 14],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_98 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 14],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_99 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 18],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_100 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_101 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 28],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_102 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 13],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_103 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 25],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_104 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_105 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_106 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_107 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_108 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_109 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 15],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_110 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_111 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_112 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_113 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_114 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_115 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_116 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_117 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 16],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_118 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_119 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_120 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 13],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_121 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_122 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_123 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_124 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_125 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_126 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 17],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_127 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 16],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_128 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_129 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 15],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_130 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 18],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_131 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 13],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_132 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_133 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_134 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_135 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_136 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 14],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_137 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_138 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_139 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_140 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_141 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_142 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_143 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_144 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 18],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_145 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 18],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_146 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_147 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_148 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_149 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_150 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_151 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 21],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_152 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_153 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_154 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 13],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_155 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_156 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_157 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_158 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 16],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_159 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_160 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_161 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_162 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_163 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_164 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 15],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_165 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_166 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_167 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_168 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_169 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 16],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_170 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_171 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_172 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_173 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_174 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_175 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_176 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 14],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_177 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_178 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_179 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_180 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 15],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_181 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_182 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_183 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 15],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_184 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_185 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_186 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_187 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_188 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 13],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_189 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_190 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_191 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_192 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_193 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 14],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_194 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 19],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_195 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_196 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 13],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_197 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 14],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_198 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_199 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_200 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_201 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_202 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_203 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_204 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_205 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_206 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_207 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_208 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 13],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_209 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_210 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_211 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 13],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_212 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_213 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_214 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 13],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_215 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_216 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 14],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_217 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_218 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_219 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 22],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_220 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_221 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_222 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 18],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_223 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 17],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_224 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_225 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_226 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 16],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_227 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 36],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_228 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 25],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_229 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 17],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_230 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 31],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_231 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 20],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_232 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 19],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_233 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_234 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_235 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_236 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_237 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_238 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_239 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_240 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 15],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_241 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 25],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_242 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_243 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_244 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 15],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_245 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 14],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_246 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 17],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_247 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 13],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_248 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_249 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_250 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_251 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 15],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_252 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 15],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_253 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_254 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 16],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_255 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 17],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_256 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 14],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_257 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 14],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_258 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_259 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 18],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_260 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 18],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_261 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_262 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_263 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 20],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_264 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_265 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 19],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_266 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 15],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_267 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_268 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 13],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_269 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_270 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_271 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_272 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 24],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_273 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 28],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_274 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 24],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_275 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_276 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 26],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_277 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 26],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_278 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_279 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_280 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_281 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_282 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_283 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 18],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_284 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 15],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_285 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 16],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_286 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 16],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_287 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_288 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_289 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_290 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_291 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_292 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_293 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_294 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_295 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_296 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_297 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_298 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_299 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 17],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_300 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_301 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_302 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_303 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_304 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 20],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_305 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_306 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_307 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_308 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_309 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_310 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_311 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_312 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_313 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_314 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_315 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_316 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 16],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_317 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_318 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_319 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_320 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_321 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_322 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_323 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_324 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_325 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_326 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 14],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_327 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_328 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_329 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 17],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_330 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_331 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 23],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_332 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 18],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_333 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_334 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_335 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_336 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 23],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_337 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_338 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_339 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_340 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_341 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_342 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_343 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_344 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 18],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_345 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_346 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_347 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_348 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_349 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_350 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_351 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_352 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_353 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_354 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 18],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_355 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_356 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 15],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_357 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_358 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_359 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 18],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_360 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 13],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_361 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_362 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_363 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_364 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 19],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_365 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_366 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 13],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_367 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_368 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_369 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_370 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_371 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 16],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_372 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 13],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_373 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_374 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_375 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 14],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_376 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_377 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_378 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_379 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_380 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_381 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_382 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_383 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_384 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 17],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_385 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_386 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_387 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_388 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_389 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_390 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_391 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_392 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 14],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_393 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_394 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_395 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 15],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_396 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 13],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_397 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 14],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_398 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_399 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_400 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_401 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 18],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_402 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_403 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_404 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_405 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_406 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_407 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 15],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_408 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_409 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_410 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_411 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_412 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 13],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_413 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_414 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_415 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_416 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 14],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_417 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 16],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_418 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_419 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_420 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_421 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 15],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_422 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_423 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_424 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_425 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_426 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_427 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_428 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_429 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_430 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_431 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_432 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_433 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_434 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 19],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_435 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_436 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_437 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_438 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 13],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_439 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_440 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_441 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_442 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_443 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_444 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 3],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_445 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_446 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_447 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_448 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_449 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_450 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 13],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_451 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_452 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_453 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_454 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_455 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_456 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_457 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_458 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_459 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_460 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_461 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_462 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_463 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 16],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_464 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_465 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_466 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 13],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_467 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_468 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_469 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 14],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_470 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_471 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_472 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_473 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_474 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_475 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_476 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_477 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_478 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 15],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_479 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_480 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_481 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_482 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_483 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_484 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_485 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_486 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_487 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_488 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_489 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 13],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_490 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_491 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_492 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_493 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 3],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_494 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_495 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_496 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 3],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_497 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_498 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 14],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_499 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_500 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_501 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_502 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_503 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_504 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_505 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_506 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_507 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 15],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_508 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 15],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_509 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_510 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_511 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 14],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_512 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 14],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_513 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 14],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_514 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 14],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_515 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_516 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_517 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_518 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 16],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_519 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_520 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 13],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_521 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_522 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 13],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_523 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_524 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 3],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_525 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_526 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_527 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_528 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_529 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_530 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_531 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_532 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 16],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_533 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_534 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_535 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_536 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_537 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_538 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_539 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_540 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_541 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_542 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_543 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_544 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_545 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_546 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 3],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_547 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_548 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_549 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_550 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_551 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_552 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_553 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_554 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_555 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 15],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_556 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_557 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_558 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_559 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_560 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_561 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_562 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_563 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_564 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_565 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 15],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_566 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_567 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_568 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_569 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 3],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_570 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_571 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_572 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_573 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_574 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 13],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_575 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_576 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_577 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_578 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_579 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_580 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_581 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_582 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_583 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_584 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_585 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_586 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_587 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_588 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_589 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_590 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_591 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 13],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_592 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_593 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_594 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_595 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_596 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 15],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_597 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_598 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_599 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_600 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_601 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_602 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_603 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_604 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_605 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 18],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_606 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 17],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_607 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_608 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_609 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 20],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_610 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_611 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_612 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_613 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_614 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 14],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_615 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_616 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_617 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_618 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_619 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_620 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_621 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_622 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 3],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_623 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_624 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 3],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_625 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_626 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_627 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_628 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_629 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_630 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_631 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_632 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 13],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_633 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 13],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_634 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 19],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_635 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_636 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_637 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_638 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_639 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_640 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_641 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_642 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_643 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_644 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_645 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_646 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_647 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_648 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_649 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_650 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_651 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_652 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_653 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_654 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_655 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_656 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_657 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 14],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_658 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 16],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_659 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_660 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_661 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_662 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_663 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_664 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_665 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_666 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_667 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 20],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_668 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_669 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_670 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 17],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_671 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 17],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_672 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_673 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_674 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_675 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_676 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_677 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_678 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_679 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_680 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_681 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_682 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_683 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_684 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_685 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_686 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_687 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_688 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 17],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_689 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_690 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_691 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_692 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_693 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_694 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_695 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_696 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_697 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_698 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_699 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_700 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_701 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_702 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 15],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_703 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_704 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_705 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_706 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 20],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_707 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_708 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_709 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_710 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_711 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_712 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_713 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_714 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 16],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_715 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_716 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_717 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_718 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_719 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_720 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_721 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_722 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_723 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_724 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_725 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_726 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_727 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_728 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 19],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_729 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_730 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_731 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_732 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_733 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 17],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_734 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_735 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_736 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_737 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_738 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_739 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_740 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_741 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_742 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_743 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_744 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_745 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_746 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_747 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_748 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_749 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_750 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_751 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_752 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_753 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_754 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 28],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_755 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_756 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_757 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_758 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 20],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_759 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_760 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_761 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_762 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_763 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_764 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_765 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_766 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_767 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_768 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_769 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_770 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_771 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_772 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_773 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_774 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_775 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_776 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 15],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_777 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_778 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_779 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_780 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_781 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_782 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_783 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_784 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_785 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_786 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 3],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_787 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_788 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_789 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_790 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_791 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 15],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_792 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_793 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 16],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_794 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_795 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_796 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_797 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_798 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_799 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 20],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_800 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_801 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_802 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_803 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_804 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_805 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_806 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_807 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_808 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_809 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_810 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 14],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_811 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_812 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_813 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_815 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 2],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_819 {
    _latin1: __SlateRecord_unicodeobject_h_173,
    _data: [u8; 2],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _PyTraceMalloc_Config {
    initialized: u32,
    tracing: i32,
    max_nframe: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_runtime_structs_h_115 {
    small_ints: [_longobject; 262],
    bytes_empty: __SlateRecord_bytesobject_h_15,
    bytes_characters: [__SlateRecord_pycore_runtime_structs_h_124; 256],
    strings: _Py_global_strings,
    _tuple_empty_gc_not_used: __SlateRecord_pycore_interp_structs_h_170,
    tuple_empty: __SlateRecord_tupleobject_h_13,
    _hamt_bitmap_node_empty_gc_not_used: __SlateRecord_pycore_interp_structs_h_170,
    hamt_bitmap_node_empty: __SlateRecord_pycore_structs_h_53,
    context_token_missing: __SlateRecord_pycore_context_h_21,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_runtime_structs_h_124 {
    ob: __SlateRecord_bytesobject_h_15,
    eos: i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct pyinterpreters {
    mutex: PyMutex,
    head: *mut _is,
    main: *mut _is,
    next_id: i64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _gilstate_runtime_state {
    check_enabled: i32,
    autoInterpreterState: *mut _is,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _getargs_runtime_state {
    static_parsers: *mut _PyArg_Parser,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_runtime_structs_h_271 {
    mutex: PyMutex,
    head: *mut _Py_AuditHookEntry,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _Py_AuditHookEntry {}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_memoryobject_c_h_32 {
    _this_is_not_used: __SlateRecord_pycore_interp_structs_h_170,
    ob_base: __SlateRecord_object_h_172,
    ob_hash: i64,
    ob_item: [*mut _object; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_memoryobject_c_h_92 {
    _this_is_not_used: __SlateRecord_pycore_interp_structs_h_170,
    ob_base: __SlateRecord_object_h_172,
    ob_hash: i64,
    ob_item: [*mut _object; 2],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_memoryobject_c_h_174 {
    _this_is_not_used: __SlateRecord_pycore_interp_structs_h_170,
    ob_base: __SlateRecord_object_h_172,
    ob_hash: i64,
    ob_item: [*mut _object; 2],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_memoryobject_c_h_286 {
    _this_is_not_used: __SlateRecord_pycore_interp_structs_h_170,
    ob_base: __SlateRecord_object_h_172,
    ob_hash: i64,
    ob_item: [*mut _object; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_memoryobject_c_h_384 {
    _this_is_not_used: __SlateRecord_pycore_interp_structs_h_170,
    ob_base: __SlateRecord_object_h_172,
    ob_hash: i64,
    ob_item: [*mut _object; 2],
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
struct tracemalloc_frame {
    filename: *mut _object,
    lineno: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct tracemalloc_traceback {
    hash: u64,
    nframe: u16,
    total_nframe: u16,
    frames: [tracemalloc_frame; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _tracemalloc_runtime_state {
    config: _PyTraceMalloc_Config,
    allocators: __SlateRecord_pycore_tracemalloc_h_70,
    tables_lock: PyMutex,
    traced_memory: u64,
    peak_traced_memory: u64,
    filenames: *mut _Py_hashtable_t,
    traceback: *mut tracemalloc_traceback,
    tracebacks: *mut _Py_hashtable_t,
    traces: *mut _Py_hashtable_t,
    domains: *mut _Py_hashtable_t,
    empty_traceback: tracemalloc_traceback,
    reentrant_key: _Py_tss_t,
}

#[repr(C, align(16))]
struct __SlateAlign16<T>(T);

unsafe impl<T> Sync for __SlateAlign16<T> {}

mod __slate_bits {
    #[bitfields::bitfield(
        u32,
        c_names = true,
        new = false,
        from_into_bits = false,
        from_traits = false,
        default = false,
        debug = false,
        builder = false,
        bit_ops = false
    )]
    pub struct __SlateBits135U0 {
        #[bits(2)]
        pub interned: u32,
        #[bits(3)]
        pub kind: u32,
        #[bits(1)]
        pub compact: u32,
        #[bits(1)]
        pub ascii: u32,
        #[bits(1)]
        pub statically_allocated: u32,
        #[bits(24, access = na)]
        pub __slate_pad_5: u32,
    }
    #[bitfields::bitfield(
        u32,
        c_names = true,
        new = false,
        from_into_bits = false,
        from_traits = false,
        default = false,
        debug = false,
        builder = false,
        bit_ops = false
    )]
    pub struct __SlateBits185U0 {
        #[bits(1)]
        pub initialized: u32,
        #[bits(1)]
        pub bound: u32,
        #[bits(1)]
        pub unbound: u32,
        #[bits(1)]
        pub bound_gilstate: u32,
        #[bits(1)]
        pub active: u32,
        #[bits(1)]
        pub finalizing: u32,
        #[bits(1)]
        pub cleared: u32,
        #[bits(1)]
        pub finalized: u32,
        #[bits(24, access = na)]
        pub __slate_pad_8: u32,
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _fileutils_state {
    force_ascii: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _PyInterpreterFrame {
    f_executable: _PyStackRef,
    previous: *mut _PyInterpreterFrame,
    f_funcobj: _PyStackRef,
    f_globals: *mut _object,
    f_builtins: *mut _object,
    f_locals: *mut _object,
    frame_obj: *mut _frame,
    instr_ptr: *mut __SlateRecord_pycore_structs_h_32,
    stackpointer: *mut _PyStackRef,
    return_offset: u16,
    owner: i8,
    visited: u8,
    localsplus: [_PyStackRef; 1],
}

// _PyGenObject_HEAD defines the initial segment of generator
// and coroutine objects.
// List of weak reference.
// Name of the generator.
// Qualified name of the generator.
// The frame
#[repr(C)]
#[derive(Clone, Copy)]
struct _Py_DebugOffsets {
    cookie: [i8; 8],
    version: u64,
    free_threaded: u64,
    runtime_state: _runtime_state,
    interpreter_state: _interpreter_state,
    thread_state: _thread_state,
    interpreter_frame: _interpreter_frame,
    code_object: _code_object,
    pyobject: _pyobject,
    type_object: _type_object,
    tuple_object: _tuple_object,
    list_object: _list_object,
    set_object: _set_object,
    dict_object: _dict_object,
    float_object: _float_object,
    long_object: _long_object,
    bytes_object: _bytes_object,
    unicode_object: _unicode_object,
    gc: _gc,
    gen_object: _gen_object,
    llist_node: _llist_node,
    debugger_support: _debugger_support,
}

#[repr(C)]
#[derive(Clone, Copy)]
union sigval {
    sival_int: i32,
    sival_ptr: *mut (),
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_siginfo_t_h_124 {
    si_signo: i32,
    si_errno: i32,
    si_code: i32,
    __pad0: i32,
    _sifields: __SlateRecord_siginfo_t_h_124_2,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct sigaction {
    __sigaction_handler: __SlateRecord_sigaction_h_31,
    sa_mask: __SlateRecord___sigset_t_h_8,
    sa_flags: i32,
    sa_restorer: Option<unsafe extern "C-unwind" fn()>,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_stack_t_h_31 {
    ss_sp: *mut (),
    ss_flags: i32,
    ss_size: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _signals_runtime_state {
    handlers: [__SlateRecord_pycore_signal_h_40; 65],
    wakeup: __SlateRecord_pycore_signal_h_46,
    is_tripped: i32,
    default_handler: *mut _object,
    ignore_handler: *mut _object,
    unhandled_keyboard_interrupt: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct faulthandler_user_signal {
    enabled: i32,
    file: *mut _object,
    fd: i32,
    all_threads: i32,
    chain: i32,
    previous: sigaction,
    interp: *mut _is,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _faulthandler_runtime_state {
    fatal_error: __SlateRecord_pycore_faulthandler_h_50,
    thread: __SlateRecord_pycore_faulthandler_h_62,
    user_signals: *mut faulthandler_user_signal,
    stack: __SlateRecord_stack_t_h_31,
    old_stack: __SlateRecord_stack_t_h_31,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_asdl_h_45 {
    size: i64,
    elements: *mut *mut (),
    typed_elements: [i32; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_74 {
    size: i64,
    elements: *mut *mut (),
    typed_elements: [*mut _expr; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_81 {
    size: i64,
    elements: *mut *mut (),
    typed_elements: [*mut _comprehension; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_104 {
    size: i64,
    elements: *mut *mut (),
    typed_elements: [*mut _arg; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_111 {
    size: i64,
    elements: *mut *mut (),
    typed_elements: [*mut _keyword; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _expr {
    kind: u32,
    v: __SlateRecord_pycore_ast_h_370,
    lineno: i32,
    col_offset: i32,
    end_lineno: i32,
    end_col_offset: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _comprehension {
    target: *mut _expr,
    iter: *mut _expr,
    ifs: *mut __SlateRecord_pycore_ast_h_74,
    is_async: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _arguments {
    posonlyargs: *mut __SlateRecord_pycore_ast_h_104,
    args: *mut __SlateRecord_pycore_ast_h_104,
    vararg: *mut _arg,
    kwonlyargs: *mut __SlateRecord_pycore_ast_h_104,
    kw_defaults: *mut __SlateRecord_pycore_ast_h_74,
    kwarg: *mut _arg,
    defaults: *mut __SlateRecord_pycore_ast_h_74,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _arg {
    arg: *mut _object,
    annotation: *mut _expr,
    type_comment: *mut _object,
    lineno: i32,
    col_offset: i32,
    end_lineno: i32,
    end_col_offset: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _keyword {
    arg: *mut _object,
    value: *mut _expr,
    lineno: i32,
    col_offset: i32,
    end_lineno: i32,
    end_col_offset: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _parser_runtime_state {
    _not_used: i32,
    dummy_name: _expr,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _Py_time_runtime_state {
    _unused: i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _Py_cached_objects {
    interned_strings: *mut _Py_hashtable_t,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _Py_global_strings {
    literals: __SlateRecord_pycore_global_strings_h_32,
    identifiers: __SlateRecord_pycore_global_strings_h_58,
    ascii: [__SlateRecord_pycore_global_strings_h_815; 128],
    latin1: [__SlateRecord_pycore_global_strings_h_819; 128],
}

// These would be in pycore_long.h if it weren't for an include cycle.
#[repr(C)]
#[derive(Clone, Copy)]
struct _Py_static_objects {
    singletons: __SlateRecord_pycore_runtime_structs_h_115,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct pyruntimestate {
    debug_offsets: _Py_DebugOffsets,
    _initialized: i32,
    preinitializing: i32,
    preinitialized: i32,
    core_initialized: i32,
    initialized: i32,
    _finalizing: *mut _ts,
    _finalizing_id: u64,
    interpreters: pyinterpreters,
    main_thread: u64,
    main_tstate: *mut _ts,
    xi: __SlateRecord_pycore_crossinterp_h_255,
    allocators: _pymem_allocators,
    obmalloc: _obmalloc_global_state,
    pyhash_state: pyhash_runtime_state,
    threads: _pythread_runtime_state,
    signals: _signals_runtime_state,
    autoTSSkey: _Py_tss_t,
    trashTSSkey: _Py_tss_t,
    orig_argv: __SlateRecord_initconfig_h_36,
    parser: _parser_runtime_state,
    atexit: _atexit_runtime_state,
    imports: _import_runtime_state,
    ceval: _ceval_runtime_state,
    gilstate: _gilstate_runtime_state,
    getargs: _getargs_runtime_state,
    fileutils: _fileutils_state,
    faulthandler: _faulthandler_runtime_state,
    tracemalloc: _tracemalloc_runtime_state,
    ref_tracer: _reftracer_runtime_state,
    stoptheworld_mutex: __SlateRecord_pycore_lock_h_189,
    stoptheworld: _stoptheworld_state,
    preconfig: PyPreConfig,
    open_code_hook: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> *mut _object>,
    open_code_userdata: *mut (),
    audit_hooks: __SlateRecord_pycore_runtime_structs_h_271,
    object_state: _py_object_runtime_state,
    float_state: _Py_float_runtime_state,
    unicode_state: _Py_unicode_runtime_state,
    types: _types_runtime_state,
    time: _Py_time_runtime_state,
    cached_objects: _Py_cached_objects,
    static_objects: _Py_static_objects,
    _main_interpreter: _is,
}

static memoryview__doc__: __SlateAlign16<[i8; 89]> = __SlateAlign16([
    109i8, 101i8, 109i8, 111i8, 114i8, 121i8, 118i8, 105i8, 101i8, 119i8, 40i8, 111i8, 98i8, 106i8,
    101i8, 99i8, 116i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 67i8, 114i8, 101i8, 97i8, 116i8, 101i8,
    32i8, 97i8, 32i8, 110i8, 101i8, 119i8, 32i8, 109i8, 101i8, 109i8, 111i8, 114i8, 121i8, 118i8,
    105i8, 101i8, 119i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 32i8, 119i8, 104i8, 105i8,
    99i8, 104i8, 32i8, 114i8, 101i8, 102i8, 101i8, 114i8, 101i8, 110i8, 99i8, 101i8, 115i8, 32i8,
    116i8, 104i8, 101i8, 32i8, 103i8, 105i8, 118i8, 101i8, 110i8, 32i8, 111i8, 98i8, 106i8, 101i8,
    99i8, 116i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.memoryobject.memoryview")]
extern "C-unwind" fn memoryview(
    r#type: *mut _typeobject,
    args: *mut _object,
    kwargs: *mut _object,
) -> *mut _object {
    let mut fastargs: *const *mut _object = unsafe { core::mem::zeroed() };
    let mut argsbuf: [*mut _object; 1] = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        let nargs: i64 = PyTuple_GET_SIZE!(args);
        fastargs = _PyArg_UnpackKeywords__2!(
            (*(args as *mut __SlateRecord_tupleobject_h_13))
                .ob_item
                .as_mut_ptr() as *mut *mut _object,
            nargs,
            kwargs,
            core::ptr::null_mut::<()>(),
            core::ptr::addr_of_mut!(_parser__45),
            1i32,
            1i32,
            0i32,
            0i32,
            argsbuf.as_mut_ptr() as *mut *mut _object
        );
        // minpos
        // maxpos
        // minkw
        // varpos
        if !fastargs.is_null() {
            let object: *mut _object = *fastargs.wrapping_offset(0 as isize);
            return_value = memoryview_impl(r#type, object);
        }
        return return_value;
    }
}

static mut _kwtuple__32: __SlateRecord_memoryobject_c_h_32 = {
    let mut __t0: __SlateRecord_memoryobject_c_h_32 = unsafe { core::mem::zeroed() };
    __t0.ob_base = __SlateRecord_object_h_172 {
        ob_base: _object {
            __slate_anon_0: __SlateRecord_object_h_122 {
                ob_refcnt_full: 1407378104778752i64,
            },
            ob_type: unsafe { core::ptr::addr_of_mut!(PyTuple_Type) },
        },
        ob_size: 1i64,
    };
    __t0.ob_hash = 1i32.wrapping_neg() as i64;
    __t0.ob_item = [unsafe {
        core::ptr::addr_of_mut!(
            _PyRuntime
                .static_objects
                .singletons
                .strings
                .identifiers
                ._py_object
                ._ascii
                .ob_base
        )
    }];
    __t0
};

static _keywords__44: __SlateAlign16<[*const i8; 2]> = __SlateAlign16([
    (b"object\0".as_ptr() as *mut i8) as *const i8,
    core::ptr::null::<i8>(),
]);

static mut _parser__45: _PyArg_Parser = {
    let mut __t0: _PyArg_Parser = unsafe { core::mem::zeroed() };
    __t0.keywords = unsafe { core::ptr::addr_of!(_keywords__44.0) as *const *const i8 };
    __t0.fname = (b"memoryview\0".as_ptr() as *mut i8) as *const i8;
    __t0.kwtuple = unsafe { core::ptr::addr_of_mut!(_kwtuple__32.ob_base.ob_base) };
    __t0
};

static memoryview__from_flags__doc__: __SlateAlign16<[i8; 107]> = __SlateAlign16([
    95i8, 102i8, 114i8, 111i8, 109i8, 95i8, 102i8, 108i8, 97i8, 103i8, 115i8, 40i8, 36i8, 116i8,
    121i8, 112i8, 101i8, 44i8, 32i8, 47i8, 44i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8,
    44i8, 32i8, 102i8, 108i8, 97i8, 103i8, 115i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 67i8, 114i8,
    101i8, 97i8, 116i8, 101i8, 32i8, 97i8, 32i8, 110i8, 101i8, 119i8, 32i8, 109i8, 101i8, 109i8,
    111i8, 114i8, 121i8, 118i8, 105i8, 101i8, 119i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8,
    32i8, 119i8, 104i8, 105i8, 99i8, 104i8, 32i8, 114i8, 101i8, 102i8, 101i8, 114i8, 101i8, 110i8,
    99i8, 101i8, 115i8, 32i8, 116i8, 104i8, 101i8, 32i8, 103i8, 105i8, 118i8, 101i8, 110i8, 32i8,
    111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.memoryobject.memoryview__from_flags")]
extern "C-unwind" fn memoryview__from_flags(
    r#type: *mut _object,
    mut args: *const *mut _object,
    nargs: i64,
    kwnames: *mut _object,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut flags: i32 = unsafe { core::mem::zeroed() };
    let mut object: *mut _object = unsafe { core::mem::zeroed() };
    let mut argsbuf: core::mem::MaybeUninit<__SlateAlign16<[*mut _object; 2]>> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_3: *mut [*mut _object; 2] =
        core::ptr::addr_of_mut!(argsbuf) as *mut [*mut _object; 2];
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        args = _PyArg_UnpackKeywords!(
            args,
            nargs,
            core::ptr::null_mut::<()>(),
            kwnames,
            core::ptr::addr_of_mut!(_parser__105),
            2i32,
            2i32,
            0i32,
            0i32,
            (*__slate_slot_3).as_mut_ptr() as *mut *mut _object
        );
        // minpos
        // maxpos
        // minkw
        // varpos
        if !args.is_null() {
            object = *args.wrapping_offset(0 as isize);
            flags = PyLong_AsInt(*args.wrapping_offset(1 as isize));
            if flags == 1i32.wrapping_neg() {
                __v0 = !PyErr_Occurred().is_null();
            } else {
                __v0 = false;
            }
            if !__v0 {
                return_value =
                    memoryview__from_flags_impl(r#type as *mut _typeobject, object, flags);
            }
        }
        return return_value;
    }
}

static mut _kwtuple__92: __SlateRecord_memoryobject_c_h_92 = {
    let mut __t0: __SlateRecord_memoryobject_c_h_92 = unsafe { core::mem::zeroed() };
    __t0.ob_base = __SlateRecord_object_h_172 {
        ob_base: _object {
            __slate_anon_0: __SlateRecord_object_h_122 {
                ob_refcnt_full: 1407378104778752i64,
            },
            ob_type: unsafe { core::ptr::addr_of_mut!(PyTuple_Type) },
        },
        ob_size: 2i64,
    };
    __t0.ob_hash = 1i32.wrapping_neg() as i64;
    __t0.ob_item = [
        unsafe {
            core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py_object
                    ._ascii
                    .ob_base
            )
        },
        unsafe {
            core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py_flags
                    ._ascii
                    .ob_base
            )
        },
    ];
    __t0
};

static _keywords__104: __SlateAlign16<[*const i8; 3]> = __SlateAlign16([
    (b"object\0".as_ptr() as *mut i8) as *const i8,
    (b"flags\0".as_ptr() as *mut i8) as *const i8,
    core::ptr::null::<i8>(),
]);

static mut _parser__105: _PyArg_Parser = {
    let mut __t0: _PyArg_Parser = unsafe { core::mem::zeroed() };
    __t0.keywords = unsafe { core::ptr::addr_of!(_keywords__104.0) as *const *const i8 };
    __t0.fname = (b"_from_flags\0".as_ptr() as *mut i8) as *const i8;
    __t0.kwtuple = unsafe { core::ptr::addr_of_mut!(_kwtuple__92.ob_base.ob_base) };
    __t0
};

static memoryview_release__doc__: __SlateAlign16<[i8; 86]> = __SlateAlign16([
    114i8, 101i8, 108i8, 101i8, 97i8, 115i8, 101i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8,
    32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 108i8, 101i8, 97i8, 115i8, 101i8,
    32i8, 116i8, 104i8, 101i8, 32i8, 117i8, 110i8, 100i8, 101i8, 114i8, 108i8, 121i8, 105i8, 110i8,
    103i8, 32i8, 98i8, 117i8, 102i8, 102i8, 101i8, 114i8, 32i8, 101i8, 120i8, 112i8, 111i8, 115i8,
    101i8, 100i8, 32i8, 98i8, 121i8, 32i8, 116i8, 104i8, 101i8, 32i8, 109i8, 101i8, 109i8, 111i8,
    114i8, 121i8, 118i8, 105i8, 101i8, 119i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 46i8,
    0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.memoryobject.memoryview_release")]
extern "C-unwind" fn memoryview_release(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    memoryview_release_impl(self_ as *mut __SlateRecord_memoryobject_h_36)
}

static memoryview_cast__doc__: __SlateAlign16<[i8; 96]> = __SlateAlign16([
    99i8, 97i8, 115i8, 116i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8, 47i8, 44i8, 32i8,
    102i8, 111i8, 114i8, 109i8, 97i8, 116i8, 44i8, 32i8, 115i8, 104i8, 97i8, 112i8, 101i8, 61i8,
    60i8, 117i8, 110i8, 114i8, 101i8, 112i8, 114i8, 101i8, 115i8, 101i8, 110i8, 116i8, 97i8, 98i8,
    108i8, 101i8, 62i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 67i8, 97i8, 115i8, 116i8, 32i8, 97i8,
    32i8, 109i8, 101i8, 109i8, 111i8, 114i8, 121i8, 118i8, 105i8, 101i8, 119i8, 32i8, 116i8, 111i8,
    32i8, 97i8, 32i8, 110i8, 101i8, 119i8, 32i8, 102i8, 111i8, 114i8, 109i8, 97i8, 116i8, 32i8,
    111i8, 114i8, 32i8, 115i8, 104i8, 97i8, 112i8, 101i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.memoryobject.memoryview_cast")]
extern "C-unwind" fn memoryview_cast(
    self_: *mut _object,
    mut args: *const *mut _object,
    nargs: i64,
    kwnames: *mut _object,
) -> *mut _object {
    let mut shape: *mut _object = unsafe { core::mem::zeroed() };
    let mut format: *mut _object = unsafe { core::mem::zeroed() };
    let mut __v2: i64 = unsafe { core::mem::zeroed() };
    let mut noptargs: i64 = unsafe { core::mem::zeroed() };
    let mut argsbuf: core::mem::MaybeUninit<__SlateAlign16<[*mut _object; 2]>> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_4: *mut [*mut _object; 2] =
        core::ptr::addr_of_mut!(argsbuf) as *mut [*mut _object; 2];
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        if !kwnames.is_null() {
            __v2 = PyTuple_GET_SIZE!(kwnames);
        } else {
            __v2 = 0;
        }
        noptargs = nargs.wrapping_add(__v2).wrapping_sub(1);
        shape = core::ptr::null_mut::<_object>();
        args = _PyArg_UnpackKeywords!(
            args,
            nargs,
            core::ptr::null_mut::<()>(),
            kwnames,
            core::ptr::addr_of_mut!(_parser__187),
            1i32,
            2i32,
            0i32,
            0i32,
            (*__slate_slot_4).as_mut_ptr() as *mut *mut _object
        );
        // minpos
        // maxpos
        // minkw
        // varpos
        if !args.is_null() {
            if PyUnicode_Check!(unsafe { *args.wrapping_offset(0 as isize) }) == 0 {
                _PyArg_BadArgument(
                    c"cast".as_ptr(),
                    c"argument 'format'".as_ptr(),
                    c"str".as_ptr(),
                    *args.wrapping_offset(0 as isize),
                );
            } else {
                format = *args.wrapping_offset(0 as isize);
                if noptargs != 0 {
                    shape = *args.wrapping_offset(1 as isize);
                }
                return_value = memoryview_cast_impl(
                    self_ as *mut __SlateRecord_memoryobject_h_36,
                    format,
                    shape,
                );
            }
        }
        return return_value;
    }
}

static mut _kwtuple__174: __SlateRecord_memoryobject_c_h_174 = {
    let mut __t0: __SlateRecord_memoryobject_c_h_174 = unsafe { core::mem::zeroed() };
    __t0.ob_base = __SlateRecord_object_h_172 {
        ob_base: _object {
            __slate_anon_0: __SlateRecord_object_h_122 {
                ob_refcnt_full: 1407378104778752i64,
            },
            ob_type: unsafe { core::ptr::addr_of_mut!(PyTuple_Type) },
        },
        ob_size: 2i64,
    };
    __t0.ob_hash = 1i32.wrapping_neg() as i64;
    __t0.ob_item = [
        unsafe {
            core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py_format
                    ._ascii
                    .ob_base
            )
        },
        unsafe {
            core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py_shape
                    ._ascii
                    .ob_base
            )
        },
    ];
    __t0
};

static _keywords__186: __SlateAlign16<[*const i8; 3]> = __SlateAlign16([
    (b"format\0".as_ptr() as *mut i8) as *const i8,
    (b"shape\0".as_ptr() as *mut i8) as *const i8,
    core::ptr::null::<i8>(),
]);

static mut _parser__187: _PyArg_Parser = {
    let mut __t0: _PyArg_Parser = unsafe { core::mem::zeroed() };
    __t0.keywords = unsafe { core::ptr::addr_of!(_keywords__186.0) as *const *const i8 };
    __t0.fname = (b"cast\0".as_ptr() as *mut i8) as *const i8;
    __t0.kwtuple = unsafe { core::ptr::addr_of_mut!(_kwtuple__174.ob_base.ob_base) };
    __t0
};

static memoryview_toreadonly__doc__: __SlateAlign16<[i8; 70]> = __SlateAlign16([
    116i8, 111i8, 114i8, 101i8, 97i8, 100i8, 111i8, 110i8, 108i8, 121i8, 40i8, 36i8, 115i8, 101i8,
    108i8, 102i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8,
    114i8, 110i8, 32i8, 97i8, 32i8, 114i8, 101i8, 97i8, 100i8, 111i8, 110i8, 108i8, 121i8, 32i8,
    118i8, 101i8, 114i8, 115i8, 105i8, 111i8, 110i8, 32i8, 111i8, 102i8, 32i8, 116i8, 104i8, 101i8,
    32i8, 109i8, 101i8, 109i8, 111i8, 114i8, 121i8, 118i8, 105i8, 101i8, 119i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.memoryobject.memoryview_toreadonly")]
extern "C-unwind" fn memoryview_toreadonly(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    memoryview_toreadonly_impl(self_ as *mut __SlateRecord_memoryobject_h_36)
}

static memoryview_tolist__doc__: __SlateAlign16<[i8; 74]> = __SlateAlign16([
    116i8, 111i8, 108i8, 105i8, 115i8, 116i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8,
    47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 116i8,
    104i8, 101i8, 32i8, 100i8, 97i8, 116i8, 97i8, 32i8, 105i8, 110i8, 32i8, 116i8, 104i8, 101i8,
    32i8, 98i8, 117i8, 102i8, 102i8, 101i8, 114i8, 32i8, 97i8, 115i8, 32i8, 97i8, 32i8, 108i8,
    105i8, 115i8, 116i8, 32i8, 111i8, 102i8, 32i8, 101i8, 108i8, 101i8, 109i8, 101i8, 110i8, 116i8,
    115i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.memoryobject.memoryview_tolist")]
extern "C-unwind" fn memoryview_tolist(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    memoryview_tolist_impl(self_ as *mut __SlateRecord_memoryobject_h_36)
}

use slate_support::memcmp;
use slate_support::memcpy;
use slate_support::memmove;
use slate_support::strcmp;
use slate_support::strcpy;
use slate_support::strlen;

static memoryview_tobytes__doc__: __SlateAlign16<[i8; 431]> = __SlateAlign16([
    116i8, 111i8, 98i8, 121i8, 116i8, 101i8, 115i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8,
    32i8, 47i8, 44i8, 32i8, 111i8, 114i8, 100i8, 101i8, 114i8, 61i8, 39i8, 67i8, 39i8, 41i8, 10i8,
    45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 116i8, 104i8, 101i8,
    32i8, 100i8, 97i8, 116i8, 97i8, 32i8, 105i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 98i8,
    117i8, 102i8, 102i8, 101i8, 114i8, 32i8, 97i8, 115i8, 32i8, 97i8, 32i8, 98i8, 121i8, 116i8,
    101i8, 32i8, 115i8, 116i8, 114i8, 105i8, 110i8, 103i8, 46i8, 10i8, 10i8, 79i8, 114i8, 100i8,
    101i8, 114i8, 32i8, 99i8, 97i8, 110i8, 32i8, 98i8, 101i8, 32i8, 123i8, 39i8, 67i8, 39i8, 44i8,
    32i8, 39i8, 70i8, 39i8, 44i8, 32i8, 39i8, 65i8, 39i8, 125i8, 46i8, 32i8, 32i8, 87i8, 104i8,
    101i8, 110i8, 32i8, 111i8, 114i8, 100i8, 101i8, 114i8, 32i8, 105i8, 115i8, 32i8, 39i8, 67i8,
    39i8, 32i8, 111i8, 114i8, 32i8, 39i8, 70i8, 39i8, 44i8, 32i8, 116i8, 104i8, 101i8, 32i8, 100i8,
    97i8, 116i8, 97i8, 32i8, 111i8, 102i8, 10i8, 116i8, 104i8, 101i8, 32i8, 111i8, 114i8, 105i8,
    103i8, 105i8, 110i8, 97i8, 108i8, 32i8, 97i8, 114i8, 114i8, 97i8, 121i8, 32i8, 105i8, 115i8,
    32i8, 99i8, 111i8, 110i8, 118i8, 101i8, 114i8, 116i8, 101i8, 100i8, 32i8, 116i8, 111i8, 32i8,
    67i8, 32i8, 111i8, 114i8, 32i8, 70i8, 111i8, 114i8, 116i8, 114i8, 97i8, 110i8, 32i8, 111i8,
    114i8, 100i8, 101i8, 114i8, 46i8, 32i8, 32i8, 70i8, 111i8, 114i8, 10i8, 99i8, 111i8, 110i8,
    116i8, 105i8, 103i8, 117i8, 111i8, 117i8, 115i8, 32i8, 118i8, 105i8, 101i8, 119i8, 115i8, 44i8,
    32i8, 39i8, 65i8, 39i8, 32i8, 114i8, 101i8, 116i8, 117i8, 114i8, 110i8, 115i8, 32i8, 97i8,
    110i8, 32i8, 101i8, 120i8, 97i8, 99i8, 116i8, 32i8, 99i8, 111i8, 112i8, 121i8, 32i8, 111i8,
    102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 112i8, 104i8, 121i8, 115i8, 105i8, 99i8, 97i8, 108i8,
    32i8, 109i8, 101i8, 109i8, 111i8, 114i8, 121i8, 46i8, 10i8, 73i8, 110i8, 32i8, 112i8, 97i8,
    114i8, 116i8, 105i8, 99i8, 117i8, 108i8, 97i8, 114i8, 44i8, 32i8, 105i8, 110i8, 45i8, 109i8,
    101i8, 109i8, 111i8, 114i8, 121i8, 32i8, 70i8, 111i8, 114i8, 116i8, 114i8, 97i8, 110i8, 32i8,
    111i8, 114i8, 100i8, 101i8, 114i8, 32i8, 105i8, 115i8, 32i8, 112i8, 114i8, 101i8, 115i8, 101i8,
    114i8, 118i8, 101i8, 100i8, 46i8, 32i8, 32i8, 70i8, 111i8, 114i8, 10i8, 110i8, 111i8, 110i8,
    45i8, 99i8, 111i8, 110i8, 116i8, 105i8, 103i8, 117i8, 111i8, 117i8, 115i8, 32i8, 118i8, 105i8,
    101i8, 119i8, 115i8, 44i8, 32i8, 116i8, 104i8, 101i8, 32i8, 100i8, 97i8, 116i8, 97i8, 32i8,
    105i8, 115i8, 32i8, 99i8, 111i8, 110i8, 118i8, 101i8, 114i8, 116i8, 101i8, 100i8, 32i8, 116i8,
    111i8, 32i8, 67i8, 32i8, 102i8, 105i8, 114i8, 115i8, 116i8, 46i8, 32i8, 32i8, 111i8, 114i8,
    100i8, 101i8, 114i8, 61i8, 78i8, 111i8, 110i8, 101i8, 10i8, 105i8, 115i8, 32i8, 116i8, 104i8,
    101i8, 32i8, 115i8, 97i8, 109i8, 101i8, 32i8, 97i8, 115i8, 32i8, 111i8, 114i8, 100i8, 101i8,
    114i8, 61i8, 39i8, 67i8, 39i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.memoryobject.memoryview_tobytes")]
extern "C-unwind" fn memoryview_tobytes(
    self_: *mut _object,
    mut args: *const *mut _object,
    nargs: i64,
    kwnames: *mut _object,
) -> *mut _object {
    let mut order_length: i64 = unsafe { core::mem::zeroed() };
    let mut order: *const i8 = unsafe { core::mem::zeroed() };
    let mut __v2: i64 = unsafe { core::mem::zeroed() };
    let mut noptargs: i64 = unsafe { core::mem::zeroed() };
    let mut argsbuf: [*mut _object; 1] = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        if !kwnames.is_null() {
            __v2 = PyTuple_GET_SIZE!(kwnames);
        } else {
            __v2 = 0;
        }
        '__join_0: {
            noptargs = nargs.wrapping_add(__v2).wrapping_sub(0);
            order = core::ptr::null::<i8>();
            args = _PyArg_UnpackKeywords!(
                args,
                nargs,
                core::ptr::null_mut::<()>(),
                kwnames,
                core::ptr::addr_of_mut!(_parser__299),
                0i32,
                1i32,
                0i32,
                0i32,
                argsbuf.as_mut_ptr() as *mut *mut _object
            );
            // minpos
            // maxpos
            // minkw
            // varpos
            if !args.is_null() {
                if noptargs != 0 {
                    if *args.wrapping_offset(0 as isize) == core::ptr::addr_of_mut!(_Py_NoneStruct)
                    {
                        order = core::ptr::null::<i8>();
                    } else {
                        if PyUnicode_Check!(unsafe { *args.wrapping_offset(0 as isize) }) != 0 {
                            order = PyUnicode_AsUTF8AndSize(
                                *args.wrapping_offset(0 as isize),
                                core::ptr::addr_of_mut!(order_length),
                            );
                            if order.is_null() {
                                break '__join_0;
                            } else {
                                if (strlen(order as *const core::ffi::c_char) as u64)
                                    != (order_length as u64)
                                {
                                    PyErr_SetString(
                                        PyExc_ValueError,
                                        c"embedded null character".as_ptr(),
                                    );
                                    break '__join_0;
                                }
                            }
                        } else {
                            _PyArg_BadArgument(
                                c"tobytes".as_ptr(),
                                c"argument 'order'".as_ptr(),
                                c"str or None".as_ptr(),
                                *args.wrapping_offset(0 as isize),
                            );
                            break '__join_0;
                        }
                    }
                }
                return_value =
                    memoryview_tobytes_impl(self_ as *mut __SlateRecord_memoryobject_h_36, order);
            }
        }
        return return_value;
    }
}

static mut _kwtuple__286: __SlateRecord_memoryobject_c_h_286 = {
    let mut __t0: __SlateRecord_memoryobject_c_h_286 = unsafe { core::mem::zeroed() };
    __t0.ob_base = __SlateRecord_object_h_172 {
        ob_base: _object {
            __slate_anon_0: __SlateRecord_object_h_122 {
                ob_refcnt_full: 1407378104778752i64,
            },
            ob_type: unsafe { core::ptr::addr_of_mut!(PyTuple_Type) },
        },
        ob_size: 1i64,
    };
    __t0.ob_hash = 1i32.wrapping_neg() as i64;
    __t0.ob_item = [unsafe {
        core::ptr::addr_of_mut!(
            _PyRuntime
                .static_objects
                .singletons
                .strings
                .identifiers
                ._py_order
                ._ascii
                .ob_base
        )
    }];
    __t0
};

static _keywords__298: __SlateAlign16<[*const i8; 2]> = __SlateAlign16([
    (b"order\0".as_ptr() as *mut i8) as *const i8,
    core::ptr::null::<i8>(),
]);

static mut _parser__299: _PyArg_Parser = {
    let mut __t0: _PyArg_Parser = unsafe { core::mem::zeroed() };
    __t0.keywords = unsafe { core::ptr::addr_of!(_keywords__298.0) as *const *const i8 };
    __t0.fname = (b"tobytes\0".as_ptr() as *mut i8) as *const i8;
    __t0.kwtuple = unsafe { core::ptr::addr_of_mut!(_kwtuple__286.ob_base.ob_base) };
    __t0
};

static memoryview_hex__doc__: __SlateAlign16<[i8; 497]> = __SlateAlign16([
    104i8, 101i8, 120i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8, 47i8, 44i8, 32i8,
    115i8, 101i8, 112i8, 61i8, 60i8, 117i8, 110i8, 114i8, 101i8, 112i8, 114i8, 101i8, 115i8, 101i8,
    110i8, 116i8, 97i8, 98i8, 108i8, 101i8, 62i8, 44i8, 32i8, 98i8, 121i8, 116i8, 101i8, 115i8,
    95i8, 112i8, 101i8, 114i8, 95i8, 115i8, 101i8, 112i8, 61i8, 49i8, 41i8, 10i8, 45i8, 45i8, 10i8,
    10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 100i8, 97i8,
    116i8, 97i8, 32i8, 105i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 98i8, 117i8, 102i8, 102i8,
    101i8, 114i8, 32i8, 97i8, 115i8, 32i8, 97i8, 32i8, 115i8, 116i8, 114i8, 32i8, 111i8, 102i8,
    32i8, 104i8, 101i8, 120i8, 97i8, 100i8, 101i8, 99i8, 105i8, 109i8, 97i8, 108i8, 32i8, 110i8,
    117i8, 109i8, 98i8, 101i8, 114i8, 115i8, 46i8, 10i8, 10i8, 32i8, 32i8, 115i8, 101i8, 112i8,
    10i8, 32i8, 32i8, 32i8, 32i8, 65i8, 110i8, 32i8, 111i8, 112i8, 116i8, 105i8, 111i8, 110i8,
    97i8, 108i8, 32i8, 115i8, 105i8, 110i8, 103i8, 108i8, 101i8, 32i8, 99i8, 104i8, 97i8, 114i8,
    97i8, 99i8, 116i8, 101i8, 114i8, 32i8, 111i8, 114i8, 32i8, 98i8, 121i8, 116i8, 101i8, 32i8,
    116i8, 111i8, 32i8, 115i8, 101i8, 112i8, 97i8, 114i8, 97i8, 116i8, 101i8, 32i8, 104i8, 101i8,
    120i8, 32i8, 98i8, 121i8, 116i8, 101i8, 115i8, 46i8, 10i8, 32i8, 32i8, 98i8, 121i8, 116i8,
    101i8, 115i8, 95i8, 112i8, 101i8, 114i8, 95i8, 115i8, 101i8, 112i8, 10i8, 32i8, 32i8, 32i8,
    32i8, 72i8, 111i8, 119i8, 32i8, 109i8, 97i8, 110i8, 121i8, 32i8, 98i8, 121i8, 116i8, 101i8,
    115i8, 32i8, 98i8, 101i8, 116i8, 119i8, 101i8, 101i8, 110i8, 32i8, 115i8, 101i8, 112i8, 97i8,
    114i8, 97i8, 116i8, 111i8, 114i8, 115i8, 46i8, 32i8, 32i8, 80i8, 111i8, 115i8, 105i8, 116i8,
    105i8, 118i8, 101i8, 32i8, 118i8, 97i8, 108i8, 117i8, 101i8, 115i8, 32i8, 99i8, 111i8, 117i8,
    110i8, 116i8, 32i8, 102i8, 114i8, 111i8, 109i8, 10i8, 32i8, 32i8, 32i8, 32i8, 116i8, 104i8,
    101i8, 32i8, 114i8, 105i8, 103i8, 104i8, 116i8, 44i8, 32i8, 110i8, 101i8, 103i8, 97i8, 116i8,
    105i8, 118i8, 101i8, 32i8, 118i8, 97i8, 108i8, 117i8, 101i8, 115i8, 32i8, 99i8, 111i8, 117i8,
    110i8, 116i8, 32i8, 102i8, 114i8, 111i8, 109i8, 32i8, 116i8, 104i8, 101i8, 32i8, 108i8, 101i8,
    102i8, 116i8, 46i8, 10i8, 10i8, 69i8, 120i8, 97i8, 109i8, 112i8, 108i8, 101i8, 58i8, 10i8,
    62i8, 62i8, 62i8, 32i8, 118i8, 97i8, 108i8, 117i8, 101i8, 32i8, 61i8, 32i8, 109i8, 101i8,
    109i8, 111i8, 114i8, 121i8, 118i8, 105i8, 101i8, 119i8, 40i8, 98i8, 39i8, 92i8, 120i8, 98i8,
    57i8, 92i8, 120i8, 48i8, 49i8, 92i8, 120i8, 101i8, 102i8, 39i8, 41i8, 10i8, 62i8, 62i8, 62i8,
    32i8, 118i8, 97i8, 108i8, 117i8, 101i8, 46i8, 104i8, 101i8, 120i8, 40i8, 41i8, 10i8, 39i8,
    98i8, 57i8, 48i8, 49i8, 101i8, 102i8, 39i8, 10i8, 62i8, 62i8, 62i8, 32i8, 118i8, 97i8, 108i8,
    117i8, 101i8, 46i8, 104i8, 101i8, 120i8, 40i8, 39i8, 58i8, 39i8, 41i8, 10i8, 39i8, 98i8, 57i8,
    58i8, 48i8, 49i8, 58i8, 101i8, 102i8, 39i8, 10i8, 62i8, 62i8, 62i8, 32i8, 118i8, 97i8, 108i8,
    117i8, 101i8, 46i8, 104i8, 101i8, 120i8, 40i8, 39i8, 58i8, 39i8, 44i8, 32i8, 50i8, 41i8, 10i8,
    39i8, 98i8, 57i8, 58i8, 48i8, 49i8, 101i8, 102i8, 39i8, 10i8, 62i8, 62i8, 62i8, 32i8, 118i8,
    97i8, 108i8, 117i8, 101i8, 46i8, 104i8, 101i8, 120i8, 40i8, 39i8, 58i8, 39i8, 44i8, 32i8, 45i8,
    50i8, 41i8, 10i8, 39i8, 98i8, 57i8, 48i8, 49i8, 58i8, 101i8, 102i8, 39i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.memoryobject.memoryview_hex")]
extern "C-unwind" fn memoryview_hex(
    self_: *mut _object,
    mut args: *const *mut _object,
    nargs: i64,
    kwnames: *mut _object,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut bytes_per_sep: i32 = unsafe { core::mem::zeroed() };
    let mut sep: *mut _object = unsafe { core::mem::zeroed() };
    let mut __v5: i64 = unsafe { core::mem::zeroed() };
    let mut noptargs: i64 = unsafe { core::mem::zeroed() };
    let mut argsbuf: core::mem::MaybeUninit<__SlateAlign16<[*mut _object; 2]>> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_7: *mut [*mut _object; 2] =
        core::ptr::addr_of_mut!(argsbuf) as *mut [*mut _object; 2];
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        if !kwnames.is_null() {
            __v5 = PyTuple_GET_SIZE!(kwnames);
        } else {
            __v5 = 0;
        }
        '__join_0: {
            noptargs = nargs.wrapping_add(__v5).wrapping_sub(0);
            sep = core::ptr::null_mut::<_object>();
            bytes_per_sep = 1;
            args = _PyArg_UnpackKeywords!(
                args,
                nargs,
                core::ptr::null_mut::<()>(),
                kwnames,
                core::ptr::addr_of_mut!(_parser__397),
                0i32,
                2i32,
                0i32,
                0i32,
                (*__slate_slot_7).as_mut_ptr() as *mut *mut _object
            );
            // minpos
            // maxpos
            // minkw
            // varpos
            if !args.is_null() {
                '__join_1: {
                    if noptargs != 0 {
                        if !unsafe { *args.wrapping_offset(0 as isize) }.is_null() {
                            sep = *args.wrapping_offset(0 as isize);
                            let __v1: i64 = noptargs.wrapping_sub(1);
                            noptargs = __v1;
                            if __v1 == 0 {
                                break '__join_1;
                            }
                        }
                        bytes_per_sep = PyLong_AsInt(*args.wrapping_offset(1 as isize));
                        if bytes_per_sep == 1i32.wrapping_neg() {
                            __v0 = !PyErr_Occurred().is_null();
                        } else {
                            __v0 = false;
                        }
                        if __v0 {
                            break '__join_0;
                        }
                    }
                }
                return_value = memoryview_hex_impl(
                    self_ as *mut __SlateRecord_memoryobject_h_36,
                    sep,
                    bytes_per_sep,
                );
            }
        }
        return return_value;
    }
}

static mut _kwtuple__384: __SlateRecord_memoryobject_c_h_384 = {
    let mut __t0: __SlateRecord_memoryobject_c_h_384 = unsafe { core::mem::zeroed() };
    __t0.ob_base = __SlateRecord_object_h_172 {
        ob_base: _object {
            __slate_anon_0: __SlateRecord_object_h_122 {
                ob_refcnt_full: 1407378104778752i64,
            },
            ob_type: unsafe { core::ptr::addr_of_mut!(PyTuple_Type) },
        },
        ob_size: 2i64,
    };
    __t0.ob_hash = 1i32.wrapping_neg() as i64;
    __t0.ob_item = [
        unsafe {
            core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py_sep
                    ._ascii
                    .ob_base
            )
        },
        unsafe {
            core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py_bytes_per_sep
                    ._ascii
                    .ob_base
            )
        },
    ];
    __t0
};

static _keywords__396: __SlateAlign16<[*const i8; 3]> = __SlateAlign16([
    (b"sep\0".as_ptr() as *mut i8) as *const i8,
    (b"bytes_per_sep\0".as_ptr() as *mut i8) as *const i8,
    core::ptr::null::<i8>(),
]);

static mut _parser__397: _PyArg_Parser = {
    let mut __t0: _PyArg_Parser = unsafe { core::mem::zeroed() };
    __t0.keywords = unsafe { core::ptr::addr_of!(_keywords__396.0) as *const *const i8 };
    __t0.fname = (b"hex\0".as_ptr() as *mut i8) as *const i8;
    __t0.kwtuple = unsafe { core::ptr::addr_of_mut!(_kwtuple__384.ob_base.ob_base) };
    __t0
};

static memoryview_count__doc__: __SlateAlign16<[i8; 71]> = __SlateAlign16([
    99i8, 111i8, 117i8, 110i8, 116i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8, 118i8,
    97i8, 108i8, 117i8, 101i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 67i8, 111i8,
    117i8, 110i8, 116i8, 32i8, 116i8, 104i8, 101i8, 32i8, 110i8, 117i8, 109i8, 98i8, 101i8, 114i8,
    32i8, 111i8, 102i8, 32i8, 111i8, 99i8, 99i8, 117i8, 114i8, 114i8, 101i8, 110i8, 99i8, 101i8,
    115i8, 32i8, 111i8, 102i8, 32i8, 97i8, 32i8, 118i8, 97i8, 108i8, 117i8, 101i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.memoryobject.memoryview_count")]
extern "C-unwind" fn memoryview_count(self_: *mut _object, value: *mut _object) -> *mut _object {
    let mut return_value: *mut _object = core::ptr::null_mut::<_object>();
    return_value = memoryview_count_impl(self_ as *mut __SlateRecord_memoryobject_h_36, value);
    return_value
}

static memoryview_index__doc__: __SlateAlign16<[i8; 155]> = __SlateAlign16([
    105i8, 110i8, 100i8, 101i8, 120i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8, 118i8,
    97i8, 108i8, 117i8, 101i8, 44i8, 32i8, 115i8, 116i8, 97i8, 114i8, 116i8, 61i8, 48i8, 44i8,
    32i8, 115i8, 116i8, 111i8, 112i8, 61i8, 115i8, 121i8, 115i8, 46i8, 109i8, 97i8, 120i8, 115i8,
    105i8, 122i8, 101i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8,
    117i8, 114i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 105i8, 110i8, 100i8, 101i8, 120i8, 32i8,
    111i8, 102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 102i8, 105i8, 114i8, 115i8, 116i8, 32i8, 111i8,
    99i8, 99i8, 117i8, 114i8, 114i8, 101i8, 110i8, 99i8, 101i8, 32i8, 111i8, 102i8, 32i8, 97i8,
    32i8, 118i8, 97i8, 108i8, 117i8, 101i8, 46i8, 10i8, 10i8, 82i8, 97i8, 105i8, 115i8, 101i8,
    115i8, 32i8, 86i8, 97i8, 108i8, 117i8, 101i8, 69i8, 114i8, 114i8, 111i8, 114i8, 32i8, 105i8,
    102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 118i8, 97i8, 108i8, 117i8, 101i8, 32i8, 105i8, 115i8,
    32i8, 110i8, 111i8, 116i8, 32i8, 112i8, 114i8, 101i8, 115i8, 101i8, 110i8, 116i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.memoryobject.memoryview_index")]
extern "C-unwind" fn memoryview_index(
    self_: *mut _object,
    args: *const *mut _object,
    nargs: i64,
) -> *mut _object {
    let mut stop: i64 = unsafe { core::mem::zeroed() };
    let mut start: i64 = unsafe { core::mem::zeroed() };
    let mut value: *mut _object = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_0: {
            return_value = core::ptr::null_mut::<_object>();
            start = 0;
            stop = 9223372036854775807;
            if _PyArg_CheckPositional!(c"index".as_ptr() as *mut i8, nargs, 1i32, 3i32) {
                value = *args.wrapping_offset(0 as isize);
                if nargs >= 2 {
                    if _PyEval_SliceIndexNotNone(
                        *args.wrapping_offset(1 as isize),
                        core::ptr::addr_of_mut!(start),
                    ) == 0
                    {
                        break '__join_0;
                    } else {
                        if nargs >= 3 {
                            if _PyEval_SliceIndexNotNone(
                                *args.wrapping_offset(2 as isize),
                                core::ptr::addr_of_mut!(stop),
                            ) == 0
                            {
                                break '__join_0;
                            }
                        }
                    }
                }
                return_value = memoryview_index_impl(
                    self_ as *mut __SlateRecord_memoryobject_h_36,
                    value,
                    start,
                    stop,
                );
            }
        }
        return return_value;
    }
}

// _PyIndex_Check()
// _PyManagedBuffer_Type
// _PyObject_GC_UNTRACK()
// _Py_strhex_with_sep()
// offsetof()
// [clinic input]
// class memoryview "PyMemoryViewObject *" "&PyMemoryView_Type"
// [clinic start generated code]
//
// [clinic end generated code: output=da39a3ee5e6b4b0d input=e2e49d2192835219]
//                           ManagedBuffer Object
// ManagedBuffer Object:
//
//   The purpose of this object is to facilitate the handling of chained
//   memoryviews that have the same underlying exporting object. PEP-3118
//   allows the underlying object to change while a view is exported. This
//   could lead to unexpected results when constructing a new memoryview
//   from an existing memoryview.
//
//   Rather than repeatedly redirecting buffer requests to the original base
//   object, all chained memoryviews use a single buffer snapshot. This
//   snapshot is generated by the constructor _PyManagedBuffer_FromObject().
//
// Ownership rules:
//
//   The master buffer inside a managed buffer is filled in by the original
//   base object. shape, strides, suboffsets and format are read-only for
//   all consumers.
//
//   A memoryview's buffer is a private copy of the exporter's buffer. shape,
//   strides and suboffsets belong to the memoryview and are thus writable.
//
//   If a memoryview itself exports several buffers via memory_getbuf(), all
//   buffer copies share shape, strides and suboffsets. In this case, the
//   arrays are NOT writable.
//
// Reference count assumptions:
//
//   The 'obj' member of a Py_buffer must either be NULL or refer to the
//   exporting base object. In the Python codebase, all getbufferprocs
//   return a new reference to view.obj (example: bytes_buffer_getbuffer()).
//
//   PyBuffer_Release() decrements view.obj (if non-NULL), so the
//   releasebufferprocs must NOT decrement view.obj.
fn mbuf_alloc() -> *mut __SlateRecord_memoryobject_h_16 {
    let mbuf: *mut __SlateRecord_memoryobject_h_16 =
        PyObject_GC_New!(__SlateRecord_memoryobject_h_16, unsafe {
            core::ptr::addr_of_mut!(_PyManagedBuffer_Type)
        });
    if mbuf.is_null() {
        return core::ptr::null_mut::<__SlateRecord_memoryobject_h_16>();
    }
    unsafe {
        (*mbuf).flags = 0;
    }
    unsafe {
        (*mbuf).exports = 0;
    }
    unsafe {
        (*mbuf).master.obj = core::ptr::null_mut::<_object>();
    }
    _PyObject_GC_TRACK!(mbuf);
    mbuf
}

fn _PyManagedBuffer_FromObject(base: *mut _object, flags: i32) -> *mut _object {
    let mut mbuf: *mut __SlateRecord_memoryobject_h_16 = unsafe { core::mem::zeroed() };
    mbuf = mbuf_alloc();
    if mbuf.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    if (unsafe { PyObject_GetBuffer(base, core::ptr::addr_of_mut!((*mbuf).master), flags) }) < 0 {
        unsafe {
            (*mbuf).master.obj = core::ptr::null_mut::<_object>();
        }
        Py_DECREF__2!(mbuf);
        return core::ptr::null_mut::<_object>();
    }
    mbuf as *mut _object
}

fn mbuf_release(self_: *mut __SlateRecord_memoryobject_h_16) {
    if (unsafe { (*self_).flags }) & 1 != 0 {
        return;
    }
    unsafe {
        (*self_).flags = (*self_).flags | 1;
    }
    // PyBuffer_Release() decrements master->obj and sets it to NULL.
    _PyObject_GC_UNTRACK__2!(self_);
    unsafe { PyBuffer_Release(core::ptr::addr_of_mut!((*self_).master)) };
}

#[unsafe(link_section = ".text.slate_distinct.memoryobject.mbuf_dealloc")]
extern "C-unwind" fn mbuf_dealloc(_self: *mut _object) {
    let self_: *mut __SlateRecord_memoryobject_h_16 = _self as *mut __SlateRecord_memoryobject_h_16;
    mbuf_release(self_);
    if (unsafe { (*self_).flags }) & 2 != 0 {
        unsafe { PyMem_Free((*self_).master.format as *mut ()) };
    }
    unsafe { PyObject_GC_Del(self_ as *mut ()) };
}

#[unsafe(link_section = ".text.slate_distinct.memoryobject.mbuf_traverse")]
extern "C-unwind" fn mbuf_traverse(
    _self: *mut _object,
    mut visit: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
    arg: *mut (),
) -> i32 {
    let self_: *mut __SlateRecord_memoryobject_h_16 = _self as *mut __SlateRecord_memoryobject_h_16;
    Py_VISIT!(unsafe { (*self_).master.obj }, visit, arg);
    0
}

#[unsafe(link_section = ".text.slate_distinct.memoryobject.mbuf_clear")]
extern "C-unwind" fn mbuf_clear(_self: *mut _object) -> i32 {
    let self_: *mut __SlateRecord_memoryobject_h_16 = _self as *mut __SlateRecord_memoryobject_h_16;
    mbuf_release(self_);
    0
}

//                             MemoryView Object
//                       Copy memoryview buffers
// The functions in this section take a source and a destination buffer
// with the same logical structure: format, itemsize, ndim and shape
// are identical, with ndim > 0.
//
// NOTE: All buffers are assumed to have PyBUF_FULL information, which
// is the case for memoryviews!
// tp_dealloc
// tp_vectorcall_offset
// tp_getattr
// tp_setattr
// tp_as_async
// tp_repr
// tp_as_number
// tp_as_sequence
// tp_as_mapping
// tp_hash
// tp_call
// tp_str
// tp_getattro
// tp_setattro
// tp_as_buffer
// tp_flags
// tp_doc
// tp_traverse
// tp_clear
fn last_dim_is_contiguous(
    dest: *const __SlateRecord_pybuffer_h_33,
    src: *const __SlateRecord_pybuffer_h_33,
) -> i32 {
    (!HAVE_SUBOFFSETS_IN_LAST_DIM!(dest)
        && !HAVE_SUBOFFSETS_IN_LAST_DIM!(src)
        && (unsafe {
            *unsafe { (*dest).strides }
                .wrapping_offset(unsafe { (*dest).ndim }.wrapping_sub(1) as isize)
        }) == unsafe { (*dest).itemsize }
        && (unsafe {
            *unsafe { (*src).strides }
                .wrapping_offset(unsafe { (*src).ndim }.wrapping_sub(1) as isize)
        }) == unsafe { (*src).itemsize }) as i32
}

/// This is not a general function for determining format equivalence.
/// It is used in copy_single() and copy_buffer() to weed out non-matching
/// formats. Skipping the '@' character is specifically used in slice
/// assignments, where the lvalue is already known to have a single character
/// format. This is a performance hack that could be rewritten (if properly
/// benchmarked).
fn equiv_format(
    dest: *const __SlateRecord_pybuffer_h_33,
    src: *const __SlateRecord_pybuffer_h_33,
) -> i32 {
    let dfmt: *const i8 =
        (if ((unsafe { *unsafe { (*dest).format }.wrapping_offset(0 as isize) }) as i32) == 64 {
            unsafe { (*dest).format }.wrapping_offset(1 as isize)
        } else {
            unsafe { (*dest).format }
        }) as *const i8;
    let sfmt: *const i8 =
        (if ((unsafe { *unsafe { (*src).format }.wrapping_offset(0 as isize) }) as i32) == 64 {
            unsafe { (*src).format }.wrapping_offset(1 as isize)
        } else {
            unsafe { (*src).format }
        }) as *const i8;
    if (unsafe {
        strcmp(
            dfmt as *const core::ffi::c_char,
            sfmt as *const core::ffi::c_char,
        ) as i32
    }) != 0
        || (unsafe { (*dest).itemsize }) != unsafe { (*src).itemsize }
    {
        return 0;
    }
    1
}

/// Two shapes are equivalent if they are either equal or identical up
/// to a zero element at the same position. For example, in NumPy arrays
/// the shapes [1, 0, 5] and [1, 0, 7] are equivalent.
fn equiv_shape(
    dest: *const __SlateRecord_pybuffer_h_33,
    src: *const __SlateRecord_pybuffer_h_33,
) -> i32 {
    let mut i: i32 = 0;
    if (unsafe { (*dest).ndim }) != unsafe { (*src).ndim } {
        return 0;
    }
    while i < unsafe { (*dest).ndim } {
        if (unsafe { *unsafe { (*dest).shape }.wrapping_offset(i as isize) })
            != unsafe { *unsafe { (*src).shape }.wrapping_offset(i as isize) }
        {
            return 0;
        }
        if (unsafe { *unsafe { (*dest).shape }.wrapping_offset(i as isize) }) == 0 {
            break;
        }
        i = i.wrapping_add(1);
    }
    1
}

/// Check that the logical structure of the destination and source buffers
/// is identical.
fn equiv_structure(
    dest: *const __SlateRecord_pybuffer_h_33,
    src: *const __SlateRecord_pybuffer_h_33,
) -> i32 {
    let __v0: bool = equiv_format(dest, src) == 0 || equiv_shape(dest, src) == 0;
    if __v0 {
        unsafe {
            PyErr_SetString(
                PyExc_ValueError,
                c"memoryview assignment: lvalue and rvalue have different structures".as_ptr(),
            )
        };
        return 0;
    }
    1
}

/// Base case for recursive multi-dimensional copying. Contiguous arrays are
/// copied with very little overhead. Assumptions: ndim == 1, mem == NULL or
/// sizeof(mem) == shape[0] * itemsize.
fn copy_base(
    shape: *const i64,
    itemsize: i64,
    mut dptr: *mut i8,
    dstrides: *const i64,
    mut dsuboffsets: *const i64,
    mut sptr: *mut i8,
    sstrides: *const i64,
    mut ssuboffsets: *const i64,
    mem: *mut i8,
) {
    if mem.is_null() {
        // contiguous
        let size: i64 = unsafe { *shape.wrapping_offset(0 as isize) }.wrapping_mul(itemsize);
        if dptr.wrapping_offset(size as isize) < sptr || sptr.wrapping_offset(size as isize) < dptr
        {
            unsafe {
                core::ptr::copy_nonoverlapping(
                    ((sptr as *const ()) as *const core::ffi::c_void) as *const u8,
                    ((dptr as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                    ((size as u64) as usize) as usize,
                )
            };
        } else {
            unsafe {
                core::ptr::copy(
                    ((sptr as *const ()) as *const core::ffi::c_void) as *const u8,
                    ((dptr as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                    ((size as u64) as usize) as usize,
                )
            };
        }
    // no overlapping
    } else {
        let mut p: *mut i8 = unsafe { core::mem::zeroed() };
        let mut i: i64 = 0;
        p = mem;
        while i < unsafe { *shape.wrapping_offset(0 as isize) } {
            let xsptr: *mut i8 = ADJUST_PTR__2!(sptr, ssuboffsets, 0i32);
            unsafe {
                core::ptr::copy_nonoverlapping(
                    ((xsptr as *const ()) as *const core::ffi::c_void) as *const u8,
                    ((p as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                    ((itemsize as u64) as usize) as usize,
                )
            };
            let __v2: *mut i8 = p;
            p = __v2.wrapping_offset(itemsize as isize);
            let __v4: *mut i8 = sptr;
            sptr =
                __v4.wrapping_offset((unsafe { *sstrides.wrapping_offset(0 as isize) }) as isize);
            i = i.wrapping_add(1);
        }
        i = 0;
        p = mem;
        while i < unsafe { *shape.wrapping_offset(0 as isize) } {
            let xdptr: *mut i8 = ADJUST_PTR__2!(dptr, dsuboffsets, 0i32);
            unsafe {
                core::ptr::copy_nonoverlapping(
                    ((p as *const ()) as *const core::ffi::c_void) as *const u8,
                    ((xdptr as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                    ((itemsize as u64) as usize) as usize,
                )
            };
            let __v10: *mut i8 = p;
            p = __v10.wrapping_offset(itemsize as isize);
            let __v12: *mut i8 = dptr;
            dptr =
                __v12.wrapping_offset((unsafe { *dstrides.wrapping_offset(0 as isize) }) as isize);
            i = i.wrapping_add(1);
        }
    }
}

/// Recursively copy a source buffer to a destination buffer. The two buffers
/// have the same ndim, shape and itemsize.
fn copy_rec(
    shape: *const i64,
    ndim: i64,
    itemsize: i64,
    mut dptr: *mut i8,
    dstrides: *const i64,
    mut dsuboffsets: *const i64,
    mut sptr: *mut i8,
    sstrides: *const i64,
    mut ssuboffsets: *const i64,
    mem: *mut i8,
) {
    let mut i: i64 = 0;
    if ndim == 1 {
        copy_base(
            shape,
            itemsize,
            dptr,
            dstrides,
            dsuboffsets,
            sptr,
            sstrides,
            ssuboffsets,
            mem,
        );
        return;
    }
    while i < unsafe { *shape.wrapping_offset(0 as isize) } {
        let xdptr: *mut i8 = ADJUST_PTR__2!(dptr, dsuboffsets, 0i32);
        let xsptr: *mut i8 = ADJUST_PTR__2!(sptr, ssuboffsets, 0i32);
        copy_rec(
            shape.wrapping_offset(1 as isize),
            ndim.wrapping_sub(1),
            itemsize,
            xdptr,
            dstrides.wrapping_offset(1 as isize),
            if !dsuboffsets.is_null() {
                dsuboffsets.wrapping_offset(1 as isize)
            } else {
                core::ptr::null::<i64>()
            },
            xsptr,
            sstrides.wrapping_offset(1 as isize),
            if !ssuboffsets.is_null() {
                ssuboffsets.wrapping_offset(1 as isize)
            } else {
                core::ptr::null::<i64>()
            },
            mem,
        );
        let __v1: *mut i8 = dptr;
        dptr = __v1.wrapping_offset((unsafe { *dstrides.wrapping_offset(0 as isize) }) as isize);
        let __v3: *mut i8 = sptr;
        sptr = __v3.wrapping_offset((unsafe { *sstrides.wrapping_offset(0 as isize) }) as isize);
        i = i.wrapping_add(1);
    }
}

/// Faster copying of one-dimensional arrays.
fn copy_single(
    self_: *mut __SlateRecord_memoryobject_h_36,
    dest: *const __SlateRecord_pybuffer_h_33,
    src: *const __SlateRecord_pybuffer_h_33,
) -> i32 {
    CHECK_RELEASED_INT_AGAIN!(self_);
    let mut mem: *mut i8 = core::ptr::null_mut::<i8>();
    if equiv_structure(dest, src) == 0 {
        return 1i32.wrapping_neg();
    }
    if last_dim_is_contiguous(dest, src) == 0 {
        mem = (unsafe {
            PyMem_Malloc(
                unsafe { *unsafe { (*dest).shape }.wrapping_offset(0 as isize) }
                    .wrapping_mul((*dest).itemsize) as u64,
            )
        }) as *mut i8;
        if mem.is_null() {
            unsafe { PyErr_NoMemory() };
            return 1i32.wrapping_neg();
        }
    }
    copy_base(
        (unsafe { (*dest).shape }) as *const i64,
        unsafe { (*dest).itemsize },
        (unsafe { (*dest).buf }) as *mut i8,
        (unsafe { (*dest).strides }) as *const i64,
        (unsafe { (*dest).suboffsets }) as *const i64,
        (unsafe { (*src).buf }) as *mut i8,
        (unsafe { (*src).strides }) as *const i64,
        (unsafe { (*src).suboffsets }) as *const i64,
        mem,
    );
    if !mem.is_null() {
        unsafe { PyMem_Free(mem as *mut ()) };
    }
    0
}

/// Recursively copy src to dest. Both buffers must have the same basic
/// structure. Copying is atomic, the function never fails with a partial
/// copy.
fn copy_buffer(
    dest: *const __SlateRecord_pybuffer_h_33,
    src: *const __SlateRecord_pybuffer_h_33,
) -> i32 {
    let mut mem: *mut i8 = core::ptr::null_mut::<i8>();
    if equiv_structure(dest, src) == 0 {
        return 1i32.wrapping_neg();
    }
    if last_dim_is_contiguous(dest, src) == 0 {
        mem = (unsafe {
            PyMem_Malloc(
                unsafe {
                    *unsafe { (*dest).shape }
                        .wrapping_offset(unsafe { (*dest).ndim }.wrapping_sub(1) as isize)
                }
                .wrapping_mul((*dest).itemsize) as u64,
            )
        }) as *mut i8;
        if mem.is_null() {
            unsafe { PyErr_NoMemory() };
            return 1i32.wrapping_neg();
        }
    }
    copy_rec(
        (unsafe { (*dest).shape }) as *const i64,
        (unsafe { (*dest).ndim }) as i64,
        unsafe { (*dest).itemsize },
        (unsafe { (*dest).buf }) as *mut i8,
        (unsafe { (*dest).strides }) as *const i64,
        (unsafe { (*dest).suboffsets }) as *const i64,
        (unsafe { (*src).buf }) as *mut i8,
        (unsafe { (*src).strides }) as *const i64,
        (unsafe { (*src).suboffsets }) as *const i64,
        mem,
    );
    if !mem.is_null() {
        unsafe { PyMem_Free(mem as *mut ()) };
    }
    0
}

/// Initialize strides for a C-contiguous array.
fn init_strides_from_shape(view: *mut __SlateRecord_pybuffer_h_33) {
    let mut i: i64 = 0;
    unsafe {
        *unsafe { (*view).strides }
            .wrapping_offset(unsafe { (*view).ndim }.wrapping_sub(1) as isize) = (*view).itemsize;
    }
    i = unsafe { (*view).ndim }.wrapping_sub(2) as i64;
    while i >= 0 {
        unsafe {
            *unsafe { (*view).strides }.wrapping_offset(i as isize) =
                unsafe { *unsafe { (*view).strides }.wrapping_offset(i.wrapping_add(1) as isize) }
                    .wrapping_mul(
                        *unsafe { (*view).shape }.wrapping_offset(i.wrapping_add(1) as isize),
                    );
        }
        i = i.wrapping_sub(1);
    }
}

/// Initialize strides for a Fortran-contiguous array.
fn init_fortran_strides_from_shape(view: *mut __SlateRecord_pybuffer_h_33) {
    let mut i: i64 = 1;
    unsafe {
        *unsafe { (*view).strides }.wrapping_offset(0 as isize) = (*view).itemsize;
    }
    while i < ((unsafe { (*view).ndim }) as i64) {
        unsafe {
            *unsafe { (*view).strides }.wrapping_offset(i as isize) =
                unsafe { *unsafe { (*view).strides }.wrapping_offset(i.wrapping_sub(1) as isize) }
                    .wrapping_mul(
                        *unsafe { (*view).shape }.wrapping_offset(i.wrapping_sub(1) as isize),
                    );
        }
        i = i.wrapping_add(1);
    }
}

/// Copy src to a contiguous representation. order is one of 'C', 'F' (Fortran)
/// or 'A' (Any). Assumptions: src has PyBUF_FULL information, src->ndim >= 1,
/// len(mem) == src->len.
fn buffer_to_contiguous(mem: *mut i8, src: *const __SlateRecord_pybuffer_h_33, order: i8) -> i32 {
    let mut dest: __SlateRecord_pybuffer_h_33 = unsafe { core::mem::zeroed() };
    let mut ret: i32 = 0;
    let strides: *mut i64 = (unsafe {
        PyMem_Malloc(((*src).ndim as u64).wrapping_mul(core::mem::size_of::<i64>() as u64))
    }) as *mut i64;
    if strides.is_null() {
        unsafe { PyErr_NoMemory() };
        return 1i32.wrapping_neg();
    }
    // initialize dest
    dest = unsafe { *src };
    dest.buf = mem as *mut ();
    // shape is constant and shared: the logical representation of the
    // array is unaltered.
    // The physical representation determined by strides (and possibly
    // suboffsets) may change.
    dest.strides = strides;
    if (order as i32) == 67 || (order as i32) == 65 {
        init_strides_from_shape(core::ptr::addr_of_mut!(dest));
    } else {
        init_fortran_strides_from_shape(core::ptr::addr_of_mut!(dest));
    }
    dest.suboffsets = core::ptr::null_mut::<i64>();
    ret = copy_buffer(
        core::ptr::addr_of_mut!(dest) as *const __SlateRecord_pybuffer_h_33,
        src,
    );
    unsafe { PyMem_Free(strides as *mut ()) };
    ret
}

//                               Constructors
/// Initialize values that are shared with the managed buffer.
fn init_shared_values(
    dest: *mut __SlateRecord_pybuffer_h_33,
    src: *const __SlateRecord_pybuffer_h_33,
) {
    unsafe {
        (*dest).obj = (*src).obj;
    }
    unsafe {
        (*dest).buf = (*src).buf;
    }
    unsafe {
        (*dest).len = (*src).len;
    }
    unsafe {
        (*dest).itemsize = (*src).itemsize;
    }
    unsafe {
        (*dest).readonly = (*src).readonly;
    }
    unsafe {
        (*dest).format = if !unsafe { (*src).format }.is_null() {
            (*src).format
        } else {
            c"B".as_ptr() as *mut i8
        };
    }
    unsafe {
        (*dest).internal = (*src).internal;
    }
}

/// Copy shape and strides. Reconstruct missing values.
fn init_shape_strides(
    dest: *mut __SlateRecord_pybuffer_h_33,
    src: *const __SlateRecord_pybuffer_h_33,
) {
    let mut i: i64 = 0;
    if (unsafe { (*src).ndim }) == 0 {
        unsafe {
            (*dest).shape = core::ptr::null_mut::<i64>();
        }
        unsafe {
            (*dest).strides = core::ptr::null_mut::<i64>();
        }
        return;
    }
    if (unsafe { (*src).ndim }) == 1 {
        unsafe {
            *unsafe { (*dest).shape }.wrapping_offset(0 as isize) =
                if !unsafe { (*src).shape }.is_null() {
                    *unsafe { (*src).shape }.wrapping_offset(0 as isize)
                } else {
                    (*src).len / (*src).itemsize
                };
        }
        unsafe {
            *unsafe { (*dest).strides }.wrapping_offset(0 as isize) =
                if !unsafe { (*src).strides }.is_null() {
                    *unsafe { (*src).strides }.wrapping_offset(0 as isize)
                } else {
                    (*src).itemsize
                };
        }
        return;
    }
    while i < ((unsafe { (*src).ndim }) as i64) {
        unsafe {
            *unsafe { (*dest).shape }.wrapping_offset(i as isize) =
                *unsafe { (*src).shape }.wrapping_offset(i as isize);
        }
        i = i.wrapping_add(1);
    }
    if !unsafe { (*src).strides }.is_null() {
        i = 0;
        while i < ((unsafe { (*src).ndim }) as i64) {
            unsafe {
                *unsafe { (*dest).strides }.wrapping_offset(i as isize) =
                    *unsafe { (*src).strides }.wrapping_offset(i as isize);
            }
            i = i.wrapping_add(1);
        }
    } else {
        init_strides_from_shape(dest);
    }
}

fn init_suboffsets(
    dest: *mut __SlateRecord_pybuffer_h_33,
    src: *const __SlateRecord_pybuffer_h_33,
) {
    let mut i: i64 = 0;
    if unsafe { (*src).suboffsets }.is_null() {
        unsafe {
            (*dest).suboffsets = core::ptr::null_mut::<i64>();
        }
        return;
    }
    while i < ((unsafe { (*src).ndim }) as i64) {
        unsafe {
            *unsafe { (*dest).suboffsets }.wrapping_offset(i as isize) =
                *unsafe { (*src).suboffsets }.wrapping_offset(i as isize);
        }
        i = i.wrapping_add(1);
    }
}

/// len = product(shape) * itemsize
fn init_len(view: *mut __SlateRecord_pybuffer_h_33) {
    let mut i: i64 = 0;
    let mut len: i64 = 1;
    while i < ((unsafe { (*view).ndim }) as i64) {
        len = len.wrapping_mul(unsafe { *unsafe { (*view).shape }.wrapping_offset(i as isize) });
        i = i.wrapping_add(1);
    }
    len = len.wrapping_mul(unsafe { (*view).itemsize });
    unsafe {
        (*view).len = len;
    }
}

/// Initialize memoryview buffer properties.
fn init_flags(mv: *mut __SlateRecord_memoryobject_h_36) {
    let view: *const __SlateRecord_pybuffer_h_33 =
        (unsafe { core::ptr::addr_of_mut!((*mv).view) }) as *const __SlateRecord_pybuffer_h_33;
    let mut flags: i32 = 0;
    match unsafe { (*view).ndim } {
        0 => {
            flags |= 8 | 2 | 4;
        }
        1 => {
            if MV_CONTIGUOUS_NDIM1!(view) {
                flags |= 2 | 4;
            }
        }
        _ => {
            if (unsafe { PyBuffer_IsContiguous(view, 67i8) }) != 0 {
                flags |= 2;
            }
            if (unsafe { PyBuffer_IsContiguous(view, 70i8) }) != 0 {
                flags |= 4;
            }
        }
    }
    if !unsafe { (*view).suboffsets }.is_null() {
        flags |= 16;
        let __v12: i32 = flags & !(2 | 4);
        flags = __v12;
    }
    unsafe {
        (*mv).flags = flags;
    }
}

/// Allocate a new memoryview and perform basic initialization. New memoryviews
/// are exclusively created through the mbuf_add functions.
fn memory_alloc(ndim: i32) -> *mut __SlateRecord_memoryobject_h_36 {
    let mv: *mut __SlateRecord_memoryobject_h_36 = PyObject_GC_NewVar__2!(
        __SlateRecord_memoryobject_h_36,
        unsafe { core::ptr::addr_of_mut!(PyMemoryView_Type) },
        3i32.wrapping_mul(ndim)
    );
    if mv.is_null() {
        return core::ptr::null_mut::<__SlateRecord_memoryobject_h_36>();
    }
    unsafe {
        (*mv).mbuf = core::ptr::null_mut::<__SlateRecord_memoryobject_h_16>();
    }
    unsafe {
        (*mv).hash = 1i32.wrapping_neg() as i64;
    }
    unsafe {
        (*mv).flags = 0;
    }
    unsafe {
        (*mv).exports = 0;
    }
    unsafe {
        (*mv).view.ndim = ndim;
    }
    unsafe {
        (*mv).view.shape = (*mv).ob_array.as_mut_ptr() as *mut i64;
    }
    unsafe {
        (*mv).view.strides =
            ((*mv).ob_array.as_mut_ptr() as *mut i64).wrapping_offset(ndim as isize);
    }
    unsafe {
        (*mv).view.suboffsets = ((*mv).ob_array.as_mut_ptr() as *mut i64)
            .wrapping_offset(2i32.wrapping_mul(ndim) as isize);
    }
    unsafe {
        (*mv).weakreflist = core::ptr::null_mut::<_object>();
    }
    _PyObject_GC_TRACK!(mv);
    mv
}

/// Return a new memoryview that is registered with mbuf. If src is NULL,
/// use mbuf->master as the underlying buffer. Otherwise, use src.
///
/// The new memoryview has full buffer information: shape and strides
/// are always present, suboffsets as needed. Arrays are copied to
/// the memoryview's ob_array field.
fn mbuf_add_view(
    mbuf: *mut __SlateRecord_memoryobject_h_16,
    mut src: *const __SlateRecord_pybuffer_h_33,
) -> *mut _object {
    let mut mv: *mut __SlateRecord_memoryobject_h_36 = unsafe { core::mem::zeroed() };
    if src.is_null() {
        src = (unsafe { core::ptr::addr_of_mut!((*mbuf).master) })
            as *const __SlateRecord_pybuffer_h_33;
    }
    if (unsafe { (*src).ndim }) > 64 {
        unsafe {
            PyErr_SetString(
                PyExc_ValueError,
                c"memoryview: number of dimensions must not exceed 64".as_ptr(),
            )
        };
        return core::ptr::null_mut::<_object>();
    }
    mv = memory_alloc(unsafe { (*src).ndim });
    if mv.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let dest: *mut __SlateRecord_pybuffer_h_33 = unsafe { core::ptr::addr_of_mut!((*mv).view) };
    init_shared_values(dest, src);
    init_shape_strides(dest, src);
    init_suboffsets(dest, src);
    init_flags(mv);
    unsafe {
        (*mv).mbuf = Py_NewRef__2!(mbuf) as *mut __SlateRecord_memoryobject_h_16;
    }
    let __v1: i64 = unsafe { (*mbuf).exports };
    unsafe {
        (*mbuf).exports = __v1.wrapping_add(1);
    }
    mv as *mut _object
}

/// Register an incomplete view: shape, strides, suboffsets and flags still
/// need to be initialized. Use 'ndim' instead of src->ndim to determine the
/// size of the memoryview's ob_array.
///
/// Assumption: ndim <= PyBUF_MAX_NDIM.
fn mbuf_add_incomplete_view(
    mbuf: *mut __SlateRecord_memoryobject_h_16,
    mut src: *const __SlateRecord_pybuffer_h_33,
    ndim: i32,
) -> *mut _object {
    let mut mv: *mut __SlateRecord_memoryobject_h_36 = unsafe { core::mem::zeroed() };
    if src.is_null() {
        src = (unsafe { core::ptr::addr_of_mut!((*mbuf).master) })
            as *const __SlateRecord_pybuffer_h_33;
    }
    mv = memory_alloc(ndim);
    if mv.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let dest: *mut __SlateRecord_pybuffer_h_33 = unsafe { core::ptr::addr_of_mut!((*mv).view) };
    init_shared_values(dest, src);
    unsafe {
        (*mv).mbuf = Py_NewRef__2!(mbuf) as *mut __SlateRecord_memoryobject_h_16;
    }
    let __v1: i64 = unsafe { (*mbuf).exports };
    unsafe {
        (*mbuf).exports = __v1.wrapping_add(1);
    }
    mv as *mut _object
}

/// Expose a raw memory area as a view of contiguous bytes. flags can be
/// PyBUF_READ or PyBUF_WRITE. view->format is set to "B" (unsigned bytes).
/// The memoryview has complete buffer information.
#[unsafe(no_mangle)]
extern "C-unwind" fn PyMemoryView_FromMemory(mem: *mut i8, size: i64, flags: i32) -> *mut _object {
    let mut mbuf: *mut __SlateRecord_memoryobject_h_16 = unsafe { core::mem::zeroed() };
    let mut readonly: i32 = 0;
    mbuf = mbuf_alloc();
    if mbuf.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    readonly = if flags == 512 { 0 } else { 1 };
    unsafe {
        PyBuffer_FillInfo(
            core::ptr::addr_of_mut!((*mbuf).master),
            core::ptr::null_mut::<_object>(),
            mem as *mut (),
            size,
            readonly,
            284i32,
        )
    };
    let mv: *mut _object = mbuf_add_view(mbuf, core::ptr::null::<__SlateRecord_pybuffer_h_33>());
    Py_DECREF__2!(mbuf);
    mv
}

/// Create a memoryview from a given Py_buffer. For simple byte views,
/// PyMemoryView_FromMemory() should be used instead.
/// This function is the only entry point that can create a master buffer
/// without full information. Because of this fact init_shape_strides()
/// must be able to reconstruct missing values.
#[unsafe(no_mangle)]
extern "C-unwind" fn PyMemoryView_FromBuffer(
    info: *const __SlateRecord_pybuffer_h_33,
) -> *mut _object {
    if unsafe { (*info).buf }.is_null() {
        unsafe {
            PyErr_SetString(
                PyExc_ValueError,
                c"PyMemoryView_FromBuffer(): info->buf must not be NULL".as_ptr(),
            )
        };
        return core::ptr::null_mut::<_object>();
    }
    let mbuf: *mut __SlateRecord_memoryobject_h_16 = mbuf_alloc();
    if mbuf.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    // info->obj is either NULL or a borrowed reference. This reference
    // should not be decremented in PyBuffer_Release().
    unsafe {
        (*mbuf).master = *info;
    }
    unsafe {
        (*mbuf).master.obj = core::ptr::null_mut::<_object>();
    }
    let mv: *mut _object = mbuf_add_view(mbuf, core::ptr::null::<__SlateRecord_pybuffer_h_33>());
    Py_DECREF__2!(mbuf);
    mv
}

/// Create a memoryview from an object that implements the buffer protocol,
/// using the given flags.
/// If the object is a memoryview, the new memoryview must be registered
/// with the same managed buffer. Otherwise, a new managed buffer is created.
fn PyMemoryView_FromObjectAndFlags(v: *mut _object, flags: i32) -> *mut _object {
    if PyMemoryView_Check!(v) != 0 {
        let mv: *mut __SlateRecord_memoryobject_h_36 = v as *mut __SlateRecord_memoryobject_h_36;
        CHECK_RELEASED!(mv);
        CHECK_RESTRICTED!(mv);
        return mbuf_add_view(
            unsafe { (*mv).mbuf },
            (unsafe { core::ptr::addr_of_mut!((*mv).view) }) as *const __SlateRecord_pybuffer_h_33,
        );
    } else {
        if (unsafe { PyObject_CheckBuffer(v) }) != 0 {
            let mbuf: *mut __SlateRecord_memoryobject_h_16 =
                _PyManagedBuffer_FromObject(v, flags) as *mut __SlateRecord_memoryobject_h_16;
            if mbuf.is_null() {
                return core::ptr::null_mut::<_object>();
            }
            let ret: *mut _object =
                mbuf_add_view(mbuf, core::ptr::null::<__SlateRecord_pybuffer_h_33>());
            Py_DECREF__2!(mbuf);
            return ret;
        }
    }
    unsafe {
        PyErr_Format(
            PyExc_TypeError,
            c"memoryview: a bytes-like object is required, not '%.200s'".as_ptr(),
            (*Py_TYPE!(v)).tp_name,
        )
    };
    core::ptr::null_mut::<_object>()
}

/// Create a memoryview from an object that implements the buffer protocol,
/// using the given flags.
/// If the object is a memoryview, the new memoryview must be registered
/// with the same managed buffer. Otherwise, a new managed buffer is created.
#[unsafe(no_mangle)]
extern "C-unwind" fn _PyMemoryView_FromBufferProc(
    v: *mut _object,
    flags: i32,
    bufferproc: Option<
        unsafe extern "C-unwind" fn(*mut _object, *mut __SlateRecord_pybuffer_h_33, i32) -> i32,
    >,
) -> *mut _object {
    let mbuf: *mut __SlateRecord_memoryobject_h_16 = mbuf_alloc();
    if mbuf.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let res: i32 =
        unsafe { bufferproc.unwrap()(v, core::ptr::addr_of_mut!((*mbuf).master), flags) };
    if res < 0 {
        unsafe {
            (*mbuf).master.obj = core::ptr::null_mut::<_object>();
        }
        Py_DECREF__2!(mbuf);
        return core::ptr::null_mut::<_object>();
    }
    let ret: *mut _object = mbuf_add_view(mbuf, core::ptr::null::<__SlateRecord_pybuffer_h_33>());
    Py_DECREF__2!(mbuf);
    ret
}

/// Create a memoryview from an object that implements the buffer protocol.
/// If the object is a memoryview, the new memoryview must be registered
/// with the same managed buffer. Otherwise, a new managed buffer is created.
#[unsafe(no_mangle)]
extern "C-unwind" fn PyMemoryView_FromObject(v: *mut _object) -> *mut _object {
    PyMemoryView_FromObjectAndFlags(v, 284i32)
}

/// Copy the format string from a base object that might vanish.
fn mbuf_copy_format(mbuf: *mut __SlateRecord_memoryobject_h_16, fmt: *const i8) -> i32 {
    if !fmt.is_null() {
        let cp: *mut i8 = (unsafe {
            PyMem_Malloc((strlen(fmt as *const core::ffi::c_char) as u64).wrapping_add(1))
        }) as *mut i8;
        if cp.is_null() {
            unsafe { PyErr_NoMemory() };
            return 1i32.wrapping_neg();
        }
        unsafe {
            (*mbuf).master.format = strcpy(
                cp as *mut core::ffi::c_char,
                fmt as *const core::ffi::c_char,
            ) as *mut i8;
        }
        unsafe {
            (*mbuf).flags = (*mbuf).flags | 2;
        }
    }
    0
}

/// Return a memoryview that is based on a contiguous copy of src.
/// Assumptions: src has PyBUF_FULL_RO information, src->ndim > 0.
///
/// Ownership rules:
///   1) As usual, the returned memoryview has a private copy
///      of src->shape, src->strides and src->suboffsets.
///   2) src->format is copied to the master buffer and released
///      in mbuf_dealloc(). The releasebufferproc of the bytes
///      object is NULL, so it does not matter that mbuf_release()
///      passes the altered format pointer to PyBuffer_Release().
fn memory_from_contiguous_copy(src: *const __SlateRecord_pybuffer_h_33, order: i8) -> *mut _object {
    let mut mv: *mut __SlateRecord_memoryobject_h_36 = unsafe { core::mem::zeroed() };
    // shape and strides
    let mut i: i32 = 0;
    let bytes: *mut _object =
        unsafe { PyBytes_FromStringAndSize(core::ptr::null::<i8>(), (*src).len) };
    if bytes.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let mbuf: *mut __SlateRecord_memoryobject_h_16 =
        _PyManagedBuffer_FromObject(bytes, 284i32) as *mut __SlateRecord_memoryobject_h_16;
    Py_DECREF!(bytes);
    if mbuf.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    if mbuf_copy_format(mbuf, (unsafe { (*src).format }) as *const i8) < 0 {
        Py_DECREF__2!(mbuf);
        return core::ptr::null_mut::<_object>();
    }
    mv = mbuf_add_incomplete_view(
        mbuf,
        core::ptr::null::<__SlateRecord_pybuffer_h_33>(),
        unsafe { (*src).ndim },
    ) as *mut __SlateRecord_memoryobject_h_36;
    Py_DECREF__2!(mbuf);
    if mv.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let dest: *mut __SlateRecord_pybuffer_h_33 = unsafe { core::ptr::addr_of_mut!((*mv).view) };
    // shared values are initialized correctly except for itemsize
    unsafe {
        (*dest).itemsize = (*src).itemsize;
    }
    while i < unsafe { (*src).ndim } {
        unsafe {
            *unsafe { (*dest).shape }.wrapping_offset(i as isize) =
                *unsafe { (*src).shape }.wrapping_offset(i as isize);
        }
        i = i.wrapping_add(1);
    }
    if (order as i32) == 67 || (order as i32) == 65 {
        init_strides_from_shape(dest);
    } else {
        init_fortran_strides_from_shape(dest);
    }
    // suboffsets
    unsafe {
        (*dest).suboffsets = core::ptr::null_mut::<i64>();
    }
    // flags
    init_flags(mv);
    if copy_buffer(dest as *const __SlateRecord_pybuffer_h_33, src) < 0 {
        Py_DECREF__2!(mv);
        return core::ptr::null_mut::<_object>();
    }
    mv as *mut _object
}

/// Return a new memoryview object based on a contiguous exporter with
/// buffertype={PyBUF_READ, PyBUF_WRITE} and order={'C', 'F'ortran, or 'A'ny}.
/// The logical structure of the input and output buffers is the same
/// (i.e. tolist(input) == tolist(output)), but the physical layout in
/// memory can be explicitly chosen.
///
/// As usual, if buffertype=PyBUF_WRITE, the exporter's buffer must be writable,
/// otherwise it may be writable or read-only.
///
/// If the exporter is already contiguous with the desired target order,
/// the memoryview will be directly based on the exporter.
///
/// Otherwise, if the buffertype is PyBUF_READ, the memoryview will be
/// based on a new bytes object. If order={'C', 'A'ny}, use 'C' order,
/// 'F'ortran order otherwise.
#[unsafe(no_mangle)]
extern "C-unwind" fn PyMemoryView_GetContiguous(
    obj: *mut _object,
    buffertype: i32,
    order: i8,
) -> *mut _object {
    let mut mv: *mut __SlateRecord_memoryobject_h_36 = unsafe { core::mem::zeroed() };
    mv = PyMemoryView_FromObject(obj) as *mut __SlateRecord_memoryobject_h_36;
    if mv.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let view: *mut __SlateRecord_pybuffer_h_33 = unsafe { core::ptr::addr_of_mut!((*mv).view) };
    if buffertype == 512 && (unsafe { (*view).readonly }) != 0 {
        unsafe {
            PyErr_SetString(
                PyExc_BufferError,
                c"underlying buffer is not writable".as_ptr(),
            )
        };
        Py_DECREF__2!(mv);
        return core::ptr::null_mut::<_object>();
    }
    if (unsafe { PyBuffer_IsContiguous(view as *const __SlateRecord_pybuffer_h_33, order) }) != 0 {
        return mv as *mut _object;
    }
    if buffertype == 512 {
        unsafe {
            PyErr_SetString(
                PyExc_BufferError,
                c"writable contiguous buffer requested for a non-contiguous object.".as_ptr(),
            )
        };
        Py_DECREF__2!(mv);
        return core::ptr::null_mut::<_object>();
    }
    let ret: *mut _object =
        memory_from_contiguous_copy(view as *const __SlateRecord_pybuffer_h_33, order);
    Py_DECREF__2!(mv);
    ret
}

// [clinic input]
// @classmethod
// memoryview.__new__
//
//     object: object
//
// Create a new memoryview object which references the given object.
// [clinic start generated code]
/// [clinic end generated code: output=7de78e184ed66db8 input=f04429eb0bdf8c6e]
fn memoryview_impl(r#type: *mut _typeobject, object: *mut _object) -> *mut _object {
    PyMemoryView_FromObject(object)
}

// [clinic input]
// @classmethod
// memoryview._from_flags
//
//     object: object
//     flags: int
//
// Create a new memoryview object which references the given object.
// [clinic start generated code]
/// [clinic end generated code: output=bf71f9906c266ee2 input=f5f82fd0e744356b]
fn memoryview__from_flags_impl(
    r#type: *mut _typeobject,
    object: *mut _object,
    flags: i32,
) -> *mut _object {
    PyMemoryView_FromObjectAndFlags(object, flags)
}

//                         Previously in abstract.c
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_memoryobject_c_1046 {
    view: __SlateRecord_pybuffer_h_33,
    array: [i64; 1],
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyBuffer_ToContiguous(
    buf: *mut (),
    mut src: *const __SlateRecord_pybuffer_h_33,
    len: i64,
    order: i8,
) -> i32 {
    let mut fb: *mut __SlateRecord_memoryobject_c_1046 =
        core::ptr::null_mut::<__SlateRecord_memoryobject_c_1046>();
    let mut ret: i32 = 0;
    if len != unsafe { (*src).len } {
        unsafe {
            PyErr_SetString(
                PyExc_ValueError,
                c"PyBuffer_ToContiguous: len != view->len".as_ptr(),
            )
        };
        return 1i32.wrapping_neg();
    }
    if (unsafe { PyBuffer_IsContiguous(src, order) }) != 0 {
        unsafe {
            core::ptr::copy_nonoverlapping(
                (((*src).buf as *const ()) as *const core::ffi::c_void) as *const u8,
                ((buf as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                ((len as u64) as usize) as usize,
            )
        };
        return 0;
    }
    // buffer_to_contiguous() assumes PyBUF_FULL
    fb = (unsafe {
        PyMem_Malloc(
            (core::mem::size_of::<__SlateRecord_memoryobject_c_1046>() as u64).wrapping_add(
                (3i32.wrapping_mul((*src).ndim) as u64)
                    .wrapping_mul(core::mem::size_of::<i64>() as u64),
            ),
        )
    }) as *mut __SlateRecord_memoryobject_c_1046;
    if fb.is_null() {
        unsafe { PyErr_NoMemory() };
        return 1i32.wrapping_neg();
    }
    unsafe {
        (*fb).view.ndim = (*src).ndim;
    }
    unsafe {
        (*fb).view.shape = (*fb).array.as_mut_ptr() as *mut i64;
    }
    unsafe {
        (*fb).view.strides =
            ((*fb).array.as_mut_ptr() as *mut i64).wrapping_offset((*src).ndim as isize);
    }
    unsafe {
        (*fb).view.suboffsets = ((*fb).array.as_mut_ptr() as *mut i64)
            .wrapping_offset(2i32.wrapping_mul((*src).ndim) as isize);
    }
    init_shared_values(unsafe { core::ptr::addr_of_mut!((*fb).view) }, src);
    init_shape_strides(unsafe { core::ptr::addr_of_mut!((*fb).view) }, src);
    init_suboffsets(unsafe { core::ptr::addr_of_mut!((*fb).view) }, src);
    src = (unsafe { core::ptr::addr_of_mut!((*fb).view) }) as *const __SlateRecord_pybuffer_h_33;
    ret = buffer_to_contiguous(buf as *mut i8, src, order);
    unsafe { PyMem_Free(fb as *mut ()) };
    ret
}

fn get_exports(buf: *mut __SlateRecord_memoryobject_h_36) -> i64 {
    unsafe { (*buf).exports }
}

//                           Release/GC management
/// Inform the managed buffer that this particular memoryview will not access
/// the underlying buffer again. If no other memoryviews are registered with
/// the managed buffer, the underlying buffer is released instantly and
/// marked as inaccessible for both the memoryview and the managed buffer.
fn _memory_release(self_: *mut __SlateRecord_memoryobject_h_36) {
    if (unsafe { (*self_).flags }) & 1 != 0 {
        return;
    }
    unsafe {
        (*self_).flags = (*self_).flags | 1;
    }
    let __v3: *mut __SlateRecord_memoryobject_h_16 = unsafe { (*self_).mbuf };
    let __v4: i64 = unsafe { (*__v3).exports };
    let __v5: i64 = __v4.wrapping_sub(1);
    unsafe {
        (*__v3).exports = __v5;
    }
    if __v5 == 0 {
        mbuf_release(unsafe { (*self_).mbuf });
    }
}

// [clinic input]
// memoryview.release
//
// Release the underlying buffer exposed by the memoryview object.
// [clinic start generated code]
/// [clinic end generated code: output=d0b7e3ba95b7fcb9 input=bc71d1d51f4a52f0]
fn memoryview_release_impl(self_: *mut __SlateRecord_memoryobject_h_36) -> *mut _object {
    let exports: i64 = get_exports(self_);
    if exports == 0 {
        _memory_release(self_);
        return unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) };
    }
    if exports > 0 {
        unsafe {
            PyErr_Format(
                PyExc_BufferError,
                c"memoryview has %zd exported buffer%s".as_ptr(),
                exports,
                if exports == 1 {
                    c"".as_ptr() as *mut i8
                } else {
                    c"s".as_ptr() as *mut i8
                },
            )
        };
        return core::ptr::null_mut::<_object>();
    }
    unsafe {
        PyErr_SetString(
            PyExc_SystemError,
            c"memoryview: negative export count".as_ptr(),
        )
    };
    core::ptr::null_mut::<_object>()
}

#[unsafe(link_section = ".text.slate_distinct.memoryobject.memory_dealloc")]
extern "C-unwind" fn memory_dealloc(_self: *mut _object) {
    let self_: *mut __SlateRecord_memoryobject_h_36 = _self as *mut __SlateRecord_memoryobject_h_36;
    _PyObject_GC_UNTRACK__2!(self_);
    _memory_release(self_);
    Py_CLEAR__14!((*self_).mbuf);
    if !unsafe { (*self_).weakreflist }.is_null() {
        unsafe { PyObject_ClearWeakRefs(self_ as *mut _object) };
    }
    unsafe { PyObject_GC_Del(self_ as *mut ()) };
}

#[unsafe(link_section = ".text.slate_distinct.memoryobject.memory_traverse")]
extern "C-unwind" fn memory_traverse(
    _self: *mut _object,
    mut visit: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
    arg: *mut (),
) -> i32 {
    let self_: *mut __SlateRecord_memoryobject_h_36 = _self as *mut __SlateRecord_memoryobject_h_36;
    Py_VISIT__17!(unsafe { (*self_).mbuf }, visit, arg);
    0
}

#[unsafe(link_section = ".text.slate_distinct.memoryobject.memory_clear")]
extern "C-unwind" fn memory_clear(_self: *mut _object) -> i32 {
    let self_: *mut __SlateRecord_memoryobject_h_36 = _self as *mut __SlateRecord_memoryobject_h_36;
    if get_exports(self_) == 0 {
        _memory_release(self_);
        Py_CLEAR__14!((*self_).mbuf);
    }
    0
}

#[unsafe(link_section = ".text.slate_distinct.memoryobject.memory_enter")]
extern "C-unwind" fn memory_enter(self_: *mut _object, args: *mut _object) -> *mut _object {
    CHECK_RELEASED__2!(self_);
    Py_NewRef!(self_)
}

#[unsafe(link_section = ".text.slate_distinct.memoryobject.memory_exit")]
extern "C-unwind" fn memory_exit(self_: *mut _object, args: *mut _object) -> *mut _object {
    memoryview_release_impl(self_ as *mut __SlateRecord_memoryobject_h_36)
}

//                         Casting format and shape
fn get_native_fmtchar(result: &mut i8, mut fmt: *const i8) -> i64 {
    let mut size: i64 = 1i32.wrapping_neg() as i64;
    if ((unsafe { *fmt.wrapping_offset(0 as isize) }) as i32) == 64 {
        let __v0: *const i8 = fmt;
        fmt = __v0.wrapping_offset(1 as isize);
    }
    match (unsafe { *fmt.wrapping_offset(0 as isize) }) as i32 {
        66 | 98 | 99 => {
            size = core::mem::size_of::<i8>() as i64;
        }
        72 | 104 => {
            size = core::mem::size_of::<i16>() as i64;
        }
        73 | 105 => {
            size = core::mem::size_of::<i32>() as i64;
        }
        76 | 108 => {
            size = core::mem::size_of::<i64>() as i64;
        }
        81 | 113 => {
            size = core::mem::size_of::<i64>() as i64;
        }
        78 | 110 => {
            size = core::mem::size_of::<i64>() as i64;
        }
        102 => {
            size = core::mem::size_of::<f32>() as i64;
        }
        100 => {
            size = core::mem::size_of::<f64>() as i64;
        }
        101 => {
            size = ((core::mem::size_of::<f32>() as u64) / 2) as i64;
        }
        63 => {
            size = core::mem::size_of::<bool>() as i64;
        }
        80 => {
            size = core::mem::size_of::<*mut ()>() as i64;
        }
        _ => {}
    }
    if size > 0 && ((unsafe { *fmt.wrapping_offset(1 as isize) }) as i32) == 0 {
        unsafe {
            *(result as *mut i8) = *fmt.wrapping_offset(0 as isize);
        }
        return size;
    }
    1i32.wrapping_neg() as i64
}

fn get_native_fmtstr(mut fmt: *const i8) -> *const i8 {
    let mut at: i32 = 0;
    if ((unsafe { *fmt.wrapping_offset(0 as isize) }) as i32) == 64 {
        at = 1;
        let __v0: *const i8 = fmt;
        fmt = __v0.wrapping_offset(1 as isize);
    }
    if ((unsafe { *fmt.wrapping_offset(0 as isize) }) as i32) == 0
        || ((unsafe { *fmt.wrapping_offset(1 as isize) }) as i32) != 0
    {
        return core::ptr::null::<i8>();
    }
    let mut __t0: i64 = match (unsafe { *fmt.wrapping_offset(0 as isize) }) as i32 {
        99 => 0,
        98 => 1,
        66 => 2,
        104 => 3,
        72 => 4,
        105 => 5,
        73 => 6,
        108 => 7,
        76 => 8,
        113 => 9,
        81 => 10,
        110 => 11,
        78 => 12,
        102 => 13,
        100 => 14,
        101 => 15,
        63 => 16,
        80 => 17,
        _ => -1,
    };
    loop {
        match __t0 {
            0 => {
                loop {
                    return (if at != 0 {
                        c"@c".as_ptr() as *mut i8
                    } else {
                        c"c".as_ptr() as *mut i8
                    }) as *const i8;
                }
                __t0 = 1;
                continue;
            }
            1 => {
                loop {
                    return (if at != 0 {
                        c"@b".as_ptr() as *mut i8
                    } else {
                        c"b".as_ptr() as *mut i8
                    }) as *const i8;
                }
                __t0 = 2;
                continue;
            }
            2 => {
                loop {
                    return (if at != 0 {
                        c"@B".as_ptr() as *mut i8
                    } else {
                        c"B".as_ptr() as *mut i8
                    }) as *const i8;
                }
                __t0 = 3;
                continue;
            }
            3 => {
                loop {
                    return (if at != 0 {
                        c"@h".as_ptr() as *mut i8
                    } else {
                        c"h".as_ptr() as *mut i8
                    }) as *const i8;
                }
                __t0 = 4;
                continue;
            }
            4 => {
                loop {
                    return (if at != 0 {
                        c"@H".as_ptr() as *mut i8
                    } else {
                        c"H".as_ptr() as *mut i8
                    }) as *const i8;
                }
                __t0 = 5;
                continue;
            }
            5 => {
                loop {
                    return (if at != 0 {
                        c"@i".as_ptr() as *mut i8
                    } else {
                        c"i".as_ptr() as *mut i8
                    }) as *const i8;
                }
                __t0 = 6;
                continue;
            }
            6 => {
                loop {
                    return (if at != 0 {
                        c"@I".as_ptr() as *mut i8
                    } else {
                        c"I".as_ptr() as *mut i8
                    }) as *const i8;
                }
                __t0 = 7;
                continue;
            }
            7 => {
                loop {
                    return (if at != 0 {
                        c"@l".as_ptr() as *mut i8
                    } else {
                        c"l".as_ptr() as *mut i8
                    }) as *const i8;
                }
                __t0 = 8;
                continue;
            }
            8 => {
                loop {
                    return (if at != 0 {
                        c"@L".as_ptr() as *mut i8
                    } else {
                        c"L".as_ptr() as *mut i8
                    }) as *const i8;
                }
                __t0 = 9;
                continue;
            }
            9 => {
                loop {
                    return (if at != 0 {
                        c"@q".as_ptr() as *mut i8
                    } else {
                        c"q".as_ptr() as *mut i8
                    }) as *const i8;
                }
                __t0 = 10;
                continue;
            }
            10 => {
                loop {
                    return (if at != 0 {
                        c"@Q".as_ptr() as *mut i8
                    } else {
                        c"Q".as_ptr() as *mut i8
                    }) as *const i8;
                }
                __t0 = 11;
                continue;
            }
            11 => {
                loop {
                    return (if at != 0 {
                        c"@n".as_ptr() as *mut i8
                    } else {
                        c"n".as_ptr() as *mut i8
                    }) as *const i8;
                }
                __t0 = 12;
                continue;
            }
            12 => {
                loop {
                    return (if at != 0 {
                        c"@N".as_ptr() as *mut i8
                    } else {
                        c"N".as_ptr() as *mut i8
                    }) as *const i8;
                }
                __t0 = 13;
                continue;
            }
            13 => {
                loop {
                    return (if at != 0 {
                        c"@f".as_ptr() as *mut i8
                    } else {
                        c"f".as_ptr() as *mut i8
                    }) as *const i8;
                }
                __t0 = 14;
                continue;
            }
            14 => {
                loop {
                    return (if at != 0 {
                        c"@d".as_ptr() as *mut i8
                    } else {
                        c"d".as_ptr() as *mut i8
                    }) as *const i8;
                }
                __t0 = 15;
                continue;
            }
            15 => {
                loop {
                    return (if at != 0 {
                        c"@e".as_ptr() as *mut i8
                    } else {
                        c"e".as_ptr() as *mut i8
                    }) as *const i8;
                }
                __t0 = 16;
                continue;
            }
            16 => {
                loop {
                    return (if at != 0 {
                        c"@?".as_ptr() as *mut i8
                    } else {
                        c"?".as_ptr() as *mut i8
                    }) as *const i8;
                }
                __t0 = 17;
                continue;
            }
            17 => {
                loop {
                    return (if at != 0 {
                        c"@P".as_ptr() as *mut i8
                    } else {
                        c"P".as_ptr() as *mut i8
                    }) as *const i8;
                }
                break;
            }
            _ => {
                break;
            }
        }
    }
    core::ptr::null::<i8>()
}

/// Cast a memoryview's data type to 'format'. The input array must be
/// C-contiguous. At least one of input-format, output-format must have
/// byte size. The output array is 1-D, with the same byte length as the
/// input array. Thus, view->len must be a multiple of the new itemsize.
fn cast_to_1D(mv: *mut __SlateRecord_memoryobject_h_36, format: *mut _object) -> i32 {
    let mut ret: i32 = unsafe { core::mem::zeroed() };
    let mut itemsize: i64 = unsafe { core::mem::zeroed() };
    let mut destchar: i8 = unsafe { core::mem::zeroed() };
    let mut srcchar: i8 = unsafe { core::mem::zeroed() };
    let mut asciifmt: *mut _object = unsafe { core::mem::zeroed() };
    let mut view: *mut __SlateRecord_pybuffer_h_33 = unsafe { core::mem::zeroed() };
    unsafe {
        view = core::ptr::addr_of_mut!((*mv).view);
        ret = 1i32.wrapping_neg();
        asciifmt = PyUnicode_AsASCIIString(format);
        if asciifmt.is_null() {
            return ret;
        } else {
            itemsize = get_native_fmtchar(
                &mut (*core::ptr::addr_of_mut!(destchar)),
                PyBytes_AS_STRING!(asciifmt) as *const i8,
            );
            if itemsize < 0 {
                PyErr_SetString(PyExc_ValueError, c"memoryview: destination format must be a native single character format prefixed with an optional '@'".as_ptr());
            } else {
                if (get_native_fmtchar(
                    &mut (*core::ptr::addr_of_mut!(srcchar)),
                    (*view).format as *const i8,
                ) < 0
                    || !IS_BYTE_FORMAT!(srcchar))
                    && !IS_BYTE_FORMAT!(destchar)
                {
                    PyErr_SetString(
                        PyExc_TypeError,
                        c"memoryview: cannot cast between two non-byte formats".as_ptr(),
                    );
                } else {
                    if (*view).len % itemsize != 0 {
                        PyErr_SetString(
                            PyExc_TypeError,
                            c"memoryview: length is not a multiple of itemsize".as_ptr(),
                        );
                    } else {
                        (*view).format =
                            get_native_fmtstr(PyBytes_AS_STRING!(asciifmt) as *const i8) as *mut i8;
                        if unsafe { (*view).format }.is_null() {
                            // NOT_REACHED: get_native_fmtchar() already validates the format.
                            PyErr_SetString(
                                PyExc_RuntimeError,
                                c"memoryview: internal error".as_ptr(),
                            );
                        } else {
                            (*view).itemsize = itemsize;
                            (*view).ndim = 1;
                            *unsafe { (*view).shape }.wrapping_offset(0 as isize) =
                                (*view).len / (*view).itemsize;
                            *unsafe { (*view).strides }.wrapping_offset(0 as isize) =
                                (*view).itemsize;
                            (*view).suboffsets = core::ptr::null_mut::<i64>();
                            init_flags(mv);
                            ret = 0;
                        }
                    }
                }
            }
            Py_DECREF!(asciifmt);
            return ret;
        }
    }
}

/// The memoryview must have space for 3*len(seq) elements.
fn copy_shape(shape: *mut i64, seq: *const _object, ndim: i64, mut len: i64) -> i64 {
    let mut x: i64 = 0;
    for i in 0..ndim {
        let tmp: *mut _object = PySequence_Fast_GET_ITEM__2!(seq, i);
        if PyLong_Check!(tmp) == 0 {
            unsafe {
                PyErr_SetString(
                    PyExc_TypeError,
                    c"memoryview.cast(): elements of shape must be integers".as_ptr(),
                )
            };
            return 1i32.wrapping_neg() as i64;
        }
        x = unsafe { PyLong_AsSsize_t(tmp) };
        let __v1: bool =
            x == (1i32.wrapping_neg() as i64) && !unsafe { PyErr_Occurred() }.is_null();
        if __v1 {
            return 1i32.wrapping_neg() as i64;
        }
        if x <= 0 {
            // In general elements of shape may be 0, but not for casting.
            unsafe {
                PyErr_Format(
                    PyExc_ValueError,
                    c"memoryview.cast(): elements of shape must be integers > 0".as_ptr(),
                )
            };
            return 1i32.wrapping_neg() as i64;
        }
        if x > 9223372036854775807 / len {
            unsafe {
                PyErr_Format(
                    PyExc_ValueError,
                    c"memoryview.cast(): product(shape) > SSIZE_MAX".as_ptr(),
                )
            };
            return 1i32.wrapping_neg() as i64;
        }
        len = len.wrapping_mul(x);
        unsafe {
            *shape.wrapping_offset(i as isize) = x;
        }
    }
    len
}

/// Cast a 1-D array to a new shape. The result array will be C-contiguous.
/// If the result array does not have exactly the same byte length as the
/// input array, raise ValueError.
fn cast_to_ND(mv: *mut __SlateRecord_memoryobject_h_36, shape: *const _object, ndim: i32) -> i32 {
    let view: *mut __SlateRecord_pybuffer_h_33 = unsafe { core::ptr::addr_of_mut!((*mv).view) };
    let mut len: i64 = 0;
    // ndim from cast_to_1D()
    // ndim of result array
    unsafe {
        (*view).ndim = ndim;
    }
    if (unsafe { (*view).ndim }) == 0 {
        unsafe {
            (*view).shape = core::ptr::null_mut::<i64>();
        }
        unsafe {
            (*view).strides = core::ptr::null_mut::<i64>();
        }
        len = unsafe { (*view).itemsize };
    } else {
        len = copy_shape(unsafe { (*view).shape }, shape, ndim as i64, unsafe {
            (*view).itemsize
        });
        if len < 0 {
            return 1i32.wrapping_neg();
        }
        init_strides_from_shape(view);
    }
    if (unsafe { (*view).len }) != len {
        unsafe {
            PyErr_SetString(
                PyExc_TypeError,
                c"memoryview: product(shape) * itemsize != buffer size".as_ptr(),
            )
        };
        return 1i32.wrapping_neg();
    }
    init_flags(mv);
    0
}

fn zero_in_shape(mv: *mut __SlateRecord_memoryobject_h_36) -> i32 {
    let view: *mut __SlateRecord_pybuffer_h_33 = unsafe { core::ptr::addr_of_mut!((*mv).view) };
    let mut i: i64 = 0;
    while i < ((unsafe { (*view).ndim }) as i64) {
        if (unsafe { *unsafe { (*view).shape }.wrapping_offset(i as isize) }) == 0 {
            return 1;
        }
        i = i.wrapping_add(1);
    }
    0
}

// Cast a copy of 'self' to a different view. The input view must
// be C-contiguous. The function always casts the input view to a
// 1-D output according to 'format'. At least one of input-format,
// output-format must have byte size.
//
// If 'shape' is given, the 1-D view from the previous step will
// be cast to a C-contiguous view with new shape and strides.
//
// All casts must result in views that will have the exact byte
// size of the original input. Otherwise, an error is raised.
//
// [clinic input]
// memoryview.cast
//
//     format: unicode
//     shape: object = NULL
//
// Cast a memoryview to a new format or shape.
// [clinic start generated code]
/// [clinic end generated code: output=bae520b3a389cbab input=138936cc9041b1a3]
fn memoryview_cast_impl(
    self_: *mut __SlateRecord_memoryobject_h_36,
    format: *mut _object,
    shape: *mut _object,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut __v1: bool = unsafe { core::mem::zeroed() };
    let mut __v2: bool = unsafe { core::mem::zeroed() };
    let mut ndim: i64 = unsafe { core::mem::zeroed() };
    let mut mv: *mut __SlateRecord_memoryobject_h_36 = unsafe { core::mem::zeroed() };
    unsafe {
        mv = core::ptr::null_mut::<__SlateRecord_memoryobject_h_36>();
        ndim = 1;
        CHECK_RELEASED!(self_);
        CHECK_RESTRICTED!(self_);
        if MV_C_CONTIGUOUS!(unsafe { (*self_).flags }) == 0 {
            PyErr_SetString(
                PyExc_TypeError,
                c"memoryview: casts are restricted to C-contiguous views".as_ptr(),
            );
            return core::ptr::null_mut::<_object>();
        } else {
            if !shape.is_null() || (*self_).view.ndim != 1 {
                __v2 = zero_in_shape(self_) != 0;
            } else {
                __v2 = false;
            }
            if __v2 {
                PyErr_SetString(
                    PyExc_TypeError,
                    c"memoryview: cannot cast view with zeros in shape or strides".as_ptr(),
                );
                return core::ptr::null_mut::<_object>();
            } else {
                if !shape.is_null() {
                    if PyList_Check!(shape) == 0 {
                        __v1 = PyTuple_Check!(shape) == 0;
                    } else {
                        __v1 = false;
                    }
                    if __v1 {
                        PyErr_SetString(
                            PyExc_TypeError,
                            c"shape must be a list or a tuple".as_ptr(),
                        );
                        return core::ptr::null_mut::<_object>();
                    } else {
                        ndim = PySequence_Fast_GET_SIZE!(shape);
                        if ndim > 64 {
                            PyErr_SetString(
                                PyExc_ValueError,
                                c"memoryview: number of dimensions must not exceed 64".as_ptr(),
                            );
                            return core::ptr::null_mut::<_object>();
                        } else {
                            if (*self_).view.ndim != 1 && ndim != 1 {
                                PyErr_SetString(
                                    PyExc_TypeError,
                                    c"memoryview: cast must be 1D -> ND or ND -> 1D".as_ptr(),
                                );
                                return core::ptr::null_mut::<_object>();
                            }
                        }
                    }
                }
                mv = mbuf_add_incomplete_view(
                    (*self_).mbuf,
                    core::ptr::addr_of_mut!((*self_).view) as *const __SlateRecord_pybuffer_h_33,
                    if ndim == 0 { 1i32 } else { ndim as i32 },
                ) as *mut __SlateRecord_memoryobject_h_36;
                if mv.is_null() {
                    return core::ptr::null_mut::<_object>();
                } else {
                    if cast_to_1D(mv, format) >= 0 {
                        if !shape.is_null() {
                            __v0 = cast_to_ND(mv, shape as *const _object, ndim as i32) < 0;
                        } else {
                            __v0 = false;
                        }
                        if !__v0 {
                            return mv as *mut _object;
                        }
                    }
                    Py_DECREF__2!(mv);
                    return core::ptr::null_mut::<_object>();
                }
            }
        }
    }
}

// [clinic input]
// memoryview.toreadonly
//
// Return a readonly version of the memoryview.
// [clinic start generated code]
/// [clinic end generated code: output=2c7e056f04c99e62 input=dc06d20f19ba236f]
fn memoryview_toreadonly_impl(mut self_: *mut __SlateRecord_memoryobject_h_36) -> *mut _object {
    CHECK_RELEASED!(self_);
    CHECK_RESTRICTED!(self_);
    // Even if self is already readonly, we still need to create a new
    // object for .release() to work correctly.
    self_ = mbuf_add_view(
        unsafe { (*self_).mbuf },
        (unsafe { core::ptr::addr_of_mut!((*self_).view) }) as *const __SlateRecord_pybuffer_h_33,
    ) as *mut __SlateRecord_memoryobject_h_36;
    if !self_.is_null() {
        unsafe {
            (*self_).view.readonly = 1;
        }
    }
    self_ as *mut _object
}

//                               getbuffer
#[unsafe(link_section = ".text.slate_distinct.memoryobject.memory_getbuf")]
extern "C-unwind" fn memory_getbuf(
    _self: *mut _object,
    view: *mut __SlateRecord_pybuffer_h_33,
    flags: i32,
) -> i32 {
    let self_: *mut __SlateRecord_memoryobject_h_36 = _self as *mut __SlateRecord_memoryobject_h_36;
    let base: *mut __SlateRecord_pybuffer_h_33 = unsafe { core::ptr::addr_of_mut!((*self_).view) };
    let baseflags: i32 = unsafe { (*self_).flags };
    CHECK_RELEASED_INT!(self_);
    CHECK_RESTRICTED_INT!(self_);
    // start with complete information
    unsafe {
        *view = *base;
    }
    unsafe {
        (*view).obj = core::ptr::null_mut::<_object>();
    }
    if REQ_WRITABLE!(flags) != 0 && (unsafe { (*base).readonly }) != 0 {
        unsafe {
            PyErr_SetString(
                PyExc_BufferError,
                c"memoryview: underlying buffer is not writable".as_ptr(),
            )
        };
        return 1i32.wrapping_neg();
    }
    if REQ_FORMAT!(flags) == 0 {
        // NULL indicates that the buffer's data type has been cast to 'B'.
        // view->itemsize is the _previous_ itemsize. If shape is present,
        // the equality product(shape) * itemsize = len still holds at this
        // point. The equality calcsize(format) = itemsize does _not_ hold
        // from here on!
        unsafe {
            (*view).format = core::ptr::null_mut::<i8>();
        }
    }
    if REQ_C_CONTIGUOUS!(flags) && MV_C_CONTIGUOUS!(baseflags) == 0 {
        unsafe {
            PyErr_SetString(
                PyExc_BufferError,
                c"memoryview: underlying buffer is not C-contiguous".as_ptr(),
            )
        };
        return 1i32.wrapping_neg();
    }
    if REQ_F_CONTIGUOUS!(flags) && MV_F_CONTIGUOUS!(baseflags) == 0 {
        unsafe {
            PyErr_SetString(
                PyExc_BufferError,
                c"memoryview: underlying buffer is not Fortran contiguous".as_ptr(),
            )
        };
        return 1i32.wrapping_neg();
    }
    if REQ_ANY_CONTIGUOUS!(flags) && MV_ANY_CONTIGUOUS!(baseflags) == 0 {
        unsafe {
            PyErr_SetString(
                PyExc_BufferError,
                c"memoryview: underlying buffer is not contiguous".as_ptr(),
            )
        };
        return 1i32.wrapping_neg();
    }
    if !REQ_INDIRECT!(flags) && baseflags & 16 != 0 {
        unsafe {
            PyErr_SetString(
                PyExc_BufferError,
                c"memoryview: underlying buffer requires suboffsets".as_ptr(),
            )
        };
        return 1i32.wrapping_neg();
    }
    if !REQ_STRIDES!(flags) {
        if MV_C_CONTIGUOUS!(baseflags) == 0 {
            unsafe {
                PyErr_SetString(
                    PyExc_BufferError,
                    c"memoryview: underlying buffer is not C-contiguous".as_ptr(),
                )
            };
            return 1i32.wrapping_neg();
        }
        unsafe {
            (*view).strides = core::ptr::null_mut::<i64>();
        }
    }
    if !REQ_SHAPE!(flags) {
        // PyBUF_SIMPLE or PyBUF_WRITABLE: at this point buf is C-contiguous,
        // so base->buf = ndbuf->data.
        if !unsafe { (*view).format }.is_null() {
            // PyBUF_SIMPLE|PyBUF_FORMAT and PyBUF_WRITABLE|PyBUF_FORMAT do
            // not make sense.
            unsafe {
                PyErr_Format(
                    PyExc_BufferError,
                    c"memoryview: cannot cast to unsigned bytes if the format flag is present"
                        .as_ptr(),
                )
            };
            return 1i32.wrapping_neg();
        }
        // product(shape) * itemsize = len and calcsize(format) = itemsize
        // do _not_ hold from here on!
        unsafe {
            (*view).ndim = 1;
        }
        unsafe {
            (*view).shape = core::ptr::null_mut::<i64>();
        }
    }
    unsafe {
        (*view).obj = Py_NewRef__2!(self_);
    }
    let __v1: i64 = unsafe { (*self_).exports };
    unsafe {
        (*self_).exports = __v1.wrapping_add(1);
    }
    0
}

#[unsafe(link_section = ".text.slate_distinct.memoryobject.memory_releasebuf")]
extern "C-unwind" fn memory_releasebuf(
    _self: *mut _object,
    view: *mut __SlateRecord_pybuffer_h_33,
) {
    let __v1: i64 = unsafe { (*(_self as *mut __SlateRecord_memoryobject_h_36)).exports };
    unsafe {
        (*(_self as *mut __SlateRecord_memoryobject_h_36)).exports =
            __v1.wrapping_add(1i32.wrapping_neg() as i64);
    }
    return;
    // PyBuffer_Release() decrements view->obj after this function returns.
}

/// Buffer methods
/// bf_getbuffer
/// bf_releasebuffer
static mut memory_as_buffer: __SlateRecord_object_h_140 = __SlateRecord_object_h_140 {
    bf_getbuffer: Some(memory_getbuf),
    bf_releasebuffer: Some(memory_releasebuf),
};

//           Optimized pack/unpack for all native format specifiers
/// Fix exceptions:
///    1) Include format string in the error message.
///    2) OverflowError -> ValueError.
///    3) The error message from PyNumber_Index() is not ideal.
fn type_error_int(fmt: *const i8) -> i32 {
    unsafe {
        PyErr_Format(
            PyExc_TypeError,
            c"memoryview: invalid type for format '%s'".as_ptr(),
            fmt,
        )
    };
    1i32.wrapping_neg()
}

fn value_error_int(fmt: *const i8) -> i32 {
    unsafe {
        PyErr_Format(
            PyExc_ValueError,
            c"memoryview: invalid value for format '%s'".as_ptr(),
            fmt,
        )
    };
    1i32.wrapping_neg()
}

fn fix_error_int(fmt: *const i8) -> i32 {
    if (unsafe { PyErr_ExceptionMatches(PyExc_TypeError) }) != 0 {
        unsafe { PyErr_Clear() };
        return type_error_int(fmt);
    } else {
        let __v0: bool = (unsafe { PyErr_ExceptionMatches(PyExc_OverflowError) }) != 0
            || (unsafe { PyErr_ExceptionMatches(PyExc_ValueError) }) != 0;
        if __v0 {
            unsafe { PyErr_Clear() };
            return value_error_int(fmt);
        }
    }
    1i32.wrapping_neg()
}

/// UNPACK_TO_BOOL: Return 0 if PTR represents "false", and 1 otherwise.
static bool_false: bool = 0i32 != 0i32;

/// Accept integer objects or objects with an __index__() method.
fn pylong_as_ld(item: *mut _object) -> i64 {
    let mut ld: i64 = 0;
    let tmp: *mut _object = unsafe { _PyNumber_Index(item) };
    if tmp.is_null() {
        return 1i32.wrapping_neg() as i64;
    }
    ld = unsafe { PyLong_AsLong(tmp) };
    Py_DECREF!(tmp);
    ld
}

fn pylong_as_lu(item: *mut _object) -> u64 {
    let mut lu: u64 = 0;
    let tmp: *mut _object = unsafe { _PyNumber_Index(item) };
    if tmp.is_null() {
        return 1i32.wrapping_neg() as u64;
    }
    lu = unsafe { PyLong_AsUnsignedLong(tmp) };
    Py_DECREF!(tmp);
    lu
}

fn pylong_as_lld(item: *mut _object) -> i64 {
    let mut lld: i64 = 0;
    let tmp: *mut _object = unsafe { _PyNumber_Index(item) };
    if tmp.is_null() {
        return 1i32.wrapping_neg() as i64;
    }
    lld = unsafe { PyLong_AsLongLong(tmp) };
    Py_DECREF!(tmp);
    lld
}

fn pylong_as_llu(item: *mut _object) -> u64 {
    let mut llu: u64 = 0;
    let tmp: *mut _object = unsafe { _PyNumber_Index(item) };
    if tmp.is_null() {
        return 1i32.wrapping_neg() as u64;
    }
    llu = unsafe { PyLong_AsUnsignedLongLong(tmp) };
    Py_DECREF!(tmp);
    llu
}

fn pylong_as_zd(item: *mut _object) -> i64 {
    let mut zd: i64 = 0;
    let tmp: *mut _object = unsafe { _PyNumber_Index(item) };
    if tmp.is_null() {
        return 1i32.wrapping_neg() as i64;
    }
    zd = unsafe { PyLong_AsSsize_t(tmp) };
    Py_DECREF!(tmp);
    zd
}

fn pylong_as_zu(item: *mut _object) -> u64 {
    let mut zu: u64 = 0;
    let tmp: *mut _object = unsafe { _PyNumber_Index(item) };
    if tmp.is_null() {
        return 1i32.wrapping_neg() as u64;
    }
    zu = unsafe { PyLong_AsSize_t(tmp) };
    Py_DECREF!(tmp);
    zu
}

// Timings with the ndarray from _testbuffer.c indicate that using the
// struct module is around 15x slower than the two functions below.
/// Unpack a single item. 'fmt' can be any native format character in struct
/// module syntax. This function is very sensitive to small changes. With this
/// layout gcc automatically generates a fast jump table.
fn unpack_single(
    self_: *mut __SlateRecord_memoryobject_h_36,
    ptr: *const i8,
    fmt: *const i8,
) -> *mut _object {
    let mut x: *mut () = unsafe { core::mem::zeroed() };
    let mut endian: i32 = unsafe { core::mem::zeroed() };
    let mut p: *mut () = unsafe { core::mem::zeroed() };
    let mut d: f64 = unsafe { core::mem::zeroed() };
    let mut zd: i64 = unsafe { core::mem::zeroed() };
    let mut ld: i64 = unsafe { core::mem::zeroed() };
    let mut lld: i64 = unsafe { core::mem::zeroed() };
    let mut zu: u64 = unsafe { core::mem::zeroed() };
    let mut lu: u64 = unsafe { core::mem::zeroed() };
    let mut llu: u64 = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_5: {
            '__join_4: {
                CHECK_RELEASED_AGAIN!(self_);
                endian = 1;
                let __t0: i32 = *fmt.wrapping_offset(0 as isize) as i32;
                if __t0 == 66 {
                    let uc: u8 = *(ptr as *const u8);
                    return PyLong_FromLong(uc as i64);
                } else {
                    if __t0 == 98 {
                        ld = *(ptr as *const i8) as i64;
                        break '__join_5;
                    } else {
                        if __t0 == 104 {
                            UNPACK_SINGLE__8!(ld, ptr, i16);
                            break '__join_5;
                        } else {
                            if __t0 == 105 {
                                UNPACK_SINGLE__3!(ld, ptr, i32);
                                break '__join_5;
                            } else {
                                if __t0 == 108 {
                                    UNPACK_SINGLE!(ld, ptr, i64);
                                    break '__join_5;
                                } else {
                                    if __t0 == 63 {
                                        ld = UNPACK_TO_BOOL!(ptr) as i64;
                                        return PyBool_FromLong(ld);
                                    } else {
                                        if __t0 == 72 {
                                            UNPACK_SINGLE__6!(lu, ptr, u16);
                                            break '__join_4;
                                        } else {
                                            if __t0 == 73 {
                                                UNPACK_SINGLE__7!(lu, ptr, u32);
                                                break '__join_4;
                                            } else {
                                                if __t0 == 76 {
                                                    UNPACK_SINGLE__2!(lu, ptr, u64);
                                                    break '__join_4;
                                                } else {
                                                    if __t0 == 113 {
                                                        UNPACK_SINGLE!(lld, ptr, i64);
                                                        return PyLong_FromLongLong(lld);
                                                    } else {
                                                        if __t0 == 81 {
                                                            UNPACK_SINGLE__2!(llu, ptr, u64);
                                                            return PyLong_FromUnsignedLongLong(
                                                                llu,
                                                            );
                                                        } else {
                                                            if __t0 == 110 {
                                                                UNPACK_SINGLE!(zd, ptr, i64);
                                                                return PyLong_FromSsize_t(zd);
                                                            } else {
                                                                if __t0 == 78 {
                                                                    UNPACK_SINGLE__2!(zu, ptr, u64);
                                                                    return PyLong_FromSize_t(zu);
                                                                } else {
                                                                    if __t0 == 102 {
                                                                        UNPACK_SINGLE__4!(
                                                                            d, ptr, f32
                                                                        );
                                                                    } else {
                                                                        if __t0 == 100 {
                                                                            UNPACK_SINGLE__5!(
                                                                                d, ptr, f64
                                                                            );
                                                                        } else {
                                                                            if __t0 == 101 {
                                                                                d = PyFloat_Unpack2(
                                                                                    ptr, endian,
                                                                                );
                                                                            } else {
                                                                                if __t0 == 99 {
                                                                                    return PyBytes_FromStringAndSize(ptr, 1i64);
                                                                                } else {
                                                                                    if __t0 == 80 {
                                                                                        loop {
                                                                                            core::ptr::copy_nonoverlapping(((ptr as *const ()) as *const core::ffi::c_void) as *const u8, (((core::ptr::addr_of_mut!(x) as *mut i8) as *mut ()) as *mut core::ffi::c_void) as *mut u8, ((core::mem::size_of::<*mut ()>() as u64) as usize) as usize);
                                                                                            p = x;
                                                                                            if true
                                                                                            {
                                                                                                break;
                                                                                            }
                                                                                        }
                                                                                        return PyLong_FromVoidPtr(p);
                                                                                    } else {
                                                                                        PyErr_Format(PyExc_NotImplementedError, c"memoryview: format %s not supported".as_ptr(), fmt);
                                                                                        return core::ptr::null_mut::<_object>();
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                return PyFloat_FromDouble(d);
            }
            return PyLong_FromUnsignedLong(lu);
        }
        return PyLong_FromLong(ld);
    }
    // signed integers and fast path for 'B'
    // boolean
    // unsigned integers
    // native 64-bit
    // ssize_t and size_t
    // floats
    // bytes object
    // pointer
    // default
    // PyLong_FromUnsignedLong() is slower
}

/// Pack a single item. 'fmt' can be any native format character in
/// struct module syntax.
fn pack_single(
    self_: *mut __SlateRecord_memoryobject_h_36,
    ptr: *mut i8,
    item: *mut _object,
    fmt: *const i8,
) -> i32 {
    let mut x: *mut () = unsafe { core::mem::zeroed() };
    let mut __v1: bool = unsafe { core::mem::zeroed() };
    let mut __v2: bool = unsafe { core::mem::zeroed() };
    let mut __v3: bool = unsafe { core::mem::zeroed() };
    let mut __v4: bool = unsafe { core::mem::zeroed() };
    let mut __v5: bool = unsafe { core::mem::zeroed() };
    let mut __v6: bool = unsafe { core::mem::zeroed() };
    let mut __v7: bool = unsafe { core::mem::zeroed() };
    let mut __v8: bool = unsafe { core::mem::zeroed() };
    let mut endian: i32 = unsafe { core::mem::zeroed() };
    let mut p: *mut () = unsafe { core::mem::zeroed() };
    let mut d: f64 = unsafe { core::mem::zeroed() };
    let mut zd: i64 = unsafe { core::mem::zeroed() };
    let mut ld: i64 = unsafe { core::mem::zeroed() };
    let mut lld: i64 = unsafe { core::mem::zeroed() };
    let mut zu: u64 = unsafe { core::mem::zeroed() };
    let mut lu: u64 = unsafe { core::mem::zeroed() };
    let mut llu: u64 = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_2: {
            '__join_1: {
                '__join_3: {
                    '__join_70: {
                        '__join_58: {
                            endian = 1;
                            let __t2: i32 = *fmt.wrapping_offset(0 as isize) as i32;
                            if __t2 == 98 {
                                break '__join_70;
                            } else {
                                if __t2 == 104 {
                                    break '__join_70;
                                } else {
                                    if __t2 == 105 {
                                        break '__join_70;
                                    } else {
                                        if __t2 == 108 {
                                            break '__join_70;
                                        } else {
                                            if __t2 == 66 {
                                                break '__join_58;
                                            } else {
                                                if __t2 == 72 {
                                                    break '__join_58;
                                                } else {
                                                    if __t2 == 73 {
                                                        break '__join_58;
                                                    } else {
                                                        if __t2 == 76 {
                                                            break '__join_58;
                                                        } else {
                                                            if __t2 == 113 {
                                                                lld = pylong_as_lld(item);
                                                                if lld
                                                                    == (1i32.wrapping_neg() as i64)
                                                                {
                                                                    __v6 =
                                                                        !PyErr_Occurred().is_null();
                                                                } else {
                                                                    __v6 = false;
                                                                }
                                                                if __v6 {
                                                                    break '__join_2;
                                                                } else {
                                                                    CHECK_RELEASED_INT_AGAIN!(
                                                                        self_
                                                                    );
                                                                    PACK_SINGLE!(ptr, lld, i64);
                                                                    break '__join_3;
                                                                }
                                                            } else {
                                                                if __t2 == 81 {
                                                                    llu = pylong_as_llu(item);
                                                                    if llu
                                                                        == (1i32.wrapping_neg()
                                                                            as u64)
                                                                    {
                                                                        __v5 = !PyErr_Occurred()
                                                                            .is_null();
                                                                    } else {
                                                                        __v5 = false;
                                                                    }
                                                                    if __v5 {
                                                                        break '__join_2;
                                                                    } else {
                                                                        CHECK_RELEASED_INT_AGAIN!(
                                                                            self_
                                                                        );
                                                                        PACK_SINGLE__2!(
                                                                            ptr, llu, u64
                                                                        );
                                                                        break '__join_3;
                                                                    }
                                                                } else {
                                                                    if __t2 == 110 {
                                                                        zd = pylong_as_zd(item);
                                                                        if zd
                                                                            == (1i32.wrapping_neg()
                                                                                as i64)
                                                                        {
                                                                            __v4 =
                                                                                !PyErr_Occurred()
                                                                                    .is_null();
                                                                        } else {
                                                                            __v4 = false;
                                                                        }
                                                                        if __v4 {
                                                                            break '__join_2;
                                                                        } else {
                                                                            CHECK_RELEASED_INT_AGAIN!(
                                                                                self_
                                                                            );
                                                                            PACK_SINGLE!(
                                                                                ptr, zd, i64
                                                                            );
                                                                            break '__join_3;
                                                                        }
                                                                    } else {
                                                                        if __t2 == 78 {
                                                                            zu = pylong_as_zu(item);
                                                                            if zu
                                                                                == (1i32
                                                                                    .wrapping_neg()
                                                                                    as u64)
                                                                            {
                                                                                __v3 = !PyErr_Occurred().is_null();
                                                                            } else {
                                                                                __v3 = false;
                                                                            }
                                                                            if __v3 {
                                                                                break '__join_2;
                                                                            } else {
                                                                                CHECK_RELEASED_INT_AGAIN!(
                                                                                    self_
                                                                                );
                                                                                PACK_SINGLE__2!(
                                                                                    ptr, zu, u64
                                                                                );
                                                                                break '__join_3;
                                                                            }
                                                                        } else {
                                                                            if __t2 != 102 {
                                                                                if __t2 != 100 {
                                                                                    if __t2 != 101 {
                                                                                        if __t2
                                                                                            == 63
                                                                                        {
                                                                                            ld = PyObject_IsTrue(item) as i64;
                                                                                            if ld
                                                                                                < 0
                                                                                            {
                                                                                                return 1i32.wrapping_neg();
                                                                                            } else {
                                                                                                // preserve original error
                                                                                                CHECK_RELEASED_INT_AGAIN!(self_);
                                                                                                PACK_SINGLE__6!(ptr, ld, bool);
                                                                                                break '__join_3;
                                                                                            }
                                                                                        } else {
                                                                                            if __t2 == 99 {
if PyBytes_Check!(item) == 0 {
return type_error_int(fmt);
} else {
if PyBytes_GET_SIZE!(item) != 1 {
return value_error_int(fmt);
} else {
*ptr = *PyBytes_AS_STRING!(item).wrapping_offset(0 as isize);
break '__join_3;
}
}
} else {
if __t2 == 80 {
p = PyLong_AsVoidPtr(item);
if p.is_null() {
__v1 = !PyErr_Occurred().is_null();
} else {
__v1 = false;
}
if __v1 {
break '__join_2;
} else {
CHECK_RELEASED_INT_AGAIN!(self_);
loop {
x = p;
core::ptr::copy_nonoverlapping((((core::ptr::addr_of_mut!(x) as *mut i8) as *const ()) as *const core::ffi::c_void) as *const u8, ((ptr as *mut ()) as *mut core::ffi::c_void) as *mut u8, ((core::mem::size_of::<*mut ()>() as u64) as usize) as usize);
if true {
break '__join_3;
}
}
}
} else {
PyErr_Format(PyExc_NotImplementedError, c"memoryview: format %s not supported".as_ptr(), fmt);
return 1i32.wrapping_neg();
}
}
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            d = PyFloat_AsDouble(item);
                            if d == -1.0f64 {
                                __v2 = !PyErr_Occurred().is_null();
                            } else {
                                __v2 = false;
                            }
                            if __v2 {
                                break '__join_2;
                            } else {
                                CHECK_RELEASED_INT_AGAIN!(self_);
                                if (*fmt.wrapping_offset(0 as isize) as i32) == 102 {
                                    PACK_SINGLE__9!(ptr, d, f32);
                                    break '__join_3;
                                } else {
                                    if (*fmt.wrapping_offset(0 as isize) as i32) == 100 {
                                        PACK_SINGLE__5!(ptr, d, f64);
                                        break '__join_3;
                                    } else {
                                        if PyFloat_Pack2(d, ptr, endian) < 0 {
                                            break '__join_2;
                                        } else {
                                            break '__join_3;
                                        }
                                    }
                                }
                            }
                        }
                        lu = pylong_as_lu(item);
                        if lu == (1i32.wrapping_neg() as u64) {
                            __v7 = !PyErr_Occurred().is_null();
                        } else {
                            __v7 = false;
                        }
                        if __v7 {
                            break '__join_2;
                        } else {
                            CHECK_RELEASED_INT_AGAIN!(self_);
                            let __t0: i32 = *fmt.wrapping_offset(0 as isize) as i32;
                            if __t0 == 66 {
                                if lu > 255u64 {
                                    break '__join_1;
                                } else {
                                    *(ptr as *mut u8) = lu as u8;
                                    break '__join_3;
                                }
                            } else {
                                if __t0 == 72 {
                                    if lu > 65535u64 {
                                        break '__join_1;
                                    } else {
                                        PACK_SINGLE__8!(ptr, lu, u16);
                                        break '__join_3;
                                    }
                                } else {
                                    if __t0 == 73 {
                                        if lu > 4294967295u64 {
                                            break '__join_1;
                                        } else {
                                            PACK_SINGLE__7!(ptr, lu, u32);
                                            break '__join_3;
                                        }
                                    } else {
                                        PACK_SINGLE__2!(ptr, lu, u64); // 'L'
                                        break '__join_3;
                                    }
                                }
                            }
                        }
                    }
                    ld = pylong_as_ld(item);
                    if ld == (1i32.wrapping_neg() as i64) {
                        __v8 = !PyErr_Occurred().is_null();
                    } else {
                        __v8 = false;
                    }
                    if __v8 {
                        break '__join_2;
                    } else {
                        CHECK_RELEASED_INT_AGAIN!(self_);
                        let __t1: i32 = *fmt.wrapping_offset(0 as isize) as i32;
                        if __t1 == 98 {
                            if ld < (127i32.wrapping_neg().wrapping_sub(1) as i64) || ld > 127 {
                                break '__join_1;
                            } else {
                                *(ptr as *mut i8) = ld as i8;
                            }
                        } else {
                            if __t1 == 104 {
                                if ld < (32767i32.wrapping_neg().wrapping_sub(1) as i64)
                                    || ld > 32767
                                {
                                    break '__join_1;
                                } else {
                                    PACK_SINGLE__4!(ptr, ld, i16);
                                }
                            } else {
                                if __t1 == 105 {
                                    if ld < (2147483647i32.wrapping_neg().wrapping_sub(1) as i64)
                                        || ld > 2147483647
                                    {
                                        break '__join_1;
                                    } else {
                                        PACK_SINGLE__3!(ptr, ld, i32);
                                    }
                                } else {
                                    PACK_SINGLE!(ptr, ld, i64); // 'l'
                                }
                            }
                        }
                    }
                }
                return 0;
            }
            return value_error_int(fmt);
        }
        return fix_error_int(fmt);
    }
    // signed integers
    // unsigned integers
    // native 64-bit
    // ssize_t and size_t
    // floats
    // bool
    // bytes object
    // pointer
    // default
}

//                       unpack using the struct module
/// For reasonable performance it is necessary to cache all objects required
/// for unpacking. An unpacker can handle the format passed to unpack_from().
/// Invariant: All pointer fields of the struct should either be NULL or valid
/// pointers.
#[repr(C)]
#[derive(Clone, Copy)]
struct unpacker {
    /// Struct.unpack_from(format)
    unpack_from: *mut _object,
    /// cached memoryview
    mview: *mut _object,
    /// buffer for mview
    item: *mut i8,
    /// len(item)
    itemsize: i64,
}

fn unpacker_new() -> *mut unpacker {
    let x: *mut unpacker =
        (unsafe { PyMem_Malloc(core::mem::size_of::<unpacker>() as u64) }) as *mut unpacker;
    if x.is_null() {
        unsafe { PyErr_NoMemory() };
        return core::ptr::null_mut::<unpacker>();
    }
    unsafe {
        (*x).unpack_from = core::ptr::null_mut::<_object>();
    }
    unsafe {
        (*x).mview = core::ptr::null_mut::<_object>();
    }
    unsafe {
        (*x).item = core::ptr::null_mut::<i8>();
    }
    unsafe {
        (*x).itemsize = 0;
    }
    x
}

fn unpacker_free(x: *mut unpacker) {
    if !x.is_null() {
        Py_XDECREF!(unsafe { (*x).unpack_from });
        Py_XDECREF!(unsafe { (*x).mview });
        unsafe { PyMem_Free((*x).item as *mut ()) };
        unsafe { PyMem_Free(x as *mut ()) };
    }
}

/// Return a new unpacker for the given format.
fn struct_get_unpacker(fmt: *const i8, itemsize: i64) -> *mut unpacker {
    let mut x: *mut unpacker = unsafe { core::mem::zeroed() };
    let mut format: *mut _object = unsafe { core::mem::zeroed() };
    let mut structobj: *mut _object = unsafe { core::mem::zeroed() };
    // XXX cache it in globals?
    let mut Struct: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        Struct = core::ptr::null_mut::<_object>();
        structobj = core::ptr::null_mut::<_object>();
        format = core::ptr::null_mut::<_object>();
        x = core::ptr::null_mut::<unpacker>();
        Struct = PyImport_ImportModuleAttrString(c"struct".as_ptr(), c"Struct".as_ptr());
        if Struct.is_null() {
            return core::ptr::null_mut::<unpacker>();
        } else {
            '__join_1: {
                x = unpacker_new();
                if !x.is_null() {
                    format = PyBytes_FromString(fmt);
                    if !format.is_null() {
                        structobj = PyObject_CallOneArg(Struct, format);
                        if !structobj.is_null() {
                            (*x).unpack_from =
                                PyObject_GetAttrString(structobj, c"unpack_from".as_ptr());
                            if !unsafe { (*x).unpack_from }.is_null() {
                                (*x).item = PyMem_Malloc(itemsize as u64) as *mut i8;
                                if unsafe { (*x).item }.is_null() {
                                    PyErr_NoMemory();
                                } else {
                                    (*x).itemsize = itemsize;
                                    (*x).mview =
                                        PyMemoryView_FromMemory((*x).item, itemsize, 512i32);
                                    if !unsafe { (*x).mview }.is_null() {
                                        break '__join_1;
                                    }
                                }
                            }
                        }
                    }
                }
                unpacker_free(x);
                x = core::ptr::null_mut::<unpacker>();
            }
            Py_XDECREF!(Struct);
            Py_XDECREF!(format);
            Py_XDECREF!(structobj);
            return x;
        }
    }
}

/// unpack a single item
fn struct_unpack_single(ptr: *const i8, x: *mut unpacker) -> *mut _object {
    unsafe {
        core::ptr::copy_nonoverlapping(
            ((ptr as *const ()) as *const core::ffi::c_void) as *const u8,
            (((*x).item as *mut ()) as *mut core::ffi::c_void) as *mut u8,
            (((*x).itemsize as u64) as usize) as usize,
        )
    };
    let v: *mut _object = unsafe { PyObject_CallOneArg((*x).unpack_from, (*x).mview) };
    if v.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    if PyTuple_GET_SIZE!(v) == 1 {
        let res: *mut _object = Py_NewRef!(PyTuple_GET_ITEM!(v, 0i32));
        Py_DECREF!(v);
        return res;
    }
    v
}

//                              Representations
/// allow explicit form of native format
fn adjust_fmt(view: *const __SlateRecord_pybuffer_h_33) -> *const i8 {
    let fmt: *const i8 =
        (if ((unsafe { *unsafe { (*view).format }.wrapping_offset(0 as isize) }) as i32) == 64 {
            unsafe { (*view).format }.wrapping_offset(1 as isize)
        } else {
            unsafe { (*view).format }
        }) as *const i8;
    if (unsafe { *fmt.wrapping_offset(0 as isize) }) != 0
        && ((unsafe { *fmt.wrapping_offset(1 as isize) }) as i32) == 0
    {
        return fmt;
    }
    unsafe {
        PyErr_Format(
            PyExc_NotImplementedError,
            c"memoryview: unsupported format %s".as_ptr(),
            (*view).format,
        )
    };
    core::ptr::null::<i8>()
}

/// Base case for multi-dimensional unpacking. Assumption: ndim == 1.
fn tolist_base(
    self_: *mut __SlateRecord_memoryobject_h_36,
    mut ptr: *const i8,
    shape: *const i64,
    strides: *const i64,
    mut suboffsets: *const i64,
    fmt: *const i8,
) -> *mut _object {
    let mut i: i64 = 0;
    let lst: *mut _object = unsafe { PyList_New(*shape.wrapping_offset(0 as isize)) };
    if lst.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    while i < unsafe { *shape.wrapping_offset(0 as isize) } {
        let xptr: *const i8 = ADJUST_PTR!(ptr, suboffsets, 0i32);
        let item: *mut _object = unpack_single(self_, xptr, fmt);
        if item.is_null() {
            Py_DECREF!(lst);
            return core::ptr::null_mut::<_object>();
        }
        PyList_SET_ITEM!(lst, i, item);
        let __v1: *const i8 = ptr;
        ptr = __v1.wrapping_offset((unsafe { *strides.wrapping_offset(0 as isize) }) as isize);
        i = i.wrapping_add(1);
    }
    lst
}

/// Unpack a multi-dimensional array into a nested list.
/// Assumption: ndim >= 1.
fn tolist_rec(
    self_: *mut __SlateRecord_memoryobject_h_36,
    mut ptr: *const i8,
    ndim: i64,
    shape: *const i64,
    strides: *const i64,
    mut suboffsets: *const i64,
    fmt: *const i8,
) -> *mut _object {
    let mut i: i64 = 0;
    if ndim == 1 {
        return tolist_base(self_, ptr, shape, strides, suboffsets, fmt);
    }
    let lst: *mut _object = unsafe { PyList_New(*shape.wrapping_offset(0 as isize)) };
    if lst.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    while i < unsafe { *shape.wrapping_offset(0 as isize) } {
        let xptr: *const i8 = ADJUST_PTR!(ptr, suboffsets, 0i32);
        let item: *mut _object = tolist_rec(
            self_,
            xptr,
            ndim.wrapping_sub(1),
            shape.wrapping_offset(1 as isize),
            strides.wrapping_offset(1 as isize),
            if !suboffsets.is_null() {
                suboffsets.wrapping_offset(1 as isize)
            } else {
                core::ptr::null::<i64>()
            },
            fmt,
        );
        if item.is_null() {
            Py_DECREF!(lst);
            return core::ptr::null_mut::<_object>();
        }
        PyList_SET_ITEM!(lst, i, item);
        let __v1: *const i8 = ptr;
        ptr = __v1.wrapping_offset((unsafe { *strides.wrapping_offset(0 as isize) }) as isize);
        i = i.wrapping_add(1);
    }
    lst
}

// Return a list representation of the memoryview. Currently only buffers
// with native format strings are supported.
//
// [clinic input]
// memoryview.tolist
//
// Return the data in the buffer as a list of elements.
// [clinic start generated code]
/// [clinic end generated code: output=a6cda89214fd5a1b input=21e7d0c1860b211a]
fn memoryview_tolist_impl(self_: *mut __SlateRecord_memoryobject_h_36) -> *mut _object {
    let view: *const __SlateRecord_pybuffer_h_33 =
        (unsafe { core::ptr::addr_of_mut!((*self_).view) }) as *const __SlateRecord_pybuffer_h_33;
    CHECK_RELEASED!(self_);
    let fmt: *const i8 = adjust_fmt(view);
    if fmt.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    if (unsafe { (*view).ndim }) == 0 {
        return unpack_single(self_, (unsafe { (*view).buf }) as *const i8, fmt);
    } else {
        if (unsafe { (*view).ndim }) == 1 {
            return tolist_base(
                self_,
                (unsafe { (*view).buf }) as *const i8,
                (unsafe { (*view).shape }) as *const i64,
                (unsafe { (*view).strides }) as *const i64,
                (unsafe { (*view).suboffsets }) as *const i64,
                fmt,
            );
        } else {
            return tolist_rec(
                self_,
                (unsafe { (*view).buf }) as *const i8,
                (unsafe { (*view).ndim }) as i64,
                (unsafe { (*view).shape }) as *const i64,
                (unsafe { (*view).strides }) as *const i64,
                (unsafe { (*view).suboffsets }) as *const i64,
                fmt,
            );
        }
    }
}

// [clinic input]
// memoryview.tobytes
//
//     order: str(accept={str, NoneType}, c_default="NULL") = 'C'
//
// Return the data in the buffer as a byte string.
//
// Order can be {'C', 'F', 'A'}.  When order is 'C' or 'F', the data of
// the original array is converted to C or Fortran order.  For
// contiguous views, 'A' returns an exact copy of the physical memory.
// In particular, in-memory Fortran order is preserved.  For
// non-contiguous views, the data is converted to C first.  order=None
// is the same as order='C'.
// [clinic start generated code]
/// [clinic end generated code: output=1288b62560a32a23 input=119c70aa91791dc8]
fn memoryview_tobytes_impl(
    self_: *mut __SlateRecord_memoryobject_h_36,
    order: *const i8,
) -> *mut _object {
    let src: *mut __SlateRecord_pybuffer_h_33 = VIEW_ADDR!(self_);
    let mut ord: i8 = 67;
    CHECK_RELEASED!(self_);
    if !order.is_null() {
        if (unsafe {
            strcmp(
                order as *const core::ffi::c_char,
                c"F".as_ptr() as *const core::ffi::c_char,
            ) as i32
        }) == 0
        {
            ord = 70;
        } else {
            if (unsafe {
                strcmp(
                    order as *const core::ffi::c_char,
                    c"A".as_ptr() as *const core::ffi::c_char,
                ) as i32
            }) == 0
            {
                ord = 65;
            } else {
                if (unsafe {
                    strcmp(
                        order as *const core::ffi::c_char,
                        c"C".as_ptr() as *const core::ffi::c_char,
                    ) as i32
                }) != 0
                {
                    unsafe {
                        PyErr_SetString(PyExc_ValueError, c"order must be 'C', 'F' or 'A'".as_ptr())
                    };
                    return core::ptr::null_mut::<_object>();
                }
            }
        }
    }
    let bytes: *mut _object =
        unsafe { PyBytes_FromStringAndSize(core::ptr::null::<i8>(), (*src).len) };
    if bytes.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    if PyBuffer_ToContiguous(
        PyBytes_AS_STRING!(bytes) as *mut (),
        src as *const __SlateRecord_pybuffer_h_33,
        unsafe { (*src).len },
        ord,
    ) < 0
    {
        Py_DECREF!(bytes);
        return core::ptr::null_mut::<_object>();
    }
    bytes
}

// [clinic input]
// memoryview.hex
//
//     sep: object = NULL
//         An optional single character or byte to separate hex bytes.
//     bytes_per_sep: int = 1
//         How many bytes between separators.  Positive values count from
//         the right, negative values count from the left.
//
// Return the data in the buffer as a str of hexadecimal numbers.
//
// Example:
// >>> value = memoryview(b'\xb9\x01\xef')
// >>> value.hex()
// 'b901ef'
// >>> value.hex(':')
// 'b9:01:ef'
// >>> value.hex(':', 2)
// 'b9:01ef'
// >>> value.hex(':', -2)
// 'b901:ef'
// [clinic start generated code]
/// [clinic end generated code: output=430ca760f94f3ca7 input=94c2495f886c786b]
fn memoryview_hex_impl(
    self_: *mut __SlateRecord_memoryobject_h_36,
    sep: *mut _object,
    bytes_per_sep: i32,
) -> *mut _object {
    let src: *mut __SlateRecord_pybuffer_h_33 = VIEW_ADDR!(self_);
    CHECK_RELEASED!(self_);
    if MV_C_CONTIGUOUS!(unsafe { (*self_).flags }) != 0 {
        // Prevent 'self' from being freed if computing len(sep) mutates 'self'
        //
        // in _Py_strhex_with_sep().
        //
        // See: https://github.com/python/cpython/issues/143195.
        let __v1: i64 = unsafe { (*self_).exports };
        unsafe {
            (*self_).exports = __v1.wrapping_add(1);
        }
        let ret: *mut _object =
            unsafe { _Py_strhex_with_sep((*src).buf as *const i8, (*src).len, sep, bytes_per_sep) };
        let __v4: i64 = unsafe { (*self_).exports };
        unsafe {
            (*self_).exports = __v4.wrapping_add(1i32.wrapping_neg() as i64);
        }
        return ret;
    }
    let bytes: *mut _object =
        unsafe { PyBytes_FromStringAndSize(core::ptr::null::<i8>(), (*src).len) };
    if bytes.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    if PyBuffer_ToContiguous(
        PyBytes_AS_STRING!(bytes) as *mut (),
        src as *const __SlateRecord_pybuffer_h_33,
        unsafe { (*src).len },
        67i8,
    ) < 0
    {
        Py_DECREF!(bytes);
        return core::ptr::null_mut::<_object>();
    }
    let ret: *mut _object = unsafe {
        _Py_strhex_with_sep(
            PyBytes_AS_STRING!(bytes) as *const i8,
            PyBytes_GET_SIZE!(bytes),
            sep,
            bytes_per_sep,
        )
    };
    Py_DECREF!(bytes);
    ret
}

#[unsafe(link_section = ".text.slate_distinct.memoryobject.memory_repr")]
extern "C-unwind" fn memory_repr(_self: *mut _object) -> *mut _object {
    let self_: *mut __SlateRecord_memoryobject_h_36 = _self as *mut __SlateRecord_memoryobject_h_36;
    if (unsafe { (*self_).flags }) & 1 != 0 {
        return unsafe { PyUnicode_FromFormat(c"<released memory at %p>".as_ptr(), self_) };
    } else {
        return unsafe { PyUnicode_FromFormat(c"<memory at %p>".as_ptr(), self_) };
    }
}

//                          Indexing and slicing
fn lookup_dimension(
    view: *const __SlateRecord_pybuffer_h_33,
    mut ptr: *mut i8,
    dim: i32,
    mut index: i64,
) -> *mut i8 {
    let mut nitems: i64 = 0; // items in the given dimension
    nitems = unsafe { *unsafe { (*view).shape }.wrapping_offset(dim as isize) };
    if index < 0 {
        index = index.wrapping_add(nitems);
    }
    if index < 0 || index >= nitems {
        unsafe {
            PyErr_Format(
                PyExc_IndexError,
                c"index out of bounds on dimension %d".as_ptr(),
                dim.wrapping_add(1),
            )
        };
        return core::ptr::null_mut::<i8>();
    }
    let __v2: *mut i8 = ptr;
    ptr = __v2.wrapping_offset(
        unsafe { *unsafe { (*view).strides }.wrapping_offset(dim as isize) }.wrapping_mul(index)
            as isize,
    );
    ptr = ADJUST_PTR__3!(ptr, (*view).suboffsets, dim);
    ptr
}

/// Get the pointer to the item at index.
fn ptr_from_index(view: *const __SlateRecord_pybuffer_h_33, index: i64) -> *mut i8 {
    let ptr: *mut i8 = (unsafe { (*view).buf }) as *mut i8;
    lookup_dimension(view, ptr, 0i32, index)
}

/// Get the pointer to the item at tuple.
fn ptr_from_tuple(view: *const __SlateRecord_pybuffer_h_33, tup: *mut _object) -> *mut i8 {
    let mut ptr: *mut i8 = (unsafe { (*view).buf }) as *mut i8;
    let nindices: i64 = PyTuple_GET_SIZE!(tup);
    if nindices > ((unsafe { (*view).ndim }) as i64) {
        unsafe {
            PyErr_Format(
                PyExc_TypeError,
                c"cannot index %d-dimension view with %zd-element tuple".as_ptr(),
                (*view).ndim,
                nindices,
            )
        };
        return core::ptr::null_mut::<i8>();
    }
    for dim in 0..nindices {
        let index: i64 =
            unsafe { PyNumber_AsSsize_t(PyTuple_GET_ITEM!(tup, dim), PyExc_IndexError) };
        let __v0: bool =
            index == (1i32.wrapping_neg() as i64) && !unsafe { PyErr_Occurred() }.is_null();
        if __v0 {
            return core::ptr::null_mut::<i8>();
        }
        ptr = lookup_dimension(view, ptr, dim as i32, index);
        if ptr.is_null() {
            return core::ptr::null_mut::<i8>();
        }
    }
    ptr
}

/// Return the item at index. In a one-dimensional view, this is an object
/// with the type specified by view->format. Otherwise, the item is a sub-view.
/// The function is used in memory_subscript() and memory_as_sequence.
#[unsafe(link_section = ".text.slate_distinct.memoryobject.memory_item")]
extern "C-unwind" fn memory_item(_self: *mut _object, index: i64) -> *mut _object {
    let self_: *mut __SlateRecord_memoryobject_h_36 = _self as *mut __SlateRecord_memoryobject_h_36;
    let view: *mut __SlateRecord_pybuffer_h_33 = unsafe { core::ptr::addr_of_mut!((*self_).view) };
    CHECK_RELEASED!(self_);
    let fmt: *const i8 = adjust_fmt(view as *const __SlateRecord_pybuffer_h_33);
    if fmt.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    if (unsafe { (*view).ndim }) == 0 {
        unsafe {
            PyErr_SetString(
                PyExc_TypeError,
                c"invalid indexing of 0-dim memory".as_ptr(),
            )
        };
        return core::ptr::null_mut::<_object>();
    }
    if (unsafe { (*view).ndim }) == 1 {
        let ptr: *mut i8 = ptr_from_index(view as *const __SlateRecord_pybuffer_h_33, index);
        if ptr.is_null() {
            return core::ptr::null_mut::<_object>();
        }
        return unpack_single(self_, ptr as *const i8, fmt);
    }
    unsafe {
        PyErr_SetString(
            PyExc_NotImplementedError,
            c"multi-dimensional sub-views are not implemented".as_ptr(),
        )
    };
    core::ptr::null_mut::<_object>()
}

/// Return the item at position *key* (a tuple of indices).
fn memory_item_multi(
    self_: *mut __SlateRecord_memoryobject_h_36,
    tup: *mut _object,
) -> *mut _object {
    let view: *mut __SlateRecord_pybuffer_h_33 = unsafe { core::ptr::addr_of_mut!((*self_).view) };
    let nindices: i64 = PyTuple_GET_SIZE!(tup);
    CHECK_RELEASED!(self_);
    let fmt: *const i8 = adjust_fmt(view as *const __SlateRecord_pybuffer_h_33);
    if fmt.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    if nindices < ((unsafe { (*view).ndim }) as i64) {
        unsafe {
            PyErr_SetString(
                PyExc_NotImplementedError,
                c"sub-views are not implemented".as_ptr(),
            )
        };
        return core::ptr::null_mut::<_object>();
    }
    let ptr: *mut i8 = ptr_from_tuple(view as *const __SlateRecord_pybuffer_h_33, tup);
    if ptr.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    unpack_single(self_, ptr as *const i8, fmt)
}

fn init_slice(base: *mut __SlateRecord_pybuffer_h_33, key: *mut _object, dim: i32) -> i32 {
    let mut n: i64 = unsafe { core::mem::zeroed() };
    let mut slicelength: i64 = unsafe { core::mem::zeroed() };
    let mut step: i64 = unsafe { core::mem::zeroed() };
    let mut stop: i64 = unsafe { core::mem::zeroed() };
    let mut start: i64 = unsafe { core::mem::zeroed() };
    unsafe {
        if PySlice_Unpack(
            key,
            core::ptr::addr_of_mut!(start),
            core::ptr::addr_of_mut!(stop),
            core::ptr::addr_of_mut!(step),
        ) < 0
        {
            return 1i32.wrapping_neg();
        } else {
            '__join_0: {
                slicelength = PySlice_AdjustIndices(
                    *unsafe { (*base).shape }.wrapping_offset(dim as isize),
                    core::ptr::addr_of_mut!(start),
                    core::ptr::addr_of_mut!(stop),
                    step,
                );
                if !(unsafe { (*base).suboffsets }.is_null() || dim == 0) {
                    n = dim.wrapping_sub(1) as i64;
                    while n >= 0 && *unsafe { (*base).suboffsets }.wrapping_offset(n as isize) < 0 {
                        n = n.wrapping_sub(1);
                    }
                    if n >= 0 {
                        // all suboffsets are negative
                        *unsafe { (*base).suboffsets }.wrapping_offset(n as isize) =
                            unsafe { *unsafe { (*base).suboffsets }.wrapping_offset(n as isize) }
                                .wrapping_add(
                                    unsafe {
                                        *unsafe { (*base).strides }.wrapping_offset(dim as isize)
                                    }
                                    .wrapping_mul(start),
                                );
                        break '__join_0;
                    }
                }
                (*base).buf = ((*base).buf as *mut i8).wrapping_offset(
                    unsafe { *unsafe { (*base).strides }.wrapping_offset(dim as isize) }
                        .wrapping_mul(start) as isize,
                ) as *mut ();
            }
            *unsafe { (*base).shape }.wrapping_offset(dim as isize) = slicelength;
            *unsafe { (*base).strides }.wrapping_offset(dim as isize) =
                unsafe { *unsafe { (*base).strides }.wrapping_offset(dim as isize) }
                    .wrapping_mul(step);
            return 0;
        }
    }
}

fn is_multislice(key: *mut _object) -> i32 {
    let mut size: i64 = 0;
    if PyTuple_Check!(key) == 0 {
        return 0;
    }
    size = PyTuple_GET_SIZE!(key);
    if size == 0 {
        return 0;
    }
    for i in 0..size {
        let x: *mut _object = PyTuple_GET_ITEM!(key, i);
        if PySlice_Check!(x) == 0 {
            return 0;
        }
    }
    1
}

fn is_multiindex(key: *mut _object) -> i64 {
    let mut size: i64 = 0;
    if PyTuple_Check!(key) == 0 {
        return 0;
    }
    size = PyTuple_GET_SIZE!(key);
    for i in 0..size {
        let x: *mut _object = PyTuple_GET_ITEM!(key, i);
        if _PyIndex_Check(x) == 0 {
            return 0;
        }
    }
    1
}

/// mv[obj] returns an object holding the data for one element if obj
/// fully indexes the memoryview or another memoryview object if it
/// does not.
///
/// 0-d memoryview objects can be referenced using mv[...] or mv[()]
/// but not with anything else.
#[unsafe(link_section = ".text.slate_distinct.memoryobject.memory_subscript")]
extern "C-unwind" fn memory_subscript(_self: *mut _object, key: *mut _object) -> *mut _object {
    let self_: *mut __SlateRecord_memoryobject_h_36 = _self as *mut __SlateRecord_memoryobject_h_36;
    let view: *mut __SlateRecord_pybuffer_h_33 = unsafe { core::ptr::addr_of_mut!((*self_).view) };
    CHECK_RELEASED!(self_);
    if (unsafe { (*view).ndim }) == 0 {
        let __v0: bool = PyTuple_Check!(key) != 0 && PyTuple_GET_SIZE!(key) == 0;
        if __v0 {
            let fmt: *const i8 = adjust_fmt(view as *const __SlateRecord_pybuffer_h_33);
            if fmt.is_null() {
                return core::ptr::null_mut::<_object>();
            }
            return unpack_single(self_, (unsafe { (*view).buf }) as *const i8, fmt);
        } else {
            if key == unsafe { core::ptr::addr_of_mut!(_Py_EllipsisObject) } {
                return Py_NewRef__2!(self_);
            } else {
                unsafe {
                    PyErr_SetString(
                        PyExc_TypeError,
                        c"invalid indexing of 0-dim memory".as_ptr(),
                    )
                };
                return core::ptr::null_mut::<_object>();
            }
        }
    }
    if _PyIndex_Check(key) != 0 {
        let index: i64 = unsafe { PyNumber_AsSsize_t(key, PyExc_IndexError) };
        let __v1: bool =
            index == (1i32.wrapping_neg() as i64) && !unsafe { PyErr_Occurred() }.is_null();
        if __v1 {
            return core::ptr::null_mut::<_object>();
        }
        return memory_item(self_ as *mut _object, index);
    } else {
        if PySlice_Check!(key) != 0 {
            CHECK_RESTRICTED!(self_);
            let mut sliced: *mut __SlateRecord_memoryobject_h_36 = unsafe { core::mem::zeroed() };
            sliced = mbuf_add_view(
                unsafe { (*self_).mbuf },
                view as *const __SlateRecord_pybuffer_h_33,
            ) as *mut __SlateRecord_memoryobject_h_36;
            if sliced.is_null() {
                return core::ptr::null_mut::<_object>();
            }
            if init_slice(
                unsafe { core::ptr::addr_of_mut!((*sliced).view) },
                key,
                0i32,
            ) < 0
            {
                Py_DECREF__2!(sliced);
                return core::ptr::null_mut::<_object>();
            }
            init_len(unsafe { core::ptr::addr_of_mut!((*sliced).view) });
            init_flags(sliced);
            return sliced as *mut _object;
        } else {
            if is_multiindex(key) != 0 {
                return memory_item_multi(self_, key);
            } else {
                if is_multislice(key) != 0 {
                    unsafe {
                        PyErr_SetString(
                            PyExc_NotImplementedError,
                            c"multi-dimensional slicing is not implemented".as_ptr(),
                        )
                    };
                    return core::ptr::null_mut::<_object>();
                }
            }
        }
    }
    unsafe { PyErr_SetString(PyExc_TypeError, c"memoryview: invalid slice key".as_ptr()) };
    core::ptr::null_mut::<_object>()
}

#[unsafe(link_section = ".text.slate_distinct.memoryobject.memory_ass_sub")]
extern "C-unwind" fn memory_ass_sub(
    _self: *mut _object,
    key: *mut _object,
    value: *mut _object,
) -> i32 {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut ptr: *mut i8 = unsafe { core::mem::zeroed() };
    let mut ret: i32 = unsafe { core::mem::zeroed() };
    let mut arrays: core::mem::MaybeUninit<__SlateAlign16<[i64; 3]>> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_3: *mut [i64; 3] = core::ptr::addr_of_mut!(arrays) as *mut [i64; 3]; // sliced view
    let mut dest: core::mem::MaybeUninit<__SlateRecord_pybuffer_h_33> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_4: *mut __SlateRecord_pybuffer_h_33 =
        core::ptr::addr_of_mut!(dest) as *mut __SlateRecord_pybuffer_h_33;
    let mut __v5: bool = unsafe { core::mem::zeroed() };
    let mut index: i64 = unsafe { core::mem::zeroed() };
    let mut __v7: bool = unsafe { core::mem::zeroed() };
    let mut __v8: bool = unsafe { core::mem::zeroed() };
    let mut ptr_1: *mut i8 = unsafe { core::mem::zeroed() };
    let mut fmt: *const i8 = unsafe { core::mem::zeroed() };
    let mut src: core::mem::MaybeUninit<__SlateRecord_pybuffer_h_33> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_11: *mut __SlateRecord_pybuffer_h_33 =
        core::ptr::addr_of_mut!(src) as *mut __SlateRecord_pybuffer_h_33;
    let mut view: *mut __SlateRecord_pybuffer_h_33 = unsafe { core::mem::zeroed() };
    let mut self_: *mut __SlateRecord_memoryobject_h_36 = unsafe { core::mem::zeroed() };
    unsafe {
        self_ = _self as *mut __SlateRecord_memoryobject_h_36;
        view = core::ptr::addr_of_mut!((*self_).view);
        CHECK_RELEASED_INT!(self_);
        fmt = adjust_fmt(view as *const __SlateRecord_pybuffer_h_33);
        if fmt.is_null() {
            return 1i32.wrapping_neg();
        } else {
            if (*view).readonly != 0 {
                PyErr_SetString(PyExc_TypeError, c"cannot modify read-only memory".as_ptr());
                return 1i32.wrapping_neg();
            } else {
                if value.is_null() {
                    PyErr_SetString(PyExc_TypeError, c"cannot delete memory".as_ptr());
                    return 1i32.wrapping_neg();
                } else {
                    if (*view).ndim == 0 {
                        if key == core::ptr::addr_of_mut!(_Py_EllipsisObject) {
                            __v8 = true;
                        } else {
                            if PyTuple_Check!(key) != 0 {
                                __v7 = PyTuple_GET_SIZE!(key) == 0;
                            } else {
                                __v7 = false;
                            }
                            __v8 = __v7;
                        }
                        if __v8 {
                            ptr_1 = (*view).buf as *mut i8;
                            return pack_single(self_, ptr_1, value, fmt);
                        } else {
                            PyErr_SetString(
                                PyExc_TypeError,
                                c"invalid indexing of 0-dim memory".as_ptr(),
                            );
                            return 1i32.wrapping_neg();
                        }
                    } else {
                        if _PyIndex_Check(key) != 0 {
                            if 1 < (*view).ndim {
                                PyErr_SetString(
                                    PyExc_NotImplementedError,
                                    c"sub-views are not implemented".as_ptr(),
                                );
                                return 1i32.wrapping_neg();
                            } else {
                                index = PyNumber_AsSsize_t(key, PyExc_IndexError);
                                if index == (1i32.wrapping_neg() as i64) {
                                    __v5 = !PyErr_Occurred().is_null();
                                } else {
                                    __v5 = false;
                                }
                                if __v5 {
                                    return 1i32.wrapping_neg();
                                } else {
                                    ptr_1 = ptr_from_index(
                                        view as *const __SlateRecord_pybuffer_h_33,
                                        index,
                                    );
                                    if ptr_1.is_null() {
                                        return 1i32.wrapping_neg();
                                    } else {
                                        return pack_single(self_, ptr_1, value, fmt);
                                    }
                                }
                            }
                        } else {
                            // one-dimensional: fast path
                            if PySlice_Check!(key) != 0 && (*view).ndim == 1 {
                                ret = 1i32.wrapping_neg();
                                // rvalue must be an exporter
                                if PyObject_GetBuffer(
                                    value,
                                    core::ptr::addr_of_mut!(*__slate_slot_11),
                                    284i32,
                                ) < 0
                                {
                                    return ret;
                                } else {
                                    *__slate_slot_4 = *view;
                                    (*__slate_slot_4).shape = ((*__slate_slot_3).as_mut_ptr()
                                        as *mut i64)
                                        .wrapping_offset(0 as isize);
                                    *(*__slate_slot_4).shape.wrapping_offset(0 as isize) =
                                        *unsafe { (*view).shape }.wrapping_offset(0 as isize);
                                    (*__slate_slot_4).strides = ((*__slate_slot_3).as_mut_ptr()
                                        as *mut i64)
                                        .wrapping_offset(1 as isize);
                                    *(*__slate_slot_4).strides.wrapping_offset(0 as isize) =
                                        *unsafe { (*view).strides }.wrapping_offset(0 as isize);
                                    if !unsafe { (*view).suboffsets }.is_null() {
                                        (*__slate_slot_4).suboffsets =
                                            ((*__slate_slot_3).as_mut_ptr() as *mut i64)
                                                .wrapping_offset(2 as isize);
                                        *(*__slate_slot_4).suboffsets.wrapping_offset(0 as isize) =
                                            *unsafe { (*view).suboffsets }
                                                .wrapping_offset(0 as isize);
                                    }
                                    if init_slice(
                                        core::ptr::addr_of_mut!(*__slate_slot_4),
                                        key,
                                        0i32,
                                    ) >= 0
                                    {
                                        (*__slate_slot_4).len = unsafe {
                                            *(*__slate_slot_4).shape.wrapping_offset(0 as isize)
                                        }
                                        .wrapping_mul((*__slate_slot_4).itemsize);
                                        ret = copy_single(
                                            self_,
                                            core::ptr::addr_of_mut!(*__slate_slot_4)
                                                as *const __SlateRecord_pybuffer_h_33,
                                            core::ptr::addr_of_mut!(*__slate_slot_11)
                                                as *const __SlateRecord_pybuffer_h_33,
                                        );
                                    }
                                    PyBuffer_Release(core::ptr::addr_of_mut!(*__slate_slot_11));
                                    return ret;
                                }
                            } else {
                                if is_multiindex(key) != 0 {
                                    if PyTuple_GET_SIZE!(key) < ((*view).ndim as i64) {
                                        PyErr_SetString(
                                            PyExc_NotImplementedError,
                                            c"sub-views are not implemented".as_ptr(),
                                        );
                                        return 1i32.wrapping_neg();
                                    } else {
                                        ptr = ptr_from_tuple(
                                            view as *const __SlateRecord_pybuffer_h_33,
                                            key,
                                        );
                                        if ptr.is_null() {
                                            return 1i32.wrapping_neg();
                                        } else {
                                            return pack_single(self_, ptr, value, fmt);
                                        }
                                    }
                                } else {
                                    if PySlice_Check!(key) != 0 {
                                        __v0 = true;
                                    } else {
                                        __v0 = is_multislice(key) != 0;
                                    }
                                    if __v0 {
                                        // Call memory_subscript() to produce a sliced lvalue, then copy
                                        // rvalue into lvalue. This is already implemented in _testbuffer.c.
                                        PyErr_SetString(PyExc_NotImplementedError, c"memoryview slice assignments are currently restricted to ndim = 1".as_ptr());
                                        return 1i32.wrapping_neg();
                                    } else {
                                        PyErr_SetString(
                                            PyExc_TypeError,
                                            c"memoryview: invalid slice key".as_ptr(),
                                        );
                                        return 1i32.wrapping_neg();
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[unsafe(link_section = ".text.slate_distinct.memoryobject.memory_length")]
extern "C-unwind" fn memory_length(_self: *mut _object) -> i64 {
    let self_: *mut __SlateRecord_memoryobject_h_36 = _self as *mut __SlateRecord_memoryobject_h_36;
    CHECK_RELEASED_INT__2!(self_);
    if (unsafe { (*self_).view.ndim }) == 0 {
        unsafe { PyErr_SetString(PyExc_TypeError, c"0-dim memory has no length".as_ptr()) };
        return 1i32.wrapping_neg() as i64;
    }
    unsafe { *unsafe { (*self_).view.shape }.wrapping_offset(0 as isize) }
}

/// As mapping
/// mp_length
/// mp_subscript
/// mp_ass_subscript
static mut memory_as_mapping: __SlateRecord_object_h_126 = __SlateRecord_object_h_126 {
    mp_length: Some(memory_length),
    mp_subscript: Some(memory_subscript),
    mp_ass_subscript: Some(memory_ass_sub),
};

/// As sequence
/// sq_length
/// sq_concat
/// sq_repeat
/// sq_item
static mut memory_as_sequence: __SlateRecord_object_h_120 = {
    let mut __t0: __SlateRecord_object_h_120 = unsafe { core::mem::zeroed() };
    __t0.sq_length = Some(memory_length);
    __t0.sq_concat = None;
    __t0.sq_repeat = None;
    __t0.sq_item = Some(memory_item);
    __t0
};

//                              Counting
// [clinic input]
// memoryview.count
//
//     value: object
//     /
//
// Count the number of occurrences of a value.
// [clinic start generated code]
/// [clinic end generated code: output=a15cb19311985063 input=e3036ce1ed7d1823]
fn memoryview_count_impl(
    self_: *mut __SlateRecord_memoryobject_h_36,
    value: *mut _object,
) -> *mut _object {
    let iter: *mut _object = unsafe { PyObject_GetIter(_PyObject_CAST__2!(self_)) };
    if iter.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let mut count: i64 = 0;
    let mut item: *mut _object = core::ptr::null_mut::<_object>();
    while (unsafe { PyIter_NextItem(iter, core::ptr::addr_of_mut!(item)) }) != 0 {
        if item.is_null() {
            Py_DECREF!(iter);
            return core::ptr::null_mut::<_object>();
        }
        if item == value {
            Py_DECREF!(item);
            count = count.wrapping_add(1); // no overflow since count <= len(mv) <= PY_SSIZE_T_MAX
        } else {
            let contained: i32 = unsafe { PyObject_RichCompareBool(item, value, 2i32) };
            Py_DECREF!(item);
            if contained > 0 {
                // more likely than 'contained < 0'
                count = count.wrapping_add(1); // no overflow since count <= len(mv) <= PY_SSIZE_T_MAX
            } else {
                if contained < 0 {
                    Py_DECREF!(iter);
                    return core::ptr::null_mut::<_object>();
                }
            }
        }
    }
    Py_DECREF!(iter);
    unsafe { PyLong_FromSsize_t(count) }
}

//                             Lookup
// [clinic input]
// memoryview.index
//
//     value: object
//     start: slice_index(accept={int}) = 0
//     stop: slice_index(accept={int}, c_default="PY_SSIZE_T_MAX") = sys.maxsize
//     /
//
// Return the index of the first occurrence of a value.
//
// Raises ValueError if the value is not present.
// [clinic start generated code]
/// [clinic end generated code: output=e0185e3819e549df input=0697a0165bf90b5a]
fn memoryview_index_impl(
    self_: *mut __SlateRecord_memoryobject_h_36,
    value: *mut _object,
    mut start: i64,
    mut stop: i64,
) -> *mut _object {
    let view: *const __SlateRecord_pybuffer_h_33 =
        (unsafe { core::ptr::addr_of_mut!((*self_).view) }) as *const __SlateRecord_pybuffer_h_33;
    CHECK_RELEASED!(self_);
    if (unsafe { (*view).ndim }) == 0 {
        unsafe { PyErr_SetString(PyExc_TypeError, c"invalid lookup on 0-dim memory".as_ptr()) };
        return core::ptr::null_mut::<_object>();
    }
    if (unsafe { (*view).ndim }) == 1 {
        let n: i64 = unsafe { *unsafe { (*view).shape }.wrapping_offset(0 as isize) };
        if start < 0 {
            start = Py_MAX__2!(start.wrapping_add(n), 0i32);
        }
        if stop < 0 {
            stop = Py_MAX__2!(stop.wrapping_add(n), 0i32);
        }
        stop = Py_MIN!(stop, n);
        start = Py_MIN!(start, stop);
        let obj: *mut _object = _PyObject_CAST__2!(self_);
        for index in start..stop {
            // Note: while memoryviews can be mutated during iterations
            //
            // when calling the == operator, their shape cannot. As such,
            //
            // it is safe to assume that the index remains valid for the
            //
            // entire loop.
            let item: *mut _object = memory_item(obj, index);
            if item.is_null() {
                return core::ptr::null_mut::<_object>();
            }
            if item == value {
                Py_DECREF!(item);
                return unsafe { PyLong_FromSsize_t(index) };
            }
            let contained: i32 = unsafe { PyObject_RichCompareBool(item, value, 2i32) };
            Py_DECREF!(item);
            if contained > 0 {
                // more likely than 'contained < 0'
                return unsafe { PyLong_FromSsize_t(index) };
            } else {
                if contained < 0 {
                    return core::ptr::null_mut::<_object>();
                }
            }
        }
        unsafe {
            PyErr_SetString(
                PyExc_ValueError,
                c"memoryview.index(x): x not found".as_ptr(),
            )
        };
        return core::ptr::null_mut::<_object>();
    }
    unsafe {
        PyErr_SetString(
            PyExc_NotImplementedError,
            c"multi-dimensional lookup is not implemented".as_ptr(),
        )
    };
    core::ptr::null_mut::<_object>()
}

//                             Comparisons
// exception
// not implemented
/// Translate a StructError to "not equal". Preserve other exceptions.
fn fix_struct_error_int() -> i32 {
    // XXX Cannot get at StructError directly?
    let __v0: bool = (unsafe { PyErr_ExceptionMatches(PyExc_ImportError) }) != 0
        || (unsafe { PyErr_ExceptionMatches(PyExc_MemoryError) }) != 0;
    if __v0 {
        return 1i32.wrapping_neg();
    }
    // StructError: invalid or unknown format -> not equal
    unsafe { PyErr_Clear() };
    0
}

/// Unpack and compare single items of p and q using the struct module.
fn struct_unpack_cmp(
    p: *const i8,
    q: *const i8,
    unpack_p: *mut unpacker,
    unpack_q: *mut unpacker,
) -> i32 {
    let mut ret: i32 = 0;
    // At this point any exception from the struct module should not be
    // StructError, since both formats have been accepted already.
    let v: *mut _object = struct_unpack_single(p, unpack_p);
    if v.is_null() {
        return 1i32.wrapping_neg();
    }
    let w: *mut _object = struct_unpack_single(q, unpack_q);
    if w.is_null() {
        Py_DECREF!(v);
        return 1i32.wrapping_neg();
    }
    // MV_COMPARE_EX == -1: exceptions are preserved
    ret = unsafe { PyObject_RichCompareBool(v, w, 2i32) };
    Py_DECREF!(v);
    Py_DECREF!(w);
    ret
}

fn unpack_cmp(
    p: *const i8,
    q: *const i8,
    fmt: i8,
    unpack_p: *mut unpacker,
    unpack_q: *mut unpacker,
) -> i32 {
    let mut equal: i32 = 0;
    match fmt as i32 {
        66 => {
            // signed integers and fast path for 'B'
            return (((unsafe { *(p as *const u8) }) as i32)
                == ((unsafe { *(q as *const u8) }) as i32)) as i32;
        }
        98 => {
            return (((unsafe { *(p as *const i8) }) as i32)
                == ((unsafe { *(q as *const i8) }) as i32)) as i32;
        }
        104 => {
            CMP_SINGLE__5!(p, q, i16, equal);
            return equal;
        }
        105 => {
            CMP_SINGLE__6!(p, q, i32, equal);
            return equal;
        }
        108 => {
            CMP_SINGLE__2!(p, q, i64, equal);
            return equal;
            // boolean
        }
        63 => {
            return ((UNPACK_TO_BOOL!(p) as i32) == (UNPACK_TO_BOOL!(q) as i32)) as i32;
            // unsigned integers
        }
        72 => {
            CMP_SINGLE__3!(p, q, u16, equal);
            return equal;
        }
        73 => {
            CMP_SINGLE__7!(p, q, u32, equal);
            return equal;
        }
        76 => {
            CMP_SINGLE!(p, q, u64, equal);
            return equal;
            // native 64-bit
        }
        113 => {
            CMP_SINGLE__2!(p, q, i64, equal);
            return equal;
        }
        81 => {
            CMP_SINGLE!(p, q, u64, equal);
            return equal;
            // ssize_t and size_t
        }
        110 => {
            CMP_SINGLE__2!(p, q, i64, equal);
            return equal;
        }
        78 => {
            CMP_SINGLE!(p, q, u64, equal);
            return equal;
            // floats
            //
            // XXX DBL_EPSILON?
        }
        102 => {
            CMP_SINGLE__4!(p, q, f32, equal);
            return equal;
        }
        100 => {
            CMP_SINGLE__8!(p, q, f64, equal);
            return equal;
        }
        101 => {
            let endian: i32 = 1;
            // Note: PyFloat_Unpack2 should never fail
            let u: f64 = unsafe { PyFloat_Unpack2(p, endian) };
            let v: f64 = unsafe { PyFloat_Unpack2(q, endian) };
            return (u == v) as i32;
            // bytes object
        }
        99 => {
            return (((unsafe { *p }) as i32) == ((unsafe { *q }) as i32)) as i32;
            // pointer
        }
        80 => {
            loop {
                let mut x: *mut () = unsafe { core::mem::zeroed() };
                let mut y: *mut () = unsafe { core::mem::zeroed() };
                unsafe {
                    core::ptr::copy_nonoverlapping(
                        ((p as *const ()) as *const core::ffi::c_void) as *const u8,
                        (((core::ptr::addr_of_mut!(x) as *mut i8) as *mut ())
                            as *mut core::ffi::c_void) as *mut u8,
                        ((core::mem::size_of::<*mut ()>() as u64) as usize) as usize,
                    )
                };
                unsafe {
                    core::ptr::copy_nonoverlapping(
                        ((q as *const ()) as *const core::ffi::c_void) as *const u8,
                        (((core::ptr::addr_of_mut!(y) as *mut i8) as *mut ())
                            as *mut core::ffi::c_void) as *mut u8,
                        ((core::mem::size_of::<*mut ()>() as u64) as usize) as usize,
                    )
                };
                equal = (x == y) as i32;
                if true {
                    break;
                }
            }
            return equal;
            // use the struct module
        }
        95 => {
            return struct_unpack_cmp(p, q, unpack_p, unpack_q);
        }
        _ => {}
    }
    // NOT REACHED
    unsafe {
        PyErr_SetString(
            PyExc_RuntimeError,
            c"memoryview: internal error in richcompare".as_ptr(),
        )
    };
    1i32.wrapping_neg()
}

/// Base case for recursive array comparisons. Assumption: ndim == 1.
fn cmp_base(
    mut p: *const i8,
    mut q: *const i8,
    shape: *const i64,
    pstrides: *const i64,
    mut psuboffsets: *const i64,
    qstrides: *const i64,
    mut qsuboffsets: *const i64,
    fmt: i8,
    unpack_p: *mut unpacker,
    unpack_q: *mut unpacker,
) -> i32 {
    let mut i: i64 = 0;
    let mut equal: i32 = 0;
    while i < unsafe { *shape.wrapping_offset(0 as isize) } {
        let xp: *const i8 = ADJUST_PTR!(p, psuboffsets, 0i32);
        let xq: *const i8 = ADJUST_PTR!(q, qsuboffsets, 0i32);
        equal = unpack_cmp(xp, xq, fmt, unpack_p, unpack_q);
        if equal <= 0 {
            return equal;
        }
        let __v1: *const i8 = p;
        p = __v1.wrapping_offset((unsafe { *pstrides.wrapping_offset(0 as isize) }) as isize);
        let __v3: *const i8 = q;
        q = __v3.wrapping_offset((unsafe { *qstrides.wrapping_offset(0 as isize) }) as isize);
        i = i.wrapping_add(1);
    }
    1
}

/// Recursively compare two multi-dimensional arrays that have the same
/// logical structure. Assumption: ndim >= 1.
fn cmp_rec(
    mut p: *const i8,
    mut q: *const i8,
    ndim: i64,
    shape: *const i64,
    pstrides: *const i64,
    mut psuboffsets: *const i64,
    qstrides: *const i64,
    mut qsuboffsets: *const i64,
    fmt: i8,
    unpack_p: *mut unpacker,
    unpack_q: *mut unpacker,
) -> i32 {
    let mut i: i64 = 0;
    let mut equal: i32 = 0;
    if ndim == 1 {
        return cmp_base(
            p,
            q,
            shape,
            pstrides,
            psuboffsets,
            qstrides,
            qsuboffsets,
            fmt,
            unpack_p,
            unpack_q,
        );
    }
    while i < unsafe { *shape.wrapping_offset(0 as isize) } {
        let xp: *const i8 = ADJUST_PTR!(p, psuboffsets, 0i32);
        let xq: *const i8 = ADJUST_PTR!(q, qsuboffsets, 0i32);
        equal = cmp_rec(
            xp,
            xq,
            ndim.wrapping_sub(1),
            shape.wrapping_offset(1 as isize),
            pstrides.wrapping_offset(1 as isize),
            if !psuboffsets.is_null() {
                psuboffsets.wrapping_offset(1 as isize)
            } else {
                core::ptr::null::<i64>()
            },
            qstrides.wrapping_offset(1 as isize),
            if !qsuboffsets.is_null() {
                qsuboffsets.wrapping_offset(1 as isize)
            } else {
                core::ptr::null::<i64>()
            },
            fmt,
            unpack_p,
            unpack_q,
        );
        if equal <= 0 {
            return equal;
        }
        let __v1: *const i8 = p;
        p = __v1.wrapping_offset((unsafe { *pstrides.wrapping_offset(0 as isize) }) as isize);
        let __v3: *const i8 = q;
        q = __v3.wrapping_offset((unsafe { *qstrides.wrapping_offset(0 as isize) }) as isize);
        i = i.wrapping_add(1);
    }
    1
}

#[unsafe(link_section = ".text.slate_distinct.memoryobject.memory_richcompare")]
extern "C-unwind" fn memory_richcompare(v: *mut _object, w: *mut _object, op: i32) -> *mut _object {
    let mut equal: i32 = unsafe { core::mem::zeroed() };
    let mut wfmt: i8 = unsafe { core::mem::zeroed() };
    let mut vfmt: i8 = unsafe { core::mem::zeroed() };
    let mut unpack_w: *mut unpacker = unsafe { core::mem::zeroed() };
    let mut unpack_v: *mut unpacker = unsafe { core::mem::zeroed() };
    let mut ww: *mut __SlateRecord_pybuffer_h_33 = unsafe { core::mem::zeroed() };
    let mut vv: *mut __SlateRecord_pybuffer_h_33 = unsafe { core::mem::zeroed() };
    let mut wbuf: core::mem::MaybeUninit<__SlateRecord_pybuffer_h_33> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_7: *mut __SlateRecord_pybuffer_h_33 =
        core::ptr::addr_of_mut!(wbuf) as *mut __SlateRecord_pybuffer_h_33;
    let mut res: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_10: {
            ww = core::ptr::null_mut::<__SlateRecord_pybuffer_h_33>();
            unpack_v = core::ptr::null_mut::<unpacker>();
            unpack_w = core::ptr::null_mut::<unpacker>();
            equal = 2i32.wrapping_neg();
            if !(op != 2 && op != 3) {
                // Py_NotImplemented
                if BASE_INACCESSIBLE__2!(v) {
                    equal = (v == w) as i32;
                } else {
                    vv = VIEW_ADDR__2!(v);
                    if PyMemoryView_Check!(w) != 0 {
                        if BASE_INACCESSIBLE__2!(w) {
                            equal = (v == w) as i32;
                            break '__join_10;
                        } else {
                            ww = VIEW_ADDR__2!(w);
                        }
                    } else {
                        if PyObject_GetBuffer(w, core::ptr::addr_of_mut!(*__slate_slot_7), 284i32)
                            < 0
                        {
                            PyErr_Clear();
                            break '__join_10;
                        } else {
                            ww = core::ptr::addr_of_mut!(*__slate_slot_7);
                        }
                    }
                    if equiv_shape(
                        vv as *const __SlateRecord_pybuffer_h_33,
                        ww as *const __SlateRecord_pybuffer_h_33,
                    ) == 0
                    {
                        PyErr_Clear();
                        equal = 0;
                    } else {
                        // Use fast unpacking for identical primitive C type formats.
                        if get_native_fmtchar(
                            &mut (*core::ptr::addr_of_mut!(vfmt)),
                            (*vv).format as *const i8,
                        ) < 0
                        {
                            vfmt = 95;
                        }
                        if get_native_fmtchar(
                            &mut (*core::ptr::addr_of_mut!(wfmt)),
                            (*ww).format as *const i8,
                        ) < 0
                        {
                            wfmt = 95;
                        }
                        if (vfmt as i32) == 95
                            || (wfmt as i32) == 95
                            || (vfmt as i32) != (wfmt as i32)
                        {
                            // Use struct module unpacking. NOTE: Even for equal format strings,
                            // memcmp() cannot be used for item comparison since it would give
                            // incorrect results in the case of NaNs or uninitialized padding
                            // bytes.
                            vfmt = 95;
                            unpack_v =
                                struct_get_unpacker((*vv).format as *const i8, (*vv).itemsize);
                            if unpack_v.is_null() {
                                equal = fix_struct_error_int();
                                break '__join_10;
                            } else {
                                unpack_w =
                                    struct_get_unpacker((*ww).format as *const i8, (*ww).itemsize);
                                if unpack_w.is_null() {
                                    equal = fix_struct_error_int();
                                    break '__join_10;
                                }
                            }
                        }
                        if (*vv).ndim == 0 {
                            equal = unpack_cmp(
                                (*vv).buf as *const i8,
                                (*ww).buf as *const i8,
                                vfmt,
                                unpack_v,
                                unpack_w,
                            );
                        } else {
                            if (*vv).ndim == 1 {
                                equal = cmp_base(
                                    (*vv).buf as *const i8,
                                    (*ww).buf as *const i8,
                                    (*vv).shape as *const i64,
                                    (*vv).strides as *const i64,
                                    (*vv).suboffsets as *const i64,
                                    (*ww).strides as *const i64,
                                    (*ww).suboffsets as *const i64,
                                    vfmt,
                                    unpack_v,
                                    unpack_w,
                                );
                            } else {
                                equal = cmp_rec(
                                    (*vv).buf as *const i8,
                                    (*ww).buf as *const i8,
                                    (*vv).ndim as i64,
                                    (*vv).shape as *const i64,
                                    (*vv).strides as *const i64,
                                    (*vv).suboffsets as *const i64,
                                    (*ww).strides as *const i64,
                                    (*ww).suboffsets as *const i64,
                                    vfmt,
                                    unpack_v,
                                    unpack_w,
                                );
                            }
                        }
                    }
                }
            }
        }
        if equal < 0 {
            if equal == 2i32.wrapping_neg() {
                res = core::ptr::addr_of_mut!(_Py_NotImplementedStruct);
            } else {
                res = core::ptr::null_mut::<_object>();
            }
        // exception
        } else {
            if equal != 0 && op == 2 || equal == 0 && op == 3 {
                res = _PyObject_CAST__2!(core::ptr::addr_of_mut!(_Py_TrueStruct));
            } else {
                res = _PyObject_CAST__2!(core::ptr::addr_of_mut!(_Py_FalseStruct));
            }
        }
        if ww == core::ptr::addr_of_mut!(*__slate_slot_7) {
            PyBuffer_Release(ww);
        }
        unpacker_free(unpack_v);
        unpacker_free(unpack_w);
        return Py_XNewRef!(res);
    }
    // Py_NotImplemented
}

//                                Hash
#[unsafe(link_section = ".text.slate_distinct.memoryobject.memory_hash")]
extern "C-unwind" fn memory_hash(_self: *mut _object) -> i64 {
    let self_: *mut __SlateRecord_memoryobject_h_36 = _self as *mut __SlateRecord_memoryobject_h_36;
    if (unsafe { (*self_).hash }) == (1i32.wrapping_neg() as i64) {
        let view: *mut __SlateRecord_pybuffer_h_33 =
            unsafe { core::ptr::addr_of_mut!((*self_).view) };
        let mut mem: *mut i8 = (unsafe { (*view).buf }) as *mut i8;
        let mut ret: i64 = 0;
        let mut fmt: i8 = 0;
        CHECK_RELEASED_INT__2!(self_);
        if (unsafe { (*view).readonly }) == 0 {
            unsafe {
                PyErr_SetString(
                    PyExc_ValueError,
                    c"cannot hash writable memoryview object".as_ptr(),
                )
            };
            return 1i32.wrapping_neg() as i64;
        }
        ret = get_native_fmtchar(
            unsafe { &mut (*core::ptr::addr_of_mut!(fmt)) },
            (unsafe { (*view).format }) as *const i8,
        );
        if ret < 0 || !IS_BYTE_FORMAT!(fmt) {
            unsafe {
                PyErr_SetString(
                    PyExc_ValueError,
                    c"memoryview: hashing is restricted to formats 'B', 'b' or 'c'".as_ptr(),
                )
            };
            return 1i32.wrapping_neg() as i64;
        }
        if !unsafe { (*view).obj }.is_null() {
            // Prevent 'self' from being freed when computing the item's hash.
            //
            // See https://github.com/python/cpython/issues/142664.
            let __v1: i64 = unsafe { (*self_).exports };
            unsafe {
                (*self_).exports = __v1.wrapping_add(1);
            }
            let h: i64 = unsafe { PyObject_Hash((*view).obj) };
            let __v4: i64 = unsafe { (*self_).exports };
            unsafe {
                (*self_).exports = __v4.wrapping_add(1i32.wrapping_neg() as i64);
            }
            if h == (1i32.wrapping_neg() as i64) {
                // Keep the original error message
                return 1i32.wrapping_neg() as i64;
            }
        }
        if MV_C_CONTIGUOUS!(unsafe { (*self_).flags }) == 0 {
            mem = (unsafe { PyMem_Malloc((*view).len as u64) }) as *mut i8;
            if mem.is_null() {
                unsafe { PyErr_NoMemory() };
                return 1i32.wrapping_neg() as i64;
            }
            if buffer_to_contiguous(mem, view as *const __SlateRecord_pybuffer_h_33, 67i8) < 0 {
                unsafe { PyMem_Free(mem as *mut ()) };
                return 1i32.wrapping_neg() as i64;
            }
        }
        // Can't fail
        unsafe {
            (*self_).hash = Py_HashBuffer(mem as *const (), (*view).len);
        }
        if mem != ((unsafe { (*view).buf }) as *mut i8) {
            unsafe { PyMem_Free(mem as *mut ()) };
        }
    }
    unsafe { (*self_).hash }
}

//                                 getters
fn _IntTupleFromSsizet(len: i32, vals: *mut i64) -> *mut _object {
    if vals.is_null() {
        return unsafe { PyTuple_New(0i64) };
    }
    let intTuple: *mut _object = unsafe { PyTuple_New(len as i64) };
    if intTuple.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    for i in 0..len {
        let o: *mut _object = unsafe { PyLong_FromSsize_t(*vals.wrapping_offset(i as isize)) };
        if o.is_null() {
            Py_DECREF!(intTuple);
            return core::ptr::null_mut::<_object>();
        }
        PyTuple_SET_ITEM!(intTuple, i, o);
    }
    intTuple
}

#[unsafe(link_section = ".text.slate_distinct.memoryobject.memory_obj_get")]
extern "C-unwind" fn memory_obj_get(_self: *mut _object, _unused_ignored: *mut ()) -> *mut _object {
    let self_: *mut __SlateRecord_memoryobject_h_36 = _self as *mut __SlateRecord_memoryobject_h_36;
    let view: *mut __SlateRecord_pybuffer_h_33 = unsafe { core::ptr::addr_of_mut!((*self_).view) };
    CHECK_RELEASED!(self_);
    if unsafe { (*view).obj }.is_null() {
        return unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) };
    }
    Py_NewRef!(unsafe { (*view).obj })
}

#[unsafe(link_section = ".text.slate_distinct.memoryobject.memory_nbytes_get")]
extern "C-unwind" fn memory_nbytes_get(
    _self: *mut _object,
    _unused_ignored: *mut (),
) -> *mut _object {
    let self_: *mut __SlateRecord_memoryobject_h_36 = _self as *mut __SlateRecord_memoryobject_h_36;
    CHECK_RELEASED!(self_);
    unsafe { PyLong_FromSsize_t((*self_).view.len) }
}

#[unsafe(link_section = ".text.slate_distinct.memoryobject.memory_format_get")]
extern "C-unwind" fn memory_format_get(
    _self: *mut _object,
    _unused_ignored: *mut (),
) -> *mut _object {
    let self_: *mut __SlateRecord_memoryobject_h_36 = _self as *mut __SlateRecord_memoryobject_h_36;
    CHECK_RELEASED!(self_);
    unsafe { PyUnicode_FromString((*self_).view.format as *const i8) }
}

#[unsafe(link_section = ".text.slate_distinct.memoryobject.memory_itemsize_get")]
extern "C-unwind" fn memory_itemsize_get(
    _self: *mut _object,
    _unused_ignored: *mut (),
) -> *mut _object {
    let self_: *mut __SlateRecord_memoryobject_h_36 = _self as *mut __SlateRecord_memoryobject_h_36;
    CHECK_RELEASED!(self_);
    unsafe { PyLong_FromSsize_t((*self_).view.itemsize) }
}

#[unsafe(link_section = ".text.slate_distinct.memoryobject.memory_shape_get")]
extern "C-unwind" fn memory_shape_get(
    _self: *mut _object,
    _unused_ignored: *mut (),
) -> *mut _object {
    let self_: *mut __SlateRecord_memoryobject_h_36 = _self as *mut __SlateRecord_memoryobject_h_36;
    CHECK_RELEASED!(self_);
    _IntTupleFromSsizet(unsafe { (*self_).view.ndim }, unsafe {
        (*self_).view.shape
    })
}

#[unsafe(link_section = ".text.slate_distinct.memoryobject.memory_strides_get")]
extern "C-unwind" fn memory_strides_get(
    _self: *mut _object,
    _unused_ignored: *mut (),
) -> *mut _object {
    let self_: *mut __SlateRecord_memoryobject_h_36 = _self as *mut __SlateRecord_memoryobject_h_36;
    CHECK_RELEASED!(self_);
    _IntTupleFromSsizet(unsafe { (*self_).view.ndim }, unsafe {
        (*self_).view.strides
    })
}

#[unsafe(link_section = ".text.slate_distinct.memoryobject.memory_suboffsets_get")]
extern "C-unwind" fn memory_suboffsets_get(
    _self: *mut _object,
    _unused_ignored: *mut (),
) -> *mut _object {
    let self_: *mut __SlateRecord_memoryobject_h_36 = _self as *mut __SlateRecord_memoryobject_h_36;
    CHECK_RELEASED!(self_);
    _IntTupleFromSsizet(unsafe { (*self_).view.ndim }, unsafe {
        (*self_).view.suboffsets
    })
}

#[unsafe(link_section = ".text.slate_distinct.memoryobject.memory_readonly_get")]
extern "C-unwind" fn memory_readonly_get(
    _self: *mut _object,
    _unused_ignored: *mut (),
) -> *mut _object {
    let self_: *mut __SlateRecord_memoryobject_h_36 = _self as *mut __SlateRecord_memoryobject_h_36;
    CHECK_RELEASED!(self_);
    unsafe { PyBool_FromLong((*self_).view.readonly as i64) }
}

#[unsafe(link_section = ".text.slate_distinct.memoryobject.memory_ndim_get")]
extern "C-unwind" fn memory_ndim_get(
    _self: *mut _object,
    _unused_ignored: *mut (),
) -> *mut _object {
    let self_: *mut __SlateRecord_memoryobject_h_36 = _self as *mut __SlateRecord_memoryobject_h_36;
    CHECK_RELEASED!(self_);
    unsafe { PyLong_FromLong((*self_).view.ndim as i64) }
}

#[unsafe(link_section = ".text.slate_distinct.memoryobject.memory_c_contiguous")]
extern "C-unwind" fn memory_c_contiguous(
    _self: *mut _object,
    _unused_ignored: *mut (),
) -> *mut _object {
    let self_: *mut __SlateRecord_memoryobject_h_36 = _self as *mut __SlateRecord_memoryobject_h_36;
    CHECK_RELEASED!(self_);
    unsafe { PyBool_FromLong(MV_C_CONTIGUOUS!(unsafe { (*self_).flags }) as i64) }
}

#[unsafe(link_section = ".text.slate_distinct.memoryobject.memory_f_contiguous")]
extern "C-unwind" fn memory_f_contiguous(
    _self: *mut _object,
    _unused_ignored: *mut (),
) -> *mut _object {
    let self_: *mut __SlateRecord_memoryobject_h_36 = _self as *mut __SlateRecord_memoryobject_h_36;
    CHECK_RELEASED!(self_);
    unsafe { PyBool_FromLong(MV_F_CONTIGUOUS!(unsafe { (*self_).flags }) as i64) }
}

#[unsafe(link_section = ".text.slate_distinct.memoryobject.memory_contiguous")]
extern "C-unwind" fn memory_contiguous(
    _self: *mut _object,
    _unused_ignored: *mut (),
) -> *mut _object {
    let self_: *mut __SlateRecord_memoryobject_h_36 = _self as *mut __SlateRecord_memoryobject_h_36;
    CHECK_RELEASED!(self_);
    unsafe { PyBool_FromLong(MV_ANY_CONTIGUOUS!(unsafe { (*self_).flags }) as i64) }
}

static memory_obj_doc: __SlateAlign16<[i8; 41]> = __SlateAlign16([
    84i8, 104i8, 101i8, 32i8, 117i8, 110i8, 100i8, 101i8, 114i8, 108i8, 121i8, 105i8, 110i8, 103i8,
    32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 32i8, 111i8, 102i8, 32i8, 116i8, 104i8, 101i8,
    32i8, 109i8, 101i8, 109i8, 111i8, 114i8, 121i8, 118i8, 105i8, 101i8, 119i8, 46i8, 0i8,
]);

static memory_nbytes_doc: __SlateAlign16<[i8; 87]> = __SlateAlign16([
    84i8, 104i8, 101i8, 32i8, 97i8, 109i8, 111i8, 117i8, 110i8, 116i8, 32i8, 111i8, 102i8, 32i8,
    115i8, 112i8, 97i8, 99i8, 101i8, 32i8, 105i8, 110i8, 32i8, 98i8, 121i8, 116i8, 101i8, 115i8,
    32i8, 116i8, 104i8, 97i8, 116i8, 32i8, 116i8, 104i8, 101i8, 32i8, 97i8, 114i8, 114i8, 97i8,
    121i8, 32i8, 119i8, 111i8, 117i8, 108i8, 100i8, 32i8, 117i8, 115i8, 101i8, 32i8, 105i8, 110i8,
    10i8, 32i8, 97i8, 32i8, 99i8, 111i8, 110i8, 116i8, 105i8, 103i8, 117i8, 111i8, 117i8, 115i8,
    32i8, 114i8, 101i8, 112i8, 114i8, 101i8, 115i8, 101i8, 110i8, 116i8, 97i8, 116i8, 105i8, 111i8,
    110i8, 46i8, 0i8,
]);

static memory_readonly_doc: __SlateAlign16<[i8; 51]> = __SlateAlign16([
    65i8, 32i8, 98i8, 111i8, 111i8, 108i8, 32i8, 105i8, 110i8, 100i8, 105i8, 99i8, 97i8, 116i8,
    105i8, 110i8, 103i8, 32i8, 119i8, 104i8, 101i8, 116i8, 104i8, 101i8, 114i8, 32i8, 116i8, 104i8,
    101i8, 32i8, 109i8, 101i8, 109i8, 111i8, 114i8, 121i8, 32i8, 105i8, 115i8, 32i8, 114i8, 101i8,
    97i8, 100i8, 32i8, 111i8, 110i8, 108i8, 121i8, 46i8, 0i8,
]);

static memory_itemsize_doc: __SlateAlign16<[i8; 53]> = __SlateAlign16([
    84i8, 104i8, 101i8, 32i8, 115i8, 105i8, 122i8, 101i8, 32i8, 105i8, 110i8, 32i8, 98i8, 121i8,
    116i8, 101i8, 115i8, 32i8, 111i8, 102i8, 32i8, 101i8, 97i8, 99i8, 104i8, 32i8, 101i8, 108i8,
    101i8, 109i8, 101i8, 110i8, 116i8, 32i8, 111i8, 102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 109i8,
    101i8, 109i8, 111i8, 114i8, 121i8, 118i8, 105i8, 101i8, 119i8, 46i8, 0i8,
]);

static memory_format_doc: __SlateAlign16<[i8; 87]> = __SlateAlign16([
    65i8, 32i8, 115i8, 116i8, 114i8, 105i8, 110i8, 103i8, 32i8, 99i8, 111i8, 110i8, 116i8, 97i8,
    105i8, 110i8, 105i8, 110i8, 103i8, 32i8, 116i8, 104i8, 101i8, 32i8, 102i8, 111i8, 114i8, 109i8,
    97i8, 116i8, 32i8, 40i8, 105i8, 110i8, 32i8, 115i8, 116i8, 114i8, 117i8, 99i8, 116i8, 32i8,
    109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 32i8, 115i8, 116i8, 121i8, 108i8, 101i8, 41i8, 10i8,
    32i8, 102i8, 111i8, 114i8, 32i8, 101i8, 97i8, 99i8, 104i8, 32i8, 101i8, 108i8, 101i8, 109i8,
    101i8, 110i8, 116i8, 32i8, 105i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 118i8, 105i8, 101i8,
    119i8, 46i8, 0i8,
]);

static memory_ndim_doc: __SlateAlign16<[i8; 95]> = __SlateAlign16([
    65i8, 110i8, 32i8, 105i8, 110i8, 116i8, 101i8, 103i8, 101i8, 114i8, 32i8, 105i8, 110i8, 100i8,
    105i8, 99i8, 97i8, 116i8, 105i8, 110i8, 103i8, 32i8, 104i8, 111i8, 119i8, 32i8, 109i8, 97i8,
    110i8, 121i8, 32i8, 100i8, 105i8, 109i8, 101i8, 110i8, 115i8, 105i8, 111i8, 110i8, 115i8, 32i8,
    111i8, 102i8, 32i8, 97i8, 32i8, 109i8, 117i8, 108i8, 116i8, 105i8, 45i8, 100i8, 105i8, 109i8,
    101i8, 110i8, 115i8, 105i8, 111i8, 110i8, 97i8, 108i8, 10i8, 32i8, 97i8, 114i8, 114i8, 97i8,
    121i8, 32i8, 116i8, 104i8, 101i8, 32i8, 109i8, 101i8, 109i8, 111i8, 114i8, 121i8, 32i8, 114i8,
    101i8, 112i8, 114i8, 101i8, 115i8, 101i8, 110i8, 116i8, 115i8, 46i8, 0i8,
]);

static memory_shape_doc: __SlateAlign16<[i8; 84]> = __SlateAlign16([
    65i8, 32i8, 116i8, 117i8, 112i8, 108i8, 101i8, 32i8, 111i8, 102i8, 32i8, 110i8, 100i8, 105i8,
    109i8, 32i8, 105i8, 110i8, 116i8, 101i8, 103i8, 101i8, 114i8, 115i8, 32i8, 103i8, 105i8, 118i8,
    105i8, 110i8, 103i8, 32i8, 116i8, 104i8, 101i8, 32i8, 115i8, 104i8, 97i8, 112i8, 101i8, 32i8,
    111i8, 102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 109i8, 101i8, 109i8, 111i8, 114i8, 121i8, 10i8,
    32i8, 97i8, 115i8, 32i8, 97i8, 110i8, 32i8, 78i8, 45i8, 100i8, 105i8, 109i8, 101i8, 110i8,
    115i8, 105i8, 111i8, 110i8, 97i8, 108i8, 32i8, 97i8, 114i8, 114i8, 97i8, 121i8, 46i8, 0i8,
]);

static memory_strides_doc: __SlateAlign16<[i8; 107]> = __SlateAlign16([
    65i8, 32i8, 116i8, 117i8, 112i8, 108i8, 101i8, 32i8, 111i8, 102i8, 32i8, 110i8, 100i8, 105i8,
    109i8, 32i8, 105i8, 110i8, 116i8, 101i8, 103i8, 101i8, 114i8, 115i8, 32i8, 103i8, 105i8, 118i8,
    105i8, 110i8, 103i8, 32i8, 116i8, 104i8, 101i8, 32i8, 115i8, 105i8, 122i8, 101i8, 32i8, 105i8,
    110i8, 32i8, 98i8, 121i8, 116i8, 101i8, 115i8, 32i8, 116i8, 111i8, 32i8, 97i8, 99i8, 99i8,
    101i8, 115i8, 115i8, 10i8, 32i8, 101i8, 97i8, 99i8, 104i8, 32i8, 101i8, 108i8, 101i8, 109i8,
    101i8, 110i8, 116i8, 32i8, 102i8, 111i8, 114i8, 32i8, 101i8, 97i8, 99i8, 104i8, 32i8, 100i8,
    105i8, 109i8, 101i8, 110i8, 115i8, 105i8, 111i8, 110i8, 32i8, 111i8, 102i8, 32i8, 116i8, 104i8,
    101i8, 32i8, 97i8, 114i8, 114i8, 97i8, 121i8, 46i8, 0i8,
]);

static memory_suboffsets_doc: __SlateAlign16<[i8; 58]> = __SlateAlign16([
    65i8, 32i8, 116i8, 117i8, 112i8, 108i8, 101i8, 32i8, 111i8, 102i8, 32i8, 105i8, 110i8, 116i8,
    101i8, 103i8, 101i8, 114i8, 115i8, 32i8, 117i8, 115i8, 101i8, 100i8, 32i8, 105i8, 110i8, 116i8,
    101i8, 114i8, 110i8, 97i8, 108i8, 108i8, 121i8, 32i8, 102i8, 111i8, 114i8, 32i8, 80i8, 73i8,
    76i8, 45i8, 115i8, 116i8, 121i8, 108i8, 101i8, 32i8, 97i8, 114i8, 114i8, 97i8, 121i8, 115i8,
    46i8, 0i8,
]);

static memory_c_contiguous_doc: __SlateAlign16<[i8; 54]> = __SlateAlign16([
    65i8, 32i8, 98i8, 111i8, 111i8, 108i8, 32i8, 105i8, 110i8, 100i8, 105i8, 99i8, 97i8, 116i8,
    105i8, 110i8, 103i8, 32i8, 119i8, 104i8, 101i8, 116i8, 104i8, 101i8, 114i8, 32i8, 116i8, 104i8,
    101i8, 32i8, 109i8, 101i8, 109i8, 111i8, 114i8, 121i8, 32i8, 105i8, 115i8, 32i8, 67i8, 32i8,
    99i8, 111i8, 110i8, 116i8, 105i8, 103i8, 117i8, 111i8, 117i8, 115i8, 46i8, 0i8,
]);

static memory_f_contiguous_doc: __SlateAlign16<[i8; 60]> = __SlateAlign16([
    65i8, 32i8, 98i8, 111i8, 111i8, 108i8, 32i8, 105i8, 110i8, 100i8, 105i8, 99i8, 97i8, 116i8,
    105i8, 110i8, 103i8, 32i8, 119i8, 104i8, 101i8, 116i8, 104i8, 101i8, 114i8, 32i8, 116i8, 104i8,
    101i8, 32i8, 109i8, 101i8, 109i8, 111i8, 114i8, 121i8, 32i8, 105i8, 115i8, 32i8, 70i8, 111i8,
    114i8, 116i8, 114i8, 97i8, 110i8, 32i8, 99i8, 111i8, 110i8, 116i8, 105i8, 103i8, 117i8, 111i8,
    117i8, 115i8, 46i8, 0i8,
]);

static memory_contiguous_doc: __SlateAlign16<[i8; 52]> = __SlateAlign16([
    65i8, 32i8, 98i8, 111i8, 111i8, 108i8, 32i8, 105i8, 110i8, 100i8, 105i8, 99i8, 97i8, 116i8,
    105i8, 110i8, 103i8, 32i8, 119i8, 104i8, 101i8, 116i8, 104i8, 101i8, 114i8, 32i8, 116i8, 104i8,
    101i8, 32i8, 109i8, 101i8, 109i8, 111i8, 114i8, 121i8, 32i8, 105i8, 115i8, 32i8, 99i8, 111i8,
    110i8, 116i8, 105i8, 103i8, 117i8, 111i8, 117i8, 115i8, 46i8, 0i8,
]);

static memory_exit_doc: __SlateAlign16<[i8; 98]> = __SlateAlign16([
    95i8, 95i8, 101i8, 120i8, 105i8, 116i8, 95i8, 95i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8,
    44i8, 32i8, 47i8, 44i8, 32i8, 42i8, 101i8, 120i8, 99i8, 95i8, 105i8, 110i8, 102i8, 111i8, 41i8,
    10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 108i8, 101i8, 97i8, 115i8, 101i8, 32i8, 116i8,
    104i8, 101i8, 32i8, 117i8, 110i8, 100i8, 101i8, 114i8, 108i8, 121i8, 105i8, 110i8, 103i8, 32i8,
    98i8, 117i8, 102i8, 102i8, 101i8, 114i8, 32i8, 101i8, 120i8, 112i8, 111i8, 115i8, 101i8, 100i8,
    32i8, 98i8, 121i8, 32i8, 116i8, 104i8, 101i8, 32i8, 109i8, 101i8, 109i8, 111i8, 114i8, 121i8,
    118i8, 105i8, 101i8, 119i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 46i8, 0i8,
]);

static mut memory_getsetlist: __SlateAlign16<[PyGetSetDef; 13]> = __SlateAlign16([
    {
        let mut __t0: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t0.name = (b"obj\0".as_ptr() as *mut i8) as *const i8;
        __t0.get = Some(memory_obj_get);
        __t0.set = None;
        __t0.doc = unsafe { core::ptr::addr_of!(memory_obj_doc.0) as *const i8 };
        __t0
    },
    {
        let mut __t1: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t1.name = (b"nbytes\0".as_ptr() as *mut i8) as *const i8;
        __t1.get = Some(memory_nbytes_get);
        __t1.set = None;
        __t1.doc = unsafe { core::ptr::addr_of!(memory_nbytes_doc.0) as *const i8 };
        __t1
    },
    {
        let mut __t2: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t2.name = (b"readonly\0".as_ptr() as *mut i8) as *const i8;
        __t2.get = Some(memory_readonly_get);
        __t2.set = None;
        __t2.doc = unsafe { core::ptr::addr_of!(memory_readonly_doc.0) as *const i8 };
        __t2
    },
    {
        let mut __t3: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t3.name = (b"itemsize\0".as_ptr() as *mut i8) as *const i8;
        __t3.get = Some(memory_itemsize_get);
        __t3.set = None;
        __t3.doc = unsafe { core::ptr::addr_of!(memory_itemsize_doc.0) as *const i8 };
        __t3
    },
    {
        let mut __t4: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t4.name = (b"format\0".as_ptr() as *mut i8) as *const i8;
        __t4.get = Some(memory_format_get);
        __t4.set = None;
        __t4.doc = unsafe { core::ptr::addr_of!(memory_format_doc.0) as *const i8 };
        __t4
    },
    {
        let mut __t5: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t5.name = (b"ndim\0".as_ptr() as *mut i8) as *const i8;
        __t5.get = Some(memory_ndim_get);
        __t5.set = None;
        __t5.doc = unsafe { core::ptr::addr_of!(memory_ndim_doc.0) as *const i8 };
        __t5
    },
    {
        let mut __t6: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t6.name = (b"shape\0".as_ptr() as *mut i8) as *const i8;
        __t6.get = Some(memory_shape_get);
        __t6.set = None;
        __t6.doc = unsafe { core::ptr::addr_of!(memory_shape_doc.0) as *const i8 };
        __t6
    },
    {
        let mut __t7: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t7.name = (b"strides\0".as_ptr() as *mut i8) as *const i8;
        __t7.get = Some(memory_strides_get);
        __t7.set = None;
        __t7.doc = unsafe { core::ptr::addr_of!(memory_strides_doc.0) as *const i8 };
        __t7
    },
    {
        let mut __t8: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t8.name = (b"suboffsets\0".as_ptr() as *mut i8) as *const i8;
        __t8.get = Some(memory_suboffsets_get);
        __t8.set = None;
        __t8.doc = unsafe { core::ptr::addr_of!(memory_suboffsets_doc.0) as *const i8 };
        __t8
    },
    {
        let mut __t9: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t9.name = (b"c_contiguous\0".as_ptr() as *mut i8) as *const i8;
        __t9.get = Some(memory_c_contiguous);
        __t9.set = None;
        __t9.doc = unsafe { core::ptr::addr_of!(memory_c_contiguous_doc.0) as *const i8 };
        __t9
    },
    {
        let mut __t10: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t10.name = (b"f_contiguous\0".as_ptr() as *mut i8) as *const i8;
        __t10.get = Some(memory_f_contiguous);
        __t10.set = None;
        __t10.doc = unsafe { core::ptr::addr_of!(memory_f_contiguous_doc.0) as *const i8 };
        __t10
    },
    {
        let mut __t11: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t11.name = (b"contiguous\0".as_ptr() as *mut i8) as *const i8;
        __t11.get = Some(memory_contiguous);
        __t11.set = None;
        __t11.doc = unsafe { core::ptr::addr_of!(memory_contiguous_doc.0) as *const i8 };
        __t11
    },
    {
        let mut __t12: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t12.name = core::ptr::null::<i8>();
        __t12.get = None;
        __t12.set = None;
        __t12.doc = core::ptr::null::<i8>();
        __t12
    },
]);

static mut memory_methods: __SlateAlign16<[PyMethodDef; 13]> = __SlateAlign16([
    {
        let mut __t0: PyMethodDef = unsafe { core::mem::zeroed() };
        __t0.ml_name = (b"release\0".as_ptr() as *mut i8) as *const i8;
        __t0.ml_meth = Some(memoryview_release);
        __t0.ml_flags = 4i32;
        __t0.ml_doc = unsafe { core::ptr::addr_of!(memoryview_release__doc__.0) as *const i8 };
        __t0
    },
    {
        let mut __t1: PyMethodDef = unsafe { core::mem::zeroed() };
        __t1.ml_name = (b"tobytes\0".as_ptr() as *mut i8) as *const i8;
        __t1.ml_meth = unsafe {
            core::mem::transmute::<
                Option<unsafe extern "C-unwind" fn()>,
                Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
            >(unsafe {
                core::mem::transmute::<
                    Option<
                        unsafe extern "C-unwind" fn(
                            *mut _object,
                            *const *mut _object,
                            i64,
                            *mut _object,
                        ) -> *mut _object,
                    >,
                    Option<unsafe extern "C-unwind" fn()>,
                >(Some(memoryview_tobytes))
            })
        };
        __t1.ml_flags = 130i32;
        __t1.ml_doc = unsafe { core::ptr::addr_of!(memoryview_tobytes__doc__.0) as *const i8 };
        __t1
    },
    {
        let mut __t2: PyMethodDef = unsafe { core::mem::zeroed() };
        __t2.ml_name = (b"hex\0".as_ptr() as *mut i8) as *const i8;
        __t2.ml_meth = unsafe {
            core::mem::transmute::<
                Option<unsafe extern "C-unwind" fn()>,
                Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
            >(unsafe {
                core::mem::transmute::<
                    Option<
                        unsafe extern "C-unwind" fn(
                            *mut _object,
                            *const *mut _object,
                            i64,
                            *mut _object,
                        ) -> *mut _object,
                    >,
                    Option<unsafe extern "C-unwind" fn()>,
                >(Some(memoryview_hex))
            })
        };
        __t2.ml_flags = 130i32;
        __t2.ml_doc = unsafe { core::ptr::addr_of!(memoryview_hex__doc__.0) as *const i8 };
        __t2
    },
    {
        let mut __t3: PyMethodDef = unsafe { core::mem::zeroed() };
        __t3.ml_name = (b"tolist\0".as_ptr() as *mut i8) as *const i8;
        __t3.ml_meth = Some(memoryview_tolist);
        __t3.ml_flags = 4i32;
        __t3.ml_doc = unsafe { core::ptr::addr_of!(memoryview_tolist__doc__.0) as *const i8 };
        __t3
    },
    {
        let mut __t4: PyMethodDef = unsafe { core::mem::zeroed() };
        __t4.ml_name = (b"cast\0".as_ptr() as *mut i8) as *const i8;
        __t4.ml_meth = unsafe {
            core::mem::transmute::<
                Option<unsafe extern "C-unwind" fn()>,
                Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
            >(unsafe {
                core::mem::transmute::<
                    Option<
                        unsafe extern "C-unwind" fn(
                            *mut _object,
                            *const *mut _object,
                            i64,
                            *mut _object,
                        ) -> *mut _object,
                    >,
                    Option<unsafe extern "C-unwind" fn()>,
                >(Some(memoryview_cast))
            })
        };
        __t4.ml_flags = 130i32;
        __t4.ml_doc = unsafe { core::ptr::addr_of!(memoryview_cast__doc__.0) as *const i8 };
        __t4
    },
    {
        let mut __t5: PyMethodDef = unsafe { core::mem::zeroed() };
        __t5.ml_name = (b"toreadonly\0".as_ptr() as *mut i8) as *const i8;
        __t5.ml_meth = Some(memoryview_toreadonly);
        __t5.ml_flags = 4i32;
        __t5.ml_doc = unsafe { core::ptr::addr_of!(memoryview_toreadonly__doc__.0) as *const i8 };
        __t5
    },
    {
        let mut __t6: PyMethodDef = unsafe { core::mem::zeroed() };
        __t6.ml_name = (b"_from_flags\0".as_ptr() as *mut i8) as *const i8;
        __t6.ml_meth = unsafe {
            core::mem::transmute::<
                Option<unsafe extern "C-unwind" fn()>,
                Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
            >(unsafe {
                core::mem::transmute::<
                    Option<
                        unsafe extern "C-unwind" fn(
                            *mut _object,
                            *const *mut _object,
                            i64,
                            *mut _object,
                        ) -> *mut _object,
                    >,
                    Option<unsafe extern "C-unwind" fn()>,
                >(Some(memoryview__from_flags))
            })
        };
        __t6.ml_flags = 146i32;
        __t6.ml_doc = unsafe { core::ptr::addr_of!(memoryview__from_flags__doc__.0) as *const i8 };
        __t6
    },
    {
        let mut __t7: PyMethodDef = unsafe { core::mem::zeroed() };
        __t7.ml_name = (b"count\0".as_ptr() as *mut i8) as *const i8;
        __t7.ml_meth = Some(memoryview_count);
        __t7.ml_flags = 8i32;
        __t7.ml_doc = unsafe { core::ptr::addr_of!(memoryview_count__doc__.0) as *const i8 };
        __t7
    },
    {
        let mut __t8: PyMethodDef = unsafe { core::mem::zeroed() };
        __t8.ml_name = (b"index\0".as_ptr() as *mut i8) as *const i8;
        __t8.ml_meth = unsafe {
            core::mem::transmute::<
                Option<unsafe extern "C-unwind" fn()>,
                Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
            >(unsafe {
                core::mem::transmute::<
                    Option<
                        unsafe extern "C-unwind" fn(
                            *mut _object,
                            *const *mut _object,
                            i64,
                        ) -> *mut _object,
                    >,
                    Option<unsafe extern "C-unwind" fn()>,
                >(Some(memoryview_index))
            })
        };
        __t8.ml_flags = 128i32;
        __t8.ml_doc = unsafe { core::ptr::addr_of!(memoryview_index__doc__.0) as *const i8 };
        __t8
    },
    {
        let mut __t9: PyMethodDef = unsafe { core::mem::zeroed() };
        __t9.ml_name = (b"__enter__\0".as_ptr() as *mut i8) as *const i8;
        __t9.ml_meth = Some(memory_enter);
        __t9.ml_flags = 4i32;
        __t9.ml_doc = core::ptr::null::<i8>();
        __t9
    },
    {
        let mut __t10: PyMethodDef = unsafe { core::mem::zeroed() };
        __t10.ml_name = (b"__exit__\0".as_ptr() as *mut i8) as *const i8;
        __t10.ml_meth = Some(memory_exit);
        __t10.ml_flags = 1i32;
        __t10.ml_doc = unsafe { core::ptr::addr_of!(memory_exit_doc.0) as *const i8 };
        __t10
    },
    {
        let mut __t11: PyMethodDef = unsafe { core::mem::zeroed() };
        __t11.ml_name = (b"__class_getitem__\0".as_ptr() as *mut i8) as *const i8;
        __t11.ml_meth = unsafe {
            core::mem::transmute::<
                *const (),
                Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
            >(Py_GenericAlias as *const ())
        };
        __t11.ml_flags = 24i32;
        __t11.ml_doc = (b"memoryviews are generic over the type of their underlying data\0".as_ptr()
            as *mut i8) as *const i8;
        __t11
    },
    {
        let mut __t12: PyMethodDef = unsafe { core::mem::zeroed() };
        __t12.ml_name = core::ptr::null::<i8>();
        __t12.ml_meth = None;
        __t12
    },
]);

//                          Memoryview Iterator
#[unsafe(no_mangle)]
static mut _PyMemoryIter_Type: _typeobject = {
    let mut __t0: _typeobject = unsafe { core::mem::zeroed() };
    __t0.ob_base = __SlateRecord_object_h_172 {
        ob_base: _object {
            __slate_anon_0: __SlateRecord_object_h_122 {
                ob_refcnt_full: 1407378104778752i64,
            },
            ob_type: unsafe { core::ptr::addr_of_mut!(PyType_Type) },
        },
        ob_size: 0i64,
    };
    __t0.tp_name = (b"memory_iterator\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_memoryobject_c_3472>() as u64) as i64;
    __t0.tp_dealloc = Some(memoryiter_dealloc);
    __t0.tp_getattro = unsafe {
        core::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
        >(PyObject_GenericGetAttr as *const ())
    };
    __t0.tp_flags = 16384u64;
    __t0.tp_traverse = Some(memoryiter_traverse);
    __t0.tp_iter = unsafe {
        core::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
        >(PyObject_SelfIter as *const ())
    };
    __t0.tp_iternext = Some(memoryiter_next);
    __t0
};

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_memoryobject_c_3472 {
    ob_base: _object,
    it_index: i64,
    /// Set to NULL when iterator is exhausted
    it_seq: *mut __SlateRecord_memoryobject_h_36,
    it_length: i64,
    it_fmt: *const i8,
}

#[unsafe(link_section = ".text.slate_distinct.memoryobject.memoryiter_dealloc")]
extern "C-unwind" fn memoryiter_dealloc(self_: *mut _object) {
    let it: *mut __SlateRecord_memoryobject_c_3472 =
        self_ as *mut __SlateRecord_memoryobject_c_3472;
    _PyObject_GC_UNTRACK__2!(it);
    Py_XDECREF__2!(unsafe { (*it).it_seq });
    unsafe { PyObject_GC_Del(it as *mut ()) };
}

#[unsafe(link_section = ".text.slate_distinct.memoryobject.memoryiter_traverse")]
extern "C-unwind" fn memoryiter_traverse(
    self_: *mut _object,
    mut visit: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
    arg: *mut (),
) -> i32 {
    let it: *mut __SlateRecord_memoryobject_c_3472 =
        self_ as *mut __SlateRecord_memoryobject_c_3472;
    Py_VISIT__16!(unsafe { (*it).it_seq }, visit, arg);
    0
}

#[unsafe(link_section = ".text.slate_distinct.memoryobject.memoryiter_next")]
extern "C-unwind" fn memoryiter_next(self_: *mut _object) -> *mut _object {
    let mut seq: *mut __SlateRecord_memoryobject_h_36 = unsafe { core::mem::zeroed() };
    seq = unsafe { (*(self_ as *mut __SlateRecord_memoryobject_c_3472)).it_seq };
    if seq.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    if (unsafe { (*(self_ as *mut __SlateRecord_memoryobject_c_3472)).it_index })
        < unsafe { (*(self_ as *mut __SlateRecord_memoryobject_c_3472)).it_length }
    {
        CHECK_RELEASED!(seq);
        let view: *mut __SlateRecord_pybuffer_h_33 =
            unsafe { core::ptr::addr_of_mut!((*seq).view) };
        let mut ptr: *mut i8 = (unsafe { (*seq).view.buf }) as *mut i8;
        let __v1: i64 = unsafe { (*(self_ as *mut __SlateRecord_memoryobject_c_3472)).it_index };
        unsafe {
            (*(self_ as *mut __SlateRecord_memoryobject_c_3472)).it_index = __v1.wrapping_add(1);
        }
        let __v3: i64 =
            unsafe { *unsafe { (*view).strides }.wrapping_offset(0 as isize) }.wrapping_mul(__v1);
        let __v4: *mut i8 = ptr;
        ptr = __v4.wrapping_offset(__v3 as isize);
        ptr = ADJUST_PTR__3!(ptr, (*view).suboffsets, 0i32);
        if ptr.is_null() {
            return core::ptr::null_mut::<_object>();
        }
        return unpack_single(seq, ptr as *const i8, unsafe {
            (*(self_ as *mut __SlateRecord_memoryobject_c_3472)).it_fmt
        });
    }
    unsafe {
        (*(self_ as *mut __SlateRecord_memoryobject_c_3472)).it_seq =
            core::ptr::null_mut::<__SlateRecord_memoryobject_h_36>();
    }
    Py_DECREF__2!(seq);
    core::ptr::null_mut::<_object>()
}

#[unsafe(link_section = ".text.slate_distinct.memoryobject.memory_iter")]
extern "C-unwind" fn memory_iter(seq: *mut _object) -> *mut _object {
    if PyMemoryView_Check!(seq) == 0 {
        PyErr_BadInternalCall__56!();
        return core::ptr::null_mut::<_object>();
    }
    CHECK_RELEASED__2!(seq);
    let obj: *mut __SlateRecord_memoryobject_h_36 = seq as *mut __SlateRecord_memoryobject_h_36;
    let ndims: i32 = unsafe { (*obj).view.ndim };
    if ndims == 0 {
        unsafe {
            PyErr_SetString(
                PyExc_TypeError,
                c"invalid indexing of 0-dim memory".as_ptr(),
            )
        };
        return core::ptr::null_mut::<_object>();
    }
    if ndims != 1 {
        unsafe {
            PyErr_SetString(
                PyExc_NotImplementedError,
                c"multi-dimensional sub-views are not implemented".as_ptr(),
            )
        };
        return core::ptr::null_mut::<_object>();
    }
    let fmt: *const i8 = adjust_fmt(
        (unsafe { core::ptr::addr_of_mut!((*obj).view) }) as *const __SlateRecord_pybuffer_h_33,
    );
    if fmt.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let it: *mut __SlateRecord_memoryobject_c_3472 =
        PyObject_GC_New!(__SlateRecord_memoryobject_c_3472, unsafe {
            core::ptr::addr_of_mut!(_PyMemoryIter_Type)
        });
    if it.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    unsafe {
        (*it).it_fmt = fmt;
    }
    unsafe {
        (*it).it_length = memory_length(obj as *mut _object);
    }
    unsafe {
        (*it).it_index = 0;
    }
    unsafe {
        (*it).it_seq = Py_NewRef__2!(obj) as *mut __SlateRecord_memoryobject_h_36;
    }
    _PyObject_GC_TRACK!(it);
    it as *mut _object
}

// methods
// tp_name
// tp_basicsize
// tp_itemsize
// tp_dealloc
// tp_vectorcall_offset
// tp_getattr
// tp_setattr
// tp_as_async
// tp_repr
// tp_as_number
// tp_as_sequence
// tp_as_mapping
// tp_hash
// tp_call
// tp_str
// tp_getattro
// tp_setattro
// tp_as_buffer
// tp_flags
// tp_doc
// tp_traverse
// tp_clear
// tp_richcompare
// tp_weaklistoffset
// tp_iter
// tp_iternext
// tp_methods
// tp_members
// tp_getset
// tp_base
// tp_dict
// tp_descr_get
// tp_descr_set
// tp_dictoffset
// tp_init
// tp_alloc
// tp_new
