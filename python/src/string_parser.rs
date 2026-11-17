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
//! _PyBytes_DecodeEscape()
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
//! Max indentation level
// Signed variants of the above
// Round up size "n" to be a multiple of "a".
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
/// Mask the old API with a call to the new API for code compiled under
/// Python 2.0:
macro_rules! PyErr_BadInternalCall__30 {
    () => {
        unsafe {
            _PyErr_BadInternalCall(
                c"/home/takashi/c-corpus/cpython/Parser/string_parser.c".as_ptr(),
                314i32,
            )
        };
    };
}

/// Mask the old API with a call to the new API for code compiled under
/// Python 2.0:
macro_rules! PyErr_BadInternalCall__104 {
    () => {
        unsafe {
            _PyErr_BadInternalCall(
                c"/home/takashi/c-corpus/cpython/Parser/string_parser.c".as_ptr(),
                303i32,
            )
        };
    };
}

/// Mask the old API with a call to the new API for code compiled under
/// Python 2.0:
macro_rules! PyErr_BadInternalCall__140 {
    () => {
        unsafe {
            _PyErr_BadInternalCall(
                c"/home/takashi/c-corpus/cpython/Parser/string_parser.c".as_ptr(),
                286i32,
            )
        };
    };
}

macro_rules! PyUnicode_DATA {
    ($op:expr) => {
        _PyUnicode_DATA(_PyObject_CAST!($op))
    };
}

macro_rules! PyUnicode_GET_LENGTH {
    ($op:expr) => {
        PyUnicode_GET_LENGTH(_PyObject_CAST!($op))
    };
}

macro_rules! PyUnicode_IS_ASCII {
    ($op:expr) => {
        PyUnicode_IS_ASCII(_PyObject_CAST!($op))
    };
}

macro_rules! PyUnicode_IS_COMPACT {
    ($op:expr) => {
        PyUnicode_IS_COMPACT(_PyObject_CAST!($op))
    };
}

/// PyUnicode_KIND(): Return one of the PyUnicode_*_KIND values defined above.
///
///
///
/// gh-89653: Converting this macro to a static inline function would introduce
///
/// new compiler warnings on "kind < PyUnicode_KIND(str)" (compare signed and
///
/// unsigned numbers) where kind type is an int or on
///
/// "unsigned int kind = PyUnicode_KIND(str)" (cast signed to unsigned).
macro_rules! PyUnicode_KIND {
    ($op:expr) => {
        _Py_RVALUE!(
            (unsafe {
                (*_PyASCIIObject_CAST!($op))
                    .state
                    .__slate_bits_0
                    .__get_kind()
            }) as i32
        )
    };
}

macro_rules! PyUnicode_READ {
    ($kind:expr, $data:expr, $index:expr) => {
        PyUnicode_READ(
            _Py_STATIC_CAST!(i32, $kind),
            _Py_STATIC_CAST__2!($data),
            $index,
        )
    };
}

/// Argument must be a char or an int in [-128, 127] or [0, 255].
macro_rules! Py_CHARMASK__2 {
    ($c:expr) => {
        (($c) & 255) as u8
    };
}

/// Argument must be a char or an int in [-128, 127] or [0, 255].
macro_rules! Py_CHARMASK {
    ($c:expr) => {
        ((($c) as i32) & 255) as u8
    };
}

macro_rules! Py_DECREF {
    ($op:expr) => {
        Py_DECREF(_PyObject_CAST!($op));
    };
}

macro_rules! Py_ISALPHA__2 {
    ($c:expr) => {
        (unsafe { _Py_ctype_table[(Py_CHARMASK__2!($c) as i32) as usize] }) & 3u32
    };
}

macro_rules! Py_XDECREF {
    ($op:expr) => {
        Py_XDECREF(_PyObject_CAST!($op));
    };
}

