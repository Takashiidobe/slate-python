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
//! No error
//! Max indentation level
//! Alternate tab spacing
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
// End Of File
// Execution error
// Too many indentation levels
// No matching outer block for dedent
// Error in decoding into Unicode
// EOF in triple-quoted string
// EOL in single-quoted string
// Unexpected characters after a line continuation
// Interactive mode stopped tokenization
// Column offset overflow
macro_rules! FTSTRING_END {
    ($tok_mode:expr) => {
        if ((unsafe { (*$tok_mode).string_kind }) as u32) == 1 {
            64i32
        } else {
            61i32
        }
    };
}

macro_rules! FTSTRING_MIDDLE {
    ($tok_mode:expr) => {
        if ((unsafe { (*$tok_mode).string_kind }) as u32) == 1 {
            63i32
        } else {
            60i32
        }
    };
}

macro_rules! INSIDE_FSTRING_EXPR_AT_TOP {
    ($tok:expr) => {
        unsafe { (*$tok).curly_bracket_depth }
            .wrapping_sub(unsafe { (*$tok).curly_bracket_expr_start_depth })
            == 1
    };
}

macro_rules! INSIDE_FSTRING_EXPR {
    ($tok:expr) => {
        (unsafe { (*$tok).curly_bracket_expr_start_depth }) >= 0
    };
}

macro_rules! INSIDE_FSTRING {
    ($tok:expr) => {
        (unsafe { (*$tok).tok_mode_stack_index }) > 0
    };
}

macro_rules! MAKE_TOKEN {
    ($token_type:expr, $tok:expr, $token:expr, $p_start:expr, $p_end:expr) => {
        unsafe { _PyLexer_token_setup($tok, $token, $token_type, $p_start, $p_end) }
    };
}

macro_rules! MAKE_TYPE_COMMENT_TOKEN {
    ($token_type:expr, $col_offset:expr, $end_col_offset:expr, $tok:expr, $token:expr, $p_start:expr, $p_end:expr) => {
        unsafe {
            _PyLexer_type_comment_token_setup(
                $tok,
                $token,
                $token_type,
                $col_offset,
                $end_col_offset,
                $p_start,
                $p_end,
            )
        }
    };
}

