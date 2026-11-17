//! The size of 'void *', as computed by sizeof.
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
//! No error
//! Max indentation level
// Signed variants of the above
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
// End Of File
macro_rules! ADVANCE_LINENO {
    ($tok:expr) => {
        let __v: *mut tok_state = $tok;
        let __v2: i32 = unsafe { (*__v).lineno };
        unsafe {
            (*__v).lineno = __v2.wrapping_add(1);
        }
        unsafe {
            (*$tok).col_offset = 0;
        }
    };
}

macro_rules! INSIDE_FSTRING {
    ($tok:expr) => {
        (unsafe { (*$tok).tok_mode_stack_index }) > 0
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

macro_rules! Py_CLEAR__2 {
    ($op:expr) => {
        loop {
            let _tmp_op_ptr: *mut *mut _object = core::ptr::addr_of_mut!($op);
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

macro_rules! Py_INCREF {
    ($op:expr) => {
        Py_INCREF(_PyObject_CAST!($op));
    };
}

macro_rules! Py_SIZE {
    ($ob:expr) => {
        Py_SIZE(_PyObject_CAST__2!($ob))
    };
}

macro_rules! Py_TYPE {
    ($ob:expr) => {
        _Py_TYPE(_PyObject_CAST!($ob))
    };
}

macro_rules! Py_XDECREF {
    ($op:expr) => {
        Py_XDECREF(_PyObject_CAST!($op));
    };
}

/// Macros and static inline functions, trading safety for speed
macro_rules! _PyBytes_CAST {
    ($op:expr) => {
        _Py_CAST!(*mut __SlateRecord_bytesobject_h_15, $op)
    };
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

/// Cast argument to PyVarObject* type.
macro_rules! _PyVarObject_CAST {
    ($op:expr) => {
        _Py_CAST!(*mut __SlateRecord_object_h_172, $op)
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
    static mut PyExc_StopIteration: *mut _object;
    static mut PyExc_TypeError: *mut _object;
    fn PyMem_Malloc(arg0: u64) -> *mut ();
    fn _Py_Dealloc(arg0: *mut _object);
    fn PyUnicode_Decode(
        arg0: *const i8,
        arg1: i64,
        arg2: *const i8,
        arg3: *const i8,
    ) -> *mut _object;
    fn PyUnicode_AsUTF8AndSize(arg0: *mut _object, arg1: *mut i64) -> *const i8;
    fn PyErr_Clear();
    fn PyErr_ExceptionMatches(arg0: *mut _object) -> i32;
    fn PyErr_NoMemory() -> *mut _object;
    fn PyErr_Format(arg0: *mut _object, arg1: *const i8, ...) -> *mut _object;
    fn PyObject_CallNoArgs(arg0: *mut _object) -> *mut _object;
    fn _PyTokenizer_tok_new() -> *mut tok_state;
    fn _PyTokenizer_Free(arg0: *mut tok_state);
    fn _PyTokenizer_error_ret(arg0: *mut tok_state) -> *mut i8;
    fn _PyTokenizer_new_string(arg0: *const i8, arg1: i64, arg2: *mut tok_state) -> *mut i8;
    fn _PyTokenizer_ensure_utf8(arg0: *const i8, arg1: *mut tok_state, arg2: i32) -> i32;
    fn _PyLexer_tok_reserve_buf(arg0: *mut tok_state, arg1: i64) -> i32;
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

fn _Py_TYPE(ob: *mut _object) -> *mut _typeobject {
    unsafe { (*ob).ob_type }
}

fn Py_SIZE(ob: *mut _object) -> i64 {
    unsafe { (*_PyVarObject_CAST!(ob)).ob_size }
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

use slate_support::memcpy;
use slate_support::strlen;

fn Py_XDECREF(op: *mut _object) {
    if !op.is_null() {
        Py_DECREF!(op);
    }
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

// Max parentheses level
// Max f-string nesting level
#[repr(C)]
#[derive(Clone, Copy)]
struct _tokenizer_mode {
    kind: u32,
    curly_bracket_depth: i32,
    curly_bracket_expr_start_depth: i32,
    quote: i8,
    quote_size: i32,
    raw: i32,
    start: *const i8,
    multi_line_start: *const i8,
    first_line: i32,
    start_offset: i64,
    multi_line_start_offset: i64,
    last_expr_start: *const i8,
    last_expr_start_offset: i64,
    in_debug: i32,
    in_format_spec: i32,
    string_kind: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct tok_state {
    buf: *mut i8,
    cur: *mut i8,
    inp: *mut i8,
    fp_interactive: i32,
    interactive_src_start: *mut i8,
    interactive_src_end: *mut i8,
    end: *const i8,
    start: *const i8,
    done: i32,
    fp: *mut slate_support::FILE,
    tabsize: i32,
    indent: i32,
    indstack: [i32; 100],
    atbol: i32,
    pendin: i32,
    prompt: *const i8,
    nextprompt: *const i8,
    lineno: i32,
    first_lineno: i32,
    starting_col_offset: i32,
    col_offset: i32,
    level: i32,
    parenstack: [i8; 200],
    parenlinenostack: [i32; 200],
    parencolstack: [i32; 200],
    filename: *mut _object,
    altindstack: [i32; 100],
    decoding_state: u32,
    decoding_erred: i32,
    encoding: *mut i8,
    cont_line: i32,
    line_start: *const i8,
    multi_line_start: *const i8,
    decoding_readline: *mut _object,
    decoding_buffer: *mut _object,
    readline: *mut _object,
    enc: *const i8,
    str: *mut i8,
    input: *mut i8,
    type_comments: i32,
    interactive_underflow: u32,
    underflow: Option<unsafe extern "C-unwind" fn(*mut tok_state) -> i32>,
    report_warnings: i32,
    tok_mode_stack: [_tokenizer_mode; 150],
    tok_mode_stack_index: i32,
    tok_extra_tokens: i32,
    comment_newline: i32,
    implicit_newline: i32,
    barry_as_bdfl: i32,
}

fn tok_readline_string(tok: *mut tok_state) -> i32 {
    let mut buflen: i64 = unsafe { core::mem::zeroed() };
    let mut raw_line: *mut _object = unsafe { core::mem::zeroed() };
    let mut line: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_0: {
            line = core::ptr::null_mut::<_object>();
            raw_line = PyObject_CallNoArgs((*tok).readline);
            if raw_line.is_null() {
                if PyErr_ExceptionMatches(PyExc_StopIteration) != 0 {
                    PyErr_Clear();
                    return 1;
                } else {
                    _PyTokenizer_error_ret(tok);
                }
            } else {
                if !unsafe { (*tok).encoding }.is_null() {
                    if PyBytes_Check!(raw_line) == 0 {
                        PyErr_Format(
                            PyExc_TypeError,
                            c"readline() returned a non-bytes object".as_ptr(),
                        );
                        _PyTokenizer_error_ret(tok);
                        break '__join_0;
                    } else {
                        line = PyUnicode_Decode(
                            PyBytes_AS_STRING!(raw_line) as *const i8,
                            PyBytes_GET_SIZE!(raw_line),
                            (*tok).encoding as *const i8,
                            c"replace".as_ptr(),
                        );
                        Py_CLEAR__2!(raw_line);
                        if line.is_null() {
                            _PyTokenizer_error_ret(tok);
                            break '__join_0;
                        }
                    }
                } else {
                    if PyUnicode_Check!(raw_line) == 0 {
                        PyErr_Format(
                            PyExc_TypeError,
                            c"readline() returned a non-string object".as_ptr(),
                        );
                        _PyTokenizer_error_ret(tok);
                        break '__join_0;
                    } else {
                        line = raw_line;
                        raw_line = core::ptr::null_mut::<_object>();
                    }
                }
                let buf: *const i8 = PyUnicode_AsUTF8AndSize(line, core::ptr::addr_of_mut!(buflen));
                if buf.is_null() {
                    _PyTokenizer_error_ret(tok);
                } else {
                    // Make room for the null terminator *and* potentially
                    //
                    // an extra newline character that we may need to artificially
                    //
                    // add.
                    let buffer_size: u64 = buflen.wrapping_add(2) as u64;
                    if _PyLexer_tok_reserve_buf(tok, buffer_size as i64) != 0 {
                        core::ptr::copy_nonoverlapping(
                            ((buf as *const ()) as *const core::ffi::c_void) as *const u8,
                            (((*tok).inp as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                            ((buflen as u64) as usize) as usize,
                        );
                        let __v1: *mut i8 = (*tok).inp;
                        (*tok).inp = __v1.wrapping_offset(buflen as isize);
                        *(*tok).inp = 0;
                        (*tok).line_start = (*tok).cur as *const i8;
                        Py_DECREF!(line);
                        return 1;
                    }
                }
            }
        }
        Py_XDECREF!(raw_line);
        Py_XDECREF!(line);
        return 0;
    }
}

#[unsafe(link_section = ".text.slate_distinct.readline_tokenizer.tok_underflow_readline")]
extern "C-unwind" fn tok_underflow_readline(tok: *mut tok_state) -> i32 {
    if unsafe { (*tok).start }.is_null() && !INSIDE_FSTRING!(tok) {
        let __v0: *mut i8 = unsafe { (*tok).buf };
        unsafe {
            (*tok).inp = __v0;
        }
        unsafe {
            (*tok).cur = __v0;
        }
    }
    if tok_readline_string(tok) == 0 {
        return 0;
    }
    if (unsafe { (*tok).inp }) == unsafe { (*tok).cur } {
        unsafe {
            (*tok).done = 11;
        }
        return 0;
    }
    unsafe {
        (*tok).implicit_newline = 0;
    }
    if ((unsafe { *unsafe { (*tok).inp }.wrapping_offset(1i32.wrapping_neg() as isize) }) as i32)
        != 10
    {
        // Last line does not end in \n, fake one
        let __v2: *mut i8 = unsafe { (*tok).inp };
        unsafe {
            (*tok).inp = __v2.wrapping_offset(1 as isize);
        }
        unsafe {
            *__v2 = 10;
        }
        unsafe {
            *(*tok).inp = 0;
        }
        unsafe {
            (*tok).implicit_newline = 1;
        }
    }
    ADVANCE_LINENO!(tok);
    // The default encoding is UTF-8, so make sure we don't have any
    // non-UTF-8 sequences in it.
    let __v7: bool = unsafe { (*tok).encoding }.is_null()
        && (unsafe { _PyTokenizer_ensure_utf8((*tok).cur as *const i8, tok, (*tok).lineno) }) == 0;
    if __v7 {
        unsafe { _PyTokenizer_error_ret(tok) };
        return 0;
    }
    ((unsafe { (*tok).done }) == 10) as i32
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyTokenizer_FromReadline(
    readline: *mut _object,
    enc: *const i8,
    exec_input: i32,
    preserve_crlf: i32,
) -> *mut tok_state {
    let tok: *mut tok_state = unsafe { _PyTokenizer_tok_new() };
    if tok.is_null() {
        return core::ptr::null_mut::<tok_state>();
    }
    let __v0: *mut i8 = (unsafe { PyMem_Malloc((slate_support::BUFSIZ as i32) as u64) }) as *mut i8;
    unsafe {
        (*tok).buf = __v0;
    }
    if __v0.is_null() {
        unsafe { _PyTokenizer_Free(tok) };
        unsafe { PyErr_NoMemory() };
        return core::ptr::null_mut::<tok_state>();
    }
    let __v1: *mut i8 = unsafe { (*tok).buf };
    unsafe {
        (*tok).inp = __v1;
    }
    unsafe {
        (*tok).cur = __v1;
    }
    unsafe {
        (*tok).end = unsafe { (*tok).buf }.wrapping_offset((slate_support::BUFSIZ as i32) as isize)
            as *const i8;
    }
    unsafe {
        (*tok).fp = core::ptr::null_mut::<slate_support::FILE>();
    }
    if !enc.is_null() {
        unsafe {
            (*tok).encoding = _PyTokenizer_new_string(
                enc,
                (strlen(enc as *const core::ffi::c_char) as u64) as i64,
                tok,
            );
        }
        if unsafe { (*tok).encoding }.is_null() {
            unsafe { _PyTokenizer_Free(tok) };
            return core::ptr::null_mut::<tok_state>();
        }
    }
    unsafe {
        (*tok).decoding_state = 2;
    }
    unsafe {
        (*tok).underflow = Some(tok_underflow_readline);
    }
    Py_INCREF!(readline);
    unsafe {
        (*tok).readline = readline;
    }
    tok
}