macro_rules! _PyASCIIObject_CAST {
    ($op:expr) => {
        _Py_CAST!(*mut __SlateRecord_unicodeobject_h_163, $op)
    };
}

macro_rules! _PyCompactUnicodeObject_CAST {
    ($op:expr) => {
        _Py_CAST!(*mut __SlateRecord_unicodeobject_h_173, $op)
    };
}

/// Cast argument to PyObject* type.
macro_rules! _PyObject_CAST {
    ($op:expr) => {
        _Py_CAST__2!($op)
    };
}

macro_rules! _PyUnicodeObject_CAST {
    ($op:expr) => {
        _Py_CAST!(*mut __SlateRecord_unicodeobject_h_184, $op)
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

/// Prevent using an expression as a l-value.
///
/// For example, "int x; _Py_RVALUE(x) = 1;" fails with a compiler error.
macro_rules! _Py_RVALUE {
    ($EXPR:expr) => {
        $EXPR
    };
}

macro_rules! _Py_STATIC_CAST__2 {
    ($expr:expr) => {
        $expr
    };
}

macro_rules! _Py_STATIC_CAST {
    ($type:ty, $expr:expr) => {
        ($expr) as $type
    };
}

unsafe extern "C" {
    static mut PyExc_OverflowError: *mut _object;
    static mut PyExc_SyntaxError: *mut _object;
    static mut PyExc_DeprecationWarning: *mut _object;
    static mut PyExc_SyntaxWarning: *mut _object;
    static mut _Py_ctype_table: [u32; 256];
    fn index(arg0: *const i8, arg1: i32) -> *mut i8;
    fn _Py_Dealloc(arg0: *mut _object);
    fn PyBytes_FromStringAndSize(arg0: *const i8, arg1: i64) -> *mut _object;
    fn PyBytes_Size(arg0: *mut _object) -> i64;
    fn PyBytes_AsString(arg0: *mut _object) -> *mut i8;
    fn PyUnicode_FromFormat(arg0: *const i8, ...) -> *mut _object;
    fn PyUnicode_DecodeUTF8(arg0: *const i8, arg1: i64, arg2: *const i8) -> *mut _object;
    fn PyUnicode_DecodeUTF8Stateful(
        arg0: *const i8,
        arg1: i64,
        arg2: *const i8,
        arg3: *mut i64,
    ) -> *mut _object;
    fn PyErr_SetString(arg0: *mut _object, arg1: *const i8);
    fn PyErr_Clear();
    fn PyErr_ExceptionMatches(arg0: *mut _object) -> i32;
    fn _PyErr_BadInternalCall(arg0: *const i8, arg1: i32);
    fn PyErr_WarnExplicitObject(
        arg0: *mut _object,
        arg1: *mut _object,
        arg2: *mut _object,
        arg3: i32,
        arg4: *mut _object,
        arg5: *mut _object,
    ) -> i32;
    fn _PyBytes_DecodeEscape2(
        arg0: *const i8,
        arg1: i64,
        arg2: *const i8,
        arg3: *mut i32,
        arg4: *mut *const i8,
    ) -> *mut _object;
    fn _PyUnicode_DecodeUnicodeEscapeInternal2(
        arg0: *const i8,
        arg1: i64,
        arg2: *const i8,
        arg3: *mut i64,
        arg4: *mut i32,
        arg5: *mut *const i8,
    ) -> *mut _object;
    fn _PyPegen_raise_error_known_location(
        arg0: *mut __SlateRecord_pegen_h_94,
        arg1: *mut _object,
        arg2: i64,
        arg3: i64,
        arg4: i64,
        arg5: i64,
        arg6: *const i8,
        arg7: core::ffi::VaList<'static>,
    ) -> *mut ();
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
struct __SlateRecord_unicodeobject_h_163 {
    ob_base: _object,
    length: i64,
    hash: i64,
    state: __SlateRecord_unicodeobject_h_163_2,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_unicodeobject_h_163_2 {
    __slate_bits_0: __slate_bits::__SlateBits90U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_unicodeobject_h_173 {
    _base: __SlateRecord_unicodeobject_h_163,
    utf8_length: i64,
    utf8: *mut i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_unicodeobject_h_184 {
    _base: __SlateRecord_unicodeobject_h_173,
    data: __SlateRecord_unicodeobject_h_184_2,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord_unicodeobject_h_184_2 {
    any: *mut (),
    latin1: *mut u8,
    ucs2: *mut u16,
    ucs4: *mut u32,
}

fn PyUnicode_IS_ASCII(op: *mut _object) -> u32 {
    ((unsafe {
        (*_PyASCIIObject_CAST!(op))
            .state
            .__slate_bits_0
            .__get_ascii()
    }) as i32) as u32
}

fn PyUnicode_IS_COMPACT(op: *mut _object) -> u32 {
    ((unsafe {
        (*_PyASCIIObject_CAST!(op))
            .state
            .__slate_bits_0
            .__get_compact()
    }) as i32) as u32
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _arena {}

fn _PyUnicode_COMPACT_DATA(op: *mut _object) -> *mut () {
    if PyUnicode_IS_ASCII!(op) != 0 {
        return _Py_STATIC_CAST!(
            *mut (),
            _PyASCIIObject_CAST!(op).wrapping_offset(1 as isize)
        );
    }
    _Py_STATIC_CAST!(
        *mut (),
        _PyCompactUnicodeObject_CAST!(op).wrapping_offset(1 as isize)
    )
}

fn _PyUnicode_NONCOMPACT_DATA(op: *mut _object) -> *mut () {
    let data: *mut () = unsafe { (*_PyUnicodeObject_CAST!(op)).data.any };
    data
}

fn _PyUnicode_DATA(op: *mut _object) -> *mut () {
    if PyUnicode_IS_COMPACT!(op) != 0 {
        return _PyUnicode_COMPACT_DATA(op);
    }
    _PyUnicode_NONCOMPACT_DATA(op)
}

fn PyUnicode_GET_LENGTH(op: *mut _object) -> i64 {
    unsafe { (*_PyASCIIObject_CAST!(op)).length }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pegen_h_61_2 {
    lineno: i32,
    comment: *mut i8,
}

fn PyUnicode_READ(kind: i32, data: *const (), index: i64) -> u32 {
    if kind == 1 {
        return (unsafe { *_Py_STATIC_CAST!(*const u8, data).wrapping_offset(index as isize) })
            as u32;
    }
    if kind == 2 {
        return (unsafe { *_Py_STATIC_CAST!(*const u16, data).wrapping_offset(index as isize) })
            as u32;
    }
    unsafe { *_Py_STATIC_CAST!(*const u32, data).wrapping_offset(index as isize) }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct PyMethodDef {
    ml_name: *const i8,
    ml_meth: Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
    ml_flags: i32,
    ml_doc: *const i8,
}

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
    pub struct __SlateBits90U0 {
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
}

use slate_support::sprintf;
use slate_support::strchr;
use slate_support::strcpy;
use slate_support::strlen;

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

#[repr(C)]
#[derive(Clone, Copy)]
struct _memo {
    r#type: i32,
    node: *mut (),
    mark: i32,
    next: *mut _memo,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pegen_h_46 {
    r#type: i32,
    bytes: *mut _object,
    level: i32,
    lineno: i32,
    col_offset: i32,
    end_lineno: i32,
    end_col_offset: i32,
    memo: *mut _memo,
    metadata: *mut _object,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pegen_h_51 {
    str: *const i8,
    r#type: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pegen_h_61 {
    items: *mut __SlateRecord_pegen_h_61_2,
    size: u64,
    num_items: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pegen_h_68 {
    lineno: i32,
    col_offset: i32,
    end_lineno: i32,
    end_col_offset: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pegen_h_94 {
    tok: *mut tok_state,
    tokens: *mut *mut __SlateRecord_pegen_h_46,
    mark: i32,
    fill: i32,
    size: i32,
    arena: *mut _arena,
    keywords: *mut *mut __SlateRecord_pegen_h_51,
    soft_keywords: *mut *mut i8,
    n_keyword_lists: i32,
    start_rule: i32,
    errcode: *mut i32,
    parsing_started: i32,
    normalize: *mut _object,
    starting_lineno: i32,
    starting_col_offset: i32,
    error_indicator: i32,
    flags: i32,
    feature_version: i32,
    type_ignore_comments: __SlateRecord_pegen_h_61,
    known_err_token: *mut __SlateRecord_pegen_h_46,
    level: i32,
    call_invalid_rules: i32,
    debug: i32,
    last_stmt_location: __SlateRecord_pegen_h_68,
}

unsafe extern "C-unwind" fn RAISE_ERROR_KNOWN_LOCATION(
    p: *mut __SlateRecord_pegen_h_94,
    errtype: *mut _object,
    lineno: i64,
    col_offset: i64,
    end_lineno: i64,
    end_col_offset: i64,
    errmsg: *const i8,
    mut __va_args: ...
) -> *mut () {
    let mut va: core::ffi::VaList<'static> = unsafe { core::mem::zeroed() };
    va = unsafe {
        core::mem::transmute::<core::ffi::VaList<'_>, core::ffi::VaList<'static>>(__va_args.clone())
    };
    let _col_offset: i64 = if col_offset == (5i32.wrapping_neg() as i64) {
        5i32.wrapping_neg() as i64
    } else {
        col_offset.wrapping_add(1)
    };
    let _end_col_offset: i64 = if end_col_offset == (5i32.wrapping_neg() as i64) {
        5i32.wrapping_neg() as i64
    } else {
        end_col_offset.wrapping_add(1)
    };
    unsafe {
        _PyPegen_raise_error_known_location(
            p,
            errtype,
            lineno,
            _col_offset,
            end_lineno,
            _end_col_offset,
            errmsg,
            va.clone(),
        )
    };
    core::ptr::null_mut::<()>()
}

// _PyUnicode_DecodeUnicodeEscapeInternal()
// / STRING HANDLING FUNCTIONS ////
fn warn_invalid_escape_sequence(
    p: *mut __SlateRecord_pegen_h_94,
    mut start: *const i8,
    first_invalid_escape: *const i8,
    t: *mut __SlateRecord_pegen_h_46,
) -> i32 {
    if (unsafe { (*p).call_invalid_rules }) != 0 {
        // Do not report warnings if we are in the second pass of the parser
        //
        // to avoid showing the warning twice.
        return 0;
    }
    let c: u8 = (unsafe { *first_invalid_escape }) as u8;
    if ((unsafe { (*t).r#type }) == 60
        || (unsafe { (*t).r#type }) == 61
        || (unsafe { (*t).r#type }) == 63
        || (unsafe { (*t).r#type }) == 64)
        && ((c as i32) == 123 || (c as i32) == 125)
    {
        // in this case the tokenizer has already emitted a warning,
        //
        // see Parser/tokenizer/helpers.c:warn_invalid_escape_sequence
        return 0;
    }
    let octal: i32 = (52 <= (c as i32) && (c as i32) <= 55) as i32;
    let __v0: *mut _object = if octal != 0 {
        unsafe {
            PyUnicode_FromFormat(c"\"\\%.3s\" is an invalid octal escape sequence. Such sequences will not work in the future. Did you mean \"\\\\%.3s\"? A raw string is also an option.".as_ptr(), first_invalid_escape, first_invalid_escape)
        }
    } else {
        unsafe {
            PyUnicode_FromFormat(c"\"\\%c\" is an invalid escape sequence. Such sequences will not work in the future. Did you mean \"\\\\%c\"? A raw string is also an option.".as_ptr(), c as i32, c as i32)
        }
    };
    let msg: *mut _object = __v0;
    if msg.is_null() {
        return 1i32.wrapping_neg();
    }
    let mut category: *mut _object = unsafe { core::mem::zeroed() };
    if (unsafe { (*p).feature_version }) >= 12 {
        category = unsafe { PyExc_SyntaxWarning };
    } else {
        category = unsafe { PyExc_DeprecationWarning };
    }
    // Calculate the lineno and the col_offset of the invalid escape sequence
    let end: *const i8 = first_invalid_escape;
    let mut lineno: i32 = unsafe { (*t).lineno };
    let mut col_offset: i32 = unsafe { (*t).col_offset };
    while start < end {
        if ((unsafe { *start }) as i32) == 10 {
            lineno = lineno.wrapping_add(1);
            col_offset = 0;
        } else {
            col_offset = col_offset.wrapping_add(1);
        }
        let __v5: *const i8 = start;
        start = __v5.wrapping_offset(1 as isize);
    }
    // Count the number of quotes in the token
    let mut first_quote: i8 = 0;
    if lineno == unsafe { (*t).lineno } {
        let mut quote_count: i32 = 0;
        let tok: *mut i8 = unsafe { PyBytes_AsString((*t).bytes) };
        let mut i: i32 = 0;
        while (i as i64) < unsafe { PyBytes_Size((*t).bytes) } {
            if ((unsafe { *tok.wrapping_offset(i as isize) }) as i32) == 39
                || ((unsafe { *tok.wrapping_offset(i as isize) }) as i32) == 34
            {
                if quote_count == 0 {
                    first_quote = unsafe { *tok.wrapping_offset(i as isize) };
                }
                if ((unsafe { *tok.wrapping_offset(i as isize) }) as i32) == (first_quote as i32) {
                    quote_count = quote_count.wrapping_add(1);
                }
            } else {
                break;
            }
            i = i.wrapping_add(1);
        }
        col_offset = col_offset.wrapping_add(quote_count);
    }
    if (unsafe {
        PyErr_WarnExplicitObject(
            category,
            msg,
            (*(*p).tok).filename,
            lineno,
            core::ptr::null_mut::<_object>(),
            core::ptr::null_mut::<_object>(),
        )
    }) < 0
    {
        if (unsafe { PyErr_ExceptionMatches(category) }) != 0 {
            // Replace the Syntax/DeprecationWarning exception with a SyntaxError
            // to get a more accurate error report
            unsafe { PyErr_Clear() };
            // This is needed, in order for the SyntaxError to point to the token t,
            // since _PyPegen_raise_error uses p->tokens[p->fill - 1] for the
            // error location, if p->known_err_token is not set.
            unsafe {
                (*p).known_err_token = t;
            }
            if octal != 0 {
                unsafe {
                    RAISE_ERROR_KNOWN_LOCATION(p, PyExc_SyntaxError, lineno as i64, col_offset.wrapping_sub(1) as i64, lineno as i64, col_offset.wrapping_add(1) as i64, c"\"\\%.3s\" is an invalid octal escape sequence. Did you mean \"\\\\%.3s\"? A raw string is also an option.".as_ptr(), first_invalid_escape, first_invalid_escape)
                };
            } else {
                unsafe {
                    RAISE_ERROR_KNOWN_LOCATION(p, PyExc_SyntaxError, lineno as i64, col_offset.wrapping_sub(1) as i64, lineno as i64, col_offset.wrapping_add(1) as i64, c"\"\\%c\" is an invalid escape sequence. Did you mean \"\\\\%c\"? A raw string is also an option.".as_ptr(), c as i32, c as i32)
                };
            }
        }
        Py_DECREF!(msg);
        return 1i32.wrapping_neg();
    }
    Py_DECREF!(msg);
    0
}

fn decode_utf8(sPtr: *mut *const i8, end: *const i8) -> *mut _object {
    let mut s: *const i8 = unsafe { core::mem::zeroed() };
    let __v0: *const i8 = unsafe { *sPtr };
    s = __v0;
    let t: *const i8 = __v0;
    while s < end && ((unsafe { *s }) as i32) & 128 != 0 {
        let __v1: *const i8 = s;
        s = __v1.wrapping_offset(1 as isize);
    }
    unsafe {
        *sPtr = s;
    }
    unsafe {
        PyUnicode_DecodeUTF8(
            t,
            s.offset_from(t as *const i8) as i64,
            core::ptr::null::<i8>(),
        )
    }
}

fn decode_unicode_with_escapes(
    parser: *mut __SlateRecord_pegen_h_94,
    mut s: *const i8,
    mut len: u64,
    t: *mut __SlateRecord_pegen_h_46,
) -> *mut _object {
    let mut p: *mut i8 = unsafe { core::mem::zeroed() };
    // check for integer overflow
    if len > 9223372036854775807 / 6 {
        return core::ptr::null_mut::<_object>();
    }
    // "ä" (2 bytes) may become "\U000000E4" (10 bytes), or 1:5
    // "\ä" (3 bytes) may become "\u005c\U000000E4" (16 bytes), or ~1:6
    let u: *mut _object = unsafe {
        PyBytes_FromStringAndSize(
            core::ptr::null_mut::<i8>() as *const i8,
            (len as i64).wrapping_mul(6),
        )
    };
    if u.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let __v0: *mut i8 = unsafe { PyBytes_AsString(u) };
    let buf: *mut i8 = __v0;
    p = __v0;
    if p.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let end: *const i8 = s.wrapping_offset(len as isize);
    while s < end {
        if ((unsafe { *s }) as i32) == 92 {
            let __v1: *const i8 = s;
            s = __v1.wrapping_offset(1 as isize);
            let __v3: *mut i8 = p;
            p = __v3.wrapping_offset(1 as isize);
            unsafe {
                *__v3 = *__v1;
            }
            if s >= end || ((unsafe { *s }) as i32) & 128 != 0 {
                unsafe {
                    strcpy(
                        p as *mut core::ffi::c_char,
                        c"u005c".as_ptr() as *const core::ffi::c_char,
                    ) as *mut i8
                };
                let __v5: *mut i8 = p;
                p = __v5.wrapping_offset(5 as isize);
                if s >= end {
                    break;
                }
            }
        }
        if ((unsafe { *s }) as i32) & 128 != 0 {
            let mut kind: i32 = 0;
            let mut w_len: i64 = 0;
            let w: *mut _object = decode_utf8(core::ptr::addr_of_mut!(s), end);
            if w.is_null() {
                Py_DECREF!(u);
                return core::ptr::null_mut::<_object>();
            }
            kind = PyUnicode_KIND!(w);
            let data: *const () = PyUnicode_DATA!(w) as *const ();
            w_len = PyUnicode_GET_LENGTH!(w);
            for i in 0..w_len {
                let chr: u32 = PyUnicode_READ!(kind, data, i);
                unsafe {
                    sprintf(
                        p as *mut core::ffi::c_char,
                        c"\\U%08x".as_ptr() as *const core::ffi::c_char,
                        chr,
                    ) as i32
                };
                let __v8: *mut i8 = p;
                p = __v8.wrapping_offset(10 as isize);
            }
            // Should be impossible to overflow
            Py_DECREF!(w);
        } else {
            let __v13: *const i8 = s;
            s = __v13.wrapping_offset(1 as isize);
            let __v15: *mut i8 = p;
            p = __v15.wrapping_offset(1 as isize);
            unsafe {
                *__v15 = *__v13;
            }
        }
    }
    len = ((unsafe { p.offset_from(buf as *mut i8) }) as i64) as u64;
    s = buf as *const i8;
    let mut first_invalid_escape_char: i32 = 0;
    let mut first_invalid_escape_ptr: *const i8 = unsafe { core::mem::zeroed() };
    let v: *mut _object = unsafe {
        _PyUnicode_DecodeUnicodeEscapeInternal2(
            s,
            len as i64,
            core::ptr::null::<i8>(),
            core::ptr::null_mut::<i64>(),
            core::ptr::addr_of_mut!(first_invalid_escape_char),
            core::ptr::addr_of_mut!(first_invalid_escape_ptr),
        )
    };
    // HACK: later we can simply pass the line no, since we don't preserve the tokens
    //
    // when we are decoding the string but we preserve the line numbers.
    if !v.is_null() && !first_invalid_escape_ptr.is_null() && !t.is_null() {
        if warn_invalid_escape_sequence(parser, s, first_invalid_escape_ptr, t) < 0 {
            // We have not decref u before because first_invalid_escape_ptr
            // points inside u.
            Py_XDECREF!(u);
            Py_DECREF!(v);
            return core::ptr::null_mut::<_object>();
        }
    }
    Py_XDECREF!(u);
    v
}

fn decode_bytes_with_escapes(
    p: *mut __SlateRecord_pegen_h_94,
    s: *const i8,
    len: i64,
    t: *mut __SlateRecord_pegen_h_46,
) -> *mut _object {
    let mut first_invalid_escape_char: i32 = 0;
    let mut first_invalid_escape_ptr: *const i8 = unsafe { core::mem::zeroed() };
    let result: *mut _object = unsafe {
        _PyBytes_DecodeEscape2(
            s,
            len,
            core::ptr::null::<i8>(),
            core::ptr::addr_of_mut!(first_invalid_escape_char),
            core::ptr::addr_of_mut!(first_invalid_escape_ptr),
        )
    };
    if result.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    if !first_invalid_escape_ptr.is_null() {
        if warn_invalid_escape_sequence(p, s, first_invalid_escape_ptr, t) < 0 {
            Py_DECREF!(result);
            return core::ptr::null_mut::<_object>();
        }
    }
    result
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyPegen_decode_string(
    p: *mut __SlateRecord_pegen_h_94,
    raw: i32,
    s: *const i8,
    len: u64,
    t: *mut __SlateRecord_pegen_h_46,
) -> *mut _object {
    if raw != 0 {
        return unsafe {
            PyUnicode_DecodeUTF8Stateful(
                s,
                len as i64,
                core::ptr::null::<i8>(),
                core::ptr::null_mut::<i64>(),
            )
        };
    }
    decode_unicode_with_escapes(p, s, len, t)
}

/// s must include the bracketing quote characters, and r, b &/or f prefixes
///  (if any), and embedded escape sequences (if any). (f-strings are handled by the parser)
/// _PyPegen_parse_string parses it, and returns the decoded Python string object.
#[unsafe(no_mangle)]
extern "C-unwind" fn _PyPegen_parse_string(
    p: *mut __SlateRecord_pegen_h_94,
    t: *mut __SlateRecord_pegen_h_46,
) -> *mut _object {
    let mut s: *const i8 = (unsafe { PyBytes_AsString((*t).bytes) }) as *const i8;
    if s.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let mut len: u64 = 0;
    let mut quote: i32 = Py_CHARMASK!(unsafe { *s }) as i32;
    let mut bytesmode: i32 = 0;
    let mut rawmode: i32 = 0;
    if Py_ISALPHA__2!(quote) != 0 {
        while bytesmode == 0 || rawmode == 0 {
            if quote == 98 || quote == 66 {
                let __v0: *const i8 = s;
                let __v1: *const i8 = __v0.wrapping_offset(1 as isize);
                s = __v1;
                quote = ((unsafe { *__v1 }) as u8) as i32;
                bytesmode = 1;
            } else {
                if quote == 117 || quote == 85 {
                    let __v2: *const i8 = s;
                    let __v3: *const i8 = __v2.wrapping_offset(1 as isize);
                    s = __v3;
                    quote = ((unsafe { *__v3 }) as u8) as i32;
                } else {
                    if quote == 114 || quote == 82 {
                        let __v4: *const i8 = s;
                        let __v5: *const i8 = __v4.wrapping_offset(1 as isize);
                        s = __v5;
                        quote = ((unsafe { *__v5 }) as u8) as i32;
                        rawmode = 1;
                    } else {
                        break;
                    }
                }
            }
        }
    }
    if quote != 39 && quote != 34 {
        PyErr_BadInternalCall__140!();
        return core::ptr::null_mut::<_object>();
    }
    // Skip the leading quote char.
    let __v7: *const i8 = s;
    s = __v7.wrapping_offset(1 as isize);
    len = unsafe { strlen(s as *const core::ffi::c_char) as u64 };
    // gh-120155: 's' contains at least the trailing quote,
    //
    // so the code '--len' below is safe.
    if len > 2147483647 {
        unsafe { PyErr_SetString(PyExc_OverflowError, c"string to parse is too long".as_ptr()) };
        return core::ptr::null_mut::<_object>();
    }
    let __v10: u64 = len.wrapping_sub(1);
    len = __v10;
    if ((unsafe { *s.wrapping_offset(__v10 as isize) }) as i32) != quote {
        // Last quote char must match the first.
        PyErr_BadInternalCall__104!();
        return core::ptr::null_mut::<_object>();
    }
    if len >= 4
        && ((unsafe { *s.wrapping_offset(0 as isize) }) as i32) == quote
        && ((unsafe { *s.wrapping_offset(1 as isize) }) as i32) == quote
    {
        // A triple quoted string. We've already skipped one quote at
        // the start and one at the end of the string. Now skip the
        // two at the start.
        let __v11: *const i8 = s;
        s = __v11.wrapping_offset(2 as isize);
        len = len.wrapping_sub(2);
        // And check that the last two match.
        let __v16: u64 = len.wrapping_sub(1);
        len = __v16;
        let __v17: bool = if ((unsafe { *s.wrapping_offset(__v16 as isize) }) as i32) != quote {
            true
        } else {
            let __v19: u64 = len.wrapping_sub(1);
            len = __v19;
            ((unsafe { *s.wrapping_offset(__v19 as isize) }) as i32) != quote
        };
        if __v17 {
            PyErr_BadInternalCall__30!();
            return core::ptr::null_mut::<_object>();
        }
    }
    // Avoid invoking escape decoding routines if possible.
    rawmode = (rawmode != 0
        || ((unsafe { strchr(s as *const core::ffi::c_char, 92i32 as i32) as *mut i8 })
            as *const i8)
            .is_null()) as i32;
    if bytesmode != 0 {
        // Disallow non-ASCII characters.
        let mut ch: *const i8 = unsafe { core::mem::zeroed() };
        ch = s;
        while (unsafe { *ch }) != 0 {
            if (Py_CHARMASK!(unsafe { *ch }) as i32) >= 128 {
                unsafe {
                    RAISE_ERROR_KNOWN_LOCATION(
                        p,
                        PyExc_SyntaxError,
                        (*t).lineno as i64,
                        (*t).col_offset as i64,
                        (*t).end_lineno as i64,
                        (*t).end_col_offset as i64,
                        c"bytes can only contain ASCII literal characters".as_ptr(),
                    )
                };
                return core::ptr::null_mut::<_object>();
            }
            let __v21: *const i8 = ch;
            ch = __v21.wrapping_offset(1 as isize);
        }
        if rawmode != 0 {
            return unsafe { PyBytes_FromStringAndSize(s, len as i64) };
        }
        return decode_bytes_with_escapes(p, s, len as i64, t);
    }
    _PyPegen_decode_string(p, rawmode, s, len, t)
}
