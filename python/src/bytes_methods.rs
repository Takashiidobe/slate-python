//! The normal alignment of 'size_t', in bytes.
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
//! Cast argument to PyListObject* type.
//! A code object, the __code__ attribute
//! To avoid repeating ourselves in deepfreeze.py, all PyCodeObject members are
//!
//! defined in this macro:
//! Remote debugger support
//! PY_TIMEOUT_T is the integral type used to specify timeouts when waiting
//!
//! on a lock (see PyThread_acquire_lock_timed() below).
//! _PyIndex_Check()
//! this is needed because some docs are shared from the .o, not static
// The size of 'size_t', as computed by sizeof.
// The size of 'void *', as computed by sizeof.
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
/// helper macro to fixup start/end slice values
macro_rules! ADJUST_INDICES {
    ($start:expr, $end:expr, $len:expr) => {
        loop {
            if ($end) > $len {
                $end = $len;
            } else {
                if ($end) < 0 {
                    let __v: i64 = $end;
                    let __v2: i64 = __v.wrapping_add($len);
                    $end = __v2;
                    if ($end) < 0 {
                        $end = 0;
                    }
                }
            }
            if ($start) < 0 {
                let __v3: i64 = $start;
                let __v4: i64 = __v3.wrapping_add($len);
                $start = __v4;
                if ($start) < 0 {
                    $start = 0;
                }
            }
            if true {
                break;
            }
        }
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

macro_rules! PyTuple_Check {
    ($op:expr) => {
        PyType_FastSubclass!(Py_TYPE!($op), 67108864u64)
    };
}

macro_rules! PyTuple_GET_ITEM {
    ($op:expr, $index:expr) => {
        unsafe {
            *((*_PyTuple_CAST!($op)).ob_item.as_mut_ptr() as *mut *mut _object)
                .wrapping_offset(($index) as isize)
        }
    };
}

macro_rules! PyTuple_GET_SIZE {
    ($op:expr) => {
        PyTuple_GET_SIZE(_PyObject_CAST!($op))
    };
}

macro_rules! PyType_FastSubclass {
    ($type:expr, $flag:expr) => {
        PyType_HasFeature($type, $flag)
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

macro_rules! Py_ISALNUM {
    ($c:expr) => {
        (unsafe { _Py_ctype_table[(Py_CHARMASK!($c) as i32) as usize] }) & 7u32
    };
}

macro_rules! Py_ISALPHA {
    ($c:expr) => {
        (unsafe { _Py_ctype_table[(Py_CHARMASK!($c) as i32) as usize] }) & 3u32
    };
}

macro_rules! Py_ISDIGIT__3 {
    ($c:expr) => {
        (unsafe { _Py_ctype_table[(Py_CHARMASK!($c) as i32) as usize] }) & 4
    };
}

/// Unlike their C counterparts, the following macros are not meant to
/// handle an int with any of the values [EOF, 0-UCHAR_MAX]. The argument
/// must be a signed/unsigned char.
macro_rules! Py_ISLOWER__2 {
    ($c:expr) => {
        (unsafe { _Py_ctype_table[(Py_CHARMASK__2!($c) as i32) as usize] }) & 1
    };
}

/// Unlike their C counterparts, the following macros are not meant to
/// handle an int with any of the values [EOF, 0-UCHAR_MAX]. The argument
/// must be a signed/unsigned char.
macro_rules! Py_ISLOWER {
    ($c:expr) => {
        (unsafe { _Py_ctype_table[(Py_CHARMASK!($c) as i32) as usize] }) & 1
    };
}

macro_rules! Py_ISSPACE {
    ($c:expr) => {
        (unsafe { _Py_ctype_table[(Py_CHARMASK!($c) as i32) as usize] }) & 8
    };
}

macro_rules! Py_ISUPPER__2 {
    ($c:expr) => {
        (unsafe { _Py_ctype_table[(Py_CHARMASK__2!($c) as i32) as usize] }) & 2
    };
}

macro_rules! Py_ISUPPER {
    ($c:expr) => {
        (unsafe { _Py_ctype_table[(Py_CHARMASK!($c) as i32) as usize] }) & 2
    };
}

/// Maximum value between x and y
macro_rules! Py_MAX {
    ($x:expr, $y:expr) => {
        if ($x) > $y { $x } else { $y }
    };
}

/// Minimum value between x and y
macro_rules! Py_MIN {
    ($x:expr, $y:expr) => {
        if ($x) > $y { $y } else { $x }
    };
}

/// Minimum value between x and y
macro_rules! Py_MIN__2 {
    ($x:expr, $y:expr) => {
        if ($x) > (($y) as i64) {
            ($y) as i64
        } else {
            $x
        }
    };
}

macro_rules! Py_SAFE_DOWNCAST {
    ($VALUE:expr, $NARROW:ty) => {
        _Py_STATIC_CAST!($NARROW, $VALUE)
    };
}

macro_rules! Py_SIZE {
    ($ob:expr) => {
        Py_SIZE(_PyObject_CAST__2!($ob))
    };
}

macro_rules! Py_TOLOWER {
    ($c:expr) => {
        unsafe { _Py_ctype_tolower[(Py_CHARMASK!($c) as i32) as usize] }
    };
}

macro_rules! Py_TOLOWER__2 {
    ($c:expr) => {
        unsafe { _Py_ctype_tolower[(Py_CHARMASK__2!($c) as i32) as usize] }
    };
}

macro_rules! Py_TOUPPER__2 {
    ($c:expr) => {
        unsafe { _Py_ctype_toupper[(Py_CHARMASK__2!($c) as i32) as usize] }
    };
}

macro_rules! Py_TOUPPER__3 {
    ($c:expr) => {
        unsafe { _Py_ctype_toupper[(Py_CHARMASK!($c) as i32) as usize] }
    };
}

macro_rules! Py_TYPE {
    ($ob:expr) => {
        _Py_TYPE(_PyObject_CAST!($ob))
    };
}

macro_rules! STRINGLIB_BLOOM_ADD {
    ($mask:expr, $ch:expr) => {
        let __v: u64 = $mask;
        let __v2: u64 = __v | 1u64 << ((($ch) as i32) & 63i32);
        $mask = __v2;
    };
}

macro_rules! STRINGLIB_BLOOM {
    ($mask:expr, $ch:expr) => {
        ($mask) & 1u64 << ((($ch) as i32) & 63i32)
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

/// Cast argument to PyTupleObject* type.
macro_rules! _PyTuple_CAST {
    ($op:expr) => {
        _Py_CAST!(*mut __SlateRecord_tupleobject_h_13, $op)
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

/// Check if pointer "p" is aligned to "a"-bytes boundary.
macro_rules! _Py_IS_ALIGNED {
    ($p:expr, $a:expr) => {
        (($p) as u64) & ($a.wrapping_sub(1) as u64) == 0
    };
}

macro_rules! _Py_STATIC_CAST {
    ($type:ty, $expr:expr) => {
        ($expr) as $type
    };
}

unsafe extern "C" {
    static mut PyExc_TypeError: *mut _object;
    static mut PyExc_ValueError: *mut _object;
    static mut _Py_FalseStruct: _longobject;
    static mut _Py_TrueStruct: _longobject;
    static mut _Py_ctype_table: [u32; 256];
    static mut _Py_ctype_tolower: [u8; 256];
    static mut _Py_ctype_toupper: [u8; 256];
    fn memrchr(arg0: *const (), arg1: i32, arg2: u64) -> *mut ();
    fn index(arg0: *const i8, arg1: i32) -> *mut i8;
    fn PyObject_CheckBuffer(arg0: *mut _object) -> i32;
    fn PyObject_GetBuffer(
        arg0: *mut _object,
        arg1: *mut __SlateRecord_pybuffer_h_33,
        arg2: i32,
    ) -> i32;
    fn PyBuffer_Release(arg0: *mut __SlateRecord_pybuffer_h_33);
    fn PyBytes_FromStringAndSize(arg0: *const i8, arg1: i64) -> *mut _object;
    fn PyErr_SetString(arg0: *mut _object, arg1: *const i8);
    fn PyErr_Occurred() -> *mut _object;
    fn PyErr_Clear();
    fn PyErr_ExceptionMatches(arg0: *mut _object) -> i32;
    fn PyErr_Format(arg0: *mut _object, arg1: *const i8, ...) -> *mut _object;
    fn PyLong_FromSsize_t(arg0: i64) -> *mut _object;
    fn PyBool_FromLong(arg0: i64) -> *mut _object;
    fn PyNumber_AsSsize_t(arg0: *mut _object, arg1: *mut _object) -> i64;
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

// Flags for getting buffers. Keep these in sync with inspect.BufferFlags.
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

fn PyType_HasFeature(r#type: *mut _typeobject, feature: u64) -> i32 {
    let flags: u64 = unsafe { (*r#type).tp_flags };
    (flags & feature != 0) as i32
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

#[repr(C, align(16))]
struct __SlateAlign16<T>(T);

unsafe impl<T> Sync for __SlateAlign16<T> {}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_tupleobject_h_13 {
    ob_base: __SlateRecord_object_h_172,
    ob_hash: i64,
    ob_item: [*mut _object; 1],
}

fn PyTuple_GET_SIZE(op: *mut _object) -> i64 {
    let tuple: *mut __SlateRecord_tupleobject_h_13 = _PyTuple_CAST!(op);
    Py_SIZE!(tuple)
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

#[unsafe(no_mangle)]
static _Py_isspace__doc__: __SlateAlign16<[i8; 130]> = __SlateAlign16([
    66i8, 46i8, 105i8, 115i8, 115i8, 112i8, 97i8, 99i8, 101i8, 40i8, 41i8, 32i8, 45i8, 62i8, 32i8,
    98i8, 111i8, 111i8, 108i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 84i8,
    114i8, 117i8, 101i8, 32i8, 105i8, 102i8, 32i8, 97i8, 108i8, 108i8, 32i8, 99i8, 104i8, 97i8,
    114i8, 97i8, 99i8, 116i8, 101i8, 114i8, 115i8, 32i8, 105i8, 110i8, 32i8, 66i8, 32i8, 97i8,
    114i8, 101i8, 32i8, 119i8, 104i8, 105i8, 116i8, 101i8, 115i8, 112i8, 97i8, 99i8, 101i8, 10i8,
    97i8, 110i8, 100i8, 32i8, 116i8, 104i8, 101i8, 114i8, 101i8, 32i8, 105i8, 115i8, 32i8, 97i8,
    116i8, 32i8, 108i8, 101i8, 97i8, 115i8, 116i8, 32i8, 111i8, 110i8, 101i8, 32i8, 99i8, 104i8,
    97i8, 114i8, 97i8, 99i8, 116i8, 101i8, 114i8, 32i8, 105i8, 110i8, 32i8, 66i8, 44i8, 32i8, 70i8,
    97i8, 108i8, 115i8, 101i8, 32i8, 111i8, 116i8, 104i8, 101i8, 114i8, 119i8, 105i8, 115i8, 101i8,
    46i8, 0i8,
]);

#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_bytes_isspace(cptr: *const i8, len: i64) -> *mut _object {
    let mut p: *const u8 = cptr as *const u8;
    // Shortcut for single character strings
    if len == 1 && Py_ISSPACE!(unsafe { *p }) != 0 {
        return _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_TrueStruct) });
    }
    // Special case for empty strings
    if len == 0 {
        return _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_FalseStruct) });
    }
    let e: *const u8 = p.wrapping_offset(len as isize);
    while p < e {
        if Py_ISSPACE!(unsafe { *p }) == 0 {
            return _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_FalseStruct) });
        }
        let __v1: *const u8 = p;
        p = __v1.wrapping_offset(1 as isize);
    }
    _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_TrueStruct) })
}

