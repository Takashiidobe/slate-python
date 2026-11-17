//! The author of this software is David M. Gay.
//!
//! Copyright (c) 1991, 2000, 2001 by Lucent Technologies.
//!
//! Permission to use, copy, modify, and distribute this software for any
//! purpose without fee is hereby granted, provided that this entire notice
//! is included in all copies of any software which is or includes a copy
//! or modification of this software and in all copies of the supporting
//! documentation for such software.
//!
//! THIS SOFTWARE IS BEING PROVIDED "AS IS", WITHOUT ANY EXPRESS OR IMPLIED
//! WARRANTY.  IN PARTICULAR, NEITHER THE AUTHOR NOR LUCENT MAKES ANY
//! REPRESENTATION OR WARRANTY OF ANY KIND CONCERNING THE MERCHANTABILITY
//! OF THIS SOFTWARE OR ITS FITNESS FOR ANY PARTICULAR PURPOSE.
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
//! Total tool ids available
// Signed variants of the above
// Prevent using an expression as a l-value.
//
// For example, "int x; _Py_RVALUE(x) = 1;" fails with a compiler error.
// 64KiB  (32KiB on 32-bit)
// 1024
// Used as a special value to encode block sizes in 32 bits.
// 35 == mi_segment_bin(MI_SLICES_PER_SEGMENT)
// initialize a local variable to zero; use memset as compilers optimize constant sized memset's
macro_rules! Bcopy {
    ($x:expr, $y:expr) => {
        unsafe {
            core::ptr::copy_nonoverlapping(
                (((core::ptr::addr_of_mut!((*$y).sign) as *mut i8) as *const ())
                    as *const core::ffi::c_void) as *const u8,
                (((core::ptr::addr_of_mut!((*$x).sign) as *mut i8) as *mut ())
                    as *mut core::ffi::c_void) as *mut u8,
                (((*$y).wds as u64)
                    .wrapping_mul(core::mem::size_of::<i32>() as u64)
                    .wrapping_add(2u64.wrapping_mul(core::mem::size_of::<i32>() as u64))
                    as usize) as usize,
            )
        };
    };
}

macro_rules! dval {
    ($x:expr) => {
        unsafe { (*$x).d }
    };
}

macro_rules! word0 {
    ($x:expr) => {
        unsafe { *((*$x).L.as_mut_ptr() as *mut u32).wrapping_offset(1 as isize) }
    };
}

macro_rules! word1 {
    ($x:expr) => {
        unsafe { *((*$x).L.as_mut_ptr() as *mut u32).wrapping_offset(0 as isize) }
    };
}

