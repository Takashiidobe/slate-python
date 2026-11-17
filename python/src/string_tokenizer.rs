//! The size of 'void *', as computed by sizeof.
//! Argument must be a char or an int in [-128, 127] or [0, 255].
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
//! End Of File
//! Max indentation level
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

macro_rules! PyBytes_AS_STRING {
    ($op:expr) => {
        PyBytes_AS_STRING(_PyObject_CAST!($op))
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

unsafe extern "C" {
    fn PyMem_Free(arg0: *mut ());
    fn _Py_Dealloc(arg0: *mut _object);
    fn PyBytes_AsString(arg0: *mut _object) -> *mut i8;
    fn _PyTokenizer_tok_new() -> *mut tok_state;
    fn _PyTokenizer_Free(arg0: *mut tok_state);
    fn _PyTokenizer_error_ret(arg0: *mut tok_state) -> *mut i8;
    fn _PyTokenizer_translate_newlines(
        arg0: *const i8,
        arg1: i32,
        arg2: i32,
        arg3: *mut tok_state,
    ) -> *mut i8;
    fn _PyTokenizer_translate_into_utf8(arg0: *const i8, arg1: *const i8) -> *mut _object;
    fn _PyTokenizer_check_bom(
        arg0: Option<unsafe extern "C-unwind" fn(*mut tok_state) -> i32>,
        arg1: Option<unsafe extern "C-unwind" fn(i32, *mut tok_state)>,
        arg2: Option<unsafe extern "C-unwind" fn(*mut tok_state, *const i8) -> i32>,
        arg3: *mut tok_state,
    ) -> i32;
    fn _PyTokenizer_check_coding_spec(
        arg0: *const i8,
        arg1: i64,
        arg2: *mut tok_state,
        arg3: Option<unsafe extern "C-unwind" fn(*mut tok_state, *const i8) -> i32>,
    ) -> i32;
    fn _PyTokenizer_ensure_utf8(arg0: *const i8, arg1: *mut tok_state, arg2: i32) -> i32;
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

#[repr(C, align(16))]
struct __SlateAlign16<T>(T);

unsafe impl<T> Sync for __SlateAlign16<T> {}

use slate_support::strchr;

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

#[unsafe(link_section = ".text.slate_distinct.string_tokenizer.tok_underflow_string")]
extern "C-unwind" fn tok_underflow_string(tok: *mut tok_state) -> i32 {
    let mut end: *mut i8 = unsafe {
        strchr(
            ((*tok).inp as *const i8) as *const core::ffi::c_char,
            10i32 as i32,
        ) as *mut i8
    };
    if !end.is_null() {
        let __v0: *mut i8 = end;
        end = __v0.wrapping_offset(1 as isize);
    } else {
        end = unsafe {
            strchr(
                ((*tok).inp as *const i8) as *const core::ffi::c_char,
                0i32 as i32,
            ) as *mut i8
        };
        if end == unsafe { (*tok).inp } {
            unsafe {
                (*tok).done = 11;
            }
            return 0;
        }
    }
    if unsafe { (*tok).start }.is_null() {
        unsafe {
            (*tok).buf = (*tok).cur;
        }
    }
    unsafe {
        (*tok).line_start = (*tok).cur as *const i8;
    }
    ADVANCE_LINENO!(tok);
    unsafe {
        (*tok).inp = end;
    }
    1
}

/// Fetch a byte from TOK, using the string buffer.
#[unsafe(link_section = ".text.slate_distinct.string_tokenizer.buf_getc")]
extern "C-unwind" fn buf_getc(__v0: *mut tok_state) -> i32 {
    let __v1: *mut i8 = unsafe { (*__v0).str };
    unsafe {
        (*__v0).str = __v1.wrapping_offset(1 as isize);
    }
    ((((unsafe { *__v1 }) as i32) & 255) as u8) as i32
}

/// Unfetch a byte from TOK, using the string buffer.
#[unsafe(link_section = ".text.slate_distinct.string_tokenizer.buf_ungetc")]
extern "C-unwind" fn buf_ungetc(c: i32, __v0: *mut tok_state) {
    let __v1: *mut i8 = unsafe { (*__v0).str };
    unsafe {
        (*__v0).str = __v1.wrapping_offset(-(1 as isize));
    }
    // tok->cur may point to read-only segment
}

/// Set the readline function for TOK to ENC. For the string-based
/// tokenizer, this means to just record the encoding.
#[unsafe(link_section = ".text.slate_distinct.string_tokenizer.buf_setreadl")]
extern "C-unwind" fn buf_setreadl(tok: *mut tok_state, enc: *const i8) -> i32 {
    unsafe {
        (*tok).enc = enc;
    }
    1
}

/// Decode a byte string STR for use as the buffer of TOK.
/// Look for encoding declarations inside STR, and record them
/// inside TOK.
fn decode_str(input: *const i8, single: i32, tok: *mut tok_state, preserve_crlf: i32) -> *mut i8 {
    let mut utf8: *mut _object = core::ptr::null_mut::<_object>();
    let mut str: *mut i8 = unsafe { core::mem::zeroed() };
    let mut s: *const i8 = unsafe { core::mem::zeroed() };
    let mut newl: __SlateAlign16<[*const i8; 2]> =
        __SlateAlign16([core::ptr::null::<i8>(), core::ptr::null::<i8>()]);
    let mut lineno: i32 = 0;
    let __v0: *mut i8 =
        unsafe { _PyTokenizer_translate_newlines(input, single, preserve_crlf, tok) };
    str = __v0;
    unsafe {
        (*tok).input = __v0;
    }
    if str.is_null() {
        return core::ptr::null_mut::<i8>();
    }
    unsafe {
        (*tok).enc = core::ptr::null::<i8>();
    }
    unsafe {
        (*tok).str = str;
    }
    if (unsafe {
        _PyTokenizer_check_bom(Some(buf_getc), Some(buf_ungetc), Some(buf_setreadl), tok)
    }) == 0
    {
        return unsafe { _PyTokenizer_error_ret(tok) };
    }
    str = unsafe { (*tok).str }; // string after BOM if any
    if !unsafe { (*tok).enc }.is_null() {
        utf8 = unsafe { _PyTokenizer_translate_into_utf8(str as *const i8, (*tok).enc) };
        if utf8.is_null() {
            return unsafe { _PyTokenizer_error_ret(tok) };
        }
        str = unsafe { PyBytes_AsString(utf8) };
    }
    s = str as *const i8;
    loop {
        if ((unsafe { *s }) as i32) == 0 {
            break;
        } else {
            if ((unsafe { *s }) as i32) == 10 {
                newl.0[lineno as usize] = s;
                lineno = lineno.wrapping_add(1);
                if lineno == 2 {
                    break;
                }
            }
        }
        let __v4: *const i8 = s;
        s = __v4.wrapping_offset(1 as isize);
    }
    unsafe {
        (*tok).enc = core::ptr::null::<i8>();
    }
    // need to check line 1 and 2 separately since check_coding_spec
    // assumes a single line as input
    if !newl.0[0].is_null() {
        unsafe {
            (*tok).lineno = 1;
        }
        if (unsafe {
            _PyTokenizer_check_coding_spec(
                str as *const i8,
                newl.0[0].offset_from(str as *const i8) as i64,
                tok,
                Some(buf_setreadl),
            )
        }) == 0
        {
            return core::ptr::null_mut::<i8>();
        }
        if unsafe { (*tok).enc }.is_null()
            && ((unsafe { (*tok).decoding_state }) as u32) != 2
            && !newl.0[1].is_null()
        {
            unsafe {
                (*tok).lineno = 2;
            }
            if (unsafe {
                _PyTokenizer_check_coding_spec(
                    newl.0[0].wrapping_offset(1 as isize),
                    newl.0[1].offset_from(newl.0[0] as *const i8) as i64,
                    tok,
                    Some(buf_setreadl),
                )
            }) == 0
            {
                return core::ptr::null_mut::<i8>();
            }
        }
    }
    unsafe {
        (*tok).lineno = 0;
    }
    if !unsafe { (*tok).enc }.is_null() {
        utf8 = unsafe { _PyTokenizer_translate_into_utf8(str as *const i8, (*tok).enc) };
        if utf8.is_null() {
            return unsafe { _PyTokenizer_error_ret(tok) };
        }
        str = PyBytes_AS_STRING!(utf8);
    } else {
        if (unsafe { _PyTokenizer_ensure_utf8(str as *const i8, tok, 1i32) }) == 0 {
            return unsafe { _PyTokenizer_error_ret(tok) };
        }
    }
    if !utf8.is_null() {
        let translated: *mut i8 = unsafe {
            _PyTokenizer_translate_newlines(str as *const i8, single, preserve_crlf, tok)
        };
        if translated.is_null() {
            Py_DECREF!(utf8);
            return unsafe { _PyTokenizer_error_ret(tok) };
        }
        unsafe { PyMem_Free((*tok).input as *mut ()) };
        unsafe {
            (*tok).input = translated;
        }
        str = translated;
        Py_CLEAR__2!(utf8);
    }
    unsafe {
        (*tok).str = str;
    }
    unsafe {
        (*tok).decoding_buffer = utf8;
    }
    // CAUTION
    str
}

/// Set up tokenizer for string
#[unsafe(no_mangle)]
extern "C-unwind" fn _PyTokenizer_FromString(
    str: *const i8,
    exec_input: i32,
    preserve_crlf: i32,
) -> *mut tok_state {
    let tok: *mut tok_state = unsafe { _PyTokenizer_tok_new() };
    if tok.is_null() {
        return core::ptr::null_mut::<tok_state>();
    }
    let decoded: *mut i8 = decode_str(str, exec_input, tok, preserve_crlf);
    if decoded.is_null() {
        unsafe { _PyTokenizer_Free(tok) };
        return core::ptr::null_mut::<tok_state>();
    }
    let __v0: *mut i8 = decoded;
    unsafe {
        (*tok).inp = __v0;
    }
    let __v1: *mut i8 = __v0;
    unsafe {
        (*tok).cur = __v1;
    }
    unsafe {
        (*tok).buf = __v1;
    }
    unsafe {
        (*tok).end = decoded as *const i8;
    }
    unsafe {
        (*tok).underflow = Some(tok_underflow_string);
    }
    tok
}