#[unsafe(no_mangle)]
static _Py_isalpha__doc__: __SlateAlign16<[i8; 130]> = __SlateAlign16([
    66i8, 46i8, 105i8, 115i8, 97i8, 108i8, 112i8, 104i8, 97i8, 40i8, 41i8, 32i8, 45i8, 62i8, 32i8,
    98i8, 111i8, 111i8, 108i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 84i8,
    114i8, 117i8, 101i8, 32i8, 105i8, 102i8, 32i8, 97i8, 108i8, 108i8, 32i8, 99i8, 104i8, 97i8,
    114i8, 97i8, 99i8, 116i8, 101i8, 114i8, 115i8, 32i8, 105i8, 110i8, 32i8, 66i8, 32i8, 97i8,
    114i8, 101i8, 32i8, 97i8, 108i8, 112i8, 104i8, 97i8, 98i8, 101i8, 116i8, 105i8, 99i8, 10i8,
    97i8, 110i8, 100i8, 32i8, 116i8, 104i8, 101i8, 114i8, 101i8, 32i8, 105i8, 115i8, 32i8, 97i8,
    116i8, 32i8, 108i8, 101i8, 97i8, 115i8, 116i8, 32i8, 111i8, 110i8, 101i8, 32i8, 99i8, 104i8,
    97i8, 114i8, 97i8, 99i8, 116i8, 101i8, 114i8, 32i8, 105i8, 110i8, 32i8, 66i8, 44i8, 32i8, 70i8,
    97i8, 108i8, 115i8, 101i8, 32i8, 111i8, 116i8, 104i8, 101i8, 114i8, 119i8, 105i8, 115i8, 101i8,
    46i8, 0i8,
]);

#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_bytes_isalpha(cptr: *const i8, len: i64) -> *mut _object {
    let mut p: *const u8 = cptr as *const u8;
    // Shortcut for single character strings
    if len == 1 && Py_ISALPHA!(unsafe { *p }) != 0 {
        return _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_TrueStruct) });
    }
    // Special case for empty strings
    if len == 0 {
        return _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_FalseStruct) });
    }
    let e: *const u8 = p.wrapping_offset(len as isize);
    while p < e {
        if Py_ISALPHA!(unsafe { *p }) == 0 {
            return _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_FalseStruct) });
        }
        let __v1: *const u8 = p;
        p = __v1.wrapping_offset(1 as isize);
    }
    _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_TrueStruct) })
}

#[unsafe(no_mangle)]
static _Py_isalnum__doc__: __SlateAlign16<[i8; 132]> = __SlateAlign16([
    66i8, 46i8, 105i8, 115i8, 97i8, 108i8, 110i8, 117i8, 109i8, 40i8, 41i8, 32i8, 45i8, 62i8, 32i8,
    98i8, 111i8, 111i8, 108i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 84i8,
    114i8, 117i8, 101i8, 32i8, 105i8, 102i8, 32i8, 97i8, 108i8, 108i8, 32i8, 99i8, 104i8, 97i8,
    114i8, 97i8, 99i8, 116i8, 101i8, 114i8, 115i8, 32i8, 105i8, 110i8, 32i8, 66i8, 32i8, 97i8,
    114i8, 101i8, 32i8, 97i8, 108i8, 112i8, 104i8, 97i8, 110i8, 117i8, 109i8, 101i8, 114i8, 105i8,
    99i8, 10i8, 97i8, 110i8, 100i8, 32i8, 116i8, 104i8, 101i8, 114i8, 101i8, 32i8, 105i8, 115i8,
    32i8, 97i8, 116i8, 32i8, 108i8, 101i8, 97i8, 115i8, 116i8, 32i8, 111i8, 110i8, 101i8, 32i8,
    99i8, 104i8, 97i8, 114i8, 97i8, 99i8, 116i8, 101i8, 114i8, 32i8, 105i8, 110i8, 32i8, 66i8,
    44i8, 32i8, 70i8, 97i8, 108i8, 115i8, 101i8, 32i8, 111i8, 116i8, 104i8, 101i8, 114i8, 119i8,
    105i8, 115i8, 101i8, 46i8, 0i8,
]);

#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_bytes_isalnum(cptr: *const i8, len: i64) -> *mut _object {
    let mut p: *const u8 = cptr as *const u8;
    // Shortcut for single character strings
    if len == 1 && Py_ISALNUM!(unsafe { *p }) != 0 {
        return _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_TrueStruct) });
    }
    // Special case for empty strings
    if len == 0 {
        return _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_FalseStruct) });
    }
    let e: *const u8 = p.wrapping_offset(len as isize);
    while p < e {
        if Py_ISALNUM!(unsafe { *p }) == 0 {
            return _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_FalseStruct) });
        }
        let __v1: *const u8 = p;
        p = __v1.wrapping_offset(1 as isize);
    }
    _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_TrueStruct) })
}

#[unsafe(no_mangle)]
static _Py_isdigit__doc__: __SlateAlign16<[i8; 126]> = __SlateAlign16([
    66i8, 46i8, 105i8, 115i8, 100i8, 105i8, 103i8, 105i8, 116i8, 40i8, 41i8, 32i8, 45i8, 62i8,
    32i8, 98i8, 111i8, 111i8, 108i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8,
    84i8, 114i8, 117i8, 101i8, 32i8, 105i8, 102i8, 32i8, 97i8, 108i8, 108i8, 32i8, 99i8, 104i8,
    97i8, 114i8, 97i8, 99i8, 116i8, 101i8, 114i8, 115i8, 32i8, 105i8, 110i8, 32i8, 66i8, 32i8,
    97i8, 114i8, 101i8, 32i8, 100i8, 105i8, 103i8, 105i8, 116i8, 115i8, 10i8, 97i8, 110i8, 100i8,
    32i8, 116i8, 104i8, 101i8, 114i8, 101i8, 32i8, 105i8, 115i8, 32i8, 97i8, 116i8, 32i8, 108i8,
    101i8, 97i8, 115i8, 116i8, 32i8, 111i8, 110i8, 101i8, 32i8, 99i8, 104i8, 97i8, 114i8, 97i8,
    99i8, 116i8, 101i8, 114i8, 32i8, 105i8, 110i8, 32i8, 66i8, 44i8, 32i8, 70i8, 97i8, 108i8,
    115i8, 101i8, 32i8, 111i8, 116i8, 104i8, 101i8, 114i8, 119i8, 105i8, 115i8, 101i8, 46i8, 0i8,
]);

#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_bytes_isdigit(cptr: *const i8, len: i64) -> *mut _object {
    let mut p: *const u8 = cptr as *const u8;
    // Shortcut for single character strings
    if len == 1 && Py_ISDIGIT__3!(unsafe { *p }) != 0 {
        return _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_TrueStruct) });
    }
    // Special case for empty strings
    if len == 0 {
        return _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_FalseStruct) });
    }
    let e: *const u8 = p.wrapping_offset(len as isize);
    while p < e {
        if Py_ISDIGIT__3!(unsafe { *p }) == 0 {
            return _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_FalseStruct) });
        }
        let __v1: *const u8 = p;
        p = __v1.wrapping_offset(1 as isize);
    }
    _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_TrueStruct) })
}

use slate_support::memchr;
use slate_support::memcmp;

#[unsafe(no_mangle)]
static _Py_islower__doc__: __SlateAlign16<[i8; 141]> = __SlateAlign16([
    66i8, 46i8, 105i8, 115i8, 108i8, 111i8, 119i8, 101i8, 114i8, 40i8, 41i8, 32i8, 45i8, 62i8,
    32i8, 98i8, 111i8, 111i8, 108i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8,
    84i8, 114i8, 117i8, 101i8, 32i8, 105i8, 102i8, 32i8, 97i8, 108i8, 108i8, 32i8, 99i8, 97i8,
    115i8, 101i8, 100i8, 32i8, 99i8, 104i8, 97i8, 114i8, 97i8, 99i8, 116i8, 101i8, 114i8, 115i8,
    32i8, 105i8, 110i8, 32i8, 66i8, 32i8, 97i8, 114i8, 101i8, 32i8, 108i8, 111i8, 119i8, 101i8,
    114i8, 99i8, 97i8, 115i8, 101i8, 32i8, 97i8, 110i8, 100i8, 32i8, 116i8, 104i8, 101i8, 114i8,
    101i8, 32i8, 105i8, 115i8, 10i8, 97i8, 116i8, 32i8, 108i8, 101i8, 97i8, 115i8, 116i8, 32i8,
    111i8, 110i8, 101i8, 32i8, 99i8, 97i8, 115i8, 101i8, 100i8, 32i8, 99i8, 104i8, 97i8, 114i8,
    97i8, 99i8, 116i8, 101i8, 114i8, 32i8, 105i8, 110i8, 32i8, 66i8, 44i8, 32i8, 70i8, 97i8, 108i8,
    115i8, 101i8, 32i8, 111i8, 116i8, 104i8, 101i8, 114i8, 119i8, 105i8, 115i8, 101i8, 46i8, 0i8,
]);

#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_bytes_islower(cptr: *const i8, len: i64) -> *mut _object {
    let mut p: *const u8 = cptr as *const u8;
    let mut cased: i32 = 0;
    // Shortcut for single character strings
    if len == 1 {
        return unsafe { PyBool_FromLong(Py_ISLOWER!(unsafe { *p }) as i64) };
    }
    // Special case for empty strings
    if len == 0 {
        return _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_FalseStruct) });
    }
    let e: *const u8 = p.wrapping_offset(len as isize);
    while p < e {
        if Py_ISUPPER!(unsafe { *p }) != 0 {
            return _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_FalseStruct) });
        } else {
            if cased == 0 && Py_ISLOWER!(unsafe { *p }) != 0 {
                cased = 1;
            }
        }
        let __v1: *const u8 = p;
        p = __v1.wrapping_offset(1 as isize);
    }
    unsafe { PyBool_FromLong(cased as i64) }
}

#[unsafe(no_mangle)]
static _Py_isupper__doc__: __SlateAlign16<[i8; 141]> = __SlateAlign16([
    66i8, 46i8, 105i8, 115i8, 117i8, 112i8, 112i8, 101i8, 114i8, 40i8, 41i8, 32i8, 45i8, 62i8,
    32i8, 98i8, 111i8, 111i8, 108i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8,
    84i8, 114i8, 117i8, 101i8, 32i8, 105i8, 102i8, 32i8, 97i8, 108i8, 108i8, 32i8, 99i8, 97i8,
    115i8, 101i8, 100i8, 32i8, 99i8, 104i8, 97i8, 114i8, 97i8, 99i8, 116i8, 101i8, 114i8, 115i8,
    32i8, 105i8, 110i8, 32i8, 66i8, 32i8, 97i8, 114i8, 101i8, 32i8, 117i8, 112i8, 112i8, 101i8,
    114i8, 99i8, 97i8, 115i8, 101i8, 32i8, 97i8, 110i8, 100i8, 32i8, 116i8, 104i8, 101i8, 114i8,
    101i8, 32i8, 105i8, 115i8, 10i8, 97i8, 116i8, 32i8, 108i8, 101i8, 97i8, 115i8, 116i8, 32i8,
    111i8, 110i8, 101i8, 32i8, 99i8, 97i8, 115i8, 101i8, 100i8, 32i8, 99i8, 104i8, 97i8, 114i8,
    97i8, 99i8, 116i8, 101i8, 114i8, 32i8, 105i8, 110i8, 32i8, 66i8, 44i8, 32i8, 70i8, 97i8, 108i8,
    115i8, 101i8, 32i8, 111i8, 116i8, 104i8, 101i8, 114i8, 119i8, 105i8, 115i8, 101i8, 46i8, 0i8,
]);

