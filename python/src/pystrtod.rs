//! -*- Mode: C; c-file-style: "python" -*-
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
//! PyOS_double_to_string's "flags" parameter can be set to 0 or more of:
// Signed variants of the above
// Round up size "n" to be a multiple of "a".
// Prevent using an expression as a l-value.
//
// For example, "int x; _Py_RVALUE(x) = 1;" fails with a compiler error.
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
// always add the sign
// if the result is an integer add ".0"
// "alternate" formatting. it's format_code
// specific
// negative zero result is coerced to 0
// PyOS_double_to_string's "type", if non-NULL, will be set to one of:
/// Mask the old API with a call to the new API for code compiled under
/// Python 2.0:
macro_rules! PyErr_BadInternalCall__10 {
    () => {
        unsafe {
            _PyErr_BadInternalCall(
                c"/home/takashi/c-corpus/cpython/Python/pystrtod.c".as_ptr(),
                1292i32,
            )
        };
    };
}

/// Mask the old API with a call to the new API for code compiled under
/// Python 2.0:
macro_rules! PyErr_BadInternalCall__96 {
    () => {
        unsafe {
            _PyErr_BadInternalCall(
                c"/home/takashi/c-corpus/cpython/Python/pystrtod.c".as_ptr(),
                1298i32,
            )
        };
    };
}

/// Mask the old API with a call to the new API for code compiled under
/// Python 2.0:
macro_rules! PyErr_BadInternalCall__107 {
    () => {
        unsafe {
            _PyErr_BadInternalCall(
                c"/home/takashi/c-corpus/cpython/Python/pystrtod.c".as_ptr(),
                1119i32,
            )
        };
    };
}

/// Argument must be a char or an int in [-128, 127] or [0, 255].
macro_rules! Py_CHARMASK {
    ($c:expr) => {
        ((($c) as i32) & 255) as u8
    };
}

macro_rules! Py_ISDIGIT__3 {
    ($c:expr) => {
        (unsafe { _Py_ctype_table[(Py_CHARMASK!($c) as i32) as usize] }) & 4
    };
}

macro_rules! Py_TOLOWER {
    ($c:expr) => {
        unsafe { _Py_ctype_tolower[(Py_CHARMASK!($c) as i32) as usize] }
    };
}

macro_rules! Py_UNREACHABLE {
    () => {
        unsafe { core::hint::unreachable_unchecked() };
    };
}

