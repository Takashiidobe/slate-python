//! Path configuration like module_search_path (sys.path)
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
//! This limitation is for performance and simplicity. If needed it can be
//! removed (with effort).
//! Remote debugger support
//! PY_TIMEOUT_T is the integral type used to specify timeouts when waiting
//!
//! on a lock (see PyThread_acquire_lock_timed() below).
//! Almost all errors causing Python initialization to fail
//!
//! other fields are set to 0
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
macro_rules! CLEAR__2 {
    ($ATTR:ident) => {
        loop {
            unsafe { _PyMem_DefaultRawFree(_Py_path_config.$ATTR as *mut ()) };
            unsafe {
                _Py_path_config.$ATTR = core::ptr::null_mut::<i32>();
            }
            if true {
                break;
            }
        }
    };
}

macro_rules! COPY_INT {
    ($ATTR:ident, $config:expr) => {
        loop {
            if (unsafe { _Py_path_config.$ATTR }) >= 0 && (unsafe { (*$config).$ATTR }) <= 0 {
                unsafe {
                    (*$config).$ATTR = _Py_path_config.$ATTR;
                }
            }
            if true {
                break;
            }
        }
    };
}

macro_rules! COPY_INT__2 {
    ($ATTR:ident, $config:expr) => {
        loop {
            if (unsafe { (*$config).$ATTR }) > 0 {
                unsafe {
                    _Py_path_config.$ATTR = (*$config).$ATTR;
                }
            }
            if true {
                break;
            }
        }
    };
}

macro_rules! Py_ARRAY_LENGTH {
    () => {
        (core::mem::size_of::<[i32; 4097]>() as u64) / (core::mem::size_of::<i32>() as u64)
    };
}

macro_rules! Py_ARRAY_LENGTH__9 {
    () => {
        (core::mem::size_of::<[i32; 4096]>() as u64) / (core::mem::size_of::<i32>() as u64)
    };
}

macro_rules! _PyStatus_EXCEPTION {
    ($err:expr) => {
        ($err._type as u32) != 0
    };
}

macro_rules! _PyStatus_GET_FUNC__75 {
    () => {
        c"_PyPathConfig_UpdateGlobal".as_ptr() as *mut i8
    };
}