#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_bytes_isupper(cptr: *const i8, len: i64) -> *mut _object {
    let mut p: *const u8 = cptr as *const u8;
    let mut cased: i32 = 0;
    // Shortcut for single character strings
    if len == 1 {
        return unsafe { PyBool_FromLong(Py_ISUPPER!(unsafe { *p }) as i64) };
    }
    // Special case for empty strings
    if len == 0 {
        return _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_FalseStruct) });
    }
    let e: *const u8 = p.wrapping_offset(len as isize);
    while p < e {
        if Py_ISLOWER!(unsafe { *p }) != 0 {
            return _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_FalseStruct) });
        } else {
            if cased == 0 && Py_ISUPPER!(unsafe { *p }) != 0 {
                cased = 1;
            }
        }
        let __v1: *const u8 = p;
        p = __v1.wrapping_offset(1 as isize);
    }
    unsafe { PyBool_FromLong(cased as i64) }
}

#[unsafe(no_mangle)]
static _Py_istitle__doc__: __SlateAlign16<[i8; 230]> = __SlateAlign16([
    66i8, 46i8, 105i8, 115i8, 116i8, 105i8, 116i8, 108i8, 101i8, 40i8, 41i8, 32i8, 45i8, 62i8,
    32i8, 98i8, 111i8, 111i8, 108i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8,
    84i8, 114i8, 117i8, 101i8, 32i8, 105i8, 102i8, 32i8, 66i8, 32i8, 105i8, 115i8, 32i8, 97i8,
    32i8, 116i8, 105i8, 116i8, 108i8, 101i8, 99i8, 97i8, 115i8, 101i8, 100i8, 32i8, 115i8, 116i8,
    114i8, 105i8, 110i8, 103i8, 32i8, 97i8, 110i8, 100i8, 32i8, 116i8, 104i8, 101i8, 114i8, 101i8,
    32i8, 105i8, 115i8, 32i8, 97i8, 116i8, 32i8, 108i8, 101i8, 97i8, 115i8, 116i8, 32i8, 111i8,
    110i8, 101i8, 10i8, 99i8, 104i8, 97i8, 114i8, 97i8, 99i8, 116i8, 101i8, 114i8, 32i8, 105i8,
    110i8, 32i8, 66i8, 44i8, 32i8, 105i8, 46i8, 101i8, 46i8, 32i8, 117i8, 112i8, 112i8, 101i8,
    114i8, 99i8, 97i8, 115i8, 101i8, 32i8, 99i8, 104i8, 97i8, 114i8, 97i8, 99i8, 116i8, 101i8,
    114i8, 115i8, 32i8, 109i8, 97i8, 121i8, 32i8, 111i8, 110i8, 108i8, 121i8, 32i8, 102i8, 111i8,
    108i8, 108i8, 111i8, 119i8, 32i8, 117i8, 110i8, 99i8, 97i8, 115i8, 101i8, 100i8, 10i8, 99i8,
    104i8, 97i8, 114i8, 97i8, 99i8, 116i8, 101i8, 114i8, 115i8, 32i8, 97i8, 110i8, 100i8, 32i8,
    108i8, 111i8, 119i8, 101i8, 114i8, 99i8, 97i8, 115i8, 101i8, 32i8, 99i8, 104i8, 97i8, 114i8,
    97i8, 99i8, 116i8, 101i8, 114i8, 115i8, 32i8, 111i8, 110i8, 108i8, 121i8, 32i8, 99i8, 97i8,
    115i8, 101i8, 100i8, 32i8, 111i8, 110i8, 101i8, 115i8, 46i8, 32i8, 82i8, 101i8, 116i8, 117i8,
    114i8, 110i8, 32i8, 70i8, 97i8, 108i8, 115i8, 101i8, 10i8, 111i8, 116i8, 104i8, 101i8, 114i8,
    119i8, 105i8, 115i8, 101i8, 46i8, 0i8,
]);

#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_bytes_istitle(cptr: *const i8, len: i64) -> *mut _object {
    let mut p: *const u8 = cptr as *const u8;
    let mut cased: i32 = 0;
    let mut previous_is_cased: i32 = 0;
    if len == 1 {
        if Py_ISUPPER!(unsafe { *p }) != 0 {
            return _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_TrueStruct) });
        }
        return _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_FalseStruct) });
    }
    // Special case for empty strings
    if len == 0 {
        return _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_FalseStruct) });
    }
    let e: *const u8 = p.wrapping_offset(len as isize);
    while p < e {
        let ch: u8 = unsafe { *p };
        if Py_ISUPPER!(ch) != 0 {
            if previous_is_cased != 0 {
                return _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_FalseStruct) });
            }
            previous_is_cased = 1;
            cased = 1;
        } else {
            if Py_ISLOWER!(ch) != 0 {
                if previous_is_cased == 0 {
                    return _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_FalseStruct) });
                }
                previous_is_cased = 1;
                cased = 1;
            } else {
                previous_is_cased = 0;
            }
        }
        let __v1: *const u8 = p;
        p = __v1.wrapping_offset(1 as isize);
    }
    unsafe { PyBool_FromLong(cased as i64) }
}

#[unsafe(no_mangle)]
static _Py_lower__doc__: __SlateAlign16<[i8; 93]> = __SlateAlign16([
    66i8, 46i8, 108i8, 111i8, 119i8, 101i8, 114i8, 40i8, 41i8, 32i8, 45i8, 62i8, 32i8, 99i8, 111i8,
    112i8, 121i8, 32i8, 111i8, 102i8, 32i8, 66i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8,
    110i8, 32i8, 97i8, 32i8, 99i8, 111i8, 112i8, 121i8, 32i8, 111i8, 102i8, 32i8, 66i8, 32i8,
    119i8, 105i8, 116i8, 104i8, 32i8, 97i8, 108i8, 108i8, 32i8, 65i8, 83i8, 67i8, 73i8, 73i8, 32i8,
    99i8, 104i8, 97i8, 114i8, 97i8, 99i8, 116i8, 101i8, 114i8, 115i8, 32i8, 99i8, 111i8, 110i8,
    118i8, 101i8, 114i8, 116i8, 101i8, 100i8, 32i8, 116i8, 111i8, 32i8, 108i8, 111i8, 119i8, 101i8,
    114i8, 99i8, 97i8, 115i8, 101i8, 46i8, 0i8,
]);

#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_bytes_lower(result: *mut i8, cptr: *const i8, len: i64) {
    for i in 0..len {
        unsafe {
            *result.wrapping_offset(i as isize) =
                Py_TOLOWER!(*cptr.wrapping_offset(i as isize) as u8) as i8;
        }
    }
}

#[unsafe(no_mangle)]
static _Py_upper__doc__: __SlateAlign16<[i8; 93]> = __SlateAlign16([
    66i8, 46i8, 117i8, 112i8, 112i8, 101i8, 114i8, 40i8, 41i8, 32i8, 45i8, 62i8, 32i8, 99i8, 111i8,
    112i8, 121i8, 32i8, 111i8, 102i8, 32i8, 66i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8,
    110i8, 32i8, 97i8, 32i8, 99i8, 111i8, 112i8, 121i8, 32i8, 111i8, 102i8, 32i8, 66i8, 32i8,
    119i8, 105i8, 116i8, 104i8, 32i8, 97i8, 108i8, 108i8, 32i8, 65i8, 83i8, 67i8, 73i8, 73i8, 32i8,
    99i8, 104i8, 97i8, 114i8, 97i8, 99i8, 116i8, 101i8, 114i8, 115i8, 32i8, 99i8, 111i8, 110i8,
    118i8, 101i8, 114i8, 116i8, 101i8, 100i8, 32i8, 116i8, 111i8, 32i8, 117i8, 112i8, 112i8, 101i8,
    114i8, 99i8, 97i8, 115i8, 101i8, 46i8, 0i8,
]);

#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_bytes_upper(result: *mut i8, cptr: *const i8, len: i64) {
    for i in 0..len {
        unsafe {
            *result.wrapping_offset(i as isize) =
                Py_TOUPPER__3!(*cptr.wrapping_offset(i as isize) as u8) as i8;
        }
    }
}

#[unsafe(no_mangle)]
static _Py_title__doc__: __SlateAlign16<[i8; 155]> = __SlateAlign16([
    66i8, 46i8, 116i8, 105i8, 116i8, 108i8, 101i8, 40i8, 41i8, 32i8, 45i8, 62i8, 32i8, 99i8, 111i8,
    112i8, 121i8, 32i8, 111i8, 102i8, 32i8, 66i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8,
    110i8, 32i8, 97i8, 32i8, 116i8, 105i8, 116i8, 108i8, 101i8, 99i8, 97i8, 115i8, 101i8, 100i8,
    32i8, 118i8, 101i8, 114i8, 115i8, 105i8, 111i8, 110i8, 32i8, 111i8, 102i8, 32i8, 66i8, 44i8,
    32i8, 105i8, 46i8, 101i8, 46i8, 32i8, 65i8, 83i8, 67i8, 73i8, 73i8, 32i8, 119i8, 111i8, 114i8,
    100i8, 115i8, 32i8, 115i8, 116i8, 97i8, 114i8, 116i8, 32i8, 119i8, 105i8, 116i8, 104i8, 10i8,
    117i8, 112i8, 112i8, 101i8, 114i8, 99i8, 97i8, 115i8, 101i8, 32i8, 99i8, 104i8, 97i8, 114i8,
    97i8, 99i8, 116i8, 101i8, 114i8, 115i8, 44i8, 32i8, 97i8, 108i8, 108i8, 32i8, 114i8, 101i8,
    109i8, 97i8, 105i8, 110i8, 105i8, 110i8, 103i8, 32i8, 99i8, 97i8, 115i8, 101i8, 100i8, 32i8,
    99i8, 104i8, 97i8, 114i8, 97i8, 99i8, 116i8, 101i8, 114i8, 115i8, 32i8, 104i8, 97i8, 118i8,
    101i8, 32i8, 108i8, 111i8, 119i8, 101i8, 114i8, 99i8, 97i8, 115i8, 101i8, 46i8, 0i8,
]);

