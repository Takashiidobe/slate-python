//! Copyright (C) 1997, 2002, 2003, 2007, 2008 Martin von Loewis
//!
//! Permission to use, copy, modify, and distribute this software and its
//! documentation for any purpose and without fee is hereby granted,
//! provided that the above copyright notice appear in all copies.
//!
//! This software comes with no warranty. Use at your own risk.
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
//! PyException_HEAD defines the initial segment of every exception class.
//! Cast argument to PyTupleObject* type.
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
//! To avoid repeating ourselves in deepfreeze.py, all PyCodeObject members are
//!
//! defined in this macro:
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
//! If we change this, we need to change the default value in the
//! signature of gc.collect.
//! Total tool ids available
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
// Define macros for inline documentation.
// Round up size "n" to be a multiple of "a".
// Prevent using an expression as a l-value.
//
// For example, "int x; _Py_RVALUE(x) = 1;" fails with a compiler error.
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
macro_rules! GET_LOCALE_STRING {
    ($ATTR:ident, $lc:expr) => {
        unsafe { PyUnicode_DecodeLocale((*$lc).$ATTR as *const i8, core::ptr::null::<i8>()) }
    };
}

macro_rules! PyList_SET_ITEM__2 {
    ($op:expr, $index:expr, $value:expr) => {
        PyList_SET_ITEM(
            _PyObject_CAST!($op),
            ($index) as i64,
            _PyObject_CAST!($value),
        );
    };
}

macro_rules! PyMem_New__9 {
    ($type:ty, $n:expr) => {{
        let __v: *mut $type =
            if (($n) as u64) > 9223372036854775807 / (core::mem::size_of::<$type>() as u64) {
                core::ptr::null_mut::<u32>()
            } else {
                (unsafe { PyMem_Malloc($n.wrapping_mul(core::mem::size_of::<$type>() as u64)) })
                    as *mut $type
            };
        __v
    }};
}

