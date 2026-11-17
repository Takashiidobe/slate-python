//! Map C struct members to Python object attributes
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
//! These would be in pycore_long.h if it weren't for an include cycle.
//! Long value tag bits:
//! 0-1: Sign bits value = (1-sign), ie. negative=2, positive=0, zero=1.
//! 2: Set to 1 for the small ints
//! 3+ Unsigned digit count
//! Bit flags for _gc_prev
//!
//! Bit 0 is set when tp_finalize is called
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
macro_rules! FT_ATOMIC_STORE_CHAR_RELAXED {
    ($value:expr, $new_value:expr) => {
        unsafe {
            $value = ($new_value) as i8;
        }
    };
}

macro_rules! FT_ATOMIC_STORE_CHAR_RELAXED__2 {
    ($value:expr, $new_value:expr) => {
        unsafe {
            $value = $new_value;
        }
    };
}

macro_rules! FT_ATOMIC_STORE_DOUBLE_RELAXED {
    ($value:expr, $new_value:expr) => {
        unsafe {
            $value = $new_value;
        }
    };
}

macro_rules! FT_ATOMIC_STORE_FLOAT_RELAXED {
    ($value:expr, $new_value:expr) => {
        unsafe {
            $value = $new_value;
        }
    };
}

macro_rules! FT_ATOMIC_STORE_INT_RELAXED {
    ($value:expr, $new_value:expr) => {
        unsafe {
            $value = $new_value;
        }
    };
}

macro_rules! FT_ATOMIC_STORE_LLONG_RELAXED {
    ($value:expr, $new_value:expr) => {
        unsafe {
            $value = $new_value;
        }
    };
}

macro_rules! FT_ATOMIC_STORE_LONG_RELAXED {
    ($value:expr, $new_value:expr) => {
        unsafe {
            $value = $new_value;
        }
    };
}

macro_rules! FT_ATOMIC_STORE_PTR_RELEASE {
    ($value:expr, $new_value:expr) => {
        unsafe {
            $value = $new_value;
        }
    };
}

macro_rules! FT_ATOMIC_STORE_SHORT_RELAXED {
    ($value:expr, $new_value:expr) => {
        unsafe {
            $value = $new_value;
        }
    };
}

macro_rules! FT_ATOMIC_STORE_SSIZE_RELAXED {
    ($value:expr, $new_value:expr) => {
        unsafe {
            $value = $new_value;
        }
    };
}

macro_rules! FT_ATOMIC_STORE_UCHAR_RELAXED {
    ($value:expr, $new_value:expr) => {
        unsafe {
            $value = $new_value;
        }
    };
}

macro_rules! FT_ATOMIC_STORE_UINT_RELAXED {
    ($value:expr, $new_value:expr) => {
        unsafe {
            $value = $new_value;
        }
    };
}

macro_rules! FT_ATOMIC_STORE_ULLONG_RELAXED {
    ($value:expr, $new_value:expr) => {
        unsafe {
            $value = $new_value;
        }
    };
}

macro_rules! FT_ATOMIC_STORE_ULONG_RELAXED {
    ($value:expr, $new_value:expr) => {
        unsafe {
            $value = $new_value;
        }
    };
}

macro_rules! FT_ATOMIC_STORE_USHORT_RELAXED {
    ($value:expr, $new_value:expr) => {
        unsafe {
            $value = $new_value;
        }
    };
}