#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_bytes_title(mut result: *mut i8, mut s: *const i8, len: i64) {
    let mut previous_is_cased: i32 = 0;
    for i in 0..len {
        let mut c: i32 = 0;
        let __v0: *const i8 = s;
        s = __v0.wrapping_offset(1 as isize);
        c = ((((unsafe { *__v0 }) as i32) & 255) as u8) as i32;
        if Py_ISLOWER__2!(c) != 0 {
            if previous_is_cased == 0 {
                c = Py_TOUPPER__2!(c) as i32;
            }
            previous_is_cased = 1;
        } else {
            if Py_ISUPPER__2!(c) != 0 {
                if previous_is_cased != 0 {
                    c = Py_TOLOWER__2!(c) as i32;
                }
                previous_is_cased = 1;
            } else {
                previous_is_cased = 0;
            }
        }
        let __v2: *mut i8 = result;
        result = __v2.wrapping_offset(1 as isize);
        unsafe {
            *__v2 = c as i8;
        }
    }
}

#[unsafe(no_mangle)]
static _Py_capitalize__doc__: __SlateAlign16<[i8; 124]> = __SlateAlign16([
    66i8, 46i8, 99i8, 97i8, 112i8, 105i8, 116i8, 97i8, 108i8, 105i8, 122i8, 101i8, 40i8, 41i8,
    32i8, 45i8, 62i8, 32i8, 99i8, 111i8, 112i8, 121i8, 32i8, 111i8, 102i8, 32i8, 66i8, 10i8, 10i8,
    82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 97i8, 32i8, 99i8, 111i8, 112i8, 121i8, 32i8,
    111i8, 102i8, 32i8, 66i8, 32i8, 119i8, 105i8, 116i8, 104i8, 32i8, 111i8, 110i8, 108i8, 121i8,
    32i8, 105i8, 116i8, 115i8, 32i8, 102i8, 105i8, 114i8, 115i8, 116i8, 32i8, 99i8, 104i8, 97i8,
    114i8, 97i8, 99i8, 116i8, 101i8, 114i8, 32i8, 99i8, 97i8, 112i8, 105i8, 116i8, 97i8, 108i8,
    105i8, 122i8, 101i8, 100i8, 32i8, 40i8, 65i8, 83i8, 67i8, 73i8, 73i8, 41i8, 10i8, 97i8, 110i8,
    100i8, 32i8, 116i8, 104i8, 101i8, 32i8, 114i8, 101i8, 115i8, 116i8, 32i8, 108i8, 111i8, 119i8,
    101i8, 114i8, 45i8, 99i8, 97i8, 115i8, 101i8, 100i8, 46i8, 0i8,
]);

#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_bytes_capitalize(result: *mut i8, s: *const i8, len: i64) {
    if len > 0 {
        unsafe {
            *result = Py_TOUPPER__3!(unsafe { *s }) as i8;
        }
        _Py_bytes_lower(
            result.wrapping_offset(1 as isize),
            s.wrapping_offset(1 as isize),
            len.wrapping_sub(1),
        );
    }
}

#[unsafe(no_mangle)]
static _Py_swapcase__doc__: __SlateAlign16<[i8; 123]> = __SlateAlign16([
    66i8, 46i8, 115i8, 119i8, 97i8, 112i8, 99i8, 97i8, 115i8, 101i8, 40i8, 41i8, 32i8, 45i8, 62i8,
    32i8, 99i8, 111i8, 112i8, 121i8, 32i8, 111i8, 102i8, 32i8, 66i8, 10i8, 10i8, 82i8, 101i8,
    116i8, 117i8, 114i8, 110i8, 32i8, 97i8, 32i8, 99i8, 111i8, 112i8, 121i8, 32i8, 111i8, 102i8,
    32i8, 66i8, 32i8, 119i8, 105i8, 116i8, 104i8, 32i8, 117i8, 112i8, 112i8, 101i8, 114i8, 99i8,
    97i8, 115i8, 101i8, 32i8, 65i8, 83i8, 67i8, 73i8, 73i8, 32i8, 99i8, 104i8, 97i8, 114i8, 97i8,
    99i8, 116i8, 101i8, 114i8, 115i8, 32i8, 99i8, 111i8, 110i8, 118i8, 101i8, 114i8, 116i8, 101i8,
    100i8, 10i8, 116i8, 111i8, 32i8, 108i8, 111i8, 119i8, 101i8, 114i8, 99i8, 97i8, 115i8, 101i8,
    32i8, 65i8, 83i8, 67i8, 73i8, 73i8, 32i8, 97i8, 110i8, 100i8, 32i8, 118i8, 105i8, 99i8, 101i8,
    32i8, 118i8, 101i8, 114i8, 115i8, 97i8, 46i8, 0i8,
]);

#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_bytes_swapcase(mut result: *mut i8, mut s: *const i8, len: i64) {
    for i in 0..len {
        let mut c: i32 = 0;
        let __v0: *const i8 = s;
        s = __v0.wrapping_offset(1 as isize);
        c = ((((unsafe { *__v0 }) as i32) & 255) as u8) as i32;
        if Py_ISLOWER__2!(c) != 0 {
            unsafe {
                *result = Py_TOUPPER__2!(c) as i8;
            }
        } else {
            if Py_ISUPPER__2!(c) != 0 {
                unsafe {
                    *result = Py_TOLOWER__2!(c) as i8;
                }
            } else {
                unsafe {
                    *result = c as i8;
                }
            }
        }
        let __v2: *mut i8 = result;
        result = __v2.wrapping_offset(1 as isize);
    }
}

#[unsafe(no_mangle)]
static _Py_maketrans__doc__: __SlateAlign16<[i8; 291]> = __SlateAlign16([
    66i8, 46i8, 109i8, 97i8, 107i8, 101i8, 116i8, 114i8, 97i8, 110i8, 115i8, 40i8, 102i8, 114i8,
    109i8, 44i8, 32i8, 116i8, 111i8, 41i8, 32i8, 45i8, 62i8, 32i8, 116i8, 114i8, 97i8, 110i8,
    115i8, 108i8, 97i8, 116i8, 105i8, 111i8, 110i8, 32i8, 116i8, 97i8, 98i8, 108i8, 101i8, 10i8,
    10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 97i8, 32i8, 116i8, 114i8, 97i8, 110i8,
    115i8, 108i8, 97i8, 116i8, 105i8, 111i8, 110i8, 32i8, 116i8, 97i8, 98i8, 108i8, 101i8, 32i8,
    40i8, 97i8, 32i8, 98i8, 121i8, 116i8, 101i8, 115i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8,
    116i8, 32i8, 111i8, 102i8, 32i8, 108i8, 101i8, 110i8, 103i8, 116i8, 104i8, 32i8, 50i8, 53i8,
    54i8, 41i8, 32i8, 115i8, 117i8, 105i8, 116i8, 97i8, 98i8, 108i8, 101i8, 10i8, 102i8, 111i8,
    114i8, 32i8, 117i8, 115i8, 101i8, 32i8, 105i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 98i8,
    121i8, 116i8, 101i8, 115i8, 32i8, 111i8, 114i8, 32i8, 98i8, 121i8, 116i8, 101i8, 97i8, 114i8,
    114i8, 97i8, 121i8, 32i8, 116i8, 114i8, 97i8, 110i8, 115i8, 108i8, 97i8, 116i8, 101i8, 32i8,
    109i8, 101i8, 116i8, 104i8, 111i8, 100i8, 32i8, 119i8, 104i8, 101i8, 114i8, 101i8, 32i8, 101i8,
    97i8, 99i8, 104i8, 32i8, 98i8, 121i8, 116i8, 101i8, 10i8, 105i8, 110i8, 32i8, 102i8, 114i8,
    109i8, 32i8, 105i8, 115i8, 32i8, 109i8, 97i8, 112i8, 112i8, 101i8, 100i8, 32i8, 116i8, 111i8,
    32i8, 116i8, 104i8, 101i8, 32i8, 98i8, 121i8, 116i8, 101i8, 32i8, 97i8, 116i8, 32i8, 116i8,
    104i8, 101i8, 32i8, 115i8, 97i8, 109i8, 101i8, 32i8, 112i8, 111i8, 115i8, 105i8, 116i8, 105i8,
    111i8, 110i8, 32i8, 105i8, 110i8, 32i8, 116i8, 111i8, 46i8, 10i8, 84i8, 104i8, 101i8, 32i8,
    98i8, 121i8, 116i8, 101i8, 115i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 115i8, 32i8,
    102i8, 114i8, 109i8, 32i8, 97i8, 110i8, 100i8, 32i8, 116i8, 111i8, 32i8, 109i8, 117i8, 115i8,
    116i8, 32i8, 98i8, 101i8, 32i8, 111i8, 102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 115i8, 97i8,
    109i8, 101i8, 32i8, 108i8, 101i8, 110i8, 103i8, 116i8, 104i8, 46i8, 0i8,
]);

#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_bytes_maketrans(
    frm: *mut __SlateRecord_pybuffer_h_33,
    to: *mut __SlateRecord_pybuffer_h_33,
) -> *mut _object {
    let mut res: *mut _object = core::ptr::null_mut::<_object>();
    let mut i: i64 = 0;
    if (unsafe { (*frm).len }) != unsafe { (*to).len } {
        unsafe {
            PyErr_Format(
                PyExc_ValueError,
                c"maketrans arguments must have same length".as_ptr(),
            )
        };
        return core::ptr::null_mut::<_object>();
    }
    res = unsafe { PyBytes_FromStringAndSize(core::ptr::null::<i8>(), 256i64) };
    if res.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let p: *mut i8 = PyBytes_AS_STRING!(res);
    while i < 256 {
        unsafe {
            *p.wrapping_offset(i as isize) = i as i8;
        }
        i = i.wrapping_add(1);
    }
    i = 0;
    while i < unsafe { (*frm).len } {
        unsafe {
            *p.wrapping_offset(
                (*((*frm).buf as *mut u8).wrapping_offset(i as isize) as i32) as isize,
            ) = *((*to).buf as *mut i8).wrapping_offset(i as isize);
        }
        i = i.wrapping_add(1);
    }
    res
}

fn stringlib_find_char(s: *const i8, n: i64, ch: i8) -> i64 {
    let mut p: *const i8 = unsafe { core::mem::zeroed() };
    p = s;
    let e: *const i8 = s.wrapping_offset(n as isize);
    if n > 15 {
        p = ((unsafe {
            memchr(
                (s as *const ()) as *const core::ffi::c_void,
                (ch as i32) as i32,
                (n as u64) as usize,
            ) as *mut ()
        }) as *const ()) as *const i8;
        if !p.is_null() {
            return (unsafe { p.offset_from(s as *const i8) }) as i64;
        }
        return 1i32.wrapping_neg() as i64;
    }
    while p < e {
        if ((unsafe { *p }) as i32) == (ch as i32) {
            return (unsafe { p.offset_from(s as *const i8) }) as i64;
        }
        let __v0: *const i8 = p;
        p = __v0.wrapping_offset(1 as isize);
    }
    1i32.wrapping_neg() as i64
}

fn stringlib_rfind_char(s: *const i8, n: i64, ch: i8) -> i64 {
    let mut p: *const i8 = unsafe { core::mem::zeroed() };
    // memrchr() is a GNU extension, available since glibc 2.1.91.  it
    // doesn't seem as optimized as memchr(), but is still quite
    // faster than our hand-written loop below. There is no wmemrchr
    // for 4-byte chars.
    if n > 15 {
        p = (unsafe { memrchr(s as *const (), ch as i32, n as u64) }) as *const i8;
        if !p.is_null() {
            return (unsafe { p.offset_from(s as *const i8) }) as i64;
        }
        return 1i32.wrapping_neg() as i64;
    }
    p = s.wrapping_offset(n as isize);
    while p > s {
        let __v0: *const i8 = p;
        p = __v0.wrapping_offset(-(1 as isize));
        if ((unsafe { *p }) as i32) == (ch as i32) {
            return (unsafe { p.offset_from(s as *const i8) }) as i64;
        }
    }
    1i32.wrapping_neg() as i64
}