unsafe extern "C" {
    fn wcscpy(arg0: *mut i32, arg1: *const i32) -> *mut i32;
    fn wcsncpy(arg0: *mut i32, arg1: *const i32, arg2: u64) -> *mut i32;
    fn wcscmp(arg0: *const i32, arg1: *const i32) -> i32;
    fn wcschr(arg0: *const i32, arg1: i32) -> *mut i32;
    fn wcsrchr(arg0: *const i32, arg1: i32) -> *mut i32;
    fn PyUnicode_FromWideChar(arg0: *const i32, arg1: i64) -> *mut _object;
    fn _Py_FatalErrorFunc(arg0: *const i8, arg1: *const i8);
    fn PyConfig_SetString(
        arg0: *mut PyConfig,
        arg1: *mut *mut i32,
        arg2: *const i32,
    ) -> __SlateRecord_initconfig_h_19;
    fn _PyMem_DefaultRawMalloc(arg0: u64) -> *mut ();
    fn _PyMem_DefaultRawFree(arg0: *mut ());
    fn _PyMem_DefaultRawWcsdup(arg0: *const i32) -> *mut i32;
    fn _Py_wreadlink(arg0: *const i32, arg1: *mut i32, arg2: u64) -> i32;
    fn _Py_wrealpath(arg0: *const i32, arg1: *mut i32, arg2: u64) -> *mut i32;
    fn _Py_wgetcwd(arg0: *mut i32, arg1: u64) -> *mut i32;
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

// _PyStatus_OK()
// _Py_wgetcwd()
// _PyMem_DefaultRawFree()
// PyMarshal_ReadObjectFromString
// DELIM
// External interface
/// Stored values set by C API functions
#[repr(C)]
#[derive(Clone, Copy)]
struct _PyPathConfig {
    /// Full path to the Python program
    program_full_path: *mut i32,
    prefix: *mut i32,
    exec_prefix: *mut i32,
    stdlib_dir: *mut i32,
    /// Set by Py_SetPath
    module_search_path: *mut i32,
    /// Set by _PyPathConfig_UpdateGlobal
    calculated_module_search_path: *mut i32,
    /// Python program name
    program_name: *mut i32,
    /// Set by Py_SetPythonHome() or PYTHONHOME environment variable
    home: *mut i32,
    _is_python_build: i32,
}

#[repr(C, align(16))]
struct __SlateAlign16<T>(T);

unsafe impl<T> Sync for __SlateAlign16<T> {}

#[unsafe(no_mangle)]
static mut _Py_path_config: _PyPathConfig = {
    let mut __t0: _PyPathConfig = unsafe { core::mem::zeroed() };
    __t0.module_search_path = core::ptr::null_mut::<i32>();
    __t0._is_python_build = 0i32;
    __t0
};

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyPathConfig_GetGlobalModuleSearchPath() -> *const i32 {
    (unsafe { _Py_path_config.module_search_path }) as *const i32
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyPathConfig_ClearGlobal() {
    CLEAR__2!(program_full_path);
    CLEAR__2!(prefix);
    CLEAR__2!(exec_prefix);
    CLEAR__2!(stdlib_dir);
    CLEAR__2!(module_search_path);
    CLEAR__2!(calculated_module_search_path);
    CLEAR__2!(program_name);
    CLEAR__2!(home);
    unsafe {
        _Py_path_config._is_python_build = 0;
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyPathConfig_ReadGlobal(
    config: *mut PyConfig,
) -> __SlateRecord_initconfig_h_19 {
    let mut __slate_compound_2: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut status: core::mem::MaybeUninit<__SlateRecord_initconfig_h_19> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_0: *mut __SlateRecord_initconfig_h_19 =
        core::ptr::addr_of_mut!(status) as *mut __SlateRecord_initconfig_h_19;
    unsafe {
        core::ptr::write(__slate_slot_0, *{
            __slate_compound_2 = {
                let mut __t0: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                __t0._type = 0u32;
                __t0
            };
            core::ptr::addr_of_mut!(__slate_compound_2)
        });
        '__join_0: {
            loop {
                if !unsafe { _Py_path_config.prefix }.is_null()
                    && unsafe { (*config).prefix }.is_null()
                {
                    *__slate_slot_0 = PyConfig_SetString(
                        config,
                        core::ptr::addr_of_mut!((*config).prefix),
                        _Py_path_config.prefix as *const i32,
                    );
                    if _PyStatus_EXCEPTION!(*__slate_slot_0) {
                        break '__join_0;
                    }
                }
                if true {
                    break;
                }
            }
            loop {
                if !unsafe { _Py_path_config.exec_prefix }.is_null()
                    && unsafe { (*config).exec_prefix }.is_null()
                {
                    *__slate_slot_0 = PyConfig_SetString(
                        config,
                        core::ptr::addr_of_mut!((*config).exec_prefix),
                        _Py_path_config.exec_prefix as *const i32,
                    );
                    if _PyStatus_EXCEPTION!(*__slate_slot_0) {
                        break '__join_0;
                    }
                }
                if true {
                    break;
                }
            }
            loop {
                if !unsafe { _Py_path_config.stdlib_dir }.is_null()
                    && unsafe { (*config).stdlib_dir }.is_null()
                {
                    *__slate_slot_0 = PyConfig_SetString(
                        config,
                        core::ptr::addr_of_mut!((*config).stdlib_dir),
                        _Py_path_config.stdlib_dir as *const i32,
                    );
                    if _PyStatus_EXCEPTION!(*__slate_slot_0) {
                        break '__join_0;
                    }
                }
                if true {
                    break;
                }
            }
            loop {
                if !unsafe { _Py_path_config.program_name }.is_null()
                    && unsafe { (*config).program_name }.is_null()
                {
                    *__slate_slot_0 = PyConfig_SetString(
                        config,
                        core::ptr::addr_of_mut!((*config).program_name),
                        _Py_path_config.program_name as *const i32,
                    );
                    if _PyStatus_EXCEPTION!(*__slate_slot_0) {
                        break '__join_0;
                    }
                }
                if true {
                    break;
                }
            }
            loop {
                if !unsafe { _Py_path_config.home }.is_null() && unsafe { (*config).home }.is_null()
                {
                    *__slate_slot_0 = PyConfig_SetString(
                        config,
                        core::ptr::addr_of_mut!((*config).home),
                        _Py_path_config.home as *const i32,
                    );
                    if _PyStatus_EXCEPTION!(*__slate_slot_0) {
                        break '__join_0;
                    }
                }
                if true {
                    break;
                }
            }
            loop {
                if !unsafe { _Py_path_config.program_full_path }.is_null()
                    && unsafe { (*config).executable }.is_null()
                {
                    *__slate_slot_0 = PyConfig_SetString(
                        config,
                        core::ptr::addr_of_mut!((*config).executable),
                        _Py_path_config.program_full_path as *const i32,
                    );
                    if _PyStatus_EXCEPTION!(*__slate_slot_0) {
                        break '__join_0;
                    }
                }
                if true {
                    break;
                }
            }
            COPY_INT!(_is_python_build, config);
            // module_search_path must be initialised - not read
        }
        return *__slate_slot_0;
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyPathConfig_UpdateGlobal(
    config: *const PyConfig,
) -> __SlateRecord_initconfig_h_19 {
    let mut __slate_compound_16: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_17: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __v2: bool = unsafe { core::mem::zeroed() };
    let mut i: i64 = unsafe { core::mem::zeroed() };
    let mut p: *mut i32 = unsafe { core::mem::zeroed() };
    let mut path: *mut i32 = unsafe { core::mem::zeroed() };
    let mut i_1: i64 = unsafe { core::mem::zeroed() };
    let mut cch: u64 = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_0: {
            loop {
                if !unsafe { (*config).prefix }.is_null() {
                    _PyMem_DefaultRawFree(_Py_path_config.prefix as *mut ());
                    _Py_path_config.prefix =
                        _PyMem_DefaultRawWcsdup((*config).prefix as *const i32);
                    if unsafe { _Py_path_config.prefix }.is_null() {
                        break '__join_0;
                    }
                }
                if true {
                    break;
                }
            }
            loop {
                if !unsafe { (*config).exec_prefix }.is_null() {
                    _PyMem_DefaultRawFree(_Py_path_config.exec_prefix as *mut ());
                    _Py_path_config.exec_prefix =
                        _PyMem_DefaultRawWcsdup((*config).exec_prefix as *const i32);
                    if unsafe { _Py_path_config.exec_prefix }.is_null() {
                        break '__join_0;
                    }
                }
                if true {
                    break;
                }
            }
            loop {
                if !unsafe { (*config).stdlib_dir }.is_null() {
                    _PyMem_DefaultRawFree(_Py_path_config.stdlib_dir as *mut ());
                    _Py_path_config.stdlib_dir =
                        _PyMem_DefaultRawWcsdup((*config).stdlib_dir as *const i32);
                    if unsafe { _Py_path_config.stdlib_dir }.is_null() {
                        break '__join_0;
                    }
                }
                if true {
                    break;
                }
            }
            loop {
                if !unsafe { (*config).program_name }.is_null() {
                    _PyMem_DefaultRawFree(_Py_path_config.program_name as *mut ());
                    _Py_path_config.program_name =
                        _PyMem_DefaultRawWcsdup((*config).program_name as *const i32);
                    if unsafe { _Py_path_config.program_name }.is_null() {
                        break '__join_0;
                    }
                }
                if true {
                    break;
                }
            }
            loop {
                if !unsafe { (*config).home }.is_null() {
                    _PyMem_DefaultRawFree(_Py_path_config.home as *mut ());
                    _Py_path_config.home = _PyMem_DefaultRawWcsdup((*config).home as *const i32);
                    if unsafe { _Py_path_config.home }.is_null() {
                        break '__join_0;
                    }
                }
                if true {
                    break;
                }
            }
            loop {
                if !unsafe { (*config).executable }.is_null() {
                    _PyMem_DefaultRawFree(_Py_path_config.program_full_path as *mut ());
                    _Py_path_config.program_full_path =
                        _PyMem_DefaultRawWcsdup((*config).executable as *const i32);
                    if unsafe { _Py_path_config.program_full_path }.is_null() {
                        break '__join_0;
                    }
                }
                if true {
                    break;
                }
            }
            COPY_INT__2!(_is_python_build, config);
            _PyMem_DefaultRawFree(_Py_path_config.module_search_path as *mut ());
            _Py_path_config.module_search_path = core::ptr::null_mut::<i32>();
            _PyMem_DefaultRawFree(_Py_path_config.calculated_module_search_path as *mut ());
            _Py_path_config.calculated_module_search_path = core::ptr::null_mut::<i32>();
            loop {
                cch = 1;
                i_1 = 0;
                while i_1 < (*config).module_search_paths.length {
                    let __v13: u64 = cch;
                    cch = __v13.wrapping_add(
                        1u64.wrapping_add(wcslen(
                            (*unsafe { (*config).module_search_paths.items }
                                .wrapping_offset(i_1 as isize)
                                as *const i32) as *const i32,
                        ) as u64),
                    );
                    i_1 = i_1.wrapping_add(1);
                }
                path =
                    _PyMem_DefaultRawMalloc((core::mem::size_of::<i32>() as u64).wrapping_mul(cch))
                        as *mut i32;
                if path.is_null() {
                    break '__join_0;
                } else {
                    p = path;
                    i = 0;
                    while i < (*config).module_search_paths.length {
                        wcscpy(
                            p,
                            *unsafe { (*config).module_search_paths.items }
                                .wrapping_offset(i as isize)
                                as *const i32,
                        );
                        p = wcschr(p as *const i32, 0i32);
                        let __v6: *mut i32 = p;
                        p = __v6.wrapping_offset(1 as isize);
                        *__v6 = 58;
                        *p = 0;
                        i = i.wrapping_add(1);
                    }
                    loop {
                        *p = 0;
                        if p != path {
                            let __v1: *mut i32 = p;
                            let __v0: *mut i32 = __v1.wrapping_offset(-(1 as isize));
                            p = __v0;
                            __v2 = *__v0 == 58;
                        } else {
                            __v2 = false;
                        }
                        if !__v2 {
                            break;
                        }
                    }
                    _Py_path_config.calculated_module_search_path = path;
                    if true {
                        break;
                    }
                }
            }
            return *{
                __slate_compound_17 = {
                    let mut __t1: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                    __t1._type = 0u32;
                    __t1
                };
                core::ptr::addr_of_mut!(__slate_compound_17)
            };
        }
        return *{
            __slate_compound_16 = {
                let mut __t0: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                __t0._type = 1u32;
                __t0.func = _PyStatus_GET_FUNC__75!() as *const i8;
                __t0.err_msg = c"memory allocation failed".as_ptr();
                __t0
            };
            core::ptr::addr_of_mut!(__slate_compound_16)
        };
    }
}

fn path_out_of_memory(func: *const i8) {
    unsafe { _Py_FatalErrorFunc(func, c"out of memory".as_ptr()) };
    unsafe { core::hint::unreachable_unchecked() };
}

use slate_support::link;
use slate_support::wcslen;

/// Removed in Python 3.13 API, but kept for the stable ABI
#[unsafe(no_mangle)]
extern "C-unwind" fn Py_SetPath(path: *const i32) {
    if path.is_null() {
        _PyPathConfig_ClearGlobal();
        return;
    }
    unsafe { _PyMem_DefaultRawFree(_Py_path_config.prefix as *mut ()) };
    unsafe { _PyMem_DefaultRawFree(_Py_path_config.exec_prefix as *mut ()) };
    unsafe { _PyMem_DefaultRawFree(_Py_path_config.stdlib_dir as *mut ()) };
    unsafe { _PyMem_DefaultRawFree(_Py_path_config.module_search_path as *mut ()) };
    unsafe { _PyMem_DefaultRawFree(_Py_path_config.calculated_module_search_path as *mut ()) };
    unsafe {
        _Py_path_config.prefix = _PyMem_DefaultRawWcsdup(
            (core::ptr::addr_of_mut!(__slate_str265) as *mut i32) as *const i32,
        );
    }
    unsafe {
        _Py_path_config.exec_prefix = _PyMem_DefaultRawWcsdup(
            (core::ptr::addr_of_mut!(__slate_str266) as *mut i32) as *const i32,
        );
    }
    // XXX Copy this from the new module_search_path?
    if !unsafe { _Py_path_config.home }.is_null() {
        unsafe {
            _Py_path_config.stdlib_dir =
                _PyMem_DefaultRawWcsdup(_Py_path_config.home as *const i32);
        }
    } else {
        unsafe {
            _Py_path_config.stdlib_dir = _PyMem_DefaultRawWcsdup(
                (core::ptr::addr_of_mut!(__slate_str267) as *mut i32) as *const i32,
            );
        }
    }
    unsafe {
        _Py_path_config.module_search_path = _PyMem_DefaultRawWcsdup(path);
    }
    unsafe {
        _Py_path_config.calculated_module_search_path = core::ptr::null_mut::<i32>();
    }
    if unsafe { _Py_path_config.prefix }.is_null()
        || unsafe { _Py_path_config.exec_prefix }.is_null()
        || unsafe { _Py_path_config.stdlib_dir }.is_null()
        || unsafe { _Py_path_config.module_search_path }.is_null()
    {
        path_out_of_memory(c"Py_SetPath".as_ptr());
        unsafe { core::hint::unreachable_unchecked() };
    }
}

static mut __slate_str265: [i32; 1] = [0];

static mut __slate_str266: [i32; 1] = [0];

static mut __slate_str267: [i32; 1] = [0];

#[unsafe(no_mangle)]
extern "C-unwind" fn Py_SetPythonHome(home: *const i32) {
    let has_value: i32 =
        (!home.is_null() && (unsafe { *home.wrapping_offset(0 as isize) }) != 0) as i32;
    unsafe { _PyMem_DefaultRawFree(_Py_path_config.home as *mut ()) };
    unsafe {
        _Py_path_config.home = core::ptr::null_mut::<i32>();
    }
    if has_value != 0 {
        unsafe {
            _Py_path_config.home = _PyMem_DefaultRawWcsdup(home);
        }
    }
    if has_value != 0 && unsafe { _Py_path_config.home }.is_null() {
        path_out_of_memory(c"Py_SetPythonHome".as_ptr());
        unsafe { core::hint::unreachable_unchecked() };
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn Py_SetProgramName(program_name: *const i32) {
    let has_value: i32 = (!program_name.is_null()
        && (unsafe { *program_name.wrapping_offset(0 as isize) }) != 0)
        as i32;
    unsafe { _PyMem_DefaultRawFree(_Py_path_config.program_name as *mut ()) };
    unsafe {
        _Py_path_config.program_name = core::ptr::null_mut::<i32>();
    }
    if has_value != 0 {
        unsafe {
            _Py_path_config.program_name = _PyMem_DefaultRawWcsdup(program_name);
        }
    }
    if has_value != 0 && unsafe { _Py_path_config.program_name }.is_null() {
        path_out_of_memory(c"Py_SetProgramName".as_ptr());
        unsafe { core::hint::unreachable_unchecked() };
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn Py_GetPath() -> *mut i32 {
    // If the user has provided a path, return that
    if !unsafe { _Py_path_config.module_search_path }.is_null() {
        return unsafe { _Py_path_config.module_search_path };
    }
    // If we have already done calculations, return the calculated path
    unsafe { _Py_path_config.calculated_module_search_path }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_GetStdlibDir() -> *mut i32 {
    let stdlib_dir: *mut i32 = unsafe { _Py_path_config.stdlib_dir };
    if !stdlib_dir.is_null() && (unsafe { *stdlib_dir.wrapping_offset(0 as isize) }) != 0 {
        return stdlib_dir;
    }
    core::ptr::null_mut::<i32>()
}

#[unsafe(no_mangle)]
extern "C-unwind" fn Py_GetPrefix() -> *mut i32 {
    unsafe { _Py_path_config.prefix }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn Py_GetExecPrefix() -> *mut i32 {
    unsafe { _Py_path_config.exec_prefix }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn Py_GetProgramFullPath() -> *mut i32 {
    unsafe { _Py_path_config.program_full_path }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn Py_GetPythonHome() -> *mut i32 {
    unsafe { _Py_path_config.home }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn Py_GetProgramName() -> *mut i32 {
    unsafe { _Py_path_config.program_name }
}

/// Compute module search path from argv[0] or the current working
/// directory ("-m module" case) which will be prepended to sys.argv:
/// sys.path[0].
///
/// Return 1 if the path is correctly resolved and written into *path0_p.
///
/// Return 0 if it fails to resolve the full path. For example, return 0 if the
/// current working directory has been removed (bpo-36236) or if argv is empty.
///
/// Raise an exception and return -1 on error.
#[unsafe(no_mangle)]
extern "C-unwind" fn _PyPathConfig_ComputeSysPath0(
    argv: *const __SlateRecord_initconfig_h_36,
    path0_p: *mut *mut _object,
) -> i32 {
    if (unsafe { (*argv).length }) == 0 {
        // Leave sys.path unchanged if sys.argv is empty
        return 0;
    }
    let argv0: *mut i32 = unsafe { *unsafe { (*argv).items }.wrapping_offset(0 as isize) };
    let have_module_arg: i32 = ((unsafe {
        wcscmp(
            argv0 as *const i32,
            (core::ptr::addr_of_mut!(__slate_str271) as *mut i32) as *const i32,
        )
    }) == 0) as i32;
    let have_script_arg: i32 = (have_module_arg == 0
        && (unsafe {
            wcscmp(
                argv0 as *const i32,
                (core::ptr::addr_of_mut!(__slate_str272) as *mut i32) as *const i32,
            )
        }) != 0) as i32;
    let mut path0: *mut i32 = argv0;
    let mut n: i64 = 0;
    let mut fullpath: __SlateAlign16<[i32; 4096]> = __SlateAlign16([0i32; 4096]);
    if have_module_arg != 0 {
        if unsafe { _Py_wgetcwd(fullpath.0.as_mut_ptr() as *mut i32, Py_ARRAY_LENGTH__9!()) }
            .is_null()
        {
            return 0;
        }
        path0 = fullpath.0.as_mut_ptr() as *mut i32;
        n = (unsafe { wcslen((path0 as *const i32) as *const i32) as u64 }) as i64;
    }
    let mut link: __SlateAlign16<[i32; 4097]> = __SlateAlign16([0i32; 4097]);
    let mut nr: i32 = 0;
    let mut path0copy: __SlateAlign16<[i32; 8193]> = __SlateAlign16([0i32; 8193]);
    if have_script_arg != 0 {
        nr = unsafe {
            _Py_wreadlink(
                path0 as *const i32,
                link.0.as_mut_ptr() as *mut i32,
                Py_ARRAY_LENGTH!(),
            )
        };
    }
    if nr > 0 {
        // It's a symlink
        link.0[nr as usize] = 0;
        if link.0[0] == 47 {
            path0 = link.0.as_mut_ptr() as *mut i32; // Link to absolute path
        } else {
            if unsafe { wcschr(link.0.as_mut_ptr() as *const i32, 47i32) }.is_null() {
                // Link without path
            } else {
                // Must join(dirname(path0), link)
                let mut q: *mut i32 = unsafe { wcsrchr(path0 as *const i32, 47i32) };
                if q.is_null() {
                    // path0 without path
                    path0 = link.0.as_mut_ptr() as *mut i32;
                } else {
                    // Must make a copy, path0copy has room for 2 * MAXPATHLEN
                    unsafe {
                        wcsncpy(
                            path0copy.0.as_mut_ptr() as *mut i32,
                            path0 as *const i32,
                            4096u64,
                        )
                    };
                    q = unsafe { wcsrchr(path0copy.0.as_mut_ptr() as *const i32, 47i32) };
                    unsafe {
                        wcsncpy(
                            q.wrapping_offset(1 as isize),
                            link.0.as_mut_ptr() as *const i32,
                            4096u64,
                        )
                    };
                    unsafe {
                        *q.wrapping_offset(4097i32 as isize) = 0;
                    }
                    path0 = path0copy.0.as_mut_ptr() as *mut i32;
                }
            }
        }
    }
    let mut p: *mut i32 = core::ptr::null_mut::<i32>();
    // All other filename syntaxes
    if have_script_arg != 0 {
        if !unsafe {
            _Py_wrealpath(
                path0 as *const i32,
                fullpath.0.as_mut_ptr() as *mut i32,
                Py_ARRAY_LENGTH__9!(),
            )
        }
        .is_null()
        {
            path0 = fullpath.0.as_mut_ptr() as *mut i32;
        }
        p = unsafe { wcsrchr(path0 as *const i32, 47i32) };
    }
    if !p.is_null() {
        n = (unsafe { p.wrapping_offset(1 as isize).offset_from(path0 as *mut i32) }) as i64;
        if n > 1 {
            // Drop trailing separator
            n = n.wrapping_sub(1);
        }
    }
    let path0_obj: *mut _object = unsafe { PyUnicode_FromWideChar(path0 as *const i32, n) };
    if path0_obj.is_null() {
        return 1i32.wrapping_neg();
    }
    unsafe {
        *path0_p = path0_obj;
    }
    1
}

static mut __slate_str271: [i32; 3] = [45, 109, 0];

static mut __slate_str272: [i32; 3] = [45, 99, 0];
