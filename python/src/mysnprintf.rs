//! The size of 'void *', as computed by sizeof.
//! Macro to use the more powerful/dangerous C-style cast even in C++.
//! Round up size "n" to be a multiple of "a".
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
//! Macros and static inline functions, trading safety for speed
//! PyUnicode_KIND(): Return one of the PyUnicode_*_KIND values defined above.
//!
//!
//!
//! gh-89653: Converting this macro to a static inline function would introduce
//!
//! new compiler warnings on "kind < PyUnicode_KIND(str)" (compare signed and
//!
//! unsigned numbers) where kind type is an int or on
//!
//! "unsigned int kind = PyUnicode_KIND(str)" (cast signed to unsigned).
//! PyException_HEAD defines the initial segment of every exception class.
//! Cast argument to PyTupleObject* type.
//! Cast argument to PyListObject* type.
//! A code object, the __code__ attribute
//! To avoid repeating ourselves in deepfreeze.py, all PyCodeObject members are
//!
//! defined in this macro:
//! Remote debugger support
//! PY_TIMEOUT_T is the integral type used to specify timeouts when waiting
//!
//! on a lock (see PyThread_acquire_lock_timed() below).
//! snprintf() and vsnprintf() wrappers.
//!
//! If the platform has vsnprintf, we use it, else we
//! emulate it in a half-hearted way.  Even if the platform has it, we wrap
//! it because platforms differ in what vsnprintf does in case the buffer
//! is too small:  C99 behavior is to return the number of characters that
//! would have been written had the buffer not been too small, and to set
//! the last byte of the buffer to \0.  At least MS _vsnprintf returns a
//! negative value instead, and fills the entire buffer with non-\0 data.
//! Unlike C99, our wrappers do not support passing a null buffer.
//!
//! The wrappers ensure that str[size-1] is always \0 upon return.
//!
//! PyOS_snprintf and PyOS_vsnprintf never write more than size bytes
//! (including the trailing '\0') into str.
//!
//! Return value (rv):
//!
//!  When 0 <= rv < size, the output conversion was unexceptional, and
//!  rv characters were written to str (excluding a trailing \0 byte at
//!  str[rv]).
//!
//!  When rv >= size, output conversion was truncated, and a buffer of
//!  size rv+1 would have been needed to avoid truncation.  str[size-1]
//!  is \0 in this case.
//!
//!  When rv < 0, "something bad happened".  str[size-1] is \0 in this
//!  case too, but the rest of str is unreliable.  It could be that
//!  an error in format codes was detected by libc, or on platforms
//!  with a non-C99 vsnprintf simply that the buffer wasn't big enough
//!  to avoid truncation, or on platforms without any vsnprintf that
//!  PyMem_Malloc couldn't obtain space for a temp buffer.
//!
//! CAUTION:  Unlike C99, str != NULL and size > 0 are required.
//! Also, size must be smaller than INT_MAX.
// Signed variants of the above
// Prevent using an expression as a l-value.
//
// For example, "int x; _Py_RVALUE(x) = 1;" fails with a compiler error.
// PyObject_VAR_HEAD defines the initial segment of all variable-size
// container objects.  These end with a declaration of an array with 1
// element, but enough space is malloc'ed so that the array actually
// has room for ob_size elements.  Note that ob_size is an element count,
// not necessarily a byte count.
// Cast argument to PyObject* type.
// Cast argument to PyVarObject* type.
// NULL or a tuple
// NULL or a dict
// NULL or a tuple of cell objects
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
unsafe extern "C" {
    fn vsnprintf(
        arg0: *mut i8,
        arg1: u64,
        arg2: *const i8,
        arg3: core::ffi::VaList<'static>,
    ) -> i32;
}

#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn PyOS_snprintf(
    str: *mut i8,
    size: u64,
    format: *const i8,
    mut __va_args: ...
) -> i32 {
    let mut rc: i32 = 0;
    let mut va: core::ffi::VaList<'static> = unsafe { core::mem::zeroed() };
    va = unsafe {
        core::mem::transmute::<core::ffi::VaList<'_>, core::ffi::VaList<'static>>(__va_args.clone())
    };
    rc = PyOS_vsnprintf(str, size, format, va.clone());
    rc
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyOS_vsnprintf(
    str: *mut i8,
    size: u64,
    format: *const i8,
    mut va: core::ffi::VaList<'static>,
) -> i32 {
    // # bytes written, excluding \0
    let mut len: i32 = unsafe { core::mem::zeroed() };
    unsafe {
        // We take a size_t as input but return an int.  Sanity check
        // our input so that it won't cause an overflow in the
        // vsnprintf return value.
        if size > 2147483646u64 {
            len = 666i32.wrapping_neg();
        } else {
            len = vsnprintf(str, size, format, va.clone());
        }
        if size > 0 {
            *str.wrapping_offset(size.wrapping_sub(1) as isize) = 0;
        }
        return len;
    }
}
