//! namespace object implementation
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
macro_rules! PyDict_CheckExact {
    ($op:expr) => {
        Py_IS_TYPE!($op, unsafe { core::ptr::addr_of_mut!(PyDict_Type) })
    };
}

macro_rules! PyObject_TypeCheck {
    ($ob:expr, $type:expr) => {
        PyObject_TypeCheck(_PyObject_CAST!($ob), $type)
    };
}

macro_rules! PyType_FastSubclass {
    ($type:expr, $flag:expr) => {
        PyType_HasFeature($type, $flag)
    };
}

macro_rules! PyUnicode_Check {
    ($op:expr) => {
        PyType_FastSubclass!(Py_TYPE!($op), 268435456u64)
    };
}

macro_rules! PyUnicode_GET_LENGTH {
    ($op:expr) => {
        PyUnicode_GET_LENGTH(_PyObject_CAST!($op))
    };
}

macro_rules! Py_CLEAR {
    ($op:expr) => {
        loop {
            let _tmp_op_ptr: *mut *mut _object = unsafe { core::ptr::addr_of_mut!($op) };
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

macro_rules! Py_DECREF__2 {
    ($op:expr) => {
        Py_DECREF(_PyObject_CAST__2!($op));
    };
}

macro_rules! Py_DECREF {
    ($op:expr) => {
        Py_DECREF(_PyObject_CAST!($op));
    };
}

macro_rules! Py_INCREF {
    ($op:expr) => {
        Py_INCREF(_PyObject_CAST!($op));
    };
}

macro_rules! Py_IS_TYPE {
    ($ob:expr, $type:expr) => {
        Py_IS_TYPE(_PyObject_CAST!($ob), $type)
    };
}

macro_rules! Py_NewRef {
    ($obj:expr) => {
        _Py_NewRef(_PyObject_CAST!($obj))
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

macro_rules! Py_XDECREF {
    ($op:expr) => {
        Py_XDECREF(_PyObject_CAST!($op));
    };
}

macro_rules! _PyASCIIObject_CAST {
    ($op:expr) => {
        _Py_CAST!(*mut __SlateRecord_unicodeobject_h_163, $op)
    };
}

macro_rules! _PyArg_NoPositional {
    ($funcname:expr, $args:expr) => {{
        let __v: bool = $args.is_null()
            || (unsafe { _PyArg_NoPositional(($funcname) as *const i8, $args) }) != 0;
        __v
    }};
}

macro_rules! _PyNamespace_CAST {
    ($op:expr) => {
        _Py_CAST!(*mut __SlateRecord_namespaceobject_c_13, $op)
    };
}

macro_rules! _PyNamespace_Check {
    ($op:expr) => {
        PyObject_TypeCheck!($op, unsafe { core::ptr::addr_of_mut!(_PyNamespace_Type) })
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

macro_rules! _Py_INCREF_IMMORTAL_STAT_INC {
    () => {};
}

macro_rules! _Py_INCREF_STAT_INC {
    () => {};
}

macro_rules! _Py_IsImmortal {
    ($op:expr) => {
        _Py_IsImmortal(_PyObject_CAST!($op))
    };
}

unsafe extern "C" {
    static mut PyType_Type: _typeobject;
    static mut _Py_NotImplementedStruct: _object;
    static mut PyExc_TypeError: *mut _object;
    static mut PyDict_Type: _typeobject;
    fn PyType_IsSubtype(arg0: *mut _typeobject, arg1: *mut _typeobject) -> i32;
    fn PyType_GenericAlloc(arg0: *mut _typeobject, arg1: i64) -> *mut _object;
    fn PyObject_RichCompare(arg0: *mut _object, arg1: *mut _object, arg2: i32) -> *mut _object;
    fn PyObject_GenericGetAttr(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn PyObject_GenericSetAttr(arg0: *mut _object, arg1: *mut _object, arg2: *mut _object) -> i32;
    fn Py_ReprEnter(arg0: *mut _object) -> i32;
    fn Py_ReprLeave(arg0: *mut _object);
    fn _PyType_Name(arg0: *mut _typeobject) -> *const i8;
    fn _Py_Dealloc(arg0: *mut _object);
    fn PyObject_GC_UnTrack(arg0: *mut ());
    fn PyObject_GC_Del(arg0: *mut ());
    fn PyUnicode_FromString(arg0: *const i8) -> *mut _object;
    fn PyUnicode_FromFormat(arg0: *const i8, ...) -> *mut _object;
    fn PyUnicode_Join(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn PyErr_Occurred() -> *mut _object;
    fn PyErr_Format(arg0: *mut _object, arg1: *const i8, ...) -> *mut _object;
    fn PyTuple_New(arg0: i64) -> *mut _object;
    fn PyTuple_Pack(arg0: i64, ...) -> *mut _object;
    fn PyList_New(arg0: i64) -> *mut _object;
    fn PyList_Append(arg0: *mut _object, arg1: *mut _object) -> i32;
    fn PyDict_New() -> *mut _object;
    fn PyDict_Keys(arg0: *mut _object) -> *mut _object;
    fn PyDict_Update(arg0: *mut _object, arg1: *mut _object) -> i32;
    fn PyDict_GetItemRef(arg0: *mut _object, arg1: *mut _object, arg2: *mut *mut _object) -> i32;
    fn PyArg_ValidateKeywordArguments(arg0: *mut _object) -> i32;
    fn PyArg_UnpackTuple(arg0: *mut _object, arg1: *const i8, arg2: i64, arg3: i64, ...) -> i32;
    fn PyObject_CallNoArgs(arg0: *mut _object) -> *mut _object;
    fn PyObject_GetIter(arg0: *mut _object) -> *mut _object;
    fn PyIter_Next(arg0: *mut _object) -> *mut _object;
    fn PyObject_CallOneArg(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn _PyArg_NoPositional(arg0: *const i8, arg1: *mut _object) -> i32;
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

fn PyObject_TypeCheck(ob: *mut _object, r#type: *mut _typeobject) -> i32 {
    let __v0: bool =
        Py_IS_TYPE!(ob, r#type) != 0 || (unsafe { PyType_IsSubtype(Py_TYPE!(ob), r#type) }) != 0;
    __v0 as i32
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

// Set if the type allows subclassing
// Objects support garbage collection (see objimpl.h)
fn PyType_HasFeature(r#type: *mut _typeobject, feature: u64) -> i32 {
    let flags: u64 = unsafe { (*r#type).tp_flags };
    (flags & feature != 0) as i32
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_unicodeobject_h_163_2 {
    __slate_bits_0: __slate_bits::__SlateBits73U0,
}

#[inline(always)]
fn _Py_IsImmortal(op: *mut _object) -> i32 {
    (_Py_CAST!(i32, unsafe {
        (*op).__slate_anon_0.__slate_anon_1.ob_refcnt
    }) < 0) as i32
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
    pub struct __SlateBits73U0 {
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

#[inline(always)]
fn Py_INCREF(op: *mut _object) {
    // Non-limited C API and limited C API for Python 3.9 and older access
    //
    // directly PyObject.ob_refcnt.
    let cur_refcnt: u32 = unsafe { (*op).__slate_anon_0.__slate_anon_1.ob_refcnt };
    if (cur_refcnt as u64) >= 3221225472u64 {
        // the object is immortal
        _Py_INCREF_IMMORTAL_STAT_INC!();
        return;
    }
    unsafe {
        (*op).__slate_anon_0.__slate_anon_1.ob_refcnt = cur_refcnt.wrapping_add(1);
    }
    _Py_INCREF_STAT_INC!();
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

fn _Py_NewRef(obj: *mut _object) -> *mut _object {
    Py_INCREF!(obj);
    obj
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_unicodeobject_h_163 {
    ob_base: _object,
    length: i64,
    hash: i64,
    state: __SlateRecord_unicodeobject_h_163_2,
}

// PyUnicode_KIND(): Return one of the PyUnicode_*_KIND values defined above.
//
//
//
// gh-89653: Converting this macro to a static inline function would introduce
//
// new compiler warnings on "kind < PyUnicode_KIND(str)" (compare signed and
//
// unsigned numbers) where kind type is an int or on
//
// "unsigned int kind = PyUnicode_KIND(str)" (cast signed to unsigned).
fn PyUnicode_GET_LENGTH(op: *mut _object) -> i64 {
    unsafe { (*_PyASCIIObject_CAST!(op)).length }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct PyMethodDef {
    ml_name: *const i8,
    ml_meth: Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
    ml_flags: i32,
    ml_doc: *const i8,
}

// Flag passed to newmethodobject
//
// #define METH_OLDARGS  0x0000   -- unsupported now
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

// Deprecated, use Py_T_OBJECT_EX instead
// Flags
#[unsafe(no_mangle)]
static mut _PyNamespace_Type: _typeobject = {
    let mut __t0: _typeobject = unsafe { core::mem::zeroed() };
    __t0.ob_base = __SlateRecord_object_h_172 {
        ob_base: _object {
            __slate_anon_0: __SlateRecord_object_h_122 {
                ob_refcnt_full: 1407378104778752i64,
            },
            ob_type: unsafe { core::ptr::addr_of_mut!(PyType_Type) },
        },
        ob_size: 0i64,
    };
    __t0.tp_name = (b"types.SimpleNamespace\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_namespaceobject_c_13>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(namespace_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = Some(namespace_repr);
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = None;
    __t0.tp_getattro = unsafe {
        core::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
        >(PyObject_GenericGetAttr as *const ())
    };
    __t0.tp_setattro = unsafe {
        core::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object, *mut _object) -> i32>,
        >(PyObject_GenericSetAttr as *const ())
    };
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 17408u64;
    __t0.tp_doc = unsafe { core::ptr::addr_of!(namespace_doc.0) as *const i8 };
    __t0.tp_traverse = Some(namespace_traverse);
    __t0.tp_clear = Some(namespace_clear);
    __t0.tp_richcompare = Some(namespace_richcompare);
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = unsafe { core::ptr::addr_of_mut!(namespace_methods.0) as *mut PyMethodDef };
    __t0.tp_members = unsafe { core::ptr::addr_of_mut!(namespace_members.0) as *mut PyMemberDef };
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = core::ptr::null_mut::<_typeobject>();
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset =
        (core::mem::offset_of!(__SlateRecord_namespaceobject_c_13, ns_dict) as u64) as i64;
    __t0.tp_init = Some(namespace_init);
    __t0.tp_alloc = unsafe {
        core::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut _typeobject, i64) -> *mut _object>,
        >(PyType_GenericAlloc as *const ())
    };
    __t0.tp_new = Some(namespace_new);
    __t0.tp_free = unsafe {
        core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
            PyObject_GC_Del as *const (),
        )
    };
    __t0
};

// _PyArg_NoPositional()
// _PyNamespace_Type
// offsetof()
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_namespaceobject_c_13 {
    ob_base: _object,
    ns_dict: *mut _object,
}

static mut namespace_members: __SlateAlign16<[PyMemberDef; 2]> = __SlateAlign16([
    {
        let mut __t0: PyMemberDef = unsafe { core::mem::zeroed() };
        __t0.name = (b"__dict__\0".as_ptr() as *mut i8) as *const i8;
        __t0.r#type = 6i32;
        __t0.offset =
            (core::mem::offset_of!(__SlateRecord_namespaceobject_c_13, ns_dict) as u64) as i64;
        __t0.flags = 1i32;
        __t0
    },
    {
        let mut __t1: PyMemberDef = unsafe { core::mem::zeroed() };
        __t1.name = core::ptr::null::<i8>();
        __t1
    },
]);

// Methods
#[unsafe(link_section = ".text.slate_distinct.namespaceobject.namespace_new")]
extern "C-unwind" fn namespace_new(
    r#type: *mut _typeobject,
    args: *mut _object,
    kwds: *mut _object,
) -> *mut _object {
    let self_: *mut _object = unsafe { unsafe { (*r#type).tp_alloc }.unwrap()(r#type, 0i64) };
    if !self_.is_null() {
        let ns: *mut __SlateRecord_namespaceobject_c_13 =
            self_ as *mut __SlateRecord_namespaceobject_c_13;
        unsafe {
            (*ns).ns_dict = PyDict_New();
        }
        if unsafe { (*ns).ns_dict }.is_null() {
            Py_DECREF__2!(ns);
            return core::ptr::null_mut::<_object>();
        }
    }
    self_
}

#[unsafe(link_section = ".text.slate_distinct.namespaceobject.namespace_init")]
extern "C-unwind" fn namespace_init(
    op: *mut _object,
    args: *mut _object,
    kwds: *mut _object,
) -> i32 {
    let ns: *mut __SlateRecord_namespaceobject_c_13 = _PyNamespace_CAST!(op);
    let mut arg: *mut _object = core::ptr::null_mut::<_object>();
    if (unsafe {
        PyArg_UnpackTuple(
            args,
            _PyType_Name(Py_TYPE__2!(ns)),
            0i64,
            1i64,
            core::ptr::addr_of_mut!(arg),
        )
    }) == 0
    {
        return 1i32.wrapping_neg();
    }
    if !arg.is_null() {
        let mut dict: *mut _object = unsafe { core::mem::zeroed() };
        if PyDict_CheckExact!(arg) != 0 {
            dict = Py_NewRef!(arg);
        } else {
            dict = unsafe {
                PyObject_CallOneArg(core::ptr::addr_of_mut!(PyDict_Type) as *mut _object, arg)
            };
            if dict.is_null() {
                return 1i32.wrapping_neg();
            }
        }
        let mut err: i32 = 0;
        let __v0: bool = (unsafe { PyArg_ValidateKeywordArguments(dict) }) == 0
            || (unsafe { PyDict_Update((*ns).ns_dict, dict) }) < 0;
        err = __v0 as i32;
        Py_DECREF!(dict);
        if err != 0 {
            return 1i32.wrapping_neg();
        }
    }
    if kwds.is_null() {
        return 0;
    }
    if (unsafe { PyArg_ValidateKeywordArguments(kwds) }) == 0 {
        return 1i32.wrapping_neg();
    }
    unsafe { PyDict_Update((*ns).ns_dict, kwds) }
}

#[unsafe(link_section = ".text.slate_distinct.namespaceobject.namespace_dealloc")]
extern "C-unwind" fn namespace_dealloc(op: *mut _object) {
    let ns: *mut __SlateRecord_namespaceobject_c_13 = _PyNamespace_CAST!(op);
    unsafe { PyObject_GC_UnTrack(ns as *mut ()) };
    Py_CLEAR!((*ns).ns_dict);
    unsafe { unsafe { (*Py_TYPE__2!(ns)).tp_free }.unwrap()(ns as *mut ()) };
}

#[unsafe(link_section = ".text.slate_distinct.namespaceobject.namespace_repr")]
extern "C-unwind" fn namespace_repr(ns: *mut _object) -> *mut _object {
    let mut has_key: i32 = unsafe { core::mem::zeroed() };
    let mut item: *mut _object = unsafe { core::mem::zeroed() };
    let mut value: *mut _object = unsafe { core::mem::zeroed() };
    let mut __v4: bool = unsafe { core::mem::zeroed() };
    let mut __v5: *mut _object = unsafe { core::mem::zeroed() };
    let mut __v6: *const i8 = unsafe { core::mem::zeroed() };
    let mut name: *const i8 = unsafe { core::mem::zeroed() };
    let mut repr: *mut _object = unsafe { core::mem::zeroed() };
    let mut pairsrepr: *mut _object = unsafe { core::mem::zeroed() };
    let mut separator: *mut _object = unsafe { core::mem::zeroed() };
    let mut key: *mut _object = unsafe { core::mem::zeroed() };
    let mut keys_iter: *mut _object = unsafe { core::mem::zeroed() };
    let mut keys: *mut _object = unsafe { core::mem::zeroed() };
    let mut d: *mut _object = unsafe { core::mem::zeroed() };
    let mut pairs: *mut _object = unsafe { core::mem::zeroed() };
    let mut loop_error: i32 = unsafe { core::mem::zeroed() };
    let mut i: i32 = unsafe { core::mem::zeroed() };
    unsafe {
        loop_error = 0;
        pairs = core::ptr::null_mut::<_object>();
        d = core::ptr::null_mut::<_object>();
        keys = core::ptr::null_mut::<_object>();
        keys_iter = core::ptr::null_mut::<_object>();
        repr = core::ptr::null_mut::<_object>();
        if Py_IS_TYPE!(ns, core::ptr::addr_of_mut!(_PyNamespace_Type)) != 0 {
            __v6 = c"namespace".as_ptr();
        } else {
            __v6 = (*Py_TYPE!(ns)).tp_name;
        }
        name = __v6;
        i = Py_ReprEnter(ns);
        if i != 0 {
            if i > 0 {
                __v5 = PyUnicode_FromFormat(c"%s(...)".as_ptr(), name);
            } else {
                __v5 = core::ptr::null_mut::<_object>();
            }
            return __v5;
        } else {
            '__join_0: {
                pairs = PyList_New(0i64);
                if !pairs.is_null() {
                    d = Py_NewRef!(unsafe {
                        (*(ns as *mut __SlateRecord_namespaceobject_c_13)).ns_dict
                    });
                    keys = PyDict_Keys(d);
                    if !keys.is_null() {
                        keys_iter = PyObject_GetIter(keys);
                        if !keys_iter.is_null() {
                            loop {
                                let __v0: *mut _object = PyIter_Next(keys_iter);
                                key = __v0;
                                if !__v0.is_null() {
                                    if PyUnicode_Check!(key) != 0 {
                                        __v4 = PyUnicode_GET_LENGTH!(key) > 0;
                                    } else {
                                        __v4 = false;
                                    }
                                    if __v4 {
                                        has_key = PyDict_GetItemRef(
                                            d,
                                            key,
                                            core::ptr::addr_of_mut!(value),
                                        );
                                        if has_key == 1 {
                                            item =
                                                PyUnicode_FromFormat(c"%U=%R".as_ptr(), key, value);
                                            Py_DECREF!(value);
                                            if item.is_null() {
                                                loop_error = 1;
                                            } else {
                                                loop_error = PyList_Append(pairs, item);
                                                Py_DECREF!(item);
                                            }
                                        } else {
                                            if has_key < 0 {
                                                loop_error = 1;
                                            }
                                        }
                                    }
                                    Py_DECREF!(key);
                                    if loop_error != 0 {
                                        break '__join_0;
                                    }
                                } else {
                                    break;
                                }
                            }
                            if PyErr_Occurred().is_null() {
                                separator = PyUnicode_FromString(c", ".as_ptr());
                                if !separator.is_null() {
                                    pairsrepr = PyUnicode_Join(separator, pairs);
                                    Py_DECREF!(separator);
                                    if !pairsrepr.is_null() {
                                        repr = PyUnicode_FromFormat(
                                            c"%s(%S)".as_ptr(),
                                            name,
                                            pairsrepr,
                                        );
                                        Py_DECREF!(pairsrepr);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Py_XDECREF!(pairs);
            Py_XDECREF!(d);
            Py_XDECREF!(keys);
            Py_XDECREF!(keys_iter);
            Py_ReprLeave(ns);
            return repr;
        }
    }
}

#[unsafe(link_section = ".text.slate_distinct.namespaceobject.namespace_traverse")]
extern "C-unwind" fn namespace_traverse(
    op: *mut _object,
    mut visit: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
    arg: *mut (),
) -> i32 {
    let ns: *mut __SlateRecord_namespaceobject_c_13 = _PyNamespace_CAST!(op);
    Py_VISIT!(unsafe { (*ns).ns_dict }, visit, arg);
    0
}

#[unsafe(link_section = ".text.slate_distinct.namespaceobject.namespace_clear")]
extern "C-unwind" fn namespace_clear(op: *mut _object) -> i32 {
    let ns: *mut __SlateRecord_namespaceobject_c_13 = _PyNamespace_CAST!(op);
    Py_CLEAR!((*ns).ns_dict);
    0
}

#[unsafe(link_section = ".text.slate_distinct.namespaceobject.namespace_richcompare")]
extern "C-unwind" fn namespace_richcompare(
    self_: *mut _object,
    other: *mut _object,
    op: i32,
) -> *mut _object {
    let __v0: bool =
        PyObject_TypeCheck!(self_, unsafe { core::ptr::addr_of_mut!(_PyNamespace_Type) }) != 0
            && PyObject_TypeCheck!(other, unsafe { core::ptr::addr_of_mut!(_PyNamespace_Type) })
                != 0;
    if __v0 {
        return unsafe {
            PyObject_RichCompare(
                (*(self_ as *mut __SlateRecord_namespaceobject_c_13)).ns_dict,
                (*(other as *mut __SlateRecord_namespaceobject_c_13)).ns_dict,
                op,
            )
        };
    }
    unsafe { core::ptr::addr_of_mut!(_Py_NotImplementedStruct) }
}

static namespace_reduce__doc__: __SlateAlign16<[i8; 38]> = __SlateAlign16([
    82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 115i8, 116i8, 97i8, 116i8, 101i8, 32i8, 105i8,
    110i8, 102i8, 111i8, 114i8, 109i8, 97i8, 116i8, 105i8, 111i8, 110i8, 32i8, 102i8, 111i8, 114i8,
    32i8, 112i8, 105i8, 99i8, 107i8, 108i8, 105i8, 110i8, 103i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.namespaceobject.namespace_reduce")]
extern "C-unwind" fn namespace_reduce(
    op: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    let ns: *mut __SlateRecord_namespaceobject_c_13 = op as *mut __SlateRecord_namespaceobject_c_13;
    let args: *mut _object = unsafe { PyTuple_New(0i64) };
    if args.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let result: *mut _object =
        unsafe { PyTuple_Pack(3i64, Py_TYPE__2!(ns) as *mut _object, args, (*ns).ns_dict) };
    Py_DECREF!(args);
    result
}

#[unsafe(link_section = ".text.slate_distinct.namespaceobject.namespace_replace")]
extern "C-unwind" fn namespace_replace(
    self_: *mut _object,
    args: *mut _object,
    kwargs: *mut _object,
) -> *mut _object {
    if !_PyArg_NoPositional!(c"__replace__".as_ptr() as *mut i8, args) {
        return core::ptr::null_mut::<_object>();
    }
    let result: *mut _object = unsafe { PyObject_CallNoArgs(Py_TYPE!(self_) as *mut _object) };
    if result.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    if _PyNamespace_Check!(result) == 0 {
        unsafe {
            PyErr_Format(
                PyExc_TypeError,
                c"expect %N type, but %T() returned '%T' object".as_ptr(),
                core::ptr::addr_of_mut!(_PyNamespace_Type),
                self_,
                result,
            )
        };
        Py_DECREF!(result);
        return core::ptr::null_mut::<_object>();
    }
    if (unsafe {
        PyDict_Update(
            (*(result as *mut __SlateRecord_namespaceobject_c_13)).ns_dict,
            (*(self_ as *mut __SlateRecord_namespaceobject_c_13)).ns_dict,
        )
    }) < 0
    {
        Py_DECREF!(result);
        return core::ptr::null_mut::<_object>();
    }
    if !kwargs.is_null() {
        if (unsafe {
            PyDict_Update(
                (*(result as *mut __SlateRecord_namespaceobject_c_13)).ns_dict,
                kwargs,
            )
        }) < 0
        {
            Py_DECREF!(result);
            return core::ptr::null_mut::<_object>();
        }
    }
    result
}

/// sentinel
static mut namespace_methods: __SlateAlign16<[PyMethodDef; 3]> = __SlateAlign16([
    {
        let mut __t0: PyMethodDef = unsafe { core::mem::zeroed() };
        __t0.ml_name = (b"__reduce__\0".as_ptr() as *mut i8) as *const i8;
        __t0.ml_meth = Some(namespace_reduce);
        __t0.ml_flags = 4i32;
        __t0.ml_doc = unsafe { core::ptr::addr_of!(namespace_reduce__doc__.0) as *const i8 };
        __t0
    },
    {
        let mut __t1: PyMethodDef = unsafe { core::mem::zeroed() };
        __t1.ml_name = (b"__replace__\0".as_ptr() as *mut i8) as *const i8;
        __t1.ml_meth = unsafe {
            core::mem::transmute::<
                Option<unsafe extern "C-unwind" fn()>,
                Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
            >(unsafe {
                core::mem::transmute::<
                    Option<
                        unsafe extern "C-unwind" fn(
                            *mut _object,
                            *mut _object,
                            *mut _object,
                        ) -> *mut _object,
                    >,
                    Option<unsafe extern "C-unwind" fn()>,
                >(Some(namespace_replace))
            })
        };
        __t1.ml_flags = 3i32;
        __t1.ml_doc = (b"__replace__($self, /, **changes)\n--\n\nReturn a copy of the namespace object with new values for the specified attributes.\0".as_ptr() as *mut i8) as *const i8;
        __t1
    },
    {
        let mut __t2: PyMethodDef = unsafe { core::mem::zeroed() };
        __t2.ml_name = core::ptr::null::<i8>();
        __t2.ml_meth = None;
        __t2
    },
]);

static namespace_doc: __SlateAlign16<[i8; 93]> = __SlateAlign16([
    83i8, 105i8, 109i8, 112i8, 108i8, 101i8, 78i8, 97i8, 109i8, 101i8, 115i8, 112i8, 97i8, 99i8,
    101i8, 40i8, 109i8, 97i8, 112i8, 112i8, 105i8, 110i8, 103i8, 95i8, 111i8, 114i8, 95i8, 105i8,
    116i8, 101i8, 114i8, 97i8, 98i8, 108i8, 101i8, 61i8, 40i8, 41i8, 44i8, 32i8, 47i8, 44i8, 32i8,
    42i8, 42i8, 107i8, 119i8, 97i8, 114i8, 103i8, 115i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 65i8,
    32i8, 115i8, 105i8, 109i8, 112i8, 108i8, 101i8, 32i8, 97i8, 116i8, 116i8, 114i8, 105i8, 98i8,
    117i8, 116i8, 101i8, 45i8, 98i8, 97i8, 115i8, 101i8, 100i8, 32i8, 110i8, 97i8, 109i8, 101i8,
    115i8, 112i8, 97i8, 99i8, 101i8, 46i8, 0i8,
]);

// tp_name
// tp_basicsize
// tp_itemsize
// tp_dealloc
// tp_vectorcall_offset
// tp_getattr
// tp_setattr
// tp_as_async
// tp_repr
// tp_as_number
// tp_as_sequence
// tp_as_mapping
// tp_hash
// tp_call
// tp_str
// tp_getattro
// tp_setattro
// tp_as_buffer
// tp_flags
// tp_doc
// tp_traverse
// tp_clear
// tp_richcompare
// tp_weaklistoffset
// tp_iter
// tp_iternext
// tp_methods
// tp_members
// tp_getset
// tp_base
// tp_dict
// tp_descr_get
// tp_descr_set
// tp_dictoffset
// tp_init
// tp_alloc
// tp_new
// tp_free
#[unsafe(no_mangle)]
extern "C-unwind" fn _PyNamespace_New(kwds: *mut _object) -> *mut _object {
    let ns: *mut _object = namespace_new(
        unsafe { core::ptr::addr_of_mut!(_PyNamespace_Type) },
        core::ptr::null_mut::<_object>(),
        core::ptr::null_mut::<_object>(),
    );
    if ns.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    if kwds.is_null() {
        return ns;
    }
    if (unsafe {
        PyDict_Update(
            (*(ns as *mut __SlateRecord_namespaceobject_c_13)).ns_dict,
            kwds,
        )
    }) != 0
    {
        Py_DECREF!(ns);
        return core::ptr::null_mut::<_object>();
    }
    ns
}
