//! New exceptions.c written in Iceland by Richard Jones and Georg Brandl.
//!
//! Thanks go to Tim Peters and Michael Hudson for debugging.
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
//! PyException_HEAD defines the initial segment of every exception class.
//! A code object, the __code__ attribute
//! This limitation is for performance and simplicity. If needed it can be
//! removed (with effort).
//! Remote debugger support
//! PY_TIMEOUT_T is the integral type used to specify timeouts when waiting
//!
//! on a lock (see PyThread_acquire_lock_timed() below).
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
//! Fast inlined version of PyType_IS_GC()
// Signed variants of the above
// Round up size "n" to be a multiple of "a".
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
macro_rules! MEMERRORS_LOCK {
    () => {};
}

macro_rules! MEMERRORS_UNLOCK {
    () => {};
}

macro_rules! OR_NONE {
    ($x:expr) => {
        if !$x.is_null() {
            $x
        } else {
            unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
        }
    };
}

macro_rules! PyBytes_AS_STRING {
    ($op:expr) => {
        PyBytes_AS_STRING(_PyObject_CAST!($op))
    };
}

macro_rules! PyBytes_Check {
    ($op:expr) => {
        PyType_FastSubclass!(Py_TYPE!($op), 134217728u64)
    };
}

macro_rules! PyBytes_GET_SIZE {
    ($self:expr) => {
        PyBytes_GET_SIZE(_PyObject_CAST!($self))
    };
}

macro_rules! PyDict_Check {
    ($op:expr) => {
        PyType_FastSubclass!(Py_TYPE!($op), 536870912u64)
    };
}

