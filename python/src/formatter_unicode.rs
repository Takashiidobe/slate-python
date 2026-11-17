//! implements the unicode (as opposed to string) version of the
//! built-in formatters for string, int, float.  that is, the versions
//! of int.__float__, etc., that take and return unicode objects
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
// if the result is an integer add ".0"
// "alternate" formatting. it's format_code
// specific
// negative zero result is coerced to 0
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
// Long value tag bits:
// 0-1: Sign bits value = (1-sign), ie. negative=2, positive=0, zero=1.
// 2: Set to 1 for the small ints
// 3+ Unsigned digit count
macro_rules! PyLong_CheckExact {
    ($op:expr) => {
        Py_IS_TYPE!($op, unsafe { core::ptr::addr_of_mut!(PyLong_Type) })
    };
}

macro_rules! PyUnicode_1BYTE_DATA {
    ($op:expr) => {
        _Py_STATIC_CAST!(*mut u8, PyUnicode_DATA!($op))
    };
}

macro_rules! PyUnicode_2BYTE_DATA {
    ($op:expr) => {
        _Py_STATIC_CAST!(*mut u16, PyUnicode_DATA!($op))
    };
}

macro_rules! PyUnicode_4BYTE_DATA {
    ($op:expr) => {
        _Py_STATIC_CAST!(*mut u32, PyUnicode_DATA!($op))
    };
}