fn stringlib__lex_search(
    needle: *const i8,
    len_needle: i64,
    return_period: &mut i64,
    invert_alphabet: i32,
) -> i64 {
    // Do a lexicographic search. Essentially this:
    //     >>> max(needle[i:] for i in range(len(needle)+1))
    // Also find the period of the right half.
    let mut max_suffix: i64 = 0;
    let mut candidate: i64 = 1;
    let mut k: i64 = 0;
    // The period of the right half.
    let mut period: i64 = 1;
    while candidate.wrapping_add(k) < len_needle {
        // each loop increases candidate + k + max_suffix
        let a: i8 = unsafe { *needle.wrapping_offset(candidate.wrapping_add(k) as isize) };
        let b: i8 = unsafe { *needle.wrapping_offset(max_suffix.wrapping_add(k) as isize) };
        // check if the suffix at candidate is better than max_suffix
        if (if invert_alphabet != 0 {
            ((b as i32) < (a as i32)) as i32
        } else {
            ((a as i32) < (b as i32)) as i32
        }) != 0
        {
            // Fell short of max_suffix.
            //
            // The next k + 1 characters are non-increasing
            //
            // from candidate, so they won't start a maximal suffix.
            candidate = candidate.wrapping_add(k.wrapping_add(1));
            k = 0;
            // We've ruled out any period smaller than what's
            //
            // been scanned since max_suffix.
            period = candidate.wrapping_sub(max_suffix);
        } else {
            if (a as i32) == (b as i32) {
                if k.wrapping_add(1) != period {
                    // Keep scanning the equal strings
                    k = k.wrapping_add(1);
                } else {
                    // Matched a whole period.
                    //
                    // Start matching the next period.
                    candidate = candidate.wrapping_add(period);
                    k = 0;
                }
            } else {
                // Did better than max_suffix, so replace it.
                max_suffix = candidate;
                candidate = candidate.wrapping_add(1);
                k = 0;
                period = 1;
            }
        }
    }
    unsafe {
        *(return_period as *mut i64) = period;
    }
    max_suffix
}

fn stringlib__factorize(needle: *const i8, len_needle: i64, return_period: *mut i64) -> i64 {
    // Do a "critical factorization", making it so that:
    // >>> needle = (left := needle[:cut]) + (right := needle[cut:])
    // where the "local period" of the cut is maximal.
    //
    // The local period of the cut is the minimal length of a string w
    // such that (left endswith w or w endswith left)
    // and (right startswith w or w startswith right).
    //
    // The Critical Factorization Theorem says that this maximal local
    // period is the global period of the string.
    //
    // Crochemore and Perrin (1991) show that this cut can be computed
    // as the later of two cuts: one that gives a lexicographically
    // maximal right half, and one that gives the same with the
    // with respect to a reversed alphabet-ordering.
    //
    // This is what we want to happen:
    //     >>> x = "GCAGAGAG"
    //     >>> cut, period = factorize(x)
    //     >>> x[:cut], (right := x[cut:])
    //     ('GC', 'AGAGAG')
    //     >>> period  # right half period
    //     2
    //     >>> right[period:] == right[:-period]
    //     True
    //
    // This is how the local period lines up in the above example:
    //          GC | AGAGAG
    //     AGAGAGC = AGAGAGC
    // The length of this minimal repetition is 7, which is indeed the
    // period of the original string.
    let mut cut1: i64 = 0;
    let mut period1: i64 = 0;
    let mut cut2: i64 = 0;
    let mut period2: i64 = 0;
    let mut cut: i64 = 0;
    let mut period: i64 = 0;
    cut1 = stringlib__lex_search(
        needle,
        len_needle,
        unsafe { &mut (*core::ptr::addr_of_mut!(period1)) },
        0i32,
    );
    cut2 = stringlib__lex_search(
        needle,
        len_needle,
        unsafe { &mut (*core::ptr::addr_of_mut!(period2)) },
        1i32,
    );
    // Take the later cut.
    if cut1 > cut2 {
        period = period1;
        cut = cut1;
    } else {
        period = period2;
        cut = cut2;
    }
    unsafe {
        *return_period = period;
    }
    cut
}

#[repr(C)]
#[derive(Clone, Copy)]
struct stringlib__pre {
    needle: *const i8,
    len_needle: i64,
    cut: i64,
    period: i64,
    gap: i64,
    is_periodic: i32,
    table: [u8; 64],
}

fn stringlib__preprocess(needle: *const i8, len_needle: i64, p: *mut stringlib__pre) {
    unsafe {
        (*p).needle = needle;
    }
    unsafe {
        (*p).len_needle = len_needle;
    }
    unsafe {
        (*p).cut = stringlib__factorize(needle, len_needle, core::ptr::addr_of_mut!((*p).period));
    }
    unsafe {
        (*p).is_periodic = (0
            == (memcmp(
                (needle as *const ()) as *const core::ffi::c_void,
                (needle.wrapping_offset((*p).period as isize) as *const ())
                    as *const core::ffi::c_void,
                (unsafe { (*p).cut }.wrapping_mul(1) as u64) as usize,
            ) as i32)) as i32;
    }
    if (unsafe { (*p).is_periodic }) == 0 {
        // A lower bound on the period
        unsafe {
            (*p).period =
                Py_MAX!(unsafe { (*p).cut }, len_needle.wrapping_sub((*p).cut)).wrapping_add(1);
        }
    }
    // The gap between the last character and the previous
    //
    // occurrence of an equivalent character (modulo TABLE_SIZE)
    unsafe {
        (*p).gap = len_needle;
    }
    let last: i8 = (((unsafe { *needle.wrapping_offset(len_needle.wrapping_sub(1) as isize) })
        as u32)
        & 63u32) as i8;
    {
        let mut i: i64 = len_needle.wrapping_sub(2);
        while i >= 0 {
            let x: i8 = (((unsafe { *needle.wrapping_offset(i as isize) }) as u32) & 63u32) as i8;
            if (x as i32) == (last as i32) {
                unsafe {
                    (*p).gap = len_needle.wrapping_sub(1).wrapping_sub(i);
                }
                break;
            }
            i = i.wrapping_sub(1);
        }
    }
    // Fill up a compressed Boyer-Moore "Bad Character" table
    let not_found_shift: i64 = Py_MIN__2!(len_needle, 255i32);
    for i in 0..64i64 {
        unsafe {
            *((*p).table.as_mut_ptr() as *mut u8).wrapping_offset(i as isize) =
                Py_SAFE_DOWNCAST!(not_found_shift, u8);
        }
    }
    let mut i: i64 = len_needle.wrapping_sub(not_found_shift);
    while i < len_needle {
        let shift: u8 = Py_SAFE_DOWNCAST!(len_needle.wrapping_sub(1).wrapping_sub(i), u8);
        unsafe {
            *((*p).table.as_mut_ptr() as *mut u8)
                .wrapping_offset(((*needle.wrapping_offset(i as isize) as u32) & 63u32) as isize) =
                shift;
        }
        i = i.wrapping_add(1);
    }
}

fn stringlib__two_way(haystack: *const i8, len_haystack: i64, p: &mut stringlib__pre) -> i64 {
    let mut shift: i64 = unsafe { core::mem::zeroed() };
    let mut i: i64 = unsafe { core::mem::zeroed() };
    let mut memory: i64 = unsafe { core::mem::zeroed() };
    let mut i_1: i64 = unsafe { core::mem::zeroed() };
    let mut i_2: i64 = unsafe { core::mem::zeroed() };
    let mut gap_jump_end: i64 = unsafe { core::mem::zeroed() };
    let mut gap: i64 = unsafe { core::mem::zeroed() };
    let mut window: *const i8 = unsafe { core::mem::zeroed() };
    let mut table: *mut u8 = unsafe { core::mem::zeroed() };
    let mut haystack_end: *const i8 = unsafe { core::mem::zeroed() };
    let mut window_last: *const i8 = unsafe { core::mem::zeroed() };
    let mut needle: *const i8 = unsafe { core::mem::zeroed() };
    let mut period: i64 = unsafe { core::mem::zeroed() };
    let mut cut: i64 = unsafe { core::mem::zeroed() };
    // Crochemore and Perrin's (1991) Two-Way algorithm.
    //
    // See http://www-igm.univ-mlv.fr/~lecroq/string/node26.html#SECTION00260
    let mut len_needle: i64 = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_0: {
            len_needle = (*(p as *mut stringlib__pre)).len_needle;
            cut = (*(p as *mut stringlib__pre)).cut;
            period = (*(p as *mut stringlib__pre)).period;
            needle = (*(p as *mut stringlib__pre)).needle;
            window_last = haystack
                .wrapping_offset(len_needle as isize)
                .wrapping_offset(-(1 as isize));
            haystack_end = haystack.wrapping_offset(len_haystack as isize);
            table = (*(p as *mut stringlib__pre)).table.as_mut_ptr() as *mut u8;
            gap = (*(p as *mut stringlib__pre)).gap;
            gap_jump_end = Py_MIN!(len_needle, cut.wrapping_add(gap));
            if (*(p as *mut stringlib__pre)).is_periodic != 0 {
                memory = 0;
                '__loop_1: loop {
                    if window_last < haystack_end {
                        loop {
                            let shift: i64 = *table
                                .wrapping_offset(((*window_last as u32) & 63u32) as isize)
                                as i64;
                            let __v16: *const i8 = window_last;
                            window_last = __v16.wrapping_offset(shift as isize);
                            if shift == 0 {
                                break;
                            } else {
                                if window_last >= haystack_end {
                                    break '__loop_1;
                                }
                            }
                        }
                        '__join_16: {
                            loop {
                                window = window_last
                                    .wrapping_offset(-(len_needle as isize))
                                    .wrapping_offset(1 as isize);
                                i = Py_MAX!(cut, memory);
                                while i < len_needle {
                                    if (*needle.wrapping_offset(i as isize) as i32)
                                        != (*window.wrapping_offset(i as isize) as i32)
                                    {
                                        break '__join_16;
                                    } else {
                                        i = i.wrapping_add(1);
                                    }
                                }
                                i = memory;
                                loop {
                                    if i < cut {
                                        if (*needle.wrapping_offset(i as isize) as i32)
                                            != (*window.wrapping_offset(i as isize) as i32)
                                        {
                                            break;
                                        } else {
                                            i = i.wrapping_add(1);
                                        }
                                    } else {
                                        return window.offset_from(haystack as *const i8) as i64;
                                    }
                                }
                                let __v7: *const i8 = window_last;
                                window_last = __v7.wrapping_offset(period as isize);
                                memory = len_needle.wrapping_sub(period);
                                if window_last >= haystack_end {
                                    return 1i32.wrapping_neg() as i64;
                                } else {
                                    shift = *table
                                        .wrapping_offset(((*window_last as u32) & 63u32) as isize)
                                        as i64;
                                    if shift != 0 {
                                        break;
                                    }
                                }
                            }
                            // A mismatch has been identified to the right
                            //
                            // of where i will next start, so we can jump
                            //
                            // at least as far as if the mismatch occurred
                            //
                            // on the first comparison.
                            let mem_jump: i64 =
                                Py_MAX!(cut, memory).wrapping_sub(cut).wrapping_add(1);
                            memory = 0;
                            let __v3: *const i8 = window_last;
                            window_last = __v3.wrapping_offset(Py_MAX!(shift, mem_jump) as isize);
                            continue '__loop_1;
                        }
                        if i < gap_jump_end {
                            let __v11: *const i8 = window_last;
                            window_last = __v11.wrapping_offset(gap as isize);
                        } else {
                            let __v13: *const i8 = window_last;
                            window_last =
                                __v13.wrapping_offset(i.wrapping_sub(cut).wrapping_add(1) as isize);
                        }
                        memory = 0;
                    } else {
                        break '__join_0;
                    }
                }
                return 1i32.wrapping_neg() as i64;
            } else {
                period = Py_MAX!(gap, period);
                '__loop_25: loop {
                    if window_last < haystack_end {
                        loop {
                            let shift: i64 = *table
                                .wrapping_offset(((*window_last as u32) & 63u32) as isize)
                                as i64;
                            let __v32: *const i8 = window_last;
                            window_last = __v32.wrapping_offset(shift as isize);
                            if shift == 0 {
                                break;
                            } else {
                                if window_last >= haystack_end {
                                    break '__loop_25;
                                }
                            }
                        }
                        window = window_last
                            .wrapping_offset(-(len_needle as isize))
                            .wrapping_offset(1 as isize);
                        i_2 = cut;
                        '__join_36: {
                            while i_2 < len_needle {
                                if (*needle.wrapping_offset(i_2 as isize) as i32)
                                    != (*window.wrapping_offset(i_2 as isize) as i32)
                                {
                                    break '__join_36;
                                } else {
                                    i_2 = i_2.wrapping_add(1);
                                }
                            }
                            i_1 = 0;
                            loop {
                                if i_1 < cut {
                                    if (*needle.wrapping_offset(i_1 as isize) as i32)
                                        != (*window.wrapping_offset(i_1 as isize) as i32)
                                    {
                                        break;
                                    } else {
                                        i_1 = i_1.wrapping_add(1);
                                    }
                                } else {
                                    return window.offset_from(haystack as *const i8) as i64;
                                }
                            }
                            let __v22: *const i8 = window_last;
                            window_last = __v22.wrapping_offset(period as isize);
                            continue '__loop_25;
                        }
                        if i_2 < gap_jump_end {
                            let __v27: *const i8 = window_last;
                            window_last = __v27.wrapping_offset(gap as isize);
                        } else {
                            let __v29: *const i8 = window_last;
                            window_last = __v29
                                .wrapping_offset(i_2.wrapping_sub(cut).wrapping_add(1) as isize);
                        }
                    } else {
                        break '__join_0;
                    }
                }
                return 1i32.wrapping_neg() as i64;
            }
        }
        return 1i32.wrapping_neg() as i64;
    }
}