macro_rules! PyExceptionClass_Check__2 {
    ($x:expr) => {{
        let __v: bool = PyType_Check__3!($x) != 0
            && PyType_FastSubclass__2!(($x) as *mut _typeobject, 1073741824u64) != 0;
        __v
    }};
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

macro_rules! PyList_Check {
    ($op:expr) => {
        PyType_FastSubclass!(Py_TYPE!($op), 33554432u64)
    };
}

macro_rules! PyList_GET_ITEM {
    ($op:expr, $index:expr) => {
        unsafe { *unsafe { (*_PyList_CAST!($op)).ob_item }.wrapping_offset(($index) as isize) }
    };
}

macro_rules! PyList_GET_SIZE {
    ($op:expr) => {
        PyList_GET_SIZE(_PyObject_CAST!($op))
    };
}

macro_rules! PyLong_CheckExact {
    ($op:expr) => {
        Py_IS_TYPE!($op, unsafe { core::ptr::addr_of_mut!(PyLong_Type) })
    };
}

macro_rules! PyLong_Check {
    ($op:expr) => {
        PyType_FastSubclass!(Py_TYPE!($op), 16777216u64)
    };
}

macro_rules! PyObject_TypeCheck {
    ($ob:expr, $type:expr) => {
        PyObject_TypeCheck(_PyObject_CAST!($ob), $type)
    };
}

macro_rules! PyTraceBack_Check {
    ($v:expr) => {
        Py_IS_TYPE!($v, unsafe { core::ptr::addr_of_mut!(PyTraceBack_Type) })
    };
}

macro_rules! PyTuple_CheckExact {
    ($op:expr) => {
        Py_IS_TYPE!($op, unsafe { core::ptr::addr_of_mut!(PyTuple_Type) })
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

macro_rules! PyTuple_SET_ITEM {
    ($op:expr, $index:expr, $value:expr) => {
        PyTuple_SET_ITEM(
            _PyObject_CAST!($op),
            ($index) as i64,
            _PyObject_CAST!($value),
        );
    };
}

macro_rules! PyType_Check {
    ($op:expr) => {
        PyType_Check(_PyObject_CAST!($op))
    };
}

macro_rules! PyType_Check__3 {
    ($op:expr) => {
        PyType_Check(_PyObject_CAST__3!($op))
    };
}

macro_rules! PyType_FastSubclass__2 {
    ($type:expr, $flag:expr) => {
        PyType_HasFeature($type, $flag)
    };
}

macro_rules! PyType_FastSubclass {
    ($type:expr, $flag:expr) => {
        PyType_HasFeature($type, $flag)
    };
}

macro_rules! PyUnicodeErrorObject_CAST {
    ($op:expr) => {
        ($op) as *mut __SlateRecord_pyerrors_h_52
    };
}

macro_rules! PyUnicodeError_CAST {
    ($PTR:expr) => {
        ($PTR) as *mut __SlateRecord_pyerrors_h_52
    };
}

macro_rules! PyUnicodeError_Check {
    ($PTR:expr) => {
        PyObject_TypeCheck!($PTR, (unsafe { PyExc_UnicodeError }) as *mut _typeobject)
    };
}

macro_rules! PyUnicode_CheckExact {
    ($op:expr) => {
        Py_IS_TYPE!($op, unsafe { core::ptr::addr_of_mut!(PyUnicode_Type) })
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

macro_rules! PyUnicode_READ {
    ($kind:expr, $data:expr, $index:expr) => {
        PyUnicode_READ(
            _Py_STATIC_CAST!(i32, $kind),
            _Py_STATIC_CAST__2!($data),
            $index,
        )
    };
}

macro_rules! PyVectorcall_NARGS {
    ($n:expr) => {
        _PyVectorcall_NARGS($n)
    };
}

macro_rules! Py_ARRAY_LENGTH__4 {
    () => {
        (core::mem::size_of::<[static_exception; 68]>() as u64)
            / (core::mem::size_of::<static_exception>() as u64)
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

macro_rules! Py_FatalError__20 {
    ($message:expr) => {
        unsafe { _Py_FatalErrorFunc(c"_PyErr_NoMemory".as_ptr(), ($message) as *const i8) };
        unsafe { core::hint::unreachable_unchecked() };
    };
}

macro_rules! Py_INCREF {
    ($op:expr) => {
        Py_INCREF(_PyObject_CAST!($op));
    };
}

macro_rules! Py_IS_TYPE__2 {
    ($ob:expr, $type:expr) => {
        Py_IS_TYPE(_PyObject_CAST__2!($ob), $type)
    };
}

macro_rules! Py_IS_TYPE {
    ($ob:expr, $type:expr) => {
        Py_IS_TYPE(_PyObject_CAST!($ob), $type)
    };
}

macro_rules! Py_IS_TYPE__5 {
    ($ob:expr) => {
        Py_IS_TYPE(_PyObject_CAST!($ob), core::ptr::null_mut::<_typeobject>())
    };
}

macro_rules! Py_IsNone {
    ($x:expr) => {
        Py_Is!($x, unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) })
    };
}

macro_rules! Py_Is {
    ($x:expr, $y:expr) => {
        ($x) == $y
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

macro_rules! Py_XDECREF__2 {
    ($op:expr) => {
        Py_XDECREF(_PyObject_CAST__2!($op));
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

macro_rules! Py_XSETREF {
    ($dst:expr, $src:expr) => {
        loop {
            let _tmp_dst_ptr: *mut *mut _object = unsafe { core::ptr::addr_of_mut!($dst) };
            let _tmp_old_dst: *mut _object = unsafe { *_tmp_dst_ptr };
            unsafe {
                *_tmp_dst_ptr = $src;
            }
            Py_XDECREF!(_tmp_old_dst);
            if true {
                break;
            }
        }
    };
}

macro_rules! _PyASCIIObject_CAST {
    ($op:expr) => {
        _Py_CAST!(*mut __SlateRecord_unicodeobject_h_163, $op)
    };
}

macro_rules! _PyArg_NoKeywords__2 {
    ($funcname:expr, $kwargs:expr) => {{
        let __v: bool =
            $kwargs.is_null() || (unsafe { _PyArg_NoKeywords($funcname, $kwargs) }) != 0;
        __v
    }};
}

macro_rules! _PyArg_NoKwnames__2 {
    ($funcname:expr, $kwnames:expr) => {{
        let __v: bool =
            $kwnames.is_null() || (unsafe { _PyArg_NoKwnames($funcname, $kwnames) }) != 0;
        __v
    }};
}

macro_rules! _PyBaseExceptionGroup_Check {
    ($x:expr) => {
        PyObject_TypeCheck!(
            $x,
            (unsafe { PyExc_BaseExceptionGroup }) as *mut _typeobject
        )
    };
}

/// Macros and static inline functions, trading safety for speed
macro_rules! _PyBytes_CAST {
    ($op:expr) => {
        _Py_CAST!(*mut __SlateRecord_bytesobject_h_15, $op)
    };
}

macro_rules! _PyCompactUnicodeObject_CAST {
    ($op:expr) => {
        _Py_CAST!(*mut __SlateRecord_unicodeobject_h_173, $op)
    };
}

/// Cast argument to PyListObject* type.
macro_rules! _PyList_CAST {
    ($op:expr) => {
        _Py_CAST!(*mut __SlateRecord_listobject_h_22, $op)
    };
}

/// Define a pair of assertion macros:
/// _PyObject_ASSERT_FROM(), _PyObject_ASSERT_WITH_MSG() and _PyObject_ASSERT().
///
/// These work like the regular C assert(), in that they will abort the
/// process with a message on stderr if the given condition fails to hold,
/// but compile away to nothing if NDEBUG is defined.
///
/// However, before aborting, Python will also try to call _PyObject_Dump() on
/// the given object.  This may be of use when investigating bugs in which a
/// particular object is corrupt (e.g. buggy a tp_visit method in an extension
/// module breaking the garbage collector), to help locate the broken objects.
///
/// The WITH_MSG variant allows you to supply an additional message that Python
/// will attempt to print to stderr, after the object dump.
///
/// No debugging: compile away the assertions:
macro_rules! _PyObject_ASSERT_FROM {
    () => {};
}

/// Cast argument to PyObject* type.
macro_rules! _PyObject_CAST__3 {
    ($op:expr) => {
        _Py_CAST__7!($op)
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

macro_rules! _PyObject_GC_TRACK {
    ($op:expr) => {
        _PyObject_GC_TRACK(_PyObject_CAST__2!($op));
    };
}

macro_rules! _PyObject_GC_UNTRACK {
    ($op:expr) => {
        _PyObject_GC_UNTRACK(_PyObject_CAST!($op));
    };
}

macro_rules! _PyObject_GC_UNTRACK__2 {
    ($op:expr) => {
        _PyObject_GC_UNTRACK(_PyObject_CAST__2!($op));
    };
}

macro_rules! _PyStatus_GET_FUNC__63 {
    () => {
        c"_PyExc_InitGlobalObjects".as_ptr() as *mut i8
    };
}

macro_rules! _PyStatus_GET_FUNC {
    () => {
        c"_PyExc_InitState".as_ptr() as *mut i8
    };
}

/// Cast argument to PyTupleObject* type.
macro_rules! _PyTuple_CAST {
    ($op:expr) => {
        _Py_CAST!(*mut __SlateRecord_tupleobject_h_13, $op)
    };
}

macro_rules! _PyType_CAST {
    ($op:expr) => {
        _Py_CAST!(*mut _typeobject, $op)
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

/// Macro to use the more powerful/dangerous C-style cast even in C++.
macro_rules! _Py_CAST__2 {
    ($expr:expr) => {
        $expr
    };
}

/// Macro to use the more powerful/dangerous C-style cast even in C++.
macro_rules! _Py_CAST__7 {
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

macro_rules! _Py_STATIC_CAST__2 {
    ($expr:expr) => {
        $expr
    };
}

macro_rules! _Py_STATIC_CAST {
    ($type:ty, $expr:expr) => {
        ($expr) as $type
    };
}

unsafe extern "C" {
    static mut PyLong_Type: _typeobject;
    static mut _Py_NoneStruct: _object;
    static mut PyUnicode_Type: _typeobject;
    static mut PyTuple_Type: _typeobject;
    static mut PyTraceBack_Type: _typeobject;
    #[thread_local]
    static mut _Py_tss_tstate: *mut _ts;
    static mut _PyRuntime: pyruntimestate;
    fn index(arg0: *const i8, arg1: i32) -> *mut i8;
    fn PyObject_GetBuffer(
        arg0: *mut _object,
        arg1: *mut __SlateRecord_pybuffer_h_33,
        arg2: i32,
    ) -> i32;
    fn PyBuffer_Release(arg0: *mut __SlateRecord_pybuffer_h_33);
    fn PyType_IsSubtype(arg0: *mut _typeobject, arg1: *mut _typeobject) -> i32;
    fn PyObject_Repr(arg0: *mut _object) -> *mut _object;
    fn PyObject_Str(arg0: *mut _object) -> *mut _object;
    fn PyObject_GetOptionalAttr(
        arg0: *mut _object,
        arg1: *mut _object,
        arg2: *mut *mut _object,
    ) -> i32;
    fn PyObject_SetAttr(arg0: *mut _object, arg1: *mut _object, arg2: *mut _object) -> i32;
    fn PyObject_GenericGetAttr(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn PyObject_GenericSetAttr(arg0: *mut _object, arg1: *mut _object, arg2: *mut _object) -> i32;
    fn PyObject_GenericSetDict(arg0: *mut _object, arg1: *mut _object, arg2: *mut ()) -> i32;
    fn PyObject_IsTrue(arg0: *mut _object) -> i32;
    fn PyCallable_Check(arg0: *mut _object) -> i32;
    fn Py_GetConstant(arg0: u32) -> *mut _object;
    fn _Py_NewReference(arg0: *mut _object);
    fn _PyType_Name(arg0: *mut _typeobject) -> *const i8;
    fn _Py_Dealloc(arg0: *mut _object);
    fn PyObject_GC_UnTrack(arg0: *mut ());
    fn PyBytes_FromStringAndSize(arg0: *const i8, arg1: i64) -> *mut _object;
    fn PyUnicode_FromString(arg0: *const i8) -> *mut _object;
    fn PyUnicode_Substring(arg0: *mut _object, arg1: i64, arg2: i64) -> *mut _object;
    fn PyUnicode_ReadChar(arg0: *mut _object, arg1: i64) -> u32;
    fn PyUnicode_FromFormat(arg0: *const i8, ...) -> *mut _object;
    fn PyErr_SetString(arg0: *mut _object, arg1: *const i8);
    fn PyErr_Occurred() -> *mut _object;
    fn PyErr_GivenExceptionMatches(arg0: *mut _object, arg1: *mut _object) -> i32;
    fn PyErr_Format(arg0: *mut _object, arg1: *const i8, ...) -> *mut _object;
    fn PyErr_NewException(arg0: *const i8, arg1: *mut _object, arg2: *mut _object) -> *mut _object;
    fn _Py_FatalErrorFunc(arg0: *const i8, arg1: *const i8);
    fn PyLong_FromLong(arg0: i64) -> *mut _object;
    fn PyLong_FromSsize_t(arg0: i64) -> *mut _object;
    fn PyLong_AsLongAndOverflow(arg0: *mut _object, arg1: *mut i32) -> i64;
    fn PyLong_FromVoidPtr(arg0: *mut ()) -> *mut _object;
    fn PyTuple_New(arg0: i64) -> *mut _object;
    fn PyTuple_Size(arg0: *mut _object) -> i64;
    fn PyTuple_GetSlice(arg0: *mut _object, arg1: i64, arg2: i64) -> *mut _object;
    fn PyTuple_Pack(arg0: i64, ...) -> *mut _object;
    fn PyList_New(arg0: i64) -> *mut _object;
    fn PyList_GetItem(arg0: *mut _object, arg1: i64) -> *mut _object;
    fn PyList_Append(arg0: *mut _object, arg1: *mut _object) -> i32;
    fn PyDict_New() -> *mut _object;
    fn PyDict_GetItemWithError(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn PyDict_SetItem(arg0: *mut _object, arg1: *mut _object, arg2: *mut _object) -> i32;
    fn PyDict_Next(
        arg0: *mut _object,
        arg1: *mut i64,
        arg2: *mut *mut _object,
        arg3: *mut *mut _object,
    ) -> i32;
    fn PyDict_Copy(arg0: *mut _object) -> *mut _object;
    fn PyDict_SetItemString(arg0: *mut _object, arg1: *const i8, arg2: *mut _object) -> i32;
    fn PyObject_GenericGetDict(arg0: *mut _object, arg1: *mut ()) -> *mut _object;
    fn PySet_New(arg0: *mut _object) -> *mut _object;
    fn PySet_Add(arg0: *mut _object, arg1: *mut _object) -> i32;
    fn PySet_Contains(arg0: *mut _object, arg1: *mut _object) -> i32;
    fn PyModule_GetDict(arg0: *mut _object) -> *mut _object;
    fn Py_GenericAlias(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn PyArg_ParseTuple(arg0: *mut _object, arg1: *const i8, ...) -> i32;
    fn PyArg_ParseTupleAndKeywords(
        arg0: *mut _object,
        arg1: *mut _object,
        arg2: *const i8,
        arg3: *const *mut i8,
        ...
    ) -> i32;
    fn PyArg_UnpackTuple(arg0: *mut _object, arg1: *const i8, arg2: i64, arg3: i64, ...) -> i32;
    fn PyObject_CallObject(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn PyObject_CallFunction(arg0: *mut _object, arg1: *const i8, ...) -> *mut _object;
    fn PyObject_CallMethod(
        arg0: *mut _object,
        arg1: *const i8,
        arg2: *const i8,
        ...
    ) -> *mut _object;
    fn PyNumber_Check(arg0: *mut _object) -> i32;
    fn PyNumber_AsSsize_t(arg0: *mut _object, arg1: *mut _object) -> i64;
    fn PySequence_Check(arg0: *mut _object) -> i32;
    fn PySequence_Size(arg0: *mut _object) -> i64;
    fn PySequence_Tuple(arg0: *mut _object) -> *mut _object;
    fn PySequence_List(arg0: *mut _object) -> *mut _object;
    fn PyObject_IsInstance(arg0: *mut _object, arg1: *mut _object) -> i32;
    fn PyObject_IsSubclass(arg0: *mut _object, arg1: *mut _object) -> i32;
    fn PyObject_CallOneArg(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn _PyErr_SetRaisedException(arg0: *mut _ts, arg1: *mut _object);
    fn _Py_CheckRecursiveCall(arg0: *mut _ts, arg1: *const i8) -> i32;
    fn _PyArg_NoKwnames(arg0: *const i8, arg1: *mut _object) -> i32;
    fn _PyArg_NoKeywords(arg0: *const i8, arg1: *mut _object) -> i32;
    fn _PyArg_BadArgument(arg0: *const i8, arg1: *const i8, arg2: *const i8, arg3: *mut _object);
    fn _PyStaticType_InitBuiltin(arg0: *mut _is, arg1: *mut _typeobject) -> i32;
    fn _PyStaticType_FiniBuiltin(arg0: *mut _is, arg1: *mut _typeobject);
    fn _PyTuple_FromArray(arg0: *const *mut _object, arg1: i64) -> *mut _object;
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

// Flags for getting buffers. Keep these in sync with inspect.BufferFlags.
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

fn PyObject_TypeCheck(ob: *mut _object, r#type: *mut _typeobject) -> i32 {
    let __v0: bool =
        Py_IS_TYPE!(ob, r#type) != 0 || (unsafe { PyType_IsSubtype(Py_TYPE!(ob), r#type) }) != 0;
    __v0 as i32
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
// Set if the type allows subclassing
// Objects support garbage collection (see objimpl.h)
fn PyType_HasFeature(r#type: *mut _typeobject, feature: u64) -> i32 {
    let flags: u64 = unsafe { (*r#type).tp_flags };
    (flags & feature != 0) as i32
}

fn PyType_Check(op: *mut _object) -> i32 {
    PyType_FastSubclass!(Py_TYPE!(op), 2147483648u64)
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
    __slate_bits_0: __slate_bits::__SlateBits137U0,
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

fn PyUnicode_READ(kind: i32, data: *const (), index: i64) -> u32 {
    if kind == 1 {
        return (unsafe { *_Py_STATIC_CAST!(*const u8, data).wrapping_offset(index as isize) })
            as u32;
    }
    if kind == 2 {
        return (unsafe { *_Py_STATIC_CAST!(*const u16, data).wrapping_offset(index as isize) })
            as u32;
    }
    unsafe { *_Py_STATIC_CAST!(*const u32, data).wrapping_offset(index as isize) }
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
struct __SlateRecord_pyerrors_h_22 {
    ob_base: _object,
    dict: *mut _object,
    args: *mut _object,
    notes: *mut _object,
    traceback: *mut _object,
    context: *mut _object,
    cause: *mut _object,
    suppress_context: i8,
    msg: *mut _object,
    excs: *mut _object,
    excs_str: *mut _object,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pyerrors_h_35 {
    ob_base: _object,
    dict: *mut _object,
    args: *mut _object,
    notes: *mut _object,
    traceback: *mut _object,
    context: *mut _object,
    cause: *mut _object,
    suppress_context: i8,
    msg: *mut _object,
    filename: *mut _object,
    lineno: *mut _object,
    offset: *mut _object,
    end_lineno: *mut _object,
    end_offset: *mut _object,
    text: *mut _object,
    print_file_and_line: *mut _object,
    metadata: *mut _object,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pyerrors_h_43 {
    ob_base: _object,
    dict: *mut _object,
    args: *mut _object,
    notes: *mut _object,
    traceback: *mut _object,
    context: *mut _object,
    cause: *mut _object,
    suppress_context: i8,
    msg: *mut _object,
    name: *mut _object,
    path: *mut _object,
    name_from: *mut _object,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pyerrors_h_52 {
    ob_base: _object,
    dict: *mut _object,
    args: *mut _object,
    notes: *mut _object,
    traceback: *mut _object,
    context: *mut _object,
    cause: *mut _object,
    suppress_context: i8,
    encoding: *mut _object,
    object: *mut _object,
    start: i64,
    end: i64,
    reason: *mut _object,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pyerrors_h_57 {
    ob_base: _object,
    dict: *mut _object,
    args: *mut _object,
    notes: *mut _object,
    traceback: *mut _object,
    context: *mut _object,
    cause: *mut _object,
    suppress_context: i8,
    code: *mut _object,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pyerrors_h_69 {
    ob_base: _object,
    dict: *mut _object,
    args: *mut _object,
    notes: *mut _object,
    traceback: *mut _object,
    context: *mut _object,
    cause: *mut _object,
    suppress_context: i8,
    myerrno: *mut _object,
    strerror: *mut _object,
    filename: *mut _object,
    filename2: *mut _object,
    written: i64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pyerrors_h_74 {
    ob_base: _object,
    dict: *mut _object,
    args: *mut _object,
    notes: *mut _object,
    traceback: *mut _object,
    context: *mut _object,
    cause: *mut _object,
    suppress_context: i8,
    value: *mut _object,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pyerrors_h_79 {
    ob_base: _object,
    dict: *mut _object,
    args: *mut _object,
    notes: *mut _object,
    traceback: *mut _object,
    context: *mut _object,
    cause: *mut _object,
    suppress_context: i8,
    name: *mut _object,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pyerrors_h_85 {
    ob_base: _object,
    dict: *mut _object,
    args: *mut _object,
    notes: *mut _object,
    traceback: *mut _object,
    context: *mut _object,
    cause: *mut _object,
    suppress_context: i8,
    obj: *mut _object,
    name: *mut _object,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pystate_h_78 {
    __slate_bits_0: __slate_bits::__SlateBits208U0,
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

fn PyTuple_SET_ITEM(op: *mut _object, index: i64, value: *mut _object) {
    let tuple: *mut __SlateRecord_tupleobject_h_13 = _PyTuple_CAST!(op);
    unsafe {
        *((*tuple).ob_item.as_mut_ptr() as *mut *mut _object).wrapping_offset(index as isize) =
            value;
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_listobject_h_22 {
    ob_base: __SlateRecord_object_h_172,
    ob_item: *mut *mut _object,
    allocated: i64,
}

fn PyList_GET_SIZE(op: *mut _object) -> i64 {
    let list: *mut __SlateRecord_listobject_h_22 = _PyList_CAST!(op);
    Py_SIZE!(list)
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_structs_h_32_2 {
    code: u8,
    arg: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct PyMethodDef {
    ml_name: *const i8,
    ml_meth: Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
    ml_flags: i32,
    ml_doc: *const i8,
}

// METH_NOARGS and METH_O must not be combined with the flags above.
// METH_CLASS and METH_STATIC are a little different; these control
// the construction of methods for a class.  These cannot be used for
// functions in modules.
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

// Deprecated, use Py_T_OBJECT_EX instead
// Added by Lillo: bools contained in the structure (assumed char)
// Py_ssize_t
// Flags
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
struct code_arena_st {}

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
struct __SlateRecord_pycore_pythread_h_64 {
    ptr: *mut __SlateRecord_pthreadtypes_h_45,
    val: __SlateRecord_pthreadtypes_h_45,
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
struct __SlateRecord_pycore_interp_structs_h_297 {
    mutex: PyMutex,
    hashtable: *mut _Py_hashtable_t,
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
struct __SlateRecord_pycore_interp_structs_h_348 {
    import_level: i32,
    accumulated: i64,
    header: i32,
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
struct xi_exceptions {
    PyExc_InterpreterError: *mut _object,
    PyExc_InterpreterNotFoundError: *mut _object,
    PyExc_NotShareableError: *mut _object,
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

#[unsafe(no_mangle)]
static mut _PyExc_IncompleteInputError: _typeobject = {
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
    __t0.tp_name = (b"_IncompleteInputError\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pyerrors_h_35>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(SyntaxError_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = None;
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = None;
    __t0.tp_getattro = None;
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 17408u64;
    __t0.tp_doc = (b"incomplete input.\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_traverse = Some(SyntaxError_traverse);
    __t0.tp_clear = Some(SyntaxError_clear);
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = core::ptr::null_mut::<PyMethodDef>();
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(_PyExc_SyntaxError) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_35, dict) as u64) as i64;
    __t0.tp_init = Some(SyntaxError_init);
    __t0.tp_alloc = None;
    __t0.tp_new = None;
    __t0
};

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
struct _rare_events {
    set_class: u8,
    set_bases: u8,
    set_eval_frame_func: u8,
    builtin_dict: u8,
    func_modification: u8,
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
union __SlateRecord_pycore_obmalloc_h_255 {
    _padding: *mut u8,
    count: u32,
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
struct _Py_unicode_runtime_state {
    ids: _Py_unicode_runtime_ids,
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
struct __SlateRecord_pycore_runtime_structs_h_49 {
    fd: i32,
    st_dev: u64,
    st_ino: u64,
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
struct __SlateRecord_pycore_tracemalloc_h_70 {
    mem: __SlateRecord_pymem_h_48,
    raw: __SlateRecord_pymem_h_48,
    obj: __SlateRecord_pymem_h_48,
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
union __SlateRecord_sigaction_h_31 {
    sa_handler: Option<unsafe extern "C-unwind" fn(i32)>,
    sa_sigaction:
        Option<unsafe extern "C-unwind" fn(i32, *mut __SlateRecord_siginfo_t_h_124, *mut ())>,
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

fn _PyInterpreterState_GET() -> *mut _is {
    let tstate: *mut _ts = _PyThreadState_GET();
    unsafe { (*tstate).interp }
}

fn _Py_get_machine_stack_pointer() -> u64 {
    (unsafe { __slate_intrinsic_2d7c29df8fdf6378(const { ((0 as i32) as u32) as i32 }) }) as u64
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

fn _Py_EnterRecursiveCallTstate(tstate: *mut _ts, r#where: *const i8) -> i32 {
    let __v0: bool =
        _Py_MakeRecCheck(tstate) != 0 && (unsafe { _Py_CheckRecursiveCall(tstate, r#where) }) != 0;
    __v0 as i32
}

fn _Py_EnterRecursiveCall(r#where: *const i8) -> i32 {
    let tstate: *mut _ts = _PyThreadState_GET();
    _Py_EnterRecursiveCallTstate(tstate, r#where)
}

fn _Py_LeaveRecursiveCall() {}

fn _Py_AS_GC(op: *mut _object) -> *mut __SlateRecord_pycore_interp_structs_h_170 {
    let gc: *mut i8 = (op as *mut i8).wrapping_offset(
        -(core::mem::size_of::<__SlateRecord_pycore_interp_structs_h_170>() as isize),
    );
    gc as *mut __SlateRecord_pycore_interp_structs_h_170
}

// Bit flags for _gc_prev
//
// Bit 0 is set when tp_finalize is called
fn _PyGCHead_NEXT(
    gc: *mut __SlateRecord_pycore_interp_structs_h_170,
) -> *mut __SlateRecord_pycore_interp_structs_h_170 {
    let next: u64 = unsafe { (*gc)._gc_next };
    next as *mut __SlateRecord_pycore_interp_structs_h_170
}

fn _PyGCHead_SET_NEXT(
    gc: *mut __SlateRecord_pycore_interp_structs_h_170,
    next: *mut __SlateRecord_pycore_interp_structs_h_170,
) {
    unsafe {
        (*gc)._gc_next = next as u64;
    }
}

fn _PyGCHead_PREV(
    gc: *mut __SlateRecord_pycore_interp_structs_h_170,
) -> *mut __SlateRecord_pycore_interp_structs_h_170 {
    let prev: u64 = (unsafe { (*gc)._gc_prev }) & (1i32.wrapping_neg() as u64) << 2i32;
    prev as *mut __SlateRecord_pycore_interp_structs_h_170
}

fn _PyGCHead_SET_PREV(
    gc: *mut __SlateRecord_pycore_interp_structs_h_170,
    prev: *mut __SlateRecord_pycore_interp_structs_h_170,
) {
    let uprev: u64 = prev as u64;
    unsafe {
        (*gc)._gc_prev = (*gc)._gc_prev & !((1i32.wrapping_neg() as u64) << 2i32) | uprev;
    }
}

/// # Arguments
///
/// * `op` - The preprocessor removes _PyObject_ASSERT_FROM() calls if NDEBUG is defined
fn _PyObject_GC_TRACK(op: *mut _object) {
    _PyObject_ASSERT_FROM!();
    let gc: *mut __SlateRecord_pycore_interp_structs_h_170 = _Py_AS_GC(op);
    _PyObject_ASSERT_FROM!();
    let gcstate: *mut _gc_runtime_state =
        unsafe { core::ptr::addr_of_mut!((*_PyInterpreterState_GET()).gc) };
    let generation0: *mut __SlateRecord_pycore_interp_structs_h_170 =
        unsafe { (*gcstate).generation0 };
    let last: *mut __SlateRecord_pycore_interp_structs_h_170 =
        (unsafe { (*generation0)._gc_prev }) as *mut __SlateRecord_pycore_interp_structs_h_170;
    _PyGCHead_SET_NEXT(last, gc);
    _PyGCHead_SET_PREV(gc, last);
    _PyGCHead_SET_NEXT(gc, generation0);
    unsafe {
        (*generation0)._gc_prev = gc as u64;
    }
    let __v1: i64 = unsafe { (*gcstate).heap_size };
    unsafe {
        (*gcstate).heap_size = __v1.wrapping_add(1);
    }
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

/// # Arguments
///
/// * `op` - The preprocessor removes _PyObject_ASSERT_FROM() calls if NDEBUG is defined
fn _PyObject_GC_UNTRACK(op: *mut _object) {
    _PyObject_ASSERT_FROM!();
    let gc: *mut __SlateRecord_pycore_interp_structs_h_170 = _Py_AS_GC(op);
    let prev: *mut __SlateRecord_pycore_interp_structs_h_170 = _PyGCHead_PREV(gc);
    let next: *mut __SlateRecord_pycore_interp_structs_h_170 = _PyGCHead_NEXT(gc);
    _PyGCHead_SET_NEXT(prev, next);
    _PyGCHead_SET_PREV(next, prev);
    unsafe {
        (*gc)._gc_next = 0;
    }
    unsafe {
        (*gc)._gc_prev = (*gc)._gc_prev & 1;
    }
    let gcstate: *mut _gc_runtime_state =
        unsafe { core::ptr::addr_of_mut!((*_PyInterpreterState_GET()).gc) };
    let __v4: i64 = unsafe { (*gcstate).heap_size };
    unsafe {
        (*gcstate).heap_size = __v4.wrapping_sub(1);
    }
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
struct __SlateRecord_pycore_obmalloc_h_622 {
    tail_hi: i32,
    tail_lo: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct arena_map_bot {
    arenas: [__SlateRecord_pycore_obmalloc_h_622; 16384],
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
    pub struct __SlateBits208U0 {
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

static BaseException___reduce____doc__: __SlateAlign16<[i8; 26]> = __SlateAlign16([
    95i8, 95i8, 114i8, 101i8, 100i8, 117i8, 99i8, 101i8, 95i8, 95i8, 40i8, 36i8, 115i8, 101i8,
    108i8, 102i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.exceptions.BaseException___reduce__")]
extern "C-unwind" fn BaseException___reduce__(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    let mut return_value: *mut _object = core::ptr::null_mut::<_object>();
    return_value = BaseException___reduce___impl(self_ as *mut __SlateRecord_pyerrors_h_15);
    return_value
}

static BaseException___setstate____doc__: __SlateAlign16<[i8; 35]> = __SlateAlign16([
    95i8, 95i8, 115i8, 101i8, 116i8, 115i8, 116i8, 97i8, 116i8, 101i8, 95i8, 95i8, 40i8, 36i8,
    115i8, 101i8, 108i8, 102i8, 44i8, 32i8, 115i8, 116i8, 97i8, 116i8, 101i8, 44i8, 32i8, 47i8,
    41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.exceptions.BaseException___setstate__")]
extern "C-unwind" fn BaseException___setstate__(
    self_: *mut _object,
    state: *mut _object,
) -> *mut _object {
    let mut return_value: *mut _object = core::ptr::null_mut::<_object>();
    return_value =
        BaseException___setstate___impl(self_ as *mut __SlateRecord_pyerrors_h_15, state);
    return_value
}

static BaseException_with_traceback__doc__: __SlateAlign16<[i8; 79]> = __SlateAlign16([
    119i8, 105i8, 116i8, 104i8, 95i8, 116i8, 114i8, 97i8, 99i8, 101i8, 98i8, 97i8, 99i8, 107i8,
    40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8, 116i8, 98i8, 44i8, 32i8, 47i8, 41i8, 10i8,
    45i8, 45i8, 10i8, 10i8, 83i8, 101i8, 116i8, 32i8, 115i8, 101i8, 108i8, 102i8, 46i8, 95i8, 95i8,
    116i8, 114i8, 97i8, 99i8, 101i8, 98i8, 97i8, 99i8, 107i8, 95i8, 95i8, 32i8, 116i8, 111i8, 32i8,
    116i8, 98i8, 32i8, 97i8, 110i8, 100i8, 32i8, 114i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8,
    115i8, 101i8, 108i8, 102i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.exceptions.BaseException_with_traceback")]
extern "C-unwind" fn BaseException_with_traceback(
    self_: *mut _object,
    tb: *mut _object,
) -> *mut _object {
    let mut return_value: *mut _object = core::ptr::null_mut::<_object>();
    return_value = BaseException_with_traceback_impl(self_ as *mut __SlateRecord_pyerrors_h_15, tb);
    return_value
}

static BaseException_add_note__doc__: __SlateAlign16<[i8; 57]> = __SlateAlign16([
    97i8, 100i8, 100i8, 95i8, 110i8, 111i8, 116i8, 101i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8,
    44i8, 32i8, 110i8, 111i8, 116i8, 101i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8,
    65i8, 100i8, 100i8, 32i8, 97i8, 32i8, 110i8, 111i8, 116i8, 101i8, 32i8, 116i8, 111i8, 32i8,
    116i8, 104i8, 101i8, 32i8, 101i8, 120i8, 99i8, 101i8, 112i8, 116i8, 105i8, 111i8, 110i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.exceptions.BaseException_add_note")]
extern "C-unwind" fn BaseException_add_note(
    self_: *mut _object,
    arg: *mut _object,
) -> *mut _object {
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        if PyUnicode_Check!(arg) == 0 {
            _PyArg_BadArgument(
                c"add_note".as_ptr(),
                c"argument".as_ptr(),
                c"str".as_ptr(),
                arg,
            );
        } else {
            let note: *mut _object = arg;
            return_value =
                BaseException_add_note_impl(self_ as *mut __SlateRecord_pyerrors_h_15, note);
        }
        return return_value;
    }
}

#[unsafe(link_section = ".text.slate_distinct.exceptions.BaseException_args_get")]
extern "C-unwind" fn BaseException_args_get(
    self_: *mut _object,
    _unused_context: *mut (),
) -> *mut _object {
    let mut return_value: *mut _object = core::ptr::null_mut::<_object>();
    return_value = BaseException_args_get_impl(self_ as *mut __SlateRecord_pyerrors_h_15);
    return_value
}

#[unsafe(link_section = ".text.slate_distinct.exceptions.BaseException_args_set")]
extern "C-unwind" fn BaseException_args_set(
    self_: *mut _object,
    value: *mut _object,
    _unused_context: *mut (),
) -> i32 {
    let mut return_value: i32 = 0;
    return_value = BaseException_args_set_impl(self_ as *mut __SlateRecord_pyerrors_h_15, value);
    return_value
}

#[unsafe(link_section = ".text.slate_distinct.exceptions.BaseException___traceback___get")]
extern "C-unwind" fn BaseException___traceback___get(
    self_: *mut _object,
    _unused_context: *mut (),
) -> *mut _object {
    let mut return_value: *mut _object = core::ptr::null_mut::<_object>();
    return_value = BaseException___traceback___get_impl(self_ as *mut __SlateRecord_pyerrors_h_15);
    return_value
}

#[unsafe(link_section = ".text.slate_distinct.exceptions.BaseException___traceback___set")]
extern "C-unwind" fn BaseException___traceback___set(
    self_: *mut _object,
    value: *mut _object,
    _unused_context: *mut (),
) -> i32 {
    let mut return_value: i32 = 0;
    return_value =
        BaseException___traceback___set_impl(self_ as *mut __SlateRecord_pyerrors_h_15, value);
    return_value
}

#[unsafe(link_section = ".text.slate_distinct.exceptions.BaseException___context___get")]
extern "C-unwind" fn BaseException___context___get(
    self_: *mut _object,
    _unused_context: *mut (),
) -> *mut _object {
    let mut return_value: *mut _object = core::ptr::null_mut::<_object>();
    return_value = BaseException___context___get_impl(self_ as *mut __SlateRecord_pyerrors_h_15);
    return_value
}

#[unsafe(link_section = ".text.slate_distinct.exceptions.BaseException___context___set")]
extern "C-unwind" fn BaseException___context___set(
    self_: *mut _object,
    value: *mut _object,
    _unused_context: *mut (),
) -> i32 {
    let mut return_value: i32 = 0;
    return_value =
        BaseException___context___set_impl(self_ as *mut __SlateRecord_pyerrors_h_15, value);
    return_value
}

#[unsafe(link_section = ".text.slate_distinct.exceptions.BaseException___cause___get")]
extern "C-unwind" fn BaseException___cause___get(
    self_: *mut _object,
    _unused_context: *mut (),
) -> *mut _object {
    let mut return_value: *mut _object = core::ptr::null_mut::<_object>();
    return_value = BaseException___cause___get_impl(self_ as *mut __SlateRecord_pyerrors_h_15);
    return_value
}

#[unsafe(link_section = ".text.slate_distinct.exceptions.BaseException___cause___set")]
extern "C-unwind" fn BaseException___cause___set(
    self_: *mut _object,
    value: *mut _object,
    _unused_context: *mut (),
) -> i32 {
    let mut return_value: i32 = 0;
    return_value =
        BaseException___cause___set_impl(self_ as *mut __SlateRecord_pyerrors_h_15, value);
    return_value
}

static BaseExceptionGroup_derive__doc__: __SlateAlign16<[i8; 28]> = __SlateAlign16([
    100i8, 101i8, 114i8, 105i8, 118i8, 101i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8,
    101i8, 120i8, 99i8, 115i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.exceptions.BaseExceptionGroup_derive")]
extern "C-unwind" fn BaseExceptionGroup_derive(
    self_: *mut _object,
    excs: *mut _object,
) -> *mut _object {
    let mut return_value: *mut _object = core::ptr::null_mut::<_object>();
    return_value = BaseExceptionGroup_derive_impl(self_ as *mut __SlateRecord_pyerrors_h_22, excs);
    return_value
}

static BaseExceptionGroup_split__doc__: __SlateAlign16<[i8; 36]> = __SlateAlign16([
    115i8, 112i8, 108i8, 105i8, 116i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8, 109i8,
    97i8, 116i8, 99i8, 104i8, 101i8, 114i8, 95i8, 118i8, 97i8, 108i8, 117i8, 101i8, 44i8, 32i8,
    47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.exceptions.BaseExceptionGroup_split")]
extern "C-unwind" fn BaseExceptionGroup_split(
    self_: *mut _object,
    matcher_value: *mut _object,
) -> *mut _object {
    let mut return_value: *mut _object = core::ptr::null_mut::<_object>();
    return_value =
        BaseExceptionGroup_split_impl(self_ as *mut __SlateRecord_pyerrors_h_22, matcher_value);
    return_value
}

static BaseExceptionGroup_subgroup__doc__: __SlateAlign16<[i8; 39]> = __SlateAlign16([
    115i8, 117i8, 98i8, 103i8, 114i8, 111i8, 117i8, 112i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8,
    44i8, 32i8, 109i8, 97i8, 116i8, 99i8, 104i8, 101i8, 114i8, 95i8, 118i8, 97i8, 108i8, 117i8,
    101i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.exceptions.BaseExceptionGroup_subgroup")]
extern "C-unwind" fn BaseExceptionGroup_subgroup(
    self_: *mut _object,
    matcher_value: *mut _object,
) -> *mut _object {
    let mut return_value: *mut _object = core::ptr::null_mut::<_object>();
    return_value =
        BaseExceptionGroup_subgroup_impl(self_ as *mut __SlateRecord_pyerrors_h_22, matcher_value);
    return_value
}

// _PyObject_RealIsSubclass()
// _Py_EnterRecursiveCall
// struct _Py_exc_state
// _PyArg_NoKeywords()
// struct _PyErr_SetRaisedException
// _PyTuple_FromArray()
// SEP
// [clinic input]
// class BaseException "PyBaseExceptionObject *" "&PyExc_BaseException"
// class BaseExceptionGroup "PyBaseExceptionGroupObject *" "&PyExc_BaseExceptionGroup"
// [clinic start generated code]
//
// [clinic end generated code: output=da39a3ee5e6b4b0d input=b7c45e78cff8edc3]
/// Compatibility aliases
/// borrowed ref
#[unsafe(no_mangle)]
static mut PyExc_EnvironmentError: *mut _object = core::ptr::null_mut::<_object>();

/// borrowed ref
#[unsafe(no_mangle)]
static mut PyExc_IOError: *mut _object = core::ptr::null_mut::<_object>();

fn get_exc_state() -> *mut _Py_exc_state {
    let interp: *mut _is = _PyInterpreterState_GET();
    unsafe { core::ptr::addr_of_mut!((*interp).exc_state) }
}

// NOTE: If the exception class hierarchy changes, don't forget to update
// Lib/test/exception_hierarchy.txt
fn PyBaseExceptionObject_CAST(exc: *mut _object) -> *mut __SlateRecord_pyerrors_h_15 {
    exc as *mut __SlateRecord_pyerrors_h_15
}

/// BaseException
#[unsafe(link_section = ".text.slate_distinct.exceptions.BaseException_new")]
extern "C-unwind" fn BaseException_new(
    r#type: *mut _typeobject,
    args: *mut _object,
    kwds: *mut _object,
) -> *mut _object {
    let self_: *mut __SlateRecord_pyerrors_h_15 =
        (unsafe { unsafe { (*r#type).tp_alloc }.unwrap()(r#type, 0i64) })
            as *mut __SlateRecord_pyerrors_h_15;
    if self_.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    // the dict is created on the fly in PyObject_GenericSetAttr
    unsafe {
        (*self_).dict = core::ptr::null_mut::<_object>();
    }
    unsafe {
        (*self_).notes = core::ptr::null_mut::<_object>();
    }
    unsafe {
        (*self_).context = core::ptr::null_mut::<_object>();
    }
    unsafe {
        (*self_).cause = core::ptr::null_mut::<_object>();
    }
    unsafe {
        (*self_).traceback = core::ptr::null_mut::<_object>();
    }
    unsafe {
        (*self_).suppress_context = 0;
    }
    if !args.is_null() {
        unsafe {
            (*self_).args = Py_NewRef!(args);
        }
        return self_ as *mut _object;
    }
    unsafe {
        (*self_).args = PyTuple_New(0i64);
    }
    if unsafe { (*self_).args }.is_null() {
        Py_DECREF__2!(self_);
        return core::ptr::null_mut::<_object>();
    }
    self_ as *mut _object
}

#[unsafe(link_section = ".text.slate_distinct.exceptions.BaseException_init")]
extern "C-unwind" fn BaseException_init(
    op: *mut _object,
    args: *mut _object,
    kwds: *mut _object,
) -> i32 {
    let self_: *mut __SlateRecord_pyerrors_h_15 = PyBaseExceptionObject_CAST(op);
    if !_PyArg_NoKeywords__2!(unsafe { (*Py_TYPE__2!(self_)).tp_name }, kwds) {
        return 1i32.wrapping_neg();
    }
    Py_XSETREF!((*self_).args, Py_NewRef!(args));
    0
}

#[unsafe(link_section = ".text.slate_distinct.exceptions.BaseException_vectorcall")]
extern "C-unwind" fn BaseException_vectorcall(
    type_obj: *mut _object,
    args: *const *mut _object,
    nargsf: u64,
    kwnames: *mut _object,
) -> *mut _object {
    let r#type: *mut _typeobject = _PyType_CAST!(type_obj);
    if !_PyArg_NoKwnames__2!(unsafe { (*r#type).tp_name }, kwnames) {
        return core::ptr::null_mut::<_object>();
    }
    let self_: *mut __SlateRecord_pyerrors_h_15 =
        (unsafe { unsafe { (*r#type).tp_alloc }.unwrap()(r#type, 0i64) })
            as *mut __SlateRecord_pyerrors_h_15;
    if self_.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    // The dict is created on the fly in PyObject_GenericSetAttr()
    unsafe {
        (*self_).dict = core::ptr::null_mut::<_object>();
    }
    unsafe {
        (*self_).notes = core::ptr::null_mut::<_object>();
    }
    unsafe {
        (*self_).traceback = core::ptr::null_mut::<_object>();
    }
    unsafe {
        (*self_).cause = core::ptr::null_mut::<_object>();
    }
    unsafe {
        (*self_).context = core::ptr::null_mut::<_object>();
    }
    unsafe {
        (*self_).suppress_context = 0;
    }
    unsafe {
        (*self_).args = _PyTuple_FromArray(args, PyVectorcall_NARGS!(nargsf));
    }
    if unsafe { (*self_).args }.is_null() {
        Py_DECREF__2!(self_);
        return core::ptr::null_mut::<_object>();
    }
    self_ as *mut _object
}

#[unsafe(link_section = ".text.slate_distinct.exceptions.BaseException_clear")]
extern "C-unwind" fn BaseException_clear(op: *mut _object) -> i32 {
    let self_: *mut __SlateRecord_pyerrors_h_15 = PyBaseExceptionObject_CAST(op);
    Py_CLEAR!((*self_).dict);
    Py_CLEAR!((*self_).args);
    Py_CLEAR!((*self_).notes);
    Py_CLEAR!((*self_).traceback);
    Py_CLEAR!((*self_).cause);
    Py_CLEAR!((*self_).context);
    0
}

#[unsafe(link_section = ".text.slate_distinct.exceptions.BaseException_dealloc")]
extern "C-unwind" fn BaseException_dealloc(op: *mut _object) {
    let self_: *mut __SlateRecord_pyerrors_h_15 = PyBaseExceptionObject_CAST(op);
    unsafe { PyObject_GC_UnTrack(self_ as *mut ()) };
    // bpo-44348: The trashcan mechanism prevents stack overflow when deleting
    //
    // long chains of exceptions. For example, exceptions can be chained
    //
    // through the __context__ attributes or the __traceback__ attribute.
    BaseException_clear(op);
    unsafe { unsafe { (*Py_TYPE__2!(self_)).tp_free }.unwrap()(self_ as *mut ()) };
}

#[unsafe(link_section = ".text.slate_distinct.exceptions.BaseException_traverse")]
extern "C-unwind" fn BaseException_traverse(
    op: *mut _object,
    mut visit: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
    arg: *mut (),
) -> i32 {
    let self_: *mut __SlateRecord_pyerrors_h_15 = PyBaseExceptionObject_CAST(op);
    Py_VISIT!(unsafe { (*self_).dict }, visit, arg);
    Py_VISIT!(unsafe { (*self_).args }, visit, arg);
    Py_VISIT!(unsafe { (*self_).notes }, visit, arg);
    Py_VISIT!(unsafe { (*self_).traceback }, visit, arg);
    Py_VISIT!(unsafe { (*self_).cause }, visit, arg);
    Py_VISIT!(unsafe { (*self_).context }, visit, arg);
    0
}

#[unsafe(link_section = ".text.slate_distinct.exceptions.BaseException_str")]
extern "C-unwind" fn BaseException_str(op: *mut _object) -> *mut _object {
    let self_: *mut __SlateRecord_pyerrors_h_15 = PyBaseExceptionObject_CAST(op);
    let mut res: *mut _object = unsafe { core::mem::zeroed() };
    match PyTuple_GET_SIZE!(unsafe { (*self_).args }) {
        0 => {
            res = unsafe { Py_GetConstant(7u32) };
        }
        1 => {
            res = unsafe { PyObject_Str(PyTuple_GET_ITEM!(unsafe { (*self_).args }, 0i32)) };
        }
        _ => {
            res = unsafe { PyObject_Str((*self_).args) };
        }
    }
    res
}

#[unsafe(link_section = ".text.slate_distinct.exceptions.BaseException_repr")]
extern "C-unwind" fn BaseException_repr(op: *mut _object) -> *mut _object {
    let self_: *mut __SlateRecord_pyerrors_h_15 = PyBaseExceptionObject_CAST(op);
    let mut res: *mut _object = unsafe { core::mem::zeroed() };
    let name: *const i8 = unsafe { _PyType_Name(Py_TYPE__2!(self_)) };
    if PyTuple_GET_SIZE!(unsafe { (*self_).args }) == 1 {
        res = unsafe {
            PyUnicode_FromFormat(
                c"%s(%R)".as_ptr(),
                name,
                PyTuple_GET_ITEM!(unsafe { (*self_).args }, 0i32),
            )
        };
    } else {
        res = unsafe { PyUnicode_FromFormat(c"%s%R".as_ptr(), name, (*self_).args) };
    }
    res
}

// Pickling support
// [clinic input]
// @critical_section
// BaseException.__reduce__
// [clinic start generated code]
/// [clinic end generated code: output=af87c1247ef98748 input=283be5a10d9c964f]
fn BaseException___reduce___impl(self_: *mut __SlateRecord_pyerrors_h_15) -> *mut _object {
    if !unsafe { (*self_).args }.is_null() && !unsafe { (*self_).dict }.is_null() {
        return unsafe { PyTuple_Pack(3i64, Py_TYPE__2!(self_), (*self_).args, (*self_).dict) };
    } else {
        return unsafe { PyTuple_Pack(2i64, Py_TYPE__2!(self_), (*self_).args) };
    }
}

// Needed for backward compatibility, since exceptions used to store
// all their attributes in the __dict__. Code is taken from cPickle's
// load_build function.
// [clinic input]
// @critical_section state
// BaseException.__setstate__
//     state: object
//     /
// [clinic start generated code]
/// [clinic end generated code: output=f3834889950453ab input=f9b1aea70382cdb6]
fn BaseException___setstate___impl(
    self_: *mut __SlateRecord_pyerrors_h_15,
    state: *mut _object,
) -> *mut _object {
    let mut d_key: *mut _object = unsafe { core::mem::zeroed() };
    let mut d_value: *mut _object = unsafe { core::mem::zeroed() };
    let mut i: i64 = 0;
    if state != unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } {
        if PyDict_Check!(state) == 0 {
            unsafe { PyErr_SetString(PyExc_TypeError, c"state is not a dictionary".as_ptr()) };
            return core::ptr::null_mut::<_object>();
        }
        while (unsafe {
            PyDict_Next(
                state,
                core::ptr::addr_of_mut!(i),
                core::ptr::addr_of_mut!(d_key),
                core::ptr::addr_of_mut!(d_value),
            )
        }) != 0
        {
            Py_INCREF!(d_key);
            Py_INCREF!(d_value);
            let res: i32 = unsafe { PyObject_SetAttr(self_ as *mut _object, d_key, d_value) };
            Py_DECREF!(d_value);
            Py_DECREF!(d_key);
            if res < 0 {
                return core::ptr::null_mut::<_object>();
            }
        }
    }
    unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
}

// [clinic input]
// @critical_section
// BaseException.with_traceback
//     tb: object
//     /
//
// Set self.__traceback__ to tb and return self.
// [clinic start generated code]
/// [clinic end generated code: output=81e92f2387927f10 input=b5fb64d834717e36]
fn BaseException_with_traceback_impl(
    self_: *mut __SlateRecord_pyerrors_h_15,
    tb: *mut _object,
) -> *mut _object {
    if BaseException___traceback___set_impl(self_, tb) < 0 {
        return core::ptr::null_mut::<_object>();
    }
    Py_NewRef__2!(self_)
}

// [clinic input]
// @critical_section
// BaseException.add_note
//     note: object(subclass_of="&PyUnicode_Type")
//     /
//
// Add a note to the exception
// [clinic start generated code]
/// [clinic end generated code: output=fb7cbcba611c187b input=e60a6b6e9596acaf]
fn BaseException_add_note_impl(
    self_: *mut __SlateRecord_pyerrors_h_15,
    note: *mut _object,
) -> *mut _object {
    let mut notes: *mut _object = unsafe { core::mem::zeroed() };
    if (unsafe {
        PyObject_GetOptionalAttr(
            self_ as *mut _object,
            core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py___notes__
                    ._ascii
                    .ob_base
            ),
            core::ptr::addr_of_mut!(notes),
        )
    }) < 0
    {
        return core::ptr::null_mut::<_object>();
    }
    if notes.is_null() {
        notes = unsafe { PyList_New(0i64) };
        if notes.is_null() {
            return core::ptr::null_mut::<_object>();
        }
        if (unsafe {
            PyObject_SetAttr(
                self_ as *mut _object,
                core::ptr::addr_of_mut!(
                    _PyRuntime
                        .static_objects
                        .singletons
                        .strings
                        .identifiers
                        ._py___notes__
                        ._ascii
                        .ob_base
                ),
                notes,
            )
        }) < 0
        {
            Py_DECREF!(notes);
            return core::ptr::null_mut::<_object>();
        }
    } else {
        if PyList_Check!(notes) == 0 {
            Py_DECREF!(notes);
            unsafe {
                PyErr_SetString(
                    PyExc_TypeError,
                    c"Cannot add note: __notes__ is not a list".as_ptr(),
                )
            };
            return core::ptr::null_mut::<_object>();
        }
    }
    if (unsafe { PyList_Append(notes, note) }) < 0 {
        Py_DECREF!(notes);
        return core::ptr::null_mut::<_object>();
    }
    Py_DECREF!(notes);
    unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
}

static mut BaseException_methods: __SlateAlign16<[PyMethodDef; 5]> = __SlateAlign16([
    {
        let mut __t0: PyMethodDef = unsafe { core::mem::zeroed() };
        __t0.ml_name = (b"__reduce__\0".as_ptr() as *mut i8) as *const i8;
        __t0.ml_meth = Some(BaseException___reduce__);
        __t0.ml_flags = 4i32;
        __t0.ml_doc =
            unsafe { core::ptr::addr_of!(BaseException___reduce____doc__.0) as *const i8 };
        __t0
    },
    {
        let mut __t1: PyMethodDef = unsafe { core::mem::zeroed() };
        __t1.ml_name = (b"__setstate__\0".as_ptr() as *mut i8) as *const i8;
        __t1.ml_meth = Some(BaseException___setstate__);
        __t1.ml_flags = 8i32;
        __t1.ml_doc =
            unsafe { core::ptr::addr_of!(BaseException___setstate____doc__.0) as *const i8 };
        __t1
    },
    {
        let mut __t2: PyMethodDef = unsafe { core::mem::zeroed() };
        __t2.ml_name = (b"with_traceback\0".as_ptr() as *mut i8) as *const i8;
        __t2.ml_meth = Some(BaseException_with_traceback);
        __t2.ml_flags = 8i32;
        __t2.ml_doc =
            unsafe { core::ptr::addr_of!(BaseException_with_traceback__doc__.0) as *const i8 };
        __t2
    },
    {
        let mut __t3: PyMethodDef = unsafe { core::mem::zeroed() };
        __t3.ml_name = (b"add_note\0".as_ptr() as *mut i8) as *const i8;
        __t3.ml_meth = Some(BaseException_add_note);
        __t3.ml_flags = 8i32;
        __t3.ml_doc = unsafe { core::ptr::addr_of!(BaseException_add_note__doc__.0) as *const i8 };
        __t3
    },
    {
        let mut __t4: PyMethodDef = unsafe { core::mem::zeroed() };
        __t4.ml_name = core::ptr::null::<i8>();
        __t4.ml_meth = None;
        __t4.ml_flags = 0i32;
        __t4.ml_doc = core::ptr::null::<i8>();
        __t4
    },
]);

// [clinic input]
// @critical_section
// @getter
// BaseException.args
// [clinic start generated code]
/// [clinic end generated code: output=e02e34e35cf4d677 input=64282386e4d7822d]
fn BaseException_args_get_impl(self_: *mut __SlateRecord_pyerrors_h_15) -> *mut _object {
    if unsafe { (*self_).args }.is_null() {
        return unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) };
    }
    Py_NewRef!(unsafe { (*self_).args })
}

// [clinic input]
// @critical_section
// @setter
// @deleter
// BaseException.args
// [clinic start generated code]
/// [clinic end generated code: output=331137e11d8f9e80 input=177ad350c8b45219]
fn BaseException_args_set_impl(
    self_: *mut __SlateRecord_pyerrors_h_15,
    value: *mut _object,
) -> i32 {
    if value.is_null() {
        unsafe { PyErr_SetString(PyExc_TypeError, c"args may not be deleted".as_ptr()) };
        return 1i32.wrapping_neg();
    }
    let seq: *mut _object = unsafe { PySequence_Tuple(value) };
    if seq.is_null() {
        return 1i32.wrapping_neg();
    }
    Py_XSETREF!((*self_).args, seq);
    0
}

// [clinic input]
// @critical_section
// @getter
// BaseException.__traceback__
// [clinic start generated code]
/// [clinic end generated code: output=17cf874a52339398 input=a2277f0de62170cf]
fn BaseException___traceback___get_impl(self_: *mut __SlateRecord_pyerrors_h_15) -> *mut _object {
    if unsafe { (*self_).traceback }.is_null() {
        return unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) };
    }
    Py_NewRef!(unsafe { (*self_).traceback })
}

// [clinic input]
// @critical_section
// @setter
// @deleter
// BaseException.__traceback__
// [clinic start generated code]
/// [clinic end generated code: output=a82c86d9f29f48f0 input=53a1df586023d786]
fn BaseException___traceback___set_impl(
    self_: *mut __SlateRecord_pyerrors_h_15,
    value: *mut _object,
) -> i32 {
    if value.is_null() {
        unsafe {
            PyErr_SetString(
                PyExc_TypeError,
                c"__traceback__ may not be deleted".as_ptr(),
            )
        };
        return 1i32.wrapping_neg();
    }
    if PyTraceBack_Check!(value) != 0 {
        Py_XSETREF!((*self_).traceback, Py_NewRef!(value));
    } else {
        if value == unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } {
            Py_CLEAR!((*self_).traceback);
        } else {
            unsafe {
                PyErr_SetString(
                    PyExc_TypeError,
                    c"__traceback__ must be a traceback or None".as_ptr(),
                )
            };
            return 1i32.wrapping_neg();
        }
    }
    0
}

// [clinic input]
// @critical_section
// @getter
// BaseException.__context__
// [clinic start generated code]
/// [clinic end generated code: output=6ec5d296ce8d1c93 input=b2d22687937e66ab]
fn BaseException___context___get_impl(self_: *mut __SlateRecord_pyerrors_h_15) -> *mut _object {
    if unsafe { (*self_).context }.is_null() {
        return unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) };
    }
    Py_NewRef!(unsafe { (*self_).context })
}

// [clinic input]
// @critical_section
// @setter
// @deleter
// BaseException.__context__
// [clinic start generated code]
/// [clinic end generated code: output=b4cb52dcca1da3bd input=fe79e7c0a0854004]
fn BaseException___context___set_impl(
    self_: *mut __SlateRecord_pyerrors_h_15,
    mut value: *mut _object,
) -> i32 {
    if value.is_null() {
        unsafe { PyErr_SetString(PyExc_TypeError, c"__context__ may not be deleted".as_ptr()) };
        return 1i32.wrapping_neg();
    } else {
        if value == unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } {
            value = core::ptr::null_mut::<_object>();
        } else {
            if PyExceptionInstance_Check!(value) == 0 {
                unsafe {
                    PyErr_SetString(
                        PyExc_TypeError,
                        c"exception context must be None or derive from BaseException".as_ptr(),
                    )
                };
                return 1i32.wrapping_neg();
            } else {
                Py_INCREF!(value);
            }
        }
    }
    Py_XSETREF!((*self_).context, value);
    0
}

// [clinic input]
// @critical_section
// @getter
// BaseException.__cause__
// [clinic start generated code]
/// [clinic end generated code: output=987f6c4d8a0bdbab input=40e0eac427b6e602]
fn BaseException___cause___get_impl(self_: *mut __SlateRecord_pyerrors_h_15) -> *mut _object {
    if unsafe { (*self_).cause }.is_null() {
        return unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) };
    }
    Py_NewRef!(unsafe { (*self_).cause })
}

// [clinic input]
// @critical_section
// @setter
// @deleter
// BaseException.__cause__
// [clinic start generated code]
/// [clinic end generated code: output=6161315398aaf541 input=3fdd9a0d1674abc9]
fn BaseException___cause___set_impl(
    self_: *mut __SlateRecord_pyerrors_h_15,
    mut value: *mut _object,
) -> i32 {
    if value.is_null() {
        unsafe { PyErr_SetString(PyExc_TypeError, c"__cause__ may not be deleted".as_ptr()) };
        return 1i32.wrapping_neg();
    } else {
        if value == unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } {
            value = core::ptr::null_mut::<_object>();
        } else {
            if PyExceptionInstance_Check!(value) == 0 {
                unsafe {
                    PyErr_SetString(
                        PyExc_TypeError,
                        c"exception cause must be None or derive from BaseException".as_ptr(),
                    )
                };
                return 1i32.wrapping_neg();
            } else {
                // PyException_SetCause steals this reference
                Py_INCREF!(value);
            }
        }
    }
    PyException_SetCause(self_ as *mut _object, value);
    0
}

static mut BaseException_getset: __SlateAlign16<[PyGetSetDef; 6]> = __SlateAlign16([
    {
        let mut __t0: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t0.name = (b"__dict__\0".as_ptr() as *mut i8) as *const i8;
        __t0.get = unsafe {
            core::mem::transmute::<
                *const (),
                Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> *mut _object>,
            >(PyObject_GenericGetDict as *const ())
        };
        __t0.set = unsafe {
            core::mem::transmute::<
                *const (),
                Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object, *mut ()) -> i32>,
            >(PyObject_GenericSetDict as *const ())
        };
        __t0
    },
    {
        let mut __t1: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t1.name = (b"args\0".as_ptr() as *mut i8) as *const i8;
        __t1.get = Some(BaseException_args_get);
        __t1.set = Some(BaseException_args_set);
        __t1.doc = core::ptr::null::<i8>();
        __t1
    },
    {
        let mut __t2: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t2.name = (b"__traceback__\0".as_ptr() as *mut i8) as *const i8;
        __t2.get = Some(BaseException___traceback___get);
        __t2.set = Some(BaseException___traceback___set);
        __t2.doc = core::ptr::null::<i8>();
        __t2
    },
    {
        let mut __t3: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t3.name = (b"__context__\0".as_ptr() as *mut i8) as *const i8;
        __t3.get = Some(BaseException___context___get);
        __t3.set = Some(BaseException___context___set);
        __t3.doc = core::ptr::null::<i8>();
        __t3
    },
    {
        let mut __t4: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t4.name = (b"__cause__\0".as_ptr() as *mut i8) as *const i8;
        __t4.get = Some(BaseException___cause___get);
        __t4.set = Some(BaseException___cause___set);
        __t4.doc = core::ptr::null::<i8>();
        __t4
    },
    {
        let mut __t5: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t5.name = core::ptr::null::<i8>();
        __t5
    },
]);

#[unsafe(no_mangle)]
extern "C-unwind" fn PyException_GetTraceback(self_: *mut _object) -> *mut _object {
    let traceback: *mut _object =
        Py_XNewRef!(unsafe { (*PyBaseExceptionObject_CAST(self_)).traceback });
    traceback
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyException_SetTraceback(self_: *mut _object, tb: *mut _object) -> i32 {
    let mut res: i32 = 0;
    res = BaseException___traceback___set_impl(PyBaseExceptionObject_CAST(self_), tb);
    res
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyException_GetCause(self_: *mut _object) -> *mut _object {
    let cause: *mut _object = Py_XNewRef!(unsafe { (*PyBaseExceptionObject_CAST(self_)).cause });
    cause
}

/// Steals a reference to cause
#[unsafe(no_mangle)]
extern "C-unwind" fn PyException_SetCause(self_: *mut _object, cause: *mut _object) {
    let base_self: *mut __SlateRecord_pyerrors_h_15 = PyBaseExceptionObject_CAST(self_);
    unsafe {
        (*base_self).suppress_context = 1;
    }
    Py_XSETREF!((*base_self).cause, cause);
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyException_GetContext(self_: *mut _object) -> *mut _object {
    let context: *mut _object =
        Py_XNewRef!(unsafe { (*PyBaseExceptionObject_CAST(self_)).context });
    context
}

/// Steals a reference to context
#[unsafe(no_mangle)]
extern "C-unwind" fn PyException_SetContext(self_: *mut _object, context: *mut _object) {
    Py_XSETREF!((*PyBaseExceptionObject_CAST(self_)).context, context);
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyException_GetArgs(self_: *mut _object) -> *mut _object {
    let args: *mut _object = Py_NewRef!(unsafe { (*PyBaseExceptionObject_CAST(self_)).args });
    args
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyException_SetArgs(self_: *mut _object, args: *mut _object) {
    Py_INCREF!(args);
    Py_XSETREF!((*PyBaseExceptionObject_CAST(self_)).args, args);
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyExceptionClass_Name(ob: *mut _object) -> *const i8 {
    unsafe { (*(ob as *mut _typeobject)).tp_name }
}

static mut BaseException_members: __SlateAlign16<[PyMemberDef; 2]> = __SlateAlign16([
    {
        let mut __t0: PyMemberDef = unsafe { core::mem::zeroed() };
        __t0.name = (b"__suppress_context__\0".as_ptr() as *mut i8) as *const i8;
        __t0.r#type = 14i32;
        __t0.offset =
            (core::mem::offset_of!(__SlateRecord_pyerrors_h_15, suppress_context) as u64) as i64;
        __t0
    },
    {
        let mut __t1: PyMemberDef = unsafe { core::mem::zeroed() };
        __t1.name = core::ptr::null::<i8>();
        __t1
    },
]);

/// tp_name
/// tp_basicsize
/// tp_itemsize
/// tp_dealloc
/// tp_vectorcall_offset
/// tp_getattr
/// tp_setattr
/// tp_as_async
/// tp_repr
/// tp_as_number
/// tp_as_sequence
/// tp_as_mapping
/// tp_hash
/// tp_call
/// tp_str
/// tp_getattro
/// tp_setattro
/// tp_as_buffer
/// tp_flags
/// tp_doc
/// tp_traverse
/// tp_clear
/// tp_richcompare
/// tp_weaklistoffset
/// tp_iter
/// tp_iternext
/// tp_methods
/// tp_members
/// tp_getset
/// tp_base
/// tp_dict
/// tp_descr_get
/// tp_descr_set
/// tp_dictoffset
/// tp_init
/// tp_alloc
/// tp_new
static mut _PyExc_BaseException: _typeobject = {
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
    __t0.tp_name = (b"BaseException\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pyerrors_h_15>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(BaseException_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = Some(BaseException_repr);
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = Some(BaseException_str);
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
    __t0.tp_flags = 1073759232u64;
    __t0.tp_doc = (b"Common base class for all exceptions\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_traverse = Some(BaseException_traverse);
    __t0.tp_clear = Some(BaseException_clear);
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods =
        unsafe { core::ptr::addr_of_mut!(BaseException_methods.0) as *mut PyMethodDef };
    __t0.tp_members =
        unsafe { core::ptr::addr_of_mut!(BaseException_members.0) as *mut PyMemberDef };
    __t0.tp_getset = unsafe { core::ptr::addr_of_mut!(BaseException_getset.0) as *mut PyGetSetDef };
    __t0.tp_base = core::ptr::null_mut::<_typeobject>();
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_15, dict) as u64) as i64;
    __t0.tp_init = Some(BaseException_init);
    __t0.tp_alloc = None;
    __t0.tp_new = Some(BaseException_new);
    __t0.tp_vectorcall = Some(BaseException_vectorcall);
    __t0
};

/// the CPython API expects exceptions to be (PyObject *) - both a hold-over
/// from the previous implementation and also allowing Python objects to be used
/// in the API
#[unsafe(no_mangle)]
static mut PyExc_BaseException: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_BaseException) }) as *mut _object;

// note these macros omit the last semicolon so the macro invocation may
// include it and not look strange.
// Exception extends BaseException
static mut _PyExc_Exception: _typeobject = {
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
    __t0.tp_name = (b"Exception\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pyerrors_h_15>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(BaseException_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = None;
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = None;
    __t0.tp_getattro = None;
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 17408u64;
    __t0.tp_doc =
        (b"Common base class for all non-exit exceptions.\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_traverse = Some(BaseException_traverse);
    __t0.tp_clear = Some(BaseException_clear);
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = core::ptr::null_mut::<PyMethodDef>();
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(_PyExc_BaseException) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_15, dict) as u64) as i64;
    __t0.tp_init = Some(BaseException_init);
    __t0.tp_alloc = None;
    __t0.tp_new = Some(BaseException_new);
    __t0
};

#[unsafe(no_mangle)]
static mut PyExc_Exception: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_Exception) }) as *mut _object;

// TypeError extends Exception
static mut _PyExc_TypeError: _typeobject = {
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
    __t0.tp_name = (b"TypeError\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pyerrors_h_15>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(BaseException_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = None;
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = None;
    __t0.tp_getattro = None;
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 17408u64;
    __t0.tp_doc = (b"Inappropriate argument type.\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_traverse = Some(BaseException_traverse);
    __t0.tp_clear = Some(BaseException_clear);
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = core::ptr::null_mut::<PyMethodDef>();
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(_PyExc_Exception) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_15, dict) as u64) as i64;
    __t0.tp_init = Some(BaseException_init);
    __t0.tp_alloc = None;
    __t0.tp_new = Some(BaseException_new);
    __t0
};

#[unsafe(no_mangle)]
static mut PyExc_TypeError: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_TypeError) }) as *mut _object;

// StopAsyncIteration extends Exception
static mut _PyExc_StopAsyncIteration: _typeobject = {
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
    __t0.tp_name = (b"StopAsyncIteration\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pyerrors_h_15>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(BaseException_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = None;
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = None;
    __t0.tp_getattro = None;
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 17408u64;
    __t0.tp_doc = (b"Signal the end from iterator.__anext__().\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_traverse = Some(BaseException_traverse);
    __t0.tp_clear = Some(BaseException_clear);
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = core::ptr::null_mut::<PyMethodDef>();
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(_PyExc_Exception) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_15, dict) as u64) as i64;
    __t0.tp_init = Some(BaseException_init);
    __t0.tp_alloc = None;
    __t0.tp_new = Some(BaseException_new);
    __t0
};

#[unsafe(no_mangle)]
static mut PyExc_StopAsyncIteration: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_StopAsyncIteration) }) as *mut _object;

// StopIteration extends Exception
/// Sentinel
static mut StopIteration_members: __SlateAlign16<[PyMemberDef; 2]> = __SlateAlign16([
    {
        let mut __t0: PyMemberDef = unsafe { core::mem::zeroed() };
        __t0.name = (b"value\0".as_ptr() as *mut i8) as *const i8;
        __t0.r#type = 6i32;
        __t0.offset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_74, value) as u64) as i64;
        __t0.flags = 0i32;
        __t0.doc = (b"generator return value\0".as_ptr() as *mut i8) as *const i8;
        __t0
    },
    {
        let mut __t1: PyMemberDef = unsafe { core::mem::zeroed() };
        __t1.name = core::ptr::null::<i8>();
        __t1
    },
]);

fn PyStopIterationObject_CAST(self_: *mut _object) -> *mut __SlateRecord_pyerrors_h_74 {
    self_ as *mut __SlateRecord_pyerrors_h_74
}

#[unsafe(link_section = ".text.slate_distinct.exceptions.StopIteration_init")]
extern "C-unwind" fn StopIteration_init(
    op: *mut _object,
    args: *mut _object,
    kwds: *mut _object,
) -> i32 {
    let size: i64 = PyTuple_GET_SIZE!(args);
    let mut value: *mut _object = unsafe { core::mem::zeroed() };
    if BaseException_init(op, args, kwds) == 1i32.wrapping_neg() {
        return 1i32.wrapping_neg();
    }
    let self_: *mut __SlateRecord_pyerrors_h_74 = PyStopIterationObject_CAST(op);
    Py_CLEAR!((*self_).value);
    if size > 0 {
        value = PyTuple_GET_ITEM!(args, 0i32);
    } else {
        value = unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) };
    }
    unsafe {
        (*self_).value = Py_NewRef!(value);
    }
    0
}

#[unsafe(link_section = ".text.slate_distinct.exceptions.StopIteration_clear")]
extern "C-unwind" fn StopIteration_clear(op: *mut _object) -> i32 {
    let self_: *mut __SlateRecord_pyerrors_h_74 = PyStopIterationObject_CAST(op);
    Py_CLEAR!((*self_).value);
    BaseException_clear(op)
}

#[unsafe(link_section = ".text.slate_distinct.exceptions.StopIteration_dealloc")]
extern "C-unwind" fn StopIteration_dealloc(self_: *mut _object) {
    unsafe { PyObject_GC_UnTrack(self_ as *mut ()) };
    StopIteration_clear(self_);
    unsafe { unsafe { (*Py_TYPE!(self_)).tp_free }.unwrap()(self_ as *mut ()) };
}

#[unsafe(link_section = ".text.slate_distinct.exceptions.StopIteration_traverse")]
extern "C-unwind" fn StopIteration_traverse(
    op: *mut _object,
    mut visit: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
    arg: *mut (),
) -> i32 {
    let self_: *mut __SlateRecord_pyerrors_h_74 = PyStopIterationObject_CAST(op);
    Py_VISIT!(unsafe { (*self_).value }, visit, arg);
    BaseException_traverse(op, visit, arg)
}

static mut _PyExc_StopIteration: _typeobject = {
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
    __t0.tp_name = (b"StopIteration\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pyerrors_h_74>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(StopIteration_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = None;
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = None;
    __t0.tp_getattro = None;
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 17408u64;
    __t0.tp_doc = (b"Signal the end from iterator.__next__().\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_traverse = Some(StopIteration_traverse);
    __t0.tp_clear = Some(StopIteration_clear);
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = core::ptr::null_mut::<PyMethodDef>();
    __t0.tp_members =
        unsafe { core::ptr::addr_of_mut!(StopIteration_members.0) as *mut PyMemberDef };
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(_PyExc_Exception) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_74, dict) as u64) as i64;
    __t0.tp_init = Some(StopIteration_init);
    __t0.tp_alloc = None;
    __t0.tp_new = None;
    __t0
};

#[unsafe(no_mangle)]
static mut PyExc_StopIteration: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_StopIteration) }) as *mut _object;

// GeneratorExit extends BaseException
static mut _PyExc_GeneratorExit: _typeobject = {
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
    __t0.tp_name = (b"GeneratorExit\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pyerrors_h_15>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(BaseException_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = None;
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = None;
    __t0.tp_getattro = None;
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 17408u64;
    __t0.tp_doc = (b"Request that a generator exit.\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_traverse = Some(BaseException_traverse);
    __t0.tp_clear = Some(BaseException_clear);
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = core::ptr::null_mut::<PyMethodDef>();
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(_PyExc_BaseException) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_15, dict) as u64) as i64;
    __t0.tp_init = Some(BaseException_init);
    __t0.tp_alloc = None;
    __t0.tp_new = Some(BaseException_new);
    __t0
};

#[unsafe(no_mangle)]
static mut PyExc_GeneratorExit: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_GeneratorExit) }) as *mut _object;

// SystemExit extends BaseException
fn PySystemExitObject_CAST(self_: *mut _object) -> *mut __SlateRecord_pyerrors_h_57 {
    self_ as *mut __SlateRecord_pyerrors_h_57
}

#[unsafe(link_section = ".text.slate_distinct.exceptions.SystemExit_init")]
extern "C-unwind" fn SystemExit_init(
    op: *mut _object,
    args: *mut _object,
    kwds: *mut _object,
) -> i32 {
    let size: i64 = PyTuple_GET_SIZE!(args);
    if BaseException_init(op, args, kwds) == 1i32.wrapping_neg() {
        return 1i32.wrapping_neg();
    }
    let self_: *mut __SlateRecord_pyerrors_h_57 = PySystemExitObject_CAST(op);
    if size == 0 {
        return 0;
    }
    if size == 1 {
        Py_XSETREF!((*self_).code, Py_NewRef!(PyTuple_GET_ITEM!(args, 0i32)));
    } else {
        // size > 1
        Py_XSETREF!((*self_).code, Py_NewRef!(args));
    }
    0
}

#[unsafe(link_section = ".text.slate_distinct.exceptions.SystemExit_clear")]
extern "C-unwind" fn SystemExit_clear(op: *mut _object) -> i32 {
    let self_: *mut __SlateRecord_pyerrors_h_57 = PySystemExitObject_CAST(op);
    Py_CLEAR!((*self_).code);
    BaseException_clear(op)
}

#[unsafe(link_section = ".text.slate_distinct.exceptions.SystemExit_dealloc")]
extern "C-unwind" fn SystemExit_dealloc(self_: *mut _object) {
    _PyObject_GC_UNTRACK!(self_);
    SystemExit_clear(self_);
    unsafe { unsafe { (*Py_TYPE!(self_)).tp_free }.unwrap()(self_ as *mut ()) };
}

#[unsafe(link_section = ".text.slate_distinct.exceptions.SystemExit_traverse")]
extern "C-unwind" fn SystemExit_traverse(
    op: *mut _object,
    mut visit: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
    arg: *mut (),
) -> i32 {
    let self_: *mut __SlateRecord_pyerrors_h_57 = PySystemExitObject_CAST(op);
    Py_VISIT!(unsafe { (*self_).code }, visit, arg);
    BaseException_traverse(op, visit, arg)
}

/// Sentinel
static mut SystemExit_members: __SlateAlign16<[PyMemberDef; 2]> = __SlateAlign16([
    {
        let mut __t0: PyMemberDef = unsafe { core::mem::zeroed() };
        __t0.name = (b"code\0".as_ptr() as *mut i8) as *const i8;
        __t0.r#type = 6i32;
        __t0.offset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_57, code) as u64) as i64;
        __t0.flags = 0i32;
        __t0.doc = (b"exception code\0".as_ptr() as *mut i8) as *const i8;
        __t0
    },
    {
        let mut __t1: PyMemberDef = unsafe { core::mem::zeroed() };
        __t1.name = core::ptr::null::<i8>();
        __t1
    },
]);

static mut _PyExc_SystemExit: _typeobject = {
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
    __t0.tp_name = (b"SystemExit\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pyerrors_h_57>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(SystemExit_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = None;
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = None;
    __t0.tp_getattro = None;
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 17408u64;
    __t0.tp_doc = (b"Request to exit from the interpreter.\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_traverse = Some(SystemExit_traverse);
    __t0.tp_clear = Some(SystemExit_clear);
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = core::ptr::null_mut::<PyMethodDef>();
    __t0.tp_members = unsafe { core::ptr::addr_of_mut!(SystemExit_members.0) as *mut PyMemberDef };
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(_PyExc_BaseException) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_57, dict) as u64) as i64;
    __t0.tp_init = Some(SystemExit_init);
    __t0.tp_alloc = None;
    __t0.tp_new = None;
    __t0
};

#[unsafe(no_mangle)]
static mut PyExc_SystemExit: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_SystemExit) }) as *mut _object;

// BaseExceptionGroup extends BaseException
// ExceptionGroup extends BaseExceptionGroup and Exception
fn PyBaseExceptionGroupObject_CAST(exc: *mut _object) -> *mut __SlateRecord_pyerrors_h_22 {
    exc as *mut __SlateRecord_pyerrors_h_22
}

#[unsafe(link_section = ".text.slate_distinct.exceptions.BaseExceptionGroup_new")]
extern "C-unwind" fn BaseExceptionGroup_new(
    r#type: *mut _typeobject,
    args: *mut _object,
    kwds: *mut _object,
) -> *mut _object {
    let mut self_: *mut __SlateRecord_pyerrors_h_22 = unsafe { core::mem::zeroed() };
    let mut nonbase: i32 = unsafe { core::mem::zeroed() };
    let mut cls: *mut _typeobject = unsafe { core::mem::zeroed() };
    let mut is_nonbase_exception: i32 = unsafe { core::mem::zeroed() };
    let mut exc: *mut _object = unsafe { core::mem::zeroed() };
    let mut i: i64 = unsafe { core::mem::zeroed() };
    let mut nested_base_exceptions: bool = unsafe { core::mem::zeroed() };
    let mut numexcs: i64 = unsafe { core::mem::zeroed() };
    // Save initial exceptions sequence as a string in case sequence is mutated
    let mut __v10: bool = unsafe { core::mem::zeroed() };
    let mut exceptions_str: *mut _object = unsafe { core::mem::zeroed() };
    let mut exceptions: *mut _object = unsafe { core::mem::zeroed() };
    let mut message: *mut _object = unsafe { core::mem::zeroed() };
    let mut PyExc_ExceptionGroup: *mut _typeobject = unsafe { core::mem::zeroed() };
    unsafe {
        let state: *mut _Py_exc_state = get_exc_state();
        PyExc_ExceptionGroup = (*state).PyExc_ExceptionGroup as *mut _typeobject;
        message = core::ptr::null_mut::<_object>();
        exceptions = core::ptr::null_mut::<_object>();
        exceptions_str = core::ptr::null_mut::<_object>();
        if PyArg_ParseTuple(
            args,
            c"UO:BaseExceptionGroup.__new__".as_ptr(),
            core::ptr::addr_of_mut!(message),
            core::ptr::addr_of_mut!(exceptions),
        ) == 0
        {
            return core::ptr::null_mut::<_object>();
        } else {
            if PySequence_Check(exceptions) == 0 {
                PyErr_SetString(
                    PyExc_TypeError,
                    c"second argument (exceptions) must be a sequence".as_ptr(),
                );
                return core::ptr::null_mut::<_object>();
            } else {
                if PyList_Check!(exceptions) == 0 {
                    __v10 = PyTuple_Check!(exceptions) == 0;
                } else {
                    __v10 = false;
                }
                '__join_0: {
                    if __v10 {
                        exceptions_str = PyObject_Repr(exceptions);
                        if exceptions_str.is_null() {
                            // We don't hold a reference to exceptions, so clear it before
                            // attempting a decref in the cleanup.
                            exceptions = core::ptr::null_mut::<_object>();
                            break '__join_0;
                        }
                    }
                    exceptions = PySequence_Tuple(exceptions);
                    if exceptions.is_null() {
                        return core::ptr::null_mut::<_object>();
                    } else {
                        // We are now holding a ref to the exceptions tuple
                        numexcs = PyTuple_GET_SIZE!(exceptions);
                        if numexcs == 0 {
                            PyErr_SetString(
                                PyExc_ValueError,
                                c"second argument (exceptions) must be a non-empty sequence"
                                    .as_ptr(),
                            );
                        } else {
                            nested_base_exceptions = false;
                            i = 0;
                            '__join_20: {
                                while i < numexcs {
                                    exc = PyTuple_GET_ITEM!(exceptions, i);
                                    if exc.is_null() {
                                        break '__join_0;
                                    } else {
                                        if PyExceptionInstance_Check!(exc) == 0 {
                                            break '__join_20;
                                        } else {
                                            is_nonbase_exception =
                                                PyObject_IsInstance(exc, PyExc_Exception);
                                            if is_nonbase_exception < 0 {
                                                break '__join_0;
                                            } else {
                                                if is_nonbase_exception == 0 {
                                                    nested_base_exceptions = true;
                                                }
                                                i = i.wrapping_add(1);
                                            }
                                        }
                                    }
                                }
                                cls = r#type;
                                if cls == PyExc_ExceptionGroup {
                                    if nested_base_exceptions {
                                        PyErr_SetString(
                                            PyExc_TypeError,
                                            c"Cannot nest BaseExceptions in an ExceptionGroup"
                                                .as_ptr(),
                                        );
                                        break '__join_0;
                                    }
                                } else {
                                    if cls == (PyExc_BaseExceptionGroup as *mut _typeobject) {
                                        if !nested_base_exceptions {
                                            // All nested exceptions are Exception subclasses,
                                            // wrap them in an ExceptionGroup
                                            cls = PyExc_ExceptionGroup;
                                        }
                                    } else {
                                        // user-defined subclass
                                        if nested_base_exceptions {
                                            nonbase = PyObject_IsSubclass(
                                                cls as *mut _object,
                                                PyExc_Exception,
                                            );
                                            if nonbase == 1i32.wrapping_neg() {
                                                break '__join_0;
                                            } else {
                                                if nonbase == 1 {
                                                    PyErr_Format(
                                                        PyExc_TypeError,
                                                        c"Cannot nest BaseExceptions in '%.200s'"
                                                            .as_ptr(),
                                                        (*cls).tp_name,
                                                    );
                                                    break '__join_0;
                                                }
                                            }
                                        }
                                    }
                                }
                                if cls.is_null() {
                                    // Don't crash during interpreter shutdown
                                    // (PyExc_ExceptionGroup may have been cleared)
                                    cls = PyExc_BaseExceptionGroup as *mut _typeobject;
                                }
                                self_ = PyBaseExceptionGroupObject_CAST(BaseException_new(
                                    cls, args, kwds,
                                ));
                                if self_.is_null() {
                                    break '__join_0;
                                } else {
                                    (*self_).msg = Py_NewRef!(message);
                                    (*self_).excs = exceptions;
                                    (*self_).excs_str = exceptions_str;
                                    return self_ as *mut _object;
                                }
                            }
                            PyErr_Format(
                                PyExc_ValueError,
                                c"Item %zd of second argument (exceptions) is not an exception"
                                    .as_ptr(),
                                i,
                            );
                        }
                    }
                }
                Py_XDECREF!(exceptions);
                Py_XDECREF!(exceptions_str);
                return core::ptr::null_mut::<_object>();
            }
        }
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyExc_CreateExceptionGroup(
    msg_str: *const i8,
    excs: *mut _object,
) -> *mut _object {
    let msg: *mut _object = unsafe { PyUnicode_FromString(msg_str) };
    if msg.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let args: *mut _object = unsafe { PyTuple_Pack(2i64, msg, excs) };
    Py_DECREF!(msg);
    if args.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let result: *mut _object = unsafe { PyObject_CallObject(PyExc_BaseExceptionGroup, args) };
    Py_DECREF!(args);
    result
}

#[unsafe(link_section = ".text.slate_distinct.exceptions.BaseExceptionGroup_init")]
extern "C-unwind" fn BaseExceptionGroup_init(
    self_: *mut _object,
    args: *mut _object,
    kwds: *mut _object,
) -> i32 {
    if !_PyArg_NoKeywords__2!(unsafe { (*Py_TYPE!(self_)).tp_name }, kwds) {
        return 1i32.wrapping_neg();
    }
    if BaseException_init(self_, args, kwds) == 1i32.wrapping_neg() {
        return 1i32.wrapping_neg();
    }
    0
}

#[unsafe(link_section = ".text.slate_distinct.exceptions.BaseExceptionGroup_clear")]
extern "C-unwind" fn BaseExceptionGroup_clear(op: *mut _object) -> i32 {
    let self_: *mut __SlateRecord_pyerrors_h_22 = PyBaseExceptionGroupObject_CAST(op);
    Py_CLEAR!((*self_).msg);
    Py_CLEAR!((*self_).excs);
    Py_CLEAR!((*self_).excs_str);
    BaseException_clear(op)
}

#[unsafe(link_section = ".text.slate_distinct.exceptions.BaseExceptionGroup_dealloc")]
extern "C-unwind" fn BaseExceptionGroup_dealloc(self_: *mut _object) {
    _PyObject_GC_UNTRACK!(self_);
    BaseExceptionGroup_clear(self_);
    unsafe { unsafe { (*Py_TYPE!(self_)).tp_free }.unwrap()(self_ as *mut ()) };
}

#[unsafe(link_section = ".text.slate_distinct.exceptions.BaseExceptionGroup_traverse")]
extern "C-unwind" fn BaseExceptionGroup_traverse(
    op: *mut _object,
    mut visit: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
    arg: *mut (),
) -> i32 {
    let self_: *mut __SlateRecord_pyerrors_h_22 = PyBaseExceptionGroupObject_CAST(op);
    Py_VISIT!(unsafe { (*self_).msg }, visit, arg);
    Py_VISIT!(unsafe { (*self_).excs }, visit, arg);
    Py_VISIT!(unsafe { (*self_).excs_str }, visit, arg);
    BaseException_traverse(op, visit, arg)
}

#[unsafe(link_section = ".text.slate_distinct.exceptions.BaseExceptionGroup_str")]
extern "C-unwind" fn BaseExceptionGroup_str(op: *mut _object) -> *mut _object {
    let self_: *mut __SlateRecord_pyerrors_h_22 = PyBaseExceptionGroupObject_CAST(op);
    let num_excs: i64 = unsafe { PyTuple_Size((*self_).excs) };
    unsafe {
        PyUnicode_FromFormat(
            c"%S (%zd sub-exception%s)".as_ptr(),
            (*self_).msg,
            num_excs,
            if num_excs > 1 {
                c"s".as_ptr() as *mut i8
            } else {
                c"".as_ptr() as *mut i8
            },
        )
    }
}

#[unsafe(link_section = ".text.slate_distinct.exceptions.BaseExceptionGroup_repr")]
extern "C-unwind" fn BaseExceptionGroup_repr(op: *mut _object) -> *mut _object {
    let self_: *mut __SlateRecord_pyerrors_h_22 = PyBaseExceptionGroupObject_CAST(op);
    let mut exceptions_str: *mut _object = core::ptr::null_mut::<_object>();
    // Use the saved exceptions string for custom sequences.
    if !unsafe { (*self_).excs_str }.is_null() {
        exceptions_str = Py_NewRef!(unsafe { (*self_).excs_str });
    } else {
        // Older versions delegated to BaseException, inserting the current
        // value of self.args[1]; but this can be mutable and go out-of-sync
        // with self.exceptions. Instead, use self.exceptions for accuracy,
        // making it look like self.args[1] for backwards compatibility.
        let __v0: bool = PyTuple_GET_SIZE!(unsafe { (*self_).args }) == 2
            && PyList_Check!(PyTuple_GET_ITEM!(unsafe { (*self_).args }, 1i32)) != 0;
        if __v0 {
            let exceptions_list: *mut _object = unsafe { PySequence_List((*self_).excs) };
            if exceptions_list.is_null() {
                return core::ptr::null_mut::<_object>();
            }
            exceptions_str = unsafe { PyObject_Repr(exceptions_list) };
            Py_DECREF!(exceptions_list);
        } else {
            exceptions_str = unsafe { PyObject_Repr((*self_).excs) };
        }
        if exceptions_str.is_null() {
            return core::ptr::null_mut::<_object>();
        }
    }
    let name: *const i8 = unsafe { _PyType_Name(Py_TYPE__2!(self_)) };
    let repr: *mut _object =
        unsafe { PyUnicode_FromFormat(c"%s(%R, %U)".as_ptr(), name, (*self_).msg, exceptions_str) };
    Py_DECREF!(exceptions_str);
    repr
}

// [clinic input]
// @critical_section
// BaseExceptionGroup.derive
//     excs: object
//     /
// [clinic start generated code]
/// [clinic end generated code: output=4307564218dfbf06 input=f72009d38e98cec1]
fn BaseExceptionGroup_derive_impl(
    self_: *mut __SlateRecord_pyerrors_h_22,
    excs: *mut _object,
) -> *mut _object {
    let init_args: *mut _object = unsafe { PyTuple_Pack(2i64, (*self_).msg, excs) };
    if init_args.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let eg: *mut _object = unsafe { PyObject_CallObject(PyExc_BaseExceptionGroup, init_args) };
    Py_DECREF!(init_args);
    eg
}

fn exceptiongroup_subset(
    _orig: *mut __SlateRecord_pyerrors_h_22,
    excs: *mut _object,
    result: *mut *mut _object,
) -> i32 {
    // Make a copy so the parts have independent notes lists.
    let mut notes_copy: *mut _object = unsafe { core::mem::zeroed() };
    let mut notes: *mut _object = unsafe { core::mem::zeroed() };
    let mut tb: *mut _object = unsafe { core::mem::zeroed() };
    let mut eg: *mut _object = unsafe { core::mem::zeroed() };
    let mut num_excs: i64 = unsafe { core::mem::zeroed() };
    // Sets *result to an ExceptionGroup wrapping excs with metadata from
    // _orig. If excs is empty, sets *result to NULL.
    // Returns 0 on success and -1 on error.
    //
    // This function is used by split() to construct the match/rest parts,
    // so excs is the matching or non-matching sub-sequence of orig->excs
    // (this function does not verify that it is a subsequence).
    let mut orig: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        orig = _orig as *mut _object;
        *result = core::ptr::null_mut::<_object>();
        num_excs = PySequence_Size(excs);
        if num_excs < 0 {
            return 1i32.wrapping_neg();
        } else {
            if num_excs == 0 {
                return 0;
            } else {
                eg = PyObject_CallMethod(orig, c"derive".as_ptr(), c"(O)".as_ptr(), excs);
                if eg.is_null() {
                    return 1i32.wrapping_neg();
                } else {
                    '__join_0: {
                        if _PyBaseExceptionGroup_Check!(eg) == 0 {
                            PyErr_SetString(
                                PyExc_TypeError,
                                c"derive must return an instance of BaseExceptionGroup".as_ptr(),
                            );
                        } else {
                            // Now we hold a reference to the new eg
                            tb = PyException_GetTraceback(orig);
                            if !tb.is_null() {
                                let res: i32 = PyException_SetTraceback(eg, tb);
                                Py_DECREF!(tb);
                                if res < 0 {
                                    break '__join_0;
                                }
                            }
                            PyException_SetContext(eg, PyException_GetContext(orig));
                            PyException_SetCause(eg, PyException_GetCause(orig));
                            if PyObject_GetOptionalAttr(
                                orig,
                                core::ptr::addr_of_mut!(
                                    _PyRuntime
                                        .static_objects
                                        .singletons
                                        .strings
                                        .identifiers
                                        ._py___notes__
                                        ._ascii
                                        .ob_base
                                ),
                                core::ptr::addr_of_mut!(notes),
                            ) >= 0
                            {
                                if !notes.is_null() {
                                    if PySequence_Check(notes) != 0 {
                                        notes_copy = PySequence_List(notes);
                                        Py_DECREF!(notes);
                                        if notes_copy.is_null() {
                                            break '__join_0;
                                        } else {
                                            let res: i32 = PyObject_SetAttr(
                                                eg,
                                                core::ptr::addr_of_mut!(
                                                    _PyRuntime
                                                        .static_objects
                                                        .singletons
                                                        .strings
                                                        .identifiers
                                                        ._py___notes__
                                                        ._ascii
                                                        .ob_base
                                                ),
                                                notes_copy,
                                            );
                                            Py_DECREF!(notes_copy);
                                            if res < 0 {
                                                break '__join_0;
                                            }
                                        }
                                    } else {
                                        // __notes__ is supposed to be a list, and split() is not a
                                        // good place to report earlier user errors, so we just ignore
                                        // notes of non-sequence type.
                                        Py_DECREF!(notes);
                                    }
                                }
                                *result = eg;
                                return 0;
                            }
                        }
                    }
                    Py_DECREF!(eg);
                    return 1i32.wrapping_neg();
                }
            }
        }
    }
}

fn get_matcher_type(value: *mut _object, r#type: &mut u32) -> i32 {
    let mut i: i64 = unsafe { core::mem::zeroed() };
    let mut n: i64 = unsafe { core::mem::zeroed() };
    let mut __v4: bool = unsafe { core::mem::zeroed() };
    unsafe {
        if PyCallable_Check(value) != 0 {
            __v4 = PyType_Check!(value) == 0;
        } else {
            __v4 = false;
        }
        if __v4 {
            *(r#type as *mut u32) = 1;
            return 0;
        } else {
            if PyExceptionClass_Check!(value) {
                *(r#type as *mut u32) = 0;
                return 0;
            } else {
                '__join_0: {
                    if PyTuple_CheckExact!(value) != 0 {
                        n = PyTuple_GET_SIZE!(value);
                        i = 0;
                        while i < n {
                            if !PyExceptionClass_Check__2!(unsafe {
                                *((*(value as *mut __SlateRecord_tupleobject_h_13))
                                    .ob_item
                                    .as_mut_ptr()
                                    as *mut *mut _object)
                                    .wrapping_offset(i as isize)
                            }) {
                                break '__join_0;
                            } else {
                                i = i.wrapping_add(1);
                            }
                        }
                        *(r#type as *mut u32) = 0;
                        return 0;
                    }
                }
                PyErr_SetString(PyExc_TypeError, c"expected an exception type, a tuple of exception types, or a callable (other than a class)".as_ptr());
                return 1i32.wrapping_neg();
            }
        }
    }
}

fn exceptiongroup_split_check_match(
    exc: *mut _object,
    matcher_type: u32,
    matcher_value: *mut _object,
) -> i32 {
    match matcher_type as u32 {
        0 => {
            return unsafe { PyErr_GivenExceptionMatches(exc, matcher_value) };
        }
        1 => {
            let exc_matches: *mut _object = unsafe { PyObject_CallOneArg(matcher_value, exc) };
            if exc_matches.is_null() {
                return 1i32.wrapping_neg();
            }
            let is_true: i32 = unsafe { PyObject_IsTrue(exc_matches) };
            Py_DECREF!(exc_matches);
            return is_true;
        }
        2 => {
            if _PyBaseExceptionGroup_Check!(exc) == 0 {
                let exc_id: *mut _object = unsafe { PyLong_FromVoidPtr(exc as *mut ()) };
                if exc_id.is_null() {
                    return 1i32.wrapping_neg();
                }
                let res: i32 = unsafe { PySet_Contains(matcher_value, exc_id) };
                Py_DECREF!(exc_id);
                return res;
            }
            return 0;
        }
        _ => {}
    }
    0
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_exceptions_c_1328 {
    r#match: *mut _object,
    rest: *mut _object,
}

fn exceptiongroup_split_recursive(
    exc: *mut _object,
    matcher_type: u32,
    matcher_value: *mut _object,
    construct_rest: bool,
    result: *mut __SlateRecord_exceptions_c_1328,
) -> i32 {
    let mut rec_result: core::mem::MaybeUninit<__SlateRecord_exceptions_c_1328> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_2: *mut __SlateRecord_exceptions_c_1328 =
        core::ptr::addr_of_mut!(rec_result) as *mut __SlateRecord_exceptions_c_1328;
    let mut e: *mut _object = unsafe { core::mem::zeroed() };
    // recursive calls
    let mut i: i64 = unsafe { core::mem::zeroed() };
    let mut rest_list: *mut _object = unsafe { core::mem::zeroed() };
    let mut match_list: *mut _object = unsafe { core::mem::zeroed() };
    let mut retval: i32 = unsafe { core::mem::zeroed() };
    let mut num_excs: i64 = unsafe { core::mem::zeroed() };
    let mut eg: *mut __SlateRecord_pyerrors_h_22 = unsafe { core::mem::zeroed() };
    let mut is_match: i32 = unsafe { core::mem::zeroed() };
    unsafe {
        (*result).r#match = core::ptr::null_mut::<_object>();
        (*result).rest = core::ptr::null_mut::<_object>();
        is_match = exceptiongroup_split_check_match(exc, matcher_type, matcher_value);
        if is_match < 0 {
            return 1i32.wrapping_neg();
        } else {
            if is_match != 0 {
                // Full match
                (*result).r#match = Py_NewRef!(exc);
                return 0;
            } else {
                if _PyBaseExceptionGroup_Check!(exc) == 0 {
                    // Leaf exception and no match
                    if construct_rest {
                        (*result).rest = Py_NewRef!(exc);
                    }
                    return 0;
                } else {
                    // Partial match
                    eg = PyBaseExceptionGroupObject_CAST(exc);
                    num_excs = PyTuple_Size((*eg).excs);
                    if num_excs < 0 {
                        return 1i32.wrapping_neg();
                    } else {
                        // checked in constructor, and excs is read-only
                        retval = 1i32.wrapping_neg();
                        match_list = PyList_New(0i64);
                        if match_list.is_null() {
                            return 1i32.wrapping_neg();
                        } else {
                            '__join_2: {
                                rest_list = core::ptr::null_mut::<_object>();
                                if construct_rest {
                                    rest_list = PyList_New(0i64);
                                    if rest_list.is_null() {
                                        break '__join_2;
                                    }
                                }
                                i = 0;
                                '__join_18: {
                                    '__join_15: {
                                        '__join_11: {
                                            while i < num_excs {
                                                e = PyTuple_GET_ITEM!(unsafe { (*eg).excs }, i);
                                                if _Py_EnterRecursiveCall(
                                                    c" in exceptiongroup_split_recursive".as_ptr(),
                                                ) != 0
                                                {
                                                    break '__join_2;
                                                } else {
                                                    if exceptiongroup_split_recursive(
                                                        e,
                                                        matcher_type,
                                                        matcher_value,
                                                        construct_rest,
                                                        core::ptr::addr_of_mut!(*__slate_slot_2),
                                                    ) < 0
                                                    {
                                                        break '__join_18;
                                                    } else {
                                                        _Py_LeaveRecursiveCall();
                                                        if !(*__slate_slot_2).r#match.is_null() {
                                                            if PyList_Append(
                                                                match_list,
                                                                (*__slate_slot_2).r#match,
                                                            ) < 0
                                                            {
                                                                break '__join_15;
                                                            } else {
                                                                Py_DECREF!(
                                                                    (*__slate_slot_2).r#match
                                                                );
                                                            }
                                                        }
                                                        if !(*__slate_slot_2).rest.is_null() {
                                                            if PyList_Append(
                                                                rest_list,
                                                                (*__slate_slot_2).rest,
                                                            ) < 0
                                                            {
                                                                break '__join_11;
                                                            } else {
                                                                Py_DECREF!((*__slate_slot_2).rest);
                                                            }
                                                        }
                                                        i = i.wrapping_add(1);
                                                    }
                                                }
                                            }
                                            // construct result
                                            if exceptiongroup_subset(
                                                eg,
                                                match_list,
                                                core::ptr::addr_of_mut!((*result).r#match),
                                            ) < 0
                                            {
                                                break '__join_2;
                                            } else {
                                                if construct_rest {
                                                    if exceptiongroup_subset(
                                                        eg,
                                                        rest_list,
                                                        core::ptr::addr_of_mut!((*result).rest),
                                                    ) < 0
                                                    {
                                                        Py_CLEAR!((*result).r#match);
                                                        break '__join_2;
                                                    }
                                                }
                                                retval = 0;
                                                break '__join_2;
                                            }
                                        }
                                        Py_DECREF!((*__slate_slot_2).rest);
                                        break '__join_2;
                                    }
                                    Py_DECREF!((*__slate_slot_2).r#match);
                                    Py_XDECREF!((*__slate_slot_2).rest);
                                    break '__join_2;
                                }
                                _Py_LeaveRecursiveCall();
                            }
                            Py_DECREF!(match_list);
                            Py_XDECREF!(rest_list);
                            if retval < 0 {
                                Py_CLEAR!((*result).r#match);
                                Py_CLEAR!((*result).rest);
                            }
                            return retval;
                        }
                    }
                }
            }
        }
    }
}

// [clinic input]
// @critical_section
// BaseExceptionGroup.split
//     matcher_value: object
//     /
// [clinic start generated code]
/// [clinic end generated code: output=d74db579da4df6e2 input=0c5cfbfed57e0052]
fn BaseExceptionGroup_split_impl(
    self_: *mut __SlateRecord_pyerrors_h_22,
    matcher_value: *mut _object,
) -> *mut _object {
    let mut matcher_type: u32 = 0;
    if get_matcher_type(matcher_value, unsafe {
        &mut (*core::ptr::addr_of_mut!(matcher_type))
    }) < 0
    {
        return core::ptr::null_mut::<_object>();
    }
    let mut split_result: __SlateRecord_exceptions_c_1328 = unsafe { core::mem::zeroed() };
    let construct_rest: bool = true;
    if exceptiongroup_split_recursive(
        self_ as *mut _object,
        matcher_type,
        matcher_value,
        construct_rest,
        core::ptr::addr_of_mut!(split_result),
    ) < 0
    {
        return core::ptr::null_mut::<_object>();
    }
    let result: *mut _object = unsafe {
        PyTuple_Pack(
            2i64,
            if !split_result.r#match.is_null() {
                split_result.r#match
            } else {
                core::ptr::addr_of_mut!(_Py_NoneStruct)
            },
            if !split_result.rest.is_null() {
                split_result.rest
            } else {
                core::ptr::addr_of_mut!(_Py_NoneStruct)
            },
        )
    };
    Py_XDECREF!(split_result.r#match);
    Py_XDECREF!(split_result.rest);
    result
}

// [clinic input]
// @critical_section
// BaseExceptionGroup.subgroup
//     matcher_value: object
//     /
// [clinic start generated code]
/// [clinic end generated code: output=07dbec8f77d4dd8e input=988ffdd755a151ce]
fn BaseExceptionGroup_subgroup_impl(
    self_: *mut __SlateRecord_pyerrors_h_22,
    matcher_value: *mut _object,
) -> *mut _object {
    let mut matcher_type: u32 = 0;
    if get_matcher_type(matcher_value, unsafe {
        &mut (*core::ptr::addr_of_mut!(matcher_type))
    }) < 0
    {
        return core::ptr::null_mut::<_object>();
    }
    let mut split_result: __SlateRecord_exceptions_c_1328 = unsafe { core::mem::zeroed() };
    let construct_rest: bool = false;
    if exceptiongroup_split_recursive(
        self_ as *mut _object,
        matcher_type,
        matcher_value,
        construct_rest,
        core::ptr::addr_of_mut!(split_result),
    ) < 0
    {
        return core::ptr::null_mut::<_object>();
    }
    let result: *mut _object = Py_NewRef!(if !split_result.r#match.is_null() {
        split_result.r#match
    } else {
        unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
    });
    Py_XDECREF!(split_result.r#match);
    result
}

fn collect_exception_group_leaf_ids(exc: *mut _object, leaf_ids: *mut _object) -> i32 {
    if Py_IsNone!(exc) {
        return 0;
    }
    // Add IDs of all leaf exceptions in exc to the leaf_ids set
    if _PyBaseExceptionGroup_Check!(exc) == 0 {
        let exc_id: *mut _object = unsafe { PyLong_FromVoidPtr(exc as *mut ()) };
        if exc_id.is_null() {
            return 1i32.wrapping_neg();
        }
        let res: i32 = unsafe { PySet_Add(leaf_ids, exc_id) };
        Py_DECREF!(exc_id);
        return res;
    }
    let eg: *mut __SlateRecord_pyerrors_h_22 = PyBaseExceptionGroupObject_CAST(exc);
    let num_excs: i64 = PyTuple_GET_SIZE!(unsafe { (*eg).excs });
    // recursive calls
    for i in 0..num_excs {
        let e: *mut _object = PyTuple_GET_ITEM!(unsafe { (*eg).excs }, i);
        if _Py_EnterRecursiveCall(c" in collect_exception_group_leaf_ids".as_ptr()) != 0 {
            return 1i32.wrapping_neg();
        }
        let res: i32 = collect_exception_group_leaf_ids(e, leaf_ids);
        _Py_LeaveRecursiveCall();
        if res < 0 {
            return 1i32.wrapping_neg();
        }
    }
    0
}

/// This function is used by the interpreter to construct reraised
/// exception groups. It takes an exception group eg and a list
/// of exception groups keep and returns the sub-exception group
/// of eg which contains all leaf exceptions that are contained
/// in any exception group in keep.
fn exception_group_projection(eg: *mut _object, keep: *mut _object) -> *mut _object {
    let leaf_ids: *mut _object = unsafe { PySet_New(core::ptr::null_mut::<_object>()) };
    if leaf_ids.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let n: i64 = PyList_GET_SIZE!(keep);
    for i in 0..n {
        let e: *mut _object = PyList_GET_ITEM!(keep, i);
        if collect_exception_group_leaf_ids(e, leaf_ids) < 0 {
            Py_DECREF!(leaf_ids);
            return core::ptr::null_mut::<_object>();
        }
    }
    let mut split_result: __SlateRecord_exceptions_c_1328 = unsafe { core::mem::zeroed() };
    let construct_rest: bool = false;
    let err: i32 = exceptiongroup_split_recursive(
        eg,
        2u32,
        leaf_ids,
        construct_rest,
        core::ptr::addr_of_mut!(split_result),
    );
    Py_DECREF!(leaf_ids);
    if err < 0 {
        return core::ptr::null_mut::<_object>();
    }
    let __v1: *mut _object = if !split_result.r#match.is_null() {
        split_result.r#match
    } else {
        Py_NewRef!(unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) })
    };
    let result: *mut _object = __v1;
    result
}

fn is_same_exception_metadata(exc1: *mut _object, exc2: *mut _object) -> bool {
    (unsafe { (*(exc1 as *mut __SlateRecord_pyerrors_h_15)).notes })
        == unsafe { (*(exc2 as *mut __SlateRecord_pyerrors_h_15)).notes }
        && (unsafe { (*(exc1 as *mut __SlateRecord_pyerrors_h_15)).traceback })
            == unsafe { (*(exc2 as *mut __SlateRecord_pyerrors_h_15)).traceback }
        && (unsafe { (*(exc1 as *mut __SlateRecord_pyerrors_h_15)).cause })
            == unsafe { (*(exc2 as *mut __SlateRecord_pyerrors_h_15)).cause }
        && (unsafe { (*(exc1 as *mut __SlateRecord_pyerrors_h_15)).context })
            == unsafe { (*(exc2 as *mut __SlateRecord_pyerrors_h_15)).context }
}

/// This function is used by the interpreter to calculate
/// the exception group to be raised at the end of a
/// try-except* construct.
///
/// orig: the original except that was caught.
/// excs: a list of exceptions that were raised/reraised
///       in the except* clauses.
///
/// Calculates an exception group to raise. It contains
/// all exceptions in excs, where those that were reraised
/// have same nesting structure as in orig, and those that
/// were raised (if any) are added as siblings in a new EG.
///
/// Returns NULL and sets an exception on failure.
#[unsafe(no_mangle)]
extern "C-unwind" fn _PyExc_PrepReraiseStar(
    orig: *mut _object,
    excs: *mut _object,
) -> *mut _object {
    let mut res: i32 = unsafe { core::mem::zeroed() };
    let mut num_raised: i64 = unsafe { core::mem::zeroed() };
    let mut reraised_eg: *mut _object = unsafe { core::mem::zeroed() };
    let mut e: *mut _object = unsafe { core::mem::zeroed() };
    // Split excs into raised and reraised by comparing metadata with orig
    let mut i: i64 = unsafe { core::mem::zeroed() };
    let mut result: *mut _object = unsafe { core::mem::zeroed() };
    let mut reraised_list: *mut _object = unsafe { core::mem::zeroed() };
    let mut raised_list: *mut _object = unsafe { core::mem::zeroed() };
    let mut numexcs: i64 = unsafe { core::mem::zeroed() };
    unsafe {
        // orig must be a raised & caught exception, so it has a traceback
        numexcs = PyList_GET_SIZE!(excs);
        if numexcs == 0 {
            return Py_NewRef!(core::ptr::addr_of_mut!(_Py_NoneStruct));
        } else {
            if _PyBaseExceptionGroup_Check!(orig) == 0 {
                // a naked exception was caught and wrapped. Only one except* clause
                // could have executed,so there is at most one exception to raise.
                let e: *mut _object = PyList_GET_ITEM!(excs, 0i32);
                return Py_NewRef!(e);
            } else {
                raised_list = PyList_New(0i64);
                if raised_list.is_null() {
                    return core::ptr::null_mut::<_object>();
                } else {
                    reraised_list = PyList_New(0i64);
                    if reraised_list.is_null() {
                        Py_DECREF!(raised_list);
                        return core::ptr::null_mut::<_object>();
                    } else {
                        // Now we are holding refs to raised_list and reraised_list
                        result = core::ptr::null_mut::<_object>();
                        i = 0;
                        '__join_0: {
                            while i < numexcs {
                                e = PyList_GET_ITEM!(excs, i);
                                if !Py_IsNone!(e) {
                                    let is_reraise: bool = is_same_exception_metadata(e, orig);
                                    let append_list: *mut _object = if is_reraise {
                                        reraised_list
                                    } else {
                                        raised_list
                                    };
                                    if PyList_Append(append_list, e) < 0 {
                                        break '__join_0;
                                    }
                                }
                                i = i.wrapping_add(1);
                            }
                            reraised_eg = exception_group_projection(orig, reraised_list);
                            if !reraised_eg.is_null() {
                                if !Py_IsNone!(reraised_eg) {}
                                num_raised = PyList_GET_SIZE!(raised_list);
                                if num_raised == 0 {
                                    result = reraised_eg;
                                } else {
                                    if num_raised > 0 {
                                        res = 0;
                                        if !Py_IsNone!(reraised_eg) {
                                            res = PyList_Append(raised_list, reraised_eg);
                                        }
                                        Py_DECREF!(reraised_eg);
                                        if res >= 0 {
                                            if PyList_GET_SIZE!(raised_list) > 1 {
                                                result = _PyExc_CreateExceptionGroup(
                                                    c"".as_ptr(),
                                                    raised_list,
                                                );
                                            } else {
                                                result =
                                                    Py_NewRef!(PyList_GetItem(raised_list, 0i64));
                                            }
                                            if result.is_null() {}
                                        }
                                    }
                                }
                            }
                        }
                        Py_XDECREF!(raised_list);
                        Py_XDECREF!(reraised_list);
                        return result;
                    }
                }
            }
        }
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyUnstable_Exc_PrepReraiseStar(
    orig: *mut _object,
    excs: *mut _object,
) -> *mut _object {
    let __v0: bool = orig.is_null() || PyExceptionInstance_Check!(orig) == 0;
    if __v0 {
        unsafe {
            PyErr_SetString(
                PyExc_TypeError,
                c"orig must be an exception instance".as_ptr(),
            )
        };
        return core::ptr::null_mut::<_object>();
    }
    let __v1: bool = excs.is_null() || PyList_Check!(excs) == 0;
    if __v1 {
        unsafe {
            PyErr_SetString(
                PyExc_TypeError,
                c"excs must be a list of exception instances".as_ptr(),
            )
        };
        return core::ptr::null_mut::<_object>();
    }
    let numexcs: i64 = PyList_GET_SIZE!(excs);
    for i in 0..numexcs {
        let exc: *mut _object = PyList_GET_ITEM!(excs, i);
        let __v2: bool =
            exc.is_null() || !(PyExceptionInstance_Check!(exc) != 0 || Py_IsNone!(exc));
        if __v2 {
            unsafe {
                PyErr_Format(
                    PyExc_TypeError,
                    c"item %zd of excs is not an exception".as_ptr(),
                    i,
                )
            };
            return core::ptr::null_mut::<_object>();
        }
    }
    // Make sure that orig has something as traceback, in the interpreter
    // it always does because it's a raised exception.
    let tb: *mut _object = PyException_GetTraceback(orig);
    if tb.is_null() {
        unsafe {
            PyErr_Format(
                PyExc_ValueError,
                c"orig must be a raised exception".as_ptr(),
            )
        };
        return core::ptr::null_mut::<_object>();
    }
    Py_DECREF!(tb);
    _PyExc_PrepReraiseStar(orig, excs)
}

/// Sentinel
static mut BaseExceptionGroup_members: __SlateAlign16<[PyMemberDef; 3]> = __SlateAlign16([
    {
        let mut __t0: PyMemberDef = unsafe { core::mem::zeroed() };
        __t0.name = (b"message\0".as_ptr() as *mut i8) as *const i8;
        __t0.r#type = 6i32;
        __t0.offset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_22, msg) as u64) as i64;
        __t0.flags = 1i32;
        __t0.doc = (b"exception message\0".as_ptr() as *mut i8) as *const i8;
        __t0
    },
    {
        let mut __t1: PyMemberDef = unsafe { core::mem::zeroed() };
        __t1.name = (b"exceptions\0".as_ptr() as *mut i8) as *const i8;
        __t1.r#type = 6i32;
        __t1.offset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_22, excs) as u64) as i64;
        __t1.flags = 1i32;
        __t1.doc = (b"nested exceptions\0".as_ptr() as *mut i8) as *const i8;
        __t1
    },
    {
        let mut __t2: PyMemberDef = unsafe { core::mem::zeroed() };
        __t2.name = core::ptr::null::<i8>();
        __t2
    },
]);

static mut BaseExceptionGroup_methods: __SlateAlign16<[PyMethodDef; 5]> = __SlateAlign16([
    {
        let mut __t0: PyMethodDef = unsafe { core::mem::zeroed() };
        __t0.ml_name = (b"__class_getitem__\0".as_ptr() as *mut i8) as *const i8;
        __t0.ml_meth = unsafe {
            core::mem::transmute::<
                *const (),
                Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
            >(Py_GenericAlias as *const ())
        };
        __t0.ml_flags = 24i32;
        __t0.ml_doc = (b"Exception groups are generic over the type of their contained exceptions\0"
            .as_ptr() as *mut i8) as *const i8;
        __t0
    },
    {
        let mut __t1: PyMethodDef = unsafe { core::mem::zeroed() };
        __t1.ml_name = (b"derive\0".as_ptr() as *mut i8) as *const i8;
        __t1.ml_meth = Some(BaseExceptionGroup_derive);
        __t1.ml_flags = 8i32;
        __t1.ml_doc =
            unsafe { core::ptr::addr_of!(BaseExceptionGroup_derive__doc__.0) as *const i8 };
        __t1
    },
    {
        let mut __t2: PyMethodDef = unsafe { core::mem::zeroed() };
        __t2.ml_name = (b"split\0".as_ptr() as *mut i8) as *const i8;
        __t2.ml_meth = Some(BaseExceptionGroup_split);
        __t2.ml_flags = 8i32;
        __t2.ml_doc =
            unsafe { core::ptr::addr_of!(BaseExceptionGroup_split__doc__.0) as *const i8 };
        __t2
    },
    {
        let mut __t3: PyMethodDef = unsafe { core::mem::zeroed() };
        __t3.ml_name = (b"subgroup\0".as_ptr() as *mut i8) as *const i8;
        __t3.ml_meth = Some(BaseExceptionGroup_subgroup);
        __t3.ml_flags = 8i32;
        __t3.ml_doc =
            unsafe { core::ptr::addr_of!(BaseExceptionGroup_subgroup__doc__.0) as *const i8 };
        __t3
    },
    {
        let mut __t4: PyMethodDef = unsafe { core::mem::zeroed() };
        __t4.ml_name = core::ptr::null::<i8>();
        __t4
    },
]);

static mut _PyExc_BaseExceptionGroup: _typeobject = {
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
    __t0.tp_name = (b"BaseExceptionGroup\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pyerrors_h_22>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(BaseExceptionGroup_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = Some(BaseExceptionGroup_repr);
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = Some(BaseExceptionGroup_str);
    __t0.tp_getattro = None;
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 17408u64;
    __t0.tp_doc =
        (b"A combination of multiple unrelated exceptions.\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_traverse = Some(BaseExceptionGroup_traverse);
    __t0.tp_clear = Some(BaseExceptionGroup_clear);
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods =
        unsafe { core::ptr::addr_of_mut!(BaseExceptionGroup_methods.0) as *mut PyMethodDef };
    __t0.tp_members =
        unsafe { core::ptr::addr_of_mut!(BaseExceptionGroup_members.0) as *mut PyMemberDef };
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(_PyExc_BaseException) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_22, dict) as u64) as i64;
    __t0.tp_init = Some(BaseExceptionGroup_init);
    __t0.tp_alloc = None;
    __t0.tp_new = Some(BaseExceptionGroup_new);
    __t0
};

/// new
/// getset
#[unsafe(no_mangle)]
static mut PyExc_BaseExceptionGroup: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_BaseExceptionGroup) }) as *mut _object;

/// ExceptionGroup extends BaseExceptionGroup, Exception
fn create_exception_group_class() -> *mut _object {
    let state: *mut _Py_exc_state = get_exc_state();
    let bases: *mut _object =
        unsafe { PyTuple_Pack(2i64, PyExc_BaseExceptionGroup, PyExc_Exception) };
    if bases.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    unsafe {
        (*state).PyExc_ExceptionGroup = PyErr_NewException(
            c"builtins.ExceptionGroup".as_ptr(),
            bases,
            core::ptr::null_mut::<_object>(),
        );
    }
    Py_DECREF!(bases);
    unsafe { (*state).PyExc_ExceptionGroup }
}

// KeyboardInterrupt extends BaseException
static mut _PyExc_KeyboardInterrupt: _typeobject = {
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
    __t0.tp_name = (b"KeyboardInterrupt\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pyerrors_h_15>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(BaseException_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = None;
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = None;
    __t0.tp_getattro = None;
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 17408u64;
    __t0.tp_doc = (b"Program interrupted by user.\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_traverse = Some(BaseException_traverse);
    __t0.tp_clear = Some(BaseException_clear);
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = core::ptr::null_mut::<PyMethodDef>();
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(_PyExc_BaseException) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_15, dict) as u64) as i64;
    __t0.tp_init = Some(BaseException_init);
    __t0.tp_alloc = None;
    __t0.tp_new = Some(BaseException_new);
    __t0
};

#[unsafe(no_mangle)]
static mut PyExc_KeyboardInterrupt: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_KeyboardInterrupt) }) as *mut _object;

// ImportError extends Exception
fn PyImportErrorObject_CAST(self_: *mut _object) -> *mut __SlateRecord_pyerrors_h_43 {
    self_ as *mut __SlateRecord_pyerrors_h_43
}

#[unsafe(link_section = ".text.slate_distinct.exceptions.ImportError_init")]
extern "C-unwind" fn ImportError_init(
    op: *mut _object,
    args: *mut _object,
    kwds: *mut _object,
) -> i32 {
    let mut msg: *mut _object = core::ptr::null_mut::<_object>();
    let mut name: *mut _object = core::ptr::null_mut::<_object>();
    let mut path: *mut _object = core::ptr::null_mut::<_object>();
    let mut name_from: *mut _object = core::ptr::null_mut::<_object>();
    if BaseException_init(op, args, core::ptr::null_mut::<_object>()) == 1i32.wrapping_neg() {
        return 1i32.wrapping_neg();
    }
    let self_: *mut __SlateRecord_pyerrors_h_43 = PyImportErrorObject_CAST(op);
    let empty_tuple: *mut _object = unsafe { PyTuple_New(0i64) };
    if empty_tuple.is_null() {
        return 1i32.wrapping_neg();
    }
    if (unsafe {
        PyArg_ParseTupleAndKeywords(
            empty_tuple,
            kwds,
            c"|$OOO:ImportError".as_ptr(),
            (core::ptr::addr_of_mut!(kwlist__1817.0) as *mut *mut i8) as *const *mut i8,
            core::ptr::addr_of_mut!(name),
            core::ptr::addr_of_mut!(path),
            core::ptr::addr_of_mut!(name_from),
        )
    }) == 0
    {
        Py_DECREF!(empty_tuple);
        return 1i32.wrapping_neg();
    }
    Py_DECREF!(empty_tuple);
    Py_XSETREF!((*self_).name, Py_XNewRef!(name));
    Py_XSETREF!((*self_).path, Py_XNewRef!(path));
    Py_XSETREF!((*self_).name_from, Py_XNewRef!(name_from));
    if PyTuple_GET_SIZE!(args) == 1 {
        msg = Py_NewRef!(PyTuple_GET_ITEM!(args, 0i32));
    }
    Py_XSETREF!((*self_).msg, msg);
    0
}

static mut kwlist__1817: __SlateAlign16<[*mut i8; 4]> = __SlateAlign16([
    b"name\0".as_ptr() as *mut i8,
    b"path\0".as_ptr() as *mut i8,
    b"name_from\0".as_ptr() as *mut i8,
    core::ptr::null_mut::<i8>(),
]);

#[unsafe(link_section = ".text.slate_distinct.exceptions.ImportError_clear")]
extern "C-unwind" fn ImportError_clear(op: *mut _object) -> i32 {
    let self_: *mut __SlateRecord_pyerrors_h_43 = PyImportErrorObject_CAST(op);
    Py_CLEAR!((*self_).msg);
    Py_CLEAR!((*self_).name);
    Py_CLEAR!((*self_).path);
    Py_CLEAR!((*self_).name_from);
    BaseException_clear(op)
}

#[unsafe(link_section = ".text.slate_distinct.exceptions.ImportError_dealloc")]
extern "C-unwind" fn ImportError_dealloc(self_: *mut _object) {
    _PyObject_GC_UNTRACK!(self_);
    ImportError_clear(self_);
    unsafe { unsafe { (*Py_TYPE!(self_)).tp_free }.unwrap()(self_ as *mut ()) };
}

#[unsafe(link_section = ".text.slate_distinct.exceptions.ImportError_traverse")]
extern "C-unwind" fn ImportError_traverse(
    op: *mut _object,
    mut visit: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
    arg: *mut (),
) -> i32 {
    let self_: *mut __SlateRecord_pyerrors_h_43 = PyImportErrorObject_CAST(op);
    Py_VISIT!(unsafe { (*self_).msg }, visit, arg);
    Py_VISIT!(unsafe { (*self_).name }, visit, arg);
    Py_VISIT!(unsafe { (*self_).path }, visit, arg);
    Py_VISIT!(unsafe { (*self_).name_from }, visit, arg);
    BaseException_traverse(op, visit, arg)
}

#[unsafe(link_section = ".text.slate_distinct.exceptions.ImportError_str")]
extern "C-unwind" fn ImportError_str(op: *mut _object) -> *mut _object {
    let self_: *mut __SlateRecord_pyerrors_h_43 = PyImportErrorObject_CAST(op);
    let __v0: bool =
        !unsafe { (*self_).msg }.is_null() && PyUnicode_CheckExact!(unsafe { (*self_).msg }) != 0;
    if __v0 {
        return Py_NewRef!(unsafe { (*self_).msg });
    }
    BaseException_str(op)
}

fn ImportError_getstate(op: *mut _object) -> *mut _object {
    let self_: *mut __SlateRecord_pyerrors_h_43 = PyImportErrorObject_CAST(op);
    let mut dict: *mut _object = unsafe { (*self_).dict };
    if !unsafe { (*self_).name }.is_null()
        || !unsafe { (*self_).path }.is_null()
        || !unsafe { (*self_).name_from }.is_null()
    {
        let __v0: *mut _object = if !dict.is_null() {
            unsafe { PyDict_Copy(dict) }
        } else {
            unsafe { PyDict_New() }
        };
        dict = __v0;
        if dict.is_null() {
            return core::ptr::null_mut::<_object>();
        }
        let __v1: bool = !unsafe { (*self_).name }.is_null()
            && (unsafe {
                PyDict_SetItem(
                    dict,
                    core::ptr::addr_of_mut!(
                        _PyRuntime
                            .static_objects
                            .singletons
                            .strings
                            .identifiers
                            ._py_name
                            ._ascii
                            .ob_base
                    ),
                    (*self_).name,
                )
            }) < 0;
        if __v1 {
            Py_DECREF!(dict);
            return core::ptr::null_mut::<_object>();
        }
        let __v2: bool = !unsafe { (*self_).path }.is_null()
            && (unsafe {
                PyDict_SetItem(
                    dict,
                    core::ptr::addr_of_mut!(
                        _PyRuntime
                            .static_objects
                            .singletons
                            .strings
                            .identifiers
                            ._py_path
                            ._ascii
                            .ob_base
                    ),
                    (*self_).path,
                )
            }) < 0;
        if __v2 {
            Py_DECREF!(dict);
            return core::ptr::null_mut::<_object>();
        }
        let __v3: bool = !unsafe { (*self_).name_from }.is_null()
            && (unsafe {
                PyDict_SetItem(
                    dict,
                    core::ptr::addr_of_mut!(
                        _PyRuntime
                            .static_objects
                            .singletons
                            .strings
                            .identifiers
                            ._py_name_from
                            ._ascii
                            .ob_base
                    ),
                    (*self_).name_from,
                )
            }) < 0;
        if __v3 {
            Py_DECREF!(dict);
            return core::ptr::null_mut::<_object>();
        }
        return dict;
    } else {
        if !dict.is_null() {
            return Py_NewRef!(dict);
        } else {
            return unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) };
        }
    }
}

/// Pickling support
#[unsafe(link_section = ".text.slate_distinct.exceptions.ImportError_reduce")]
extern "C-unwind" fn ImportError_reduce(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    let mut res: *mut _object = unsafe { core::mem::zeroed() };
    let state: *mut _object = ImportError_getstate(self_);
    if state.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let exc: *mut __SlateRecord_pyerrors_h_15 = PyBaseExceptionObject_CAST(self_);
    if state == unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } {
        res = unsafe { PyTuple_Pack(2i64, Py_TYPE!(self_), (*exc).args) };
    } else {
        res = unsafe { PyTuple_Pack(3i64, Py_TYPE!(self_), (*exc).args, state) };
    }
    Py_DECREF!(state);
    res
}

/// Sentinel
static mut ImportError_members: __SlateAlign16<[PyMemberDef; 5]> = __SlateAlign16([
    {
        let mut __t0: PyMemberDef = unsafe { core::mem::zeroed() };
        __t0.name = (b"msg\0".as_ptr() as *mut i8) as *const i8;
        __t0.r#type = 6i32;
        __t0.offset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_43, msg) as u64) as i64;
        __t0.flags = 0i32;
        __t0.doc = (b"exception message\0".as_ptr() as *mut i8) as *const i8;
        __t0
    },
    {
        let mut __t1: PyMemberDef = unsafe { core::mem::zeroed() };
        __t1.name = (b"name\0".as_ptr() as *mut i8) as *const i8;
        __t1.r#type = 6i32;
        __t1.offset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_43, name) as u64) as i64;
        __t1.flags = 0i32;
        __t1.doc = (b"module name\0".as_ptr() as *mut i8) as *const i8;
        __t1
    },
    {
        let mut __t2: PyMemberDef = unsafe { core::mem::zeroed() };
        __t2.name = (b"path\0".as_ptr() as *mut i8) as *const i8;
        __t2.r#type = 6i32;
        __t2.offset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_43, path) as u64) as i64;
        __t2.flags = 0i32;
        __t2.doc = (b"module path\0".as_ptr() as *mut i8) as *const i8;
        __t2
    },
    {
        let mut __t3: PyMemberDef = unsafe { core::mem::zeroed() };
        __t3.name = (b"name_from\0".as_ptr() as *mut i8) as *const i8;
        __t3.r#type = 6i32;
        __t3.offset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_43, name_from) as u64) as i64;
        __t3.flags = 0i32;
        __t3.doc = (b"name imported from module\0".as_ptr() as *mut i8) as *const i8;
        __t3
    },
    {
        let mut __t4: PyMemberDef = unsafe { core::mem::zeroed() };
        __t4.name = core::ptr::null::<i8>();
        __t4
    },
]);

static mut ImportError_methods: __SlateAlign16<[PyMethodDef; 2]> = __SlateAlign16([
    {
        let mut __t0: PyMethodDef = unsafe { core::mem::zeroed() };
        __t0.ml_name = (b"__reduce__\0".as_ptr() as *mut i8) as *const i8;
        __t0.ml_meth = Some(ImportError_reduce);
        __t0.ml_flags = 4i32;
        __t0
    },
    {
        let mut __t1: PyMethodDef = unsafe { core::mem::zeroed() };
        __t1.ml_name = core::ptr::null::<i8>();
        __t1
    },
]);

static mut _PyExc_ImportError: _typeobject = {
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
    __t0.tp_name = (b"ImportError\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pyerrors_h_43>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(ImportError_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = None;
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = Some(ImportError_str);
    __t0.tp_getattro = None;
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 17408u64;
    __t0.tp_doc = (b"Import can't find module, or can't find name in module.\0".as_ptr() as *mut i8)
        as *const i8;
    __t0.tp_traverse = Some(ImportError_traverse);
    __t0.tp_clear = Some(ImportError_clear);
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = unsafe { core::ptr::addr_of_mut!(ImportError_methods.0) as *mut PyMethodDef };
    __t0.tp_members = unsafe { core::ptr::addr_of_mut!(ImportError_members.0) as *mut PyMemberDef };
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(_PyExc_Exception) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_43, dict) as u64) as i64;
    __t0.tp_init = Some(ImportError_init);
    __t0.tp_alloc = None;
    __t0.tp_new = None;
    __t0
};

/// new
/// getset
#[unsafe(no_mangle)]
static mut PyExc_ImportError: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_ImportError) }) as *mut _object;

// ModuleNotFoundError extends ImportError
static mut _PyExc_ModuleNotFoundError: _typeobject = {
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
    __t0.tp_name = (b"ModuleNotFoundError\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pyerrors_h_43>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(ImportError_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = None;
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = None;
    __t0.tp_getattro = None;
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 17408u64;
    __t0.tp_doc = (b"Module not found.\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_traverse = Some(ImportError_traverse);
    __t0.tp_clear = Some(ImportError_clear);
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = core::ptr::null_mut::<PyMethodDef>();
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(_PyExc_ImportError) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_43, dict) as u64) as i64;
    __t0.tp_init = Some(ImportError_init);
    __t0.tp_alloc = None;
    __t0.tp_new = None;
    __t0
};

use slate_support::strerror;

#[unsafe(no_mangle)]
static mut PyExc_ModuleNotFoundError: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_ModuleNotFoundError) }) as *mut _object;

// OSError extends Exception
fn PyOSErrorObject_CAST(self_: *mut _object) -> *mut __SlateRecord_pyerrors_h_69 {
    self_ as *mut __SlateRecord_pyerrors_h_69
}

// Where a function has a single filename, such as open() or some
// of the os module functions, PyErr_SetFromErrnoWithFilename() is
// called, giving a third argument which is the filename.  But, so
// that old code using in-place unpacking doesn't break, e.g.:
//
// except OSError, (errno, strerror):
//
// we hack args so that it only contains two items.  This also
// means we need our own __str__() which prints out the filename
// when it was supplied.
//
// (If a function has two filenames, such as rename(), symlink(),
// or copy(), PyErr_SetFromErrnoWithFilenameObjects() is called,
// which allows passing in a second filename.)
/// This function doesn't cleanup on error, the caller should
fn oserror_parse_args(
    p_args: &*mut _object,
    myerrno: *mut *mut _object,
    strerror: *mut *mut _object,
    filename: *mut *mut _object,
    filename2: *mut *mut _object,
) -> i32 {
    let mut nargs: i64 = 0;
    let args: *mut _object = unsafe { *(p_args as *const *mut _object) };
    // ignored on non-Windows platforms,
    // but parsed so OSError has a consistent signature
    let mut _winerror: *mut _object = core::ptr::null_mut::<_object>();
    let winerror: *mut *mut _object = core::ptr::addr_of_mut!(_winerror);
    nargs = PyTuple_GET_SIZE!(args);
    if nargs >= 2 && nargs <= 5 {
        if (unsafe {
            PyArg_UnpackTuple(
                args,
                c"OSError".as_ptr(),
                2i64,
                5i64,
                myerrno,
                strerror,
                filename,
                winerror,
                filename2,
            )
        }) == 0
        {
            return 1i32.wrapping_neg();
        }
    }
    0
}

fn oserror_init(
    self_: *mut __SlateRecord_pyerrors_h_69,
    p_args: &mut *mut _object,
    myerrno: *mut _object,
    strerror: *mut _object,
    filename: *mut _object,
    filename2: *mut _object,
) -> i32 {
    let mut args: *mut _object = unsafe { *(p_args as *mut *mut _object) };
    let nargs: i64 = PyTuple_GET_SIZE!(args);
    // self->filename will remain Py_None otherwise
    if !filename.is_null() && filename != unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } {
        let __v0: bool = Py_IS_TYPE__2!(
            self_,
            (unsafe { PyExc_BlockingIOError }) as *mut _typeobject
        ) != 0
            && (unsafe { PyNumber_Check(filename) }) != 0;
        if __v0 {
            // BlockingIOError's 3rd argument can be the number of
            // characters written.
            unsafe {
                (*self_).written = PyNumber_AsSsize_t(filename, PyExc_ValueError);
            }
            let __v1: bool = (unsafe { (*self_).written }) == (1i32.wrapping_neg() as i64)
                && !unsafe { PyErr_Occurred() }.is_null();
            if __v1 {
                return 1i32.wrapping_neg();
            }
        } else {
            Py_XSETREF!((*self_).filename, Py_NewRef!(filename));
            if !filename2.is_null()
                && filename2 != unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
            {
                Py_XSETREF!((*self_).filename2, Py_NewRef!(filename2));
            }
            if nargs >= 2 && nargs <= 5 {
                // filename, filename2, and winerror are removed from the args tuple
                // (for compatibility purposes, see test_exceptions.py)
                let subslice: *mut _object = unsafe { PyTuple_GetSlice(args, 0i64, 2i64) };
                if subslice.is_null() {
                    return 1i32.wrapping_neg();
                }
                Py_DECREF!(args); // replacing args
                let __v4: *mut _object = subslice;
                args = __v4;
                unsafe {
                    *(p_args as *mut *mut _object) = __v4;
                }
            }
        }
    }
    Py_XSETREF!((*self_).myerrno, Py_XNewRef!(myerrno));
    Py_XSETREF!((*self_).strerror, Py_XNewRef!(strerror));
    // Steals the reference to args
    Py_XSETREF!((*self_).args, args);
    args = core::ptr::null_mut::<_object>();
    unsafe {
        *(p_args as *mut *mut _object) = core::ptr::null_mut::<_object>();
    }
    0
}

fn oserror_use_init(r#type: *mut _typeobject) -> i32 {
    // When __init__ is defined in an OSError subclass, we want any
    // extraneous argument to __new__ to be ignored.  The only reasonable
    // solution, given __new__ takes a variable number of arguments,
    // is to defer arg parsing and initialization to __init__.
    //
    // But when __new__ is overridden as well, it should call our __new__
    // with the right arguments.
    //
    // (see http://bugs.python.org/issue12555#msg148829 )
    if (unsafe { (*r#type).tp_init }) != Some(OSError_init)
        && (unsafe { (*r#type).tp_new }) == Some(OSError_new)
    {
        return 1;
    }
    0
}

#[unsafe(link_section = ".text.slate_distinct.exceptions.OSError_new")]
extern "C-unwind" fn OSError_new(
    mut r#type: *mut _typeobject,
    mut args: *mut _object,
    kwds: *mut _object,
) -> *mut _object {
    let mut newtype: *mut _object = unsafe { core::mem::zeroed() };
    let mut __v1: bool = unsafe { core::mem::zeroed() };
    let mut state: *mut _Py_exc_state = unsafe { core::mem::zeroed() };
    let mut filename2: *mut _object = unsafe { core::mem::zeroed() };
    let mut filename: *mut _object = unsafe { core::mem::zeroed() };
    let mut strerror: *mut _object = unsafe { core::mem::zeroed() };
    let mut myerrno: *mut _object = unsafe { core::mem::zeroed() };
    let mut self_: *mut __SlateRecord_pyerrors_h_69 = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_0: {
            self_ = core::ptr::null_mut::<__SlateRecord_pyerrors_h_69>();
            myerrno = core::ptr::null_mut::<_object>();
            strerror = core::ptr::null_mut::<_object>();
            filename = core::ptr::null_mut::<_object>();
            filename2 = core::ptr::null_mut::<_object>();
            Py_INCREF!(args);
            if oserror_use_init(r#type) == 0 {
                if !_PyArg_NoKeywords__2!(unsafe { (*r#type).tp_name }, kwds) {
                    break '__join_0;
                } else {
                    if oserror_parse_args(
                        &(*core::ptr::addr_of_mut!(args)),
                        core::ptr::addr_of_mut!(myerrno),
                        core::ptr::addr_of_mut!(strerror),
                        core::ptr::addr_of_mut!(filename),
                        core::ptr::addr_of_mut!(filename2),
                    ) != 0
                    {
                        break '__join_0;
                    } else {
                        state = get_exc_state();
                        if !myerrno.is_null() {
                            __v1 = PyLong_Check!(myerrno) != 0;
                        } else {
                            __v1 = false;
                        }
                        if __v1
                            && !unsafe { (*state).errnomap }.is_null()
                            && (r#type as *mut _object) == PyExc_OSError
                        {
                            newtype = PyDict_GetItemWithError((*state).errnomap, myerrno);
                            if !newtype.is_null() {
                                r#type = _PyType_CAST!(newtype);
                            } else {
                                if !PyErr_Occurred().is_null() {
                                    break '__join_0;
                                }
                            }
                        }
                    }
                }
            }
            self_ = unsafe { (*r#type).tp_alloc }.unwrap()(r#type, 0i64)
                as *mut __SlateRecord_pyerrors_h_69;
            if !self_.is_null() {
                (*self_).dict = core::ptr::null_mut::<_object>();
                (*self_).context = core::ptr::null_mut::<_object>();
                (*self_).cause = core::ptr::null_mut::<_object>();
                (*self_).traceback = core::ptr::null_mut::<_object>();
                (*self_).written = 1i32.wrapping_neg() as i64;
                if oserror_use_init(r#type) == 0 {
                    if oserror_init(
                        self_,
                        &mut (*core::ptr::addr_of_mut!(args)),
                        myerrno,
                        strerror,
                        filename,
                        filename2,
                    ) != 0
                    {
                        break '__join_0;
                    }
                } else {
                    (*self_).args = PyTuple_New(0i64);
                    if unsafe { (*self_).args }.is_null() {
                        break '__join_0;
                    }
                }
                Py_XDECREF!(args);
                return self_ as *mut _object;
            }
        }
        Py_XDECREF!(args);
        Py_XDECREF__2!(self_);
        return core::ptr::null_mut::<_object>();
    }
}

#[unsafe(link_section = ".text.slate_distinct.exceptions.OSError_init")]
extern "C-unwind" fn OSError_init(
    op: *mut _object,
    mut args: *mut _object,
    kwds: *mut _object,
) -> i32 {
    let mut filename2: *mut _object = unsafe { core::mem::zeroed() };
    let mut filename: *mut _object = unsafe { core::mem::zeroed() };
    let mut strerror: *mut _object = unsafe { core::mem::zeroed() };
    let mut myerrno: *mut _object = unsafe { core::mem::zeroed() };
    let mut self_: *mut __SlateRecord_pyerrors_h_69 = unsafe { core::mem::zeroed() };
    unsafe {
        self_ = PyOSErrorObject_CAST(op);
        myerrno = core::ptr::null_mut::<_object>();
        strerror = core::ptr::null_mut::<_object>();
        filename = core::ptr::null_mut::<_object>();
        filename2 = core::ptr::null_mut::<_object>();
        if oserror_use_init(Py_TYPE__2!(self_)) == 0 {
            return 0;
        } else {
            // Everything already done in OSError_new
            if !_PyArg_NoKeywords__2!(unsafe { (*Py_TYPE__2!(self_)).tp_name }, kwds) {
                return 1i32.wrapping_neg();
            } else {
                Py_INCREF!(args);
                if oserror_parse_args(
                    &(*core::ptr::addr_of_mut!(args)),
                    core::ptr::addr_of_mut!(myerrno),
                    core::ptr::addr_of_mut!(strerror),
                    core::ptr::addr_of_mut!(filename),
                    core::ptr::addr_of_mut!(filename2),
                ) == 0
                {
                    if oserror_init(
                        self_,
                        &mut (*core::ptr::addr_of_mut!(args)),
                        myerrno,
                        strerror,
                        filename,
                        filename2,
                    ) == 0
                    {
                        return 0;
                    }
                }
                Py_DECREF!(args);
                return 1i32.wrapping_neg();
            }
        }
    }
}

#[unsafe(link_section = ".text.slate_distinct.exceptions.OSError_clear")]
extern "C-unwind" fn OSError_clear(op: *mut _object) -> i32 {
    let self_: *mut __SlateRecord_pyerrors_h_69 = PyOSErrorObject_CAST(op);
    Py_CLEAR!((*self_).myerrno);
    Py_CLEAR!((*self_).strerror);
    Py_CLEAR!((*self_).filename);
    Py_CLEAR!((*self_).filename2);
    BaseException_clear(op)
}

#[unsafe(link_section = ".text.slate_distinct.exceptions.OSError_dealloc")]
extern "C-unwind" fn OSError_dealloc(self_: *mut _object) {
    _PyObject_GC_UNTRACK!(self_);
    OSError_clear(self_);
    unsafe { unsafe { (*Py_TYPE!(self_)).tp_free }.unwrap()(self_ as *mut ()) };
}

#[unsafe(link_section = ".text.slate_distinct.exceptions.OSError_traverse")]
extern "C-unwind" fn OSError_traverse(
    op: *mut _object,
    mut visit: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
    arg: *mut (),
) -> i32 {
    let self_: *mut __SlateRecord_pyerrors_h_69 = PyOSErrorObject_CAST(op);
    Py_VISIT!(unsafe { (*self_).myerrno }, visit, arg);
    Py_VISIT!(unsafe { (*self_).strerror }, visit, arg);
    Py_VISIT!(unsafe { (*self_).filename }, visit, arg);
    Py_VISIT!(unsafe { (*self_).filename2 }, visit, arg);
    BaseException_traverse(op, visit, arg)
}

#[unsafe(link_section = ".text.slate_distinct.exceptions.OSError_str")]
extern "C-unwind" fn OSError_str(op: *mut _object) -> *mut _object {
    let self_: *mut __SlateRecord_pyerrors_h_69 = PyOSErrorObject_CAST(op);
    if !unsafe { (*self_).filename }.is_null() {
        if !unsafe { (*self_).filename2 }.is_null() {
            return unsafe {
                PyUnicode_FromFormat(
                    c"[Errno %S] %S: %R -> %R".as_ptr(),
                    OR_NONE!(unsafe { (*self_).myerrno }),
                    OR_NONE!(unsafe { (*self_).strerror }),
                    (*self_).filename,
                    (*self_).filename2,
                )
            };
        } else {
            return unsafe {
                PyUnicode_FromFormat(
                    c"[Errno %S] %S: %R".as_ptr(),
                    OR_NONE!(unsafe { (*self_).myerrno }),
                    OR_NONE!(unsafe { (*self_).strerror }),
                    (*self_).filename,
                )
            };
        }
    }
    if !unsafe { (*self_).myerrno }.is_null() && !unsafe { (*self_).strerror }.is_null() {
        return unsafe {
            PyUnicode_FromFormat(
                c"[Errno %S] %S".as_ptr(),
                (*self_).myerrno,
                (*self_).strerror,
            )
        };
    }
    BaseException_str(op)
}

#[unsafe(link_section = ".text.slate_distinct.exceptions.OSError_reduce")]
extern "C-unwind" fn OSError_reduce(
    op: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    let self_: *mut __SlateRecord_pyerrors_h_69 = PyOSErrorObject_CAST(op);
    let mut args: *mut _object = unsafe { (*self_).args };
    let mut res: *mut _object = core::ptr::null_mut::<_object>();
    // self->args is only the first two real arguments if there was a
    // file name given to OSError.
    if PyTuple_GET_SIZE!(args) == 2 && !unsafe { (*self_).filename }.is_null() {
        let size: i64 = (if !unsafe { (*self_).filename2 }.is_null() {
            5i32
        } else {
            3i32
        }) as i64;
        args = unsafe { PyTuple_New(size) };
        if args.is_null() {
            return core::ptr::null_mut::<_object>();
        }
        PyTuple_SET_ITEM!(
            args,
            0i32,
            Py_NewRef!(PyTuple_GET_ITEM!(unsafe { (*self_).args }, 0i32))
        );
        PyTuple_SET_ITEM!(
            args,
            1i32,
            Py_NewRef!(PyTuple_GET_ITEM!(unsafe { (*self_).args }, 1i32))
        );
        PyTuple_SET_ITEM!(args, 2i32, Py_NewRef!(unsafe { (*self_).filename }));
        if !unsafe { (*self_).filename2 }.is_null() {
            // This tuple is essentially used as OSError(*args).
            // So, to recreate filename2, we need to pass in
            // winerror as well.
            PyTuple_SET_ITEM!(
                args,
                3i32,
                Py_NewRef!(unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) })
            );
            // filename2
            PyTuple_SET_ITEM!(args, 4i32, Py_NewRef!(unsafe { (*self_).filename2 }));
        }
    } else {
        Py_INCREF!(args);
    }
    if !unsafe { (*self_).dict }.is_null() {
        res = unsafe { PyTuple_Pack(3i64, Py_TYPE__2!(self_), args, (*self_).dict) };
    } else {
        res = unsafe { PyTuple_Pack(2i64, Py_TYPE__2!(self_), args) };
    }
    Py_DECREF!(args);
    res
}

#[unsafe(link_section = ".text.slate_distinct.exceptions.OSError_written_get")]
extern "C-unwind" fn OSError_written_get(op: *mut _object, context: *mut ()) -> *mut _object {
    let self_: *mut __SlateRecord_pyerrors_h_69 = PyOSErrorObject_CAST(op);
    if (unsafe { (*self_).written }) == (1i32.wrapping_neg() as i64) {
        unsafe { PyErr_SetString(PyExc_AttributeError, c"characters_written".as_ptr()) };
        return core::ptr::null_mut::<_object>();
    }
    unsafe { PyLong_FromSsize_t((*self_).written) }
}

#[unsafe(link_section = ".text.slate_distinct.exceptions.OSError_written_set")]
extern "C-unwind" fn OSError_written_set(
    op: *mut _object,
    arg: *mut _object,
    context: *mut (),
) -> i32 {
    let self_: *mut __SlateRecord_pyerrors_h_69 = PyOSErrorObject_CAST(op);
    if arg.is_null() {
        if (unsafe { (*self_).written }) == (1i32.wrapping_neg() as i64) {
            unsafe { PyErr_SetString(PyExc_AttributeError, c"characters_written".as_ptr()) };
            return 1i32.wrapping_neg();
        }
        unsafe {
            (*self_).written = 1i32.wrapping_neg() as i64;
        }
        return 0;
    }
    let n: i64 = unsafe { PyNumber_AsSsize_t(arg, PyExc_ValueError) };
    let __v0: bool = n == (1i32.wrapping_neg() as i64) && !unsafe { PyErr_Occurred() }.is_null();
    if __v0 {
        return 1i32.wrapping_neg();
    }
    unsafe {
        (*self_).written = n;
    }
    0
}

/// Sentinel
static mut OSError_members: __SlateAlign16<[PyMemberDef; 5]> = __SlateAlign16([
    {
        let mut __t0: PyMemberDef = unsafe { core::mem::zeroed() };
        __t0.name = (b"errno\0".as_ptr() as *mut i8) as *const i8;
        __t0.r#type = 6i32;
        __t0.offset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_69, myerrno) as u64) as i64;
        __t0.flags = 0i32;
        __t0.doc = (b"POSIX exception code\0".as_ptr() as *mut i8) as *const i8;
        __t0
    },
    {
        let mut __t1: PyMemberDef = unsafe { core::mem::zeroed() };
        __t1.name = (b"strerror\0".as_ptr() as *mut i8) as *const i8;
        __t1.r#type = 6i32;
        __t1.offset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_69, strerror) as u64) as i64;
        __t1.flags = 0i32;
        __t1.doc = (b"exception strerror\0".as_ptr() as *mut i8) as *const i8;
        __t1
    },
    {
        let mut __t2: PyMemberDef = unsafe { core::mem::zeroed() };
        __t2.name = (b"filename\0".as_ptr() as *mut i8) as *const i8;
        __t2.r#type = 6i32;
        __t2.offset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_69, filename) as u64) as i64;
        __t2.flags = 0i32;
        __t2.doc = (b"exception filename\0".as_ptr() as *mut i8) as *const i8;
        __t2
    },
    {
        let mut __t3: PyMemberDef = unsafe { core::mem::zeroed() };
        __t3.name = (b"filename2\0".as_ptr() as *mut i8) as *const i8;
        __t3.r#type = 6i32;
        __t3.offset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_69, filename2) as u64) as i64;
        __t3.flags = 0i32;
        __t3.doc = (b"second exception filename\0".as_ptr() as *mut i8) as *const i8;
        __t3
    },
    {
        let mut __t4: PyMemberDef = unsafe { core::mem::zeroed() };
        __t4.name = core::ptr::null::<i8>();
        __t4
    },
]);

static mut OSError_methods: __SlateAlign16<[PyMethodDef; 2]> = __SlateAlign16([
    {
        let mut __t0: PyMethodDef = unsafe { core::mem::zeroed() };
        __t0.ml_name = (b"__reduce__\0".as_ptr() as *mut i8) as *const i8;
        __t0.ml_meth = Some(OSError_reduce);
        __t0.ml_flags = 4i32;
        __t0
    },
    {
        let mut __t1: PyMethodDef = unsafe { core::mem::zeroed() };
        __t1.ml_name = core::ptr::null::<i8>();
        __t1
    },
]);

static mut OSError_getset: __SlateAlign16<[PyGetSetDef; 2]> = __SlateAlign16([
    {
        let mut __t0: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t0.name = (b"characters_written\0".as_ptr() as *mut i8) as *const i8;
        __t0.get = Some(OSError_written_get);
        __t0.set = Some(OSError_written_set);
        __t0.doc = core::ptr::null::<i8>();
        __t0
    },
    {
        let mut __t1: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t1.name = core::ptr::null::<i8>();
        __t1
    },
]);

static mut _PyExc_OSError: _typeobject = {
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
    __t0.tp_name = (b"OSError\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pyerrors_h_69>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(OSError_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = None;
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = Some(OSError_str);
    __t0.tp_getattro = None;
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 17408u64;
    __t0.tp_doc = (b"Base class for I/O related errors.\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_traverse = Some(OSError_traverse);
    __t0.tp_clear = Some(OSError_clear);
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = unsafe { core::ptr::addr_of_mut!(OSError_methods.0) as *mut PyMethodDef };
    __t0.tp_members = unsafe { core::ptr::addr_of_mut!(OSError_members.0) as *mut PyMemberDef };
    __t0.tp_getset = unsafe { core::ptr::addr_of_mut!(OSError_getset.0) as *mut PyGetSetDef };
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(_PyExc_Exception) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_69, dict) as u64) as i64;
    __t0.tp_init = Some(OSError_init);
    __t0.tp_alloc = None;
    __t0.tp_new = Some(OSError_new);
    __t0
};

#[unsafe(no_mangle)]
static mut PyExc_OSError: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_OSError) }) as *mut _object;

// Various OSError subclasses
static mut _PyExc_BlockingIOError: _typeobject = {
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
    __t0.tp_name = (b"BlockingIOError\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pyerrors_h_69>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(OSError_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = None;
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = None;
    __t0.tp_getattro = None;
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 17408u64;
    __t0.tp_doc = (b"I/O operation would block.\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_traverse = Some(OSError_traverse);
    __t0.tp_clear = Some(OSError_clear);
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = core::ptr::null_mut::<PyMethodDef>();
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(_PyExc_OSError) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_69, dict) as u64) as i64;
    __t0.tp_init = Some(OSError_init);
    __t0.tp_alloc = None;
    __t0.tp_new = None;
    __t0
};

#[unsafe(no_mangle)]
static mut PyExc_BlockingIOError: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_BlockingIOError) }) as *mut _object;

static mut _PyExc_ConnectionError: _typeobject = {
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
    __t0.tp_name = (b"ConnectionError\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pyerrors_h_69>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(OSError_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = None;
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = None;
    __t0.tp_getattro = None;
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 17408u64;
    __t0.tp_doc = (b"Connection error.\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_traverse = Some(OSError_traverse);
    __t0.tp_clear = Some(OSError_clear);
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = core::ptr::null_mut::<PyMethodDef>();
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(_PyExc_OSError) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_69, dict) as u64) as i64;
    __t0.tp_init = Some(OSError_init);
    __t0.tp_alloc = None;
    __t0.tp_new = None;
    __t0
};

#[unsafe(no_mangle)]
static mut PyExc_ConnectionError: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_ConnectionError) }) as *mut _object;

static mut _PyExc_ChildProcessError: _typeobject = {
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
    __t0.tp_name = (b"ChildProcessError\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pyerrors_h_69>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(OSError_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = None;
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = None;
    __t0.tp_getattro = None;
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 17408u64;
    __t0.tp_doc = (b"Child process error.\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_traverse = Some(OSError_traverse);
    __t0.tp_clear = Some(OSError_clear);
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = core::ptr::null_mut::<PyMethodDef>();
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(_PyExc_OSError) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_69, dict) as u64) as i64;
    __t0.tp_init = Some(OSError_init);
    __t0.tp_alloc = None;
    __t0.tp_new = None;
    __t0
};

#[unsafe(no_mangle)]
static mut PyExc_ChildProcessError: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_ChildProcessError) }) as *mut _object;

static mut _PyExc_BrokenPipeError: _typeobject = {
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
    __t0.tp_name = (b"BrokenPipeError\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pyerrors_h_69>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(OSError_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = None;
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = None;
    __t0.tp_getattro = None;
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 17408u64;
    __t0.tp_doc = (b"Broken pipe.\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_traverse = Some(OSError_traverse);
    __t0.tp_clear = Some(OSError_clear);
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = core::ptr::null_mut::<PyMethodDef>();
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(_PyExc_ConnectionError) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_69, dict) as u64) as i64;
    __t0.tp_init = Some(OSError_init);
    __t0.tp_alloc = None;
    __t0.tp_new = None;
    __t0
};

#[unsafe(no_mangle)]
static mut PyExc_BrokenPipeError: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_BrokenPipeError) }) as *mut _object;

static mut _PyExc_ConnectionAbortedError: _typeobject = {
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
    __t0.tp_name = (b"ConnectionAbortedError\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pyerrors_h_69>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(OSError_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = None;
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = None;
    __t0.tp_getattro = None;
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 17408u64;
    __t0.tp_doc = (b"Connection aborted.\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_traverse = Some(OSError_traverse);
    __t0.tp_clear = Some(OSError_clear);
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = core::ptr::null_mut::<PyMethodDef>();
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(_PyExc_ConnectionError) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_69, dict) as u64) as i64;
    __t0.tp_init = Some(OSError_init);
    __t0.tp_alloc = None;
    __t0.tp_new = None;
    __t0
};

#[unsafe(no_mangle)]
static mut PyExc_ConnectionAbortedError: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_ConnectionAbortedError) }) as *mut _object;