macro_rules! PyUnicode_CheckExact {
    ($op:expr) => {
        Py_IS_TYPE!($op, unsafe { core::ptr::addr_of_mut!(PyUnicode_Type) })
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

macro_rules! PyUnicode_MAX_CHAR_VALUE {
    ($op:expr) => {
        PyUnicode_MAX_CHAR_VALUE(_PyObject_CAST!($op))
    };
}

macro_rules! PyUnicode_READ_CHAR {
    ($unicode:expr, $index:expr) => {
        PyUnicode_READ_CHAR(_PyObject_CAST!($unicode), $index)
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

macro_rules! PyUnicode_WRITE {
    ($kind:expr, $data:expr, $index:expr, $value:expr) => {
        PyUnicode_WRITE(
            _Py_STATIC_CAST!(i32, $kind),
            _Py_CAST__2!($data),
            $index,
            _Py_STATIC_CAST!(u32, $value),
        );
    };
}

macro_rules! PyUnicode_WRITE__2 {
    ($kind:expr, $data:expr, $index:expr, $value:expr) => {
        PyUnicode_WRITE(
            _Py_STATIC_CAST!(i32, $kind),
            _Py_CAST!(*mut (), $data),
            $index,
            _Py_STATIC_CAST!(u32, $value),
        );
    };
}

/// Argument must be a char or an int in [-128, 127] or [0, 255].
macro_rules! Py_CHARMASK__3 {
    ($c:expr) => {
        (($c) & 255) as u8
    };
}

macro_rules! Py_DECREF {
    ($op:expr) => {
        Py_DECREF(_PyObject_CAST!($op));
    };
}

macro_rules! Py_ISDIGIT__2 {
    ($c:expr) => {
        (unsafe { _Py_ctype_table[(Py_CHARMASK__3!($c) as i32) as usize] }) & 4
    };
}

macro_rules! Py_IS_TYPE {
    ($ob:expr, $type:expr) => {
        Py_IS_TYPE(_PyObject_CAST!($ob), $type)
    };
}

/// Maximum value between x and y
macro_rules! Py_MAX__6 {
    ($x:expr, $y:expr) => {{
        let __v: u32 = if ($x) > $y { $x } else { $y };
        __v
    }};
}

/// Maximum value between x and y
macro_rules! Py_MAX {
    ($x:expr, $y:expr) => {
        if ($x) > $y { $x } else { $y }
    };
}

macro_rules! Py_TOUPPER {
    ($c:expr) => {
        unsafe { _Py_ctype_toupper[(Py_CHARMASK__3!($c) as i32) as usize] }
    };
}

macro_rules! Py_TYPE {
    ($ob:expr) => {
        _Py_TYPE(_PyObject_CAST!($ob))
    };
}

macro_rules! Py_UNICODE_TODECIMAL {
    ($ch:expr) => {
        unsafe { _PyUnicode_ToDecimalDigit($ch) }
    };
}

macro_rules! Py_UNREACHABLE {
    () => {
        unsafe { core::hint::unreachable_unchecked() };
    };
}

macro_rules! Py_XDECREF {
    ($op:expr) => {
        Py_XDECREF(_PyObject_CAST!($op));
    };
}

macro_rules! READ_spec {
    ($index:expr, $kind:expr, $data:expr) => {
        PyUnicode_READ!($kind, $data, $index)
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

/// Prepare the buffer to write 'length' characters
/// with the specified maximum character.
///
/// Return 0 on success, raise an exception and return -1 on error.
macro_rules! _PyUnicodeWriter_Prepare__3 {
    ($WRITER:expr, $LENGTH:expr, $MAXCHAR:expr) => {{
        let __v: i32 = if ($MAXCHAR) <= unsafe { (*$WRITER).maxchar }
            && ($LENGTH) <= unsafe { (*$WRITER).size }.wrapping_sub(unsafe { (*$WRITER).pos })
        {
            0
        } else {
            let __v2: i32 = if ($LENGTH) == 0 {
                0
            } else {
                unsafe { _PyUnicodeWriter_PrepareInternal($WRITER, $LENGTH, $MAXCHAR) }
            };
            __v2
        };
        __v
    }};
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
    static mut PyLong_Type: _typeobject;
    static mut PyUnicode_Type: _typeobject;
    static mut PyExc_OverflowError: *mut _object;
    static mut PyExc_SystemError: *mut _object;
    static mut PyExc_ValueError: *mut _object;
    static mut _Py_ctype_table: [u32; 256];
    static mut _Py_ctype_toupper: [u8; 256];
    fn copysign(arg0: f64, arg1: f64) -> f64;
    fn index(arg0: *const i8, arg1: i32) -> *mut i8;
    fn PyMem_Free(arg0: *mut ());
    fn PyObject_Str(arg0: *mut _object) -> *mut _object;
    fn Py_GetConstant(arg0: u32) -> *mut _object;
    fn _Py_Dealloc(arg0: *mut _object);
    fn PyUnicode_FromOrdinal(arg0: i32) -> *mut _object;
    fn PyUnicode_FromKindAndData(arg0: i32, arg1: *const (), arg2: i64) -> *mut _object;
    fn _PyUnicodeWriter_PrepareInternal(
        arg0: *mut __SlateRecord_unicodeobject_h_539,
        arg1: i64,
        arg2: u32,
    ) -> i32;
    fn _PyUnicodeWriter_WriteStr(
        arg0: *mut __SlateRecord_unicodeobject_h_539,
        arg1: *mut _object,
    ) -> i32;
    fn _PyUnicodeWriter_WriteASCIIString(
        arg0: *mut __SlateRecord_unicodeobject_h_539,
        arg1: *const i8,
        arg2: i64,
    ) -> i32;
    fn _PyUnicode_ToDecimalDigit(arg0: u32) -> i32;
    fn PyErr_SetString(arg0: *mut _object, arg1: *const i8);
    fn PyErr_Occurred() -> *mut _object;
    fn PyErr_NoMemory() -> *mut _object;
    fn PyErr_Format(arg0: *mut _object, arg1: *const i8, ...) -> *mut _object;
    fn PyLong_AsLong(arg0: *mut _object) -> i64;
    fn PyFloat_AsDouble(arg0: *mut _object) -> f64;
    fn PyComplex_RealAsDouble(arg0: *mut _object) -> f64;
    fn PyComplex_ImagAsDouble(arg0: *mut _object) -> f64;
    fn PyNumber_Float(arg0: *mut _object) -> *mut _object;
    fn PyOS_double_to_string(arg0: f64, arg1: i8, arg2: i32, arg3: i32, arg4: *mut i32) -> *mut i8;
    fn _PyMem_Strdup(arg0: *const i8) -> *mut i8;
    fn _Py_GetLocaleconvNumeric(
        arg0: *mut slate_support::lconv,
        arg1: *mut *mut _object,
        arg2: *mut *mut _object,
    ) -> i32;
    fn _PyLong_Format(arg0: *mut _object, arg1: i32) -> *mut _object;
    fn _PyLong_FormatWriter(
        arg0: *mut __SlateRecord_unicodeobject_h_539,
        arg1: *mut _object,
        arg2: i32,
        arg3: i32,
    ) -> i32;
    fn _PyUnicode_FastFill(arg0: *mut _object, arg1: i64, arg2: i64, arg3: u32);
    fn _PyUnicode_FastCopyCharacters(
        arg0: *mut _object,
        arg1: i64,
        arg2: *mut _object,
        arg3: i64,
        arg4: i64,
    );
    fn _PyUnicode_FromASCII(arg0: *const i8, arg1: i64) -> *mut _object;
    fn _PyUnicode_FindMaxChar(arg0: *mut _object, arg1: i64, arg2: i64) -> u32;
    fn _PyUnicode_InsertThousandsGrouping(
        arg0: *mut __SlateRecord_unicodeobject_h_539,
        arg1: i64,
        arg2: *mut _object,
        arg3: i64,
        arg4: i64,
        arg5: i64,
        arg6: *const i8,
        arg7: *mut _object,
        arg8: *mut u32,
        arg9: i32,
    ) -> i64;
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

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_unicodeobject_h_163_2 {
    __slate_bits_0: __slate_bits::__SlateBits78U0,
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
union __SlateRecord_unicodeobject_h_184_2 {
    any: *mut (),
    latin1: *mut u8,
    ucs2: *mut u16,
    ucs4: *mut u32,
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
    pub struct __SlateBits78U0 {
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

use slate_support::localeconv;
use slate_support::strlen;

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

fn PyUnicode_WRITE(kind: i32, data: *mut (), index: i64, value: u32) {
    if kind == 1 {
        unsafe {
            *_Py_STATIC_CAST!(*mut u8, data).wrapping_offset(index as isize) =
                _Py_STATIC_CAST!(u8, value);
        }
    } else {
        if kind == 2 {
            unsafe {
                *_Py_STATIC_CAST!(*mut u16, data).wrapping_offset(index as isize) =
                    _Py_STATIC_CAST!(u16, value);
            }
        } else {
            unsafe {
                *_Py_STATIC_CAST!(*mut u32, data).wrapping_offset(index as isize) = value;
            }
        }
    }
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

fn PyUnicode_READ_CHAR(unicode: *mut _object, index: i64) -> u32 {
    let mut kind: i32 = 0;
    // Tolerate reading the NUL character at str[len(str)]
    kind = PyUnicode_KIND!(unicode);
    if kind == 1 {
        return (unsafe { *PyUnicode_1BYTE_DATA!(unicode).wrapping_offset(index as isize) }) as u32;
    }
    if kind == 2 {
        return (unsafe { *PyUnicode_2BYTE_DATA!(unicode).wrapping_offset(index as isize) }) as u32;
    }
    unsafe { *PyUnicode_4BYTE_DATA!(unicode).wrapping_offset(index as isize) }
}

fn PyUnicode_MAX_CHAR_VALUE(op: *mut _object) -> u32 {
    let mut kind: i32 = 0;
    if PyUnicode_IS_ASCII!(op) != 0 {
        return 127;
    }
    kind = PyUnicode_KIND!(op);
    if kind == 1 {
        return 255;
    }
    if kind == 2 {
        return 65535;
    }
    1114111
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_unicodeobject_h_539 {
    buffer: *mut _object,
    data: *mut (),
    kind: i32,
    maxchar: u32,
    size: i64,
    pos: i64,
    min_length: i64,
    min_char: u32,
    overallocate: u8,
    readonly: u8,
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

// _Py_GetLocaleconvNumeric()
// _PyLong_FormatWriter()
// PyUnicode_MAX_CHAR_VALUE()
// Raises an exception about an unknown presentation type for this
// type.
fn unknown_presentation_type(presentation_type: u32, type_name: *const i8) {
    // %c might be out-of-range, hence the two cases.
    if presentation_type > 32 && presentation_type < 128 {
        unsafe {
            PyErr_Format(
                PyExc_ValueError,
                c"Unknown format code '%c' for object of type '%.200s'".as_ptr(),
                (presentation_type as i8) as i32,
                type_name,
            )
        };
    } else {
        unsafe {
            PyErr_Format(
                PyExc_ValueError,
                c"Unknown format code '\\x%x' for object of type '%.200s'".as_ptr(),
                presentation_type,
                type_name,
            )
        };
    }
}

fn invalid_thousands_separator_type(specifier: i8, presentation_type: u32) {
    if presentation_type > 32 && presentation_type < 128 {
        unsafe {
            PyErr_Format(
                PyExc_ValueError,
                c"Cannot specify '%c' with '%c'.".as_ptr(),
                specifier as i32,
                (presentation_type as i8) as i32,
            )
        };
    } else {
        unsafe {
            PyErr_Format(
                PyExc_ValueError,
                c"Cannot specify '%c' with '\\x%x'.".as_ptr(),
                specifier as i32,
                presentation_type,
            )
        };
    }
}

fn invalid_comma_and_underscore() {
    unsafe {
        PyErr_Format(
            PyExc_ValueError,
            c"Cannot specify both ',' and '_'.".as_ptr(),
        )
    };
}

/// get_integer consumes 0 or more decimal digit characters from an
/// input string, updates *result with the corresponding positive
/// integer, and returns the number of digits consumed.
///
/// returns -1 on error.
fn get_integer(str: *mut _object, ppos: &mut i64, end: i64, result: *mut i64) -> i32 {
    let mut accumulator: i64 = 0;
    let mut digitval: i64 = 0;
    let mut pos: i64 = unsafe { *(ppos as *mut i64) };
    let mut numdigits: i32 = 0;
    let kind: i32 = PyUnicode_KIND!(str);
    let data: *const () = PyUnicode_DATA!(str) as *const ();
    while pos < end {
        digitval = Py_UNICODE_TODECIMAL!(PyUnicode_READ!(kind, data, pos)) as i64;
        if digitval < 0 {
            break;
        }
        // Detect possible overflow before it happens:
        //
        //    accumulator * 10 + digitval > PY_SSIZE_T_MAX if and only if
        //    accumulator > (PY_SSIZE_T_MAX - digitval) / 10.
        if accumulator > 9223372036854775807i64.wrapping_sub(digitval) / 10 {
            unsafe {
                PyErr_Format(
                    PyExc_ValueError,
                    c"Too many decimal digits in format string".as_ptr(),
                )
            };
            unsafe {
                *(ppos as *mut i64) = pos;
            }
            return 1i32.wrapping_neg();
        }
        accumulator = accumulator.wrapping_mul(10).wrapping_add(digitval);
        pos = pos.wrapping_add(1);
        numdigits = numdigits.wrapping_add(1);
    }
    unsafe {
        *(ppos as *mut i64) = pos;
    }
    unsafe {
        *result = accumulator;
    }
    numdigits
}

// standard format specifier parsing
/// returns true if this character is a specifier alignment token
fn is_alignment_token(c: u32) -> i32 {
    match c {
        60..=62 | 94 => {
            return 1;
        }
        _ => {
            return 0;
        }
    }
    unsafe { core::mem::zeroed() }
}

/// returns true if this character is a sign element
fn is_sign_element(c: u32) -> i32 {
    match c {
        32 | 43 | 45 => {
            return 1;
        }
        _ => {
            return 0;
        }
    }
    unsafe { core::mem::zeroed() }
}

// Locale type codes. LT_NO_LOCALE must be zero.
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_formatter_unicode_c_141 {
    fill_char: u32,
    align: u32,
    alternate: i32,
    no_neg_0: i32,
    sign: u32,
    width: i64,
    thousands_separators: u32,
    precision: i64,
    frac_thousands_separator: u32,
    r#type: u32,
}

/// ptr points to the start of the format_spec, end points just past its end.
/// fills in format with the parsed information.
/// returns 1 on success, 0 on failure.
/// if failure, sets the exception
fn parse_internal_render_format_spec(
    obj: *mut _object,
    format_spec: *mut _object,
    start: i64,
    end: i64,
    format: &mut __SlateRecord_formatter_unicode_c_141,
    default_type: i8,
    default_align: i8,
) -> i32 {
    let mut pos: i64 = start;
    let kind: i32 = PyUnicode_KIND!(format_spec);
    let data: *const () = PyUnicode_DATA!(format_spec) as *const ();
    // end-pos is used throughout this code to specify the length of
    // the input string
    let mut consumed: i64 = 0;
    let mut align_specified: i32 = 0;
    let mut fill_char_specified: i32 = 0;
    unsafe {
        (*(format as *mut __SlateRecord_formatter_unicode_c_141)).fill_char = 32;
    }
    unsafe {
        (*(format as *mut __SlateRecord_formatter_unicode_c_141)).align = default_align as u32;
    }
    unsafe {
        (*(format as *mut __SlateRecord_formatter_unicode_c_141)).alternate = 0;
    }
    unsafe {
        (*(format as *mut __SlateRecord_formatter_unicode_c_141)).no_neg_0 = 0;
    }
    unsafe {
        (*(format as *mut __SlateRecord_formatter_unicode_c_141)).sign = 0;
    }
    unsafe {
        (*(format as *mut __SlateRecord_formatter_unicode_c_141)).width =
            1i32.wrapping_neg() as i64;
    }
    unsafe {
        (*(format as *mut __SlateRecord_formatter_unicode_c_141)).thousands_separators = 0;
    }
    unsafe {
        (*(format as *mut __SlateRecord_formatter_unicode_c_141)).frac_thousands_separator = 0;
    }
    unsafe {
        (*(format as *mut __SlateRecord_formatter_unicode_c_141)).precision =
            1i32.wrapping_neg() as i64;
    }
    unsafe {
        (*(format as *mut __SlateRecord_formatter_unicode_c_141)).r#type = default_type as u32;
    }
    // If the second char is an alignment token,
    // then parse the fill char
    let __v0: bool = end.wrapping_sub(pos) >= 2
        && is_alignment_token(READ_spec!(pos.wrapping_add(1), kind, data)) != 0;
    if __v0 {
        unsafe {
            (*(format as *mut __SlateRecord_formatter_unicode_c_141)).align =
                READ_spec!(pos.wrapping_add(1), kind, data);
        }
        unsafe {
            (*(format as *mut __SlateRecord_formatter_unicode_c_141)).fill_char =
                READ_spec!(pos, kind, data);
        }
        fill_char_specified = 1;
        align_specified = 1;
        pos = pos.wrapping_add(2);
    } else {
        let __v3: bool =
            end.wrapping_sub(pos) >= 1 && is_alignment_token(READ_spec!(pos, kind, data)) != 0;
        if __v3 {
            unsafe {
                (*(format as *mut __SlateRecord_formatter_unicode_c_141)).align =
                    READ_spec!(pos, kind, data);
            }
            align_specified = 1;
            pos = pos.wrapping_add(1);
        }
    }
    // Parse the various sign options
    let __v6: bool =
        end.wrapping_sub(pos) >= 1 && is_sign_element(READ_spec!(pos, kind, data)) != 0;
    if __v6 {
        unsafe {
            (*(format as *mut __SlateRecord_formatter_unicode_c_141)).sign =
                READ_spec!(pos, kind, data);
        }
        pos = pos.wrapping_add(1);
    }
    // If the next character is z, request coercion of negative 0.
    // Applies only to floats.
    let __v9: bool = end.wrapping_sub(pos) >= 1 && READ_spec!(pos, kind, data) == 122;
    if __v9 {
        unsafe {
            (*(format as *mut __SlateRecord_formatter_unicode_c_141)).no_neg_0 = 1;
        }
        pos = pos.wrapping_add(1);
    }
    // If the next character is #, we're in alternate mode.  This only
    // applies to integers.
    let __v12: bool = end.wrapping_sub(pos) >= 1 && READ_spec!(pos, kind, data) == 35;
    if __v12 {
        unsafe {
            (*(format as *mut __SlateRecord_formatter_unicode_c_141)).alternate = 1;
        }
        pos = pos.wrapping_add(1);
    }
    // The special case for 0-padding (backwards compat)
    let __v15: bool =
        fill_char_specified == 0 && end.wrapping_sub(pos) >= 1 && READ_spec!(pos, kind, data) == 48;
    if __v15 {
        unsafe {
            (*(format as *mut __SlateRecord_formatter_unicode_c_141)).fill_char = 48;
        }
        if align_specified == 0 && (default_align as i32) == 62 {
            unsafe {
                (*(format as *mut __SlateRecord_formatter_unicode_c_141)).align = 61;
            }
        }
        pos = pos.wrapping_add(1);
    }
    consumed = get_integer(
        format_spec,
        unsafe { &mut (*core::ptr::addr_of_mut!(pos)) },
        end,
        unsafe {
            core::ptr::addr_of_mut!((*(format as *mut __SlateRecord_formatter_unicode_c_141)).width)
        },
    ) as i64;
    if consumed == (1i32.wrapping_neg() as i64) {
        return 0;
    }
    // Overflow error. Exception already set.
    // If consumed is 0, we didn't consume any characters for the
    // width. In that case, reset the width to -1, because
    // get_integer() will have set it to zero. -1 is how we record
    // that the width wasn't specified.
    if consumed == 0 {
        unsafe {
            (*(format as *mut __SlateRecord_formatter_unicode_c_141)).width =
                1i32.wrapping_neg() as i64;
        }
    }
    // Comma signifies add thousands separators
    let __v18: bool = end.wrapping_sub(pos) != 0 && READ_spec!(pos, kind, data) == 44;
    if __v18 {
        unsafe {
            (*(format as *mut __SlateRecord_formatter_unicode_c_141)).thousands_separators = 44;
        }
        pos = pos.wrapping_add(1);
    }
    // Underscore signifies add thousands separators
    let __v21: bool = end.wrapping_sub(pos) != 0 && READ_spec!(pos, kind, data) == 95;
    if __v21 {
        if ((unsafe {
            (*(format as *mut __SlateRecord_formatter_unicode_c_141)).thousands_separators
        }) as u32)
            != 0
        {
            invalid_comma_and_underscore();
            return 0;
        }
        unsafe {
            (*(format as *mut __SlateRecord_formatter_unicode_c_141)).thousands_separators = 95;
        }
        pos = pos.wrapping_add(1);
    }
    let __v24: bool = end.wrapping_sub(pos) != 0 && READ_spec!(pos, kind, data) == 44;
    if __v24 {
        if ((unsafe {
            (*(format as *mut __SlateRecord_formatter_unicode_c_141)).thousands_separators
        }) as u32)
            == 95
        {
            invalid_comma_and_underscore();
            return 0;
        }
    }
    // Parse field precision
    let __v25: bool = end.wrapping_sub(pos) != 0 && READ_spec!(pos, kind, data) == 46;
    if __v25 {
        pos = pos.wrapping_add(1);
        consumed = get_integer(
            format_spec,
            unsafe { &mut (*core::ptr::addr_of_mut!(pos)) },
            end,
            unsafe {
                core::ptr::addr_of_mut!(
                    (*(format as *mut __SlateRecord_formatter_unicode_c_141)).precision
                )
            },
        ) as i64;
        if consumed == (1i32.wrapping_neg() as i64) {
            return 0;
        }
        // Overflow error. Exception already set.
        let __v28: bool = end.wrapping_sub(pos) != 0 && READ_spec!(pos, kind, data) == 44;
        if __v28 {
            if consumed == 0 {
                unsafe {
                    (*(format as *mut __SlateRecord_formatter_unicode_c_141)).precision =
                        1i32.wrapping_neg() as i64;
                }
            }
            unsafe {
                (*(format as *mut __SlateRecord_formatter_unicode_c_141))
                    .frac_thousands_separator = 44;
            }
            pos = pos.wrapping_add(1);
            consumed = consumed.wrapping_add(1);
        }
        let __v33: bool = end.wrapping_sub(pos) != 0 && READ_spec!(pos, kind, data) == 95;
        if __v33 {
            if ((unsafe {
                (*(format as *mut __SlateRecord_formatter_unicode_c_141)).frac_thousands_separator
            }) as u32)
                != 0
            {
                invalid_comma_and_underscore();
                return 0;
            }
            if consumed == 0 {
                unsafe {
                    (*(format as *mut __SlateRecord_formatter_unicode_c_141)).precision =
                        1i32.wrapping_neg() as i64;
                }
            }
            unsafe {
                (*(format as *mut __SlateRecord_formatter_unicode_c_141))
                    .frac_thousands_separator = 95;
            }
            pos = pos.wrapping_add(1);
            consumed = consumed.wrapping_add(1);
        }
        let __v38: bool = end.wrapping_sub(pos) != 0 && READ_spec!(pos, kind, data) == 44;
        if __v38 {
            if ((unsafe {
                (*(format as *mut __SlateRecord_formatter_unicode_c_141)).frac_thousands_separator
            }) as u32)
                == 95
            {
                invalid_comma_and_underscore();
                return 0;
            }
        }
        // Not having a precision or underscore/comma after a dot
        // is an error.
        if consumed == 0 {
            unsafe {
                PyErr_Format(
                    PyExc_ValueError,
                    c"Format specifier missing precision".as_ptr(),
                )
            };
            return 0;
        }
    }
    // Finally, parse the type field.
    if end.wrapping_sub(pos) > 1 {
        // More than one char remains, so this is an invalid format
        // specifier.
        //
        // Create a temporary object that contains the format spec we're
        // operating on.  It's format_spec[start:end] (in Python syntax).
        let actual_format_spec: *mut _object = unsafe {
            PyUnicode_FromKindAndData(
                kind,
                (data as *mut i8).wrapping_offset((kind as i64).wrapping_mul(start) as isize)
                    as *const (),
                end.wrapping_sub(start),
            )
        };
        if !actual_format_spec.is_null() {
            unsafe {
                PyErr_Format(
                    PyExc_ValueError,
                    c"Invalid format specifier '%U' for object of type '%.200s'".as_ptr(),
                    actual_format_spec,
                    (*Py_TYPE!(obj)).tp_name,
                )
            };
            Py_DECREF!(actual_format_spec);
        }
        return 0;
    }
    if end.wrapping_sub(pos) == 1 {
        unsafe {
            (*(format as *mut __SlateRecord_formatter_unicode_c_141)).r#type =
                READ_spec!(pos, kind, data);
        }
        pos = pos.wrapping_add(1);
    }
    // Do as much validating as we can, just by looking at the format
    // specifier.  Do not take into account what type of formatting
    // we're doing (int, float, string).
    if ((unsafe { (*(format as *mut __SlateRecord_formatter_unicode_c_141)).thousands_separators })
        as u32)
        != 0
    {
        // These are allowed. See PEP 378.
        // Every four digits, not every three, in bin/oct/hex.
        // Underscores are allowed in bin/oct/hex. See PEP 515.
        '__slate_break_41: {
            match unsafe { (*(format as *mut __SlateRecord_formatter_unicode_c_141)).r#type } {
                0 | 37 | 69..=71 | 100..=103 => {
                    break '__slate_break_41; // These are allowed. See PEP 378.
                }
                88 | 98 | 111 | 120 => {
                    if ((unsafe {
                        (*(format as *mut __SlateRecord_formatter_unicode_c_141))
                            .thousands_separators
                    }) as u32)
                        == 95
                    {
                        // Every four digits, not every three, in bin/oct/hex.
                        unsafe {
                            (*(format as *mut __SlateRecord_formatter_unicode_c_141))
                                .thousands_separators = 96;
                        }
                    } else {
                        // Underscores are allowed in bin/oct/hex. See PEP 515.
                        invalid_thousands_separator_type(
                            ((unsafe {
                                (*(format as *mut __SlateRecord_formatter_unicode_c_141))
                                    .thousands_separators
                            }) as u32) as i8,
                            unsafe {
                                (*(format as *mut __SlateRecord_formatter_unicode_c_141)).r#type
                            },
                        );
                        return 0;
                    }
                }
                _ => {
                    invalid_thousands_separator_type(
                        ((unsafe {
                            (*(format as *mut __SlateRecord_formatter_unicode_c_141))
                                .thousands_separators
                        }) as u32) as i8,
                        unsafe { (*(format as *mut __SlateRecord_formatter_unicode_c_141)).r#type },
                    );
                    return 0;
                }
            }
        }
    }
    if (unsafe { (*(format as *mut __SlateRecord_formatter_unicode_c_141)).r#type }) == 110
        && ((unsafe {
            (*(format as *mut __SlateRecord_formatter_unicode_c_141)).frac_thousands_separator
        }) as u32)
            != 0
    {
        invalid_thousands_separator_type(
            ((unsafe {
                (*(format as *mut __SlateRecord_formatter_unicode_c_141)).frac_thousands_separator
            }) as u32) as i8,
            unsafe { (*(format as *mut __SlateRecord_formatter_unicode_c_141)).r#type },
        );
        return 0;
    }
    1
}

// Calculate the padding needed.
fn calc_padding(
    nchars: i64,
    width: i64,
    align: u32,
    n_lpadding: &mut i64,
    n_rpadding: &mut i64,
    n_total: &mut i64,
) {
    if width >= 0 {
        if nchars > width {
            unsafe {
                *(n_total as *mut i64) = nchars;
            }
        } else {
            unsafe {
                *(n_total as *mut i64) = width;
            }
        }
    } else {
        // not specified, use all of the chars and no more
        unsafe {
            *(n_total as *mut i64) = nchars;
        }
    }
    // Figure out how much leading space we need, based on the
    // aligning
    if align == 62 {
        unsafe {
            *(n_lpadding as *mut i64) = unsafe { *(n_total as *mut i64) }.wrapping_sub(nchars);
        }
    } else {
        if align == 94 {
            unsafe {
                *(n_lpadding as *mut i64) =
                    unsafe { *(n_total as *mut i64) }.wrapping_sub(nchars) / 2;
            }
        } else {
            if align == 60 || align == 61 {
                unsafe {
                    *(n_lpadding as *mut i64) = 0;
                }
            } else {
                // We should never have an unspecified alignment.
                Py_UNREACHABLE!();
            }
        }
    }
    unsafe {
        *(n_rpadding as *mut i64) = unsafe { *(n_total as *mut i64) }
            .wrapping_sub(nchars)
            .wrapping_sub(*(n_lpadding as *mut i64));
    }
}

/// Do the padding, and return a pointer to where the caller-supplied
/// content goes.
fn fill_padding(
    writer: *mut __SlateRecord_unicodeobject_h_539,
    nchars: i64,
    fill_char: u32,
    n_lpadding: i64,
    n_rpadding: i64,
) -> i32 {
    let mut pos: i64 = 0;
    // Pad on left.
    if n_lpadding != 0 {
        pos = unsafe { (*writer).pos };
        unsafe { _PyUnicode_FastFill((*writer).buffer, pos, n_lpadding, fill_char) };
    }
    // Pad on right.
    if n_rpadding != 0 {
        pos = unsafe { (*writer).pos }
            .wrapping_add(nchars)
            .wrapping_add(n_lpadding);
        unsafe { _PyUnicode_FastFill((*writer).buffer, pos, n_rpadding, fill_char) };
    }
    // Pointer to the user content.
    let __v1: i64 = unsafe { (*writer).pos };
    unsafe {
        (*writer).pos = __v1.wrapping_add(n_lpadding);
    }
    0
}

// common routines for numeric formatting
/// Locale info needed for formatting integers and the part of floats
/// before and including the decimal. Note that locales only support
/// 8-bit chars, not unicode.
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_formatter_unicode_c_447 {
    decimal_point: *mut _object,
    thousands_sep: *mut _object,
    frac_thousands_sep: *mut _object,
    grouping: *const i8,
    grouping_buffer: *mut i8,
}

/// describes the layout for an integer, see the comment in
/// calc_number_widths() for details
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_formatter_unicode_c_475 {
    n_lpadding: i64,
    n_prefix: i64,
    n_spadding: i64,
    n_rpadding: i64,
    sign: i8,
    /// number of digits needed for sign (0/1)
    n_sign: i64,
    /// Space taken up by the digits, including
    /// any grouping chars.
    n_grouped_digits: i64,
    /// 0 if only an integer
    n_decimal: i64,
    /// Digits in decimal and/or exponent part,
    /// excluding the decimal itself, if
    /// present.
    n_remainder: i64,
    n_frac: i64,
    n_grouped_frac_digits: i64,
    /// These 2 are not the widths of fields, but are needed by
    /// STRINGLIB_GROUPING.
    /// The number of digits before a decimal
    /// or exponent.
    n_digits: i64,
    /// The min_width we used when we computed
    /// the n_grouped_digits width.
    n_min_width: i64,
}

/// Given a number of the form:
/// digits[remainder]
/// where ptr points to the start and end points to the end, find where
///  the integer part ends. This could be a decimal, an exponent, both,
///  or neither.
/// If a decimal point is present, set *has_decimal and increment
///  remainder beyond it.
/// Results are undefined (but shouldn't crash) for improperly
///  formatted strings.
fn parse_number(
    s: *mut _object,
    mut pos: i64,
    end: i64,
    n_remainder: &mut i64,
    n_frac: &mut i64,
    has_decimal: &mut i32,
) {
    let mut frac: i64 = 0;
    let kind: i32 = PyUnicode_KIND!(s);
    let data: *const () = PyUnicode_DATA!(s) as *const ();
    loop {
        let __v2: bool = pos < end && Py_ISDIGIT__2!(PyUnicode_READ!(kind, data, pos)) != 0;
        if !__v2 {
            break;
        }
        pos = pos.wrapping_add(1);
    }
    frac = pos;
    // Does remainder start with a decimal point?
    let __v4: bool = pos < end && PyUnicode_READ!(kind, data, frac) == 46;
    unsafe {
        *(has_decimal as *mut i32) = __v4 as i32;
    }
    // Skip the decimal point.
    if (unsafe { *(has_decimal as *mut i32) }) != 0 {
        frac = frac.wrapping_add(1);
        pos = pos.wrapping_add(1);
    }
    loop {
        let __v11: bool = pos < end && Py_ISDIGIT__2!(PyUnicode_READ!(kind, data, pos)) != 0;
        if !__v11 {
            break;
        }
        pos = pos.wrapping_add(1);
    }
    unsafe {
        *(n_frac as *mut i64) = pos.wrapping_sub(frac);
    }
    unsafe {
        *(n_remainder as *mut i64) = end.wrapping_sub(pos);
    }
}

/// not all fields of format are used.  for example, precision is
/// unused.  should this take discrete params in order to be more clear
/// about what it does?  or is passing a single format parameter easier
/// and more efficient enough to justify a little obfuscation?
/// Return -1 on error.
fn calc_number_widths(
    spec: &mut __SlateRecord_formatter_unicode_c_475,
    n_prefix: i64,
    sign_char: u32,
    n_start: i64,
    n_end: i64,
    n_remainder: i64,
    n_frac: i64,
    has_decimal: i32,
    locale: *const __SlateRecord_formatter_unicode_c_447,
    format: *const __SlateRecord_formatter_unicode_c_141,
    maxchar: *mut u32,
) -> i64 {
    let mut n_non_digit_non_padding: i64 = 0;
    let mut n_padding: i64 = 0;
    unsafe {
        (*(spec as *mut __SlateRecord_formatter_unicode_c_475)).n_digits = n_end
            .wrapping_sub(n_start)
            .wrapping_sub(n_frac)
            .wrapping_sub(n_remainder)
            .wrapping_sub((if has_decimal != 0 { 1i32 } else { 0i32 }) as i64);
    }
    unsafe {
        (*(spec as *mut __SlateRecord_formatter_unicode_c_475)).n_lpadding = 0;
    }
    unsafe {
        (*(spec as *mut __SlateRecord_formatter_unicode_c_475)).n_prefix = n_prefix;
    }
    let __v0: i64 = if has_decimal != 0 {
        PyUnicode_GET_LENGTH!(unsafe { (*locale).decimal_point })
    } else {
        0
    };
    unsafe {
        (*(spec as *mut __SlateRecord_formatter_unicode_c_475)).n_decimal = __v0;
    }
    unsafe {
        (*(spec as *mut __SlateRecord_formatter_unicode_c_475)).n_remainder = n_remainder;
    }
    unsafe {
        (*(spec as *mut __SlateRecord_formatter_unicode_c_475)).n_frac = n_frac;
    }
    unsafe {
        (*(spec as *mut __SlateRecord_formatter_unicode_c_475)).n_spadding = 0;
    }
    unsafe {
        (*(spec as *mut __SlateRecord_formatter_unicode_c_475)).n_rpadding = 0;
    }
    unsafe {
        (*(spec as *mut __SlateRecord_formatter_unicode_c_475)).sign = 0;
    }
    unsafe {
        (*(spec as *mut __SlateRecord_formatter_unicode_c_475)).n_sign = 0;
    }
    // the output will look like:
    // |                                                                                         |
    // | <lpadding> <sign> <prefix> <spadding> <grouped_digits> <decimal> <remainder> <rpadding> |
    // |                                                                                         |
    //
    // sign is computed from format->sign and the actual
    // sign of the number
    //
    // prefix is given (it's for the '0x' prefix)
    //
    // digits is already known
    //
    // the total width is either given, or computed from the
    // actual digits
    //
    // only one of lpadding, spadding, and rpadding can be non-zero,
    // and it's calculated from the width and other fields
    // compute the various parts we're going to write
    match unsafe { (*format).sign } {
        43 => {
            unsafe {
                (*(spec as *mut __SlateRecord_formatter_unicode_c_475)).n_sign = 1;
            }
            // always put a + or -
            unsafe {
                (*(spec as *mut __SlateRecord_formatter_unicode_c_475)).sign =
                    (if sign_char == 45 { 45i32 } else { 43i32 }) as i8;
            }
        }
        32 => {
            unsafe {
                (*(spec as *mut __SlateRecord_formatter_unicode_c_475)).n_sign = 1;
            }
            unsafe {
                (*(spec as *mut __SlateRecord_formatter_unicode_c_475)).sign =
                    (if sign_char == 45 { 45i32 } else { 32i32 }) as i8;
            }
        }
        _ => {
            if sign_char == 45 {
                unsafe {
                    (*(spec as *mut __SlateRecord_formatter_unicode_c_475)).n_sign = 1;
                }
                unsafe {
                    (*(spec as *mut __SlateRecord_formatter_unicode_c_475)).sign = 45;
                }
            }
            // Not specified, or the default (-)
        }
    }
    if (unsafe { (*(spec as *mut __SlateRecord_formatter_unicode_c_475)).n_frac }) == 0 {
        unsafe {
            (*(spec as *mut __SlateRecord_formatter_unicode_c_475)).n_grouped_frac_digits = 0;
        }
    } else {
        let mut grouping_maxchar: u32 = 0;
        unsafe {
            (*(spec as *mut __SlateRecord_formatter_unicode_c_475)).n_grouped_frac_digits =
                _PyUnicode_InsertThousandsGrouping(
                    core::ptr::null_mut::<__SlateRecord_unicodeobject_h_539>(),
                    0i64,
                    core::ptr::null_mut::<_object>(),
                    0i64,
                    (*(spec as *mut __SlateRecord_formatter_unicode_c_475)).n_frac,
                    (*(spec as *mut __SlateRecord_formatter_unicode_c_475)).n_frac,
                    (*locale).grouping,
                    (*locale).frac_thousands_sep,
                    core::ptr::addr_of_mut!(grouping_maxchar),
                    1i32,
                );
        }
        if (unsafe {
            (*(spec as *mut __SlateRecord_formatter_unicode_c_475)).n_grouped_frac_digits
        }) == (1i32.wrapping_neg() as i64)
        {
            return 1i32.wrapping_neg() as i64;
        }
        unsafe {
            *maxchar = Py_MAX!(unsafe { *maxchar }, grouping_maxchar);
        }
    }
    // The number of chars used for non-digits and non-padding.
    n_non_digit_non_padding =
        unsafe { (*(spec as *mut __SlateRecord_formatter_unicode_c_475)).n_sign }
            .wrapping_add(unsafe {
                (*(spec as *mut __SlateRecord_formatter_unicode_c_475)).n_prefix
            })
            .wrapping_add(unsafe {
                (*(spec as *mut __SlateRecord_formatter_unicode_c_475)).n_decimal
            })
            .wrapping_add(unsafe { (*(spec as *mut __SlateRecord_formatter_unicode_c_475)).n_frac })
            .wrapping_add(unsafe {
                (*(spec as *mut __SlateRecord_formatter_unicode_c_475)).n_remainder
            });
    // min_width can go negative, that's okay. format->width == -1 means
    // we don't care.
    if (unsafe { (*format).fill_char }) == 48 && (unsafe { (*format).align }) == 61 {
        unsafe {
            (*(spec as *mut __SlateRecord_formatter_unicode_c_475)).n_min_width =
                unsafe { (*format).width }
                    .wrapping_sub(n_non_digit_non_padding)
                    .wrapping_add((*(spec as *mut __SlateRecord_formatter_unicode_c_475)).n_frac)
                    .wrapping_sub(
                        (*(spec as *mut __SlateRecord_formatter_unicode_c_475))
                            .n_grouped_frac_digits,
                    );
        }
    } else {
        unsafe {
            (*(spec as *mut __SlateRecord_formatter_unicode_c_475)).n_min_width = 0;
        }
    }
    if (unsafe { (*(spec as *mut __SlateRecord_formatter_unicode_c_475)).n_digits }) == 0 {
        unsafe {
            (*(spec as *mut __SlateRecord_formatter_unicode_c_475)).n_grouped_digits = 0;
        }
    } else {
        let mut grouping_maxchar: u32 = 0;
        unsafe {
            (*(spec as *mut __SlateRecord_formatter_unicode_c_475)).n_grouped_digits =
                _PyUnicode_InsertThousandsGrouping(
                    core::ptr::null_mut::<__SlateRecord_unicodeobject_h_539>(),
                    0i64,
                    core::ptr::null_mut::<_object>(),
                    0i64,
                    (*(spec as *mut __SlateRecord_formatter_unicode_c_475)).n_digits,
                    (*(spec as *mut __SlateRecord_formatter_unicode_c_475)).n_min_width,
                    (*locale).grouping,
                    (*locale).thousands_sep,
                    core::ptr::addr_of_mut!(grouping_maxchar),
                    0i32,
                );
        }
        if (unsafe { (*(spec as *mut __SlateRecord_formatter_unicode_c_475)).n_grouped_digits })
            == (1i32.wrapping_neg() as i64)
        {
            return 1i32.wrapping_neg() as i64;
        }
        unsafe {
            *maxchar = Py_MAX!(unsafe { *maxchar }, grouping_maxchar);
        }
    }
    // This case only occurs when using 'c' formatting, we need
    // to special case it because the grouping code always wants
    // to have at least one character.
    // Given the desired width and the total of digit and non-digit
    // space we consume, see if we need any padding. format->width can
    // be negative (meaning no padding), but this code still works in
    // that case.
    n_padding = unsafe { (*format).width }.wrapping_sub(
        n_non_digit_non_padding
            .wrapping_add(unsafe {
                (*(spec as *mut __SlateRecord_formatter_unicode_c_475)).n_grouped_digits
            })
            .wrapping_add(unsafe {
                (*(spec as *mut __SlateRecord_formatter_unicode_c_475)).n_grouped_frac_digits
            })
            .wrapping_sub(unsafe {
                (*(spec as *mut __SlateRecord_formatter_unicode_c_475)).n_frac
            }),
    );
    if n_padding > 0 {
        // Some padding is needed. Determine if it's left, space, or right.
        match unsafe { (*format).align } {
            60 => unsafe {
                (*(spec as *mut __SlateRecord_formatter_unicode_c_475)).n_rpadding = n_padding;
            },
            94 => {
                unsafe {
                    (*(spec as *mut __SlateRecord_formatter_unicode_c_475)).n_lpadding =
                        n_padding / 2;
                }
                unsafe {
                    (*(spec as *mut __SlateRecord_formatter_unicode_c_475)).n_rpadding = n_padding
                        .wrapping_sub(
                            (*(spec as *mut __SlateRecord_formatter_unicode_c_475)).n_lpadding,
                        );
                }
            }
            61 => unsafe {
                (*(spec as *mut __SlateRecord_formatter_unicode_c_475)).n_spadding = n_padding;
            },
            62 => unsafe {
                (*(spec as *mut __SlateRecord_formatter_unicode_c_475)).n_lpadding = n_padding;
            },
            _ => {
                Py_UNREACHABLE!(); // Shouldn't get here
            }
        }
    }
    if (unsafe { (*(spec as *mut __SlateRecord_formatter_unicode_c_475)).n_lpadding }) != 0
        || (unsafe { (*(spec as *mut __SlateRecord_formatter_unicode_c_475)).n_spadding }) != 0
        || (unsafe { (*(spec as *mut __SlateRecord_formatter_unicode_c_475)).n_rpadding }) != 0
    {
        unsafe {
            *maxchar = Py_MAX!(unsafe { *maxchar }, unsafe { (*format).fill_char });
        }
    }
    if (unsafe { (*(spec as *mut __SlateRecord_formatter_unicode_c_475)).n_decimal }) != 0 {
        unsafe {
            *maxchar = Py_MAX__6!(
                unsafe { *maxchar },
                PyUnicode_MAX_CHAR_VALUE((*locale).decimal_point)
            );
        }
    }
    unsafe { (*(spec as *mut __SlateRecord_formatter_unicode_c_475)).n_lpadding }
        .wrapping_add(unsafe { (*(spec as *mut __SlateRecord_formatter_unicode_c_475)).n_sign })
        .wrapping_add(unsafe { (*(spec as *mut __SlateRecord_formatter_unicode_c_475)).n_prefix })
        .wrapping_add(unsafe { (*(spec as *mut __SlateRecord_formatter_unicode_c_475)).n_spadding })
        .wrapping_add(unsafe {
            (*(spec as *mut __SlateRecord_formatter_unicode_c_475)).n_grouped_digits
        })
        .wrapping_add(unsafe { (*(spec as *mut __SlateRecord_formatter_unicode_c_475)).n_decimal })
        .wrapping_add(unsafe {
            (*(spec as *mut __SlateRecord_formatter_unicode_c_475)).n_grouped_frac_digits
        })
        .wrapping_add(unsafe {
            (*(spec as *mut __SlateRecord_formatter_unicode_c_475)).n_remainder
        })
        .wrapping_add(unsafe { (*(spec as *mut __SlateRecord_formatter_unicode_c_475)).n_rpadding })
}

/// Fill in the digit parts of a number's string representation,
/// as determined in calc_number_widths().
/// Return -1 on error, or 0 on success.
fn fill_number(
    writer: *mut __SlateRecord_unicodeobject_h_539,
    spec: *const __SlateRecord_formatter_unicode_c_475,
    digits: *mut _object,
    mut d_pos: i64,
    prefix: *mut _object,
    p_start: i64,
    fill_char: u32,
    locale: &__SlateRecord_formatter_unicode_c_447,
    toupper: i32,
) -> i32 {
    // Used to keep track of digits, decimal, and remainder.
    let kind: i32 = unsafe { (*writer).kind };
    let data: *const () = (unsafe { (*writer).data }) as *const ();
    let mut r: i64 = 0;
    if (unsafe { (*spec).n_lpadding }) != 0 {
        unsafe {
            _PyUnicode_FastFill(
                (*writer).buffer,
                (*writer).pos,
                (*spec).n_lpadding,
                fill_char,
            )
        };
        let __v1: i64 = unsafe { (*writer).pos };
        unsafe {
            (*writer).pos = __v1.wrapping_add((*spec).n_lpadding);
        }
    }
    if (unsafe { (*spec).n_sign }) == 1 {
        PyUnicode_WRITE__2!(kind, data, unsafe { (*writer).pos }, unsafe {
            (*spec).sign
        });
        let __v4: i64 = unsafe { (*writer).pos };
        unsafe {
            (*writer).pos = __v4.wrapping_add(1);
        }
    }
    if (unsafe { (*spec).n_prefix }) != 0 {
        unsafe {
            _PyUnicode_FastCopyCharacters(
                (*writer).buffer,
                (*writer).pos,
                prefix,
                p_start,
                (*spec).n_prefix,
            )
        };
        if toupper != 0 {
            let mut t: i64 = 0;
            while t < unsafe { (*spec).n_prefix } {
                let mut c: u32 =
                    PyUnicode_READ!(kind, data, unsafe { (*writer).pos }.wrapping_add(t));
                c = Py_TOUPPER!(c) as u32;
                PyUnicode_WRITE__2!(kind, data, unsafe { (*writer).pos }.wrapping_add(t), c);
                t = t.wrapping_add(1);
            }
        }
        let __v10: i64 = unsafe { (*writer).pos };
        unsafe {
            (*writer).pos = __v10.wrapping_add((*spec).n_prefix);
        }
    }
    if (unsafe { (*spec).n_spadding }) != 0 {
        unsafe {
            _PyUnicode_FastFill(
                (*writer).buffer,
                (*writer).pos,
                (*spec).n_spadding,
                fill_char,
            )
        };
        let __v13: i64 = unsafe { (*writer).pos };
        unsafe {
            (*writer).pos = __v13.wrapping_add((*spec).n_spadding);
        }
    }
    // Only for type 'c' special case, it has no digits.
    if (unsafe { (*spec).n_digits }) != 0 {
        // Fill the digits with InsertThousandsGrouping.
        r = unsafe {
            _PyUnicode_InsertThousandsGrouping(
                writer,
                (*spec).n_grouped_digits,
                digits,
                d_pos,
                (*spec).n_digits,
                (*spec).n_min_width,
                (*(locale as *const __SlateRecord_formatter_unicode_c_447)).grouping,
                (*(locale as *const __SlateRecord_formatter_unicode_c_447)).thousands_sep,
                core::ptr::null_mut::<u32>(),
                0i32,
            )
        };
        if r == (1i32.wrapping_neg() as i64) {
            return 1i32.wrapping_neg();
        }
        d_pos = d_pos.wrapping_add(unsafe { (*spec).n_digits });
    }
    if toupper != 0 {
        let mut t: i64 = 0;
        while t < unsafe { (*spec).n_grouped_digits } {
            let mut c: u32 = PyUnicode_READ!(kind, data, unsafe { (*writer).pos }.wrapping_add(t));
            c = Py_TOUPPER!(c) as u32;
            if c > 127 {
                unsafe { PyErr_SetString(PyExc_SystemError, c"non-ascii grouped digit".as_ptr()) };
                return 1i32.wrapping_neg();
            }
            PyUnicode_WRITE__2!(kind, data, unsafe { (*writer).pos }.wrapping_add(t), c);
            t = t.wrapping_add(1);
        }
    }
    let __v21: i64 = unsafe { (*writer).pos };
    unsafe {
        (*writer).pos = __v21.wrapping_add((*spec).n_grouped_digits);
    }
    if (unsafe { (*spec).n_decimal }) != 0 {
        unsafe {
            _PyUnicode_FastCopyCharacters(
                (*writer).buffer,
                (*writer).pos,
                (*(locale as *const __SlateRecord_formatter_unicode_c_447)).decimal_point,
                0i64,
                (*spec).n_decimal,
            )
        };
        let __v24: i64 = unsafe { (*writer).pos };
        unsafe {
            (*writer).pos = __v24.wrapping_add((*spec).n_decimal);
        }
        d_pos = d_pos.wrapping_add(1);
    }
    if (unsafe { (*spec).n_frac }) != 0 {
        r = unsafe {
            _PyUnicode_InsertThousandsGrouping(
                writer,
                (*spec).n_grouped_frac_digits,
                digits,
                d_pos,
                (*spec).n_frac,
                (*spec).n_frac,
                (*(locale as *const __SlateRecord_formatter_unicode_c_447)).grouping,
                (*(locale as *const __SlateRecord_formatter_unicode_c_447)).frac_thousands_sep,
                core::ptr::null_mut::<u32>(),
                1i32,
            )
        };
        if r == (1i32.wrapping_neg() as i64) {
            return 1i32.wrapping_neg();
        }
        d_pos = d_pos.wrapping_add(unsafe { (*spec).n_frac });
        let __v31: i64 = unsafe { (*writer).pos };
        unsafe {
            (*writer).pos = __v31.wrapping_add((*spec).n_grouped_frac_digits);
        }
    }
    if (unsafe { (*spec).n_remainder }) != 0 {
        unsafe {
            _PyUnicode_FastCopyCharacters(
                (*writer).buffer,
                (*writer).pos,
                digits,
                d_pos,
                (*spec).n_remainder,
            )
        };
        let __v34: i64 = unsafe { (*writer).pos };
        unsafe {
            (*writer).pos = __v34.wrapping_add((*spec).n_remainder);
        }
        // d_pos += spec->n_remainder;
    }
    if (unsafe { (*spec).n_rpadding }) != 0 {
        unsafe {
            _PyUnicode_FastFill(
                (*writer).buffer,
                (*writer).pos,
                (*spec).n_rpadding,
                fill_char,
            )
        };
        let __v37: i64 = unsafe { (*writer).pos };
        unsafe {
            (*writer).pos = __v37.wrapping_add((*spec).n_rpadding);
        }
    }
    0
}

static no_grouping: [i8; 1] = [127];

/// Find the decimal point character(s?), thousands_separator(s?), and
/// grouping description, either for the current locale if type is
/// LT_CURRENT_LOCALE, a hard-coded locale if LT_DEFAULT_LOCALE or
/// LT_UNDERSCORE_LOCALE/LT_UNDER_FOUR_LOCALE, or none if LT_NO_LOCALE.
fn get_locale_info(
    r#type: u32,
    frac_type: u32,
    locale_info: &mut __SlateRecord_formatter_unicode_c_447,
) -> i32 {
    match r#type as u32 {
        97 => {
            let lc: *mut slate_support::lconv =
                unsafe { localeconv() as *mut slate_support::lconv };
            if (unsafe {
                _Py_GetLocaleconvNumeric(
                    lc,
                    core::ptr::addr_of_mut!(
                        (*(locale_info as *mut __SlateRecord_formatter_unicode_c_447))
                            .decimal_point
                    ),
                    core::ptr::addr_of_mut!(
                        (*(locale_info as *mut __SlateRecord_formatter_unicode_c_447))
                            .thousands_sep
                    ),
                )
            }) < 0
            {
                return 1i32.wrapping_neg();
            }
            // localeconv() grouping can become a dangling pointer or point
            // to a different string if another thread calls localeconv() during
            // the string formatting. Copy the string to avoid this risk.
            unsafe {
                (*(locale_info as *mut __SlateRecord_formatter_unicode_c_447)).grouping_buffer =
                    _PyMem_Strdup((*lc).grouping as *const i8);
            }
            if unsafe {
                (*(locale_info as *mut __SlateRecord_formatter_unicode_c_447)).grouping_buffer
            }
            .is_null()
            {
                unsafe { PyErr_NoMemory() };
                return 1i32.wrapping_neg();
            }
            unsafe {
                (*(locale_info as *mut __SlateRecord_formatter_unicode_c_447)).grouping =
                    (*(locale_info as *mut __SlateRecord_formatter_unicode_c_447)).grouping_buffer
                        as *const i8;
            }
        }
        44 | 95 | 96 => {
            unsafe {
                (*(locale_info as *mut __SlateRecord_formatter_unicode_c_447)).decimal_point =
                    PyUnicode_FromOrdinal(46i32);
            }
            unsafe {
                (*(locale_info as *mut __SlateRecord_formatter_unicode_c_447)).thousands_sep =
                    PyUnicode_FromOrdinal(if (r#type as u32) == 44 { 44i32 } else { 95i32 });
            }
            if unsafe {
                (*(locale_info as *mut __SlateRecord_formatter_unicode_c_447)).decimal_point
            }
            .is_null()
                || unsafe {
                    (*(locale_info as *mut __SlateRecord_formatter_unicode_c_447)).thousands_sep
                }
                .is_null()
            {
                return 1i32.wrapping_neg();
            }
            if (r#type as u32) != 96 {
                unsafe {
                    (*(locale_info as *mut __SlateRecord_formatter_unicode_c_447)).grouping =
                        c"\x03".as_ptr();
                }
            } else {
                unsafe {
                    (*(locale_info as *mut __SlateRecord_formatter_unicode_c_447)).grouping =
                        c"\x04".as_ptr();
                }
            }
            // Group every 3 characters.  The
            // (implicit) trailing 0 means repeat
            // infinitely.
            // Bin/oct/hex group every four.
        }
        0 => {
            unsafe {
                (*(locale_info as *mut __SlateRecord_formatter_unicode_c_447)).decimal_point =
                    PyUnicode_FromOrdinal(46i32);
            }
            unsafe {
                (*(locale_info as *mut __SlateRecord_formatter_unicode_c_447)).thousands_sep =
                    Py_GetConstant(7u32);
            }
            if unsafe {
                (*(locale_info as *mut __SlateRecord_formatter_unicode_c_447)).decimal_point
            }
            .is_null()
                || unsafe {
                    (*(locale_info as *mut __SlateRecord_formatter_unicode_c_447)).thousands_sep
                }
                .is_null()
            {
                return 1i32.wrapping_neg();
            }
            unsafe {
                (*(locale_info as *mut __SlateRecord_formatter_unicode_c_447)).grouping =
                    core::ptr::addr_of!(no_grouping) as *const i8;
            }
        }
        _ => {}
    }
    if (frac_type as u32) != 0 {
        unsafe {
            (*(locale_info as *mut __SlateRecord_formatter_unicode_c_447)).frac_thousands_sep =
                PyUnicode_FromOrdinal(if (frac_type as u32) == 44 {
                    44i32
                } else {
                    95i32
                });
        }
        if unsafe {
            (*(locale_info as *mut __SlateRecord_formatter_unicode_c_447)).frac_thousands_sep
        }
        .is_null()
        {
            return 1i32.wrapping_neg();
        }
        if (unsafe { (*(locale_info as *mut __SlateRecord_formatter_unicode_c_447)).grouping })
            == unsafe { core::ptr::addr_of!(no_grouping) as *const i8 }
        {
            unsafe {
                (*(locale_info as *mut __SlateRecord_formatter_unicode_c_447)).grouping =
                    c"\x03".as_ptr();
            }
        }
    } else {
        unsafe {
            (*(locale_info as *mut __SlateRecord_formatter_unicode_c_447)).frac_thousands_sep =
                Py_GetConstant(7u32);
        }
    }
    0
}

fn free_locale_info(locale_info: *mut __SlateRecord_formatter_unicode_c_447) {
    Py_XDECREF!(unsafe { (*locale_info).decimal_point });
    Py_XDECREF!(unsafe { (*locale_info).thousands_sep });
    Py_XDECREF!(unsafe { (*locale_info).frac_thousands_sep });
    unsafe { PyMem_Free((*locale_info).grouping_buffer as *mut ()) };
}

// string formatting
fn format_string_internal(
    value: *mut _object,
    format: *const __SlateRecord_formatter_unicode_c_141,
    writer: *mut __SlateRecord_unicodeobject_h_539,
) -> i32 {
    let mut maxchar: u32 = unsafe { core::mem::zeroed() };
    let mut result: i32 = unsafe { core::mem::zeroed() };
    let mut len: i64 = unsafe { core::mem::zeroed() };
    let mut total: i64 = unsafe { core::mem::zeroed() };
    let mut rpad: i64 = unsafe { core::mem::zeroed() };
    let mut lpad: i64 = unsafe { core::mem::zeroed() };
    unsafe {
        result = 1i32.wrapping_neg();
        len = PyUnicode_GET_LENGTH!(value);
        // sign is not allowed on strings
        if (*format).sign != 0 {
            if (*format).sign == 32 {
                PyErr_SetString(
                    PyExc_ValueError,
                    c"Space not allowed in string format specifier".as_ptr(),
                );
            } else {
                PyErr_SetString(
                    PyExc_ValueError,
                    c"Sign not allowed in string format specifier".as_ptr(),
                );
            }
        } else {
            // negative 0 coercion is not allowed on strings
            if (*format).no_neg_0 != 0 {
                PyErr_SetString(
                    PyExc_ValueError,
                    c"Negative zero coercion (z) not allowed in string format specifier".as_ptr(),
                );
            } else {
                // alternate is not allowed on strings
                if (*format).alternate != 0 {
                    PyErr_SetString(
                        PyExc_ValueError,
                        c"Alternate form (#) not allowed in string format specifier".as_ptr(),
                    );
                } else {
                    // '=' alignment not allowed on strings
                    if (*format).align == 61 {
                        PyErr_SetString(
                            PyExc_ValueError,
                            c"'=' alignment not allowed in string format specifier".as_ptr(),
                        );
                    } else {
                        if ((*format).width == (1i32.wrapping_neg() as i64)
                            || (*format).width <= len)
                            && ((*format).precision == (1i32.wrapping_neg() as i64)
                                || (*format).precision >= len)
                        {
                            // Fast path
                            return _PyUnicodeWriter_WriteStr(writer, value);
                        } else {
                            // if precision is specified, output no more that format.precision
                            // characters
                            if (*format).precision >= 0 && len >= (*format).precision {
                                len = (*format).precision;
                            }
                            calc_padding(
                                len,
                                (*format).width,
                                (*format).align,
                                &mut (*core::ptr::addr_of_mut!(lpad)),
                                &mut (*core::ptr::addr_of_mut!(rpad)),
                                &mut (*core::ptr::addr_of_mut!(total)),
                            );
                            maxchar = (*writer).maxchar;
                            if lpad != 0 || rpad != 0 {
                                maxchar = Py_MAX!(maxchar, unsafe { (*format).fill_char });
                            }
                            if PyUnicode_MAX_CHAR_VALUE!(value) > maxchar {
                                let valmaxchar: u32 = _PyUnicode_FindMaxChar(value, 0i64, len);
                                maxchar = Py_MAX!(maxchar, valmaxchar);
                            }
                            // allocate the resulting string
                            if _PyUnicodeWriter_Prepare__3!(writer, total, maxchar)
                                != 1i32.wrapping_neg()
                            {
                                // Write into that space. First the padding.
                                result = fill_padding(writer, len, (*format).fill_char, lpad, rpad);
                                if result != 1i32.wrapping_neg() {
                                    // Then the source string.
                                    if len != 0 {
                                        _PyUnicode_FastCopyCharacters(
                                            (*writer).buffer,
                                            (*writer).pos,
                                            value,
                                            0i64,
                                            len,
                                        );
                                    }
                                    let __v1: i64 = (*writer).pos;
                                    (*writer).pos = __v1.wrapping_add(len.wrapping_add(rpad));
                                    result = 0;
                                }
                            }
                        }
                    }
                }
            }
        }
        return result;
    }
}

// long formatting
fn format_long_internal(
    value: *mut _object,
    format: *const __SlateRecord_formatter_unicode_c_141,
    writer: *mut __SlateRecord_unicodeobject_h_539,
) -> i32 {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut __v9: bool = unsafe { core::mem::zeroed() };
    // Number of characters added by
    // PyNumber_ToBase that we want to
    // skip over.
    let mut leading_chars_to_skip: i32 = unsafe { core::mem::zeroed() };
    let mut base: i32 = unsafe { core::mem::zeroed() };
    // Locale settings, either from the actual locale or
    // from a hard-code pseudo-locale
    let mut locale: core::mem::MaybeUninit<__SlateRecord_formatter_unicode_c_447> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_12: *mut __SlateRecord_formatter_unicode_c_447 =
        core::ptr::addr_of_mut!(locale) as *mut __SlateRecord_formatter_unicode_c_447;
    let mut x: i64 = unsafe { core::mem::zeroed() };
    let mut spec: core::mem::MaybeUninit<__SlateRecord_formatter_unicode_c_475> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_14: *mut __SlateRecord_formatter_unicode_c_475 =
        core::ptr::addr_of_mut!(spec) as *mut __SlateRecord_formatter_unicode_c_475;
    let mut prefix: i64 = unsafe { core::mem::zeroed() };
    let mut n_total: i64 = unsafe { core::mem::zeroed() };
    // Count of prefix chars, (e.g., '0x')
    let mut n_prefix: i64 = unsafe { core::mem::zeroed() };
    // Used only for 'c' formatting, which
    // produces non-digits
    let mut n_remainder: i64 = unsafe { core::mem::zeroed() };
    // count of digits need from the computed
    // string
    let mut n_digits: i64 = unsafe { core::mem::zeroed() };
    let mut sign_char: u32 = unsafe { core::mem::zeroed() };
    let mut inumeric_chars: i64 = unsafe { core::mem::zeroed() };
    let mut tmp: *mut _object = unsafe { core::mem::zeroed() };
    let mut maxchar: u32 = unsafe { core::mem::zeroed() };
    let mut result: i32 = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_0: {
            result = 1i32.wrapping_neg();
            maxchar = 127;
            tmp = core::ptr::null_mut::<_object>();
            sign_char = 0;
            n_remainder = 0;
            n_prefix = 0;
            prefix = 0;
            core::ptr::write(__slate_slot_12, {
                let mut __t1: __SlateRecord_formatter_unicode_c_447 = core::mem::zeroed();
                __t1.decimal_point = core::ptr::null_mut::<_object>();
                __t1.thousands_sep = core::ptr::null_mut::<_object>();
                __t1.frac_thousands_sep = core::ptr::null_mut::<_object>();
                __t1.grouping = core::ptr::null::<i8>();
                __t1
            });
            // no precision allowed on integers
            if (*format).precision != (1i32.wrapping_neg() as i64) {
                PyErr_SetString(
                    PyExc_ValueError,
                    c"Precision not allowed in integer format specifier".as_ptr(),
                );
            } else {
                // no negative zero coercion on integers
                if (*format).no_neg_0 != 0 {
                    PyErr_SetString(
                        PyExc_ValueError,
                        c"Negative zero coercion (z) not allowed in integer format specifier"
                            .as_ptr(),
                    );
                } else {
                    // special case for character formatting
                    if (*format).r#type == 99 {
                        // error to specify a sign
                        if (*format).sign != 0 {
                            PyErr_SetString(
                                PyExc_ValueError,
                                c"Sign not allowed with integer format specifier 'c'".as_ptr(),
                            );
                            break '__join_0;
                        } else {
                            // error to request alternate format
                            if (*format).alternate != 0 {
                                PyErr_SetString(PyExc_ValueError, c"Alternate form (#) not allowed with integer format specifier 'c'".as_ptr());
                                break '__join_0;
                            } else {
                                // taken from unicodeobject.c formatchar()
                                //
                                // Integer input truncated to a character
                                x = PyLong_AsLong(value);
                                if x == (1i32.wrapping_neg() as i64) {
                                    __v0 = !PyErr_Occurred().is_null();
                                } else {
                                    __v0 = false;
                                }
                                if __v0 {
                                    break '__join_0;
                                } else {
                                    if x < 0 || x > 1114111 {
                                        PyErr_SetString(
                                            PyExc_OverflowError,
                                            c"%c arg not in range(0x110000)".as_ptr(),
                                        );
                                        break '__join_0;
                                    } else {
                                        tmp = PyUnicode_FromOrdinal(x as i32);
                                        inumeric_chars = 0;
                                        n_digits = 1;
                                        maxchar = Py_MAX!(maxchar, x as u32);
                                        // As a sort-of hack, we tell calc_number_widths that we only
                                        // have "remainder" characters. calc_number_widths thinks
                                        // these are characters that don't get formatted, only copied
                                        // into the output string. We do this for 'c' formatting,
                                        // because the characters are likely to be non-digits.
                                        n_remainder = 1;
                                    }
                                }
                            }
                        }
                    } else {
                        '__join_26: {
                            '__join_28: {
                                leading_chars_to_skip = 0;
                                // Compute the base and how many characters will be added by
                                // PyNumber_ToBase
                                let __t0: u32 = (*format).r#type;
                                if __t0 == 98 {
                                    base = 2;
                                    leading_chars_to_skip = 2; // 0b
                                    break '__join_26;
                                } else {
                                    if __t0 == 111 {
                                        base = 8;
                                        leading_chars_to_skip = 2; // 0o
                                        break '__join_26;
                                    } else {
                                        if __t0 == 120 {
                                            break '__join_28;
                                        } else {
                                            if __t0 == 88 {
                                                break '__join_28;
                                            }
                                        }
                                    }
                                }
                                base = 10; // shouldn't be needed, but stops a compiler warning
                                break '__join_26;
                            }
                            base = 16;
                            leading_chars_to_skip = 2; // 0x
                        }
                        if (*format).sign != 43
                            && (*format).sign != 32
                            && (*format).width == (1i32.wrapping_neg() as i64)
                            && (*format).r#type != 88
                            && (*format).r#type != 110
                            && ((*format).thousands_separators as u32) == 0
                        {
                            __v9 = PyLong_CheckExact!(value) != 0;
                        } else {
                            __v9 = false;
                        }
                        if __v9 {
                            // Fast path
                            return _PyLong_FormatWriter(writer, value, base, (*format).alternate);
                        } else {
                            // The number of prefix chars is the same as the leading
                            // chars to skip
                            if (*format).alternate != 0 {
                                n_prefix = leading_chars_to_skip as i64;
                            }
                            // Do the hard part, converting to a string in a given base
                            tmp = _PyLong_Format(value, base);
                            if tmp.is_null() {
                                break '__join_0;
                            } else {
                                inumeric_chars = 0;
                                n_digits = PyUnicode_GET_LENGTH!(tmp);
                                prefix = inumeric_chars;
                                // Is a sign character present in the output?  If so, remember it
                                // and skip it
                                if PyUnicode_READ_CHAR!(tmp, inumeric_chars) == 45 {
                                    sign_char = 45;
                                    prefix = prefix.wrapping_add(1);
                                    leading_chars_to_skip = leading_chars_to_skip.wrapping_add(1);
                                }
                                // Skip over the leading chars (0x, 0b, etc.)
                                n_digits = n_digits.wrapping_sub(leading_chars_to_skip as i64);
                                inumeric_chars =
                                    inumeric_chars.wrapping_add(leading_chars_to_skip as i64);
                            }
                        }
                    }
                    // Determine the grouping, separator, and decimal point, if any.
                    if get_locale_info(
                        (if (*format).r#type == 110 {
                            97u32
                        } else {
                            (*format).thousands_separators as u32
                        }) as u32,
                        0u32,
                        &mut (*core::ptr::addr_of_mut!(*__slate_slot_12)),
                    ) != 1i32.wrapping_neg()
                    {
                        // Calculate how much memory we'll need.
                        n_total = calc_number_widths(
                            &mut (*core::ptr::addr_of_mut!(*__slate_slot_14)),
                            n_prefix,
                            sign_char,
                            inumeric_chars,
                            inumeric_chars.wrapping_add(n_digits),
                            n_remainder,
                            0i64,
                            0i32,
                            core::ptr::addr_of_mut!(*__slate_slot_12)
                                as *const __SlateRecord_formatter_unicode_c_447,
                            format,
                            core::ptr::addr_of_mut!(maxchar),
                        );
                        if n_total != (1i32.wrapping_neg() as i64) {
                            // Allocate the memory.
                            if _PyUnicodeWriter_Prepare__3!(writer, n_total, maxchar)
                                != 1i32.wrapping_neg()
                            {
                                // Populate the memory.
                                result = fill_number(
                                    writer,
                                    core::ptr::addr_of_mut!(*__slate_slot_14)
                                        as *const __SlateRecord_formatter_unicode_c_475,
                                    tmp,
                                    inumeric_chars,
                                    tmp,
                                    prefix,
                                    (*format).fill_char,
                                    &(*core::ptr::addr_of_mut!(*__slate_slot_12)),
                                    ((*format).r#type == 88) as i32,
                                );
                            }
                        }
                    }
                }
            }
        }
        Py_XDECREF!(tmp);
        free_locale_info(core::ptr::addr_of_mut!(*__slate_slot_12));
        return result;
    }
}

// float formatting
/// much of this is taken from unicodeobject.c
fn format_float_internal(
    value: *mut _object,
    format: *const __SlateRecord_formatter_unicode_c_141,
    writer: *mut __SlateRecord_unicodeobject_h_539,
) -> i32 {
    let mut __v8: bool = unsafe { core::mem::zeroed() };
    // Locale settings, either from the actual locale or
    // from a hard-code pseudo-locale
    let mut locale: core::mem::MaybeUninit<__SlateRecord_formatter_unicode_c_447> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_15: *mut __SlateRecord_formatter_unicode_c_447 =
        core::ptr::addr_of_mut!(locale) as *mut __SlateRecord_formatter_unicode_c_447;
    let mut unicode_tmp: *mut _object = unsafe { core::mem::zeroed() };
    // Used to see if we have a nan, inf, or regular float.
    let mut float_type: i32 = unsafe { core::mem::zeroed() };
    let mut sign_char: u32 = unsafe { core::mem::zeroed() };
    let mut maxchar: u32 = unsafe { core::mem::zeroed() };
    let mut result: i32 = unsafe { core::mem::zeroed() };
    let mut flags: i32 = unsafe { core::mem::zeroed() };
    let mut spec: core::mem::MaybeUninit<__SlateRecord_formatter_unicode_c_475> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_22: *mut __SlateRecord_formatter_unicode_c_475 =
        core::ptr::addr_of_mut!(spec) as *mut __SlateRecord_formatter_unicode_c_475;
    let mut index: i64 = unsafe { core::mem::zeroed() };
    let mut add_pct: i32 = unsafe { core::mem::zeroed() };
    let mut r#type: u32 = unsafe { core::mem::zeroed() };
    let mut default_precision: i32 = unsafe { core::mem::zeroed() };
    let mut precision: i32 = unsafe { core::mem::zeroed() };
    let mut val: f64 = unsafe { core::mem::zeroed() };
    let mut has_decimal: i32 = unsafe { core::mem::zeroed() };
    let mut n_total: i64 = unsafe { core::mem::zeroed() };
    let mut n_frac: i64 = unsafe { core::mem::zeroed() };
    let mut n_remainder: i64 = unsafe { core::mem::zeroed() };
    let mut n_digits: i64 = unsafe { core::mem::zeroed() };
    // buffer returned from PyOS_double_to_string
    let mut buf: *mut i8 = unsafe { core::mem::zeroed() };
    unsafe {
        buf = core::ptr::null_mut::<i8>();
        default_precision = 6;
        r#type = (*format).r#type;
        add_pct = 0;
        flags = 0;
        result = 1i32.wrapping_neg();
        maxchar = 127;
        sign_char = 0;
        unicode_tmp = core::ptr::null_mut::<_object>();
        core::ptr::write(__slate_slot_15, {
            let mut __t0: __SlateRecord_formatter_unicode_c_447 = core::mem::zeroed();
            __t0.decimal_point = core::ptr::null_mut::<_object>();
            __t0.thousands_sep = core::ptr::null_mut::<_object>();
            __t0.frac_thousands_sep = core::ptr::null_mut::<_object>();
            __t0.grouping = core::ptr::null::<i8>();
            __t0
        });
        if (*format).precision > 2147483647 {
            PyErr_SetString(PyExc_ValueError, c"precision too big".as_ptr());
        } else {
            precision = (*format).precision as i32;
            if (*format).alternate != 0 {
                flags |= 4;
            }
            if (*format).no_neg_0 != 0 {
                flags |= 8;
            }
            if r#type == 0 {
                // Omitted type specifier.  Behaves in the same way as repr(x)
                // and str(x) if no precision is given, else like 'g', but with
                // at least one digit after the decimal point.
                flags |= 2;
                r#type = 114;
                default_precision = 0;
            }
            if r#type == 110 {
                r#type = 103;
            }
            // 'n' is the same as 'g', except for the locale used to
            // format the result. We take care of that later.
            val = PyFloat_AsDouble(value);
            if val == -1.0f64 {
                __v8 = !PyErr_Occurred().is_null();
            } else {
                __v8 = false;
            }
            if !__v8 {
                if r#type == 37 {
                    r#type = 102;
                    val *= 100i32 as f64;
                    add_pct = 1;
                }
                if precision < 0 {
                    precision = default_precision;
                } else {
                    if r#type == 114 {
                        r#type = 103;
                    }
                }
                // Cast "type", because if we're in unicode we need to pass an
                // 8-bit char. This is safe, because we've restricted what "type"
                // can be.
                buf = PyOS_double_to_string(
                    val,
                    r#type as i8,
                    precision,
                    flags,
                    core::ptr::addr_of_mut!(float_type),
                );
                if !buf.is_null() {
                    n_digits =
                        (strlen((buf as *const i8) as *const core::ffi::c_char) as u64) as i64;
                    if add_pct != 0 {
                        // We know that buf has a trailing zero (since we just called
                        // strlen() on it), and we don't use that fact any more. So we
                        // can just write over the trailing zero.
                        *buf.wrapping_offset(n_digits as isize) = 37;
                        n_digits = n_digits.wrapping_add(1);
                    }
                    if (*format).sign != 43
                        && (*format).sign != 32
                        && (*format).width == (1i32.wrapping_neg() as i64)
                        && (*format).r#type != 110
                        && ((*format).thousands_separators as u32) == 0
                        && ((*format).frac_thousands_separator as u32) == 0
                    {
                        // Fast path
                        result =
                            _PyUnicodeWriter_WriteASCIIString(writer, buf as *const i8, n_digits);
                        PyMem_Free(buf as *mut ());
                        return result;
                    } else {
                        // Since there is no unicode version of PyOS_double_to_string,
                        // just use the 8 bit version and then convert to unicode.
                        unicode_tmp = _PyUnicode_FromASCII(buf as *const i8, n_digits);
                        PyMem_Free(buf as *mut ());
                        if !unicode_tmp.is_null() {
                            // Is a sign character present in the output?  If so, remember it
                            // and skip it
                            index = 0;
                            if PyUnicode_READ_CHAR!(unicode_tmp, index) == 45 {
                                sign_char = 45;
                                index = index.wrapping_add(1);
                                n_digits = n_digits.wrapping_sub(1);
                            }
                            // Determine if we have any "remainder" (after the digits, might include
                            // decimal or exponent or both (or neither))
                            parse_number(
                                unicode_tmp,
                                index,
                                index.wrapping_add(n_digits),
                                &mut (*core::ptr::addr_of_mut!(n_remainder)),
                                &mut (*core::ptr::addr_of_mut!(n_frac)),
                                &mut (*core::ptr::addr_of_mut!(has_decimal)),
                            );
                            // Determine the grouping, separator, and decimal point, if any.
                            if get_locale_info(
                                (if (*format).r#type == 110 {
                                    97u32
                                } else {
                                    (*format).thousands_separators as u32
                                }) as u32,
                                (*format).frac_thousands_separator,
                                &mut (*core::ptr::addr_of_mut!(*__slate_slot_15)),
                            ) != 1i32.wrapping_neg()
                            {
                                // Calculate how much memory we'll need.
                                n_total = calc_number_widths(
                                    &mut (*core::ptr::addr_of_mut!(*__slate_slot_22)),
                                    0i64,
                                    sign_char,
                                    index,
                                    index.wrapping_add(n_digits),
                                    n_remainder,
                                    n_frac,
                                    has_decimal,
                                    core::ptr::addr_of_mut!(*__slate_slot_15)
                                        as *const __SlateRecord_formatter_unicode_c_447,
                                    format,
                                    core::ptr::addr_of_mut!(maxchar),
                                );
                                if n_total != (1i32.wrapping_neg() as i64) {
                                    // Allocate the memory.
                                    if _PyUnicodeWriter_Prepare__3!(writer, n_total, maxchar)
                                        != 1i32.wrapping_neg()
                                    {
                                        // Populate the memory.
                                        result = fill_number(
                                            writer,
                                            core::ptr::addr_of_mut!(*__slate_slot_22)
                                                as *const __SlateRecord_formatter_unicode_c_475,
                                            unicode_tmp,
                                            index,
                                            core::ptr::null_mut::<_object>(),
                                            0i64,
                                            (*format).fill_char,
                                            &(*core::ptr::addr_of_mut!(*__slate_slot_15)),
                                            0i32,
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Py_XDECREF!(unicode_tmp);
        free_locale_info(core::ptr::addr_of_mut!(*__slate_slot_15));
        return result;
    }
}

// complex formatting
fn format_complex_internal(
    value: *mut _object,
    format: *const __SlateRecord_formatter_unicode_c_141,
    writer: *mut __SlateRecord_unicodeobject_h_539,
) -> i32 {
    let mut __v24: bool = unsafe { core::mem::zeroed() };
    let mut __v25: bool = unsafe { core::mem::zeroed() };
    // Locale settings, either from the actual locale or
    // from a hard-code pseudo-locale
    let mut locale: core::mem::MaybeUninit<__SlateRecord_formatter_unicode_c_447> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_26: *mut __SlateRecord_formatter_unicode_c_447 =
        core::ptr::addr_of_mut!(locale) as *mut __SlateRecord_formatter_unicode_c_447;
    let mut im_unicode_tmp: *mut _object = unsafe { core::mem::zeroed() };
    let mut re_unicode_tmp: *mut _object = unsafe { core::mem::zeroed() };
    let mut total: i64 = unsafe { core::mem::zeroed() };
    let mut rpad: i64 = unsafe { core::mem::zeroed() };
    let mut lpad: i64 = unsafe { core::mem::zeroed() };
    let mut skip_re: i32 = unsafe { core::mem::zeroed() };
    let mut add_parens: i32 = unsafe { core::mem::zeroed() };
    let mut im_float_type: i32 = unsafe { core::mem::zeroed() };
    // Used to see if we have a nan, inf, or regular float.
    let mut re_float_type: i32 = unsafe { core::mem::zeroed() };
    let mut im_sign_char: u32 = unsafe { core::mem::zeroed() };
    let mut re_sign_char: u32 = unsafe { core::mem::zeroed() };
    let mut rdata: *mut () = unsafe { core::mem::zeroed() };
    let mut rkind: i32 = unsafe { core::mem::zeroed() };
    let mut maxchar: u32 = unsafe { core::mem::zeroed() };
    let mut result: i32 = unsafe { core::mem::zeroed() };
    let mut flags: i32 = unsafe { core::mem::zeroed() };
    let mut im_spec: core::mem::MaybeUninit<__SlateRecord_formatter_unicode_c_475> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_43: *mut __SlateRecord_formatter_unicode_c_475 =
        core::ptr::addr_of_mut!(im_spec) as *mut __SlateRecord_formatter_unicode_c_475;
    let mut re_spec: core::mem::MaybeUninit<__SlateRecord_formatter_unicode_c_475> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_44: *mut __SlateRecord_formatter_unicode_c_475 =
        core::ptr::addr_of_mut!(re_spec) as *mut __SlateRecord_formatter_unicode_c_475;
    let mut i_im: i64 = unsafe { core::mem::zeroed() };
    let mut i_re: i64 = unsafe { core::mem::zeroed() };
    let mut r#type: u32 = unsafe { core::mem::zeroed() };
    let mut default_precision: i32 = unsafe { core::mem::zeroed() };
    let mut precision: i32 = unsafe { core::mem::zeroed() };
    let mut im_has_decimal: i32 = unsafe { core::mem::zeroed() };
    let mut re_has_decimal: i32 = unsafe { core::mem::zeroed() };
    let mut n_im_total: i64 = unsafe { core::mem::zeroed() };
    let mut n_re_total: i64 = unsafe { core::mem::zeroed() };
    let mut n_im_frac: i64 = unsafe { core::mem::zeroed() };
    let mut n_re_frac: i64 = unsafe { core::mem::zeroed() };
    let mut n_im_remainder: i64 = unsafe { core::mem::zeroed() };
    let mut n_re_remainder: i64 = unsafe { core::mem::zeroed() };
    let mut n_im_digits: i64 = unsafe { core::mem::zeroed() };
    let mut n_re_digits: i64 = unsafe { core::mem::zeroed() };
    let mut tmp_format: core::mem::MaybeUninit<__SlateRecord_formatter_unicode_c_141> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_60: *mut __SlateRecord_formatter_unicode_c_141 =
        core::ptr::addr_of_mut!(tmp_format) as *mut __SlateRecord_formatter_unicode_c_141; // buffer returned from PyOS_double_to_string
    let mut im_buf: *mut i8 = unsafe { core::mem::zeroed() };
    // buffer returned from PyOS_double_to_string
    let mut re_buf: *mut i8 = unsafe { core::mem::zeroed() };
    let mut im: f64 = unsafe { core::mem::zeroed() };
    let mut re: f64 = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_0: {
            re_buf = core::ptr::null_mut::<i8>();
            im_buf = core::ptr::null_mut::<i8>();
            core::ptr::write(__slate_slot_60, *format);
            default_precision = 6;
            r#type = (*format).r#type;
            flags = 0;
            result = 1i32.wrapping_neg();
            maxchar = 127;
            re_sign_char = 0;
            im_sign_char = 0;
            add_parens = 0;
            skip_re = 0;
            re_unicode_tmp = core::ptr::null_mut::<_object>();
            im_unicode_tmp = core::ptr::null_mut::<_object>();
            core::ptr::write(__slate_slot_26, {
                let mut __t0: __SlateRecord_formatter_unicode_c_447 = core::mem::zeroed();
                __t0.decimal_point = core::ptr::null_mut::<_object>();
                __t0.thousands_sep = core::ptr::null_mut::<_object>();
                __t0.frac_thousands_sep = core::ptr::null_mut::<_object>();
                __t0.grouping = core::ptr::null::<i8>();
                __t0
            });
            if (*format).precision > 2147483647 {
                PyErr_SetString(PyExc_ValueError, c"precision too big".as_ptr());
            } else {
                precision = (*format).precision as i32;
                // Zero padding is not allowed.
                if (*format).fill_char == 48 {
                    PyErr_SetString(
                        PyExc_ValueError,
                        c"Zero padding is not allowed in complex format specifier".as_ptr(),
                    );
                } else {
                    // Neither is '=' alignment .
                    if (*format).align == 61 {
                        PyErr_SetString(
                            PyExc_ValueError,
                            c"'=' alignment flag is not allowed in complex format specifier"
                                .as_ptr(),
                        );
                    } else {
                        re = PyComplex_RealAsDouble(value);
                        if re == -1.0f64 {
                            __v25 = !PyErr_Occurred().is_null();
                        } else {
                            __v25 = false;
                        }
                        if !__v25 {
                            im = PyComplex_ImagAsDouble(value);
                            if im == -1.0f64 {
                                __v24 = !PyErr_Occurred().is_null();
                            } else {
                                __v24 = false;
                            }
                            if !__v24 {
                                if (*format).alternate != 0 {
                                    flags |= 4;
                                }
                                if (*format).no_neg_0 != 0 {
                                    flags |= 8;
                                }
                                if r#type == 0 {
                                    // Omitted type specifier. Should be like str(self).
                                    r#type = 114;
                                    default_precision = 0;
                                    if re == 0.0f64 && copysign(1.0f64, re) == 1.0f64 {
                                        skip_re = 1;
                                    } else {
                                        add_parens = 1;
                                    }
                                }
                                if r#type == 110 {
                                    r#type = 103;
                                }
                                // 'n' is the same as 'g', except for the locale used to
                                // format the result. We take care of that later.
                                if precision < 0 {
                                    precision = default_precision;
                                } else {
                                    if r#type == 114 {
                                        r#type = 103;
                                    }
                                }
                                // Cast "type", because if we're in unicode we need to pass an
                                // 8-bit char. This is safe, because we've restricted what "type"
                                // can be.
                                re_buf = PyOS_double_to_string(
                                    re,
                                    r#type as i8,
                                    precision,
                                    flags,
                                    core::ptr::addr_of_mut!(re_float_type),
                                );
                                if !re_buf.is_null() {
                                    im_buf = PyOS_double_to_string(
                                        im,
                                        r#type as i8,
                                        precision,
                                        flags,
                                        core::ptr::addr_of_mut!(im_float_type),
                                    );
                                    if !im_buf.is_null() {
                                        n_re_digits = (strlen(
                                            (re_buf as *const i8) as *const core::ffi::c_char,
                                        )
                                            as u64)
                                            as i64;
                                        n_im_digits = (strlen(
                                            (im_buf as *const i8) as *const core::ffi::c_char,
                                        )
                                            as u64)
                                            as i64;
                                        // Since there is no unicode version of PyOS_double_to_string,
                                        // just use the 8 bit version and then convert to unicode.
                                        re_unicode_tmp =
                                            _PyUnicode_FromASCII(re_buf as *const i8, n_re_digits);
                                        if !re_unicode_tmp.is_null() {
                                            i_re = 0;
                                            im_unicode_tmp = _PyUnicode_FromASCII(
                                                im_buf as *const i8,
                                                n_im_digits,
                                            );
                                            if !im_unicode_tmp.is_null() {
                                                i_im = 0;
                                                // Is a sign character present in the output?  If so, remember it
                                                // and skip it
                                                if PyUnicode_READ_CHAR!(re_unicode_tmp, i_re) == 45
                                                {
                                                    re_sign_char = 45;
                                                    i_re = i_re.wrapping_add(1);
                                                    n_re_digits = n_re_digits.wrapping_sub(1);
                                                }
                                                if PyUnicode_READ_CHAR!(im_unicode_tmp, i_im) == 45
                                                {
                                                    im_sign_char = 45;
                                                    i_im = i_im.wrapping_add(1);
                                                    n_im_digits = n_im_digits.wrapping_sub(1);
                                                }
                                                // Determine if we have any "remainder" (after the digits, might include
                                                // decimal or exponent or both (or neither))
                                                parse_number(
                                                    re_unicode_tmp,
                                                    i_re,
                                                    i_re.wrapping_add(n_re_digits),
                                                    &mut (*core::ptr::addr_of_mut!(n_re_remainder)),
                                                    &mut (*core::ptr::addr_of_mut!(n_re_frac)),
                                                    &mut (*core::ptr::addr_of_mut!(re_has_decimal)),
                                                );
                                                parse_number(
                                                    im_unicode_tmp,
                                                    i_im,
                                                    i_im.wrapping_add(n_im_digits),
                                                    &mut (*core::ptr::addr_of_mut!(n_im_remainder)),
                                                    &mut (*core::ptr::addr_of_mut!(n_im_frac)),
                                                    &mut (*core::ptr::addr_of_mut!(im_has_decimal)),
                                                );
                                                // Determine the grouping, separator, and decimal point, if any.
                                                if get_locale_info(
                                                    (if (*format).r#type == 110 {
                                                        97u32
                                                    } else {
                                                        (*format).thousands_separators as u32
                                                    })
                                                        as u32,
                                                    (*format).frac_thousands_separator,
                                                    &mut (*core::ptr::addr_of_mut!(
                                                        *__slate_slot_26
                                                    )),
                                                ) != 1i32.wrapping_neg()
                                                {
                                                    // Turn off any padding. We'll do it later after we've composed
                                                    // the numbers without padding.
                                                    (*__slate_slot_60).fill_char = 0;
                                                    (*__slate_slot_60).align = 60;
                                                    (*__slate_slot_60).width =
                                                        1i32.wrapping_neg() as i64;
                                                    // Calculate how much memory we'll need.
                                                    n_re_total = calc_number_widths(&mut (*core::ptr::addr_of_mut!(*__slate_slot_44)), 0i64, re_sign_char, i_re, i_re.wrapping_add(n_re_digits), n_re_remainder, n_re_frac, re_has_decimal, core::ptr::addr_of_mut!(*__slate_slot_26) as *const __SlateRecord_formatter_unicode_c_447, core::ptr::addr_of_mut!(*__slate_slot_60) as *const __SlateRecord_formatter_unicode_c_141, core::ptr::addr_of_mut!(maxchar));
                                                    if n_re_total != (1i32.wrapping_neg() as i64) {
                                                        // Same formatting, but always include a sign, unless the real part is
                                                        // going to be omitted, in which case we use whatever sign convention was
                                                        // requested by the original format.
                                                        if skip_re == 0 {
                                                            (*__slate_slot_60).sign = 43;
                                                        }
                                                        n_im_total = calc_number_widths(&mut (*core::ptr::addr_of_mut!(*__slate_slot_43)), 0i64, im_sign_char, i_im, i_im.wrapping_add(n_im_digits), n_im_remainder, n_im_frac, im_has_decimal, core::ptr::addr_of_mut!(*__slate_slot_26) as *const __SlateRecord_formatter_unicode_c_447, core::ptr::addr_of_mut!(*__slate_slot_60) as *const __SlateRecord_formatter_unicode_c_141, core::ptr::addr_of_mut!(maxchar));
                                                        if n_im_total
                                                            != (1i32.wrapping_neg() as i64)
                                                        {
                                                            if skip_re != 0 {
                                                                n_re_total = 0;
                                                            }
                                                            // Add 1 for the 'j', and optionally 2 for parens.
                                                            calc_padding(
                                                                n_re_total
                                                                    .wrapping_add(n_im_total)
                                                                    .wrapping_add(1)
                                                                    .wrapping_add(
                                                                        add_parens.wrapping_mul(2)
                                                                            as i64,
                                                                    ),
                                                                (*format).width,
                                                                (*format).align,
                                                                &mut (*core::ptr::addr_of_mut!(
                                                                    lpad
                                                                )),
                                                                &mut (*core::ptr::addr_of_mut!(
                                                                    rpad
                                                                )),
                                                                &mut (*core::ptr::addr_of_mut!(
                                                                    total
                                                                )),
                                                            );
                                                            if lpad != 0 || rpad != 0 {
                                                                maxchar =
                                                                    Py_MAX!(maxchar, unsafe {
                                                                        (*format).fill_char
                                                                    });
                                                            }
                                                            if _PyUnicodeWriter_Prepare__3!(
                                                                writer, total, maxchar
                                                            ) != 1i32.wrapping_neg()
                                                            {
                                                                rkind = (*writer).kind;
                                                                rdata = (*writer).data;
                                                                // Populate the memory. First, the padding.
                                                                result = fill_padding(
                                                                    writer,
                                                                    n_re_total
                                                                        .wrapping_add(n_im_total)
                                                                        .wrapping_add(1)
                                                                        .wrapping_add(
                                                                            add_parens
                                                                                .wrapping_mul(2)
                                                                                as i64,
                                                                        ),
                                                                    (*format).fill_char,
                                                                    lpad,
                                                                    rpad,
                                                                );
                                                                if result != 1i32.wrapping_neg() {
                                                                    if add_parens != 0 {
                                                                        PyUnicode_WRITE!(
                                                                            rkind,
                                                                            rdata,
                                                                            unsafe {
                                                                                (*writer).pos
                                                                            },
                                                                            40i32
                                                                        );
                                                                        let __v10: i64 =
                                                                            (*writer).pos;
                                                                        (*writer).pos =
                                                                            __v10.wrapping_add(1);
                                                                    }
                                                                    if skip_re == 0 {
                                                                        result = fill_number(writer, core::ptr::addr_of_mut!(*__slate_slot_44) as *const __SlateRecord_formatter_unicode_c_475, re_unicode_tmp, i_re, core::ptr::null_mut::<_object>(), 0i64, 0u32, &(*core::ptr::addr_of_mut!(*__slate_slot_26)), 0i32);
                                                                        if result
                                                                            == 1i32.wrapping_neg()
                                                                        {
                                                                            break '__join_0;
                                                                        }
                                                                    }
                                                                    result = fill_number(writer, core::ptr::addr_of_mut!(*__slate_slot_43) as *const __SlateRecord_formatter_unicode_c_475, im_unicode_tmp, i_im, core::ptr::null_mut::<_object>(), 0i64, 0u32, &(*core::ptr::addr_of_mut!(*__slate_slot_26)), 0i32);
                                                                    if result != 1i32.wrapping_neg()
                                                                    {
                                                                        PyUnicode_WRITE!(
                                                                            rkind,
                                                                            rdata,
                                                                            unsafe {
                                                                                (*writer).pos
                                                                            },
                                                                            106i32
                                                                        );
                                                                        let __v7: i64 =
                                                                            (*writer).pos;
                                                                        (*writer).pos =
                                                                            __v7.wrapping_add(1);
                                                                        if add_parens != 0 {
                                                                            PyUnicode_WRITE!(
                                                                                rkind,
                                                                                rdata,
                                                                                unsafe {
                                                                                    (*writer).pos
                                                                                },
                                                                                41i32
                                                                            );
                                                                            let __v4: i64 =
                                                                                (*writer).pos;
                                                                            (*writer).pos = __v4
                                                                                .wrapping_add(1);
                                                                        }
                                                                        let __v1: i64 =
                                                                            (*writer).pos;
                                                                        (*writer).pos =
                                                                            __v1.wrapping_add(rpad);
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        PyMem_Free(re_buf as *mut ());
        PyMem_Free(im_buf as *mut ());
        Py_XDECREF!(re_unicode_tmp);
        Py_XDECREF!(im_unicode_tmp);
        free_locale_info(core::ptr::addr_of_mut!(*__slate_slot_26));
        return result;
    }
}

/// built in formatters
fn format_obj(obj: *mut _object, writer: *mut __SlateRecord_unicodeobject_h_539) -> i32 {
    let mut err: i32 = 0;
    let str: *mut _object = unsafe { PyObject_Str(obj) };
    if str.is_null() {
        return 1i32.wrapping_neg();
    }
    err = unsafe { _PyUnicodeWriter_WriteStr(writer, str) };
    Py_DECREF!(str);
    err
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.formatter_unicode._PyUnicode_FormatAdvancedWriter")]
extern "C-unwind" fn _PyUnicode_FormatAdvancedWriter(
    writer: *mut __SlateRecord_unicodeobject_h_539,
    obj: *mut _object,
    format_spec: *mut _object,
    start: i64,
    end: i64,
) -> i32 {
    let mut format: __SlateRecord_formatter_unicode_c_141 = unsafe { core::mem::zeroed() };
    // check for the special case of zero length format spec, make
    // it equivalent to str(obj)
    if start == end {
        if PyUnicode_CheckExact!(obj) != 0 {
            return unsafe { _PyUnicodeWriter_WriteStr(writer, obj) };
        } else {
            return format_obj(obj, writer);
        }
    }
    // parse the format_spec
    if parse_internal_render_format_spec(
        obj,
        format_spec,
        start,
        end,
        unsafe { &mut (*core::ptr::addr_of_mut!(format)) },
        115i8,
        60i8,
    ) == 0
    {
        return 1i32.wrapping_neg();
    }
    // type conversion?
    match format.r#type {
        115 => {
            return format_string_internal(
                obj,
                core::ptr::addr_of_mut!(format) as *const __SlateRecord_formatter_unicode_c_141,
                writer,
            ); // no type conversion needed, already a string.  do the formatting
        }
        _ => {
            unknown_presentation_type(format.r#type, unsafe { (*Py_TYPE!(obj)).tp_name }); // unknown
            return 1i32.wrapping_neg();
        }
    }
    unsafe { core::mem::zeroed() }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.formatter_unicode._PyLong_FormatAdvancedWriter")]
extern "C-unwind" fn _PyLong_FormatAdvancedWriter(
    writer: *mut __SlateRecord_unicodeobject_h_539,
    obj: *mut _object,
    format_spec: *mut _object,
    start: i64,
    end: i64,
) -> i32 {
    let mut result: i32 = unsafe { core::mem::zeroed() };
    let mut format: core::mem::MaybeUninit<__SlateRecord_formatter_unicode_c_141> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_1: *mut __SlateRecord_formatter_unicode_c_141 =
        core::ptr::addr_of_mut!(format) as *mut __SlateRecord_formatter_unicode_c_141;
    let mut tmp: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        tmp = core::ptr::null_mut::<_object>();
        result = 1i32.wrapping_neg();
        // check for the special case of zero length format spec, make
        // it equivalent to str(obj)
        if start == end {
            if PyLong_CheckExact!(obj) != 0 {
                return _PyLong_FormatWriter(writer, obj, 10i32, 0i32);
            } else {
                return format_obj(obj, writer);
            }
        } else {
            '__join_0: {
                // parse the format_spec
                if parse_internal_render_format_spec(
                    obj,
                    format_spec,
                    start,
                    end,
                    &mut (*core::ptr::addr_of_mut!(*__slate_slot_1)),
                    100i8,
                    62i8,
                ) != 0
                {
                    '__join_4: {
                        // type conversion?
                        let __t0: u32 = (*__slate_slot_1).r#type;
                        if __t0 == 98 {
                            break '__join_4;
                        } else {
                            if __t0 == 99 {
                                break '__join_4;
                            } else {
                                if __t0 == 100 {
                                    break '__join_4;
                                } else {
                                    if __t0 == 111 {
                                        break '__join_4;
                                    } else {
                                        if __t0 == 120 {
                                            break '__join_4;
                                        } else {
                                            if __t0 == 88 {
                                                break '__join_4;
                                            } else {
                                                if __t0 == 110 {
                                                    break '__join_4;
                                                } else {
                                                    if __t0 != 101 {
                                                        if __t0 != 69 {
                                                            if __t0 != 102 {
                                                                if __t0 != 70 {
                                                                    if __t0 != 103 {
                                                                        if __t0 != 71 {
                                                                            if __t0 != 37 {
                                                                                unknown_presentation_type((*__slate_slot_1).r#type, (*Py_TYPE!(obj)).tp_name); // unknown
                                                                                break '__join_0;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        tmp = PyNumber_Float(obj); // convert to float
                        if tmp.is_null() {
                            break '__join_0;
                        } else {
                            result = format_float_internal(
                                tmp,
                                core::ptr::addr_of_mut!(*__slate_slot_1)
                                    as *const __SlateRecord_formatter_unicode_c_141,
                                writer,
                            );
                            break '__join_0;
                        }
                    }
                    result = format_long_internal(
                        obj,
                        core::ptr::addr_of_mut!(*__slate_slot_1)
                            as *const __SlateRecord_formatter_unicode_c_141,
                        writer,
                    ); // no type conversion needed, already an int.  do the formatting
                }
            }
            Py_XDECREF!(tmp);
            return result;
        }
    }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.formatter_unicode._PyFloat_FormatAdvancedWriter")]
extern "C-unwind" fn _PyFloat_FormatAdvancedWriter(
    writer: *mut __SlateRecord_unicodeobject_h_539,
    obj: *mut _object,
    format_spec: *mut _object,
    start: i64,
    end: i64,
) -> i32 {
    let mut format: __SlateRecord_formatter_unicode_c_141 = unsafe { core::mem::zeroed() };
    // check for the special case of zero length format spec, make
    // it equivalent to str(obj)
    if start == end {
        return format_obj(obj, writer);
    }
    // parse the format_spec
    if parse_internal_render_format_spec(
        obj,
        format_spec,
        start,
        end,
        unsafe { &mut (*core::ptr::addr_of_mut!(format)) },
        0i8,
        62i8,
    ) == 0
    {
        return 1i32.wrapping_neg();
    }
    // type conversion?
    match format.r#type {
        0 | 37 | 69..=71 | 101..=103 | 110 => {
            return format_float_internal(
                obj,
                core::ptr::addr_of_mut!(format) as *const __SlateRecord_formatter_unicode_c_141,
                writer,
            ); // No format code: like 'g', but with at least one decimal.
            // no conversion, already a float.  do the formatting
        }
        _ => {
            unknown_presentation_type(format.r#type, unsafe { (*Py_TYPE!(obj)).tp_name }); // unknown
            return 1i32.wrapping_neg();
        }
    }
    unsafe { core::mem::zeroed() }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.formatter_unicode._PyComplex_FormatAdvancedWriter")]
extern "C-unwind" fn _PyComplex_FormatAdvancedWriter(
    writer: *mut __SlateRecord_unicodeobject_h_539,
    obj: *mut _object,
    format_spec: *mut _object,
    start: i64,
    end: i64,
) -> i32 {
    let mut format: __SlateRecord_formatter_unicode_c_141 = unsafe { core::mem::zeroed() };
    // check for the special case of zero length format spec, make
    // it equivalent to str(obj)
    if start == end {
        return format_obj(obj, writer);
    }
    // parse the format_spec
    if parse_internal_render_format_spec(
        obj,
        format_spec,
        start,
        end,
        unsafe { &mut (*core::ptr::addr_of_mut!(format)) },
        0i8,
        62i8,
    ) == 0
    {
        return 1i32.wrapping_neg();
    }
    // type conversion?
    match format.r#type {
        0 | 69..=71 | 101..=103 | 110 => {
            return format_complex_internal(
                obj,
                core::ptr::addr_of_mut!(format) as *const __SlateRecord_formatter_unicode_c_141,
                writer,
            ); // No format code: like 'g', but with at least one decimal.
            // no conversion, already a complex.  do the formatting
        }
        _ => {
            unknown_presentation_type(format.r#type, unsafe { (*Py_TYPE!(obj)).tp_name }); // unknown
            return 1i32.wrapping_neg();
        }
    }
    unsafe { core::mem::zeroed() }
}
