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
//! PY_TIMEOUT_T is the integral type used to specify timeouts when waiting
//!
//! on a lock (see PyThread_acquire_lock_timed() below).
//! _PyObject_CallNoArgs()
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
//! Suggested size (number of positional arguments) for arrays of PyObject*
//! allocated on a C stack to avoid allocating memory on the heap memory. Such
//! array is used to pass positional arguments to call functions of the
//! PyObject_Vectorcall() family.
//!
//! The size is chosen to not abuse the C stack and so limit the risk of stack
//! overflow. The size is also chosen to allow using the small stack for most
//! function calls of the Python standard library. On 64-bit CPU, it allocates
//! 40 bytes on the stack.
//! Total tool ids available
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
macro_rules! EVAL_CALL_STAT_INC_IF_FUNCTION {
    () => {};
}

macro_rules! FT_ATOMIC_STORE_SSIZE_RELAXED__2 {
    ($value:expr, $new_value:expr) => {
        unsafe {
            $value = ($new_value) as i64;
        }
    };
}

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

macro_rules! PyDict_CheckExact {
    ($op:expr) => {
        Py_IS_TYPE!($op, unsafe { core::ptr::addr_of_mut!(PyDict_Type) })
    };
}

macro_rules! PyDict_Check {
    ($op:expr) => {
        PyType_FastSubclass!(Py_TYPE!($op), 536870912u64)
    };
}

macro_rules! PyDict_GET_SIZE {
    ($op:expr) => {
        PyDict_GET_SIZE(_PyObject_CAST!($op))
    };
}

macro_rules! PyLong_CheckExact {
    ($op:expr) => {
        Py_IS_TYPE!($op, unsafe { core::ptr::addr_of_mut!(PyLong_Type) })
    };
}

macro_rules! PyObject_GC_New {
    ($type:ty, $typeobj:expr) => {
        _Py_CAST!(*mut $type, unsafe { _PyObject_GC_New($typeobj) })
    };
}

macro_rules! PyObject_New {
    ($type:ty, $typeobj:expr) => {
        (unsafe { _PyObject_New($typeobj) }) as *mut $type
    };
}