static mut _PyExc_ConnectionRefusedError: _typeobject = {
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
    __t0.tp_name = (b"ConnectionRefusedError\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pyerrors_h_69>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(OSError_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = None;
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = None;
    __t0.tp_getattro = None;
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 17408u64;
    __t0.tp_doc = (b"Connection refused.\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_traverse = Some(OSError_traverse);
    __t0.tp_clear = Some(OSError_clear);
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = core::ptr::null_mut::<PyMethodDef>();
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(_PyExc_ConnectionError) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_69, dict) as u64) as i64;
    __t0.tp_init = Some(OSError_init);
    __t0.tp_alloc = None;
    __t0.tp_new = None;
    __t0
};

#[unsafe(no_mangle)]
static mut PyExc_ConnectionRefusedError: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_ConnectionRefusedError) }) as *mut _object;

static mut _PyExc_ConnectionResetError: _typeobject = {
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
    __t0.tp_name = (b"ConnectionResetError\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pyerrors_h_69>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(OSError_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = None;
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = None;
    __t0.tp_getattro = None;
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 17408u64;
    __t0.tp_doc = (b"Connection reset.\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_traverse = Some(OSError_traverse);
    __t0.tp_clear = Some(OSError_clear);
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = core::ptr::null_mut::<PyMethodDef>();
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(_PyExc_ConnectionError) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_69, dict) as u64) as i64;
    __t0.tp_init = Some(OSError_init);
    __t0.tp_alloc = None;
    __t0.tp_new = None;
    __t0
};

