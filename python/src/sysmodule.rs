//! System module
//! Version parsed out into numeric values
//!
//! --start constants--
//! The size of 'void *', as computed by sizeof.
//! Convert the argument to a string. For example, Py_STRINGIFY(123) is replaced
//! with "123" by the preprocessor. Defines are also replaced by their value.
//! For example Py_STRINGIFY(__LINE__) is replaced by the line number, not
//! by "__LINE__".
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
//! The following values are used for 'what' for tracefunc functions
//!
//! To add a new kind of trace event, also update "trace_init" in
//! Python/sysmodule.c to define the Python level event name
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
//! Threshold for max digits check.  For performance reasons int() and
//! int.__str__() don't checks values that are smaller than this
//! threshold.  Acts as a guaranteed minimum size limit for bignums that
//! applications can expect from CPython.
//!
//! % python -m timeit -s 's = "1"*640; v = int(s)' 'str(int(s))'
//! 20000 loops, best of 5: 12 usec per loop
//!
//! "640 digits should be enough for anyone." - gps
//! fits a ~2126 bit decimal number.
// Version as a single 4-byte hex number, e.g. 0x010502B2 == 1.5.2b2.
// Use this for numeric comparisons, e.g. #if PY_VERSION_HEX >= ...
// framework name
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
// Long value tag bits:
// 0-1: Sign bits value = (1-sign), ie. negative=2, positive=0, zero=1.
// 2: Set to 1 for the small ints
// 3+ Unsigned digit count
macro_rules! BITS_TO_PTR_MASKED {
    ($REF:expr) => {
        ((unsafe { $REF.bits }) & 18446744073709551614u64) as *mut _object
    };
}

macro_rules! EVAL_CALL_STAT_INC_IF_FUNCTION {
    () => {};
}

macro_rules! PyBytes_AS_STRING {
    ($op:expr) => {
        PyBytes_AS_STRING(_PyObject_CAST!($op))
    };
}

macro_rules! PyDict_Check {
    ($op:expr) => {
        PyType_FastSubclass!(Py_TYPE!($op), 536870912u64)
    };
}

macro_rules! PyFloat_AS_DOUBLE {
    ($op:expr) => {
        PyFloat_AS_DOUBLE(_PyObject_CAST!($op))
    };
}

macro_rules! PyFloat_CheckExact {
    ($op:expr) => {
        Py_IS_TYPE!($op, unsafe { core::ptr::addr_of_mut!(PyFloat_Type) })
    };
}

macro_rules! PyList_Check {
    ($op:expr) => {
        PyType_FastSubclass!(Py_TYPE!($op), 33554432u64)
    };
}

macro_rules! PyList_GET_SIZE {
    ($op:expr) => {
        PyList_GET_SIZE(_PyObject_CAST!($op))
    };
}

macro_rules! PyList_SET_ITEM {
    ($op:expr, $index:expr, $value:expr) => {
        PyList_SET_ITEM(_PyObject_CAST!($op), $index, _PyObject_CAST!($value));
    };
}

macro_rules! PyList_SET_ITEM__2 {
    ($op:expr, $index:expr, $value:expr) => {
        PyList_SET_ITEM(
            _PyObject_CAST!($op),
            ($index) as i64,
            _PyObject_CAST!($value),
        );
    };
}

macro_rules! PyObject_TypeCheck {
    ($ob:expr, $type:expr) => {
        PyObject_TypeCheck(_PyObject_CAST!($ob), $type)
    };
}

macro_rules! PyStackRef_IsNull {
    ($ref:expr) => {
        (unsafe { $ref.bits }) == 1
    };
}

macro_rules! PyTuple_Check {
    ($op:expr) => {
        PyType_FastSubclass!(Py_TYPE!($op), 67108864u64)
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

macro_rules! PyUnicode_CHECK_INTERNED {
    ($op:expr) => {
        PyUnicode_CHECK_INTERNED(_PyObject_CAST!($op))
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

macro_rules! PyVectorcall_NARGS {
    ($n:expr) => {
        _PyVectorcall_NARGS($n)
    };
}

macro_rules! Py_ARRAY_LENGTH__16 {
    () => {
        (core::mem::size_of::<[*const i8; 297]>() as u64)
            / (core::mem::size_of::<*const i8>() as u64)
    };
}

macro_rules! Py_ARRAY_LENGTH__17 {
    () => {
        (core::mem::size_of::<[PyStructSequence_Field; 22]>() as u64)
            / (core::mem::size_of::<PyStructSequence_Field>() as u64)
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

macro_rules! Py_FatalError__11 {
    ($message:expr) => {
        unsafe { _Py_FatalErrorFunc(c"PySys_SetPath".as_ptr(), ($message) as *const i8) };
        unsafe { core::hint::unreachable_unchecked() };
    };
}

macro_rules! Py_FatalError__3 {
    ($message:expr) => {
        unsafe { _Py_FatalErrorFunc(c"PySys_SetArgvEx".as_ptr(), ($message) as *const i8) };
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

macro_rules! SET_HASH_INFO_ITEM {
    ($CALL:expr, $hash_info:expr, $field:expr) => {{
        loop {
            let item: *mut _object = $CALL;
            if item.is_null() {
                Py_CLEAR__2!($hash_info);
                return core::ptr::null_mut::<_object>();
            }
            let __v: i32 = $field;
            let __v2: i32 = __v.wrapping_add(1);
            $field = __v2;
            unsafe { PyStructSequence_SetItem($hash_info, __v as i64, item) };
            if true {
                break;
            }
        }
    }};
}

macro_rules! SetFlagObj {
    ($expr:expr, $flags:expr, $pos:expr) => {{
        loop {
            let value: *mut _object = $expr;
            if value.is_null() {
                return 1i32.wrapping_neg();
            }
            sys_set_flag($flags, $pos, value);
            Py_DECREF!(value);
            let __v: i64 = $pos;
            let __v2: i64 = __v.wrapping_add(1);
            $pos = __v2;
            if true {
                break;
            }
        }
    }};
}

macro_rules! SetFlag {
    ($expr:expr, $flags:expr, $pos:expr) => {{
        SetFlagObj!(unsafe { PyLong_FromLong(($expr) as i64) }, $flags, $pos);
    }};
}

macro_rules! SetIntItem {
    ($flag:expr, $pos:expr, $version_info:expr) => {
        let __v: i32 = $pos;
        let __v2: i32 = __v.wrapping_add(1);
        $pos = __v2;
        unsafe {
            PyStructSequence_SetItem($version_info, __v as i64, PyLong_FromLong(($flag) as i64))
        };
    };
}

macro_rules! SetStrItem {
    ($flag:expr, $pos:expr, $version_info:expr) => {
        let __v: i32 = $pos;
        let __v2: i32 = __v.wrapping_add(1);
        $pos = __v2;
        unsafe {
            PyStructSequence_SetItem(
                $version_info,
                __v as i64,
                PyUnicode_FromString(($flag) as *const i8),
            )
        };
    };
}

macro_rules! _PyASCIIObject_CAST {
    ($op:expr) => {
        _Py_CAST!(*mut __SlateRecord_unicodeobject_h_163, $op)
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

macro_rules! _PyArg_CheckPositional__3 {
    ($funcname:expr, $nargs:expr, $min:expr, $max:expr) => {{
        let __v: bool = !_Py_ANY_VARARGS__2!($max) && (($min) as i64) <= $nargs && ($nargs) <= $max
            || (unsafe {
                _PyArg_CheckPositional(($funcname) as *const i8, $nargs, ($min) as i64, $max)
            }) != 0;
        __v
    }};
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

macro_rules! _PyFloat_CAST {
    ($op:expr) => {
        _Py_CAST!(*mut __SlateRecord_floatobject_h_8, $op)
    };
}

/// Cast argument to PyListObject* type.
macro_rules! _PyList_CAST {
    ($op:expr) => {
        _Py_CAST!(*mut __SlateRecord_listobject_h_22, $op)
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

macro_rules! _PyStatus_GET_FUNC__14 {
    () => {
        c"_PySys_Create".as_ptr() as *mut i8
    };
}

macro_rules! _PyStatus_GET_FUNC__44 {
    () => {
        c"_PySys_SetPreliminaryStderr".as_ptr() as *mut i8
    };
}

macro_rules! _PyStatus_GET_FUNC__33 {
    () => {
        c"_PySys_InitCore".as_ptr() as *mut i8
    };
}

/// Cast argument to PyTupleObject* type.
macro_rules! _PyTuple_CAST {
    ($op:expr) => {
        _Py_CAST!(*mut __SlateRecord_tupleobject_h_13, $op)
    };
}

/// Fast inlined version of PyType_IS_GC()
macro_rules! _PyType_IS_GC {
    ($t:expr) => {
        _PyType_HasFeature($t, 16384u64)
    };
}

/// Cast argument to PyVarObject* type.
macro_rules! _PyVarObject_CAST {
    ($op:expr) => {
        _Py_CAST!(*mut __SlateRecord_object_h_172, $op)
    };
}

macro_rules! _Py_ANY_VARARGS__2 {
    ($n:expr) => {
        ($n) == 9223372036854775807
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

/// Call Py_FatalError() if tstate is NULL
macro_rules! _Py_EnsureTstateNotNULL {
    ($tstate:expr) => {
        _Py_EnsureFuncTstateNotNULL(c"PySys_AddWarnOptionUnicode".as_ptr(), $tstate);
    };
}

/// Call Py_FatalError() if tstate is NULL
macro_rules! _Py_EnsureTstateNotNULL__7 {
    ($tstate:expr) => {
        _Py_EnsureFuncTstateNotNULL(c"sys_audit_impl".as_ptr(), $tstate);
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

macro_rules! _Py_LATIN1_CHR {
    ($CH:expr) => {
        if ($CH) < 128 {
            unsafe {
                core::ptr::addr_of_mut!(_PyRuntime.static_objects.singletons.strings.ascii)
                    as *mut __SlateRecord_pycore_global_strings_h_815
            }
            .wrapping_offset(($CH) as isize) as *mut _object
        } else {
            unsafe {
                core::ptr::addr_of_mut!(_PyRuntime.static_objects.singletons.strings.latin1)
                    as *mut __SlateRecord_pycore_global_strings_h_819
            }
            .wrapping_offset($CH.wrapping_sub(128) as isize) as *mut _object
        }
    };
}

macro_rules! _Py_PACK_FULL_VERSION {
    ($X:expr, $Y:expr, $Z:expr, $LEVEL:expr, $SERIAL:expr) => {
        (($X) & 255) << 24i32
            | (($Y) & 255) << 16i32
            | (($Z) & 255) << 8i32
            | (($LEVEL) & 15) << 4i32
            | (($SERIAL) & 15) << 0i32
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
    static mut PyType_Type: _typeobject;
    static mut _Py_NoneStruct: _object;
    static mut Py_IsolatedFlag: i32;
    static mut PyUnicode_Type: _typeobject;
    static mut PyExc_Exception: *mut _object;
    static mut PyExc_AttributeError: *mut _object;
    static mut PyExc_OSError: *mut _object;
    static mut PyExc_ImportError: *mut _object;
    static mut PyExc_RuntimeError: *mut _object;
    static mut PyExc_RecursionError: *mut _object;
    static mut PyExc_SystemExit: *mut _object;
    static mut PyExc_TypeError: *mut _object;
    static mut PyExc_UnicodeEncodeError: *mut _object;
    static mut PyExc_ValueError: *mut _object;
    static mut PyExc_FileNotFoundError: *mut _object;
    static mut PyExc_PermissionError: *mut _object;
    static mut PyExc_DeprecationWarning: *mut _object;
    static mut PyExc_RuntimeWarning: *mut _object;
    static mut _Py_FalseStruct: _longobject;
    static mut _Py_TrueStruct: _longobject;
    static mut PyFloat_Type: _typeobject;
    static mut PyTuple_Type: _typeobject;
    static mut PyFrozenSet_Type: _typeobject;
    #[thread_local]
    static mut _Py_tss_tstate: *mut _ts;
    static mut _Py_perfmap_callbacks: __SlateRecord_pycore_ceval_h_102;
    static mut _Py_perfmap_jit_callbacks: __SlateRecord_pycore_ceval_h_102;
    static mut _PyRuntime: pyruntimestate;
    fn wcscmp(arg0: *const i32, arg1: *const i32) -> i32;
    fn wcschr(arg0: *const i32, arg1: i32) -> *mut i32;
    fn fflush(arg0: *mut slate_support::FILE) -> i32;
    #[link_name = "fopen64"]
    fn fopen(arg0: *const i8, arg1: *const i8) -> *mut slate_support::FILE;
    fn fdopen(arg0: i32, arg1: *const i8) -> *mut slate_support::FILE;
    fn fread(arg0: *mut (), arg1: u64, arg2: u64, arg3: *mut slate_support::FILE) -> u64;
    fn fwrite(arg0: *const (), arg1: u64, arg2: u64, arg3: *mut slate_support::FILE) -> u64;
    fn fileno(arg0: *mut slate_support::FILE) -> i32;
    fn index(arg0: *const i8, arg1: i32) -> *mut i8;
    fn PyMem_RawMalloc(arg0: u64) -> *mut ();
    fn PyMem_RawFree(arg0: *mut ());
    fn PyMutex_Lock(arg0: *mut PyMutex);
    fn PyMutex_Unlock(arg0: *mut PyMutex);
    fn PyType_IsSubtype(arg0: *mut _typeobject, arg1: *mut _typeobject) -> i32;
    fn PyType_Ready(arg0: *mut _typeobject) -> i32;
    fn PyType_ClearCache() -> u32;
    fn PyType_Modified(arg0: *mut _typeobject);
    fn PyObject_Repr(arg0: *mut _object) -> *mut _object;
    fn PyObject_GetAttrString(arg0: *mut _object, arg1: *const i8) -> *mut _object;
    fn PyObject_GetAttr(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn PyObject_GetOptionalAttr(
        arg0: *mut _object,
        arg1: *mut _object,
        arg2: *mut *mut _object,
    ) -> i32;
    fn PyObject_SetAttr(arg0: *mut _object, arg1: *mut _object, arg2: *mut _object) -> i32;
    fn PyObject_IsTrue(arg0: *mut _object) -> i32;
    fn PyCallable_Check(arg0: *mut _object) -> i32;
    fn PyUnstable_IsImmortal(arg0: *mut _object) -> i32;
    fn _Py_Dealloc(arg0: *mut _object);
    fn PyObject_GC_UnTrack(arg0: *mut ());
    fn PyHash_GetFuncDef() -> *mut __SlateRecord_pyhash_h_39;
    fn Py_GETENV(arg0: *const i8) -> *mut i8;
    fn PyUnicode_FromStringAndSize(arg0: *const i8, arg1: i64) -> *mut _object;
    fn PyUnicode_FromString(arg0: *const i8) -> *mut _object;
    fn PyUnicode_FromEncodedObject(
        arg0: *mut _object,
        arg1: *const i8,
        arg2: *const i8,
    ) -> *mut _object;
    fn PyUnicode_FromFormatV(arg0: *const i8, arg1: core::ffi::VaList<'static>) -> *mut _object;
    fn PyUnicode_InternFromString(arg0: *const i8) -> *mut _object;
    fn PyUnicode_FromWideChar(arg0: *const i32, arg1: i64) -> *mut _object;
    fn PyUnicode_AsEncodedString(
        arg0: *mut _object,
        arg1: *const i8,
        arg2: *const i8,
    ) -> *mut _object;
    fn PyUnicode_AsUTF8AndSize(arg0: *mut _object, arg1: *mut i64) -> *const i8;
    fn PyUnicode_FSConverter(arg0: *mut _object, arg1: *mut ()) -> i32;
    fn PyUnicode_AsUTF8(arg0: *mut _object) -> *const i8;
    fn PyErr_SetObject(arg0: *mut _object, arg1: *mut _object);
    fn PyErr_SetString(arg0: *mut _object, arg1: *const i8);
    fn PyErr_Occurred() -> *mut _object;
    fn PyErr_Clear();
    fn PyErr_SetFromErrno(arg0: *mut _object) -> *mut _object;
    fn PyErr_Format(arg0: *mut _object, arg1: *const i8, ...) -> *mut _object;
    fn PyOS_vsnprintf(
        arg0: *mut i8,
        arg1: u64,
        arg2: *const i8,
        arg3: core::ffi::VaList<'static>,
    ) -> i32;
    fn _Py_FatalErrorFunc(arg0: *const i8, arg1: *const i8);
    fn PyErr_FormatUnraisable(arg0: *const i8, ...);
    fn PyLong_FromLong(arg0: i64) -> *mut _object;
    fn PyLong_FromSize_t(arg0: u64) -> *mut _object;
    fn PyLong_FromSsize_t(arg0: i64) -> *mut _object;
    fn PyLong_AsSsize_t(arg0: *mut _object) -> i64;
    fn PyLong_AsInt(arg0: *mut _object) -> i32;
    fn PyLong_GetInfo() -> *mut _object;
    fn PyBool_FromLong(arg0: i64) -> *mut _object;
    fn PyFloat_GetInfo() -> *mut _object;
    fn PyFloat_FromDouble(arg0: f64) -> *mut _object;
    fn PyFloat_AsDouble(arg0: *mut _object) -> f64;
    fn PyTuple_New(arg0: i64) -> *mut _object;
    fn PyTuple_Pack(arg0: i64, ...) -> *mut _object;
    fn PyList_New(arg0: i64) -> *mut _object;
    fn PyList_Insert(arg0: *mut _object, arg1: i64, arg2: *mut _object) -> i32;
    fn PyList_Append(arg0: *mut _object, arg1: *mut _object) -> i32;
    fn PyList_SetSlice(arg0: *mut _object, arg1: i64, arg2: i64, arg3: *mut _object) -> i32;
    fn PyList_Sort(arg0: *mut _object) -> i32;
    fn PyList_AsTuple(arg0: *mut _object) -> *mut _object;
    fn PyDict_New() -> *mut _object;
    fn PyDict_SetItem(arg0: *mut _object, arg1: *mut _object, arg2: *mut _object) -> i32;
    fn PyDict_Copy(arg0: *mut _object) -> *mut _object;
    fn PyDict_SetItemString(arg0: *mut _object, arg1: *const i8, arg2: *mut _object) -> i32;
    fn PyDict_GetItemRef(arg0: *mut _object, arg1: *mut _object, arg2: *mut *mut _object) -> i32;
    fn PyDict_GetItemStringRef(arg0: *mut _object, arg1: *const i8, arg2: *mut *mut _object)
    -> i32;
    fn PyDict_Pop(arg0: *mut _object, arg1: *mut _object, arg2: *mut *mut _object) -> i32;
    fn PyModule_GetDict(arg0: *mut _object) -> *mut _object;
    fn PyFunction_GetModule(arg0: *mut _object) -> *mut _object;
    fn PyFile_WriteObject(arg0: *mut _object, arg1: *mut _object, arg2: i32) -> i32;
    fn PyFile_NewStdPrinter(arg0: i32) -> *mut _object;
    fn PyWideStringList_Append(
        arg0: *mut __SlateRecord_initconfig_h_36,
        arg1: *const i32,
    ) -> __SlateRecord_initconfig_h_19;
    fn PyThreadState_EnterTracing(arg0: *mut _ts);
    fn PyThreadState_LeaveTracing(arg0: *mut _ts);
    fn _PyThread_CurrentFrames() -> *mut _object;
    fn PyErr_WarnEx(arg0: *mut _object, arg1: *const i8, arg2: i64) -> i32;
    fn PyErr_WarnFormat(arg0: *mut _object, arg1: i64, arg2: *const i8, ...) -> i32;
    fn PyStructSequence_New(arg0: *mut _typeobject) -> *mut _object;
    fn PyStructSequence_SetItem(arg0: *mut _object, arg1: i64, arg2: *mut _object);
    fn PyStructSequence_GetItem(arg0: *mut _object, arg1: i64) -> *mut _object;
    fn PyThread_allocate_lock() -> *mut ();
    fn PyThread_free_lock(arg0: *mut ());
    fn PyThread_acquire_lock(arg0: *mut (), arg1: i32) -> i32;
    fn PyThread_release_lock(arg0: *mut ());
    fn PyThread_GetInfo() -> *mut _object;
    fn PyArg_ParseTupleAndKeywords(
        arg0: *mut _object,
        arg1: *mut _object,
        arg2: *const i8,
        arg3: *const *mut i8,
        ...
    ) -> i32;
    fn Py_BuildValue(arg0: *const i8, ...) -> *mut _object;
    fn Py_VaBuildValue(arg0: *const i8, arg1: core::ffi::VaList<'static>) -> *mut _object;
    fn PyErr_Display(arg0: *mut _object, arg1: *mut _object, arg2: *mut _object);
    fn PyRun_AnyFileExFlags(
        arg0: *mut slate_support::FILE,
        arg1: *const i8,
        arg2: i32,
        arg3: *mut __SlateRecord_compile_h_30,
    ) -> i32;
    fn Py_GetVersion() -> *const i8;
    fn Py_GetPlatform() -> *const i8;
    fn Py_GetCopyright() -> *const i8;
    fn Py_IsFinalizing() -> i32;
    fn Py_SetRecursionLimit(arg0: i32);
    fn Py_GetRecursionLimit() -> i32;
    fn PyImport_GetModule(arg0: *mut _object) -> *mut _object;
    fn PyImport_Import(arg0: *mut _object) -> *mut _object;
    fn PyObject_CallFunction(arg0: *mut _object, arg1: *const i8, ...) -> *mut _object;
    fn PyObject_Vectorcall(
        arg0: *mut _object,
        arg1: *const *mut _object,
        arg2: u64,
        arg3: *mut _object,
    ) -> *mut _object;
    fn PyObject_VectorcallMethod(
        arg0: *mut _object,
        arg1: *const *mut _object,
        arg2: u64,
        arg3: *mut _object,
    ) -> *mut _object;
    fn PyObject_GetIter(arg0: *mut _object) -> *mut _object;
    fn PyIter_Next(arg0: *mut _object) -> *mut _object;
    fn PyMapping_GetItemString(arg0: *mut _object, arg1: *const i8) -> *mut _object;
    fn Py_fopen(arg0: *mut _object, arg1: *const i8) -> *mut slate_support::FILE;
    fn _PyMem_RawStrdup(arg0: *const i8) -> *mut i8;
    fn _PyMem_DefaultRawCalloc(arg0: u64, arg1: u64) -> *mut ();
    fn _PyMem_DefaultRawFree(arg0: *mut ());
    fn _PyMem_DefaultRawWcsdup(arg0: *const i32) -> *mut i32;
    fn _PyThread_CurrentExceptions() -> *mut _object;
    fn _Py_GetConfig() -> *const PyConfig;
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
    fn _PyErr_GetTopmostException(arg0: *mut _ts) -> *mut _err_stackitem;
    fn _PyErr_StackItemToExcInfoTuple(arg0: *mut _err_stackitem) -> *mut _object;
    fn _PyErr_GetRaisedException(arg0: *mut _ts) -> *mut _object;
    fn _PyErr_ExceptionMatches(arg0: *mut _ts, arg1: *mut _object) -> i32;
    fn _PyErr_SetRaisedException(arg0: *mut _ts, arg1: *mut _object);
    fn _PyErr_Clear(arg0: *mut _ts);
    fn _PyErr_NoMemory(arg0: *mut _ts) -> *mut _object;
    fn _PyErr_SetString(arg0: *mut _ts, arg1: *mut _object, arg2: *const i8);
    fn _PyErr_Format(arg0: *mut _ts, arg1: *mut _object, arg2: *const i8, ...) -> *mut _object;
    fn _PyInterpreterState_GetConfig(arg0: *mut _is) -> *const PyConfig;
    fn _PyEval_SetProfile(
        arg0: *mut _ts,
        arg1: Option<
            unsafe extern "C-unwind" fn(*mut _object, *mut _frame, i32, *mut _object) -> i32,
        >,
        arg2: *mut _object,
    ) -> i32;
    fn _PyEval_SetProfileAllThreads(
        arg0: *mut _is,
        arg1: Option<
            unsafe extern "C-unwind" fn(*mut _object, *mut _frame, i32, *mut _object) -> i32,
        >,
        arg2: *mut _object,
    ) -> i32;
    fn _PyEval_SetTrace(
        arg0: *mut _ts,
        arg1: Option<
            unsafe extern "C-unwind" fn(*mut _object, *mut _frame, i32, *mut _object) -> i32,
        >,
        arg2: *mut _object,
    ) -> i32;
    fn _PyEval_SetTraceAllThreads(
        arg0: *mut _is,
        arg1: Option<
            unsafe extern "C-unwind" fn(*mut _object, *mut _frame, i32, *mut _object) -> i32,
        >,
        arg2: *mut _object,
    ) -> i32;
    fn _PyEval_SetSwitchInterval(arg0: u64);
    fn _PyEval_GetSwitchInterval() -> u64;
    fn _PyEval_CallTracing(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn _PyEval_GetAsyncGenFirstiter() -> *mut _object;
    fn _PyEval_GetAsyncGenFinalizer() -> *mut _object;
    fn _PyEval_SetAsyncGenFirstiter(arg0: *mut _object) -> i32;
    fn _PyEval_SetAsyncGenFinalizer(arg0: *mut _object) -> i32;
    fn _PyEval_GetCoroutineOriginTrackingDepth() -> i32;
    fn _PyEval_SetCoroutineOriginTrackingDepth(arg0: i32) -> i32;
    fn _PyPerfTrampoline_SetCallbacks(arg0: *mut __SlateRecord_pycore_ceval_h_102) -> i32;
    fn _PyPerfTrampoline_GetCallbacks(arg0: *mut __SlateRecord_pycore_ceval_h_102);
    fn _PyPerfTrampoline_Init(arg0: i32) -> i32;
    fn _PyIsPerfTrampolineActive() -> i32;
    fn _PyImport_FixupBuiltin(
        arg0: *mut _ts,
        arg1: *mut _object,
        arg2: *const i8,
        arg3: *mut _object,
    ) -> i32;
    fn _PyImport_GetDLOpenFlags(arg0: *mut _is) -> i32;
    fn _PyImport_SetDLOpenFlags(arg0: *mut _is, arg1: i32);
    fn _PyImport_InitModules(arg0: *mut _is) -> *mut _object;
    fn _PyImport_GetBuiltinModuleNames() -> *mut _object;
    fn _PyWideStringList_AsList(arg0: *const __SlateRecord_initconfig_h_36) -> *mut _object;
    fn _PyConfig_CreateXOptionsDict(arg0: *const PyConfig) -> *mut _object;
    fn _Py_GetGlobalAllocatedBlocks() -> i64;
    fn _PyObject_DebugMallocStats(arg0: *mut slate_support::FILE) -> i32;
    fn _PyRuntime_Initialize() -> __SlateRecord_initconfig_h_19;
    fn _PyType_GetDict(arg0: *mut _typeobject) -> *mut _object;
    fn _PyObject_DebugTypeStats(arg0: *mut slate_support::FILE);
    fn _PyObject_LookupSpecial(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn _PyFrame_MakeAndSetFrameObject(arg0: *mut _PyInterpreterFrame) -> *mut _frame;
    fn _PyArg_CheckPositional(arg0: *const i8, arg1: i64, arg2: i64, arg3: i64) -> i32;
    fn _PyModule_CreateInitialized(arg0: *mut PyModuleDef, arg1: i32) -> *mut _object;
    fn _PyArg_BadArgument(arg0: *const i8, arg1: *const i8, arg2: *const i8, arg3: *mut _object);
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
    fn _PyNamespace_New(arg0: *mut _object) -> *mut _object;
    fn _PyDumpExecutors(arg0: *mut slate_support::FILE) -> i32;
    fn _PyPathConfig_ComputeSysPath0(
        arg0: *const __SlateRecord_initconfig_h_36,
        arg1: *mut *mut _object,
    ) -> i32;
    fn _Py_GetStdlibDir() -> *mut i32;
    fn _PyErr_WriteUnraisableDefaultHook(arg0: *mut _object) -> *mut _object;
    fn _Py_gitidentifier() -> *const i8;
    fn _Py_gitversion() -> *const i8;
    fn _PyStructSequence_InitBuiltinWithFlags(
        arg0: *mut _is,
        arg1: *mut _typeobject,
        arg2: *mut PyStructSequence_Desc,
        arg3: u64,
    ) -> i32;
    fn _PyStructSequence_FiniBuiltin(arg0: *mut _is, arg1: *mut _typeobject);
    fn _PySysRemoteDebug_SendExec(arg0: i32, arg1: i32, arg2: *const i8) -> i32;
    fn _PyUnicode_InternedSize() -> i64;
    fn _PyUnicode_InternedSize_Immortal() -> i64;
    fn _PyUnicode_InternMortal(arg0: *mut _is, arg1: *mut *mut _object);
    fn _PyUnicode_InternImmortal(arg0: *mut _is, arg1: *mut *mut _object);
    #[link_name = "open64"]
    fn open(arg0: *const i8, arg1: i32, ...) -> i32;
    fn _PyTuple_FromArray(arg0: *const *mut _object, arg1: i64) -> *mut _object;
    fn _Py_CreateMonitoringObject() -> *mut _object;
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

fn _Py_atomic_load_int(obj: *const i32) -> i32 {
    unsafe {
        core::sync::atomic::AtomicI32::from_ptr(obj as *mut i32)
            .load(core::sync::atomic::Ordering::SeqCst)
    }
}

fn _Py_atomic_load_ptr_relaxed(obj: *const ()) -> *mut () {
    unsafe {
        core::sync::atomic::AtomicPtr::from_ptr(obj as *mut *mut ())
            .load(core::sync::atomic::Ordering::Relaxed) as *mut ()
    }
}

fn _Py_atomic_store_int(obj: *mut i32, value: i32) {
    unsafe {
        core::sync::atomic::AtomicI32::from_ptr(obj as *mut i32)
            .store(value, core::sync::atomic::Ordering::SeqCst)
    };
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

// Flag bits for printing:
// No string quotes etc.
// Track types initialized using _PyStaticType_InitBuiltin().
// Set if the type object is immutable: type attributes cannot be set nor deleted
// Set if the type object is dynamically allocated
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

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_unicodeobject_h_163_2 {
    __slate_bits_0: __slate_bits::__SlateBits147U0,
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
struct __SlateRecord_objimpl_h_68 {
    ctx: *mut (),
    alloc: Option<unsafe extern "C-unwind" fn(*mut (), u64) -> *mut ()>,
    free: Option<unsafe extern "C-unwind" fn(*mut (), *mut (), u64)>,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pyhash_h_39 {
    hash: Option<unsafe extern "C-unwind" fn(*const (), i64) -> i64>,
    name: *const i8,
    hash_bits: i32,
    seed_bits: i32,
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

fn PyUnicode_CHECK_INTERNED(op: *mut _object) -> u32 {
    ((unsafe {
        (*_PyASCIIObject_CAST!(op))
            .state
            .__slate_bits_0
            .__get_interned()
    }) as i32) as u32
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
struct __SlateRecord_floatobject_h_8 {
    ob_base: _object,
    ob_fval: f64,
}

fn PyFloat_AS_DOUBLE(op: *mut _object) -> f64 {
    unsafe { (*_PyFloat_CAST!(op)).ob_fval }
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
struct __SlateRecord_pystate_h_78 {
    __slate_bits_0: __slate_bits::__SlateBits200U0,
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

fn PyList_SET_ITEM(op: *mut _object, index: i64, value: *mut _object) {
    let list: *mut __SlateRecord_listobject_h_22 = _PyList_CAST!(op);
    unsafe {
        *unsafe { (*list).ob_item }.wrapping_offset(index as isize) = value;
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

// Flag passed to newmethodobject
//
// #define METH_OLDARGS  0x0000   -- unsupported now
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
struct __SlateRecord_pycore_structs_h_32_2 {
    code: u8,
    arg: u8,
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

// Remote debugger support
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
struct PyStructSequence_Field {
    name: *const i8,
    doc: *const i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct PyStructSequence_Desc {
    name: *const i8,
    doc: *const i8,
    fields: *mut PyStructSequence_Field,
    n_in_sequence: i32,
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
struct __SlateRecord_ceval_h_33 {
    perf_map: *mut slate_support::FILE,
    map_lock: *mut (),
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
struct code_arena_st {}

fn PyObject_CallMethodOneArg(
    self_: *mut _object,
    name: *mut _object,
    arg: *mut _object,
) -> *mut _object {
    let mut args: __SlateAlign16<[*mut _object; 2]> = __SlateAlign16([self_, arg]);
    let nargsf: u64 = 2 | _Py_STATIC_CAST!(u64, 1i32)
        << 8u64
            .wrapping_mul(core::mem::size_of::<u64>() as u64)
            .wrapping_sub(1);
    unsafe {
        PyObject_VectorcallMethod(
            name,
            args.0.as_mut_ptr() as *const *mut _object,
            nargsf,
            core::ptr::null_mut::<_object>(),
        )
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _Py_AuditHookEntry {
    next: *mut _Py_AuditHookEntry,
    hookCFunction: Option<unsafe extern "C-unwind" fn(*const i8, *mut _object, *mut ()) -> i32>,
    userData: *mut (),
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

fn _PyThreadState_GET() -> *mut _ts {
    unsafe { _Py_tss_tstate }
}

fn _Py_EnsureFuncTstateNotNULL(func: *const i8, tstate: *mut _ts) {
    if tstate.is_null() {
        unsafe {
            _Py_FatalErrorFunc(func, c"the function must be called with the GIL held, after Python initialization and before Python finalization, but the GIL is released (the current Python thread state is NULL)".as_ptr())
        };
        unsafe { core::hint::unreachable_unchecked() };
    }
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
struct xi_exceptions {
    PyExc_InterpreterError: *mut _object,
    PyExc_InterpreterNotFoundError: *mut _object,
    PyExc_NotShareableError: *mut _object,
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
struct __SlateRecord_pycore_interp_structs_h_718 {
    _not_used: i32,
    _hamt_empty_gc_not_used: __SlateRecord_pycore_interp_structs_h_170,
    hamt_empty: __SlateRecord_pycore_structs_h_47,
    last_resort_memory_error: __SlateRecord_pyerrors_h_15,
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
union __SlateRecord_pycore_obmalloc_h_255 {
    _padding: *mut u8,
    count: u32,
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
struct __SlateRecord_pycore_runtime_structs_h_49 {
    fd: i32,
    st_dev: u64,
    st_ino: u64,
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
struct __SlateRecord_pycore_tracemalloc_h_70 {
    mem: __SlateRecord_pymem_h_48,
    raw: __SlateRecord_pymem_h_48,
    obj: __SlateRecord_pymem_h_48,
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
union __SlateRecord_sigaction_h_31 {
    sa_handler: Option<unsafe extern "C-unwind" fn(i32)>,
    sa_sigaction:
        Option<unsafe extern "C-unwind" fn(i32, *mut __SlateRecord_siginfo_t_h_124, *mut ())>,
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
struct __SlateRecord_pycore_ceval_h_102 {
    init_state: Option<unsafe extern "C-unwind" fn() -> *mut ()>,
    write_state: Option<unsafe extern "C-unwind" fn(*mut (), *const (), u32, *mut PyCodeObject)>,
    free_state: Option<unsafe extern "C-unwind" fn(*mut ()) -> i32>,
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
struct _obmalloc_global_state {
    dump_debug_stats: i32,
    interpreter_leaks: i64,
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
struct pyhash_runtime_state {
    urandom_cache: __SlateRecord_pycore_runtime_structs_h_49,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_sysmodule_c_h_36 {
    _this_is_not_used: __SlateRecord_pycore_interp_structs_h_170,
    ob_base: __SlateRecord_object_h_172,
    ob_hash: i64,
    ob_item: [*mut _object; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_sysmodule_c_h_626 {
    _this_is_not_used: __SlateRecord_pycore_interp_structs_h_170,
    ob_base: __SlateRecord_object_h_172,
    ob_hash: i64,
    ob_item: [*mut _object; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_sysmodule_c_h_930 {
    _this_is_not_used: __SlateRecord_pycore_interp_structs_h_170,
    ob_base: __SlateRecord_object_h_172,
    ob_hash: i64,
    ob_item: [*mut _object; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_sysmodule_c_h_1082 {
    _this_is_not_used: __SlateRecord_pycore_interp_structs_h_170,
    ob_base: __SlateRecord_object_h_172,
    ob_hash: i64,
    ob_item: [*mut _object; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_sysmodule_c_h_1582 {
    _this_is_not_used: __SlateRecord_pycore_interp_structs_h_170,
    ob_base: __SlateRecord_object_h_172,
    ob_hash: i64,
    ob_item: [*mut _object; 2],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_sysmodule_c_h_1645 {
    _this_is_not_used: __SlateRecord_pycore_interp_structs_h_170,
    ob_base: __SlateRecord_object_h_172,
    ob_hash: i64,
    ob_item: [*mut _object; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_sysmodule_c_h_1709 {
    _this_is_not_used: __SlateRecord_pycore_interp_structs_h_170,
    ob_base: __SlateRecord_object_h_172,
    ob_hash: i64,
    ob_item: [*mut _object; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _PyTraceMalloc_Config {
    initialized: u32,
    tracing: i32,
    max_nframe: i32,
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
    pub struct __SlateBits1323U0 {
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
    pub struct __SlateBits1321U0 {
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
    pub struct __SlateBits200U0 {
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
// _Py_DECLARE_STR() should precede all uses of _Py_STR() in a function.
//
// This is true even if the same string has already been declared
// elsewhere, even in the same file.  Mismatched duplicates are detected
// by Tools/scripts/generate-global-objects.py.
//
// Pairing _Py_DECLARE_STR() with every use of _Py_STR() makes sure the
// string keeps working even if the declaration is removed somewhere
// else.  It also makes it clear what the actual string is at every
// place it is being used.
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

fn _PyRuntimeState_GetFinalizing(runtime: *mut pyruntimestate) -> *mut _ts {
    _Py_atomic_load_ptr_relaxed(
        (unsafe { core::ptr::addr_of_mut!((*runtime)._finalizing) }) as *const (),
    ) as *mut _ts
}

fn _PyType_HasFeature(r#type: *mut _typeobject, feature: u64) -> i32 {
    ((unsafe { (*r#type).tp_flags }) & feature != 0) as i32
}

fn _PyType_PreHeaderSize(tp: *mut _typeobject) -> u64 {
    (_PyType_IS_GC!(tp) as u64)
        .wrapping_mul(core::mem::size_of::<__SlateRecord_pycore_interp_structs_h_170>() as u64)
        .wrapping_add(
            (_PyType_HasFeature(tp, 24u64) as u64)
                .wrapping_mul(2)
                .wrapping_mul(core::mem::size_of::<*mut _object>() as u64),
        )
}

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

fn _PyFrame_IsIncomplete(frame: *mut _PyInterpreterFrame) -> bool {
    if ((unsafe { (*frame).owner }) as i32) >= 3 {
        return true;
    }
    ((unsafe { (*frame).owner }) as i32) != 1
        && (unsafe { (*frame).instr_ptr })
            < _PyFrame_GetBytecode(frame).wrapping_offset(
                (unsafe { (*_PyFrame_GetCode(frame))._co_firsttraceable }) as isize,
            )
}

fn _PyFrame_GetFirstComplete(mut frame: *mut _PyInterpreterFrame) -> *mut _PyInterpreterFrame {
    loop {
        let __v0: bool = !frame.is_null() && _PyFrame_IsIncomplete(frame);
        if !__v0 {
            break;
        }
        frame = unsafe { (*frame).previous };
    }
    frame
}

fn _PyFrame_GetFrameObject(frame: *mut _PyInterpreterFrame) -> *mut _frame {
    let res: *mut _frame = unsafe { (*frame).frame_obj };
    if !res.is_null() {
        return res;
    }
    unsafe { _PyFrame_MakeAndSetFrameObject(frame) }
}

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
    __slate_bits_0: __slate_bits::__SlateBits1321U0,
    warm: bool,
    index: i32,
    bloom: __SlateRecord_pycore_optimizer_h_28,
    links: _PyExecutorLinkListNode,
    code: *mut PyCodeObject,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_optimizer_h_67 {
    __slate_bits_0: __slate_bits::__SlateBits1323U0,
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

// Holds locals, stack, locals, stack ... co_consts (in that order)
// Need extras for root frame and for overflow frame (see TRACE_STACK_PUSH())
fn _PyStructSequence_InitBuiltin(
    interp: *mut _is,
    r#type: *mut _typeobject,
    desc: *mut PyStructSequence_Desc,
) -> i32 {
    unsafe { _PyStructSequence_InitBuiltinWithFlags(interp, r#type, desc, 0u64) }
}

fn PyDTrace_AUDIT(arg0: *const i8, arg1: *mut ()) {}

fn PyDTrace_AUDIT_ENABLED() -> i32 {
    0
}

static mut _Py_stdlib_module_names: __SlateAlign16<[*const i8; 297]> = __SlateAlign16([
    (b"__future__\0".as_ptr() as *mut i8) as *const i8,
    (b"_abc\0".as_ptr() as *mut i8) as *const i8,
    (b"_aix_support\0".as_ptr() as *mut i8) as *const i8,
    (b"_android_support\0".as_ptr() as *mut i8) as *const i8,
    (b"_apple_support\0".as_ptr() as *mut i8) as *const i8,
    (b"_ast\0".as_ptr() as *mut i8) as *const i8,
    (b"_ast_unparse\0".as_ptr() as *mut i8) as *const i8,
    (b"_asyncio\0".as_ptr() as *mut i8) as *const i8,
    (b"_bisect\0".as_ptr() as *mut i8) as *const i8,
    (b"_blake2\0".as_ptr() as *mut i8) as *const i8,
    (b"_bz2\0".as_ptr() as *mut i8) as *const i8,
    (b"_codecs\0".as_ptr() as *mut i8) as *const i8,
    (b"_codecs_cn\0".as_ptr() as *mut i8) as *const i8,
    (b"_codecs_hk\0".as_ptr() as *mut i8) as *const i8,
    (b"_codecs_iso2022\0".as_ptr() as *mut i8) as *const i8,
    (b"_codecs_jp\0".as_ptr() as *mut i8) as *const i8,
    (b"_codecs_kr\0".as_ptr() as *mut i8) as *const i8,
    (b"_codecs_tw\0".as_ptr() as *mut i8) as *const i8,
    (b"_collections\0".as_ptr() as *mut i8) as *const i8,
    (b"_collections_abc\0".as_ptr() as *mut i8) as *const i8,
    (b"_colorize\0".as_ptr() as *mut i8) as *const i8,
    (b"_compat_pickle\0".as_ptr() as *mut i8) as *const i8,
    (b"_contextvars\0".as_ptr() as *mut i8) as *const i8,
    (b"_csv\0".as_ptr() as *mut i8) as *const i8,
    (b"_ctypes\0".as_ptr() as *mut i8) as *const i8,
    (b"_curses\0".as_ptr() as *mut i8) as *const i8,
    (b"_curses_panel\0".as_ptr() as *mut i8) as *const i8,
    (b"_datetime\0".as_ptr() as *mut i8) as *const i8,
    (b"_dbm\0".as_ptr() as *mut i8) as *const i8,
    (b"_decimal\0".as_ptr() as *mut i8) as *const i8,
    (b"_elementtree\0".as_ptr() as *mut i8) as *const i8,
    (b"_frozen_importlib\0".as_ptr() as *mut i8) as *const i8,
    (b"_frozen_importlib_external\0".as_ptr() as *mut i8) as *const i8,
    (b"_functools\0".as_ptr() as *mut i8) as *const i8,
    (b"_gdbm\0".as_ptr() as *mut i8) as *const i8,
    (b"_hashlib\0".as_ptr() as *mut i8) as *const i8,
    (b"_heapq\0".as_ptr() as *mut i8) as *const i8,
    (b"_hmac\0".as_ptr() as *mut i8) as *const i8,
    (b"_imp\0".as_ptr() as *mut i8) as *const i8,
    (b"_interpchannels\0".as_ptr() as *mut i8) as *const i8,
    (b"_interpqueues\0".as_ptr() as *mut i8) as *const i8,
    (b"_interpreters\0".as_ptr() as *mut i8) as *const i8,
    (b"_io\0".as_ptr() as *mut i8) as *const i8,
    (b"_ios_support\0".as_ptr() as *mut i8) as *const i8,
    (b"_json\0".as_ptr() as *mut i8) as *const i8,
    (b"_locale\0".as_ptr() as *mut i8) as *const i8,
    (b"_lsprof\0".as_ptr() as *mut i8) as *const i8,
    (b"_lzma\0".as_ptr() as *mut i8) as *const i8,
    (b"_markupbase\0".as_ptr() as *mut i8) as *const i8,
    (b"_md5\0".as_ptr() as *mut i8) as *const i8,
    (b"_multibytecodec\0".as_ptr() as *mut i8) as *const i8,
    (b"_multiprocessing\0".as_ptr() as *mut i8) as *const i8,
    (b"_opcode\0".as_ptr() as *mut i8) as *const i8,
    (b"_opcode_metadata\0".as_ptr() as *mut i8) as *const i8,
    (b"_operator\0".as_ptr() as *mut i8) as *const i8,
    (b"_osx_support\0".as_ptr() as *mut i8) as *const i8,
    (b"_overlapped\0".as_ptr() as *mut i8) as *const i8,
    (b"_pickle\0".as_ptr() as *mut i8) as *const i8,
    (b"_posixshmem\0".as_ptr() as *mut i8) as *const i8,
    (b"_posixsubprocess\0".as_ptr() as *mut i8) as *const i8,
    (b"_py_abc\0".as_ptr() as *mut i8) as *const i8,
    (b"_py_warnings\0".as_ptr() as *mut i8) as *const i8,
    (b"_pydatetime\0".as_ptr() as *mut i8) as *const i8,
    (b"_pydecimal\0".as_ptr() as *mut i8) as *const i8,
    (b"_pyio\0".as_ptr() as *mut i8) as *const i8,
    (b"_pylong\0".as_ptr() as *mut i8) as *const i8,
    (b"_pyrepl\0".as_ptr() as *mut i8) as *const i8,
    (b"_queue\0".as_ptr() as *mut i8) as *const i8,
    (b"_random\0".as_ptr() as *mut i8) as *const i8,
    (b"_remote_debugging\0".as_ptr() as *mut i8) as *const i8,
    (b"_scproxy\0".as_ptr() as *mut i8) as *const i8,
    (b"_sha1\0".as_ptr() as *mut i8) as *const i8,
    (b"_sha2\0".as_ptr() as *mut i8) as *const i8,
    (b"_sha3\0".as_ptr() as *mut i8) as *const i8,
    (b"_signal\0".as_ptr() as *mut i8) as *const i8,
    (b"_sitebuiltins\0".as_ptr() as *mut i8) as *const i8,
    (b"_socket\0".as_ptr() as *mut i8) as *const i8,
    (b"_sqlite3\0".as_ptr() as *mut i8) as *const i8,
    (b"_sre\0".as_ptr() as *mut i8) as *const i8,
    (b"_ssl\0".as_ptr() as *mut i8) as *const i8,
    (b"_stat\0".as_ptr() as *mut i8) as *const i8,
    (b"_statistics\0".as_ptr() as *mut i8) as *const i8,
    (b"_string\0".as_ptr() as *mut i8) as *const i8,
    (b"_strptime\0".as_ptr() as *mut i8) as *const i8,
    (b"_struct\0".as_ptr() as *mut i8) as *const i8,
    (b"_suggestions\0".as_ptr() as *mut i8) as *const i8,
    (b"_symtable\0".as_ptr() as *mut i8) as *const i8,
    (b"_sysconfig\0".as_ptr() as *mut i8) as *const i8,
    (b"_thread\0".as_ptr() as *mut i8) as *const i8,
    (b"_threading_local\0".as_ptr() as *mut i8) as *const i8,
    (b"_tkinter\0".as_ptr() as *mut i8) as *const i8,
    (b"_tokenize\0".as_ptr() as *mut i8) as *const i8,
    (b"_tracemalloc\0".as_ptr() as *mut i8) as *const i8,
    (b"_types\0".as_ptr() as *mut i8) as *const i8,
    (b"_typing\0".as_ptr() as *mut i8) as *const i8,
    (b"_uuid\0".as_ptr() as *mut i8) as *const i8,
    (b"_warnings\0".as_ptr() as *mut i8) as *const i8,
    (b"_weakref\0".as_ptr() as *mut i8) as *const i8,
    (b"_weakrefset\0".as_ptr() as *mut i8) as *const i8,
    (b"_winapi\0".as_ptr() as *mut i8) as *const i8,
    (b"_wmi\0".as_ptr() as *mut i8) as *const i8,
    (b"_zoneinfo\0".as_ptr() as *mut i8) as *const i8,
    (b"_zstd\0".as_ptr() as *mut i8) as *const i8,
    (b"abc\0".as_ptr() as *mut i8) as *const i8,
    (b"annotationlib\0".as_ptr() as *mut i8) as *const i8,
    (b"antigravity\0".as_ptr() as *mut i8) as *const i8,
    (b"argparse\0".as_ptr() as *mut i8) as *const i8,
    (b"array\0".as_ptr() as *mut i8) as *const i8,
    (b"ast\0".as_ptr() as *mut i8) as *const i8,
    (b"asyncio\0".as_ptr() as *mut i8) as *const i8,
    (b"atexit\0".as_ptr() as *mut i8) as *const i8,
    (b"base64\0".as_ptr() as *mut i8) as *const i8,
    (b"bdb\0".as_ptr() as *mut i8) as *const i8,
    (b"binascii\0".as_ptr() as *mut i8) as *const i8,
    (b"bisect\0".as_ptr() as *mut i8) as *const i8,
    (b"builtins\0".as_ptr() as *mut i8) as *const i8,
    (b"bz2\0".as_ptr() as *mut i8) as *const i8,
    (b"cProfile\0".as_ptr() as *mut i8) as *const i8,
    (b"calendar\0".as_ptr() as *mut i8) as *const i8,
    (b"cmath\0".as_ptr() as *mut i8) as *const i8,
    (b"cmd\0".as_ptr() as *mut i8) as *const i8,
    (b"code\0".as_ptr() as *mut i8) as *const i8,
    (b"codecs\0".as_ptr() as *mut i8) as *const i8,
    (b"codeop\0".as_ptr() as *mut i8) as *const i8,
    (b"collections\0".as_ptr() as *mut i8) as *const i8,
    (b"colorsys\0".as_ptr() as *mut i8) as *const i8,
    (b"compileall\0".as_ptr() as *mut i8) as *const i8,
    (b"compression\0".as_ptr() as *mut i8) as *const i8,
    (b"concurrent\0".as_ptr() as *mut i8) as *const i8,
    (b"configparser\0".as_ptr() as *mut i8) as *const i8,
    (b"contextlib\0".as_ptr() as *mut i8) as *const i8,
    (b"contextvars\0".as_ptr() as *mut i8) as *const i8,
    (b"copy\0".as_ptr() as *mut i8) as *const i8,
    (b"copyreg\0".as_ptr() as *mut i8) as *const i8,
    (b"csv\0".as_ptr() as *mut i8) as *const i8,
    (b"ctypes\0".as_ptr() as *mut i8) as *const i8,
    (b"curses\0".as_ptr() as *mut i8) as *const i8,
    (b"dataclasses\0".as_ptr() as *mut i8) as *const i8,
    (b"datetime\0".as_ptr() as *mut i8) as *const i8,
    (b"dbm\0".as_ptr() as *mut i8) as *const i8,
    (b"decimal\0".as_ptr() as *mut i8) as *const i8,
    (b"difflib\0".as_ptr() as *mut i8) as *const i8,
    (b"dis\0".as_ptr() as *mut i8) as *const i8,
    (b"doctest\0".as_ptr() as *mut i8) as *const i8,
    (b"email\0".as_ptr() as *mut i8) as *const i8,
    (b"encodings\0".as_ptr() as *mut i8) as *const i8,
    (b"ensurepip\0".as_ptr() as *mut i8) as *const i8,
    (b"enum\0".as_ptr() as *mut i8) as *const i8,
    (b"errno\0".as_ptr() as *mut i8) as *const i8,
    (b"faulthandler\0".as_ptr() as *mut i8) as *const i8,
    (b"fcntl\0".as_ptr() as *mut i8) as *const i8,
    (b"filecmp\0".as_ptr() as *mut i8) as *const i8,
    (b"fileinput\0".as_ptr() as *mut i8) as *const i8,
    (b"fnmatch\0".as_ptr() as *mut i8) as *const i8,
    (b"fractions\0".as_ptr() as *mut i8) as *const i8,
    (b"ftplib\0".as_ptr() as *mut i8) as *const i8,
    (b"functools\0".as_ptr() as *mut i8) as *const i8,
    (b"gc\0".as_ptr() as *mut i8) as *const i8,
    (b"genericpath\0".as_ptr() as *mut i8) as *const i8,
    (b"getopt\0".as_ptr() as *mut i8) as *const i8,
    (b"getpass\0".as_ptr() as *mut i8) as *const i8,
    (b"gettext\0".as_ptr() as *mut i8) as *const i8,
    (b"glob\0".as_ptr() as *mut i8) as *const i8,
    (b"graphlib\0".as_ptr() as *mut i8) as *const i8,
    (b"grp\0".as_ptr() as *mut i8) as *const i8,
    (b"gzip\0".as_ptr() as *mut i8) as *const i8,
    (b"hashlib\0".as_ptr() as *mut i8) as *const i8,
    (b"heapq\0".as_ptr() as *mut i8) as *const i8,
    (b"hmac\0".as_ptr() as *mut i8) as *const i8,
    (b"html\0".as_ptr() as *mut i8) as *const i8,
    (b"http\0".as_ptr() as *mut i8) as *const i8,
    (b"idlelib\0".as_ptr() as *mut i8) as *const i8,
    (b"imaplib\0".as_ptr() as *mut i8) as *const i8,
    (b"importlib\0".as_ptr() as *mut i8) as *const i8,
    (b"inspect\0".as_ptr() as *mut i8) as *const i8,
    (b"io\0".as_ptr() as *mut i8) as *const i8,
    (b"ipaddress\0".as_ptr() as *mut i8) as *const i8,
    (b"itertools\0".as_ptr() as *mut i8) as *const i8,
    (b"json\0".as_ptr() as *mut i8) as *const i8,
    (b"keyword\0".as_ptr() as *mut i8) as *const i8,
    (b"linecache\0".as_ptr() as *mut i8) as *const i8,
    (b"locale\0".as_ptr() as *mut i8) as *const i8,
    (b"logging\0".as_ptr() as *mut i8) as *const i8,
    (b"lzma\0".as_ptr() as *mut i8) as *const i8,
    (b"mailbox\0".as_ptr() as *mut i8) as *const i8,
    (b"marshal\0".as_ptr() as *mut i8) as *const i8,
    (b"math\0".as_ptr() as *mut i8) as *const i8,
    (b"mimetypes\0".as_ptr() as *mut i8) as *const i8,
    (b"mmap\0".as_ptr() as *mut i8) as *const i8,
    (b"modulefinder\0".as_ptr() as *mut i8) as *const i8,
    (b"msvcrt\0".as_ptr() as *mut i8) as *const i8,
    (b"multiprocessing\0".as_ptr() as *mut i8) as *const i8,
    (b"netrc\0".as_ptr() as *mut i8) as *const i8,
    (b"nt\0".as_ptr() as *mut i8) as *const i8,
    (b"ntpath\0".as_ptr() as *mut i8) as *const i8,
    (b"nturl2path\0".as_ptr() as *mut i8) as *const i8,
    (b"numbers\0".as_ptr() as *mut i8) as *const i8,
    (b"opcode\0".as_ptr() as *mut i8) as *const i8,
    (b"operator\0".as_ptr() as *mut i8) as *const i8,
    (b"optparse\0".as_ptr() as *mut i8) as *const i8,
    (b"os\0".as_ptr() as *mut i8) as *const i8,
    (b"pathlib\0".as_ptr() as *mut i8) as *const i8,
    (b"pdb\0".as_ptr() as *mut i8) as *const i8,
    (b"pickle\0".as_ptr() as *mut i8) as *const i8,
    (b"pickletools\0".as_ptr() as *mut i8) as *const i8,
    (b"pkgutil\0".as_ptr() as *mut i8) as *const i8,
    (b"platform\0".as_ptr() as *mut i8) as *const i8,
    (b"plistlib\0".as_ptr() as *mut i8) as *const i8,
    (b"poplib\0".as_ptr() as *mut i8) as *const i8,
    (b"posix\0".as_ptr() as *mut i8) as *const i8,
    (b"posixpath\0".as_ptr() as *mut i8) as *const i8,
    (b"pprint\0".as_ptr() as *mut i8) as *const i8,
    (b"profile\0".as_ptr() as *mut i8) as *const i8,
    (b"pstats\0".as_ptr() as *mut i8) as *const i8,
    (b"pty\0".as_ptr() as *mut i8) as *const i8,
    (b"pwd\0".as_ptr() as *mut i8) as *const i8,
    (b"py_compile\0".as_ptr() as *mut i8) as *const i8,
    (b"pyclbr\0".as_ptr() as *mut i8) as *const i8,
    (b"pydoc\0".as_ptr() as *mut i8) as *const i8,
    (b"pydoc_data\0".as_ptr() as *mut i8) as *const i8,
    (b"pyexpat\0".as_ptr() as *mut i8) as *const i8,
    (b"queue\0".as_ptr() as *mut i8) as *const i8,
    (b"quopri\0".as_ptr() as *mut i8) as *const i8,
    (b"random\0".as_ptr() as *mut i8) as *const i8,
    (b"re\0".as_ptr() as *mut i8) as *const i8,
    (b"readline\0".as_ptr() as *mut i8) as *const i8,
    (b"reprlib\0".as_ptr() as *mut i8) as *const i8,
    (b"resource\0".as_ptr() as *mut i8) as *const i8,
    (b"rlcompleter\0".as_ptr() as *mut i8) as *const i8,
    (b"runpy\0".as_ptr() as *mut i8) as *const i8,
    (b"sched\0".as_ptr() as *mut i8) as *const i8,
    (b"secrets\0".as_ptr() as *mut i8) as *const i8,
    (b"select\0".as_ptr() as *mut i8) as *const i8,
    (b"selectors\0".as_ptr() as *mut i8) as *const i8,
    (b"shelve\0".as_ptr() as *mut i8) as *const i8,
    (b"shlex\0".as_ptr() as *mut i8) as *const i8,
    (b"shutil\0".as_ptr() as *mut i8) as *const i8,
    (b"signal\0".as_ptr() as *mut i8) as *const i8,
    (b"site\0".as_ptr() as *mut i8) as *const i8,
    (b"smtplib\0".as_ptr() as *mut i8) as *const i8,
    (b"socket\0".as_ptr() as *mut i8) as *const i8,
    (b"socketserver\0".as_ptr() as *mut i8) as *const i8,
    (b"sqlite3\0".as_ptr() as *mut i8) as *const i8,
    (b"sre_compile\0".as_ptr() as *mut i8) as *const i8,
    (b"sre_constants\0".as_ptr() as *mut i8) as *const i8,
    (b"sre_parse\0".as_ptr() as *mut i8) as *const i8,
    (b"ssl\0".as_ptr() as *mut i8) as *const i8,
    (b"stat\0".as_ptr() as *mut i8) as *const i8,
    (b"statistics\0".as_ptr() as *mut i8) as *const i8,
    (b"string\0".as_ptr() as *mut i8) as *const i8,
    (b"stringprep\0".as_ptr() as *mut i8) as *const i8,
    (b"struct\0".as_ptr() as *mut i8) as *const i8,
    (b"subprocess\0".as_ptr() as *mut i8) as *const i8,
    (b"symtable\0".as_ptr() as *mut i8) as *const i8,
    (b"sys\0".as_ptr() as *mut i8) as *const i8,
    (b"sysconfig\0".as_ptr() as *mut i8) as *const i8,
    (b"syslog\0".as_ptr() as *mut i8) as *const i8,
    (b"tabnanny\0".as_ptr() as *mut i8) as *const i8,
    (b"tarfile\0".as_ptr() as *mut i8) as *const i8,
    (b"tempfile\0".as_ptr() as *mut i8) as *const i8,
    (b"termios\0".as_ptr() as *mut i8) as *const i8,
    (b"textwrap\0".as_ptr() as *mut i8) as *const i8,
    (b"this\0".as_ptr() as *mut i8) as *const i8,
    (b"threading\0".as_ptr() as *mut i8) as *const i8,
    (b"time\0".as_ptr() as *mut i8) as *const i8,
    (b"timeit\0".as_ptr() as *mut i8) as *const i8,
    (b"tkinter\0".as_ptr() as *mut i8) as *const i8,
    (b"token\0".as_ptr() as *mut i8) as *const i8,
    (b"tokenize\0".as_ptr() as *mut i8) as *const i8,
    (b"tomllib\0".as_ptr() as *mut i8) as *const i8,
    (b"trace\0".as_ptr() as *mut i8) as *const i8,
    (b"traceback\0".as_ptr() as *mut i8) as *const i8,
    (b"tracemalloc\0".as_ptr() as *mut i8) as *const i8,
    (b"tty\0".as_ptr() as *mut i8) as *const i8,
    (b"turtle\0".as_ptr() as *mut i8) as *const i8,
    (b"turtledemo\0".as_ptr() as *mut i8) as *const i8,
    (b"types\0".as_ptr() as *mut i8) as *const i8,
    (b"typing\0".as_ptr() as *mut i8) as *const i8,
    (b"unicodedata\0".as_ptr() as *mut i8) as *const i8,
    (b"unittest\0".as_ptr() as *mut i8) as *const i8,
    (b"urllib\0".as_ptr() as *mut i8) as *const i8,
    (b"uuid\0".as_ptr() as *mut i8) as *const i8,
    (b"venv\0".as_ptr() as *mut i8) as *const i8,
    (b"warnings\0".as_ptr() as *mut i8) as *const i8,
    (b"wave\0".as_ptr() as *mut i8) as *const i8,
    (b"weakref\0".as_ptr() as *mut i8) as *const i8,
    (b"webbrowser\0".as_ptr() as *mut i8) as *const i8,
    (b"winreg\0".as_ptr() as *mut i8) as *const i8,
    (b"winsound\0".as_ptr() as *mut i8) as *const i8,
    (b"wsgiref\0".as_ptr() as *mut i8) as *const i8,
    (b"xml\0".as_ptr() as *mut i8) as *const i8,
    (b"xmlrpc\0".as_ptr() as *mut i8) as *const i8,
    (b"zipapp\0".as_ptr() as *mut i8) as *const i8,
    (b"zipfile\0".as_ptr() as *mut i8) as *const i8,
    (b"zipimport\0".as_ptr() as *mut i8) as *const i8,
    (b"zlib\0".as_ptr() as *mut i8) as *const i8,
    (b"zoneinfo\0".as_ptr() as *mut i8) as *const i8,
]);

static sys_addaudithook__doc__: __SlateAlign16<[i8; 67]> = __SlateAlign16([
    97i8, 100i8, 100i8, 97i8, 117i8, 100i8, 105i8, 116i8, 104i8, 111i8, 111i8, 107i8, 40i8, 36i8,
    109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 44i8, 32i8, 47i8, 44i8, 32i8, 104i8, 111i8, 111i8,
    107i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 65i8, 100i8, 100i8, 115i8, 32i8, 97i8, 32i8, 110i8,
    101i8, 119i8, 32i8, 97i8, 117i8, 100i8, 105i8, 116i8, 32i8, 104i8, 111i8, 111i8, 107i8, 32i8,
    99i8, 97i8, 108i8, 108i8, 98i8, 97i8, 99i8, 107i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.sysmodule.sys_addaudithook")]
extern "C-unwind" fn sys_addaudithook(
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
            core::ptr::addr_of_mut!(_parser__49),
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
            let hook: *mut _object = *args.wrapping_offset(0 as isize);
            return_value = sys_addaudithook_impl(module, hook);
        }
        return return_value;
    }
}

static mut _kwtuple__36: __SlateRecord_sysmodule_c_h_36 = {
    let mut __t0: __SlateRecord_sysmodule_c_h_36 = unsafe { core::mem::zeroed() };
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
                ._py_hook
                ._ascii
                .ob_base
        )
    }];
    __t0
};

static _keywords__48: __SlateAlign16<[*const i8; 2]> = __SlateAlign16([
    (b"hook\0".as_ptr() as *mut i8) as *const i8,
    core::ptr::null::<i8>(),
]);

static mut _parser__49: _PyArg_Parser = {
    let mut __t0: _PyArg_Parser = unsafe { core::mem::zeroed() };
    __t0.keywords = unsafe { core::ptr::addr_of!(_keywords__48.0) as *const *const i8 };
    __t0.fname = (b"addaudithook\0".as_ptr() as *mut i8) as *const i8;
    __t0.kwtuple = unsafe { core::ptr::addr_of_mut!(_kwtuple__36.ob_base.ob_base) };
    __t0
};

static sys_audit__doc__: __SlateAlign16<[i8; 91]> = __SlateAlign16([
    97i8, 117i8, 100i8, 105i8, 116i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 44i8,
    32i8, 101i8, 118i8, 101i8, 110i8, 116i8, 44i8, 32i8, 47i8, 44i8, 32i8, 42i8, 97i8, 114i8,
    103i8, 115i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 80i8, 97i8, 115i8, 115i8, 101i8, 115i8, 32i8,
    116i8, 104i8, 101i8, 32i8, 101i8, 118i8, 101i8, 110i8, 116i8, 32i8, 116i8, 111i8, 32i8, 97i8,
    110i8, 121i8, 32i8, 97i8, 117i8, 100i8, 105i8, 116i8, 32i8, 104i8, 111i8, 111i8, 107i8, 115i8,
    32i8, 116i8, 104i8, 97i8, 116i8, 32i8, 97i8, 114i8, 101i8, 32i8, 97i8, 116i8, 116i8, 97i8,
    99i8, 104i8, 101i8, 100i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.sysmodule.sys_audit")]
extern "C-unwind" fn sys_audit(
    module: *mut _object,
    args: *const *mut _object,
    nargs: i64,
) -> *mut _object {
    let mut event_length: i64 = unsafe { core::mem::zeroed() };
    let mut __clinic_args: *mut _object = unsafe { core::mem::zeroed() };
    let mut event: *const i8 = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        __clinic_args = core::ptr::null_mut::<_object>();
        if _PyArg_CheckPositional__3!(
            c"audit".as_ptr() as *mut i8,
            nargs,
            1i32,
            9223372036854775807i64
        ) {
            if PyUnicode_Check!(unsafe { *args.wrapping_offset(0 as isize) }) == 0 {
                _PyArg_BadArgument(
                    c"audit".as_ptr(),
                    c"argument 1".as_ptr(),
                    c"str".as_ptr(),
                    *args.wrapping_offset(0 as isize),
                );
            } else {
                event = PyUnicode_AsUTF8AndSize(
                    *args.wrapping_offset(0 as isize),
                    core::ptr::addr_of_mut!(event_length),
                );
                if !event.is_null() {
                    if (strlen(event as *const core::ffi::c_char) as u64) != (event_length as u64) {
                        PyErr_SetString(PyExc_ValueError, c"embedded null character".as_ptr());
                    } else {
                        __clinic_args = _PyTuple_FromArray(
                            args.wrapping_offset(1 as isize),
                            nargs.wrapping_sub(1),
                        );
                        if !__clinic_args.is_null() {
                            return_value = sys_audit_impl(module, event, __clinic_args);
                        }
                    }
                }
            }
        }
        Py_XDECREF!(__clinic_args); // Cleanup for args
        return return_value;
    }
}

static sys_displayhook__doc__: __SlateAlign16<[i8; 97]> = __SlateAlign16([
    100i8, 105i8, 115i8, 112i8, 108i8, 97i8, 121i8, 104i8, 111i8, 111i8, 107i8, 40i8, 36i8, 109i8,
    111i8, 100i8, 117i8, 108i8, 101i8, 44i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 44i8,
    32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 80i8, 114i8, 105i8, 110i8, 116i8, 32i8, 97i8,
    110i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 32i8, 116i8, 111i8, 32i8, 115i8, 121i8,
    115i8, 46i8, 115i8, 116i8, 100i8, 111i8, 117i8, 116i8, 32i8, 97i8, 110i8, 100i8, 32i8, 97i8,
    108i8, 115i8, 111i8, 32i8, 115i8, 97i8, 118i8, 101i8, 32i8, 105i8, 116i8, 32i8, 105i8, 110i8,
    32i8, 98i8, 117i8, 105i8, 108i8, 116i8, 105i8, 110i8, 115i8, 46i8, 95i8, 0i8,
]);

static sys_excepthook__doc__: __SlateAlign16<[i8; 123]> = __SlateAlign16([
    101i8, 120i8, 99i8, 101i8, 112i8, 116i8, 104i8, 111i8, 111i8, 107i8, 40i8, 36i8, 109i8, 111i8,
    100i8, 117i8, 108i8, 101i8, 44i8, 32i8, 101i8, 120i8, 99i8, 116i8, 121i8, 112i8, 101i8, 44i8,
    32i8, 118i8, 97i8, 108i8, 117i8, 101i8, 44i8, 32i8, 116i8, 114i8, 97i8, 99i8, 101i8, 98i8,
    97i8, 99i8, 107i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 72i8, 97i8, 110i8,
    100i8, 108i8, 101i8, 32i8, 97i8, 110i8, 32i8, 101i8, 120i8, 99i8, 101i8, 112i8, 116i8, 105i8,
    111i8, 110i8, 32i8, 98i8, 121i8, 32i8, 100i8, 105i8, 115i8, 112i8, 108i8, 97i8, 121i8, 105i8,
    110i8, 103i8, 32i8, 105i8, 116i8, 32i8, 119i8, 105i8, 116i8, 104i8, 32i8, 97i8, 32i8, 116i8,
    114i8, 97i8, 99i8, 101i8, 98i8, 97i8, 99i8, 107i8, 32i8, 111i8, 110i8, 32i8, 115i8, 121i8,
    115i8, 46i8, 115i8, 116i8, 100i8, 101i8, 114i8, 114i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.sysmodule.sys_excepthook")]
extern "C-unwind" fn sys_excepthook(
    module: *mut _object,
    args: *const *mut _object,
    nargs: i64,
) -> *mut _object {
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        if _PyArg_CheckPositional!(c"excepthook".as_ptr() as *mut i8, nargs, 3i32, 3i32) {
            let exctype: *mut _object = *args.wrapping_offset(0 as isize);
            let value: *mut _object = *args.wrapping_offset(1 as isize);
            let traceback: *mut _object = *args.wrapping_offset(2 as isize);
            return_value = sys_excepthook_impl(module, exctype, value, traceback);
        }
        return return_value;
    }
}

static sys_exception__doc__: __SlateAlign16<[i8; 209]> = __SlateAlign16([
    101i8, 120i8, 99i8, 101i8, 112i8, 116i8, 105i8, 111i8, 110i8, 40i8, 36i8, 109i8, 111i8, 100i8,
    117i8, 108i8, 101i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8,
    117i8, 114i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 99i8, 117i8, 114i8, 114i8, 101i8, 110i8,
    116i8, 32i8, 101i8, 120i8, 99i8, 101i8, 112i8, 116i8, 105i8, 111i8, 110i8, 46i8, 10i8, 10i8,
    82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 109i8, 111i8, 115i8,
    116i8, 32i8, 114i8, 101i8, 99i8, 101i8, 110i8, 116i8, 32i8, 101i8, 120i8, 99i8, 101i8, 112i8,
    116i8, 105i8, 111i8, 110i8, 32i8, 99i8, 97i8, 117i8, 103i8, 104i8, 116i8, 32i8, 98i8, 121i8,
    32i8, 97i8, 110i8, 32i8, 101i8, 120i8, 99i8, 101i8, 112i8, 116i8, 32i8, 99i8, 108i8, 97i8,
    117i8, 115i8, 101i8, 10i8, 105i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 99i8, 117i8, 114i8,
    114i8, 101i8, 110i8, 116i8, 32i8, 115i8, 116i8, 97i8, 99i8, 107i8, 32i8, 102i8, 114i8, 97i8,
    109i8, 101i8, 32i8, 111i8, 114i8, 32i8, 105i8, 110i8, 32i8, 97i8, 110i8, 32i8, 111i8, 108i8,
    100i8, 101i8, 114i8, 32i8, 115i8, 116i8, 97i8, 99i8, 107i8, 32i8, 102i8, 114i8, 97i8, 109i8,
    101i8, 44i8, 32i8, 111i8, 114i8, 32i8, 78i8, 111i8, 110i8, 101i8, 10i8, 105i8, 102i8, 32i8,
    110i8, 111i8, 32i8, 115i8, 117i8, 99i8, 104i8, 32i8, 101i8, 120i8, 99i8, 101i8, 112i8, 116i8,
    105i8, 111i8, 110i8, 32i8, 101i8, 120i8, 105i8, 115i8, 116i8, 115i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.sysmodule.sys_exception")]
extern "C-unwind" fn sys_exception(
    module: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    sys_exception_impl(module)
}

static sys_exc_info__doc__: __SlateAlign16<[i8; 223]> = __SlateAlign16([
    101i8, 120i8, 99i8, 95i8, 105i8, 110i8, 102i8, 111i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8,
    108i8, 101i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8,
    114i8, 110i8, 32i8, 99i8, 117i8, 114i8, 114i8, 101i8, 110i8, 116i8, 32i8, 101i8, 120i8, 99i8,
    101i8, 112i8, 116i8, 105i8, 111i8, 110i8, 32i8, 105i8, 110i8, 102i8, 111i8, 114i8, 109i8, 97i8,
    116i8, 105i8, 111i8, 110i8, 58i8, 32i8, 40i8, 116i8, 121i8, 112i8, 101i8, 44i8, 32i8, 118i8,
    97i8, 108i8, 117i8, 101i8, 44i8, 32i8, 116i8, 114i8, 97i8, 99i8, 101i8, 98i8, 97i8, 99i8,
    107i8, 41i8, 46i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 105i8, 110i8,
    102i8, 111i8, 114i8, 109i8, 97i8, 116i8, 105i8, 111i8, 110i8, 32i8, 97i8, 98i8, 111i8, 117i8,
    116i8, 32i8, 116i8, 104i8, 101i8, 32i8, 109i8, 111i8, 115i8, 116i8, 32i8, 114i8, 101i8, 99i8,
    101i8, 110i8, 116i8, 32i8, 101i8, 120i8, 99i8, 101i8, 112i8, 116i8, 105i8, 111i8, 110i8, 32i8,
    99i8, 97i8, 117i8, 103i8, 104i8, 116i8, 32i8, 98i8, 121i8, 32i8, 97i8, 110i8, 32i8, 101i8,
    120i8, 99i8, 101i8, 112i8, 116i8, 10i8, 99i8, 108i8, 97i8, 117i8, 115i8, 101i8, 32i8, 105i8,
    110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 99i8, 117i8, 114i8, 114i8, 101i8, 110i8, 116i8, 32i8,
    115i8, 116i8, 97i8, 99i8, 107i8, 32i8, 102i8, 114i8, 97i8, 109i8, 101i8, 32i8, 111i8, 114i8,
    32i8, 105i8, 110i8, 32i8, 97i8, 110i8, 32i8, 111i8, 108i8, 100i8, 101i8, 114i8, 32i8, 115i8,
    116i8, 97i8, 99i8, 107i8, 32i8, 102i8, 114i8, 97i8, 109i8, 101i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.sysmodule.sys_exc_info")]
extern "C-unwind" fn sys_exc_info(
    module: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    sys_exc_info_impl(module)
}

static sys_unraisablehook__doc__: __SlateAlign16<[i8; 345]> = __SlateAlign16([
    117i8, 110i8, 114i8, 97i8, 105i8, 115i8, 97i8, 98i8, 108i8, 101i8, 104i8, 111i8, 111i8, 107i8,
    40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 44i8, 32i8, 117i8, 110i8, 114i8, 97i8,
    105i8, 115i8, 97i8, 98i8, 108i8, 101i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8,
    72i8, 97i8, 110i8, 100i8, 108i8, 101i8, 32i8, 97i8, 110i8, 32i8, 117i8, 110i8, 114i8, 97i8,
    105i8, 115i8, 97i8, 98i8, 108i8, 101i8, 32i8, 101i8, 120i8, 99i8, 101i8, 112i8, 116i8, 105i8,
    111i8, 110i8, 46i8, 10i8, 10i8, 84i8, 104i8, 101i8, 32i8, 117i8, 110i8, 114i8, 97i8, 105i8,
    115i8, 97i8, 98i8, 108i8, 101i8, 32i8, 97i8, 114i8, 103i8, 117i8, 109i8, 101i8, 110i8, 116i8,
    32i8, 104i8, 97i8, 115i8, 32i8, 116i8, 104i8, 101i8, 32i8, 102i8, 111i8, 108i8, 108i8, 111i8,
    119i8, 105i8, 110i8, 103i8, 32i8, 97i8, 116i8, 116i8, 114i8, 105i8, 98i8, 117i8, 116i8, 101i8,
    115i8, 58i8, 10i8, 10i8, 42i8, 32i8, 101i8, 120i8, 99i8, 95i8, 116i8, 121i8, 112i8, 101i8,
    58i8, 32i8, 69i8, 120i8, 99i8, 101i8, 112i8, 116i8, 105i8, 111i8, 110i8, 32i8, 116i8, 121i8,
    112i8, 101i8, 46i8, 10i8, 42i8, 32i8, 101i8, 120i8, 99i8, 95i8, 118i8, 97i8, 108i8, 117i8,
    101i8, 58i8, 32i8, 69i8, 120i8, 99i8, 101i8, 112i8, 116i8, 105i8, 111i8, 110i8, 32i8, 118i8,
    97i8, 108i8, 117i8, 101i8, 44i8, 32i8, 99i8, 97i8, 110i8, 32i8, 98i8, 101i8, 32i8, 78i8, 111i8,
    110i8, 101i8, 46i8, 10i8, 42i8, 32i8, 101i8, 120i8, 99i8, 95i8, 116i8, 114i8, 97i8, 99i8,
    101i8, 98i8, 97i8, 99i8, 107i8, 58i8, 32i8, 69i8, 120i8, 99i8, 101i8, 112i8, 116i8, 105i8,
    111i8, 110i8, 32i8, 116i8, 114i8, 97i8, 99i8, 101i8, 98i8, 97i8, 99i8, 107i8, 44i8, 32i8, 99i8,
    97i8, 110i8, 32i8, 98i8, 101i8, 32i8, 78i8, 111i8, 110i8, 101i8, 46i8, 10i8, 42i8, 32i8, 101i8,
    114i8, 114i8, 95i8, 109i8, 115i8, 103i8, 58i8, 32i8, 69i8, 114i8, 114i8, 111i8, 114i8, 32i8,
    109i8, 101i8, 115i8, 115i8, 97i8, 103i8, 101i8, 44i8, 32i8, 99i8, 97i8, 110i8, 32i8, 98i8,
    101i8, 32i8, 78i8, 111i8, 110i8, 101i8, 46i8, 10i8, 42i8, 32i8, 111i8, 98i8, 106i8, 101i8,
    99i8, 116i8, 58i8, 32i8, 79i8, 98i8, 106i8, 101i8, 99i8, 116i8, 32i8, 99i8, 97i8, 117i8, 115i8,
    105i8, 110i8, 103i8, 32i8, 116i8, 104i8, 101i8, 32i8, 101i8, 120i8, 99i8, 101i8, 112i8, 116i8,
    105i8, 111i8, 110i8, 44i8, 32i8, 99i8, 97i8, 110i8, 32i8, 98i8, 101i8, 32i8, 78i8, 111i8,
    110i8, 101i8, 46i8, 0i8,
]);

static sys_exit__doc__: __SlateAlign16<[i8; 338]> = __SlateAlign16([
    101i8, 120i8, 105i8, 116i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 44i8, 32i8,
    115i8, 116i8, 97i8, 116i8, 117i8, 115i8, 61i8, 78i8, 111i8, 110i8, 101i8, 44i8, 32i8, 47i8,
    41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 69i8, 120i8, 105i8, 116i8, 32i8, 116i8, 104i8, 101i8, 32i8,
    105i8, 110i8, 116i8, 101i8, 114i8, 112i8, 114i8, 101i8, 116i8, 101i8, 114i8, 32i8, 98i8, 121i8,
    32i8, 114i8, 97i8, 105i8, 115i8, 105i8, 110i8, 103i8, 32i8, 83i8, 121i8, 115i8, 116i8, 101i8,
    109i8, 69i8, 120i8, 105i8, 116i8, 40i8, 115i8, 116i8, 97i8, 116i8, 117i8, 115i8, 41i8, 46i8,
    10i8, 10i8, 73i8, 102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 115i8, 116i8, 97i8, 116i8, 117i8,
    115i8, 32i8, 105i8, 115i8, 32i8, 111i8, 109i8, 105i8, 116i8, 116i8, 101i8, 100i8, 32i8, 111i8,
    114i8, 32i8, 78i8, 111i8, 110i8, 101i8, 44i8, 32i8, 105i8, 116i8, 32i8, 100i8, 101i8, 102i8,
    97i8, 117i8, 108i8, 116i8, 115i8, 32i8, 116i8, 111i8, 32i8, 122i8, 101i8, 114i8, 111i8, 32i8,
    40i8, 105i8, 46i8, 101i8, 46i8, 44i8, 32i8, 115i8, 117i8, 99i8, 99i8, 101i8, 115i8, 115i8,
    41i8, 46i8, 10i8, 73i8, 102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 115i8, 116i8, 97i8, 116i8,
    117i8, 115i8, 32i8, 105i8, 115i8, 32i8, 97i8, 110i8, 32i8, 105i8, 110i8, 116i8, 101i8, 103i8,
    101i8, 114i8, 44i8, 32i8, 105i8, 116i8, 32i8, 119i8, 105i8, 108i8, 108i8, 32i8, 98i8, 101i8,
    32i8, 117i8, 115i8, 101i8, 100i8, 32i8, 97i8, 115i8, 32i8, 116i8, 104i8, 101i8, 32i8, 115i8,
    121i8, 115i8, 116i8, 101i8, 109i8, 32i8, 101i8, 120i8, 105i8, 116i8, 32i8, 115i8, 116i8, 97i8,
    116i8, 117i8, 115i8, 46i8, 10i8, 73i8, 102i8, 32i8, 105i8, 116i8, 32i8, 105i8, 115i8, 32i8,
    97i8, 110i8, 111i8, 116i8, 104i8, 101i8, 114i8, 32i8, 107i8, 105i8, 110i8, 100i8, 32i8, 111i8,
    102i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 44i8, 32i8, 105i8, 116i8, 32i8, 119i8,
    105i8, 108i8, 108i8, 32i8, 98i8, 101i8, 32i8, 112i8, 114i8, 105i8, 110i8, 116i8, 101i8, 100i8,
    32i8, 97i8, 110i8, 100i8, 32i8, 116i8, 104i8, 101i8, 32i8, 115i8, 121i8, 115i8, 116i8, 101i8,
    109i8, 10i8, 101i8, 120i8, 105i8, 116i8, 32i8, 115i8, 116i8, 97i8, 116i8, 117i8, 115i8, 32i8,
    119i8, 105i8, 108i8, 108i8, 32i8, 98i8, 101i8, 32i8, 111i8, 110i8, 101i8, 32i8, 40i8, 105i8,
    46i8, 101i8, 46i8, 44i8, 32i8, 102i8, 97i8, 105i8, 108i8, 117i8, 114i8, 101i8, 41i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.sysmodule.sys_exit")]
extern "C-unwind" fn sys_exit(
    module: *mut _object,
    args: *const *mut _object,
    nargs: i64,
) -> *mut _object {
    let mut status: *mut _object = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        status = core::ptr::addr_of_mut!(_Py_NoneStruct);
        if _PyArg_CheckPositional!(c"exit".as_ptr() as *mut i8, nargs, 0i32, 1i32) {
            if nargs >= 1 {
                status = *args.wrapping_offset(0 as isize);
            }
            return_value = sys_exit_impl(module, status);
        }
        return return_value;
    }
}

static sys_getdefaultencoding__doc__: __SlateAlign16<[i8; 107]> = __SlateAlign16([
    103i8, 101i8, 116i8, 100i8, 101i8, 102i8, 97i8, 117i8, 108i8, 116i8, 101i8, 110i8, 99i8, 111i8,
    100i8, 105i8, 110i8, 103i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 44i8, 32i8,
    47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 116i8,
    104i8, 101i8, 32i8, 99i8, 117i8, 114i8, 114i8, 101i8, 110i8, 116i8, 32i8, 100i8, 101i8, 102i8,
    97i8, 117i8, 108i8, 116i8, 32i8, 101i8, 110i8, 99i8, 111i8, 100i8, 105i8, 110i8, 103i8, 32i8,
    117i8, 115i8, 101i8, 100i8, 32i8, 98i8, 121i8, 32i8, 116i8, 104i8, 101i8, 32i8, 85i8, 110i8,
    105i8, 99i8, 111i8, 100i8, 101i8, 32i8, 105i8, 109i8, 112i8, 108i8, 101i8, 109i8, 101i8, 110i8,
    116i8, 97i8, 116i8, 105i8, 111i8, 110i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.sysmodule.sys_getdefaultencoding")]
extern "C-unwind" fn sys_getdefaultencoding(
    module: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    sys_getdefaultencoding_impl(module)
}

static sys_getfilesystemencoding__doc__: __SlateAlign16<[i8; 109]> = __SlateAlign16([
    103i8, 101i8, 116i8, 102i8, 105i8, 108i8, 101i8, 115i8, 121i8, 115i8, 116i8, 101i8, 109i8,
    101i8, 110i8, 99i8, 111i8, 100i8, 105i8, 110i8, 103i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8,
    108i8, 101i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8,
    114i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 101i8, 110i8, 99i8, 111i8, 100i8, 105i8, 110i8,
    103i8, 32i8, 117i8, 115i8, 101i8, 100i8, 32i8, 116i8, 111i8, 32i8, 99i8, 111i8, 110i8, 118i8,
    101i8, 114i8, 116i8, 32i8, 85i8, 110i8, 105i8, 99i8, 111i8, 100i8, 101i8, 32i8, 102i8, 105i8,
    108i8, 101i8, 110i8, 97i8, 109i8, 101i8, 115i8, 32i8, 116i8, 111i8, 32i8, 79i8, 83i8, 32i8,
    102i8, 105i8, 108i8, 101i8, 110i8, 97i8, 109i8, 101i8, 115i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.sysmodule.sys_getfilesystemencoding")]
extern "C-unwind" fn sys_getfilesystemencoding(
    module: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    sys_getfilesystemencoding_impl(module)
}

static sys_getfilesystemencodeerrors__doc__: __SlateAlign16<[i8; 104]> = __SlateAlign16([
    103i8, 101i8, 116i8, 102i8, 105i8, 108i8, 101i8, 115i8, 121i8, 115i8, 116i8, 101i8, 109i8,
    101i8, 110i8, 99i8, 111i8, 100i8, 101i8, 101i8, 114i8, 114i8, 111i8, 114i8, 115i8, 40i8, 36i8,
    109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8,
    82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 101i8, 114i8, 114i8,
    111i8, 114i8, 32i8, 109i8, 111i8, 100i8, 101i8, 32i8, 117i8, 115i8, 101i8, 100i8, 32i8, 85i8,
    110i8, 105i8, 99i8, 111i8, 100i8, 101i8, 32i8, 116i8, 111i8, 32i8, 79i8, 83i8, 32i8, 102i8,
    105i8, 108i8, 101i8, 110i8, 97i8, 109i8, 101i8, 32i8, 99i8, 111i8, 110i8, 118i8, 101i8, 114i8,
    115i8, 105i8, 111i8, 110i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.sysmodule.sys_getfilesystemencodeerrors")]
extern "C-unwind" fn sys_getfilesystemencodeerrors(
    module: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    sys_getfilesystemencodeerrors_impl(module)
}

static sys_intern__doc__: __SlateAlign16<[i8; 262]> = __SlateAlign16([
    105i8, 110i8, 116i8, 101i8, 114i8, 110i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8,
    44i8, 32i8, 115i8, 116i8, 114i8, 105i8, 110i8, 103i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8,
    10i8, 10i8, 96i8, 96i8, 73i8, 110i8, 116i8, 101i8, 114i8, 110i8, 39i8, 39i8, 32i8, 116i8,
    104i8, 101i8, 32i8, 103i8, 105i8, 118i8, 101i8, 110i8, 32i8, 115i8, 116i8, 114i8, 105i8, 110i8,
    103i8, 46i8, 10i8, 10i8, 84i8, 104i8, 105i8, 115i8, 32i8, 101i8, 110i8, 116i8, 101i8, 114i8,
    115i8, 32i8, 116i8, 104i8, 101i8, 32i8, 115i8, 116i8, 114i8, 105i8, 110i8, 103i8, 32i8, 105i8,
    110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 40i8, 103i8, 108i8, 111i8, 98i8, 97i8, 108i8, 41i8,
    32i8, 116i8, 97i8, 98i8, 108i8, 101i8, 32i8, 111i8, 102i8, 32i8, 105i8, 110i8, 116i8, 101i8,
    114i8, 110i8, 101i8, 100i8, 32i8, 115i8, 116i8, 114i8, 105i8, 110i8, 103i8, 115i8, 32i8, 119i8,
    104i8, 111i8, 115i8, 101i8, 10i8, 112i8, 117i8, 114i8, 112i8, 111i8, 115i8, 101i8, 32i8, 105i8,
    115i8, 32i8, 116i8, 111i8, 32i8, 115i8, 112i8, 101i8, 101i8, 100i8, 32i8, 117i8, 112i8, 32i8,
    100i8, 105i8, 99i8, 116i8, 105i8, 111i8, 110i8, 97i8, 114i8, 121i8, 32i8, 108i8, 111i8, 111i8,
    107i8, 117i8, 112i8, 115i8, 46i8, 32i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 116i8,
    104i8, 101i8, 32i8, 115i8, 116i8, 114i8, 105i8, 110i8, 103i8, 32i8, 105i8, 116i8, 115i8, 101i8,
    108i8, 102i8, 32i8, 111i8, 114i8, 10i8, 116i8, 104i8, 101i8, 32i8, 112i8, 114i8, 101i8, 118i8,
    105i8, 111i8, 117i8, 115i8, 108i8, 121i8, 32i8, 105i8, 110i8, 116i8, 101i8, 114i8, 110i8,
    101i8, 100i8, 32i8, 115i8, 116i8, 114i8, 105i8, 110i8, 103i8, 32i8, 111i8, 98i8, 106i8, 101i8,
    99i8, 116i8, 32i8, 119i8, 105i8, 116i8, 104i8, 32i8, 116i8, 104i8, 101i8, 32i8, 115i8, 97i8,
    109i8, 101i8, 32i8, 118i8, 97i8, 108i8, 117i8, 101i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.sysmodule.sys_intern")]
extern "C-unwind" fn sys_intern(module: *mut _object, arg: *mut _object) -> *mut _object {
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        if PyUnicode_Check!(arg) == 0 {
            _PyArg_BadArgument(
                c"intern".as_ptr(),
                c"argument".as_ptr(),
                c"str".as_ptr(),
                arg,
            );
        } else {
            let s: *mut _object = arg;
            return_value = sys_intern_impl(module, s);
        }
        return return_value;
    }
}

static sys__is_interned__doc__: __SlateAlign16<[i8; 84]> = __SlateAlign16([
    95i8, 105i8, 115i8, 95i8, 105i8, 110i8, 116i8, 101i8, 114i8, 110i8, 101i8, 100i8, 40i8, 36i8,
    109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 44i8, 32i8, 115i8, 116i8, 114i8, 105i8, 110i8, 103i8,
    44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8,
    32i8, 84i8, 114i8, 117i8, 101i8, 32i8, 105i8, 102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 103i8,
    105i8, 118i8, 101i8, 110i8, 32i8, 115i8, 116i8, 114i8, 105i8, 110i8, 103i8, 32i8, 105i8, 115i8,
    32i8, 34i8, 105i8, 110i8, 116i8, 101i8, 114i8, 110i8, 101i8, 100i8, 34i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.sysmodule.sys__is_interned")]
extern "C-unwind" fn sys__is_interned(module: *mut _object, arg: *mut _object) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut _return_value: i32 = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        if PyUnicode_Check!(arg) == 0 {
            _PyArg_BadArgument(
                c"_is_interned".as_ptr(),
                c"argument".as_ptr(),
                c"str".as_ptr(),
                arg,
            );
        } else {
            let string: *mut _object = arg;
            _return_value = sys__is_interned_impl(module, string);
            if _return_value == 1i32.wrapping_neg() {
                __v0 = !PyErr_Occurred().is_null();
            } else {
                __v0 = false;
            }
            if !__v0 {
                return_value = PyBool_FromLong(_return_value as i64);
            }
        }
        return return_value;
    }
}

static sys__is_immortal__doc__: __SlateAlign16<[i8; 153]> = __SlateAlign16([
    95i8, 105i8, 115i8, 95i8, 105i8, 109i8, 109i8, 111i8, 114i8, 116i8, 97i8, 108i8, 40i8, 36i8,
    109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 44i8, 32i8, 111i8, 112i8, 44i8, 32i8, 47i8, 41i8,
    10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 84i8, 114i8,
    117i8, 101i8, 32i8, 105i8, 102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 103i8, 105i8, 118i8, 101i8,
    110i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 32i8, 105i8, 115i8, 32i8, 34i8, 105i8,
    109i8, 109i8, 111i8, 114i8, 116i8, 97i8, 108i8, 34i8, 32i8, 112i8, 101i8, 114i8, 32i8, 80i8,
    69i8, 80i8, 32i8, 54i8, 56i8, 51i8, 46i8, 10i8, 10i8, 84i8, 104i8, 105i8, 115i8, 32i8, 102i8,
    117i8, 110i8, 99i8, 116i8, 105i8, 111i8, 110i8, 32i8, 115i8, 104i8, 111i8, 117i8, 108i8, 100i8,
    32i8, 98i8, 101i8, 32i8, 117i8, 115i8, 101i8, 100i8, 32i8, 102i8, 111i8, 114i8, 32i8, 115i8,
    112i8, 101i8, 99i8, 105i8, 97i8, 108i8, 105i8, 122i8, 101i8, 100i8, 32i8, 112i8, 117i8, 114i8,
    112i8, 111i8, 115i8, 101i8, 115i8, 32i8, 111i8, 110i8, 108i8, 121i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.sysmodule.sys__is_immortal")]
extern "C-unwind" fn sys__is_immortal(module: *mut _object, op: *mut _object) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut _return_value: i32 = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        _return_value = sys__is_immortal_impl(module, op);
        if _return_value == 1i32.wrapping_neg() {
            __v0 = !PyErr_Occurred().is_null();
        } else {
            __v0 = false;
        }
        if !__v0 {
            return_value = PyBool_FromLong(_return_value as i64);
        }
        return return_value;
    }
}

static sys_settrace__doc__: __SlateAlign16<[i8; 165]> = __SlateAlign16([
    115i8, 101i8, 116i8, 116i8, 114i8, 97i8, 99i8, 101i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8,
    108i8, 101i8, 44i8, 32i8, 102i8, 117i8, 110i8, 99i8, 116i8, 105i8, 111i8, 110i8, 44i8, 32i8,
    47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 83i8, 101i8, 116i8, 32i8, 116i8, 104i8, 101i8, 32i8,
    103i8, 108i8, 111i8, 98i8, 97i8, 108i8, 32i8, 100i8, 101i8, 98i8, 117i8, 103i8, 32i8, 116i8,
    114i8, 97i8, 99i8, 105i8, 110i8, 103i8, 32i8, 102i8, 117i8, 110i8, 99i8, 116i8, 105i8, 111i8,
    110i8, 46i8, 10i8, 10i8, 73i8, 116i8, 32i8, 119i8, 105i8, 108i8, 108i8, 32i8, 98i8, 101i8,
    32i8, 99i8, 97i8, 108i8, 108i8, 101i8, 100i8, 32i8, 111i8, 110i8, 32i8, 101i8, 97i8, 99i8,
    104i8, 32i8, 102i8, 117i8, 110i8, 99i8, 116i8, 105i8, 111i8, 110i8, 32i8, 99i8, 97i8, 108i8,
    108i8, 46i8, 32i8, 32i8, 83i8, 101i8, 101i8, 32i8, 116i8, 104i8, 101i8, 32i8, 100i8, 101i8,
    98i8, 117i8, 103i8, 103i8, 101i8, 114i8, 32i8, 99i8, 104i8, 97i8, 112i8, 116i8, 101i8, 114i8,
    10i8, 105i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 108i8, 105i8, 98i8, 114i8, 97i8, 114i8,
    121i8, 32i8, 109i8, 97i8, 110i8, 117i8, 97i8, 108i8, 46i8, 0i8,
]);

static sys__settraceallthreads__doc__: __SlateAlign16<[i8; 235]> = __SlateAlign16([
    95i8, 115i8, 101i8, 116i8, 116i8, 114i8, 97i8, 99i8, 101i8, 97i8, 108i8, 108i8, 116i8, 104i8,
    114i8, 101i8, 97i8, 100i8, 115i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 44i8,
    32i8, 102i8, 117i8, 110i8, 99i8, 116i8, 105i8, 111i8, 110i8, 44i8, 32i8, 47i8, 41i8, 10i8,
    45i8, 45i8, 10i8, 10i8, 83i8, 101i8, 116i8, 32i8, 116i8, 104i8, 101i8, 32i8, 103i8, 108i8,
    111i8, 98i8, 97i8, 108i8, 32i8, 100i8, 101i8, 98i8, 117i8, 103i8, 32i8, 116i8, 114i8, 97i8,
    99i8, 105i8, 110i8, 103i8, 32i8, 102i8, 117i8, 110i8, 99i8, 116i8, 105i8, 111i8, 110i8, 32i8,
    105i8, 110i8, 32i8, 97i8, 108i8, 108i8, 32i8, 114i8, 117i8, 110i8, 110i8, 105i8, 110i8, 103i8,
    32i8, 116i8, 104i8, 114i8, 101i8, 97i8, 100i8, 115i8, 32i8, 98i8, 101i8, 108i8, 111i8, 110i8,
    103i8, 105i8, 110i8, 103i8, 32i8, 116i8, 111i8, 32i8, 116i8, 104i8, 101i8, 32i8, 99i8, 117i8,
    114i8, 114i8, 101i8, 110i8, 116i8, 32i8, 105i8, 110i8, 116i8, 101i8, 114i8, 112i8, 114i8,
    101i8, 116i8, 101i8, 114i8, 46i8, 10i8, 10i8, 73i8, 116i8, 32i8, 119i8, 105i8, 108i8, 108i8,
    32i8, 98i8, 101i8, 32i8, 99i8, 97i8, 108i8, 108i8, 101i8, 100i8, 32i8, 111i8, 110i8, 32i8,
    101i8, 97i8, 99i8, 104i8, 32i8, 102i8, 117i8, 110i8, 99i8, 116i8, 105i8, 111i8, 110i8, 32i8,
    99i8, 97i8, 108i8, 108i8, 46i8, 32i8, 83i8, 101i8, 101i8, 32i8, 116i8, 104i8, 101i8, 32i8,
    100i8, 101i8, 98i8, 117i8, 103i8, 103i8, 101i8, 114i8, 32i8, 99i8, 104i8, 97i8, 112i8, 116i8,
    101i8, 114i8, 10i8, 105i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 108i8, 105i8, 98i8, 114i8,
    97i8, 114i8, 121i8, 32i8, 109i8, 97i8, 110i8, 117i8, 97i8, 108i8, 46i8, 0i8,
]);

static sys_gettrace__doc__: __SlateAlign16<[i8; 138]> = __SlateAlign16([
    103i8, 101i8, 116i8, 116i8, 114i8, 97i8, 99i8, 101i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8,
    108i8, 101i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8,
    114i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 103i8, 108i8, 111i8, 98i8, 97i8, 108i8, 32i8,
    100i8, 101i8, 98i8, 117i8, 103i8, 32i8, 116i8, 114i8, 97i8, 99i8, 105i8, 110i8, 103i8, 32i8,
    102i8, 117i8, 110i8, 99i8, 116i8, 105i8, 111i8, 110i8, 32i8, 115i8, 101i8, 116i8, 32i8, 119i8,
    105i8, 116i8, 104i8, 32i8, 115i8, 121i8, 115i8, 46i8, 115i8, 101i8, 116i8, 116i8, 114i8, 97i8,
    99i8, 101i8, 46i8, 10i8, 10i8, 83i8, 101i8, 101i8, 32i8, 116i8, 104i8, 101i8, 32i8, 100i8,
    101i8, 98i8, 117i8, 103i8, 103i8, 101i8, 114i8, 32i8, 99i8, 104i8, 97i8, 112i8, 116i8, 101i8,
    114i8, 32i8, 105i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 108i8, 105i8, 98i8, 114i8, 97i8,
    114i8, 121i8, 32i8, 109i8, 97i8, 110i8, 117i8, 97i8, 108i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.sysmodule.sys_gettrace")]
extern "C-unwind" fn sys_gettrace(
    module: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    sys_gettrace_impl(module)
}

static sys_setprofile__doc__: __SlateAlign16<[i8; 167]> = __SlateAlign16([
    115i8, 101i8, 116i8, 112i8, 114i8, 111i8, 102i8, 105i8, 108i8, 101i8, 40i8, 36i8, 109i8, 111i8,
    100i8, 117i8, 108i8, 101i8, 44i8, 32i8, 102i8, 117i8, 110i8, 99i8, 116i8, 105i8, 111i8, 110i8,
    44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 83i8, 101i8, 116i8, 32i8, 116i8, 104i8,
    101i8, 32i8, 112i8, 114i8, 111i8, 102i8, 105i8, 108i8, 105i8, 110i8, 103i8, 32i8, 102i8, 117i8,
    110i8, 99i8, 116i8, 105i8, 111i8, 110i8, 46i8, 10i8, 10i8, 73i8, 116i8, 32i8, 119i8, 105i8,
    108i8, 108i8, 32i8, 98i8, 101i8, 32i8, 99i8, 97i8, 108i8, 108i8, 101i8, 100i8, 32i8, 111i8,
    110i8, 32i8, 101i8, 97i8, 99i8, 104i8, 32i8, 102i8, 117i8, 110i8, 99i8, 116i8, 105i8, 111i8,
    110i8, 32i8, 99i8, 97i8, 108i8, 108i8, 32i8, 97i8, 110i8, 100i8, 32i8, 114i8, 101i8, 116i8,
    117i8, 114i8, 110i8, 46i8, 32i8, 32i8, 83i8, 101i8, 101i8, 32i8, 116i8, 104i8, 101i8, 32i8,
    112i8, 114i8, 111i8, 102i8, 105i8, 108i8, 101i8, 114i8, 10i8, 99i8, 104i8, 97i8, 112i8, 116i8,
    101i8, 114i8, 32i8, 105i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 108i8, 105i8, 98i8, 114i8,
    97i8, 114i8, 121i8, 32i8, 109i8, 97i8, 110i8, 117i8, 97i8, 108i8, 46i8, 0i8,
]);

static sys__setprofileallthreads__doc__: __SlateAlign16<[i8; 238]> = __SlateAlign16([
    95i8, 115i8, 101i8, 116i8, 112i8, 114i8, 111i8, 102i8, 105i8, 108i8, 101i8, 97i8, 108i8, 108i8,
    116i8, 104i8, 114i8, 101i8, 97i8, 100i8, 115i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8,
    101i8, 44i8, 32i8, 102i8, 117i8, 110i8, 99i8, 116i8, 105i8, 111i8, 110i8, 44i8, 32i8, 47i8,
    41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 83i8, 101i8, 116i8, 32i8, 116i8, 104i8, 101i8, 32i8, 112i8,
    114i8, 111i8, 102i8, 105i8, 108i8, 105i8, 110i8, 103i8, 32i8, 102i8, 117i8, 110i8, 99i8, 116i8,
    105i8, 111i8, 110i8, 32i8, 105i8, 110i8, 32i8, 97i8, 108i8, 108i8, 32i8, 114i8, 117i8, 110i8,
    110i8, 105i8, 110i8, 103i8, 32i8, 116i8, 104i8, 114i8, 101i8, 97i8, 100i8, 115i8, 32i8, 98i8,
    101i8, 108i8, 111i8, 110i8, 103i8, 105i8, 110i8, 103i8, 32i8, 116i8, 111i8, 32i8, 116i8, 104i8,
    101i8, 32i8, 99i8, 117i8, 114i8, 114i8, 101i8, 110i8, 116i8, 32i8, 105i8, 110i8, 116i8, 101i8,
    114i8, 112i8, 114i8, 101i8, 116i8, 101i8, 114i8, 46i8, 10i8, 10i8, 73i8, 116i8, 32i8, 119i8,
    105i8, 108i8, 108i8, 32i8, 98i8, 101i8, 32i8, 99i8, 97i8, 108i8, 108i8, 101i8, 100i8, 32i8,
    111i8, 110i8, 32i8, 101i8, 97i8, 99i8, 104i8, 32i8, 102i8, 117i8, 110i8, 99i8, 116i8, 105i8,
    111i8, 110i8, 32i8, 99i8, 97i8, 108i8, 108i8, 32i8, 97i8, 110i8, 100i8, 32i8, 114i8, 101i8,
    116i8, 117i8, 114i8, 110i8, 46i8, 32i8, 32i8, 83i8, 101i8, 101i8, 32i8, 116i8, 104i8, 101i8,
    32i8, 112i8, 114i8, 111i8, 102i8, 105i8, 108i8, 101i8, 114i8, 10i8, 99i8, 104i8, 97i8, 112i8,
    116i8, 101i8, 114i8, 32i8, 105i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 108i8, 105i8, 98i8,
    114i8, 97i8, 114i8, 121i8, 32i8, 109i8, 97i8, 110i8, 117i8, 97i8, 108i8, 46i8, 0i8,
]);

static sys_getprofile__doc__: __SlateAlign16<[i8; 131]> = __SlateAlign16([
    103i8, 101i8, 116i8, 112i8, 114i8, 111i8, 102i8, 105i8, 108i8, 101i8, 40i8, 36i8, 109i8, 111i8,
    100i8, 117i8, 108i8, 101i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8,
    116i8, 117i8, 114i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 112i8, 114i8, 111i8, 102i8, 105i8,
    108i8, 105i8, 110i8, 103i8, 32i8, 102i8, 117i8, 110i8, 99i8, 116i8, 105i8, 111i8, 110i8, 32i8,
    115i8, 101i8, 116i8, 32i8, 119i8, 105i8, 116i8, 104i8, 32i8, 115i8, 121i8, 115i8, 46i8, 115i8,
    101i8, 116i8, 112i8, 114i8, 111i8, 102i8, 105i8, 108i8, 101i8, 46i8, 10i8, 10i8, 83i8, 101i8,
    101i8, 32i8, 116i8, 104i8, 101i8, 32i8, 112i8, 114i8, 111i8, 102i8, 105i8, 108i8, 101i8, 114i8,
    32i8, 99i8, 104i8, 97i8, 112i8, 116i8, 101i8, 114i8, 32i8, 105i8, 110i8, 32i8, 116i8, 104i8,
    101i8, 32i8, 108i8, 105i8, 98i8, 114i8, 97i8, 114i8, 121i8, 32i8, 109i8, 97i8, 110i8, 117i8,
    97i8, 108i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.sysmodule.sys_getprofile")]
extern "C-unwind" fn sys_getprofile(
    module: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    sys_getprofile_impl(module)
}

static sys_setswitchinterval__doc__: __SlateAlign16<[i8; 405]> = __SlateAlign16([
    115i8, 101i8, 116i8, 115i8, 119i8, 105i8, 116i8, 99i8, 104i8, 105i8, 110i8, 116i8, 101i8,
    114i8, 118i8, 97i8, 108i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 44i8, 32i8,
    105i8, 110i8, 116i8, 101i8, 114i8, 118i8, 97i8, 108i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8,
    45i8, 10i8, 10i8, 83i8, 101i8, 116i8, 32i8, 116i8, 104i8, 101i8, 32i8, 105i8, 100i8, 101i8,
    97i8, 108i8, 32i8, 116i8, 104i8, 114i8, 101i8, 97i8, 100i8, 32i8, 115i8, 119i8, 105i8, 116i8,
    99i8, 104i8, 105i8, 110i8, 103i8, 32i8, 100i8, 101i8, 108i8, 97i8, 121i8, 32i8, 105i8, 110i8,
    115i8, 105i8, 100i8, 101i8, 32i8, 116i8, 104i8, 101i8, 32i8, 80i8, 121i8, 116i8, 104i8, 111i8,
    110i8, 32i8, 105i8, 110i8, 116i8, 101i8, 114i8, 112i8, 114i8, 101i8, 116i8, 101i8, 114i8, 46i8,
    10i8, 10i8, 84i8, 104i8, 101i8, 32i8, 97i8, 99i8, 116i8, 117i8, 97i8, 108i8, 32i8, 102i8,
    114i8, 101i8, 113i8, 117i8, 101i8, 110i8, 99i8, 121i8, 32i8, 111i8, 102i8, 32i8, 115i8, 119i8,
    105i8, 116i8, 99i8, 104i8, 105i8, 110i8, 103i8, 32i8, 116i8, 104i8, 114i8, 101i8, 97i8, 100i8,
    115i8, 32i8, 99i8, 97i8, 110i8, 32i8, 98i8, 101i8, 32i8, 108i8, 111i8, 119i8, 101i8, 114i8,
    32i8, 105i8, 102i8, 32i8, 116i8, 104i8, 101i8, 10i8, 105i8, 110i8, 116i8, 101i8, 114i8, 112i8,
    114i8, 101i8, 116i8, 101i8, 114i8, 32i8, 101i8, 120i8, 101i8, 99i8, 117i8, 116i8, 101i8, 115i8,
    32i8, 108i8, 111i8, 110i8, 103i8, 32i8, 115i8, 101i8, 113i8, 117i8, 101i8, 110i8, 99i8, 101i8,
    115i8, 32i8, 111i8, 102i8, 32i8, 117i8, 110i8, 105i8, 110i8, 116i8, 101i8, 114i8, 114i8, 117i8,
    112i8, 116i8, 105i8, 98i8, 108i8, 101i8, 32i8, 99i8, 111i8, 100i8, 101i8, 10i8, 40i8, 116i8,
    104i8, 105i8, 115i8, 32i8, 105i8, 115i8, 32i8, 105i8, 109i8, 112i8, 108i8, 101i8, 109i8, 101i8,
    110i8, 116i8, 97i8, 116i8, 105i8, 111i8, 110i8, 45i8, 115i8, 112i8, 101i8, 99i8, 105i8, 102i8,
    105i8, 99i8, 32i8, 97i8, 110i8, 100i8, 32i8, 119i8, 111i8, 114i8, 107i8, 108i8, 111i8, 97i8,
    100i8, 45i8, 100i8, 101i8, 112i8, 101i8, 110i8, 100i8, 101i8, 110i8, 116i8, 41i8, 46i8, 10i8,
    10i8, 84i8, 104i8, 101i8, 32i8, 112i8, 97i8, 114i8, 97i8, 109i8, 101i8, 116i8, 101i8, 114i8,
    32i8, 109i8, 117i8, 115i8, 116i8, 32i8, 114i8, 101i8, 112i8, 114i8, 101i8, 115i8, 101i8, 110i8,
    116i8, 32i8, 116i8, 104i8, 101i8, 32i8, 100i8, 101i8, 115i8, 105i8, 114i8, 101i8, 100i8, 32i8,
    115i8, 119i8, 105i8, 116i8, 99i8, 104i8, 105i8, 110i8, 103i8, 32i8, 100i8, 101i8, 108i8, 97i8,
    121i8, 32i8, 105i8, 110i8, 32i8, 115i8, 101i8, 99i8, 111i8, 110i8, 100i8, 115i8, 10i8, 65i8,
    32i8, 116i8, 121i8, 112i8, 105i8, 99i8, 97i8, 108i8, 32i8, 118i8, 97i8, 108i8, 117i8, 101i8,
    32i8, 105i8, 115i8, 32i8, 48i8, 46i8, 48i8, 48i8, 53i8, 32i8, 40i8, 53i8, 32i8, 109i8, 105i8,
    108i8, 108i8, 105i8, 115i8, 101i8, 99i8, 111i8, 110i8, 100i8, 115i8, 41i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.sysmodule.sys_setswitchinterval")]
extern "C-unwind" fn sys_setswitchinterval(
    module: *mut _object,
    arg: *mut _object,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut interval: f64 = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_0: {
            return_value = core::ptr::null_mut::<_object>();
            if PyFloat_CheckExact!(arg) != 0 {
                interval = PyFloat_AS_DOUBLE!(arg);
            } else {
                interval = PyFloat_AsDouble(arg);
                if interval == -1.0f64 {
                    __v0 = !PyErr_Occurred().is_null();
                } else {
                    __v0 = false;
                }
                if __v0 {
                    break '__join_0;
                }
            }
            return_value = sys_setswitchinterval_impl(module, interval);
        }
        return return_value;
    }
}

static sys_getswitchinterval__doc__: __SlateAlign16<[i8; 106]> = __SlateAlign16([
    103i8, 101i8, 116i8, 115i8, 119i8, 105i8, 116i8, 99i8, 104i8, 105i8, 110i8, 116i8, 101i8,
    114i8, 118i8, 97i8, 108i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 44i8, 32i8,
    47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 116i8,
    104i8, 101i8, 32i8, 99i8, 117i8, 114i8, 114i8, 101i8, 110i8, 116i8, 32i8, 116i8, 104i8, 114i8,
    101i8, 97i8, 100i8, 32i8, 115i8, 119i8, 105i8, 116i8, 99i8, 104i8, 32i8, 105i8, 110i8, 116i8,
    101i8, 114i8, 118i8, 97i8, 108i8, 59i8, 32i8, 115i8, 101i8, 101i8, 32i8, 115i8, 121i8, 115i8,
    46i8, 115i8, 101i8, 116i8, 115i8, 119i8, 105i8, 116i8, 99i8, 104i8, 105i8, 110i8, 116i8, 101i8,
    114i8, 118i8, 97i8, 108i8, 40i8, 41i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.sysmodule.sys_getswitchinterval")]
extern "C-unwind" fn sys_getswitchinterval(
    module: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut _return_value: f64 = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        _return_value = sys_getswitchinterval_impl(module);
        if _return_value == -1.0f64 {
            __v0 = !PyErr_Occurred().is_null();
        } else {
            __v0 = false;
        }
        if !__v0 {
            return_value = PyFloat_FromDouble(_return_value);
        }
        return return_value;
    }
}

static sys_setrecursionlimit__doc__: __SlateAlign16<[i8; 254]> = __SlateAlign16([
    115i8, 101i8, 116i8, 114i8, 101i8, 99i8, 117i8, 114i8, 115i8, 105i8, 111i8, 110i8, 108i8,
    105i8, 109i8, 105i8, 116i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 44i8, 32i8,
    108i8, 105i8, 109i8, 105i8, 116i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 83i8,
    101i8, 116i8, 32i8, 116i8, 104i8, 101i8, 32i8, 109i8, 97i8, 120i8, 105i8, 109i8, 117i8, 109i8,
    32i8, 100i8, 101i8, 112i8, 116i8, 104i8, 32i8, 111i8, 102i8, 32i8, 116i8, 104i8, 101i8, 32i8,
    80i8, 121i8, 116i8, 104i8, 111i8, 110i8, 32i8, 105i8, 110i8, 116i8, 101i8, 114i8, 112i8, 114i8,
    101i8, 116i8, 101i8, 114i8, 32i8, 115i8, 116i8, 97i8, 99i8, 107i8, 32i8, 116i8, 111i8, 32i8,
    110i8, 46i8, 10i8, 10i8, 84i8, 104i8, 105i8, 115i8, 32i8, 108i8, 105i8, 109i8, 105i8, 116i8,
    32i8, 112i8, 114i8, 101i8, 118i8, 101i8, 110i8, 116i8, 115i8, 32i8, 105i8, 110i8, 102i8, 105i8,
    110i8, 105i8, 116i8, 101i8, 32i8, 114i8, 101i8, 99i8, 117i8, 114i8, 115i8, 105i8, 111i8, 110i8,
    32i8, 102i8, 114i8, 111i8, 109i8, 32i8, 99i8, 97i8, 117i8, 115i8, 105i8, 110i8, 103i8, 32i8,
    97i8, 110i8, 32i8, 111i8, 118i8, 101i8, 114i8, 102i8, 108i8, 111i8, 119i8, 32i8, 111i8, 102i8,
    32i8, 116i8, 104i8, 101i8, 32i8, 67i8, 10i8, 115i8, 116i8, 97i8, 99i8, 107i8, 32i8, 97i8,
    110i8, 100i8, 32i8, 99i8, 114i8, 97i8, 115i8, 104i8, 105i8, 110i8, 103i8, 32i8, 80i8, 121i8,
    116i8, 104i8, 111i8, 110i8, 46i8, 32i8, 32i8, 84i8, 104i8, 101i8, 32i8, 104i8, 105i8, 103i8,
    104i8, 101i8, 115i8, 116i8, 32i8, 112i8, 111i8, 115i8, 115i8, 105i8, 98i8, 108i8, 101i8, 32i8,
    108i8, 105i8, 109i8, 105i8, 116i8, 32i8, 105i8, 115i8, 32i8, 112i8, 108i8, 97i8, 116i8, 102i8,
    111i8, 114i8, 109i8, 45i8, 10i8, 100i8, 101i8, 112i8, 101i8, 110i8, 100i8, 101i8, 110i8, 116i8,
    46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.sysmodule.sys_setrecursionlimit")]
extern "C-unwind" fn sys_setrecursionlimit(
    module: *mut _object,
    arg: *mut _object,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut new_limit: i32 = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        new_limit = PyLong_AsInt(arg);
        if new_limit == 1i32.wrapping_neg() {
            __v0 = !PyErr_Occurred().is_null();
        } else {
            __v0 = false;
        }
        if !__v0 {
            return_value = sys_setrecursionlimit_impl(module, new_limit);
        }
        return return_value;
    }
}

static sys_set_coroutine_origin_tracking_depth__doc__: __SlateAlign16<[i8; 299]> =
    __SlateAlign16([
        115i8, 101i8, 116i8, 95i8, 99i8, 111i8, 114i8, 111i8, 117i8, 116i8, 105i8, 110i8, 101i8,
        95i8, 111i8, 114i8, 105i8, 103i8, 105i8, 110i8, 95i8, 116i8, 114i8, 97i8, 99i8, 107i8,
        105i8, 110i8, 103i8, 95i8, 100i8, 101i8, 112i8, 116i8, 104i8, 40i8, 36i8, 109i8, 111i8,
        100i8, 117i8, 108i8, 101i8, 44i8, 32i8, 47i8, 44i8, 32i8, 100i8, 101i8, 112i8, 116i8,
        104i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 69i8, 110i8, 97i8, 98i8, 108i8, 101i8, 32i8,
        111i8, 114i8, 32i8, 100i8, 105i8, 115i8, 97i8, 98i8, 108i8, 101i8, 32i8, 111i8, 114i8,
        105i8, 103i8, 105i8, 110i8, 32i8, 116i8, 114i8, 97i8, 99i8, 107i8, 105i8, 110i8, 103i8,
        32i8, 102i8, 111i8, 114i8, 32i8, 99i8, 111i8, 114i8, 111i8, 117i8, 116i8, 105i8, 110i8,
        101i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 115i8, 32i8, 105i8, 110i8, 32i8,
        116i8, 104i8, 105i8, 115i8, 32i8, 116i8, 104i8, 114i8, 101i8, 97i8, 100i8, 46i8, 10i8,
        10i8, 67i8, 111i8, 114i8, 111i8, 117i8, 116i8, 105i8, 110i8, 101i8, 32i8, 111i8, 98i8,
        106i8, 101i8, 99i8, 116i8, 115i8, 32i8, 119i8, 105i8, 108i8, 108i8, 32i8, 116i8, 114i8,
        97i8, 99i8, 107i8, 32i8, 39i8, 100i8, 101i8, 112i8, 116i8, 104i8, 39i8, 32i8, 102i8, 114i8,
        97i8, 109i8, 101i8, 115i8, 32i8, 111i8, 102i8, 32i8, 116i8, 114i8, 97i8, 99i8, 101i8, 98i8,
        97i8, 99i8, 107i8, 32i8, 105i8, 110i8, 102i8, 111i8, 114i8, 109i8, 97i8, 116i8, 105i8,
        111i8, 110i8, 10i8, 97i8, 98i8, 111i8, 117i8, 116i8, 32i8, 119i8, 104i8, 101i8, 114i8,
        101i8, 32i8, 116i8, 104i8, 101i8, 121i8, 32i8, 99i8, 97i8, 109i8, 101i8, 32i8, 102i8,
        114i8, 111i8, 109i8, 44i8, 32i8, 97i8, 118i8, 97i8, 105i8, 108i8, 97i8, 98i8, 108i8, 101i8,
        32i8, 105i8, 110i8, 32i8, 116i8, 104i8, 101i8, 105i8, 114i8, 32i8, 99i8, 114i8, 95i8,
        111i8, 114i8, 105i8, 103i8, 105i8, 110i8, 32i8, 97i8, 116i8, 116i8, 114i8, 105i8, 98i8,
        117i8, 116i8, 101i8, 46i8, 10i8, 10i8, 83i8, 101i8, 116i8, 32i8, 97i8, 32i8, 100i8, 101i8,
        112i8, 116i8, 104i8, 32i8, 111i8, 102i8, 32i8, 48i8, 32i8, 116i8, 111i8, 32i8, 100i8,
        105i8, 115i8, 97i8, 98i8, 108i8, 101i8, 46i8, 0i8,
    ]);

#[unsafe(link_section = ".text.slate_distinct.sysmodule.sys_set_coroutine_origin_tracking_depth")]
extern "C-unwind" fn sys_set_coroutine_origin_tracking_depth(
    module: *mut _object,
    mut args: *const *mut _object,
    nargs: i64,
    kwnames: *mut _object,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut depth: i32 = unsafe { core::mem::zeroed() };
    let mut argsbuf: [*mut _object; 1] = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        args = _PyArg_UnpackKeywords!(
            args,
            nargs,
            core::ptr::null_mut::<()>(),
            kwnames,
            core::ptr::addr_of_mut!(_parser__639),
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
            depth = PyLong_AsInt(*args.wrapping_offset(0 as isize));
            if depth == 1i32.wrapping_neg() {
                __v0 = !PyErr_Occurred().is_null();
            } else {
                __v0 = false;
            }
            if !__v0 {
                return_value = sys_set_coroutine_origin_tracking_depth_impl(module, depth);
            }
        }
        return return_value;
    }
}

static mut _kwtuple__626: __SlateRecord_sysmodule_c_h_626 = {
    let mut __t0: __SlateRecord_sysmodule_c_h_626 = unsafe { core::mem::zeroed() };
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
                ._py_depth
                ._ascii
                .ob_base
        )
    }];
    __t0
};

static _keywords__638: __SlateAlign16<[*const i8; 2]> = __SlateAlign16([
    (b"depth\0".as_ptr() as *mut i8) as *const i8,
    core::ptr::null::<i8>(),
]);

static mut _parser__639: _PyArg_Parser = {
    let mut __t0: _PyArg_Parser = unsafe { core::mem::zeroed() };
    __t0.keywords = unsafe { core::ptr::addr_of!(_keywords__638.0) as *const *const i8 };
    __t0.fname = (b"set_coroutine_origin_tracking_depth\0".as_ptr() as *mut i8) as *const i8;
    __t0.kwtuple = unsafe { core::ptr::addr_of_mut!(_kwtuple__626.ob_base.ob_base) };
    __t0
};

static sys_get_coroutine_origin_tracking_depth__doc__: __SlateAlign16<[i8; 122]> =
    __SlateAlign16([
        103i8, 101i8, 116i8, 95i8, 99i8, 111i8, 114i8, 111i8, 117i8, 116i8, 105i8, 110i8, 101i8,
        95i8, 111i8, 114i8, 105i8, 103i8, 105i8, 110i8, 95i8, 116i8, 114i8, 97i8, 99i8, 107i8,
        105i8, 110i8, 103i8, 95i8, 100i8, 101i8, 112i8, 116i8, 104i8, 40i8, 36i8, 109i8, 111i8,
        100i8, 117i8, 108i8, 101i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 67i8,
        104i8, 101i8, 99i8, 107i8, 32i8, 115i8, 116i8, 97i8, 116i8, 117i8, 115i8, 32i8, 111i8,
        102i8, 32i8, 111i8, 114i8, 105i8, 103i8, 105i8, 110i8, 32i8, 116i8, 114i8, 97i8, 99i8,
        107i8, 105i8, 110i8, 103i8, 32i8, 102i8, 111i8, 114i8, 32i8, 99i8, 111i8, 114i8, 111i8,
        117i8, 116i8, 105i8, 110i8, 101i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 115i8,
        32i8, 105i8, 110i8, 32i8, 116i8, 104i8, 105i8, 115i8, 32i8, 116i8, 104i8, 114i8, 101i8,
        97i8, 100i8, 46i8, 0i8,
    ]);

#[unsafe(link_section = ".text.slate_distinct.sysmodule.sys_get_coroutine_origin_tracking_depth")]
extern "C-unwind" fn sys_get_coroutine_origin_tracking_depth(
    module: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut _return_value: i32 = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        _return_value = sys_get_coroutine_origin_tracking_depth_impl(module);
        if _return_value == 1i32.wrapping_neg() {
            __v0 = !PyErr_Occurred().is_null();
        } else {
            __v0 = false;
        }
        if !__v0 {
            return_value = PyLong_FromLong(_return_value as i64);
        }
        return return_value;
    }
}

static sys_get_asyncgen_hooks__doc__: __SlateAlign16<[i8; 150]> = __SlateAlign16([
    103i8, 101i8, 116i8, 95i8, 97i8, 115i8, 121i8, 110i8, 99i8, 103i8, 101i8, 110i8, 95i8, 104i8,
    111i8, 111i8, 107i8, 115i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 44i8, 32i8,
    47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 116i8,
    104i8, 101i8, 32i8, 105i8, 110i8, 115i8, 116i8, 97i8, 108i8, 108i8, 101i8, 100i8, 32i8, 97i8,
    115i8, 121i8, 110i8, 99i8, 104i8, 114i8, 111i8, 110i8, 111i8, 117i8, 115i8, 32i8, 103i8, 101i8,
    110i8, 101i8, 114i8, 97i8, 116i8, 111i8, 114i8, 115i8, 32i8, 104i8, 111i8, 111i8, 107i8, 115i8,
    46i8, 10i8, 10i8, 84i8, 104i8, 105i8, 115i8, 32i8, 114i8, 101i8, 116i8, 117i8, 114i8, 110i8,
    115i8, 32i8, 97i8, 32i8, 110i8, 97i8, 109i8, 101i8, 100i8, 116i8, 117i8, 112i8, 108i8, 101i8,
    32i8, 111i8, 102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 102i8, 111i8, 114i8, 109i8, 32i8, 40i8,
    102i8, 105i8, 114i8, 115i8, 116i8, 105i8, 116i8, 101i8, 114i8, 44i8, 32i8, 102i8, 105i8, 110i8,
    97i8, 108i8, 105i8, 122i8, 101i8, 114i8, 41i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.sysmodule.sys_get_asyncgen_hooks")]
extern "C-unwind" fn sys_get_asyncgen_hooks(
    module: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    sys_get_asyncgen_hooks_impl(module)
}

static sys_getrecursionlimit__doc__: __SlateAlign16<[i8; 259]> = __SlateAlign16([
    103i8, 101i8, 116i8, 114i8, 101i8, 99i8, 117i8, 114i8, 115i8, 105i8, 111i8, 110i8, 108i8,
    105i8, 109i8, 105i8, 116i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 44i8, 32i8,
    47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 116i8,
    104i8, 101i8, 32i8, 99i8, 117i8, 114i8, 114i8, 101i8, 110i8, 116i8, 32i8, 118i8, 97i8, 108i8,
    117i8, 101i8, 32i8, 111i8, 102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 114i8, 101i8, 99i8, 117i8,
    114i8, 115i8, 105i8, 111i8, 110i8, 32i8, 108i8, 105i8, 109i8, 105i8, 116i8, 46i8, 10i8, 10i8,
    84i8, 104i8, 101i8, 32i8, 114i8, 101i8, 99i8, 117i8, 114i8, 115i8, 105i8, 111i8, 110i8, 32i8,
    108i8, 105i8, 109i8, 105i8, 116i8, 32i8, 105i8, 115i8, 32i8, 116i8, 104i8, 101i8, 32i8, 109i8,
    97i8, 120i8, 105i8, 109i8, 117i8, 109i8, 32i8, 100i8, 101i8, 112i8, 116i8, 104i8, 32i8, 111i8,
    102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 80i8, 121i8, 116i8, 104i8, 111i8, 110i8, 32i8, 105i8,
    110i8, 116i8, 101i8, 114i8, 112i8, 114i8, 101i8, 116i8, 101i8, 114i8, 10i8, 115i8, 116i8, 97i8,
    99i8, 107i8, 46i8, 32i8, 32i8, 84i8, 104i8, 105i8, 115i8, 32i8, 108i8, 105i8, 109i8, 105i8,
    116i8, 32i8, 112i8, 114i8, 101i8, 118i8, 101i8, 110i8, 116i8, 115i8, 32i8, 105i8, 110i8, 102i8,
    105i8, 110i8, 105i8, 116i8, 101i8, 32i8, 114i8, 101i8, 99i8, 117i8, 114i8, 115i8, 105i8, 111i8,
    110i8, 32i8, 102i8, 114i8, 111i8, 109i8, 32i8, 99i8, 97i8, 117i8, 115i8, 105i8, 110i8, 103i8,
    32i8, 97i8, 110i8, 32i8, 111i8, 118i8, 101i8, 114i8, 102i8, 108i8, 111i8, 119i8, 10i8, 111i8,
    102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 67i8, 32i8, 115i8, 116i8, 97i8, 99i8, 107i8, 32i8,
    97i8, 110i8, 100i8, 32i8, 99i8, 114i8, 97i8, 115i8, 104i8, 105i8, 110i8, 103i8, 32i8, 80i8,
    121i8, 116i8, 104i8, 111i8, 110i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.sysmodule.sys_getrecursionlimit")]
extern "C-unwind" fn sys_getrecursionlimit(
    module: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    sys_getrecursionlimit_impl(module)
}

static sys_setdlopenflags__doc__: __SlateAlign16<[i8; 489]> = __SlateAlign16([
    115i8, 101i8, 116i8, 100i8, 108i8, 111i8, 112i8, 101i8, 110i8, 102i8, 108i8, 97i8, 103i8,
    115i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 44i8, 32i8, 102i8, 108i8, 97i8,
    103i8, 115i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 83i8, 101i8, 116i8, 32i8,
    116i8, 104i8, 101i8, 32i8, 102i8, 108i8, 97i8, 103i8, 115i8, 32i8, 117i8, 115i8, 101i8, 100i8,
    32i8, 98i8, 121i8, 32i8, 116i8, 104i8, 101i8, 32i8, 105i8, 110i8, 116i8, 101i8, 114i8, 112i8,
    114i8, 101i8, 116i8, 101i8, 114i8, 32i8, 102i8, 111i8, 114i8, 32i8, 100i8, 108i8, 111i8, 112i8,
    101i8, 110i8, 32i8, 99i8, 97i8, 108i8, 108i8, 115i8, 46i8, 10i8, 10i8, 84i8, 104i8, 105i8,
    115i8, 32i8, 105i8, 115i8, 32i8, 117i8, 115i8, 101i8, 100i8, 44i8, 32i8, 102i8, 111i8, 114i8,
    32i8, 101i8, 120i8, 97i8, 109i8, 112i8, 108i8, 101i8, 44i8, 32i8, 119i8, 104i8, 101i8, 110i8,
    32i8, 116i8, 104i8, 101i8, 32i8, 105i8, 110i8, 116i8, 101i8, 114i8, 112i8, 114i8, 101i8, 116i8,
    101i8, 114i8, 32i8, 108i8, 111i8, 97i8, 100i8, 115i8, 32i8, 101i8, 120i8, 116i8, 101i8, 110i8,
    115i8, 105i8, 111i8, 110i8, 10i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 115i8, 46i8, 32i8,
    65i8, 109i8, 111i8, 110i8, 103i8, 32i8, 111i8, 116i8, 104i8, 101i8, 114i8, 32i8, 116i8, 104i8,
    105i8, 110i8, 103i8, 115i8, 44i8, 32i8, 116i8, 104i8, 105i8, 115i8, 32i8, 119i8, 105i8, 108i8,
    108i8, 32i8, 101i8, 110i8, 97i8, 98i8, 108i8, 101i8, 32i8, 97i8, 32i8, 108i8, 97i8, 122i8,
    121i8, 32i8, 114i8, 101i8, 115i8, 111i8, 108i8, 118i8, 105i8, 110i8, 103i8, 32i8, 111i8, 102i8,
    10i8, 115i8, 121i8, 109i8, 98i8, 111i8, 108i8, 115i8, 32i8, 119i8, 104i8, 101i8, 110i8, 32i8,
    105i8, 109i8, 112i8, 111i8, 114i8, 116i8, 105i8, 110i8, 103i8, 32i8, 97i8, 32i8, 109i8, 111i8,
    100i8, 117i8, 108i8, 101i8, 44i8, 32i8, 105i8, 102i8, 32i8, 99i8, 97i8, 108i8, 108i8, 101i8,
    100i8, 32i8, 97i8, 115i8, 32i8, 115i8, 121i8, 115i8, 46i8, 115i8, 101i8, 116i8, 100i8, 108i8,
    111i8, 112i8, 101i8, 110i8, 102i8, 108i8, 97i8, 103i8, 115i8, 40i8, 48i8, 41i8, 46i8, 10i8,
    84i8, 111i8, 32i8, 115i8, 104i8, 97i8, 114i8, 101i8, 32i8, 115i8, 121i8, 109i8, 98i8, 111i8,
    108i8, 115i8, 32i8, 97i8, 99i8, 114i8, 111i8, 115i8, 115i8, 32i8, 101i8, 120i8, 116i8, 101i8,
    110i8, 115i8, 105i8, 111i8, 110i8, 32i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 115i8, 44i8,
    32i8, 99i8, 97i8, 108i8, 108i8, 32i8, 97i8, 115i8, 10i8, 115i8, 121i8, 115i8, 46i8, 115i8,
    101i8, 116i8, 100i8, 108i8, 111i8, 112i8, 101i8, 110i8, 102i8, 108i8, 97i8, 103i8, 115i8, 40i8,
    111i8, 115i8, 46i8, 82i8, 84i8, 76i8, 68i8, 95i8, 71i8, 76i8, 79i8, 66i8, 65i8, 76i8, 41i8,
    46i8, 32i8, 32i8, 83i8, 121i8, 109i8, 98i8, 111i8, 108i8, 105i8, 99i8, 32i8, 110i8, 97i8,
    109i8, 101i8, 115i8, 32i8, 102i8, 111i8, 114i8, 32i8, 116i8, 104i8, 101i8, 32i8, 102i8, 108i8,
    97i8, 103i8, 10i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 115i8, 32i8, 99i8, 97i8, 110i8,
    32i8, 98i8, 101i8, 32i8, 102i8, 111i8, 117i8, 110i8, 100i8, 32i8, 105i8, 110i8, 32i8, 116i8,
    104i8, 101i8, 32i8, 111i8, 115i8, 32i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 32i8, 40i8,
    82i8, 84i8, 76i8, 68i8, 95i8, 120i8, 120i8, 120i8, 32i8, 99i8, 111i8, 110i8, 115i8, 116i8,
    97i8, 110i8, 116i8, 115i8, 44i8, 32i8, 101i8, 46i8, 103i8, 46i8, 10i8, 111i8, 115i8, 46i8,
    82i8, 84i8, 76i8, 68i8, 95i8, 76i8, 65i8, 90i8, 89i8, 41i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.sysmodule.sys_setdlopenflags")]
extern "C-unwind" fn sys_setdlopenflags(module: *mut _object, arg: *mut _object) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut new_val: i32 = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        new_val = PyLong_AsInt(arg);
        if new_val == 1i32.wrapping_neg() {
            __v0 = !PyErr_Occurred().is_null();
        } else {
            __v0 = false;
        }
        if !__v0 {
            return_value = sys_setdlopenflags_impl(module, new_val);
        }
        return return_value;
    }
}

static sys_getdlopenflags__doc__: __SlateAlign16<[i8; 151]> = __SlateAlign16([
    103i8, 101i8, 116i8, 100i8, 108i8, 111i8, 112i8, 101i8, 110i8, 102i8, 108i8, 97i8, 103i8,
    115i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 44i8, 32i8, 47i8, 41i8, 10i8,
    45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 116i8, 104i8, 101i8,
    32i8, 99i8, 117i8, 114i8, 114i8, 101i8, 110i8, 116i8, 32i8, 118i8, 97i8, 108i8, 117i8, 101i8,
    32i8, 111i8, 102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 102i8, 108i8, 97i8, 103i8, 115i8, 32i8,
    116i8, 104i8, 97i8, 116i8, 32i8, 97i8, 114i8, 101i8, 32i8, 117i8, 115i8, 101i8, 100i8, 32i8,
    102i8, 111i8, 114i8, 32i8, 100i8, 108i8, 111i8, 112i8, 101i8, 110i8, 32i8, 99i8, 97i8, 108i8,
    108i8, 115i8, 46i8, 10i8, 10i8, 84i8, 104i8, 101i8, 32i8, 102i8, 108i8, 97i8, 103i8, 32i8,
    99i8, 111i8, 110i8, 115i8, 116i8, 97i8, 110i8, 116i8, 115i8, 32i8, 97i8, 114i8, 101i8, 32i8,
    100i8, 101i8, 102i8, 105i8, 110i8, 101i8, 100i8, 32i8, 105i8, 110i8, 32i8, 116i8, 104i8, 101i8,
    32i8, 111i8, 115i8, 32i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.sysmodule.sys_getdlopenflags")]
extern "C-unwind" fn sys_getdlopenflags(
    module: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    sys_getdlopenflags_impl(module)
}

static sys_get_int_max_str_digits__doc__: __SlateAlign16<[i8; 116]> = __SlateAlign16([
    103i8, 101i8, 116i8, 95i8, 105i8, 110i8, 116i8, 95i8, 109i8, 97i8, 120i8, 95i8, 115i8, 116i8,
    114i8, 95i8, 100i8, 105i8, 103i8, 105i8, 116i8, 115i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8,
    108i8, 101i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8,
    114i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 109i8, 97i8, 120i8, 105i8, 109i8, 117i8, 109i8,
    32i8, 115i8, 116i8, 114i8, 105i8, 110i8, 103i8, 32i8, 100i8, 105i8, 103i8, 105i8, 116i8, 115i8,
    32i8, 108i8, 105i8, 109i8, 105i8, 116i8, 32i8, 102i8, 111i8, 114i8, 32i8, 110i8, 111i8, 110i8,
    45i8, 98i8, 105i8, 110i8, 97i8, 114i8, 121i8, 32i8, 105i8, 110i8, 116i8, 60i8, 45i8, 62i8,
    115i8, 116i8, 114i8, 32i8, 99i8, 111i8, 110i8, 118i8, 101i8, 114i8, 115i8, 105i8, 111i8, 110i8,
    115i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.sysmodule.sys_get_int_max_str_digits")]
extern "C-unwind" fn sys_get_int_max_str_digits(
    module: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    sys_get_int_max_str_digits_impl(module)
}

static sys_set_int_max_str_digits__doc__: __SlateAlign16<[i8; 124]> = __SlateAlign16([
    115i8, 101i8, 116i8, 95i8, 105i8, 110i8, 116i8, 95i8, 109i8, 97i8, 120i8, 95i8, 115i8, 116i8,
    114i8, 95i8, 100i8, 105i8, 103i8, 105i8, 116i8, 115i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8,
    108i8, 101i8, 44i8, 32i8, 47i8, 44i8, 32i8, 109i8, 97i8, 120i8, 100i8, 105i8, 103i8, 105i8,
    116i8, 115i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 83i8, 101i8, 116i8, 32i8, 116i8, 104i8,
    101i8, 32i8, 109i8, 97i8, 120i8, 105i8, 109i8, 117i8, 109i8, 32i8, 115i8, 116i8, 114i8, 105i8,
    110i8, 103i8, 32i8, 100i8, 105i8, 103i8, 105i8, 116i8, 115i8, 32i8, 108i8, 105i8, 109i8, 105i8,
    116i8, 32i8, 102i8, 111i8, 114i8, 32i8, 110i8, 111i8, 110i8, 45i8, 98i8, 105i8, 110i8, 97i8,
    114i8, 121i8, 32i8, 105i8, 110i8, 116i8, 60i8, 45i8, 62i8, 115i8, 116i8, 114i8, 32i8, 99i8,
    111i8, 110i8, 118i8, 101i8, 114i8, 115i8, 105i8, 111i8, 110i8, 115i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.sysmodule.sys_set_int_max_str_digits")]
extern "C-unwind" fn sys_set_int_max_str_digits(
    module: *mut _object,
    mut args: *const *mut _object,
    nargs: i64,
    kwnames: *mut _object,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut maxdigits: i32 = unsafe { core::mem::zeroed() };
    let mut argsbuf: [*mut _object; 1] = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        args = _PyArg_UnpackKeywords!(
            args,
            nargs,
            core::ptr::null_mut::<()>(),
            kwnames,
            core::ptr::addr_of_mut!(_parser__943),
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
            maxdigits = PyLong_AsInt(*args.wrapping_offset(0 as isize));
            if maxdigits == 1i32.wrapping_neg() {
                __v0 = !PyErr_Occurred().is_null();
            } else {
                __v0 = false;
            }
            if !__v0 {
                return_value = sys_set_int_max_str_digits_impl(module, maxdigits);
            }
        }
        return return_value;
    }
}

static mut _kwtuple__930: __SlateRecord_sysmodule_c_h_930 = {
    let mut __t0: __SlateRecord_sysmodule_c_h_930 = unsafe { core::mem::zeroed() };
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
                ._py_maxdigits
                ._ascii
                .ob_base
        )
    }];
    __t0
};

static _keywords__942: __SlateAlign16<[*const i8; 2]> = __SlateAlign16([
    (b"maxdigits\0".as_ptr() as *mut i8) as *const i8,
    core::ptr::null::<i8>(),
]);

static mut _parser__943: _PyArg_Parser = {
    let mut __t0: _PyArg_Parser = unsafe { core::mem::zeroed() };
    __t0.keywords = unsafe { core::ptr::addr_of!(_keywords__942.0) as *const *const i8 };
    __t0.fname = (b"set_int_max_str_digits\0".as_ptr() as *mut i8) as *const i8;
    __t0.kwtuple = unsafe { core::ptr::addr_of_mut!(_kwtuple__930.ob_base.ob_base) };
    __t0
};

static sys_getrefcount__doc__: __SlateAlign16<[i8; 220]> = __SlateAlign16([
    103i8, 101i8, 116i8, 114i8, 101i8, 102i8, 99i8, 111i8, 117i8, 110i8, 116i8, 40i8, 36i8, 109i8,
    111i8, 100i8, 117i8, 108i8, 101i8, 44i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 44i8,
    32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8,
    116i8, 104i8, 101i8, 32i8, 114i8, 101i8, 102i8, 101i8, 114i8, 101i8, 110i8, 99i8, 101i8, 32i8,
    99i8, 111i8, 117i8, 110i8, 116i8, 32i8, 111i8, 102i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8,
    116i8, 46i8, 10i8, 10i8, 84i8, 104i8, 101i8, 32i8, 99i8, 111i8, 117i8, 110i8, 116i8, 32i8,
    114i8, 101i8, 116i8, 117i8, 114i8, 110i8, 101i8, 100i8, 32i8, 105i8, 115i8, 32i8, 103i8, 101i8,
    110i8, 101i8, 114i8, 97i8, 108i8, 108i8, 121i8, 32i8, 111i8, 110i8, 101i8, 32i8, 104i8, 105i8,
    103i8, 104i8, 101i8, 114i8, 32i8, 116i8, 104i8, 97i8, 110i8, 32i8, 121i8, 111i8, 117i8, 32i8,
    109i8, 105i8, 103i8, 104i8, 116i8, 32i8, 101i8, 120i8, 112i8, 101i8, 99i8, 116i8, 44i8, 10i8,
    98i8, 101i8, 99i8, 97i8, 117i8, 115i8, 101i8, 32i8, 105i8, 116i8, 32i8, 105i8, 110i8, 99i8,
    108i8, 117i8, 100i8, 101i8, 115i8, 32i8, 116i8, 104i8, 101i8, 32i8, 40i8, 116i8, 101i8, 109i8,
    112i8, 111i8, 114i8, 97i8, 114i8, 121i8, 41i8, 32i8, 114i8, 101i8, 102i8, 101i8, 114i8, 101i8,
    110i8, 99i8, 101i8, 32i8, 97i8, 115i8, 32i8, 97i8, 110i8, 32i8, 97i8, 114i8, 103i8, 117i8,
    109i8, 101i8, 110i8, 116i8, 32i8, 116i8, 111i8, 10i8, 103i8, 101i8, 116i8, 114i8, 101i8, 102i8,
    99i8, 111i8, 117i8, 110i8, 116i8, 40i8, 41i8, 46i8, 0i8,
]);

use slate_support::access;
use slate_support::close;
use slate_support::getpid;
use slate_support::memcpy;
use slate_support::snprintf;
use slate_support::strcmp;
use slate_support::strlen;
use slate_support::strrchr;
use slate_support::wcslen;

#[unsafe(link_section = ".text.slate_distinct.sysmodule.sys_getrefcount")]
extern "C-unwind" fn sys_getrefcount(module: *mut _object, object: *mut _object) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut _return_value: i64 = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        _return_value = sys_getrefcount_impl(module, object);
        if _return_value == (1i32.wrapping_neg() as i64) {
            __v0 = !PyErr_Occurred().is_null();
        } else {
            __v0 = false;
        }
        if !__v0 {
            return_value = PyLong_FromSsize_t(_return_value);
        }
        return return_value;
    }
}

static sys_getallocatedblocks__doc__: __SlateAlign16<[i8; 91]> = __SlateAlign16([
    103i8, 101i8, 116i8, 97i8, 108i8, 108i8, 111i8, 99i8, 97i8, 116i8, 101i8, 100i8, 98i8, 108i8,
    111i8, 99i8, 107i8, 115i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 44i8, 32i8,
    47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 116i8,
    104i8, 101i8, 32i8, 110i8, 117i8, 109i8, 98i8, 101i8, 114i8, 32i8, 111i8, 102i8, 32i8, 109i8,
    101i8, 109i8, 111i8, 114i8, 121i8, 32i8, 98i8, 108i8, 111i8, 99i8, 107i8, 115i8, 32i8, 99i8,
    117i8, 114i8, 114i8, 101i8, 110i8, 116i8, 108i8, 121i8, 32i8, 97i8, 108i8, 108i8, 111i8, 99i8,
    97i8, 116i8, 101i8, 100i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.sysmodule.sys_getallocatedblocks")]
extern "C-unwind" fn sys_getallocatedblocks(
    module: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut _return_value: i64 = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        _return_value = sys_getallocatedblocks_impl(module);
        if _return_value == (1i32.wrapping_neg() as i64) {
            __v0 = !PyErr_Occurred().is_null();
        } else {
            __v0 = false;
        }
        if !__v0 {
            return_value = PyLong_FromSsize_t(_return_value);
        }
        return return_value;
    }
}

static sys_getunicodeinternedsize__doc__: __SlateAlign16<[i8; 129]> = __SlateAlign16([
    103i8, 101i8, 116i8, 117i8, 110i8, 105i8, 99i8, 111i8, 100i8, 101i8, 105i8, 110i8, 116i8,
    101i8, 114i8, 110i8, 101i8, 100i8, 115i8, 105i8, 122i8, 101i8, 40i8, 36i8, 109i8, 111i8, 100i8,
    117i8, 108i8, 101i8, 44i8, 32i8, 47i8, 44i8, 32i8, 42i8, 44i8, 32i8, 95i8, 111i8, 110i8, 108i8,
    121i8, 95i8, 105i8, 109i8, 109i8, 111i8, 114i8, 116i8, 97i8, 108i8, 61i8, 70i8, 97i8, 108i8,
    115i8, 101i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8,
    32i8, 116i8, 104i8, 101i8, 32i8, 110i8, 117i8, 109i8, 98i8, 101i8, 114i8, 32i8, 111i8, 102i8,
    32i8, 101i8, 108i8, 101i8, 109i8, 101i8, 110i8, 116i8, 115i8, 32i8, 111i8, 102i8, 32i8, 116i8,
    104i8, 101i8, 32i8, 117i8, 110i8, 105i8, 99i8, 111i8, 100i8, 101i8, 32i8, 105i8, 110i8, 116i8,
    101i8, 114i8, 110i8, 101i8, 100i8, 32i8, 100i8, 105i8, 99i8, 116i8, 105i8, 111i8, 110i8, 97i8,
    114i8, 121i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.sysmodule.sys_getunicodeinternedsize")]
extern "C-unwind" fn sys_getunicodeinternedsize(
    module: *mut _object,
    mut args: *const *mut _object,
    nargs: i64,
    kwnames: *mut _object,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut _return_value: i64 = unsafe { core::mem::zeroed() };
    let mut _only_immortal: i32 = unsafe { core::mem::zeroed() };
    let mut __v3: i64 = unsafe { core::mem::zeroed() };
    let mut noptargs: i64 = unsafe { core::mem::zeroed() };
    let mut argsbuf: [*mut _object; 1] = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        if !kwnames.is_null() {
            __v3 = PyTuple_GET_SIZE!(kwnames);
        } else {
            __v3 = 0;
        }
        '__join_0: {
            noptargs = nargs.wrapping_add(__v3).wrapping_sub(0);
            _only_immortal = 0;
            args = _PyArg_UnpackKeywords!(
                args,
                nargs,
                core::ptr::null_mut::<()>(),
                kwnames,
                core::ptr::addr_of_mut!(_parser__1095),
                0i32,
                0i32,
                0i32,
                0i32,
                argsbuf.as_mut_ptr() as *mut *mut _object
            );
            // minpos
            // maxpos
            // minkw
            // varpos
            if !args.is_null() {
                if noptargs != 0 {
                    _only_immortal = PyObject_IsTrue(*args.wrapping_offset(0 as isize));
                    if _only_immortal < 0 {
                        break '__join_0;
                    }
                }
                _return_value = sys_getunicodeinternedsize_impl(module, _only_immortal);
                if _return_value == (1i32.wrapping_neg() as i64) {
                    __v0 = !PyErr_Occurred().is_null();
                } else {
                    __v0 = false;
                }
                if !__v0 {
                    return_value = PyLong_FromSsize_t(_return_value);
                }
            }
        }
        return return_value;
    }
}

static mut _kwtuple__1082: __SlateRecord_sysmodule_c_h_1082 = {
    let mut __t0: __SlateRecord_sysmodule_c_h_1082 = unsafe { core::mem::zeroed() };
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
                ._py__only_immortal
                ._ascii
                .ob_base
        )
    }];
    __t0
};

static _keywords__1094: __SlateAlign16<[*const i8; 2]> = __SlateAlign16([
    (b"_only_immortal\0".as_ptr() as *mut i8) as *const i8,
    core::ptr::null::<i8>(),
]);

static mut _parser__1095: _PyArg_Parser = {
    let mut __t0: _PyArg_Parser = unsafe { core::mem::zeroed() };
    __t0.keywords = unsafe { core::ptr::addr_of!(_keywords__1094.0) as *const *const i8 };
    __t0.fname = (b"getunicodeinternedsize\0".as_ptr() as *mut i8) as *const i8;
    __t0.kwtuple = unsafe { core::ptr::addr_of_mut!(_kwtuple__1082.ob_base.ob_base) };
    __t0
};

static sys__getframe__doc__: __SlateAlign16<[i8; 401]> = __SlateAlign16([
    95i8, 103i8, 101i8, 116i8, 102i8, 114i8, 97i8, 109i8, 101i8, 40i8, 36i8, 109i8, 111i8, 100i8,
    117i8, 108i8, 101i8, 44i8, 32i8, 100i8, 101i8, 112i8, 116i8, 104i8, 61i8, 48i8, 44i8, 32i8,
    47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 97i8,
    32i8, 102i8, 114i8, 97i8, 109i8, 101i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 32i8,
    102i8, 114i8, 111i8, 109i8, 32i8, 116i8, 104i8, 101i8, 32i8, 99i8, 97i8, 108i8, 108i8, 32i8,
    115i8, 116i8, 97i8, 99i8, 107i8, 46i8, 10i8, 10i8, 73i8, 102i8, 32i8, 111i8, 112i8, 116i8,
    105i8, 111i8, 110i8, 97i8, 108i8, 32i8, 105i8, 110i8, 116i8, 101i8, 103i8, 101i8, 114i8, 32i8,
    100i8, 101i8, 112i8, 116i8, 104i8, 32i8, 105i8, 115i8, 32i8, 103i8, 105i8, 118i8, 101i8, 110i8,
    44i8, 32i8, 114i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 102i8,
    114i8, 97i8, 109i8, 101i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 32i8, 116i8, 104i8,
    97i8, 116i8, 32i8, 109i8, 97i8, 110i8, 121i8, 10i8, 99i8, 97i8, 108i8, 108i8, 115i8, 32i8,
    98i8, 101i8, 108i8, 111i8, 119i8, 32i8, 116i8, 104i8, 101i8, 32i8, 116i8, 111i8, 112i8, 32i8,
    111i8, 102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 115i8, 116i8, 97i8, 99i8, 107i8, 46i8, 32i8,
    32i8, 73i8, 102i8, 32i8, 116i8, 104i8, 97i8, 116i8, 32i8, 105i8, 115i8, 32i8, 100i8, 101i8,
    101i8, 112i8, 101i8, 114i8, 32i8, 116i8, 104i8, 97i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8,
    99i8, 97i8, 108i8, 108i8, 10i8, 115i8, 116i8, 97i8, 99i8, 107i8, 44i8, 32i8, 86i8, 97i8, 108i8,
    117i8, 101i8, 69i8, 114i8, 114i8, 111i8, 114i8, 32i8, 105i8, 115i8, 32i8, 114i8, 97i8, 105i8,
    115i8, 101i8, 100i8, 46i8, 32i8, 32i8, 84i8, 104i8, 101i8, 32i8, 100i8, 101i8, 102i8, 97i8,
    117i8, 108i8, 116i8, 32i8, 102i8, 111i8, 114i8, 32i8, 100i8, 101i8, 112i8, 116i8, 104i8, 32i8,
    105i8, 115i8, 32i8, 122i8, 101i8, 114i8, 111i8, 44i8, 32i8, 114i8, 101i8, 116i8, 117i8, 114i8,
    110i8, 105i8, 110i8, 103i8, 10i8, 116i8, 104i8, 101i8, 32i8, 102i8, 114i8, 97i8, 109i8, 101i8,
    32i8, 97i8, 116i8, 32i8, 116i8, 104i8, 101i8, 32i8, 116i8, 111i8, 112i8, 32i8, 111i8, 102i8,
    32i8, 116i8, 104i8, 101i8, 32i8, 99i8, 97i8, 108i8, 108i8, 32i8, 115i8, 116i8, 97i8, 99i8,
    107i8, 46i8, 10i8, 10i8, 84i8, 104i8, 105i8, 115i8, 32i8, 102i8, 117i8, 110i8, 99i8, 116i8,
    105i8, 111i8, 110i8, 32i8, 115i8, 104i8, 111i8, 117i8, 108i8, 100i8, 32i8, 98i8, 101i8, 32i8,
    117i8, 115i8, 101i8, 100i8, 32i8, 102i8, 111i8, 114i8, 32i8, 105i8, 110i8, 116i8, 101i8, 114i8,
    110i8, 97i8, 108i8, 32i8, 97i8, 110i8, 100i8, 32i8, 115i8, 112i8, 101i8, 99i8, 105i8, 97i8,
    108i8, 105i8, 122i8, 101i8, 100i8, 32i8, 112i8, 117i8, 114i8, 112i8, 111i8, 115i8, 101i8,
    115i8, 10i8, 111i8, 110i8, 108i8, 121i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.sysmodule.sys__getframe")]
extern "C-unwind" fn sys__getframe(
    module: *mut _object,
    args: *const *mut _object,
    nargs: i64,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut depth: i32 = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_0: {
            return_value = core::ptr::null_mut::<_object>();
            depth = 0;
            if _PyArg_CheckPositional!(c"_getframe".as_ptr() as *mut i8, nargs, 0i32, 1i32) {
                if nargs >= 1 {
                    depth = PyLong_AsInt(*args.wrapping_offset(0 as isize));
                    if depth == 1i32.wrapping_neg() {
                        __v0 = !PyErr_Occurred().is_null();
                    } else {
                        __v0 = false;
                    }
                    if __v0 {
                        break '__join_0;
                    }
                }
                return_value = sys__getframe_impl(module, depth);
            }
        }
        return return_value;
    }
}

static sys__current_frames__doc__: __SlateAlign16<[i8; 167]> = __SlateAlign16([
    95i8, 99i8, 117i8, 114i8, 114i8, 101i8, 110i8, 116i8, 95i8, 102i8, 114i8, 97i8, 109i8, 101i8,
    115i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 44i8, 32i8, 47i8, 41i8, 10i8,
    45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 97i8, 32i8, 100i8,
    105i8, 99i8, 116i8, 32i8, 109i8, 97i8, 112i8, 112i8, 105i8, 110i8, 103i8, 32i8, 101i8, 97i8,
    99i8, 104i8, 32i8, 116i8, 104i8, 114i8, 101i8, 97i8, 100i8, 39i8, 115i8, 32i8, 116i8, 104i8,
    114i8, 101i8, 97i8, 100i8, 32i8, 105i8, 100i8, 32i8, 116i8, 111i8, 32i8, 105i8, 116i8, 115i8,
    32i8, 99i8, 117i8, 114i8, 114i8, 101i8, 110i8, 116i8, 32i8, 115i8, 116i8, 97i8, 99i8, 107i8,
    32i8, 102i8, 114i8, 97i8, 109i8, 101i8, 46i8, 10i8, 10i8, 84i8, 104i8, 105i8, 115i8, 32i8,
    102i8, 117i8, 110i8, 99i8, 116i8, 105i8, 111i8, 110i8, 32i8, 115i8, 104i8, 111i8, 117i8, 108i8,
    100i8, 32i8, 98i8, 101i8, 32i8, 117i8, 115i8, 101i8, 100i8, 32i8, 102i8, 111i8, 114i8, 32i8,
    115i8, 112i8, 101i8, 99i8, 105i8, 97i8, 108i8, 105i8, 122i8, 101i8, 100i8, 32i8, 112i8, 117i8,
    114i8, 112i8, 111i8, 115i8, 101i8, 115i8, 32i8, 111i8, 110i8, 108i8, 121i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.sysmodule.sys__current_frames")]
extern "C-unwind" fn sys__current_frames(
    module: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    sys__current_frames_impl(module)
}

static sys__current_exceptions__doc__: __SlateAlign16<[i8; 177]> = __SlateAlign16([
    95i8, 99i8, 117i8, 114i8, 114i8, 101i8, 110i8, 116i8, 95i8, 101i8, 120i8, 99i8, 101i8, 112i8,
    116i8, 105i8, 111i8, 110i8, 115i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 44i8,
    32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8,
    97i8, 32i8, 100i8, 105i8, 99i8, 116i8, 32i8, 109i8, 97i8, 112i8, 112i8, 105i8, 110i8, 103i8,
    32i8, 101i8, 97i8, 99i8, 104i8, 32i8, 116i8, 104i8, 114i8, 101i8, 97i8, 100i8, 39i8, 115i8,
    32i8, 105i8, 100i8, 101i8, 110i8, 116i8, 105i8, 102i8, 105i8, 101i8, 114i8, 32i8, 116i8, 111i8,
    32i8, 105i8, 116i8, 115i8, 32i8, 99i8, 117i8, 114i8, 114i8, 101i8, 110i8, 116i8, 32i8, 114i8,
    97i8, 105i8, 115i8, 101i8, 100i8, 32i8, 101i8, 120i8, 99i8, 101i8, 112i8, 116i8, 105i8, 111i8,
    110i8, 46i8, 10i8, 10i8, 84i8, 104i8, 105i8, 115i8, 32i8, 102i8, 117i8, 110i8, 99i8, 116i8,
    105i8, 111i8, 110i8, 32i8, 115i8, 104i8, 111i8, 117i8, 108i8, 100i8, 32i8, 98i8, 101i8, 32i8,
    117i8, 115i8, 101i8, 100i8, 32i8, 102i8, 111i8, 114i8, 32i8, 115i8, 112i8, 101i8, 99i8, 105i8,
    97i8, 108i8, 105i8, 122i8, 101i8, 100i8, 32i8, 112i8, 117i8, 114i8, 112i8, 111i8, 115i8, 101i8,
    115i8, 32i8, 111i8, 110i8, 108i8, 121i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.sysmodule.sys__current_exceptions")]
extern "C-unwind" fn sys__current_exceptions(
    module: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    sys__current_exceptions_impl(module)
}

static sys_call_tracing__doc__: __SlateAlign16<[i8; 243]> = __SlateAlign16([
    99i8, 97i8, 108i8, 108i8, 95i8, 116i8, 114i8, 97i8, 99i8, 105i8, 110i8, 103i8, 40i8, 36i8,
    109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 44i8, 32i8, 102i8, 117i8, 110i8, 99i8, 44i8, 32i8,
    97i8, 114i8, 103i8, 115i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 67i8, 97i8,
    108i8, 108i8, 32i8, 102i8, 117i8, 110i8, 99i8, 40i8, 42i8, 97i8, 114i8, 103i8, 115i8, 41i8,
    44i8, 32i8, 119i8, 104i8, 105i8, 108i8, 101i8, 32i8, 116i8, 114i8, 97i8, 99i8, 105i8, 110i8,
    103i8, 32i8, 105i8, 115i8, 32i8, 101i8, 110i8, 97i8, 98i8, 108i8, 101i8, 100i8, 46i8, 10i8,
    10i8, 84i8, 104i8, 101i8, 32i8, 116i8, 114i8, 97i8, 99i8, 105i8, 110i8, 103i8, 32i8, 115i8,
    116i8, 97i8, 116i8, 101i8, 32i8, 105i8, 115i8, 32i8, 115i8, 97i8, 118i8, 101i8, 100i8, 44i8,
    32i8, 97i8, 110i8, 100i8, 32i8, 114i8, 101i8, 115i8, 116i8, 111i8, 114i8, 101i8, 100i8, 32i8,
    97i8, 102i8, 116i8, 101i8, 114i8, 119i8, 97i8, 114i8, 100i8, 115i8, 46i8, 32i8, 32i8, 84i8,
    104i8, 105i8, 115i8, 32i8, 105i8, 115i8, 32i8, 105i8, 110i8, 116i8, 101i8, 110i8, 100i8, 101i8,
    100i8, 10i8, 116i8, 111i8, 32i8, 98i8, 101i8, 32i8, 99i8, 97i8, 108i8, 108i8, 101i8, 100i8,
    32i8, 102i8, 114i8, 111i8, 109i8, 32i8, 97i8, 32i8, 100i8, 101i8, 98i8, 117i8, 103i8, 103i8,
    101i8, 114i8, 32i8, 102i8, 114i8, 111i8, 109i8, 32i8, 97i8, 32i8, 99i8, 104i8, 101i8, 99i8,
    107i8, 112i8, 111i8, 105i8, 110i8, 116i8, 44i8, 32i8, 116i8, 111i8, 32i8, 114i8, 101i8, 99i8,
    117i8, 114i8, 115i8, 105i8, 118i8, 101i8, 108i8, 121i8, 32i8, 100i8, 101i8, 98i8, 117i8, 103i8,
    10i8, 115i8, 111i8, 109i8, 101i8, 32i8, 111i8, 116i8, 104i8, 101i8, 114i8, 32i8, 99i8, 111i8,
    100i8, 101i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.sysmodule.sys_call_tracing")]
extern "C-unwind" fn sys_call_tracing(
    module: *mut _object,
    args: *const *mut _object,
    nargs: i64,
) -> *mut _object {
    let mut func: *mut _object = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        if _PyArg_CheckPositional!(c"call_tracing".as_ptr() as *mut i8, nargs, 2i32, 2i32) {
            func = *args.wrapping_offset(0 as isize);
            if PyTuple_Check!(unsafe { *args.wrapping_offset(1 as isize) }) == 0 {
                _PyArg_BadArgument(
                    c"call_tracing".as_ptr(),
                    c"argument 2".as_ptr(),
                    c"tuple".as_ptr(),
                    *args.wrapping_offset(1 as isize),
                );
            } else {
                let funcargs: *mut _object = *args.wrapping_offset(1 as isize);
                return_value = sys_call_tracing_impl(module, func, funcargs);
            }
        }
        return return_value;
    }
}

static sys__debugmallocstats__doc__: __SlateAlign16<[i8; 181]> = __SlateAlign16([
    95i8, 100i8, 101i8, 98i8, 117i8, 103i8, 109i8, 97i8, 108i8, 108i8, 111i8, 99i8, 115i8, 116i8,
    97i8, 116i8, 115i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 44i8, 32i8, 47i8,
    41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 80i8, 114i8, 105i8, 110i8, 116i8, 32i8, 115i8, 117i8,
    109i8, 109i8, 97i8, 114i8, 121i8, 32i8, 105i8, 110i8, 102i8, 111i8, 32i8, 116i8, 111i8, 32i8,
    115i8, 116i8, 100i8, 101i8, 114i8, 114i8, 32i8, 97i8, 98i8, 111i8, 117i8, 116i8, 32i8, 116i8,
    104i8, 101i8, 32i8, 115i8, 116i8, 97i8, 116i8, 101i8, 32i8, 111i8, 102i8, 32i8, 112i8, 121i8,
    109i8, 97i8, 108i8, 108i8, 111i8, 99i8, 39i8, 115i8, 32i8, 115i8, 116i8, 114i8, 117i8, 99i8,
    116i8, 117i8, 114i8, 101i8, 115i8, 46i8, 10i8, 10i8, 73i8, 110i8, 32i8, 80i8, 121i8, 95i8,
    68i8, 69i8, 66i8, 85i8, 71i8, 32i8, 109i8, 111i8, 100i8, 101i8, 44i8, 32i8, 97i8, 108i8, 115i8,
    111i8, 32i8, 112i8, 101i8, 114i8, 102i8, 111i8, 114i8, 109i8, 32i8, 115i8, 111i8, 109i8, 101i8,
    32i8, 101i8, 120i8, 112i8, 101i8, 110i8, 115i8, 105i8, 118i8, 101i8, 32i8, 105i8, 110i8, 116i8,
    101i8, 114i8, 110i8, 97i8, 108i8, 32i8, 99i8, 111i8, 110i8, 115i8, 105i8, 115i8, 116i8, 101i8,
    110i8, 99i8, 121i8, 10i8, 99i8, 104i8, 101i8, 99i8, 107i8, 115i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.sysmodule.sys__debugmallocstats")]
extern "C-unwind" fn sys__debugmallocstats(
    module: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    sys__debugmallocstats_impl(module)
}

static sys__clear_type_cache__doc__: __SlateAlign16<[i8; 72]> = __SlateAlign16([
    95i8, 99i8, 108i8, 101i8, 97i8, 114i8, 95i8, 116i8, 121i8, 112i8, 101i8, 95i8, 99i8, 97i8,
    99i8, 104i8, 101i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 44i8, 32i8, 47i8,
    41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 67i8, 108i8, 101i8, 97i8, 114i8, 32i8, 116i8, 104i8, 101i8,
    32i8, 105i8, 110i8, 116i8, 101i8, 114i8, 110i8, 97i8, 108i8, 32i8, 116i8, 121i8, 112i8, 101i8,
    32i8, 108i8, 111i8, 111i8, 107i8, 117i8, 112i8, 32i8, 99i8, 97i8, 99i8, 104i8, 101i8, 46i8,
    0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.sysmodule.sys__clear_type_cache")]
extern "C-unwind" fn sys__clear_type_cache(
    module: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    sys__clear_type_cache_impl(module)
}

static sys__clear_internal_caches__doc__: __SlateAlign16<[i8; 86]> = __SlateAlign16([
    95i8, 99i8, 108i8, 101i8, 97i8, 114i8, 95i8, 105i8, 110i8, 116i8, 101i8, 114i8, 110i8, 97i8,
    108i8, 95i8, 99i8, 97i8, 99i8, 104i8, 101i8, 115i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8,
    108i8, 101i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 67i8, 108i8, 101i8, 97i8,
    114i8, 32i8, 97i8, 108i8, 108i8, 32i8, 105i8, 110i8, 116i8, 101i8, 114i8, 110i8, 97i8, 108i8,
    32i8, 112i8, 101i8, 114i8, 102i8, 111i8, 114i8, 109i8, 97i8, 110i8, 99i8, 101i8, 45i8, 114i8,
    101i8, 108i8, 97i8, 116i8, 101i8, 100i8, 32i8, 99i8, 97i8, 99i8, 104i8, 101i8, 115i8, 46i8,
    0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.sysmodule.sys__clear_internal_caches")]
extern "C-unwind" fn sys__clear_internal_caches(
    module: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    sys__clear_internal_caches_impl(module)
}

static sys_is_finalizing__doc__: __SlateAlign16<[i8; 64]> = __SlateAlign16([
    105i8, 115i8, 95i8, 102i8, 105i8, 110i8, 97i8, 108i8, 105i8, 122i8, 105i8, 110i8, 103i8, 40i8,
    36i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8,
    10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 84i8, 114i8, 117i8, 101i8, 32i8, 105i8,
    102i8, 32i8, 80i8, 121i8, 116i8, 104i8, 111i8, 110i8, 32i8, 105i8, 115i8, 32i8, 101i8, 120i8,
    105i8, 116i8, 105i8, 110i8, 103i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.sysmodule.sys_is_finalizing")]
extern "C-unwind" fn sys_is_finalizing(
    module: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    sys_is_finalizing_impl(module)
}

static sys_activate_stack_trampoline__doc__: __SlateAlign16<[i8; 97]> = __SlateAlign16([
    97i8, 99i8, 116i8, 105i8, 118i8, 97i8, 116i8, 101i8, 95i8, 115i8, 116i8, 97i8, 99i8, 107i8,
    95i8, 116i8, 114i8, 97i8, 109i8, 112i8, 111i8, 108i8, 105i8, 110i8, 101i8, 40i8, 36i8, 109i8,
    111i8, 100i8, 117i8, 108i8, 101i8, 44i8, 32i8, 98i8, 97i8, 99i8, 107i8, 101i8, 110i8, 100i8,
    44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 65i8, 99i8, 116i8, 105i8, 118i8, 97i8,
    116i8, 101i8, 32i8, 115i8, 116i8, 97i8, 99i8, 107i8, 32i8, 112i8, 114i8, 111i8, 102i8, 105i8,
    108i8, 101i8, 114i8, 32i8, 116i8, 114i8, 97i8, 109i8, 112i8, 111i8, 108i8, 105i8, 110i8, 101i8,
    32i8, 42i8, 98i8, 97i8, 99i8, 107i8, 101i8, 110i8, 100i8, 42i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.sysmodule.sys_activate_stack_trampoline")]
extern "C-unwind" fn sys_activate_stack_trampoline(
    module: *mut _object,
    arg: *mut _object,
) -> *mut _object {
    let mut backend_length: i64 = unsafe { core::mem::zeroed() };
    let mut backend: *const i8 = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        if PyUnicode_Check!(arg) == 0 {
            _PyArg_BadArgument(
                c"activate_stack_trampoline".as_ptr(),
                c"argument".as_ptr(),
                c"str".as_ptr(),
                arg,
            );
        } else {
            backend = PyUnicode_AsUTF8AndSize(arg, core::ptr::addr_of_mut!(backend_length));
            if !backend.is_null() {
                if (strlen(backend as *const core::ffi::c_char) as u64) != (backend_length as u64) {
                    PyErr_SetString(PyExc_ValueError, c"embedded null character".as_ptr());
                } else {
                    return_value = sys_activate_stack_trampoline_impl(module, backend);
                }
            }
        }
        return return_value;
    }
}

static sys_deactivate_stack_trampoline__doc__: __SlateAlign16<[i8; 167]> = __SlateAlign16([
    100i8, 101i8, 97i8, 99i8, 116i8, 105i8, 118i8, 97i8, 116i8, 101i8, 95i8, 115i8, 116i8, 97i8,
    99i8, 107i8, 95i8, 116i8, 114i8, 97i8, 109i8, 112i8, 111i8, 108i8, 105i8, 110i8, 101i8, 40i8,
    36i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8,
    10i8, 68i8, 101i8, 97i8, 99i8, 116i8, 105i8, 118i8, 97i8, 116i8, 101i8, 32i8, 116i8, 104i8,
    101i8, 32i8, 99i8, 117i8, 114i8, 114i8, 101i8, 110i8, 116i8, 32i8, 115i8, 116i8, 97i8, 99i8,
    107i8, 32i8, 112i8, 114i8, 111i8, 102i8, 105i8, 108i8, 101i8, 114i8, 32i8, 116i8, 114i8, 97i8,
    109i8, 112i8, 111i8, 108i8, 105i8, 110i8, 101i8, 32i8, 98i8, 97i8, 99i8, 107i8, 101i8, 110i8,
    100i8, 46i8, 10i8, 10i8, 73i8, 102i8, 32i8, 110i8, 111i8, 32i8, 115i8, 116i8, 97i8, 99i8,
    107i8, 32i8, 112i8, 114i8, 111i8, 102i8, 105i8, 108i8, 101i8, 114i8, 32i8, 105i8, 115i8, 32i8,
    97i8, 99i8, 116i8, 105i8, 118i8, 97i8, 116i8, 101i8, 100i8, 44i8, 32i8, 116i8, 104i8, 105i8,
    115i8, 32i8, 102i8, 117i8, 110i8, 99i8, 116i8, 105i8, 111i8, 110i8, 32i8, 104i8, 97i8, 115i8,
    32i8, 110i8, 111i8, 32i8, 101i8, 102i8, 102i8, 101i8, 99i8, 116i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.sysmodule.sys_deactivate_stack_trampoline")]
extern "C-unwind" fn sys_deactivate_stack_trampoline(
    module: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    sys_deactivate_stack_trampoline_impl(module)
}

static sys_is_stack_trampoline_active__doc__: __SlateAlign16<[i8; 99]> = __SlateAlign16([
    105i8, 115i8, 95i8, 115i8, 116i8, 97i8, 99i8, 107i8, 95i8, 116i8, 114i8, 97i8, 109i8, 112i8,
    111i8, 108i8, 105i8, 110i8, 101i8, 95i8, 97i8, 99i8, 116i8, 105i8, 118i8, 101i8, 40i8, 36i8,
    109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8,
    82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 42i8, 84i8, 114i8, 117i8, 101i8, 42i8, 32i8,
    105i8, 102i8, 32i8, 97i8, 32i8, 115i8, 116i8, 97i8, 99i8, 107i8, 32i8, 112i8, 114i8, 111i8,
    102i8, 105i8, 108i8, 101i8, 114i8, 32i8, 116i8, 114i8, 97i8, 109i8, 112i8, 111i8, 108i8, 105i8,
    110i8, 101i8, 32i8, 105i8, 115i8, 32i8, 97i8, 99i8, 116i8, 105i8, 118i8, 101i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.sysmodule.sys_is_stack_trampoline_active")]
extern "C-unwind" fn sys_is_stack_trampoline_active(
    module: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    sys_is_stack_trampoline_active_impl(module)
}

static sys_is_remote_debug_enabled__doc__: __SlateAlign16<[i8; 101]> = __SlateAlign16([
    105i8, 115i8, 95i8, 114i8, 101i8, 109i8, 111i8, 116i8, 101i8, 95i8, 100i8, 101i8, 98i8, 117i8,
    103i8, 95i8, 101i8, 110i8, 97i8, 98i8, 108i8, 101i8, 100i8, 40i8, 36i8, 109i8, 111i8, 100i8,
    117i8, 108i8, 101i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8,
    117i8, 114i8, 110i8, 32i8, 84i8, 114i8, 117i8, 101i8, 32i8, 105i8, 102i8, 32i8, 114i8, 101i8,
    109i8, 111i8, 116i8, 101i8, 32i8, 100i8, 101i8, 98i8, 117i8, 103i8, 103i8, 105i8, 110i8, 103i8,
    32i8, 105i8, 115i8, 32i8, 101i8, 110i8, 97i8, 98i8, 108i8, 101i8, 100i8, 44i8, 32i8, 70i8,
    97i8, 108i8, 115i8, 101i8, 32i8, 111i8, 116i8, 104i8, 101i8, 114i8, 119i8, 105i8, 115i8, 101i8,
    46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.sysmodule.sys_is_remote_debug_enabled")]
extern "C-unwind" fn sys_is_remote_debug_enabled(
    module: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    sys_is_remote_debug_enabled_impl(module)
}

static sys_remote_exec__doc__: __SlateAlign16<[i8; 955]> = __SlateAlign16([
    114i8, 101i8, 109i8, 111i8, 116i8, 101i8, 95i8, 101i8, 120i8, 101i8, 99i8, 40i8, 36i8, 109i8,
    111i8, 100i8, 117i8, 108i8, 101i8, 44i8, 32i8, 47i8, 44i8, 32i8, 112i8, 105i8, 100i8, 44i8,
    32i8, 115i8, 99i8, 114i8, 105i8, 112i8, 116i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 69i8, 120i8,
    101i8, 99i8, 117i8, 116i8, 101i8, 115i8, 32i8, 97i8, 32i8, 102i8, 105i8, 108i8, 101i8, 32i8,
    99i8, 111i8, 110i8, 116i8, 97i8, 105i8, 110i8, 105i8, 110i8, 103i8, 32i8, 80i8, 121i8, 116i8,
    104i8, 111i8, 110i8, 32i8, 99i8, 111i8, 100i8, 101i8, 32i8, 105i8, 110i8, 32i8, 97i8, 32i8,
    103i8, 105i8, 118i8, 101i8, 110i8, 32i8, 114i8, 101i8, 109i8, 111i8, 116i8, 101i8, 32i8, 80i8,
    121i8, 116i8, 104i8, 111i8, 110i8, 32i8, 112i8, 114i8, 111i8, 99i8, 101i8, 115i8, 115i8, 46i8,
    10i8, 10i8, 84i8, 104i8, 105i8, 115i8, 32i8, 102i8, 117i8, 110i8, 99i8, 116i8, 105i8, 111i8,
    110i8, 32i8, 114i8, 101i8, 116i8, 117i8, 114i8, 110i8, 115i8, 32i8, 105i8, 109i8, 109i8, 101i8,
    100i8, 105i8, 97i8, 116i8, 101i8, 108i8, 121i8, 44i8, 32i8, 97i8, 110i8, 100i8, 32i8, 116i8,
    104i8, 101i8, 32i8, 99i8, 111i8, 100i8, 101i8, 32i8, 119i8, 105i8, 108i8, 108i8, 32i8, 98i8,
    101i8, 32i8, 101i8, 120i8, 101i8, 99i8, 117i8, 116i8, 101i8, 100i8, 32i8, 98i8, 121i8, 32i8,
    116i8, 104i8, 101i8, 10i8, 116i8, 97i8, 114i8, 103i8, 101i8, 116i8, 32i8, 112i8, 114i8, 111i8,
    99i8, 101i8, 115i8, 115i8, 39i8, 115i8, 32i8, 109i8, 97i8, 105i8, 110i8, 32i8, 116i8, 104i8,
    114i8, 101i8, 97i8, 100i8, 32i8, 97i8, 116i8, 32i8, 116i8, 104i8, 101i8, 32i8, 110i8, 101i8,
    120i8, 116i8, 32i8, 97i8, 118i8, 97i8, 105i8, 108i8, 97i8, 98i8, 108i8, 101i8, 32i8, 111i8,
    112i8, 112i8, 111i8, 114i8, 116i8, 117i8, 110i8, 105i8, 116i8, 121i8, 44i8, 10i8, 115i8, 105i8,
    109i8, 105i8, 108i8, 97i8, 114i8, 108i8, 121i8, 32i8, 116i8, 111i8, 32i8, 104i8, 111i8, 119i8,
    32i8, 115i8, 105i8, 103i8, 110i8, 97i8, 108i8, 115i8, 32i8, 97i8, 114i8, 101i8, 32i8, 104i8,
    97i8, 110i8, 100i8, 108i8, 101i8, 100i8, 46i8, 32i8, 32i8, 84i8, 104i8, 101i8, 114i8, 101i8,
    32i8, 105i8, 115i8, 32i8, 110i8, 111i8, 32i8, 105i8, 110i8, 116i8, 101i8, 114i8, 102i8, 97i8,
    99i8, 101i8, 32i8, 116i8, 111i8, 10i8, 100i8, 101i8, 116i8, 101i8, 114i8, 109i8, 105i8, 110i8,
    101i8, 32i8, 119i8, 104i8, 101i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 99i8, 111i8, 100i8,
    101i8, 32i8, 104i8, 97i8, 115i8, 32i8, 98i8, 101i8, 101i8, 110i8, 32i8, 101i8, 120i8, 101i8,
    99i8, 117i8, 116i8, 101i8, 100i8, 46i8, 32i8, 32i8, 84i8, 104i8, 101i8, 32i8, 99i8, 97i8,
    108i8, 108i8, 101i8, 114i8, 32i8, 105i8, 115i8, 32i8, 114i8, 101i8, 115i8, 112i8, 111i8, 110i8,
    115i8, 105i8, 98i8, 108i8, 101i8, 10i8, 102i8, 111i8, 114i8, 32i8, 109i8, 97i8, 107i8, 105i8,
    110i8, 103i8, 32i8, 115i8, 117i8, 114i8, 101i8, 32i8, 116i8, 104i8, 97i8, 116i8, 32i8, 116i8,
    104i8, 101i8, 32i8, 102i8, 105i8, 108i8, 101i8, 32i8, 115i8, 116i8, 105i8, 108i8, 108i8, 32i8,
    101i8, 120i8, 105i8, 115i8, 116i8, 115i8, 32i8, 119i8, 104i8, 101i8, 110i8, 101i8, 118i8,
    101i8, 114i8, 32i8, 116i8, 104i8, 101i8, 32i8, 114i8, 101i8, 109i8, 111i8, 116i8, 101i8, 32i8,
    112i8, 114i8, 111i8, 99i8, 101i8, 115i8, 115i8, 10i8, 116i8, 114i8, 105i8, 101i8, 115i8, 32i8,
    116i8, 111i8, 32i8, 114i8, 101i8, 97i8, 100i8, 32i8, 105i8, 116i8, 32i8, 97i8, 110i8, 100i8,
    32i8, 116i8, 104i8, 97i8, 116i8, 32i8, 105i8, 116i8, 32i8, 104i8, 97i8, 115i8, 110i8, 39i8,
    116i8, 32i8, 98i8, 101i8, 101i8, 110i8, 32i8, 111i8, 118i8, 101i8, 114i8, 119i8, 114i8, 105i8,
    116i8, 116i8, 101i8, 110i8, 46i8, 10i8, 10i8, 84i8, 104i8, 101i8, 32i8, 114i8, 101i8, 109i8,
    111i8, 116i8, 101i8, 32i8, 112i8, 114i8, 111i8, 99i8, 101i8, 115i8, 115i8, 32i8, 109i8, 117i8,
    115i8, 116i8, 32i8, 98i8, 101i8, 32i8, 114i8, 117i8, 110i8, 110i8, 105i8, 110i8, 103i8, 32i8,
    97i8, 32i8, 67i8, 80i8, 121i8, 116i8, 104i8, 111i8, 110i8, 32i8, 105i8, 110i8, 116i8, 101i8,
    114i8, 112i8, 114i8, 101i8, 116i8, 101i8, 114i8, 32i8, 111i8, 102i8, 32i8, 116i8, 104i8, 101i8,
    32i8, 115i8, 97i8, 109i8, 101i8, 10i8, 109i8, 97i8, 106i8, 111i8, 114i8, 32i8, 97i8, 110i8,
    100i8, 32i8, 109i8, 105i8, 110i8, 111i8, 114i8, 32i8, 118i8, 101i8, 114i8, 115i8, 105i8, 111i8,
    110i8, 32i8, 97i8, 115i8, 32i8, 116i8, 104i8, 101i8, 32i8, 108i8, 111i8, 99i8, 97i8, 108i8,
    32i8, 112i8, 114i8, 111i8, 99i8, 101i8, 115i8, 115i8, 46i8, 32i8, 32i8, 73i8, 102i8, 32i8,
    101i8, 105i8, 116i8, 104i8, 101i8, 114i8, 32i8, 116i8, 104i8, 101i8, 32i8, 108i8, 111i8, 99i8,
    97i8, 108i8, 32i8, 111i8, 114i8, 10i8, 114i8, 101i8, 109i8, 111i8, 116i8, 101i8, 32i8, 105i8,
    110i8, 116i8, 101i8, 114i8, 112i8, 114i8, 101i8, 116i8, 101i8, 114i8, 32i8, 105i8, 115i8, 32i8,
    112i8, 114i8, 101i8, 45i8, 114i8, 101i8, 108i8, 101i8, 97i8, 115i8, 101i8, 32i8, 40i8, 97i8,
    108i8, 112i8, 104i8, 97i8, 44i8, 32i8, 98i8, 101i8, 116i8, 97i8, 44i8, 32i8, 111i8, 114i8,
    32i8, 114i8, 101i8, 108i8, 101i8, 97i8, 115i8, 101i8, 32i8, 99i8, 97i8, 110i8, 100i8, 105i8,
    100i8, 97i8, 116i8, 101i8, 41i8, 10i8, 116i8, 104i8, 101i8, 110i8, 32i8, 116i8, 104i8, 101i8,
    32i8, 108i8, 111i8, 99i8, 97i8, 108i8, 32i8, 97i8, 110i8, 100i8, 32i8, 114i8, 101i8, 109i8,
    111i8, 116i8, 101i8, 32i8, 105i8, 110i8, 116i8, 101i8, 114i8, 112i8, 114i8, 101i8, 116i8,
    101i8, 114i8, 115i8, 32i8, 109i8, 117i8, 115i8, 116i8, 32i8, 98i8, 101i8, 32i8, 116i8, 104i8,
    101i8, 32i8, 115i8, 97i8, 109i8, 101i8, 32i8, 101i8, 120i8, 97i8, 99i8, 116i8, 32i8, 118i8,
    101i8, 114i8, 115i8, 105i8, 111i8, 110i8, 46i8, 10i8, 10i8, 65i8, 114i8, 103i8, 115i8, 58i8,
    10i8, 32i8, 32i8, 32i8, 32i8, 32i8, 112i8, 105i8, 100i8, 32i8, 40i8, 105i8, 110i8, 116i8, 41i8,
    58i8, 32i8, 84i8, 104i8, 101i8, 32i8, 112i8, 114i8, 111i8, 99i8, 101i8, 115i8, 115i8, 32i8,
    73i8, 68i8, 32i8, 111i8, 102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 116i8, 97i8, 114i8, 103i8,
    101i8, 116i8, 32i8, 80i8, 121i8, 116i8, 104i8, 111i8, 110i8, 32i8, 112i8, 114i8, 111i8, 99i8,
    101i8, 115i8, 115i8, 46i8, 10i8, 32i8, 32i8, 32i8, 32i8, 32i8, 115i8, 99i8, 114i8, 105i8,
    112i8, 116i8, 32i8, 40i8, 115i8, 116i8, 114i8, 124i8, 98i8, 121i8, 116i8, 101i8, 115i8, 41i8,
    58i8, 32i8, 84i8, 104i8, 101i8, 32i8, 112i8, 97i8, 116i8, 104i8, 32i8, 116i8, 111i8, 32i8,
    97i8, 32i8, 102i8, 105i8, 108i8, 101i8, 32i8, 99i8, 111i8, 110i8, 116i8, 97i8, 105i8, 110i8,
    105i8, 110i8, 103i8, 10i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 116i8, 104i8,
    101i8, 32i8, 80i8, 121i8, 116i8, 104i8, 111i8, 110i8, 32i8, 99i8, 111i8, 100i8, 101i8, 32i8,
    116i8, 111i8, 32i8, 98i8, 101i8, 32i8, 101i8, 120i8, 101i8, 99i8, 117i8, 116i8, 101i8, 100i8,
    46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.sysmodule.sys_remote_exec")]
extern "C-unwind" fn sys_remote_exec(
    module: *mut _object,
    mut args: *const *mut _object,
    nargs: i64,
    kwnames: *mut _object,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut pid: i32 = unsafe { core::mem::zeroed() };
    let mut argsbuf: core::mem::MaybeUninit<__SlateAlign16<[*mut _object; 2]>> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_3: *mut [*mut _object; 2] =
        core::ptr::addr_of_mut!(argsbuf) as *mut [*mut _object; 2];
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        args = _PyArg_UnpackKeywords!(
            args,
            nargs,
            core::ptr::null_mut::<()>(),
            kwnames,
            core::ptr::addr_of_mut!(_parser__1595),
            2i32,
            2i32,
            0i32,
            0i32,
            (*__slate_slot_3).as_mut_ptr() as *mut *mut _object
        );
        // minpos
        // maxpos
        // minkw
        // varpos
        if !args.is_null() {
            pid = PyLong_AsInt(*args.wrapping_offset(0 as isize));
            if pid == 1i32.wrapping_neg() {
                __v0 = !PyErr_Occurred().is_null();
            } else {
                __v0 = false;
            }
            if !__v0 {
                let script: *mut _object = *args.wrapping_offset(1 as isize);
                return_value = sys_remote_exec_impl(module, pid, script);
            }
        }
        return return_value;
    }
}

static mut _kwtuple__1582: __SlateRecord_sysmodule_c_h_1582 = {
    let mut __t0: __SlateRecord_sysmodule_c_h_1582 = unsafe { core::mem::zeroed() };
    __t0.ob_base = __SlateRecord_object_h_172 {
        ob_base: _object {
            __slate_anon_0: __SlateRecord_object_h_122 {
                ob_refcnt_full: 1407378104778752i64,
            },
            ob_type: unsafe { core::ptr::addr_of_mut!(PyTuple_Type) },
        },
        ob_size: 2i64,
    };
    __t0.ob_hash = 1i32.wrapping_neg() as i64;
    __t0.ob_item = [
        unsafe {
            core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py_pid
                    ._ascii
                    .ob_base
            )
        },
        unsafe {
            core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py_script
                    ._ascii
                    .ob_base
            )
        },
    ];
    __t0
};

static _keywords__1594: __SlateAlign16<[*const i8; 3]> = __SlateAlign16([
    (b"pid\0".as_ptr() as *mut i8) as *const i8,
    (b"script\0".as_ptr() as *mut i8) as *const i8,
    core::ptr::null::<i8>(),
]);

static mut _parser__1595: _PyArg_Parser = {
    let mut __t0: _PyArg_Parser = unsafe { core::mem::zeroed() };
    __t0.keywords = unsafe { core::ptr::addr_of!(_keywords__1594.0) as *const *const i8 };
    __t0.fname = (b"remote_exec\0".as_ptr() as *mut i8) as *const i8;
    __t0.kwtuple = unsafe { core::ptr::addr_of_mut!(_kwtuple__1582.ob_base.ob_base) };
    __t0
};

static sys__dump_tracelets__doc__: __SlateAlign16<[i8; 88]> = __SlateAlign16([
    95i8, 100i8, 117i8, 109i8, 112i8, 95i8, 116i8, 114i8, 97i8, 99i8, 101i8, 108i8, 101i8, 116i8,
    115i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 44i8, 32i8, 47i8, 44i8, 32i8,
    111i8, 117i8, 116i8, 112i8, 97i8, 116i8, 104i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 68i8,
    117i8, 109i8, 112i8, 32i8, 116i8, 104i8, 101i8, 32i8, 103i8, 114i8, 97i8, 112i8, 104i8, 32i8,
    111i8, 102i8, 32i8, 116i8, 114i8, 97i8, 99i8, 101i8, 108i8, 101i8, 116i8, 115i8, 32i8, 105i8,
    110i8, 32i8, 103i8, 114i8, 97i8, 112i8, 104i8, 118i8, 105i8, 122i8, 32i8, 102i8, 111i8, 114i8,
    109i8, 97i8, 116i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.sysmodule.sys__dump_tracelets")]
extern "C-unwind" fn sys__dump_tracelets(
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
            core::ptr::addr_of_mut!(_parser__1658),
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
            let outpath: *mut _object = *args.wrapping_offset(0 as isize);
            return_value = sys__dump_tracelets_impl(module, outpath);
        }
        return return_value;
    }
}

static mut _kwtuple__1645: __SlateRecord_sysmodule_c_h_1645 = {
    let mut __t0: __SlateRecord_sysmodule_c_h_1645 = unsafe { core::mem::zeroed() };
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
                ._py_outpath
                ._ascii
                .ob_base
        )
    }];
    __t0
};

static _keywords__1657: __SlateAlign16<[*const i8; 2]> = __SlateAlign16([
    (b"outpath\0".as_ptr() as *mut i8) as *const i8,
    core::ptr::null::<i8>(),
]);

static mut _parser__1658: _PyArg_Parser = {
    let mut __t0: _PyArg_Parser = unsafe { core::mem::zeroed() };
    __t0.keywords = unsafe { core::ptr::addr_of!(_keywords__1657.0) as *const *const i8 };
    __t0.fname = (b"_dump_tracelets\0".as_ptr() as *mut i8) as *const i8;
    __t0.kwtuple = unsafe { core::ptr::addr_of_mut!(_kwtuple__1645.ob_base.ob_base) };
    __t0
};

static sys__getframemodulename__doc__: __SlateAlign16<[i8; 337]> = __SlateAlign16([
    95i8, 103i8, 101i8, 116i8, 102i8, 114i8, 97i8, 109i8, 101i8, 109i8, 111i8, 100i8, 117i8, 108i8,
    101i8, 110i8, 97i8, 109i8, 101i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 44i8,
    32i8, 47i8, 44i8, 32i8, 100i8, 101i8, 112i8, 116i8, 104i8, 61i8, 48i8, 41i8, 10i8, 45i8, 45i8,
    10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 110i8,
    97i8, 109i8, 101i8, 32i8, 111i8, 102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 109i8, 111i8, 100i8,
    117i8, 108i8, 101i8, 32i8, 102i8, 111i8, 114i8, 32i8, 97i8, 32i8, 99i8, 97i8, 108i8, 108i8,
    105i8, 110i8, 103i8, 32i8, 102i8, 114i8, 97i8, 109i8, 101i8, 46i8, 10i8, 10i8, 84i8, 104i8,
    101i8, 32i8, 100i8, 101i8, 102i8, 97i8, 117i8, 108i8, 116i8, 32i8, 100i8, 101i8, 112i8, 116i8,
    104i8, 32i8, 114i8, 101i8, 116i8, 117i8, 114i8, 110i8, 115i8, 32i8, 116i8, 104i8, 101i8, 32i8,
    109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 32i8, 99i8, 111i8, 110i8, 116i8, 97i8, 105i8, 110i8,
    105i8, 110i8, 103i8, 32i8, 116i8, 104i8, 101i8, 32i8, 99i8, 97i8, 108i8, 108i8, 32i8, 116i8,
    111i8, 32i8, 116i8, 104i8, 105i8, 115i8, 32i8, 65i8, 80i8, 73i8, 46i8, 10i8, 65i8, 32i8, 109i8,
    111i8, 114i8, 101i8, 32i8, 116i8, 121i8, 112i8, 105i8, 99i8, 97i8, 108i8, 32i8, 117i8, 115i8,
    101i8, 32i8, 105i8, 110i8, 32i8, 97i8, 32i8, 108i8, 105i8, 98i8, 114i8, 97i8, 114i8, 121i8,
    32i8, 119i8, 105i8, 108i8, 108i8, 32i8, 112i8, 97i8, 115i8, 115i8, 32i8, 97i8, 32i8, 100i8,
    101i8, 112i8, 116i8, 104i8, 32i8, 111i8, 102i8, 32i8, 49i8, 32i8, 116i8, 111i8, 32i8, 103i8,
    101i8, 116i8, 32i8, 116i8, 104i8, 101i8, 32i8, 117i8, 115i8, 101i8, 114i8, 39i8, 115i8, 10i8,
    109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 32i8, 114i8, 97i8, 116i8, 104i8, 101i8, 114i8, 32i8,
    116i8, 104i8, 97i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 108i8, 105i8, 98i8, 114i8, 97i8,
    114i8, 121i8, 32i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 46i8, 10i8, 10i8, 73i8, 102i8,
    32i8, 110i8, 111i8, 32i8, 102i8, 114i8, 97i8, 109i8, 101i8, 44i8, 32i8, 109i8, 111i8, 100i8,
    117i8, 108i8, 101i8, 44i8, 32i8, 111i8, 114i8, 32i8, 110i8, 97i8, 109i8, 101i8, 32i8, 99i8,
    97i8, 110i8, 32i8, 98i8, 101i8, 32i8, 102i8, 111i8, 117i8, 110i8, 100i8, 44i8, 32i8, 114i8,
    101i8, 116i8, 117i8, 114i8, 110i8, 115i8, 32i8, 78i8, 111i8, 110i8, 101i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.sysmodule.sys__getframemodulename")]
extern "C-unwind" fn sys__getframemodulename(
    module: *mut _object,
    mut args: *const *mut _object,
    nargs: i64,
    kwnames: *mut _object,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut depth: i32 = unsafe { core::mem::zeroed() };
    let mut __v2: i64 = unsafe { core::mem::zeroed() };
    let mut noptargs: i64 = unsafe { core::mem::zeroed() };
    let mut argsbuf: [*mut _object; 1] = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        if !kwnames.is_null() {
            __v2 = PyTuple_GET_SIZE!(kwnames);
        } else {
            __v2 = 0;
        }
        '__join_0: {
            noptargs = nargs.wrapping_add(__v2).wrapping_sub(0);
            depth = 0;
            args = _PyArg_UnpackKeywords!(
                args,
                nargs,
                core::ptr::null_mut::<()>(),
                kwnames,
                core::ptr::addr_of_mut!(_parser__1722),
                0i32,
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
                if noptargs != 0 {
                    depth = PyLong_AsInt(*args.wrapping_offset(0 as isize));
                    if depth == 1i32.wrapping_neg() {
                        __v0 = !PyErr_Occurred().is_null();
                    } else {
                        __v0 = false;
                    }
                    if __v0 {
                        break '__join_0;
                    }
                }
                return_value = sys__getframemodulename_impl(module, depth);
            }
        }
        return return_value;
    }
}

static mut _kwtuple__1709: __SlateRecord_sysmodule_c_h_1709 = {
    let mut __t0: __SlateRecord_sysmodule_c_h_1709 = unsafe { core::mem::zeroed() };
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
                ._py_depth
                ._ascii
                .ob_base
        )
    }];
    __t0
};

static _keywords__1721: __SlateAlign16<[*const i8; 2]> = __SlateAlign16([
    (b"depth\0".as_ptr() as *mut i8) as *const i8,
    core::ptr::null::<i8>(),
]);

static mut _parser__1722: _PyArg_Parser = {
    let mut __t0: _PyArg_Parser = unsafe { core::mem::zeroed() };
    __t0.keywords = unsafe { core::ptr::addr_of!(_keywords__1721.0) as *const *const i8 };
    __t0.fname = (b"_getframemodulename\0".as_ptr() as *mut i8) as *const i8;
    __t0.kwtuple = unsafe { core::ptr::addr_of_mut!(_kwtuple__1709.ob_base.ob_base) };
    __t0
};

static sys__get_cpu_count_config__doc__: __SlateAlign16<[i8; 86]> = __SlateAlign16([
    95i8, 103i8, 101i8, 116i8, 95i8, 99i8, 112i8, 117i8, 95i8, 99i8, 111i8, 117i8, 110i8, 116i8,
    95i8, 99i8, 111i8, 110i8, 102i8, 105i8, 103i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8,
    101i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 80i8, 114i8, 105i8, 118i8, 97i8,
    116i8, 101i8, 32i8, 102i8, 117i8, 110i8, 99i8, 116i8, 105i8, 111i8, 110i8, 32i8, 102i8, 111i8,
    114i8, 32i8, 103i8, 101i8, 116i8, 116i8, 105i8, 110i8, 103i8, 32i8, 80i8, 121i8, 67i8, 111i8,
    110i8, 102i8, 105i8, 103i8, 46i8, 99i8, 112i8, 117i8, 95i8, 99i8, 111i8, 117i8, 110i8, 116i8,
    0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.sysmodule.sys__get_cpu_count_config")]
extern "C-unwind" fn sys__get_cpu_count_config(
    module: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut _return_value: i32 = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        _return_value = sys__get_cpu_count_config_impl(module);
        if _return_value == 1i32.wrapping_neg() {
            __v0 = !PyErr_Occurred().is_null();
        } else {
            __v0 = false;
        }
        if !__v0 {
            return_value = PyLong_FromLong(_return_value as i64);
        }
        return return_value;
    }
}

static sys__baserepl__doc__: __SlateAlign16<[i8; 69]> = __SlateAlign16([
    95i8, 98i8, 97i8, 115i8, 101i8, 114i8, 101i8, 112i8, 108i8, 40i8, 36i8, 109i8, 111i8, 100i8,
    117i8, 108i8, 101i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 80i8, 114i8, 105i8,
    118i8, 97i8, 116i8, 101i8, 32i8, 102i8, 117i8, 110i8, 99i8, 116i8, 105i8, 111i8, 110i8, 32i8,
    102i8, 111i8, 114i8, 32i8, 103i8, 101i8, 116i8, 116i8, 105i8, 110i8, 103i8, 32i8, 116i8, 104i8,
    101i8, 32i8, 98i8, 97i8, 115i8, 101i8, 32i8, 82i8, 69i8, 80i8, 76i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.sysmodule.sys__baserepl")]
extern "C-unwind" fn sys__baserepl(
    module: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    sys__baserepl_impl(module)
}

static sys__clear_type_descriptors__doc__: __SlateAlign16<[i8; 150]> = __SlateAlign16([
    95i8, 99i8, 108i8, 101i8, 97i8, 114i8, 95i8, 116i8, 121i8, 112i8, 101i8, 95i8, 100i8, 101i8,
    115i8, 99i8, 114i8, 105i8, 112i8, 116i8, 111i8, 114i8, 115i8, 40i8, 36i8, 109i8, 111i8, 100i8,
    117i8, 108i8, 101i8, 44i8, 32i8, 116i8, 121i8, 112i8, 101i8, 44i8, 32i8, 47i8, 41i8, 10i8,
    45i8, 45i8, 10i8, 10i8, 80i8, 114i8, 105i8, 118i8, 97i8, 116i8, 101i8, 32i8, 102i8, 117i8,
    110i8, 99i8, 116i8, 105i8, 111i8, 110i8, 32i8, 102i8, 111i8, 114i8, 32i8, 99i8, 108i8, 101i8,
    97i8, 114i8, 105i8, 110i8, 103i8, 32i8, 99i8, 101i8, 114i8, 116i8, 97i8, 105i8, 110i8, 32i8,
    100i8, 101i8, 115i8, 99i8, 114i8, 105i8, 112i8, 116i8, 111i8, 114i8, 115i8, 32i8, 102i8, 114i8,
    111i8, 109i8, 32i8, 97i8, 32i8, 116i8, 121i8, 112i8, 101i8, 39i8, 115i8, 32i8, 100i8, 105i8,
    99i8, 116i8, 105i8, 111i8, 110i8, 97i8, 114i8, 121i8, 46i8, 10i8, 10i8, 83i8, 101i8, 101i8,
    32i8, 103i8, 104i8, 45i8, 49i8, 51i8, 53i8, 50i8, 50i8, 56i8, 32i8, 102i8, 111i8, 114i8, 32i8,
    99i8, 111i8, 110i8, 116i8, 101i8, 120i8, 116i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.sysmodule.sys__clear_type_descriptors")]
extern "C-unwind" fn sys__clear_type_descriptors(
    module: *mut _object,
    arg: *mut _object,
) -> *mut _object {
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        if PyObject_TypeCheck!(arg, core::ptr::addr_of_mut!(PyType_Type)) == 0 {
            _PyArg_BadArgument(
                c"_clear_type_descriptors".as_ptr(),
                c"argument".as_ptr(),
                (*core::ptr::addr_of_mut!(PyType_Type)).tp_name,
                arg,
            );
        } else {
            let r#type: *mut _object = arg;
            return_value = sys__clear_type_descriptors_impl(module, r#type);
        }
        return return_value;
    }
}

static sys__is_gil_enabled__doc__: __SlateAlign16<[i8; 97]> = __SlateAlign16([
    95i8, 105i8, 115i8, 95i8, 103i8, 105i8, 108i8, 95i8, 101i8, 110i8, 97i8, 98i8, 108i8, 101i8,
    100i8, 40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 44i8, 32i8, 47i8, 41i8, 10i8,
    45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 84i8, 114i8, 117i8,
    101i8, 32i8, 105i8, 102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 71i8, 73i8, 76i8, 32i8, 105i8,
    115i8, 32i8, 99i8, 117i8, 114i8, 114i8, 101i8, 110i8, 116i8, 108i8, 121i8, 32i8, 101i8, 110i8,
    97i8, 98i8, 108i8, 101i8, 100i8, 32i8, 97i8, 110i8, 100i8, 32i8, 70i8, 97i8, 108i8, 115i8,
    101i8, 32i8, 111i8, 116i8, 104i8, 101i8, 114i8, 119i8, 105i8, 115i8, 101i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.sysmodule.sys__is_gil_enabled")]
extern "C-unwind" fn sys__is_gil_enabled(
    module: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut _return_value: i32 = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        _return_value = sys__is_gil_enabled_impl(module);
        if _return_value == 1i32.wrapping_neg() {
            __v0 = !PyErr_Occurred().is_null();
        } else {
            __v0 = false;
        }
        if !__v0 {
            return_value = PyBool_FromLong(_return_value as i64);
        }
        return return_value;
    }
}

static _jit_is_available__doc__: __SlateAlign16<[i8; 121]> = __SlateAlign16([
    105i8, 115i8, 95i8, 97i8, 118i8, 97i8, 105i8, 108i8, 97i8, 98i8, 108i8, 101i8, 40i8, 36i8,
    109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8,
    82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 84i8, 114i8, 117i8, 101i8, 32i8, 105i8, 102i8,
    32i8, 116i8, 104i8, 101i8, 32i8, 99i8, 117i8, 114i8, 114i8, 101i8, 110i8, 116i8, 32i8, 80i8,
    121i8, 116i8, 104i8, 111i8, 110i8, 32i8, 101i8, 120i8, 101i8, 99i8, 117i8, 116i8, 97i8, 98i8,
    108i8, 101i8, 32i8, 115i8, 117i8, 112i8, 112i8, 111i8, 114i8, 116i8, 115i8, 32i8, 74i8, 73i8,
    84i8, 32i8, 99i8, 111i8, 109i8, 112i8, 105i8, 108i8, 97i8, 116i8, 105i8, 111i8, 110i8, 44i8,
    32i8, 97i8, 110i8, 100i8, 32i8, 70i8, 97i8, 108i8, 115i8, 101i8, 32i8, 111i8, 116i8, 104i8,
    101i8, 114i8, 119i8, 105i8, 115i8, 101i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.sysmodule._jit_is_available")]
extern "C-unwind" fn _jit_is_available(
    module: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut _return_value: i32 = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        _return_value = _jit_is_available_impl(module);
        if _return_value == 1i32.wrapping_neg() {
            __v0 = !PyErr_Occurred().is_null();
        } else {
            __v0 = false;
        }
        if !__v0 {
            return_value = PyBool_FromLong(_return_value as i64);
        }
        return return_value;
    }
}

static _jit_is_enabled__doc__: __SlateAlign16<[i8; 156]> = __SlateAlign16([
    105i8, 115i8, 95i8, 101i8, 110i8, 97i8, 98i8, 108i8, 101i8, 100i8, 40i8, 36i8, 109i8, 111i8,
    100i8, 117i8, 108i8, 101i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8,
    116i8, 117i8, 114i8, 110i8, 32i8, 84i8, 114i8, 117i8, 101i8, 32i8, 105i8, 102i8, 32i8, 74i8,
    73i8, 84i8, 32i8, 99i8, 111i8, 109i8, 112i8, 105i8, 108i8, 97i8, 116i8, 105i8, 111i8, 110i8,
    32i8, 105i8, 115i8, 32i8, 101i8, 110i8, 97i8, 98i8, 108i8, 101i8, 100i8, 32i8, 102i8, 111i8,
    114i8, 32i8, 116i8, 104i8, 101i8, 32i8, 99i8, 117i8, 114i8, 114i8, 101i8, 110i8, 116i8, 32i8,
    80i8, 121i8, 116i8, 104i8, 111i8, 110i8, 32i8, 112i8, 114i8, 111i8, 99i8, 101i8, 115i8, 115i8,
    32i8, 40i8, 105i8, 109i8, 112i8, 108i8, 105i8, 101i8, 115i8, 32i8, 115i8, 121i8, 115i8, 46i8,
    95i8, 106i8, 105i8, 116i8, 46i8, 105i8, 115i8, 95i8, 97i8, 118i8, 97i8, 105i8, 108i8, 97i8,
    98i8, 108i8, 101i8, 40i8, 41i8, 41i8, 44i8, 32i8, 97i8, 110i8, 100i8, 32i8, 70i8, 97i8, 108i8,
    115i8, 101i8, 32i8, 111i8, 116i8, 104i8, 101i8, 114i8, 119i8, 105i8, 115i8, 101i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.sysmodule._jit_is_enabled")]
extern "C-unwind" fn _jit_is_enabled(
    module: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut _return_value: i32 = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        _return_value = _jit_is_enabled_impl(module);
        if _return_value == 1i32.wrapping_neg() {
            __v0 = !PyErr_Occurred().is_null();
        } else {
            __v0 = false;
        }
        if !__v0 {
            return_value = PyBool_FromLong(_return_value as i64);
        }
        return return_value;
    }
}

static _jit_is_active__doc__: __SlateAlign16<[i8; 152]> = __SlateAlign16([
    105i8, 115i8, 95i8, 97i8, 99i8, 116i8, 105i8, 118i8, 101i8, 40i8, 36i8, 109i8, 111i8, 100i8,
    117i8, 108i8, 101i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8,
    117i8, 114i8, 110i8, 32i8, 84i8, 114i8, 117i8, 101i8, 32i8, 105i8, 102i8, 32i8, 116i8, 104i8,
    101i8, 32i8, 116i8, 111i8, 112i8, 109i8, 111i8, 115i8, 116i8, 32i8, 80i8, 121i8, 116i8, 104i8,
    111i8, 110i8, 32i8, 102i8, 114i8, 97i8, 109i8, 101i8, 32i8, 105i8, 115i8, 32i8, 99i8, 117i8,
    114i8, 114i8, 101i8, 110i8, 116i8, 108i8, 121i8, 32i8, 101i8, 120i8, 101i8, 99i8, 117i8, 116i8,
    105i8, 110i8, 103i8, 32i8, 74i8, 73i8, 84i8, 32i8, 99i8, 111i8, 100i8, 101i8, 32i8, 40i8,
    105i8, 109i8, 112i8, 108i8, 105i8, 101i8, 115i8, 32i8, 115i8, 121i8, 115i8, 46i8, 95i8, 106i8,
    105i8, 116i8, 46i8, 105i8, 115i8, 95i8, 101i8, 110i8, 97i8, 98i8, 108i8, 101i8, 100i8, 40i8,
    41i8, 41i8, 44i8, 32i8, 97i8, 110i8, 100i8, 32i8, 70i8, 97i8, 108i8, 115i8, 101i8, 32i8, 111i8,
    116i8, 104i8, 101i8, 114i8, 119i8, 105i8, 115i8, 101i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.sysmodule._jit_is_active")]
extern "C-unwind" fn _jit_is_active(
    module: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut _return_value: i32 = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        _return_value = _jit_is_active_impl(module);
        if _return_value == 1i32.wrapping_neg() {
            __v0 = !PyErr_Occurred().is_null();
        } else {
            __v0 = false;
        }
        if !__v0 {
            return_value = PyBool_FromLong(_return_value as i64);
        }
        return return_value;
    }
}

// Various bits of information used by the interpreter are collected in
// module 'sys'.
// Function member:
// - exit(sts): raise SystemExit
// Data members:
// - stdin, stdout, stderr: standard file objects
// - modules: the table of modules (dictionary)
// - path: module search path (list of strings)
// - argv: script arguments (list of strings)
// - ps1, ps2: optional primary and secondary prompts (strings)
// _Py_AuditHookEntry
// _PyObject_CallNoArgs()
// _PyEval_SetAsyncGenFinalizer()
// _PyInterpreterFrame
// _PyImport_SetDLOpenFlags()
// _PyStatus_EXCEPTION()
// _PyFrame_GetFirstComplete()
// _PY_LONG_MAX_STR_DIGITS_THRESHOLD
// _PyModule_CreateInitialized()
// _PyNamespace_New()
// _PyObject_DebugTypeStats()
// _PyDumpExecutors()
// _PyPathConfig_ComputeSysPath0()
// _PyErr_GetRaisedException()
// _PyErr_WriteUnraisableDefaultHook()
// _PY_SHORT_FLOAT_REPR
// _PyMem_DefaultRawFree()
// _PyThreadState_GET()
// _Py_PrintSpecializationStats()
// _PyStructSequence_InitBuiltinWithFlags()
// export _PySys_GetSizeOf()
// _PyUnicode_InternImmortal()
// PyDTrace_AUDIT()
// DELIM
// _Py_stdlib_module_names
// getpid()
// [clinic input]
// module sys
// [clinic start generated code]
//
// [clinic end generated code: output=da39a3ee5e6b4b0d input=3726b388feee8cea]
#[unsafe(no_mangle)]
extern "C-unwind" fn _PySys_GetRequiredAttr(name: *mut _object) -> *mut _object {
    if PyUnicode_Check!(name) == 0 {
        unsafe {
            PyErr_Format(
                PyExc_TypeError,
                c"attribute name must be string, not '%.200s'".as_ptr(),
                (*Py_TYPE!(name)).tp_name,
            )
        };
        return core::ptr::null_mut::<_object>();
    }
    let tstate: *mut _ts = _PyThreadState_GET();
    let sysdict: *mut _object = unsafe { (*(*tstate).interp).sysdict };
    if sysdict.is_null() {
        unsafe { PyErr_SetString(PyExc_RuntimeError, c"no sys module".as_ptr()) };
        return core::ptr::null_mut::<_object>();
    }
    let mut value: *mut _object = unsafe { core::mem::zeroed() };
    if (unsafe { PyDict_GetItemRef(sysdict, name, core::ptr::addr_of_mut!(value)) }) == 0 {
        unsafe { PyErr_Format(PyExc_RuntimeError, c"lost sys.%U".as_ptr(), name) };
    }
    value
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PySys_GetRequiredAttrString(name: *const i8) -> *mut _object {
    let tstate: *mut _ts = _PyThreadState_GET();
    let sysdict: *mut _object = unsafe { (*(*tstate).interp).sysdict };
    if sysdict.is_null() {
        unsafe { PyErr_SetString(PyExc_RuntimeError, c"no sys module".as_ptr()) };
        return core::ptr::null_mut::<_object>();
    }
    let mut value: *mut _object = unsafe { core::mem::zeroed() };
    if (unsafe { PyDict_GetItemStringRef(sysdict, name, core::ptr::addr_of_mut!(value)) }) == 0 {
        unsafe { PyErr_Format(PyExc_RuntimeError, c"lost sys.%s".as_ptr(), name) };
    }
    value
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PySys_GetOptionalAttr(name: *mut _object, value: *mut *mut _object) -> i32 {
    if PyUnicode_Check!(name) == 0 {
        unsafe {
            PyErr_Format(
                PyExc_TypeError,
                c"attribute name must be string, not '%.200s'".as_ptr(),
                (*Py_TYPE!(name)).tp_name,
            )
        };
        unsafe {
            *value = core::ptr::null_mut::<_object>();
        }
        return 1i32.wrapping_neg();
    }
    let tstate: *mut _ts = _PyThreadState_GET();
    let sysdict: *mut _object = unsafe { (*(*tstate).interp).sysdict };
    if sysdict.is_null() {
        unsafe {
            *value = core::ptr::null_mut::<_object>();
        }
        return 0;
    }
    unsafe { PyDict_GetItemRef(sysdict, name, value) }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PySys_GetOptionalAttrString(
    name: *const i8,
    value: *mut *mut _object,
) -> i32 {
    let tstate: *mut _ts = _PyThreadState_GET();
    let sysdict: *mut _object = unsafe { (*(*tstate).interp).sysdict };
    if sysdict.is_null() {
        unsafe {
            *value = core::ptr::null_mut::<_object>();
        }
        return 0;
    }
    unsafe { PyDict_GetItemStringRef(sysdict, name, value) }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PySys_GetObject(name: *const i8) -> *mut _object {
    let tstate: *mut _ts = _PyThreadState_GET();
    let sysdict: *mut _object = unsafe { (*(*tstate).interp).sysdict };
    if sysdict.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let exc: *mut _object = unsafe { _PyErr_GetRaisedException(tstate) };
    let mut value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe { PyDict_GetItemStringRef(sysdict, name, core::ptr::addr_of_mut!(value)) };
    // XXX Suppress a new exception if it was raised and restore
    // the old one.
    if !_PyErr_Occurred(tstate).is_null() {
        unsafe { PyErr_FormatUnraisable(c"Exception ignored in PySys_GetObject()".as_ptr()) };
    }
    unsafe { _PyErr_SetRaisedException(tstate, exc) };
    Py_XDECREF!(value); // return a borrowed reference
    value
}

fn sys_set_object(interp: *mut _is, key: *mut _object, v: *mut _object) -> i32 {
    if key.is_null() {
        return 1i32.wrapping_neg();
    }
    let sd: *mut _object = unsafe { (*interp).sysdict };
    if sd.is_null() {
        unsafe { PyErr_SetString(PyExc_RuntimeError, c"no sys module".as_ptr()) };
        return 1i32.wrapping_neg();
    }
    if v.is_null() {
        if (unsafe { PyDict_Pop(sd, key, core::ptr::null_mut::<*mut _object>()) }) < 0 {
            return 1i32.wrapping_neg();
        }
        return 0;
    } else {
        return unsafe { PyDict_SetItem(sd, key, v) };
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PySys_SetAttr(key: *mut _object, v: *mut _object) -> i32 {
    let interp: *mut _is = _PyInterpreterState_GET();
    sys_set_object(interp, key, v)
}

fn sys_set_object_str(interp: *mut _is, name: *const i8, v: *mut _object) -> i32 {
    let __v0: *mut _object = if !v.is_null() {
        unsafe { PyUnicode_InternFromString(name) }
    } else {
        unsafe { PyUnicode_FromString(name) }
    };
    let key: *mut _object = __v0;
    let r: i32 = sys_set_object(interp, key, v);
    Py_XDECREF!(key);
    r
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PySys_SetObject(name: *const i8, v: *mut _object) -> i32 {
    let interp: *mut _is = _PyInterpreterState_GET();
    sys_set_object_str(interp, name, v)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PySys_ClearAttrString(
    interp: *mut _is,
    name: *const i8,
    verbose: i32,
) -> i32 {
    if verbose != 0 {
        unsafe { PySys_WriteStderr(c"# clear sys.%s\n".as_ptr(), name) };
    }
    // To play it safe, we set the attr to None instead of deleting it.
    if (unsafe {
        PyDict_SetItemString(
            (*interp).sysdict,
            name,
            core::ptr::addr_of_mut!(_Py_NoneStruct),
        )
    }) < 0
    {
        return 1i32.wrapping_neg();
    }
    0
}

fn should_audit(interp: *mut _is) -> i32 {
    // interp must not be NULL, but test it just in case for extra safety
    if interp.is_null() {
        return 0;
    }
    let __v0: bool = !unsafe { (*(*interp).runtime).audit_hooks.head }.is_null()
        || !unsafe { (*interp).audit_hooks }.is_null()
        || PyDTrace_AUDIT_ENABLED() != 0;
    __v0 as i32
}

fn sys_audit_tstate(
    ts: *mut _ts,
    event: *const i8,
    argFormat: *const i8,
    mut vargs: core::ffi::VaList<'static>,
) -> i32 {
    let mut args: core::mem::MaybeUninit<__SlateAlign16<[*mut _object; 2]>> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_1: *mut [*mut _object; 2] =
        core::ptr::addr_of_mut!(args) as *mut [*mut _object; 2];
    let mut canTrace: i32 = unsafe { core::mem::zeroed() };
    let mut o: *mut _object = unsafe { core::mem::zeroed() };
    // Call global hooks
    //
    // We don't worry about any races on hooks getting added,
    // since that would not leave is in an inconsistent state.
    let mut e: *mut _Py_AuditHookEntry = unsafe { core::mem::zeroed() };
    let mut __v6: bool = unsafe { core::mem::zeroed() };
    let mut exc: *mut _object = unsafe { core::mem::zeroed() };
    let mut dtrace: i32 = unsafe { core::mem::zeroed() };
    let mut res: i32 = unsafe { core::mem::zeroed() };
    let mut hook: *mut _object = unsafe { core::mem::zeroed() };
    let mut hooks: *mut _object = unsafe { core::mem::zeroed() };
    let mut eventArgs: *mut _object = unsafe { core::mem::zeroed() };
    let mut eventName: *mut _object = unsafe { core::mem::zeroed() };
    // Early exit when no hooks are registered
    let mut is: *mut _is = unsafe { core::mem::zeroed() };
    unsafe {
        if ts.is_null() {
            // Audit hooks cannot be called with a NULL thread state
            return 0;
        } else {
            // The current implementation cannot be called if tstate is not
            // the current Python thread state.
            is = (*ts).interp;
            if should_audit(is) == 0 {
                return 0;
            } else {
                eventName = core::ptr::null_mut::<_object>();
                eventArgs = core::ptr::null_mut::<_object>();
                hooks = core::ptr::null_mut::<_object>();
                hook = core::ptr::null_mut::<_object>();
                res = 1i32.wrapping_neg();
                dtrace = PyDTrace_AUDIT_ENABLED();
                exc = _PyErr_GetRaisedException(ts);
                // Initialize event args now
                if !argFormat.is_null() && *argFormat.wrapping_offset(0 as isize) != 0 {
                    eventArgs = Py_VaBuildValue(argFormat, vargs.clone());
                    if !eventArgs.is_null() {
                        __v6 = PyTuple_Check!(eventArgs) == 0;
                    } else {
                        __v6 = false;
                    }
                    if __v6 {
                        let argTuple: *mut _object = PyTuple_Pack(1i64, eventArgs);
                        Py_SETREF!(eventArgs, argTuple);
                    }
                } else {
                    eventArgs = PyTuple_New(0i64);
                }
                '__join_3: {
                    if !eventArgs.is_null() {
                        e = (*(*is).runtime).audit_hooks.head;
                        while !e.is_null() {
                            if unsafe { (*e).hookCFunction }.unwrap()(
                                event,
                                eventArgs,
                                (*e).userData,
                            ) < 0
                            {
                                break '__join_3;
                            } else {
                                e = (*e).next;
                            }
                        }
                        // Dtrace USDT point
                        if dtrace != 0 {
                            PyDTrace_AUDIT(event, eventArgs as *mut ());
                        }
                        // Call interpreter hooks
                        if !unsafe { (*is).audit_hooks }.is_null() {
                            eventName = PyUnicode_FromString(event);
                            if eventName.is_null() {
                                break '__join_3;
                            } else {
                                hooks = PyObject_GetIter((*is).audit_hooks);
                                if hooks.is_null() {
                                    break '__join_3;
                                } else {
                                    // Disallow tracing in hooks unless explicitly enabled
                                    PyThreadState_EnterTracing(ts);
                                    loop {
                                        let __v0: *mut _object = PyIter_Next(hooks);
                                        hook = __v0;
                                        if !__v0.is_null() {
                                            canTrace = PyObject_GetOptionalAttr(
                                                hook,
                                                core::ptr::addr_of_mut!(
                                                    _PyRuntime
                                                        .static_objects
                                                        .singletons
                                                        .strings
                                                        .identifiers
                                                        ._py___cantrace__
                                                        ._ascii
                                                        .ob_base
                                                ),
                                                core::ptr::addr_of_mut!(o),
                                            );
                                            if !o.is_null() {
                                                canTrace = PyObject_IsTrue(o);
                                                Py_DECREF!(o);
                                            }
                                            if canTrace < 0 {
                                                break;
                                            } else {
                                                if canTrace != 0 {
                                                    PyThreadState_LeaveTracing(ts);
                                                }
                                                core::ptr::write(
                                                    __slate_slot_1,
                                                    [eventName, eventArgs],
                                                );
                                                o = _PyObject_VectorcallTstate(
                                                    ts,
                                                    hook,
                                                    (*__slate_slot_1).as_mut_ptr()
                                                        as *const *mut _object,
                                                    2u64,
                                                    core::ptr::null_mut::<_object>(),
                                                );
                                                if canTrace != 0 {
                                                    PyThreadState_EnterTracing(ts);
                                                }
                                                if o.is_null() {
                                                    break;
                                                } else {
                                                    Py_DECREF!(o);
                                                    Py_CLEAR__2!(hook);
                                                }
                                            }
                                        } else {
                                            break;
                                        }
                                    }
                                    PyThreadState_LeaveTracing(ts);
                                    if !_PyErr_Occurred(ts).is_null() {
                                        break '__join_3;
                                    }
                                }
                            }
                        }
                        res = 0;
                    }
                }
                Py_XDECREF!(hook);
                Py_XDECREF!(hooks);
                Py_XDECREF!(eventName);
                Py_XDECREF!(eventArgs);
                if res == 0 {
                    _PyErr_SetRaisedException(ts, exc);
                } else {
                    Py_XDECREF!(exc);
                }
                return res;
            }
        }
    }
}

#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn _PySys_Audit(
    tstate: *mut _ts,
    event: *const i8,
    argFormat: *const i8,
    mut __va_args: ...
) -> i32 {
    let mut vargs: core::ffi::VaList<'static> = unsafe { core::mem::zeroed() };
    vargs = unsafe {
        core::mem::transmute::<core::ffi::VaList<'_>, core::ffi::VaList<'static>>(__va_args.clone())
    };
    let res: i32 = sys_audit_tstate(tstate, event, argFormat, vargs.clone());
    res
}

#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn PySys_Audit(
    event: *const i8,
    argFormat: *const i8,
    mut __va_args: ...
) -> i32 {
    let tstate: *mut _ts = _PyThreadState_GET();
    let mut vargs: core::ffi::VaList<'static> = unsafe { core::mem::zeroed() };
    vargs = unsafe {
        core::mem::transmute::<core::ffi::VaList<'_>, core::ffi::VaList<'static>>(__va_args.clone())
    };
    let res: i32 = sys_audit_tstate(tstate, event, argFormat, vargs.clone());
    res
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PySys_AuditTuple(event: *const i8, args: *mut _object) -> i32 {
    if args.is_null() {
        return unsafe { PySys_Audit(event, core::ptr::null::<i8>()) };
    }
    if PyTuple_Check!(args) == 0 {
        unsafe {
            PyErr_Format(
                PyExc_TypeError,
                c"args must be tuple, got %s".as_ptr(),
                (*Py_TYPE!(args)).tp_name,
            )
        };
        return 1i32.wrapping_neg();
    }
    unsafe { PySys_Audit(event, c"O".as_ptr(), args) }
}

/// We expose this function primarily for our own cleanup during
/// finalization. In general, it should not need to be called,
/// and as such the function is not exported.
///
/// Must be finalizing to clear hooks
#[unsafe(no_mangle)]
extern "C-unwind" fn _PySys_ClearAuditHooks(ts: *mut _ts) {
    if ts.is_null() {
        return;
    }
    let runtime: *mut pyruntimestate = unsafe { (*(*ts).interp).runtime };
    // The hooks are global so we have to check for runtime finalization.
    let finalizing: *mut _ts = _PyRuntimeState_GetFinalizing(runtime);
    if finalizing != ts {
        return;
    }
    let config: *const PyConfig = unsafe { _PyInterpreterState_GetConfig((*ts).interp) };
    if (unsafe { (*config).verbose }) != 0 {
        unsafe { PySys_WriteStderr(c"# clear sys.audit hooks\n".as_ptr()) };
    }
    // Hooks can abort later hooks for this event, but cannot
    // abort the clear operation itself.
    unsafe {
        _PySys_Audit(
            ts,
            c"cpython._PySys_ClearAuditHooks".as_ptr(),
            core::ptr::null::<i8>(),
        )
    };
    unsafe { _PyErr_Clear(ts) };
    // We don't worry about the very unlikely race right here,
    // since it's entirely benign.  Nothing else removes entries
    // from the list and adding an entry right now would not cause
    // any trouble.
    let mut e: *mut _Py_AuditHookEntry = unsafe { (*runtime).audit_hooks.head };
    unsafe {
        (*runtime).audit_hooks.head = core::ptr::null_mut::<_Py_AuditHookEntry>();
    }
    while !e.is_null() {
        let n: *mut _Py_AuditHookEntry = unsafe { (*e).next };
        unsafe { PyMem_RawFree(e as *mut ()) };
        e = n;
    }
}

fn add_audit_hook_entry_unlocked(runtime: *mut pyruntimestate, entry: *mut _Py_AuditHookEntry) {
    if unsafe { (*runtime).audit_hooks.head }.is_null() {
        unsafe {
            (*runtime).audit_hooks.head = entry;
        }
    } else {
        let mut last: *mut _Py_AuditHookEntry = unsafe { (*runtime).audit_hooks.head };
        while !unsafe { (*last).next }.is_null() {
            last = unsafe { (*last).next };
        }
        unsafe {
            (*last).next = entry;
        }
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PySys_AddAuditHook(
    hook: Option<unsafe extern "C-unwind" fn(*const i8, *mut _object, *mut ()) -> i32>,
    userData: *mut (),
) -> i32 {
    // tstate can be NULL, so access directly _PyRuntime:
    // PySys_AddAuditHook() can be called before Python is initialized.
    let runtime: *mut pyruntimestate = unsafe { core::ptr::addr_of_mut!(_PyRuntime) };
    let mut tstate: *mut _ts = unsafe { core::mem::zeroed() };
    if (unsafe { (*runtime).initialized }) != 0 {
        tstate = _PyThreadState_GET();
    } else {
        tstate = core::ptr::null_mut::<_ts>();
    }
    // Invoke existing audit hooks to allow them an opportunity to abort.
    //
    // Cannot invoke hooks until we are initialized
    if !tstate.is_null() {
        if (unsafe {
            _PySys_Audit(
                tstate,
                c"sys.addaudithook".as_ptr(),
                core::ptr::null::<i8>(),
            )
        }) < 0
        {
            if (unsafe { _PyErr_ExceptionMatches(tstate, PyExc_RuntimeError) }) != 0 {
                // We do not report errors derived from RuntimeError
                unsafe { _PyErr_Clear(tstate) };
                return 0;
            }
            return 1i32.wrapping_neg();
        }
    }
    let e: *mut _Py_AuditHookEntry =
        (unsafe { PyMem_RawMalloc(core::mem::size_of::<_Py_AuditHookEntry>() as u64) })
            as *mut _Py_AuditHookEntry;
    if e.is_null() {
        if !tstate.is_null() {
            unsafe { _PyErr_NoMemory(tstate) };
        }
        return 1i32.wrapping_neg();
    }
    unsafe {
        (*e).next = core::ptr::null_mut::<_Py_AuditHookEntry>();
    }
    unsafe {
        (*e).hookCFunction = hook;
    }
    unsafe {
        (*e).userData = userData;
    }
    _PyMutex_Lock(unsafe { core::ptr::addr_of_mut!((*runtime).audit_hooks.mutex) });
    add_audit_hook_entry_unlocked(runtime, e);
    _PyMutex_Unlock(unsafe { core::ptr::addr_of_mut!((*runtime).audit_hooks.mutex) });
    0
}

// [clinic input]
// sys.addaudithook
//
//     hook: object
//
// Adds a new audit hook callback.
// [clinic start generated code]
/// [clinic end generated code: output=4f9c17aaeb02f44e input=0f3e191217a45e34]
fn sys_addaudithook_impl(module: *mut _object, hook: *mut _object) -> *mut _object {
    let tstate: *mut _ts = _PyThreadState_GET();
    // Invoke existing audit hooks to allow them an opportunity to abort.
    if (unsafe {
        _PySys_Audit(
            tstate,
            c"sys.addaudithook".as_ptr(),
            core::ptr::null::<i8>(),
        )
    }) < 0
    {
        if (unsafe { _PyErr_ExceptionMatches(tstate, PyExc_Exception) }) != 0 {
            // We do not report errors derived from Exception
            unsafe { _PyErr_Clear(tstate) };
            return unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) };
        }
        return core::ptr::null_mut::<_object>();
    }
    let interp: *mut _is = unsafe { (*tstate).interp };
    if unsafe { (*interp).audit_hooks }.is_null() {
        unsafe {
            (*interp).audit_hooks = PyList_New(0i64);
        }
        if unsafe { (*interp).audit_hooks }.is_null() {
            return core::ptr::null_mut::<_object>();
        }
        // Avoid having our list of hooks show up in the GC module
        unsafe { PyObject_GC_UnTrack((*interp).audit_hooks as *mut ()) };
    }
    if (unsafe { PyList_Append((*interp).audit_hooks, hook) }) < 0 {
        return core::ptr::null_mut::<_object>();
    }
    unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
}

// [clinic input]
// sys.audit
//
//     event: str
//     /
//     *args: tuple
//
// Passes the event to any audit hooks that are attached.
// [clinic start generated code]
/// [clinic end generated code: output=1d0fc82da768f49d input=ec3b688527945109]
fn sys_audit_impl(module: *mut _object, event: *const i8, args: *mut _object) -> *mut _object {
    let tstate: *mut _ts = _PyThreadState_GET();
    _Py_EnsureTstateNotNULL__7!(tstate);
    if should_audit(unsafe { (*tstate).interp }) == 0 {
        return unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) };
    }
    let res: i32 = unsafe { _PySys_Audit(tstate, event, c"O".as_ptr(), args) };
    if res < 0 {
        return core::ptr::null_mut::<_object>();
    }
    unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
}

#[unsafe(link_section = ".text.slate_distinct.sysmodule.sys_breakpointhook")]
extern "C-unwind" fn sys_breakpointhook(
    self_: *mut _object,
    args: *const *mut _object,
    nargs: i64,
    keywords__0: *mut _object,
) -> *mut _object {
    let mut hook: *mut _object = unsafe { core::mem::zeroed() };
    let mut module: *mut _object = unsafe { core::mem::zeroed() };
    let mut modulepath: *mut _object = unsafe { core::mem::zeroed() };
    let mut attrname: *const i8 = unsafe { core::mem::zeroed() };
    let mut last_dot: *const i8 = unsafe { core::mem::zeroed() };
    let mut envar: *mut i8 = unsafe { core::mem::zeroed() };
    let mut tstate: *mut _ts = unsafe { core::mem::zeroed() };
    unsafe {
        tstate = _PyThreadState_GET();
        envar = Py_GETENV(c"PYTHONBREAKPOINT".as_ptr());
        if envar.is_null() || (strlen((envar as *const i8) as *const core::ffi::c_char) as u64) == 0
        {
            envar = c"pdb.set_trace".as_ptr() as *mut i8;
        } else {
            if (strcmp(
                (envar as *const i8) as *const core::ffi::c_char,
                c"0".as_ptr() as *const core::ffi::c_char,
            ) as i32)
                == 0
            {
                // The breakpoint is explicitly no-op'd.
                return core::ptr::addr_of_mut!(_Py_NoneStruct);
            }
        }
        // According to POSIX the string returned by getenv() might be invalidated
        // or the string content might be overwritten by a subsequent call to
        // getenv().  Since importing a module can performs the getenv() calls,
        // we need to save a copy of envar.
        envar = _PyMem_RawStrdup(envar as *const i8);
        if envar.is_null() {
            _PyErr_NoMemory(tstate);
            return core::ptr::null_mut::<_object>();
        } else {
            '__join_2: {
                last_dot = (strrchr(
                    (envar as *const i8) as *const core::ffi::c_char,
                    46i32 as i32,
                ) as *mut i8) as *const i8;
                attrname = core::ptr::null::<i8>();
                modulepath = core::ptr::null_mut::<_object>();
                if last_dot.is_null() {
                    // The breakpoint is a built-in, e.g. PYTHONBREAKPOINT=int
                    modulepath = PyUnicode_FromString(c"builtins".as_ptr());
                    attrname = envar as *const i8;
                } else {
                    if last_dot != (envar as *const i8) {
                        // Split on the last dot;
                        modulepath = PyUnicode_FromStringAndSize(
                            envar as *const i8,
                            last_dot.offset_from(envar as *const i8) as i64,
                        );
                        attrname = last_dot.wrapping_offset(1 as isize);
                    } else {
                        break '__join_2;
                    }
                }
                if modulepath.is_null() {
                    PyMem_RawFree(envar as *mut ());
                    return core::ptr::null_mut::<_object>();
                } else {
                    module = PyImport_Import(modulepath);
                    Py_DECREF!(modulepath);
                    if module.is_null() {
                        if _PyErr_ExceptionMatches(tstate, PyExc_ImportError) == 0 {
                            PyMem_RawFree(envar as *mut ());
                            return core::ptr::null_mut::<_object>();
                        }
                    } else {
                        hook = PyObject_GetAttrString(module, attrname);
                        Py_DECREF!(module);
                        if hook.is_null() {
                            if _PyErr_ExceptionMatches(tstate, PyExc_AttributeError) == 0 {
                                PyMem_RawFree(envar as *mut ());
                                return core::ptr::null_mut::<_object>();
                            }
                        } else {
                            PyMem_RawFree(envar as *mut ());
                            let retval: *mut _object =
                                PyObject_Vectorcall(hook, args, nargs as u64, keywords__0);
                            Py_DECREF!(hook);
                            return retval;
                        }
                    }
                }
            }
            _PyErr_Clear(tstate); // If any of the imports went wrong, then warn and ignore.
            let status: i32 = PyErr_WarnFormat(
                PyExc_RuntimeWarning,
                0i64,
                c"Ignoring unimportable $PYTHONBREAKPOINT: \"%s\"".as_ptr(),
                envar,
            );
            PyMem_RawFree(envar as *mut ());
            if status < 0 {
                // Printing the warning raised an exception.
                return core::ptr::null_mut::<_object>();
            } else {
                // The warning was (probably) issued.
                return core::ptr::addr_of_mut!(_Py_NoneStruct);
            }
        }
    }
}

static breakpointhook_doc: __SlateAlign16<[i8; 104]> = __SlateAlign16([
    98i8, 114i8, 101i8, 97i8, 107i8, 112i8, 111i8, 105i8, 110i8, 116i8, 104i8, 111i8, 111i8, 107i8,
    40i8, 36i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 44i8, 32i8, 47i8, 44i8, 32i8, 42i8, 97i8,
    114i8, 103i8, 115i8, 44i8, 32i8, 42i8, 42i8, 107i8, 119i8, 97i8, 114i8, 103i8, 115i8, 41i8,
    10i8, 45i8, 45i8, 10i8, 10i8, 84i8, 104i8, 105i8, 115i8, 32i8, 104i8, 111i8, 111i8, 107i8,
    32i8, 102i8, 117i8, 110i8, 99i8, 116i8, 105i8, 111i8, 110i8, 32i8, 105i8, 115i8, 32i8, 99i8,
    97i8, 108i8, 108i8, 101i8, 100i8, 32i8, 98i8, 121i8, 32i8, 98i8, 117i8, 105i8, 108i8, 116i8,
    45i8, 105i8, 110i8, 32i8, 98i8, 114i8, 101i8, 97i8, 107i8, 112i8, 111i8, 105i8, 110i8, 116i8,
    40i8, 41i8, 46i8, 10i8, 0i8,
]);

/// Write repr(o) to sys.stdout using sys.stdout.encoding and 'backslashreplace'
/// error handler. If sys.stdout has a buffer attribute, use
/// sys.stdout.buffer.write(encoded), otherwise redecode the string and use
/// sys.stdout.write(redecoded).
///
/// Helper function for sys_displayhook().
fn sys_displayhook_unencodable(outf: *mut _object, o: *mut _object) -> i32 {
    let mut ret: i32 = unsafe { core::mem::zeroed() };
    let mut stdout_encoding_str: *const i8 = unsafe { core::mem::zeroed() };
    let mut result: *mut _object = unsafe { core::mem::zeroed() };
    let mut buffer: *mut _object = unsafe { core::mem::zeroed() };
    let mut repr_str: *mut _object = unsafe { core::mem::zeroed() };
    let mut escaped_str: *mut _object = unsafe { core::mem::zeroed() };
    let mut encoded: *mut _object = unsafe { core::mem::zeroed() };
    let mut stdout_encoding: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_0: {
            '__join_1: {
                stdout_encoding = core::ptr::null_mut::<_object>();
                stdout_encoding = PyObject_GetAttr(
                    outf,
                    core::ptr::addr_of_mut!(
                        _PyRuntime
                            .static_objects
                            .singletons
                            .strings
                            .identifiers
                            ._py_encoding
                            ._ascii
                            .ob_base
                    ),
                );
                if !stdout_encoding.is_null() {
                    stdout_encoding_str = PyUnicode_AsUTF8(stdout_encoding);
                    if !stdout_encoding_str.is_null() {
                        repr_str = PyObject_Repr(o);
                        if !repr_str.is_null() {
                            encoded = PyUnicode_AsEncodedString(
                                repr_str,
                                stdout_encoding_str,
                                c"backslashreplace".as_ptr(),
                            );
                            Py_DECREF!(repr_str);
                            if !encoded.is_null() {
                                if PyObject_GetOptionalAttr(
                                    outf,
                                    core::ptr::addr_of_mut!(
                                        _PyRuntime
                                            .static_objects
                                            .singletons
                                            .strings
                                            .identifiers
                                            ._py_buffer
                                            ._ascii
                                            .ob_base
                                    ),
                                    core::ptr::addr_of_mut!(buffer),
                                ) < 0
                                {
                                    Py_DECREF!(encoded);
                                } else {
                                    if !buffer.is_null() {
                                        result = PyObject_CallMethodOneArg(
                                            buffer,
                                            core::ptr::addr_of_mut!(
                                                _PyRuntime
                                                    .static_objects
                                                    .singletons
                                                    .strings
                                                    .identifiers
                                                    ._py_write
                                                    ._ascii
                                                    .ob_base
                                            ),
                                            encoded,
                                        );
                                        Py_DECREF!(buffer);
                                        Py_DECREF!(encoded);
                                        if result.is_null() {
                                            break '__join_1;
                                        } else {
                                            Py_DECREF!(result);
                                        }
                                    } else {
                                        escaped_str = PyUnicode_FromEncodedObject(
                                            encoded,
                                            stdout_encoding_str,
                                            c"strict".as_ptr(),
                                        );
                                        Py_DECREF!(encoded);
                                        if PyFile_WriteObject(escaped_str, outf, 1i32) != 0 {
                                            Py_DECREF!(escaped_str);
                                            break '__join_1;
                                        } else {
                                            Py_DECREF!(escaped_str);
                                        }
                                    }
                                    ret = 0;
                                    break '__join_0;
                                }
                            }
                        }
                    }
                }
            }
            ret = 1i32.wrapping_neg();
        }
        Py_XDECREF!(stdout_encoding);
        return ret;
    }
}

// [clinic input]
// sys.displayhook
//
//     object as o: object
//     /
//
// Print an object to sys.stdout and also save it in builtins._
// [clinic start generated code]
#[unsafe(link_section = ".text.slate_distinct.sysmodule.sys_displayhook")]
extern "C-unwind" fn sys_displayhook(module: *mut _object, o: *mut _object) -> *mut _object {
    let tstate: *mut _ts = _PyThreadState_GET();
    let builtins: *mut _object = unsafe {
        PyImport_GetModule(core::ptr::addr_of_mut!(
            _PyRuntime
                .static_objects
                .singletons
                .strings
                .identifiers
                ._py_builtins
                ._ascii
                .ob_base
        ))
    };
    if builtins.is_null() {
        if _PyErr_Occurred(tstate).is_null() {
            unsafe {
                _PyErr_SetString(tstate, PyExc_RuntimeError, c"lost builtins module".as_ptr())
            };
        }
        return core::ptr::null_mut::<_object>();
    }
    Py_DECREF!(builtins);
    // Print value except if None
    //
    // After printing, also assign to '_'
    //
    // Before, set '_' to None to avoid recursion
    if o == unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } {
        return unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) };
    }
    if (unsafe {
        PyObject_SetAttr(
            builtins,
            _Py_LATIN1_CHR!(95i32),
            core::ptr::addr_of_mut!(_Py_NoneStruct),
        )
    }) != 0
    {
        return core::ptr::null_mut::<_object>();
    }
    let outf: *mut _object = _PySys_GetRequiredAttr(unsafe {
        core::ptr::addr_of_mut!(
            _PyRuntime
                .static_objects
                .singletons
                .strings
                .identifiers
                ._py_stdout
                ._ascii
                .ob_base
        )
    });
    if outf.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    if outf == unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } {
        unsafe { _PyErr_SetString(tstate, PyExc_RuntimeError, c"lost sys.stdout".as_ptr()) };
        Py_DECREF!(outf);
        return core::ptr::null_mut::<_object>();
    }
    if (unsafe { PyFile_WriteObject(o, outf, 0i32) }) != 0 {
        if (unsafe { _PyErr_ExceptionMatches(tstate, PyExc_UnicodeEncodeError) }) != 0 {
            let mut err: i32 = 0;
            // repr(o) is not encodable to sys.stdout.encoding with
            // sys.stdout.errors error handler (which is probably 'strict')
            unsafe { _PyErr_Clear(tstate) };
            err = sys_displayhook_unencodable(outf, o);
            if err != 0 {
                Py_DECREF!(outf);
                return core::ptr::null_mut::<_object>();
            }
        } else {
            Py_DECREF!(outf);
            return core::ptr::null_mut::<_object>();
        }
    }
    if (unsafe { PyFile_WriteObject(_Py_LATIN1_CHR!(10i32), outf, 1i32) }) != 0 {
        Py_DECREF!(outf);
        return core::ptr::null_mut::<_object>();
    }
    Py_DECREF!(outf);
    if (unsafe { PyObject_SetAttr(builtins, _Py_LATIN1_CHR!(95i32), o) }) != 0 {
        return core::ptr::null_mut::<_object>();
    }
    unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
}

// [clinic input]
// sys.excepthook
//
//     exctype:   object
//     value:     object
//     traceback: object
//     /
//
// Handle an exception by displaying it with a traceback on sys.stderr.
// [clinic start generated code]
// [clinic end generated code: output=347477d006df92ed input=08ba730166d7ef72]
/// [clinic end generated code: output=18d99fdda21b6b5e input=ecf606fa826f19d9]
fn sys_excepthook_impl(
    module: *mut _object,
    exctype: *mut _object,
    value: *mut _object,
    traceback: *mut _object,
) -> *mut _object {
    unsafe { PyErr_Display(core::ptr::null_mut::<_object>(), value, traceback) };
    unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
}

// [clinic input]
// sys.exception
//
// Return the current exception.
//
// Return the most recent exception caught by an except clause
// in the current stack frame or in an older stack frame, or None
// if no such exception exists.
// [clinic start generated code]
/// [clinic end generated code: output=2381ee2f25953e40 input=c88fbb94b6287431]
fn sys_exception_impl(module: *mut _object) -> *mut _object {
    let err_info: *mut _err_stackitem = unsafe { _PyErr_GetTopmostException(_PyThreadState_GET()) };
    if !unsafe { (*err_info).exc_value }.is_null() {
        return Py_NewRef!(unsafe { (*err_info).exc_value });
    }
    unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
}

// [clinic input]
// sys.exc_info
//
// Return current exception information: (type, value, traceback).
//
// Return information about the most recent exception caught by an except
// clause in the current stack frame or in an older stack frame.
// [clinic start generated code]
/// [clinic end generated code: output=3afd0940cf3a4d30 input=b5c5bf077788a3e5]
fn sys_exc_info_impl(module: *mut _object) -> *mut _object {
    let err_info: *mut _err_stackitem = unsafe { _PyErr_GetTopmostException(_PyThreadState_GET()) };
    unsafe { _PyErr_StackItemToExcInfoTuple(err_info) }
}

// [clinic input]
// sys.unraisablehook
//
//     unraisable: object
//     /
//
// Handle an unraisable exception.
//
// The unraisable argument has the following attributes:
//
// * exc_type: Exception type.
// * exc_value: Exception value, can be None.
// * exc_traceback: Exception traceback, can be None.
// * err_msg: Error message, can be None.
// * object: Object causing the exception, can be None.
// [clinic start generated code]
/// [clinic end generated code: output=bb92838b32abaa14 input=ec3af148294af8d3]
#[unsafe(link_section = ".text.slate_distinct.sysmodule.sys_unraisablehook")]
extern "C-unwind" fn sys_unraisablehook(
    module: *mut _object,
    unraisable: *mut _object,
) -> *mut _object {
    unsafe { _PyErr_WriteUnraisableDefaultHook(unraisable) }
}

// [clinic input]
// sys.exit
//
//     status: object = None
//     /
//
// Exit the interpreter by raising SystemExit(status).
//
// If the status is omitted or None, it defaults to zero (i.e., success).
// If the status is an integer, it will be used as the system exit status.
// If it is another kind of object, it will be printed and the system
// exit status will be one (i.e., failure).
// [clinic start generated code]
/// [clinic end generated code: output=13870986c1ab2ec0 input=b86ca9497baa94f2]
fn sys_exit_impl(module: *mut _object, status: *mut _object) -> *mut _object {
    // Raise SystemExit so callers may catch it or clean up.
    unsafe { PyErr_SetObject(PyExc_SystemExit, status) };
    core::ptr::null_mut::<_object>()
}

fn get_utf8_unicode() -> *mut _object {
    let ret: *mut _object = unsafe {
        core::ptr::addr_of_mut!(
            _PyRuntime
                .static_objects
                .singletons
                .strings
                .literals
                ._py_utf_8
                ._ascii
                .ob_base
        )
    };
    Py_NewRef!(ret)
}

// [clinic input]
// sys.getdefaultencoding
//
// Return the current default encoding used by the Unicode implementation.
// [clinic start generated code]
/// [clinic end generated code: output=256d19dfcc0711e6 input=d416856ddbef6909]
fn sys_getdefaultencoding_impl(module: *mut _object) -> *mut _object {
    get_utf8_unicode()
}

// [clinic input]
// sys.getfilesystemencoding
//
// Return the encoding used to convert Unicode filenames to OS filenames.
// [clinic start generated code]
/// [clinic end generated code: output=1dc4bdbe9be44aa7 input=8475f8649b8c7d8c]
fn sys_getfilesystemencoding_impl(module: *mut _object) -> *mut _object {
    let interp: *mut _is = _PyInterpreterState_GET();
    let config: *const PyConfig = unsafe { _PyInterpreterState_GetConfig(interp) };
    if (unsafe {
        wcscmp(
            (*config).filesystem_encoding as *const i32,
            (core::ptr::addr_of_mut!(__slate_str4111) as *mut i32) as *const i32,
        )
    }) == 0
    {
        return get_utf8_unicode();
    }
    let mut u: *mut _object = unsafe {
        PyUnicode_FromWideChar(
            (*config).filesystem_encoding as *const i32,
            1i32.wrapping_neg() as i64,
        )
    };
    if u.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    unsafe { _PyUnicode_InternImmortal(interp, core::ptr::addr_of_mut!(u)) };
    u
}

static mut __slate_str4111: [i32; 6] = [117, 116, 102, 45, 56, 0];

// [clinic input]
// sys.getfilesystemencodeerrors
//
// Return the error mode used Unicode to OS filename conversion.
// [clinic start generated code]
/// [clinic end generated code: output=ba77b36bbf7c96f5 input=22a1e8365566f1e5]
fn sys_getfilesystemencodeerrors_impl(module: *mut _object) -> *mut _object {
    let interp: *mut _is = _PyInterpreterState_GET();
    let config: *const PyConfig = unsafe { _PyInterpreterState_GetConfig(interp) };
    let mut u: *mut _object = unsafe {
        PyUnicode_FromWideChar(
            (*config).filesystem_errors as *const i32,
            1i32.wrapping_neg() as i64,
        )
    };
    if u.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    unsafe { _PyUnicode_InternImmortal(interp, core::ptr::addr_of_mut!(u)) };
    u
}

// [clinic input]
// sys.intern
//
//     string as s: unicode
//     /
//
// ``Intern'' the given string.
//
// This enters the string in the (global) table of interned strings whose
// purpose is to speed up dictionary lookups. Return the string itself or
// the previously interned string object with the same value.
// [clinic start generated code]
/// [clinic end generated code: output=be680c24f5c9e5d6 input=849483c006924e2f]
fn sys_intern_impl(module: *mut _object, mut s: *mut _object) -> *mut _object {
    if PyUnicode_CheckExact!(s) != 0 {
        let interp: *mut _is = _PyInterpreterState_GET();
        Py_INCREF!(s);
        unsafe { _PyUnicode_InternMortal(interp, core::ptr::addr_of_mut!(s)) };
        return s;
    } else {
        unsafe {
            PyErr_Format(
                PyExc_TypeError,
                c"can't intern %.400s".as_ptr(),
                (*Py_TYPE!(s)).tp_name,
            )
        };
        return core::ptr::null_mut::<_object>();
    }
}

// [clinic input]
// sys._is_interned -> bool
//
//   string: unicode
//   /
//
// Return True if the given string is "interned".
// [clinic start generated code]
/// [clinic end generated code: output=c3678267b4e9d7ed input=039843e17883b606]
fn sys__is_interned_impl(module: *mut _object, string: *mut _object) -> i32 {
    PyUnicode_CHECK_INTERNED!(string) as i32
}

// [clinic input]
// sys._is_immortal -> bool
//
//   op: object
//   /
//
// Return True if the given object is "immortal" per PEP 683.
//
// This function should be used for specialized purposes only.
// [clinic start generated code]
/// [clinic end generated code: output=c2f5d6a80efb8d1a input=4609c9bf5481db76]
fn sys__is_immortal_impl(module: *mut _object, op: *mut _object) -> i32 {
    unsafe { PyUnstable_IsImmortal(op) }
}

/// Cached interned string objects used for calling the profile and
/// trace functions.
static mut whatstrings: __SlateAlign16<[*mut _object; 8]> = __SlateAlign16([
    unsafe {
        core::ptr::addr_of_mut!(
            _PyRuntime
                .static_objects
                .singletons
                .strings
                .identifiers
                ._py_call
                ._ascii
                .ob_base
        )
    },
    unsafe {
        core::ptr::addr_of_mut!(
            _PyRuntime
                .static_objects
                .singletons
                .strings
                .identifiers
                ._py_exception
                ._ascii
                .ob_base
        )
    },
    unsafe {
        core::ptr::addr_of_mut!(
            _PyRuntime
                .static_objects
                .singletons
                .strings
                .identifiers
                ._py_line
                ._ascii
                .ob_base
        )
    },
    unsafe {
        core::ptr::addr_of_mut!(
            _PyRuntime
                .static_objects
                .singletons
                .strings
                .identifiers
                ._py_return
                ._ascii
                .ob_base
        )
    },
    unsafe {
        core::ptr::addr_of_mut!(
            _PyRuntime
                .static_objects
                .singletons
                .strings
                .identifiers
                ._py_c_call
                ._ascii
                .ob_base
        )
    },
    unsafe {
        core::ptr::addr_of_mut!(
            _PyRuntime
                .static_objects
                .singletons
                .strings
                .identifiers
                ._py_c_exception
                ._ascii
                .ob_base
        )
    },
    unsafe {
        core::ptr::addr_of_mut!(
            _PyRuntime
                .static_objects
                .singletons
                .strings
                .identifiers
                ._py_c_return
                ._ascii
                .ob_base
        )
    },
    unsafe {
        core::ptr::addr_of_mut!(
            _PyRuntime
                .static_objects
                .singletons
                .strings
                .identifiers
                ._py_opcode
                ._ascii
                .ob_base
        )
    },
]);

fn call_trampoline(
    tstate: *mut _ts,
    callback: *mut _object,
    frame: *mut _frame,
    what: i32,
    mut arg: *mut _object,
) -> *mut _object {
    // call the Python-level function
    if arg.is_null() {
        arg = unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) };
    }
    let mut args: __SlateAlign16<[*mut _object; 3]> = __SlateAlign16([
        frame as *mut _object,
        unsafe { whatstrings.0[what as usize] },
        arg,
    ]);
    let result: *mut _object = _PyObject_VectorcallTstate(
        tstate,
        callback,
        args.0.as_mut_ptr() as *const *mut _object,
        3u64,
        core::ptr::null_mut::<_object>(),
    );
    result
}

#[unsafe(link_section = ".text.slate_distinct.sysmodule.profile_trampoline")]
extern "C-unwind" fn profile_trampoline(
    self_: *mut _object,
    frame: *mut _frame,
    what: i32,
    arg: *mut _object,
) -> i32 {
    let tstate: *mut _ts = _PyThreadState_GET();
    let result: *mut _object = call_trampoline(tstate, self_, frame, what, arg);
    if result.is_null() {
        unsafe { _PyEval_SetProfile(tstate, None, core::ptr::null_mut::<_object>()) };
        return 1i32.wrapping_neg();
    }
    Py_DECREF!(result);
    0
}

#[unsafe(link_section = ".text.slate_distinct.sysmodule.trace_trampoline")]
extern "C-unwind" fn trace_trampoline(
    self_: *mut _object,
    frame: *mut _frame,
    what: i32,
    arg: *mut _object,
) -> i32 {
    let mut callback: *mut _object = unsafe { core::mem::zeroed() };
    if what == 0 {
        callback = self_;
    } else {
        callback = unsafe { (*frame).f_trace };
    }
    if callback.is_null() {
        return 0;
    }
    let tstate: *mut _ts = _PyThreadState_GET();
    let result: *mut _object = call_trampoline(tstate, callback, frame, what, arg);
    if result.is_null() {
        unsafe { _PyEval_SetTrace(tstate, None, core::ptr::null_mut::<_object>()) };
        Py_CLEAR!((*frame).f_trace);
        return 1i32.wrapping_neg();
    }
    if result != unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } {
        Py_XSETREF!((*frame).f_trace, result);
    } else {
        Py_DECREF!(result);
    }
    0
}

// [clinic input]
// sys.settrace
//
//     function: object
//     /
//
// Set the global debug tracing function.
//
// It will be called on each function call.  See the debugger chapter
// in the library manual.
// [clinic start generated code]
/// [clinic end generated code: output=999d12e9d6ec4678 input=8107feb01c5f1c4e]
#[unsafe(link_section = ".text.slate_distinct.sysmodule.sys_settrace")]
extern "C-unwind" fn sys_settrace(module: *mut _object, function: *mut _object) -> *mut _object {
    let tstate: *mut _ts = _PyThreadState_GET();
    if function == unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } {
        if (unsafe { _PyEval_SetTrace(tstate, None, core::ptr::null_mut::<_object>()) }) < 0 {
            return core::ptr::null_mut::<_object>();
        }
    } else {
        if (unsafe { _PyEval_SetTrace(tstate, Some(trace_trampoline), function) }) < 0 {
            return core::ptr::null_mut::<_object>();
        }
    }
    unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
}

// [clinic input]
// sys._settraceallthreads
//
//     function as arg: object
//     /
//
// Set the global debug tracing function in all running threads belonging to the current interpreter.
//
// It will be called on each function call. See the debugger chapter
// in the library manual.
// [clinic start generated code]
/// [clinic end generated code: output=161cca30207bf3ca input=d4bde1f810d73675]
#[unsafe(link_section = ".text.slate_distinct.sysmodule.sys__settraceallthreads")]
extern "C-unwind" fn sys__settraceallthreads(
    module: *mut _object,
    arg: *mut _object,
) -> *mut _object {
    let mut argument: *mut _object = core::ptr::null_mut::<_object>();
    let mut func: Option<
        unsafe extern "C-unwind" fn(*mut _object, *mut _frame, i32, *mut _object) -> i32,
    > = None;
    if arg != unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } {
        func = Some(trace_trampoline);
        argument = arg;
    }
    let interp: *mut _is = _PyInterpreterState_GET();
    if (unsafe { _PyEval_SetTraceAllThreads(interp, func, argument) }) < 0 {
        return core::ptr::null_mut::<_object>();
    }
    unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
}

// [clinic input]
// sys.gettrace
//
// Return the global debug tracing function set with sys.settrace.
//
// See the debugger chapter in the library manual.
// [clinic start generated code]
/// [clinic end generated code: output=e97e3a4d8c971b6e input=373b51bb2147f4d8]
fn sys_gettrace_impl(module: *mut _object) -> *mut _object {
    let tstate: *mut _ts = _PyThreadState_GET();
    let mut temp: *mut _object = unsafe { (*tstate).c_traceobj };
    if temp.is_null() {
        temp = unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) };
    }
    Py_NewRef!(temp)
}

// [clinic input]
// sys.setprofile
//
//     function: object
//     /
//
// Set the profiling function.
//
// It will be called on each function call and return.  See the profiler
// chapter in the library manual.
// [clinic start generated code]
/// [clinic end generated code: output=1c3503105939db9c input=055d0d7961413a62]
#[unsafe(link_section = ".text.slate_distinct.sysmodule.sys_setprofile")]
extern "C-unwind" fn sys_setprofile(module: *mut _object, function: *mut _object) -> *mut _object {
    let tstate: *mut _ts = _PyThreadState_GET();
    if function == unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } {
        if (unsafe { _PyEval_SetProfile(tstate, None, core::ptr::null_mut::<_object>()) }) < 0 {
            return core::ptr::null_mut::<_object>();
        }
    } else {
        if (unsafe { _PyEval_SetProfile(tstate, Some(profile_trampoline), function) }) < 0 {
            return core::ptr::null_mut::<_object>();
        }
    }
    unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
}

// [clinic input]
// sys._setprofileallthreads
//
//     function as arg: object
//     /
//
// Set the profiling function in all running threads belonging to the current interpreter.
//
// It will be called on each function call and return.  See the profiler
// chapter in the library manual.
// [clinic start generated code]
/// [clinic end generated code: output=2d61319e27b309fe input=a10589439ba20cee]
#[unsafe(link_section = ".text.slate_distinct.sysmodule.sys__setprofileallthreads")]
extern "C-unwind" fn sys__setprofileallthreads(
    module: *mut _object,
    arg: *mut _object,
) -> *mut _object {
    let mut argument: *mut _object = core::ptr::null_mut::<_object>();
    let mut func: Option<
        unsafe extern "C-unwind" fn(*mut _object, *mut _frame, i32, *mut _object) -> i32,
    > = None;
    if arg != unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } {
        func = Some(profile_trampoline);
        argument = arg;
    }
    let interp: *mut _is = _PyInterpreterState_GET();
    if (unsafe { _PyEval_SetProfileAllThreads(interp, func, argument) }) < 0 {
        return core::ptr::null_mut::<_object>();
    }
    unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
}

// [clinic input]
// sys.getprofile
//
// Return the profiling function set with sys.setprofile.
//
// See the profiler chapter in the library manual.
// [clinic start generated code]
/// [clinic end generated code: output=579b96b373448188 input=1b3209d89a32965d]
fn sys_getprofile_impl(module: *mut _object) -> *mut _object {
    let tstate: *mut _ts = _PyThreadState_GET();
    let mut temp: *mut _object = unsafe { (*tstate).c_profileobj };
    if temp.is_null() {
        temp = unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) };
    }
    Py_NewRef!(temp)
}

// [clinic input]
// sys.setswitchinterval
//
//     interval: double
//     /
//
// Set the ideal thread switching delay inside the Python interpreter.
//
// The actual frequency of switching threads can be lower if the
// interpreter executes long sequences of uninterruptible code
// (this is implementation-specific and workload-dependent).
//
// The parameter must represent the desired switching delay in seconds
// A typical value is 0.005 (5 milliseconds).
// [clinic start generated code]
/// [clinic end generated code: output=65a19629e5153983 input=561b477134df91d9]
fn sys_setswitchinterval_impl(module: *mut _object, interval: f64) -> *mut _object {
    if interval <= 0.0f64 {
        unsafe {
            PyErr_SetString(
                PyExc_ValueError,
                c"switch interval must be strictly positive".as_ptr(),
            )
        };
        return core::ptr::null_mut::<_object>();
    }
    unsafe { _PyEval_SetSwitchInterval((1000000.0f64 * interval) as u64) };
    unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
}

// [clinic input]
// sys.getswitchinterval -> double
//
// Return the current thread switch interval; see sys.setswitchinterval().
// [clinic start generated code]
/// [clinic end generated code: output=a38c277c85b5096d input=bdf9d39c0ebbbb6f]
fn sys_getswitchinterval_impl(module: *mut _object) -> f64 {
    1e-6f64 * ((unsafe { _PyEval_GetSwitchInterval() }) as f64)
}

// [clinic input]
// sys.setrecursionlimit
//
//     limit as new_limit: int
//     /
//
// Set the maximum depth of the Python interpreter stack to n.
//
// This limit prevents infinite recursion from causing an overflow of the C
// stack and crashing Python.  The highest possible limit is platform-
// dependent.
// [clinic start generated code]
/// [clinic end generated code: output=35e1c64754800ace input=b0f7a23393924af3]
fn sys_setrecursionlimit_impl(module: *mut _object, new_limit: i32) -> *mut _object {
    let tstate: *mut _ts = _PyThreadState_GET();
    if new_limit < 1 {
        unsafe {
            _PyErr_SetString(
                tstate,
                PyExc_ValueError,
                c"recursion limit must be greater or equal than 1".as_ptr(),
            )
        };
        return core::ptr::null_mut::<_object>();
    }
    // Reject too low new limit if the current recursion depth is higher than
    // the new low-water mark.
    let depth: i32 = unsafe { (*tstate).py_recursion_limit }
        .wrapping_sub(unsafe { (*tstate).py_recursion_remaining });
    if depth >= new_limit {
        unsafe {
            _PyErr_Format(tstate, PyExc_RecursionError, c"cannot set the recursion limit to %i at the recursion depth %i: the limit is too low".as_ptr(), new_limit, depth)
        };
        return core::ptr::null_mut::<_object>();
    }
    unsafe { Py_SetRecursionLimit(new_limit) };
    unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
}

// [clinic input]
// sys.set_coroutine_origin_tracking_depth
//
//   depth: int
//
// Enable or disable origin tracking for coroutine objects in this thread.
//
// Coroutine objects will track 'depth' frames of traceback information
// about where they came from, available in their cr_origin attribute.
//
// Set a depth of 0 to disable.
// [clinic start generated code]
/// [clinic end generated code: output=0a2123c1cc6759c5 input=a1d0a05f89d2c426]
fn sys_set_coroutine_origin_tracking_depth_impl(module: *mut _object, depth: i32) -> *mut _object {
    if (unsafe { _PyEval_SetCoroutineOriginTrackingDepth(depth) }) < 0 {
        return core::ptr::null_mut::<_object>();
    }
    unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
}

// [clinic input]
// sys.get_coroutine_origin_tracking_depth -> int
//
// Check status of origin tracking for coroutine objects in this thread.
// [clinic start generated code]
/// [clinic end generated code: output=3699f7be95a3afb8 input=335266a71205b61a]
fn sys_get_coroutine_origin_tracking_depth_impl(module: *mut _object) -> i32 {
    unsafe { _PyEval_GetCoroutineOriginTrackingDepth() }
}

static mut AsyncGenHooksType: _typeobject = unsafe { core::mem::zeroed() };

static asyncgen_hooks_doc: __SlateAlign16<[i8; 120]> = __SlateAlign16([
    97i8, 115i8, 121i8, 110i8, 99i8, 103i8, 101i8, 110i8, 95i8, 104i8, 111i8, 111i8, 107i8, 115i8,
    10i8, 10i8, 65i8, 32i8, 110i8, 97i8, 109i8, 101i8, 100i8, 32i8, 116i8, 117i8, 112i8, 108i8,
    101i8, 32i8, 112i8, 114i8, 111i8, 118i8, 105i8, 100i8, 105i8, 110i8, 103i8, 32i8, 105i8, 110i8,
    102i8, 111i8, 114i8, 109i8, 97i8, 116i8, 105i8, 111i8, 110i8, 32i8, 97i8, 98i8, 111i8, 117i8,
    116i8, 32i8, 97i8, 115i8, 121i8, 110i8, 99i8, 104i8, 114i8, 111i8, 110i8, 111i8, 117i8, 115i8,
    10i8, 103i8, 101i8, 110i8, 101i8, 114i8, 97i8, 116i8, 111i8, 114i8, 115i8, 32i8, 104i8, 111i8,
    111i8, 107i8, 115i8, 46i8, 32i8, 32i8, 84i8, 104i8, 101i8, 32i8, 97i8, 116i8, 116i8, 114i8,
    105i8, 98i8, 117i8, 116i8, 101i8, 115i8, 32i8, 97i8, 114i8, 101i8, 32i8, 114i8, 101i8, 97i8,
    100i8, 32i8, 111i8, 110i8, 108i8, 121i8, 46i8, 0i8,
]);

static mut asyncgen_hooks_fields: __SlateAlign16<[PyStructSequence_Field; 3]> = __SlateAlign16([
    PyStructSequence_Field {
        name: (b"firstiter\0".as_ptr() as *mut i8) as *const i8,
        doc: (b"Hook to intercept first iteration\0".as_ptr() as *mut i8) as *const i8,
    },
    PyStructSequence_Field {
        name: (b"finalizer\0".as_ptr() as *mut i8) as *const i8,
        doc: (b"Hook to intercept finalization\0".as_ptr() as *mut i8) as *const i8,
    },
    {
        let mut __t0: PyStructSequence_Field = unsafe { core::mem::zeroed() };
        __t0.name = core::ptr::null::<i8>();
        __t0
    },
]);

/// name
/// doc
/// fields
static mut asyncgen_hooks_desc: PyStructSequence_Desc = {
    let mut __t0: PyStructSequence_Desc = unsafe { core::mem::zeroed() };
    __t0.name = (b"asyncgen_hooks\0".as_ptr() as *mut i8) as *const i8;
    __t0.doc = unsafe { core::ptr::addr_of!(asyncgen_hooks_doc.0) as *const i8 };
    __t0.fields =
        unsafe { core::ptr::addr_of_mut!(asyncgen_hooks_fields.0) as *mut PyStructSequence_Field };
    __t0.n_in_sequence = 2i32;
    __t0
};

#[unsafe(link_section = ".text.slate_distinct.sysmodule.sys_set_asyncgen_hooks")]
extern "C-unwind" fn sys_set_asyncgen_hooks(
    self_: *mut _object,
    args: *mut _object,
    kw: *mut _object,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut __v1: bool = unsafe { core::mem::zeroed() };
    let mut cur_finalizer: *mut _object = unsafe { core::mem::zeroed() };
    let mut finalizer: *mut _object = unsafe { core::mem::zeroed() };
    let mut firstiter: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        firstiter = core::ptr::null_mut::<_object>();
        finalizer = core::ptr::null_mut::<_object>();
        if PyArg_ParseTupleAndKeywords(
            args,
            kw,
            c"|OO".as_ptr(),
            (core::ptr::addr_of_mut!(keywords.0) as *mut *mut i8) as *const *mut i8,
            core::ptr::addr_of_mut!(firstiter),
            core::ptr::addr_of_mut!(finalizer),
        ) == 0
        {
            return core::ptr::null_mut::<_object>();
        } else {
            if !finalizer.is_null() && finalizer != core::ptr::addr_of_mut!(_Py_NoneStruct) {
                if PyCallable_Check(finalizer) == 0 {
                    PyErr_Format(
                        PyExc_TypeError,
                        c"callable finalizer expected, got %.50s".as_ptr(),
                        (*Py_TYPE!(finalizer)).tp_name,
                    );
                    return core::ptr::null_mut::<_object>();
                }
            }
            if !firstiter.is_null() && firstiter != core::ptr::addr_of_mut!(_Py_NoneStruct) {
                if PyCallable_Check(firstiter) == 0 {
                    PyErr_Format(
                        PyExc_TypeError,
                        c"callable firstiter expected, got %.50s".as_ptr(),
                        (*Py_TYPE!(firstiter)).tp_name,
                    );
                    return core::ptr::null_mut::<_object>();
                }
            }
            cur_finalizer = _PyEval_GetAsyncGenFinalizer();
            if !finalizer.is_null() && finalizer != core::ptr::addr_of_mut!(_Py_NoneStruct) {
                if _PyEval_SetAsyncGenFinalizer(finalizer) < 0 {
                    return core::ptr::null_mut::<_object>();
                }
            } else {
                if finalizer == core::ptr::addr_of_mut!(_Py_NoneStruct) {
                    __v1 = _PyEval_SetAsyncGenFinalizer(core::ptr::null_mut::<_object>()) < 0;
                } else {
                    __v1 = false;
                }
                if __v1 {
                    return core::ptr::null_mut::<_object>();
                }
            }
            '__join_0: {
                if !firstiter.is_null() && firstiter != core::ptr::addr_of_mut!(_Py_NoneStruct) {
                    if _PyEval_SetAsyncGenFirstiter(firstiter) < 0 {
                        break '__join_0;
                    }
                } else {
                    if firstiter == core::ptr::addr_of_mut!(_Py_NoneStruct) {
                        __v0 = _PyEval_SetAsyncGenFirstiter(core::ptr::null_mut::<_object>()) < 0;
                    } else {
                        __v0 = false;
                    }
                    if __v0 {
                        break '__join_0;
                    }
                }
                return core::ptr::addr_of_mut!(_Py_NoneStruct);
            }
            _PyEval_SetAsyncGenFinalizer(cur_finalizer);
            return core::ptr::null_mut::<_object>();
        }
    }
}

static mut keywords: __SlateAlign16<[*mut i8; 3]> = __SlateAlign16([
    b"firstiter\0".as_ptr() as *mut i8,
    b"finalizer\0".as_ptr() as *mut i8,
    core::ptr::null_mut::<i8>(),
]);

static set_asyncgen_hooks_doc: __SlateAlign16<[i8; 93]> = __SlateAlign16([
    115i8, 101i8, 116i8, 95i8, 97i8, 115i8, 121i8, 110i8, 99i8, 103i8, 101i8, 110i8, 95i8, 104i8,
    111i8, 111i8, 107i8, 115i8, 40i8, 91i8, 102i8, 105i8, 114i8, 115i8, 116i8, 105i8, 116i8, 101i8,
    114i8, 93i8, 32i8, 91i8, 44i8, 32i8, 102i8, 105i8, 110i8, 97i8, 108i8, 105i8, 122i8, 101i8,
    114i8, 93i8, 41i8, 10i8, 10i8, 83i8, 101i8, 116i8, 32i8, 97i8, 32i8, 102i8, 105i8, 110i8, 97i8,
    108i8, 105i8, 122i8, 101i8, 114i8, 32i8, 102i8, 111i8, 114i8, 32i8, 97i8, 115i8, 121i8, 110i8,
    99i8, 32i8, 103i8, 101i8, 110i8, 101i8, 114i8, 97i8, 116i8, 111i8, 114i8, 115i8, 32i8, 111i8,
    98i8, 106i8, 101i8, 99i8, 116i8, 115i8, 46i8, 0i8,
]);

// [clinic input]
// sys.get_asyncgen_hooks
//
// Return the installed asynchronous generators hooks.
//
// This returns a namedtuple of the form (firstiter, finalizer).
// [clinic start generated code]
/// [clinic end generated code: output=53a253707146f6cf input=3676b9ea62b14625]
fn sys_get_asyncgen_hooks_impl(module: *mut _object) -> *mut _object {
    let mut firstiter: *mut _object = unsafe { _PyEval_GetAsyncGenFirstiter() };
    let mut finalizer: *mut _object = unsafe { _PyEval_GetAsyncGenFinalizer() };
    let res: *mut _object =
        unsafe { PyStructSequence_New(core::ptr::addr_of_mut!(AsyncGenHooksType)) };
    if res.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    if firstiter.is_null() {
        firstiter = unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) };
    }
    if finalizer.is_null() {
        finalizer = unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) };
    }
    unsafe { PyStructSequence_SetItem(res, 0i64, Py_NewRef!(firstiter)) };
    unsafe { PyStructSequence_SetItem(res, 1i64, Py_NewRef!(finalizer)) };
    res
}

static mut Hash_InfoType: _typeobject = unsafe { core::mem::zeroed() };

static hash_info_doc: __SlateAlign16<[i8; 103]> = __SlateAlign16([
    104i8, 97i8, 115i8, 104i8, 95i8, 105i8, 110i8, 102i8, 111i8, 10i8, 10i8, 65i8, 32i8, 110i8,
    97i8, 109i8, 101i8, 100i8, 32i8, 116i8, 117i8, 112i8, 108i8, 101i8, 32i8, 112i8, 114i8, 111i8,
    118i8, 105i8, 100i8, 105i8, 110i8, 103i8, 32i8, 112i8, 97i8, 114i8, 97i8, 109i8, 101i8, 116i8,
    101i8, 114i8, 115i8, 32i8, 117i8, 115i8, 101i8, 100i8, 32i8, 102i8, 111i8, 114i8, 32i8, 99i8,
    111i8, 109i8, 112i8, 117i8, 116i8, 105i8, 110i8, 103i8, 10i8, 104i8, 97i8, 115i8, 104i8, 101i8,
    115i8, 46i8, 32i8, 84i8, 104i8, 101i8, 32i8, 97i8, 116i8, 116i8, 114i8, 105i8, 98i8, 117i8,
    116i8, 101i8, 115i8, 32i8, 97i8, 114i8, 101i8, 32i8, 114i8, 101i8, 97i8, 100i8, 32i8, 111i8,
    110i8, 108i8, 121i8, 46i8, 0i8,
]);

static mut hash_info_fields: __SlateAlign16<[PyStructSequence_Field; 10]> = __SlateAlign16([
    PyStructSequence_Field {
        name: (b"width\0".as_ptr() as *mut i8) as *const i8,
        doc: (b"width of the type used for hashing, in bits\0".as_ptr() as *mut i8) as *const i8,
    },
    PyStructSequence_Field {
        name: (b"modulus\0".as_ptr() as *mut i8) as *const i8,
        doc: (b"prime number giving the modulus on which the hash function is based\0".as_ptr()
            as *mut i8) as *const i8,
    },
    PyStructSequence_Field {
        name: (b"inf\0".as_ptr() as *mut i8) as *const i8,
        doc: (b"value to be used for hash of a positive infinity\0".as_ptr() as *mut i8)
            as *const i8,
    },
    PyStructSequence_Field {
        name: (b"nan\0".as_ptr() as *mut i8) as *const i8,
        doc: (b"value to be used for hash of a nan\0".as_ptr() as *mut i8) as *const i8,
    },
    PyStructSequence_Field {
        name: (b"imag\0".as_ptr() as *mut i8) as *const i8,
        doc: (b"multiplier used for the imaginary part of a complex number\0".as_ptr() as *mut i8)
            as *const i8,
    },
    PyStructSequence_Field {
        name: (b"algorithm\0".as_ptr() as *mut i8) as *const i8,
        doc: (b"name of the algorithm for hashing of str, bytes and memoryviews\0".as_ptr()
            as *mut i8) as *const i8,
    },
    PyStructSequence_Field {
        name: (b"hash_bits\0".as_ptr() as *mut i8) as *const i8,
        doc: (b"internal output size of hash algorithm\0".as_ptr() as *mut i8) as *const i8,
    },
    PyStructSequence_Field {
        name: (b"seed_bits\0".as_ptr() as *mut i8) as *const i8,
        doc: (b"seed size of hash algorithm\0".as_ptr() as *mut i8) as *const i8,
    },
    PyStructSequence_Field {
        name: (b"cutoff\0".as_ptr() as *mut i8) as *const i8,
        doc: (b"small string optimization cutoff\0".as_ptr() as *mut i8) as *const i8,
    },
    PyStructSequence_Field {
        name: core::ptr::null::<i8>(),
        doc: core::ptr::null::<i8>(),
    },
]);

static mut hash_info_desc: PyStructSequence_Desc = {
    let mut __t0: PyStructSequence_Desc = unsafe { core::mem::zeroed() };
    __t0.name = (b"sys.hash_info\0".as_ptr() as *mut i8) as *const i8;
    __t0.doc = unsafe { core::ptr::addr_of!(hash_info_doc.0) as *const i8 };
    __t0.fields =
        unsafe { core::ptr::addr_of_mut!(hash_info_fields.0) as *mut PyStructSequence_Field };
    __t0.n_in_sequence = 9i32;
    __t0
};

fn get_hash_info(tstate: *mut _ts) -> *mut _object {
    let mut hash_info: *mut _object = unsafe { core::mem::zeroed() };
    let mut field: i32 = 0;
    hash_info = unsafe { PyStructSequence_New(core::ptr::addr_of_mut!(Hash_InfoType)) };
    if hash_info.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let hashfunc: *mut __SlateRecord_pyhash_h_39 = unsafe { PyHash_GetFuncDef() };
    SET_HASH_INFO_ITEM!(
        unsafe { PyLong_FromLong(8u64.wrapping_mul(core::mem::size_of::<i64>() as u64) as i64) },
        hash_info,
        field
    );
    SET_HASH_INFO_ITEM!(
        unsafe { PyLong_FromSsize_t(2305843009213693951i64) },
        hash_info,
        field
    );
    SET_HASH_INFO_ITEM!(unsafe { PyLong_FromLong(314159i64) }, hash_info, field);
    SET_HASH_INFO_ITEM!(unsafe { PyLong_FromLong(0i64) }, hash_info, field); // This is no longer used
    SET_HASH_INFO_ITEM!(unsafe { PyLong_FromLong(1000003i64) }, hash_info, field);
    SET_HASH_INFO_ITEM!(
        unsafe { PyUnicode_FromString((*hashfunc).name) },
        hash_info,
        field
    );
    SET_HASH_INFO_ITEM!(
        unsafe { PyLong_FromLong((*hashfunc).hash_bits as i64) },
        hash_info,
        field
    );
    SET_HASH_INFO_ITEM!(
        unsafe { PyLong_FromLong((*hashfunc).seed_bits as i64) },
        hash_info,
        field
    );
    SET_HASH_INFO_ITEM!(unsafe { PyLong_FromLong(0i64) }, hash_info, field);
    hash_info
}

// [clinic input]
// sys.getrecursionlimit
//
// Return the current value of the recursion limit.
//
// The recursion limit is the maximum depth of the Python interpreter
// stack.  This limit prevents infinite recursion from causing an overflow
// of the C stack and crashing Python.
// [clinic start generated code]
/// [clinic end generated code: output=d571fb6b4549ef2e input=1c6129fd2efaeea8]
fn sys_getrecursionlimit_impl(module: *mut _object) -> *mut _object {
    unsafe { PyLong_FromLong(Py_GetRecursionLimit() as i64) }
}

// [clinic input]
// sys.setdlopenflags
//
//     flags as new_val: int
//     /
//
// Set the flags used by the interpreter for dlopen calls.
//
// This is used, for example, when the interpreter loads extension
// modules. Among other things, this will enable a lazy resolving of
// symbols when importing a module, if called as sys.setdlopenflags(0).
// To share symbols across extension modules, call as
// sys.setdlopenflags(os.RTLD_GLOBAL).  Symbolic names for the flag
// modules can be found in the os module (RTLD_xxx constants, e.g.
// os.RTLD_LAZY).
// [clinic start generated code]
/// [clinic end generated code: output=ec918b7fe0a37281 input=4c838211e857a77f]
fn sys_setdlopenflags_impl(module: *mut _object, new_val: i32) -> *mut _object {
    let interp: *mut _is = _PyInterpreterState_GET();
    unsafe { _PyImport_SetDLOpenFlags(interp, new_val) };
    unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
}

// [clinic input]
// sys.getdlopenflags
//
// Return the current value of the flags that are used for dlopen calls.
//
// The flag constants are defined in the os module.
// [clinic start generated code]
/// [clinic end generated code: output=e92cd1bc5005da6e input=dc4ea0899c53b4b6]
fn sys_getdlopenflags_impl(module: *mut _object) -> *mut _object {
    let interp: *mut _is = _PyInterpreterState_GET();
    unsafe { PyLong_FromLong(_PyImport_GetDLOpenFlags(interp) as i64) }
}

// [clinic input]
// sys.get_int_max_str_digits
//
// Return the maximum string digits limit for non-binary int<->str conversions.
// [clinic start generated code]
/// [clinic end generated code: output=0042f5e8ae0e8631 input=61bf9f99bc8b112d]
fn sys_get_int_max_str_digits_impl(module: *mut _object) -> *mut _object {
    let interp: *mut _is = _PyInterpreterState_GET();
    let maxdigits: i32 = _Py_atomic_load_int(
        (unsafe { core::ptr::addr_of_mut!((*interp).long_state.max_str_digits) }) as *const i32,
    );
    unsafe { PyLong_FromLong(maxdigits as i64) }
}

// [clinic input]
// sys.set_int_max_str_digits
//
//     maxdigits: int
//
// Set the maximum string digits limit for non-binary int<->str conversions.
// [clinic start generated code]
/// [clinic end generated code: output=734d4c2511f2a56d input=d7e3f325db6910c5]
fn sys_set_int_max_str_digits_impl(module: *mut _object, maxdigits: i32) -> *mut _object {
    if _PySys_SetIntMaxStrDigits(maxdigits) < 0 {
        return core::ptr::null_mut::<_object>();
    }
    unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PySys_GetSizeOf(o: *mut _object) -> u64 {
    let mut res: *mut _object = core::ptr::null_mut::<_object>();
    let mut size: i64 = 0;
    let tstate: *mut _ts = _PyThreadState_GET();
    // Make sure the type is initialized. float gets initialized late
    if (unsafe { PyType_Ready(Py_TYPE!(o)) }) < 0 {
        return 1i32.wrapping_neg() as u64;
    }
    let method: *mut _object = unsafe {
        _PyObject_LookupSpecial(
            o,
            core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py___sizeof__
                    ._ascii
                    .ob_base
            ),
        )
    };
    if method.is_null() {
        if _PyErr_Occurred(tstate).is_null() {
            unsafe {
                _PyErr_Format(
                    tstate,
                    PyExc_TypeError,
                    c"Type %.100s doesn't define __sizeof__".as_ptr(),
                    (*Py_TYPE!(o)).tp_name,
                )
            };
        }
    } else {
        res = _PyObject_CallNoArgs(method);
        Py_DECREF!(method);
    }
    if res.is_null() {
        return 1i32.wrapping_neg() as u64;
    }
    size = unsafe { PyLong_AsSsize_t(res) };
    Py_DECREF!(res);
    let __v0: bool = size == (1i32.wrapping_neg() as i64) && !_PyErr_Occurred(tstate).is_null();
    if __v0 {
        return 1i32.wrapping_neg() as u64;
    }
    if size < 0 {
        unsafe {
            _PyErr_SetString(
                tstate,
                PyExc_ValueError,
                c"__sizeof__() should return >= 0".as_ptr(),
            )
        };
        return 1i32.wrapping_neg() as u64;
    }
    let mut presize: u64 = 0;
    let __v1: bool = Py_IS_TYPE!(o, unsafe { core::ptr::addr_of_mut!(PyType_Type) }) == 0
        || PyType_HasFeature(o as *mut _typeobject, 512u64) != 0;
    if __v1 {
        // Add the size of the pre-header if "o" is not a static type
        presize = _PyType_PreHeaderSize(Py_TYPE!(o));
    }
    (size as u64).wrapping_add(presize)
}

#[unsafe(link_section = ".text.slate_distinct.sysmodule.sys_getsizeof")]
extern "C-unwind" fn sys_getsizeof(
    self_: *mut _object,
    args: *mut _object,
    kwds: *mut _object,
) -> *mut _object {
    let mut size: u64 = 0;
    let mut o: *mut _object = unsafe { core::mem::zeroed() };
    let mut dflt: *mut _object = core::ptr::null_mut::<_object>();
    let tstate: *mut _ts = _PyThreadState_GET();
    if (unsafe {
        PyArg_ParseTupleAndKeywords(
            args,
            kwds,
            c"O|O:getsizeof".as_ptr(),
            (core::ptr::addr_of_mut!(kwlist.0) as *mut *mut i8) as *const *mut i8,
            core::ptr::addr_of_mut!(o),
            core::ptr::addr_of_mut!(dflt),
        )
    }) == 0
    {
        return core::ptr::null_mut::<_object>();
    }
    size = _PySys_GetSizeOf(o);
    let __v0: bool = size == (1i32.wrapping_neg() as u64) && !_PyErr_Occurred(tstate).is_null();
    if __v0 {
        // Has a default value been given
        let __v1: bool =
            !dflt.is_null() && (unsafe { _PyErr_ExceptionMatches(tstate, PyExc_TypeError) }) != 0;
        if __v1 {
            unsafe { _PyErr_Clear(tstate) };
            return Py_NewRef!(dflt);
        } else {
            return core::ptr::null_mut::<_object>();
        }
    }
    unsafe { PyLong_FromSize_t(size) }
}

static mut kwlist: __SlateAlign16<[*mut i8; 3]> = __SlateAlign16([
    b"object\0".as_ptr() as *mut i8,
    b"default\0".as_ptr() as *mut i8,
    core::ptr::null_mut::<i8>(),
]);

static getsizeof_doc: __SlateAlign16<[i8; 74]> = __SlateAlign16([
    103i8, 101i8, 116i8, 115i8, 105i8, 122i8, 101i8, 111i8, 102i8, 40i8, 111i8, 98i8, 106i8, 101i8,
    99i8, 116i8, 32i8, 91i8, 44i8, 32i8, 100i8, 101i8, 102i8, 97i8, 117i8, 108i8, 116i8, 93i8,
    41i8, 32i8, 45i8, 62i8, 32i8, 105i8, 110i8, 116i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8,
    114i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 115i8, 105i8, 122i8, 101i8, 32i8, 111i8, 102i8,
    32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 32i8, 105i8, 110i8, 32i8, 98i8, 121i8, 116i8,
    101i8, 115i8, 46i8, 0i8,
]);

// [clinic input]
// sys.getrefcount -> Py_ssize_t
//
//     object:  object
//     /
//
// Return the reference count of object.
//
// The count returned is generally one higher than you might expect,
// because it includes the (temporary) reference as an argument to
// getrefcount().
// [clinic start generated code]
/// [clinic end generated code: output=5fd477f2264b85b2 input=bf474efd50a21535]
fn sys_getrefcount_impl(module: *mut _object, object: *mut _object) -> i64 {
    Py_REFCNT!(object)
}

// [clinic input]
// sys.getallocatedblocks -> Py_ssize_t
//
// Return the number of memory blocks currently allocated.
// [clinic start generated code]
/// [clinic end generated code: output=f0c4e873f0b6dcf7 input=dab13ee346a0673e]
fn sys_getallocatedblocks_impl(module: *mut _object) -> i64 {
    // It might make sense to return the count
    //
    // for just the current interpreter.
    unsafe { _Py_GetGlobalAllocatedBlocks() }
}

// [clinic input]
// sys.getunicodeinternedsize -> Py_ssize_t
//
//     *
//     _only_immortal: bool = False
//
// Return the number of elements of the unicode interned dictionary
// [clinic start generated code]
/// [clinic end generated code: output=29a6377a94a14f70 input=0330b3408dd5bcc6]
fn sys_getunicodeinternedsize_impl(module: *mut _object, _only_immortal: i32) -> i64 {
    if _only_immortal != 0 {
        return unsafe { _PyUnicode_InternedSize_Immortal() };
    } else {
        return unsafe { _PyUnicode_InternedSize() };
    }
}

// [clinic input]
// sys._getframe
//
//     depth: int = 0
//     /
//
// Return a frame object from the call stack.
//
// If optional integer depth is given, return the frame object that many
// calls below the top of the stack.  If that is deeper than the call
// stack, ValueError is raised.  The default for depth is zero, returning
// the frame at the top of the call stack.
//
// This function should be used for internal and specialized purposes
// only.
// [clinic start generated code]
/// [clinic end generated code: output=d438776c04d59804 input=c1be8a6464b11ee5]
fn sys__getframe_impl(module: *mut _object, mut depth: i32) -> *mut _object {
    let tstate: *mut _ts = _PyThreadState_GET();
    let mut frame: *mut _PyInterpreterFrame = unsafe { (*tstate).current_frame };
    if !frame.is_null() {
        while depth > 0 {
            frame = _PyFrame_GetFirstComplete(unsafe { (*frame).previous });
            if frame.is_null() {
                break;
            }
            depth = depth.wrapping_sub(1);
        }
    }
    if frame.is_null() {
        unsafe {
            _PyErr_SetString(
                tstate,
                PyExc_ValueError,
                c"call stack is not deep enough".as_ptr(),
            )
        };
        return core::ptr::null_mut::<_object>();
    }
    let pyFrame: *mut _object = Py_XNewRef!(_PyFrame_GetFrameObject(frame) as *mut _object);
    let __v3: bool = !pyFrame.is_null()
        && (unsafe { _PySys_Audit(tstate, c"sys._getframe".as_ptr(), c"(O)".as_ptr(), pyFrame) })
            < 0;
    if __v3 {
        Py_DECREF!(pyFrame);
        return core::ptr::null_mut::<_object>();
    }
    pyFrame
}

// [clinic input]
// sys._current_frames
//
// Return a dict mapping each thread's thread id to its current stack frame.
//
// This function should be used for specialized purposes only.
// [clinic start generated code]
/// [clinic end generated code: output=d2a41ac0a0a3809a input=2a9049c5f5033691]
fn sys__current_frames_impl(module: *mut _object) -> *mut _object {
    unsafe { _PyThread_CurrentFrames() }
}

// [clinic input]
// sys._current_exceptions
//
// Return a dict mapping each thread's identifier to its current raised exception.
//
// This function should be used for specialized purposes only.
// [clinic start generated code]
/// [clinic end generated code: output=2ccfd838c746f0ba input=0e91818fbf2edc1f]
fn sys__current_exceptions_impl(module: *mut _object) -> *mut _object {
    unsafe { _PyThread_CurrentExceptions() }
}

// [clinic input]
// sys.call_tracing
//
//     func: object
//     args as funcargs: object(subclass_of='&PyTuple_Type')
//     /
//
// Call func(*args), while tracing is enabled.
//
// The tracing state is saved, and restored afterwards.  This is intended
// to be called from a debugger from a checkpoint, to recursively debug
// some other code.
// [clinic start generated code]
/// [clinic end generated code: output=7e4999853cd4e5a6 input=5102e8b11049f92f]
fn sys_call_tracing_impl(
    module: *mut _object,
    func: *mut _object,
    funcargs: *mut _object,
) -> *mut _object {
    unsafe { _PyEval_CallTracing(func, funcargs) }
}

// [clinic input]
// sys._debugmallocstats
//
// Print summary info to stderr about the state of pymalloc's structures.
//
// In Py_DEBUG mode, also perform some expensive internal consistency
// checks.
// [clinic start generated code]
/// [clinic end generated code: output=ec3565f8c7cee46a input=33c0c9c416f98424]
fn sys__debugmallocstats_impl(module: *mut _object) -> *mut _object {
    if (unsafe { _PyObject_DebugMallocStats(slate_support::stderr()) }) != 0 {
        let _ = std::io::Write::flush(&mut std::io::stdout());
        unsafe { slate_support::fputc(10i32, slate_support::stderr()) };
        unsafe { fflush(core::ptr::null_mut()) };
    }
    unsafe { _PyObject_DebugTypeStats(slate_support::stderr()) };
    unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
}

// [clinic input]
// sys._clear_type_cache
//
// Clear the internal type lookup cache.
// [clinic start generated code]
/// [clinic end generated code: output=20e48ca54a6f6971 input=127f3e04a8d9b555]
fn sys__clear_type_cache_impl(module: *mut _object) -> *mut _object {
    if (unsafe {
        PyErr_WarnEx(PyExc_DeprecationWarning, c"sys._clear_type_cache() is deprecated and scheduled for removal in a future version. Use sys._clear_internal_caches() instead.".as_ptr(), 1i64)
    }) < 0
    {
        return core::ptr::null_mut::<_object>();
    }
    unsafe { PyType_ClearCache() };
    unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
}

// [clinic input]
// sys._clear_internal_caches
//
// Clear all internal performance-related caches.
// [clinic start generated code]
/// [clinic end generated code: output=0ee128670a4966d6 input=253e741ca744f6e8]
fn sys__clear_internal_caches_impl(module: *mut _object) -> *mut _object {
    unsafe { PyType_ClearCache() };
    unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
}

// Note that, for now, we do not have a per-interpreter equivalent
// for sys.is_finalizing().
// [clinic input]
// sys.is_finalizing
//
// Return True if Python is exiting.
// [clinic start generated code]
/// [clinic end generated code: output=735b5ff7962ab281 input=f0df747a039948a5]
fn sys_is_finalizing_impl(module: *mut _object) -> *mut _object {
    unsafe { PyBool_FromLong(Py_IsFinalizing() as i64) }
}

// [clinic input]
// sys.activate_stack_trampoline
//
//     backend: str
//     /
//
// Activate stack profiler trampoline *backend*.
// [clinic start generated code]
/// [clinic end generated code: output=5783cdeb51874b43 input=a12df928758a82b4]
fn sys_activate_stack_trampoline_impl(module: *mut _object, backend: *const i8) -> *mut _object {
    if (unsafe {
        strcmp(
            backend as *const core::ffi::c_char,
            c"perf".as_ptr() as *const core::ffi::c_char,
        ) as i32
    }) == 0
    {
        let mut cur_cb: __SlateRecord_pycore_ceval_h_102 = unsafe { core::mem::zeroed() };
        unsafe { _PyPerfTrampoline_GetCallbacks(core::ptr::addr_of_mut!(cur_cb)) };
        if cur_cb.write_state != unsafe { _Py_perfmap_callbacks.write_state } {
            if (unsafe {
                _PyPerfTrampoline_SetCallbacks(core::ptr::addr_of_mut!(_Py_perfmap_callbacks))
            }) < 0
            {
                unsafe {
                    PyErr_SetString(PyExc_ValueError, c"can't activate perf trampoline".as_ptr())
                };
                return core::ptr::null_mut::<_object>();
            }
        }
    } else {
        if (unsafe {
            strcmp(
                backend as *const core::ffi::c_char,
                c"perf_jit".as_ptr() as *const core::ffi::c_char,
            ) as i32
        }) == 0
        {
            let mut cur_cb: __SlateRecord_pycore_ceval_h_102 = unsafe { core::mem::zeroed() };
            unsafe { _PyPerfTrampoline_GetCallbacks(core::ptr::addr_of_mut!(cur_cb)) };
            if cur_cb.write_state != unsafe { _Py_perfmap_jit_callbacks.write_state } {
                if (unsafe {
                    _PyPerfTrampoline_SetCallbacks(core::ptr::addr_of_mut!(
                        _Py_perfmap_jit_callbacks
                    ))
                }) < 0
                {
                    unsafe {
                        PyErr_SetString(
                            PyExc_ValueError,
                            c"can't activate perf jit trampoline".as_ptr(),
                        )
                    };
                    return core::ptr::null_mut::<_object>();
                }
            }
        } else {
            unsafe { PyErr_Format(PyExc_ValueError, c"invalid backend: %s".as_ptr(), backend) };
            return core::ptr::null_mut::<_object>();
        }
    }
    if (unsafe { _PyPerfTrampoline_Init(1i32) }) < 0 {
        return core::ptr::null_mut::<_object>();
    }
    unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
}

// [clinic input]
// sys.deactivate_stack_trampoline
//
// Deactivate the current stack profiler trampoline backend.
//
// If no stack profiler is activated, this function has no effect.
// [clinic start generated code]
/// [clinic end generated code: output=b50da25465df0ef1 input=9f629a6be9fe7fc8]
fn sys_deactivate_stack_trampoline_impl(module: *mut _object) -> *mut _object {
    if (unsafe { _PyPerfTrampoline_Init(0i32) }) < 0 {
        return core::ptr::null_mut::<_object>();
    }
    unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
}

// [clinic input]
// sys.is_stack_trampoline_active
//
// Return *True* if a stack profiler trampoline is active.
// [clinic start generated code]
/// [clinic end generated code: output=ab2746de0ad9d293 input=29616b7bf6a0b703]
fn sys_is_stack_trampoline_active_impl(module: *mut _object) -> *mut _object {
    if (unsafe { _PyIsPerfTrampolineActive() }) != 0 {
        return _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_TrueStruct) });
    }
    _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_FalseStruct) })
}

// [clinic input]
// sys.is_remote_debug_enabled
//
// Return True if remote debugging is enabled, False otherwise.
// [clinic start generated code]
/// [clinic end generated code: output=7ca3d38bdd5935eb input=7335c4a2fe8cf4f3]
fn sys_is_remote_debug_enabled_impl(module: *mut _object) -> *mut _object {
    let config: *const PyConfig = unsafe { _Py_GetConfig() };
    unsafe { PyBool_FromLong((*config).remote_debug as i64) }
}

// [clinic input]
// sys.remote_exec
//
//     pid: int
//     script: object
//
// Executes a file containing Python code in a given remote Python process.
//
// This function returns immediately, and the code will be executed by the
// target process's main thread at the next available opportunity,
// similarly to how signals are handled.  There is no interface to
// determine when the code has been executed.  The caller is responsible
// for making sure that the file still exists whenever the remote process
// tries to read it and that it hasn't been overwritten.
//
// The remote process must be running a CPython interpreter of the same
// major and minor version as the local process.  If either the local or
// remote interpreter is pre-release (alpha, beta, or release candidate)
// then the local and remote interpreters must be the same exact version.
//
// Args:
//      pid (int): The process ID of the target Python process.
//      script (str|bytes): The path to a file containing
//          the Python code to be executed.
// [clinic start generated code]
/// [clinic end generated code: output=7d94c56afe4a52c0 input=7bd58f8da20cb74c]
fn sys_remote_exec_impl(module: *mut _object, pid: i32, script: *mut _object) -> *mut _object {
    let mut debugger_script_path: *const i8 = unsafe { core::mem::zeroed() };
    let mut path: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        if PyUnicode_FSConverter(script, core::ptr::addr_of_mut!(path) as *mut ()) == 0 {
            return core::ptr::null_mut::<_object>();
        } else {
            if PySys_Audit(c"sys.remote_exec".as_ptr(), c"iO".as_ptr(), pid, script) < 0 {
                return core::ptr::null_mut::<_object>();
            } else {
                debugger_script_path = PyBytes_AS_STRING!(path) as *const i8;
                if (access(
                    debugger_script_path as *const core::ffi::c_char,
                    ((slate_support::F_OK as i32) | (slate_support::R_OK as i32)) as i32,
                ) as i32)
                    != 0
                {
                    let __t0: i32 = *slate_support::errno_location();
                    if __t0 == 2 {
                        PyErr_SetString(
                            PyExc_FileNotFoundError,
                            c"Script file does not exist".as_ptr(),
                        );
                    } else {
                        if __t0 == 13 {
                            PyErr_SetString(
                                PyExc_PermissionError,
                                c"Script file cannot be read".as_ptr(),
                            );
                        } else {
                            PyErr_SetFromErrno(PyExc_OSError);
                        }
                    }
                } else {
                    if _PySysRemoteDebug_SendExec(pid, 0i32, debugger_script_path) >= 0 {
                        Py_DECREF!(path);
                        return core::ptr::addr_of_mut!(_Py_NoneStruct);
                    }
                }
                Py_DECREF!(path);
                return core::ptr::null_mut::<_object>();
            }
        }
    }
}

// [clinic input]
// sys._dump_tracelets
//
//     outpath: object
//
// Dump the graph of tracelets in graphviz format
// [clinic start generated code]
/// [clinic end generated code: output=a7fe265e2bc3b674 input=5bff6880cd28ffd1]
fn sys__dump_tracelets_impl(module: *mut _object, outpath: *mut _object) -> *mut _object {
    let out: *mut slate_support::FILE = unsafe { Py_fopen(outpath, c"wb".as_ptr()) };
    if out.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let err: i32 = unsafe { _PyDumpExecutors(out) };
    unsafe { slate_support::fclose(out) };
    if err != 0 {
        return core::ptr::null_mut::<_object>();
    }
    unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
}

// [clinic input]
// sys._getframemodulename
//
//     depth: int = 0
//
// Return the name of the module for a calling frame.
//
// The default depth returns the module containing the call to this API.
// A more typical use in a library will pass a depth of 1 to get the user's
// module rather than the library module.
//
// If no frame, module, or name can be found, returns None.
// [clinic start generated code]
/// [clinic end generated code: output=1d70ef691f09d2db input=d4f1a8ed43b8fb46]
fn sys__getframemodulename_impl(module: *mut _object, mut depth: i32) -> *mut _object {
    if (unsafe { PySys_Audit(c"sys._getframemodulename".as_ptr(), c"i".as_ptr(), depth) }) < 0 {
        return core::ptr::null_mut::<_object>();
    }
    let mut f: *mut _PyInterpreterFrame = unsafe { (*_PyThreadState_GET()).current_frame };
    loop {
        let __v0: bool = if !f.is_null() {
            let __v1: bool = if _PyFrame_IsIncomplete(f) {
                true
            } else {
                let __v2: i32 = depth;
                depth = __v2.wrapping_sub(1);
                __v2 > 0
            };
            __v1
        } else {
            false
        };
        if !__v0 {
            break;
        }
        f = unsafe { (*f).previous };
    }
    if f.is_null() || PyStackRef_IsNull!((*f).f_funcobj) {
        return unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) };
    }
    let func: *mut _object = PyStackRef_AsPyObjectBorrow(unsafe { (*f).f_funcobj });
    let mut r: *mut _object = unsafe { PyFunction_GetModule(func) };
    if r.is_null() {
        unsafe { PyErr_Clear() };
        r = unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) };
    }
    Py_NewRef!(r)
}

// [clinic input]
// sys._get_cpu_count_config -> int
//
// Private function for getting PyConfig.cpu_count
// [clinic start generated code]
/// [clinic end generated code: output=36611bb5efad16dc input=523e1ade2204084e]
fn sys__get_cpu_count_config_impl(module: *mut _object) -> i32 {
    let config: *const PyConfig = unsafe { _Py_GetConfig() };
    unsafe { (*config).cpu_count }
}

// [clinic input]
// sys._baserepl
//
// Private function for getting the base REPL
// [clinic start generated code]
/// [clinic end generated code: output=f19a36375ebe0a45 input=ade0ebb9fab56f3c]
fn sys__baserepl_impl(module: *mut _object) -> *mut _object {
    let mut __slate_compound_0: __SlateRecord_compile_h_30 = unsafe { core::mem::zeroed() };
    let mut cf: __SlateRecord_compile_h_30 = unsafe {
        *{
            __slate_compound_0 = __SlateRecord_compile_h_30 {
                cf_flags: 0,
                cf_feature_version: 14,
            };
            core::ptr::addr_of_mut!(__slate_compound_0)
        }
    };
    unsafe {
        PyRun_AnyFileExFlags(
            slate_support::stdin(),
            c"<stdin>".as_ptr(),
            0i32,
            core::ptr::addr_of_mut!(cf),
        )
    };
    unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
}

// [clinic input]
// sys._clear_type_descriptors
//
//     type: object(subclass_of='&PyType_Type')
//     /
//
// Private function for clearing certain descriptors from a type's dictionary.
//
// See gh-135228 for context.
// [clinic start generated code]
/// [clinic end generated code: output=5ad17851b762b6d9 input=dc536c97fde07251]
fn sys__clear_type_descriptors_impl(module: *mut _object, r#type: *mut _object) -> *mut _object {
    let typeobj: *mut _typeobject = r#type as *mut _typeobject;
    if _PyType_HasFeature(typeobj, 256u64) != 0 {
        unsafe { PyErr_SetString(PyExc_TypeError, c"argument is immutable".as_ptr()) };
        return core::ptr::null_mut::<_object>();
    }
    let dict: *mut _object = unsafe { _PyType_GetDict(typeobj) };
    let mut dunder_dict: *mut _object = core::ptr::null_mut::<_object>();
    if (unsafe {
        PyDict_Pop(
            dict,
            core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py___dict__
                    ._ascii
                    .ob_base
            ),
            core::ptr::addr_of_mut!(dunder_dict),
        )
    }) < 0
    {
        return core::ptr::null_mut::<_object>();
    }
    let mut dunder_weakref: *mut _object = core::ptr::null_mut::<_object>();
    if (unsafe {
        PyDict_Pop(
            dict,
            core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py___weakref__
                    ._ascii
                    .ob_base
            ),
            core::ptr::addr_of_mut!(dunder_weakref),
        )
    }) < 0
    {
        unsafe { PyType_Modified(typeobj) };
        Py_XDECREF!(dunder_dict);
        return core::ptr::null_mut::<_object>();
    }
    unsafe { PyType_Modified(typeobj) };
    // We try to hold onto a reference to these until after we call
    //
    // PyType_Modified(), in case their deallocation triggers somer user code
    //
    // that tries to do something to the type.
    Py_XDECREF!(dunder_dict);
    Py_XDECREF!(dunder_weakref);
    unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
}

// [clinic input]
// sys._is_gil_enabled -> bool
//
// Return True if the GIL is currently enabled and False otherwise.
// [clinic start generated code]
/// [clinic end generated code: output=57732cf53f5b9120 input=7e9c47f15a00e809]
fn sys__is_gil_enabled_impl(module: *mut _object) -> i32 {
    1
}

static mut perf_map_state: __SlateRecord_ceval_h_33 = unsafe { core::mem::zeroed() };

#[unsafe(no_mangle)]
extern "C-unwind" fn PyUnstable_PerfMapState_Init() -> i32 {
    let mut filename: __SlateAlign16<[i8; 100]> = __SlateAlign16([0i8; 100]);
    let pid: i32 = unsafe { getpid() as i32 };
    // Use nofollow flag to prevent symlink attacks.
    let mut flags: i32 = (slate_support::O_WRONLY as i32)
        | (slate_support::O_CREAT as i32)
        | (slate_support::O_APPEND as i32)
        | 131072;
    flags |= 524288;
    unsafe {
        snprintf(
            (filename.0.as_mut_ptr() as *mut i8) as *mut core::ffi::c_char,
            (core::mem::size_of::<[i8; 100]>() as u64).wrapping_sub(1) as usize,
            c"/tmp/perf-%jd.map".as_ptr() as *const core::ffi::c_char,
            pid as i64,
        ) as i32
    };
    let fd: i32 = unsafe { open(filename.0.as_mut_ptr() as *const i8, flags, 384i32) };
    if fd == 1i32.wrapping_neg() {
        return 1i32.wrapping_neg();
    } else {
        unsafe {
            perf_map_state.perf_map = fdopen(fd, c"a".as_ptr());
        }
        if unsafe { perf_map_state.perf_map }.is_null() {
            unsafe { close(fd as i32) as i32 };
            return 1i32.wrapping_neg();
        }
    }
    unsafe {
        perf_map_state.map_lock = PyThread_allocate_lock();
    }
    if unsafe { perf_map_state.map_lock }.is_null() {
        unsafe { slate_support::fclose(perf_map_state.perf_map) };
        return 2i32.wrapping_neg();
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyUnstable_WritePerfMapEntry(
    code_addr: *const (),
    code_size: u32,
    entry_name: *const i8,
) -> i32 {
    if unsafe { perf_map_state.perf_map }.is_null() {
        let ret: i32 = PyUnstable_PerfMapState_Init();
        if ret != 0 {
            return ret;
        }
    }
    unsafe { PyThread_acquire_lock(perf_map_state.map_lock, 1i32) };
    let _ = std::io::Write::flush(&mut std::io::stdout());
    unsafe {
        slate_support::fprintf(
            perf_map_state.perf_map,
            c"%lx %x %s\n".as_ptr(),
            code_addr as u64,
            code_size,
            entry_name,
        )
    };
    unsafe { fflush(core::ptr::null_mut()) };
    unsafe { fflush(perf_map_state.perf_map) };
    unsafe { PyThread_release_lock(perf_map_state.map_lock) };
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyUnstable_PerfMapState_Fini() {
    if !unsafe { perf_map_state.perf_map }.is_null() {
        // close the file
        unsafe { PyThread_acquire_lock(perf_map_state.map_lock, 1i32) };
        unsafe { slate_support::fclose(perf_map_state.perf_map) };
        unsafe { PyThread_release_lock(perf_map_state.map_lock) };
        // clean up the lock and state
        unsafe { PyThread_free_lock(perf_map_state.map_lock) };
        unsafe {
            perf_map_state.perf_map = core::ptr::null_mut::<slate_support::FILE>();
        }
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyUnstable_CopyPerfMapFile(parent_filename: *const i8) -> i32 {
    if unsafe { perf_map_state.perf_map }.is_null() {
        let ret: i32 = PyUnstable_PerfMapState_Init();
        if ret != 0 {
            return ret;
        }
    }
    let from: *mut slate_support::FILE = unsafe { fopen(parent_filename, c"r".as_ptr()) };
    if from.is_null() {
        return 1i32.wrapping_neg();
    }
    let mut buf: __SlateAlign16<[i8; 4096]> = __SlateAlign16([0i8; 4096]);
    unsafe { PyThread_acquire_lock(perf_map_state.map_lock, 1i32) };
    let mut result: i32 = 0;
    while true {
        let bytes_read: u64 = unsafe {
            fread(
                buf.0.as_mut_ptr() as *mut (),
                1u64,
                core::mem::size_of::<[i8; 4096]>() as u64,
                from,
            )
        };
        if bytes_read == 0 {
            if (unsafe { slate_support::ferror(from) }) != 0 {
                result = 1i32.wrapping_neg();
            }
            break;
        }
        let _ = std::io::Write::flush(&mut std::io::stdout());
        let bytes_written: u64 = unsafe {
            fwrite(
                buf.0.as_mut_ptr() as *const (),
                1u64,
                bytes_read,
                perf_map_state.perf_map,
            )
        };
        unsafe { fflush(core::ptr::null_mut()) };
        if bytes_written < bytes_read {
            result = 1i32.wrapping_neg();
            break;
        }
        if (unsafe { fflush(perf_map_state.perf_map) }) != 0 {
            result = 1i32.wrapping_neg();
            break;
        }
        let __v1: bool = bytes_read < (core::mem::size_of::<[i8; 4096]>() as u64)
            && (unsafe { slate_support::feof(from) }) != 0;
        if __v1 {
            break;
        }
    }
    unsafe { slate_support::fclose(from) };
    unsafe { PyThread_release_lock(perf_map_state.map_lock) };
    result
}

/// Might as well keep this in alphabetic order
/// sentinel
static mut sys_methods: __SlateAlign16<[PyMethodDef; 57]> = __SlateAlign16([
    {
        let mut __t0: PyMethodDef = unsafe { core::mem::zeroed() };
        __t0.ml_name = (b"addaudithook\0".as_ptr() as *mut i8) as *const i8;
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
                >(Some(sys_addaudithook))
            })
        };
        __t0.ml_flags = 130i32;
        __t0.ml_doc = unsafe { core::ptr::addr_of!(sys_addaudithook__doc__.0) as *const i8 };
        __t0
    },
    {
        let mut __t1: PyMethodDef = unsafe { core::mem::zeroed() };
        __t1.ml_name = (b"audit\0".as_ptr() as *mut i8) as *const i8;
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
                        ) -> *mut _object,
                    >,
                    Option<unsafe extern "C-unwind" fn()>,
                >(Some(sys_audit))
            })
        };
        __t1.ml_flags = 128i32;
        __t1.ml_doc = unsafe { core::ptr::addr_of!(sys_audit__doc__.0) as *const i8 };
        __t1
    },
    {
        let mut __t2: PyMethodDef = unsafe { core::mem::zeroed() };
        __t2.ml_name = (b"breakpointhook\0".as_ptr() as *mut i8) as *const i8;
        __t2.ml_meth = unsafe {
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
                >(Some(sys_breakpointhook))
            })
        };
        __t2.ml_flags = 130i32;
        __t2.ml_doc = unsafe { core::ptr::addr_of!(breakpointhook_doc.0) as *const i8 };
        __t2
    },
    {
        let mut __t3: PyMethodDef = unsafe { core::mem::zeroed() };
        __t3.ml_name = (b"_clear_internal_caches\0".as_ptr() as *mut i8) as *const i8;
        __t3.ml_meth = Some(sys__clear_internal_caches);
        __t3.ml_flags = 4i32;
        __t3.ml_doc =
            unsafe { core::ptr::addr_of!(sys__clear_internal_caches__doc__.0) as *const i8 };
        __t3
    },
    {
        let mut __t4: PyMethodDef = unsafe { core::mem::zeroed() };
        __t4.ml_name = (b"_clear_type_cache\0".as_ptr() as *mut i8) as *const i8;
        __t4.ml_meth = Some(sys__clear_type_cache);
        __t4.ml_flags = 4i32;
        __t4.ml_doc = unsafe { core::ptr::addr_of!(sys__clear_type_cache__doc__.0) as *const i8 };
        __t4
    },
    {
        let mut __t5: PyMethodDef = unsafe { core::mem::zeroed() };
        __t5.ml_name = (b"_current_frames\0".as_ptr() as *mut i8) as *const i8;
        __t5.ml_meth = Some(sys__current_frames);
        __t5.ml_flags = 4i32;
        __t5.ml_doc = unsafe { core::ptr::addr_of!(sys__current_frames__doc__.0) as *const i8 };
        __t5
    },
    {
        let mut __t6: PyMethodDef = unsafe { core::mem::zeroed() };
        __t6.ml_name = (b"_current_exceptions\0".as_ptr() as *mut i8) as *const i8;
        __t6.ml_meth = Some(sys__current_exceptions);
        __t6.ml_flags = 4i32;
        __t6.ml_doc = unsafe { core::ptr::addr_of!(sys__current_exceptions__doc__.0) as *const i8 };
        __t6
    },
    {
        let mut __t7: PyMethodDef = unsafe { core::mem::zeroed() };
        __t7.ml_name = (b"displayhook\0".as_ptr() as *mut i8) as *const i8;
        __t7.ml_meth = Some(sys_displayhook);
        __t7.ml_flags = 8i32;
        __t7.ml_doc = unsafe { core::ptr::addr_of!(sys_displayhook__doc__.0) as *const i8 };
        __t7
    },
    {
        let mut __t8: PyMethodDef = unsafe { core::mem::zeroed() };
        __t8.ml_name = (b"exception\0".as_ptr() as *mut i8) as *const i8;
        __t8.ml_meth = Some(sys_exception);
        __t8.ml_flags = 4i32;
        __t8.ml_doc = unsafe { core::ptr::addr_of!(sys_exception__doc__.0) as *const i8 };
        __t8
    },
    {
        let mut __t9: PyMethodDef = unsafe { core::mem::zeroed() };
        __t9.ml_name = (b"exc_info\0".as_ptr() as *mut i8) as *const i8;
        __t9.ml_meth = Some(sys_exc_info);
        __t9.ml_flags = 4i32;
        __t9.ml_doc = unsafe { core::ptr::addr_of!(sys_exc_info__doc__.0) as *const i8 };
        __t9
    },
    {
        let mut __t10: PyMethodDef = unsafe { core::mem::zeroed() };
        __t10.ml_name = (b"excepthook\0".as_ptr() as *mut i8) as *const i8;
        __t10.ml_meth = unsafe {
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
                >(Some(sys_excepthook))
            })
        };
        __t10.ml_flags = 128i32;
        __t10.ml_doc = unsafe { core::ptr::addr_of!(sys_excepthook__doc__.0) as *const i8 };
        __t10
    },
    {
        let mut __t11: PyMethodDef = unsafe { core::mem::zeroed() };
        __t11.ml_name = (b"exit\0".as_ptr() as *mut i8) as *const i8;
        __t11.ml_meth = unsafe {
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
                >(Some(sys_exit))
            })
        };
        __t11.ml_flags = 128i32;
        __t11.ml_doc = unsafe { core::ptr::addr_of!(sys_exit__doc__.0) as *const i8 };
        __t11
    },
    {
        let mut __t12: PyMethodDef = unsafe { core::mem::zeroed() };
        __t12.ml_name = (b"getdefaultencoding\0".as_ptr() as *mut i8) as *const i8;
        __t12.ml_meth = Some(sys_getdefaultencoding);
        __t12.ml_flags = 4i32;
        __t12.ml_doc = unsafe { core::ptr::addr_of!(sys_getdefaultencoding__doc__.0) as *const i8 };
        __t12
    },
    {
        let mut __t13: PyMethodDef = unsafe { core::mem::zeroed() };
        __t13.ml_name = (b"getdlopenflags\0".as_ptr() as *mut i8) as *const i8;
        __t13.ml_meth = Some(sys_getdlopenflags);
        __t13.ml_flags = 4i32;
        __t13.ml_doc = unsafe { core::ptr::addr_of!(sys_getdlopenflags__doc__.0) as *const i8 };
        __t13
    },
    {
        let mut __t14: PyMethodDef = unsafe { core::mem::zeroed() };
        __t14.ml_name = (b"getallocatedblocks\0".as_ptr() as *mut i8) as *const i8;
        __t14.ml_meth = Some(sys_getallocatedblocks);
        __t14.ml_flags = 4i32;
        __t14.ml_doc = unsafe { core::ptr::addr_of!(sys_getallocatedblocks__doc__.0) as *const i8 };
        __t14
    },
    {
        let mut __t15: PyMethodDef = unsafe { core::mem::zeroed() };
        __t15.ml_name = (b"getunicodeinternedsize\0".as_ptr() as *mut i8) as *const i8;
        __t15.ml_meth = unsafe {
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
                >(Some(sys_getunicodeinternedsize))
            })
        };
        __t15.ml_flags = 130i32;
        __t15.ml_doc =
            unsafe { core::ptr::addr_of!(sys_getunicodeinternedsize__doc__.0) as *const i8 };
        __t15
    },
    {
        let mut __t16: PyMethodDef = unsafe { core::mem::zeroed() };
        __t16.ml_name = (b"getfilesystemencoding\0".as_ptr() as *mut i8) as *const i8;
        __t16.ml_meth = Some(sys_getfilesystemencoding);
        __t16.ml_flags = 4i32;
        __t16.ml_doc =
            unsafe { core::ptr::addr_of!(sys_getfilesystemencoding__doc__.0) as *const i8 };
        __t16
    },
    {
        let mut __t17: PyMethodDef = unsafe { core::mem::zeroed() };
        __t17.ml_name = (b"getfilesystemencodeerrors\0".as_ptr() as *mut i8) as *const i8;
        __t17.ml_meth = Some(sys_getfilesystemencodeerrors);
        __t17.ml_flags = 4i32;
        __t17.ml_doc =
            unsafe { core::ptr::addr_of!(sys_getfilesystemencodeerrors__doc__.0) as *const i8 };
        __t17
    },
    {
        let mut __t18: PyMethodDef = unsafe { core::mem::zeroed() };
        __t18.ml_name = (b"getrefcount\0".as_ptr() as *mut i8) as *const i8;
        __t18.ml_meth = Some(sys_getrefcount);
        __t18.ml_flags = 8i32;
        __t18.ml_doc = unsafe { core::ptr::addr_of!(sys_getrefcount__doc__.0) as *const i8 };
        __t18
    },
    {
        let mut __t19: PyMethodDef = unsafe { core::mem::zeroed() };
        __t19.ml_name = (b"getrecursionlimit\0".as_ptr() as *mut i8) as *const i8;
        __t19.ml_meth = Some(sys_getrecursionlimit);
        __t19.ml_flags = 4i32;
        __t19.ml_doc = unsafe { core::ptr::addr_of!(sys_getrecursionlimit__doc__.0) as *const i8 };
        __t19
    },
    {
        let mut __t20: PyMethodDef = unsafe { core::mem::zeroed() };
        __t20.ml_name = (b"getsizeof\0".as_ptr() as *mut i8) as *const i8;
        __t20.ml_meth = unsafe {
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
                >(Some(sys_getsizeof))
            })
        };
        __t20.ml_flags = 3i32;
        __t20.ml_doc = unsafe { core::ptr::addr_of!(getsizeof_doc.0) as *const i8 };
        __t20
    },
    {
        let mut __t21: PyMethodDef = unsafe { core::mem::zeroed() };
        __t21.ml_name = (b"_getframe\0".as_ptr() as *mut i8) as *const i8;
        __t21.ml_meth = unsafe {
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
                >(Some(sys__getframe))
            })
        };
        __t21.ml_flags = 128i32;
        __t21.ml_doc = unsafe { core::ptr::addr_of!(sys__getframe__doc__.0) as *const i8 };
        __t21
    },
    {
        let mut __t22: PyMethodDef = unsafe { core::mem::zeroed() };
        __t22.ml_name = (b"_getframemodulename\0".as_ptr() as *mut i8) as *const i8;
        __t22.ml_meth = unsafe {
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
                >(Some(sys__getframemodulename))
            })
        };
        __t22.ml_flags = 130i32;
        __t22.ml_doc =
            unsafe { core::ptr::addr_of!(sys__getframemodulename__doc__.0) as *const i8 };
        __t22
    },
    {
        let mut __t23: PyMethodDef = unsafe { core::mem::zeroed() };
        __t23.ml_name = (b"_is_immortal\0".as_ptr() as *mut i8) as *const i8;
        __t23.ml_meth = Some(sys__is_immortal);
        __t23.ml_flags = 8i32;
        __t23.ml_doc = unsafe { core::ptr::addr_of!(sys__is_immortal__doc__.0) as *const i8 };
        __t23
    },
    {
        let mut __t24: PyMethodDef = unsafe { core::mem::zeroed() };
        __t24.ml_name = (b"intern\0".as_ptr() as *mut i8) as *const i8;
        __t24.ml_meth = Some(sys_intern);
        __t24.ml_flags = 8i32;
        __t24.ml_doc = unsafe { core::ptr::addr_of!(sys_intern__doc__.0) as *const i8 };
        __t24
    },
    {
        let mut __t25: PyMethodDef = unsafe { core::mem::zeroed() };
        __t25.ml_name = (b"_is_interned\0".as_ptr() as *mut i8) as *const i8;
        __t25.ml_meth = Some(sys__is_interned);
        __t25.ml_flags = 8i32;
        __t25.ml_doc = unsafe { core::ptr::addr_of!(sys__is_interned__doc__.0) as *const i8 };
        __t25
    },
    {
        let mut __t26: PyMethodDef = unsafe { core::mem::zeroed() };
        __t26.ml_name = (b"is_finalizing\0".as_ptr() as *mut i8) as *const i8;
        __t26.ml_meth = Some(sys_is_finalizing);
        __t26.ml_flags = 4i32;
        __t26.ml_doc = unsafe { core::ptr::addr_of!(sys_is_finalizing__doc__.0) as *const i8 };
        __t26
    },
    {
        let mut __t27: PyMethodDef = unsafe { core::mem::zeroed() };
        __t27.ml_name = (b"setswitchinterval\0".as_ptr() as *mut i8) as *const i8;
        __t27.ml_meth = Some(sys_setswitchinterval);
        __t27.ml_flags = 8i32;
        __t27.ml_doc = unsafe { core::ptr::addr_of!(sys_setswitchinterval__doc__.0) as *const i8 };
        __t27
    },
    {
        let mut __t28: PyMethodDef = unsafe { core::mem::zeroed() };
        __t28.ml_name = (b"getswitchinterval\0".as_ptr() as *mut i8) as *const i8;
        __t28.ml_meth = Some(sys_getswitchinterval);
        __t28.ml_flags = 4i32;
        __t28.ml_doc = unsafe { core::ptr::addr_of!(sys_getswitchinterval__doc__.0) as *const i8 };
        __t28
    },
    {
        let mut __t29: PyMethodDef = unsafe { core::mem::zeroed() };
        __t29.ml_name = (b"setdlopenflags\0".as_ptr() as *mut i8) as *const i8;
        __t29.ml_meth = Some(sys_setdlopenflags);
        __t29.ml_flags = 8i32;
        __t29.ml_doc = unsafe { core::ptr::addr_of!(sys_setdlopenflags__doc__.0) as *const i8 };
        __t29
    },
    {
        let mut __t30: PyMethodDef = unsafe { core::mem::zeroed() };
        __t30.ml_name = (b"setprofile\0".as_ptr() as *mut i8) as *const i8;
        __t30.ml_meth = Some(sys_setprofile);
        __t30.ml_flags = 8i32;
        __t30.ml_doc = unsafe { core::ptr::addr_of!(sys_setprofile__doc__.0) as *const i8 };
        __t30
    },
    {
        let mut __t31: PyMethodDef = unsafe { core::mem::zeroed() };
        __t31.ml_name = (b"_setprofileallthreads\0".as_ptr() as *mut i8) as *const i8;
        __t31.ml_meth = Some(sys__setprofileallthreads);
        __t31.ml_flags = 8i32;
        __t31.ml_doc =
            unsafe { core::ptr::addr_of!(sys__setprofileallthreads__doc__.0) as *const i8 };
        __t31
    },
    {
        let mut __t32: PyMethodDef = unsafe { core::mem::zeroed() };
        __t32.ml_name = (b"getprofile\0".as_ptr() as *mut i8) as *const i8;
        __t32.ml_meth = Some(sys_getprofile);
        __t32.ml_flags = 4i32;
        __t32.ml_doc = unsafe { core::ptr::addr_of!(sys_getprofile__doc__.0) as *const i8 };
        __t32
    },
    {
        let mut __t33: PyMethodDef = unsafe { core::mem::zeroed() };
        __t33.ml_name = (b"setrecursionlimit\0".as_ptr() as *mut i8) as *const i8;
        __t33.ml_meth = Some(sys_setrecursionlimit);
        __t33.ml_flags = 8i32;
        __t33.ml_doc = unsafe { core::ptr::addr_of!(sys_setrecursionlimit__doc__.0) as *const i8 };
        __t33
    },
    {
        let mut __t34: PyMethodDef = unsafe { core::mem::zeroed() };
        __t34.ml_name = (b"settrace\0".as_ptr() as *mut i8) as *const i8;
        __t34.ml_meth = Some(sys_settrace);
        __t34.ml_flags = 8i32;
        __t34.ml_doc = unsafe { core::ptr::addr_of!(sys_settrace__doc__.0) as *const i8 };
        __t34
    },
    {
        let mut __t35: PyMethodDef = unsafe { core::mem::zeroed() };
        __t35.ml_name = (b"_settraceallthreads\0".as_ptr() as *mut i8) as *const i8;
        __t35.ml_meth = Some(sys__settraceallthreads);
        __t35.ml_flags = 8i32;
        __t35.ml_doc =
            unsafe { core::ptr::addr_of!(sys__settraceallthreads__doc__.0) as *const i8 };
        __t35
    },
    {
        let mut __t36: PyMethodDef = unsafe { core::mem::zeroed() };
        __t36.ml_name = (b"gettrace\0".as_ptr() as *mut i8) as *const i8;
        __t36.ml_meth = Some(sys_gettrace);
        __t36.ml_flags = 4i32;
        __t36.ml_doc = unsafe { core::ptr::addr_of!(sys_gettrace__doc__.0) as *const i8 };
        __t36
    },
    {
        let mut __t37: PyMethodDef = unsafe { core::mem::zeroed() };
        __t37.ml_name = (b"call_tracing\0".as_ptr() as *mut i8) as *const i8;
        __t37.ml_meth = unsafe {
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
                >(Some(sys_call_tracing))
            })
        };
        __t37.ml_flags = 128i32;
        __t37.ml_doc = unsafe { core::ptr::addr_of!(sys_call_tracing__doc__.0) as *const i8 };
        __t37
    },
    {
        let mut __t38: PyMethodDef = unsafe { core::mem::zeroed() };
        __t38.ml_name = (b"_debugmallocstats\0".as_ptr() as *mut i8) as *const i8;
        __t38.ml_meth = Some(sys__debugmallocstats);
        __t38.ml_flags = 4i32;
        __t38.ml_doc = unsafe { core::ptr::addr_of!(sys__debugmallocstats__doc__.0) as *const i8 };
        __t38
    },
    {
        let mut __t39: PyMethodDef = unsafe { core::mem::zeroed() };
        __t39.ml_name = (b"set_coroutine_origin_tracking_depth\0".as_ptr() as *mut i8) as *const i8;
        __t39.ml_meth = unsafe {
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
                >(Some(sys_set_coroutine_origin_tracking_depth))
            })
        };
        __t39.ml_flags = 130i32;
        __t39.ml_doc = unsafe {
            core::ptr::addr_of!(sys_set_coroutine_origin_tracking_depth__doc__.0) as *const i8
        };
        __t39
    },
    {
        let mut __t40: PyMethodDef = unsafe { core::mem::zeroed() };
        __t40.ml_name = (b"get_coroutine_origin_tracking_depth\0".as_ptr() as *mut i8) as *const i8;
        __t40.ml_meth = Some(sys_get_coroutine_origin_tracking_depth);
        __t40.ml_flags = 4i32;
        __t40.ml_doc = unsafe {
            core::ptr::addr_of!(sys_get_coroutine_origin_tracking_depth__doc__.0) as *const i8
        };
        __t40
    },
    {
        let mut __t41: PyMethodDef = unsafe { core::mem::zeroed() };
        __t41.ml_name = (b"set_asyncgen_hooks\0".as_ptr() as *mut i8) as *const i8;
        __t41.ml_meth = unsafe {
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
                >(Some(sys_set_asyncgen_hooks))
            })
        };
        __t41.ml_flags = 3i32;
        __t41.ml_doc = unsafe { core::ptr::addr_of!(set_asyncgen_hooks_doc.0) as *const i8 };
        __t41
    },
    {
        let mut __t42: PyMethodDef = unsafe { core::mem::zeroed() };
        __t42.ml_name = (b"get_asyncgen_hooks\0".as_ptr() as *mut i8) as *const i8;
        __t42.ml_meth = Some(sys_get_asyncgen_hooks);
        __t42.ml_flags = 4i32;
        __t42.ml_doc = unsafe { core::ptr::addr_of!(sys_get_asyncgen_hooks__doc__.0) as *const i8 };
        __t42
    },
    {
        let mut __t43: PyMethodDef = unsafe { core::mem::zeroed() };
        __t43.ml_name = (b"activate_stack_trampoline\0".as_ptr() as *mut i8) as *const i8;
        __t43.ml_meth = Some(sys_activate_stack_trampoline);
        __t43.ml_flags = 8i32;
        __t43.ml_doc =
            unsafe { core::ptr::addr_of!(sys_activate_stack_trampoline__doc__.0) as *const i8 };
        __t43
    },
    {
        let mut __t44: PyMethodDef = unsafe { core::mem::zeroed() };
        __t44.ml_name = (b"deactivate_stack_trampoline\0".as_ptr() as *mut i8) as *const i8;
        __t44.ml_meth = Some(sys_deactivate_stack_trampoline);
        __t44.ml_flags = 4i32;
        __t44.ml_doc =
            unsafe { core::ptr::addr_of!(sys_deactivate_stack_trampoline__doc__.0) as *const i8 };
        __t44
    },
    {
        let mut __t45: PyMethodDef = unsafe { core::mem::zeroed() };
        __t45.ml_name = (b"is_stack_trampoline_active\0".as_ptr() as *mut i8) as *const i8;
        __t45.ml_meth = Some(sys_is_stack_trampoline_active);
        __t45.ml_flags = 4i32;
        __t45.ml_doc =
            unsafe { core::ptr::addr_of!(sys_is_stack_trampoline_active__doc__.0) as *const i8 };
        __t45
    },
    {
        let mut __t46: PyMethodDef = unsafe { core::mem::zeroed() };
        __t46.ml_name = (b"is_remote_debug_enabled\0".as_ptr() as *mut i8) as *const i8;
        __t46.ml_meth = Some(sys_is_remote_debug_enabled);
        __t46.ml_flags = 4i32;
        __t46.ml_doc =
            unsafe { core::ptr::addr_of!(sys_is_remote_debug_enabled__doc__.0) as *const i8 };
        __t46
    },
    {
        let mut __t47: PyMethodDef = unsafe { core::mem::zeroed() };
        __t47.ml_name = (b"remote_exec\0".as_ptr() as *mut i8) as *const i8;
        __t47.ml_meth = unsafe {
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
                >(Some(sys_remote_exec))
            })
        };
        __t47.ml_flags = 130i32;
        __t47.ml_doc = unsafe { core::ptr::addr_of!(sys_remote_exec__doc__.0) as *const i8 };
        __t47
    },
    {
        let mut __t48: PyMethodDef = unsafe { core::mem::zeroed() };
        __t48.ml_name = (b"unraisablehook\0".as_ptr() as *mut i8) as *const i8;
        __t48.ml_meth = Some(sys_unraisablehook);
        __t48.ml_flags = 8i32;
        __t48.ml_doc = unsafe { core::ptr::addr_of!(sys_unraisablehook__doc__.0) as *const i8 };
        __t48
    },
    {
        let mut __t49: PyMethodDef = unsafe { core::mem::zeroed() };
        __t49.ml_name = (b"get_int_max_str_digits\0".as_ptr() as *mut i8) as *const i8;
        __t49.ml_meth = Some(sys_get_int_max_str_digits);
        __t49.ml_flags = 4i32;
        __t49.ml_doc =
            unsafe { core::ptr::addr_of!(sys_get_int_max_str_digits__doc__.0) as *const i8 };
        __t49
    },
    {
        let mut __t50: PyMethodDef = unsafe { core::mem::zeroed() };
        __t50.ml_name = (b"set_int_max_str_digits\0".as_ptr() as *mut i8) as *const i8;
        __t50.ml_meth = unsafe {
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
                >(Some(sys_set_int_max_str_digits))
            })
        };
        __t50.ml_flags = 130i32;
        __t50.ml_doc =
            unsafe { core::ptr::addr_of!(sys_set_int_max_str_digits__doc__.0) as *const i8 };
        __t50
    },
    {
        let mut __t51: PyMethodDef = unsafe { core::mem::zeroed() };
        __t51.ml_name = (b"_baserepl\0".as_ptr() as *mut i8) as *const i8;
        __t51.ml_meth = Some(sys__baserepl);
        __t51.ml_flags = 4i32;
        __t51.ml_doc = unsafe { core::ptr::addr_of!(sys__baserepl__doc__.0) as *const i8 };
        __t51
    },
    {
        let mut __t52: PyMethodDef = unsafe { core::mem::zeroed() };
        __t52.ml_name = (b"_get_cpu_count_config\0".as_ptr() as *mut i8) as *const i8;
        __t52.ml_meth = Some(sys__get_cpu_count_config);
        __t52.ml_flags = 4i32;
        __t52.ml_doc =
            unsafe { core::ptr::addr_of!(sys__get_cpu_count_config__doc__.0) as *const i8 };
        __t52
    },
    {
        let mut __t53: PyMethodDef = unsafe { core::mem::zeroed() };
        __t53.ml_name = (b"_clear_type_descriptors\0".as_ptr() as *mut i8) as *const i8;
        __t53.ml_meth = Some(sys__clear_type_descriptors);
        __t53.ml_flags = 8i32;
        __t53.ml_doc =
            unsafe { core::ptr::addr_of!(sys__clear_type_descriptors__doc__.0) as *const i8 };
        __t53
    },
    {
        let mut __t54: PyMethodDef = unsafe { core::mem::zeroed() };
        __t54.ml_name = (b"_is_gil_enabled\0".as_ptr() as *mut i8) as *const i8;
        __t54.ml_meth = Some(sys__is_gil_enabled);
        __t54.ml_flags = 4i32;
        __t54.ml_doc = unsafe { core::ptr::addr_of!(sys__is_gil_enabled__doc__.0) as *const i8 };
        __t54
    },
    {
        let mut __t55: PyMethodDef = unsafe { core::mem::zeroed() };
        __t55.ml_name = (b"_dump_tracelets\0".as_ptr() as *mut i8) as *const i8;
        __t55.ml_meth = unsafe {
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
                >(Some(sys__dump_tracelets))
            })
        };
        __t55.ml_flags = 130i32;
        __t55.ml_doc = unsafe { core::ptr::addr_of!(sys__dump_tracelets__doc__.0) as *const i8 };
        __t55
    },
    {
        let mut __t56: PyMethodDef = unsafe { core::mem::zeroed() };
        __t56.ml_name = core::ptr::null::<i8>();
        __t56.ml_meth = None;
        __t56
    },
]);

fn list_builtin_module_names() -> *mut _object {
    let mut list: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        list = _PyImport_GetBuiltinModuleNames();
        if list.is_null() {
            return core::ptr::null_mut::<_object>();
        } else {
            if PyList_Sort(list) != 0 {
                Py_DECREF!(list);
                return core::ptr::null_mut::<_object>();
            } else {
                let tuple: *mut _object = PyList_AsTuple(list);
                Py_DECREF!(list);
                return tuple;
            }
        }
    }
}

fn list_stdlib_module_names() -> *mut _object {
    let len: i64 = Py_ARRAY_LENGTH__16!() as i64;
    let names: *mut _object = unsafe { PyTuple_New(len) };
    if names.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    for i in 0..len {
        let name: *mut _object =
            unsafe { PyUnicode_FromString(_Py_stdlib_module_names.0[i as usize]) };
        if name.is_null() {
            Py_DECREF!(names);
            return core::ptr::null_mut::<_object>();
        }
        PyTuple_SET_ITEM__2!(names, i, name);
    }
    let set: *mut _object = unsafe {
        PyObject_CallFunction(
            core::ptr::addr_of_mut!(PyFrozenSet_Type) as *mut _object,
            c"(O)".as_ptr(),
            names,
        )
    };
    Py_DECREF!(names);
    set
}

// Pre-initialization support for sys.warnoptions and sys._xoptions
//
// Modern internal code paths:
//   These APIs get called after _Py_InitializeCore and get to use the
//   regular CPython list, dict, and unicode APIs.
//
// Legacy embedding code paths:
//   The multi-phase initialization API isn't public yet, so embedding
//   apps still need to be able configure sys.warnoptions and sys._xoptions
//   before they call Py_Initialize. To support this, we stash copies of
//   the supplied wchar * sequences in linked lists, and then migrate the
//   contents of those lists to the sys module in _PyInitializeCore.
#[repr(C)]
#[derive(Clone, Copy)]
struct _preinit_entry {
    value: *mut i32,
    next: *mut _preinit_entry,
}

static mut _preinit_warnoptions: *mut _preinit_entry = core::ptr::null_mut::<_preinit_entry>();

static mut _preinit_xoptions: *mut _preinit_entry = core::ptr::null_mut::<_preinit_entry>();

fn _alloc_preinit_entry(value: *const i32) -> *mut _preinit_entry {
    // To get this to work, we have to initialize the runtime implicitly
    unsafe { _PyRuntime_Initialize() };
    // Use the default allocator, so we can ensure that it also gets used to
    // destroy the linked list in _clear_preinit_entries.
    let mut node: *mut _preinit_entry =
        (unsafe { _PyMem_DefaultRawCalloc(1u64, core::mem::size_of::<_preinit_entry>() as u64) })
            as *mut _preinit_entry;
    if !node.is_null() {
        unsafe {
            (*node).value = _PyMem_DefaultRawWcsdup(value);
        }
        if unsafe { (*node).value }.is_null() {
            unsafe { _PyMem_DefaultRawFree(node as *mut ()) };
            node = core::ptr::null_mut::<_preinit_entry>();
        }
    }
    node
}

fn _append_preinit_entry(optionlist: *mut *mut _preinit_entry, value: *const i32) -> i32 {
    let new_entry: *mut _preinit_entry = _alloc_preinit_entry(value);
    if new_entry.is_null() {
        return 1i32.wrapping_neg();
    }
    // We maintain the linked list in this order so it's easy to play back
    // the add commands in the same order later on in _Py_InitializeCore
    let mut last_entry: *mut _preinit_entry = unsafe { *optionlist };
    if last_entry.is_null() {
        unsafe {
            *optionlist = new_entry;
        }
    } else {
        while !unsafe { (*last_entry).next }.is_null() {
            last_entry = unsafe { (*last_entry).next };
        }
        unsafe {
            (*last_entry).next = new_entry;
        }
    }
    0
}

fn _clear_preinit_entries(optionlist: *mut *mut _preinit_entry) {
    let mut current: *mut _preinit_entry = unsafe { *optionlist };
    unsafe {
        *optionlist = core::ptr::null_mut::<_preinit_entry>();
    }
    // Deallocate the nodes and their contents using the default allocator
    while !current.is_null() {
        let next: *mut _preinit_entry = unsafe { (*current).next };
        unsafe { _PyMem_DefaultRawFree((*current).value as *mut ()) };
        unsafe { _PyMem_DefaultRawFree(current as *mut ()) };
        current = next;
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PySys_ReadPreinitWarnOptions(
    options: *mut __SlateRecord_initconfig_h_36,
) -> __SlateRecord_initconfig_h_19 {
    let mut __slate_compound_1: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut status: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut entry: *mut _preinit_entry = unsafe { core::mem::zeroed() };
    entry = unsafe { _preinit_warnoptions };
    while !entry.is_null() {
        status = unsafe { PyWideStringList_Append(options, (*entry).value as *const i32) };
        if _PyStatus_EXCEPTION!(status) {
            return status;
        }
        entry = unsafe { (*entry).next };
    }
    _clear_preinit_entries(unsafe { core::ptr::addr_of_mut!(_preinit_warnoptions) });
    return unsafe {
        *{
            __slate_compound_1 = {
                let mut __t0: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                __t0._type = 0u32;
                __t0
            };
            core::ptr::addr_of_mut!(__slate_compound_1)
        }
    };
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PySys_ReadPreinitXOptions(
    config: *mut PyConfig,
) -> __SlateRecord_initconfig_h_19 {
    let mut __slate_compound_1: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut status: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut entry: *mut _preinit_entry = unsafe { core::mem::zeroed() };
    entry = unsafe { _preinit_xoptions };
    while !entry.is_null() {
        status = unsafe {
            PyWideStringList_Append(
                core::ptr::addr_of_mut!((*config).xoptions),
                (*entry).value as *const i32,
            )
        };
        if _PyStatus_EXCEPTION!(status) {
            return status;
        }
        entry = unsafe { (*entry).next };
    }
    _clear_preinit_entries(unsafe { core::ptr::addr_of_mut!(_preinit_xoptions) });
    return unsafe {
        *{
            __slate_compound_1 = {
                let mut __t0: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                __t0._type = 0u32;
                __t0
            };
            core::ptr::addr_of_mut!(__slate_compound_1)
        }
    };
}

fn get_warnoptions(tstate: *mut _ts) -> *mut _object {
    let mut warnoptions: *mut _object = unsafe { core::mem::zeroed() };
    if _PySys_GetOptionalAttr(
        unsafe {
            core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py_warnoptions
                    ._ascii
                    .ob_base
            )
        },
        core::ptr::addr_of_mut!(warnoptions),
    ) < 0
    {
        return core::ptr::null_mut::<_object>();
    }
    let __v0: bool = warnoptions.is_null() || PyList_Check!(warnoptions) == 0;
    if __v0 {
        // PEP432 TODO: we can reach this if warnoptions is NULL in the main
        //  interpreter config. When that happens, we need to properly set
        // the `warnoptions` reference in the main interpreter config as well.
        //
        // For Python 3.7, we shouldn't be able to get here due to the
        // combination of how _PyMainInterpreter_ReadConfig and _PySys_EndInit
        // work, but we expect 3.8+ to make the _PyMainInterpreter_ReadConfig
        // call optional for embedding applications, thus making this
        // reachable again.
        Py_XDECREF!(warnoptions);
        warnoptions = unsafe { PyList_New(0i64) };
        if warnoptions.is_null() {
            return core::ptr::null_mut::<_object>();
        }
        if sys_set_object(
            unsafe { (*tstate).interp },
            unsafe {
                core::ptr::addr_of_mut!(
                    _PyRuntime
                        .static_objects
                        .singletons
                        .strings
                        .identifiers
                        ._py_warnoptions
                        ._ascii
                        .ob_base
                )
            },
            warnoptions,
        ) != 0
        {
            Py_DECREF!(warnoptions);
            return core::ptr::null_mut::<_object>();
        }
    }
    warnoptions
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PySys_ResetWarnOptions() {
    let tstate: *mut _ts = _PyThreadState_GET();
    if tstate.is_null() {
        _clear_preinit_entries(unsafe { core::ptr::addr_of_mut!(_preinit_warnoptions) });
        return;
    }
    let mut warnoptions: *mut _object = unsafe { core::mem::zeroed() };
    if _PySys_GetOptionalAttr(
        unsafe {
            core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py_warnoptions
                    ._ascii
                    .ob_base
            )
        },
        core::ptr::addr_of_mut!(warnoptions),
    ) < 0
    {
        unsafe { PyErr_Clear() };
        return;
    }
    let __v0: bool = !warnoptions.is_null() && PyList_Check!(warnoptions) != 0;
    if __v0 {
        unsafe {
            PyList_SetSlice(
                warnoptions,
                0i64,
                PyList_GET_SIZE!(warnoptions),
                core::ptr::null_mut::<_object>(),
            )
        };
    }
    Py_XDECREF!(warnoptions);
}

fn _PySys_AddWarnOptionWithError(tstate: *mut _ts, option: *mut _object) -> i32 {
    let warnoptions: *mut _object = get_warnoptions(tstate);
    if warnoptions.is_null() {
        return 1i32.wrapping_neg();
    }
    if (unsafe { PyList_Append(warnoptions, option) }) != 0 {
        Py_DECREF!(warnoptions);
        return 1i32.wrapping_neg();
    }
    Py_DECREF!(warnoptions);
    0
}

/// Removed in Python 3.13 API, but kept for the stable ABI
#[unsafe(no_mangle)]
extern "C-unwind" fn PySys_AddWarnOptionUnicode(option: *mut _object) {
    let tstate: *mut _ts = _PyThreadState_GET();
    _Py_EnsureTstateNotNULL!(tstate);
    if _PySys_AddWarnOptionWithError(tstate, option) < 0 {
        // No return value, therefore clear error state if possible
        unsafe { _PyErr_Clear(tstate) };
    }
}

/// Removed in Python 3.13 API, but kept for the stable ABI
#[unsafe(no_mangle)]
extern "C-unwind" fn PySys_AddWarnOption(s: *const i32) {
    let tstate: *mut _ts = _PyThreadState_GET();
    if tstate.is_null() {
        _append_preinit_entry(unsafe { core::ptr::addr_of_mut!(_preinit_warnoptions) }, s);
        return;
    }
    let unicode: *mut _object = unsafe { PyUnicode_FromWideChar(s, 1i32.wrapping_neg() as i64) };
    if unicode.is_null() {
        return;
    }
    PySys_AddWarnOptionUnicode(unicode);
    Py_DECREF!(unicode);
}

/// Removed in Python 3.13 API, but kept for the stable ABI
#[unsafe(no_mangle)]
extern "C-unwind" fn PySys_HasWarnOptions() -> i32 {
    let mut warnoptions: *mut _object = unsafe { core::mem::zeroed() };
    if _PySys_GetOptionalAttr(
        unsafe {
            core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py_warnoptions
                    ._ascii
                    .ob_base
            )
        },
        core::ptr::addr_of_mut!(warnoptions),
    ) < 0
    {
        unsafe { PyErr_Clear() };
        return 0;
    }
    let mut r: i32 = 0;
    let __v0: bool = !warnoptions.is_null() && PyList_Check!(warnoptions) != 0;
    let __v1: bool = __v0 && PyList_GET_SIZE!(warnoptions) > 0;
    r = __v1 as i32;
    Py_XDECREF!(warnoptions);
    r
}

fn get_xoptions(tstate: *mut _ts) -> *mut _object {
    let mut xoptions: *mut _object = unsafe { core::mem::zeroed() };
    if _PySys_GetOptionalAttr(
        unsafe {
            core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py__xoptions
                    ._ascii
                    .ob_base
            )
        },
        core::ptr::addr_of_mut!(xoptions),
    ) < 0
    {
        return core::ptr::null_mut::<_object>();
    }
    let __v0: bool = xoptions.is_null() || PyDict_Check!(xoptions) == 0;
    if __v0 {
        // PEP432 TODO: we can reach this if xoptions is NULL in the main
        //  interpreter config. When that happens, we need to properly set
        // the `xoptions` reference in the main interpreter config as well.
        //
        // For Python 3.7, we shouldn't be able to get here due to the
        // combination of how _PyMainInterpreter_ReadConfig and _PySys_EndInit
        // work, but we expect 3.8+ to make the _PyMainInterpreter_ReadConfig
        // call optional for embedding applications, thus making this
        // reachable again.
        Py_XDECREF!(xoptions);
        xoptions = unsafe { PyDict_New() };
        if xoptions.is_null() {
            return core::ptr::null_mut::<_object>();
        }
        if sys_set_object(
            unsafe { (*tstate).interp },
            unsafe {
                core::ptr::addr_of_mut!(
                    _PyRuntime
                        .static_objects
                        .singletons
                        .strings
                        .identifiers
                        ._py__xoptions
                        ._ascii
                        .ob_base
                )
            },
            xoptions,
        ) != 0
        {
            Py_DECREF!(xoptions);
            return core::ptr::null_mut::<_object>();
        }
    }
    xoptions
}

fn _PySys_AddXOptionWithError(s: *const i32) -> i32 {
    let mut name_end: *const i32 = unsafe { core::mem::zeroed() };
    let mut opts: *mut _object = unsafe { core::mem::zeroed() };
    let mut value: *mut _object = unsafe { core::mem::zeroed() };
    let mut name: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_0: {
            name = core::ptr::null_mut::<_object>();
            value = core::ptr::null_mut::<_object>();
            let tstate: *mut _ts = _PyThreadState_GET();
            opts = get_xoptions(tstate);
            if !opts.is_null() {
                name_end = wcschr(s, 61i32) as *const i32;
                if name_end.is_null() {
                    name = PyUnicode_FromWideChar(s, 1i32.wrapping_neg() as i64);
                    if name.is_null() {
                        break '__join_0;
                    } else {
                        value =
                            Py_NewRef!(_PyObject_CAST__2!(core::ptr::addr_of_mut!(_Py_TrueStruct)));
                    }
                } else {
                    name = PyUnicode_FromWideChar(s, name_end.offset_from(s as *const i32) as i64);
                    if name.is_null() {
                        break '__join_0;
                    } else {
                        value = PyUnicode_FromWideChar(
                            name_end.wrapping_offset(1 as isize),
                            1i32.wrapping_neg() as i64,
                        );
                        if value.is_null() {
                            break '__join_0;
                        }
                    }
                }
                if PyDict_SetItem(opts, name, value) >= 0 {
                    Py_DECREF!(name);
                    Py_DECREF!(value);
                    Py_DECREF!(opts);
                    return 0;
                }
            }
        }
        Py_XDECREF!(name);
        Py_XDECREF!(value);
        Py_XDECREF!(opts);
        return 1i32.wrapping_neg();
    }
}

/// Removed in Python 3.13 API, but kept for the stable ABI
#[unsafe(no_mangle)]
extern "C-unwind" fn PySys_AddXOption(s: *const i32) {
    let tstate: *mut _ts = _PyThreadState_GET();
    if tstate.is_null() {
        _append_preinit_entry(unsafe { core::ptr::addr_of_mut!(_preinit_xoptions) }, s);
        return;
    }
    if _PySys_AddXOptionWithError(s) < 0 {
        // No return value, therefore clear error state if possible
        unsafe { _PyErr_Clear(tstate) };
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PySys_GetXOptions() -> *mut _object {
    let tstate: *mut _ts = _PyThreadState_GET();
    let opts: *mut _object = get_xoptions(tstate);
    Py_XDECREF!(opts);
    opts
}

/// XXX This doc string is too long to be a single string literal in VC++ 5.0.
/// Two literals concatenated works just fine.  If you have a K&R compiler
/// or other abomination that however *does* understand longer strings,
/// get rid of the !!! comment in the middle and the quotes that surround it.
/// concatenating string here
/// end of sys_doc
static sys_doc: __SlateAlign16<[i8; 3571]> = __SlateAlign16([
    84i8, 104i8, 105i8, 115i8, 32i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 32i8, 112i8, 114i8,
    111i8, 118i8, 105i8, 100i8, 101i8, 115i8, 32i8, 97i8, 99i8, 99i8, 101i8, 115i8, 115i8, 32i8,
    116i8, 111i8, 32i8, 115i8, 111i8, 109i8, 101i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8,
    115i8, 32i8, 117i8, 115i8, 101i8, 100i8, 32i8, 111i8, 114i8, 32i8, 109i8, 97i8, 105i8, 110i8,
    116i8, 97i8, 105i8, 110i8, 101i8, 100i8, 32i8, 98i8, 121i8, 32i8, 116i8, 104i8, 101i8, 10i8,
    105i8, 110i8, 116i8, 101i8, 114i8, 112i8, 114i8, 101i8, 116i8, 101i8, 114i8, 32i8, 97i8, 110i8,
    100i8, 32i8, 116i8, 111i8, 32i8, 102i8, 117i8, 110i8, 99i8, 116i8, 105i8, 111i8, 110i8, 115i8,
    32i8, 116i8, 104i8, 97i8, 116i8, 32i8, 105i8, 110i8, 116i8, 101i8, 114i8, 97i8, 99i8, 116i8,
    32i8, 115i8, 116i8, 114i8, 111i8, 110i8, 103i8, 108i8, 121i8, 32i8, 119i8, 105i8, 116i8, 104i8,
    32i8, 116i8, 104i8, 101i8, 32i8, 105i8, 110i8, 116i8, 101i8, 114i8, 112i8, 114i8, 101i8, 116i8,
    101i8, 114i8, 46i8, 10i8, 10i8, 68i8, 121i8, 110i8, 97i8, 109i8, 105i8, 99i8, 32i8, 111i8,
    98i8, 106i8, 101i8, 99i8, 116i8, 115i8, 58i8, 10i8, 10i8, 97i8, 114i8, 103i8, 118i8, 32i8,
    45i8, 45i8, 32i8, 99i8, 111i8, 109i8, 109i8, 97i8, 110i8, 100i8, 32i8, 108i8, 105i8, 110i8,
    101i8, 32i8, 97i8, 114i8, 103i8, 117i8, 109i8, 101i8, 110i8, 116i8, 115i8, 59i8, 32i8, 97i8,
    114i8, 103i8, 118i8, 91i8, 48i8, 93i8, 32i8, 105i8, 115i8, 32i8, 116i8, 104i8, 101i8, 32i8,
    115i8, 99i8, 114i8, 105i8, 112i8, 116i8, 32i8, 112i8, 97i8, 116i8, 104i8, 110i8, 97i8, 109i8,
    101i8, 32i8, 105i8, 102i8, 32i8, 107i8, 110i8, 111i8, 119i8, 110i8, 10i8, 112i8, 97i8, 116i8,
    104i8, 32i8, 45i8, 45i8, 32i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 32i8, 115i8, 101i8,
    97i8, 114i8, 99i8, 104i8, 32i8, 112i8, 97i8, 116i8, 104i8, 59i8, 32i8, 112i8, 97i8, 116i8,
    104i8, 91i8, 48i8, 93i8, 32i8, 105i8, 115i8, 32i8, 116i8, 104i8, 101i8, 32i8, 115i8, 99i8,
    114i8, 105i8, 112i8, 116i8, 32i8, 100i8, 105i8, 114i8, 101i8, 99i8, 116i8, 111i8, 114i8, 121i8,
    44i8, 32i8, 101i8, 108i8, 115i8, 101i8, 32i8, 39i8, 39i8, 10i8, 109i8, 111i8, 100i8, 117i8,
    108i8, 101i8, 115i8, 32i8, 45i8, 45i8, 32i8, 100i8, 105i8, 99i8, 116i8, 105i8, 111i8, 110i8,
    97i8, 114i8, 121i8, 32i8, 111i8, 102i8, 32i8, 108i8, 111i8, 97i8, 100i8, 101i8, 100i8, 32i8,
    109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 115i8, 10i8, 10i8, 100i8, 105i8, 115i8, 112i8, 108i8,
    97i8, 121i8, 104i8, 111i8, 111i8, 107i8, 32i8, 45i8, 45i8, 32i8, 99i8, 97i8, 108i8, 108i8,
    101i8, 100i8, 32i8, 116i8, 111i8, 32i8, 115i8, 104i8, 111i8, 119i8, 32i8, 114i8, 101i8, 115i8,
    117i8, 108i8, 116i8, 115i8, 32i8, 105i8, 110i8, 32i8, 97i8, 110i8, 32i8, 105i8, 110i8, 116i8,
    101i8, 114i8, 97i8, 99i8, 116i8, 105i8, 118i8, 101i8, 32i8, 115i8, 101i8, 115i8, 115i8, 105i8,
    111i8, 110i8, 10i8, 101i8, 120i8, 99i8, 101i8, 112i8, 116i8, 104i8, 111i8, 111i8, 107i8, 32i8,
    45i8, 45i8, 32i8, 99i8, 97i8, 108i8, 108i8, 101i8, 100i8, 32i8, 116i8, 111i8, 32i8, 104i8,
    97i8, 110i8, 100i8, 108i8, 101i8, 32i8, 97i8, 110i8, 121i8, 32i8, 117i8, 110i8, 99i8, 97i8,
    117i8, 103i8, 104i8, 116i8, 32i8, 101i8, 120i8, 99i8, 101i8, 112i8, 116i8, 105i8, 111i8, 110i8,
    32i8, 111i8, 116i8, 104i8, 101i8, 114i8, 32i8, 116i8, 104i8, 97i8, 110i8, 32i8, 83i8, 121i8,
    115i8, 116i8, 101i8, 109i8, 69i8, 120i8, 105i8, 116i8, 10i8, 32i8, 32i8, 84i8, 111i8, 32i8,
    99i8, 117i8, 115i8, 116i8, 111i8, 109i8, 105i8, 122i8, 101i8, 32i8, 112i8, 114i8, 105i8, 110i8,
    116i8, 105i8, 110i8, 103i8, 32i8, 105i8, 110i8, 32i8, 97i8, 110i8, 32i8, 105i8, 110i8, 116i8,
    101i8, 114i8, 97i8, 99i8, 116i8, 105i8, 118i8, 101i8, 32i8, 115i8, 101i8, 115i8, 115i8, 105i8,
    111i8, 110i8, 32i8, 111i8, 114i8, 32i8, 116i8, 111i8, 32i8, 105i8, 110i8, 115i8, 116i8, 97i8,
    108i8, 108i8, 32i8, 97i8, 32i8, 99i8, 117i8, 115i8, 116i8, 111i8, 109i8, 10i8, 32i8, 32i8,
    116i8, 111i8, 112i8, 45i8, 108i8, 101i8, 118i8, 101i8, 108i8, 32i8, 101i8, 120i8, 99i8, 101i8,
    112i8, 116i8, 105i8, 111i8, 110i8, 32i8, 104i8, 97i8, 110i8, 100i8, 108i8, 101i8, 114i8, 44i8,
    32i8, 97i8, 115i8, 115i8, 105i8, 103i8, 110i8, 32i8, 111i8, 116i8, 104i8, 101i8, 114i8, 32i8,
    102i8, 117i8, 110i8, 99i8, 116i8, 105i8, 111i8, 110i8, 115i8, 32i8, 116i8, 111i8, 32i8, 114i8,
    101i8, 112i8, 108i8, 97i8, 99i8, 101i8, 32i8, 116i8, 104i8, 101i8, 115i8, 101i8, 46i8, 10i8,
    10i8, 115i8, 116i8, 100i8, 105i8, 110i8, 32i8, 45i8, 45i8, 32i8, 115i8, 116i8, 97i8, 110i8,
    100i8, 97i8, 114i8, 100i8, 32i8, 105i8, 110i8, 112i8, 117i8, 116i8, 32i8, 102i8, 105i8, 108i8,
    101i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 59i8, 32i8, 117i8, 115i8, 101i8, 100i8,
    32i8, 98i8, 121i8, 32i8, 105i8, 110i8, 112i8, 117i8, 116i8, 40i8, 41i8, 10i8, 115i8, 116i8,
    100i8, 111i8, 117i8, 116i8, 32i8, 45i8, 45i8, 32i8, 115i8, 116i8, 97i8, 110i8, 100i8, 97i8,
    114i8, 100i8, 32i8, 111i8, 117i8, 116i8, 112i8, 117i8, 116i8, 32i8, 102i8, 105i8, 108i8, 101i8,
    32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 59i8, 32i8, 117i8, 115i8, 101i8, 100i8, 32i8,
    98i8, 121i8, 32i8, 112i8, 114i8, 105i8, 110i8, 116i8, 40i8, 41i8, 10i8, 115i8, 116i8, 100i8,
    101i8, 114i8, 114i8, 32i8, 45i8, 45i8, 32i8, 115i8, 116i8, 97i8, 110i8, 100i8, 97i8, 114i8,
    100i8, 32i8, 101i8, 114i8, 114i8, 111i8, 114i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8,
    59i8, 32i8, 117i8, 115i8, 101i8, 100i8, 32i8, 102i8, 111i8, 114i8, 32i8, 101i8, 114i8, 114i8,
    111i8, 114i8, 32i8, 109i8, 101i8, 115i8, 115i8, 97i8, 103i8, 101i8, 115i8, 10i8, 32i8, 32i8,
    66i8, 121i8, 32i8, 97i8, 115i8, 115i8, 105i8, 103i8, 110i8, 105i8, 110i8, 103i8, 32i8, 111i8,
    116i8, 104i8, 101i8, 114i8, 32i8, 102i8, 105i8, 108i8, 101i8, 32i8, 111i8, 98i8, 106i8, 101i8,
    99i8, 116i8, 115i8, 32i8, 40i8, 111i8, 114i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8,
    115i8, 32i8, 116i8, 104i8, 97i8, 116i8, 32i8, 98i8, 101i8, 104i8, 97i8, 118i8, 101i8, 32i8,
    108i8, 105i8, 107i8, 101i8, 32i8, 102i8, 105i8, 108i8, 101i8, 115i8, 41i8, 10i8, 32i8, 32i8,
    116i8, 111i8, 32i8, 116i8, 104i8, 101i8, 115i8, 101i8, 44i8, 32i8, 105i8, 116i8, 32i8, 105i8,
    115i8, 32i8, 112i8, 111i8, 115i8, 115i8, 105i8, 98i8, 108i8, 101i8, 32i8, 116i8, 111i8, 32i8,
    114i8, 101i8, 100i8, 105i8, 114i8, 101i8, 99i8, 116i8, 32i8, 97i8, 108i8, 108i8, 32i8, 111i8,
    102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 105i8, 110i8, 116i8, 101i8, 114i8, 112i8, 114i8, 101i8,
    116i8, 101i8, 114i8, 39i8, 115i8, 32i8, 73i8, 47i8, 79i8, 46i8, 10i8, 10i8, 108i8, 97i8, 115i8,
    116i8, 95i8, 101i8, 120i8, 99i8, 32i8, 45i8, 32i8, 116i8, 104i8, 101i8, 32i8, 108i8, 97i8,
    115i8, 116i8, 32i8, 117i8, 110i8, 99i8, 97i8, 117i8, 103i8, 104i8, 116i8, 32i8, 101i8, 120i8,
    99i8, 101i8, 112i8, 116i8, 105i8, 111i8, 110i8, 10i8, 32i8, 32i8, 79i8, 110i8, 108i8, 121i8,
    32i8, 97i8, 118i8, 97i8, 105i8, 108i8, 97i8, 98i8, 108i8, 101i8, 32i8, 105i8, 110i8, 32i8,
    97i8, 110i8, 32i8, 105i8, 110i8, 116i8, 101i8, 114i8, 97i8, 99i8, 116i8, 105i8, 118i8, 101i8,
    32i8, 115i8, 101i8, 115i8, 115i8, 105i8, 111i8, 110i8, 32i8, 97i8, 102i8, 116i8, 101i8, 114i8,
    32i8, 97i8, 10i8, 32i8, 32i8, 116i8, 114i8, 97i8, 99i8, 101i8, 98i8, 97i8, 99i8, 107i8, 32i8,
    104i8, 97i8, 115i8, 32i8, 98i8, 101i8, 101i8, 110i8, 32i8, 112i8, 114i8, 105i8, 110i8, 116i8,
    101i8, 100i8, 46i8, 10i8, 108i8, 97i8, 115i8, 116i8, 95i8, 116i8, 121i8, 112i8, 101i8, 32i8,
    45i8, 45i8, 32i8, 116i8, 121i8, 112i8, 101i8, 32i8, 111i8, 102i8, 32i8, 108i8, 97i8, 115i8,
    116i8, 32i8, 117i8, 110i8, 99i8, 97i8, 117i8, 103i8, 104i8, 116i8, 32i8, 101i8, 120i8, 99i8,
    101i8, 112i8, 116i8, 105i8, 111i8, 110i8, 10i8, 108i8, 97i8, 115i8, 116i8, 95i8, 118i8, 97i8,
    108i8, 117i8, 101i8, 32i8, 45i8, 45i8, 32i8, 118i8, 97i8, 108i8, 117i8, 101i8, 32i8, 111i8,
    102i8, 32i8, 108i8, 97i8, 115i8, 116i8, 32i8, 117i8, 110i8, 99i8, 97i8, 117i8, 103i8, 104i8,
    116i8, 32i8, 101i8, 120i8, 99i8, 101i8, 112i8, 116i8, 105i8, 111i8, 110i8, 10i8, 108i8, 97i8,
    115i8, 116i8, 95i8, 116i8, 114i8, 97i8, 99i8, 101i8, 98i8, 97i8, 99i8, 107i8, 32i8, 45i8, 45i8,
    32i8, 116i8, 114i8, 97i8, 99i8, 101i8, 98i8, 97i8, 99i8, 107i8, 32i8, 111i8, 102i8, 32i8,
    108i8, 97i8, 115i8, 116i8, 32i8, 117i8, 110i8, 99i8, 97i8, 117i8, 103i8, 104i8, 116i8, 32i8,
    101i8, 120i8, 99i8, 101i8, 112i8, 116i8, 105i8, 111i8, 110i8, 10i8, 32i8, 32i8, 84i8, 104i8,
    101i8, 115i8, 101i8, 32i8, 116i8, 104i8, 114i8, 101i8, 101i8, 32i8, 97i8, 114i8, 101i8, 32i8,
    116i8, 104i8, 101i8, 32i8, 40i8, 100i8, 101i8, 112i8, 114i8, 101i8, 99i8, 97i8, 116i8, 101i8,
    100i8, 41i8, 32i8, 108i8, 101i8, 103i8, 97i8, 99i8, 121i8, 32i8, 114i8, 101i8, 112i8, 114i8,
    101i8, 115i8, 101i8, 110i8, 116i8, 97i8, 116i8, 105i8, 111i8, 110i8, 32i8, 111i8, 102i8, 32i8,
    108i8, 97i8, 115i8, 116i8, 95i8, 101i8, 120i8, 99i8, 46i8, 10i8, 10i8, 83i8, 116i8, 97i8,
    116i8, 105i8, 99i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 115i8, 58i8, 10i8, 10i8,
    98i8, 117i8, 105i8, 108i8, 116i8, 105i8, 110i8, 95i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8,
    95i8, 110i8, 97i8, 109i8, 101i8, 115i8, 32i8, 45i8, 45i8, 32i8, 116i8, 117i8, 112i8, 108i8,
    101i8, 32i8, 111i8, 102i8, 32i8, 109i8, 111i8, 100i8, 117i8, 108i8, 101i8, 32i8, 110i8, 97i8,
    109i8, 101i8, 115i8, 32i8, 98i8, 117i8, 105i8, 108i8, 116i8, 32i8, 105i8, 110i8, 116i8, 111i8,
    32i8, 116i8, 104i8, 105i8, 115i8, 32i8, 105i8, 110i8, 116i8, 101i8, 114i8, 112i8, 114i8, 101i8,
    116i8, 101i8, 114i8, 10i8, 99i8, 111i8, 112i8, 121i8, 114i8, 105i8, 103i8, 104i8, 116i8, 32i8,
    45i8, 45i8, 32i8, 99i8, 111i8, 112i8, 121i8, 114i8, 105i8, 103i8, 104i8, 116i8, 32i8, 110i8,
    111i8, 116i8, 105i8, 99i8, 101i8, 32i8, 112i8, 101i8, 114i8, 116i8, 97i8, 105i8, 110i8, 105i8,
    110i8, 103i8, 32i8, 116i8, 111i8, 32i8, 116i8, 104i8, 105i8, 115i8, 32i8, 105i8, 110i8, 116i8,
    101i8, 114i8, 112i8, 114i8, 101i8, 116i8, 101i8, 114i8, 10i8, 101i8, 120i8, 101i8, 99i8, 95i8,
    112i8, 114i8, 101i8, 102i8, 105i8, 120i8, 32i8, 45i8, 45i8, 32i8, 112i8, 114i8, 101i8, 102i8,
    105i8, 120i8, 32i8, 117i8, 115i8, 101i8, 100i8, 32i8, 116i8, 111i8, 32i8, 102i8, 105i8, 110i8,
    100i8, 32i8, 116i8, 104i8, 101i8, 32i8, 109i8, 97i8, 99i8, 104i8, 105i8, 110i8, 101i8, 45i8,
    115i8, 112i8, 101i8, 99i8, 105i8, 102i8, 105i8, 99i8, 32i8, 80i8, 121i8, 116i8, 104i8, 111i8,
    110i8, 32i8, 108i8, 105i8, 98i8, 114i8, 97i8, 114i8, 121i8, 10i8, 101i8, 120i8, 101i8, 99i8,
    117i8, 116i8, 97i8, 98i8, 108i8, 101i8, 32i8, 45i8, 45i8, 32i8, 97i8, 98i8, 115i8, 111i8,
    108i8, 117i8, 116i8, 101i8, 32i8, 112i8, 97i8, 116i8, 104i8, 32i8, 111i8, 102i8, 32i8, 116i8,
    104i8, 101i8, 32i8, 101i8, 120i8, 101i8, 99i8, 117i8, 116i8, 97i8, 98i8, 108i8, 101i8, 32i8,
    98i8, 105i8, 110i8, 97i8, 114i8, 121i8, 32i8, 111i8, 102i8, 32i8, 116i8, 104i8, 101i8, 32i8,
    80i8, 121i8, 116i8, 104i8, 111i8, 110i8, 32i8, 105i8, 110i8, 116i8, 101i8, 114i8, 112i8, 114i8,
    101i8, 116i8, 101i8, 114i8, 10i8, 102i8, 108i8, 111i8, 97i8, 116i8, 95i8, 105i8, 110i8, 102i8,
    111i8, 32i8, 45i8, 45i8, 32i8, 97i8, 32i8, 110i8, 97i8, 109i8, 101i8, 100i8, 32i8, 116i8,
    117i8, 112i8, 108i8, 101i8, 32i8, 119i8, 105i8, 116i8, 104i8, 32i8, 105i8, 110i8, 102i8, 111i8,
    114i8, 109i8, 97i8, 116i8, 105i8, 111i8, 110i8, 32i8, 97i8, 98i8, 111i8, 117i8, 116i8, 32i8,
    116i8, 104i8, 101i8, 32i8, 102i8, 108i8, 111i8, 97i8, 116i8, 32i8, 105i8, 109i8, 112i8, 108i8,
    101i8, 109i8, 101i8, 110i8, 116i8, 97i8, 116i8, 105i8, 111i8, 110i8, 46i8, 10i8, 102i8, 108i8,
    111i8, 97i8, 116i8, 95i8, 114i8, 101i8, 112i8, 114i8, 95i8, 115i8, 116i8, 121i8, 108i8, 101i8,
    32i8, 45i8, 45i8, 32i8, 115i8, 116i8, 114i8, 105i8, 110i8, 103i8, 32i8, 105i8, 110i8, 100i8,
    105i8, 99i8, 97i8, 116i8, 105i8, 110i8, 103i8, 32i8, 116i8, 104i8, 101i8, 32i8, 115i8, 116i8,
    121i8, 108i8, 101i8, 32i8, 111i8, 102i8, 32i8, 114i8, 101i8, 112i8, 114i8, 40i8, 41i8, 32i8,
    111i8, 117i8, 116i8, 112i8, 117i8, 116i8, 32i8, 102i8, 111i8, 114i8, 32i8, 102i8, 108i8, 111i8,
    97i8, 116i8, 115i8, 10i8, 104i8, 97i8, 115i8, 104i8, 95i8, 105i8, 110i8, 102i8, 111i8, 32i8,
    45i8, 45i8, 32i8, 97i8, 32i8, 110i8, 97i8, 109i8, 101i8, 100i8, 32i8, 116i8, 117i8, 112i8,
    108i8, 101i8, 32i8, 119i8, 105i8, 116i8, 104i8, 32i8, 105i8, 110i8, 102i8, 111i8, 114i8, 109i8,
    97i8, 116i8, 105i8, 111i8, 110i8, 32i8, 97i8, 98i8, 111i8, 117i8, 116i8, 32i8, 116i8, 104i8,
    101i8, 32i8, 104i8, 97i8, 115i8, 104i8, 32i8, 97i8, 108i8, 103i8, 111i8, 114i8, 105i8, 116i8,
    104i8, 109i8, 46i8, 10i8, 104i8, 101i8, 120i8, 118i8, 101i8, 114i8, 115i8, 105i8, 111i8, 110i8,
    32i8, 45i8, 45i8, 32i8, 118i8, 101i8, 114i8, 115i8, 105i8, 111i8, 110i8, 32i8, 105i8, 110i8,
    102i8, 111i8, 114i8, 109i8, 97i8, 116i8, 105i8, 111i8, 110i8, 32i8, 101i8, 110i8, 99i8, 111i8,
    100i8, 101i8, 100i8, 32i8, 97i8, 115i8, 32i8, 97i8, 32i8, 115i8, 105i8, 110i8, 103i8, 108i8,
    101i8, 32i8, 105i8, 110i8, 116i8, 101i8, 103i8, 101i8, 114i8, 10i8, 105i8, 109i8, 112i8, 108i8,
    101i8, 109i8, 101i8, 110i8, 116i8, 97i8, 116i8, 105i8, 111i8, 110i8, 32i8, 45i8, 45i8, 32i8,
    80i8, 121i8, 116i8, 104i8, 111i8, 110i8, 32i8, 105i8, 109i8, 112i8, 108i8, 101i8, 109i8, 101i8,
    110i8, 116i8, 97i8, 116i8, 105i8, 111i8, 110i8, 32i8, 105i8, 110i8, 102i8, 111i8, 114i8, 109i8,
    97i8, 116i8, 105i8, 111i8, 110i8, 46i8, 10i8, 105i8, 110i8, 116i8, 95i8, 105i8, 110i8, 102i8,
    111i8, 32i8, 45i8, 45i8, 32i8, 97i8, 32i8, 110i8, 97i8, 109i8, 101i8, 100i8, 32i8, 116i8,
    117i8, 112i8, 108i8, 101i8, 32i8, 119i8, 105i8, 116i8, 104i8, 32i8, 105i8, 110i8, 102i8, 111i8,
    114i8, 109i8, 97i8, 116i8, 105i8, 111i8, 110i8, 32i8, 97i8, 98i8, 111i8, 117i8, 116i8, 32i8,
    116i8, 104i8, 101i8, 32i8, 105i8, 110i8, 116i8, 32i8, 105i8, 109i8, 112i8, 108i8, 101i8, 109i8,
    101i8, 110i8, 116i8, 97i8, 116i8, 105i8, 111i8, 110i8, 46i8, 10i8, 109i8, 97i8, 120i8, 115i8,
    105i8, 122i8, 101i8, 32i8, 45i8, 45i8, 32i8, 116i8, 104i8, 101i8, 32i8, 108i8, 97i8, 114i8,
    103i8, 101i8, 115i8, 116i8, 32i8, 115i8, 117i8, 112i8, 112i8, 111i8, 114i8, 116i8, 101i8,
    100i8, 32i8, 108i8, 101i8, 110i8, 103i8, 116i8, 104i8, 32i8, 111i8, 102i8, 32i8, 99i8, 111i8,
    110i8, 116i8, 97i8, 105i8, 110i8, 101i8, 114i8, 115i8, 46i8, 10i8, 109i8, 97i8, 120i8, 117i8,
    110i8, 105i8, 99i8, 111i8, 100i8, 101i8, 32i8, 45i8, 45i8, 32i8, 116i8, 104i8, 101i8, 32i8,
    118i8, 97i8, 108i8, 117i8, 101i8, 32i8, 111i8, 102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 108i8,
    97i8, 114i8, 103i8, 101i8, 115i8, 116i8, 32i8, 85i8, 110i8, 105i8, 99i8, 111i8, 100i8, 101i8,
    32i8, 99i8, 111i8, 100i8, 101i8, 32i8, 112i8, 111i8, 105i8, 110i8, 116i8, 10i8, 112i8, 108i8,
    97i8, 116i8, 102i8, 111i8, 114i8, 109i8, 32i8, 45i8, 45i8, 32i8, 112i8, 108i8, 97i8, 116i8,
    102i8, 111i8, 114i8, 109i8, 32i8, 105i8, 100i8, 101i8, 110i8, 116i8, 105i8, 102i8, 105i8,
    101i8, 114i8, 10i8, 112i8, 114i8, 101i8, 102i8, 105i8, 120i8, 32i8, 45i8, 45i8, 32i8, 112i8,
    114i8, 101i8, 102i8, 105i8, 120i8, 32i8, 117i8, 115i8, 101i8, 100i8, 32i8, 116i8, 111i8, 32i8,
    102i8, 105i8, 110i8, 100i8, 32i8, 116i8, 104i8, 101i8, 32i8, 80i8, 121i8, 116i8, 104i8, 111i8,
    110i8, 32i8, 108i8, 105i8, 98i8, 114i8, 97i8, 114i8, 121i8, 10i8, 116i8, 104i8, 114i8, 101i8,
    97i8, 100i8, 95i8, 105i8, 110i8, 102i8, 111i8, 32i8, 45i8, 45i8, 32i8, 97i8, 32i8, 110i8, 97i8,
    109i8, 101i8, 100i8, 32i8, 116i8, 117i8, 112i8, 108i8, 101i8, 32i8, 119i8, 105i8, 116i8, 104i8,
    32i8, 105i8, 110i8, 102i8, 111i8, 114i8, 109i8, 97i8, 116i8, 105i8, 111i8, 110i8, 32i8, 97i8,
    98i8, 111i8, 117i8, 116i8, 32i8, 116i8, 104i8, 101i8, 32i8, 116i8, 104i8, 114i8, 101i8, 97i8,
    100i8, 32i8, 105i8, 109i8, 112i8, 108i8, 101i8, 109i8, 101i8, 110i8, 116i8, 97i8, 116i8, 105i8,
    111i8, 110i8, 46i8, 10i8, 118i8, 101i8, 114i8, 115i8, 105i8, 111i8, 110i8, 32i8, 45i8, 45i8,
    32i8, 116i8, 104i8, 101i8, 32i8, 118i8, 101i8, 114i8, 115i8, 105i8, 111i8, 110i8, 32i8, 111i8,
    102i8, 32i8, 116i8, 104i8, 105i8, 115i8, 32i8, 105i8, 110i8, 116i8, 101i8, 114i8, 112i8, 114i8,
    101i8, 116i8, 101i8, 114i8, 32i8, 97i8, 115i8, 32i8, 97i8, 32i8, 115i8, 116i8, 114i8, 105i8,
    110i8, 103i8, 10i8, 118i8, 101i8, 114i8, 115i8, 105i8, 111i8, 110i8, 95i8, 105i8, 110i8, 102i8,
    111i8, 32i8, 45i8, 45i8, 32i8, 118i8, 101i8, 114i8, 115i8, 105i8, 111i8, 110i8, 32i8, 105i8,
    110i8, 102i8, 111i8, 114i8, 109i8, 97i8, 116i8, 105i8, 111i8, 110i8, 32i8, 97i8, 115i8, 32i8,
    97i8, 32i8, 110i8, 97i8, 109i8, 101i8, 100i8, 32i8, 116i8, 117i8, 112i8, 108i8, 101i8, 10i8,
    95i8, 95i8, 115i8, 116i8, 100i8, 105i8, 110i8, 95i8, 95i8, 32i8, 45i8, 45i8, 32i8, 116i8,
    104i8, 101i8, 32i8, 111i8, 114i8, 105i8, 103i8, 105i8, 110i8, 97i8, 108i8, 32i8, 115i8, 116i8,
    100i8, 105i8, 110i8, 59i8, 32i8, 100i8, 111i8, 110i8, 39i8, 116i8, 32i8, 116i8, 111i8, 117i8,
    99i8, 104i8, 33i8, 10i8, 95i8, 95i8, 115i8, 116i8, 100i8, 111i8, 117i8, 116i8, 95i8, 95i8,
    32i8, 45i8, 45i8, 32i8, 116i8, 104i8, 101i8, 32i8, 111i8, 114i8, 105i8, 103i8, 105i8, 110i8,
    97i8, 108i8, 32i8, 115i8, 116i8, 100i8, 111i8, 117i8, 116i8, 59i8, 32i8, 100i8, 111i8, 110i8,
    39i8, 116i8, 32i8, 116i8, 111i8, 117i8, 99i8, 104i8, 33i8, 10i8, 95i8, 95i8, 115i8, 116i8,
    100i8, 101i8, 114i8, 114i8, 95i8, 95i8, 32i8, 45i8, 45i8, 32i8, 116i8, 104i8, 101i8, 32i8,
    111i8, 114i8, 105i8, 103i8, 105i8, 110i8, 97i8, 108i8, 32i8, 115i8, 116i8, 100i8, 101i8, 114i8,
    114i8, 59i8, 32i8, 100i8, 111i8, 110i8, 39i8, 116i8, 32i8, 116i8, 111i8, 117i8, 99i8, 104i8,
    33i8, 10i8, 95i8, 95i8, 100i8, 105i8, 115i8, 112i8, 108i8, 97i8, 121i8, 104i8, 111i8, 111i8,
    107i8, 95i8, 95i8, 32i8, 45i8, 45i8, 32i8, 116i8, 104i8, 101i8, 32i8, 111i8, 114i8, 105i8,
    103i8, 105i8, 110i8, 97i8, 108i8, 32i8, 100i8, 105i8, 115i8, 112i8, 108i8, 97i8, 121i8, 104i8,
    111i8, 111i8, 107i8, 59i8, 32i8, 100i8, 111i8, 110i8, 39i8, 116i8, 32i8, 116i8, 111i8, 117i8,
    99i8, 104i8, 33i8, 10i8, 95i8, 95i8, 101i8, 120i8, 99i8, 101i8, 112i8, 116i8, 104i8, 111i8,
    111i8, 107i8, 95i8, 95i8, 32i8, 45i8, 45i8, 32i8, 116i8, 104i8, 101i8, 32i8, 111i8, 114i8,
    105i8, 103i8, 105i8, 110i8, 97i8, 108i8, 32i8, 101i8, 120i8, 99i8, 101i8, 112i8, 116i8, 104i8,
    111i8, 111i8, 107i8, 59i8, 32i8, 100i8, 111i8, 110i8, 39i8, 116i8, 32i8, 116i8, 111i8, 117i8,
    99i8, 104i8, 33i8, 10i8, 10i8, 70i8, 117i8, 110i8, 99i8, 116i8, 105i8, 111i8, 110i8, 115i8,
    58i8, 10i8, 10i8, 100i8, 105i8, 115i8, 112i8, 108i8, 97i8, 121i8, 104i8, 111i8, 111i8, 107i8,
    40i8, 41i8, 32i8, 45i8, 45i8, 32i8, 112i8, 114i8, 105i8, 110i8, 116i8, 32i8, 97i8, 110i8, 32i8,
    111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 32i8, 116i8, 111i8, 32i8, 116i8, 104i8, 101i8, 32i8,
    115i8, 99i8, 114i8, 101i8, 101i8, 110i8, 44i8, 32i8, 97i8, 110i8, 100i8, 32i8, 115i8, 97i8,
    118i8, 101i8, 32i8, 105i8, 116i8, 32i8, 105i8, 110i8, 32i8, 98i8, 117i8, 105i8, 108i8, 116i8,
    105i8, 110i8, 115i8, 46i8, 95i8, 10i8, 101i8, 120i8, 99i8, 101i8, 112i8, 116i8, 104i8, 111i8,
    111i8, 107i8, 40i8, 41i8, 32i8, 45i8, 45i8, 32i8, 112i8, 114i8, 105i8, 110i8, 116i8, 32i8,
    97i8, 110i8, 32i8, 101i8, 120i8, 99i8, 101i8, 112i8, 116i8, 105i8, 111i8, 110i8, 32i8, 97i8,
    110i8, 100i8, 32i8, 105i8, 116i8, 115i8, 32i8, 116i8, 114i8, 97i8, 99i8, 101i8, 98i8, 97i8,
    99i8, 107i8, 32i8, 116i8, 111i8, 32i8, 115i8, 121i8, 115i8, 46i8, 115i8, 116i8, 100i8, 101i8,
    114i8, 114i8, 10i8, 101i8, 120i8, 99i8, 101i8, 112i8, 116i8, 105i8, 111i8, 110i8, 40i8, 41i8,
    32i8, 45i8, 45i8, 32i8, 114i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 116i8, 104i8, 101i8,
    32i8, 99i8, 117i8, 114i8, 114i8, 101i8, 110i8, 116i8, 32i8, 116i8, 104i8, 114i8, 101i8, 97i8,
    100i8, 39i8, 115i8, 32i8, 97i8, 99i8, 116i8, 105i8, 118i8, 101i8, 32i8, 101i8, 120i8, 99i8,
    101i8, 112i8, 116i8, 105i8, 111i8, 110i8, 10i8, 101i8, 120i8, 99i8, 95i8, 105i8, 110i8, 102i8,
    111i8, 40i8, 41i8, 32i8, 45i8, 45i8, 32i8, 114i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8,
    105i8, 110i8, 102i8, 111i8, 114i8, 109i8, 97i8, 116i8, 105i8, 111i8, 110i8, 32i8, 97i8, 98i8,
    111i8, 117i8, 116i8, 32i8, 116i8, 104i8, 101i8, 32i8, 99i8, 117i8, 114i8, 114i8, 101i8, 110i8,
    116i8, 32i8, 116i8, 104i8, 114i8, 101i8, 97i8, 100i8, 39i8, 115i8, 32i8, 97i8, 99i8, 116i8,
    105i8, 118i8, 101i8, 32i8, 101i8, 120i8, 99i8, 101i8, 112i8, 116i8, 105i8, 111i8, 110i8, 10i8,
    101i8, 120i8, 105i8, 116i8, 40i8, 41i8, 32i8, 45i8, 45i8, 32i8, 101i8, 120i8, 105i8, 116i8,
    32i8, 116i8, 104i8, 101i8, 32i8, 105i8, 110i8, 116i8, 101i8, 114i8, 112i8, 114i8, 101i8, 116i8,
    101i8, 114i8, 32i8, 98i8, 121i8, 32i8, 114i8, 97i8, 105i8, 115i8, 105i8, 110i8, 103i8, 32i8,
    83i8, 121i8, 115i8, 116i8, 101i8, 109i8, 69i8, 120i8, 105i8, 116i8, 10i8, 103i8, 101i8, 116i8,
    100i8, 108i8, 111i8, 112i8, 101i8, 110i8, 102i8, 108i8, 97i8, 103i8, 115i8, 40i8, 41i8, 32i8,
    45i8, 45i8, 32i8, 114i8, 101i8, 116i8, 117i8, 114i8, 110i8, 115i8, 32i8, 102i8, 108i8, 97i8,
    103i8, 115i8, 32i8, 116i8, 111i8, 32i8, 98i8, 101i8, 32i8, 117i8, 115i8, 101i8, 100i8, 32i8,
    102i8, 111i8, 114i8, 32i8, 100i8, 108i8, 111i8, 112i8, 101i8, 110i8, 40i8, 41i8, 32i8, 99i8,
    97i8, 108i8, 108i8, 115i8, 10i8, 103i8, 101i8, 116i8, 112i8, 114i8, 111i8, 102i8, 105i8, 108i8,
    101i8, 40i8, 41i8, 32i8, 45i8, 45i8, 32i8, 103i8, 101i8, 116i8, 32i8, 116i8, 104i8, 101i8,
    32i8, 103i8, 108i8, 111i8, 98i8, 97i8, 108i8, 32i8, 112i8, 114i8, 111i8, 102i8, 105i8, 108i8,
    105i8, 110i8, 103i8, 32i8, 102i8, 117i8, 110i8, 99i8, 116i8, 105i8, 111i8, 110i8, 10i8, 103i8,
    101i8, 116i8, 114i8, 101i8, 102i8, 99i8, 111i8, 117i8, 110i8, 116i8, 40i8, 41i8, 32i8, 45i8,
    45i8, 32i8, 114i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 114i8,
    101i8, 102i8, 101i8, 114i8, 101i8, 110i8, 99i8, 101i8, 32i8, 99i8, 111i8, 117i8, 110i8, 116i8,
    32i8, 102i8, 111i8, 114i8, 32i8, 97i8, 110i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8,
    32i8, 40i8, 112i8, 108i8, 117i8, 115i8, 32i8, 111i8, 110i8, 101i8, 32i8, 58i8, 45i8, 41i8,
    10i8, 103i8, 101i8, 116i8, 114i8, 101i8, 99i8, 117i8, 114i8, 115i8, 105i8, 111i8, 110i8, 108i8,
    105i8, 109i8, 105i8, 116i8, 40i8, 41i8, 32i8, 45i8, 45i8, 32i8, 114i8, 101i8, 116i8, 117i8,
    114i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 109i8, 97i8, 120i8, 32i8, 114i8, 101i8, 99i8,
    117i8, 114i8, 115i8, 105i8, 111i8, 110i8, 32i8, 100i8, 101i8, 112i8, 116i8, 104i8, 32i8, 102i8,
    111i8, 114i8, 32i8, 116i8, 104i8, 101i8, 32i8, 105i8, 110i8, 116i8, 101i8, 114i8, 112i8, 114i8,
    101i8, 116i8, 101i8, 114i8, 10i8, 103i8, 101i8, 116i8, 115i8, 105i8, 122i8, 101i8, 111i8,
    102i8, 40i8, 41i8, 32i8, 45i8, 45i8, 32i8, 114i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8,
    116i8, 104i8, 101i8, 32i8, 115i8, 105i8, 122i8, 101i8, 32i8, 111i8, 102i8, 32i8, 97i8, 110i8,
    32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 32i8, 105i8, 110i8, 32i8, 98i8, 121i8, 116i8,
    101i8, 115i8, 10i8, 103i8, 101i8, 116i8, 116i8, 114i8, 97i8, 99i8, 101i8, 40i8, 41i8, 32i8,
    45i8, 45i8, 32i8, 103i8, 101i8, 116i8, 32i8, 116i8, 104i8, 101i8, 32i8, 103i8, 108i8, 111i8,
    98i8, 97i8, 108i8, 32i8, 100i8, 101i8, 98i8, 117i8, 103i8, 32i8, 116i8, 114i8, 97i8, 99i8,
    105i8, 110i8, 103i8, 32i8, 102i8, 117i8, 110i8, 99i8, 116i8, 105i8, 111i8, 110i8, 10i8, 115i8,
    101i8, 116i8, 100i8, 108i8, 111i8, 112i8, 101i8, 110i8, 102i8, 108i8, 97i8, 103i8, 115i8, 40i8,
    41i8, 32i8, 45i8, 45i8, 32i8, 115i8, 101i8, 116i8, 32i8, 116i8, 104i8, 101i8, 32i8, 102i8,
    108i8, 97i8, 103i8, 115i8, 32i8, 116i8, 111i8, 32i8, 98i8, 101i8, 32i8, 117i8, 115i8, 101i8,
    100i8, 32i8, 102i8, 111i8, 114i8, 32i8, 100i8, 108i8, 111i8, 112i8, 101i8, 110i8, 40i8, 41i8,
    32i8, 99i8, 97i8, 108i8, 108i8, 115i8, 10i8, 115i8, 101i8, 116i8, 112i8, 114i8, 111i8, 102i8,
    105i8, 108i8, 101i8, 40i8, 41i8, 32i8, 45i8, 45i8, 32i8, 115i8, 101i8, 116i8, 32i8, 116i8,
    104i8, 101i8, 32i8, 103i8, 108i8, 111i8, 98i8, 97i8, 108i8, 32i8, 112i8, 114i8, 111i8, 102i8,
    105i8, 108i8, 105i8, 110i8, 103i8, 32i8, 102i8, 117i8, 110i8, 99i8, 116i8, 105i8, 111i8, 110i8,
    10i8, 115i8, 101i8, 116i8, 114i8, 101i8, 99i8, 117i8, 114i8, 115i8, 105i8, 111i8, 110i8, 108i8,
    105i8, 109i8, 105i8, 116i8, 40i8, 41i8, 32i8, 45i8, 45i8, 32i8, 115i8, 101i8, 116i8, 32i8,
    116i8, 104i8, 101i8, 32i8, 109i8, 97i8, 120i8, 32i8, 114i8, 101i8, 99i8, 117i8, 114i8, 115i8,
    105i8, 111i8, 110i8, 32i8, 100i8, 101i8, 112i8, 116i8, 104i8, 32i8, 102i8, 111i8, 114i8, 32i8,
    116i8, 104i8, 101i8, 32i8, 105i8, 110i8, 116i8, 101i8, 114i8, 112i8, 114i8, 101i8, 116i8,
    101i8, 114i8, 10i8, 115i8, 101i8, 116i8, 116i8, 114i8, 97i8, 99i8, 101i8, 40i8, 41i8, 32i8,
    45i8, 45i8, 32i8, 115i8, 101i8, 116i8, 32i8, 116i8, 104i8, 101i8, 32i8, 103i8, 108i8, 111i8,
    98i8, 97i8, 108i8, 32i8, 100i8, 101i8, 98i8, 117i8, 103i8, 32i8, 116i8, 114i8, 97i8, 99i8,
    105i8, 110i8, 103i8, 32i8, 102i8, 117i8, 110i8, 99i8, 116i8, 105i8, 111i8, 110i8, 10i8, 0i8,
]);

static flags__doc__: __SlateAlign16<[i8; 78]> = __SlateAlign16([
    115i8, 121i8, 115i8, 46i8, 102i8, 108i8, 97i8, 103i8, 115i8, 10i8, 10i8, 70i8, 108i8, 97i8,
    103i8, 115i8, 32i8, 112i8, 114i8, 111i8, 118i8, 105i8, 100i8, 101i8, 100i8, 32i8, 116i8, 104i8,
    114i8, 111i8, 117i8, 103i8, 104i8, 32i8, 99i8, 111i8, 109i8, 109i8, 97i8, 110i8, 100i8, 32i8,
    108i8, 105i8, 110i8, 101i8, 32i8, 97i8, 114i8, 103i8, 117i8, 109i8, 101i8, 110i8, 116i8, 115i8,
    32i8, 111i8, 114i8, 32i8, 101i8, 110i8, 118i8, 105i8, 114i8, 111i8, 110i8, 109i8, 101i8, 110i8,
    116i8, 32i8, 118i8, 97i8, 114i8, 115i8, 46i8, 0i8,
]);

static mut FlagsType: _typeobject = unsafe { core::mem::zeroed() };

static mut flags_fields: __SlateAlign16<[PyStructSequence_Field; 22]> = __SlateAlign16([
    PyStructSequence_Field {
        name: (b"debug\0".as_ptr() as *mut i8) as *const i8,
        doc: (b"-d\0".as_ptr() as *mut i8) as *const i8,
    },
    PyStructSequence_Field {
        name: (b"inspect\0".as_ptr() as *mut i8) as *const i8,
        doc: (b"-i\0".as_ptr() as *mut i8) as *const i8,
    },
    PyStructSequence_Field {
        name: (b"interactive\0".as_ptr() as *mut i8) as *const i8,
        doc: (b"-i\0".as_ptr() as *mut i8) as *const i8,
    },
    PyStructSequence_Field {
        name: (b"optimize\0".as_ptr() as *mut i8) as *const i8,
        doc: (b"-O or -OO\0".as_ptr() as *mut i8) as *const i8,
    },
    PyStructSequence_Field {
        name: (b"dont_write_bytecode\0".as_ptr() as *mut i8) as *const i8,
        doc: (b"-B\0".as_ptr() as *mut i8) as *const i8,
    },
    PyStructSequence_Field {
        name: (b"no_user_site\0".as_ptr() as *mut i8) as *const i8,
        doc: (b"-s\0".as_ptr() as *mut i8) as *const i8,
    },
    PyStructSequence_Field {
        name: (b"no_site\0".as_ptr() as *mut i8) as *const i8,
        doc: (b"-S\0".as_ptr() as *mut i8) as *const i8,
    },
    PyStructSequence_Field {
        name: (b"ignore_environment\0".as_ptr() as *mut i8) as *const i8,
        doc: (b"-E\0".as_ptr() as *mut i8) as *const i8,
    },
    PyStructSequence_Field {
        name: (b"verbose\0".as_ptr() as *mut i8) as *const i8,
        doc: (b"-v\0".as_ptr() as *mut i8) as *const i8,
    },
    PyStructSequence_Field {
        name: (b"bytes_warning\0".as_ptr() as *mut i8) as *const i8,
        doc: (b"-b\0".as_ptr() as *mut i8) as *const i8,
    },
    PyStructSequence_Field {
        name: (b"quiet\0".as_ptr() as *mut i8) as *const i8,
        doc: (b"-q\0".as_ptr() as *mut i8) as *const i8,
    },
    PyStructSequence_Field {
        name: (b"hash_randomization\0".as_ptr() as *mut i8) as *const i8,
        doc: (b"-R\0".as_ptr() as *mut i8) as *const i8,
    },
    PyStructSequence_Field {
        name: (b"isolated\0".as_ptr() as *mut i8) as *const i8,
        doc: (b"-I\0".as_ptr() as *mut i8) as *const i8,
    },
    PyStructSequence_Field {
        name: (b"dev_mode\0".as_ptr() as *mut i8) as *const i8,
        doc: (b"-X dev\0".as_ptr() as *mut i8) as *const i8,
    },
    PyStructSequence_Field {
        name: (b"utf8_mode\0".as_ptr() as *mut i8) as *const i8,
        doc: (b"-X utf8\0".as_ptr() as *mut i8) as *const i8,
    },
    PyStructSequence_Field {
        name: (b"warn_default_encoding\0".as_ptr() as *mut i8) as *const i8,
        doc: (b"-X warn_default_encoding\0".as_ptr() as *mut i8) as *const i8,
    },
    PyStructSequence_Field {
        name: (b"safe_path\0".as_ptr() as *mut i8) as *const i8,
        doc: (b"-P\0".as_ptr() as *mut i8) as *const i8,
    },
    PyStructSequence_Field {
        name: (b"int_max_str_digits\0".as_ptr() as *mut i8) as *const i8,
        doc: (b"-X int_max_str_digits\0".as_ptr() as *mut i8) as *const i8,
    },
    PyStructSequence_Field {
        name: (b"gil\0".as_ptr() as *mut i8) as *const i8,
        doc: (b"-X gil\0".as_ptr() as *mut i8) as *const i8,
    },
    PyStructSequence_Field {
        name: (b"thread_inherit_context\0".as_ptr() as *mut i8) as *const i8,
        doc: (b"-X thread_inherit_context\0".as_ptr() as *mut i8) as *const i8,
    },
    PyStructSequence_Field {
        name: (b"context_aware_warnings\0".as_ptr() as *mut i8) as *const i8,
        doc: (b"-X context_aware_warnings\0".as_ptr() as *mut i8) as *const i8,
    },
    {
        let mut __t0: PyStructSequence_Field = unsafe { core::mem::zeroed() };
        __t0.name = core::ptr::null::<i8>();
        __t0
    },
]);

/// name
/// doc
/// fields
static mut flags_desc: PyStructSequence_Desc = {
    let mut __t0: PyStructSequence_Desc = unsafe { core::mem::zeroed() };
    __t0.name = (b"sys.flags\0".as_ptr() as *mut i8) as *const i8;
    __t0.doc = unsafe { core::ptr::addr_of!(flags__doc__.0) as *const i8 };
    __t0.fields = unsafe { core::ptr::addr_of_mut!(flags_fields.0) as *mut PyStructSequence_Field };
    __t0.n_in_sequence = 18i32;
    __t0
};

fn sys_set_flag(flags: *mut _object, pos: i64, value: *mut _object) {
    let old_value: *mut _object = unsafe { PyStructSequence_GetItem(flags, pos) };
    unsafe { PyStructSequence_SetItem(flags, pos, Py_NewRef!(value)) };
    Py_XDECREF!(old_value);
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PySys_SetFlagObj(pos: i64, value: *mut _object) -> i32 {
    let mut i: i64 = unsafe { core::mem::zeroed() };
    let mut old_flags: *mut _object = unsafe { core::mem::zeroed() };
    // immortal ref
    let mut flags_str: *mut _object = unsafe { core::mem::zeroed() };
    let mut new_flags: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        new_flags = core::ptr::null_mut::<_object>();
        flags_str = core::ptr::addr_of_mut!(
            _PyRuntime
                .static_objects
                .singletons
                .strings
                .identifiers
                ._py_flags
                ._ascii
                .ob_base
        );
        old_flags = _PySys_GetRequiredAttr(flags_str);
        if !old_flags.is_null() {
            new_flags = PyStructSequence_New(core::ptr::addr_of_mut!(FlagsType));
            if !new_flags.is_null() {
                i = 0;
                while i < (Py_ARRAY_LENGTH__17!().wrapping_sub(1) as i64) {
                    if i != pos {
                        let old_value: *mut _object = PyStructSequence_GetItem(old_flags, i); // borrowed ref
                        sys_set_flag(new_flags, i, old_value);
                    } else {
                        sys_set_flag(new_flags, pos, value);
                    }
                    i = i.wrapping_add(1);
                }
                let res: i32 = _PySys_SetAttr(flags_str, new_flags);
                Py_DECREF!(old_flags);
                Py_DECREF!(new_flags);
                return res;
            }
        }
        Py_XDECREF!(old_flags);
        Py_XDECREF!(new_flags);
        return 1i32.wrapping_neg();
    }
}

fn _PySys_SetFlagInt(pos: i64, value: i32) -> i32 {
    let obj: *mut _object = unsafe { PyLong_FromLong(value as i64) };
    if obj.is_null() {
        return 1i32.wrapping_neg();
    }
    let res: i32 = _PySys_SetFlagObj(pos, obj);
    Py_DECREF!(obj);
    res
}

fn set_flags_from_config(interp: *mut _is, flags: *mut _object) -> i32 {
    let preconfig: *const PyPreConfig =
        (unsafe { core::ptr::addr_of_mut!((*(*interp).runtime).preconfig) }) as *const PyPreConfig;
    let config: *const PyConfig = unsafe { _PyInterpreterState_GetConfig(interp) };
    let mut pos: i64 = 0;
    SetFlag!(unsafe { (*config).parser_debug }, flags, pos);
    SetFlag!(unsafe { (*config).inspect }, flags, pos);
    SetFlag!(unsafe { (*config).interactive }, flags, pos);
    SetFlag!(unsafe { (*config).optimization_level }, flags, pos);
    SetFlag!((unsafe { (*config).write_bytecode }) == 0, flags, pos);
    SetFlag!((unsafe { (*config).user_site_directory }) == 0, flags, pos);
    SetFlag!((unsafe { (*config).site_import }) == 0, flags, pos);
    SetFlag!((unsafe { (*config).use_environment }) == 0, flags, pos);
    SetFlag!(unsafe { (*config).verbose }, flags, pos);
    SetFlag!(unsafe { (*config).bytes_warning }, flags, pos);
    SetFlag!(unsafe { (*config).quiet }, flags, pos);
    SetFlag!(
        (unsafe { (*config).use_hash_seed }) == 0 || (unsafe { (*config).hash_seed }) != 0,
        flags,
        pos
    );
    SetFlag!(unsafe { (*config).isolated }, flags, pos);
    SetFlagObj!(
        unsafe { PyBool_FromLong((*config).dev_mode as i64) },
        flags,
        pos
    );
    SetFlag!(unsafe { (*preconfig).utf8_mode }, flags, pos);
    SetFlag!(unsafe { (*config).warn_default_encoding }, flags, pos);
    SetFlagObj!(
        unsafe { PyBool_FromLong((*config).safe_path as i64) },
        flags,
        pos
    );
    SetFlag!(unsafe { (*config).int_max_str_digits }, flags, pos);
    SetFlagObj!(unsafe { PyLong_FromLong(1i64) }, flags, pos);
    SetFlag!(unsafe { (*config).thread_inherit_context }, flags, pos);
    SetFlag!(unsafe { (*config).context_aware_warnings }, flags, pos);
    0
}

fn make_flags(interp: *mut _is) -> *mut _object {
    let flags: *mut _object = unsafe { PyStructSequence_New(core::ptr::addr_of_mut!(FlagsType)) };
    if flags.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    if set_flags_from_config(interp, flags) < 0 {
        Py_DECREF!(flags);
        return core::ptr::null_mut::<_object>();
    }
    flags
}

static version_info__doc__: __SlateAlign16<[i8; 56]> = __SlateAlign16([
    115i8, 121i8, 115i8, 46i8, 118i8, 101i8, 114i8, 115i8, 105i8, 111i8, 110i8, 95i8, 105i8, 110i8,
    102i8, 111i8, 10i8, 10i8, 86i8, 101i8, 114i8, 115i8, 105i8, 111i8, 110i8, 32i8, 105i8, 110i8,
    102i8, 111i8, 114i8, 109i8, 97i8, 116i8, 105i8, 111i8, 110i8, 32i8, 97i8, 115i8, 32i8, 97i8,
    32i8, 110i8, 97i8, 109i8, 101i8, 100i8, 32i8, 116i8, 117i8, 112i8, 108i8, 101i8, 46i8, 0i8,
]);

static mut VersionInfoType: _typeobject = unsafe { core::mem::zeroed() };

static mut version_info_fields: __SlateAlign16<[PyStructSequence_Field; 6]> = __SlateAlign16([
    PyStructSequence_Field {
        name: (b"major\0".as_ptr() as *mut i8) as *const i8,
        doc: (b"Major release number\0".as_ptr() as *mut i8) as *const i8,
    },
    PyStructSequence_Field {
        name: (b"minor\0".as_ptr() as *mut i8) as *const i8,
        doc: (b"Minor release number\0".as_ptr() as *mut i8) as *const i8,
    },
    PyStructSequence_Field {
        name: (b"micro\0".as_ptr() as *mut i8) as *const i8,
        doc: (b"Patch release number\0".as_ptr() as *mut i8) as *const i8,
    },
    PyStructSequence_Field {
        name: (b"releaselevel\0".as_ptr() as *mut i8) as *const i8,
        doc: (b"'alpha', 'beta', 'candidate', or 'final'\0".as_ptr() as *mut i8) as *const i8,
    },
    PyStructSequence_Field {
        name: (b"serial\0".as_ptr() as *mut i8) as *const i8,
        doc: (b"Serial release number\0".as_ptr() as *mut i8) as *const i8,
    },
    {
        let mut __t0: PyStructSequence_Field = unsafe { core::mem::zeroed() };
        __t0.name = core::ptr::null::<i8>();
        __t0
    },
]);

/// name
/// doc
/// fields
static mut version_info_desc: PyStructSequence_Desc = {
    let mut __t0: PyStructSequence_Desc = unsafe { core::mem::zeroed() };
    __t0.name = (b"sys.version_info\0".as_ptr() as *mut i8) as *const i8;
    __t0.doc = unsafe { core::ptr::addr_of!(version_info__doc__.0) as *const i8 };
    __t0.fields =
        unsafe { core::ptr::addr_of_mut!(version_info_fields.0) as *mut PyStructSequence_Field };
    __t0.n_in_sequence = 5i32;
    __t0
};

fn make_version_info(tstate: *mut _ts) -> *mut _object {
    let mut version_info: *mut _object = unsafe { core::mem::zeroed() };
    let mut pos: i32 = 0;
    version_info = unsafe { PyStructSequence_New(core::ptr::addr_of_mut!(VersionInfoType)) };
    if version_info.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    // These release level checks are mutually exclusive and cover
    // the field, so don't get too fancy with the pre-processor!
    let s: *mut i8 = c"final".as_ptr() as *mut i8;
    SetIntItem!(3i32, pos, version_info);
    SetIntItem!(14i32, pos, version_info);
    SetIntItem!(8i32, pos, version_info);
    SetStrItem!(s, pos, version_info);
    SetIntItem!(0i32, pos, version_info);
    if !_PyErr_Occurred(tstate).is_null() {
        Py_CLEAR__2!(version_info);
        return core::ptr::null_mut::<_object>();
    }
    version_info
}

// sys.implementation values
#[unsafe(no_mangle)]
static mut _PySys_ImplName: *const i8 = (b"cpython\0".as_ptr() as *mut i8) as *const i8;

#[unsafe(no_mangle)]
static mut _PySys_ImplCacheTag: *const i8 = (b"cpython-314\0".as_ptr() as *mut i8) as *const i8;

fn make_impl_info(version_info: *mut _object) -> *mut _object {
    let mut value: *mut _object = unsafe { core::mem::zeroed() };
    let mut impl_info: *mut _object = unsafe { core::mem::zeroed() };
    let mut res: i32 = unsafe { core::mem::zeroed() };
    unsafe {
        impl_info = PyDict_New();
        if impl_info.is_null() {
            return core::ptr::null_mut::<_object>();
        } else {
            // populate the dict
            value = PyUnicode_FromString(_PySys_ImplName);
            if !value.is_null() {
                res = PyDict_SetItemString(impl_info, c"name".as_ptr(), value);
                Py_DECREF!(value);
                if res >= 0 {
                    value = PyUnicode_FromString(_PySys_ImplCacheTag);
                    if !value.is_null() {
                        res = PyDict_SetItemString(impl_info, c"cache_tag".as_ptr(), value);
                        Py_DECREF!(value);
                        if res >= 0 {
                            res =
                                PyDict_SetItemString(impl_info, c"version".as_ptr(), version_info);
                            if res >= 0 {
                                value = PyLong_FromLong(_Py_PACK_FULL_VERSION!(
                                    3i32, 14i32, 8i32, 15i32, 0i32
                                ) as i64);
                                if !value.is_null() {
                                    res = PyDict_SetItemString(
                                        impl_info,
                                        c"hexversion".as_ptr(),
                                        value,
                                    );
                                    Py_DECREF!(value);
                                    if res >= 0 {
                                        value = PyUnicode_FromString(c"x86_64-linux-gnu".as_ptr());
                                        if !value.is_null() {
                                            res = PyDict_SetItemString(
                                                impl_info,
                                                c"_multiarch".as_ptr(),
                                                value,
                                            );
                                            Py_DECREF!(value);
                                            if res >= 0 {
                                                // PEP-734
                                                value = _PyObject_CAST__2!(
                                                    core::ptr::addr_of_mut!(_Py_TrueStruct)
                                                );
                                                res = PyDict_SetItemString(
                                                    impl_info,
                                                    c"supports_isolated_interpreters".as_ptr(),
                                                    value,
                                                );
                                                if res >= 0 {
                                                    // dict ready
                                                    let ns: *mut _object =
                                                        _PyNamespace_New(impl_info);
                                                    Py_DECREF!(impl_info);
                                                    return ns;
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
            Py_CLEAR__2!(impl_info);
            return core::ptr::null_mut::<_object>();
        }
    }
}

/// multiple "initialization" just copies the module dict.
static mut sysmodule: PyModuleDef = PyModuleDef {
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
    m_name: (b"sys\0".as_ptr() as *mut i8) as *const i8,
    m_doc: unsafe { core::ptr::addr_of!(sys_doc.0) as *const i8 },
    m_size: 1i32.wrapping_neg() as i64,
    m_methods: unsafe { core::ptr::addr_of_mut!(sys_methods.0) as *mut PyMethodDef },
    m_slots: core::ptr::null_mut::<PyModuleDef_Slot>(),
    m_traverse: None,
    m_clear: None,
    m_free: None,
};

// Updating the sys namespace, returning NULL pointer on error
fn _PySys_InitCore(tstate: *mut _ts, sysdict: *mut _object) -> __SlateRecord_initconfig_h_19 {
    let mut __slate_compound_31: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_32: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_33: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut v: *mut _object = unsafe { core::mem::zeroed() };
    let mut v_1: *mut _object = unsafe { core::mem::zeroed() };
    let mut v_2: *mut _object = unsafe { core::mem::zeroed() };
    let mut v_3: *mut _object = unsafe { core::mem::zeroed() };
    let mut v_4: *mut _object = unsafe { core::mem::zeroed() };
    let mut v_5: *mut _object = unsafe { core::mem::zeroed() };
    let mut v_6: *mut _object = unsafe { core::mem::zeroed() };
    let mut v_7: *mut _object = unsafe { core::mem::zeroed() };
    let mut v_8: *mut _object = unsafe { core::mem::zeroed() };
    let mut v_9: *mut _object = unsafe { core::mem::zeroed() };
    let mut v_10: *mut _object = unsafe { core::mem::zeroed() };
    let mut v_11: *mut _object = unsafe { core::mem::zeroed() };
    let mut v_12: *mut _object = unsafe { core::mem::zeroed() };
    let mut v_13: *mut _object = unsafe { core::mem::zeroed() };
    let mut v_14: *mut _object = unsafe { core::mem::zeroed() };
    let mut v_15: *mut _object = unsafe { core::mem::zeroed() };
    let mut v_16: *mut _object = unsafe { core::mem::zeroed() };
    let mut v_17: *mut _object = unsafe { core::mem::zeroed() };
    let mut v_18: *mut _object = unsafe { core::mem::zeroed() };
    let mut v_19: *mut _object = unsafe { core::mem::zeroed() };
    let mut v_20: *mut _object = unsafe { core::mem::zeroed() };
    let mut v_21: *mut _object = unsafe { core::mem::zeroed() };
    let mut v_22: *mut _object = unsafe { core::mem::zeroed() };
    let mut v_23: *mut _object = unsafe { core::mem::zeroed() };
    let mut v_24: *mut _object = unsafe { core::mem::zeroed() };
    let mut v_25: *mut _object = unsafe { core::mem::zeroed() };
    let mut v_26: *mut _object = unsafe { core::mem::zeroed() };
    let mut v_27: *mut _object = unsafe { core::mem::zeroed() };
    let mut interp: *mut _is = unsafe { core::mem::zeroed() };
    let mut res: i32 = unsafe { core::mem::zeroed() };
    let mut version_info: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        interp = (*tstate).interp;
        // stdin/stdout/stderr are set in pylifecycle.c
        '__join_0: {
            loop {
                v_27 = PyMapping_GetItemString(sysdict, c"displayhook".as_ptr());
                if v_27.is_null() {
                    break '__join_0;
                } else {
                    res = PyDict_SetItemString(sysdict, c"__displayhook__".as_ptr(), v_27);
                    Py_DECREF!(v_27);
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
                v_26 = PyMapping_GetItemString(sysdict, c"excepthook".as_ptr());
                if v_26.is_null() {
                    break '__join_0;
                } else {
                    res = PyDict_SetItemString(sysdict, c"__excepthook__".as_ptr(), v_26);
                    Py_DECREF!(v_26);
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
                v_25 = PyMapping_GetItemString(sysdict, c"breakpointhook".as_ptr());
                if v_25.is_null() {
                    break '__join_0;
                } else {
                    res = PyDict_SetItemString(sysdict, c"__breakpointhook__".as_ptr(), v_25);
                    Py_DECREF!(v_25);
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
                v_24 = PyMapping_GetItemString(sysdict, c"unraisablehook".as_ptr());
                if v_24.is_null() {
                    break '__join_0;
                } else {
                    res = PyDict_SetItemString(sysdict, c"__unraisablehook__".as_ptr(), v_24);
                    Py_DECREF!(v_24);
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
                v_23 = PyUnicode_FromString(Py_GetVersion());
                if v_23.is_null() {
                    break '__join_0;
                } else {
                    res = PyDict_SetItemString(sysdict, c"version".as_ptr(), v_23);
                    Py_DECREF!(v_23);
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
                v_22 =
                    PyLong_FromLong(_Py_PACK_FULL_VERSION!(3i32, 14i32, 8i32, 15i32, 0i32) as i64);
                if v_22.is_null() {
                    break '__join_0;
                } else {
                    res = PyDict_SetItemString(sysdict, c"hexversion".as_ptr(), v_22);
                    Py_DECREF!(v_22);
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
                v_21 = Py_BuildValue(
                    c"(szz)".as_ptr(),
                    c"CPython".as_ptr() as *mut i8,
                    _Py_gitidentifier(),
                    _Py_gitversion(),
                );
                if v_21.is_null() {
                    break '__join_0;
                } else {
                    res = PyDict_SetItemString(sysdict, c"_git".as_ptr(), v_21);
                    Py_DECREF!(v_21);
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
                v_20 = PyUnicode_FromString(c"".as_ptr());
                if v_20.is_null() {
                    break '__join_0;
                } else {
                    res = PyDict_SetItemString(sysdict, c"_framework".as_ptr(), v_20);
                    Py_DECREF!(v_20);
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
                v_19 = PyLong_FromLong(1013i64);
                if v_19.is_null() {
                    break '__join_0;
                } else {
                    res = PyDict_SetItemString(sysdict, c"api_version".as_ptr(), v_19);
                    Py_DECREF!(v_19);
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
                v_18 = PyUnicode_FromString(Py_GetCopyright());
                if v_18.is_null() {
                    break '__join_0;
                } else {
                    res = PyDict_SetItemString(sysdict, c"copyright".as_ptr(), v_18);
                    Py_DECREF!(v_18);
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
                v_17 = PyUnicode_FromString(Py_GetPlatform());
                if v_17.is_null() {
                    break '__join_0;
                } else {
                    res = PyDict_SetItemString(sysdict, c"platform".as_ptr(), v_17);
                    Py_DECREF!(v_17);
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
                v_16 = PyLong_FromSsize_t(9223372036854775807i64);
                if v_16.is_null() {
                    break '__join_0;
                } else {
                    res = PyDict_SetItemString(sysdict, c"maxsize".as_ptr(), v_16);
                    Py_DECREF!(v_16);
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
                v_15 = PyFloat_GetInfo();
                if v_15.is_null() {
                    break '__join_0;
                } else {
                    res = PyDict_SetItemString(sysdict, c"float_info".as_ptr(), v_15);
                    Py_DECREF!(v_15);
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
                v_14 = PyLong_GetInfo();
                if v_14.is_null() {
                    break '__join_0;
                } else {
                    res = PyDict_SetItemString(sysdict, c"int_info".as_ptr(), v_14);
                    Py_DECREF!(v_14);
                    if res < 0 {
                        break '__join_0;
                    } else {
                        if true {
                            break;
                        }
                    }
                }
            }
            '__join_1: {
                // initialize hash_info
                if _PyStructSequence_InitBuiltin(
                    interp,
                    core::ptr::addr_of_mut!(Hash_InfoType),
                    core::ptr::addr_of_mut!(hash_info_desc),
                ) >= 0
                {
                    loop {
                        v_13 = get_hash_info(tstate);
                        if v_13.is_null() {
                            break '__join_0;
                        } else {
                            res = PyDict_SetItemString(sysdict, c"hash_info".as_ptr(), v_13);
                            Py_DECREF!(v_13);
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
                        v_12 = PyLong_FromLong(1114111i64);
                        if v_12.is_null() {
                            break '__join_0;
                        } else {
                            res = PyDict_SetItemString(sysdict, c"maxunicode".as_ptr(), v_12);
                            Py_DECREF!(v_12);
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
                        v_11 = list_builtin_module_names();
                        if v_11.is_null() {
                            break '__join_0;
                        } else {
                            res = PyDict_SetItemString(
                                sysdict,
                                c"builtin_module_names".as_ptr(),
                                v_11,
                            );
                            Py_DECREF!(v_11);
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
                        v_10 = list_stdlib_module_names();
                        if v_10.is_null() {
                            break '__join_0;
                        } else {
                            res = PyDict_SetItemString(
                                sysdict,
                                c"stdlib_module_names".as_ptr(),
                                v_10,
                            );
                            Py_DECREF!(v_10);
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
                        v_9 = PyUnicode_FromString(c"little".as_ptr());
                        if v_9.is_null() {
                            break '__join_0;
                        } else {
                            res = PyDict_SetItemString(sysdict, c"byteorder".as_ptr(), v_9);
                            Py_DECREF!(v_9);
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
                        v_8 = PyUnicode_FromString(c"".as_ptr());
                        if v_8.is_null() {
                            break '__join_0;
                        } else {
                            res = PyDict_SetItemString(sysdict, c"abiflags".as_ptr(), v_8);
                            Py_DECREF!(v_8);
                            if res < 0 {
                                break '__join_0;
                            } else {
                                if true {
                                    break;
                                }
                            }
                        }
                    }
                    // version_info
                    loop {
                        if _PyStructSequence_InitBuiltinWithFlags(
                            interp,
                            core::ptr::addr_of_mut!(VersionInfoType),
                            core::ptr::addr_of_mut!(version_info_desc),
                            128u64,
                        ) < 0
                        {
                            break '__join_1;
                        } else {
                            if true {
                                break;
                            }
                        }
                    }
                    version_info = make_version_info(tstate);
                    loop {
                        v_7 = version_info;
                        if v_7.is_null() {
                            break '__join_0;
                        } else {
                            res = PyDict_SetItemString(sysdict, c"version_info".as_ptr(), v_7);
                            Py_DECREF!(v_7);
                            if res < 0 {
                                break '__join_0;
                            } else {
                                if true {
                                    break;
                                }
                            }
                        }
                    }
                    // implementation
                    loop {
                        v_6 = make_impl_info(version_info);
                        if v_6.is_null() {
                            break '__join_0;
                        } else {
                            res = PyDict_SetItemString(sysdict, c"implementation".as_ptr(), v_6);
                            Py_DECREF!(v_6);
                            if res < 0 {
                                break '__join_0;
                            } else {
                                if true {
                                    break;
                                }
                            }
                        }
                    }
                    // sys.flags: updated later by _PySys_UpdateConfig()
                    loop {
                        if _PyStructSequence_InitBuiltinWithFlags(
                            interp,
                            core::ptr::addr_of_mut!(FlagsType),
                            core::ptr::addr_of_mut!(flags_desc),
                            128u64,
                        ) < 0
                        {
                            break '__join_1;
                        } else {
                            if true {
                                break;
                            }
                        }
                    }
                    loop {
                        v_5 = make_flags((*tstate).interp);
                        if v_5.is_null() {
                            break '__join_0;
                        } else {
                            res = PyDict_SetItemString(sysdict, c"flags".as_ptr(), v_5);
                            Py_DECREF!(v_5);
                            if res < 0 {
                                break '__join_0;
                            } else {
                                if true {
                                    break;
                                }
                            }
                        }
                    }
                    // float repr style: 0.03 (short) vs 0.029999999999999999 (legacy)
                    loop {
                        v_4 = PyUnicode_FromString(c"short".as_ptr());
                        if v_4.is_null() {
                            break '__join_0;
                        } else {
                            res = PyDict_SetItemString(sysdict, c"float_repr_style".as_ptr(), v_4);
                            Py_DECREF!(v_4);
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
                        v_3 = PyThread_GetInfo();
                        if v_3.is_null() {
                            break '__join_0;
                        } else {
                            res = PyDict_SetItemString(sysdict, c"thread_info".as_ptr(), v_3);
                            Py_DECREF!(v_3);
                            if res < 0 {
                                break '__join_0;
                            } else {
                                if true {
                                    break;
                                }
                            }
                        }
                    }
                    // initialize asyncgen_hooks
                    if _PyStructSequence_InitBuiltin(
                        interp,
                        core::ptr::addr_of_mut!(AsyncGenHooksType),
                        core::ptr::addr_of_mut!(asyncgen_hooks_desc),
                    ) >= 0
                    {
                        // adding sys.path_hooks and sys.path_importer_cache
                        loop {
                            v_2 = PyList_New(0i64);
                            if v_2.is_null() {
                                break '__join_0;
                            } else {
                                res = PyDict_SetItemString(sysdict, c"meta_path".as_ptr(), v_2);
                                Py_DECREF!(v_2);
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
                            v_1 = PyDict_New();
                            if v_1.is_null() {
                                break '__join_0;
                            } else {
                                res = PyDict_SetItemString(
                                    sysdict,
                                    c"path_importer_cache".as_ptr(),
                                    v_1,
                                );
                                Py_DECREF!(v_1);
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
                            v = PyList_New(0i64);
                            if v.is_null() {
                                break '__join_0;
                            } else {
                                res = PyDict_SetItemString(sysdict, c"path_hooks".as_ptr(), v);
                                Py_DECREF!(v);
                                if res < 0 {
                                    break '__join_0;
                                } else {
                                    if true {
                                        break;
                                    }
                                }
                            }
                        }
                        if !_PyErr_Occurred(tstate).is_null() {
                            break '__join_0;
                        } else {
                            return *{
                                __slate_compound_33 = {
                                    let mut __t2: __SlateRecord_initconfig_h_19 =
                                        core::mem::zeroed();
                                    __t2._type = 0u32;
                                    __t2
                                };
                                core::ptr::addr_of_mut!(__slate_compound_33)
                            };
                        }
                    }
                }
            }
            return *{
                __slate_compound_32 = {
                    let mut __t1: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                    __t1._type = 1u32;
                    __t1.func = _PyStatus_GET_FUNC__33!() as *const i8;
                    __t1.err_msg = c"failed to initialize a type".as_ptr();
                    __t1
                };
                core::ptr::addr_of_mut!(__slate_compound_32)
            };
        }
        return *{
            __slate_compound_31 = {
                let mut __t0: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                __t0._type = 1u32;
                __t0.func = _PyStatus_GET_FUNC__33!() as *const i8;
                __t0.err_msg = c"can't initialize sys module".as_ptr();
                __t0
            };
            core::ptr::addr_of_mut!(__slate_compound_31)
        };
    }
}

/// Update sys attributes for a new PyConfig configuration.
///
/// This function also adds attributes that _PySys_InitCore() didn't add.
#[unsafe(no_mangle)]
extern "C-unwind" fn _PySys_UpdateConfig(tstate: *mut _ts) -> i32 {
    let mut v: *mut _object = unsafe { core::mem::zeroed() };
    // replace sys.flags
    let mut new_flags: *mut _object = unsafe { core::mem::zeroed() };
    let mut v_1: *mut _object = unsafe { core::mem::zeroed() };
    let mut stdlibdir: *const i32 = unsafe { core::mem::zeroed() };
    let mut v_2: *mut _object = unsafe { core::mem::zeroed() };
    let mut v_3: *mut _object = unsafe { core::mem::zeroed() };
    let mut v_4: *mut _object = unsafe { core::mem::zeroed() };
    let mut v_5: *mut _object = unsafe { core::mem::zeroed() };
    let mut v_6: *mut _object = unsafe { core::mem::zeroed() };
    let mut v_7: *mut _object = unsafe { core::mem::zeroed() };
    let mut v_8: *mut _object = unsafe { core::mem::zeroed() };
    let mut v_9: *mut _object = unsafe { core::mem::zeroed() };
    let mut v_10: *mut _object = unsafe { core::mem::zeroed() };
    let mut v_11: *mut _object = unsafe { core::mem::zeroed() };
    let mut v_12: *mut _object = unsafe { core::mem::zeroed() };
    let mut v_13: *mut _object = unsafe { core::mem::zeroed() };
    let mut v_14: *mut _object = unsafe { core::mem::zeroed() };
    let mut res: i32 = unsafe { core::mem::zeroed() };
    let mut config: *const PyConfig = unsafe { core::mem::zeroed() };
    let mut sysdict: *mut _object = unsafe { core::mem::zeroed() };
    let mut interp: *mut _is = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_0: {
            interp = (*tstate).interp;
            sysdict = (*interp).sysdict;
            config = _PyInterpreterState_GetConfig(interp);
            if (*config).module_search_paths_set != 0 {
                loop {
                    v_14 = _PyWideStringList_AsList(core::ptr::addr_of!(
                        (*config).module_search_paths
                    ));
                    if v_14.is_null() {
                        break '__join_0;
                    } else {
                        res = PyDict_SetItemString(sysdict, c"path".as_ptr(), v_14);
                        Py_DECREF!(v_14);
                        if res < 0 {
                            break '__join_0;
                        } else {
                            if true {
                                break;
                            }
                        }
                    }
                }
            }
            if !unsafe { (*config).executable }.is_null() {
                loop {
                    v_13 = PyUnicode_FromWideChar(
                        (*config).executable as *const i32,
                        1i32.wrapping_neg() as i64,
                    );
                    if v_13.is_null() {
                        break '__join_0;
                    } else {
                        res = PyDict_SetItemString(sysdict, c"executable".as_ptr(), v_13);
                        Py_DECREF!(v_13);
                        if res < 0 {
                            break '__join_0;
                        } else {
                            if true {
                                break;
                            }
                        }
                    }
                }
            }
            if !unsafe { (*config).base_executable }.is_null() {
                loop {
                    v_12 = PyUnicode_FromWideChar(
                        (*config).base_executable as *const i32,
                        1i32.wrapping_neg() as i64,
                    );
                    if v_12.is_null() {
                        break '__join_0;
                    } else {
                        res = PyDict_SetItemString(sysdict, c"_base_executable".as_ptr(), v_12);
                        Py_DECREF!(v_12);
                        if res < 0 {
                            break '__join_0;
                        } else {
                            if true {
                                break;
                            }
                        }
                    }
                }
            }
            if !unsafe { (*config).prefix }.is_null() {
                loop {
                    v_11 = PyUnicode_FromWideChar(
                        (*config).prefix as *const i32,
                        1i32.wrapping_neg() as i64,
                    );
                    if v_11.is_null() {
                        break '__join_0;
                    } else {
                        res = PyDict_SetItemString(sysdict, c"prefix".as_ptr(), v_11);
                        Py_DECREF!(v_11);
                        if res < 0 {
                            break '__join_0;
                        } else {
                            if true {
                                break;
                            }
                        }
                    }
                }
            }
            if !unsafe { (*config).base_prefix }.is_null() {
                loop {
                    v_10 = PyUnicode_FromWideChar(
                        (*config).base_prefix as *const i32,
                        1i32.wrapping_neg() as i64,
                    );
                    if v_10.is_null() {
                        break '__join_0;
                    } else {
                        res = PyDict_SetItemString(sysdict, c"base_prefix".as_ptr(), v_10);
                        Py_DECREF!(v_10);
                        if res < 0 {
                            break '__join_0;
                        } else {
                            if true {
                                break;
                            }
                        }
                    }
                }
            }
            if !unsafe { (*config).exec_prefix }.is_null() {
                loop {
                    v_9 = PyUnicode_FromWideChar(
                        (*config).exec_prefix as *const i32,
                        1i32.wrapping_neg() as i64,
                    );
                    if v_9.is_null() {
                        break '__join_0;
                    } else {
                        res = PyDict_SetItemString(sysdict, c"exec_prefix".as_ptr(), v_9);
                        Py_DECREF!(v_9);
                        if res < 0 {
                            break '__join_0;
                        } else {
                            if true {
                                break;
                            }
                        }
                    }
                }
            }
            if !unsafe { (*config).base_exec_prefix }.is_null() {
                loop {
                    v_8 = PyUnicode_FromWideChar(
                        (*config).base_exec_prefix as *const i32,
                        1i32.wrapping_neg() as i64,
                    );
                    if v_8.is_null() {
                        break '__join_0;
                    } else {
                        res = PyDict_SetItemString(sysdict, c"base_exec_prefix".as_ptr(), v_8);
                        Py_DECREF!(v_8);
                        if res < 0 {
                            break '__join_0;
                        } else {
                            if true {
                                break;
                            }
                        }
                    }
                }
            }
            if !unsafe { (*config).platlibdir }.is_null() {
                loop {
                    v_7 = PyUnicode_FromWideChar(
                        (*config).platlibdir as *const i32,
                        1i32.wrapping_neg() as i64,
                    );
                    if v_7.is_null() {
                        break '__join_0;
                    } else {
                        res = PyDict_SetItemString(sysdict, c"platlibdir".as_ptr(), v_7);
                        Py_DECREF!(v_7);
                        if res < 0 {
                            break '__join_0;
                        } else {
                            if true {
                                break;
                            }
                        }
                    }
                }
            }
            if !unsafe { (*config).pycache_prefix }.is_null() {
                loop {
                    v_6 = PyUnicode_FromWideChar(
                        (*config).pycache_prefix as *const i32,
                        1i32.wrapping_neg() as i64,
                    );
                    if v_6.is_null() {
                        break '__join_0;
                    } else {
                        res = PyDict_SetItemString(sysdict, c"pycache_prefix".as_ptr(), v_6);
                        Py_DECREF!(v_6);
                        if res < 0 {
                            break '__join_0;
                        } else {
                            if true {
                                break;
                            }
                        }
                    }
                }
            } else {
                if PyDict_SetItemString(
                    sysdict,
                    c"pycache_prefix".as_ptr(),
                    core::ptr::addr_of_mut!(_Py_NoneStruct),
                ) < 0
                {
                    return 1i32.wrapping_neg();
                }
            }
            loop {
                v_5 = _PyWideStringList_AsList(core::ptr::addr_of!((*config).argv));
                if v_5.is_null() {
                    break '__join_0;
                } else {
                    res = PyDict_SetItemString(sysdict, c"argv".as_ptr(), v_5);
                    Py_DECREF!(v_5);
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
                v_4 = _PyWideStringList_AsList(core::ptr::addr_of!((*config).orig_argv));
                if v_4.is_null() {
                    break '__join_0;
                } else {
                    res = PyDict_SetItemString(sysdict, c"orig_argv".as_ptr(), v_4);
                    Py_DECREF!(v_4);
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
                v_3 = _PyWideStringList_AsList(core::ptr::addr_of!((*config).warnoptions));
                if v_3.is_null() {
                    break '__join_0;
                } else {
                    res = PyDict_SetItemString(sysdict, c"warnoptions".as_ptr(), v_3);
                    Py_DECREF!(v_3);
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
                v_2 = _PyConfig_CreateXOptionsDict(config);
                if v_2.is_null() {
                    break '__join_0;
                } else {
                    res = PyDict_SetItemString(sysdict, c"_xoptions".as_ptr(), v_2);
                    Py_DECREF!(v_2);
                    if res < 0 {
                        break '__join_0;
                    } else {
                        if true {
                            break;
                        }
                    }
                }
            }
            stdlibdir = _Py_GetStdlibDir() as *const i32;
            if !stdlibdir.is_null() {
                loop {
                    v_1 = PyUnicode_FromWideChar(stdlibdir, 1i32.wrapping_neg() as i64);
                    if v_1.is_null() {
                        break '__join_0;
                    } else {
                        res = PyDict_SetItemString(sysdict, c"_stdlib_dir".as_ptr(), v_1);
                        Py_DECREF!(v_1);
                        if res < 0 {
                            break '__join_0;
                        } else {
                            if true {
                                break;
                            }
                        }
                    }
                }
            } else {
                if PyDict_SetItemString(
                    sysdict,
                    c"_stdlib_dir".as_ptr(),
                    core::ptr::addr_of_mut!(_Py_NoneStruct),
                ) < 0
                {
                    return 1i32.wrapping_neg();
                }
            }
            new_flags = PyStructSequence_New(core::ptr::addr_of_mut!(FlagsType));
            if new_flags.is_null() {
                return 1i32.wrapping_neg();
            } else {
                if set_flags_from_config(interp, new_flags) < 0 {
                    Py_DECREF!(new_flags);
                    return 1i32.wrapping_neg();
                } else {
                    res = _PySys_SetAttr(
                        core::ptr::addr_of_mut!(
                            _PyRuntime
                                .static_objects
                                .singletons
                                .strings
                                .identifiers
                                ._py_flags
                                ._ascii
                                .ob_base
                        ),
                        new_flags,
                    );
                    Py_DECREF!(new_flags);
                    if res < 0 {
                        return 1i32.wrapping_neg();
                    } else {
                        loop {
                            v = PyBool_FromLong(((*config).write_bytecode == 0) as i64);
                            if v.is_null() {
                                break '__join_0;
                            } else {
                                res = PyDict_SetItemString(
                                    sysdict,
                                    c"dont_write_bytecode".as_ptr(),
                                    v,
                                );
                                Py_DECREF!(v);
                                if res < 0 {
                                    break '__join_0;
                                } else {
                                    if true {
                                        break;
                                    }
                                }
                            }
                        }
                        if _PyErr_Occurred(tstate).is_null() {
                            return 0;
                        }
                    }
                }
            }
        }
        return 1i32.wrapping_neg();
    }
}

/// Set up a preliminary stderr printer until we have enough
/// infrastructure for the io module in place.
///
/// Use UTF-8/backslashreplace and ignore EAGAIN errors.
fn _PySys_SetPreliminaryStderr(sysdict: *mut _object) -> __SlateRecord_initconfig_h_19 {
    let mut __slate_compound_1: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_2: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut pstderr: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        pstderr = PyFile_NewStdPrinter(fileno(slate_support::stderr()));
        if !pstderr.is_null() {
            if PyDict_SetItem(
                sysdict,
                core::ptr::addr_of_mut!(
                    _PyRuntime
                        .static_objects
                        .singletons
                        .strings
                        .identifiers
                        ._py_stderr
                        ._ascii
                        .ob_base
                ),
                pstderr,
            ) >= 0
            {
                if PyDict_SetItemString(sysdict, c"__stderr__".as_ptr(), pstderr) >= 0 {
                    Py_DECREF!(pstderr);
                    return *{
                        __slate_compound_2 = {
                            let mut __t1: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                            __t1._type = 0u32;
                            __t1
                        };
                        core::ptr::addr_of_mut!(__slate_compound_2)
                    };
                }
            }
        }
        Py_XDECREF!(pstderr);
        return *{
            __slate_compound_1 = {
                let mut __t0: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                __t0._type = 1u32;
                __t0.func = _PyStatus_GET_FUNC__44!() as *const i8;
                __t0.err_msg = c"can't set preliminary stderr".as_ptr();
                __t0
            };
            core::ptr::addr_of_mut!(__slate_compound_1)
        };
    }
}

// [clinic input]
// module _jit
// [clinic start generated code]
//
// [clinic end generated code: output=da39a3ee5e6b4b0d input=10952f74d7bbd972]
static _jit_doc: __SlateAlign16<[i8; 50]> = __SlateAlign16([
    85i8, 116i8, 105i8, 108i8, 105i8, 116i8, 105i8, 101i8, 115i8, 32i8, 102i8, 111i8, 114i8, 32i8,
    111i8, 98i8, 115i8, 101i8, 114i8, 118i8, 105i8, 110i8, 103i8, 32i8, 106i8, 117i8, 115i8, 116i8,
    45i8, 105i8, 110i8, 45i8, 116i8, 105i8, 109i8, 101i8, 32i8, 99i8, 111i8, 109i8, 112i8, 105i8,
    108i8, 97i8, 116i8, 105i8, 111i8, 110i8, 46i8, 0i8,
]);

// [clinic input]
// _jit.is_available -> bool
// Return True if the current Python executable supports JIT compilation, and False otherwise.
// [clinic start generated code]
/// [clinic end generated code: output=6849a9cd2ff4aac9 input=03add84aa8347cf1]
fn _jit_is_available_impl(module: *mut _object) -> i32 {
    0
}

// [clinic input]
// _jit.is_enabled -> bool
// Return True if JIT compilation is enabled for the current Python process (implies sys._jit.is_available()), and False otherwise.
// [clinic start generated code]
/// [clinic end generated code: output=55865f8de993fe42 input=02439394da8e873f]
fn _jit_is_enabled_impl(module: *mut _object) -> i32 {
    (unsafe { (*_PyInterpreterState_GET()).jit }) as i32
}

// [clinic input]
// _jit.is_active -> bool
// Return True if the topmost Python frame is currently executing JIT code (implies sys._jit.is_enabled()), and False otherwise.
// [clinic start generated code]
/// [clinic end generated code: output=7facca06b10064d4 input=be2fcd8a269d9b72]
fn _jit_is_active_impl(module: *mut _object) -> i32 {
    !unsafe { (*_PyThreadState_GET()).current_executor }.is_null() as i32
}

static mut _jit_methods: __SlateAlign16<[PyMethodDef; 4]> = __SlateAlign16([
    {
        let mut __t0: PyMethodDef = unsafe { core::mem::zeroed() };
        __t0.ml_name = (b"is_available\0".as_ptr() as *mut i8) as *const i8;
        __t0.ml_meth = Some(_jit_is_available);
        __t0.ml_flags = 4i32;
        __t0.ml_doc = unsafe { core::ptr::addr_of!(_jit_is_available__doc__.0) as *const i8 };
        __t0
    },
    {
        let mut __t1: PyMethodDef = unsafe { core::mem::zeroed() };
        __t1.ml_name = (b"is_enabled\0".as_ptr() as *mut i8) as *const i8;
        __t1.ml_meth = Some(_jit_is_enabled);
        __t1.ml_flags = 4i32;
        __t1.ml_doc = unsafe { core::ptr::addr_of!(_jit_is_enabled__doc__.0) as *const i8 };
        __t1
    },
    {
        let mut __t2: PyMethodDef = unsafe { core::mem::zeroed() };
        __t2.ml_name = (b"is_active\0".as_ptr() as *mut i8) as *const i8;
        __t2.ml_meth = Some(_jit_is_active);
        __t2.ml_flags = 4i32;
        __t2.ml_doc = unsafe { core::ptr::addr_of!(_jit_is_active__doc__.0) as *const i8 };
        __t2
    },
    {
        let mut __t3: PyMethodDef = unsafe { core::mem::zeroed() };
        __t3.ml_name = core::ptr::null::<i8>();
        __t3
    },
]);

static mut _jit_module: PyModuleDef = {
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
    __t0.m_name = (b"sys._jit\0".as_ptr() as *mut i8) as *const i8;
    __t0.m_doc = unsafe { core::ptr::addr_of!(_jit_doc.0) as *const i8 };
    __t0.m_size = 1i32.wrapping_neg() as i64;
    __t0.m_methods = unsafe { core::ptr::addr_of_mut!(_jit_methods.0) as *mut PyMethodDef };
    __t0
};

/// Create sys module without all attributes.
/// _PySys_UpdateConfig() should be called later to add remaining attributes.
#[unsafe(no_mangle)]
extern "C-unwind" fn _PySys_Create(
    tstate: *mut _ts,
    sysmod_p: *mut *mut _object,
) -> __SlateRecord_initconfig_h_19 {
    let mut __slate_compound_8: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_9: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_10: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut _jit: *mut _object = unsafe { core::mem::zeroed() };
    let mut err: i32 = unsafe { core::mem::zeroed() };
    let mut monitoring: *mut _object = unsafe { core::mem::zeroed() };
    let mut status: core::mem::MaybeUninit<__SlateRecord_initconfig_h_19> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_3: *mut __SlateRecord_initconfig_h_19 =
        core::ptr::addr_of_mut!(status) as *mut __SlateRecord_initconfig_h_19;
    let mut sysdict: *mut _object = unsafe { core::mem::zeroed() };
    let mut sysmod: *mut _object = unsafe { core::mem::zeroed() };
    let mut modules: *mut _object = unsafe { core::mem::zeroed() };
    let mut interp: *mut _is = unsafe { core::mem::zeroed() };
    unsafe {
        interp = (*tstate).interp;
        modules = _PyImport_InitModules(interp);
        if !modules.is_null() {
            sysmod = _PyModule_CreateInitialized(core::ptr::addr_of_mut!(sysmodule), 1013i32);
            if sysmod.is_null() {
                return *{
                    __slate_compound_10 = {
                        let mut __t2: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                        __t2._type = 1u32;
                        __t2.func = _PyStatus_GET_FUNC__14!() as *const i8;
                        __t2.err_msg = c"failed to create a module object".as_ptr();
                        __t2
                    };
                    core::ptr::addr_of_mut!(__slate_compound_10)
                };
            } else {
                sysdict = PyModule_GetDict(sysmod);
                if !sysdict.is_null() {
                    (*interp).sysdict = Py_NewRef!(sysdict);
                    (*interp).sysdict_copy = PyDict_Copy(sysdict);
                    if !unsafe { (*interp).sysdict_copy }.is_null() {
                        if PyDict_SetItemString(sysdict, c"modules".as_ptr(), modules) >= 0 {
                            core::ptr::write(__slate_slot_3, _PySys_SetPreliminaryStderr(sysdict));
                            if _PyStatus_EXCEPTION!(*__slate_slot_3) {
                                return *__slate_slot_3;
                            } else {
                                *__slate_slot_3 = _PySys_InitCore(tstate, sysdict);
                                if _PyStatus_EXCEPTION!(*__slate_slot_3) {
                                    return *__slate_slot_3;
                                } else {
                                    if _PyImport_FixupBuiltin(
                                        tstate,
                                        sysmod,
                                        c"sys".as_ptr(),
                                        modules,
                                    ) >= 0
                                    {
                                        monitoring = _Py_CreateMonitoringObject();
                                        if !monitoring.is_null() {
                                            err = PyDict_SetItemString(
                                                sysdict,
                                                c"monitoring".as_ptr(),
                                                monitoring,
                                            );
                                            Py_DECREF!(monitoring);
                                            if err >= 0 {
                                                _jit = _PyModule_CreateInitialized(
                                                    core::ptr::addr_of_mut!(_jit_module),
                                                    1013i32,
                                                );
                                                if !_jit.is_null() {
                                                    err = PyDict_SetItemString(
                                                        sysdict,
                                                        c"_jit".as_ptr(),
                                                        _jit,
                                                    );
                                                    Py_DECREF!(_jit);
                                                    if err == 0 {
                                                        *sysmod_p = sysmod;
                                                        return *{
                                                            __slate_compound_9 = {
                                                                let mut __t1: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                                                                __t1._type = 0u32;
                                                                __t1
                                                            };
                                                            core::ptr::addr_of_mut!(
                                                                __slate_compound_9
                                                            )
                                                        };
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
            }
        }
        return *{
            __slate_compound_8 = {
                let mut __t0: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                __t0._type = 1u32;
                __t0.func = _PyStatus_GET_FUNC__14!() as *const i8;
                __t0.err_msg = c"can't initialize sys module".as_ptr();
                __t0
            };
            core::ptr::addr_of_mut!(__slate_compound_8)
        };
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PySys_FiniTypes(interp: *mut _is) {
    unsafe { _PyStructSequence_FiniBuiltin(interp, core::ptr::addr_of_mut!(VersionInfoType)) };
    unsafe { _PyStructSequence_FiniBuiltin(interp, core::ptr::addr_of_mut!(FlagsType)) };
    unsafe { _PyStructSequence_FiniBuiltin(interp, core::ptr::addr_of_mut!(Hash_InfoType)) };
    unsafe { _PyStructSequence_FiniBuiltin(interp, core::ptr::addr_of_mut!(AsyncGenHooksType)) };
}

fn makepathobject(mut path: *const i32, delim: i32) -> *mut _object {
    let mut i: i32 = 0;
    let mut n: i32 = 1;
    let mut p: *const i32 = unsafe { core::mem::zeroed() };
    p = path;
    loop {
        let __v4: *const i32 = (unsafe { wcschr(p, delim) }) as *const i32;
        p = __v4;
        if __v4.is_null() {
            break;
        }
        n = n.wrapping_add(1);
        let __v2: *const i32 = p;
        p = __v2.wrapping_offset(1 as isize);
    }
    let v: *mut _object = unsafe { PyList_New(n as i64) };
    if v.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    loop {
        p = (unsafe { wcschr(path, delim) }) as *const i32;
        if p.is_null() {
            p = path.wrapping_offset((unsafe { wcslen(path as *const i32) as u64 }) as isize);
        }
        // End of string
        let w: *mut _object =
            unsafe { PyUnicode_FromWideChar(path, p.offset_from(path as *const i32) as i64) };
        if w.is_null() {
            Py_DECREF!(v);
            return core::ptr::null_mut::<_object>();
        }
        PyList_SET_ITEM__2!(v, i, w);
        if (unsafe { *p }) == 0 {
            break;
        }
        path = p.wrapping_offset(1 as isize);
        i = i.wrapping_add(1);
    }
    v
}

/// Removed in Python 3.13 API, but kept for the stable ABI
#[unsafe(no_mangle)]
extern "C-unwind" fn PySys_SetPath(path: *const i32) {
    let __v0: *mut _object = makepathobject(path, 58i32);
    let v: *mut _object = __v0;
    if __v0.is_null() {
        Py_FatalError__11!(c"can't create sys.path".as_ptr() as *mut i8);
    }
    let interp: *mut _is = _PyInterpreterState_GET();
    if sys_set_object(
        interp,
        unsafe {
            core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py_path
                    ._ascii
                    .ob_base
            )
        },
        v,
    ) != 0
    {
        Py_FatalError__11!(c"can't assign sys.path".as_ptr() as *mut i8);
    }
    Py_DECREF!(v);
}

fn make_sys_argv(argc: i32, argv: *const *mut i32) -> *mut _object {
    let list: *mut _object = unsafe { PyList_New(argc as i64) };
    if list.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    for i in 0..argc as i64 {
        let v: *mut _object = unsafe {
            PyUnicode_FromWideChar(
                *argv.wrapping_offset(i as isize) as *const i32,
                1i32.wrapping_neg() as i64,
            )
        };
        if v.is_null() {
            Py_DECREF!(list);
            return core::ptr::null_mut::<_object>();
        }
        PyList_SET_ITEM!(list, i, v);
    }
    list
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PySys_SetArgvEx(mut argc: i32, mut argv: *mut *mut i32, updatepath: i32) {
    let mut empty_argv: [*mut i32; 1] =
        [unsafe { core::ptr::addr_of_mut!(__slate_str4485) as *mut i32 }];
    let tstate: *mut _ts = _PyThreadState_GET();
    if argc < 1 || argv.is_null() {
        // Ensure at least one (empty) argument is seen
        argv = empty_argv.as_mut_ptr() as *mut *mut i32;
        argc = 1;
    }
    let av: *mut _object = make_sys_argv(argc, argv as *const *mut i32);
    if av.is_null() {
        Py_FatalError__3!(c"no mem for sys.argv".as_ptr() as *mut i8);
    }
    if sys_set_object_str(unsafe { (*tstate).interp }, c"argv".as_ptr(), av) != 0 {
        Py_DECREF!(av);
        Py_FatalError__3!(c"can't assign sys.argv".as_ptr() as *mut i8);
    }
    Py_DECREF!(av);
    if updatepath != 0 {
        // If argv[0] is not '-c' nor '-m', prepend argv[0] to sys.path.
        // If argv[0] is a symlink, use the real path.
        let argv_list: __SlateRecord_initconfig_h_36 = __SlateRecord_initconfig_h_36 {
            length: argc as i64,
            items: argv,
        };
        let mut path0: *mut _object = core::ptr::null_mut::<_object>();
        if (unsafe {
            _PyPathConfig_ComputeSysPath0(
                core::ptr::addr_of!(argv_list),
                core::ptr::addr_of_mut!(path0),
            )
        }) != 0
        {
            if path0.is_null() {
                Py_FatalError__3!(c"can't compute path0 from argv".as_ptr() as *mut i8);
            }
            let mut sys_path: *mut _object = unsafe { core::mem::zeroed() };
            if _PySys_GetOptionalAttr(
                unsafe {
                    core::ptr::addr_of_mut!(
                        _PyRuntime
                            .static_objects
                            .singletons
                            .strings
                            .identifiers
                            ._py_path
                            ._ascii
                            .ob_base
                    )
                },
                core::ptr::addr_of_mut!(sys_path),
            ) < 0
            {
                Py_FatalError__3!(c"can't get sys.path".as_ptr() as *mut i8);
            } else {
                if !sys_path.is_null() {
                    if (unsafe { PyList_Insert(sys_path, 0i64, path0) }) < 0 {
                        Py_FatalError__3!(c"can't prepend path0 to sys.path".as_ptr() as *mut i8);
                    }
                    Py_DECREF!(sys_path);
                }
            }
            Py_DECREF!(path0);
        }
    }
}

static mut __slate_str4485: [i32; 1] = [0];

#[unsafe(no_mangle)]
extern "C-unwind" fn PySys_SetArgv(argc: i32, argv: *mut *mut i32) {
    PySys_SetArgvEx(argc, argv, ((unsafe { Py_IsolatedFlag }) == 0) as i32);
}

// Reimplementation of PyFile_WriteString() no calling indirectly
// PyErr_CheckSignals(): avoid the call to PyObject_Str().
fn sys_pyfile_write_unicode(unicode: *mut _object, file: *mut _object) -> i32 {
    if file.is_null() {
        return 1i32.wrapping_neg();
    }
    let result: *mut _object = PyObject_CallMethodOneArg(
        file,
        unsafe {
            core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py_write
                    ._ascii
                    .ob_base
            )
        },
        unicode,
    );
    if result.is_null() {
        return 1i32.wrapping_neg();
    }
    Py_DECREF!(result);
    0
}

fn sys_pyfile_write(text: *const i8, file: *mut _object) -> i32 {
    let mut unicode: *mut _object = core::ptr::null_mut::<_object>();
    let mut err: i32 = 0;
    if file.is_null() {
        return 1i32.wrapping_neg();
    }
    unicode = unsafe { PyUnicode_FromString(text) };
    if unicode.is_null() {
        return 1i32.wrapping_neg();
    }
    err = sys_pyfile_write_unicode(unicode, file);
    Py_DECREF!(unicode);
    err
}

// APIs to write to sys.stdout or sys.stderr using a printf-like interface.
// Adapted from code submitted by Just van Rossum.
//
// PySys_WriteStdout(format, ...)
// PySys_WriteStderr(format, ...)
//
//    The first function writes to sys.stdout; the second to sys.stderr.  When
//    there is a problem, they write to the real (C level) stdout or stderr;
//    no exceptions are raised.
//
//    PyErr_CheckSignals() is not called to avoid the execution of the Python
//    signal handlers: they may raise a new exception whereas sys_write()
//    ignores all exceptions.
//
//    Both take a printf-style format string as their first argument followed
//    by a variable length argument list determined by the format string.
//
//    *** WARNING ***
//
//    The format should limit the total size of the formatted output string to
//    1000 bytes.  In particular, this means that no unrestricted "%s" formats
//    should occur; these should be limited using "%.<N>s where <N> is a
//    decimal number calculated so that <N> plus the maximum size of other
//    formatted text does not exceed 1000 bytes.  Also watch out for "%f",
//    which can print hundreds of digits for very large numbers.
fn sys_write(
    key: *mut _object,
    fp: *mut slate_support::FILE,
    format: *const i8,
    mut va: core::ffi::VaList<'static>,
) {
    let mut buffer: __SlateAlign16<[i8; 1001]> = __SlateAlign16([0i8; 1001]);
    let mut written: i32 = 0;
    let tstate: *mut _ts = _PyThreadState_GET();
    let exc: *mut _object = unsafe { _PyErr_GetRaisedException(tstate) };
    written = unsafe {
        PyOS_vsnprintf(
            buffer.0.as_mut_ptr() as *mut i8,
            core::mem::size_of::<[i8; 1001]>() as u64,
            format,
            va.clone(),
        )
    };
    let file: *mut _object = _PySys_GetRequiredAttr(key);
    if sys_pyfile_write(buffer.0.as_mut_ptr() as *const i8, file) != 0 {
        unsafe { _PyErr_Clear(tstate) };
        let _ = std::io::Write::flush(&mut std::io::stdout());
        unsafe { slate_support::fputs(buffer.0.as_mut_ptr() as *const i8, fp) };
        unsafe { fflush(core::ptr::null_mut()) };
    }
    if written < 0 || (written as u64) >= (core::mem::size_of::<[i8; 1001]>() as u64) {
        let truncated: *const i8 = c"... truncated".as_ptr();
        if sys_pyfile_write(truncated, file) != 0 {
            let _ = std::io::Write::flush(&mut std::io::stdout());
            unsafe { slate_support::fputs(truncated, fp) };
            unsafe { fflush(core::ptr::null_mut()) };
        }
    }
    Py_XDECREF!(file);
    unsafe { _PyErr_SetRaisedException(tstate, exc) };
}

#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn PySys_WriteStdout(format: *const i8, mut __va_args: ...) {
    let mut va: core::ffi::VaList<'static> = unsafe { core::mem::zeroed() };
    va = unsafe {
        core::mem::transmute::<core::ffi::VaList<'_>, core::ffi::VaList<'static>>(__va_args.clone())
    };
    sys_write(
        unsafe {
            core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py_stdout
                    ._ascii
                    .ob_base
            )
        },
        slate_support::stdout(),
        format,
        va.clone(),
    );
}

#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn PySys_WriteStderr(format: *const i8, mut __va_args: ...) {
    let mut va: core::ffi::VaList<'static> = unsafe { core::mem::zeroed() };
    va = unsafe {
        core::mem::transmute::<core::ffi::VaList<'_>, core::ffi::VaList<'static>>(__va_args.clone())
    };
    sys_write(
        unsafe {
            core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py_stderr
                    ._ascii
                    .ob_base
            )
        },
        slate_support::stderr(),
        format,
        va.clone(),
    );
}

fn sys_format(
    key: *mut _object,
    fp: *mut slate_support::FILE,
    format: *const i8,
    mut va: core::ffi::VaList<'static>,
) {
    let tstate: *mut _ts = _PyThreadState_GET();
    let exc: *mut _object = unsafe { _PyErr_GetRaisedException(tstate) };
    let message: *mut _object = unsafe { PyUnicode_FromFormatV(format, va.clone()) };
    if !message.is_null() {
        let file: *mut _object = _PySys_GetRequiredAttr(key);
        if sys_pyfile_write_unicode(message, file) != 0 {
            unsafe { _PyErr_Clear(tstate) };
            let utf8: *const i8 = unsafe { PyUnicode_AsUTF8(message) };
            if !utf8.is_null() {
                let _ = std::io::Write::flush(&mut std::io::stdout());
                unsafe { slate_support::fputs(utf8, fp) };
                unsafe { fflush(core::ptr::null_mut()) };
            }
        }
        Py_XDECREF!(file);
        Py_DECREF!(message);
    }
    unsafe { _PyErr_SetRaisedException(tstate, exc) };
}

#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn PySys_FormatStdout(format: *const i8, mut __va_args: ...) {
    let mut va: core::ffi::VaList<'static> = unsafe { core::mem::zeroed() };
    va = unsafe {
        core::mem::transmute::<core::ffi::VaList<'_>, core::ffi::VaList<'static>>(__va_args.clone())
    };
    sys_format(
        unsafe {
            core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py_stdout
                    ._ascii
                    .ob_base
            )
        },
        slate_support::stdout(),
        format,
        va.clone(),
    );
}

#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn PySys_FormatStderr(format: *const i8, mut __va_args: ...) {
    let mut va: core::ffi::VaList<'static> = unsafe { core::mem::zeroed() };
    va = unsafe {
        core::mem::transmute::<core::ffi::VaList<'_>, core::ffi::VaList<'static>>(__va_args.clone())
    };
    sys_format(
        unsafe {
            core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py_stderr
                    ._ascii
                    .ob_base
            )
        },
        slate_support::stderr(),
        format,
        va.clone(),
    );
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PySys_SetIntMaxStrDigits(maxdigits: i32) -> i32 {
    if maxdigits != 0 && maxdigits < 640 {
        unsafe {
            PyErr_Format(
                PyExc_ValueError,
                c"maxdigits must be >= %d or 0 for unlimited".as_ptr(),
                640i32,
            )
        };
        return 1i32.wrapping_neg();
    }
    // Set sys.flags.int_max_str_digits
    let pos: i64 = 17;
    if _PySys_SetFlagInt(pos, maxdigits) < 0 {
        return 1i32.wrapping_neg();
    }
    // Set PyInterpreterState.long_state.max_str_digits
    //
    // and PyInterpreterState.config.int_max_str_digits.
    let interp: *mut _is = _PyInterpreterState_GET();
    _Py_atomic_store_int(
        unsafe { core::ptr::addr_of_mut!((*interp).long_state.max_str_digits) },
        maxdigits,
    );
    _Py_atomic_store_int(
        unsafe { core::ptr::addr_of_mut!((*interp).config.int_max_str_digits) },
        maxdigits,
    );
    0
}
