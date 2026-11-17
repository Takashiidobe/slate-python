//! The size of 'void *', as computed by sizeof.
//! Macro to use the more powerful/dangerous C-style cast even in C++.
//! Define macros for inline documentation.
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
//! Cast a function to the PyCFunction type to use it with PyMethodDef.
//!
//!
//!
//! This macro can be used to prevent compiler warnings if the first parameter
//!
//! uses a different pointer type than PyObject* (ex: METH_VARARGS and METH_O
//!
//! calling conventions).
//!
//!
//!
//! The macro can also be used for METH_FASTCALL and METH_VARARGS|METH_KEYWORDS
//!
//! calling conventions to avoid compiler warnings because the function has more
//!
//! than 2 parameters. The macro first casts the function to the
//!
//! "void func(void)" type to prevent compiler warnings.
//!
//!
//!
//! If a function is declared with the METH_NOARGS calling convention, it must
//!
//! have 2 parameters. Since the second parameter is unused, Py_UNUSED() can be
//!
//! used to prevent a compiler warning. If the function has a single parameter,
//!
//! it triggers an undefined behavior when Python calls it with 2 parameters
//!
//! (bpo-33012).
//! A code object, the __code__ attribute
//! To avoid repeating ourselves in deepfreeze.py, all PyCodeObject members are
//!
//! defined in this macro:
//! Remote debugger support
//! PY_TIMEOUT_T is the integral type used to specify timeouts when waiting
//!
//! on a lock (see PyThread_acquire_lock_timed() below).
//! _PyTraceMalloc_IsTracing
// Cast a function to another function type T.
//
//
//
// The macro first casts the function to the "void func(void)" type
//
// to prevent compiler warnings.
//
//
//
// Note that using this cast only prevents the compiler from emitting
//
// warnings, but does not prevent an undefined behavior at runtime if
//
// the original function signature is not respected.
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
macro_rules! PyModule_Create {
    ($module:expr) => {
        unsafe { PyModule_Create2($module, 1013i32) }
    };
}

macro_rules! _PyArg_CheckPositional {
    ($funcname:expr, $nargs:expr, $min:expr, $max:expr) => {{
        let __v: bool =
            !_Py_ANY_VARARGS!($max) && (($min) as i64) <= $nargs && ($nargs) <= (($max) as i64)
                || (unsafe {
                    _PyArg_CheckPositional(
                        ($funcname) as *const i8,
                        $nargs,
                        ($min) as i64,
                        ($max) as i64,
                    )
                }) != 0;
        __v
    }};
}

macro_rules! _Py_ANY_VARARGS {
    ($n:expr) => {
        (($n) as i64) == 9223372036854775807
    };
}

