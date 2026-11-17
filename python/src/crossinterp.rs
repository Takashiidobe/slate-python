//! API for managing interactions between isolated interpreters
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
//! bpo-39562: CO_FUTURE_ and PyCF_ constants must be kept unique.
//! PyCF_ constants can use bits from 0x0100 to 0x10000.
//! CO_FUTURE_ constants use bits starting at 0x20000.
//! Initial JUMP_BACKWARD counter.
//! This determines when we create a trace for a loop.
//! Chosen so that _PyObjectStackChunk is a power-of-two size.
//! Largest tuple to save on freelist
//! leads to warnings... __attribute__((cdecl))
//! Various defines for all used memory orders in mimalloc
//! Main tuning parameters for segment and page sizes
//!
//! Sizes for 64-bit (usually divide by two for 32-bit)
//! Inlined definitions
//! A value of 1 means that we attempt to specialize the *second* time each
//!
//! instruction is executed. Executing twice is a much better indicator of
//!
//! "hotness" than executing once, but additional warmup delays only prevent
//!
//! specialization. Most types stabilize by the second execution, too:
//! Total tool ids available
//! Almost all errors causing Python initialization to fail
//!
//! other fields are set to 0
//! Bit flags for _gc_prev
//!
//! Bit 0 is set when tp_finalize is called
// Signed variants of the above
// Initial exit temperature.
// Must be larger than ADAPTIVE_COOLDOWN_VALUE,
// otherwise when a side exit warms up we may construct
// a new trace before the Tier 1 code has properly re-specialized.
// 64KiB  (32KiB on 32-bit)
// 1024
// Used as a special value to encode block sizes in 32 bits.
// 35 == mi_segment_bin(MI_SLICES_PER_SEGMENT)
// initialize a local variable to zero; use memset as compilers optimize constant sized memset's
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
macro_rules! PyBytes_Check {
    ($op:expr) => {
        PyType_FastSubclass!(Py_TYPE!($op), 134217728u64)
    };
}

macro_rules! PyCode_Check {
    ($op:expr) => {
        Py_IS_TYPE!($op, unsafe { core::ptr::addr_of_mut!(PyCode_Type) })
    };
}

macro_rules! PyDict_CheckExact {
    ($op:expr) => {
        Py_IS_TYPE!($op, unsafe { core::ptr::addr_of_mut!(PyDict_Type) })
    };
}

/// Mask the old API with a call to the new API for code compiled under
/// Python 2.0:
macro_rules! PyErr_BadInternalCall__49 {
    () => {
        unsafe {
            _PyErr_BadInternalCall(
                c"/home/takashi/c-corpus/cpython/Python/crossinterp.c".as_ptr(),
                2258i32,
            )
        };
    };
}

macro_rules! PyExceptionClass_Check {
    ($x:expr) => {{
        let __v: bool = PyType_Check!($x) != 0
            && PyType_FastSubclass!(($x) as *mut _typeobject, 1073741824u64) != 0;
        __v
    }};
}

macro_rules! PyExceptionInstance_Check {
    ($x:expr) => {
        PyType_FastSubclass!(Py_TYPE!($x), 1073741824u64)
    };
}

macro_rules! PyFunction_Check {
    ($op:expr) => {
        Py_IS_TYPE!($op, unsafe { core::ptr::addr_of_mut!(PyFunction_Type) })
    };
}

macro_rules! PyFunction_GET_CODE {
    ($func:expr) => {
        PyFunction_GET_CODE(_PyObject_CAST!($func))
    };
}