unsafe extern "C" {
    #[thread_local]
    static mut _Py_tss_tstate: *mut _ts;
    fn j1(arg0: f64) -> f64;
    fn PyMem_Malloc(arg0: u64) -> *mut ();
    fn PyMem_Free(arg0: *mut ());
    fn PyStatus_Ok() -> __SlateRecord_initconfig_h_19;
    fn PyStatus_NoMemory() -> __SlateRecord_initconfig_h_19;
    fn __builtin_flt_rounds() -> i32;
    fn __slate_fenv_flt_rounds() -> i32;
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord_atomic_wide_counter_h_33 {
    __value64: u64,
    __value32: __SlateRecord_atomic_wide_counter_h_33_2,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_atomic_wide_counter_h_33_2 {
    __low: u32,
    __high: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __pthread_internal_list {
    __prev: *mut __pthread_internal_list,
    __next: *mut __pthread_internal_list,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __pthread_mutex_s {
    __lock: i32,
    __count: u32,
    __owner: i32,
    __nusers: u32,
    __kind: i32,
    __spins: i16,
    __glibc_reserved: i16,
    __list: __pthread_internal_list,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __pthread_cond_s {
    __wseq: __SlateRecord_atomic_wide_counter_h_33,
    __g1_start: __SlateRecord_atomic_wide_counter_h_33,
    __g_size: [u32; 2],
    __g1_orig_size: u32,
    __wrefs: u32,
    __g_signals: [u32; 2],
    __unused_initialized_1: u32,
    __unused_initialized_2: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord_pthreadtypes_h_72 {
    __data: __pthread_mutex_s,
    __size: [i8; 40],
    __align: i64,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord_pthreadtypes_h_80 {
    __data: __pthread_cond_s,
    __size: [i8; 48],
    __align: i64,
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

#[repr(C)]
#[derive(Clone, Copy)]
struct PyMutex {
    _bits: u8,
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
struct _frame {}

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
struct __SlateRecord_pyerrors_h_15 {
    ob_base: _object,
    dict: *mut _object,
    args: *mut _object,
    notes: *mut _object,
    traceback: *mut _object,
    context: *mut _object,
    cause: *mut _object,
    suppress_context: i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct PyMethodDef {
    ml_name: *const i8,
    ml_meth: Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
    ml_flags: i32,
    ml_doc: *const i8,
}

// NULL or a tuple
// NULL or a dict
// NULL or a tuple of cell objects
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_funcobject_h_62 {
    ob_base: _object,
    func_globals: *mut _object,
    func_builtins: *mut _object,
    func_name: *mut _object,
    func_qualname: *mut _object,
    func_code: *mut _object,
    func_defaults: *mut _object,
    func_kwdefaults: *mut _object,
    func_closure: *mut _object,
    func_doc: *mut _object,
    func_dict: *mut _object,
    func_weakreflist: *mut _object,
    func_module: *mut _object,
    func_annotations: *mut _object,
    func_annotate: *mut _object,
    func_typeparams: *mut _object,
    vectorcall: Option<
        unsafe extern "C-unwind" fn(
            *mut _object,
            *const *mut _object,
            u64,
            *mut _object,
        ) -> *mut _object,
    >,
    func_version: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_code_h_16 {
    _co_code: *mut _object,
    _co_varnames: *mut _object,
    _co_cellvars: *mut _object,
    _co_freevars: *mut _object,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_code_h_22 {
    size: i32,
    capacity: i32,
    executors: [*mut _PyExecutorObject; 1],
}

// To avoid repeating ourselves in deepfreeze.py, all PyCodeObject members are
//
// defined in this macro:
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
#[repr(C)]
#[derive(Clone, Copy)]
struct PyCodeObject {
    ob_base: __SlateRecord_object_h_172,
    co_consts: *mut _object,
    co_names: *mut _object,
    co_exceptiontable: *mut _object,
    co_flags: i32,
    co_argcount: i32,
    co_posonlyargcount: i32,
    co_kwonlyargcount: i32,
    co_stacksize: i32,
    co_firstlineno: i32,
    co_nlocalsplus: i32,
    co_framesize: i32,
    co_nlocals: i32,
    co_ncellvars: i32,
    co_nfreevars: i32,
    co_version: u32,
    co_localsplusnames: *mut _object,
    co_localspluskinds: *mut _object,
    co_filename: *mut _object,
    co_name: *mut _object,
    co_qualname: *mut _object,
    co_linetable: *mut _object,
    co_weakreflist: *mut _object,
    co_executors: *mut __SlateRecord_code_h_22,
    _co_cached: *mut __SlateRecord_code_h_16,
    _co_instrumentation_version: u64,
    _co_monitoring: *mut _PyCoMonitoringData,
    _co_unique_id: i64,
    _co_firsttraceable: i32,
    co_extra: *mut (),
    co_code_adaptive: [i8; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_initconfig_h_19 {
    _type: u32,
    func: *const i8,
    err_msg: *const i8,
    exitcode: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_initconfig_h_36 {
    length: i64,
    items: *mut *mut i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct PyConfig {
    _config_init: i32,
    isolated: i32,
    use_environment: i32,
    dev_mode: i32,
    install_signal_handlers: i32,
    use_hash_seed: i32,
    hash_seed: u64,
    faulthandler: i32,
    tracemalloc: i32,
    perf_profiling: i32,
    remote_debug: i32,
    import_time: i32,
    code_debug_ranges: i32,
    show_ref_count: i32,
    dump_refs: i32,
    dump_refs_file: *mut i32,
    malloc_stats: i32,
    filesystem_encoding: *mut i32,
    filesystem_errors: *mut i32,
    pycache_prefix: *mut i32,
    parse_argv: i32,
    orig_argv: __SlateRecord_initconfig_h_36,
    argv: __SlateRecord_initconfig_h_36,
    xoptions: __SlateRecord_initconfig_h_36,
    warnoptions: __SlateRecord_initconfig_h_36,
    site_import: i32,
    bytes_warning: i32,
    warn_default_encoding: i32,
    inspect: i32,
    interactive: i32,
    optimization_level: i32,
    parser_debug: i32,
    write_bytecode: i32,
    verbose: i32,
    quiet: i32,
    user_site_directory: i32,
    configure_c_stdio: i32,
    buffered_stdio: i32,
    stdio_encoding: *mut i32,
    stdio_errors: *mut i32,
    check_hash_pycs_mode: *mut i32,
    use_frozen_modules: i32,
    safe_path: i32,
    int_max_str_digits: i32,
    thread_inherit_context: i32,
    context_aware_warnings: i32,
    cpu_count: i32,
    pathconfig_warnings: i32,
    program_name: *mut i32,
    pythonpath_env: *mut i32,
    home: *mut i32,
    platlibdir: *mut i32,
    module_search_paths_set: i32,
    module_search_paths: __SlateRecord_initconfig_h_36,
    stdlib_dir: *mut i32,
    executable: *mut i32,
    base_executable: *mut i32,
    prefix: *mut i32,
    base_prefix: *mut i32,
    exec_prefix: *mut i32,
    base_exec_prefix: *mut i32,
    skip_source_first_line: i32,
    run_command: *mut i32,
    run_module: *mut i32,
    run_filename: *mut i32,
    sys_path_0: *mut i32,
    _install_importlib: i32,
    _init_main: i32,
    _is_python_build: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pystate_h_35 {
    debugger_pending_call: i32,
    debugger_script_path: [i8; 512],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _PyExecutorObject {}

#[repr(C)]
#[derive(Clone, Copy)]
struct _err_stackitem {
    exc_value: *mut _object,
    previous_item: *mut _err_stackitem,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _PyInterpreterFrame {}

#[repr(C)]
#[derive(Clone, Copy)]
struct _stack_chunk {
    previous: *mut _stack_chunk,
    size: u64,
    top: u64,
    data: [*mut _object; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _ts {
    prev: *mut _ts,
    next: *mut _ts,
    interp: *mut _is,
    eval_breaker: u64,
    _status: __SlateRecord_pystate_h_78,
    holds_gil: i32,
    _whence: i32,
    state: i32,
    py_recursion_remaining: i32,
    py_recursion_limit: i32,
    recursion_headroom: i32,
    tracing: i32,
    what_event: i32,
    current_frame: *mut _PyInterpreterFrame,
    c_profilefunc:
        Option<unsafe extern "C-unwind" fn(*mut _object, *mut _frame, i32, *mut _object) -> i32>,
    c_tracefunc:
        Option<unsafe extern "C-unwind" fn(*mut _object, *mut _frame, i32, *mut _object) -> i32>,
    c_profileobj: *mut _object,
    c_traceobj: *mut _object,
    current_exception: *mut _object,
    exc_info: *mut _err_stackitem,
    dict: *mut _object,
    gilstate_counter: i32,
    async_exc: *mut _object,
    thread_id: u64,
    native_thread_id: u64,
    delete_later: *mut _object,
    critical_section: u64,
    coroutine_origin_tracking_depth: i32,
    async_gen_firstiter: *mut _object,
    async_gen_finalizer: *mut _object,
    context: *mut _object,
    context_ver: u64,
    id: u64,
    datastack_chunk: *mut _stack_chunk,
    datastack_top: *mut *mut _object,
    datastack_limit: *mut *mut _object,
    exc_state: _err_stackitem,
    current_executor: *mut _object,
    dict_global_version: u64,
    threading_local_key: *mut _object,
    threading_local_sentinel: *mut _object,
    remote_debugger_support: __SlateRecord_pystate_h_35,
    datastack_cached_chunk: *mut _stack_chunk,
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
struct wrapperbase {
    name: *const i8,
    offset: i32,
    function: *mut (),
    wrapper:
        Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object, *mut ()) -> *mut _object>,
    doc: *const i8,
    flags: i32,
    name_strobj: *mut _object,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_modsupport_h_9 {
    v: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pystate_h_78 {
    __slate_bits_0: __slate_bits::__SlateBits137U0,
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_dg_strtod(mut s00: *const i8, se: *mut *mut i8) -> f64 {
    let mut __v37: bool = unsafe { core::mem::zeroed() };
    let mut __v38: bool = unsafe { core::mem::zeroed() };
    let mut __v102: bool = unsafe { core::mem::zeroed() };
    let mut __v132: bool = unsafe { core::mem::zeroed() };
    let mut result: f64 = unsafe { core::mem::zeroed() };
    let mut fraclen: u64 = unsafe { core::mem::zeroed() };
    let mut ndigits: u64 = unsafe { core::mem::zeroed() };
    let mut delta: *mut Bigint = unsafe { core::mem::zeroed() };
    let mut bs: *mut Bigint = unsafe { core::mem::zeroed() };
    let mut bd0: *mut Bigint = unsafe { core::mem::zeroed() };
    let mut bd: *mut Bigint = unsafe { core::mem::zeroed() };
    let mut bb: *mut Bigint = unsafe { core::mem::zeroed() };
    let mut bc: core::mem::MaybeUninit<BCinfo> = core::mem::MaybeUninit::uninit();
    let __slate_slot_180: *mut BCinfo = core::ptr::addr_of_mut!(bc) as *mut BCinfo;
    let mut L: i32 = unsafe { core::mem::zeroed() };
    let mut abs_exp: u32 = unsafe { core::mem::zeroed() };
    let mut z: u32 = unsafe { core::mem::zeroed() };
    let mut y: u32 = unsafe { core::mem::zeroed() };
    let mut rv0: core::mem::MaybeUninit<__SlateRecord_dtoa_c_181> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_185: *mut __SlateRecord_dtoa_c_181 =
        core::ptr::addr_of_mut!(rv0) as *mut __SlateRecord_dtoa_c_181;
    let mut rv: core::mem::MaybeUninit<__SlateRecord_dtoa_c_181> = core::mem::MaybeUninit::uninit();
    let __slate_slot_186: *mut __SlateRecord_dtoa_c_181 =
        core::ptr::addr_of_mut!(rv) as *mut __SlateRecord_dtoa_c_181;
    let mut adj: core::mem::MaybeUninit<__SlateRecord_dtoa_c_181> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_187: *mut __SlateRecord_dtoa_c_181 =
        core::ptr::addr_of_mut!(adj) as *mut __SlateRecord_dtoa_c_181;
    let mut aadj2: core::mem::MaybeUninit<__SlateRecord_dtoa_c_181> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_188: *mut __SlateRecord_dtoa_c_181 =
        core::ptr::addr_of_mut!(aadj2) as *mut __SlateRecord_dtoa_c_181;
    let mut aadj1: f64 = unsafe { core::mem::zeroed() };
    let mut aadj: f64 = unsafe { core::mem::zeroed() };
    let mut s1: *const i8 = unsafe { core::mem::zeroed() };
    let mut s0: *const i8 = unsafe { core::mem::zeroed() };
    let mut s: *const i8 = unsafe { core::mem::zeroed() };
    let mut sign: i32 = unsafe { core::mem::zeroed() };
    let mut odd: i32 = unsafe { core::mem::zeroed() };
    let mut nd0: i32 = unsafe { core::mem::zeroed() };
    let mut nd: i32 = unsafe { core::mem::zeroed() };
    let mut lz: i32 = unsafe { core::mem::zeroed() };
    let mut k: i32 = unsafe { core::mem::zeroed() };
    let mut j: i32 = unsafe { core::mem::zeroed() };
    let mut i: i32 = unsafe { core::mem::zeroed() };
    let mut esign: i32 = unsafe { core::mem::zeroed() };
    let mut e1: i32 = unsafe { core::mem::zeroed() };
    let mut e: i32 = unsafe { core::mem::zeroed() };
    let mut dsign: i32 = unsafe { core::mem::zeroed() };
    let mut c: i32 = unsafe { core::mem::zeroed() };
    let mut bs2: i32 = unsafe { core::mem::zeroed() };
    let mut bd5: i32 = unsafe { core::mem::zeroed() };
    let mut bd2: i32 = unsafe { core::mem::zeroed() };
    let mut bbe: i32 = unsafe { core::mem::zeroed() };
    let mut bb5: i32 = unsafe { core::mem::zeroed() };
    let mut bb2: i32 = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_222: {
            bb = core::ptr::null_mut::<Bigint>();
            bd = core::ptr::null_mut::<Bigint>();
            bd0 = core::ptr::null_mut::<Bigint>();
            bs = core::ptr::null_mut::<Bigint>();
            delta = core::ptr::null_mut::<Bigint>();
            (*core::ptr::addr_of_mut!(*__slate_slot_186)).d = 0.0f64;
            // Start parsing.
            let __v171: *const i8 = s00;
            s = __v171;
            c = *__v171 as i32;
            // Parse optional sign, if present.
            sign = 0;
            let __t1: i32 = c;
            if __t1 == 45 {
                sign = 1;
            } else {
                if __t1 != 43 {
                    break '__join_222;
                }
            }
            let __v170: *const i8 = s;
            let __v169: *const i8 = __v170.wrapping_offset(1 as isize);
            s = __v169;
            c = *__v169 as i32;
        }
        // Skip leading zeros: lz is true iff there were leading zeros.
        s1 = s;
        while c == 48 {
            let __v168: *const i8 = s;
            let __v167: *const i8 = __v168.wrapping_offset(1 as isize);
            s = __v167;
            c = *__v167 as i32;
        }
        lz = (s != s1) as i32;
        // Point s0 at the first nonzero digit (if any).  fraclen will be the
        // number of digits between the decimal point and the end of the
        // digit string.  ndigits will be the total number of digits ignoring
        // leading zeros.
        let __v166: *const i8 = s;
        s1 = __v166;
        s0 = __v166;
        while 48 <= c && c <= 57 {
            let __v165: *const i8 = s;
            let __v164: *const i8 = __v165.wrapping_offset(1 as isize);
            s = __v164;
            c = *__v164 as i32;
        }
        ndigits = (s.offset_from(s1 as *const i8) as i64) as u64;
        fraclen = 0;
        // Parse decimal point and following digits.
        if c == 46 {
            let __v163: *const i8 = s;
            let __v162: *const i8 = __v163.wrapping_offset(1 as isize);
            s = __v162;
            c = *__v162 as i32;
            if ndigits == 0 {
                s1 = s;
                while c == 48 {
                    let __v161: *const i8 = s;
                    let __v160: *const i8 = __v161.wrapping_offset(1 as isize);
                    s = __v160;
                    c = *__v160 as i32;
                }
                lz = (lz != 0 || s != s1) as i32;
                let __v159: u64 = fraclen;
                fraclen = __v159.wrapping_add((s.offset_from(s1 as *const i8) as i64) as u64);
                s0 = s;
            }
            s1 = s;
            while 48 <= c && c <= 57 {
                let __v157: *const i8 = s;
                let __v156: *const i8 = __v157.wrapping_offset(1 as isize);
                s = __v156;
                c = *__v156 as i32;
            }
            let __v155: u64 = ndigits;
            ndigits = __v155.wrapping_add((s.offset_from(s1 as *const i8) as i64) as u64);
            let __v153: u64 = fraclen;
            fraclen = __v153.wrapping_add((s.offset_from(s1 as *const i8) as i64) as u64);
        }
        '__join_0: {
            // Now lz is true if and only if there were leading zero digits, and
            // ndigits gives the total number of digits ignoring leading zeros.  A
            // valid input must have at least one digit.
            if ndigits == 0 && lz == 0 {
                if !se.is_null() {
                    *se = s00 as *mut i8;
                }
            } else {
                // Range check ndigits and fraclen to make sure that they, and values
                // computed with them, can safely fit in an int.
                if ndigits > 1000000000 || fraclen > 1000000000 {
                    if !se.is_null() {
                        *se = s00 as *mut i8;
                    }
                } else {
                    nd = ndigits as i32;
                    nd0 = (ndigits as i32).wrapping_sub(fraclen as i32);
                    // Parse exponent.
                    e = 0;
                    if c == 101 || c == 69 {
                        '__join_196: {
                            s00 = s;
                            let __v151: *const i8 = s;
                            let __v150: *const i8 = __v151.wrapping_offset(1 as isize);
                            s = __v150;
                            c = *__v150 as i32;
                            // Exponent sign.
                            esign = 0;
                            let __t0: i32 = c;
                            if __t0 == 45 {
                                esign = 1;
                            } else {
                                if __t0 != 43 {
                                    break '__join_196;
                                }
                            }
                            let __v149: *const i8 = s;
                            let __v148: *const i8 = __v149.wrapping_offset(1 as isize);
                            s = __v148;
                            c = *__v148 as i32;
                        }
                        // Skip zeros.  lz is true iff there are leading zeros.
                        s1 = s;
                        while c == 48 {
                            let __v147: *const i8 = s;
                            let __v146: *const i8 = __v147.wrapping_offset(1 as isize);
                            s = __v146;
                            c = *__v146 as i32;
                        }
                        lz = (s != s1) as i32;
                        // Get absolute value of the exponent.
                        s1 = s;
                        abs_exp = 0;
                        while 48 <= c && c <= 57 {
                            abs_exp = 10u32
                                .wrapping_mul(abs_exp)
                                .wrapping_add(c.wrapping_sub(48) as u32);
                            let __v145: *const i8 = s;
                            let __v144: *const i8 = __v145.wrapping_offset(1 as isize);
                            s = __v144;
                            c = *__v144 as i32;
                        }
                        // abs_exp will be correct modulo 2**32.  But 10**9 < 2**32, so if
                        // there are at most 9 significant exponent digits then overflow is
                        // impossible.
                        if (s.offset_from(s1 as *const i8) as i64) > 9 || abs_exp > 1100000000 {
                            e = 1100000000;
                        } else {
                            e = abs_exp as i32;
                        }
                        if esign != 0 {
                            e = e.wrapping_neg();
                        }
                        // A valid exponent must have at least one digit.
                        if s == s1 && lz == 0 {
                            s = s00;
                        }
                    }
                    // Adjust exponent to take into account position of the point.
                    e = e.wrapping_sub(nd.wrapping_sub(nd0));
                    if nd0 <= 0 {
                        nd0 = nd;
                    }
                    // Finished parsing.  Set se to indicate how far we parsed
                    if !se.is_null() {
                        *se = s as *mut i8;
                    }
                    '__join_5: {
                        // If all digits were zero, exit with return value +-0.0.  Otherwise,
                        // strip trailing zeros: scan back until we hit a nonzero digit.
                        if nd != 0 {
                            i = nd;
                            '__join_174: {
                                loop {
                                    if i > 0 {
                                        i = i.wrapping_sub(1);
                                        if (*s0.wrapping_offset(
                                            (if i < nd0 { i } else { i.wrapping_add(1) }) as isize,
                                        ) as i32)
                                            != 48
                                        {
                                            break;
                                        }
                                    } else {
                                        break '__join_174;
                                    }
                                }
                                i = i.wrapping_add(1);
                            }
                            e = e.wrapping_add(nd.wrapping_sub(i));
                            nd = i;
                            if nd0 > nd {
                                nd0 = nd;
                            }
                            // Summary of parsing results.  After parsing, and dealing with zero
                            // inputs, we have values s0, nd0, nd, e, sign, where:
                            //
                            //  - s0 points to the first significant digit of the input string
                            //
                            //  - nd is the total number of significant digits (here, and
                            //    below, 'significant digits' means the set of digits of the
                            //    significand of the input that remain after ignoring leading
                            //    and trailing zeros).
                            //
                            //  - nd0 indicates the position of the decimal point, if present; it
                            //    satisfies 1 <= nd0 <= nd.  The nd significant digits are in
                            //    s0[0:nd0] and s0[nd0+1:nd+1] using the usual Python half-open slice
                            //    notation.  (If nd0 < nd, then s0[nd0] contains a '.'  character; if
                            //    nd0 == nd, then s0[nd0] could be any non-digit character.)
                            //
                            //  - e is the adjusted exponent: the absolute value of the number
                            //    represented by the original input string is n * 10**e, where
                            //    n is the integer represented by the concatenation of
                            //    s0[0:nd0] and s0[nd0+1:nd+1]
                            //
                            //  - sign gives the sign of the input:  1 for negative, 0 for positive
                            //
                            //  - the first and last significant digits are nonzero
                            // put first DBL_DIG+1 digits into integer y and z.
                            //
                            // - y contains the value represented by the first min(9, nd)
                            //   significant digits
                            //
                            // - if nd > 9, z contains the value represented by significant digits
                            //   with indices in [9, min(16, nd)).  So y * 10**(min(16, nd) - 9) + z
                            //   gives the value represented by the first min(16, nd) sig. digits.
                            let __v135: i32 = e;
                            e1 = __v135;
                            (*__slate_slot_180).e0 = __v135;
                            z = 0;
                            y = 0;
                            i = 0;
                            while i < nd {
                                if i < 9 {
                                    y = 10u32
                                        .wrapping_mul(y)
                                        .wrapping_add(*s0.wrapping_offset(
                                            (if i < nd0 { i } else { i.wrapping_add(1) }) as isize,
                                        )
                                            as u32)
                                        .wrapping_sub(48);
                                } else {
                                    if i < 16i32 {
                                        z = 10u32
                                            .wrapping_mul(z)
                                            .wrapping_add(*s0.wrapping_offset(
                                                (if i < nd0 { i } else { i.wrapping_add(1) })
                                                    as isize,
                                            )
                                                as u32)
                                            .wrapping_sub(48);
                                    } else {
                                        break;
                                    }
                                }
                                i = i.wrapping_add(1);
                            }
                            k = if nd < 16i32 { nd } else { 16i32 };
                            (*core::ptr::addr_of_mut!(*__slate_slot_186)).d = y as f64;
                            if k > 9 {
                                (*core::ptr::addr_of_mut!(*__slate_slot_186)).d = tens.0
                                    [k.wrapping_sub(9) as usize]
                                    * dval!(core::ptr::addr_of_mut!(*__slate_slot_186))
                                    + (z as f64);
                            }
                            if nd <= 15 {
                                __v132 = __slate_fenv_flt_rounds() == 1;
                            } else {
                                __v132 = false;
                            }
                            if __v132 {
                                if e == 0 {
                                    break '__join_5;
                                } else {
                                    if e > 0 {
                                        if e <= 22 {
                                            let __v128: *mut __SlateRecord_dtoa_c_181 =
                                                core::ptr::addr_of_mut!(*__slate_slot_186);
                                            (*__v128).d = (*__v128).d * tens.0[e as usize];
                                            break '__join_5;
                                        } else {
                                            i = 15i32.wrapping_sub(nd);
                                            if e <= 22i32.wrapping_add(i) {
                                                // A fancier test would sometimes let us do
                                                // this for larger i values.
                                                e = e.wrapping_sub(i);
                                                let __v123: *mut __SlateRecord_dtoa_c_181 =
                                                    core::ptr::addr_of_mut!(*__slate_slot_186);
                                                (*__v123).d = (*__v123).d * tens.0[i as usize];
                                                let __v120: *mut __SlateRecord_dtoa_c_181 =
                                                    core::ptr::addr_of_mut!(*__slate_slot_186);
                                                (*__v120).d = (*__v120).d * tens.0[e as usize];
                                                break '__join_5;
                                            }
                                        }
                                    } else {
                                        if e >= 22i32.wrapping_neg() {
                                            let __v131: *mut __SlateRecord_dtoa_c_181 =
                                                core::ptr::addr_of_mut!(*__slate_slot_186);
                                            (*__v131).d =
                                                (*__v131).d / tens.0[e.wrapping_neg() as usize];
                                            break '__join_5;
                                        }
                                    }
                                }
                            }
                            '__join_1: {
                                '__join_2: {
                                    e1 = e1.wrapping_add(nd.wrapping_sub(k));
                                    (*__slate_slot_180).scale = 0;
                                    // Get starting approximation = rv * 10**e1
                                    if e1 > 0 {
                                        let __v94: i32 = e1 & 15;
                                        i = __v94;
                                        if __v94 != 0 {
                                            let __v93: *mut __SlateRecord_dtoa_c_181 =
                                                core::ptr::addr_of_mut!(*__slate_slot_186);
                                            (*__v93).d = (*__v93).d * tens.0[i as usize];
                                        }
                                        let __v89: i32 = e1 & !15;
                                        e1 = __v89;
                                        if __v89 != 0 {
                                            if e1 > 308 {
                                                break '__join_1;
                                            } else {
                                                e1 >>= 4i32;
                                                j = 0;
                                                while e1 > 1 {
                                                    if e1 & 1 != 0 {
                                                        let __v86: *mut __SlateRecord_dtoa_c_181 = core::ptr::addr_of_mut!(
                                                            *__slate_slot_186
                                                        );
                                                        (*__v86).d =
                                                            (*__v86).d * bigtens.0[j as usize];
                                                    }
                                                    j = j.wrapping_add(1);
                                                    e1 >>= 1i32;
                                                }
                                                // The last multiplication could overflow.
                                                let __v79: *mut u32 =
                                                    ((*core::ptr::addr_of_mut!(*__slate_slot_186))
                                                        .L
                                                        .as_mut_ptr()
                                                        as *mut u32)
                                                        .wrapping_offset(1 as isize);
                                                let __v78: u32 = *__v79;
                                                *__v79 = __v78.wrapping_sub(55574528u32);
                                                let __v76: *mut __SlateRecord_dtoa_c_181 =
                                                    core::ptr::addr_of_mut!(*__slate_slot_186);
                                                (*__v76).d = (*__v76).d * bigtens.0[j as usize];
                                                let __v73: u32 = word0!(core::ptr::addr_of_mut!(
                                                    *__slate_slot_186
                                                )) & 2146435072;
                                                z = __v73;
                                                if __v73 > 2090860544u32 {
                                                    break '__join_1;
                                                } else {
                                                    if z > 2089811968u32 {
                                                        // set to largest number
                                                        //
                                                        // (Can't trust DBL_MAX)
                                                        *((*core::ptr::addr_of_mut!(
                                                            *__slate_slot_186
                                                        ))
                                                        .L
                                                        .as_mut_ptr()
                                                            as *mut u32)
                                                            .wrapping_offset(1 as isize) =
                                                            2146435071u32;
                                                        *((*core::ptr::addr_of_mut!(
                                                            *__slate_slot_186
                                                        ))
                                                        .L
                                                        .as_mut_ptr()
                                                            as *mut u32)
                                                            .wrapping_offset(0 as isize) =
                                                            4294967295;
                                                    } else {
                                                        let __v72: *mut u32 =
                                                            ((*core::ptr::addr_of_mut!(
                                                                *__slate_slot_186
                                                            ))
                                                            .L
                                                            .as_mut_ptr()
                                                                as *mut u32)
                                                                .wrapping_offset(1 as isize);
                                                        let __v71: u32 = *__v72;
                                                        *__v72 = __v71.wrapping_add(55574528u32);
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        if e1 < 0 {
                                            // The input decimal value lies in [10**e1, 10**(e1+16)).
                                            //
                                            // If e1 <= -512, underflow immediately.
                                            // If e1 <= -256, set bc.scale to 2*P.
                                            //
                                            // So for input value < 1e-256, bc.scale is always set;
                                            // for input value >= 1e-240, bc.scale is never set.
                                            // For input values in [1e-256, 1e-240), bc.scale may or may
                                            // not be set.
                                            e1 = e1.wrapping_neg();
                                            let __v115: i32 = e1 & 15;
                                            i = __v115;
                                            if __v115 != 0 {
                                                let __v114: *mut __SlateRecord_dtoa_c_181 =
                                                    core::ptr::addr_of_mut!(*__slate_slot_186);
                                                (*__v114).d = (*__v114).d / tens.0[i as usize];
                                            }
                                            let __v110: i32 = e1 >> 4i32;
                                            e1 = __v110;
                                            if __v110 != 0 {
                                                if e1 >= 32i32 {
                                                    break '__join_2;
                                                } else {
                                                    if e1 & 16 != 0 {
                                                        (*__slate_slot_180).scale = 106i32;
                                                    }
                                                    j = 0;
                                                    while e1 > 0 {
                                                        if e1 & 1 != 0 {
                                                            let __v109: *mut __SlateRecord_dtoa_c_181 = core::ptr::addr_of_mut!(*__slate_slot_186);
                                                            (*__v109).d = (*__v109).d
                                                                * tinytens.0[j as usize];
                                                        }
                                                        j = j.wrapping_add(1);
                                                        e1 >>= 1i32;
                                                    }
                                                    if (*__slate_slot_180).scale != 0 {
                                                        let __v101: i32 = 107u32.wrapping_sub(
                                                            (word0!(core::ptr::addr_of_mut!(
                                                                *__slate_slot_186
                                                            )) & 2146435072)
                                                                >> 20i32,
                                                        )
                                                            as i32;
                                                        j = __v101;
                                                        __v102 = __v101 > 0;
                                                    } else {
                                                        __v102 = false;
                                                    }
                                                    if __v102 {
                                                        // scaled rv is denormal; clear j low bits
                                                        if j >= 32 {
                                                            *((*core::ptr::addr_of_mut!(
                                                                *__slate_slot_186
                                                            ))
                                                            .L
                                                            .as_mut_ptr()
                                                                as *mut u32)
                                                                .wrapping_offset(0 as isize) = 0;
                                                            if j >= 53 {
                                                                *((*core::ptr::addr_of_mut!(
                                                                    *__slate_slot_186
                                                                ))
                                                                .L
                                                                .as_mut_ptr()
                                                                    as *mut u32)
                                                                    .wrapping_offset(1 as isize) =
                                                                    57671680u32;
                                                            } else {
                                                                let __v97: *mut u32 =
                                                                    ((*core::ptr::addr_of_mut!(
                                                                        *__slate_slot_186
                                                                    ))
                                                                    .L
                                                                    .as_mut_ptr()
                                                                        as *mut u32)
                                                                        .wrapping_offset(
                                                                            1 as isize,
                                                                        );
                                                                *__v97 = *__v97
                                                                    & 4294967295u32
                                                                        << j.wrapping_sub(32);
                                                            }
                                                        } else {
                                                            let __v100: *mut u32 =
                                                                ((*core::ptr::addr_of_mut!(
                                                                    *__slate_slot_186
                                                                ))
                                                                .L
                                                                .as_mut_ptr()
                                                                    as *mut u32)
                                                                    .wrapping_offset(0 as isize);
                                                            *__v100 = *__v100 & 4294967295u32 << j;
                                                        }
                                                    }
                                                    if dval!(core::ptr::addr_of_mut!(
                                                        *__slate_slot_186
                                                    )) == 0.0f64
                                                    {
                                                        break '__join_2;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    // Now the hard part -- adjusting rv to the correct value.
                                    // Put digits into bd: true value = bd * 10^e
                                    (*__slate_slot_180).nd = nd;
                                    (*__slate_slot_180).nd0 = nd0;
                                    // Only needed if nd > STRTOD_DIGLIM, but done here
                                    //
                                    // to silence an erroneous warning about bc.nd0
                                    //
                                    // possibly not being initialized.
                                    if nd > 40 {
                                        // ASSERT(STRTOD_DIGLIM >= 18); 18 == one more than the
                                        //
                                        // minimum number of decimal digits to distinguish double values
                                        //
                                        // in IEEE arithmetic.
                                        // Truncate input to 18 significant digits, then discard any trailing
                                        // zeros on the result by updating nd, nd0, e and y suitably. (There's
                                        // no need to update z; it's not reused beyond this point.)
                                        i = 18;
                                        '__join_110: {
                                            loop {
                                                if i > 0 {
                                                    // scan back until we hit a nonzero digit.  significant digit 'i'
                                                    // is s0[i] if i < nd0, s0[i+1] if i >= nd0.
                                                    i = i.wrapping_sub(1);
                                                    if (*s0.wrapping_offset(
                                                        (if i < nd0 {
                                                            i
                                                        } else {
                                                            i.wrapping_add(1)
                                                        })
                                                            as isize,
                                                    )
                                                        as i32)
                                                        != 48
                                                    {
                                                        break;
                                                    }
                                                } else {
                                                    break '__join_110;
                                                }
                                            }
                                            i = i.wrapping_add(1);
                                        }
                                        e = e.wrapping_add(nd.wrapping_sub(i));
                                        nd = i;
                                        if nd0 > nd {
                                            nd0 = nd;
                                        }
                                        if nd < 9 {
                                            // must recompute y
                                            y = 0;
                                            i = 0;
                                            while i < nd0 {
                                                y = 10u32
                                                    .wrapping_mul(y)
                                                    .wrapping_add(
                                                        *s0.wrapping_offset(i as isize) as u32
                                                    )
                                                    .wrapping_sub(48);
                                                i = i.wrapping_add(1);
                                            }
                                            while i < nd {
                                                y =
                                                    10u32
                                                        .wrapping_mul(y)
                                                        .wrapping_add(*s0.wrapping_offset(
                                                            i.wrapping_add(1) as isize,
                                                        )
                                                            as u32)
                                                        .wrapping_sub(48);
                                                i = i.wrapping_add(1);
                                            }
                                        }
                                    }
                                    '__join_3: {
                                        bd0 = s2b(s0, nd0, nd, y);
                                        if !bd0.is_null() {
                                            // Notation for the comments below.  Write:
                                            //
                                            // - dv for the absolute value of the number represented by the original
                                            //   decimal input string.
                                            //
                                            // - if we've truncated dv, write tdv for the truncated value.
                                            //   Otherwise, set tdv == dv.
                                            //
                                            // - srv for the quantity rv/2^bc.scale; so srv is the current binary
                                            //   approximation to tdv (and dv).  It should be exactly representable
                                            //   in an IEEE 754 double.
                                            '__join_9: {
                                                '__join_71: {
                                                    '__join_60: {
                                                        '__join_68: {
                                                            '__join_62: {
                                                                loop {
                                                                    // This is the main correction loop for _Py_dg_strtod.
                                                                    //
                                                                    // We've got a decimal value tdv, and a floating-point approximation
                                                                    // srv=rv/2^bc.scale to tdv.  The aim is to determine whether srv is
                                                                    // close enough (i.e., within 0.5 ulps) to tdv, and to compute a new
                                                                    // approximation if not.
                                                                    //
                                                                    // To determine whether srv is close enough to tdv, compute integers
                                                                    // bd, bb and bs proportional to tdv, srv and 0.5 ulp(srv)
                                                                    // respectively, and then use integer arithmetic to determine whether
                                                                    // |tdv - srv| is less than, equal to, or greater than 0.5 ulp(srv).
                                                                    bd = Balloc((*bd0).k);
                                                                    if bd.is_null() {
                                                                        break '__join_3;
                                                                    } else {
                                                                        Bcopy!(bd, bd0);
                                                                        bb = sd2b(
                                                                            core::ptr::addr_of_mut!(
                                                                                *__slate_slot_186
                                                                            ),
                                                                            (*__slate_slot_180)
                                                                                .scale,
                                                                            &mut (*core::ptr::addr_of_mut!(
                                                                                bbe
                                                                            )),
                                                                        ); // srv = bb * 2^bbe
                                                                        if bb.is_null() {
                                                                            break '__join_3;
                                                                        } else {
                                                                            // Record whether lsb of bb is odd, in case we need this
                                                                            // for the round-to-even step later.
                                                                            odd = (*((*bb)
                                                                                .x
                                                                                .as_mut_ptr()
                                                                                as *mut u32)
                                                                                .wrapping_offset(
                                                                                    0 as isize,
                                                                                )
                                                                                & 1)
                                                                                as i32;
                                                                            // tdv = bd * 10**e;  srv = bb * 2**bbe
                                                                            bs = i2b(1i32);
                                                                            if bs.is_null() {
                                                                                break '__join_3;
                                                                            } else {
                                                                                if e >= 0 {
                                                                                    bb5 = 0;
                                                                                    bb2 = 0;
                                                                                    let __v58: i32 =
                                                                                        e;
                                                                                    bd5 = __v58;
                                                                                    bd2 = __v58;
                                                                                } else {
                                                                                    let __v59: i32 = e.wrapping_neg();
                                                                                    bb5 = __v59;
                                                                                    bb2 = __v59;
                                                                                    bd5 = 0;
                                                                                    bd2 = 0;
                                                                                }
                                                                                if bbe >= 0 {
                                                                                    bb2 = bb2.wrapping_add(bbe);
                                                                                } else {
                                                                                    bd2 = bd2.wrapping_sub(bbe);
                                                                                }
                                                                                bs2 = bb2;
                                                                                bb2 = bb2
                                                                                    .wrapping_add(
                                                                                        1,
                                                                                    );
                                                                                bd2 = bd2
                                                                                    .wrapping_add(
                                                                                        1,
                                                                                    );
                                                                                // At this stage bd5 - bb5 == e == bd2 - bb2 + bbe, bb2 - bs2 == 1,
                                                                                // and bs == 1, so:
                                                                                //
                                                                                //    tdv == bd * 10**e = bd * 2**(bbe - bb2 + bd2) * 5**(bd5 - bb5)
                                                                                //    srv == bb * 2**bbe = bb * 2**(bbe - bb2 + bb2)
                                                                                //    0.5 ulp(srv) == 2**(bbe-1) = bs * 2**(bbe - bb2 + bs2)
                                                                                //
                                                                                // It follows that:
                                                                                //
                                                                                //    M * tdv = bd * 2**bd2 * 5**bd5
                                                                                //    M * srv = bb * 2**bb2 * 5**bb5
                                                                                //    M * 0.5 ulp(srv) = bs * 2**bs2 * 5**bb5
                                                                                //
                                                                                // for some constant M.  (Actually, M == 2**(bb2 - bbe) * 5**bb5, but
                                                                                // this fact is not needed below.)
                                                                                // Remove factor of 2**i, where i = min(bb2, bd2, bs2).
                                                                                i = if bb2 < bd2 {
                                                                                    bb2
                                                                                } else {
                                                                                    bd2
                                                                                };
                                                                                if i > bs2 {
                                                                                    i = bs2;
                                                                                }
                                                                                if i > 0 {
                                                                                    bb2 = bb2.wrapping_sub(i);
                                                                                    bd2 = bd2.wrapping_sub(i);
                                                                                    bs2 = bs2.wrapping_sub(i);
                                                                                }
                                                                                // Scale bb, bd, bs by the appropriate powers of 2 and 5.
                                                                                if bb5 > 0 {
                                                                                    bs = pow5mult(
                                                                                        bs, bb5,
                                                                                    );
                                                                                    if bs.is_null()
                                                                                    {
                                                                                        break '__join_3;
                                                                                    } else {
                                                                                        let bb1: *mut Bigint = mult(bs, bb);
                                                                                        Bfree(bb);
                                                                                        bb = bb1;
                                                                                        if bb.is_null() {
break '__join_3;
}
                                                                                    }
                                                                                }
                                                                                if bb2 > 0 {
                                                                                    bb = lshift(
                                                                                        bb, bb2,
                                                                                    );
                                                                                    if bb.is_null()
                                                                                    {
                                                                                        break '__join_3;
                                                                                    }
                                                                                }
                                                                                if bd5 > 0 {
                                                                                    bd = pow5mult(
                                                                                        bd, bd5,
                                                                                    );
                                                                                    if bd.is_null()
                                                                                    {
                                                                                        break '__join_3;
                                                                                    }
                                                                                }
                                                                                if bd2 > 0 {
                                                                                    bd = lshift(
                                                                                        bd, bd2,
                                                                                    );
                                                                                    if bd.is_null()
                                                                                    {
                                                                                        break '__join_3;
                                                                                    }
                                                                                }
                                                                                if bs2 > 0 {
                                                                                    bs = lshift(
                                                                                        bs, bs2,
                                                                                    );
                                                                                    if bs.is_null()
                                                                                    {
                                                                                        break '__join_3;
                                                                                    }
                                                                                }
                                                                                // Now bd, bb and bs are scaled versions of tdv, srv and 0.5 ulp(srv),
                                                                                // respectively.  Compute the difference |tdv - srv|, and compare
                                                                                // with 0.5 ulp(srv).
                                                                                delta =
                                                                                    diff(bb, bd);
                                                                                if delta.is_null() {
                                                                                    break '__join_3;
                                                                                } else {
                                                                                    dsign =
                                                                                        (*delta)
                                                                                            .sign;
                                                                                    (*delta).sign =
                                                                                        0;
                                                                                    i = cmp(
                                                                                        delta, bs,
                                                                                    );
                                                                                    if (*__slate_slot_180).nd > nd && i <= 0 {
if dsign != 0 {
break '__join_9;
} else {
'__join_70: {
// Must use bigcomp().
// Here rv overestimates the truncated decimal value by at most
// 0.5 ulp(rv).  Hence rv either overestimates the true decimal
// value by <= 0.5 ulp(rv), or underestimates it by some small
// amount (< 0.1 ulp(rv)); either way, rv is within 0.5 ulps of
// the true decimal value, so it's possible to exit.
//
// Exception: if scaled rv is a normal exact power of 2, but not
// DBL_MIN, then rv - 0.5 ulp(rv) takes us all the way down to the
// next double, so the correctly rounded result is either rv - 0.5
// ulp(rv) or rv; in this case, use bigcomp to distinguish.
if word1!(core::ptr::addr_of_mut!(*__slate_slot_186)) == 0 && word0!(core::ptr::addr_of_mut!(*__slate_slot_186)) & 1048575 == 0 {
// rv can't be 0, since it's an overestimate for some
// nonzero value.  So rv is a normal power of 2.
j = ((word0!(core::ptr::addr_of_mut!(*__slate_slot_186)) & 2146435072) as i32) >> 20i32;
// rv / 2^bc.scale = 2^(j - 1023 - bc.scale); use bigcomp if
// rv / 2^bc.scale >= 2^-1021.
if j.wrapping_sub((*__slate_slot_180).scale) >= 2 {
break '__join_71;
}
} else {
break '__join_70;
}
}
(*__slate_slot_180).nd = nd;
i = 1i32.wrapping_neg(); // Discarded digits make delta smaller.
}
}
                                                                                    if i < 0 {
                                                                                        break '__join_68;
                                                                                    } else {
                                                                                        if i == 0 {
                                                                                            break '__join_62;
                                                                                        } else {
                                                                                            let __v27: f64 = ratio(delta, bs);
                                                                                            aadj = __v27;
                                                                                            if __v27 <= 2.0f64 {
if dsign != 0 {
aadj1 = 1.0f64;
aadj = 1.0f64;
} else {
if word1!(core::ptr::addr_of_mut!(*__slate_slot_186)) != 0 || word0!(core::ptr::addr_of_mut!(*__slate_slot_186)) & 1048575 != 0 {
if word1!(core::ptr::addr_of_mut!(*__slate_slot_186)) == 1 && word0!(core::ptr::addr_of_mut!(*__slate_slot_186)) == 0 {
break;
} else {
aadj = 1.0f64;
aadj1 = -1.0f64;
}
} else {
// special case -- power of FLT_RADIX to be
//
// rounded down...
if aadj < 2.0f64 / (2i32 as f64) {
aadj = 1.0f64 / (2i32 as f64);
} else {
let __v21: f64 = aadj * 0.5f64;
aadj = __v21;
}
aadj1 = -aadj;
}
}
} else {
let __v25: f64 = aadj * 0.5f64;
aadj = __v25;
aadj1 = if dsign != 0 { aadj } else { -aadj };
if __slate_fenv_flt_rounds() == 0 {
let __v23: f64 = aadj1 + 0.5f64;
aadj1 = __v23;
}
}
                                                                                            '__join_10: {
                                                                                                y = word0!(core::ptr::addr_of_mut!(*__slate_slot_186)) & 2146435072;
                                                                                                // Check for overflow
                                                                                                if y == 2145386496u32 {
(*core::ptr::addr_of_mut!(*__slate_slot_185)).d = dval!(core::ptr::addr_of_mut!(*__slate_slot_186));
let __v13: *mut u32 = ((*core::ptr::addr_of_mut!(*__slate_slot_186)).L.as_mut_ptr() as *mut u32).wrapping_offset(1 as isize);
let __v12: u32 = *__v13;
*__v13 = __v12.wrapping_sub(55574528u32);
(*__slate_slot_187).d = aadj1 * ulp(core::ptr::addr_of_mut!(*__slate_slot_186));
let __v10: *mut __SlateRecord_dtoa_c_181 = core::ptr::addr_of_mut!(*__slate_slot_186);
(*__v10).d = (*__v10).d + (*__slate_slot_187).d;
if word0!(core::ptr::addr_of_mut!(*__slate_slot_186)) & 2146435072 >= 2090860544u32 {
if word0!(core::ptr::addr_of_mut!(*__slate_slot_185)) == 2146435071u32 && word1!(core::ptr::addr_of_mut!(*__slate_slot_185)) == 4294967295 {
break '__join_1;
} else {
*((*core::ptr::addr_of_mut!(*__slate_slot_186)).L.as_mut_ptr() as *mut u32).wrapping_offset(1 as isize) = 2146435071u32;
*((*core::ptr::addr_of_mut!(*__slate_slot_186)).L.as_mut_ptr() as *mut u32).wrapping_offset(0 as isize) = 4294967295;
break '__join_10;
}
} else {
let __v7: *mut u32 = ((*core::ptr::addr_of_mut!(*__slate_slot_186)).L.as_mut_ptr() as *mut u32).wrapping_offset(1 as isize);
let __v6: u32 = *__v7;
*__v7 = __v6.wrapping_add(55574528u32);
}
} else {
if (*__slate_slot_180).scale != 0 && y <= 111149056u32 {
if aadj <= (2147483647i32 as f64) {
let __v20: u32 = aadj as u32;
z = __v20;
if __v20 <= 0 {
z = 1;
}
aadj = z as f64;
aadj1 = if dsign != 0 { aadj } else { -aadj };
}
(*core::ptr::addr_of_mut!(*__slate_slot_188)).d = aadj1;
let __v19: *mut u32 = ((*core::ptr::addr_of_mut!(*__slate_slot_188)).L.as_mut_ptr() as *mut u32).wrapping_offset(1 as isize);
let __v18: u32 = *__v19;
*__v19 = __v18.wrapping_add(112197632u32.wrapping_sub(y));
aadj1 = dval!(core::ptr::addr_of_mut!(*__slate_slot_188));
}
(*__slate_slot_187).d = aadj1 * ulp(core::ptr::addr_of_mut!(*__slate_slot_186));
let __v16: *mut __SlateRecord_dtoa_c_181 = core::ptr::addr_of_mut!(*__slate_slot_186);
(*__v16).d = (*__v16).d + (*__slate_slot_187).d;
}
                                                                                                z = word0!(core::ptr::addr_of_mut!(*__slate_slot_186)) & 2146435072;
                                                                                                if (*__slate_slot_180).nd == nd {
if (*__slate_slot_180).scale == 0 {
if y == z {
// Can we stop now?
L = aadj as i32;
aadj -= L as f64;
// The tolerances below are conservative.
if dsign != 0 || word1!(core::ptr::addr_of_mut!(*__slate_slot_186)) != 0 || word0!(core::ptr::addr_of_mut!(*__slate_slot_186)) & 1048575 != 0 {
if aadj < 0.4999999f64 || aadj > 0.5000001f64 {
break '__join_9;
}
} else {
if aadj < 0.4999999f64 / (2i32 as f64) {
break '__join_9;
}
}
}
}
}
                                                                                            }
                                                                                            Bfree(
                                                                                                bb,
                                                                                            );
                                                                                            bb = core::ptr::null_mut::<Bigint>();
                                                                                            Bfree(
                                                                                                bd,
                                                                                            );
                                                                                            bd = core::ptr::null_mut::<Bigint>();
                                                                                            Bfree(
                                                                                                bs,
                                                                                            );
                                                                                            bs = core::ptr::null_mut::<Bigint>();
                                                                                            Bfree(delta);
                                                                                            delta = core::ptr::null_mut::<Bigint>();
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                if (*__slate_slot_180).nd > nd {
                                                                    break '__join_9;
                                                                } else {
                                                                    break '__join_2;
                                                                }
                                                            }
                                                            // exactly half-way between
                                                            if dsign != 0 {
                                                                if word0!(core::ptr::addr_of_mut!(
                                                                    *__slate_slot_186
                                                                )) & 1048575
                                                                    == 1048575
                                                                {
                                                                    if (*__slate_slot_180).scale
                                                                        != 0
                                                                    {
                                                                        let __v36: u32 = word0!(
                                                                            core::ptr::addr_of_mut!(
                                                                                *__slate_slot_186
                                                                            )
                                                                        )
                                                                            & 2146435072;
                                                                        y = __v36;
                                                                        __v37 =
                                                                            __v36 <= 111149056u32;
                                                                    } else {
                                                                        __v37 = false;
                                                                    }
                                                                    __v38 = word1!(
                                                                        core::ptr::addr_of_mut!(
                                                                            *__slate_slot_186
                                                                        )
                                                                    ) == if __v37 {
                                                                        4294967295
                                                                            & 4294967295u32
                                                                                << 107u32
                                                                                    .wrapping_sub(
                                                                                        y >> 20i32,
                                                                                    )
                                                                    } else {
                                                                        4294967295
                                                                    };
                                                                } else {
                                                                    __v38 = false;
                                                                }
                                                                if __v38 {
                                                                    // boundary case -- increment exponent
                                                                    *((*core::ptr::addr_of_mut!(
                                                                        *__slate_slot_186
                                                                    ))
                                                                    .L
                                                                    .as_mut_ptr()
                                                                        as *mut u32)
                                                                        .wrapping_offset(
                                                                            1 as isize,
                                                                        ) = (word0!(
                                                                        core::ptr::addr_of_mut!(
                                                                            *__slate_slot_186
                                                                        )
                                                                    ) & 2146435072)
                                                                        .wrapping_add(1048576);
                                                                    *((*core::ptr::addr_of_mut!(
                                                                        *__slate_slot_186
                                                                    ))
                                                                    .L
                                                                    .as_mut_ptr()
                                                                        as *mut u32)
                                                                        .wrapping_offset(
                                                                            0 as isize,
                                                                        ) = 0;
                                                                    // dsign = 0;
                                                                    break '__join_9;
                                                                }
                                                            } else {
                                                                if word0!(core::ptr::addr_of_mut!(
                                                                    *__slate_slot_186
                                                                )) & 1048575
                                                                    == 0
                                                                    && word1!(
                                                                        core::ptr::addr_of_mut!(
                                                                            *__slate_slot_186
                                                                        )
                                                                    ) == 0
                                                                {
                                                                    break '__join_60;
                                                                }
                                                            }
                                                            if odd == 0 {
                                                                break '__join_9;
                                                            } else {
                                                                if dsign != 0 {
                                                                    let __v31: f64 = sulp(
                                                                        core::ptr::addr_of_mut!(
                                                                            *__slate_slot_186
                                                                        ),
                                                                        &(*core::ptr::addr_of_mut!(
                                                                            *__slate_slot_180
                                                                        )),
                                                                    );
                                                                    let __v30: *mut __SlateRecord_dtoa_c_181 = core::ptr::addr_of_mut!(*__slate_slot_186);
                                                                    (*__v30).d = (*__v30).d + __v31;
                                                                } else {
                                                                    let __v35: f64 = sulp(
                                                                        core::ptr::addr_of_mut!(
                                                                            *__slate_slot_186
                                                                        ),
                                                                        &(*core::ptr::addr_of_mut!(
                                                                            *__slate_slot_180
                                                                        )),
                                                                    );
                                                                    let __v34: *mut __SlateRecord_dtoa_c_181 = core::ptr::addr_of_mut!(*__slate_slot_186);
                                                                    (*__v34).d = (*__v34).d - __v35;
                                                                    if dval!(
                                                                        core::ptr::addr_of_mut!(
                                                                            *__slate_slot_186
                                                                        )
                                                                    ) == 0.0f64
                                                                    {
                                                                        if (*__slate_slot_180).nd
                                                                            > nd
                                                                        {
                                                                            break '__join_9;
                                                                        } else {
                                                                            break '__join_2;
                                                                        }
                                                                    }
                                                                }
                                                                // dsign = 1 - dsign;
                                                                break '__join_9;
                                                            }
                                                        }
                                                        // Error is less than half an ulp -- check for
                                                        // special case of mantissa a power of two.
                                                        if dsign != 0
                                                            || word1!(core::ptr::addr_of_mut!(
                                                                *__slate_slot_186
                                                            )) != 0
                                                            || word0!(core::ptr::addr_of_mut!(
                                                                *__slate_slot_186
                                                            )) & 1048575
                                                                != 0
                                                            || word0!(core::ptr::addr_of_mut!(
                                                                *__slate_slot_186
                                                            )) & 2146435072
                                                                <= 112197632u32
                                                        {
                                                            break '__join_9;
                                                        } else {
                                                            if *((*delta).x.as_mut_ptr()
                                                                as *mut u32)
                                                                .wrapping_offset(0 as isize)
                                                                == 0
                                                                && (*delta).wds <= 1
                                                            {
                                                                // exact result
                                                                break '__join_9;
                                                            } else {
                                                                delta = lshift(delta, 1i32);
                                                                if delta.is_null() {
                                                                    break '__join_3;
                                                                } else {
                                                                    if cmp(delta, bs) <= 0 {
                                                                        break '__join_9;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    if (*__slate_slot_180).scale != 0 {
                                                        L = (word0!(core::ptr::addr_of_mut!(
                                                            *__slate_slot_186
                                                        )) & 2146435072)
                                                            as i32;
                                                        if L <= 112197632i32 {
                                                            if L > 57671680i32 {
                                                                break '__join_9;
                                                            } else {
                                                                // round even ==>
                                                                //
                                                                // accept rv
                                                                // rv = smallest denormal
                                                                if (*__slate_slot_180).nd > nd {
                                                                    break '__join_9;
                                                                } else {
                                                                    break '__join_2;
                                                                }
                                                            }
                                                        }
                                                    }
                                                    // boundary case -- decrement exponent
                                                    L = (word0!(core::ptr::addr_of_mut!(
                                                        *__slate_slot_186
                                                    )) & 2146435072)
                                                        .wrapping_sub(1048576)
                                                        as i32;
                                                    *((*core::ptr::addr_of_mut!(*__slate_slot_186)).L.as_mut_ptr() as *mut u32).wrapping_offset(1 as isize) = (L | 1048575) as u32;
                                                    *((*core::ptr::addr_of_mut!(*__slate_slot_186)).L.as_mut_ptr() as *mut u32).wrapping_offset(0 as isize) = 4294967295;
                                                    break '__join_9;
                                                }
                                                let __v42: f64 = 0.5f64
                                                    * sulp(
                                                        core::ptr::addr_of_mut!(*__slate_slot_186),
                                                        &(*core::ptr::addr_of_mut!(
                                                            *__slate_slot_180
                                                        )),
                                                    );
                                                let __v41: *mut __SlateRecord_dtoa_c_181 =
                                                    core::ptr::addr_of_mut!(*__slate_slot_186);
                                                (*__v41).d = (*__v41).d - __v42;
                                            }
                                            if (*__slate_slot_180).nd > nd {
                                                let error: i32 = bigcomp(
                                                    core::ptr::addr_of_mut!(*__slate_slot_186),
                                                    s0,
                                                    &(*core::ptr::addr_of_mut!(*__slate_slot_180)),
                                                );
                                                if error != 0 {
                                                    break '__join_3;
                                                }
                                            }
                                            if (*__slate_slot_180).scale != 0 {
                                                *((*core::ptr::addr_of_mut!(*__slate_slot_185))
                                                    .L
                                                    .as_mut_ptr()
                                                    as *mut u32)
                                                    .wrapping_offset(1 as isize) = 961544192u32;
                                                *((*core::ptr::addr_of_mut!(*__slate_slot_185))
                                                    .L
                                                    .as_mut_ptr()
                                                    as *mut u32)
                                                    .wrapping_offset(0 as isize) = 0;
                                                let __v2: *mut __SlateRecord_dtoa_c_181 =
                                                    core::ptr::addr_of_mut!(*__slate_slot_186);
                                                let __v1: f64 = (*__v2).d;
                                                let __v0: f64 = __v1
                                                    * dval!(core::ptr::addr_of_mut!(
                                                        *__slate_slot_185
                                                    ));
                                                (*__v2).d = __v0;
                                                break '__join_5;
                                            } else {
                                                break '__join_5;
                                            }
                                        }
                                    }
                                    *slate_support::errno_location() = slate_support::ENOMEM as i32;
                                    result = -1.0f64;
                                    break '__join_0;
                                }
                                result = if sign != 0 { -0.0f64 } else { 0.0f64 };
                                break '__join_0;
                            }
                            *slate_support::errno_location() = slate_support::ERANGE as i32;
                            // Can't trust HUGE_VAL
                            *((*core::ptr::addr_of_mut!(*__slate_slot_186)).L.as_mut_ptr()
                                as *mut u32)
                                .wrapping_offset(1 as isize) = 2146435072;
                            *((*core::ptr::addr_of_mut!(*__slate_slot_186)).L.as_mut_ptr()
                                as *mut u32)
                                .wrapping_offset(0 as isize) = 0;
                            result = if sign != 0 {
                                -dval!(core::ptr::addr_of_mut!(*__slate_slot_186))
                            } else {
                                dval!(core::ptr::addr_of_mut!(*__slate_slot_186))
                            };
                            break '__join_0;
                        }
                    }
                    result = if sign != 0 {
                        -dval!(core::ptr::addr_of_mut!(*__slate_slot_186))
                    } else {
                        dval!(core::ptr::addr_of_mut!(*__slate_slot_186))
                    };
                    break '__join_0;
                }
            }
            result = 0.0f64;
        }
        Bfree(bb);
        Bfree(bd);
        Bfree(bs);
        Bfree(bd0);
        Bfree(delta);
        return result;
    }
    // Use bigcomp.
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_lock_h_69 {
    v: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_lock_h_153 {
    mutex: PyMutex,
    thread: u64,
    level: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct ast_state {
    once: __SlateRecord_modsupport_h_9,
    finalized: i32,
    AST_type: *mut _object,
    Add_singleton: *mut _object,
    Add_type: *mut _object,
    And_singleton: *mut _object,
    And_type: *mut _object,
    AnnAssign_type: *mut _object,
    Assert_type: *mut _object,
    Assign_type: *mut _object,
    AsyncFor_type: *mut _object,
    AsyncFunctionDef_type: *mut _object,
    AsyncWith_type: *mut _object,
    Attribute_type: *mut _object,
    AugAssign_type: *mut _object,
    Await_type: *mut _object,
    BinOp_type: *mut _object,
    BitAnd_singleton: *mut _object,
    BitAnd_type: *mut _object,
    BitOr_singleton: *mut _object,
    BitOr_type: *mut _object,
    BitXor_singleton: *mut _object,
    BitXor_type: *mut _object,
    BoolOp_type: *mut _object,
    Break_type: *mut _object,
    Call_type: *mut _object,
    ClassDef_type: *mut _object,
    Compare_type: *mut _object,
    Constant_type: *mut _object,
    Continue_type: *mut _object,
    Del_singleton: *mut _object,
    Del_type: *mut _object,
    Delete_type: *mut _object,
    DictComp_type: *mut _object,
    Dict_type: *mut _object,
    Div_singleton: *mut _object,
    Div_type: *mut _object,
    Eq_singleton: *mut _object,
    Eq_type: *mut _object,
    ExceptHandler_type: *mut _object,
    Expr_type: *mut _object,
    Expression_type: *mut _object,
    FloorDiv_singleton: *mut _object,
    FloorDiv_type: *mut _object,
    For_type: *mut _object,
    FormattedValue_type: *mut _object,
    FunctionDef_type: *mut _object,
    FunctionType_type: *mut _object,
    GeneratorExp_type: *mut _object,
    Global_type: *mut _object,
    GtE_singleton: *mut _object,
    GtE_type: *mut _object,
    Gt_singleton: *mut _object,
    Gt_type: *mut _object,
    IfExp_type: *mut _object,
    If_type: *mut _object,
    ImportFrom_type: *mut _object,
    Import_type: *mut _object,
    In_singleton: *mut _object,
    In_type: *mut _object,
    Interactive_type: *mut _object,
    Interpolation_type: *mut _object,
    Invert_singleton: *mut _object,
    Invert_type: *mut _object,
    IsNot_singleton: *mut _object,
    IsNot_type: *mut _object,
    Is_singleton: *mut _object,
    Is_type: *mut _object,
    JoinedStr_type: *mut _object,
    LShift_singleton: *mut _object,
    LShift_type: *mut _object,
    Lambda_type: *mut _object,
    ListComp_type: *mut _object,
    List_type: *mut _object,
    Load_singleton: *mut _object,
    Load_type: *mut _object,
    LtE_singleton: *mut _object,
    LtE_type: *mut _object,
    Lt_singleton: *mut _object,
    Lt_type: *mut _object,
    MatMult_singleton: *mut _object,
    MatMult_type: *mut _object,
    MatchAs_type: *mut _object,
    MatchClass_type: *mut _object,
    MatchMapping_type: *mut _object,
    MatchOr_type: *mut _object,
    MatchSequence_type: *mut _object,
    MatchSingleton_type: *mut _object,
    MatchStar_type: *mut _object,
    MatchValue_type: *mut _object,
    Match_type: *mut _object,
    Mod_singleton: *mut _object,
    Mod_type: *mut _object,
    Module_type: *mut _object,
    Mult_singleton: *mut _object,
    Mult_type: *mut _object,
    Name_type: *mut _object,
    NamedExpr_type: *mut _object,
    Nonlocal_type: *mut _object,
    NotEq_singleton: *mut _object,
    NotEq_type: *mut _object,
    NotIn_singleton: *mut _object,
    NotIn_type: *mut _object,
    Not_singleton: *mut _object,
    Not_type: *mut _object,
    Or_singleton: *mut _object,
    Or_type: *mut _object,
    ParamSpec_type: *mut _object,
    Pass_type: *mut _object,
    Pow_singleton: *mut _object,
    Pow_type: *mut _object,
    RShift_singleton: *mut _object,
    RShift_type: *mut _object,
    Raise_type: *mut _object,
    Return_type: *mut _object,
    SetComp_type: *mut _object,
    Set_type: *mut _object,
    Slice_type: *mut _object,
    Starred_type: *mut _object,
    Store_singleton: *mut _object,
    Store_type: *mut _object,
    Sub_singleton: *mut _object,
    Sub_type: *mut _object,
    Subscript_type: *mut _object,
    TemplateStr_type: *mut _object,
    TryStar_type: *mut _object,
    Try_type: *mut _object,
    Tuple_type: *mut _object,
    TypeAlias_type: *mut _object,
    TypeIgnore_type: *mut _object,
    TypeVarTuple_type: *mut _object,
    TypeVar_type: *mut _object,
    UAdd_singleton: *mut _object,
    UAdd_type: *mut _object,
    USub_singleton: *mut _object,
    USub_type: *mut _object,
    UnaryOp_type: *mut _object,
    While_type: *mut _object,
    With_type: *mut _object,
    YieldFrom_type: *mut _object,
    Yield_type: *mut _object,
    __dict__: *mut _object,
    __doc__: *mut _object,
    __match_args__: *mut _object,
    __module__: *mut _object,
    _attributes: *mut _object,
    _fields: *mut _object,
    alias_type: *mut _object,
    annotation: *mut _object,
    arg: *mut _object,
    arg_type: *mut _object,
    args: *mut _object,
    argtypes: *mut _object,
    arguments_type: *mut _object,
    asname: *mut _object,
    ast: *mut _object,
    attr: *mut _object,
    bases: *mut _object,
    body: *mut _object,
    boolop_type: *mut _object,
    bound: *mut _object,
    cases: *mut _object,
    cause: *mut _object,
    cls: *mut _object,
    cmpop_type: *mut _object,
    col_offset: *mut _object,
    comparators: *mut _object,
    comprehension_type: *mut _object,
    context_expr: *mut _object,
    conversion: *mut _object,
    ctx: *mut _object,
    decorator_list: *mut _object,
    default_value: *mut _object,
    defaults: *mut _object,
    elt: *mut _object,
    elts: *mut _object,
    end_col_offset: *mut _object,
    end_lineno: *mut _object,
    exc: *mut _object,
    excepthandler_type: *mut _object,
    expr_context_type: *mut _object,
    expr_type: *mut _object,
    finalbody: *mut _object,
    format_spec: *mut _object,
    func: *mut _object,
    generators: *mut _object,
    guard: *mut _object,
    handlers: *mut _object,
    id: *mut _object,
    ifs: *mut _object,
    is_async: *mut _object,
    items: *mut _object,
    iter: *mut _object,
    key: *mut _object,
    keys: *mut _object,
    keyword_type: *mut _object,
    keywords: *mut _object,
    kind: *mut _object,
    kw_defaults: *mut _object,
    kwarg: *mut _object,
    kwd_attrs: *mut _object,
    kwd_patterns: *mut _object,
    kwonlyargs: *mut _object,
    left: *mut _object,
    level: *mut _object,
    lineno: *mut _object,
    lower: *mut _object,
    match_case_type: *mut _object,
    mod_type: *mut _object,
    module: *mut _object,
    msg: *mut _object,
    name: *mut _object,
    names: *mut _object,
    op: *mut _object,
    operand: *mut _object,
    operator_type: *mut _object,
    ops: *mut _object,
    optional_vars: *mut _object,
    orelse: *mut _object,
    pattern: *mut _object,
    pattern_type: *mut _object,
    patterns: *mut _object,
    posonlyargs: *mut _object,
    rest: *mut _object,
    returns: *mut _object,
    right: *mut _object,
    simple: *mut _object,
    slice: *mut _object,
    step: *mut _object,
    stmt_type: *mut _object,
    str: *mut _object,
    subject: *mut _object,
    tag: *mut _object,
    target: *mut _object,
    targets: *mut _object,
    test: *mut _object,
    r#type: *mut _object,
    type_comment: *mut _object,
    type_ignore_type: *mut _object,
    type_ignores: *mut _object,
    type_param_type: *mut _object,
    type_params: *mut _object,
    unaryop_type: *mut _object,
    upper: *mut _object,
    value: *mut _object,
    values: *mut _object,
    vararg: *mut _object,
    withitem_type: *mut _object,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct llist_node {
    next: *mut llist_node,
    prev: *mut llist_node,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_structs_h_38 {
    ob_base: _object,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_structs_h_47 {
    ob_base: _object,
    h_root: *mut __SlateRecord_pycore_structs_h_38,
    h_weakreflist: *mut _object,
    h_count: i64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _Py_freelist {
    freelist: *mut (),
    size: i64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _Py_freelists {
    floats: _Py_freelist,
    ints: _Py_freelist,
    tuples: [_Py_freelist; 20],
    lists: _Py_freelist,
    list_iters: _Py_freelist,
    tuple_iters: _Py_freelist,
    dicts: _Py_freelist,
    dictkeys: _Py_freelist,
    slices: _Py_freelist,
    ranges: _Py_freelist,
    range_iters: _Py_freelist,
    contexts: _Py_freelist,
    async_gens: _Py_freelist,
    async_gen_asends: _Py_freelist,
    futureiters: _Py_freelist,
    object_stack_chunks: _Py_freelist,
    unicode_writers: _Py_freelist,
    pycfunctionobject: _Py_freelist,
    pycmethodobject: _Py_freelist,
    pymethodobjects: _Py_freelist,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _qsbr_thread_state {
    seq: u64,
    shared: *mut _qsbr_shared,
    tstate: *mut _ts,
    deferred_count: i32,
    deferred_memory: u64,
    deferred_page_memory: u64,
    should_process: bool,
    allocated: bool,
    freelist_next: *mut _qsbr_thread_state,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _qsbr_pad {
    qsbr: _qsbr_thread_state,
    __padding: [i8; 0],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _qsbr_shared {
    wr_seq: u64,
    rd_seq: u64,
    array: *mut _qsbr_pad,
    array_raw: *mut (),
    size: i64,
    mutex: PyMutex,
    freelist: *mut _qsbr_thread_state,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _PyThreadStateImpl {
    base: _ts,
    refcount: i64,
    c_stack_top: u64,
    c_stack_soft_limit: u64,
    c_stack_hard_limit: u64,
    asyncio_running_loop: *mut _object,
    asyncio_running_task: *mut _object,
    asyncio_tasks_head: llist_node,
    qsbr: *mut _qsbr_thread_state,
    mem_free_queue: llist_node,
    c_stack_init_base: u64,
    c_stack_init_top: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _pending_call {
    func: Option<unsafe extern "C-unwind" fn(*mut ()) -> i32>,
    arg: *mut (),
    flags: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct pyruntimestate {}

#[repr(C)]
#[derive(Clone, Copy)]
struct _pending_calls {
    handling_thread: *mut _ts,
    mutex: PyMutex,
    npending: i32,
    max: i32,
    maxloop: i32,
    calls: [_pending_call; 300],
    first: i32,
    next: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _ceval_state {
    instrumentation_version: u64,
    recursion_limit: i32,
    gil: *mut _gil_runtime_state,
    own_gil: i32,
    pending: _pending_calls,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct atexit_callback {
    func: Option<unsafe extern "C-unwind" fn(*mut ())>,
    data: *mut (),
    next: *mut atexit_callback,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct atexit_state {
    ll_callbacks: *mut atexit_callback,
    callbacks: *mut _object,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_interp_structs_h_170 {
    _gc_next: u64,
    _gc_prev: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct gc_generation {
    head: __SlateRecord_pycore_interp_structs_h_170,
    threshold: i32,
    count: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct gc_generation_stats {
    collections: i64,
    collected: i64,
    uncollectable: i64,
}

// If we change this, we need to change the default value in the
// signature of gc.collect.
#[repr(C)]
#[derive(Clone, Copy)]
struct _gc_runtime_state {
    trash_delete_later: *mut _object,
    trash_delete_nesting: i32,
    enabled: i32,
    debug: i32,
    generations: [gc_generation; 3],
    permanent_generation: gc_generation,
    generation_stats: [gc_generation_stats; 3],
    collecting: i32,
    garbage: *mut _object,
    callbacks: *mut _object,
    heap_size: i64,
    dummy1: i64,
    dummy2: i32,
    dummy3: i32,
    long_lived_total: i64,
    long_lived_pending: i64,
    generation0: *mut __SlateRecord_pycore_interp_structs_h_170,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _gil_runtime_state {
    interval: u64,
    last_holder: *mut _ts,
    locked: i32,
    switch_number: u64,
    cond: __SlateRecord_pthreadtypes_h_80,
    mutex: __SlateRecord_pthreadtypes_h_72,
    switch_cond: __SlateRecord_pthreadtypes_h_80,
    switch_mutex: __SlateRecord_pthreadtypes_h_72,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _import_state {
    modules: *mut _object,
    modules_by_index: *mut _object,
    importlib: *mut _object,
    override_frozen_modules: i32,
    override_multi_interp_extensions_check: i32,
    dlopenflags: i32,
    import_func: *mut _object,
    lock: __SlateRecord_pycore_lock_h_153,
    find_and_load: __SlateRecord_pycore_interp_structs_h_348,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _Py_slist_item_s {
    next: *mut _Py_slist_item_s,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_hashtable_h_19 {
    head: *mut _Py_slist_item_s,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_interp_structs_h_348 {
    import_level: i32,
    accumulated: i64,
    header: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_hashtable_h_35 {
    _Py_slist_item: _Py_slist_item_s,
    key_hash: u64,
    key: *mut (),
    value: *mut (),
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_hashtable_h_56 {
    malloc: Option<unsafe extern "C-unwind" fn(u64) -> *mut ()>,
    free: Option<unsafe extern "C-unwind" fn(*mut ())>,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _Py_hashtable_t {
    nentries: u64,
    nbuckets: u64,
    buckets: *mut __SlateRecord_pycore_hashtable_h_19,
    get_entry_func: Option<
        unsafe extern "C-unwind" fn(
            *mut _Py_hashtable_t,
            *const (),
        ) -> *mut __SlateRecord_pycore_hashtable_h_35,
    >,
    hash_func: Option<unsafe extern "C-unwind" fn(*const ()) -> u64>,
    compare_func: Option<unsafe extern "C-unwind" fn(*const (), *const ()) -> i32>,
    key_destroy_func: Option<unsafe extern "C-unwind" fn(*mut ())>,
    value_destroy_func: Option<unsafe extern "C-unwind" fn(*mut ())>,
    alloc: __SlateRecord_pycore_hashtable_h_56,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _py_object_state {
    freelists: _Py_freelists,
    _not_used: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _xidata {
    data: *mut (),
    obj: *mut _object,
    interpid: i64,
    new_object: Option<unsafe extern "C-unwind" fn(*mut _xidata) -> *mut _object>,
    free: Option<unsafe extern "C-unwind" fn(*mut ())>,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_crossinterp_h_150 {
    basic: Option<unsafe extern "C-unwind" fn(*mut _ts, *mut _object, *mut _xidata) -> i32>,
    fallback: Option<unsafe extern "C-unwind" fn(*mut _ts, *mut _object, i32, *mut _xidata) -> i32>,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _xid_regitem {
    prev: *mut _xid_regitem,
    next: *mut _xid_regitem,
    cls: *mut _typeobject,
    weakref: *mut _object,
    refcount: u64,
    getdata: __SlateRecord_pycore_crossinterp_h_150,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_crossinterp_data_registry_h_28 {
    global: i32,
    initialized: i32,
    mutex: PyMutex,
    head: *mut _xid_regitem,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _xid_lookup_state {
    registry: __SlateRecord_pycore_crossinterp_data_registry_h_28,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_crossinterp_h_268 {
    data_lookup: _xid_lookup_state,
    exceptions: xi_exceptions,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _Py_long_state {
    max_str_digits: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct xi_exceptions {
    PyExc_InterpreterError: *mut _object,
    PyExc_InterpreterNotFoundError: *mut _object,
    PyExc_NotShareableError: *mut _object,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct codecs_state {
    search_path: *mut _object,
    search_cache: *mut _object,
    error_registry: *mut _object,
    initialized: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _stoptheworld_state {
    mutex: PyMutex,
    requested: bool,
    world_stopped: bool,
    is_global: bool,
    stop_event: __SlateRecord_pycore_lock_h_69,
    thread_countdown: i64,
    requester: *mut _ts,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _rare_events {
    set_class: u8,
    set_bases: u8,
    set_eval_frame_func: u8,
    builtin_dict: u8,
    func_modification: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Bigint {
    next: *mut Bigint,
    k: i32,
    maxwds: i32,
    sign: i32,
    wds: i32,
    x: [u32; 1],
}

// The size of the Bigint freelist
// The size of the cached powers of 5 array
#[repr(C)]
#[derive(Clone, Copy)]
struct _dtoa_state {
    p5s: [*mut Bigint; 8],
    freelist: [*mut Bigint; 8],
    preallocated: [f64; 288],
    preallocated_next: *mut f64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _py_code_state {
    mutex: PyMutex,
    constants: *mut _Py_hashtable_t,
}

// Must be a power of 2
#[repr(C)]
#[derive(Clone, Copy)]
struct _func_version_cache_item {
    func: *mut __SlateRecord_funcobject_h_62,
    code: *mut _object,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _py_func_state {
    next_version: u32,
    func_version_cache: [_func_version_cache_item; 4096],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _Py_dict_state {
    next_keys_version: u32,
    watcher_mutex: PyMutex,
    watcher_setup_once: __SlateRecord_modsupport_h_9,
    watchers: [Option<
        unsafe extern "C-unwind" fn(u32, *mut _object, *mut _object, *mut _object) -> i32,
    >; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _Py_exc_state {
    errnomap: *mut _object,
    memerrors_freelist: *mut __SlateRecord_pyerrors_h_15,
    memerrors_numfree: i32,
    PyExc_ExceptionGroup: *mut _object,
}

// For now we hard-code this to a value for which we are confident
// all the static builtin types will fit (for all builds).
#[repr(C)]
#[derive(Clone, Copy)]
struct type_cache_entry {
    version: u32,
    name: *mut _object,
    value: *mut _object,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct type_cache {
    hashtable: [type_cache_entry; 4096],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_interp_structs_h_544 {
    r#type: *mut _typeobject,
    isbuiltin: i32,
    readying: i32,
    ready: i32,
    tp_dict: *mut _object,
    tp_subclasses: *mut _object,
    tp_weaklist: *mut _object,
}

// Must be a power of 2
#[repr(C)]
#[derive(Clone, Copy)]
struct types_state {
    next_version_tag: u32,
    type_cache: type_cache,
    builtins: __SlateRecord_pycore_interp_structs_h_592,
    for_extensions: __SlateRecord_pycore_interp_structs_h_597,
    mutex: PyMutex,
    type_version_cache: [*mut _typeobject; 4096],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _warnings_runtime_state {
    filters: *mut _object,
    once_registry: *mut _object,
    default_action: *mut _object,
    lock: __SlateRecord_pycore_lock_h_153,
    filters_version: i64,
    context: *mut _object,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_interp_structs_h_592 {
    num_initialized: u64,
    initialized: [__SlateRecord_pycore_interp_structs_h_544; 200],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_interp_structs_h_597 {
    num_initialized: u64,
    next_index: u64,
    initialized: [__SlateRecord_pycore_interp_structs_h_544; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _Py_mem_interp_free_queue {
    has_work: i32,
    mutex: PyMutex,
    head: llist_node,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _Py_unicode_fs_codec {
    encoding: *mut i8,
    utf8: i32,
    errors: *mut i8,
    error_handler: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _Py_unicode_ids {
    size: i64,
    array: *mut *mut _object,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ucnhash_h_29 {
    getname: Option<unsafe extern "C-unwind" fn(u32, *mut i8, i32, i32) -> i32>,
    getcode: Option<unsafe extern "C-unwind" fn(*const i8, i32, *mut u32, i32) -> i32>,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _Py_unicode_state {
    fs_codec: _Py_unicode_fs_codec,
    ucnhash_capi: *mut __SlateRecord_pycore_ucnhash_h_29,
    ids: _Py_unicode_ids,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct callable_cache {
    isinstance: *mut _object,
    len: *mut _object,
    list_append: *mut _object,
    object__getattribute__: *mut _object,
}

// Length of array of slotdef pointers used to store slots with the
// same __name__.  There should be at most MAX_EQUIV-1 slotdef entries with
// the same __name__, for any __name__. Since that's a static property, it is
// appropriate to declare fixed-size arrays for this.
#[repr(C)]
#[derive(Clone, Copy)]
struct _Py_interp_cached_objects {
    interned_strings: *mut _object,
    objreduce: *mut _object,
    type_slots_pname: *mut _object,
    type_slots_ptrs: [*mut wrapperbase; 10],
    generic_type: *mut _typeobject,
    typevar_type: *mut _typeobject,
    typevartuple_type: *mut _typeobject,
    paramspec_type: *mut _typeobject,
    paramspecargs_type: *mut _typeobject,
    paramspeckwargs_type: *mut _typeobject,
    constevaluator_type: *mut _typeobject,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _Py_interp_static_objects {
    singletons: __SlateRecord_pycore_interp_structs_h_718,
}

// Count of all local monitoring events
// Count of all "real" monitoring events (not derived from other events)
// Count of all  monitoring events
#[repr(C)]
#[derive(Clone, Copy)]
struct _Py_LocalMonitors {
    tools: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_interp_structs_h_718 {
    _not_used: i32,
    _hamt_empty_gc_not_used: __SlateRecord_pycore_interp_structs_h_170,
    hamt_empty: __SlateRecord_pycore_structs_h_47,
    last_resort_memory_error: __SlateRecord_pyerrors_h_15,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _Py_GlobalMonitors {
    tools: [u8; 16],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_instruments_h_94 {
    bytes_per_entry: u8,
    data: [u8; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _PyCoMonitoringData {
    local_monitors: _Py_LocalMonitors,
    active_monitors: _Py_LocalMonitors,
    tools: *mut u8,
    tool_versions: [u64; 8],
    lines: *mut __SlateRecord_pycore_instruments_h_94,
    line_tools: *mut u8,
    per_instruction_opcodes: *mut u8,
    per_instruction_tools: *mut u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct pythreads {
    next_unique_id: u64,
    head: *mut _ts,
    preallocated: *mut _PyThreadStateImpl,
    main: *mut _ts,
    count: i64,
    stacksize: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _obmalloc_state {}

#[repr(C)]
#[derive(Clone, Copy)]
struct _is {
    ceval: _ceval_state,
    _malloced: *mut (),
    next: *mut _is,
    id: i64,
    id_refcount: i64,
    requires_idref: i32,
    _whence: i64,
    _initialized: i32,
    _ready: i32,
    finalizing: i32,
    last_restart_version: u64,
    threads: pythreads,
    runtime: *mut pyruntimestate,
    _finalizing: *mut _ts,
    _finalizing_id: u64,
    gc: _gc_runtime_state,
    sysdict: *mut _object,
    builtins: *mut _object,
    imports: _import_state,
    _gil: _gil_runtime_state,
    _code_object_generation: u64,
    codecs: codecs_state,
    config: PyConfig,
    feature_flags: u64,
    dict: *mut _object,
    sysdict_copy: *mut _object,
    builtins_copy: *mut _object,
    eval_frame: Option<
        unsafe extern "C-unwind" fn(*mut _ts, *mut _PyInterpreterFrame, i32) -> *mut _object,
    >,
    func_watchers: [Option<
        unsafe extern "C-unwind" fn(u32, *mut __SlateRecord_funcobject_h_62, *mut _object) -> i32,
    >; 8],
    active_func_watchers: u8,
    co_extra_user_count: i64,
    co_extra_freefuncs: [Option<unsafe extern "C-unwind" fn(*mut ())>; 255],
    xi: __SlateRecord_pycore_crossinterp_h_268,
    before_forkers: *mut _object,
    after_forkers_parent: *mut _object,
    after_forkers_child: *mut _object,
    warnings: _warnings_runtime_state,
    atexit: atexit_state,
    stoptheworld: _stoptheworld_state,
    qsbr: _qsbr_shared,
    asyncio_tasks_head: llist_node,
    asyncio_tasks_lock: PyMutex,
    obmalloc: *mut _obmalloc_state,
    audit_hooks: *mut _object,
    type_watchers: [Option<unsafe extern "C-unwind" fn(*mut _typeobject) -> i32>; 8],
    code_watchers: [Option<unsafe extern "C-unwind" fn(u32, *mut PyCodeObject) -> i32>; 8],
    context_watchers: [Option<unsafe extern "C-unwind" fn(u32, *mut _object) -> i32>; 8],
    active_code_watchers: u8,
    active_context_watchers: u8,
    object_state: _py_object_state,
    unicode: _Py_unicode_state,
    long_state: _Py_long_state,
    dtoa: _dtoa_state,
    func_state: _py_func_state,
    code_state: _py_code_state,
    dict_state: _Py_dict_state,
    exc_state: _Py_exc_state,
    mem_free_queue: _Py_mem_interp_free_queue,
    ast: ast_state,
    types: types_state,
    callable_cache: callable_cache,
    common_consts: [*mut _object; 5],
    jit: bool,
    executor_list_head: *mut _PyExecutorObject,
    executor_deletion_list_head: *mut _PyExecutorObject,
    executor_deletion_list_remaining_capacity: i32,
    trace_run_counter: u64,
    rare_events: _rare_events,
    builtins_dict_watcher:
        Option<unsafe extern "C-unwind" fn(u32, *mut _object, *mut _object, *mut _object) -> i32>,
    monitors: _Py_GlobalMonitors,
    sys_profile_once_flag: __SlateRecord_modsupport_h_9,
    sys_trace_once_flag: __SlateRecord_modsupport_h_9,
    sys_profiling_threads: i64,
    sys_tracing_threads: i64,
    monitoring_callables: [[*mut _object; 19]; 8],
    monitoring_tool_names: [*mut _object; 8],
    monitoring_tool_versions: [u64; 8],
    cached_objects: _Py_interp_cached_objects,
    static_objects: _Py_interp_static_objects,
    _interactive_src_count: i64,
    _initial_thread: _PyThreadStateImpl,
}

fn _PyThreadState_GET() -> *mut _ts {
    unsafe { _Py_tss_tstate }
}

#[repr(C, align(16))]
struct __SlateAlign16<T>(T);

unsafe impl<T> Sync for __SlateAlign16<T> {}

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
    pub struct __SlateBits137U0 {
        #[bits(1)]
        pub initialized: u32,
        #[bits(1)]
        pub bound: u32,
        #[bits(1)]
        pub unbound: u32,
        #[bits(1)]
        pub bound_gilstate: u32,
        #[bits(1)]
        pub active: u32,
        #[bits(1)]
        pub finalizing: u32,
        #[bits(1)]
        pub cleared: u32,
        #[bits(1)]
        pub finalized: u32,
        #[bits(24, access = na)]
        pub __slate_pad_8: u32,
    }
}

fn _PyInterpreterState_GET() -> *mut _is {
    let tstate: *mut _ts = _PyThreadState_GET();
    unsafe { (*tstate).interp }
}

// This is dtoa.c by David M. Gay, downloaded from
// http://www.netlib.org/fp/dtoa.c on April 15, 2009 and modified for
// inclusion into the Python core by Mark E. T. Dickinson and Eric V. Smith.
//
// Please remember to check http://www.netlib.org/fp regularly (and especially
// before any Python release) for bugfixes and updates.
//
// The major modifications from Gay's original code are as follows:
//
//  0. The original code has been specialized to Python's needs by removing
//     many of the #ifdef'd sections.  In particular, code to support VAX and
//     IBM floating-point formats, hex NaNs, hex floats, locale-aware
//     treatment of the decimal point, and setting of the inexact flag have
//     been removed.
//
//  1. We use PyMem_Malloc and PyMem_Free in place of malloc and free.
//
//  2. The public functions strtod, dtoa and freedtoa all now have
//     a _Py_dg_ prefix.
//
//  3. Instead of assuming that PyMem_Malloc always succeeds, we thread
//     PyMem_Malloc failures through the code.  The functions
//
//       Balloc, multadd, s2b, i2b, mult, pow5mult, lshift, diff, d2b
//
//     of return type *Bigint all return NULL to indicate a malloc failure.
//     Similarly, rv_alloc and nrv_alloc (return type char *) return NULL on
//     failure.  bigcomp now has return type int (it used to be void) and
//     returns -1 on failure and 0 otherwise.  _Py_dg_dtoa returns NULL
//     on failure.  _Py_dg_strtod indicates failure due to malloc failure
//     by returning -1.0, setting errno=ENOMEM and *se to s00.
//
//  4. The static variable dtoa_result has been removed.  Callers of
//     _Py_dg_dtoa are expected to call _Py_dg_freedtoa to free
//     the memory allocated by _Py_dg_dtoa.
//
//  5. The code has been reformatted to better fit with Python's
//     C style guide (PEP 7).
//
//  6. A bug in the memory allocation has been fixed: to avoid FREEing memory
//     that hasn't been MALLOC'ed, private_mem should only be used when k <=
//     Kmax.
//
//  7. _Py_dg_strtod has been modified so that it doesn't accept strings with
//     leading whitespace.
//
//  8. A corner case where _Py_dg_dtoa didn't strip trailing zeros has been
//     fixed. (bugs.python.org/issue40780)
//
//  9. _Py_dg_dtoa clamps ndigits in modes 3 and 5 so that its buffer size
//     arithmetic cannot exceed the int range, and rv_alloc's size doubling
//     uses size_t. (gh-158446)
// Please send bug reports for the original dtoa.c code to David M. Gay (dmg
// at acm dot org, with " at " changed at "@" and " dot " changed to ".").
// Please report bugs for this modified version using the Python issue tracker
// as detailed at (https://devguide.python.org/triage/issue-tracker/).
// On a machine with IEEE extended-precision registers, it is
// necessary to specify double-precision (53-bit) rounding precision
// before invoking strtod or dtoa.  If the machine uses (the equivalent
// of) Intel 80x87 arithmetic, the call
//      _control87(PC_53, MCW_PC);
// does this with many compilers.  Whether this or another call is
// appropriate depends on the compiler; for this to work, it may be
// necessary to #include "float.h" or another system-dependent header
// file.
// strtod for IEEE-, VAX-, and IBM-arithmetic machines.
//
// This strtod returns a nearest machine number to the input decimal
// string (or sets errno to ERANGE).  With IEEE arithmetic, ties are
// broken by the IEEE round-even rule.  Otherwise ties are broken by
// biased rounding (add half and chop).
//
// Inspired loosely by William D. Clinger's paper "How to Read Floating
// Point Numbers Accurately" [Proc. ACM SIGPLAN '90, pp. 92-101].
//
// Modifications:
//
//      1. We only require IEEE, IBM, or VAX double-precision
//              arithmetic (not IEEE double-extended).
//      2. We get by with floating-point arithmetic in a case that
//              Clinger missed -- when we're computing d * 10^n
//              for a small integer d and the integer n is not too
//              much larger than 22 (the maximum integer k for which
//              we can represent 10^k exactly), we may be able to
//              compute (d*10^k) * 10^(e-k) with just one roundoff.
//      3. Rather than a bit-at-a-time adjustment of the binary
//              result in the hard case, we use floating-point
//              arithmetic to determine the adjustment to within
//              one bit; only in really hard cases do we need to
//              compute a second residual.
//      4. Because of 3., we don't need a large table of powers of 10
//              for ten-to-e (just some small tables, e.g. of 10^k
//              for 0 <= k <= 22).
// Linking of Python's #defines to Gay's #defines starts here.
// _PY_SHORT_FLOAT_REPR
// struct Bigint
// _PyInterpreterState_GET()
// exit()
// if _PY_SHORT_FLOAT_REPR == 0, then don't even try to compile
// the following code
// This code should also work for ARM mixed-endian format on little-endian
// machines, where doubles have byte order 45670123 (in increasing address
// order, 0 being the least significant byte).
// End Python #define linking
/// The code below assumes that the endianness of integers matches the
/// endianness of the two 32-bit words of a double.  Check this.
#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord_dtoa_c_181 {
    d: f64,
    L: [u32; 2],
}

// maximum permitted exponent value for strtod; exponents larger than
// MAX_ABS_EXP in absolute value get truncated to +-MAX_ABS_EXP.  MAX_ABS_EXP
// should fit into an int.
// Bound on length of pieces of input strings in _Py_dg_strtod; specifically,
// this is used to bound the total number of digits ignoring leading zeros and
// the number of digits that follow the decimal point.  Ideally, MAX_DIGITS
// should satisfy MAX_DIGITS + 400 < MAX_ABS_EXP; that ensures that the
// exponent clipping in _Py_dg_strtod can't affect the value of the output.
// Guard against trying to use the above values on unusual platforms with ints
// of width less than 32 bits.
// The following definition of Storeinc is appropriate for MIPS processors.
// An alternative that might be better on some machines is
// #define Storeinc(a,b,c) (*a++ = b << 16 | c & 0xffff)
// #define P DBL_MANT_DIG
//
// Ten_pmax = floor(P*log(2)/log(5))
//
// Bletch = (highest power of 2 < DBL_MAX_10_EXP) / 16
//
// Quick_max = floor((P-1)*log(FLT_RADIX)/log(10) - 1)
//
// Int_max = floor(P*log(FLT_RADIX)/log(10) - 1)
// smallest denormal is 2**Etiny
// Bits of the representation of positive infinity.
// struct BCinfo is used to pass information from _Py_dg_strtod to bigcomp
#[repr(C)]
#[derive(Clone, Copy)]
struct BCinfo {
    e0: i32,
    nd: i32,
    nd0: i32,
    scale: i32,
}

// struct Bigint is used to represent arbitrary-precision integers.  These
// integers are stored in sign-magnitude format, with the magnitude stored as
// an array of base 2**32 digits.  Bigints are always normalized: if x is a
// Bigint then x->wds >= 1, and either x->wds == 1 or x[wds-1] is nonzero.
//
// The Bigint fields are as follows:
//
//   - next is a header used by Balloc and Bfree to keep track of lists
//       of freed Bigints;  it's also used for the linked list of
//       powers of 5 of the form 5**2**i used by pow5mult.
//   - k indicates which pool this Bigint was allocated from
//   - maxwds is the maximum number of words space was allocated for
//     (usually maxwds == 2**k)
//   - sign is 1 for negative Bigints, 0 for positive.  The sign is unused
//     (ignored on inputs, set to 0 on outputs) in almost all operations
//     involving Bigints: a notable exception is the diff function, which
//     ignores signs on inputs but sets the sign of the output correctly.
//   - wds is the actual number of significant words
//   - x contains the vector of words (digits) for this Bigint, from least
//     significant (x[0]) to most significant (x[wds-1]).
// Memory management: memory is allocated from, and returned to, Kmax+1 pools
// of memory, where pool k (0 <= k <= Kmax) is for Bigints b with b->maxwds ==
// 1 << k.  These pools are maintained as linked lists, with freelist[k]
// pointing to the head of the list for pool k.
//
// On allocation, if there's no free slot in the appropriate pool, MALLOC is
// called to get more memory.  This memory is not returned to the system until
// Python quits.  There's also a private memory pool that's allocated from
// in preference to using MALLOC.
//
// For Bigints with more than (1 << Kmax) digits (which implies at least 1233
// decimal digits), memory is directly allocated using MALLOC, and freed using
// FREE.
//
// XXX: it would be easy to bypass this memory-management system and
// translate each call to Balloc into a call to PyMem_Malloc, and each
// Bfree to PyMem_Free.  Investigate whether this has any significant
// performance on impact.
// Allocate space for a Bigint with up to 1<<k digits
// struct Bigint is defined in pycore_dtoa.h.
fn Balloc(k: i32) -> *mut Bigint {
    let mut x: i32 = 0;
    let mut rv: *mut Bigint = unsafe { core::mem::zeroed() };
    let mut len: u32 = 0;
    let interp: *mut _is = _PyInterpreterState_GET();
    let __v0: bool = if k <= 7 {
        let __v1: *mut Bigint = unsafe {
            *((*interp).dtoa.freelist.as_mut_ptr() as *mut *mut Bigint).wrapping_offset(k as isize)
        };
        rv = __v1;
        !__v1.is_null()
    } else {
        false
    };
    if __v0 {
        unsafe {
            *((*interp).dtoa.freelist.as_mut_ptr() as *mut *mut Bigint)
                .wrapping_offset(k as isize) = (*rv).next;
        }
    } else {
        x = 1i32 << k;
        len = ((core::mem::size_of::<Bigint>() as u64)
            .wrapping_add(
                (x.wrapping_sub(1) as u64).wrapping_mul(core::mem::size_of::<u32>() as u64),
            )
            .wrapping_add(core::mem::size_of::<f64>() as u64)
            .wrapping_sub(1)
            / (core::mem::size_of::<f64>() as u64)) as u32;
        if k <= 7
            && ((unsafe {
                unsafe { (*interp).dtoa.preallocated_next }
                    .offset_from(((*interp).dtoa.preallocated.as_mut_ptr() as *mut f64) as *mut f64)
            }) as i64)
                .wrapping_add(len as i64)
                <= ((2304u64
                    .wrapping_add(core::mem::size_of::<f64>() as u64)
                    .wrapping_sub(1)
                    / (core::mem::size_of::<f64>() as u64)) as i64)
        {
            rv = (unsafe { (*interp).dtoa.preallocated_next }) as *mut Bigint;
            let __v3: *mut f64 = unsafe { (*interp).dtoa.preallocated_next };
            unsafe {
                (*interp).dtoa.preallocated_next = __v3.wrapping_offset(len as isize);
            }
        } else {
            rv = (unsafe {
                PyMem_Malloc((len as u64).wrapping_mul(core::mem::size_of::<f64>() as u64))
            }) as *mut Bigint;
            if rv.is_null() {
                return core::ptr::null_mut::<Bigint>();
            }
        }
        unsafe {
            (*rv).k = k;
        }
        unsafe {
            (*rv).maxwds = x;
        }
    }
    unsafe {
        (*rv).wds = 0;
    }
    unsafe {
        (*rv).sign = 0;
    }
    rv
}

use slate_support::memcpy;

// Free a Bigint allocated with Balloc
fn Bfree(v: *mut Bigint) {
    if !v.is_null() {
        if (unsafe { (*v).k }) > 7 {
            unsafe { PyMem_Free(v as *mut ()) };
        } else {
            let interp: *mut _is = _PyInterpreterState_GET();
            unsafe {
                (*v).next = *((*interp).dtoa.freelist.as_mut_ptr() as *mut *mut Bigint)
                    .wrapping_offset((*v).k as isize);
            }
            unsafe {
                *((*interp).dtoa.freelist.as_mut_ptr() as *mut *mut Bigint)
                    .wrapping_offset((*v).k as isize) = v;
            }
        }
    }
}

// Multiply a Bigint b by m and add a.  Either modifies b in place and returns
// a pointer to the modified b, or Bfrees b and returns a pointer to a copy.
// On failure, return NULL.  In this case, b will have been already freed.
/// multiply by m and add a
fn multadd(mut b: *mut Bigint, m: i32, a: i32) -> *mut Bigint {
    let mut i: i32 = 0;
    let mut wds: i32 = 0;
    let mut x: *mut u32 = unsafe { core::mem::zeroed() };
    let mut carry: u64 = 0;
    let mut y: u64 = 0;
    wds = unsafe { (*b).wds };
    x = unsafe { (*b).x.as_mut_ptr() as *mut u32 };
    carry = a as u64;
    loop {
        y = ((unsafe { *x }) as u64)
            .wrapping_mul(m as u64)
            .wrapping_add(carry);
        carry = y >> 32i32;
        let __v0: *mut u32 = x;
        x = __v0.wrapping_offset(1 as isize);
        unsafe {
            *__v0 = (y & 4294967295) as u32;
        }
        let __v4: i32 = i.wrapping_add(1);
        i = __v4;
        if __v4 >= wds {
            break;
        }
    }
    if carry != 0 {
        if wds >= unsafe { (*b).maxwds } {
            let b1: *mut Bigint = Balloc(unsafe { (*b).k }.wrapping_add(1));
            if b1.is_null() {
                Bfree(b);
                return core::ptr::null_mut::<Bigint>();
            }
            Bcopy!(b1, b);
            Bfree(b);
            b = b1;
        }
        let __v5: i32 = wds;
        wds = __v5.wrapping_add(1);
        unsafe {
            *((*b).x.as_mut_ptr() as *mut u32).wrapping_offset(__v5 as isize) = carry as u32;
        }
        unsafe {
            (*b).wds = wds;
        }
    }
    b
}

// convert a string s containing nd decimal digits (possibly containing a
// decimal separator at position nd0, which is ignored) to a Bigint.  This
// function carries on where the parsing code in _Py_dg_strtod leaves off: on
// entry, y9 contains the result of converting the first 9 digits.  Returns
// NULL on failure.
fn s2b(mut s: *const i8, nd0: i32, nd: i32, y9: u32) -> *mut Bigint {
    let mut b: *mut Bigint = unsafe { core::mem::zeroed() };
    let mut i: i32 = 9;
    let mut k: i32 = 0;
    let x: i32 = nd.wrapping_add(8) / 9;
    let mut y: i32 = 1;
    while x > y {
        y <<= 1i32;
        k = k.wrapping_add(1);
    }
    b = Balloc(k);
    if b.is_null() {
        return core::ptr::null_mut::<Bigint>();
    }
    unsafe {
        *((*b).x.as_mut_ptr() as *mut u32).wrapping_offset(0 as isize) = y9;
    }
    unsafe {
        (*b).wds = 1;
    }
    if nd <= 9 {
        return b;
    }
    let __v5: *const i8 = s;
    s = __v5.wrapping_offset(9 as isize);
    while i < nd0 {
        let __v7: *const i8 = s;
        s = __v7.wrapping_offset(1 as isize);
        b = multadd(b, 10i32, ((unsafe { *__v7 }) as i32).wrapping_sub(48));
        if b.is_null() {
            return core::ptr::null_mut::<Bigint>();
        }
        i = i.wrapping_add(1);
    }
    let __v12: *const i8 = s;
    s = __v12.wrapping_offset(1 as isize);
    while i < nd {
        let __v14: *const i8 = s;
        s = __v14.wrapping_offset(1 as isize);
        b = multadd(b, 10i32, ((unsafe { *__v14 }) as i32).wrapping_sub(48));
        if b.is_null() {
            return core::ptr::null_mut::<Bigint>();
        }
        i = i.wrapping_add(1);
    }
    b
}

// count leading 0 bits in the 32-bit integer x.
fn hi0bits(mut x: u32) -> i32 {
    let mut k: i32 = 0;
    if x & 4294901760 == 0 {
        k = 16;
        x <<= 16i32;
    }
    if x & 4278190080 == 0 {
        k = k.wrapping_add(8);
        x <<= 8i32;
    }
    if x & 4026531840 == 0 {
        k = k.wrapping_add(4);
        x <<= 4i32;
    }
    if x & 3221225472 == 0 {
        k = k.wrapping_add(2);
        x <<= 2i32;
    }
    if x & 2147483648 == 0 {
        k = k.wrapping_add(1);
        if x & 1073741824 == 0 {
            return 32;
        }
    }
    k
}

// count trailing 0 bits in the 32-bit integer y, and shift y right by that
// number of bits.
fn lo0bits(y: &mut u32) -> i32 {
    let mut k: i32 = 0;
    let mut x: u32 = unsafe { *(y as *mut u32) };
    if x & 7 != 0 {
        if x & 1 != 0 {
            return 0;
        }
        if x & 2 != 0 {
            unsafe {
                *(y as *mut u32) = x >> 1i32;
            }
            return 1;
        }
        unsafe {
            *(y as *mut u32) = x >> 2i32;
        }
        return 2;
    }
    if x & 65535 == 0 {
        k = 16;
        x >>= 16i32;
    }
    if x & 255 == 0 {
        k = k.wrapping_add(8);
        x >>= 8i32;
    }
    if x & 15 == 0 {
        k = k.wrapping_add(4);
        x >>= 4i32;
    }
    if x & 3 == 0 {
        k = k.wrapping_add(2);
        x >>= 2i32;
    }
    if x & 1 == 0 {
        k = k.wrapping_add(1);
        x >>= 1i32;
        if x == 0 {
            return 32;
        }
    }
    unsafe {
        *(y as *mut u32) = x;
    }
    k
}

// convert a small nonnegative integer to a Bigint
fn i2b(i: i32) -> *mut Bigint {
    let b: *mut Bigint = Balloc(1i32);
    if b.is_null() {
        return core::ptr::null_mut::<Bigint>();
    }
    unsafe {
        *((*b).x.as_mut_ptr() as *mut u32).wrapping_offset(0 as isize) = i as u32;
    }
    unsafe {
        (*b).wds = 1;
    }
    b
}

// multiply two Bigints.  Returns a new Bigint, or NULL on failure.  Ignores
// the signs of a and b.
fn mult(mut a: *mut Bigint, mut b: *mut Bigint) -> *mut Bigint {
    let mut c: *mut Bigint = unsafe { core::mem::zeroed() };
    let mut k: i32 = 0;
    let mut wa: i32 = 0;
    let mut wb: i32 = 0;
    let mut wc: i32 = 0;
    let mut x: *mut u32 = unsafe { core::mem::zeroed() };
    let mut xa: *mut u32 = unsafe { core::mem::zeroed() };
    let mut xb: *mut u32 = unsafe { core::mem::zeroed() };
    let mut xc: *mut u32 = unsafe { core::mem::zeroed() };
    let mut xc0: *mut u32 = unsafe { core::mem::zeroed() };
    let mut y: u32 = 0;
    let mut carry: u64 = 0;
    let mut z: u64 = 0;
    if (unsafe { *((*a).x.as_mut_ptr() as *mut u32).wrapping_offset(0 as isize) }) == 0
        && (unsafe { (*a).wds }) == 1
        || (unsafe { *((*b).x.as_mut_ptr() as *mut u32).wrapping_offset(0 as isize) }) == 0
            && (unsafe { (*b).wds }) == 1
    {
        c = Balloc(0i32);
        if c.is_null() {
            return core::ptr::null_mut::<Bigint>();
        }
        unsafe {
            (*c).wds = 1;
        }
        unsafe {
            *((*c).x.as_mut_ptr() as *mut u32).wrapping_offset(0 as isize) = 0;
        }
        return c;
    }
    if (unsafe { (*a).wds }) < unsafe { (*b).wds } {
        c = a;
        a = b;
        b = c;
    }
    k = unsafe { (*a).k };
    wa = unsafe { (*a).wds };
    wb = unsafe { (*b).wds };
    wc = wa.wrapping_add(wb);
    if wc > unsafe { (*a).maxwds } {
        k = k.wrapping_add(1);
    }
    c = Balloc(k);
    if c.is_null() {
        return core::ptr::null_mut::<Bigint>();
    }
    x = unsafe { (*c).x.as_mut_ptr() as *mut u32 };
    xa = x.wrapping_offset(wc as isize);
    while x < xa {
        unsafe {
            *x = 0;
        }
        let __v4: *mut u32 = x;
        x = __v4.wrapping_offset(1 as isize);
    }
    xa = unsafe { (*a).x.as_mut_ptr() as *mut u32 };
    let xae: *mut u32 = xa.wrapping_offset(wa as isize);
    xb = unsafe { (*b).x.as_mut_ptr() as *mut u32 };
    let xbe: *mut u32 = xb.wrapping_offset(wb as isize);
    xc0 = unsafe { (*c).x.as_mut_ptr() as *mut u32 };
    while xb < xbe {
        let __v6: *mut u32 = xb;
        xb = __v6.wrapping_offset(1 as isize);
        let __v8: u32 = unsafe { *__v6 };
        y = __v8;
        if __v8 != 0 {
            x = xa;
            xc = xc0;
            carry = 0;
            loop {
                let __v9: *mut u32 = x;
                x = __v9.wrapping_offset(1 as isize);
                z = ((unsafe { *__v9 }) as u64)
                    .wrapping_mul(y as u64)
                    .wrapping_add((unsafe { *xc }) as u64)
                    .wrapping_add(carry);
                carry = z >> 32i32;
                let __v11: *mut u32 = xc;
                xc = __v11.wrapping_offset(1 as isize);
                unsafe {
                    *__v11 = (z & 4294967295) as u32;
                }
                if !(x < xae) {
                    break;
                }
            }
            unsafe {
                *xc = carry as u32;
            }
        }
        let __v15: *mut u32 = xc0;
        xc0 = __v15.wrapping_offset(1 as isize);
    }
    xc0 = unsafe { (*c).x.as_mut_ptr() as *mut u32 };
    xc = xc0.wrapping_offset(wc as isize);
    loop {
        let __v18: bool = if wc > 0 {
            let __v19: *mut u32 = xc;
            let __v20: *mut u32 = __v19.wrapping_offset(-(1 as isize));
            xc = __v20;
            (unsafe { *__v20 }) == 0
        } else {
            false
        };
        if !__v18 {
            break;
        }
        wc = wc.wrapping_sub(1);
    }
    unsafe {
        (*c).wds = wc;
    }
    c
}

// multiply the Bigint b by 5**k.  Returns a pointer to the result, or NULL on
// failure; if the returned pointer is distinct from b then the original
// Bigint b will have been Bfree'd.   Ignores the sign of b.
fn pow5mult(mut b: *mut Bigint, mut k: i32) -> *mut Bigint {
    let mut p5s: *mut *mut Bigint = unsafe { core::mem::zeroed() };
    let mut i: i32 = 0;
    // For double-to-string conversion, the maximum value of k is limited by
    //
    // DBL_MAX_10_EXP (308), the maximum decimal base-10 exponent for binary64.
    //
    // For string-to-double conversion, the extreme case is constrained by our
    //
    // hardcoded exponent limit before we underflow of -512, adjusted by
    //
    // STRTOD_DIGLIM-DBL_DIG-1, giving a maximum of k=535.
    let __v0: i32 = k & 3;
    i = __v0;
    if __v0 != 0 {
        b = multadd(b, unsafe { p05[i.wrapping_sub(1) as usize] }, 0i32);
        if b.is_null() {
            return core::ptr::null_mut::<Bigint>();
        }
    }
    let __v2: i32 = k >> 2i32;
    k = __v2;
    if __v2 == 0 {
        return b;
    }
    let interp: *mut _is = _PyInterpreterState_GET();
    p5s = unsafe { (*interp).dtoa.p5s.as_mut_ptr() as *mut *mut Bigint };
    loop {
        let p5: *mut Bigint = unsafe { *p5s };
        let __v3: *mut *mut Bigint = p5s;
        p5s = __v3.wrapping_offset(1 as isize);
        if k & 1 != 0 {
            let b1: *mut Bigint = mult(b, p5);
            Bfree(b);
            b = b1;
            if b.is_null() {
                return core::ptr::null_mut::<Bigint>();
            }
        }
        let __v6: i32 = k >> 1i32;
        k = __v6;
        if __v6 == 0 {
            break;
        }
    }
    b
}

static p05: [i32; 3] = [5, 25, 125];

// shift a Bigint b left by k bits.  Return a pointer to the shifted result,
// or NULL on failure.  If the returned pointer is distinct from b then the
// original b will have been Bfree'd.   Ignores the sign of b.
fn lshift(b: *mut Bigint, mut k: i32) -> *mut Bigint {
    let mut i: i32 = 0;
    let mut k1: i32 = 0;
    let mut n: i32 = 0;
    let mut n1: i32 = 0;
    let mut x: *mut u32 = unsafe { core::mem::zeroed() };
    let mut x1: *mut u32 = unsafe { core::mem::zeroed() };
    let mut z: u32 = 0;
    if k == 0
        || (unsafe { *((*b).x.as_mut_ptr() as *mut u32).wrapping_offset(0 as isize) }) == 0
            && (unsafe { (*b).wds }) == 1
    {
        return b;
    }
    n = k >> 5i32;
    k1 = unsafe { (*b).k };
    n1 = n.wrapping_add(unsafe { (*b).wds }).wrapping_add(1);
    i = unsafe { (*b).maxwds };
    while n1 > i {
        k1 = k1.wrapping_add(1);
        i <<= 1i32;
    }
    let b1: *mut Bigint = Balloc(k1);
    if b1.is_null() {
        Bfree(b);
        return core::ptr::null_mut::<Bigint>();
    }
    x1 = unsafe { (*b1).x.as_mut_ptr() as *mut u32 };
    i = 0;
    while i < n {
        let __v5: *mut u32 = x1;
        x1 = __v5.wrapping_offset(1 as isize);
        unsafe {
            *__v5 = 0;
        }
        i = i.wrapping_add(1);
    }
    x = unsafe { (*b).x.as_mut_ptr() as *mut u32 };
    let xe: *mut u32 = x.wrapping_offset((unsafe { (*b).wds }) as isize);
    let __v11: i32 = k & 31;
    k = __v11;
    if __v11 != 0 {
        k1 = 32i32.wrapping_sub(k);
        z = 0;
        loop {
            let __v12: *mut u32 = x1;
            x1 = __v12.wrapping_offset(1 as isize);
            unsafe {
                *__v12 = *x << k | z;
            }
            let __v14: *mut u32 = x;
            x = __v14.wrapping_offset(1 as isize);
            z = (unsafe { *__v14 }) >> k1;
            if !(x < xe) {
                break;
            }
        }
        let __v17: u32 = z;
        unsafe {
            *x1 = __v17;
        }
        if __v17 != 0 {
            n1 = n1.wrapping_add(1);
        }
    } else {
        loop {
            let __v20: *mut u32 = x;
            x = __v20.wrapping_offset(1 as isize);
            let __v22: *mut u32 = x1;
            x1 = __v22.wrapping_offset(1 as isize);
            unsafe {
                *__v22 = *__v20;
            }
            if !(x < xe) {
                break;
            }
        }
    }
    unsafe {
        (*b1).wds = n1.wrapping_sub(1);
    }
    Bfree(b);
    b1
}

// Do a three-way compare of a and b, returning -1 if a < b, 0 if a == b and
// 1 if a > b.  Ignores signs of a and b.
fn cmp(a: *mut Bigint, b: *mut Bigint) -> i32 {
    let mut xa: *mut u32 = unsafe { core::mem::zeroed() };
    let mut xb: *mut u32 = unsafe { core::mem::zeroed() };
    let mut i: i32 = unsafe { (*a).wds };
    let j: i32 = unsafe { (*b).wds };
    let __v1: i32 = i.wrapping_sub(j);
    i = __v1;
    if __v1 != 0 {
        return i;
    }
    let xa0: *mut u32 = unsafe { (*a).x.as_mut_ptr() as *mut u32 };
    xa = xa0.wrapping_offset(j as isize);
    let xb0: *mut u32 = unsafe { (*b).x.as_mut_ptr() as *mut u32 };
    xb = xb0.wrapping_offset(j as isize);
    loop {
        let __v2: *mut u32 = xa;
        let __v3: *mut u32 = __v2.wrapping_offset(-(1 as isize));
        xa = __v3;
        let __v4: *mut u32 = xb;
        let __v5: *mut u32 = __v4.wrapping_offset(-(1 as isize));
        xb = __v5;
        if (unsafe { *__v3 }) != unsafe { *__v5 } {
            return if (unsafe { *xa }) < unsafe { *xb } {
                1i32.wrapping_neg()
            } else {
                1
            };
        }
        if xa <= xa0 {
            break;
        }
    }
    0
}

// Take the difference of Bigints a and b, returning a new Bigint.  Returns
// NULL on failure.  The signs of a and b are ignored, but the sign of the
// result is set appropriately.
fn diff(mut a: *mut Bigint, mut b: *mut Bigint) -> *mut Bigint {
    let mut c: *mut Bigint = unsafe { core::mem::zeroed() };
    let mut i: i32 = 0;
    let mut wa: i32 = 0;
    let mut wb: i32 = 0;
    let mut xa: *mut u32 = unsafe { core::mem::zeroed() };
    let mut xb: *mut u32 = unsafe { core::mem::zeroed() };
    let mut xc: *mut u32 = unsafe { core::mem::zeroed() };
    let mut borrow: u64 = 0;
    let mut y: u64 = 0;
    i = cmp(a, b);
    if i == 0 {
        c = Balloc(0i32);
        if c.is_null() {
            return core::ptr::null_mut::<Bigint>();
        }
        unsafe {
            (*c).wds = 1;
        }
        unsafe {
            *((*c).x.as_mut_ptr() as *mut u32).wrapping_offset(0 as isize) = 0;
        }
        return c;
    }
    if i < 0 {
        c = a;
        a = b;
        b = c;
        i = 1;
    } else {
        i = 0;
    }
    c = Balloc(unsafe { (*a).k });
    if c.is_null() {
        return core::ptr::null_mut::<Bigint>();
    }
    unsafe {
        (*c).sign = i;
    }
    wa = unsafe { (*a).wds };
    xa = unsafe { (*a).x.as_mut_ptr() as *mut u32 };
    let xae: *mut u32 = xa.wrapping_offset(wa as isize);
    wb = unsafe { (*b).wds };
    xb = unsafe { (*b).x.as_mut_ptr() as *mut u32 };
    let xbe: *mut u32 = xb.wrapping_offset(wb as isize);
    xc = unsafe { (*c).x.as_mut_ptr() as *mut u32 };
    loop {
        let __v0: *mut u32 = xa;
        xa = __v0.wrapping_offset(1 as isize);
        let __v2: *mut u32 = xb;
        xb = __v2.wrapping_offset(1 as isize);
        y = ((unsafe { *__v0 }) as u64)
            .wrapping_sub((unsafe { *__v2 }) as u64)
            .wrapping_sub(borrow);
        borrow = y >> 32i32 & 1;
        let __v4: *mut u32 = xc;
        xc = __v4.wrapping_offset(1 as isize);
        unsafe {
            *__v4 = (y & 4294967295) as u32;
        }
        if !(xb < xbe) {
            break;
        }
    }
    while xa < xae {
        let __v7: *mut u32 = xa;
        xa = __v7.wrapping_offset(1 as isize);
        y = ((unsafe { *__v7 }) as u64).wrapping_sub(borrow);
        borrow = y >> 32i32 & 1;
        let __v9: *mut u32 = xc;
        xc = __v9.wrapping_offset(1 as isize);
        unsafe {
            *__v9 = (y & 4294967295) as u32;
        }
    }
    loop {
        let __v14: *mut u32 = xc;
        let __v15: *mut u32 = __v14.wrapping_offset(-(1 as isize));
        xc = __v15;
        if (unsafe { *__v15 }) != 0 {
            break;
        }
        wa = wa.wrapping_sub(1);
    }
    unsafe {
        (*c).wds = wa;
    }
    c
}

// Given a positive normal double x, return the difference between x and the
// next double up.  Doesn't give correct results for subnormals.
fn ulp(x: *mut __SlateRecord_dtoa_c_181) -> f64 {
    let mut L: i32 = 0;
    let mut u: __SlateRecord_dtoa_c_181 = unsafe { core::mem::zeroed() };
    L = (word0!(x) & 2146435072).wrapping_sub(54525952u32) as i32;
    unsafe {
        *((*core::ptr::addr_of_mut!(u)).L.as_mut_ptr() as *mut u32).wrapping_offset(1 as isize) =
            L as u32;
    }
    unsafe {
        *((*core::ptr::addr_of_mut!(u)).L.as_mut_ptr() as *mut u32).wrapping_offset(0 as isize) = 0;
    }
    dval!(core::ptr::addr_of_mut!(u))
}

// Convert a Bigint to a double plus an exponent
fn b2d(a: *mut Bigint, e: &mut i32) -> f64 {
    let mut __v2: u32 = unsafe { core::mem::zeroed() };
    let mut __v7: u32 = unsafe { core::mem::zeroed() };
    let mut __v10: u32 = unsafe { core::mem::zeroed() };
    let mut d: core::mem::MaybeUninit<__SlateRecord_dtoa_c_181> = core::mem::MaybeUninit::uninit();
    let __slate_slot_13: *mut __SlateRecord_dtoa_c_181 =
        core::ptr::addr_of_mut!(d) as *mut __SlateRecord_dtoa_c_181;
    let mut k: i32 = unsafe { core::mem::zeroed() };
    let mut z: u32 = unsafe { core::mem::zeroed() };
    let mut y: u32 = unsafe { core::mem::zeroed() };
    let mut xa0: *mut u32 = unsafe { core::mem::zeroed() };
    let mut xa: *mut u32 = unsafe { core::mem::zeroed() };
    unsafe {
        xa0 = (*a).x.as_mut_ptr() as *mut u32;
        xa = xa0.wrapping_offset((*a).wds as isize);
        let __v12: *mut u32 = xa;
        let __v11: *mut u32 = __v12.wrapping_offset(-(1 as isize));
        xa = __v11;
        y = *__v11;
        k = hi0bits(y);
        *(e as *mut i32) = 32i32.wrapping_sub(k);
        if k < 11 {
            *((*core::ptr::addr_of_mut!(*__slate_slot_13)).L.as_mut_ptr() as *mut u32)
                .wrapping_offset(1 as isize) = 1072693248 | y >> 11i32.wrapping_sub(k);
            if xa > xa0 {
                let __v9: *mut u32 = xa;
                let __v8: *mut u32 = __v9.wrapping_offset(-(1 as isize));
                xa = __v8;
                __v10 = *__v8;
            } else {
                __v10 = 0;
            }
            let w: u32 = __v10;
            *((*core::ptr::addr_of_mut!(*__slate_slot_13)).L.as_mut_ptr() as *mut u32)
                .wrapping_offset(0 as isize) =
                y << 21i32.wrapping_add(k) | w >> 11i32.wrapping_sub(k);
        } else {
            if xa > xa0 {
                let __v6: *mut u32 = xa;
                let __v5: *mut u32 = __v6.wrapping_offset(-(1 as isize));
                xa = __v5;
                __v7 = *__v5;
            } else {
                __v7 = 0;
            }
            z = __v7;
            let __v3: i32 = k.wrapping_sub(11);
            k = __v3;
            if __v3 != 0 {
                *((*core::ptr::addr_of_mut!(*__slate_slot_13)).L.as_mut_ptr() as *mut u32)
                    .wrapping_offset(1 as isize) = 1072693248 | y << k | z >> 32i32.wrapping_sub(k);
                if xa > xa0 {
                    let __v1: *mut u32 = xa;
                    let __v0: *mut u32 = __v1.wrapping_offset(-(1 as isize));
                    xa = __v0;
                    __v2 = *__v0;
                } else {
                    __v2 = 0;
                }
                y = __v2;
                *((*core::ptr::addr_of_mut!(*__slate_slot_13)).L.as_mut_ptr() as *mut u32)
                    .wrapping_offset(0 as isize) = z << k | y >> 32i32.wrapping_sub(k);
            } else {
                *((*core::ptr::addr_of_mut!(*__slate_slot_13)).L.as_mut_ptr() as *mut u32)
                    .wrapping_offset(1 as isize) = 1072693248 | y;
                *((*core::ptr::addr_of_mut!(*__slate_slot_13)).L.as_mut_ptr() as *mut u32)
                    .wrapping_offset(0 as isize) = z;
            }
        }
        return dval!(core::ptr::addr_of_mut!(*__slate_slot_13));
    }
}

// Convert a scaled double to a Bigint plus an exponent.  Similar to d2b,
// except that it accepts the scale parameter used in _Py_dg_strtod (which
// should be either 0 or 2*P), and the normalization for the return value is
// different (see below).  On input, d should be finite and nonnegative, and d
// / 2**scale should be exactly representable as an IEEE 754 double.
//
// Returns a Bigint b and an integer e such that
//
//   dval(d) / 2**scale = b * 2**e.
//
// Unlike d2b, b is not necessarily odd: b and e are normalized so
// that either 2**(P-1) <= b < 2**P and e >= Etiny, or b < 2**P
// and e == Etiny.  This applies equally to an input of 0.0: in that
// case the return values are b = 0 and e = Etiny.
//
// The above normalization ensures that for all possible inputs d,
// 2**e gives ulp(d/2**scale).
//
// Returns NULL on failure.
fn sd2b(d: *mut __SlateRecord_dtoa_c_181, mut scale: i32, e: &mut i32) -> *mut Bigint {
    let b: *mut Bigint = Balloc(1i32);
    if b.is_null() {
        return core::ptr::null_mut::<Bigint>();
    }
    // First construct b and e assuming that scale == 0.
    unsafe {
        (*b).wds = 2;
    }
    unsafe {
        *((*b).x.as_mut_ptr() as *mut u32).wrapping_offset(0 as isize) = word1!(d);
    }
    unsafe {
        *((*b).x.as_mut_ptr() as *mut u32).wrapping_offset(1 as isize) = word0!(d) & 1048575;
    }
    unsafe {
        *(e as *mut i32) = 1074i32
            .wrapping_neg()
            .wrapping_sub(1)
            .wrapping_add(((word0!(d) & 2146435072) >> 20i32) as i32);
    }
    if (unsafe { *(e as *mut i32) }) < 1074i32.wrapping_neg() {
        unsafe {
            *(e as *mut i32) = 1074i32.wrapping_neg();
        }
    } else {
        let __v0: *mut u32 = unsafe { (*b).x.as_mut_ptr() as *mut u32 }.wrapping_offset(1 as isize);
        unsafe {
            *__v0 = *__v0 | 1048576;
        }
    }
    // Now adjust for scale, provided that b != 0.
    if scale != 0
        && ((unsafe { *((*b).x.as_mut_ptr() as *mut u32).wrapping_offset(0 as isize) }) != 0
            || (unsafe { *((*b).x.as_mut_ptr() as *mut u32).wrapping_offset(1 as isize) }) != 0)
    {
        let __v4: i32 = unsafe { *e };
        unsafe {
            *e = __v4.wrapping_sub(scale);
        }
        if (unsafe { *(e as *mut i32) }) < 1074i32.wrapping_neg() {
            scale = 1074i32
                .wrapping_neg()
                .wrapping_sub(unsafe { *(e as *mut i32) });
            unsafe {
                *(e as *mut i32) = 1074i32.wrapping_neg();
            }
            // We can't shift more than P-1 bits without shifting out a 1.
            if scale >= 32 {
                // The bits shifted out should all be zero.
                unsafe {
                    *((*b).x.as_mut_ptr() as *mut u32).wrapping_offset(0 as isize) =
                        *((*b).x.as_mut_ptr() as *mut u32).wrapping_offset(1 as isize);
                }
                unsafe {
                    *((*b).x.as_mut_ptr() as *mut u32).wrapping_offset(1 as isize) = 0;
                }
                scale = scale.wrapping_sub(32);
            }
            if scale != 0 {
                // The bits shifted out should all be zero.
                unsafe {
                    *((*b).x.as_mut_ptr() as *mut u32).wrapping_offset(0 as isize) =
                        *((*b).x.as_mut_ptr() as *mut u32).wrapping_offset(0 as isize) >> scale
                            | *((*b).x.as_mut_ptr() as *mut u32).wrapping_offset(1 as isize)
                                << 32i32.wrapping_sub(scale);
                }
                let __v8: *mut u32 =
                    unsafe { (*b).x.as_mut_ptr() as *mut u32 }.wrapping_offset(1 as isize);
                unsafe {
                    *__v8 = *__v8 >> scale;
                }
            }
        }
    }
    // Ensure b is normalized.
    if (unsafe { *((*b).x.as_mut_ptr() as *mut u32).wrapping_offset(1 as isize) }) == 0 {
        unsafe {
            (*b).wds = 1;
        }
    }
    b
}

// Convert a double to a Bigint plus an exponent.  Return NULL on failure.
//
// Given a finite nonzero double d, return an odd Bigint b and exponent *e
// such that fabs(d) = b * 2**e.  On return, *bbits gives the number of
// significant bits of b; that is, 2**(*bbits-1) <= b < 2**(*bbits).
//
// If d is zero, then b == 0, *e == -1010, *bbits = 0.
fn d2b(d: *mut __SlateRecord_dtoa_c_181, e: &mut i32, bits: &mut i32) -> *mut Bigint {
    let mut de: i32 = 0;
    let mut k: i32 = 0;
    let mut y: u32 = 0;
    let mut z: u32 = 0;
    let mut i: i32 = 0;
    let b: *mut Bigint = Balloc(1i32);
    if b.is_null() {
        return core::ptr::null_mut::<Bigint>();
    }
    let x: *mut u32 = unsafe { (*b).x.as_mut_ptr() as *mut u32 };
    z = word0!(d) & 1048575;
    let __v0: *mut u32 = unsafe { (*d).L.as_mut_ptr() as *mut u32 }.wrapping_offset(1 as isize);
    unsafe {
        *__v0 = *__v0 & 2147483647;
    }
    // clear sign bit, which we ignore
    let __v3: i32 = (word0!(d) >> 20i32) as i32;
    de = __v3;
    if __v3 != 0 {
        z |= 1048576;
    }
    let __v6: u32 = word1!(d);
    y = __v6;
    if __v6 != 0 {
        let __v7: i32 = lo0bits(unsafe { &mut (*core::ptr::addr_of_mut!(y)) });
        k = __v7;
        if __v7 != 0 {
            unsafe {
                *x.wrapping_offset(0 as isize) = y | z << 32i32.wrapping_sub(k);
            }
            z >>= k;
        } else {
            unsafe {
                *x.wrapping_offset(0 as isize) = y;
            }
        }
        let __v10: u32 = z;
        unsafe {
            *x.wrapping_offset(1 as isize) = __v10;
        }
        let __v11: i32 = if __v10 != 0 { 2 } else { 1 };
        unsafe {
            (*b).wds = __v11;
        }
        i = __v11;
    } else {
        k = lo0bits(unsafe { &mut (*core::ptr::addr_of_mut!(z)) });
        unsafe {
            *x.wrapping_offset(0 as isize) = z;
        }
        unsafe {
            (*b).wds = 1;
        }
        i = 1;
        k = k.wrapping_add(32);
    }
    if de != 0 {
        unsafe {
            *(e as *mut i32) = de.wrapping_sub(1023).wrapping_sub(52i32).wrapping_add(k);
        }
        unsafe {
            *(bits as *mut i32) = 53i32.wrapping_sub(k);
        }
    } else {
        unsafe {
            *(e as *mut i32) = de
                .wrapping_sub(1023)
                .wrapping_sub(52i32)
                .wrapping_add(1)
                .wrapping_add(k);
        }
        unsafe {
            *(bits as *mut i32) = 32i32
                .wrapping_mul(i)
                .wrapping_sub(hi0bits(*x.wrapping_offset(i.wrapping_sub(1) as isize)));
        }
    }
    b
}

// Compute the ratio of two Bigints, as a double.  The result may have an
// error of up to 2.5 ulps.
fn ratio(a: *mut Bigint, b: *mut Bigint) -> f64 {
    let mut da: __SlateRecord_dtoa_c_181 = unsafe { core::mem::zeroed() };
    let mut db: __SlateRecord_dtoa_c_181 = unsafe { core::mem::zeroed() };
    let mut k: i32 = 0;
    let mut ka: i32 = 0;
    let mut kb: i32 = 0;
    unsafe {
        (*core::ptr::addr_of_mut!(da)).d = b2d(a, &mut (*core::ptr::addr_of_mut!(ka)));
    }
    unsafe {
        (*core::ptr::addr_of_mut!(db)).d = b2d(b, &mut (*core::ptr::addr_of_mut!(kb)));
    }
    k = ka
        .wrapping_sub(kb)
        .wrapping_add(32i32.wrapping_mul(unsafe { (*a).wds }.wrapping_sub(unsafe { (*b).wds })));
    if k > 0 {
        let __v0: *mut u32 = unsafe { (*core::ptr::addr_of_mut!(da)).L.as_mut_ptr() as *mut u32 }
            .wrapping_offset(1 as isize);
        let __v1: u32 = unsafe { *__v0 };
        unsafe {
            *__v0 = __v1.wrapping_add(k.wrapping_mul(1048576) as u32);
        }
    } else {
        k = k.wrapping_neg();
        let __v3: *mut u32 = unsafe { (*core::ptr::addr_of_mut!(db)).L.as_mut_ptr() as *mut u32 }
            .wrapping_offset(1 as isize);
        let __v4: u32 = unsafe { *__v3 };
        unsafe {
            *__v3 = __v4.wrapping_add(k.wrapping_mul(1048576) as u32);
        }
    }
    dval!(core::ptr::addr_of_mut!(da)) / dval!(core::ptr::addr_of_mut!(db))
}

static tens: __SlateAlign16<[f64; 23]> = __SlateAlign16([
    1.0f64,
    10.0f64,
    100.0f64,
    1000.0f64,
    10000.0f64,
    100000.0f64,
    1000000.0f64,
    10000000.0f64,
    100000000.0f64,
    1000000000.0f64,
    10000000000.0f64,
    100000000000.0f64,
    1000000000000.0f64,
    10000000000000.0f64,
    100000000000000.0f64,
    1000000000000000.0f64,
    1e16f64,
    1e17f64,
    1e18f64,
    1e19f64,
    1e20f64,
    1e21f64,
    1e22f64,
]);

static bigtens: __SlateAlign16<[f64; 5]> =
    __SlateAlign16([1e16f64, 1e32f64, 1e64f64, 1e128f64, 1e256f64]);

/// = 2^106 * 1e-256
static tinytens: __SlateAlign16<[f64; 5]> = __SlateAlign16([
    1e-16f64,
    1e-32f64,
    1e-64f64,
    1e-128f64,
    9007199254740992.0f64 * 9.007199254740992e-241f64,
]);

// The factor of 2^53 in tinytens[4] helps us avoid setting the underflow
//
// flag unnecessarily.  It leads to a song and dance at the end of strtod.
fn dshift(b: *mut Bigint, p2: i32) -> i32 {
    let mut rv: i32 = hi0bits(unsafe {
        *((*b).x.as_mut_ptr() as *mut u32)
            .wrapping_offset(unsafe { (*b).wds }.wrapping_sub(1) as isize)
    })
    .wrapping_sub(4);
    if p2 > 0 {
        rv = rv.wrapping_sub(p2);
    }
    rv & 31
}

// special case of Bigint division.  The quotient is always in the range 0 <=
// quotient < 10, and on entry the divisor S is normalized so that its top 4
// bits (28--31) are zero and bit 27 is set.
fn quorem(b: *mut Bigint, S: *mut Bigint) -> i32 {
    let mut n: i32 = 0;
    let mut bx: *mut u32 = unsafe { core::mem::zeroed() };
    let mut bxe: *mut u32 = unsafe { core::mem::zeroed() };
    let mut q: u32 = 0;
    let mut sx: *mut u32 = unsafe { core::mem::zeroed() };
    let mut borrow: u64 = 0;
    let mut carry: u64 = 0;
    let mut y: u64 = 0;
    let mut ys: u64 = 0;
    n = unsafe { (*S).wds };
    if (unsafe { (*b).wds }) < n {
        return 0;
    }
    sx = unsafe { (*S).x.as_mut_ptr() as *mut u32 };
    let __v1: i32 = n.wrapping_sub(1);
    n = __v1;
    let sxe: *mut u32 = sx.wrapping_offset(__v1 as isize);
    bx = unsafe { (*b).x.as_mut_ptr() as *mut u32 };
    bxe = bx.wrapping_offset(n as isize);
    q = (unsafe { *bxe }) / unsafe { *sxe }.wrapping_add(1); // ensure q <= true quotient
    if q != 0 {
        borrow = 0;
        carry = 0;
        loop {
            let __v2: *mut u32 = sx;
            sx = __v2.wrapping_offset(1 as isize);
            ys = ((unsafe { *__v2 }) as u64)
                .wrapping_mul(q as u64)
                .wrapping_add(carry);
            carry = ys >> 32i32;
            y = ((unsafe { *bx }) as u64)
                .wrapping_sub(ys & 4294967295)
                .wrapping_sub(borrow);
            borrow = y >> 32i32 & 1;
            let __v4: *mut u32 = bx;
            bx = __v4.wrapping_offset(1 as isize);
            unsafe {
                *__v4 = (y & 4294967295) as u32;
            }
            if !(sx <= sxe) {
                break;
            }
        }
        if (unsafe { *bxe }) == 0 {
            bx = unsafe { (*b).x.as_mut_ptr() as *mut u32 };
            loop {
                let __v9: *mut u32 = bxe;
                let __v10: *mut u32 = __v9.wrapping_offset(-(1 as isize));
                bxe = __v10;
                if !(__v10 > bx && (unsafe { *bxe }) == 0) {
                    break;
                }
                n = n.wrapping_sub(1);
            }
            unsafe {
                (*b).wds = n;
            }
        }
    }
    if cmp(b, S) >= 0 {
        q = q.wrapping_add(1);
        borrow = 0;
        carry = 0;
        bx = unsafe { (*b).x.as_mut_ptr() as *mut u32 };
        sx = unsafe { (*S).x.as_mut_ptr() as *mut u32 };
        loop {
            let __v14: *mut u32 = sx;
            sx = __v14.wrapping_offset(1 as isize);
            ys = ((unsafe { *__v14 }) as u64).wrapping_add(carry);
            carry = ys >> 32i32;
            y = ((unsafe { *bx }) as u64)
                .wrapping_sub(ys & 4294967295)
                .wrapping_sub(borrow);
            borrow = y >> 32i32 & 1;
            let __v16: *mut u32 = bx;
            bx = __v16.wrapping_offset(1 as isize);
            unsafe {
                *__v16 = (y & 4294967295) as u32;
            }
            if !(sx <= sxe) {
                break;
            }
        }
        bx = unsafe { (*b).x.as_mut_ptr() as *mut u32 };
        bxe = bx.wrapping_offset(n as isize);
        if (unsafe { *bxe }) == 0 {
            loop {
                let __v21: *mut u32 = bxe;
                let __v22: *mut u32 = __v21.wrapping_offset(-(1 as isize));
                bxe = __v22;
                if !(__v22 > bx && (unsafe { *bxe }) == 0) {
                    break;
                }
                n = n.wrapping_sub(1);
            }
            unsafe {
                (*b).wds = n;
            }
        }
    }
    q as i32
}

// sulp(x) is a version of ulp(x) that takes bc.scale into account.
//
// Assuming that x is finite and nonnegative (positive zero is fine
// here) and x / 2^bc.scale is exactly representable as a double,
// sulp(x) is equivalent to 2^bc.scale * ulp(x / 2^bc.scale).
fn sulp(x: *mut __SlateRecord_dtoa_c_181, bc: &BCinfo) -> f64 {
    let mut u: __SlateRecord_dtoa_c_181 = unsafe { core::mem::zeroed() };
    if (unsafe { (*(bc as *const BCinfo)).scale }) != 0
        && 107i32 > (((word0!(x) & 2146435072) >> 20i32) as i32)
    {
        // rv/2^bc->scale is subnormal
        unsafe {
            *((*core::ptr::addr_of_mut!(u)).L.as_mut_ptr() as *mut u32)
                .wrapping_offset(1 as isize) = 57671680u32;
        }
        unsafe {
            *((*core::ptr::addr_of_mut!(u)).L.as_mut_ptr() as *mut u32)
                .wrapping_offset(0 as isize) = 0;
        }
        return unsafe { u.d };
    } else {
        // x != 0.0
        return ulp(x);
    }
}

// The bigcomp function handles some hard cases for strtod, for inputs
// with more than STRTOD_DIGLIM digits.  It's called once an initial
// estimate for the double corresponding to the input string has
// already been obtained by the code in _Py_dg_strtod.
//
// The bigcomp function is only called after _Py_dg_strtod has found a
// double value rv such that either rv or rv + 1ulp represents the
// correctly rounded value corresponding to the original string.  It
// determines which of these two values is the correct one by
// computing the decimal digits of rv + 0.5ulp and comparing them with
// the corresponding digits of s0.
//
// In the following, write dv for the absolute value of the number represented
// by the input string.
//
// Inputs:
//
//   s0 points to the first significant digit of the input string.
//
//   rv is a (possibly scaled) estimate for the closest double value to the
//      value represented by the original input to _Py_dg_strtod.  If
//      bc->scale is nonzero, then rv/2^(bc->scale) is the approximation to
//      the input value.
//
//   bc is a struct containing information gathered during the parsing and
//      estimation steps of _Py_dg_strtod.  Description of fields follows:
//
//      bc->e0 gives the exponent of the input value, such that dv = (integer
//         given by the bd->nd digits of s0) * 10**e0
//
//      bc->nd gives the total number of significant digits of s0.  It will
//         be at least 1.
//
//      bc->nd0 gives the number of significant digits of s0 before the
//         decimal separator.  If there's no decimal separator, bc->nd0 ==
//         bc->nd.
//
//      bc->scale is the value used to scale rv to avoid doing arithmetic with
//         subnormal values.  It's either 0 or 2*P (=106).
//
// Outputs:
//
//   On successful exit, rv/2^(bc->scale) is the closest double to dv.
//
//   Returns 0 on success, -1 on failure (e.g., due to a failed malloc call).
fn bigcomp(rv: *mut __SlateRecord_dtoa_c_181, s0: *const i8, bc: &BCinfo) -> i32 {
    let mut b: *mut Bigint = unsafe { core::mem::zeroed() };
    let mut d: *mut Bigint = unsafe { core::mem::zeroed() };
    let mut b2: i32 = 0;
    let mut d2: i32 = 0;
    let mut dd: i32 = 0;
    let mut i: i32 = 0;
    let nd: i32 = unsafe { (*(bc as *const BCinfo)).nd };
    let nd0: i32 = unsafe { (*(bc as *const BCinfo)).nd0 };
    let mut odd: i32 = 0;
    let mut p2: i32 = 0;
    let p5: i32 = nd.wrapping_add(unsafe { (*(bc as *const BCinfo)).e0 });
    b = sd2b(rv, unsafe { (*(bc as *const BCinfo)).scale }, unsafe {
        &mut (*core::ptr::addr_of_mut!(p2))
    });
    if b.is_null() {
        return 1i32.wrapping_neg();
    }
    // record whether the lsb of rv/2^(bc->scale) is odd:  in the exact halfway
    // case, this is used for round to even.
    odd = ((unsafe { *((*b).x.as_mut_ptr() as *mut u32).wrapping_offset(0 as isize) }) & 1) as i32;
    // left shift b by 1 bit and or a 1 into the least significant bit;
    // this gives us b * 2**p2 = rv/2^(bc->scale) + 0.5 ulp.
    b = lshift(b, 1i32);
    if b.is_null() {
        return 1i32.wrapping_neg();
    }
    let __v0: *mut u32 = unsafe { (*b).x.as_mut_ptr() as *mut u32 }.wrapping_offset(0 as isize);
    unsafe {
        *__v0 = *__v0 | 1;
    }
    p2 = p2.wrapping_sub(1);
    p2 = p2.wrapping_sub(p5);
    d = i2b(1i32);
    if d.is_null() {
        Bfree(b);
        return 1i32.wrapping_neg();
    }
    // Arrange for convenient computation of quotients:
    // shift left if necessary so divisor has 4 leading 0 bits.
    if p5 > 0 {
        d = pow5mult(d, p5);
        if d.is_null() {
            Bfree(b);
            return 1i32.wrapping_neg();
        }
    } else {
        if p5 < 0 {
            b = pow5mult(b, p5.wrapping_neg());
            if b.is_null() {
                Bfree(d);
                return 1i32.wrapping_neg();
            }
        }
    }
    if p2 > 0 {
        b2 = p2;
        d2 = 0;
    } else {
        b2 = 0;
        d2 = p2.wrapping_neg();
    }
    i = dshift(d, d2);
    let __v8: i32 = b2.wrapping_add(i);
    b2 = __v8;
    if __v8 > 0 {
        b = lshift(b, b2);
        if b.is_null() {
            Bfree(d);
            return 1i32.wrapping_neg();
        }
    }
    let __v10: i32 = d2.wrapping_add(i);
    d2 = __v10;
    if __v10 > 0 {
        d = lshift(d, d2);
        if d.is_null() {
            Bfree(b);
            return 1i32.wrapping_neg();
        }
    }
    // Compare s0 with b/d: set dd to -1, 0, or 1 according as s0 < b/d, s0 ==
    // b/d, or s0 > b/d.  Here the digits of s0 are thought of as representing
    // a number in the range [0.1, 1).
    if cmp(b, d) >= 0 {
        dd = 1i32.wrapping_neg();
    } else {
        i = 0;
        loop {
            b = multadd(b, 10i32, 0i32);
            if b.is_null() {
                Bfree(d);
                return 1i32.wrapping_neg();
            }
            dd = ((unsafe {
                *s0.wrapping_offset((if i < nd0 { i } else { i.wrapping_add(1) }) as isize)
            }) as i32)
                .wrapping_sub(48)
                .wrapping_sub(quorem(b, d));
            i = i.wrapping_add(1);
            if dd != 0 {
                break;
            }
            if (unsafe { *((*b).x.as_mut_ptr() as *mut u32).wrapping_offset(0 as isize) }) == 0
                && (unsafe { (*b).wds }) == 1
            {
                // b/d == 0
                dd = (i < nd) as i32;
                break;
            }
            if i >= nd {
                // b/d != 0, but digits of s0 exhausted
                dd = 1i32.wrapping_neg();
                break;
            }
        }
    }
    // b/d >= 1
    Bfree(b);
    Bfree(d);
    if dd > 0 || dd == 0 && odd != 0 {
        let __v14: f64 = sulp(rv, bc);
        unsafe {
            (*rv).d = (*rv).d + __v14;
        }
    }
    0
}

fn rv_alloc(i: i32) -> *mut i8 {
    let mut k: i32 = 0;
    let mut j: u64 = core::mem::size_of::<u32>() as u64; // size_t so that j <<= 1 cannot overflow for i near INT_MAX
    while (core::mem::size_of::<Bigint>() as u64)
        .wrapping_sub(core::mem::size_of::<u32>() as u64)
        .wrapping_sub(core::mem::size_of::<i32>() as u64)
        .wrapping_add(j)
        <= ((i as u32) as u64)
    {
        k = k.wrapping_add(1);
        j <<= 1i32;
    }
    let r: *mut i32 = Balloc(k) as *mut i32;
    if r.is_null() {
        return core::ptr::null_mut::<i8>();
    }
    unsafe {
        *r = k;
    }
    r.wrapping_offset(1 as isize) as *mut i8
}

fn nrv_alloc(mut s: *const i8, rve: *mut *mut i8, n: i32) -> *mut i8 {
    let mut t: *mut i8 = unsafe { core::mem::zeroed() };
    let rv: *mut i8 = rv_alloc(n);
    if rv.is_null() {
        return core::ptr::null_mut::<i8>();
    }
    t = rv;
    loop {
        let __v2: *const i8 = s;
        s = __v2.wrapping_offset(1 as isize);
        let __v4: i8 = unsafe { *__v2 };
        unsafe {
            *t = __v4;
        }
        if __v4 == 0 {
            break;
        }
        let __v0: *mut i8 = t;
        t = __v0.wrapping_offset(1 as isize);
    }
    if !rve.is_null() {
        unsafe {
            *rve = t;
        }
    }
    rv
}

// freedtoa(s) must be used to free values s returned by dtoa
// when MULTIPLE_THREADS is #defined.  It should be used in all cases,
// but for consistency with earlier versions of dtoa, it is optional
// when MULTIPLE_THREADS is not defined.
#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_dg_freedtoa(s: *mut i8) {
    let b: *mut Bigint = (s as *mut i32).wrapping_offset(-(1 as isize)) as *mut Bigint;
    let __v0: i32 = unsafe { *(b as *mut i32) };
    unsafe {
        (*b).k = __v0;
    }
    unsafe {
        (*b).maxwds = 1i32 << __v0;
    }
    Bfree(b);
}

// dtoa for IEEE arithmetic (dmg): convert double to ASCII string.
//
// Inspired by "How to Print Floating-Point Numbers Accurately" by
// Guy L. Steele, Jr. and Jon L. White [Proc. ACM SIGPLAN '90, pp. 112-126].
//
// Modifications:
//      1. Rather than iterating, we use a simple numeric overestimate
//         to determine k = floor(log10(d)).  We scale relevant
//         quantities using O(log2(k)) rather than O(k) multiplications.
//      2. For some modes > 2 (corresponding to ecvt and fcvt), we don't
//         try to generate digits strictly left to right.  Instead, we
//         compute with fewer bits and propagate the carry if necessary
//         when rounding the final digit up.  This is often faster.
//      3. Under the assumption that input will be rounded nearest,
//         mode 0 renders 1e23 as 1e23 rather than 9.999999999999999e22.
//         That is, we allow equality in stopping tests when the
//         round-nearest rule will give the same floating-point value
//         as would satisfaction of the stopping test with strict
//         inequality.
//      4. We remove common factors of powers of 2 from relevant
//         quantities.
//      5. When converting floating-point integers less than 1e16,
//         we use floating-point arithmetic rather than resorting
//         to multiple-precision integers.
//      6. When asked to produce fewer than 15 digits, we first try
//         to get by with floating-point arithmetic; we resort to
//         multiple-precision integer arithmetic only if we cannot
//         guarantee that the floating-point calculation has given
//         the correctly rounded result.  For k requested digits and
//         "uniformly" distributed input, the probability is
//         something like 10^(k-15) that we must resort to the Long
//         calculation.
// Additional notes (METD): (1) returns NULL on failure.  (2) to avoid memory
// leakage, a successful call to _Py_dg_dtoa should always be matched by a
// call to _Py_dg_freedtoa.
#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_dg_dtoa(
    dd: f64,
    mut mode: i32,
    mut ndigits: i32,
    decpt: *mut i32,
    sign: *mut i32,
    rve: *mut *mut i8,
) -> *mut i8 {
    let mut __v28: bool = unsafe { core::mem::zeroed() };
    let mut __v33: i32 = unsafe { core::mem::zeroed() };
    let mut s0: *mut i8 = unsafe { core::mem::zeroed() };
    let mut s: *mut i8 = unsafe { core::mem::zeroed() };
    let mut ds: f64 = unsafe { core::mem::zeroed() };
    let mut u: core::mem::MaybeUninit<__SlateRecord_dtoa_c_181> = core::mem::MaybeUninit::uninit();
    let __slate_slot_197: *mut __SlateRecord_dtoa_c_181 =
        core::ptr::addr_of_mut!(u) as *mut __SlateRecord_dtoa_c_181;
    let mut eps: core::mem::MaybeUninit<__SlateRecord_dtoa_c_181> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_198: *mut __SlateRecord_dtoa_c_181 =
        core::ptr::addr_of_mut!(eps) as *mut __SlateRecord_dtoa_c_181;
    let mut d2: core::mem::MaybeUninit<__SlateRecord_dtoa_c_181> = core::mem::MaybeUninit::uninit();
    let __slate_slot_199: *mut __SlateRecord_dtoa_c_181 =
        core::ptr::addr_of_mut!(d2) as *mut __SlateRecord_dtoa_c_181;
    let mut S: *mut Bigint = unsafe { core::mem::zeroed() };
    let mut mhi: *mut Bigint = unsafe { core::mem::zeroed() };
    let mut mlo: *mut Bigint = unsafe { core::mem::zeroed() };
    let mut delta: *mut Bigint = unsafe { core::mem::zeroed() };
    let mut b: *mut Bigint = unsafe { core::mem::zeroed() };
    let mut denorm: i32 = unsafe { core::mem::zeroed() };
    let mut L: i32 = unsafe { core::mem::zeroed() };
    let mut try_quick: i32 = unsafe { core::mem::zeroed() };
    let mut spec_case: i32 = unsafe { core::mem::zeroed() };
    let mut s5: i32 = unsafe { core::mem::zeroed() };
    let mut s2: i32 = unsafe { core::mem::zeroed() };
    let mut m5: i32 = unsafe { core::mem::zeroed() };
    let mut m2: i32 = unsafe { core::mem::zeroed() };
    let mut leftright: i32 = unsafe { core::mem::zeroed() };
    let mut k_check: i32 = unsafe { core::mem::zeroed() };
    let mut k0: i32 = unsafe { core::mem::zeroed() };
    let mut k: i32 = unsafe { core::mem::zeroed() };
    let mut j1: i32 = unsafe { core::mem::zeroed() };
    let mut j: i32 = unsafe { core::mem::zeroed() };
    let mut ilim1: i32 = unsafe { core::mem::zeroed() };
    let mut ilim0: i32 = unsafe { core::mem::zeroed() };
    let mut ilim: i32 = unsafe { core::mem::zeroed() };
    let mut ieps: i32 = unsafe { core::mem::zeroed() };
    let mut i: i32 = unsafe { core::mem::zeroed() };
    let mut dig: i32 = unsafe { core::mem::zeroed() };
    let mut be: i32 = unsafe { core::mem::zeroed() };
    let mut b5: i32 = unsafe { core::mem::zeroed() };
    let mut b2: i32 = unsafe { core::mem::zeroed() };
    let mut bbits: i32 = unsafe { core::mem::zeroed() };
    unsafe {
        //  Arguments ndigits, decpt, sign are similar to those
        // of ecvt and fcvt; trailing zeros are suppressed from
        // the returned string.  If not null, *rve is set to point
        // to the end of the return value.  If d is +-Infinity or NaN,
        // then *decpt is set to 9999.
        //
        // mode:
        // 0 ==> shortest string that yields d when read in
        // and rounded to nearest.
        // 1 ==> like 0, but with Steele & White stopping rule;
        // e.g. with IEEE P754 arithmetic , mode 0 gives
        // 1e23 whereas mode 1 gives 9.999999999999999e22.
        // 2 ==> max(1,ndigits) significant digits.  This gives a
        // return value similar to that of ecvt, except
        // that trailing zeros are suppressed.
        // 3 ==> through ndigits past the decimal point.  This
        // gives a return value similar to that from fcvt,
        // except that trailing zeros are suppressed, and
        // ndigits can be negative.
        // 4,5 ==> similar to 2 and 3, respectively, but (in
        // round-nearest mode) with the tests of mode 0 to
        // possibly return a shorter string that rounds to d.
        // With IEEE arithmetic and compilation with
        // -DHonor_FLT_ROUNDS, modes 4 and 5 behave the same
        // as modes 2 and 3 when FLT_ROUNDS != 1.
        // 6-9 ==> Debugging modes similar to mode - 4:  don't try
        // fast floating-point estimate (if applicable).
        //
        // Values of mode other than 0-9 are treated as mode 0.
        //
        // Sufficient space is allocated to the return value
        // to hold the suppressed trailing zeros.
        // set pointers to NULL, to silence gcc compiler warnings and make
        // cleanup easier on error
        S = core::ptr::null_mut::<Bigint>();
        mhi = core::ptr::null_mut::<Bigint>();
        mlo = core::ptr::null_mut::<Bigint>();
        s0 = core::ptr::null_mut::<i8>();
        (*__slate_slot_197).d = dd;
        if word0!(core::ptr::addr_of_mut!(*__slate_slot_197)) & 2147483648 != 0 {
            // set sign for everything, including 0's and NaNs
            *sign = 1;
            let __v193: *mut u32 = ((*core::ptr::addr_of_mut!(*__slate_slot_197)).L.as_mut_ptr()
                as *mut u32)
                .wrapping_offset(1 as isize);
            let __v191: u32 = *__v193 & !2147483648;
            *__v193 = __v191;
        // clear sign bit
        } else {
            *sign = 0;
        }
        // quick return for Infinities, NaNs and zeros
        if word0!(core::ptr::addr_of_mut!(*__slate_slot_197)) & 2146435072 == 2146435072 {
            // Infinity or NaN
            *decpt = 9999;
            if word1!(core::ptr::addr_of_mut!(*__slate_slot_197)) == 0
                && word0!(core::ptr::addr_of_mut!(*__slate_slot_197)) & 1048575 == 0
            {
                return nrv_alloc(c"Infinity".as_ptr(), rve, 8i32);
            } else {
                return nrv_alloc(c"NaN".as_ptr(), rve, 3i32);
            }
        } else {
            if dval!(core::ptr::addr_of_mut!(*__slate_slot_197)) == 0.0f64 {
                *decpt = 1;
                return nrv_alloc(c"0".as_ptr(), rve, 1i32);
            } else {
                '__join_10: {
                    // compute k = floor(log10(d)).  The computation may leave k
                    // one too large, but should never leave k too small.
                    b = d2b(
                        core::ptr::addr_of_mut!(*__slate_slot_197),
                        &mut (*core::ptr::addr_of_mut!(be)),
                        &mut (*core::ptr::addr_of_mut!(bbits)),
                    );
                    if !b.is_null() {
                        let __v190: i32 = (word0!(core::ptr::addr_of_mut!(*__slate_slot_197))
                            >> 20i32
                            & 2047u32) as i32;
                        i = __v190;
                        if __v190 != 0 {
                            (*core::ptr::addr_of_mut!(*__slate_slot_199)).d =
                                dval!(core::ptr::addr_of_mut!(*__slate_slot_197));
                            let __v184: *mut u32 =
                                ((*core::ptr::addr_of_mut!(*__slate_slot_199)).L.as_mut_ptr()
                                    as *mut u32)
                                    .wrapping_offset(1 as isize);
                            *__v184 = *__v184 & 1048575;
                            let __v181: *mut u32 =
                                ((*core::ptr::addr_of_mut!(*__slate_slot_199)).L.as_mut_ptr()
                                    as *mut u32)
                                    .wrapping_offset(1 as isize);
                            *__v181 = *__v181 | 1072693248;
                            // log(x)       ~=~ log(1.5) + (x-1.5)/1.5
                            // log10(x)      =  log(x) / log(10)
                            //              ~=~ log(1.5)/log(10) + (x-1.5)/(1.5*log(10))
                            // log10(d) = (i-Bias)*log(2)/log(10) + log10(d2)
                            //
                            // This suggests computing an approximation k to log10(d) by
                            //
                            // k = (i - Bias)*0.301029995663981
                            //      + ( (d2-1.5)*0.289529654602168 + 0.176091259055681 );
                            //
                            // We want k to be too large rather than too small.
                            // The error in the first-order Taylor series approximation
                            // is in our favor, so we just round up the constant enough
                            // to compensate for any error in the multiplication of
                            // (i - Bias) by 0.301029995663981; since |i - Bias| <= 1077,
                            // and 1077 * 0.30103 * 2^-52 ~=~ 7.2e-14,
                            // adding 1e-13 to the constant term more than suffices.
                            // Hence we adjust the constant term to 0.1760912590558.
                            // (We could get a more accurate k by invoking log10,
                            //  but this is probably not worthwhile.)
                            i = i.wrapping_sub(1023);
                            denorm = 0;
                        } else {
                            // d is denormalized
                            i = bbits.wrapping_add(be).wrapping_add(1074i32);
                            let x: u32 = if i > 32 {
                                word0!(core::ptr::addr_of_mut!(*__slate_slot_197))
                                    << 64i32.wrapping_sub(i)
                                    | word1!(core::ptr::addr_of_mut!(*__slate_slot_197))
                                        >> i.wrapping_sub(32)
                            } else {
                                word1!(core::ptr::addr_of_mut!(*__slate_slot_197))
                                    << 32i32.wrapping_sub(i)
                            };
                            (*core::ptr::addr_of_mut!(*__slate_slot_199)).d = x as f64;
                            let __v189: *mut u32 =
                                ((*core::ptr::addr_of_mut!(*__slate_slot_199)).L.as_mut_ptr()
                                    as *mut u32)
                                    .wrapping_offset(1 as isize);
                            let __v188: u32 = *__v189;
                            *__v189 = __v188.wrapping_sub(32505856u32);
                            // adjust exponent
                            i = i.wrapping_sub(1075i32);
                            denorm = 1;
                        }
                        ds = (dval!(core::ptr::addr_of_mut!(*__slate_slot_199)) - 1.5f64)
                            * 0.289529654602168f64
                            + 0.1760912590558f64
                            + (i as f64) * 0.301029995663981f64;
                        k = ds as i32;
                        if ds < 0.0f64 && ds != (k as f64) {
                            k = k.wrapping_sub(1);
                        }
                        // want k = floor(ds)
                        k_check = 1;
                        if k >= 0 && k <= 22 {
                            if dval!(core::ptr::addr_of_mut!(*__slate_slot_197))
                                < tens.0[k as usize]
                            {
                                k = k.wrapping_sub(1);
                            }
                            k_check = 0;
                        }
                        j = bbits.wrapping_sub(i).wrapping_sub(1);
                        if j >= 0 {
                            b2 = 0;
                            s2 = j;
                        } else {
                            b2 = j.wrapping_neg();
                            s2 = 0;
                        }
                        if k >= 0 {
                            b5 = 0;
                            s5 = k;
                            s2 = s2.wrapping_add(k);
                        } else {
                            b2 = b2.wrapping_sub(k);
                            b5 = k.wrapping_neg();
                            s5 = 0;
                        }
                        if mode < 0 || mode > 9 {
                            mode = 0;
                        }
                        try_quick = 1;
                        if mode > 5 {
                            mode = mode.wrapping_sub(4);
                            try_quick = 0;
                        }
                        '__join_157: {
                            '__join_169: {
                                '__join_167: {
                                    leftright = 1;
                                    let __v166: i32 = 1i32.wrapping_neg();
                                    ilim1 = __v166;
                                    ilim = __v166; // Values for cases 0 and 1; done here to
                                    // silence erroneous "gcc -Wall" warning.
                                    let __t0: i32 = mode;
                                    if __t0 == 0 {
                                        break '__join_169;
                                    } else {
                                        if __t0 == 1 {
                                            break '__join_169;
                                        } else {
                                            if __t0 == 2 {
                                                leftright = 0;
                                                break '__join_167;
                                            } else {
                                                if __t0 == 4 {
                                                    break '__join_167;
                                                } else {
                                                    if __t0 == 3 {
                                                        leftright = 0;
                                                    } else {
                                                        if __t0 != 5 {
                                                            break '__join_157;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    if ndigits > 2147482623i32 {
                                        ndigits = 2147482623i32;
                                    } else {
                                        if ndigits < 2147482623i32.wrapping_neg() {
                                            ndigits = 2147482623i32.wrapping_neg();
                                        }
                                    }
                                    // -330 < k < 330 for any finite nonzero double.  Clamp ndigits so
                                    // that ndigits + k + 1 stays within int range; no double has
                                    // anywhere near this many decimal digits so the digits returned
                                    // are unaffected (*decpt saturates in the no_digits case).  Same
                                    // bound as DOUBLE_TO_STRING_PRECISION_MAX in pystrtod.c.
                                    i = ndigits.wrapping_add(k).wrapping_add(1);
                                    ilim = i;
                                    ilim1 = i.wrapping_sub(1);
                                    if i <= 0 {
                                        i = 1;
                                        break '__join_157;
                                    } else {
                                        break '__join_157;
                                    }
                                }
                                if ndigits <= 0 {
                                    ndigits = 1;
                                }
                                let __v165: i32 = ndigits;
                                i = __v165;
                                let __v164: i32 = __v165;
                                ilim1 = __v164;
                                ilim = __v164;
                                break '__join_157;
                            }
                            i = 18;
                            ndigits = 0;
                        }
                        s0 = rv_alloc(i);
                        if !s0.is_null() {
                            '__join_13: {
                                '__join_17: {
                                    '__join_70: {
                                        '__join_69: {
                                            '__join_106: {
                                                s = s0;
                                                if ilim >= 0 && ilim <= 14 && try_quick != 0 {
                                                    // Try to get by with floating-point arithmetic.
                                                    i = 0;
                                                    (*core::ptr::addr_of_mut!(*__slate_slot_199))
                                                        .d = dval!(core::ptr::addr_of_mut!(
                                                        *__slate_slot_197
                                                    ));
                                                    k0 = k;
                                                    ilim0 = ilim;
                                                    ieps = 2; // conservative
                                                    if k > 0 {
                                                        ds = tens.0[(k & 15) as usize];
                                                        j = k >> 4i32;
                                                        if j & 16 != 0 {
                                                            // prevent overflows
                                                            j &= 15i32;
                                                            let __v148: *mut __SlateRecord_dtoa_c_181 = core::ptr::addr_of_mut!(*__slate_slot_197);
                                                            (*__v148).d = (*__v148).d
                                                                / bigtens.0[4i32 as usize];
                                                            ieps = ieps.wrapping_add(1);
                                                        }
                                                        while j != 0 {
                                                            if j & 1 != 0 {
                                                                ieps = ieps.wrapping_add(1);
                                                                ds *= bigtens.0[i as usize];
                                                            }
                                                            j >>= 1i32;
                                                            i = i.wrapping_add(1);
                                                        }
                                                        let __v135: *mut __SlateRecord_dtoa_c_181 = core::ptr::addr_of_mut!(
                                                            *__slate_slot_197
                                                        );
                                                        (*__v135).d = (*__v135).d / ds;
                                                    } else {
                                                        let __v163: i32 = k.wrapping_neg();
                                                        j1 = __v163;
                                                        if __v163 != 0 {
                                                            let __v162: *mut __SlateRecord_dtoa_c_181 = core::ptr::addr_of_mut!(*__slate_slot_197);
                                                            (*__v162).d = (*__v162).d
                                                                * tens.0[(j1 & 15) as usize];
                                                            j = j1 >> 4i32;
                                                            while j != 0 {
                                                                if j & 1 != 0 {
                                                                    ieps = ieps.wrapping_add(1);
                                                                    let __v157: *mut __SlateRecord_dtoa_c_181 = core::ptr::addr_of_mut!(*__slate_slot_197);
                                                                    (*__v157).d = (*__v157).d
                                                                        * bigtens.0[i as usize];
                                                                }
                                                                j >>= 1i32;
                                                                i = i.wrapping_add(1);
                                                            }
                                                        }
                                                    }
                                                    '__join_119: {
                                                        if k_check != 0
                                                            && dval!(core::ptr::addr_of_mut!(
                                                                *__slate_slot_197
                                                            )) < 1.0f64
                                                            && ilim > 0
                                                        {
                                                            if ilim1 <= 0 {
                                                                break '__join_119;
                                                            } else {
                                                                ilim = ilim1;
                                                                k = k.wrapping_sub(1);
                                                                let __v130: *mut __SlateRecord_dtoa_c_181 = core::ptr::addr_of_mut!(*__slate_slot_197);
                                                                let __v128: f64 =
                                                                    (*__v130).d * 10.0f64;
                                                                (*__v130).d = __v128;
                                                                ieps = ieps.wrapping_add(1);
                                                            }
                                                        }
                                                        (*core::ptr::addr_of_mut!(
                                                            *__slate_slot_198
                                                        ))
                                                        .d = (ieps as f64)
                                                            * dval!(core::ptr::addr_of_mut!(
                                                                *__slate_slot_197
                                                            ))
                                                            + 7.0f64;
                                                        let __v125: *mut u32 =
                                                            ((*core::ptr::addr_of_mut!(
                                                                *__slate_slot_198
                                                            ))
                                                            .L
                                                            .as_mut_ptr()
                                                                as *mut u32)
                                                                .wrapping_offset(1 as isize);
                                                        let __v124: u32 = *__v125;
                                                        *__v125 = __v124.wrapping_sub(54525952u32);
                                                        if ilim == 0 {
                                                            mhi = core::ptr::null_mut::<Bigint>();
                                                            S = core::ptr::null_mut::<Bigint>();
                                                            let __v122: *mut __SlateRecord_dtoa_c_181 = core::ptr::addr_of_mut!(*__slate_slot_197);
                                                            let __v120: f64 = (*__v122).d - 5.0f64;
                                                            (*__v122).d = __v120;
                                                            if dval!(core::ptr::addr_of_mut!(
                                                                *__slate_slot_197
                                                            )) > dval!(core::ptr::addr_of_mut!(
                                                                *__slate_slot_198
                                                            )) {
                                                                break '__join_69;
                                                            } else {
                                                                if dval!(core::ptr::addr_of_mut!(
                                                                    *__slate_slot_197
                                                                )) < -dval!(
                                                                    core::ptr::addr_of_mut!(
                                                                        *__slate_slot_198
                                                                    )
                                                                ) {
                                                                    break '__join_70;
                                                                }
                                                            }
                                                        } else {
                                                            if leftright != 0 {
                                                                // Use Steele & White method of only
                                                                // generating digits needed.
                                                                (*core::ptr::addr_of_mut!(
                                                                    *__slate_slot_198
                                                                ))
                                                                .d = 0.5f64
                                                                    / tens.0[ilim.wrapping_sub(1)
                                                                        as usize]
                                                                    - dval!(
                                                                        core::ptr::addr_of_mut!(
                                                                            *__slate_slot_198
                                                                        )
                                                                    );
                                                                i = 0;
                                                                loop {
                                                                    L = dval!(
                                                                        core::ptr::addr_of_mut!(
                                                                            *__slate_slot_197
                                                                        )
                                                                    )
                                                                        as i32;
                                                                    let __v102: *mut __SlateRecord_dtoa_c_181 = core::ptr::addr_of_mut!(*__slate_slot_197);
                                                                    (*__v102).d =
                                                                        (*__v102).d - (L as f64);
                                                                    let __v99: *mut i8 = s;
                                                                    s = __v99.wrapping_offset(
                                                                        1 as isize,
                                                                    );
                                                                    *__v99 =
                                                                        48i32.wrapping_add(L) as i8;
                                                                    if dval!(
                                                                        core::ptr::addr_of_mut!(
                                                                            *__slate_slot_197
                                                                        )
                                                                    ) < dval!(
                                                                        core::ptr::addr_of_mut!(
                                                                            *__slate_slot_198
                                                                        )
                                                                    ) {
                                                                        break '__join_13;
                                                                    } else {
                                                                        if 1.0f64 - dval!(core::ptr::addr_of_mut!(*__slate_slot_197)) < dval!(core::ptr::addr_of_mut!(*__slate_slot_198)) {
break '__join_106;
} else {
let __v96: i32 = i.wrapping_add(1);
i = __v96;
if __v96 >= ilim {
break;
} else {
let __v95: *mut __SlateRecord_dtoa_c_181 = core::ptr::addr_of_mut!(*__slate_slot_198);
let __v93: f64 = (*__v95).d * 10.0f64;
(*__v95).d = __v93;
let __v92: *mut __SlateRecord_dtoa_c_181 = core::ptr::addr_of_mut!(*__slate_slot_197);
let __v90: f64 = (*__v92).d * 10.0f64;
(*__v92).d = __v90;
}
}
                                                                    }
                                                                }
                                                            } else {
                                                                // Generate ilim digits, then fix them up.
                                                                let __v119: *mut __SlateRecord_dtoa_c_181 = core::ptr::addr_of_mut!(*__slate_slot_198);
                                                                (*__v119).d = (*__v119).d
                                                                    * tens.0[ilim.wrapping_sub(1)
                                                                        as usize];
                                                                i = 1;
                                                                loop {
                                                                    L = dval!(
                                                                        core::ptr::addr_of_mut!(
                                                                            *__slate_slot_197
                                                                        )
                                                                    )
                                                                        as i32;
                                                                    let __v116: *mut __SlateRecord_dtoa_c_181 = core::ptr::addr_of_mut!(*__slate_slot_197);
                                                                    let __v114: f64 =
                                                                        (*__v116).d - (L as f64);
                                                                    (*__v116).d = __v114;
                                                                    if __v114 == 0.0f64 {
                                                                        ilim = i;
                                                                    }
                                                                    let __v113: *mut i8 = s;
                                                                    s = __v113.wrapping_offset(
                                                                        1 as isize,
                                                                    );
                                                                    *__v113 =
                                                                        48i32.wrapping_add(L) as i8;
                                                                    if i == ilim {
                                                                        break;
                                                                    } else {
                                                                        i = i.wrapping_add(1);
                                                                        let __v105: *mut __SlateRecord_dtoa_c_181 = core::ptr::addr_of_mut!(*__slate_slot_197);
                                                                        let __v103: f64 =
                                                                            (*__v105).d * 10.0f64;
                                                                        (*__v105).d = __v103;
                                                                    }
                                                                }
                                                                if dval!(core::ptr::addr_of_mut!(
                                                                    *__slate_slot_197
                                                                )) > 0.5f64
                                                                    + dval!(
                                                                        core::ptr::addr_of_mut!(
                                                                            *__slate_slot_198
                                                                        )
                                                                    )
                                                                {
                                                                    break '__join_106;
                                                                } else {
                                                                    if dval!(
                                                                        core::ptr::addr_of_mut!(
                                                                            *__slate_slot_197
                                                                        )
                                                                    ) < 0.5f64
                                                                        - dval!(
                                                                            core::ptr::addr_of_mut!(
                                                                                *__slate_slot_198
                                                                            )
                                                                        )
                                                                    {
                                                                        loop {
                                                                            let __v111: *mut i8 = s;
                                                                            let __v110: *mut i8 = __v111.wrapping_offset(-(1 as isize));
                                                                            s = __v110;
                                                                            if (*__v110 as i32)
                                                                                != 48
                                                                            {
                                                                                break;
                                                                            }
                                                                        }
                                                                        let __v109: *mut i8 = s;
                                                                        s = __v109.wrapping_offset(
                                                                            1 as isize,
                                                                        );
                                                                        break '__join_13;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    s = s0;
                                                    (*core::ptr::addr_of_mut!(*__slate_slot_197))
                                                        .d = dval!(core::ptr::addr_of_mut!(
                                                        *__slate_slot_199
                                                    ));
                                                    k = k0;
                                                    ilim = ilim0;
                                                }
                                                // Do we have a "small" integer?
                                                if be >= 0 && k <= 14 {
                                                    // Yes.
                                                    ds = tens.0[k as usize];
                                                    if ndigits < 0 && ilim <= 0 {
                                                        mhi = core::ptr::null_mut::<Bigint>();
                                                        S = core::ptr::null_mut::<Bigint>();
                                                        if ilim < 0
                                                            || dval!(core::ptr::addr_of_mut!(
                                                                *__slate_slot_197
                                                            )) <= (5i32 as f64) * ds
                                                        {
                                                            break '__join_70;
                                                        } else {
                                                            break '__join_69;
                                                        }
                                                    } else {
                                                        i = 1;
                                                        loop {
                                                            L = (dval!(core::ptr::addr_of_mut!(
                                                                *__slate_slot_197
                                                            )) / ds)
                                                                as i32;
                                                            let __v89: *mut __SlateRecord_dtoa_c_181 = core::ptr::addr_of_mut!(*__slate_slot_197);
                                                            (*__v89).d =
                                                                (*__v89).d - (L as f64) * ds;
                                                            let __v86: *mut i8 = s;
                                                            s = __v86.wrapping_offset(1 as isize);
                                                            *__v86 = 48i32.wrapping_add(L) as i8;
                                                            if dval!(core::ptr::addr_of_mut!(
                                                                *__slate_slot_197
                                                            )) == 0.0f64
                                                            {
                                                                break '__join_13;
                                                            } else {
                                                                if i == ilim {
                                                                    break;
                                                                } else {
                                                                    i = i.wrapping_add(1);
                                                                    let __v68: *mut __SlateRecord_dtoa_c_181 = core::ptr::addr_of_mut!(*__slate_slot_197);
                                                                    let __v66: f64 =
                                                                        (*__v68).d * 10.0f64;
                                                                    (*__v68).d = __v66;
                                                                }
                                                            }
                                                        }
                                                        let __v84: *mut __SlateRecord_dtoa_c_181 = core::ptr::addr_of_mut!(
                                                            *__slate_slot_197
                                                        );
                                                        let __v83: f64 = (*__v84).d;
                                                        let __v82: f64 = __v83
                                                            + dval!(core::ptr::addr_of_mut!(
                                                                *__slate_slot_197
                                                            ));
                                                        (*__v84).d = __v82;
                                                        if !(dval!(core::ptr::addr_of_mut!(
                                                            *__slate_slot_197
                                                        )) > ds
                                                            || dval!(core::ptr::addr_of_mut!(
                                                                *__slate_slot_197
                                                            )) == ds
                                                                && L & 1 != 0)
                                                        {
                                                            // Strip trailing zeros. This branch was missing from the
                                                            // original dtoa.c, leading to surplus trailing zeros in
                                                            // some cases. See bugs.python.org/issue40780.
                                                            loop {
                                                                if s > s0
                                                                    && (*s.wrapping_offset(
                                                                        1i32.wrapping_neg()
                                                                            as isize,
                                                                    )
                                                                        as i32)
                                                                        == 48
                                                                {
                                                                    let __v81: *mut i8 = s;
                                                                    s = __v81.wrapping_offset(
                                                                        -(1 as isize),
                                                                    );
                                                                } else {
                                                                    break '__join_13;
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    m2 = b2;
                                                    m5 = b5;
                                                    if leftright != 0 {
                                                        i = if denorm != 0 {
                                                            be.wrapping_add(1075i32)
                                                        } else {
                                                            54i32.wrapping_sub(bbits)
                                                        };
                                                        b2 = b2.wrapping_add(i);
                                                        s2 = s2.wrapping_add(i);
                                                        mhi = i2b(1i32);
                                                        if mhi.is_null() {
                                                            break '__join_10;
                                                        }
                                                    }
                                                    if m2 > 0 && s2 > 0 {
                                                        i = if m2 < s2 { m2 } else { s2 };
                                                        b2 = b2.wrapping_sub(i);
                                                        m2 = m2.wrapping_sub(i);
                                                        s2 = s2.wrapping_sub(i);
                                                    }
                                                    if b5 > 0 {
                                                        if leftright != 0 {
                                                            if m5 > 0 {
                                                                mhi = pow5mult(mhi, m5);
                                                                if mhi.is_null() {
                                                                    break '__join_10;
                                                                } else {
                                                                    let b1: *mut Bigint =
                                                                        mult(mhi, b);
                                                                    Bfree(b);
                                                                    b = b1;
                                                                    if b.is_null() {
                                                                        break '__join_10;
                                                                    }
                                                                }
                                                            }
                                                            let __v55: i32 = b5.wrapping_sub(m5);
                                                            j = __v55;
                                                            if __v55 != 0 {
                                                                b = pow5mult(b, j);
                                                                if b.is_null() {
                                                                    break '__join_10;
                                                                }
                                                            }
                                                        } else {
                                                            b = pow5mult(b, b5);
                                                            if b.is_null() {
                                                                break '__join_10;
                                                            }
                                                        }
                                                    }
                                                    S = i2b(1i32);
                                                    if S.is_null() {
                                                        break '__join_10;
                                                    } else {
                                                        if s5 > 0 {
                                                            S = pow5mult(S, s5);
                                                            if S.is_null() {
                                                                break '__join_10;
                                                            }
                                                        }
                                                        // Check for special case that d is a normalized power of 2.
                                                        spec_case = 0;
                                                        if mode < 2 || leftright != 0 {
                                                            if word1!(core::ptr::addr_of_mut!(
                                                                *__slate_slot_197
                                                            )) == 0
                                                                && word0!(core::ptr::addr_of_mut!(
                                                                    *__slate_slot_197
                                                                )) & 1048575
                                                                    == 0
                                                                && word0!(core::ptr::addr_of_mut!(
                                                                    *__slate_slot_197
                                                                )) & 2145386496u32
                                                                    != 0
                                                            {
                                                                // The special case
                                                                b2 = b2.wrapping_add(1);
                                                                s2 = s2.wrapping_add(1);
                                                                spec_case = 1;
                                                            }
                                                        }
                                                        // Arrange for convenient computation of quotients:
                                                        // shift left if necessary so divisor has 4 leading 0 bits.
                                                        //
                                                        // Perhaps we should just compute leading 28 bits of S once
                                                        // and for all and pass them and a shift to quorem, so it
                                                        // can do shifts and ors to compute the numerator for q.
                                                        i = dshift(S, s2);
                                                        b2 = b2.wrapping_add(i);
                                                        m2 = m2.wrapping_add(i);
                                                        s2 = s2.wrapping_add(i);
                                                        if b2 > 0 {
                                                            b = lshift(b, b2);
                                                            if b.is_null() {
                                                                break '__join_10;
                                                            }
                                                        }
                                                        if s2 > 0 {
                                                            S = lshift(S, s2);
                                                            if S.is_null() {
                                                                break '__join_10;
                                                            }
                                                        }
                                                        if k_check != 0 {
                                                            if cmp(b, S) < 0 {
                                                                k = k.wrapping_sub(1);
                                                                b = multadd(b, 10i32, 0i32); // we botched the k estimate
                                                                if b.is_null() {
                                                                    break '__join_10;
                                                                } else {
                                                                    if leftright != 0 {
                                                                        mhi = multadd(
                                                                            mhi, 10i32, 0i32,
                                                                        );
                                                                        if mhi.is_null() {
                                                                            break '__join_10;
                                                                        }
                                                                    }
                                                                    ilim = ilim1;
                                                                }
                                                            }
                                                        }
                                                        if ilim <= 0 && (mode == 3 || mode == 5) {
                                                            if ilim < 0 {
                                                                // no digits, fcvt style
                                                                break '__join_70;
                                                            } else {
                                                                S = multadd(S, 5i32, 0i32);
                                                                if S.is_null() {
                                                                    break '__join_10;
                                                                } else {
                                                                    if cmp(b, S) <= 0 {
                                                                        break '__join_70;
                                                                    } else {
                                                                        break '__join_69;
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            '__join_19: {
                                                                '__join_26: {
                                                                    if leftright != 0 {
                                                                        if m2 > 0 {
                                                                            mhi = lshift(mhi, m2);
                                                                            if mhi.is_null() {
                                                                                break '__join_10;
                                                                            }
                                                                        }
                                                                        // Compute mlo -- check for special case
                                                                        // that d is a normalized power of 2.
                                                                        mlo = mhi;
                                                                        if spec_case != 0 {
                                                                            mhi = Balloc((*mhi).k);
                                                                            if mhi.is_null() {
                                                                                break '__join_10;
                                                                            } else {
                                                                                Bcopy!(mhi, mlo);
                                                                                mhi = lshift(
                                                                                    mhi, 1i32,
                                                                                );
                                                                                if mhi.is_null() {
                                                                                    break '__join_10;
                                                                                }
                                                                            }
                                                                        }
                                                                        i = 1;
                                                                        '__join_35: {
                                                                            '__join_51: {
                                                                                '__join_46: {
                                                                                    loop {
                                                                                        dig = quorem(b, S).wrapping_add(48);
                                                                                        // Do we yet have the shortest decimal string
                                                                                        // that will round to d?
                                                                                        j = cmp(
                                                                                            b, mlo,
                                                                                        );
                                                                                        delta =
                                                                                            diff(
                                                                                                S,
                                                                                                mhi,
                                                                                            );
                                                                                        if delta.is_null() {
break '__join_10;
} else {
if (*delta).sign != 0 {
__v33 = 1;
} else {
__v33 = cmp(b, delta);
}
j1 = __v33;
Bfree(delta);
if j1 == 0 && mode != 1 && word1!(core::ptr::addr_of_mut!(*__slate_slot_197)) & 1 == 0 {
break '__join_51;
} else {
if j < 0 || j == 0 && mode != 1 && word1!(core::ptr::addr_of_mut!(*__slate_slot_197)) & 1 == 0 {
break '__join_46;
} else {
if j1 > 0 {
break;
} else {
let __v19: *mut i8 = s;
s = __v19.wrapping_offset(1 as isize);
*__v19 = dig as i8;
if i == ilim {
break '__join_26;
} else {
b = multadd(b, 10i32, 0i32);
if b.is_null() {
break '__join_10;
} else {
if mlo == mhi {
let __v17: *mut Bigint = multadd(mhi, 10i32, 0i32);
mhi = __v17;
mlo = __v17;
if mlo.is_null() {
break '__join_10;
}
} else {
mlo = multadd(mlo, 10i32, 0i32);
if mlo.is_null() {
break '__join_10;
} else {
mhi = multadd(mhi, 10i32, 0i32);
if mhi.is_null() {
break '__join_10;
}
}
}
i = i.wrapping_add(1);
}
}
}
}
}
}
                                                                                    }
                                                                                    if dig == 57 {
                                                                                        // possible if i == 1
                                                                                        break '__join_35;
                                                                                    } else {
                                                                                        let __v21: *mut i8 = s;
                                                                                        s = __v21.wrapping_offset(1 as isize);
                                                                                        *__v21 = dig.wrapping_add(1) as i8;
                                                                                        break '__join_17;
                                                                                    }
                                                                                }
                                                                                if !(*((*b).x.as_mut_ptr() as *mut u32).wrapping_offset(0 as isize) == 0 && (*b).wds <= 1) {
if j1 > 0 {
b = lshift(b, 1i32);
if b.is_null() {
break '__join_10;
} else {
j1 = cmp(b, S);
if j1 > 0 || j1 == 0 && dig & 1 != 0 {
let __v27: i32 = dig;
dig = __v27.wrapping_add(1);
__v28 = __v27 == 57;
} else {
__v28 = false;
}
if __v28 {
break '__join_35;
}
}
}
}
                                                                                let __v25: *mut i8 =
                                                                                    s;
                                                                                s = __v25.wrapping_offset(1 as isize);
                                                                                *__v25 = dig as i8;
                                                                                break '__join_17;
                                                                            }
                                                                            if dig != 57 {
                                                                                if j > 0 {
                                                                                    dig = dig.wrapping_add(1);
                                                                                }
                                                                                let __v30: *mut i8 =
                                                                                    s;
                                                                                s = __v30.wrapping_offset(1 as isize);
                                                                                *__v30 = dig as i8;
                                                                                break '__join_17;
                                                                            }
                                                                        }
                                                                        let __v23: *mut i8 = s;
                                                                        s = __v23.wrapping_offset(
                                                                            1 as isize,
                                                                        );
                                                                        *__v23 = 57;
                                                                        break '__join_19;
                                                                    } else {
                                                                        i = 1;
                                                                        loop {
                                                                            let __v38: i32 =
                                                                                quorem(b, S)
                                                                                    .wrapping_add(
                                                                                        48,
                                                                                    );
                                                                            dig = __v38;
                                                                            let __v37: *mut i8 = s;
                                                                            s = __v37
                                                                                .wrapping_offset(
                                                                                    1 as isize,
                                                                                );
                                                                            *__v37 = __v38 as i8;
                                                                            if *((*b).x.as_mut_ptr()
                                                                                as *mut u32)
                                                                                .wrapping_offset(
                                                                                    0 as isize,
                                                                                )
                                                                                == 0
                                                                                && (*b).wds <= 1
                                                                            {
                                                                                break '__join_17;
                                                                            } else {
                                                                                if i >= ilim {
                                                                                    break;
                                                                                } else {
                                                                                    b = multadd(
                                                                                        b, 10i32,
                                                                                        0i32,
                                                                                    );
                                                                                    if b.is_null() {
                                                                                        break '__join_10;
                                                                                    } else {
                                                                                        i = i.wrapping_add(1);
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                // Round off last digit
                                                                b = lshift(b, 1i32);
                                                                if b.is_null() {
                                                                    break '__join_10;
                                                                } else {
                                                                    j = cmp(b, S);
                                                                    if !(j > 0
                                                                        || j == 0 && dig & 1 != 0)
                                                                    {
                                                                        loop {
                                                                            let __v14: *mut i8 = s;
                                                                            let __v13: *mut i8 = __v14.wrapping_offset(-(1 as isize));
                                                                            s = __v13;
                                                                            if (*__v13 as i32) != 48
                                                                            {
                                                                                break;
                                                                            }
                                                                        }
                                                                        let __v12: *mut i8 = s;
                                                                        s = __v12.wrapping_offset(
                                                                            1 as isize,
                                                                        );
                                                                        break '__join_17;
                                                                    }
                                                                }
                                                            }
                                                            '__join_20: {
                                                                loop {
                                                                    let __v6: *mut i8 = s;
                                                                    let __v5: *mut i8 = __v6
                                                                        .wrapping_offset(
                                                                            -(1 as isize),
                                                                        );
                                                                    s = __v5;
                                                                    if (*__v5 as i32) == 57 {
                                                                        if s == s0 {
                                                                            break '__join_20;
                                                                        }
                                                                    } else {
                                                                        break;
                                                                    }
                                                                }
                                                                let __v4: *mut i8 = s;
                                                                s = __v4
                                                                    .wrapping_offset(1 as isize);
                                                                *__v4 = (*__v4 as i32)
                                                                    .wrapping_add(1)
                                                                    as i8;
                                                                break '__join_17;
                                                            }
                                                            k = k.wrapping_add(1);
                                                            let __v8: *mut i8 = s;
                                                            s = __v8.wrapping_offset(1 as isize);
                                                            *__v8 = 49;
                                                            break '__join_17;
                                                        }
                                                    }
                                                }
                                            }
                                            '__join_105: {
                                                loop {
                                                    let __v77: *mut i8 = s;
                                                    let __v76: *mut i8 =
                                                        __v77.wrapping_offset(-(1 as isize));
                                                    s = __v76;
                                                    if (*__v76 as i32) == 57 {
                                                        if s == s0 {
                                                            break;
                                                        }
                                                    } else {
                                                        break '__join_105;
                                                    }
                                                }
                                                k = k.wrapping_add(1);
                                                *s = 48;
                                            }
                                            let __v75: *mut i8 = s;
                                            s = __v75.wrapping_offset(1 as isize);
                                            *__v75 = (*__v75 as i32).wrapping_add(1) as i8;
                                            break '__join_13;
                                        }
                                        let __v42: *mut i8 = s;
                                        s = __v42.wrapping_offset(1 as isize);
                                        *__v42 = 49;
                                        k = k.wrapping_add(1);
                                        break '__join_17;
                                    }
                                    k = 1i32.wrapping_neg().wrapping_sub(ndigits);
                                }
                                Bfree(S);
                                if !mhi.is_null() {
                                    if !mlo.is_null() && mlo != mhi {
                                        Bfree(mlo);
                                    }
                                    Bfree(mhi);
                                }
                            }
                            Bfree(b);
                            *s = 0;
                            *decpt = k.wrapping_add(1);
                            if !rve.is_null() {
                                *rve = s;
                            }
                            return s0;
                        }
                    }
                }
                if !S.is_null() {
                    Bfree(S);
                }
                if !mlo.is_null() && mlo != mhi {
                    Bfree(mlo);
                }
                if !mhi.is_null() {
                    Bfree(mhi);
                }
                if !b.is_null() {
                    Bfree(b);
                }
                if !s0.is_null() {
                    _Py_dg_freedtoa(s0);
                }
                return core::ptr::null_mut::<i8>();
            }
        }
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyDtoa_Init(interp: *mut _is) -> __SlateRecord_initconfig_h_19 {
    let p5s: *mut *mut Bigint = unsafe { (*interp).dtoa.p5s.as_mut_ptr() as *mut *mut Bigint };
    // 5**4 = 625
    let mut p5: *mut Bigint = i2b(625i32);
    if p5.is_null() {
        return unsafe { PyStatus_NoMemory() };
    }
    unsafe {
        *p5s.wrapping_offset(0 as isize) = p5;
    }
    // compute 5**8, 5**16, 5**32, ..., 5**512
    for i in 1i64..8i64 {
        p5 = mult(p5, p5);
        if p5.is_null() {
            return unsafe { PyStatus_NoMemory() };
        }
        unsafe {
            *p5s.wrapping_offset(i as isize) = p5;
        }
    }
    return unsafe { PyStatus_Ok() };
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyDtoa_Fini(interp: *mut _is) {
    let p5s: *mut *mut Bigint = unsafe { (*interp).dtoa.p5s.as_mut_ptr() as *mut *mut Bigint };
    for i in 0i64..8i64 {
        let p5: *mut Bigint = unsafe { *p5s.wrapping_offset(i as isize) };
        unsafe {
            *p5s.wrapping_offset(i as isize) = core::ptr::null_mut::<Bigint>();
        }
        Bfree(p5);
    }
}