macro_rules! PyBytes_GET_SIZE {
    ($self:expr) => {
        PyBytes_GET_SIZE(_PyObject_CAST!($self))
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

macro_rules! PyUnicode_READ_CHAR {
    ($unicode:expr, $index:expr) => {
        PyUnicode_READ_CHAR(_PyObject_CAST!($unicode), $index)
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

macro_rules! Py_FatalError__15 {
    ($message:expr) => {
        unsafe { _Py_FatalErrorFunc(c"tok_backup".as_ptr(), ($message) as *const i8) };
        unsafe { core::hint::unreachable_unchecked() };
    };
}

macro_rules! Py_ISALNUM {
    ($c:expr) => {
        (unsafe { _Py_ctype_table[(Py_CHARMASK!($c) as i32) as usize] }) & 7u32
    };
}

macro_rules! Py_ISDIGIT {
    ($c:expr) => {
        (unsafe { _Py_ctype_table[(Py_CHARMASK__2!($c) as i32) as usize] }) & 4
    };
}

macro_rules! Py_ISXDIGIT {
    ($c:expr) => {
        (unsafe { _Py_ctype_table[(Py_CHARMASK__2!($c) as i32) as usize] }) & 16
    };
}

macro_rules! Py_SETREF {
    ($dst:expr, $src:expr) => {
        loop {
            let _tmp_dst_ptr: *mut *mut _object = core::ptr::addr_of_mut!($dst);
            let _tmp_old_dst: *mut _object = unsafe { *_tmp_dst_ptr };
            unsafe {
                *_tmp_dst_ptr = $src;
            }
            Py_DECREF!(_tmp_old_dst);
            if true {
                break;
            }
        }
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

macro_rules! Py_UNICODE_ISPRINTABLE {
    ($ch:expr) => {
        unsafe { _PyUnicode_IsPrintable($ch) }
    };
}

macro_rules! Py_UNICODE_ISPRINTABLE__2 {
    ($ch:expr) => {
        unsafe { _PyUnicode_IsPrintable(($ch) as u32) }
    };
}

macro_rules! Py_UNREACHABLE {
    () => {
        unsafe { core::hint::unreachable_unchecked() };
    };
}

macro_rules! TOK_GET_MODE {
    ($tok:expr) => {
        unsafe { (*$tok).tok_mode_stack.as_mut_ptr() as *mut _tokenizer_mode }
            .wrapping_offset((unsafe { (*$tok).tok_mode_stack_index }) as isize)
    };
}

macro_rules! TOK_GET_STRING_PREFIX {
    ($tok:expr) => {
        if ((unsafe { (*TOK_GET_MODE!($tok)).string_kind }) as u32) == 1 {
            116i32
        } else {
            102i32
        }
    };
}

macro_rules! TOK_NEXT_MODE {
    ($tok:expr) => {{
        let __v: *mut tok_state = $tok;
        let __v2: i32 = unsafe { (*__v).tok_mode_stack_index };
        let __v3: i32 = __v2.wrapping_add(1);
        unsafe {
            (*__v).tok_mode_stack_index = __v3;
        }
        unsafe { (*$tok).tok_mode_stack.as_mut_ptr() as *mut _tokenizer_mode }
            .wrapping_offset(__v3 as isize)
    }};
}

macro_rules! _PyASCIIObject_CAST {
    ($op:expr) => {
        _Py_CAST!(*mut __SlateRecord_unicodeobject_h_163, $op)
    };
}

/// Macros and static inline functions, trading safety for speed
macro_rules! _PyBytes_CAST {
    ($op:expr) => {
        _Py_CAST!(*mut __SlateRecord_bytesobject_h_15, $op)
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

/// Cast argument to PyObject* type.
macro_rules! _PyObject_CAST__2 {
    ($op:expr) => {
        _Py_CAST!(*mut _object, $op)
    };
}

macro_rules! _PyUnicodeObject_CAST {
    ($op:expr) => {
        _Py_CAST!(*mut __SlateRecord_unicodeobject_h_184, $op)
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

macro_rules! _Py_STATIC_CAST {
    ($type:ty, $expr:expr) => {
        ($expr) as $type
    };
}

macro_rules! is_potential_identifier_char {
    ($c:expr) => {
        ($c) >= 97 && ($c) <= 122
            || ($c) >= 65 && ($c) <= 90
            || ($c) >= 48 && ($c) <= 57
            || ($c) == 95
            || ($c) >= 128
    };
}

macro_rules! is_potential_identifier_start {
    ($c:expr) => {
        ($c) >= 97 && ($c) <= 122 || ($c) >= 65 && ($c) <= 90 || ($c) == 95 || ($c) >= 128
    };
}

unsafe extern "C" {
    static mut PyExc_UnicodeDecodeError: *mut _object;
    static mut PyExc_SyntaxWarning: *mut _object;
    static mut _Py_ctype_table: [u32; 256];
    static mut _Py_ctype_tolower: [u8; 256];
    fn index(arg0: *const i8, arg1: i32) -> *mut i8;
    fn PyMem_Malloc(arg0: u64) -> *mut ();
    fn PyMem_Free(arg0: *mut ());
    fn _Py_Dealloc(arg0: *mut _object);
    fn PyUnicode_Substring(arg0: *mut _object, arg1: i64, arg2: i64) -> *mut _object;
    fn PyUnicode_DecodeUTF8(arg0: *const i8, arg1: i64, arg2: *const i8) -> *mut _object;
    fn PyUnicode_AsUTF8String(arg0: *mut _object) -> *mut _object;
    fn _PyUnicode_IsPrintable(arg0: u32) -> i32;
    fn PyErr_Occurred() -> *mut _object;
    fn PyErr_ExceptionMatches(arg0: *mut _object) -> i32;
    fn _Py_FatalErrorFunc(arg0: *const i8, arg1: *const i8);
    fn _PyToken_OneChar(arg0: i32) -> i32;
    fn _PyToken_TwoChars(arg0: i32, arg1: i32) -> i32;
    fn _PyToken_ThreeChars(arg0: i32, arg1: i32, arg2: i32) -> i32;
    fn _PyUnicode_ScanIdentifier(arg0: *mut _object) -> i64;
    fn _PyLexer_type_comment_token_setup(
        arg0: *mut tok_state,
        arg1: *mut token,
        arg2: i32,
        arg3: i32,
        arg4: i32,
        arg5: *const i8,
        arg6: *const i8,
    ) -> i32;
    fn _PyLexer_token_setup(
        arg0: *mut tok_state,
        arg1: *mut token,
        arg2: i32,
        arg3: *const i8,
        arg4: *const i8,
    ) -> i32;
    fn _PyTokenizer_syntaxerror(arg0: *mut tok_state, arg1: *const i8, ...) -> i32;
    fn _PyTokenizer_syntaxerror_known_range(
        arg0: *mut tok_state,
        arg1: i32,
        arg2: i32,
        arg3: *const i8,
        ...
    ) -> i32;
    fn _PyTokenizer_indenterror(arg0: *mut tok_state) -> i32;
    fn _PyTokenizer_warn_invalid_escape_sequence(arg0: *mut tok_state, arg1: i32) -> i32;
    fn _PyTokenizer_parser_warn(
        arg0: *mut tok_state,
        arg1: *mut _object,
        arg2: *const i8,
        ...
    ) -> i32;
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
struct __SlateRecord_bytesobject_h_15 {
    ob_base: __SlateRecord_object_h_172,
    ob_shash: i64,
    ob_sval: [i8; 1],
}

fn PyBytes_GET_SIZE(op: *mut _object) -> i64 {
    let self_: *mut __SlateRecord_bytesobject_h_15 = _PyBytes_CAST!(op);
    Py_SIZE!(self_)
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_unicodeobject_h_163_2 {
    __slate_bits_0: __slate_bits::__SlateBits89U0,
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
struct __SlateRecord_unicodeobject_h_173 {
    _base: __SlateRecord_unicodeobject_h_163,
    utf8_length: i64,
    utf8: *mut i8,
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
    pub struct __SlateBits89U0 {
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

use slate_support::memchr;
use slate_support::memcmp;

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
struct token {
    level: i32,
    lineno: i32,
    col_offset: i32,
    end_lineno: i32,
    end_col_offset: i32,
    start: *const i8,
    end: *const i8,
    metadata: *mut _object,
}

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

// Spaces in this constant are treated as "zero or more spaces or tabs" when
// tokenizing.
static mut type_comment_prefix: *const i8 = (b"# type: \0".as_ptr() as *mut i8) as *const i8;

fn contains_null_bytes(str: *const i8, size: u64) -> i32 {
    !((unsafe {
        memchr(
            (str as *const ()) as *const core::ffi::c_void,
            0i32 as i32,
            size as usize,
        ) as *mut ()
    }) as *const ())
        .is_null() as i32
}

// Get next char, updating state; error code goes into tok->done
fn tok_nextc(tok: *mut tok_state) -> i32 {
    let mut rc: i32 = 0;
    loop {
        if (unsafe { (*tok).cur }) != unsafe { (*tok).inp } {
            if ((unsafe { (*tok).col_offset }) as u32) >= 2147483647 {
                unsafe {
                    (*tok).done = 29;
                }
                return slate_support::EOF as i32;
            }
            let __v1: i32 = unsafe { (*tok).col_offset };
            unsafe {
                (*tok).col_offset = __v1.wrapping_add(1);
            }
            let __v4: *mut i8 = unsafe { (*tok).cur };
            unsafe {
                (*tok).cur = __v4.wrapping_offset(1 as isize);
            }
            return ((((unsafe { *__v4 }) as i32) & 255) as u8) as i32; // Fast path
        }
        if (unsafe { (*tok).done }) != 10 {
            return slate_support::EOF as i32;
        }
        rc = unsafe { unsafe { (*tok).underflow }.unwrap()(tok) };
        if rc == 0 {
            unsafe {
                (*tok).cur = (*tok).inp;
            }
            return slate_support::EOF as i32;
        }
        unsafe {
            (*tok).line_start = (*tok).cur as *const i8;
        }
        if contains_null_bytes(
            unsafe { (*tok).line_start },
            ((unsafe { unsafe { (*tok).inp }.offset_from((*tok).line_start as *mut i8) }) as i64)
                as u64,
        ) != 0
        {
            unsafe {
                _PyTokenizer_syntaxerror(tok, c"source code cannot contain null bytes".as_ptr())
            };
            unsafe {
                (*tok).cur = (*tok).inp;
            }
            return slate_support::EOF as i32;
        }
    }
    Py_UNREACHABLE!();
    unsafe { core::mem::zeroed() }
}

/// Back-up one character
fn tok_backup(tok: *mut tok_state, c: i32) {
    if c != (slate_support::EOF as i32) {
        let __v1: *mut i8 = unsafe { (*tok).cur };
        let __v2: *mut i8 = __v1.wrapping_offset(-(1 as isize));
        unsafe {
            (*tok).cur = __v2;
        }
        if __v2 < unsafe { (*tok).buf } {
            Py_FatalError__15!(c"tokenizer beginning of buffer".as_ptr() as *mut i8);
        }
        if (((unsafe { *(*tok).cur }) as u8) as i32) != (Py_CHARMASK__2!(c) as i32) {
            Py_FatalError__15!(c"tok_backup: wrong character".as_ptr() as *mut i8);
        }
        let __v4: i32 = unsafe { (*tok).col_offset };
        unsafe {
            (*tok).col_offset = __v4.wrapping_sub(1);
        }
    }
}

fn set_ftstring_expr(tok: *mut tok_state, token: *mut token, c: i8) -> i32 {
    let tok_mode: *mut _tokenizer_mode = TOK_GET_MODE!(tok);
    if !((unsafe { (*tok_mode).in_debug }) != 0
        || ((unsafe { (*tok_mode).string_kind }) as u32) == 1)
        || !unsafe { (*token).metadata }.is_null()
    {
        return 0;
    }
    let expression: *const i8 = unsafe { (*tok_mode).last_expr_start };
    let expression_size: i64 =
        (unsafe { unsafe { (*tok).start }.offset_from(expression as *const i8) }) as i64;
    let mut res: *mut _object = core::ptr::null_mut::<_object>();
    // Look for a # character outside of string literals
    let mut hash_detected: i32 = 0;
    let mut in_string: i32 = 0;
    let mut quote_char: i8 = 0;
    {
        let mut i: i64 = 0;
        while i < expression_size {
            let ch: i8 = unsafe { *expression.wrapping_offset(i as isize) };
            // Skip escaped characters
            if (ch as i32) == 92 {
                i = i.wrapping_add(1);
            } else {
                // Handle quotes
                if (ch as i32) == 34 || (ch as i32) == 39 {
                    // The following if/else block works becase there is an off number
                    //
                    // of quotes in STRING tokens and the lexer only ever reaches this
                    //
                    // function with valid STRING tokens.
                    //
                    // For example: """hello"""
                    //
                    // First quote: in_string = 1
                    //
                    // Second quote: in_string = 0
                    //
                    // Third quote: in_string = 1
                    if in_string == 0 {
                        in_string = 1;
                        quote_char = ch;
                    } else {
                        if (ch as i32) == (quote_char as i32) {
                            in_string = 0;
                        }
                    }
                } else {
                    // Check for # outside strings
                    if (ch as i32) == 35 && in_string == 0 {
                        hash_detected = 1;
                        break;
                    }
                }
            }
            i = i.wrapping_add(1);
        }
    }
    // If we found a # character in the expression, we need to handle comments
    if hash_detected != 0 {
        // Allocate buffer for processed result
        let result: *mut i8 = (unsafe {
            PyMem_Malloc(
                (expression_size.wrapping_add(1) as u64)
                    .wrapping_mul(core::mem::size_of::<i8>() as u64),
            )
        }) as *mut i8;
        if result.is_null() {
            return 1i32.wrapping_neg();
        }
        let mut i: i64 = 0; // Input position
        let mut j: i64 = 0; // Output position
        in_string = 0; // Whether we're in a string
        quote_char = 0; // Current string quote char
        // Process each character
        while i < expression_size {
            let ch: i8 = unsafe { *expression.wrapping_offset(i as isize) };
            // Copy escaped characters without interpreting the escaped
            //
            // character as a quote or comment marker.
            if (ch as i32) == 92 {
                let __v5: i64 = j;
                j = __v5.wrapping_add(1);
                unsafe {
                    *result.wrapping_offset(__v5 as isize) = ch;
                }
                i = i.wrapping_add(1);
                if i < expression_size {
                    let __v9: i64 = j;
                    j = __v9.wrapping_add(1);
                    unsafe {
                        *result.wrapping_offset(__v9 as isize) =
                            *expression.wrapping_offset(i as isize);
                    }
                }
            } else {
                if (ch as i32) == 34 || (ch as i32) == 39 {
                    // See comment above to understand this part
                    if in_string == 0 {
                        in_string = 1;
                        quote_char = ch;
                    } else {
                        if (ch as i32) == (quote_char as i32) {
                            in_string = 0;
                        }
                    }
                    let __v11: i64 = j;
                    j = __v11.wrapping_add(1);
                    unsafe {
                        *result.wrapping_offset(__v11 as isize) = ch;
                    }
                } else {
                    if (ch as i32) == 35 && in_string == 0 {
                        while i < expression_size
                            && ((unsafe { *expression.wrapping_offset(i as isize) }) as i32) != 10
                        {
                            i = i.wrapping_add(1);
                        }
                        if i < expression_size {
                            let __v16: i64 = j;
                            j = __v16.wrapping_add(1);
                            unsafe {
                                *result.wrapping_offset(__v16 as isize) = 10;
                            }
                        }
                    } else {
                        let __v18: i64 = j;
                        j = __v18.wrapping_add(1);
                        unsafe {
                            *result.wrapping_offset(__v18 as isize) = ch;
                        }
                    }
                }
            }
            // Handle string quotes
            // Skip comments
            // Copy other chars
            i = i.wrapping_add(1);
        }
        unsafe {
            *result.wrapping_offset(j as isize) = 0;
        }
        // Null-terminate the result string
        res = unsafe { PyUnicode_DecodeUTF8(result as *const i8, j, core::ptr::null::<i8>()) };
        unsafe { PyMem_Free(result as *mut ()) };
    } else {
        res = unsafe { PyUnicode_DecodeUTF8(expression, expression_size, core::ptr::null::<i8>()) };
    }
    if res.is_null() {
        return 1i32.wrapping_neg();
    }
    unsafe {
        (*token).metadata = res;
    }
    0
}

fn lookahead(tok: *mut tok_state, test: *const i8) -> i32 {
    let mut s: *const i8 = test;
    let mut res: i32 = 0;
    while true {
        '__slate_continue_2: {
            let c: i32 = tok_nextc(tok);
            if ((unsafe { *s }) as i32) == 0 {
                res = !is_potential_identifier_char!(c) as i32;
            } else {
                if c == ((unsafe { *s }) as i32) {
                    let __v0: *const i8 = s;
                    s = __v0.wrapping_offset(1 as isize);
                    break '__slate_continue_2;
                }
            }
            tok_backup(tok, c);
            while s != test {
                let __v3: *const i8 = s;
                let __v4: *const i8 = __v3.wrapping_offset(-(1 as isize));
                s = __v4;
                tok_backup(tok, (unsafe { *__v4 }) as i32);
            }
            return res;
        }
    }
    unsafe { core::mem::zeroed() }
}

fn verify_end_of_number(tok: *mut tok_state, c: i32, kind: *const i8) -> i32 {
    if (unsafe { (*tok).tok_extra_tokens }) != 0 {
        // When we are parsing extra tokens, we don't want to emit warnings
        //
        // about invalid literals, because we want to be a bit more liberal.
        return 1;
    }
    // Emit a deprecation warning only if the numeric literal is immediately
    // followed by one of keywords which can occur after a numeric literal
    // in valid code: "and", "else", "for", "if", "in", "is" and "or".
    // It allows to gradually deprecate existing valid code without adding
    // warning before error in most cases of invalid numeric literal (which
    // would be confusing and break existing tests).
    // Raise a syntax error with slightly better message than plain
    // "invalid syntax" if the numeric literal is immediately followed by
    // other keyword or identifier.
    let mut r: i32 = 0;
    if c == 97 {
        r = lookahead(tok, c"nd".as_ptr());
    } else {
        if c == 101 {
            r = lookahead(tok, c"lse".as_ptr());
        } else {
            if c == 102 {
                r = lookahead(tok, c"or".as_ptr());
            } else {
                if c == 105 {
                    let c2: i32 = tok_nextc(tok);
                    if c2 == 102 || c2 == 110 || c2 == 115 {
                        r = 1;
                    }
                    tok_backup(tok, c2);
                } else {
                    if c == 111 {
                        r = lookahead(tok, c"r".as_ptr());
                    } else {
                        if c == 110 {
                            r = lookahead(tok, c"ot".as_ptr());
                        }
                    }
                }
            }
        }
    }
    if r != 0 {
        tok_backup(tok, c);
        if (unsafe {
            _PyTokenizer_parser_warn(
                tok,
                PyExc_SyntaxWarning,
                c"invalid %s literal".as_ptr(),
                kind,
            )
        }) != 0
        {
            return 0;
        }
        tok_nextc(tok);
    } else {
        if c < 128 && is_potential_identifier_char!(c) {
            tok_backup(tok, c);
            unsafe { _PyTokenizer_syntaxerror(tok, c"invalid %s literal".as_ptr(), kind) };
            return 0;
        }
    }
    // In future releases, only error will remain.
    1
}

/// Verify that the identifier follows PEP 3131.
fn verify_identifier(tok: *mut tok_state) -> i32 {
    if (unsafe { (*tok).tok_extra_tokens }) != 0 {
        return 1;
    }
    let mut s: *mut _object = unsafe { core::mem::zeroed() };
    if (unsafe { (*tok).decoding_erred }) != 0 {
        return 0;
    }
    s = unsafe {
        PyUnicode_DecodeUTF8(
            (*tok).start,
            unsafe { (*tok).cur }.offset_from((*tok).start as *mut i8) as i64,
            core::ptr::null::<i8>(),
        )
    };
    if s.is_null() {
        if (unsafe { PyErr_ExceptionMatches(PyExc_UnicodeDecodeError) }) != 0 {
            unsafe {
                (*tok).done = 22;
            }
        } else {
            unsafe {
                (*tok).done = 17;
            }
        }
        return 0;
    }
    let invalid: i64 = unsafe { _PyUnicode_ScanIdentifier(s) };
    if invalid < PyUnicode_GET_LENGTH!(s) {
        let ch: u32 = PyUnicode_READ_CHAR!(s, invalid);
        if invalid.wrapping_add(1) < PyUnicode_GET_LENGTH!(s) {
            // Determine the offset in UTF-8 encoded input
            Py_SETREF!(s, unsafe {
                PyUnicode_Substring(s, 0i64, invalid.wrapping_add(1))
            });
            if !s.is_null() {
                Py_SETREF!(s, unsafe { PyUnicode_AsUTF8String(s) });
            }
            if s.is_null() {
                unsafe {
                    (*tok).done = 17;
                }
                return 0;
            }
            unsafe {
                (*tok).cur =
                    ((*tok).start as *mut i8).wrapping_offset(PyBytes_GET_SIZE!(s) as isize);
            }
        }
        Py_DECREF!(s);
        if Py_UNICODE_ISPRINTABLE!(ch) != 0 {
            unsafe {
                _PyTokenizer_syntaxerror(tok, c"invalid character '%c' (U+%04X)".as_ptr(), ch, ch)
            };
        } else {
            unsafe {
                _PyTokenizer_syntaxerror(
                    tok,
                    c"invalid non-printable character U+%04X".as_ptr(),
                    ch,
                )
            };
        }
        return 0;
    }
    Py_DECREF!(s);
    1
}

fn tok_decimal_tail(tok: *mut tok_state) -> i32 {
    let mut c: i32 = 0;
    while true {
        loop {
            c = tok_nextc(tok);
            if Py_ISDIGIT!(c) == 0 {
                break;
            }
        }
        if c != 95 {
            break;
        }
        c = tok_nextc(tok);
        if Py_ISDIGIT!(c) == 0 {
            tok_backup(tok, c);
            unsafe { _PyTokenizer_syntaxerror(tok, c"invalid decimal literal".as_ptr()) };
            return 0;
        }
    }
    c
}

fn tok_continuation_line(tok: *mut tok_state) -> i32 {
    let mut c: i32 = tok_nextc(tok);
    if c == 13 {
        c = tok_nextc(tok);
    }
    if c != 10 {
        unsafe {
            (*tok).done = 25;
        }
        return 1i32.wrapping_neg();
    }
    c = tok_nextc(tok);
    if c == (slate_support::EOF as i32) {
        unsafe {
            (*tok).done = 11;
        }
        unsafe {
            (*tok).cur = (*tok).inp;
        }
        return 1i32.wrapping_neg();
    } else {
        tok_backup(tok, c);
    }
    c
}

fn maybe_raise_syntax_error_for_string_prefixes(
    tok: *mut tok_state,
    saw_b: i32,
    saw_r: i32,
    saw_u: i32,
    saw_f: i32,
    saw_t: i32,
) -> i32 {
    // Supported: rb, rf, rt (in any order)
    //
    // Unsupported: ub, ur, uf, ut, bf, bt, ft (in any order)
    if saw_u != 0 && saw_b != 0 {
        loop {
            unsafe {
                _PyTokenizer_syntaxerror_known_range(
                    tok,
                    (unsafe { (*tok).start }
                        .wrapping_offset(1 as isize)
                        .offset_from((*tok).line_start as *const i8) as i64)
                        as i32,
                    (unsafe { (*tok).cur }.offset_from((*tok).line_start as *mut i8) as i64) as i32,
                    c"'u' and 'b' prefixes are incompatible".as_ptr(),
                )
            };
            return 1i32.wrapping_neg();
        }
    }
    if saw_u != 0 && saw_r != 0 {
        loop {
            unsafe {
                _PyTokenizer_syntaxerror_known_range(
                    tok,
                    (unsafe { (*tok).start }
                        .wrapping_offset(1 as isize)
                        .offset_from((*tok).line_start as *const i8) as i64)
                        as i32,
                    (unsafe { (*tok).cur }.offset_from((*tok).line_start as *mut i8) as i64) as i32,
                    c"'u' and 'r' prefixes are incompatible".as_ptr(),
                )
            };
            return 1i32.wrapping_neg();
        }
    }
    if saw_u != 0 && saw_f != 0 {
        loop {
            unsafe {
                _PyTokenizer_syntaxerror_known_range(
                    tok,
                    (unsafe { (*tok).start }
                        .wrapping_offset(1 as isize)
                        .offset_from((*tok).line_start as *const i8) as i64)
                        as i32,
                    (unsafe { (*tok).cur }.offset_from((*tok).line_start as *mut i8) as i64) as i32,
                    c"'u' and 'f' prefixes are incompatible".as_ptr(),
                )
            };
            return 1i32.wrapping_neg();
        }
    }
    if saw_u != 0 && saw_t != 0 {
        loop {
            unsafe {
                _PyTokenizer_syntaxerror_known_range(
                    tok,
                    (unsafe { (*tok).start }
                        .wrapping_offset(1 as isize)
                        .offset_from((*tok).line_start as *const i8) as i64)
                        as i32,
                    (unsafe { (*tok).cur }.offset_from((*tok).line_start as *mut i8) as i64) as i32,
                    c"'u' and 't' prefixes are incompatible".as_ptr(),
                )
            };
            return 1i32.wrapping_neg();
        }
    }
    if saw_b != 0 && saw_f != 0 {
        loop {
            unsafe {
                _PyTokenizer_syntaxerror_known_range(
                    tok,
                    (unsafe { (*tok).start }
                        .wrapping_offset(1 as isize)
                        .offset_from((*tok).line_start as *const i8) as i64)
                        as i32,
                    (unsafe { (*tok).cur }.offset_from((*tok).line_start as *mut i8) as i64) as i32,
                    c"'b' and 'f' prefixes are incompatible".as_ptr(),
                )
            };
            return 1i32.wrapping_neg();
        }
    }
    if saw_b != 0 && saw_t != 0 {
        loop {
            unsafe {
                _PyTokenizer_syntaxerror_known_range(
                    tok,
                    (unsafe { (*tok).start }
                        .wrapping_offset(1 as isize)
                        .offset_from((*tok).line_start as *const i8) as i64)
                        as i32,
                    (unsafe { (*tok).cur }.offset_from((*tok).line_start as *mut i8) as i64) as i32,
                    c"'b' and 't' prefixes are incompatible".as_ptr(),
                )
            };
            return 1i32.wrapping_neg();
        }
    }
    if saw_f != 0 && saw_t != 0 {
        loop {
            unsafe {
                _PyTokenizer_syntaxerror_known_range(
                    tok,
                    (unsafe { (*tok).start }
                        .wrapping_offset(1 as isize)
                        .offset_from((*tok).line_start as *const i8) as i64)
                        as i32,
                    (unsafe { (*tok).cur }.offset_from((*tok).line_start as *mut i8) as i64) as i32,
                    c"'f' and 't' prefixes are incompatible".as_ptr(),
                )
            };
            return 1i32.wrapping_neg();
        }
    }
    0
}

fn tok_get_normal_mode(
    tok: *mut tok_state,
    current_tok: *mut _tokenizer_mode,
    token: *mut token,
) -> i32 {
    let mut opening: i32 = unsafe { core::mem::zeroed() };
    let mut current_token3: i32 = unsafe { core::mem::zeroed() };
    let mut c3: i32 = unsafe { core::mem::zeroed() };
    let mut current_token: i32 = unsafe { core::mem::zeroed() };
    // Check for two-character token
    let mut c2: i32 = unsafe { core::mem::zeroed() };
    let mut __v21: bool = unsafe { core::mem::zeroed() };
    // This code block gets executed before the curly_bracket_depth is incremented
    // by the `{` case, so for ensuring that we are on the 0th level, we need
    // to adjust it manually
    let mut cursor: i32 = unsafe { core::mem::zeroed() };
    let mut start: i32 = unsafe { core::mem::zeroed() };
    let mut has_escaped_quote: i32 = unsafe { core::mem::zeroed() };
    let mut end_quote_size: i32 = unsafe { core::mem::zeroed() };
    // 1 or 3
    let mut quote_size: i32 = unsafe { core::mem::zeroed() };
    let mut quote: i32 = unsafe { core::mem::zeroed() };
    let mut __v39: i32 = unsafe { core::mem::zeroed() };
    let mut string_kind: u32 = unsafe { core::mem::zeroed() };
    let mut the_current_tok_1: *mut _tokenizer_mode = unsafe { core::mem::zeroed() };
    let mut after_after_quote: i32 = unsafe { core::mem::zeroed() };
    // Find the quote size and start of string
    let mut after_quote: i32 = unsafe { core::mem::zeroed() };
    // 1 or 3
    let mut quote_size_1: i32 = unsafe { core::mem::zeroed() };
    let mut quote_1: i32 = unsafe { core::mem::zeroed() };
    let mut zeros_end: *mut i8 = unsafe { core::mem::zeroed() };
    let mut nonzero: i32 = unsafe { core::mem::zeroed() };
    let mut e: i32 = unsafe { core::mem::zeroed() };
    let mut __v49: bool = unsafe { core::mem::zeroed() };
    let mut saw_t: i32 = unsafe { core::mem::zeroed() };
    let mut saw_f: i32 = unsafe { core::mem::zeroed() };
    let mut saw_u: i32 = unsafe { core::mem::zeroed() };
    let mut saw_r: i32 = unsafe { core::mem::zeroed() };
    // Process the various legal combinations of b"", r"", u"", and f"".
    let mut saw_b: i32 = unsafe { core::mem::zeroed() };
    let mut ignore_end_col_offset: i32 = unsafe { core::mem::zeroed() };
    // +6 in order to skip the word 'ignore'
    let mut ignore_end: *const i8 = unsafe { core::mem::zeroed() };
    let mut is_type_ignore: i32 = unsafe { core::mem::zeroed() };
    let mut current_starting_col_offset: i32 = unsafe { core::mem::zeroed() };
    let mut type_start: *const i8 = unsafe { core::mem::zeroed() };
    let mut prefix: *const i8 = unsafe { core::mem::zeroed() };
    let mut p: *const i8 = unsafe { core::mem::zeroed() };
    let mut __v96: bool = unsafe { core::mem::zeroed() };
    let mut cont_line_col: i32 = unsafe { core::mem::zeroed() };
    let mut altcol: i32 = unsafe { core::mem::zeroed() };
    let mut col: i32 = unsafe { core::mem::zeroed() };
    let mut p_end: *const i8 = unsafe { core::mem::zeroed() };
    let mut p_start: *const i8 = unsafe { core::mem::zeroed() };
    let mut nonascii: i32 = unsafe { core::mem::zeroed() };
    let mut blankline: i32 = unsafe { core::mem::zeroed() };
    let mut c: i32 = unsafe { core::mem::zeroed() };
    unsafe {
        p_start = core::ptr::null::<i8>();
        p_end = core::ptr::null::<i8>();
        '__join_266: {
            '__join_240: {
                '__join_230: {
                    '__join_97: {
                        '__join_78: {
                            '__join_41: {
                                '__join_207: {
                                    '__join_195: {
                                        '__join_192: {
                                            '__join_99: {
                                                '__join_157: {
                                                    '__join_161: {
                                                        '__join_170: {
                                                            '__join_171: {
                                                                '__join_175: {
                                                                    '__join_188: {
                                                                        '__join_180: {
                                                                            loop {
                                                                                (*tok).start =
                                                                                    core::ptr::null::<
                                                                                        i8,
                                                                                    >(
                                                                                    );
                                                                                (*tok).starting_col_offset = 1i32.wrapping_neg();
                                                                                blankline = 0;
                                                                                // Get indentation level
                                                                                if (*tok).atbol != 0
                                                                                {
                                                                                    col = 0;
                                                                                    altcol = 0;
                                                                                    (*tok).atbol =
                                                                                        0;
                                                                                    cont_line_col =
                                                                                        0;
                                                                                    loop {
                                                                                        c = tok_nextc(tok);
                                                                                        if c == 32 {
                                                                                            col = col.wrapping_add(1);
                                                                                            altcol = altcol.wrapping_add(1);
                                                                                        } else {
                                                                                            if c
                                                                                                == 9
                                                                                            {
                                                                                                col = (col / (*tok).tabsize).wrapping_add(1).wrapping_mul((*tok).tabsize);
                                                                                                altcol = (altcol / 1).wrapping_add(1).wrapping_mul(1);
                                                                                            } else {
                                                                                                if c == 12 {
// Control-L (formfeed)
altcol = 0;
col = 0; // For Emacs users
} else {
if c == 92 {
// Indentation cannot be split over multiple physical lines
//
// using backslashes. This means that if we found a backslash
//
// preceded by whitespace, **the first one we find** determines
//
// the level of indentation of whatever comes next.
cont_line_col = if cont_line_col != 0 { cont_line_col } else { col };
let __v95: i32 = tok_continuation_line(tok);
c = __v95;
if __v95 == 1i32.wrapping_neg() {
return MAKE_TOKEN!(67i32, tok, token, p_start, p_end);
}
} else {
break;
}
}
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                    if c == (slate_support::EOF as i32) {
__v96 = !PyErr_Occurred().is_null();
} else {
__v96 = false;
}
                                                                                    if __v96 {
                                                                                        return MAKE_TOKEN!(
                                                                                            67i32,
                                                                                            tok,
                                                                                            token,
                                                                                            p_start,
                                                                                            p_end
                                                                                        );
                                                                                    } else {
                                                                                        tok_backup(
                                                                                            tok, c,
                                                                                        );
                                                                                        if c == 35 || c == 10 || c == 13 {
// Lines with only whitespace and/or comments
// shouldn't affect the indentation and are
// not passed to the parser as NEWLINE tokens,
// except *totally* empty lines in interactive
// mode, which signal the end of a command group.
if col == 0 && c == 10 && !unsafe { (*tok).prompt }.is_null() {
blankline = 0; // Let it through
} else {
if !unsafe { (*tok).prompt }.is_null() && (*tok).lineno == 1 {
// In interactive mode, if the first line contains
// only spaces and/or a comment, let it through.
blankline = 0;
altcol = 0;
col = 0;
} else {
blankline = 1; // Ignore completely
}
}
// We can't jump back right here since we still
// may need to skip to the end of a comment
}
                                                                                        if blankline == 0 && (*tok).level == 0 {
col = if cont_line_col != 0 { cont_line_col } else { col };
altcol = if cont_line_col != 0 { cont_line_col } else { altcol };
if col == *((*tok).indstack.as_mut_ptr() as *mut i32).wrapping_offset((*tok).indent as isize) {
// No change
if altcol != *((*tok).altindstack.as_mut_ptr() as *mut i32).wrapping_offset((*tok).indent as isize) {
return MAKE_TOKEN!(_PyTokenizer_indenterror(tok), tok, token, p_start, p_end);
}
} else {
if col > *((*tok).indstack.as_mut_ptr() as *mut i32).wrapping_offset((*tok).indent as isize) {
// Indent -- always one
if unsafe { (*tok).indent }.wrapping_add(1) >= 100 {
(*tok).done = 20;
(*tok).cur = (*tok).inp;
return MAKE_TOKEN!(67i32, tok, token, p_start, p_end);
} else {
if altcol <= *((*tok).altindstack.as_mut_ptr() as *mut i32).wrapping_offset((*tok).indent as isize) {
return MAKE_TOKEN!(_PyTokenizer_indenterror(tok), tok, token, p_start, p_end);
} else {
let __v83: i32 = (*tok).pendin;
(*tok).pendin = __v83.wrapping_add(1);
let __v80: i32 = (*tok).indent;
let __v79: i32 = __v80.wrapping_add(1);
(*tok).indent = __v79;
*((*tok).indstack.as_mut_ptr() as *mut i32).wrapping_offset(__v79 as isize) = col;
*((*tok).altindstack.as_mut_ptr() as *mut i32).wrapping_offset((*tok).indent as isize) = altcol;
}
}
} else {
// col < tok->indstack[tok->indent]
// Dedent -- any number, must be consistent
while (*tok).indent > 0 && col < *((*tok).indstack.as_mut_ptr() as *mut i32).wrapping_offset((*tok).indent as isize) {
let __v89: i32 = (*tok).pendin;
(*tok).pendin = __v89.wrapping_sub(1);
let __v86: i32 = (*tok).indent;
(*tok).indent = __v86.wrapping_sub(1);
}
if col != *((*tok).indstack.as_mut_ptr() as *mut i32).wrapping_offset((*tok).indent as isize) {
(*tok).done = 21;
(*tok).cur = (*tok).inp;
return MAKE_TOKEN!(67i32, tok, token, p_start, p_end);
} else {
if altcol != *((*tok).altindstack.as_mut_ptr() as *mut i32).wrapping_offset((*tok).indent as isize) {
break;
}
}
}
}
}
                                                                                    }
                                                                                }
                                                                                (*tok).start = (*tok).cur as *const i8;
                                                                                (*tok).starting_col_offset = (*tok).col_offset;
                                                                                // Return pending indents/dedents
                                                                                if (*tok).pendin
                                                                                    != 0
                                                                                {
                                                                                    break '__join_266;
                                                                                } else {
                                                                                    // Peek ahead at the next character
                                                                                    c = tok_nextc(
                                                                                        tok,
                                                                                    );
                                                                                    tok_backup(
                                                                                        tok, c,
                                                                                    );
                                                                                    '__loop_258: loop {
                                                                                        (*tok).start = core::ptr::null::<i8>();
                                                                                        // Skip spaces
                                                                                        loop {
                                                                                            c = tok_nextc(tok);
                                                                                            if !(c == 32 || c == 9 || c == 12) {
break;
}
                                                                                        }
                                                                                        // Set start of current token
                                                                                        (*tok).start = (if unsafe { (*tok).cur }.is_null() { core::ptr::null_mut::<i8>() } else { unsafe { (*tok).cur }.wrapping_offset(-(1 as isize)) }) as *const i8;
                                                                                        (*tok).starting_col_offset = unsafe { (*tok).col_offset }.wrapping_sub(1);
                                                                                        // Skip comment, unless it's a type comment
                                                                                        if c == 35 {
                                                                                            p = core::ptr::null::<i8>();
                                                                                            while c != (slate_support::EOF as i32) && c != 10 && c != 13 {
c = tok_nextc(tok);
}
                                                                                            if (*tok).tok_extra_tokens != 0 {
p = (*tok).start;
}
                                                                                            if (*tok).type_comments != 0 {
p = (*tok).start;
current_starting_col_offset = (*tok).starting_col_offset;
prefix = type_comment_prefix;
while *prefix != 0 && p < ((*tok).cur as *const i8) {
if (*prefix as i32) == 32 {
while (*p as i32) == 32 || (*p as i32) == 9 {
let __v64: *const i8 = p;
p = __v64.wrapping_offset(1 as isize);
current_starting_col_offset = current_starting_col_offset.wrapping_add(1);
}
} else {
if (*prefix as i32) == (*p as i32) {
let __v68: *const i8 = p;
p = __v68.wrapping_offset(1 as isize);
current_starting_col_offset = current_starting_col_offset.wrapping_add(1);
} else {
break;
}
}
let __v60: *const i8 = prefix;
prefix = __v60.wrapping_offset(1 as isize);
}
// This is a type comment if we matched all of type_comment_prefix.
if *prefix == 0 {
break '__join_240;
}
}
                                                                                            if (*tok).tok_extra_tokens != 0 {
tok_backup(tok, c); // don't eat the newline or EOF
p_start = p;
p_end = (*tok).cur as *const i8;
(*tok).comment_newline = blankline;
return MAKE_TOKEN!(65i32, tok, token, p_start, p_end);
}
                                                                                        }
                                                                                        if (*tok)
                                                                                            .done
                                                                                            == 28
                                                                                        {
                                                                                            return MAKE_TOKEN!(0i32, tok, token, p_start, p_end);
                                                                                        } else {
                                                                                            // Check for EOF and errors now
                                                                                            if c == (slate_support::EOF as i32) {
break '__join_230;
} else {
'__join_79: {
// Identifier (most frequent token!)
nonascii = 0;
if is_potential_identifier_start!(c) {
saw_b = 0;
saw_r = 0;
saw_u = 0;
saw_f = 0;
saw_t = 0;
loop {
if true {
if saw_b == 0 && (c == 98 || c == 66) {
saw_b = 1;
} else {
if saw_u == 0 && (c == 117 || c == 85) {
saw_u = 1;
} else {
if saw_r == 0 && (c == 114 || c == 82) {
saw_r = 1;
} else {
if saw_f == 0 && (c == 102 || c == 70) {
saw_f = 1;
} else {
if saw_t == 0 && (c == 116 || c == 84) {
saw_t = 1;
} else {
break '__join_207;
}
}
}
}
}
// Since this is a backwards compatibility support literal we don't
// want to support it in arbitrary order like byte literals.
// ur"" and ru"" are not supported
c = tok_nextc(tok);
if c == 34 || c == 39 {
break;
}
} else {
break '__join_207;
}
}
// Raise error on incompatible string prefixes:
let status: i32 = maybe_raise_syntax_error_for_string_prefixes(tok, saw_b, saw_r, saw_u, saw_f, saw_t);
if status < 0 {
return MAKE_TOKEN!(67i32, tok, token, p_start, p_end);
} else {
// Handle valid f or t string creation:
if !(saw_f != 0 || saw_t != 0) {
break '__join_79;
}
}
} else {
if c == 13 {
c = tok_nextc(tok);
}
// Newline
if c == 10 {
break '__loop_258;
} else {
// Period or number starting with period?
if c == 46 {
break '__join_188;
} else {
// Number
if Py_ISDIGIT!(c) != 0 {
break '__join_180;
}
}
}
}
if ((Py_TOLOWER!(unsafe { *(*tok).start }) as i32) == 102 || (Py_TOLOWER!(unsafe { *(*tok).start }) as i32) == 114 || (Py_TOLOWER!(unsafe { *(*tok).start }) as i32) == 116) && (c == 39 || c == 34) {
break '__join_97;
}
}
if c == 39 || c == 34 {
break '__join_78;
} else {
// String
// Line continuation
if c == 92 {
let __v27: i32 = tok_continuation_line(tok);
c = __v27;
if __v27 == 1i32.wrapping_neg() {
return MAKE_TOKEN!(67i32, tok, token, p_start, p_end);
} else {
(*tok).cont_line = 1;
}
} else {
break '__join_41;
}
}
}
                                                                                        }
                                                                                    }
                                                                                    (*tok).atbol =
                                                                                        1;
                                                                                    if blankline
                                                                                        != 0
                                                                                        || (*tok)
                                                                                            .level
                                                                                            > 0
                                                                                    {
                                                                                        if (*tok).tok_extra_tokens != 0 {
break '__join_195;
}
                                                                                    } else {
                                                                                        break '__join_192;
                                                                                    }
                                                                                }
                                                                            }
                                                                            return MAKE_TOKEN!(_PyTokenizer_indenterror(tok), tok, token, p_start, p_end);
                                                                        }
                                                                        if c == 48 {
                                                                            // Hex, octal or binary -- maybe.
                                                                            c = tok_nextc(tok);
                                                                            if c == 120 || c == 88 {
                                                                                // Hex
                                                                                c = tok_nextc(tok);
                                                                                loop {
                                                                                    if c == 95 {
                                                                                        c = tok_nextc(tok);
                                                                                    }
                                                                                    if Py_ISXDIGIT!(
                                                                                        c
                                                                                    ) == 0
                                                                                    {
                                                                                        tok_backup(
                                                                                            tok, c,
                                                                                        );
                                                                                        return MAKE_TOKEN!(_PyTokenizer_syntaxerror(tok, c"invalid hexadecimal literal".as_ptr()), tok, token, p_start, p_end);
                                                                                    } else {
                                                                                        loop {
                                                                                            c = tok_nextc(tok);
                                                                                            if Py_ISXDIGIT!(
                                                                                                c
                                                                                            )
                                                                                                == 0
                                                                                            {
                                                                                                break;
                                                                                            }
                                                                                        }
                                                                                        if c != 95 {
                                                                                            break;
                                                                                        }
                                                                                    }
                                                                                }
                                                                                if verify_end_of_number(tok, c, c"hexadecimal".as_ptr()) == 0 {
return MAKE_TOKEN!(67i32, tok, token, p_start, p_end);
} else {
break '__join_99;
}
                                                                            } else {
                                                                                if c == 111
                                                                                    || c == 79
                                                                                {
                                                                                    // Octal
                                                                                    c = tok_nextc(
                                                                                        tok,
                                                                                    );
                                                                                    '__join_117: {
                                                                                        loop {
                                                                                            if c == 95 {
c = tok_nextc(tok);
}
                                                                                            if c < 48 || c >= 56 {
break '__join_117;
} else {
loop {
c = tok_nextc(tok);
if !(48 <= c && c < 56) {
break;
}
}
if c != 95 {
break;
}
}
                                                                                        }
                                                                                        if Py_ISDIGIT!(
                                                                                            c
                                                                                        ) != 0
                                                                                        {
                                                                                            return MAKE_TOKEN!(_PyTokenizer_syntaxerror(tok, c"invalid digit '%c' in octal literal".as_ptr(), c), tok, token, p_start, p_end);
                                                                                        } else {
                                                                                            if verify_end_of_number(tok, c, c"octal".as_ptr()) == 0 {
return MAKE_TOKEN!(67i32, tok, token, p_start, p_end);
} else {
break '__join_99;
}
                                                                                        }
                                                                                    }
                                                                                    if Py_ISDIGIT!(
                                                                                        c
                                                                                    ) != 0
                                                                                    {
                                                                                        return MAKE_TOKEN!(_PyTokenizer_syntaxerror(tok, c"invalid digit '%c' in octal literal".as_ptr(), c), tok, token, p_start, p_end);
                                                                                    } else {
                                                                                        tok_backup(
                                                                                            tok, c,
                                                                                        );
                                                                                        return MAKE_TOKEN!(_PyTokenizer_syntaxerror(tok, c"invalid octal literal".as_ptr()), tok, token, p_start, p_end);
                                                                                    }
                                                                                } else {
                                                                                    if c == 98
                                                                                        || c == 66
                                                                                    {
                                                                                        // Binary
                                                                                        c = tok_nextc(tok);
                                                                                        '__join_130: {
                                                                                            loop {
                                                                                                if c == 95 {
c = tok_nextc(tok);
}
                                                                                                if c != 48 && c != 49 {
break '__join_130;
} else {
loop {
c = tok_nextc(tok);
if !(c == 48 || c == 49) {
break;
}
}
if c != 95 {
break;
}
}
                                                                                            }
                                                                                            if Py_ISDIGIT!(
                                                                                                c
                                                                                            )
                                                                                                != 0
                                                                                            {
                                                                                                return MAKE_TOKEN!(_PyTokenizer_syntaxerror(tok, c"invalid digit '%c' in binary literal".as_ptr(), c), tok, token, p_start, p_end);
                                                                                            } else {
                                                                                                if verify_end_of_number(tok, c, c"binary".as_ptr()) == 0 {
return MAKE_TOKEN!(67i32, tok, token, p_start, p_end);
} else {
break '__join_99;
}
                                                                                            }
                                                                                        }
                                                                                        if Py_ISDIGIT!(
                                                                                            c
                                                                                        ) != 0
                                                                                        {
                                                                                            return MAKE_TOKEN!(_PyTokenizer_syntaxerror(tok, c"invalid digit '%c' in binary literal".as_ptr(), c), tok, token, p_start, p_end);
                                                                                        } else {
                                                                                            tok_backup(tok, c);
                                                                                            return MAKE_TOKEN!(_PyTokenizer_syntaxerror(tok, c"invalid binary literal".as_ptr()), tok, token, p_start, p_end);
                                                                                        }
                                                                                    } else {
                                                                                        nonzero = 0;
                                                                                        // maybe old-style octal; c is first char of it
                                                                                        //
                                                                                        // in any case, allow '0' as a literal
                                                                                        while true {
                                                                                            if c == 95 {
c = tok_nextc(tok);
if Py_ISDIGIT!(c) == 0 {
tok_backup(tok, c);
return MAKE_TOKEN!(_PyTokenizer_syntaxerror(tok, c"invalid decimal literal".as_ptr()), tok, token, p_start, p_end);
}
}
                                                                                            if c != 48 {
break;
} else {
c = tok_nextc(tok);
}
                                                                                        }
                                                                                        zeros_end =
                                                                                            (*tok)
                                                                                                .cur;
                                                                                        if Py_ISDIGIT!(
                                                                                            c
                                                                                        ) != 0
                                                                                        {
                                                                                            nonzero = 1;
                                                                                            c = tok_decimal_tail(tok);
                                                                                            if c
                                                                                                == 0
                                                                                            {
                                                                                                return MAKE_TOKEN!(67i32, tok, token, p_start, p_end);
                                                                                            }
                                                                                        }
                                                                                        if c == 46 {
                                                                                            c = tok_nextc(tok);
                                                                                            break '__join_175;
                                                                                        } else {
                                                                                            if c == 101 || c == 69 {
break '__join_170;
} else {
if c == 106 || c == 74 {
break '__join_157;
} else {
if nonzero != 0 && (*tok).tok_extra_tokens == 0 {
// Old-style octal: now disallowed.
tok_backup(tok, c);
return MAKE_TOKEN!(_PyTokenizer_syntaxerror_known_range(tok, (unsafe { (*tok).start }.wrapping_offset(1 as isize).offset_from((*tok).line_start as *const i8) as i64) as i32, (zeros_end.offset_from((*tok).line_start as *mut i8) as i64) as i32, c"leading zeros in decimal integer literals are not permitted; use an 0o prefix for octal integers".as_ptr()), tok, token, p_start, p_end);
} else {
if verify_end_of_number(tok, c, c"decimal".as_ptr()) == 0 {
return MAKE_TOKEN!(67i32, tok, token, p_start, p_end);
} else {
break '__join_99;
}
}
}
}
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        } else {
                                                                            // Decimal
                                                                            c = tok_decimal_tail(
                                                                                tok,
                                                                            );
                                                                            if c == 0 {
                                                                                return MAKE_TOKEN!(
                                                                                    67i32, tok,
                                                                                    token, p_start,
                                                                                    p_end
                                                                                );
                                                                            } else {
                                                                                // Accept floating-point numbers.
                                                                                if c == 46 {
                                                                                    c = tok_nextc(
                                                                                        tok,
                                                                                    );
                                                                                    break '__join_175;
                                                                                } else {
                                                                                    break '__join_171;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    c = tok_nextc(tok);
                                                                    if Py_ISDIGIT!(c) == 0 {
                                                                        if c == 46 {
                                                                            c = tok_nextc(tok);
                                                                            if c == 46 {
                                                                                p_start =
                                                                                    (*tok).start;
                                                                                p_end = (*tok).cur
                                                                                    as *const i8;
                                                                                return MAKE_TOKEN!(
                                                                                    52i32, tok,
                                                                                    token, p_start,
                                                                                    p_end
                                                                                );
                                                                            } else {
                                                                                tok_backup(tok, c);
                                                                                tok_backup(
                                                                                    tok, 46i32,
                                                                                );
                                                                            }
                                                                        } else {
                                                                            tok_backup(tok, c);
                                                                        }
                                                                        p_start = (*tok).start;
                                                                        p_end =
                                                                            (*tok).cur as *const i8;
                                                                        return MAKE_TOKEN!(
                                                                            23i32, tok, token,
                                                                            p_start, p_end
                                                                        );
                                                                    }
                                                                }
                                                                if Py_ISDIGIT!(c) != 0 {
                                                                    c = tok_decimal_tail(tok);
                                                                    if c == 0 {
                                                                        return MAKE_TOKEN!(
                                                                            67i32, tok, token,
                                                                            p_start, p_end
                                                                        );
                                                                    }
                                                                }
                                                                // Fraction
                                                            }
                                                            if !(c == 101 || c == 69) {
                                                                break '__join_161;
                                                            }
                                                        }
                                                        e = c;
                                                        // Exponent part
                                                        c = tok_nextc(tok);
                                                        if c == 43 || c == 45 {
                                                            c = tok_nextc(tok);
                                                            if Py_ISDIGIT!(c) == 0 {
                                                                tok_backup(tok, c);
                                                                return MAKE_TOKEN!(
                                                                    _PyTokenizer_syntaxerror(
                                                                        tok,
                                                                        c"invalid decimal literal"
                                                                            .as_ptr()
                                                                    ),
                                                                    tok,
                                                                    token,
                                                                    p_start,
                                                                    p_end
                                                                );
                                                            }
                                                        } else {
                                                            if Py_ISDIGIT!(c) == 0 {
                                                                tok_backup(tok, c);
                                                                if verify_end_of_number(
                                                                    tok,
                                                                    e,
                                                                    c"decimal".as_ptr(),
                                                                ) == 0
                                                                {
                                                                    return MAKE_TOKEN!(
                                                                        67i32, tok, token, p_start,
                                                                        p_end
                                                                    );
                                                                } else {
                                                                    tok_backup(tok, e);
                                                                    p_start = (*tok).start;
                                                                    p_end = (*tok).cur as *const i8;
                                                                    return MAKE_TOKEN!(
                                                                        2i32, tok, token, p_start,
                                                                        p_end
                                                                    );
                                                                }
                                                            }
                                                        }
                                                        c = tok_decimal_tail(tok);
                                                        if c == 0 {
                                                            return MAKE_TOKEN!(
                                                                67i32, tok, token, p_start, p_end
                                                            );
                                                        }
                                                    }
                                                    if c == 106 || c == 74 {
                                                        // Imaginary part
                                                    } else {
                                                        if verify_end_of_number(
                                                            tok,
                                                            c,
                                                            c"decimal".as_ptr(),
                                                        ) == 0
                                                        {
                                                            return MAKE_TOKEN!(
                                                                67i32, tok, token, p_start, p_end
                                                            );
                                                        } else {
                                                            break '__join_99;
                                                        }
                                                    }
                                                }
                                                c = tok_nextc(tok);
                                                if verify_end_of_number(
                                                    tok,
                                                    c,
                                                    c"imaginary".as_ptr(),
                                                ) == 0
                                                {
                                                    return MAKE_TOKEN!(
                                                        67i32, tok, token, p_start, p_end
                                                    );
                                                }
                                            }
                                            tok_backup(tok, c);
                                            p_start = (*tok).start;
                                            p_end = (*tok).cur as *const i8;
                                            return MAKE_TOKEN!(2i32, tok, token, p_start, p_end);
                                        }
                                        if (*tok).comment_newline != 0
                                            && (*tok).tok_extra_tokens != 0
                                        {
                                            (*tok).comment_newline = 0;
                                            p_start = (*tok).start;
                                            p_end = (*tok).cur as *const i8;
                                            return MAKE_TOKEN!(66i32, tok, token, p_start, p_end);
                                        } else {
                                            p_start = (*tok).start;
                                            p_end = unsafe { (*tok).cur }
                                                .wrapping_offset(-(1 as isize))
                                                as *const i8; // Leave '\n' out of the string
                                            (*tok).cont_line = 0;
                                            return MAKE_TOKEN!(4i32, tok, token, p_start, p_end);
                                        }
                                    }
                                    if (*tok).comment_newline != 0 {
                                        (*tok).comment_newline = 0;
                                    }
                                    p_start = (*tok).start;
                                    p_end = (*tok).cur as *const i8;
                                    return MAKE_TOKEN!(66i32, tok, token, p_start, p_end);
                                }
                                while is_potential_identifier_char!(c) {
                                    if c >= 128 {
                                        nonascii = 1;
                                    }
                                    c = tok_nextc(tok);
                                }
                                tok_backup(tok, c);
                                if nonascii != 0 {
                                    __v49 = verify_identifier(tok) == 0;
                                } else {
                                    __v49 = false;
                                }
                                if __v49 {
                                    return MAKE_TOKEN!(67i32, tok, token, p_start, p_end);
                                } else {
                                    p_start = (*tok).start;
                                    p_end = (*tok).cur as *const i8;
                                    return MAKE_TOKEN!(1i32, tok, token, p_start, p_end);
                                }
                            }
                            // Punctuation character
                            let is_punctuation: i32 =
                                (c == 58 || c == 125 || c == 33 || c == 123) as i32;
                            if is_punctuation != 0
                                && INSIDE_FSTRING!(tok)
                                && INSIDE_FSTRING_EXPR!(current_tok)
                            {
                                cursor = unsafe { (*current_tok).curly_bracket_depth }
                                    .wrapping_sub((c != 123) as i32);
                                let in_format_spec: i32 = (*current_tok).in_format_spec;
                                let cursor_in_format_with_debug: i32 = (cursor == 1
                                    && ((*current_tok).in_debug != 0 || in_format_spec != 0))
                                    as i32;
                                let cursor_valid: i32 =
                                    (cursor == 0 || cursor_in_format_with_debug != 0) as i32;
                                if cursor_valid != 0 && c != 123 {
                                    __v21 = set_ftstring_expr(tok, token, c as i8) != 0;
                                } else {
                                    __v21 = false;
                                }
                                if __v21 {
                                    return MAKE_TOKEN!(67i32, tok, token, p_start, p_end);
                                } else {
                                    if c == 58
                                        && cursor == (*current_tok).curly_bracket_expr_start_depth
                                    {
                                        (*current_tok).kind = 1;
                                        (*current_tok).in_format_spec = 1;
                                        p_start = (*tok).start;
                                        p_end = (*tok).cur as *const i8;
                                        return MAKE_TOKEN!(
                                            _PyToken_OneChar(c),
                                            tok,
                                            token,
                                            p_start,
                                            p_end
                                        );
                                    }
                                }
                            }
                            c2 = tok_nextc(tok);
                            current_token = _PyToken_TwoChars(c, c2);
                            if c == 60 && c2 == 62 && (*tok).barry_as_bdfl == 0 {
                                current_token = 55;
                            }
                            if current_token != 55 {
                                c3 = tok_nextc(tok);
                                current_token3 = _PyToken_ThreeChars(c, c2, c3);
                                if current_token3 != 55 {
                                    current_token = current_token3;
                                } else {
                                    tok_backup(tok, c3);
                                }
                                p_start = (*tok).start;
                                p_end = (*tok).cur as *const i8;
                                return MAKE_TOKEN!(current_token, tok, token, p_start, p_end);
                            } else {
                                '__join_4: {
                                    '__join_25: {
                                        tok_backup(tok, c2);
                                        // Keep track of parentheses nesting level
                                        let __t0: i32 = c;
                                        if __t0 == 40 {
                                            break '__join_25;
                                        } else {
                                            if __t0 == 91 {
                                                break '__join_25;
                                            } else {
                                                if __t0 == 123 {
                                                    break '__join_25;
                                                } else {
                                                    if __t0 != 41 {
                                                        if __t0 != 93 {
                                                            if __t0 != 125 {
                                                                break '__join_4;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        if INSIDE_FSTRING!(tok)
                                            && (*current_tok).curly_bracket_depth == 0
                                            && c == 125
                                        {
                                            return MAKE_TOKEN!(
                                                _PyTokenizer_syntaxerror(
                                                    tok,
                                                    c"%c-string: single '}' is not allowed"
                                                        .as_ptr(),
                                                    TOK_GET_STRING_PREFIX!(tok)
                                                ),
                                                tok,
                                                token,
                                                p_start,
                                                p_end
                                            );
                                        } else {
                                            if (*tok).tok_extra_tokens == 0 && (*tok).level == 0 {
                                                return MAKE_TOKEN!(
                                                    _PyTokenizer_syntaxerror(
                                                        tok,
                                                        c"unmatched '%c'".as_ptr(),
                                                        c
                                                    ),
                                                    tok,
                                                    token,
                                                    p_start,
                                                    p_end
                                                );
                                            } else {
                                                if (*tok).level > 0 {
                                                    let __v9: i32 = (*tok).level;
                                                    (*tok).level = __v9.wrapping_sub(1);
                                                    opening = *((*tok).parenstack.as_mut_ptr()
                                                        as *mut i8)
                                                        .wrapping_offset((*tok).level as isize)
                                                        as i32;
                                                    if (*tok).tok_extra_tokens == 0
                                                        && !(opening == 40 && c == 41
                                                            || opening == 91 && c == 93
                                                            || opening == 123 && c == 125)
                                                    {
                                                        // If the opening bracket belongs to an f-string's expression
                                                        // part (e.g. f"{)}") and the closing bracket is an arbitrary
                                                        // nested expression, then instead of matching a different
                                                        // syntactical construct with it; we'll throw an unmatched
                                                        // parentheses error.
                                                        if INSIDE_FSTRING!(tok) && opening == 123 {
                                                            let previous_bracket: i32 = unsafe {
                                                                (*current_tok).curly_bracket_depth
                                                            }
                                                            .wrapping_sub(1);
                                                            if previous_bracket
                                                                == (*current_tok)
                                                                    .curly_bracket_expr_start_depth
                                                            {
                                                                return MAKE_TOKEN!(_PyTokenizer_syntaxerror(tok, c"%c-string: unmatched '%c'".as_ptr(), TOK_GET_STRING_PREFIX!(tok), c), tok, token, p_start, p_end);
                                                            }
                                                        }
                                                        if *((*tok).parenlinenostack.as_mut_ptr()
                                                            as *mut i32)
                                                            .wrapping_offset((*tok).level as isize)
                                                            != (*tok).lineno
                                                        {
                                                            return MAKE_TOKEN!(_PyTokenizer_syntaxerror(tok, c"closing parenthesis '%c' does not match opening parenthesis '%c' on line %d".as_ptr(), c, opening, *((*tok).parenlinenostack.as_mut_ptr() as *mut i32).wrapping_offset((*tok).level as isize)), tok, token, p_start, p_end);
                                                        } else {
                                                            return MAKE_TOKEN!(_PyTokenizer_syntaxerror(tok, c"closing parenthesis '%c' does not match opening parenthesis '%c'".as_ptr(), c, opening), tok, token, p_start, p_end);
                                                        }
                                                    }
                                                }
                                                if INSIDE_FSTRING!(tok) {
                                                    let __v4: i32 =
                                                        (*current_tok).curly_bracket_depth;
                                                    (*current_tok).curly_bracket_depth =
                                                        __v4.wrapping_sub(1);
                                                    if (*current_tok).curly_bracket_depth < 0 {
                                                        return MAKE_TOKEN!(
                                                            _PyTokenizer_syntaxerror(
                                                                tok,
                                                                c"%c-string: unmatched '%c'"
                                                                    .as_ptr(),
                                                                TOK_GET_STRING_PREFIX!(tok),
                                                                c
                                                            ),
                                                            tok,
                                                            token,
                                                            p_start,
                                                            p_end
                                                        );
                                                    } else {
                                                        if c == 125
                                                            && (*current_tok).curly_bracket_depth
                                                                == (*current_tok)
                                                                    .curly_bracket_expr_start_depth
                                                        {
                                                            let __v1: i32 = (*current_tok)
                                                                .curly_bracket_expr_start_depth;
                                                            (*current_tok)
                                                                .curly_bracket_expr_start_depth =
                                                                __v1.wrapping_sub(1);
                                                            (*current_tok).kind = 1;
                                                            (*current_tok).in_format_spec = 0;
                                                            (*current_tok).in_debug = 0;
                                                            break '__join_4;
                                                        } else {
                                                            break '__join_4;
                                                        }
                                                    }
                                                } else {
                                                    break '__join_4;
                                                }
                                            }
                                        }
                                    }
                                    if (*tok).level >= 200 {
                                        return MAKE_TOKEN!(
                                            _PyTokenizer_syntaxerror(
                                                tok,
                                                c"too many nested parentheses".as_ptr()
                                            ),
                                            tok,
                                            token,
                                            p_start,
                                            p_end
                                        );
                                    } else {
                                        *((*tok).parenstack.as_mut_ptr() as *mut i8)
                                            .wrapping_offset((*tok).level as isize) = c as i8;
                                        *((*tok).parenlinenostack.as_mut_ptr() as *mut i32)
                                            .wrapping_offset((*tok).level as isize) = (*tok).lineno;
                                        *((*tok).parencolstack.as_mut_ptr() as *mut i32)
                                            .wrapping_offset((*tok).level as isize) =
                                            (unsafe { (*tok).start }
                                                .offset_from((*tok).line_start as *const i8)
                                                as i64)
                                                as i32;
                                        let __v15: i32 = (*tok).level;
                                        (*tok).level = __v15.wrapping_add(1);
                                        if INSIDE_FSTRING!(tok) {
                                            let __v12: i32 = (*current_tok).curly_bracket_depth;
                                            (*current_tok).curly_bracket_depth =
                                                __v12.wrapping_add(1);
                                        }
                                    }
                                }
                                if Py_UNICODE_ISPRINTABLE__2!(c) == 0 {
                                    return MAKE_TOKEN!(
                                        _PyTokenizer_syntaxerror(
                                            tok,
                                            c"invalid non-printable character U+%04X".as_ptr(),
                                            c
                                        ),
                                        tok,
                                        token,
                                        p_start,
                                        p_end
                                    );
                                } else {
                                    if c == 61 && INSIDE_FSTRING_EXPR_AT_TOP!(current_tok) {
                                        (*current_tok).in_debug = 1;
                                    }
                                    // Punctuation character
                                    p_start = (*tok).start;
                                    p_end = (*tok).cur as *const i8;
                                    return MAKE_TOKEN!(
                                        _PyToken_OneChar(c),
                                        tok,
                                        token,
                                        p_start,
                                        p_end
                                    );
                                }
                            }
                        }
                        quote = c;
                        quote_size = 1;
                        end_quote_size = 0;
                        has_escaped_quote = 0;
                        // Nodes of type STRING, especially multi line strings
                        // must be handled differently in order to get both
                        // the starting line number and the column offset right.
                        // (cf. issue 16806)
                        (*tok).first_lineno = (*tok).lineno;
                        (*tok).multi_line_start = (*tok).line_start;
                        // Find the quote size and start of string
                        c = tok_nextc(tok);
                        if c == quote {
                            c = tok_nextc(tok);
                            if c == quote {
                                quote_size = 3;
                            } else {
                                end_quote_size = 1; // empty string found
                            }
                        }
                        if c != quote {
                            tok_backup(tok, c);
                        }
                        // Get rest of string
                        '__join_46: {
                            loop {
                                if end_quote_size != quote_size {
                                    c = tok_nextc(tok);
                                    if (*tok).done == 17 {
                                        return MAKE_TOKEN!(67i32, tok, token, p_start, p_end);
                                    } else {
                                        if (*tok).done == 22 {
                                            break '__join_46;
                                        } else {
                                            if c == (slate_support::EOF as i32)
                                                || quote_size == 1 && c == 10
                                            {
                                                break;
                                            } else {
                                                if c == quote {
                                                    end_quote_size = end_quote_size.wrapping_add(1);
                                                } else {
                                                    end_quote_size = 0;
                                                    if c == 92 {
                                                        c = tok_nextc(tok); // skip escaped char
                                                        if c == quote {
                                                            // but record whether the escaped char was a quote
                                                            has_escaped_quote = 1;
                                                        }
                                                        if c == 13 {
                                                            c = tok_nextc(tok);
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    break '__join_46;
                                }
                            }
                            // shift the tok_state's location into
                            //
                            // the start of string, and report the error
                            //
                            // from the initial quote character
                            (*tok).cur = (*tok).start as *mut i8;
                            let __v33: *mut i8 = (*tok).cur;
                            (*tok).cur = __v33.wrapping_offset(1 as isize);
                            (*tok).line_start = (*tok).multi_line_start;
                            start = (*tok).lineno;
                            (*tok).lineno = (*tok).first_lineno;
                            if INSIDE_FSTRING!(tok) {
                                // When we are in an f-string, before raising the
                                // unterminated string literal error, check whether
                                // does the initial quote matches with f-strings quotes
                                // and if it is, then this must be a missing '}' token
                                // so raise the proper error
                                let the_current_tok: *mut _tokenizer_mode = TOK_GET_MODE!(tok);
                                if ((*the_current_tok).quote as i32) == quote
                                    && (*the_current_tok).quote_size == quote_size
                                {
                                    return MAKE_TOKEN!(
                                        _PyTokenizer_syntaxerror(
                                            tok,
                                            c"%c-string: expecting '}'".as_ptr(),
                                            TOK_GET_STRING_PREFIX!(tok)
                                        ),
                                        tok,
                                        token,
                                        p_start,
                                        p_end
                                    );
                                }
                            }
                            if quote_size == 3 {
                                _PyTokenizer_syntaxerror(tok, c"unterminated triple-quoted string literal (detected at line %d)".as_ptr(), start);
                                if c != 10 {
                                    (*tok).done = 23;
                                }
                                return MAKE_TOKEN!(67i32, tok, token, p_start, p_end);
                            } else {
                                if has_escaped_quote != 0 {
                                    _PyTokenizer_syntaxerror(tok, c"unterminated string literal (detected at line %d); perhaps you escaped the end quote?".as_ptr(), start);
                                } else {
                                    _PyTokenizer_syntaxerror(
                                        tok,
                                        c"unterminated string literal (detected at line %d)"
                                            .as_ptr(),
                                        start,
                                    );
                                }
                                if c != 10 {
                                    (*tok).done = 24;
                                }
                                return MAKE_TOKEN!(67i32, tok, token, p_start, p_end);
                            }
                        }
                        p_start = (*tok).start;
                        p_end = (*tok).cur as *const i8;
                        return MAKE_TOKEN!(3i32, tok, token, p_start, p_end);
                    }
                    quote_1 = c;
                    quote_size_1 = 1;
                    // Nodes of type STRING, especially multi line strings
                    // must be handled differently in order to get both
                    // the starting line number and the column offset right.
                    // (cf. issue 16806)
                    (*tok).first_lineno = (*tok).lineno;
                    (*tok).multi_line_start = (*tok).line_start;
                    after_quote = tok_nextc(tok);
                    if after_quote == quote_1 {
                        after_after_quote = tok_nextc(tok);
                        if after_after_quote == quote_1 {
                            quote_size_1 = 3;
                        } else {
                            // TODO: Check this
                            tok_backup(tok, after_after_quote);
                            tok_backup(tok, after_quote);
                        }
                    }
                    if after_quote != quote_1 {
                        tok_backup(tok, after_quote);
                    }
                    p_start = (*tok).start;
                    p_end = (*tok).cur as *const i8;
                    if unsafe { (*tok).tok_mode_stack_index }.wrapping_add(1) >= 150 {
                        return MAKE_TOKEN!(
                            _PyTokenizer_syntaxerror(
                                tok,
                                c"too many nested f-strings or t-strings".as_ptr()
                            ),
                            tok,
                            token,
                            p_start,
                            p_end
                        );
                    } else {
                        '__join_83: {
                            '__join_88: {
                                '__join_87: {
                                    the_current_tok_1 = TOK_NEXT_MODE!(tok);
                                    (*the_current_tok_1).kind = 1;
                                    (*the_current_tok_1).quote = quote_1 as i8;
                                    (*the_current_tok_1).quote_size = quote_size_1;
                                    (*the_current_tok_1).start = (*tok).start;
                                    (*the_current_tok_1).multi_line_start = (*tok).line_start;
                                    (*the_current_tok_1).first_line = (*tok).lineno;
                                    (*the_current_tok_1).start_offset = 1i32.wrapping_neg() as i64;
                                    (*the_current_tok_1).multi_line_start_offset =
                                        1i32.wrapping_neg() as i64;
                                    (*the_current_tok_1).last_expr_start = core::ptr::null::<i8>();
                                    (*the_current_tok_1).last_expr_start_offset =
                                        1i32.wrapping_neg() as i64;
                                    (*the_current_tok_1).in_format_spec = 0;
                                    (*the_current_tok_1).in_debug = 0;
                                    string_kind = 0;
                                    let __t1: i32 = *(*tok).start as i32;
                                    if __t1 == 84 {
                                        break '__join_88;
                                    } else {
                                        if __t1 == 116 {
                                            break '__join_88;
                                        } else {
                                            if __t1 == 70 {
                                                break '__join_87;
                                            } else {
                                                if __t1 == 102 {
                                                    break '__join_87;
                                                } else {
                                                    if __t1 != 82 {
                                                        if __t1 != 114 {
                                                            Py_UNREACHABLE!();
                                                            break '__join_83;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    (*the_current_tok_1).raw = 1;
                                    if (Py_TOLOWER!(unsafe {
                                        *unsafe { (*tok).start }.wrapping_offset(1 as isize)
                                    }) as i32)
                                        == 116
                                    {
                                        string_kind = 1;
                                        break '__join_83;
                                    } else {
                                        break '__join_83;
                                    }
                                }
                                (*the_current_tok_1).raw =
                                    ((Py_TOLOWER!(unsafe {
                                        *unsafe { (*tok).start }.wrapping_offset(1 as isize)
                                    }) as i32)
                                        == 114) as i32;
                                break '__join_83;
                            }
                            (*the_current_tok_1).raw = ((Py_TOLOWER!(unsafe {
                                *unsafe { (*tok).start }.wrapping_offset(1 as isize)
                            }) as i32)
                                == 114)
                                as i32;
                            string_kind = 1;
                        }
                        (*the_current_tok_1).string_kind = string_kind;
                        (*the_current_tok_1).curly_bracket_depth = 0;
                        (*the_current_tok_1).curly_bracket_expr_start_depth = 1i32.wrapping_neg();
                        if (string_kind as u32) == 1 {
                            __v39 = MAKE_TOKEN!(62i32, tok, token, p_start, p_end);
                        } else {
                            __v39 = MAKE_TOKEN!(59i32, tok, token, p_start, p_end);
                        }
                        return __v39;
                    }
                }
                if (*tok).level != 0 {
                    return MAKE_TOKEN!(67i32, tok, token, p_start, p_end);
                } else {
                    return MAKE_TOKEN!(
                        if (*tok).done == 11 { 0i32 } else { 67i32 },
                        tok,
                        token,
                        p_start,
                        p_end
                    );
                }
            }
            is_type_ignore = 1;
            ignore_end = p.wrapping_offset(6 as isize);
            ignore_end_col_offset = current_starting_col_offset.wrapping_add(6);
            tok_backup(tok, c); // don't eat the newline or EOF
            type_start = p;
            // A TYPE_IGNORE is "type: ignore" followed by the end of the token
            // or anything ASCII and non-alphanumeric.
            is_type_ignore = ((*tok).cur >= (ignore_end as *mut i8)
                && (memcmp(
                    (p as *const ()) as *const core::ffi::c_void,
                    (b"ignore\0".as_ptr() as *const ()) as *const core::ffi::c_void,
                    6u64 as usize,
                ) as i32)
                    == 0
                && !((*tok).cur > (ignore_end as *mut i8)
                    && (((*ignore_end.wrapping_offset(0 as isize) as u8) as i32) >= 128
                        || Py_ISALNUM!(unsafe { *ignore_end.wrapping_offset(0 as isize) }) != 0)))
                as i32;
            if is_type_ignore != 0 {
                p_start = ignore_end;
                p_end = (*tok).cur as *const i8;
                // If this type ignore is the only thing on the line, consume the newline also.
                if blankline != 0 {
                    tok_nextc(tok);
                    (*tok).atbol = 1;
                }
                return MAKE_TYPE_COMMENT_TOKEN!(
                    56i32,
                    ignore_end_col_offset,
                    unsafe { (*tok).col_offset },
                    tok,
                    token,
                    p_start,
                    p_end
                );
            } else {
                p_start = type_start;
                p_end = (*tok).cur as *const i8;
                return MAKE_TYPE_COMMENT_TOKEN!(
                    57i32,
                    current_starting_col_offset,
                    unsafe { (*tok).col_offset },
                    tok,
                    token,
                    p_start,
                    p_end
                );
            }
        }
        if (*tok).pendin < 0 {
            if (*tok).tok_extra_tokens != 0 {
                p_start = (*tok).cur as *const i8;
                p_end = (*tok).cur as *const i8;
            }
            let __v74: i32 = (*tok).pendin;
            (*tok).pendin = __v74.wrapping_add(1);
            return MAKE_TOKEN!(6i32, tok, token, p_start, p_end);
        } else {
            if (*tok).tok_extra_tokens != 0 {
                p_start = (*tok).buf as *const i8;
                p_end = (*tok).cur as *const i8;
            }
            let __v77: i32 = (*tok).pendin;
            (*tok).pendin = __v77.wrapping_sub(1);
            return MAKE_TOKEN!(5i32, tok, token, p_start, p_end);
        }
    }
    // Read next line
}

fn tok_get_fstring_mode(
    tok: *mut tok_state,
    current_tok: *mut _tokenizer_mode,
    token: *mut token,
) -> i32 {
    // Backup the f-string quotes to emit a final FSTRING_MIDDLE and
    //
    // add the quotes to the FSTRING_END in the next tokenizer iteration.
    let mut i: i32 = unsafe { core::mem::zeroed() };
    let mut peek: i32 = unsafe { core::mem::zeroed() };
    let mut peek_1: i32 = unsafe { core::mem::zeroed() };
    let mut peek_2: i32 = unsafe { core::mem::zeroed() };
    let mut start: i32 = unsafe { core::mem::zeroed() };
    let mut in_format_spec: i32 = unsafe { core::mem::zeroed() };
    let mut c: i32 = unsafe { core::mem::zeroed() };
    let mut quote: i32 = unsafe { core::mem::zeroed() };
    // Check if we are at the end of the string
    let mut i_1: i32 = unsafe { core::mem::zeroed() };
    let mut peek1: i32 = unsafe { core::mem::zeroed() };
    // If we start with a bracket, we defer to the normal mode as there is nothing for us to tokenize
    //
    // before it.
    let mut start_char: i32 = unsafe { core::mem::zeroed() };
    let mut unicode_escape: i32 = unsafe { core::mem::zeroed() };
    let mut end_quote_size: i32 = unsafe { core::mem::zeroed() };
    let mut p_end: *const i8 = unsafe { core::mem::zeroed() };
    let mut p_start: *const i8 = unsafe { core::mem::zeroed() };
    unsafe {
        p_start = core::ptr::null::<i8>();
        p_end = core::ptr::null::<i8>();
        end_quote_size = 0;
        unicode_escape = 0;
        (*tok).start = (*tok).cur as *const i8;
        (*tok).first_lineno = (*tok).lineno;
        (*tok).starting_col_offset = (*tok).col_offset;
        start_char = tok_nextc(tok);
        if start_char == 123 {
            peek1 = tok_nextc(tok);
            tok_backup(tok, peek1);
            if peek1 != 123 {
                (*current_tok).last_expr_start = (*tok).cur as *const i8;
            }
            tok_backup(tok, start_char);
            if peek1 != 123 {
                let __v27: i32 = (*current_tok).curly_bracket_expr_start_depth;
                (*current_tok).curly_bracket_expr_start_depth = __v27.wrapping_add(1);
                if (*current_tok).curly_bracket_expr_start_depth >= 3 {
                    return MAKE_TOKEN!(
                        _PyTokenizer_syntaxerror(
                            tok,
                            c"%c-string: expressions nested too deeply".as_ptr(),
                            TOK_GET_STRING_PREFIX!(tok)
                        ),
                        tok,
                        token,
                        p_start,
                        p_end
                    );
                } else {
                    (*TOK_GET_MODE!(tok)).kind = 0;
                    return tok_get_normal_mode(tok, current_tok, token);
                }
            }
        } else {
            tok_backup(tok, start_char);
        }
        i_1 = 0;
        '__join_52: {
            while i_1 < (*current_tok).quote_size {
                quote = tok_nextc(tok);
                if quote != ((*current_tok).quote as i32) {
                    break '__join_52;
                } else {
                    i_1 = i_1.wrapping_add(1);
                }
            }
            p_start = (*tok).start;
            p_end = (*tok).cur as *const i8;
            let __v20: i32 = (*tok).tok_mode_stack_index;
            (*tok).tok_mode_stack_index = __v20.wrapping_sub(1);
            return MAKE_TOKEN!(FTSTRING_END!(current_tok), tok, token, p_start, p_end);
        }
        tok_backup(tok, quote);
        (*tok).multi_line_start = (*tok).line_start;
        // TODO: This is a bit of a hack, but it works for now. We need to find a better way to handle
        //
        // this.
        '__join_45: {
            '__join_10: {
                '__join_16: {
                    while end_quote_size != (*current_tok).quote_size {
                        c = tok_nextc(tok);
                        if (*tok).done == 17 || (*tok).done == 22 {
                            return MAKE_TOKEN!(67i32, tok, token, p_start, p_end);
                        } else {
                            in_format_spec = ((*current_tok).in_format_spec != 0
                                && INSIDE_FSTRING_EXPR!(current_tok))
                                as i32;
                            if c == (slate_support::EOF as i32)
                                || (*current_tok).quote_size == 1 && c == 10
                            {
                                break '__join_45;
                            } else {
                                if c == ((*current_tok).quote as i32) {
                                    end_quote_size = end_quote_size.wrapping_add(1);
                                } else {
                                    end_quote_size = 0;
                                    if c == 123 {
                                        break '__join_10;
                                    } else {
                                        if c == 125 {
                                            break '__join_16;
                                        } else {
                                            if c == 92 {
                                                peek_2 = tok_nextc(tok);
                                                if peek_2 == 13 {
                                                    peek_2 = tok_nextc(tok);
                                                }
                                                // Special case when the backslash is right before a curly
                                                //
                                                // brace. We have to restore and return the control back
                                                //
                                                // to the loop for the next iteration.
                                                if peek_2 == 123 || peek_2 == 125 {
                                                    if (*current_tok).raw == 0 {
                                                        if _PyTokenizer_warn_invalid_escape_sequence(
                                                            tok, peek_2,
                                                        ) != 0
                                                        {
                                                            return MAKE_TOKEN!(
                                                                67i32, tok, token, p_start, p_end
                                                            );
                                                        }
                                                    }
                                                    tok_backup(tok, peek_2);
                                                } else {
                                                    if (*current_tok).raw == 0 {
                                                        if peek_2 == 78 {
                                                            // Handle named unicode escapes (\N{BULLET})
                                                            peek_2 = tok_nextc(tok);
                                                            if peek_2 == 123 {
                                                                unicode_escape = 1;
                                                            } else {
                                                                tok_backup(tok, peek_2);
                                                            }
                                                        }
                                                    }
                                                    // else {
                                                    //     skip the escaped character
                                                    // }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    i = 0;
                    while i < (*current_tok).quote_size {
                        tok_backup(tok, (*current_tok).quote as i32);
                        i = i.wrapping_add(1);
                    }
                    p_start = (*tok).start;
                    p_end = (*tok).cur as *const i8;
                    return MAKE_TOKEN!(FTSTRING_MIDDLE!(current_tok), tok, token, p_start, p_end);
                }
                if unicode_escape != 0 {
                    p_start = (*tok).start;
                    p_end = (*tok).cur as *const i8;
                    return MAKE_TOKEN!(FTSTRING_MIDDLE!(current_tok), tok, token, p_start, p_end);
                } else {
                    peek_1 = tok_nextc(tok);
                    // The tokenizer can only be in the format spec if we have already completed the expression
                    //
                    // scanning (indicated by the end of the expression being set) and we are not at the top level
                    //
                    // of the bracket stack (-1 is the top level). Since format specifiers can't legally use double
                    //
                    // brackets, we can bypass it here.
                    let cursor: i32 = (*current_tok).curly_bracket_depth;
                    if peek_1 == 125 && in_format_spec == 0 && cursor == 0 {
                        p_start = (*tok).start;
                        p_end = unsafe { (*tok).cur }.wrapping_offset(-(1 as isize)) as *const i8;
                    } else {
                        tok_backup(tok, peek_1);
                        tok_backup(tok, c);
                        (*TOK_GET_MODE!(tok)).kind = 0;
                        (*current_tok).in_format_spec = 0;
                        p_start = (*tok).start;
                        p_end = (*tok).cur as *const i8;
                    }
                    return MAKE_TOKEN!(FTSTRING_MIDDLE!(current_tok), tok, token, p_start, p_end);
                }
            }
            peek = tok_nextc(tok);
            if peek != 123 || in_format_spec != 0 {
                tok_backup(tok, peek);
                (*current_tok).last_expr_start = (*tok).cur as *const i8;
                tok_backup(tok, c);
                let __v4: i32 = (*current_tok).curly_bracket_expr_start_depth;
                (*current_tok).curly_bracket_expr_start_depth = __v4.wrapping_add(1);
                if (*current_tok).curly_bracket_expr_start_depth >= 3 {
                    return MAKE_TOKEN!(
                        _PyTokenizer_syntaxerror(
                            tok,
                            c"%c-string: expressions nested too deeply".as_ptr(),
                            TOK_GET_STRING_PREFIX!(tok)
                        ),
                        tok,
                        token,
                        p_start,
                        p_end
                    );
                } else {
                    (*TOK_GET_MODE!(tok)).kind = 0;
                    (*current_tok).in_format_spec = 0;
                    p_start = (*tok).start;
                    p_end = (*tok).cur as *const i8;
                }
            } else {
                p_start = (*tok).start;
                p_end = unsafe { (*tok).cur }.wrapping_offset(-(1 as isize)) as *const i8;
            }
            return MAKE_TOKEN!(FTSTRING_MIDDLE!(current_tok), tok, token, p_start, p_end);
        }
        if (*tok).decoding_erred != 0 {
            return MAKE_TOKEN!(67i32, tok, token, p_start, p_end);
        } else {
            // If we are in a format spec and we found a newline,
            //
            // it means that the format spec ends here and we should
            //
            // return to the regular mode.
            if in_format_spec != 0 && c == 10 {
                if (*current_tok).quote_size == 1 {
                    return MAKE_TOKEN!(_PyTokenizer_syntaxerror(tok, c"%c-string: newlines are not allowed in format specifiers for single quoted %c-strings".as_ptr(), TOK_GET_STRING_PREFIX!(tok), TOK_GET_STRING_PREFIX!(tok)), tok, token, p_start, p_end);
                } else {
                    tok_backup(tok, c);
                    (*TOK_GET_MODE!(tok)).kind = 0;
                    (*current_tok).in_format_spec = 0;
                    p_start = (*tok).start;
                    p_end = (*tok).cur as *const i8;
                    return MAKE_TOKEN!(FTSTRING_MIDDLE!(current_tok), tok, token, p_start, p_end);
                }
            } else {
                // shift the tok_state's location into
                //
                // the start of string, and report the error
                //
                // from the initial quote character
                (*tok).cur = (*current_tok).start as *mut i8;
                let __v15: *mut i8 = (*tok).cur;
                (*tok).cur = __v15.wrapping_offset(1 as isize);
                (*tok).line_start = (*current_tok).multi_line_start;
                start = (*tok).lineno;
                let the_current_tok: *mut _tokenizer_mode = TOK_GET_MODE!(tok);
                (*tok).lineno = (*the_current_tok).first_line;
                if (*current_tok).quote_size == 3 {
                    _PyTokenizer_syntaxerror(
                        tok,
                        c"unterminated triple-quoted %c-string literal (detected at line %d)"
                            .as_ptr(),
                        TOK_GET_STRING_PREFIX!(tok),
                        start,
                    );
                    if c != 10 {
                        (*tok).done = 23;
                    }
                    return MAKE_TOKEN!(67i32, tok, token, p_start, p_end);
                } else {
                    return MAKE_TOKEN!(
                        _PyTokenizer_syntaxerror(
                            tok,
                            c"unterminated %c-string literal (detected at line %d)".as_ptr(),
                            TOK_GET_STRING_PREFIX!(tok),
                            start
                        ),
                        tok,
                        token,
                        p_start,
                        p_end
                    );
                }
            }
        }
    }
}

fn tok_get(tok: *mut tok_state, token: *mut token) -> i32 {
    let current_tok: *mut _tokenizer_mode = TOK_GET_MODE!(tok);
    if ((unsafe { (*current_tok).kind }) as u32) == 0 {
        return tok_get_normal_mode(tok, current_tok, token);
    } else {
        return tok_get_fstring_mode(tok, current_tok, token);
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyTokenizer_Get(tok: *mut tok_state, token: *mut token) -> i32 {
    let mut result: i32 = tok_get(tok, token);
    if (unsafe { (*tok).decoding_erred }) != 0 {
        result = 67;
        unsafe {
            (*tok).done = 22;
        }
    }
    result
}