fn stringlib__two_way_find(
    haystack: *const i8,
    len_haystack: i64,
    needle: *const i8,
    len_needle: i64,
) -> i64 {
    let mut p: stringlib__pre = unsafe { core::mem::zeroed() };
    stringlib__preprocess(needle, len_needle, core::ptr::addr_of_mut!(p));
    stringlib__two_way(haystack, len_haystack, unsafe {
        &mut (*core::ptr::addr_of_mut!(p))
    })
}

fn stringlib__two_way_count(
    haystack: *const i8,
    len_haystack: i64,
    needle: *const i8,
    len_needle: i64,
    maxcount: i64,
) -> i64 {
    let mut p: stringlib__pre = unsafe { core::mem::zeroed() };
    stringlib__preprocess(needle, len_needle, core::ptr::addr_of_mut!(p));
    let mut index: i64 = 0;
    let mut count: i64 = 0;
    while true {
        let result: i64 = stringlib__two_way(
            haystack.wrapping_offset(index as isize),
            len_haystack.wrapping_sub(index),
            unsafe { &mut (*core::ptr::addr_of_mut!(p)) },
        );
        if result == (1i32.wrapping_neg() as i64) {
            return count;
        }
        count = count.wrapping_add(1);
        if count == maxcount {
            return maxcount;
        }
        index = index.wrapping_add(result.wrapping_add(len_needle));
    }
    count
}

fn stringlib_default_find(
    s: *const i8,
    n: i64,
    p: *const i8,
    m: i64,
    maxcount: i64,
    mode: i32,
) -> i64 {
    let w: i64 = n.wrapping_sub(m);
    let mlast: i64 = m.wrapping_sub(1);
    let mut count: i64 = 0;
    let mut gap: i64 = mlast;
    let last: i8 = unsafe { *p.wrapping_offset(mlast as isize) };
    let ss: *const i8 = s.wrapping_offset(mlast as isize);
    let mut mask: u64 = 0;
    for i in 0..mlast {
        STRINGLIB_BLOOM_ADD!(mask, unsafe { *p.wrapping_offset(i as isize) });
        if ((unsafe { *p.wrapping_offset(i as isize) }) as i32) == (last as i32) {
            gap = mlast.wrapping_sub(i).wrapping_sub(1);
        }
    }
    STRINGLIB_BLOOM_ADD!(mask, last);
    let mut i: i64 = 0;
    while i <= w {
        if ((unsafe { *ss.wrapping_offset(i as isize) }) as i32) == (last as i32) {
            // candidate match
            let mut j: i64 = 0;
            while j < mlast {
                if ((unsafe { *s.wrapping_offset(i.wrapping_add(j) as isize) }) as i32)
                    != ((unsafe { *p.wrapping_offset(j as isize) }) as i32)
                {
                    break;
                }
                j = j.wrapping_add(1);
            }
            if j == mlast {
                // got a match!
                if mode != 0 {
                    return i;
                }
                count = count.wrapping_add(1);
                if count == maxcount {
                    return maxcount;
                }
                i = i.wrapping_add(mlast);
            } else {
                // miss: check if next character is part of pattern
                if i.wrapping_add(1) <= w
                    && STRINGLIB_BLOOM!(mask, unsafe {
                        *ss.wrapping_offset(i.wrapping_add(1) as isize)
                    }) == 0
                {
                    i = i.wrapping_add(m);
                } else {
                    i = i.wrapping_add(gap);
                }
            }
        } else {
            // skip: check if next character is part of pattern
            if i.wrapping_add(1) <= w
                && STRINGLIB_BLOOM!(mask, unsafe {
                    *ss.wrapping_offset(i.wrapping_add(1) as isize)
                }) == 0
            {
                i = i.wrapping_add(m);
            }
        }
        i = i.wrapping_add(1);
    }
    if mode == 0 {
        count
    } else {
        1i32.wrapping_neg() as i64
    }
}

fn stringlib_adaptive_find(
    s: *const i8,
    n: i64,
    p: *const i8,
    m: i64,
    maxcount: i64,
    mode: i32,
) -> i64 {
    let w: i64 = n.wrapping_sub(m);
    let mlast: i64 = m.wrapping_sub(1);
    let mut count: i64 = 0;
    let mut gap: i64 = mlast;
    let mut hits: i64 = 0;
    let mut res: i64 = 0;
    let last: i8 = unsafe { *p.wrapping_offset(mlast as isize) };
    let ss: *const i8 = s.wrapping_offset(mlast as isize);
    let mut mask: u64 = 0;
    for i in 0..mlast {
        STRINGLIB_BLOOM_ADD!(mask, unsafe { *p.wrapping_offset(i as isize) });
        if ((unsafe { *p.wrapping_offset(i as isize) }) as i32) == (last as i32) {
            gap = mlast.wrapping_sub(i).wrapping_sub(1);
        }
    }
    STRINGLIB_BLOOM_ADD!(mask, last);
    let mut i: i64 = 0;
    while i <= w {
        if ((unsafe { *ss.wrapping_offset(i as isize) }) as i32) == (last as i32) {
            // candidate match
            let mut j: i64 = 0;
            while j < mlast {
                if ((unsafe { *s.wrapping_offset(i.wrapping_add(j) as isize) }) as i32)
                    != ((unsafe { *p.wrapping_offset(j as isize) }) as i32)
                {
                    break;
                }
                j = j.wrapping_add(1);
            }
            if j == mlast {
                // got a match!
                if mode != 0 {
                    return i;
                }
                count = count.wrapping_add(1);
                if count == maxcount {
                    return maxcount;
                }
                i = i.wrapping_add(mlast);
            } else {
                hits = hits.wrapping_add(j.wrapping_add(1));
                if hits > m / 4 && w.wrapping_sub(i) > 2000 {
                    if mode == 1 {
                        res = stringlib__two_way_find(
                            s.wrapping_offset(i as isize),
                            n.wrapping_sub(i),
                            p,
                            m,
                        );
                        return if res == (1i32.wrapping_neg() as i64) {
                            1i32.wrapping_neg() as i64
                        } else {
                            res.wrapping_add(i)
                        };
                    } else {
                        res = stringlib__two_way_count(
                            s.wrapping_offset(i as isize),
                            n.wrapping_sub(i),
                            p,
                            m,
                            maxcount.wrapping_sub(count),
                        );
                        return res.wrapping_add(count);
                    }
                }
                // miss: check if next character is part of pattern
                if i.wrapping_add(1) <= w
                    && STRINGLIB_BLOOM!(mask, unsafe {
                        *ss.wrapping_offset(i.wrapping_add(1) as isize)
                    }) == 0
                {
                    i = i.wrapping_add(m);
                } else {
                    i = i.wrapping_add(gap);
                }
            }
        } else {
            // skip: check if next character is part of pattern
            if i.wrapping_add(1) <= w
                && STRINGLIB_BLOOM!(mask, unsafe {
                    *ss.wrapping_offset(i.wrapping_add(1) as isize)
                }) == 0
            {
                i = i.wrapping_add(m);
            }
        }
        i = i.wrapping_add(1);
    }
    if mode == 0 {
        count
    } else {
        1i32.wrapping_neg() as i64
    }
}

fn stringlib_default_rfind(
    s: *const i8,
    n: i64,
    p: *const i8,
    m: i64,
    maxcount: i64,
    mode: i32,
) -> i64 {
    // create compressed boyer-moore delta 1 table
    let mut mask: u64 = 0;
    let mut i: i64 = 0;
    let mut j: i64 = 0;
    let mlast: i64 = m.wrapping_sub(1);
    let mut skip: i64 = m.wrapping_sub(1);
    let w: i64 = n.wrapping_sub(m);
    // process pattern[0] outside the loop
    STRINGLIB_BLOOM_ADD!(mask, unsafe { *p.wrapping_offset(0 as isize) });
    // process pattern[:0:-1]
    i = mlast;
    while i > 0 {
        STRINGLIB_BLOOM_ADD!(mask, unsafe { *p.wrapping_offset(i as isize) });
        if ((unsafe { *p.wrapping_offset(i as isize) }) as i32)
            == ((unsafe { *p.wrapping_offset(0 as isize) }) as i32)
        {
            skip = i.wrapping_sub(1);
        }
        i = i.wrapping_sub(1);
    }
    i = w;
    while i >= 0 {
        if ((unsafe { *s.wrapping_offset(i as isize) }) as i32)
            == ((unsafe { *p.wrapping_offset(0 as isize) }) as i32)
        {
            // candidate match
            j = mlast;
            while j > 0 {
                if ((unsafe { *s.wrapping_offset(i.wrapping_add(j) as isize) }) as i32)
                    != ((unsafe { *p.wrapping_offset(j as isize) }) as i32)
                {
                    break;
                }
                j = j.wrapping_sub(1);
            }
            if j == 0 {
                // got a match!
                return i;
            }
            // miss: check if previous character is part of pattern
            if i > 0
                && STRINGLIB_BLOOM!(mask, unsafe {
                    *s.wrapping_offset(i.wrapping_sub(1) as isize)
                }) == 0
            {
                i = i.wrapping_sub(m);
            } else {
                i = i.wrapping_sub(skip);
            }
        } else {
            // skip: check if previous character is part of pattern
            if i > 0
                && STRINGLIB_BLOOM!(mask, unsafe {
                    *s.wrapping_offset(i.wrapping_sub(1) as isize)
                }) == 0
            {
                i = i.wrapping_sub(m);
            }
        }
        i = i.wrapping_sub(1);
    }
    1i32.wrapping_neg() as i64
}