macro_rules! PyObject_TypeCheck {
    ($ob:expr, $type:expr) => {
        PyObject_TypeCheck(_PyObject_CAST!($ob), $type)
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

macro_rules! PyTuple_SET_ITEM__2 {
    ($op:expr, $index:expr, $value:expr) => {
        PyTuple_SET_ITEM(_PyObject_CAST!($op), $index, _PyObject_CAST!($value));
    };
}

macro_rules! PyType_FastSubclass {
    ($type:expr, $flag:expr) => {
        PyType_HasFeature($type, $flag)
    };
}

macro_rules! PyUnicode_CheckExact {
    ($op:expr) => {
        Py_IS_TYPE!($op, unsafe { core::ptr::addr_of_mut!(PyUnicode_Type) })
    };
}

macro_rules! PyVectorcall_NARGS {
    ($n:expr) => {
        _PyVectorcall_NARGS($n)
    };
}

macro_rules! Py_ARRAY_LENGTH__2 {
    () => {
        (core::mem::size_of::<[*mut _object; 5]>() as u64)
            / (core::mem::size_of::<*mut _object>() as u64)
    };
}

macro_rules! Py_CLEAR__3 {
    ($op:expr) => {
        loop {
            let _tmp_op_ptr: *mut *mut _typeobject = unsafe { core::ptr::addr_of_mut!($op) };
            let _tmp_old_op: *mut _typeobject = unsafe { *_tmp_op_ptr };
            if !_tmp_old_op.is_null() {
                unsafe {
                    *_tmp_op_ptr = core::ptr::null_mut::<_typeobject>();
                }
                Py_DECREF__2!(_tmp_old_op);
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

macro_rules! Py_REFCNT {
    ($ob:expr) => {
        _Py_REFCNT(_PyObject_CAST!($ob))
    };
}

macro_rules! Py_SETREF {
    ($dst:expr, $src:expr) => {
        loop {
            let _tmp_dst_ptr: *mut *mut _object = core::ptr::addr_of_mut!($dst);
            let _tmp_old_dst: *mut _object = unsafe { *_tmp_dst_ptr };
            unsafe {
                *_tmp_dst_ptr = $src;
            }
            Py_DECREF!(_tmp_old_dst);
            if true {
                break;
            }
        }
    };
}

macro_rules! Py_SETREF__2 {
    ($dst:expr, $src:expr) => {
        loop {
            let _tmp_dst_ptr: *mut *mut _object = unsafe { core::ptr::addr_of_mut!($dst) };
            let _tmp_old_dst: *mut _object = unsafe { *_tmp_dst_ptr };
            unsafe {
                *_tmp_dst_ptr = $src;
            }
            Py_DECREF!(_tmp_old_dst);
            if true {
                break;
            }
        }
    };
}

macro_rules! Py_SETREF__13 {
    ($dst:expr, $src:expr) => {
        loop {
            let _tmp_dst_ptr: *mut *mut lru_list_elem = core::ptr::addr_of_mut!($dst);
            let _tmp_old_dst: *mut lru_list_elem = unsafe { *_tmp_dst_ptr };
            unsafe {
                *_tmp_dst_ptr = $src;
            }
            Py_DECREF__2!(_tmp_old_dst);
            if true {
                break;
            }
        }
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

/// Utility macro to help write tp_traverse functions.
/// To use this macro, the tp_traverse function must name its arguments
/// "visit" and "arg".  This is intended to keep tp_traverse functions
/// looking as much alike as possible.
macro_rules! Py_VISIT__2 {
    ($op:expr, $visit:expr, $arg:expr) => {{
        loop {
            if !$op.is_null() {
                let vret: i32 = unsafe { $visit.unwrap()(_PyObject_CAST__2!($op), $arg) };
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

macro_rules! Py_XSETREF__4 {
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

macro_rules! _PyArg_UnpackKeywords {
    ($args:expr, $nargs:expr, $kwargs:expr, $kwnames:expr, $parser:expr, $minpos:expr, $maxpos:expr, $minkw:expr, $varpos:expr, $buf:expr) => {{
        let __v: *const *mut _object = if ($minkw) == 0
            && $kwargs.is_null()
            && $kwnames.is_null()
            && (($minpos) as i64) <= $nargs
            && (($varpos) != 0 || ($nargs) <= (($maxpos) as i64))
            && !$args.is_null()
        {
            $args
        } else {
            unsafe {
                _PyArg_UnpackKeywords(
                    $args,
                    $nargs,
                    core::ptr::null_mut::<_object>(),
                    $kwnames,
                    $parser,
                    $minpos,
                    $maxpos,
                    $minkw,
                    $varpos,
                    $buf,
                )
            }
        };
        __v
    }};
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

macro_rules! _PyObject_GC_IS_TRACKED {
    ($op:expr) => {
        _PyObject_GC_IS_TRACKED(_Py_CAST__2!($op))
    };
}

macro_rules! _PyObject_GC_TRACK__2 {
    ($op:expr) => {
        _PyObject_GC_TRACK(_PyObject_CAST!($op));
    };
}

/// Cast argument to PyTupleObject* type.
macro_rules! _PyTuple_CAST {
    ($op:expr) => {
        _Py_CAST!(*mut __SlateRecord_tupleobject_h_13, $op)
    };
}

macro_rules! _PyTuple_ITEMS {
    ($op:expr) => {
        _Py_RVALUE!(unsafe { (*_PyTuple_CAST!($op)).ob_item.as_mut_ptr() as *mut *mut _object })
    };
}

macro_rules! _PyTuple_RESET_HASH_CACHE {
    ($op:expr) => {
        loop {
            unsafe {
                (*_PyTuple_CAST!($op)).ob_hash = 1i32.wrapping_neg() as i64;
            }
            if true {
                break;
            }
        }
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

macro_rules! keyobject_CAST {
    ($op:expr) => {
        ($op) as *mut __SlateRecord__functoolsmodule_c_813
    };
}

macro_rules! lru_cache_object_CAST {
    ($op:expr) => {
        ($op) as *mut lru_cache_object
    };
}

macro_rules! lru_list_elem_CAST {
    ($op:expr) => {
        ($op) as *mut lru_list_elem
    };
}

/// cast a PyObject pointer PTR to a partialobject pointer (no type checks)
macro_rules! partialobject_CAST {
    ($op:expr) => {
        ($op) as *mut __SlateRecord__functoolsmodule_c_146
    };
}

unsafe extern "C" {
    static mut PyLong_Type: _typeobject;
    static mut PyBaseObject_Type: _typeobject;
    static mut _Py_NoneStruct: _object;
    static mut PyUnicode_Type: _typeobject;
    static mut PyExc_AttributeError: *mut _object;
    static mut PyExc_OverflowError: *mut _object;
    static mut PyExc_TypeError: *mut _object;
    static mut PyTuple_Type: _typeobject;
    static mut PyDict_Type: _typeobject;
    #[thread_local]
    static mut _Py_tss_tstate: *mut _ts;
    static mut _PyRuntime: pyruntimestate;
    fn index(arg0: *const i8, arg1: i32) -> *mut i8;
    fn PyMem_Malloc(arg0: u64) -> *mut ();
    fn PyMem_Free(arg0: *mut ());
    fn PyType_FromModuleAndSpec(
        arg0: *mut _object,
        arg1: *mut __SlateRecord_object_h_385,
        arg2: *mut _object,
    ) -> *mut _object;
    fn PyType_GetQualName(arg0: *mut _typeobject) -> *mut _object;
    fn PyType_GetModuleName(arg0: *mut _typeobject) -> *mut _object;
    fn PyType_IsSubtype(arg0: *mut _typeobject, arg1: *mut _typeobject) -> i32;
    fn PyType_GenericNew(
        arg0: *mut _typeobject,
        arg1: *mut _object,
        arg2: *mut _object,
    ) -> *mut _object;
    fn PyObject_RichCompare(arg0: *mut _object, arg1: *mut _object, arg2: i32) -> *mut _object;
    fn PyObject_GetAttrString(arg0: *mut _object, arg1: *const i8) -> *mut _object;
    fn PyObject_GenericGetAttr(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn PyObject_GenericSetAttr(arg0: *mut _object, arg1: *mut _object, arg2: *mut _object) -> i32;
    fn PyObject_GenericSetDict(arg0: *mut _object, arg1: *mut _object, arg2: *mut ()) -> i32;
    fn PyObject_Hash(arg0: *mut _object) -> i64;
    fn PyCallable_Check(arg0: *mut _object) -> i32;
    fn PyObject_ClearWeakRefs(arg0: *mut _object);
    fn Py_ReprEnter(arg0: *mut _object) -> i32;
    fn Py_ReprLeave(arg0: *mut _object);
    fn Py_GetConstant(arg0: u32) -> *mut _object;
    fn PyType_GetModuleByDef(arg0: *mut _typeobject, arg1: *mut PyModuleDef) -> *mut _object;
    fn _Py_Dealloc(arg0: *mut _object);
    fn _PyObject_New(arg0: *mut _typeobject) -> *mut _object;
    fn _PyObject_GC_New(arg0: *mut _typeobject) -> *mut _object;
    fn PyObject_GC_Track(arg0: *mut ());
    fn PyObject_GC_UnTrack(arg0: *mut ());
    fn PyObject_GC_Del(arg0: *mut ());
    fn PyUnicode_FromString(arg0: *const i8) -> *mut _object;
    fn PyUnicode_FromFormat(arg0: *const i8, ...) -> *mut _object;
    fn PyErr_SetString(arg0: *mut _object, arg1: *const i8);
    fn PyErr_Occurred() -> *mut _object;
    fn PyErr_ExceptionMatches(arg0: *mut _object) -> i32;
    fn PyErr_NoMemory() -> *mut _object;
    fn PyErr_Format(arg0: *mut _object, arg1: *const i8, ...) -> *mut _object;
    fn PyTuple_New(arg0: i64) -> *mut _object;
    fn PyTuple_GetSlice(arg0: *mut _object, arg1: i64, arg2: i64) -> *mut _object;
    fn PyDict_New() -> *mut _object;
    fn PyDict_Clear(arg0: *mut _object);
    fn PyDict_Next(
        arg0: *mut _object,
        arg1: *mut i64,
        arg2: *mut *mut _object,
        arg3: *mut *mut _object,
    ) -> i32;
    fn PyDict_Copy(arg0: *mut _object) -> *mut _object;
    fn PyDict_Merge(arg0: *mut _object, arg1: *mut _object, arg2: i32) -> i32;
    fn PyObject_GenericGetDict(arg0: *mut _object, arg1: *mut ()) -> *mut _object;
    fn PyModuleDef_Init(arg0: *mut PyModuleDef) -> *mut _object;
    fn PyMethod_New(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn Py_GenericAlias(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn PyArg_ParseTuple(arg0: *mut _object, arg1: *const i8, ...) -> i32;
    fn PyArg_ParseTupleAndKeywords(
        arg0: *mut _object,
        arg1: *mut _object,
        arg2: *const i8,
        arg3: *const *mut i8,
        ...
    ) -> i32;
    fn Py_BuildValue(arg0: *const i8, ...) -> *mut _object;
    fn PyModule_AddObjectRef(arg0: *mut _object, arg1: *const i8, arg2: *mut _object) -> i32;
    fn PyModule_AddType(arg0: *mut _object, arg1: *mut _typeobject) -> i32;
    fn PyObject_CallNoArgs(arg0: *mut _object) -> *mut _object;
    fn PyObject_Call(arg0: *mut _object, arg1: *mut _object, arg2: *mut _object) -> *mut _object;
    fn PyObject_CallFunction(arg0: *mut _object, arg1: *const i8, ...) -> *mut _object;
    fn PyObject_Vectorcall(
        arg0: *mut _object,
        arg1: *const *mut _object,
        arg2: u64,
        arg3: *mut _object,
    ) -> *mut _object;
    fn PyObject_GetIter(arg0: *mut _object) -> *mut _object;
    fn PyIter_Next(arg0: *mut _object) -> *mut _object;
    fn PyIndex_Check(arg0: *mut _object) -> i32;
    fn PyNumber_AsSsize_t(arg0: *mut _object, arg1: *mut _object) -> i64;
    fn PySequence_Concat(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn PySequence_Tuple(arg0: *mut _object) -> *mut _object;
    fn PyVectorcall_Function(
        arg0: *mut _object,
    ) -> Option<
        unsafe extern "C-unwind" fn(
            *mut _object,
            *const *mut _object,
            u64,
            *mut _object,
        ) -> *mut _object,
    >;
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
    fn _PyDict_SetItem_KnownHash(
        arg0: *mut _object,
        arg1: *mut _object,
        arg2: *mut _object,
        arg3: i64,
    ) -> i32;
    fn _PyDict_SetItem_KnownHash_LockHeld(
        arg0: *mut __SlateRecord_dictobject_h_33,
        arg1: *mut _object,
        arg2: *mut _object,
        arg3: i64,
    ) -> i32;
    fn _PyDict_GetItemRef_KnownHash_LockHeld(
        arg0: *mut __SlateRecord_dictobject_h_33,
        arg1: *mut _object,
        arg2: i64,
        arg3: *mut *mut _object,
    ) -> i32;
    fn _PyDict_GetItemRef_KnownHash(
        arg0: *mut __SlateRecord_dictobject_h_33,
        arg1: *mut _object,
        arg2: i64,
        arg3: *mut *mut _object,
    ) -> i32;
    fn _PyDict_Pop_KnownHash(
        arg0: *mut __SlateRecord_dictobject_h_33,
        arg1: *mut _object,
        arg2: i64,
        arg3: *mut *mut _object,
    ) -> i32;
    fn _PyDict_Clear_LockHeld(arg0: *mut _object);
    fn _PyArg_UnpackKeywords(
        arg0: *const *mut _object,
        arg1: i64,
        arg2: *mut _object,
        arg3: *mut _object,
        arg4: *mut _PyArg_Parser,
        arg5: i32,
        arg6: i32,
        arg7: i32,
        arg8: i32,
        arg9: *mut *mut _object,
    ) -> *const *mut _object;
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
struct __SlateRecord_object_h_377 {
    slot: i32,
    pfunc: *mut (),
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_object_h_385 {
    name: *const i8,
    basicsize: i32,
    itemsize: i32,
    flags: u32,
    slots: *mut __SlateRecord_object_h_377,
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
// Disallow creating instances of the type: set tp_new to NULL and don't create
// the "__new__" key in the type dictionary.
// Set if the type object is immutable: type attributes cannot be set nor deleted
// Set if the type allows subclassing
// Objects support garbage collection (see objimpl.h)
// Objects behave like an unbound method
fn PyType_HasFeature(r#type: *mut _typeobject, feature: u64) -> i32 {
    let flags: u64 = unsafe { (*r#type).tp_flags };
    (flags & feature != 0) as i32
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

fn Py_XINCREF(op: *mut _object) {
    if !op.is_null() {
        Py_INCREF!(op);
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

fn _Py_XNewRef(obj: *mut _object) -> *mut _object {
    Py_XINCREF!(obj);
    obj
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_unicodeobject_h_163_2 {
    __slate_bits_0: __slate_bits::__SlateBits147U0,
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
struct _PyExecutorObject {}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_dictobject_h_33 {
    ob_base: _object,
    ma_used: i64,
    _ma_watcher_tag: u64,
    ma_keys: *mut _dictkeysobject,
    ma_values: *mut _dictvalues,
}

fn PyDict_GET_SIZE(op: *mut _object) -> i64 {
    let mp: *mut __SlateRecord_dictobject_h_33 = _Py_CAST!(*mut __SlateRecord_dictobject_h_33, op);
    unsafe { (*mp).ma_used }
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
// METH_CLASS and METH_STATIC are a little different; these control
// the construction of methods for a class.  These cannot be used for
// functions in modules.
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
    __slate_bits_0: __slate_bits::__SlateBits198U0,
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
struct __SlateRecord_pycore_structs_h_32_2 {
    code: u8,
    arg: u8,
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
struct code_arena_st {}

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

fn _PyThreadState_GET() -> *mut _ts {
    unsafe { _Py_tss_tstate }
}

fn _PyInterpreterState_GET() -> *mut _is {
    let tstate: *mut _ts = _PyThreadState_GET();
    unsafe { (*tstate).interp }
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

fn _PyObject_CallNoArgs(func: *mut _object) -> *mut _object {
    EVAL_CALL_STAT_INC_IF_FUNCTION!();
    let tstate: *mut _ts = _PyThreadState_GET();
    _PyObject_VectorcallTstate(
        tstate,
        func,
        core::ptr::null::<*mut _object>(),
        0u64,
        core::ptr::null_mut::<_object>(),
    )
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
struct __SlateRecord_pycore_pythread_h_64 {
    ptr: *mut __SlateRecord_pthreadtypes_h_45,
    val: __SlateRecord_pthreadtypes_h_45,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_lock_h_189 {
    bits: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_interp_structs_h_297 {
    mutex: PyMutex,
    hashtable: *mut _Py_hashtable_t,
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
struct xi_exceptions {
    PyExc_InterpreterError: *mut _object,
    PyExc_InterpreterNotFoundError: *mut _object,
    PyExc_NotShareableError: *mut _object,
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

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_interp_structs_h_718 {
    _not_used: i32,
    _hamt_empty_gc_not_used: __SlateRecord_pycore_interp_structs_h_170,
    hamt_empty: __SlateRecord_pycore_structs_h_47,
    last_resort_memory_error: __SlateRecord_pyerrors_h_15,
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
union __SlateRecord_pycore_obmalloc_h_255 {
    _padding: *mut u8,
    count: u32,
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
struct _Py_interp_static_objects {
    singletons: __SlateRecord_pycore_interp_structs_h_718,
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
struct __SlateRecord_pycore_runtime_structs_h_49 {
    fd: i32,
    st_dev: u64,
    st_ino: u64,
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

fn _Py_AS_GC(op: *mut _object) -> *mut __SlateRecord_pycore_interp_structs_h_170 {
    let gc: *mut i8 = (op as *mut i8).wrapping_offset(
        -(core::mem::size_of::<__SlateRecord_pycore_interp_structs_h_170>() as isize),
    );
    gc as *mut __SlateRecord_pycore_interp_structs_h_170
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_tracemalloc_h_70 {
    mem: __SlateRecord_pymem_h_48,
    raw: __SlateRecord_pymem_h_48,
    obj: __SlateRecord_pymem_h_48,
}

fn _PyObject_GC_IS_TRACKED(op: *mut _object) -> i32 {
    let gc: *mut __SlateRecord_pycore_interp_structs_h_170 = _Py_AS_GC(op);
    ((unsafe { (*gc)._gc_next }) != 0) as i32
}

// Bit flags for _gc_prev
//
// Bit 0 is set when tp_finalize is called
fn _PyGCHead_SET_NEXT(
    gc: *mut __SlateRecord_pycore_interp_structs_h_170,
    next: *mut __SlateRecord_pycore_interp_structs_h_170,
) {
    unsafe {
        (*gc)._gc_next = next as u64;
    }
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
union __SlateRecord_sigaction_h_31 {
    sa_handler: Option<unsafe extern "C-unwind" fn(i32)>,
    sa_sigaction:
        Option<unsafe extern "C-unwind" fn(i32, *mut __SlateRecord_siginfo_t_h_124, *mut ())>,
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
struct _fileutils_state {
    force_ascii: i32,
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
struct __SlateRecord__functoolsmodule_c_h_39 {
    _this_is_not_used: __SlateRecord_pycore_interp_structs_h_170,
    ob_base: __SlateRecord_object_h_172,
    ob_hash: i64,
    ob_item: [*mut _object; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord__functoolsmodule_c_h_105 {
    _this_is_not_used: __SlateRecord_pycore_interp_structs_h_170,
    ob_base: __SlateRecord_object_h_172,
    ob_hash: i64,
    ob_item: [*mut _object; 1],
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
    pub struct __SlateBits147U0 {
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
    pub struct __SlateBits198U0 {
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

fn _PyModule_GetState(r#mod: *mut _object) -> *mut () {
    unsafe { (*(r#mod as *mut __SlateRecord_pycore_moduleobject_h_30)).md_state }
}

fn _PyObject_IsUniquelyReferenced(ob: *mut _object) -> i32 {
    (Py_REFCNT!(ob) == 1) as i32
}

// Fast inlined version of PyType_IS_GC()
#[repr(C)]
#[derive(Clone, Copy)]
struct _dictkeysobject {
    dk_refcnt: i64,
    dk_log2_size: u8,
    dk_log2_index_bytes: u8,
    dk_kind: u8,
    dk_version: u32,
    dk_usable: i64,
    dk_nentries: i64,
    dk_indices: [i8; 0],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _dictvalues {
    capacity: u8,
    size: u8,
    embedded: u8,
    valid: u8,
    values: [*mut _object; 1],
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
fn _PyTuple_Recycle(op: *mut _object) {
    _PyTuple_RESET_HASH_CACHE!(op);
    if _PyObject_GC_IS_TRACKED!(op) == 0 {
        _PyObject_GC_TRACK__2!(op);
    }
}

static _functools_cmp_to_key__doc__: __SlateAlign16<[i8; 129]> = __SlateAlign16([
    99i8, 109i8, 112i8, 95i8, 116i8, 111i8, 95i8, 107i8, 101i8, 121i8, 40i8, 36i8, 109i8, 111i8,
    100i8, 117i8, 108i8, 101i8, 44i8, 32i8, 47i8, 44i8, 32i8, 109i8, 121i8, 99i8, 109i8, 112i8,
    41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 67i8, 111i8, 110i8, 118i8, 101i8, 114i8, 116i8, 32i8, 97i8,
    32i8, 99i8, 109i8, 112i8, 61i8, 32i8, 102i8, 117i8, 110i8, 99i8, 116i8, 105i8, 111i8, 110i8,
    32i8, 105i8, 110i8, 116i8, 111i8, 32i8, 97i8, 32i8, 107i8, 101i8, 121i8, 61i8, 32i8, 102i8,
    117i8, 110i8, 99i8, 116i8, 105i8, 111i8, 110i8, 46i8, 10i8, 10i8, 32i8, 32i8, 109i8, 121i8,
    99i8, 109i8, 112i8, 10i8, 32i8, 32i8, 32i8, 32i8, 70i8, 117i8, 110i8, 99i8, 116i8, 105i8,
    111i8, 110i8, 32i8, 116i8, 104i8, 97i8, 116i8, 32i8, 99i8, 111i8, 109i8, 112i8, 97i8, 114i8,
    101i8, 115i8, 32i8, 116i8, 119i8, 111i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 115i8,
    46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct._functoolsmodule._functools_cmp_to_key")]
extern "C-unwind" fn _functools_cmp_to_key(
    module: *mut _object,
    mut args: *const *mut _object,
    nargs: i64,
    kwnames: *mut _object,
) -> *mut _object {
    let mut argsbuf: [*mut _object; 1] = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        args = _PyArg_UnpackKeywords!(
            args,
            nargs,
            core::ptr::null_mut::<()>(),
            kwnames,
            core::ptr::addr_of_mut!(_parser__52),
            1i32,
            1i32,
            0i32,
            0i32,
            argsbuf.as_mut_ptr() as *mut *mut _object
        );
        // minpos
        // maxpos
        // minkw
        // varpos
        if !args.is_null() {
            let mycmp: *mut _object = *args.wrapping_offset(0 as isize);
            return_value = _functools_cmp_to_key_impl(module, mycmp);
        }
        return return_value;
    }
}

static mut _kwtuple__39: __SlateRecord__functoolsmodule_c_h_39 = {
    let mut __t0: __SlateRecord__functoolsmodule_c_h_39 = unsafe { core::mem::zeroed() };
    __t0.ob_base = __SlateRecord_object_h_172 {
        ob_base: _object {
            __slate_anon_0: __SlateRecord_object_h_122 {
                ob_refcnt_full: 1407378104778752i64,
            },
            ob_type: unsafe { core::ptr::addr_of_mut!(PyTuple_Type) },
        },
        ob_size: 1i64,
    };
    __t0.ob_hash = 1i32.wrapping_neg() as i64;
    __t0.ob_item = [unsafe {
        core::ptr::addr_of_mut!(
            _PyRuntime
                .static_objects
                .singletons
                .strings
                .identifiers
                ._py_mycmp
                ._ascii
                .ob_base
        )
    }];
    __t0
};

static _keywords__51: __SlateAlign16<[*const i8; 2]> = __SlateAlign16([
    (b"mycmp\0".as_ptr() as *mut i8) as *const i8,
    core::ptr::null::<i8>(),
]);

static mut _parser__52: _PyArg_Parser = {
    let mut __t0: _PyArg_Parser = unsafe { core::mem::zeroed() };
    __t0.keywords = unsafe { core::ptr::addr_of!(_keywords__51.0) as *const *const i8 };
    __t0.fname = (b"cmp_to_key\0".as_ptr() as *mut i8) as *const i8;
    __t0.kwtuple = unsafe { core::ptr::addr_of_mut!(_kwtuple__39.ob_base.ob_base) };
    __t0
};

static _functools_reduce__doc__: __SlateAlign16<[i8; 460]> = __SlateAlign16([
    114i8, 101i8, 100i8, 117i8, 99i8, 101i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8,
    44i8, 32i8, 102i8, 117i8, 110i8, 99i8, 116i8, 105i8, 111i8, 110i8, 44i8, 32i8, 105i8, 116i8,
    101i8, 114i8, 97i8, 98i8, 108i8, 101i8, 44i8, 32i8, 47i8, 44i8, 32i8, 105i8, 110i8, 105i8,
    116i8, 105i8, 97i8, 108i8, 61i8, 60i8, 117i8, 110i8, 114i8, 101i8, 112i8, 114i8, 101i8, 115i8,
    101i8, 110i8, 116i8, 97i8, 98i8, 108i8, 101i8, 62i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 65i8,
    112i8, 112i8, 108i8, 121i8, 32i8, 97i8, 32i8, 102i8, 117i8, 110i8, 99i8, 116i8, 105i8, 111i8,
    110i8, 32i8, 111i8, 102i8, 32i8, 116i8, 119i8, 111i8, 32i8, 97i8, 114i8, 103i8, 117i8, 109i8,
    101i8, 110i8, 116i8, 115i8, 32i8, 99i8, 117i8, 109i8, 117i8, 108i8, 97i8, 116i8, 105i8, 118i8,
    101i8, 108i8, 121i8, 32i8, 116i8, 111i8, 32i8, 116i8, 104i8, 101i8, 32i8, 105i8, 116i8, 101i8,
    109i8, 115i8, 32i8, 111i8, 102i8, 32i8, 97i8, 110i8, 32i8, 105i8, 116i8, 101i8, 114i8, 97i8,
    98i8, 108i8, 101i8, 44i8, 32i8, 102i8, 114i8, 111i8, 109i8, 32i8, 108i8, 101i8, 102i8, 116i8,
    32i8, 116i8, 111i8, 32i8, 114i8, 105i8, 103i8, 104i8, 116i8, 46i8, 10i8, 10i8, 84i8, 104i8,
    105i8, 115i8, 32i8, 101i8, 102i8, 102i8, 101i8, 99i8, 116i8, 105i8, 118i8, 101i8, 108i8, 121i8,
    32i8, 114i8, 101i8, 100i8, 117i8, 99i8, 101i8, 115i8, 32i8, 116i8, 104i8, 101i8, 32i8, 105i8,
    116i8, 101i8, 114i8, 97i8, 98i8, 108i8, 101i8, 32i8, 116i8, 111i8, 32i8, 97i8, 32i8, 115i8,
    105i8, 110i8, 103i8, 108i8, 101i8, 32i8, 118i8, 97i8, 108i8, 117i8, 101i8, 46i8, 32i8, 32i8,
    73i8, 102i8, 32i8, 105i8, 110i8, 105i8, 116i8, 105i8, 97i8, 108i8, 32i8, 105i8, 115i8, 10i8,
    112i8, 114i8, 101i8, 115i8, 101i8, 110i8, 116i8, 44i8, 32i8, 105i8, 116i8, 32i8, 105i8, 115i8,
    32i8, 112i8, 108i8, 97i8, 99i8, 101i8, 100i8, 32i8, 98i8, 101i8, 102i8, 111i8, 114i8, 101i8,
    32i8, 116i8, 104i8, 101i8, 32i8, 105i8, 116i8, 101i8, 109i8, 115i8, 32i8, 111i8, 102i8, 32i8,
    116i8, 104i8, 101i8, 32i8, 105i8, 116i8, 101i8, 114i8, 97i8, 98i8, 108i8, 101i8, 32i8, 105i8,
    110i8, 32i8, 116i8, 104i8, 101i8, 10i8, 99i8, 97i8, 108i8, 99i8, 117i8, 108i8, 97i8, 116i8,
    105i8, 111i8, 110i8, 44i8, 32i8, 97i8, 110i8, 100i8, 32i8, 115i8, 101i8, 114i8, 118i8, 101i8,
    115i8, 32i8, 97i8, 115i8, 32i8, 97i8, 32i8, 100i8, 101i8, 102i8, 97i8, 117i8, 108i8, 116i8,
    32i8, 119i8, 104i8, 101i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 105i8, 116i8, 101i8, 114i8,
    97i8, 98i8, 108i8, 101i8, 32i8, 105i8, 115i8, 32i8, 101i8, 109i8, 112i8, 116i8, 121i8, 46i8,
    10i8, 10i8, 70i8, 111i8, 114i8, 32i8, 101i8, 120i8, 97i8, 109i8, 112i8, 108i8, 101i8, 44i8,
    32i8, 114i8, 101i8, 100i8, 117i8, 99i8, 101i8, 40i8, 108i8, 97i8, 109i8, 98i8, 100i8, 97i8,
    32i8, 120i8, 44i8, 32i8, 121i8, 58i8, 32i8, 120i8, 43i8, 121i8, 44i8, 32i8, 91i8, 49i8, 44i8,
    32i8, 50i8, 44i8, 32i8, 51i8, 44i8, 32i8, 52i8, 44i8, 32i8, 53i8, 93i8, 41i8, 10i8, 99i8, 97i8,
    108i8, 99i8, 117i8, 108i8, 97i8, 116i8, 101i8, 115i8, 32i8, 40i8, 40i8, 40i8, 40i8, 49i8, 32i8,
    43i8, 32i8, 50i8, 41i8, 32i8, 43i8, 32i8, 51i8, 41i8, 32i8, 43i8, 32i8, 52i8, 41i8, 32i8, 43i8,
    32i8, 53i8, 41i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct._functoolsmodule._functools_reduce")]
extern "C-unwind" fn _functools_reduce(
    module: *mut _object,
    mut args: *const *mut _object,
    nargs: i64,
    kwnames: *mut _object,
) -> *mut _object {
    let mut result: *mut _object = unsafe { core::mem::zeroed() };
    let mut seq: *mut _object = unsafe { core::mem::zeroed() };
    let mut func: *mut _object = unsafe { core::mem::zeroed() };
    let mut __v3: i64 = unsafe { core::mem::zeroed() };
    let mut noptargs: i64 = unsafe { core::mem::zeroed() };
    let mut argsbuf: core::mem::MaybeUninit<__SlateAlign16<[*mut _object; 3]>> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_5: *mut [*mut _object; 3] =
        core::ptr::addr_of_mut!(argsbuf) as *mut [*mut _object; 3];
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        if !kwnames.is_null() {
            __v3 = PyTuple_GET_SIZE!(kwnames);
        } else {
            __v3 = 0;
        }
        noptargs = nargs.wrapping_add(__v3).wrapping_sub(2);
        result = core::ptr::null_mut::<_object>();
        args = _PyArg_UnpackKeywords!(
            args,
            nargs,
            core::ptr::null_mut::<()>(),
            kwnames,
            core::ptr::addr_of_mut!(_parser__118),
            2i32,
            3i32,
            0i32,
            0i32,
            (*__slate_slot_5).as_mut_ptr() as *mut *mut _object
        );
        // minpos
        // maxpos
        // minkw
        // varpos
        if !args.is_null() {
            func = *args.wrapping_offset(0 as isize);
            seq = *args.wrapping_offset(1 as isize);
            if noptargs != 0 {
                result = *args.wrapping_offset(2 as isize);
            }
            return_value = _functools_reduce_impl(module, func, seq, result);
        }
        return return_value;
    }
}

static mut _kwtuple__105: __SlateRecord__functoolsmodule_c_h_105 = {
    let mut __t0: __SlateRecord__functoolsmodule_c_h_105 = unsafe { core::mem::zeroed() };
    __t0.ob_base = __SlateRecord_object_h_172 {
        ob_base: _object {
            __slate_anon_0: __SlateRecord_object_h_122 {
                ob_refcnt_full: 1407378104778752i64,
            },
            ob_type: unsafe { core::ptr::addr_of_mut!(PyTuple_Type) },
        },
        ob_size: 1i64,
    };
    __t0.ob_hash = 1i32.wrapping_neg() as i64;
    __t0.ob_item = [unsafe {
        core::ptr::addr_of_mut!(
            _PyRuntime
                .static_objects
                .singletons
                .strings
                .identifiers
                ._py_initial
                ._ascii
                .ob_base
        )
    }];
    __t0
};

static _keywords__117: __SlateAlign16<[*const i8; 4]> = __SlateAlign16([
    (b"\0".as_ptr() as *mut i8) as *const i8,
    (b"\0".as_ptr() as *mut i8) as *const i8,
    (b"initial\0".as_ptr() as *mut i8) as *const i8,
    core::ptr::null::<i8>(),
]);

static mut _parser__118: _PyArg_Parser = {
    let mut __t0: _PyArg_Parser = unsafe { core::mem::zeroed() };
    __t0.keywords = unsafe { core::ptr::addr_of!(_keywords__117.0) as *const *const i8 };
    __t0.fname = (b"reduce\0".as_ptr() as *mut i8) as *const i8;
    __t0.kwtuple = unsafe { core::ptr::addr_of_mut!(_kwtuple__105.ob_base.ob_base) };
    __t0
};

static _functools__lru_cache_wrapper_cache_info__doc__: __SlateAlign16<[i8; 49]> =
    __SlateAlign16([
        99i8, 97i8, 99i8, 104i8, 101i8, 95i8, 105i8, 110i8, 102i8, 111i8, 40i8, 36i8, 115i8, 101i8,
        108i8, 102i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 112i8,
        111i8, 114i8, 116i8, 32i8, 99i8, 97i8, 99i8, 104i8, 101i8, 32i8, 115i8, 116i8, 97i8, 116i8,
        105i8, 115i8, 116i8, 105i8, 99i8, 115i8, 0i8,
    ]);

#[unsafe(link_section = ".text.slate_distinct._functoolsmodule._functools__lru_cache_wrapper_cache_info")]
extern "C-unwind" fn _functools__lru_cache_wrapper_cache_info(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    let mut return_value: *mut _object = core::ptr::null_mut::<_object>();
    return_value = _functools__lru_cache_wrapper_cache_info_impl(self_);
    return_value
}

static _functools__lru_cache_wrapper_cache_clear__doc__: __SlateAlign16<[i8; 63]> =
    __SlateAlign16([
        99i8, 97i8, 99i8, 104i8, 101i8, 95i8, 99i8, 108i8, 101i8, 97i8, 114i8, 40i8, 36i8, 115i8,
        101i8, 108i8, 102i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 67i8, 108i8,
        101i8, 97i8, 114i8, 32i8, 116i8, 104i8, 101i8, 32i8, 99i8, 97i8, 99i8, 104i8, 101i8, 32i8,
        97i8, 110i8, 100i8, 32i8, 99i8, 97i8, 99i8, 104i8, 101i8, 32i8, 115i8, 116i8, 97i8, 116i8,
        105i8, 115i8, 116i8, 105i8, 99i8, 115i8, 0i8,
    ]);

use slate_support::link;
use slate_support::memcpy;

#[unsafe(link_section = ".text.slate_distinct._functoolsmodule._functools__lru_cache_wrapper_cache_clear")]
extern "C-unwind" fn _functools__lru_cache_wrapper_cache_clear(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    let mut return_value: *mut _object = core::ptr::null_mut::<_object>();
    return_value = _functools__lru_cache_wrapper_cache_clear_impl(self_);
    return_value
}

// _PyDict_Pop_KnownHash()
// _PyLong_GetZero()
// _PyModule_GetState()
// _PyObject_GC_TRACK
// _PyThreadState_GET()
// _PyTuple_ITEMS()
// FT_CLEAR_WEAKREFS()
// [clinic input]
// module _functools
// class _functools._lru_cache_wrapper "PyObject *" "&lru_cache_type_spec"
// [clinic start generated code]
//
// [clinic end generated code: output=da39a3ee5e6b4b0d input=bece4053896b09c0]
// _functools module written and maintained
// by Hye-Shik Chang <perky@FreeBSD.org>
// with adaptations by Raymond Hettinger <python@rcn.com>
// Copyright (c) 2004 Python Software Foundation.
// All rights reserved.
#[repr(C)]
#[derive(Clone, Copy)]
struct _functools_state {
    /// this object is used delimit args and keywords in the cache keys
    kwd_mark: *mut _object,
    placeholder_type: *mut _typeobject,
    /// strong reference (singleton)
    placeholder: *mut _object,
    partial_type: *mut _typeobject,
    keyobject_type: *mut _typeobject,
    lru_list_elem_type: *mut _typeobject,
}

fn get_functools_state(module: *mut _object) -> *mut _functools_state {
    let state: *mut () = _PyModule_GetState(module);
    state as *mut _functools_state
}

// partial object
// The 'Placeholder' singleton indicates which formal positional
//
// parameters are to be bound first when using a 'partial' object.
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord__functoolsmodule_c_53 {
    ob_base: _object,
}

static placeholder_doc: __SlateAlign16<[i8; 85]> = __SlateAlign16([
    84i8, 104i8, 101i8, 32i8, 116i8, 121i8, 112i8, 101i8, 32i8, 111i8, 102i8, 32i8, 116i8, 104i8,
    101i8, 32i8, 80i8, 108i8, 97i8, 99i8, 101i8, 104i8, 111i8, 108i8, 100i8, 101i8, 114i8, 32i8,
    115i8, 105i8, 110i8, 103i8, 108i8, 101i8, 116i8, 111i8, 110i8, 46i8, 10i8, 10i8, 85i8, 115i8,
    101i8, 100i8, 32i8, 97i8, 115i8, 32i8, 97i8, 32i8, 112i8, 108i8, 97i8, 99i8, 101i8, 104i8,
    111i8, 108i8, 100i8, 101i8, 114i8, 32i8, 102i8, 111i8, 114i8, 32i8, 112i8, 97i8, 114i8, 116i8,
    105i8, 97i8, 108i8, 32i8, 97i8, 114i8, 103i8, 117i8, 109i8, 101i8, 110i8, 116i8, 115i8, 46i8,
    0i8,
]);

#[unsafe(link_section = ".text.slate_distinct._functoolsmodule.placeholder_repr")]
extern "C-unwind" fn placeholder_repr(op: *mut _object) -> *mut _object {
    unsafe { PyUnicode_FromString(c"Placeholder".as_ptr()) }
}

#[unsafe(link_section = ".text.slate_distinct._functoolsmodule.placeholder_reduce")]
extern "C-unwind" fn placeholder_reduce(
    op: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    unsafe { PyUnicode_FromString(c"Placeholder".as_ptr()) }
}

static mut placeholder_methods: __SlateAlign16<[PyMethodDef; 2]> = __SlateAlign16([
    {
        let mut __t0: PyMethodDef = unsafe { core::mem::zeroed() };
        __t0.ml_name = (b"__reduce__\0".as_ptr() as *mut i8) as *const i8;
        __t0.ml_meth = Some(placeholder_reduce);
        __t0.ml_flags = 4i32;
        __t0.ml_doc = core::ptr::null::<i8>();
        __t0
    },
    {
        let mut __t1: PyMethodDef = unsafe { core::mem::zeroed() };
        __t1.ml_name = core::ptr::null::<i8>();
        __t1.ml_meth = None;
        __t1
    },
]);

#[unsafe(link_section = ".text.slate_distinct._functoolsmodule.placeholder_dealloc")]
extern "C-unwind" fn placeholder_dealloc(self_: *mut _object) {
    unsafe { PyObject_GC_UnTrack(self_ as *mut ()) };
    let tp: *mut _typeobject = Py_TYPE!(self_);
    unsafe { unsafe { (*tp).tp_free }.unwrap()(self_ as *mut ()) };
    Py_DECREF__2!(tp);
}

#[unsafe(link_section = ".text.slate_distinct._functoolsmodule.placeholder_new")]
extern "C-unwind" fn placeholder_new(
    r#type: *mut _typeobject,
    args: *mut _object,
    kwargs__0: *mut _object,
) -> *mut _object {
    let __v0: bool =
        PyTuple_GET_SIZE!(args) != 0 || !kwargs__0.is_null() && PyDict_GET_SIZE!(kwargs__0) != 0;
    if __v0 {
        unsafe {
            PyErr_SetString(
                PyExc_TypeError,
                c"PlaceholderType takes no arguments".as_ptr(),
            )
        };
        return core::ptr::null_mut::<_object>();
    }
    let state: *mut _functools_state = get_functools_state_by_type(r#type);
    if !unsafe { (*state).placeholder }.is_null() {
        return Py_NewRef!(unsafe { (*state).placeholder });
    }
    let placeholder: *mut _object = unsafe {
        PyType_GenericNew(
            r#type,
            core::ptr::null_mut::<_object>(),
            core::ptr::null_mut::<_object>(),
        )
    };
    if placeholder.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    if unsafe { (*state).placeholder }.is_null() {
        unsafe {
            (*state).placeholder = Py_NewRef!(placeholder);
        }
    }
    placeholder
}

#[unsafe(link_section = ".text.slate_distinct._functoolsmodule.placeholder_traverse")]
extern "C-unwind" fn placeholder_traverse(
    self_: *mut _object,
    mut visit: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
    arg: *mut (),
) -> i32 {
    Py_VISIT__2!(_Py_TYPE(self_), visit, arg);
    0
}

static mut placeholder_type_slots: __SlateAlign16<[__SlateRecord_object_h_377; 7]> =
    __SlateAlign16([
        {
            let mut __t0: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
            __t0.slot = 52i32;
            __t0.pfunc = unsafe {
                core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut _object)>, *mut ()>(
                    Some(placeholder_dealloc),
                )
            };
            __t0
        },
        {
            let mut __t1: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
            __t1.slot = 66i32;
            __t1.pfunc = unsafe {
                core::mem::transmute::<
                    Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
                    *mut (),
                >(Some(placeholder_repr))
            };
            __t1
        },
        {
            let mut __t2: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
            __t2.slot = 56i32;
            __t2.pfunc =
                (unsafe { core::ptr::addr_of!(placeholder_doc.0) as *const i8 }) as *mut ();
            __t2
        },
        {
            let mut __t3: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
            __t3.slot = 64i32;
            __t3.pfunc =
                (unsafe { core::ptr::addr_of_mut!(placeholder_methods.0) as *mut PyMethodDef })
                    as *mut ();
            __t3
        },
        {
            let mut __t4: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
            __t4.slot = 65i32;
            __t4.pfunc = unsafe {
                core::mem::transmute::<
                    Option<
                        unsafe extern "C-unwind" fn(
                            *mut _typeobject,
                            *mut _object,
                            *mut _object,
                        ) -> *mut _object,
                    >,
                    *mut (),
                >(Some(placeholder_new))
            };
            __t4
        },
        {
            let mut __t5: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
            __t5.slot = 71i32;
            __t5.pfunc = unsafe {
                core::mem::transmute::<
                    Option<
                        unsafe extern "C-unwind" fn(
                            *mut _object,
                            Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
                            *mut (),
                        ) -> i32,
                    >,
                    *mut (),
                >(Some(placeholder_traverse))
            };
            __t5
        },
        {
            let mut __t6: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
            __t6.slot = 0i32;
            __t6.pfunc = core::ptr::null_mut::<()>();
            __t6
        },
    ]);

static mut placeholder_type_spec: __SlateRecord_object_h_385 = {
    let mut __t0: __SlateRecord_object_h_385 = unsafe { core::mem::zeroed() };
    __t0.name = (b"functools._PlaceholderType\0".as_ptr() as *mut i8) as *const i8;
    __t0.basicsize = (core::mem::size_of::<__SlateRecord__functoolsmodule_c_53>() as u64) as i32;
    __t0.flags = 16640u32;
    __t0.slots = unsafe {
        core::ptr::addr_of_mut!(placeholder_type_slots.0) as *mut __SlateRecord_object_h_377
    };
    __t0
};

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord__functoolsmodule_c_146 {
    ob_base: _object,
    r#fn: *mut _object,
    args: *mut _object,
    kw: *mut _object,
    /// __dict__
    dict: *mut _object,
    /// List of weak references
    weakreflist: *mut _object,
    /// Placeholder for positional arguments
    placeholder: *mut _object,
    /// Number of placeholders
    phcount: i64,
    vectorcall: Option<
        unsafe extern "C-unwind" fn(
            *mut _object,
            *const *mut _object,
            u64,
            *mut _object,
        ) -> *mut _object,
    >,
}

static mut _functools_module: PyModuleDef = PyModuleDef {
    m_base: PyModuleDef_Base {
        ob_base: _object {
            __slate_anon_0: __SlateRecord_object_h_122 {
                ob_refcnt_full: 1407378104778752i64,
            },
            ob_type: core::ptr::null_mut::<_typeobject>(),
        },
        m_init: None,
        m_index: 0i64,
        m_copy: core::ptr::null_mut::<_object>(),
    },
    m_name: (b"_functools\0".as_ptr() as *mut i8) as *const i8,
    m_doc: unsafe { core::ptr::addr_of!(_functools_doc.0) as *const i8 },
    m_size: (core::mem::size_of::<_functools_state>() as u64) as i64,
    m_methods: unsafe { core::ptr::addr_of_mut!(_functools_methods.0) as *mut PyMethodDef },
    m_slots: unsafe { core::ptr::addr_of_mut!(_functools_slots.0) as *mut PyModuleDef_Slot },
    m_traverse: Some(_functools_traverse),
    m_clear: Some(_functools_clear),
    m_free: Some(_functools_free),
};

fn get_functools_state_by_type(r#type: *mut _typeobject) -> *mut _functools_state {
    let module: *mut _object =
        unsafe { PyType_GetModuleByDef(r#type, core::ptr::addr_of_mut!(_functools_module)) };
    if module.is_null() {
        return core::ptr::null_mut::<_functools_state>();
    }
    get_functools_state(module)
}

/// Not converted to argument clinic, because of `*args, **kwargs` arguments.
#[unsafe(link_section = ".text.slate_distinct._functoolsmodule.partial_new")]
extern "C-unwind" fn partial_new(
    r#type: *mut _typeobject,
    args: *mut _object,
    kw: *mut _object,
) -> *mut _object {
    let mut func: *mut _object = unsafe { core::mem::zeroed() };
    let mut pto_args: *mut _object = unsafe { core::mem::zeroed() };
    let mut pto_kw: *mut _object = unsafe { core::mem::zeroed() };
    let mut pto_phcount: i64 = 0;
    let new_nargs: i64 = PyTuple_GET_SIZE!(args).wrapping_sub(1);
    if new_nargs < 0 {
        unsafe {
            PyErr_SetString(
                PyExc_TypeError,
                c"type 'partial' takes at least one argument".as_ptr(),
            )
        };
        return core::ptr::null_mut::<_object>();
    }
    func = PyTuple_GET_ITEM!(args, 0i32);
    if (unsafe { PyCallable_Check(func) }) == 0 {
        unsafe {
            PyErr_SetString(
                PyExc_TypeError,
                c"the first argument must be callable".as_ptr(),
            )
        };
        return core::ptr::null_mut::<_object>();
    }
    let state: *mut _functools_state = get_functools_state_by_type(r#type);
    if state.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let phold: *mut _object = unsafe { (*state).placeholder };
    // Placeholder restrictions
    let __v0: bool = new_nargs != 0 && PyTuple_GET_ITEM!(args, new_nargs) == phold;
    if __v0 {
        unsafe {
            PyErr_SetString(
                PyExc_TypeError,
                c"trailing Placeholders are not allowed".as_ptr(),
            )
        };
        return core::ptr::null_mut::<_object>();
    }
    // keyword Placeholder prohibition
    if !kw.is_null() {
        let mut key: *mut _object = unsafe { core::mem::zeroed() };
        let mut val: *mut _object = unsafe { core::mem::zeroed() };
        let mut pos: i64 = 0;
        while (unsafe {
            PyDict_Next(
                kw,
                core::ptr::addr_of_mut!(pos),
                core::ptr::addr_of_mut!(key),
                core::ptr::addr_of_mut!(val),
            )
        }) != 0
        {
            if val == phold {
                unsafe {
                    PyErr_SetString(
                        PyExc_TypeError,
                        c"Placeholder cannot be passed as a keyword argument".as_ptr(),
                    )
                };
                return core::ptr::null_mut::<_object>();
            }
        }
    }
    // check wrapped function / object
    pto_kw = core::ptr::null_mut::<_object>();
    pto_args = core::ptr::null_mut::<_object>();
    let res: i32 = PyObject_TypeCheck!(func, unsafe { (*state).partial_type });
    if res == 1i32.wrapping_neg() {
        return core::ptr::null_mut::<_object>();
    }
    if res == 1 {
        // We can use its underlying function directly and merge the arguments.
        let part: *mut __SlateRecord__functoolsmodule_c_146 =
            func as *mut __SlateRecord__functoolsmodule_c_146;
        if unsafe { (*part).dict }.is_null() {
            pto_args = unsafe { (*part).args };
            pto_kw = unsafe { (*part).kw };
            func = unsafe { (*part).r#fn };
            pto_phcount = unsafe { (*part).phcount };
        }
    }
    // create partialobject structure
    let pto: *mut __SlateRecord__functoolsmodule_c_146 =
        (unsafe { unsafe { (*r#type).tp_alloc }.unwrap()(r#type, 0i64) })
            as *mut __SlateRecord__functoolsmodule_c_146;
    if pto.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    unsafe {
        (*pto).r#fn = Py_NewRef!(func);
    }
    unsafe {
        (*pto).placeholder = phold;
    }
    let new_args: *mut _object = unsafe { PyTuple_GetSlice(args, 1i64, new_nargs.wrapping_add(1)) };
    if new_args.is_null() {
        Py_DECREF__2!(pto);
        return core::ptr::null_mut::<_object>();
    }
    // Count placeholders
    let mut phcount: i64 = 0;
    {
        let mut i: i64 = 0;
        while i < new_nargs.wrapping_sub(1) {
            if PyTuple_GET_ITEM!(new_args, i) == phold {
                phcount = phcount.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
    }
    // merge args with args of `func` which is `partial`
    if pto_phcount > 0 && new_nargs > 0 {
        let npargs: i64 = PyTuple_GET_SIZE!(pto_args);
        let mut tot_nargs: i64 = npargs;
        if new_nargs > pto_phcount {
            tot_nargs = tot_nargs.wrapping_add(new_nargs.wrapping_sub(pto_phcount));
        }
        let mut item: *mut _object = unsafe { core::mem::zeroed() };
        let tot_args: *mut _object = unsafe { PyTuple_New(tot_nargs) };
        let mut j: i64 = 0;
        for i in 0..tot_nargs {
            if i < npargs {
                item = PyTuple_GET_ITEM!(pto_args, i);
                if j < new_nargs && item == phold {
                    item = PyTuple_GET_ITEM!(new_args, j);
                    j = j.wrapping_add(1);
                    pto_phcount = pto_phcount.wrapping_sub(1);
                }
            } else {
                item = PyTuple_GET_ITEM!(new_args, j);
                j = j.wrapping_add(1);
            }
            Py_INCREF!(item);
            PyTuple_SET_ITEM__2!(tot_args, i, item);
        }
        unsafe {
            (*pto).args = tot_args;
        }
        unsafe {
            (*pto).phcount = pto_phcount.wrapping_add(phcount);
        }
        Py_DECREF!(new_args);
    } else {
        if pto_args.is_null() {
            unsafe {
                (*pto).args = new_args;
            }
            unsafe {
                (*pto).phcount = phcount;
            }
        } else {
            unsafe {
                (*pto).args = PySequence_Concat(pto_args, new_args);
            }
            unsafe {
                (*pto).phcount = pto_phcount.wrapping_add(phcount);
            }
            Py_DECREF!(new_args);
            if unsafe { (*pto).args }.is_null() {
                Py_DECREF__2!(pto);
                return core::ptr::null_mut::<_object>();
            }
        }
    }
    let __v18: bool = pto_kw.is_null() || PyDict_GET_SIZE!(pto_kw) == 0;
    if __v18 {
        if kw.is_null() {
            unsafe {
                (*pto).kw = PyDict_New();
            }
        } else {
            if _PyObject_IsUniquelyReferenced(kw) != 0 {
                unsafe {
                    (*pto).kw = Py_NewRef!(kw);
                }
            } else {
                unsafe {
                    (*pto).kw = PyDict_Copy(kw);
                }
            }
        }
    } else {
        unsafe {
            (*pto).kw = PyDict_Copy(pto_kw);
        }
        if !kw.is_null() && !unsafe { (*pto).kw }.is_null() {
            if (unsafe { PyDict_Merge((*pto).kw, kw, 1i32) }) != 0 {
                Py_DECREF__2!(pto);
                return core::ptr::null_mut::<_object>();
            }
        }
    }
    if unsafe { (*pto).kw }.is_null() {
        Py_DECREF__2!(pto);
        return core::ptr::null_mut::<_object>();
    }
    partial_setvectorcall(pto);
    pto as *mut _object
}

#[unsafe(link_section = ".text.slate_distinct._functoolsmodule.partial_clear")]
extern "C-unwind" fn partial_clear(self_: *mut _object) -> i32 {
    let pto: *mut __SlateRecord__functoolsmodule_c_146 = partialobject_CAST!(self_);
    Py_CLEAR!((*pto).r#fn);
    Py_CLEAR!((*pto).args);
    Py_CLEAR!((*pto).kw);
    Py_CLEAR!((*pto).dict);
    0
}

#[unsafe(link_section = ".text.slate_distinct._functoolsmodule.partial_traverse")]
extern "C-unwind" fn partial_traverse(
    self_: *mut _object,
    mut visit: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
    arg: *mut (),
) -> i32 {
    let pto: *mut __SlateRecord__functoolsmodule_c_146 = partialobject_CAST!(self_);
    Py_VISIT__2!(_Py_TYPE(pto as *mut _object), visit, arg);
    Py_VISIT!(unsafe { (*pto).r#fn }, visit, arg);
    Py_VISIT!(unsafe { (*pto).args }, visit, arg);
    Py_VISIT!(unsafe { (*pto).kw }, visit, arg);
    Py_VISIT!(unsafe { (*pto).dict }, visit, arg);
    0
}

#[unsafe(link_section = ".text.slate_distinct._functoolsmodule.partial_dealloc")]
extern "C-unwind" fn partial_dealloc(self_: *mut _object) {
    let tp: *mut _typeobject = Py_TYPE!(self_);
    // bpo-31095: UnTrack is needed before calling any callbacks
    unsafe { PyObject_GC_UnTrack(self_ as *mut ()) };
    FT_CLEAR_WEAKREFS!(self_, unsafe { (*partialobject_CAST!(self_)).weakreflist });
    partial_clear(self_);
    unsafe { unsafe { (*tp).tp_free }.unwrap()(self_ as *mut ()) };
    Py_DECREF__2!(tp);
}

#[unsafe(link_section = ".text.slate_distinct._functoolsmodule.partial_descr_get")]
extern "C-unwind" fn partial_descr_get(
    self_: *mut _object,
    obj: *mut _object,
    r#type: *mut _object,
) -> *mut _object {
    if obj == unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } || obj.is_null() {
        return Py_NewRef!(self_);
    }
    unsafe { PyMethod_New(self_, obj) }
}

/// Merging keyword arguments using the vectorcall convention is messy, so
/// if we would need to do that, we stop using vectorcall and fall back
/// to using partial_call() instead.
#[inline(never)]
fn partial_vectorcall_fallback(
    tstate: *mut _ts,
    pto: *mut __SlateRecord__functoolsmodule_c_146,
    args: *const *mut _object,
    nargsf: u64,
    kwnames: *mut _object,
) -> *mut _object {
    unsafe {
        (*pto).vectorcall = None;
    }
    let nargs: i64 = PyVectorcall_NARGS!(nargsf);
    unsafe { _PyObject_MakeTpCall(tstate, pto as *mut _object, args, nargs, kwnames) }
}

#[unsafe(link_section = ".text.slate_distinct._functoolsmodule.partial_vectorcall")]
extern "C-unwind" fn partial_vectorcall(
    self_: *mut _object,
    args: *const *mut _object,
    nargsf: u64,
    kwnames: *mut _object,
) -> *mut _object {
    let pto: *mut __SlateRecord__functoolsmodule_c_146 = partialobject_CAST!(self_);
    let tstate: *mut _ts = _PyThreadState_GET();
    let nargs: i64 = PyVectorcall_NARGS!(nargsf);
    // pto->kw is mutable, so need to check every time
    if PyDict_GET_SIZE!(unsafe { (*pto).kw }) != 0 {
        return partial_vectorcall_fallback(tstate, pto, args, nargsf, kwnames);
    }
    let pto_phcount: i64 = unsafe { (*pto).phcount };
    if nargs < pto_phcount {
        unsafe {
            PyErr_Format(
                PyExc_TypeError,
                c"missing positional arguments in 'partial' call; expected at least %zd, got %zd"
                    .as_ptr(),
                pto_phcount,
                nargs,
            )
        };
        return core::ptr::null_mut::<_object>();
    }
    let mut nargskw: i64 = nargs;
    if !kwnames.is_null() {
        let __v0: i64 = PyTuple_GET_SIZE!(kwnames);
        nargskw = nargskw.wrapping_add(__v0);
    }
    let pto_args: *mut *mut _object = _PyTuple_ITEMS!(unsafe { (*pto).args });
    let pto_nargs: i64 = PyTuple_GET_SIZE!(unsafe { (*pto).args });
    // Fast path if we're called without arguments
    if nargskw == 0 {
        return _PyObject_VectorcallTstate(
            tstate,
            unsafe { (*pto).r#fn },
            pto_args as *const *mut _object,
            pto_nargs as u64,
            core::ptr::null_mut::<_object>(),
        );
    }
    // Fast path using PY_VECTORCALL_ARGUMENTS_OFFSET to prepend a single
    // positional argument
    if pto_nargs == 1
        && nargsf
            & _Py_STATIC_CAST!(u64, 1i32)
                << 8u64
                    .wrapping_mul(core::mem::size_of::<u64>() as u64)
                    .wrapping_sub(1)
            != 0
    {
        let newargs: *mut *mut _object = (args as *mut *mut _object).wrapping_offset(-(1 as isize));
        let tmp: *mut _object = unsafe { *newargs.wrapping_offset(0 as isize) };
        unsafe {
            *newargs.wrapping_offset(0 as isize) = *pto_args.wrapping_offset(0 as isize);
        }
        let ret: *mut _object = _PyObject_VectorcallTstate(
            tstate,
            unsafe { (*pto).r#fn },
            newargs as *const *mut _object,
            nargs.wrapping_add(1) as u64,
            kwnames,
        );
        unsafe {
            *newargs.wrapping_offset(0 as isize) = tmp;
        }
        return ret;
    }
    let mut small_stack: __SlateAlign16<[*mut _object; 5]> = __SlateAlign16([0 as *mut _object; 5]);
    let mut stack: *mut *mut _object = unsafe { core::mem::zeroed() };
    let tot_nargskw: i64 = pto_nargs.wrapping_add(nargskw).wrapping_sub(pto_phcount);
    if tot_nargskw <= (Py_ARRAY_LENGTH__2!() as i64) {
        stack = small_stack.0.as_mut_ptr() as *mut *mut _object;
    } else {
        stack = (unsafe {
            PyMem_Malloc(
                (tot_nargskw as u64).wrapping_mul(core::mem::size_of::<*mut _object>() as u64),
            )
        }) as *mut *mut _object;
        if stack.is_null() {
            unsafe { PyErr_NoMemory() };
            return core::ptr::null_mut::<_object>();
        }
    }
    let mut tot_nargs: i64 = 0;
    if pto_phcount != 0 {
        tot_nargs = pto_nargs.wrapping_add(nargs).wrapping_sub(pto_phcount);
        let mut j: i64 = 0; // New args index
        for i in 0..pto_nargs {
            if (unsafe { *pto_args.wrapping_offset(i as isize) }) == unsafe { (*pto).placeholder } {
                unsafe {
                    *stack.wrapping_offset(i as isize) = *args.wrapping_offset(j as isize);
                }
                j = j.wrapping_add(1);
            } else {
                unsafe {
                    *stack.wrapping_offset(i as isize) = *pto_args.wrapping_offset(i as isize);
                }
            }
        }
        if nargskw > pto_phcount {
            unsafe {
                core::ptr::copy_nonoverlapping(
                    ((args.wrapping_offset(j as isize) as *const ()) as *const core::ffi::c_void)
                        as *const u8,
                    ((stack.wrapping_offset(pto_nargs as isize) as *mut ())
                        as *mut core::ffi::c_void) as *mut u8,
                    ((nargskw.wrapping_sub(j) as u64)
                        .wrapping_mul(core::mem::size_of::<*mut _object>() as u64)
                        as usize) as usize,
                )
            };
        }
    } else {
        tot_nargs = pto_nargs.wrapping_add(nargs);
        // Copy to new stack, using borrowed references
        unsafe {
            core::ptr::copy_nonoverlapping(
                ((pto_args as *const ()) as *const core::ffi::c_void) as *const u8,
                ((stack as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                ((pto_nargs as u64).wrapping_mul(core::mem::size_of::<*mut _object>() as u64)
                    as usize) as usize,
            )
        };
        unsafe {
            core::ptr::copy_nonoverlapping(
                ((args as *const ()) as *const core::ffi::c_void) as *const u8,
                ((stack.wrapping_offset(pto_nargs as isize) as *mut ()) as *mut core::ffi::c_void)
                    as *mut u8,
                ((nargskw as u64).wrapping_mul(core::mem::size_of::<*mut _object>() as u64)
                    as usize) as usize,
            )
        };
    }
    let ret: *mut _object = _PyObject_VectorcallTstate(
        tstate,
        unsafe { (*pto).r#fn },
        stack as *const *mut _object,
        tot_nargs as u64,
        kwnames,
    );
    if stack != (small_stack.0.as_mut_ptr() as *mut *mut _object) {
        unsafe { PyMem_Free(stack as *mut ()) };
    }
    ret
}

/// Set pto->vectorcall depending on the parameters of the partial object
fn partial_setvectorcall(pto: *mut __SlateRecord__functoolsmodule_c_146) {
    if (unsafe { PyVectorcall_Function((*pto).r#fn) }) == None {
        // Don't use vectorcall if the underlying function doesn't support it
        unsafe {
            (*pto).vectorcall = None;
        }
    } else {
        unsafe {
            (*pto).vectorcall = Some(partial_vectorcall);
        }
    }
    // We could have a special case if there are no arguments,
    // but that is unlikely (why use partial without arguments?),
    // so we don't optimize that
}

/// Not converted to argument clinic, because of `*args, **kwargs` arguments.
#[unsafe(link_section = ".text.slate_distinct._functoolsmodule.partial_call")]
extern "C-unwind" fn partial_call(
    self_: *mut _object,
    args: *mut _object,
    kwargs__0: *mut _object,
) -> *mut _object {
    let pto: *mut __SlateRecord__functoolsmodule_c_146 = partialobject_CAST!(self_);
    let nargs: i64 = PyTuple_GET_SIZE!(args);
    let pto_phcount: i64 = unsafe { (*pto).phcount };
    if nargs < pto_phcount {
        unsafe {
            PyErr_Format(
                PyExc_TypeError,
                c"missing positional arguments in 'partial' call; expected at least %zd, got %zd"
                    .as_ptr(),
                pto_phcount,
                nargs,
            )
        };
        return core::ptr::null_mut::<_object>();
    }
    // Merge keywords
    let mut tot_kw: *mut _object = unsafe { core::mem::zeroed() };
    if PyDict_GET_SIZE!(unsafe { (*pto).kw }) == 0 {
        // kwargs can be NULL
        tot_kw = Py_XNewRef!(kwargs__0);
    } else {
        // bpo-27840, bpo-29318: dictionary of keyword parameters must be
        // copied, because a function using "**kwargs" can modify the
        // dictionary.
        tot_kw = unsafe { PyDict_Copy((*pto).kw) };
        if tot_kw.is_null() {
            return core::ptr::null_mut::<_object>();
        }
        if !kwargs__0.is_null() {
            if (unsafe { PyDict_Merge(tot_kw, kwargs__0, 1i32) }) != 0 {
                Py_DECREF!(tot_kw);
                return core::ptr::null_mut::<_object>();
            }
        }
    }
    // Merge positional arguments
    let mut tot_args: *mut _object = unsafe { core::mem::zeroed() };
    if pto_phcount != 0 {
        let pto_nargs: i64 = PyTuple_GET_SIZE!(unsafe { (*pto).args });
        let tot_nargs: i64 = pto_nargs.wrapping_add(nargs).wrapping_sub(pto_phcount);
        tot_args = unsafe { PyTuple_New(tot_nargs) };
        if tot_args.is_null() {
            Py_XDECREF!(tot_kw);
            return core::ptr::null_mut::<_object>();
        }
        let pto_args: *mut _object = unsafe { (*pto).args };
        let mut item: *mut _object = unsafe { core::mem::zeroed() };
        let mut j: i64 = 0; // New args index
        for i in 0..pto_nargs {
            item = PyTuple_GET_ITEM!(pto_args, i);
            if item == unsafe { (*pto).placeholder } {
                item = PyTuple_GET_ITEM!(args, j);
                j = j.wrapping_add(1);
            }
            Py_INCREF!(item);
            PyTuple_SET_ITEM__2!(tot_args, i, item);
        }
        for i in pto_nargs..tot_nargs {
            item = PyTuple_GET_ITEM!(args, j);
            Py_INCREF!(item);
            PyTuple_SET_ITEM__2!(tot_args, i, item);
            j = j.wrapping_add(1);
        }
    } else {
        // Note: tupleconcat() is optimized for empty tuples
        tot_args = unsafe { PySequence_Concat((*pto).args, args) };
        if tot_args.is_null() {
            Py_XDECREF!(tot_kw);
            return core::ptr::null_mut::<_object>();
        }
    }
    let res: *mut _object = unsafe { PyObject_Call((*pto).r#fn, tot_args, tot_kw) };
    Py_DECREF!(tot_args);
    Py_XDECREF!(tot_kw);
    res
}

static partial_doc: __SlateAlign16<[i8; 124]> = __SlateAlign16([
    112i8, 97i8, 114i8, 116i8, 105i8, 97i8, 108i8, 40i8, 102i8, 117i8, 110i8, 99i8, 44i8, 32i8,
    47i8, 44i8, 32i8, 42i8, 97i8, 114i8, 103i8, 115i8, 44i8, 32i8, 42i8, 42i8, 107i8, 101i8, 121i8,
    119i8, 111i8, 114i8, 100i8, 115i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 67i8, 114i8, 101i8,
    97i8, 116i8, 101i8, 32i8, 97i8, 32i8, 110i8, 101i8, 119i8, 32i8, 102i8, 117i8, 110i8, 99i8,
    116i8, 105i8, 111i8, 110i8, 32i8, 119i8, 105i8, 116i8, 104i8, 32i8, 112i8, 97i8, 114i8, 116i8,
    105i8, 97i8, 108i8, 32i8, 97i8, 112i8, 112i8, 108i8, 105i8, 99i8, 97i8, 116i8, 105i8, 111i8,
    110i8, 32i8, 111i8, 102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 103i8, 105i8, 118i8, 101i8, 110i8,
    32i8, 97i8, 114i8, 103i8, 117i8, 109i8, 101i8, 110i8, 116i8, 115i8, 10i8, 97i8, 110i8, 100i8,
    32i8, 107i8, 101i8, 121i8, 119i8, 111i8, 114i8, 100i8, 115i8, 46i8, 0i8,
]);

/// Sentinel
static mut partial_memberlist: __SlateAlign16<[PyMemberDef; 7]> = __SlateAlign16([
    {
        let mut __t0: PyMemberDef = unsafe { core::mem::zeroed() };
        __t0.name = (b"func\0".as_ptr() as *mut i8) as *const i8;
        __t0.r#type = 6i32;
        __t0.offset =
            (core::mem::offset_of!(__SlateRecord__functoolsmodule_c_146, r#fn) as u64) as i64;
        __t0.flags = 1i32;
        __t0.doc =
            (b"function object to use in future partial calls\0".as_ptr() as *mut i8) as *const i8;
        __t0
    },
    {
        let mut __t1: PyMemberDef = unsafe { core::mem::zeroed() };
        __t1.name = (b"args\0".as_ptr() as *mut i8) as *const i8;
        __t1.r#type = 6i32;
        __t1.offset =
            (core::mem::offset_of!(__SlateRecord__functoolsmodule_c_146, args) as u64) as i64;
        __t1.flags = 1i32;
        __t1.doc =
            (b"tuple of arguments to future partial calls\0".as_ptr() as *mut i8) as *const i8;
        __t1
    },
    {
        let mut __t2: PyMemberDef = unsafe { core::mem::zeroed() };
        __t2.name = (b"keywords\0".as_ptr() as *mut i8) as *const i8;
        __t2.r#type = 6i32;
        __t2.offset =
            (core::mem::offset_of!(__SlateRecord__functoolsmodule_c_146, kw) as u64) as i64;
        __t2.flags = 1i32;
        __t2.doc = (b"dictionary of keyword arguments to future partial calls\0".as_ptr()
            as *mut i8) as *const i8;
        __t2
    },
    {
        let mut __t3: PyMemberDef = unsafe { core::mem::zeroed() };
        __t3.name = (b"__weaklistoffset__\0".as_ptr() as *mut i8) as *const i8;
        __t3.r#type = 19i32;
        __t3.offset = (core::mem::offset_of!(__SlateRecord__functoolsmodule_c_146, weakreflist)
            as u64) as i64;
        __t3.flags = 1i32;
        __t3
    },
    {
        let mut __t4: PyMemberDef = unsafe { core::mem::zeroed() };
        __t4.name = (b"__dictoffset__\0".as_ptr() as *mut i8) as *const i8;
        __t4.r#type = 19i32;
        __t4.offset =
            (core::mem::offset_of!(__SlateRecord__functoolsmodule_c_146, dict) as u64) as i64;
        __t4.flags = 1i32;
        __t4
    },
    {
        let mut __t5: PyMemberDef = unsafe { core::mem::zeroed() };
        __t5.name = (b"__vectorcalloffset__\0".as_ptr() as *mut i8) as *const i8;
        __t5.r#type = 19i32;
        __t5.offset =
            (core::mem::offset_of!(__SlateRecord__functoolsmodule_c_146, vectorcall) as u64) as i64;
        __t5.flags = 1i32;
        __t5
    },
    {
        let mut __t6: PyMemberDef = unsafe { core::mem::zeroed() };
        __t6.name = core::ptr::null::<i8>();
        __t6
    },
]);

/// Sentinel
static mut partial_getsetlist: __SlateAlign16<[PyGetSetDef; 2]> = __SlateAlign16([
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
        __t1.name = core::ptr::null::<i8>();
        __t1
    },
]);

#[unsafe(link_section = ".text.slate_distinct._functoolsmodule.partial_repr")]
extern "C-unwind" fn partial_repr(self_: *mut _object) -> *mut _object {
    // Pack keyword arguments
    let mut error: i32 = unsafe { core::mem::zeroed() };
    let mut kw: *mut _object = unsafe { core::mem::zeroed() };
    let mut args: *mut _object = unsafe { core::mem::zeroed() };
    // Reference arguments in case they change
    let mut r#fn: *mut _object = unsafe { core::mem::zeroed() };
    let mut status: i32 = unsafe { core::mem::zeroed() };
    let mut value: *mut _object = unsafe { core::mem::zeroed() };
    let mut key: *mut _object = unsafe { core::mem::zeroed() };
    let mut n: i64 = unsafe { core::mem::zeroed() };
    let mut i: i64 = unsafe { core::mem::zeroed() };
    let mut name: *mut _object = unsafe { core::mem::zeroed() };
    let mut r#mod: *mut _object = unsafe { core::mem::zeroed() };
    let mut arglist: *mut _object = unsafe { core::mem::zeroed() };
    let mut result: *mut _object = unsafe { core::mem::zeroed() };
    let mut pto: *mut __SlateRecord__functoolsmodule_c_146 = unsafe { core::mem::zeroed() };
    unsafe {
        pto = partialobject_CAST!(self_);
        result = core::ptr::null_mut::<_object>();
        arglist = core::ptr::null_mut::<_object>();
        r#mod = core::ptr::null_mut::<_object>();
        name = core::ptr::null_mut::<_object>();
        status = Py_ReprEnter(self_);
        if status != 0 {
            if status < 0 {
                return core::ptr::null_mut::<_object>();
            } else {
                return PyUnicode_FromString(c"...".as_ptr());
            }
        } else {
            '__join_0: {
                r#fn = Py_NewRef!(unsafe { (*pto).r#fn });
                args = Py_NewRef!(unsafe { (*pto).args });
                kw = Py_NewRef!(unsafe { (*pto).kw });
                arglist = Py_GetConstant(7u32);
                if !arglist.is_null() {
                    // Pack positional arguments
                    n = PyTuple_GET_SIZE!(args);
                    i = 0;
                    while i < n {
                        Py_SETREF!(
                            arglist,
                            PyUnicode_FromFormat(
                                c"%U, %R".as_ptr(),
                                arglist,
                                PyTuple_GET_ITEM!(args, i)
                            )
                        );
                        if arglist.is_null() {
                            break '__join_0;
                        } else {
                            i = i.wrapping_add(1);
                        }
                    }
                    error = 0;
                    i = 0;
                    '__join_4: {
                        loop {
                            if PyDict_Next(
                                kw,
                                core::ptr::addr_of_mut!(i),
                                core::ptr::addr_of_mut!(key),
                                core::ptr::addr_of_mut!(value),
                            ) != 0
                            {
                                // Prevent key.__str__ from deleting the value.
                                Py_INCREF!(value);
                                Py_SETREF!(
                                    arglist,
                                    PyUnicode_FromFormat(
                                        c"%U, %S=%R".as_ptr(),
                                        arglist,
                                        key,
                                        value
                                    )
                                );
                                Py_DECREF!(value);
                                if arglist.is_null() {
                                    break;
                                }
                            } else {
                                break '__join_4;
                            }
                        }
                        error = 1;
                    }
                    if error == 0 {
                        r#mod = PyType_GetModuleName(Py_TYPE__2!(pto));
                        if !r#mod.is_null() {
                            name = PyType_GetQualName(Py_TYPE__2!(pto));
                            if !name.is_null() {
                                result = PyUnicode_FromFormat(
                                    c"%S.%S(%R%U)".as_ptr(),
                                    r#mod,
                                    name,
                                    r#fn,
                                    arglist,
                                );
                            }
                        }
                    }
                }
            }
            Py_XDECREF!(name);
            Py_XDECREF!(r#mod);
            Py_XDECREF!(arglist);
            Py_DECREF!(r#fn);
            Py_DECREF!(args);
            Py_DECREF!(kw);
            Py_ReprLeave(self_);
            return result;
        }
    }
}

// Pickle strategy:
// __reduce__ by itself doesn't support getting kwargs in the unpickle
// operation so we define a __setstate__ that replaces all the information
// about the partial.  If we only replaced part of it someone would use
// it as a hook to do strange things.
#[unsafe(link_section = ".text.slate_distinct._functoolsmodule.partial_reduce")]
extern "C-unwind" fn partial_reduce(
    self_: *mut _object,
    _unused_args: *mut _object,
) -> *mut _object {
    let pto: *mut __SlateRecord__functoolsmodule_c_146 = partialobject_CAST!(self_);
    unsafe {
        Py_BuildValue(
            c"O(O)(OOOO)".as_ptr(),
            Py_TYPE__2!(pto),
            (*pto).r#fn,
            (*pto).r#fn,
            (*pto).args,
            (*pto).kw,
            if !unsafe { (*pto).dict }.is_null() {
                (*pto).dict
            } else {
                core::ptr::addr_of_mut!(_Py_NoneStruct)
            },
        )
    }
}

#[unsafe(link_section = ".text.slate_distinct._functoolsmodule.partial_setstate")]
extern "C-unwind" fn partial_setstate(self_: *mut _object, state: *mut _object) -> *mut _object {
    let pto: *mut __SlateRecord__functoolsmodule_c_146 = partialobject_CAST!(self_);
    let mut r#fn: *mut _object = unsafe { core::mem::zeroed() };
    let mut fnargs: *mut _object = unsafe { core::mem::zeroed() };
    let mut kw: *mut _object = unsafe { core::mem::zeroed() };
    let mut dict: *mut _object = unsafe { core::mem::zeroed() };
    if PyTuple_Check!(state) == 0 {
        unsafe { PyErr_SetString(PyExc_TypeError, c"invalid partial state".as_ptr()) };
        return core::ptr::null_mut::<_object>();
    }
    let __v0: bool = (unsafe {
        PyArg_ParseTuple(
            state,
            c"OOOO".as_ptr(),
            core::ptr::addr_of_mut!(r#fn),
            core::ptr::addr_of_mut!(fnargs),
            core::ptr::addr_of_mut!(kw),
            core::ptr::addr_of_mut!(dict),
        )
    }) == 0
        || (unsafe { PyCallable_Check(r#fn) }) == 0;
    let __v1: bool = __v0 || PyTuple_Check!(fnargs) == 0;
    let __v2: bool =
        __v1 || kw != unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } && PyDict_Check!(kw) == 0;
    let __v4: bool = __v2
        || dict != unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } && PyDict_Check!(dict) == 0;
    if __v4 {
        unsafe { PyErr_SetString(PyExc_TypeError, c"invalid partial state".as_ptr()) };
        return core::ptr::null_mut::<_object>();
    }
    let nargs: i64 = PyTuple_GET_SIZE!(fnargs);
    let __v6: bool = nargs != 0
        && PyTuple_GET_ITEM!(fnargs, nargs.wrapping_sub(1)) == unsafe { (*pto).placeholder };
    if __v6 {
        unsafe {
            PyErr_SetString(
                PyExc_TypeError,
                c"trailing Placeholders are not allowed".as_ptr(),
            )
        };
        return core::ptr::null_mut::<_object>();
    }
    // Count placeholders
    let mut phcount: i64 = 0;
    for i in 0..nargs.wrapping_sub(1) {
        if PyTuple_GET_ITEM!(fnargs, i) == unsafe { (*pto).placeholder } {
            phcount = phcount.wrapping_add(1);
        }
    }
    if PyTuple_CheckExact!(fnargs) == 0 {
        fnargs = unsafe { PySequence_Tuple(fnargs) };
    } else {
        Py_INCREF!(fnargs);
    }
    if fnargs.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    if kw == unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } {
        kw = unsafe { PyDict_New() };
    } else {
        if PyDict_CheckExact!(kw) == 0 {
            kw = unsafe { PyDict_Copy(kw) };
        } else {
            Py_INCREF!(kw);
        }
    }
    if kw.is_null() {
        Py_DECREF!(fnargs);
        return core::ptr::null_mut::<_object>();
    }
    if dict == unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } {
        dict = core::ptr::null_mut::<_object>();
    } else {
        Py_INCREF!(dict);
    }
    Py_SETREF__2!((*pto).r#fn, Py_NewRef!(r#fn));
    Py_SETREF__2!((*pto).args, fnargs);
    Py_SETREF__2!((*pto).kw, kw);
    unsafe {
        (*pto).phcount = phcount;
    }
    Py_XSETREF!((*pto).dict, dict);
    partial_setvectorcall(pto);
    unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
}

/// sentinel
static mut partial_methods: __SlateAlign16<[PyMethodDef; 4]> = __SlateAlign16([
    {
        let mut __t0: PyMethodDef = unsafe { core::mem::zeroed() };
        __t0.ml_name = (b"__reduce__\0".as_ptr() as *mut i8) as *const i8;
        __t0.ml_meth = Some(partial_reduce);
        __t0.ml_flags = 4i32;
        __t0
    },
    {
        let mut __t1: PyMethodDef = unsafe { core::mem::zeroed() };
        __t1.ml_name = (b"__setstate__\0".as_ptr() as *mut i8) as *const i8;
        __t1.ml_meth = Some(partial_setstate);
        __t1.ml_flags = 8i32;
        __t1
    },
    {
        let mut __t2: PyMethodDef = unsafe { core::mem::zeroed() };
        __t2.ml_name = (b"__class_getitem__\0".as_ptr() as *mut i8) as *const i8;
        __t2.ml_meth = unsafe {
            core::mem::transmute::<
                *const (),
                Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
            >(Py_GenericAlias as *const ())
        };
        __t2.ml_flags = 24i32;
        __t2.ml_doc = (b"partial is generic over the wrapped function's return type\0".as_ptr()
            as *mut i8) as *const i8;
        __t2
    },
    {
        let mut __t3: PyMethodDef = unsafe { core::mem::zeroed() };
        __t3.ml_name = core::ptr::null::<i8>();
        __t3.ml_meth = None;
        __t3
    },
]);

static mut partial_type_slots: __SlateAlign16<[__SlateRecord_object_h_377; 15]> = __SlateAlign16([
    {
        let mut __t0: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t0.slot = 52i32;
        __t0.pfunc = unsafe {
            core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut _object)>, *mut ()>(
                Some(partial_dealloc),
            )
        };
        __t0
    },
    {
        let mut __t1: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t1.slot = 66i32;
        __t1.pfunc = unsafe {
            core::mem::transmute::<
                Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
                *mut (),
            >(Some(partial_repr))
        };
        __t1
    },
    {
        let mut __t2: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t2.slot = 50i32;
        __t2.pfunc = unsafe {
            core::mem::transmute::<
                Option<
                    unsafe extern "C-unwind" fn(
                        *mut _object,
                        *mut _object,
                        *mut _object,
                    ) -> *mut _object,
                >,
                *mut (),
            >(Some(partial_call))
        };
        __t2
    },
    {
        let mut __t3: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t3.slot = 58i32;
        __t3.pfunc = unsafe {
            core::mem::transmute::<
                Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
                *mut (),
            >(unsafe {
                core::mem::transmute::<
                    *const (),
                    Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
                >(PyObject_GenericGetAttr as *const ())
            })
        };
        __t3
    },
    {
        let mut __t4: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t4.slot = 69i32;
        __t4.pfunc = unsafe {
            core::mem::transmute::<
                Option<
                    unsafe extern "C-unwind" fn(*mut _object, *mut _object, *mut _object) -> i32,
                >,
                *mut (),
            >(unsafe {
                core::mem::transmute::<
                    *const (),
                    Option<
                        unsafe extern "C-unwind" fn(
                            *mut _object,
                            *mut _object,
                            *mut _object,
                        ) -> i32,
                    >,
                >(PyObject_GenericSetAttr as *const ())
            })
        };
        __t4
    },
    {
        let mut __t5: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t5.slot = 56i32;
        __t5.pfunc = (unsafe { core::ptr::addr_of!(partial_doc.0) as *const i8 }) as *mut ();
        __t5
    },
    {
        let mut __t6: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t6.slot = 71i32;
        __t6.pfunc = unsafe {
            core::mem::transmute::<
                Option<
                    unsafe extern "C-unwind" fn(
                        *mut _object,
                        Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
                        *mut (),
                    ) -> i32,
                >,
                *mut (),
            >(Some(partial_traverse))
        };
        __t6
    },
    {
        let mut __t7: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t7.slot = 51i32;
        __t7.pfunc = unsafe {
            core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut _object) -> i32>, *mut ()>(
                Some(partial_clear),
            )
        };
        __t7
    },
    {
        let mut __t8: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t8.slot = 64i32;
        __t8.pfunc =
            (unsafe { core::ptr::addr_of_mut!(partial_methods.0) as *mut PyMethodDef }) as *mut ();
        __t8
    },
    {
        let mut __t9: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t9.slot = 72i32;
        __t9.pfunc = (unsafe { core::ptr::addr_of_mut!(partial_memberlist.0) as *mut PyMemberDef })
            as *mut ();
        __t9
    },
    {
        let mut __t10: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t10.slot = 73i32;
        __t10.pfunc = (unsafe { core::ptr::addr_of_mut!(partial_getsetlist.0) as *mut PyGetSetDef })
            as *mut ();
        __t10
    },
    {
        let mut __t11: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t11.slot = 54i32;
        __t11.pfunc = unsafe {
            core::mem::transmute::<
                Option<
                    unsafe extern "C-unwind" fn(
                        *mut _object,
                        *mut _object,
                        *mut _object,
                    ) -> *mut _object,
                >,
                *mut (),
            >(Some(partial_descr_get))
        };
        __t11
    },
    {
        let mut __t12: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t12.slot = 65i32;
        __t12.pfunc = unsafe {
            core::mem::transmute::<
                Option<
                    unsafe extern "C-unwind" fn(
                        *mut _typeobject,
                        *mut _object,
                        *mut _object,
                    ) -> *mut _object,
                >,
                *mut (),
            >(Some(partial_new))
        };
        __t12
    },
    {
        let mut __t13: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t13.slot = 74i32;
        __t13.pfunc = unsafe {
            core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut ())>, *mut ()>(unsafe {
                core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
                    PyObject_GC_Del as *const (),
                )
            })
        };
        __t13
    },
    {
        let mut __t14: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
        __t14.slot = 0i32;
        __t14.pfunc = core::ptr::null_mut::<()>();
        __t14
    },
]);

static mut partial_type_spec: __SlateRecord_object_h_385 = {
    let mut __t0: __SlateRecord_object_h_385 = unsafe { core::mem::zeroed() };
    __t0.name = (b"functools.partial\0".as_ptr() as *mut i8) as *const i8;
    __t0.basicsize = (core::mem::size_of::<__SlateRecord__functoolsmodule_c_146>() as u64) as i32;
    __t0.flags = 19712u32;
    __t0.slots =
        unsafe { core::ptr::addr_of_mut!(partial_type_slots.0) as *mut __SlateRecord_object_h_377 };
    __t0
};

// cmp_to_key
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord__functoolsmodule_c_813 {
    ob_base: _object,
    cmp: *mut _object,
    object: *mut _object,
}

#[unsafe(link_section = ".text.slate_distinct._functoolsmodule.keyobject_clear")]
extern "C-unwind" fn keyobject_clear(op: *mut _object) -> i32 {
    let ko: *mut __SlateRecord__functoolsmodule_c_813 = keyobject_CAST!(op);
    Py_CLEAR!((*ko).cmp);
    Py_CLEAR!((*ko).object);
    0
}

#[unsafe(link_section = ".text.slate_distinct._functoolsmodule.keyobject_dealloc")]
extern "C-unwind" fn keyobject_dealloc(ko: *mut _object) {
    let tp: *mut _typeobject = Py_TYPE!(ko);
    unsafe { PyObject_GC_UnTrack(ko as *mut ()) };
    keyobject_clear(ko);
    unsafe { unsafe { (*tp).tp_free }.unwrap()(ko as *mut ()) };
    Py_DECREF__2!(tp);
}

#[unsafe(link_section = ".text.slate_distinct._functoolsmodule.keyobject_traverse")]
extern "C-unwind" fn keyobject_traverse(
    op: *mut _object,
    mut visit: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
    arg: *mut (),
) -> i32 {
    let ko: *mut __SlateRecord__functoolsmodule_c_813 = keyobject_CAST!(op);
    Py_VISIT__2!(_Py_TYPE(ko as *mut _object), visit, arg);
    Py_VISIT!(unsafe { (*ko).cmp }, visit, arg);
    Py_VISIT!(unsafe { (*ko).object }, visit, arg);
    0
}

static mut keyobject_members: __SlateAlign16<[PyMemberDef; 2]> = __SlateAlign16([
    {
        let mut __t0: PyMemberDef = unsafe { core::mem::zeroed() };
        __t0.name = (b"obj\0".as_ptr() as *mut i8) as *const i8;
        __t0.r#type = 6i32;
        __t0.offset =
            (core::mem::offset_of!(__SlateRecord__functoolsmodule_c_813, object) as u64) as i64;
        __t0.flags = 0i32;
        __t0.doc = (b"Value wrapped by a key function.\0".as_ptr() as *mut i8) as *const i8;
        __t0
    },
    {
        let mut __t1: PyMemberDef = unsafe { core::mem::zeroed() };
        __t1.name = core::ptr::null::<i8>();
        __t1
    },
]);

#[unsafe(link_section = ".text.slate_distinct._functoolsmodule.keyobject_text_signature")]
extern "C-unwind" fn keyobject_text_signature(
    _unused_self: *mut _object,
    _unused_ignored: *mut (),
) -> *mut _object {
    unsafe { PyUnicode_FromString(c"(obj)".as_ptr()) }
}

static mut keyobject_getset: __SlateAlign16<[PyGetSetDef; 2]> = __SlateAlign16([
    {
        let mut __t0: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t0.name = (b"__text_signature__\0".as_ptr() as *mut i8) as *const i8;
        __t0.get = Some(keyobject_text_signature);
        __t0.set = None;
        __t0
    },
    {
        let mut __t1: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t1.name = core::ptr::null::<i8>();
        __t1
    },
]);

static mut keyobject_type_slots: __SlateAlign16<[__SlateRecord_object_h_377; 9]> =
    __SlateAlign16([
        {
            let mut __t0: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
            __t0.slot = 52i32;
            __t0.pfunc = unsafe {
                core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut _object)>, *mut ()>(
                    Some(keyobject_dealloc),
                )
            };
            __t0
        },
        {
            let mut __t1: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
            __t1.slot = 50i32;
            __t1.pfunc = unsafe {
                core::mem::transmute::<
                    Option<
                        unsafe extern "C-unwind" fn(
                            *mut _object,
                            *mut _object,
                            *mut _object,
                        ) -> *mut _object,
                    >,
                    *mut (),
                >(Some(keyobject_call))
            };
            __t1
        },
        {
            let mut __t2: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
            __t2.slot = 58i32;
            __t2.pfunc = unsafe {
                core::mem::transmute::<
                    Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
                    *mut (),
                >(unsafe {
                    core::mem::transmute::<
                        *const (),
                        Option<
                            unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object,
                        >,
                    >(PyObject_GenericGetAttr as *const ())
                })
            };
            __t2
        },
        {
            let mut __t3: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
            __t3.slot = 71i32;
            __t3.pfunc = unsafe {
                core::mem::transmute::<
                    Option<
                        unsafe extern "C-unwind" fn(
                            *mut _object,
                            Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
                            *mut (),
                        ) -> i32,
                    >,
                    *mut (),
                >(Some(keyobject_traverse))
            };
            __t3
        },
        {
            let mut __t4: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
            __t4.slot = 51i32;
            __t4.pfunc = unsafe {
                core::mem::transmute::<
                    Option<unsafe extern "C-unwind" fn(*mut _object) -> i32>,
                    *mut (),
                >(Some(keyobject_clear))
            };
            __t4
        },
        {
            let mut __t5: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
            __t5.slot = 67i32;
            __t5.pfunc = unsafe {
                core::mem::transmute::<
                    Option<
                        unsafe extern "C-unwind" fn(
                            *mut _object,
                            *mut _object,
                            i32,
                        ) -> *mut _object,
                    >,
                    *mut (),
                >(Some(keyobject_richcompare))
            };
            __t5
        },
        {
            let mut __t6: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
            __t6.slot = 72i32;
            __t6.pfunc =
                (unsafe { core::ptr::addr_of_mut!(keyobject_members.0) as *mut PyMemberDef })
                    as *mut ();
            __t6
        },
        {
            let mut __t7: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
            __t7.slot = 73i32;
            __t7.pfunc =
                (unsafe { core::ptr::addr_of_mut!(keyobject_getset.0) as *mut PyGetSetDef })
                    as *mut ();
            __t7
        },
        {
            let mut __t8: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
            __t8.slot = 0i32;
            __t8.pfunc = core::ptr::null_mut::<()>();
            __t8
        },
    ]);

static mut keyobject_type_spec: __SlateRecord_object_h_385 = {
    let mut __t0: __SlateRecord_object_h_385 = unsafe { core::mem::zeroed() };
    __t0.name = (b"functools.KeyWrapper\0".as_ptr() as *mut i8) as *const i8;
    __t0.basicsize = (core::mem::size_of::<__SlateRecord__functoolsmodule_c_813>() as u64) as i32;
    __t0.flags = 16768u32;
    __t0.slots = unsafe {
        core::ptr::addr_of_mut!(keyobject_type_slots.0) as *mut __SlateRecord_object_h_377
    };
    __t0
};

#[unsafe(link_section = ".text.slate_distinct._functoolsmodule.keyobject_call")]
extern "C-unwind" fn keyobject_call(
    self_: *mut _object,
    args: *mut _object,
    kwds: *mut _object,
) -> *mut _object {
    let mut object: *mut _object = unsafe { core::mem::zeroed() };
    let ko: *mut __SlateRecord__functoolsmodule_c_813 = keyobject_CAST!(self_);
    if (unsafe {
        PyArg_ParseTupleAndKeywords(
            args,
            kwds,
            c"O:K".as_ptr(),
            (core::ptr::addr_of_mut!(kwargs.0) as *mut *mut i8) as *const *mut i8,
            core::ptr::addr_of_mut!(object),
        )
    }) == 0
    {
        return core::ptr::null_mut::<_object>();
    }
    let result: *mut __SlateRecord__functoolsmodule_c_813 =
        PyObject_GC_New!(__SlateRecord__functoolsmodule_c_813, Py_TYPE__2!(ko));
    if result.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    unsafe {
        (*result).cmp = Py_NewRef!(unsafe { (*ko).cmp });
    }
    unsafe {
        (*result).object = Py_NewRef!(object);
    }
    unsafe { PyObject_GC_Track(result as *mut ()) };
    result as *mut _object
}

static mut kwargs: __SlateAlign16<[*mut i8; 2]> =
    __SlateAlign16([b"obj\0".as_ptr() as *mut i8, core::ptr::null_mut::<i8>()]);

#[unsafe(link_section = ".text.slate_distinct._functoolsmodule.keyobject_richcompare")]
extern "C-unwind" fn keyobject_richcompare(
    self_: *mut _object,
    other: *mut _object,
    op: i32,
) -> *mut _object {
    if Py_IS_TYPE!(other, Py_TYPE!(self_)) == 0 {
        unsafe {
            PyErr_Format(
                PyExc_TypeError,
                c"other argument must be K instance".as_ptr(),
            )
        };
        return core::ptr::null_mut::<_object>();
    }
    let lhs: *mut __SlateRecord__functoolsmodule_c_813 = keyobject_CAST!(self_);
    let rhs: *mut __SlateRecord__functoolsmodule_c_813 = keyobject_CAST!(other);
    let compare: *mut _object = unsafe { (*lhs).cmp };
    let x: *mut _object = unsafe { (*lhs).object };
    let y: *mut _object = unsafe { (*rhs).object };
    if x.is_null() || y.is_null() {
        unsafe { PyErr_Format(PyExc_AttributeError, c"object".as_ptr()) };
        return core::ptr::null_mut::<_object>();
    }
    // Call the user's comparison function and translate the 3-way
    // result into true or false (or error).
    let mut args: __SlateAlign16<[*mut _object; 2]> = __SlateAlign16([x, y]);
    let res: *mut _object = unsafe {
        PyObject_Vectorcall(
            compare,
            args.0.as_mut_ptr() as *const *mut _object,
            2u64,
            core::ptr::null_mut::<_object>(),
        )
    };
    if res.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let answer: *mut _object = unsafe { PyObject_RichCompare(res, _PyLong_GetZero(), op) };
    Py_DECREF!(res);
    answer
}

// [clinic input]
// _functools.cmp_to_key
//
//     mycmp: object
//         Function that compares two objects.
//
// Convert a cmp= function into a key= function.
// [clinic start generated code]
/// [clinic end generated code: output=71eaad0f4fc81f33 input=d1b76f231c0dfeb3]
fn _functools_cmp_to_key_impl(module: *mut _object, mycmp: *mut _object) -> *mut _object {
    let state: *mut _functools_state = get_functools_state(module);
    let object: *mut __SlateRecord__functoolsmodule_c_813 =
        PyObject_GC_New!(__SlateRecord__functoolsmodule_c_813, unsafe {
            (*state).keyobject_type
        });
    if object.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    unsafe {
        (*object).cmp = Py_NewRef!(mycmp);
    }
    unsafe {
        (*object).object = core::ptr::null_mut::<_object>();
    }
    unsafe { PyObject_GC_Track(object as *mut ()) };
    object as *mut _object
}

// reduce (used to be a builtin)
// [clinic input]
// _functools.reduce
//
//     function as func: object
//     iterable as seq: object
//     /
//     initial as result: object = NULL
//
// Apply a function of two arguments cumulatively to the items of an iterable, from left to right.
//
// This effectively reduces the iterable to a single value.  If initial is
// present, it is placed before the items of the iterable in the
// calculation, and serves as a default when the iterable is empty.
//
// For example, reduce(lambda x, y: x+y, [1, 2, 3, 4, 5])
// calculates ((((1 + 2) + 3) + 4) + 5).
// [clinic start generated code]
/// [clinic end generated code: output=30d898fe1267c79d input=1e2c850f5229ff2a]
fn _functools_reduce_impl(
    module: *mut _object,
    func: *mut _object,
    seq: *mut _object,
    mut result: *mut _object,
) -> *mut _object {
    let mut op2: *mut _object = unsafe { core::mem::zeroed() };
    let mut it: *mut _object = unsafe { core::mem::zeroed() };
    let mut args: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        if !result.is_null() {
            Py_INCREF!(result);
        }
        it = PyObject_GetIter(seq);
        if it.is_null() {
            if PyErr_ExceptionMatches(PyExc_TypeError) != 0 {
                PyErr_SetString(
                    PyExc_TypeError,
                    c"reduce() arg 2 must support iteration".as_ptr(),
                );
            }
            Py_XDECREF!(result);
            return core::ptr::null_mut::<_object>();
        } else {
            '__join_0: {
                let __v3: *mut _object = PyTuple_New(2i64);
                args = __v3;
                if !__v3.is_null() {
                    loop {
                        if _PyObject_IsUniquelyReferenced(args) == 0 {
                            Py_DECREF!(args);
                            let __v1: *mut _object = PyTuple_New(2i64);
                            args = __v1;
                            if __v1.is_null() {
                                break '__join_0;
                            }
                        }
                        op2 = PyIter_Next(it);
                        if op2.is_null() {
                            break;
                        } else {
                            if result.is_null() {
                                result = op2;
                            } else {
                                // Update the args tuple in-place
                                Py_XSETREF__4!(
                                    *((*(args as *mut __SlateRecord_tupleobject_h_13))
                                        .ob_item
                                        .as_mut_ptr()
                                        as *mut *mut _object)
                                        .wrapping_offset(0 as isize),
                                    result
                                );
                                Py_XSETREF__4!(
                                    *((*(args as *mut __SlateRecord_tupleobject_h_13))
                                        .ob_item
                                        .as_mut_ptr()
                                        as *mut *mut _object)
                                        .wrapping_offset(1 as isize),
                                    op2
                                );
                                let __v0: *mut _object =
                                    PyObject_Call(func, args, core::ptr::null_mut::<_object>());
                                result = __v0;
                                if __v0.is_null() {
                                    break '__join_0;
                                } else {
                                    // bpo-42536: The GC may have untracked this args tuple. Since we're
                                    //
                                    // recycling it, make sure it's tracked again:
                                    _PyTuple_Recycle(args);
                                }
                            }
                        }
                    }
                    if PyErr_Occurred().is_null() {
                        Py_DECREF!(args);
                        if result.is_null() {
                            PyErr_SetString(
                                PyExc_TypeError,
                                c"reduce() of empty iterable with no initial value".as_ptr(),
                            );
                        }
                        Py_DECREF!(it);
                        return result;
                    }
                }
            }
            Py_XDECREF!(args);
            Py_XDECREF!(result);
            Py_DECREF!(it);
            return core::ptr::null_mut::<_object>();
        }
    }
}

// lru_cache object
// There are four principal algorithmic differences from the pure python version:
//
// 1). The C version relies on the GIL instead of having its own reentrant lock.
//
// 2). The prev/next link fields use borrowed references.
//
// 3). For a full cache, the pure python version rotates the location of the
//     root entry so that it never has to move individual links and it can
//     limit updates to just the key and result fields.  However, in the C
//     version, links are temporarily removed while the cache dict updates are
//     occurring. Afterwards, they are appended or prepended back into the
//     doubly-linked lists.
//
// 4)  In the Python version, the _HashSeq class is used to prevent __hash__
//     from being called more than once.  In the C version, the "known hash"
//     variants of dictionary calls as used to the same effect.
#[repr(C)]
#[derive(Clone, Copy)]
struct lru_list_elem {
    ob_base: _object,
    prev: *mut lru_list_elem,
    /// borrowed links
    next: *mut lru_list_elem,
    hash: i64,
    key: *mut _object,
    result: *mut _object,
}

#[unsafe(link_section = ".text.slate_distinct._functoolsmodule.lru_list_elem_dealloc")]
extern "C-unwind" fn lru_list_elem_dealloc(op: *mut _object) {
    let link: *mut lru_list_elem = lru_list_elem_CAST!(op);
    let tp: *mut _typeobject = Py_TYPE__2!(link);
    Py_XDECREF!(unsafe { (*link).key });
    Py_XDECREF!(unsafe { (*link).result });
    unsafe { unsafe { (*tp).tp_free }.unwrap()(link as *mut ()) };
    Py_DECREF__2!(tp);
}

static mut lru_list_elem_type_slots: __SlateAlign16<[__SlateRecord_object_h_377; 2]> =
    __SlateAlign16([
        {
            let mut __t0: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
            __t0.slot = 52i32;
            __t0.pfunc = unsafe {
                core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut _object)>, *mut ()>(
                    Some(lru_list_elem_dealloc),
                )
            };
            __t0
        },
        {
            let mut __t1: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
            __t1.slot = 0i32;
            __t1.pfunc = core::ptr::null_mut::<()>();
            __t1
        },
    ]);

static mut lru_list_elem_type_spec: __SlateRecord_object_h_385 = {
    let mut __t0: __SlateRecord_object_h_385 = unsafe { core::mem::zeroed() };
    __t0.name = (b"functools._lru_list_elem\0".as_ptr() as *mut i8) as *const i8;
    __t0.basicsize = (core::mem::size_of::<lru_list_elem>() as u64) as i32;
    __t0.flags = 384u32;
    __t0.slots = unsafe {
        core::ptr::addr_of_mut!(lru_list_elem_type_slots.0) as *mut __SlateRecord_object_h_377
    };
    __t0
};

#[repr(C)]
#[derive(Clone, Copy)]
struct lru_cache_object {
    /// includes PyObject_HEAD
    root: lru_list_elem,
    wrapper: Option<
        unsafe extern "C-unwind" fn(
            *mut lru_cache_object,
            *mut _object,
            *mut _object,
        ) -> *mut _object,
    >,
    typed: i32,
    cache: *mut _object,
    hits: i64,
    func: *mut _object,
    maxsize: i64,
    misses: i64,
    /// the kwd_mark is used delimit args and keywords in the cache keys
    kwd_mark: *mut _object,
    lru_list_elem_type: *mut _typeobject,
    cache_info_type: *mut _object,
    dict: *mut _object,
    weakreflist: *mut _object,
}

fn lru_cache_make_key(
    kwd_mark: *mut _object,
    args: *mut _object,
    kwds: *mut _object,
    typed: i32,
) -> *mut _object {
    let mut key: *mut _object = unsafe { core::mem::zeroed() };
    let mut keyword: *mut _object = unsafe { core::mem::zeroed() };
    let mut value: *mut _object = unsafe { core::mem::zeroed() };
    let mut key_size: i64 = 0;
    let mut pos: i64 = 0;
    let mut key_pos: i64 = 0;
    let mut kwds_size: i64 = 0;
    let __v0: i64 = if !kwds.is_null() {
        PyDict_GET_SIZE!(kwds)
    } else {
        0
    };
    kwds_size = __v0;
    // short path, key will match args anyway, which is a tuple
    if typed == 0 && kwds_size == 0 {
        if PyTuple_GET_SIZE!(args) == 1 {
            key = PyTuple_GET_ITEM!(args, 0i32);
            let __v1: bool = PyUnicode_CheckExact!(key) != 0 || PyLong_CheckExact!(key) != 0;
            if __v1 {
                // For common scalar keys, save space by
                // dropping the enclosing args tuple
                return Py_NewRef!(key);
            }
        }
        return Py_NewRef!(args);
    }
    key_size = PyTuple_GET_SIZE!(args);
    if kwds_size != 0 {
        key_size = key_size.wrapping_add(kwds_size.wrapping_mul(2).wrapping_add(1));
    }
    if typed != 0 {
        let __v4: i64 = PyTuple_GET_SIZE!(args).wrapping_add(kwds_size);
        key_size = key_size.wrapping_add(__v4);
    }
    key = unsafe { PyTuple_New(key_size) };
    if key.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    while pos < PyTuple_GET_SIZE!(args) {
        let item: *mut _object = PyTuple_GET_ITEM!(args, pos);
        let __v7: i64 = key_pos;
        key_pos = __v7.wrapping_add(1);
        PyTuple_SET_ITEM(
            _PyObject_CAST!(key),
            __v7,
            _PyObject_CAST!(Py_NewRef!(item)),
        );
        pos = pos.wrapping_add(1);
    }
    if kwds_size != 0 {
        let __v12: i64 = key_pos;
        key_pos = __v12.wrapping_add(1);
        PyTuple_SET_ITEM(
            _PyObject_CAST!(key),
            __v12,
            _PyObject_CAST!(Py_NewRef!(kwd_mark)),
        );
        pos = 0;
        while (unsafe {
            PyDict_Next(
                kwds,
                core::ptr::addr_of_mut!(pos),
                core::ptr::addr_of_mut!(keyword),
                core::ptr::addr_of_mut!(value),
            )
        }) != 0
        {
            let __v14: i64 = key_pos;
            key_pos = __v14.wrapping_add(1);
            PyTuple_SET_ITEM(
                _PyObject_CAST!(key),
                __v14,
                _PyObject_CAST!(Py_NewRef!(keyword)),
            );
            let __v16: i64 = key_pos;
            key_pos = __v16.wrapping_add(1);
            PyTuple_SET_ITEM(
                _PyObject_CAST!(key),
                __v16,
                _PyObject_CAST!(Py_NewRef!(value)),
            );
        }
    }
    if typed != 0 {
        pos = 0;
        while pos < PyTuple_GET_SIZE!(args) {
            let item: *mut _object = Py_TYPE!(PyTuple_GET_ITEM!(args, pos)) as *mut _object;
            let __v19: i64 = key_pos;
            key_pos = __v19.wrapping_add(1);
            PyTuple_SET_ITEM(
                _PyObject_CAST!(key),
                __v19,
                _PyObject_CAST!(Py_NewRef!(item)),
            );
            pos = pos.wrapping_add(1);
        }
        if kwds_size != 0 {
            pos = 0;
            while (unsafe {
                PyDict_Next(
                    kwds,
                    core::ptr::addr_of_mut!(pos),
                    core::ptr::addr_of_mut!(keyword),
                    core::ptr::addr_of_mut!(value),
                )
            }) != 0
            {
                let item: *mut _object = Py_TYPE!(value) as *mut _object;
                let __v24: i64 = key_pos;
                key_pos = __v24.wrapping_add(1);
                PyTuple_SET_ITEM(
                    _PyObject_CAST!(key),
                    __v24,
                    _PyObject_CAST!(Py_NewRef!(item)),
                );
            }
        }
    }
    key
}

#[unsafe(link_section = ".text.slate_distinct._functoolsmodule.uncached_lru_cache_wrapper")]
extern "C-unwind" fn uncached_lru_cache_wrapper(
    self_: *mut lru_cache_object,
    args: *mut _object,
    kwds: *mut _object,
) -> *mut _object {
    let __v1: i64 = unsafe { (*self_).misses };
    unsafe {
        (*self_).misses = __v1.wrapping_add(1);
    }
    let result: *mut _object = unsafe { PyObject_Call((*self_).func, args, kwds) };
    if result.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    result
}

#[unsafe(link_section = ".text.slate_distinct._functoolsmodule.infinite_lru_cache_wrapper")]
extern "C-unwind" fn infinite_lru_cache_wrapper(
    self_: *mut lru_cache_object,
    args: *mut _object,
    kwds: *mut _object,
) -> *mut _object {
    let mut result: *mut _object = unsafe { core::mem::zeroed() };
    let mut hash: i64 = 0;
    let key: *mut _object = lru_cache_make_key(unsafe { (*self_).kwd_mark }, args, kwds, unsafe {
        (*self_).typed
    });
    if key.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    hash = unsafe { PyObject_Hash(key) };
    if hash == (1i32.wrapping_neg() as i64) {
        Py_DECREF!(key);
        return core::ptr::null_mut::<_object>();
    }
    let res: i32 = unsafe {
        _PyDict_GetItemRef_KnownHash(
            (*self_).cache as *mut __SlateRecord_dictobject_h_33,
            key,
            hash,
            core::ptr::addr_of_mut!(result),
        )
    };
    if res > 0 {
        let __v1: i64 = unsafe { (*self_).hits };
        unsafe {
            (*self_).hits = __v1.wrapping_add(1);
        }
        Py_DECREF!(key);
        return result;
    }
    if res < 0 {
        Py_DECREF!(key);
        return core::ptr::null_mut::<_object>();
    }
    let __v4: i64 = unsafe { (*self_).misses };
    unsafe {
        (*self_).misses = __v4.wrapping_add(1);
    }
    result = unsafe { PyObject_Call((*self_).func, args, kwds) };
    if result.is_null() {
        Py_DECREF!(key);
        return core::ptr::null_mut::<_object>();
    }
    if (unsafe { _PyDict_SetItem_KnownHash((*self_).cache, key, result, hash) }) < 0 {
        Py_DECREF!(result);
        Py_DECREF!(key);
        return core::ptr::null_mut::<_object>();
    }
    Py_DECREF!(key);
    result
}

fn lru_cache_extract_link(link: *mut lru_list_elem) {
    let link_prev: *mut lru_list_elem = unsafe { (*link).prev };
    let link_next: *mut lru_list_elem = unsafe { (*link).next };
    unsafe {
        (*link_prev).next = (*link).next;
    }
    unsafe {
        (*link_next).prev = (*link).prev;
    }
}

fn lru_cache_append_link(self_: *mut lru_cache_object, link: *mut lru_list_elem) {
    let root: *mut lru_list_elem = unsafe { core::ptr::addr_of_mut!((*self_).root) };
    let last: *mut lru_list_elem = unsafe { (*root).prev };
    let __v0: *mut lru_list_elem = link;
    unsafe {
        (*root).prev = __v0;
    }
    unsafe {
        (*last).next = __v0;
    }
    unsafe {
        (*link).prev = last;
    }
    unsafe {
        (*link).next = root;
    }
}

fn lru_cache_prepend_link(self_: *mut lru_cache_object, link: *mut lru_list_elem) {
    let root: *mut lru_list_elem = unsafe { core::ptr::addr_of_mut!((*self_).root) };
    let first: *mut lru_list_elem = unsafe { (*root).next };
    let __v0: *mut lru_list_elem = link;
    unsafe {
        (*root).next = __v0;
    }
    unsafe {
        (*first).prev = __v0;
    }
    unsafe {
        (*link).prev = root;
    }
    unsafe {
        (*link).next = first;
    }
}

// General note on reentrancy:
//
// There are four dictionary calls in the bounded_lru_cache_wrapper():
// 1) The initial check for a cache match.  2) The post user-function
// check for a cache match.  3) The deletion of the oldest entry.
// 4) The addition of the newest entry.
//
// In all four calls, we have a known hash which lets use avoid a call
// to __hash__().  That leaves only __eq__ as a possible source of a
// reentrant call.
//
// The __eq__ method call is always made for a cache hit (dict access #1).
// Accordingly, we have make sure not modify the cache state prior to
// this call.
//
// The __eq__ method call is never made for the deletion (dict access #3)
// because it is an identity match.
//
// For the other two accesses (#2 and #4), calls to __eq__ only occur
// when some other entry happens to have an exactly matching hash (all
// 64-bits).  Though rare, this can happen, so we have to make sure to
// either call it at the top of its code path before any cache
// state modifications (dict access #2) or be prepared to restore
// invariants at the end of the code path (dict access #4).
//
// Another possible source of reentrancy is a decref which can trigger
// arbitrary code execution.  To make the code easier to reason about,
// the decrefs are deferred to the end of the each possible code path
// so that we know the cache is a consistent state.
fn bounded_lru_cache_get_lock_held(
    self_: *mut lru_cache_object,
    args: *mut _object,
    kwds: *mut _object,
    result: &mut *mut _object,
    key: &mut *mut _object,
    hash: &mut i64,
) -> i32 {
    let mut link: *mut lru_list_elem = unsafe { core::mem::zeroed() };
    let __v0: *mut _object = lru_cache_make_key(unsafe { (*self_).kwd_mark }, args, kwds, unsafe {
        (*self_).typed
    });
    unsafe {
        *(key as *mut *mut _object) = __v0;
    }
    let key_: *mut _object = __v0;
    if key_.is_null() {
        return 1i32.wrapping_neg();
    }
    let mut hash_: i64 = 0;
    let __v1: i64 = unsafe { PyObject_Hash(key_) };
    unsafe {
        *(hash as *mut i64) = __v1;
    }
    hash_ = __v1;
    if hash_ == (1i32.wrapping_neg() as i64) {
        Py_DECREF!(key_); // dead reference left in *key, is not used
        return 1i32.wrapping_neg();
    }
    let res: i32 = unsafe {
        _PyDict_GetItemRef_KnownHash_LockHeld(
            (*self_).cache as *mut __SlateRecord_dictobject_h_33,
            key_,
            hash_,
            core::ptr::addr_of_mut!(link) as *mut *mut _object,
        )
    };
    if res > 0 {
        lru_cache_extract_link(link);
        lru_cache_append_link(self_, link);
        unsafe {
            *(result as *mut *mut _object) = (*link).result;
        }
        let __v3: i64 = unsafe { (*self_).hits };
        unsafe {
            (*self_).hits = __v3.wrapping_add(1);
        }
        Py_INCREF!(unsafe { (*link).result });
        Py_DECREF__2!(link);
        Py_DECREF!(key_);
        return 1;
    }
    if res < 0 {
        Py_DECREF!(key_);
        return 1i32.wrapping_neg();
    }
    let __v6: i64 = unsafe { (*self_).misses };
    unsafe {
        (*self_).misses = __v6.wrapping_add(1);
    }
    0
}

fn bounded_lru_cache_update_lock_held(
    self_: *mut lru_cache_object,
    result: *mut _object,
    key: *mut _object,
    hash: i64,
) -> *mut _object {
    let mut link: *mut lru_list_elem = unsafe { core::mem::zeroed() };
    let mut testresult: *mut _object = unsafe { core::mem::zeroed() };
    let mut res: i32 = 0;
    if result.is_null() {
        Py_DECREF!(key);
        return core::ptr::null_mut::<_object>();
    }
    res = unsafe {
        _PyDict_GetItemRef_KnownHash_LockHeld(
            (*self_).cache as *mut __SlateRecord_dictobject_h_33,
            key,
            hash,
            core::ptr::addr_of_mut!(testresult),
        )
    };
    if res > 0 {
        // Getting here means that this same key was added to the cache
        // during the PyObject_Call().  Since the link update is already
        // done, we need only return the computed result.
        Py_DECREF!(testresult);
        Py_DECREF!(key);
        return result;
    }
    if res < 0 {
        // This is an unusual case since this same lookup
        // did not previously trigger an error during lookup.
        // Treat it the same as an error in user function
        // and return with the error set.
        Py_DECREF!(key);
        Py_DECREF!(result);
        return core::ptr::null_mut::<_object>();
    }
    // This is the normal case.  The new key wasn't found before
    // user function call and it is still not there.  So we
    // proceed normally and update the cache with the new result.
    if PyDict_GET_SIZE!(unsafe { (*self_).cache }) < unsafe { (*self_).maxsize }
        || (unsafe { (*self_).root.next }) == unsafe { core::ptr::addr_of_mut!((*self_).root) }
    {
        // Cache is not full, so put the result in a new link
        link = PyObject_New!(lru_list_elem, unsafe { (*self_).lru_list_elem_type });
        if link.is_null() {
            Py_DECREF!(key);
            Py_DECREF!(result);
            return core::ptr::null_mut::<_object>();
        }
        unsafe {
            (*link).hash = hash;
        }
        unsafe {
            (*link).key = key;
        }
        unsafe {
            (*link).result = result;
        }
        // What is really needed here is a SetItem variant with a "no clobber"
        // option.  If the __eq__ call triggers a reentrant call that adds
        // this same key, then this setitem call will update the cache dict
        // with this new link, leaving the old link as an orphan (i.e. not
        // having a cache dict entry that refers to it).
        if (unsafe {
            _PyDict_SetItem_KnownHash_LockHeld(
                (*self_).cache as *mut __SlateRecord_dictobject_h_33,
                key,
                link as *mut _object,
                hash,
            )
        }) < 0
        {
            Py_DECREF__2!(link);
            return core::ptr::null_mut::<_object>();
        }
        lru_cache_append_link(self_, link);
        return Py_NewRef!(result);
    }
    let mut popresult: *mut _object = unsafe { core::mem::zeroed() };
    // Extract the oldest item.
    link = unsafe { (*self_).root.next };
    lru_cache_extract_link(link);
    // Remove it from the cache.
    // The cache dict holds one reference to the link.
    // We created one other reference when the link was created.
    // The linked list only has borrowed references.
    res = unsafe {
        _PyDict_Pop_KnownHash(
            (*self_).cache as *mut __SlateRecord_dictobject_h_33,
            (*link).key,
            (*link).hash,
            core::ptr::addr_of_mut!(popresult),
        )
    };
    if res < 0 {
        // An error arose while trying to remove the oldest key (the one
        // being evicted) from the cache.  We restore the link to its
        // original position as the oldest link.  Then we allow the
        // error propagate upward; treating it the same as an error
        // arising in the user function.
        lru_cache_prepend_link(self_, link);
        Py_DECREF!(key);
        Py_DECREF!(result);
        return core::ptr::null_mut::<_object>();
    }
    if res == 0 {
        // Getting here means that the user function call or another
        // thread has already removed the old key from the dictionary.
        // This link is now an orphan.  Since we don't want to leave the
        // cache in an inconsistent state, we don't restore the link.
        Py_DECREF__2!(link);
        Py_DECREF!(key);
        return result;
    }
    // Keep a reference to the old key and old result to prevent their
    // ref counts from going to zero during the update. That will
    // prevent potentially arbitrary object clean-up code (i.e. __del__)
    // from running while we're still adjusting the links.
    // Since the cache is full, we need to evict an old key and add
    // a new key.  Rather than free the old link and allocate a new
    // one, we reuse the link for the new key and result and move it
    // to front of the cache to mark it as recently used.
    //
    // We try to assure all code paths (including errors) leave all
    // of the links in place.  Either the link is successfully
    // updated and moved or it is restored to its old position.
    // However if an unrecoverable error is found, it doesn't
    // make sense to reinsert the link, so we leave it out
    // and the cache will no longer register as full.
    let oldkey: *mut _object = unsafe { (*link).key };
    let oldresult: *mut _object = unsafe { (*link).result };
    unsafe {
        (*link).hash = hash;
    }
    unsafe {
        (*link).key = key;
    }
    unsafe {
        (*link).result = result;
    }
    // Note:  The link is being added to the cache dict without the
    // prev and next fields set to valid values.   We have to wait
    // for successful insertion in the cache dict before adding the
    // link to the linked list.  Otherwise, the potentially reentrant
    // __eq__ call could cause the then orphan link to be visited.
    if (unsafe {
        _PyDict_SetItem_KnownHash_LockHeld(
            (*self_).cache as *mut __SlateRecord_dictobject_h_33,
            key,
            link as *mut _object,
            hash,
        )
    }) < 0
    {
        // Somehow the cache dict update failed.  We no longer can
        // restore the old link.  Let the error propagate upward and
        // leave the cache short one link.
        Py_DECREF!(popresult);
        Py_DECREF__2!(link);
        Py_DECREF!(oldkey);
        Py_DECREF!(oldresult);
        return core::ptr::null_mut::<_object>();
    }
    lru_cache_append_link(self_, link);
    Py_INCREF!(result); // for return
    Py_DECREF!(popresult);
    Py_DECREF!(oldkey);
    Py_DECREF!(oldresult);
    result
}

#[unsafe(link_section = ".text.slate_distinct._functoolsmodule.bounded_lru_cache_wrapper")]
extern "C-unwind" fn bounded_lru_cache_wrapper(
    self_: *mut lru_cache_object,
    args: *mut _object,
    kwds: *mut _object,
) -> *mut _object {
    let mut key: *mut _object = unsafe { core::mem::zeroed() };
    let mut result: *mut _object = unsafe { core::mem::zeroed() };
    let mut hash: i64 = 0;
    let res: i32 = bounded_lru_cache_get_lock_held(
        self_,
        args,
        kwds,
        unsafe { &mut (*core::ptr::addr_of_mut!(result)) },
        unsafe { &mut (*core::ptr::addr_of_mut!(key)) },
        unsafe { &mut (*core::ptr::addr_of_mut!(hash)) },
    );
    if res < 0 {
        return core::ptr::null_mut::<_object>();
    }
    if res > 0 {
        return result;
    }
    result = unsafe { PyObject_Call((*self_).func, args, kwds) };
    // Note:  key will be stolen in the below function, and
    // result may be stolen or sometimes re-returned as a passthrough.
    // Treat both as being stolen.
    result = bounded_lru_cache_update_lock_held(self_, result, key, hash);
    result
}

#[unsafe(link_section = ".text.slate_distinct._functoolsmodule.lru_cache_new")]
extern "C-unwind" fn lru_cache_new(
    r#type: *mut _typeobject,
    args: *mut _object,
    kw: *mut _object,
) -> *mut _object {
    let mut func: *mut _object = unsafe { core::mem::zeroed() };
    let mut maxsize_O: *mut _object = unsafe { core::mem::zeroed() };
    let mut cache_info_type: *mut _object = unsafe { core::mem::zeroed() };
    let mut typed: i32 = 0;
    let mut obj: *mut lru_cache_object = unsafe { core::mem::zeroed() };
    let mut maxsize: i64 = 0;
    let mut wrapper: Option<
        unsafe extern "C-unwind" fn(
            *mut lru_cache_object,
            *mut _object,
            *mut _object,
        ) -> *mut _object,
    > = unsafe { core::mem::zeroed() };
    if (unsafe {
        PyArg_ParseTupleAndKeywords(
            args,
            kw,
            c"OOpO:lru_cache".as_ptr(),
            (core::ptr::addr_of_mut!(keywords.0) as *mut *mut i8) as *const *mut i8,
            core::ptr::addr_of_mut!(func),
            core::ptr::addr_of_mut!(maxsize_O),
            core::ptr::addr_of_mut!(typed),
            core::ptr::addr_of_mut!(cache_info_type),
        )
    }) == 0
    {
        return core::ptr::null_mut::<_object>();
    }
    if (unsafe { PyCallable_Check(func) }) == 0 {
        unsafe {
            PyErr_SetString(
                PyExc_TypeError,
                c"the first argument must be callable".as_ptr(),
            )
        };
        return core::ptr::null_mut::<_object>();
    }
    let state: *mut _functools_state = get_functools_state_by_type(r#type);
    if state.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    // select the caching function, and make/inc maxsize_O
    if maxsize_O == unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } {
        wrapper = Some(infinite_lru_cache_wrapper);
        // use this only to initialize lru_cache_object attribute maxsize
        maxsize = 1i32.wrapping_neg() as i64;
    } else {
        if (unsafe { PyIndex_Check(maxsize_O) }) != 0 {
            maxsize = unsafe { PyNumber_AsSsize_t(maxsize_O, PyExc_OverflowError) };
            let __v0: bool =
                maxsize == (1i32.wrapping_neg() as i64) && !unsafe { PyErr_Occurred() }.is_null();
            if __v0 {
                return core::ptr::null_mut::<_object>();
            }
            if maxsize < 0 {
                maxsize = 0;
            }
            if maxsize == 0 {
                wrapper = Some(uncached_lru_cache_wrapper);
            } else {
                wrapper = Some(bounded_lru_cache_wrapper);
            }
        } else {
            unsafe {
                PyErr_SetString(
                    PyExc_TypeError,
                    c"maxsize should be integer or None".as_ptr(),
                )
            };
            return core::ptr::null_mut::<_object>();
        }
    }
    let __v1: *mut _object = unsafe { PyDict_New() };
    let cachedict: *mut _object = __v1;
    if __v1.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    obj =
        (unsafe { unsafe { (*r#type).tp_alloc }.unwrap()(r#type, 0i64) }) as *mut lru_cache_object;
    if obj.is_null() {
        Py_DECREF!(cachedict);
        return core::ptr::null_mut::<_object>();
    }
    unsafe {
        (*obj).root.prev = core::ptr::addr_of_mut!((*obj).root);
    }
    unsafe {
        (*obj).root.next = core::ptr::addr_of_mut!((*obj).root);
    }
    unsafe {
        (*obj).wrapper = wrapper;
    }
    unsafe {
        (*obj).typed = typed;
    }
    unsafe {
        (*obj).cache = cachedict;
    }
    unsafe {
        (*obj).func = Py_NewRef!(func);
    }
    unsafe {
        (*obj).hits = 0;
    }
    unsafe {
        (*obj).misses = 0;
    }
    unsafe {
        (*obj).maxsize = maxsize;
    }
    unsafe {
        (*obj).kwd_mark = Py_NewRef!(unsafe { (*state).kwd_mark });
    }
    unsafe {
        (*obj).lru_list_elem_type =
            Py_NewRef__2!(unsafe { (*state).lru_list_elem_type }) as *mut _typeobject;
    }
    unsafe {
        (*obj).cache_info_type = Py_NewRef!(cache_info_type);
    }
    unsafe {
        (*obj).dict = core::ptr::null_mut::<_object>();
    }
    unsafe {
        (*obj).weakreflist = core::ptr::null_mut::<_object>();
    }
    obj as *mut _object
}

static mut keywords: __SlateAlign16<[*mut i8; 5]> = __SlateAlign16([
    b"user_function\0".as_ptr() as *mut i8,
    b"maxsize\0".as_ptr() as *mut i8,
    b"typed\0".as_ptr() as *mut i8,
    b"cache_info_type\0".as_ptr() as *mut i8,
    core::ptr::null_mut::<i8>(),
]);

fn lru_cache_unlink_list(self_: *mut lru_cache_object) -> *mut lru_list_elem {
    let root: *mut lru_list_elem = unsafe { core::ptr::addr_of_mut!((*self_).root) };
    let link: *mut lru_list_elem = unsafe { (*root).next };
    if link == root {
        return core::ptr::null_mut::<lru_list_elem>();
    }
    unsafe {
        (*(*root).prev).next = core::ptr::null_mut::<lru_list_elem>();
    }
    let __v0: *mut lru_list_elem = root;
    unsafe {
        (*root).prev = __v0;
    }
    unsafe {
        (*root).next = __v0;
    }
    link
}

fn lru_cache_clear_list(mut link: *mut lru_list_elem) {
    while !link.is_null() {
        let next: *mut lru_list_elem = unsafe { (*link).next };
        Py_SETREF__13!(link, next);
    }
}

#[unsafe(link_section = ".text.slate_distinct._functoolsmodule.lru_cache_tp_clear")]
extern "C-unwind" fn lru_cache_tp_clear(op: *mut _object) -> i32 {
    let self_: *mut lru_cache_object = lru_cache_object_CAST!(op);
    let list: *mut lru_list_elem = lru_cache_unlink_list(self_);
    Py_CLEAR!((*self_).cache);
    Py_CLEAR!((*self_).func);
    Py_CLEAR!((*self_).kwd_mark);
    Py_CLEAR__3!((*self_).lru_list_elem_type);
    Py_CLEAR!((*self_).cache_info_type);
    Py_CLEAR!((*self_).dict);
    lru_cache_clear_list(list);
    0
}

#[unsafe(link_section = ".text.slate_distinct._functoolsmodule.lru_cache_dealloc")]
extern "C-unwind" fn lru_cache_dealloc(op: *mut _object) {
    let obj: *mut lru_cache_object = lru_cache_object_CAST!(op);
    let tp: *mut _typeobject = Py_TYPE__2!(obj);
    // bpo-31095: UnTrack is needed before calling any callbacks
    unsafe { PyObject_GC_UnTrack(obj as *mut ()) };
    FT_CLEAR_WEAKREFS!(op, unsafe { (*obj).weakreflist });
    lru_cache_tp_clear(op);
    unsafe { unsafe { (*tp).tp_free }.unwrap()(obj as *mut ()) };
    Py_DECREF__2!(tp);
}

#[unsafe(link_section = ".text.slate_distinct._functoolsmodule.lru_cache_call")]
extern "C-unwind" fn lru_cache_call(
    op: *mut _object,
    args: *mut _object,
    kwds: *mut _object,
) -> *mut _object {
    let self_: *mut lru_cache_object = lru_cache_object_CAST!(op);
    let result: *mut _object = unsafe { unsafe { (*self_).wrapper }.unwrap()(self_, args, kwds) };
    result
}

#[unsafe(link_section = ".text.slate_distinct._functoolsmodule.lru_cache_descr_get")]
extern "C-unwind" fn lru_cache_descr_get(
    self_: *mut _object,
    obj: *mut _object,
    r#type: *mut _object,
) -> *mut _object {
    if obj == unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } || obj.is_null() {
        return Py_NewRef!(self_);
    }
    unsafe { PyMethod_New(self_, obj) }
}

// [clinic input]
// @critical_section
// _functools._lru_cache_wrapper.cache_info
//
// Report cache statistics
// [clinic start generated code]
/// [clinic end generated code: output=cc796a0b06dbd717 input=00e1acb31aa21ecc]
fn _functools__lru_cache_wrapper_cache_info_impl(self_: *mut _object) -> *mut _object {
    let _self: *mut lru_cache_object = self_ as *mut lru_cache_object;
    if (unsafe { (*_self).maxsize }) == (1i32.wrapping_neg() as i64) {
        return unsafe {
            PyObject_CallFunction(
                (*_self).cache_info_type,
                c"nnOn".as_ptr(),
                (*_self).hits,
                (*_self).misses,
                core::ptr::addr_of_mut!(_Py_NoneStruct),
                PyDict_GET_SIZE!(unsafe { (*_self).cache }),
            )
        };
    }
    unsafe {
        PyObject_CallFunction(
            (*_self).cache_info_type,
            c"nnnn".as_ptr(),
            (*_self).hits,
            (*_self).misses,
            (*_self).maxsize,
            PyDict_GET_SIZE!(unsafe { (*_self).cache }),
        )
    }
}

// [clinic input]
// @critical_section
// _functools._lru_cache_wrapper.cache_clear
//
// Clear the cache and cache statistics
// [clinic start generated code]
/// [clinic end generated code: output=58423b35efc3e381 input=dfa33acbecf8b4b2]
fn _functools__lru_cache_wrapper_cache_clear_impl(self_: *mut _object) -> *mut _object {
    let _self: *mut lru_cache_object = self_ as *mut lru_cache_object;
    let list: *mut lru_list_elem = lru_cache_unlink_list(_self);
    FT_ATOMIC_STORE_SSIZE_RELAXED__2!((*_self).hits, 0i32);
    FT_ATOMIC_STORE_SSIZE_RELAXED__2!((*_self).misses, 0i32);
    if (unsafe { (*_self).wrapper }) == Some(bounded_lru_cache_wrapper) {
        // The critical section on the lru cache itself protects the dictionary
        // for bounded_lru_cache instances.
        unsafe { _PyDict_Clear_LockHeld((*_self).cache) };
    } else {
        unsafe { PyDict_Clear((*_self).cache) };
    }
    lru_cache_clear_list(list);
    unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
}

#[unsafe(link_section = ".text.slate_distinct._functoolsmodule.lru_cache_reduce")]
extern "C-unwind" fn lru_cache_reduce(
    self_: *mut _object,
    _unused_dummy: *mut _object,
) -> *mut _object {
    unsafe { PyObject_GetAttrString(self_, c"__qualname__".as_ptr()) }
}

#[unsafe(link_section = ".text.slate_distinct._functoolsmodule.lru_cache_copy")]
extern "C-unwind" fn lru_cache_copy(
    self_: *mut _object,
    _unused_args: *mut _object,
) -> *mut _object {
    Py_NewRef!(self_)
}

#[unsafe(link_section = ".text.slate_distinct._functoolsmodule.lru_cache_deepcopy")]
extern "C-unwind" fn lru_cache_deepcopy(
    self_: *mut _object,
    _unused_args: *mut _object,
) -> *mut _object {
    Py_NewRef!(self_)
}

#[unsafe(link_section = ".text.slate_distinct._functoolsmodule.lru_cache_tp_traverse")]
extern "C-unwind" fn lru_cache_tp_traverse(
    op: *mut _object,
    mut visit: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
    arg: *mut (),
) -> i32 {
    let self_: *mut lru_cache_object = lru_cache_object_CAST!(op);
    Py_VISIT__2!(_Py_TYPE(self_ as *mut _object), visit, arg);
    let mut link: *mut lru_list_elem = unsafe { (*self_).root.next };
    while link != unsafe { core::ptr::addr_of_mut!((*self_).root) } {
        let next: *mut lru_list_elem = unsafe { (*link).next };
        Py_VISIT!(unsafe { (*link).key }, visit, arg);
        Py_VISIT!(unsafe { (*link).result }, visit, arg);
        Py_VISIT__2!(_Py_TYPE(link as *mut _object), visit, arg);
        link = next;
    }
    Py_VISIT!(unsafe { (*self_).cache }, visit, arg);
    Py_VISIT!(unsafe { (*self_).func }, visit, arg);
    Py_VISIT!(unsafe { (*self_).kwd_mark }, visit, arg);
    Py_VISIT__2!(unsafe { (*self_).lru_list_elem_type }, visit, arg);
    Py_VISIT!(unsafe { (*self_).cache_info_type }, visit, arg);
    Py_VISIT!(unsafe { (*self_).dict }, visit, arg);
    0
}

static lru_cache_doc: __SlateAlign16<[i8; 453]> = __SlateAlign16([
    67i8, 114i8, 101i8, 97i8, 116i8, 101i8, 32i8, 97i8, 32i8, 99i8, 97i8, 99i8, 104i8, 101i8,
    100i8, 32i8, 99i8, 97i8, 108i8, 108i8, 97i8, 98i8, 108i8, 101i8, 32i8, 116i8, 104i8, 97i8,
    116i8, 32i8, 119i8, 114i8, 97i8, 112i8, 115i8, 32i8, 97i8, 110i8, 111i8, 116i8, 104i8, 101i8,
    114i8, 32i8, 102i8, 117i8, 110i8, 99i8, 116i8, 105i8, 111i8, 110i8, 46i8, 10i8, 10i8, 117i8,
    115i8, 101i8, 114i8, 95i8, 102i8, 117i8, 110i8, 99i8, 116i8, 105i8, 111i8, 110i8, 58i8, 32i8,
    32i8, 32i8, 32i8, 32i8, 32i8, 116i8, 104i8, 101i8, 32i8, 102i8, 117i8, 110i8, 99i8, 116i8,
    105i8, 111i8, 110i8, 32i8, 98i8, 101i8, 105i8, 110i8, 103i8, 32i8, 99i8, 97i8, 99i8, 104i8,
    101i8, 100i8, 10i8, 10i8, 109i8, 97i8, 120i8, 115i8, 105i8, 122i8, 101i8, 58i8, 32i8, 32i8,
    48i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 102i8, 111i8, 114i8, 32i8, 110i8,
    111i8, 32i8, 99i8, 97i8, 99i8, 104i8, 105i8, 110i8, 103i8, 10i8, 32i8, 32i8, 32i8, 32i8, 32i8,
    32i8, 32i8, 32i8, 32i8, 32i8, 78i8, 111i8, 110i8, 101i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8,
    102i8, 111i8, 114i8, 32i8, 117i8, 110i8, 108i8, 105i8, 109i8, 105i8, 116i8, 101i8, 100i8, 32i8,
    99i8, 97i8, 99i8, 104i8, 101i8, 32i8, 115i8, 105i8, 122i8, 101i8, 10i8, 32i8, 32i8, 32i8, 32i8,
    32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 110i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8,
    32i8, 102i8, 111i8, 114i8, 32i8, 97i8, 32i8, 98i8, 111i8, 117i8, 110i8, 100i8, 101i8, 100i8,
    32i8, 99i8, 97i8, 99i8, 104i8, 101i8, 10i8, 10i8, 116i8, 121i8, 112i8, 101i8, 100i8, 58i8,
    32i8, 32i8, 32i8, 32i8, 70i8, 97i8, 108i8, 115i8, 101i8, 32i8, 32i8, 32i8, 32i8, 32i8, 99i8,
    97i8, 99i8, 104i8, 101i8, 32i8, 102i8, 40i8, 51i8, 41i8, 32i8, 97i8, 110i8, 100i8, 32i8, 102i8,
    40i8, 51i8, 46i8, 48i8, 41i8, 32i8, 97i8, 115i8, 32i8, 105i8, 100i8, 101i8, 110i8, 116i8,
    105i8, 99i8, 97i8, 108i8, 32i8, 99i8, 97i8, 108i8, 108i8, 115i8, 10i8, 32i8, 32i8, 32i8, 32i8,
    32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 84i8, 114i8, 117i8, 101i8, 32i8, 32i8, 32i8, 32i8, 32i8,
    32i8, 99i8, 97i8, 99i8, 104i8, 101i8, 32i8, 102i8, 40i8, 51i8, 41i8, 32i8, 97i8, 110i8, 100i8,
    32i8, 102i8, 40i8, 51i8, 46i8, 48i8, 41i8, 32i8, 97i8, 115i8, 32i8, 100i8, 105i8, 115i8, 116i8,
    105i8, 110i8, 99i8, 116i8, 32i8, 99i8, 97i8, 108i8, 108i8, 115i8, 10i8, 10i8, 99i8, 97i8, 99i8,
    104i8, 101i8, 95i8, 105i8, 110i8, 102i8, 111i8, 95i8, 116i8, 121i8, 112i8, 101i8, 58i8, 32i8,
    32i8, 32i8, 32i8, 110i8, 97i8, 109i8, 101i8, 100i8, 116i8, 117i8, 112i8, 108i8, 101i8, 32i8,
    99i8, 108i8, 97i8, 115i8, 115i8, 32i8, 119i8, 105i8, 116i8, 104i8, 32i8, 116i8, 104i8, 101i8,
    32i8, 102i8, 105i8, 101i8, 108i8, 100i8, 115i8, 58i8, 10i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8,
    32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8,
    32i8, 32i8, 104i8, 105i8, 116i8, 115i8, 32i8, 109i8, 105i8, 115i8, 115i8, 101i8, 115i8, 32i8,
    99i8, 117i8, 114i8, 114i8, 115i8, 105i8, 122i8, 101i8, 32i8, 109i8, 97i8, 120i8, 115i8, 105i8,
    122i8, 101i8, 10i8, 0i8,
]);

static mut lru_cache_methods: __SlateAlign16<[PyMethodDef; 6]> = __SlateAlign16([
    {
        let mut __t0: PyMethodDef = unsafe { core::mem::zeroed() };
        __t0.ml_name = (b"cache_info\0".as_ptr() as *mut i8) as *const i8;
        __t0.ml_meth = Some(_functools__lru_cache_wrapper_cache_info);
        __t0.ml_flags = 4i32;
        __t0.ml_doc = unsafe {
            core::ptr::addr_of!(_functools__lru_cache_wrapper_cache_info__doc__.0) as *const i8
        };
        __t0
    },
    {
        let mut __t1: PyMethodDef = unsafe { core::mem::zeroed() };
        __t1.ml_name = (b"cache_clear\0".as_ptr() as *mut i8) as *const i8;
        __t1.ml_meth = Some(_functools__lru_cache_wrapper_cache_clear);
        __t1.ml_flags = 4i32;
        __t1.ml_doc = unsafe {
            core::ptr::addr_of!(_functools__lru_cache_wrapper_cache_clear__doc__.0) as *const i8
        };
        __t1
    },
    {
        let mut __t2: PyMethodDef = unsafe { core::mem::zeroed() };
        __t2.ml_name = (b"__reduce__\0".as_ptr() as *mut i8) as *const i8;
        __t2.ml_meth = Some(lru_cache_reduce);
        __t2.ml_flags = 4i32;
        __t2
    },
    {
        let mut __t3: PyMethodDef = unsafe { core::mem::zeroed() };
        __t3.ml_name = (b"__copy__\0".as_ptr() as *mut i8) as *const i8;
        __t3.ml_meth = Some(lru_cache_copy);
        __t3.ml_flags = 1i32;
        __t3
    },
    {
        let mut __t4: PyMethodDef = unsafe { core::mem::zeroed() };
        __t4.ml_name = (b"__deepcopy__\0".as_ptr() as *mut i8) as *const i8;
        __t4.ml_meth = Some(lru_cache_deepcopy);
        __t4.ml_flags = 1i32;
        __t4
    },
    {
        let mut __t5: PyMethodDef = unsafe { core::mem::zeroed() };
        __t5.ml_name = core::ptr::null::<i8>();
        __t5
    },
]);

static mut lru_cache_getsetlist: __SlateAlign16<[PyGetSetDef; 2]> = __SlateAlign16([
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
        __t1.name = core::ptr::null::<i8>();
        __t1
    },
]);

/// Sentinel
static mut lru_cache_memberlist: __SlateAlign16<[PyMemberDef; 3]> = __SlateAlign16([
    {
        let mut __t0: PyMemberDef = unsafe { core::mem::zeroed() };
        __t0.name = (b"__dictoffset__\0".as_ptr() as *mut i8) as *const i8;
        __t0.r#type = 19i32;
        __t0.offset = (core::mem::offset_of!(lru_cache_object, dict) as u64) as i64;
        __t0.flags = 1i32;
        __t0
    },
    {
        let mut __t1: PyMemberDef = unsafe { core::mem::zeroed() };
        __t1.name = (b"__weaklistoffset__\0".as_ptr() as *mut i8) as *const i8;
        __t1.r#type = 19i32;
        __t1.offset = (core::mem::offset_of!(lru_cache_object, weakreflist) as u64) as i64;
        __t1.flags = 1i32;
        __t1
    },
    {
        let mut __t2: PyMemberDef = unsafe { core::mem::zeroed() };
        __t2.name = core::ptr::null::<i8>();
        __t2
    },
]);

static mut lru_cache_type_slots: __SlateAlign16<[__SlateRecord_object_h_377; 11]> =
    __SlateAlign16([
        {
            let mut __t0: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
            __t0.slot = 52i32;
            __t0.pfunc = unsafe {
                core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut _object)>, *mut ()>(
                    Some(lru_cache_dealloc),
                )
            };
            __t0
        },
        {
            let mut __t1: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
            __t1.slot = 50i32;
            __t1.pfunc = unsafe {
                core::mem::transmute::<
                    Option<
                        unsafe extern "C-unwind" fn(
                            *mut _object,
                            *mut _object,
                            *mut _object,
                        ) -> *mut _object,
                    >,
                    *mut (),
                >(Some(lru_cache_call))
            };
            __t1
        },
        {
            let mut __t2: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
            __t2.slot = 56i32;
            __t2.pfunc = (unsafe { core::ptr::addr_of!(lru_cache_doc.0) as *const i8 }) as *mut ();
            __t2
        },
        {
            let mut __t3: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
            __t3.slot = 71i32;
            __t3.pfunc = unsafe {
                core::mem::transmute::<
                    Option<
                        unsafe extern "C-unwind" fn(
                            *mut _object,
                            Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
                            *mut (),
                        ) -> i32,
                    >,
                    *mut (),
                >(Some(lru_cache_tp_traverse))
            };
            __t3
        },
        {
            let mut __t4: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
            __t4.slot = 51i32;
            __t4.pfunc = unsafe {
                core::mem::transmute::<
                    Option<unsafe extern "C-unwind" fn(*mut _object) -> i32>,
                    *mut (),
                >(Some(lru_cache_tp_clear))
            };
            __t4
        },
        {
            let mut __t5: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
            __t5.slot = 64i32;
            __t5.pfunc =
                (unsafe { core::ptr::addr_of_mut!(lru_cache_methods.0) as *mut PyMethodDef })
                    as *mut ();
            __t5
        },
        {
            let mut __t6: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
            __t6.slot = 72i32;
            __t6.pfunc =
                (unsafe { core::ptr::addr_of_mut!(lru_cache_memberlist.0) as *mut PyMemberDef })
                    as *mut ();
            __t6
        },
        {
            let mut __t7: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
            __t7.slot = 73i32;
            __t7.pfunc =
                (unsafe { core::ptr::addr_of_mut!(lru_cache_getsetlist.0) as *mut PyGetSetDef })
                    as *mut ();
            __t7
        },
        {
            let mut __t8: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
            __t8.slot = 54i32;
            __t8.pfunc = unsafe {
                core::mem::transmute::<
                    Option<
                        unsafe extern "C-unwind" fn(
                            *mut _object,
                            *mut _object,
                            *mut _object,
                        ) -> *mut _object,
                    >,
                    *mut (),
                >(Some(lru_cache_descr_get))
            };
            __t8
        },
        {
            let mut __t9: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
            __t9.slot = 65i32;
            __t9.pfunc = unsafe {
                core::mem::transmute::<
                    Option<
                        unsafe extern "C-unwind" fn(
                            *mut _typeobject,
                            *mut _object,
                            *mut _object,
                        ) -> *mut _object,
                    >,
                    *mut (),
                >(Some(lru_cache_new))
            };
            __t9
        },
        {
            let mut __t10: __SlateRecord_object_h_377 = unsafe { core::mem::zeroed() };
            __t10.slot = 0i32;
            __t10.pfunc = core::ptr::null_mut::<()>();
            __t10
        },
    ]);

static mut lru_cache_type_spec: __SlateRecord_object_h_385 = {
    let mut __t0: __SlateRecord_object_h_385 = unsafe { core::mem::zeroed() };
    __t0.name = (b"functools._lru_cache_wrapper\0".as_ptr() as *mut i8) as *const i8;
    __t0.basicsize = (core::mem::size_of::<lru_cache_object>() as u64) as i32;
    __t0.flags = 147712u32;
    __t0.slots = unsafe {
        core::ptr::addr_of_mut!(lru_cache_type_slots.0) as *mut __SlateRecord_object_h_377
    };
    __t0
};

// module level code
static _functools_doc: __SlateAlign16<[i8; 33]> = __SlateAlign16([
    84i8, 111i8, 111i8, 108i8, 115i8, 32i8, 116i8, 104i8, 97i8, 116i8, 32i8, 111i8, 112i8, 101i8,
    114i8, 97i8, 116i8, 101i8, 32i8, 111i8, 110i8, 32i8, 102i8, 117i8, 110i8, 99i8, 116i8, 105i8,
    111i8, 110i8, 115i8, 46i8, 0i8,
]);

/// sentinel
static mut _functools_methods: __SlateAlign16<[PyMethodDef; 3]> = __SlateAlign16([
    {
        let mut __t0: PyMethodDef = unsafe { core::mem::zeroed() };
        __t0.ml_name = (b"reduce\0".as_ptr() as *mut i8) as *const i8;
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
                            *mut _object,
                        ) -> *mut _object,
                    >,
                    Option<unsafe extern "C-unwind" fn()>,
                >(Some(_functools_reduce))
            })
        };
        __t0.ml_flags = 130i32;
        __t0.ml_doc = unsafe { core::ptr::addr_of!(_functools_reduce__doc__.0) as *const i8 };
        __t0
    },
    {
        let mut __t1: PyMethodDef = unsafe { core::mem::zeroed() };
        __t1.ml_name = (b"cmp_to_key\0".as_ptr() as *mut i8) as *const i8;
        __t1.ml_meth = unsafe {
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
                            *mut _object,
                        ) -> *mut _object,
                    >,
                    Option<unsafe extern "C-unwind" fn()>,
                >(Some(_functools_cmp_to_key))
            })
        };
        __t1.ml_flags = 130i32;
        __t1.ml_doc = unsafe { core::ptr::addr_of!(_functools_cmp_to_key__doc__.0) as *const i8 };
        __t1
    },
    {
        let mut __t2: PyMethodDef = unsafe { core::mem::zeroed() };
        __t2.ml_name = core::ptr::null::<i8>();
        __t2.ml_meth = None;
        __t2
    },
]);

#[unsafe(link_section = ".text.slate_distinct._functoolsmodule._functools_exec")]
extern "C-unwind" fn _functools_exec(module: *mut _object) -> i32 {
    let state: *mut _functools_state = get_functools_state(module);
    unsafe {
        (*state).kwd_mark =
            _PyObject_CallNoArgs(core::ptr::addr_of_mut!(PyBaseObject_Type) as *mut _object);
    }
    if unsafe { (*state).kwd_mark }.is_null() {
        return 1i32.wrapping_neg();
    }
    unsafe {
        (*state).placeholder_type = PyType_FromModuleAndSpec(
            module,
            core::ptr::addr_of_mut!(placeholder_type_spec),
            core::ptr::null_mut::<_object>(),
        ) as *mut _typeobject;
    }
    if unsafe { (*state).placeholder_type }.is_null() {
        return 1i32.wrapping_neg();
    }
    if (unsafe { PyModule_AddType(module, (*state).placeholder_type) }) < 0 {
        return 1i32.wrapping_neg();
    }
    let placeholder: *mut _object =
        unsafe { PyObject_CallNoArgs((*state).placeholder_type as *mut _object) };
    if placeholder.is_null() {
        return 1i32.wrapping_neg();
    }
    if (unsafe { PyModule_AddObjectRef(module, c"Placeholder".as_ptr(), placeholder) }) < 0 {
        Py_DECREF!(placeholder);
        return 1i32.wrapping_neg();
    }
    Py_DECREF!(placeholder);
    unsafe {
        (*state).partial_type = PyType_FromModuleAndSpec(
            module,
            core::ptr::addr_of_mut!(partial_type_spec),
            core::ptr::null_mut::<_object>(),
        ) as *mut _typeobject;
    }
    if unsafe { (*state).partial_type }.is_null() {
        return 1i32.wrapping_neg();
    }
    if (unsafe { PyModule_AddType(module, (*state).partial_type) }) < 0 {
        return 1i32.wrapping_neg();
    }
    let lru_cache_type: *mut _object = unsafe {
        PyType_FromModuleAndSpec(
            module,
            core::ptr::addr_of_mut!(lru_cache_type_spec),
            core::ptr::null_mut::<_object>(),
        )
    };
    if lru_cache_type.is_null() {
        return 1i32.wrapping_neg();
    }
    if (unsafe { PyModule_AddType(module, lru_cache_type as *mut _typeobject) }) < 0 {
        Py_DECREF!(lru_cache_type);
        return 1i32.wrapping_neg();
    }
    Py_DECREF!(lru_cache_type);
    unsafe {
        (*state).keyobject_type = PyType_FromModuleAndSpec(
            module,
            core::ptr::addr_of_mut!(keyobject_type_spec),
            core::ptr::null_mut::<_object>(),
        ) as *mut _typeobject;
    }
    if unsafe { (*state).keyobject_type }.is_null() {
        return 1i32.wrapping_neg();
    }
    // keyobject_type is used only internally.
    //
    // So we don't expose it in module namespace.
    unsafe {
        (*state).lru_list_elem_type = PyType_FromModuleAndSpec(
            module,
            core::ptr::addr_of_mut!(lru_list_elem_type_spec),
            core::ptr::null_mut::<_object>(),
        ) as *mut _typeobject;
    }
    if unsafe { (*state).lru_list_elem_type }.is_null() {
        return 1i32.wrapping_neg();
    }
    // lru_list_elem is used only in _lru_cache_wrapper.
    //
    // So we don't expose it in module namespace.
    0
}

#[unsafe(link_section = ".text.slate_distinct._functoolsmodule._functools_traverse")]
extern "C-unwind" fn _functools_traverse(
    module: *mut _object,
    mut visit: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
    arg: *mut (),
) -> i32 {
    let state: *mut _functools_state = get_functools_state(module);
    Py_VISIT!(unsafe { (*state).kwd_mark }, visit, arg);
    Py_VISIT__2!(unsafe { (*state).placeholder_type }, visit, arg);
    Py_VISIT!(unsafe { (*state).placeholder }, visit, arg);
    Py_VISIT__2!(unsafe { (*state).partial_type }, visit, arg);
    Py_VISIT__2!(unsafe { (*state).keyobject_type }, visit, arg);
    Py_VISIT__2!(unsafe { (*state).lru_list_elem_type }, visit, arg);
    0
}

#[unsafe(link_section = ".text.slate_distinct._functoolsmodule._functools_clear")]
extern "C-unwind" fn _functools_clear(module: *mut _object) -> i32 {
    let state: *mut _functools_state = get_functools_state(module);
    Py_CLEAR!((*state).kwd_mark);
    Py_CLEAR__3!((*state).placeholder_type);
    Py_CLEAR!((*state).placeholder);
    Py_CLEAR__3!((*state).partial_type);
    Py_CLEAR__3!((*state).keyobject_type);
    Py_CLEAR__3!((*state).lru_list_elem_type);
    0
}

#[unsafe(link_section = ".text.slate_distinct._functoolsmodule._functools_free")]
extern "C-unwind" fn _functools_free(module: *mut ()) {
    _functools_clear(module as *mut _object);
}

static mut _functools_slots: __SlateAlign16<[PyModuleDef_Slot; 4]> = __SlateAlign16([
    {
        let mut __t0: PyModuleDef_Slot = unsafe { core::mem::zeroed() };
        __t0.slot = 2i32;
        __t0.value = unsafe {
            core::mem::transmute::<Option<unsafe extern "C-unwind" fn(*mut _object) -> i32>, *mut ()>(
                Some(_functools_exec),
            )
        };
        __t0
    },
    {
        let mut __t1: PyModuleDef_Slot = unsafe { core::mem::zeroed() };
        __t1.slot = 3i32;
        __t1.value = 2i32 as *mut ();
        __t1
    },
    {
        let mut __t2: PyModuleDef_Slot = unsafe { core::mem::zeroed() };
        __t2.slot = 4i32;
        __t2.value = 1i32 as *mut ();
        __t2
    },
    {
        let mut __t3: PyModuleDef_Slot = unsafe { core::mem::zeroed() };
        __t3.slot = 0i32;
        __t3.value = core::ptr::null_mut::<()>();
        __t3
    },
]);

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct._functoolsmodule.PyInit__functools")]
extern "C-unwind" fn PyInit__functools() -> *mut _object {
    unsafe { PyModuleDef_Init(core::ptr::addr_of_mut!(_functools_module)) }
}
