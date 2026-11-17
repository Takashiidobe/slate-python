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
//! This limitation is for performance and simplicity. If needed it can be
//! removed (with effort).
//! Remote debugger support
//! PY_TIMEOUT_T is the integral type used to specify timeouts when waiting
//!
//! on a lock (see PyThread_acquire_lock_timed() below).
//! _PyStatus_ERR
//! Chosen so that _PyObjectStackChunk is a power-of-two size.
//! Largest tuple to save on freelist
//! leads to warnings... __attribute__((cdecl))
//! Various defines for all used memory orders in mimalloc
//! Main tuning parameters for segment and page sizes
//!
//! Sizes for 64-bit (usually divide by two for 32-bit)
//! Inlined definitions
//! If we change this, we need to change the default value in the
//! signature of gc.collect.
//! Total tool ids available
//! _PyGenObject_HEAD defines the initial segment of generator
//! and coroutine objects.
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
// 64KiB  (32KiB on 32-bit)
// 1024
// Used as a special value to encode block sizes in 32 bits.
// 35 == mi_segment_bin(MI_SLICES_PER_SEGMENT)
// initialize a local variable to zero; use memset as compilers optimize constant sized memset's
// The size of the Bigint freelist
// The size of the cached powers of 5 array
// Must be a power of 2
// For now we hard-code this to a value for which we are confident
// all the static builtin types will fit (for all builds).
// Must be a power of 2
// Length of array of slotdef pointers used to store slots with the
// same __name__.  There should be at most MAX_EQUIV-1 slotdef entries with
// the same __name__, for any __name__. Since that's a static property, it is
// appropriate to declare fixed-size arrays for this.
// Count of all local monitoring events
// Count of all "real" monitoring events (not derived from other events)
// Count of all  monitoring events
// List of weak reference.
// Name of the generator.
// Qualified name of the generator.
// The frame
macro_rules! PyFloat_Check {
    ($op:expr) => {
        PyObject_TypeCheck!($op, unsafe { core::ptr::addr_of_mut!(PyFloat_Type) })
    };
}

macro_rules! PyLong_Check {
    ($op:expr) => {
        PyType_FastSubclass!(Py_TYPE!($op), 16777216u64)
    };
}

macro_rules! PyObject_TypeCheck {
    ($ob:expr, $type:expr) => {
        PyObject_TypeCheck(_PyObject_CAST!($ob), $type)
    };
}

macro_rules! PyType_FastSubclass {
    ($type:expr, $flag:expr) => {
        PyType_HasFeature($type, $flag)
    };
}

/// Absolute value of the number x
macro_rules! Py_ABS {
    ($x:expr) => {
        if ($x) < 0 { $x.wrapping_neg() } else { $x }
    };
}

macro_rules! Py_IS_TYPE {
    ($ob:expr, $type:expr) => {
        Py_IS_TYPE(_PyObject_CAST!($ob), $type)
    };
}

macro_rules! Py_TYPE {
    ($ob:expr) => {
        _Py_TYPE(_PyObject_CAST!($ob))
    };
}

/// Cast argument to PyObject* type.
macro_rules! _PyObject_CAST {
    ($op:expr) => {
        _Py_CAST__2!($op)
    };
}

/// Macro to use the more powerful/dangerous C-style cast even in C++.
macro_rules! _Py_CAST__2 {
    ($expr:expr) => {
        $expr
    };
}

