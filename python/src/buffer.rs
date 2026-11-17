//! The size of 'void *', as computed by sizeof.
//! Macro to use the more powerful/dangerous C-style cast even in C++.
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
//! Remote debugger support
//! PY_TIMEOUT_T is the integral type used to specify timeouts when waiting
//!
//! on a lock (see PyThread_acquire_lock_timed() below).
//! Ran out of memory
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
/// Maximum value between x and y
macro_rules! Py_MAX {
    ($x:expr, $y:expr) => {
        if ($x) > $y { $x } else { $y }
    };
}

unsafe extern "C" {
    fn index(arg0: *const i8, arg1: i32) -> *mut i8;
    fn PyMem_Realloc(arg0: *mut (), arg1: u64) -> *mut ();
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

// Cast argument to PyObject* type.
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

/// Traverse and remember all f-string buffers, in order to be able to restore
/// them after reallocating tok->buf
#[unsafe(no_mangle)]
extern "C-unwind" fn _PyLexer_remember_fstring_buffers(tok: *mut tok_state) {
    let mut index: i32 = unsafe { (*tok).tok_mode_stack_index };
    while index >= 0 {
        let mode: *mut _tokenizer_mode =
            unsafe { (*tok).tok_mode_stack.as_mut_ptr() as *mut _tokenizer_mode }
                .wrapping_offset(index as isize);
        unsafe {
            (*mode).start_offset = if unsafe { (*mode).start }.is_null() {
                1i32.wrapping_neg() as i64
            } else {
                unsafe { (*mode).start }.offset_from((*tok).buf as *const i8) as i64
            };
        }
        unsafe {
            (*mode).multi_line_start_offset = if unsafe { (*mode).multi_line_start }.is_null() {
                1i32.wrapping_neg() as i64
            } else {
                unsafe { (*mode).multi_line_start }.offset_from((*tok).buf as *const i8) as i64
            };
        }
        unsafe {
            (*mode).last_expr_start_offset = if unsafe { (*mode).last_expr_start }.is_null() {
                1i32.wrapping_neg() as i64
            } else {
                unsafe { (*mode).last_expr_start }.offset_from((*tok).buf as *const i8) as i64
            };
        }
        index = index.wrapping_sub(1);
    }
}

/// Traverse and restore all f-string buffers after reallocating tok->buf
#[unsafe(no_mangle)]
extern "C-unwind" fn _PyLexer_restore_fstring_buffers(tok: *mut tok_state) {
    let mut index: i32 = unsafe { (*tok).tok_mode_stack_index };
    while index >= 0 {
        let mode: *mut _tokenizer_mode =
            unsafe { (*tok).tok_mode_stack.as_mut_ptr() as *mut _tokenizer_mode }
                .wrapping_offset(index as isize);
        unsafe {
            (*mode).start = (if (*mode).start_offset < 0 {
                core::ptr::null_mut::<i8>()
            } else {
                unsafe { (*tok).buf }.wrapping_offset((*mode).start_offset as isize)
            }) as *const i8;
        }
        unsafe {
            (*mode).multi_line_start = (if (*mode).multi_line_start_offset < 0 {
                core::ptr::null_mut::<i8>()
            } else {
                unsafe { (*tok).buf }.wrapping_offset((*mode).multi_line_start_offset as isize)
            }) as *const i8;
        }
        unsafe {
            (*mode).last_expr_start = (if (*mode).last_expr_start_offset < 0 {
                core::ptr::null_mut::<i8>()
            } else {
                unsafe { (*tok).buf }.wrapping_offset((*mode).last_expr_start_offset as isize)
            }) as *const i8;
        }
        index = index.wrapping_sub(1);
    }
}

/// Read a line of text from TOK into S, using the stream in TOK.
/// Return NULL on failure, else S.
///
/// On entry, tok->decoding_buffer will be one of:
///   1) NULL: need to call tok->decoding_readline to get a new line
///   2) PyUnicodeObject *: decoding_feof has called tok->decoding_readline and
///     stored the result in tok->decoding_buffer
///   3) PyByteArrayObject *: previous call to tok_readline_recode did not have enough room
///     (in the s buffer) to copy entire contents of the line read
///     by tok->decoding_readline.  tok->decoding_buffer has the overflow.
///     In this case, tok_readline_recode is called in a loop (with an expanded buffer)
///     until the buffer ends with a '\n' (or until the end of the file is
///     reached): see tok_nextc and its calls to tok_reserve_buf.
#[unsafe(no_mangle)]
extern "C-unwind" fn _PyLexer_tok_reserve_buf(tok: *mut tok_state, size: i64) -> i32 {
    let cur: i64 = (unsafe { unsafe { (*tok).cur }.offset_from((*tok).buf as *mut i8) }) as i64;
    let oldsize: i64 = (unsafe { unsafe { (*tok).inp }.offset_from((*tok).buf as *mut i8) }) as i64;
    let newsize: i64 = oldsize.wrapping_add(Py_MAX!(size, oldsize >> 1i32));
    if newsize > ((unsafe { unsafe { (*tok).end }.offset_from((*tok).buf as *const i8) }) as i64) {
        let mut newbuf: *mut i8 = unsafe { (*tok).buf };
        let start: i64 = if unsafe { (*tok).start }.is_null() {
            1i32.wrapping_neg() as i64
        } else {
            (unsafe { unsafe { (*tok).start }.offset_from((*tok).buf as *const i8) }) as i64
        };
        let line_start: i64 = if unsafe { (*tok).start }.is_null() {
            1i32.wrapping_neg() as i64
        } else {
            (unsafe { unsafe { (*tok).line_start }.offset_from((*tok).buf as *const i8) }) as i64
        };
        let multi_line_start: i64 =
            (unsafe { unsafe { (*tok).multi_line_start }.offset_from((*tok).buf as *const i8) })
                as i64;
        _PyLexer_remember_fstring_buffers(tok);
        newbuf = (unsafe { PyMem_Realloc(newbuf as *mut (), newsize as u64) }) as *mut i8;
        if newbuf.is_null() {
            unsafe {
                (*tok).done = 15;
            }
            return 0;
        }
        unsafe {
            (*tok).buf = newbuf;
        }
        unsafe {
            (*tok).cur = unsafe { (*tok).buf }.wrapping_offset(cur as isize);
        }
        unsafe {
            (*tok).inp = unsafe { (*tok).buf }.wrapping_offset(oldsize as isize);
        }
        unsafe {
            (*tok).end = unsafe { (*tok).buf }.wrapping_offset(newsize as isize) as *const i8;
        }
        unsafe {
            (*tok).start = (if start < 0 {
                core::ptr::null_mut::<i8>()
            } else {
                unsafe { (*tok).buf }.wrapping_offset(start as isize)
            }) as *const i8;
        }
        unsafe {
            (*tok).line_start = (if line_start < 0 {
                core::ptr::null_mut::<i8>()
            } else {
                unsafe { (*tok).buf }.wrapping_offset(line_start as isize)
            }) as *const i8;
        }
        unsafe {
            (*tok).multi_line_start = (if multi_line_start < 0 {
                core::ptr::null_mut::<i8>()
            } else {
                unsafe { (*tok).buf }.wrapping_offset(multi_line_start as isize)
            }) as *const i8;
        }
        _PyLexer_restore_fstring_buffers(tok);
    }
    1
}