#[unsafe(no_mangle)]
static mut PyExc_ConnectionResetError: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_ConnectionResetError) }) as *mut _object;

static mut _PyExc_FileExistsError: _typeobject = {
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
    __t0.tp_name = (b"FileExistsError\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pyerrors_h_69>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(OSError_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = None;
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = None;
    __t0.tp_getattro = None;
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 17408u64;
    __t0.tp_doc = (b"File already exists.\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_traverse = Some(OSError_traverse);
    __t0.tp_clear = Some(OSError_clear);
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = core::ptr::null_mut::<PyMethodDef>();
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(_PyExc_OSError) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_69, dict) as u64) as i64;
    __t0.tp_init = Some(OSError_init);
    __t0.tp_alloc = None;
    __t0.tp_new = None;
    __t0
};

#[unsafe(no_mangle)]
static mut PyExc_FileExistsError: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_FileExistsError) }) as *mut _object;

static mut _PyExc_FileNotFoundError: _typeobject = {
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
    __t0.tp_name = (b"FileNotFoundError\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pyerrors_h_69>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(OSError_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = None;
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = None;
    __t0.tp_getattro = None;
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 17408u64;
    __t0.tp_doc = (b"File not found.\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_traverse = Some(OSError_traverse);
    __t0.tp_clear = Some(OSError_clear);
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = core::ptr::null_mut::<PyMethodDef>();
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(_PyExc_OSError) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_69, dict) as u64) as i64;
    __t0.tp_init = Some(OSError_init);
    __t0.tp_alloc = None;
    __t0.tp_new = None;
    __t0
};

