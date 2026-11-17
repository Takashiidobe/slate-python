//! Module support implementation
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
//! Bit flags for _gc_prev
//!
//! Bit 0 is set when tp_finalize is called
//! _PyGenObject_HEAD defines the initial segment of generator
//! and coroutine objects.
//! These would be in pycore_long.h if it weren't for an include cycle.
//! Fast inlined version of PyType_IS_GC()
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
macro_rules! PyList_SET_ITEM {
    ($op:expr, $index:expr, $value:expr) => {
        PyList_SET_ITEM(_PyObject_CAST!($op), $index, _PyObject_CAST!($value));
    };
}

macro_rules! PyModule_Check {
    ($op:expr) => {
        PyObject_TypeCheck!($op, unsafe { core::ptr::addr_of_mut!(PyModule_Type) })
    };
}

macro_rules! PyObject_TypeCheck {
    ($ob:expr, $type:expr) => {
        PyObject_TypeCheck(_PyObject_CAST!($ob), $type)
    };
}

macro_rules! PyTuple_SET_ITEM__2 {
    ($op:expr, $index:expr, $value:expr) => {
        PyTuple_SET_ITEM(_PyObject_CAST!($op), $index, _PyObject_CAST!($value));
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

macro_rules! Py_NewRef {
    ($obj:expr) => {
        _Py_NewRef(_PyObject_CAST!($obj))
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

/// Cast argument to PyTupleObject* type.
macro_rules! _PyTuple_CAST {
    ($op:expr) => {
        _Py_CAST!(*mut __SlateRecord_tupleobject_h_13, $op)
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

macro_rules! _Py_PACK_FULL_VERSION {
    ($X:expr, $Y:expr, $Z:expr, $LEVEL:expr, $SERIAL:expr) => {
        (($X) & 255) << 24i32
            | (($Y) & 255) << 16i32
            | (($Z) & 255) << 8i32
            | (($LEVEL) & 15) << 4i32
            | (($SERIAL) & 15) << 0i32
    };
}

unsafe extern "C" {
    static mut _Py_NoneStruct: _object;
    static mut PyExc_OverflowError: *mut _object;
    static mut PyExc_SystemError: *mut _object;
    static mut PyExc_TypeError: *mut _object;
    static mut PyModule_Type: _typeobject;
    fn index(arg0: *const i8, arg1: i32) -> *mut i8;
    fn PyMem_Malloc(arg0: u64) -> *mut ();
    fn PyMem_Free(arg0: *mut ());
    fn PyType_IsSubtype(arg0: *mut _typeobject, arg1: *mut _typeobject) -> i32;
    fn PyType_Ready(arg0: *mut _typeobject) -> i32;
    fn _PyType_Name(arg0: *mut _typeobject) -> *const i8;
    fn _Py_Dealloc(arg0: *mut _object);
    fn PyBytes_FromStringAndSize(arg0: *const i8, arg1: i64) -> *mut _object;
    fn PyUnicode_FromStringAndSize(arg0: *const i8, arg1: i64) -> *mut _object;
    fn PyUnicode_FromString(arg0: *const i8) -> *mut _object;
    fn PyUnicode_FromWideChar(arg0: *const i32, arg1: i64) -> *mut _object;
    fn PyUnicode_FromOrdinal(arg0: i32) -> *mut _object;
    fn PyErr_SetString(arg0: *mut _object, arg1: *const i8);
    fn PyErr_Occurred() -> *mut _object;
    fn PyErr_GetRaisedException() -> *mut _object;
    fn PyErr_SetRaisedException(arg0: *mut _object);
    fn PyErr_NoMemory() -> *mut _object;
    fn PyErr_Format(arg0: *mut _object, arg1: *const i8, ...) -> *mut _object;
    fn PyLong_FromLong(arg0: i64) -> *mut _object;
    fn PyLong_FromUnsignedLong(arg0: u64) -> *mut _object;
    fn PyLong_FromLongLong(arg0: i64) -> *mut _object;
    fn PyLong_FromUnsignedLongLong(arg0: u64) -> *mut _object;
    fn PyBool_FromLong(arg0: i64) -> *mut _object;
    fn PyFloat_FromDouble(arg0: f64) -> *mut _object;
    fn PyComplex_FromCComplex(arg0: __SlateRecord_complexobject_h_8) -> *mut _object;
    fn PyTuple_New(arg0: i64) -> *mut _object;
    fn PyList_New(arg0: i64) -> *mut _object;
    fn PyDict_New() -> *mut _object;
    fn PyDict_SetItem(arg0: *mut _object, arg1: *mut _object, arg2: *mut _object) -> i32;
    fn PyDict_SetItemString(arg0: *mut _object, arg1: *const i8, arg2: *mut _object) -> i32;
    fn PyModule_GetDict(arg0: *mut _object) -> *mut _object;
    fn PyModule_GetName(arg0: *mut _object) -> *const i8;
    fn PyNumber_AsSsize_t(arg0: *mut _object, arg1: *mut _object) -> i64;
    fn _PyType_GetDict(arg0: *mut _typeobject) -> *mut _object;
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

// Track types initialized using _PyStaticType_InitBuiltin().
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

// Define a pair of assertion macros:
// _PyObject_ASSERT_FROM(), _PyObject_ASSERT_WITH_MSG() and _PyObject_ASSERT().
//
// These work like the regular C assert(), in that they will abort the
// process with a message on stderr if the given condition fails to hold,
// but compile away to nothing if NDEBUG is defined.
//
// However, before aborting, Python will also try to call _PyObject_Dump() on
// the given object.  This may be of use when investigating bugs in which a
// particular object is corrupt (e.g. buggy a tp_visit method in an extension
// module breaking the garbage collector), to help locate the broken objects.
//
// The WITH_MSG variant allows you to supply an additional message that Python
// will attempt to print to stderr, after the object dump.
//
// No debugging: compile away the assertions:
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

use slate_support::strlen;
use slate_support::wcslen;

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
struct __SlateRecord_complexobject_h_8 {
    real: f64,
    imag: f64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_tupleobject_h_13 {
    ob_base: __SlateRecord_object_h_172,
    ob_hash: i64,
    ob_item: [*mut _object; 1],
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

fn _PyIndex_Check(obj: *mut _object) -> i32 {
    let tp_as_number: *mut __SlateRecord_object_h_106 = unsafe { (*Py_TYPE!(obj)).tp_as_number };
    (!tp_as_number.is_null() && (unsafe { (*tp_as_number).nb_index }) != None) as i32
}

fn _PyType_IsReady(r#type: *mut _typeobject) -> i32 {
    !unsafe { _PyType_GetDict(r#type) }.is_null() as i32
}

// _PyIndex_Check()
// _PyType_IsReady()
#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_convert_optional_to_ssize_t(obj: *mut _object, result: *mut ()) -> i32 {
    let mut limit: i64 = 0;
    if obj == unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } {
        return 1;
    } else {
        if _PyIndex_Check(obj) != 0 {
            limit = unsafe { PyNumber_AsSsize_t(obj, PyExc_OverflowError) };
            let __v0: bool =
                limit == (1i32.wrapping_neg() as i64) && !unsafe { PyErr_Occurred() }.is_null();
            if __v0 {
                return 0;
            }
        } else {
            unsafe {
                PyErr_Format(
                    PyExc_TypeError,
                    c"argument should be integer or None, not '%.200s'".as_ptr(),
                    (*Py_TYPE!(obj)).tp_name,
                )
            };
            return 0;
        }
    }
    unsafe {
        *(result as *mut i64) = limit;
    }
    1
}

// Helper for mkvalue() to scan the length of a format
fn countformat(mut format: *const i8, endchar: i8) -> i64 {
    let mut count: i64 = 0;
    let mut level: i32 = 0;
    while level > 0 || ((unsafe { *format }) as i32) != (endchar as i32) {
        match (unsafe { *format }) as i32 {
            0 => {
                unsafe {
                    PyErr_SetString(PyExc_SystemError, c"unmatched paren in format".as_ptr())
                }; // Premature end
                return 1i32.wrapping_neg() as i64;
            }
            40 | 91 | 123 => {
                if level == 0 {
                    count = count.wrapping_add(1);
                }
                level = level.wrapping_add(1);
            }
            41 | 93 | 125 => {
                level = level.wrapping_sub(1);
            }
            9 | 32 | 35 | 38 | 44 | 58 => {}
            _ => {
                if level == 0 {
                    count = count.wrapping_add(1);
                }
            }
        }
        let __v9: *const i8 = format;
        format = __v9.wrapping_offset(1 as isize);
    }
    count
}

// Generic function to create a value -- the inverse of getargs()
//
// After an original idea and first implementation by Steven Miale
fn check_end(p_format: &mut *const i8, endchar: i8) -> i32 {
    let mut f: *const i8 = unsafe { *(p_format as *mut *const i8) };
    while ((unsafe { *f }) as i32) != (endchar as i32) {
        if ((unsafe { *f }) as i32) != 32
            && ((unsafe { *f }) as i32) != 9
            && ((unsafe { *f }) as i32) != 44
            && ((unsafe { *f }) as i32) != 58
        {
            unsafe { PyErr_SetString(PyExc_SystemError, c"Unmatched paren in format".as_ptr()) };
            return 0;
        }
        let __v0: *const i8 = f;
        f = __v0.wrapping_offset(1 as isize);
    }
    if endchar != 0 {
        let __v3: *const i8 = f;
        f = __v3.wrapping_offset(1 as isize);
    }
    unsafe {
        *(p_format as *mut *const i8) = f;
    }
    1
}

fn do_ignore(p_format: &mut *const i8, p_va: &mut core::ffi::VaList<'static>, endchar: i8, n: i64) {
    let v: *mut _object = unsafe { PyTuple_New(n) };
    for i in 0..n {
        let exc: *mut _object = unsafe { PyErr_GetRaisedException() };
        let w: *mut _object = do_mkvalue(p_format, p_va);
        unsafe { PyErr_SetRaisedException(exc) };
        if !w.is_null() {
            if !v.is_null() {
                PyTuple_SET_ITEM__2!(v, i, w);
            } else {
                Py_DECREF!(w);
            }
        }
    }
    Py_XDECREF!(v);
    if check_end(p_format, endchar) == 0 {
        return;
    }
}

fn do_mkdict(
    p_format: &mut *const i8,
    p_va: &mut core::ffi::VaList<'static>,
    endchar: i8,
    n: i64,
) -> *mut _object {
    let mut i: i64 = 0;
    if n < 0 {
        return core::ptr::null_mut::<_object>();
    }
    if n % 2 != 0 {
        unsafe { PyErr_SetString(PyExc_SystemError, c"Bad dict format".as_ptr()) };
        do_ignore(p_format, p_va, endchar, n);
        return core::ptr::null_mut::<_object>();
    }
    // Note that we can't bail immediately on error as this will leak
    // refcounts on any 'N' arguments.
    let __v0: *mut _object = unsafe { PyDict_New() };
    let d: *mut _object = __v0;
    if __v0.is_null() {
        do_ignore(p_format, p_va, endchar, n);
        return core::ptr::null_mut::<_object>();
    }
    while i < n {
        let k: *mut _object = do_mkvalue(p_format, p_va);
        if k.is_null() {
            do_ignore(p_format, p_va, endchar, n.wrapping_sub(i).wrapping_sub(1));
            Py_DECREF!(d);
            return core::ptr::null_mut::<_object>();
        }
        let v: *mut _object = do_mkvalue(p_format, p_va);
        let __v1: bool = v.is_null() || (unsafe { PyDict_SetItem(d, k, v) }) < 0;
        if __v1 {
            do_ignore(p_format, p_va, endchar, n.wrapping_sub(i).wrapping_sub(2));
            Py_DECREF!(k);
            Py_XDECREF!(v);
            Py_DECREF!(d);
            return core::ptr::null_mut::<_object>();
        }
        Py_DECREF!(k);
        Py_DECREF!(v);
        i = i.wrapping_add(2);
    }
    if check_end(p_format, endchar) == 0 {
        Py_DECREF!(d);
        return core::ptr::null_mut::<_object>();
    }
    d
}

fn do_mklist(
    p_format: &mut *const i8,
    p_va: &mut core::ffi::VaList<'static>,
    endchar: i8,
    n: i64,
) -> *mut _object {
    if n < 0 {
        return core::ptr::null_mut::<_object>();
    }
    // Note that we can't bail immediately on error as this will leak
    // refcounts on any 'N' arguments.
    let v: *mut _object = unsafe { PyList_New(n) };
    if v.is_null() {
        do_ignore(p_format, p_va, endchar, n);
        return core::ptr::null_mut::<_object>();
    }
    for i in 0..n {
        let w: *mut _object = do_mkvalue(p_format, p_va);
        if w.is_null() {
            do_ignore(p_format, p_va, endchar, n.wrapping_sub(i).wrapping_sub(1));
            Py_DECREF!(v);
            return core::ptr::null_mut::<_object>();
        }
        PyList_SET_ITEM!(v, i, w);
    }
    if check_end(p_format, endchar) == 0 {
        Py_DECREF!(v);
        return core::ptr::null_mut::<_object>();
    }
    v
}

fn do_mkstack(
    stack: *mut *mut _object,
    p_format: &mut *const i8,
    p_va: &mut core::ffi::VaList<'static>,
    endchar: i8,
    mut n: i64,
) -> i32 {
    let mut w: *mut _object = unsafe { core::mem::zeroed() };
    let mut i: i64 = unsafe { core::mem::zeroed() };
    unsafe {
        if n < 0 {
            return 1i32.wrapping_neg();
        } else {
            // Note that we can't bail immediately on error as this will leak
            // refcounts on any 'N' arguments.
            i = 0;
            '__join_3: {
                '__join_8: {
                    while i < n {
                        w = do_mkvalue(p_format, p_va);
                        if w.is_null() {
                            break '__join_8;
                        } else {
                            *stack.wrapping_offset(i as isize) = w;
                            i = i.wrapping_add(1);
                        }
                    }
                    if check_end(p_format, endchar) == 0 {
                        break '__join_3;
                    } else {
                        return 0;
                    }
                }
                do_ignore(p_format, p_va, endchar, n.wrapping_sub(i).wrapping_sub(1));
            }
            n = i;
            i = 0;
            while i < n {
                Py_DECREF!(unsafe { *stack.wrapping_offset(i as isize) });
                i = i.wrapping_add(1);
            }
            return 1i32.wrapping_neg();
        }
    }
}

fn do_mktuple(
    p_format: &mut *const i8,
    p_va: &mut core::ffi::VaList<'static>,
    endchar: i8,
    n: i64,
) -> *mut _object {
    if n < 0 {
        return core::ptr::null_mut::<_object>();
    }
    // Note that we can't bail immediately on error as this will leak
    // refcounts on any 'N' arguments.
    let __v0: *mut _object = unsafe { PyTuple_New(n) };
    let v: *mut _object = __v0;
    if __v0.is_null() {
        do_ignore(p_format, p_va, endchar, n);
        return core::ptr::null_mut::<_object>();
    }
    for i in 0..n {
        let w: *mut _object = do_mkvalue(p_format, p_va);
        if w.is_null() {
            do_ignore(p_format, p_va, endchar, n.wrapping_sub(i).wrapping_sub(1));
            Py_DECREF!(v);
            return core::ptr::null_mut::<_object>();
        }
        PyTuple_SET_ITEM__2!(v, i, w);
    }
    if check_end(p_format, endchar) == 0 {
        Py_DECREF!(v);
        return core::ptr::null_mut::<_object>();
    }
    v
}

fn do_mkvalue(p_format: &mut *const i8, p_va: &mut core::ffi::VaList<'static>) -> *mut _object {
    '__slate_break_16: loop {
        let __v1: *const i8 = unsafe { *p_format };
        unsafe {
            *p_format = __v1.wrapping_offset(1 as isize);
        }
        // Fall through from 'n' to 'l' if Py_ssize_t is long
        // XXX deprecated alias
        // If a NULL was passed
        // because a call that should
        // have constructed a value
        // failed, that's OK, and we
        // pass the error on; but if
        // no error occurred it's not
        // clear that the caller knew
        // what she was doing.
        match (unsafe { *__v1 }) as i32 {
            40 => {
                return {
                    let __arg3 = countformat(unsafe { *(p_format as *mut *const i8) }, 41i8);
                    do_mktuple(p_format, p_va, 41i8, __arg3)
                };
            }
            91 => {
                return {
                    let __arg3 = countformat(unsafe { *(p_format as *mut *const i8) }, 93i8);
                    do_mklist(p_format, p_va, 93i8, __arg3)
                };
            }
            123 => {
                return {
                    let __arg3 = countformat(unsafe { *(p_format as *mut *const i8) }, 125i8);
                    do_mkdict(p_format, p_va, 125i8, __arg3)
                };
            }
            66 | 98 | 104 | 105 => {
                return unsafe {
                    PyLong_FromLong(
                        (*core::ptr::addr_of_mut!(*(p_va as *mut core::ffi::VaList<'static>)))
                            .next_arg::<i32>() as i64,
                    )
                };
            }
            72 => {
                return unsafe {
                    PyLong_FromLong(
                        (*core::ptr::addr_of_mut!(*(p_va as *mut core::ffi::VaList<'static>)))
                            .next_arg::<u32>() as i64,
                    )
                };
            }
            73 => {
                let n: u32 = unsafe {
                    (*core::ptr::addr_of_mut!(*(p_va as *mut core::ffi::VaList<'static>)))
                        .next_arg::<u32>()
                };
                return unsafe { PyLong_FromUnsignedLong(n as u64) };
            }
            108 | 110 => {
                // Fall through from 'n' to 'l' if Py_ssize_t is long
                return unsafe {
                    PyLong_FromLong(
                        (*core::ptr::addr_of_mut!(*(p_va as *mut core::ffi::VaList<'static>)))
                            .next_arg::<i64>(),
                    )
                };
            }
            107 => {
                let n: u64 = unsafe {
                    (*core::ptr::addr_of_mut!(*(p_va as *mut core::ffi::VaList<'static>)))
                        .next_arg::<u64>()
                };
                return unsafe { PyLong_FromUnsignedLong(n) };
            }
            76 => {
                return unsafe {
                    PyLong_FromLongLong(
                        (*core::ptr::addr_of_mut!(*(p_va as *mut core::ffi::VaList<'static>)))
                            .next_arg::<i64>(),
                    )
                };
            }
            75 => {
                return unsafe {
                    PyLong_FromUnsignedLongLong(
                        (*core::ptr::addr_of_mut!(*(p_va as *mut core::ffi::VaList<'static>)))
                            .next_arg::<u64>(),
                    )
                };
            }
            117 => {
                let mut v: *mut _object = unsafe { core::mem::zeroed() };
                let u: *const i32 = (unsafe {
                    (*core::ptr::addr_of_mut!(*(p_va as *mut core::ffi::VaList<'static>)))
                        .next_arg::<*mut i32>()
                }) as *const i32;
                let mut n: i64 = if ((unsafe { *(*(p_format as *mut *const i8)) }) as i32) == 35 {
                    let __v5: *const i8 = unsafe { *p_format };
                    unsafe {
                        *p_format = __v5.wrapping_offset(1 as isize);
                    }
                    unsafe {
                        (*core::ptr::addr_of_mut!(*(p_va as *mut core::ffi::VaList<'static>)))
                            .next_arg::<i64>()
                    }
                } else {
                    1i32.wrapping_neg() as i64
                };
                if u.is_null() {
                    v = Py_NewRef!(unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) });
                } else {
                    if n < 0 {
                        n = (unsafe { wcslen(u as *const i32) as u64 }) as i64;
                    }
                    v = unsafe { PyUnicode_FromWideChar(u, n) };
                }
                return v;
            }
            100 | 102 => {
                return unsafe {
                    PyFloat_FromDouble(
                        (*core::ptr::addr_of_mut!(*(p_va as *mut core::ffi::VaList<'static>)))
                            .next_arg::<f64>(),
                    )
                };
            }
            68 => {
                return unsafe {
                    PyComplex_FromCComplex(
                        *(*core::ptr::addr_of_mut!(*(p_va as *mut core::ffi::VaList<'static>)))
                            .next_arg::<*mut __SlateRecord_complexobject_h_8>(),
                    )
                };
            }
            99 => {
                let mut p: [i8; 1] = [0i8; 1];
                p[0] = (unsafe {
                    (*core::ptr::addr_of_mut!(*(p_va as *mut core::ffi::VaList<'static>)))
                        .next_arg::<i32>()
                }) as i8;
                return unsafe { PyBytes_FromStringAndSize(p.as_mut_ptr() as *const i8, 1i64) };
            }
            67 => {
                let i: i32 = unsafe {
                    (*core::ptr::addr_of_mut!(*(p_va as *mut core::ffi::VaList<'static>)))
                        .next_arg::<i32>()
                };
                return unsafe { PyUnicode_FromOrdinal(i) };
            }
            112 => {
                let i: i32 = unsafe {
                    (*core::ptr::addr_of_mut!(*(p_va as *mut core::ffi::VaList<'static>)))
                        .next_arg::<i32>()
                };
                return unsafe { PyBool_FromLong(i as i64) };
            }
            85 | 115 | 122 => {
                let mut v: *mut _object = unsafe { core::mem::zeroed() };
                let str: *const i8 = unsafe {
                    (*core::ptr::addr_of_mut!(*(p_va as *mut core::ffi::VaList<'static>)))
                        .next_arg::<*const i8>()
                };
                let mut n: i64 = if ((unsafe { *(*(p_format as *mut *const i8)) }) as i32) == 35 {
                    let __v8: *const i8 = unsafe { *p_format };
                    unsafe {
                        *p_format = __v8.wrapping_offset(1 as isize);
                    }
                    unsafe {
                        (*core::ptr::addr_of_mut!(*(p_va as *mut core::ffi::VaList<'static>)))
                            .next_arg::<i64>()
                    }
                } else {
                    1i32.wrapping_neg() as i64
                };
                if str.is_null() {
                    v = Py_NewRef!(unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) });
                } else {
                    if n < 0 {
                        let m: u64 = unsafe { strlen(str as *const core::ffi::c_char) as u64 };
                        if m > 9223372036854775807 {
                            unsafe {
                                PyErr_SetString(
                                    PyExc_OverflowError,
                                    c"string too long for Python string".as_ptr(),
                                )
                            };
                            return core::ptr::null_mut::<_object>();
                        }
                        n = m as i64;
                    }
                    v = unsafe { PyUnicode_FromStringAndSize(str, n) };
                }
                return v;
                // XXX deprecated alias
            }
            121 => {
                let mut v: *mut _object = unsafe { core::mem::zeroed() };
                let str: *const i8 = unsafe {
                    (*core::ptr::addr_of_mut!(*(p_va as *mut core::ffi::VaList<'static>)))
                        .next_arg::<*const i8>()
                };
                let mut n: i64 = if ((unsafe { *(*(p_format as *mut *const i8)) }) as i32) == 35 {
                    let __v11: *const i8 = unsafe { *p_format };
                    unsafe {
                        *p_format = __v11.wrapping_offset(1 as isize);
                    }
                    unsafe {
                        (*core::ptr::addr_of_mut!(*(p_va as *mut core::ffi::VaList<'static>)))
                            .next_arg::<i64>()
                    }
                } else {
                    1i32.wrapping_neg() as i64
                };
                if str.is_null() {
                    v = Py_NewRef!(unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) });
                } else {
                    if n < 0 {
                        let m: u64 = unsafe { strlen(str as *const core::ffi::c_char) as u64 };
                        if m > 9223372036854775807 {
                            unsafe {
                                PyErr_SetString(
                                    PyExc_OverflowError,
                                    c"string too long for Python bytes".as_ptr(),
                                )
                            };
                            return core::ptr::null_mut::<_object>();
                        }
                        n = m as i64;
                    }
                    v = unsafe { PyBytes_FromStringAndSize(str, n) };
                }
                return v;
            }
            78 | 79 | 83 => {
                if ((unsafe { *(*(p_format as *mut *const i8)) }) as i32) == 38 {
                    let func: Option<unsafe extern "C-unwind" fn(*mut ()) -> *mut _object> = unsafe {
                        unsafe {
                            core::mem::transmute::<
                                *const u8,
                                Option<unsafe extern "C-unwind" fn(*mut ()) -> *mut _object>,
                            >(
                                (*core::ptr::addr_of_mut!(
                                    *(p_va as *mut core::ffi::VaList<'static>)
                                ))
                                .next_arg::<*const u8>(),
                            )
                        }
                    };
                    let arg: *mut () = unsafe {
                        (*core::ptr::addr_of_mut!(*(p_va as *mut core::ffi::VaList<'static>)))
                            .next_arg::<*mut ()>()
                    };
                    let __v14: *const i8 = unsafe { *p_format };
                    unsafe {
                        *p_format = __v14.wrapping_offset(1 as isize);
                    }
                    return unsafe { func.unwrap()(arg) };
                } else {
                    let v: *mut _object = unsafe {
                        (*core::ptr::addr_of_mut!(*(p_va as *mut core::ffi::VaList<'static>)))
                            .next_arg::<*mut _object>()
                    };
                    if !v.is_null() {
                        if ((unsafe {
                            *unsafe { *(p_format as *mut *const i8) }.wrapping_offset(-(1 as isize))
                        }) as i32)
                            != 78
                        {
                            Py_INCREF!(v);
                        }
                    } else {
                        if unsafe { PyErr_Occurred() }.is_null() {
                            unsafe {
                                PyErr_SetString(
                                    PyExc_SystemError,
                                    c"NULL object passed to Py_BuildValue".as_ptr(),
                                )
                            };
                        }
                    }
                    // If a NULL was passed
                    // because a call that should
                    // have constructed a value
                    // failed, that's OK, and we
                    // pass the error on; but if
                    // no error occurred it's not
                    // clear that the caller knew
                    // what she was doing.
                    return v;
                }
            }
            9 | 32 | 44 | 58 => {}
            _ => {
                unsafe {
                    PyErr_SetString(
                        PyExc_SystemError,
                        c"bad format char passed to Py_BuildValue".as_ptr(),
                    )
                };
                return core::ptr::null_mut::<_object>();
            }
        }
    }
    unsafe { core::mem::zeroed() }
}

#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn Py_BuildValue(format: *const i8, mut __va_args: ...) -> *mut _object {
    let mut va: core::ffi::VaList<'static> = unsafe { core::mem::zeroed() };
    va = unsafe {
        core::mem::transmute::<core::ffi::VaList<'_>, core::ffi::VaList<'static>>(__va_args.clone())
    };
    let retval: *mut _object = va_build_value(format, va.clone());
    retval
}

/// abi only
#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn _Py_BuildValue_SizeT(
    format: *const i8,
    mut __va_args: ...
) -> *mut _object {
    let mut va: core::ffi::VaList<'static> = unsafe { core::mem::zeroed() };
    va = unsafe {
        core::mem::transmute::<core::ffi::VaList<'_>, core::ffi::VaList<'static>>(__va_args.clone())
    };
    let retval: *mut _object = va_build_value(format, va.clone());
    retval
}

#[unsafe(no_mangle)]
extern "C-unwind" fn Py_VaBuildValue(
    format: *const i8,
    mut va: core::ffi::VaList<'static>,
) -> *mut _object {
    va_build_value(format, va.clone())
}

/// abi only
#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_VaBuildValue_SizeT(
    format: *const i8,
    mut va: core::ffi::VaList<'static>,
) -> *mut _object {
    va_build_value(format, va.clone())
}

fn va_build_value(mut f: *const i8, mut va: core::ffi::VaList<'static>) -> *mut _object {
    let n: i64 = countformat(f, 0i8);
    let mut lva: core::ffi::VaList<'static> = unsafe { core::mem::zeroed() };
    let mut retval: *mut _object = unsafe { core::mem::zeroed() };
    if n < 0 {
        return core::ptr::null_mut::<_object>();
    }
    if n == 0 {
        return unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) };
    }
    lva = va.clone();
    if n == 1 {
        retval = do_mkvalue(unsafe { &mut (*core::ptr::addr_of_mut!(f)) }, unsafe {
            &mut (*core::ptr::addr_of_mut!(lva))
        });
    } else {
        retval = do_mktuple(
            unsafe { &mut (*core::ptr::addr_of_mut!(f)) },
            unsafe { &mut (*core::ptr::addr_of_mut!(lva)) },
            0i8,
            n,
        );
    }
    retval
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_VaBuildStack(
    small_stack: *mut *mut _object,
    small_stack_len: i64,
    format: *const i8,
    mut va: core::ffi::VaList<'static>,
    p_nargs: *mut i64,
) -> *mut *mut _object {
    let mut f: *const i8 = unsafe { core::mem::zeroed() };
    let mut n: i64 = 0;
    let mut lva: core::ffi::VaList<'static> = unsafe { core::mem::zeroed() };
    let mut stack: *mut *mut _object = unsafe { core::mem::zeroed() };
    let mut res: i32 = 0;
    n = countformat(format, 0i8);
    if n < 0 {
        unsafe {
            *p_nargs = 0;
        }
        return core::ptr::null_mut::<*mut _object>();
    }
    if n == 0 {
        unsafe {
            *p_nargs = 0;
        }
        return small_stack;
    }
    if n <= small_stack_len {
        stack = small_stack;
    } else {
        stack = (unsafe {
            PyMem_Malloc((n as u64).wrapping_mul(core::mem::size_of::<*mut _object>() as u64))
        }) as *mut *mut _object;
        if stack.is_null() {
            unsafe { PyErr_NoMemory() };
            return core::ptr::null_mut::<*mut _object>();
        }
    }
    lva = va.clone();
    f = format;
    res = do_mkstack(
        stack,
        unsafe { &mut (*core::ptr::addr_of_mut!(f)) },
        unsafe { &mut (*core::ptr::addr_of_mut!(lva)) },
        0i8,
        n,
    );
    if res < 0 {
        if stack != small_stack {
            unsafe { PyMem_Free(stack as *mut ()) };
        }
        return core::ptr::null_mut::<*mut _object>();
    }
    unsafe {
        *p_nargs = n;
    }
    stack
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyModule_AddObjectRef(
    r#mod: *mut _object,
    name: *const i8,
    value: *mut _object,
) -> i32 {
    if PyModule_Check!(r#mod) == 0 {
        unsafe {
            PyErr_SetString(
                PyExc_TypeError,
                c"PyModule_AddObjectRef() first argument must be a module".as_ptr(),
            )
        };
        return 1i32.wrapping_neg();
    }
    if value.is_null() {
        if unsafe { PyErr_Occurred() }.is_null() {
            unsafe {
                PyErr_SetString(PyExc_SystemError, c"PyModule_AddObjectRef() must be called with an exception raised if value is NULL".as_ptr())
            };
        }
        return 1i32.wrapping_neg();
    }
    let dict: *mut _object = unsafe { PyModule_GetDict(r#mod) };
    if dict.is_null() {
        // Internal error -- modules must have a dict!
        unsafe {
            PyErr_Format(
                PyExc_SystemError,
                c"module '%s' has no __dict__".as_ptr(),
                PyModule_GetName(r#mod),
            )
        };
        return 1i32.wrapping_neg();
    }
    unsafe { PyDict_SetItemString(dict, name, value) }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyModule_Add(
    r#mod: *mut _object,
    name: *const i8,
    value: *mut _object,
) -> i32 {
    let res: i32 = PyModule_AddObjectRef(r#mod, name, value);
    Py_XDECREF!(value);
    res
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyModule_AddObject(
    r#mod: *mut _object,
    name: *const i8,
    value: *mut _object,
) -> i32 {
    let res: i32 = PyModule_AddObjectRef(r#mod, name, value);
    if res == 0 {
        Py_DECREF!(value);
    }
    res
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyModule_AddIntConstant(m: *mut _object, name: *const i8, value: i64) -> i32 {
    PyModule_Add(m, name, unsafe { PyLong_FromLong(value) })
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyModule_AddStringConstant(
    m: *mut _object,
    name: *const i8,
    value: *const i8,
) -> i32 {
    PyModule_Add(m, name, unsafe { PyUnicode_FromString(value) })
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyModule_AddType(module: *mut _object, r#type: *mut _typeobject) -> i32 {
    let __v0: bool = _PyType_IsReady(r#type) == 0 && (unsafe { PyType_Ready(r#type) }) < 0;
    if __v0 {
        return 1i32.wrapping_neg();
    }
    let name: *const i8 = unsafe { _PyType_Name(r#type) };
    PyModule_AddObjectRef(module, name, r#type as *mut _object)
}

// Exported functions for version helper macros
#[unsafe(no_mangle)]
extern "C-unwind" fn Py_PACK_FULL_VERSION(x: i32, y: i32, z: i32, level: i32, serial: i32) -> u32 {
    _Py_PACK_FULL_VERSION!(x, y, z, level, serial) as u32
}

#[unsafe(no_mangle)]
extern "C-unwind" fn Py_PACK_VERSION(x: i32, y: i32) -> u32 {
    Py_PACK_FULL_VERSION(x, y, 0i32, 0i32, 0i32)
}