unsafe extern "C" {
    static mut PyExc_OSError: *mut _object;
    static mut PyExc_OverflowError: *mut _object;
    static mut PyExc_TypeError: *mut _object;
    static mut PyExc_ValueError: *mut _object;
    static mut PyFloat_Type: _typeobject;
    fn modf(arg0: f64, arg1: *mut f64) -> f64;
    fn ceil(arg0: f64) -> f64;
    fn fabs(arg0: f64) -> f64;
    fn floor(arg0: f64) -> f64;
    fn round(arg0: f64) -> f64;
    fn div(arg0: i32, arg1: i32) -> __SlateRecord_stdlib_h_67;
    fn PyType_IsSubtype(arg0: *mut _typeobject, arg1: *mut _typeobject) -> i32;
    fn PyErr_SetString(arg0: *mut _object, arg1: *const i8);
    fn PyErr_Occurred() -> *mut _object;
    fn PyErr_ExceptionMatches(arg0: *mut _object) -> i32;
    fn PyErr_SetFromErrno(arg0: *mut _object) -> *mut _object;
    fn PyErr_Format(arg0: *mut _object, arg1: *const i8, ...) -> *mut _object;
    fn PyLong_FromLongLong(arg0: i64) -> *mut _object;
    fn PyLong_AsLongLong(arg0: *mut _object) -> i64;
    fn PyFloat_AsDouble(arg0: *mut _object) -> f64;
    fn PyStatus_Ok() -> __SlateRecord_initconfig_h_19;
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_stdlib_h_67 {
    quot: i32,
    rem: i32,
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

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_object_h_172 {
    ob_base: _object,
    ob_size: i64,
}

// Cast argument to PyVarObject* type.
fn _Py_TYPE(ob: *mut _object) -> *mut _typeobject {
    unsafe { (*ob).ob_type }
}

fn Py_IS_TYPE(ob: *mut _object, r#type: *mut _typeobject) -> i32 {
    (Py_TYPE!(ob) == r#type) as i32
}

fn PyObject_TypeCheck(ob: *mut _object, r#type: *mut _typeobject) -> i32 {
    let __v0: bool =
        Py_IS_TYPE!(ob, r#type) != 0 || (unsafe { PyType_IsSubtype(Py_TYPE!(ob), r#type) }) != 0;
    __v0 as i32
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

fn PyType_HasFeature(r#type: *mut _typeobject, feature: u64) -> i32 {
    let flags: u64 = unsafe { (*r#type).tp_flags };
    (flags & feature != 0) as i32
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
struct __SlateRecord_initconfig_h_19 {
    _type: u32,
    func: *const i8,
    err_msg: *const i8,
    exitcode: i32,
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

use slate_support::clock_getres;
use slate_support::clock_gettime;
use slate_support::gmtime_r;
use slate_support::localtime_r;

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
struct __SlateRecord_pycore_runtime_structs_h_92 {
    numer: i64,
    denom: i64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _Py_time_runtime_state {
    _unused: i8,
}

// These would be in pycore_long.h if it weren't for an include cycle.
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_time_h_251 {
    implementation: *const i8,
    monotonic: i32,
    adjustable: i32,
    resolution: f64,
}

// _Py_AssertHoldsTstate()
// _PyRuntime
// PyTime_t
// gmtime_r()
// gettimeofday()
// To millisecond (10^-3)
// To microseconds (10^-6)
// To nanoseconds (10^-9)
// Conversion from nanoseconds
fn _PyTime_GCD(mut x: i64, mut y: i64) -> i64 {
    // Euclidean algorithm
    while y != 0 {
        let tmp: i64 = y;
        y = x % y;
        x = tmp;
    }
    x
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyTimeFraction_Set(
    frac: *mut __SlateRecord_pycore_runtime_structs_h_92,
    numer: i64,
    denom: i64,
) -> i32 {
    if numer < 1 || denom < 1 {
        return 1i32.wrapping_neg();
    }
    let gcd: i64 = _PyTime_GCD(numer, denom);
    unsafe {
        (*frac).numer = numer / gcd;
    }
    unsafe {
        (*frac).denom = denom / gcd;
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyTimeFraction_Resolution(
    frac: *const __SlateRecord_pycore_runtime_structs_h_92,
) -> f64 {
    ((unsafe { (*frac).numer }) as f64) / ((unsafe { (*frac).denom }) as f64) / 1000000000.0f64
}

fn pytime_time_t_overflow() {
    unsafe {
        PyErr_SetString(
            PyExc_OverflowError,
            c"timestamp out of range for platform time_t".as_ptr(),
        )
    };
}

fn pytime_overflow() {
    unsafe {
        PyErr_SetString(
            PyExc_OverflowError,
            c"timestamp out of range for C PyTime_t".as_ptr(),
        )
    };
}

/// Compute t1 + t2. Clamp to [PyTime_MIN; PyTime_MAX] on overflow.
fn pytime_add(t1: &mut i64, t2: i64) -> i32 {
    if t2 > 0 && (unsafe { *(t1 as *mut i64) }) > 9223372036854775807i64.wrapping_sub(t2) {
        unsafe {
            *(t1 as *mut i64) = 9223372036854775807;
        }
        return 1i32.wrapping_neg();
    } else {
        if t2 < 0
            && (unsafe { *(t1 as *mut i64) })
                < 9223372036854775807i64
                    .wrapping_neg()
                    .wrapping_sub(1)
                    .wrapping_sub(t2)
        {
            unsafe {
                *(t1 as *mut i64) = 9223372036854775807i64.wrapping_neg().wrapping_sub(1);
            }
            return 1i32.wrapping_neg();
        } else {
            let __v1: i64 = unsafe { *t1 };
            unsafe {
                *t1 = __v1.wrapping_add(t2);
            }
            return 0;
        }
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyTime_Add(mut t1: i64, t2: i64) -> i64 {
    pytime_add(unsafe { &mut (*core::ptr::addr_of_mut!(t1)) }, t2);
    t1
}

fn pytime_mul_check_overflow(a: i64, b: i64) -> i32 {
    if b != 0 {
        return (a < 9223372036854775807i64.wrapping_neg().wrapping_sub(1) / b
            || 9223372036854775807 / b < a) as i32;
    } else {
        return 0;
    }
}

/// Compute t * k. Clamp to [PyTime_MIN; PyTime_MAX] on overflow.
fn pytime_mul(t: &mut i64, k: i64) -> i32 {
    if pytime_mul_check_overflow(unsafe { *(t as *mut i64) }, k) != 0 {
        unsafe {
            *(t as *mut i64) = if *(t as *mut i64) >= 0 {
                9223372036854775807
            } else {
                9223372036854775807i64.wrapping_neg().wrapping_sub(1)
            };
        }
        return 1i32.wrapping_neg();
    } else {
        let __v1: i64 = unsafe { *t };
        unsafe {
            *t = __v1.wrapping_mul(k);
        }
        return 0;
    }
}

/// Compute t * k. Clamp to [PyTime_MIN; PyTime_MAX] on overflow.
fn _PyTime_Mul(mut t: i64, k: i64) -> i64 {
    pytime_mul(unsafe { &mut (*core::ptr::addr_of_mut!(t)) }, k);
    t
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyTimeFraction_Mul(
    mut ticks: i64,
    frac: *const __SlateRecord_pycore_runtime_structs_h_92,
) -> i64 {
    let mul: i64 = unsafe { (*frac).numer };
    let div: i64 = unsafe { (*frac).denom };
    if div == 1 {
        // Fast-path taken by mach_absolute_time() with 1/1 time base.
        return _PyTime_Mul(ticks, mul);
    }
    // Compute (ticks * mul / div) in two parts to reduce the risk of integer
    // overflow: compute the integer part, and then the remaining part.
    //
    // (ticks * mul) / div == (ticks / div) * mul + (ticks % div) * mul / div
    let intpart: i64 = ticks / div;
    let mut remaining: i64 = 0;
    ticks %= div;
    remaining = _PyTime_Mul(ticks, mul) / div;
    // intpart * mul + remaining
    _PyTime_Add(_PyTime_Mul(intpart, mul), remaining)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyLong_AsTime_t(obj: *mut _object) -> i64 {
    let val: i64 = unsafe { PyLong_AsLongLong(obj) };
    let __v0: bool = val == (1i32.wrapping_neg() as i64) && !unsafe { PyErr_Occurred() }.is_null();
    if __v0 {
        if (unsafe { PyErr_ExceptionMatches(PyExc_OverflowError) }) != 0 {
            pytime_time_t_overflow();
        }
        return 1i32.wrapping_neg() as i64;
    }
    val
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyLong_FromTime_t(t: i64) -> *mut _object {
    unsafe { PyLong_FromLongLong(t) }
}

/// Convert PyTime_t to time_t.
///
/// Return 0 on success. Return -1 and clamp the value on overflow.
fn _PyTime_AsTime_t(t: i64, t2: *mut i64) -> i32 {
    unsafe {
        *t2 = t;
    }
    0
}

/// Round to nearest with ties going to nearest even integer
/// (_PyTime_ROUND_HALF_EVEN)
fn pytime_round_half_even(x: f64) -> f64 {
    let mut rounded: f64 = unsafe { round(x) };
    if (unsafe { fabs(x - rounded) }) == 0.5f64 {
        // halfway case: round to even
        rounded = 2.0f64 * unsafe { round(x / 2.0f64) };
    }
    rounded
}

fn pytime_round(x: f64, round: u32) -> f64 {
    // volatile avoids optimization changing how numbers are rounded
    let mut d: f64 = 0 as f64;
    unsafe { core::ptr::write_volatile(core::ptr::addr_of_mut!(d), x) };
    if (round as u32) == 2 {
        unsafe {
            core::ptr::write_volatile(
                core::ptr::addr_of_mut!(d),
                pytime_round_half_even(core::ptr::read_volatile(core::ptr::addr_of!(d))),
            )
        };
    } else {
        if (round as u32) == 1 {
            unsafe {
                core::ptr::write_volatile(
                    core::ptr::addr_of_mut!(d),
                    ceil(core::ptr::read_volatile(core::ptr::addr_of!(d))),
                )
            };
        } else {
            if (round as u32) == 0 {
                unsafe {
                    core::ptr::write_volatile(
                        core::ptr::addr_of_mut!(d),
                        floor(core::ptr::read_volatile(core::ptr::addr_of!(d))),
                    )
                };
            } else {
                unsafe {
                    core::ptr::write_volatile(
                        core::ptr::addr_of_mut!(d),
                        if core::ptr::read_volatile(core::ptr::addr_of!(d)) >= 0.0f64 {
                            ceil(core::ptr::read_volatile(core::ptr::addr_of!(d)))
                        } else {
                            floor(core::ptr::read_volatile(core::ptr::addr_of!(d)))
                        },
                    )
                };
            }
        }
    }
    unsafe { core::ptr::read_volatile(core::ptr::addr_of!(d)) }
}

fn pytime_double_to_denominator(
    d: f64,
    sec: *mut i64,
    numerator: *mut i64,
    idenominator: i64,
    round: u32,
) -> i32 {
    let denominator: f64 = idenominator as f64;
    let mut intpart: f64 = 0 as f64;
    // volatile avoids optimization changing how numbers are rounded
    let mut floatpart: f64 = 0 as f64;
    unsafe {
        core::ptr::write_volatile(
            core::ptr::addr_of_mut!(floatpart),
            modf(d, core::ptr::addr_of_mut!(intpart)),
        )
    };
    let __v0: f64 = unsafe { core::ptr::read_volatile(core::ptr::addr_of!(floatpart)) };
    unsafe { core::ptr::write_volatile(core::ptr::addr_of_mut!(floatpart), __v0 * denominator) };
    unsafe {
        core::ptr::write_volatile(
            core::ptr::addr_of_mut!(floatpart),
            pytime_round(
                core::ptr::read_volatile(core::ptr::addr_of!(floatpart)),
                round,
            ),
        )
    };
    if (unsafe { core::ptr::read_volatile(core::ptr::addr_of!(floatpart)) }) >= denominator {
        let __v2: f64 = unsafe { core::ptr::read_volatile(core::ptr::addr_of!(floatpart)) };
        unsafe {
            core::ptr::write_volatile(core::ptr::addr_of_mut!(floatpart), __v2 - denominator)
        };
        let __v5: f64 = intpart + 1.0f64;
        intpart = __v5;
    } else {
        if (unsafe { core::ptr::read_volatile(core::ptr::addr_of!(floatpart)) }) < (0i32 as f64) {
            let __v6: f64 = unsafe { core::ptr::read_volatile(core::ptr::addr_of!(floatpart)) };
            unsafe {
                core::ptr::write_volatile(core::ptr::addr_of_mut!(floatpart), __v6 + denominator)
            };
            let __v9: f64 = intpart - 1.0f64;
            intpart = __v9;
        }
    }
    // Conversion of an out-of-range value to time_t gives undefined behaviour
    // (C99 §6.3.1.4p1), so we must guard against it. However, checking that
    // `intpart` is in range is delicate: the obvious expression `intpart <=
    // PY_TIME_T_MAX` will first convert the value `PY_TIME_T_MAX` to a double,
    // potentially changing its value and leading to us failing to catch some
    // UB-inducing values. The code below works correctly under the mild
    // assumption that time_t is a two's complement integer type with no trap
    // representation, and that `PY_TIME_T_MIN` is within the representable
    // range of a C double.
    //
    // Note: we want the `if` condition below to be true for NaNs; therefore,
    // resist any temptation to simplify by applying De Morgan's laws.
    if !((9223372036854775807i64.wrapping_neg().wrapping_sub(1) as f64) <= intpart
        && intpart < -(9223372036854775807i64.wrapping_neg().wrapping_sub(1) as f64))
    {
        pytime_time_t_overflow();
        return 1i32.wrapping_neg();
    }
    unsafe {
        *sec = intpart as i64;
    }
    unsafe {
        *numerator = core::ptr::read_volatile(core::ptr::addr_of!(floatpart)) as i64;
    }
    0
}

fn pytime_object_to_denominator(
    obj: *mut _object,
    sec: *mut i64,
    numerator: *mut i64,
    denominator: i64,
    round: u32,
) -> i32 {
    if PyFloat_Check!(obj) != 0 {
        let d: f64 = unsafe { PyFloat_AsDouble(obj) };
        if d.is_nan() {
            unsafe {
                *numerator = 0;
            }
            unsafe {
                PyErr_SetString(
                    PyExc_ValueError,
                    c"Invalid value NaN (not a number)".as_ptr(),
                )
            };
            return 1i32.wrapping_neg();
        }
        return pytime_double_to_denominator(d, sec, numerator, denominator, round);
    } else {
        unsafe {
            *sec = _PyLong_AsTime_t(obj);
        }
        unsafe {
            *numerator = 0;
        }
        let __v0: bool = (unsafe { *sec }) == (1i32.wrapping_neg() as i64)
            && !unsafe { PyErr_Occurred() }.is_null();
        if __v0 {
            if (unsafe { PyErr_ExceptionMatches(PyExc_TypeError) }) != 0 {
                unsafe {
                    PyErr_Format(
                        PyExc_TypeError,
                        c"argument must be int or float, not %T".as_ptr(),
                        obj,
                    )
                };
            }
            return 1i32.wrapping_neg();
        }
        return 0;
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyTime_ObjectToTime_t(obj: *mut _object, sec: *mut i64, round: u32) -> i32 {
    if PyFloat_Check!(obj) != 0 {
        let mut intpart: f64 = 0 as f64;
        // volatile avoids optimization changing how numbers are rounded
        let mut d: f64 = 0 as f64;
        unsafe { core::ptr::write_volatile(core::ptr::addr_of_mut!(d), PyFloat_AsDouble(obj)) };
        if unsafe { core::ptr::read_volatile(core::ptr::addr_of!(d)) }.is_nan() {
            unsafe {
                PyErr_SetString(
                    PyExc_ValueError,
                    c"Invalid value NaN (not a number)".as_ptr(),
                )
            };
            return 1i32.wrapping_neg();
        }
        unsafe {
            core::ptr::write_volatile(
                core::ptr::addr_of_mut!(d),
                pytime_round(core::ptr::read_volatile(core::ptr::addr_of!(d)), round),
            )
        };
        unsafe {
            modf(
                core::ptr::read_volatile(core::ptr::addr_of!(d)),
                core::ptr::addr_of_mut!(intpart),
            )
        };
        // See comments in pytime_double_to_denominator
        if !((9223372036854775807i64.wrapping_neg().wrapping_sub(1) as f64) <= intpart
            && intpart < -(9223372036854775807i64.wrapping_neg().wrapping_sub(1) as f64))
        {
            pytime_time_t_overflow();
            return 1i32.wrapping_neg();
        }
        unsafe {
            *sec = intpart as i64;
        }
        return 0;
    } else {
        unsafe {
            *sec = _PyLong_AsTime_t(obj);
        }
        let __v0: bool = (unsafe { *sec }) == (1i32.wrapping_neg() as i64)
            && !unsafe { PyErr_Occurred() }.is_null();
        if __v0 {
            return 1i32.wrapping_neg();
        }
        return 0;
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyTime_ObjectToTimespec(
    obj: *mut _object,
    sec: *mut i64,
    nsec: *mut i64,
    round: u32,
) -> i32 {
    pytime_object_to_denominator(obj, sec, nsec, 1000000000i64, round)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyTime_ObjectToTimeval(
    obj: *mut _object,
    sec: *mut i64,
    usec: *mut i64,
    round: u32,
) -> i32 {
    pytime_object_to_denominator(obj, sec, usec, 1000000i64, round)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyTime_FromSeconds(seconds: i32) -> i64 {
    // ensure that integer overflow cannot happen, int type should have 32
    // bits, whereas PyTime_t type has at least 64 bits (SEC_TO_NS takes 30
    // bits).
    let mut t: i64 = seconds as i64;
    t = t.wrapping_mul(1000000000i64);
    t
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyTime_FromMicrosecondsClamp(us: i64) -> i64 {
    let ns: i64 = _PyTime_Mul(us, 1000i64);
    ns
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyTime_FromLong(tp: *mut i64, obj: *mut _object) -> i32 {
    if PyLong_Check!(obj) == 0 {
        unsafe {
            PyErr_Format(
                PyExc_TypeError,
                c"expect int, got %s".as_ptr(),
                (*Py_TYPE!(obj)).tp_name,
            )
        };
        return 1i32.wrapping_neg();
    }
    let nsec: i64 = unsafe { PyLong_AsLongLong(obj) };
    let __v0: bool = nsec == (1i32.wrapping_neg() as i64) && !unsafe { PyErr_Occurred() }.is_null();
    if __v0 {
        if (unsafe { PyErr_ExceptionMatches(PyExc_OverflowError) }) != 0 {
            pytime_overflow();
        }
        return 1i32.wrapping_neg();
    }
    let t: i64 = nsec;
    unsafe {
        *tp = t;
    }
    0
}

fn pytime_fromtimespec(tp: *mut i64, ts: *const slate_support::timespec, raise_exc: i32) -> i32 {
    let mut t: i64 = unsafe { (*ts).tv_sec };
    let mut tv_nsec: i64 = 0;
    let res1: i32 = pytime_mul(unsafe { &mut (*core::ptr::addr_of_mut!(t)) }, 1000000000i64);
    tv_nsec = unsafe { (*ts).tv_nsec };
    let res2: i32 = pytime_add(unsafe { &mut (*core::ptr::addr_of_mut!(t)) }, tv_nsec);
    unsafe {
        *tp = t;
    }
    if raise_exc != 0 && (res1 < 0 || res2 < 0) {
        pytime_overflow();
        return 1i32.wrapping_neg();
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyTime_FromTimespec(tp: *mut i64, ts: *const slate_support::timespec) -> i32 {
    pytime_fromtimespec(tp, ts, 1i32)
}

fn pytime_fromtimeval(tp: *mut i64, tv: *mut slate_support::timeval, raise_exc: i32) -> i32 {
    let mut t: i64 = unsafe { (*tv).tv_sec };
    let res1: i32 = pytime_mul(unsafe { &mut (*core::ptr::addr_of_mut!(t)) }, 1000000000i64);
    let usec: i64 = unsafe { (*tv).tv_usec }.wrapping_mul(1000);
    let res2: i32 = pytime_add(unsafe { &mut (*core::ptr::addr_of_mut!(t)) }, usec);
    unsafe {
        *tp = t;
    }
    if raise_exc != 0 && (res1 < 0 || res2 < 0) {
        pytime_overflow();
        return 1i32.wrapping_neg();
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyTime_FromTimeval(tp: *mut i64, tv: *mut slate_support::timeval) -> i32 {
    pytime_fromtimeval(tp, tv, 1i32)
}

fn pytime_from_double(tp: *mut i64, value: f64, round: u32, unit_to_ns: i64) -> i32 {
    // volatile avoids optimization changing how numbers are rounded
    let mut d: f64 = 0 as f64;
    // convert to a number of nanoseconds
    unsafe { core::ptr::write_volatile(core::ptr::addr_of_mut!(d), value) };
    let __v0: f64 = unsafe { core::ptr::read_volatile(core::ptr::addr_of!(d)) };
    unsafe { core::ptr::write_volatile(core::ptr::addr_of_mut!(d), __v0 * (unit_to_ns as f64)) };
    unsafe {
        core::ptr::write_volatile(
            core::ptr::addr_of_mut!(d),
            pytime_round(core::ptr::read_volatile(core::ptr::addr_of!(d)), round),
        )
    };
    // See comments in pytime_double_to_denominator
    if !((9223372036854775807i64.wrapping_neg().wrapping_sub(1) as f64)
        <= unsafe { core::ptr::read_volatile(core::ptr::addr_of!(d)) }
        && (unsafe { core::ptr::read_volatile(core::ptr::addr_of!(d)) })
            < -(9223372036854775807i64.wrapping_neg().wrapping_sub(1) as f64))
    {
        pytime_overflow();
        unsafe {
            *tp = 0;
        }
        return 1i32.wrapping_neg();
    }
    let ns: i64 = (unsafe { core::ptr::read_volatile(core::ptr::addr_of!(d)) }) as i64;
    unsafe {
        *tp = ns;
    }
    0
}

fn pytime_from_object(tp: *mut i64, obj: *mut _object, round: u32, unit_to_ns: i64) -> i32 {
    if PyFloat_Check!(obj) != 0 {
        let d: f64 = unsafe { PyFloat_AsDouble(obj) };
        if d.is_nan() {
            unsafe {
                PyErr_SetString(
                    PyExc_ValueError,
                    c"Invalid value NaN (not a number)".as_ptr(),
                )
            };
            return 1i32.wrapping_neg();
        }
        return pytime_from_double(tp, d, round, unit_to_ns);
    }
    let sec: i64 = unsafe { PyLong_AsLongLong(obj) };
    let __v0: bool = sec == (1i32.wrapping_neg() as i64) && !unsafe { PyErr_Occurred() }.is_null();
    if __v0 {
        if (unsafe { PyErr_ExceptionMatches(PyExc_OverflowError) }) != 0 {
            pytime_overflow();
        } else {
            if (unsafe { PyErr_ExceptionMatches(PyExc_TypeError) }) != 0 {
                unsafe {
                    PyErr_Format(
                        PyExc_TypeError,
                        c"'%T' object cannot be interpreted as an integer or float".as_ptr(),
                        obj,
                    )
                };
            }
        }
        return 1i32.wrapping_neg();
    }
    let mut ns: i64 = sec;
    if pytime_mul(unsafe { &mut (*core::ptr::addr_of_mut!(ns)) }, unit_to_ns) < 0 {
        pytime_overflow();
        return 1i32.wrapping_neg();
    }
    unsafe {
        *tp = ns;
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyTime_FromSecondsObject(tp: *mut i64, obj: *mut _object, round: u32) -> i32 {
    pytime_from_object(tp, obj, round, 1000000000i64)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyTime_FromMillisecondsObject(
    tp: *mut i64,
    obj: *mut _object,
    round: u32,
) -> i32 {
    pytime_from_object(tp, obj, round, 1000000i64)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyTime_AsSecondsDouble(ns: i64) -> f64 {
    // volatile avoids optimization changing how numbers are rounded
    let mut d: f64 = 0 as f64;
    if ns % 1000000000i64 == 0 {
        // Divide using integers to avoid rounding issues on the integer part.
        // 1e-9 cannot be stored exactly in IEEE 64-bit.
        let secs: i64 = ns / 1000000000i64;
        unsafe { core::ptr::write_volatile(core::ptr::addr_of_mut!(d), secs as f64) };
    } else {
        unsafe { core::ptr::write_volatile(core::ptr::addr_of_mut!(d), ns as f64) };
        let __v0: f64 = unsafe { core::ptr::read_volatile(core::ptr::addr_of!(d)) };
        let __v1: f64 = __v0 / 1000000000.0f64;
        unsafe { core::ptr::write_volatile(core::ptr::addr_of_mut!(d), __v1) };
    }
    unsafe { core::ptr::read_volatile(core::ptr::addr_of!(d)) }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyTime_AsLong(ns: i64) -> *mut _object {
    unsafe { PyLong_FromLongLong(ns) }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyTime_FromSecondsDouble(seconds: f64, round: u32, result: *mut i64) -> i32 {
    pytime_from_double(result, seconds, round, 1000000000i64)
}

fn pytime_divide_round_up(t: i64, k: i64) -> i64 {
    if t >= 0 {
        // Don't use (t + k - 1) / k to avoid integer overflow
        //
        // if t is equal to PyTime_MAX
        let mut q: i64 = t / k;
        if t % k != 0 {
            q = q.wrapping_add(1);
        }
        return q;
    } else {
        // Don't use (t - (k - 1)) / k to avoid integer overflow
        //
        // if t is equals to PyTime_MIN.
        let mut q: i64 = t / k;
        if t % k != 0 {
            q = q.wrapping_sub(1);
        }
        return q;
    }
}

fn pytime_divide(t: i64, k: i64, round: u32) -> i64 {
    if (round as u32) == 2 {
        let mut x: i64 = t / k;
        let r: i64 = t % k;
        let abs_r: i64 = Py_ABS!(r);
        if abs_r > k / 2 || abs_r == k / 2 && Py_ABS!(x) & 1 != 0 {
            if t >= 0 {
                x = x.wrapping_add(1);
            } else {
                x = x.wrapping_sub(1);
            }
        }
        return x;
    } else {
        if (round as u32) == 1 {
            if t >= 0 {
                return pytime_divide_round_up(t, k);
            } else {
                return t / k;
            }
        } else {
            if (round as u32) == 0 {
                if t >= 0 {
                    return t / k;
                } else {
                    return pytime_divide_round_up(t, k);
                }
            } else {
                return pytime_divide_round_up(t, k);
            }
        }
    }
}

/// Compute (t / k, t % k) in (pq, pr).
///
/// Make sure that 0 <= pr < k.
///
/// Return 0 on success.
///
/// Return -1 on underflow and store (PyTime_MIN, 0) in (pq, pr).
fn pytime_divmod(t: i64, k: i64, pq: &mut i64, pr: &mut i64) -> i32 {
    let mut q: i64 = t / k;
    let mut r: i64 = t % k;
    if r < 0 {
        if q == 9223372036854775807i64.wrapping_neg().wrapping_sub(1) {
            unsafe {
                *(pq as *mut i64) = 9223372036854775807i64.wrapping_neg().wrapping_sub(1);
            }
            unsafe {
                *(pr as *mut i64) = 0;
            }
            return 1i32.wrapping_neg();
        }
        r = r.wrapping_add(k);
        q = q.wrapping_sub(1);
    }
    unsafe {
        *(pq as *mut i64) = q;
    }
    unsafe {
        *(pr as *mut i64) = r;
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyTime_AsMicroseconds(ns: i64, round: u32) -> i64 {
    pytime_divide(ns, 1000i64, round)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyTime_AsMilliseconds(ns: i64, round: u32) -> i64 {
    pytime_divide(ns, 1000000i64, round)
}

fn pytime_as_timeval(ns: i64, ptv_sec: &mut i64, ptv_usec: *mut i32, round: u32) -> i32 {
    let us: i64 = pytime_divide(ns, 1000i64, round);
    let mut tv_sec: i64 = 0;
    let mut tv_usec: i64 = 0;
    let res: i32 = pytime_divmod(
        us,
        1000000i64,
        unsafe { &mut (*core::ptr::addr_of_mut!(tv_sec)) },
        unsafe { &mut (*core::ptr::addr_of_mut!(tv_usec)) },
    );
    unsafe {
        *(ptv_sec as *mut i64) = tv_sec;
    }
    unsafe {
        *ptv_usec = tv_usec as i32;
    }
    res
}

fn pytime_as_timeval_struct(
    t: i64,
    tv: *mut slate_support::timeval,
    round: u32,
    raise_exc: i32,
) -> i32 {
    let mut tv_sec: i64 = 0;
    let mut tv_usec: i32 = 0;
    let res: i32 = pytime_as_timeval(
        t,
        unsafe { &mut (*core::ptr::addr_of_mut!(tv_sec)) },
        core::ptr::addr_of_mut!(tv_usec),
        round,
    );
    let res2: i32 = _PyTime_AsTime_t(tv_sec, unsafe { core::ptr::addr_of_mut!((*tv).tv_sec) });
    if res2 < 0 {
        tv_usec = 0;
    }
    unsafe {
        (*tv).tv_usec = tv_usec as i64;
    }
    if raise_exc != 0 && (res < 0 || res2 < 0) {
        pytime_time_t_overflow();
        return 1i32.wrapping_neg();
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyTime_AsTimeval(t: i64, tv: *mut slate_support::timeval, round: u32) -> i32 {
    pytime_as_timeval_struct(t, tv, round, 1i32)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyTime_AsTimeval_clamp(t: i64, tv: *mut slate_support::timeval, round: u32) {
    pytime_as_timeval_struct(t, tv, round, 0i32);
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyTime_AsTimevalTime_t(
    t: i64,
    p_secs: *mut i64,
    us: *mut i32,
    round: u32,
) -> i32 {
    let mut secs: i64 = 0;
    if pytime_as_timeval(
        t,
        unsafe { &mut (*core::ptr::addr_of_mut!(secs)) },
        us,
        round,
    ) < 0
    {
        pytime_time_t_overflow();
        return 1i32.wrapping_neg();
    }
    if _PyTime_AsTime_t(secs, p_secs) < 0 {
        pytime_time_t_overflow();
        return 1i32.wrapping_neg();
    }
    0
}

fn pytime_as_timespec(ns: i64, ts: *mut slate_support::timespec, raise_exc: i32) -> i32 {
    let mut tv_sec: i64 = 0;
    let mut tv_nsec: i64 = 0;
    let res: i32 = pytime_divmod(
        ns,
        1000000000i64,
        unsafe { &mut (*core::ptr::addr_of_mut!(tv_sec)) },
        unsafe { &mut (*core::ptr::addr_of_mut!(tv_nsec)) },
    );
    let res2: i32 = _PyTime_AsTime_t(tv_sec, unsafe { core::ptr::addr_of_mut!((*ts).tv_sec) });
    if res2 < 0 {
        tv_nsec = 0;
    }
    unsafe {
        (*ts).tv_nsec = tv_nsec;
    }
    if raise_exc != 0 && (res < 0 || res2 < 0) {
        pytime_time_t_overflow();
        return 1i32.wrapping_neg();
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyTime_AsTimespec_clamp(t: i64, ts: *mut slate_support::timespec) {
    pytime_as_timespec(t, ts, 0i32);
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyTime_AsTimespec(t: i64, ts: *mut slate_support::timespec) -> i32 {
    pytime_as_timespec(t, ts, 1i32)
}

/// N.B. If raise_exc=0, this may be called without a thread state.
fn py_get_system_clock(
    tp: *mut i64,
    info: *mut __SlateRecord_pycore_time_h_251,
    raise_exc: i32,
) -> i32 {
    // raise_exc requires to hold a thread state
    let mut err: i32 = 0;
    let mut ts: slate_support::timespec = unsafe { core::mem::zeroed() };
    err = unsafe {
        clock_gettime(
            (slate_support::CLOCK_REALTIME as i32) as i32,
            core::ptr::addr_of_mut!(ts) as *mut slate_support::timespec,
        ) as i32
    };
    if err != 0 {
        if raise_exc != 0 {
            unsafe { PyErr_SetFromErrno(PyExc_OSError) };
        }
        return 1i32.wrapping_neg();
    }
    if pytime_fromtimespec(
        tp,
        core::ptr::addr_of_mut!(ts) as *const slate_support::timespec,
        raise_exc,
    ) < 0
    {
        return 1i32.wrapping_neg();
    }
    if !info.is_null() {
        let mut res: slate_support::timespec = unsafe { core::mem::zeroed() };
        unsafe {
            (*info).implementation = c"clock_gettime(CLOCK_REALTIME)".as_ptr();
        }
        unsafe {
            (*info).monotonic = 0;
        }
        unsafe {
            (*info).adjustable = 1;
        }
        if (unsafe {
            clock_getres(
                (slate_support::CLOCK_REALTIME as i32) as i32,
                core::ptr::addr_of_mut!(res) as *mut slate_support::timespec,
            ) as i32
        }) == 0
        {
            unsafe {
                (*info).resolution = (res.tv_sec as f64) + (res.tv_nsec as f64) * 1e-9f64;
            }
        } else {
            unsafe {
                (*info).resolution = 1e-9f64;
            }
        }
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyTime_Time(result: *mut i64) -> i32 {
    if py_get_system_clock(
        result,
        core::ptr::null_mut::<__SlateRecord_pycore_time_h_251>(),
        1i32,
    ) < 0
    {
        unsafe {
            *result = 0;
        }
        return 1i32.wrapping_neg();
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyTime_TimeRaw(result: *mut i64) -> i32 {
    if py_get_system_clock(
        result,
        core::ptr::null_mut::<__SlateRecord_pycore_time_h_251>(),
        0i32,
    ) < 0
    {
        unsafe {
            *result = 0;
        }
        return 1i32.wrapping_neg();
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyTime_TimeWithInfo(
    t: *mut i64,
    info: *mut __SlateRecord_pycore_time_h_251,
) -> i32 {
    py_get_system_clock(t, info, 1i32)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyTime_Init(
    state: *mut _Py_time_runtime_state,
) -> __SlateRecord_initconfig_h_19 {
    return unsafe { PyStatus_Ok() };
}

/// N.B. If raise_exc=0, this may be called without a thread state.
fn py_get_monotonic_clock(
    tp: *mut i64,
    info: *mut __SlateRecord_pycore_time_h_251,
    raise_exc: i32,
) -> i32 {
    // raise_exc requires to hold a thread state
    let clk_id: i32 = slate_support::CLOCK_MONOTONIC as i32;
    let implementation: *const i8 = c"clock_gettime(CLOCK_MONOTONIC)".as_ptr();
    let mut ts: slate_support::timespec = unsafe { core::mem::zeroed() };
    if (unsafe {
        clock_gettime(
            clk_id as i32,
            core::ptr::addr_of_mut!(ts) as *mut slate_support::timespec,
        ) as i32
    }) != 0
    {
        if raise_exc != 0 {
            unsafe { PyErr_SetFromErrno(PyExc_OSError) };
            return 1i32.wrapping_neg();
        }
        return 1i32.wrapping_neg();
    }
    if pytime_fromtimespec(
        tp,
        core::ptr::addr_of_mut!(ts) as *const slate_support::timespec,
        raise_exc,
    ) < 0
    {
        return 1i32.wrapping_neg();
    }
    if !info.is_null() {
        unsafe {
            (*info).monotonic = 1;
        }
        unsafe {
            (*info).implementation = implementation;
        }
        unsafe {
            (*info).adjustable = 0;
        }
        let mut res: slate_support::timespec = unsafe { core::mem::zeroed() };
        if (unsafe {
            clock_getres(
                clk_id as i32,
                core::ptr::addr_of_mut!(res) as *mut slate_support::timespec,
            ) as i32
        }) != 0
        {
            unsafe { PyErr_SetFromErrno(PyExc_OSError) };
            return 1i32.wrapping_neg();
        }
        unsafe {
            (*info).resolution = (res.tv_sec as f64) + (res.tv_nsec as f64) * 1e-9f64;
        }
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyTime_Monotonic(result: *mut i64) -> i32 {
    if py_get_monotonic_clock(
        result,
        core::ptr::null_mut::<__SlateRecord_pycore_time_h_251>(),
        1i32,
    ) < 0
    {
        unsafe {
            *result = 0;
        }
        return 1i32.wrapping_neg();
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyTime_MonotonicRaw(result: *mut i64) -> i32 {
    if py_get_monotonic_clock(
        result,
        core::ptr::null_mut::<__SlateRecord_pycore_time_h_251>(),
        0i32,
    ) < 0
    {
        unsafe {
            *result = 0;
        }
        return 1i32.wrapping_neg();
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyTime_MonotonicWithInfo(
    tp: *mut i64,
    info: *mut __SlateRecord_pycore_time_h_251,
) -> i32 {
    py_get_monotonic_clock(tp, info, 1i32)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyTime_PerfCounterWithInfo(
    t: *mut i64,
    info: *mut __SlateRecord_pycore_time_h_251,
) -> i32 {
    _PyTime_MonotonicWithInfo(t, info)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyTime_PerfCounter(result: *mut i64) -> i32 {
    PyTime_Monotonic(result)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyTime_PerfCounterRaw(result: *mut i64) -> i32 {
    PyTime_MonotonicRaw(result)
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.pytime._PyTime_localtime")]
extern "C-unwind" fn _PyTime_localtime(mut t: i64, tm: *mut slate_support::tm) -> i32 {
    unsafe {
        *slate_support::errno_location() = 0;
    }
    if unsafe {
        localtime_r(
            (core::ptr::addr_of_mut!(t) as *const i64) as *const i64,
            tm as *mut slate_support::tm,
        ) as *mut slate_support::tm
    }
    .is_null()
    {
        if (unsafe { *slate_support::errno_location() }) == 0 {
            unsafe {
                *slate_support::errno_location() = slate_support::EINVAL as i32;
            }
        }
        unsafe { PyErr_SetFromErrno(PyExc_OSError) };
        return 1i32.wrapping_neg();
    }
    0
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.pytime._PyTime_gmtime")]
extern "C-unwind" fn _PyTime_gmtime(mut t: i64, tm: *mut slate_support::tm) -> i32 {
    if unsafe {
        gmtime_r(
            (core::ptr::addr_of_mut!(t) as *const i64) as *const i64,
            tm as *mut slate_support::tm,
        ) as *mut slate_support::tm
    }
    .is_null()
    {
        if (unsafe { *slate_support::errno_location() }) == 0 {
            unsafe {
                *slate_support::errno_location() = slate_support::EINVAL as i32;
            }
        }
        unsafe { PyErr_SetFromErrno(PyExc_OSError) };
        return 1i32.wrapping_neg();
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyDeadline_Init(timeout: i64) -> i64 {
    let mut now: i64 = 0;
    // silently ignore error: cannot report error to the caller
    PyTime_MonotonicRaw(core::ptr::addr_of_mut!(now));
    _PyTime_Add(now, timeout)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyDeadline_Get(deadline: i64) -> i64 {
    let mut now: i64 = 0;
    // silently ignore error: cannot report error to the caller
    PyTime_MonotonicRaw(core::ptr::addr_of_mut!(now));
    deadline.wrapping_sub(now)
}