unsafe extern "C" {
    static mut PyExc_ValueError: *mut _object;
    static mut _Py_ctype_table: [u32; 256];
    static mut _Py_ctype_tolower: [u8; 256];
    fn exp(arg0: f64) -> f64;
    fn fabs(arg0: f64) -> f64;
    fn PyMem_Malloc(arg0: u64) -> *mut ();
    fn PyMem_Free(arg0: *mut ());
    fn PyErr_SetString(arg0: *mut _object, arg1: *const i8);
    fn PyErr_NoMemory() -> *mut _object;
    fn PyErr_Format(arg0: *mut _object, arg1: *const i8, ...) -> *mut _object;
    fn _PyErr_BadInternalCall(arg0: *const i8, arg1: i32);
    fn _Py_get_387controlword() -> u16;
    fn _Py_set_387controlword(arg0: u16);
    fn _Py_dg_strtod(arg0: *const i8, arg1: *mut *mut i8) -> f64;
    fn _Py_dg_dtoa(
        arg0: f64,
        arg1: i32,
        arg2: i32,
        arg3: *mut i32,
        arg4: *mut i32,
        arg5: *mut *mut i8,
    ) -> *mut i8;
    fn _Py_dg_freedtoa(arg0: *mut i8);
    fn __builtin_unreachable();
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

// Cast argument to PyObject* type.
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_object_h_172 {
    ob_base: _object,
    ob_size: i64,
}

// Cast argument to PyVarObject* type.
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
struct __SlateRecord_object_h_135 {
    am_await: Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
    am_aiter: Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
    am_anext: Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
    am_send:
        Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object, *mut *mut _object) -> i32>,
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

#[repr(C)]
#[derive(Clone, Copy)]
struct PyMethodDef {
    ml_name: *const i8,
    ml_meth: Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
    ml_flags: i32,
    ml_doc: *const i8,
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

#[repr(C, align(16))]
struct __SlateAlign16<T>(T);

unsafe impl<T> Sync for __SlateAlign16<T> {}

// _Py_dg_strtod()
// _PY_SHORT_FLOAT_REPR
// localeconv()
// Case-insensitive string match used for nan and inf detection; t should be
// lower-case.  Returns 1 for a successful match, 0 otherwise.
fn case_insensitive_match(mut s: *const i8, mut t: *const i8) -> i32 {
    while (unsafe { *t }) != 0 && (Py_TOLOWER!(unsafe { *s }) as i32) == ((unsafe { *t }) as i32) {
        let __v0: *const i8 = s;
        s = __v0.wrapping_offset(1 as isize);
        let __v2: *const i8 = t;
        t = __v2.wrapping_offset(1 as isize);
    }
    if (unsafe { *t }) != 0 { 0 } else { 1 }
}

/// _Py_parse_inf_or_nan: Attempt to parse a string of the form "nan", "inf" or
/// "infinity", with an optional leading sign of "+" or "-".  On success,
/// return the NaN or Infinity as a double and set *endptr to point just beyond
/// the successfully parsed portion of the string.  On failure, return -1.0 and
/// set *endptr to point to the start of the string.
#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_parse_inf_or_nan(p: *const i8, endptr: *mut *mut i8) -> f64 {
    let mut retval: f64 = 0 as f64;
    let mut s: *const i8 = unsafe { core::mem::zeroed() };
    let mut negate: i32 = 0;
    s = p;
    if ((unsafe { *s }) as i32) == 45 {
        negate = 1;
        let __v0: *const i8 = s;
        s = __v0.wrapping_offset(1 as isize);
    } else {
        if ((unsafe { *s }) as i32) == 43 {
            let __v2: *const i8 = s;
            s = __v2.wrapping_offset(1 as isize);
        }
    }
    if case_insensitive_match(s, c"inf".as_ptr()) != 0 {
        let __v4: *const i8 = s;
        s = __v4.wrapping_offset(3 as isize);
        if case_insensitive_match(s, c"inity".as_ptr()) != 0 {
            let __v6: *const i8 = s;
            s = __v6.wrapping_offset(5 as isize);
        }
        retval = if negate != 0 {
            -(f32::INFINITY as f64)
        } else {
            f32::INFINITY as f64
        };
    } else {
        if case_insensitive_match(s, c"nan".as_ptr()) != 0 {
            let __v8: *const i8 = s;
            s = __v8.wrapping_offset(3 as isize);
            retval = if negate != 0 {
                -unsafe { fabs(f32::from_bits(0x7fc00000) as f64) }
            } else {
                unsafe { fabs(f32::from_bits(0x7fc00000) as f64) }
            };
        } else {
            s = p;
            retval = -1.0f64;
        }
    }
    unsafe {
        *endptr = s as *mut i8;
    }
    retval
}

use slate_support::dup;
use slate_support::memset;
use slate_support::sprintf;
use slate_support::strchr;
use slate_support::strncpy;

// _PyOS_ascii_strtod:
// @nptr:    the string to convert to a numeric value.
// @endptr:  if non-%NULL, it returns the character after
//           the last character used in the conversion.
//
// Converts a string to a #gdouble value.
// This function behaves like the standard strtod() function
// does in the C locale. It does this without actually
// changing the current locale, since that would not be
// thread-safe.
//
// This function is typically used when reading configuration
// files or other non-user input that should be locale independent.
// To handle input from the user you should normally use the
// locale-sensitive system strtod() function.
//
// If the correct value would cause overflow, plus or minus %HUGE_VAL
// is returned (according to the sign of the value), and %ERANGE is
// stored in %errno. If the correct value would cause underflow,
// zero is returned and %ERANGE is stored in %errno.
// If memory allocation fails, %ENOMEM is stored in %errno.
//
// This function resets %errno before calling strtod() so that
// you can reliably detect overflow and underflow.
//
// Return value: the #gdouble value.
fn _PyOS_ascii_strtod(nptr: *const i8, endptr: *mut *mut i8) -> f64 {
    let mut result: f64 = 0 as f64;
    let mut old_387controlword: u16 = 0;
    let mut new_387controlword: u16 = 0;
    // Set errno to zero, so that we can distinguish zero results
    // and underflows
    unsafe {
        *slate_support::errno_location() = 0;
    }
    loop {
        old_387controlword = unsafe { _Py_get_387controlword() };
        new_387controlword = ((old_387controlword as i32) & !3840 | 512) as u16;
        if (new_387controlword as i32) != (old_387controlword as i32) {
            unsafe { _Py_set_387controlword(new_387controlword) };
        }
        if true {
            break;
        }
    }
    result = unsafe { _Py_dg_strtod(nptr, endptr) };
    loop {
        if (new_387controlword as i32) != (old_387controlword as i32) {
            unsafe { _Py_set_387controlword(old_387controlword) };
        }
        if true {
            break;
        }
    }
    if (unsafe { *endptr }) == (nptr as *mut i8) {
        result = _Py_parse_inf_or_nan(nptr, endptr);
    }
    // string might represent an inf or nan
    result
}

// PyOS_string_to_double converts a null-terminated byte string s (interpreted
// as a string of ASCII characters) to a float.  The string should not have
// leading or trailing whitespace.  The conversion is independent of the
// current locale.
//
// If endptr is NULL, try to convert the whole string.  Raise ValueError and
// return -1.0 if the string is not a valid representation of a floating-point
// number.
//
// If endptr is non-NULL, try to convert as much of the string as possible.
// If no initial segment of the string is the valid representation of a
// floating-point number then *endptr is set to point to the beginning of the
// string, -1.0 is returned and again ValueError is raised.
//
// On overflow (e.g., when trying to convert '1e500' on an IEEE 754 machine),
// if overflow_exception is NULL then +-Py_INFINITY is returned, and no Python
// exception is raised.  Otherwise, overflow_exception should point to
// a Python exception, this exception will be raised, -1.0 will be returned,
// and *endptr will point just past the end of the converted value.
//
// If any other failure occurs (for example lack of memory), -1.0 is returned
// and the appropriate Python exception will have been set.
#[unsafe(no_mangle)]
extern "C-unwind" fn PyOS_string_to_double(
    s: *const i8,
    endptr: *mut *mut i8,
    overflow_exception: *mut _object,
) -> f64 {
    let mut x: f64 = 0 as f64;
    let mut result: f64 = -1.0f64;
    let mut fail_pos: *mut i8 = unsafe { core::mem::zeroed() };
    unsafe {
        *slate_support::errno_location() = 0;
    }
    x = _PyOS_ascii_strtod(s, core::ptr::addr_of_mut!(fail_pos));
    if (unsafe { *slate_support::errno_location() }) == (slate_support::ENOMEM as i32) {
        unsafe { PyErr_NoMemory() };
        fail_pos = s as *mut i8;
    } else {
        if endptr.is_null() && (fail_pos == (s as *mut i8) || ((unsafe { *fail_pos }) as i32) != 0)
        {
            unsafe {
                PyErr_Format(
                    PyExc_ValueError,
                    c"could not convert string to float: '%.200s'".as_ptr(),
                    s,
                )
            };
        } else {
            if fail_pos == (s as *mut i8) {
                unsafe {
                    PyErr_Format(
                        PyExc_ValueError,
                        c"could not convert string to float: '%.200s'".as_ptr(),
                        s,
                    )
                };
            } else {
                if (unsafe { *slate_support::errno_location() }) == (slate_support::ERANGE as i32)
                    && (unsafe { fabs(x) }) >= 1.0f64
                    && !overflow_exception.is_null()
                {
                    unsafe {
                        PyErr_Format(
                            overflow_exception,
                            c"value too large to convert to float: '%.200s'".as_ptr(),
                            s,
                        )
                    };
                } else {
                    result = x;
                }
            }
        }
    }
    if !endptr.is_null() {
        unsafe {
            *endptr = fail_pos;
        }
    }
    result
}

/// Remove underscores that follow the underscore placement rule from
/// the string and then call the `innerfunc` function on the result.
/// It should return a new object or NULL on exception.
///
/// `what` is used for the error message emitted when underscores are detected
/// that don't follow the rule. `arg` is an opaque pointer passed to the inner
/// function.
///
/// This is used to implement underscore-agnostic conversion for floats
/// and complex numbers.
#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_string_to_number_with_underscores(
    s: *const i8,
    orig_len: i64,
    what: *const i8,
    obj: *mut _object,
    arg: *mut (),
    innerfunc: Option<unsafe extern "C-unwind" fn(*const i8, i64, *mut ()) -> *mut _object>,
) -> *mut _object {
    let mut end: *mut i8 = unsafe { core::mem::zeroed() };
    let mut dup: *mut i8 = unsafe { core::mem::zeroed() };
    let mut last: *const i8 = unsafe { core::mem::zeroed() };
    let mut p: *const i8 = unsafe { core::mem::zeroed() };
    let mut prev: i8 = unsafe { core::mem::zeroed() };
    unsafe {
        if ((strchr(s as *const core::ffi::c_char, 95i32 as i32) as *mut i8) as *const i8).is_null()
        {
            return innerfunc.unwrap()(s, orig_len, arg);
        } else {
            dup = PyMem_Malloc(orig_len.wrapping_add(1) as u64) as *mut i8;
            if dup.is_null() {
                return PyErr_NoMemory();
            } else {
                end = dup;
                prev = 0;
                last = s.wrapping_offset(orig_len as isize);
                p = s;
                '__join_0: {
                    while *p != 0 {
                        if (*p as i32) == 95 {
                            // Underscores are only allowed after digits.
                            if !((prev as i32) >= 48 && (prev as i32) <= 57) {
                                break '__join_0;
                            }
                        } else {
                            let __v3: *mut i8 = end;
                            end = __v3.wrapping_offset(1 as isize);
                            *__v3 = *p;
                            // Underscores are only allowed before digits.
                            if (prev as i32) == 95 && !((*p as i32) >= 48 && (*p as i32) <= 57) {
                                break '__join_0;
                            }
                        }
                        prev = *p;
                        let __v1: *const i8 = p;
                        p = __v1.wrapping_offset(1 as isize);
                    }
                    // Underscores are not allowed at the end.
                    if (prev as i32) != 95 {
                        // No embedded NULs allowed.
                        if p == last {
                            *end = 0;
                            let result: *mut _object = innerfunc.unwrap()(
                                dup as *const i8,
                                end.offset_from(dup as *mut i8) as i64,
                                arg,
                            );
                            PyMem_Free(dup as *mut ());
                            return result;
                        }
                    }
                }
                PyMem_Free(dup as *mut ());
                PyErr_Format(
                    PyExc_ValueError,
                    c"could not convert string to %s: %R".as_ptr(),
                    what,
                    obj,
                );
                return core::ptr::null_mut::<_object>();
            }
        }
    }
}

