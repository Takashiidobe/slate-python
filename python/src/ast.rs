//! This file exposes PyAST_Validate interface to check the integrity
//! of the given abstract syntax tree (potentially constructed manually).
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
// Signed variants of the above
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
macro_rules! ENTER_RECURSIVE {
    () => {{
        if (unsafe { Py_EnterRecursiveCall(c" during compilation".as_ptr()) }) != 0 {
            return 0i32;
        }
    }};
}

macro_rules! LEAVE_RECURSIVE {
    () => {
        unsafe { Py_LeaveRecursiveCall() };
    };
}

macro_rules! PyBool_Check {
    ($x:expr) => {
        Py_IS_TYPE!($x, unsafe { core::ptr::addr_of_mut!(PyBool_Type) })
    };
}

macro_rules! PyBytes_CheckExact {
    ($op:expr) => {
        Py_IS_TYPE!($op, unsafe { core::ptr::addr_of_mut!(PyBytes_Type) })
    };
}

macro_rules! PyComplex_CheckExact {
    ($op:expr) => {
        Py_IS_TYPE!($op, unsafe { core::ptr::addr_of_mut!(PyComplex_Type) })
    };
}

macro_rules! PyFloat_CheckExact {
    ($op:expr) => {
        Py_IS_TYPE!($op, unsafe { core::ptr::addr_of_mut!(PyFloat_Type) })
    };
}

macro_rules! PyFrozenSet_CheckExact {
    ($ob:expr) => {
        Py_IS_TYPE!($ob, unsafe { core::ptr::addr_of_mut!(PyFrozenSet_Type) })
    };
}

macro_rules! PyLong_CheckExact {
    ($op:expr) => {
        Py_IS_TYPE!($op, unsafe { core::ptr::addr_of_mut!(PyLong_Type) })
    };
}

macro_rules! PyTuple_CheckExact {
    ($op:expr) => {
        Py_IS_TYPE!($op, unsafe { core::ptr::addr_of_mut!(PyTuple_Type) })
    };
}

macro_rules! PyUnicode_CheckExact {
    ($op:expr) => {
        Py_IS_TYPE!($op, unsafe { core::ptr::addr_of_mut!(PyUnicode_Type) })
    };
}