macro_rules! PyMem_New__3 {
    ($type:ty, $n:expr) => {{
        let __v: *mut $type =
            if (($n) as u64) > 9223372036854775807 / (core::mem::size_of::<$type>() as u64) {
                core::ptr::null_mut::<i32>()
            } else {
                (unsafe {
                    PyMem_Malloc((($n) as u64).wrapping_mul(core::mem::size_of::<$type>() as u64))
                }) as *mut $type
            };
        __v
    }};
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

macro_rules! Py_CLEAR {
    ($op:expr) => {
        loop {
            let _tmp_op_ptr: *mut *mut _object = unsafe { core::ptr::addr_of_mut!($op) };
            let _tmp_old_op: *mut _object = unsafe { *_tmp_op_ptr };
            if !_tmp_old_op.is_null() {
                unsafe {
                    *_tmp_op_ptr = core::ptr::null_mut::<_object>();
                }
                Py_DECREF!(_tmp_old_op);
            }
            if true {
                break;
            }
        }
    };
}

macro_rules! Py_DECREF {
    ($op:expr) => {
        Py_DECREF(_PyObject_CAST!($op));
    };
}

/// Convert the argument to a string. For example, Py_STRINGIFY(123) is replaced
/// with "123" by the preprocessor. Defines are also replaced by their value.
/// For example Py_STRINGIFY(__LINE__) is replaced by the line number, not
/// by "__LINE__".
macro_rules! Py_STRINGIFY__8 {
    () => {
        c"int_curr_symbol".as_ptr() as *mut i8
    };
}

/// Convert the argument to a string. For example, Py_STRINGIFY(123) is replaced
/// with "123" by the preprocessor. Defines are also replaced by their value.
/// For example Py_STRINGIFY(__LINE__) is replaced by the line number, not
/// by "__LINE__".
macro_rules! Py_STRINGIFY__22 {
    () => {
        c"currency_symbol".as_ptr() as *mut i8
    };
}

/// Convert the argument to a string. For example, Py_STRINGIFY(123) is replaced
/// with "123" by the preprocessor. Defines are also replaced by their value.
/// For example Py_STRINGIFY(__LINE__) is replaced by the line number, not
/// by "__LINE__".
macro_rules! Py_STRINGIFY__34 {
    () => {
        c"mon_thousands_sep".as_ptr() as *mut i8
    };
}

/// Convert the argument to a string. For example, Py_STRINGIFY(123) is replaced
/// with "123" by the preprocessor. Defines are also replaced by their value.
/// For example Py_STRINGIFY(__LINE__) is replaced by the line number, not
/// by "__LINE__".
macro_rules! Py_STRINGIFY__40 {
    () => {
        c"mon_decimal_point".as_ptr() as *mut i8
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

macro_rules! Py_XDECREF {
    ($op:expr) => {
        Py_XDECREF(_PyObject_CAST!($op));
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

/// Cast argument to PyListObject* type.
macro_rules! _PyList_CAST {
    ($op:expr) => {
        _Py_CAST!(*mut __SlateRecord_listobject_h_22, $op)
    };
}

/// Cast argument to PyObject* type.
macro_rules! _PyObject_CAST {
    ($op:expr) => {
        _Py_CAST__2!($op)
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

macro_rules! _Py_IsImmortal {
    ($op:expr) => {
        _Py_IsImmortal(_PyObject_CAST!($op))
    };
}

unsafe extern "C" {
    static mut _Py_NoneStruct: _object;
    static mut PyExc_OSError: *mut _object;
    static mut PyExc_RuntimeError: *mut _object;
    static mut PyExc_ValueError: *mut _object;
    static mut PyExc_RuntimeWarning: *mut _object;
    fn wcscoll(arg0: *const i32, arg1: *const i32) -> i32;
    fn wcsxfrm(arg0: *mut i32, arg1: *const i32, arg2: u64) -> u64;
    fn index(arg0: *const i8, arg1: i32) -> *mut i8;
    fn PyMem_Malloc(arg0: u64) -> *mut ();
    fn PyMem_Realloc(arg0: *mut (), arg1: u64) -> *mut ();
    fn PyMem_Free(arg0: *mut ());
    fn _Py_Dealloc(arg0: *mut _object);
    fn PyBytes_AsString(arg0: *mut _object) -> *mut i8;
    fn PyUnicode_AsWideCharString(arg0: *mut _object, arg1: *mut i64) -> *mut i32;
    fn PyUnicode_AsUTF8AndSize(arg0: *mut _object, arg1: *mut i64) -> *const i8;
    fn PyUnicode_DecodeLocale(arg0: *const i8, arg1: *const i8) -> *mut _object;
    fn PyUnicode_FSConverter(arg0: *mut _object, arg1: *mut ()) -> i32;
    fn PyUnicode_FromKindAndData(arg0: i32, arg1: *const (), arg2: i64) -> *mut _object;
    fn PyErr_SetString(arg0: *mut _object, arg1: *const i8);
    fn PyErr_Occurred() -> *mut _object;
    fn PyErr_NoMemory() -> *mut _object;
    fn PyErr_SetFromErrno(arg0: *mut _object) -> *mut _object;
    fn PyErr_NewException(arg0: *const i8, arg1: *mut _object, arg2: *mut _object) -> *mut _object;
    fn PyLong_FromLong(arg0: i64) -> *mut _object;
    fn PyLong_AsInt(arg0: *mut _object) -> i32;
    fn PyList_New(arg0: i64) -> *mut _object;
    fn PyDict_New() -> *mut _object;
    fn PyDict_SetItemString(arg0: *mut _object, arg1: *const i8, arg2: *mut _object) -> i32;
    fn PyModule_GetState(arg0: *mut _object) -> *mut ();
    fn PyModuleDef_Init(arg0: *mut PyModuleDef) -> *mut _object;
    fn PyModule_AddObjectRef(arg0: *mut _object, arg1: *const i8, arg2: *mut _object) -> i32;
    fn PyModule_AddIntConstant(arg0: *mut _object, arg1: *const i8, arg2: i64) -> i32;
    fn _PyMem_Strdup(arg0: *const i8) -> *mut i8;
    fn _Py_GetLocaleconvNumeric(
        arg0: *mut slate_support::lconv,
        arg1: *mut *mut _object,
        arg2: *mut *mut _object,
    ) -> i32;
    fn _Py_GetLocaleEncodingObject() -> *mut _object;
    fn dcgettext(arg0: *const i8, arg1: *const i8, arg2: i32) -> *mut i8;
    fn textdomain(arg0: *const i8) -> *mut i8;
    fn bindtextdomain(arg0: *const i8, arg1: *const i8) -> *mut i8;
    fn bind_textdomain_codeset(arg0: *const i8, arg1: *const i8) -> *mut i8;
    fn _PyArg_CheckPositional(arg0: *const i8, arg1: i64, arg2: i64, arg3: i64) -> i32;
    fn _PyArg_BadArgument(arg0: *const i8, arg1: *const i8, arg2: *const i8, arg3: *mut _object);
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

fn Py_XDECREF(op: *mut _object) {
    if !op.is_null() {
        Py_DECREF!(op);
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_listobject_h_22 {
    ob_base: __SlateRecord_object_h_172,
    ob_item: *mut *mut _object,
    allocated: i64,
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

#[repr(C)]
#[derive(Clone, Copy)]
struct PyModuleDef_Base {
    ob_base: _object,
    m_init: Option<unsafe extern "C-unwind" fn() -> *mut _object>,
    m_index: i64,
    m_copy: *mut _object,
}

#[repr(C, align(16))]
struct __SlateAlign16<T>(T);

unsafe impl<T> Sync for __SlateAlign16<T> {}

// m_init
// m_index
// m_copy
#[repr(C)]
#[derive(Clone, Copy)]
struct PyModuleDef_Slot {
    slot: i32,
    value: *mut (),
}

#[repr(C)]
#[derive(Clone, Copy)]
struct PyModuleDef {
    m_base: PyModuleDef_Base,
    m_name: *const i8,
    m_doc: *const i8,
    m_size: i64,
    m_methods: *mut PyMethodDef,
    m_slots: *mut PyModuleDef_Slot,
    m_traverse: Option<
        unsafe extern "C-unwind" fn(
            *mut _object,
            Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
            *mut (),
        ) -> i32,
    >,
    m_clear: Option<unsafe extern "C-unwind" fn(*mut _object) -> i32>,
    m_free: Option<unsafe extern "C-unwind" fn(*mut ())>,
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

// _Py_GetLocaleconvNumeric()
// _PyMem_Strdup()
// setlocale()
// strlen()
// errno
// nl_langinfo()
static locale__doc__: __SlateAlign16<[i8; 27]> = __SlateAlign16([
    83i8, 117i8, 112i8, 112i8, 111i8, 114i8, 116i8, 32i8, 102i8, 111i8, 114i8, 32i8, 80i8, 79i8,
    83i8, 73i8, 88i8, 32i8, 108i8, 111i8, 99i8, 97i8, 108i8, 101i8, 115i8, 46i8, 0i8,
]);

#[repr(C)]
#[derive(Clone, Copy)]
struct _locale_state {
    Error: *mut _object,
}

fn get_locale_state(m: *mut _object) -> *mut _locale_state {
    let state: *mut () = unsafe { PyModule_GetState(m) };
    state as *mut _locale_state
}

static _locale_setlocale__doc__: __SlateAlign16<[i8; 86]> = __SlateAlign16([
    115i8, 101i8, 116i8, 108i8, 111i8, 99i8, 97i8, 108i8, 101i8, 40i8, 36i8, 109i8, 111i8, 100i8,
    117i8, 108i8, 101i8, 44i8, 32i8, 99i8, 97i8, 116i8, 101i8, 103i8, 111i8, 114i8, 121i8, 44i8,
    32i8, 108i8, 111i8, 99i8, 97i8, 108i8, 101i8, 61i8, 78i8, 111i8, 110i8, 101i8, 44i8, 32i8,
    47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 65i8, 99i8, 116i8, 105i8, 118i8, 97i8, 116i8, 101i8,
    115i8, 47i8, 113i8, 117i8, 101i8, 114i8, 105i8, 101i8, 115i8, 32i8, 108i8, 111i8, 99i8, 97i8,
    108i8, 101i8, 32i8, 112i8, 114i8, 111i8, 99i8, 101i8, 115i8, 115i8, 105i8, 110i8, 103i8, 46i8,
    0i8,
]);

#[unsafe(link_section = ".text.slate_distinct._localemodule._locale_setlocale")]
extern "C-unwind" fn _locale_setlocale(
    module: *mut _object,
    args: *const *mut _object,
    nargs: i64,
) -> *mut _object {
    let mut locale_length: i64 = unsafe { core::mem::zeroed() };
    let mut __v1: bool = unsafe { core::mem::zeroed() };
    let mut locale: *const i8 = unsafe { core::mem::zeroed() };
    let mut category: i32 = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_0: {
            return_value = core::ptr::null_mut::<_object>();
            locale = core::ptr::null::<i8>();
            if _PyArg_CheckPositional!(c"setlocale".as_ptr() as *mut i8, nargs, 1i32, 2i32) {
                category = PyLong_AsInt(*args.wrapping_offset(0 as isize));
                if category == 1i32.wrapping_neg() {
                    __v1 = !PyErr_Occurred().is_null();
                } else {
                    __v1 = false;
                }
                if !__v1 {
                    if nargs >= 2 {
                        if *args.wrapping_offset(1 as isize)
                            == core::ptr::addr_of_mut!(_Py_NoneStruct)
                        {
                            locale = core::ptr::null::<i8>();
                        } else {
                            if PyUnicode_Check!(unsafe { *args.wrapping_offset(1 as isize) }) != 0 {
                                locale = PyUnicode_AsUTF8AndSize(
                                    *args.wrapping_offset(1 as isize),
                                    core::ptr::addr_of_mut!(locale_length),
                                );
                                if locale.is_null() {
                                    break '__join_0;
                                } else {
                                    if (strlen(locale as *const core::ffi::c_char) as u64)
                                        != (locale_length as u64)
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
                                    c"setlocale".as_ptr(),
                                    c"argument 2".as_ptr(),
                                    c"str or None".as_ptr(),
                                    *args.wrapping_offset(1 as isize),
                                );
                                break '__join_0;
                            }
                        }
                    }
                    return_value = _locale_setlocale_impl(module, category, locale);
                }
            }
        }
        return return_value;
    }
}

static _locale_localeconv__doc__: __SlateAlign16<[i8; 84]> = __SlateAlign16([
    108i8, 111i8, 99i8, 97i8, 108i8, 101i8, 99i8, 111i8, 110i8, 118i8, 40i8, 36i8, 109i8, 111i8,
    100i8, 117i8, 108i8, 101i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8,
    116i8, 117i8, 114i8, 110i8, 115i8, 32i8, 110i8, 117i8, 109i8, 101i8, 114i8, 105i8, 99i8, 32i8,
    97i8, 110i8, 100i8, 32i8, 109i8, 111i8, 110i8, 101i8, 116i8, 97i8, 114i8, 121i8, 32i8, 108i8,
    111i8, 99i8, 97i8, 108i8, 101i8, 45i8, 115i8, 112i8, 101i8, 99i8, 105i8, 102i8, 105i8, 99i8,
    32i8, 112i8, 97i8, 114i8, 97i8, 109i8, 101i8, 116i8, 101i8, 114i8, 115i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct._localemodule._locale_localeconv")]
extern "C-unwind" fn _locale_localeconv(
    module: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    _locale_localeconv_impl(module)
}

static _locale_strcoll__doc__: __SlateAlign16<[i8; 80]> = __SlateAlign16([
    115i8, 116i8, 114i8, 99i8, 111i8, 108i8, 108i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8,
    101i8, 44i8, 32i8, 111i8, 115i8, 49i8, 44i8, 32i8, 111i8, 115i8, 50i8, 44i8, 32i8, 47i8, 41i8,
    10i8, 45i8, 45i8, 10i8, 10i8, 67i8, 111i8, 109i8, 112i8, 97i8, 114i8, 101i8, 115i8, 32i8,
    116i8, 119i8, 111i8, 32i8, 115i8, 116i8, 114i8, 105i8, 110i8, 103i8, 115i8, 32i8, 97i8, 99i8,
    99i8, 111i8, 114i8, 100i8, 105i8, 110i8, 103i8, 32i8, 116i8, 111i8, 32i8, 116i8, 104i8, 101i8,
    32i8, 108i8, 111i8, 99i8, 97i8, 108i8, 101i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct._localemodule._locale_strcoll")]
extern "C-unwind" fn _locale_strcoll(
    module: *mut _object,
    args: *const *mut _object,
    nargs: i64,
) -> *mut _object {
    let mut os1: *mut _object = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        if _PyArg_CheckPositional!(c"strcoll".as_ptr() as *mut i8, nargs, 2i32, 2i32) {
            if PyUnicode_Check!(unsafe { *args.wrapping_offset(0 as isize) }) == 0 {
                _PyArg_BadArgument(
                    c"strcoll".as_ptr(),
                    c"argument 1".as_ptr(),
                    c"str".as_ptr(),
                    *args.wrapping_offset(0 as isize),
                );
            } else {
                os1 = *args.wrapping_offset(0 as isize);
                if PyUnicode_Check!(unsafe { *args.wrapping_offset(1 as isize) }) == 0 {
                    _PyArg_BadArgument(
                        c"strcoll".as_ptr(),
                        c"argument 2".as_ptr(),
                        c"str".as_ptr(),
                        *args.wrapping_offset(1 as isize),
                    );
                } else {
                    let os2: *mut _object = *args.wrapping_offset(1 as isize);
                    return_value = _locale_strcoll_impl(module, os1, os2);
                }
            }
        }
        return return_value;
    }
}

static _locale_strxfrm__doc__: __SlateAlign16<[i8; 104]> = __SlateAlign16([
    115i8, 116i8, 114i8, 120i8, 102i8, 114i8, 109i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8,
    101i8, 44i8, 32i8, 115i8, 116i8, 114i8, 105i8, 110i8, 103i8, 44i8, 32i8, 47i8, 41i8, 10i8,
    45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 97i8, 32i8, 115i8,
    116i8, 114i8, 105i8, 110i8, 103i8, 32i8, 116i8, 104i8, 97i8, 116i8, 32i8, 99i8, 97i8, 110i8,
    32i8, 98i8, 101i8, 32i8, 117i8, 115i8, 101i8, 100i8, 32i8, 97i8, 115i8, 32i8, 97i8, 32i8,
    107i8, 101i8, 121i8, 32i8, 102i8, 111i8, 114i8, 32i8, 108i8, 111i8, 99i8, 97i8, 108i8, 101i8,
    45i8, 97i8, 119i8, 97i8, 114i8, 101i8, 32i8, 99i8, 111i8, 109i8, 112i8, 97i8, 114i8, 105i8,
    115i8, 111i8, 110i8, 115i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct._localemodule._locale_strxfrm")]
extern "C-unwind" fn _locale_strxfrm(module: *mut _object, arg: *mut _object) -> *mut _object {
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        if PyUnicode_Check!(arg) == 0 {
            _PyArg_BadArgument(
                c"strxfrm".as_ptr(),
                c"argument".as_ptr(),
                c"str".as_ptr(),
                arg,
            );
        } else {
            let str: *mut _object = arg;
            return_value = _locale_strxfrm_impl(module, str);
        }
        return return_value;
    }
}

static _locale_nl_langinfo__doc__: __SlateAlign16<[i8; 98]> = __SlateAlign16([
    110i8, 108i8, 95i8, 108i8, 97i8, 110i8, 103i8, 105i8, 110i8, 102i8, 111i8, 40i8, 36i8, 109i8,
    111i8, 100i8, 117i8, 108i8, 101i8, 44i8, 32i8, 107i8, 101i8, 121i8, 44i8, 32i8, 47i8, 41i8,
    10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 116i8, 104i8,
    101i8, 32i8, 118i8, 97i8, 108i8, 117i8, 101i8, 32i8, 102i8, 111i8, 114i8, 32i8, 116i8, 104i8,
    101i8, 32i8, 108i8, 111i8, 99i8, 97i8, 108i8, 101i8, 32i8, 105i8, 110i8, 102i8, 111i8, 114i8,
    109i8, 97i8, 116i8, 105i8, 111i8, 110i8, 32i8, 97i8, 115i8, 115i8, 111i8, 99i8, 105i8, 97i8,
    116i8, 101i8, 100i8, 32i8, 119i8, 105i8, 116i8, 104i8, 32i8, 107i8, 101i8, 121i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct._localemodule._locale_nl_langinfo")]
extern "C-unwind" fn _locale_nl_langinfo(module: *mut _object, arg: *mut _object) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut item: i32 = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        item = PyLong_AsInt(arg);
        if item == 1i32.wrapping_neg() {
            __v0 = !PyErr_Occurred().is_null();
        } else {
            __v0 = false;
        }
        if !__v0 {
            return_value = _locale_nl_langinfo_impl(module, item);
        }
        return return_value;
    }
}

use slate_support::localeconv;
use slate_support::memcpy;
use slate_support::nl_langinfo;
use slate_support::setlocale;
use slate_support::strcmp;
use slate_support::strlen;
use slate_support::wcslen;

static _locale_gettext__doc__: __SlateAlign16<[i8; 80]> = __SlateAlign16([
    103i8, 101i8, 116i8, 116i8, 101i8, 120i8, 116i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8,
    101i8, 44i8, 32i8, 109i8, 115i8, 103i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8,
    103i8, 101i8, 116i8, 116i8, 101i8, 120i8, 116i8, 40i8, 109i8, 115i8, 103i8, 41i8, 32i8, 45i8,
    62i8, 32i8, 115i8, 116i8, 114i8, 105i8, 110i8, 103i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8,
    114i8, 110i8, 32i8, 116i8, 114i8, 97i8, 110i8, 115i8, 108i8, 97i8, 116i8, 105i8, 111i8, 110i8,
    32i8, 111i8, 102i8, 32i8, 109i8, 115i8, 103i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct._localemodule._locale_gettext")]
extern "C-unwind" fn _locale_gettext(module: *mut _object, arg: *mut _object) -> *mut _object {
    let mut in_length: i64 = unsafe { core::mem::zeroed() };
    let mut r#in: *const i8 = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        if PyUnicode_Check!(arg) == 0 {
            _PyArg_BadArgument(
                c"gettext".as_ptr(),
                c"argument".as_ptr(),
                c"str".as_ptr(),
                arg,
            );
        } else {
            r#in = PyUnicode_AsUTF8AndSize(arg, core::ptr::addr_of_mut!(in_length));
            if !r#in.is_null() {
                if (strlen(r#in as *const core::ffi::c_char) as u64) != (in_length as u64) {
                    PyErr_SetString(PyExc_ValueError, c"embedded null character".as_ptr());
                } else {
                    return_value = _locale_gettext_impl(module, r#in);
                }
            }
        }
        return return_value;
    }
}

static _locale_dgettext__doc__: __SlateAlign16<[i8; 108]> = __SlateAlign16([
    100i8, 103i8, 101i8, 116i8, 116i8, 101i8, 120i8, 116i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8,
    108i8, 101i8, 44i8, 32i8, 100i8, 111i8, 109i8, 97i8, 105i8, 110i8, 44i8, 32i8, 109i8, 115i8,
    103i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 100i8, 103i8, 101i8, 116i8, 116i8,
    101i8, 120i8, 116i8, 40i8, 100i8, 111i8, 109i8, 97i8, 105i8, 110i8, 44i8, 32i8, 109i8, 115i8,
    103i8, 41i8, 32i8, 45i8, 62i8, 32i8, 115i8, 116i8, 114i8, 105i8, 110i8, 103i8, 10i8, 10i8,
    82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 116i8, 114i8, 97i8, 110i8, 115i8, 108i8, 97i8,
    116i8, 105i8, 111i8, 110i8, 32i8, 111i8, 102i8, 32i8, 109i8, 115i8, 103i8, 32i8, 105i8, 110i8,
    32i8, 100i8, 111i8, 109i8, 97i8, 105i8, 110i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct._localemodule._locale_dgettext")]
extern "C-unwind" fn _locale_dgettext(
    module: *mut _object,
    args: *const *mut _object,
    nargs: i64,
) -> *mut _object {
    let mut in_length: i64 = unsafe { core::mem::zeroed() };
    let mut domain_length: i64 = unsafe { core::mem::zeroed() };
    let mut r#in: *const i8 = unsafe { core::mem::zeroed() };
    let mut domain: *const i8 = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_0: {
            return_value = core::ptr::null_mut::<_object>();
            if _PyArg_CheckPositional!(c"dgettext".as_ptr() as *mut i8, nargs, 2i32, 2i32) {
                if *args.wrapping_offset(0 as isize) == core::ptr::addr_of_mut!(_Py_NoneStruct) {
                    domain = core::ptr::null::<i8>();
                } else {
                    if PyUnicode_Check!(unsafe { *args.wrapping_offset(0 as isize) }) != 0 {
                        domain = PyUnicode_AsUTF8AndSize(
                            *args.wrapping_offset(0 as isize),
                            core::ptr::addr_of_mut!(domain_length),
                        );
                        if domain.is_null() {
                            break '__join_0;
                        } else {
                            if (strlen(domain as *const core::ffi::c_char) as u64)
                                != (domain_length as u64)
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
                            c"dgettext".as_ptr(),
                            c"argument 1".as_ptr(),
                            c"str or None".as_ptr(),
                            *args.wrapping_offset(0 as isize),
                        );
                        break '__join_0;
                    }
                }
                if PyUnicode_Check!(unsafe { *args.wrapping_offset(1 as isize) }) == 0 {
                    _PyArg_BadArgument(
                        c"dgettext".as_ptr(),
                        c"argument 2".as_ptr(),
                        c"str".as_ptr(),
                        *args.wrapping_offset(1 as isize),
                    );
                } else {
                    r#in = PyUnicode_AsUTF8AndSize(
                        *args.wrapping_offset(1 as isize),
                        core::ptr::addr_of_mut!(in_length),
                    );
                    if !r#in.is_null() {
                        if (strlen(r#in as *const core::ffi::c_char) as u64) != (in_length as u64) {
                            PyErr_SetString(PyExc_ValueError, c"embedded null character".as_ptr());
                        } else {
                            return_value = _locale_dgettext_impl(module, domain, r#in);
                        }
                    }
                }
            }
        }
        return return_value;
    }
}

static _locale_dcgettext__doc__: __SlateAlign16<[i8; 99]> = __SlateAlign16([
    100i8, 99i8, 103i8, 101i8, 116i8, 116i8, 101i8, 120i8, 116i8, 40i8, 36i8, 109i8, 111i8, 100i8,
    117i8, 108i8, 101i8, 44i8, 32i8, 100i8, 111i8, 109i8, 97i8, 105i8, 110i8, 44i8, 32i8, 109i8,
    115i8, 103i8, 44i8, 32i8, 99i8, 97i8, 116i8, 101i8, 103i8, 111i8, 114i8, 121i8, 44i8, 32i8,
    47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 116i8,
    114i8, 97i8, 110i8, 115i8, 108i8, 97i8, 116i8, 105i8, 111i8, 110i8, 32i8, 111i8, 102i8, 32i8,
    109i8, 115i8, 103i8, 32i8, 105i8, 110i8, 32i8, 100i8, 111i8, 109i8, 97i8, 105i8, 110i8, 32i8,
    97i8, 110i8, 100i8, 32i8, 99i8, 97i8, 116i8, 101i8, 103i8, 111i8, 114i8, 121i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct._localemodule._locale_dcgettext")]
extern "C-unwind" fn _locale_dcgettext(
    module: *mut _object,
    args: *const *mut _object,
    nargs: i64,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut msgid_length: i64 = unsafe { core::mem::zeroed() };
    let mut domain_length: i64 = unsafe { core::mem::zeroed() };
    let mut category: i32 = unsafe { core::mem::zeroed() };
    let mut msgid: *const i8 = unsafe { core::mem::zeroed() };
    let mut domain: *const i8 = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_0: {
            return_value = core::ptr::null_mut::<_object>();
            if _PyArg_CheckPositional!(c"dcgettext".as_ptr() as *mut i8, nargs, 3i32, 3i32) {
                if *args.wrapping_offset(0 as isize) == core::ptr::addr_of_mut!(_Py_NoneStruct) {
                    domain = core::ptr::null::<i8>();
                } else {
                    if PyUnicode_Check!(unsafe { *args.wrapping_offset(0 as isize) }) != 0 {
                        domain = PyUnicode_AsUTF8AndSize(
                            *args.wrapping_offset(0 as isize),
                            core::ptr::addr_of_mut!(domain_length),
                        );
                        if domain.is_null() {
                            break '__join_0;
                        } else {
                            if (strlen(domain as *const core::ffi::c_char) as u64)
                                != (domain_length as u64)
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
                            c"dcgettext".as_ptr(),
                            c"argument 1".as_ptr(),
                            c"str or None".as_ptr(),
                            *args.wrapping_offset(0 as isize),
                        );
                        break '__join_0;
                    }
                }
                if PyUnicode_Check!(unsafe { *args.wrapping_offset(1 as isize) }) == 0 {
                    _PyArg_BadArgument(
                        c"dcgettext".as_ptr(),
                        c"argument 2".as_ptr(),
                        c"str".as_ptr(),
                        *args.wrapping_offset(1 as isize),
                    );
                } else {
                    msgid = PyUnicode_AsUTF8AndSize(
                        *args.wrapping_offset(1 as isize),
                        core::ptr::addr_of_mut!(msgid_length),
                    );
                    if !msgid.is_null() {
                        if (strlen(msgid as *const core::ffi::c_char) as u64)
                            != (msgid_length as u64)
                        {
                            PyErr_SetString(PyExc_ValueError, c"embedded null character".as_ptr());
                        } else {
                            category = PyLong_AsInt(*args.wrapping_offset(2 as isize));
                            if category == 1i32.wrapping_neg() {
                                __v0 = !PyErr_Occurred().is_null();
                            } else {
                                __v0 = false;
                            }
                            if !__v0 {
                                return_value =
                                    _locale_dcgettext_impl(module, domain, msgid, category);
                            }
                        }
                    }
                }
            }
        }
        return return_value;
    }
}

static _locale_textdomain__doc__: __SlateAlign16<[i8; 102]> = __SlateAlign16([
    116i8, 101i8, 120i8, 116i8, 100i8, 111i8, 109i8, 97i8, 105i8, 110i8, 40i8, 36i8, 109i8, 111i8,
    100i8, 117i8, 108i8, 101i8, 44i8, 32i8, 100i8, 111i8, 109i8, 97i8, 105i8, 110i8, 44i8, 32i8,
    47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 83i8, 101i8, 116i8, 32i8, 116i8, 104i8, 101i8, 32i8,
    67i8, 32i8, 108i8, 105i8, 98i8, 114i8, 97i8, 114i8, 121i8, 39i8, 115i8, 32i8, 116i8, 101i8,
    120i8, 116i8, 100i8, 109i8, 97i8, 105i8, 110i8, 32i8, 116i8, 111i8, 32i8, 100i8, 111i8, 109i8,
    97i8, 105i8, 110i8, 44i8, 32i8, 114i8, 101i8, 116i8, 117i8, 114i8, 110i8, 105i8, 110i8, 103i8,
    32i8, 116i8, 104i8, 101i8, 32i8, 110i8, 101i8, 119i8, 32i8, 100i8, 111i8, 109i8, 97i8, 105i8,
    110i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct._localemodule._locale_textdomain")]
extern "C-unwind" fn _locale_textdomain(module: *mut _object, arg: *mut _object) -> *mut _object {
    let mut domain_length: i64 = unsafe { core::mem::zeroed() };
    let mut domain: *const i8 = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_0: {
            return_value = core::ptr::null_mut::<_object>();
            if arg == core::ptr::addr_of_mut!(_Py_NoneStruct) {
                domain = core::ptr::null::<i8>();
            } else {
                if PyUnicode_Check!(arg) != 0 {
                    domain = PyUnicode_AsUTF8AndSize(arg, core::ptr::addr_of_mut!(domain_length));
                    if domain.is_null() {
                        break '__join_0;
                    } else {
                        if (strlen(domain as *const core::ffi::c_char) as u64)
                            != (domain_length as u64)
                        {
                            PyErr_SetString(PyExc_ValueError, c"embedded null character".as_ptr());
                            break '__join_0;
                        }
                    }
                } else {
                    _PyArg_BadArgument(
                        c"textdomain".as_ptr(),
                        c"argument".as_ptr(),
                        c"str or None".as_ptr(),
                        arg,
                    );
                    break '__join_0;
                }
            }
            return_value = _locale_textdomain_impl(module, domain);
        }
        return return_value;
    }
}

static _locale_bindtextdomain__doc__: __SlateAlign16<[i8; 80]> = __SlateAlign16([
    98i8, 105i8, 110i8, 100i8, 116i8, 101i8, 120i8, 116i8, 100i8, 111i8, 109i8, 97i8, 105i8, 110i8,
    40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 44i8, 32i8, 100i8, 111i8, 109i8, 97i8,
    105i8, 110i8, 44i8, 32i8, 100i8, 105i8, 114i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8,
    10i8, 66i8, 105i8, 110i8, 100i8, 32i8, 116i8, 104i8, 101i8, 32i8, 67i8, 32i8, 108i8, 105i8,
    98i8, 114i8, 97i8, 114i8, 121i8, 39i8, 115i8, 32i8, 100i8, 111i8, 109i8, 97i8, 105i8, 110i8,
    32i8, 116i8, 111i8, 32i8, 100i8, 105i8, 114i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct._localemodule._locale_bindtextdomain")]
extern "C-unwind" fn _locale_bindtextdomain(
    module: *mut _object,
    args: *const *mut _object,
    nargs: i64,
) -> *mut _object {
    let mut domain_length: i64 = unsafe { core::mem::zeroed() };
    let mut domain: *const i8 = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        if _PyArg_CheckPositional!(c"bindtextdomain".as_ptr() as *mut i8, nargs, 2i32, 2i32) {
            if PyUnicode_Check!(unsafe { *args.wrapping_offset(0 as isize) }) == 0 {
                _PyArg_BadArgument(
                    c"bindtextdomain".as_ptr(),
                    c"argument 1".as_ptr(),
                    c"str".as_ptr(),
                    *args.wrapping_offset(0 as isize),
                );
            } else {
                domain = PyUnicode_AsUTF8AndSize(
                    *args.wrapping_offset(0 as isize),
                    core::ptr::addr_of_mut!(domain_length),
                );
                if !domain.is_null() {
                    if (strlen(domain as *const core::ffi::c_char) as u64) != (domain_length as u64)
                    {
                        PyErr_SetString(PyExc_ValueError, c"embedded null character".as_ptr());
                    } else {
                        let dirname_obj: *mut _object = *args.wrapping_offset(1 as isize);
                        return_value = _locale_bindtextdomain_impl(module, domain, dirname_obj);
                    }
                }
            }
        }
        return return_value;
    }
}

static _locale_bind_textdomain_codeset__doc__: __SlateAlign16<[i8; 97]> = __SlateAlign16([
    98i8, 105i8, 110i8, 100i8, 95i8, 116i8, 101i8, 120i8, 116i8, 100i8, 111i8, 109i8, 97i8, 105i8,
    110i8, 95i8, 99i8, 111i8, 100i8, 101i8, 115i8, 101i8, 116i8, 40i8, 36i8, 109i8, 111i8, 100i8,
    117i8, 108i8, 101i8, 44i8, 32i8, 100i8, 111i8, 109i8, 97i8, 105i8, 110i8, 44i8, 32i8, 99i8,
    111i8, 100i8, 101i8, 115i8, 101i8, 116i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8,
    66i8, 105i8, 110i8, 100i8, 32i8, 116i8, 104i8, 101i8, 32i8, 67i8, 32i8, 108i8, 105i8, 98i8,
    114i8, 97i8, 114i8, 121i8, 39i8, 115i8, 32i8, 100i8, 111i8, 109i8, 97i8, 105i8, 110i8, 32i8,
    116i8, 111i8, 32i8, 99i8, 111i8, 100i8, 101i8, 115i8, 101i8, 116i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct._localemodule._locale_bind_textdomain_codeset")]
extern "C-unwind" fn _locale_bind_textdomain_codeset(
    module: *mut _object,
    args: *const *mut _object,
    nargs: i64,
) -> *mut _object {
    let mut codeset_length: i64 = unsafe { core::mem::zeroed() };
    let mut domain_length: i64 = unsafe { core::mem::zeroed() };
    let mut codeset: *const i8 = unsafe { core::mem::zeroed() };
    let mut domain: *const i8 = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_0: {
            return_value = core::ptr::null_mut::<_object>();
            if _PyArg_CheckPositional!(
                c"bind_textdomain_codeset".as_ptr() as *mut i8,
                nargs,
                2i32,
                2i32
            ) {
                if PyUnicode_Check!(unsafe { *args.wrapping_offset(0 as isize) }) == 0 {
                    _PyArg_BadArgument(
                        c"bind_textdomain_codeset".as_ptr(),
                        c"argument 1".as_ptr(),
                        c"str".as_ptr(),
                        *args.wrapping_offset(0 as isize),
                    );
                } else {
                    domain = PyUnicode_AsUTF8AndSize(
                        *args.wrapping_offset(0 as isize),
                        core::ptr::addr_of_mut!(domain_length),
                    );
                    if !domain.is_null() {
                        if (strlen(domain as *const core::ffi::c_char) as u64)
                            != (domain_length as u64)
                        {
                            PyErr_SetString(PyExc_ValueError, c"embedded null character".as_ptr());
                        } else {
                            if *args.wrapping_offset(1 as isize)
                                == core::ptr::addr_of_mut!(_Py_NoneStruct)
                            {
                                codeset = core::ptr::null::<i8>();
                            } else {
                                if PyUnicode_Check!(unsafe { *args.wrapping_offset(1 as isize) })
                                    != 0
                                {
                                    codeset = PyUnicode_AsUTF8AndSize(
                                        *args.wrapping_offset(1 as isize),
                                        core::ptr::addr_of_mut!(codeset_length),
                                    );
                                    if codeset.is_null() {
                                        break '__join_0;
                                    } else {
                                        if (strlen(codeset as *const core::ffi::c_char) as u64)
                                            != (codeset_length as u64)
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
                                        c"bind_textdomain_codeset".as_ptr(),
                                        c"argument 2".as_ptr(),
                                        c"str or None".as_ptr(),
                                        *args.wrapping_offset(1 as isize),
                                    );
                                    break '__join_0;
                                }
                            }
                            return_value =
                                _locale_bind_textdomain_codeset_impl(module, domain, codeset);
                        }
                    }
                }
            }
        }
        return return_value;
    }
}

static _locale_getencoding__doc__: __SlateAlign16<[i8; 61]> = __SlateAlign16([
    103i8, 101i8, 116i8, 101i8, 110i8, 99i8, 111i8, 100i8, 105i8, 110i8, 103i8, 40i8, 36i8, 109i8,
    111i8, 100i8, 117i8, 108i8, 101i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 71i8,
    101i8, 116i8, 32i8, 116i8, 104i8, 101i8, 32i8, 99i8, 117i8, 114i8, 114i8, 101i8, 110i8, 116i8,
    32i8, 108i8, 111i8, 99i8, 97i8, 108i8, 101i8, 32i8, 101i8, 110i8, 99i8, 111i8, 100i8, 105i8,
    110i8, 103i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct._localemodule._locale_getencoding")]
extern "C-unwind" fn _locale_getencoding(
    module: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    _locale_getencoding_impl(module)
}

// [clinic input]
// module _locale
// [clinic start generated code]
//
// [clinic end generated code: output=da39a3ee5e6b4b0d input=ed98569b726feada]
// support functions for formatting floating-point numbers
/// the grouping is terminated by either 0 or CHAR_MAX
fn copy_grouping(s: *const i8) -> *mut _object {
    let mut i: i32 = 0;
    let mut val: *mut _object = core::ptr::null_mut::<_object>();
    if ((unsafe { *s.wrapping_offset(0 as isize) }) as i32) == 0 {
        // empty string: no grouping at all
        return unsafe { PyList_New(0i64) };
    }
    while ((unsafe { *s.wrapping_offset(i as isize) }) as i32) != 0
        && ((unsafe { *s.wrapping_offset(i as isize) }) as i32) != 127
    {
        i = i.wrapping_add(1);
    }
    // nothing
    let result: *mut _object = unsafe { PyList_New(i.wrapping_add(1) as i64) };
    if result.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    i = 1i32.wrapping_neg();
    loop {
        i = i.wrapping_add(1);
        val = unsafe { PyLong_FromLong(*s.wrapping_offset(i as isize) as i64) };
        if val.is_null() {
            Py_DECREF!(result);
            return core::ptr::null_mut::<_object>();
        }
        PyList_SET_ITEM__2!(result, i, val);
        if !(((unsafe { *s.wrapping_offset(i as isize) }) as i32) != 0
            && ((unsafe { *s.wrapping_offset(i as isize) }) as i32) != 127)
        {
            break;
        }
    }
    result
}

// [clinic input]
// _locale.setlocale
//
//     category: int
//     locale: str(accept={str, NoneType}) = None
//     /
//
// Activates/queries locale processing.
// [clinic start generated code]
/// [clinic end generated code: output=a0e777ae5d2ff117 input=b53449b63407179b]
fn _locale_setlocale_impl(module: *mut _object, category: i32, locale: *const i8) -> *mut _object {
    let mut result: *mut i8 = unsafe { core::mem::zeroed() };
    let mut result_object: *mut _object = unsafe { core::mem::zeroed() };
    if !locale.is_null() {
        // set locale
        result =
            unsafe { setlocale(category as i32, locale as *const core::ffi::c_char) as *mut i8 };
        if result.is_null() {
            // operation failed, no setting was changed
            unsafe {
                PyErr_SetString(
                    (*get_locale_state(module)).Error,
                    c"unsupported locale setting".as_ptr(),
                )
            };
            return core::ptr::null_mut::<_object>();
        }
        result_object =
            unsafe { PyUnicode_DecodeLocale(result as *const i8, core::ptr::null::<i8>()) };
        if result_object.is_null() {
            return core::ptr::null_mut::<_object>();
        }
    } else {
        // get locale
        result = unsafe {
            setlocale(
                category as i32,
                core::ptr::null::<i8>() as *const core::ffi::c_char,
            ) as *mut i8
        };
        if result.is_null() {
            unsafe {
                PyErr_SetString(
                    (*get_locale_state(module)).Error,
                    c"locale query failed".as_ptr(),
                )
            };
            return core::ptr::null_mut::<_object>();
        }
        result_object =
            unsafe { PyUnicode_DecodeLocale(result as *const i8, core::ptr::null::<i8>()) };
    }
    result_object
}

fn locale_is_ascii(str: *const i8) -> i32 {
    ((unsafe { strlen(str as *const core::ffi::c_char) as u64 }) == 1
        && (((unsafe { *str.wrapping_offset(0 as isize) }) as u8) as i32) <= 127) as i32
}

fn is_all_ascii(mut str: *const i8) -> i32 {
    while (unsafe { *str }) != 0 {
        if (((unsafe { *str }) as u8) as i32) > 127 {
            return 0;
        }
        let __v1: *const i8 = str;
        str = __v1.wrapping_offset(1 as isize);
    }
    1
}

fn locale_decode_monetary(dict: *mut _object, lc: *mut slate_support::lconv) -> i32 {
    let mut obj: *mut _object = unsafe { core::mem::zeroed() };
    let mut obj_1: *mut _object = unsafe { core::mem::zeroed() };
    let mut obj_2: *mut _object = unsafe { core::mem::zeroed() };
    let mut obj_3: *mut _object = unsafe { core::mem::zeroed() };
    let mut res: i32 = unsafe { core::mem::zeroed() };
    let mut loc: *mut i8 = unsafe { core::mem::zeroed() };
    // Keep a copy of the LC_CTYPE locale
    let mut oldloc: *mut i8 = unsafe { core::mem::zeroed() };
    let mut __v7: bool = unsafe { core::mem::zeroed() };
    let mut __v8: bool = unsafe { core::mem::zeroed() };
    let mut __v9: bool = unsafe { core::mem::zeroed() };
    unsafe {
        if locale_is_ascii((*lc).int_curr_symbol as *const i8) == 0 {
            __v9 = true;
        } else {
            __v9 = locale_is_ascii((*lc).currency_symbol as *const i8) == 0;
        }
        if __v9 {
            __v8 = true;
        } else {
            __v8 = locale_is_ascii((*lc).mon_decimal_point as *const i8) == 0;
        }
        if __v8 {
            __v7 = true;
        } else {
            __v7 = locale_is_ascii((*lc).mon_thousands_sep as *const i8) == 0;
        }
        let change_locale: i32 = __v7 as i32;
        oldloc = core::ptr::null_mut::<i8>();
        loc = core::ptr::null_mut::<i8>();
        if change_locale != 0 {
            oldloc = setlocale(
                0i32 as i32,
                core::ptr::null::<i8>() as *const core::ffi::c_char,
            ) as *mut i8;
            if oldloc.is_null() {
                PyErr_SetString(
                    PyExc_RuntimeWarning,
                    c"failed to get LC_CTYPE locale".as_ptr(),
                );
                return 1i32.wrapping_neg();
            } else {
                oldloc = _PyMem_Strdup(oldloc as *const i8);
                if oldloc.is_null() {
                    PyErr_NoMemory();
                    return 1i32.wrapping_neg();
                } else {
                    loc = setlocale(
                        4i32 as i32,
                        core::ptr::null::<i8>() as *const core::ffi::c_char,
                    ) as *mut i8;
                    if !loc.is_null()
                        && (strcmp(
                            (loc as *const i8) as *const core::ffi::c_char,
                            (oldloc as *const i8) as *const core::ffi::c_char,
                        ) as i32)
                            == 0
                    {
                        loc = core::ptr::null_mut::<i8>();
                    }
                    if !loc.is_null() {
                        // Only set the locale temporarily the LC_CTYPE locale
                        // to the LC_MONETARY locale if the two locales are different and
                        // at least one string is non-ASCII.
                        setlocale(0i32 as i32, (loc as *const i8) as *const core::ffi::c_char)
                            as *mut i8;
                    }
                }
            }
        }
        res = 1i32.wrapping_neg();
        '__join_2: {
            '__join_17: {
                loop {
                    obj_3 = GET_LOCALE_STRING!(int_curr_symbol, lc);
                    if obj_3.is_null() {
                        break '__join_2;
                    } else {
                        if PyDict_SetItemString(dict, Py_STRINGIFY__8!() as *const i8, obj_3) < 0 {
                            break '__join_17;
                        } else {
                            Py_DECREF!(obj_3);
                            if true {
                                break;
                            }
                        }
                    }
                }
                '__join_13: {
                    loop {
                        obj_2 = GET_LOCALE_STRING!(currency_symbol, lc);
                        if obj_2.is_null() {
                            break '__join_2;
                        } else {
                            if PyDict_SetItemString(dict, Py_STRINGIFY__22!() as *const i8, obj_2)
                                < 0
                            {
                                break '__join_13;
                            } else {
                                Py_DECREF!(obj_2);
                                if true {
                                    break;
                                }
                            }
                        }
                    }
                    '__join_9: {
                        loop {
                            obj_1 = GET_LOCALE_STRING!(mon_decimal_point, lc);
                            if obj_1.is_null() {
                                break '__join_2;
                            } else {
                                if PyDict_SetItemString(
                                    dict,
                                    Py_STRINGIFY__40!() as *const i8,
                                    obj_1,
                                ) < 0
                                {
                                    break '__join_9;
                                } else {
                                    Py_DECREF!(obj_1);
                                    if true {
                                        break;
                                    }
                                }
                            }
                        }
                        '__join_5: {
                            loop {
                                obj = GET_LOCALE_STRING!(mon_thousands_sep, lc);
                                if obj.is_null() {
                                    break '__join_2;
                                } else {
                                    if PyDict_SetItemString(
                                        dict,
                                        Py_STRINGIFY__34!() as *const i8,
                                        obj,
                                    ) < 0
                                    {
                                        break '__join_5;
                                    } else {
                                        Py_DECREF!(obj);
                                        if true {
                                            break;
                                        }
                                    }
                                }
                            }
                            res = 0;
                            break '__join_2;
                        }
                        Py_DECREF!(obj);
                        break '__join_2;
                    }
                    Py_DECREF!(obj_1);
                    break '__join_2;
                }
                Py_DECREF!(obj_2);
                break '__join_2;
            }
            Py_DECREF!(obj_3);
        }
        if !loc.is_null() {
            setlocale(
                0i32 as i32,
                (oldloc as *const i8) as *const core::ffi::c_char,
            ) as *mut i8;
        }
        PyMem_Free(oldloc as *mut ());
        return res;
    }
}

// [clinic input]
// _locale.localeconv
//
// Returns numeric and monetary locale-specific parameters.
// [clinic start generated code]
/// [clinic end generated code: output=43a54515e0a2aef5 input=f1132d15accf4444]
fn _locale_localeconv_impl(module: *mut _object) -> *mut _object {
    let mut thousands_sep: *mut _object = unsafe { core::mem::zeroed() };
    // Numeric information: LC_NUMERIC encoding
    let mut decimal_point: *mut _object = unsafe { core::mem::zeroed() };
    let mut x: *mut _object = unsafe { core::mem::zeroed() };
    let mut lc: *mut slate_support::lconv = unsafe { core::mem::zeroed() };
    let mut result: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        result = PyDict_New();
        if result.is_null() {
            return core::ptr::null_mut::<_object>();
        } else {
            '__join_0: {
                // if LC_NUMERIC is different in the C library, use saved value
                lc = localeconv() as *mut slate_support::lconv;
                // hopefully, the localeconv result survives the C library calls
                // involved herein
                // Monetary information: LC_MONETARY encoding
                if locale_decode_monetary(result, lc) >= 0 {
                    x = copy_grouping((*lc).mon_grouping as *const i8);
                    '__join_74: {
                        loop {
                            if x.is_null() {
                                break '__join_0;
                            } else {
                                if PyDict_SetItemString(result, c"mon_grouping".as_ptr(), x) < 0 {
                                    break '__join_74;
                                } else {
                                    Py_DECREF!(x);
                                    if true {
                                        break;
                                    }
                                }
                            }
                        }
                        '__join_69: {
                            loop {
                                x = GET_LOCALE_STRING!(positive_sign, lc);
                                loop {
                                    if x.is_null() {
                                        break '__join_0;
                                    } else {
                                        if PyDict_SetItemString(
                                            result,
                                            c"positive_sign".as_ptr(),
                                            x,
                                        ) < 0
                                        {
                                            break '__join_69;
                                        } else {
                                            Py_DECREF!(x);
                                            if true {
                                                break;
                                            }
                                        }
                                    }
                                }
                                if true {
                                    break;
                                }
                            }
                            '__join_63: {
                                loop {
                                    x = GET_LOCALE_STRING!(negative_sign, lc);
                                    loop {
                                        if x.is_null() {
                                            break '__join_0;
                                        } else {
                                            if PyDict_SetItemString(
                                                result,
                                                c"negative_sign".as_ptr(),
                                                x,
                                            ) < 0
                                            {
                                                break '__join_63;
                                            } else {
                                                Py_DECREF!(x);
                                                if true {
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                    if true {
                                        break;
                                    }
                                }
                                '__join_57: {
                                    loop {
                                        x = PyLong_FromLong((*lc).int_frac_digits as i64);
                                        loop {
                                            if x.is_null() {
                                                break '__join_0;
                                            } else {
                                                if PyDict_SetItemString(
                                                    result,
                                                    c"int_frac_digits".as_ptr(),
                                                    x,
                                                ) < 0
                                                {
                                                    break '__join_57;
                                                } else {
                                                    Py_DECREF!(x);
                                                    if true {
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                        if true {
                                            break;
                                        }
                                    }
                                    '__join_51: {
                                        loop {
                                            x = PyLong_FromLong((*lc).frac_digits as i64);
                                            loop {
                                                if x.is_null() {
                                                    break '__join_0;
                                                } else {
                                                    if PyDict_SetItemString(
                                                        result,
                                                        c"frac_digits".as_ptr(),
                                                        x,
                                                    ) < 0
                                                    {
                                                        break '__join_51;
                                                    } else {
                                                        Py_DECREF!(x);
                                                        if true {
                                                            break;
                                                        }
                                                    }
                                                }
                                            }
                                            if true {
                                                break;
                                            }
                                        }
                                        '__join_45: {
                                            loop {
                                                x = PyLong_FromLong((*lc).p_cs_precedes as i64);
                                                loop {
                                                    if x.is_null() {
                                                        break '__join_0;
                                                    } else {
                                                        if PyDict_SetItemString(
                                                            result,
                                                            c"p_cs_precedes".as_ptr(),
                                                            x,
                                                        ) < 0
                                                        {
                                                            break '__join_45;
                                                        } else {
                                                            Py_DECREF!(x);
                                                            if true {
                                                                break;
                                                            }
                                                        }
                                                    }
                                                }
                                                if true {
                                                    break;
                                                }
                                            }
                                            '__join_39: {
                                                loop {
                                                    x = PyLong_FromLong(
                                                        (*lc).p_sep_by_space as i64,
                                                    );
                                                    loop {
                                                        if x.is_null() {
                                                            break '__join_0;
                                                        } else {
                                                            if PyDict_SetItemString(
                                                                result,
                                                                c"p_sep_by_space".as_ptr(),
                                                                x,
                                                            ) < 0
                                                            {
                                                                break '__join_39;
                                                            } else {
                                                                Py_DECREF!(x);
                                                                if true {
                                                                    break;
                                                                }
                                                            }
                                                        }
                                                    }
                                                    if true {
                                                        break;
                                                    }
                                                }
                                                '__join_33: {
                                                    loop {
                                                        x = PyLong_FromLong(
                                                            (*lc).n_cs_precedes as i64,
                                                        );
                                                        loop {
                                                            if x.is_null() {
                                                                break '__join_0;
                                                            } else {
                                                                if PyDict_SetItemString(
                                                                    result,
                                                                    c"n_cs_precedes".as_ptr(),
                                                                    x,
                                                                ) < 0
                                                                {
                                                                    break '__join_33;
                                                                } else {
                                                                    Py_DECREF!(x);
                                                                    if true {
                                                                        break;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        if true {
                                                            break;
                                                        }
                                                    }
                                                    '__join_27: {
                                                        loop {
                                                            x = PyLong_FromLong(
                                                                (*lc).n_sep_by_space as i64,
                                                            );
                                                            loop {
                                                                if x.is_null() {
                                                                    break '__join_0;
                                                                } else {
                                                                    if PyDict_SetItemString(
                                                                        result,
                                                                        c"n_sep_by_space".as_ptr(),
                                                                        x,
                                                                    ) < 0
                                                                    {
                                                                        break '__join_27;
                                                                    } else {
                                                                        Py_DECREF!(x);
                                                                        if true {
                                                                            break;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            if true {
                                                                break;
                                                            }
                                                        }
                                                        '__join_21: {
                                                            loop {
                                                                x = PyLong_FromLong(
                                                                    (*lc).p_sign_posn as i64,
                                                                );
                                                                loop {
                                                                    if x.is_null() {
                                                                        break '__join_0;
                                                                    } else {
                                                                        if PyDict_SetItemString(
                                                                            result,
                                                                            c"p_sign_posn".as_ptr(),
                                                                            x,
                                                                        ) < 0
                                                                        {
                                                                            break '__join_21;
                                                                        } else {
                                                                            Py_DECREF!(x);
                                                                            if true {
                                                                                break;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                if true {
                                                                    break;
                                                                }
                                                            }
                                                            '__join_15: {
                                                                loop {
                                                                    x = PyLong_FromLong(
                                                                        (*lc).n_sign_posn as i64,
                                                                    );
                                                                    loop {
                                                                        if x.is_null() {
                                                                            break '__join_0;
                                                                        } else {
                                                                            if PyDict_SetItemString(
                                                                                result,
                                                                                c"n_sign_posn"
                                                                                    .as_ptr(),
                                                                                x,
                                                                            ) < 0
                                                                            {
                                                                                break '__join_15;
                                                                            } else {
                                                                                Py_DECREF!(x);
                                                                                if true {
                                                                                    break;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    if true {
                                                                        break;
                                                                    }
                                                                }
                                                                decimal_point =
                                                                    core::ptr::null_mut::<_object>(
                                                                    );
                                                                thousands_sep =
                                                                    core::ptr::null_mut::<_object>(
                                                                    );
                                                                if _Py_GetLocaleconvNumeric(
                                                                    lc,
                                                                    core::ptr::addr_of_mut!(
                                                                        decimal_point
                                                                    ),
                                                                    core::ptr::addr_of_mut!(
                                                                        thousands_sep
                                                                    ),
                                                                ) < 0
                                                                {
                                                                    Py_XDECREF!(decimal_point);
                                                                    Py_XDECREF!(thousands_sep);
                                                                    break '__join_0;
                                                                } else {
                                                                    if PyDict_SetItemString(
                                                                        result,
                                                                        c"decimal_point".as_ptr(),
                                                                        decimal_point,
                                                                    ) < 0
                                                                    {
                                                                        Py_DECREF!(decimal_point);
                                                                        Py_DECREF!(thousands_sep);
                                                                        break '__join_0;
                                                                    } else {
                                                                        Py_DECREF!(decimal_point);
                                                                        if PyDict_SetItemString(
                                                                            result,
                                                                            c"thousands_sep"
                                                                                .as_ptr(),
                                                                            thousands_sep,
                                                                        ) < 0
                                                                        {
                                                                            Py_DECREF!(
                                                                                thousands_sep
                                                                            );
                                                                            break '__join_0;
                                                                        } else {
                                                                            Py_DECREF!(
                                                                                thousands_sep
                                                                            );
                                                                            x = copy_grouping(
                                                                                (*lc).grouping
                                                                                    as *const i8,
                                                                            );
                                                                            '__join_3: {
                                                                                loop {
                                                                                    if x.is_null() {
                                                                                        break '__join_0;
                                                                                    } else {
                                                                                        if PyDict_SetItemString(result, c"grouping".as_ptr(), x) < 0 {
break '__join_3;
} else {
Py_DECREF!(x);
if true {
break;
}
}
                                                                                    }
                                                                                }
                                                                                return result;
                                                                            }
                                                                            Py_DECREF!(x);
                                                                            break '__join_0;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            Py_DECREF!(x);
                                                            break '__join_0;
                                                        }
                                                        Py_DECREF!(x);
                                                        break '__join_0;
                                                    }
                                                    Py_DECREF!(x);
                                                    break '__join_0;
                                                }
                                                Py_DECREF!(x);
                                                break '__join_0;
                                            }
                                            Py_DECREF!(x);
                                            break '__join_0;
                                        }
                                        Py_DECREF!(x);
                                        break '__join_0;
                                    }
                                    Py_DECREF!(x);
                                    break '__join_0;
                                }
                                Py_DECREF!(x);
                                break '__join_0;
                            }
                            Py_DECREF!(x);
                            break '__join_0;
                        }
                        Py_DECREF!(x);
                        break '__join_0;
                    }
                    Py_DECREF!(x);
                }
            }
            Py_DECREF!(result);
            return core::ptr::null_mut::<_object>();
        }
    }
}

// [clinic input]
// _locale.strcoll
//
//     os1: unicode
//     os2: unicode
//     /
//
// Compares two strings according to the locale.
// [clinic start generated code]
/// [clinic end generated code: output=82ddc6d62c76d618 input=693cd02bcbf38dd8]
fn _locale_strcoll_impl(
    module: *mut _object,
    os1: *mut _object,
    os2: *mut _object,
) -> *mut _object {
    let mut ws2: *mut i32 = unsafe { core::mem::zeroed() };
    let mut ws1: *mut i32 = unsafe { core::mem::zeroed() };
    let mut result: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        result = core::ptr::null_mut::<_object>();
        ws1 = core::ptr::null_mut::<i32>();
        ws2 = core::ptr::null_mut::<i32>();
        // Convert the unicode strings to wchar[].
        ws1 = PyUnicode_AsWideCharString(os1, core::ptr::null_mut::<i64>());
        if !ws1.is_null() {
            ws2 = PyUnicode_AsWideCharString(os2, core::ptr::null_mut::<i64>());
            if !ws2.is_null() {
                // Collate the strings.
                result = PyLong_FromLong(wcscoll(ws1 as *const i32, ws2 as *const i32) as i64);
            }
        }
        if !ws1.is_null() {
            PyMem_Free(ws1 as *mut ());
        }
        // Deallocate everything.
        if !ws2.is_null() {
            PyMem_Free(ws2 as *mut ());
        }
        return result;
    }
}

// [clinic input]
// _locale.strxfrm
//
//     string as str: unicode
//     /
//
// Return a string that can be used as a key for locale-aware comparisons.
// [clinic start generated code]
/// [clinic end generated code: output=3081866ebffc01af input=1378bbe6a88b4780]
fn _locale_strxfrm_impl(module: *mut _object, str: *mut _object) -> *mut _object {
    let mut c: u32 = unsafe { core::mem::zeroed() };
    let mut i: u64 = unsafe { core::mem::zeroed() };
    let mut j: u64 = unsafe { core::mem::zeroed() };
    let mut buf2: *mut u32 = unsafe { core::mem::zeroed() };
    let mut i_1: u64 = unsafe { core::mem::zeroed() };
    // The result is just a sequence of integers, they are not necessary
    // Unicode code points, so PyUnicode_FromWideChar() cannot be used
    // here. For example, 0xD83D 0xDC0D should not be larger than 0xFF41.
    // Some codes can exceed the range of Unicode code points
    // (0 - 0x10FFFF), so they cannot be directly used in
    // PyUnicode_FromKindAndData(). They should be first encoded in
    // a way that preserves the lexicographical order.
    //
    // Codes in the range 0-0xFFFF represent themself.
    // Codes larger than 0xFFFF are encoded as a pair:
    // * 0x1xxxx -- the highest 16 bits
    // * 0x0xxxx -- the lowest 16 bits
    let mut n3: u64 = unsafe { core::mem::zeroed() };
    // more space needed
    let mut new_buf: *mut i32 = unsafe { core::mem::zeroed() };
    let mut result: *mut _object = unsafe { core::mem::zeroed() };
    let mut n2: u64 = unsafe { core::mem::zeroed() };
    let mut buf: *mut i32 = unsafe { core::mem::zeroed() };
    let mut s: *mut i32 = unsafe { core::mem::zeroed() };
    let mut n1: i64 = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_0: {
            s = core::ptr::null_mut::<i32>();
            buf = core::ptr::null_mut::<i32>();
            result = core::ptr::null_mut::<_object>();
            s = PyUnicode_AsWideCharString(str, core::ptr::addr_of_mut!(n1));
            if !s.is_null() {
                if (wcslen((s as *const i32) as *const i32) as u64) != (n1 as u64) {
                    PyErr_SetString(PyExc_ValueError, c"embedded null character".as_ptr());
                } else {
                    // assume no change in size, first
                    n1 = n1.wrapping_add(1);
                    // Yet another +1 is needed to work around a platform bug in wcsxfrm()
                    // on macOS. See gh-130567.
                    buf = PyMem_New__3!(i32, n1.wrapping_add(1));
                    if buf.is_null() {
                        PyErr_NoMemory();
                    } else {
                        *slate_support::errno_location() = 0;
                        n2 = wcsxfrm(buf, s as *const i32, n1 as u64);
                        if *slate_support::errno_location() != 0
                            && *slate_support::errno_location() != (slate_support::ERANGE as i32)
                        {
                            PyErr_SetFromErrno(PyExc_OSError);
                        } else {
                            if n2 >= (n1 as u64) {
                                new_buf = PyMem_Realloc(
                                    buf as *mut (),
                                    n2.wrapping_add(1)
                                        .wrapping_mul(core::mem::size_of::<i32>() as u64),
                                ) as *mut i32;
                                if new_buf.is_null() {
                                    PyErr_NoMemory();
                                    break '__join_0;
                                } else {
                                    buf = new_buf;
                                    *slate_support::errno_location() = 0;
                                    n2 = wcsxfrm(buf, s as *const i32, n2.wrapping_add(1));
                                    if *slate_support::errno_location() != 0 {
                                        PyErr_SetFromErrno(PyExc_OSError);
                                        break '__join_0;
                                    }
                                }
                            }
                            n3 = 0;
                            i_1 = 0;
                            while i_1 < n2 {
                                if (*buf.wrapping_offset(i_1 as isize) as u32) > 65536 {
                                    n3 = n3.wrapping_add(1);
                                }
                                i_1 = i_1.wrapping_add(1);
                            }
                            if n3 != 0 {
                                n3 = n3.wrapping_add(n2); // no integer overflow
                                buf2 = PyMem_New__9!(u32, n3);
                                if buf2.is_null() {
                                    PyErr_NoMemory();
                                } else {
                                    j = 0;
                                    i = 0;
                                    while i < n2 {
                                        c = *buf.wrapping_offset(i as isize) as u32;
                                        if c > 65536 {
                                            let __v5: u64 = j;
                                            j = __v5.wrapping_add(1);
                                            *buf2.wrapping_offset(__v5 as isize) =
                                                c >> 16i32 | 65536;
                                            let __v3: u64 = j;
                                            j = __v3.wrapping_add(1);
                                            *buf2.wrapping_offset(__v3 as isize) = c & 65535;
                                        } else {
                                            let __v7: u64 = j;
                                            j = __v7.wrapping_add(1);
                                            *buf2.wrapping_offset(__v7 as isize) = c;
                                        }
                                        i = i.wrapping_add(1);
                                    }
                                    result = PyUnicode_FromKindAndData(
                                        4i32,
                                        buf2 as *const (),
                                        n3 as i64,
                                    );
                                    PyMem_Free(buf2 as *mut ());
                                }
                            } else {
                                result = PyUnicode_FromKindAndData(
                                    core::mem::size_of::<i32>() as i32,
                                    buf as *const (),
                                    n2 as i64,
                                );
                            }
                        }
                    }
                }
            }
        }
        PyMem_Free(buf as *mut ());
        PyMem_Free(s as *mut ());
        return result;
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct langinfo_constant {
    name: *const i8,
    value: i32,
    category: i32,
}

/// These constants should exist on any langinfo implementation
/// The following are not available with glibc 2.0
/// YESSTR and NOSTR are deprecated in glibc, since they are
///    a special case of message translation, which should be rather
///    done using gettext. So we don't expose it to Python in the
///    first place.
/// LANGINFO(YESSTR, LC_MESSAGES),
/// LANGINFO(NOSTR, LC_MESSAGES),
/// The following constants are available only with XPG4, but...
/// OpenBSD doesn't have CODESET but has T_FMT_AMPM, and doesn't have
/// a few of the others.
/// Solution: ifdef-test them all.
/// This is not available in all glibc versions that have CODESET.
static mut langinfo_constants: __SlateAlign16<[langinfo_constant; 57]> = __SlateAlign16([
    langinfo_constant {
        name: (b"DAY_1\0".as_ptr() as *mut i8) as *const i8,
        value: 131079i32,
        category: 2i32,
    },
    langinfo_constant {
        name: (b"DAY_2\0".as_ptr() as *mut i8) as *const i8,
        value: 131080i32,
        category: 2i32,
    },
    langinfo_constant {
        name: (b"DAY_3\0".as_ptr() as *mut i8) as *const i8,
        value: 131081i32,
        category: 2i32,
    },
    langinfo_constant {
        name: (b"DAY_4\0".as_ptr() as *mut i8) as *const i8,
        value: 131082i32,
        category: 2i32,
    },
    langinfo_constant {
        name: (b"DAY_5\0".as_ptr() as *mut i8) as *const i8,
        value: 131083i32,
        category: 2i32,
    },
    langinfo_constant {
        name: (b"DAY_6\0".as_ptr() as *mut i8) as *const i8,
        value: 131084i32,
        category: 2i32,
    },
    langinfo_constant {
        name: (b"DAY_7\0".as_ptr() as *mut i8) as *const i8,
        value: 131085i32,
        category: 2i32,
    },
    langinfo_constant {
        name: (b"ABDAY_1\0".as_ptr() as *mut i8) as *const i8,
        value: 131072i32,
        category: 2i32,
    },
    langinfo_constant {
        name: (b"ABDAY_2\0".as_ptr() as *mut i8) as *const i8,
        value: 131073i32,
        category: 2i32,
    },
    langinfo_constant {
        name: (b"ABDAY_3\0".as_ptr() as *mut i8) as *const i8,
        value: 131074i32,
        category: 2i32,
    },
    langinfo_constant {
        name: (b"ABDAY_4\0".as_ptr() as *mut i8) as *const i8,
        value: 131075i32,
        category: 2i32,
    },
    langinfo_constant {
        name: (b"ABDAY_5\0".as_ptr() as *mut i8) as *const i8,
        value: 131076i32,
        category: 2i32,
    },
    langinfo_constant {
        name: (b"ABDAY_6\0".as_ptr() as *mut i8) as *const i8,
        value: 131077i32,
        category: 2i32,
    },
    langinfo_constant {
        name: (b"ABDAY_7\0".as_ptr() as *mut i8) as *const i8,
        value: 131078i32,
        category: 2i32,
    },
    langinfo_constant {
        name: (b"MON_1\0".as_ptr() as *mut i8) as *const i8,
        value: 131098i32,
        category: 2i32,
    },
    langinfo_constant {
        name: (b"MON_2\0".as_ptr() as *mut i8) as *const i8,
        value: 131099i32,
        category: 2i32,
    },
    langinfo_constant {
        name: (b"MON_3\0".as_ptr() as *mut i8) as *const i8,
        value: 131100i32,
        category: 2i32,
    },
    langinfo_constant {
        name: (b"MON_4\0".as_ptr() as *mut i8) as *const i8,
        value: 131101i32,
        category: 2i32,
    },
    langinfo_constant {
        name: (b"MON_5\0".as_ptr() as *mut i8) as *const i8,
        value: 131102i32,
        category: 2i32,
    },
    langinfo_constant {
        name: (b"MON_6\0".as_ptr() as *mut i8) as *const i8,
        value: 131103i32,
        category: 2i32,
    },
    langinfo_constant {
        name: (b"MON_7\0".as_ptr() as *mut i8) as *const i8,
        value: 131104i32,
        category: 2i32,
    },
    langinfo_constant {
        name: (b"MON_8\0".as_ptr() as *mut i8) as *const i8,
        value: 131105i32,
        category: 2i32,
    },
    langinfo_constant {
        name: (b"MON_9\0".as_ptr() as *mut i8) as *const i8,
        value: 131106i32,
        category: 2i32,
    },
    langinfo_constant {
        name: (b"MON_10\0".as_ptr() as *mut i8) as *const i8,
        value: 131107i32,
        category: 2i32,
    },
    langinfo_constant {
        name: (b"MON_11\0".as_ptr() as *mut i8) as *const i8,
        value: 131108i32,
        category: 2i32,
    },
    langinfo_constant {
        name: (b"MON_12\0".as_ptr() as *mut i8) as *const i8,
        value: 131109i32,
        category: 2i32,
    },
    langinfo_constant {
        name: (b"ABMON_1\0".as_ptr() as *mut i8) as *const i8,
        value: 131086i32,
        category: 2i32,
    },
    langinfo_constant {
        name: (b"ABMON_2\0".as_ptr() as *mut i8) as *const i8,
        value: 131087i32,
        category: 2i32,
    },
    langinfo_constant {
        name: (b"ABMON_3\0".as_ptr() as *mut i8) as *const i8,
        value: 131088i32,
        category: 2i32,
    },
    langinfo_constant {
        name: (b"ABMON_4\0".as_ptr() as *mut i8) as *const i8,
        value: 131089i32,
        category: 2i32,
    },
    langinfo_constant {
        name: (b"ABMON_5\0".as_ptr() as *mut i8) as *const i8,
        value: 131090i32,
        category: 2i32,
    },
    langinfo_constant {
        name: (b"ABMON_6\0".as_ptr() as *mut i8) as *const i8,
        value: 131091i32,
        category: 2i32,
    },
    langinfo_constant {
        name: (b"ABMON_7\0".as_ptr() as *mut i8) as *const i8,
        value: 131092i32,
        category: 2i32,
    },
    langinfo_constant {
        name: (b"ABMON_8\0".as_ptr() as *mut i8) as *const i8,
        value: 131093i32,
        category: 2i32,
    },
    langinfo_constant {
        name: (b"ABMON_9\0".as_ptr() as *mut i8) as *const i8,
        value: 131094i32,
        category: 2i32,
    },
    langinfo_constant {
        name: (b"ABMON_10\0".as_ptr() as *mut i8) as *const i8,
        value: 131095i32,
        category: 2i32,
    },
    langinfo_constant {
        name: (b"ABMON_11\0".as_ptr() as *mut i8) as *const i8,
        value: 131096i32,
        category: 2i32,
    },
    langinfo_constant {
        name: (b"ABMON_12\0".as_ptr() as *mut i8) as *const i8,
        value: 131097i32,
        category: 2i32,
    },
    langinfo_constant {
        name: (b"RADIXCHAR\0".as_ptr() as *mut i8) as *const i8,
        value: 65536i32,
        category: 1i32,
    },
    langinfo_constant {
        name: (b"THOUSEP\0".as_ptr() as *mut i8) as *const i8,
        value: 65537i32,
        category: 1i32,
    },
    langinfo_constant {
        name: (b"CRNCYSTR\0".as_ptr() as *mut i8) as *const i8,
        value: 262159i32,
        category: 4i32,
    },
    langinfo_constant {
        name: (b"D_T_FMT\0".as_ptr() as *mut i8) as *const i8,
        value: 131112i32,
        category: 2i32,
    },
    langinfo_constant {
        name: (b"D_FMT\0".as_ptr() as *mut i8) as *const i8,
        value: 131113i32,
        category: 2i32,
    },
    langinfo_constant {
        name: (b"T_FMT\0".as_ptr() as *mut i8) as *const i8,
        value: 131114i32,
        category: 2i32,
    },
    langinfo_constant {
        name: (b"AM_STR\0".as_ptr() as *mut i8) as *const i8,
        value: 131110i32,
        category: 2i32,
    },
    langinfo_constant {
        name: (b"PM_STR\0".as_ptr() as *mut i8) as *const i8,
        value: 131111i32,
        category: 2i32,
    },
    langinfo_constant {
        name: (b"CODESET\0".as_ptr() as *mut i8) as *const i8,
        value: 14i32,
        category: 0i32,
    },
    langinfo_constant {
        name: (b"T_FMT_AMPM\0".as_ptr() as *mut i8) as *const i8,
        value: 131115i32,
        category: 2i32,
    },
    langinfo_constant {
        name: (b"ERA\0".as_ptr() as *mut i8) as *const i8,
        value: 131116i32,
        category: 2i32,
    },
    langinfo_constant {
        name: (b"ERA_D_FMT\0".as_ptr() as *mut i8) as *const i8,
        value: 131118i32,
        category: 2i32,
    },
    langinfo_constant {
        name: (b"ERA_D_T_FMT\0".as_ptr() as *mut i8) as *const i8,
        value: 131120i32,
        category: 2i32,
    },
    langinfo_constant {
        name: (b"ERA_T_FMT\0".as_ptr() as *mut i8) as *const i8,
        value: 131121i32,
        category: 2i32,
    },
    langinfo_constant {
        name: (b"ALT_DIGITS\0".as_ptr() as *mut i8) as *const i8,
        value: 131119i32,
        category: 2i32,
    },
    langinfo_constant {
        name: (b"YESEXPR\0".as_ptr() as *mut i8) as *const i8,
        value: 327680i32,
        category: 5i32,
    },
    langinfo_constant {
        name: (b"NOEXPR\0".as_ptr() as *mut i8) as *const i8,
        value: 327681i32,
        category: 5i32,
    },
    langinfo_constant {
        name: (b"_DATE_FMT\0".as_ptr() as *mut i8) as *const i8,
        value: 131180i32,
        category: 2i32,
    },
    langinfo_constant {
        name: core::ptr::null::<i8>(),
        value: 0i32,
        category: 0i32,
    },
]);

/// Temporary make the LC_CTYPE locale to be the same as
/// the locale of the specified category.
fn change_locale(category: i32, oldloc: &mut *mut i8) -> i32 {
    // Keep a copy of the LC_CTYPE locale
    unsafe {
        *(oldloc as *mut *mut i8) = setlocale(
            0i32 as i32,
            core::ptr::null::<i8>() as *const core::ffi::c_char,
        ) as *mut i8;
    }
    if unsafe { *(oldloc as *mut *mut i8) }.is_null() {
        unsafe {
            PyErr_SetString(
                PyExc_RuntimeError,
                c"failed to get LC_CTYPE locale".as_ptr(),
            )
        };
        return 1i32.wrapping_neg();
    }
    unsafe {
        *(oldloc as *mut *mut i8) = _PyMem_Strdup(*(oldloc as *mut *mut i8) as *const i8);
    }
    if unsafe { *(oldloc as *mut *mut i8) }.is_null() {
        unsafe { PyErr_NoMemory() };
        return 1i32.wrapping_neg();
    }
    // Set a new locale if it is different.
    let loc: *mut i8 = unsafe {
        setlocale(
            category as i32,
            core::ptr::null::<i8>() as *const core::ffi::c_char,
        ) as *mut i8
    };
    if loc.is_null()
        || (unsafe {
            strcmp(
                (loc as *const i8) as *const core::ffi::c_char,
                (*(oldloc as *mut *mut i8) as *const i8) as *const core::ffi::c_char,
            ) as i32
        }) == 0
    {
        unsafe { PyMem_Free(*(oldloc as *mut *mut i8) as *mut ()) };
        unsafe {
            *(oldloc as *mut *mut i8) = core::ptr::null_mut::<i8>();
        }
        return 0;
    }
    unsafe { setlocale(0i32 as i32, (loc as *const i8) as *const core::ffi::c_char) as *mut i8 };
    1
}

/// Restore the old LC_CTYPE locale.
fn restore_locale(oldloc: *mut i8) {
    if !oldloc.is_null() {
        unsafe {
            setlocale(
                0i32 as i32,
                (oldloc as *const i8) as *const core::ffi::c_char,
            ) as *mut i8
        };
        unsafe { PyMem_Free(oldloc as *mut ()) };
    }
}

fn decode_strings(result: *const i8, max_count: u64) -> *mut _object {
    // Convert a sequence of NUL-separated C strings to a Python string
    // containing semicolon separated items.
    let mut i: u64 = 0;
    let mut count: u64 = 0;
    while count < max_count && (unsafe { *result.wrapping_offset(i as isize) }) != 0 {
        let __v0: u64 = i;
        i = __v0.wrapping_add(
            unsafe {
                strlen(result.wrapping_offset(i as isize) as *const core::ffi::c_char) as u64
            }
            .wrapping_add(1),
        );
        count = count.wrapping_add(1);
    }
    let buf: *mut i8 = (unsafe { PyMem_Malloc(i) }) as *mut i8;
    if buf.is_null() {
        unsafe { PyErr_NoMemory() };
        return core::ptr::null_mut::<_object>();
    }
    unsafe {
        core::ptr::copy_nonoverlapping(
            ((result as *const ()) as *const core::ffi::c_void) as *const u8,
            ((buf as *mut ()) as *mut core::ffi::c_void) as *mut u8,
            (i as usize) as usize,
        )
    };
    // Replace all NULs with semicolons.
    i = 0;
    loop {
        let __v10: u64 = count.wrapping_sub(1);
        count = __v10;
        if __v10 == 0 {
            break;
        }
        let __v5: u64 = i;
        i = __v5.wrapping_add(unsafe {
            strlen((buf.wrapping_offset(i as isize) as *const i8) as *const core::ffi::c_char)
                as u64
        });
        let __v7: u64 = i;
        i = __v7.wrapping_add(1);
        unsafe {
            *buf.wrapping_offset(__v7 as isize) = 59;
        }
    }
    let pyresult: *mut _object =
        unsafe { PyUnicode_DecodeLocale(buf as *const i8, core::ptr::null::<i8>()) };
    unsafe { PyMem_Free(buf as *mut ()) };
    pyresult
}

// [clinic input]
// _locale.nl_langinfo
//
//     key as item: int
//     /
//
// Return the value for the locale information associated with key.
// [clinic start generated code]
/// [clinic end generated code: output=6aea457b47e077a3 input=00798143eecfeddc]
fn _locale_nl_langinfo_impl(module: *mut _object, item: i32) -> *mut _object {
    // Check whether this is a supported constant. GNU libc sometimes
    // returns numeric values in the char* return value, which would
    // crash PyUnicode_FromString.
    let mut i: i32 = 0;
    while !unsafe { langinfo_constants.0[i as usize].name }.is_null() {
        if (unsafe { langinfo_constants.0[i as usize].value }) == item {
            // Check NULL as a workaround for GNU libc's returning NULL
            // instead of an empty string for nl_langinfo(ERA).
            let mut result: *const i8 =
                (unsafe { nl_langinfo(item as i32) as *mut i8 }) as *const i8;
            result = if !result.is_null() {
                result
            } else {
                c"".as_ptr()
            };
            let mut oldloc: *mut i8 = core::ptr::null_mut::<i8>();
            let __v0: bool = (unsafe { langinfo_constants.0[i as usize].category }) != 0
                && (unsafe { *result }) != 0
                && (item == (slate_support::ALT_DIGITS as i32)
                    || item == (slate_support::ERA as i32)
                    || is_all_ascii(result) == 0);
            let __v2: bool = __v0
                && change_locale(
                    unsafe { langinfo_constants.0[i as usize].category },
                    unsafe { &mut (*core::ptr::addr_of_mut!(oldloc)) },
                ) < 0;
            if __v2 {
                return core::ptr::null_mut::<_object>();
            }
            // gh-133740: Always change the locale for ALT_DIGITS and ERA
            let mut pyresult: *mut _object = unsafe { core::mem::zeroed() };
            // According to the POSIX specification the result must be
            // a sequence of semicolon-separated strings.
            // But in Glibc they are NUL-separated.
            if item == (slate_support::ALT_DIGITS as i32) && (unsafe { *result }) != 0 {
                pyresult = decode_strings(result, 100u64);
            } else {
                if item == (slate_support::ERA as i32) && (unsafe { *result }) != 0 {
                    pyresult = decode_strings(result, 18446744073709551615u64);
                } else {
                    pyresult = unsafe { PyUnicode_DecodeLocale(result, core::ptr::null::<i8>()) };
                }
            }
            restore_locale(oldloc);
            return pyresult;
        }
        i = i.wrapping_add(1);
    }
    unsafe { PyErr_SetString(PyExc_ValueError, c"unsupported langinfo constant".as_ptr()) };
    core::ptr::null_mut::<_object>()
}

// [clinic input]
// _locale.gettext
//
//     msg as in: str
//     /
//
// gettext(msg) -> string
//
// Return translation of msg.
// [clinic start generated code]
/// [clinic end generated code: output=493bb4b38a4704fe input=949fc8efc2bb3bc3]
fn _locale_gettext_impl(module: *mut _object, r#in: *const i8) -> *mut _object {
    unsafe {
        PyUnicode_DecodeLocale(
            dcgettext(core::ptr::null::<i8>(), r#in, 5i32) as *const i8,
            core::ptr::null::<i8>(),
        )
    }
}

// [clinic input]
// _locale.dgettext
//
//     domain: str(accept={str, NoneType})
//     msg as in: str
//     /
//
// dgettext(domain, msg) -> string
//
// Return translation of msg in domain.
// [clinic start generated code]
/// [clinic end generated code: output=3c0cd5287b972c8f input=a277388a635109d8]
fn _locale_dgettext_impl(module: *mut _object, domain: *const i8, r#in: *const i8) -> *mut _object {
    unsafe {
        PyUnicode_DecodeLocale(
            dcgettext(domain, r#in, 5i32) as *const i8,
            core::ptr::null::<i8>(),
        )
    }
}

// [clinic input]
// _locale.dcgettext
//
//     domain: str(accept={str, NoneType})
//     msg as msgid: str
//     category: int
//     /
//
// Return translation of msg in domain and category.
// [clinic start generated code]
/// [clinic end generated code: output=0f4cc4fce0aa283f input=ec5f8fed4336de67]
fn _locale_dcgettext_impl(
    module: *mut _object,
    domain: *const i8,
    msgid: *const i8,
    category: i32,
) -> *mut _object {
    unsafe {
        PyUnicode_DecodeLocale(
            dcgettext(domain, msgid, category) as *const i8,
            core::ptr::null::<i8>(),
        )
    }
}

// [clinic input]
// _locale.textdomain
//
//     domain: str(accept={str, NoneType})
//     /
//
// Set the C library's textdmain to domain, returning the new domain.
// [clinic start generated code]
/// [clinic end generated code: output=7992df06aadec313 input=66359716f5eb1d38]
fn _locale_textdomain_impl(module: *mut _object, mut domain: *const i8) -> *mut _object {
    domain = (unsafe { textdomain(domain) }) as *const i8;
    if domain.is_null() {
        unsafe { PyErr_SetFromErrno(PyExc_OSError) };
        return core::ptr::null_mut::<_object>();
    }
    unsafe { PyUnicode_DecodeLocale(domain, core::ptr::null::<i8>()) }
}

// [clinic input]
// _locale.bindtextdomain
//
//     domain: str
//     dir as dirname_obj: object
//     /
//
// Bind the C library's domain to dir.
// [clinic start generated code]
/// [clinic end generated code: output=6d6f3c7b345d785c input=c0dff085acfe272b]
fn _locale_bindtextdomain_impl(
    module: *mut _object,
    domain: *const i8,
    dirname_obj: *mut _object,
) -> *mut _object {
    let mut dirname: *const i8 = unsafe { core::mem::zeroed() };
    let mut dirname_bytes: *mut _object = core::ptr::null_mut::<_object>();
    if (unsafe { strlen(domain as *const core::ffi::c_char) as u64 }) == 0 {
        unsafe {
            PyErr_SetString(
                (*get_locale_state(module)).Error,
                c"domain must be a non-empty string".as_ptr(),
            )
        };
        return core::ptr::null_mut::<_object>();
    }
    if dirname_obj != unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } {
        if (unsafe {
            PyUnicode_FSConverter(
                dirname_obj,
                core::ptr::addr_of_mut!(dirname_bytes) as *mut (),
            )
        }) == 0
        {
            return core::ptr::null_mut::<_object>();
        }
        dirname = (unsafe { PyBytes_AsString(dirname_bytes) }) as *const i8;
    } else {
        dirname_bytes = core::ptr::null_mut::<_object>();
        dirname = core::ptr::null::<i8>();
    }
    let current_dirname: *const i8 = (unsafe { bindtextdomain(domain, dirname) }) as *const i8;
    if current_dirname.is_null() {
        unsafe { PyErr_SetFromErrno(PyExc_OSError) };
        Py_XDECREF!(dirname_bytes);
        return core::ptr::null_mut::<_object>();
    }
    let result: *mut _object =
        unsafe { PyUnicode_DecodeLocale(current_dirname, core::ptr::null::<i8>()) };
    Py_XDECREF!(dirname_bytes);
    result
}

// [clinic input]
// _locale.bind_textdomain_codeset
//
//     domain: str
//     codeset: str(accept={str, NoneType})
//     /
//
// Bind the C library's domain to codeset.
// [clinic start generated code]
/// [clinic end generated code: output=fa452f9c8b1b9e89 input=23fbe3540400f259]
fn _locale_bind_textdomain_codeset_impl(
    module: *mut _object,
    domain: *const i8,
    mut codeset: *const i8,
) -> *mut _object {
    codeset = (unsafe { bind_textdomain_codeset(domain, codeset) }) as *const i8;
    if !codeset.is_null() {
        return unsafe { PyUnicode_DecodeLocale(codeset, core::ptr::null::<i8>()) };
    }
    unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
}

// [clinic input]
// _locale.getencoding
//
// Get the current locale encoding.
// [clinic start generated code]
/// [clinic end generated code: output=86b326b971872e46 input=6503d11e5958b360]
fn _locale_getencoding_impl(module: *mut _object) -> *mut _object {
    unsafe { _Py_GetLocaleEncodingObject() }
}

static mut PyLocale_Methods: __SlateAlign16<[PyMethodDef; 13]> = __SlateAlign16([
    {
        let mut __t0: PyMethodDef = unsafe { core::mem::zeroed() };
        __t0.ml_name = (b"setlocale\0".as_ptr() as *mut i8) as *const i8;
        __t0.ml_meth = unsafe {
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
                >(Some(_locale_setlocale))
            })
        };
        __t0.ml_flags = 128i32;
        __t0.ml_doc = unsafe { core::ptr::addr_of!(_locale_setlocale__doc__.0) as *const i8 };
        __t0
    },
    {
        let mut __t1: PyMethodDef = unsafe { core::mem::zeroed() };
        __t1.ml_name = (b"localeconv\0".as_ptr() as *mut i8) as *const i8;
        __t1.ml_meth = Some(_locale_localeconv);
        __t1.ml_flags = 4i32;
        __t1.ml_doc = unsafe { core::ptr::addr_of!(_locale_localeconv__doc__.0) as *const i8 };
        __t1
    },
    {
        let mut __t2: PyMethodDef = unsafe { core::mem::zeroed() };
        __t2.ml_name = (b"strcoll\0".as_ptr() as *mut i8) as *const i8;
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
                        ) -> *mut _object,
                    >,
                    Option<unsafe extern "C-unwind" fn()>,
                >(Some(_locale_strcoll))
            })
        };
        __t2.ml_flags = 128i32;
        __t2.ml_doc = unsafe { core::ptr::addr_of!(_locale_strcoll__doc__.0) as *const i8 };
        __t2
    },
    {
        let mut __t3: PyMethodDef = unsafe { core::mem::zeroed() };
        __t3.ml_name = (b"strxfrm\0".as_ptr() as *mut i8) as *const i8;
        __t3.ml_meth = Some(_locale_strxfrm);
        __t3.ml_flags = 8i32;
        __t3.ml_doc = unsafe { core::ptr::addr_of!(_locale_strxfrm__doc__.0) as *const i8 };
        __t3
    },
    {
        let mut __t4: PyMethodDef = unsafe { core::mem::zeroed() };
        __t4.ml_name = (b"nl_langinfo\0".as_ptr() as *mut i8) as *const i8;
        __t4.ml_meth = Some(_locale_nl_langinfo);
        __t4.ml_flags = 8i32;
        __t4.ml_doc = unsafe { core::ptr::addr_of!(_locale_nl_langinfo__doc__.0) as *const i8 };
        __t4
    },
    {
        let mut __t5: PyMethodDef = unsafe { core::mem::zeroed() };
        __t5.ml_name = (b"gettext\0".as_ptr() as *mut i8) as *const i8;
        __t5.ml_meth = Some(_locale_gettext);
        __t5.ml_flags = 8i32;
        __t5.ml_doc = unsafe { core::ptr::addr_of!(_locale_gettext__doc__.0) as *const i8 };
        __t5
    },
    {
        let mut __t6: PyMethodDef = unsafe { core::mem::zeroed() };
        __t6.ml_name = (b"dgettext\0".as_ptr() as *mut i8) as *const i8;
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
                        ) -> *mut _object,
                    >,
                    Option<unsafe extern "C-unwind" fn()>,
                >(Some(_locale_dgettext))
            })
        };
        __t6.ml_flags = 128i32;
        __t6.ml_doc = unsafe { core::ptr::addr_of!(_locale_dgettext__doc__.0) as *const i8 };
        __t6
    },
    {
        let mut __t7: PyMethodDef = unsafe { core::mem::zeroed() };
        __t7.ml_name = (b"dcgettext\0".as_ptr() as *mut i8) as *const i8;
        __t7.ml_meth = unsafe {
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
                >(Some(_locale_dcgettext))
            })
        };
        __t7.ml_flags = 128i32;
        __t7.ml_doc = unsafe { core::ptr::addr_of!(_locale_dcgettext__doc__.0) as *const i8 };
        __t7
    },
    {
        let mut __t8: PyMethodDef = unsafe { core::mem::zeroed() };
        __t8.ml_name = (b"textdomain\0".as_ptr() as *mut i8) as *const i8;
        __t8.ml_meth = Some(_locale_textdomain);
        __t8.ml_flags = 8i32;
        __t8.ml_doc = unsafe { core::ptr::addr_of!(_locale_textdomain__doc__.0) as *const i8 };
        __t8
    },
    {
        let mut __t9: PyMethodDef = unsafe { core::mem::zeroed() };
        __t9.ml_name = (b"bindtextdomain\0".as_ptr() as *mut i8) as *const i8;
        __t9.ml_meth = unsafe {
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
                >(Some(_locale_bindtextdomain))
            })
        };
        __t9.ml_flags = 128i32;
        __t9.ml_doc = unsafe { core::ptr::addr_of!(_locale_bindtextdomain__doc__.0) as *const i8 };
        __t9
    },
    {
        let mut __t10: PyMethodDef = unsafe { core::mem::zeroed() };
        __t10.ml_name = (b"bind_textdomain_codeset\0".as_ptr() as *mut i8) as *const i8;
        __t10.ml_meth = unsafe {
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
                >(Some(_locale_bind_textdomain_codeset))
            })
        };
        __t10.ml_flags = 128i32;
        __t10.ml_doc =
            unsafe { core::ptr::addr_of!(_locale_bind_textdomain_codeset__doc__.0) as *const i8 };
        __t10
    },
    {
        let mut __t11: PyMethodDef = unsafe { core::mem::zeroed() };
        __t11.ml_name = (b"getencoding\0".as_ptr() as *mut i8) as *const i8;
        __t11.ml_meth = Some(_locale_getencoding);
        __t11.ml_flags = 4i32;
        __t11.ml_doc = unsafe { core::ptr::addr_of!(_locale_getencoding__doc__.0) as *const i8 };
        __t11
    },
    {
        let mut __t12: PyMethodDef = unsafe { core::mem::zeroed() };
        __t12.ml_name = core::ptr::null::<i8>();
        __t12.ml_meth = None;
        __t12
    },
]);

#[unsafe(link_section = ".text.slate_distinct._localemodule._locale_exec")]
extern "C-unwind" fn _locale_exec(module: *mut _object) -> i32 {
    let mut i: i32 = 0;
    loop {
        if (unsafe { PyModule_AddIntConstant(module, c"LC_CTYPE".as_ptr(), 0i64) }) < 0 {
            return 1i32.wrapping_neg();
        }
        if true {
            break;
        }
    }
    loop {
        if (unsafe { PyModule_AddIntConstant(module, c"LC_TIME".as_ptr(), 2i64) }) < 0 {
            return 1i32.wrapping_neg();
        }
        if true {
            break;
        }
    }
    loop {
        if (unsafe { PyModule_AddIntConstant(module, c"LC_COLLATE".as_ptr(), 3i64) }) < 0 {
            return 1i32.wrapping_neg();
        }
        if true {
            break;
        }
    }
    loop {
        if (unsafe { PyModule_AddIntConstant(module, c"LC_MONETARY".as_ptr(), 4i64) }) < 0 {
            return 1i32.wrapping_neg();
        }
        if true {
            break;
        }
    }
    loop {
        if (unsafe { PyModule_AddIntConstant(module, c"LC_MESSAGES".as_ptr(), 5i64) }) < 0 {
            return 1i32.wrapping_neg();
        }
        if true {
            break;
        }
    }
    loop {
        if (unsafe { PyModule_AddIntConstant(module, c"LC_NUMERIC".as_ptr(), 1i64) }) < 0 {
            return 1i32.wrapping_neg();
        }
        if true {
            break;
        }
    }
    loop {
        if (unsafe { PyModule_AddIntConstant(module, c"LC_ALL".as_ptr(), 6i64) }) < 0 {
            return 1i32.wrapping_neg();
        }
        if true {
            break;
        }
    }
    loop {
        if (unsafe { PyModule_AddIntConstant(module, c"CHAR_MAX".as_ptr(), 127i64) }) < 0 {
            return 1i32.wrapping_neg();
        }
        if true {
            break;
        }
    }
    let state: *mut _locale_state = get_locale_state(module);
    unsafe {
        (*state).Error = PyErr_NewException(
            c"locale.Error".as_ptr(),
            core::ptr::null_mut::<_object>(),
            core::ptr::null_mut::<_object>(),
        );
    }
    if (unsafe { PyModule_AddObjectRef(module, c"Error".as_ptr(), (*state).Error) }) < 0 {
        return 1i32.wrapping_neg();
    }
    while !unsafe { langinfo_constants.0[i as usize].name }.is_null() {
        if (unsafe {
            PyModule_AddIntConstant(
                module,
                langinfo_constants.0[i as usize].name,
                langinfo_constants.0[i as usize].value as i64,
            )
        }) < 0
        {
            return 1i32.wrapping_neg();
        }
        i = i.wrapping_add(1);
    }
    if !unsafe { PyErr_Occurred() }.is_null() {
        return 1i32.wrapping_neg();
    }
    0
}

static mut _locale_slots: __SlateAlign16<[PyModuleDef_Slot; 4]> = __SlateAlign16([
    {
        let mut __t0: PyModuleDef_Slot = unsafe { core::mem::zeroed() };
        __t0.slot = 2i32;
        __t0.value = unsafe {
            core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut _object) -> i32>, *mut ()>(
                Some(_locale_exec),
            )
        };
        __t0
    },
    {
        let mut __t1: PyModuleDef_Slot = unsafe { core::mem::zeroed() };
        __t1.slot = 3i32;
        __t1.value = 2i32 as *mut ();
        __t1
    },
    {
        let mut __t2: PyModuleDef_Slot = unsafe { core::mem::zeroed() };
        __t2.slot = 4i32;
        __t2.value = 1i32 as *mut ();
        __t2
    },
    {
        let mut __t3: PyModuleDef_Slot = unsafe { core::mem::zeroed() };
        __t3.slot = 0i32;
        __t3.value = core::ptr::null_mut::<()>();
        __t3
    },
]);

#[unsafe(link_section = ".text.slate_distinct._localemodule.locale_traverse")]
extern "C-unwind" fn locale_traverse(
    module: *mut _object,
    mut visit: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
    arg: *mut (),
) -> i32 {
    let state: *mut _locale_state = get_locale_state(module);
    Py_VISIT!(unsafe { (*state).Error }, visit, arg);
    0
}

#[unsafe(link_section = ".text.slate_distinct._localemodule.locale_clear")]
extern "C-unwind" fn locale_clear(module: *mut _object) -> i32 {
    let state: *mut _locale_state = get_locale_state(module);
    Py_CLEAR!((*state).Error);
    0
}

#[unsafe(link_section = ".text.slate_distinct._localemodule.locale_free")]
extern "C-unwind" fn locale_free(module: *mut ()) {
    locale_clear(module as *mut _object);
}

static mut _localemodule: PyModuleDef = PyModuleDef {
    m_base: PyModuleDef_Base {
        ob_base: _object {
            __slate_anon_0: __SlateRecord_object_h_122 {
                ob_refcnt_full: 1407378104778752i64,
            },
            ob_type: core::ptr::null_mut::<_typeobject>(),
        },
        m_init: None,
        m_index: 0i64,
        m_copy: core::ptr::null_mut::<_object>(),
    },
    m_name: (b"_locale\0".as_ptr() as *mut i8) as *const i8,
    m_doc: unsafe { core::ptr::addr_of!(locale__doc__.0) as *const i8 },
    m_size: (core::mem::size_of::<_locale_state>() as u64) as i64,
    m_methods: unsafe { core::ptr::addr_of_mut!(PyLocale_Methods.0) as *mut PyMethodDef },
    m_slots: unsafe { core::ptr::addr_of_mut!(_locale_slots.0) as *mut PyModuleDef_Slot },
    m_traverse: Some(locale_traverse),
    m_clear: Some(locale_clear),
    m_free: Some(locale_free),
};

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct._localemodule.PyInit__locale")]
extern "C-unwind" fn PyInit__locale() -> *mut _object {
    unsafe { PyModuleDef_Init(core::ptr::addr_of_mut!(_localemodule)) }
}

// Local variables:
// c-basic-offset: 4
// indent-tabs-mode: nil
// End:
