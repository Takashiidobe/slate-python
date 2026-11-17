//! PickleBuffer object implementation
//! The size of 'void *', as computed by sizeof.
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
//! Bit flags for _gc_prev
//!
//! Bit 0 is set when tp_finalize is called
//! _PyGenObject_HEAD defines the initial segment of generator
//! and coroutine objects.
//! These would be in pycore_long.h if it weren't for an include cycle.
//! Fast inlined version of PyType_IS_GC()
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
macro_rules! FT_CLEAR_WEAKREFS {
    ($obj:expr, $weakref_list:expr) => {
        loop {
            if !$weakref_list.is_null() {
                unsafe { PyObject_ClearWeakRefs($obj) };
            }
            if true {
                break;
            }
        }
    };
}

macro_rules! PyPickleBuffer_Check {
    ($op:expr) => {
        Py_IS_TYPE!($op, unsafe { core::ptr::addr_of_mut!(PyPickleBuffer_Type) })
    };
}

macro_rules! Py_DECREF__2 {
    ($op:expr) => {
        Py_DECREF(_PyObject_CAST__2!($op));
    };
}

macro_rules! Py_IS_TYPE {
    ($ob:expr, $type:expr) => {
        Py_IS_TYPE(_PyObject_CAST!($ob), $type)
    };
}

macro_rules! Py_TYPE__2 {
    ($ob:expr) => {
        _Py_TYPE(_PyObject_CAST__2!($ob))
    };
}

macro_rules! Py_TYPE {
    ($ob:expr) => {
        _Py_TYPE(_PyObject_CAST!($ob))
    };
}

/// Utility macro to help write tp_traverse functions.
/// To use this macro, the tp_traverse function must name its arguments
/// "visit" and "arg".  This is intended to keep tp_traverse functions
/// looking as much alike as possible.
macro_rules! Py_VISIT {
    ($op:expr, $visit:expr, $arg:expr) => {{
        loop {
            if !$op.is_null() {
                let vret: i32 = unsafe { $visit.unwrap()(_PyObject_CAST!($op), $arg) };
                if vret != 0 {
                    return vret;
                }
            }
            if true {
                break;
            }
        }
    }};
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
    static mut _Py_NoneStruct: _object;
    static mut PyExc_BufferError: *mut _object;
    static mut PyExc_TypeError: *mut _object;
    static mut PyExc_ValueError: *mut _object;
    fn PyObject_GetBuffer(
        arg0: *mut _object,
        arg1: *mut __SlateRecord_pybuffer_h_33,
        arg2: i32,
    ) -> i32;
    fn PyBuffer_IsContiguous(arg0: *const __SlateRecord_pybuffer_h_33, arg1: i8) -> i32;
    fn PyBuffer_Release(arg0: *mut __SlateRecord_pybuffer_h_33);
    fn PyObject_ClearWeakRefs(arg0: *mut _object);
    fn _Py_Dealloc(arg0: *mut _object);
    fn PyObject_GC_UnTrack(arg0: *mut ());
    fn PyErr_SetString(arg0: *mut _object, arg1: *const i8);
    fn PyErr_Format(arg0: *mut _object, arg1: *const i8, ...) -> *mut _object;
    fn PyMemoryView_FromObject(arg0: *mut _object) -> *mut _object;
    fn PyArg_ParseTupleAndKeywords(
        arg0: *mut _object,
        arg1: *mut _object,
        arg2: *const i8,
        arg3: *const *mut i8,
        ...
    ) -> i32;
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

// Track types initialized using _PyStaticType_InitBuiltin().
// Objects support garbage collection (see objimpl.h)
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

// Define a pair of assertion macros:
// _PyObject_ASSERT_FROM(), _PyObject_ASSERT_WITH_MSG() and _PyObject_ASSERT().
//
// These work like the regular C assert(), in that they will abort the
// process with a message on stderr if the given condition fails to hold,
// but compile away to nothing if NDEBUG is defined.
//
// However, before aborting, Python will also try to call _PyObject_Dump() on
// the given object.  This may be of use when investigating bugs in which a
// particular object is corrupt (e.g. buggy a tp_visit method in an extension
// module breaking the garbage collector), to help locate the broken objects.
//
// The WITH_MSG variant allows you to supply an additional message that Python
// will attempt to print to stderr, after the object dump.
//
// No debugging: compile away the assertions:
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
struct __SlateRecord_memoryobject_h_16 {
    ob_base: _object,
    flags: i32,
    exports: i64,
    master: __SlateRecord_pybuffer_h_33,
}

// C-contiguous layout
// Fortran contiguous layout
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_memoryobject_h_36 {
    ob_base: __SlateRecord_object_h_172,
    mbuf: *mut __SlateRecord_memoryobject_h_16,
    hash: i64,
    flags: i32,
    exports: i64,
    view: __SlateRecord_pybuffer_h_33,
    weakreflist: *mut _object,
    ob_array: [i64; 1],
}

#[repr(C, align(16))]
struct __SlateAlign16<T>(T);

unsafe impl<T> Sync for __SlateAlign16<T> {}

#[repr(C)]
#[derive(Clone, Copy)]
struct PyMethodDef {
    ml_name: *const i8,
    ml_meth: Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
    ml_flags: i32,
    ml_doc: *const i8,
}

// METH_NOARGS and METH_O must not be combined with the flags above.
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

#[unsafe(no_mangle)]
static mut PyPickleBuffer_Type: _typeobject = {
    let mut __t0: _typeobject = unsafe { core::mem::zeroed() };
    __t0.ob_base = __SlateRecord_object_h_172 {
        ob_base: _object {
            __slate_anon_0: __SlateRecord_object_h_122 {
                ob_refcnt_full: 1407378104778752i64,
            },
            ob_type: core::ptr::null_mut::<_typeobject>(),
        },
        ob_size: 0i64,
    };
    __t0.tp_name = (b"pickle.PickleBuffer\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_picklebufobject_c_12>() as u64) as i64;
    __t0.tp_dealloc = Some(picklebuf_dealloc);
    __t0.tp_as_buffer = unsafe { core::ptr::addr_of_mut!(picklebuf_as_buffer) };
    __t0.tp_flags = 16384u64;
    __t0.tp_doc =
        (b"Wrapper for potentially out-of-band buffers\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_traverse = Some(picklebuf_traverse);
    __t0.tp_clear = Some(picklebuf_clear);
    __t0.tp_weaklistoffset =
        (core::mem::offset_of!(__SlateRecord_picklebufobject_c_12, weakreflist) as u64) as i64;
    __t0.tp_methods = unsafe { core::ptr::addr_of_mut!(picklebuf_methods.0) as *mut PyMethodDef };
    __t0.tp_new = Some(picklebuf_new);
    __t0
};

// FT_CLEAR_WEAKREFS()
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_picklebufobject_c_12 {
    ob_base: _object,
    /// The view exported by the original object
    view: __SlateRecord_pybuffer_h_33,
    weakreflist: *mut _object,
}

// C API
#[unsafe(no_mangle)]
extern "C-unwind" fn PyPickleBuffer_FromObject(base: *mut _object) -> *mut _object {
    let r#type: *mut _typeobject = unsafe { core::ptr::addr_of_mut!(PyPickleBuffer_Type) };
    let mut self_: *mut __SlateRecord_picklebufobject_c_12 = unsafe { core::mem::zeroed() };
    self_ = (unsafe { unsafe { (*r#type).tp_alloc }.unwrap()(r#type, 0i64) })
        as *mut __SlateRecord_picklebufobject_c_12;
    if self_.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    unsafe {
        (*self_).view.obj = core::ptr::null_mut::<_object>();
    }
    unsafe {
        (*self_).weakreflist = core::ptr::null_mut::<_object>();
    }
    if (unsafe { PyObject_GetBuffer(base, core::ptr::addr_of_mut!((*self_).view), 284i32) }) < 0 {
        Py_DECREF__2!(self_);
        return core::ptr::null_mut::<_object>();
    }
    self_ as *mut _object
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyPickleBuffer_GetBuffer(
    obj: *mut _object,
) -> *const __SlateRecord_pybuffer_h_33 {
    if PyPickleBuffer_Check!(obj) == 0 {
        unsafe {
            PyErr_Format(
                PyExc_TypeError,
                c"expected PickleBuffer, %.200s found".as_ptr(),
                (*Py_TYPE!(obj)).tp_name,
            )
        };
        return core::ptr::null::<__SlateRecord_pybuffer_h_33>();
    }
    if unsafe { (*(obj as *mut __SlateRecord_picklebufobject_c_12)).view.obj }.is_null() {
        unsafe {
            PyErr_SetString(
                PyExc_ValueError,
                c"operation forbidden on released PickleBuffer object".as_ptr(),
            )
        };
        return core::ptr::null::<__SlateRecord_pybuffer_h_33>();
    }
    (unsafe { core::ptr::addr_of_mut!((*(obj as *mut __SlateRecord_picklebufobject_c_12)).view) })
        as *const __SlateRecord_pybuffer_h_33
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyPickleBuffer_Release(obj: *mut _object) -> i32 {
    if PyPickleBuffer_Check!(obj) == 0 {
        unsafe {
            PyErr_Format(
                PyExc_TypeError,
                c"expected PickleBuffer, %.200s found".as_ptr(),
                (*Py_TYPE!(obj)).tp_name,
            )
        };
        return 1i32.wrapping_neg();
    }
    unsafe {
        PyBuffer_Release(core::ptr::addr_of_mut!(
            (*(obj as *mut __SlateRecord_picklebufobject_c_12)).view
        ))
    };
    0
}

#[unsafe(link_section = ".text.slate_distinct.picklebufobject.picklebuf_new")]
extern "C-unwind" fn picklebuf_new(
    r#type: *mut _typeobject,
    args: *mut _object,
    kwds: *mut _object,
) -> *mut _object {
    let mut self_: *mut __SlateRecord_picklebufobject_c_12 = unsafe { core::mem::zeroed() };
    let mut base: *mut _object = unsafe { core::mem::zeroed() };
    let mut keywords: __SlateAlign16<[*mut i8; 2]> =
        __SlateAlign16([c"".as_ptr() as *mut i8, core::ptr::null_mut::<i8>()]);
    if (unsafe {
        PyArg_ParseTupleAndKeywords(
            args,
            kwds,
            c"O:PickleBuffer".as_ptr(),
            keywords.0.as_mut_ptr() as *const *mut i8,
            core::ptr::addr_of_mut!(base),
        )
    }) == 0
    {
        return core::ptr::null_mut::<_object>();
    }
    self_ = (unsafe { unsafe { (*r#type).tp_alloc }.unwrap()(r#type, 0i64) })
        as *mut __SlateRecord_picklebufobject_c_12;
    if self_.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    unsafe {
        (*self_).view.obj = core::ptr::null_mut::<_object>();
    }
    unsafe {
        (*self_).weakreflist = core::ptr::null_mut::<_object>();
    }
    if (unsafe { PyObject_GetBuffer(base, core::ptr::addr_of_mut!((*self_).view), 284i32) }) < 0 {
        Py_DECREF__2!(self_);
        return core::ptr::null_mut::<_object>();
    }
    self_ as *mut _object
}

#[unsafe(link_section = ".text.slate_distinct.picklebufobject.picklebuf_traverse")]
extern "C-unwind" fn picklebuf_traverse(
    op: *mut _object,
    mut visit: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
    arg: *mut (),
) -> i32 {
    let self_: *mut __SlateRecord_picklebufobject_c_12 =
        op as *mut __SlateRecord_picklebufobject_c_12;
    Py_VISIT!(unsafe { (*self_).view.obj }, visit, arg);
    0
}

#[unsafe(link_section = ".text.slate_distinct.picklebufobject.picklebuf_clear")]
extern "C-unwind" fn picklebuf_clear(op: *mut _object) -> i32 {
    unsafe {
        PyBuffer_Release(core::ptr::addr_of_mut!(
            (*(op as *mut __SlateRecord_picklebufobject_c_12)).view
        ))
    };
    0
}

#[unsafe(link_section = ".text.slate_distinct.picklebufobject.picklebuf_dealloc")]
extern "C-unwind" fn picklebuf_dealloc(op: *mut _object) {
    let self_: *mut __SlateRecord_picklebufobject_c_12 =
        op as *mut __SlateRecord_picklebufobject_c_12;
    unsafe { PyObject_GC_UnTrack(self_ as *mut ()) };
    FT_CLEAR_WEAKREFS!(op, unsafe { (*self_).weakreflist });
    unsafe { PyBuffer_Release(core::ptr::addr_of_mut!((*self_).view)) };
    unsafe { unsafe { (*Py_TYPE__2!(self_)).tp_free }.unwrap()(self_ as *mut ()) };
}

// Buffer API
#[unsafe(link_section = ".text.slate_distinct.picklebufobject.picklebuf_getbuf")]
extern "C-unwind" fn picklebuf_getbuf(
    op: *mut _object,
    view: *mut __SlateRecord_pybuffer_h_33,
    flags: i32,
) -> i32 {
    if unsafe { (*(op as *mut __SlateRecord_picklebufobject_c_12)).view.obj }.is_null() {
        unsafe {
            PyErr_SetString(
                PyExc_ValueError,
                c"operation forbidden on released PickleBuffer object".as_ptr(),
            )
        };
        return 1i32.wrapping_neg();
    }
    unsafe {
        PyObject_GetBuffer(
            (*(op as *mut __SlateRecord_picklebufobject_c_12)).view.obj,
            view,
            flags,
        )
    }
}

#[unsafe(link_section = ".text.slate_distinct.picklebufobject.picklebuf_releasebuf")]
extern "C-unwind" fn picklebuf_releasebuf(
    self_: *mut _object,
    view: *mut __SlateRecord_pybuffer_h_33,
) {
    // Since our bf_getbuffer redirects to the original object, this
    // implementation is never called.  It only exists to signal that
    // buffers exported by PickleBuffer have non-trivial releasing
    // behaviour (see check in Python/getargs.c).
}

static mut picklebuf_as_buffer: __SlateRecord_object_h_140 = __SlateRecord_object_h_140 {
    bf_getbuffer: Some(picklebuf_getbuf),
    bf_releasebuffer: Some(picklebuf_releasebuf),
};

// Methods
#[unsafe(link_section = ".text.slate_distinct.picklebufobject.picklebuf_raw")]
extern "C-unwind" fn picklebuf_raw(
    op: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    let self_: *mut __SlateRecord_picklebufobject_c_12 =
        op as *mut __SlateRecord_picklebufobject_c_12;
    if unsafe { (*self_).view.obj }.is_null() {
        unsafe {
            PyErr_SetString(
                PyExc_ValueError,
                c"operation forbidden on released PickleBuffer object".as_ptr(),
            )
        };
        return core::ptr::null_mut::<_object>();
    }
    let __v0: bool = !unsafe { (*self_).view.suboffsets }.is_null()
        || (unsafe {
            PyBuffer_IsContiguous(
                core::ptr::addr_of_mut!((*self_).view) as *const __SlateRecord_pybuffer_h_33,
                65i8,
            )
        }) == 0;
    if __v0 {
        unsafe {
            PyErr_SetString(
                PyExc_BufferError,
                c"cannot extract raw buffer from non-contiguous buffer".as_ptr(),
            )
        };
        return core::ptr::null_mut::<_object>();
    }
    let m: *mut _object = unsafe { PyMemoryView_FromObject(self_ as *mut _object) };
    if m.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    // Mutate memoryview instance to make it a "raw" memoryview
    unsafe {
        (*(m as *mut __SlateRecord_memoryobject_h_36)).view.format = c"B".as_ptr() as *mut i8;
    }
    unsafe {
        (*(m as *mut __SlateRecord_memoryobject_h_36)).view.ndim = 1;
    }
    unsafe {
        (*(m as *mut __SlateRecord_memoryobject_h_36)).view.itemsize = 1;
    }
    // shape = (length,)
    unsafe {
        (*(m as *mut __SlateRecord_memoryobject_h_36)).view.shape =
            core::ptr::addr_of_mut!((*(m as *mut __SlateRecord_memoryobject_h_36)).view.len);
    }
    // strides = (1,)
    unsafe {
        (*(m as *mut __SlateRecord_memoryobject_h_36)).view.strides =
            core::ptr::addr_of_mut!((*(m as *mut __SlateRecord_memoryobject_h_36)).view.itemsize);
    }
    // Fix memoryview state flags
    //
    // XXX Expose memoryobject.c's init_flags() instead?
    unsafe {
        (*(m as *mut __SlateRecord_memoryobject_h_36)).flags = 2 | 4;
    }
    m
}

static picklebuf_raw_doc: __SlateAlign16<[i8; 135]> = __SlateAlign16([
    114i8, 97i8, 119i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8,
    45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 97i8, 32i8, 109i8, 101i8,
    109i8, 111i8, 114i8, 121i8, 118i8, 105i8, 101i8, 119i8, 32i8, 111i8, 102i8, 32i8, 116i8, 104i8,
    101i8, 32i8, 114i8, 97i8, 119i8, 32i8, 109i8, 101i8, 109i8, 111i8, 114i8, 121i8, 32i8, 117i8,
    110i8, 100i8, 101i8, 114i8, 108i8, 121i8, 105i8, 110i8, 103i8, 32i8, 116i8, 104i8, 105i8,
    115i8, 32i8, 98i8, 117i8, 102i8, 102i8, 101i8, 114i8, 46i8, 10i8, 87i8, 105i8, 108i8, 108i8,
    32i8, 114i8, 97i8, 105i8, 115i8, 101i8, 32i8, 66i8, 117i8, 102i8, 102i8, 101i8, 114i8, 69i8,
    114i8, 114i8, 111i8, 114i8, 32i8, 105i8, 115i8, 32i8, 116i8, 104i8, 101i8, 32i8, 98i8, 117i8,
    102i8, 102i8, 101i8, 114i8, 32i8, 105i8, 115i8, 110i8, 39i8, 116i8, 32i8, 99i8, 111i8, 110i8,
    116i8, 105i8, 103i8, 117i8, 111i8, 117i8, 115i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.picklebufobject.picklebuf_release")]
extern "C-unwind" fn picklebuf_release(
    op: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    unsafe {
        PyBuffer_Release(core::ptr::addr_of_mut!(
            (*(op as *mut __SlateRecord_picklebufobject_c_12)).view
        ))
    };
    unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
}

static picklebuf_release_doc: __SlateAlign16<[i8; 88]> = __SlateAlign16([
    114i8, 101i8, 108i8, 101i8, 97i8, 115i8, 101i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8,
    32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 108i8, 101i8, 97i8, 115i8, 101i8,
    32i8, 116i8, 104i8, 101i8, 32i8, 117i8, 110i8, 100i8, 101i8, 114i8, 108i8, 121i8, 105i8, 110i8,
    103i8, 32i8, 98i8, 117i8, 102i8, 102i8, 101i8, 114i8, 32i8, 101i8, 120i8, 112i8, 111i8, 115i8,
    101i8, 100i8, 32i8, 98i8, 121i8, 32i8, 116i8, 104i8, 101i8, 32i8, 80i8, 105i8, 99i8, 107i8,
    108i8, 101i8, 66i8, 117i8, 102i8, 102i8, 101i8, 114i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8,
    116i8, 46i8, 0i8,
]);

static mut picklebuf_methods: __SlateAlign16<[PyMethodDef; 3]> = __SlateAlign16([
    {
        let mut __t0: PyMethodDef = unsafe { core::mem::zeroed() };
        __t0.ml_name = (b"raw\0".as_ptr() as *mut i8) as *const i8;
        __t0.ml_meth = Some(picklebuf_raw);
        __t0.ml_flags = 4i32;
        __t0.ml_doc = unsafe { core::ptr::addr_of!(picklebuf_raw_doc.0) as *const i8 };
        __t0
    },
    {
        let mut __t1: PyMethodDef = unsafe { core::mem::zeroed() };
        __t1.ml_name = (b"release\0".as_ptr() as *mut i8) as *const i8;
        __t1.ml_meth = Some(picklebuf_release);
        __t1.ml_flags = 4i32;
        __t1.ml_doc = unsafe { core::ptr::addr_of!(picklebuf_release_doc.0) as *const i8 };
        __t1
    },
    {
        let mut __t2: PyMethodDef = unsafe { core::mem::zeroed() };
        __t2.ml_name = core::ptr::null::<i8>();
        __t2.ml_meth = None;
        __t2
    },
]);