/// Alias to PyThreadState_Get()
macro_rules! PyThreadState_GET {
    () => {
        unsafe { PyThreadState_Get() }
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

macro_rules! PyTuple_SET_ITEM__2 {
    ($op:expr, $index:expr, $value:expr) => {
        PyTuple_SET_ITEM(_PyObject_CAST!($op), $index, _PyObject_CAST!($value));
    };
}

macro_rules! PyType_Check {
    ($op:expr) => {
        PyType_Check(_PyObject_CAST!($op))
    };
}

macro_rules! PyType_Check__2 {
    ($op:expr) => {
        PyType_Check(_PyObject_CAST__2!($op))
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

macro_rules! PyUnicode_DATA {
    ($op:expr) => {
        _PyUnicode_DATA(_PyObject_CAST!($op))
    };
}

macro_rules! PyUnicode_GET_LENGTH {
    ($op:expr) => {
        PyUnicode_GET_LENGTH(_PyObject_CAST!($op))
    };
}

macro_rules! PyUnicode_IS_ASCII {
    ($op:expr) => {
        PyUnicode_IS_ASCII(_PyObject_CAST!($op))
    };
}

macro_rules! PyUnicode_IS_COMPACT {
    ($op:expr) => {
        PyUnicode_IS_COMPACT(_PyObject_CAST!($op))
    };
}

/// PyUnicode_KIND(): Return one of the PyUnicode_*_KIND values defined above.
///
///
///
/// gh-89653: Converting this macro to a static inline function would introduce
///
/// new compiler warnings on "kind < PyUnicode_KIND(str)" (compare signed and
///
/// unsigned numbers) where kind type is an int or on
///
/// "unsigned int kind = PyUnicode_KIND(str)" (cast signed to unsigned).
macro_rules! PyUnicode_KIND {
    ($op:expr) => {
        _Py_RVALUE!(
            (unsafe {
                (*_PyASCIIObject_CAST!($op))
                    .state
                    .__slate_bits_0
                    .__get_kind()
            }) as i32
        )
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

macro_rules! Py_DECREF {
    ($op:expr) => {
        Py_DECREF(_PyObject_CAST!($op));
    };
}

macro_rules! Py_FatalError__2 {
    ($message:expr) => {
        unsafe {
            _Py_FatalErrorFunc(
                c"_register_builtins_for_crossinterpreter_data".as_ptr(),
                ($message) as *const i8,
            )
        };
        unsafe { core::hint::unreachable_unchecked() };
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

macro_rules! Py_IsTrue {
    ($x:expr) => {
        Py_Is!(
            $x,
            _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_TrueStruct) })
        )
    };
}

macro_rules! Py_Is {
    ($x:expr, $y:expr) => {
        ($x) == $y
    };
}

macro_rules! Py_NewRef {
    ($obj:expr) => {
        _Py_NewRef(_PyObject_CAST!($obj))
    };
}

macro_rules! Py_REFCNT {
    ($ob:expr) => {
        _Py_REFCNT(_PyObject_CAST!($ob))
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

macro_rules! _PyASCIIObject_CAST {
    ($op:expr) => {
        _Py_CAST!(*mut __SlateRecord_unicodeobject_h_163, $op)
    };
}

macro_rules! _PyCompactUnicodeObject_CAST {
    ($op:expr) => {
        _Py_CAST!(*mut __SlateRecord_unicodeobject_h_173, $op)
    };
}

macro_rules! _PyFunction_CAST {
    ($func:expr) => {
        _Py_CAST!(*mut __SlateRecord_funcobject_h_62, $func)
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

macro_rules! _PyStatus_GET_FUNC__62 {
    () => {
        c"_PyXI_InitTypes".as_ptr() as *mut i8
    };
}

macro_rules! _PyStatus_GET_FUNC__6 {
    () => {
        c"_PyXI_Init".as_ptr() as *mut i8
    };
}

/// Cast argument to PyTupleObject* type.
macro_rules! _PyTuple_CAST {
    ($op:expr) => {
        _Py_CAST!(*mut __SlateRecord_tupleobject_h_13, $op)
    };
}

macro_rules! _PyUnicodeObject_CAST {
    ($op:expr) => {
        _Py_CAST!(*mut __SlateRecord_unicodeobject_h_184, $op)
    };
}

/// Cast argument to PyVarObject* type.
macro_rules! _PyVarObject_CAST {
    ($op:expr) => {
        _Py_CAST!(*mut __SlateRecord_object_h_172, $op)
    };
}

macro_rules! _PyXIData_INTERPID {
    ($DATA:expr) => {
        unsafe { (*$DATA).interpid }
    };
}

/// Normally the Init* functions are sufficient.  The only time
///
/// additional initialization might be needed is to set the "free" func,
///
/// though that should be infrequent.
macro_rules! _PyXIData_SET_FREE {
    ($DATA:expr, $FUNC:expr) => {
        loop {
            unsafe {
                (*$DATA).free = $FUNC;
            }
            if true {
                break;
            }
        }
    };
}

/// Additionally, some shareable types are essentially light wrappers
///
/// around other shareable types.  The xidatafunc of the wrapper
///
/// can often be implemented by calling the wrapped object's
///
/// xidatafunc and then changing the "new_object" function.
///
/// We have _PyXIData_SET_NEW_OBJECT() here for that,
///
/// but might be better to have a function like
///
/// _PyXIData_AdaptToWrapper() instead.
macro_rules! _PyXIData_SET_NEW_OBJECT {
    ($DATA:expr, $FUNC:expr) => {
        loop {
            unsafe {
                (*$DATA).new_object = $FUNC;
            }
            if true {
                break;
            }
        }
    };
}

macro_rules! _PyXI_GET_GLOBAL_STATE {
    ($interp:expr) => {
        unsafe { core::ptr::addr_of_mut!((*(*$interp).runtime).xi) }
    };
}

macro_rules! _PyXI_GET_STATE {
    ($interp:expr) => {
        unsafe { core::ptr::addr_of_mut!((*$interp).xi) }
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

unsafe extern "C" {
    static mut PyLong_Type: _typeobject;
    static mut PyBool_Type: _typeobject;
    static mut _Py_NoneStruct: _object;
    static mut PyBytes_Type: _typeobject;
    static mut PyUnicode_Type: _typeobject;
    static mut PyExc_Exception: *mut _object;
    static mut PyExc_AttributeError: *mut _object;
    static mut PyExc_MemoryError: *mut _object;
    static mut PyExc_OverflowError: *mut _object;
    static mut PyExc_RuntimeError: *mut _object;
    static mut PyExc_NotImplementedError: *mut _object;
    static mut PyExc_SystemError: *mut _object;
    static mut PyExc_TypeError: *mut _object;
    static mut PyExc_ValueError: *mut _object;
    static mut _Py_FalseStruct: _longobject;
    static mut _Py_TrueStruct: _longobject;
    static mut PyFloat_Type: _typeobject;
    static mut PyTuple_Type: _typeobject;
    static mut PyDict_Type: _typeobject;
    static mut PyFunction_Type: _typeobject;
    static mut PyCode_Type: _typeobject;
    #[thread_local]
    static mut _Py_tss_tstate: *mut _ts;
    static mut _PyRuntime: pyruntimestate;
    fn index(arg0: *const i8, arg1: i32) -> *mut i8;
    fn PyMem_Calloc(arg0: u64, arg1: u64) -> *mut ();
    fn PyMem_Free(arg0: *mut ());
    fn PyMem_RawMalloc(arg0: u64) -> *mut ();
    fn PyMem_RawCalloc(arg0: u64, arg1: u64) -> *mut ();
    fn PyMem_RawFree(arg0: *mut ());
    fn PyMutex_Lock(arg0: *mut PyMutex);
    fn PyMutex_Unlock(arg0: *mut PyMutex);
    fn PyType_GetName(arg0: *mut _typeobject) -> *mut _object;
    fn PyType_GetQualName(arg0: *mut _typeobject) -> *mut _object;
    fn PyType_GetModuleName(arg0: *mut _typeobject) -> *mut _object;
    fn PyObject_Str(arg0: *mut _object) -> *mut _object;
    fn PyObject_GetAttrString(arg0: *mut _object, arg1: *const i8) -> *mut _object;
    fn PyObject_SetAttrString(arg0: *mut _object, arg1: *const i8, arg2: *mut _object) -> i32;
    fn _Py_Dealloc(arg0: *mut _object);
    fn PyBytes_FromStringAndSize(arg0: *const i8, arg1: i64) -> *mut _object;
    fn PyBytes_AsStringAndSize(arg0: *mut _object, arg1: *mut *mut i8, arg2: *mut i64) -> i32;
    fn PyUnicode_FromString(arg0: *const i8) -> *mut _object;
    fn PyUnicode_FromFormatV(arg0: *const i8, arg1: core::ffi::VaList<'static>) -> *mut _object;
    fn PyUnicode_FromFormat(arg0: *const i8, ...) -> *mut _object;
    fn PyUnicode_AsUTF8AndSize(arg0: *mut _object, arg1: *mut i64) -> *const i8;
    fn PyUnicode_Join(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn PyUnicode_FromKindAndData(arg0: i32, arg1: *const (), arg2: i64) -> *mut _object;
    fn PyUnicode_AsUTF8(arg0: *mut _object) -> *const i8;
    fn PyErr_SetNone(arg0: *mut _object);
    fn PyErr_SetObject(arg0: *mut _object, arg1: *mut _object);
    fn PyErr_SetString(arg0: *mut _object, arg1: *const i8);
    fn PyErr_Occurred() -> *mut _object;
    fn PyErr_Clear();
    fn PyErr_GetRaisedException() -> *mut _object;
    fn PyErr_SetRaisedException(arg0: *mut _object);
    fn PyErr_GivenExceptionMatches(arg0: *mut _object, arg1: *mut _object) -> i32;
    fn PyErr_ExceptionMatches(arg0: *mut _object) -> i32;
    fn PyException_GetCause(arg0: *mut _object) -> *mut _object;
    fn PyException_SetCause(arg0: *mut _object, arg1: *mut _object);
    fn PyException_SetContext(arg0: *mut _object, arg1: *mut _object);
    fn PyException_GetArgs(arg0: *mut _object) -> *mut _object;
    fn PyErr_NoMemory() -> *mut _object;
    fn PyErr_Format(arg0: *mut _object, arg1: *const i8, ...) -> *mut _object;
    fn _PyErr_BadInternalCall(arg0: *const i8, arg1: i32);
    fn PyErr_NewException(arg0: *const i8, arg1: *mut _object, arg2: *mut _object) -> *mut _object;
    fn _PyErr_ChainExceptions1(arg0: *mut _object);
    fn _Py_FatalErrorFunc(arg0: *const i8, arg1: *const i8);
    fn PyErr_FormatUnraisable(arg0: *const i8, ...);
    fn PyLong_FromSsize_t(arg0: i64) -> *mut _object;
    fn PyLong_AsSsize_t(arg0: *mut _object) -> i64;
    fn PyFloat_FromDouble(arg0: f64) -> *mut _object;
    fn PyFloat_AsDouble(arg0: *mut _object) -> f64;
    fn PyMemoryView_FromMemory(arg0: *mut i8, arg1: i64, arg2: i32) -> *mut _object;
    fn PyTuple_New(arg0: i64) -> *mut _object;
    fn PyTuple_Pack(arg0: i64, ...) -> *mut _object;
    fn PyDict_New() -> *mut _object;
    fn PyDict_SetItem(arg0: *mut _object, arg1: *mut _object, arg2: *mut _object) -> i32;
    fn PyDict_Next(
        arg0: *mut _object,
        arg1: *mut i64,
        arg2: *mut *mut _object,
        arg3: *mut *mut _object,
    ) -> i32;
    fn PyDict_Size(arg0: *mut _object) -> i64;
    fn PyDict_Update(arg0: *mut _object, arg1: *mut _object) -> i32;
    fn PyDict_GetItemString(arg0: *mut _object, arg1: *const i8) -> *mut _object;
    fn PyDict_SetItemString(arg0: *mut _object, arg1: *const i8, arg2: *mut _object) -> i32;
    fn PyDict_GetItemRef(arg0: *mut _object, arg1: *mut _object, arg2: *mut *mut _object) -> i32;
    fn PyDict_GetItemStringRef(arg0: *mut _object, arg1: *const i8, arg2: *mut *mut _object)
    -> i32;
    fn PyModule_NewObject(arg0: *mut _object) -> *mut _object;
    fn PyModule_GetDict(arg0: *mut _object) -> *mut _object;
    fn PyFunction_New(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn PyStatus_Exception(arg0: __SlateRecord_initconfig_h_19) -> i32;
    fn PyInterpreterState_Delete(arg0: *mut _is);
    fn PyInterpreterState_Get() -> *mut _is;
    fn PyInterpreterState_GetDict(arg0: *mut _is) -> *mut _object;
    fn PyInterpreterState_GetID(arg0: *mut _is) -> i64;
    fn PyThreadState_Clear(arg0: *mut _ts);
    fn PyThreadState_Delete(arg0: *mut _ts);
    fn PyThreadState_Get() -> *mut _ts;
    fn PyThreadState_Swap(arg0: *mut _ts) -> *mut _ts;
    fn PyThreadState_GetInterpreter(arg0: *mut _ts) -> *mut _is;
    fn PyWeakref_NewRef(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn Py_BuildValue(arg0: *const i8, ...) -> *mut _object;
    fn PyErr_PrintEx(arg0: i32);
    fn Py_CompileStringExFlags(
        arg0: *const i8,
        arg1: *const i8,
        arg2: i32,
        arg3: *mut __SlateRecord_compile_h_30,
        arg4: i32,
    ) -> *mut _object;
    fn Py_EndInterpreter(arg0: *mut _ts);
    fn Py_NewInterpreterFromConfig(
        arg0: *mut *mut _ts,
        arg1: *const __SlateRecord_pylifecycle_h_50,
    ) -> __SlateRecord_initconfig_h_19;
    fn PyImport_ImportModuleAttrString(arg0: *const i8, arg1: *const i8) -> *mut _object;
    fn PyObject_Call(arg0: *mut _object, arg1: *mut _object, arg2: *mut _object) -> *mut _object;
    fn PyObject_CallMethod(
        arg0: *mut _object,
        arg1: *const i8,
        arg2: *const i8,
        ...
    ) -> *mut _object;
    fn PyObject_Size(arg0: *mut _object) -> i64;
    fn PySequence_Check(arg0: *mut _object) -> i32;
    fn PySequence_Size(arg0: *mut _object) -> i64;
    fn PySequence_GetItem(arg0: *mut _object, arg1: i64) -> *mut _object;
    fn PyObject_CallOneArg(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn PyMarshal_ReadObjectFromString(arg0: *const i8, arg1: i64) -> *mut _object;
    fn PyMarshal_WriteObjectToString(arg0: *mut _object, arg1: i32) -> *mut _object;
    fn _PyCode_VerifyStateless(
        arg0: *mut _ts,
        arg1: *mut PyCodeObject,
        arg2: *mut _object,
        arg3: *mut _object,
        arg4: *mut _object,
    ) -> i32;
    fn _PyCode_ReturnsOnlyNone(arg0: *mut PyCodeObject) -> i32;
    fn _PyErr_GetRaisedException(arg0: *mut _ts) -> *mut _object;
    fn _PyErr_ExceptionMatches(arg0: *mut _ts, arg1: *mut _object) -> i32;
    fn _PyErr_SetRaisedException(arg0: *mut _ts, arg1: *mut _object);
    fn _PyErr_SetObject(arg0: *mut _ts, arg1: *mut _object, arg2: *mut _object);
    fn _PyErr_ChainExceptions1Tstate(arg0: *mut _ts, arg1: *mut _object);
    fn _PyErr_Clear(arg0: *mut _ts);
    fn _PyErr_SetString(arg0: *mut _ts, arg1: *mut _object, arg2: *const i8);
    fn _PyErr_Format(arg0: *mut _ts, arg1: *mut _object, arg2: *const i8, ...) -> *mut _object;
    fn _PyInterpreterState_LookUpID(arg0: i64) -> *mut _is;
    fn _PyInterpreterState_IsReady(arg0: *mut _is) -> i32;
    fn _PyInterpreterState_SetWhence(arg0: *mut _is, arg1: i64);
    fn _PyObject_SupportedAsScript(arg0: *mut _object) -> i32;
    fn _Py_SourceAsString(
        arg0: *mut _object,
        arg1: *const i8,
        arg2: *const i8,
        arg3: *mut __SlateRecord_compile_h_30,
        arg4: *mut *mut _object,
    ) -> *const i8;
    fn _PyInterpreterState_Main() -> *mut _is;
    fn _PyInterpreterState_SetRunningMain(arg0: *mut _is) -> i32;
    fn _PyInterpreterState_SetNotRunningMain(arg0: *mut _is);
    fn _PyErr_SetInterpreterAlreadyRunning();
    fn _PyThreadState_NewBound(arg0: *mut _is, arg1: i32) -> *mut _ts;
    fn _Py_GetMainModule(arg0: *mut _ts) -> *mut _object;
    fn _Py_CheckMainModule(arg0: *mut _object) -> i32;
    fn _PyEval_AddPendingCall(
        arg0: *mut _is,
        arg1: Option<unsafe extern "C-unwind" fn(*mut ()) -> i32>,
        arg2: *mut (),
        arg3: i32,
    ) -> i32;
    fn _PyEval_GetBuiltins(arg0: *mut _ts) -> *mut _object;
    fn _Py_CheckRecursiveCall(arg0: *mut _ts, arg1: *const i8) -> i32;
    fn _PyEval_GetGlobalsFromRunningMain(arg0: *mut _ts) -> *mut _object;
    fn _PyEval_EnsureBuiltins(arg0: *mut _ts, arg1: *mut _object, arg2: *mut *mut _object) -> i32;
    fn _PyFunction_VerifyStateless(arg0: *mut _ts, arg1: *mut _object) -> i32;
    fn _PyImport_SetModule(arg0: *mut _object, arg1: *mut _object) -> i32;
    fn _PyErr_SetFromPyStatus(arg0: __SlateRecord_initconfig_h_19);
    fn _PyNamespace_New(arg0: *mut _object) -> *mut _object;
    fn _PyModule_GetFilenameUTF8(arg0: *mut _object, arg1: *mut i8, arg2: i64) -> i64;
    fn _PyStaticType_InitBuiltin(arg0: *mut _is, arg1: *mut _typeobject) -> i32;
    fn _PyStaticType_FiniBuiltin(arg0: *mut _is, arg1: *mut _typeobject);
}

unsafe extern "llvm-intrinsic" {
    #[link_name = "llvm.frameaddress.p0"]
    fn __slate_intrinsic_2d7c29df8fdf6378(arg0: i32) -> *mut ();
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

#[repr(C)]
#[derive(Clone, Copy)]
struct PyMutex {
    _bits: u8,
}

fn _PyMutex_Lock(m: *mut PyMutex) {
    let mut expected: u8 = 0;
    if _Py_atomic_compare_exchange_uint8(
        unsafe { core::ptr::addr_of_mut!((*m)._bits) },
        unsafe { &mut (*core::ptr::addr_of_mut!(expected)) },
        1u8,
    ) == 0
    {
        unsafe { PyMutex_Lock(m) };
    }
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

#[repr(C)]
#[derive(Clone, Copy)]
struct _frame {}

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

// Track types initialized using _PyStaticType_InitBuiltin().
// Set if the type object is dynamically allocated
// Set if the type allows subclassing
// Objects support garbage collection (see objimpl.h)
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

fn PyType_Check(op: *mut _object) -> i32 {
    PyType_FastSubclass!(Py_TYPE!(op), 2147483648u64)
}

fn _Py_REFCNT(ob: *mut _object) -> i64 {
    (unsafe { (*ob).__slate_anon_0.__slate_anon_1.ob_refcnt }) as i64
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

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_unicodeobject_h_163_2 {
    __slate_bits_0: __slate_bits::__SlateBits144U0,
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
union __SlateRecord_unicodeobject_h_184_2 {
    any: *mut (),
    latin1: *mut u8,
    ucs2: *mut u16,
    ucs4: *mut u32,
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

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_unicodeobject_h_184 {
    _base: __SlateRecord_unicodeobject_h_173,
    data: __SlateRecord_unicodeobject_h_184_2,
}

fn PyUnicode_IS_ASCII(op: *mut _object) -> u32 {
    ((unsafe {
        (*_PyASCIIObject_CAST!(op))
            .state
            .__slate_bits_0
            .__get_ascii()
    }) as i32) as u32
}

fn PyUnicode_IS_COMPACT(op: *mut _object) -> u32 {
    ((unsafe {
        (*_PyASCIIObject_CAST!(op))
            .state
            .__slate_bits_0
            .__get_compact()
    }) as i32) as u32
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _PyExecutorObject {}

fn _PyUnicode_COMPACT_DATA(op: *mut _object) -> *mut () {
    if PyUnicode_IS_ASCII!(op) != 0 {
        return _Py_STATIC_CAST!(
            *mut (),
            _PyASCIIObject_CAST!(op).wrapping_offset(1 as isize)
        );
    }
    _Py_STATIC_CAST!(
        *mut (),
        _PyCompactUnicodeObject_CAST!(op).wrapping_offset(1 as isize)
    )
}

fn _PyUnicode_NONCOMPACT_DATA(op: *mut _object) -> *mut () {
    let data: *mut () = unsafe { (*_PyUnicodeObject_CAST!(op)).data.any };
    data
}

fn _PyUnicode_DATA(op: *mut _object) -> *mut () {
    if PyUnicode_IS_COMPACT!(op) != 0 {
        return _PyUnicode_COMPACT_DATA(op);
    }
    _PyUnicode_NONCOMPACT_DATA(op)
}

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

fn PyTuple_GET_SIZE(op: *mut _object) -> i64 {
    let tuple: *mut __SlateRecord_tupleobject_h_13 = _PyTuple_CAST!(op);
    Py_SIZE!(tuple)
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pystate_h_78 {
    __slate_bits_0: __slate_bits::__SlateBits196U0,
}

fn PyTuple_SET_ITEM(op: *mut _object, index: i64, value: *mut _object) {
    let tuple: *mut __SlateRecord_tupleobject_h_13 = _PyTuple_CAST!(op);
    unsafe {
        *((*tuple).ob_item.as_mut_ptr() as *mut *mut _object).wrapping_offset(index as isize) =
            value;
    }
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

fn PyFunction_GET_CODE(func: *mut _object) -> *mut _object {
    unsafe { (*_PyFunction_CAST!(func)).func_code }
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
struct __SlateRecord_pycore_structs_h_32_2 {
    code: u8,
    arg: u8,
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
struct _PyWeakReference {
    ob_base: _object,
    wr_object: *mut _object,
    wr_callback: *mut _object,
    hash: i64,
    wr_prev: *mut _PyWeakReference,
    wr_next: *mut _PyWeakReference,
    vectorcall: Option<
        unsafe extern "C-unwind" fn(
            *mut _object,
            *const *mut _object,
            u64,
            *mut _object,
        ) -> *mut _object,
    >,
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
struct __SlateRecord_compile_h_30 {
    cf_flags: i32,
    cf_feature_version: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pylifecycle_h_50 {
    use_main_obmalloc: i32,
    allow_fork: i32,
    allow_exec: i32,
    allow_threads: i32,
    allow_daemon_threads: i32,
    check_multi_interp_extensions: i32,
    gil: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _inittab {
    name: *const i8,
    initfunc: Option<unsafe extern "C-unwind" fn() -> *mut _object>,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_structs_h_15 {
    value_and_backoff: u16,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct code_arena_st {}

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
struct __SlateRecord_pycore_pythread_h_64 {
    ptr: *mut __SlateRecord_pthreadtypes_h_45,
    val: __SlateRecord_pthreadtypes_h_45,
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
struct __SlateRecord_pycore_interp_structs_h_297 {
    mutex: PyMutex,
    hashtable: *mut _Py_hashtable_t,
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
struct __SlateRecord_pycore_interp_structs_h_348 {
    import_level: i32,
    accumulated: i64,
    header: i32,
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
struct xi_exceptions {
    PyExc_InterpreterError: *mut _object,
    PyExc_InterpreterNotFoundError: *mut _object,
    PyExc_NotShareableError: *mut _object,
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
struct _excinfo_type {
    builtin: *mut _typeobject,
    name: *const i8,
    qualname: *const i8,
    module: *const i8,
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

fn _PyErr_Occurred(tstate: *mut _ts) -> *mut _object {
    if unsafe { (*tstate).current_exception }.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    Py_TYPE!(unsafe { (*tstate).current_exception }) as *mut _object
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
struct __SlateRecord_pycore_crossinterp_h_180 {
    bytes: *const i8,
    len: i64,
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
struct _excinfo {
    r#type: _excinfo_type,
    msg: *const i8,
    errdisplay: *const i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_crossinterp_h_360 {
    preserved: *mut _object,
    excinfo: *mut _object,
    errcode: i32,
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
struct __SlateRecord_pycore_interp_structs_h_718 {
    _not_used: i32,
    _hamt_empty_gc_not_used: __SlateRecord_pycore_interp_structs_h_170,
    hamt_empty: __SlateRecord_pycore_structs_h_47,
    last_resort_memory_error: __SlateRecord_pyerrors_h_15,
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
struct __SlateRecord_pycore_interp_structs_h_544 {
    r#type: *mut _typeobject,
    isbuiltin: i32,
    readying: i32,
    ready: i32,
    tp_dict: *mut _object,
    tp_subclasses: *mut _object,
    tp_weaklist: *mut _object,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord_pycore_obmalloc_h_255 {
    _padding: *mut u8,
    count: u32,
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
struct __SlateRecord_pycore_runtime_structs_h_49 {
    fd: i32,
    st_dev: u64,
    st_ino: u64,
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

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_tracemalloc_h_70 {
    mem: __SlateRecord_pymem_h_48,
    raw: __SlateRecord_pymem_h_48,
    obj: __SlateRecord_pymem_h_48,
}

fn _Py_IsMainInterpreter(interp: *mut _is) -> i32 {
    (interp == unsafe { _PyInterpreterState_Main() }) as i32
}

fn _PyThreadState_GET() -> *mut _ts {
    unsafe { _Py_tss_tstate }
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

fn _Py_get_machine_stack_pointer() -> u64 {
    (unsafe { __slate_intrinsic_2d7c29df8fdf6378(const { ((0 as i32) as u32) as i32 }) }) as u64
}

fn _Py_MakeRecCheck(tstate: *mut _ts) -> i32 {
    let here_addr: u64 = _Py_get_machine_stack_pointer();
    // Overflow if stack pointer is between soft limit and the base of the hardware stack.
    //
    // If it is below the hardware stack base, assume that we have the wrong stack limits, and do nothing.
    //
    // We could have the wrong stack limits because of limited platform support, or user-space threads.
    (here_addr < unsafe { (*(tstate as *mut _PyThreadStateImpl)).c_stack_soft_limit }
        && here_addr
            >= unsafe { (*(tstate as *mut _PyThreadStateImpl)).c_stack_soft_limit }.wrapping_sub(
                2u64.wrapping_mul(2048u64.wrapping_mul(core::mem::size_of::<*mut ()>() as u64)),
            )) as i32
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

fn _Py_EnterRecursiveCallTstate(tstate: *mut _ts, r#where: *const i8) -> i32 {
    let __v0: bool =
        _Py_MakeRecCheck(tstate) != 0 && (unsafe { _Py_CheckRecursiveCall(tstate, r#where) }) != 0;
    __v0 as i32
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord_sigaction_h_31 {
    sa_handler: Option<unsafe extern "C-unwind" fn(i32)>,
    sa_sigaction:
        Option<unsafe extern "C-unwind" fn(i32, *mut __SlateRecord_siginfo_t_h_124, *mut ())>,
}

fn _Py_LeaveRecursiveCallTstate(tstate: *mut _ts) {}

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
struct pyhash_runtime_state {
    urandom_cache: __SlateRecord_pycore_runtime_structs_h_49,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _PyTraceMalloc_Config {
    initialized: u32,
    tracing: i32,
    max_nframe: i32,
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
struct __SlateRecord_crossinterp_c_694 {
    utf8: *const i8,
    len: u64,
    _utf8: [i8; 4097],
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
    pub struct __SlateBits144U0 {
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
    pub struct __SlateBits196U0 {
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

use slate_support::strcmp;
use slate_support::strcpy;
use slate_support::strlen;
use slate_support::strncmp;

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

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_moduleobject_h_30 {
    ob_base: _object,
    md_dict: *mut _object,
    md_def: *mut PyModuleDef,
    md_state: *mut (),
    md_weaklist: *mut _object,
    md_name: *mut _object,
}

fn _PyModule_GetDict(r#mod: *mut _object) -> *mut _object {
    let dict: *mut _object =
        unsafe { (*(r#mod as *mut __SlateRecord_pycore_moduleobject_h_30)).md_dict };
    // _PyModule_GetDict(mod) must not be used after calling module_clear(mod)
    return dict; // borrowed reference
}

// PyMarshal_WriteObjectToString()
// MAXPATHLEN
// _Py_simple_func
// _PyXIData_t
// _PyFunction_VerifyStateless()
// _Py_ID()
// _PyImport_SetModule()
// _PyStatus_OK()
// _PyNamespace_New()
// _Py_SourceAsString()
// _PyRuntime
// _PySet_NextEntry()
// _PyStaticType_InitBuiltin()
fn _Py_GetMainfile(buffer: *mut i8, maxlen: u64) -> i64 {
    // We don't expect subinterpreters to have the __main__ module's
    //
    // __name__ set, but proceed just in case.
    let tstate: *mut _ts = _PyThreadState_GET();
    let module: *mut _object = unsafe { _Py_GetMainModule(tstate) };
    if (unsafe { _Py_CheckMainModule(module) }) < 0 {
        Py_XDECREF!(module);
        return 1i32.wrapping_neg() as i64;
    }
    let size: i64 = unsafe { _PyModule_GetFilenameUTF8(module, buffer, maxlen as i64) };
    Py_DECREF!(module);
    size
}

fn runpy_run_path(filename: *const i8, modname: *const i8) -> *mut _object {
    let run_path: *mut _object =
        unsafe { PyImport_ImportModuleAttrString(c"runpy".as_ptr(), c"run_path".as_ptr()) };
    if run_path.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let args: *mut _object = unsafe {
        Py_BuildValue(
            c"(sOs)".as_ptr(),
            filename,
            core::ptr::addr_of_mut!(_Py_NoneStruct),
            modname,
        )
    };
    if args.is_null() {
        Py_DECREF!(run_path);
        return core::ptr::null_mut::<_object>();
    }
    let ns: *mut _object =
        unsafe { PyObject_Call(run_path, args, core::ptr::null_mut::<_object>()) };
    Py_DECREF!(run_path);
    Py_DECREF!(args);
    ns
}

fn set_exc_with_cause(exctype: *mut _object, msg: *const i8) {
    let cause: *mut _object = unsafe { PyErr_GetRaisedException() };
    unsafe { PyErr_SetString(exctype, msg) };
    let exc: *mut _object = unsafe { PyErr_GetRaisedException() };
    unsafe { PyException_SetCause(exc, cause) };
    unsafe { PyErr_SetRaisedException(exc) };
}

// module duplication utils
#[repr(C)]
#[derive(Clone, Copy)]
struct sync_module_result {
    module: *mut _object,
    loaded: *mut _object,
    failed: *mut _object,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct sync_module {
    filename: *const i8,
    _filename: [i8; 4097],
    cached: sync_module_result,
}

fn sync_module_clear(data: *mut sync_module) {
    unsafe {
        (*data).filename = core::ptr::null::<i8>();
    }
    Py_CLEAR!((*data).cached.module);
    Py_CLEAR!((*data).cached.loaded);
    Py_CLEAR!((*data).cached.failed);
}

fn sync_module_capture_exc(tstate: *mut _ts, data: *mut sync_module) {
    let context: *mut _object = unsafe { (*data).cached.failed };
    let exc: *mut _object = unsafe { _PyErr_GetRaisedException(tstate) };
    unsafe { _PyErr_SetRaisedException(tstate, Py_NewRef!(exc)) };
    if !context.is_null() {
        unsafe { PyException_SetContext(exc, context) };
    }
    unsafe {
        (*data).cached.failed = exc;
    }
}

fn ensure_isolated_main(tstate: *mut _ts, main: *mut sync_module) -> i32 {
    let mut __slate_compound_7: sync_module_result = unsafe { core::mem::zeroed() };
    // We don't want to trigger "if __name__ == '__main__':",
    //
    // so we use a bogus module name.
    let mut loaded_ns: *mut _object = unsafe { core::mem::zeroed() };
    let mut ns: *mut _object = unsafe { core::mem::zeroed() };
    let mut key: *mut _object = unsafe { core::mem::zeroed() };
    // Try the per-interpreter cache for the loaded module.
    //
    // XXX Store it in sys.modules?
    let mut interpns: *mut _object = unsafe { core::mem::zeroed() };
    let mut loaded: *mut _object = unsafe { core::mem::zeroed() };
    // It wasn't in the local cache so we'll need to populate it.
    let mut r#mod: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        // Load the module from the original file (or from a cache).
        // First try the local cache.
        if !unsafe { (*main).cached.failed }.is_null() {
            // We'll deal with this in apply_isolated_main().
            return 0;
        } else {
            if !unsafe { (*main).cached.loaded }.is_null() {
                return 0;
            } else {
                if unsafe { (*main).filename }.is_null() {
                    _PyErr_SetString(tstate, PyExc_NotImplementedError, c"".as_ptr());
                    return 1i32.wrapping_neg();
                } else {
                    r#mod = _Py_GetMainModule(tstate);
                    if _Py_CheckMainModule(r#mod) < 0 {
                        // This is probably unrecoverable, so don't bother caching the error.
                        Py_XDECREF!(r#mod);
                        return 1i32.wrapping_neg();
                    } else {
                        loaded = core::ptr::null_mut::<_object>();
                        interpns = PyInterpreterState_GetDict((*tstate).interp);
                        key = PyUnicode_FromString(c"CACHED_MODULE_NS___main__".as_ptr());
                        if key.is_null() {
                            // It's probably unrecoverable, so don't bother caching the error.
                            Py_DECREF!(r#mod);
                            return 1i32.wrapping_neg();
                        } else {
                            if PyDict_GetItemRef(interpns, key, core::ptr::addr_of_mut!(loaded)) < 0
                            {
                                // It's probably unrecoverable, so don't bother caching the error.
                                Py_DECREF!(r#mod);
                                Py_DECREF!(key);
                                return 1i32.wrapping_neg();
                            } else {
                                '__join_1: {
                                    if loaded.is_null() {
                                        // It wasn't already loaded from file.
                                        loaded = PyModule_NewObject(core::ptr::addr_of_mut!(
                                            _PyRuntime
                                                .static_objects
                                                .singletons
                                                .strings
                                                .identifiers
                                                ._py___main__
                                                ._ascii
                                                .ob_base
                                        ));
                                        if !loaded.is_null() {
                                            ns = _PyModule_GetDict(loaded);
                                            loaded_ns = runpy_run_path(
                                                (*main).filename,
                                                c"<fake __main__>".as_ptr(),
                                            );
                                            if !loaded_ns.is_null() {
                                                let res: i32 = PyDict_Update(ns, loaded_ns);
                                                Py_DECREF!(loaded_ns);
                                                if res >= 0 {
                                                    // Set the per-interpreter cache entry.
                                                    if PyDict_SetItem(interpns, key, loaded) >= 0 {
                                                        break '__join_1;
                                                    }
                                                }
                                            }
                                        }
                                        sync_module_capture_exc(tstate, main);
                                        Py_XDECREF!(loaded);
                                        Py_DECREF!(r#mod);
                                        Py_XDECREF!(key);
                                        return 1i32.wrapping_neg();
                                    }
                                }
                                Py_DECREF!(key);
                                (*main).cached = *{
                                    __slate_compound_7 = {
                                        let mut __t0: sync_module_result = core::mem::zeroed();
                                        __t0.module = r#mod;
                                        __t0.loaded = loaded;
                                        __t0
                                    };
                                    core::ptr::addr_of_mut!(__slate_compound_7)
                                };
                                return 0;
                            }
                        }
                    }
                }
            }
        }
    }
}

fn apply_isolated_main(tstate: *mut _ts, main: *mut sync_module) -> i32 {
    if !unsafe { (*main).cached.failed }.is_null() {
        // It must have failed previously.
        unsafe { _PyErr_SetRaisedException(tstate, (*main).cached.failed) };
        return 1i32.wrapping_neg();
    }
    if (unsafe {
        _PyImport_SetModule(
            core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py___main__
                    ._ascii
                    .ob_base
            ),
            (*main).cached.loaded,
        )
    }) < 0
    {
        sync_module_capture_exc(tstate, main);
        return 1i32.wrapping_neg();
    }
    0
}

fn restore_main(tstate: *mut _ts, main: *mut sync_module) {
    let exc: *mut _object = unsafe { _PyErr_GetRaisedException(tstate) };
    let res: i32 = unsafe {
        _PyImport_SetModule(
            core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py___main__
                    ._ascii
                    .ob_base
            ),
            (*main).cached.module,
        )
    };
    if res < 0 {
        unsafe { PyErr_FormatUnraisable(c"Exception ignored while restoring __main__".as_ptr()) };
    }
    unsafe { _PyErr_SetRaisedException(tstate, exc) };
}

fn _ensure_current_cause(tstate: *mut _ts, cause: *mut _object) {
    if cause.is_null() {
        return;
    }
    let exc: *mut _object = unsafe { _PyErr_GetRaisedException(tstate) };
    unsafe { PyException_SetCause(exc, Py_NewRef!(cause)) };
    unsafe { _PyErr_SetRaisedException(tstate, exc) };
}

static mut _PyExc_InterpreterError: _typeobject = {
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
    __t0.tp_name = (b"concurrent.interpreters.InterpreterError\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_flags = 17408u64;
    __t0.tp_doc = (b"A cross-interpreter operation failed\0".as_ptr() as *mut i8) as *const i8;
    __t0
};

#[unsafe(no_mangle)]
static mut PyExc_InterpreterError: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_InterpreterError) }) as *mut _object;

static mut _PyExc_InterpreterNotFoundError: _typeobject = {
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
    __t0.tp_name =
        (b"concurrent.interpreters.InterpreterNotFoundError\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_flags = 17408u64;
    __t0.tp_doc = (b"An interpreter was not found\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(_PyExc_InterpreterError) };
    __t0
};

#[unsafe(no_mangle)]
static mut PyExc_InterpreterNotFoundError: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_InterpreterNotFoundError) }) as *mut _object;

fn _init_notshareableerror(state: *mut xi_exceptions) -> i32 {
    let name: *const i8 = c"concurrent.interpreters.NotShareableError".as_ptr();
    let base: *mut _object = unsafe { PyExc_TypeError };
    let ns: *mut _object = core::ptr::null_mut::<_object>();
    let exctype: *mut _object = unsafe { PyErr_NewException(name, base, ns) };
    if exctype.is_null() {
        return 1i32.wrapping_neg();
    }
    unsafe {
        (*state).PyExc_NotShareableError = exctype;
    }
    0
}

fn _fini_notshareableerror(state: *mut xi_exceptions) {
    Py_CLEAR!((*state).PyExc_NotShareableError);
}

fn get_notshareableerror_type(tstate: *mut _ts) -> *mut _object {
    let local: *mut __SlateRecord_pycore_crossinterp_h_268 =
        _PyXI_GET_STATE!(unsafe { (*tstate).interp });
    if local.is_null() {
        unsafe { PyErr_Clear() };
        return core::ptr::null_mut::<_object>();
    }
    unsafe { (*local).exceptions.PyExc_NotShareableError }
}

fn _ensure_notshareableerror(
    tstate: *mut _ts,
    cause: *mut _object,
    force: i32,
    msgobj: *mut _object,
) {
    let ctx: *mut _object = unsafe { _PyErr_GetRaisedException(tstate) };
    let mut exctype: *mut _object = get_notshareableerror_type(tstate);
    if !exctype.is_null() {
        let __v0: bool =
            force == 0 && !ctx.is_null() && Py_TYPE!(ctx) == (exctype as *mut _typeobject);
        if __v0 {
            // A NotShareableError instance is already set.
            unsafe { _PyErr_SetRaisedException(tstate, ctx) };
        }
    } else {
        exctype = unsafe { PyExc_TypeError };
    }
    unsafe { _PyErr_SetObject(tstate, exctype, msgobj) };
    // We have to set the context manually since _PyErr_SetObject() doesn't.
    unsafe { _PyErr_ChainExceptions1Tstate(tstate, ctx) };
    _ensure_current_cause(tstate, cause);
}

fn set_notshareableerror(tstate: *mut _ts, cause: *mut _object, force: i32, msg: *const i8) {
    let msgobj: *mut _object = unsafe { PyUnicode_FromString(msg) };
    if !msgobj.is_null() {
        _ensure_notshareableerror(tstate, cause, force, msgobj);
        Py_DECREF!(msgobj);
    }
}

fn format_notshareableerror_v(
    tstate: *mut _ts,
    cause: *mut _object,
    force: i32,
    format: *const i8,
    mut vargs: core::ffi::VaList<'static>,
) {
    let msgobj: *mut _object = unsafe { PyUnicode_FromFormatV(format, vargs.clone()) };
    if !msgobj.is_null() {
        _ensure_notshareableerror(tstate, cause, force, msgobj);
        Py_DECREF!(msgobj);
    }
}

unsafe extern "C-unwind" fn format_notshareableerror(
    tstate: *mut _ts,
    cause: *mut _object,
    force: i32,
    format: *const i8,
    mut __va_args: ...
) {
    let mut vargs: core::ffi::VaList<'static> = unsafe { core::mem::zeroed() };
    vargs = unsafe {
        core::mem::transmute::<core::ffi::VaList<'_>, core::ffi::VaList<'static>>(__va_args.clone())
    };
    format_notshareableerror_v(tstate, cause, force, format, vargs.clone());
}

fn init_static_exctypes(state: *mut xi_exceptions, interp: *mut _is) -> i32 {
    let mut base: *mut _typeobject = unsafe { core::mem::zeroed() };
    unsafe {
        base = PyExc_Exception as *mut _typeobject;
        // PyExc_InterpreterError
        _PyExc_InterpreterError.tp_base = base;
        _PyExc_InterpreterError.tp_traverse = (*base).tp_traverse;
        _PyExc_InterpreterError.tp_clear = (*base).tp_clear;
        if _PyStaticType_InitBuiltin(interp, core::ptr::addr_of_mut!(_PyExc_InterpreterError)) >= 0
        {
            (*state).PyExc_InterpreterError =
                core::ptr::addr_of_mut!(_PyExc_InterpreterError) as *mut _object;
            // PyExc_InterpreterNotFoundError
            _PyExc_InterpreterNotFoundError.tp_traverse = (*base).tp_traverse;
            _PyExc_InterpreterNotFoundError.tp_clear = (*base).tp_clear;
            if _PyStaticType_InitBuiltin(
                interp,
                core::ptr::addr_of_mut!(_PyExc_InterpreterNotFoundError),
            ) >= 0
            {
                (*state).PyExc_InterpreterNotFoundError =
                    core::ptr::addr_of_mut!(_PyExc_InterpreterNotFoundError) as *mut _object;
                return 0;
            }
        }
        fini_static_exctypes(state, interp);
        return 1i32.wrapping_neg();
    }
}

fn fini_static_exctypes(state: *mut xi_exceptions, interp: *mut _is) {
    if !unsafe { (*state).PyExc_InterpreterNotFoundError }.is_null() {
        unsafe {
            (*state).PyExc_InterpreterNotFoundError = core::ptr::null_mut::<_object>();
        }
        unsafe {
            _PyStaticType_FiniBuiltin(
                interp,
                core::ptr::addr_of_mut!(_PyExc_InterpreterNotFoundError),
            )
        };
    }
    if !unsafe { (*state).PyExc_InterpreterError }.is_null() {
        unsafe {
            (*state).PyExc_InterpreterError = core::ptr::null_mut::<_object>();
        }
        unsafe {
            _PyStaticType_FiniBuiltin(interp, core::ptr::addr_of_mut!(_PyExc_InterpreterError))
        };
    }
}

fn init_heap_exctypes(state: *mut xi_exceptions) -> i32 {
    unsafe {
        if _init_notshareableerror(state) < 0 {
            fini_heap_exctypes(state);
            return 1i32.wrapping_neg();
        } else {
            return 0;
        }
    }
}

fn fini_heap_exctypes(state: *mut xi_exceptions) {
    _fini_notshareableerror(state);
}

// exceptions
// cross-interpreter calls
#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_CallInInterpreter(
    interp: *mut _is,
    func: Option<unsafe extern "C-unwind" fn(*mut ()) -> i32>,
    arg: *mut (),
) -> i32 {
    if interp == unsafe { PyInterpreterState_Get() } {
        return unsafe { func.unwrap()(arg) };
    }
    // XXX Emit a warning if this fails?
    unsafe { _PyEval_AddPendingCall(interp, func, arg, 0i32) };
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_CallInInterpreterAndRawFree(
    interp: *mut _is,
    func: Option<unsafe extern "C-unwind" fn(*mut ()) -> i32>,
    arg: *mut (),
) -> i32 {
    if interp == unsafe { PyInterpreterState_Get() } {
        let res: i32 = unsafe { func.unwrap()(arg) };
        unsafe { PyMem_RawFree(arg) };
        return res;
    }
    // XXX Emit a warning if this fails?
    unsafe { _PyEval_AddPendingCall(interp, func, arg, 2i32) };
    0
}

fn _Py_TryIncref(op: *mut _object) -> i32 {
    if Py_REFCNT!(op) > 0 {
        Py_INCREF!(op);
        return 1;
    }
    0
}

// Fast inlined version of PyType_IS_GC()
fn _PyWeakref_GET_REF(ref_obj: *mut _object) -> *mut _object {
    let r#ref: *mut _PyWeakReference = _Py_CAST!(*mut _PyWeakReference, ref_obj);
    let obj: *mut _object = unsafe { (*r#ref).wr_object };
    if obj == unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } {
        // clear_weakref() was called
        return core::ptr::null_mut::<_object>();
    }
    if _Py_TryIncref(obj) != 0 {
        return obj;
    }
    core::ptr::null_mut::<_object>()
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _dlcontext {
    global: *mut _xid_lookup_state,
    local: *mut _xid_lookup_state,
}

fn xid_lookup_init(state: *mut _xid_lookup_state) {
    _xidregistry_init(unsafe { core::ptr::addr_of_mut!((*state).registry) });
}

fn xid_lookup_fini(state: *mut _xid_lookup_state) {
    _xidregistry_fini(unsafe { core::ptr::addr_of_mut!((*state).registry) });
}

fn get_lookup_context(tstate: *mut _ts, res: &mut _dlcontext) -> i32 {
    let mut __slate_compound_0: _dlcontext = unsafe { core::mem::zeroed() };
    let global: *mut __SlateRecord_pycore_crossinterp_h_255 =
        _PyXI_GET_GLOBAL_STATE!(unsafe { (*tstate).interp });
    if global.is_null() {
        return 1i32.wrapping_neg();
    }
    let local: *mut __SlateRecord_pycore_crossinterp_h_268 =
        _PyXI_GET_STATE!(unsafe { (*tstate).interp });
    if local.is_null() {
        return 1i32.wrapping_neg();
    }
    unsafe {
        *(res as *mut _dlcontext) = *{
            __slate_compound_0 = _dlcontext {
                global: core::ptr::addr_of_mut!((*global).data_lookup),
                local: core::ptr::addr_of_mut!((*local).data_lookup),
            };
            core::ptr::addr_of_mut!(__slate_compound_0)
        };
    }
    0
}

fn lookup_getdata(ctx: &_dlcontext, obj: *mut _object) -> __SlateRecord_pycore_crossinterp_h_150 {
    // Cross-interpreter objects are looked up by exact match on the class.
    // We can reassess this policy when we move from a global registry to a
    // tp_* slot.
    return _lookup_getdata_from_registry(ctx, obj);
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyXIData_GetNotShareableErrorType(tstate: *mut _ts) -> *mut _object {
    let exctype: *mut _object = get_notshareableerror_type(tstate);
    exctype
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyXIData_SetNotShareableError(tstate: *mut _ts, msg: *const i8) {
    let cause: *mut _object = core::ptr::null_mut::<_object>();
    set_notshareableerror(tstate, cause, 1i32, msg);
}

#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn _PyXIData_FormatNotShareableError(
    tstate: *mut _ts,
    format: *const i8,
    mut __va_args: ...
) {
    let cause: *mut _object = core::ptr::null_mut::<_object>();
    let mut vargs: core::ffi::VaList<'static> = unsafe { core::mem::zeroed() };
    vargs = unsafe {
        core::mem::transmute::<core::ffi::VaList<'_>, core::ffi::VaList<'static>>(__va_args.clone())
    };
    format_notshareableerror_v(tstate, cause, 1i32, format, vargs.clone());
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyXI_UnwrapNotShareableError(
    tstate: *mut _ts,
    failure: *mut xi_failure,
) -> i32 {
    let exctype: *mut _object = get_notshareableerror_type(tstate);
    if (unsafe { _PyErr_ExceptionMatches(tstate, exctype) }) == 0 {
        return 1i32.wrapping_neg();
    }
    let mut exc: *mut _object = unsafe { _PyErr_GetRaisedException(tstate) };
    if !failure.is_null() {
        let code: i32 = -9;
        if _PyXI_InitFailure(failure, code, exc) < 0 {
            return 1i32.wrapping_neg();
        }
    }
    let cause: *mut _object = unsafe { PyException_GetCause(exc) };
    if !cause.is_null() {
        Py_DECREF!(exc);
        exc = cause;
    }
    unsafe { _PyErr_SetRaisedException(tstate, exc) };
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyXIData_Lookup(
    tstate: *mut _ts,
    obj: *mut _object,
) -> __SlateRecord_pycore_crossinterp_h_150 {
    let mut __slate_compound_0: __SlateRecord_pycore_crossinterp_h_150 =
        unsafe { core::mem::zeroed() };
    let mut ctx: _dlcontext = unsafe { core::mem::zeroed() };
    if get_lookup_context(tstate, unsafe { &mut (*core::ptr::addr_of_mut!(ctx)) }) < 0 {
        return unsafe {
            *{
                __slate_compound_0 = {
                    let mut __t0: __SlateRecord_pycore_crossinterp_h_150 = core::mem::zeroed();
                    __t0.basic = None;
                    __t0
                };
                core::ptr::addr_of_mut!(__slate_compound_0)
            }
        };
    }
    return lookup_getdata(unsafe { &(*core::ptr::addr_of_mut!(ctx)) }, obj);
}

fn _xidregistry_init(registry: *mut __SlateRecord_pycore_crossinterp_data_registry_h_28) {
    if (unsafe { (*registry).initialized }) != 0 {
        return;
    }
    unsafe {
        (*registry).initialized = 1;
    }
    if (unsafe { (*registry).global }) != 0 {
        // Registering the builtins is cheap so we don't bother doing it lazily.
        _register_builtins_for_crossinterpreter_data(registry);
    }
}

fn _xidregistry_fini(registry: *mut __SlateRecord_pycore_crossinterp_data_registry_h_28) {
    if (unsafe { (*registry).initialized }) == 0 {
        return;
    }
    unsafe {
        (*registry).initialized = 0;
    }
    _xidregistry_clear(registry);
}

fn _xidregistry_lock(registry: *mut __SlateRecord_pycore_crossinterp_data_registry_h_28) {
    if (unsafe { (*registry).global }) != 0 {
        _PyMutex_Lock(unsafe { core::ptr::addr_of_mut!((*registry).mutex) });
    }
    // else: Within an interpreter we rely on the GIL instead of a separate lock.
}

fn _xidregistry_unlock(registry: *mut __SlateRecord_pycore_crossinterp_data_registry_h_28) {
    if (unsafe { (*registry).global }) != 0 {
        _PyMutex_Unlock(unsafe { core::ptr::addr_of_mut!((*registry).mutex) });
    }
}

fn _get_xidregistry_for_type(
    ctx: &_dlcontext,
    cls: *mut _typeobject,
) -> *mut __SlateRecord_pycore_crossinterp_data_registry_h_28 {
    if (unsafe { (*cls).tp_flags }) & 512u64 != 0 {
        return unsafe { core::ptr::addr_of_mut!((*(*(ctx as *const _dlcontext)).local).registry) };
    }
    unsafe { core::ptr::addr_of_mut!((*(*(ctx as *const _dlcontext)).global).registry) }
}

fn _xidregistry_find_type(
    xidregistry: *mut __SlateRecord_pycore_crossinterp_data_registry_h_28,
    cls: *mut _typeobject,
) -> *mut _xid_regitem {
    let mut cur: *mut _xid_regitem = unsafe { (*xidregistry).head };
    while !cur.is_null() {
        '__slate_continue_0: {
            if !unsafe { (*cur).weakref }.is_null() {
                // cur is/was a heap type.
                let registered: *mut _object = _PyWeakref_GET_REF(unsafe { (*cur).weakref });
                if registered.is_null() {
                    // The weakly ref'ed object was freed.
                    cur = _xidregistry_remove_entry(xidregistry, cur);
                    break '__slate_continue_0;
                }
                Py_DECREF!(registered);
            }
            if (unsafe { (*cur).cls }) == cls {
                return cur;
            }
            cur = unsafe { (*cur).next };
        }
    }
    core::ptr::null_mut::<_xid_regitem>()
}

fn _lookup_getdata_from_registry(
    ctx: &_dlcontext,
    obj: *mut _object,
) -> __SlateRecord_pycore_crossinterp_h_150 {
    let mut __slate_compound_0: __SlateRecord_pycore_crossinterp_h_150 =
        unsafe { core::mem::zeroed() };
    let cls: *mut _typeobject = Py_TYPE!(obj);
    let xidregistry: *mut __SlateRecord_pycore_crossinterp_data_registry_h_28 =
        _get_xidregistry_for_type(ctx, cls);
    _xidregistry_lock(xidregistry);
    let matched: *mut _xid_regitem = _xidregistry_find_type(xidregistry, cls);
    let getdata: __SlateRecord_pycore_crossinterp_h_150 = if !matched.is_null() {
        unsafe { (*matched).getdata }
    } else {
        unsafe {
            *{
                __slate_compound_0 = {
                    let mut __t0: __SlateRecord_pycore_crossinterp_h_150 = core::mem::zeroed();
                    __t0.basic = None;
                    __t0
                };
                core::ptr::addr_of_mut!(__slate_compound_0)
            }
        }
    };
    _xidregistry_unlock(xidregistry);
    return getdata;
}

fn _xidregistry_add_type(
    xidregistry: *mut __SlateRecord_pycore_crossinterp_data_registry_h_28,
    cls: *mut _typeobject,
    getdata: __SlateRecord_pycore_crossinterp_h_150,
) -> i32 {
    let mut __slate_compound_0: _xid_regitem = unsafe { core::mem::zeroed() };
    let newhead: *mut _xid_regitem =
        (unsafe { PyMem_RawMalloc(core::mem::size_of::<_xid_regitem>() as u64) })
            as *mut _xid_regitem;
    if newhead.is_null() {
        return 1i32.wrapping_neg();
    }
    unsafe {
        *newhead = *{
            __slate_compound_0 = {
                let mut __t0: _xid_regitem = core::mem::zeroed();
                __t0.cls = cls;
                __t0.refcount = 1u64;
                __t0.getdata = getdata;
                __t0
            };
            core::ptr::addr_of_mut!(__slate_compound_0)
        };
    }
    // We do not keep a reference, to avoid keeping the class alive.
    if (unsafe { (*cls).tp_flags }) & 512u64 != 0 {
        // XXX Assign a callback to clear the entry from the registry?
        unsafe {
            (*newhead).weakref =
                PyWeakref_NewRef(cls as *mut _object, core::ptr::null_mut::<_object>());
        }
        if unsafe { (*newhead).weakref }.is_null() {
            unsafe { PyMem_RawFree(newhead as *mut ()) };
            return 1i32.wrapping_neg();
        }
    }
    unsafe {
        (*newhead).next = (*xidregistry).head;
    }
    if !unsafe { (*newhead).next }.is_null() {
        unsafe {
            (*(*newhead).next).prev = newhead;
        }
    }
    unsafe {
        (*xidregistry).head = newhead;
    }
    0
}

fn _xidregistry_remove_entry(
    xidregistry: *mut __SlateRecord_pycore_crossinterp_data_registry_h_28,
    entry: *mut _xid_regitem,
) -> *mut _xid_regitem {
    let next: *mut _xid_regitem = unsafe { (*entry).next };
    if !unsafe { (*entry).prev }.is_null() {
        unsafe {
            (*(*entry).prev).next = next;
        }
    } else {
        unsafe {
            (*xidregistry).head = next;
        }
    }
    if !next.is_null() {
        unsafe {
            (*next).prev = (*entry).prev;
        }
    }
    Py_XDECREF!(unsafe { (*entry).weakref });
    unsafe { PyMem_RawFree(entry as *mut ()) };
    next
}

fn _xidregistry_clear(xidregistry: *mut __SlateRecord_pycore_crossinterp_data_registry_h_28) {
    let mut cur: *mut _xid_regitem = unsafe { (*xidregistry).head };
    unsafe {
        (*xidregistry).head = core::ptr::null_mut::<_xid_regitem>();
    }
    while !cur.is_null() {
        let next: *mut _xid_regitem = unsafe { (*cur).next };
        Py_XDECREF!(unsafe { (*cur).weakref });
        unsafe { PyMem_RawFree(cur as *mut ()) };
        cur = next;
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyXIData_RegisterClass(
    tstate: *mut _ts,
    cls: *mut _typeobject,
    getdata: __SlateRecord_pycore_crossinterp_h_150,
) -> i32 {
    let mut matched: *mut _xid_regitem = unsafe { core::mem::zeroed() };
    let mut xidregistry: *mut __SlateRecord_pycore_crossinterp_data_registry_h_28 =
        unsafe { core::mem::zeroed() };
    let mut ctx: core::mem::MaybeUninit<_dlcontext> = core::mem::MaybeUninit::uninit();
    let __slate_slot_5: *mut _dlcontext = core::ptr::addr_of_mut!(ctx) as *mut _dlcontext;
    let mut res: i32 = unsafe { core::mem::zeroed() };
    unsafe {
        if PyType_Check__2!(cls) == 0 {
            PyErr_Format(PyExc_ValueError, c"only classes may be registered".as_ptr());
            return 1i32.wrapping_neg();
        } else {
            if getdata.basic == None && getdata.fallback == None {
                PyErr_Format(PyExc_ValueError, c"missing 'getdata' func".as_ptr());
                return 1i32.wrapping_neg();
            } else {
                res = 0;
                if get_lookup_context(tstate, &mut (*core::ptr::addr_of_mut!(*__slate_slot_5))) < 0
                {
                    return 1i32.wrapping_neg();
                } else {
                    xidregistry = _get_xidregistry_for_type(
                        &(*core::ptr::addr_of_mut!(*__slate_slot_5)),
                        cls,
                    );
                    _xidregistry_lock(xidregistry);
                    matched = _xidregistry_find_type(xidregistry, cls);
                    if !matched.is_null() {
                        let __v1: u64 = (*matched).refcount;
                        (*matched).refcount = __v1.wrapping_add(1);
                    } else {
                        res = _xidregistry_add_type(xidregistry, cls, getdata);
                    }
                    _xidregistry_unlock(xidregistry);
                    return res;
                }
            }
        }
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyXIData_UnregisterClass(tstate: *mut _ts, cls: *mut _typeobject) -> i32 {
    let mut res: i32 = 0;
    let mut ctx: _dlcontext = unsafe { core::mem::zeroed() };
    if get_lookup_context(tstate, unsafe { &mut (*core::ptr::addr_of_mut!(ctx)) }) < 0 {
        return 1i32.wrapping_neg();
    }
    let xidregistry: *mut __SlateRecord_pycore_crossinterp_data_registry_h_28 =
        _get_xidregistry_for_type(unsafe { &(*core::ptr::addr_of_mut!(ctx)) }, cls);
    _xidregistry_lock(xidregistry);
    let matched: *mut _xid_regitem = _xidregistry_find_type(xidregistry, cls);
    if !matched.is_null() {
        let __v1: u64 = unsafe { (*matched).refcount };
        unsafe {
            (*matched).refcount = __v1.wrapping_sub(1);
        }
        if (unsafe { (*matched).refcount }) == 0 {
            _xidregistry_remove_entry(xidregistry, matched);
        }
        res = 1;
    }
    _xidregistry_unlock(xidregistry);
    res
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyBytes_GetData(
    obj: *mut _object,
    data: *mut __SlateRecord_pycore_crossinterp_h_180,
) -> i32 {
    let mut __slate_compound_0: __SlateRecord_pycore_crossinterp_h_180 =
        unsafe { core::mem::zeroed() };
    if PyBytes_Check!(obj) == 0 {
        unsafe { PyErr_Format(PyExc_TypeError, c"expected bytes, got %R".as_ptr(), obj) };
        return 1i32.wrapping_neg();
    }
    let mut bytes: *mut i8 = unsafe { core::mem::zeroed() };
    let mut len: i64 = 0;
    if (unsafe {
        PyBytes_AsStringAndSize(
            obj,
            core::ptr::addr_of_mut!(bytes),
            core::ptr::addr_of_mut!(len),
        )
    }) < 0
    {
        return 1i32.wrapping_neg();
    }
    unsafe {
        *data = *{
            __slate_compound_0 = __SlateRecord_pycore_crossinterp_h_180 {
                bytes: bytes as *const i8,
                len: len,
            };
            core::ptr::addr_of_mut!(__slate_compound_0)
        };
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyBytes_FromData(
    data: *mut __SlateRecord_pycore_crossinterp_h_180,
) -> *mut _object {
    unsafe { PyBytes_FromStringAndSize((*data).bytes, (*data).len) }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.crossinterp._PyBytes_FromXIData")]
extern "C-unwind" fn _PyBytes_FromXIData(xidata: *mut _xidata) -> *mut _object {
    let data: *mut __SlateRecord_pycore_crossinterp_h_180 =
        (unsafe { (*xidata).data }) as *mut __SlateRecord_pycore_crossinterp_h_180;
    _PyBytes_FromData(data)
}

fn _bytes_shared(
    tstate: *mut _ts,
    obj: *mut _object,
    size: u64,
    newfunc: Option<unsafe extern "C-unwind" fn(*mut _xidata) -> *mut _object>,
    xidata: *mut _xidata,
) -> i32 {
    if _PyXIData_InitWithSize(xidata, unsafe { (*tstate).interp }, size, obj, newfunc) < 0 {
        return 1i32.wrapping_neg();
    }
    let data: *mut __SlateRecord_pycore_crossinterp_h_180 =
        (unsafe { (*xidata).data }) as *mut __SlateRecord_pycore_crossinterp_h_180;
    if _PyBytes_GetData(obj, data) < 0 {
        _PyXIData_Clear(unsafe { (*tstate).interp }, xidata);
        return 1i32.wrapping_neg();
    }
    0
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.crossinterp._PyBytes_GetXIData")]
extern "C-unwind" fn _PyBytes_GetXIData(
    tstate: *mut _ts,
    obj: *mut _object,
    xidata: *mut _xidata,
) -> i32 {
    if PyBytes_Check!(obj) == 0 {
        unsafe { PyErr_Format(PyExc_TypeError, c"expected bytes, got %R".as_ptr(), obj) };
        return 1i32.wrapping_neg();
    }
    let size: u64 = core::mem::size_of::<__SlateRecord_pycore_crossinterp_h_180>() as u64;
    _bytes_shared(tstate, obj, size, Some(_PyBytes_FromXIData), xidata)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyBytes_GetXIDataWrapped(
    tstate: *mut _ts,
    obj: *mut _object,
    size: u64,
    mut newfunc: Option<unsafe extern "C-unwind" fn(*mut _xidata) -> *mut _object>,
    xidata: *mut _xidata,
) -> *mut __SlateRecord_pycore_crossinterp_h_180 {
    if PyBytes_Check!(obj) == 0 {
        unsafe { PyErr_Format(PyExc_TypeError, c"expected bytes, got %R".as_ptr(), obj) };
        return core::ptr::null_mut::<__SlateRecord_pycore_crossinterp_h_180>();
    }
    if size < (core::mem::size_of::<__SlateRecord_pycore_crossinterp_h_180>() as u64) {
        unsafe {
            PyErr_Format(
                PyExc_ValueError,
                c"expected size >= %zu, got %zu".as_ptr(),
                core::mem::size_of::<__SlateRecord_pycore_crossinterp_h_180>() as u64,
                size,
            )
        };
        return core::ptr::null_mut::<__SlateRecord_pycore_crossinterp_h_180>();
    }
    if newfunc == None {
        if size == (core::mem::size_of::<__SlateRecord_pycore_crossinterp_h_180>() as u64) {
            unsafe { PyErr_SetString(PyExc_ValueError, c"missing new_object func".as_ptr()) };
            return core::ptr::null_mut::<__SlateRecord_pycore_crossinterp_h_180>();
        }
        newfunc = Some(_PyBytes_FromXIData);
    }
    if _bytes_shared(tstate, obj, size, newfunc, xidata) < 0 {
        return core::ptr::null_mut::<__SlateRecord_pycore_crossinterp_h_180>();
    }
    (unsafe { (*xidata).data }) as *mut __SlateRecord_pycore_crossinterp_h_180
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _shared_str_data {
    kind: i32,
    buffer: *const (),
    len: i64,
}

#[unsafe(link_section = ".text.slate_distinct.crossinterp._new_str_object")]
extern "C-unwind" fn _new_str_object(xidata: *mut _xidata) -> *mut _object {
    let shared: *mut _shared_str_data = (unsafe { (*xidata).data }) as *mut _shared_str_data;
    unsafe { PyUnicode_FromKindAndData((*shared).kind, (*shared).buffer, (*shared).len) }
}

#[unsafe(link_section = ".text.slate_distinct.crossinterp._str_shared")]
extern "C-unwind" fn _str_shared(tstate: *mut _ts, obj: *mut _object, xidata: *mut _xidata) -> i32 {
    if _PyXIData_InitWithSize(
        xidata,
        unsafe { (*tstate).interp },
        core::mem::size_of::<_shared_str_data>() as u64,
        obj,
        Some(_new_str_object),
    ) < 0
    {
        return 1i32.wrapping_neg();
    }
    let shared: *mut _shared_str_data = (unsafe { (*xidata).data }) as *mut _shared_str_data;
    unsafe {
        (*shared).kind = PyUnicode_KIND!(obj);
    }
    unsafe {
        (*shared).buffer = PyUnicode_DATA!(obj) as *const ();
    }
    unsafe {
        (*shared).len = PyUnicode_GET_LENGTH!(obj);
    }
    0
}

#[unsafe(link_section = ".text.slate_distinct.crossinterp._new_long_object")]
extern "C-unwind" fn _new_long_object(xidata: *mut _xidata) -> *mut _object {
    unsafe { PyLong_FromSsize_t((*xidata).data as i64) }
}

#[unsafe(link_section = ".text.slate_distinct.crossinterp._long_shared")]
extern "C-unwind" fn _long_shared(
    tstate: *mut _ts,
    obj: *mut _object,
    xidata: *mut _xidata,
) -> i32 {
    // Note that this means the size of shareable ints is bounded by
    // sys.maxsize.  Hence on 32-bit architectures that is half the
    // size of maximum shareable ints on 64-bit.
    let value: i64 = unsafe { PyLong_AsSsize_t(obj) };
    let __v0: bool =
        value == (1i32.wrapping_neg() as i64) && !unsafe { PyErr_Occurred() }.is_null();
    if __v0 {
        if (unsafe { PyErr_ExceptionMatches(PyExc_OverflowError) }) != 0 {
            unsafe { PyErr_SetString(PyExc_OverflowError, c"try sending as bytes".as_ptr()) };
        }
        return 1i32.wrapping_neg();
    }
    _PyXIData_Init(
        xidata,
        unsafe { (*tstate).interp },
        value as *mut (),
        core::ptr::null_mut::<_object>(),
        Some(_new_long_object),
    );
    // xidata->obj and xidata->free remain NULL
    0
}

#[unsafe(link_section = ".text.slate_distinct.crossinterp._new_float_object")]
extern "C-unwind" fn _new_float_object(xidata: *mut _xidata) -> *mut _object {
    let value_ptr: *mut f64 = (unsafe { (*xidata).data }) as *mut f64;
    unsafe { PyFloat_FromDouble(*value_ptr) }
}

#[unsafe(link_section = ".text.slate_distinct.crossinterp._float_shared")]
extern "C-unwind" fn _float_shared(
    tstate: *mut _ts,
    obj: *mut _object,
    xidata: *mut _xidata,
) -> i32 {
    if _PyXIData_InitWithSize(
        xidata,
        unsafe { (*tstate).interp },
        core::mem::size_of::<f64>() as u64,
        core::ptr::null_mut::<_object>(),
        Some(_new_float_object),
    ) < 0
    {
        return 1i32.wrapping_neg();
    }
    let shared: *mut f64 = (unsafe { (*xidata).data }) as *mut f64;
    unsafe {
        *shared = PyFloat_AsDouble(obj);
    }
    0
}

#[unsafe(link_section = ".text.slate_distinct.crossinterp._new_none_object")]
extern "C-unwind" fn _new_none_object(xidata: *mut _xidata) -> *mut _object {
    // XXX Singleton refcounts are problematic across interpreters...
    Py_NewRef!(unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) })
}

#[unsafe(link_section = ".text.slate_distinct.crossinterp._none_shared")]
extern "C-unwind" fn _none_shared(
    tstate: *mut _ts,
    obj: *mut _object,
    xidata: *mut _xidata,
) -> i32 {
    _PyXIData_Init(
        xidata,
        unsafe { (*tstate).interp },
        core::ptr::null_mut::<()>(),
        core::ptr::null_mut::<_object>(),
        Some(_new_none_object),
    );
    // xidata->data, xidata->obj and xidata->free remain NULL
    0
}

#[unsafe(link_section = ".text.slate_distinct.crossinterp._new_bool_object")]
extern "C-unwind" fn _new_bool_object(xidata: *mut _xidata) -> *mut _object {
    if !unsafe { (*xidata).data }.is_null() {
        return _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_TrueStruct) });
    }
    _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_FalseStruct) })
}

#[unsafe(link_section = ".text.slate_distinct.crossinterp._bool_shared")]
extern "C-unwind" fn _bool_shared(
    tstate: *mut _ts,
    obj: *mut _object,
    xidata: *mut _xidata,
) -> i32 {
    _PyXIData_Init(
        xidata,
        unsafe { (*tstate).interp },
        (if Py_IsTrue!(obj) { 1u64 } else { 0u64 }) as *mut (),
        core::ptr::null_mut::<_object>(),
        Some(_new_bool_object),
    );
    // xidata->obj and xidata->free remain NULL
    0
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _shared_tuple_data {
    len: i64,
    items: *mut *mut _xidata,
}

#[unsafe(link_section = ".text.slate_distinct.crossinterp._new_tuple_object")]
extern "C-unwind" fn _new_tuple_object(xidata: *mut _xidata) -> *mut _object {
    let shared: *mut _shared_tuple_data = (unsafe { (*xidata).data }) as *mut _shared_tuple_data;
    let tuple: *mut _object = unsafe { PyTuple_New((*shared).len) };
    if tuple.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let mut i: i64 = 0;
    while i < unsafe { (*shared).len } {
        let item: *mut _object =
            _PyXIData_NewObject(unsafe { *unsafe { (*shared).items }.wrapping_offset(i as isize) });
        if item.is_null() {
            Py_DECREF!(tuple);
            return core::ptr::null_mut::<_object>();
        }
        PyTuple_SET_ITEM__2!(tuple, i, item);
        i = i.wrapping_add(1);
    }
    tuple
}

#[unsafe(link_section = ".text.slate_distinct.crossinterp._tuple_shared_free")]
extern "C-unwind" fn _tuple_shared_free(data: *mut ()) {
    let shared: *mut _shared_tuple_data = data as *mut _shared_tuple_data;
    let mut i: i64 = 0;
    while i < unsafe { (*shared).len } {
        if !unsafe { *unsafe { (*shared).items }.wrapping_offset(i as isize) }.is_null() {
            _PyXIData_Release(unsafe { *unsafe { (*shared).items }.wrapping_offset(i as isize) });
            unsafe {
                PyMem_RawFree(*unsafe { (*shared).items }.wrapping_offset(i as isize) as *mut ())
            };
            unsafe {
                *unsafe { (*shared).items }.wrapping_offset(i as isize) =
                    core::ptr::null_mut::<_xidata>();
            }
        }
        i = i.wrapping_add(1);
    }
    unsafe { PyMem_Free((*shared).items as *mut ()) };
    unsafe { PyMem_RawFree(shared as *mut ()) };
}

#[unsafe(link_section = ".text.slate_distinct.crossinterp._tuple_shared")]
extern "C-unwind" fn _tuple_shared(
    tstate: *mut _ts,
    obj: *mut _object,
    fallback: i32,
    xidata: *mut _xidata,
) -> i32 {
    let mut res: i32 = unsafe { core::mem::zeroed() };
    let mut item: *mut _object = unsafe { core::mem::zeroed() };
    let mut xidata_i: *mut _xidata = unsafe { core::mem::zeroed() };
    let mut i: i64 = unsafe { core::mem::zeroed() };
    let mut shared: *mut _shared_tuple_data = unsafe { core::mem::zeroed() };
    let mut len: i64 = unsafe { core::mem::zeroed() };
    unsafe {
        len = PyTuple_GET_SIZE!(obj);
        if len < 0 {
            return 1i32.wrapping_neg();
        } else {
            shared = PyMem_RawMalloc(core::mem::size_of::<_shared_tuple_data>() as u64)
                as *mut _shared_tuple_data;
            if shared.is_null() {
                PyErr_NoMemory();
                return 1i32.wrapping_neg();
            } else {
                (*shared).len = len;
                (*shared).items = PyMem_Calloc(
                    (*shared).len as u64,
                    core::mem::size_of::<*mut _xidata>() as u64,
                ) as *mut *mut _xidata;
                if unsafe { (*shared).items }.is_null() {
                    PyErr_NoMemory();
                    return 1i32.wrapping_neg();
                } else {
                    i = 0;
                    '__join_0: {
                        '__join_4: {
                            while i < (*shared).len {
                                xidata_i = _PyXIData_New();
                                if xidata_i.is_null() {
                                    break '__join_0;
                                } else {
                                    item = PyTuple_GET_ITEM!(obj, i);
                                    res = 1i32.wrapping_neg();
                                    if _Py_EnterRecursiveCallTstate(
                                        tstate,
                                        c" while sharing a tuple".as_ptr(),
                                    ) == 0
                                    {
                                        res = _PyObject_GetXIData(tstate, item, fallback, xidata_i);
                                        _Py_LeaveRecursiveCallTstate(tstate);
                                    }
                                    if res < 0 {
                                        break '__join_4;
                                    } else {
                                        *unsafe { (*shared).items }.wrapping_offset(i as isize) =
                                            xidata_i;
                                        i = i.wrapping_add(1);
                                    }
                                }
                            }
                            _PyXIData_Init(
                                xidata,
                                (*tstate).interp,
                                shared as *mut (),
                                obj,
                                Some(_new_tuple_object),
                            );
                            _PyXIData_SET_FREE!(xidata, Some(_tuple_shared_free));
                            return 0;
                        }
                        PyMem_RawFree(xidata_i as *mut ());
                    }
                    _tuple_shared_free(shared as *mut ());
                    return 1i32.wrapping_neg();
                }
            }
        }
    }
    // PyErr_NoMemory already set
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.crossinterp._PyCode_FromXIData")]
extern "C-unwind" fn _PyCode_FromXIData(xidata: *mut _xidata) -> *mut _object {
    _PyMarshal_ReadObjectFromXIData(xidata)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyCode_GetXIData(
    tstate: *mut _ts,
    obj: *mut _object,
    xidata: *mut _xidata,
) -> i32 {
    if PyCode_Check!(obj) == 0 {
        unsafe {
            _PyXIData_FormatNotShareableError(tstate, c"expected code, got %R".as_ptr(), obj)
        };
        return 1i32.wrapping_neg();
    }
    if _PyMarshal_GetXIData(tstate, obj, xidata) < 0 {
        return 1i32.wrapping_neg();
    }
    _PyXIData_SET_NEW_OBJECT!(xidata, Some(_PyCode_FromXIData));
    0
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.crossinterp._PyFunction_FromXIData")]
extern "C-unwind" fn _PyFunction_FromXIData(xidata: *mut _xidata) -> *mut _object {
    // For now "stateless" functions are the only ones we must accommodate.
    let code: *mut _object = _PyMarshal_ReadObjectFromXIData(xidata);
    if code.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    // Create a new function.
    //
    // For stateless functions (no globals) we use __main__ as __globals__,
    //
    // just like we do for builtins like exec().
    let tstate: *mut _ts = _PyThreadState_GET();
    let mut globals: *mut _object = unsafe { _PyEval_GetGlobalsFromRunningMain(tstate) }; // borrowed
    if globals.is_null() {
        if !_PyErr_Occurred(tstate).is_null() {
            Py_DECREF!(code);
            return core::ptr::null_mut::<_object>();
        }
        globals = unsafe { PyDict_New() };
        if globals.is_null() {
            Py_DECREF!(code);
            return core::ptr::null_mut::<_object>();
        }
    } else {
        Py_INCREF!(globals);
    }
    if (unsafe { _PyEval_EnsureBuiltins(tstate, globals, core::ptr::null_mut::<*mut _object>()) })
        < 0
    {
        Py_DECREF!(code);
        Py_DECREF!(globals);
        return core::ptr::null_mut::<_object>();
    }
    let func: *mut _object = unsafe { PyFunction_New(code, globals) };
    Py_DECREF!(code);
    Py_DECREF!(globals);
    func
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyFunction_GetXIData(
    tstate: *mut _ts,
    func: *mut _object,
    xidata: *mut _xidata,
) -> i32 {
    if PyFunction_Check!(func) == 0 {
        let msg: *const i8 = c"expected a function, got %R".as_ptr();
        unsafe {
            format_notshareableerror(tstate, core::ptr::null_mut::<_object>(), 0i32, msg, func)
        };
        return 1i32.wrapping_neg();
    }
    if (unsafe { _PyFunction_VerifyStateless(tstate, func) }) < 0 {
        let cause: *mut _object = unsafe { _PyErr_GetRaisedException(tstate) };
        let msg: *const i8 = c"only stateless functions are shareable".as_ptr();
        set_notshareableerror(tstate, cause, 0i32, msg);
        Py_DECREF!(cause);
        return 1i32.wrapping_neg();
    }
    let code: *mut _object = PyFunction_GET_CODE!(func);
    // Ideally code objects would be immortal and directly shareable.
    //
    // In the meantime, we use marshal.
    if _PyMarshal_GetXIData(tstate, code, xidata) < 0 {
        return 1i32.wrapping_neg();
    }
    // Replace _PyMarshal_ReadObjectFromXIData.
    //
    // (_PyFunction_FromXIData() will call it.)
    _PyXIData_SET_NEW_OBJECT!(xidata, Some(_PyFunction_FromXIData));
    0
}

fn _register_builtins_for_crossinterpreter_data(
    xidregistry: *mut __SlateRecord_pycore_crossinterp_data_registry_h_28,
) {
    let mut __slate_compound_0: __SlateRecord_pycore_crossinterp_h_150 =
        unsafe { core::mem::zeroed() };
    let mut __slate_compound_1: __SlateRecord_pycore_crossinterp_h_150 =
        unsafe { core::mem::zeroed() };
    let mut __slate_compound_2: __SlateRecord_pycore_crossinterp_h_150 =
        unsafe { core::mem::zeroed() };
    let mut __slate_compound_3: __SlateRecord_pycore_crossinterp_h_150 =
        unsafe { core::mem::zeroed() };
    let mut __slate_compound_4: __SlateRecord_pycore_crossinterp_h_150 =
        unsafe { core::mem::zeroed() };
    let mut __slate_compound_5: __SlateRecord_pycore_crossinterp_h_150 =
        unsafe { core::mem::zeroed() };
    let mut __slate_compound_6: __SlateRecord_pycore_crossinterp_h_150 =
        unsafe { core::mem::zeroed() };
    // None
    if _xidregistry_add_type(
        xidregistry,
        Py_TYPE!(unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }),
        unsafe {
            *{
                __slate_compound_0 = {
                    let mut __t0: __SlateRecord_pycore_crossinterp_h_150 = core::mem::zeroed();
                    __t0.basic = Some(_none_shared);
                    __t0
                };
                core::ptr::addr_of_mut!(__slate_compound_0)
            }
        },
    ) != 0
    {
        Py_FatalError__2!(
            c"could not register None for cross-interpreter sharing".as_ptr() as *mut i8
        );
    }
    // int
    if _xidregistry_add_type(
        xidregistry,
        unsafe { core::ptr::addr_of_mut!(PyLong_Type) },
        unsafe {
            *{
                __slate_compound_1 = {
                    let mut __t1: __SlateRecord_pycore_crossinterp_h_150 = core::mem::zeroed();
                    __t1.basic = Some(_long_shared);
                    __t1
                };
                core::ptr::addr_of_mut!(__slate_compound_1)
            }
        },
    ) != 0
    {
        Py_FatalError__2!(
            c"could not register int for cross-interpreter sharing".as_ptr() as *mut i8
        );
    }
    // bytes
    if _xidregistry_add_type(
        xidregistry,
        unsafe { core::ptr::addr_of_mut!(PyBytes_Type) },
        unsafe {
            *{
                __slate_compound_2 = {
                    let mut __t2: __SlateRecord_pycore_crossinterp_h_150 = core::mem::zeroed();
                    __t2.basic = Some(_PyBytes_GetXIData);
                    __t2
                };
                core::ptr::addr_of_mut!(__slate_compound_2)
            }
        },
    ) != 0
    {
        Py_FatalError__2!(
            c"could not register bytes for cross-interpreter sharing".as_ptr() as *mut i8
        );
    }
    // str
    if _xidregistry_add_type(
        xidregistry,
        unsafe { core::ptr::addr_of_mut!(PyUnicode_Type) },
        unsafe {
            *{
                __slate_compound_3 = {
                    let mut __t3: __SlateRecord_pycore_crossinterp_h_150 = core::mem::zeroed();
                    __t3.basic = Some(_str_shared);
                    __t3
                };
                core::ptr::addr_of_mut!(__slate_compound_3)
            }
        },
    ) != 0
    {
        Py_FatalError__2!(
            c"could not register str for cross-interpreter sharing".as_ptr() as *mut i8
        );
    }
    // bool
    if _xidregistry_add_type(
        xidregistry,
        unsafe { core::ptr::addr_of_mut!(PyBool_Type) },
        unsafe {
            *{
                __slate_compound_4 = {
                    let mut __t4: __SlateRecord_pycore_crossinterp_h_150 = core::mem::zeroed();
                    __t4.basic = Some(_bool_shared);
                    __t4
                };
                core::ptr::addr_of_mut!(__slate_compound_4)
            }
        },
    ) != 0
    {
        Py_FatalError__2!(
            c"could not register bool for cross-interpreter sharing".as_ptr() as *mut i8
        );
    }
    // float
    if _xidregistry_add_type(
        xidregistry,
        unsafe { core::ptr::addr_of_mut!(PyFloat_Type) },
        unsafe {
            *{
                __slate_compound_5 = {
                    let mut __t5: __SlateRecord_pycore_crossinterp_h_150 = core::mem::zeroed();
                    __t5.basic = Some(_float_shared);
                    __t5
                };
                core::ptr::addr_of_mut!(__slate_compound_5)
            }
        },
    ) != 0
    {
        Py_FatalError__2!(
            c"could not register float for cross-interpreter sharing".as_ptr() as *mut i8
        );
    }
    // tuple
    if _xidregistry_add_type(
        xidregistry,
        unsafe { core::ptr::addr_of_mut!(PyTuple_Type) },
        unsafe {
            *{
                __slate_compound_6 = {
                    let mut __t6: __SlateRecord_pycore_crossinterp_h_150 = core::mem::zeroed();
                    __t6.fallback = Some(_tuple_shared);
                    __t6
                };
                core::ptr::addr_of_mut!(__slate_compound_6)
            }
        },
    ) != 0
    {
        Py_FatalError__2!(
            c"could not register tuple for cross-interpreter sharing".as_ptr() as *mut i8
        );
    }
    // For now, we do not register PyCode_Type or PyFunction_Type.
}

// cross-interpreter data
// registry of {type -> _PyXIData_getdata_t}
// For now we use a global registry of shareable classes.
// An alternative would be to add a tp_* slot for a class's
// _PyXIData_getdata_t.  It would be simpler and more efficient.
// lifecycle
#[unsafe(no_mangle)]
extern "C-unwind" fn _PyXIData_New() -> *mut _xidata {
    let xid: *mut _xidata =
        (unsafe { PyMem_RawCalloc(1u64, core::mem::size_of::<_xidata>() as u64) }) as *mut _xidata;
    if xid.is_null() {
        unsafe { PyErr_NoMemory() };
    }
    xid
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyXIData_Free(xid: *mut _xidata) {
    let interp: *mut _is = unsafe { PyInterpreterState_Get() };
    _PyXIData_Clear(interp, xid);
    unsafe { PyMem_RawFree(xid as *mut ()) };
}

// defining cross-interpreter data
fn _xidata_init(xidata: *mut _xidata) {
    let mut __slate_compound_0: _xidata = unsafe { core::mem::zeroed() };
    // If the value is being reused
    //
    // then _xidata_clear() should have been called already.
    unsafe {
        *xidata = *{
            __slate_compound_0 = {
                let mut __t0: _xidata = core::mem::zeroed();
                __t0.data = core::ptr::null_mut::<()>();
                __t0
            };
            core::ptr::addr_of_mut!(__slate_compound_0)
        };
    }
    unsafe {
        (*xidata).interpid = 1i32.wrapping_neg() as i64;
    }
}

fn _xidata_clear(xidata: *mut _xidata) {
    // _PyXIData_t only has two members that need to be
    //
    // cleaned up, if set: "xidata" must be freed and "obj" must be decref'ed.
    //
    // In both cases the original (owning) interpreter must be used,
    //
    // which is the caller's responsibility to ensure.
    if !unsafe { (*xidata).data }.is_null() {
        if (unsafe { (*xidata).free }) != None {
            unsafe { unsafe { (*xidata).free }.unwrap()((*xidata).data) };
        }
        unsafe {
            (*xidata).data = core::ptr::null_mut::<()>();
        }
    }
    Py_CLEAR!((*xidata).obj);
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyXIData_Init(
    xidata: *mut _xidata,
    interp: *mut _is,
    shared: *mut (),
    obj: *mut _object,
    new_object: Option<unsafe extern "C-unwind" fn(*mut _xidata) -> *mut _object>,
) {
    _xidata_init(xidata);
    unsafe {
        (*xidata).data = shared;
    }
    if !obj.is_null() {
        // released in _PyXIData_Clear()
        unsafe {
            (*xidata).obj = Py_NewRef!(obj);
        }
    }
    // Ideally every object would know its owning interpreter.
    //
    // Until then, we have to rely on the caller to identify it
    //
    // (but we don't need it in all cases).
    let __v0: i64 = if !interp.is_null() {
        unsafe { PyInterpreterState_GetID(interp) }
    } else {
        1i32.wrapping_neg() as i64
    };
    unsafe {
        (*xidata).interpid = __v0;
    }
    unsafe {
        (*xidata).new_object = new_object;
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyXIData_InitWithSize(
    xidata: *mut _xidata,
    interp: *mut _is,
    size: u64,
    obj: *mut _object,
    new_object: Option<unsafe extern "C-unwind" fn(*mut _xidata) -> *mut _object>,
) -> i32 {
    // For now we always free the shared data in the same interpreter
    //
    // where it was allocated, so the interpreter is required.
    _PyXIData_Init(xidata, interp, core::ptr::null_mut::<()>(), obj, new_object);
    unsafe {
        (*xidata).data = PyMem_RawCalloc(1u64, size);
    }
    if unsafe { (*xidata).data }.is_null() {
        return 1i32.wrapping_neg();
    }
    unsafe {
        (*xidata).free = unsafe {
            core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
                PyMem_RawFree as *const (),
            )
        };
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyXIData_Clear(interp: *mut _is, xidata: *mut _xidata) {
    // This must be called in the owning interpreter.
    _xidata_clear(xidata);
}

// getting cross-interpreter data
fn _set_xid_lookup_failure(
    tstate: *mut _ts,
    obj: *mut _object,
    mut msg: *const i8,
    cause: *mut _object,
) {
    if !msg.is_null() {
        set_notshareableerror(tstate, cause, 0i32, msg);
    } else {
        if obj.is_null() {
            msg = c"object does not support cross-interpreter data".as_ptr();
            set_notshareableerror(tstate, cause, 0i32, msg);
        } else {
            msg = c"%R does not support cross-interpreter data".as_ptr();
            unsafe { format_notshareableerror(tstate, cause, 0i32, msg, obj) };
        }
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyObject_CheckXIData(tstate: *mut _ts, obj: *mut _object) -> i32 {
    let mut ctx: _dlcontext = unsafe { core::mem::zeroed() };
    if get_lookup_context(tstate, unsafe { &mut (*core::ptr::addr_of_mut!(ctx)) }) < 0 {
        return 1i32.wrapping_neg();
    }
    let getdata: __SlateRecord_pycore_crossinterp_h_150 =
        lookup_getdata(unsafe { &(*core::ptr::addr_of_mut!(ctx)) }, obj);
    if getdata.basic == None && getdata.fallback == None {
        if _PyErr_Occurred(tstate).is_null() {
            _set_xid_lookup_failure(
                tstate,
                obj,
                core::ptr::null::<i8>(),
                core::ptr::null_mut::<_object>(),
            );
        }
        return 1i32.wrapping_neg();
    }
    0
}

fn _check_xidata(tstate: *mut _ts, xidata: *mut _xidata) -> i32 {
    // xidata->data can be anything, including NULL, so we don't check it.
    // xidata->obj may be NULL, so we don't check it.
    if _PyXIData_INTERPID!(xidata) < 0 {
        unsafe { PyErr_SetString(PyExc_SystemError, c"missing interp".as_ptr()) };
        return 1i32.wrapping_neg();
    }
    if (unsafe { (*xidata).new_object }) == None {
        unsafe { PyErr_SetString(PyExc_SystemError, c"missing new_object func".as_ptr()) };
        return 1i32.wrapping_neg();
    }
    // xidata->free may be NULL, so we don't check it.
    0
}

fn _get_xidata(tstate: *mut _ts, obj: *mut _object, fallback: i32, xidata: *mut _xidata) -> i32 {
    let interp: *mut _is = unsafe { (*tstate).interp };
    if !unsafe { (*xidata).data }.is_null() || !unsafe { (*xidata).obj }.is_null() {
        unsafe { _PyErr_SetString(tstate, PyExc_ValueError, c"xidata not cleared".as_ptr()) };
        return 1i32.wrapping_neg();
    }
    // Call the "getdata" func for the object.
    let mut ctx: _dlcontext = unsafe { core::mem::zeroed() };
    if get_lookup_context(tstate, unsafe { &mut (*core::ptr::addr_of_mut!(ctx)) }) < 0 {
        return 1i32.wrapping_neg();
    }
    Py_INCREF!(obj);
    let mut getdata: __SlateRecord_pycore_crossinterp_h_150 =
        lookup_getdata(unsafe { &(*core::ptr::addr_of_mut!(ctx)) }, obj);
    if getdata.basic == None && getdata.fallback == None {
        if !unsafe { PyErr_Occurred() }.is_null() {
            Py_DECREF!(obj);
            return 1i32.wrapping_neg();
        }
        // Fall back to obj
        Py_DECREF!(obj);
        if _PyErr_Occurred(tstate).is_null() {
            _set_xid_lookup_failure(
                tstate,
                obj,
                core::ptr::null::<i8>(),
                core::ptr::null_mut::<_object>(),
            );
        }
        return 1i32.wrapping_neg();
    }
    let mut res: i32 = 0;
    let __v0: i32 = if getdata.basic != None {
        unsafe { getdata.basic.unwrap()(tstate, obj, xidata) }
    } else {
        unsafe { getdata.fallback.unwrap()(tstate, obj, fallback, xidata) }
    };
    res = __v0;
    Py_DECREF!(obj);
    if res != 0 {
        let cause: *mut _object = unsafe { _PyErr_GetRaisedException(tstate) };
        _set_xid_lookup_failure(tstate, obj, core::ptr::null::<i8>(), cause);
        Py_XDECREF!(cause);
        return 1i32.wrapping_neg();
    }
    // Fill in the blanks and validate the result.
    unsafe {
        (*xidata).interpid = PyInterpreterState_GetID(interp);
    }
    if _check_xidata(tstate, xidata) != 0 {
        _PyXIData_Release(xidata);
        return 1i32.wrapping_neg();
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyObject_GetXIDataNoFallback(
    tstate: *mut _ts,
    obj: *mut _object,
    xidata: *mut _xidata,
) -> i32 {
    _get_xidata(tstate, obj, 0i32, xidata)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyObject_GetXIData(
    tstate: *mut _ts,
    obj: *mut _object,
    fallback: i32,
    xidata: *mut _xidata,
) -> i32 {
    match fallback {
        0 => {
            return _get_xidata(tstate, obj, fallback, xidata);
        }
        1 => {
            if _get_xidata(tstate, obj, fallback, xidata) == 0 {
                return 0;
            }
            let exc: *mut _object = unsafe { _PyErr_GetRaisedException(tstate) };
            if PyFunction_Check!(obj) != 0 {
                if _PyFunction_GetXIData(tstate, obj, xidata) == 0 {
                    Py_DECREF!(exc);
                    return 0;
                }
                unsafe { _PyErr_Clear(tstate) };
            }
            // We could try _PyMarshal_GetXIData() but we won't for now.
            if _PyPickle_GetXIData(tstate, obj, xidata) == 0 {
                Py_DECREF!(exc);
                return 0;
            }
            // Raise the original exception.
            unsafe { _PyErr_SetRaisedException(tstate, exc) };
            return 1i32.wrapping_neg();
        }
        _ => {
            unsafe {
                _PyErr_SetString(
                    tstate,
                    PyExc_SystemError,
                    c"unsupported xidata fallback option".as_ptr(),
                )
            };
            return 1i32.wrapping_neg();
        }
    }
    unsafe { core::mem::zeroed() }
}

// pickle C-API
#[repr(C)]
#[derive(Clone, Copy)]
struct _pickle_context {
    tstate: *mut _ts,
}

fn _PyPickle_Dumps(ctx: &_pickle_context, obj: *mut _object) -> *mut _object {
    let dumps: *mut _object =
        unsafe { PyImport_ImportModuleAttrString(c"pickle".as_ptr(), c"dumps".as_ptr()) };
    if dumps.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let bytes: *mut _object = unsafe { PyObject_CallOneArg(dumps, obj) };
    Py_DECREF!(dumps);
    bytes
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _unpickle_context {
    tstate: *mut _ts,
    /// We only special-case the __main__ module,
    ///
    /// since other modules behave consistently.
    main: sync_module,
}

fn _unpickle_context_clear(ctx: &mut _unpickle_context) {
    sync_module_clear(unsafe { core::ptr::addr_of_mut!((*(ctx as *mut _unpickle_context)).main) });
}

fn check_missing___main___attr(exc: *mut _object) -> i32 {
    if (unsafe { PyErr_GivenExceptionMatches(exc, PyExc_AttributeError) }) == 0 {
        return 0;
    }
    // Get the error message.
    let args: *mut _object = unsafe { PyException_GetArgs(exc) };
    let __v0: bool = args.is_null()
        || args == unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
        || (unsafe { PyObject_Size(args) }) < 1;
    if __v0 {
        Py_XDECREF!(args);
        return 0;
    }
    let mut msgobj: *mut _object = args;
    if PyUnicode_Check!(msgobj) == 0 {
        msgobj = unsafe { PySequence_GetItem(args, 0i64) };
        Py_DECREF!(args);
        if msgobj.is_null() {
            unsafe { PyErr_Clear() };
            return 0;
        }
    }
    let err: *const i8 = unsafe { PyUnicode_AsUTF8(msgobj) };
    // Check if it's a missing __main__ attr.
    let cmp: i32 = unsafe {
        strncmp(
            err as *const core::ffi::c_char,
            c"module '__main__' has no attribute '".as_ptr() as *const core::ffi::c_char,
            36u64 as usize,
        ) as i32
    };
    Py_DECREF!(msgobj);
    (cmp == 0) as i32
}

fn _PyPickle_Loads(ctx: *mut _unpickle_context, pickled: *mut _object) -> *mut _object {
    // Make an initial attempt to unpickle.
    let mut obj: *mut _object = unsafe { core::mem::zeroed() };
    let mut loads: *mut _object = unsafe { core::mem::zeroed() };
    let mut exc: *mut _object = unsafe { core::mem::zeroed() };
    let mut tstate: *mut _ts = unsafe { core::mem::zeroed() };
    unsafe {
        tstate = (*ctx).tstate;
        exc = core::ptr::null_mut::<_object>();
        loads = PyImport_ImportModuleAttrString(c"pickle".as_ptr(), c"loads".as_ptr());
        if loads.is_null() {
            return core::ptr::null_mut::<_object>();
        } else {
            obj = PyObject_CallOneArg(loads, pickled);
            if obj.is_null() {
                if !ctx.is_null() {
                    exc = _PyErr_GetRaisedException(tstate);
                    if check_missing___main___attr(exc) != 0 {
                        // Temporarily swap in a fake __main__ loaded from the original
                        //
                        // file and cached.  Note that functions will use the cached ns
                        //
                        // for __globals__, // not the actual module.
                        if ensure_isolated_main(tstate, core::ptr::addr_of_mut!((*ctx).main)) >= 0 {
                            if apply_isolated_main(tstate, core::ptr::addr_of_mut!((*ctx).main))
                                >= 0
                            {
                                // Try to unpickle once more.
                                obj = PyObject_CallOneArg(loads, pickled);
                                restore_main(tstate, core::ptr::addr_of_mut!((*ctx).main));
                                if !obj.is_null() {
                                    Py_CLEAR__2!(exc);
                                }
                            }
                        }
                    }
                }
            }
            if !exc.is_null() {
                if !_PyErr_Occurred(tstate).is_null() {
                    sync_module_capture_exc(tstate, core::ptr::addr_of_mut!((*ctx).main));
                }
                // We restore the original exception.
                //
                // It might make sense to chain it (__context__).
                _PyErr_SetRaisedException(tstate, exc);
            }
            Py_DECREF!(loads);
            return obj;
        }
    }
}

// pickle wrapper
#[repr(C)]
#[derive(Clone, Copy)]
struct _pickle_xid_context {
    /// __main__.__file__
    mainfile: __SlateRecord_crossinterp_c_694,
}

fn _set_pickle_xid_context(tstate: *mut _ts, ctx: *mut _pickle_xid_context) -> i32 {
    // Set mainfile if possible.
    let len: i64 = _Py_GetMainfile(
        unsafe { (*ctx).mainfile._utf8.as_mut_ptr() as *mut i8 },
        4096u64,
    );
    if len < 0 {
        // For now we ignore any exceptions.
        unsafe { PyErr_Clear() };
    } else {
        if len > 0 {
            unsafe {
                (*ctx).mainfile.utf8 = ((*ctx).mainfile._utf8.as_mut_ptr() as *mut i8) as *const i8;
            }
            unsafe {
                (*ctx).mainfile.len = len as u64;
            }
        }
    }
    0
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _shared_pickle_data {
    /// Must be first if we use _PyBytes_FromXIData().
    pickled: __SlateRecord_pycore_crossinterp_h_180,
    ctx: _pickle_xid_context,
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.crossinterp._PyPickle_LoadFromXIData")]
extern "C-unwind" fn _PyPickle_LoadFromXIData(xidata: *mut _xidata) -> *mut _object {
    let tstate: *mut _ts = _PyThreadState_GET();
    let shared: *mut _shared_pickle_data = (unsafe { (*xidata).data }) as *mut _shared_pickle_data;
    // We avoid copying the pickled data by wrapping it in a memoryview.
    //
    // The alternative is to get a bytes object using _PyBytes_FromXIData().
    let pickled: *mut _object = unsafe {
        PyMemoryView_FromMemory(
            (*shared).pickled.bytes as *mut i8,
            (*shared).pickled.len,
            256i32,
        )
    };
    if pickled.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    // Unpickle the object.
    let mut ctx: _unpickle_context = _unpickle_context {
        tstate: tstate,
        main: {
            let mut __t0: sync_module = unsafe { core::mem::zeroed() };
            __t0.filename = unsafe { (*shared).ctx.mainfile.utf8 };
            __t0
        },
    };
    let obj: *mut _object = _PyPickle_Loads(core::ptr::addr_of_mut!(ctx), pickled);
    Py_DECREF!(pickled);
    _unpickle_context_clear(unsafe { &mut (*core::ptr::addr_of_mut!(ctx)) });
    if obj.is_null() {
        let cause: *mut _object = unsafe { _PyErr_GetRaisedException(tstate) };
        _set_xid_lookup_failure(
            tstate,
            core::ptr::null_mut::<_object>(),
            c"object could not be unpickled".as_ptr(),
            cause,
        );
        Py_DECREF!(cause);
    }
    obj
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyPickle_GetXIData(
    tstate: *mut _ts,
    obj: *mut _object,
    xidata: *mut _xidata,
) -> i32 {
    // Pickle the object.
    let mut ctx: _pickle_context = _pickle_context { tstate: tstate };
    let bytes: *mut _object = _PyPickle_Dumps(unsafe { &(*core::ptr::addr_of_mut!(ctx)) }, obj);
    if bytes.is_null() {
        let cause: *mut _object = unsafe { _PyErr_GetRaisedException(tstate) };
        _set_xid_lookup_failure(
            tstate,
            core::ptr::null_mut::<_object>(),
            c"object could not be pickled".as_ptr(),
            cause,
        );
        Py_DECREF!(cause);
        return 1i32.wrapping_neg();
    }
    // If we had an "unwrapper" mechnanism, we could call
    //
    // _PyObject_GetXIData() on the bytes object directly and add
    //
    // a simple unwrapper to call pickle.loads() on the bytes.
    let size: u64 = core::mem::size_of::<_shared_pickle_data>() as u64;
    let shared: *mut _shared_pickle_data =
        _PyBytes_GetXIDataWrapped(tstate, bytes, size, Some(_PyPickle_LoadFromXIData), xidata)
            as *mut _shared_pickle_data;
    Py_DECREF!(bytes);
    if shared.is_null() {
        return 1i32.wrapping_neg();
    }
    // If it mattered, we could skip getting __main__.__file__
    //
    // when "__main__" doesn't show up in the pickle bytes.
    if _set_pickle_xid_context(tstate, unsafe { core::ptr::addr_of_mut!((*shared).ctx) }) < 0 {
        _xidata_clear(xidata);
        return 1i32.wrapping_neg();
    }
    0
}

// marshal wrapper
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.crossinterp._PyMarshal_ReadObjectFromXIData")]
extern "C-unwind" fn _PyMarshal_ReadObjectFromXIData(xidata: *mut _xidata) -> *mut _object {
    let tstate: *mut _ts = _PyThreadState_GET();
    let shared: *mut __SlateRecord_pycore_crossinterp_h_180 =
        (unsafe { (*xidata).data }) as *mut __SlateRecord_pycore_crossinterp_h_180;
    let obj: *mut _object =
        unsafe { PyMarshal_ReadObjectFromString((*shared).bytes, (*shared).len) };
    if obj.is_null() {
        let cause: *mut _object = unsafe { _PyErr_GetRaisedException(tstate) };
        _set_xid_lookup_failure(
            tstate,
            core::ptr::null_mut::<_object>(),
            c"object could not be unmarshalled".as_ptr(),
            cause,
        );
        Py_DECREF!(cause);
        return core::ptr::null_mut::<_object>();
    }
    obj
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyMarshal_GetXIData(
    tstate: *mut _ts,
    obj: *mut _object,
    xidata: *mut _xidata,
) -> i32 {
    let bytes: *mut _object = unsafe { PyMarshal_WriteObjectToString(obj, 5i32) };
    if bytes.is_null() {
        let cause: *mut _object = unsafe { _PyErr_GetRaisedException(tstate) };
        _set_xid_lookup_failure(
            tstate,
            core::ptr::null_mut::<_object>(),
            c"object could not be marshalled".as_ptr(),
            cause,
        );
        Py_DECREF!(cause);
        return 1i32.wrapping_neg();
    }
    let size: u64 = core::mem::size_of::<__SlateRecord_pycore_crossinterp_h_180>() as u64;
    let shared: *mut __SlateRecord_pycore_crossinterp_h_180 = _PyBytes_GetXIDataWrapped(
        tstate,
        bytes,
        size,
        Some(_PyMarshal_ReadObjectFromXIData),
        xidata,
    );
    Py_DECREF!(bytes);
    if shared.is_null() {
        return 1i32.wrapping_neg();
    }
    0
}

// script wrapper
fn verify_script(tstate: *mut _ts, co: *mut PyCodeObject, checked: i32, pure: i32) -> i32 {
    // Make sure it isn't a closure and (optionally) doesn't use globals.
    let mut builtins: *mut _object = core::ptr::null_mut::<_object>();
    if pure != 0 {
        builtins = unsafe { _PyEval_GetBuiltins(tstate) };
    }
    if checked == 0 {
        if (unsafe {
            _PyCode_VerifyStateless(
                tstate,
                co,
                core::ptr::null_mut::<_object>(),
                core::ptr::null_mut::<_object>(),
                builtins,
            )
        }) < 0
        {
            return 1i32.wrapping_neg();
        }
    }
    // Make sure it doesn't have args.
    if (unsafe { (*co).co_argcount }) > 0
        || (unsafe { (*co).co_posonlyargcount }) > 0
        || (unsafe { (*co).co_kwonlyargcount }) > 0
        || (unsafe { (*co).co_flags }) & (4 | 8) != 0
    {
        unsafe {
            _PyErr_SetString(
                tstate,
                PyExc_ValueError,
                c"code with args not supported".as_ptr(),
            )
        };
        return 1i32.wrapping_neg();
    }
    // Make sure it doesn't return anything.
    if (unsafe { _PyCode_ReturnsOnlyNone(co) }) == 0 {
        unsafe {
            _PyErr_SetString(
                tstate,
                PyExc_ValueError,
                c"code that returns a value is not a script".as_ptr(),
            )
        };
        return 1i32.wrapping_neg();
    }
    0
}

fn get_script_xidata(tstate: *mut _ts, obj: *mut _object, pure: i32, xidata: *mut _xidata) -> i32 {
    let mut __slate_compound_9: __SlateRecord_compile_h_30 = unsafe { core::mem::zeroed() };
    let mut script: *const i8 = unsafe { core::mem::zeroed() };
    let mut r#ref: *mut _object = unsafe { core::mem::zeroed() };
    let mut cf: core::mem::MaybeUninit<__SlateRecord_compile_h_30> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_4: *mut __SlateRecord_compile_h_30 =
        core::ptr::addr_of_mut!(cf) as *mut __SlateRecord_compile_h_30;
    let mut optimize: i32 = unsafe { core::mem::zeroed() };
    let mut filename: *const i8 = unsafe { core::mem::zeroed() };
    let mut checked: i32 = unsafe { core::mem::zeroed() };
    // Get the corresponding code object.
    let mut code: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_0: {
            code = core::ptr::null_mut::<_object>();
            checked = 0;
            if PyCode_Check!(obj) != 0 {
                code = obj;
                Py_INCREF!(code);
            } else {
                if PyFunction_Check!(obj) != 0 {
                    code = PyFunction_GET_CODE!(obj);
                    Py_INCREF!(code);
                    if pure != 0 {
                        if _PyFunction_VerifyStateless(tstate, obj) < 0 {
                            break '__join_0;
                        } else {
                            checked = 1;
                        }
                    }
                } else {
                    filename = c"<script>".as_ptr();
                    optimize = 0;
                    core::ptr::write(__slate_slot_4, *{
                        __slate_compound_9 = __SlateRecord_compile_h_30 {
                            cf_flags: 0,
                            cf_feature_version: 14,
                        };
                        core::ptr::addr_of_mut!(__slate_compound_9)
                    });
                    (*__slate_slot_4).cf_flags = 256;
                    r#ref = core::ptr::null_mut::<_object>();
                    script = _Py_SourceAsString(
                        obj,
                        c"???".as_ptr(),
                        c"???".as_ptr(),
                        core::ptr::addr_of_mut!(*__slate_slot_4),
                        core::ptr::addr_of_mut!(r#ref),
                    );
                    if script.is_null() {
                        if _PyObject_SupportedAsScript(obj) == 0 {
                            // We discard the raised exception.
                            _PyErr_Format(
                                tstate,
                                PyExc_TypeError,
                                c"unsupported script %R".as_ptr(),
                                obj,
                            );
                            break '__join_0;
                        } else {
                            break '__join_0;
                        }
                    } else {
                        code = Py_CompileStringExFlags(
                            script,
                            filename,
                            257i32,
                            core::ptr::addr_of_mut!(*__slate_slot_4),
                            optimize,
                        );
                        Py_XDECREF!(r#ref);
                        if code.is_null() {
                            break '__join_0;
                        } else {
                            // Compiled text can't have args or any return statements,
                            //
                            // nor be a closure.  It can use globals though.
                            if pure == 0 {
                                // We don't need to check for globals either.
                                checked = 1;
                            }
                        }
                    }
                }
            }
            // Make sure it's actually a script.
            if verify_script(tstate, code as *mut PyCodeObject, checked, pure) >= 0 {
                // Convert the code object.
                let res: i32 = _PyCode_GetXIData(tstate, code, xidata);
                Py_DECREF!(code);
                if res < 0 {
                    return 1i32.wrapping_neg();
                } else {
                    return 0;
                }
            }
        }
        Py_XDECREF!(code);
        let cause: *mut _object = _PyErr_GetRaisedException(tstate);
        _set_xid_lookup_failure(
            tstate,
            core::ptr::null_mut::<_object>(),
            c"object not a valid script".as_ptr(),
            cause,
        );
        Py_DECREF!(cause);
        return 1i32.wrapping_neg();
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyCode_GetScriptXIData(
    tstate: *mut _ts,
    obj: *mut _object,
    xidata: *mut _xidata,
) -> i32 {
    get_script_xidata(tstate, obj, 0i32, xidata)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyCode_GetPureScriptXIData(
    tstate: *mut _ts,
    obj: *mut _object,
    xidata: *mut _xidata,
) -> i32 {
    get_script_xidata(tstate, obj, 1i32, xidata)
}

// using cross-interpreter data
#[unsafe(no_mangle)]
extern "C-unwind" fn _PyXIData_NewObject(xidata: *mut _xidata) -> *mut _object {
    unsafe { unsafe { (*xidata).new_object }.unwrap()(xidata) }
}

#[unsafe(link_section = ".text.slate_distinct.crossinterp._call_clear_xidata")]
extern "C-unwind" fn _call_clear_xidata(data: *mut ()) -> i32 {
    _xidata_clear(data as *mut _xidata);
    0
}

fn _xidata_release(xidata: *mut _xidata, rawfree: i32) -> i32 {
    if (unsafe { (*xidata).data }.is_null() || (unsafe { (*xidata).free }) == None)
        && unsafe { (*xidata).obj }.is_null()
    {
        // Nothing to release!
        if rawfree != 0 {
            unsafe { PyMem_RawFree(xidata as *mut ()) };
        } else {
            unsafe {
                (*xidata).data = core::ptr::null_mut::<()>();
            }
        }
        return 0;
    }
    // Switch to the original interpreter.
    let interp: *mut _is = unsafe { _PyInterpreterState_LookUpID(_PyXIData_INTERPID!(xidata)) };
    if interp.is_null() {
        // The interpreter was already destroyed.
        //
        // This function shouldn't have been called.
        //
        // XXX Someone leaked some memory...
        if rawfree != 0 {
            unsafe { PyMem_RawFree(xidata as *mut ()) };
        }
        return 1i32.wrapping_neg();
    }
    // "Release" the data and/or the object.
    if rawfree != 0 {
        return _Py_CallInInterpreterAndRawFree(
            interp,
            Some(_call_clear_xidata),
            xidata as *mut (),
        );
    } else {
        return _Py_CallInInterpreter(interp, Some(_call_clear_xidata), xidata as *mut ());
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyXIData_Release(xidata: *mut _xidata) -> i32 {
    _xidata_release(xidata, 0i32)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyXIData_ReleaseAndRawFree(xidata: *mut _xidata) -> i32 {
    _xidata_release(xidata, 1i32)
}

// convenience utilities
fn _copy_string_obj_raw(strobj: *mut _object, p_size: *mut i64) -> *mut i8 {
    let mut size: i64 = 1i32.wrapping_neg() as i64;
    let str: *const i8 = unsafe { PyUnicode_AsUTF8AndSize(strobj, core::ptr::addr_of_mut!(size)) };
    if str.is_null() {
        return core::ptr::null_mut::<i8>();
    }
    if size != ((unsafe { strlen(str as *const core::ffi::c_char) as u64 }) as i64) {
        unsafe { PyErr_SetString(PyExc_ValueError, c"found embedded NULL character".as_ptr()) };
        return core::ptr::null_mut::<i8>();
    }
    let copied: *mut i8 = (unsafe { PyMem_RawMalloc(size.wrapping_add(1) as u64) }) as *mut i8;
    if copied.is_null() {
        unsafe { PyErr_NoMemory() };
        return core::ptr::null_mut::<i8>();
    }
    unsafe {
        strcpy(
            copied as *mut core::ffi::c_char,
            str as *const core::ffi::c_char,
        ) as *mut i8
    };
    if !p_size.is_null() {
        unsafe {
            *p_size = size;
        }
    }
    copied
}

fn _convert_exc_to_TracebackException(exc: *mut _object, p_tbexc: &mut *mut _object) -> i32 {
    let mut tbexc: *mut _object = unsafe { core::mem::zeroed() };
    // This is inspired by _PyErr_Display().
    let mut tbexc_type: *mut _object = unsafe { core::mem::zeroed() };
    let mut create: *mut _object = unsafe { core::mem::zeroed() };
    let mut kwargs: *mut _object = unsafe { core::mem::zeroed() };
    let mut args: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        args = core::ptr::null_mut::<_object>();
        kwargs = core::ptr::null_mut::<_object>();
        create = core::ptr::null_mut::<_object>();
        tbexc_type =
            PyImport_ImportModuleAttrString(c"traceback".as_ptr(), c"TracebackException".as_ptr());
        if tbexc_type.is_null() {
            return 1i32.wrapping_neg();
        } else {
            create = PyObject_GetAttrString(tbexc_type, c"from_exception".as_ptr());
            Py_DECREF!(tbexc_type);
            if create.is_null() {
                return 1i32.wrapping_neg();
            } else {
                args = PyTuple_Pack(1i64, exc);
                if !args.is_null() {
                    kwargs = PyDict_New();
                    if !kwargs.is_null() {
                        if PyDict_SetItemString(
                            kwargs,
                            c"save_exc_type".as_ptr(),
                            _PyObject_CAST__2!(core::ptr::addr_of_mut!(_Py_FalseStruct)),
                        ) >= 0
                        {
                            if PyDict_SetItemString(
                                kwargs,
                                c"lookup_lines".as_ptr(),
                                _PyObject_CAST__2!(core::ptr::addr_of_mut!(_Py_FalseStruct)),
                            ) >= 0
                            {
                                tbexc = PyObject_Call(create, args, kwargs);
                                Py_DECREF!(args);
                                Py_DECREF!(kwargs);
                                Py_DECREF!(create);
                                if !tbexc.is_null() {
                                    *(p_tbexc as *mut *mut _object) = tbexc;
                                    return 0;
                                }
                            }
                        }
                    }
                }
                Py_XDECREF!(args);
                Py_XDECREF!(kwargs);
                Py_XDECREF!(create);
                return 1i32.wrapping_neg();
            }
        }
    }
}

// We accommodate backports here.
fn _format_TracebackException(tbexc: *mut _object) -> *const i8 {
    let lines: *mut _object =
        unsafe { PyObject_CallMethod(tbexc, c"format".as_ptr(), core::ptr::null::<i8>()) };
    if lines.is_null() {
        return core::ptr::null::<i8>();
    }
    let formatted_obj: *mut _object = unsafe {
        PyUnicode_Join(
            core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .literals
                    ._py_empty
                    ._ascii
                    .ob_base
            ),
            lines,
        )
    };
    Py_DECREF!(lines);
    if formatted_obj.is_null() {
        return core::ptr::null::<i8>();
    }
    let mut size: i64 = 1i32.wrapping_neg() as i64;
    let formatted: *mut i8 = _copy_string_obj_raw(formatted_obj, core::ptr::addr_of_mut!(size));
    Py_DECREF!(formatted_obj);
    if formatted.is_null() || size == 0 {
        return formatted as *const i8;
    }
    // Remove a trailing newline if needed.
    if ((unsafe { *formatted.wrapping_offset(size.wrapping_sub(1) as isize) }) as i32) == 10 {
        unsafe {
            *formatted.wrapping_offset(size.wrapping_sub(1) as isize) = 0;
        }
    }
    formatted as *const i8
}

fn _release_xid_data(xidata: *mut _xidata, rawfree: i32) -> i32 {
    let exc: *mut _object = unsafe { PyErr_GetRaisedException() };
    let mut res: i32 = 0;
    let __v0: i32 = if rawfree != 0 {
        _PyXIData_ReleaseAndRawFree(xidata)
    } else {
        _PyXIData_Release(xidata)
    };
    res = __v0;
    if res < 0 {
        // The owning interpreter is already destroyed.
        _PyXIData_Clear(core::ptr::null_mut::<_is>(), xidata);
        // XXX Emit a warning?
        unsafe { PyErr_Clear() };
    }
    unsafe { PyErr_SetRaisedException(exc) };
    res
}

// exception snapshots
fn _excinfo_init_type_from_exception(info: *mut _excinfo_type, exc: *mut _object) -> i32 {
    // Note that this copies directly rather than into an intermediate
    // struct and does not clear on error.  If we need that then we
    // should have a separate function to wrap this one
    // and do all that there.
    let mut strobj: *mut _object = core::ptr::null_mut::<_object>();
    let r#type: *mut _typeobject = Py_TYPE!(exc);
    if (unsafe { (*r#type).tp_flags }) & 2u64 != 0 {
        unsafe {
            (*info).builtin = r#type;
        }
    } else {
        // Only builtin types are preserved.
        unsafe {
            (*info).builtin = core::ptr::null_mut::<_typeobject>();
        }
    }
    // __name__
    strobj = unsafe { PyType_GetName(r#type) };
    if strobj.is_null() {
        return 1i32.wrapping_neg();
    }
    unsafe {
        (*info).name = _copy_string_obj_raw(strobj, core::ptr::null_mut::<i64>()) as *const i8;
    }
    Py_DECREF!(strobj);
    if unsafe { (*info).name }.is_null() {
        return 1i32.wrapping_neg();
    }
    // __qualname__
    strobj = unsafe { PyType_GetQualName(r#type) };
    if strobj.is_null() {
        return 1i32.wrapping_neg();
    }
    unsafe {
        (*info).qualname = _copy_string_obj_raw(strobj, core::ptr::null_mut::<i64>()) as *const i8;
    }
    Py_DECREF!(strobj);
    if unsafe { (*info).qualname }.is_null() {
        return 1i32.wrapping_neg();
    }
    // __module__
    strobj = unsafe { PyType_GetModuleName(r#type) };
    if strobj.is_null() {
        return 1i32.wrapping_neg();
    }
    unsafe {
        (*info).module = _copy_string_obj_raw(strobj, core::ptr::null_mut::<i64>()) as *const i8;
    }
    Py_DECREF!(strobj);
    if unsafe { (*info).module }.is_null() {
        return 1i32.wrapping_neg();
    }
    0
}

fn _excinfo_init_type_from_object(info: *mut _excinfo_type, exctype: *mut _object) -> i32 {
    let mut strobj: *mut _object = core::ptr::null_mut::<_object>();
    // __name__
    strobj = unsafe { PyObject_GetAttrString(exctype, c"__name__".as_ptr()) };
    if strobj.is_null() {
        return 1i32.wrapping_neg();
    }
    unsafe {
        (*info).name = _copy_string_obj_raw(strobj, core::ptr::null_mut::<i64>()) as *const i8;
    }
    Py_DECREF!(strobj);
    if unsafe { (*info).name }.is_null() {
        return 1i32.wrapping_neg();
    }
    // __qualname__
    strobj = unsafe { PyObject_GetAttrString(exctype, c"__qualname__".as_ptr()) };
    if strobj.is_null() {
        return 1i32.wrapping_neg();
    }
    unsafe {
        (*info).qualname = _copy_string_obj_raw(strobj, core::ptr::null_mut::<i64>()) as *const i8;
    }
    Py_DECREF!(strobj);
    if unsafe { (*info).qualname }.is_null() {
        return 1i32.wrapping_neg();
    }
    // __module__
    strobj = unsafe { PyObject_GetAttrString(exctype, c"__module__".as_ptr()) };
    if strobj.is_null() {
        return 1i32.wrapping_neg();
    }
    unsafe {
        (*info).module = _copy_string_obj_raw(strobj, core::ptr::null_mut::<i64>()) as *const i8;
    }
    Py_DECREF!(strobj);
    if unsafe { (*info).module }.is_null() {
        return 1i32.wrapping_neg();
    }
    0
}

fn _excinfo_clear_type(info: *mut _excinfo_type) {
    let mut __slate_compound_0: _excinfo_type = unsafe { core::mem::zeroed() };
    if !unsafe { (*info).builtin }.is_null() {}
    if !unsafe { (*info).name }.is_null() {
        unsafe { PyMem_RawFree((*info).name as *mut ()) };
    }
    if !unsafe { (*info).qualname }.is_null() {
        unsafe { PyMem_RawFree((*info).qualname as *mut ()) };
    }
    if !unsafe { (*info).module }.is_null() {
        unsafe { PyMem_RawFree((*info).module as *mut ()) };
    }
    unsafe {
        *info = *{
            __slate_compound_0 = {
                let mut __t0: _excinfo_type = core::mem::zeroed();
                __t0.builtin = core::ptr::null_mut::<_typeobject>();
                __t0
            };
            core::ptr::addr_of_mut!(__slate_compound_0)
        };
    }
}

fn _excinfo_normalize_type(
    info: *mut _excinfo_type,
    p_module: &mut *const i8,
    p_qualname: &mut *const i8,
) {
    if unsafe { (*info).name }.is_null() {
        // This is inspired by TracebackException.format_exception_only().
        unsafe {
            *(p_module as *mut *const i8) = core::ptr::null::<i8>();
        }
        unsafe {
            *(p_qualname as *mut *const i8) = core::ptr::null::<i8>();
        }
        return;
    }
    let mut module: *const i8 = unsafe { (*info).module };
    let mut qualname: *const i8 = unsafe { (*info).qualname };
    if qualname.is_null() {
        qualname = unsafe { (*info).name };
    }
    if (unsafe {
        strcmp(
            module as *const core::ffi::c_char,
            c"builtins".as_ptr() as *const core::ffi::c_char,
        ) as i32
    }) == 0
    {
        module = core::ptr::null::<i8>();
    } else {
        if (unsafe {
            strcmp(
                module as *const core::ffi::c_char,
                c"__main__".as_ptr() as *const core::ffi::c_char,
            ) as i32
        }) == 0
        {
            module = core::ptr::null::<i8>();
        }
    }
    unsafe {
        *(p_qualname as *mut *const i8) = qualname;
    }
    unsafe {
        *(p_module as *mut *const i8) = module;
    }
}

fn excinfo_is_set(info: *mut _excinfo) -> i32 {
    (!unsafe { (*info).r#type.name }.is_null() || !unsafe { (*info).msg }.is_null()) as i32
}

fn _PyXI_excinfo_clear(info: *mut _excinfo) {
    let mut __slate_compound_0: _excinfo = unsafe { core::mem::zeroed() };
    _excinfo_clear_type(unsafe { core::ptr::addr_of_mut!((*info).r#type) });
    if !unsafe { (*info).msg }.is_null() {
        unsafe { PyMem_RawFree((*info).msg as *mut ()) };
    }
    if !unsafe { (*info).errdisplay }.is_null() {
        unsafe { PyMem_RawFree((*info).errdisplay as *mut ()) };
    }
    unsafe {
        *info = *{
            __slate_compound_0 = {
                let mut __t0: _excinfo = core::mem::zeroed();
                __t0.r#type = {
                    let mut __t1: _excinfo_type = core::mem::zeroed();
                    __t1.builtin = core::ptr::null_mut::<_typeobject>();
                    __t1
                };
                __t0
            };
            core::ptr::addr_of_mut!(__slate_compound_0)
        };
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyXI_excinfo_format(info: *mut _excinfo) -> *mut _object {
    let mut module: *const i8 = unsafe { core::mem::zeroed() };
    let mut qualname: *const i8 = unsafe { core::mem::zeroed() };
    _excinfo_normalize_type(
        unsafe { core::ptr::addr_of_mut!((*info).r#type) },
        unsafe { &mut (*core::ptr::addr_of_mut!(module)) },
        unsafe { &mut (*core::ptr::addr_of_mut!(qualname)) },
    );
    if !qualname.is_null() {
        if !module.is_null() {
            if !unsafe { (*info).msg }.is_null() {
                return unsafe {
                    PyUnicode_FromFormat(c"%s.%s: %s".as_ptr(), module, qualname, (*info).msg)
                };
            } else {
                return unsafe { PyUnicode_FromFormat(c"%s.%s".as_ptr(), module, qualname) };
            }
        } else {
            if !unsafe { (*info).msg }.is_null() {
                return unsafe { PyUnicode_FromFormat(c"%s: %s".as_ptr(), qualname, (*info).msg) };
            } else {
                return unsafe { PyUnicode_FromString(qualname) };
            }
        }
    } else {
        if !unsafe { (*info).msg }.is_null() {
            return unsafe { PyUnicode_FromString((*info).msg) };
        } else {
            return unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) };
        }
    }
}

fn _PyXI_excinfo_InitFromException(info: *mut _excinfo, exc: *mut _object) -> *const i8 {
    // Pickle a traceback.TracebackException.
    let mut tbexc: *mut _object = unsafe { core::mem::zeroed() };
    // Extract the exception message.
    let mut msgobj: *mut _object = unsafe { core::mem::zeroed() };
    let mut failure: *const i8 = unsafe { core::mem::zeroed() };
    unsafe {
        if PyErr_GivenExceptionMatches(exc, PyExc_MemoryError) != 0 {
            _PyXI_excinfo_clear(info);
            return core::ptr::null::<i8>();
        } else {
            failure = core::ptr::null::<i8>();
            if _excinfo_init_type_from_exception(core::ptr::addr_of_mut!((*info).r#type), exc) < 0 {
                failure = c"error while initializing exception type snapshot".as_ptr();
            } else {
                msgobj = PyObject_Str(exc);
                if msgobj.is_null() {
                    failure = c"error while formatting exception".as_ptr();
                } else {
                    (*info).msg =
                        _copy_string_obj_raw(msgobj, core::ptr::null_mut::<i64>()) as *const i8;
                    Py_DECREF!(msgobj);
                    if unsafe { (*info).msg }.is_null() {
                        failure = c"error while copying exception message".as_ptr();
                    } else {
                        tbexc = core::ptr::null_mut::<_object>();
                        if _convert_exc_to_TracebackException(
                            exc,
                            &mut (*core::ptr::addr_of_mut!(tbexc)),
                        ) < 0
                        {
                            PyErr_Clear();
                        } else {
                            (*info).errdisplay = _format_TracebackException(tbexc);
                            Py_DECREF!(tbexc);
                            if unsafe { (*info).errdisplay }.is_null() {
                                PyErr_Clear();
                            }
                        }
                        return core::ptr::null::<i8>();
                    }
                }
            }
            _PyXI_excinfo_clear(info);
            return failure;
        }
    }
}

fn _PyXI_excinfo_InitFromObject(info: *mut _excinfo, obj: *mut _object) -> *const i8 {
    // Pickle a traceback.TracebackException.
    let mut errdisplay: *mut _object = unsafe { core::mem::zeroed() };
    // Extract the exception message.
    let mut msgobj: *mut _object = unsafe { core::mem::zeroed() };
    let mut exctype: *mut _object = unsafe { core::mem::zeroed() };
    let mut failure: *const i8 = unsafe { core::mem::zeroed() };
    unsafe {
        failure = core::ptr::null::<i8>();
        exctype = PyObject_GetAttrString(obj, c"type".as_ptr());
        if exctype.is_null() {
            failure = c"exception snapshot missing 'type' attribute".as_ptr();
        } else {
            let res: i32 =
                _excinfo_init_type_from_object(core::ptr::addr_of_mut!((*info).r#type), exctype);
            Py_DECREF!(exctype);
            if res < 0 {
                failure = c"error while initializing exception type snapshot".as_ptr();
            } else {
                msgobj = PyObject_GetAttrString(obj, c"msg".as_ptr());
                if msgobj.is_null() {
                    failure = c"exception snapshot missing 'msg' attribute".as_ptr();
                } else {
                    (*info).msg =
                        _copy_string_obj_raw(msgobj, core::ptr::null_mut::<i64>()) as *const i8;
                    Py_DECREF!(msgobj);
                    if unsafe { (*info).msg }.is_null() {
                        failure = c"error while copying exception message".as_ptr();
                    } else {
                        errdisplay = PyObject_GetAttrString(obj, c"errdisplay".as_ptr());
                        if errdisplay.is_null() {
                            failure = c"exception snapshot missing 'errdisplay' attribute".as_ptr();
                        } else {
                            (*info).errdisplay =
                                _copy_string_obj_raw(errdisplay, core::ptr::null_mut::<i64>())
                                    as *const i8;
                            Py_DECREF!(errdisplay);
                            if unsafe { (*info).errdisplay }.is_null() {
                                failure = c"error while copying exception error display".as_ptr();
                            } else {
                                return core::ptr::null::<i8>();
                            }
                        }
                    }
                }
            }
        }
        _PyXI_excinfo_clear(info);
        return failure;
    }
}

fn _PyXI_excinfo_Apply(info: *mut _excinfo, exctype: *mut _object) {
    let mut tbexc: *mut _object = core::ptr::null_mut::<_object>();
    if !unsafe { (*info).errdisplay }.is_null() {
        tbexc = unsafe { PyUnicode_FromString((*info).errdisplay) };
        if tbexc.is_null() {
            unsafe { PyErr_Clear() };
        } else {
            unsafe { PyErr_SetObject(exctype, tbexc) };
            Py_DECREF!(tbexc);
            return;
        }
    }
    let formatted: *mut _object = _PyXI_excinfo_format(info);
    unsafe { PyErr_SetObject(exctype, formatted) };
    Py_DECREF!(formatted);
    if !tbexc.is_null() {
        let exc: *mut _object = unsafe { PyErr_GetRaisedException() };
        if (unsafe { PyObject_SetAttrString(exc, c"_errdisplay".as_ptr(), tbexc) }) < 0 {
            unsafe { PyErr_Clear() };
        }
        Py_DECREF!(tbexc);
        unsafe { PyErr_SetRaisedException(exc) };
    }
}

fn _PyXI_excinfo_TypeAsObject(info: *mut _excinfo) -> *mut _object {
    let mut module: *mut _object = unsafe { core::mem::zeroed() };
    let mut qualname: *mut _object = unsafe { core::mem::zeroed() };
    let mut name: *mut _object = unsafe { core::mem::zeroed() };
    let mut empty: i32 = unsafe { core::mem::zeroed() };
    let mut ns: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        ns = _PyNamespace_New(core::ptr::null_mut::<_object>());
        if ns.is_null() {
            return core::ptr::null_mut::<_object>();
        } else {
            '__join_0: {
                empty = 1;
                if !unsafe { (*info).r#type.name }.is_null() {
                    name = PyUnicode_FromString((*info).r#type.name);
                    if name.is_null() {
                        break '__join_0;
                    } else {
                        let res: i32 = PyObject_SetAttrString(ns, c"__name__".as_ptr(), name);
                        Py_DECREF!(name);
                        if res < 0 {
                            break '__join_0;
                        } else {
                            empty = 0;
                        }
                    }
                }
                if !unsafe { (*info).r#type.qualname }.is_null() {
                    qualname = PyUnicode_FromString((*info).r#type.qualname);
                    if qualname.is_null() {
                        break '__join_0;
                    } else {
                        let res: i32 =
                            PyObject_SetAttrString(ns, c"__qualname__".as_ptr(), qualname);
                        Py_DECREF!(qualname);
                        if res < 0 {
                            break '__join_0;
                        } else {
                            empty = 0;
                        }
                    }
                }
                if !unsafe { (*info).r#type.module }.is_null() {
                    module = PyUnicode_FromString((*info).r#type.module);
                    if module.is_null() {
                        break '__join_0;
                    } else {
                        let res: i32 = PyObject_SetAttrString(ns, c"__module__".as_ptr(), module);
                        Py_DECREF!(module);
                        if res < 0 {
                            break '__join_0;
                        } else {
                            empty = 0;
                        }
                    }
                }
                if empty != 0 {
                    Py_CLEAR__2!(ns);
                }
                return ns;
            }
            Py_DECREF!(ns);
            return core::ptr::null_mut::<_object>();
        }
    }
}

fn _PyXI_excinfo_AsObject(info: *mut _excinfo) -> *mut _object {
    let mut tbexc: *mut _object = unsafe { core::mem::zeroed() };
    let mut formatted: *mut _object = unsafe { core::mem::zeroed() };
    let mut __v2: *mut _object = unsafe { core::mem::zeroed() };
    let mut msg: *mut _object = unsafe { core::mem::zeroed() };
    let mut r#type: *mut _object = unsafe { core::mem::zeroed() };
    let mut res: i32 = unsafe { core::mem::zeroed() };
    let mut ns: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        ns = _PyNamespace_New(core::ptr::null_mut::<_object>());
        if ns.is_null() {
            return core::ptr::null_mut::<_object>();
        } else {
            '__join_0: {
                r#type = _PyXI_excinfo_TypeAsObject(info);
                if r#type.is_null() {
                    if !PyErr_Occurred().is_null() {
                        break '__join_0;
                    } else {
                        r#type = Py_NewRef!(core::ptr::addr_of_mut!(_Py_NoneStruct));
                    }
                }
                res = PyObject_SetAttrString(ns, c"type".as_ptr(), r#type);
                Py_DECREF!(r#type);
                if res >= 0 {
                    if !unsafe { (*info).msg }.is_null() {
                        __v2 = PyUnicode_FromString((*info).msg);
                    } else {
                        __v2 = Py_NewRef!(core::ptr::addr_of_mut!(_Py_NoneStruct));
                    }
                    msg = __v2;
                    if !msg.is_null() {
                        res = PyObject_SetAttrString(ns, c"msg".as_ptr(), msg);
                        Py_DECREF!(msg);
                        if res >= 0 {
                            formatted = _PyXI_excinfo_format(info);
                            if !formatted.is_null() {
                                res = PyObject_SetAttrString(ns, c"formatted".as_ptr(), formatted);
                                Py_DECREF!(formatted);
                                if res >= 0 {
                                    if !unsafe { (*info).errdisplay }.is_null() {
                                        tbexc = PyUnicode_FromString((*info).errdisplay);
                                        if tbexc.is_null() {
                                            PyErr_Clear();
                                        } else {
                                            res = PyObject_SetAttrString(
                                                ns,
                                                c"errdisplay".as_ptr(),
                                                tbexc,
                                            );
                                            Py_DECREF!(tbexc);
                                            if res < 0 {
                                                break '__join_0;
                                            }
                                        }
                                    }
                                    return ns;
                                }
                            }
                        }
                    }
                }
            }
            Py_DECREF!(ns);
            return core::ptr::null_mut::<_object>();
        }
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyXI_NewExcInfo(exc: *mut _object) -> *mut _excinfo {
    if exc.is_null() || exc == unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } {
        unsafe { PyErr_SetString(PyExc_ValueError, c"missing exc".as_ptr()) };
        return core::ptr::null_mut::<_excinfo>();
    }
    let info: *mut _excinfo =
        (unsafe { PyMem_RawCalloc(1u64, core::mem::size_of::<_excinfo>() as u64) })
            as *mut _excinfo;
    if info.is_null() {
        unsafe { PyErr_NoMemory() };
        return core::ptr::null_mut::<_excinfo>();
    }
    let mut failure: *const i8 = unsafe { core::mem::zeroed() };
    let __v0: bool = PyExceptionInstance_Check!(exc) != 0 || PyExceptionClass_Check!(exc);
    if __v0 {
        failure = _PyXI_excinfo_InitFromException(info, exc);
    } else {
        failure = _PyXI_excinfo_InitFromObject(info, exc);
    }
    if !failure.is_null() {
        unsafe { PyMem_RawFree(info as *mut ()) };
        set_exc_with_cause(unsafe { PyExc_Exception }, failure);
        return core::ptr::null_mut::<_excinfo>();
    }
    info
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyXI_FreeExcInfo(info: *mut _excinfo) {
    _PyXI_excinfo_clear(info);
    unsafe { PyMem_RawFree(info as *mut ()) };
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyXI_FormatExcInfo(info: *mut _excinfo) -> *mut _object {
    _PyXI_excinfo_format(info)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyXI_ExcInfoAsObject(info: *mut _excinfo) -> *mut _object {
    _PyXI_excinfo_AsObject(info)
}

// short-term data sharing
// error codes
fn _PyXI_ApplyErrorCode(code: i32, interp: *mut _is) -> i32 {
    let tstate: *mut _ts = _PyThreadState_GET();
    match code as i32 {
        -2 => {
            unsafe { PyErr_SetNone(PyExc_InterpreterError) }; // XXX msg?
        }
        -3 => {
            unsafe { PyErr_NoMemory() };
        }
        -4 => {
            unsafe { _PyErr_SetInterpreterAlreadyRunning() };
        }
        -5 => {
            unsafe {
                PyErr_SetString(
                    PyExc_InterpreterError,
                    c"failed to get __main__ namespace".as_ptr(),
                )
            };
        }
        -6 => {
            unsafe {
                PyErr_SetString(
                    PyExc_InterpreterError,
                    c"failed to apply namespace to __main__".as_ptr(),
                )
            };
        }
        -7 => {
            unsafe {
                PyErr_SetString(
                    PyExc_InterpreterError,
                    c"failed to preserve objects across session".as_ptr(),
                )
            };
        }
        -8 => {
            unsafe {
                PyErr_SetString(
                    PyExc_InterpreterError,
                    c"failed to transfer exception between interpreters".as_ptr(),
                )
            };
        }
        -9 => {
            _set_xid_lookup_failure(
                tstate,
                core::ptr::null_mut::<_object>(),
                core::ptr::null::<i8>(),
                core::ptr::null_mut::<_object>(),
            );
        }
        _ => {
            unsafe {
                PyErr_Format(
                    PyExc_RuntimeError,
                    c"unsupported error code %d".as_ptr(),
                    code as i32,
                )
            };
        }
    }
    1i32.wrapping_neg()
}

// basic failure info
/// _PyXI_failure
#[repr(C)]
#[derive(Clone, Copy)]
struct xi_failure {
    /// The kind of error to propagate.
    code: i32,
    /// The propagated message.
    msg: *const i8,
    msg_owned: i32,
}

fn clear_xi_failure(failure: *mut xi_failure) {
    let mut __slate_compound_0: xi_failure = unsafe { core::mem::zeroed() };
    if !unsafe { (*failure).msg }.is_null() && (unsafe { (*failure).msg_owned }) != 0 {
        unsafe { PyMem_RawFree((*failure).msg as *mut ()) };
    }
    unsafe {
        *failure = *{
            __slate_compound_0 = {
                let mut __t0: xi_failure = core::mem::zeroed();
                __t0.code = 0i32;
                __t0
            };
            core::ptr::addr_of_mut!(__slate_compound_0)
        };
    }
}

fn copy_xi_failure(dest: *mut xi_failure, src: *mut xi_failure) {
    unsafe {
        *dest = *src;
    }
    unsafe {
        (*dest).msg_owned = 0;
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyXI_NewFailure() -> *mut xi_failure {
    let mut __slate_compound_0: xi_failure = unsafe { core::mem::zeroed() };
    let failure: *mut xi_failure =
        (unsafe { PyMem_RawMalloc(core::mem::size_of::<xi_failure>() as u64) }) as *mut xi_failure;
    if failure.is_null() {
        unsafe { PyErr_NoMemory() };
        return core::ptr::null_mut::<xi_failure>();
    }
    unsafe {
        *failure = *{
            __slate_compound_0 = {
                let mut __t0: xi_failure = core::mem::zeroed();
                __t0.code = 0i32;
                __t0
            };
            core::ptr::addr_of_mut!(__slate_compound_0)
        };
    }
    failure
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyXI_FreeFailure(failure: *mut xi_failure) {
    clear_xi_failure(failure);
    unsafe { PyMem_RawFree(failure as *mut ()) };
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyXI_GetFailureCode(failure: *mut xi_failure) -> i32 {
    if failure.is_null() {
        return 0;
    }
    unsafe { (*failure).code }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyXI_InitFailureUTF8(failure: *mut xi_failure, code: i32, msg: *const i8) {
    let mut __slate_compound_0: xi_failure = unsafe { core::mem::zeroed() };
    unsafe {
        *failure = *{
            __slate_compound_0 = {
                let mut __t0: xi_failure = core::mem::zeroed();
                __t0.code = code;
                __t0.msg = msg;
                __t0.msg_owned = 0i32;
                __t0
            };
            core::ptr::addr_of_mut!(__slate_compound_0)
        };
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyXI_InitFailure(
    failure: *mut xi_failure,
    code: i32,
    obj: *mut _object,
) -> i32 {
    let mut __slate_compound_0: xi_failure = unsafe { core::mem::zeroed() };
    let mut __slate_compound_1: xi_failure = unsafe { core::mem::zeroed() };
    unsafe {
        *failure = *{
            __slate_compound_0 = {
                let mut __t0: xi_failure = core::mem::zeroed();
                __t0.code = code;
                __t0.msg = core::ptr::null::<i8>();
                __t0.msg_owned = 0i32;
                __t0
            };
            core::ptr::addr_of_mut!(__slate_compound_0)
        };
    }
    if obj.is_null() {
        return 0;
    }
    let msgobj: *mut _object = unsafe { PyObject_Str(obj) };
    if msgobj.is_null() {
        return 1i32.wrapping_neg();
    }
    // This will leak if not paired with clear_xi_failure().
    //
    // That happens automatically in _capture_current_exception().
    let msg: *const i8 = _copy_string_obj_raw(msgobj, core::ptr::null_mut::<i64>()) as *const i8;
    Py_DECREF!(msgobj);
    if msg.is_null() {
        return 1i32.wrapping_neg();
    }
    unsafe {
        *failure = *{
            __slate_compound_1 = {
                let mut __t1: xi_failure = core::mem::zeroed();
                __t1.code = code;
                __t1.msg = msg;
                __t1.msg_owned = 1i32;
                __t1
            };
            core::ptr::addr_of_mut!(__slate_compound_1)
        };
    }
    0
}

// shared exceptions
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_crossinterp_c_1856 {
    /// The originating interpreter.
    interp: *mut _is,
    /// The error to propagate, if different from the uncaught exception.
    r#override: *mut xi_failure,
    _override: xi_failure,
    /// The exception information to propagate, if applicable.
    ///
    /// This is populated only for some error codes,
    ///
    /// but always for _PyXI_ERR_UNCAUGHT_EXCEPTION.
    uncaught: _excinfo,
}

fn xi_error_clear(err: &mut __SlateRecord_crossinterp_c_1856) {
    unsafe {
        (*(err as *mut __SlateRecord_crossinterp_c_1856)).interp = core::ptr::null_mut::<_is>();
    }
    if !unsafe { (*(err as *mut __SlateRecord_crossinterp_c_1856)).r#override }.is_null() {
        clear_xi_failure(unsafe { (*(err as *mut __SlateRecord_crossinterp_c_1856)).r#override });
    }
    _PyXI_excinfo_clear(unsafe {
        core::ptr::addr_of_mut!((*(err as *mut __SlateRecord_crossinterp_c_1856)).uncaught)
    });
}

fn xi_error_is_set(error: &mut __SlateRecord_crossinterp_c_1856) -> i32 {
    if !unsafe { (*(error as *mut __SlateRecord_crossinterp_c_1856)).r#override }.is_null() {
        return 1;
    }
    excinfo_is_set(unsafe {
        core::ptr::addr_of_mut!((*(error as *mut __SlateRecord_crossinterp_c_1856)).uncaught)
    })
}

fn xi_error_has_override(err: &__SlateRecord_crossinterp_c_1856) -> i32 {
    if unsafe { (*(err as *const __SlateRecord_crossinterp_c_1856)).r#override }.is_null() {
        return 0;
    }
    (((unsafe { (*(*(err as *const __SlateRecord_crossinterp_c_1856)).r#override).code }) as i32)
        != 0
        && ((unsafe { (*(*(err as *const __SlateRecord_crossinterp_c_1856)).r#override).code })
            as i32)
            != -1) as i32
}

fn xi_error_resolve_current_exc(tstate: *mut _ts, r#override: *mut xi_failure) -> *mut _object {
    let mut exc: *mut _object = unsafe { _PyErr_GetRaisedException(tstate) };
    if !exc.is_null() {
        if r#override.is_null() {
            // This is equivalent to _PyXI_ERR_UNCAUGHT_EXCEPTION.
        } else {
            if ((unsafe { (*r#override).code }) as i32) == -1 {
                // We want to actually capture the current exception.
            } else {
                if !exc.is_null() {
                    // It might make sense to do similarly for other codes.
                    if ((unsafe { (*r#override).code }) as i32) == -4 {
                        // We don't need the exception info.
                        Py_CLEAR__2!(exc);
                    }
                    // ...else we want to actually capture the current exception.
                }
            }
        }
    }
    exc
}

fn xi_error_set_override(
    tstate: *mut _ts,
    err: &mut __SlateRecord_crossinterp_c_1856,
    r#override: *mut xi_failure,
) {
    // Use xi_error_set_exc() instead of setting _PyXI_ERR_UNCAUGHT_EXCEPTION..
    unsafe {
        (*(err as *mut __SlateRecord_crossinterp_c_1856)).r#override =
            core::ptr::addr_of_mut!((*(err as *mut __SlateRecord_crossinterp_c_1856))._override);
    }
    // The caller still owns override->msg.
    copy_xi_failure(
        unsafe {
            core::ptr::addr_of_mut!((*(err as *mut __SlateRecord_crossinterp_c_1856))._override)
        },
        r#override,
    );
    unsafe {
        (*(err as *mut __SlateRecord_crossinterp_c_1856)).interp = (*tstate).interp;
    }
}

fn xi_error_set_override_code(
    tstate: *mut _ts,
    err: &mut __SlateRecord_crossinterp_c_1856,
    code: i32,
) {
    let mut __slate_compound_0: xi_failure = unsafe { core::mem::zeroed() };
    let mut r#override: xi_failure = unsafe {
        *{
            __slate_compound_0 = {
                let mut __t0: xi_failure = core::mem::zeroed();
                __t0.code = 0i32;
                __t0
            };
            core::ptr::addr_of_mut!(__slate_compound_0)
        }
    };
    r#override.code = code;
    xi_error_set_override(tstate, err, core::ptr::addr_of_mut!(r#override));
}

fn xi_error_set_exc(
    tstate: *mut _ts,
    err: &mut __SlateRecord_crossinterp_c_1856,
    exc: *mut _object,
) -> *const i8 {
    let failure: *const i8 = _PyXI_excinfo_InitFromException(
        unsafe {
            core::ptr::addr_of_mut!((*(err as *mut __SlateRecord_crossinterp_c_1856)).uncaught)
        },
        exc,
    );
    if !failure.is_null() {
        // We failed to initialize err->uncaught.
        //
        // XXX Print the excobj/traceback?  Emit a warning?
        //
        // XXX Print the current exception/traceback?
        if (unsafe { _PyErr_ExceptionMatches(tstate, PyExc_MemoryError) }) != 0 {
            xi_error_set_override_code(tstate, err, -3i32);
        } else {
            xi_error_set_override_code(tstate, err, -2i32);
        }
        unsafe { PyErr_Clear() };
    }
    failure
}

fn _PyXI_ApplyError(
    error: &mut __SlateRecord_crossinterp_c_1856,
    failure: *const i8,
) -> *mut _object {
    let tstate: *mut _ts = unsafe { PyThreadState_Get() };
    if !failure.is_null() {
        xi_error_clear(error);
        return core::ptr::null_mut::<_object>();
    }
    let mut code: i32 = -1;
    if !unsafe { (*(error as *mut __SlateRecord_crossinterp_c_1856)).r#override }.is_null() {
        code = unsafe { (*(*(error as *mut __SlateRecord_crossinterp_c_1856)).r#override).code };
    }
    if (code as i32) == -1 {
        // We will raise an exception that proxies the propagated exception.
        return _PyXI_excinfo_AsObject(unsafe {
            core::ptr::addr_of_mut!((*(error as *mut __SlateRecord_crossinterp_c_1856)).uncaught)
        });
    } else {
        if (code as i32) == -9 {
            // Propagate the exception directly.
            let mut cause: *mut _object = core::ptr::null_mut::<_object>();
            if excinfo_is_set(unsafe {
                core::ptr::addr_of_mut!(
                    (*(error as *mut __SlateRecord_crossinterp_c_1856)).uncaught
                )
            }) != 0
            {
                // Maybe instead set a PyExc_ExceptionSnapshot as __cause__?
                //
                // That type doesn't exist currently
                //
                // but would look like interpreters.ExecutionFailed.
                _PyXI_excinfo_Apply(
                    unsafe {
                        core::ptr::addr_of_mut!(
                            (*(error as *mut __SlateRecord_crossinterp_c_1856)).uncaught
                        )
                    },
                    unsafe { PyExc_Exception },
                );
                cause = unsafe { _PyErr_GetRaisedException(tstate) };
            }
            let msg: *const i8 =
                if !unsafe { (*(error as *mut __SlateRecord_crossinterp_c_1856)).r#override }
                    .is_null()
                {
                    unsafe { (*(*(error as *mut __SlateRecord_crossinterp_c_1856)).r#override).msg }
                } else {
                    unsafe {
                        (*(error as *mut __SlateRecord_crossinterp_c_1856))
                            .uncaught
                            .msg
                    }
                };
            _set_xid_lookup_failure(tstate, core::ptr::null_mut::<_object>(), msg, cause);
            Py_XDECREF!(cause);
        } else {
            // Raise an exception corresponding to the code.
            _PyXI_ApplyErrorCode(code, unsafe {
                (*(error as *mut __SlateRecord_crossinterp_c_1856)).interp
            });
            if excinfo_is_set(unsafe {
                core::ptr::addr_of_mut!(
                    (*(error as *mut __SlateRecord_crossinterp_c_1856)).uncaught
                )
            }) != 0
            {
                // __context__ will be set to a proxy of the propagated exception.
                //
                // (or use PyExc_ExceptionSnapshot like _PyXI_ERR_NOT_SHAREABLE?)
                let exc: *mut _object = unsafe { _PyErr_GetRaisedException(tstate) };
                _PyXI_excinfo_Apply(
                    unsafe {
                        core::ptr::addr_of_mut!(
                            (*(error as *mut __SlateRecord_crossinterp_c_1856)).uncaught
                        )
                    },
                    unsafe { PyExc_InterpreterError },
                );
                let exc2: *mut _object = unsafe { _PyErr_GetRaisedException(tstate) };
                unsafe { PyException_SetContext(exc, exc2) };
                unsafe { _PyErr_SetRaisedException(tstate, exc) };
            }
        }
    }
    core::ptr::null_mut::<_object>()
}

// shared namespaces
// Shared namespaces are expected to have relatively short lifetimes.
// This means dealloc of a shared namespace will normally happen "soon".
// Namespace items hold cross-interpreter data, which must get released.
// If the namespace/items are cleared in a different interpreter than
// where the items' cross-interpreter data was set then that will cause
// pending calls to be used to release the cross-interpreter data.
// The tricky bit is that the pending calls can happen sufficiently
// later that the namespace/items might already be deallocated.  This is
// a problem if the cross-interpreter data is allocated as part of a
// namespace item.  If that's the case then we must ensure the shared
// namespace is only cleared/freed *after* that data has been released.
#[repr(C)]
#[derive(Clone, Copy)]
struct _sharednsitem {
    name: *const i8,
    xidata: *mut _xidata,
    // We could have a "PyXIData _data" field, so it would
    //
    // be allocated as part of the item and avoid an extra allocation.
    //
    // However, doing so adds a bunch of complexity because we must
    //
    // ensure the item isn't freed before a pending call might happen
    //
    // in a different interpreter to release the XI data.
}

fn _sharednsitem_init(item: *mut _sharednsitem, key: *mut _object) -> i32 {
    unsafe {
        (*item).name = _copy_string_obj_raw(key, core::ptr::null_mut::<i64>()) as *const i8;
    }
    if unsafe { (*item).name }.is_null() {
        return 1i32.wrapping_neg();
    }
    unsafe {
        (*item).xidata = core::ptr::null_mut::<_xidata>();
    }
    0
}

fn _sharednsitem_has_value(item: *mut _sharednsitem, p_interpid: *mut i64) -> i32 {
    if unsafe { (*item).xidata }.is_null() {
        return 0;
    }
    if !p_interpid.is_null() {
        unsafe {
            *p_interpid = _PyXIData_INTERPID!(unsafe { (*item).xidata });
        }
    }
    1
}

fn _sharednsitem_set_value(item: *mut _sharednsitem, value: *mut _object, fallback: i32) -> i32 {
    unsafe {
        (*item).xidata = _PyXIData_New();
    }
    if unsafe { (*item).xidata }.is_null() {
        return 1i32.wrapping_neg();
    }
    let tstate: *mut _ts = unsafe { PyThreadState_Get() };
    if _PyObject_GetXIData(tstate, value, fallback, unsafe { (*item).xidata }) < 0 {
        unsafe { PyMem_RawFree((*item).xidata as *mut ()) };
        unsafe {
            (*item).xidata = core::ptr::null_mut::<_xidata>();
        }
        // The caller may want to propagate PyExc_NotShareableError
        //
        // if currently switched between interpreters.
        return 1i32.wrapping_neg();
    }
    0
}

fn _sharednsitem_clear_value(item: *mut _sharednsitem) {
    let xidata: *mut _xidata = unsafe { (*item).xidata };
    if !xidata.is_null() {
        unsafe {
            (*item).xidata = core::ptr::null_mut::<_xidata>();
        }
        let rawfree: i32 = 1;
        _release_xid_data(xidata, rawfree);
    }
}

fn _sharednsitem_clear(item: *mut _sharednsitem) {
    if !unsafe { (*item).name }.is_null() {
        unsafe { PyMem_RawFree((*item).name as *mut ()) };
        unsafe {
            (*item).name = core::ptr::null::<i8>();
        }
    }
    _sharednsitem_clear_value(item);
}

fn _sharednsitem_copy_from_ns(item: *mut _sharednsitem, ns: *mut _object, fallback: i32) -> i32 {
    let value: *mut _object = unsafe { PyDict_GetItemString(ns, (*item).name) }; // borrowed
    if value.is_null() {
        if !unsafe { PyErr_Occurred() }.is_null() {
            return 1i32.wrapping_neg();
        }
        // When applied, this item will be set to the default (or fail).
        return 0;
    }
    if _sharednsitem_set_value(item, value, fallback) < 0 {
        return 1i32.wrapping_neg();
    }
    0
}

fn _sharednsitem_apply(item: *mut _sharednsitem, ns: *mut _object, dflt: *mut _object) -> i32 {
    let name: *mut _object = unsafe { PyUnicode_FromString((*item).name) };
    if name.is_null() {
        return 1i32.wrapping_neg();
    }
    let mut value: *mut _object = unsafe { core::mem::zeroed() };
    if !unsafe { (*item).xidata }.is_null() {
        value = _PyXIData_NewObject(unsafe { (*item).xidata });
        if value.is_null() {
            Py_DECREF!(name);
            return 1i32.wrapping_neg();
        }
    } else {
        value = Py_NewRef!(dflt);
    }
    let res: i32 = unsafe { PyDict_SetItem(ns, name, value) };
    Py_DECREF!(name);
    Py_DECREF!(value);
    res
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_crossinterp_c_2174 {
    maxitems: i64,
    numnames: i64,
    numvalues: i64,
    items: [_sharednsitem; 1],
}

fn _sharedns_alloc(maxitems: i64) -> *mut __SlateRecord_crossinterp_c_2174 {
    if maxitems < 0 {
        if unsafe { PyErr_Occurred() }.is_null() {
            PyErr_BadInternalCall__49!();
        }
        return core::ptr::null_mut::<__SlateRecord_crossinterp_c_2174>();
    } else {
        if maxitems == 0 {
            unsafe { PyErr_SetString(PyExc_ValueError, c"empty namespaces not allowed".as_ptr()) };
            return core::ptr::null_mut::<__SlateRecord_crossinterp_c_2174>();
        }
    }
    // Check for overflow.
    let fixedsize: u64 = (core::mem::size_of::<__SlateRecord_crossinterp_c_2174>() as u64)
        .wrapping_sub(core::mem::size_of::<_sharednsitem>() as u64);
    if (maxitems as u64)
        > 9223372036854775807u64.wrapping_sub(fixedsize)
            / (core::mem::size_of::<_sharednsitem>() as u64)
    {
        unsafe { PyErr_NoMemory() };
        return core::ptr::null_mut::<__SlateRecord_crossinterp_c_2174>();
    }
    // Allocate the value, including items.
    let size: u64 = fixedsize
        .wrapping_add((core::mem::size_of::<_sharednsitem>() as u64).wrapping_mul(maxitems as u64));
    let ns: *mut __SlateRecord_crossinterp_c_2174 =
        (unsafe { PyMem_RawCalloc(size, 1u64) }) as *mut __SlateRecord_crossinterp_c_2174;
    if ns.is_null() {
        unsafe { PyErr_NoMemory() };
        return core::ptr::null_mut::<__SlateRecord_crossinterp_c_2174>();
    }
    unsafe {
        (*ns).maxitems = maxitems;
    }
    ns
}

fn _sharedns_free(ns: *mut __SlateRecord_crossinterp_c_2174) {
    // If we weren't always dynamically allocating the cross-interpreter
    //
    // data in each item then we would need to use a pending call
    //
    // to call _sharedns_free(), to avoid the race between freeing
    //
    // the shared namespace and releasing the XI data.
    let mut i: i64 = 0;
    let mut item: *mut _sharednsitem = unsafe { core::mem::zeroed() };
    if (unsafe { (*ns).numvalues }) > 0 {
        // One or more items may have interpreter-specific data.
        while i < unsafe { (*ns).numvalues } {
            item = unsafe { (*ns).items.as_mut_ptr() as *mut _sharednsitem }
                .wrapping_offset(i as isize);
            // While we do want to ensure consistency across items,
            //
            // technically they don't need to match the current
            //
            // interpreter.  However, we keep the constraint for
            //
            // simplicity, by giving _PyXI_FreeNamespace() the exclusive
            //
            // responsibility of dealing with the owning interpreter.
            _sharednsitem_clear(item);
            i = i.wrapping_add(1);
        }
    }
    while i < unsafe { (*ns).numnames } {
        item =
            unsafe { (*ns).items.as_mut_ptr() as *mut _sharednsitem }.wrapping_offset(i as isize);
        _sharednsitem_clear(item);
        i = i.wrapping_add(1);
    }
    unsafe { PyMem_RawFree(ns as *mut ()) };
}

fn _create_sharedns(names: *mut _object) -> *mut __SlateRecord_crossinterp_c_2174 {
    let mut name: *mut _object = unsafe { core::mem::zeroed() };
    let mut pos: i64 = unsafe { core::mem::zeroed() };
    let mut i: i64 = unsafe { core::mem::zeroed() };
    let mut name_1: *mut _object = unsafe { core::mem::zeroed() };
    let mut i_1: i64 = unsafe { core::mem::zeroed() };
    let mut items: *mut _sharednsitem = unsafe { core::mem::zeroed() };
    let mut ns: *mut __SlateRecord_crossinterp_c_2174 = unsafe { core::mem::zeroed() };
    let mut __v18: i64 = unsafe { core::mem::zeroed() };
    let mut numnames: i64 = unsafe { core::mem::zeroed() };
    unsafe {
        if PyDict_CheckExact!(names) != 0 {
            __v18 = PyDict_Size(names);
        } else {
            __v18 = PySequence_Size(names);
        }
        numnames = __v18;
        ns = _sharedns_alloc(numnames);
        if ns.is_null() {
            return core::ptr::null_mut::<__SlateRecord_crossinterp_c_2174>();
        } else {
            '__join_0: {
                items = (*ns).items.as_mut_ptr() as *mut _sharednsitem;
                // Fill in the names.
                if PyDict_CheckExact!(names) != 0 {
                    i = 0;
                    pos = 0;
                    while PyDict_Next(
                        names,
                        core::ptr::addr_of_mut!(pos),
                        core::ptr::addr_of_mut!(name),
                        core::ptr::null_mut::<*mut _object>(),
                    ) != 0
                    {
                        if _sharednsitem_init(items.wrapping_offset(i as isize), name) < 0 {
                            break '__join_0;
                        } else {
                            let __v3: i64 = (*ns).numnames;
                            (*ns).numnames = __v3.wrapping_add(1);
                            i = i.wrapping_add(1);
                        }
                    }
                } else {
                    if PySequence_Check(names) != 0 {
                        i_1 = 0;
                        while i_1 < numnames {
                            name_1 = PySequence_GetItem(names, i_1);
                            if name_1.is_null() {
                                break '__join_0;
                            } else {
                                let res: i32 =
                                    _sharednsitem_init(items.wrapping_offset(i_1 as isize), name_1);
                                Py_DECREF!(name_1);
                                if res < 0 {
                                    break '__join_0;
                                } else {
                                    let __v11: i64 = (*ns).numnames;
                                    (*ns).numnames = __v11.wrapping_add(1);
                                    i_1 = i_1.wrapping_add(1);
                                }
                            }
                        }
                    } else {
                        PyErr_SetString(
                            PyExc_NotImplementedError,
                            c"non-sequence namespace not supported".as_ptr(),
                        );
                        break '__join_0;
                    }
                }
                return ns;
            }
            _sharedns_free(ns);
            return core::ptr::null_mut::<__SlateRecord_crossinterp_c_2174>();
        }
    }
}

fn _fill_sharedns(
    ns: *mut __SlateRecord_crossinterp_c_2174,
    nsobj: *mut _object,
    fallback: i32,
    p_err: *mut xi_failure,
) -> i32 {
    // All items are expected to be shareable.
    let tstate: *mut _ts = unsafe { PyThreadState_Get() };
    let mut i: i64 = 0;
    while i < unsafe { (*ns).maxitems } {
        if _sharednsitem_copy_from_ns(
            unsafe { (*ns).items.as_mut_ptr() as *mut _sharednsitem }.wrapping_offset(i as isize),
            nsobj,
            fallback,
        ) < 0
        {
            if !p_err.is_null() {
                _propagate_not_shareable_error(tstate, p_err);
            }
            // Clear out the ones we set so far.
            for j in 0..i {
                _sharednsitem_clear_value(
                    unsafe { (*ns).items.as_mut_ptr() as *mut _sharednsitem }
                        .wrapping_offset(j as isize),
                );
                let __v1: i64 = unsafe { (*ns).numvalues };
                unsafe {
                    (*ns).numvalues = __v1.wrapping_sub(1);
                }
            }
            return 1i32.wrapping_neg();
        }
        let __v5: i64 = unsafe { (*ns).numvalues };
        unsafe {
            (*ns).numvalues = __v5.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    0
}

#[unsafe(link_section = ".text.slate_distinct.crossinterp._sharedns_free_pending")]
extern "C-unwind" fn _sharedns_free_pending(data: *mut ()) -> i32 {
    _sharedns_free(data as *mut __SlateRecord_crossinterp_c_2174);
    0
}

fn _destroy_sharedns(ns: *mut __SlateRecord_crossinterp_c_2174) {
    if (unsafe { (*ns).numvalues }) == 0 {
        _sharedns_free(ns);
        return;
    }
    let mut interpid0: i64 = 0;
    if _sharednsitem_has_value(
        unsafe { (*ns).items.as_mut_ptr() as *mut _sharednsitem }.wrapping_offset(0 as isize),
        core::ptr::addr_of_mut!(interpid0),
    ) == 0
    {
        // This shouldn't have been possible.
        //
        // We can deal with it in _sharedns_free().
        _sharedns_free(ns);
        return;
    }
    let interp: *mut _is = unsafe { _PyInterpreterState_LookUpID(interpid0) };
    if interp == unsafe { PyInterpreterState_Get() } {
        _sharedns_free(ns);
        return;
    }
    // One or more items may have interpreter-specific data.
    //
    // Currently the xidata for each value is dynamically allocated,
    //
    // so technically we don't need to worry about that.
    //
    // However, explicitly adding a pending call here is simpler.
    _Py_CallInInterpreter(interp, Some(_sharedns_free_pending), ns as *mut ());
}

fn _apply_sharedns(
    ns: *mut __SlateRecord_crossinterp_c_2174,
    nsobj: *mut _object,
    dflt: *mut _object,
) -> i32 {
    let mut i: i64 = 0;
    while i < unsafe { (*ns).maxitems } {
        if _sharednsitem_apply(
            unsafe { (*ns).items.as_mut_ptr() as *mut _sharednsitem }.wrapping_offset(i as isize),
            nsobj,
            dflt,
        ) != 0
        {
            return 1i32.wrapping_neg();
        }
        i = i.wrapping_add(1);
    }
    0
}

// switched-interpreter sessions
#[repr(C)]
#[derive(Clone, Copy)]
struct xi_session {
    status: i32,
    switched: i32,
    /// Once a session has been entered, this is the tstate that was
    ///
    /// current before the session.  If it is different from cur_tstate
    ///
    /// then we must have switched interpreters.  Either way, this will
    ///
    /// be the current tstate once we exit the session.
    prev_tstate: *mut _ts,
    /// Once a session has been entered, this is the current tstate.
    ///
    /// It must be current when the session exits.
    init_tstate: *mut _ts,
    /// This is true if init_tstate needs cleanup during exit.
    own_init_tstate: i32,
    /// This is true if, while entering the session, init_thread took
    ///
    /// "ownership" of the interpreter's __main__ module.  This means
    ///
    /// it is the only thread that is allowed to run code there.
    ///
    /// (Caveat: for now, users may still run exec() against the
    ///
    /// __main__ module's dict, though that isn't advisable.)
    running: i32,
    /// This is a cached reference to the __dict__ of the entered
    ///
    /// interpreter's __main__ module.  It is looked up when at the
    ///
    /// beginning of the session as a convenience.
    main_ns: *mut _object,
    /// This is a dict of objects that will be available (via sharing)
    ///
    /// once the session exits.  Do not access this directly; use
    ///
    /// _PyXI_Preserve() and _PyXI_GetPreserved() instead;
    _preserved: *mut _object,
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyXI_NewSession() -> *mut xi_session {
    let session: *mut xi_session =
        (unsafe { PyMem_RawCalloc(1u64, core::mem::size_of::<xi_session>() as u64) })
            as *mut xi_session;
    if session.is_null() {
        unsafe { PyErr_NoMemory() };
        return core::ptr::null_mut::<xi_session>();
    }
    session
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyXI_FreeSession(session: *mut xi_session) {
    unsafe { PyMem_RawFree(session as *mut ()) };
}

fn _session_is_active(session: *mut xi_session) -> i32 {
    ((unsafe { (*session).status }) == 1) as i32
}

// enter/exit a cross-interpreter session
fn _enter_session(session: *mut xi_session, interp: *mut _is) {
    let mut __slate_compound_0: xi_session = unsafe { core::mem::zeroed() };
    // Set here and cleared in _exit_session().
    // Set elsewhere and cleared in _exit_session().
    // Switch to interpreter.
    let mut tstate: *mut _ts = unsafe { PyThreadState_Get() };
    let prev: *mut _ts = tstate;
    let same_interp: i32 = (interp == unsafe { (*tstate).interp }) as i32;
    if same_interp == 0 {
        tstate = unsafe { _PyThreadState_NewBound(interp, 5i32) };
        // XXX Possible GILState issues?
        let swapped: *mut _ts = unsafe { PyThreadState_Swap(tstate) };
    }
    unsafe {
        *session = *{
            __slate_compound_0 = {
                let mut __t0: xi_session = core::mem::zeroed();
                __t0.status = 1i32;
                __t0.switched = (same_interp == 0) as i32;
                __t0.prev_tstate = prev;
                __t0.init_tstate = tstate;
                __t0.own_init_tstate = (same_interp == 0) as i32;
                __t0
            };
            core::ptr::addr_of_mut!(__slate_compound_0)
        };
    }
}

fn _exit_session(session: *mut xi_session) {
    let mut __slate_compound_2: xi_session = unsafe { core::mem::zeroed() };
    let tstate: *mut _ts = unsafe { (*session).init_tstate };
    // Release any of the entered interpreters resources.
    Py_CLEAR!((*session).main_ns);
    Py_CLEAR!((*session)._preserved);
    // Ensure this thread no longer owns __main__.
    if (unsafe { (*session).running }) != 0 {
        unsafe { _PyInterpreterState_SetNotRunningMain((*tstate).interp) };
        unsafe {
            (*session).running = 0;
        }
    }
    // Switch back.
    if (unsafe { (*session).prev_tstate }) != unsafe { (*session).init_tstate } {
        unsafe {
            (*session).own_init_tstate = 0;
        }
        unsafe { PyThreadState_Clear(tstate) };
        unsafe { PyThreadState_Swap((*session).prev_tstate) };
        unsafe { PyThreadState_Delete(tstate) };
    }
    unsafe {
        *session = *{
            __slate_compound_2 = {
                let mut __t0: xi_session = core::mem::zeroed();
                __t0.status = 0i32;
                __t0
            };
            core::ptr::addr_of_mut!(__slate_compound_2)
        };
    }
}

fn _propagate_not_shareable_error(tstate: *mut _ts, r#override: *mut xi_failure) {
    let mut __slate_compound_0: xi_failure = unsafe { core::mem::zeroed() };
    let exctype: *mut _object = get_notshareableerror_type(tstate);
    if exctype.is_null() {
        unsafe {
            PyErr_FormatUnraisable(
                c"Exception ignored while propagating not shareable error".as_ptr(),
            )
        };
        return;
    }
    if (unsafe { PyErr_ExceptionMatches(exctype) }) != 0 {
        // We want to propagate the exception directly.
        unsafe {
            *r#override = *{
                __slate_compound_0 = {
                    let mut __t0: xi_failure = core::mem::zeroed();
                    __t0.code = -9i32;
                    __t0
                };
                core::ptr::addr_of_mut!(__slate_compound_0)
            };
        }
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyXI_Enter(
    session: *mut xi_session,
    interp: *mut _is,
    nsupdates: *mut _object,
    result: *mut __SlateRecord_pycore_crossinterp_h_360,
) -> i32 {
    let mut __slate_compound_8: xi_failure = unsafe { core::mem::zeroed() };
    let mut __slate_compound_9: xi_failure = unsafe { core::mem::zeroed() };
    // Apply the error from the other interpreter.
    let mut excinfo: *mut _object = unsafe { core::mem::zeroed() };
    let mut failure: *const i8 = unsafe { core::mem::zeroed() };
    let mut err: core::mem::MaybeUninit<__SlateRecord_crossinterp_c_1856> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_2: *mut __SlateRecord_crossinterp_c_1856 =
        core::ptr::addr_of_mut!(err) as *mut __SlateRecord_crossinterp_c_1856;
    let mut r#override: core::mem::MaybeUninit<xi_failure> = core::mem::MaybeUninit::uninit();
    let __slate_slot_3: *mut xi_failure = core::ptr::addr_of_mut!(r#override) as *mut xi_failure;
    let mut _err: core::mem::MaybeUninit<xi_failure> = core::mem::MaybeUninit::uninit();
    let __slate_slot_4: *mut xi_failure = core::ptr::addr_of_mut!(_err) as *mut xi_failure;
    let mut len: i64 = unsafe { core::mem::zeroed() };
    // Convert the attrs for cross-interpreter use.
    let mut sharedns: *mut __SlateRecord_crossinterp_c_2174 = unsafe { core::mem::zeroed() };
    unsafe {
        sharedns = core::ptr::null_mut::<__SlateRecord_crossinterp_c_2174>();
        if !nsupdates.is_null() {
            len = PyDict_Size(nsupdates);
            if len < 0 {
                if !result.is_null() {
                    (*result).errcode = -6;
                }
                return 1i32.wrapping_neg();
            } else {
                if len > 0 {
                    sharedns = _create_sharedns(nsupdates);
                    if sharedns.is_null() {
                        if !result.is_null() {
                            (*result).errcode = -6;
                        }
                        return 1i32.wrapping_neg();
                    } else {
                        // For now we limit it to shareable objects.
                        let fallback: i32 = 0;
                        core::ptr::write(__slate_slot_4, *{
                            __slate_compound_9 = {
                                let mut __t2: xi_failure = core::mem::zeroed();
                                __t2.code = 0i32;
                                __t2
                            };
                            core::ptr::addr_of_mut!(__slate_compound_9)
                        });
                        if _fill_sharedns(
                            sharedns,
                            nsupdates,
                            fallback,
                            core::ptr::addr_of_mut!(*__slate_slot_4),
                        ) < 0
                        {
                            if ((*__slate_slot_4).code as i32) == 0 {
                                (*__slate_slot_4).code = -1;
                            }
                            _destroy_sharedns(sharedns);
                            if !result.is_null() {
                                (*result).errcode = (*__slate_slot_4).code;
                            }
                            return 1i32.wrapping_neg();
                        }
                    }
                }
            }
        }
        '__join_6: {
            // Switch to the requested interpreter (if necessary).
            _enter_session(session, interp);
            core::ptr::write(__slate_slot_3, *{
                __slate_compound_8 = {
                    let mut __t1: xi_failure = core::mem::zeroed();
                    __t1.code = 0i32;
                    __t1
                };
                core::ptr::addr_of_mut!(__slate_compound_8)
            });
            (*__slate_slot_3).code = -1;
            // Ensure this thread owns __main__.
            if _PyInterpreterState_SetRunningMain(interp) < 0 {
                // In the case where we didn't switch interpreters, it would
                //
                // be more efficient to leave the exception in place and return
                //
                // immediately.  However, life is simpler if we don't.
                (*__slate_slot_3).code = -4;
            } else {
                (*session).running = 1;
                // Apply the cross-interpreter data.
                if !sharedns.is_null() {
                    if _ensure_main_ns(session, core::ptr::addr_of_mut!(*__slate_slot_3)) < 0 {
                        break '__join_6;
                    } else {
                        if _apply_sharedns(
                            sharedns,
                            (*session).main_ns,
                            core::ptr::null_mut::<_object>(),
                        ) < 0
                        {
                            (*__slate_slot_3).code = -6;
                            break '__join_6;
                        } else {
                            _destroy_sharedns(sharedns);
                        }
                    }
                }
                (*__slate_slot_3).code = 0;
                return 0;
            }
        }
        // We want to propagate all exceptions here directly (best effort).
        core::ptr::write(__slate_slot_2, {
            let mut __t0: __SlateRecord_crossinterp_c_1856 = core::mem::zeroed();
            __t0.interp = core::ptr::null_mut::<_is>();
            __t0
        });
        failure = capture_session_error(
            session,
            &mut (*core::ptr::addr_of_mut!(*__slate_slot_2)),
            core::ptr::addr_of_mut!(*__slate_slot_3),
        );
        // Exit the session.
        _exit_session(session);
        if !sharedns.is_null() {
            _destroy_sharedns(sharedns);
        }
        excinfo = _PyXI_ApplyError(&mut (*core::ptr::addr_of_mut!(*__slate_slot_2)), failure);
        xi_error_clear(&mut (*core::ptr::addr_of_mut!(*__slate_slot_2)));
        if !excinfo.is_null() {
            if !result.is_null() {
                (*result).excinfo = excinfo;
            } else {
                Py_DECREF!(excinfo);
            }
        }
        return 1i32.wrapping_neg();
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyXI_Exit(
    session: *mut xi_session,
    mut r#override: *mut xi_failure,
    result: *mut __SlateRecord_pycore_crossinterp_h_360,
) -> i32 {
    let mut __slate_compound_0: xi_failure = unsafe { core::mem::zeroed() };
    let mut __slate_compound_2: __SlateRecord_pycore_crossinterp_h_360 =
        unsafe { core::mem::zeroed() };
    let mut tstate: *mut _ts = _PyThreadState_GET();
    let mut res: i32 = 0;
    // Capture the raised exception, if any.
    let mut err: __SlateRecord_crossinterp_c_1856 = {
        let mut __t0: __SlateRecord_crossinterp_c_1856 = unsafe { core::mem::zeroed() };
        __t0.interp = core::ptr::null_mut::<_is>();
        __t0
    };
    let mut failure: *const i8 = core::ptr::null::<i8>();
    if !r#override.is_null() && ((unsafe { (*r#override).code }) as i32) == 0 {
        r#override = core::ptr::null_mut::<xi_failure>();
    }
    if !_PyErr_Occurred(tstate).is_null() {
        failure = capture_session_error(
            session,
            unsafe { &mut (*core::ptr::addr_of_mut!(err)) },
            r#override,
        );
    }
    // Capture the preserved namespace.
    let mut preserved: *mut __SlateRecord_crossinterp_c_2174 =
        core::ptr::null_mut::<__SlateRecord_crossinterp_c_2174>();
    let mut preservedobj: *mut _object = core::ptr::null_mut::<_object>();
    if !result.is_null() {
        let mut _override: xi_failure = unsafe {
            *{
                __slate_compound_0 = {
                    let mut __t1: xi_failure = core::mem::zeroed();
                    __t1.code = 0i32;
                    __t1
                };
                core::ptr::addr_of_mut!(__slate_compound_0)
            }
        };
        if _pop_preserved(
            session,
            unsafe { &mut (*core::ptr::addr_of_mut!(preserved)) },
            unsafe { &mut (*core::ptr::addr_of_mut!(preservedobj)) },
            core::ptr::addr_of_mut!(_override),
        ) < 0
        {
            if xi_error_is_set(unsafe { &mut (*core::ptr::addr_of_mut!(err)) }) != 0 {
                // XXX Chain the exception (i.e. set __context__)?
                unsafe {
                    PyErr_FormatUnraisable(
                        c"Exception ignored while capturing preserved objects".as_ptr(),
                    )
                };
                clear_xi_failure(core::ptr::addr_of_mut!(_override));
            } else {
                if (_override.code as i32) == 0 {
                    _override.code = -1;
                }
                failure = capture_session_error(
                    session,
                    unsafe { &mut (*core::ptr::addr_of_mut!(err)) },
                    core::ptr::addr_of_mut!(_override),
                );
            }
        }
    }
    // Exit the session.
    _exit_session(session);
    tstate = _PyThreadState_GET();
    // Restore the preserved namespace.
    if _finish_preserved(preserved, core::ptr::addr_of_mut!(preservedobj)) < 0 {
        if xi_error_is_set(unsafe { &mut (*core::ptr::addr_of_mut!(err)) }) != 0 {
            // XXX Chain the exception (i.e. set __context__)?
            unsafe {
                PyErr_FormatUnraisable(
                    c"Exception ignored while capturing preserved objects".as_ptr(),
                )
            };
        } else {
            xi_error_set_override_code(
                tstate,
                unsafe { &mut (*core::ptr::addr_of_mut!(err)) },
                -7i32,
            );
            _propagate_not_shareable_error(tstate, err.r#override);
        }
    }
    if !result.is_null() {
        unsafe {
            (*result).preserved = preservedobj;
        }
        unsafe {
            (*result).errcode = (if !err.r#override.is_null() {
                (*err.r#override).code as i32
            } else {
                0i32
            }) as i32;
        }
    }
    // Apply the error from the other interpreter, if any.
    if xi_error_is_set(unsafe { &mut (*core::ptr::addr_of_mut!(err)) }) != 0 {
        res = 1i32.wrapping_neg();
        let excinfo: *mut _object =
            _PyXI_ApplyError(unsafe { &mut (*core::ptr::addr_of_mut!(err)) }, failure);
        if excinfo.is_null() {
            let __v1: bool = !result.is_null()
                && xi_error_has_override(unsafe { &(*core::ptr::addr_of_mut!(err)) }) == 0;
            if __v1 {
                _PyXI_ClearResult(result);
                unsafe {
                    *result = *{
                        __slate_compound_2 = {
                            let mut __t2: __SlateRecord_pycore_crossinterp_h_360 =
                                core::mem::zeroed();
                            __t2.errcode = -8i32;
                            __t2
                        };
                        core::ptr::addr_of_mut!(__slate_compound_2)
                    };
                }
            }
        } else {
            if !result.is_null() {
                unsafe {
                    (*result).excinfo = excinfo;
                }
            } else {
                Py_DECREF!(excinfo);
            }
        }
        xi_error_clear(unsafe { &mut (*core::ptr::addr_of_mut!(err)) });
    }
    res
}

// in an active cross-interpreter session
fn capture_session_error(
    session: *mut xi_session,
    err: &mut __SlateRecord_crossinterp_c_1856,
    mut r#override: *mut xi_failure,
) -> *const i8 {
    let tstate: *mut _ts = unsafe { (*session).init_tstate };
    // Normalize the exception override.
    if !r#override.is_null() {
        if ((unsafe { (*r#override).code }) as i32) == -1 {
            r#override = core::ptr::null_mut::<xi_failure>();
        }
    }
    // Handle the exception, if any.
    let mut failure: *const i8 = core::ptr::null::<i8>();
    let exc: *mut _object = xi_error_resolve_current_exc(tstate, r#override);
    if !exc.is_null() {
        // There is an unhandled exception we need to preserve.
        failure = xi_error_set_exc(tstate, err, exc);
        Py_DECREF!(exc);
        if !_PyErr_Occurred(tstate).is_null() {
            unsafe { PyErr_FormatUnraisable(failure) };
        }
    }
    // Handle the override.
    if !r#override.is_null() && failure.is_null() {
        xi_error_set_override(tstate, err, r#override);
    }
    // Finished!
    failure
}

fn _ensure_main_ns(session: *mut xi_session, failure: *mut xi_failure) -> i32 {
    let mut __slate_compound_0: xi_failure = unsafe { core::mem::zeroed() };
    let mut __slate_compound_1: xi_failure = unsafe { core::mem::zeroed() };
    let tstate: *mut _ts = unsafe { (*session).init_tstate };
    if !unsafe { (*session).main_ns }.is_null() {
        return 0;
    }
    // Cache __main__.__dict__.
    let main_mod: *mut _object = unsafe { _Py_GetMainModule(tstate) };
    if (unsafe { _Py_CheckMainModule(main_mod) }) < 0 {
        Py_XDECREF!(main_mod);
        if !failure.is_null() {
            unsafe {
                *failure = *{
                    __slate_compound_0 = {
                        let mut __t0: xi_failure = core::mem::zeroed();
                        __t0.code = -5i32;
                        __t0
                    };
                    core::ptr::addr_of_mut!(__slate_compound_0)
                };
            }
        }
        return 1i32.wrapping_neg();
    }
    let ns: *mut _object = unsafe { PyModule_GetDict(main_mod) }; // borrowed
    Py_DECREF!(main_mod);
    if ns.is_null() {
        if !failure.is_null() {
            unsafe {
                *failure = *{
                    __slate_compound_1 = {
                        let mut __t1: xi_failure = core::mem::zeroed();
                        __t1.code = -5i32;
                        __t1
                    };
                    core::ptr::addr_of_mut!(__slate_compound_1)
                };
            }
        }
        return 1i32.wrapping_neg();
    }
    unsafe {
        (*session).main_ns = Py_NewRef!(ns);
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyXI_GetMainNamespace(
    session: *mut xi_session,
    failure: *mut xi_failure,
) -> *mut _object {
    if _session_is_active(session) == 0 {
        unsafe { PyErr_SetString(PyExc_RuntimeError, c"session not active".as_ptr()) };
        return core::ptr::null_mut::<_object>();
    }
    if _ensure_main_ns(session, failure) < 0 {
        return core::ptr::null_mut::<_object>();
    }
    unsafe { (*session).main_ns }
}

fn _pop_preserved(
    session: *mut xi_session,
    p_xidata: &mut *mut __SlateRecord_crossinterp_c_2174,
    p_obj: &mut *mut _object,
    p_failure: *mut xi_failure,
) -> i32 {
    let mut __slate_compound_4: xi_failure = unsafe { core::mem::zeroed() };
    // We did switch interpreters.
    let mut len: i64 = unsafe { core::mem::zeroed() };
    let mut xidata: *mut __SlateRecord_crossinterp_c_2174 = unsafe { core::mem::zeroed() };
    let mut failure: core::mem::MaybeUninit<xi_failure> = core::mem::MaybeUninit::uninit();
    let __slate_slot_2: *mut xi_failure = core::ptr::addr_of_mut!(failure) as *mut xi_failure;
    unsafe {
        core::ptr::write(__slate_slot_2, *{
            __slate_compound_4 = {
                let mut __t0: xi_failure = core::mem::zeroed();
                __t0.code = 0i32;
                __t0
            };
            core::ptr::addr_of_mut!(__slate_compound_4)
        });
        xidata = core::ptr::null_mut::<__SlateRecord_crossinterp_c_2174>();
        // active session
        if unsafe { (*session)._preserved }.is_null() {
            *(p_xidata as *mut *mut __SlateRecord_crossinterp_c_2174) =
                core::ptr::null_mut::<__SlateRecord_crossinterp_c_2174>();
            *(p_obj as *mut *mut _object) = core::ptr::null_mut::<_object>();
            return 0;
        } else {
            if (*session).init_tstate == (*session).prev_tstate {
                // We did not switch interpreters.
                *(p_xidata as *mut *mut __SlateRecord_crossinterp_c_2174) =
                    core::ptr::null_mut::<__SlateRecord_crossinterp_c_2174>();
                *(p_obj as *mut *mut _object) = (*session)._preserved;
                (*session)._preserved = core::ptr::null_mut::<_object>();
                return 0;
            } else {
                '__join_4: {
                    *(p_obj as *mut *mut _object) = core::ptr::null_mut::<_object>();
                    len = PyDict_Size((*session)._preserved);
                    if len < 0 {
                        (*__slate_slot_2).code = -7;
                    } else {
                        if len == 0 {
                            *(p_xidata as *mut *mut __SlateRecord_crossinterp_c_2174) =
                                core::ptr::null_mut::<__SlateRecord_crossinterp_c_2174>();
                        } else {
                            xidata = _create_sharedns((*session)._preserved);
                            if xidata.is_null() {
                                (*__slate_slot_2).code = -7;
                                break '__join_4;
                            } else {
                                if _fill_sharedns(
                                    xidata,
                                    (*session)._preserved,
                                    1i32,
                                    core::ptr::addr_of_mut!(*__slate_slot_2),
                                ) < 0
                                {
                                    if ((*__slate_slot_2).code as i32) != -9 {
                                        (*__slate_slot_2).code = -7;
                                        break '__join_4;
                                    } else {
                                        break '__join_4;
                                    }
                                } else {
                                    *(p_xidata as *mut *mut __SlateRecord_crossinterp_c_2174) =
                                        xidata;
                                }
                            }
                        }
                        Py_CLEAR!((*session)._preserved);
                        return 0;
                    }
                }
                if !p_failure.is_null() {
                    *p_failure = *__slate_slot_2;
                }
                if !xidata.is_null() {
                    _destroy_sharedns(xidata);
                }
                return 1i32.wrapping_neg();
            }
        }
    }
}

fn _finish_preserved(
    xidata: *mut __SlateRecord_crossinterp_c_2174,
    p_preserved: *mut *mut _object,
) -> i32 {
    let mut ns: *mut _object = unsafe { core::mem::zeroed() };
    let mut res: i32 = unsafe { core::mem::zeroed() };
    unsafe {
        if xidata.is_null() {
            return 0;
        } else {
            '__join_0: {
                res = 1i32.wrapping_neg();
                if !p_preserved.is_null() {
                    ns = PyDict_New();
                    if ns.is_null() {
                        break '__join_0;
                    } else {
                        if _apply_sharedns(xidata, ns, core::ptr::null_mut::<_object>()) < 0 {
                            Py_CLEAR__2!(ns);
                            break '__join_0;
                        } else {
                            *p_preserved = ns;
                        }
                    }
                }
                res = 0;
            }
            _destroy_sharedns(xidata);
            return res;
        }
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyXI_Preserve(
    session: *mut xi_session,
    name: *const i8,
    value: *mut _object,
    p_failure: *mut xi_failure,
) -> i32 {
    let mut __slate_compound_1: xi_failure = unsafe { core::mem::zeroed() };
    let mut failure: core::mem::MaybeUninit<xi_failure> = core::mem::MaybeUninit::uninit();
    let __slate_slot_0: *mut xi_failure = core::ptr::addr_of_mut!(failure) as *mut xi_failure;
    unsafe {
        core::ptr::write(__slate_slot_0, *{
            __slate_compound_1 = {
                let mut __t0: xi_failure = core::mem::zeroed();
                __t0.code = 0i32;
                __t0
            };
            core::ptr::addr_of_mut!(__slate_compound_1)
        });
        if _session_is_active(session) == 0 {
            PyErr_SetString(PyExc_RuntimeError, c"session not active".as_ptr());
            return 1i32.wrapping_neg();
        } else {
            '__join_2: {
                if unsafe { (*session)._preserved }.is_null() {
                    (*session)._preserved = PyDict_New();
                    if unsafe { (*session)._preserved }.is_null() {
                        set_exc_with_cause(
                            PyExc_RuntimeError,
                            c"failed to initialize preserved objects".as_ptr(),
                        );
                        (*__slate_slot_0).code = -7;
                        break '__join_2;
                    }
                }
                if PyDict_SetItemString((*session)._preserved, name, value) < 0 {
                    set_exc_with_cause(PyExc_RuntimeError, c"failed to preserve object".as_ptr());
                    (*__slate_slot_0).code = -7;
                } else {
                    return 0;
                }
            }
            if !p_failure.is_null() {
                *p_failure = *__slate_slot_0;
            }
            return 1i32.wrapping_neg();
        }
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyXI_GetPreserved(
    result: *mut __SlateRecord_pycore_crossinterp_h_360,
    name: *const i8,
) -> *mut _object {
    let mut value: *mut _object = core::ptr::null_mut::<_object>();
    if !unsafe { (*result).preserved }.is_null() {
        unsafe {
            PyDict_GetItemStringRef((*result).preserved, name, core::ptr::addr_of_mut!(value))
        };
    }
    value
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyXI_ClearResult(result: *mut __SlateRecord_pycore_crossinterp_h_360) {
    Py_CLEAR!((*result).preserved);
    Py_CLEAR!((*result).excinfo);
}

// runtime lifecycle
#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_xi_global_state_init(
    state: *mut __SlateRecord_pycore_crossinterp_h_255,
) -> i32 {
    xid_lookup_init(unsafe { core::ptr::addr_of_mut!((*state).data_lookup) });
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_xi_global_state_fini(state: *mut __SlateRecord_pycore_crossinterp_h_255) {
    xid_lookup_fini(unsafe { core::ptr::addr_of_mut!((*state).data_lookup) });
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_xi_state_init(
    state: *mut __SlateRecord_pycore_crossinterp_h_268,
    interp: *mut _is,
) -> i32 {
    xid_lookup_init(unsafe { core::ptr::addr_of_mut!((*state).data_lookup) });
    // Initialize exceptions.
    if !interp.is_null() {
        if init_static_exctypes(
            unsafe { core::ptr::addr_of_mut!((*state).exceptions) },
            interp,
        ) < 0
        {
            fini_heap_exctypes(unsafe { core::ptr::addr_of_mut!((*state).exceptions) });
            return 1i32.wrapping_neg();
        }
    }
    if init_heap_exctypes(unsafe { core::ptr::addr_of_mut!((*state).exceptions) }) < 0 {
        return 1i32.wrapping_neg();
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_xi_state_fini(
    state: *mut __SlateRecord_pycore_crossinterp_h_268,
    interp: *mut _is,
) {
    fini_heap_exctypes(unsafe { core::ptr::addr_of_mut!((*state).exceptions) });
    if !interp.is_null() {
        fini_static_exctypes(
            unsafe { core::ptr::addr_of_mut!((*state).exceptions) },
            interp,
        );
    }
    xid_lookup_fini(unsafe { core::ptr::addr_of_mut!((*state).data_lookup) });
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyXI_Init(interp: *mut _is) -> __SlateRecord_initconfig_h_19 {
    let mut __slate_compound_0: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_1: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_2: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_3: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_4: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    if _Py_IsMainInterpreter(interp) != 0 {
        let global_state: *mut __SlateRecord_pycore_crossinterp_h_255 =
            _PyXI_GET_GLOBAL_STATE!(interp);
        if global_state.is_null() {
            unsafe { PyErr_PrintEx(0i32) };
            return unsafe {
                *{
                    __slate_compound_0 = {
                        let mut __t0: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                        __t0._type = 1u32;
                        __t0.func = _PyStatus_GET_FUNC__6!() as *const i8;
                        __t0.err_msg = c"failed to get global cross-interpreter state".as_ptr();
                        __t0
                    };
                    core::ptr::addr_of_mut!(__slate_compound_0)
                }
            };
        }
        if _Py_xi_global_state_init(global_state) < 0 {
            unsafe { PyErr_PrintEx(0i32) };
            return unsafe {
                *{
                    __slate_compound_1 = {
                        let mut __t1: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                        __t1._type = 1u32;
                        __t1.func = _PyStatus_GET_FUNC__6!() as *const i8;
                        __t1.err_msg =
                            c"failed to initialize  global cross-interpreter state".as_ptr();
                        __t1
                    };
                    core::ptr::addr_of_mut!(__slate_compound_1)
                }
            };
        }
    }
    let state: *mut __SlateRecord_pycore_crossinterp_h_268 = _PyXI_GET_STATE!(interp);
    if state.is_null() {
        unsafe { PyErr_PrintEx(0i32) };
        return unsafe {
            *{
                __slate_compound_2 = {
                    let mut __t2: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                    __t2._type = 1u32;
                    __t2.func = _PyStatus_GET_FUNC__6!() as *const i8;
                    __t2.err_msg = c"failed to get interpreter's cross-interpreter state".as_ptr();
                    __t2
                };
                core::ptr::addr_of_mut!(__slate_compound_2)
            }
        };
    }
    // The static types were already initialized in _PyXI_InitTypes(),
    //
    // so we pass in NULL here to avoid initializing them again.
    if _Py_xi_state_init(state, core::ptr::null_mut::<_is>()) < 0 {
        unsafe { PyErr_PrintEx(0i32) };
        return unsafe {
            *{
                __slate_compound_3 = {
                    let mut __t3: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                    __t3._type = 1u32;
                    __t3.func = _PyStatus_GET_FUNC__6!() as *const i8;
                    __t3.err_msg =
                        c"failed to initialize interpreter's cross-interpreter state".as_ptr();
                    __t3
                };
                core::ptr::addr_of_mut!(__slate_compound_3)
            }
        };
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

// _PyXI_Fini() must be called before the interpreter is cleared,
//
// since we must clear some heap objects.
#[unsafe(no_mangle)]
extern "C-unwind" fn _PyXI_Fini(interp: *mut _is) {
    let state: *mut __SlateRecord_pycore_crossinterp_h_268 = _PyXI_GET_STATE!(interp);
    // The static types will be finalized soon in _PyXI_FiniTypes(),
    //
    // so we pass in NULL here to avoid finalizing them right now.
    _Py_xi_state_fini(state, core::ptr::null_mut::<_is>());
    if _Py_IsMainInterpreter(interp) != 0 {
        let global_state: *mut __SlateRecord_pycore_crossinterp_h_255 =
            _PyXI_GET_GLOBAL_STATE!(interp);
        _Py_xi_global_state_fini(global_state);
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyXI_InitTypes(interp: *mut _is) -> __SlateRecord_initconfig_h_19 {
    let mut __slate_compound_0: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_1: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    if init_static_exctypes(
        unsafe { core::ptr::addr_of_mut!((*_PyXI_GET_STATE!(interp)).exceptions) },
        interp,
    ) < 0
    {
        unsafe { PyErr_PrintEx(0i32) };
        return unsafe {
            *{
                __slate_compound_0 = {
                    let mut __t0: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                    __t0._type = 1u32;
                    __t0.func = _PyStatus_GET_FUNC__62!() as *const i8;
                    __t0.err_msg =
                        c"failed to initialize the cross-interpreter exception types".as_ptr();
                    __t0
                };
                core::ptr::addr_of_mut!(__slate_compound_0)
            }
        };
    }
    // We would initialize heap types here too but that leads to ref leaks.
    //
    // Instead, we intialize them in _PyXI_Init().
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

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyXI_FiniTypes(interp: *mut _is) {
    // We would finalize heap types here too but that leads to ref leaks.
    //
    // Instead, we finalize them in _PyXI_Fini().
    fini_static_exctypes(
        unsafe { core::ptr::addr_of_mut!((*_PyXI_GET_STATE!(interp)).exceptions) },
        interp,
    );
}

// other API
#[unsafe(no_mangle)]
extern "C-unwind" fn _PyXI_NewInterpreter(
    config: *mut __SlateRecord_pylifecycle_h_50,
    maybe_whence: *mut i64,
    p_tstate: *mut *mut _ts,
    p_save_tstate: *mut *mut _ts,
) -> *mut _is {
    let mut save_tstate: *mut _ts = unsafe { PyThreadState_Swap(core::ptr::null_mut::<_ts>()) };
    let mut tstate: *mut _ts = unsafe { core::mem::zeroed() };
    let status: __SlateRecord_initconfig_h_19 = unsafe {
        Py_NewInterpreterFromConfig(
            core::ptr::addr_of_mut!(tstate),
            config as *const __SlateRecord_pylifecycle_h_50,
        )
    };
    if (unsafe { PyStatus_Exception(status) }) != 0 {
        // Since no new thread state was created, there is no exception
        //
        // to propagate; raise a fresh one after swapping back in the
        //
        // old thread state.
        unsafe { PyThreadState_Swap(save_tstate) };
        unsafe { _PyErr_SetFromPyStatus(status) };
        let exc: *mut _object = unsafe { PyErr_GetRaisedException() };
        unsafe {
            PyErr_SetString(
                PyExc_InterpreterError,
                c"sub-interpreter creation failed".as_ptr(),
            )
        };
        unsafe { _PyErr_ChainExceptions1(exc) };
        return core::ptr::null_mut::<_is>();
    }
    let interp: *mut _is = unsafe { PyThreadState_GetInterpreter(tstate) };
    let mut whence: i64 = 4;
    if !maybe_whence.is_null() {
        whence = unsafe { *maybe_whence };
    }
    unsafe { _PyInterpreterState_SetWhence(interp, whence) };
    if !p_tstate.is_null() {
        // We leave the new thread state as the current one.
        unsafe {
            *p_tstate = tstate;
        }
    } else {
        // Throw away the initial tstate.
        unsafe { PyThreadState_Clear(tstate) };
        unsafe { PyThreadState_Swap(save_tstate) };
        unsafe { PyThreadState_Delete(tstate) };
        save_tstate = core::ptr::null_mut::<_ts>();
    }
    if !p_save_tstate.is_null() {
        unsafe {
            *p_save_tstate = save_tstate;
        }
    }
    interp
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyXI_EndInterpreter(
    interp: *mut _is,
    mut tstate: *mut _ts,
    p_save_tstate: *mut *mut _ts,
) {
    if (unsafe { _PyInterpreterState_IsReady(interp) }) == 0 {
        // PyInterpreterState_Clear() requires the GIL,
        //
        // which a not-ready does not have, so we don't clear it.
        //
        // That means there may be leaks here until clearing the
        //
        // interpreter is fixed.
        unsafe { PyInterpreterState_Delete(interp) };
        return;
    }
    let mut save_tstate: *mut _ts = core::ptr::null_mut::<_ts>();
    let cur_tstate: *mut _ts = PyThreadState_GET!();
    if tstate.is_null() {
        if (unsafe { PyThreadState_GetInterpreter(cur_tstate) }) == interp {
            tstate = cur_tstate;
        } else {
            tstate = unsafe { _PyThreadState_NewBound(interp, 2i32) };
            save_tstate = unsafe { PyThreadState_Swap(tstate) };
        }
    } else {
        if tstate != cur_tstate {
            save_tstate = unsafe { PyThreadState_Swap(tstate) };
        }
    }
    unsafe { Py_EndInterpreter(tstate) };
    if !p_save_tstate.is_null() {
        save_tstate = unsafe { *p_save_tstate };
    }
    unsafe { PyThreadState_Swap(save_tstate) };
}