// Largest precision magnitude accepted by PyOS_double_to_string().  The
// output buffer sizes computed below and within _Py_dg_dtoa() use int and
// Py_ssize_t arithmetic on roughly precision + (digits before the point, at
// most DBL_MAX_10_EXP + 1 == 309) + a few bytes of sign, point and exponent.
// Staying this far inside the int range keeps all of those sums in range.
// (Only C callers can pass a negative precision.)  _Py_dg_dtoa() applies
// the same bound to its ndigits argument.
// _Py_dg_dtoa is available.
// I'm using a lookup table here so that I don't have to invent a non-locale
// specific way to convert to uppercase
// The lengths of these are known to the code below, so don't change them
static lc_float_strings: __SlateAlign16<[*const i8; 3]> = __SlateAlign16([
    (b"inf\0".as_ptr() as *mut i8) as *const i8,
    (b"nan\0".as_ptr() as *mut i8) as *const i8,
    (b"e\0".as_ptr() as *mut i8) as *const i8,
]);

static uc_float_strings: __SlateAlign16<[*const i8; 3]> = __SlateAlign16([
    (b"INF\0".as_ptr() as *mut i8) as *const i8,
    (b"NAN\0".as_ptr() as *mut i8) as *const i8,
    (b"E\0".as_ptr() as *mut i8) as *const i8,
]);

