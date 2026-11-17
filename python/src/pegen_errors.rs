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
//! No error
//! _PyErr_ProgramDecodedTextObject()
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
// Interrupted
// Bad token
// Ran out of memory
// Parsing complete
// Inconsistent mixing of tabs and spaces
// Too many indentation levels
// No matching outer block for dedent
// Unexpected characters after a line continuation
// Column offset overflow
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
macro_rules! Py_DECREF {
    ($op:expr) => {
        Py_DECREF(_PyObject_CAST!($op));
    };
}

macro_rules! Py_SAFE_DOWNCAST {
    ($VALUE:expr, $NARROW:ty) => {
        _Py_STATIC_CAST!($NARROW, $VALUE)
    };
}

macro_rules! Py_XDECREF {
    ($op:expr) => {
        Py_XDECREF(_PyObject_CAST!($op));
    };
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

macro_rules! _Py_STATIC_CAST {
    ($type:ty, $expr:expr) => {
        ($expr) as $type
    };
}

unsafe extern "C" {
    static mut PyExc_KeyboardInterrupt: *mut _object;
    static mut PyExc_MemoryError: *mut _object;
    static mut PyExc_OverflowError: *mut _object;
    static mut PyExc_SyntaxError: *mut _object;
    static mut PyExc_IndentationError: *mut _object;
    static mut PyExc_TabError: *mut _object;
    static mut PyExc_UnicodeError: *mut _object;
    static mut PyExc_ValueError: *mut _object;
    fn PyObject_Str(arg0: *mut _object) -> *mut _object;
    fn Py_GetConstant(arg0: u32) -> *mut _object;
    fn _Py_Dealloc(arg0: *mut _object);
    fn PyUnicode_FromFormatV(arg0: *const i8, arg1: core::ffi::VaList<'static>) -> *mut _object;
    fn PyUnicode_DecodeUTF8(arg0: *const i8, arg1: i64, arg2: *const i8) -> *mut _object;
    fn PyErr_SetNone(arg0: *mut _object);
    fn PyErr_SetObject(arg0: *mut _object, arg1: *mut _object);
    fn PyErr_SetString(arg0: *mut _object, arg1: *const i8);
    fn PyErr_Occurred() -> *mut _object;
    fn PyErr_Clear();
    fn PyErr_Fetch(arg0: *mut *mut _object, arg1: *mut *mut _object, arg2: *mut *mut _object);
    fn PyErr_Restore(arg0: *mut _object, arg1: *mut _object, arg2: *mut _object);
    fn PyErr_ExceptionMatches(arg0: *mut _object) -> i32;
    fn PyErr_NoMemory() -> *mut _object;
    fn PyTuple_Pack(arg0: i64, ...) -> *mut _object;
    fn Py_BuildValue(arg0: *const i8, ...) -> *mut _object;
    fn _PyErr_ProgramDecodedTextObject(
        arg0: *mut _object,
        arg1: i32,
        arg2: *const i8,
    ) -> *mut _object;
    fn _PyToken_Free(arg0: *mut token);
    fn _PyToken_Init(arg0: *mut token);
    fn _PyTokenizer_Get(arg0: *mut tok_state, arg1: *mut token) -> i32;
    fn _PyPegen_fill_token(arg0: *mut __SlateRecord_pegen_h_94) -> i32;
    fn _PyPegen_byte_offset_to_character_offset(arg0: *mut _object, arg1: i64) -> i64;
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
struct _arena {}

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
struct __SlateRecord_pegen_h_61_2 {
    lineno: i32,
    comment: *mut i8,
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

use slate_support::strchr;
use slate_support::strlen;

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
    _PyPegen_raise_error_known_location(
        p,
        errtype,
        lineno,
        _col_offset,
        end_lineno,
        _end_col_offset,
        errmsg,
        va.clone(),
    );
    core::ptr::null_mut::<()>()
}

// _Py_ID()
// TOKENIZER ERRORS
fn raise_unclosed_parentheses_error(p: *mut __SlateRecord_pegen_h_94) {
    let error_lineno: i32 = unsafe {
        *((*(*p).tok).parenlinenostack.as_mut_ptr() as *mut i32)
            .wrapping_offset(unsafe { (*(*p).tok).level }.wrapping_sub(1) as isize)
    };
    let error_col: i32 = unsafe {
        *((*(*p).tok).parencolstack.as_mut_ptr() as *mut i32)
            .wrapping_offset(unsafe { (*(*p).tok).level }.wrapping_sub(1) as isize)
    };
    unsafe {
        RAISE_ERROR_KNOWN_LOCATION(
            p,
            PyExc_SyntaxError,
            error_lineno as i64,
            error_col as i64,
            error_lineno as i64,
            1i32.wrapping_neg() as i64,
            c"'%c' was never closed".as_ptr(),
            *((*(*p).tok).parenstack.as_mut_ptr() as *mut i8)
                .wrapping_offset(unsafe { (*(*p).tok).level }.wrapping_sub(1) as isize)
                as i32,
        )
    };
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _Pypegen_tokenizer_error(p: *mut __SlateRecord_pegen_h_94) -> i32 {
    if !unsafe { PyErr_Occurred() }.is_null() {
        return 1i32.wrapping_neg();
    }
    let mut msg: *const i8 = core::ptr::null::<i8>();
    let mut errtype: *mut _object = unsafe { PyExc_SyntaxError };
    let mut col_offset: i64 = 1i32.wrapping_neg() as i64;
    unsafe {
        (*p).error_indicator = 1;
    }
    match unsafe { (*(*p).tok).done } {
        13 => {
            msg = c"invalid token".as_ptr();
        }
        11 => {
            if (unsafe { (*(*p).tok).level }) != 0 {
                raise_unclosed_parentheses_error(p);
            } else {
                unsafe {
                    _PyPegen_raise_error(
                        p,
                        PyExc_SyntaxError,
                        0i32,
                        c"unexpected EOF while parsing".as_ptr(),
                    )
                };
            }
            return 1i32.wrapping_neg();
        }
        21 => {
            unsafe {
                _PyPegen_raise_error(
                    p,
                    PyExc_IndentationError,
                    0i32,
                    c"unindent does not match any outer indentation level".as_ptr(),
                )
            };
            return 1i32.wrapping_neg();
        }
        12 => {
            if unsafe { PyErr_Occurred() }.is_null() {
                unsafe { PyErr_SetNone(PyExc_KeyboardInterrupt) };
            }
            return 1i32.wrapping_neg();
        }
        15 => {
            unsafe { PyErr_NoMemory() };
            return 1i32.wrapping_neg();
        }
        18 => {
            errtype = unsafe { PyExc_TabError };
            msg = c"inconsistent use of tabs and spaces in indentation".as_ptr();
        }
        20 => {
            errtype = unsafe { PyExc_IndentationError };
            msg = c"too many levels of indentation".as_ptr();
        }
        25 => {
            col_offset =
                ((unsafe { unsafe { (*(*p).tok).cur }.offset_from((*(*p).tok).buf as *mut i8) })
                    as i64)
                    .wrapping_sub(1);
            msg = c"unexpected character after line continuation character".as_ptr();
        }
        29 => {
            unsafe {
                PyErr_SetString(
                    PyExc_OverflowError,
                    c"Parser column offset overflow - source line is too big".as_ptr(),
                )
            };
            return 1i32.wrapping_neg();
        }
        _ => {
            msg = c"unknown parsing error".as_ptr();
        }
    }
    unsafe {
        RAISE_ERROR_KNOWN_LOCATION(
            p,
            errtype,
            (*(*p).tok).lineno as i64,
            if col_offset >= 0 { col_offset } else { 0i64 },
            (*(*p).tok).lineno as i64,
            1i32.wrapping_neg() as i64,
            msg,
        )
    };
    1i32.wrapping_neg()
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _Pypegen_raise_decode_error(p: *mut __SlateRecord_pegen_h_94) -> i32 {
    let mut errtype: *const i8 = core::ptr::null::<i8>();
    if (unsafe { PyErr_ExceptionMatches(PyExc_UnicodeError) }) != 0 {
        errtype = c"unicode error".as_ptr();
    } else {
        if (unsafe { PyErr_ExceptionMatches(PyExc_ValueError) }) != 0 {
            errtype = c"value error".as_ptr();
        }
    }
    if !errtype.is_null() {
        let mut r#type: *mut _object = unsafe { core::mem::zeroed() };
        let mut value: *mut _object = unsafe { core::mem::zeroed() };
        let mut tback: *mut _object = unsafe { core::mem::zeroed() };
        unsafe {
            PyErr_Fetch(
                core::ptr::addr_of_mut!(r#type),
                core::ptr::addr_of_mut!(value),
                core::ptr::addr_of_mut!(tback),
            )
        };
        let errstr: *mut _object = unsafe { PyObject_Str(value) };
        if !errstr.is_null() {
            unsafe {
                _PyPegen_raise_error(
                    p,
                    PyExc_SyntaxError,
                    0i32,
                    c"(%s) %U".as_ptr(),
                    errtype,
                    errstr,
                )
            };
            Py_DECREF!(errstr);
        } else {
            unsafe { PyErr_Clear() };
            unsafe {
                _PyPegen_raise_error(
                    p,
                    PyExc_SyntaxError,
                    0i32,
                    c"(%s) unknown error".as_ptr(),
                    errtype,
                )
            };
        }
        Py_XDECREF!(r#type);
        Py_XDECREF!(value);
        Py_XDECREF!(tback);
    }
    1i32.wrapping_neg()
}

fn _PyPegen_tokenize_full_source_to_check_for_errors(p: *mut __SlateRecord_pegen_h_94) -> i32 {
    let mut new_token: core::mem::MaybeUninit<token> = core::mem::MaybeUninit::uninit();
    let __slate_slot_1: *mut token = core::ptr::addr_of_mut!(new_token) as *mut token;
    let mut ret: i32 = unsafe { core::mem::zeroed() };
    let mut current_err_line: i64 = unsafe { core::mem::zeroed() };
    let mut traceback: *mut _object = unsafe { core::mem::zeroed() };
    let mut value: *mut _object = unsafe { core::mem::zeroed() };
    let mut r#type: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        // Tokenize the whole input to see if there are any tokenization
        //
        // errors such as mismatching parentheses. These will get priority
        //
        // over generic syntax errors only if the line number of the error is
        //
        // before the one that we had for the generic error.
        // We don't want to tokenize to the end for interactive input
        if !unsafe { (*(*p).tok).prompt }.is_null() {
            return 0;
        } else {
            PyErr_Fetch(
                core::ptr::addr_of_mut!(r#type),
                core::ptr::addr_of_mut!(value),
                core::ptr::addr_of_mut!(traceback),
            );
            let current_token: *mut __SlateRecord_pegen_h_46 =
                if !unsafe { (*p).known_err_token }.is_null() {
                    (*p).known_err_token
                } else {
                    *unsafe { (*p).tokens }
                        .wrapping_offset(unsafe { (*p).fill }.wrapping_sub(1) as isize)
                };
            current_err_line = (*current_token).lineno as i64;
            ret = 0;
            _PyToken_Init(core::ptr::addr_of_mut!(*__slate_slot_1));
            '__join_3: {
                loop {
                    let __t0: i32 =
                        _PyTokenizer_Get((*p).tok, core::ptr::addr_of_mut!(*__slate_slot_1));
                    if __t0 == 67 {
                        break;
                    } else {
                        if __t0 == 0 {
                            break '__join_3;
                        }
                    }
                }
                if !PyErr_Occurred().is_null() {
                    ret = 1i32.wrapping_neg();
                } else {
                    if (*(*p).tok).level != 0 {
                        let error_lineno: i32 = *((*(*p).tok).parenlinenostack.as_mut_ptr()
                            as *mut i32)
                            .wrapping_offset(unsafe { (*(*p).tok).level }.wrapping_sub(1) as isize);
                        if current_err_line > (error_lineno as i64) {
                            raise_unclosed_parentheses_error(p);
                            ret = 1i32.wrapping_neg();
                        }
                    }
                }
            }
            _PyToken_Free(core::ptr::addr_of_mut!(*__slate_slot_1));
            // If we're in an f-string, we want the syntax error in the expression part
            //
            // to propagate, so that tokenizer errors (like expecting '}') that happen afterwards
            //
            // do not swallow it.
            if !PyErr_Occurred().is_null() && (*(*p).tok).tok_mode_stack_index <= 0 {
                Py_XDECREF!(value);
                Py_XDECREF!(r#type);
                Py_XDECREF!(traceback);
            } else {
                PyErr_Restore(r#type, value, traceback);
            }
            return ret;
        }
    }
}

// PARSER ERRORS
#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn _PyPegen_raise_error(
    p: *mut __SlateRecord_pegen_h_94,
    errtype: *mut _object,
    use_mark: i32,
    errmsg: *const i8,
    mut __va_args: ...
) -> *mut () {
    // Bail out if we already have an error set.
    let __v0: bool =
        (unsafe { (*p).error_indicator }) != 0 && !unsafe { PyErr_Occurred() }.is_null();
    if __v0 {
        return core::ptr::null_mut::<()>();
    }
    if (unsafe { (*p).fill }) == 0 {
        let mut va: core::ffi::VaList<'static> = unsafe { core::mem::zeroed() };
        va = unsafe {
            core::mem::transmute::<core::ffi::VaList<'_>, core::ffi::VaList<'static>>(
                __va_args.clone(),
            )
        };
        _PyPegen_raise_error_known_location(
            p,
            errtype,
            0i64,
            0i64,
            0i64,
            1i32.wrapping_neg() as i64,
            errmsg,
            va.clone(),
        );
        return core::ptr::null_mut::<()>();
    }
    let __v1: bool = use_mark != 0
        && (unsafe { (*p).mark }) == unsafe { (*p).fill }
        && (unsafe { _PyPegen_fill_token(p) }) < 0;
    if __v1 {
        unsafe {
            (*p).error_indicator = 1;
        }
        return core::ptr::null_mut::<()>();
    }
    let t: *mut __SlateRecord_pegen_h_46 = if !unsafe { (*p).known_err_token }.is_null() {
        unsafe { (*p).known_err_token }
    } else {
        unsafe {
            *unsafe { (*p).tokens }.wrapping_offset(
                (if use_mark != 0 {
                    (*p).mark
                } else {
                    unsafe { (*p).fill }.wrapping_sub(1)
                }) as isize,
            )
        }
    };
    let mut col_offset: i64 = 0;
    let mut end_col_offset: i64 = 1i32.wrapping_neg() as i64;
    if (unsafe { (*t).col_offset }) == 1i32.wrapping_neg() {
        if (unsafe { (*(*p).tok).cur }) == unsafe { (*(*p).tok).buf } {
            col_offset = 0;
        } else {
            let start: *const i8 = if !unsafe { (*(*p).tok).buf }.is_null() {
                unsafe { (*(*p).tok).line_start }
            } else {
                (unsafe { (*(*p).tok).buf }) as *const i8
            };
            col_offset = Py_SAFE_DOWNCAST!(
                (unsafe { unsafe { (*(*p).tok).cur }.offset_from(start as *mut i8) }) as i64,
                i32
            ) as i64;
        }
    } else {
        col_offset = unsafe { (*t).col_offset }.wrapping_add(1) as i64;
    }
    if (unsafe { (*t).end_col_offset }) != 1i32.wrapping_neg() {
        end_col_offset = unsafe { (*t).end_col_offset }.wrapping_add(1) as i64;
    }
    let mut va: core::ffi::VaList<'static> = unsafe { core::mem::zeroed() };
    va = unsafe {
        core::mem::transmute::<core::ffi::VaList<'_>, core::ffi::VaList<'static>>(__va_args.clone())
    };
    _PyPegen_raise_error_known_location(
        p,
        errtype,
        (unsafe { (*t).lineno }) as i64,
        col_offset,
        (unsafe { (*t).end_lineno }) as i64,
        end_col_offset,
        errmsg,
        va.clone(),
    );
    core::ptr::null_mut::<()>()
}

fn get_error_line_from_tokenizer_buffers(
    p: *mut __SlateRecord_pegen_h_94,
    lineno: i64,
) -> *mut _object {
    // If the file descriptor is interactive, the source lines of the current
    // (multi-line) statement are stored in p->tok->interactive_src_start.
    // If not, we're parsing from a string, which means that the whole source
    // is stored in p->tok->str.
    let mut cur_line: *mut i8 = if (unsafe { (*(*p).tok).fp_interactive }) != 0 {
        unsafe { (*(*p).tok).interactive_src_start }
    } else {
        unsafe { (*(*p).tok).str }
    };
    if cur_line.is_null() {
        // We can reach this point if the tokenizer buffers for interactive source have not been
        //
        // initialized because we failed to decode the original source with the given locale.
        return unsafe { Py_GetConstant(7u32) };
    }
    let relative_lineno: i64 = if (unsafe { (*p).starting_lineno }) != 0 {
        lineno
            .wrapping_sub((unsafe { (*p).starting_lineno }) as i64)
            .wrapping_add(1)
    } else {
        lineno
    };
    let mut buf_end: *const i8 = (if (unsafe { (*(*p).tok).fp_interactive }) != 0 {
        unsafe { (*(*p).tok).interactive_src_end }
    } else {
        unsafe { (*(*p).tok).inp }
    }) as *const i8;
    if buf_end < (cur_line as *const i8) {
        buf_end = cur_line.wrapping_offset(
            (unsafe { strlen((cur_line as *const i8) as *const core::ffi::c_char) as u64 })
                as isize,
        ) as *const i8;
    }
    let mut i: i32 = 0;
    while (i as i64) < relative_lineno.wrapping_sub(1) {
        let new_line: *mut i8 = unsafe {
            strchr(
                (cur_line as *const i8) as *const core::ffi::c_char,
                10i32 as i32,
            ) as *mut i8
        };
        // The assert is here for debug builds but the conditional that
        //
        // follows is there so in release builds we do not crash at the cost
        //
        // to report a potentially wrong line.
        if new_line.is_null() || new_line.wrapping_offset(1 as isize) > (buf_end as *mut i8) {
            break;
        }
        cur_line = new_line.wrapping_offset(1 as isize);
        i = i.wrapping_add(1);
    }
    let mut next_newline: *mut i8 = unsafe { core::mem::zeroed() };
    let __v3: *mut i8 = unsafe {
        strchr(
            (cur_line as *const i8) as *const core::ffi::c_char,
            10i32 as i32,
        ) as *mut i8
    };
    next_newline = __v3;
    if __v3.is_null() {
        // This is the last line
        next_newline = cur_line.wrapping_offset(
            (unsafe { strlen((cur_line as *const i8) as *const core::ffi::c_char) as u64 })
                as isize,
        );
    }
    unsafe {
        PyUnicode_DecodeUTF8(
            cur_line as *const i8,
            next_newline.offset_from(cur_line as *mut i8) as i64,
            c"replace".as_ptr(),
        )
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyPegen_raise_error_known_location(
    p: *mut __SlateRecord_pegen_h_94,
    errtype: *mut _object,
    lineno: i64,
    col_offset: i64,
    mut end_lineno: i64,
    mut end_col_offset: i64,
    errmsg: *const i8,
    mut va: core::ffi::VaList<'static>,
) -> *mut () {
    let mut end_col_number: i64 = unsafe { core::mem::zeroed() };
    let mut col_number: i64 = unsafe { core::mem::zeroed() };
    let mut tmp: *mut _object = unsafe { core::mem::zeroed() };
    let mut error_line: *mut _object = unsafe { core::mem::zeroed() };
    let mut errstr: *mut _object = unsafe { core::mem::zeroed() };
    let mut value: *mut _object = unsafe { core::mem::zeroed() };
    // Bail out if we already have an error set.
    let mut __v7: bool = unsafe { core::mem::zeroed() };
    unsafe {
        if (*p).error_indicator != 0 {
            __v7 = !PyErr_Occurred().is_null();
        } else {
            __v7 = false;
        }
        if __v7 {
            return core::ptr::null_mut::<()>();
        } else {
            value = core::ptr::null_mut::<_object>();
            errstr = core::ptr::null_mut::<_object>();
            error_line = core::ptr::null_mut::<_object>();
            tmp = core::ptr::null_mut::<_object>();
            (*p).error_indicator = 1;
            if end_lineno == (5i32.wrapping_neg() as i64) {
                end_lineno = (*(*p).tok).lineno as i64;
            }
            if end_col_offset == (5i32.wrapping_neg() as i64) {
                end_col_offset = unsafe { (*(*p).tok).cur }
                    .offset_from((*(*p).tok).line_start as *mut i8)
                    as i64;
            }
            '__join_0: {
                errstr = PyUnicode_FromFormatV(errmsg, va.clone());
                if !errstr.is_null() {
                    if (*(*p).tok).fp_interactive != 0
                        && !unsafe { (*(*p).tok).interactive_src_start }.is_null()
                    {
                        error_line = get_error_line_from_tokenizer_buffers(p, lineno);
                    } else {
                        if (*p).start_rule == 257 {
                            error_line = _PyErr_ProgramDecodedTextObject(
                                (*(*p).tok).filename,
                                lineno as i32,
                                (*(*p).tok).encoding as *const i8,
                            );
                        }
                    }
                    if error_line.is_null() {
                        // PyErr_ProgramTextObject was not called or returned NULL. If it was not called,
                        // then we need to find the error line from some other source, because
                        // p->start_rule != Py_file_input. If it returned NULL, then it either unexpectedly
                        // failed or we're parsing from a string or the REPL. There's a third edge case where
                        // we're actually parsing from a file, which has an E_EOF SyntaxError and in that case
                        // `PyErr_ProgramTextObject` fails because lineno points to last_file_line + 1, which
                        // does not physically exist
                        if ((*(*p).tok).lineno as i64) <= lineno
                            && (*(*p).tok).inp > (*(*p).tok).buf
                        {
                            let size: i64 = unsafe { (*(*p).tok).inp }
                                .offset_from((*(*p).tok).line_start as *mut i8)
                                as i64;
                            error_line = PyUnicode_DecodeUTF8(
                                (*(*p).tok).line_start,
                                size,
                                c"replace".as_ptr(),
                            );
                        } else {
                            if unsafe { (*(*p).tok).fp }.is_null()
                                || (*(*p).tok).fp == slate_support::stdin()
                            {
                                error_line = get_error_line_from_tokenizer_buffers(p, lineno);
                            } else {
                                error_line = Py_GetConstant(7u32);
                            }
                        }
                        if error_line.is_null() {
                            break '__join_0;
                        }
                    }
                    col_number = col_offset;
                    end_col_number = end_col_offset;
                    col_number = _PyPegen_byte_offset_to_character_offset(error_line, col_offset);
                    if col_number >= 0 {
                        if end_col_offset > 0 {
                            end_col_number = _PyPegen_byte_offset_to_character_offset(
                                error_line,
                                end_col_offset,
                            );
                            if end_col_number < 0 {
                                break '__join_0;
                            }
                        }
                        tmp = Py_BuildValue(
                            c"(OnnNnn)".as_ptr(),
                            (*(*p).tok).filename,
                            lineno,
                            col_number,
                            error_line,
                            end_lineno,
                            end_col_number,
                        );
                        if !tmp.is_null() {
                            value = PyTuple_Pack(2i64, errstr, tmp);
                            Py_DECREF!(tmp);
                            if !value.is_null() {
                                PyErr_SetObject(errtype, value);
                                Py_DECREF!(errstr);
                                Py_DECREF!(value);
                                return core::ptr::null_mut::<()>();
                            }
                        }
                    }
                }
            }
            Py_XDECREF!(errstr);
            Py_XDECREF!(error_line);
            return core::ptr::null_mut::<()>();
        }
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _Pypegen_set_syntax_error(
    p: *mut __SlateRecord_pegen_h_94,
    last_token: *mut __SlateRecord_pegen_h_46,
) {
    // Existing syntax error
    if !unsafe { PyErr_Occurred() }.is_null() {
        // Prioritize tokenizer errors to custom syntax errors raised
        //
        // on the second phase only if the errors come from the parser.
        let is_tok_ok: i32 =
            ((unsafe { (*(*p).tok).done }) == 16 || (unsafe { (*(*p).tok).done }) == 10) as i32;
        let __v0: bool =
            is_tok_ok != 0 && (unsafe { PyErr_ExceptionMatches(PyExc_SyntaxError) }) != 0;
        if __v0 {
            _PyPegen_tokenize_full_source_to_check_for_errors(p);
        }
        // Propagate the existing syntax error.
        return;
    }
    // Initialization error
    if (unsafe { (*p).fill }) == 0 {
        unsafe {
            _PyPegen_raise_error(
                p,
                PyExc_SyntaxError,
                0i32,
                c"error at start before reading any input".as_ptr(),
            )
        };
    }
    // Parser encountered EOF (End of File) unexpectedtly
    if (unsafe { (*last_token).r#type }) == 67 && (unsafe { (*(*p).tok).done }) == 11 {
        if (unsafe { (*(*p).tok).level }) != 0 {
            raise_unclosed_parentheses_error(p);
        } else {
            unsafe {
                _PyPegen_raise_error(
                    p,
                    PyExc_SyntaxError,
                    0i32,
                    c"unexpected EOF while parsing".as_ptr(),
                )
            };
        }
        return;
    }
    // Indentation error in the tokenizer
    if (unsafe { (*last_token).r#type }) == 5 || (unsafe { (*last_token).r#type }) == 6 {
        unsafe {
            _PyPegen_raise_error(
                p,
                PyExc_IndentationError,
                0i32,
                (if (*last_token).r#type == 5 {
                    c"unexpected indent".as_ptr() as *mut i8
                } else {
                    c"unexpected unindent".as_ptr() as *mut i8
                }) as *const i8,
            )
        };
        return;
    }
    // Unknown error (generic case)
    // Use the last token we found on the first pass to avoid reporting
    //
    // incorrect locations for generic syntax errors just because we reached
    //
    // further away when trying to find specific syntax errors in the second
    //
    // pass.
    unsafe {
        RAISE_ERROR_KNOWN_LOCATION(
            p,
            PyExc_SyntaxError,
            (*last_token).lineno as i64,
            (*last_token).col_offset as i64,
            (*last_token).end_lineno as i64,
            (*last_token).end_col_offset as i64,
            c"invalid syntax".as_ptr(),
        )
    };
    // _PyPegen_tokenize_full_source_to_check_for_errors will override the existing
    //
    // generic SyntaxError we just raised if errors are found.
    _PyPegen_tokenize_full_source_to_check_for_errors(p);
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _Pypegen_stack_overflow(p: *mut __SlateRecord_pegen_h_94) {
    unsafe {
        (*p).error_indicator = 1;
    }
    unsafe {
        PyErr_SetString(
            PyExc_MemoryError,
            c"Parser stack overflowed - Python source too complex to parse".as_ptr(),
        )
    };
}