fn stringlib_count_char(s: *const i8, n: i64, p0: i8, maxcount: i64) -> i64 {
    let mut count: i64 = 0;
    for i in 0..n {
        if ((unsafe { *s.wrapping_offset(i as isize) }) as i32) == (p0 as i32) {
            count = count.wrapping_add(1);
            if count == maxcount {
                return maxcount;
            }
        }
    }
    count
}

fn stringlib_count_char_no_maxcount(s: *const i8, n: i64, p0: i8) -> i64 {
    let mut count: i64 = 0;
    for i in 0..n {
        if ((unsafe { *s.wrapping_offset(i as isize) }) as i32) == (p0 as i32) {
            count = count.wrapping_add(1);
        }
    }
    count
}

fn fastsearch(s: *const i8, n: i64, p: *const i8, m: i64, maxcount: i64, mode: i32) -> i64 {
    if n < m || mode == 0 && maxcount == 0 {
        return 1i32.wrapping_neg() as i64;
    }
    // look for special cases
    if m <= 1 {
        if m <= 0 {
            return 1i32.wrapping_neg() as i64;
        }
        // use special case for 1-character strings
        if mode == 1 {
            return stringlib_find_char(s, n, unsafe { *p.wrapping_offset(0 as isize) });
        } else {
            if mode == 2 {
                return stringlib_rfind_char(s, n, unsafe { *p.wrapping_offset(0 as isize) });
            } else {
                if maxcount == 9223372036854775807 {
                    return stringlib_count_char_no_maxcount(s, n, unsafe {
                        *p.wrapping_offset(0 as isize)
                    });
                }
                return stringlib_count_char(
                    s,
                    n,
                    unsafe { *p.wrapping_offset(0 as isize) },
                    maxcount,
                );
            }
        }
    }
    if mode != 2 {
        if n < 2500 || m < 100 && n < 30000 || m < 6 {
            return stringlib_default_find(s, n, p, m, maxcount, mode);
        } else {
            if (m >> 2i32).wrapping_mul(3) < n >> 2i32 {
                // 33% threshold, but don't overflow.
                //
                // For larger problems where the needle isn't a huge
                // percentage of the size of the haystack, the relatively
                // expensive O(m) startup cost of the two-way algorithm
                // will surely pay off.
                if mode == 1 {
                    return stringlib__two_way_find(s, n, p, m);
                } else {
                    return stringlib__two_way_count(s, n, p, m, maxcount);
                }
            } else {
                // To ensure that we have good worst-case behavior,
                // here's an adaptive version of the algorithm, where if
                // we match O(m) characters without any matches of the
                // entire needle, then we predict that the startup cost of
                // the two-way algorithm will probably be worth it.
                return stringlib_adaptive_find(s, n, p, m, maxcount, mode);
            }
        }
    } else {
        // FAST_RSEARCH
        return stringlib_default_rfind(s, n, p, m, maxcount, mode);
    }
}

fn stringlib_count(
    str: *const i8,
    str_len: i64,
    sub: *const i8,
    sub_len: i64,
    maxcount: i64,
) -> i64 {
    let mut count: i64 = 0;
    if str_len < 0 {
        return 0;
    }
    // start > len(str)
    if sub_len == 0 {
        return if str_len < maxcount {
            str_len.wrapping_add(1)
        } else {
            maxcount
        };
    }
    count = fastsearch(str, str_len, sub, sub_len, maxcount, 0i32);
    if count < 0 {
        return 0;
    }
    // no match
    count
}

fn stringlib_find(str: *const i8, str_len: i64, sub: *const i8, sub_len: i64, offset: i64) -> i64 {
    let mut pos: i64 = 0;
    if sub_len == 0 {
        return offset;
    }
    pos = fastsearch(str, str_len, sub, sub_len, 1i32.wrapping_neg() as i64, 1i32);
    if pos >= 0 {
        pos = pos.wrapping_add(offset);
    }
    pos
}

fn stringlib_rfind(str: *const i8, str_len: i64, sub: *const i8, sub_len: i64, offset: i64) -> i64 {
    let mut pos: i64 = 0;
    if sub_len == 0 {
        return str_len.wrapping_add(offset);
    }
    pos = fastsearch(str, str_len, sub, sub_len, 1i32.wrapping_neg() as i64, 2i32);
    if pos >= 0 {
        pos = pos.wrapping_add(offset);
    }
    pos
}

fn stringlib_find_slice(
    str: *const i8,
    str_len: i64,
    sub: *const i8,
    sub_len: i64,
    start: i64,
    end: i64,
) -> i64 {
    stringlib_find(
        str.wrapping_offset(start as isize),
        end.wrapping_sub(start),
        sub,
        sub_len,
        start,
    )
}

fn stringlib_rfind_slice(
    str: *const i8,
    str_len: i64,
    sub: *const i8,
    sub_len: i64,
    start: i64,
    end: i64,
) -> i64 {
    stringlib_rfind(
        str.wrapping_offset(start as isize),
        end.wrapping_sub(start),
        sub,
        sub_len,
        start,
    )
}

fn stringlib_find_max_char(begin: *const i8, end: *const i8) -> u32 {
    let mut p: *const u8 = begin as *const u8;
    let _end: *const u8 = end as *const u8;
    while p < _end {
        if _Py_IS_ALIGNED!(p, 8i32) {
            // Help register allocation
            let mut _p: *const u8 = p;
            while _p.wrapping_offset(8 as isize) <= _end {
                let value: u64 = unsafe { *(_p as *const u64) };
                if value & 9259542123273814144 != 0 {
                    return 255;
                }
                let __v0: *const u8 = _p;
                _p = __v0.wrapping_offset(8 as isize);
            }
            p = _p;
            if p == _end {
                break;
            }
        }
        let __v4: *const u8 = p;
        p = __v4.wrapping_offset(1 as isize);
        if ((unsafe { *__v4 }) as i32) & 128 != 0 {
            return 255;
        }
    }
    127
}

// Wraps stringlib_parse_args_finds() and additionally checks the first
// argument type.
//
// In case the first argument is a bytes-like object, sets it to subobj,
// and doesn't touch the byte parameter.
// In case it is an integer in range(0, 256), writes the integer value
// to byte, and sets subobj to NULL.
//
// The other parameters are similar to those of
// stringlib_parse_args_finds().
fn parse_args_finds_byte(
    function_name: *const i8,
    subobj: *mut *mut _object,
    byte: &mut i8,
) -> i32 {
    if (unsafe { PyObject_CheckBuffer(*subobj) }) != 0 {
        return 1;
    }
    if _PyIndex_Check(unsafe { *subobj }) == 0 {
        unsafe {
            PyErr_Format(
                PyExc_TypeError,
                c"argument should be integer or bytes-like object, not '%.200s'".as_ptr(),
                (*Py_TYPE!(unsafe { *subobj })).tp_name,
            )
        };
        return 0;
    }
    let ival: i64 = unsafe { PyNumber_AsSsize_t(*subobj, core::ptr::null_mut::<_object>()) };
    let __v0: bool = ival == (1i32.wrapping_neg() as i64) && !unsafe { PyErr_Occurred() }.is_null();
    if __v0 {
        return 0;
    }
    if ival < 0 || ival > 255 {
        unsafe { PyErr_SetString(PyExc_ValueError, c"byte must be in range(0, 256)".as_ptr()) };
        return 0;
    }
    unsafe {
        *subobj = core::ptr::null_mut::<_object>();
    }
    unsafe {
        *(byte as *mut i8) = ival as i8;
    }
    1
}