#[unsafe(no_mangle)]
static mut PyExc_FileNotFoundError: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_FileNotFoundError) }) as *mut _object;

static mut _PyExc_IsADirectoryError: _typeobject = {
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
    __t0.tp_name = (b"IsADirectoryError\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pyerrors_h_69>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(OSError_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = None;
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = None;
    __t0.tp_getattro = None;
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 17408u64;
    __t0.tp_doc = (b"Operation doesn't work on directories.\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_traverse = Some(OSError_traverse);
    __t0.tp_clear = Some(OSError_clear);
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = core::ptr::null_mut::<PyMethodDef>();
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(_PyExc_OSError) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_69, dict) as u64) as i64;
    __t0.tp_init = Some(OSError_init);
    __t0.tp_alloc = None;
    __t0.tp_new = None;
    __t0
};

#[unsafe(no_mangle)]
static mut PyExc_IsADirectoryError: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_IsADirectoryError) }) as *mut _object;

static mut _PyExc_NotADirectoryError: _typeobject = {
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
    __t0.tp_name = (b"NotADirectoryError\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pyerrors_h_69>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(OSError_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = None;
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = None;
    __t0.tp_getattro = None;
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 17408u64;
    __t0.tp_doc = (b"Operation only works on directories.\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_traverse = Some(OSError_traverse);
    __t0.tp_clear = Some(OSError_clear);
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = core::ptr::null_mut::<PyMethodDef>();
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(_PyExc_OSError) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_69, dict) as u64) as i64;
    __t0.tp_init = Some(OSError_init);
    __t0.tp_alloc = None;
    __t0.tp_new = None;
    __t0
};

#[unsafe(no_mangle)]
static mut PyExc_NotADirectoryError: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_NotADirectoryError) }) as *mut _object;

static mut _PyExc_InterruptedError: _typeobject = {
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
    __t0.tp_name = (b"InterruptedError\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pyerrors_h_69>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(OSError_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = None;
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = None;
    __t0.tp_getattro = None;
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 17408u64;
    __t0.tp_doc = (b"Interrupted by signal.\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_traverse = Some(OSError_traverse);
    __t0.tp_clear = Some(OSError_clear);
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = core::ptr::null_mut::<PyMethodDef>();
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(_PyExc_OSError) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_69, dict) as u64) as i64;
    __t0.tp_init = Some(OSError_init);
    __t0.tp_alloc = None;
    __t0.tp_new = None;
    __t0
};

#[unsafe(no_mangle)]
static mut PyExc_InterruptedError: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_InterruptedError) }) as *mut _object;

static mut _PyExc_PermissionError: _typeobject = {
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
    __t0.tp_name = (b"PermissionError\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pyerrors_h_69>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(OSError_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = None;
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = None;
    __t0.tp_getattro = None;
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 17408u64;
    __t0.tp_doc = (b"Not enough permissions.\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_traverse = Some(OSError_traverse);
    __t0.tp_clear = Some(OSError_clear);
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = core::ptr::null_mut::<PyMethodDef>();
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(_PyExc_OSError) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_69, dict) as u64) as i64;
    __t0.tp_init = Some(OSError_init);
    __t0.tp_alloc = None;
    __t0.tp_new = None;
    __t0
};

#[unsafe(no_mangle)]
static mut PyExc_PermissionError: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_PermissionError) }) as *mut _object;

static mut _PyExc_ProcessLookupError: _typeobject = {
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
    __t0.tp_name = (b"ProcessLookupError\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pyerrors_h_69>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(OSError_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = None;
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = None;
    __t0.tp_getattro = None;
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 17408u64;
    __t0.tp_doc = (b"Process not found.\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_traverse = Some(OSError_traverse);
    __t0.tp_clear = Some(OSError_clear);
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = core::ptr::null_mut::<PyMethodDef>();
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(_PyExc_OSError) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_69, dict) as u64) as i64;
    __t0.tp_init = Some(OSError_init);
    __t0.tp_alloc = None;
    __t0.tp_new = None;
    __t0
};

#[unsafe(no_mangle)]
static mut PyExc_ProcessLookupError: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_ProcessLookupError) }) as *mut _object;

static mut _PyExc_TimeoutError: _typeobject = {
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
    __t0.tp_name = (b"TimeoutError\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pyerrors_h_69>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(OSError_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = None;
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = None;
    __t0.tp_getattro = None;
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 17408u64;
    __t0.tp_doc = (b"Timeout expired.\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_traverse = Some(OSError_traverse);
    __t0.tp_clear = Some(OSError_clear);
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = core::ptr::null_mut::<PyMethodDef>();
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(_PyExc_OSError) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_69, dict) as u64) as i64;
    __t0.tp_init = Some(OSError_init);
    __t0.tp_alloc = None;
    __t0.tp_new = None;
    __t0
};

#[unsafe(no_mangle)]
static mut PyExc_TimeoutError: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_TimeoutError) }) as *mut _object;

// EOFError extends Exception
static mut _PyExc_EOFError: _typeobject = {
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
    __t0.tp_name = (b"EOFError\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pyerrors_h_15>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(BaseException_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = None;
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = None;
    __t0.tp_getattro = None;
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 17408u64;
    __t0.tp_doc = (b"Read beyond end of file.\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_traverse = Some(BaseException_traverse);
    __t0.tp_clear = Some(BaseException_clear);
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = core::ptr::null_mut::<PyMethodDef>();
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(_PyExc_Exception) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_15, dict) as u64) as i64;
    __t0.tp_init = Some(BaseException_init);
    __t0.tp_alloc = None;
    __t0.tp_new = Some(BaseException_new);
    __t0
};

#[unsafe(no_mangle)]
static mut PyExc_EOFError: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_EOFError) }) as *mut _object;

// RuntimeError extends Exception
static mut _PyExc_RuntimeError: _typeobject = {
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
    __t0.tp_name = (b"RuntimeError\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pyerrors_h_15>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(BaseException_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = None;
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = None;
    __t0.tp_getattro = None;
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 17408u64;
    __t0.tp_doc = (b"Unspecified run-time error.\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_traverse = Some(BaseException_traverse);
    __t0.tp_clear = Some(BaseException_clear);
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = core::ptr::null_mut::<PyMethodDef>();
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(_PyExc_Exception) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_15, dict) as u64) as i64;
    __t0.tp_init = Some(BaseException_init);
    __t0.tp_alloc = None;
    __t0.tp_new = Some(BaseException_new);
    __t0
};

#[unsafe(no_mangle)]
static mut PyExc_RuntimeError: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_RuntimeError) }) as *mut _object;

// RecursionError extends RuntimeError
static mut _PyExc_RecursionError: _typeobject = {
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
    __t0.tp_name = (b"RecursionError\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pyerrors_h_15>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(BaseException_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = None;
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = None;
    __t0.tp_getattro = None;
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 17408u64;
    __t0.tp_doc = (b"Recursion limit exceeded.\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_traverse = Some(BaseException_traverse);
    __t0.tp_clear = Some(BaseException_clear);
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = core::ptr::null_mut::<PyMethodDef>();
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(_PyExc_RuntimeError) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_15, dict) as u64) as i64;
    __t0.tp_init = Some(BaseException_init);
    __t0.tp_alloc = None;
    __t0.tp_new = Some(BaseException_new);
    __t0
};

#[unsafe(no_mangle)]
static mut PyExc_RecursionError: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_RecursionError) }) as *mut _object;

// PythonFinalizationError extends RuntimeError
static mut _PyExc_PythonFinalizationError: _typeobject = {
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
    __t0.tp_name = (b"PythonFinalizationError\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pyerrors_h_15>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(BaseException_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = None;
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = None;
    __t0.tp_getattro = None;
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 17408u64;
    __t0.tp_doc =
        (b"Operation blocked during Python finalization.\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_traverse = Some(BaseException_traverse);
    __t0.tp_clear = Some(BaseException_clear);
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = core::ptr::null_mut::<PyMethodDef>();
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(_PyExc_RuntimeError) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_15, dict) as u64) as i64;
    __t0.tp_init = Some(BaseException_init);
    __t0.tp_alloc = None;
    __t0.tp_new = Some(BaseException_new);
    __t0
};

#[unsafe(no_mangle)]
static mut PyExc_PythonFinalizationError: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_PythonFinalizationError) }) as *mut _object;

// NotImplementedError extends RuntimeError
static mut _PyExc_NotImplementedError: _typeobject = {
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
    __t0.tp_name = (b"NotImplementedError\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pyerrors_h_15>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(BaseException_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = None;
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = None;
    __t0.tp_getattro = None;
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 17408u64;
    __t0.tp_doc =
        (b"Method or function hasn't been implemented yet.\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_traverse = Some(BaseException_traverse);
    __t0.tp_clear = Some(BaseException_clear);
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = core::ptr::null_mut::<PyMethodDef>();
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(_PyExc_RuntimeError) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_15, dict) as u64) as i64;
    __t0.tp_init = Some(BaseException_init);
    __t0.tp_alloc = None;
    __t0.tp_new = Some(BaseException_new);
    __t0
};

#[unsafe(no_mangle)]
static mut PyExc_NotImplementedError: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_NotImplementedError) }) as *mut _object;

// NameError extends Exception
fn PyNameErrorObject_CAST(self_: *mut _object) -> *mut __SlateRecord_pyerrors_h_79 {
    self_ as *mut __SlateRecord_pyerrors_h_79
}

#[unsafe(link_section = ".text.slate_distinct.exceptions.NameError_init")]
extern "C-unwind" fn NameError_init(
    op: *mut _object,
    args: *mut _object,
    kwds: *mut _object,
) -> i32 {
    let mut name: *mut _object = core::ptr::null_mut::<_object>();
    if BaseException_init(op, args, core::ptr::null_mut::<_object>()) == 1i32.wrapping_neg() {
        return 1i32.wrapping_neg();
    }
    let empty_tuple: *mut _object = unsafe { PyTuple_New(0i64) };
    if empty_tuple.is_null() {
        return 1i32.wrapping_neg();
    }
    if (unsafe {
        PyArg_ParseTupleAndKeywords(
            empty_tuple,
            kwds,
            c"|$O:NameError".as_ptr(),
            (core::ptr::addr_of_mut!(kwlist__2511.0) as *mut *mut i8) as *const *mut i8,
            core::ptr::addr_of_mut!(name),
        )
    }) == 0
    {
        Py_DECREF!(empty_tuple);
        return 1i32.wrapping_neg();
    }
    Py_DECREF!(empty_tuple);
    let self_: *mut __SlateRecord_pyerrors_h_79 = PyNameErrorObject_CAST(op);
    Py_XSETREF!((*self_).name, Py_XNewRef!(name));
    0
}

static mut kwlist__2511: __SlateAlign16<[*mut i8; 2]> =
    __SlateAlign16([b"name\0".as_ptr() as *mut i8, core::ptr::null_mut::<i8>()]);

#[unsafe(link_section = ".text.slate_distinct.exceptions.NameError_clear")]
extern "C-unwind" fn NameError_clear(op: *mut _object) -> i32 {
    let self_: *mut __SlateRecord_pyerrors_h_79 = PyNameErrorObject_CAST(op);
    Py_CLEAR!((*self_).name);
    BaseException_clear(op)
}

#[unsafe(link_section = ".text.slate_distinct.exceptions.NameError_dealloc")]
extern "C-unwind" fn NameError_dealloc(self_: *mut _object) {
    _PyObject_GC_UNTRACK!(self_);
    NameError_clear(self_);
    unsafe { unsafe { (*Py_TYPE!(self_)).tp_free }.unwrap()(self_ as *mut ()) };
}

#[unsafe(link_section = ".text.slate_distinct.exceptions.NameError_traverse")]
extern "C-unwind" fn NameError_traverse(
    op: *mut _object,
    mut visit: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
    arg: *mut (),
) -> i32 {
    let self_: *mut __SlateRecord_pyerrors_h_79 = PyNameErrorObject_CAST(op);
    Py_VISIT!(unsafe { (*self_).name }, visit, arg);
    BaseException_traverse(op, visit, arg)
}

/// Sentinel
static mut NameError_members: __SlateAlign16<[PyMemberDef; 2]> = __SlateAlign16([
    {
        let mut __t0: PyMemberDef = unsafe { core::mem::zeroed() };
        __t0.name = (b"name\0".as_ptr() as *mut i8) as *const i8;
        __t0.r#type = 6i32;
        __t0.offset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_79, name) as u64) as i64;
        __t0.flags = 0i32;
        __t0.doc = (b"name\0".as_ptr() as *mut i8) as *const i8;
        __t0
    },
    {
        let mut __t1: PyMemberDef = unsafe { core::mem::zeroed() };
        __t1.name = core::ptr::null::<i8>();
        __t1
    },
]);

/// Sentinel
static mut NameError_methods: __SlateAlign16<[PyMethodDef; 1]> = __SlateAlign16([{
    let mut __t0: PyMethodDef = unsafe { core::mem::zeroed() };
    __t0.ml_name = core::ptr::null::<i8>();
    __t0
}]);

static mut _PyExc_NameError: _typeobject = {
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
    __t0.tp_name = (b"NameError\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pyerrors_h_79>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(NameError_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = None;
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = Some(BaseException_str);
    __t0.tp_getattro = None;
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 17408u64;
    __t0.tp_doc = (b"Name not found globally.\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_traverse = Some(NameError_traverse);
    __t0.tp_clear = Some(NameError_clear);
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = unsafe { core::ptr::addr_of_mut!(NameError_methods.0) as *mut PyMethodDef };
    __t0.tp_members = unsafe { core::ptr::addr_of_mut!(NameError_members.0) as *mut PyMemberDef };
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(_PyExc_Exception) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_79, dict) as u64) as i64;
    __t0.tp_init = Some(NameError_init);
    __t0.tp_alloc = None;
    __t0.tp_new = None;
    __t0
};

#[unsafe(no_mangle)]
static mut PyExc_NameError: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_NameError) }) as *mut _object;

// UnboundLocalError extends NameError
static mut _PyExc_UnboundLocalError: _typeobject = {
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
    __t0.tp_name = (b"UnboundLocalError\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pyerrors_h_79>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(NameError_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = None;
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = None;
    __t0.tp_getattro = None;
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 17408u64;
    __t0.tp_doc =
        (b"Local name referenced but not bound to a value.\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_traverse = Some(NameError_traverse);
    __t0.tp_clear = Some(NameError_clear);
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = core::ptr::null_mut::<PyMethodDef>();
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(_PyExc_NameError) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_79, dict) as u64) as i64;
    __t0.tp_init = Some(NameError_init);
    __t0.tp_alloc = None;
    __t0.tp_new = None;
    __t0
};

#[unsafe(no_mangle)]
static mut PyExc_UnboundLocalError: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_UnboundLocalError) }) as *mut _object;

// AttributeError extends Exception
fn PyAttributeErrorObject_CAST(self_: *mut _object) -> *mut __SlateRecord_pyerrors_h_85 {
    self_ as *mut __SlateRecord_pyerrors_h_85
}

#[unsafe(link_section = ".text.slate_distinct.exceptions.AttributeError_init")]
extern "C-unwind" fn AttributeError_init(
    op: *mut _object,
    args: *mut _object,
    kwds: *mut _object,
) -> i32 {
    let mut name: *mut _object = core::ptr::null_mut::<_object>();
    let mut obj: *mut _object = core::ptr::null_mut::<_object>();
    if BaseException_init(op, args, core::ptr::null_mut::<_object>()) == 1i32.wrapping_neg() {
        return 1i32.wrapping_neg();
    }
    let empty_tuple: *mut _object = unsafe { PyTuple_New(0i64) };
    if empty_tuple.is_null() {
        return 1i32.wrapping_neg();
    }
    if (unsafe {
        PyArg_ParseTupleAndKeywords(
            empty_tuple,
            kwds,
            c"|$OO:AttributeError".as_ptr(),
            (core::ptr::addr_of_mut!(kwlist__2594.0) as *mut *mut i8) as *const *mut i8,
            core::ptr::addr_of_mut!(name),
            core::ptr::addr_of_mut!(obj),
        )
    }) == 0
    {
        Py_DECREF!(empty_tuple);
        return 1i32.wrapping_neg();
    }
    Py_DECREF!(empty_tuple);
    let self_: *mut __SlateRecord_pyerrors_h_85 = PyAttributeErrorObject_CAST(op);
    Py_XSETREF!((*self_).name, Py_XNewRef!(name));
    Py_XSETREF!((*self_).obj, Py_XNewRef!(obj));
    0
}

static mut kwlist__2594: __SlateAlign16<[*mut i8; 3]> = __SlateAlign16([
    b"name\0".as_ptr() as *mut i8,
    b"obj\0".as_ptr() as *mut i8,
    core::ptr::null_mut::<i8>(),
]);

#[unsafe(link_section = ".text.slate_distinct.exceptions.AttributeError_clear")]
extern "C-unwind" fn AttributeError_clear(op: *mut _object) -> i32 {
    let self_: *mut __SlateRecord_pyerrors_h_85 = PyAttributeErrorObject_CAST(op);
    Py_CLEAR!((*self_).obj);
    Py_CLEAR!((*self_).name);
    BaseException_clear(op)
}

#[unsafe(link_section = ".text.slate_distinct.exceptions.AttributeError_dealloc")]
extern "C-unwind" fn AttributeError_dealloc(self_: *mut _object) {
    _PyObject_GC_UNTRACK!(self_);
    AttributeError_clear(self_);
    unsafe { unsafe { (*Py_TYPE!(self_)).tp_free }.unwrap()(self_ as *mut ()) };
}

#[unsafe(link_section = ".text.slate_distinct.exceptions.AttributeError_traverse")]
extern "C-unwind" fn AttributeError_traverse(
    op: *mut _object,
    mut visit: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
    arg: *mut (),
) -> i32 {
    let self_: *mut __SlateRecord_pyerrors_h_85 = PyAttributeErrorObject_CAST(op);
    Py_VISIT!(unsafe { (*self_).obj }, visit, arg);
    Py_VISIT!(unsafe { (*self_).name }, visit, arg);
    BaseException_traverse(op, visit, arg)
}

/// Pickling support
#[unsafe(link_section = ".text.slate_distinct.exceptions.AttributeError_getstate")]
extern "C-unwind" fn AttributeError_getstate(
    op: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    let self_: *mut __SlateRecord_pyerrors_h_85 = PyAttributeErrorObject_CAST(op);
    let mut dict: *mut _object = unsafe { (*self_).dict };
    if !unsafe { (*self_).name }.is_null() || !unsafe { (*self_).args }.is_null() {
        let __v0: *mut _object = if !dict.is_null() {
            unsafe { PyDict_Copy(dict) }
        } else {
            unsafe { PyDict_New() }
        };
        dict = __v0;
        if dict.is_null() {
            return core::ptr::null_mut::<_object>();
        }
        let __v1: bool = !unsafe { (*self_).name }.is_null()
            && (unsafe { PyDict_SetItemString(dict, c"name".as_ptr(), (*self_).name) }) < 0;
        if __v1 {
            Py_DECREF!(dict);
            return core::ptr::null_mut::<_object>();
        }
        // We specifically are not pickling the obj attribute since there are many
        // cases where it is unlikely to be picklable. See GH-103352.
        let __v2: bool = !unsafe { (*self_).args }.is_null()
            && (unsafe { PyDict_SetItemString(dict, c"args".as_ptr(), (*self_).args) }) < 0;
        if __v2 {
            Py_DECREF!(dict);
            return core::ptr::null_mut::<_object>();
        }
        return dict;
    } else {
        if !dict.is_null() {
            return Py_NewRef!(dict);
        }
    }
    unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
}

#[unsafe(link_section = ".text.slate_distinct.exceptions.AttributeError_reduce")]
extern "C-unwind" fn AttributeError_reduce(
    op: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    let state: *mut _object = AttributeError_getstate(op, core::ptr::null_mut::<_object>());
    if state.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let self_: *mut __SlateRecord_pyerrors_h_85 = PyAttributeErrorObject_CAST(op);
    let return_value: *mut _object =
        unsafe { PyTuple_Pack(3i64, Py_TYPE__2!(self_), (*self_).args, state) };
    Py_DECREF!(state);
    return_value
}

/// Sentinel
static mut AttributeError_members: __SlateAlign16<[PyMemberDef; 3]> = __SlateAlign16([
    {
        let mut __t0: PyMemberDef = unsafe { core::mem::zeroed() };
        __t0.name = (b"name\0".as_ptr() as *mut i8) as *const i8;
        __t0.r#type = 6i32;
        __t0.offset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_85, name) as u64) as i64;
        __t0.flags = 0i32;
        __t0.doc = (b"attribute name\0".as_ptr() as *mut i8) as *const i8;
        __t0
    },
    {
        let mut __t1: PyMemberDef = unsafe { core::mem::zeroed() };
        __t1.name = (b"obj\0".as_ptr() as *mut i8) as *const i8;
        __t1.r#type = 6i32;
        __t1.offset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_85, obj) as u64) as i64;
        __t1.flags = 0i32;
        __t1.doc = (b"object\0".as_ptr() as *mut i8) as *const i8;
        __t1
    },
    {
        let mut __t2: PyMemberDef = unsafe { core::mem::zeroed() };
        __t2.name = core::ptr::null::<i8>();
        __t2
    },
]);

static mut AttributeError_methods: __SlateAlign16<[PyMethodDef; 3]> = __SlateAlign16([
    {
        let mut __t0: PyMethodDef = unsafe { core::mem::zeroed() };
        __t0.ml_name = (b"__getstate__\0".as_ptr() as *mut i8) as *const i8;
        __t0.ml_meth = Some(AttributeError_getstate);
        __t0.ml_flags = 4i32;
        __t0
    },
    {
        let mut __t1: PyMethodDef = unsafe { core::mem::zeroed() };
        __t1.ml_name = (b"__reduce__\0".as_ptr() as *mut i8) as *const i8;
        __t1.ml_meth = Some(AttributeError_reduce);
        __t1.ml_flags = 4i32;
        __t1
    },
    {
        let mut __t2: PyMethodDef = unsafe { core::mem::zeroed() };
        __t2.ml_name = core::ptr::null::<i8>();
        __t2
    },
]);

static mut _PyExc_AttributeError: _typeobject = {
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
    __t0.tp_name = (b"AttributeError\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pyerrors_h_85>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(AttributeError_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = None;
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = Some(BaseException_str);
    __t0.tp_getattro = None;
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 17408u64;
    __t0.tp_doc = (b"Attribute not found.\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_traverse = Some(AttributeError_traverse);
    __t0.tp_clear = Some(AttributeError_clear);
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods =
        unsafe { core::ptr::addr_of_mut!(AttributeError_methods.0) as *mut PyMethodDef };
    __t0.tp_members =
        unsafe { core::ptr::addr_of_mut!(AttributeError_members.0) as *mut PyMemberDef };
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(_PyExc_Exception) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_85, dict) as u64) as i64;
    __t0.tp_init = Some(AttributeError_init);
    __t0.tp_alloc = None;
    __t0.tp_new = None;
    __t0
};

#[unsafe(no_mangle)]
static mut PyExc_AttributeError: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_AttributeError) }) as *mut _object;

// SyntaxError extends Exception
fn PySyntaxErrorObject_CAST(self_: *mut _object) -> *mut __SlateRecord_pyerrors_h_35 {
    self_ as *mut __SlateRecord_pyerrors_h_35
}

#[unsafe(link_section = ".text.slate_distinct.exceptions.SyntaxError_init")]
extern "C-unwind" fn SyntaxError_init(
    op: *mut _object,
    args: *mut _object,
    kwds: *mut _object,
) -> i32 {
    let mut info: *mut _object = core::ptr::null_mut::<_object>();
    let lenargs: i64 = PyTuple_GET_SIZE!(args);
    if BaseException_init(op, args, kwds) == 1i32.wrapping_neg() {
        return 1i32.wrapping_neg();
    }
    let self_: *mut __SlateRecord_pyerrors_h_35 = PySyntaxErrorObject_CAST(op);
    if lenargs >= 1 {
        Py_XSETREF!((*self_).msg, Py_NewRef!(PyTuple_GET_ITEM!(args, 0i32)));
    }
    if lenargs == 2 {
        info = PyTuple_GET_ITEM!(args, 1i32);
        info = unsafe { PySequence_Tuple(info) };
        if info.is_null() {
            return 1i32.wrapping_neg();
        }
        let mut filename: *mut _object = unsafe { core::mem::zeroed() };
        let mut lineno: *mut _object = unsafe { core::mem::zeroed() };
        let mut offset: *mut _object = unsafe { core::mem::zeroed() };
        let mut text: *mut _object = unsafe { core::mem::zeroed() };
        let mut end_lineno: *mut _object = core::ptr::null_mut::<_object>();
        let mut end_offset: *mut _object = core::ptr::null_mut::<_object>();
        let mut metadata: *mut _object = core::ptr::null_mut::<_object>();
        if (unsafe {
            PyArg_ParseTuple(
                info,
                c"OOOO|OOO".as_ptr(),
                core::ptr::addr_of_mut!(filename),
                core::ptr::addr_of_mut!(lineno),
                core::ptr::addr_of_mut!(offset),
                core::ptr::addr_of_mut!(text),
                core::ptr::addr_of_mut!(end_lineno),
                core::ptr::addr_of_mut!(end_offset),
                core::ptr::addr_of_mut!(metadata),
            )
        }) == 0
        {
            Py_DECREF!(info);
            return 1i32.wrapping_neg();
        }
        Py_XSETREF!((*self_).filename, Py_NewRef!(filename));
        Py_XSETREF!((*self_).lineno, Py_NewRef!(lineno));
        Py_XSETREF!((*self_).offset, Py_NewRef!(offset));
        Py_XSETREF!((*self_).text, Py_NewRef!(text));
        Py_XSETREF!((*self_).end_lineno, Py_XNewRef!(end_lineno));
        Py_XSETREF!((*self_).end_offset, Py_XNewRef!(end_offset));
        Py_XSETREF!((*self_).metadata, Py_XNewRef!(metadata));
        Py_DECREF!(info);
        if !unsafe { (*self_).end_lineno }.is_null() && unsafe { (*self_).end_offset }.is_null() {
            unsafe {
                PyErr_SetString(
                    PyExc_TypeError,
                    c"end_offset must be provided when end_lineno is provided".as_ptr(),
                )
            };
            return 1i32.wrapping_neg();
        }
    }
    0
}

#[unsafe(link_section = ".text.slate_distinct.exceptions.SyntaxError_clear")]
extern "C-unwind" fn SyntaxError_clear(op: *mut _object) -> i32 {
    let self_: *mut __SlateRecord_pyerrors_h_35 = PySyntaxErrorObject_CAST(op);
    Py_CLEAR!((*self_).msg);
    Py_CLEAR!((*self_).filename);
    Py_CLEAR!((*self_).lineno);
    Py_CLEAR!((*self_).offset);
    Py_CLEAR!((*self_).end_lineno);
    Py_CLEAR!((*self_).end_offset);
    Py_CLEAR!((*self_).text);
    Py_CLEAR!((*self_).print_file_and_line);
    Py_CLEAR!((*self_).metadata);
    BaseException_clear(op)
}

#[unsafe(link_section = ".text.slate_distinct.exceptions.SyntaxError_dealloc")]
extern "C-unwind" fn SyntaxError_dealloc(self_: *mut _object) {
    _PyObject_GC_UNTRACK!(self_);
    SyntaxError_clear(self_);
    unsafe { unsafe { (*Py_TYPE!(self_)).tp_free }.unwrap()(self_ as *mut ()) };
}

#[unsafe(link_section = ".text.slate_distinct.exceptions.SyntaxError_traverse")]
extern "C-unwind" fn SyntaxError_traverse(
    op: *mut _object,
    mut visit: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
    arg: *mut (),
) -> i32 {
    let self_: *mut __SlateRecord_pyerrors_h_35 = PySyntaxErrorObject_CAST(op);
    Py_VISIT!(unsafe { (*self_).msg }, visit, arg);
    Py_VISIT!(unsafe { (*self_).filename }, visit, arg);
    Py_VISIT!(unsafe { (*self_).lineno }, visit, arg);
    Py_VISIT!(unsafe { (*self_).offset }, visit, arg);
    Py_VISIT!(unsafe { (*self_).end_lineno }, visit, arg);
    Py_VISIT!(unsafe { (*self_).end_offset }, visit, arg);
    Py_VISIT!(unsafe { (*self_).text }, visit, arg);
    Py_VISIT!(unsafe { (*self_).print_file_and_line }, visit, arg);
    Py_VISIT!(unsafe { (*self_).metadata }, visit, arg);
    BaseException_traverse(op, visit, arg)
}

/// This is called "my_basename" instead of just "basename" to avoid name
/// conflicts with glibc; basename is already prototyped if _GNU_SOURCE is
/// defined, and Python does define that.
fn my_basename(name: *mut _object) -> *mut _object {
    let mut size: i64 = 0;
    let mut offset: i64 = 0;
    let kind: i32 = PyUnicode_KIND!(name);
    let data: *const () = PyUnicode_DATA!(name) as *const ();
    size = PyUnicode_GET_LENGTH!(name);
    for i in 0..size {
        if PyUnicode_READ!(kind, data, i) == 47 {
            offset = i.wrapping_add(1);
        }
    }
    if offset != 0 {
        return unsafe { PyUnicode_Substring(name, offset, size) };
    } else {
        return Py_NewRef!(name);
    }
}

