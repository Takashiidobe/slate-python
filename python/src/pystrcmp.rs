//! Cross platform case insensitive string compare functions
//! The size of 'void *', as computed by sizeof.
//! Macro to use the more powerful/dangerous C-style cast even in C++.
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
// Signed variants of the above
// Round up size "n" to be a multiple of "a".
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
/// Argument must be a char or an int in [-128, 127] or [0, 255].
macro_rules! Py_CHARMASK {
    ($c:expr) => {
        ((($c) as i32) & 255) as u8
    };
}

macro_rules! Py_TOLOWER {
    ($c:expr) => {
        unsafe { _Py_ctype_tolower[(Py_CHARMASK!($c) as i32) as usize] }
    };
}

unsafe extern "C" {
    static mut _Py_ctype_tolower: [u8; 256];
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.pystrcmp.PyOS_mystrnicmp")]
extern "C-unwind" fn PyOS_mystrnicmp(s1: *const i8, s2: *const i8, mut size: i64) -> i32 {
    let mut p1: *const u8 = unsafe { core::mem::zeroed() };
    let mut p2: *const u8 = unsafe { core::mem::zeroed() };
    if size == 0 {
        return 0;
    }
    p1 = s1 as *const u8;
    p2 = s2 as *const u8;
    loop {
        let __v1: i64 = size.wrapping_sub(1);
        size = __v1;
        if !(__v1 > 0
            && (unsafe { *p1 }) != 0
            && (unsafe { *p2 }) != 0
            && (Py_TOLOWER!(unsafe { *p1 }) as i32) == (Py_TOLOWER!(unsafe { *p2 }) as i32))
        {
            break;
        }
        let __v3: *const u8 = p1;
        p1 = __v3.wrapping_offset(1 as isize);
        let __v5: *const u8 = p2;
        p2 = __v5.wrapping_offset(1 as isize);
    }
    (Py_TOLOWER!(unsafe { *p1 }) as i32).wrapping_sub(Py_TOLOWER!(unsafe { *p2 }) as i32)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyOS_mystricmp(s1: *const i8, s2: *const i8) -> i32 {
    let mut p1: *const u8 = s1 as *const u8;
    let mut p2: *const u8 = s2 as *const u8;
    while (unsafe { *p1 }) != 0
        && (unsafe { *p2 }) != 0
        && (Py_TOLOWER!(unsafe { *p1 }) as i32) == (Py_TOLOWER!(unsafe { *p2 }) as i32)
    {
        let __v1: *const u8 = p1;
        p1 = __v1.wrapping_offset(1 as isize);
        let __v3: *const u8 = p2;
        p2 = __v3.wrapping_offset(1 as isize);
    }
    (Py_TOLOWER!(unsafe { *p1 }) as i32).wrapping_sub(Py_TOLOWER!(unsafe { *p2 }) as i32)
}
