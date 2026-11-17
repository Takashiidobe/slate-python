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
//! PyException_HEAD defines the initial segment of every exception class.
//! Cast argument to PyListObject* type.
//! A code object, the __code__ attribute
//! This limitation is for performance and simplicity. If needed it can be
//! removed (with effort).
//! Remote debugger support
//! PY_TIMEOUT_T is the integral type used to specify timeouts when waiting
//!
//! on a lock (see PyThread_acquire_lock_timed() below).
//! DECODE_LOCALE_ERR
//! Chosen so that _PyObjectStackChunk is a power-of-two size.
//! Largest tuple to save on freelist
//! leads to warnings... __attribute__((cdecl))
//! Various defines for all used memory orders in mimalloc
//! Main tuning parameters for segment and page sizes
//!
//! Sizes for 64-bit (usually divide by two for 32-bit)
//! Inlined definitions
//! Total tool ids available
//! This is used after getting NULL back from Py_DecodeLocale().
//! Almost all errors causing Python initialization to fail
//!
//! other fields are set to 0
// Signed variants of the above
// Prevent using an expression as a l-value.
//
// For example, "int x; _Py_RVALUE(x) = 1;" fails with a compiler error.
// 64KiB  (32KiB on 32-bit)
// 1024
// Used as a special value to encode block sizes in 32 bits.
// 35 == mi_segment_bin(MI_SLICES_PER_SEGMENT)
// initialize a local variable to zero; use memset as compilers optimize constant sized memset's
macro_rules! COPY_ATTR__4 {
    ($ATTR:ident, $config:expr, $cmdline:expr) => {
        if (unsafe { (*$config).$ATTR }) != 1i32.wrapping_neg() {
            unsafe {
                (*$cmdline).$ATTR = (*$config).$ATTR;
            }
        }
    };
}

macro_rules! COPY_ATTR__2 {
    ($ATTR:ident, $config:expr, $cmdline:expr) => {
        unsafe {
            (*$config).$ATTR = (*$cmdline).$ATTR;
        }
    };
}

macro_rules! COPY_ATTR__3 {
    ($ATTR:ident, $config:expr, $preconfig:expr) => {
        if (unsafe { (*$config).$ATTR }) != 1i32.wrapping_neg() {
            unsafe {
                (*$preconfig).$ATTR = (*$config).$ATTR;
            }
        }
    };
}

macro_rules! COPY_ATTR {
    ($ATTR:ident, $config:expr, $config2:expr) => {
        unsafe {
            (*$config).$ATTR = (*$config2).$ATTR;
        }
    };
}

macro_rules! COPY_FLAG__4 {
    ($ATTR:ident, $VALUE:expr, $config:expr) => {
        if (unsafe { (*$config).$ATTR }) < 0 {
            unsafe {
                (*$config).$ATTR = $VALUE;
            }
        }
    };
}

macro_rules! COPY_FLAG__3 {
    ($ATTR:ident, $VAR:expr, $config:expr) => {
        if (unsafe { (*$config).$ATTR }) >= 0 {
            unsafe {
                $VAR = (*$config).$ATTR;
            }
        }
    };
}

macro_rules! COPY_NOT_FLAG__3 {
    ($ATTR:ident, $VALUE:expr, $config:expr) => {
        if (unsafe { (*$config).$ATTR }) < 0 {
            unsafe {
                (*$config).$ATTR = (($VALUE) == 0) as i32;
            }
        }
    };
}

macro_rules! COPY_NOT_FLAG__4 {
    ($ATTR:ident, $VAR:expr, $config:expr) => {
        if (unsafe { (*$config).$ATTR }) >= 0 {
            unsafe {
                $VAR = ((*$config).$ATTR == 0) as i32;
            }
        }
    };
}

macro_rules! Py_DECREF {
    ($op:expr) => {
        Py_DECREF(_PyObject_CAST!($op));
    };
}

/// Cast argument to PyObject* type.
macro_rules! _PyObject_CAST {
    ($op:expr) => {
        _Py_CAST__2!($op)
    };
}

macro_rules! _PyStatus_EXCEPTION {
    ($err:expr) => {
        ($err._type as u32) != 0
    };
}

macro_rules! _PyStatus_GET_FUNC__8 {
    () => {
        c"_PyPreConfig_Read".as_ptr() as *mut i8
    };
}

macro_rules! _PyStatus_GET_FUNC__51 {
    () => {
        c"preconfig_init_allocator".as_ptr() as *mut i8
    };
}

macro_rules! _PyStatus_GET_FUNC__58 {
    () => {
        c"_PyPreConfig_Write".as_ptr() as *mut i8
    };
}

macro_rules! _PyStatus_GET_FUNC__5 {
    () => {
        c"_PyArgv_AsWstrList".as_ptr() as *mut i8
    };
}