#[unsafe(link_section = ".text.slate_distinct.exceptions.SyntaxError_str")]
extern "C-unwind" fn SyntaxError_str(op: *mut _object) -> *mut _object {
    let self_: *mut __SlateRecord_pyerrors_h_35 = PySyntaxErrorObject_CAST(op);
    let mut have_lineno: i32 = 0;
    let mut filename: *mut _object = unsafe { core::mem::zeroed() };
    let mut result: *mut _object = unsafe { core::mem::zeroed() };
    // Below, we always ignore overflow errors, just printing -1.
    // Still, we cannot allow an OverflowError to be raised, so
    // we need to call PyLong_AsLongAndOverflow.
    let mut overflow: i32 = 0;
    // XXX -- do all the additional formatting with filename and
    // lineno here
    let __v0: bool = !unsafe { (*self_).filename }.is_null()
        && PyUnicode_Check!(unsafe { (*self_).filename }) != 0;
    if __v0 {
        filename = my_basename(unsafe { (*self_).filename });
        if filename.is_null() {
            return core::ptr::null_mut::<_object>();
        }
    } else {
        filename = core::ptr::null_mut::<_object>();
    }
    let __v1: bool = !unsafe { (*self_).lineno }.is_null()
        && PyLong_CheckExact!(unsafe { (*self_).lineno }) != 0;
    have_lineno = __v1 as i32;
    if filename.is_null() && have_lineno == 0 {
        return unsafe {
            PyObject_Str(if !unsafe { (*self_).msg }.is_null() {
                (*self_).msg
            } else {
                core::ptr::addr_of_mut!(_Py_NoneStruct)
            })
        };
    }
    // Even if 'filename' can be an instance of a subclass of 'str',
    //
    // we only render its "true" content and do not use str(filename).
    if !filename.is_null() && have_lineno != 0 {
        result = unsafe {
            PyUnicode_FromFormat(
                c"%S (%U, line %ld)".as_ptr(),
                if !unsafe { (*self_).msg }.is_null() {
                    (*self_).msg
                } else {
                    core::ptr::addr_of_mut!(_Py_NoneStruct)
                },
                filename,
                PyLong_AsLongAndOverflow((*self_).lineno, core::ptr::addr_of_mut!(overflow)),
            )
        };
    } else {
        if !filename.is_null() {
            result = unsafe {
                PyUnicode_FromFormat(
                    c"%S (%U)".as_ptr(),
                    if !unsafe { (*self_).msg }.is_null() {
                        (*self_).msg
                    } else {
                        core::ptr::addr_of_mut!(_Py_NoneStruct)
                    },
                    filename,
                )
            };
        } else {
            result = unsafe {
                PyUnicode_FromFormat(
                    c"%S (line %ld)".as_ptr(),
                    if !unsafe { (*self_).msg }.is_null() {
                        (*self_).msg
                    } else {
                        core::ptr::addr_of_mut!(_Py_NoneStruct)
                    },
                    PyLong_AsLongAndOverflow((*self_).lineno, core::ptr::addr_of_mut!(overflow)),
                )
            };
        }
    }
    // only have_lineno
    Py_XDECREF!(filename);
    result
}

/// Sentinel
static mut SyntaxError_members: __SlateAlign16<[PyMemberDef; 10]> = __SlateAlign16([
    {
        let mut __t0: PyMemberDef = unsafe { core::mem::zeroed() };
        __t0.name = (b"msg\0".as_ptr() as *mut i8) as *const i8;
        __t0.r#type = 6i32;
        __t0.offset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_35, msg) as u64) as i64;
        __t0.flags = 0i32;
        __t0.doc = (b"exception msg\0".as_ptr() as *mut i8) as *const i8;
        __t0
    },
    {
        let mut __t1: PyMemberDef = unsafe { core::mem::zeroed() };
        __t1.name = (b"filename\0".as_ptr() as *mut i8) as *const i8;
        __t1.r#type = 6i32;
        __t1.offset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_35, filename) as u64) as i64;
        __t1.flags = 0i32;
        __t1.doc = (b"exception filename\0".as_ptr() as *mut i8) as *const i8;
        __t1
    },
    {
        let mut __t2: PyMemberDef = unsafe { core::mem::zeroed() };
        __t2.name = (b"lineno\0".as_ptr() as *mut i8) as *const i8;
        __t2.r#type = 6i32;
        __t2.offset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_35, lineno) as u64) as i64;
        __t2.flags = 0i32;
        __t2.doc = (b"exception lineno\0".as_ptr() as *mut i8) as *const i8;
        __t2
    },
    {
        let mut __t3: PyMemberDef = unsafe { core::mem::zeroed() };
        __t3.name = (b"offset\0".as_ptr() as *mut i8) as *const i8;
        __t3.r#type = 6i32;
        __t3.offset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_35, offset) as u64) as i64;
        __t3.flags = 0i32;
        __t3.doc = (b"exception offset\0".as_ptr() as *mut i8) as *const i8;
        __t3
    },
    {
        let mut __t4: PyMemberDef = unsafe { core::mem::zeroed() };
        __t4.name = (b"text\0".as_ptr() as *mut i8) as *const i8;
        __t4.r#type = 6i32;
        __t4.offset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_35, text) as u64) as i64;
        __t4.flags = 0i32;
        __t4.doc = (b"exception text\0".as_ptr() as *mut i8) as *const i8;
        __t4
    },
    {
        let mut __t5: PyMemberDef = unsafe { core::mem::zeroed() };
        __t5.name = (b"end_lineno\0".as_ptr() as *mut i8) as *const i8;
        __t5.r#type = 6i32;
        __t5.offset =
            (core::mem::offset_of!(__SlateRecord_pyerrors_h_35, end_lineno) as u64) as i64;
        __t5.flags = 0i32;
        __t5.doc = (b"exception end lineno\0".as_ptr() as *mut i8) as *const i8;
        __t5
    },
    {
        let mut __t6: PyMemberDef = unsafe { core::mem::zeroed() };
        __t6.name = (b"end_offset\0".as_ptr() as *mut i8) as *const i8;
        __t6.r#type = 6i32;
        __t6.offset =
            (core::mem::offset_of!(__SlateRecord_pyerrors_h_35, end_offset) as u64) as i64;
        __t6.flags = 0i32;
        __t6.doc = (b"exception end offset\0".as_ptr() as *mut i8) as *const i8;
        __t6
    },
    {
        let mut __t7: PyMemberDef = unsafe { core::mem::zeroed() };
        __t7.name = (b"print_file_and_line\0".as_ptr() as *mut i8) as *const i8;
        __t7.r#type = 6i32;
        __t7.offset =
            (core::mem::offset_of!(__SlateRecord_pyerrors_h_35, print_file_and_line) as u64) as i64;
        __t7.flags = 0i32;
        __t7.doc = (b"exception print_file_and_line\0".as_ptr() as *mut i8) as *const i8;
        __t7
    },
    {
        let mut __t8: PyMemberDef = unsafe { core::mem::zeroed() };
        __t8.name = (b"_metadata\0".as_ptr() as *mut i8) as *const i8;
        __t8.r#type = 6i32;
        __t8.offset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_35, metadata) as u64) as i64;
        __t8.flags = 0i32;
        __t8.doc = (b"exception private metadata\0".as_ptr() as *mut i8) as *const i8;
        __t8
    },
    {
        let mut __t9: PyMemberDef = unsafe { core::mem::zeroed() };
        __t9.name = core::ptr::null::<i8>();
        __t9
    },
]);

static mut _PyExc_SyntaxError: _typeobject = {
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
    __t0.tp_name = (b"SyntaxError\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pyerrors_h_35>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(SyntaxError_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = None;
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = Some(SyntaxError_str);
    __t0.tp_getattro = None;
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 17408u64;
    __t0.tp_doc = (b"Invalid syntax.\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_traverse = Some(SyntaxError_traverse);
    __t0.tp_clear = Some(SyntaxError_clear);
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = core::ptr::null_mut::<PyMethodDef>();
    __t0.tp_members = unsafe { core::ptr::addr_of_mut!(SyntaxError_members.0) as *mut PyMemberDef };
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(_PyExc_Exception) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_35, dict) as u64) as i64;
    __t0.tp_init = Some(SyntaxError_init);
    __t0.tp_alloc = None;
    __t0.tp_new = None;
    __t0
};

#[unsafe(no_mangle)]
static mut PyExc_SyntaxError: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_SyntaxError) }) as *mut _object;

// IndentationError extends SyntaxError
static mut _PyExc_IndentationError: _typeobject = {
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
    __t0.tp_name = (b"IndentationError\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pyerrors_h_35>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(SyntaxError_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = None;
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = None;
    __t0.tp_getattro = None;
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 17408u64;
    __t0.tp_doc = (b"Improper indentation.\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_traverse = Some(SyntaxError_traverse);
    __t0.tp_clear = Some(SyntaxError_clear);
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = core::ptr::null_mut::<PyMethodDef>();
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(_PyExc_SyntaxError) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_35, dict) as u64) as i64;
    __t0.tp_init = Some(SyntaxError_init);
    __t0.tp_alloc = None;
    __t0.tp_new = None;
    __t0
};

#[unsafe(no_mangle)]
static mut PyExc_IndentationError: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_IndentationError) }) as *mut _object;

// TabError extends IndentationError
static mut _PyExc_TabError: _typeobject = {
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
    __t0.tp_name = (b"TabError\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pyerrors_h_35>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(SyntaxError_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = None;
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = None;
    __t0.tp_getattro = None;
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 17408u64;
    __t0.tp_doc = (b"Improper mixture of spaces and tabs.\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_traverse = Some(SyntaxError_traverse);
    __t0.tp_clear = Some(SyntaxError_clear);
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = core::ptr::null_mut::<PyMethodDef>();
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(_PyExc_IndentationError) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_35, dict) as u64) as i64;
    __t0.tp_init = Some(SyntaxError_init);
    __t0.tp_alloc = None;
    __t0.tp_new = None;
    __t0
};

#[unsafe(no_mangle)]
static mut PyExc_TabError: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_TabError) }) as *mut _object;

// IncompleteInputError extends SyntaxError
// LookupError extends Exception
static mut _PyExc_LookupError: _typeobject = {
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
    __t0.tp_name = (b"LookupError\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pyerrors_h_15>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(BaseException_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = None;
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = None;
    __t0.tp_getattro = None;
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 17408u64;
    __t0.tp_doc = (b"Base class for lookup errors.\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_traverse = Some(BaseException_traverse);
    __t0.tp_clear = Some(BaseException_clear);
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = core::ptr::null_mut::<PyMethodDef>();
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(_PyExc_Exception) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_15, dict) as u64) as i64;
    __t0.tp_init = Some(BaseException_init);
    __t0.tp_alloc = None;
    __t0.tp_new = Some(BaseException_new);
    __t0
};

#[unsafe(no_mangle)]
static mut PyExc_LookupError: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_LookupError) }) as *mut _object;

// IndexError extends LookupError
static mut _PyExc_IndexError: _typeobject = {
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
    __t0.tp_name = (b"IndexError\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pyerrors_h_15>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(BaseException_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = None;
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = None;
    __t0.tp_getattro = None;
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 17408u64;
    __t0.tp_doc = (b"Sequence index out of range.\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_traverse = Some(BaseException_traverse);
    __t0.tp_clear = Some(BaseException_clear);
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = core::ptr::null_mut::<PyMethodDef>();
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(_PyExc_LookupError) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_15, dict) as u64) as i64;
    __t0.tp_init = Some(BaseException_init);
    __t0.tp_alloc = None;
    __t0.tp_new = Some(BaseException_new);
    __t0
};

#[unsafe(no_mangle)]
static mut PyExc_IndexError: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_IndexError) }) as *mut _object;

// KeyError extends LookupError
#[unsafe(link_section = ".text.slate_distinct.exceptions.KeyError_str")]
extern "C-unwind" fn KeyError_str(op: *mut _object) -> *mut _object {
    // If args is a tuple of exactly one item, apply repr to args[0].
    // This is done so that e.g. the exception raised by {}[''] prints
    //   KeyError: ''
    // rather than the confusing
    //   KeyError
    // alone.  The downside is that if KeyError is raised with an explanatory
    // string, that string will be displayed in quotes.  Too bad.
    // If args is anything else, use the default BaseException__str__().
    let self_: *mut __SlateRecord_pyerrors_h_15 = PyBaseExceptionObject_CAST(op);
    if PyTuple_GET_SIZE!(unsafe { (*self_).args }) == 1 {
        return unsafe { PyObject_Repr(PyTuple_GET_ITEM!(unsafe { (*self_).args }, 0i32)) };
    }
    BaseException_str(op)
}

static mut _PyExc_KeyError: _typeobject = {
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
    __t0.tp_name = (b"KeyError\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pyerrors_h_15>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(BaseException_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = None;
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = Some(KeyError_str);
    __t0.tp_getattro = None;
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 17408u64;
    __t0.tp_doc = (b"Mapping key not found.\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_traverse = Some(BaseException_traverse);
    __t0.tp_clear = Some(BaseException_clear);
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = core::ptr::null_mut::<PyMethodDef>();
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(_PyExc_LookupError) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_15, dict) as u64) as i64;
    __t0.tp_init = Some(BaseException_init);
    __t0.tp_alloc = None;
    __t0.tp_new = None;
    __t0
};

#[unsafe(no_mangle)]
static mut PyExc_KeyError: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_KeyError) }) as *mut _object;

// ValueError extends Exception
static mut _PyExc_ValueError: _typeobject = {
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
    __t0.tp_name = (b"ValueError\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pyerrors_h_15>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(BaseException_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = None;
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = None;
    __t0.tp_getattro = None;
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 17408u64;
    __t0.tp_doc =
        (b"Inappropriate argument value (of correct type).\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_traverse = Some(BaseException_traverse);
    __t0.tp_clear = Some(BaseException_clear);
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = core::ptr::null_mut::<PyMethodDef>();
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(_PyExc_Exception) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_15, dict) as u64) as i64;
    __t0.tp_init = Some(BaseException_init);
    __t0.tp_alloc = None;
    __t0.tp_new = Some(BaseException_new);
    __t0
};

#[unsafe(no_mangle)]
static mut PyExc_ValueError: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_ValueError) }) as *mut _object;

// UnicodeError extends ValueError
static mut _PyExc_UnicodeError: _typeobject = {
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
    __t0.tp_name = (b"UnicodeError\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pyerrors_h_15>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(BaseException_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = None;
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = None;
    __t0.tp_getattro = None;
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 17408u64;
    __t0.tp_doc = (b"Unicode related error.\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_traverse = Some(BaseException_traverse);
    __t0.tp_clear = Some(BaseException_clear);
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = core::ptr::null_mut::<PyMethodDef>();
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(_PyExc_ValueError) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_15, dict) as u64) as i64;
    __t0.tp_init = Some(BaseException_init);
    __t0.tp_alloc = None;
    __t0.tp_new = Some(BaseException_new);
    __t0
};

#[unsafe(no_mangle)]
static mut PyExc_UnicodeError: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_UnicodeError) }) as *mut _object;

/// Check the validity of 'attr' as a unicode or bytes object depending
/// on 'as_bytes'.
///
/// The 'name' is the attribute name and is only used for error reporting.
///
/// On success, this returns 0.
/// On failure, this sets a TypeError and returns -1.
fn check_unicode_error_attribute(attr: *mut _object, name: *const i8, as_bytes: i32) -> i32 {
    if attr.is_null() {
        unsafe {
            PyErr_Format(
                PyExc_TypeError,
                c"UnicodeError '%s' attribute is not set".as_ptr(),
                name,
            )
        };
        return 1i32.wrapping_neg();
    }
    let __v0: i32 = if as_bytes != 0 {
        PyBytes_Check!(attr)
    } else {
        PyUnicode_Check!(attr)
    };
    if __v0 == 0 {
        unsafe {
            PyErr_Format(
                PyExc_TypeError,
                c"UnicodeError '%s' attribute must be a %s".as_ptr(),
                name,
                if as_bytes != 0 {
                    c"bytes".as_ptr() as *mut i8
                } else {
                    c"string".as_ptr() as *mut i8
                },
            )
        };
        return 1i32.wrapping_neg();
    }
    0
}

/// Check the validity of 'attr' as a unicode or bytes object depending
/// on 'as_bytes' and return a new reference on it if it is the case.
///
/// The 'name' is the attribute name and is only used for error reporting.
///
/// On success, this returns a strong reference on 'attr'.
/// On failure, this sets a TypeError and returns NULL.
fn as_unicode_error_attribute(attr: *mut _object, name: *const i8, as_bytes: i32) -> *mut _object {
    let rc: i32 = check_unicode_error_attribute(attr, name, as_bytes);
    let __v0: *mut _object = if rc < 0 {
        core::ptr::null_mut::<_object>()
    } else {
        Py_NewRef!(attr)
    };
    __v0
}

// class names to use when reporting errors
/// Check that 'self' is a UnicodeError object.
///
/// On success, this returns 0.
/// On failure, this sets a TypeError exception and returns -1.
///
/// The 'expect_type' is the name of the expected type, which is
/// only used for error reporting.
///
/// As an implementation detail, the `PyUnicode*Error_*` functions
/// currently allow *any* subclass of UnicodeError as 'self'.
///
/// Use one of the `Py_UNICODE_*_ERROR_NAME` macros to avoid typos.
fn check_unicode_error_type(self_: *mut _object, expect_type: *const i8) -> i32 {
    if PyUnicodeError_Check!(self_) == 0 {
        unsafe {
            PyErr_Format(
                PyExc_TypeError,
                c"expecting a %s object, got %T".as_ptr(),
                expect_type,
                self_,
            )
        };
        return 1i32.wrapping_neg();
    }
    0
}

// --- PyUnicodeEncodeObject: internal helpers --------------------------------
//
//
//
// In the helpers below, the caller is responsible to ensure that 'self'
//
// is a PyUnicodeErrorObject, although this is verified on DEBUG builds
//
// through PyUnicodeError_CAST().
/// Return the underlying (str) 'encoding' attribute of a UnicodeError object.
fn unicode_error_get_encoding_impl(self_: *mut _object) -> *mut _object {
    let exc: *mut __SlateRecord_pyerrors_h_52 = PyUnicodeError_CAST!(self_);
    as_unicode_error_attribute(unsafe { (*exc).encoding }, c"encoding".as_ptr(), 0i32)
}

/// Return the underlying 'object' attribute of a UnicodeError object
/// as a bytes or a string instance, depending on the 'as_bytes' flag.
fn unicode_error_get_object_impl(self_: *mut _object, as_bytes: i32) -> *mut _object {
    let exc: *mut __SlateRecord_pyerrors_h_52 = PyUnicodeError_CAST!(self_);
    as_unicode_error_attribute(unsafe { (*exc).object }, c"object".as_ptr(), as_bytes)
}

/// Return the underlying (str) 'reason' attribute of a UnicodeError object.
fn unicode_error_get_reason_impl(self_: *mut _object) -> *mut _object {
    let exc: *mut __SlateRecord_pyerrors_h_52 = PyUnicodeError_CAST!(self_);
    as_unicode_error_attribute(unsafe { (*exc).reason }, c"reason".as_ptr(), 0i32)
}

/// Set the underlying (str) 'reason' attribute of a UnicodeError object.
///
/// Return 0 on success and -1 on failure.
fn unicode_error_set_reason_impl(self_: *mut _object, reason: *const i8) -> i32 {
    let value: *mut _object = unsafe { PyUnicode_FromString(reason) };
    if value.is_null() {
        return 1i32.wrapping_neg();
    }
    let exc: *mut __SlateRecord_pyerrors_h_52 = PyUnicodeError_CAST!(self_);
    Py_XSETREF!((*exc).reason, value);
    0
}

/// Set the 'start' attribute of a UnicodeError object.
///
/// Return 0 on success and -1 on failure.
fn unicode_error_set_start_impl(self_: *mut _object, start: i64) -> i32 {
    let exc: *mut __SlateRecord_pyerrors_h_52 = PyUnicodeError_CAST!(self_);
    unsafe {
        (*exc).start = start;
    }
    0
}

/// Set the 'end' attribute of a UnicodeError object.
///
/// Return 0 on success and -1 on failure.
fn unicode_error_set_end_impl(self_: *mut _object, end: i64) -> i32 {
    let exc: *mut __SlateRecord_pyerrors_h_52 = PyUnicodeError_CAST!(self_);
    unsafe {
        (*exc).end = end;
    }
    0
}

// --- PyUnicodeEncodeObject: internal getters --------------------------------
/// Adjust the (inclusive) 'start' value of a UnicodeError object.
///
/// The 'start' can be negative or not, but when adjusting the value,
/// we clip it in [0, max(0, objlen - 1)] and do not interpret it as
/// a relative offset.
///
/// This function always succeeds.
fn unicode_error_adjust_start(mut start: i64, objlen: i64) -> i64 {
    if start < 0 {
        start = 0;
    }
    if start >= objlen {
        start = if objlen == 0 {
            0
        } else {
            objlen.wrapping_sub(1)
        };
    }
    start
}

/// Assert some properties of the adjusted 'start' value.
/// Adjust the (exclusive) 'end' value of a UnicodeError object.
///
/// The 'end' can be negative or not, but when adjusting the value,
/// we clip it in [min(1, objlen), max(min(1, objlen), objlen)] and
/// do not interpret it as a relative offset.
///
/// This function always succeeds.
fn unicode_error_adjust_end(mut end: i64, objlen: i64) -> i64 {
    if end < 1 {
        end = 1;
    }
    if end > objlen {
        end = objlen;
    }
    end
}

/// Assert some properties of the adjusted 'end' value.
/// Adjust the length of the range described by a UnicodeError object.
///
/// The 'start' and 'end' arguments must have been obtained by
/// unicode_error_adjust_start() and unicode_error_adjust_end().
///
/// The result is clipped in [0, objlen]. By construction, it
/// will always be smaller than 'objlen' as 'start' and 'end'
/// are smaller than 'objlen'.
fn unicode_error_adjust_len(start: i64, end: i64, objlen: i64) -> i64 {
    let ranlen: i64 = end.wrapping_sub(start);
    if ranlen < 0 { 0 } else { ranlen }
}

/// Assert some properties of the adjusted range 'len' value.
/// Get various common parameters of a UnicodeError object.
///
/// The caller is responsible to ensure that 'self' is a PyUnicodeErrorObject,
/// although this condition is verified by this function on DEBUG builds.
///
/// Return 0 on success and -1 on failure.
///
/// Output parameters:
///
///     obj          A strong reference to the 'object' attribute.
///     objlen       The 'object' length.
///     start        The clipped 'start' attribute.
///     end          The clipped 'end' attribute.
///     slen         The length of the slice described by the clipped 'start'
///                  and 'end' values. It always lies in [0, objlen].
///
/// An output parameter can be NULL to indicate that
/// the corresponding value does not need to be stored.
///
/// Input parameter:
///
///     as_bytes     If true, the error's 'object' attribute must be a `bytes`,
///                  i.e. 'self' is a `UnicodeDecodeError` instance. Otherwise,
///                  the 'object' attribute must be a string.
///
///                  A TypeError is raised if the 'object' type is incompatible.
#[unsafe(no_mangle)]
extern "C-unwind" fn _PyUnicodeError_GetParams(
    self_: *mut _object,
    obj: *mut *mut _object,
    objlen: *mut i64,
    start: *mut i64,
    end: *mut i64,
    slen: *mut i64,
    as_bytes: i32,
) -> i32 {
    let exc: *mut __SlateRecord_pyerrors_h_52 = PyUnicodeError_CAST!(self_);
    let r: *mut _object =
        as_unicode_error_attribute(unsafe { (*exc).object }, c"object".as_ptr(), as_bytes);
    if r.is_null() {
        return 1i32.wrapping_neg();
    }
    let mut n: i64 = 0;
    let __v0: i64 = if as_bytes != 0 {
        PyBytes_GET_SIZE!(r)
    } else {
        PyUnicode_GET_LENGTH!(r)
    };
    n = __v0;
    if !objlen.is_null() {
        unsafe {
            *objlen = n;
        }
    }
    let mut start_value: i64 = 1i32.wrapping_neg() as i64;
    if !start.is_null() || !slen.is_null() {
        start_value = unicode_error_adjust_start(unsafe { (*exc).start }, n);
    }
    if !start.is_null() {
        unsafe {
            *start = start_value;
        }
    }
    let mut end_value: i64 = 1i32.wrapping_neg() as i64;
    if !end.is_null() || !slen.is_null() {
        end_value = unicode_error_adjust_end(unsafe { (*exc).end }, n);
    }
    if !end.is_null() {
        unsafe {
            *end = end_value;
        }
    }
    if !slen.is_null() {
        unsafe {
            *slen = unicode_error_adjust_len(start_value, end_value, n);
        }
    }
    if !obj.is_null() {
        unsafe {
            *obj = r;
        }
    } else {
        Py_DECREF!(r);
    }
    0
}

