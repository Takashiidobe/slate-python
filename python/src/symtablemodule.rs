//! The size of 'void *', as computed by sizeof.
//! Define macros for inline documentation.
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
//! Remote debugger support
//! PY_TIMEOUT_T is the integral type used to specify timeouts when waiting
//!
//! on a lock (see PyThread_acquire_lock_timed() below).
//! These definitions must match corresponding definitions in graminit.h.
//! bpo-39562: CO_FUTURE_ and PyCF_ constants must be kept unique.
//! PyCF_ constants can use bits from 0x0100 to 0x10000.
//! CO_FUTURE_ constants use bits starting at 0x20000.
//! _Py_SourceAsString()
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

macro_rules! Py_NewRef__2 {
    ($obj:expr) => {
        _Py_NewRef(_PyObject_CAST__2!($obj))
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
    static mut PyExc_ValueError: *mut _object;
    fn _Py_Dealloc(arg0: *mut _object);
    fn PyUnicode_AsUTF8AndSize(arg0: *mut _object, arg1: *mut i64) -> *const i8;
    fn PyUnicode_FSDecoder(arg0: *mut _object, arg1: *mut ()) -> i32;
    fn PyErr_SetString(arg0: *mut _object, arg1: *const i8);
    fn PyModuleDef_Init(arg0: *mut PyModuleDef) -> *mut _object;
    fn PyModule_AddIntConstant(arg0: *mut _object, arg1: *const i8, arg2: i64) -> i32;
    fn _Py_SourceAsString(
        arg0: *mut _object,
        arg1: *const i8,
        arg2: *const i8,
        arg3: *mut __SlateRecord_compile_h_30,
        arg4: *mut *mut _object,
    ) -> *const i8;
    fn _PySymtable_Free(arg0: *mut symtable);
    fn _Py_SymtableStringObjectFlags(
        arg0: *const i8,
        arg1: *mut _object,
        arg2: i32,
        arg3: *mut __SlateRecord_compile_h_30,
    ) -> *mut symtable;
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

#[repr(C, align(16))]
struct __SlateAlign16<T>(T);

unsafe impl<T> Sync for __SlateAlign16<T> {}

mod __slate_bits {
    #[bitfields::bitfield(
        u8,
        c_names = true,
        new = false,
        from_into_bits = false,
        from_traits = false,
        default = false,
        debug = false,
        builder = false,
        bit_ops = false
    )]
    pub struct __SlateBits91U0 {
        #[bits(1)]
        pub ste_generator: u32,
        #[bits(1)]
        pub ste_coroutine: u32,
        #[bits(1)]
        pub ste_annotations_used: u32,
        #[bits(5, access = na)]
        pub __slate_pad_3: u8,
    }
    #[bitfields::bitfield(
        u16,
        c_names = true,
        new = false,
        from_into_bits = false,
        from_traits = false,
        default = false,
        debug = false,
        builder = false,
        bit_ops = false
    )]
    pub struct __SlateBits91U1 {
        #[bits(1)]
        pub ste_varargs: u32,
        #[bits(1)]
        pub ste_varkeywords: u32,
        #[bits(1)]
        pub ste_returns_value: u32,
        #[bits(1)]
        pub ste_needs_class_closure: u32,
        #[bits(1)]
        pub ste_needs_classdict: u32,
        #[bits(1)]
        pub ste_comp_inlined: u32,
        #[bits(1)]
        pub ste_comp_iter_target: u32,
        #[bits(1)]
        pub ste_can_see_class_scope: u32,
        #[bits(1)]
        pub ste_has_docstring: u32,
        #[bits(1)]
        pub ste_method: u32,
        #[bits(1)]
        pub ste_has_conditional_annotations: u32,
        #[bits(1)]
        pub ste_in_conditional_block: u32,
        #[bits(1)]
        pub ste_in_unevaluated_annotation: u32,
        #[bits(3, access = na)]
        pub __slate_pad_13: u8,
    }
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
struct __SlateRecord_compile_h_30 {
    cf_flags: i32,
    cf_feature_version: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_symtable_h_51 {
    lineno: i32,
    end_lineno: i32,
    col_offset: i32,
    end_col_offset: i32,
}

use slate_support::strcmp;
use slate_support::strlen;

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_symtable_h_67 {
    ff_features: i32,
    ff_location: __SlateRecord_pycore_symtable_h_51,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct symtable {
    st_filename: *mut _object,
    st_cur: *mut _symtable_entry,
    st_top: *mut _symtable_entry,
    st_blocks: *mut _object,
    st_stack: *mut _object,
    st_global: *mut _object,
    st_nblocks: i32,
    st_private: *mut _object,
    st_future: *mut __SlateRecord_pycore_symtable_h_67,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _symtable_entry {
    ob_base: _object,
    ste_id: *mut _object,
    ste_symbols: *mut _object,
    ste_name: *mut _object,
    ste_varnames: *mut _object,
    ste_children: *mut _object,
    ste_directives: *mut _object,
    ste_mangled_names: *mut _object,
    ste_type: u32,
    ste_scope_info: *const i8,
    ste_nested: i32,
    __slate_bits_0: __slate_bits::__SlateBits91U0,
    ste_comprehension: u32,
    __slate_bits_1: __slate_bits::__SlateBits91U1,
    ste_comp_iter_expr: i32,
    ste_loc: __SlateRecord_pycore_symtable_h_51,
    ste_annotation_block: *mut _symtable_entry,
    ste_table: *mut symtable,
}

// global stmt
// assignment in code block
// formal parameter
// nonlocal stmt
// name is used
// free variable from class's method
// assignment occurred via import
// this name is annotated
// this name is a comprehension iteration variable
// this name is a type parameter
// this name is a cell in an inlined comprehension
// GLOBAL_EXPLICIT and GLOBAL_IMPLICIT are used internally by the symbol
// table.  GLOBAL is returned from _PyST_GetScope() for either of them.
// It is stored in ste_symbols at bits 13-16.
static _symtable_symtable__doc__: __SlateAlign16<[i8; 119]> = __SlateAlign16([
    115i8, 121i8, 109i8, 116i8, 97i8, 98i8, 108i8, 101i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8,
    108i8, 101i8, 44i8, 32i8, 115i8, 111i8, 117i8, 114i8, 99i8, 101i8, 44i8, 32i8, 102i8, 105i8,
    108i8, 101i8, 110i8, 97i8, 109i8, 101i8, 44i8, 32i8, 115i8, 116i8, 97i8, 114i8, 116i8, 115i8,
    116i8, 114i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8,
    114i8, 110i8, 32i8, 115i8, 121i8, 109i8, 98i8, 111i8, 108i8, 32i8, 97i8, 110i8, 100i8, 32i8,
    115i8, 99i8, 111i8, 112i8, 101i8, 32i8, 100i8, 105i8, 99i8, 116i8, 105i8, 111i8, 110i8, 97i8,
    114i8, 105i8, 101i8, 115i8, 32i8, 117i8, 115i8, 101i8, 100i8, 32i8, 105i8, 110i8, 116i8, 101i8,
    114i8, 110i8, 97i8, 108i8, 108i8, 121i8, 32i8, 98i8, 121i8, 32i8, 99i8, 111i8, 109i8, 112i8,
    105i8, 108i8, 101i8, 114i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.symtablemodule._symtable_symtable")]
extern "C-unwind" fn _symtable_symtable(
    module: *mut _object,
    args: *const *mut _object,
    nargs: i64,
) -> *mut _object {
    let mut startstr_length: i64 = unsafe { core::mem::zeroed() };
    let mut startstr: *const i8 = unsafe { core::mem::zeroed() };
    let mut filename: *mut _object = unsafe { core::mem::zeroed() };
    let mut source: *mut _object = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        filename = core::ptr::null_mut::<_object>();
        if _PyArg_CheckPositional!(c"symtable".as_ptr() as *mut i8, nargs, 3i32, 3i32) {
            source = *args.wrapping_offset(0 as isize);
            if PyUnicode_FSDecoder(
                *args.wrapping_offset(1 as isize),
                core::ptr::addr_of_mut!(filename) as *mut (),
            ) != 0
            {
                if PyUnicode_Check!(unsafe { *args.wrapping_offset(2 as isize) }) == 0 {
                    _PyArg_BadArgument(
                        c"symtable".as_ptr(),
                        c"argument 3".as_ptr(),
                        c"str".as_ptr(),
                        *args.wrapping_offset(2 as isize),
                    );
                } else {
                    startstr = PyUnicode_AsUTF8AndSize(
                        *args.wrapping_offset(2 as isize),
                        core::ptr::addr_of_mut!(startstr_length),
                    );
                    if !startstr.is_null() {
                        if (strlen(startstr as *const core::ffi::c_char) as u64)
                            != (startstr_length as u64)
                        {
                            PyErr_SetString(PyExc_ValueError, c"embedded null character".as_ptr());
                        } else {
                            return_value =
                                _symtable_symtable_impl(module, source, filename, startstr);
                        }
                    }
                }
            }
        }
        Py_XDECREF!(filename); // Cleanup for filename
        return return_value;
    }
}

// struct symtable
// [clinic input]
// module _symtable
// [clinic start generated code]
//
// [clinic end generated code: output=da39a3ee5e6b4b0d input=f4685845a7100605]
// [clinic input]
// _symtable.symtable
//
//     source:    object
//     filename:  unicode_fs_decoded
//     startstr:  str
//     /
//
// Return symbol and scope dictionaries used internally by compiler.
// [clinic start generated code]
/// [clinic end generated code: output=59eb0d5fc7285ac4 input=436ffff90d02e4f6]
fn _symtable_symtable_impl(
    module: *mut _object,
    source: *mut _object,
    filename: *mut _object,
    startstr: *const i8,
) -> *mut _object {
    let mut __slate_compound_0: __SlateRecord_compile_h_30 = unsafe { core::mem::zeroed() };
    let mut start: i32 = 0;
    let mut cf: __SlateRecord_compile_h_30 = unsafe {
        *{
            __slate_compound_0 = __SlateRecord_compile_h_30 {
                cf_flags: 0,
                cf_feature_version: 14,
            };
            core::ptr::addr_of_mut!(__slate_compound_0)
        }
    };
    let mut source_copy: *mut _object = core::ptr::null_mut::<_object>();
    cf.cf_flags = 256;
    let str: *const i8 = unsafe {
        _Py_SourceAsString(
            source,
            c"symtable".as_ptr(),
            c"string or bytes".as_ptr(),
            core::ptr::addr_of_mut!(cf),
            core::ptr::addr_of_mut!(source_copy),
        )
    };
    if str.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    if (unsafe {
        strcmp(
            startstr as *const core::ffi::c_char,
            c"exec".as_ptr() as *const core::ffi::c_char,
        ) as i32
    }) == 0
    {
        start = 257;
    } else {
        if (unsafe {
            strcmp(
                startstr as *const core::ffi::c_char,
                c"eval".as_ptr() as *const core::ffi::c_char,
            ) as i32
        }) == 0
        {
            start = 258;
        } else {
            if (unsafe {
                strcmp(
                    startstr as *const core::ffi::c_char,
                    c"single".as_ptr() as *const core::ffi::c_char,
                ) as i32
            }) == 0
            {
                start = 256;
            } else {
                unsafe {
                    PyErr_SetString(
                        PyExc_ValueError,
                        c"symtable() arg 3 must be 'exec' or 'eval' or 'single'".as_ptr(),
                    )
                };
                Py_XDECREF!(source_copy);
                return core::ptr::null_mut::<_object>();
            }
        }
    }
    let st: *mut symtable =
        unsafe { _Py_SymtableStringObjectFlags(str, filename, start, core::ptr::addr_of_mut!(cf)) };
    Py_XDECREF!(source_copy);
    if st.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let t: *mut _object = Py_NewRef__2!(unsafe { (*st).st_top });
    unsafe { _PySymtable_Free(st) };
    t
}

/// sentinel
static mut symtable_methods: __SlateAlign16<[PyMethodDef; 2]> = __SlateAlign16([
    {
        let mut __t0: PyMethodDef = unsafe { core::mem::zeroed() };
        __t0.ml_name = (b"symtable\0".as_ptr() as *mut i8) as *const i8;
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
                >(Some(_symtable_symtable))
            })
        };
        __t0.ml_flags = 128i32;
        __t0.ml_doc = unsafe { core::ptr::addr_of!(_symtable_symtable__doc__.0) as *const i8 };
        __t0
    },
    {
        let mut __t1: PyMethodDef = unsafe { core::mem::zeroed() };
        __t1.ml_name = core::ptr::null::<i8>();
        __t1.ml_meth = None;
        __t1
    },
]);

#[unsafe(link_section = ".text.slate_distinct.symtablemodule.symtable_init_constants")]
extern "C-unwind" fn symtable_init_constants(m: *mut _object) -> i32 {
    if (unsafe { PyModule_AddIntConstant(m, c"USE".as_ptr(), 16i64) }) < 0 {
        return 1i32.wrapping_neg();
    }
    if (unsafe { PyModule_AddIntConstant(m, c"DEF_GLOBAL".as_ptr(), 1i64) }) < 0 {
        return 1i32.wrapping_neg();
    }
    if (unsafe { PyModule_AddIntConstant(m, c"DEF_NONLOCAL".as_ptr(), 8i64) }) < 0 {
        return 1i32.wrapping_neg();
    }
    if (unsafe { PyModule_AddIntConstant(m, c"DEF_LOCAL".as_ptr(), 2i64) }) < 0 {
        return 1i32.wrapping_neg();
    }
    if (unsafe { PyModule_AddIntConstant(m, c"DEF_PARAM".as_ptr(), 4i64) }) < 0 {
        return 1i32.wrapping_neg();
    }
    if (unsafe { PyModule_AddIntConstant(m, c"DEF_TYPE_PARAM".as_ptr(), 1024i64) }) < 0 {
        return 1i32.wrapping_neg();
    }
    if (unsafe { PyModule_AddIntConstant(m, c"DEF_FREE_CLASS".as_ptr(), 64i64) }) < 0 {
        return 1i32.wrapping_neg();
    }
    if (unsafe { PyModule_AddIntConstant(m, c"DEF_IMPORT".as_ptr(), 128i64) }) < 0 {
        return 1i32.wrapping_neg();
    }
    if (unsafe { PyModule_AddIntConstant(m, c"DEF_BOUND".as_ptr(), 134i64) }) < 0 {
        return 1i32.wrapping_neg();
    }
    if (unsafe { PyModule_AddIntConstant(m, c"DEF_ANNOT".as_ptr(), 256i64) }) < 0 {
        return 1i32.wrapping_neg();
    }
    if (unsafe { PyModule_AddIntConstant(m, c"DEF_COMP_ITER".as_ptr(), 512i64) }) < 0 {
        return 1i32.wrapping_neg();
    }
    if (unsafe { PyModule_AddIntConstant(m, c"DEF_COMP_CELL".as_ptr(), 2048i64) }) < 0 {
        return 1i32.wrapping_neg();
    }
    if (unsafe { PyModule_AddIntConstant(m, c"TYPE_FUNCTION".as_ptr(), 0i64) }) < 0 {
        return 1i32.wrapping_neg();
    }
    if (unsafe { PyModule_AddIntConstant(m, c"TYPE_CLASS".as_ptr(), 1i64) }) < 0 {
        return 1i32.wrapping_neg();
    }
    if (unsafe { PyModule_AddIntConstant(m, c"TYPE_MODULE".as_ptr(), 2i64) }) < 0 {
        return 1i32.wrapping_neg();
    }
    if (unsafe { PyModule_AddIntConstant(m, c"TYPE_ANNOTATION".as_ptr(), 3i64) }) < 0 {
        return 1i32.wrapping_neg();
    }
    if (unsafe { PyModule_AddIntConstant(m, c"TYPE_TYPE_ALIAS".as_ptr(), 4i64) }) < 0 {
        return 1i32.wrapping_neg();
    }
    if (unsafe { PyModule_AddIntConstant(m, c"TYPE_TYPE_PARAMETERS".as_ptr(), 5i64) }) < 0 {
        return 1i32.wrapping_neg();
    }
    if (unsafe { PyModule_AddIntConstant(m, c"TYPE_TYPE_VARIABLE".as_ptr(), 6i64) }) < 0 {
        return 1i32.wrapping_neg();
    }
    if (unsafe { PyModule_AddIntConstant(m, c"LOCAL".as_ptr(), 1i64) }) < 0 {
        return 1i32.wrapping_neg();
    }
    if (unsafe { PyModule_AddIntConstant(m, c"GLOBAL_EXPLICIT".as_ptr(), 2i64) }) < 0 {
        return 1i32.wrapping_neg();
    }
    if (unsafe { PyModule_AddIntConstant(m, c"GLOBAL_IMPLICIT".as_ptr(), 3i64) }) < 0 {
        return 1i32.wrapping_neg();
    }
    if (unsafe { PyModule_AddIntConstant(m, c"FREE".as_ptr(), 4i64) }) < 0 {
        return 1i32.wrapping_neg();
    }
    if (unsafe { PyModule_AddIntConstant(m, c"CELL".as_ptr(), 5i64) }) < 0 {
        return 1i32.wrapping_neg();
    }
    if (unsafe { PyModule_AddIntConstant(m, c"SCOPE_OFF".as_ptr(), 12i64) }) < 0 {
        return 1i32.wrapping_neg();
    }
    if (unsafe { PyModule_AddIntConstant(m, c"SCOPE_MASK".as_ptr(), (1 | 2 | 4i32 | 8i32) as i64) })
        < 0
    {
        return 1i32.wrapping_neg();
    }
    0
}

static mut symtable_slots: __SlateAlign16<[PyModuleDef_Slot; 4]> = __SlateAlign16([
    {
        let mut __t0: PyModuleDef_Slot = unsafe { core::mem::zeroed() };
        __t0.slot = 2i32;
        __t0.value = unsafe {
            core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut _object) -> i32>, *mut ()>(
                Some(symtable_init_constants),
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

static mut symtablemodule: PyModuleDef = {
    let mut __t0: PyModuleDef = unsafe { core::mem::zeroed() };
    __t0.m_base = PyModuleDef_Base {
        ob_base: _object {
            __slate_anon_0: __SlateRecord_object_h_122 {
                ob_refcnt_full: 1407378104778752i64,
            },
            ob_type: core::ptr::null_mut::<_typeobject>(),
        },
        m_init: None,
        m_index: 0i64,
        m_copy: core::ptr::null_mut::<_object>(),
    };
    __t0.m_name = (b"_symtable\0".as_ptr() as *mut i8) as *const i8;
    __t0.m_size = 0i64;
    __t0.m_methods = unsafe { core::ptr::addr_of_mut!(symtable_methods.0) as *mut PyMethodDef };
    __t0.m_slots = unsafe { core::ptr::addr_of_mut!(symtable_slots.0) as *mut PyModuleDef_Slot };
    __t0
};

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.symtablemodule.PyInit__symtable")]
extern "C-unwind" fn PyInit__symtable() -> *mut _object {
    unsafe { PyModuleDef_Init(core::ptr::addr_of_mut!(symtablemodule)) }
}