macro_rules! _PyStatus_GET_FUNC__36 {
    () => {
        c"preconfig_init_utf8_mode".as_ptr() as *mut i8
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
    static mut Py_IgnoreEnvironmentFlag: i32;
    static mut Py_IsolatedFlag: i32;
    static mut Py_UTF8Mode: i32;
    static mut _PyOS_opterr: i32;
    static mut _PyOS_optarg: *const i32;
    static mut _PyRuntime: pyruntimestate;
    fn wcscmp(arg0: *const i32, arg1: *const i32) -> i32;
    fn wcsncmp(arg0: *const i32, arg1: *const i32, arg2: u64) -> i32;
    fn wcschr(arg0: *const i32, arg1: i32) -> *mut i32;
    #[link_name = "__isoc23_strtol"]
    fn strtol(arg0: *const i8, arg1: *mut *mut i8, arg2: i32) -> i64;
    fn PyMem_RawMalloc(arg0: u64) -> *mut ();
    fn PyMem_RawFree(arg0: *mut ());
    fn _Py_Dealloc(arg0: *mut _object);
    fn PyLong_FromLong(arg0: i64) -> *mut _object;
    fn PyDict_New() -> *mut _object;
    fn PyDict_SetItemString(arg0: *mut _object, arg1: *const i8, arg2: *mut _object) -> i32;
    fn PyWideStringList_Append(
        arg0: *mut __SlateRecord_initconfig_h_36,
        arg1: *const i32,
    ) -> __SlateRecord_initconfig_h_19;
    fn Py_DecodeLocale(arg0: *const i8, arg1: *mut u64) -> *mut i32;
    fn _PyMem_RawStrdup(arg0: *const i8) -> *mut i8;
    fn _PyMem_GetAllocatorName(arg0: *const i8, arg1: *mut u32) -> i32;
    fn _PyMem_SetupAllocators(arg0: u32) -> i32;
    fn _PyOS_ResetGetOpt();
    fn _PyOS_GetOpt(arg0: i64, arg1: *const *mut i32, arg2: *mut i32) -> i32;
    fn _PyWideStringList_Clear(arg0: *mut __SlateRecord_initconfig_h_36);
    fn _PyWideStringList_Copy(
        arg0: *mut __SlateRecord_initconfig_h_36,
        arg1: *const __SlateRecord_initconfig_h_36,
    ) -> i32;
    fn _PyWideStringList_Extend(
        arg0: *mut __SlateRecord_initconfig_h_36,
        arg1: *const __SlateRecord_initconfig_h_36,
    ) -> __SlateRecord_initconfig_h_19;
    fn _Py_CoerceLegacyLocale(arg0: i32) -> i32;
    fn _Py_LegacyLocaleDetected(arg0: i32) -> i32;
    fn _Py_SetLocaleFromEnv(arg0: i32) -> *mut i8;
    fn _PyRuntime_Initialize() -> __SlateRecord_initconfig_h_19;
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord___sigset_t_h_8 {
    __val: [u64; 16],
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
union __SlateRecord_pthreadtypes_h_45 {
    __size: [i8; 4],
    __align: i32,
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
struct __SlateRecord_pymem_h_48 {
    ctx: *mut (),
    malloc: Option<unsafe extern "C-unwind" fn(*mut (), u64) -> *mut ()>,
    calloc: Option<unsafe extern "C-unwind" fn(*mut (), u64, u64) -> *mut ()>,
    realloc: Option<unsafe extern "C-unwind" fn(*mut (), *mut (), u64) -> *mut ()>,
    free: Option<unsafe extern "C-unwind" fn(*mut (), *mut ())>,
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
struct _frame {}

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
struct __SlateRecord_objimpl_h_68 {
    ctx: *mut (),
    alloc: Option<unsafe extern "C-unwind" fn(*mut (), u64) -> *mut ()>,
    free: Option<unsafe extern "C-unwind" fn(*mut (), *mut (), u64)>,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_bytesobject_h_15 {
    ob_base: __SlateRecord_object_h_172,
    ob_shash: i64,
    ob_sval: [i8; 1],
}

// Macros and static inline functions, trading safety for speed
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_unicodeobject_h_163 {
    ob_base: _object,
    length: i64,
    hash: i64,
    state: __SlateRecord_unicodeobject_h_163_2,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_unicodeobject_h_173 {
    _base: __SlateRecord_unicodeobject_h_163,
    utf8_length: i64,
    utf8: *mut i8,
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
struct _PyLongValue {
    lv_tag: u64,
    ob_digit: [u32; 1],
}

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

// Cast argument to PyTupleObject* type.
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_unicodeobject_h_163_2 {
    __slate_bits_0: __slate_bits::__SlateBits137U0,
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
struct _PyExecutorObject {}

#[repr(C)]
#[derive(Clone, Copy)]
struct PyPreConfig {
    _config_init: i32,
    parse_argv: i32,
    isolated: i32,
    use_environment: i32,
    configure_locale: i32,
    coerce_c_locale: i32,
    coerce_c_locale_warn: i32,
    utf8_mode: i32,
    dev_mode: i32,
    allocator: i32,
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
struct __SlateRecord_pystate_h_78 {
    __slate_bits_0: __slate_bits::__SlateBits184U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _Py_tss_t {
    _is_initialized: i32,
    _key: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_modsupport_h_9 {
    v: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _PyArg_Parser {
    format: *const i8,
    keywords: *const *const i8,
    fname: *const i8,
    custom_msg: *const i8,
    once: __SlateRecord_modsupport_h_9,
    is_kwtuple_owned: i32,
    pos: i32,
    min: i32,
    max: i32,
    kwtuple: *mut _object,
    next: *mut _PyArg_Parser,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _inittab {
    name: *const i8,
    initfunc: Option<unsafe extern "C-unwind" fn() -> *mut _object>,
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
struct __SlateRecord_pycore_lock_h_189 {
    bits: u64,
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
struct __SlateRecord_pycore_structs_h_15 {
    value_and_backoff: u16,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord_pycore_structs_h_32 {
    cache: u16,
    op: __SlateRecord_pycore_structs_h_32_2,
    counter: __SlateRecord_pycore_structs_h_15,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_structs_h_38 {
    ob_base: _object,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_structs_h_32_2 {
    code: u8,
    arg: u8,
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
struct __SlateRecord_pycore_structs_h_53 {
    ob_base: __SlateRecord_object_h_172,
    b_bitmap: u32,
    b_array: [*mut _object; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_context_h_21 {
    ob_base: _object,
}

#[repr(C)]
#[derive(Clone, Copy)]
union _PyStackRef {
    bits: u64,
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
struct trampoline_api_st {
    init_state: Option<unsafe extern "C-unwind" fn() -> *mut ()>,
    write_state: Option<unsafe extern "C-unwind" fn(*mut (), *const (), u32, *mut PyCodeObject)>,
    free_state: Option<unsafe extern "C-unwind" fn(*mut ()) -> i32>,
    state: *mut (),
    code_padding: i64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _ceval_runtime_state {
    perf: __SlateRecord_pycore_interp_structs_h_81,
    pending_mainthread: _pending_calls,
    unused_sys_trace_profile_mutex: PyMutex,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct code_arena_st {}

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
struct _atexit_runtime_state {
    mutex: PyMutex,
    callbacks: [Option<unsafe extern "C-unwind" fn()>; 32],
    ncallbacks: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_interp_structs_h_81 {
    status: i32,
    perf_trampoline_type: i32,
    extra_code_index: i64,
    code_arena: *mut code_arena_st,
    trampoline_api: trampoline_api_st,
    map_file: *mut slate_support::FILE,
    persist_after_fork: i64,
    prev_eval_frame: Option<
        unsafe extern "C-unwind" fn(*mut _ts, *mut _PyInterpreterFrame, i32) -> *mut _object,
    >,
    trampoline_refcount: i64,
    code_watcher_id: i32,
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
struct _pythread_runtime_state {
    initialized: i32,
    _condattr_monotonic: __SlateRecord_pycore_pythread_h_64,
    handles: llist_node,
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
struct _import_runtime_state {
    inittab: *mut _inittab,
    last_module_index: i64,
    extensions: __SlateRecord_pycore_interp_structs_h_297,
    pkgcontext: *const i8,
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
struct __SlateRecord_pycore_pythread_h_64 {
    ptr: *mut __SlateRecord_pthreadtypes_h_45,
    val: __SlateRecord_pthreadtypes_h_45,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _Py_slist_item_s {
    next: *mut _Py_slist_item_s,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_interp_structs_h_297 {
    mutex: PyMutex,
    hashtable: *mut _Py_hashtable_t,
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
struct __SlateRecord_pycore_interp_structs_h_348 {
    import_level: i32,
    accumulated: i64,
    header: i32,
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
struct _reftracer_runtime_state {
    tracer_func: Option<unsafe extern "C-unwind" fn(*mut _object, u32, *mut ()) -> i32>,
    tracer_data: *mut (),
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _py_object_runtime_state {
    _not_used: i32,
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
struct __SlateRecord_pycore_crossinterp_h_255 {
    data_lookup: _xid_lookup_state,
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
struct xi_exceptions {
    PyExc_InterpreterError: *mut _object,
    PyExc_InterpreterNotFoundError: *mut _object,
    PyExc_NotShareableError: *mut _object,
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
struct _types_runtime_state {
    next_version_tag: u32,
    managed_static: __SlateRecord_pycore_interp_structs_h_504,
}

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
struct __SlateRecord_pycore_interp_structs_h_504 {
    types: [__SlateRecord_pycore_interp_structs_h_505; 210],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_interp_structs_h_505 {
    r#type: *mut _typeobject,
    interp_count: i64,
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
struct _Py_mem_interp_free_queue {
    has_work: i32,
    mutex: PyMutex,
    head: llist_node,
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
struct _Py_unicode_runtime_ids {
    mutex: PyMutex,
    next_index: i64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _Py_unicode_runtime_state {
    ids: _Py_unicode_runtime_ids,
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
struct _Py_GlobalMonitors {
    tools: [u8; 16],
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
struct _PyArgv {
    argc: i64,
    use_bytes_argv: i32,
    bytes_argv: *const *mut i8,
    wchar_argv: *const *mut i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_initconfig_h_108 {
    argv: __SlateRecord_initconfig_h_36,
    xoptions: __SlateRecord_initconfig_h_36,
    isolated: i32,
    use_environment: i32,
    dev_mode: i32,
    warn_default_encoding: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct pool_header {
    r#ref: __SlateRecord_pycore_obmalloc_h_255,
    freeblock: *mut u8,
    nextpool: *mut pool_header,
    prevpool: *mut pool_header,
    arenaindex: u32,
    szidx: u32,
    nextoffset: u32,
    maxnextoffset: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct arena_object {
    address: u64,
    pool_address: *mut u8,
    nfreepools: u32,
    ntotalpools: u32,
    freepools: *mut pool_header,
    nextarena: *mut arena_object,
    prevarena: *mut arena_object,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord_pycore_obmalloc_h_255 {
    _padding: *mut u8,
    count: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _obmalloc_pools {
    used: [*mut pool_header; 64],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _obmalloc_mgmt {
    arenas: *mut arena_object,
    maxarenas: u32,
    unused_arena_objects: *mut arena_object,
    usable_arenas: *mut arena_object,
    nfp2lasta: [*mut arena_object; 65],
    narenas_currently_allocated: u64,
    ntimes_arena_allocated: u64,
    narenas_highwater: u64,
    raw_allocated_blocks: i64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_obmalloc_h_622 {
    tail_hi: i32,
    tail_lo: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct arena_map_bot {
    arenas: [__SlateRecord_pycore_obmalloc_h_622; 16384],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct arena_map_mid {
    ptrs: [*mut arena_map_bot; 32768],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct arena_map_top {
    ptrs: [*mut arena_map_mid; 32768],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _obmalloc_usage {
    arena_map_root: arena_map_top,
    arena_map_mid_count: i32,
    arena_map_bot_count: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _obmalloc_global_state {
    dump_debug_stats: i32,
    interpreter_leaks: i64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _obmalloc_state {
    pools: _obmalloc_pools,
    mgmt: _obmalloc_mgmt,
    usage: _obmalloc_usage,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_runtime_structs_h_19 {
    api_id: i8,
    alloc: __SlateRecord_pymem_h_48,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _pymem_allocators {
    mutex: PyMutex,
    standard: __SlateRecord_pycore_runtime_structs_h_23,
    debug: __SlateRecord_pycore_runtime_structs_h_28,
    is_debug_enabled: i32,
    obj_arena: __SlateRecord_objimpl_h_68,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_runtime_structs_h_23 {
    raw: __SlateRecord_pymem_h_48,
    mem: __SlateRecord_pymem_h_48,
    obj: __SlateRecord_pymem_h_48,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_runtime_structs_h_28 {
    raw: __SlateRecord_pycore_runtime_structs_h_19,
    mem: __SlateRecord_pycore_runtime_structs_h_19,
    obj: __SlateRecord_pycore_runtime_structs_h_19,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _Py_float_runtime_state {
    float_format: u32,
    double_format: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct pyhash_runtime_state {
    urandom_cache: __SlateRecord_pycore_runtime_structs_h_49,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_runtime_structs_h_49 {
    fd: i32,
    st_dev: u64,
    st_ino: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _PyTraceMalloc_Config {
    initialized: u32,
    tracing: i32,
    max_nframe: i32,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
struct tracemalloc_frame {
    filename: *mut _object,
    lineno: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct tracemalloc_traceback {
    hash: u64,
    nframe: u16,
    total_nframe: u16,
    frames: [tracemalloc_frame; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _tracemalloc_runtime_state {
    config: _PyTraceMalloc_Config,
    allocators: __SlateRecord_pycore_tracemalloc_h_70,
    tables_lock: PyMutex,
    traced_memory: u64,
    peak_traced_memory: u64,
    filenames: *mut _Py_hashtable_t,
    traceback: *mut tracemalloc_traceback,
    tracebacks: *mut _Py_hashtable_t,
    traces: *mut _Py_hashtable_t,
    domains: *mut _Py_hashtable_t,
    empty_traceback: tracemalloc_traceback,
    reentrant_key: _Py_tss_t,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_tracemalloc_h_70 {
    mem: __SlateRecord_pymem_h_48,
    raw: __SlateRecord_pymem_h_48,
    obj: __SlateRecord_pymem_h_48,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _fileutils_state {
    force_ascii: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _PyInterpreterFrame {
    f_executable: _PyStackRef,
    previous: *mut _PyInterpreterFrame,
    f_funcobj: _PyStackRef,
    f_globals: *mut _object,
    f_builtins: *mut _object,
    f_locals: *mut _object,
    frame_obj: *mut _frame,
    instr_ptr: *mut __SlateRecord_pycore_structs_h_32,
    stackpointer: *mut _PyStackRef,
    return_offset: u16,
    owner: i8,
    visited: u8,
    localsplus: [_PyStackRef; 1],
}

// _PyGenObject_HEAD defines the initial segment of generator
// and coroutine objects.
// List of weak reference.
// Name of the generator.
// Qualified name of the generator.
// The frame
#[repr(C)]
#[derive(Clone, Copy)]
struct _runtime_state {
    size: u64,
    finalizing: u64,
    interpreters_head: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _interpreter_state {
    size: u64,
    id: u64,
    next: u64,
    threads_head: u64,
    threads_main: u64,
    gc: u64,
    imports_modules: u64,
    sysdict: u64,
    builtins: u64,
    ceval_gil: u64,
    gil_runtime_state: u64,
    gil_runtime_state_enabled: u64,
    gil_runtime_state_locked: u64,
    gil_runtime_state_holder: u64,
    code_object_generation: u64,
    tlbc_generation: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _thread_state {
    size: u64,
    prev: u64,
    next: u64,
    interp: u64,
    current_frame: u64,
    thread_id: u64,
    native_thread_id: u64,
    datastack_chunk: u64,
    status: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _interpreter_frame {
    size: u64,
    previous: u64,
    executable: u64,
    instr_ptr: u64,
    localsplus: u64,
    owner: u64,
    stackpointer: u64,
    tlbc_index: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _code_object {
    size: u64,
    filename: u64,
    name: u64,
    qualname: u64,
    linetable: u64,
    firstlineno: u64,
    argcount: u64,
    localsplusnames: u64,
    localspluskinds: u64,
    co_code_adaptive: u64,
    co_tlbc: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _pyobject {
    size: u64,
    ob_type: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _type_object {
    size: u64,
    tp_name: u64,
    tp_repr: u64,
    tp_flags: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _tuple_object {
    size: u64,
    ob_item: u64,
    ob_size: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _list_object {
    size: u64,
    ob_item: u64,
    ob_size: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _set_object {
    size: u64,
    used: u64,
    table: u64,
    mask: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _dict_object {
    size: u64,
    ma_keys: u64,
    ma_values: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _float_object {
    size: u64,
    ob_fval: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _long_object {
    size: u64,
    lv_tag: u64,
    ob_digit: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _bytes_object {
    size: u64,
    ob_size: u64,
    ob_sval: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _unicode_object {
    size: u64,
    state: u64,
    length: u64,
    asciiobject_size: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _gc {
    size: u64,
    collecting: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _gen_object {
    size: u64,
    gi_name: u64,
    gi_iframe: u64,
    gi_frame_state: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _llist_node {
    next: u64,
    prev: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _debugger_support {
    eval_breaker: u64,
    remote_debugger_support: u64,
    remote_debugging_enabled: u64,
    debugger_pending_call: u64,
    debugger_script_path: u64,
    debugger_script_path_size: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _Py_DebugOffsets {
    cookie: [i8; 8],
    version: u64,
    free_threaded: u64,
    runtime_state: _runtime_state,
    interpreter_state: _interpreter_state,
    thread_state: _thread_state,
    interpreter_frame: _interpreter_frame,
    code_object: _code_object,
    pyobject: _pyobject,
    type_object: _type_object,
    tuple_object: _tuple_object,
    list_object: _list_object,
    set_object: _set_object,
    dict_object: _dict_object,
    float_object: _float_object,
    long_object: _long_object,
    bytes_object: _bytes_object,
    unicode_object: _unicode_object,
    gc: _gc,
    gen_object: _gen_object,
    llist_node: _llist_node,
    debugger_support: _debugger_support,
}

#[repr(C)]
#[derive(Clone, Copy)]
union sigval {
    sival_int: i32,
    sival_ptr: *mut (),
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord_siginfo_t_h_124_2 {
    _pad: [i32; 28],
    _kill: __SlateRecord_siginfo_t_h_124_3,
    _timer: __SlateRecord_siginfo_t_h_124_4,
    _rt: __SlateRecord_siginfo_t_h_124_5,
    _sigchld: __SlateRecord_siginfo_t_h_124_6,
    _sigfault: __SlateRecord_siginfo_t_h_124_7,
    _sigpoll: __SlateRecord_siginfo_t_h_124_10,
    _sigsys: __SlateRecord_siginfo_t_h_124_11,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_siginfo_t_h_124_3 {
    si_pid: i32,
    si_uid: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_siginfo_t_h_124_4 {
    si_tid: i32,
    si_overrun: i32,
    si_sigval: sigval,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_siginfo_t_h_124_5 {
    si_pid: i32,
    si_uid: u32,
    si_sigval: sigval,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_siginfo_t_h_124_6 {
    si_pid: i32,
    si_uid: u32,
    si_status: i32,
    si_utime: i64,
    si_stime: i64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_siginfo_t_h_124_7 {
    si_addr: *mut (),
    si_addr_lsb: i16,
    _bounds: __SlateRecord_siginfo_t_h_124_8,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord_siginfo_t_h_124_8 {
    _addr_bnd: __SlateRecord_siginfo_t_h_124_9,
    _pkey: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_siginfo_t_h_124_9 {
    _lower: *mut (),
    _upper: *mut (),
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_siginfo_t_h_124_10 {
    si_band: i64,
    si_fd: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_siginfo_t_h_124_11 {
    _call_addr: *mut (),
    _syscall: i32,
    _arch: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_siginfo_t_h_124 {
    si_signo: i32,
    si_errno: i32,
    si_code: i32,
    __pad0: i32,
    _sifields: __SlateRecord_siginfo_t_h_124_2,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord_sigaction_h_31 {
    sa_handler: Option<unsafe extern "C-unwind" fn(i32)>,
    sa_sigaction:
        Option<unsafe extern "C-unwind" fn(i32, *mut __SlateRecord_siginfo_t_h_124, *mut ())>,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct sigaction {
    __sigaction_handler: __SlateRecord_sigaction_h_31,
    sa_mask: __SlateRecord___sigset_t_h_8,
    sa_flags: i32,
    sa_restorer: Option<unsafe extern "C-unwind" fn()>,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_stack_t_h_31 {
    ss_sp: *mut (),
    ss_flags: i32,
    ss_size: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_signal_h_40 {
    tripped: i32,
    func: *mut _object,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_signal_h_46 {
    fd: i32,
    warn_on_full_buffer: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _signals_runtime_state {
    handlers: [__SlateRecord_pycore_signal_h_40; 65],
    wakeup: __SlateRecord_pycore_signal_h_46,
    is_tripped: i32,
    default_handler: *mut _object,
    ignore_handler: *mut _object,
    unhandled_keyboard_interrupt: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct faulthandler_user_signal {
    enabled: i32,
    file: *mut _object,
    fd: i32,
    all_threads: i32,
    chain: i32,
    previous: sigaction,
    interp: *mut _is,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_faulthandler_h_50 {
    enabled: i32,
    file: *mut _object,
    fd: i32,
    all_threads: i32,
    interp: *mut _is,
    c_stack: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_faulthandler_h_62 {
    file: *mut _object,
    fd: i32,
    timeout_us: i64,
    repeat: i32,
    interp: *mut _is,
    exit: i32,
    header: *mut i8,
    header_len: u64,
    cancel_event: *mut (),
    running: *mut (),
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _faulthandler_runtime_state {
    fatal_error: __SlateRecord_pycore_faulthandler_h_50,
    thread: __SlateRecord_pycore_faulthandler_h_62,
    user_signals: *mut faulthandler_user_signal,
    stack: __SlateRecord_stack_t_h_31,
    old_stack: __SlateRecord_stack_t_h_31,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_asdl_h_45 {
    size: i64,
    elements: *mut *mut (),
    typed_elements: [i32; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_74 {
    size: i64,
    elements: *mut *mut (),
    typed_elements: [*mut _expr; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_81 {
    size: i64,
    elements: *mut *mut (),
    typed_elements: [*mut _comprehension; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_104 {
    size: i64,
    elements: *mut *mut (),
    typed_elements: [*mut _arg; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_111 {
    size: i64,
    elements: *mut *mut (),
    typed_elements: [*mut _keyword; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _expr {
    kind: u32,
    v: __SlateRecord_pycore_ast_h_370,
    lineno: i32,
    col_offset: i32,
    end_lineno: i32,
    end_col_offset: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _comprehension {
    target: *mut _expr,
    iter: *mut _expr,
    ifs: *mut __SlateRecord_pycore_ast_h_74,
    is_async: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _arguments {
    posonlyargs: *mut __SlateRecord_pycore_ast_h_104,
    args: *mut __SlateRecord_pycore_ast_h_104,
    vararg: *mut _arg,
    kwonlyargs: *mut __SlateRecord_pycore_ast_h_104,
    kw_defaults: *mut __SlateRecord_pycore_ast_h_74,
    kwarg: *mut _arg,
    defaults: *mut __SlateRecord_pycore_ast_h_74,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _arg {
    arg: *mut _object,
    annotation: *mut _expr,
    type_comment: *mut _object,
    lineno: i32,
    col_offset: i32,
    end_lineno: i32,
    end_col_offset: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord_pycore_ast_h_370 {
    BoolOp: __SlateRecord_pycore_ast_h_371,
    NamedExpr: __SlateRecord_pycore_ast_h_376,
    BinOp: __SlateRecord_pycore_ast_h_381,
    UnaryOp: __SlateRecord_pycore_ast_h_387,
    Lambda: __SlateRecord_pycore_ast_h_392,
    IfExp: __SlateRecord_pycore_ast_h_397,
    Dict: __SlateRecord_pycore_ast_h_403,
    Set: __SlateRecord_pycore_ast_h_408,
    ListComp: __SlateRecord_pycore_ast_h_412,
    SetComp: __SlateRecord_pycore_ast_h_417,
    DictComp: __SlateRecord_pycore_ast_h_422,
    GeneratorExp: __SlateRecord_pycore_ast_h_428,
    Await: __SlateRecord_pycore_ast_h_433,
    Yield: __SlateRecord_pycore_ast_h_437,
    YieldFrom: __SlateRecord_pycore_ast_h_441,
    Compare: __SlateRecord_pycore_ast_h_445,
    Call: __SlateRecord_pycore_ast_h_451,
    FormattedValue: __SlateRecord_pycore_ast_h_457,
    Interpolation: __SlateRecord_pycore_ast_h_463,
    JoinedStr: __SlateRecord_pycore_ast_h_470,
    TemplateStr: __SlateRecord_pycore_ast_h_474,
    Constant: __SlateRecord_pycore_ast_h_478,
    Attribute: __SlateRecord_pycore_ast_h_483,
    Subscript: __SlateRecord_pycore_ast_h_489,
    Starred: __SlateRecord_pycore_ast_h_495,
    Name: __SlateRecord_pycore_ast_h_500,
    List: __SlateRecord_pycore_ast_h_505,
    Tuple: __SlateRecord_pycore_ast_h_510,
    Slice: __SlateRecord_pycore_ast_h_515,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_371 {
    op: u32,
    values: *mut __SlateRecord_pycore_ast_h_74,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_376 {
    target: *mut _expr,
    value: *mut _expr,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_381 {
    left: *mut _expr,
    op: u32,
    right: *mut _expr,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_387 {
    op: u32,
    operand: *mut _expr,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_392 {
    args: *mut _arguments,
    body: *mut _expr,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_397 {
    test: *mut _expr,
    body: *mut _expr,
    orelse: *mut _expr,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_403 {
    keys: *mut __SlateRecord_pycore_ast_h_74,
    values: *mut __SlateRecord_pycore_ast_h_74,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_408 {
    elts: *mut __SlateRecord_pycore_ast_h_74,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_412 {
    elt: *mut _expr,
    generators: *mut __SlateRecord_pycore_ast_h_81,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_417 {
    elt: *mut _expr,
    generators: *mut __SlateRecord_pycore_ast_h_81,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_422 {
    key: *mut _expr,
    value: *mut _expr,
    generators: *mut __SlateRecord_pycore_ast_h_81,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_428 {
    elt: *mut _expr,
    generators: *mut __SlateRecord_pycore_ast_h_81,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_433 {
    value: *mut _expr,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_437 {
    value: *mut _expr,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_441 {
    value: *mut _expr,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_445 {
    left: *mut _expr,
    ops: *mut __SlateRecord_pycore_asdl_h_45,
    comparators: *mut __SlateRecord_pycore_ast_h_74,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_451 {
    func: *mut _expr,
    args: *mut __SlateRecord_pycore_ast_h_74,
    keywords: *mut __SlateRecord_pycore_ast_h_111,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_457 {
    value: *mut _expr,
    conversion: i32,
    format_spec: *mut _expr,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_463 {
    value: *mut _expr,
    str: *mut _object,
    conversion: i32,
    format_spec: *mut _expr,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_470 {
    values: *mut __SlateRecord_pycore_ast_h_74,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_474 {
    values: *mut __SlateRecord_pycore_ast_h_74,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_478 {
    value: *mut _object,
    kind: *mut _object,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_483 {
    value: *mut _expr,
    attr: *mut _object,
    ctx: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_489 {
    value: *mut _expr,
    slice: *mut _expr,
    ctx: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_495 {
    value: *mut _expr,
    ctx: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_500 {
    id: *mut _object,
    ctx: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_505 {
    elts: *mut __SlateRecord_pycore_ast_h_74,
    ctx: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_510 {
    elts: *mut __SlateRecord_pycore_ast_h_74,
    ctx: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_ast_h_515 {
    lower: *mut _expr,
    upper: *mut _expr,
    step: *mut _expr,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _keyword {
    arg: *mut _object,
    value: *mut _expr,
    lineno: i32,
    col_offset: i32,
    end_lineno: i32,
    end_col_offset: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _parser_runtime_state {
    _not_used: i32,
    dummy_name: _expr,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _Py_time_runtime_state {
    _unused: i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _Py_cached_objects {
    interned_strings: *mut _Py_hashtable_t,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_32 {
    _py_anon_dictcomp: __SlateRecord_pycore_global_strings_h_33,
    _py_anon_genexpr: __SlateRecord_pycore_global_strings_h_34,
    _py_anon_lambda: __SlateRecord_pycore_global_strings_h_35,
    _py_anon_listcomp: __SlateRecord_pycore_global_strings_h_36,
    _py_anon_module: __SlateRecord_pycore_global_strings_h_37,
    _py_anon_null: __SlateRecord_pycore_global_strings_h_38,
    _py_anon_setcomp: __SlateRecord_pycore_global_strings_h_39,
    _py_anon_string: __SlateRecord_pycore_global_strings_h_40,
    _py_anon_unknown: __SlateRecord_pycore_global_strings_h_41,
    _py_dbl_close_br: __SlateRecord_pycore_global_strings_h_42,
    _py_dbl_open_br: __SlateRecord_pycore_global_strings_h_43,
    _py_dbl_percent: __SlateRecord_pycore_global_strings_h_44,
    _py_defaults: __SlateRecord_pycore_global_strings_h_45,
    _py_dot_locals: __SlateRecord_pycore_global_strings_h_46,
    _py_empty: __SlateRecord_pycore_global_strings_h_47,
    _py_format: __SlateRecord_pycore_global_strings_h_48,
    _py_generic_base: __SlateRecord_pycore_global_strings_h_49,
    _py_json_decoder: __SlateRecord_pycore_global_strings_h_50,
    _py_kwdefaults: __SlateRecord_pycore_global_strings_h_51,
    _py_list_err: __SlateRecord_pycore_global_strings_h_52,
    _py_str_replace_inf: __SlateRecord_pycore_global_strings_h_53,
    _py_type_params: __SlateRecord_pycore_global_strings_h_54,
    _py_utf_8: __SlateRecord_pycore_global_strings_h_55,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_33 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_34 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_35 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_36 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_37 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_38 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_39 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_40 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_41 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_42 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 3],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_43 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 3],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_44 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 3],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_45 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_46 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_47 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_48 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_49 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 14],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_50 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 13],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_51 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_52 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 24],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_53 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_54 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 13],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_55 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_58 {
    _py_CANCELLED: __SlateRecord_pycore_global_strings_h_59,
    _py_FINISHED: __SlateRecord_pycore_global_strings_h_60,
    _py_False: __SlateRecord_pycore_global_strings_h_61,
    _py_JSONDecodeError: __SlateRecord_pycore_global_strings_h_62,
    _py_PENDING: __SlateRecord_pycore_global_strings_h_63,
    _py_Py_Repr: __SlateRecord_pycore_global_strings_h_64,
    _py_TextIOWrapper: __SlateRecord_pycore_global_strings_h_65,
    _py_True: __SlateRecord_pycore_global_strings_h_66,
    _py_WarningMessage: __SlateRecord_pycore_global_strings_h_67,
    _py__WindowsConsoleIO: __SlateRecord_pycore_global_strings_h_68,
    _py___IOBase_closed: __SlateRecord_pycore_global_strings_h_69,
    _py___abc_tpflags__: __SlateRecord_pycore_global_strings_h_70,
    _py___abs__: __SlateRecord_pycore_global_strings_h_71,
    _py___abstractmethods__: __SlateRecord_pycore_global_strings_h_72,
    _py___add__: __SlateRecord_pycore_global_strings_h_73,
    _py___aenter__: __SlateRecord_pycore_global_strings_h_74,
    _py___aexit__: __SlateRecord_pycore_global_strings_h_75,
    _py___aiter__: __SlateRecord_pycore_global_strings_h_76,
    _py___all__: __SlateRecord_pycore_global_strings_h_77,
    _py___and__: __SlateRecord_pycore_global_strings_h_78,
    _py___anext__: __SlateRecord_pycore_global_strings_h_79,
    _py___annotate__: __SlateRecord_pycore_global_strings_h_80,
    _py___annotate_func__: __SlateRecord_pycore_global_strings_h_81,
    _py___annotations__: __SlateRecord_pycore_global_strings_h_82,
    _py___annotations_cache__: __SlateRecord_pycore_global_strings_h_83,
    _py___args__: __SlateRecord_pycore_global_strings_h_84,
    _py___await__: __SlateRecord_pycore_global_strings_h_85,
    _py___bases__: __SlateRecord_pycore_global_strings_h_86,
    _py___bool__: __SlateRecord_pycore_global_strings_h_87,
    _py___buffer__: __SlateRecord_pycore_global_strings_h_88,
    _py___build_class__: __SlateRecord_pycore_global_strings_h_89,
    _py___builtins__: __SlateRecord_pycore_global_strings_h_90,
    _py___bytes__: __SlateRecord_pycore_global_strings_h_91,
    _py___call__: __SlateRecord_pycore_global_strings_h_92,
    _py___cantrace__: __SlateRecord_pycore_global_strings_h_93,
    _py___ceil__: __SlateRecord_pycore_global_strings_h_94,
    _py___class__: __SlateRecord_pycore_global_strings_h_95,
    _py___class_getitem__: __SlateRecord_pycore_global_strings_h_96,
    _py___classcell__: __SlateRecord_pycore_global_strings_h_97,
    _py___classdict__: __SlateRecord_pycore_global_strings_h_98,
    _py___classdictcell__: __SlateRecord_pycore_global_strings_h_99,
    _py___complex__: __SlateRecord_pycore_global_strings_h_100,
    _py___conditional_annotations__: __SlateRecord_pycore_global_strings_h_101,
    _py___contains__: __SlateRecord_pycore_global_strings_h_102,
    _py___ctypes_from_outparam__: __SlateRecord_pycore_global_strings_h_103,
    _py___del__: __SlateRecord_pycore_global_strings_h_104,
    _py___delattr__: __SlateRecord_pycore_global_strings_h_105,
    _py___delete__: __SlateRecord_pycore_global_strings_h_106,
    _py___delitem__: __SlateRecord_pycore_global_strings_h_107,
    _py___dict__: __SlateRecord_pycore_global_strings_h_108,
    _py___dictoffset__: __SlateRecord_pycore_global_strings_h_109,
    _py___dir__: __SlateRecord_pycore_global_strings_h_110,
    _py___divmod__: __SlateRecord_pycore_global_strings_h_111,
    _py___doc__: __SlateRecord_pycore_global_strings_h_112,
    _py___enter__: __SlateRecord_pycore_global_strings_h_113,
    _py___eq__: __SlateRecord_pycore_global_strings_h_114,
    _py___exit__: __SlateRecord_pycore_global_strings_h_115,
    _py___file__: __SlateRecord_pycore_global_strings_h_116,
    _py___firstlineno__: __SlateRecord_pycore_global_strings_h_117,
    _py___float__: __SlateRecord_pycore_global_strings_h_118,
    _py___floor__: __SlateRecord_pycore_global_strings_h_119,
    _py___floordiv__: __SlateRecord_pycore_global_strings_h_120,
    _py___format__: __SlateRecord_pycore_global_strings_h_121,
    _py___fspath__: __SlateRecord_pycore_global_strings_h_122,
    _py___ge__: __SlateRecord_pycore_global_strings_h_123,
    _py___get__: __SlateRecord_pycore_global_strings_h_124,
    _py___getattr__: __SlateRecord_pycore_global_strings_h_125,
    _py___getattribute__: __SlateRecord_pycore_global_strings_h_126,
    _py___getinitargs__: __SlateRecord_pycore_global_strings_h_127,
    _py___getitem__: __SlateRecord_pycore_global_strings_h_128,
    _py___getnewargs__: __SlateRecord_pycore_global_strings_h_129,
    _py___getnewargs_ex__: __SlateRecord_pycore_global_strings_h_130,
    _py___getstate__: __SlateRecord_pycore_global_strings_h_131,
    _py___gt__: __SlateRecord_pycore_global_strings_h_132,
    _py___hash__: __SlateRecord_pycore_global_strings_h_133,
    _py___iadd__: __SlateRecord_pycore_global_strings_h_134,
    _py___iand__: __SlateRecord_pycore_global_strings_h_135,
    _py___ifloordiv__: __SlateRecord_pycore_global_strings_h_136,
    _py___ilshift__: __SlateRecord_pycore_global_strings_h_137,
    _py___imatmul__: __SlateRecord_pycore_global_strings_h_138,
    _py___imod__: __SlateRecord_pycore_global_strings_h_139,
    _py___import__: __SlateRecord_pycore_global_strings_h_140,
    _py___imul__: __SlateRecord_pycore_global_strings_h_141,
    _py___index__: __SlateRecord_pycore_global_strings_h_142,
    _py___init__: __SlateRecord_pycore_global_strings_h_143,
    _py___init_subclass__: __SlateRecord_pycore_global_strings_h_144,
    _py___instancecheck__: __SlateRecord_pycore_global_strings_h_145,
    _py___int__: __SlateRecord_pycore_global_strings_h_146,
    _py___invert__: __SlateRecord_pycore_global_strings_h_147,
    _py___ior__: __SlateRecord_pycore_global_strings_h_148,
    _py___ipow__: __SlateRecord_pycore_global_strings_h_149,
    _py___irshift__: __SlateRecord_pycore_global_strings_h_150,
    _py___isabstractmethod__: __SlateRecord_pycore_global_strings_h_151,
    _py___isub__: __SlateRecord_pycore_global_strings_h_152,
    _py___iter__: __SlateRecord_pycore_global_strings_h_153,
    _py___itruediv__: __SlateRecord_pycore_global_strings_h_154,
    _py___ixor__: __SlateRecord_pycore_global_strings_h_155,
    _py___le__: __SlateRecord_pycore_global_strings_h_156,
    _py___len__: __SlateRecord_pycore_global_strings_h_157,
    _py___length_hint__: __SlateRecord_pycore_global_strings_h_158,
    _py___lltrace__: __SlateRecord_pycore_global_strings_h_159,
    _py___loader__: __SlateRecord_pycore_global_strings_h_160,
    _py___lshift__: __SlateRecord_pycore_global_strings_h_161,
    _py___lt__: __SlateRecord_pycore_global_strings_h_162,
    _py___main__: __SlateRecord_pycore_global_strings_h_163,
    _py___match_args__: __SlateRecord_pycore_global_strings_h_164,
    _py___matmul__: __SlateRecord_pycore_global_strings_h_165,
    _py___missing__: __SlateRecord_pycore_global_strings_h_166,
    _py___mod__: __SlateRecord_pycore_global_strings_h_167,
    _py___module__: __SlateRecord_pycore_global_strings_h_168,
    _py___mro_entries__: __SlateRecord_pycore_global_strings_h_169,
    _py___mul__: __SlateRecord_pycore_global_strings_h_170,
    _py___name__: __SlateRecord_pycore_global_strings_h_171,
    _py___ne__: __SlateRecord_pycore_global_strings_h_172,
    _py___neg__: __SlateRecord_pycore_global_strings_h_173,
    _py___new__: __SlateRecord_pycore_global_strings_h_174,
    _py___newobj__: __SlateRecord_pycore_global_strings_h_175,
    _py___newobj_ex__: __SlateRecord_pycore_global_strings_h_176,
    _py___next__: __SlateRecord_pycore_global_strings_h_177,
    _py___notes__: __SlateRecord_pycore_global_strings_h_178,
    _py___or__: __SlateRecord_pycore_global_strings_h_179,
    _py___orig_class__: __SlateRecord_pycore_global_strings_h_180,
    _py___origin__: __SlateRecord_pycore_global_strings_h_181,
    _py___package__: __SlateRecord_pycore_global_strings_h_182,
    _py___parameters__: __SlateRecord_pycore_global_strings_h_183,
    _py___path__: __SlateRecord_pycore_global_strings_h_184,
    _py___pos__: __SlateRecord_pycore_global_strings_h_185,
    _py___pow__: __SlateRecord_pycore_global_strings_h_186,
    _py___prepare__: __SlateRecord_pycore_global_strings_h_187,
    _py___qualname__: __SlateRecord_pycore_global_strings_h_188,
    _py___radd__: __SlateRecord_pycore_global_strings_h_189,
    _py___rand__: __SlateRecord_pycore_global_strings_h_190,
    _py___rdivmod__: __SlateRecord_pycore_global_strings_h_191,
    _py___reduce__: __SlateRecord_pycore_global_strings_h_192,
    _py___reduce_ex__: __SlateRecord_pycore_global_strings_h_193,
    _py___release_buffer__: __SlateRecord_pycore_global_strings_h_194,
    _py___repr__: __SlateRecord_pycore_global_strings_h_195,
    _py___reversed__: __SlateRecord_pycore_global_strings_h_196,
    _py___rfloordiv__: __SlateRecord_pycore_global_strings_h_197,
    _py___rlshift__: __SlateRecord_pycore_global_strings_h_198,
    _py___rmatmul__: __SlateRecord_pycore_global_strings_h_199,
    _py___rmod__: __SlateRecord_pycore_global_strings_h_200,
    _py___rmul__: __SlateRecord_pycore_global_strings_h_201,
    _py___ror__: __SlateRecord_pycore_global_strings_h_202,
    _py___round__: __SlateRecord_pycore_global_strings_h_203,
    _py___rpow__: __SlateRecord_pycore_global_strings_h_204,
    _py___rrshift__: __SlateRecord_pycore_global_strings_h_205,
    _py___rshift__: __SlateRecord_pycore_global_strings_h_206,
    _py___rsub__: __SlateRecord_pycore_global_strings_h_207,
    _py___rtruediv__: __SlateRecord_pycore_global_strings_h_208,
    _py___rxor__: __SlateRecord_pycore_global_strings_h_209,
    _py___set__: __SlateRecord_pycore_global_strings_h_210,
    _py___set_name__: __SlateRecord_pycore_global_strings_h_211,
    _py___setattr__: __SlateRecord_pycore_global_strings_h_212,
    _py___setitem__: __SlateRecord_pycore_global_strings_h_213,
    _py___setstate__: __SlateRecord_pycore_global_strings_h_214,
    _py___sizeof__: __SlateRecord_pycore_global_strings_h_215,
    _py___slotnames__: __SlateRecord_pycore_global_strings_h_216,
    _py___slots__: __SlateRecord_pycore_global_strings_h_217,
    _py___spec__: __SlateRecord_pycore_global_strings_h_218,
    _py___static_attributes__: __SlateRecord_pycore_global_strings_h_219,
    _py___str__: __SlateRecord_pycore_global_strings_h_220,
    _py___sub__: __SlateRecord_pycore_global_strings_h_221,
    _py___subclasscheck__: __SlateRecord_pycore_global_strings_h_222,
    _py___subclasshook__: __SlateRecord_pycore_global_strings_h_223,
    _py___truediv__: __SlateRecord_pycore_global_strings_h_224,
    _py___trunc__: __SlateRecord_pycore_global_strings_h_225,
    _py___type_params__: __SlateRecord_pycore_global_strings_h_226,
    _py___typing_is_unpacked_typevartuple__: __SlateRecord_pycore_global_strings_h_227,
    _py___typing_prepare_subst__: __SlateRecord_pycore_global_strings_h_228,
    _py___typing_subst__: __SlateRecord_pycore_global_strings_h_229,
    _py___typing_unpacked_tuple_args__: __SlateRecord_pycore_global_strings_h_230,
    _py___warningregistry__: __SlateRecord_pycore_global_strings_h_231,
    _py___weaklistoffset__: __SlateRecord_pycore_global_strings_h_232,
    _py___weakref__: __SlateRecord_pycore_global_strings_h_233,
    _py___xor__: __SlateRecord_pycore_global_strings_h_234,
    _py__abc_impl: __SlateRecord_pycore_global_strings_h_235,
    _py__abstract_: __SlateRecord_pycore_global_strings_h_236,
    _py__active: __SlateRecord_pycore_global_strings_h_237,
    _py__anonymous_: __SlateRecord_pycore_global_strings_h_238,
    _py__argtypes_: __SlateRecord_pycore_global_strings_h_239,
    _py__as_parameter_: __SlateRecord_pycore_global_strings_h_240,
    _py__asyncio_future_blocking: __SlateRecord_pycore_global_strings_h_241,
    _py__blksize: __SlateRecord_pycore_global_strings_h_242,
    _py__bootstrap: __SlateRecord_pycore_global_strings_h_243,
    _py__check_retval_: __SlateRecord_pycore_global_strings_h_244,
    _py__dealloc_warn: __SlateRecord_pycore_global_strings_h_245,
    _py__feature_version: __SlateRecord_pycore_global_strings_h_246,
    _py__field_types: __SlateRecord_pycore_global_strings_h_247,
    _py__fields_: __SlateRecord_pycore_global_strings_h_248,
    _py__filters: __SlateRecord_pycore_global_strings_h_249,
    _py__finalizing: __SlateRecord_pycore_global_strings_h_250,
    _py__find_and_load: __SlateRecord_pycore_global_strings_h_251,
    _py__fix_up_module: __SlateRecord_pycore_global_strings_h_252,
    _py__flags_: __SlateRecord_pycore_global_strings_h_253,
    _py__get_sourcefile: __SlateRecord_pycore_global_strings_h_254,
    _py__handle_fromlist: __SlateRecord_pycore_global_strings_h_255,
    _py__initializing: __SlateRecord_pycore_global_strings_h_256,
    _py__internal_use: __SlateRecord_pycore_global_strings_h_257,
    _py__io: __SlateRecord_pycore_global_strings_h_258,
    _py__is_text_encoding: __SlateRecord_pycore_global_strings_h_259,
    _py__isatty_open_only: __SlateRecord_pycore_global_strings_h_260,
    _py__length_: __SlateRecord_pycore_global_strings_h_261,
    _py__limbo: __SlateRecord_pycore_global_strings_h_262,
    _py__lock_unlock_module: __SlateRecord_pycore_global_strings_h_263,
    _py__loop: __SlateRecord_pycore_global_strings_h_264,
    _py__needs_com_addref_: __SlateRecord_pycore_global_strings_h_265,
    _py__only_immortal: __SlateRecord_pycore_global_strings_h_266,
    _py__restype_: __SlateRecord_pycore_global_strings_h_267,
    _py__showwarnmsg: __SlateRecord_pycore_global_strings_h_268,
    _py__shutdown: __SlateRecord_pycore_global_strings_h_269,
    _py__slotnames: __SlateRecord_pycore_global_strings_h_270,
    _py__strptime: __SlateRecord_pycore_global_strings_h_271,
    _py__strptime_datetime_date: __SlateRecord_pycore_global_strings_h_272,
    _py__strptime_datetime_datetime: __SlateRecord_pycore_global_strings_h_273,
    _py__strptime_datetime_time: __SlateRecord_pycore_global_strings_h_274,
    _py__type_: __SlateRecord_pycore_global_strings_h_275,
    _py__uninitialized_submodules: __SlateRecord_pycore_global_strings_h_276,
    _py__warn_unawaited_coroutine: __SlateRecord_pycore_global_strings_h_277,
    _py__xoptions: __SlateRecord_pycore_global_strings_h_278,
    _py_abs_tol: __SlateRecord_pycore_global_strings_h_279,
    _py_access: __SlateRecord_pycore_global_strings_h_280,
    _py_aclose: __SlateRecord_pycore_global_strings_h_281,
    _py_add: __SlateRecord_pycore_global_strings_h_282,
    _py_add_done_callback: __SlateRecord_pycore_global_strings_h_283,
    _py_after_in_child: __SlateRecord_pycore_global_strings_h_284,
    _py_after_in_parent: __SlateRecord_pycore_global_strings_h_285,
    _py_aggregate_class: __SlateRecord_pycore_global_strings_h_286,
    _py_alias: __SlateRecord_pycore_global_strings_h_287,
    _py_align: __SlateRecord_pycore_global_strings_h_288,
    _py_all: __SlateRecord_pycore_global_strings_h_289,
    _py_all_threads: __SlateRecord_pycore_global_strings_h_290,
    _py_allow_code: __SlateRecord_pycore_global_strings_h_291,
    _py_any: __SlateRecord_pycore_global_strings_h_292,
    _py_append: __SlateRecord_pycore_global_strings_h_293,
    _py_arg: __SlateRecord_pycore_global_strings_h_294,
    _py_argdefs: __SlateRecord_pycore_global_strings_h_295,
    _py_args: __SlateRecord_pycore_global_strings_h_296,
    _py_arguments: __SlateRecord_pycore_global_strings_h_297,
    _py_argv: __SlateRecord_pycore_global_strings_h_298,
    _py_as_integer_ratio: __SlateRecord_pycore_global_strings_h_299,
    _py_asend: __SlateRecord_pycore_global_strings_h_300,
    _py_ast: __SlateRecord_pycore_global_strings_h_301,
    _py_athrow: __SlateRecord_pycore_global_strings_h_302,
    _py_attribute: __SlateRecord_pycore_global_strings_h_303,
    _py_authorizer_callback: __SlateRecord_pycore_global_strings_h_304,
    _py_autocommit: __SlateRecord_pycore_global_strings_h_305,
    _py_backtick: __SlateRecord_pycore_global_strings_h_306,
    _py_base: __SlateRecord_pycore_global_strings_h_307,
    _py_before: __SlateRecord_pycore_global_strings_h_308,
    _py_big: __SlateRecord_pycore_global_strings_h_309,
    _py_binary_form: __SlateRecord_pycore_global_strings_h_310,
    _py_bit_offset: __SlateRecord_pycore_global_strings_h_311,
    _py_bit_size: __SlateRecord_pycore_global_strings_h_312,
    _py_block: __SlateRecord_pycore_global_strings_h_313,
    _py_bound: __SlateRecord_pycore_global_strings_h_314,
    _py_buffer: __SlateRecord_pycore_global_strings_h_315,
    _py_buffer_callback: __SlateRecord_pycore_global_strings_h_316,
    _py_buffer_size: __SlateRecord_pycore_global_strings_h_317,
    _py_buffering: __SlateRecord_pycore_global_strings_h_318,
    _py_buffers: __SlateRecord_pycore_global_strings_h_319,
    _py_bufsize: __SlateRecord_pycore_global_strings_h_320,
    _py_builtins: __SlateRecord_pycore_global_strings_h_321,
    _py_byte_offset: __SlateRecord_pycore_global_strings_h_322,
    _py_byte_size: __SlateRecord_pycore_global_strings_h_323,
    _py_byteorder: __SlateRecord_pycore_global_strings_h_324,
    _py_bytes: __SlateRecord_pycore_global_strings_h_325,
    _py_bytes_per_sep: __SlateRecord_pycore_global_strings_h_326,
    _py_c_call: __SlateRecord_pycore_global_strings_h_327,
    _py_c_exception: __SlateRecord_pycore_global_strings_h_328,
    _py_c_parameter_type: __SlateRecord_pycore_global_strings_h_329,
    _py_c_return: __SlateRecord_pycore_global_strings_h_330,
    _py_cached_datetime_module: __SlateRecord_pycore_global_strings_h_331,
    _py_cached_statements: __SlateRecord_pycore_global_strings_h_332,
    _py_cadata: __SlateRecord_pycore_global_strings_h_333,
    _py_cafile: __SlateRecord_pycore_global_strings_h_334,
    _py_call: __SlateRecord_pycore_global_strings_h_335,
    _py_call_exception_handler: __SlateRecord_pycore_global_strings_h_336,
    _py_call_soon: __SlateRecord_pycore_global_strings_h_337,
    _py_callback: __SlateRecord_pycore_global_strings_h_338,
    _py_cancel: __SlateRecord_pycore_global_strings_h_339,
    _py_capath: __SlateRecord_pycore_global_strings_h_340,
    _py_category: __SlateRecord_pycore_global_strings_h_341,
    _py_cb_type: __SlateRecord_pycore_global_strings_h_342,
    _py_certfile: __SlateRecord_pycore_global_strings_h_343,
    _py_check_same_thread: __SlateRecord_pycore_global_strings_h_344,
    _py_clear: __SlateRecord_pycore_global_strings_h_345,
    _py_close: __SlateRecord_pycore_global_strings_h_346,
    _py_closed: __SlateRecord_pycore_global_strings_h_347,
    _py_closefd: __SlateRecord_pycore_global_strings_h_348,
    _py_closure: __SlateRecord_pycore_global_strings_h_349,
    _py_co_argcount: __SlateRecord_pycore_global_strings_h_350,
    _py_co_cellvars: __SlateRecord_pycore_global_strings_h_351,
    _py_co_code: __SlateRecord_pycore_global_strings_h_352,
    _py_co_consts: __SlateRecord_pycore_global_strings_h_353,
    _py_co_exceptiontable: __SlateRecord_pycore_global_strings_h_354,
    _py_co_filename: __SlateRecord_pycore_global_strings_h_355,
    _py_co_firstlineno: __SlateRecord_pycore_global_strings_h_356,
    _py_co_flags: __SlateRecord_pycore_global_strings_h_357,
    _py_co_freevars: __SlateRecord_pycore_global_strings_h_358,
    _py_co_kwonlyargcount: __SlateRecord_pycore_global_strings_h_359,
    _py_co_linetable: __SlateRecord_pycore_global_strings_h_360,
    _py_co_name: __SlateRecord_pycore_global_strings_h_361,
    _py_co_names: __SlateRecord_pycore_global_strings_h_362,
    _py_co_nlocals: __SlateRecord_pycore_global_strings_h_363,
    _py_co_posonlyargcount: __SlateRecord_pycore_global_strings_h_364,
    _py_co_qualname: __SlateRecord_pycore_global_strings_h_365,
    _py_co_stacksize: __SlateRecord_pycore_global_strings_h_366,
    _py_co_varnames: __SlateRecord_pycore_global_strings_h_367,
    _py_code: __SlateRecord_pycore_global_strings_h_368,
    _py_col_offset: __SlateRecord_pycore_global_strings_h_369,
    _py_command: __SlateRecord_pycore_global_strings_h_370,
    _py_comment_factory: __SlateRecord_pycore_global_strings_h_371,
    _py_compile_mode: __SlateRecord_pycore_global_strings_h_372,
    _py_consts: __SlateRecord_pycore_global_strings_h_373,
    _py_context: __SlateRecord_pycore_global_strings_h_374,
    _py_contravariant: __SlateRecord_pycore_global_strings_h_375,
    _py_conversion: __SlateRecord_pycore_global_strings_h_376,
    _py_cookie: __SlateRecord_pycore_global_strings_h_377,
    _py_copy: __SlateRecord_pycore_global_strings_h_378,
    _py_copyreg: __SlateRecord_pycore_global_strings_h_379,
    _py_coro: __SlateRecord_pycore_global_strings_h_380,
    _py_count: __SlateRecord_pycore_global_strings_h_381,
    _py_covariant: __SlateRecord_pycore_global_strings_h_382,
    _py_cwd: __SlateRecord_pycore_global_strings_h_383,
    _py_d_parameter_type: __SlateRecord_pycore_global_strings_h_384,
    _py_data: __SlateRecord_pycore_global_strings_h_385,
    _py_database: __SlateRecord_pycore_global_strings_h_386,
    _py_day: __SlateRecord_pycore_global_strings_h_387,
    _py_debug: __SlateRecord_pycore_global_strings_h_388,
    _py_decode: __SlateRecord_pycore_global_strings_h_389,
    _py_decoder: __SlateRecord_pycore_global_strings_h_390,
    _py_default: __SlateRecord_pycore_global_strings_h_391,
    _py_defaultaction: __SlateRecord_pycore_global_strings_h_392,
    _py_delete: __SlateRecord_pycore_global_strings_h_393,
    _py_depth: __SlateRecord_pycore_global_strings_h_394,
    _py_desired_access: __SlateRecord_pycore_global_strings_h_395,
    _py_detect_types: __SlateRecord_pycore_global_strings_h_396,
    _py_deterministic: __SlateRecord_pycore_global_strings_h_397,
    _py_device: __SlateRecord_pycore_global_strings_h_398,
    _py_dict: __SlateRecord_pycore_global_strings_h_399,
    _py_dictcomp: __SlateRecord_pycore_global_strings_h_400,
    _py_difference_update: __SlateRecord_pycore_global_strings_h_401,
    _py_digest: __SlateRecord_pycore_global_strings_h_402,
    _py_digest_size: __SlateRecord_pycore_global_strings_h_403,
    _py_digestmod: __SlateRecord_pycore_global_strings_h_404,
    _py_dir_fd: __SlateRecord_pycore_global_strings_h_405,
    _py_discard: __SlateRecord_pycore_global_strings_h_406,
    _py_dispatch_table: __SlateRecord_pycore_global_strings_h_407,
    _py_displayhook: __SlateRecord_pycore_global_strings_h_408,
    _py_dklen: __SlateRecord_pycore_global_strings_h_409,
    _py_doc: __SlateRecord_pycore_global_strings_h_410,
    _py_done: __SlateRecord_pycore_global_strings_h_411,
    _py_dont_inherit: __SlateRecord_pycore_global_strings_h_412,
    _py_dst: __SlateRecord_pycore_global_strings_h_413,
    _py_dst_dir_fd: __SlateRecord_pycore_global_strings_h_414,
    _py_eager_start: __SlateRecord_pycore_global_strings_h_415,
    _py_effective_ids: __SlateRecord_pycore_global_strings_h_416,
    _py_element_factory: __SlateRecord_pycore_global_strings_h_417,
    _py_encode: __SlateRecord_pycore_global_strings_h_418,
    _py_encoding: __SlateRecord_pycore_global_strings_h_419,
    _py_end: __SlateRecord_pycore_global_strings_h_420,
    _py_end_col_offset: __SlateRecord_pycore_global_strings_h_421,
    _py_end_lineno: __SlateRecord_pycore_global_strings_h_422,
    _py_end_offset: __SlateRecord_pycore_global_strings_h_423,
    _py_endpos: __SlateRecord_pycore_global_strings_h_424,
    _py_entrypoint: __SlateRecord_pycore_global_strings_h_425,
    _py_env: __SlateRecord_pycore_global_strings_h_426,
    _py_errors: __SlateRecord_pycore_global_strings_h_427,
    _py_event: __SlateRecord_pycore_global_strings_h_428,
    _py_eventmask: __SlateRecord_pycore_global_strings_h_429,
    _py_exc_type: __SlateRecord_pycore_global_strings_h_430,
    _py_exc_value: __SlateRecord_pycore_global_strings_h_431,
    _py_excepthook: __SlateRecord_pycore_global_strings_h_432,
    _py_exception: __SlateRecord_pycore_global_strings_h_433,
    _py_existing_file_name: __SlateRecord_pycore_global_strings_h_434,
    _py_exp: __SlateRecord_pycore_global_strings_h_435,
    _py_expression: __SlateRecord_pycore_global_strings_h_436,
    _py_extend: __SlateRecord_pycore_global_strings_h_437,
    _py_extra_tokens: __SlateRecord_pycore_global_strings_h_438,
    _py_facility: __SlateRecord_pycore_global_strings_h_439,
    _py_factory: __SlateRecord_pycore_global_strings_h_440,
    _py_false: __SlateRecord_pycore_global_strings_h_441,
    _py_family: __SlateRecord_pycore_global_strings_h_442,
    _py_fanout: __SlateRecord_pycore_global_strings_h_443,
    _py_fd: __SlateRecord_pycore_global_strings_h_444,
    _py_fd2: __SlateRecord_pycore_global_strings_h_445,
    _py_fdel: __SlateRecord_pycore_global_strings_h_446,
    _py_fget: __SlateRecord_pycore_global_strings_h_447,
    _py_fields: __SlateRecord_pycore_global_strings_h_448,
    _py_file: __SlateRecord_pycore_global_strings_h_449,
    _py_file_actions: __SlateRecord_pycore_global_strings_h_450,
    _py_filename: __SlateRecord_pycore_global_strings_h_451,
    _py_fileno: __SlateRecord_pycore_global_strings_h_452,
    _py_filepath: __SlateRecord_pycore_global_strings_h_453,
    _py_fillvalue: __SlateRecord_pycore_global_strings_h_454,
    _py_filter: __SlateRecord_pycore_global_strings_h_455,
    _py_filters: __SlateRecord_pycore_global_strings_h_456,
    _py_final: __SlateRecord_pycore_global_strings_h_457,
    _py_find_class: __SlateRecord_pycore_global_strings_h_458,
    _py_fix_imports: __SlateRecord_pycore_global_strings_h_459,
    _py_flags: __SlateRecord_pycore_global_strings_h_460,
    _py_flush: __SlateRecord_pycore_global_strings_h_461,
    _py_fold: __SlateRecord_pycore_global_strings_h_462,
    _py_follow_symlinks: __SlateRecord_pycore_global_strings_h_463,
    _py_format: __SlateRecord_pycore_global_strings_h_464,
    _py_format_spec: __SlateRecord_pycore_global_strings_h_465,
    _py_frame_buffer: __SlateRecord_pycore_global_strings_h_466,
    _py_from_param: __SlateRecord_pycore_global_strings_h_467,
    _py_fromlist: __SlateRecord_pycore_global_strings_h_468,
    _py_fromtimestamp: __SlateRecord_pycore_global_strings_h_469,
    _py_fromutc: __SlateRecord_pycore_global_strings_h_470,
    _py_fset: __SlateRecord_pycore_global_strings_h_471,
    _py_func: __SlateRecord_pycore_global_strings_h_472,
    _py_future: __SlateRecord_pycore_global_strings_h_473,
    _py_generation: __SlateRecord_pycore_global_strings_h_474,
    _py_genexpr: __SlateRecord_pycore_global_strings_h_475,
    _py_get: __SlateRecord_pycore_global_strings_h_476,
    _py_get_debug: __SlateRecord_pycore_global_strings_h_477,
    _py_get_event_loop: __SlateRecord_pycore_global_strings_h_478,
    _py_get_loop: __SlateRecord_pycore_global_strings_h_479,
    _py_get_source: __SlateRecord_pycore_global_strings_h_480,
    _py_getattr: __SlateRecord_pycore_global_strings_h_481,
    _py_getstate: __SlateRecord_pycore_global_strings_h_482,
    _py_gid: __SlateRecord_pycore_global_strings_h_483,
    _py_globals: __SlateRecord_pycore_global_strings_h_484,
    _py_groupindex: __SlateRecord_pycore_global_strings_h_485,
    _py_groups: __SlateRecord_pycore_global_strings_h_486,
    _py_handle: __SlateRecord_pycore_global_strings_h_487,
    _py_handle_seq: __SlateRecord_pycore_global_strings_h_488,
    _py_has_location: __SlateRecord_pycore_global_strings_h_489,
    _py_hash_name: __SlateRecord_pycore_global_strings_h_490,
    _py_header: __SlateRecord_pycore_global_strings_h_491,
    _py_headers: __SlateRecord_pycore_global_strings_h_492,
    _py_hi: __SlateRecord_pycore_global_strings_h_493,
    _py_hook: __SlateRecord_pycore_global_strings_h_494,
    _py_hour: __SlateRecord_pycore_global_strings_h_495,
    _py_id: __SlateRecord_pycore_global_strings_h_496,
    _py_ident: __SlateRecord_pycore_global_strings_h_497,
    _py_identity_hint: __SlateRecord_pycore_global_strings_h_498,
    _py_ignore: __SlateRecord_pycore_global_strings_h_499,
    _py_imag: __SlateRecord_pycore_global_strings_h_500,
    _py_importlib: __SlateRecord_pycore_global_strings_h_501,
    _py_in_fd: __SlateRecord_pycore_global_strings_h_502,
    _py_incoming: __SlateRecord_pycore_global_strings_h_503,
    _py_index: __SlateRecord_pycore_global_strings_h_504,
    _py_indexgroup: __SlateRecord_pycore_global_strings_h_505,
    _py_inf: __SlateRecord_pycore_global_strings_h_506,
    _py_infer_variance: __SlateRecord_pycore_global_strings_h_507,
    _py_inherit_handle: __SlateRecord_pycore_global_strings_h_508,
    _py_inheritable: __SlateRecord_pycore_global_strings_h_509,
    _py_initial: __SlateRecord_pycore_global_strings_h_510,
    _py_initial_bytes: __SlateRecord_pycore_global_strings_h_511,
    _py_initial_owner: __SlateRecord_pycore_global_strings_h_512,
    _py_initial_state: __SlateRecord_pycore_global_strings_h_513,
    _py_initial_value: __SlateRecord_pycore_global_strings_h_514,
    _py_initval: __SlateRecord_pycore_global_strings_h_515,
    _py_inner_size: __SlateRecord_pycore_global_strings_h_516,
    _py_input: __SlateRecord_pycore_global_strings_h_517,
    _py_insert_comments: __SlateRecord_pycore_global_strings_h_518,
    _py_insert_pis: __SlateRecord_pycore_global_strings_h_519,
    _py_instructions: __SlateRecord_pycore_global_strings_h_520,
    _py_intern: __SlateRecord_pycore_global_strings_h_521,
    _py_intersection: __SlateRecord_pycore_global_strings_h_522,
    _py_interval: __SlateRecord_pycore_global_strings_h_523,
    _py_io: __SlateRecord_pycore_global_strings_h_524,
    _py_is_compress: __SlateRecord_pycore_global_strings_h_525,
    _py_is_raw: __SlateRecord_pycore_global_strings_h_526,
    _py_is_running: __SlateRecord_pycore_global_strings_h_527,
    _py_is_struct: __SlateRecord_pycore_global_strings_h_528,
    _py_isatty: __SlateRecord_pycore_global_strings_h_529,
    _py_isinstance: __SlateRecord_pycore_global_strings_h_530,
    _py_isoformat: __SlateRecord_pycore_global_strings_h_531,
    _py_isolation_level: __SlateRecord_pycore_global_strings_h_532,
    _py_istext: __SlateRecord_pycore_global_strings_h_533,
    _py_item: __SlateRecord_pycore_global_strings_h_534,
    _py_items: __SlateRecord_pycore_global_strings_h_535,
    _py_iter: __SlateRecord_pycore_global_strings_h_536,
    _py_iterable: __SlateRecord_pycore_global_strings_h_537,
    _py_iterations: __SlateRecord_pycore_global_strings_h_538,
    _py_join: __SlateRecord_pycore_global_strings_h_539,
    _py_jump: __SlateRecord_pycore_global_strings_h_540,
    _py_keepends: __SlateRecord_pycore_global_strings_h_541,
    _py_key: __SlateRecord_pycore_global_strings_h_542,
    _py_keyfile: __SlateRecord_pycore_global_strings_h_543,
    _py_keys: __SlateRecord_pycore_global_strings_h_544,
    _py_kind: __SlateRecord_pycore_global_strings_h_545,
    _py_kw: __SlateRecord_pycore_global_strings_h_546,
    _py_kw1: __SlateRecord_pycore_global_strings_h_547,
    _py_kw2: __SlateRecord_pycore_global_strings_h_548,
    _py_kwdefaults: __SlateRecord_pycore_global_strings_h_549,
    _py_label: __SlateRecord_pycore_global_strings_h_550,
    _py_lambda: __SlateRecord_pycore_global_strings_h_551,
    _py_last: __SlateRecord_pycore_global_strings_h_552,
    _py_last_exc: __SlateRecord_pycore_global_strings_h_553,
    _py_last_node: __SlateRecord_pycore_global_strings_h_554,
    _py_last_traceback: __SlateRecord_pycore_global_strings_h_555,
    _py_last_type: __SlateRecord_pycore_global_strings_h_556,
    _py_last_value: __SlateRecord_pycore_global_strings_h_557,
    _py_latin1: __SlateRecord_pycore_global_strings_h_558,
    _py_leaf_size: __SlateRecord_pycore_global_strings_h_559,
    _py_len: __SlateRecord_pycore_global_strings_h_560,
    _py_length: __SlateRecord_pycore_global_strings_h_561,
    _py_level: __SlateRecord_pycore_global_strings_h_562,
    _py_limit: __SlateRecord_pycore_global_strings_h_563,
    _py_line: __SlateRecord_pycore_global_strings_h_564,
    _py_line_buffering: __SlateRecord_pycore_global_strings_h_565,
    _py_lineno: __SlateRecord_pycore_global_strings_h_566,
    _py_listcomp: __SlateRecord_pycore_global_strings_h_567,
    _py_little: __SlateRecord_pycore_global_strings_h_568,
    _py_lo: __SlateRecord_pycore_global_strings_h_569,
    _py_locale: __SlateRecord_pycore_global_strings_h_570,
    _py_locals: __SlateRecord_pycore_global_strings_h_571,
    _py_logoption: __SlateRecord_pycore_global_strings_h_572,
    _py_loop: __SlateRecord_pycore_global_strings_h_573,
    _py_manual_reset: __SlateRecord_pycore_global_strings_h_574,
    _py_mapping: __SlateRecord_pycore_global_strings_h_575,
    _py_match: __SlateRecord_pycore_global_strings_h_576,
    _py_max_length: __SlateRecord_pycore_global_strings_h_577,
    _py_maxdigits: __SlateRecord_pycore_global_strings_h_578,
    _py_maxevents: __SlateRecord_pycore_global_strings_h_579,
    _py_maxlen: __SlateRecord_pycore_global_strings_h_580,
    _py_maxmem: __SlateRecord_pycore_global_strings_h_581,
    _py_maxsplit: __SlateRecord_pycore_global_strings_h_582,
    _py_maxvalue: __SlateRecord_pycore_global_strings_h_583,
    _py_memLevel: __SlateRecord_pycore_global_strings_h_584,
    _py_memlimit: __SlateRecord_pycore_global_strings_h_585,
    _py_message: __SlateRecord_pycore_global_strings_h_586,
    _py_metaclass: __SlateRecord_pycore_global_strings_h_587,
    _py_metadata: __SlateRecord_pycore_global_strings_h_588,
    _py_method: __SlateRecord_pycore_global_strings_h_589,
    _py_microsecond: __SlateRecord_pycore_global_strings_h_590,
    _py_milliseconds: __SlateRecord_pycore_global_strings_h_591,
    _py_minute: __SlateRecord_pycore_global_strings_h_592,
    _py_mod: __SlateRecord_pycore_global_strings_h_593,
    _py_mode: __SlateRecord_pycore_global_strings_h_594,
    _py_module: __SlateRecord_pycore_global_strings_h_595,
    _py_module_globals: __SlateRecord_pycore_global_strings_h_596,
    _py_modules: __SlateRecord_pycore_global_strings_h_597,
    _py_month: __SlateRecord_pycore_global_strings_h_598,
    _py_mro: __SlateRecord_pycore_global_strings_h_599,
    _py_msg: __SlateRecord_pycore_global_strings_h_600,
    _py_mutex: __SlateRecord_pycore_global_strings_h_601,
    _py_mycmp: __SlateRecord_pycore_global_strings_h_602,
    _py_n_arg: __SlateRecord_pycore_global_strings_h_603,
    _py_n_fields: __SlateRecord_pycore_global_strings_h_604,
    _py_n_sequence_fields: __SlateRecord_pycore_global_strings_h_605,
    _py_n_unnamed_fields: __SlateRecord_pycore_global_strings_h_606,
    _py_name: __SlateRecord_pycore_global_strings_h_607,
    _py_name_from: __SlateRecord_pycore_global_strings_h_608,
    _py_namespace_separator: __SlateRecord_pycore_global_strings_h_609,
    _py_namespaces: __SlateRecord_pycore_global_strings_h_610,
    _py_narg: __SlateRecord_pycore_global_strings_h_611,
    _py_ndigits: __SlateRecord_pycore_global_strings_h_612,
    _py_nested: __SlateRecord_pycore_global_strings_h_613,
    _py_new_file_name: __SlateRecord_pycore_global_strings_h_614,
    _py_new_limit: __SlateRecord_pycore_global_strings_h_615,
    _py_newline: __SlateRecord_pycore_global_strings_h_616,
    _py_newlines: __SlateRecord_pycore_global_strings_h_617,
    _py_next: __SlateRecord_pycore_global_strings_h_618,
    _py_nlocals: __SlateRecord_pycore_global_strings_h_619,
    _py_node_depth: __SlateRecord_pycore_global_strings_h_620,
    _py_node_offset: __SlateRecord_pycore_global_strings_h_621,
    _py_ns: __SlateRecord_pycore_global_strings_h_622,
    _py_nstype: __SlateRecord_pycore_global_strings_h_623,
    _py_nt: __SlateRecord_pycore_global_strings_h_624,
    _py_null: __SlateRecord_pycore_global_strings_h_625,
    _py_number: __SlateRecord_pycore_global_strings_h_626,
    _py_obj: __SlateRecord_pycore_global_strings_h_627,
    _py_object: __SlateRecord_pycore_global_strings_h_628,
    _py_offset: __SlateRecord_pycore_global_strings_h_629,
    _py_offset_dst: __SlateRecord_pycore_global_strings_h_630,
    _py_offset_src: __SlateRecord_pycore_global_strings_h_631,
    _py_on_type_read: __SlateRecord_pycore_global_strings_h_632,
    _py_onceregistry: __SlateRecord_pycore_global_strings_h_633,
    _py_only_active_thread: __SlateRecord_pycore_global_strings_h_634,
    _py_only_keys: __SlateRecord_pycore_global_strings_h_635,
    _py_oparg: __SlateRecord_pycore_global_strings_h_636,
    _py_opcode: __SlateRecord_pycore_global_strings_h_637,
    _py_open: __SlateRecord_pycore_global_strings_h_638,
    _py_opener: __SlateRecord_pycore_global_strings_h_639,
    _py_operation: __SlateRecord_pycore_global_strings_h_640,
    _py_optimize: __SlateRecord_pycore_global_strings_h_641,
    _py_options: __SlateRecord_pycore_global_strings_h_642,
    _py_order: __SlateRecord_pycore_global_strings_h_643,
    _py_origin: __SlateRecord_pycore_global_strings_h_644,
    _py_out_fd: __SlateRecord_pycore_global_strings_h_645,
    _py_outgoing: __SlateRecord_pycore_global_strings_h_646,
    _py_outpath: __SlateRecord_pycore_global_strings_h_647,
    _py_overlapped: __SlateRecord_pycore_global_strings_h_648,
    _py_owner: __SlateRecord_pycore_global_strings_h_649,
    _py_pages: __SlateRecord_pycore_global_strings_h_650,
    _py_parameter: __SlateRecord_pycore_global_strings_h_651,
    _py_parent: __SlateRecord_pycore_global_strings_h_652,
    _py_password: __SlateRecord_pycore_global_strings_h_653,
    _py_path: __SlateRecord_pycore_global_strings_h_654,
    _py_pattern: __SlateRecord_pycore_global_strings_h_655,
    _py_peek: __SlateRecord_pycore_global_strings_h_656,
    _py_persistent_id: __SlateRecord_pycore_global_strings_h_657,
    _py_persistent_load: __SlateRecord_pycore_global_strings_h_658,
    _py_person: __SlateRecord_pycore_global_strings_h_659,
    _py_pi_factory: __SlateRecord_pycore_global_strings_h_660,
    _py_pid: __SlateRecord_pycore_global_strings_h_661,
    _py_policy: __SlateRecord_pycore_global_strings_h_662,
    _py_pos: __SlateRecord_pycore_global_strings_h_663,
    _py_pos1: __SlateRecord_pycore_global_strings_h_664,
    _py_pos2: __SlateRecord_pycore_global_strings_h_665,
    _py_posix: __SlateRecord_pycore_global_strings_h_666,
    _py_print_file_and_line: __SlateRecord_pycore_global_strings_h_667,
    _py_priority: __SlateRecord_pycore_global_strings_h_668,
    _py_progress: __SlateRecord_pycore_global_strings_h_669,
    _py_progress_handler: __SlateRecord_pycore_global_strings_h_670,
    _py_progress_routine: __SlateRecord_pycore_global_strings_h_671,
    _py_proto: __SlateRecord_pycore_global_strings_h_672,
    _py_protocol: __SlateRecord_pycore_global_strings_h_673,
    _py_ps1: __SlateRecord_pycore_global_strings_h_674,
    _py_ps2: __SlateRecord_pycore_global_strings_h_675,
    _py_query: __SlateRecord_pycore_global_strings_h_676,
    _py_quotetabs: __SlateRecord_pycore_global_strings_h_677,
    _py_raw: __SlateRecord_pycore_global_strings_h_678,
    _py_read: __SlateRecord_pycore_global_strings_h_679,
    _py_read1: __SlateRecord_pycore_global_strings_h_680,
    _py_readable: __SlateRecord_pycore_global_strings_h_681,
    _py_readall: __SlateRecord_pycore_global_strings_h_682,
    _py_readinto: __SlateRecord_pycore_global_strings_h_683,
    _py_readinto1: __SlateRecord_pycore_global_strings_h_684,
    _py_readline: __SlateRecord_pycore_global_strings_h_685,
    _py_readonly: __SlateRecord_pycore_global_strings_h_686,
    _py_real: __SlateRecord_pycore_global_strings_h_687,
    _py_reducer_override: __SlateRecord_pycore_global_strings_h_688,
    _py_registry: __SlateRecord_pycore_global_strings_h_689,
    _py_rel_tol: __SlateRecord_pycore_global_strings_h_690,
    _py_release: __SlateRecord_pycore_global_strings_h_691,
    _py_reload: __SlateRecord_pycore_global_strings_h_692,
    _py_repl: __SlateRecord_pycore_global_strings_h_693,
    _py_replace: __SlateRecord_pycore_global_strings_h_694,
    _py_reserved: __SlateRecord_pycore_global_strings_h_695,
    _py_reset: __SlateRecord_pycore_global_strings_h_696,
    _py_resetids: __SlateRecord_pycore_global_strings_h_697,
    _py_return: __SlateRecord_pycore_global_strings_h_698,
    _py_reverse: __SlateRecord_pycore_global_strings_h_699,
    _py_reversed: __SlateRecord_pycore_global_strings_h_700,
    _py_salt: __SlateRecord_pycore_global_strings_h_701,
    _py_sched_priority: __SlateRecord_pycore_global_strings_h_702,
    _py_scheduler: __SlateRecord_pycore_global_strings_h_703,
    _py_script: __SlateRecord_pycore_global_strings_h_704,
    _py_second: __SlateRecord_pycore_global_strings_h_705,
    _py_security_attributes: __SlateRecord_pycore_global_strings_h_706,
    _py_seek: __SlateRecord_pycore_global_strings_h_707,
    _py_seekable: __SlateRecord_pycore_global_strings_h_708,
    _py_selectors: __SlateRecord_pycore_global_strings_h_709,
    _py_self: __SlateRecord_pycore_global_strings_h_710,
    _py_send: __SlateRecord_pycore_global_strings_h_711,
    _py_sep: __SlateRecord_pycore_global_strings_h_712,
    _py_sequence: __SlateRecord_pycore_global_strings_h_713,
    _py_server_hostname: __SlateRecord_pycore_global_strings_h_714,
    _py_server_side: __SlateRecord_pycore_global_strings_h_715,
    _py_session: __SlateRecord_pycore_global_strings_h_716,
    _py_setcomp: __SlateRecord_pycore_global_strings_h_717,
    _py_setpgroup: __SlateRecord_pycore_global_strings_h_718,
    _py_setsid: __SlateRecord_pycore_global_strings_h_719,
    _py_setsigdef: __SlateRecord_pycore_global_strings_h_720,
    _py_setsigmask: __SlateRecord_pycore_global_strings_h_721,
    _py_setstate: __SlateRecord_pycore_global_strings_h_722,
    _py_shape: __SlateRecord_pycore_global_strings_h_723,
    _py_show_cmd: __SlateRecord_pycore_global_strings_h_724,
    _py_signed: __SlateRecord_pycore_global_strings_h_725,
    _py_size: __SlateRecord_pycore_global_strings_h_726,
    _py_sizehint: __SlateRecord_pycore_global_strings_h_727,
    _py_skip_file_prefixes: __SlateRecord_pycore_global_strings_h_728,
    _py_sleep: __SlateRecord_pycore_global_strings_h_729,
    _py_sock: __SlateRecord_pycore_global_strings_h_730,
    _py_sort: __SlateRecord_pycore_global_strings_h_731,
    _py_source: __SlateRecord_pycore_global_strings_h_732,
    _py_source_traceback: __SlateRecord_pycore_global_strings_h_733,
    _py_spam: __SlateRecord_pycore_global_strings_h_734,
    _py_src: __SlateRecord_pycore_global_strings_h_735,
    _py_src_dir_fd: __SlateRecord_pycore_global_strings_h_736,
    _py_stacklevel: __SlateRecord_pycore_global_strings_h_737,
    _py_start: __SlateRecord_pycore_global_strings_h_738,
    _py_statement: __SlateRecord_pycore_global_strings_h_739,
    _py_status: __SlateRecord_pycore_global_strings_h_740,
    _py_stderr: __SlateRecord_pycore_global_strings_h_741,
    _py_stdin: __SlateRecord_pycore_global_strings_h_742,
    _py_stdout: __SlateRecord_pycore_global_strings_h_743,
    _py_step: __SlateRecord_pycore_global_strings_h_744,
    _py_steps: __SlateRecord_pycore_global_strings_h_745,
    _py_store_name: __SlateRecord_pycore_global_strings_h_746,
    _py_strategy: __SlateRecord_pycore_global_strings_h_747,
    _py_strftime: __SlateRecord_pycore_global_strings_h_748,
    _py_strict: __SlateRecord_pycore_global_strings_h_749,
    _py_strict_mode: __SlateRecord_pycore_global_strings_h_750,
    _py_string: __SlateRecord_pycore_global_strings_h_751,
    _py_sub_key: __SlateRecord_pycore_global_strings_h_752,
    _py_subcalls: __SlateRecord_pycore_global_strings_h_753,
    _py_symmetric_difference_update: __SlateRecord_pycore_global_strings_h_754,
    _py_tabsize: __SlateRecord_pycore_global_strings_h_755,
    _py_tag: __SlateRecord_pycore_global_strings_h_756,
    _py_target: __SlateRecord_pycore_global_strings_h_757,
    _py_target_is_directory: __SlateRecord_pycore_global_strings_h_758,
    _py_task: __SlateRecord_pycore_global_strings_h_759,
    _py_tb_frame: __SlateRecord_pycore_global_strings_h_760,
    _py_tb_lasti: __SlateRecord_pycore_global_strings_h_761,
    _py_tb_lineno: __SlateRecord_pycore_global_strings_h_762,
    _py_tb_next: __SlateRecord_pycore_global_strings_h_763,
    _py_tell: __SlateRecord_pycore_global_strings_h_764,
    _py_template: __SlateRecord_pycore_global_strings_h_765,
    _py_term: __SlateRecord_pycore_global_strings_h_766,
    _py_text: __SlateRecord_pycore_global_strings_h_767,
    _py_threading: __SlateRecord_pycore_global_strings_h_768,
    _py_throw: __SlateRecord_pycore_global_strings_h_769,
    _py_timeout: __SlateRecord_pycore_global_strings_h_770,
    _py_timer: __SlateRecord_pycore_global_strings_h_771,
    _py_times: __SlateRecord_pycore_global_strings_h_772,
    _py_timetuple: __SlateRecord_pycore_global_strings_h_773,
    _py_timeunit: __SlateRecord_pycore_global_strings_h_774,
    _py_top: __SlateRecord_pycore_global_strings_h_775,
    _py_trace_callback: __SlateRecord_pycore_global_strings_h_776,
    _py_traceback: __SlateRecord_pycore_global_strings_h_777,
    _py_trailers: __SlateRecord_pycore_global_strings_h_778,
    _py_translate: __SlateRecord_pycore_global_strings_h_779,
    _py_true: __SlateRecord_pycore_global_strings_h_780,
    _py_truncate: __SlateRecord_pycore_global_strings_h_781,
    _py_twice: __SlateRecord_pycore_global_strings_h_782,
    _py_txt: __SlateRecord_pycore_global_strings_h_783,
    _py_type: __SlateRecord_pycore_global_strings_h_784,
    _py_type_params: __SlateRecord_pycore_global_strings_h_785,
    _py_tz: __SlateRecord_pycore_global_strings_h_786,
    _py_tzinfo: __SlateRecord_pycore_global_strings_h_787,
    _py_tzname: __SlateRecord_pycore_global_strings_h_788,
    _py_uid: __SlateRecord_pycore_global_strings_h_789,
    _py_unlink: __SlateRecord_pycore_global_strings_h_790,
    _py_unraisablehook: __SlateRecord_pycore_global_strings_h_791,
    _py_uri: __SlateRecord_pycore_global_strings_h_792,
    _py_usedforsecurity: __SlateRecord_pycore_global_strings_h_793,
    _py_value: __SlateRecord_pycore_global_strings_h_794,
    _py_values: __SlateRecord_pycore_global_strings_h_795,
    _py_version: __SlateRecord_pycore_global_strings_h_796,
    _py_volume: __SlateRecord_pycore_global_strings_h_797,
    _py_wait_all: __SlateRecord_pycore_global_strings_h_798,
    _py_warn_on_full_buffer: __SlateRecord_pycore_global_strings_h_799,
    _py_warnings: __SlateRecord_pycore_global_strings_h_800,
    _py_warnoptions: __SlateRecord_pycore_global_strings_h_801,
    _py_wbits: __SlateRecord_pycore_global_strings_h_802,
    _py_week: __SlateRecord_pycore_global_strings_h_803,
    _py_weekday: __SlateRecord_pycore_global_strings_h_804,
    _py_which: __SlateRecord_pycore_global_strings_h_805,
    _py_who: __SlateRecord_pycore_global_strings_h_806,
    _py_withdata: __SlateRecord_pycore_global_strings_h_807,
    _py_writable: __SlateRecord_pycore_global_strings_h_808,
    _py_write: __SlateRecord_pycore_global_strings_h_809,
    _py_write_through: __SlateRecord_pycore_global_strings_h_810,
    _py_year: __SlateRecord_pycore_global_strings_h_811,
    _py_zdict: __SlateRecord_pycore_global_strings_h_812,
    _py_zstd_dict: __SlateRecord_pycore_global_strings_h_813,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_59 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_60 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_61 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_62 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 16],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_63 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_64 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_65 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 14],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_66 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_67 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 15],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_68 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 18],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_69 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 16],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_70 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 16],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_71 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_72 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 20],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_73 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_74 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_75 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_76 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_77 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_78 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_79 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_80 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 13],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_81 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 18],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_82 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 16],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_83 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 22],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_84 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_85 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_86 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_87 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_88 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_89 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 16],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_90 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 13],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_91 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_92 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_93 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 13],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_94 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_95 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_96 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 18],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_97 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 14],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_98 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 14],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_99 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 18],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_100 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_101 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 28],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_102 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 13],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_103 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 25],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_104 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_105 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_106 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_107 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_108 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_109 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 15],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_110 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_111 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_112 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_113 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_114 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_115 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_116 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_117 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 16],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_118 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_119 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_120 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 13],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_121 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_122 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_123 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_124 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_125 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_126 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 17],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_127 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 16],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_128 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_129 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 15],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_130 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 18],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_131 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 13],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_132 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_133 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_134 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_135 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_136 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 14],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_137 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_138 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_139 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_140 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_141 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_142 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_143 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_144 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 18],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_145 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 18],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_146 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_147 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_148 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_149 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_150 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_151 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 21],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_152 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_153 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_154 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 13],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_155 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_156 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_157 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_158 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 16],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_159 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_160 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_161 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_162 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_163 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_164 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 15],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_165 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_166 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_167 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_168 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_169 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 16],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_170 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_171 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_172 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_173 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_174 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_175 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_176 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 14],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_177 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_178 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_179 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_180 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 15],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_181 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_182 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_183 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 15],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_184 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_185 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_186 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_187 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_188 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 13],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_189 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_190 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_191 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_192 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_193 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 14],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_194 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 19],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_195 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_196 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 13],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_197 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 14],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_198 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_199 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_200 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_201 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_202 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_203 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_204 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_205 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_206 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_207 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_208 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 13],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_209 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_210 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_211 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 13],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_212 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_213 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_214 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 13],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_215 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_216 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 14],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_217 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_218 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_219 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 22],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_220 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_221 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_222 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 18],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_223 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 17],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_224 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_225 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_226 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 16],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_227 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 36],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_228 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 25],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_229 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 17],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_230 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 31],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_231 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 20],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_232 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 19],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_233 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_234 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_235 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_236 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_237 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_238 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_239 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_240 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 15],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_241 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 25],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_242 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_243 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_244 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 15],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_245 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 14],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_246 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 17],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_247 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 13],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_248 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_249 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_250 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_251 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 15],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_252 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 15],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_253 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_254 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 16],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_255 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 17],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_256 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 14],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_257 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 14],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_258 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_259 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 18],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_260 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 18],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_261 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_262 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_263 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 20],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_264 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_265 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 19],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_266 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 15],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_267 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_268 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 13],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_269 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_270 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_271 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_272 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 24],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_273 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 28],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_274 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 24],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_275 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_276 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 26],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_277 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 26],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_278 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_279 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_280 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_281 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_282 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_283 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 18],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_284 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 15],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_285 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 16],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_286 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 16],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_287 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_288 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_289 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_290 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_291 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_292 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_293 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_294 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_295 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_296 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_297 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_298 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_299 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 17],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_300 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_301 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_302 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_303 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_304 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 20],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_305 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_306 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_307 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_308 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_309 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_310 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_311 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_312 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_313 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_314 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_315 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_316 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 16],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_317 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_318 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_319 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_320 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_321 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_322 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_323 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_324 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_325 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_326 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 14],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_327 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_328 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_329 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 17],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_330 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_331 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 23],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_332 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 18],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_333 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_334 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_335 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_336 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 23],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_337 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_338 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_339 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_340 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_341 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_342 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_343 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_344 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 18],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_345 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_346 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_347 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_348 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_349 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_350 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_351 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_352 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_353 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_354 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 18],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_355 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_356 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 15],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_357 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_358 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_359 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 18],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_360 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 13],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_361 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_362 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_363 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_364 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 19],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_365 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_366 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 13],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_367 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_368 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_369 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_370 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_371 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 16],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_372 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 13],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_373 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_374 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_375 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 14],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_376 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_377 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_378 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_379 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_380 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_381 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_382 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_383 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_384 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 17],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_385 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_386 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_387 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_388 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_389 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_390 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_391 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_392 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 14],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_393 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_394 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_395 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 15],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_396 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 13],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_397 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 14],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_398 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_399 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_400 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_401 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 18],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_402 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_403 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_404 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_405 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_406 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_407 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 15],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_408 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_409 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_410 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_411 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_412 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 13],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_413 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_414 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_415 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_416 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 14],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_417 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 16],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_418 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_419 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_420 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_421 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 15],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_422 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_423 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_424 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_425 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_426 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_427 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_428 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_429 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_430 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_431 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_432 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_433 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_434 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 19],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_435 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_436 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_437 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_438 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 13],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_439 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_440 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_441 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_442 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_443 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_444 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 3],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_445 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_446 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_447 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_448 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_449 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_450 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 13],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_451 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_452 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_453 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_454 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_455 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_456 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_457 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_458 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_459 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_460 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_461 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_462 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_463 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 16],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_464 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_465 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_466 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 13],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_467 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_468 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_469 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 14],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_470 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_471 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_472 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_473 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_474 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_475 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_476 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_477 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_478 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 15],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_479 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_480 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_481 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_482 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_483 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_484 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_485 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_486 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_487 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_488 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_489 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 13],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_490 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_491 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_492 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_493 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 3],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_494 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_495 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_496 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 3],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_497 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_498 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 14],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_499 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_500 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_501 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_502 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_503 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_504 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_505 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_506 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_507 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 15],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_508 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 15],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_509 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_510 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_511 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 14],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_512 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 14],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_513 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 14],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_514 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 14],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_515 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_516 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_517 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_518 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 16],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_519 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_520 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 13],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_521 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_522 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 13],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_523 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_524 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 3],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_525 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_526 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_527 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_528 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_529 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_530 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_531 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_532 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 16],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_533 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_534 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_535 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_536 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_537 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_538 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_539 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_540 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_541 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_542 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_543 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_544 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_545 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_546 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 3],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_547 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_548 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_549 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_550 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_551 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_552 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_553 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_554 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_555 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 15],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_556 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_557 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_558 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_559 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_560 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_561 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_562 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_563 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_564 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_565 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 15],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_566 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_567 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_568 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_569 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 3],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_570 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_571 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_572 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_573 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_574 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 13],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_575 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_576 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_577 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_578 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_579 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_580 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_581 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_582 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_583 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_584 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_585 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_586 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_587 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_588 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_589 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_590 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_591 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 13],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_592 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_593 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_594 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_595 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_596 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 15],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_597 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_598 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_599 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_600 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_601 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_602 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_603 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_604 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_605 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 18],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_606 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 17],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_607 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_608 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_609 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 20],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_610 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_611 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_612 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_613 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_614 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 14],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_615 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_616 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_617 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_618 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_619 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_620 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_621 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_622 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 3],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_623 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_624 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 3],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_625 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_626 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_627 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_628 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_629 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_630 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_631 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_632 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 13],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_633 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 13],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_634 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 19],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_635 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_636 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_637 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_638 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_639 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_640 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_641 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_642 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_643 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_644 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_645 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_646 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_647 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_648 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_649 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_650 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_651 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_652 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_653 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_654 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_655 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_656 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_657 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 14],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_658 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 16],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_659 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_660 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_661 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_662 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_663 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_664 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_665 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_666 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_667 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 20],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_668 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_669 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_670 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 17],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_671 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 17],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_672 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_673 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_674 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_675 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_676 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_677 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_678 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_679 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_680 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_681 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_682 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_683 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_684 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_685 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_686 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_687 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_688 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 17],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_689 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_690 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_691 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_692 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_693 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_694 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_695 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_696 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_697 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_698 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_699 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_700 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_701 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_702 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 15],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_703 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_704 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_705 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_706 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 20],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_707 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_708 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_709 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_710 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_711 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_712 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_713 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_714 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 16],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_715 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_716 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_717 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_718 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_719 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_720 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_721 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_722 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_723 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_724 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_725 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_726 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_727 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_728 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 19],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_729 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_730 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_731 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_732 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_733 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 17],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_734 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_735 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_736 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_737 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_738 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_739 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_740 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_741 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_742 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_743 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_744 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_745 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_746 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_747 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_748 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_749 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_750 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_751 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_752 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_753 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_754 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 28],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_755 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_756 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_757 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_758 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 20],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_759 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_760 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_761 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_762 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_763 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_764 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_765 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_766 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_767 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_768 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_769 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_770 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_771 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_772 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_773 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_774 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_775 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_776 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 15],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_777 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_778 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_779 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_780 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_781 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_782 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_783 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_784 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_785 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_786 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 3],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_787 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_788 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_789 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_790 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_791 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 15],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_792 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_793 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 16],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_794 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_795 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_796 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_797 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_798 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_799 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 20],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_800 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_801 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 12],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_802 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_803 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_804 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_805 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_806 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_807 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_808 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 9],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_809 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_810 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 14],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_811 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_812 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_813 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 10],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_815 {
    _ascii: __SlateRecord_unicodeobject_h_163,
    _data: [u8; 2],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_global_strings_h_819 {
    _latin1: __SlateRecord_unicodeobject_h_173,
    _data: [u8; 2],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _Py_global_strings {
    literals: __SlateRecord_pycore_global_strings_h_32,
    identifiers: __SlateRecord_pycore_global_strings_h_58,
    ascii: [__SlateRecord_pycore_global_strings_h_815; 128],
    latin1: [__SlateRecord_pycore_global_strings_h_819; 128],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_runtime_structs_h_115 {
    small_ints: [_longobject; 262],
    bytes_empty: __SlateRecord_bytesobject_h_15,
    bytes_characters: [__SlateRecord_pycore_runtime_structs_h_124; 256],
    strings: _Py_global_strings,
    _tuple_empty_gc_not_used: __SlateRecord_pycore_interp_structs_h_170,
    tuple_empty: __SlateRecord_tupleobject_h_13,
    _hamt_bitmap_node_empty_gc_not_used: __SlateRecord_pycore_interp_structs_h_170,
    hamt_bitmap_node_empty: __SlateRecord_pycore_structs_h_53,
    context_token_missing: __SlateRecord_pycore_context_h_21,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_runtime_structs_h_124 {
    ob: __SlateRecord_bytesobject_h_15,
    eos: i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct pyinterpreters {
    mutex: PyMutex,
    head: *mut _is,
    main: *mut _is,
    next_id: i64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _gilstate_runtime_state {
    check_enabled: i32,
    autoInterpreterState: *mut _is,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _getargs_runtime_state {
    static_parsers: *mut _PyArg_Parser,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_runtime_structs_h_271 {
    mutex: PyMutex,
    head: *mut _Py_AuditHookEntry,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _Py_AuditHookEntry {}

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
    pub struct __SlateBits184U0 {
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
    pub struct __SlateBits137U0 {
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

// These would be in pycore_long.h if it weren't for an include cycle.
#[repr(C)]
#[derive(Clone, Copy)]
struct _Py_static_objects {
    singletons: __SlateRecord_pycore_runtime_structs_h_115,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct pyruntimestate {
    debug_offsets: _Py_DebugOffsets,
    _initialized: i32,
    preinitializing: i32,
    preinitialized: i32,
    core_initialized: i32,
    initialized: i32,
    _finalizing: *mut _ts,
    _finalizing_id: u64,
    interpreters: pyinterpreters,
    main_thread: u64,
    main_tstate: *mut _ts,
    xi: __SlateRecord_pycore_crossinterp_h_255,
    allocators: _pymem_allocators,
    obmalloc: _obmalloc_global_state,
    pyhash_state: pyhash_runtime_state,
    threads: _pythread_runtime_state,
    signals: _signals_runtime_state,
    autoTSSkey: _Py_tss_t,
    trashTSSkey: _Py_tss_t,
    orig_argv: __SlateRecord_initconfig_h_36,
    parser: _parser_runtime_state,
    atexit: _atexit_runtime_state,
    imports: _import_runtime_state,
    ceval: _ceval_runtime_state,
    gilstate: _gilstate_runtime_state,
    getargs: _getargs_runtime_state,
    fileutils: _fileutils_state,
    faulthandler: _faulthandler_runtime_state,
    tracemalloc: _tracemalloc_runtime_state,
    ref_tracer: _reftracer_runtime_state,
    stoptheworld_mutex: __SlateRecord_pycore_lock_h_189,
    stoptheworld: _stoptheworld_state,
    preconfig: PyPreConfig,
    open_code_hook: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> *mut _object>,
    open_code_userdata: *mut (),
    audit_hooks: __SlateRecord_pycore_runtime_structs_h_271,
    object_state: _py_object_runtime_state,
    float_state: _Py_float_runtime_state,
    unicode_state: _Py_unicode_runtime_state,
    types: _types_runtime_state,
    time: _Py_time_runtime_state,
    cached_objects: _Py_cached_objects,
    static_objects: _Py_static_objects,
    _main_interpreter: _is,
}

// _PyOS_GetOpt()
// _PyArgv
// _Py_LegacyLocaleDetected()
// _PyMem_GetAllocatorName()
// _PyRuntime_Initialize()
// setlocale()
// getenv()
// --- File system encoding/errors --------------------------------
#[unsafe(no_mangle)]
static mut Py_FileSystemDefaultEncoding: *const i8 = core::ptr::null::<i8>();

#[unsafe(no_mangle)]
static mut Py_HasFileSystemDefaultEncoding: i32 = 0;

#[unsafe(no_mangle)]
static mut Py_FileSystemDefaultEncodeErrors: *const i8 = core::ptr::null::<i8>();

#[unsafe(no_mangle)]
static mut _Py_HasFileSystemDefaultEncodeErrors: i32 = 0;

#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_ClearFileSystemEncoding() {
    if (unsafe { Py_HasFileSystemDefaultEncoding }) == 0
        && !unsafe { Py_FileSystemDefaultEncoding }.is_null()
    {
        unsafe { PyMem_RawFree((Py_FileSystemDefaultEncoding as *mut i8) as *mut ()) };
        unsafe {
            Py_FileSystemDefaultEncoding = core::ptr::null::<i8>();
        }
    }
    if (unsafe { _Py_HasFileSystemDefaultEncodeErrors }) == 0
        && !unsafe { Py_FileSystemDefaultEncodeErrors }.is_null()
    {
        unsafe { PyMem_RawFree((Py_FileSystemDefaultEncodeErrors as *mut i8) as *mut ()) };
        unsafe {
            Py_FileSystemDefaultEncodeErrors = core::ptr::null::<i8>();
        }
    }
}

/// Set Py_FileSystemDefaultEncoding and Py_FileSystemDefaultEncodeErrors
/// global configuration variables to PyConfig.filesystem_encoding and
/// PyConfig.filesystem_errors (encoded to UTF-8).
///
/// Function called by _PyUnicode_InitEncodings().
#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_SetFileSystemEncoding(encoding: *const i8, errors: *const i8) -> i32 {
    let encoding2: *mut i8 = unsafe { _PyMem_RawStrdup(encoding) };
    if encoding2.is_null() {
        return 1i32.wrapping_neg();
    }
    let errors2: *mut i8 = unsafe { _PyMem_RawStrdup(errors) };
    if errors2.is_null() {
        unsafe { PyMem_RawFree(encoding2 as *mut ()) };
        return 1i32.wrapping_neg();
    }
    _Py_ClearFileSystemEncoding();
    unsafe {
        Py_FileSystemDefaultEncoding = encoding2 as *const i8;
    }
    unsafe {
        Py_HasFileSystemDefaultEncoding = 0;
    }
    unsafe {
        Py_FileSystemDefaultEncodeErrors = errors2 as *const i8;
    }
    unsafe {
        _Py_HasFileSystemDefaultEncodeErrors = 0;
    }
    0
}

// --- _PyArgv ----------------------------------------------------
/// Decode bytes_argv using Py_DecodeLocale()
#[unsafe(no_mangle)]
extern "C-unwind" fn _PyArgv_AsWstrList(
    args: *const _PyArgv,
    list: *mut __SlateRecord_initconfig_h_36,
) -> __SlateRecord_initconfig_h_19 {
    let mut __slate_compound_0: __SlateRecord_initconfig_h_36 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_1: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_2: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_3: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_9: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_10: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut wargv: __SlateRecord_initconfig_h_36 = unsafe {
        *{
            __slate_compound_0 = __SlateRecord_initconfig_h_36 {
                length: 0,
                items: core::ptr::null_mut::<*mut i32>(),
            };
            core::ptr::addr_of_mut!(__slate_compound_0)
        }
    };
    if (unsafe { (*args).use_bytes_argv }) != 0 {
        let size: u64 = (core::mem::size_of::<*mut i32>() as u64)
            .wrapping_mul((unsafe { (*args).argc }) as u64);
        wargv.items = (unsafe { PyMem_RawMalloc(size) }) as *mut *mut i32;
        if wargv.items.is_null() {
            return unsafe {
                *{
                    __slate_compound_1 = {
                        let mut __t0: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                        __t0._type = 1u32;
                        __t0.func = _PyStatus_GET_FUNC__5!() as *const i8;
                        __t0.err_msg = c"memory allocation failed".as_ptr();
                        __t0
                    };
                    core::ptr::addr_of_mut!(__slate_compound_1)
                }
            };
        }
        let mut i: i64 = 0;
        '__slate_break_6: while i < unsafe { (*args).argc } {
            let mut len: u64 = 0;
            let arg: *mut i32 = unsafe {
                Py_DecodeLocale(
                    *unsafe { (*args).bytes_argv }.wrapping_offset(i as isize) as *const i8,
                    core::ptr::addr_of_mut!(len),
                )
            };
            if arg.is_null() {
                unsafe { _PyWideStringList_Clear(core::ptr::addr_of_mut!(wargv)) };
                return if len == (2i32.wrapping_neg() as u64) {
                    unsafe {
                        *{
                            __slate_compound_2 = {
                                let mut __t1: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                                __t1._type = 1u32;
                                __t1.func = _PyStatus_GET_FUNC__5!() as *const i8;
                                __t1.err_msg = c"cannot decode command line arguments".as_ptr();
                                __t1
                            };
                            core::ptr::addr_of_mut!(__slate_compound_2)
                        }
                    }
                } else {
                    unsafe {
                        *{
                            __slate_compound_3 = {
                                let mut __t2: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                                __t2._type = 1u32;
                                __t2.func = _PyStatus_GET_FUNC__5!() as *const i8;
                                __t2.err_msg = c"memory allocation failed".as_ptr();
                                __t2
                            };
                            core::ptr::addr_of_mut!(__slate_compound_3)
                        }
                    }
                };
            }
            unsafe {
                *wargv.items.wrapping_offset(i as isize) = arg;
            }
            let __v4: i64 = wargv.length;
            wargv.length = __v4.wrapping_add(1);
            i = i.wrapping_add(1);
        }
        unsafe { _PyWideStringList_Clear(list) };
        unsafe {
            *list = wargv;
        }
    } else {
        wargv.length = unsafe { (*args).argc };
        wargv.items = (unsafe { (*args).wchar_argv }) as *mut *mut i32;
        if (unsafe {
            _PyWideStringList_Copy(
                list,
                core::ptr::addr_of_mut!(wargv) as *const __SlateRecord_initconfig_h_36,
            )
        }) < 0
        {
            return unsafe {
                *{
                    __slate_compound_9 = {
                        let mut __t3: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                        __t3._type = 1u32;
                        __t3.func = _PyStatus_GET_FUNC__5!() as *const i8;
                        __t3.err_msg = c"memory allocation failed".as_ptr();
                        __t3
                    };
                    core::ptr::addr_of_mut!(__slate_compound_9)
                }
            };
        }
    }
    return unsafe {
        *{
            __slate_compound_10 = {
                let mut __t4: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                __t4._type = 0u32;
                __t4
            };
            core::ptr::addr_of_mut!(__slate_compound_10)
        }
    };
}

use slate_support::getenv;
use slate_support::memset;
use slate_support::setlocale;
use slate_support::strcmp;
use slate_support::wcslen;

// --- _PyPreCmdline -------------------------------------------------
#[unsafe(no_mangle)]
extern "C-unwind" fn _PyPreCmdline_Clear(cmdline: *mut __SlateRecord_pycore_initconfig_h_108) {
    unsafe { _PyWideStringList_Clear(core::ptr::addr_of_mut!((*cmdline).argv)) };
    unsafe { _PyWideStringList_Clear(core::ptr::addr_of_mut!((*cmdline).xoptions)) };
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyPreCmdline_SetArgv(
    cmdline: *mut __SlateRecord_pycore_initconfig_h_108,
    args: *const _PyArgv,
) -> __SlateRecord_initconfig_h_19 {
    return _PyArgv_AsWstrList(args, unsafe { core::ptr::addr_of_mut!((*cmdline).argv) });
}

fn precmdline_get_preconfig(
    cmdline: *mut __SlateRecord_pycore_initconfig_h_108,
    config: *const PyPreConfig,
) {
    COPY_ATTR__4!(isolated, config, cmdline);
    COPY_ATTR__4!(use_environment, config, cmdline);
    COPY_ATTR__4!(dev_mode, config, cmdline);
}

fn precmdline_set_preconfig(
    cmdline: *const __SlateRecord_pycore_initconfig_h_108,
    config: *mut PyPreConfig,
) {
    COPY_ATTR__2!(isolated, config, cmdline);
    COPY_ATTR__2!(use_environment, config, cmdline);
    COPY_ATTR__2!(dev_mode, config, cmdline);
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyPreCmdline_SetConfig(
    cmdline: *const __SlateRecord_pycore_initconfig_h_108,
    config: *mut PyConfig,
) -> __SlateRecord_initconfig_h_19 {
    let mut __slate_compound_0: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let status: __SlateRecord_initconfig_h_19 = unsafe {
        _PyWideStringList_Extend(
            core::ptr::addr_of_mut!((*config).xoptions),
            core::ptr::addr_of!((*cmdline).xoptions),
        )
    };
    if _PyStatus_EXCEPTION!(status) {
        return status;
    }
    COPY_ATTR__2!(isolated, config, cmdline);
    COPY_ATTR__2!(use_environment, config, cmdline);
    COPY_ATTR__2!(dev_mode, config, cmdline);
    COPY_ATTR__2!(warn_default_encoding, config, cmdline);
    return unsafe {
        *{
            __slate_compound_0 = {
                let mut __t0: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                __t0._type = 0u32;
                __t0
            };
            core::ptr::addr_of_mut!(__slate_compound_0)
        }
    };
}

/// Parse the command line arguments
fn precmdline_parse_cmdline(
    cmdline: *mut __SlateRecord_pycore_initconfig_h_108,
) -> __SlateRecord_initconfig_h_19 {
    let mut __slate_compound_2: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let argv: *const __SlateRecord_initconfig_h_36 =
        (unsafe { core::ptr::addr_of_mut!((*cmdline).argv) })
            as *const __SlateRecord_initconfig_h_36;
    unsafe { _PyOS_ResetGetOpt() };
    // Don't log parsing errors into stderr here: PyConfig_Read()
    // is responsible for that
    unsafe {
        _PyOS_opterr = 0;
    }
    loop {
        let mut longindex: i32 = 1i32.wrapping_neg();
        let c: i32 = unsafe {
            _PyOS_GetOpt(
                (*argv).length,
                (*argv).items as *const *mut i32,
                core::ptr::addr_of_mut!(longindex),
            )
        };
        if c == (slate_support::EOF as i32) || c == 99 || c == 109 {
            break;
        }
        '__slate_break_1: {
            match c {
                69 => unsafe {
                    (*cmdline).use_environment = 0;
                },
                73 => unsafe {
                    (*cmdline).isolated = 1;
                },
                88 => {
                    let status: __SlateRecord_initconfig_h_19 = unsafe {
                        PyWideStringList_Append(
                            core::ptr::addr_of_mut!((*cmdline).xoptions),
                            _PyOS_optarg,
                        )
                    };
                    if _PyStatus_EXCEPTION!(status) {
                        return status;
                    }
                }
                _ => {
                    break '__slate_break_1;
                    // ignore other argument:
                    // handled by PyConfig_Read()
                }
            }
        }
        if false {
            break;
        }
    }
    return unsafe {
        *{
            __slate_compound_2 = {
                let mut __t0: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                __t0._type = 0u32;
                __t0
            };
            core::ptr::addr_of_mut!(__slate_compound_2)
        }
    };
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyPreCmdline_Read(
    cmdline: *mut __SlateRecord_pycore_initconfig_h_108,
    preconfig: *const PyPreConfig,
) -> __SlateRecord_initconfig_h_19 {
    let mut __slate_compound_3: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    precmdline_get_preconfig(cmdline, preconfig);
    if (unsafe { (*preconfig).parse_argv }) != 0 {
        let status: __SlateRecord_initconfig_h_19 = precmdline_parse_cmdline(cmdline);
        if _PyStatus_EXCEPTION!(status) {
            return status;
        }
    }
    // isolated, use_environment
    if (unsafe { (*cmdline).isolated }) < 0 {
        unsafe {
            (*cmdline).isolated = 0;
        }
    }
    if (unsafe { (*cmdline).isolated }) > 0 {
        unsafe {
            (*cmdline).use_environment = 0;
        }
    }
    if (unsafe { (*cmdline).use_environment }) < 0 {
        unsafe {
            (*cmdline).use_environment = 0;
        }
    }
    // dev_mode
    let __v0: bool = (unsafe { (*cmdline).dev_mode }) < 0
        && (!_Py_get_xoption(
            (unsafe { core::ptr::addr_of_mut!((*cmdline).xoptions) })
                as *const __SlateRecord_initconfig_h_36,
            (unsafe { core::ptr::addr_of_mut!(__slate_str1833) as *mut i32 }) as *const i32,
        )
        .is_null()
            || !_Py_GetEnv(
                unsafe { (*cmdline).use_environment },
                c"PYTHONDEVMODE".as_ptr(),
            )
            .is_null());
    if __v0 {
        unsafe {
            (*cmdline).dev_mode = 1;
        }
    }
    if (unsafe { (*cmdline).dev_mode }) < 0 {
        unsafe {
            (*cmdline).dev_mode = 0;
        }
    }
    // warn_default_encoding
    let __v2: bool = !_Py_get_xoption(
        (unsafe { core::ptr::addr_of_mut!((*cmdline).xoptions) })
            as *const __SlateRecord_initconfig_h_36,
        (unsafe { core::ptr::addr_of_mut!(__slate_str1835) as *mut i32 }) as *const i32,
    )
    .is_null()
        || !_Py_GetEnv(
            unsafe { (*cmdline).use_environment },
            c"PYTHONWARNDEFAULTENCODING".as_ptr(),
        )
        .is_null();
    if __v2 {
        unsafe {
            (*cmdline).warn_default_encoding = 1;
        }
    }
    return unsafe {
        *{
            __slate_compound_3 = {
                let mut __t0: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                __t0._type = 0u32;
                __t0
            };
            core::ptr::addr_of_mut!(__slate_compound_3)
        }
    };
}

static mut __slate_str1833: [i32; 4] = [100, 101, 118, 0];

static mut __slate_str1835: [i32; 22] = [
    119, 97, 114, 110, 95, 100, 101, 102, 97, 117, 108, 116, 95, 101, 110, 99, 111, 100, 105, 110,
    103, 0,
];

// --- PyPreConfig -----------------------------------------------
#[unsafe(no_mangle)]
extern "C-unwind" fn _PyPreConfig_InitCompatConfig(config: *mut PyPreConfig) {
    unsafe {
        core::ptr::write_bytes(
            ((config as *mut ()) as *mut core::ffi::c_void) as *mut u8,
            (0i32 as i32) as u8,
            ((core::mem::size_of::<PyPreConfig>() as u64) as usize) as usize,
        )
    };
    unsafe {
        (*config)._config_init = 1;
    }
    unsafe {
        (*config).parse_argv = 0;
    }
    unsafe {
        (*config).isolated = 1i32.wrapping_neg();
    }
    unsafe {
        (*config).use_environment = 1i32.wrapping_neg();
    }
    unsafe {
        (*config).configure_locale = 1;
    }
    // bpo-36443: C locale coercion (PEP 538) and UTF-8 Mode (PEP 540)
    // are disabled by default using the Compat configuration.
    //
    // Py_UTF8Mode=1 enables the UTF-8 mode. PYTHONUTF8 environment variable
    // is ignored (even if use_environment=1).
    unsafe {
        (*config).utf8_mode = 0;
    }
    unsafe {
        (*config).coerce_c_locale = 0;
    }
    unsafe {
        (*config).coerce_c_locale_warn = 0;
    }
    unsafe {
        (*config).dev_mode = 1i32.wrapping_neg();
    }
    unsafe {
        (*config).allocator = 0;
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyPreConfig_InitPythonConfig(config: *mut PyPreConfig) {
    _PyPreConfig_InitCompatConfig(config);
    unsafe {
        (*config)._config_init = 2;
    }
    unsafe {
        (*config).isolated = 0;
    }
    unsafe {
        (*config).parse_argv = 1;
    }
    unsafe {
        (*config).use_environment = 1;
    }
    // Set to -1 to enable C locale coercion (PEP 538) and UTF-8 Mode (PEP 540)
    // depending on the LC_CTYPE locale, PYTHONUTF8 and PYTHONCOERCECLOCALE
    // environment variables.
    unsafe {
        (*config).coerce_c_locale = 1i32.wrapping_neg();
    }
    unsafe {
        (*config).coerce_c_locale_warn = 1i32.wrapping_neg();
    }
    unsafe {
        (*config).utf8_mode = 1i32.wrapping_neg();
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyPreConfig_InitIsolatedConfig(config: *mut PyPreConfig) {
    _PyPreConfig_InitCompatConfig(config);
    unsafe {
        (*config)._config_init = 3;
    }
    unsafe {
        (*config).configure_locale = 0;
    }
    unsafe {
        (*config).isolated = 1;
    }
    unsafe {
        (*config).use_environment = 0;
    }
    unsafe {
        (*config).utf8_mode = 0;
    }
    unsafe {
        (*config).dev_mode = 0;
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyPreConfig_InitFromPreConfig(
    config: *mut PyPreConfig,
    config2: *const PyPreConfig,
) -> __SlateRecord_initconfig_h_19 {
    let mut __slate_compound_0: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    PyPreConfig_InitPythonConfig(config);
    preconfig_copy(config, config2);
    return unsafe {
        *{
            __slate_compound_0 = {
                let mut __t0: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                __t0._type = 0u32;
                __t0
            };
            core::ptr::addr_of_mut!(__slate_compound_0)
        }
    };
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyPreConfig_InitFromConfig(
    preconfig: *mut PyPreConfig,
    config: *const PyConfig,
) {
    let config_init: u32 = (unsafe { (*config)._config_init }) as u32;
    match config_init as u32 {
        2 => {
            PyPreConfig_InitPythonConfig(preconfig);
        }
        3 => {
            PyPreConfig_InitIsolatedConfig(preconfig);
        }
        _ => {
            _PyPreConfig_InitCompatConfig(preconfig);
        }
    }
    _PyPreConfig_GetConfig(preconfig, config);
}

/// Forward declarations
fn preconfig_copy(config: *mut PyPreConfig, config2: *const PyPreConfig) {
    COPY_ATTR!(_config_init, config, config2);
    COPY_ATTR!(parse_argv, config, config2);
    COPY_ATTR!(isolated, config, config2);
    COPY_ATTR!(use_environment, config, config2);
    COPY_ATTR!(configure_locale, config, config2);
    COPY_ATTR!(dev_mode, config, config2);
    COPY_ATTR!(coerce_c_locale, config, config2);
    COPY_ATTR!(coerce_c_locale_warn, config, config2);
    COPY_ATTR!(utf8_mode, config, config2);
    COPY_ATTR!(allocator, config, config2);
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyPreConfig_AsDict(config: *const PyPreConfig) -> *mut _object {
    let mut obj: *mut _object = unsafe { core::mem::zeroed() };
    let mut obj_1: *mut _object = unsafe { core::mem::zeroed() };
    let mut obj_2: *mut _object = unsafe { core::mem::zeroed() };
    let mut obj_3: *mut _object = unsafe { core::mem::zeroed() };
    let mut obj_4: *mut _object = unsafe { core::mem::zeroed() };
    let mut obj_5: *mut _object = unsafe { core::mem::zeroed() };
    let mut obj_6: *mut _object = unsafe { core::mem::zeroed() };
    let mut obj_7: *mut _object = unsafe { core::mem::zeroed() };
    let mut obj_8: *mut _object = unsafe { core::mem::zeroed() };
    let mut obj_9: *mut _object = unsafe { core::mem::zeroed() };
    let mut dict: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        dict = PyDict_New();
        if dict.is_null() {
            return core::ptr::null_mut::<_object>();
        } else {
            '__join_0: {
                loop {
                    obj_9 = PyLong_FromLong((*config)._config_init as i64);
                    if obj_9.is_null() {
                        break '__join_0;
                    } else {
                        let res: i32 = PyDict_SetItemString(dict, c"_config_init".as_ptr(), obj_9);
                        Py_DECREF!(obj_9);
                        if res < 0 {
                            break '__join_0;
                        } else {
                            if true {
                                break;
                            }
                        }
                    }
                }
                loop {
                    obj_8 = PyLong_FromLong((*config).parse_argv as i64);
                    if obj_8.is_null() {
                        break '__join_0;
                    } else {
                        let res: i32 = PyDict_SetItemString(dict, c"parse_argv".as_ptr(), obj_8);
                        Py_DECREF!(obj_8);
                        if res < 0 {
                            break '__join_0;
                        } else {
                            if true {
                                break;
                            }
                        }
                    }
                }
                loop {
                    obj_7 = PyLong_FromLong((*config).isolated as i64);
                    if obj_7.is_null() {
                        break '__join_0;
                    } else {
                        let res: i32 = PyDict_SetItemString(dict, c"isolated".as_ptr(), obj_7);
                        Py_DECREF!(obj_7);
                        if res < 0 {
                            break '__join_0;
                        } else {
                            if true {
                                break;
                            }
                        }
                    }
                }
                loop {
                    obj_6 = PyLong_FromLong((*config).use_environment as i64);
                    if obj_6.is_null() {
                        break '__join_0;
                    } else {
                        let res: i32 =
                            PyDict_SetItemString(dict, c"use_environment".as_ptr(), obj_6);
                        Py_DECREF!(obj_6);
                        if res < 0 {
                            break '__join_0;
                        } else {
                            if true {
                                break;
                            }
                        }
                    }
                }
                loop {
                    obj_5 = PyLong_FromLong((*config).configure_locale as i64);
                    if obj_5.is_null() {
                        break '__join_0;
                    } else {
                        let res: i32 =
                            PyDict_SetItemString(dict, c"configure_locale".as_ptr(), obj_5);
                        Py_DECREF!(obj_5);
                        if res < 0 {
                            break '__join_0;
                        } else {
                            if true {
                                break;
                            }
                        }
                    }
                }
                loop {
                    obj_4 = PyLong_FromLong((*config).coerce_c_locale as i64);
                    if obj_4.is_null() {
                        break '__join_0;
                    } else {
                        let res: i32 =
                            PyDict_SetItemString(dict, c"coerce_c_locale".as_ptr(), obj_4);
                        Py_DECREF!(obj_4);
                        if res < 0 {
                            break '__join_0;
                        } else {
                            if true {
                                break;
                            }
                        }
                    }
                }
                loop {
                    obj_3 = PyLong_FromLong((*config).coerce_c_locale_warn as i64);
                    if obj_3.is_null() {
                        break '__join_0;
                    } else {
                        let res: i32 =
                            PyDict_SetItemString(dict, c"coerce_c_locale_warn".as_ptr(), obj_3);
                        Py_DECREF!(obj_3);
                        if res < 0 {
                            break '__join_0;
                        } else {
                            if true {
                                break;
                            }
                        }
                    }
                }
                loop {
                    obj_2 = PyLong_FromLong((*config).utf8_mode as i64);
                    if obj_2.is_null() {
                        break '__join_0;
                    } else {
                        let res: i32 = PyDict_SetItemString(dict, c"utf8_mode".as_ptr(), obj_2);
                        Py_DECREF!(obj_2);
                        if res < 0 {
                            break '__join_0;
                        } else {
                            if true {
                                break;
                            }
                        }
                    }
                }
                loop {
                    obj_1 = PyLong_FromLong((*config).dev_mode as i64);
                    if obj_1.is_null() {
                        break '__join_0;
                    } else {
                        let res: i32 = PyDict_SetItemString(dict, c"dev_mode".as_ptr(), obj_1);
                        Py_DECREF!(obj_1);
                        if res < 0 {
                            break '__join_0;
                        } else {
                            if true {
                                break;
                            }
                        }
                    }
                }
                loop {
                    obj = PyLong_FromLong((*config).allocator as i64);
                    if obj.is_null() {
                        break '__join_0;
                    } else {
                        let res: i32 = PyDict_SetItemString(dict, c"allocator".as_ptr(), obj);
                        Py_DECREF!(obj);
                        if res < 0 {
                            break '__join_0;
                        } else {
                            if true {
                                break;
                            }
                        }
                    }
                }
                return dict;
            }
            Py_DECREF!(dict);
            return core::ptr::null_mut::<_object>();
        }
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyPreConfig_GetConfig(preconfig: *mut PyPreConfig, config: *const PyConfig) {
    COPY_ATTR__3!(parse_argv, config, preconfig);
    COPY_ATTR__3!(isolated, config, preconfig);
    COPY_ATTR__3!(use_environment, config, preconfig);
    COPY_ATTR__3!(dev_mode, config, preconfig);
}

fn preconfig_get_global_vars(config: *mut PyPreConfig) {
    if (unsafe { (*config)._config_init }) != 1 {
        // Python and Isolated configuration ignore global variables
        return;
    }
    COPY_FLAG__4!(isolated, unsafe { Py_IsolatedFlag }, config);
    COPY_NOT_FLAG__3!(use_environment, unsafe { Py_IgnoreEnvironmentFlag }, config);
    if (unsafe { Py_UTF8Mode }) > 0 {
        unsafe {
            (*config).utf8_mode = Py_UTF8Mode;
        }
    }
}

fn preconfig_set_global_vars(config: *const PyPreConfig) {
    COPY_FLAG__3!(isolated, Py_IsolatedFlag, config);
    COPY_NOT_FLAG__4!(use_environment, Py_IgnoreEnvironmentFlag, config);
    COPY_FLAG__3!(utf8_mode, Py_UTF8Mode, config);
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_GetEnv(use_environment: i32, name: *const i8) -> *const i8 {
    if use_environment == 0 {
        return core::ptr::null::<i8>();
    }
    let var: *const i8 =
        (unsafe { getenv(name as *const core::ffi::c_char) as *mut i8 }) as *const i8;
    if !var.is_null() && ((unsafe { *var.wrapping_offset(0 as isize) }) as i32) != 0 {
        return var;
    } else {
        return core::ptr::null::<i8>();
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_str_to_int(str: *const i8, result: *mut i32) -> i32 {
    let mut endptr: *const i8 = str;
    unsafe {
        *slate_support::errno_location() = 0;
    }
    let value: i64 = unsafe { strtol(str, core::ptr::addr_of_mut!(endptr) as *mut *mut i8, 10i32) };
    if ((unsafe { *endptr }) as i32) != 0
        || (unsafe { *slate_support::errno_location() }) == (slate_support::ERANGE as i32)
    {
        return 1i32.wrapping_neg();
    }
    if value < (2147483647i32.wrapping_neg().wrapping_sub(1) as i64) || value > 2147483647 {
        return 1i32.wrapping_neg();
    }
    unsafe {
        *result = value as i32;
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_get_env_flag(use_environment: i32, flag: *mut i32, name: *const i8) {
    let var: *const i8 = _Py_GetEnv(use_environment, name);
    if var.is_null() {
        return;
    }
    let mut value: i32 = 0;
    if _Py_str_to_int(var, core::ptr::addr_of_mut!(value)) < 0 || value < 0 {
        // PYTHONDEBUG=text and PYTHONDEBUG=-2 behave as PYTHONDEBUG=1
        value = 1;
    }
    if (unsafe { *flag }) < value {
        unsafe {
            *flag = value;
        }
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_get_xoption(
    xoptions: *const __SlateRecord_initconfig_h_36,
    name: *const i32,
) -> *const i32 {
    let mut i: i64 = 0;
    while i < unsafe { (*xoptions).length } {
        let option: *const i32 =
            (unsafe { *unsafe { (*xoptions).items }.wrapping_offset(i as isize) }) as *const i32;
        let mut len: u64 = 0;
        let sep: *const i32 = (unsafe { wcschr(option, 61i32) }) as *const i32;
        if !sep.is_null() {
            len = ((unsafe { sep.offset_from(option as *const i32) }) as i64) as u64;
        } else {
            len = unsafe { wcslen(option as *const i32) as u64 };
        }
        if (unsafe { wcsncmp(option, name, len) }) == 0
            && (unsafe { *name.wrapping_offset(len as isize) }) == 0
        {
            return option;
        }
        i = i.wrapping_add(1);
    }
    core::ptr::null::<i32>()
}

fn preconfig_init_utf8_mode(
    config: *mut PyPreConfig,
    cmdline: *const __SlateRecord_pycore_initconfig_h_108,
) -> __SlateRecord_initconfig_h_19 {
    let mut __slate_compound_0: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_1: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_2: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_3: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_4: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_5: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    if (unsafe { (*config).utf8_mode }) >= 0 {
        return unsafe {
            *{
                __slate_compound_0 = {
                    let mut __t0: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                    __t0._type = 0u32;
                    __t0
                };
                core::ptr::addr_of_mut!(__slate_compound_0)
            }
        };
    }
    let mut xopt: *const i32 = unsafe { core::mem::zeroed() };
    xopt = _Py_get_xoption(
        unsafe { core::ptr::addr_of!((*cmdline).xoptions) },
        (unsafe { core::ptr::addr_of_mut!(__slate_str1862) as *mut i32 }) as *const i32,
    );
    if !xopt.is_null() {
        let sep: *const i32 = (unsafe { wcschr(xopt, 61i32) }) as *const i32;
        if !sep.is_null() {
            xopt = sep.wrapping_offset(1 as isize);
            if (unsafe {
                wcscmp(
                    xopt,
                    (core::ptr::addr_of_mut!(__slate_str1863) as *mut i32) as *const i32,
                )
            }) == 0
            {
                unsafe {
                    (*config).utf8_mode = 1;
                }
            } else {
                if (unsafe {
                    wcscmp(
                        xopt,
                        (core::ptr::addr_of_mut!(__slate_str1864) as *mut i32) as *const i32,
                    )
                }) == 0
                {
                    unsafe {
                        (*config).utf8_mode = 0;
                    }
                } else {
                    return unsafe {
                        *{
                            __slate_compound_1 = {
                                let mut __t1: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                                __t1._type = 1u32;
                                __t1.func = _PyStatus_GET_FUNC__36!() as *const i8;
                                __t1.err_msg = c"invalid -X utf8 option value".as_ptr();
                                __t1
                            };
                            core::ptr::addr_of_mut!(__slate_compound_1)
                        }
                    };
                }
            }
        } else {
            unsafe {
                (*config).utf8_mode = 1;
            }
        }
        return unsafe {
            *{
                __slate_compound_2 = {
                    let mut __t2: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                    __t2._type = 0u32;
                    __t2
                };
                core::ptr::addr_of_mut!(__slate_compound_2)
            }
        };
    }
    let opt: *const i8 = _Py_GetEnv(unsafe { (*config).use_environment }, c"PYTHONUTF8".as_ptr());
    if !opt.is_null() {
        if (unsafe {
            strcmp(
                opt as *const core::ffi::c_char,
                c"1".as_ptr() as *const core::ffi::c_char,
            ) as i32
        }) == 0
        {
            unsafe {
                (*config).utf8_mode = 1;
            }
        } else {
            if (unsafe {
                strcmp(
                    opt as *const core::ffi::c_char,
                    c"0".as_ptr() as *const core::ffi::c_char,
                ) as i32
            }) == 0
            {
                unsafe {
                    (*config).utf8_mode = 0;
                }
            } else {
                return unsafe {
                    *{
                        __slate_compound_3 = {
                            let mut __t3: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                            __t3._type = 1u32;
                            __t3.func = _PyStatus_GET_FUNC__36!() as *const i8;
                            __t3.err_msg =
                                c"invalid PYTHONUTF8 environment variable value".as_ptr();
                            __t3
                        };
                        core::ptr::addr_of_mut!(__slate_compound_3)
                    }
                };
            }
        }
        return unsafe {
            *{
                __slate_compound_4 = {
                    let mut __t4: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                    __t4._type = 0u32;
                    __t4
                };
                core::ptr::addr_of_mut!(__slate_compound_4)
            }
        };
    }
    if (unsafe { (*config).utf8_mode }) < 0 {
        // The C locale and the POSIX locale enable the UTF-8 Mode (PEP 540)
        let ctype_loc: *const i8 = (unsafe {
            setlocale(
                0i32 as i32,
                core::ptr::null::<i8>() as *const core::ffi::c_char,
            ) as *mut i8
        }) as *const i8;
        if !ctype_loc.is_null()
            && ((unsafe {
                strcmp(
                    ctype_loc as *const core::ffi::c_char,
                    c"C".as_ptr() as *const core::ffi::c_char,
                ) as i32
            }) == 0
                || (unsafe {
                    strcmp(
                        ctype_loc as *const core::ffi::c_char,
                        c"POSIX".as_ptr() as *const core::ffi::c_char,
                    ) as i32
                }) == 0)
        {
            unsafe {
                (*config).utf8_mode = 1;
            }
        }
    }
    if (unsafe { (*config).utf8_mode }) < 0 {
        unsafe {
            (*config).utf8_mode = 0;
        }
    }
    return unsafe {
        *{
            __slate_compound_5 = {
                let mut __t5: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                __t5._type = 0u32;
                __t5
            };
            core::ptr::addr_of_mut!(__slate_compound_5)
        }
    };
}

static mut __slate_str1862: [i32; 5] = [117, 116, 102, 56, 0];

static mut __slate_str1863: [i32; 2] = [49, 0];

static mut __slate_str1864: [i32; 2] = [48, 0];

fn preconfig_init_coerce_c_locale(config: *mut PyPreConfig) {
    if (unsafe { (*config).configure_locale }) == 0 {
        unsafe {
            (*config).coerce_c_locale = 0;
        }
        unsafe {
            (*config).coerce_c_locale_warn = 0;
        }
        return;
    }
    let env: *const i8 = _Py_GetEnv(
        unsafe { (*config).use_environment },
        c"PYTHONCOERCECLOCALE".as_ptr(),
    );
    if !env.is_null() {
        if (unsafe {
            strcmp(
                env as *const core::ffi::c_char,
                c"0".as_ptr() as *const core::ffi::c_char,
            ) as i32
        }) == 0
        {
            if (unsafe { (*config).coerce_c_locale }) < 0 {
                unsafe {
                    (*config).coerce_c_locale = 0;
                }
            }
        } else {
            if (unsafe {
                strcmp(
                    env as *const core::ffi::c_char,
                    c"warn".as_ptr() as *const core::ffi::c_char,
                ) as i32
            }) == 0
            {
                if (unsafe { (*config).coerce_c_locale_warn }) < 0 {
                    unsafe {
                        (*config).coerce_c_locale_warn = 1;
                    }
                }
            } else {
                if (unsafe { (*config).coerce_c_locale }) < 0 {
                    unsafe {
                        (*config).coerce_c_locale = 1;
                    }
                }
            }
        }
    }
    // Test if coerce_c_locale equals to -1 or equals to 1:
    // PYTHONCOERCECLOCALE=1 doesn't imply that the C locale is always coerced.
    // It is only coerced if if the LC_CTYPE locale is "C".
    if (unsafe { (*config).coerce_c_locale }) < 0 || (unsafe { (*config).coerce_c_locale }) == 1 {
        // The C locale enables the C locale coercion (PEP 538)
        if (unsafe { _Py_LegacyLocaleDetected(0i32) }) != 0 {
            unsafe {
                (*config).coerce_c_locale = 2;
            }
        } else {
            unsafe {
                (*config).coerce_c_locale = 0;
            }
        }
    }
    if (unsafe { (*config).coerce_c_locale_warn }) < 0 {
        unsafe {
            (*config).coerce_c_locale_warn = 0;
        }
    }
}

fn preconfig_init_allocator(config: *mut PyPreConfig) -> __SlateRecord_initconfig_h_19 {
    let mut __slate_compound_0: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_1: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    if (unsafe { (*config).allocator }) == 0 {
        // bpo-34247. The PYTHONMALLOC environment variable has the priority
        // over PYTHONDEV env var and "-X dev" command line option.
        // For example, PYTHONMALLOC=malloc PYTHONDEVMODE=1 sets the memory
        // allocators to "malloc" (and not to "debug").
        let envvar: *const i8 = _Py_GetEnv(
            unsafe { (*config).use_environment },
            c"PYTHONMALLOC".as_ptr(),
        );
        if !envvar.is_null() {
            let mut name: u32 = 0;
            if (unsafe { _PyMem_GetAllocatorName(envvar, core::ptr::addr_of_mut!(name)) }) < 0 {
                return unsafe {
                    *{
                        __slate_compound_0 = {
                            let mut __t0: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                            __t0._type = 1u32;
                            __t0.func = _PyStatus_GET_FUNC__51!() as *const i8;
                            __t0.err_msg = c"PYTHONMALLOC: unknown allocator".as_ptr();
                            __t0
                        };
                        core::ptr::addr_of_mut!(__slate_compound_0)
                    }
                };
            }
            unsafe {
                (*config).allocator = (name as u32) as i32;
            }
        }
    }
    if (unsafe { (*config).dev_mode }) != 0 && (unsafe { (*config).allocator }) == 0 {
        unsafe {
            (*config).allocator = 2;
        }
    }
    return unsafe {
        *{
            __slate_compound_1 = {
                let mut __t1: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                __t1._type = 0u32;
                __t1
            };
            core::ptr::addr_of_mut!(__slate_compound_1)
        }
    };
}

fn preconfig_read(
    config: *mut PyPreConfig,
    cmdline: *mut __SlateRecord_pycore_initconfig_h_108,
) -> __SlateRecord_initconfig_h_19 {
    let mut __slate_compound_0: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut status: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    status = _PyPreCmdline_Read(cmdline, config as *const PyPreConfig);
    if _PyStatus_EXCEPTION!(status) {
        return status;
    }
    precmdline_set_preconfig(
        cmdline as *const __SlateRecord_pycore_initconfig_h_108,
        config,
    );
    // legacy_windows_fs_encoding, coerce_c_locale, utf8_mode
    preconfig_init_coerce_c_locale(config);
    status = preconfig_init_utf8_mode(
        config,
        cmdline as *const __SlateRecord_pycore_initconfig_h_108,
    );
    if _PyStatus_EXCEPTION!(status) {
        return status;
    }
    // allocator
    status = preconfig_init_allocator(config);
    if _PyStatus_EXCEPTION!(status) {
        return status;
    }
    return unsafe {
        *{
            __slate_compound_0 = {
                let mut __t0: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                __t0._type = 0u32;
                __t0
            };
            core::ptr::addr_of_mut!(__slate_compound_0)
        }
    };
}

/// Read the configuration from:
///
/// - command line arguments
/// - environment variables
/// - Py_xxx global configuration variables
/// - the LC_CTYPE locale
#[unsafe(no_mangle)]
extern "C-unwind" fn _PyPreConfig_Read(
    config: *mut PyPreConfig,
    args: *const _PyArgv,
) -> __SlateRecord_initconfig_h_19 {
    let mut __slate_compound_14: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_15: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_16: __SlateRecord_pycore_initconfig_h_108 =
        unsafe { core::mem::zeroed() };
    let mut __slate_compound_17: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_18: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    // The legacy C locale assumes ASCII as the default text encoding, which
    // causes problems not only for the CPython runtime, but also other
    // components like GNU readline.
    //
    // Accordingly, when the CLI detects it, it attempts to coerce it to a
    // more capable UTF-8 based alternative.
    //
    // See the documentation of the PYTHONCOERCECLOCALE setting for more
    // details.
    let mut encoding_changed: i32 = unsafe { core::mem::zeroed() };
    let mut utf8_mode: i32 = unsafe { core::mem::zeroed() };
    let mut loops: i32 = unsafe { core::mem::zeroed() };
    let mut locale_coerced: i32 = unsafe { core::mem::zeroed() };
    let mut cmdline: core::mem::MaybeUninit<__SlateRecord_pycore_initconfig_h_108> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_8: *mut __SlateRecord_pycore_initconfig_h_108 =
        core::ptr::addr_of_mut!(cmdline) as *mut __SlateRecord_pycore_initconfig_h_108;
    let mut save_runtime_config: core::mem::MaybeUninit<PyPreConfig> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_9: *mut PyPreConfig =
        core::ptr::addr_of_mut!(save_runtime_config) as *mut PyPreConfig;
    // Save the config to be able to restore it if encodings change
    let mut save_config: core::mem::MaybeUninit<PyPreConfig> = core::mem::MaybeUninit::uninit();
    let __slate_slot_10: *mut PyPreConfig =
        core::ptr::addr_of_mut!(save_config) as *mut PyPreConfig;
    let mut init_ctype_locale: *mut i8 = unsafe { core::mem::zeroed() };
    // Copy LC_CTYPE locale, since it's modified later
    let mut loc: *const i8 = unsafe { core::mem::zeroed() };
    let mut status: core::mem::MaybeUninit<__SlateRecord_initconfig_h_19> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_13: *mut __SlateRecord_initconfig_h_19 =
        core::ptr::addr_of_mut!(status) as *mut __SlateRecord_initconfig_h_19;
    unsafe {
        *__slate_slot_13 = _PyRuntime_Initialize();
        if _PyStatus_EXCEPTION!(*__slate_slot_13) {
            return *__slate_slot_13;
        } else {
            preconfig_get_global_vars(config);
            loc = (setlocale(
                0i32 as i32,
                core::ptr::null::<i8>() as *const core::ffi::c_char,
            ) as *mut i8) as *const i8;
            if loc.is_null() {
                return *{
                    __slate_compound_18 = {
                        let mut __t4: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                        __t4._type = 1u32;
                        __t4.func = _PyStatus_GET_FUNC__8!() as *const i8;
                        __t4.err_msg = c"failed to LC_CTYPE locale".as_ptr();
                        __t4
                    };
                    core::ptr::addr_of_mut!(__slate_compound_18)
                };
            } else {
                init_ctype_locale = _PyMem_RawStrdup(loc);
                if init_ctype_locale.is_null() {
                    return *{
                        __slate_compound_17 = {
                            let mut __t3: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                            __t3._type = 1u32;
                            __t3.func = _PyStatus_GET_FUNC__8!() as *const i8;
                            __t3.err_msg = c"memory allocation failed".as_ptr();
                            __t3
                        };
                        core::ptr::addr_of_mut!(__slate_compound_17)
                    };
                } else {
                    *__slate_slot_13 = _PyPreConfig_InitFromPreConfig(
                        core::ptr::addr_of_mut!(*__slate_slot_10),
                        config as *const PyPreConfig,
                    );
                    if _PyStatus_EXCEPTION!(*__slate_slot_13) {
                        return *__slate_slot_13;
                    } else {
                        // Set LC_CTYPE to the user preferred locale
                        if (*config).configure_locale != 0 {
                            _Py_SetLocaleFromEnv(0i32);
                        }
                        preconfig_copy(
                            core::ptr::addr_of_mut!(*__slate_slot_9),
                            core::ptr::addr_of_mut!(_PyRuntime.preconfig) as *const PyPreConfig,
                        );
                        core::ptr::write(__slate_slot_8, *{
                            __slate_compound_16 = {
                                let mut __t2: __SlateRecord_pycore_initconfig_h_108 =
                                    core::mem::zeroed();
                                __t2.isolated = 1i32.wrapping_neg();
                                __t2.use_environment = 1i32.wrapping_neg();
                                __t2.dev_mode = 1i32.wrapping_neg();
                                __t2
                            };
                            core::ptr::addr_of_mut!(__slate_compound_16)
                        });
                        locale_coerced = 0;
                        loops = 0;
                        '__join_0: {
                            '__join_15: {
                                while true {
                                    utf8_mode = (*config).utf8_mode;
                                    // Watchdog to prevent an infinite loop
                                    loops = loops.wrapping_add(1);
                                    if loops == 3 {
                                        break '__join_15;
                                    } else {
                                        // bpo-34207: Py_DecodeLocale() and Py_EncodeLocale() depend
                                        // on the utf8_mode and legacy_windows_fs_encoding members
                                        // of _PyRuntime.preconfig.
                                        preconfig_copy(
                                            core::ptr::addr_of_mut!(_PyRuntime.preconfig),
                                            config as *const PyPreConfig,
                                        );
                                        if !args.is_null() {
                                            // Set command line arguments at each iteration. If they are bytes
                                            //
                                            // strings, they are decoded from the new encoding.
                                            *__slate_slot_13 = _PyPreCmdline_SetArgv(
                                                core::ptr::addr_of_mut!(*__slate_slot_8),
                                                args,
                                            );
                                            if _PyStatus_EXCEPTION!(*__slate_slot_13) {
                                                break '__join_0;
                                            }
                                        }
                                        *__slate_slot_13 = preconfig_read(
                                            config,
                                            core::ptr::addr_of_mut!(*__slate_slot_8),
                                        );
                                        if _PyStatus_EXCEPTION!(*__slate_slot_13) {
                                            break '__join_0;
                                        } else {
                                            encoding_changed = 0;
                                            if (*config).coerce_c_locale != 0 && locale_coerced == 0
                                            {
                                                locale_coerced = 1;
                                                _Py_CoerceLegacyLocale(0i32);
                                                encoding_changed = 1;
                                            }
                                            if utf8_mode == 1i32.wrapping_neg() {
                                                if (*config).utf8_mode == 1 {
                                                    // UTF-8 Mode enabled
                                                    encoding_changed = 1;
                                                }
                                            } else {
                                                if (*config).utf8_mode != utf8_mode {
                                                    encoding_changed = 1;
                                                }
                                            }
                                            if encoding_changed == 0 {
                                                break;
                                            } else {
                                                // Reset the configuration before reading again the configuration,
                                                // just keep UTF-8 Mode and coerce C locale value.
                                                let new_utf8_mode: i32 = (*config).utf8_mode;
                                                let new_coerce_c_locale: i32 =
                                                    (*config).coerce_c_locale;
                                                preconfig_copy(
                                                    config,
                                                    core::ptr::addr_of_mut!(*__slate_slot_10)
                                                        as *const PyPreConfig,
                                                );
                                                (*config).utf8_mode = new_utf8_mode;
                                                (*config).coerce_c_locale = new_coerce_c_locale;
                                                // The encoding changed: read again the configuration
                                                // with the new encoding
                                            }
                                        }
                                    }
                                }
                                *__slate_slot_13 = *{
                                    __slate_compound_14 = {
                                        let mut __t0: __SlateRecord_initconfig_h_19 =
                                            core::mem::zeroed();
                                        __t0._type = 0u32;
                                        __t0
                                    };
                                    core::ptr::addr_of_mut!(__slate_compound_14)
                                };
                                break '__join_0;
                            }
                            *__slate_slot_13 = *{
                                __slate_compound_15 = {
                                    let mut __t1: __SlateRecord_initconfig_h_19 =
                                        core::mem::zeroed();
                                    __t1._type = 1u32;
                                    __t1.func = _PyStatus_GET_FUNC__8!() as *const i8;
                                    __t1.err_msg =
                                        c"Encoding changed twice while reading the configuration"
                                            .as_ptr();
                                    __t1
                                };
                                core::ptr::addr_of_mut!(__slate_compound_15)
                            };
                        }
                        setlocale(
                            0i32 as i32,
                            (init_ctype_locale as *const i8) as *const core::ffi::c_char,
                        ) as *mut i8; // Revert side effects
                        PyMem_RawFree(init_ctype_locale as *mut ());
                        preconfig_copy(
                            core::ptr::addr_of_mut!(_PyRuntime.preconfig),
                            core::ptr::addr_of_mut!(*__slate_slot_9) as *const PyPreConfig,
                        );
                        _PyPreCmdline_Clear(core::ptr::addr_of_mut!(*__slate_slot_8));
                        return *__slate_slot_13;
                    }
                }
            }
        }
    }
}

/// Write the pre-configuration:
///
/// - set the memory allocators
/// - set Py_xxx global configuration variables
/// - set the LC_CTYPE locale (coerce C locale, PEP 538) and set the UTF-8 mode
///   (PEP 540)
///
/// The applied configuration is written into _PyRuntime.preconfig.
/// If the C locale cannot be coerced, set coerce_c_locale to 0.
///
/// Do nothing if called after Py_Initialize(): ignore the new
/// pre-configuration.
#[unsafe(no_mangle)]
extern "C-unwind" fn _PyPreConfig_Write(
    src_config: *const PyPreConfig,
) -> __SlateRecord_initconfig_h_19 {
    let mut __slate_compound_0: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_1: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_2: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut config: PyPreConfig = unsafe { core::mem::zeroed() };
    let status: __SlateRecord_initconfig_h_19 =
        _PyPreConfig_InitFromPreConfig(core::ptr::addr_of_mut!(config), src_config);
    if _PyStatus_EXCEPTION!(status) {
        return status;
    }
    if (unsafe { _PyRuntime.core_initialized }) != 0 {
        // bpo-34008: Calling this functions after Py_Initialize() ignores
        // the new configuration.
        return unsafe {
            *{
                __slate_compound_0 = {
                    let mut __t0: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                    __t0._type = 0u32;
                    __t0
                };
                core::ptr::addr_of_mut!(__slate_compound_0)
            }
        };
    }
    let name: u32 = config.allocator as u32;
    if (name as u32) != 0 {
        if (unsafe { _PyMem_SetupAllocators(name) }) < 0 {
            return unsafe {
                *{
                    __slate_compound_1 = {
                        let mut __t1: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                        __t1._type = 1u32;
                        __t1.func = _PyStatus_GET_FUNC__58!() as *const i8;
                        __t1.err_msg = c"Unknown PYTHONMALLOC allocator".as_ptr();
                        __t1
                    };
                    core::ptr::addr_of_mut!(__slate_compound_1)
                }
            };
        }
    }
    preconfig_set_global_vars(core::ptr::addr_of_mut!(config) as *const PyPreConfig);
    if config.configure_locale != 0 {
        if config.coerce_c_locale != 0 {
            if (unsafe { _Py_CoerceLegacyLocale(config.coerce_c_locale_warn) }) == 0 {
                // C locale not coerced
                config.coerce_c_locale = 0;
            }
        }
        // Set LC_CTYPE to the user preferred locale
        unsafe { _Py_SetLocaleFromEnv(0i32) };
    }
    // Write the new pre-configuration into _PyRuntime
    preconfig_copy(
        unsafe { core::ptr::addr_of_mut!(_PyRuntime.preconfig) },
        core::ptr::addr_of_mut!(config) as *const PyPreConfig,
    );
    return unsafe {
        *{
            __slate_compound_2 = {
                let mut __t2: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                __t2._type = 0u32;
                __t2
            };
            core::ptr::addr_of_mut!(__slate_compound_2)
        }
    };
}