// --- PyUnicodeEncodeObject: 'encoding' getters ------------------------------
//
// Note: PyUnicodeTranslateError does not have an 'encoding' attribute.
#[unsafe(no_mangle)]
extern "C-unwind" fn PyUnicodeEncodeError_GetEncoding(self_: *mut _object) -> *mut _object {
    let rc: i32 = check_unicode_error_type(self_, c"UnicodeEncodeError".as_ptr());
    let __v0: *mut _object = if rc < 0 {
        core::ptr::null_mut::<_object>()
    } else {
        unicode_error_get_encoding_impl(self_)
    };
    __v0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyUnicodeDecodeError_GetEncoding(self_: *mut _object) -> *mut _object {
    let rc: i32 = check_unicode_error_type(self_, c"UnicodeDecodeError".as_ptr());
    let __v0: *mut _object = if rc < 0 {
        core::ptr::null_mut::<_object>()
    } else {
        unicode_error_get_encoding_impl(self_)
    };
    __v0
}

// --- PyUnicodeEncodeObject: 'object' getters --------------------------------
#[unsafe(no_mangle)]
extern "C-unwind" fn PyUnicodeEncodeError_GetObject(self_: *mut _object) -> *mut _object {
    let rc: i32 = check_unicode_error_type(self_, c"UnicodeEncodeError".as_ptr());
    let __v0: *mut _object = if rc < 0 {
        core::ptr::null_mut::<_object>()
    } else {
        unicode_error_get_object_impl(self_, 0i32)
    };
    __v0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyUnicodeDecodeError_GetObject(self_: *mut _object) -> *mut _object {
    let rc: i32 = check_unicode_error_type(self_, c"UnicodeDecodeError".as_ptr());
    let __v0: *mut _object = if rc < 0 {
        core::ptr::null_mut::<_object>()
    } else {
        unicode_error_get_object_impl(self_, 1i32)
    };
    __v0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyUnicodeTranslateError_GetObject(self_: *mut _object) -> *mut _object {
    let rc: i32 = check_unicode_error_type(self_, c"UnicodeTranslateError".as_ptr());
    let __v0: *mut _object = if rc < 0 {
        core::ptr::null_mut::<_object>()
    } else {
        unicode_error_get_object_impl(self_, 0i32)
    };
    __v0
}

// --- PyUnicodeEncodeObject: 'start' getters ---------------------------------
/// Specialization of _PyUnicodeError_GetParams() for the 'start' attribute.
///
/// The caller is responsible to ensure that 'self' is a PyUnicodeErrorObject,
/// although this condition is verified by this function on DEBUG builds.
fn unicode_error_get_start_impl(self_: *mut _object, start: *mut i64, as_bytes: i32) -> i32 {
    _PyUnicodeError_GetParams(
        self_,
        core::ptr::null_mut::<*mut _object>(),
        core::ptr::null_mut::<i64>(),
        start,
        core::ptr::null_mut::<i64>(),
        core::ptr::null_mut::<i64>(),
        as_bytes,
    )
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyUnicodeEncodeError_GetStart(self_: *mut _object, start: *mut i64) -> i32 {
    let rc: i32 = check_unicode_error_type(self_, c"UnicodeEncodeError".as_ptr());
    let __v0: i32 = if rc < 0 {
        1i32.wrapping_neg()
    } else {
        unicode_error_get_start_impl(self_, start, 0i32)
    };
    __v0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyUnicodeDecodeError_GetStart(self_: *mut _object, start: *mut i64) -> i32 {
    let rc: i32 = check_unicode_error_type(self_, c"UnicodeDecodeError".as_ptr());
    let __v0: i32 = if rc < 0 {
        1i32.wrapping_neg()
    } else {
        unicode_error_get_start_impl(self_, start, 1i32)
    };
    __v0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyUnicodeTranslateError_GetStart(self_: *mut _object, start: *mut i64) -> i32 {
    let rc: i32 = check_unicode_error_type(self_, c"UnicodeTranslateError".as_ptr());
    let __v0: i32 = if rc < 0 {
        1i32.wrapping_neg()
    } else {
        unicode_error_get_start_impl(self_, start, 0i32)
    };
    __v0
}

// --- PyUnicodeEncodeObject: 'start' setters ---------------------------------
#[unsafe(no_mangle)]
extern "C-unwind" fn PyUnicodeEncodeError_SetStart(self_: *mut _object, start: i64) -> i32 {
    let rc: i32 = check_unicode_error_type(self_, c"UnicodeEncodeError".as_ptr());
    let __v0: i32 = if rc < 0 {
        1i32.wrapping_neg()
    } else {
        unicode_error_set_start_impl(self_, start)
    };
    __v0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyUnicodeDecodeError_SetStart(self_: *mut _object, start: i64) -> i32 {
    let rc: i32 = check_unicode_error_type(self_, c"UnicodeDecodeError".as_ptr());
    let __v0: i32 = if rc < 0 {
        1i32.wrapping_neg()
    } else {
        unicode_error_set_start_impl(self_, start)
    };
    __v0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyUnicodeTranslateError_SetStart(self_: *mut _object, start: i64) -> i32 {
    let rc: i32 = check_unicode_error_type(self_, c"UnicodeTranslateError".as_ptr());
    let __v0: i32 = if rc < 0 {
        1i32.wrapping_neg()
    } else {
        unicode_error_set_start_impl(self_, start)
    };
    __v0
}

// --- PyUnicodeEncodeObject: 'end' getters -----------------------------------
/// Specialization of _PyUnicodeError_GetParams() for the 'end' attribute.
///
/// The caller is responsible to ensure that 'self' is a PyUnicodeErrorObject,
/// although this condition is verified by this function on DEBUG builds.
fn unicode_error_get_end_impl(self_: *mut _object, end: *mut i64, as_bytes: i32) -> i32 {
    _PyUnicodeError_GetParams(
        self_,
        core::ptr::null_mut::<*mut _object>(),
        core::ptr::null_mut::<i64>(),
        core::ptr::null_mut::<i64>(),
        end,
        core::ptr::null_mut::<i64>(),
        as_bytes,
    )
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyUnicodeEncodeError_GetEnd(self_: *mut _object, end: *mut i64) -> i32 {
    let rc: i32 = check_unicode_error_type(self_, c"UnicodeEncodeError".as_ptr());
    let __v0: i32 = if rc < 0 {
        1i32.wrapping_neg()
    } else {
        unicode_error_get_end_impl(self_, end, 0i32)
    };
    __v0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyUnicodeDecodeError_GetEnd(self_: *mut _object, end: *mut i64) -> i32 {
    let rc: i32 = check_unicode_error_type(self_, c"UnicodeDecodeError".as_ptr());
    let __v0: i32 = if rc < 0 {
        1i32.wrapping_neg()
    } else {
        unicode_error_get_end_impl(self_, end, 1i32)
    };
    __v0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyUnicodeTranslateError_GetEnd(self_: *mut _object, end: *mut i64) -> i32 {
    let rc: i32 = check_unicode_error_type(self_, c"UnicodeTranslateError".as_ptr());
    let __v0: i32 = if rc < 0 {
        1i32.wrapping_neg()
    } else {
        unicode_error_get_end_impl(self_, end, 0i32)
    };
    __v0
}

// --- PyUnicodeEncodeObject: 'end' setters -----------------------------------
#[unsafe(no_mangle)]
extern "C-unwind" fn PyUnicodeEncodeError_SetEnd(self_: *mut _object, end: i64) -> i32 {
    let rc: i32 = check_unicode_error_type(self_, c"UnicodeEncodeError".as_ptr());
    let __v0: i32 = if rc < 0 {
        1i32.wrapping_neg()
    } else {
        unicode_error_set_end_impl(self_, end)
    };
    __v0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyUnicodeDecodeError_SetEnd(self_: *mut _object, end: i64) -> i32 {
    let rc: i32 = check_unicode_error_type(self_, c"UnicodeDecodeError".as_ptr());
    let __v0: i32 = if rc < 0 {
        1i32.wrapping_neg()
    } else {
        unicode_error_set_end_impl(self_, end)
    };
    __v0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyUnicodeTranslateError_SetEnd(self_: *mut _object, end: i64) -> i32 {
    let rc: i32 = check_unicode_error_type(self_, c"UnicodeTranslateError".as_ptr());
    let __v0: i32 = if rc < 0 {
        1i32.wrapping_neg()
    } else {
        unicode_error_set_end_impl(self_, end)
    };
    __v0
}

// --- PyUnicodeEncodeObject: 'reason' getters --------------------------------
#[unsafe(no_mangle)]
extern "C-unwind" fn PyUnicodeEncodeError_GetReason(self_: *mut _object) -> *mut _object {
    let rc: i32 = check_unicode_error_type(self_, c"UnicodeEncodeError".as_ptr());
    let __v0: *mut _object = if rc < 0 {
        core::ptr::null_mut::<_object>()
    } else {
        unicode_error_get_reason_impl(self_)
    };
    __v0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyUnicodeDecodeError_GetReason(self_: *mut _object) -> *mut _object {
    let rc: i32 = check_unicode_error_type(self_, c"UnicodeDecodeError".as_ptr());
    let __v0: *mut _object = if rc < 0 {
        core::ptr::null_mut::<_object>()
    } else {
        unicode_error_get_reason_impl(self_)
    };
    __v0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyUnicodeTranslateError_GetReason(self_: *mut _object) -> *mut _object {
    let rc: i32 = check_unicode_error_type(self_, c"UnicodeTranslateError".as_ptr());
    let __v0: *mut _object = if rc < 0 {
        core::ptr::null_mut::<_object>()
    } else {
        unicode_error_get_reason_impl(self_)
    };
    __v0
}

// --- PyUnicodeEncodeObject: 'reason' setters --------------------------------
#[unsafe(no_mangle)]
extern "C-unwind" fn PyUnicodeEncodeError_SetReason(self_: *mut _object, reason: *const i8) -> i32 {
    let rc: i32 = check_unicode_error_type(self_, c"UnicodeEncodeError".as_ptr());
    let __v0: i32 = if rc < 0 {
        1i32.wrapping_neg()
    } else {
        unicode_error_set_reason_impl(self_, reason)
    };
    __v0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyUnicodeDecodeError_SetReason(self_: *mut _object, reason: *const i8) -> i32 {
    let rc: i32 = check_unicode_error_type(self_, c"UnicodeDecodeError".as_ptr());
    let __v0: i32 = if rc < 0 {
        1i32.wrapping_neg()
    } else {
        unicode_error_set_reason_impl(self_, reason)
    };
    __v0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyUnicodeTranslateError_SetReason(
    self_: *mut _object,
    reason: *const i8,
) -> i32 {
    let rc: i32 = check_unicode_error_type(self_, c"UnicodeTranslateError".as_ptr());
    let __v0: i32 = if rc < 0 {
        1i32.wrapping_neg()
    } else {
        unicode_error_set_reason_impl(self_, reason)
    };
    __v0
}

#[unsafe(link_section = ".text.slate_distinct.exceptions.UnicodeError_clear")]
extern "C-unwind" fn UnicodeError_clear(self_: *mut _object) -> i32 {
    let exc: *mut __SlateRecord_pyerrors_h_52 = PyUnicodeErrorObject_CAST!(self_);
    Py_CLEAR!((*exc).encoding);
    Py_CLEAR!((*exc).object);
    Py_CLEAR!((*exc).reason);
    BaseException_clear(self_)
}

#[unsafe(link_section = ".text.slate_distinct.exceptions.UnicodeError_dealloc")]
extern "C-unwind" fn UnicodeError_dealloc(self_: *mut _object) {
    let r#type: *mut _typeobject = Py_TYPE!(self_);
    _PyObject_GC_UNTRACK!(self_);
    UnicodeError_clear(self_);
    unsafe { unsafe { (*r#type).tp_free }.unwrap()(self_ as *mut ()) };
}

#[unsafe(link_section = ".text.slate_distinct.exceptions.UnicodeError_traverse")]
extern "C-unwind" fn UnicodeError_traverse(
    self_: *mut _object,
    mut visit: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
    arg: *mut (),
) -> i32 {
    let exc: *mut __SlateRecord_pyerrors_h_52 = PyUnicodeErrorObject_CAST!(self_);
    Py_VISIT!(unsafe { (*exc).encoding }, visit, arg);
    Py_VISIT!(unsafe { (*exc).object }, visit, arg);
    Py_VISIT!(unsafe { (*exc).reason }, visit, arg);
    BaseException_traverse(self_, visit, arg)
}

/// Sentinel
static mut UnicodeError_members: __SlateAlign16<[PyMemberDef; 6]> = __SlateAlign16([
    {
        let mut __t0: PyMemberDef = unsafe { core::mem::zeroed() };
        __t0.name = (b"encoding\0".as_ptr() as *mut i8) as *const i8;
        __t0.r#type = 6i32;
        __t0.offset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_52, encoding) as u64) as i64;
        __t0.flags = 0i32;
        __t0.doc = (b"exception encoding\0".as_ptr() as *mut i8) as *const i8;
        __t0
    },
    {
        let mut __t1: PyMemberDef = unsafe { core::mem::zeroed() };
        __t1.name = (b"object\0".as_ptr() as *mut i8) as *const i8;
        __t1.r#type = 6i32;
        __t1.offset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_52, object) as u64) as i64;
        __t1.flags = 0i32;
        __t1.doc = (b"exception object\0".as_ptr() as *mut i8) as *const i8;
        __t1
    },
    {
        let mut __t2: PyMemberDef = unsafe { core::mem::zeroed() };
        __t2.name = (b"start\0".as_ptr() as *mut i8) as *const i8;
        __t2.r#type = 19i32;
        __t2.offset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_52, start) as u64) as i64;
        __t2.flags = 0i32;
        __t2.doc = (b"exception start\0".as_ptr() as *mut i8) as *const i8;
        __t2
    },
    {
        let mut __t3: PyMemberDef = unsafe { core::mem::zeroed() };
        __t3.name = (b"end\0".as_ptr() as *mut i8) as *const i8;
        __t3.r#type = 19i32;
        __t3.offset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_52, end) as u64) as i64;
        __t3.flags = 0i32;
        __t3.doc = (b"exception end\0".as_ptr() as *mut i8) as *const i8;
        __t3
    },
    {
        let mut __t4: PyMemberDef = unsafe { core::mem::zeroed() };
        __t4.name = (b"reason\0".as_ptr() as *mut i8) as *const i8;
        __t4.r#type = 6i32;
        __t4.offset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_52, reason) as u64) as i64;
        __t4.flags = 0i32;
        __t4.doc = (b"exception reason\0".as_ptr() as *mut i8) as *const i8;
        __t4
    },
    {
        let mut __t5: PyMemberDef = unsafe { core::mem::zeroed() };
        __t5.name = core::ptr::null::<i8>();
        __t5
    },
]);

// UnicodeEncodeError extends UnicodeError
#[unsafe(link_section = ".text.slate_distinct.exceptions.UnicodeEncodeError_init")]
extern "C-unwind" fn UnicodeEncodeError_init(
    self_: *mut _object,
    args: *mut _object,
    kwds: *mut _object,
) -> i32 {
    if BaseException_init(self_, args, kwds) == 1i32.wrapping_neg() {
        return 1i32.wrapping_neg();
    }
    let mut encoding: *mut _object = core::ptr::null_mut::<_object>();
    let mut object: *mut _object = core::ptr::null_mut::<_object>();
    let mut reason: *mut _object = core::ptr::null_mut::<_object>(); // borrowed
    let mut start: i64 = 1i32.wrapping_neg() as i64;
    let mut end: i64 = 1i32.wrapping_neg() as i64;
    if (unsafe {
        PyArg_ParseTuple(
            args,
            c"UUnnU".as_ptr(),
            core::ptr::addr_of_mut!(encoding),
            core::ptr::addr_of_mut!(object),
            core::ptr::addr_of_mut!(start),
            core::ptr::addr_of_mut!(end),
            core::ptr::addr_of_mut!(reason),
        )
    }) == 0
    {
        return 1i32.wrapping_neg();
    }
    let exc: *mut __SlateRecord_pyerrors_h_52 = PyUnicodeErrorObject_CAST!(self_);
    Py_XSETREF!((*exc).encoding, Py_NewRef!(encoding));
    Py_XSETREF!((*exc).object, Py_NewRef!(object));
    unsafe {
        (*exc).start = start;
    }
    unsafe {
        (*exc).end = end;
    }
    Py_XSETREF!((*exc).reason, Py_NewRef!(reason));
    0
}

#[unsafe(link_section = ".text.slate_distinct.exceptions.UnicodeEncodeError_str")]
extern "C-unwind" fn UnicodeEncodeError_str(self_: *mut _object) -> *mut _object {
    let mut fmt: *const i8 = unsafe { core::mem::zeroed() };
    let mut badchar: u32 = unsafe { core::mem::zeroed() };
    let mut end: i64 = unsafe { core::mem::zeroed() };
    let mut start: i64 = unsafe { core::mem::zeroed() };
    let mut encoding_str: *mut _object = unsafe { core::mem::zeroed() };
    let mut reason_str: *mut _object = unsafe { core::mem::zeroed() };
    let mut result: *mut _object = unsafe { core::mem::zeroed() };
    let mut exc: *mut __SlateRecord_pyerrors_h_52 = unsafe { core::mem::zeroed() };
    unsafe {
        exc = PyUnicodeErrorObject_CAST!(self_);
        result = core::ptr::null_mut::<_object>();
        reason_str = core::ptr::null_mut::<_object>();
        encoding_str = core::ptr::null_mut::<_object>();
        if unsafe { (*exc).object }.is_null() {
            // Not properly initialized.
            return Py_GetConstant(7u32);
        } else {
            // Get reason and encoding as strings, which they might not be if
            // they've been modified after we were constructed.
            reason_str = PyObject_Str((*exc).reason);
            if !reason_str.is_null() {
                encoding_str = PyObject_Str((*exc).encoding);
                if !encoding_str.is_null() {
                    // calls to PyObject_Str(...) above might mutate 'exc->object'
                    if check_unicode_error_attribute((*exc).object, c"object".as_ptr(), 0i32) >= 0 {
                        let len: i64 = PyUnicode_GET_LENGTH!(unsafe { (*exc).object });
                        start = (*exc).start;
                        end = (*exc).end;
                        if start >= 0
                            && start < len
                            && (end >= 0 && end <= len)
                            && end == start.wrapping_add(1)
                        {
                            badchar = PyUnicode_ReadChar((*exc).object, start);
                            if badchar <= 255 {
                                fmt = c"'%U' codec can't encode character '\\x%02x' in position %zd: %U".as_ptr();
                            } else {
                                if badchar <= 65535 {
                                    fmt = c"'%U' codec can't encode character '\\u%04x' in position %zd: %U".as_ptr();
                                } else {
                                    fmt = c"'%U' codec can't encode character '\\U%08x' in position %zd: %U".as_ptr();
                                }
                            }
                            result = PyUnicode_FromFormat(
                                fmt,
                                encoding_str,
                                badchar as i32,
                                start,
                                reason_str,
                            );
                        } else {
                            result = PyUnicode_FromFormat(
                                c"'%U' codec can't encode characters in position %zd-%zd: %U"
                                    .as_ptr(),
                                encoding_str,
                                start,
                                end.wrapping_sub(1),
                                reason_str,
                            );
                        }
                    }
                }
            }
            Py_XDECREF!(reason_str);
            Py_XDECREF!(encoding_str);
            return result;
        }
    }
}

static mut _PyExc_UnicodeEncodeError: _typeobject = {
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
    __t0.tp_name = (b"UnicodeEncodeError\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pyerrors_h_52>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(UnicodeError_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = None;
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = Some(UnicodeEncodeError_str);
    __t0.tp_getattro = None;
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 17408u64;
    __t0.tp_doc = (b"Unicode encoding error.\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_traverse = Some(UnicodeError_traverse);
    __t0.tp_clear = Some(UnicodeError_clear);
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = core::ptr::null_mut::<PyMethodDef>();
    __t0.tp_members =
        unsafe { core::ptr::addr_of_mut!(UnicodeError_members.0) as *mut PyMemberDef };
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(_PyExc_UnicodeError) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_52, dict) as u64) as i64;
    __t0.tp_init = Some(UnicodeEncodeError_init);
    __t0.tp_alloc = None;
    __t0.tp_new = Some(BaseException_new);
    __t0
};

#[unsafe(no_mangle)]
static mut PyExc_UnicodeEncodeError: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_UnicodeEncodeError) }) as *mut _object;

// UnicodeDecodeError extends UnicodeError
#[unsafe(link_section = ".text.slate_distinct.exceptions.UnicodeDecodeError_init")]
extern "C-unwind" fn UnicodeDecodeError_init(
    self_: *mut _object,
    args: *mut _object,
    kwds: *mut _object,
) -> i32 {
    if BaseException_init(self_, args, kwds) == 1i32.wrapping_neg() {
        return 1i32.wrapping_neg();
    }
    let mut encoding: *mut _object = core::ptr::null_mut::<_object>();
    let mut object: *mut _object = core::ptr::null_mut::<_object>();
    let mut reason: *mut _object = core::ptr::null_mut::<_object>(); // borrowed
    let mut start: i64 = 1i32.wrapping_neg() as i64;
    let mut end: i64 = 1i32.wrapping_neg() as i64;
    if (unsafe {
        PyArg_ParseTuple(
            args,
            c"UOnnU".as_ptr(),
            core::ptr::addr_of_mut!(encoding),
            core::ptr::addr_of_mut!(object),
            core::ptr::addr_of_mut!(start),
            core::ptr::addr_of_mut!(end),
            core::ptr::addr_of_mut!(reason),
        )
    }) == 0
    {
        return 1i32.wrapping_neg();
    }
    if PyBytes_Check!(object) != 0 {
        Py_INCREF!(object); // make 'object' a strong reference
    } else {
        let mut view: __SlateRecord_pybuffer_h_33 = unsafe { core::mem::zeroed() };
        if (unsafe { PyObject_GetBuffer(object, core::ptr::addr_of_mut!(view), 0i32) }) != 0 {
            return 1i32.wrapping_neg();
        }
        // 'object' is borrowed, so we can re-use the variable
        object = unsafe { PyBytes_FromStringAndSize(view.buf as *const i8, view.len) };
        unsafe { PyBuffer_Release(core::ptr::addr_of_mut!(view)) };
        if object.is_null() {
            return 1i32.wrapping_neg();
        }
    }
    let exc: *mut __SlateRecord_pyerrors_h_52 = PyUnicodeErrorObject_CAST!(self_);
    Py_XSETREF!((*exc).encoding, Py_NewRef!(encoding));
    Py_XSETREF!((*exc).object, object); // already a strong reference
    unsafe {
        (*exc).start = start;
    }
    unsafe {
        (*exc).end = end;
    }
    Py_XSETREF!((*exc).reason, Py_NewRef!(reason));
    0
}

#[unsafe(link_section = ".text.slate_distinct.exceptions.UnicodeDecodeError_str")]
extern "C-unwind" fn UnicodeDecodeError_str(self_: *mut _object) -> *mut _object {
    let mut end: i64 = unsafe { core::mem::zeroed() };
    let mut start: i64 = unsafe { core::mem::zeroed() };
    let mut encoding_str: *mut _object = unsafe { core::mem::zeroed() };
    let mut reason_str: *mut _object = unsafe { core::mem::zeroed() };
    let mut result: *mut _object = unsafe { core::mem::zeroed() };
    let mut exc: *mut __SlateRecord_pyerrors_h_52 = unsafe { core::mem::zeroed() };
    unsafe {
        exc = PyUnicodeErrorObject_CAST!(self_);
        result = core::ptr::null_mut::<_object>();
        reason_str = core::ptr::null_mut::<_object>();
        encoding_str = core::ptr::null_mut::<_object>();
        if unsafe { (*exc).object }.is_null() {
            // Not properly initialized.
            return Py_GetConstant(7u32);
        } else {
            // Get reason and encoding as strings, which they might not be if
            // they've been modified after we were constructed.
            reason_str = PyObject_Str((*exc).reason);
            if !reason_str.is_null() {
                encoding_str = PyObject_Str((*exc).encoding);
                if !encoding_str.is_null() {
                    // calls to PyObject_Str(...) above might mutate 'exc->object'
                    if check_unicode_error_attribute((*exc).object, c"object".as_ptr(), 1i32) >= 0 {
                        let len: i64 = PyBytes_GET_SIZE!(unsafe { (*exc).object });
                        start = (*exc).start;
                        end = (*exc).end;
                        if start >= 0
                            && start < len
                            && (end >= 0 && end <= len)
                            && end == start.wrapping_add(1)
                        {
                            let badbyte: i32 = (*PyBytes_AS_STRING!(unsafe { (*exc).object })
                                .wrapping_offset(start as isize)
                                as i32)
                                & 255;
                            result = PyUnicode_FromFormat(
                                c"'%U' codec can't decode byte 0x%02x in position %zd: %U".as_ptr(),
                                encoding_str,
                                badbyte,
                                start,
                                reason_str,
                            );
                        } else {
                            result = PyUnicode_FromFormat(
                                c"'%U' codec can't decode bytes in position %zd-%zd: %U".as_ptr(),
                                encoding_str,
                                start,
                                end.wrapping_sub(1),
                                reason_str,
                            );
                        }
                    }
                }
            }
            Py_XDECREF!(reason_str);
            Py_XDECREF!(encoding_str);
            return result;
        }
    }
}

static mut _PyExc_UnicodeDecodeError: _typeobject = {
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
    __t0.tp_name = (b"UnicodeDecodeError\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pyerrors_h_52>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(UnicodeError_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = None;
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = Some(UnicodeDecodeError_str);
    __t0.tp_getattro = None;
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 17408u64;
    __t0.tp_doc = (b"Unicode decoding error.\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_traverse = Some(UnicodeError_traverse);
    __t0.tp_clear = Some(UnicodeError_clear);
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = core::ptr::null_mut::<PyMethodDef>();
    __t0.tp_members =
        unsafe { core::ptr::addr_of_mut!(UnicodeError_members.0) as *mut PyMemberDef };
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(_PyExc_UnicodeError) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_52, dict) as u64) as i64;
    __t0.tp_init = Some(UnicodeDecodeError_init);
    __t0.tp_alloc = None;
    __t0.tp_new = Some(BaseException_new);
    __t0
};

#[unsafe(no_mangle)]
static mut PyExc_UnicodeDecodeError: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_UnicodeDecodeError) }) as *mut _object;

#[unsafe(no_mangle)]
extern "C-unwind" fn PyUnicodeDecodeError_Create(
    encoding: *const i8,
    object: *const i8,
    length: i64,
    start: i64,
    end: i64,
    reason: *const i8,
) -> *mut _object {
    unsafe {
        PyObject_CallFunction(
            PyExc_UnicodeDecodeError,
            c"sy#nns".as_ptr(),
            encoding,
            object,
            length,
            start,
            end,
            reason,
        )
    }
}

// UnicodeTranslateError extends UnicodeError
#[unsafe(link_section = ".text.slate_distinct.exceptions.UnicodeTranslateError_init")]
extern "C-unwind" fn UnicodeTranslateError_init(
    self_: *mut _object,
    args: *mut _object,
    kwds: *mut _object,
) -> i32 {
    if BaseException_init(self_, args, kwds) == 1i32.wrapping_neg() {
        return 1i32.wrapping_neg();
    }
    let mut object: *mut _object = core::ptr::null_mut::<_object>();
    let mut reason: *mut _object = core::ptr::null_mut::<_object>(); // borrowed
    let mut start: i64 = 1i32.wrapping_neg() as i64;
    let mut end: i64 = 1i32.wrapping_neg() as i64;
    if (unsafe {
        PyArg_ParseTuple(
            args,
            c"UnnU".as_ptr(),
            core::ptr::addr_of_mut!(object),
            core::ptr::addr_of_mut!(start),
            core::ptr::addr_of_mut!(end),
            core::ptr::addr_of_mut!(reason),
        )
    }) == 0
    {
        return 1i32.wrapping_neg();
    }
    let exc: *mut __SlateRecord_pyerrors_h_52 = PyUnicodeErrorObject_CAST!(self_);
    Py_XSETREF!((*exc).object, Py_NewRef!(object));
    unsafe {
        (*exc).start = start;
    }
    unsafe {
        (*exc).end = end;
    }
    Py_XSETREF!((*exc).reason, Py_NewRef!(reason));
    0
}

#[unsafe(link_section = ".text.slate_distinct.exceptions.UnicodeTranslateError_str")]
extern "C-unwind" fn UnicodeTranslateError_str(self_: *mut _object) -> *mut _object {
    let mut fmt: *const i8 = unsafe { core::mem::zeroed() };
    let mut badchar: u32 = unsafe { core::mem::zeroed() };
    let mut end: i64 = unsafe { core::mem::zeroed() };
    let mut start: i64 = unsafe { core::mem::zeroed() };
    let mut reason_str: *mut _object = unsafe { core::mem::zeroed() };
    let mut result: *mut _object = unsafe { core::mem::zeroed() };
    let mut exc: *mut __SlateRecord_pyerrors_h_52 = unsafe { core::mem::zeroed() };
    unsafe {
        exc = PyUnicodeErrorObject_CAST!(self_);
        result = core::ptr::null_mut::<_object>();
        reason_str = core::ptr::null_mut::<_object>();
        if unsafe { (*exc).object }.is_null() {
            // Not properly initialized.
            return Py_GetConstant(7u32);
        } else {
            // Get reason as a string, which it might not be if it's been
            // modified after we were constructed.
            reason_str = PyObject_Str((*exc).reason);
            if !reason_str.is_null() {
                // call to PyObject_Str(...) above might mutate 'exc->object'
                if check_unicode_error_attribute((*exc).object, c"object".as_ptr(), 0i32) >= 0 {
                    let len: i64 = PyUnicode_GET_LENGTH!(unsafe { (*exc).object });
                    start = (*exc).start;
                    end = (*exc).end;
                    if start >= 0
                        && start < len
                        && (end >= 0 && end <= len)
                        && end == start.wrapping_add(1)
                    {
                        badchar = PyUnicode_ReadChar((*exc).object, start);
                        if badchar <= 255 {
                            fmt =
                                c"can't translate character '\\x%02x' in position %zd: %U".as_ptr();
                        } else {
                            if badchar <= 65535 {
                                fmt = c"can't translate character '\\u%04x' in position %zd: %U"
                                    .as_ptr();
                            } else {
                                fmt = c"can't translate character '\\U%08x' in position %zd: %U"
                                    .as_ptr();
                            }
                        }
                        result = PyUnicode_FromFormat(fmt, badchar as i32, start, reason_str);
                    } else {
                        result = PyUnicode_FromFormat(
                            c"can't translate characters in position %zd-%zd: %U".as_ptr(),
                            start,
                            end.wrapping_sub(1),
                            reason_str,
                        );
                    }
                }
            }
            Py_XDECREF!(reason_str);
            return result;
        }
    }
}

static mut _PyExc_UnicodeTranslateError: _typeobject = {
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
    __t0.tp_name = (b"UnicodeTranslateError\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pyerrors_h_52>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(UnicodeError_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = None;
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = Some(UnicodeTranslateError_str);
    __t0.tp_getattro = None;
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 17408u64;
    __t0.tp_doc = (b"Unicode translation error.\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_traverse = Some(UnicodeError_traverse);
    __t0.tp_clear = Some(UnicodeError_clear);
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = core::ptr::null_mut::<PyMethodDef>();
    __t0.tp_members =
        unsafe { core::ptr::addr_of_mut!(UnicodeError_members.0) as *mut PyMemberDef };
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(_PyExc_UnicodeError) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_52, dict) as u64) as i64;
    __t0.tp_init = Some(UnicodeTranslateError_init);
    __t0.tp_alloc = None;
    __t0.tp_new = Some(BaseException_new);
    __t0
};

#[unsafe(no_mangle)]
static mut PyExc_UnicodeTranslateError: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_UnicodeTranslateError) }) as *mut _object;

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyUnicodeTranslateError_Create(
    object: *mut _object,
    start: i64,
    end: i64,
    reason: *const i8,
) -> *mut _object {
    unsafe {
        PyObject_CallFunction(
            PyExc_UnicodeTranslateError,
            c"Onns".as_ptr(),
            object,
            start,
            end,
            reason,
        )
    }
}

// AssertionError extends Exception
static mut _PyExc_AssertionError: _typeobject = {
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
    __t0.tp_name = (b"AssertionError\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pyerrors_h_15>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(BaseException_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = None;
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = None;
    __t0.tp_getattro = None;
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 17408u64;
    __t0.tp_doc = (b"Assertion failed.\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_traverse = Some(BaseException_traverse);
    __t0.tp_clear = Some(BaseException_clear);
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = core::ptr::null_mut::<PyMethodDef>();
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(_PyExc_Exception) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_15, dict) as u64) as i64;
    __t0.tp_init = Some(BaseException_init);
    __t0.tp_alloc = None;
    __t0.tp_new = Some(BaseException_new);
    __t0
};

#[unsafe(no_mangle)]
static mut PyExc_AssertionError: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_AssertionError) }) as *mut _object;

// ArithmeticError extends Exception
static mut _PyExc_ArithmeticError: _typeobject = {
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
    __t0.tp_name = (b"ArithmeticError\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pyerrors_h_15>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(BaseException_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = None;
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = None;
    __t0.tp_getattro = None;
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 17408u64;
    __t0.tp_doc = (b"Base class for arithmetic errors.\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_traverse = Some(BaseException_traverse);
    __t0.tp_clear = Some(BaseException_clear);
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = core::ptr::null_mut::<PyMethodDef>();
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(_PyExc_Exception) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_15, dict) as u64) as i64;
    __t0.tp_init = Some(BaseException_init);
    __t0.tp_alloc = None;
    __t0.tp_new = Some(BaseException_new);
    __t0
};

#[unsafe(no_mangle)]
static mut PyExc_ArithmeticError: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_ArithmeticError) }) as *mut _object;

// FloatingPointError extends ArithmeticError
static mut _PyExc_FloatingPointError: _typeobject = {
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
    __t0.tp_name = (b"FloatingPointError\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pyerrors_h_15>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(BaseException_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = None;
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = None;
    __t0.tp_getattro = None;
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 17408u64;
    __t0.tp_doc = (b"Floating-point operation failed.\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_traverse = Some(BaseException_traverse);
    __t0.tp_clear = Some(BaseException_clear);
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = core::ptr::null_mut::<PyMethodDef>();
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(_PyExc_ArithmeticError) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_15, dict) as u64) as i64;
    __t0.tp_init = Some(BaseException_init);
    __t0.tp_alloc = None;
    __t0.tp_new = Some(BaseException_new);
    __t0
};

#[unsafe(no_mangle)]
static mut PyExc_FloatingPointError: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_FloatingPointError) }) as *mut _object;

// OverflowError extends ArithmeticError
static mut _PyExc_OverflowError: _typeobject = {
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
    __t0.tp_name = (b"OverflowError\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pyerrors_h_15>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(BaseException_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = None;
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = None;
    __t0.tp_getattro = None;
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 17408u64;
    __t0.tp_doc = (b"Result too large to be represented.\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_traverse = Some(BaseException_traverse);
    __t0.tp_clear = Some(BaseException_clear);
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = core::ptr::null_mut::<PyMethodDef>();
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(_PyExc_ArithmeticError) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_15, dict) as u64) as i64;
    __t0.tp_init = Some(BaseException_init);
    __t0.tp_alloc = None;
    __t0.tp_new = Some(BaseException_new);
    __t0
};

#[unsafe(no_mangle)]
static mut PyExc_OverflowError: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_OverflowError) }) as *mut _object;

// ZeroDivisionError extends ArithmeticError
static mut _PyExc_ZeroDivisionError: _typeobject = {
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
    __t0.tp_name = (b"ZeroDivisionError\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pyerrors_h_15>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(BaseException_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = None;
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = None;
    __t0.tp_getattro = None;
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 17408u64;
    __t0.tp_doc = (b"Second argument to a division or modulo operation was zero.\0".as_ptr()
        as *mut i8) as *const i8;
    __t0.tp_traverse = Some(BaseException_traverse);
    __t0.tp_clear = Some(BaseException_clear);
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = core::ptr::null_mut::<PyMethodDef>();
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(_PyExc_ArithmeticError) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_15, dict) as u64) as i64;
    __t0.tp_init = Some(BaseException_init);
    __t0.tp_alloc = None;
    __t0.tp_new = Some(BaseException_new);
    __t0
};

#[unsafe(no_mangle)]
static mut PyExc_ZeroDivisionError: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_ZeroDivisionError) }) as *mut _object;

// SystemError extends Exception
static mut _PyExc_SystemError: _typeobject = {
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
    __t0.tp_name = (b"SystemError\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pyerrors_h_15>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(BaseException_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = None;
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = None;
    __t0.tp_getattro = None;
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 17408u64;
    __t0.tp_doc = (b"Internal error in the Python interpreter.\n\nPlease report this to the Python maintainer, along with the traceback,\nthe Python version, and the hardware/OS platform and version.\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_traverse = Some(BaseException_traverse);
    __t0.tp_clear = Some(BaseException_clear);
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = core::ptr::null_mut::<PyMethodDef>();
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(_PyExc_Exception) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_15, dict) as u64) as i64;
    __t0.tp_init = Some(BaseException_init);
    __t0.tp_alloc = None;
    __t0.tp_new = Some(BaseException_new);
    __t0
};

#[unsafe(no_mangle)]
static mut PyExc_SystemError: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_SystemError) }) as *mut _object;

// ReferenceError extends Exception
static mut _PyExc_ReferenceError: _typeobject = {
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
    __t0.tp_name = (b"ReferenceError\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pyerrors_h_15>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(BaseException_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = None;
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = None;
    __t0.tp_getattro = None;
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 17408u64;
    __t0.tp_doc =
        (b"Weak ref proxy used after referent went away.\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_traverse = Some(BaseException_traverse);
    __t0.tp_clear = Some(BaseException_clear);
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = core::ptr::null_mut::<PyMethodDef>();
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(_PyExc_Exception) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_15, dict) as u64) as i64;
    __t0.tp_init = Some(BaseException_init);
    __t0.tp_alloc = None;
    __t0.tp_new = Some(BaseException_new);
    __t0
};

#[unsafe(no_mangle)]
static mut PyExc_ReferenceError: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_ReferenceError) }) as *mut _object;

// MemoryError extends Exception
fn get_memory_error(allow_allocation: i32, args: *mut _object, kwds: *mut _object) -> *mut _object {
    let mut self_: *mut __SlateRecord_pyerrors_h_15 =
        core::ptr::null_mut::<__SlateRecord_pyerrors_h_15>();
    let state: *mut _Py_exc_state = get_exc_state();
    MEMERRORS_LOCK!();
    if !unsafe { (*state).memerrors_freelist }.is_null() {
        // Fetch MemoryError from freelist and initialize it
        self_ = unsafe { (*state).memerrors_freelist };
        unsafe {
            (*state).memerrors_freelist = (*self_).dict as *mut __SlateRecord_pyerrors_h_15;
        }
        let __v1: i32 = unsafe { (*state).memerrors_numfree };
        unsafe {
            (*state).memerrors_numfree = __v1.wrapping_sub(1);
        }
        unsafe {
            (*self_).dict = core::ptr::null_mut::<_object>();
        }
        unsafe {
            (*self_).args =
                core::ptr::addr_of_mut!(_PyRuntime.static_objects.singletons.tuple_empty)
                    as *mut _object;
        }
        unsafe { _Py_NewReference(self_ as *mut _object) };
        _PyObject_GC_TRACK!(self_);
    }
    MEMERRORS_UNLOCK!();
    if !self_.is_null() {
        return self_ as *mut _object;
    }
    if allow_allocation == 0 {
        let interp: *mut _is = _PyInterpreterState_GET();
        return Py_NewRef__2!(unsafe {
            core::ptr::addr_of_mut!((*interp).static_objects.singletons.last_resort_memory_error)
        });
    }
    BaseException_new(
        (unsafe { PyExc_MemoryError }) as *mut _typeobject,
        args,
        kwds,
    )
}

#[unsafe(link_section = ".text.slate_distinct.exceptions.MemoryError_new")]
extern "C-unwind" fn MemoryError_new(
    r#type: *mut _typeobject,
    args: *mut _object,
    kwds: *mut _object,
) -> *mut _object {
    // If this is a subclass of MemoryError, don't use the freelist
    // and just return a fresh object
    if r#type != ((unsafe { PyExc_MemoryError }) as *mut _typeobject) {
        return BaseException_new(r#type, args, kwds);
    }
    get_memory_error(1i32, args, kwds)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyErr_NoMemory(tstate: *mut _ts) -> *mut _object {
    if Py_IS_TYPE__5!(unsafe { PyExc_MemoryError }) != 0 {
        // PyErr_NoMemory() has been called before PyExc_MemoryError has been
        // initialized by _PyExc_Init()
        Py_FatalError__20!(
            c"Out of memory and PyExc_MemoryError is not initialized yet".as_ptr() as *mut i8
        );
    }
    let err: *mut _object = get_memory_error(
        0i32,
        core::ptr::null_mut::<_object>(),
        core::ptr::null_mut::<_object>(),
    );
    if !err.is_null() {
        unsafe { _PyErr_SetRaisedException(tstate, err) };
    }
    core::ptr::null_mut::<_object>()
}

#[unsafe(link_section = ".text.slate_distinct.exceptions.MemoryError_dealloc")]
extern "C-unwind" fn MemoryError_dealloc(op: *mut _object) {
    let self_: *mut __SlateRecord_pyerrors_h_15 = PyBaseExceptionObject_CAST(op);
    _PyObject_GC_UNTRACK__2!(self_);
    BaseException_clear(op);
    // If this is a subclass of MemoryError, we don't need to
    // do anything in the free-list
    if Py_IS_TYPE__2!(self_, (unsafe { PyExc_MemoryError }) as *mut _typeobject) == 0 {
        unsafe { unsafe { (*Py_TYPE__2!(self_)).tp_free }.unwrap()(op as *mut ()) };
        return;
    }
    let state: *mut _Py_exc_state = get_exc_state();
    MEMERRORS_LOCK!();
    if (unsafe { (*state).memerrors_numfree }) < 16 {
        unsafe {
            (*self_).dict = (*state).memerrors_freelist as *mut _object;
        }
        unsafe {
            (*state).memerrors_freelist = self_;
        }
        let __v1: i32 = unsafe { (*state).memerrors_numfree };
        unsafe {
            (*state).memerrors_numfree = __v1.wrapping_add(1);
        }
        MEMERRORS_UNLOCK!();
        return;
    }
    MEMERRORS_UNLOCK!();
    unsafe { unsafe { (*Py_TYPE__2!(self_)).tp_free }.unwrap()(self_ as *mut ()) };
}

fn preallocate_memerrors() -> i32 {
    // We create enough MemoryErrors and then decref them, which will fill
    // up the freelist.
    let mut i: i32 = 0;
    let mut errors: __SlateAlign16<[*mut _object; 16]> = __SlateAlign16([0 as *mut _object; 16]);
    while i < 16 {
        errors.0[i as usize] = MemoryError_new(
            (unsafe { PyExc_MemoryError }) as *mut _typeobject,
            core::ptr::null_mut::<_object>(),
            core::ptr::null_mut::<_object>(),
        );
        if errors.0[i as usize].is_null() {
            return 1i32.wrapping_neg();
        }
        i = i.wrapping_add(1);
    }
    i = 0;
    while i < 16 {
        Py_DECREF!(errors.0[i as usize]);
        i = i.wrapping_add(1);
    }
    0
}

fn free_preallocated_memerrors(state: *mut _Py_exc_state) {
    while !unsafe { (*state).memerrors_freelist }.is_null() {
        let self_: *mut _object = (unsafe { (*state).memerrors_freelist }) as *mut _object;
        unsafe {
            (*state).memerrors_freelist =
                (*(*state).memerrors_freelist).dict as *mut __SlateRecord_pyerrors_h_15;
        }
        unsafe { unsafe { (*Py_TYPE!(self_)).tp_free }.unwrap()(self_ as *mut ()) };
    }
}

#[unsafe(no_mangle)]
static mut _PyExc_MemoryError: _typeobject = {
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
    __t0.tp_name = (b"MemoryError\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pyerrors_h_15>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(MemoryError_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = None;
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = None;
    __t0.tp_getattro = None;
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 17408u64;
    __t0.tp_doc = (b"Out of memory.\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_traverse = Some(BaseException_traverse);
    __t0.tp_clear = Some(BaseException_clear);
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = core::ptr::null_mut::<PyMethodDef>();
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(_PyExc_Exception) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_15, dict) as u64) as i64;
    __t0.tp_init = Some(BaseException_init);
    __t0.tp_alloc = None;
    __t0.tp_new = Some(MemoryError_new);
    __t0
};

#[unsafe(no_mangle)]
static mut PyExc_MemoryError: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_MemoryError) }) as *mut _object;

// BufferError extends Exception
static mut _PyExc_BufferError: _typeobject = {
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
    __t0.tp_name = (b"BufferError\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pyerrors_h_15>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(BaseException_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = None;
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = None;
    __t0.tp_getattro = None;
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 17408u64;
    __t0.tp_doc = (b"Buffer error.\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_traverse = Some(BaseException_traverse);
    __t0.tp_clear = Some(BaseException_clear);
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = core::ptr::null_mut::<PyMethodDef>();
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(_PyExc_Exception) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_15, dict) as u64) as i64;
    __t0.tp_init = Some(BaseException_init);
    __t0.tp_alloc = None;
    __t0.tp_new = Some(BaseException_new);
    __t0
};

#[unsafe(no_mangle)]
static mut PyExc_BufferError: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_BufferError) }) as *mut _object;

// Warning category docstrings
// Warning extends Exception
static mut _PyExc_Warning: _typeobject = {
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
    __t0.tp_name = (b"Warning\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pyerrors_h_15>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(BaseException_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = None;
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = None;
    __t0.tp_getattro = None;
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 17408u64;
    __t0.tp_doc = (b"Base class for warning categories.\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_traverse = Some(BaseException_traverse);
    __t0.tp_clear = Some(BaseException_clear);
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = core::ptr::null_mut::<PyMethodDef>();
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(_PyExc_Exception) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_15, dict) as u64) as i64;
    __t0.tp_init = Some(BaseException_init);
    __t0.tp_alloc = None;
    __t0.tp_new = Some(BaseException_new);
    __t0
};

#[unsafe(no_mangle)]
static mut PyExc_Warning: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_Warning) }) as *mut _object;

// UserWarning extends Warning
static mut _PyExc_UserWarning: _typeobject = {
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
    __t0.tp_name = (b"UserWarning\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pyerrors_h_15>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(BaseException_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = None;
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = None;
    __t0.tp_getattro = None;
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 17408u64;
    __t0.tp_doc =
        (b"Base class for warnings generated by user code.\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_traverse = Some(BaseException_traverse);
    __t0.tp_clear = Some(BaseException_clear);
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = core::ptr::null_mut::<PyMethodDef>();
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(_PyExc_Warning) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_15, dict) as u64) as i64;
    __t0.tp_init = Some(BaseException_init);
    __t0.tp_alloc = None;
    __t0.tp_new = Some(BaseException_new);
    __t0
};

#[unsafe(no_mangle)]
static mut PyExc_UserWarning: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_UserWarning) }) as *mut _object;

// DeprecationWarning extends Warning
static mut _PyExc_DeprecationWarning: _typeobject = {
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
    __t0.tp_name = (b"DeprecationWarning\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pyerrors_h_15>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(BaseException_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = None;
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = None;
    __t0.tp_getattro = None;
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 17408u64;
    __t0.tp_doc =
        (b"Base class for warnings about deprecated features.\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_traverse = Some(BaseException_traverse);
    __t0.tp_clear = Some(BaseException_clear);
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = core::ptr::null_mut::<PyMethodDef>();
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(_PyExc_Warning) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_15, dict) as u64) as i64;
    __t0.tp_init = Some(BaseException_init);
    __t0.tp_alloc = None;
    __t0.tp_new = Some(BaseException_new);
    __t0
};

#[unsafe(no_mangle)]
static mut PyExc_DeprecationWarning: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_DeprecationWarning) }) as *mut _object;

// PendingDeprecationWarning extends Warning
static mut _PyExc_PendingDeprecationWarning: _typeobject = {
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
    __t0.tp_name = (b"PendingDeprecationWarning\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pyerrors_h_15>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(BaseException_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = None;
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = None;
    __t0.tp_getattro = None;
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 17408u64;
    __t0.tp_doc =
        (b"Base class for warnings about features which will be deprecated\nin the future.\0"
            .as_ptr() as *mut i8) as *const i8;
    __t0.tp_traverse = Some(BaseException_traverse);
    __t0.tp_clear = Some(BaseException_clear);
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = core::ptr::null_mut::<PyMethodDef>();
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(_PyExc_Warning) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_15, dict) as u64) as i64;
    __t0.tp_init = Some(BaseException_init);
    __t0.tp_alloc = None;
    __t0.tp_new = Some(BaseException_new);
    __t0
};

#[unsafe(no_mangle)]
static mut PyExc_PendingDeprecationWarning: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_PendingDeprecationWarning) }) as *mut _object;

// SyntaxWarning extends Warning
static mut _PyExc_SyntaxWarning: _typeobject = {
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
    __t0.tp_name = (b"SyntaxWarning\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pyerrors_h_15>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(BaseException_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = None;
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = None;
    __t0.tp_getattro = None;
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 17408u64;
    __t0.tp_doc =
        (b"Base class for warnings about dubious syntax.\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_traverse = Some(BaseException_traverse);
    __t0.tp_clear = Some(BaseException_clear);
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = core::ptr::null_mut::<PyMethodDef>();
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(_PyExc_Warning) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_15, dict) as u64) as i64;
    __t0.tp_init = Some(BaseException_init);
    __t0.tp_alloc = None;
    __t0.tp_new = Some(BaseException_new);
    __t0
};

#[unsafe(no_mangle)]
static mut PyExc_SyntaxWarning: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_SyntaxWarning) }) as *mut _object;

// RuntimeWarning extends Warning
static mut _PyExc_RuntimeWarning: _typeobject = {
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
    __t0.tp_name = (b"RuntimeWarning\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pyerrors_h_15>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(BaseException_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = None;
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = None;
    __t0.tp_getattro = None;
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 17408u64;
    __t0.tp_doc = (b"Base class for warnings about dubious runtime behavior.\0".as_ptr() as *mut i8)
        as *const i8;
    __t0.tp_traverse = Some(BaseException_traverse);
    __t0.tp_clear = Some(BaseException_clear);
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = core::ptr::null_mut::<PyMethodDef>();
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(_PyExc_Warning) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_15, dict) as u64) as i64;
    __t0.tp_init = Some(BaseException_init);
    __t0.tp_alloc = None;
    __t0.tp_new = Some(BaseException_new);
    __t0
};

#[unsafe(no_mangle)]
static mut PyExc_RuntimeWarning: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_RuntimeWarning) }) as *mut _object;

// FutureWarning extends Warning
static mut _PyExc_FutureWarning: _typeobject = {
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
    __t0.tp_name = (b"FutureWarning\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pyerrors_h_15>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(BaseException_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = None;
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = None;
    __t0.tp_getattro = None;
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 17408u64;
    __t0.tp_doc =
        (b"Base class for warnings about constructs that will change semantically\nin the future.\0"
            .as_ptr() as *mut i8) as *const i8;
    __t0.tp_traverse = Some(BaseException_traverse);
    __t0.tp_clear = Some(BaseException_clear);
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = core::ptr::null_mut::<PyMethodDef>();
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(_PyExc_Warning) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_15, dict) as u64) as i64;
    __t0.tp_init = Some(BaseException_init);
    __t0.tp_alloc = None;
    __t0.tp_new = Some(BaseException_new);
    __t0
};

#[unsafe(no_mangle)]
static mut PyExc_FutureWarning: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_FutureWarning) }) as *mut _object;

// ImportWarning extends Warning
static mut _PyExc_ImportWarning: _typeobject = {
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
    __t0.tp_name = (b"ImportWarning\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pyerrors_h_15>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(BaseException_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = None;
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = None;
    __t0.tp_getattro = None;
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 17408u64;
    __t0.tp_doc = (b"Base class for warnings about probable mistakes in module imports\0".as_ptr()
        as *mut i8) as *const i8;
    __t0.tp_traverse = Some(BaseException_traverse);
    __t0.tp_clear = Some(BaseException_clear);
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = core::ptr::null_mut::<PyMethodDef>();
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(_PyExc_Warning) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_15, dict) as u64) as i64;
    __t0.tp_init = Some(BaseException_init);
    __t0.tp_alloc = None;
    __t0.tp_new = Some(BaseException_new);
    __t0
};

#[unsafe(no_mangle)]
static mut PyExc_ImportWarning: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_ImportWarning) }) as *mut _object;

// UnicodeWarning extends Warning
static mut _PyExc_UnicodeWarning: _typeobject = {
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
    __t0.tp_name = (b"UnicodeWarning\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pyerrors_h_15>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(BaseException_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = None;
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = None;
    __t0.tp_getattro = None;
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 17408u64;
    __t0.tp_doc = (b"Base class for warnings about Unicode related problems, mostly\nrelated to conversion problems.\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_traverse = Some(BaseException_traverse);
    __t0.tp_clear = Some(BaseException_clear);
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = core::ptr::null_mut::<PyMethodDef>();
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(_PyExc_Warning) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_15, dict) as u64) as i64;
    __t0.tp_init = Some(BaseException_init);
    __t0.tp_alloc = None;
    __t0.tp_new = Some(BaseException_new);
    __t0
};

#[unsafe(no_mangle)]
static mut PyExc_UnicodeWarning: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_UnicodeWarning) }) as *mut _object;