unsafe extern "C" {
    static mut _Py_NoneStruct: _object;
    fn PyErr_Occurred() -> *mut _object;
    fn PyLong_FromLong(arg0: i64) -> *mut _object;
    fn PyLong_FromSize_t(arg0: u64) -> *mut _object;
    fn PyLong_AsInt(arg0: *mut _object) -> i32;
    fn PyBool_FromLong(arg0: i64) -> *mut _object;
    fn PyModule_Create2(arg0: *mut PyModuleDef, arg1: i32) -> *mut _object;
    fn _PyTraceMalloc_IsTracing() -> i32;
    fn _PyTraceMalloc_ClearTraces();
    fn _PyTraceMalloc_GetTraces() -> *mut _object;
    fn _PyTraceMalloc_GetObjectTraceback(arg0: *mut _object) -> *mut _object;
    fn _PyTraceMalloc_Start(arg0: i32) -> i32;
    fn _PyTraceMalloc_Stop();
    fn _PyTraceMalloc_GetTracebackLimit() -> i32;
    fn _PyTraceMalloc_GetMemory() -> u64;
    fn _PyTraceMalloc_GetTracedMemory() -> *mut _object;
    fn _PyTraceMalloc_ResetPeak();
    fn _PyArg_CheckPositional(arg0: *const i8, arg1: i64, arg2: i64, arg3: i64) -> i32;
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
struct PyModuleDef_Base {
    ob_base: _object,
    m_init: Option<unsafe extern "C-unwind" fn() -> *mut _object>,
    m_index: i64,
    m_copy: *mut _object,
}

// m_init
// m_index
// m_copy
#[repr(C)]
#[derive(Clone, Copy)]
struct PyModuleDef_Slot {
    slot: i32,
    value: *mut (),
}

#[repr(C)]
#[derive(Clone, Copy)]
struct PyModuleDef {
    m_base: PyModuleDef_Base,
    m_name: *const i8,
    m_doc: *const i8,
    m_size: i64,
    m_methods: *mut PyMethodDef,
    m_slots: *mut PyModuleDef_Slot,
    m_traverse: Option<
        unsafe extern "C-unwind" fn(
            *mut _object,
            Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
            *mut (),
        ) -> i32,
    >,
    m_clear: Option<unsafe extern "C-unwind" fn(*mut _object) -> i32>,
    m_free: Option<unsafe extern "C-unwind" fn(*mut ())>,
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

#[repr(C, align(16))]
struct __SlateAlign16<T>(T);

unsafe impl<T> Sync for __SlateAlign16<T> {}

static _tracemalloc_is_tracing__doc__: __SlateAlign16<[i8; 103]> = __SlateAlign16([
    105i8, 115i8, 95i8, 116i8, 114i8, 97i8, 99i8, 105i8, 110i8, 103i8, 40i8, 36i8, 109i8, 111i8,
    100i8, 117i8, 108i8, 101i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8,
    116i8, 117i8, 114i8, 110i8, 32i8, 84i8, 114i8, 117i8, 101i8, 32i8, 105i8, 102i8, 32i8, 116i8,
    104i8, 101i8, 32i8, 116i8, 114i8, 97i8, 99i8, 101i8, 109i8, 97i8, 108i8, 108i8, 111i8, 99i8,
    32i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 32i8, 105i8, 115i8, 32i8, 116i8, 114i8, 97i8,
    99i8, 105i8, 110i8, 103i8, 32i8, 80i8, 121i8, 116i8, 104i8, 111i8, 110i8, 32i8, 109i8, 101i8,
    109i8, 111i8, 114i8, 121i8, 32i8, 97i8, 108i8, 108i8, 111i8, 99i8, 97i8, 116i8, 105i8, 111i8,
    110i8, 115i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct._tracemalloc._tracemalloc_is_tracing")]
extern "C-unwind" fn _tracemalloc_is_tracing(
    module: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    _tracemalloc_is_tracing_impl(module)
}

static _tracemalloc_clear_traces__doc__: __SlateAlign16<[i8; 80]> = __SlateAlign16([
    99i8, 108i8, 101i8, 97i8, 114i8, 95i8, 116i8, 114i8, 97i8, 99i8, 101i8, 115i8, 40i8, 36i8,
    109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8,
    67i8, 108i8, 101i8, 97i8, 114i8, 32i8, 116i8, 114i8, 97i8, 99i8, 101i8, 115i8, 32i8, 111i8,
    102i8, 32i8, 109i8, 101i8, 109i8, 111i8, 114i8, 121i8, 32i8, 98i8, 108i8, 111i8, 99i8, 107i8,
    115i8, 32i8, 97i8, 108i8, 108i8, 111i8, 99i8, 97i8, 116i8, 101i8, 100i8, 32i8, 98i8, 121i8,
    32i8, 80i8, 121i8, 116i8, 104i8, 111i8, 110i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct._tracemalloc._tracemalloc_clear_traces")]
extern "C-unwind" fn _tracemalloc_clear_traces(
    module: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    _tracemalloc_clear_traces_impl(module)
}

static _tracemalloc__get_traces__doc__: __SlateAlign16<[i8; 259]> = __SlateAlign16([
    95i8, 103i8, 101i8, 116i8, 95i8, 116i8, 114i8, 97i8, 99i8, 101i8, 115i8, 40i8, 36i8, 109i8,
    111i8, 100i8, 117i8, 108i8, 101i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 71i8,
    101i8, 116i8, 32i8, 116i8, 114i8, 97i8, 99i8, 101i8, 115i8, 32i8, 111i8, 102i8, 32i8, 97i8,
    108i8, 108i8, 32i8, 109i8, 101i8, 109i8, 111i8, 114i8, 121i8, 32i8, 98i8, 108i8, 111i8, 99i8,
    107i8, 115i8, 32i8, 97i8, 108i8, 108i8, 111i8, 99i8, 97i8, 116i8, 101i8, 100i8, 32i8, 98i8,
    121i8, 32i8, 80i8, 121i8, 116i8, 104i8, 111i8, 110i8, 46i8, 10i8, 10i8, 82i8, 101i8, 116i8,
    117i8, 114i8, 110i8, 32i8, 97i8, 32i8, 108i8, 105i8, 115i8, 116i8, 32i8, 111i8, 102i8, 32i8,
    40i8, 115i8, 105i8, 122i8, 101i8, 58i8, 32i8, 105i8, 110i8, 116i8, 44i8, 32i8, 116i8, 114i8,
    97i8, 99i8, 101i8, 98i8, 97i8, 99i8, 107i8, 58i8, 32i8, 116i8, 117i8, 112i8, 108i8, 101i8,
    41i8, 32i8, 116i8, 117i8, 112i8, 108i8, 101i8, 115i8, 46i8, 10i8, 116i8, 114i8, 97i8, 99i8,
    101i8, 98i8, 97i8, 99i8, 107i8, 32i8, 105i8, 115i8, 32i8, 97i8, 32i8, 116i8, 117i8, 112i8,
    108i8, 101i8, 32i8, 111i8, 102i8, 32i8, 40i8, 102i8, 105i8, 108i8, 101i8, 110i8, 97i8, 109i8,
    101i8, 58i8, 32i8, 115i8, 116i8, 114i8, 44i8, 32i8, 108i8, 105i8, 110i8, 101i8, 110i8, 111i8,
    58i8, 32i8, 105i8, 110i8, 116i8, 41i8, 32i8, 116i8, 117i8, 112i8, 108i8, 101i8, 115i8, 46i8,
    10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 97i8, 110i8, 32i8, 101i8, 109i8,
    112i8, 116i8, 121i8, 32i8, 108i8, 105i8, 115i8, 116i8, 32i8, 105i8, 102i8, 32i8, 116i8, 104i8,
    101i8, 32i8, 116i8, 114i8, 97i8, 99i8, 101i8, 109i8, 97i8, 108i8, 108i8, 111i8, 99i8, 32i8,
    109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 32i8, 105i8, 115i8, 32i8, 100i8, 105i8, 115i8, 97i8,
    98i8, 108i8, 101i8, 100i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct._tracemalloc._tracemalloc__get_traces")]
extern "C-unwind" fn _tracemalloc__get_traces(
    module: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    _tracemalloc__get_traces_impl(module)
}

static _tracemalloc__get_object_traceback__doc__: __SlateAlign16<[i8; 257]> = __SlateAlign16([
    95i8, 103i8, 101i8, 116i8, 95i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 95i8, 116i8, 114i8,
    97i8, 99i8, 101i8, 98i8, 97i8, 99i8, 107i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8,
    101i8, 44i8, 32i8, 111i8, 98i8, 106i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8,
    71i8, 101i8, 116i8, 32i8, 116i8, 104i8, 101i8, 32i8, 116i8, 114i8, 97i8, 99i8, 101i8, 98i8,
    97i8, 99i8, 107i8, 32i8, 119i8, 104i8, 101i8, 114i8, 101i8, 32i8, 116i8, 104i8, 101i8, 32i8,
    80i8, 121i8, 116i8, 104i8, 111i8, 110i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 32i8,
    111i8, 98i8, 106i8, 32i8, 119i8, 97i8, 115i8, 32i8, 97i8, 108i8, 108i8, 111i8, 99i8, 97i8,
    116i8, 101i8, 100i8, 46i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 97i8,
    32i8, 116i8, 117i8, 112i8, 108i8, 101i8, 32i8, 111i8, 102i8, 32i8, 40i8, 102i8, 105i8, 108i8,
    101i8, 110i8, 97i8, 109i8, 101i8, 58i8, 32i8, 115i8, 116i8, 114i8, 44i8, 32i8, 108i8, 105i8,
    110i8, 101i8, 110i8, 111i8, 58i8, 32i8, 105i8, 110i8, 116i8, 41i8, 32i8, 116i8, 117i8, 112i8,
    108i8, 101i8, 115i8, 46i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 78i8, 111i8,
    110i8, 101i8, 32i8, 105i8, 102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 116i8, 114i8, 97i8, 99i8,
    101i8, 109i8, 97i8, 108i8, 108i8, 111i8, 99i8, 32i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8,
    32i8, 105i8, 115i8, 32i8, 100i8, 105i8, 115i8, 97i8, 98i8, 108i8, 101i8, 100i8, 32i8, 111i8,
    114i8, 32i8, 100i8, 105i8, 100i8, 32i8, 110i8, 111i8, 116i8, 10i8, 116i8, 114i8, 97i8, 99i8,
    101i8, 32i8, 116i8, 104i8, 101i8, 32i8, 97i8, 108i8, 108i8, 111i8, 99i8, 97i8, 116i8, 105i8,
    111i8, 110i8, 32i8, 111i8, 102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 111i8, 98i8, 106i8, 101i8,
    99i8, 116i8, 46i8, 0i8,
]);

static _tracemalloc_start__doc__: __SlateAlign16<[i8; 158]> = __SlateAlign16([
    115i8, 116i8, 97i8, 114i8, 116i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 44i8,
    32i8, 110i8, 102i8, 114i8, 97i8, 109i8, 101i8, 61i8, 49i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8,
    45i8, 10i8, 10i8, 83i8, 116i8, 97i8, 114i8, 116i8, 32i8, 116i8, 114i8, 97i8, 99i8, 105i8,
    110i8, 103i8, 32i8, 80i8, 121i8, 116i8, 104i8, 111i8, 110i8, 32i8, 109i8, 101i8, 109i8, 111i8,
    114i8, 121i8, 32i8, 97i8, 108i8, 108i8, 111i8, 99i8, 97i8, 116i8, 105i8, 111i8, 110i8, 115i8,
    46i8, 10i8, 10i8, 65i8, 108i8, 115i8, 111i8, 32i8, 115i8, 101i8, 116i8, 32i8, 116i8, 104i8,
    101i8, 32i8, 109i8, 97i8, 120i8, 105i8, 109i8, 117i8, 109i8, 32i8, 110i8, 117i8, 109i8, 98i8,
    101i8, 114i8, 32i8, 111i8, 102i8, 32i8, 102i8, 114i8, 97i8, 109i8, 101i8, 115i8, 32i8, 115i8,
    116i8, 111i8, 114i8, 101i8, 100i8, 32i8, 105i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 116i8,
    114i8, 97i8, 99i8, 101i8, 98i8, 97i8, 99i8, 107i8, 32i8, 111i8, 102i8, 32i8, 97i8, 10i8, 116i8,
    114i8, 97i8, 99i8, 101i8, 32i8, 116i8, 111i8, 32i8, 110i8, 102i8, 114i8, 97i8, 109i8, 101i8,
    46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct._tracemalloc._tracemalloc_start")]
extern "C-unwind" fn _tracemalloc_start(
    module: *mut _object,
    args: *const *mut _object,
    nargs: i64,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut nframe: i32 = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_0: {
            return_value = core::ptr::null_mut::<_object>();
            nframe = 1;
            if _PyArg_CheckPositional!(c"start".as_ptr() as *mut i8, nargs, 0i32, 1i32) {
                if nargs >= 1 {
                    nframe = PyLong_AsInt(*args.wrapping_offset(0 as isize));
                    if nframe == 1i32.wrapping_neg() {
                        __v0 = !PyErr_Occurred().is_null();
                    } else {
                        __v0 = false;
                    }
                    if __v0 {
                        break '__join_0;
                    }
                }
                return_value = _tracemalloc_start_impl(module, nframe);
            }
        }
        return return_value;
    }
}

static _tracemalloc_stop__doc__: __SlateAlign16<[i8; 118]> = __SlateAlign16([
    115i8, 116i8, 111i8, 112i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 44i8, 32i8,
    47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 83i8, 116i8, 111i8, 112i8, 32i8, 116i8, 114i8, 97i8,
    99i8, 105i8, 110i8, 103i8, 32i8, 80i8, 121i8, 116i8, 104i8, 111i8, 110i8, 32i8, 109i8, 101i8,
    109i8, 111i8, 114i8, 121i8, 32i8, 97i8, 108i8, 108i8, 111i8, 99i8, 97i8, 116i8, 105i8, 111i8,
    110i8, 115i8, 46i8, 10i8, 10i8, 65i8, 108i8, 115i8, 111i8, 32i8, 99i8, 108i8, 101i8, 97i8,
    114i8, 32i8, 116i8, 114i8, 97i8, 99i8, 101i8, 115i8, 32i8, 111i8, 102i8, 32i8, 109i8, 101i8,
    109i8, 111i8, 114i8, 121i8, 32i8, 98i8, 108i8, 111i8, 99i8, 107i8, 115i8, 32i8, 97i8, 108i8,
    108i8, 111i8, 99i8, 97i8, 116i8, 101i8, 100i8, 32i8, 98i8, 121i8, 32i8, 80i8, 121i8, 116i8,
    104i8, 111i8, 110i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct._tracemalloc._tracemalloc_stop")]
extern "C-unwind" fn _tracemalloc_stop(
    module: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    _tracemalloc_stop_impl(module)
}

static _tracemalloc_get_traceback_limit__doc__: __SlateAlign16<[i8; 206]> = __SlateAlign16([
    103i8, 101i8, 116i8, 95i8, 116i8, 114i8, 97i8, 99i8, 101i8, 98i8, 97i8, 99i8, 107i8, 95i8,
    108i8, 105i8, 109i8, 105i8, 116i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 44i8,
    32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 71i8, 101i8, 116i8, 32i8, 116i8, 104i8, 101i8,
    32i8, 109i8, 97i8, 120i8, 105i8, 109i8, 117i8, 109i8, 32i8, 110i8, 117i8, 109i8, 98i8, 101i8,
    114i8, 32i8, 111i8, 102i8, 32i8, 102i8, 114i8, 97i8, 109i8, 101i8, 115i8, 32i8, 115i8, 116i8,
    111i8, 114i8, 101i8, 100i8, 32i8, 105i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 116i8, 114i8,
    97i8, 99i8, 101i8, 98i8, 97i8, 99i8, 107i8, 32i8, 111i8, 102i8, 32i8, 97i8, 32i8, 116i8, 114i8,
    97i8, 99i8, 101i8, 46i8, 10i8, 10i8, 66i8, 121i8, 32i8, 100i8, 101i8, 102i8, 97i8, 117i8,
    108i8, 116i8, 44i8, 32i8, 97i8, 32i8, 116i8, 114i8, 97i8, 99i8, 101i8, 32i8, 111i8, 102i8,
    32i8, 97i8, 110i8, 32i8, 97i8, 108i8, 108i8, 111i8, 99i8, 97i8, 116i8, 101i8, 100i8, 32i8,
    109i8, 101i8, 109i8, 111i8, 114i8, 121i8, 32i8, 98i8, 108i8, 111i8, 99i8, 107i8, 32i8, 111i8,
    110i8, 108i8, 121i8, 32i8, 115i8, 116i8, 111i8, 114i8, 101i8, 115i8, 10i8, 116i8, 104i8, 101i8,
    32i8, 109i8, 111i8, 115i8, 116i8, 32i8, 114i8, 101i8, 99i8, 101i8, 110i8, 116i8, 32i8, 102i8,
    114i8, 97i8, 109i8, 101i8, 58i8, 32i8, 116i8, 104i8, 101i8, 32i8, 108i8, 105i8, 109i8, 105i8,
    116i8, 32i8, 105i8, 115i8, 32i8, 49i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct._tracemalloc._tracemalloc_get_traceback_limit")]
extern "C-unwind" fn _tracemalloc_get_traceback_limit(
    module: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    _tracemalloc_get_traceback_limit_impl(module)
}

static _tracemalloc_get_tracemalloc_memory__doc__: __SlateAlign16<[i8; 157]> = __SlateAlign16([
    103i8, 101i8, 116i8, 95i8, 116i8, 114i8, 97i8, 99i8, 101i8, 109i8, 97i8, 108i8, 108i8, 111i8,
    99i8, 95i8, 109i8, 101i8, 109i8, 111i8, 114i8, 121i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8,
    108i8, 101i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 71i8, 101i8, 116i8, 32i8,
    116i8, 104i8, 101i8, 32i8, 109i8, 101i8, 109i8, 111i8, 114i8, 121i8, 32i8, 117i8, 115i8, 97i8,
    103i8, 101i8, 32i8, 105i8, 110i8, 32i8, 98i8, 121i8, 116i8, 101i8, 115i8, 32i8, 111i8, 102i8,
    32i8, 116i8, 104i8, 101i8, 32i8, 116i8, 114i8, 97i8, 99i8, 101i8, 109i8, 97i8, 108i8, 108i8,
    111i8, 99i8, 32i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 46i8, 10i8, 10i8, 84i8, 104i8,
    105i8, 115i8, 32i8, 109i8, 101i8, 109i8, 111i8, 114i8, 121i8, 32i8, 105i8, 115i8, 32i8, 117i8,
    115i8, 101i8, 100i8, 32i8, 105i8, 110i8, 116i8, 101i8, 114i8, 110i8, 97i8, 108i8, 108i8, 121i8,
    32i8, 116i8, 111i8, 32i8, 116i8, 114i8, 97i8, 99i8, 101i8, 32i8, 109i8, 101i8, 109i8, 111i8,
    114i8, 121i8, 32i8, 97i8, 108i8, 108i8, 111i8, 99i8, 97i8, 116i8, 105i8, 111i8, 110i8, 115i8,
    46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct._tracemalloc._tracemalloc_get_tracemalloc_memory")]
extern "C-unwind" fn _tracemalloc_get_tracemalloc_memory(
    module: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    _tracemalloc_get_tracemalloc_memory_impl(module)
}

static _tracemalloc_get_traced_memory__doc__: __SlateAlign16<[i8; 154]> = __SlateAlign16([
    103i8, 101i8, 116i8, 95i8, 116i8, 114i8, 97i8, 99i8, 101i8, 100i8, 95i8, 109i8, 101i8, 109i8,
    111i8, 114i8, 121i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 44i8, 32i8, 47i8,
    41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 71i8, 101i8, 116i8, 32i8, 116i8, 104i8, 101i8, 32i8, 99i8,
    117i8, 114i8, 114i8, 101i8, 110i8, 116i8, 32i8, 115i8, 105i8, 122i8, 101i8, 32i8, 97i8, 110i8,
    100i8, 32i8, 112i8, 101i8, 97i8, 107i8, 32i8, 115i8, 105i8, 122i8, 101i8, 32i8, 111i8, 102i8,
    32i8, 109i8, 101i8, 109i8, 111i8, 114i8, 121i8, 32i8, 98i8, 108i8, 111i8, 99i8, 107i8, 115i8,
    32i8, 116i8, 114i8, 97i8, 99i8, 101i8, 100i8, 32i8, 98i8, 121i8, 32i8, 116i8, 114i8, 97i8,
    99i8, 101i8, 109i8, 97i8, 108i8, 108i8, 111i8, 99i8, 46i8, 10i8, 10i8, 82i8, 101i8, 116i8,
    117i8, 114i8, 110i8, 115i8, 32i8, 97i8, 32i8, 116i8, 117i8, 112i8, 108i8, 101i8, 58i8, 32i8,
    40i8, 99i8, 117i8, 114i8, 114i8, 101i8, 110i8, 116i8, 58i8, 32i8, 105i8, 110i8, 116i8, 44i8,
    32i8, 112i8, 101i8, 97i8, 107i8, 58i8, 32i8, 105i8, 110i8, 116i8, 41i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct._tracemalloc._tracemalloc_get_traced_memory")]
extern "C-unwind" fn _tracemalloc_get_traced_memory(
    module: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    _tracemalloc_get_traced_memory_impl(module)
}

static _tracemalloc_reset_peak__doc__: __SlateAlign16<[i8; 178]> = __SlateAlign16([
    114i8, 101i8, 115i8, 101i8, 116i8, 95i8, 112i8, 101i8, 97i8, 107i8, 40i8, 36i8, 109i8, 111i8,
    100i8, 117i8, 108i8, 101i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 83i8, 101i8,
    116i8, 32i8, 116i8, 104i8, 101i8, 32i8, 112i8, 101i8, 97i8, 107i8, 32i8, 115i8, 105i8, 122i8,
    101i8, 32i8, 111i8, 102i8, 32i8, 109i8, 101i8, 109i8, 111i8, 114i8, 121i8, 32i8, 98i8, 108i8,
    111i8, 99i8, 107i8, 115i8, 32i8, 116i8, 114i8, 97i8, 99i8, 101i8, 100i8, 32i8, 98i8, 121i8,
    32i8, 116i8, 114i8, 97i8, 99i8, 101i8, 109i8, 97i8, 108i8, 108i8, 111i8, 99i8, 32i8, 116i8,
    111i8, 32i8, 116i8, 104i8, 101i8, 32i8, 99i8, 117i8, 114i8, 114i8, 101i8, 110i8, 116i8, 32i8,
    115i8, 105i8, 122i8, 101i8, 46i8, 10i8, 10i8, 68i8, 111i8, 32i8, 110i8, 111i8, 116i8, 104i8,
    105i8, 110i8, 103i8, 32i8, 105i8, 102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 116i8, 114i8, 97i8,
    99i8, 101i8, 109i8, 97i8, 108i8, 108i8, 111i8, 99i8, 32i8, 109i8, 111i8, 100i8, 117i8, 108i8,
    101i8, 32i8, 105i8, 115i8, 32i8, 110i8, 111i8, 116i8, 32i8, 116i8, 114i8, 97i8, 99i8, 105i8,
    110i8, 103i8, 32i8, 109i8, 101i8, 109i8, 111i8, 114i8, 121i8, 32i8, 97i8, 108i8, 108i8, 111i8,
    99i8, 97i8, 116i8, 105i8, 111i8, 110i8, 115i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct._tracemalloc._tracemalloc_reset_peak")]
extern "C-unwind" fn _tracemalloc_reset_peak(
    module: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    _tracemalloc_reset_peak_impl(module)
}

// [clinic input]
// module _tracemalloc
// [clinic start generated code]
//
// [clinic end generated code: output=da39a3ee5e6b4b0d input=708a98302fc46e5f]
// [clinic input]
// _tracemalloc.is_tracing
//
// Return True if the tracemalloc module is tracing Python memory allocations.
// [clinic start generated code]
/// [clinic end generated code: output=2d763b42601cd3ef input=af104b0a00192f63]
fn _tracemalloc_is_tracing_impl(module: *mut _object) -> *mut _object {
    unsafe { PyBool_FromLong(_PyTraceMalloc_IsTracing() as i64) }
}

// [clinic input]
// _tracemalloc.clear_traces
//
// Clear traces of memory blocks allocated by Python.
// [clinic start generated code]
/// [clinic end generated code: output=a86080ee41b84197 input=0dab5b6c785183a5]
fn _tracemalloc_clear_traces_impl(module: *mut _object) -> *mut _object {
    unsafe { _PyTraceMalloc_ClearTraces() };
    unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
}

// [clinic input]
// _tracemalloc._get_traces
//
// Get traces of all memory blocks allocated by Python.
//
// Return a list of (size: int, traceback: tuple) tuples.
// traceback is a tuple of (filename: str, lineno: int) tuples.
//
// Return an empty list if the tracemalloc module is disabled.
// [clinic start generated code]
/// [clinic end generated code: output=e9929876ced4b5cc input=6c7d2230b24255aa]
fn _tracemalloc__get_traces_impl(module: *mut _object) -> *mut _object {
    unsafe { _PyTraceMalloc_GetTraces() }
}

// [clinic input]
// _tracemalloc._get_object_traceback
//
//     obj: object
//     /
//
// Get the traceback where the Python object obj was allocated.
//
// Return a tuple of (filename: str, lineno: int) tuples.
// Return None if the tracemalloc module is disabled or did not
// trace the allocation of the object.
// [clinic start generated code]
/// [clinic end generated code: output=41ee0553a658b0aa input=29495f1b21c53212]
#[unsafe(link_section = ".text.slate_distinct._tracemalloc._tracemalloc__get_object_traceback")]
extern "C-unwind" fn _tracemalloc__get_object_traceback(
    module: *mut _object,
    obj: *mut _object,
) -> *mut _object {
    unsafe { _PyTraceMalloc_GetObjectTraceback(obj) }
}

// [clinic input]
// _tracemalloc.start
//
//     nframe: int = 1
//     /
//
// Start tracing Python memory allocations.
//
// Also set the maximum number of frames stored in the traceback of a
// trace to nframe.
// [clinic start generated code]
/// [clinic end generated code: output=caae05c23c159d3c input=40d849b5b29d1933]
fn _tracemalloc_start_impl(module: *mut _object, nframe: i32) -> *mut _object {
    if (unsafe { _PyTraceMalloc_Start(nframe) }) < 0 {
        return core::ptr::null_mut::<_object>();
    }
    unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
}

// [clinic input]
// _tracemalloc.stop
//
// Stop tracing Python memory allocations.
//
// Also clear traces of memory blocks allocated by Python.
// [clinic start generated code]
/// [clinic end generated code: output=c3c42ae03e3955cd input=7478f075e51dae18]
fn _tracemalloc_stop_impl(module: *mut _object) -> *mut _object {
    unsafe { _PyTraceMalloc_Stop() };
    unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
}

// [clinic input]
// _tracemalloc.get_traceback_limit
//
// Get the maximum number of frames stored in the traceback of a trace.
//
// By default, a trace of an allocated memory block only stores
// the most recent frame: the limit is 1.
// [clinic start generated code]
/// [clinic end generated code: output=d556d9306ba95567 input=da3cd977fc68ae3b]
fn _tracemalloc_get_traceback_limit_impl(module: *mut _object) -> *mut _object {
    unsafe { PyLong_FromLong(_PyTraceMalloc_GetTracebackLimit() as i64) }
}

// [clinic input]
// _tracemalloc.get_tracemalloc_memory
//
// Get the memory usage in bytes of the tracemalloc module.
//
// This memory is used internally to trace memory allocations.
// [clinic start generated code]
/// [clinic end generated code: output=e3f14e280a55f5aa input=5d919c0f4d5132ad]
fn _tracemalloc_get_tracemalloc_memory_impl(module: *mut _object) -> *mut _object {
    unsafe { PyLong_FromSize_t(_PyTraceMalloc_GetMemory()) }
}

// [clinic input]
// _tracemalloc.get_traced_memory
//
// Get the current size and peak size of memory blocks traced by tracemalloc.
//
// Returns a tuple: (current: int, peak: int).
// [clinic start generated code]
/// [clinic end generated code: output=5b167189adb9e782 input=61ddb5478400ff66]
fn _tracemalloc_get_traced_memory_impl(module: *mut _object) -> *mut _object {
    unsafe { _PyTraceMalloc_GetTracedMemory() }
}

// [clinic input]
// _tracemalloc.reset_peak
//
// Set the peak size of memory blocks traced by tracemalloc to the current size.
//
// Do nothing if the tracemalloc module is not tracing memory allocations.
//
// [clinic start generated code]
/// [clinic end generated code: output=140c2870f691dbb2 input=18afd0635066e9ce]
fn _tracemalloc_reset_peak_impl(module: *mut _object) -> *mut _object {
    unsafe { _PyTraceMalloc_ResetPeak() };
    unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
}

/// sentinel
static mut module_methods: __SlateAlign16<[PyMethodDef; 11]> = __SlateAlign16([
    {
        let mut __t0: PyMethodDef = unsafe { core::mem::zeroed() };
        __t0.ml_name = (b"is_tracing\0".as_ptr() as *mut i8) as *const i8;
        __t0.ml_meth = Some(_tracemalloc_is_tracing);
        __t0.ml_flags = 4i32;
        __t0.ml_doc = unsafe { core::ptr::addr_of!(_tracemalloc_is_tracing__doc__.0) as *const i8 };
        __t0
    },
    {
        let mut __t1: PyMethodDef = unsafe { core::mem::zeroed() };
        __t1.ml_name = (b"clear_traces\0".as_ptr() as *mut i8) as *const i8;
        __t1.ml_meth = Some(_tracemalloc_clear_traces);
        __t1.ml_flags = 4i32;
        __t1.ml_doc =
            unsafe { core::ptr::addr_of!(_tracemalloc_clear_traces__doc__.0) as *const i8 };
        __t1
    },
    {
        let mut __t2: PyMethodDef = unsafe { core::mem::zeroed() };
        __t2.ml_name = (b"_get_traces\0".as_ptr() as *mut i8) as *const i8;
        __t2.ml_meth = Some(_tracemalloc__get_traces);
        __t2.ml_flags = 4i32;
        __t2.ml_doc =
            unsafe { core::ptr::addr_of!(_tracemalloc__get_traces__doc__.0) as *const i8 };
        __t2
    },
    {
        let mut __t3: PyMethodDef = unsafe { core::mem::zeroed() };
        __t3.ml_name = (b"_get_object_traceback\0".as_ptr() as *mut i8) as *const i8;
        __t3.ml_meth = Some(_tracemalloc__get_object_traceback);
        __t3.ml_flags = 8i32;
        __t3.ml_doc = unsafe {
            core::ptr::addr_of!(_tracemalloc__get_object_traceback__doc__.0) as *const i8
        };
        __t3
    },
    {
        let mut __t4: PyMethodDef = unsafe { core::mem::zeroed() };
        __t4.ml_name = (b"start\0".as_ptr() as *mut i8) as *const i8;
        __t4.ml_meth = unsafe {
            core::mem::transmute::<
                Option<unsafe extern "C-unwind" fn()>,
                Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
            >(unsafe {
                core::mem::transmute::<
                    Option<
                        unsafe extern "C-unwind" fn(
                            *mut _object,
                            *const *mut _object,
                            i64,
                        ) -> *mut _object,
                    >,
                    Option<unsafe extern "C-unwind" fn()>,
                >(Some(_tracemalloc_start))
            })
        };
        __t4.ml_flags = 128i32;
        __t4.ml_doc = unsafe { core::ptr::addr_of!(_tracemalloc_start__doc__.0) as *const i8 };
        __t4
    },
    {
        let mut __t5: PyMethodDef = unsafe { core::mem::zeroed() };
        __t5.ml_name = (b"stop\0".as_ptr() as *mut i8) as *const i8;
        __t5.ml_meth = Some(_tracemalloc_stop);
        __t5.ml_flags = 4i32;
        __t5.ml_doc = unsafe { core::ptr::addr_of!(_tracemalloc_stop__doc__.0) as *const i8 };
        __t5
    },
    {
        let mut __t6: PyMethodDef = unsafe { core::mem::zeroed() };
        __t6.ml_name = (b"get_traceback_limit\0".as_ptr() as *mut i8) as *const i8;
        __t6.ml_meth = Some(_tracemalloc_get_traceback_limit);
        __t6.ml_flags = 4i32;
        __t6.ml_doc =
            unsafe { core::ptr::addr_of!(_tracemalloc_get_traceback_limit__doc__.0) as *const i8 };
        __t6
    },
    {
        let mut __t7: PyMethodDef = unsafe { core::mem::zeroed() };
        __t7.ml_name = (b"get_tracemalloc_memory\0".as_ptr() as *mut i8) as *const i8;
        __t7.ml_meth = Some(_tracemalloc_get_tracemalloc_memory);
        __t7.ml_flags = 4i32;
        __t7.ml_doc = unsafe {
            core::ptr::addr_of!(_tracemalloc_get_tracemalloc_memory__doc__.0) as *const i8
        };
        __t7
    },
    {
        let mut __t8: PyMethodDef = unsafe { core::mem::zeroed() };
        __t8.ml_name = (b"get_traced_memory\0".as_ptr() as *mut i8) as *const i8;
        __t8.ml_meth = Some(_tracemalloc_get_traced_memory);
        __t8.ml_flags = 4i32;
        __t8.ml_doc =
            unsafe { core::ptr::addr_of!(_tracemalloc_get_traced_memory__doc__.0) as *const i8 };
        __t8
    },
    {
        let mut __t9: PyMethodDef = unsafe { core::mem::zeroed() };
        __t9.ml_name = (b"reset_peak\0".as_ptr() as *mut i8) as *const i8;
        __t9.ml_meth = Some(_tracemalloc_reset_peak);
        __t9.ml_flags = 4i32;
        __t9.ml_doc = unsafe { core::ptr::addr_of!(_tracemalloc_reset_peak__doc__.0) as *const i8 };
        __t9
    },
    {
        let mut __t10: PyMethodDef = unsafe { core::mem::zeroed() };
        __t10.ml_name = core::ptr::null::<i8>();
        __t10.ml_meth = None;
        __t10
    },
]);

static module_doc: __SlateAlign16<[i8; 57]> = __SlateAlign16([
    68i8, 101i8, 98i8, 117i8, 103i8, 32i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 32i8, 116i8,
    111i8, 32i8, 116i8, 114i8, 97i8, 99i8, 101i8, 32i8, 109i8, 101i8, 109i8, 111i8, 114i8, 121i8,
    32i8, 98i8, 108i8, 111i8, 99i8, 107i8, 115i8, 32i8, 97i8, 108i8, 108i8, 111i8, 99i8, 97i8,
    116i8, 101i8, 100i8, 32i8, 98i8, 121i8, 32i8, 80i8, 121i8, 116i8, 104i8, 111i8, 110i8, 46i8,
    0i8,
]);

/// non-negative size to be able to unload the module
static mut module_def: PyModuleDef = {
    let mut __t0: PyModuleDef = unsafe { core::mem::zeroed() };
    __t0.m_base = PyModuleDef_Base {
        ob_base: _object {
            __slate_anon_0: __SlateRecord_object_h_122 {
                ob_refcnt_full: 1407378104778752i64,
            },
            ob_type: core::ptr::null_mut::<_typeobject>(),
        },
        m_init: None,
        m_index: 0i64,
        m_copy: core::ptr::null_mut::<_object>(),
    };
    __t0.m_name = (b"_tracemalloc\0".as_ptr() as *mut i8) as *const i8;
    __t0.m_doc = unsafe { core::ptr::addr_of!(module_doc.0) as *const i8 };
    __t0.m_size = 0i64;
    __t0.m_methods = unsafe { core::ptr::addr_of_mut!(module_methods.0) as *mut PyMethodDef };
    __t0.m_slots = core::ptr::null_mut::<PyModuleDef_Slot>();
    __t0
};

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct._tracemalloc.PyInit__tracemalloc")]
extern "C-unwind" fn PyInit__tracemalloc() -> *mut _object {
    let r#mod: *mut _object = PyModule_Create!(unsafe { core::ptr::addr_of_mut!(module_def) });
    if r#mod.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    r#mod
}