// Convert a double d to a string, and return a PyMem_Malloc'd block of
// memory contain the resulting string.
//
// Arguments:
//   d is the double to be converted
//   format_code is one of 'e', 'f', 'g', 'r'.  'e', 'f' and 'g'
//     correspond to '%e', '%f' and '%g';  'r' corresponds to repr.
//   mode is one of '0', '2' or '3', and is completely determined by
//     format_code: 'e' and 'g' use mode 2; 'f' mode 3, 'r' mode 0.
//   precision is the desired precision
//   always_add_sign is nonzero if a '+' sign should be included for positive
//     numbers
//   add_dot_0_if_integer is nonzero if integers in non-exponential form
//     should have ".0" added.  Only applies to format codes 'r' and 'g'.
//   use_alt_formatting is nonzero if alternative formatting should be
//     used.  Only applies to format codes 'e', 'f' and 'g'.  For code 'g',
//     at most one of use_alt_formatting and add_dot_0_if_integer should
//     be nonzero.
//   type, if non-NULL, will be set to one of these constants to identify
//     the type of the 'd' argument:
//   Py_DTST_FINITE
//   Py_DTST_INFINITE
//   Py_DTST_NAN
//
// Returns a PyMem_Malloc'd block of memory containing the resulting string,
//  or NULL on error. If NULL is returned, the Python error has been set.
fn format_float_short(
    d: f64,
    format_code: i8,
    mode: i32,
    precision: i32,
    always_add_sign: i32,
    add_dot_0_if_integer: i32,
    use_alt_formatting: i32,
    no_negative_zero: i32,
    float_strings: *const *const i8,
    r#type: *mut i32,
) -> *mut i8 {
    let mut new_387controlword: u16 = unsafe { core::mem::zeroed() };
    let mut old_387controlword: u16 = unsafe { core::mem::zeroed() };
    let mut vdigits_end: i64 = unsafe { core::mem::zeroed() };
    let mut vdigits_start: i64 = unsafe { core::mem::zeroed() };
    let mut digits_len: i64 = unsafe { core::mem::zeroed() };
    let mut decpt: i64 = unsafe { core::mem::zeroed() };
    let mut use_exp: i32 = unsafe { core::mem::zeroed() };
    let mut exp: i32 = unsafe { core::mem::zeroed() };
    let mut sign: i32 = unsafe { core::mem::zeroed() };
    let mut decpt_as_int: i32 = unsafe { core::mem::zeroed() };
    let mut digits_end: *mut i8 = unsafe { core::mem::zeroed() };
    let mut digits: *mut i8 = unsafe { core::mem::zeroed() };
    let mut bufsize: i64 = unsafe { core::mem::zeroed() };
    let mut p: *mut i8 = unsafe { core::mem::zeroed() };
    let mut buf: *mut i8 = unsafe { core::mem::zeroed() };
    unsafe {
        buf = core::ptr::null_mut::<i8>();
        p = core::ptr::null_mut::<i8>();
        bufsize = 0;
        exp = 0;
        use_exp = 0;
        // _Py_dg_dtoa returns a digit string (no decimal point or exponent).
        // Must be matched by a call to _Py_dg_freedtoa.
        loop {
            old_387controlword = _Py_get_387controlword();
            new_387controlword = ((old_387controlword as i32) & !3840 | 512) as u16;
            if (new_387controlword as i32) != (old_387controlword as i32) {
                _Py_set_387controlword(new_387controlword);
            }
            if true {
                break;
            }
        }
        digits = _Py_dg_dtoa(
            d,
            mode,
            precision,
            core::ptr::addr_of_mut!(decpt_as_int),
            core::ptr::addr_of_mut!(sign),
            core::ptr::addr_of_mut!(digits_end),
        );
        loop {
            if (new_387controlword as i32) != (old_387controlword as i32) {
                _Py_set_387controlword(old_387controlword);
            }
            if true {
                break;
            }
        }
        '__join_4: {
            decpt = decpt_as_int as i64;
            if digits.is_null() {
                // The only failure mode is no memory.
                PyErr_NoMemory();
            } else {
                digits_len = digits_end.offset_from(digits as *mut i8) as i64;
                if no_negative_zero != 0
                    && sign == 1
                    && (digits_len == 0
                        || digits_len == 1 && (*digits.wrapping_offset(0 as isize) as i32) == 48)
                {
                    sign = 0;
                }
                if digits_len != 0
                    && Py_ISDIGIT__3!(unsafe { *digits.wrapping_offset(0 as isize) }) == 0
                {
                    // Infinities and nans here; adapt Gay's output,
                    // so convert Infinity to inf and NaN to nan, and
                    // ignore sign of nan. Then return.
                    // ignore the actual sign of a nan
                    if (*digits.wrapping_offset(0 as isize) as i32) == 110
                        || (*digits.wrapping_offset(0 as isize) as i32) == 78
                    {
                        sign = 0;
                    }
                    // We only need 5 bytes to hold the result "+inf\0" .
                    bufsize = 5; // Used later in an assert.
                    buf = PyMem_Malloc(bufsize as u64) as *mut i8;
                    if buf.is_null() {
                        PyErr_NoMemory();
                    } else {
                        p = buf;
                        if sign == 1 {
                            let __v39: *mut i8 = p;
                            p = __v39.wrapping_offset(1 as isize);
                            *__v39 = 45;
                        } else {
                            if always_add_sign != 0 {
                                let __v41: *mut i8 = p;
                                p = __v41.wrapping_offset(1 as isize);
                                *__v41 = 43;
                            }
                        }
                        if (*digits.wrapping_offset(0 as isize) as i32) == 105
                            || (*digits.wrapping_offset(0 as isize) as i32) == 73
                        {
                            strncpy(
                                p as *mut core::ffi::c_char,
                                *float_strings.wrapping_offset(0 as isize)
                                    as *const core::ffi::c_char,
                                3u64 as usize,
                            ) as *mut i8;
                            let __v35: *mut i8 = p;
                            p = __v35.wrapping_offset(3 as isize);
                            if !r#type.is_null() {
                                *r#type = 1;
                            }
                        } else {
                            if (*digits.wrapping_offset(0 as isize) as i32) == 110
                                || (*digits.wrapping_offset(0 as isize) as i32) == 78
                            {
                                strncpy(
                                    p as *mut core::ffi::c_char,
                                    *float_strings.wrapping_offset(1 as isize)
                                        as *const core::ffi::c_char,
                                    3u64 as usize,
                                ) as *mut i8;
                                let __v37: *mut i8 = p;
                                p = __v37.wrapping_offset(3 as isize);
                                if !r#type.is_null() {
                                    *r#type = 2;
                                }
                            } else {
                                // shouldn't get here: Gay's code should always return
                                // something starting with a digit, an 'I',  or 'N'
                                Py_UNREACHABLE!();
                            }
                        }
                    }
                } else {
                    // The result must be finite (not inf or nan).
                    if !r#type.is_null() {
                        *r#type = 0;
                    }
                    // We got digits back, format them.  We may need to pad 'digits'
                    // either on the left or right (or both) with extra zeros, so in
                    // general the resulting string has the form
                    //
                    //   [<sign>]<zeros><digits><zeros>[<exponent>]
                    //
                    // where either of the <zeros> pieces could be empty, and there's a
                    // decimal point that could appear either in <digits> or in the
                    // leading or trailing <zeros>.
                    //
                    // Imagine an infinite 'virtual' string vdigits, consisting of the
                    // string 'digits' (starting at index 0) padded on both the left and
                    // right with infinite strings of zeros.  We want to output a slice
                    //
                    //   vdigits[vdigits_start : vdigits_end]
                    //
                    // of this virtual string.  Thus if vdigits_start < 0 then we'll end
                    // up producing some leading zeros; if vdigits_end > digits_len there
                    // will be trailing zeros in the output.  The next section of code
                    // determines whether to use an exponent or not, figures out the
                    // position 'decpt' of the decimal point, and computes 'vdigits_start'
                    // and 'vdigits_end'.
                    vdigits_end = digits_len;
                    let __t0: i32 = format_code as i32;
                    if __t0 == 101 {
                        use_exp = 1;
                        vdigits_end = precision as i64;
                    } else {
                        if __t0 == 102 {
                            vdigits_end = decpt.wrapping_add(precision as i64);
                        } else {
                            if __t0 == 103 {
                                if decpt <= (4i32.wrapping_neg() as i64)
                                    || decpt
                                        > ((if add_dot_0_if_integer != 0 {
                                            precision.wrapping_sub(1)
                                        } else {
                                            precision
                                        }) as i64)
                                {
                                    use_exp = 1;
                                }
                                if use_alt_formatting != 0 {
                                    vdigits_end = precision as i64;
                                }
                            } else {
                                if __t0 == 114 {
                                    if decpt <= (4i32.wrapping_neg() as i64) || decpt > 16 {
                                        use_exp = 1;
                                    }
                                // convert to exponential format at 1e16.  We used to convert
                                // at 1e17, but that gives odd-looking results for some values
                                // when a 16-digit 'shortest' repr is padded with bogus zeros.
                                // For example, repr(2e16+8) would give 20000000000000010.0;
                                // the true value is 20000000000000008.0.
                                } else {
                                    PyErr_BadInternalCall__107!();
                                    break '__join_4;
                                }
                            }
                        }
                    }
                    // if using an exponent, reset decimal point position to 1 and adjust
                    // exponent accordingly.
                    if use_exp != 0 {
                        exp = (decpt as i32).wrapping_sub(1);
                        decpt = 1;
                    }
                    // ensure vdigits_start < decpt <= vdigits_end, or vdigits_start <
                    // decpt < vdigits_end if add_dot_0_if_integer and no exponent
                    vdigits_start = if decpt <= 0 { decpt.wrapping_sub(1) } else { 0 };
                    if use_exp == 0 && add_dot_0_if_integer != 0 {
                        vdigits_end = if vdigits_end > decpt {
                            vdigits_end
                        } else {
                            decpt.wrapping_add(1)
                        };
                    } else {
                        vdigits_end = if vdigits_end > decpt {
                            vdigits_end
                        } else {
                            decpt
                        };
                    }
                    // double check inequalities
                    // decimal point should be in (vdigits_start, vdigits_end]
                    // Compute an upper bound how much memory we need. This might be a few
                    // chars too long, but no big deal.
                    bufsize = 3i64
                        .wrapping_add(vdigits_end.wrapping_sub(vdigits_start))
                        .wrapping_add((if use_exp != 0 { 5i32 } else { 0i32 }) as i64); // sign, decimal point and trailing 0 byte
                    // total digit count (including zero padding on both sides)
                    // exponent "e+100", max 3 numerical digits
                    // Now allocate the memory and initialize p to point to the start of
                    // it.
                    buf = PyMem_Malloc(bufsize as u64) as *mut i8;
                    if buf.is_null() {
                        PyErr_NoMemory();
                    } else {
                        p = buf;
                        // Add a negative sign if negative, and a plus sign if non-negative
                        // and always_add_sign is true.
                        if sign == 1 {
                            let __v31: *mut i8 = p;
                            p = __v31.wrapping_offset(1 as isize);
                            *__v31 = 45;
                        } else {
                            if always_add_sign != 0 {
                                let __v33: *mut i8 = p;
                                p = __v33.wrapping_offset(1 as isize);
                                *__v33 = 43;
                            }
                        }
                        // note that exactly one of the three 'if' conditions is true,
                        // so we include exactly one decimal point
                        //
                        // Zero padding on left of digit string
                        if decpt <= 0 {
                            core::ptr::write_bytes(
                                ((p as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                                (48i32 as i32) as u8,
                                ((decpt.wrapping_sub(vdigits_start) as u64) as usize) as usize,
                            );
                            let __v27: *mut i8 = p;
                            p = __v27.wrapping_offset(decpt.wrapping_sub(vdigits_start) as isize);
                            let __v25: *mut i8 = p;
                            p = __v25.wrapping_offset(1 as isize);
                            *__v25 = 46;
                            core::ptr::write_bytes(
                                ((p as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                                (48i32 as i32) as u8,
                                ((0i64.wrapping_sub(decpt) as u64) as usize) as usize,
                            );
                            let __v23: *mut i8 = p;
                            p = __v23.wrapping_offset(0i64.wrapping_sub(decpt) as isize);
                        } else {
                            core::ptr::write_bytes(
                                ((p as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                                (48i32 as i32) as u8,
                                ((0i64.wrapping_sub(vdigits_start) as u64) as usize) as usize,
                            );
                            let __v29: *mut i8 = p;
                            p = __v29.wrapping_offset(0i64.wrapping_sub(vdigits_start) as isize);
                        }
                        // Digits, with included decimal point
                        if 0 < decpt && decpt <= digits_len {
                            strncpy(
                                p as *mut core::ffi::c_char,
                                (digits as *const i8) as *const core::ffi::c_char,
                                (decpt.wrapping_sub(0) as u64) as usize,
                            ) as *mut i8;
                            let __v19: *mut i8 = p;
                            p = __v19.wrapping_offset(decpt.wrapping_sub(0) as isize);
                            let __v17: *mut i8 = p;
                            p = __v17.wrapping_offset(1 as isize);
                            *__v17 = 46;
                            strncpy(
                                p as *mut core::ffi::c_char,
                                (digits.wrapping_offset(decpt as isize) as *const i8)
                                    as *const core::ffi::c_char,
                                (digits_len.wrapping_sub(decpt) as u64) as usize,
                            ) as *mut i8;
                            let __v15: *mut i8 = p;
                            p = __v15.wrapping_offset(digits_len.wrapping_sub(decpt) as isize);
                        } else {
                            strncpy(
                                p as *mut core::ffi::c_char,
                                (digits as *const i8) as *const core::ffi::c_char,
                                (digits_len as u64) as usize,
                            ) as *mut i8;
                            let __v21: *mut i8 = p;
                            p = __v21.wrapping_offset(digits_len as isize);
                        }
                        // And zeros on the right
                        if digits_len < decpt {
                            core::ptr::write_bytes(
                                ((p as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                                (48i32 as i32) as u8,
                                ((decpt.wrapping_sub(digits_len) as u64) as usize) as usize,
                            );
                            let __v11: *mut i8 = p;
                            p = __v11.wrapping_offset(decpt.wrapping_sub(digits_len) as isize);
                            let __v9: *mut i8 = p;
                            p = __v9.wrapping_offset(1 as isize);
                            *__v9 = 46;
                            core::ptr::write_bytes(
                                ((p as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                                (48i32 as i32) as u8,
                                ((vdigits_end.wrapping_sub(decpt) as u64) as usize) as usize,
                            );
                            let __v7: *mut i8 = p;
                            p = __v7.wrapping_offset(vdigits_end.wrapping_sub(decpt) as isize);
                        } else {
                            core::ptr::write_bytes(
                                ((p as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                                (48i32 as i32) as u8,
                                ((vdigits_end.wrapping_sub(digits_len) as u64) as usize) as usize,
                            );
                            let __v13: *mut i8 = p;
                            p = __v13
                                .wrapping_offset(vdigits_end.wrapping_sub(digits_len) as isize);
                        }
                        // Delete a trailing decimal pt unless using alternative formatting.
                        if (*p.wrapping_offset(1i32.wrapping_neg() as isize) as i32) == 46
                            && use_alt_formatting == 0
                        {
                            let __v5: *mut i8 = p;
                            p = __v5.wrapping_offset(-(1 as isize));
                        }
                        // Now that we've done zero padding, add an exponent if needed.
                        if use_exp != 0 {
                            let __v3: *mut i8 = p;
                            p = __v3.wrapping_offset(1 as isize);
                            *__v3 = *unsafe { *float_strings.wrapping_offset(2 as isize) }
                                .wrapping_offset(0 as isize);
                            let exp_len: i32 = sprintf(
                                p as *mut core::ffi::c_char,
                                c"%+.02d".as_ptr() as *const core::ffi::c_char,
                                exp,
                            ) as i32;
                            let __v1: *mut i8 = p;
                            p = __v1.wrapping_offset(exp_len as isize);
                        }
                    }
                }
            }
        }
        if !buf.is_null() {
            *p = 0;
            // It's too late if this fails, as we've already stepped on
            // memory that isn't ours. But it's an okay debugging test.
        }
        if !digits.is_null() {
            _Py_dg_freedtoa(digits);
        }
        return buf;
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyOS_double_to_string(
    val: f64,
    mut format_code: i8,
    mut precision: i32,
    flags: i32,
    r#type: *mut i32,
) -> *mut i8 {
    let mut float_strings: *const *const i8 =
        unsafe { core::ptr::addr_of!(lc_float_strings.0) as *const *const i8 };
    let mut mode: i32 = 0;
    if precision > 2147482623i32 || precision < 2147482623i32.wrapping_neg() {
        unsafe { PyErr_SetString(PyExc_ValueError, c"precision too big".as_ptr()) };
        return core::ptr::null_mut::<i8>();
    }
    // Validate format_code, and map upper and lower case. Compute the
    // mode and make any adjustments as needed.
    // exponent
    // fixed
    // general
    // precision 0 makes no sense for 'g' format; interpret as 1
    // repr format
    // Supplied precision is unused, must be 0.
    '__slate_break_0: {
        match format_code as i32 {
            69 => {
                // exponent
                float_strings =
                    unsafe { core::ptr::addr_of!(uc_float_strings.0) as *const *const i8 };
                format_code = 101;
                mode = 2;
                let _v3: i32 = precision;
                let _v4: i32 = _v3.wrapping_add(1);
                precision = _v4;
                break '__slate_break_0;
                // fixed
            }
            101 => {
                mode = 2;
                precision = precision.wrapping_add(1);
                break '__slate_break_0;
                // fixed
            }
            70 => {
                float_strings =
                    unsafe { core::ptr::addr_of!(uc_float_strings.0) as *const *const i8 };
                format_code = 102;
                mode = 3;
                break '__slate_break_0;
                // general
            }
            102 => {
                mode = 3;
                break '__slate_break_0;
                // general
            }
            71 => {
                float_strings =
                    unsafe { core::ptr::addr_of!(uc_float_strings.0) as *const *const i8 };
                format_code = 103;
                mode = 2;
                // precision 0 makes no sense for 'g' format; interpret as 1
                if precision == 0 {
                    precision = 1;
                }
                break '__slate_break_0;
                // repr format
            }
            103 => {
                mode = 2;
                // precision 0 makes no sense for 'g' format; interpret as 1
                if precision == 0 {
                    precision = 1;
                }
                break '__slate_break_0;
                // repr format
            }
            114 => {
                mode = 0;
                // Supplied precision is unused, must be 0.
                if precision != 0 {
                    PyErr_BadInternalCall__10!();
                    return core::ptr::null_mut::<i8>();
                }
            }
            _ => {
                PyErr_BadInternalCall__96!();
                return core::ptr::null_mut::<i8>();
            }
        }
    }
    format_float_short(
        val,
        format_code,
        mode,
        precision,
        flags & 1,
        flags & 2,
        flags & 4,
        flags & 8,
        float_strings,
        r#type,
    )
}