macro_rules! PyBool_Check {
    ($x:expr) => {
        Py_IS_TYPE!($x, unsafe { core::ptr::addr_of_mut!(PyBool_Type) })
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

macro_rules! WARN {
    ($msg:expr) => {{
        loop {
            if (unsafe { PyErr_WarnEx(PyExc_RuntimeWarning, ($msg) as *const i8, 1i64) }) < 0 {
                return 1i32.wrapping_neg();
            }
            if true {
                break;
            }
        }
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
    static mut PyBool_Type: _typeobject;
    static mut _Py_NoneStruct: _object;
    static mut PyExc_AttributeError: *mut _object;
    static mut PyExc_SystemError: *mut _object;
    static mut PyExc_TypeError: *mut _object;
    static mut PyExc_RuntimeWarning: *mut _object;
    static mut _Py_TrueStruct: _longobject;
    fn _Py_Dealloc(arg0: *mut _object);
    fn PyUnicode_FromStringAndSize(arg0: *const i8, arg1: i64) -> *mut _object;
    fn PyUnicode_FromString(arg0: *const i8) -> *mut _object;
    fn PyUnicode_AsUTF8AndSize(arg0: *mut _object, arg1: *mut i64) -> *const i8;
    fn PyErr_SetString(arg0: *mut _object, arg1: *const i8);
    fn PyErr_Occurred() -> *mut _object;
    fn PyErr_BadArgument() -> i32;
    fn PyErr_Format(arg0: *mut _object, arg1: *const i8, ...) -> *mut _object;
    fn PyLong_FromLong(arg0: i64) -> *mut _object;
    fn PyLong_FromUnsignedLong(arg0: u64) -> *mut _object;
    fn PyLong_FromSsize_t(arg0: i64) -> *mut _object;
    fn PyLong_AsLong(arg0: *mut _object) -> i64;
    fn PyLong_AsSsize_t(arg0: *mut _object) -> i64;
    fn PyLong_AsUnsignedLong(arg0: *mut _object) -> u64;
    fn PyLong_FromLongLong(arg0: i64) -> *mut _object;
    fn PyLong_FromUnsignedLongLong(arg0: u64) -> *mut _object;
    fn PyLong_AsLongLong(arg0: *mut _object) -> i64;
    fn PyLong_AsUnsignedLongLong(arg0: *mut _object) -> u64;
    fn PyBool_FromLong(arg0: i64) -> *mut _object;
    fn PyFloat_FromDouble(arg0: f64) -> *mut _object;
    fn PyFloat_AsDouble(arg0: *mut _object) -> f64;
    fn PyErr_WarnEx(arg0: *mut _object, arg1: *const i8, arg2: i64) -> i32;
    fn _PyNumber_Index(arg0: *mut _object) -> *mut _object;
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

// Track types initialized using _PyStaticType_InitBuiltin().
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

fn _Py_XNewRef(obj: *mut _object) -> *mut _object {
    Py_XINCREF!(obj);
    obj
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

// Types
// Deprecated, use Py_T_OBJECT_EX instead
// the ordering here is weird for binary compatibility
// 1-character string
// 8-bit signed int
// unsigned variants:
// Added by Jack: strings contained in the structure
// Added by Lillo: bools contained in the structure (assumed char)
// Py_ssize_t
// Deprecated. Value is always None.
// Flags
fn _PyLong_IsNegative(op: *const _longobject) -> bool {
    (unsafe { (*op).long_value.lv_tag }) & 3 == 2
}

// _PyNumber_Index()
// _PyLong_IsNegative()
// _Py_TryIncrefCompare(), FT_ATOMIC_*()
fn member_get_object(addr: *const i8, obj_addr: *const i8, l: *mut PyMemberDef) -> *mut _object {
    let v: *mut _object = unsafe { *(addr as *mut *mut _object) };
    if v.is_null() {
        unsafe {
            PyErr_Format(
                PyExc_AttributeError,
                c"'%T' object has no attribute '%s'".as_ptr(),
                obj_addr as *mut _object,
                (*l).name,
            )
        };
    }
    v
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyMember_GetOne(obj_addr: *const i8, l: *mut PyMemberDef) -> *mut _object {
    let mut v: *mut _object = unsafe { core::mem::zeroed() };
    if (unsafe { (*l).flags }) & 8 != 0 {
        unsafe {
            PyErr_SetString(
                PyExc_SystemError,
                c"PyMember_GetOne used with Py_RELATIVE_OFFSET".as_ptr(),
            )
        };
        return core::ptr::null_mut::<_object>();
    }
    let addr: *const i8 = obj_addr.wrapping_offset((unsafe { (*l).offset }) as isize);
    match unsafe { (*l).r#type } {
        14 => {
            v = unsafe { PyBool_FromLong(*(addr as *mut i8) as i64) };
        }
        8 => {
            v = unsafe { PyLong_FromLong(*(addr as *mut i8) as i64) };
        }
        9 => {
            v = unsafe { PyLong_FromUnsignedLong(*(addr as *mut u8) as u64) };
        }
        0 => {
            v = unsafe { PyLong_FromLong(*(addr as *mut i16) as i64) };
        }
        10 => {
            v = unsafe { PyLong_FromUnsignedLong(*(addr as *mut u16) as u64) };
        }
        1 => {
            v = unsafe { PyLong_FromLong(*(addr as *mut i32) as i64) };
        }
        11 => {
            v = unsafe { PyLong_FromUnsignedLong(*(addr as *mut u32) as u64) };
        }
        2 => {
            v = unsafe { PyLong_FromLong(*(addr as *mut i64)) };
        }
        12 => {
            v = unsafe { PyLong_FromUnsignedLong(*(addr as *mut u64)) };
        }
        19 => {
            v = unsafe { PyLong_FromSsize_t(*(addr as *mut i64)) };
        }
        3 => {
            v = unsafe { PyFloat_FromDouble(*(addr as *mut f32) as f64) };
        }
        4 => {
            v = unsafe { PyFloat_FromDouble(*(addr as *mut f64)) };
        }
        5 => {
            if unsafe { *(addr as *mut *mut i8) }.is_null() {
                v = Py_NewRef!(unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) });
            } else {
                v = unsafe { PyUnicode_FromString(*(addr as *mut *mut i8) as *const i8) };
            }
        }
        13 => {
            v = unsafe { PyUnicode_FromString(addr as *const i8) };
        }
        7 => {
            let mut char_val: i8 = unsafe { *addr };
            v = unsafe {
                PyUnicode_FromStringAndSize(core::ptr::addr_of_mut!(char_val) as *const i8, 1i64)
            };
        }
        6 => {
            v = unsafe { *(addr as *mut *mut _object) };
            if !v.is_null() {
                Py_INCREF!(v);
            }
            if v.is_null() {
                v = unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) };
            }
        }
        16 => {
            v = member_get_object(addr, obj_addr, l);
            Py_XINCREF!(v);
        }
        17 => {
            v = unsafe { PyLong_FromLongLong(*(addr as *mut i64)) };
        }
        18 => {
            v = unsafe { PyLong_FromUnsignedLongLong(*(addr as *mut u64)) };
        }
        20 => {
            v = Py_NewRef!(unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }); // doesn't require free-threading code path
        }
        _ => {
            unsafe { PyErr_SetString(PyExc_SystemError, c"bad memberdescr type".as_ptr()) };
            v = core::ptr::null_mut::<_object>();
        }
    }
    v
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyMember_SetOne(
    mut addr: *mut i8,
    l: *mut PyMemberDef,
    mut v: *mut _object,
) -> i32 {
    if (unsafe { (*l).flags }) & 8 != 0 {
        unsafe {
            PyErr_SetString(
                PyExc_SystemError,
                c"PyMember_SetOne used with Py_RELATIVE_OFFSET".as_ptr(),
            )
        };
        return 1i32.wrapping_neg();
    }
    let __v0: *mut i8 = addr;
    addr = __v0.wrapping_offset((unsafe { (*l).offset }) as isize);
    if (unsafe { (*l).flags }) & 1 != 0 {
        unsafe { PyErr_SetString(PyExc_AttributeError, c"readonly attribute".as_ptr()) };
        return 1i32.wrapping_neg();
    }
    if v.is_null() && (unsafe { (*l).r#type }) != 16 && (unsafe { (*l).r#type }) != 6 {
        unsafe {
            PyErr_SetString(
                PyExc_TypeError,
                c"can't delete numeric/char attribute".as_ptr(),
            )
        };
        return 1i32.wrapping_neg();
    }
    match unsafe { (*l).r#type } {
        14 => {
            if PyBool_Check!(v) == 0 {
                unsafe {
                    PyErr_SetString(
                        PyExc_TypeError,
                        c"attribute value type must be bool".as_ptr(),
                    )
                };
                return 1i32.wrapping_neg();
            }
            if v == _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_TrueStruct) }) {
                FT_ATOMIC_STORE_CHAR_RELAXED!(*addr, 1i32);
            } else {
                FT_ATOMIC_STORE_CHAR_RELAXED!(*addr, 0i32);
            }
        }
        8 => {
            let long_val: i64 = unsafe { PyLong_AsLong(v) };
            let __v3: bool =
                long_val == (1i32.wrapping_neg() as i64) && !unsafe { PyErr_Occurred() }.is_null();
            if __v3 {
                return 1i32.wrapping_neg();
            }
            FT_ATOMIC_STORE_CHAR_RELAXED__2!(*addr, long_val as i8);
            // XXX: For compatibility, only warn about truncations
            // for now.
            if long_val > 127 || long_val < (127i32.wrapping_neg().wrapping_sub(1) as i64) {
                WARN!(c"Truncation of value to char".as_ptr() as *mut i8);
            }
        }
        9 => {
            let long_val: i64 = unsafe { PyLong_AsLong(v) };
            let __v5: bool =
                long_val == (1i32.wrapping_neg() as i64) && !unsafe { PyErr_Occurred() }.is_null();
            if __v5 {
                return 1i32.wrapping_neg();
            }
            FT_ATOMIC_STORE_UCHAR_RELAXED!(*(addr as *mut u8), long_val as u8);
            if long_val > 255i64 || long_val < 0 {
                WARN!(c"Truncation of value to unsigned char".as_ptr() as *mut i8);
            }
        }
        0 => {
            let long_val: i64 = unsafe { PyLong_AsLong(v) };
            let __v7: bool =
                long_val == (1i32.wrapping_neg() as i64) && !unsafe { PyErr_Occurred() }.is_null();
            if __v7 {
                return 1i32.wrapping_neg();
            }
            FT_ATOMIC_STORE_SHORT_RELAXED!(*(addr as *mut i16), long_val as i16);
            if long_val > 32767 || long_val < (32767i32.wrapping_neg().wrapping_sub(1) as i64) {
                WARN!(c"Truncation of value to short".as_ptr() as *mut i8);
            }
        }
        10 => {
            let long_val: i64 = unsafe { PyLong_AsLong(v) };
            let __v9: bool =
                long_val == (1i32.wrapping_neg() as i64) && !unsafe { PyErr_Occurred() }.is_null();
            if __v9 {
                return 1i32.wrapping_neg();
            }
            FT_ATOMIC_STORE_USHORT_RELAXED!(*(addr as *mut u16), long_val as u16);
            if long_val > 65535i64 || long_val < 0 {
                WARN!(c"Truncation of value to unsigned short".as_ptr() as *mut i8);
            }
        }
        1 => {
            let long_val: i64 = unsafe { PyLong_AsLong(v) };
            let __v11: bool =
                long_val == (1i32.wrapping_neg() as i64) && !unsafe { PyErr_Occurred() }.is_null();
            if __v11 {
                return 1i32.wrapping_neg();
            }
            FT_ATOMIC_STORE_INT_RELAXED!(*(addr as *mut i32), long_val as i32);
            if long_val > 2147483647
                || long_val < (2147483647i32.wrapping_neg().wrapping_sub(1) as i64)
            {
                WARN!(c"Truncation of value to int".as_ptr() as *mut i8);
            }
        }
        11 => {
            // XXX: For compatibility, accept negative int values
            // as well.
            v = unsafe { _PyNumber_Index(v) };
            if v.is_null() {
                return 1i32.wrapping_neg();
            }
            if _PyLong_IsNegative(v as *const _longobject) {
                let long_val: i64 = unsafe { PyLong_AsLong(v) };
                Py_DECREF!(v);
                let __v13: bool = long_val == (1i32.wrapping_neg() as i64)
                    && !unsafe { PyErr_Occurred() }.is_null();
                if __v13 {
                    return 1i32.wrapping_neg();
                }
                FT_ATOMIC_STORE_UINT_RELAXED!(*(addr as *mut u32), (long_val as u64) as u32);
                WARN!(c"Writing negative value into unsigned field".as_ptr() as *mut i8);
            } else {
                let ulong_val: u64 = unsafe { PyLong_AsUnsignedLong(v) };
                Py_DECREF!(v);
                let __v15: bool = ulong_val == (1i32.wrapping_neg() as u64)
                    && !unsafe { PyErr_Occurred() }.is_null();
                if __v15 {
                    return 1i32.wrapping_neg();
                }
                FT_ATOMIC_STORE_UINT_RELAXED!(*(addr as *mut u32), ulong_val as u32);
                if ulong_val > 4294967295u64 {
                    WARN!(c"Truncation of value to unsigned int".as_ptr() as *mut i8);
                }
            }
        }
        2 => {
            let long_val: i64 = unsafe { PyLong_AsLong(v) };
            let __v17: bool =
                long_val == (1i32.wrapping_neg() as i64) && !unsafe { PyErr_Occurred() }.is_null();
            if __v17 {
                return 1i32.wrapping_neg();
            }
            FT_ATOMIC_STORE_LONG_RELAXED!(*(addr as *mut i64), long_val);
        }
        12 => {
            // XXX: For compatibility, accept negative int values
            // as well.
            v = unsafe { _PyNumber_Index(v) };
            if v.is_null() {
                return 1i32.wrapping_neg();
            }
            if _PyLong_IsNegative(v as *const _longobject) {
                let long_val: i64 = unsafe { PyLong_AsLong(v) };
                Py_DECREF!(v);
                let __v18: bool = long_val == (1i32.wrapping_neg() as i64)
                    && !unsafe { PyErr_Occurred() }.is_null();
                if __v18 {
                    return 1i32.wrapping_neg();
                }
                FT_ATOMIC_STORE_ULONG_RELAXED!(*(addr as *mut u64), long_val as u64);
                WARN!(c"Writing negative value into unsigned field".as_ptr() as *mut i8);
            } else {
                let ulong_val: u64 = unsafe { PyLong_AsUnsignedLong(v) };
                Py_DECREF!(v);
                let __v20: bool = ulong_val == (1i32.wrapping_neg() as u64)
                    && !unsafe { PyErr_Occurred() }.is_null();
                if __v20 {
                    return 1i32.wrapping_neg();
                }
                FT_ATOMIC_STORE_ULONG_RELAXED!(*(addr as *mut u64), ulong_val);
            }
        }
        19 => {
            let ssize_val: i64 = unsafe { PyLong_AsSsize_t(v) };
            let __v21: bool =
                ssize_val == (1i32.wrapping_neg() as i64) && !unsafe { PyErr_Occurred() }.is_null();
            if __v21 {
                return 1i32.wrapping_neg();
            }
            FT_ATOMIC_STORE_SSIZE_RELAXED!(*(addr as *mut i64), ssize_val);
        }
        3 => {
            let double_val: f64 = unsafe { PyFloat_AsDouble(v) };
            let __v22: bool = double_val == (1i32.wrapping_neg() as f64)
                && !unsafe { PyErr_Occurred() }.is_null();
            if __v22 {
                return 1i32.wrapping_neg();
            }
            FT_ATOMIC_STORE_FLOAT_RELAXED!(*(addr as *mut f32), double_val as f32);
        }
        4 => {
            let double_val: f64 = unsafe { PyFloat_AsDouble(v) };
            let __v23: bool = double_val == (1i32.wrapping_neg() as f64)
                && !unsafe { PyErr_Occurred() }.is_null();
            if __v23 {
                return 1i32.wrapping_neg();
            }
            FT_ATOMIC_STORE_DOUBLE_RELAXED!(*(addr as *mut f64), double_val);
        }
        6 | 16 => {
            let oldv: *mut _object = unsafe { *(addr as *mut *mut _object) };
            FT_ATOMIC_STORE_PTR_RELEASE!(*(addr as *mut *mut _object), Py_XNewRef!(v));
            if v.is_null() && oldv.is_null() && (unsafe { (*l).r#type }) == 16 {
                // Raise an exception when attempting to delete an already deleted
                //
                // attribute.
                //
                // Differently from Py_T_OBJECT_EX, _Py_T_OBJECT does not raise an
                //
                // exception here (PyMember_GetOne will return Py_None instead of
                //
                // NULL).
                unsafe { PyErr_SetString(PyExc_AttributeError, (*l).name) };
                return 1i32.wrapping_neg();
            }
            Py_XDECREF!(oldv);
        }
        7 => {
            let mut len: i64 = 0;
            let string: *const i8 =
                unsafe { PyUnicode_AsUTF8AndSize(v, core::ptr::addr_of_mut!(len)) };
            if string.is_null() || len != 1 {
                unsafe { PyErr_BadArgument() };
                return 1i32.wrapping_neg();
            }
            FT_ATOMIC_STORE_CHAR_RELAXED__2!(*addr, unsafe { *string.wrapping_offset(0 as isize) });
        }
        5 | 13 => {
            unsafe { PyErr_SetString(PyExc_TypeError, c"readonly attribute".as_ptr()) };
            return 1i32.wrapping_neg();
        }
        17 => {
            let value: i64 = unsafe { PyLong_AsLongLong(v) };
            let __v24: bool =
                value == (1i32.wrapping_neg() as i64) && !unsafe { PyErr_Occurred() }.is_null();
            if __v24 {
                return 1i32.wrapping_neg();
            }
            FT_ATOMIC_STORE_LLONG_RELAXED!(*(addr as *mut i64), value);
        }
        18 => {
            v = unsafe { _PyNumber_Index(v) };
            if v.is_null() {
                return 1i32.wrapping_neg();
            }
            if _PyLong_IsNegative(v as *const _longobject) {
                let long_val: i64 = unsafe { PyLong_AsLong(v) };
                Py_DECREF!(v);
                let __v25: bool = long_val == (1i32.wrapping_neg() as i64)
                    && !unsafe { PyErr_Occurred() }.is_null();
                if __v25 {
                    return 1i32.wrapping_neg();
                }
                FT_ATOMIC_STORE_ULLONG_RELAXED!(*(addr as *mut u64), long_val as u64);
                WARN!(c"Writing negative value into unsigned field".as_ptr() as *mut i8);
            } else {
                let ulonglong_val: u64 = unsafe { PyLong_AsUnsignedLongLong(v) };
                Py_DECREF!(v);
                let __v27: bool = ulonglong_val == (1i32.wrapping_neg() as u64)
                    && !unsafe { PyErr_Occurred() }.is_null();
                if __v27 {
                    return 1i32.wrapping_neg();
                }
                FT_ATOMIC_STORE_ULLONG_RELAXED!(*(addr as *mut u64), ulonglong_val);
            }
        }
        _ => {
            unsafe {
                PyErr_Format(
                    PyExc_SystemError,
                    c"bad memberdescr type for %s".as_ptr(),
                    (*l).name,
                )
            };
            return 1i32.wrapping_neg();
        }
    }
    0
}
