//! Generated automatically from /home/takashi/c-corpus/cpython/Modules/config.c.in by makesetup.
//!
//! Module configuration
//! The size of 'void *', as computed by sizeof.
//! Macro to use the more powerful/dangerous C-style cast even in C++.
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
//! Remote debugger support
//! PY_TIMEOUT_T is the integral type used to specify timeouts when waiting
//!
//! on a lock (see PyThread_acquire_lock_timed() below).
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
unsafe extern "C" {
    fn PyInit_atexit() -> *mut _object;
    fn PyInit_faulthandler() -> *mut _object;
    fn PyInit_posix() -> *mut _object;
    fn PyInit__signal() -> *mut _object;
    fn PyInit__tracemalloc() -> *mut _object;
    fn PyInit__suggestions() -> *mut _object;
    fn PyInit__datetime() -> *mut _object;
    fn PyInit__codecs() -> *mut _object;
    fn PyInit__collections() -> *mut _object;
    fn PyInit_errno() -> *mut _object;
    fn PyInit__io() -> *mut _object;
    fn PyInit_itertools() -> *mut _object;
    fn PyInit__sre() -> *mut _object;
    fn PyInit__sysconfig() -> *mut _object;
    fn PyInit__thread() -> *mut _object;
    fn PyInit_time() -> *mut _object;
    fn PyInit__types() -> *mut _object;
    fn PyInit__typing() -> *mut _object;
    fn PyInit__weakref() -> *mut _object;
    fn PyInit__abc() -> *mut _object;
    fn PyInit__functools() -> *mut _object;
    fn PyInit__locale() -> *mut _object;
    fn PyInit__opcode() -> *mut _object;
    fn PyInit__operator() -> *mut _object;
    fn PyInit__stat() -> *mut _object;
    fn PyInit__symtable() -> *mut _object;
    fn PyInit_pwd() -> *mut _object;
    fn PyMarshal_Init() -> *mut _object;
    fn PyInit__imp() -> *mut _object;
    fn PyInit_gc() -> *mut _object;
    fn PyInit__ast() -> *mut _object;
    fn PyInit__tokenize() -> *mut _object;
    fn PyInit__contextvars() -> *mut _object;
    fn _PyWarnings_Init() -> *mut _object;
    fn PyInit__string() -> *mut _object;
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

#[repr(C)]
#[derive(Clone, Copy)]
struct _inittab {
    name: *const i8,
    initfunc: Option<unsafe extern "C-unwind" fn() -> *mut _object>,
}

#[repr(C, align(16))]
struct __SlateAlign16<T>(T);

unsafe impl<T> Sync for __SlateAlign16<T> {}

// !!! !!! !!! This file is edited by the makesetup script !!! !!! !!!
// This file contains the table of built-in modules.
// See create_builtin() in import.c.
// -- ADDMODULE MARKER 1 --
/// -- ADDMODULE MARKER 2 --
/// This module lives in marshal.c
/// This lives in import.c
/// This lives in Python/Python-ast.c
/// This lives in Python/Python-tokenize.c
/// These entries are here for sys.builtin_module_names
/// This lives in gcmodule.c
/// This lives in Python/_contextvars.c
/// This lives in _warnings.c
/// This lives in Objects/unicodeobject.c
/// Sentinel
#[unsafe(no_mangle)]
static mut _PyImport_Inittab: __SlateAlign16<[_inittab; 38]> = __SlateAlign16([
    _inittab {
        name: (b"atexit\0".as_ptr() as *mut i8) as *const i8,
        initfunc: unsafe {
            core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn() -> *mut _object>>(
                PyInit_atexit as *const (),
            )
        },
    },
    _inittab {
        name: (b"faulthandler\0".as_ptr() as *mut i8) as *const i8,
        initfunc: unsafe {
            core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn() -> *mut _object>>(
                PyInit_faulthandler as *const (),
            )
        },
    },
    _inittab {
        name: (b"posix\0".as_ptr() as *mut i8) as *const i8,
        initfunc: unsafe {
            core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn() -> *mut _object>>(
                PyInit_posix as *const (),
            )
        },
    },
    _inittab {
        name: (b"_signal\0".as_ptr() as *mut i8) as *const i8,
        initfunc: unsafe {
            core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn() -> *mut _object>>(
                PyInit__signal as *const (),
            )
        },
    },
    _inittab {
        name: (b"_tracemalloc\0".as_ptr() as *mut i8) as *const i8,
        initfunc: unsafe {
            core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn() -> *mut _object>>(
                PyInit__tracemalloc as *const (),
            )
        },
    },
    _inittab {
        name: (b"_suggestions\0".as_ptr() as *mut i8) as *const i8,
        initfunc: unsafe {
            core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn() -> *mut _object>>(
                PyInit__suggestions as *const (),
            )
        },
    },
    _inittab {
        name: (b"_datetime\0".as_ptr() as *mut i8) as *const i8,
        initfunc: unsafe {
            core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn() -> *mut _object>>(
                PyInit__datetime as *const (),
            )
        },
    },
    _inittab {
        name: (b"_codecs\0".as_ptr() as *mut i8) as *const i8,
        initfunc: unsafe {
            core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn() -> *mut _object>>(
                PyInit__codecs as *const (),
            )
        },
    },
    _inittab {
        name: (b"_collections\0".as_ptr() as *mut i8) as *const i8,
        initfunc: unsafe {
            core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn() -> *mut _object>>(
                PyInit__collections as *const (),
            )
        },
    },
    _inittab {
        name: (b"errno\0".as_ptr() as *mut i8) as *const i8,
        initfunc: unsafe {
            core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn() -> *mut _object>>(
                PyInit_errno as *const (),
            )
        },
    },
    _inittab {
        name: (b"_io\0".as_ptr() as *mut i8) as *const i8,
        initfunc: unsafe {
            core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn() -> *mut _object>>(
                PyInit__io as *const (),
            )
        },
    },
    _inittab {
        name: (b"itertools\0".as_ptr() as *mut i8) as *const i8,
        initfunc: unsafe {
            core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn() -> *mut _object>>(
                PyInit_itertools as *const (),
            )
        },
    },
    _inittab {
        name: (b"_sre\0".as_ptr() as *mut i8) as *const i8,
        initfunc: unsafe {
            core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn() -> *mut _object>>(
                PyInit__sre as *const (),
            )
        },
    },
    _inittab {
        name: (b"_sysconfig\0".as_ptr() as *mut i8) as *const i8,
        initfunc: unsafe {
            core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn() -> *mut _object>>(
                PyInit__sysconfig as *const (),
            )
        },
    },
    _inittab {
        name: (b"_thread\0".as_ptr() as *mut i8) as *const i8,
        initfunc: unsafe {
            core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn() -> *mut _object>>(
                PyInit__thread as *const (),
            )
        },
    },
    _inittab {
        name: (b"time\0".as_ptr() as *mut i8) as *const i8,
        initfunc: unsafe {
            core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn() -> *mut _object>>(
                PyInit_time as *const (),
            )
        },
    },
    _inittab {
        name: (b"_types\0".as_ptr() as *mut i8) as *const i8,
        initfunc: unsafe {
            core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn() -> *mut _object>>(
                PyInit__types as *const (),
            )
        },
    },
    _inittab {
        name: (b"_typing\0".as_ptr() as *mut i8) as *const i8,
        initfunc: unsafe {
            core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn() -> *mut _object>>(
                PyInit__typing as *const (),
            )
        },
    },
    _inittab {
        name: (b"_weakref\0".as_ptr() as *mut i8) as *const i8,
        initfunc: unsafe {
            core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn() -> *mut _object>>(
                PyInit__weakref as *const (),
            )
        },
    },
    _inittab {
        name: (b"_abc\0".as_ptr() as *mut i8) as *const i8,
        initfunc: unsafe {
            core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn() -> *mut _object>>(
                PyInit__abc as *const (),
            )
        },
    },
    _inittab {
        name: (b"_functools\0".as_ptr() as *mut i8) as *const i8,
        initfunc: unsafe {
            core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn() -> *mut _object>>(
                PyInit__functools as *const (),
            )
        },
    },
    _inittab {
        name: (b"_locale\0".as_ptr() as *mut i8) as *const i8,
        initfunc: unsafe {
            core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn() -> *mut _object>>(
                PyInit__locale as *const (),
            )
        },
    },
    _inittab {
        name: (b"_opcode\0".as_ptr() as *mut i8) as *const i8,
        initfunc: unsafe {
            core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn() -> *mut _object>>(
                PyInit__opcode as *const (),
            )
        },
    },
    _inittab {
        name: (b"_operator\0".as_ptr() as *mut i8) as *const i8,
        initfunc: unsafe {
            core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn() -> *mut _object>>(
                PyInit__operator as *const (),
            )
        },
    },
    _inittab {
        name: (b"_stat\0".as_ptr() as *mut i8) as *const i8,
        initfunc: unsafe {
            core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn() -> *mut _object>>(
                PyInit__stat as *const (),
            )
        },
    },
    _inittab {
        name: (b"_symtable\0".as_ptr() as *mut i8) as *const i8,
        initfunc: unsafe {
            core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn() -> *mut _object>>(
                PyInit__symtable as *const (),
            )
        },
    },
    _inittab {
        name: (b"pwd\0".as_ptr() as *mut i8) as *const i8,
        initfunc: unsafe {
            core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn() -> *mut _object>>(
                PyInit_pwd as *const (),
            )
        },
    },
    _inittab {
        name: (b"marshal\0".as_ptr() as *mut i8) as *const i8,
        initfunc: unsafe {
            core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn() -> *mut _object>>(
                PyMarshal_Init as *const (),
            )
        },
    },
    _inittab {
        name: (b"_imp\0".as_ptr() as *mut i8) as *const i8,
        initfunc: unsafe {
            core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn() -> *mut _object>>(
                PyInit__imp as *const (),
            )
        },
    },
    _inittab {
        name: (b"_ast\0".as_ptr() as *mut i8) as *const i8,
        initfunc: unsafe {
            core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn() -> *mut _object>>(
                PyInit__ast as *const (),
            )
        },
    },
    _inittab {
        name: (b"_tokenize\0".as_ptr() as *mut i8) as *const i8,
        initfunc: unsafe {
            core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn() -> *mut _object>>(
                PyInit__tokenize as *const (),
            )
        },
    },
    _inittab {
        name: (b"builtins\0".as_ptr() as *mut i8) as *const i8,
        initfunc: None,
    },
    _inittab {
        name: (b"sys\0".as_ptr() as *mut i8) as *const i8,
        initfunc: None,
    },
    _inittab {
        name: (b"gc\0".as_ptr() as *mut i8) as *const i8,
        initfunc: unsafe {
            core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn() -> *mut _object>>(
                PyInit_gc as *const (),
            )
        },
    },
    _inittab {
        name: (b"_contextvars\0".as_ptr() as *mut i8) as *const i8,
        initfunc: unsafe {
            core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn() -> *mut _object>>(
                PyInit__contextvars as *const (),
            )
        },
    },
    _inittab {
        name: (b"_warnings\0".as_ptr() as *mut i8) as *const i8,
        initfunc: unsafe {
            core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn() -> *mut _object>>(
                _PyWarnings_Init as *const (),
            )
        },
    },
    _inittab {
        name: (b"_string\0".as_ptr() as *mut i8) as *const i8,
        initfunc: unsafe {
            core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn() -> *mut _object>>(
                PyInit__string as *const (),
            )
        },
    },
    _inittab {
        name: core::ptr::null::<i8>(),
        initfunc: None,
    },
]);