// BytesWarning extends Warning
static mut _PyExc_BytesWarning: _typeobject = {
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
    __t0.tp_name = (b"BytesWarning\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pyerrors_h_15>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(BaseException_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = None;
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = None;
    __t0.tp_getattro = None;
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 17408u64;
    __t0.tp_doc = (b"Base class for warnings about bytes and buffer related problems, mostly\nrelated to conversion from str or comparing to str.\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_traverse = Some(BaseException_traverse);
    __t0.tp_clear = Some(BaseException_clear);
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = core::ptr::null_mut::<PyMethodDef>();
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(_PyExc_Warning) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_15, dict) as u64) as i64;
    __t0.tp_init = Some(BaseException_init);
    __t0.tp_alloc = None;
    __t0.tp_new = Some(BaseException_new);
    __t0
};

#[unsafe(no_mangle)]
static mut PyExc_BytesWarning: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_BytesWarning) }) as *mut _object;

// EncodingWarning extends Warning
static mut _PyExc_EncodingWarning: _typeobject = {
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
    __t0.tp_name = (b"EncodingWarning\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pyerrors_h_15>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(BaseException_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = None;
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = None;
    __t0.tp_getattro = None;
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 17408u64;
    __t0.tp_doc = (b"Base class for warnings about encodings.\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_traverse = Some(BaseException_traverse);
    __t0.tp_clear = Some(BaseException_clear);
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = core::ptr::null_mut::<PyMethodDef>();
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(_PyExc_Warning) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_15, dict) as u64) as i64;
    __t0.tp_init = Some(BaseException_init);
    __t0.tp_alloc = None;
    __t0.tp_new = Some(BaseException_new);
    __t0
};

#[unsafe(no_mangle)]
static mut PyExc_EncodingWarning: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_EncodingWarning) }) as *mut _object;

// ResourceWarning extends Warning
static mut _PyExc_ResourceWarning: _typeobject = {
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
    __t0.tp_name = (b"ResourceWarning\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pyerrors_h_15>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(BaseException_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = None;
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = None;
    __t0.tp_call = None;
    __t0.tp_str = None;
    __t0.tp_getattro = None;
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 17408u64;
    __t0.tp_doc =
        (b"Base class for warnings about resource usage.\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_traverse = Some(BaseException_traverse);
    __t0.tp_clear = Some(BaseException_clear);
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = core::ptr::null_mut::<PyMethodDef>();
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = unsafe { core::ptr::addr_of_mut!(_PyExc_Warning) };
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = (core::mem::offset_of!(__SlateRecord_pyerrors_h_15, dict) as u64) as i64;
    __t0.tp_init = Some(BaseException_init);
    __t0.tp_alloc = None;
    __t0.tp_new = Some(BaseException_new);
    __t0
};

#[unsafe(no_mangle)]
static mut PyExc_ResourceWarning: *mut _object =
    (unsafe { core::ptr::addr_of_mut!(_PyExc_ResourceWarning) }) as *mut _object;

#[repr(C)]
#[derive(Clone, Copy)]
struct static_exception {
    exc: *mut _typeobject,
    name: *const i8,
}

/// Level 1
/// Level 2: BaseException subclasses
/// Level 3: Exception(BaseException) subclasses
/// ITEM(ExceptionGroup),
/// Level 4: ArithmeticError(Exception) subclasses
/// Level 4: Warning(Exception) subclasses
/// Level 4: OSError(Exception) subclasses
/// Level 4: Other subclasses
/// base: SyntaxError(Exception)
/// base: SyntaxError(Exception)
/// base: LookupError(Exception)
/// base: LookupError(Exception)
/// base: ImportError(Exception)
/// base: RuntimeError(Exception)
/// base: RuntimeError(Exception)
/// base: RuntimeError(Exception)
/// base: NameError(Exception)
/// base: ValueError(Exception)
/// Level 5: ConnectionError(OSError) subclasses
/// Level 5: IndentationError(SyntaxError) subclasses
/// base: IndentationError
/// Level 5: UnicodeError(ValueError) subclasses
static mut static_exceptions: __SlateAlign16<[static_exception; 68]> = __SlateAlign16([
    static_exception {
        exc: unsafe { core::ptr::addr_of_mut!(_PyExc_BaseException) },
        name: (b"BaseException\0".as_ptr() as *mut i8) as *const i8,
    },
    static_exception {
        exc: unsafe { core::ptr::addr_of_mut!(_PyExc_BaseExceptionGroup) },
        name: (b"BaseExceptionGroup\0".as_ptr() as *mut i8) as *const i8,
    },
    static_exception {
        exc: unsafe { core::ptr::addr_of_mut!(_PyExc_Exception) },
        name: (b"Exception\0".as_ptr() as *mut i8) as *const i8,
    },
    static_exception {
        exc: unsafe { core::ptr::addr_of_mut!(_PyExc_GeneratorExit) },
        name: (b"GeneratorExit\0".as_ptr() as *mut i8) as *const i8,
    },
    static_exception {
        exc: unsafe { core::ptr::addr_of_mut!(_PyExc_KeyboardInterrupt) },
        name: (b"KeyboardInterrupt\0".as_ptr() as *mut i8) as *const i8,
    },
    static_exception {
        exc: unsafe { core::ptr::addr_of_mut!(_PyExc_SystemExit) },
        name: (b"SystemExit\0".as_ptr() as *mut i8) as *const i8,
    },
    static_exception {
        exc: unsafe { core::ptr::addr_of_mut!(_PyExc_ArithmeticError) },
        name: (b"ArithmeticError\0".as_ptr() as *mut i8) as *const i8,
    },
    static_exception {
        exc: unsafe { core::ptr::addr_of_mut!(_PyExc_AssertionError) },
        name: (b"AssertionError\0".as_ptr() as *mut i8) as *const i8,
    },
    static_exception {
        exc: unsafe { core::ptr::addr_of_mut!(_PyExc_AttributeError) },
        name: (b"AttributeError\0".as_ptr() as *mut i8) as *const i8,
    },
    static_exception {
        exc: unsafe { core::ptr::addr_of_mut!(_PyExc_BufferError) },
        name: (b"BufferError\0".as_ptr() as *mut i8) as *const i8,
    },
    static_exception {
        exc: unsafe { core::ptr::addr_of_mut!(_PyExc_EOFError) },
        name: (b"EOFError\0".as_ptr() as *mut i8) as *const i8,
    },
    static_exception {
        exc: unsafe { core::ptr::addr_of_mut!(_PyExc_ImportError) },
        name: (b"ImportError\0".as_ptr() as *mut i8) as *const i8,
    },
    static_exception {
        exc: unsafe { core::ptr::addr_of_mut!(_PyExc_LookupError) },
        name: (b"LookupError\0".as_ptr() as *mut i8) as *const i8,
    },
    static_exception {
        exc: unsafe { core::ptr::addr_of_mut!(_PyExc_MemoryError) },
        name: (b"MemoryError\0".as_ptr() as *mut i8) as *const i8,
    },
    static_exception {
        exc: unsafe { core::ptr::addr_of_mut!(_PyExc_NameError) },
        name: (b"NameError\0".as_ptr() as *mut i8) as *const i8,
    },
    static_exception {
        exc: unsafe { core::ptr::addr_of_mut!(_PyExc_OSError) },
        name: (b"OSError\0".as_ptr() as *mut i8) as *const i8,
    },
    static_exception {
        exc: unsafe { core::ptr::addr_of_mut!(_PyExc_ReferenceError) },
        name: (b"ReferenceError\0".as_ptr() as *mut i8) as *const i8,
    },
    static_exception {
        exc: unsafe { core::ptr::addr_of_mut!(_PyExc_RuntimeError) },
        name: (b"RuntimeError\0".as_ptr() as *mut i8) as *const i8,
    },
    static_exception {
        exc: unsafe { core::ptr::addr_of_mut!(_PyExc_StopAsyncIteration) },
        name: (b"StopAsyncIteration\0".as_ptr() as *mut i8) as *const i8,
    },
    static_exception {
        exc: unsafe { core::ptr::addr_of_mut!(_PyExc_StopIteration) },
        name: (b"StopIteration\0".as_ptr() as *mut i8) as *const i8,
    },
    static_exception {
        exc: unsafe { core::ptr::addr_of_mut!(_PyExc_SyntaxError) },
        name: (b"SyntaxError\0".as_ptr() as *mut i8) as *const i8,
    },
    static_exception {
        exc: unsafe { core::ptr::addr_of_mut!(_PyExc_SystemError) },
        name: (b"SystemError\0".as_ptr() as *mut i8) as *const i8,
    },
    static_exception {
        exc: unsafe { core::ptr::addr_of_mut!(_PyExc_TypeError) },
        name: (b"TypeError\0".as_ptr() as *mut i8) as *const i8,
    },
    static_exception {
        exc: unsafe { core::ptr::addr_of_mut!(_PyExc_ValueError) },
        name: (b"ValueError\0".as_ptr() as *mut i8) as *const i8,
    },
    static_exception {
        exc: unsafe { core::ptr::addr_of_mut!(_PyExc_Warning) },
        name: (b"Warning\0".as_ptr() as *mut i8) as *const i8,
    },
    static_exception {
        exc: unsafe { core::ptr::addr_of_mut!(_PyExc_FloatingPointError) },
        name: (b"FloatingPointError\0".as_ptr() as *mut i8) as *const i8,
    },
    static_exception {
        exc: unsafe { core::ptr::addr_of_mut!(_PyExc_OverflowError) },
        name: (b"OverflowError\0".as_ptr() as *mut i8) as *const i8,
    },
    static_exception {
        exc: unsafe { core::ptr::addr_of_mut!(_PyExc_ZeroDivisionError) },
        name: (b"ZeroDivisionError\0".as_ptr() as *mut i8) as *const i8,
    },
    static_exception {
        exc: unsafe { core::ptr::addr_of_mut!(_PyExc_BytesWarning) },
        name: (b"BytesWarning\0".as_ptr() as *mut i8) as *const i8,
    },
    static_exception {
        exc: unsafe { core::ptr::addr_of_mut!(_PyExc_DeprecationWarning) },
        name: (b"DeprecationWarning\0".as_ptr() as *mut i8) as *const i8,
    },
    static_exception {
        exc: unsafe { core::ptr::addr_of_mut!(_PyExc_EncodingWarning) },
        name: (b"EncodingWarning\0".as_ptr() as *mut i8) as *const i8,
    },
    static_exception {
        exc: unsafe { core::ptr::addr_of_mut!(_PyExc_FutureWarning) },
        name: (b"FutureWarning\0".as_ptr() as *mut i8) as *const i8,
    },
    static_exception {
        exc: unsafe { core::ptr::addr_of_mut!(_PyExc_ImportWarning) },
        name: (b"ImportWarning\0".as_ptr() as *mut i8) as *const i8,
    },
    static_exception {
        exc: unsafe { core::ptr::addr_of_mut!(_PyExc_PendingDeprecationWarning) },
        name: (b"PendingDeprecationWarning\0".as_ptr() as *mut i8) as *const i8,
    },
    static_exception {
        exc: unsafe { core::ptr::addr_of_mut!(_PyExc_ResourceWarning) },
        name: (b"ResourceWarning\0".as_ptr() as *mut i8) as *const i8,
    },
    static_exception {
        exc: unsafe { core::ptr::addr_of_mut!(_PyExc_RuntimeWarning) },
        name: (b"RuntimeWarning\0".as_ptr() as *mut i8) as *const i8,
    },
    static_exception {
        exc: unsafe { core::ptr::addr_of_mut!(_PyExc_SyntaxWarning) },
        name: (b"SyntaxWarning\0".as_ptr() as *mut i8) as *const i8,
    },
    static_exception {
        exc: unsafe { core::ptr::addr_of_mut!(_PyExc_UnicodeWarning) },
        name: (b"UnicodeWarning\0".as_ptr() as *mut i8) as *const i8,
    },
    static_exception {
        exc: unsafe { core::ptr::addr_of_mut!(_PyExc_UserWarning) },
        name: (b"UserWarning\0".as_ptr() as *mut i8) as *const i8,
    },
    static_exception {
        exc: unsafe { core::ptr::addr_of_mut!(_PyExc_BlockingIOError) },
        name: (b"BlockingIOError\0".as_ptr() as *mut i8) as *const i8,
    },
    static_exception {
        exc: unsafe { core::ptr::addr_of_mut!(_PyExc_ChildProcessError) },
        name: (b"ChildProcessError\0".as_ptr() as *mut i8) as *const i8,
    },
    static_exception {
        exc: unsafe { core::ptr::addr_of_mut!(_PyExc_ConnectionError) },
        name: (b"ConnectionError\0".as_ptr() as *mut i8) as *const i8,
    },
    static_exception {
        exc: unsafe { core::ptr::addr_of_mut!(_PyExc_FileExistsError) },
        name: (b"FileExistsError\0".as_ptr() as *mut i8) as *const i8,
    },
    static_exception {
        exc: unsafe { core::ptr::addr_of_mut!(_PyExc_FileNotFoundError) },
        name: (b"FileNotFoundError\0".as_ptr() as *mut i8) as *const i8,
    },
    static_exception {
        exc: unsafe { core::ptr::addr_of_mut!(_PyExc_InterruptedError) },
        name: (b"InterruptedError\0".as_ptr() as *mut i8) as *const i8,
    },
    static_exception {
        exc: unsafe { core::ptr::addr_of_mut!(_PyExc_IsADirectoryError) },
        name: (b"IsADirectoryError\0".as_ptr() as *mut i8) as *const i8,
    },
    static_exception {
        exc: unsafe { core::ptr::addr_of_mut!(_PyExc_NotADirectoryError) },
        name: (b"NotADirectoryError\0".as_ptr() as *mut i8) as *const i8,
    },
    static_exception {
        exc: unsafe { core::ptr::addr_of_mut!(_PyExc_PermissionError) },
        name: (b"PermissionError\0".as_ptr() as *mut i8) as *const i8,
    },
    static_exception {
        exc: unsafe { core::ptr::addr_of_mut!(_PyExc_ProcessLookupError) },
        name: (b"ProcessLookupError\0".as_ptr() as *mut i8) as *const i8,
    },
    static_exception {
        exc: unsafe { core::ptr::addr_of_mut!(_PyExc_TimeoutError) },
        name: (b"TimeoutError\0".as_ptr() as *mut i8) as *const i8,
    },
    static_exception {
        exc: unsafe { core::ptr::addr_of_mut!(_PyExc_IndentationError) },
        name: (b"IndentationError\0".as_ptr() as *mut i8) as *const i8,
    },
    static_exception {
        exc: unsafe { core::ptr::addr_of_mut!(_PyExc_IncompleteInputError) },
        name: (b"_IncompleteInputError\0".as_ptr() as *mut i8) as *const i8,
    },
    static_exception {
        exc: unsafe { core::ptr::addr_of_mut!(_PyExc_IndexError) },
        name: (b"IndexError\0".as_ptr() as *mut i8) as *const i8,
    },
    static_exception {
        exc: unsafe { core::ptr::addr_of_mut!(_PyExc_KeyError) },
        name: (b"KeyError\0".as_ptr() as *mut i8) as *const i8,
    },
    static_exception {
        exc: unsafe { core::ptr::addr_of_mut!(_PyExc_ModuleNotFoundError) },
        name: (b"ModuleNotFoundError\0".as_ptr() as *mut i8) as *const i8,
    },
    static_exception {
        exc: unsafe { core::ptr::addr_of_mut!(_PyExc_NotImplementedError) },
        name: (b"NotImplementedError\0".as_ptr() as *mut i8) as *const i8,
    },
    static_exception {
        exc: unsafe { core::ptr::addr_of_mut!(_PyExc_PythonFinalizationError) },
        name: (b"PythonFinalizationError\0".as_ptr() as *mut i8) as *const i8,
    },
    static_exception {
        exc: unsafe { core::ptr::addr_of_mut!(_PyExc_RecursionError) },
        name: (b"RecursionError\0".as_ptr() as *mut i8) as *const i8,
    },
    static_exception {
        exc: unsafe { core::ptr::addr_of_mut!(_PyExc_UnboundLocalError) },
        name: (b"UnboundLocalError\0".as_ptr() as *mut i8) as *const i8,
    },
    static_exception {
        exc: unsafe { core::ptr::addr_of_mut!(_PyExc_UnicodeError) },
        name: (b"UnicodeError\0".as_ptr() as *mut i8) as *const i8,
    },
    static_exception {
        exc: unsafe { core::ptr::addr_of_mut!(_PyExc_BrokenPipeError) },
        name: (b"BrokenPipeError\0".as_ptr() as *mut i8) as *const i8,
    },
    static_exception {
        exc: unsafe { core::ptr::addr_of_mut!(_PyExc_ConnectionAbortedError) },
        name: (b"ConnectionAbortedError\0".as_ptr() as *mut i8) as *const i8,
    },
    static_exception {
        exc: unsafe { core::ptr::addr_of_mut!(_PyExc_ConnectionRefusedError) },
        name: (b"ConnectionRefusedError\0".as_ptr() as *mut i8) as *const i8,
    },
    static_exception {
        exc: unsafe { core::ptr::addr_of_mut!(_PyExc_ConnectionResetError) },
        name: (b"ConnectionResetError\0".as_ptr() as *mut i8) as *const i8,
    },
    static_exception {
        exc: unsafe { core::ptr::addr_of_mut!(_PyExc_TabError) },
        name: (b"TabError\0".as_ptr() as *mut i8) as *const i8,
    },
    static_exception {
        exc: unsafe { core::ptr::addr_of_mut!(_PyExc_UnicodeDecodeError) },
        name: (b"UnicodeDecodeError\0".as_ptr() as *mut i8) as *const i8,
    },
    static_exception {
        exc: unsafe { core::ptr::addr_of_mut!(_PyExc_UnicodeEncodeError) },
        name: (b"UnicodeEncodeError\0".as_ptr() as *mut i8) as *const i8,
    },
    static_exception {
        exc: unsafe { core::ptr::addr_of_mut!(_PyExc_UnicodeTranslateError) },
        name: (b"UnicodeTranslateError\0".as_ptr() as *mut i8) as *const i8,
    },
]);

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyExc_InitTypes(interp: *mut _is) -> i32 {
    let mut i: u64 = 0;
    while i < Py_ARRAY_LENGTH__4!() {
        let exc: *mut _typeobject = unsafe { static_exceptions.0[i as usize].exc };
        if (unsafe { _PyStaticType_InitBuiltin(interp, exc) }) < 0 {
            return 1i32.wrapping_neg();
        }
        if (unsafe { (*exc).tp_new }) == Some(BaseException_new)
            && (unsafe { (*exc).tp_init }) == Some(BaseException_init)
        {
            unsafe {
                (*exc).tp_vectorcall = Some(BaseException_vectorcall);
            }
        }
        i = i.wrapping_add(1);
    }
    0
}

fn _PyExc_FiniTypes(interp: *mut _is) {
    let mut i: i64 = Py_ARRAY_LENGTH__4!().wrapping_sub(1) as i64;
    while i >= 0 {
        let exc: *mut _typeobject = unsafe { static_exceptions.0[i as usize].exc };
        unsafe { _PyStaticType_FiniBuiltin(interp, exc) };
        i = i.wrapping_sub(1);
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyExc_InitGlobalObjects(interp: *mut _is) -> __SlateRecord_initconfig_h_19 {
    let mut __slate_compound_0: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_1: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    if preallocate_memerrors() < 0 {
        return unsafe {
            *{
                __slate_compound_0 = {
                    let mut __t0: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                    __t0._type = 1u32;
                    __t0.func = _PyStatus_GET_FUNC__63!() as *const i8;
                    __t0.err_msg = c"memory allocation failed".as_ptr();
                    __t0
                };
                core::ptr::addr_of_mut!(__slate_compound_0)
            }
        };
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

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyExc_InitState(interp: *mut _is) -> __SlateRecord_initconfig_h_19 {
    let mut __slate_compound_0: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_2: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_5: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_8: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_11: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_14: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_17: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_20: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_23: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_26: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_29: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_32: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_35: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_38: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_41: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_44: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_47: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_50: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_53: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_56: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_58: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let state: *mut _Py_exc_state = unsafe { core::ptr::addr_of_mut!((*interp).exc_state) };
    // Add exceptions to errnomap
    unsafe {
        (*state).errnomap = PyDict_New();
    }
    if unsafe { (*state).errnomap }.is_null() {
        return unsafe {
            *{
                __slate_compound_0 = {
                    let mut __t0: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                    __t0._type = 1u32;
                    __t0.func = _PyStatus_GET_FUNC!() as *const i8;
                    __t0.err_msg = c"memory allocation failed".as_ptr();
                    __t0
                };
                core::ptr::addr_of_mut!(__slate_compound_0)
            }
        };
    }
    '__slate_break_3: loop {
        let _code: *mut _object = unsafe { PyLong_FromLong((slate_support::EAGAIN as i32) as i64) };
        let __v1: bool = _code.is_null()
            || (unsafe { PyDict_SetItem((*state).errnomap, _code, PyExc_BlockingIOError) }) != 0;
        if __v1 {
            Py_XDECREF!(_code);
            return unsafe {
                *{
                    __slate_compound_2 = {
                        let mut __t1: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                        __t1._type = 1u32;
                        __t1.func = _PyStatus_GET_FUNC!() as *const i8;
                        __t1.err_msg = c"errmap insertion problem.".as_ptr();
                        __t1
                    };
                    core::ptr::addr_of_mut!(__slate_compound_2)
                }
            };
        }
        Py_DECREF!(_code);
        if true {
            break;
        }
    }
    '__slate_break_6: loop {
        let _code: *mut _object =
            unsafe { PyLong_FromLong((slate_support::EALREADY as i32) as i64) };
        let __v4: bool = _code.is_null()
            || (unsafe { PyDict_SetItem((*state).errnomap, _code, PyExc_BlockingIOError) }) != 0;
        if __v4 {
            Py_XDECREF!(_code);
            return unsafe {
                *{
                    __slate_compound_5 = {
                        let mut __t2: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                        __t2._type = 1u32;
                        __t2.func = _PyStatus_GET_FUNC!() as *const i8;
                        __t2.err_msg = c"errmap insertion problem.".as_ptr();
                        __t2
                    };
                    core::ptr::addr_of_mut!(__slate_compound_5)
                }
            };
        }
        Py_DECREF!(_code);
        if true {
            break;
        }
    }
    '__slate_break_9: loop {
        let _code: *mut _object =
            unsafe { PyLong_FromLong((slate_support::EINPROGRESS as i32) as i64) };
        let __v7: bool = _code.is_null()
            || (unsafe { PyDict_SetItem((*state).errnomap, _code, PyExc_BlockingIOError) }) != 0;
        if __v7 {
            Py_XDECREF!(_code);
            return unsafe {
                *{
                    __slate_compound_8 = {
                        let mut __t3: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                        __t3._type = 1u32;
                        __t3.func = _PyStatus_GET_FUNC!() as *const i8;
                        __t3.err_msg = c"errmap insertion problem.".as_ptr();
                        __t3
                    };
                    core::ptr::addr_of_mut!(__slate_compound_8)
                }
            };
        }
        Py_DECREF!(_code);
        if true {
            break;
        }
    }
    '__slate_break_12: loop {
        let _code: *mut _object = unsafe { PyLong_FromLong(11i64) };
        let __v10: bool = _code.is_null()
            || (unsafe { PyDict_SetItem((*state).errnomap, _code, PyExc_BlockingIOError) }) != 0;
        if __v10 {
            Py_XDECREF!(_code);
            return unsafe {
                *{
                    __slate_compound_11 = {
                        let mut __t4: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                        __t4._type = 1u32;
                        __t4.func = _PyStatus_GET_FUNC!() as *const i8;
                        __t4.err_msg = c"errmap insertion problem.".as_ptr();
                        __t4
                    };
                    core::ptr::addr_of_mut!(__slate_compound_11)
                }
            };
        }
        Py_DECREF!(_code);
        if true {
            break;
        }
    }
    '__slate_break_15: loop {
        let _code: *mut _object = unsafe { PyLong_FromLong((slate_support::EPIPE as i32) as i64) };
        let __v13: bool = _code.is_null()
            || (unsafe { PyDict_SetItem((*state).errnomap, _code, PyExc_BrokenPipeError) }) != 0;
        if __v13 {
            Py_XDECREF!(_code);
            return unsafe {
                *{
                    __slate_compound_14 = {
                        let mut __t5: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                        __t5._type = 1u32;
                        __t5.func = _PyStatus_GET_FUNC!() as *const i8;
                        __t5.err_msg = c"errmap insertion problem.".as_ptr();
                        __t5
                    };
                    core::ptr::addr_of_mut!(__slate_compound_14)
                }
            };
        }
        Py_DECREF!(_code);
        if true {
            break;
        }
    }
    '__slate_break_18: loop {
        let _code: *mut _object =
            unsafe { PyLong_FromLong((slate_support::ESHUTDOWN as i32) as i64) };
        let __v16: bool = _code.is_null()
            || (unsafe { PyDict_SetItem((*state).errnomap, _code, PyExc_BrokenPipeError) }) != 0;
        if __v16 {
            Py_XDECREF!(_code);
            return unsafe {
                *{
                    __slate_compound_17 = {
                        let mut __t6: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                        __t6._type = 1u32;
                        __t6.func = _PyStatus_GET_FUNC!() as *const i8;
                        __t6.err_msg = c"errmap insertion problem.".as_ptr();
                        __t6
                    };
                    core::ptr::addr_of_mut!(__slate_compound_17)
                }
            };
        }
        Py_DECREF!(_code);
        if true {
            break;
        }
    }
    '__slate_break_21: loop {
        let _code: *mut _object = unsafe { PyLong_FromLong((slate_support::ECHILD as i32) as i64) };
        let __v19: bool = _code.is_null()
            || (unsafe { PyDict_SetItem((*state).errnomap, _code, PyExc_ChildProcessError) }) != 0;
        if __v19 {
            Py_XDECREF!(_code);
            return unsafe {
                *{
                    __slate_compound_20 = {
                        let mut __t7: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                        __t7._type = 1u32;
                        __t7.func = _PyStatus_GET_FUNC!() as *const i8;
                        __t7.err_msg = c"errmap insertion problem.".as_ptr();
                        __t7
                    };
                    core::ptr::addr_of_mut!(__slate_compound_20)
                }
            };
        }
        Py_DECREF!(_code);
        if true {
            break;
        }
    }
    '__slate_break_24: loop {
        let _code: *mut _object =
            unsafe { PyLong_FromLong((slate_support::ECONNABORTED as i32) as i64) };
        let __v22: bool = _code.is_null()
            || (unsafe { PyDict_SetItem((*state).errnomap, _code, PyExc_ConnectionAbortedError) })
                != 0;
        if __v22 {
            Py_XDECREF!(_code);
            return unsafe {
                *{
                    __slate_compound_23 = {
                        let mut __t8: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                        __t8._type = 1u32;
                        __t8.func = _PyStatus_GET_FUNC!() as *const i8;
                        __t8.err_msg = c"errmap insertion problem.".as_ptr();
                        __t8
                    };
                    core::ptr::addr_of_mut!(__slate_compound_23)
                }
            };
        }
        Py_DECREF!(_code);
        if true {
            break;
        }
    }
    '__slate_break_27: loop {
        let _code: *mut _object =
            unsafe { PyLong_FromLong((slate_support::ECONNREFUSED as i32) as i64) };
        let __v25: bool = _code.is_null()
            || (unsafe { PyDict_SetItem((*state).errnomap, _code, PyExc_ConnectionRefusedError) })
                != 0;
        if __v25 {
            Py_XDECREF!(_code);
            return unsafe {
                *{
                    __slate_compound_26 = {
                        let mut __t9: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                        __t9._type = 1u32;
                        __t9.func = _PyStatus_GET_FUNC!() as *const i8;
                        __t9.err_msg = c"errmap insertion problem.".as_ptr();
                        __t9
                    };
                    core::ptr::addr_of_mut!(__slate_compound_26)
                }
            };
        }
        Py_DECREF!(_code);
        if true {
            break;
        }
    }
    '__slate_break_30: loop {
        let _code: *mut _object =
            unsafe { PyLong_FromLong((slate_support::ECONNRESET as i32) as i64) };
        let __v28: bool = _code.is_null()
            || (unsafe { PyDict_SetItem((*state).errnomap, _code, PyExc_ConnectionResetError) })
                != 0;
        if __v28 {
            Py_XDECREF!(_code);
            return unsafe {
                *{
                    __slate_compound_29 = {
                        let mut __t10: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                        __t10._type = 1u32;
                        __t10.func = _PyStatus_GET_FUNC!() as *const i8;
                        __t10.err_msg = c"errmap insertion problem.".as_ptr();
                        __t10
                    };
                    core::ptr::addr_of_mut!(__slate_compound_29)
                }
            };
        }
        Py_DECREF!(_code);
        if true {
            break;
        }
    }
    '__slate_break_33: loop {
        let _code: *mut _object = unsafe { PyLong_FromLong((slate_support::EEXIST as i32) as i64) };
        let __v31: bool = _code.is_null()
            || (unsafe { PyDict_SetItem((*state).errnomap, _code, PyExc_FileExistsError) }) != 0;
        if __v31 {
            Py_XDECREF!(_code);
            return unsafe {
                *{
                    __slate_compound_32 = {
                        let mut __t11: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                        __t11._type = 1u32;
                        __t11.func = _PyStatus_GET_FUNC!() as *const i8;
                        __t11.err_msg = c"errmap insertion problem.".as_ptr();
                        __t11
                    };
                    core::ptr::addr_of_mut!(__slate_compound_32)
                }
            };
        }
        Py_DECREF!(_code);
        if true {
            break;
        }
    }
    '__slate_break_36: loop {
        let _code: *mut _object = unsafe { PyLong_FromLong((slate_support::ENOENT as i32) as i64) };
        let __v34: bool = _code.is_null()
            || (unsafe { PyDict_SetItem((*state).errnomap, _code, PyExc_FileNotFoundError) }) != 0;
        if __v34 {
            Py_XDECREF!(_code);
            return unsafe {
                *{
                    __slate_compound_35 = {
                        let mut __t12: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                        __t12._type = 1u32;
                        __t12.func = _PyStatus_GET_FUNC!() as *const i8;
                        __t12.err_msg = c"errmap insertion problem.".as_ptr();
                        __t12
                    };
                    core::ptr::addr_of_mut!(__slate_compound_35)
                }
            };
        }
        Py_DECREF!(_code);
        if true {
            break;
        }
    }
    '__slate_break_39: loop {
        let _code: *mut _object = unsafe { PyLong_FromLong((slate_support::EISDIR as i32) as i64) };
        let __v37: bool = _code.is_null()
            || (unsafe { PyDict_SetItem((*state).errnomap, _code, PyExc_IsADirectoryError) }) != 0;
        if __v37 {
            Py_XDECREF!(_code);
            return unsafe {
                *{
                    __slate_compound_38 = {
                        let mut __t13: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                        __t13._type = 1u32;
                        __t13.func = _PyStatus_GET_FUNC!() as *const i8;
                        __t13.err_msg = c"errmap insertion problem.".as_ptr();
                        __t13
                    };
                    core::ptr::addr_of_mut!(__slate_compound_38)
                }
            };
        }
        Py_DECREF!(_code);
        if true {
            break;
        }
    }
    '__slate_break_42: loop {
        let _code: *mut _object =
            unsafe { PyLong_FromLong((slate_support::ENOTDIR as i32) as i64) };
        let __v40: bool = _code.is_null()
            || (unsafe { PyDict_SetItem((*state).errnomap, _code, PyExc_NotADirectoryError) }) != 0;
        if __v40 {
            Py_XDECREF!(_code);
            return unsafe {
                *{
                    __slate_compound_41 = {
                        let mut __t14: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                        __t14._type = 1u32;
                        __t14.func = _PyStatus_GET_FUNC!() as *const i8;
                        __t14.err_msg = c"errmap insertion problem.".as_ptr();
                        __t14
                    };
                    core::ptr::addr_of_mut!(__slate_compound_41)
                }
            };
        }
        Py_DECREF!(_code);
        if true {
            break;
        }
    }
    '__slate_break_45: loop {
        let _code: *mut _object = unsafe { PyLong_FromLong((slate_support::EINTR as i32) as i64) };
        let __v43: bool = _code.is_null()
            || (unsafe { PyDict_SetItem((*state).errnomap, _code, PyExc_InterruptedError) }) != 0;
        if __v43 {
            Py_XDECREF!(_code);
            return unsafe {
                *{
                    __slate_compound_44 = {
                        let mut __t15: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                        __t15._type = 1u32;
                        __t15.func = _PyStatus_GET_FUNC!() as *const i8;
                        __t15.err_msg = c"errmap insertion problem.".as_ptr();
                        __t15
                    };
                    core::ptr::addr_of_mut!(__slate_compound_44)
                }
            };
        }
        Py_DECREF!(_code);
        if true {
            break;
        }
    }
    '__slate_break_48: loop {
        let _code: *mut _object = unsafe { PyLong_FromLong((slate_support::EACCES as i32) as i64) };
        let __v46: bool = _code.is_null()
            || (unsafe { PyDict_SetItem((*state).errnomap, _code, PyExc_PermissionError) }) != 0;
        if __v46 {
            Py_XDECREF!(_code);
            return unsafe {
                *{
                    __slate_compound_47 = {
                        let mut __t16: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                        __t16._type = 1u32;
                        __t16.func = _PyStatus_GET_FUNC!() as *const i8;
                        __t16.err_msg = c"errmap insertion problem.".as_ptr();
                        __t16
                    };
                    core::ptr::addr_of_mut!(__slate_compound_47)
                }
            };
        }
        Py_DECREF!(_code);
        if true {
            break;
        }
    }
    '__slate_break_51: loop {
        let _code: *mut _object = unsafe { PyLong_FromLong((slate_support::EPERM as i32) as i64) };
        let __v49: bool = _code.is_null()
            || (unsafe { PyDict_SetItem((*state).errnomap, _code, PyExc_PermissionError) }) != 0;
        if __v49 {
            Py_XDECREF!(_code);
            return unsafe {
                *{
                    __slate_compound_50 = {
                        let mut __t17: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                        __t17._type = 1u32;
                        __t17.func = _PyStatus_GET_FUNC!() as *const i8;
                        __t17.err_msg = c"errmap insertion problem.".as_ptr();
                        __t17
                    };
                    core::ptr::addr_of_mut!(__slate_compound_50)
                }
            };
        }
        Py_DECREF!(_code);
        if true {
            break;
        }
    }
    '__slate_break_54: loop {
        let _code: *mut _object = unsafe { PyLong_FromLong((slate_support::ESRCH as i32) as i64) };
        let __v52: bool = _code.is_null()
            || (unsafe { PyDict_SetItem((*state).errnomap, _code, PyExc_ProcessLookupError) }) != 0;
        if __v52 {
            Py_XDECREF!(_code);
            return unsafe {
                *{
                    __slate_compound_53 = {
                        let mut __t18: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                        __t18._type = 1u32;
                        __t18.func = _PyStatus_GET_FUNC!() as *const i8;
                        __t18.err_msg = c"errmap insertion problem.".as_ptr();
                        __t18
                    };
                    core::ptr::addr_of_mut!(__slate_compound_53)
                }
            };
        }
        Py_DECREF!(_code);
        if true {
            break;
        }
    }
    '__slate_break_57: loop {
        let _code: *mut _object =
            unsafe { PyLong_FromLong((slate_support::ETIMEDOUT as i32) as i64) };
        let __v55: bool = _code.is_null()
            || (unsafe { PyDict_SetItem((*state).errnomap, _code, PyExc_TimeoutError) }) != 0;
        if __v55 {
            Py_XDECREF!(_code);
            return unsafe {
                *{
                    __slate_compound_56 = {
                        let mut __t19: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                        __t19._type = 1u32;
                        __t19.func = _PyStatus_GET_FUNC!() as *const i8;
                        __t19.err_msg = c"errmap insertion problem.".as_ptr();
                        __t19
                    };
                    core::ptr::addr_of_mut!(__slate_compound_56)
                }
            };
        }
        Py_DECREF!(_code);
        if true {
            break;
        }
    }
    return unsafe {
        *{
            __slate_compound_58 = {
                let mut __t20: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                __t20._type = 0u32;
                __t20
            };
            core::ptr::addr_of_mut!(__slate_compound_58)
        }
    };
}

/// Add exception types to the builtins module
#[unsafe(no_mangle)]
extern "C-unwind" fn _PyBuiltins_AddExceptions(bltinmod: *mut _object) -> i32 {
    let mod_dict: *mut _object = unsafe { PyModule_GetDict(bltinmod) };
    if mod_dict.is_null() {
        return 1i32.wrapping_neg();
    }
    let mut i: u64 = 0;
    while i < Py_ARRAY_LENGTH__4!() {
        let item: static_exception = unsafe { static_exceptions.0[i as usize] };
        if (unsafe { PyDict_SetItemString(mod_dict, item.name, item.exc as *mut _object) }) != 0 {
            return 1i32.wrapping_neg();
        }
        i = i.wrapping_add(1);
    }
    let PyExc_ExceptionGroup: *mut _object = create_exception_group_class();
    if PyExc_ExceptionGroup.is_null() {
        return 1i32.wrapping_neg();
    }
    if (unsafe { PyDict_SetItemString(mod_dict, c"ExceptionGroup".as_ptr(), PyExc_ExceptionGroup) })
        != 0
    {
        return 1i32.wrapping_neg();
    }
    loop {
        unsafe {
            PyExc_EnvironmentError = PyExc_OSError;
        }
        if (unsafe { PyDict_SetItemString(mod_dict, c"EnvironmentError".as_ptr(), PyExc_OSError) })
            != 0
        {
            return 1i32.wrapping_neg();
        }
        if true {
            break;
        }
    }
    loop {
        unsafe {
            PyExc_IOError = PyExc_OSError;
        }
        if (unsafe { PyDict_SetItemString(mod_dict, c"IOError".as_ptr(), PyExc_OSError) }) != 0 {
            return 1i32.wrapping_neg();
        }
        if true {
            break;
        }
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyExc_ClearExceptionGroupType(interp: *mut _is) {
    let state: *mut _Py_exc_state = unsafe { core::ptr::addr_of_mut!((*interp).exc_state) };
    Py_CLEAR!((*state).PyExc_ExceptionGroup);
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyExc_Fini(interp: *mut _is) {
    let state: *mut _Py_exc_state = unsafe { core::ptr::addr_of_mut!((*interp).exc_state) };
    free_preallocated_memerrors(state);
    Py_CLEAR!((*state).errnomap);
    _PyExc_FiniTypes(interp);
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyException_AddNote(exc: *mut _object, note: *mut _object) -> i32 {
    if PyExceptionInstance_Check!(exc) == 0 {
        unsafe {
            PyErr_Format(
                PyExc_TypeError,
                c"exc must be an exception, not '%s'".as_ptr(),
                (*Py_TYPE!(exc)).tp_name,
            )
        };
        return 1i32.wrapping_neg();
    }
    let r: *mut _object = BaseException_add_note(exc, note);
    let res: i32 = if r.is_null() { 1i32.wrapping_neg() } else { 0 };
    Py_XDECREF!(r);
    res
}
