//! The size of 'void *', as computed by sizeof.
//! Use static_assert() on C11 and newer
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
//! This limitation is for performance and simplicity. If needed it can be
//! removed (with effort).
//! Remote debugger support
//! _Py_popcount32()
//! Chosen so that _PyObjectStackChunk is a power-of-two size.
//! Largest tuple to save on freelist
//! leads to warnings... __attribute__((cdecl))
//! Various defines for all used memory orders in mimalloc
//! Main tuning parameters for segment and page sizes
//!
//! Sizes for 64-bit (usually divide by two for 32-bit)
//! Inlined definitions
//! These are defined in PEP 669 for convenience to avoid clashes
//! Bit flags for _gc_prev
//!
//! Bit 0 is set when tp_finalize is called
//! Fast inlined version of PyType_IS_GC()
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
// Define macros for inline documentation.
// Round up size "n" to be a multiple of "a".
// PY_TIMEOUT_T is the integral type used to specify timeouts when waiting
//
// on a lock (see PyThread_acquire_lock_timed() below).
// 64KiB  (32KiB on 32-bit)
// 1024
// Used as a special value to encode block sizes in 32 bits.
// 35 == mi_segment_bin(MI_SLICES_PER_SEGMENT)
// initialize a local variable to zero; use memset as compilers optimize constant sized memset's
macro_rules! BITS_TO_PTR_MASKED {
    ($REF:expr) => {
        ((unsafe { $REF.bits }) & 18446744073709551614u64) as *mut _object
    };
}

macro_rules! CHECK__2 {
    () => {};
}

macro_rules! FT_ATOMIC_STORE_UINT8_RELAXED {
    ($value:expr, $new_value:expr) => {
        unsafe {
            $value = ($new_value) as u8;
        }
    };
}

macro_rules! FT_ATOMIC_STORE_UINT8 {
    ($value:expr, $new_value:expr) => {
        unsafe {
            $value = ($new_value) as u8;
        }
    };
}

macro_rules! FT_ATOMIC_STORE_UINTPTR_RELEASE {
    ($value:expr, $new_value:expr) => {
        unsafe {
            $value = ($new_value) as u64;
        }
    };
}

macro_rules! HEAD_LOCK {
    ($runtime:expr) => {
        PyMutex_LockFlags(
            unsafe { core::ptr::addr_of_mut!((*$runtime).interpreters.mutex) },
            0u32,
        );
    };
}

macro_rules! HEAD_UNLOCK {
    ($runtime:expr) => {
        _PyMutex_Unlock(unsafe { core::ptr::addr_of_mut!((*$runtime).interpreters.mutex) });
    };
}

macro_rules! IS_CONDITIONAL_JUMP_OPCODE {
    ($opcode:expr) => {
        (($opcode) as i32) == 100
            || (($opcode) as i32) == 103
            || (($opcode) as i32) == 101
            || (($opcode) as i32) == 102
    };
}

macro_rules! PY_MONITORING_IS_INSTRUMENTED_EVENT {
    ($ev:expr) => {
        ($ev) < 11
    };
}

macro_rules! PyBytes_AS_STRING {
    ($op:expr) => {
        PyBytes_AS_STRING(_PyObject_CAST!($op))
    };
}

macro_rules! PyBytes_GET_SIZE {
    ($self:expr) => {
        PyBytes_GET_SIZE(_PyObject_CAST!($self))
    };
}

macro_rules! PyCode_Check {
    ($op:expr) => {
        Py_IS_TYPE!($op, unsafe { core::ptr::addr_of_mut!(PyCode_Type) })
    };
}

/// Alias to PyObject_New(). In Python 3.8, PyObject_NEW() called directly
///
/// PyObject_MALLOC() with _PyObject_SIZE().
macro_rules! PyObject_NEW {
    ($type:ty, $typeobj:expr) => {
        PyObject_New!($type, $typeobj)
    };
}