macro_rules! Py_DECREF {
    ($op:expr) => {
        Py_DECREF(_PyObject_CAST!($op));
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

macro_rules! Py_UNREACHABLE {
    () => {
        unsafe { core::hint::unreachable_unchecked() };
    };
}

macro_rules! VALIDATE_POSITIONS {
    ($node:expr) => {{
        if (unsafe { (*$node).lineno }) > unsafe { (*$node).end_lineno } {
            unsafe {
                PyErr_Format(
                    PyExc_ValueError,
                    c"AST node line range (%d, %d) is not valid".as_ptr(),
                    (*$node).lineno,
                    (*$node).end_lineno,
                )
            };
            return 0i32;
        }
        if (unsafe { (*$node).lineno }) < 0
            && (unsafe { (*$node).end_lineno }) != unsafe { (*$node).lineno }
            || (unsafe { (*$node).col_offset }) < 0
                && (unsafe { (*$node).col_offset }) != unsafe { (*$node).end_col_offset }
        {
            unsafe {
                PyErr_Format(
                    PyExc_ValueError,
                    c"AST node column range (%d, %d) for line range (%d, %d) is not valid".as_ptr(),
                    (*$node).col_offset,
                    (*$node).end_col_offset,
                    (*$node).lineno,
                    (*$node).end_lineno,
                )
            };
            return 0i32;
        }
        if (unsafe { (*$node).lineno }) == unsafe { (*$node).end_lineno }
            && (unsafe { (*$node).col_offset }) > unsafe { (*$node).end_col_offset }
        {
            unsafe {
                PyErr_Format(
                    PyExc_ValueError,
                    c"line %d, column %d-%d is not a valid range".as_ptr(),
                    (*$node).lineno,
                    (*$node).col_offset,
                    (*$node).end_col_offset,
                )
            };
            return 0i32;
        }
    }};
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

macro_rules! asdl_seq_GET__9 {
    ($S:expr, $I:expr) => {
        _Py_RVALUE!(unsafe {
            *((*$S).typed_elements.as_mut_ptr() as *mut *mut _alias).wrapping_offset(($I) as isize)
        })
    };
}

macro_rules! asdl_seq_GET__10 {
    ($S:expr, $I:expr) => {
        _Py_RVALUE!(unsafe {
            *((*$S).typed_elements.as_mut_ptr() as *mut *mut _excepthandler)
                .wrapping_offset(($I) as isize)
        })
    };
}

macro_rules! asdl_seq_GET__7 {
    ($S:expr, $I:expr) => {
        _Py_RVALUE!(unsafe {
            *((*$S).typed_elements.as_mut_ptr() as *mut *mut _type_param)
                .wrapping_offset(($I) as isize)
        })
    };
}

macro_rules! asdl_seq_GET__2 {
    ($S:expr, $I:expr) => {
        _Py_RVALUE!(unsafe {
            *((*$S).typed_elements.as_mut_ptr() as *mut *mut _stmt).wrapping_offset(($I) as isize)
        })
    };
}

macro_rules! asdl_seq_GET__3 {
    ($S:expr, $I:expr) => {
        _Py_RVALUE!(unsafe {
            *((*$S).typed_elements.as_mut_ptr() as *mut *mut _pattern)
                .wrapping_offset(($I) as isize)
        })
    };
}

macro_rules! asdl_seq_GET__13 {
    ($S:expr, $I:expr) => {
        _Py_RVALUE!(unsafe {
            *((*$S).typed_elements.as_mut_ptr() as *mut *mut _match_case)
                .wrapping_offset(($I) as isize)
        })
    };
}

macro_rules! asdl_seq_GET__11 {
    ($S:expr, $I:expr) => {
        _Py_RVALUE!(unsafe {
            *((*$S).typed_elements.as_mut_ptr() as *mut *mut _withitem)
                .wrapping_offset(($I) as isize)
        })
    };
}

macro_rules! asdl_seq_GET__6 {
    ($S:expr, $I:expr) => {
        _Py_RVALUE!(unsafe {
            *((*$S).typed_elements.as_mut_ptr() as *mut *mut _arg).wrapping_offset(($I) as isize)
        })
    };
}

macro_rules! asdl_seq_GET {
    ($S:expr, $I:expr) => {
        _Py_RVALUE!(unsafe {
            *((*$S).typed_elements.as_mut_ptr() as *mut *mut _expr).wrapping_offset(($I) as isize)
        })
    };
}

macro_rules! asdl_seq_GET__12 {
    ($S:expr, $I:expr) => {
        _Py_RVALUE!(unsafe {
            *((*$S).typed_elements.as_mut_ptr() as *mut *mut _object).wrapping_offset(($I) as isize)
        })
    };
}

macro_rules! asdl_seq_GET__5 {
    ($S:expr, $I:expr) => {
        _Py_RVALUE!(unsafe {
            *((*$S).typed_elements.as_mut_ptr() as *mut *mut _comprehension)
                .wrapping_offset(($I) as isize)
        })
    };
}

macro_rules! asdl_seq_GET__4 {
    ($S:expr, $I:expr) => {
        _Py_RVALUE!(unsafe {
            *((*$S).typed_elements.as_mut_ptr() as *mut *mut _keyword)
                .wrapping_offset(($I) as isize)
        })
    };
}

macro_rules! asdl_seq_LEN__14 {
    ($S:expr) => {
        _Py_RVALUE!(if $S.is_null() {
            0i64
        } else {
            unsafe { (*$S).size }
        })
    };
}

macro_rules! asdl_seq_LEN__4 {
    ($S:expr) => {
        _Py_RVALUE!(if $S.is_null() {
            0i64
        } else {
            unsafe { (*$S).size }
        })
    };
}

macro_rules! asdl_seq_LEN__9 {
    ($S:expr) => {
        _Py_RVALUE!(if $S.is_null() {
            0i64
        } else {
            unsafe { (*$S).size }
        })
    };
}

macro_rules! asdl_seq_LEN__3 {
    ($S:expr) => {
        _Py_RVALUE!(if $S.is_null() {
            0i64
        } else {
            unsafe { (*$S).size }
        })
    };
}

macro_rules! asdl_seq_LEN__8 {
    ($S:expr) => {
        _Py_RVALUE!(if $S.is_null() {
            0i64
        } else {
            unsafe { (*$S).size }
        })
    };
}

macro_rules! asdl_seq_LEN__13 {
    ($S:expr) => {
        _Py_RVALUE!(if $S.is_null() {
            0i64
        } else {
            unsafe { (*$S).size }
        })
    };
}

macro_rules! asdl_seq_LEN__7 {
    ($S:expr) => {
        _Py_RVALUE!(if $S.is_null() {
            0i64
        } else {
            unsafe { (*$S).size }
        })
    };
}

macro_rules! asdl_seq_LEN__10 {
    ($S:expr) => {
        _Py_RVALUE!(if $S.is_null() {
            0i64
        } else {
            unsafe { (*$S).size }
        })
    };
}

macro_rules! asdl_seq_LEN__6 {
    ($S:expr) => {
        _Py_RVALUE!(if $S.is_null() {
            0i64
        } else {
            unsafe { (*$S).size }
        })
    };
}

macro_rules! asdl_seq_LEN {
    ($S:expr) => {
        _Py_RVALUE!(if $S.is_null() {
            0i64
        } else {
            unsafe { (*$S).size }
        })
    };
}

macro_rules! asdl_seq_LEN__11 {
    ($S:expr) => {
        _Py_RVALUE!(if $S.is_null() {
            0i64
        } else {
            unsafe { (*$S).size }
        })
    };
}

macro_rules! asdl_seq_LEN__12 {
    ($S:expr) => {
        _Py_RVALUE!(if $S.is_null() {
            0i64
        } else {
            unsafe { (*$S).size }
        })
    };
}

macro_rules! asdl_seq_LEN__5 {
    ($S:expr) => {
        _Py_RVALUE!(if $S.is_null() {
            0i64
        } else {
            unsafe { (*$S).size }
        })
    };
}

macro_rules! asdl_seq_LEN__2 {
    ($S:expr) => {
        _Py_RVALUE!(if $S.is_null() {
            0i64
        } else {
            unsafe { (*$S).size }
        })
    };
}

macro_rules! validate_nonempty_seq {
    ($seq:expr, $what:expr, $owner:expr) => {
        _validate_nonempty_seq(
            ($seq) as *mut __SlateRecord_pycore_asdl_h_30,
            ($what) as *const i8,
            ($owner) as *const i8,
        )
    };
}

macro_rules! validate_nonempty_seq__2 {
    ($seq:expr, $what:expr, $owner:expr) => {
        _validate_nonempty_seq(($seq) as *mut __SlateRecord_pycore_asdl_h_30, $what, $owner)
    };
}

macro_rules! validate_nonempty_seq__3 {
    ($seq:expr, $what:expr, $owner:expr) => {
        _validate_nonempty_seq(
            ($seq) as *mut __SlateRecord_pycore_asdl_h_30,
            ($what) as *const i8,
            $owner,
        )
    };
}

unsafe extern "C" {
    static mut PyLong_Type: _typeobject;
    static mut PyBool_Type: _typeobject;
    static mut _Py_NoneStruct: _object;
    static mut PyBytes_Type: _typeobject;
    static mut PyUnicode_Type: _typeobject;
    static mut PyExc_SystemError: *mut _object;
    static mut PyExc_TypeError: *mut _object;
    static mut PyExc_ValueError: *mut _object;
    static mut PyFloat_Type: _typeobject;
    static mut PyComplex_Type: _typeobject;
    static mut PyTuple_Type: _typeobject;
    static mut PyFrozenSet_Type: _typeobject;
    static mut _Py_EllipsisObject: _object;
    fn exp(arg0: f64) -> f64;
    fn _PyType_Name(arg0: *mut _typeobject) -> *const i8;
    fn _Py_Dealloc(arg0: *mut _object);
    fn PyErr_SetString(arg0: *mut _object, arg1: *const i8);
    fn PyErr_Occurred() -> *mut _object;
    fn PyErr_Format(arg0: *mut _object, arg1: *const i8, ...) -> *mut _object;
    fn Py_EnterRecursiveCall(arg0: *const i8) -> i32;
    fn Py_LeaveRecursiveCall();
    fn PyObject_GetIter(arg0: *mut _object) -> *mut _object;
    fn PyIter_Next(arg0: *mut _object) -> *mut _object;
    fn _PyUnicode_EqualToASCIIString(arg0: *mut _object, arg1: *const i8) -> i32;
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

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_asdl_h_30 {
    size: i64,
    elements: *mut *mut (),
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_asdl_h_40 {
    size: i64,
    elements: *mut *mut (),
    typed_elements: [*mut _object; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_asdl_h_45 {
    size: i64,
    elements: *mut *mut (),
    typed_elements: [i32; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_67 {
    size: i64,
    elements: *mut *mut (),
    typed_elements: [*mut _stmt; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_74 {
    size: i64,
    elements: *mut *mut (),
    typed_elements: [*mut _expr; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_81 {
    size: i64,
    elements: *mut *mut (),
    typed_elements: [*mut _comprehension; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_89 {
    size: i64,
    elements: *mut *mut (),
    typed_elements: [*mut _excepthandler; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_104 {
    size: i64,
    elements: *mut *mut (),
    typed_elements: [*mut _arg; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_111 {
    size: i64,
    elements: *mut *mut (),
    typed_elements: [*mut _keyword; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_118 {
    size: i64,
    elements: *mut *mut (),
    typed_elements: [*mut _alias; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_125 {
    size: i64,
    elements: *mut *mut (),
    typed_elements: [*mut _withitem; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_132 {
    size: i64,
    elements: *mut *mut (),
    typed_elements: [*mut _match_case; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_140 {
    size: i64,
    elements: *mut *mut (),
    typed_elements: [*mut _pattern; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_147 {
    size: i64,
    elements: *mut *mut (),
    typed_elements: [*mut _type_ignore; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_155 {
    size: i64,
    elements: *mut *mut (),
    typed_elements: [*mut _type_param; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _mod {
    kind: u32,
    v: __SlateRecord_pycore_ast_h_165,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _stmt {
    kind: u32,
    v: __SlateRecord_pycore_ast_h_198,
    lineno: i32,
    col_offset: i32,
    end_lineno: i32,
    end_col_offset: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _expr {
    kind: u32,
    v: __SlateRecord_pycore_ast_h_370,
    lineno: i32,
    col_offset: i32,
    end_lineno: i32,
    end_col_offset: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _comprehension {
    target: *mut _expr,
    iter: *mut _expr,
    ifs: *mut __SlateRecord_pycore_ast_h_74,
    is_async: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _excepthandler {
    kind: u32,
    v: __SlateRecord_pycore_ast_h_538,
    lineno: i32,
    col_offset: i32,
    end_lineno: i32,
    end_col_offset: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _arguments {
    posonlyargs: *mut __SlateRecord_pycore_ast_h_104,
    args: *mut __SlateRecord_pycore_ast_h_104,
    vararg: *mut _arg,
    kwonlyargs: *mut __SlateRecord_pycore_ast_h_104,
    kw_defaults: *mut __SlateRecord_pycore_ast_h_74,
    kwarg: *mut _arg,
    defaults: *mut __SlateRecord_pycore_ast_h_74,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _arg {
    arg: *mut _object,
    annotation: *mut _expr,
    type_comment: *mut _object,
    lineno: i32,
    col_offset: i32,
    end_lineno: i32,
    end_col_offset: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _keyword {
    arg: *mut _object,
    value: *mut _expr,
    lineno: i32,
    col_offset: i32,
    end_lineno: i32,
    end_col_offset: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _alias {
    name: *mut _object,
    asname: *mut _object,
    lineno: i32,
    col_offset: i32,
    end_lineno: i32,
    end_col_offset: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _withitem {
    context_expr: *mut _expr,
    optional_vars: *mut _expr,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord_pycore_ast_h_165 {
    Module: __SlateRecord_pycore_ast_h_166,
    Interactive: __SlateRecord_pycore_ast_h_171,
    Expression: __SlateRecord_pycore_ast_h_175,
    FunctionType: __SlateRecord_pycore_ast_h_179,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_166 {
    body: *mut __SlateRecord_pycore_ast_h_67,
    type_ignores: *mut __SlateRecord_pycore_ast_h_147,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_171 {
    body: *mut __SlateRecord_pycore_ast_h_67,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_175 {
    body: *mut _expr,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_179 {
    argtypes: *mut __SlateRecord_pycore_ast_h_74,
    returns: *mut _expr,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord_pycore_ast_h_198 {
    FunctionDef: __SlateRecord_pycore_ast_h_199,
    AsyncFunctionDef: __SlateRecord_pycore_ast_h_209,
    ClassDef: __SlateRecord_pycore_ast_h_219,
    Return: __SlateRecord_pycore_ast_h_228,
    Delete: __SlateRecord_pycore_ast_h_232,
    Assign: __SlateRecord_pycore_ast_h_236,
    TypeAlias: __SlateRecord_pycore_ast_h_242,
    AugAssign: __SlateRecord_pycore_ast_h_248,
    AnnAssign: __SlateRecord_pycore_ast_h_254,
    For: __SlateRecord_pycore_ast_h_261,
    AsyncFor: __SlateRecord_pycore_ast_h_269,
    While: __SlateRecord_pycore_ast_h_277,
    If: __SlateRecord_pycore_ast_h_283,
    With: __SlateRecord_pycore_ast_h_289,
    AsyncWith: __SlateRecord_pycore_ast_h_295,
    Match: __SlateRecord_pycore_ast_h_301,
    Raise: __SlateRecord_pycore_ast_h_306,
    Try: __SlateRecord_pycore_ast_h_311,
    TryStar: __SlateRecord_pycore_ast_h_318,
    Assert: __SlateRecord_pycore_ast_h_325,
    Import: __SlateRecord_pycore_ast_h_330,
    ImportFrom: __SlateRecord_pycore_ast_h_334,
    Global: __SlateRecord_pycore_ast_h_340,
    Nonlocal: __SlateRecord_pycore_ast_h_344,
    Expr: __SlateRecord_pycore_ast_h_348,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_199 {
    name: *mut _object,
    args: *mut _arguments,
    body: *mut __SlateRecord_pycore_ast_h_67,
    decorator_list: *mut __SlateRecord_pycore_ast_h_74,
    returns: *mut _expr,
    type_comment: *mut _object,
    type_params: *mut __SlateRecord_pycore_ast_h_155,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_209 {
    name: *mut _object,
    args: *mut _arguments,
    body: *mut __SlateRecord_pycore_ast_h_67,
    decorator_list: *mut __SlateRecord_pycore_ast_h_74,
    returns: *mut _expr,
    type_comment: *mut _object,
    type_params: *mut __SlateRecord_pycore_ast_h_155,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_219 {
    name: *mut _object,
    bases: *mut __SlateRecord_pycore_ast_h_74,
    keywords: *mut __SlateRecord_pycore_ast_h_111,
    body: *mut __SlateRecord_pycore_ast_h_67,
    decorator_list: *mut __SlateRecord_pycore_ast_h_74,
    type_params: *mut __SlateRecord_pycore_ast_h_155,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_228 {
    value: *mut _expr,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_232 {
    targets: *mut __SlateRecord_pycore_ast_h_74,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_236 {
    targets: *mut __SlateRecord_pycore_ast_h_74,
    value: *mut _expr,
    type_comment: *mut _object,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_242 {
    name: *mut _expr,
    type_params: *mut __SlateRecord_pycore_ast_h_155,
    value: *mut _expr,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_248 {
    target: *mut _expr,
    op: u32,
    value: *mut _expr,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_254 {
    target: *mut _expr,
    annotation: *mut _expr,
    value: *mut _expr,
    simple: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_261 {
    target: *mut _expr,
    iter: *mut _expr,
    body: *mut __SlateRecord_pycore_ast_h_67,
    orelse: *mut __SlateRecord_pycore_ast_h_67,
    type_comment: *mut _object,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_269 {
    target: *mut _expr,
    iter: *mut _expr,
    body: *mut __SlateRecord_pycore_ast_h_67,
    orelse: *mut __SlateRecord_pycore_ast_h_67,
    type_comment: *mut _object,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_277 {
    test: *mut _expr,
    body: *mut __SlateRecord_pycore_ast_h_67,
    orelse: *mut __SlateRecord_pycore_ast_h_67,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_283 {
    test: *mut _expr,
    body: *mut __SlateRecord_pycore_ast_h_67,
    orelse: *mut __SlateRecord_pycore_ast_h_67,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_289 {
    items: *mut __SlateRecord_pycore_ast_h_125,
    body: *mut __SlateRecord_pycore_ast_h_67,
    type_comment: *mut _object,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_295 {
    items: *mut __SlateRecord_pycore_ast_h_125,
    body: *mut __SlateRecord_pycore_ast_h_67,
    type_comment: *mut _object,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_301 {
    subject: *mut _expr,
    cases: *mut __SlateRecord_pycore_ast_h_132,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_306 {
    exc: *mut _expr,
    cause: *mut _expr,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_311 {
    body: *mut __SlateRecord_pycore_ast_h_67,
    handlers: *mut __SlateRecord_pycore_ast_h_89,
    orelse: *mut __SlateRecord_pycore_ast_h_67,
    finalbody: *mut __SlateRecord_pycore_ast_h_67,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_318 {
    body: *mut __SlateRecord_pycore_ast_h_67,
    handlers: *mut __SlateRecord_pycore_ast_h_89,
    orelse: *mut __SlateRecord_pycore_ast_h_67,
    finalbody: *mut __SlateRecord_pycore_ast_h_67,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_325 {
    test: *mut _expr,
    msg: *mut _expr,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_330 {
    names: *mut __SlateRecord_pycore_ast_h_118,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_334 {
    module: *mut _object,
    names: *mut __SlateRecord_pycore_ast_h_118,
    level: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_340 {
    names: *mut __SlateRecord_pycore_asdl_h_40,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_344 {
    names: *mut __SlateRecord_pycore_asdl_h_40,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_348 {
    value: *mut _expr,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord_pycore_ast_h_370 {
    BoolOp: __SlateRecord_pycore_ast_h_371,
    NamedExpr: __SlateRecord_pycore_ast_h_376,
    BinOp: __SlateRecord_pycore_ast_h_381,
    UnaryOp: __SlateRecord_pycore_ast_h_387,
    Lambda: __SlateRecord_pycore_ast_h_392,
    IfExp: __SlateRecord_pycore_ast_h_397,
    Dict: __SlateRecord_pycore_ast_h_403,
    Set: __SlateRecord_pycore_ast_h_408,
    ListComp: __SlateRecord_pycore_ast_h_412,
    SetComp: __SlateRecord_pycore_ast_h_417,
    DictComp: __SlateRecord_pycore_ast_h_422,
    GeneratorExp: __SlateRecord_pycore_ast_h_428,
    Await: __SlateRecord_pycore_ast_h_433,
    Yield: __SlateRecord_pycore_ast_h_437,
    YieldFrom: __SlateRecord_pycore_ast_h_441,
    Compare: __SlateRecord_pycore_ast_h_445,
    Call: __SlateRecord_pycore_ast_h_451,
    FormattedValue: __SlateRecord_pycore_ast_h_457,
    Interpolation: __SlateRecord_pycore_ast_h_463,
    JoinedStr: __SlateRecord_pycore_ast_h_470,
    TemplateStr: __SlateRecord_pycore_ast_h_474,
    Constant: __SlateRecord_pycore_ast_h_478,
    Attribute: __SlateRecord_pycore_ast_h_483,
    Subscript: __SlateRecord_pycore_ast_h_489,
    Starred: __SlateRecord_pycore_ast_h_495,
    Name: __SlateRecord_pycore_ast_h_500,
    List: __SlateRecord_pycore_ast_h_505,
    Tuple: __SlateRecord_pycore_ast_h_510,
    Slice: __SlateRecord_pycore_ast_h_515,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_371 {
    op: u32,
    values: *mut __SlateRecord_pycore_ast_h_74,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_376 {
    target: *mut _expr,
    value: *mut _expr,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_381 {
    left: *mut _expr,
    op: u32,
    right: *mut _expr,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_387 {
    op: u32,
    operand: *mut _expr,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_392 {
    args: *mut _arguments,
    body: *mut _expr,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_397 {
    test: *mut _expr,
    body: *mut _expr,
    orelse: *mut _expr,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_403 {
    keys: *mut __SlateRecord_pycore_ast_h_74,
    values: *mut __SlateRecord_pycore_ast_h_74,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_408 {
    elts: *mut __SlateRecord_pycore_ast_h_74,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_412 {
    elt: *mut _expr,
    generators: *mut __SlateRecord_pycore_ast_h_81,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_417 {
    elt: *mut _expr,
    generators: *mut __SlateRecord_pycore_ast_h_81,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_422 {
    key: *mut _expr,
    value: *mut _expr,
    generators: *mut __SlateRecord_pycore_ast_h_81,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_428 {
    elt: *mut _expr,
    generators: *mut __SlateRecord_pycore_ast_h_81,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_433 {
    value: *mut _expr,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_437 {
    value: *mut _expr,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_441 {
    value: *mut _expr,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_445 {
    left: *mut _expr,
    ops: *mut __SlateRecord_pycore_asdl_h_45,
    comparators: *mut __SlateRecord_pycore_ast_h_74,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_451 {
    func: *mut _expr,
    args: *mut __SlateRecord_pycore_ast_h_74,
    keywords: *mut __SlateRecord_pycore_ast_h_111,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_457 {
    value: *mut _expr,
    conversion: i32,
    format_spec: *mut _expr,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_463 {
    value: *mut _expr,
    str: *mut _object,
    conversion: i32,
    format_spec: *mut _expr,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_470 {
    values: *mut __SlateRecord_pycore_ast_h_74,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_474 {
    values: *mut __SlateRecord_pycore_ast_h_74,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_478 {
    value: *mut _object,
    kind: *mut _object,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_483 {
    value: *mut _expr,
    attr: *mut _object,
    ctx: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_489 {
    value: *mut _expr,
    slice: *mut _expr,
    ctx: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_495 {
    value: *mut _expr,
    ctx: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_500 {
    id: *mut _object,
    ctx: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_505 {
    elts: *mut __SlateRecord_pycore_ast_h_74,
    ctx: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_510 {
    elts: *mut __SlateRecord_pycore_ast_h_74,
    ctx: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_515 {
    lower: *mut _expr,
    upper: *mut _expr,
    step: *mut _expr,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord_pycore_ast_h_538 {
    ExceptHandler: __SlateRecord_pycore_ast_h_539,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_539 {
    r#type: *mut _expr,
    name: *mut _object,
    body: *mut __SlateRecord_pycore_ast_h_67,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord_pycore_ast_h_607 {
    MatchValue: __SlateRecord_pycore_ast_h_608,
    MatchSingleton: __SlateRecord_pycore_ast_h_612,
    MatchSequence: __SlateRecord_pycore_ast_h_616,
    MatchMapping: __SlateRecord_pycore_ast_h_620,
    MatchClass: __SlateRecord_pycore_ast_h_626,
    MatchStar: __SlateRecord_pycore_ast_h_633,
    MatchAs: __SlateRecord_pycore_ast_h_637,
    MatchOr: __SlateRecord_pycore_ast_h_642,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_608 {
    value: *mut _expr,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_612 {
    value: *mut _object,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_616 {
    patterns: *mut __SlateRecord_pycore_ast_h_140,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_620 {
    keys: *mut __SlateRecord_pycore_ast_h_74,
    patterns: *mut __SlateRecord_pycore_ast_h_140,
    rest: *mut _object,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_626 {
    cls: *mut _expr,
    patterns: *mut __SlateRecord_pycore_ast_h_140,
    kwd_attrs: *mut __SlateRecord_pycore_asdl_h_40,
    kwd_patterns: *mut __SlateRecord_pycore_ast_h_140,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_633 {
    name: *mut _object,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_637 {
    pattern: *mut _pattern,
    name: *mut _object,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_642 {
    patterns: *mut __SlateRecord_pycore_ast_h_140,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord_pycore_ast_h_656 {
    TypeIgnore: __SlateRecord_pycore_ast_h_657,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_657 {
    lineno: i32,
    tag: *mut _object,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord_pycore_ast_h_668 {
    TypeVar: __SlateRecord_pycore_ast_h_669,
    ParamSpec: __SlateRecord_pycore_ast_h_675,
    TypeVarTuple: __SlateRecord_pycore_ast_h_680,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_669 {
    name: *mut _object,
    bound: *mut _expr,
    default_value: *mut _expr,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_675 {
    name: *mut _object,
    default_value: *mut _expr,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_680 {
    name: *mut _object,
    default_value: *mut _expr,
}

#[repr(C, align(16))]
struct __SlateAlign16<T>(T);

unsafe impl<T> Sync for __SlateAlign16<T> {}

#[repr(C)]
#[derive(Clone, Copy)]
struct _match_case {
    pattern: *mut _pattern,
    guard: *mut _expr,
    body: *mut __SlateRecord_pycore_ast_h_67,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _pattern {
    kind: u32,
    v: __SlateRecord_pycore_ast_h_607,
    lineno: i32,
    col_offset: i32,
    end_lineno: i32,
    end_col_offset: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _type_ignore {
    kind: u32,
    v: __SlateRecord_pycore_ast_h_656,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _type_param {
    kind: u32,
    v: __SlateRecord_pycore_ast_h_668,
    lineno: i32,
    col_offset: i32,
    end_lineno: i32,
    end_col_offset: i32,
}

// asdl_stmt_seq
// _PyThreadState_GET()
// _PyUnicode_EqualToASCIIString()
// bool
fn validate_name(name: *mut _object) -> i32 {
    let mut i: i32 = 0;
    while !unsafe { forbidden.0[i as usize] }.is_null() {
        if (unsafe { _PyUnicode_EqualToASCIIString(name, forbidden.0[i as usize]) }) != 0 {
            unsafe {
                PyErr_Format(
                    PyExc_ValueError,
                    c"identifier field can't represent '%s' constant".as_ptr(),
                    forbidden.0[i as usize],
                )
            };
            return 0;
        }
        i = i.wrapping_add(1);
    }
    1
}

static forbidden: __SlateAlign16<[*const i8; 4]> = __SlateAlign16([
    (b"None\0".as_ptr() as *mut i8) as *const i8,
    (b"True\0".as_ptr() as *mut i8) as *const i8,
    (b"False\0".as_ptr() as *mut i8) as *const i8,
    core::ptr::null::<i8>(),
]);

fn validate_comprehension(gens: *mut __SlateRecord_pycore_ast_h_81) -> i32 {
    if asdl_seq_LEN__9!(gens) == 0 {
        unsafe {
            PyErr_SetString(
                PyExc_ValueError,
                c"comprehension with no generators".as_ptr(),
            )
        };
        return 0;
    }
    let mut i: i64 = 0;
    while i < asdl_seq_LEN__9!(gens) {
        let comp: *mut _comprehension = asdl_seq_GET__5!(gens, i);
        let __v0: bool = validate_expr(unsafe { (*comp).target }, 2u32) == 0
            || validate_expr(unsafe { (*comp).iter }, 1u32) == 0;
        let __v1: bool = __v0 || validate_exprs(unsafe { (*comp).ifs }, 1u32, 0i32) == 0;
        if __v1 {
            return 0;
        }
        i = i.wrapping_add(1);
    }
    1
}

fn validate_keywords(keywords: *mut __SlateRecord_pycore_ast_h_111) -> i32 {
    let mut i: i64 = 0;
    while i < asdl_seq_LEN__7!(keywords) {
        if validate_expr(unsafe { (*asdl_seq_GET__4!(keywords, i)).value }, 1u32) == 0 {
            return 0;
        }
        i = i.wrapping_add(1);
    }
    1
}

fn validate_args(args: *mut __SlateRecord_pycore_ast_h_104) -> i32 {
    let mut i: i64 = 0;
    while i < asdl_seq_LEN__4!(args) {
        let arg: *mut _arg = asdl_seq_GET__6!(args, i);
        VALIDATE_POSITIONS!(arg);
        let __v0: bool = !unsafe { (*arg).annotation }.is_null()
            && validate_expr(unsafe { (*arg).annotation }, 1u32) == 0;
        if __v0 {
            return 0;
        }
        i = i.wrapping_add(1);
    }
    1
}

fn expr_context_name(ctx: u32) -> *const i8 {
    match ctx as u32 {
        1 => {
            return c"Load".as_ptr();
        }
        2 => {
            return c"Store".as_ptr();
        }
        3 => {
            return c"Del".as_ptr();
            // No default case so compiler emits warning for unhandled cases
        }
        _ => {}
    }
    Py_UNREACHABLE!();
    unsafe { core::mem::zeroed() }
}

fn validate_arguments(args: *mut _arguments) -> i32 {
    let __v0: bool = validate_args(unsafe { (*args).posonlyargs }) == 0
        || validate_args(unsafe { (*args).args }) == 0;
    if __v0 {
        return 0;
    }
    let __v1: bool = !unsafe { (*args).vararg }.is_null()
        && !unsafe { (*(*args).vararg).annotation }.is_null()
        && validate_expr(unsafe { (*(*args).vararg).annotation }, 1u32) == 0;
    if __v1 {
        return 0;
    }
    if validate_args(unsafe { (*args).kwonlyargs }) == 0 {
        return 0;
    }
    let __v2: bool = !unsafe { (*args).kwarg }.is_null()
        && !unsafe { (*(*args).kwarg).annotation }.is_null()
        && validate_expr(unsafe { (*(*args).kwarg).annotation }, 1u32) == 0;
    if __v2 {
        return 0;
    }
    if asdl_seq_LEN!(unsafe { (*args).defaults })
        > asdl_seq_LEN__4!(unsafe { (*args).posonlyargs })
            .wrapping_add(asdl_seq_LEN__4!(unsafe { (*args).args }))
    {
        unsafe {
            PyErr_SetString(
                PyExc_ValueError,
                c"more positional defaults than args on arguments".as_ptr(),
            )
        };
        return 0;
    }
    if asdl_seq_LEN!(unsafe { (*args).kw_defaults })
        != asdl_seq_LEN__4!(unsafe { (*args).kwonlyargs })
    {
        unsafe {
            PyErr_SetString(
                PyExc_ValueError,
                c"length of kwonlyargs is not the same as kw_defaults on arguments".as_ptr(),
            )
        };
        return 0;
    }
    let __v3: bool = validate_exprs(unsafe { (*args).defaults }, 1u32, 0i32) != 0
        && validate_exprs(unsafe { (*args).kw_defaults }, 1u32, 1i32) != 0;
    __v3 as i32
}

fn validate_constant(value: *mut _object) -> i32 {
    if value == unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
        || value == unsafe { core::ptr::addr_of_mut!(_Py_EllipsisObject) }
    {
        return 1;
    }
    let __v0: bool = PyLong_CheckExact!(value) != 0 || PyFloat_CheckExact!(value) != 0;
    let __v1: bool = __v0 || PyComplex_CheckExact!(value) != 0;
    let __v2: bool = __v1 || PyBool_Check!(value) != 0;
    let __v3: bool = __v2 || PyUnicode_CheckExact!(value) != 0;
    let __v4: bool = __v3 || PyBytes_CheckExact!(value) != 0;
    if __v4 {
        return 1;
    }
    let __v5: bool = PyTuple_CheckExact!(value) != 0 || PyFrozenSet_CheckExact!(value) != 0;
    if __v5 {
        ENTER_RECURSIVE!();
        let it: *mut _object = unsafe { PyObject_GetIter(value) };
        if it.is_null() {
            return 0;
        }
        while true {
            let item: *mut _object = unsafe { PyIter_Next(it) };
            if item.is_null() {
                if !unsafe { PyErr_Occurred() }.is_null() {
                    Py_DECREF!(it);
                    return 0;
                }
                break;
            }
            if validate_constant(item) == 0 {
                Py_DECREF!(it);
                Py_DECREF!(item);
                return 0;
            }
            Py_DECREF!(item);
        }
        Py_DECREF!(it);
        LEAVE_RECURSIVE!();
        return 1;
    }
    if unsafe { PyErr_Occurred() }.is_null() {
        unsafe {
            PyErr_Format(
                PyExc_TypeError,
                c"got an invalid type in Constant: %s".as_ptr(),
                _PyType_Name(Py_TYPE!(value)),
            )
        };
    }
    0
}

fn validate_expr(exp: *mut _expr, ctx: u32) -> i32 {
    VALIDATE_POSITIONS!(exp);
    let mut ret: i32 = 1i32.wrapping_neg();
    ENTER_RECURSIVE!();
    let mut check_ctx: i32 = 1;
    let mut actual_ctx: u32 = 0;
    // First check expression context.
    match (unsafe { (*exp).kind }) as u32 {
        23 => {
            actual_ctx = unsafe { (*exp).v.Attribute.ctx };
        }
        24 => {
            actual_ctx = unsafe { (*exp).v.Subscript.ctx };
        }
        25 => {
            actual_ctx = unsafe { (*exp).v.Starred.ctx };
        }
        26 => {
            if validate_name(unsafe { (*exp).v.Name.id }) == 0 {
                return 0;
            }
            actual_ctx = unsafe { (*exp).v.Name.ctx };
        }
        27 => {
            actual_ctx = unsafe { (*exp).v.List.ctx };
        }
        28 => {
            actual_ctx = unsafe { (*exp).v.Tuple.ctx };
        }
        _ => {
            if (ctx as u32) != 1 {
                unsafe {
                    PyErr_Format(
                        PyExc_ValueError,
                        c"expression which can't be assigned to in %s context".as_ptr(),
                        expr_context_name(ctx),
                    )
                };
                return 0;
            }
            check_ctx = 0;
            // set actual_ctx to prevent gcc warning
            actual_ctx = 0;
        }
    }
    if check_ctx != 0 && (actual_ctx as u32) != (ctx as u32) {
        unsafe {
            PyErr_Format(
                PyExc_ValueError,
                c"expression must have %s context but has %s instead".as_ptr(),
                expr_context_name(ctx),
                expr_context_name(actual_ctx),
            )
        };
        return 0;
    }
    // Now validate expression.
    '__slate_break_1: {
        match (unsafe { (*exp).kind }) as u32 {
            1 => {
                if asdl_seq_LEN!(unsafe { (*exp).v.BoolOp.values }) < 2 {
                    unsafe {
                        PyErr_SetString(
                            PyExc_ValueError,
                            c"BoolOp with less than 2 values".as_ptr(),
                        )
                    };
                    return 0;
                }
                ret = validate_exprs(unsafe { (*exp).v.BoolOp.values }, 1u32, 0i32);
            }
            3 => {
                let __v2: bool = validate_expr(unsafe { (*exp).v.BinOp.left }, 1u32) != 0
                    && validate_expr(unsafe { (*exp).v.BinOp.right }, 1u32) != 0;
                ret = __v2 as i32;
            }
            4 => {
                ret = validate_expr(unsafe { (*exp).v.UnaryOp.operand }, 1u32);
            }
            5 => {
                let __v3: bool = validate_arguments(unsafe { (*exp).v.Lambda.args }) != 0
                    && validate_expr(unsafe { (*exp).v.Lambda.body }, 1u32) != 0;
                ret = __v3 as i32;
            }
            6 => {
                let __v4: bool = validate_expr(unsafe { (*exp).v.IfExp.test }, 1u32) != 0
                    && validate_expr(unsafe { (*exp).v.IfExp.body }, 1u32) != 0;
                let __v5: bool = __v4 && validate_expr(unsafe { (*exp).v.IfExp.orelse }, 1u32) != 0;
                ret = __v5 as i32;
            }
            7 => {
                if asdl_seq_LEN!(unsafe { (*exp).v.Dict.keys })
                    != asdl_seq_LEN!(unsafe { (*exp).v.Dict.values })
                {
                    unsafe {
                        PyErr_SetString(
                            PyExc_ValueError,
                            c"Dict doesn't have the same number of keys as values".as_ptr(),
                        )
                    };
                    return 0;
                }
                // null_ok=1 for keys expressions to allow dict unpacking to work in
                // dict literals, i.e. ``{**{a:b}}``
                let __v6: bool = validate_exprs(unsafe { (*exp).v.Dict.keys }, 1u32, 1i32) != 0
                    && validate_exprs(unsafe { (*exp).v.Dict.values }, 1u32, 0i32) != 0;
                ret = __v6 as i32; // null_ok=
                // null_ok=
            }
            8 => {
                ret = validate_exprs(unsafe { (*exp).v.Set.elts }, 1u32, 0i32);
            }
            9 => {
                let __v7: bool = validate_comprehension(unsafe { (*exp).v.ListComp.generators })
                    != 0
                    && validate_expr(unsafe { (*exp).v.ListComp.elt }, 1u32) != 0;
                ret = __v7 as i32;
            }
            10 => {
                let __v8: bool = validate_comprehension(unsafe { (*exp).v.SetComp.generators })
                    != 0
                    && validate_expr(unsafe { (*exp).v.SetComp.elt }, 1u32) != 0;
                ret = __v8 as i32;
            }
            12 => {
                let __v9: bool =
                    validate_comprehension(unsafe { (*exp).v.GeneratorExp.generators }) != 0
                        && validate_expr(unsafe { (*exp).v.GeneratorExp.elt }, 1u32) != 0;
                ret = __v9 as i32;
            }
            11 => {
                let __v10: bool = validate_comprehension(unsafe { (*exp).v.DictComp.generators })
                    != 0
                    && validate_expr(unsafe { (*exp).v.DictComp.key }, 1u32) != 0;
                let __v11: bool =
                    __v10 && validate_expr(unsafe { (*exp).v.DictComp.value }, 1u32) != 0;
                ret = __v11 as i32;
            }
            14 => {
                let __v12: bool = unsafe { (*exp).v.Yield.value }.is_null()
                    || validate_expr(unsafe { (*exp).v.Yield.value }, 1u32) != 0;
                ret = __v12 as i32;
            }
            15 => {
                ret = validate_expr(unsafe { (*exp).v.YieldFrom.value }, 1u32);
            }
            13 => {
                ret = validate_expr(unsafe { (*exp).v.Await.value }, 1u32);
            }
            16 => {
                if asdl_seq_LEN!(unsafe { (*exp).v.Compare.comparators }) == 0 {
                    unsafe {
                        PyErr_SetString(PyExc_ValueError, c"Compare with no comparators".as_ptr())
                    };
                    return 0;
                }
                if asdl_seq_LEN!(unsafe { (*exp).v.Compare.comparators })
                    != asdl_seq_LEN__13!(unsafe { (*exp).v.Compare.ops })
                {
                    unsafe {
                        PyErr_SetString(
                            PyExc_ValueError,
                            c"Compare has a different number of comparators and operands".as_ptr(),
                        )
                    };
                    return 0;
                }
                let __v13: bool =
                    validate_exprs(unsafe { (*exp).v.Compare.comparators }, 1u32, 0i32) != 0
                        && validate_expr(unsafe { (*exp).v.Compare.left }, 1u32) != 0;
                ret = __v13 as i32;
            }
            17 => {
                let __v14: bool = validate_expr(unsafe { (*exp).v.Call.func }, 1u32) != 0
                    && validate_exprs(unsafe { (*exp).v.Call.args }, 1u32, 0i32) != 0;
                let __v15: bool =
                    __v14 && validate_keywords(unsafe { (*exp).v.Call.keywords }) != 0;
                ret = __v15 as i32;
            }
            22 => {
                if validate_constant(unsafe { (*exp).v.Constant.value }) == 0 {
                    return 0;
                }
                ret = 1;
            }
            20 => {
                ret = validate_exprs(unsafe { (*exp).v.JoinedStr.values }, 1u32, 0i32);
            }
            21 => {
                ret = validate_exprs(unsafe { (*exp).v.TemplateStr.values }, 1u32, 0i32);
            }
            18 => {
                if validate_expr(unsafe { (*exp).v.FormattedValue.value }, 1u32) == 0 {
                    return 0;
                }
                if !unsafe { (*exp).v.FormattedValue.format_spec }.is_null() {
                    ret = validate_expr(unsafe { (*exp).v.FormattedValue.format_spec }, 1u32);
                } else {
                    ret = 1;
                }
            }
            19 => {
                if validate_expr(unsafe { (*exp).v.Interpolation.value }, 1u32) == 0 {
                    return 0;
                }
                if !unsafe { (*exp).v.Interpolation.format_spec }.is_null() {
                    ret = validate_expr(unsafe { (*exp).v.Interpolation.format_spec }, 1u32);
                } else {
                    ret = 1;
                }
            }
            23 => {
                ret = validate_expr(unsafe { (*exp).v.Attribute.value }, 1u32);
            }
            24 => {
                let __v16: bool = validate_expr(unsafe { (*exp).v.Subscript.slice }, 1u32) != 0
                    && validate_expr(unsafe { (*exp).v.Subscript.value }, 1u32) != 0;
                ret = __v16 as i32;
            }
            25 => {
                ret = validate_expr(unsafe { (*exp).v.Starred.value }, ctx);
            }
            29 => {
                let __v17: bool = unsafe { (*exp).v.Slice.lower }.is_null()
                    || validate_expr(unsafe { (*exp).v.Slice.lower }, 1u32) != 0;
                let __v18: bool = __v17
                    && (unsafe { (*exp).v.Slice.upper }.is_null()
                        || validate_expr(unsafe { (*exp).v.Slice.upper }, 1u32) != 0);
                let __v20: bool = __v18
                    && (unsafe { (*exp).v.Slice.step }.is_null()
                        || validate_expr(unsafe { (*exp).v.Slice.step }, 1u32) != 0);
                ret = __v20 as i32;
            }
            27 => {
                ret = validate_exprs(unsafe { (*exp).v.List.elts }, ctx, 0i32);
            }
            28 => {
                ret = validate_exprs(unsafe { (*exp).v.Tuple.elts }, ctx, 0i32);
            }
            2 => {
                if ((unsafe { (*(*exp).v.NamedExpr.target).kind }) as u32) != 26 {
                    unsafe {
                        PyErr_SetString(
                            PyExc_TypeError,
                            c"NamedExpr target must be a Name".as_ptr(),
                        )
                    };
                    return 0;
                }
                ret = validate_expr(unsafe { (*exp).v.NamedExpr.value }, 1u32);
                break '__slate_break_1;
                // This last case doesn't have any checking.
            }
            26 => {
                ret = 1;
                break '__slate_break_1;
                // No default case so compiler emits warning for unhandled cases
            }
            _ => {}
        }
    }
    if ret < 0 {
        unsafe { PyErr_SetString(PyExc_SystemError, c"unexpected expression".as_ptr()) };
        ret = 0;
    }
    LEAVE_RECURSIVE!();
    ret
}

/// Note: the ensure_literal_* functions are only used to validate a restricted
///
///       set of non-recursive literals that have already been checked with
///
///       validate_expr, so they don't accept the validator state
fn ensure_literal_number(exp: *mut _expr, allow_real: bool, allow_imaginary: bool) -> i32 {
    let value: *mut _object = unsafe { (*exp).v.Constant.value };
    let __v0: bool = allow_real && PyFloat_CheckExact!(value) != 0;
    let __v1: bool = __v0 || allow_real && PyLong_CheckExact!(value) != 0;
    let __v3: bool = __v1 || allow_imaginary && PyComplex_CheckExact!(value) != 0;
    __v3 as i32
}

fn ensure_literal_negative(exp: *mut _expr, allow_real: bool, allow_imaginary: bool) -> i32 {
    // Must be negation ...
    if ((unsafe { (*exp).v.UnaryOp.op }) as u32) != 4 {
        return 0;
    }
    // ... of a constant ...
    let operand: *mut _expr = unsafe { (*exp).v.UnaryOp.operand };
    if ((unsafe { (*operand).kind }) as u32) != 22 {
        return 0;
    }
    // ... number
    ensure_literal_number(operand, allow_real, allow_imaginary)
}

fn ensure_literal_complex(exp: *mut _expr) -> i32 {
    let left: *mut _expr = unsafe { (*exp).v.BinOp.left };
    let right: *mut _expr = unsafe { (*exp).v.BinOp.right };
    // Ensure op is addition or subtraction
    if ((unsafe { (*exp).v.BinOp.op }) as u32) != 1 && ((unsafe { (*exp).v.BinOp.op }) as u32) != 2
    {
        return 0;
    }
    // Check LHS is a real number (potentially signed)
    match (unsafe { (*left).kind }) as u32 {
        22 => {
            if ensure_literal_number(left, true, false) == 0 {
                // real=
                // imaginary=
                return 0;
            }
        }
        4 => {
            if ensure_literal_negative(left, true, false) == 0 {
                // real=
                // imaginary=
                return 0;
            }
        }
        _ => {
            return 0;
        }
    }
    // Check RHS is an imaginary number (no separate sign allowed)
    match (unsafe { (*right).kind }) as u32 {
        22 => {
            if ensure_literal_number(right, false, true) == 0 {
                // real=
                // imaginary=
                return 0;
            }
        }
        _ => {
            return 0;
        }
    }
    1
}

fn validate_pattern_match_value(exp: *mut _expr) -> i32 {
    if validate_expr(exp, 1u32) == 0 {
        return 0;
    }
    match (unsafe { (*exp).kind }) as u32 {
        22 => {
            if validate_expr(exp, 1u32) == 0 {
                return 0;
            }
            // Ellipsis and immutable sequences are not allowed.
            // For True, False and None, MatchSingleton() should
            // be used
            let literal: *mut _object = unsafe { (*exp).v.Constant.value };
            let __v0: bool = PyLong_CheckExact!(literal) != 0 || PyFloat_CheckExact!(literal) != 0;
            let __v1: bool = __v0 || PyBytes_CheckExact!(literal) != 0;
            let __v2: bool = __v1 || PyComplex_CheckExact!(literal) != 0;
            let __v3: bool = __v2 || PyUnicode_CheckExact!(literal) != 0;
            if __v3 {
                return 1;
            }
            unsafe {
                PyErr_SetString(
                    PyExc_ValueError,
                    c"unexpected constant inside of a literal pattern".as_ptr(),
                )
            };
            return 0;
        }
        23 => {
            return 1; // Constants and attribute lookups are always permitted
        }
        4 => {
            if ensure_literal_negative(exp, true, true) != 0 {
                // real=
                // imaginary=
                return 1;
            }
            // Negated numbers are permitted (whether real or imaginary)
            //
            // Compiler will complain if AST folding doesn't create a constant
        }
        3 => {
            if ensure_literal_complex(exp) != 0 {
                return 1;
            }
            // Complex literals are permitted
            //
            // Compiler will complain if AST folding doesn't create a constant
        }
        20 | 21 => {
            return 1; // Handled in the later stages
        }
        _ => {}
    }
    unsafe {
        PyErr_SetString(
            PyExc_ValueError,
            c"patterns may only match literals and attribute lookups".as_ptr(),
        )
    };
    0
}

fn validate_capture(name: *mut _object) -> i32 {
    if (unsafe { _PyUnicode_EqualToASCIIString(name, c"_".as_ptr()) }) != 0 {
        unsafe {
            PyErr_Format(
                PyExc_ValueError,
                c"can't capture name '_' in patterns".as_ptr(),
            )
        };
        return 0;
    }
    validate_name(name)
}

fn validate_pattern(p: *mut _pattern, star_ok: i32) -> i32 {
    VALIDATE_POSITIONS!(p);
    let mut ret: i32 = 1i32.wrapping_neg();
    ENTER_RECURSIVE!();
    '__slate_break_0: {
        match (unsafe { (*p).kind }) as u32 {
            1 => {
                ret = validate_pattern_match_value(unsafe { (*p).v.MatchValue.value });
            }
            2 => {
                let __v1: bool = (unsafe { (*p).v.MatchSingleton.value })
                    == unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
                    || PyBool_Check!(unsafe { (*p).v.MatchSingleton.value }) != 0;
                ret = __v1 as i32;
                if ret == 0 {
                    unsafe {
                        PyErr_SetString(
                            PyExc_ValueError,
                            c"MatchSingleton can only contain True, False and None".as_ptr(),
                        )
                    };
                }
            }
            3 => {
                ret = validate_patterns(unsafe { (*p).v.MatchSequence.patterns }, 1i32); // star_ok=
            }
            4 => {
                if asdl_seq_LEN!(unsafe { (*p).v.MatchMapping.keys })
                    != asdl_seq_LEN__5!(unsafe { (*p).v.MatchMapping.patterns })
                {
                    unsafe {
                        PyErr_SetString(
                            PyExc_ValueError,
                            c"MatchMapping doesn't have the same number of keys as patterns"
                                .as_ptr(),
                        )
                    };
                    ret = 0;
                } else {
                    let __v2: bool = !unsafe { (*p).v.MatchMapping.rest }.is_null()
                        && validate_capture(unsafe { (*p).v.MatchMapping.rest }) == 0;
                    if __v2 {
                        ret = 0;
                    } else {
                        let keys: *mut __SlateRecord_pycore_ast_h_74 =
                            unsafe { (*p).v.MatchMapping.keys };
                        let mut i: i64 = 0;
                        '__slate_break_4: while i < asdl_seq_LEN!(keys) {
                            '__slate_continue_4: {
                                let key: *mut _expr = asdl_seq_GET!(keys, i);
                                if ((unsafe { (*key).kind }) as u32) == 22 {
                                    let literal: *mut _object = unsafe { (*key).v.Constant.value };
                                    let __v3: bool = literal
                                        == unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
                                        || PyBool_Check!(literal) != 0;
                                    if __v3 {
                                        // validate_pattern_match_value will ensure the key
                                        // doesn't contain True, False and None but it is
                                        // syntactically valid, so we will pass those on in
                                        // a special case.
                                        break '__slate_continue_4;
                                    }
                                }
                                if validate_pattern_match_value(key) == 0 {
                                    ret = 0;
                                    break '__slate_break_4;
                                }
                            }
                            i = i.wrapping_add(1);
                        }
                        if ret != 0 {
                            ret = validate_patterns(unsafe { (*p).v.MatchMapping.patterns }, 0i32); // star_ok=
                        }
                    }
                }
            }
            5 => {
                if asdl_seq_LEN__12!(unsafe { (*p).v.MatchClass.kwd_attrs })
                    != asdl_seq_LEN__5!(unsafe { (*p).v.MatchClass.kwd_patterns })
                {
                    unsafe {
                        PyErr_SetString(PyExc_ValueError, c"MatchClass doesn't have the same number of keyword attributes as patterns".as_ptr())
                    };
                    ret = 0;
                } else {
                    if validate_expr(unsafe { (*p).v.MatchClass.cls }, 1u32) == 0 {
                        ret = 0;
                    } else {
                        let mut cls: *mut _expr = unsafe { (*p).v.MatchClass.cls };
                        while true {
                            if ((unsafe { (*cls).kind }) as u32) == 26 {
                                break;
                            } else {
                                if ((unsafe { (*cls).kind }) as u32) == 23 {
                                    cls = unsafe { (*cls).v.Attribute.value };
                                } else {
                                    unsafe {
                                        PyErr_SetString(PyExc_ValueError, c"MatchClass cls field can only contain Name or Attribute nodes.".as_ptr())
                                    };
                                    ret = 0;
                                    break;
                                }
                            }
                        }
                        if ret != 0 {
                            let mut i: i64 = 0;
                            while i < asdl_seq_LEN__12!(unsafe { (*p).v.MatchClass.kwd_attrs }) {
                                let identifier: *mut _object =
                                    asdl_seq_GET__12!(unsafe { (*p).v.MatchClass.kwd_attrs }, i);
                                if validate_name(identifier) == 0 {
                                    ret = 0;
                                    break;
                                }
                                i = i.wrapping_add(1);
                            }
                            if ret != 0 {
                                if validate_patterns(unsafe { (*p).v.MatchClass.patterns }, 0i32)
                                    == 0
                                {
                                    // star_ok=
                                    ret = 0;
                                } else {
                                    ret = validate_patterns(
                                        unsafe { (*p).v.MatchClass.kwd_patterns },
                                        0i32,
                                    ); // star_ok=
                                }
                            }
                        }
                    }
                }
            }
            6 => {
                if star_ok == 0 {
                    unsafe {
                        PyErr_SetString(PyExc_ValueError, c"can't use MatchStar here".as_ptr())
                    };
                    ret = 0;
                } else {
                    let __v11: bool = unsafe { (*p).v.MatchStar.name }.is_null()
                        || validate_capture(unsafe { (*p).v.MatchStar.name }) != 0;
                    ret = __v11 as i32;
                }
            }
            7 => {
                let __v12: bool = !unsafe { (*p).v.MatchAs.name }.is_null()
                    && validate_capture(unsafe { (*p).v.MatchAs.name }) == 0;
                if __v12 {
                    ret = 0;
                } else {
                    if unsafe { (*p).v.MatchAs.pattern }.is_null() {
                        ret = 1;
                    } else {
                        if unsafe { (*p).v.MatchAs.name }.is_null() {
                            unsafe {
                                PyErr_SetString(
                                    PyExc_ValueError,
                                    c"MatchAs must specify a target name if a pattern is given"
                                        .as_ptr(),
                                )
                            };
                            ret = 0;
                        } else {
                            ret = validate_pattern(unsafe { (*p).v.MatchAs.pattern }, 0i32); // star_ok=
                        }
                    }
                }
            }
            8 => {
                if asdl_seq_LEN__5!(unsafe { (*p).v.MatchOr.patterns }) < 2 {
                    unsafe {
                        PyErr_SetString(
                            PyExc_ValueError,
                            c"MatchOr requires at least 2 patterns".as_ptr(),
                        )
                    };
                    ret = 0;
                } else {
                    ret = validate_patterns(unsafe { (*p).v.MatchOr.patterns }, 0i32); // star_ok=
                    break '__slate_break_0;
                    // No default case, so the compiler will emit a warning if new pattern
                    //
                    // kinds are added without being handled here
                }
            }
            _ => {}
        }
    }
    if ret < 0 {
        unsafe { PyErr_SetString(PyExc_SystemError, c"unexpected pattern".as_ptr()) };
        ret = 0;
    }
    LEAVE_RECURSIVE!();
    ret
}

fn _validate_nonempty_seq(
    seq: *mut __SlateRecord_pycore_asdl_h_30,
    what: *const i8,
    owner: *const i8,
) -> i32 {
    if asdl_seq_LEN__3!(seq) != 0 {
        return 1;
    }
    unsafe { PyErr_Format(PyExc_ValueError, c"empty %s on %s".as_ptr(), what, owner) };
    0
}

fn validate_import_names(
    seq: *mut __SlateRecord_pycore_ast_h_118,
    what: *const i8,
    owner: *const i8,
) -> i32 {
    if validate_nonempty_seq__2!(seq, what, owner) == 0 {
        return 0;
    }
    let n: i64 = asdl_seq_LEN__11!(seq);
    for i in 0..n {
        let alias: *mut _alias = asdl_seq_GET__9!(seq, i);
        let __v0: bool = validate_name(unsafe { (*alias).name }) == 0
            || !unsafe { (*alias).asname }.is_null()
                && validate_name(unsafe { (*alias).asname }) == 0;
        if __v0 {
            return 0;
        }
    }
    1
}

fn validate_assignlist(targets: *mut __SlateRecord_pycore_ast_h_74, ctx: u32) -> i32 {
    let __v0: bool = validate_nonempty_seq!(
        targets,
        c"targets".as_ptr() as *mut i8,
        if (ctx as u32) == 3 {
            c"Delete".as_ptr() as *mut i8
        } else {
            c"Assign".as_ptr() as *mut i8
        }
    ) != 0
        && validate_exprs(targets, ctx, 0i32) != 0;
    __v0 as i32
}

fn validate_body(body: *mut __SlateRecord_pycore_ast_h_67, owner: *const i8) -> i32 {
    let __v0: bool = validate_nonempty_seq__3!(body, c"body".as_ptr() as *mut i8, owner) != 0
        && validate_stmts(body) != 0;
    __v0 as i32
}

fn validate_stmt(stmt: *mut _stmt) -> i32 {
    VALIDATE_POSITIONS!(stmt);
    let mut ret: i32 = 1i32.wrapping_neg();
    ENTER_RECURSIVE!();
    '__slate_break_6: {
        match (unsafe { (*stmt).kind }) as u32 {
            1 => {
                let __v0: bool = validate_body(
                    unsafe { (*stmt).v.FunctionDef.body },
                    c"FunctionDef".as_ptr(),
                ) != 0
                    && validate_name(unsafe { (*stmt).v.FunctionDef.name }) != 0;
                let __v1: bool =
                    __v0 && validate_type_params(unsafe { (*stmt).v.FunctionDef.type_params }) != 0;
                let __v2: bool =
                    __v1 && validate_arguments(unsafe { (*stmt).v.FunctionDef.args }) != 0;
                let __v3: bool = __v2
                    && validate_exprs(unsafe { (*stmt).v.FunctionDef.decorator_list }, 1u32, 0i32)
                        != 0;
                let __v4: bool = __v3
                    && (unsafe { (*stmt).v.FunctionDef.returns }.is_null()
                        || validate_expr(unsafe { (*stmt).v.FunctionDef.returns }, 1u32) != 0);
                ret = __v4 as i32;
            }
            3 => {
                let __v7: bool =
                    validate_body(unsafe { (*stmt).v.ClassDef.body }, c"ClassDef".as_ptr()) != 0
                        && validate_name(unsafe { (*stmt).v.ClassDef.name }) != 0;
                let __v8: bool =
                    __v7 && validate_type_params(unsafe { (*stmt).v.ClassDef.type_params }) != 0;
                let __v9: bool =
                    __v8 && validate_exprs(unsafe { (*stmt).v.ClassDef.bases }, 1u32, 0i32) != 0;
                let __v10: bool =
                    __v9 && validate_keywords(unsafe { (*stmt).v.ClassDef.keywords }) != 0;
                let __v11: bool = __v10
                    && validate_exprs(unsafe { (*stmt).v.ClassDef.decorator_list }, 1u32, 0i32)
                        != 0;
                ret = __v11 as i32;
            }
            4 => {
                let __v12: bool = unsafe { (*stmt).v.Return.value }.is_null()
                    || validate_expr(unsafe { (*stmt).v.Return.value }, 1u32) != 0;
                ret = __v12 as i32;
            }
            5 => {
                ret = validate_assignlist(unsafe { (*stmt).v.Delete.targets }, 3u32);
            }
            6 => {
                let __v13: bool = validate_assignlist(unsafe { (*stmt).v.Assign.targets }, 2u32)
                    != 0
                    && validate_expr(unsafe { (*stmt).v.Assign.value }, 1u32) != 0;
                ret = __v13 as i32;
            }
            8 => {
                let __v14: bool = validate_expr(unsafe { (*stmt).v.AugAssign.target }, 2u32) != 0
                    && validate_expr(unsafe { (*stmt).v.AugAssign.value }, 1u32) != 0;
                ret = __v14 as i32;
            }
            9 => {
                if ((unsafe { (*(*stmt).v.AnnAssign.target).kind }) as u32) != 26
                    && (unsafe { (*stmt).v.AnnAssign.simple }) != 0
                {
                    unsafe {
                        PyErr_SetString(
                            PyExc_TypeError,
                            c"AnnAssign with simple non-Name target".as_ptr(),
                        )
                    };
                    return 0;
                }
                let __v15: bool = validate_expr(unsafe { (*stmt).v.AnnAssign.target }, 2u32) != 0
                    && (unsafe { (*stmt).v.AnnAssign.value }.is_null()
                        || validate_expr(unsafe { (*stmt).v.AnnAssign.value }, 1u32) != 0);
                let __v17: bool =
                    __v15 && validate_expr(unsafe { (*stmt).v.AnnAssign.annotation }, 1u32) != 0;
                ret = __v17 as i32;
            }
            7 => {
                if ((unsafe { (*(*stmt).v.TypeAlias.name).kind }) as u32) != 26 {
                    unsafe {
                        PyErr_SetString(PyExc_TypeError, c"TypeAlias with non-Name name".as_ptr())
                    };
                    return 0;
                }
                let __v18: bool = validate_expr(unsafe { (*stmt).v.TypeAlias.name }, 2u32) != 0
                    && validate_type_params(unsafe { (*stmt).v.TypeAlias.type_params }) != 0;
                let __v19: bool =
                    __v18 && validate_expr(unsafe { (*stmt).v.TypeAlias.value }, 1u32) != 0;
                ret = __v19 as i32;
            }
            10 => {
                let __v20: bool = validate_expr(unsafe { (*stmt).v.For.target }, 2u32) != 0
                    && validate_expr(unsafe { (*stmt).v.For.iter }, 1u32) != 0;
                let __v21: bool =
                    __v20 && validate_body(unsafe { (*stmt).v.For.body }, c"For".as_ptr()) != 0;
                let __v22: bool = __v21 && validate_stmts(unsafe { (*stmt).v.For.orelse }) != 0;
                ret = __v22 as i32;
            }
            11 => {
                let __v23: bool = validate_expr(unsafe { (*stmt).v.AsyncFor.target }, 2u32) != 0
                    && validate_expr(unsafe { (*stmt).v.AsyncFor.iter }, 1u32) != 0;
                let __v24: bool = __v23
                    && validate_body(unsafe { (*stmt).v.AsyncFor.body }, c"AsyncFor".as_ptr()) != 0;
                let __v25: bool =
                    __v24 && validate_stmts(unsafe { (*stmt).v.AsyncFor.orelse }) != 0;
                ret = __v25 as i32;
            }
            12 => {
                let __v26: bool = validate_expr(unsafe { (*stmt).v.While.test }, 1u32) != 0
                    && validate_body(unsafe { (*stmt).v.While.body }, c"While".as_ptr()) != 0;
                let __v27: bool = __v26 && validate_stmts(unsafe { (*stmt).v.While.orelse }) != 0;
                ret = __v27 as i32;
            }
            13 => {
                let __v28: bool = validate_expr(unsafe { (*stmt).v.If.test }, 1u32) != 0
                    && validate_body(unsafe { (*stmt).v.If.body }, c"If".as_ptr()) != 0;
                let __v29: bool = __v28 && validate_stmts(unsafe { (*stmt).v.If.orelse }) != 0;
                ret = __v29 as i32;
            }
            14 => {
                if validate_nonempty_seq!(
                    unsafe { (*stmt).v.With.items },
                    c"items".as_ptr() as *mut i8,
                    c"With".as_ptr() as *mut i8
                ) == 0
                {
                    return 0;
                }
                let mut i: i64 = 0;
                while i < asdl_seq_LEN__10!(unsafe { (*stmt).v.With.items }) {
                    let item: *mut _withitem =
                        asdl_seq_GET__11!(unsafe { (*stmt).v.With.items }, i);
                    let __v30: bool = validate_expr(unsafe { (*item).context_expr }, 1u32) == 0
                        || !unsafe { (*item).optional_vars }.is_null()
                            && validate_expr(unsafe { (*item).optional_vars }, 2u32) == 0;
                    if __v30 {
                        return 0;
                    }
                    i = i.wrapping_add(1);
                }
                ret = validate_body(unsafe { (*stmt).v.With.body }, c"With".as_ptr());
            }
            15 => {
                if validate_nonempty_seq!(
                    unsafe { (*stmt).v.AsyncWith.items },
                    c"items".as_ptr() as *mut i8,
                    c"AsyncWith".as_ptr() as *mut i8
                ) == 0
                {
                    return 0;
                }
                let mut i: i64 = 0;
                while i < asdl_seq_LEN__10!(unsafe { (*stmt).v.AsyncWith.items }) {
                    let item: *mut _withitem =
                        asdl_seq_GET__11!(unsafe { (*stmt).v.AsyncWith.items }, i);
                    let __v35: bool = validate_expr(unsafe { (*item).context_expr }, 1u32) == 0
                        || !unsafe { (*item).optional_vars }.is_null()
                            && validate_expr(unsafe { (*item).optional_vars }, 2u32) == 0;
                    if __v35 {
                        return 0;
                    }
                    i = i.wrapping_add(1);
                }
                ret = validate_body(unsafe { (*stmt).v.AsyncWith.body }, c"AsyncWith".as_ptr());
            }
            16 => {
                let __v40: bool = validate_expr(unsafe { (*stmt).v.Match.subject }, 1u32) == 0
                    || validate_nonempty_seq!(
                        unsafe { (*stmt).v.Match.cases },
                        c"cases".as_ptr() as *mut i8,
                        c"Match".as_ptr() as *mut i8
                    ) == 0;
                if __v40 {
                    return 0;
                }
                let mut i: i64 = 0;
                while i < asdl_seq_LEN__14!(unsafe { (*stmt).v.Match.cases }) {
                    let m: *mut _match_case =
                        asdl_seq_GET__13!(unsafe { (*stmt).v.Match.cases }, i);
                    let __v41: bool = validate_pattern(unsafe { (*m).pattern }, 0i32) == 0
                        || !unsafe { (*m).guard }.is_null()
                            && validate_expr(unsafe { (*m).guard }, 1u32) == 0;
                    let __v43: bool =
                        __v41 || validate_body(unsafe { (*m).body }, c"match_case".as_ptr()) == 0;
                    if __v43 {
                        return 0;
                    }
                    // star_ok=
                    i = i.wrapping_add(1);
                }
                ret = 1;
            }
            17 => {
                if !unsafe { (*stmt).v.Raise.exc }.is_null() {
                    let __v47: bool = validate_expr(unsafe { (*stmt).v.Raise.exc }, 1u32) != 0
                        && (unsafe { (*stmt).v.Raise.cause }.is_null()
                            || validate_expr(unsafe { (*stmt).v.Raise.cause }, 1u32) != 0);
                    ret = __v47 as i32;
                } else {
                    if !unsafe { (*stmt).v.Raise.cause }.is_null() {
                        unsafe {
                            PyErr_SetString(
                                PyExc_ValueError,
                                c"Raise with cause but no exception".as_ptr(),
                            )
                        };
                        return 0;
                    }
                    ret = 1;
                }
            }
            18 => {
                if validate_body(unsafe { (*stmt).v.Try.body }, c"Try".as_ptr()) == 0 {
                    return 0;
                }
                let __v49: bool = asdl_seq_LEN__8!(unsafe { (*stmt).v.Try.handlers }) == 0
                    && asdl_seq_LEN__2!(unsafe { (*stmt).v.Try.finalbody }) == 0;
                if __v49 {
                    unsafe {
                        PyErr_SetString(
                            PyExc_ValueError,
                            c"Try has neither except handlers nor finalbody".as_ptr(),
                        )
                    };
                    return 0;
                }
                let __v50: bool = asdl_seq_LEN__8!(unsafe { (*stmt).v.Try.handlers }) == 0
                    && asdl_seq_LEN__2!(unsafe { (*stmt).v.Try.orelse }) != 0;
                if __v50 {
                    unsafe {
                        PyErr_SetString(
                            PyExc_ValueError,
                            c"Try has orelse but no except handlers".as_ptr(),
                        )
                    };
                    return 0;
                }
                let mut i: i64 = 0;
                while i < asdl_seq_LEN__8!(unsafe { (*stmt).v.Try.handlers }) {
                    let handler: *mut _excepthandler =
                        asdl_seq_GET__10!(unsafe { (*stmt).v.Try.handlers }, i);
                    VALIDATE_POSITIONS!(handler);
                    let __v51: bool = !unsafe { (*handler).v.ExceptHandler.r#type }.is_null()
                        && validate_expr(unsafe { (*handler).v.ExceptHandler.r#type }, 1u32) == 0;
                    let __v52: bool = __v51
                        || !unsafe { (*handler).v.ExceptHandler.name }.is_null()
                            && validate_name(unsafe { (*handler).v.ExceptHandler.name }) == 0;
                    let __v54: bool = __v52
                        || validate_body(
                            unsafe { (*handler).v.ExceptHandler.body },
                            c"ExceptHandler".as_ptr(),
                        ) == 0;
                    if __v54 {
                        return 0;
                    }
                    i = i.wrapping_add(1);
                }
                let __v58: bool = asdl_seq_LEN__2!(unsafe { (*stmt).v.Try.finalbody }) == 0
                    || validate_stmts(unsafe { (*stmt).v.Try.finalbody }) != 0;
                let __v59: bool = __v58
                    && (asdl_seq_LEN__2!(unsafe { (*stmt).v.Try.orelse }) == 0
                        || validate_stmts(unsafe { (*stmt).v.Try.orelse }) != 0);
                ret = __v59 as i32;
            }
            19 => {
                if validate_body(unsafe { (*stmt).v.TryStar.body }, c"TryStar".as_ptr()) == 0 {
                    return 0;
                }
                let __v61: bool = asdl_seq_LEN__8!(unsafe { (*stmt).v.TryStar.handlers }) == 0
                    && asdl_seq_LEN__2!(unsafe { (*stmt).v.TryStar.finalbody }) == 0;
                if __v61 {
                    unsafe {
                        PyErr_SetString(
                            PyExc_ValueError,
                            c"TryStar has neither except handlers nor finalbody".as_ptr(),
                        )
                    };
                    return 0;
                }
                let __v62: bool = asdl_seq_LEN__8!(unsafe { (*stmt).v.TryStar.handlers }) == 0
                    && asdl_seq_LEN__2!(unsafe { (*stmt).v.TryStar.orelse }) != 0;
                if __v62 {
                    unsafe {
                        PyErr_SetString(
                            PyExc_ValueError,
                            c"TryStar has orelse but no except handlers".as_ptr(),
                        )
                    };
                    return 0;
                }
                let mut i: i64 = 0;
                while i < asdl_seq_LEN__8!(unsafe { (*stmt).v.TryStar.handlers }) {
                    let handler: *mut _excepthandler =
                        asdl_seq_GET__10!(unsafe { (*stmt).v.TryStar.handlers }, i);
                    let __v63: bool = !unsafe { (*handler).v.ExceptHandler.r#type }.is_null()
                        && validate_expr(unsafe { (*handler).v.ExceptHandler.r#type }, 1u32) == 0;
                    let __v64: bool = __v63
                        || validate_body(
                            unsafe { (*handler).v.ExceptHandler.body },
                            c"ExceptHandler".as_ptr(),
                        ) == 0;
                    if __v64 {
                        return 0;
                    }
                    i = i.wrapping_add(1);
                }
                let __v68: bool = asdl_seq_LEN__2!(unsafe { (*stmt).v.TryStar.finalbody }) == 0
                    || validate_stmts(unsafe { (*stmt).v.TryStar.finalbody }) != 0;
                let __v69: bool = __v68
                    && (asdl_seq_LEN__2!(unsafe { (*stmt).v.TryStar.orelse }) == 0
                        || validate_stmts(unsafe { (*stmt).v.TryStar.orelse }) != 0);
                ret = __v69 as i32;
            }
            20 => {
                let __v71: bool = validate_expr(unsafe { (*stmt).v.Assert.test }, 1u32) != 0
                    && (unsafe { (*stmt).v.Assert.msg }.is_null()
                        || validate_expr(unsafe { (*stmt).v.Assert.msg }, 1u32) != 0);
                ret = __v71 as i32;
            }
            21 => {
                ret = validate_import_names(
                    unsafe { (*stmt).v.Import.names },
                    c"names".as_ptr(),
                    c"Import".as_ptr(),
                );
            }
            22 => {
                if (unsafe { (*stmt).v.ImportFrom.level }) < 0 {
                    unsafe {
                        PyErr_SetString(PyExc_ValueError, c"Negative ImportFrom level".as_ptr())
                    };
                    return 0;
                }
                ret = validate_import_names(
                    unsafe { (*stmt).v.ImportFrom.names },
                    c"names".as_ptr(),
                    c"ImportFrom".as_ptr(),
                );
            }
            23 => {
                ret = validate_nonempty_seq!(
                    unsafe { (*stmt).v.Global.names },
                    c"names".as_ptr() as *mut i8,
                    c"Global".as_ptr() as *mut i8
                );
            }
            24 => {
                ret = validate_nonempty_seq!(
                    unsafe { (*stmt).v.Nonlocal.names },
                    c"names".as_ptr() as *mut i8,
                    c"Nonlocal".as_ptr() as *mut i8
                );
            }
            25 => {
                ret = validate_expr(unsafe { (*stmt).v.Expr.value }, 1u32);
            }
            2 => {
                let __v73: bool = validate_body(
                    unsafe { (*stmt).v.AsyncFunctionDef.body },
                    c"AsyncFunctionDef".as_ptr(),
                ) != 0
                    && validate_name(unsafe { (*stmt).v.AsyncFunctionDef.name }) != 0;
                let __v74: bool = __v73
                    && validate_type_params(unsafe { (*stmt).v.AsyncFunctionDef.type_params }) != 0;
                let __v75: bool =
                    __v74 && validate_arguments(unsafe { (*stmt).v.AsyncFunctionDef.args }) != 0;
                let __v76: bool = __v75
                    && validate_exprs(
                        unsafe { (*stmt).v.AsyncFunctionDef.decorator_list },
                        1u32,
                        0i32,
                    ) != 0;
                let __v77: bool = __v76
                    && (unsafe { (*stmt).v.AsyncFunctionDef.returns }.is_null()
                        || validate_expr(unsafe { (*stmt).v.AsyncFunctionDef.returns }, 1u32) != 0);
                ret = __v77 as i32;
            }
            26..=28 => {
                ret = 1;
                break '__slate_break_6;
                // No default case so compiler emits warning for unhandled cases
            }
            _ => {}
        }
    }
    if ret < 0 {
        unsafe { PyErr_SetString(PyExc_SystemError, c"unexpected statement".as_ptr()) };
        ret = 0;
    }
    LEAVE_RECURSIVE!();
    ret
}

fn validate_stmts(seq: *mut __SlateRecord_pycore_ast_h_67) -> i32 {
    let mut i: i64 = 0;
    while i < asdl_seq_LEN__2!(seq) {
        let stmt: *mut _stmt = asdl_seq_GET__2!(seq, i);
        if !stmt.is_null() {
            if validate_stmt(stmt) == 0 {
                return 0;
            }
        } else {
            unsafe {
                PyErr_SetString(
                    PyExc_ValueError,
                    c"None disallowed in statement list".as_ptr(),
                )
            };
            return 0;
        }
        i = i.wrapping_add(1);
    }
    1
}

fn validate_exprs(exprs: *mut __SlateRecord_pycore_ast_h_74, ctx: u32, null_ok: i32) -> i32 {
    let mut i: i64 = 0;
    while i < asdl_seq_LEN!(exprs) {
        let expr: *mut _expr = asdl_seq_GET!(exprs, i);
        if !expr.is_null() {
            if validate_expr(expr, ctx) == 0 {
                return 0;
            }
        } else {
            if null_ok == 0 {
                unsafe {
                    PyErr_SetString(
                        PyExc_ValueError,
                        c"None disallowed in expression list".as_ptr(),
                    )
                };
                return 0;
            }
        }
        i = i.wrapping_add(1);
    }
    1
}

fn validate_patterns(patterns: *mut __SlateRecord_pycore_ast_h_140, star_ok: i32) -> i32 {
    let mut i: i64 = 0;
    while i < asdl_seq_LEN__5!(patterns) {
        let pattern: *mut _pattern = asdl_seq_GET__3!(patterns, i);
        if validate_pattern(pattern, star_ok) == 0 {
            return 0;
        }
        i = i.wrapping_add(1);
    }
    1
}

fn validate_typeparam(tp: *mut _type_param) -> i32 {
    VALIDATE_POSITIONS!(tp);
    let mut ret: i32 = 1i32.wrapping_neg();
    match (unsafe { (*tp).kind }) as u32 {
        1 => {
            let __v0: bool = validate_name(unsafe { (*tp).v.TypeVar.name }) != 0
                && (unsafe { (*tp).v.TypeVar.bound }.is_null()
                    || validate_expr(unsafe { (*tp).v.TypeVar.bound }, 1u32) != 0);
            let __v2: bool = __v0
                && (unsafe { (*tp).v.TypeVar.default_value }.is_null()
                    || validate_expr(unsafe { (*tp).v.TypeVar.default_value }, 1u32) != 0);
            ret = __v2 as i32;
        }
        2 => {
            let __v5: bool = validate_name(unsafe { (*tp).v.ParamSpec.name }) != 0
                && (unsafe { (*tp).v.ParamSpec.default_value }.is_null()
                    || validate_expr(unsafe { (*tp).v.ParamSpec.default_value }, 1u32) != 0);
            ret = __v5 as i32;
        }
        3 => {
            let __v7: bool = validate_name(unsafe { (*tp).v.TypeVarTuple.name }) != 0
                && (unsafe { (*tp).v.TypeVarTuple.default_value }.is_null()
                    || validate_expr(unsafe { (*tp).v.TypeVarTuple.default_value }, 1u32) != 0);
            ret = __v7 as i32;
        }
        _ => {}
    }
    ret
}

fn validate_type_params(tps: *mut __SlateRecord_pycore_ast_h_155) -> i32 {
    let mut i: i64 = 0;
    while i < asdl_seq_LEN__6!(tps) {
        let tp: *mut _type_param = asdl_seq_GET__7!(tps, i);
        if !tp.is_null() {
            if validate_typeparam(tp) == 0 {
                return 0;
            }
        }
        i = i.wrapping_add(1);
    }
    1
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyAST_Validate(r#mod: *mut _mod) -> i32 {
    let mut res: i32 = 1i32.wrapping_neg();
    '__slate_break_0: {
        match (unsafe { (*r#mod).kind }) as u32 {
            1 => {
                res = validate_stmts(unsafe { (*r#mod).v.Module.body });
            }
            2 => {
                res = validate_stmts(unsafe { (*r#mod).v.Interactive.body });
            }
            3 => {
                res = validate_expr(unsafe { (*r#mod).v.Expression.body }, 1u32);
            }
            4 => {
                let __v1: bool =
                    validate_exprs(unsafe { (*r#mod).v.FunctionType.argtypes }, 1u32, 0i32) != 0
                        && validate_expr(unsafe { (*r#mod).v.FunctionType.returns }, 1u32) != 0;
                res = __v1 as i32; // null_ok=
                break '__slate_break_0;
                // No default case so compiler emits warning for unhandled cases
            }
            _ => {}
        }
    }
    if res < 0 {
        unsafe { PyErr_SetString(PyExc_SystemError, c"impossible module node".as_ptr()) };
        return 0;
    }
    res
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyAST_GetDocString(body: *mut __SlateRecord_pycore_ast_h_67) -> *mut _object {
    if asdl_seq_LEN__2!(body) == 0 {
        return core::ptr::null_mut::<_object>();
    }
    let st: *mut _stmt = asdl_seq_GET__2!(body, 0i32);
    if ((unsafe { (*st).kind }) as u32) != 25 {
        return core::ptr::null_mut::<_object>();
    }
    let e: *mut _expr = unsafe { (*st).v.Expr.value };
    let __v0: bool = ((unsafe { (*e).kind }) as u32) == 22
        && PyUnicode_CheckExact!(unsafe { (*e).v.Constant.value }) != 0;
    if __v0 {
        return unsafe { (*e).v.Constant.value };
    }
    core::ptr::null_mut::<_object>()
}
