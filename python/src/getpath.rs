//! Return the initial module search path.
//! Version parsed out into numeric values
//!
//! --start constants--
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
//! PyException_HEAD defines the initial segment of every exception class.
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
macro_rules! PyCFunction_NewEx__2 {
    ($ML:expr) => {
        unsafe {
            PyCMethod_New(
                $ML,
                core::ptr::null_mut::<_object>(),
                core::ptr::null_mut::<_object>(),
                core::ptr::null_mut::<_typeobject>(),
            )
        }
    };
}

macro_rules! PyCode_Check {
    ($op:expr) => {
        Py_IS_TYPE!($op, unsafe { core::ptr::addr_of_mut!(PyCode_Type) })
    };
}

macro_rules! PyTuple_Check {
    ($op:expr) => {
        PyType_FastSubclass!(Py_TYPE!($op), 67108864u64)
    };
}

macro_rules! PyTuple_GET_ITEM {
    ($op:expr, $index:expr) => {
        unsafe {
            *((*_PyTuple_CAST!($op)).ob_item.as_mut_ptr() as *mut *mut _object)
                .wrapping_offset(($index) as isize)
        }
    };
}

macro_rules! PyTuple_GET_SIZE {
    ($op:expr) => {
        PyTuple_GET_SIZE(_PyObject_CAST!($op))
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

macro_rules! Py_ARRAY_LENGTH {
    () => {
        (core::mem::size_of::<[i32; 4097]>() as u64) / (core::mem::size_of::<i32>() as u64)
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

macro_rules! Py_SIZE {
    ($ob:expr) => {
        Py_SIZE(_PyObject_CAST__2!($ob))
    };
}

macro_rules! Py_TYPE {
    ($ob:expr) => {
        _Py_TYPE(_PyObject_CAST!($ob))
    };
}

macro_rules! Py_XDECREF {
    ($op:expr) => {
        Py_XDECREF(_PyObject_CAST!($op));
    };
}

macro_rules! Py_XINCREF {
    ($op:expr) => {
        Py_XINCREF(_PyObject_CAST!($op));
    };
}

macro_rules! Py_XNewRef {
    ($obj:expr) => {
        _Py_XNewRef(_PyObject_CAST!($obj))
    };
}

macro_rules! _PyASCIIObject_CAST {
    ($op:expr) => {
        _Py_CAST!(*mut __SlateRecord_unicodeobject_h_163, $op)
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

macro_rules! _PyStatus_EXCEPTION {
    ($err:expr) => {
        ($err._type as u32) != 0
    };
}

/// Cast argument to PyTupleObject* type.
macro_rules! _PyTuple_CAST {
    ($op:expr) => {
        _Py_CAST!(*mut __SlateRecord_tupleobject_h_13, $op)
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
    static mut _Py_NoneStruct: _object;
    static mut PyExc_OSError: *mut _object;
    static mut PyExc_MemoryError: *mut _object;
    static mut PyExc_SystemError: *mut _object;
    static mut PyExc_TypeError: *mut _object;
    static mut _Py_FalseStruct: _longobject;
    static mut _Py_TrueStruct: _longobject;
    static mut PyCode_Type: _typeobject;
    #[thread_local]
    static mut _Py_tss_tstate: *mut _ts;
    fn wcscpy(arg0: *mut i32, arg1: *const i32) -> *mut i32;
    fn wcscmp(arg0: *const i32, arg1: *const i32) -> i32;
    fn wcschr(arg0: *const i32, arg1: i32) -> *mut i32;
    fn wcsrchr(arg0: *const i32, arg1: i32) -> *mut i32;
    fn fread(arg0: *mut (), arg1: u64, arg2: u64, arg3: *mut slate_support::FILE) -> u64;
    fn PyMem_Malloc(arg0: u64) -> *mut ();
    fn PyMem_Free(arg0: *mut ());
    fn PyMem_RawFree(arg0: *mut ());
    fn Py_GetConstant(arg0: u32) -> *mut _object;
    fn _Py_Dealloc(arg0: *mut _object);
    fn PyUnicode_Substring(arg0: *mut _object, arg1: i64, arg2: i64) -> *mut _object;
    fn PyUnicode_FromWideChar(arg0: *const i32, arg1: i64) -> *mut _object;
    fn PyUnicode_AsWideCharString(arg0: *mut _object, arg1: *mut i64) -> *mut i32;
    fn PyUnicode_FindChar(arg0: *mut _object, arg1: u32, arg2: i64, arg3: i64, arg4: i32) -> i64;
    fn PyUnicode_AsUTF8(arg0: *mut _object) -> *const i8;
    fn PyErr_SetString(arg0: *mut _object, arg1: *const i8);
    fn PyErr_Clear();
    fn PyErr_NoMemory() -> *mut _object;
    fn PyErr_SetFromErrno(arg0: *mut _object) -> *mut _object;
    fn PyErr_FormatUnraisable(arg0: *const i8, ...);
    fn PyLong_FromLong(arg0: i64) -> *mut _object;
    fn PyList_New(arg0: i64) -> *mut _object;
    fn PyList_Append(arg0: *mut _object, arg1: *mut _object) -> i32;
    fn PyDict_New() -> *mut _object;
    fn PyDict_SetItemString(arg0: *mut _object, arg1: *const i8, arg2: *mut _object) -> i32;
    fn PyCMethod_New(
        arg0: *mut PyMethodDef,
        arg1: *mut _object,
        arg2: *mut _object,
        arg3: *mut _typeobject,
    ) -> *mut _object;
    fn PyStatus_Error(arg0: *const i8) -> __SlateRecord_initconfig_h_19;
    fn PyStatus_NoMemory() -> __SlateRecord_initconfig_h_19;
    fn PyArg_ParseTuple(arg0: *mut _object, arg1: *const i8, ...) -> i32;
    fn PyEval_EvalCode(arg0: *mut _object, arg1: *mut _object, arg2: *mut _object) -> *mut _object;
    fn PyEval_GetBuiltins() -> *mut _object;
    fn Py_DecodeLocale(arg0: *const i8, arg1: *mut u64) -> *mut i32;
    fn _PyMem_RawWcsdup(arg0: *const i32) -> *mut i32;
    fn _Py_wfopen(arg0: *const i32, arg1: *const i32) -> *mut slate_support::FILE;
    fn _Py_wreadlink(arg0: *const i32, arg1: *mut i32, arg2: u64) -> i32;
    fn _Py_DecodeUTF8_surrogateescape(arg0: *const i8, arg1: i64, arg2: *mut u64) -> *mut i32;
    fn _Py_wstat(arg0: *const i32, arg1: *mut stat) -> i32;
    fn _Py_isabs(arg0: *const i32) -> i32;
    fn _Py_abspath(arg0: *const i32, arg1: *mut *mut i32) -> i32;
    fn _Py_join_relfile(arg0: *const i32, arg1: *const i32) -> *mut i32;
    fn _Py_add_relfile(arg0: *mut i32, arg1: *const i32, arg2: u64) -> i32;
    fn _Py_normpath(arg0: *mut i32, arg1: i64) -> *mut i32;
    fn _PyConfig_AsDict(arg0: *const PyConfig) -> *mut _object;
    fn _PyConfig_FromDict(arg0: *mut PyConfig, arg1: *mut _object) -> i32;
    fn _PyPathConfig_ReadGlobal(arg0: *mut PyConfig) -> __SlateRecord_initconfig_h_19;
    fn _PyPathConfig_GetGlobalModuleSearchPath() -> *const i32;
    fn PyMarshal_ReadObjectFromString(arg0: *const i8, arg1: i64) -> *mut _object;
}

unsafe extern "C" {
    fn fflush(_0: *mut slate_support::FILE) -> i32;
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

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_object_h_172 {
    ob_base: _object,
    ob_size: i64,
}

fn _Py_TYPE(ob: *mut _object) -> *mut _typeobject {
    unsafe { (*ob).ob_type }
}

fn Py_SIZE(ob: *mut _object) -> i64 {
    unsafe { (*_PyVarObject_CAST!(ob)).ob_size }
}

fn Py_IS_TYPE(ob: *mut _object, r#type: *mut _typeobject) -> i32 {
    (Py_TYPE!(ob) == r#type) as i32
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _frame {}

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

fn PyType_HasFeature(r#type: *mut _typeobject, feature: u64) -> i32 {
    let flags: u64 = unsafe { (*r#type).tp_flags };
    (flags & feature != 0) as i32
}

#[inline(always)]
fn _Py_IsImmortal(op: *mut _object) -> i32 {
    (_Py_CAST!(i32, unsafe {
        (*op).__slate_anon_0.__slate_anon_1.ob_refcnt
    }) < 0) as i32
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

fn Py_XINCREF(op: *mut _object) {
    if !op.is_null() {
        Py_INCREF!(op);
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_unicodeobject_h_163_2 {
    __slate_bits_0: __slate_bits::__SlateBits124U0,
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

fn _Py_XNewRef(obj: *mut _object) -> *mut _object {
    Py_XINCREF!(obj);
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
struct _PyExecutorObject {}

#[repr(C)]
#[derive(Clone, Copy)]
struct _PyLongValue {
    lv_tag: u64,
    ob_digit: [u32; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _PyInterpreterFrame {}

#[repr(C)]
#[derive(Clone, Copy)]
struct _longobject {
    ob_base: _object,
    long_value: _PyLongValue,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_tupleobject_h_13 {
    ob_base: __SlateRecord_object_h_172,
    ob_hash: i64,
    ob_item: [*mut _object; 1],
}

fn PyTuple_GET_SIZE(op: *mut _object) -> i64 {
    let tuple: *mut __SlateRecord_tupleobject_h_13 = _PyTuple_CAST!(op);
    Py_SIZE!(tuple)
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
struct __SlateRecord_pystate_h_78 {
    __slate_bits_0: __slate_bits::__SlateBits166U0,
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
struct _err_stackitem {
    exc_value: *mut _object,
    previous_item: *mut _err_stackitem,
}

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
struct stat {
    st_dev: u64,
    st_ino: u64,
    st_nlink: u64,
    st_mode: u32,
    st_uid: u32,
    st_gid: u32,
    __pad0: i32,
    st_rdev: u64,
    st_size: i64,
    st_blksize: i64,
    st_blocks: i64,
    st_atim: slate_support::timespec,
    st_mtim: slate_support::timespec,
    st_ctim: slate_support::timespec,
    __glibc_reserved: [i64; 3],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_lock_h_69 {
    v: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct pyruntimestate {}

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
struct __SlateRecord_pycore_interp_structs_h_348 {
    import_level: i32,
    accumulated: i64,
    header: i32,
}

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
struct xi_exceptions {
    PyExc_InterpreterError: *mut _object,
    PyExc_InterpreterNotFoundError: *mut _object,
    PyExc_NotShareableError: *mut _object,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_hashtable_h_19 {
    head: *mut _Py_slist_item_s,
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
struct __SlateRecord_pycore_interp_structs_h_718 {
    _not_used: i32,
    _hamt_empty_gc_not_used: __SlateRecord_pycore_interp_structs_h_170,
    hamt_empty: __SlateRecord_pycore_structs_h_47,
    last_resort_memory_error: __SlateRecord_pyerrors_h_15,
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
    pub struct __SlateBits166U0 {
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
    pub struct __SlateBits124U0 {
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

use slate_support::abs;
use slate_support::getenv;
use slate_support::memset;
use slate_support::unsetenv;

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

#[unsafe(no_mangle)]
static _Py_M__getpath: __SlateAlign16<[u8; 14718]> = __SlateAlign16([
    99u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 9u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 243u8, 72u8, 29u8, 0u8, 0u8, 128u8, 0u8, 93u8, 0u8, 80u8, 3u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 82u8, 0u8, 52u8, 1u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 59u8, 1u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 103u8, 3u8,
    0u8, 0u8, 28u8, 0u8, 31u8, 0u8, 93u8, 2u8, 116u8, 3u8, 93u8, 4u8, 59u8, 1u8, 39u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 103u8, 3u8, 0u8, 0u8, 28u8, 0u8, 31u8, 0u8, 82u8, 1u8, 116u8, 4u8,
    93u8, 5u8, 82u8, 2u8, 56u8, 88u8, 0u8, 0u8, 103u8, 8u8, 0u8, 0u8, 28u8, 0u8, 93u8, 5u8, 82u8,
    3u8, 56u8, 88u8, 0u8, 0u8, 100u8, 65u8, 0u8, 0u8, 28u8, 0u8, 82u8, 4u8, 116u8, 6u8, 82u8, 5u8,
    93u8, 7u8, 12u8, 0u8, 50u8, 2u8, 116u8, 8u8, 93u8, 3u8, 12u8, 0u8, 82u8, 6u8, 93u8, 7u8, 12u8,
    0u8, 82u8, 7u8, 93u8, 9u8, 12u8, 0u8, 93u8, 4u8, 12u8, 0u8, 50u8, 6u8, 116u8, 10u8, 93u8, 10u8,
    12u8, 0u8, 82u8, 8u8, 50u8, 2u8, 93u8, 10u8, 12u8, 0u8, 82u8, 9u8, 50u8, 2u8, 46u8, 2u8, 116u8,
    11u8, 93u8, 3u8, 12u8, 0u8, 82u8, 6u8, 93u8, 7u8, 12u8, 0u8, 82u8, 7u8, 93u8, 9u8, 12u8, 0u8,
    93u8, 4u8, 12u8, 0u8, 82u8, 10u8, 50u8, 7u8, 116u8, 12u8, 82u8, 11u8, 46u8, 1u8, 116u8, 13u8,
    82u8, 12u8, 116u8, 14u8, 93u8, 3u8, 12u8, 0u8, 82u8, 6u8, 93u8, 7u8, 12u8, 0u8, 93u8, 9u8,
    12u8, 0u8, 93u8, 4u8, 12u8, 0u8, 82u8, 13u8, 50u8, 6u8, 116u8, 15u8, 82u8, 14u8, 116u8, 16u8,
    82u8, 15u8, 116u8, 17u8, 77u8, 61u8, 93u8, 5u8, 82u8, 16u8, 56u8, 88u8, 0u8, 0u8, 100u8, 55u8,
    0u8, 0u8, 28u8, 0u8, 82u8, 4u8, 116u8, 6u8, 82u8, 5u8, 116u8, 8u8, 82u8, 17u8, 116u8, 10u8,
    93u8, 10u8, 12u8, 0u8, 82u8, 18u8, 50u8, 2u8, 93u8, 10u8, 12u8, 0u8, 82u8, 19u8, 50u8, 2u8,
    46u8, 2u8, 116u8, 11u8, 93u8, 3u8, 12u8, 0u8, 116u8, 12u8, 82u8, 20u8, 46u8, 1u8, 116u8, 13u8,
    82u8, 12u8, 116u8, 14u8, 82u8, 5u8, 93u8, 7u8, 12u8, 0u8, 93u8, 9u8, 12u8, 0u8, 93u8, 18u8,
    59u8, 1u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 103u8, 3u8, 0u8, 0u8, 28u8, 0u8, 31u8, 0u8,
    82u8, 1u8, 12u8, 0u8, 82u8, 13u8, 50u8, 5u8, 116u8, 15u8, 82u8, 21u8, 93u8, 19u8, 12u8, 0u8,
    82u8, 22u8, 50u8, 3u8, 116u8, 20u8, 82u8, 23u8, 116u8, 16u8, 82u8, 24u8, 116u8, 17u8, 82u8,
    25u8, 93u8, 21u8, 47u8, 1u8, 82u8, 26u8, 23u8, 0u8, 108u8, 2u8, 116u8, 22u8, 93u8, 0u8, 80u8,
    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    82u8, 27u8, 52u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 116u8, 23u8, 93u8, 0u8, 80u8, 3u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 82u8,
    28u8, 52u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 116u8, 24u8, 93u8, 0u8, 80u8, 3u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 82u8, 29u8,
    52u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 116u8, 25u8, 93u8, 0u8, 80u8, 3u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 82u8, 30u8, 52u8, 1u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 116u8, 26u8, 93u8, 0u8, 80u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 82u8, 31u8, 52u8, 1u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 116u8, 27u8, 93u8, 0u8, 80u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 82u8, 32u8, 52u8, 1u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 116u8, 28u8, 93u8, 0u8, 80u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 82u8, 33u8, 52u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    116u8, 29u8, 93u8, 0u8, 80u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 82u8, 34u8, 52u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 116u8, 30u8,
    93u8, 0u8, 82u8, 35u8, 44u8, 26u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 116u8,
    31u8, 93u8, 0u8, 80u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 82u8, 36u8, 94u8, 1u8, 52u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 116u8,
    32u8, 93u8, 0u8, 80u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 82u8, 37u8, 52u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 116u8, 33u8,
    93u8, 0u8, 80u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 82u8, 38u8, 52u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 116u8, 34u8, 93u8,
    0u8, 80u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 82u8, 39u8, 52u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 116u8, 35u8, 93u8, 36u8, 33u8,
    0u8, 93u8, 35u8, 52u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 116u8, 37u8, 82u8, 40u8, 116u8, 38u8,
    82u8, 40u8, 116u8, 39u8, 93u8, 23u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 103u8, 27u8, 0u8,
    0u8, 28u8, 0u8, 27u8, 0u8, 93u8, 0u8, 80u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 82u8, 41u8, 46u8, 0u8, 52u8, 2u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 94u8, 0u8, 44u8, 26u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 116u8,
    23u8, 93u8, 23u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 103u8, 3u8, 0u8, 0u8, 28u8, 0u8,
    93u8, 8u8, 116u8, 23u8, 93u8, 41u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 100u8, 46u8, 0u8,
    0u8, 28u8, 0u8, 93u8, 42u8, 33u8, 0u8, 93u8, 23u8, 93u8, 41u8, 52u8, 2u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 103u8, 31u8, 0u8, 0u8, 28u8, 0u8, 93u8,
    43u8, 33u8, 0u8, 93u8, 23u8, 93u8, 41u8, 44u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 52u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    100u8, 10u8, 0u8, 0u8, 28u8, 0u8, 93u8, 23u8, 93u8, 41u8, 44u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 116u8, 23u8, 93u8, 44u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    100u8, 11u8, 0u8, 0u8, 28u8, 0u8, 93u8, 25u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 103u8,
    3u8, 0u8, 0u8, 28u8, 0u8, 93u8, 45u8, 116u8, 25u8, 93u8, 25u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 103u8, 16u8, 0u8, 0u8, 28u8, 0u8, 93u8, 17u8, 93u8, 23u8, 57u8, 0u8, 0u8, 0u8, 100u8,
    9u8, 0u8, 0u8, 28u8, 0u8, 93u8, 46u8, 33u8, 0u8, 93u8, 23u8, 52u8, 1u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 116u8, 25u8, 93u8, 25u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 103u8, 4u8, 0u8,
    0u8, 28u8, 0u8, 93u8, 45u8, 116u8, 25u8, 77u8, 9u8, 93u8, 5u8, 82u8, 3u8, 56u8, 88u8, 0u8, 0u8,
    100u8, 3u8, 0u8, 0u8, 28u8, 0u8, 93u8, 25u8, 116u8, 45u8, 93u8, 25u8, 39u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 103u8, 68u8, 0u8, 0u8, 28u8, 0u8, 93u8, 23u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 100u8, 60u8, 0u8, 0u8, 28u8, 0u8, 93u8, 47u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 100u8, 52u8, 0u8, 0u8, 28u8, 0u8, 93u8, 47u8, 80u8, 97u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 93u8, 16u8, 52u8, 1u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 16u8, 0u8, 70u8, 30u8, 0u8, 0u8, 116u8, 49u8, 93u8, 50u8, 33u8, 0u8, 93u8,
    49u8, 93u8, 23u8, 52u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 116u8, 49u8, 93u8, 43u8, 33u8, 0u8,
    93u8, 49u8, 52u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    103u8, 3u8, 0u8, 0u8, 28u8, 0u8, 75u8, 28u8, 0u8, 0u8, 93u8, 49u8, 116u8, 25u8, 31u8, 0u8,
    77u8, 2u8, 9u8, 0u8, 30u8, 0u8, 93u8, 25u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 103u8,
    13u8, 0u8, 0u8, 28u8, 0u8, 82u8, 1u8, 116u8, 25u8, 93u8, 46u8, 33u8, 0u8, 82u8, 7u8, 52u8, 1u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 116u8, 51u8, 93u8, 51u8, 116u8, 38u8, 93u8, 52u8, 39u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 103u8, 9u8, 0u8, 0u8, 28u8, 0u8, 93u8, 53u8, 39u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 100u8, 75u8, 0u8, 0u8, 28u8, 0u8, 93u8, 5u8, 82u8, 3u8, 56u8, 88u8, 0u8, 0u8,
    100u8, 26u8, 0u8, 0u8, 28u8, 0u8, 93u8, 54u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 100u8,
    18u8, 0u8, 0u8, 28u8, 0u8, 93u8, 55u8, 33u8, 0u8, 93u8, 56u8, 52u8, 1u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 12u8, 0u8, 82u8, 42u8, 93u8, 7u8, 12u8, 0u8, 82u8, 7u8, 93u8, 9u8, 12u8, 0u8, 50u8,
    5u8, 116u8, 26u8, 77u8, 12u8, 93u8, 45u8, 59u8, 1u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    103u8, 3u8, 0u8, 0u8, 28u8, 0u8, 31u8, 0u8, 93u8, 25u8, 116u8, 26u8, 93u8, 45u8, 39u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 103u8, 3u8, 0u8, 0u8, 28u8, 0u8, 93u8, 26u8, 116u8, 45u8, 93u8,
    52u8, 59u8, 1u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 103u8, 3u8, 0u8, 0u8, 28u8, 0u8,
    31u8, 0u8, 93u8, 53u8, 116u8, 25u8, 93u8, 55u8, 33u8, 0u8, 93u8, 25u8, 52u8, 1u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 116u8, 51u8, 82u8, 43u8, 116u8, 57u8, 93u8, 24u8, 39u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 100u8, 4u8, 0u8, 0u8, 28u8, 0u8, 82u8, 44u8, 116u8, 57u8, 77u8, 26u8, 93u8,
    32u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 100u8, 19u8, 0u8, 0u8, 28u8, 0u8, 93u8, 58u8,
    39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 100u8, 11u8, 0u8, 0u8, 28u8, 0u8, 93u8, 44u8, 39u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 103u8, 3u8, 0u8, 0u8, 28u8, 0u8, 93u8, 58u8, 116u8, 24u8,
    82u8, 40u8, 116u8, 59u8, 93u8, 44u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 69u8, 1u8, 103u8,
    43u8, 0u8, 0u8, 28u8, 0u8, 27u8, 0u8, 93u8, 51u8, 59u8, 1u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 103u8, 9u8, 0u8, 0u8, 28u8, 0u8, 31u8, 0u8, 93u8, 55u8, 33u8, 0u8, 93u8, 25u8, 52u8,
    1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 116u8, 60u8, 93u8, 55u8, 33u8, 0u8, 93u8, 60u8, 52u8, 1u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 116u8, 59u8, 27u8, 0u8, 93u8, 61u8, 33u8, 0u8, 93u8, 50u8, 33u8,
    0u8, 93u8, 59u8, 93u8, 14u8, 52u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 52u8, 1u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 116u8, 62u8, 93u8, 62u8, 16u8, 0u8, 70u8, 249u8, 0u8, 0u8, 116u8, 65u8, 93u8,
    65u8, 80u8, 133u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 82u8, 45u8, 52u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 119u8, 3u8, 0u8, 0u8,
    116u8, 67u8, 116u8, 68u8, 116u8, 69u8, 93u8, 68u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    103u8, 3u8, 0u8, 0u8, 28u8, 0u8, 75u8, 34u8, 0u8, 0u8, 93u8, 67u8, 80u8, 141u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 52u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 80u8, 143u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 52u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 82u8, 28u8, 56u8, 88u8,
    0u8, 0u8, 103u8, 3u8, 0u8, 0u8, 28u8, 0u8, 75u8, 71u8, 0u8, 0u8, 93u8, 24u8, 39u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 100u8, 3u8, 0u8, 0u8, 28u8, 0u8, 31u8, 0u8, 77u8, 172u8, 93u8, 69u8,
    80u8, 141u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 52u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 59u8, 1u8, 116u8, 51u8, 116u8, 38u8, 93u8,
    26u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 103u8, 143u8, 0u8, 0u8, 28u8, 0u8, 27u8, 0u8,
    93u8, 72u8, 33u8, 0u8, 93u8, 25u8, 52u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 116u8, 26u8, 93u8,
    26u8, 93u8, 25u8, 56u8, 88u8, 0u8, 0u8, 100u8, 3u8, 0u8, 0u8, 28u8, 0u8, 82u8, 1u8, 116u8,
    26u8, 93u8, 26u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 103u8, 117u8, 0u8, 0u8, 28u8, 0u8,
    93u8, 50u8, 33u8, 0u8, 93u8, 51u8, 93u8, 74u8, 33u8, 0u8, 93u8, 25u8, 52u8, 1u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 52u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 116u8, 26u8, 93u8, 5u8, 82u8, 16u8,
    56u8, 119u8, 0u8, 0u8, 100u8, 95u8, 0u8, 0u8, 28u8, 0u8, 93u8, 21u8, 33u8, 0u8, 93u8, 26u8,
    52u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 103u8, 81u8,
    0u8, 0u8, 28u8, 0u8, 93u8, 74u8, 33u8, 0u8, 93u8, 25u8, 52u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 116u8, 75u8, 93u8, 8u8, 82u8, 5u8, 93u8, 7u8, 12u8, 0u8, 82u8, 7u8, 93u8, 9u8, 12u8, 0u8,
    50u8, 4u8, 51u8, 2u8, 16u8, 0u8, 70u8, 58u8, 0u8, 0u8, 116u8, 76u8, 93u8, 76u8, 93u8, 41u8,
    39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 100u8, 3u8, 0u8, 0u8, 28u8, 0u8, 93u8, 41u8, 77u8,
    1u8, 82u8, 1u8, 44u8, 13u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 116u8, 76u8,
    93u8, 75u8, 93u8, 76u8, 56u8, 88u8, 0u8, 0u8, 100u8, 3u8, 0u8, 0u8, 28u8, 0u8, 75u8, 31u8, 0u8,
    0u8, 93u8, 50u8, 33u8, 0u8, 93u8, 51u8, 93u8, 76u8, 52u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    116u8, 76u8, 93u8, 21u8, 33u8, 0u8, 93u8, 76u8, 52u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 39u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 103u8, 3u8, 0u8, 0u8, 28u8, 0u8, 75u8, 56u8, 0u8, 0u8, 93u8,
    76u8, 116u8, 26u8, 31u8, 0u8, 77u8, 2u8, 9u8, 0u8, 30u8, 0u8, 31u8, 0u8, 77u8, 2u8, 9u8, 0u8,
    30u8, 0u8, 93u8, 26u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 103u8, 23u8, 0u8, 0u8, 28u8,
    0u8, 93u8, 25u8, 59u8, 1u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 103u8, 13u8, 0u8, 0u8,
    28u8, 0u8, 31u8, 0u8, 93u8, 45u8, 59u8, 1u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 103u8,
    3u8, 0u8, 0u8, 28u8, 0u8, 31u8, 0u8, 82u8, 1u8, 116u8, 26u8, 93u8, 45u8, 39u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 103u8, 3u8, 0u8, 0u8, 28u8, 0u8, 93u8, 26u8, 116u8, 45u8, 93u8, 45u8, 39u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 100u8, 10u8, 0u8, 0u8, 28u8, 0u8, 27u8, 0u8, 93u8, 72u8,
    33u8, 0u8, 93u8, 45u8, 52u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 116u8, 45u8, 93u8, 51u8, 39u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 103u8, 88u8, 0u8, 0u8, 28u8, 0u8, 93u8, 5u8, 82u8, 3u8,
    56u8, 88u8, 0u8, 0u8, 100u8, 81u8, 0u8, 0u8, 28u8, 0u8, 93u8, 56u8, 39u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 100u8, 73u8, 0u8, 0u8, 28u8, 0u8, 93u8, 55u8, 33u8, 0u8, 93u8, 56u8, 52u8, 1u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 116u8, 79u8, 93u8, 80u8, 59u8, 1u8, 81u8, 4u8, 74u8, 0u8, 100u8,
    28u8, 0u8, 0u8, 28u8, 0u8, 31u8, 0u8, 82u8, 47u8, 23u8, 0u8, 93u8, 11u8, 16u8, 0u8, 52u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 70u8, 12u8, 0u8, 0u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    103u8, 3u8, 0u8, 0u8, 28u8, 0u8, 75u8, 11u8, 0u8, 0u8, 30u8, 0u8, 82u8, 44u8, 77u8, 17u8, 9u8,
    0u8, 30u8, 0u8, 82u8, 43u8, 77u8, 13u8, 33u8, 0u8, 82u8, 47u8, 23u8, 0u8, 93u8, 11u8, 16u8,
    0u8, 52u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 52u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 39u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 100u8, 11u8, 0u8, 0u8, 28u8, 0u8, 93u8, 72u8, 33u8, 0u8,
    93u8, 79u8, 52u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 116u8, 51u8, 93u8, 51u8, 116u8, 38u8,
    93u8, 51u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 103u8, 19u8, 0u8, 0u8, 28u8, 0u8, 93u8,
    45u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 100u8, 11u8, 0u8, 0u8, 28u8, 0u8, 93u8, 55u8,
    33u8, 0u8, 93u8, 45u8, 52u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 59u8, 1u8, 116u8, 51u8, 116u8,
    38u8, 93u8, 38u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 103u8, 17u8, 0u8, 0u8, 28u8, 0u8,
    93u8, 45u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 100u8, 9u8, 0u8, 0u8, 28u8, 0u8, 93u8,
    55u8, 33u8, 0u8, 93u8, 45u8, 52u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 116u8, 38u8, 82u8, 40u8,
    116u8, 81u8, 82u8, 40u8, 116u8, 82u8, 93u8, 44u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    103u8, 132u8, 0u8, 0u8, 28u8, 0u8, 93u8, 57u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 103u8,
    124u8, 0u8, 0u8, 28u8, 0u8, 93u8, 56u8, 93u8, 25u8, 93u8, 45u8, 51u8, 3u8, 16u8, 0u8, 70u8,
    100u8, 0u8, 0u8, 116u8, 49u8, 93u8, 49u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 103u8, 3u8,
    0u8, 0u8, 28u8, 0u8, 75u8, 13u8, 0u8, 0u8, 93u8, 5u8, 82u8, 16u8, 56u8, 88u8, 0u8, 0u8, 100u8,
    55u8, 0u8, 0u8, 28u8, 0u8, 93u8, 42u8, 33u8, 0u8, 93u8, 49u8, 82u8, 48u8, 52u8, 2u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 103u8, 16u8, 0u8, 0u8, 28u8, 0u8,
    93u8, 42u8, 33u8, 0u8, 93u8, 49u8, 82u8, 49u8, 52u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 39u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 100u8, 25u8, 0u8, 0u8, 28u8, 0u8, 93u8, 49u8, 80u8, 167u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 82u8,
    7u8, 52u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 94u8, 0u8, 44u8, 26u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 116u8, 49u8, 93u8, 49u8, 82u8, 50u8, 44u8, 13u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 116u8, 49u8, 27u8, 0u8, 93u8, 61u8, 33u8, 0u8, 93u8, 49u8, 52u8,
    1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 116u8, 81u8, 93u8, 55u8, 33u8, 0u8, 93u8, 49u8, 52u8, 1u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 116u8, 82u8, 31u8, 0u8, 77u8, 2u8, 9u8, 0u8, 30u8, 0u8, 93u8,
    82u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 100u8, 7u8, 0u8, 0u8, 28u8, 0u8, 94u8, 0u8,
    116u8, 32u8, 93u8, 82u8, 116u8, 24u8, 46u8, 0u8, 116u8, 33u8, 82u8, 40u8, 116u8, 84u8, 93u8,
    57u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 103u8, 17u8, 0u8, 0u8, 28u8, 0u8, 93u8, 38u8,
    39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 100u8, 9u8, 0u8, 0u8, 28u8, 0u8, 93u8, 44u8, 39u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 100u8, 24u8, 0u8, 0u8, 28u8, 0u8, 93u8, 0u8, 80u8, 3u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 82u8,
    51u8, 94u8, 0u8, 52u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 94u8, 0u8, 56u8, 148u8, 0u8, 0u8,
    100u8, 128u8, 0u8, 0u8, 28u8, 0u8, 27u8, 0u8, 93u8, 50u8, 33u8, 0u8, 93u8, 38u8, 93u8, 61u8,
    33u8, 0u8, 93u8, 50u8, 33u8, 0u8, 93u8, 38u8, 93u8, 6u8, 52u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 52u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 94u8, 0u8, 44u8, 26u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 52u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 116u8, 39u8, 93u8, 50u8,
    33u8, 0u8, 93u8, 38u8, 93u8, 85u8, 52u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 116u8, 84u8, 93u8,
    84u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 100u8, 81u8, 0u8, 0u8, 28u8, 0u8, 93u8, 5u8,
    82u8, 16u8, 56u8, 88u8, 0u8, 0u8, 100u8, 4u8, 0u8, 0u8, 28u8, 0u8, 93u8, 84u8, 116u8, 86u8,
    77u8, 10u8, 93u8, 22u8, 33u8, 0u8, 93u8, 84u8, 46u8, 1u8, 93u8, 13u8, 79u8, 1u8, 53u8, 6u8,
    33u8, 0u8, 4u8, 0u8, 116u8, 86u8, 93u8, 37u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 103u8,
    28u8, 0u8, 0u8, 28u8, 0u8, 93u8, 86u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 100u8, 11u8,
    0u8, 0u8, 28u8, 0u8, 93u8, 50u8, 33u8, 0u8, 93u8, 86u8, 82u8, 17u8, 52u8, 2u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 116u8, 35u8, 77u8, 9u8, 93u8, 50u8, 33u8, 0u8, 93u8, 84u8, 82u8, 17u8, 52u8,
    2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 116u8, 35u8, 93u8, 27u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 103u8, 3u8, 0u8, 0u8, 28u8, 0u8, 93u8, 86u8, 116u8, 27u8, 93u8, 28u8, 39u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 103u8, 3u8, 0u8, 0u8, 28u8, 0u8, 93u8, 84u8, 116u8, 28u8, 94u8, 1u8, 93u8,
    0u8, 82u8, 51u8, 38u8, 0u8, 0u8, 0u8, 93u8, 44u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    100u8, 7u8, 0u8, 0u8, 28u8, 0u8, 82u8, 1u8, 59u8, 1u8, 116u8, 27u8, 116u8, 28u8, 69u8, 2u8,
    77u8, 180u8, 93u8, 24u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 100u8, 42u8, 0u8, 0u8, 28u8,
    0u8, 93u8, 24u8, 80u8, 133u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 93u8, 16u8, 52u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 119u8, 3u8, 0u8,
    0u8, 116u8, 27u8, 116u8, 87u8, 116u8, 28u8, 93u8, 87u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 103u8, 3u8, 0u8, 0u8, 28u8, 0u8, 93u8, 27u8, 116u8, 28u8, 93u8, 37u8, 39u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 103u8, 3u8, 0u8, 0u8, 28u8, 0u8, 82u8, 40u8, 116u8, 35u8, 93u8, 56u8, 39u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 100u8, 162u8, 0u8, 0u8, 28u8, 0u8, 93u8, 27u8, 39u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 103u8, 154u8, 0u8, 0u8, 28u8, 0u8, 93u8, 55u8, 33u8, 0u8, 93u8,
    56u8, 52u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 116u8, 79u8, 93u8, 15u8, 39u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 100u8, 41u8, 0u8, 0u8, 28u8, 0u8, 93u8, 5u8, 82u8, 16u8, 56u8, 88u8, 0u8,
    0u8, 100u8, 25u8, 0u8, 0u8, 28u8, 0u8, 93u8, 21u8, 33u8, 0u8, 93u8, 50u8, 33u8, 0u8, 93u8,
    79u8, 93u8, 15u8, 52u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 52u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 100u8, 3u8, 0u8, 0u8, 28u8, 0u8, 93u8, 79u8,
    116u8, 27u8, 77u8, 9u8, 93u8, 22u8, 33u8, 0u8, 93u8, 79u8, 93u8, 15u8, 52u8, 2u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 116u8, 27u8, 93u8, 10u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 100u8,
    90u8, 0u8, 0u8, 28u8, 0u8, 93u8, 11u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 100u8, 82u8,
    0u8, 0u8, 28u8, 0u8, 93u8, 27u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 103u8, 74u8, 0u8,
    0u8, 28u8, 0u8, 93u8, 80u8, 59u8, 1u8, 81u8, 4u8, 74u8, 0u8, 100u8, 28u8, 0u8, 0u8, 28u8, 0u8,
    31u8, 0u8, 82u8, 52u8, 23u8, 0u8, 93u8, 11u8, 16u8, 0u8, 52u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 70u8, 12u8, 0u8, 0u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 103u8, 3u8, 0u8, 0u8, 28u8,
    0u8, 75u8, 11u8, 0u8, 0u8, 30u8, 0u8, 82u8, 44u8, 77u8, 17u8, 9u8, 0u8, 30u8, 0u8, 82u8, 43u8,
    77u8, 13u8, 33u8, 0u8, 82u8, 52u8, 23u8, 0u8, 93u8, 11u8, 16u8, 0u8, 52u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 52u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 100u8, 20u8, 0u8, 0u8, 28u8, 0u8, 93u8, 79u8, 116u8, 27u8, 93u8, 37u8, 39u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 103u8, 10u8, 0u8, 0u8, 28u8, 0u8, 93u8, 50u8, 33u8, 0u8, 93u8, 27u8,
    93u8, 10u8, 52u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 116u8, 35u8, 93u8, 15u8, 39u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 100u8, 98u8, 0u8, 0u8, 28u8, 0u8, 93u8, 51u8, 39u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 100u8, 90u8, 0u8, 0u8, 28u8, 0u8, 93u8, 27u8, 39u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 103u8, 82u8, 0u8, 0u8, 28u8, 0u8, 93u8, 5u8, 82u8, 16u8, 56u8, 88u8, 0u8, 0u8,
    100u8, 25u8, 0u8, 0u8, 28u8, 0u8, 93u8, 21u8, 33u8, 0u8, 93u8, 50u8, 33u8, 0u8, 93u8, 51u8,
    93u8, 15u8, 52u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 52u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 100u8, 3u8, 0u8, 0u8, 28u8, 0u8, 93u8, 51u8, 116u8,
    27u8, 77u8, 9u8, 93u8, 22u8, 33u8, 0u8, 93u8, 51u8, 93u8, 15u8, 52u8, 2u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 116u8, 27u8, 93u8, 27u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 100u8, 34u8, 0u8,
    0u8, 28u8, 0u8, 93u8, 37u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 103u8, 26u8, 0u8, 0u8,
    28u8, 0u8, 93u8, 50u8, 33u8, 0u8, 93u8, 27u8, 93u8, 10u8, 52u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 116u8, 35u8, 93u8, 88u8, 33u8, 0u8, 93u8, 35u8, 52u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 103u8, 3u8, 0u8, 0u8, 28u8, 0u8, 82u8, 40u8, 116u8,
    35u8, 93u8, 10u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 100u8, 60u8, 0u8, 0u8, 28u8, 0u8,
    93u8, 11u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 100u8, 52u8, 0u8, 0u8, 28u8, 0u8, 93u8,
    51u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 100u8, 44u8, 0u8, 0u8, 28u8, 0u8, 93u8, 27u8,
    39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 103u8, 36u8, 0u8, 0u8, 28u8, 0u8, 93u8, 22u8, 33u8,
    0u8, 93u8, 51u8, 46u8, 1u8, 93u8, 11u8, 79u8, 1u8, 53u8, 6u8, 33u8, 0u8, 4u8, 0u8, 116u8, 27u8,
    93u8, 27u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 100u8, 18u8, 0u8, 0u8, 28u8, 0u8, 93u8,
    35u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 103u8, 10u8, 0u8, 0u8, 28u8, 0u8, 93u8, 50u8,
    33u8, 0u8, 93u8, 27u8, 93u8, 10u8, 52u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 116u8, 35u8, 93u8,
    89u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 100u8, 73u8, 0u8, 0u8, 28u8, 0u8, 93u8, 27u8,
    39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 103u8, 65u8, 0u8, 0u8, 28u8, 0u8, 93u8, 89u8, 116u8,
    27u8, 93u8, 80u8, 59u8, 1u8, 81u8, 4u8, 74u8, 0u8, 100u8, 28u8, 0u8, 0u8, 28u8, 0u8, 31u8, 0u8,
    82u8, 53u8, 23u8, 0u8, 93u8, 11u8, 16u8, 0u8, 52u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 70u8,
    12u8, 0u8, 0u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 103u8, 3u8, 0u8, 0u8, 28u8, 0u8, 75u8,
    11u8, 0u8, 0u8, 30u8, 0u8, 82u8, 44u8, 77u8, 17u8, 9u8, 0u8, 30u8, 0u8, 82u8, 43u8, 77u8, 13u8,
    33u8, 0u8, 82u8, 53u8, 23u8, 0u8, 93u8, 11u8, 16u8, 0u8, 52u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 52u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 103u8,
    9u8, 0u8, 0u8, 28u8, 0u8, 93u8, 78u8, 33u8, 0u8, 82u8, 54u8, 52u8, 1u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 31u8, 0u8, 93u8, 27u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 103u8, 17u8, 0u8,
    0u8, 28u8, 0u8, 93u8, 46u8, 33u8, 0u8, 82u8, 1u8, 52u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    116u8, 27u8, 93u8, 78u8, 33u8, 0u8, 82u8, 54u8, 52u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 31u8,
    0u8, 93u8, 12u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 100u8, 157u8, 0u8, 0u8, 28u8, 0u8,
    93u8, 28u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 103u8, 149u8, 0u8, 0u8, 28u8, 0u8, 93u8,
    5u8, 82u8, 16u8, 56u8, 88u8, 0u8, 0u8, 100u8, 3u8, 0u8, 0u8, 28u8, 0u8, 93u8, 27u8, 116u8,
    28u8, 93u8, 28u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 103u8, 32u8, 0u8, 0u8, 28u8, 0u8,
    93u8, 27u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 100u8, 24u8, 0u8, 0u8, 28u8, 0u8, 93u8,
    88u8, 33u8, 0u8, 93u8, 50u8, 33u8, 0u8, 93u8, 27u8, 93u8, 12u8, 52u8, 2u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 52u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    100u8, 3u8, 0u8, 0u8, 28u8, 0u8, 93u8, 27u8, 116u8, 28u8, 93u8, 28u8, 39u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 103u8, 20u8, 0u8, 0u8, 28u8, 0u8, 93u8, 51u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 100u8, 12u8, 0u8, 0u8, 28u8, 0u8, 93u8, 22u8, 33u8, 0u8, 93u8, 51u8, 93u8, 12u8,
    93u8, 88u8, 82u8, 55u8, 55u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 116u8, 28u8, 93u8, 28u8, 39u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 103u8, 11u8, 0u8, 0u8, 28u8, 0u8, 93u8, 90u8, 39u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 100u8, 3u8, 0u8, 0u8, 28u8, 0u8, 93u8, 90u8, 116u8, 28u8, 93u8,
    28u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 100u8, 22u8, 0u8, 0u8, 28u8, 0u8, 93u8, 88u8,
    33u8, 0u8, 93u8, 50u8, 33u8, 0u8, 93u8, 28u8, 93u8, 12u8, 52u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 52u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 103u8,
    27u8, 0u8, 0u8, 28u8, 0u8, 93u8, 5u8, 82u8, 16u8, 56u8, 88u8, 0u8, 0u8, 100u8, 12u8, 0u8, 0u8,
    28u8, 0u8, 93u8, 39u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 103u8, 3u8, 0u8, 0u8, 28u8,
    0u8, 93u8, 28u8, 116u8, 39u8, 77u8, 8u8, 93u8, 78u8, 33u8, 0u8, 82u8, 56u8, 52u8, 1u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 31u8, 0u8, 93u8, 28u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 103u8,
    3u8, 0u8, 0u8, 28u8, 0u8, 93u8, 27u8, 116u8, 28u8, 93u8, 27u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 100u8, 9u8, 0u8, 0u8, 28u8, 0u8, 93u8, 28u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    103u8, 9u8, 0u8, 0u8, 28u8, 0u8, 93u8, 78u8, 33u8, 0u8, 82u8, 57u8, 52u8, 1u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 31u8, 0u8, 93u8, 59u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 100u8, 25u8,
    0u8, 0u8, 28u8, 0u8, 93u8, 29u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 103u8, 3u8, 0u8, 0u8,
    28u8, 0u8, 93u8, 27u8, 116u8, 29u8, 93u8, 30u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 103u8,
    3u8, 0u8, 0u8, 28u8, 0u8, 93u8, 28u8, 116u8, 30u8, 93u8, 59u8, 59u8, 1u8, 116u8, 27u8, 116u8,
    28u8, 93u8, 29u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 103u8, 3u8, 0u8, 0u8, 28u8, 0u8,
    93u8, 27u8, 116u8, 29u8, 93u8, 30u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 103u8, 3u8, 0u8,
    0u8, 28u8, 0u8, 93u8, 28u8, 116u8, 30u8, 93u8, 44u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    100u8, 28u8, 0u8, 0u8, 28u8, 0u8, 93u8, 44u8, 80u8, 97u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 93u8, 16u8, 52u8, 1u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 93u8, 0u8, 82u8, 37u8, 38u8, 0u8, 0u8, 0u8, 94u8, 1u8, 93u8, 0u8, 82u8, 38u8,
    38u8, 0u8, 0u8, 0u8, 69u8, 2u8, 77u8, 94u8, 93u8, 34u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 69u8, 2u8, 103u8, 86u8, 0u8, 0u8, 28u8, 0u8, 46u8, 0u8, 116u8, 33u8, 93u8, 32u8, 39u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 100u8, 56u8, 0u8, 0u8, 28u8, 0u8, 93u8, 31u8, 39u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 100u8, 48u8, 0u8, 0u8, 28u8, 0u8, 93u8, 31u8, 80u8, 97u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 93u8,
    16u8, 52u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 16u8, 0u8, 70u8, 26u8, 0u8, 0u8, 116u8, 49u8,
    93u8, 33u8, 80u8, 183u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 93u8, 46u8, 33u8, 0u8, 93u8, 49u8, 52u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    52u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 31u8, 0u8, 75u8, 28u8, 0u8, 0u8, 9u8, 0u8, 30u8, 0u8,
    93u8, 5u8, 82u8, 16u8, 56u8, 88u8, 0u8, 0u8, 100u8, 45u8, 0u8, 0u8, 28u8, 0u8, 93u8, 56u8,
    39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 100u8, 10u8, 0u8, 0u8, 28u8, 0u8, 93u8, 55u8, 33u8,
    0u8, 93u8, 56u8, 52u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 116u8, 79u8, 77u8, 2u8, 93u8, 51u8,
    116u8, 79u8, 93u8, 33u8, 80u8, 183u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 93u8, 50u8, 33u8, 0u8, 93u8, 79u8, 93u8, 15u8, 52u8, 2u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 52u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 31u8, 0u8, 77u8, 57u8,
    93u8, 84u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 100u8, 26u8, 0u8, 0u8, 28u8, 0u8, 93u8,
    33u8, 80u8, 183u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 93u8, 50u8, 33u8, 0u8, 93u8, 89u8, 93u8, 15u8, 52u8, 2u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 52u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 31u8, 0u8, 77u8, 24u8, 93u8, 33u8, 80u8,
    183u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 93u8, 50u8, 33u8, 0u8, 93u8, 29u8, 93u8, 15u8, 52u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    52u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 31u8, 0u8, 93u8, 5u8, 82u8, 16u8, 56u8, 88u8, 0u8,
    0u8, 100u8, 162u8, 0u8, 0u8, 28u8, 0u8, 93u8, 32u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    100u8, 154u8, 0u8, 0u8, 28u8, 0u8, 93u8, 92u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 100u8,
    146u8, 0u8, 0u8, 28u8, 0u8, 93u8, 92u8, 80u8, 186u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 93u8, 92u8, 80u8, 188u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 51u8, 2u8, 16u8, 0u8,
    70u8, 117u8, 0u8, 0u8, 116u8, 95u8, 27u8, 0u8, 93u8, 92u8, 80u8, 193u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 93u8, 95u8, 93u8, 20u8,
    52u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 116u8, 67u8, 27u8, 0u8, 94u8, 0u8, 116u8, 97u8, 27u8,
    0u8, 27u8, 0u8, 93u8, 92u8, 80u8, 197u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 93u8, 67u8, 93u8, 92u8, 80u8, 199u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 93u8, 67u8, 93u8, 97u8,
    52u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 52u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 116u8, 100u8,
    93u8, 101u8, 33u8, 0u8, 93u8, 100u8, 93u8, 102u8, 52u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 100u8, 33u8, 0u8, 0u8, 28u8, 0u8, 93u8, 33u8, 80u8,
    207u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 93u8, 100u8, 80u8, 97u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 93u8, 16u8, 52u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 52u8, 1u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 31u8, 0u8, 93u8, 97u8, 94u8, 1u8, 44u8, 13u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 116u8, 97u8, 75u8, 94u8, 0u8, 0u8, 9u8, 0u8, 30u8, 0u8, 93u8, 105u8,
    39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 100u8, 49u8, 0u8, 0u8, 28u8, 0u8, 93u8, 105u8, 80u8,
    97u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    93u8, 16u8, 52u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 16u8, 0u8, 70u8, 27u8, 0u8, 0u8, 116u8,
    49u8, 93u8, 33u8, 80u8, 183u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 93u8, 50u8, 33u8, 0u8, 93u8, 29u8, 93u8, 49u8, 52u8, 2u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 52u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 31u8, 0u8, 75u8, 29u8, 0u8, 0u8,
    9u8, 0u8, 30u8, 0u8, 93u8, 35u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 103u8, 18u8, 0u8,
    0u8, 28u8, 0u8, 93u8, 29u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 100u8, 10u8, 0u8, 0u8,
    28u8, 0u8, 93u8, 50u8, 33u8, 0u8, 93u8, 29u8, 93u8, 10u8, 52u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 116u8, 35u8, 93u8, 39u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 103u8, 18u8, 0u8, 0u8,
    28u8, 0u8, 93u8, 30u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 100u8, 10u8, 0u8, 0u8, 28u8,
    0u8, 93u8, 50u8, 33u8, 0u8, 93u8, 30u8, 93u8, 12u8, 52u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    116u8, 39u8, 93u8, 5u8, 82u8, 16u8, 56u8, 88u8, 0u8, 0u8, 100u8, 84u8, 0u8, 0u8, 28u8, 0u8,
    93u8, 39u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 100u8, 18u8, 0u8, 0u8, 28u8, 0u8, 93u8,
    33u8, 80u8, 183u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 93u8, 39u8, 52u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 31u8, 0u8, 93u8, 35u8,
    39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 100u8, 18u8, 0u8, 0u8, 28u8, 0u8, 93u8, 33u8, 80u8,
    183u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 93u8, 35u8, 52u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 31u8, 0u8, 93u8, 51u8, 39u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 100u8, 25u8, 0u8, 0u8, 28u8, 0u8, 93u8, 51u8, 93u8, 33u8, 57u8,
    1u8, 0u8, 0u8, 100u8, 18u8, 0u8, 0u8, 28u8, 0u8, 93u8, 33u8, 80u8, 183u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 93u8, 51u8, 52u8, 1u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 31u8, 0u8, 77u8, 50u8, 93u8, 35u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 100u8, 18u8, 0u8, 0u8, 28u8, 0u8, 93u8, 33u8, 80u8, 183u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 93u8, 35u8, 52u8, 1u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 31u8, 0u8, 93u8, 39u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 100u8,
    18u8, 0u8, 0u8, 28u8, 0u8, 93u8, 33u8, 80u8, 183u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 93u8, 39u8, 52u8, 1u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 31u8, 0u8, 93u8, 33u8, 93u8, 0u8, 82u8, 37u8, 38u8, 0u8, 0u8, 0u8, 94u8, 1u8, 93u8,
    0u8, 82u8, 38u8, 38u8, 0u8, 0u8, 0u8, 93u8, 5u8, 82u8, 16u8, 56u8, 119u8, 0u8, 0u8, 100u8,
    145u8, 0u8, 0u8, 28u8, 0u8, 93u8, 84u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 100u8, 137u8,
    0u8, 0u8, 28u8, 0u8, 93u8, 59u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 103u8, 65u8, 0u8,
    0u8, 28u8, 0u8, 93u8, 0u8, 80u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 82u8, 31u8, 52u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 59u8,
    1u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 103u8, 3u8, 0u8, 0u8, 28u8, 0u8, 31u8, 0u8, 93u8,
    89u8, 116u8, 27u8, 93u8, 0u8, 80u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 82u8, 32u8, 52u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 59u8,
    1u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 103u8, 13u8, 0u8, 0u8, 28u8, 0u8, 31u8, 0u8,
    93u8, 90u8, 59u8, 1u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 103u8, 3u8, 0u8, 0u8, 28u8,
    0u8, 31u8, 0u8, 93u8, 27u8, 116u8, 28u8, 93u8, 0u8, 80u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 82u8, 33u8, 52u8, 1u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 59u8, 1u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 103u8, 3u8, 0u8, 0u8,
    28u8, 0u8, 31u8, 0u8, 93u8, 89u8, 116u8, 29u8, 93u8, 0u8, 80u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 82u8, 34u8, 52u8, 1u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 59u8, 1u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 103u8, 13u8, 0u8,
    0u8, 28u8, 0u8, 31u8, 0u8, 93u8, 90u8, 59u8, 1u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    103u8, 3u8, 0u8, 0u8, 28u8, 0u8, 31u8, 0u8, 93u8, 29u8, 116u8, 30u8, 93u8, 81u8, 39u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 100u8, 161u8, 0u8, 0u8, 28u8, 0u8, 94u8, 1u8, 93u8, 0u8, 82u8,
    58u8, 38u8, 0u8, 0u8, 0u8, 94u8, 0u8, 93u8, 0u8, 82u8, 36u8, 38u8, 0u8, 0u8, 0u8, 94u8, 0u8,
    93u8, 0u8, 82u8, 59u8, 38u8, 0u8, 0u8, 0u8, 94u8, 1u8, 93u8, 0u8, 82u8, 60u8, 38u8, 0u8, 0u8,
    0u8, 46u8, 0u8, 116u8, 33u8, 93u8, 81u8, 16u8, 0u8, 70u8, 122u8, 0u8, 0u8, 116u8, 65u8, 93u8,
    65u8, 80u8, 133u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 82u8, 61u8, 52u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 94u8, 0u8, 44u8, 26u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 80u8, 141u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 52u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    116u8, 65u8, 93u8, 65u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 103u8, 3u8, 0u8, 0u8, 28u8,
    0u8, 75u8, 51u8, 0u8, 0u8, 93u8, 65u8, 82u8, 62u8, 56u8, 88u8, 0u8, 0u8, 100u8, 8u8, 0u8, 0u8,
    28u8, 0u8, 94u8, 1u8, 93u8, 0u8, 82u8, 59u8, 38u8, 0u8, 0u8, 0u8, 75u8, 65u8, 0u8, 0u8, 93u8,
    65u8, 80u8, 213u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 82u8, 63u8, 52u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 39u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 100u8, 11u8, 0u8, 0u8, 28u8, 0u8, 93u8, 78u8, 33u8, 0u8, 82u8, 64u8, 52u8, 1u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 31u8, 0u8, 75u8, 98u8, 0u8, 0u8, 93u8, 33u8, 80u8, 183u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 93u8,
    50u8, 33u8, 0u8, 93u8, 82u8, 93u8, 65u8, 52u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 52u8, 1u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 31u8, 0u8, 75u8, 124u8, 0u8, 0u8, 9u8, 0u8, 30u8, 0u8, 93u8,
    33u8, 93u8, 0u8, 82u8, 37u8, 38u8, 0u8, 0u8, 0u8, 94u8, 1u8, 93u8, 0u8, 82u8, 38u8, 38u8, 0u8,
    0u8, 0u8, 93u8, 23u8, 93u8, 0u8, 82u8, 27u8, 38u8, 0u8, 0u8, 0u8, 93u8, 24u8, 93u8, 0u8, 82u8,
    28u8, 38u8, 0u8, 0u8, 0u8, 93u8, 25u8, 93u8, 0u8, 82u8, 29u8, 38u8, 0u8, 0u8, 0u8, 93u8, 26u8,
    93u8, 0u8, 82u8, 30u8, 38u8, 0u8, 0u8, 0u8, 93u8, 27u8, 93u8, 0u8, 82u8, 31u8, 38u8, 0u8, 0u8,
    0u8, 93u8, 28u8, 93u8, 0u8, 82u8, 32u8, 38u8, 0u8, 0u8, 0u8, 93u8, 29u8, 93u8, 0u8, 82u8, 33u8,
    38u8, 0u8, 0u8, 0u8, 93u8, 30u8, 93u8, 0u8, 82u8, 34u8, 38u8, 0u8, 0u8, 0u8, 93u8, 3u8, 93u8,
    0u8, 82u8, 0u8, 38u8, 0u8, 0u8, 0u8, 93u8, 35u8, 59u8, 1u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 103u8, 3u8, 0u8, 0u8, 28u8, 0u8, 31u8, 0u8, 82u8, 1u8, 93u8, 0u8, 82u8, 39u8, 38u8, 0u8,
    0u8, 0u8, 93u8, 39u8, 59u8, 1u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 103u8, 3u8, 0u8, 0u8,
    28u8, 0u8, 31u8, 0u8, 82u8, 1u8, 93u8, 0u8, 82u8, 65u8, 38u8, 0u8, 0u8, 0u8, 82u8, 40u8, 35u8,
    0u8, 32u8, 0u8, 93u8, 40u8, 6u8, 0u8, 100u8, 5u8, 0u8, 0u8, 28u8, 0u8, 31u8, 0u8, 29u8, 0u8,
    69u8, 11u8, 76u8, 167u8, 105u8, 0u8, 59u8, 3u8, 29u8, 0u8, 105u8, 1u8, 32u8, 0u8, 93u8, 63u8,
    93u8, 64u8, 51u8, 2u8, 6u8, 0u8, 100u8, 22u8, 0u8, 0u8, 28u8, 0u8, 31u8, 0u8, 93u8, 61u8, 33u8,
    0u8, 93u8, 50u8, 33u8, 0u8, 93u8, 60u8, 93u8, 14u8, 52u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    52u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 116u8, 62u8, 93u8, 60u8, 116u8, 59u8, 29u8, 0u8, 69u8,
    10u8, 76u8, 54u8, 105u8, 0u8, 59u8, 3u8, 29u8, 0u8, 105u8, 1u8, 32u8, 0u8, 93u8, 63u8, 93u8,
    64u8, 51u8, 2u8, 6u8, 0u8, 100u8, 9u8, 0u8, 0u8, 28u8, 0u8, 31u8, 0u8, 82u8, 40u8, 116u8, 59u8,
    46u8, 0u8, 116u8, 62u8, 29u8, 0u8, 69u8, 10u8, 76u8, 74u8, 105u8, 0u8, 59u8, 3u8, 29u8, 0u8,
    105u8, 1u8, 32u8, 0u8, 93u8, 73u8, 6u8, 0u8, 100u8, 5u8, 0u8, 0u8, 28u8, 0u8, 31u8, 0u8, 29u8,
    0u8, 69u8, 9u8, 76u8, 217u8, 105u8, 0u8, 59u8, 3u8, 29u8, 0u8, 105u8, 1u8, 32u8, 0u8, 93u8,
    73u8, 6u8, 0u8, 100u8, 37u8, 0u8, 0u8, 28u8, 0u8, 116u8, 77u8, 93u8, 21u8, 33u8, 0u8, 93u8,
    45u8, 52u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 100u8,
    12u8, 0u8, 0u8, 28u8, 0u8, 93u8, 78u8, 33u8, 0u8, 82u8, 46u8, 93u8, 45u8, 12u8, 0u8, 50u8, 2u8,
    52u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 31u8, 0u8, 29u8, 0u8, 82u8, 40u8, 116u8, 77u8, 65u8,
    77u8, 69u8, 9u8, 76u8, 74u8, 82u8, 40u8, 116u8, 77u8, 65u8, 77u8, 105u8, 1u8, 105u8, 0u8, 59u8,
    3u8, 29u8, 0u8, 105u8, 1u8, 32u8, 0u8, 93u8, 73u8, 6u8, 0u8, 100u8, 6u8, 0u8, 0u8, 28u8, 0u8,
    31u8, 0u8, 29u8, 0u8, 69u8, 8u8, 75u8, 179u8, 0u8, 0u8, 105u8, 0u8, 59u8, 3u8, 29u8, 0u8,
    105u8, 1u8, 32u8, 0u8, 93u8, 40u8, 6u8, 0u8, 100u8, 16u8, 0u8, 0u8, 28u8, 0u8, 31u8, 0u8, 93u8,
    38u8, 116u8, 39u8, 93u8, 50u8, 33u8, 0u8, 93u8, 38u8, 93u8, 85u8, 52u8, 2u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 116u8, 84u8, 29u8, 0u8, 69u8, 7u8, 76u8, 254u8, 93u8, 63u8, 93u8, 64u8, 51u8,
    2u8, 6u8, 0u8, 100u8, 5u8, 0u8, 0u8, 28u8, 0u8, 31u8, 0u8, 29u8, 0u8, 69u8, 8u8, 76u8, 9u8,
    105u8, 0u8, 59u8, 3u8, 29u8, 0u8, 105u8, 1u8, 32u8, 0u8, 93u8, 73u8, 6u8, 0u8, 100u8, 4u8, 0u8,
    0u8, 28u8, 0u8, 31u8, 0u8, 29u8, 0u8, 77u8, 4u8, 105u8, 0u8, 59u8, 3u8, 29u8, 0u8, 105u8, 1u8,
    93u8, 24u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 103u8, 74u8, 0u8, 0u8, 28u8, 0u8, 93u8,
    35u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 103u8, 66u8, 0u8, 0u8, 28u8, 0u8, 93u8, 92u8,
    80u8, 197u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 93u8, 67u8, 82u8, 40u8, 52u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 116u8, 100u8, 93u8,
    101u8, 33u8, 0u8, 93u8, 100u8, 93u8, 102u8, 52u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 39u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 100u8, 33u8, 0u8, 0u8, 28u8, 0u8, 93u8, 33u8, 80u8, 207u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 93u8,
    100u8, 80u8, 97u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 93u8, 16u8, 52u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 52u8, 1u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 31u8, 0u8, 93u8, 92u8, 80u8, 209u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 93u8, 67u8, 52u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 31u8, 0u8, 69u8, 4u8, 75u8, 41u8, 0u8, 0u8, 32u8, 0u8, 93u8, 92u8, 80u8, 209u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 93u8, 67u8,
    52u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 31u8, 0u8, 105u8, 0u8, 59u8, 3u8, 29u8, 0u8, 105u8,
    1u8, 32u8, 0u8, 93u8, 73u8, 6u8, 0u8, 100u8, 6u8, 0u8, 0u8, 28u8, 0u8, 31u8, 0u8, 29u8, 0u8,
    69u8, 4u8, 75u8, 74u8, 0u8, 0u8, 105u8, 0u8, 59u8, 3u8, 29u8, 0u8, 105u8, 1u8, 41u8, 66u8,
    218u8, 10u8, 112u8, 108u8, 97u8, 116u8, 108u8, 105u8, 98u8, 100u8, 105u8, 114u8, 218u8, 0u8,
    218u8, 5u8, 112u8, 111u8, 115u8, 105u8, 120u8, 218u8, 6u8, 100u8, 97u8, 114u8, 119u8, 105u8,
    110u8, 122u8, 14u8, 112u8, 121u8, 98u8, 117u8, 105u8, 108u8, 100u8, 100u8, 105u8, 114u8, 46u8,
    116u8, 120u8, 116u8, 218u8, 6u8, 112u8, 121u8, 116u8, 104u8, 111u8, 110u8, 122u8, 7u8, 47u8,
    112u8, 121u8, 116u8, 104u8, 111u8, 110u8, 218u8, 1u8, 46u8, 122u8, 6u8, 47u8, 111u8, 115u8,
    46u8, 112u8, 121u8, 122u8, 7u8, 47u8, 111u8, 115u8, 46u8, 112u8, 121u8, 99u8, 122u8, 12u8,
    47u8, 108u8, 105u8, 98u8, 45u8, 100u8, 121u8, 110u8, 108u8, 111u8, 97u8, 100u8, 122u8, 9u8,
    76u8, 105u8, 98u8, 47u8, 111u8, 115u8, 46u8, 112u8, 121u8, 122u8, 10u8, 112u8, 121u8, 118u8,
    101u8, 110u8, 118u8, 46u8, 99u8, 102u8, 103u8, 122u8, 4u8, 46u8, 122u8, 105u8, 112u8, 218u8,
    1u8, 58u8, 218u8, 1u8, 47u8, 218u8, 2u8, 110u8, 116u8, 218u8, 3u8, 76u8, 105u8, 98u8, 122u8,
    6u8, 92u8, 111u8, 115u8, 46u8, 112u8, 121u8, 122u8, 7u8, 92u8, 111u8, 115u8, 46u8, 112u8,
    121u8, 99u8, 122u8, 9u8, 76u8, 105u8, 98u8, 92u8, 111u8, 115u8, 46u8, 112u8, 121u8, 122u8,
    27u8, 83u8, 79u8, 70u8, 84u8, 87u8, 65u8, 82u8, 69u8, 92u8, 80u8, 121u8, 116u8, 104u8, 111u8,
    110u8, 92u8, 80u8, 121u8, 116u8, 104u8, 111u8, 110u8, 67u8, 111u8, 114u8, 101u8, 92u8, 122u8,
    11u8, 92u8, 80u8, 121u8, 116u8, 104u8, 111u8, 110u8, 80u8, 97u8, 116u8, 104u8, 218u8, 1u8,
    59u8, 218u8, 1u8, 92u8, 218u8, 4u8, 116u8, 101u8, 115u8, 116u8, 99u8, 1u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8, 7u8, 0u8, 0u8, 0u8, 243u8, 188u8, 0u8,
    0u8, 0u8, 97u8, 0u8, 97u8, 1u8, 128u8, 0u8, 83u8, 0u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    100u8, 82u8, 0u8, 0u8, 28u8, 0u8, 92u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 59u8, 1u8,
    81u8, 4u8, 74u8, 0u8, 100u8, 32u8, 0u8, 0u8, 28u8, 0u8, 31u8, 0u8, 86u8, 0u8, 86u8, 1u8, 51u8,
    2u8, 82u8, 0u8, 23u8, 0u8, 108u8, 8u8, 86u8, 2u8, 16u8, 0u8, 52u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 70u8, 12u8, 0u8, 0u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 103u8, 3u8, 0u8, 0u8,
    28u8, 0u8, 75u8, 11u8, 0u8, 0u8, 30u8, 0u8, 82u8, 1u8, 77u8, 21u8, 9u8, 0u8, 30u8, 0u8, 82u8,
    2u8, 77u8, 17u8, 33u8, 0u8, 86u8, 0u8, 86u8, 1u8, 51u8, 2u8, 82u8, 0u8, 23u8, 0u8, 108u8, 8u8,
    86u8, 2u8, 16u8, 0u8, 52u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 52u8, 1u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 39u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 100u8, 3u8, 0u8, 0u8, 28u8, 0u8, 83u8, 0u8,
    35u8, 0u8, 92u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 83u8, 0u8, 52u8, 1u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 111u8, 0u8, 75u8, 89u8, 0u8, 0u8, 82u8, 3u8, 35u8, 0u8, 41u8, 4u8, 99u8,
    1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 7u8, 0u8, 0u8, 0u8, 51u8, 0u8, 0u8,
    0u8, 243u8, 72u8, 0u8, 0u8, 0u8, 60u8, 2u8, 34u8, 0u8, 31u8, 0u8, 128u8, 0u8, 84u8, 0u8, 70u8,
    23u8, 0u8, 0u8, 112u8, 1u8, 83u8, 3u8, 33u8, 0u8, 92u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 83u8, 2u8, 86u8, 1u8, 52u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 52u8, 1u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 120u8, 0u8, 128u8, 5u8, 31u8, 0u8, 75u8, 25u8, 0u8, 0u8, 9u8, 0u8, 30u8, 0u8,
    82u8, 0u8, 35u8, 0u8, 53u8, 3u8, 105u8, 1u8, 169u8, 1u8, 78u8, 41u8, 1u8, 218u8, 8u8, 106u8,
    111u8, 105u8, 110u8, 112u8, 97u8, 116u8, 104u8, 41u8, 4u8, 218u8, 2u8, 46u8, 48u8, 218u8, 1u8,
    102u8, 218u8, 6u8, 112u8, 114u8, 101u8, 102u8, 105u8, 120u8, 114u8, 13u8, 0u8, 0u8, 0u8, 115u8,
    4u8, 0u8, 0u8, 0u8, 38u8, 32u8, 128u8, 128u8, 218u8, 16u8, 60u8, 102u8, 114u8, 111u8, 122u8,
    101u8, 110u8, 32u8, 103u8, 101u8, 116u8, 112u8, 97u8, 116u8, 104u8, 62u8, 218u8, 9u8, 60u8,
    103u8, 101u8, 110u8, 101u8, 120u8, 112u8, 114u8, 62u8, 218u8, 28u8, 115u8, 101u8, 97u8, 114u8,
    99u8, 104u8, 95u8, 117u8, 112u8, 46u8, 60u8, 108u8, 111u8, 99u8, 97u8, 108u8, 115u8, 62u8,
    46u8, 60u8, 103u8, 101u8, 110u8, 101u8, 120u8, 112u8, 114u8, 62u8, 209u8, 0u8, 0u8, 0u8, 115u8,
    32u8, 0u8, 0u8, 0u8, 248u8, 233u8, 0u8, 128u8, 0u8, 208u8, 14u8, 60u8, 177u8, 41u8, 168u8,
    81u8, 137u8, 116u8, 148u8, 72u8, 152u8, 86u8, 160u8, 81u8, 211u8, 20u8, 39u8, 215u8, 15u8,
    40u8, 208u8, 15u8, 40u8, 179u8, 41u8, 249u8, 115u8, 4u8, 0u8, 0u8, 0u8, 131u8, 31u8, 34u8, 1u8,
    84u8, 70u8, 78u8, 41u8, 2u8, 218u8, 3u8, 97u8, 110u8, 121u8, 218u8, 7u8, 100u8, 105u8, 114u8,
    110u8, 97u8, 109u8, 101u8, 41u8, 3u8, 114u8, 20u8, 0u8, 0u8, 0u8, 114u8, 13u8, 0u8, 0u8, 0u8,
    218u8, 9u8, 108u8, 97u8, 110u8, 100u8, 109u8, 97u8, 114u8, 107u8, 115u8, 115u8, 3u8, 0u8, 0u8,
    0u8, 102u8, 100u8, 42u8, 114u8, 21u8, 0u8, 0u8, 0u8, 218u8, 9u8, 115u8, 101u8, 97u8, 114u8,
    99u8, 104u8, 95u8, 117u8, 112u8, 114u8, 27u8, 0u8, 0u8, 0u8, 207u8, 0u8, 0u8, 0u8, 115u8, 58u8,
    0u8, 0u8, 0u8, 249u8, 128u8, 0u8, 223u8, 10u8, 16u8, 223u8, 11u8, 14u8, 139u8, 51u8, 213u8,
    14u8, 60u8, 177u8, 41u8, 211u8, 14u8, 60u8, 143u8, 51u8, 143u8, 51u8, 138u8, 51u8, 213u8, 14u8,
    60u8, 177u8, 41u8, 211u8, 14u8, 60u8, 215u8, 11u8, 60u8, 210u8, 11u8, 60u8, 216u8, 19u8, 25u8,
    136u8, 77u8, 220u8, 17u8, 24u8, 152u8, 22u8, 147u8, 31u8, 138u8, 6u8, 241u8, 7u8, 0u8, 11u8,
    17u8, 243u8, 0u8, 0u8, 0u8, 0u8, 218u8, 12u8, 112u8, 114u8, 111u8, 103u8, 114u8, 97u8, 109u8,
    95u8, 110u8, 97u8, 109u8, 101u8, 218u8, 4u8, 104u8, 111u8, 109u8, 101u8, 218u8, 10u8, 101u8,
    120u8, 101u8, 99u8, 117u8, 116u8, 97u8, 98u8, 108u8, 101u8, 218u8, 15u8, 98u8, 97u8, 115u8,
    101u8, 95u8, 101u8, 120u8, 101u8, 99u8, 117u8, 116u8, 97u8, 98u8, 108u8, 101u8, 114u8, 20u8,
    0u8, 0u8, 0u8, 218u8, 11u8, 101u8, 120u8, 101u8, 99u8, 95u8, 112u8, 114u8, 101u8, 102u8, 105u8,
    120u8, 218u8, 11u8, 98u8, 97u8, 115u8, 101u8, 95u8, 112u8, 114u8, 101u8, 102u8, 105u8, 120u8,
    218u8, 16u8, 98u8, 97u8, 115u8, 101u8, 95u8, 101u8, 120u8, 101u8, 99u8, 95u8, 112u8, 114u8,
    101u8, 102u8, 105u8, 120u8, 218u8, 14u8, 112u8, 121u8, 116u8, 104u8, 111u8, 110u8, 112u8, 97u8,
    116u8, 104u8, 95u8, 101u8, 110u8, 118u8, 218u8, 15u8, 117u8, 115u8, 101u8, 95u8, 101u8, 110u8,
    118u8, 105u8, 114u8, 111u8, 110u8, 109u8, 101u8, 110u8, 116u8, 218u8, 19u8, 109u8, 111u8,
    100u8, 117u8, 108u8, 101u8, 95u8, 115u8, 101u8, 97u8, 114u8, 99u8, 104u8, 95u8, 112u8, 97u8,
    116u8, 104u8, 115u8, 218u8, 23u8, 109u8, 111u8, 100u8, 117u8, 108u8, 101u8, 95u8, 115u8, 101u8,
    97u8, 114u8, 99u8, 104u8, 95u8, 112u8, 97u8, 116u8, 104u8, 115u8, 95u8, 115u8, 101u8, 116u8,
    218u8, 10u8, 115u8, 116u8, 100u8, 108u8, 105u8, 98u8, 95u8, 100u8, 105u8, 114u8, 78u8, 218u8,
    9u8, 111u8, 114u8, 105u8, 103u8, 95u8, 97u8, 114u8, 103u8, 118u8, 122u8, 11u8, 47u8, 98u8,
    105u8, 110u8, 47u8, 112u8, 121u8, 116u8, 104u8, 111u8, 110u8, 70u8, 84u8, 218u8, 1u8, 61u8,
    122u8, 32u8, 70u8, 97u8, 105u8, 108u8, 101u8, 100u8, 32u8, 116u8, 111u8, 32u8, 102u8, 105u8,
    110u8, 100u8, 32u8, 114u8, 101u8, 97u8, 108u8, 32u8, 108u8, 111u8, 99u8, 97u8, 116u8, 105u8,
    111u8, 110u8, 32u8, 111u8, 102u8, 32u8, 99u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 7u8, 0u8, 0u8, 0u8, 35u8, 0u8, 0u8, 0u8, 243u8, 84u8, 0u8, 0u8, 0u8, 34u8, 0u8, 31u8,
    0u8, 128u8, 0u8, 84u8, 0u8, 70u8, 30u8, 0u8, 0u8, 112u8, 1u8, 92u8, 1u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 92u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 92u8, 4u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 86u8, 1u8, 52u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 52u8, 1u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 120u8, 0u8, 128u8, 5u8, 31u8, 0u8, 75u8, 32u8, 0u8, 0u8, 9u8,
    0u8, 30u8, 0u8, 82u8, 0u8, 35u8, 0u8, 53u8, 3u8, 105u8, 1u8, 114u8, 16u8, 0u8, 0u8, 0u8, 169u8,
    3u8, 218u8, 6u8, 105u8, 115u8, 102u8, 105u8, 108u8, 101u8, 114u8, 17u8, 0u8, 0u8, 0u8, 218u8,
    11u8, 108u8, 105u8, 98u8, 114u8, 97u8, 114u8, 121u8, 95u8, 100u8, 105u8, 114u8, 41u8, 2u8,
    114u8, 18u8, 0u8, 0u8, 0u8, 218u8, 1u8, 112u8, 115u8, 2u8, 0u8, 0u8, 0u8, 38u8, 32u8, 114u8,
    21u8, 0u8, 0u8, 0u8, 114u8, 22u8, 0u8, 0u8, 0u8, 114u8, 22u8, 0u8, 0u8, 0u8, 181u8, 1u8, 0u8,
    0u8, 115u8, 33u8, 0u8, 0u8, 0u8, 233u8, 0u8, 128u8, 0u8, 208u8, 10u8, 70u8, 209u8, 53u8, 69u8,
    176u8, 1u8, 140u8, 54u8, 148u8, 40u8, 156u8, 59u8, 168u8, 1u8, 211u8, 18u8, 42u8, 215u8, 11u8,
    43u8, 208u8, 11u8, 43u8, 211u8, 53u8, 69u8, 249u8, 243u8, 4u8, 0u8, 0u8, 0u8, 130u8, 38u8,
    40u8, 1u8, 218u8, 3u8, 101u8, 120u8, 101u8, 218u8, 3u8, 100u8, 108u8, 108u8, 122u8, 5u8, 46u8,
    95u8, 112u8, 116u8, 104u8, 218u8, 16u8, 95u8, 105u8, 115u8, 95u8, 112u8, 121u8, 116u8, 104u8,
    111u8, 110u8, 95u8, 98u8, 117u8, 105u8, 108u8, 100u8, 99u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 7u8, 0u8, 0u8, 0u8, 35u8, 0u8, 0u8, 0u8, 243u8, 84u8, 0u8, 0u8, 0u8,
    34u8, 0u8, 31u8, 0u8, 128u8, 0u8, 84u8, 0u8, 70u8, 30u8, 0u8, 0u8, 112u8, 1u8, 92u8, 1u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 92u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 92u8,
    4u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 86u8, 1u8, 52u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 52u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 120u8, 0u8, 128u8, 5u8, 31u8, 0u8, 75u8, 32u8,
    0u8, 0u8, 9u8, 0u8, 30u8, 0u8, 82u8, 0u8, 35u8, 0u8, 53u8, 3u8, 105u8, 1u8, 114u8, 16u8, 0u8,
    0u8, 0u8, 114u8, 44u8, 0u8, 0u8, 0u8, 169u8, 2u8, 114u8, 18u8, 0u8, 0u8, 0u8, 114u8, 19u8, 0u8,
    0u8, 0u8, 115u8, 2u8, 0u8, 0u8, 0u8, 38u8, 32u8, 114u8, 21u8, 0u8, 0u8, 0u8, 114u8, 22u8, 0u8,
    0u8, 0u8, 114u8, 22u8, 0u8, 0u8, 0u8, 63u8, 2u8, 0u8, 0u8, 115u8, 33u8, 0u8, 0u8, 0u8, 233u8,
    0u8, 128u8, 0u8, 208u8, 18u8, 78u8, 209u8, 61u8, 77u8, 184u8, 1u8, 148u8, 54u8, 156u8, 40u8,
    164u8, 59u8, 176u8, 1u8, 211u8, 26u8, 50u8, 215u8, 19u8, 51u8, 208u8, 19u8, 51u8, 211u8, 61u8,
    77u8, 249u8, 114u8, 48u8, 0u8, 0u8, 0u8, 99u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 7u8, 0u8, 0u8, 0u8, 35u8, 0u8, 0u8, 0u8, 243u8, 84u8, 0u8, 0u8, 0u8, 34u8, 0u8,
    31u8, 0u8, 128u8, 0u8, 84u8, 0u8, 70u8, 30u8, 0u8, 0u8, 112u8, 1u8, 92u8, 1u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 92u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 92u8, 4u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 86u8, 1u8, 52u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 52u8,
    1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 120u8, 0u8, 128u8, 5u8, 31u8, 0u8, 75u8, 32u8, 0u8, 0u8,
    9u8, 0u8, 30u8, 0u8, 82u8, 0u8, 35u8, 0u8, 53u8, 3u8, 105u8, 1u8, 114u8, 16u8, 0u8, 0u8, 0u8,
    41u8, 3u8, 114u8, 45u8, 0u8, 0u8, 0u8, 114u8, 17u8, 0u8, 0u8, 0u8, 114u8, 20u8, 0u8, 0u8, 0u8,
    114u8, 53u8, 0u8, 0u8, 0u8, 115u8, 2u8, 0u8, 0u8, 0u8, 38u8, 32u8, 114u8, 21u8, 0u8, 0u8, 0u8,
    114u8, 22u8, 0u8, 0u8, 0u8, 114u8, 22u8, 0u8, 0u8, 0u8, 91u8, 2u8, 0u8, 0u8, 115u8, 33u8, 0u8,
    0u8, 0u8, 233u8, 0u8, 128u8, 0u8, 208u8, 18u8, 73u8, 209u8, 56u8, 72u8, 176u8, 49u8, 148u8,
    54u8, 156u8, 40u8, 164u8, 54u8, 168u8, 49u8, 211u8, 26u8, 45u8, 215u8, 19u8, 46u8, 208u8, 19u8,
    46u8, 211u8, 56u8, 72u8, 249u8, 114u8, 48u8, 0u8, 0u8, 0u8, 122u8, 54u8, 67u8, 111u8, 117u8,
    108u8, 100u8, 32u8, 110u8, 111u8, 116u8, 32u8, 102u8, 105u8, 110u8, 100u8, 32u8, 112u8, 108u8,
    97u8, 116u8, 102u8, 111u8, 114u8, 109u8, 32u8, 105u8, 110u8, 100u8, 101u8, 112u8, 101u8, 110u8,
    100u8, 101u8, 110u8, 116u8, 32u8, 108u8, 105u8, 98u8, 114u8, 97u8, 114u8, 105u8, 101u8, 115u8,
    32u8, 60u8, 112u8, 114u8, 101u8, 102u8, 105u8, 120u8, 62u8, 41u8, 1u8, 114u8, 13u8, 0u8, 0u8,
    0u8, 122u8, 57u8, 67u8, 111u8, 117u8, 108u8, 100u8, 32u8, 110u8, 111u8, 116u8, 32u8, 102u8,
    105u8, 110u8, 100u8, 32u8, 112u8, 108u8, 97u8, 116u8, 102u8, 111u8, 114u8, 109u8, 32u8, 100u8,
    101u8, 112u8, 101u8, 110u8, 100u8, 101u8, 110u8, 116u8, 32u8, 108u8, 105u8, 98u8, 114u8, 97u8,
    114u8, 105u8, 101u8, 115u8, 32u8, 60u8, 101u8, 120u8, 101u8, 99u8, 95u8, 112u8, 114u8, 101u8,
    102u8, 105u8, 120u8, 62u8, 122u8, 56u8, 67u8, 111u8, 110u8, 115u8, 105u8, 100u8, 101u8, 114u8,
    32u8, 115u8, 101u8, 116u8, 116u8, 105u8, 110u8, 103u8, 32u8, 36u8, 80u8, 89u8, 84u8, 72u8,
    79u8, 78u8, 72u8, 79u8, 77u8, 69u8, 32u8, 116u8, 111u8, 32u8, 60u8, 112u8, 114u8, 101u8, 102u8,
    105u8, 120u8, 62u8, 91u8, 58u8, 60u8, 101u8, 120u8, 101u8, 99u8, 95u8, 112u8, 114u8, 101u8,
    102u8, 105u8, 120u8, 62u8, 93u8, 218u8, 8u8, 105u8, 115u8, 111u8, 108u8, 97u8, 116u8, 101u8,
    100u8, 218u8, 11u8, 115u8, 105u8, 116u8, 101u8, 95u8, 105u8, 109u8, 112u8, 111u8, 114u8, 116u8,
    218u8, 9u8, 115u8, 97u8, 102u8, 101u8, 95u8, 112u8, 97u8, 116u8, 104u8, 218u8, 1u8, 35u8,
    122u8, 11u8, 105u8, 109u8, 112u8, 111u8, 114u8, 116u8, 32u8, 115u8, 105u8, 116u8, 101u8, 122u8,
    7u8, 105u8, 109u8, 112u8, 111u8, 114u8, 116u8, 32u8, 122u8, 39u8, 117u8, 110u8, 115u8, 117u8,
    112u8, 112u8, 111u8, 114u8, 116u8, 101u8, 100u8, 32u8, 39u8, 105u8, 109u8, 112u8, 111u8, 114u8,
    116u8, 39u8, 32u8, 108u8, 105u8, 110u8, 101u8, 32u8, 105u8, 110u8, 32u8, 46u8, 95u8, 112u8,
    116u8, 104u8, 32u8, 102u8, 105u8, 108u8, 101u8, 218u8, 14u8, 112u8, 108u8, 97u8, 116u8, 115u8,
    116u8, 100u8, 108u8, 105u8, 98u8, 95u8, 100u8, 105u8, 114u8, 41u8, 107u8, 218u8, 6u8, 99u8,
    111u8, 110u8, 102u8, 105u8, 103u8, 218u8, 3u8, 103u8, 101u8, 116u8, 218u8, 10u8, 80u8, 76u8,
    65u8, 84u8, 76u8, 73u8, 66u8, 68u8, 73u8, 82u8, 114u8, 1u8, 0u8, 0u8, 0u8, 218u8, 10u8, 65u8,
    66u8, 73u8, 95u8, 84u8, 72u8, 82u8, 69u8, 65u8, 68u8, 218u8, 7u8, 111u8, 115u8, 95u8, 110u8,
    97u8, 109u8, 101u8, 218u8, 12u8, 66u8, 85u8, 73u8, 76u8, 68u8, 68u8, 73u8, 82u8, 95u8, 84u8,
    88u8, 84u8, 218u8, 13u8, 86u8, 69u8, 82u8, 83u8, 73u8, 79u8, 78u8, 95u8, 77u8, 65u8, 74u8,
    79u8, 82u8, 218u8, 20u8, 68u8, 69u8, 70u8, 65u8, 85u8, 76u8, 84u8, 95u8, 80u8, 82u8, 79u8,
    71u8, 82u8, 65u8, 77u8, 95u8, 78u8, 65u8, 77u8, 69u8, 218u8, 13u8, 86u8, 69u8, 82u8, 83u8,
    73u8, 79u8, 78u8, 95u8, 77u8, 73u8, 78u8, 79u8, 82u8, 218u8, 13u8, 83u8, 84u8, 68u8, 76u8,
    73u8, 66u8, 95u8, 83u8, 85u8, 66u8, 68u8, 73u8, 82u8, 218u8, 16u8, 83u8, 84u8, 68u8, 76u8,
    73u8, 66u8, 95u8, 76u8, 65u8, 78u8, 68u8, 77u8, 65u8, 82u8, 75u8, 83u8, 218u8, 19u8, 80u8,
    76u8, 65u8, 84u8, 83u8, 84u8, 68u8, 76u8, 73u8, 66u8, 95u8, 76u8, 65u8, 78u8, 68u8, 77u8, 65u8,
    82u8, 75u8, 218u8, 21u8, 66u8, 85u8, 73u8, 76u8, 68u8, 83u8, 84u8, 68u8, 76u8, 73u8, 66u8,
    95u8, 76u8, 65u8, 78u8, 68u8, 77u8, 65u8, 82u8, 75u8, 83u8, 218u8, 13u8, 86u8, 69u8, 78u8,
    86u8, 95u8, 76u8, 65u8, 78u8, 68u8, 77u8, 65u8, 82u8, 75u8, 218u8, 12u8, 90u8, 73u8, 80u8,
    95u8, 76u8, 65u8, 78u8, 68u8, 77u8, 65u8, 82u8, 75u8, 218u8, 5u8, 68u8, 69u8, 76u8, 73u8, 77u8,
    218u8, 3u8, 83u8, 69u8, 80u8, 218u8, 10u8, 80u8, 89u8, 68u8, 69u8, 66u8, 85u8, 71u8, 69u8,
    88u8, 84u8, 218u8, 8u8, 80u8, 89u8, 87u8, 73u8, 78u8, 86u8, 69u8, 82u8, 218u8, 10u8, 87u8,
    73u8, 78u8, 82u8, 69u8, 71u8, 95u8, 75u8, 69u8, 89u8, 114u8, 45u8, 0u8, 0u8, 0u8, 114u8, 27u8,
    0u8, 0u8, 0u8, 114u8, 29u8, 0u8, 0u8, 0u8, 114u8, 30u8, 0u8, 0u8, 0u8, 114u8, 31u8, 0u8, 0u8,
    0u8, 114u8, 32u8, 0u8, 0u8, 0u8, 114u8, 20u8, 0u8, 0u8, 0u8, 114u8, 33u8, 0u8, 0u8, 0u8, 114u8,
    34u8, 0u8, 0u8, 0u8, 114u8, 35u8, 0u8, 0u8, 0u8, 218u8, 14u8, 69u8, 78u8, 86u8, 95u8, 80u8,
    89u8, 84u8, 72u8, 79u8, 78u8, 80u8, 65u8, 84u8, 72u8, 114u8, 37u8, 0u8, 0u8, 0u8, 218u8, 10u8,
    112u8, 121u8, 116u8, 104u8, 111u8, 110u8, 112u8, 97u8, 116u8, 104u8, 218u8, 18u8, 112u8, 121u8,
    116u8, 104u8, 111u8, 110u8, 112u8, 97u8, 116u8, 104u8, 95u8, 119u8, 97u8, 115u8, 95u8, 115u8,
    101u8, 116u8, 114u8, 40u8, 0u8, 0u8, 0u8, 218u8, 4u8, 98u8, 111u8, 111u8, 108u8, 218u8, 28u8,
    115u8, 116u8, 100u8, 108u8, 105u8, 98u8, 95u8, 100u8, 105u8, 114u8, 95u8, 119u8, 97u8, 115u8,
    95u8, 115u8, 101u8, 116u8, 95u8, 105u8, 110u8, 95u8, 99u8, 111u8, 110u8, 102u8, 105u8, 103u8,
    218u8, 19u8, 114u8, 101u8, 97u8, 108u8, 95u8, 101u8, 120u8, 101u8, 99u8, 117u8, 116u8, 97u8,
    98u8, 108u8, 101u8, 95u8, 100u8, 105u8, 114u8, 114u8, 59u8, 0u8, 0u8, 0u8, 218u8, 10u8, 73u8,
    110u8, 100u8, 101u8, 120u8, 69u8, 114u8, 114u8, 111u8, 114u8, 218u8, 10u8, 69u8, 88u8, 69u8,
    95u8, 83u8, 85u8, 70u8, 70u8, 73u8, 88u8, 218u8, 9u8, 104u8, 97u8, 115u8, 115u8, 117u8, 102u8,
    102u8, 105u8, 120u8, 218u8, 7u8, 105u8, 115u8, 120u8, 102u8, 105u8, 108u8, 101u8, 218u8, 10u8,
    112u8, 121u8, 95u8, 115u8, 101u8, 116u8, 112u8, 97u8, 116u8, 104u8, 218u8, 15u8, 114u8, 101u8,
    97u8, 108u8, 95u8, 101u8, 120u8, 101u8, 99u8, 117u8, 116u8, 97u8, 98u8, 108u8, 101u8, 218u8,
    7u8, 97u8, 98u8, 115u8, 112u8, 97u8, 116u8, 104u8, 218u8, 8u8, 69u8, 78u8, 86u8, 95u8, 80u8,
    65u8, 84u8, 72u8, 218u8, 5u8, 115u8, 112u8, 108u8, 105u8, 116u8, 114u8, 47u8, 0u8, 0u8, 0u8,
    114u8, 17u8, 0u8, 0u8, 0u8, 218u8, 14u8, 101u8, 120u8, 101u8, 99u8, 117u8, 116u8, 97u8, 98u8,
    108u8, 101u8, 95u8, 100u8, 105u8, 114u8, 218u8, 20u8, 69u8, 78u8, 86u8, 95u8, 80u8, 89u8, 84u8,
    72u8, 79u8, 78u8, 69u8, 88u8, 69u8, 67u8, 85u8, 84u8, 65u8, 66u8, 76u8, 69u8, 218u8, 23u8,
    69u8, 78u8, 86u8, 95u8, 95u8, 95u8, 80u8, 89u8, 86u8, 69u8, 78u8, 86u8, 95u8, 76u8, 65u8, 85u8,
    78u8, 67u8, 72u8, 69u8, 82u8, 95u8, 95u8, 218u8, 19u8, 87u8, 73u8, 84u8, 72u8, 95u8, 78u8,
    69u8, 88u8, 84u8, 95u8, 70u8, 82u8, 65u8, 77u8, 69u8, 87u8, 79u8, 82u8, 75u8, 114u8, 25u8, 0u8,
    0u8, 0u8, 218u8, 7u8, 108u8, 105u8, 98u8, 114u8, 97u8, 114u8, 121u8, 218u8, 12u8, 104u8, 111u8,
    109u8, 101u8, 95u8, 119u8, 97u8, 115u8, 95u8, 115u8, 101u8, 116u8, 218u8, 14u8, 69u8, 78u8,
    86u8, 95u8, 80u8, 89u8, 84u8, 72u8, 79u8, 78u8, 72u8, 79u8, 77u8, 69u8, 218u8, 11u8, 118u8,
    101u8, 110u8, 118u8, 95u8, 112u8, 114u8, 101u8, 102u8, 105u8, 120u8, 218u8, 12u8, 118u8, 101u8,
    110u8, 118u8, 95u8, 112u8, 114u8, 101u8, 102u8, 105u8, 120u8, 50u8, 218u8, 9u8, 114u8, 101u8,
    97u8, 100u8, 108u8, 105u8, 110u8, 101u8, 115u8, 218u8, 9u8, 112u8, 121u8, 118u8, 101u8, 110u8,
    118u8, 99u8, 102u8, 103u8, 218u8, 17u8, 70u8, 105u8, 108u8, 101u8, 78u8, 111u8, 116u8, 70u8,
    111u8, 117u8, 110u8, 100u8, 69u8, 114u8, 114u8, 111u8, 114u8, 218u8, 15u8, 80u8, 101u8, 114u8,
    109u8, 105u8, 115u8, 115u8, 105u8, 111u8, 110u8, 69u8, 114u8, 114u8, 111u8, 114u8, 218u8, 4u8,
    108u8, 105u8, 110u8, 101u8, 218u8, 9u8, 112u8, 97u8, 114u8, 116u8, 105u8, 116u8, 105u8, 111u8,
    110u8, 218u8, 3u8, 107u8, 101u8, 121u8, 218u8, 7u8, 104u8, 97u8, 100u8, 95u8, 101u8, 113u8,
    117u8, 218u8, 5u8, 118u8, 97u8, 108u8, 117u8, 101u8, 218u8, 5u8, 115u8, 116u8, 114u8, 105u8,
    112u8, 218u8, 5u8, 108u8, 111u8, 119u8, 101u8, 114u8, 218u8, 8u8, 114u8, 101u8, 97u8, 108u8,
    112u8, 97u8, 116u8, 104u8, 218u8, 7u8, 79u8, 83u8, 69u8, 114u8, 114u8, 111u8, 114u8, 218u8,
    8u8, 98u8, 97u8, 115u8, 101u8, 110u8, 97u8, 109u8, 101u8, 218u8, 8u8, 98u8, 97u8, 115u8, 101u8,
    95u8, 101u8, 120u8, 101u8, 218u8, 9u8, 99u8, 97u8, 110u8, 100u8, 105u8, 100u8, 97u8, 116u8,
    101u8, 218u8, 2u8, 101u8, 120u8, 218u8, 4u8, 119u8, 97u8, 114u8, 110u8, 114u8, 46u8, 0u8, 0u8,
    0u8, 114u8, 24u8, 0u8, 0u8, 0u8, 218u8, 3u8, 112u8, 116u8, 104u8, 218u8, 7u8, 112u8, 116u8,
    104u8, 95u8, 100u8, 105u8, 114u8, 218u8, 10u8, 114u8, 112u8, 97u8, 114u8, 116u8, 105u8, 116u8,
    105u8, 111u8, 110u8, 218u8, 12u8, 98u8, 117u8, 105u8, 108u8, 100u8, 95u8, 112u8, 114u8, 101u8,
    102u8, 105u8, 120u8, 218u8, 5u8, 86u8, 80u8, 65u8, 84u8, 72u8, 218u8, 19u8, 98u8, 117u8, 105u8,
    108u8, 100u8, 95u8, 115u8, 116u8, 100u8, 108u8, 105u8, 98u8, 95u8, 112u8, 114u8, 101u8, 102u8,
    105u8, 120u8, 218u8, 9u8, 104u8, 97u8, 100u8, 95u8, 100u8, 101u8, 108u8, 105u8, 109u8, 218u8,
    5u8, 105u8, 115u8, 100u8, 105u8, 114u8, 218u8, 6u8, 80u8, 82u8, 69u8, 70u8, 73u8, 88u8, 218u8,
    11u8, 69u8, 88u8, 69u8, 67u8, 95u8, 80u8, 82u8, 69u8, 70u8, 73u8, 88u8, 218u8, 6u8, 97u8,
    112u8, 112u8, 101u8, 110u8, 100u8, 218u8, 6u8, 119u8, 105u8, 110u8, 114u8, 101u8, 103u8, 218u8,
    17u8, 72u8, 75u8, 69u8, 89u8, 95u8, 67u8, 85u8, 82u8, 82u8, 69u8, 78u8, 84u8, 95u8, 85u8, 83u8,
    69u8, 82u8, 218u8, 18u8, 72u8, 75u8, 69u8, 89u8, 95u8, 76u8, 79u8, 67u8, 65u8, 76u8, 95u8,
    77u8, 65u8, 67u8, 72u8, 73u8, 78u8, 69u8, 218u8, 2u8, 104u8, 107u8, 218u8, 9u8, 79u8, 112u8,
    101u8, 110u8, 75u8, 101u8, 121u8, 69u8, 120u8, 218u8, 1u8, 105u8, 218u8, 10u8, 81u8, 117u8,
    101u8, 114u8, 121u8, 86u8, 97u8, 108u8, 117u8, 101u8, 218u8, 7u8, 69u8, 110u8, 117u8, 109u8,
    75u8, 101u8, 121u8, 218u8, 1u8, 118u8, 218u8, 10u8, 105u8, 115u8, 105u8, 110u8, 115u8, 116u8,
    97u8, 110u8, 99u8, 101u8, 218u8, 3u8, 115u8, 116u8, 114u8, 218u8, 6u8, 101u8, 120u8, 116u8,
    101u8, 110u8, 100u8, 218u8, 8u8, 67u8, 108u8, 111u8, 115u8, 101u8, 75u8, 101u8, 121u8, 218u8,
    10u8, 80u8, 89u8, 84u8, 72u8, 79u8, 78u8, 80u8, 65u8, 84u8, 72u8, 218u8, 10u8, 115u8, 116u8,
    97u8, 114u8, 116u8, 115u8, 119u8, 105u8, 116u8, 104u8, 169u8, 0u8, 114u8, 28u8, 0u8, 0u8, 0u8,
    114u8, 21u8, 0u8, 0u8, 0u8, 218u8, 8u8, 60u8, 109u8, 111u8, 100u8, 117u8, 108u8, 101u8, 62u8,
    114u8, 149u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 115u8, 106u8, 14u8, 0u8, 0u8, 240u8, 3u8, 1u8,
    1u8, 1u8, 240u8, 92u8, 5u8, 0u8, 14u8, 20u8, 143u8, 90u8, 137u8, 90u8, 152u8, 12u8, 211u8,
    13u8, 37u8, 215u8, 13u8, 51u8, 208u8, 13u8, 51u8, 168u8, 26u8, 128u8, 10u8, 216u8, 13u8, 23u8,
    215u8, 13u8, 29u8, 208u8, 13u8, 29u8, 152u8, 50u8, 128u8, 10u8, 224u8, 3u8, 10u8, 136u8, 103u8,
    212u8, 3u8, 21u8, 152u8, 23u8, 160u8, 72u8, 212u8, 25u8, 44u8, 216u8, 19u8, 35u8, 128u8, 76u8,
    216u8, 29u8, 35u8, 160u8, 77u8, 160u8, 63u8, 208u8, 27u8, 51u8, 208u8, 4u8, 24u8, 216u8, 23u8,
    33u8, 144u8, 108u8, 160u8, 39u8, 168u8, 45u8, 168u8, 31u8, 184u8, 1u8, 184u8, 45u8, 184u8,
    31u8, 200u8, 26u8, 200u8, 12u8, 208u8, 20u8, 85u8, 128u8, 77u8, 216u8, 27u8, 40u8, 152u8, 47u8,
    168u8, 22u8, 208u8, 24u8, 48u8, 176u8, 93u8, 176u8, 79u8, 192u8, 55u8, 208u8, 50u8, 75u8,
    208u8, 23u8, 76u8, 208u8, 4u8, 20u8, 216u8, 29u8, 39u8, 152u8, 76u8, 168u8, 7u8, 176u8, 13u8,
    168u8, 127u8, 184u8, 97u8, 192u8, 13u8, 184u8, 127u8, 200u8, 122u8, 200u8, 108u8, 208u8, 90u8,
    102u8, 208u8, 26u8, 103u8, 208u8, 4u8, 23u8, 216u8, 29u8, 40u8, 152u8, 77u8, 208u8, 4u8, 25u8,
    216u8, 20u8, 32u8, 128u8, 77u8, 216u8, 22u8, 32u8, 144u8, 92u8, 160u8, 23u8, 168u8, 29u8,
    168u8, 15u8, 184u8, 13u8, 176u8, 127u8, 192u8, 122u8, 192u8, 108u8, 208u8, 82u8, 86u8, 208u8,
    19u8, 87u8, 128u8, 76u8, 216u8, 12u8, 15u8, 128u8, 69u8, 216u8, 10u8, 13u8, 129u8, 67u8, 224u8,
    5u8, 12u8, 144u8, 4u8, 132u8, 95u8, 216u8, 19u8, 35u8, 128u8, 76u8, 216u8, 29u8, 35u8, 208u8,
    4u8, 24u8, 216u8, 20u8, 25u8, 128u8, 77u8, 216u8, 27u8, 40u8, 152u8, 47u8, 168u8, 23u8, 208u8,
    24u8, 49u8, 176u8, 109u8, 176u8, 95u8, 192u8, 72u8, 208u8, 51u8, 77u8, 208u8, 23u8, 78u8,
    208u8, 4u8, 20u8, 216u8, 29u8, 39u8, 152u8, 76u8, 208u8, 4u8, 23u8, 216u8, 29u8, 41u8, 152u8,
    78u8, 208u8, 4u8, 25u8, 216u8, 20u8, 32u8, 128u8, 77u8, 216u8, 21u8, 27u8, 152u8, 77u8, 152u8,
    63u8, 168u8, 61u8, 168u8, 47u8, 184u8, 42u8, 215u8, 58u8, 74u8, 208u8, 58u8, 74u8, 200u8, 2u8,
    208u8, 57u8, 75u8, 200u8, 52u8, 208u8, 19u8, 80u8, 128u8, 76u8, 216u8, 19u8, 49u8, 176u8, 40u8,
    176u8, 26u8, 184u8, 60u8, 208u8, 17u8, 72u8, 128u8, 74u8, 216u8, 12u8, 15u8, 128u8, 69u8,
    216u8, 10u8, 14u8, 128u8, 67u8, 240u8, 14u8, 4u8, 1u8, 33u8, 160u8, 118u8, 244u8, 0u8, 4u8,
    1u8, 33u8, 240u8, 22u8, 0u8, 16u8, 22u8, 143u8, 122u8, 137u8, 122u8, 152u8, 46u8, 211u8, 15u8,
    41u8, 128u8, 12u8, 216u8, 7u8, 13u8, 135u8, 122u8, 129u8, 122u8, 144u8, 38u8, 211u8, 7u8, 25u8,
    128u8, 4u8, 216u8, 13u8, 19u8, 143u8, 90u8, 137u8, 90u8, 152u8, 12u8, 211u8, 13u8, 37u8, 128u8,
    10u8, 216u8, 18u8, 24u8, 151u8, 42u8, 145u8, 42u8, 208u8, 29u8, 46u8, 211u8, 18u8, 47u8, 128u8,
    15u8, 216u8, 9u8, 15u8, 143u8, 26u8, 137u8, 26u8, 144u8, 72u8, 211u8, 9u8, 29u8, 128u8, 6u8,
    216u8, 14u8, 20u8, 143u8, 106u8, 137u8, 106u8, 152u8, 29u8, 211u8, 14u8, 39u8, 128u8, 11u8,
    216u8, 14u8, 20u8, 143u8, 106u8, 137u8, 106u8, 152u8, 29u8, 211u8, 14u8, 39u8, 128u8, 11u8,
    216u8, 19u8, 25u8, 151u8, 58u8, 145u8, 58u8, 208u8, 30u8, 48u8, 211u8, 19u8, 49u8, 208u8, 0u8,
    16u8, 216u8, 17u8, 23u8, 208u8, 24u8, 40u8, 213u8, 17u8, 41u8, 128u8, 14u8, 216u8, 18u8, 24u8,
    151u8, 42u8, 145u8, 42u8, 208u8, 29u8, 46u8, 176u8, 1u8, 211u8, 18u8, 50u8, 128u8, 15u8, 224u8,
    13u8, 19u8, 143u8, 90u8, 137u8, 90u8, 208u8, 24u8, 45u8, 211u8, 13u8, 46u8, 128u8, 10u8, 216u8,
    21u8, 27u8, 151u8, 90u8, 145u8, 90u8, 208u8, 32u8, 57u8, 211u8, 21u8, 58u8, 208u8, 0u8, 18u8,
    216u8, 13u8, 19u8, 143u8, 90u8, 137u8, 90u8, 152u8, 12u8, 211u8, 13u8, 37u8, 128u8, 10u8,
    217u8, 31u8, 35u8, 160u8, 74u8, 211u8, 31u8, 47u8, 208u8, 0u8, 28u8, 224u8, 22u8, 26u8, 208u8,
    0u8, 19u8, 216u8, 17u8, 21u8, 128u8, 14u8, 247u8, 12u8, 0u8, 8u8, 20u8, 240u8, 2u8, 3u8, 5u8,
    13u8, 216u8, 23u8, 29u8, 151u8, 122u8, 145u8, 122u8, 160u8, 43u8, 168u8, 114u8, 211u8, 23u8,
    50u8, 176u8, 49u8, 213u8, 23u8, 53u8, 136u8, 12u8, 247u8, 8u8, 0u8, 8u8, 20u8, 216u8, 19u8,
    39u8, 128u8, 76u8, 231u8, 3u8, 13u8, 145u8, 105u8, 160u8, 12u8, 168u8, 106u8, 215u8, 22u8,
    57u8, 210u8, 22u8, 57u8, 185u8, 103u8, 192u8, 108u8, 208u8, 85u8, 95u8, 213u8, 70u8, 95u8,
    215u8, 62u8, 96u8, 210u8, 62u8, 96u8, 216u8, 19u8, 31u8, 160u8, 42u8, 213u8, 19u8, 44u8, 128u8,
    76u8, 247u8, 14u8, 0u8, 4u8, 14u8, 247u8, 6u8, 0u8, 12u8, 22u8, 216u8, 21u8, 36u8, 136u8, 10u8,
    231u8, 7u8, 17u8, 144u8, 99u8, 152u8, 92u8, 212u8, 22u8, 41u8, 225u8, 17u8, 24u8, 152u8, 28u8,
    211u8, 17u8, 38u8, 128u8, 74u8, 231u8, 7u8, 17u8, 240u8, 6u8, 0u8, 18u8, 33u8, 129u8, 74u8,
    216u8, 5u8, 12u8, 144u8, 8u8, 212u8, 5u8, 24u8, 240u8, 12u8, 0u8, 23u8, 33u8, 128u8, 79u8,
    231u8, 7u8, 17u8, 151u8, 108u8, 167u8, 120u8, 240u8, 8u8, 0u8, 14u8, 22u8, 143u8, 94u8, 137u8,
    94u8, 152u8, 69u8, 214u8, 13u8, 34u8, 136u8, 1u8, 217u8, 12u8, 20u8, 144u8, 81u8, 152u8, 12u8,
    211u8, 12u8, 37u8, 136u8, 1u8, 217u8, 11u8, 18u8, 144u8, 49u8, 143u8, 58u8, 140u8, 58u8, 216u8,
    25u8, 26u8, 136u8, 74u8, 217u8, 12u8, 17u8, 241u8, 9u8, 0u8, 14u8, 35u8, 247u8, 12u8, 0u8, 8u8,
    18u8, 216u8, 17u8, 19u8, 128u8, 74u8, 241u8, 8u8, 0u8, 22u8, 29u8, 152u8, 83u8, 147u8, 92u8,
    128u8, 78u8, 240u8, 6u8, 0u8, 27u8, 41u8, 208u8, 4u8, 23u8, 231u8, 3u8, 23u8, 215u8, 27u8,
    50u8, 240u8, 10u8, 0u8, 8u8, 15u8, 144u8, 40u8, 212u8, 7u8, 26u8, 215u8, 31u8, 50u8, 241u8,
    10u8, 0u8, 30u8, 37u8, 160u8, 87u8, 211u8, 29u8, 45u8, 208u8, 28u8, 46u8, 168u8, 107u8, 184u8,
    45u8, 184u8, 31u8, 200u8, 1u8, 200u8, 45u8, 200u8, 31u8, 208u8, 26u8, 89u8, 137u8, 15u8, 240u8,
    10u8, 0u8, 27u8, 42u8, 215u8, 26u8, 55u8, 208u8, 26u8, 55u8, 168u8, 90u8, 136u8, 15u8, 231u8,
    11u8, 26u8, 216u8, 26u8, 41u8, 136u8, 15u8, 224u8, 17u8, 37u8, 215u8, 17u8, 64u8, 208u8, 17u8,
    64u8, 208u8, 41u8, 64u8, 128u8, 74u8, 217u8, 21u8, 28u8, 152u8, 90u8, 211u8, 21u8, 40u8, 128u8,
    78u8, 240u8, 18u8, 0u8, 16u8, 21u8, 128u8, 12u8, 231u8, 3u8, 7u8, 216u8, 19u8, 23u8, 129u8,
    76u8, 223u8, 5u8, 20u8, 159u8, 30u8, 183u8, 10u8, 216u8, 11u8, 25u8, 128u8, 68u8, 240u8, 14u8,
    0u8, 15u8, 19u8, 128u8, 11u8, 247u8, 10u8, 0u8, 8u8, 18u8, 128u8, 122u8, 240u8, 2u8, 14u8, 5u8,
    23u8, 240u8, 6u8, 0u8, 24u8, 38u8, 215u8, 23u8, 60u8, 208u8, 23u8, 60u8, 169u8, 23u8, 176u8,
    26u8, 211u8, 41u8, 60u8, 136u8, 12u8, 217u8, 22u8, 29u8, 152u8, 108u8, 211u8, 22u8, 43u8,
    136u8, 11u8, 240u8, 2u8, 6u8, 9u8, 39u8, 225u8, 24u8, 33u8, 161u8, 40u8, 168u8, 59u8, 184u8,
    13u8, 211u8, 34u8, 70u8, 211u8, 24u8, 71u8, 136u8, 73u8, 243u8, 24u8, 0u8, 17u8, 26u8, 136u8,
    4u8, 216u8, 30u8, 34u8, 159u8, 110u8, 153u8, 110u8, 168u8, 83u8, 211u8, 30u8, 49u8, 209u8, 8u8,
    27u8, 136u8, 3u8, 136u8, 87u8, 144u8, 101u8, 223u8, 11u8, 18u8, 137u8, 55u8, 144u8, 115u8,
    151u8, 121u8, 145u8, 121u8, 147u8, 123u8, 215u8, 23u8, 40u8, 209u8, 23u8, 40u8, 211u8, 23u8,
    42u8, 168u8, 102u8, 214u8, 23u8, 52u8, 231u8, 15u8, 19u8, 217u8, 16u8, 21u8, 240u8, 8u8, 0u8,
    52u8, 57u8, 183u8, 59u8, 177u8, 59u8, 179u8, 61u8, 208u8, 12u8, 64u8, 136u8, 78u8, 208u8, 29u8,
    48u8, 247u8, 8u8, 0u8, 20u8, 35u8, 240u8, 6u8, 6u8, 17u8, 25u8, 217u8, 38u8, 46u8, 168u8,
    122u8, 211u8, 38u8, 58u8, 144u8, 79u8, 216u8, 23u8, 38u8, 168u8, 42u8, 212u8, 23u8, 52u8,
    224u8, 42u8, 44u8, 152u8, 15u8, 247u8, 6u8, 0u8, 24u8, 39u8, 217u8, 38u8, 46u8, 168u8, 126u8,
    185u8, 120u8, 200u8, 10u8, 211u8, 63u8, 83u8, 211u8, 38u8, 84u8, 144u8, 79u8, 240u8, 12u8, 0u8,
    24u8, 31u8, 160u8, 36u8, 148u8, 127u8, 169u8, 118u8, 176u8, 111u8, 215u8, 47u8, 70u8, 210u8,
    47u8, 70u8, 217u8, 35u8, 43u8, 168u8, 74u8, 211u8, 35u8, 55u8, 152u8, 8u8, 216u8, 42u8, 62u8,
    192u8, 38u8, 200u8, 29u8, 200u8, 15u8, 208u8, 87u8, 88u8, 208u8, 89u8, 102u8, 208u8, 88u8,
    103u8, 208u8, 64u8, 104u8, 211u8, 41u8, 105u8, 152u8, 73u8, 216u8, 28u8, 37u8, 183u8, 122u8,
    169u8, 26u8, 192u8, 114u8, 213u8, 28u8, 73u8, 152u8, 73u8, 216u8, 31u8, 39u8, 168u8, 57u8,
    212u8, 31u8, 52u8, 217u8, 32u8, 40u8, 217u8, 40u8, 48u8, 176u8, 30u8, 192u8, 25u8, 211u8, 40u8,
    75u8, 152u8, 73u8, 241u8, 10u8, 0u8, 32u8, 38u8, 160u8, 105u8, 215u8, 31u8, 48u8, 212u8, 31u8,
    48u8, 216u8, 50u8, 59u8, 160u8, 15u8, 217u8, 32u8, 37u8, 241u8, 23u8, 0u8, 42u8, 106u8, 1u8,
    241u8, 26u8, 0u8, 13u8, 18u8, 241u8, 91u8, 1u8, 0u8, 17u8, 26u8, 247u8, 104u8, 1u8, 0u8, 8u8,
    23u8, 216u8, 22u8, 32u8, 215u8, 22u8, 57u8, 208u8, 22u8, 57u8, 160u8, 79u8, 215u8, 22u8, 57u8,
    208u8, 22u8, 57u8, 176u8, 114u8, 128u8, 79u8, 231u8, 7u8, 22u8, 216u8, 22u8, 37u8, 128u8, 79u8,
    231u8, 3u8, 18u8, 240u8, 2u8, 6u8, 5u8, 71u8, 1u8, 217u8, 26u8, 34u8, 160u8, 63u8, 211u8, 26u8,
    51u8, 136u8, 15u8, 247u8, 14u8, 0u8, 8u8, 22u8, 152u8, 39u8, 160u8, 88u8, 212u8, 26u8, 45u8,
    183u8, 39u8, 225u8, 18u8, 25u8, 152u8, 39u8, 211u8, 18u8, 34u8, 128u8, 75u8, 223u8, 7u8, 10u8,
    209u8, 10u8, 70u8, 209u8, 53u8, 69u8, 211u8, 10u8, 70u8, 135u8, 115u8, 135u8, 115u8, 130u8,
    115u8, 209u8, 10u8, 70u8, 209u8, 53u8, 69u8, 211u8, 10u8, 70u8, 215u8, 7u8, 70u8, 210u8, 7u8,
    70u8, 241u8, 6u8, 0u8, 26u8, 34u8, 160u8, 43u8, 211u8, 25u8, 46u8, 136u8, 14u8, 216u8, 30u8,
    44u8, 208u8, 8u8, 27u8, 247u8, 8u8, 0u8, 8u8, 22u8, 159u8, 47u8, 217u8, 43u8, 50u8, 176u8,
    63u8, 211u8, 43u8, 67u8, 208u8, 4u8, 67u8, 128u8, 78u8, 208u8, 21u8, 40u8, 247u8, 8u8, 0u8,
    8u8, 27u8, 159u8, 127u8, 217u8, 26u8, 33u8, 160u8, 47u8, 211u8, 26u8, 50u8, 208u8, 4u8, 23u8,
    240u8, 18u8, 0u8, 7u8, 11u8, 128u8, 3u8, 216u8, 10u8, 14u8, 128u8, 7u8, 247u8, 8u8, 0u8, 8u8,
    18u8, 159u8, 44u8, 240u8, 12u8, 0u8, 15u8, 22u8, 144u8, 122u8, 160u8, 63u8, 211u8, 13u8, 51u8,
    136u8, 1u8, 223u8, 11u8, 12u8, 137u8, 49u8, 216u8, 15u8, 22u8, 152u8, 36u8, 140u8, 127u8,
    161u8, 73u8, 168u8, 97u8, 176u8, 21u8, 215u8, 36u8, 55u8, 210u8, 36u8, 55u8, 185u8, 57u8,
    192u8, 81u8, 200u8, 5u8, 215u8, 59u8, 78u8, 210u8, 59u8, 78u8, 216u8, 20u8, 21u8, 151u8, 76u8,
    145u8, 76u8, 160u8, 19u8, 211u8, 20u8, 37u8, 160u8, 97u8, 213u8, 20u8, 40u8, 144u8, 1u8, 216u8,
    12u8, 13u8, 144u8, 23u8, 141u8, 76u8, 136u8, 65u8, 240u8, 2u8, 5u8, 13u8, 21u8, 217u8, 22u8,
    31u8, 160u8, 1u8, 147u8, 108u8, 144u8, 3u8, 217u8, 26u8, 33u8, 160u8, 33u8, 155u8, 42u8, 144u8,
    7u8, 217u8, 16u8, 21u8, 241u8, 17u8, 0u8, 14u8, 52u8, 247u8, 28u8, 0u8, 8u8, 15u8, 216u8, 26u8,
    27u8, 136u8, 15u8, 216u8, 15u8, 22u8, 136u8, 4u8, 216u8, 21u8, 23u8, 136u8, 10u8, 240u8, 14u8,
    0u8, 16u8, 20u8, 128u8, 12u8, 231u8, 9u8, 21u8, 215u8, 26u8, 45u8, 183u8, 106u8, 216u8, 11u8,
    17u8, 143u8, 58u8, 137u8, 58u8, 208u8, 22u8, 40u8, 168u8, 33u8, 211u8, 11u8, 44u8, 168u8,
    113u8, 212u8, 11u8, 48u8, 240u8, 6u8, 13u8, 5u8, 13u8, 217u8, 25u8, 33u8, 216u8, 12u8, 31u8,
    217u8, 12u8, 21u8, 145u8, 104u8, 208u8, 31u8, 50u8, 176u8, 76u8, 211u8, 22u8, 65u8, 211u8,
    12u8, 66u8, 192u8, 49u8, 213u8, 12u8, 69u8, 243u8, 5u8, 3u8, 26u8, 10u8, 136u8, 14u8, 241u8,
    8u8, 0u8, 24u8, 32u8, 208u8, 32u8, 51u8, 176u8, 85u8, 211u8, 23u8, 59u8, 136u8, 12u8, 247u8,
    20u8, 0u8, 8u8, 20u8, 216u8, 11u8, 18u8, 144u8, 100u8, 140u8, 63u8, 224u8, 34u8, 46u8, 209u8,
    12u8, 31u8, 225u8, 34u8, 43u8, 168u8, 76u8, 208u8, 34u8, 81u8, 208u8, 59u8, 80u8, 211u8, 34u8,
    81u8, 208u8, 12u8, 31u8, 231u8, 15u8, 43u8, 223u8, 15u8, 34u8, 217u8, 29u8, 37u8, 208u8, 38u8,
    57u8, 184u8, 53u8, 211u8, 29u8, 65u8, 145u8, 10u8, 225u8, 29u8, 37u8, 160u8, 108u8, 176u8,
    69u8, 211u8, 29u8, 58u8, 144u8, 10u8, 231u8, 15u8, 21u8, 216u8, 21u8, 40u8, 136u8, 70u8, 247u8,
    8u8, 0u8, 16u8, 27u8, 216u8, 26u8, 38u8, 136u8, 75u8, 240u8, 8u8, 0u8, 38u8, 39u8, 136u8, 6u8,
    208u8, 15u8, 33u8, 209u8, 8u8, 34u8, 247u8, 14u8, 0u8, 4u8, 14u8, 240u8, 6u8, 0u8, 28u8, 30u8,
    208u8, 4u8, 29u8, 128u8, 70u8, 138u8, 91u8, 247u8, 8u8, 0u8, 8u8, 12u8, 240u8, 6u8, 0u8, 42u8,
    46u8, 175u8, 30u8, 169u8, 30u8, 184u8, 5u8, 211u8, 41u8, 62u8, 209u8, 8u8, 38u8, 136u8, 6u8,
    144u8, 9u8, 152u8, 59u8, 223u8, 15u8, 24u8, 216u8, 26u8, 32u8, 136u8, 75u8, 231u8, 15u8, 43u8,
    216u8, 25u8, 29u8, 136u8, 74u8, 247u8, 8u8, 0u8, 8u8, 15u8, 151u8, 118u8, 217u8, 22u8, 29u8,
    152u8, 103u8, 211u8, 22u8, 38u8, 136u8, 11u8, 223u8, 11u8, 23u8, 216u8, 15u8, 22u8, 152u8,
    36u8, 140u8, 127u8, 225u8, 19u8, 25u8, 153u8, 40u8, 160u8, 59u8, 176u8, 12u8, 211u8, 26u8,
    61u8, 215u8, 19u8, 62u8, 210u8, 19u8, 62u8, 216u8, 29u8, 40u8, 144u8, 70u8, 248u8, 225u8, 25u8,
    34u8, 160u8, 59u8, 176u8, 12u8, 211u8, 25u8, 61u8, 144u8, 6u8, 223u8, 11u8, 24u8, 215u8, 29u8,
    45u8, 183u8, 102u8, 223u8, 15u8, 18u8, 209u8, 18u8, 78u8, 209u8, 61u8, 77u8, 211u8, 18u8, 78u8,
    143u8, 115u8, 143u8, 115u8, 138u8, 115u8, 209u8, 18u8, 78u8, 209u8, 61u8, 77u8, 211u8, 18u8,
    78u8, 215u8, 15u8, 78u8, 210u8, 15u8, 78u8, 216u8, 25u8, 36u8, 144u8, 6u8, 223u8, 23u8, 51u8,
    217u8, 33u8, 41u8, 168u8, 38u8, 176u8, 45u8, 211u8, 33u8, 64u8, 144u8, 74u8, 247u8, 8u8, 0u8,
    8u8, 20u8, 159u8, 14u8, 175u8, 118u8, 216u8, 11u8, 18u8, 144u8, 100u8, 140u8, 63u8, 225u8,
    15u8, 21u8, 145u8, 104u8, 152u8, 126u8, 168u8, 124u8, 211u8, 22u8, 60u8, 215u8, 15u8, 61u8,
    210u8, 15u8, 61u8, 216u8, 25u8, 39u8, 144u8, 6u8, 248u8, 225u8, 21u8, 30u8, 152u8, 126u8,
    168u8, 124u8, 211u8, 21u8, 60u8, 136u8, 70u8, 223u8, 11u8, 17u8, 215u8, 26u8, 54u8, 217u8,
    25u8, 33u8, 160u8, 38u8, 168u8, 45u8, 211u8, 25u8, 56u8, 136u8, 74u8, 217u8, 19u8, 24u8, 152u8,
    26u8, 215u8, 19u8, 36u8, 210u8, 19u8, 36u8, 216u8, 29u8, 33u8, 144u8, 10u8, 247u8, 8u8, 0u8,
    8u8, 21u8, 215u8, 25u8, 41u8, 175u8, 110u8, 199u8, 86u8, 217u8, 17u8, 26u8, 152u8, 62u8, 208u8,
    17u8, 61u8, 208u8, 44u8, 60u8, 211u8, 17u8, 61u8, 136u8, 6u8, 223u8, 11u8, 17u8, 159u8, 42u8,
    217u8, 25u8, 33u8, 160u8, 38u8, 168u8, 45u8, 211u8, 25u8, 56u8, 136u8, 74u8, 231u8, 7u8, 13u8,
    151u8, 102u8, 216u8, 17u8, 23u8, 136u8, 6u8, 223u8, 15u8, 18u8, 209u8, 18u8, 73u8, 209u8, 56u8,
    72u8, 211u8, 18u8, 73u8, 143u8, 115u8, 143u8, 115u8, 138u8, 115u8, 209u8, 18u8, 73u8, 209u8,
    56u8, 72u8, 211u8, 18u8, 73u8, 215u8, 15u8, 73u8, 210u8, 15u8, 73u8, 217u8, 12u8, 16u8, 208u8,
    17u8, 73u8, 212u8, 12u8, 74u8, 231u8, 11u8, 17u8, 217u8, 17u8, 24u8, 152u8, 18u8, 147u8, 27u8,
    136u8, 6u8, 217u8, 8u8, 12u8, 208u8, 13u8, 69u8, 212u8, 8u8, 70u8, 247u8, 8u8, 0u8, 8u8, 27u8,
    167u8, 59u8, 216u8, 11u8, 18u8, 144u8, 100u8, 140u8, 63u8, 240u8, 8u8, 0u8, 27u8, 33u8, 136u8,
    75u8, 223u8, 15u8, 26u8, 159u8, 118u8, 169u8, 37u8, 177u8, 8u8, 184u8, 22u8, 208u8, 65u8, 84u8,
    211u8, 48u8, 85u8, 215u8, 42u8, 86u8, 210u8, 42u8, 86u8, 216u8, 26u8, 32u8, 136u8, 75u8, 223u8,
    15u8, 26u8, 159u8, 126u8, 217u8, 26u8, 35u8, 160u8, 78u8, 208u8, 52u8, 71u8, 200u8, 101u8,
    212u8, 26u8, 84u8, 136u8, 75u8, 223u8, 15u8, 26u8, 159u8, 123u8, 216u8, 26u8, 37u8, 136u8,
    75u8, 223u8, 15u8, 26u8, 161u8, 37u8, 169u8, 8u8, 176u8, 27u8, 208u8, 62u8, 81u8, 211u8, 40u8,
    82u8, 215u8, 34u8, 83u8, 210u8, 34u8, 83u8, 216u8, 15u8, 22u8, 152u8, 36u8, 140u8, 127u8,
    247u8, 6u8, 0u8, 24u8, 38u8, 240u8, 14u8, 0u8, 38u8, 49u8, 144u8, 78u8, 248u8, 225u8, 16u8,
    20u8, 208u8, 21u8, 80u8, 212u8, 16u8, 81u8, 247u8, 8u8, 0u8, 12u8, 23u8, 216u8, 22u8, 28u8,
    136u8, 11u8, 247u8, 6u8, 0u8, 12u8, 18u8, 159u8, 27u8, 217u8, 8u8, 12u8, 208u8, 13u8, 71u8,
    212u8, 8u8, 72u8, 247u8, 8u8, 0u8, 4u8, 15u8, 223u8, 11u8, 22u8, 216u8, 22u8, 28u8, 136u8,
    11u8, 223u8, 11u8, 27u8, 216u8, 27u8, 38u8, 208u8, 8u8, 24u8, 216u8, 27u8, 38u8, 208u8, 4u8,
    38u8, 128u8, 70u8, 136u8, 91u8, 247u8, 10u8, 0u8, 8u8, 19u8, 216u8, 18u8, 24u8, 128u8, 75u8,
    223u8, 7u8, 23u8, 216u8, 23u8, 34u8, 208u8, 4u8, 20u8, 247u8, 14u8, 0u8, 4u8, 14u8, 224u8,
    36u8, 46u8, 215u8, 36u8, 52u8, 209u8, 36u8, 52u8, 176u8, 85u8, 211u8, 36u8, 59u8, 128u8, 70u8,
    208u8, 11u8, 32u8, 209u8, 4u8, 33u8, 216u8, 40u8, 41u8, 128u8, 70u8, 208u8, 11u8, 36u8, 211u8,
    4u8, 37u8, 231u8, 9u8, 27u8, 208u8, 9u8, 27u8, 240u8, 8u8, 0u8, 18u8, 20u8, 128u8, 74u8, 247u8,
    6u8, 0u8, 8u8, 23u8, 159u8, 62u8, 216u8, 17u8, 31u8, 215u8, 17u8, 37u8, 209u8, 17u8, 37u8,
    160u8, 101u8, 214u8, 17u8, 44u8, 136u8, 65u8, 216u8, 12u8, 22u8, 215u8, 12u8, 29u8, 209u8,
    12u8, 29u8, 153u8, 103u8, 160u8, 97u8, 155u8, 106u8, 214u8, 12u8, 41u8, 241u8, 3u8, 0u8, 18u8,
    45u8, 240u8, 8u8, 0u8, 8u8, 15u8, 144u8, 36u8, 132u8, 127u8, 231u8, 11u8, 18u8, 217u8, 26u8,
    33u8, 160u8, 39u8, 211u8, 26u8, 42u8, 137u8, 75u8, 224u8, 26u8, 40u8, 136u8, 75u8, 216u8, 8u8,
    18u8, 215u8, 8u8, 25u8, 209u8, 8u8, 25u8, 153u8, 40u8, 160u8, 59u8, 176u8, 12u8, 211u8, 26u8,
    61u8, 213u8, 8u8, 62u8, 223u8, 9u8, 21u8, 224u8, 8u8, 18u8, 215u8, 8u8, 25u8, 209u8, 8u8, 25u8,
    153u8, 40u8, 160u8, 54u8, 168u8, 60u8, 211u8, 26u8, 56u8, 213u8, 8u8, 57u8, 224u8, 8u8, 18u8,
    215u8, 8u8, 25u8, 209u8, 8u8, 25u8, 153u8, 40u8, 160u8, 59u8, 176u8, 12u8, 211u8, 26u8, 61u8,
    212u8, 8u8, 62u8, 224u8, 7u8, 14u8, 144u8, 36u8, 132u8, 127u8, 159u8, 63u8, 175u8, 118u8,
    240u8, 10u8, 0u8, 20u8, 26u8, 215u8, 19u8, 43u8, 209u8, 19u8, 43u8, 168u8, 86u8, 215u8, 45u8,
    70u8, 209u8, 45u8, 70u8, 211u8, 18u8, 71u8, 136u8, 66u8, 240u8, 2u8, 22u8, 13u8, 21u8, 216u8,
    22u8, 28u8, 215u8, 22u8, 38u8, 209u8, 22u8, 38u8, 160u8, 114u8, 168u8, 58u8, 211u8, 22u8, 54u8,
    144u8, 3u8, 240u8, 2u8, 18u8, 17u8, 41u8, 216u8, 24u8, 25u8, 144u8, 65u8, 216u8, 26u8, 30u8,
    240u8, 2u8, 3u8, 25u8, 34u8, 216u8, 32u8, 38u8, 215u8, 32u8, 49u8, 209u8, 32u8, 49u8, 176u8,
    35u8, 176u8, 118u8, 183u8, 126u8, 177u8, 126u8, 192u8, 99u8, 200u8, 49u8, 211u8, 55u8, 77u8,
    211u8, 32u8, 78u8, 152u8, 65u8, 241u8, 6u8, 0u8, 28u8, 38u8, 160u8, 97u8, 168u8, 19u8, 215u8,
    27u8, 45u8, 210u8, 27u8, 45u8, 216u8, 28u8, 38u8, 215u8, 28u8, 45u8, 209u8, 28u8, 45u8, 168u8,
    97u8, 175u8, 103u8, 169u8, 103u8, 176u8, 101u8, 171u8, 110u8, 212u8, 28u8, 61u8, 216u8, 24u8,
    25u8, 152u8, 81u8, 157u8, 6u8, 154u8, 1u8, 241u8, 25u8, 0u8, 19u8, 72u8, 1u8, 247u8, 52u8, 0u8,
    8u8, 18u8, 216u8, 17u8, 27u8, 215u8, 17u8, 33u8, 209u8, 17u8, 33u8, 160u8, 37u8, 214u8, 17u8,
    40u8, 136u8, 65u8, 216u8, 12u8, 22u8, 215u8, 12u8, 29u8, 209u8, 12u8, 29u8, 153u8, 104u8,
    160u8, 123u8, 176u8, 65u8, 211u8, 30u8, 54u8, 214u8, 12u8, 55u8, 241u8, 3u8, 0u8, 18u8, 41u8,
    247u8, 8u8, 0u8, 12u8, 22u8, 159u8, 43u8, 217u8, 21u8, 29u8, 152u8, 107u8, 168u8, 61u8, 211u8,
    21u8, 57u8, 136u8, 10u8, 223u8, 11u8, 25u8, 215u8, 30u8, 46u8, 217u8, 25u8, 33u8, 208u8, 34u8,
    50u8, 208u8, 52u8, 71u8, 211u8, 25u8, 72u8, 136u8, 14u8, 224u8, 7u8, 14u8, 144u8, 36u8, 132u8,
    127u8, 231u8, 11u8, 25u8, 216u8, 12u8, 22u8, 215u8, 12u8, 29u8, 209u8, 12u8, 29u8, 152u8,
    110u8, 212u8, 12u8, 45u8, 223u8, 11u8, 21u8, 216u8, 12u8, 22u8, 215u8, 12u8, 29u8, 209u8, 12u8,
    29u8, 152u8, 106u8, 212u8, 12u8, 41u8, 223u8, 11u8, 25u8, 152u8, 110u8, 176u8, 74u8, 212u8,
    30u8, 62u8, 240u8, 10u8, 0u8, 13u8, 23u8, 215u8, 12u8, 29u8, 209u8, 12u8, 29u8, 152u8, 110u8,
    212u8, 12u8, 45u8, 248u8, 231u8, 11u8, 21u8, 216u8, 12u8, 22u8, 215u8, 12u8, 29u8, 209u8, 12u8,
    29u8, 152u8, 106u8, 212u8, 12u8, 41u8, 223u8, 11u8, 25u8, 216u8, 12u8, 22u8, 215u8, 12u8, 29u8,
    209u8, 12u8, 29u8, 152u8, 110u8, 212u8, 12u8, 45u8, 224u8, 36u8, 46u8, 128u8, 70u8, 208u8,
    11u8, 32u8, 209u8, 4u8, 33u8, 216u8, 40u8, 41u8, 128u8, 70u8, 208u8, 11u8, 36u8, 209u8, 4u8,
    37u8, 240u8, 20u8, 0u8, 4u8, 11u8, 136u8, 100u8, 132u8, 63u8, 151u8, 124u8, 223u8, 11u8, 22u8,
    216u8, 17u8, 23u8, 151u8, 26u8, 145u8, 26u8, 152u8, 72u8, 211u8, 17u8, 37u8, 215u8, 17u8, 47u8,
    208u8, 17u8, 47u8, 168u8, 22u8, 136u8, 6u8, 216u8, 22u8, 28u8, 151u8, 106u8, 145u8, 106u8,
    160u8, 29u8, 211u8, 22u8, 47u8, 215u8, 22u8, 72u8, 208u8, 22u8, 72u8, 176u8, 59u8, 215u8, 22u8,
    72u8, 208u8, 22u8, 72u8, 192u8, 38u8, 136u8, 11u8, 216u8, 18u8, 24u8, 151u8, 42u8, 145u8, 42u8,
    152u8, 93u8, 211u8, 18u8, 43u8, 215u8, 18u8, 53u8, 208u8, 18u8, 53u8, 168u8, 118u8, 128u8,
    75u8, 216u8, 23u8, 29u8, 151u8, 122u8, 145u8, 122u8, 208u8, 34u8, 52u8, 211u8, 23u8, 53u8,
    215u8, 23u8, 83u8, 208u8, 23u8, 83u8, 184u8, 27u8, 215u8, 23u8, 83u8, 208u8, 23u8, 83u8, 200u8,
    11u8, 208u8, 4u8, 20u8, 247u8, 14u8, 0u8, 4u8, 7u8, 216u8, 25u8, 26u8, 128u8, 70u8, 136u8,
    58u8, 209u8, 4u8, 22u8, 216u8, 32u8, 33u8, 128u8, 70u8, 208u8, 11u8, 28u8, 209u8, 4u8, 29u8,
    216u8, 28u8, 29u8, 128u8, 70u8, 136u8, 61u8, 209u8, 4u8, 25u8, 216u8, 26u8, 27u8, 128u8, 70u8,
    136u8, 59u8, 209u8, 4u8, 23u8, 216u8, 17u8, 19u8, 128u8, 74u8, 219u8, 16u8, 19u8, 136u8, 4u8,
    216u8, 15u8, 19u8, 143u8, 126u8, 137u8, 126u8, 152u8, 99u8, 211u8, 15u8, 34u8, 160u8, 49u8,
    213u8, 15u8, 37u8, 215u8, 15u8, 43u8, 209u8, 15u8, 43u8, 211u8, 15u8, 45u8, 136u8, 4u8, 223u8,
    15u8, 19u8, 217u8, 12u8, 16u8, 216u8, 13u8, 17u8, 144u8, 93u8, 212u8, 13u8, 34u8, 216u8, 36u8,
    37u8, 136u8, 70u8, 144u8, 61u8, 211u8, 12u8, 33u8, 216u8, 13u8, 17u8, 143u8, 95u8, 137u8, 95u8,
    152u8, 89u8, 215u8, 13u8, 39u8, 210u8, 13u8, 39u8, 217u8, 12u8, 16u8, 208u8, 17u8, 58u8, 214u8,
    12u8, 59u8, 224u8, 12u8, 22u8, 215u8, 12u8, 29u8, 209u8, 12u8, 29u8, 153u8, 104u8, 160u8,
    119u8, 176u8, 4u8, 211u8, 30u8, 53u8, 214u8, 12u8, 54u8, 241u8, 19u8, 0u8, 17u8, 20u8, 240u8,
    20u8, 0u8, 37u8, 47u8, 128u8, 70u8, 208u8, 11u8, 32u8, 209u8, 4u8, 33u8, 216u8, 40u8, 41u8,
    128u8, 70u8, 208u8, 11u8, 36u8, 209u8, 4u8, 37u8, 240u8, 12u8, 0u8, 26u8, 38u8, 128u8, 6u8,
    128u8, 126u8, 209u8, 0u8, 22u8, 216u8, 17u8, 21u8, 128u8, 6u8, 128u8, 118u8, 129u8, 14u8,
    216u8, 23u8, 33u8, 128u8, 6u8, 128u8, 124u8, 209u8, 0u8, 20u8, 216u8, 28u8, 43u8, 128u8, 6u8,
    208u8, 7u8, 24u8, 209u8, 0u8, 25u8, 216u8, 19u8, 25u8, 128u8, 6u8, 128u8, 120u8, 209u8, 0u8,
    16u8, 216u8, 24u8, 35u8, 128u8, 6u8, 128u8, 125u8, 209u8, 0u8, 21u8, 216u8, 24u8, 35u8, 128u8,
    6u8, 128u8, 125u8, 209u8, 0u8, 21u8, 216u8, 29u8, 45u8, 128u8, 6u8, 208u8, 7u8, 25u8, 209u8,
    0u8, 26u8, 224u8, 23u8, 33u8, 128u8, 6u8, 128u8, 124u8, 209u8, 0u8, 20u8, 224u8, 23u8, 33u8,
    215u8, 23u8, 39u8, 208u8, 23u8, 39u8, 160u8, 82u8, 128u8, 6u8, 128u8, 124u8, 209u8, 0u8, 20u8,
    216u8, 27u8, 41u8, 215u8, 27u8, 47u8, 208u8, 27u8, 47u8, 168u8, 82u8, 128u8, 6u8, 208u8, 7u8,
    23u8, 211u8, 0u8, 24u8, 248u8, 240u8, 123u8, 17u8, 0u8, 12u8, 22u8, 244u8, 0u8, 1u8, 5u8, 13u8,
    218u8, 8u8, 12u8, 240u8, 3u8, 1u8, 5u8, 13u8, 251u8, 240u8, 96u8, 3u8, 0u8, 17u8, 34u8, 160u8,
    63u8, 208u8, 15u8, 51u8, 244u8, 0u8, 3u8, 9u8, 39u8, 225u8, 24u8, 33u8, 161u8, 40u8, 168u8,
    60u8, 184u8, 29u8, 211u8, 34u8, 71u8, 211u8, 24u8, 72u8, 136u8, 73u8, 216u8, 26u8, 38u8, 139u8,
    75u8, 240u8, 7u8, 3u8, 9u8, 39u8, 251u8, 240u8, 8u8, 0u8, 13u8, 30u8, 152u8, 127u8, 208u8,
    11u8, 47u8, 244u8, 0u8, 2u8, 5u8, 23u8, 216u8, 22u8, 26u8, 136u8, 11u8, 216u8, 20u8, 22u8,
    139u8, 9u8, 240u8, 5u8, 2u8, 5u8, 23u8, 251u8, 240u8, 56u8, 0u8, 24u8, 31u8, 244u8, 0u8, 1u8,
    17u8, 25u8, 218u8, 20u8, 24u8, 240u8, 3u8, 1u8, 17u8, 25u8, 251u8, 240u8, 80u8, 1u8, 0u8, 12u8,
    19u8, 244u8, 0u8, 4u8, 5u8, 71u8, 1u8, 241u8, 6u8, 0u8, 12u8, 18u8, 144u8, 47u8, 215u8, 11u8,
    34u8, 210u8, 11u8, 34u8, 217u8, 12u8, 16u8, 208u8, 19u8, 51u8, 176u8, 79u8, 208u8, 51u8, 68u8,
    208u8, 17u8, 69u8, 212u8, 12u8, 70u8, 255u8, 249u8, 240u8, 9u8, 4u8, 5u8, 71u8, 1u8, 251u8,
    240u8, 104u8, 1u8, 0u8, 20u8, 27u8, 244u8, 0u8, 1u8, 13u8, 21u8, 219u8, 16u8, 20u8, 240u8, 3u8,
    1u8, 13u8, 21u8, 251u8, 240u8, 54u8, 0u8, 12u8, 22u8, 244u8, 0u8, 3u8, 5u8, 60u8, 224u8, 25u8,
    44u8, 136u8, 14u8, 217u8, 23u8, 31u8, 208u8, 32u8, 51u8, 176u8, 85u8, 211u8, 23u8, 59u8, 139u8,
    12u8, 216u8, 12u8, 29u8, 152u8, 127u8, 208u8, 11u8, 47u8, 244u8, 0u8, 3u8, 5u8, 13u8, 242u8,
    6u8, 0u8, 9u8, 13u8, 240u8, 7u8, 3u8, 5u8, 13u8, 251u8, 240u8, 84u8, 6u8, 0u8, 32u8, 39u8,
    244u8, 0u8, 1u8, 25u8, 34u8, 217u8, 28u8, 33u8, 240u8, 3u8, 1u8, 25u8, 34u8, 250u8, 247u8,
    16u8, 0u8, 28u8, 32u8, 175u8, 10u8, 216u8, 28u8, 34u8, 215u8, 28u8, 45u8, 209u8, 28u8, 45u8,
    168u8, 99u8, 176u8, 52u8, 211u8, 28u8, 56u8, 152u8, 1u8, 217u8, 27u8, 37u8, 160u8, 97u8, 168u8,
    19u8, 215u8, 27u8, 45u8, 210u8, 27u8, 45u8, 216u8, 28u8, 38u8, 215u8, 28u8, 45u8, 209u8, 28u8,
    45u8, 168u8, 97u8, 175u8, 103u8, 169u8, 103u8, 176u8, 101u8, 171u8, 110u8, 212u8, 28u8, 61u8,
    224u8, 20u8, 26u8, 151u8, 79u8, 145u8, 79u8, 160u8, 67u8, 215u8, 20u8, 40u8, 248u8, 144u8,
    70u8, 151u8, 79u8, 145u8, 79u8, 160u8, 67u8, 213u8, 20u8, 40u8, 251u8, 216u8, 19u8, 26u8,
    244u8, 0u8, 1u8, 13u8, 21u8, 219u8, 16u8, 20u8, 240u8, 3u8, 1u8, 13u8, 21u8, 250u8, 115u8, 8u8,
    1u8, 0u8, 0u8, 198u8, 37u8, 25u8, 117u8, 27u8, 0u8, 204u8, 38u8, 8u8, 118u8, 10u8, 0u8, 204u8,
    47u8, 17u8, 118u8, 10u8, 0u8, 205u8, 1u8, 15u8, 117u8, 41u8, 0u8, 206u8, 62u8, 17u8, 118u8,
    30u8, 2u8, 210u8, 0u8, 8u8, 118u8, 44u8, 0u8, 214u8, 6u8, 16u8, 119u8, 26u8, 2u8, 215u8, 26u8,
    38u8, 119u8, 41u8, 0u8, 233u8, 26u8, 18u8, 122u8, 21u8, 2u8, 233u8, 45u8, 3u8, 121u8, 63u8,
    2u8, 233u8, 49u8, 34u8, 120u8, 13u8, 2u8, 234u8, 19u8, 14u8, 121u8, 63u8, 2u8, 234u8, 34u8,
    43u8, 121u8, 63u8, 2u8, 245u8, 27u8, 7u8, 117u8, 38u8, 3u8, 245u8, 37u8, 1u8, 117u8, 38u8, 3u8,
    245u8, 41u8, 26u8, 118u8, 7u8, 3u8, 246u8, 3u8, 1u8, 118u8, 10u8, 0u8, 246u8, 6u8, 1u8, 118u8,
    7u8, 3u8, 246u8, 7u8, 3u8, 118u8, 10u8, 0u8, 246u8, 10u8, 13u8, 118u8, 27u8, 3u8, 246u8, 26u8,
    1u8, 118u8, 27u8, 3u8, 246u8, 30u8, 7u8, 118u8, 41u8, 5u8, 246u8, 40u8, 1u8, 118u8, 41u8, 5u8,
    246u8, 44u8, 7u8, 119u8, 23u8, 3u8, 246u8, 51u8, 25u8, 119u8, 18u8, 3u8, 247u8, 18u8, 5u8,
    119u8, 23u8, 3u8, 247u8, 26u8, 7u8, 119u8, 38u8, 5u8, 247u8, 37u8, 1u8, 119u8, 38u8, 5u8,
    247u8, 41u8, 18u8, 120u8, 10u8, 3u8, 247u8, 62u8, 6u8, 120u8, 10u8, 3u8, 248u8, 5u8, 1u8,
    120u8, 10u8, 3u8, 248u8, 9u8, 1u8, 120u8, 10u8, 3u8, 248u8, 13u8, 7u8, 120u8, 23u8, 5u8, 248u8,
    20u8, 2u8, 121u8, 63u8, 2u8, 248u8, 22u8, 1u8, 120u8, 23u8, 5u8, 248u8, 23u8, 10u8, 121u8,
    63u8, 2u8, 248u8, 34u8, 7u8, 121u8, 63u8, 2u8, 248u8, 42u8, 32u8, 121u8, 63u8, 2u8, 249u8,
    11u8, 32u8, 121u8, 63u8, 2u8, 249u8, 43u8, 17u8, 122u8, 21u8, 2u8, 249u8, 63u8, 19u8, 122u8,
    18u8, 5u8, 250u8, 18u8, 3u8, 122u8, 21u8, 2u8, 250u8, 21u8, 7u8, 122u8, 33u8, 5u8, 250u8, 32u8,
    1u8, 122u8, 33u8, 5u8,
]);

// _Py_abspath()
// _PyStatus_EXCEPTION()
// _PyPathConfig_ReadGlobal()
// _PyMem_RawWcsdup()
// _PyThreadState_GET()
// PyMarshal_ReadObjectFromString
// DELIM
// Reference the precompiled getpath.py
// HELPER FUNCTIONS for getpath.py
#[unsafe(link_section = ".text.slate_distinct.getpath.getpath_abspath")]
extern "C-unwind" fn getpath_abspath(
    _unused_self: *mut _object,
    args: *mut _object,
) -> *mut _object {
    let mut r: *mut _object = core::ptr::null_mut::<_object>();
    let mut pathobj: *mut _object = unsafe { core::mem::zeroed() };
    if (unsafe { PyArg_ParseTuple(args, c"U".as_ptr(), core::ptr::addr_of_mut!(pathobj)) }) == 0 {
        return core::ptr::null_mut::<_object>();
    }
    let mut len: i64 = 0;
    let path: *mut i32 =
        unsafe { PyUnicode_AsWideCharString(pathobj, core::ptr::addr_of_mut!(len)) };
    if !path.is_null() {
        let mut abs: *mut i32 = unsafe { core::mem::zeroed() };
        if (unsafe {
            _Py_abspath(
                _Py_normpath(path, 1i32.wrapping_neg() as i64) as *const i32,
                core::ptr::addr_of_mut!(abs),
            )
        }) == 0
            && !abs.is_null()
        {
            r = unsafe { PyUnicode_FromWideChar(abs as *const i32, 1i32.wrapping_neg() as i64) };
            unsafe { PyMem_RawFree(abs as *mut ()) };
        } else {
            unsafe { PyErr_SetString(PyExc_OSError, c"failed to make path absolute".as_ptr()) };
        }
        unsafe { PyMem_Free(path as *mut ()) };
    }
    r
}

#[unsafe(link_section = ".text.slate_distinct.getpath.getpath_basename")]
extern "C-unwind" fn getpath_basename(
    _unused_self: *mut _object,
    args: *mut _object,
) -> *mut _object {
    let mut path: *mut _object = unsafe { core::mem::zeroed() };
    if (unsafe { PyArg_ParseTuple(args, c"U".as_ptr(), core::ptr::addr_of_mut!(path)) }) == 0 {
        return core::ptr::null_mut::<_object>();
    }
    let end: i64 = PyUnicode_GET_LENGTH!(path);
    let pos: i64 = unsafe { PyUnicode_FindChar(path, 47u32, 0i64, end, 1i32.wrapping_neg()) };
    if pos < 0 {
        return Py_NewRef!(path);
    }
    unsafe { PyUnicode_Substring(path, pos.wrapping_add(1), end) }
}

#[unsafe(link_section = ".text.slate_distinct.getpath.getpath_dirname")]
extern "C-unwind" fn getpath_dirname(
    _unused_self: *mut _object,
    args: *mut _object,
) -> *mut _object {
    let mut path: *mut _object = unsafe { core::mem::zeroed() };
    if (unsafe { PyArg_ParseTuple(args, c"U".as_ptr(), core::ptr::addr_of_mut!(path)) }) == 0 {
        return core::ptr::null_mut::<_object>();
    }
    let end: i64 = PyUnicode_GET_LENGTH!(path);
    let pos: i64 = unsafe { PyUnicode_FindChar(path, 47u32, 0i64, end, 1i32.wrapping_neg()) };
    if pos < 0 {
        return unsafe { Py_GetConstant(7u32) };
    }
    unsafe { PyUnicode_Substring(path, 0i64, pos) }
}

#[unsafe(link_section = ".text.slate_distinct.getpath.getpath_isabs")]
extern "C-unwind" fn getpath_isabs(_unused_self: *mut _object, args: *mut _object) -> *mut _object {
    let mut r: *mut _object = core::ptr::null_mut::<_object>();
    let mut pathobj: *mut _object = unsafe { core::mem::zeroed() };
    if (unsafe { PyArg_ParseTuple(args, c"U".as_ptr(), core::ptr::addr_of_mut!(pathobj)) }) == 0 {
        return core::ptr::null_mut::<_object>();
    }
    let path: *const i32 =
        (unsafe { PyUnicode_AsWideCharString(pathobj, core::ptr::null_mut::<i64>()) })
            as *const i32;
    if !path.is_null() {
        r = if (unsafe { _Py_isabs(path) }) != 0 {
            _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_TrueStruct) })
        } else {
            _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_FalseStruct) })
        };
        unsafe { PyMem_Free(path as *mut ()) };
    }
    Py_XNewRef!(r)
}

#[unsafe(link_section = ".text.slate_distinct.getpath.getpath_hassuffix")]
extern "C-unwind" fn getpath_hassuffix(
    _unused_self: *mut _object,
    args: *mut _object,
) -> *mut _object {
    let mut r: *mut _object = core::ptr::null_mut::<_object>();
    let mut pathobj: *mut _object = unsafe { core::mem::zeroed() };
    let mut suffixobj: *mut _object = unsafe { core::mem::zeroed() };
    if (unsafe {
        PyArg_ParseTuple(
            args,
            c"UU".as_ptr(),
            core::ptr::addr_of_mut!(pathobj),
            core::ptr::addr_of_mut!(suffixobj),
        )
    }) == 0
    {
        return core::ptr::null_mut::<_object>();
    }
    let mut len: i64 = 0;
    let mut suffixLen: i64 = 0;
    let path: *const i32 =
        (unsafe { PyUnicode_AsWideCharString(pathobj, core::ptr::addr_of_mut!(len)) })
            as *const i32;
    if !path.is_null() {
        let suffix: *const i32 =
            (unsafe { PyUnicode_AsWideCharString(suffixobj, core::ptr::addr_of_mut!(suffixLen)) })
                as *const i32;
        if !suffix.is_null() {
            if suffixLen > len
                || (unsafe {
                    wcscmp(
                        path.wrapping_offset(len.wrapping_sub(suffixLen) as isize),
                        suffix,
                    )
                }) != 0
            {
                r = Py_NewRef!(_PyObject_CAST__2!(unsafe {
                    core::ptr::addr_of_mut!(_Py_FalseStruct)
                }));
            } else {
                r = Py_NewRef!(_PyObject_CAST__2!(unsafe {
                    core::ptr::addr_of_mut!(_Py_TrueStruct)
                }));
            }
            unsafe { PyMem_Free(suffix as *mut ()) };
        }
        unsafe { PyMem_Free(path as *mut ()) };
    }
    r
}

#[unsafe(link_section = ".text.slate_distinct.getpath.getpath_isdir")]
extern "C-unwind" fn getpath_isdir(_unused_self: *mut _object, args: *mut _object) -> *mut _object {
    let mut r: *mut _object = core::ptr::null_mut::<_object>();
    let mut pathobj: *mut _object = unsafe { core::mem::zeroed() };
    if (unsafe { PyArg_ParseTuple(args, c"U".as_ptr(), core::ptr::addr_of_mut!(pathobj)) }) == 0 {
        return core::ptr::null_mut::<_object>();
    }
    let path: *const i32 =
        (unsafe { PyUnicode_AsWideCharString(pathobj, core::ptr::null_mut::<i64>()) })
            as *const i32;
    if !path.is_null() {
        let mut st: stat = unsafe { core::mem::zeroed() };
        r = if (unsafe { _Py_wstat(path, core::ptr::addr_of_mut!(st)) }) == 0
            && st.st_mode & 61440 == 16384
        {
            _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_TrueStruct) })
        } else {
            _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_FalseStruct) })
        };
        unsafe { PyMem_Free(path as *mut ()) };
    }
    Py_XNewRef!(r)
}

#[unsafe(link_section = ".text.slate_distinct.getpath.getpath_isfile")]
extern "C-unwind" fn getpath_isfile(
    _unused_self: *mut _object,
    args: *mut _object,
) -> *mut _object {
    let mut r: *mut _object = core::ptr::null_mut::<_object>();
    let mut pathobj: *mut _object = unsafe { core::mem::zeroed() };
    if (unsafe { PyArg_ParseTuple(args, c"U".as_ptr(), core::ptr::addr_of_mut!(pathobj)) }) == 0 {
        return core::ptr::null_mut::<_object>();
    }
    let path: *const i32 =
        (unsafe { PyUnicode_AsWideCharString(pathobj, core::ptr::null_mut::<i64>()) })
            as *const i32;
    if !path.is_null() {
        let mut st: stat = unsafe { core::mem::zeroed() };
        r = if (unsafe { _Py_wstat(path, core::ptr::addr_of_mut!(st)) }) == 0
            && st.st_mode & 61440 == 32768
        {
            _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_TrueStruct) })
        } else {
            _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_FalseStruct) })
        };
        unsafe { PyMem_Free(path as *mut ()) };
    }
    Py_XNewRef!(r)
}

#[unsafe(link_section = ".text.slate_distinct.getpath.getpath_isxfile")]
extern "C-unwind" fn getpath_isxfile(
    _unused_self: *mut _object,
    args: *mut _object,
) -> *mut _object {
    let mut r: *mut _object = core::ptr::null_mut::<_object>();
    let mut pathobj: *mut _object = unsafe { core::mem::zeroed() };
    let mut cchPath: i64 = 0;
    if (unsafe { PyArg_ParseTuple(args, c"U".as_ptr(), core::ptr::addr_of_mut!(pathobj)) }) == 0 {
        return core::ptr::null_mut::<_object>();
    }
    let path: *const i32 =
        (unsafe { PyUnicode_AsWideCharString(pathobj, core::ptr::addr_of_mut!(cchPath)) })
            as *const i32;
    if !path.is_null() {
        let mut st: stat = unsafe { core::mem::zeroed() };
        r = if (unsafe { _Py_wstat(path, core::ptr::addr_of_mut!(st)) }) == 0
            && st.st_mode & 61440 == 32768
            && st.st_mode & 73 != 0
        {
            _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_TrueStruct) })
        } else {
            _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_FalseStruct) })
        };
        unsafe { PyMem_Free(path as *mut ()) };
    }
    Py_XNewRef!(r)
}

#[unsafe(link_section = ".text.slate_distinct.getpath.getpath_joinpath")]
extern "C-unwind" fn getpath_joinpath(
    _unused_self: *mut _object,
    args: *mut _object,
) -> *mut _object {
    if PyTuple_Check!(args) == 0 {
        unsafe { PyErr_SetString(PyExc_TypeError, c"requires tuple of arguments".as_ptr()) };
        return core::ptr::null_mut::<_object>();
    }
    let n: i64 = PyTuple_GET_SIZE!(args);
    if n == 0 {
        return unsafe { Py_GetConstant(7u32) };
    }
    // Convert all parts to wchar and accumulate max final length
    let parts: *mut *mut i32 =
        (unsafe { PyMem_Malloc((n as u64).wrapping_mul(core::mem::size_of::<*mut i32>() as u64)) })
            as *mut *mut i32;
    if parts.is_null() {
        unsafe { PyErr_NoMemory() };
        return core::ptr::null_mut::<_object>();
    }
    unsafe {
        core::ptr::write_bytes(
            ((parts as *mut ()) as *mut core::ffi::c_void) as *mut u8,
            (0i32 as i32) as u8,
            ((n as u64).wrapping_mul(core::mem::size_of::<*mut i32>() as u64) as usize) as usize,
        )
    };
    let mut cchFinal: i64 = 0;
    let mut first: i64 = 0;
    for i in 0..n {
        let s: *mut _object = PyTuple_GET_ITEM!(args, i);
        let mut cch: i64 = 0;
        if s == unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } {
            cch = 0;
        } else {
            if PyUnicode_Check!(s) != 0 {
                unsafe {
                    *parts.wrapping_offset(i as isize) =
                        PyUnicode_AsWideCharString(s, core::ptr::addr_of_mut!(cch));
                }
                if unsafe { *parts.wrapping_offset(i as isize) }.is_null() {
                    cchFinal = 1i32.wrapping_neg() as i64;
                    break;
                }
                if (unsafe { _Py_isabs(*parts.wrapping_offset(i as isize) as *const i32) }) != 0 {
                    first = i;
                }
            } else {
                unsafe {
                    PyErr_SetString(
                        PyExc_TypeError,
                        c"all arguments to joinpath() must be str or None".as_ptr(),
                    )
                };
                cchFinal = 1i32.wrapping_neg() as i64;
                break;
            }
        }
        cchFinal = cchFinal.wrapping_add(cch.wrapping_add(1));
    }
    let mut r#final: *mut i32 = unsafe { core::mem::zeroed() };
    let __v3: *mut i32 = if cchFinal > 0 {
        (unsafe {
            PyMem_Malloc((cchFinal as u64).wrapping_mul(core::mem::size_of::<i32>() as u64))
        }) as *mut i32
    } else {
        core::ptr::null_mut::<i32>()
    };
    r#final = __v3;
    if r#final.is_null() {
        for i in 0..n {
            unsafe { PyMem_Free(*parts.wrapping_offset(i as isize) as *mut ()) };
        }
        unsafe { PyMem_Free(parts as *mut ()) };
        if cchFinal != 0 {
            unsafe { PyErr_NoMemory() };
            return core::ptr::null_mut::<_object>();
        }
        return unsafe { Py_GetConstant(7u32) };
    }
    unsafe {
        *r#final.wrapping_offset(0 as isize) = 0;
    }
    // Now join all the paths. The final result should be shorter than the buffer
    for i in 0..n {
        if unsafe { *parts.wrapping_offset(i as isize) }.is_null() {
            continue;
        }
        if i >= first && !r#final.is_null() {
            if (unsafe { *r#final.wrapping_offset(0 as isize) }) == 0 {
                // final is definitely long enough to fit any individual part
                unsafe { wcscpy(r#final, *parts.wrapping_offset(i as isize) as *const i32) };
            } else {
                if (unsafe {
                    _Py_add_relfile(
                        r#final,
                        *parts.wrapping_offset(i as isize) as *const i32,
                        cchFinal as u64,
                    )
                }) < 0
                {
                    // if we fail, keep iterating to free memory, but stop adding parts
                    unsafe { PyMem_Free(r#final as *mut ()) };
                    r#final = core::ptr::null_mut::<i32>();
                }
            }
        }
        unsafe { PyMem_Free(*parts.wrapping_offset(i as isize) as *mut ()) };
    }
    unsafe { PyMem_Free(parts as *mut ()) };
    if r#final.is_null() {
        unsafe { PyErr_SetString(PyExc_SystemError, c"failed to join paths".as_ptr()) };
        return core::ptr::null_mut::<_object>();
    }
    let r: *mut _object = unsafe {
        PyUnicode_FromWideChar(
            _Py_normpath(r#final, 1i32.wrapping_neg() as i64) as *const i32,
            1i32.wrapping_neg() as i64,
        )
    };
    unsafe { PyMem_Free(r#final as *mut ()) };
    r
}

#[unsafe(link_section = ".text.slate_distinct.getpath.getpath_readlines")]
extern "C-unwind" fn getpath_readlines(
    _unused_self: *mut _object,
    args: *mut _object,
) -> *mut _object {
    let mut r: *mut _object = core::ptr::null_mut::<_object>();
    let mut pathobj: *mut _object = unsafe { core::mem::zeroed() };
    if (unsafe { PyArg_ParseTuple(args, c"U".as_ptr(), core::ptr::addr_of_mut!(pathobj)) }) == 0 {
        return core::ptr::null_mut::<_object>();
    }
    let path: *const i32 =
        (unsafe { PyUnicode_AsWideCharString(pathobj, core::ptr::null_mut::<i64>()) })
            as *const i32;
    if path.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let fp: *mut slate_support::FILE = unsafe {
        _Py_wfopen(
            path,
            (core::ptr::addr_of_mut!(__slate_str804) as *mut i32) as *const i32,
        )
    };
    if fp.is_null() {
        unsafe { PyErr_SetFromErrno(PyExc_OSError) };
        unsafe { PyMem_Free(path as *mut ()) };
        return core::ptr::null_mut::<_object>();
    }
    unsafe { PyMem_Free(path as *mut ()) };
    r = unsafe { PyList_New(0i64) };
    if r.is_null() {
        unsafe { slate_support::fclose(fp) };
        return core::ptr::null_mut::<_object>();
    }
    let MAX_FILE: u64 = 32768u64;
    let buffer: *mut i8 = (unsafe { PyMem_Malloc(MAX_FILE) }) as *mut i8;
    if buffer.is_null() {
        Py_DECREF!(r);
        unsafe { slate_support::fclose(fp) };
        return core::ptr::null_mut::<_object>();
    }
    let cb: u64 = unsafe { fread(buffer as *mut (), 1u64, MAX_FILE, fp) };
    unsafe { slate_support::fclose(fp) };
    if cb == 0 {
        return r;
    }
    if cb >= MAX_FILE {
        Py_DECREF!(r);
        unsafe {
            PyErr_SetString(
                PyExc_MemoryError,
                c"cannot read file larger than 32KB during initialization".as_ptr(),
            )
        };
        return core::ptr::null_mut::<_object>();
    }
    unsafe {
        *buffer.wrapping_offset(cb as isize) = 0;
    }
    let mut len: u64 = 0;
    let wbuffer: *mut i32 = unsafe {
        _Py_DecodeUTF8_surrogateescape(buffer as *const i8, cb as i64, core::ptr::addr_of_mut!(len))
    };
    unsafe { PyMem_Free(buffer as *mut ()) };
    if wbuffer.is_null() {
        Py_DECREF!(r);
        unsafe { PyErr_NoMemory() };
        return core::ptr::null_mut::<_object>();
    }
    let mut p1: *mut i32 = wbuffer;
    let mut p2: *mut i32 = p1;
    loop {
        let __v6: *mut i32 = unsafe { wcschr(p1 as *const i32, 10i32) };
        p2 = __v6;
        if __v6.is_null() {
            break;
        }
        let mut cb: i64 = (unsafe { p2.offset_from(p1 as *mut i32) }) as i64;
        while cb >= 0
            && ((unsafe { *p1.wrapping_offset(cb as isize) }) == 10
                || (unsafe { *p1.wrapping_offset(cb as isize) }) == 13)
        {
            cb = cb.wrapping_sub(1);
        }
        let u: *mut _object = unsafe {
            PyUnicode_FromWideChar(
                p1 as *const i32,
                if cb >= 0 { cb.wrapping_add(1) } else { 0i64 },
            )
        };
        let __v3: bool = u.is_null() || (unsafe { PyList_Append(r, u) }) < 0;
        if __v3 {
            Py_XDECREF!(u);
            Py_CLEAR__2!(r);
            break;
        }
        Py_DECREF!(u);
        p1 = p2.wrapping_offset(1 as isize);
    }
    if !r.is_null() && !p1.is_null() && (unsafe { *p1 }) != 0 {
        let u: *mut _object =
            unsafe { PyUnicode_FromWideChar(p1 as *const i32, 1i32.wrapping_neg() as i64) };
        let __v7: bool = u.is_null() || (unsafe { PyList_Append(r, u) }) < 0;
        if __v7 {
            Py_CLEAR__2!(r);
        }
        Py_XDECREF!(u);
    }
    unsafe { PyMem_RawFree(wbuffer as *mut ()) };
    r
}

static mut __slate_str804: [i32; 3] = [114, 98, 0];

#[unsafe(link_section = ".text.slate_distinct.getpath.getpath_realpath")]
extern "C-unwind" fn getpath_realpath(
    _unused_self: *mut _object,
    args: *mut _object,
) -> *mut _object {
    let mut s: *mut i32 = unsafe { core::mem::zeroed() };
    let mut resolved: core::mem::MaybeUninit<__SlateAlign16<[i32; 4097]>> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_4: *mut [i32; 4097] = core::ptr::addr_of_mut!(resolved) as *mut [i32; 4097];
    let mut path2: *mut i32 = unsafe { core::mem::zeroed() };
    let mut path: *mut i32 = unsafe { core::mem::zeroed() };
    let mut nlink: i32 = unsafe { core::mem::zeroed() };
    // This readlink calculation only resolves a symlinked file, and
    // does not resolve any path segments. This is consistent with
    // prior releases, however, the realpath implementation below is
    // potentially correct in more cases.
    let mut r: *mut _object = unsafe { core::mem::zeroed() };
    let mut pathobj: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        if PyArg_ParseTuple(args, c"U".as_ptr(), core::ptr::addr_of_mut!(pathobj)) == 0 {
            return core::ptr::null_mut::<_object>();
        } else {
            r = core::ptr::null_mut::<_object>();
            nlink = 0;
            path = PyUnicode_AsWideCharString(pathobj, core::ptr::null_mut::<i64>());
            if !path.is_null() {
                path2 = _PyMem_RawWcsdup(path as *const i32);
                PyMem_Free(path as *mut ());
                path = path2;
                '__join_2: {
                    '__join_13: {
                        loop {
                            if !path.is_null() {
                                let linklen: i32 = _Py_wreadlink(
                                    path as *const i32,
                                    (*__slate_slot_4).as_mut_ptr() as *mut i32,
                                    Py_ARRAY_LENGTH!(),
                                );
                                if linklen == 1i32.wrapping_neg() {
                                    break '__join_13;
                                } else {
                                    if _Py_isabs((*__slate_slot_4).as_mut_ptr() as *const i32) != 0
                                    {
                                        PyMem_RawFree(path as *mut ());
                                        path = _PyMem_RawWcsdup(
                                            (*__slate_slot_4).as_mut_ptr() as *const i32
                                        );
                                    } else {
                                        s = wcsrchr(path as *const i32, 47i32);
                                        if !s.is_null() {
                                            *s = 0;
                                        }
                                        path2 = _Py_join_relfile(
                                            path as *const i32,
                                            (*__slate_slot_4).as_mut_ptr() as *const i32,
                                        );
                                        if !path2.is_null() {
                                            path2 = _Py_normpath(path2, 1i32.wrapping_neg() as i64);
                                        }
                                        PyMem_RawFree(path as *mut ());
                                        path = path2;
                                    }
                                    nlink = nlink.wrapping_add(1);
                                    // 40 is the Linux kernel 4.2 limit
                                    if nlink >= 40 {
                                        break;
                                    }
                                }
                            } else {
                                break '__join_2;
                            }
                        }
                        PyErr_SetString(
                            PyExc_OSError,
                            c"maximum number of symbolic links reached".as_ptr(),
                        );
                        break '__join_2;
                    }
                    r = PyUnicode_FromWideChar(path as *const i32, 1i32.wrapping_neg() as i64);
                }
                if path.is_null() {
                    PyErr_NoMemory();
                }
            }
            PyMem_RawFree(path as *mut ());
            return r;
        }
    }
}

static mut getpath_methods: __SlateAlign16<[PyMethodDef; 12]> = __SlateAlign16([
    {
        let mut __t0: PyMethodDef = unsafe { core::mem::zeroed() };
        __t0.ml_name = (b"abspath\0".as_ptr() as *mut i8) as *const i8;
        __t0.ml_meth = Some(getpath_abspath);
        __t0.ml_flags = 1i32;
        __t0.ml_doc = core::ptr::null::<i8>();
        __t0
    },
    {
        let mut __t1: PyMethodDef = unsafe { core::mem::zeroed() };
        __t1.ml_name = (b"basename\0".as_ptr() as *mut i8) as *const i8;
        __t1.ml_meth = Some(getpath_basename);
        __t1.ml_flags = 1i32;
        __t1.ml_doc = core::ptr::null::<i8>();
        __t1
    },
    {
        let mut __t2: PyMethodDef = unsafe { core::mem::zeroed() };
        __t2.ml_name = (b"dirname\0".as_ptr() as *mut i8) as *const i8;
        __t2.ml_meth = Some(getpath_dirname);
        __t2.ml_flags = 1i32;
        __t2.ml_doc = core::ptr::null::<i8>();
        __t2
    },
    {
        let mut __t3: PyMethodDef = unsafe { core::mem::zeroed() };
        __t3.ml_name = (b"hassuffix\0".as_ptr() as *mut i8) as *const i8;
        __t3.ml_meth = Some(getpath_hassuffix);
        __t3.ml_flags = 1i32;
        __t3.ml_doc = core::ptr::null::<i8>();
        __t3
    },
    {
        let mut __t4: PyMethodDef = unsafe { core::mem::zeroed() };
        __t4.ml_name = (b"isabs\0".as_ptr() as *mut i8) as *const i8;
        __t4.ml_meth = Some(getpath_isabs);
        __t4.ml_flags = 1i32;
        __t4.ml_doc = core::ptr::null::<i8>();
        __t4
    },
    {
        let mut __t5: PyMethodDef = unsafe { core::mem::zeroed() };
        __t5.ml_name = (b"isdir\0".as_ptr() as *mut i8) as *const i8;
        __t5.ml_meth = Some(getpath_isdir);
        __t5.ml_flags = 1i32;
        __t5.ml_doc = core::ptr::null::<i8>();
        __t5
    },
    {
        let mut __t6: PyMethodDef = unsafe { core::mem::zeroed() };
        __t6.ml_name = (b"isfile\0".as_ptr() as *mut i8) as *const i8;
        __t6.ml_meth = Some(getpath_isfile);
        __t6.ml_flags = 1i32;
        __t6.ml_doc = core::ptr::null::<i8>();
        __t6
    },
    {
        let mut __t7: PyMethodDef = unsafe { core::mem::zeroed() };
        __t7.ml_name = (b"isxfile\0".as_ptr() as *mut i8) as *const i8;
        __t7.ml_meth = Some(getpath_isxfile);
        __t7.ml_flags = 1i32;
        __t7.ml_doc = core::ptr::null::<i8>();
        __t7
    },
    {
        let mut __t8: PyMethodDef = unsafe { core::mem::zeroed() };
        __t8.ml_name = (b"joinpath\0".as_ptr() as *mut i8) as *const i8;
        __t8.ml_meth = Some(getpath_joinpath);
        __t8.ml_flags = 1i32;
        __t8.ml_doc = core::ptr::null::<i8>();
        __t8
    },
    {
        let mut __t9: PyMethodDef = unsafe { core::mem::zeroed() };
        __t9.ml_name = (b"readlines\0".as_ptr() as *mut i8) as *const i8;
        __t9.ml_meth = Some(getpath_readlines);
        __t9.ml_flags = 1i32;
        __t9.ml_doc = core::ptr::null::<i8>();
        __t9
    },
    {
        let mut __t10: PyMethodDef = unsafe { core::mem::zeroed() };
        __t10.ml_name = (b"realpath\0".as_ptr() as *mut i8) as *const i8;
        __t10.ml_meth = Some(getpath_realpath);
        __t10.ml_flags = 1i32;
        __t10.ml_doc = core::ptr::null::<i8>();
        __t10
    },
    {
        let mut __t11: PyMethodDef = unsafe { core::mem::zeroed() };
        __t11.ml_name = core::ptr::null::<i8>();
        __t11.ml_meth = None;
        __t11.ml_flags = 0i32;
        __t11.ml_doc = core::ptr::null::<i8>();
        __t11
    },
]);

// Two implementations of warn() to use depending on whether warnings
// are enabled or not.
#[unsafe(link_section = ".text.slate_distinct.getpath.getpath_warn")]
extern "C-unwind" fn getpath_warn(_unused_self: *mut _object, args: *mut _object) -> *mut _object {
    let mut msgobj: *mut _object = unsafe { core::mem::zeroed() };
    if (unsafe { PyArg_ParseTuple(args, c"U".as_ptr(), core::ptr::addr_of_mut!(msgobj)) }) == 0 {
        return core::ptr::null_mut::<_object>();
    }
    let _ = std::io::Write::flush(&mut std::io::stdout());
    unsafe {
        slate_support::fprintf(
            slate_support::stderr(),
            c"%s\n".as_ptr(),
            PyUnicode_AsUTF8(msgobj),
        )
    };
    unsafe { fflush(core::ptr::null_mut()) };
    unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
}

#[unsafe(link_section = ".text.slate_distinct.getpath.getpath_nowarn")]
extern "C-unwind" fn getpath_nowarn(
    _unused_self: *mut _object,
    args: *mut _object,
) -> *mut _object {
    unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
}

static mut getpath_warn_method: PyMethodDef = {
    let mut __t0: PyMethodDef = unsafe { core::mem::zeroed() };
    __t0.ml_name = (b"warn\0".as_ptr() as *mut i8) as *const i8;
    __t0.ml_meth = Some(getpath_warn);
    __t0.ml_flags = 1i32;
    __t0.ml_doc = core::ptr::null::<i8>();
    __t0
};

static mut getpath_nowarn_method: PyMethodDef = {
    let mut __t0: PyMethodDef = unsafe { core::mem::zeroed() };
    __t0.ml_name = (b"warn\0".as_ptr() as *mut i8) as *const i8;
    __t0.ml_meth = Some(getpath_nowarn);
    __t0.ml_flags = 1i32;
    __t0.ml_doc = core::ptr::null::<i8>();
    __t0
};

/// Add the helper functions to the dict
fn funcs_to_dict(dict: *mut _object, warnings: i32) -> i32 {
    let mut m: *mut PyMethodDef =
        unsafe { core::ptr::addr_of_mut!(getpath_methods.0) as *mut PyMethodDef };
    while !unsafe { (*m).ml_name }.is_null() {
        let f: *mut _object = PyCFunction_NewEx__2!(m);
        if f.is_null() {
            return 0;
        }
        if (unsafe { PyDict_SetItemString(dict, (*m).ml_name, f) }) < 0 {
            Py_DECREF!(f);
            return 0;
        }
        Py_DECREF!(f);
        let __v1: *mut PyMethodDef = m;
        m = __v1.wrapping_offset(1 as isize);
    }
    let m2: *mut PyMethodDef = if warnings != 0 {
        unsafe { core::ptr::addr_of_mut!(getpath_warn_method) }
    } else {
        unsafe { core::ptr::addr_of_mut!(getpath_nowarn_method) }
    };
    let f: *mut _object = PyCFunction_NewEx__2!(m2);
    if f.is_null() {
        return 0;
    }
    if (unsafe { PyDict_SetItemString(dict, (*m2).ml_name, f) }) < 0 {
        Py_DECREF!(f);
        return 0;
    }
    Py_DECREF!(f);
    1
}

/// Add a wide-character string constant to the dict
fn wchar_to_dict(dict: *mut _object, key: *const i8, s: *const i32) -> i32 {
    let mut u: *mut _object = unsafe { core::mem::zeroed() };
    let mut r: i32 = 0;
    if !s.is_null() && (unsafe { *s.wrapping_offset(0 as isize) }) != 0 {
        u = unsafe { PyUnicode_FromWideChar(s, 1i32.wrapping_neg() as i64) };
        if u.is_null() {
            return 0;
        }
    } else {
        u = Py_NewRef!(unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) });
    }
    r = ((unsafe { PyDict_SetItemString(dict, key, u) }) == 0) as i32;
    Py_DECREF!(u);
    r
}

/// Add a narrow string constant to the dict, using default locale decoding
fn decode_to_dict(dict: *mut _object, key: *const i8, s: *const i8) -> i32 {
    let mut u: *mut _object = core::ptr::null_mut::<_object>();
    let mut r: i32 = 0;
    if !s.is_null() && (unsafe { *s.wrapping_offset(0 as isize) }) != 0 {
        let mut len: u64 = 0;
        let w: *const i32 =
            (unsafe { Py_DecodeLocale(s, core::ptr::addr_of_mut!(len)) }) as *const i32;
        if !w.is_null() {
            u = unsafe { PyUnicode_FromWideChar(w, len as i64) };
            unsafe { PyMem_RawFree(w as *mut ()) };
        }
        if u.is_null() {
            return 0;
        }
    } else {
        u = Py_NewRef!(unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) });
    }
    r = ((unsafe { PyDict_SetItemString(dict, key, u) }) == 0) as i32;
    Py_DECREF!(u);
    r
}

/// Add an environment variable to the dict, optionally clearing it afterwards
fn env_to_dict(dict: *mut _object, key: *const i8, and_clear: i32) -> i32 {
    let mut u: *mut _object = core::ptr::null_mut::<_object>();
    let mut r: i32 = 0;
    let v: *const i8 =
        (unsafe { getenv(key.wrapping_offset(4 as isize) as *const core::ffi::c_char) as *mut i8 })
            as *const i8;
    if !v.is_null() {
        let mut len: u64 = 0;
        let w: *const i32 =
            (unsafe { Py_DecodeLocale(v, core::ptr::addr_of_mut!(len)) }) as *const i32;
        if !w.is_null() {
            u = unsafe { PyUnicode_FromWideChar(w, len as i64) };
            if u.is_null() {
                unsafe { PyErr_Clear() };
            }
            unsafe { PyMem_RawFree(w as *mut ()) };
        }
    }
    if !u.is_null() {
        r = ((unsafe { PyDict_SetItemString(dict, key, u) }) == 0) as i32;
        Py_DECREF!(u);
    } else {
        r = ((unsafe { PyDict_SetItemString(dict, key, core::ptr::addr_of_mut!(_Py_NoneStruct)) })
            == 0) as i32;
    }
    if r != 0 && and_clear != 0 {
        unsafe { unsetenv(key.wrapping_offset(4 as isize) as *const core::ffi::c_char) as i32 };
    }
    r
}

/// Add an integer constant to the dict
fn int_to_dict(dict: *mut _object, key: *const i8, v: i32) -> i32 {
    let mut r: i32 = 0;
    let o: *mut _object = unsafe { PyLong_FromLong(v as i64) };
    if o.is_null() {
        return 0;
    }
    r = ((unsafe { PyDict_SetItemString(dict, key, o) }) == 0) as i32;
    Py_DECREF!(o);
    r
}

/// Add the current executable's path to the dict
fn progname_to_dict(dict: *mut _object, key: *const i8) -> i32 {
    ((unsafe { PyDict_SetItemString(dict, key, core::ptr::addr_of_mut!(_Py_NoneStruct)) }) == 0)
        as i32
}

/// Add the runtime library's path to the dict
fn library_to_dict(dict: *mut _object, key: *const i8) -> i32 {
    // macOS framework builds do not link against a libpython dynamic library, but
    // instead link against a macOS Framework.
    ((unsafe { PyDict_SetItemString(dict, key, core::ptr::addr_of_mut!(_Py_NoneStruct)) }) == 0)
        as i32
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_Get_Getpath_CodeObject() -> *mut _object {
    unsafe {
        PyMarshal_ReadObjectFromString(
            (core::ptr::addr_of!(_Py_M__getpath.0) as *const u8) as *const i8,
            core::mem::size_of::<[u8; 14718]>() as i64,
        )
    }
}

/// Perform the actual path calculation.
///
/// When compute_path_config is 0, this only reads any initialised path
/// config values into the PyConfig struct. For example, Py_SetHome() or
/// Py_SetPath(). The only error should be due to failed memory allocation.
///
/// When compute_path_config is 1, full path calculation is performed.
/// The GIL must be held, and there may be filesystem access, side
/// effects, and potential unraisable errors that are reported directly
/// to stderr.
///
/// Calling this function multiple times on the same PyConfig is only
/// safe because already-configured values are not recalculated. To
/// actually recalculate paths, you need a clean PyConfig.
#[unsafe(no_mangle)]
extern "C-unwind" fn _PyConfig_InitPathConfig(
    config: *mut PyConfig,
    compute_path_config: i32,
) -> __SlateRecord_initconfig_h_19 {
    let mut __slate_compound_24: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let status: __SlateRecord_initconfig_h_19 = unsafe { _PyPathConfig_ReadGlobal(config) };
    if _PyStatus_EXCEPTION!(status) || compute_path_config == 0 {
        return status;
    }
    if _PyThreadState_GET().is_null() {
        return unsafe {
            PyStatus_Error(c"cannot calculate path configuration without GIL".as_ptr())
        };
    }
    let configDict: *mut _object = unsafe { _PyConfig_AsDict(config as *const PyConfig) };
    if configDict.is_null() {
        unsafe { PyErr_Clear() };
        return unsafe { PyStatus_NoMemory() };
    }
    let dict: *mut _object = unsafe { PyDict_New() };
    if dict.is_null() {
        unsafe { PyErr_Clear() };
        Py_DECREF!(configDict);
        return unsafe { PyStatus_NoMemory() };
    }
    if (unsafe { PyDict_SetItemString(dict, c"config".as_ptr(), configDict) }) < 0 {
        unsafe { PyErr_Clear() };
        Py_DECREF!(configDict);
        Py_DECREF!(dict);
        return unsafe { PyStatus_NoMemory() };
    }
    // reference now held by dict
    Py_DECREF!(configDict);
    let co: *mut _object = _Py_Get_Getpath_CodeObject();
    let __v0: bool = co.is_null() || PyCode_Check!(co) == 0;
    if __v0 {
        unsafe { PyErr_Clear() };
        Py_XDECREF!(co);
        Py_DECREF!(dict);
        return unsafe { PyStatus_Error(c"error reading frozen getpath.py".as_ptr()) };
    }
    let __v1: bool = decode_to_dict(dict, c"os_name".as_ptr(), c"posix".as_ptr()) == 0
        || int_to_dict(dict, c"WITH_NEXT_FRAMEWORK".as_ptr(), 0i32) == 0;
    let __v2: bool = __v1 || decode_to_dict(dict, c"PREFIX".as_ptr(), c"/usr/local".as_ptr()) == 0;
    let __v3: bool =
        __v2 || decode_to_dict(dict, c"EXEC_PREFIX".as_ptr(), c"/usr/local".as_ptr()) == 0;
    let __v4: bool = __v3 || decode_to_dict(dict, c"PYTHONPATH".as_ptr(), c"".as_ptr()) == 0;
    let __v5: bool = __v4
        || decode_to_dict(
            dict,
            c"VPATH".as_ptr(),
            c"/home/takashi/c-corpus/cpython".as_ptr(),
        ) == 0;
    let __v6: bool = __v5 || decode_to_dict(dict, c"PLATLIBDIR".as_ptr(), c"lib".as_ptr()) == 0;
    let __v7: bool =
        __v6 || decode_to_dict(dict, c"PYDEBUGEXT".as_ptr(), core::ptr::null::<i8>()) == 0;
    let __v8: bool = __v7 || int_to_dict(dict, c"VERSION_MAJOR".as_ptr(), 3i32) == 0;
    let __v9: bool = __v8 || int_to_dict(dict, c"VERSION_MINOR".as_ptr(), 14i32) == 0;
    let __v10: bool =
        __v9 || decode_to_dict(dict, c"PYWINVER".as_ptr(), core::ptr::null::<i8>()) == 0;
    let __v11: bool =
        __v10 || wchar_to_dict(dict, c"EXE_SUFFIX".as_ptr(), core::ptr::null::<i32>()) == 0;
    let __v12: bool = __v11 || env_to_dict(dict, c"ENV_PATH".as_ptr(), 0i32) == 0;
    let __v13: bool = __v12 || env_to_dict(dict, c"ENV_PYTHONHOME".as_ptr(), 0i32) == 0;
    let __v14: bool = __v13 || env_to_dict(dict, c"ENV_PYTHONEXECUTABLE".as_ptr(), 0i32) == 0;
    let __v15: bool = __v14 || env_to_dict(dict, c"ENV___PYVENV_LAUNCHER__".as_ptr(), 1i32) == 0;
    let __v16: bool = __v15 || progname_to_dict(dict, c"real_executable".as_ptr()) == 0;
    let __v17: bool = __v16 || library_to_dict(dict, c"library".as_ptr()) == 0;
    let __v18: bool =
        __v17 || wchar_to_dict(dict, c"executable_dir".as_ptr(), core::ptr::null::<i32>()) == 0;
    let __v19: bool = __v18
        || wchar_to_dict(dict, c"py_setpath".as_ptr(), unsafe {
            _PyPathConfig_GetGlobalModuleSearchPath()
        }) == 0;
    let __v20: bool = __v19 || funcs_to_dict(dict, unsafe { (*config).pathconfig_warnings }) == 0;
    let __v21: bool = __v20 || decode_to_dict(dict, c"ABI_THREAD".as_ptr(), c"".as_ptr()) == 0;
    let __v22: bool = __v21
        || (unsafe {
            PyDict_SetItemString(
                dict,
                c"winreg".as_ptr(),
                core::ptr::addr_of_mut!(_Py_NoneStruct),
            )
        }) < 0;
    let __v23: bool = __v22
        || (unsafe { PyDict_SetItemString(dict, c"__builtins__".as_ptr(), PyEval_GetBuiltins()) })
            < 0;
    if __v23 {
        Py_DECREF!(co);
        Py_DECREF!(dict);
        unsafe { PyErr_FormatUnraisable(c"Exception ignored while preparing getpath".as_ptr()) };
        return unsafe { PyStatus_Error(c"error evaluating initial values".as_ptr()) };
    }
    let r: *mut _object = unsafe { PyEval_EvalCode(co, dict, dict) };
    Py_DECREF!(co);
    if r.is_null() {
        Py_DECREF!(dict);
        unsafe { PyErr_FormatUnraisable(c"Exception ignored while running getpath".as_ptr()) };
        return unsafe { PyStatus_Error(c"error evaluating path".as_ptr()) };
    }
    Py_DECREF!(r);
    if (unsafe { _PyConfig_FromDict(config, configDict) }) < 0 {
        unsafe {
            PyErr_FormatUnraisable(c"Exception ignored while reading getpath results".as_ptr())
        };
        Py_DECREF!(dict);
        return unsafe { PyStatus_Error(c"error getting getpath results".as_ptr()) };
    }
    Py_DECREF!(dict);
    return unsafe {
        *{
            __slate_compound_24 = {
                let mut __t0: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                __t0._type = 0u32;
                __t0
            };
            core::ptr::addr_of_mut!(__slate_compound_24)
        }
    };
}