fn find_internal(
    str: *const i8,
    len: i64,
    function_name: *const i8,
    mut subobj: *mut _object,
    mut start: i64,
    mut end: i64,
    dir: i32,
) -> i64 {
    let mut byte: i8 = 0;
    let mut subbuf: __SlateRecord_pybuffer_h_33 = unsafe { core::mem::zeroed() };
    let mut sub: *const i8 = unsafe { core::mem::zeroed() };
    let mut sub_len: i64 = 0;
    let mut res: i64 = 0;
    if parse_args_finds_byte(function_name, core::ptr::addr_of_mut!(subobj), unsafe {
        &mut (*core::ptr::addr_of_mut!(byte))
    }) == 0
    {
        return 2i32.wrapping_neg() as i64;
    }
    if !subobj.is_null() {
        if (unsafe { PyObject_GetBuffer(subobj, core::ptr::addr_of_mut!(subbuf), 0i32) }) != 0 {
            return 2i32.wrapping_neg() as i64;
        }
        sub = subbuf.buf as *const i8;
        sub_len = subbuf.len;
    } else {
        sub = core::ptr::addr_of_mut!(byte) as *const i8;
        sub_len = 1;
    }
    ADJUST_INDICES!(start, end, len);
    if end.wrapping_sub(start) < sub_len {
        res = 1i32.wrapping_neg() as i64;
    } else {
        if sub_len == 1 {
            if dir > 0 {
                res = stringlib_find_char(
                    str.wrapping_offset(start as isize),
                    end.wrapping_sub(start),
                    unsafe { *sub },
                );
            } else {
                res = stringlib_rfind_char(
                    str.wrapping_offset(start as isize),
                    end.wrapping_sub(start),
                    unsafe { *sub },
                );
            }
            if res >= 0 {
                res = res.wrapping_add(start);
            }
        } else {
            if dir > 0 {
                res = stringlib_find_slice(str, len, sub, sub_len, start, end);
            } else {
                res = stringlib_rfind_slice(str, len, sub, sub_len, start, end);
            }
        }
    }
    if !subobj.is_null() {
        unsafe { PyBuffer_Release(core::ptr::addr_of_mut!(subbuf)) };
    }
    res
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.bytes_methods._Py_bytes_find")]
extern "C-unwind" fn _Py_bytes_find(
    str: *const i8,
    len: i64,
    sub: *mut _object,
    start: i64,
    end: i64,
) -> *mut _object {
    let result: i64 = find_internal(str, len, c"find".as_ptr(), sub, start, end, 1i32);
    if result == (2i32.wrapping_neg() as i64) {
        return core::ptr::null_mut::<_object>();
    }
    unsafe { PyLong_FromSsize_t(result) }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.bytes_methods._Py_bytes_index")]
extern "C-unwind" fn _Py_bytes_index(
    str: *const i8,
    len: i64,
    sub: *mut _object,
    start: i64,
    end: i64,
) -> *mut _object {
    let result: i64 = find_internal(str, len, c"index".as_ptr(), sub, start, end, 1i32);
    if result == (2i32.wrapping_neg() as i64) {
        return core::ptr::null_mut::<_object>();
    }
    if result == (1i32.wrapping_neg() as i64) {
        unsafe { PyErr_SetString(PyExc_ValueError, c"subsection not found".as_ptr()) };
        return core::ptr::null_mut::<_object>();
    }
    unsafe { PyLong_FromSsize_t(result) }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.bytes_methods._Py_bytes_rfind")]
extern "C-unwind" fn _Py_bytes_rfind(
    str: *const i8,
    len: i64,
    sub: *mut _object,
    start: i64,
    end: i64,
) -> *mut _object {
    let result: i64 = find_internal(
        str,
        len,
        c"rfind".as_ptr(),
        sub,
        start,
        end,
        1i32.wrapping_neg(),
    );
    if result == (2i32.wrapping_neg() as i64) {
        return core::ptr::null_mut::<_object>();
    }
    unsafe { PyLong_FromSsize_t(result) }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.bytes_methods._Py_bytes_rindex")]
extern "C-unwind" fn _Py_bytes_rindex(
    str: *const i8,
    len: i64,
    sub: *mut _object,
    start: i64,
    end: i64,
) -> *mut _object {
    let result: i64 = find_internal(
        str,
        len,
        c"rindex".as_ptr(),
        sub,
        start,
        end,
        1i32.wrapping_neg(),
    );
    if result == (2i32.wrapping_neg() as i64) {
        return core::ptr::null_mut::<_object>();
    }
    if result == (1i32.wrapping_neg() as i64) {
        unsafe { PyErr_SetString(PyExc_ValueError, c"subsection not found".as_ptr()) };
        return core::ptr::null_mut::<_object>();
    }
    unsafe { PyLong_FromSsize_t(result) }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.bytes_methods._Py_bytes_count")]
extern "C-unwind" fn _Py_bytes_count(
    str: *const i8,
    len: i64,
    mut sub_obj: *mut _object,
    mut start: i64,
    mut end: i64,
) -> *mut _object {
    let mut sub: *const i8 = unsafe { core::mem::zeroed() };
    let mut sub_len: i64 = 0;
    let mut byte: i8 = 0;
    let mut vsub: __SlateRecord_pybuffer_h_33 = unsafe { core::mem::zeroed() };
    if parse_args_finds_byte(
        c"count".as_ptr(),
        core::ptr::addr_of_mut!(sub_obj),
        unsafe { &mut (*core::ptr::addr_of_mut!(byte)) },
    ) == 0
    {
        return core::ptr::null_mut::<_object>();
    }
    if !sub_obj.is_null() {
        if (unsafe { PyObject_GetBuffer(sub_obj, core::ptr::addr_of_mut!(vsub), 0i32) }) != 0 {
            return core::ptr::null_mut::<_object>();
        }
        sub = vsub.buf as *const i8;
        sub_len = vsub.len;
    } else {
        sub = core::ptr::addr_of_mut!(byte) as *const i8;
        sub_len = 1;
    }
    ADJUST_INDICES!(start, end, len);
    let count_obj: *mut _object = unsafe {
        PyLong_FromSsize_t(stringlib_count(
            str.wrapping_offset(start as isize),
            end.wrapping_sub(start),
            sub,
            sub_len,
            9223372036854775807i64,
        ))
    };
    if !sub_obj.is_null() {
        unsafe { PyBuffer_Release(core::ptr::addr_of_mut!(vsub)) };
    }
    count_obj
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_bytes_contains(str: *const i8, len: i64, arg: *mut _object) -> i32 {
    let ival: i64 = unsafe { PyNumber_AsSsize_t(arg, core::ptr::null_mut::<_object>()) };
    let __v0: bool = ival == (1i32.wrapping_neg() as i64) && !unsafe { PyErr_Occurred() }.is_null();
    if __v0 {
        let mut varg: __SlateRecord_pybuffer_h_33 = unsafe { core::mem::zeroed() };
        let mut pos: i64 = 0;
        unsafe { PyErr_Clear() };
        if (unsafe { PyObject_GetBuffer(arg, core::ptr::addr_of_mut!(varg), 0i32) }) != 0 {
            return 1i32.wrapping_neg();
        }
        pos = stringlib_find(str, len, varg.buf as *const i8, varg.len, 0i64);
        unsafe { PyBuffer_Release(core::ptr::addr_of_mut!(varg)) };
        return (pos >= 0) as i32;
    }
    if ival < 0 || ival >= 256 {
        unsafe { PyErr_SetString(PyExc_ValueError, c"byte must be in range(0, 256)".as_ptr()) };
        return 1i32.wrapping_neg();
    }
    !((unsafe {
        memchr(
            (str as *const ()) as *const core::ffi::c_void,
            (ival as i32) as i32,
            (len as u64) as usize,
        ) as *mut ()
    }) as *const ())
        .is_null() as i32
}

/// Matches the end (direction >= 0) or start (direction < 0) of the buffer
/// against substr, using the start and end arguments. Returns
/// -1 on error, 0 if not found and 1 if found.
fn tailmatch(
    str: *const i8,
    len: i64,
    substr: *mut _object,
    mut start: i64,
    mut end: i64,
    direction: i32,
) -> i32 {
    let mut slen: i64 = unsafe { core::mem::zeroed() };
    let mut sub: *const i8 = unsafe { core::mem::zeroed() };
    let mut sub_view: core::mem::MaybeUninit<__SlateRecord_pybuffer_h_33> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_2: *mut __SlateRecord_pybuffer_h_33 =
        core::ptr::addr_of_mut!(sub_view) as *mut __SlateRecord_pybuffer_h_33;
    unsafe {
        core::ptr::write(__slate_slot_2, {
            let mut __t0: __SlateRecord_pybuffer_h_33 = core::mem::zeroed();
            __t0.buf = core::ptr::null_mut::<()>();
            __t0.obj = core::ptr::null_mut::<_object>();
            __t0
        });
        if PyBytes_Check!(substr) != 0 {
            sub = PyBytes_AS_STRING!(substr) as *const i8;
            slen = PyBytes_GET_SIZE!(substr);
        } else {
            if PyObject_GetBuffer(substr, core::ptr::addr_of_mut!(*__slate_slot_2), 0i32) != 0 {
                return 1i32.wrapping_neg();
            } else {
                sub = (*__slate_slot_2).buf as *const i8;
                slen = (*__slate_slot_2).len;
            }
        }
        '__join_0: {
            ADJUST_INDICES!(start, end, len);
            if direction < 0 {
                // startswith
                if start > len.wrapping_sub(slen) {
                    break '__join_0;
                }
            } else {
                // endswith
                if end.wrapping_sub(start) < slen || start > len {
                    break '__join_0;
                } else {
                    if end.wrapping_sub(slen) > start {
                        start = end.wrapping_sub(slen);
                    }
                }
            }
            if end.wrapping_sub(start) >= slen {
                if (memcmp(
                    (str.wrapping_offset(start as isize) as *const ()) as *const core::ffi::c_void,
                    (sub as *const ()) as *const core::ffi::c_void,
                    (slen as u64) as usize,
                ) as i32)
                    == 0
                {
                    PyBuffer_Release(core::ptr::addr_of_mut!(*__slate_slot_2));
                    return 1;
                }
            }
        }
        PyBuffer_Release(core::ptr::addr_of_mut!(*__slate_slot_2));
        return 0;
    }
}

fn _Py_bytes_tailmatch(
    str: *const i8,
    len: i64,
    function_name: *const i8,
    subobj: *mut _object,
    start: i64,
    end: i64,
    direction: i32,
) -> *mut _object {
    if PyTuple_Check!(subobj) != 0 {
        let mut i: i64 = 0;
        while i < PyTuple_GET_SIZE!(subobj) {
            let item: *mut _object = PyTuple_GET_ITEM!(subobj, i);
            let result: i32 = tailmatch(str, len, item, start, end, direction);
            if result < 0 {
                return core::ptr::null_mut::<_object>();
            } else {
                if result != 0 {
                    return _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_TrueStruct) });
                }
            }
            i = i.wrapping_add(1);
        }
        return _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_FalseStruct) });
    }
    let result: i32 = tailmatch(str, len, subobj, start, end, direction);
    if result == 1i32.wrapping_neg() {
        if (unsafe { PyErr_ExceptionMatches(PyExc_TypeError) }) != 0 {
            unsafe {
                PyErr_Format(
                    PyExc_TypeError,
                    c"%s first arg must be bytes or a tuple of bytes, not %s".as_ptr(),
                    function_name,
                    (*Py_TYPE!(subobj)).tp_name,
                )
            };
        }
        return core::ptr::null_mut::<_object>();
    }
    unsafe { PyBool_FromLong(result as i64) }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.bytes_methods._Py_bytes_startswith")]
extern "C-unwind" fn _Py_bytes_startswith(
    str: *const i8,
    len: i64,
    subobj: *mut _object,
    start: i64,
    end: i64,
) -> *mut _object {
    _Py_bytes_tailmatch(
        str,
        len,
        c"startswith".as_ptr(),
        subobj,
        start,
        end,
        1i32.wrapping_neg(),
    )
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.bytes_methods._Py_bytes_endswith")]
extern "C-unwind" fn _Py_bytes_endswith(
    str: *const i8,
    len: i64,
    subobj: *mut _object,
    start: i64,
    end: i64,
) -> *mut _object {
    _Py_bytes_tailmatch(str, len, c"endswith".as_ptr(), subobj, start, end, 1i32)
}

#[unsafe(no_mangle)]
static _Py_isascii__doc__: __SlateAlign16<[i8; 98]> = __SlateAlign16([
    66i8, 46i8, 105i8, 115i8, 97i8, 115i8, 99i8, 105i8, 105i8, 40i8, 41i8, 32i8, 45i8, 62i8, 32i8,
    98i8, 111i8, 111i8, 108i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 84i8,
    114i8, 117i8, 101i8, 32i8, 105i8, 102i8, 32i8, 66i8, 32i8, 105i8, 115i8, 32i8, 101i8, 109i8,
    112i8, 116i8, 121i8, 32i8, 111i8, 114i8, 32i8, 97i8, 108i8, 108i8, 32i8, 99i8, 104i8, 97i8,
    114i8, 97i8, 99i8, 116i8, 101i8, 114i8, 115i8, 32i8, 105i8, 110i8, 32i8, 66i8, 32i8, 97i8,
    114i8, 101i8, 32i8, 65i8, 83i8, 67i8, 73i8, 73i8, 44i8, 10i8, 70i8, 97i8, 108i8, 115i8, 101i8,
    32i8, 111i8, 116i8, 104i8, 101i8, 114i8, 119i8, 105i8, 115i8, 101i8, 46i8, 0i8,
]);

#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_bytes_isascii(cptr: *const i8, len: i64) -> *mut _object {
    let p: *const i8 = cptr;
    let end: *const i8 = p.wrapping_offset(len as isize);
    let max_char: i64 = stringlib_find_max_char(cptr, end) as i64;
    if max_char > 127 {
        return _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_FalseStruct) });
    }
    _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_TrueStruct) })
}