macro_rules! PyObject_New {
    ($type:ty, $typeobj:expr) => {
        (unsafe { _PyObject_New($typeobj) }) as *mut $type
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

macro_rules! PyVectorcall_NARGS {
    ($n:expr) => {
        _PyVectorcall_NARGS($n)
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

macro_rules! Py_INCREF__2 {
    ($op:expr) => {
        Py_INCREF(_PyObject_CAST__2!($op));
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

macro_rules! Py_NewRef__2 {
    ($obj:expr) => {
        _Py_NewRef(_PyObject_CAST__2!($obj))
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

/// Macros and static inline functions, trading safety for speed
macro_rules! _PyBytes_CAST {
    ($op:expr) => {
        _Py_CAST!(*mut __SlateRecord_bytesobject_h_15, $op)
    };
}

macro_rules! _PyCode_CODE {
    ($CO:expr) => {
        _Py_RVALUE!(
            (unsafe { (*$CO).co_code_adaptive.as_mut_ptr() as *mut i8 })
                as *mut __SlateRecord_pycore_structs_h_32
        )
    };
}

macro_rules! _PyLegacyBranchEventHandler_CAST {
    ($op:expr) => {
        ($op) as *mut _PyLegacyBranchEventHandler
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

/// Cast argument to PyVarObject* type.
macro_rules! _PyVarObject_CAST {
    ($op:expr) => {
        _Py_CAST!(*mut __SlateRecord_object_h_172, $op)
    };
}

macro_rules! _Py_ANY_VARARGS {
    ($n:expr) => {
        (($n) as i64) == 9223372036854775807
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

macro_rules! _Py_FOR_EACH_TSTATE_END {
    ($interp:expr) => {
        HEAD_UNLOCK!(unsafe { (*$interp).runtime });
    };
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

/// Prevent using an expression as a l-value.
///
/// For example, "int x; _Py_RVALUE(x) = 1;" fails with a compiler error.
macro_rules! _Py_RVALUE {
    ($EXPR:expr) => {
        $EXPR
    };
}

macro_rules! _Py_STATIC_CAST {
    ($type:ty, $expr:expr) => {
        ($expr) as $type
    };
}

macro_rules! branchesiterator_CAST {
    ($op:expr) => {
        ($op) as *mut __SlateRecord_instrumentation_c_3074
    };
}

unsafe extern "C" {
    static mut PyType_Type: _typeobject;
    static mut PyBaseObject_Type: _typeobject;
    static mut _Py_NoneStruct: _object;
    static mut PyExc_StopIteration: *mut _object;
    static mut PyExc_OverflowError: *mut _object;
    static mut PyExc_SystemError: *mut _object;
    static mut PyExc_TypeError: *mut _object;
    static mut PyExc_ValueError: *mut _object;
    static mut PyCode_Type: _typeobject;
    #[thread_local]
    static mut _Py_tss_tstate: *mut _ts;
    static mut _PyRuntime: pyruntimestate;
    static mut _PyOpcode_Caches: [u8; 256];
    static mut _PyOpcode_Deopt: [u8; 256];
    fn index(arg0: *const i8, arg1: i32) -> *mut i8;
    fn PyMem_Malloc(arg0: u64) -> *mut ();
    fn PyMutex_Unlock(arg0: *mut PyMutex);
    fn PyType_GenericAlloc(arg0: *mut _typeobject, arg1: i64) -> *mut _object;
    fn PyObject_SetAttrString(arg0: *mut _object, arg1: *const i8, arg2: *mut _object) -> i32;
    fn PyObject_SelfIter(arg0: *mut _object) -> *mut _object;
    fn _Py_Dealloc(arg0: *mut _object);
    fn PyObject_Free(arg0: *mut ());
    fn _PyObject_New(arg0: *mut _typeobject) -> *mut _object;
    fn PyErr_SetObject(arg0: *mut _object, arg1: *mut _object);
    fn PyErr_SetString(arg0: *mut _object, arg1: *const i8);
    fn PyErr_Occurred() -> *mut _object;
    fn PyErr_GetRaisedException() -> *mut _object;
    fn PyErr_SetRaisedException(arg0: *mut _object);
    fn PyErr_NoMemory() -> *mut _object;
    fn PyErr_Format(arg0: *mut _object, arg1: *const i8, ...) -> *mut _object;
    fn PyLong_FromLong(arg0: i64) -> *mut _object;
    fn PyLong_AsLong(arg0: *mut _object) -> i64;
    fn PyLong_AsInt(arg0: *mut _object) -> i32;
    fn PyDict_New() -> *mut _object;
    fn PyDict_SetItemString(arg0: *mut _object, arg1: *const i8, arg2: *mut _object) -> i32;
    fn _PyCode_CheckLineNumber(arg0: i32, arg1: *mut _line_offsets) -> i32;
    fn PySys_Audit(arg0: *const i8, arg1: *const i8, ...) -> i32;
    fn PyVectorcall_Call(
        arg0: *mut _object,
        arg1: *mut _object,
        arg2: *mut _object,
    ) -> *mut _object;
    fn PyObject_Vectorcall(
        arg0: *mut _object,
        arg1: *const *mut _object,
        arg2: u64,
        arg3: *mut _object,
    ) -> *mut _object;
    fn _PyCode_InitAddressRange(arg0: *mut PyCodeObject, arg1: *mut _line_offsets) -> i32;
    fn _PyEval_StopTheWorld(arg0: *mut _is);
    fn _PyEval_StartTheWorld(arg0: *mut _is);
    fn _Py_CheckFunctionResult(
        arg0: *mut _ts,
        arg1: *mut _object,
        arg2: *mut _object,
        arg3: *const i8,
    ) -> *mut _object;
    fn _PyObject_MakeTpCall(
        arg0: *mut _ts,
        arg1: *mut _object,
        arg2: *const *mut _object,
        arg3: i64,
        arg4: *mut _object,
    ) -> *mut _object;
    fn _PyMutex_LockTimed(arg0: *mut PyMutex, arg1: i64, arg2: u32) -> u32;
    fn _PyErr_GetRaisedException(arg0: *mut _ts) -> *mut _object;
    fn _PyErr_SetRaisedException(arg0: *mut _ts, arg1: *mut _object);
    fn _PyFrame_MakeAndSetFrameObject(arg0: *mut _PyInterpreterFrame) -> *mut _frame;
    fn _PyArg_CheckPositional(arg0: *const i8, arg1: i64, arg2: i64, arg3: i64) -> i32;
    fn _PyModule_CreateInitialized(arg0: *mut PyModuleDef, arg1: i32) -> *mut _object;
    fn _PyNamespace_New(arg0: *mut _object) -> *mut _object;
    fn _PyTuple_FromArraySteal(arg0: *const *mut _object, arg1: i64) -> *mut _object;
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

fn _Py_atomic_compare_exchange_uint8(obj: *mut u8, expected: &mut u8, desired: u8) -> i32 {
    let __v0: bool = unsafe {
        let __t0 = expected as *mut u8;
        let __t1 = desired;
        let __t2 = core::sync::atomic::AtomicU8::from_ptr(obj as *mut u8).compare_exchange(
            *__t0,
            __t1,
            core::sync::atomic::Ordering::SeqCst,
            core::sync::atomic::Ordering::SeqCst,
        );
        match __t2 {
            Ok(_) => true,
            Err(__t3) => {
                *__t0 = __t3;
                false
            }
        }
    };
    __v0 as i32
}

fn _Py_atomic_compare_exchange_uintptr(obj: *mut u64, expected: &mut u64, desired: u64) -> i32 {
    let __v0: bool = unsafe {
        let __t0 = expected as *mut u64;
        let __t1 = desired;
        let __t2 = core::sync::atomic::AtomicU64::from_ptr(obj as *mut u64).compare_exchange(
            *__t0,
            __t1,
            core::sync::atomic::Ordering::SeqCst,
            core::sync::atomic::Ordering::SeqCst,
        );
        match __t2 {
            Ok(_) => true,
            Err(__t3) => {
                *__t0 = __t3;
                false
            }
        }
    };
    __v0 as i32
}

fn _Py_atomic_load_uintptr_relaxed(obj: *const u64) -> u64 {
    unsafe {
        core::sync::atomic::AtomicU64::from_ptr(obj as *mut u64)
            .load(core::sync::atomic::Ordering::Relaxed)
    }
}

fn _Py_atomic_store_ptr_release(obj: *mut (), value: *mut ()) {
    unsafe {
        core::sync::atomic::AtomicPtr::from_ptr(obj as *mut *mut ())
            .store(value as *mut (), core::sync::atomic::Ordering::Release)
    };
}

#[repr(C)]
#[derive(Clone, Copy)]
struct PyMutex {
    _bits: u8,
}

fn _PyMutex_Unlock(m: *mut PyMutex) {
    let mut expected: u8 = 1;
    if _Py_atomic_compare_exchange_uint8(
        unsafe { core::ptr::addr_of_mut!((*m)._bits) },
        unsafe { &mut (*core::ptr::addr_of_mut!(expected)) },
        0u8,
    ) == 0
    {
        unsafe { PyMutex_Unlock(m) };
    }
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

// Track types initialized using _PyStaticType_InitBuiltin().
// Disallow creating instances of the type: set tp_new to NULL and don't create
// the "__new__" key in the type dictionary.
// Set if the type allows subclassing
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
    __slate_bits_0: __slate_bits::__SlateBits141U0,
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

fn PyBytes_AS_STRING(op: *mut _object) -> *mut i8 {
    unsafe { (*_PyBytes_CAST!(op)).ob_sval.as_mut_ptr() as *mut i8 }
}

fn PyBytes_GET_SIZE(op: *mut _object) -> i64 {
    let self_: *mut __SlateRecord_bytesobject_h_15 = _PyBytes_CAST!(op);
    Py_SIZE!(self_)
}

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
struct _PyMonitoringState {
    active: u8,
    opaque: u8,
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
struct __SlateRecord_pystate_h_78 {
    __slate_bits_0: __slate_bits::__SlateBits192U0,
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
struct _opaque {
    computed_line: i32,
    lo_next: *const u8,
    limit: *const u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _line_offsets {
    ar_start: i32,
    ar_end: i32,
    ar_line: i32,
    opaque: _opaque,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_initconfig_h_36 {
    length: i64,
    items: *mut *mut i32,
}

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
struct __SlateRecord_pycore_structs_h_32_2 {
    code: u8,
    arg: u8,
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

fn _PyVectorcall_NARGS(n: u64) -> i64 {
    (n & !(_Py_STATIC_CAST!(u64, 1i32)
        << 8u64
            .wrapping_mul(core::mem::size_of::<u64>() as u64)
            .wrapping_sub(1))) as i64
}

fn _Py_popcount32(x: u32) -> i32 {
    while false {}
    x.count_ones() as i32
}

fn _Py_bit_length(x: u64) -> i32 {
    if x != 0 {
        // __builtin_clzl() is available since GCC 3.4.
        //
        // Undefined behavior for x == 0.
        return (core::mem::size_of::<u64>() as i32)
            .wrapping_mul(8)
            .wrapping_sub(x.leading_zeros() as i32);
    } else {
        return 0;
    }
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
struct code_arena_st {}

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

fn make_backoff_counter(value: u16, backoff: u16) -> __SlateRecord_pycore_structs_h_15 {
    let mut result: __SlateRecord_pycore_structs_h_15 = unsafe { core::mem::zeroed() };
    result.value_and_backoff = ((value as i32) << 4i32 | (backoff as i32)) as u16;
    return result;
}

// Initial JUMP_BACKWARD counter.
// This determines when we create a trace for a loop.
// Initial exit temperature.
// Must be larger than ADAPTIVE_COOLDOWN_VALUE,
// otherwise when a side exit warms up we may construct
// a new trace before the Tier 1 code has properly re-specialized.
#[repr(C)]
#[derive(Clone, Copy)]
struct llist_node {
    next: *mut llist_node,
    prev: *mut llist_node,
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

fn parse_varint(mut p: *mut u8, result: &mut i32) -> *mut u8 {
    let mut val: i32 = ((unsafe { *p.wrapping_offset(0 as isize) }) as i32) & 63;
    while ((unsafe { *p.wrapping_offset(0 as isize) }) as i32) & 64 != 0 {
        let __v0: *mut u8 = p;
        p = __v0.wrapping_offset(1 as isize);
        val = val << 6i32 | ((unsafe { *p.wrapping_offset(0 as isize) }) as i32) & 63;
    }
    unsafe {
        *(result as *mut i32) = val;
    }
    p.wrapping_offset(1 as isize)
}

// A value of 1 means that we attempt to specialize the *second* time each
//
// instruction is executed. Executing twice is a much better indicator of
//
// "hotness" than executing once, but additional warmup delays only prevent
//
// specialization. Most types stabilize by the second execution, too:
// A value of 52 means that we attempt to re-specialize after 53 misses (a prime
//
// number, useful for avoiding artifacts if every nth value is a different type
//
// or something). Setting the backoff to 0 means that the counter is reset to
//
// the same state as a warming-up instruction (value == 1, backoff == 1) after
//
// deoptimization. This isn't strictly necessary, but it is bit easier to reason
//
// about when thinking about the opcode transitions as a state machine:
fn adaptive_counter_bits(value: u16, backoff: u16) -> __SlateRecord_pycore_structs_h_15 {
    return make_backoff_counter(value, backoff);
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_pythread_h_64 {
    ptr: *mut __SlateRecord_pthreadtypes_h_45,
    val: __SlateRecord_pthreadtypes_h_45,
}

fn adaptive_counter_warmup() -> __SlateRecord_pycore_structs_h_15 {
    return adaptive_counter_bits(1u16, 1u16);
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_interp_structs_h_297 {
    mutex: PyMutex,
    hashtable: *mut _Py_hashtable_t,
}

fn _PyThreadState_GET() -> *mut _ts {
    unsafe { _Py_tss_tstate }
}

fn _PyInterpreterState_GET() -> *mut _is {
    let tstate: *mut _ts = _PyThreadState_GET();
    unsafe { (*tstate).interp }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_interp_structs_h_348 {
    import_level: i32,
    accumulated: i64,
    header: i32,
}

fn _PyVectorcall_FunctionInline(
    callable: *mut _object,
) -> Option<
    unsafe extern "C-unwind" fn(
        *mut _object,
        *const *mut _object,
        u64,
        *mut _object,
    ) -> *mut _object,
> {
    let tp: *mut _typeobject = Py_TYPE!(callable);
    if PyType_HasFeature(tp, 2048u64) == 0 {
        return None;
    }
    let offset: i64 = unsafe { (*tp).tp_vectorcall_offset };
    let mut ptr: Option<
        unsafe extern "C-unwind" fn(
            *mut _object,
            *const *mut _object,
            u64,
            *mut _object,
        ) -> *mut _object,
    > = unsafe { core::mem::zeroed() };
    unsafe {
        core::ptr::copy_nonoverlapping(
            (((callable as *mut i8).wrapping_offset(offset as isize) as *const ())
                as *const core::ffi::c_void) as *const u8,
            ((core::ptr::addr_of_mut!(ptr) as *mut ()) as *mut core::ffi::c_void) as *mut u8,
            ((core::mem::size_of::<
                Option<
                    unsafe extern "C-unwind" fn(
                        *mut _object,
                        *const *mut _object,
                        u64,
                        *mut _object,
                    ) -> *mut _object,
                >,
            >() as u64) as usize) as usize,
        )
    };
    ptr
}

fn _PyObject_VectorcallTstate(
    tstate: *mut _ts,
    callable: *mut _object,
    args: *const *mut _object,
    nargsf: u64,
    kwnames: *mut _object,
) -> *mut _object {
    let func: Option<
        unsafe extern "C-unwind" fn(
            *mut _object,
            *const *mut _object,
            u64,
            *mut _object,
        ) -> *mut _object,
    > = _PyVectorcall_FunctionInline(callable);
    if func == None {
        let nargs: i64 = PyVectorcall_NARGS!(nargsf);
        return unsafe { _PyObject_MakeTpCall(tstate, callable, args, nargs, kwnames) };
    }
    let res: *mut _object = unsafe { func.unwrap()(callable, args, nargsf, kwnames) };
    unsafe { _Py_CheckFunctionResult(tstate, callable, res, core::ptr::null::<i8>()) }
}

fn PyMutex_LockFlags(m: *mut PyMutex, flags: u32) {
    let mut expected: u8 = 0;
    if _Py_atomic_compare_exchange_uint8(
        unsafe { core::ptr::addr_of_mut!((*m)._bits) },
        unsafe { &mut (*core::ptr::addr_of_mut!(expected)) },
        1u8,
    ) == 0
    {
        unsafe { _PyMutex_LockTimed(m, 1i32.wrapping_neg() as i64, flags) };
    }
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
struct atexit_callback {
    func: Option<unsafe extern "C-unwind" fn(*mut ())>,
    data: *mut (),
    next: *mut atexit_callback,
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
struct __SlateRecord_pycore_hashtable_h_35 {
    _Py_slist_item: _Py_slist_item_s,
    key_hash: u64,
    key: *mut (),
    value: *mut (),
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
struct __SlateRecord_pycore_interp_structs_h_718 {
    _not_used: i32,
    _hamt_empty_gc_not_used: __SlateRecord_pycore_interp_structs_h_170,
    hamt_empty: __SlateRecord_pycore_structs_h_47,
    last_resort_memory_error: __SlateRecord_pyerrors_h_15,
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
union __SlateRecord_pycore_obmalloc_h_255 {
    _padding: *mut u8,
    count: u32,
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
struct _Py_unicode_runtime_ids {
    mutex: PyMutex,
    next_index: i64,
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
struct __SlateRecord_pycore_runtime_structs_h_49 {
    fd: i32,
    st_dev: u64,
    st_ino: u64,
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

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_tracemalloc_h_70 {
    mem: __SlateRecord_pymem_h_48,
    raw: __SlateRecord_pymem_h_48,
    obj: __SlateRecord_pymem_h_48,
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

// Internal IDs used to support sys.setprofile() and sys.settrace()
// Total tool ids available
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
struct __SlateRecord_pycore_instruments_h_94 {
    bytes_per_entry: u8,
    data: [u8; 1],
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
struct _frame {
    ob_base: _object,
    f_back: *mut _frame,
    f_frame: *mut _PyInterpreterFrame,
    f_trace: *mut _object,
    f_lineno: i32,
    f_trace_lines: i8,
    f_trace_opcodes: i8,
    f_extra_locals: *mut _object,
    f_locals_cache: *mut _object,
    f_overwritten_fast_locals: *mut _object,
    _f_frame_data: [*mut _object; 1],
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
struct _Py_float_runtime_state {
    float_format: u32,
    double_format: u32,
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
struct pyhash_runtime_state {
    urandom_cache: __SlateRecord_pycore_runtime_structs_h_49,
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
union __SlateRecord_pycore_optimizer_h_67_2 {
    target: u32,
    __slate_anon_1: __SlateRecord_pycore_optimizer_h_67_3,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_optimizer_h_67_3 {
    jump_target: u16,
    error_target: u16,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _fileutils_state {
    force_ascii: i32,
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

#[repr(C, align(16))]
struct __SlateAlign16<T>(T);

unsafe impl<T> Sync for __SlateAlign16<T> {}

mod __slate_bits {
    #[bitfields::bitfield(
        u16,
        c_names = true,
        new = false,
        from_into_bits = false,
        from_traits = false,
        default = false,
        debug = false,
        builder = false,
        bit_ops = false
    )]
    pub struct __SlateBits1307U0 {
        #[bits(15)]
        pub opcode: u16,
        #[bits(1)]
        pub format: u16,
    }
    #[bitfields::bitfield(
        u8,
        c_names = true,
        new = false,
        from_into_bits = false,
        from_traits = false,
        default = false,
        debug = false,
        builder = false,
        bit_ops = false
    )]
    pub struct __SlateBits1305U0 {
        #[bits(1)]
        pub valid: u8,
        #[bits(1)]
        pub linked: u8,
        #[bits(6)]
        pub chain_depth: u8,
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
    pub struct __SlateBits141U0 {
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
    pub struct __SlateBits192U0 {
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
struct _Py_global_strings {
    literals: __SlateRecord_pycore_global_strings_h_32,
    identifiers: __SlateRecord_pycore_global_strings_h_58,
    ascii: [__SlateRecord_pycore_global_strings_h_815; 128],
    latin1: [__SlateRecord_pycore_global_strings_h_819; 128],
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

use slate_support::memcpy;

fn PyStackRef_AsPyObjectBorrow(r#ref: _PyStackRef) -> *mut _object {
    BITS_TO_PTR_MASKED!(r#ref)
}

fn _PyFrame_GetCode(f: *mut _PyInterpreterFrame) -> *mut PyCodeObject {
    let executable: *mut _object = PyStackRef_AsPyObjectBorrow(unsafe { (*f).f_executable });
    executable as *mut PyCodeObject
}

fn _PyFrame_GetBytecode(f: *mut _PyInterpreterFrame) -> *mut __SlateRecord_pycore_structs_h_32 {
    _PyCode_CODE!(_PyFrame_GetCode(f))
}

fn _PyFrame_GetFrameObject(frame: *mut _PyInterpreterFrame) -> *mut _frame {
    let res: *mut _frame = unsafe { (*frame).frame_obj };
    if !res.is_null() {
        return res;
    }
    unsafe { _PyFrame_MakeAndSetFrameObject(frame) }
}

fn _PyLong_GetZero() -> *mut _object {
    unsafe {
        core::ptr::addr_of_mut!(_PyRuntime.static_objects.singletons.small_ints) as *mut _longobject
    }
    .wrapping_offset(5 as isize) as *mut _object
}

// Long value tag bits:
// 0-1: Sign bits value = (1-sign), ie. negative=2, positive=0, zero=1.
// 2: Set to 1 for the small ints
// 3+ Unsigned digit count
#[repr(C)]
#[derive(Clone, Copy)]
struct _PyExecutorLinkListNode {
    next: *mut _PyExecutorObject,
    previous: *mut _PyExecutorObject,
}

// Bloom filter with m = 256
// https://en.wikipedia.org/wiki/Bloom_filter
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_optimizer_h_28 {
    bits: [u32; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_optimizer_h_41 {
    opcode: u8,
    oparg: u8,
    __slate_bits_0: __slate_bits::__SlateBits1305U0,
    warm: bool,
    index: i32,
    bloom: __SlateRecord_pycore_optimizer_h_28,
    links: _PyExecutorLinkListNode,
    code: *mut PyCodeObject,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_optimizer_h_67 {
    __slate_bits_0: __slate_bits::__SlateBits1307U0,
    oparg: u16,
    __slate_anon_3: __SlateRecord_pycore_optimizer_h_67_2,
    operand0: u64,
    operand1: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_optimizer_h_73 {
    target: u32,
    temperature: __SlateRecord_pycore_structs_h_15,
    executor: *mut _PyExecutorObject,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _PyExecutorObject {
    ob_base: __SlateRecord_object_h_172,
    trace: *const __SlateRecord_pycore_optimizer_h_67,
    vm_data: __SlateRecord_pycore_optimizer_h_41,
    exit_count: u32,
    code_size: u32,
    jit_size: u64,
    jit_code: *mut (),
    jit_side_entry: *mut (),
    exits: [__SlateRecord_pycore_optimizer_h_73; 1],
}

// _PyObject_VectorcallTstate()
// _PY_EVAL_EVENTS_BITS
// _PyCode_Clear_Executors()
// _Py_CRITICAL_SECTION_ASSERT_OBJECT_LOCKED()
// PyFrameObject
// _PyFrame_GetBytecode()
// _PyLong_GetZero()
// _PyModule_CreateInitialized()
// _PyNamespace_New()
// IS_VALID_OPCODE()
// IS_CONDITIONAL_JUMP_OPCODE()
// _PyExecutorObject
// Holds locals, stack, locals, stack ... co_consts (in that order)
// Need extras for root frame and for overflow frame (see TRACE_STACK_PUSH())
// FT_ATOMIC_STORE_UINTPTR_RELEASE()
// _PyInterpreterState_GET()
// _PyCoMonitoringData
// _PyTuple_FromArraySteal()
// Uncomment this to dump debugging output when assertions fail
//
// #define INSTRUMENT_DEBUG 1
#[unsafe(no_mangle)]
static mut _PyInstrumentation_DISABLE: _object = _object {
    __slate_anon_0: {
        let mut __t0: __SlateRecord_object_h_122 = unsafe { core::mem::zeroed() };
        __t0.__slate_anon_1 = {
            let mut __t1: __SlateRecord_object_h_125 = unsafe { core::mem::zeroed() };
            __t1.ob_refcnt = 3221225472u32;
            __t1.ob_flags = 5u16;
            __t1
        };
        __t0
    },
    ob_type: unsafe { core::ptr::addr_of_mut!(PyBaseObject_Type) },
};

#[unsafe(no_mangle)]
static mut _PyInstrumentation_MISSING: _object = _object {
    __slate_anon_0: {
        let mut __t0: __SlateRecord_object_h_122 = unsafe { core::mem::zeroed() };
        __t0.__slate_anon_1 = {
            let mut __t1: __SlateRecord_object_h_125 = unsafe { core::mem::zeroed() };
            __t1.ob_refcnt = 3221225472u32;
            __t1.ob_flags = 5u16;
            __t1
        };
        __t0
    },
    ob_type: unsafe { core::ptr::addr_of_mut!(PyBaseObject_Type) },
};

static EVENT_FOR_OPCODE: __SlateAlign16<[i8; 256]> = __SlateAlign16({
    let mut __t0: [i8; 256] = unsafe { core::mem::zeroed() };
    __t0[4] = 4i8;
    __t0[9] = 10i8;
    __t0[10] = 10i8;
    __t0[28] = 8i8;
    __t0[30] = 9i8;
    __t0[35] = 2i8;
    __t0[52] = 4i8;
    __t0[55] = 4i8;
    __t0[68] = 9i8;
    __t0[70] = 8i8;
    __t0[75] = 7i8;
    __t0[77] = 7i8;
    __t0[96] = 4i8;
    __t0[100] = 9i8;
    __t0[101] = 9i8;
    __t0[102] = 9i8;
    __t0[103] = 9i8;
    __t0[120] = 3i8;
    __t0[128] = 1i32.wrapping_neg() as i8;
    __t0[234] = 10i8;
    __t0[235] = 9i8;
    __t0[236] = 10i8;
    __t0[237] = 8i8;
    __t0[239] = 7i8;
    __t0[240] = 8i8;
    __t0[241] = 9i8;
    __t0[242] = 9i8;
    __t0[243] = 9i8;
    __t0[244] = 9i8;
    __t0[246] = 2i8;
    __t0[247] = 3i8;
    __t0[249] = 4i8;
    __t0[250] = 4i8;
    __t0[251] = 4i8;
    __t0[252] = 4i8;
    __t0[253] = 7i8;
    __t0
});

static DE_INSTRUMENT: __SlateAlign16<[u8; 256]> = __SlateAlign16({
    let mut __t0: [u8; 256] = unsafe { core::mem::zeroed() };
    __t0[234] = 9u8;
    __t0[235] = 30u8;
    __t0[236] = 10u8;
    __t0[237] = 70u8;
    __t0[239] = 77u8;
    __t0[240] = 28u8;
    __t0[241] = 103u8;
    __t0[242] = 100u8;
    __t0[243] = 101u8;
    __t0[244] = 102u8;
    __t0[245] = 128u8;
    __t0[246] = 35u8;
    __t0[247] = 120u8;
    __t0[248] = 68u8;
    __t0[249] = 96u8;
    __t0[250] = 52u8;
    __t0[251] = 55u8;
    __t0[252] = 4u8;
    __t0[253] = 75u8;
    __t0
});

static INSTRUMENTED_OPCODES: __SlateAlign16<[u8; 256]> = __SlateAlign16({
    let mut __t0: [u8; 256] = unsafe { core::mem::zeroed() };
    __t0[4] = 252u8;
    __t0[9] = 234u8;
    __t0[10] = 236u8;
    __t0[28] = 240u8;
    __t0[30] = 235u8;
    __t0[35] = 246u8;
    __t0[52] = 250u8;
    __t0[55] = 251u8;
    __t0[68] = 248u8;
    __t0[70] = 237u8;
    __t0[75] = 253u8;
    __t0[77] = 239u8;
    __t0[96] = 249u8;
    __t0[100] = 242u8;
    __t0[101] = 243u8;
    __t0[102] = 244u8;
    __t0[103] = 241u8;
    __t0[120] = 247u8;
    __t0[128] = 245u8;
    __t0[234] = 234u8;
    __t0[235] = 235u8;
    __t0[236] = 236u8;
    __t0[237] = 237u8;
    __t0[238] = 238u8;
    __t0[239] = 239u8;
    __t0[240] = 240u8;
    __t0[241] = 241u8;
    __t0[242] = 242u8;
    __t0[243] = 243u8;
    __t0[244] = 244u8;
    __t0[245] = 245u8;
    __t0[246] = 246u8;
    __t0[247] = 247u8;
    __t0[248] = 248u8;
    __t0[249] = 249u8;
    __t0[250] = 250u8;
    __t0[251] = 251u8;
    __t0[252] = 252u8;
    __t0[253] = 253u8;
    __t0[254] = 254u8;
    __t0
});

fn opcode_has_event(opcode: i32) -> bool {
    opcode != 254 && ((unsafe { INSTRUMENTED_OPCODES.0[opcode as usize] }) as i32) > 0
}

fn is_instrumented(opcode: i32) -> bool {
    opcode != 255 && opcode >= 234
}

fn monitors_sub(a: _Py_LocalMonitors, b: _Py_LocalMonitors) -> _Py_LocalMonitors {
    let mut res: _Py_LocalMonitors = unsafe { core::mem::zeroed() };
    for i in 0i32..11i32 {
        res.tools[i as usize] =
            ((a.tools[i as usize] as i32) & !(b.tools[i as usize] as i32)) as u8;
    }
    return res;
}

/// The union of the *local* events in a and b.
/// Global events like RAISE are ignored.
/// Used for instrumentation, as only local
/// events get instrumented.
fn local_union(a: _Py_GlobalMonitors, b: _Py_LocalMonitors) -> _Py_LocalMonitors {
    let mut res: _Py_LocalMonitors = unsafe { core::mem::zeroed() };
    for i in 0i32..11i32 {
        res.tools[i as usize] = ((a.tools[i as usize] as i32) | (b.tools[i as usize] as i32)) as u8;
    }
    return res;
}

fn monitors_are_empty(m: _Py_LocalMonitors) -> bool {
    for i in 0i32..11i32 {
        if m.tools[i as usize] != 0 {
            return false;
        }
    }
    true
}

fn multiple_tools(m: &mut _Py_LocalMonitors) -> bool {
    for i in 0i32..11i32 {
        if _Py_popcount32(
            (unsafe {
                *((*(m as *mut _Py_LocalMonitors)).tools.as_mut_ptr() as *mut u8)
                    .wrapping_offset(i as isize)
            }) as u32,
        ) > 1
        {
            return true;
        }
    }
    false
}

fn get_local_events(m: *mut _Py_LocalMonitors, tool_id: i32) -> u32 {
    let mut result: u32 = 0;
    for e in 0i32..11i32 {
        if ((unsafe { *((*m).tools.as_mut_ptr() as *mut u8).wrapping_offset(e as isize) }) as i32)
            >> tool_id
            & 1
            != 0
        {
            result |= (1i32 << e) as u32;
        }
    }
    result
}

fn get_events(m: *mut _Py_GlobalMonitors, tool_id: i32) -> u32 {
    let mut result: u32 = 0;
    for e in 0i32..16i32 {
        if ((unsafe { *((*m).tools.as_mut_ptr() as *mut u8).wrapping_offset(e as isize) }) as i32)
            >> tool_id
            & 1
            != 0
        {
            result |= (1i32 << e) as u32;
        }
    }
    result
}

// Module code can have line 0, even though modules start at line 1,
// so -1 is a legal delta.
/// Returns the line delta. Defined as:
/// if line is None:
///     line_delta = NO_LINE
/// else:
///     line_delta = line - first_line
fn compute_line_delta(code: *mut PyCodeObject, line: i32) -> i32 {
    if line < 0 {
        return 2i32.wrapping_neg();
    }
    let delta: i32 = line.wrapping_sub(unsafe { (*code).co_firstlineno });
    delta
}

fn compute_line(code: *mut PyCodeObject, line_delta: i32) -> i32 {
    if line_delta == 2i32.wrapping_neg() {
        return 1i32.wrapping_neg();
    }
    unsafe { (*code).co_firstlineno }.wrapping_add(line_delta)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyInstruction_GetLength(code: *mut PyCodeObject, offset: i32) -> i32 {
    let inst: __SlateRecord_pycore_structs_h_32 = _Py_GetBaseCodeUnit(code, offset);
    1i32.wrapping_add((unsafe { _PyOpcode_Caches[(inst.op.code as i32) as usize] }) as i32)
}

fn get_original_opcode(line_data: *mut __SlateRecord_pycore_instruments_h_94, index: i32) -> u8 {
    unsafe {
        *((*line_data).data.as_mut_ptr() as *mut u8)
            .wrapping_offset(index.wrapping_mul((*line_data).bytes_per_entry as i32) as isize)
    }
}

fn get_original_opcode_ptr(
    line_data: *mut __SlateRecord_pycore_instruments_h_94,
    index: i32,
) -> *mut u8 {
    unsafe { (*line_data).data.as_mut_ptr() as *mut u8 }.wrapping_offset(
        index.wrapping_mul((unsafe { (*line_data).bytes_per_entry }) as i32) as isize,
    )
}

fn set_original_opcode(
    line_data: *mut __SlateRecord_pycore_instruments_h_94,
    index: i32,
    opcode: u8,
) {
    unsafe {
        *((*line_data).data.as_mut_ptr() as *mut u8)
            .wrapping_offset(index.wrapping_mul((*line_data).bytes_per_entry as i32) as isize) =
            opcode;
    }
}

fn get_line_delta(line_data: *mut __SlateRecord_pycore_instruments_h_94, index: i32) -> i32 {
    let mut ptr: *mut u8 = unsafe { (*line_data).data.as_mut_ptr() as *mut u8 }.wrapping_offset(
        index
            .wrapping_mul((unsafe { (*line_data).bytes_per_entry }) as i32)
            .wrapping_add(1) as isize,
    );
    let mut value: u32 = (unsafe { *ptr }) as u32;
    let mut idx: i32 = 2;
    while idx < ((unsafe { (*line_data).bytes_per_entry }) as i32) {
        let __v0: *mut u8 = ptr;
        ptr = __v0.wrapping_offset(1 as isize);
        let shift: i32 = idx.wrapping_sub(1).wrapping_mul(8);
        value |= ((unsafe { *ptr }) as u32) << shift;
        idx = idx.wrapping_add(1);
    }
    // NO_LINE is stored as zero.
    (value as i32).wrapping_add(2i32.wrapping_neg())
}

fn set_line_delta(
    line_data: *mut __SlateRecord_pycore_instruments_h_94,
    index: i32,
    line_delta: i32,
) {
    // Store line_delta + 2 as we need -2 to represent no line number
    let mut adjusted: u32 = line_delta.wrapping_sub(2i32.wrapping_neg()) as u32;
    let mut ptr: *mut u8 = unsafe { (*line_data).data.as_mut_ptr() as *mut u8 }.wrapping_offset(
        index
            .wrapping_mul((unsafe { (*line_data).bytes_per_entry }) as i32)
            .wrapping_add(1) as isize,
    );
    unsafe {
        *ptr = (adjusted & 255) as u8;
    }
    let mut idx: i32 = 2;
    while idx < ((unsafe { (*line_data).bytes_per_entry }) as i32) {
        let __v0: *mut u8 = ptr;
        ptr = __v0.wrapping_offset(1 as isize);
        adjusted >>= 8i32;
        unsafe {
            *ptr = (adjusted & 255) as u8;
        }
        idx = idx.wrapping_add(1);
    }
}

/// Get the underlying code unit, stripping instrumentation and ENTER_EXECUTOR
#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_GetBaseCodeUnit(
    code: *mut PyCodeObject,
    i: i32,
) -> __SlateRecord_pycore_structs_h_32 {
    let src_instr: *mut __SlateRecord_pycore_structs_h_32 =
        _PyCode_CODE!(code).wrapping_offset(i as isize);
    let mut inst: __SlateRecord_pycore_structs_h_32 = __SlateRecord_pycore_structs_h_32 {
        cache: unsafe { *(src_instr as *mut u16) },
    };
    let mut opcode: i32 = (unsafe { inst.op.code }) as i32;
    if opcode < 234 {
        unsafe {
            inst.op.code = _PyOpcode_Deopt[opcode as usize];
        }
        return inst;
    }
    if opcode == 255 {
        let exec: *mut _PyExecutorObject = unsafe {
            *((*(*code).co_executors).executors.as_mut_ptr() as *mut *mut _PyExecutorObject)
                .wrapping_offset((inst.op.arg as i32) as isize)
        };
        opcode = (unsafe { _PyOpcode_Deopt[((*exec).vm_data.opcode as i32) as usize] }) as i32;
        unsafe {
            inst.op.code = opcode as u8;
        }
        unsafe {
            inst.op.arg = (*exec).vm_data.oparg;
        }
        return inst;
    }
    if opcode == 254 {
        opcode = get_original_opcode(unsafe { (*(*code)._co_monitoring).lines }, i) as i32;
    }
    if opcode == 238 {
        opcode = (unsafe {
            *unsafe { (*(*code)._co_monitoring).per_instruction_opcodes }
                .wrapping_offset(i as isize)
        }) as i32;
    }
    CHECK__2!();
    CHECK__2!();
    let deinstrumented: i32 = (unsafe { DE_INSTRUMENT.0[opcode as usize] }) as i32;
    if deinstrumented != 0 {
        unsafe {
            inst.op.code = deinstrumented as u8;
        }
    } else {
        unsafe {
            inst.op.code = _PyOpcode_Deopt[opcode as usize];
        }
    }
    return inst;
}

fn de_instrument(
    code: *mut PyCodeObject,
    bytecode: *mut __SlateRecord_pycore_structs_h_32,
    monitoring: *mut _PyCoMonitoringData,
    i: i32,
    event: i32,
) {
    let mut __slate_compound_0: __SlateRecord_pycore_structs_h_15 = unsafe { core::mem::zeroed() };
    let instr: *mut __SlateRecord_pycore_structs_h_32 = bytecode.wrapping_offset(i as isize);
    let mut opcode_ptr: *mut u8 = unsafe { core::ptr::addr_of_mut!((*instr).op.code) };
    let mut opcode: i32 = (unsafe { *opcode_ptr }) as i32;
    if opcode == 254 {
        opcode_ptr = get_original_opcode_ptr(unsafe { (*monitoring).lines }, i);
        opcode = (unsafe { *opcode_ptr }) as i32;
    }
    if opcode == 238 {
        opcode_ptr = unsafe { (*monitoring).per_instruction_opcodes }.wrapping_offset(i as isize);
        opcode = (unsafe { *opcode_ptr }) as i32;
    }
    let deinstrumented: i32 = (unsafe { DE_INSTRUMENT.0[opcode as usize] }) as i32;
    if deinstrumented == 0 {
        return;
    }
    CHECK__2!();
    FT_ATOMIC_STORE_UINT8_RELAXED!(*opcode_ptr, deinstrumented);
    if (unsafe { _PyOpcode_Caches[deinstrumented as usize] }) != 0 {
        unsafe {
            (*instr.wrapping_offset(1 as isize))
                .counter
                .value_and_backoff = (*{
                __slate_compound_0 = adaptive_counter_warmup();
                core::ptr::addr_of_mut!(__slate_compound_0)
            })
            .value_and_backoff;
        }
    }
}

fn de_instrument_line(
    code: *mut PyCodeObject,
    bytecode: *mut __SlateRecord_pycore_structs_h_32,
    monitoring: *mut _PyCoMonitoringData,
    i: i32,
) {
    let mut __slate_compound_0: __SlateRecord_pycore_structs_h_15 = unsafe { core::mem::zeroed() };
    let instr: *mut __SlateRecord_pycore_structs_h_32 = bytecode.wrapping_offset(i as isize);
    let opcode: i32 = (unsafe { (*instr).op.code }) as i32;
    if opcode != 254 {
        return;
    }
    let lines: *mut __SlateRecord_pycore_instruments_h_94 = unsafe { (*monitoring).lines };
    let original_opcode: i32 = get_original_opcode(lines, i) as i32;
    if original_opcode == 238 {
        set_original_opcode(lines, i, unsafe {
            *unsafe { (*monitoring).per_instruction_opcodes }.wrapping_offset(i as isize)
        });
    }
    CHECK__2!();
    CHECK__2!();
    FT_ATOMIC_STORE_UINT8!((*instr).op.code, original_opcode);
    if (unsafe { _PyOpcode_Caches[original_opcode as usize] }) != 0 {
        unsafe {
            (*instr.wrapping_offset(1 as isize))
                .counter
                .value_and_backoff = (*{
                __slate_compound_0 = adaptive_counter_warmup();
                core::ptr::addr_of_mut!(__slate_compound_0)
            })
            .value_and_backoff;
        }
    }
}

fn de_instrument_per_instruction(
    code: *mut PyCodeObject,
    bytecode: *mut __SlateRecord_pycore_structs_h_32,
    monitoring: *mut _PyCoMonitoringData,
    i: i32,
) {
    let mut __slate_compound_0: __SlateRecord_pycore_structs_h_15 = unsafe { core::mem::zeroed() };
    let instr: *mut __SlateRecord_pycore_structs_h_32 = bytecode.wrapping_offset(i as isize);
    let mut opcode_ptr: *mut u8 = unsafe { core::ptr::addr_of_mut!((*instr).op.code) };
    let mut opcode: i32 = (unsafe { *opcode_ptr }) as i32;
    if opcode == 254 {
        opcode_ptr = get_original_opcode_ptr(unsafe { (*monitoring).lines }, i);
        opcode = (unsafe { *opcode_ptr }) as i32;
    }
    if opcode != 238 {
        return;
    }
    let original_opcode: i32 =
        (unsafe { *unsafe { (*monitoring).per_instruction_opcodes }.wrapping_offset(i as isize) })
            as i32;
    CHECK__2!();
    CHECK__2!();
    FT_ATOMIC_STORE_UINT8_RELAXED!(*opcode_ptr, original_opcode);
    if (unsafe { _PyOpcode_Caches[original_opcode as usize] }) != 0 {
        unsafe {
            (*instr.wrapping_offset(1 as isize))
                .counter
                .value_and_backoff = (*{
                __slate_compound_0 = adaptive_counter_warmup();
                core::ptr::addr_of_mut!(__slate_compound_0)
            })
            .value_and_backoff;
        }
    }
}

fn instrument(
    code: *mut PyCodeObject,
    bytecode: *mut __SlateRecord_pycore_structs_h_32,
    monitoring: *mut _PyCoMonitoringData,
    i: i32,
) {
    let mut __slate_compound_0: __SlateRecord_pycore_structs_h_15 = unsafe { core::mem::zeroed() };
    let instr: *mut __SlateRecord_pycore_structs_h_32 = bytecode.wrapping_offset(i as isize);
    let mut opcode_ptr: *mut u8 = unsafe { core::ptr::addr_of_mut!((*instr).op.code) };
    let mut opcode: i32 = (unsafe { *opcode_ptr }) as i32;
    if opcode == 254 {
        opcode_ptr = get_original_opcode_ptr(unsafe { (*monitoring).lines }, i);
        opcode = (unsafe { *opcode_ptr }) as i32;
    }
    if opcode == 238 {
        opcode_ptr = unsafe { (*monitoring).per_instruction_opcodes }.wrapping_offset(i as isize);
        opcode = (unsafe { *opcode_ptr }) as i32;
        CHECK__2!();
        CHECK__2!();
    }
    CHECK__2!();
    if !is_instrumented(opcode) {
        let deopt: i32 = (unsafe { _PyOpcode_Deopt[opcode as usize] }) as i32;
        let instrumented: i32 = (unsafe { INSTRUMENTED_OPCODES.0[deopt as usize] }) as i32;
        FT_ATOMIC_STORE_UINT8_RELAXED!(*opcode_ptr, instrumented);
        if (unsafe { _PyOpcode_Caches[deopt as usize] }) != 0 {
            unsafe {
                (*instr.wrapping_offset(1 as isize))
                    .counter
                    .value_and_backoff = (*{
                    __slate_compound_0 = adaptive_counter_warmup();
                    core::ptr::addr_of_mut!(__slate_compound_0)
                })
                .value_and_backoff;
            }
        }
    }
}

fn instrument_line(
    code: *mut PyCodeObject,
    bytecode: *mut __SlateRecord_pycore_structs_h_32,
    monitoring: *mut _PyCoMonitoringData,
    i: i32,
) {
    let opcode_ptr: *mut u8 =
        unsafe { core::ptr::addr_of_mut!((*bytecode.wrapping_offset(i as isize)).op.code) };
    let opcode: i32 = (unsafe { *opcode_ptr }) as i32;
    if opcode == 254 {
        return;
    }
    set_original_opcode(unsafe { (*monitoring).lines }, i, unsafe {
        _PyOpcode_Deopt[opcode as usize]
    });
    CHECK__2!();
    FT_ATOMIC_STORE_UINT8_RELAXED!(*opcode_ptr, 254i32);
}

fn instrument_per_instruction(
    code: *mut PyCodeObject,
    bytecode: *mut __SlateRecord_pycore_structs_h_32,
    monitoring: *mut _PyCoMonitoringData,
    i: i32,
) {
    let instr: *mut __SlateRecord_pycore_structs_h_32 = bytecode.wrapping_offset(i as isize);
    let mut opcode_ptr: *mut u8 = unsafe { core::ptr::addr_of_mut!((*instr).op.code) };
    let mut opcode: i32 = (unsafe { *opcode_ptr }) as i32;
    if opcode == 254 {
        opcode_ptr = get_original_opcode_ptr(unsafe { (*monitoring).lines }, i);
        opcode = (unsafe { *opcode_ptr }) as i32;
    }
    if opcode == 238 {
        return;
    }
    CHECK__2!();
    if is_instrumented(opcode) {
        unsafe {
            *unsafe { (*monitoring).per_instruction_opcodes }.wrapping_offset(i as isize) =
                opcode as u8;
        }
    } else {
        unsafe {
            *unsafe { (*monitoring).per_instruction_opcodes }.wrapping_offset(i as isize) =
                _PyOpcode_Deopt[opcode as usize];
        }
    }
    FT_ATOMIC_STORE_UINT8_RELAXED!(*opcode_ptr, 238i32);
}

fn remove_tools(code: *mut PyCodeObject, offset: i32, event: i32, tools: i32) {
    let monitoring: *mut _PyCoMonitoringData = unsafe { (*code)._co_monitoring };
    let should_de_instrument: bool = if !unsafe { (*monitoring).tools }.is_null() {
        let __v0: *mut u8 = unsafe { (*monitoring).tools }.wrapping_offset(offset as isize);
        let __v2: u8 = (((unsafe { *__v0 }) as i32) & !tools) as u8;
        unsafe {
            *__v0 = __v2;
        }
        ((unsafe { *unsafe { (*monitoring).tools }.wrapping_offset(offset as isize) }) as i32) == 0
    } else {
        // Single tool
        let single_tool: u8 = unsafe {
            *((*monitoring).active_monitors.tools.as_mut_ptr() as *mut u8)
                .wrapping_offset(event as isize)
        };
        (single_tool as i32) & tools == (single_tool as i32)
    };
    if should_de_instrument {
        de_instrument(code, _PyCode_CODE!(code), monitoring, offset, event);
    }
}

fn remove_line_tools(code: *mut PyCodeObject, offset: i32, tools: i32) {
    let monitoring: *mut _PyCoMonitoringData = unsafe { (*code)._co_monitoring };
    let should_de_instrument: bool = if !unsafe { (*monitoring).line_tools }.is_null() {
        let toolsptr: *mut u8 =
            unsafe { (*monitoring).line_tools }.wrapping_offset(offset as isize);
        let __v2: u8 = (((unsafe { *toolsptr }) as i32) & !tools) as u8;
        unsafe {
            *toolsptr = __v2;
        }
        ((unsafe { *toolsptr }) as i32) == 0
    } else {
        // Single tool
        let single_tool: u8 = unsafe {
            *((*monitoring).active_monitors.tools.as_mut_ptr() as *mut u8)
                .wrapping_offset(5 as isize)
        };
        (single_tool as i32) & tools == (single_tool as i32)
    };
    if should_de_instrument {
        de_instrument_line(code, _PyCode_CODE!(code), monitoring, offset);
    }
}

fn add_tools(code: *mut PyCodeObject, offset: i32, event: i32, tools: i32) {
    if !unsafe { (*code)._co_monitoring }.is_null()
        && !unsafe { (*(*code)._co_monitoring).tools }.is_null()
    {
        let __v0: *mut u8 =
            unsafe { (*(*code)._co_monitoring).tools }.wrapping_offset(offset as isize);
        unsafe {
            *__v0 = ((*__v0 as i32) | tools) as u8;
        }
    }
    // Single tool
    instrument(
        code,
        _PyCode_CODE!(code),
        unsafe { (*code)._co_monitoring },
        offset,
    );
}

fn add_line_tools(code: *mut PyCodeObject, offset: i32, tools: i32) {
    if !unsafe { (*(*code)._co_monitoring).line_tools }.is_null() {
        let __v0: *mut u8 =
            unsafe { (*(*code)._co_monitoring).line_tools }.wrapping_offset(offset as isize);
        unsafe {
            *__v0 = ((*__v0 as i32) | tools) as u8;
        }
    }
    // Single tool
    instrument_line(
        code,
        _PyCode_CODE!(code),
        unsafe { (*code)._co_monitoring },
        offset,
    );
}

fn add_per_instruction_tools(code: *mut PyCodeObject, offset: i32, tools: i32) {
    if !unsafe { (*(*code)._co_monitoring).per_instruction_tools }.is_null() {
        let __v0: *mut u8 = unsafe { (*(*code)._co_monitoring).per_instruction_tools }
            .wrapping_offset(offset as isize);
        unsafe {
            *__v0 = ((*__v0 as i32) | tools) as u8;
        }
    }
    // Single tool
    instrument_per_instruction(
        code,
        _PyCode_CODE!(code),
        unsafe { (*code)._co_monitoring },
        offset,
    );
}

fn remove_per_instruction_tools(code: *mut PyCodeObject, offset: i32, tools: i32) {
    let monitoring: *mut _PyCoMonitoringData = unsafe { (*code)._co_monitoring };
    let should_de_instrument: bool =
        if !unsafe { (*(*code)._co_monitoring).per_instruction_tools }.is_null() {
            let toolsptr: *mut u8 = unsafe { (*(*code)._co_monitoring).per_instruction_tools }
                .wrapping_offset(offset as isize);
            let __v2: u8 = (((unsafe { *toolsptr }) as i32) & !tools) as u8;
            unsafe {
                *toolsptr = __v2;
            }
            ((unsafe { *toolsptr }) as i32) == 0
        } else {
            // Single tool
            let single_tool: u8 = unsafe {
                *((*(*code)._co_monitoring).active_monitors.tools.as_mut_ptr() as *mut u8)
                    .wrapping_offset(6 as isize)
            };
            (single_tool as i32) & tools == (single_tool as i32)
        };
    if should_de_instrument {
        de_instrument_per_instruction(code, _PyCode_CODE!(code), monitoring, offset);
    }
}

/// Return 1 if DISABLE returned, -1 if error, 0 otherwise
fn call_one_instrument(
    interp: *mut _is,
    tstate: *mut _ts,
    args: *mut *mut _object,
    nargsf: u64,
    tool: i8,
    event: i32,
) -> i32 {
    let instrument: *mut _object = unsafe {
        *((*((*interp).monitoring_callables.as_mut_ptr() as *mut [*mut _object; 19])
            .wrapping_offset((tool as i32) as isize))
        .as_mut_ptr() as *mut *mut _object)
            .wrapping_offset(event as isize)
    };
    if instrument.is_null() {
        return 0;
    }
    let old_what: i32 = unsafe { (*tstate).what_event };
    unsafe {
        (*tstate).what_event = event;
    }
    let __v1: i32 = unsafe { (*tstate).tracing };
    unsafe {
        (*tstate).tracing = __v1.wrapping_add(1);
    }
    let res: *mut _object = _PyObject_VectorcallTstate(
        tstate,
        instrument,
        args as *const *mut _object,
        nargsf,
        core::ptr::null_mut::<_object>(),
    );
    let __v4: i32 = unsafe { (*tstate).tracing };
    unsafe {
        (*tstate).tracing = __v4.wrapping_sub(1);
    }
    unsafe {
        (*tstate).what_event = old_what;
    }
    if res.is_null() {
        return 1i32.wrapping_neg();
    }
    if res == unsafe { core::ptr::addr_of_mut!(_PyInstrumentation_DISABLE) } {
        return 1;
    }
    Py_DECREF!(res);
    0
}

static MOST_SIGNIFICANT_BITS: __SlateAlign16<[i8; 16]> = __SlateAlign16([
    1i32.wrapping_neg() as i8,
    0i8,
    1i8,
    1i8,
    2i8,
    2i8,
    2i8,
    2i8,
    3i8,
    3i8,
    3i8,
    3i8,
    3i8,
    3i8,
    3i8,
    3i8,
]);

/// We could use _Py_bit_length here, but that is designed for larger (32/64)
/// bit ints, and can perform relatively poorly on platforms without the
/// necessary intrinsics.
fn most_significant_bit(bits: u8) -> i32 {
    if (bits as i32) > 15 {
        return ((unsafe { MOST_SIGNIFICANT_BITS.0[((bits as i32) >> 4i32) as usize] }) as i32)
            .wrapping_add(4);
    }
    (unsafe { MOST_SIGNIFICANT_BITS.0[(bits as i32) as usize] }) as i32
}

fn global_version(interp: *mut _is) -> u32 {
    let version: u32 = _Py_atomic_load_uintptr_relaxed(
        (unsafe { core::ptr::addr_of_mut!((*interp).ceval.instrumentation_version) }) as *const u64,
    ) as u32;
    version
}

/// Atomically set the given version in the given location, without touching
/// anything in _PY_EVAL_EVENTS_MASK.
fn set_version_raw(ptr: *mut u64, version: u32) {
    let mut old: u64 = _Py_atomic_load_uintptr_relaxed(ptr as *const u64);
    let mut new: u64 = 0;
    loop {
        new = old & 255u64 | (version as u64);
        if _Py_atomic_compare_exchange_uintptr(
            ptr,
            unsafe { &mut (*core::ptr::addr_of_mut!(old)) },
            new,
        ) != 0
        {
            break;
        }
    }
}

fn set_global_version(tstate: *mut _ts, version: u32) {
    let interp: *mut _is = unsafe { (*tstate).interp };
    set_version_raw(
        unsafe { core::ptr::addr_of_mut!((*interp).ceval.instrumentation_version) },
        version,
    );
    // Normal builds take the current version from instrumentation_version when
    //
    // attaching a thread, so we only have to set the current thread's version.
    set_version_raw(
        unsafe { core::ptr::addr_of_mut!((*tstate).eval_breaker) },
        version,
    );
}

fn is_version_up_to_date(code: *mut PyCodeObject, interp: *mut _is) -> bool {
    (global_version(interp) as u64) == unsafe { (*code)._co_instrumentation_version }
}

fn get_tools_for_instruction(
    code: *mut PyCodeObject,
    interp: *mut _is,
    i: i32,
    mut event: i32,
) -> u8 {
    let mut tools: u8 = 0;
    if event >= 16 {
        event = 4;
    }
    if PY_MONITORING_IS_INSTRUMENTED_EVENT!(event) {
        CHECK__2!();
        if !unsafe { (*(*code)._co_monitoring).tools }.is_null() {
            tools =
                unsafe { *unsafe { (*(*code)._co_monitoring).tools }.wrapping_offset(i as isize) };
        } else {
            tools = unsafe {
                *((*(*code)._co_monitoring).active_monitors.tools.as_mut_ptr() as *mut u8)
                    .wrapping_offset(event as isize)
            };
        }
    } else {
        tools = unsafe {
            *((*interp).monitors.tools.as_mut_ptr() as *mut u8).wrapping_offset(event as isize)
        };
    }
    tools
}

static event_names: __SlateAlign16<[*const i8; 19]> = __SlateAlign16([
    (b"PY_START\0".as_ptr() as *mut i8) as *const i8,
    (b"PY_RESUME\0".as_ptr() as *mut i8) as *const i8,
    (b"PY_RETURN\0".as_ptr() as *mut i8) as *const i8,
    (b"PY_YIELD\0".as_ptr() as *mut i8) as *const i8,
    (b"CALL\0".as_ptr() as *mut i8) as *const i8,
    (b"LINE\0".as_ptr() as *mut i8) as *const i8,
    (b"INSTRUCTION\0".as_ptr() as *mut i8) as *const i8,
    (b"JUMP\0".as_ptr() as *mut i8) as *const i8,
    (b"BRANCH_LEFT\0".as_ptr() as *mut i8) as *const i8,
    (b"BRANCH_RIGHT\0".as_ptr() as *mut i8) as *const i8,
    (b"STOP_ITERATION\0".as_ptr() as *mut i8) as *const i8,
    (b"RAISE\0".as_ptr() as *mut i8) as *const i8,
    (b"EXCEPTION_HANDLED\0".as_ptr() as *mut i8) as *const i8,
    (b"PY_UNWIND\0".as_ptr() as *mut i8) as *const i8,
    (b"PY_THROW\0".as_ptr() as *mut i8) as *const i8,
    (b"RERAISE\0".as_ptr() as *mut i8) as *const i8,
    (b"C_RETURN\0".as_ptr() as *mut i8) as *const i8,
    (b"C_RAISE\0".as_ptr() as *mut i8) as *const i8,
    (b"BRANCH\0".as_ptr() as *mut i8) as *const i8,
]);

fn call_instrumentation_vector(
    instr: *mut __SlateRecord_pycore_structs_h_32,
    tstate: *mut _ts,
    event: i32,
    frame: *mut _PyInterpreterFrame,
    arg2: *mut __SlateRecord_pycore_structs_h_32,
    nargs: i64,
    args: *mut *mut _object,
) -> i32 {
    if (unsafe { (*tstate).tracing }) != 0 {
        return 0;
    }
    let code: *mut PyCodeObject = _PyFrame_GetCode(frame);
    unsafe {
        *args.wrapping_offset(1 as isize) = code as *mut _object;
    }
    let offset: i32 = ((unsafe {
        instr.offset_from(_PyFrame_GetBytecode(frame) as *mut __SlateRecord_pycore_structs_h_32)
    }) as i64) as i32;
    // Offset visible to user should be the offset in bytes, as that is the
    // convention for APIs involving code offsets.
    let bytes_arg2: i32 = (((unsafe {
        arg2.offset_from(_PyFrame_GetBytecode(frame) as *mut __SlateRecord_pycore_structs_h_32)
    }) as i64) as i32)
        .wrapping_mul(core::mem::size_of::<__SlateRecord_pycore_structs_h_32>() as i32);
    let arg2_obj: *mut _object = unsafe { PyLong_FromLong(bytes_arg2 as i64) };
    if arg2_obj.is_null() {
        return 1i32.wrapping_neg();
    }
    unsafe {
        *args.wrapping_offset(2 as isize) = arg2_obj;
    }
    let interp: *mut _is = unsafe { (*tstate).interp };
    let mut tools: u8 = get_tools_for_instruction(code, interp, offset, event);
    let nargsf: u64 = (nargs as u64)
        | _Py_STATIC_CAST!(u64, 1i32)
            << 8u64
                .wrapping_mul(core::mem::size_of::<u64>() as u64)
                .wrapping_sub(1);
    let callargs: *mut *mut _object = args.wrapping_offset(1 as isize);
    let mut err: i32 = 0;
    while tools != 0 {
        let tool: i32 = most_significant_bit(tools);
        tools = ((tools as i32) ^ 1i32 << tool) as u8;
        let res: i32 = call_one_instrument(interp, tstate, callargs, nargsf, tool as i8, event);
        if res == 0 {
            // Nothing to do
        } else {
            if res < 0 {
                // error
                err = 1i32.wrapping_neg();
                break;
            } else {
                // DISABLE
                if !PY_MONITORING_IS_INSTRUMENTED_EVENT!(event) {
                    unsafe {
                        PyErr_Format(
                            PyExc_ValueError,
                            c"Cannot disable %s events. Callback removed.".as_ptr(),
                            event_names.0[event as usize],
                        )
                    };
                    // Clear tool to prevent infinite loop
                    Py_CLEAR!(
                        *unsafe {
                            (*((*interp).monitoring_callables.as_mut_ptr()
                                as *mut [*mut _object; 19])
                                .wrapping_offset(tool as isize))
                            .as_mut_ptr() as *mut *mut _object
                        }
                        .wrapping_offset(event as isize)
                    );
                    err = 1i32.wrapping_neg();
                    break;
                } else {
                    let interp: *mut _is = unsafe { (*tstate).interp };
                    unsafe { _PyEval_StopTheWorld(interp) };
                    remove_tools(code, offset, event, 1i32 << tool);
                    unsafe { _PyEval_StartTheWorld(interp) };
                }
            }
        }
    }
    Py_DECREF!(arg2_obj);
    err
}

#[inline(never)]
#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_call_instrumentation(
    tstate: *mut _ts,
    event: i32,
    frame: *mut _PyInterpreterFrame,
    instr: *mut __SlateRecord_pycore_structs_h_32,
) -> i32 {
    let mut args: __SlateAlign16<[*mut _object; 3]> = __SlateAlign16([
        core::ptr::null_mut::<_object>(),
        core::ptr::null_mut::<_object>(),
        core::ptr::null_mut::<_object>(),
    ]);
    call_instrumentation_vector(
        instr,
        tstate,
        event,
        frame,
        instr,
        2i64,
        args.0.as_mut_ptr() as *mut *mut _object,
    )
}

#[inline(never)]
#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_call_instrumentation_arg(
    tstate: *mut _ts,
    event: i32,
    frame: *mut _PyInterpreterFrame,
    instr: *mut __SlateRecord_pycore_structs_h_32,
    arg: *mut _object,
) -> i32 {
    let mut args: __SlateAlign16<[*mut _object; 4]> = __SlateAlign16([
        core::ptr::null_mut::<_object>(),
        core::ptr::null_mut::<_object>(),
        core::ptr::null_mut::<_object>(),
        arg,
    ]);
    call_instrumentation_vector(
        instr,
        tstate,
        event,
        frame,
        instr,
        3i64,
        args.0.as_mut_ptr() as *mut *mut _object,
    )
}

#[inline(never)]
#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_call_instrumentation_2args(
    tstate: *mut _ts,
    event: i32,
    frame: *mut _PyInterpreterFrame,
    instr: *mut __SlateRecord_pycore_structs_h_32,
    arg0: *mut _object,
    arg1: *mut _object,
) -> i32 {
    let mut args: __SlateAlign16<[*mut _object; 5]> = __SlateAlign16([
        core::ptr::null_mut::<_object>(),
        core::ptr::null_mut::<_object>(),
        core::ptr::null_mut::<_object>(),
        arg0,
        arg1,
    ]);
    call_instrumentation_vector(
        instr,
        tstate,
        event,
        frame,
        instr,
        4i64,
        args.0.as_mut_ptr() as *mut *mut _object,
    )
}

#[inline(never)]
#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_call_instrumentation_jump(
    instr: *mut __SlateRecord_pycore_structs_h_32,
    tstate: *mut _ts,
    event: i32,
    frame: *mut _PyInterpreterFrame,
    src: *mut __SlateRecord_pycore_structs_h_32,
    dest: *mut __SlateRecord_pycore_structs_h_32,
) -> *mut __SlateRecord_pycore_structs_h_32 {
    let to: i32 = ((unsafe {
        dest.offset_from(_PyFrame_GetBytecode(frame) as *mut __SlateRecord_pycore_structs_h_32)
    }) as i64) as i32;
    let to_obj: *mut _object = unsafe {
        PyLong_FromLong(
            to.wrapping_mul(core::mem::size_of::<__SlateRecord_pycore_structs_h_32>() as i32)
                as i64,
        )
    };
    if to_obj.is_null() {
        return core::ptr::null_mut::<__SlateRecord_pycore_structs_h_32>();
    }
    let mut args: __SlateAlign16<[*mut _object; 4]> = __SlateAlign16([
        core::ptr::null_mut::<_object>(),
        core::ptr::null_mut::<_object>(),
        core::ptr::null_mut::<_object>(),
        to_obj,
    ]);
    let instr_ptr: *mut __SlateRecord_pycore_structs_h_32 = unsafe { (*frame).instr_ptr };
    let err: i32 = call_instrumentation_vector(
        instr,
        tstate,
        event,
        frame,
        src,
        3i64,
        args.0.as_mut_ptr() as *mut *mut _object,
    );
    Py_DECREF!(to_obj);
    if err != 0 {
        return core::ptr::null_mut::<__SlateRecord_pycore_structs_h_32>();
    }
    if (unsafe { (*frame).instr_ptr }) != instr_ptr {
        // The callback has caused a jump (by setting the line number)
        return unsafe { (*frame).instr_ptr };
    }
    dest
}

fn call_instrumentation_vector_protected(
    tstate: *mut _ts,
    event: i32,
    frame: *mut _PyInterpreterFrame,
    instr: *mut __SlateRecord_pycore_structs_h_32,
    nargs: i64,
    args: *mut *mut _object,
) {
    let exc: *mut _object = unsafe { _PyErr_GetRaisedException(tstate) };
    let err: i32 = call_instrumentation_vector(instr, tstate, event, frame, instr, nargs, args);
    if err != 0 {
        Py_XDECREF!(exc);
    } else {
        unsafe { _PyErr_SetRaisedException(tstate, exc) };
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_call_instrumentation_exc2(
    tstate: *mut _ts,
    event: i32,
    frame: *mut _PyInterpreterFrame,
    instr: *mut __SlateRecord_pycore_structs_h_32,
    arg0: *mut _object,
    arg1: *mut _object,
) {
    let mut args: __SlateAlign16<[*mut _object; 5]> = __SlateAlign16([
        core::ptr::null_mut::<_object>(),
        core::ptr::null_mut::<_object>(),
        core::ptr::null_mut::<_object>(),
        arg0,
        arg1,
    ]);
    call_instrumentation_vector_protected(
        tstate,
        event,
        frame,
        instr,
        4i64,
        args.0.as_mut_ptr() as *mut *mut _object,
    );
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_Instrumentation_GetLine(
    code: *mut PyCodeObject,
    line_data: *mut __SlateRecord_pycore_instruments_h_94,
    index: i32,
) -> i32 {
    let line_delta: i32 = get_line_delta(line_data, index);
    let line: i32 = compute_line(code, line_delta);
    line
}

#[inline(never)]
#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_call_instrumentation_line(
    tstate: *mut _ts,
    frame: *mut _PyInterpreterFrame,
    instr: *mut __SlateRecord_pycore_structs_h_32,
    prev: *mut __SlateRecord_pycore_structs_h_32,
) -> i32 {
    let mut res: i32 = unsafe { core::mem::zeroed() };
    let mut tool: i32 = unsafe { core::mem::zeroed() };
    let mut args: core::mem::MaybeUninit<__SlateAlign16<[*mut _object; 3]>> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_6: *mut [*mut _object; 3] =
        core::ptr::addr_of_mut!(args) as *mut [*mut _object; 3];
    let mut line_obj: *mut _object = unsafe { core::mem::zeroed() };
    let mut frame_obj: *mut _frame = unsafe { core::mem::zeroed() };
    let mut tools: u8 = unsafe { core::mem::zeroed() };
    let mut prev_index: i32 = unsafe { core::mem::zeroed() };
    let mut line: i32 = unsafe { core::mem::zeroed() };
    let mut interp_1: *mut _is = unsafe { core::mem::zeroed() };
    let mut line_data: *mut __SlateRecord_pycore_instruments_h_94 = unsafe { core::mem::zeroed() };
    let mut i: i32 = unsafe { core::mem::zeroed() };
    let mut bytecode: *mut __SlateRecord_pycore_structs_h_32 = unsafe { core::mem::zeroed() };
    let mut code: *mut PyCodeObject = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_0: {
            code = _PyFrame_GetCode(frame);
            bytecode = _PyFrame_GetBytecode(frame);
            i = (instr.offset_from(bytecode as *mut __SlateRecord_pycore_structs_h_32) as i64)
                as i32;
            let monitoring: *mut _PyCoMonitoringData = (*code)._co_monitoring;
            line_data = (*monitoring).lines;
            interp_1 = (*tstate).interp;
            line = _Py_Instrumentation_GetLine(code, line_data, i);
            prev_index = (prev.offset_from(bytecode as *mut __SlateRecord_pycore_structs_h_32)
                as i64) as i32;
            let prev_line: i32 = _Py_Instrumentation_GetLine(code, line_data, prev_index);
            if prev_line == line {
                let prev_opcode: i32 =
                    (*bytecode.wrapping_offset(prev_index as isize)).op.code as i32;
                // RESUME and INSTRUMENTED_RESUME are needed for the operation of
                // instrumentation, so must never be hidden by an INSTRUMENTED_LINE.
                if prev_opcode != 128 && prev_opcode != 245 {
                    break '__join_0;
                }
            }
            tools = (if !unsafe { (*(*code)._co_monitoring).line_tools }.is_null() {
                *unsafe { (*(*code)._co_monitoring).line_tools }.wrapping_offset(i as isize) as i32
            } else {
                (*((*interp_1).monitors.tools.as_mut_ptr() as *mut u8).wrapping_offset(5 as isize)
                    as i32)
                    | (*((*(*code)._co_monitoring).local_monitors.tools.as_mut_ptr() as *mut u8)
                        .wrapping_offset(5 as isize) as i32)
            }) as u8;
            // Special case sys.settrace to avoid boxing the line number,
            // only to immediately unbox it.
            if (tools as i32) & 128i32 != 0 {
                if (*tstate).c_tracefunc != None {
                    frame_obj = _PyFrame_GetFrameObject(frame);
                    if frame_obj.is_null() {
                        return 1i32.wrapping_neg();
                    } else {
                        if (*frame_obj).f_trace_lines != 0 {
                            // Need to set tracing and what_event as if using
                            // the instrumentation call.
                            let old_what: i32 = (*tstate).what_event;
                            (*tstate).what_event = 5;
                            let __v15: i32 = (*tstate).tracing;
                            (*tstate).tracing = __v15.wrapping_add(1);
                            // Call c_tracefunc directly, having set the line number.
                            Py_INCREF__2!(frame_obj);
                            (*frame_obj).f_lineno = line;
                            let err: i32 = unsafe { (*tstate).c_tracefunc }.unwrap()(
                                (*tstate).c_traceobj,
                                frame_obj,
                                2i32,
                                core::ptr::addr_of_mut!(_Py_NoneStruct),
                            );
                            (*frame_obj).f_lineno = 0;
                            let __v11: i32 = (*tstate).tracing;
                            (*tstate).tracing = __v11.wrapping_sub(1);
                            (*tstate).what_event = old_what;
                            Py_DECREF__2!(frame_obj);
                            if err != 0 {
                                return 1i32.wrapping_neg();
                            }
                        }
                    }
                }
                tools = ((tools as i32) & 127i32) as u8;
            }
            if (tools as i32) != 0 {
                line_obj = PyLong_FromLong(line as i64);
                if line_obj.is_null() {
                    return 1i32.wrapping_neg();
                } else {
                    core::ptr::write(
                        __slate_slot_6,
                        [
                            core::ptr::null_mut::<_object>(),
                            code as *mut _object,
                            line_obj,
                        ],
                    );
                    '__join_1: {
                        loop {
                            tool = most_significant_bit(tools);
                            let __v3: u8 = ((tools as i32) & !(1i32 << tool)) as u8;
                            tools = __v3;
                            res = call_one_instrument(
                                interp_1,
                                tstate,
                                ((*__slate_slot_6).as_mut_ptr() as *mut *mut _object)
                                    .wrapping_offset(1 as isize),
                                2 | _Py_STATIC_CAST!(u64, 1i32)
                                    << 8u64
                                        .wrapping_mul(core::mem::size_of::<u64>() as u64)
                                        .wrapping_sub(1),
                                tool as i8,
                                5i32,
                            );
                            if res == 0 {
                                // Nothing to do
                            } else {
                                if res < 0 {
                                    break;
                                } else {
                                    // DISABLE
                                    let interp: *mut _is = (*tstate).interp;
                                    _PyEval_StopTheWorld(interp);
                                    remove_line_tools(code, i, 1i32 << tool);
                                    _PyEval_StartTheWorld(interp);
                                }
                            }
                            if tools == 0 {
                                break '__join_1;
                            }
                        }
                        // error
                        Py_DECREF!(line_obj);
                        return 1i32.wrapping_neg();
                    }
                    Py_DECREF!(line_obj);
                }
            }
        }
        let original_opcode: u8 = get_original_opcode(line_data, i);
        return original_opcode as i32;
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_call_instrumentation_instruction(
    tstate: *mut _ts,
    frame: *mut _PyInterpreterFrame,
    instr: *mut __SlateRecord_pycore_structs_h_32,
) -> i32 {
    let code: *mut PyCodeObject = _PyFrame_GetCode(frame);
    let offset: i32 = ((unsafe {
        instr.offset_from(_PyFrame_GetBytecode(frame) as *mut __SlateRecord_pycore_structs_h_32)
    }) as i64) as i32;
    let instrumentation_data: *mut _PyCoMonitoringData = unsafe { (*code)._co_monitoring };
    let next_opcode: i32 = (unsafe {
        *unsafe { (*instrumentation_data).per_instruction_opcodes }.wrapping_offset(offset as isize)
    }) as i32;
    if (unsafe { (*tstate).tracing }) != 0 {
        return next_opcode;
    }
    let interp: *mut _is = unsafe { (*tstate).interp };
    let mut tools: u8 = (if !unsafe { (*instrumentation_data).per_instruction_tools }.is_null() {
        (unsafe {
            *unsafe { (*instrumentation_data).per_instruction_tools }
                .wrapping_offset(offset as isize)
        }) as i32
    } else {
        ((unsafe {
            *((*interp).monitors.tools.as_mut_ptr() as *mut u8).wrapping_offset(6 as isize)
        }) as i32)
            | ((unsafe {
                *((*(*code)._co_monitoring).local_monitors.tools.as_mut_ptr() as *mut u8)
                    .wrapping_offset(6 as isize)
            }) as i32)
    }) as u8;
    let bytes_offset: i32 =
        offset.wrapping_mul(core::mem::size_of::<__SlateRecord_pycore_structs_h_32>() as i32);
    let offset_obj: *mut _object = unsafe { PyLong_FromLong(bytes_offset as i64) };
    if offset_obj.is_null() {
        return 1i32.wrapping_neg();
    }
    let mut args: __SlateAlign16<[*mut _object; 3]> = __SlateAlign16([
        core::ptr::null_mut::<_object>(),
        code as *mut _object,
        offset_obj,
    ]);
    while tools != 0 {
        let tool: i32 = most_significant_bit(tools);
        let __v1: u8 = ((tools as i32) & !(1i32 << tool)) as u8;
        tools = __v1;
        let res: i32 = call_one_instrument(
            interp,
            tstate,
            (args.0.as_mut_ptr() as *mut *mut _object).wrapping_offset(1 as isize),
            2 | _Py_STATIC_CAST!(u64, 1i32)
                << 8u64
                    .wrapping_mul(core::mem::size_of::<u64>() as u64)
                    .wrapping_sub(1),
            tool as i8,
            6i32,
        );
        if res == 0 {
            // Nothing to do
        } else {
            if res < 0 {
                // error
                Py_DECREF!(offset_obj);
                return 1i32.wrapping_neg();
            } else {
                // DISABLE
                let interp: *mut _is = unsafe { (*tstate).interp };
                unsafe { _PyEval_StopTheWorld(interp) };
                remove_per_instruction_tools(code, offset, 1i32 << tool);
                unsafe { _PyEval_StartTheWorld(interp) };
            }
        }
    }
    Py_DECREF!(offset_obj);
    next_opcode
}

fn initialize_tools(code: *mut PyCodeObject) {
    let tools: *mut u8 = unsafe { (*(*code)._co_monitoring).tools };
    let code_len: i32 = Py_SIZE!(code) as i32;
    let mut i: i32 = 0;
    while i < code_len {
        let instr: *mut __SlateRecord_pycore_structs_h_32 =
            _PyCode_CODE!(code).wrapping_offset(i as isize);
        let mut opcode: i32 = (unsafe { (*instr).op.code }) as i32;
        if opcode == 254 {
            opcode = get_original_opcode(unsafe { (*(*code)._co_monitoring).lines }, i) as i32;
        }
        if opcode == 238 {
            opcode = (unsafe {
                *unsafe { (*(*code)._co_monitoring).per_instruction_opcodes }
                    .wrapping_offset(i as isize)
            }) as i32;
        }
        let instrumented: bool = is_instrumented(opcode);
        if instrumented {
            opcode = (unsafe { DE_INSTRUMENT.0[opcode as usize] }) as i32;
        }
        opcode = (unsafe { _PyOpcode_Deopt[opcode as usize] }) as i32;
        if opcode_has_event(opcode) {
            if instrumented {
                let event: i8 = if opcode == 128 {
                    (((unsafe { (*instr).op.arg }) as i32) != 0) as i8
                } else {
                    unsafe { EVENT_FOR_OPCODE.0[opcode as usize] }
                };
                unsafe {
                    *tools.wrapping_offset(i as isize) =
                        *((*(*code)._co_monitoring).active_monitors.tools.as_mut_ptr() as *mut u8)
                            .wrapping_offset((event as i32) as isize);
                }
                CHECK__2!();
            } else {
                unsafe {
                    *tools.wrapping_offset(i as isize) = 0;
                }
            }
        }
        i = i.wrapping_add((unsafe { _PyOpcode_Caches[opcode as usize] }) as i32);
        i = i.wrapping_add(1);
    }
}

fn initialize_lines(
    line_data: *mut __SlateRecord_pycore_instruments_h_94,
    code: *mut PyCodeObject,
    bytes_per_entry: i32,
) {
    let mut __slate_compound_1: __SlateRecord_pycore_structs_h_32 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_0: __SlateRecord_pycore_structs_h_32_2 =
        unsafe { core::mem::zeroed() };
    let mut __slate_compound_16: __SlateRecord_pycore_structs_h_32 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_15: __SlateRecord_pycore_structs_h_32_2 =
        unsafe { core::mem::zeroed() };
    let mut __slate_compound_18: __SlateRecord_pycore_structs_h_32 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_17: __SlateRecord_pycore_structs_h_32_2 =
        unsafe { core::mem::zeroed() };
    unsafe {
        (*line_data).bytes_per_entry = bytes_per_entry as u8;
    }
    let code_len: i32 = Py_SIZE!(code) as i32;
    let mut range: _line_offsets = unsafe { core::mem::zeroed() };
    unsafe { _PyCode_InitAddressRange(code, core::ptr::addr_of_mut!(range)) };
    let mut current_line: i32 = 1i32.wrapping_neg();
    {
        let mut i: i32 = 0;
        '__slate_break_6: while i < code_len {
            let opcode: i32 = (unsafe {
                (*{
                    __slate_compound_0 = (*{
                        __slate_compound_1 = _Py_GetBaseCodeUnit(code, i);
                        core::ptr::addr_of_mut!(__slate_compound_1)
                    })
                    .op;
                    core::ptr::addr_of_mut!(__slate_compound_0)
                })
                .code
            }) as i32;
            let line: i32 = unsafe {
                _PyCode_CheckLineNumber(i.wrapping_mul(core::mem::size_of::<__SlateRecord_pycore_structs_h_32>() as i32), core::ptr::addr_of_mut!(range))
            };
            set_line_delta(line_data, i, compute_line_delta(code, line));
            let length: i32 = _PyInstruction_GetLength(code, i);
            if i < unsafe { (*code)._co_firsttraceable } {
                set_original_opcode(line_data, i, 0u8);
            } else {
                match opcode {
                    9 | 10 | 30 | 68 | 128 => {
                        set_original_opcode(line_data, i, 0u8);
                        // END_FOR cannot start a line, as it is skipped by FOR_ITER
                        // END_SEND cannot start a line, as it is skipped by SEND
                        // RESUME and POP_ITER must not be instrumented with INSTRUMENTED_LINE
                    }
                    _ => {
                        if line != current_line && line >= 0 {
                            set_original_opcode(line_data, i, opcode as u8);
                            CHECK__2!();
                        } else {
                            set_original_opcode(line_data, i, 0u8);
                        }
                        // Set original_opcode to the opcode iff the instruction
                        // starts a line, and thus should be instrumented.
                        // This saves having to perform this check every time the
                        // we turn instrumentation on or off, and serves as a sanity
                        // check when debugging.
                        current_line = line;
                    }
                }
            }
            for j in 1..length {
                set_original_opcode(line_data, i.wrapping_add(j), 0u8);
                set_line_delta(line_data, i.wrapping_add(j), 2i32.wrapping_neg());
            }
            i = i.wrapping_add(length);
        }
    }
    let mut i: i32 = unsafe { (*code)._co_firsttraceable };
    '__slate_break_14: while i < code_len {
        '__slate_continue_14: {
            let mut inst: __SlateRecord_pycore_structs_h_32 = _Py_GetBaseCodeUnit(code, i);
            let mut opcode_1: i32 = (unsafe { inst.op.code }) as i32;
            let mut oparg: i32 = 0;
            while opcode_1 == 69 {
                oparg = oparg << 8i32 | ((unsafe { inst.op.arg }) as i32);
                i = i.wrapping_add(1);
                inst = _Py_GetBaseCodeUnit(code, i);
                opcode_1 = (unsafe { inst.op.code }) as i32;
            }
            oparg = oparg << 8i32 | ((unsafe { inst.op.arg }) as i32);
            let __v10: i32 = _PyInstruction_GetLength(code, i);
            i = i.wrapping_add(__v10);
            let mut target: i32 = 1i32.wrapping_neg();
            match opcode_1 {
                77 | 100..=103 => {
                    target = i.wrapping_add(oparg);
                }
                70 | 106 => {
                    // Skip over END_FOR/END_SEND
                    target = i.wrapping_add(oparg).wrapping_add(1);
                }
                75 | 76 => {
                    target = i.wrapping_sub(oparg);
                }
                _ => {
                    break '__slate_continue_14;
                }
            }
            if get_line_delta(line_data, target) != 2i32.wrapping_neg() {
                let opcode: i32 = (unsafe {
                    (*{
                        __slate_compound_15 = (*{
                            __slate_compound_16 = _Py_GetBaseCodeUnit(code, target);
                            core::ptr::addr_of_mut!(__slate_compound_16)
                        })
                        .op;
                        core::ptr::addr_of_mut!(__slate_compound_15)
                    })
                    .code
                }) as i32;
                if opcode != 30 {
                    set_original_opcode(line_data, target, opcode as u8);
                }
            }
        }
    }
    // Scan exception table
    let start: *mut u8 = PyBytes_AS_STRING!(unsafe { (*code).co_exceptiontable }) as *mut u8;
    let end: *mut u8 =
        start.wrapping_offset(PyBytes_GET_SIZE!(unsafe { (*code).co_exceptiontable }) as isize);
    let mut scan: *mut u8 = start;
    '__slate_break_19: while scan < end {
        let mut start_offset: i32 = 0;
        let mut size: i32 = 0;
        let mut handler: i32 = 0;
        scan = parse_varint(scan, unsafe {
            &mut (*core::ptr::addr_of_mut!(start_offset))
        });
        scan = parse_varint(scan, unsafe { &mut (*core::ptr::addr_of_mut!(size)) });
        scan = parse_varint(scan, unsafe { &mut (*core::ptr::addr_of_mut!(handler)) });
        let mut depth_and_lasti: i32 = 0;
        scan = parse_varint(scan, unsafe {
            &mut (*core::ptr::addr_of_mut!(depth_and_lasti))
        });
        let original_opcode: i32 = (unsafe {
            (*{
                __slate_compound_17 = (*{
                    __slate_compound_18 = _Py_GetBaseCodeUnit(code, handler);
                    core::ptr::addr_of_mut!(__slate_compound_18)
                })
                .op;
                core::ptr::addr_of_mut!(__slate_compound_17)
            })
            .code
        }) as i32;
        // Skip if not the start of a line.
        // END_ASYNC_FOR is a bit special as it marks the end of
        // an `async for` loop, which should not generate its own
        // line event.
        if get_line_delta(line_data, handler) != 2i32.wrapping_neg() && original_opcode != 68 {
            set_original_opcode(line_data, handler, original_opcode as u8);
        }
    }
}

fn initialize_line_tools(code: *mut PyCodeObject, all_events: &mut _Py_LocalMonitors) {
    let line_tools: *mut u8 = unsafe { (*(*code)._co_monitoring).line_tools };
    let code_len: i32 = Py_SIZE!(code) as i32;
    for i in 0..code_len {
        unsafe {
            *line_tools.wrapping_offset(i as isize) =
                *((*(all_events as *mut _Py_LocalMonitors)).tools.as_mut_ptr() as *mut u8)
                    .wrapping_offset(5 as isize);
        }
    }
}

fn allocate_instrumentation_data(code: *mut PyCodeObject) -> i32 {
    let mut __slate_compound_0: _Py_LocalMonitors = unsafe { core::mem::zeroed() };
    let mut __slate_compound_1: _Py_LocalMonitors = unsafe { core::mem::zeroed() };
    if unsafe { (*code)._co_monitoring }.is_null() {
        let monitoring: *mut _PyCoMonitoringData =
            (unsafe { PyMem_Malloc(core::mem::size_of::<_PyCoMonitoringData>() as u64) })
                as *mut _PyCoMonitoringData;
        if monitoring.is_null() {
            unsafe { PyErr_NoMemory() };
            return 1i32.wrapping_neg();
        }
        unsafe {
            (*monitoring).local_monitors = *{
                __slate_compound_0 = _Py_LocalMonitors {
                    tools: {
                        let mut __t0: [u8; 11] = core::mem::zeroed();
                        __t0[0] = 0u8;
                        __t0
                    },
                };
                core::ptr::addr_of_mut!(__slate_compound_0)
            };
        }
        unsafe {
            (*monitoring).active_monitors = *{
                __slate_compound_1 = _Py_LocalMonitors {
                    tools: {
                        let mut __t1: [u8; 11] = core::mem::zeroed();
                        __t1[0] = 0u8;
                        __t1
                    },
                };
                core::ptr::addr_of_mut!(__slate_compound_1)
            };
        }
        unsafe {
            (*monitoring).tools = core::ptr::null_mut::<u8>();
        }
        unsafe {
            (*monitoring).lines = core::ptr::null_mut::<__SlateRecord_pycore_instruments_h_94>();
        }
        unsafe {
            (*monitoring).line_tools = core::ptr::null_mut::<u8>();
        }
        unsafe {
            (*monitoring).per_instruction_opcodes = core::ptr::null_mut::<u8>();
        }
        unsafe {
            (*monitoring).per_instruction_tools = core::ptr::null_mut::<u8>();
        }
        _Py_atomic_store_ptr_release(
            (unsafe { core::ptr::addr_of_mut!((*code)._co_monitoring) }) as *mut (),
            monitoring as *mut (),
        );
    }
    0
}

fn update_instrumentation_data(code: *mut PyCodeObject, interp: *mut _is) -> i32 {
    let code_len: i32 = Py_SIZE!(code) as i32;
    if allocate_instrumentation_data(code) != 0 {
        return 1i32.wrapping_neg();
    }
    // If the local monitors are out of date, clear them up
    let local_monitors: *mut _Py_LocalMonitors =
        unsafe { core::ptr::addr_of_mut!((*(*code)._co_monitoring).local_monitors) };
    for i in 0i32..8i32 {
        if (unsafe {
            *((*(*code)._co_monitoring).tool_versions.as_mut_ptr() as *mut u64)
                .wrapping_offset(i as isize)
        }) != unsafe {
            *((*interp).monitoring_tool_versions.as_mut_ptr() as *mut u64)
                .wrapping_offset(i as isize)
        } {
            for j in 0i32..11i32 {
                let __v0: *mut u8 = unsafe { (*local_monitors).tools.as_mut_ptr() as *mut u8 }
                    .wrapping_offset(j as isize);
                let __v2: u8 = (((unsafe { *__v0 }) as i32) & !(1i32 << i)) as u8;
                unsafe {
                    *__v0 = __v2;
                }
            }
        }
    }
    let mut all_events: _Py_LocalMonitors = local_union(unsafe { (*interp).monitors }, unsafe {
        (*(*code)._co_monitoring).local_monitors
    });
    let multitools: bool = multiple_tools(unsafe { &mut (*core::ptr::addr_of_mut!(all_events)) });
    if unsafe { (*(*code)._co_monitoring).tools }.is_null() && multitools {
        unsafe {
            (*(*code)._co_monitoring).tools = PyMem_Malloc(code_len as u64) as *mut u8;
        }
        if unsafe { (*(*code)._co_monitoring).tools }.is_null() {
            unsafe { PyErr_NoMemory() };
            return 1i32.wrapping_neg();
        }
        initialize_tools(code);
    }
    if all_events.tools[5] != 0 {
        if unsafe { (*(*code)._co_monitoring).lines }.is_null() {
            let mut range: _line_offsets = unsafe { core::mem::zeroed() };
            unsafe { _PyCode_InitAddressRange(code, core::ptr::addr_of_mut!(range)) };
            let mut max_line: i32 = unsafe { (*code).co_firstlineno }.wrapping_add(1);
            unsafe { _PyCode_InitAddressRange(code, core::ptr::addr_of_mut!(range)) };
            let mut i: i32 = unsafe { (*code)._co_firsttraceable };
            while i < code_len {
                let line: i32 = unsafe {
                    _PyCode_CheckLineNumber(
                        i.wrapping_mul(
                            core::mem::size_of::<__SlateRecord_pycore_structs_h_32>() as i32
                        ),
                        core::ptr::addr_of_mut!(range),
                    )
                };
                if line > max_line {
                    max_line = line;
                }
                let length: i32 = _PyInstruction_GetLength(code, i);
                i = i.wrapping_add(length);
            }
            let mut bytes_per_entry: i32 = 0;
            let max_delta: i32 = max_line.wrapping_sub(unsafe { (*code).co_firstlineno });
            // We store delta+2 in the table, so 253 is max for one byte
            if max_delta < 256i32.wrapping_add(2i32.wrapping_neg()) {
                bytes_per_entry = 2;
            } else {
                if max_delta < 65536i32.wrapping_add(2i32.wrapping_neg()) {
                    bytes_per_entry = 3;
                } else {
                    if max_delta < 16777216i32.wrapping_add(2i32.wrapping_neg()) {
                        bytes_per_entry = 4;
                    } else {
                        bytes_per_entry = 5;
                    }
                }
            }
            let lines: *mut __SlateRecord_pycore_instruments_h_94 = (unsafe {
                PyMem_Malloc(1i32.wrapping_add(code_len.wrapping_mul(bytes_per_entry)) as u64)
            })
                as *mut __SlateRecord_pycore_instruments_h_94;
            if lines.is_null() {
                unsafe { PyErr_NoMemory() };
                return 1i32.wrapping_neg();
            }
            initialize_lines(lines, code, bytes_per_entry);
            _Py_atomic_store_ptr_release(
                (unsafe { core::ptr::addr_of_mut!((*(*code)._co_monitoring).lines) }) as *mut (),
                lines as *mut (),
            );
        }
        if multitools && unsafe { (*(*code)._co_monitoring).line_tools }.is_null() {
            unsafe {
                (*(*code)._co_monitoring).line_tools = PyMem_Malloc(code_len as u64) as *mut u8;
            }
            if unsafe { (*(*code)._co_monitoring).line_tools }.is_null() {
                unsafe { PyErr_NoMemory() };
                return 1i32.wrapping_neg();
            }
            initialize_line_tools(code, unsafe { &mut (*core::ptr::addr_of_mut!(all_events)) });
        }
    }
    if all_events.tools[6] != 0 {
        if unsafe { (*(*code)._co_monitoring).per_instruction_opcodes }.is_null() {
            unsafe {
                (*(*code)._co_monitoring).per_instruction_opcodes =
                    PyMem_Malloc((code_len as u64).wrapping_mul(core::mem::size_of::<
                        __SlateRecord_pycore_instruments_h_94,
                    >() as u64)) as *mut u8;
            }
            if unsafe { (*(*code)._co_monitoring).per_instruction_opcodes }.is_null() {
                unsafe { PyErr_NoMemory() };
                return 1i32.wrapping_neg();
            }
            // Initialize all of the instructions so if local events change while another thread is executing
            //
            // we know what the original opcode was.
            for i in 0..code_len {
                let opcode: i32 =
                    (unsafe { (*_PyCode_CODE!(code).wrapping_offset(i as isize)).op.code }) as i32;
                unsafe {
                    *unsafe { (*(*code)._co_monitoring).per_instruction_opcodes }
                        .wrapping_offset(i as isize) = _PyOpcode_Deopt[opcode as usize];
                }
            }
        }
        if multitools && unsafe { (*(*code)._co_monitoring).per_instruction_tools }.is_null() {
            unsafe {
                (*(*code)._co_monitoring).per_instruction_tools =
                    PyMem_Malloc(code_len as u64) as *mut u8;
            }
            if unsafe { (*(*code)._co_monitoring).per_instruction_tools }.is_null() {
                unsafe { PyErr_NoMemory() };
                return 1i32.wrapping_neg();
            }
            for i in 0..code_len {
                unsafe {
                    *unsafe { (*(*code)._co_monitoring).per_instruction_tools }
                        .wrapping_offset(i as isize) = 0;
                }
            }
        }
    }
    0
}

fn force_instrument_lock_held(code: *mut PyCodeObject, interp: *mut _is) -> i32 {
    let mut __slate_compound_46: __SlateRecord_pycore_structs_h_32 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_45: __SlateRecord_pycore_structs_h_32_2 =
        unsafe { core::mem::zeroed() };
    let mut __slate_compound_48: __SlateRecord_pycore_structs_h_32 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_47: __SlateRecord_pycore_structs_h_32_2 =
        unsafe { core::mem::zeroed() };
    let mut i: i32 = unsafe { core::mem::zeroed() };
    let mut i_1: i32 = unsafe { core::mem::zeroed() };
    let mut line_data: *mut __SlateRecord_pycore_instruments_h_94 = unsafe { core::mem::zeroed() };
    let mut new_per_instruction_tools: u8 = unsafe { core::mem::zeroed() };
    let mut new_line_tools: u8 = unsafe { core::mem::zeroed() };
    let mut i_2: i32 = unsafe { core::mem::zeroed() };
    let mut i_3: i32 = unsafe { core::mem::zeroed() };
    let mut line_data_1: *mut __SlateRecord_pycore_instruments_h_94 =
        unsafe { core::mem::zeroed() };
    let mut removed_per_instruction_tools: u8 = unsafe { core::mem::zeroed() };
    // GH-103845: We need to remove both the line and instruction instrumentation before
    //
    // adding new ones, otherwise we may remove the newly added instrumentation.
    let mut removed_line_tools: u8 = unsafe { core::mem::zeroed() };
    let mut new_tools: u8 = unsafe { core::mem::zeroed() };
    let mut removed_tools: u8 = unsafe { core::mem::zeroed() };
    let mut event: i8 = unsafe { core::mem::zeroed() };
    let mut base_opcode: i32 = unsafe { core::mem::zeroed() };
    let mut instr: core::mem::MaybeUninit<__SlateRecord_pycore_structs_h_32> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_37: *mut __SlateRecord_pycore_structs_h_32 =
        core::ptr::addr_of_mut!(instr) as *mut __SlateRecord_pycore_structs_h_32;
    // Insert instrumentation
    let mut i_4: i32 = unsafe { core::mem::zeroed() };
    let mut __v39: bool = unsafe { core::mem::zeroed() };
    let mut removed_events: core::mem::MaybeUninit<_Py_LocalMonitors> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_41: *mut _Py_LocalMonitors =
        core::ptr::addr_of_mut!(removed_events) as *mut _Py_LocalMonitors;
    let mut new_events: core::mem::MaybeUninit<_Py_LocalMonitors> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_42: *mut _Py_LocalMonitors =
        core::ptr::addr_of_mut!(new_events) as *mut _Py_LocalMonitors;
    let mut active_events: core::mem::MaybeUninit<_Py_LocalMonitors> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_43: *mut _Py_LocalMonitors =
        core::ptr::addr_of_mut!(active_events) as *mut _Py_LocalMonitors;
    let mut code_len: i32 = unsafe { core::mem::zeroed() };
    unsafe {
        code_len = Py_SIZE!(code) as i32;
        // Exit early to avoid creating instrumentation
        // data for potential statically allocated code
        // objects.
        // See https://github.com/python/cpython/issues/108390
        if (*code).co_flags & 33554432 != 0 {
            return 0;
        } else {
            if update_instrumentation_data(code, interp) != 0 {
                return 1i32.wrapping_neg();
            } else {
                core::ptr::write(
                    __slate_slot_43,
                    local_union((*interp).monitors, (*(*code)._co_monitoring).local_monitors),
                );
                let restarted: bool =
                    (*interp).last_restart_version > (*code)._co_instrumentation_version;
                if restarted {
                    *__slate_slot_41 = (*(*code)._co_monitoring).active_monitors;
                    *__slate_slot_42 = *__slate_slot_43;
                } else {
                    *__slate_slot_41 =
                        monitors_sub((*(*code)._co_monitoring).active_monitors, *__slate_slot_43);
                    *__slate_slot_42 =
                        monitors_sub(*__slate_slot_43, (*(*code)._co_monitoring).active_monitors);
                }
                (*(*code)._co_monitoring).active_monitors = *__slate_slot_43;
                if monitors_are_empty(*__slate_slot_42) {
                    __v39 = monitors_are_empty(*__slate_slot_41);
                } else {
                    __v39 = false;
                }
                if !__v39 {
                    i_4 = (*code)._co_firsttraceable;
                    while i_4 < code_len {
                        core::ptr::write(__slate_slot_37, _Py_GetBaseCodeUnit(code, i_4));
                        CHECK__2!();
                        base_opcode = (*__slate_slot_37).op.code as i32;
                        if opcode_has_event(base_opcode) {
                            if base_opcode == 128 {
                                event = (((*__slate_slot_37).op.arg as i32) > 0) as i8;
                            } else {
                                event = EVENT_FOR_OPCODE.0[base_opcode as usize];
                            }
                            removed_tools = (*__slate_slot_41).tools[(event as i32) as usize];
                            if removed_tools != 0 {
                                remove_tools(code, i_4, event as i32, removed_tools as i32);
                            }
                            new_tools = (*__slate_slot_42).tools[(event as i32) as usize];
                            if new_tools != 0 {
                                add_tools(code, i_4, event as i32, new_tools as i32);
                            }
                        }
                        let __v32: i32 = _PyInstruction_GetLength(code, i_4);
                        i_4 = i_4.wrapping_add(__v32);
                    }
                    removed_line_tools = (*__slate_slot_41).tools[5];
                    removed_per_instruction_tools = (*__slate_slot_41).tools[6];
                    if removed_line_tools != 0 {
                        line_data_1 = (*(*code)._co_monitoring).lines;
                        i_3 = (*code)._co_firsttraceable;
                        while i_3 < code_len {
                            if get_original_opcode(line_data_1, i_3) != 0 {
                                remove_line_tools(code, i_3, removed_line_tools as i32);
                            }
                            let __v25: i32 = _PyInstruction_GetLength(code, i_3);
                            i_3 = i_3.wrapping_add(__v25);
                        }
                    }
                    if removed_per_instruction_tools != 0 {
                        i_2 = (*code)._co_firsttraceable;
                        while i_2 < code_len {
                            let opcode: i32 = (*{
                                __slate_compound_47 = (*{
                                    __slate_compound_48 = _Py_GetBaseCodeUnit(code, i_2);
                                    core::ptr::addr_of_mut!(__slate_compound_48)
                                })
                                .op;
                                core::ptr::addr_of_mut!(__slate_compound_47)
                            })
                            .code as i32;
                            if opcode == 128 || opcode == 9 {
                                let __v20: i32 = _PyInstruction_GetLength(code, i_2);
                                i_2 = i_2.wrapping_add(__v20);
                            } else {
                                remove_per_instruction_tools(
                                    code,
                                    i_2,
                                    removed_per_instruction_tools as i32,
                                );
                                let __v17: i32 = _PyInstruction_GetLength(code, i_2);
                                i_2 = i_2.wrapping_add(__v17);
                            }
                        }
                    }
                    new_line_tools = (*__slate_slot_42).tools[5];
                    new_per_instruction_tools = (*__slate_slot_42).tools[6];
                    if new_line_tools != 0 {
                        line_data = (*(*code)._co_monitoring).lines;
                        i_1 = (*code)._co_firsttraceable;
                        while i_1 < code_len {
                            if get_original_opcode(line_data, i_1) != 0 {
                                add_line_tools(code, i_1, new_line_tools as i32);
                            }
                            let __v10: i32 = _PyInstruction_GetLength(code, i_1);
                            i_1 = i_1.wrapping_add(__v10);
                        }
                    }
                    if new_per_instruction_tools != 0 {
                        i = (*code)._co_firsttraceable;
                        while i < code_len {
                            let opcode: i32 = (*{
                                __slate_compound_45 = (*{
                                    __slate_compound_46 = _Py_GetBaseCodeUnit(code, i);
                                    core::ptr::addr_of_mut!(__slate_compound_46)
                                })
                                .op;
                                core::ptr::addr_of_mut!(__slate_compound_45)
                            })
                            .code as i32;
                            if opcode == 128 || opcode == 9 {
                                let __v5: i32 = _PyInstruction_GetLength(code, i);
                                i = i.wrapping_add(__v5);
                            } else {
                                add_per_instruction_tools(
                                    code,
                                    i,
                                    new_per_instruction_tools as i32,
                                );
                                let __v2: i32 = _PyInstruction_GetLength(code, i);
                                i = i.wrapping_add(__v2);
                            }
                        }
                    }
                }
                FT_ATOMIC_STORE_UINTPTR_RELEASE!(
                    (*code)._co_instrumentation_version,
                    global_version(interp)
                );
                return 0;
            }
        }
    }
}

fn instrument_lock_held(code: *mut PyCodeObject, interp: *mut _is) -> i32 {
    if is_version_up_to_date(code, interp) {
        return 0;
    }
    force_instrument_lock_held(code, interp)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_Instrument(code: *mut PyCodeObject, interp: *mut _is) -> i32 {
    let mut res: i32 = 0;
    res = instrument_lock_held(code, interp);
    res
}

fn instrument_all_executing_code_objects(interp: *mut _is) -> i32 {
    let mut frame: *mut _PyInterpreterFrame = unsafe { core::mem::zeroed() };
    let mut ts: *mut _ts = unsafe { core::mem::zeroed() };
    let mut err: i32 = unsafe { core::mem::zeroed() };
    unsafe {
        err = 0;
        HEAD_LOCK!(unsafe { (*interp).runtime });
        ts = (*interp).threads.head;
        '__loop_1: while !ts.is_null() {
            frame = (*ts).current_frame;
            while !frame.is_null() {
                if ((*frame).owner as i32) < 3 {
                    err = instrument_lock_held(_PyFrame_GetCode(frame), interp);
                    if err != 0 {
                        break '__loop_1;
                    }
                }
                frame = (*frame).previous;
            }
            ts = (*ts).next;
        }
        _Py_FOR_EACH_TSTATE_END!(interp);
        return err;
    }
}

fn set_events(m: *mut _Py_GlobalMonitors, tool_id: i32, events: u32) {
    for e in 0i32..16i32 {
        let tools: *mut u8 =
            unsafe { (*m).tools.as_mut_ptr() as *mut u8 }.wrapping_offset(e as isize);
        let active: i32 = (events >> e & 1) as i32;
        let __v2: u8 = (((unsafe { *tools }) as i32) & !(1i32 << tool_id)) as u8;
        unsafe {
            *tools = __v2;
        }
        unsafe {
            *tools = ((*tools as i32) | active << tool_id) as u8;
        }
    }
}

fn set_local_events(m: *mut _Py_LocalMonitors, tool_id: i32, events: u32) {
    for e in 0i32..11i32 {
        let tools: *mut u8 =
            unsafe { (*m).tools.as_mut_ptr() as *mut u8 }.wrapping_offset(e as isize);
        let val: i32 = (events >> e & 1) as i32;
        let __v2: u8 = (((unsafe { *tools }) as i32) & !(1i32 << tool_id)) as u8;
        unsafe {
            *tools = __v2;
        }
        unsafe {
            *tools = ((*tools as i32) | val << tool_id) as u8;
        }
    }
}

fn check_tool(interp: *mut _is, tool_id: i32) -> i32 {
    if tool_id < 6
        && unsafe {
            *((*interp).monitoring_tool_names.as_mut_ptr() as *mut *mut _object)
                .wrapping_offset(tool_id as isize)
        }
        .is_null()
    {
        unsafe { PyErr_Format(PyExc_ValueError, c"tool %d is not in use".as_ptr(), tool_id) };
        return 1i32.wrapping_neg();
    }
    0
}

// We share the eval-breaker with flags, so the monitoring
// version goes in the top 24 bits
#[unsafe(no_mangle)]
extern "C-unwind" fn _PyMonitoring_SetEvents(tool_id: i32, events: u32) -> i32 {
    let tstate: *mut _ts = _PyThreadState_GET();
    let interp: *mut _is = unsafe { (*tstate).interp };
    if check_tool(interp, tool_id) != 0 {
        return 1i32.wrapping_neg();
    }
    let existing_events: u32 = get_events(
        unsafe { core::ptr::addr_of_mut!((*interp).monitors) },
        tool_id,
    );
    if existing_events == events {
        return 0;
    }
    let new_version: u32 = global_version(interp).wrapping_add(256u32);
    if new_version == 0 {
        unsafe { PyErr_Format(PyExc_OverflowError, c"events set too many times".as_ptr()) };
        return 1i32.wrapping_neg();
    }
    set_events(
        unsafe { core::ptr::addr_of_mut!((*interp).monitors) },
        tool_id,
        events,
    );
    set_global_version(tstate, new_version);
    instrument_all_executing_code_objects(interp)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyMonitoring_SetLocalEvents(
    code: *mut PyCodeObject,
    tool_id: i32,
    events: u32,
) -> i32 {
    let interp: *mut _is = _PyInterpreterState_GET();
    if ((unsafe { (*code)._co_firsttraceable }) as i64) >= Py_SIZE!(code) {
        unsafe {
            PyErr_Format(
                PyExc_SystemError,
                c"cannot instrument shim code object '%U'".as_ptr(),
                (*code).co_name,
            )
        };
        return 1i32.wrapping_neg();
    }
    if check_tool(interp, tool_id) != 0 {
        return 1i32.wrapping_neg();
    }
    if allocate_instrumentation_data(code) != 0 {
        return 1i32.wrapping_neg();
    }
    unsafe {
        *((*(*code)._co_monitoring).tool_versions.as_mut_ptr() as *mut u64)
            .wrapping_offset(tool_id as isize) = *((*interp).monitoring_tool_versions.as_mut_ptr()
            as *mut u64)
            .wrapping_offset(tool_id as isize);
    }
    let local: *mut _Py_LocalMonitors =
        unsafe { core::ptr::addr_of_mut!((*(*code)._co_monitoring).local_monitors) };
    let existing_events: u32 = get_local_events(local, tool_id);
    if existing_events == events {
        return 0;
    }
    set_local_events(local, tool_id, events);
    force_instrument_lock_held(code, interp)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyMonitoring_GetLocalEvents(
    code: *mut PyCodeObject,
    tool_id: i32,
    events: *mut u32,
) -> i32 {
    let interp: *mut _is = _PyInterpreterState_GET();
    if check_tool(interp, tool_id) != 0 {
        return 1i32.wrapping_neg();
    }
    if unsafe { (*code)._co_monitoring }.is_null() {
        unsafe {
            *events = 0;
        }
        return 0;
    }
    let local: *mut _Py_LocalMonitors =
        unsafe { core::ptr::addr_of_mut!((*(*code)._co_monitoring).local_monitors) };
    unsafe {
        *events = get_local_events(local, tool_id);
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyMonitoring_ClearToolId(tool_id: i32) -> i32 {
    let interp: *mut _is = _PyInterpreterState_GET();
    for i in 0i32..19i32 {
        let func: *mut _object =
            _PyMonitoring_RegisterCallback(tool_id, i, core::ptr::null_mut::<_object>());
        if !func.is_null() {
            Py_DECREF!(func);
        }
    }
    unsafe { _PyEval_StopTheWorld(interp) };
    if _PyMonitoring_SetEvents(tool_id, 0u32) < 0 {
        unsafe { _PyEval_StartTheWorld(interp) };
        return 1i32.wrapping_neg();
    }
    let version: u32 = global_version(interp).wrapping_add(256u32);
    if version == 0 {
        unsafe { PyErr_Format(PyExc_OverflowError, c"events set too many times".as_ptr()) };
        unsafe { _PyEval_StartTheWorld(interp) };
        return 1i32.wrapping_neg();
    }
    // monitoring_tool_versions[tool_id] is set to latest global version here to
    //
    //   1. invalidate local events on all existing code objects
    //
    //   2. be ready for the next call to set local events
    unsafe {
        *((*interp).monitoring_tool_versions.as_mut_ptr() as *mut u64)
            .wrapping_offset(tool_id as isize) = version as u64;
    }
    // Set the new global version so all the code objects can refresh the
    //
    // instrumentation.
    set_global_version(_PyThreadState_GET(), version);
    let res: i32 = instrument_all_executing_code_objects(interp);
    unsafe { _PyEval_StartTheWorld(interp) };
    res
}

static monitoring_use_tool_id__doc__: __SlateAlign16<[i8; 44]> = __SlateAlign16([
    117i8, 115i8, 101i8, 95i8, 116i8, 111i8, 111i8, 108i8, 95i8, 105i8, 100i8, 40i8, 36i8, 109i8,
    111i8, 100i8, 117i8, 108i8, 101i8, 44i8, 32i8, 116i8, 111i8, 111i8, 108i8, 95i8, 105i8, 100i8,
    44i8, 32i8, 110i8, 97i8, 109i8, 101i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8,
    0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.instrumentation.monitoring_use_tool_id")]
extern "C-unwind" fn monitoring_use_tool_id(
    module: *mut _object,
    args: *const *mut _object,
    nargs: i64,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut tool_id: i32 = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        if _PyArg_CheckPositional!(c"use_tool_id".as_ptr() as *mut i8, nargs, 2i32, 2i32) {
            tool_id = PyLong_AsInt(*args.wrapping_offset(0 as isize));
            if tool_id == 1i32.wrapping_neg() {
                __v0 = !PyErr_Occurred().is_null();
            } else {
                __v0 = false;
            }
            if !__v0 {
                let name: *mut _object = *args.wrapping_offset(1 as isize);
                return_value = monitoring_use_tool_id_impl(module, tool_id, name);
            }
        }
        return return_value;
    }
}

static monitoring_clear_tool_id__doc__: __SlateAlign16<[i8; 40]> = __SlateAlign16([
    99i8, 108i8, 101i8, 97i8, 114i8, 95i8, 116i8, 111i8, 111i8, 108i8, 95i8, 105i8, 100i8, 40i8,
    36i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 44i8, 32i8, 116i8, 111i8, 111i8, 108i8, 95i8,
    105i8, 100i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.instrumentation.monitoring_clear_tool_id")]
extern "C-unwind" fn monitoring_clear_tool_id(
    module: *mut _object,
    arg: *mut _object,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut tool_id: i32 = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        tool_id = PyLong_AsInt(arg);
        if tool_id == 1i32.wrapping_neg() {
            __v0 = !PyErr_Occurred().is_null();
        } else {
            __v0 = false;
        }
        if !__v0 {
            return_value = monitoring_clear_tool_id_impl(module, tool_id);
        }
        return return_value;
    }
}

static monitoring_free_tool_id__doc__: __SlateAlign16<[i8; 39]> = __SlateAlign16([
    102i8, 114i8, 101i8, 101i8, 95i8, 116i8, 111i8, 111i8, 108i8, 95i8, 105i8, 100i8, 40i8, 36i8,
    109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 44i8, 32i8, 116i8, 111i8, 111i8, 108i8, 95i8, 105i8,
    100i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.instrumentation.monitoring_free_tool_id")]
extern "C-unwind" fn monitoring_free_tool_id(
    module: *mut _object,
    arg: *mut _object,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut tool_id: i32 = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        tool_id = PyLong_AsInt(arg);
        if tool_id == 1i32.wrapping_neg() {
            __v0 = !PyErr_Occurred().is_null();
        } else {
            __v0 = false;
        }
        if !__v0 {
            return_value = monitoring_free_tool_id_impl(module, tool_id);
        }
        return return_value;
    }
}

static monitoring_get_tool__doc__: __SlateAlign16<[i8; 35]> = __SlateAlign16([
    103i8, 101i8, 116i8, 95i8, 116i8, 111i8, 111i8, 108i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8,
    108i8, 101i8, 44i8, 32i8, 116i8, 111i8, 111i8, 108i8, 95i8, 105i8, 100i8, 44i8, 32i8, 47i8,
    41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.instrumentation.monitoring_get_tool")]
extern "C-unwind" fn monitoring_get_tool(module: *mut _object, arg: *mut _object) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut tool_id: i32 = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        tool_id = PyLong_AsInt(arg);
        if tool_id == 1i32.wrapping_neg() {
            __v0 = !PyErr_Occurred().is_null();
        } else {
            __v0 = false;
        }
        if !__v0 {
            return_value = monitoring_get_tool_impl(module, tool_id);
        }
        return return_value;
    }
}

static monitoring_register_callback__doc__: __SlateAlign16<[i8; 57]> = __SlateAlign16([
    114i8, 101i8, 103i8, 105i8, 115i8, 116i8, 101i8, 114i8, 95i8, 99i8, 97i8, 108i8, 108i8, 98i8,
    97i8, 99i8, 107i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 44i8, 32i8, 116i8,
    111i8, 111i8, 108i8, 95i8, 105i8, 100i8, 44i8, 32i8, 101i8, 118i8, 101i8, 110i8, 116i8, 44i8,
    32i8, 102i8, 117i8, 110i8, 99i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.instrumentation.monitoring_register_callback")]
extern "C-unwind" fn monitoring_register_callback(
    module: *mut _object,
    args: *const *mut _object,
    nargs: i64,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut __v1: bool = unsafe { core::mem::zeroed() };
    let mut event: i32 = unsafe { core::mem::zeroed() };
    let mut tool_id: i32 = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        if _PyArg_CheckPositional!(c"register_callback".as_ptr() as *mut i8, nargs, 3i32, 3i32) {
            tool_id = PyLong_AsInt(*args.wrapping_offset(0 as isize));
            if tool_id == 1i32.wrapping_neg() {
                __v1 = !PyErr_Occurred().is_null();
            } else {
                __v1 = false;
            }
            if !__v1 {
                event = PyLong_AsInt(*args.wrapping_offset(1 as isize));
                if event == 1i32.wrapping_neg() {
                    __v0 = !PyErr_Occurred().is_null();
                } else {
                    __v0 = false;
                }
                if !__v0 {
                    let func: *mut _object = *args.wrapping_offset(2 as isize);
                    return_value = monitoring_register_callback_impl(module, tool_id, event, func);
                }
            }
        }
        return return_value;
    }
}

static monitoring_get_events__doc__: __SlateAlign16<[i8; 37]> = __SlateAlign16([
    103i8, 101i8, 116i8, 95i8, 101i8, 118i8, 101i8, 110i8, 116i8, 115i8, 40i8, 36i8, 109i8, 111i8,
    100i8, 117i8, 108i8, 101i8, 44i8, 32i8, 116i8, 111i8, 111i8, 108i8, 95i8, 105i8, 100i8, 44i8,
    32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.instrumentation.monitoring_get_events")]
extern "C-unwind" fn monitoring_get_events(
    module: *mut _object,
    arg: *mut _object,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut __v1: bool = unsafe { core::mem::zeroed() };
    let mut _return_value: i32 = unsafe { core::mem::zeroed() };
    let mut tool_id: i32 = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        tool_id = PyLong_AsInt(arg);
        if tool_id == 1i32.wrapping_neg() {
            __v1 = !PyErr_Occurred().is_null();
        } else {
            __v1 = false;
        }
        if !__v1 {
            _return_value = monitoring_get_events_impl(module, tool_id);
            if _return_value == 1i32.wrapping_neg() {
                __v0 = !PyErr_Occurred().is_null();
            } else {
                __v0 = false;
            }
            if !__v0 {
                return_value = PyLong_FromLong(_return_value as i64);
            }
        }
        return return_value;
    }
}

static monitoring_set_events__doc__: __SlateAlign16<[i8; 48]> = __SlateAlign16([
    115i8, 101i8, 116i8, 95i8, 101i8, 118i8, 101i8, 110i8, 116i8, 115i8, 40i8, 36i8, 109i8, 111i8,
    100i8, 117i8, 108i8, 101i8, 44i8, 32i8, 116i8, 111i8, 111i8, 108i8, 95i8, 105i8, 100i8, 44i8,
    32i8, 101i8, 118i8, 101i8, 110i8, 116i8, 95i8, 115i8, 101i8, 116i8, 44i8, 32i8, 47i8, 41i8,
    10i8, 45i8, 45i8, 10i8, 10i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.instrumentation.monitoring_set_events")]
extern "C-unwind" fn monitoring_set_events(
    module: *mut _object,
    args: *const *mut _object,
    nargs: i64,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut __v1: bool = unsafe { core::mem::zeroed() };
    let mut event_set: i32 = unsafe { core::mem::zeroed() };
    let mut tool_id: i32 = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        if _PyArg_CheckPositional!(c"set_events".as_ptr() as *mut i8, nargs, 2i32, 2i32) {
            tool_id = PyLong_AsInt(*args.wrapping_offset(0 as isize));
            if tool_id == 1i32.wrapping_neg() {
                __v1 = !PyErr_Occurred().is_null();
            } else {
                __v1 = false;
            }
            if !__v1 {
                event_set = PyLong_AsInt(*args.wrapping_offset(1 as isize));
                if event_set == 1i32.wrapping_neg() {
                    __v0 = !PyErr_Occurred().is_null();
                } else {
                    __v0 = false;
                }
                if !__v0 {
                    return_value = monitoring_set_events_impl(module, tool_id, event_set);
                }
            }
        }
        return return_value;
    }
}

static monitoring_get_local_events__doc__: __SlateAlign16<[i8; 49]> = __SlateAlign16([
    103i8, 101i8, 116i8, 95i8, 108i8, 111i8, 99i8, 97i8, 108i8, 95i8, 101i8, 118i8, 101i8, 110i8,
    116i8, 115i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 44i8, 32i8, 116i8, 111i8,
    111i8, 108i8, 95i8, 105i8, 100i8, 44i8, 32i8, 99i8, 111i8, 100i8, 101i8, 44i8, 32i8, 47i8,
    41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.instrumentation.monitoring_get_local_events")]
extern "C-unwind" fn monitoring_get_local_events(
    module: *mut _object,
    args: *const *mut _object,
    nargs: i64,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut __v1: bool = unsafe { core::mem::zeroed() };
    let mut _return_value: i32 = unsafe { core::mem::zeroed() };
    let mut tool_id: i32 = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        if _PyArg_CheckPositional!(c"get_local_events".as_ptr() as *mut i8, nargs, 2i32, 2i32) {
            tool_id = PyLong_AsInt(*args.wrapping_offset(0 as isize));
            if tool_id == 1i32.wrapping_neg() {
                __v1 = !PyErr_Occurred().is_null();
            } else {
                __v1 = false;
            }
            if !__v1 {
                let code: *mut _object = *args.wrapping_offset(1 as isize);
                _return_value = monitoring_get_local_events_impl(module, tool_id, code);
                if _return_value == 1i32.wrapping_neg() {
                    __v0 = !PyErr_Occurred().is_null();
                } else {
                    __v0 = false;
                }
                if !__v0 {
                    return_value = PyLong_FromLong(_return_value as i64);
                }
            }
        }
        return return_value;
    }
}

static monitoring_set_local_events__doc__: __SlateAlign16<[i8; 60]> = __SlateAlign16([
    115i8, 101i8, 116i8, 95i8, 108i8, 111i8, 99i8, 97i8, 108i8, 95i8, 101i8, 118i8, 101i8, 110i8,
    116i8, 115i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 44i8, 32i8, 116i8, 111i8,
    111i8, 108i8, 95i8, 105i8, 100i8, 44i8, 32i8, 99i8, 111i8, 100i8, 101i8, 44i8, 32i8, 101i8,
    118i8, 101i8, 110i8, 116i8, 95i8, 115i8, 101i8, 116i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8,
    45i8, 10i8, 10i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.instrumentation.monitoring_set_local_events")]
extern "C-unwind" fn monitoring_set_local_events(
    module: *mut _object,
    args: *const *mut _object,
    nargs: i64,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut __v1: bool = unsafe { core::mem::zeroed() };
    let mut event_set: i32 = unsafe { core::mem::zeroed() };
    let mut code: *mut _object = unsafe { core::mem::zeroed() };
    let mut tool_id: i32 = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        if _PyArg_CheckPositional!(c"set_local_events".as_ptr() as *mut i8, nargs, 3i32, 3i32) {
            tool_id = PyLong_AsInt(*args.wrapping_offset(0 as isize));
            if tool_id == 1i32.wrapping_neg() {
                __v1 = !PyErr_Occurred().is_null();
            } else {
                __v1 = false;
            }
            if !__v1 {
                code = *args.wrapping_offset(1 as isize);
                event_set = PyLong_AsInt(*args.wrapping_offset(2 as isize));
                if event_set == 1i32.wrapping_neg() {
                    __v0 = !PyErr_Occurred().is_null();
                } else {
                    __v0 = false;
                }
                if !__v0 {
                    return_value =
                        monitoring_set_local_events_impl(module, tool_id, code, event_set);
                }
            }
        }
        return return_value;
    }
}

static monitoring_restart_events__doc__: __SlateAlign16<[i8; 32]> = __SlateAlign16([
    114i8, 101i8, 115i8, 116i8, 97i8, 114i8, 116i8, 95i8, 101i8, 118i8, 101i8, 110i8, 116i8, 115i8,
    40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8,
    10i8, 10i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.instrumentation.monitoring_restart_events")]
extern "C-unwind" fn monitoring_restart_events(
    module: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    monitoring_restart_events_impl(module)
}

static monitoring__all_events__doc__: __SlateAlign16<[i8; 29]> = __SlateAlign16([
    95i8, 97i8, 108i8, 108i8, 95i8, 101i8, 118i8, 101i8, 110i8, 116i8, 115i8, 40i8, 36i8, 109i8,
    111i8, 100i8, 117i8, 108i8, 101i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.instrumentation.monitoring__all_events")]
extern "C-unwind" fn monitoring__all_events(
    module: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    monitoring__all_events_impl(module)
}

// [clinic input]
// module monitoring
// [clinic start generated code]
//
// [clinic end generated code: output=da39a3ee5e6b4b0d input=37257f5987a360cf]
//
// [clinic end generated code]
fn check_valid_tool(tool_id: i32) -> i32 {
    if tool_id < 0 || tool_id >= 6 {
        unsafe {
            PyErr_Format(
                PyExc_ValueError,
                c"invalid tool %d (must be between 0 and 5)".as_ptr(),
                tool_id,
            )
        };
        return 1i32.wrapping_neg();
    }
    0
}

// [clinic input]
// monitoring.use_tool_id
//
//     tool_id: int
//     name: object
//     /
//
// [clinic start generated code]
/// [clinic end generated code: output=30d76dc92b7cd653 input=ebc453761c621be1]
fn monitoring_use_tool_id_impl(
    module: *mut _object,
    tool_id: i32,
    name: *mut _object,
) -> *mut _object {
    if check_valid_tool(tool_id) != 0 {
        return core::ptr::null_mut::<_object>();
    }
    if PyUnicode_Check!(name) == 0 {
        unsafe { PyErr_SetString(PyExc_ValueError, c"tool name must be a str".as_ptr()) };
        return core::ptr::null_mut::<_object>();
    }
    let interp: *mut _is = _PyInterpreterState_GET();
    if !unsafe {
        *((*interp).monitoring_tool_names.as_mut_ptr() as *mut *mut _object)
            .wrapping_offset(tool_id as isize)
    }
    .is_null()
    {
        unsafe {
            PyErr_Format(
                PyExc_ValueError,
                c"tool %d is already in use".as_ptr(),
                tool_id,
            )
        };
        return core::ptr::null_mut::<_object>();
    }
    unsafe {
        *((*interp).monitoring_tool_names.as_mut_ptr() as *mut *mut _object)
            .wrapping_offset(tool_id as isize) = Py_NewRef!(name);
    }
    unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
}

// [clinic input]
// monitoring.clear_tool_id
//
//     tool_id: int
//     /
//
// [clinic start generated code]
/// [clinic end generated code: output=04defc23470b1be7 input=af643d6648a66163]
fn monitoring_clear_tool_id_impl(module: *mut _object, tool_id: i32) -> *mut _object {
    if check_valid_tool(tool_id) != 0 {
        return core::ptr::null_mut::<_object>();
    }
    let interp: *mut _is = _PyInterpreterState_GET();
    if !unsafe {
        *((*interp).monitoring_tool_names.as_mut_ptr() as *mut *mut _object)
            .wrapping_offset(tool_id as isize)
    }
    .is_null()
    {
        if _PyMonitoring_ClearToolId(tool_id) < 0 {
            return core::ptr::null_mut::<_object>();
        }
    }
    unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
}

// [clinic input]
// monitoring.free_tool_id
//
//     tool_id: int
//     /
//
// [clinic start generated code]
/// [clinic end generated code: output=86c2d2a1219a8591 input=a23fb6be3a8618e9]
fn monitoring_free_tool_id_impl(module: *mut _object, tool_id: i32) -> *mut _object {
    if check_valid_tool(tool_id) != 0 {
        return core::ptr::null_mut::<_object>();
    }
    let interp: *mut _is = _PyInterpreterState_GET();
    if !unsafe {
        *((*interp).monitoring_tool_names.as_mut_ptr() as *mut *mut _object)
            .wrapping_offset(tool_id as isize)
    }
    .is_null()
    {
        if _PyMonitoring_ClearToolId(tool_id) < 0 {
            return core::ptr::null_mut::<_object>();
        }
    }
    Py_CLEAR!(
        *unsafe { (*interp).monitoring_tool_names.as_mut_ptr() as *mut *mut _object }
            .wrapping_offset(tool_id as isize)
    );
    unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
}

// [clinic input]
// monitoring.get_tool
//
//     tool_id: int
//     /
//
// [clinic start generated code]
/// [clinic end generated code: output=1c05a98b404a9a16 input=eeee9bebd0bcae9d]
/// [clinic end generated code]
fn monitoring_get_tool_impl(module: *mut _object, tool_id: i32) -> *mut _object {
    if check_valid_tool(tool_id) != 0 {
        return core::ptr::null_mut::<_object>();
    }
    let interp: *mut _is = _PyInterpreterState_GET();
    let name: *mut _object = unsafe {
        *((*interp).monitoring_tool_names.as_mut_ptr() as *mut *mut _object)
            .wrapping_offset(tool_id as isize)
    };
    if name.is_null() {
        return unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) };
    }
    Py_NewRef!(name)
}

// [clinic input]
// monitoring.register_callback
//
//
//     tool_id: int
//     event: int
//     func: object
//     /
//
// [clinic start generated code]
/// [clinic end generated code: output=e64daa363004030c input=df6d70ea4cf81007]
fn monitoring_register_callback_impl(
    module: *mut _object,
    tool_id: i32,
    event: i32,
    mut func: *mut _object,
) -> *mut _object {
    if check_valid_tool(tool_id) != 0 {
        return core::ptr::null_mut::<_object>();
    }
    if _Py_popcount32(event as u32) != 1 {
        unsafe {
            PyErr_SetString(
                PyExc_ValueError,
                c"The callback can only be set for one event at a time".as_ptr(),
            )
        };
        return core::ptr::null_mut::<_object>();
    }
    let event_id: i32 = _Py_bit_length(event as u64).wrapping_sub(1);
    if event_id < 0 || event_id >= 19 {
        unsafe { PyErr_Format(PyExc_ValueError, c"invalid event %d".as_ptr(), event) };
        return core::ptr::null_mut::<_object>();
    }
    if (unsafe {
        PySys_Audit(
            c"sys.monitoring.register_callback".as_ptr(),
            c"O".as_ptr(),
            func,
        )
    }) < 0
    {
        return core::ptr::null_mut::<_object>();
    }
    if func == unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } {
        func = core::ptr::null_mut::<_object>();
    }
    func = _PyMonitoring_RegisterCallback(tool_id, event_id, func);
    if func.is_null() {
        return unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) };
    }
    func
}

// [clinic input]
// monitoring.get_events -> int
//
//     tool_id: int
//     /
//
// [clinic start generated code]
/// [clinic end generated code: output=4450cc13f826c8c0 input=a64b238f76c4b2f7]
fn monitoring_get_events_impl(module: *mut _object, tool_id: i32) -> i32 {
    if check_valid_tool(tool_id) != 0 {
        return 1i32.wrapping_neg();
    }
    let m: *mut _Py_GlobalMonitors =
        unsafe { core::ptr::addr_of_mut!((*_PyInterpreterState_GET()).monitors) };
    let event_set: u32 = get_events(m, tool_id);
    event_set as i32
}

// [clinic input]
// monitoring.set_events
//
//     tool_id: int
//     event_set: int
//     /
//
// [clinic start generated code]
/// [clinic end generated code: output=1916c1e49cfb5bdb input=a77ba729a242142b]
fn monitoring_set_events_impl(
    module: *mut _object,
    tool_id: i32,
    mut event_set: i32,
) -> *mut _object {
    if check_valid_tool(tool_id) != 0 {
        return core::ptr::null_mut::<_object>();
    }
    if event_set < 0 || event_set >= 524288i32 {
        unsafe {
            PyErr_Format(
                PyExc_ValueError,
                c"invalid event set 0x%x".as_ptr(),
                event_set,
            )
        };
        return core::ptr::null_mut::<_object>();
    }
    if event_set & 196608i32 != 0 && event_set & 196624i32 != 196624i32 {
        unsafe {
            PyErr_Format(
                PyExc_ValueError,
                c"cannot set C_RETURN or C_RAISE events independently".as_ptr(),
            )
        };
        return core::ptr::null_mut::<_object>();
    }
    let __v1: i32 = event_set & -196609i32;
    event_set = __v1;
    if event_set & 262144i32 != 0 {
        let __v3: i32 = event_set & -262145i32;
        event_set = __v3;
        event_set |= 768i32;
    }
    let interp: *mut _is = _PyInterpreterState_GET();
    unsafe { _PyEval_StopTheWorld(interp) };
    let err: i32 = _PyMonitoring_SetEvents(tool_id, event_set as u32);
    unsafe { _PyEval_StartTheWorld(interp) };
    if err != 0 {
        return core::ptr::null_mut::<_object>();
    }
    unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
}

// [clinic input]
// monitoring.get_local_events -> int
//
//     tool_id: int
//     code: object
//     /
//
// [clinic start generated code]
/// [clinic end generated code: output=d3e92c1c9c1de8f9 input=bb0f927530386a94]
fn monitoring_get_local_events_impl(module: *mut _object, tool_id: i32, code: *mut _object) -> i32 {
    if PyCode_Check!(code) == 0 {
        unsafe { PyErr_Format(PyExc_TypeError, c"code must be a code object".as_ptr()) };
        return 1i32.wrapping_neg();
    }
    if check_valid_tool(tool_id) != 0 {
        return 1i32.wrapping_neg();
    }
    let mut event_set: u32 = 0;
    let data: *mut _PyCoMonitoringData = unsafe { (*(code as *mut PyCodeObject))._co_monitoring };
    if !data.is_null() {
        for e in 0i32..11i32 {
            if ((unsafe {
                *((*data).local_monitors.tools.as_mut_ptr() as *mut u8).wrapping_offset(e as isize)
            }) as i32)
                >> tool_id
                & 1
                != 0
            {
                event_set |= (1i32 << e) as u32;
            }
        }
    }
    event_set as i32
}

// [clinic input]
// monitoring.set_local_events
//
//     tool_id: int
//     code: object
//     event_set: int
//     /
//
// [clinic start generated code]
/// [clinic end generated code: output=68cc755a65dfea99 input=5655ecd78d937a29]
fn monitoring_set_local_events_impl(
    module: *mut _object,
    tool_id: i32,
    code: *mut _object,
    mut event_set: i32,
) -> *mut _object {
    if PyCode_Check!(code) == 0 {
        unsafe { PyErr_Format(PyExc_TypeError, c"code must be a code object".as_ptr()) };
        return core::ptr::null_mut::<_object>();
    }
    if check_valid_tool(tool_id) != 0 {
        return core::ptr::null_mut::<_object>();
    }
    if event_set & 196608i32 != 0 && event_set & 196624i32 != 196624i32 {
        unsafe {
            PyErr_Format(
                PyExc_ValueError,
                c"cannot set C_RETURN or C_RAISE events independently".as_ptr(),
            )
        };
        return core::ptr::null_mut::<_object>();
    }
    let __v1: i32 = event_set & -196609i32;
    event_set = __v1;
    if event_set & 262144i32 != 0 {
        let __v3: i32 = event_set & -262145i32;
        event_set = __v3;
        event_set |= 768i32;
    }
    if event_set < 0 || event_set >= 2048i32 {
        unsafe {
            PyErr_Format(
                PyExc_ValueError,
                c"invalid local event set 0x%x".as_ptr(),
                event_set,
            )
        };
        return core::ptr::null_mut::<_object>();
    }
    let interp: *mut _is = _PyInterpreterState_GET();
    unsafe { _PyEval_StopTheWorld(interp) };
    let err: i32 =
        _PyMonitoring_SetLocalEvents(code as *mut PyCodeObject, tool_id, event_set as u32);
    unsafe { _PyEval_StartTheWorld(interp) };
    if err != 0 {
        return core::ptr::null_mut::<_object>();
    }
    unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
}

// [clinic input]
// monitoring.restart_events
//
// [clinic start generated code]
/// [clinic end generated code: output=e025dd5ba33314c4 input=add8a855063c8008]
fn monitoring_restart_events_impl(module: *mut _object) -> *mut _object {
    // We want to ensure that:
    // last restart version > instrumented version for all code objects
    // last restart version < current version
    let tstate: *mut _ts = _PyThreadState_GET();
    let interp: *mut _is = unsafe { (*tstate).interp };
    unsafe { _PyEval_StopTheWorld(interp) };
    let restart_version: u32 = global_version(interp).wrapping_add(256u32);
    let new_version: u32 = restart_version.wrapping_add(256u32);
    if new_version <= 256u32 {
        unsafe { _PyEval_StartTheWorld(interp) };
        unsafe { PyErr_Format(PyExc_OverflowError, c"events set too many times".as_ptr()) };
        return core::ptr::null_mut::<_object>();
    }
    unsafe {
        (*interp).last_restart_version = restart_version as u64;
    }
    set_global_version(tstate, new_version);
    let res: i32 = instrument_all_executing_code_objects(interp);
    unsafe { _PyEval_StartTheWorld(interp) };
    if res != 0 {
        return core::ptr::null_mut::<_object>();
    }
    unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
}

fn add_power2_constant(obj: *mut _object, name: *const i8, i: i32) -> i32 {
    let val: *mut _object = unsafe { PyLong_FromLong((1i32 << i) as i64) };
    if val.is_null() {
        return 1i32.wrapping_neg();
    }
    let err: i32 = unsafe { PyObject_SetAttrString(obj, name, val) };
    Py_DECREF!(val);
    err
}

// [clinic input]
// monitoring._all_events
// [clinic start generated code]
/// [clinic end generated code: output=6b7581e2dbb690f6 input=62ee9672c17b7f0e]
fn monitoring__all_events_impl(module: *mut _object) -> *mut _object {
    let interp: *mut _is = _PyInterpreterState_GET();
    let res: *mut _object = unsafe { PyDict_New() };
    if res.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    for e in 0i32..16i32 {
        let tools: u8 = unsafe {
            *((*interp).monitors.tools.as_mut_ptr() as *mut u8).wrapping_offset(e as isize)
        };
        if (tools as i32) == 0 {
            continue;
        }
        let tools_obj: *mut _object = unsafe { PyLong_FromLong(tools as i64) };
        let err: i32 = unsafe { PyDict_SetItemString(res, event_names.0[e as usize], tools_obj) };
        Py_DECREF!(tools_obj);
        if err < 0 {
            Py_DECREF!(res);
            return core::ptr::null_mut::<_object>();
        }
    }
    res
}

/// sentinel
static mut methods: __SlateAlign16<[PyMethodDef; 12]> = __SlateAlign16([
    {
        let mut __t0: PyMethodDef = unsafe { core::mem::zeroed() };
        __t0.ml_name = (b"use_tool_id\0".as_ptr() as *mut i8) as *const i8;
        __t0.ml_meth = unsafe {
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
                >(Some(monitoring_use_tool_id))
            })
        };
        __t0.ml_flags = 128i32;
        __t0.ml_doc = unsafe { core::ptr::addr_of!(monitoring_use_tool_id__doc__.0) as *const i8 };
        __t0
    },
    {
        let mut __t1: PyMethodDef = unsafe { core::mem::zeroed() };
        __t1.ml_name = (b"clear_tool_id\0".as_ptr() as *mut i8) as *const i8;
        __t1.ml_meth = Some(monitoring_clear_tool_id);
        __t1.ml_flags = 8i32;
        __t1.ml_doc =
            unsafe { core::ptr::addr_of!(monitoring_clear_tool_id__doc__.0) as *const i8 };
        __t1
    },
    {
        let mut __t2: PyMethodDef = unsafe { core::mem::zeroed() };
        __t2.ml_name = (b"free_tool_id\0".as_ptr() as *mut i8) as *const i8;
        __t2.ml_meth = Some(monitoring_free_tool_id);
        __t2.ml_flags = 8i32;
        __t2.ml_doc = unsafe { core::ptr::addr_of!(monitoring_free_tool_id__doc__.0) as *const i8 };
        __t2
    },
    {
        let mut __t3: PyMethodDef = unsafe { core::mem::zeroed() };
        __t3.ml_name = (b"get_tool\0".as_ptr() as *mut i8) as *const i8;
        __t3.ml_meth = Some(monitoring_get_tool);
        __t3.ml_flags = 8i32;
        __t3.ml_doc = unsafe { core::ptr::addr_of!(monitoring_get_tool__doc__.0) as *const i8 };
        __t3
    },
    {
        let mut __t4: PyMethodDef = unsafe { core::mem::zeroed() };
        __t4.ml_name = (b"register_callback\0".as_ptr() as *mut i8) as *const i8;
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
                >(Some(monitoring_register_callback))
            })
        };
        __t4.ml_flags = 128i32;
        __t4.ml_doc =
            unsafe { core::ptr::addr_of!(monitoring_register_callback__doc__.0) as *const i8 };
        __t4
    },
    {
        let mut __t5: PyMethodDef = unsafe { core::mem::zeroed() };
        __t5.ml_name = (b"get_events\0".as_ptr() as *mut i8) as *const i8;
        __t5.ml_meth = Some(monitoring_get_events);
        __t5.ml_flags = 8i32;
        __t5.ml_doc = unsafe { core::ptr::addr_of!(monitoring_get_events__doc__.0) as *const i8 };
        __t5
    },
    {
        let mut __t6: PyMethodDef = unsafe { core::mem::zeroed() };
        __t6.ml_name = (b"set_events\0".as_ptr() as *mut i8) as *const i8;
        __t6.ml_meth = unsafe {
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
                >(Some(monitoring_set_events))
            })
        };
        __t6.ml_flags = 128i32;
        __t6.ml_doc = unsafe { core::ptr::addr_of!(monitoring_set_events__doc__.0) as *const i8 };
        __t6
    },
    {
        let mut __t7: PyMethodDef = unsafe { core::mem::zeroed() };
        __t7.ml_name = (b"get_local_events\0".as_ptr() as *mut i8) as *const i8;
        __t7.ml_meth = unsafe {
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
                >(Some(monitoring_get_local_events))
            })
        };
        __t7.ml_flags = 128i32;
        __t7.ml_doc =
            unsafe { core::ptr::addr_of!(monitoring_get_local_events__doc__.0) as *const i8 };
        __t7
    },
    {
        let mut __t8: PyMethodDef = unsafe { core::mem::zeroed() };
        __t8.ml_name = (b"set_local_events\0".as_ptr() as *mut i8) as *const i8;
        __t8.ml_meth = unsafe {
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
                >(Some(monitoring_set_local_events))
            })
        };
        __t8.ml_flags = 128i32;
        __t8.ml_doc =
            unsafe { core::ptr::addr_of!(monitoring_set_local_events__doc__.0) as *const i8 };
        __t8
    },
    {
        let mut __t9: PyMethodDef = unsafe { core::mem::zeroed() };
        __t9.ml_name = (b"restart_events\0".as_ptr() as *mut i8) as *const i8;
        __t9.ml_meth = Some(monitoring_restart_events);
        __t9.ml_flags = 4i32;
        __t9.ml_doc =
            unsafe { core::ptr::addr_of!(monitoring_restart_events__doc__.0) as *const i8 };
        __t9
    },
    {
        let mut __t10: PyMethodDef = unsafe { core::mem::zeroed() };
        __t10.ml_name = (b"_all_events\0".as_ptr() as *mut i8) as *const i8;
        __t10.ml_meth = Some(monitoring__all_events);
        __t10.ml_flags = 4i32;
        __t10.ml_doc = unsafe { core::ptr::addr_of!(monitoring__all_events__doc__.0) as *const i8 };
        __t10
    },
    {
        let mut __t11: PyMethodDef = unsafe { core::mem::zeroed() };
        __t11.ml_name = core::ptr::null::<i8>();
        __t11.ml_meth = None;
        __t11
    },
]);

/// multiple "initialization" just copies the module dict.
static mut monitoring_module: PyModuleDef = {
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
    __t0.m_name = (b"sys.monitoring\0".as_ptr() as *mut i8) as *const i8;
    __t0.m_size = 1i32.wrapping_neg() as i64;
    __t0.m_methods = unsafe { core::ptr::addr_of_mut!(methods.0) as *mut PyMethodDef };
    __t0
};

#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_CreateMonitoringObject() -> *mut _object {
    let mut val: *mut _object = unsafe { core::mem::zeroed() };
    let mut i: i32 = unsafe { core::mem::zeroed() };
    let mut err: i32 = unsafe { core::mem::zeroed() };
    let mut events: *mut _object = unsafe { core::mem::zeroed() };
    let mut r#mod: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        r#mod = _PyModule_CreateInitialized(core::ptr::addr_of_mut!(monitoring_module), 1013i32);
        if r#mod.is_null() {
            return core::ptr::null_mut::<_object>();
        } else {
            '__join_0: {
                if PyObject_SetAttrString(
                    r#mod,
                    c"DISABLE".as_ptr(),
                    core::ptr::addr_of_mut!(_PyInstrumentation_DISABLE),
                ) == 0
                {
                    if PyObject_SetAttrString(
                        r#mod,
                        c"MISSING".as_ptr(),
                        core::ptr::addr_of_mut!(_PyInstrumentation_MISSING),
                    ) == 0
                    {
                        events = _PyNamespace_New(core::ptr::null_mut::<_object>());
                        if !events.is_null() {
                            err = PyObject_SetAttrString(r#mod, c"events".as_ptr(), events);
                            Py_DECREF!(events);
                            if err == 0 {
                                i = 0;
                                while i < 19 {
                                    if add_power2_constant(events, event_names.0[i as usize], i)
                                        != 0
                                    {
                                        break '__join_0;
                                    } else {
                                        i = i.wrapping_add(1);
                                    }
                                }
                                err = PyObject_SetAttrString(
                                    events,
                                    c"NO_EVENTS".as_ptr(),
                                    _PyLong_GetZero(),
                                );
                                if err == 0 {
                                    val = PyLong_FromLong(0i64);
                                    // Can't return NULL because the int is small.
                                    err =
                                        PyObject_SetAttrString(r#mod, c"DEBUGGER_ID".as_ptr(), val);
                                    Py_DECREF!(val);
                                    if err == 0 {
                                        val = PyLong_FromLong(1i64);
                                        err = PyObject_SetAttrString(
                                            r#mod,
                                            c"COVERAGE_ID".as_ptr(),
                                            val,
                                        );
                                        Py_DECREF!(val);
                                        if err == 0 {
                                            val = PyLong_FromLong(2i64);
                                            err = PyObject_SetAttrString(
                                                r#mod,
                                                c"PROFILER_ID".as_ptr(),
                                                val,
                                            );
                                            Py_DECREF!(val);
                                            if err == 0 {
                                                val = PyLong_FromLong(5i64);
                                                err = PyObject_SetAttrString(
                                                    r#mod,
                                                    c"OPTIMIZER_ID".as_ptr(),
                                                    val,
                                                );
                                                Py_DECREF!(val);
                                                if err == 0 {
                                                    return r#mod;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Py_DECREF!(r#mod);
            return core::ptr::null_mut::<_object>();
        }
    }
}

fn capi_call_instrumentation(
    state: *mut _PyMonitoringState,
    codelike: *mut _object,
    offset: i32,
    args: *mut *mut _object,
    nargs: i64,
    event: i32,
) -> i32 {
    let tstate: *mut _ts = _PyThreadState_GET();
    let interp: *mut _is = unsafe { (*tstate).interp };
    let mut tools: u8 = unsafe { (*state).active };
    unsafe {
        *args.wrapping_offset(1 as isize) = codelike;
    }
    if offset < 0 {
        unsafe { PyErr_SetString(PyExc_ValueError, c"offset must be non-negative".as_ptr()) };
        return 1i32.wrapping_neg();
    }
    if event != 5 {
        let offset_obj: *mut _object = unsafe { PyLong_FromLong(offset as i64) };
        if offset_obj.is_null() {
            return 1i32.wrapping_neg();
        }
        unsafe {
            *args.wrapping_offset(2 as isize) = offset_obj;
        }
    }
    let nargsf: u64 = (nargs as u64)
        | _Py_STATIC_CAST!(u64, 1i32)
            << 8u64
                .wrapping_mul(core::mem::size_of::<u64>() as u64)
                .wrapping_sub(1);
    let callargs: *mut *mut _object = args.wrapping_offset(1 as isize);
    let mut err: i32 = 0;
    while tools != 0 {
        let tool: i32 = most_significant_bit(tools);
        tools = ((tools as i32) ^ 1i32 << tool) as u8;
        let res: i32 = call_one_instrument(interp, tstate, callargs, nargsf, tool as i8, event);
        if res == 0 {
            // Nothing to do
        } else {
            if res < 0 {
                // error
                err = 1i32.wrapping_neg();
                break;
            } else {
                // DISABLE
                if !PY_MONITORING_IS_INSTRUMENTED_EVENT!(event) {
                    unsafe {
                        PyErr_Format(
                            PyExc_ValueError,
                            c"Cannot disable %s events. Callback removed.".as_ptr(),
                            event_names.0[event as usize],
                        )
                    };
                    // Clear tool to prevent infinite loop
                    Py_CLEAR!(
                        *unsafe {
                            (*((*interp).monitoring_callables.as_mut_ptr()
                                as *mut [*mut _object; 19])
                                .wrapping_offset(tool as isize))
                            .as_mut_ptr() as *mut *mut _object
                        }
                        .wrapping_offset(event as isize)
                    );
                    err = 1i32.wrapping_neg();
                    break;
                } else {
                    let __v6: u8 = (((unsafe { (*state).active }) as i32) & !(1i32 << tool)) as u8;
                    unsafe {
                        (*state).active = __v6;
                    }
                }
            }
        }
    }
    err
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyMonitoring_EnterScope(
    state_array: *mut _PyMonitoringState,
    version: *mut u64,
    event_types: *const u8,
    length: i64,
) -> i32 {
    let interp: *mut _is = _PyInterpreterState_GET();
    if (global_version(interp) as u64) == unsafe { *version } {
        return 0;
    }
    let m: *mut _Py_GlobalMonitors = unsafe { core::ptr::addr_of_mut!((*interp).monitors) };
    for i in 0..length {
        let event: i32 = (unsafe { *event_types.wrapping_offset(i as isize) }) as i32;
        unsafe {
            (*state_array.wrapping_offset(i as isize)).active =
                *((*m).tools.as_mut_ptr() as *mut u8).wrapping_offset(event as isize);
        }
    }
    unsafe {
        *version = global_version(interp) as u64;
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyMonitoring_ExitScope() -> i32 {
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyMonitoring_FirePyStartEvent(
    state: *mut _PyMonitoringState,
    codelike: *mut _object,
    offset: i32,
) -> i32 {
    let mut args: __SlateAlign16<[*mut _object; 3]> = __SlateAlign16([
        core::ptr::null_mut::<_object>(),
        core::ptr::null_mut::<_object>(),
        core::ptr::null_mut::<_object>(),
    ]);
    capi_call_instrumentation(
        state,
        codelike,
        offset,
        args.0.as_mut_ptr() as *mut *mut _object,
        2i64,
        0i32,
    )
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyMonitoring_FirePyResumeEvent(
    state: *mut _PyMonitoringState,
    codelike: *mut _object,
    offset: i32,
) -> i32 {
    let mut args: __SlateAlign16<[*mut _object; 3]> = __SlateAlign16([
        core::ptr::null_mut::<_object>(),
        core::ptr::null_mut::<_object>(),
        core::ptr::null_mut::<_object>(),
    ]);
    capi_call_instrumentation(
        state,
        codelike,
        offset,
        args.0.as_mut_ptr() as *mut *mut _object,
        2i64,
        1i32,
    )
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyMonitoring_FirePyReturnEvent(
    state: *mut _PyMonitoringState,
    codelike: *mut _object,
    offset: i32,
    retval: *mut _object,
) -> i32 {
    let mut args: __SlateAlign16<[*mut _object; 4]> = __SlateAlign16([
        core::ptr::null_mut::<_object>(),
        core::ptr::null_mut::<_object>(),
        core::ptr::null_mut::<_object>(),
        retval,
    ]);
    capi_call_instrumentation(
        state,
        codelike,
        offset,
        args.0.as_mut_ptr() as *mut *mut _object,
        3i64,
        2i32,
    )
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyMonitoring_FirePyYieldEvent(
    state: *mut _PyMonitoringState,
    codelike: *mut _object,
    offset: i32,
    retval: *mut _object,
) -> i32 {
    let mut args: __SlateAlign16<[*mut _object; 4]> = __SlateAlign16([
        core::ptr::null_mut::<_object>(),
        core::ptr::null_mut::<_object>(),
        core::ptr::null_mut::<_object>(),
        retval,
    ]);
    capi_call_instrumentation(
        state,
        codelike,
        offset,
        args.0.as_mut_ptr() as *mut *mut _object,
        3i64,
        3i32,
    )
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyMonitoring_FireCallEvent(
    state: *mut _PyMonitoringState,
    codelike: *mut _object,
    offset: i32,
    callable: *mut _object,
    arg0: *mut _object,
) -> i32 {
    let mut args: __SlateAlign16<[*mut _object; 5]> = __SlateAlign16([
        core::ptr::null_mut::<_object>(),
        core::ptr::null_mut::<_object>(),
        core::ptr::null_mut::<_object>(),
        callable,
        arg0,
    ]);
    capi_call_instrumentation(
        state,
        codelike,
        offset,
        args.0.as_mut_ptr() as *mut *mut _object,
        4i64,
        4i32,
    )
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyMonitoring_FireLineEvent(
    state: *mut _PyMonitoringState,
    codelike: *mut _object,
    offset: i32,
    lineno: i32,
) -> i32 {
    let lno: *mut _object = unsafe { PyLong_FromLong(lineno as i64) };
    if lno.is_null() {
        return 1i32.wrapping_neg();
    }
    let mut args: __SlateAlign16<[*mut _object; 3]> = __SlateAlign16([
        core::ptr::null_mut::<_object>(),
        core::ptr::null_mut::<_object>(),
        lno,
    ]);
    let res: i32 = capi_call_instrumentation(
        state,
        codelike,
        offset,
        args.0.as_mut_ptr() as *mut *mut _object,
        2i64,
        5i32,
    );
    Py_DECREF!(lno);
    res
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyMonitoring_FireJumpEvent(
    state: *mut _PyMonitoringState,
    codelike: *mut _object,
    offset: i32,
    target_offset: *mut _object,
) -> i32 {
    let mut args: __SlateAlign16<[*mut _object; 4]> = __SlateAlign16([
        core::ptr::null_mut::<_object>(),
        core::ptr::null_mut::<_object>(),
        core::ptr::null_mut::<_object>(),
        target_offset,
    ]);
    capi_call_instrumentation(
        state,
        codelike,
        offset,
        args.0.as_mut_ptr() as *mut *mut _object,
        3i64,
        7i32,
    )
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyMonitoring_FireBranchEvent(
    state: *mut _PyMonitoringState,
    codelike: *mut _object,
    offset: i32,
    target_offset: *mut _object,
) -> i32 {
    let mut args: __SlateAlign16<[*mut _object; 4]> = __SlateAlign16([
        core::ptr::null_mut::<_object>(),
        core::ptr::null_mut::<_object>(),
        core::ptr::null_mut::<_object>(),
        target_offset,
    ]);
    capi_call_instrumentation(
        state,
        codelike,
        offset,
        args.0.as_mut_ptr() as *mut *mut _object,
        3i64,
        9i32,
    )
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyMonitoring_FireBranchRightEvent(
    state: *mut _PyMonitoringState,
    codelike: *mut _object,
    offset: i32,
    target_offset: *mut _object,
) -> i32 {
    let mut args: __SlateAlign16<[*mut _object; 4]> = __SlateAlign16([
        core::ptr::null_mut::<_object>(),
        core::ptr::null_mut::<_object>(),
        core::ptr::null_mut::<_object>(),
        target_offset,
    ]);
    capi_call_instrumentation(
        state,
        codelike,
        offset,
        args.0.as_mut_ptr() as *mut *mut _object,
        3i64,
        9i32,
    )
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyMonitoring_FireBranchLeftEvent(
    state: *mut _PyMonitoringState,
    codelike: *mut _object,
    offset: i32,
    target_offset: *mut _object,
) -> i32 {
    let mut args: __SlateAlign16<[*mut _object; 4]> = __SlateAlign16([
        core::ptr::null_mut::<_object>(),
        core::ptr::null_mut::<_object>(),
        core::ptr::null_mut::<_object>(),
        target_offset,
    ]);
    capi_call_instrumentation(
        state,
        codelike,
        offset,
        args.0.as_mut_ptr() as *mut *mut _object,
        3i64,
        8i32,
    )
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyMonitoring_FireCReturnEvent(
    state: *mut _PyMonitoringState,
    codelike: *mut _object,
    offset: i32,
    retval: *mut _object,
) -> i32 {
    let mut args: __SlateAlign16<[*mut _object; 4]> = __SlateAlign16([
        core::ptr::null_mut::<_object>(),
        core::ptr::null_mut::<_object>(),
        core::ptr::null_mut::<_object>(),
        retval,
    ]);
    capi_call_instrumentation(
        state,
        codelike,
        offset,
        args.0.as_mut_ptr() as *mut *mut _object,
        3i64,
        16i32,
    )
}

fn exception_event_setup(exc: &mut *mut _object, event: i32) -> i32 {
    unsafe {
        *(exc as *mut *mut _object) = PyErr_GetRaisedException();
    }
    if unsafe { *(exc as *mut *mut _object) }.is_null() {
        unsafe {
            PyErr_Format(
                PyExc_ValueError,
                c"Firing event %d with no exception set".as_ptr(),
                event,
            )
        };
        return 1i32.wrapping_neg();
    }
    0
}

fn exception_event_teardown(err: i32, exc: *mut _object) -> i32 {
    if err == 0 {
        unsafe { PyErr_SetRaisedException(exc) };
    } else {
        Py_XDECREF!(exc);
    }
    err
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyMonitoring_FirePyThrowEvent(
    state: *mut _PyMonitoringState,
    codelike: *mut _object,
    offset: i32,
) -> i32 {
    let event: i32 = 14;
    let mut exc: *mut _object = unsafe { core::mem::zeroed() };
    if exception_event_setup(unsafe { &mut (*core::ptr::addr_of_mut!(exc)) }, event) < 0 {
        return 1i32.wrapping_neg();
    }
    let mut args: __SlateAlign16<[*mut _object; 4]> = __SlateAlign16([
        core::ptr::null_mut::<_object>(),
        core::ptr::null_mut::<_object>(),
        core::ptr::null_mut::<_object>(),
        exc,
    ]);
    let err: i32 = capi_call_instrumentation(
        state,
        codelike,
        offset,
        args.0.as_mut_ptr() as *mut *mut _object,
        3i64,
        event,
    );
    exception_event_teardown(err, exc)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyMonitoring_FireRaiseEvent(
    state: *mut _PyMonitoringState,
    codelike: *mut _object,
    offset: i32,
) -> i32 {
    let event: i32 = 11;
    let mut exc: *mut _object = unsafe { core::mem::zeroed() };
    if exception_event_setup(unsafe { &mut (*core::ptr::addr_of_mut!(exc)) }, event) < 0 {
        return 1i32.wrapping_neg();
    }
    let mut args: __SlateAlign16<[*mut _object; 4]> = __SlateAlign16([
        core::ptr::null_mut::<_object>(),
        core::ptr::null_mut::<_object>(),
        core::ptr::null_mut::<_object>(),
        exc,
    ]);
    let err: i32 = capi_call_instrumentation(
        state,
        codelike,
        offset,
        args.0.as_mut_ptr() as *mut *mut _object,
        3i64,
        event,
    );
    exception_event_teardown(err, exc)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyMonitoring_FireCRaiseEvent(
    state: *mut _PyMonitoringState,
    codelike: *mut _object,
    offset: i32,
) -> i32 {
    let event: i32 = 17;
    let mut exc: *mut _object = unsafe { core::mem::zeroed() };
    if exception_event_setup(unsafe { &mut (*core::ptr::addr_of_mut!(exc)) }, event) < 0 {
        return 1i32.wrapping_neg();
    }
    let mut args: __SlateAlign16<[*mut _object; 4]> = __SlateAlign16([
        core::ptr::null_mut::<_object>(),
        core::ptr::null_mut::<_object>(),
        core::ptr::null_mut::<_object>(),
        exc,
    ]);
    let err: i32 = capi_call_instrumentation(
        state,
        codelike,
        offset,
        args.0.as_mut_ptr() as *mut *mut _object,
        3i64,
        event,
    );
    exception_event_teardown(err, exc)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyMonitoring_FireReraiseEvent(
    state: *mut _PyMonitoringState,
    codelike: *mut _object,
    offset: i32,
) -> i32 {
    let event: i32 = 15;
    let mut exc: *mut _object = unsafe { core::mem::zeroed() };
    if exception_event_setup(unsafe { &mut (*core::ptr::addr_of_mut!(exc)) }, event) < 0 {
        return 1i32.wrapping_neg();
    }
    let mut args: __SlateAlign16<[*mut _object; 4]> = __SlateAlign16([
        core::ptr::null_mut::<_object>(),
        core::ptr::null_mut::<_object>(),
        core::ptr::null_mut::<_object>(),
        exc,
    ]);
    let err: i32 = capi_call_instrumentation(
        state,
        codelike,
        offset,
        args.0.as_mut_ptr() as *mut *mut _object,
        3i64,
        event,
    );
    exception_event_teardown(err, exc)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyMonitoring_FireExceptionHandledEvent(
    state: *mut _PyMonitoringState,
    codelike: *mut _object,
    offset: i32,
) -> i32 {
    let event: i32 = 12;
    let mut exc: *mut _object = unsafe { core::mem::zeroed() };
    if exception_event_setup(unsafe { &mut (*core::ptr::addr_of_mut!(exc)) }, event) < 0 {
        return 1i32.wrapping_neg();
    }
    let mut args: __SlateAlign16<[*mut _object; 4]> = __SlateAlign16([
        core::ptr::null_mut::<_object>(),
        core::ptr::null_mut::<_object>(),
        core::ptr::null_mut::<_object>(),
        exc,
    ]);
    let err: i32 = capi_call_instrumentation(
        state,
        codelike,
        offset,
        args.0.as_mut_ptr() as *mut *mut _object,
        3i64,
        event,
    );
    exception_event_teardown(err, exc)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyMonitoring_FirePyUnwindEvent(
    state: *mut _PyMonitoringState,
    codelike: *mut _object,
    offset: i32,
) -> i32 {
    let event: i32 = 13;
    let mut exc: *mut _object = unsafe { core::mem::zeroed() };
    if exception_event_setup(unsafe { &mut (*core::ptr::addr_of_mut!(exc)) }, event) < 0 {
        return 1i32.wrapping_neg();
    }
    let mut args: __SlateAlign16<[*mut _object; 4]> = __SlateAlign16([
        core::ptr::null_mut::<_object>(),
        core::ptr::null_mut::<_object>(),
        core::ptr::null_mut::<_object>(),
        exc,
    ]);
    let err: i32 = capi_call_instrumentation(
        state,
        codelike,
        offset,
        args.0.as_mut_ptr() as *mut *mut _object,
        3i64,
        event,
    );
    exception_event_teardown(err, exc)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyMonitoring_FireStopIterationEvent(
    state: *mut _PyMonitoringState,
    codelike: *mut _object,
    offset: i32,
    value: *mut _object,
) -> i32 {
    let event: i32 = 10;
    unsafe { PyErr_SetObject(PyExc_StopIteration, value) };
    let mut exc: *mut _object = unsafe { core::mem::zeroed() };
    if exception_event_setup(unsafe { &mut (*core::ptr::addr_of_mut!(exc)) }, event) < 0 {
        return 1i32.wrapping_neg();
    }
    let mut args: __SlateAlign16<[*mut _object; 4]> = __SlateAlign16([
        core::ptr::null_mut::<_object>(),
        core::ptr::null_mut::<_object>(),
        core::ptr::null_mut::<_object>(),
        exc,
    ]);
    let err: i32 = capi_call_instrumentation(
        state,
        codelike,
        offset,
        args.0.as_mut_ptr() as *mut *mut _object,
        3i64,
        event,
    );
    Py_DECREF!(exc);
    exception_event_teardown(err, core::ptr::null_mut::<_object>())
}

// Handle legacy BRANCH event
#[repr(C)]
#[derive(Clone, Copy)]
struct _PyLegacyBranchEventHandler {
    ob_base: _object,
    vectorcall: Option<
        unsafe extern "C-unwind" fn(
            *mut _object,
            *const *mut _object,
            u64,
            *mut _object,
        ) -> *mut _object,
    >,
    handler: *mut _object,
    right: bool,
    tool_id: i32,
}

#[unsafe(link_section = ".text.slate_distinct.instrumentation.dealloc_branch_handler")]
extern "C-unwind" fn dealloc_branch_handler(op: *mut _object) {
    let self_: *mut _PyLegacyBranchEventHandler = _PyLegacyBranchEventHandler_CAST!(op);
    Py_CLEAR!((*self_).handler);
    unsafe { PyObject_Free(self_ as *mut ()) };
}

static mut _PyLegacyBranchEventHandler_Type: _typeobject = {
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
    __t0.tp_name = (b"sys.monitoring.branch_event_handler\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<_PyLegacyBranchEventHandler>() as u64) as i64;
    __t0.tp_dealloc = Some(dealloc_branch_handler);
    __t0.tp_vectorcall_offset =
        (core::mem::offset_of!(_PyLegacyBranchEventHandler, vectorcall) as u64) as i64;
    __t0.tp_call = unsafe {
        core::mem::transmute::<
            *const (),
            Option<
                unsafe extern "C-unwind" fn(
                    *mut _object,
                    *mut _object,
                    *mut _object,
                ) -> *mut _object,
            >,
        >(PyVectorcall_Call as *const ())
    };
    __t0.tp_flags = 3200u64;
    __t0
};

#[unsafe(link_section = ".text.slate_distinct.instrumentation.branch_handler_vectorcall")]
extern "C-unwind" fn branch_handler_vectorcall(
    op: *mut _object,
    args: *const *mut _object,
    nargsf: u64,
    kwnames: *mut _object,
) -> *mut _object {
    let self_: *mut _PyLegacyBranchEventHandler = _PyLegacyBranchEventHandler_CAST!(op);
    // Find the other instrumented instruction and remove tool
    //
    // The spec (PEP 669) allows spurious events after a DISABLE,
    //
    // so a best effort is good enough.
    let code: *mut PyCodeObject =
        (unsafe { *args.wrapping_offset(0 as isize) }) as *mut PyCodeObject;
    let src_offset: i32 = (unsafe { PyLong_AsLong(*args.wrapping_offset(1 as isize)) }) as i32;
    if !unsafe { PyErr_Occurred() }.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let instr: __SlateRecord_pycore_structs_h_32 =
        unsafe { *_PyCode_CODE!(code).wrapping_offset((src_offset / 2) as isize) };
    if !is_instrumented((unsafe { instr.op.code }) as i32) {
        // Already disabled
        return unsafe { core::ptr::addr_of_mut!(_PyInstrumentation_DISABLE) };
    }
    let res: *mut _object = unsafe { PyObject_Vectorcall((*self_).handler, args, nargsf, kwnames) };
    if res == unsafe { core::ptr::addr_of_mut!(_PyInstrumentation_DISABLE) } {
        // We need FOR_ITER and POP_JUMP_ to be the same size
        let mut offset: i32 = 0;
        let mut other_event: i32 = 0;
        if ((unsafe { instr.op.code }) as i32) == 70 {
            if unsafe { (*self_).right } {
                offset = src_offset / 2;
                other_event = 8;
            } else {
                // We don't know where the POP_ITER is, so
                //
                // we cannot de-instrument it.
                return res;
            }
        } else {
            if IS_CONDITIONAL_JUMP_OPCODE!(unsafe { instr.op.code }) {
                if unsafe { (*self_).right } {
                    offset = (src_offset / 2).wrapping_add(2);
                    other_event = 8;
                } else {
                    offset = src_offset / 2;
                    other_event = 9;
                }
            } else {
                // Orphaned NOT_TAKEN -- Jump removed by the compiler
                return res;
            }
        }
        let interp: *mut _is = _PyInterpreterState_GET();
        unsafe { _PyEval_StopTheWorld(interp) };
        remove_tools(
            code,
            offset,
            other_event,
            1i32 << unsafe { (*self_).tool_id },
        );
        unsafe { _PyEval_StartTheWorld(interp) };
    }
    res
}

fn make_branch_handler(tool_id: i32, handler: *mut _object, right: bool) -> *mut _object {
    let callback: *mut _PyLegacyBranchEventHandler =
        PyObject_NEW!(_PyLegacyBranchEventHandler, unsafe {
            core::ptr::addr_of_mut!(_PyLegacyBranchEventHandler_Type)
        });
    if callback.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    unsafe {
        (*callback).vectorcall = Some(branch_handler_vectorcall);
    }
    unsafe {
        (*callback).handler = Py_NewRef!(handler);
    }
    unsafe {
        (*callback).right = right;
    }
    unsafe {
        (*callback).tool_id = tool_id;
    }
    callback as *mut _object
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyMonitoring_RegisterCallback(
    tool_id: i32,
    event_id: i32,
    obj: *mut _object,
) -> *mut _object {
    let mut res: *mut _object = unsafe { core::mem::zeroed() };
    if event_id == 18 {
        let mut left: *mut _object = unsafe { core::mem::zeroed() };
        let mut right: *mut _object = unsafe { core::mem::zeroed() };
        if obj.is_null() {
            left = core::ptr::null_mut::<_object>();
            right = core::ptr::null_mut::<_object>();
        } else {
            right = make_branch_handler(tool_id, obj, true);
            if right.is_null() {
                return core::ptr::null_mut::<_object>();
            }
            left = make_branch_handler(tool_id, obj, false);
            if left.is_null() {
                Py_DECREF!(right);
                return core::ptr::null_mut::<_object>();
            }
        }
        let interp: *mut _is = _PyInterpreterState_GET();
        unsafe { _PyEval_StopTheWorld(interp) };
        let old_right: *mut _object = unsafe {
            *((*((*interp).monitoring_callables.as_mut_ptr() as *mut [*mut _object; 19])
                .wrapping_offset(tool_id as isize))
            .as_mut_ptr() as *mut *mut _object)
                .wrapping_offset(9 as isize)
        };
        unsafe {
            *((*((*interp).monitoring_callables.as_mut_ptr() as *mut [*mut _object; 19])
                .wrapping_offset(tool_id as isize))
            .as_mut_ptr() as *mut *mut _object)
                .wrapping_offset(9 as isize) = right;
        }
        res = unsafe {
            *((*((*interp).monitoring_callables.as_mut_ptr() as *mut [*mut _object; 19])
                .wrapping_offset(tool_id as isize))
            .as_mut_ptr() as *mut *mut _object)
                .wrapping_offset(8 as isize)
        };
        unsafe {
            *((*((*interp).monitoring_callables.as_mut_ptr() as *mut [*mut _object; 19])
                .wrapping_offset(tool_id as isize))
            .as_mut_ptr() as *mut *mut _object)
                .wrapping_offset(8 as isize) = left;
        }
        unsafe { _PyEval_StartTheWorld(interp) };
        Py_XDECREF!(old_right);
    } else {
        let interp: *mut _is = _PyInterpreterState_GET();
        unsafe { _PyEval_StopTheWorld(interp) };
        res = unsafe {
            *((*((*interp).monitoring_callables.as_mut_ptr() as *mut [*mut _object; 19])
                .wrapping_offset(tool_id as isize))
            .as_mut_ptr() as *mut *mut _object)
                .wrapping_offset(event_id as isize)
        };
        unsafe {
            *((*((*interp).monitoring_callables.as_mut_ptr() as *mut [*mut _object; 19])
                .wrapping_offset(tool_id as isize))
            .as_mut_ptr() as *mut *mut _object)
                .wrapping_offset(event_id as isize) = Py_XNewRef!(obj);
        }
        unsafe { _PyEval_StartTheWorld(interp) };
    }
    let __v0: bool = !res.is_null()
        && Py_TYPE!(res) == unsafe { core::ptr::addr_of_mut!(_PyLegacyBranchEventHandler_Type) };
    if __v0 {
        let wrapper: *mut _PyLegacyBranchEventHandler = res as *mut _PyLegacyBranchEventHandler;
        res = Py_NewRef!(unsafe { (*wrapper).handler });
        Py_DECREF__2!(wrapper);
    }
    res
}

// Branch Iterator
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_instrumentation_c_3074 {
    ob_base: _object,
    bi_code: *mut PyCodeObject,
    bi_offset: i32,
}

fn int_triple(a: i32, b: i32, c: i32) -> *mut _object {
    let mut array: core::mem::MaybeUninit<__SlateAlign16<[*mut _object; 3]>> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_0: *mut [*mut _object; 3] =
        core::ptr::addr_of_mut!(array) as *mut [*mut _object; 3];
    let mut objc: *mut _object = unsafe { core::mem::zeroed() };
    let mut objb: *mut _object = unsafe { core::mem::zeroed() };
    let mut obja: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        obja = PyLong_FromLong(a as i64);
        objb = core::ptr::null_mut::<_object>();
        objc = core::ptr::null_mut::<_object>();
        if !obja.is_null() {
            objb = PyLong_FromLong(b as i64);
            if !objb.is_null() {
                objc = PyLong_FromLong(c as i64);
                if !objc.is_null() {
                    core::ptr::write(__slate_slot_0, [obja, objb, objc]);
                    return _PyTuple_FromArraySteal(
                        (*__slate_slot_0).as_mut_ptr() as *const *mut _object,
                        3i64,
                    );
                }
            }
        }
        Py_XDECREF!(obja);
        Py_XDECREF!(objb);
        Py_XDECREF!(objc);
        return core::ptr::null_mut::<_object>();
    }
}

#[unsafe(link_section = ".text.slate_distinct.instrumentation.branchesiter_next")]
extern "C-unwind" fn branchesiter_next(op: *mut _object) -> *mut _object {
    let bi: *mut __SlateRecord_instrumentation_c_3074 = branchesiterator_CAST!(op);
    let mut offset: i32 = unsafe { (*bi).bi_offset };
    let mut oparg: i32 = 0;
    while (offset as i64) < Py_SIZE!(unsafe { (*bi).bi_code }) {
        let inst: __SlateRecord_pycore_structs_h_32 =
            _Py_GetBaseCodeUnit(unsafe { (*bi).bi_code }, offset);
        let next_offset: i32 = offset
            .wrapping_add(1)
            .wrapping_add((unsafe { _PyOpcode_Caches[(inst.op.code as i32) as usize] }) as i32);
        let mut not_taken: i32 = 0;
        match (unsafe { inst.op.code }) as i32 {
            69 => {
                oparg = oparg << 8i32 | ((unsafe { inst.op.arg }) as i32);
            }
            70 => {
                oparg = oparg << 8i32 | ((unsafe { inst.op.arg }) as i32);
                unsafe {
                    (*bi).bi_offset = next_offset;
                }
                let target: i32 = next_offset.wrapping_add(oparg).wrapping_add(2); // Skips END_FOR and POP_ITER
                return int_triple(
                    offset.wrapping_mul(2),
                    next_offset.wrapping_mul(2),
                    target.wrapping_mul(2),
                );
            }
            100..=103 => {
                oparg = oparg << 8i32 | ((unsafe { inst.op.arg }) as i32);
                // Skip NOT_TAKEN
                not_taken = next_offset.wrapping_add(1);
                unsafe {
                    (*bi).bi_offset = not_taken;
                }
                return int_triple(
                    offset.wrapping_mul(2),
                    not_taken.wrapping_mul(2),
                    next_offset.wrapping_add(oparg).wrapping_mul(2),
                );
            }
            68 => {
                oparg = oparg << 8i32 | ((unsafe { inst.op.arg }) as i32);
                let src_offset: i32 = next_offset.wrapping_sub(oparg);
                unsafe {
                    (*bi).bi_offset = next_offset;
                }
                not_taken = src_offset.wrapping_add(2);
                return int_triple(
                    src_offset.wrapping_mul(2),
                    not_taken.wrapping_mul(2),
                    next_offset.wrapping_mul(2),
                );
            }
            _ => {
                oparg = 0;
            }
        }
        offset = next_offset;
    }
    core::ptr::null_mut::<_object>()
}

#[unsafe(link_section = ".text.slate_distinct.instrumentation.branchesiter_dealloc")]
extern "C-unwind" fn branchesiter_dealloc(op: *mut _object) {
    let bi: *mut __SlateRecord_instrumentation_c_3074 = branchesiterator_CAST!(op);
    Py_DECREF__2!(unsafe { (*bi).bi_code });
    unsafe { PyObject_Free(bi as *mut ()) };
}

/// tp_name
/// tp_basicsize
/// tp_itemsize
/// methods
static mut _PyBranchesIterator: _typeobject = {
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
    __t0.tp_name = (b"line_iterator\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize =
        (core::mem::size_of::<__SlateRecord_instrumentation_c_3074>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(branchesiter_dealloc);
    __t0.tp_flags = 1024u64;
    __t0.tp_iter = unsafe {
        core::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
        >(PyObject_SelfIter as *const ())
    };
    __t0.tp_iternext = Some(branchesiter_next);
    __t0.tp_free = unsafe {
        core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
            PyObject_Free as *const (),
        )
    };
    __t0
};

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyInstrumentation_BranchesIterator(code: *mut PyCodeObject) -> *mut _object {
    let bi: *mut __SlateRecord_instrumentation_c_3074 =
        (unsafe { PyType_GenericAlloc(core::ptr::addr_of_mut!(_PyBranchesIterator), 0i64) })
            as *mut __SlateRecord_instrumentation_c_3074;
    if bi.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    unsafe {
        (*bi).bi_code = Py_NewRef__2!(code) as *mut PyCodeObject;
    }
    unsafe {
        (*bi).bi_offset = 0;
    }
    bi as *mut _object
}
