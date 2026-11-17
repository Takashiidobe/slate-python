//! Float object implementation
//! The size of 'void *', as computed by sizeof.
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
//! Chosen so that _PyObjectStackChunk is a power-of-two size.
//! Largest tuple to save on freelist
//! leads to warnings... __attribute__((cdecl))
//! Various defines for all used memory orders in mimalloc
//! Main tuning parameters for segment and page sizes
//!
//! Sizes for 64-bit (usually divide by two for 32-bit)
//! Inlined definitions
//! Total tool ids available
//! Almost all errors causing Python initialization to fail
//!
//! other fields are set to 0
//! Bit flags for _gc_prev
//!
//! Bit 0 is set when tp_finalize is called
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
// Prevent using an expression as a l-value.
//
// For example, "int x; _Py_RVALUE(x) = 1;" fails with a compiler error.
// if the result is an integer add ".0"
// 64KiB  (32KiB on 32-bit)
// 1024
// Used as a special value to encode block sizes in 32 bits.
// 35 == mi_segment_bin(MI_SLICES_PER_SEGMENT)
// initialize a local variable to zero; use memset as compilers optimize constant sized memset's
macro_rules! BITS_TO_PTR {
    ($REF:expr) => {
        (unsafe { $REF.bits }) as *mut _object
    };
}

/// Macro and helper that convert PyObject obj to a C double and store
/// the value in dbl.  If conversion to double raises an exception, obj is
/// set to NULL, and the function invoking this macro returns NULL.  If
/// obj is not of float or int type, Py_NotImplemented is incref'ed,
/// stored in obj, and returned from the function invoking this macro.
macro_rules! CONVERT_TO_DOUBLE {
    ($obj:expr, $dbl:expr) => {{
        if PyFloat_Check!($obj) != 0 {
            $dbl = PyFloat_AS_DOUBLE!($obj);
        } else {
            if _Py_convert_int_to_double(
                core::ptr::addr_of_mut!($obj),
                core::ptr::addr_of_mut!($dbl),
            ) < 0
            {
                return $obj;
            }
        }
    }};
}

/// determine whether x is an odd integer or not;  assumes that
/// x is not an infinity or nan.
macro_rules! DOUBLE_IS_ODD_INTEGER {
    ($x:expr) => {
        (unsafe { fmod(fabs($x), 2.0f64) }) == 1.0f64
    };
}

macro_rules! FT_ATOMIC_STORE_PTR_RELAXED {
    ($value:expr, $new_value:expr) => {
        unsafe {
            $value = $new_value;
        }
    };
}

macro_rules! HEX_DIGIT {
    ($j:expr, $fdigits:expr, $coeff_end:expr) => {
        hex_from_char(
            (unsafe {
                *if ($j) < $fdigits {
                    $coeff_end.wrapping_offset(-(($j) as isize))
                } else {
                    $coeff_end
                        .wrapping_offset(-(1 as isize))
                        .wrapping_offset(-(($j) as isize))
                }
            }) as u8,
        )
    };
}

macro_rules! OBJECT_STAT_INC {
    () => {};
}

macro_rules! PyByteArray_AS_STRING__2 {
    ($self:expr) => {
        PyByteArray_AS_STRING(_PyObject_CAST!($self))
    };
}

/// Type check macros
macro_rules! PyByteArray_Check {
    ($self:expr) => {
        PyObject_TypeCheck!($self, unsafe { core::ptr::addr_of_mut!(PyByteArray_Type) })
    };
}

macro_rules! PyByteArray_GET_SIZE {
    ($self:expr) => {
        PyByteArray_GET_SIZE(_PyObject_CAST!($self))
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

macro_rules! PyFloat_AS_DOUBLE {
    ($op:expr) => {
        PyFloat_AS_DOUBLE(_PyObject_CAST!($op))
    };
}

macro_rules! PyFloat_AS_DOUBLE__2 {
    ($op:expr) => {
        PyFloat_AS_DOUBLE(_PyObject_CAST__2!($op))
    };
}

macro_rules! PyFloat_CheckExact {
    ($op:expr) => {
        Py_IS_TYPE!($op, unsafe { core::ptr::addr_of_mut!(PyFloat_Type) })
    };
}

macro_rules! PyFloat_Check {
    ($op:expr) => {
        PyObject_TypeCheck!($op, unsafe { core::ptr::addr_of_mut!(PyFloat_Type) })
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

macro_rules! PyStackRef_CheckValid {
    () => {};
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

macro_rules! PyUnicode_GET_LENGTH {
    ($op:expr) => {
        PyUnicode_GET_LENGTH(_PyObject_CAST!($op))
    };
}

macro_rules! PyVectorcall_NARGS {
    ($n:expr) => {
        _PyVectorcall_NARGS($n)
    };
}

/// Absolute value of the number x
macro_rules! Py_ABS__2 {
    ($x:expr) => {
        if ($x) < 0 { $x.wrapping_neg() } else { $x }
    };
}

/// Argument must be a char or an int in [-128, 127] or [0, 255].
macro_rules! Py_CHARMASK {
    ($c:expr) => {
        ((($c) as i32) & 255) as u8
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

macro_rules! Py_DECREF_MORTAL_SPECIALIZED {
    ($op:expr, $destruct:expr) => {
        Py_DECREF_MORTAL_SPECIALIZED(_PyObject_CAST!($op), $destruct);
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

macro_rules! Py_ISSPACE {
    ($c:expr) => {
        (unsafe { _Py_ctype_table[(Py_CHARMASK!($c) as i32) as usize] }) & 8
    };
}

macro_rules! Py_IS_TYPE {
    ($ob:expr, $type:expr) => {
        Py_IS_TYPE(_PyObject_CAST!($ob), $type)
    };
}

/// Maximum value between x and y
macro_rules! Py_MAX {
    ($x:expr, $y:expr) => {
        if ($x) > $y { $x } else { $y }
    };
}

/// Minimum value between x and y
macro_rules! Py_MIN {
    ($x:expr, $y:expr) => {
        if ($x) > $y { $y } else { $x }
    };
}

macro_rules! Py_NewRef {
    ($obj:expr) => {
        _Py_NewRef(_PyObject_CAST!($obj))
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

macro_rules! Py_SET_TYPE {
    ($ob:expr, $type:expr) => {
        Py_SET_TYPE(_PyObject_CAST!($ob), $type);
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

macro_rules! SetDblFlag {
    ($FLAG:expr, $floatinfo:expr, $pos:expr) => {{
        SetFlag__2!(PyFloat_FromDouble($FLAG), $floatinfo, $pos);
    }};
}

macro_rules! SetFlag__2 {
    ($CALL:expr, $floatinfo:expr, $pos:expr) => {{
        loop {
            let flag: *mut _object = $CALL;
            if flag.is_null() {
                Py_CLEAR__2!($floatinfo);
                return core::ptr::null_mut::<_object>();
            }
            let __v: i32 = $pos;
            let __v2: i32 = __v.wrapping_add(1);
            $pos = __v2;
            unsafe { PyStructSequence_SetItem($floatinfo, __v as i64, flag) };
            if true {
                break;
            }
        }
    }};
}

macro_rules! SetIntFlag {
    ($FLAG:expr, $floatinfo:expr, $pos:expr) => {{
        SetFlag__2!(unsafe { PyLong_FromLong(($FLAG) as i64) }, $floatinfo, $pos);
    }};
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

macro_rules! _PyArg_CheckPositional__2 {
    ($funcname:expr, $nargs:expr, $min:expr, $max:expr) => {{
        let __v: bool = !_Py_ANY_VARARGS!($max) && (($min) as i64) <= $nargs;
        let __v2: bool = __v && ($nargs) <= (($max) as i64);
        __v2 || (unsafe {
            _PyArg_CheckPositional(
                ($funcname) as *const i8,
                $nargs,
                ($min) as i64,
                ($max) as i64,
            )
        }) != 0
    }};
}

macro_rules! _PyArg_NoKeywords {
    ($funcname:expr, $kwargs:expr) => {{
        let __v: bool = $kwargs.is_null()
            || (unsafe { _PyArg_NoKeywords(($funcname) as *const i8, $kwargs) }) != 0;
        __v
    }};
}

macro_rules! _PyArg_NoKwnames {
    ($funcname:expr, $kwnames:expr) => {{
        let __v: bool = $kwnames.is_null()
            || (unsafe { _PyArg_NoKwnames(($funcname) as *const i8, $kwnames) }) != 0;
        __v
    }};
}

/// Macros and static inline functions, trading safety for speed
macro_rules! _PyByteArray_CAST {
    ($op:expr) => {
        _Py_CAST!(*mut __SlateRecord_bytearrayobject_h_12, $op)
    };
}

/// Macros and static inline functions, trading safety for speed
macro_rules! _PyBytes_CAST {
    ($op:expr) => {
        _Py_CAST!(*mut __SlateRecord_bytesobject_h_15, $op)
    };
}

macro_rules! _PyFloat_CAST {
    ($op:expr) => {
        _Py_CAST!(*mut __SlateRecord_floatobject_h_8, $op)
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

macro_rules! _PyReftracerTrack {
    ($obj:expr, $operation:expr) => {
        loop {
            let tracer: *mut _reftracer_runtime_state =
                unsafe { core::ptr::addr_of_mut!(_PyRuntime.ref_tracer) };
            if (unsafe { (*tracer).tracer_func }) != None {
                let data: *mut () = unsafe { (*tracer).tracer_data };
                unsafe {
                    unsafe { (*tracer).tracer_func }.unwrap()($obj, ($operation) as u32, data)
                };
            }
            if true {
                break;
            }
        }
    };
}

macro_rules! _PyStatus_GET_FUNC__61 {
    () => {
        c"_PyFloat_InitTypes".as_ptr() as *mut i8
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

/// Pops a PyObject from the freelist, returns NULL if the freelist is empty.
macro_rules! _Py_FREELIST_POP {
    ($TYPE:ty, $NAME:ident) => {
        _Py_CAST!(
            *mut $TYPE,
            _PyFreeList_Pop(unsafe { core::ptr::addr_of_mut!((*_Py_freelists_GET()).$NAME) })
        )
    };
}

macro_rules! _Py_FREELIST_SIZE {
    ($NAME:ident) => {
        (unsafe { (*_Py_freelists_GET()).$NAME.size }) as i32
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

macro_rules! _Py_STATIC_CAST {
    ($type:ty, $expr:expr) => {
        ($expr) as $type
    };
}

unsafe extern "C" {
    static mut PyLong_Type: _typeobject;
    static mut PyType_Type: _typeobject;
    static mut _Py_NoneStruct: _object;
    static mut _Py_NotImplementedStruct: _object;
    static mut PyByteArray_Type: _typeobject;
    static mut _PyByteArray_empty_string: [i8; 0];
    static mut PyUnicode_Type: _typeobject;
    static mut PyExc_OverflowError: *mut _object;
    static mut PyExc_RuntimeError: *mut _object;
    static mut PyExc_SystemError: *mut _object;
    static mut PyExc_TypeError: *mut _object;
    static mut PyExc_ValueError: *mut _object;
    static mut PyExc_ZeroDivisionError: *mut _object;
    static mut PyExc_DeprecationWarning: *mut _object;
    static mut _Py_FalseStruct: _longobject;
    static mut _Py_TrueStruct: _longobject;
    static mut PyComplex_Type: _typeobject;
    static mut Py_hexdigits: *const i8;
    static mut _Py_ctype_table: [u32; 256];
    #[thread_local]
    static mut _Py_tss_tstate: *mut _ts;
    static mut _PyRuntime: pyruntimestate;
    fn exp(arg0: f64) -> f64;
    fn frexp(arg0: f64, arg1: *mut i32) -> f64;
    fn ldexp(arg0: f64, arg1: i32) -> f64;
    fn modf(arg0: f64, arg1: *mut f64) -> f64;
    fn pow(arg0: f64, arg1: f64) -> f64;
    fn ceil(arg0: f64) -> f64;
    fn fabs(arg0: f64) -> f64;
    fn floor(arg0: f64) -> f64;
    fn fmod(arg0: f64, arg1: f64) -> f64;
    fn copysign(arg0: f64, arg1: f64) -> f64;
    fn round(arg0: f64) -> f64;
    #[link_name = "__isoc23_strtol"]
    fn strtol(arg0: *const i8, arg1: *mut *mut i8, arg2: i32) -> i64;
    fn div(arg0: i32, arg1: i32) -> __SlateRecord_stdlib_h_67;
    fn PyMem_Malloc(arg0: u64) -> *mut ();
    fn PyMem_Free(arg0: *mut ());
    fn PyObject_GetBuffer(
        arg0: *mut _object,
        arg1: *mut __SlateRecord_pybuffer_h_33,
        arg2: i32,
    ) -> i32;
    fn PyBuffer_Release(arg0: *mut __SlateRecord_pybuffer_h_33);
    fn PyType_IsSubtype(arg0: *mut _typeobject, arg1: *mut _typeobject) -> i32;
    fn PyObject_RichCompareBool(arg0: *mut _object, arg1: *mut _object, arg2: i32) -> i32;
    fn PyObject_GenericGetAttr(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn _Py_NewReference(arg0: *mut _object);
    fn _Py_Dealloc(arg0: *mut _object);
    fn PyObject_Malloc(arg0: u64) -> *mut ();
    fn PyObject_Free(arg0: *mut ());
    fn _Py_HashDouble(arg0: *mut _object, arg1: f64) -> i64;
    fn PyBytes_FromStringAndSize(arg0: *const i8, arg1: i64) -> *mut _object;
    fn PyUnicode_FromString(arg0: *const i8) -> *mut _object;
    fn PyUnicode_FromFormat(arg0: *const i8, ...) -> *mut _object;
    fn PyUnicode_AsUTF8AndSize(arg0: *mut _object, arg1: *mut i64) -> *const i8;
    fn _PyUnicodeWriter_Init(arg0: *mut __SlateRecord_unicodeobject_h_539);
    fn _PyUnicodeWriter_Finish(arg0: *mut __SlateRecord_unicodeobject_h_539) -> *mut _object;
    fn _PyUnicodeWriter_Dealloc(arg0: *mut __SlateRecord_unicodeobject_h_539);
    fn PyErr_SetString(arg0: *mut _object, arg1: *const i8);
    fn PyErr_Occurred() -> *mut _object;
    fn PyErr_BadArgument() -> i32;
    fn PyErr_NoMemory() -> *mut _object;
    fn PyErr_SetFromErrno(arg0: *mut _object) -> *mut _object;
    fn PyErr_Format(arg0: *mut _object, arg1: *const i8, ...) -> *mut _object;
    fn PyOS_snprintf(arg0: *mut i8, arg1: u64, arg2: *const i8, ...) -> i32;
    fn PyLong_FromLong(arg0: i64) -> *mut _object;
    fn PyLong_FromDouble(arg0: f64) -> *mut _object;
    fn PyLong_AsDouble(arg0: *mut _object) -> f64;
    fn PyLong_GetSign(arg0: *mut _object, arg1: *mut i32) -> i32;
    fn _PyLong_NumBits(arg0: *mut _object) -> i64;
    fn PyBool_FromLong(arg0: i64) -> *mut _object;
    fn PyTuple_Pack(arg0: i64, ...) -> *mut _object;
    fn PyErr_WarnFormat(arg0: *mut _object, arg1: i64, arg2: *const i8, ...) -> i32;
    fn PyStructSequence_New(arg0: *mut _typeobject) -> *mut _object;
    fn PyStructSequence_SetItem(arg0: *mut _object, arg1: i64, arg2: *mut _object);
    fn Py_BuildValue(arg0: *const i8, ...) -> *mut _object;
    fn PyNumber_AsSsize_t(arg0: *mut _object, arg1: *mut _object) -> i64;
    fn PyNumber_Float(arg0: *mut _object) -> *mut _object;
    fn PyObject_CallOneArg(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn PyOS_string_to_double(arg0: *const i8, arg1: *mut *mut i8, arg2: *mut _object) -> f64;
    fn PyOS_double_to_string(arg0: f64, arg1: i8, arg2: i32, arg3: i32, arg4: *mut i32) -> *mut i8;
    fn _PyNumber_Index(arg0: *mut _object) -> *mut _object;
    fn _Py_get_387controlword() -> u16;
    fn _Py_set_387controlword(arg0: u16);
    fn _Py_dg_strtod(arg0: *const i8, arg1: *mut *mut i8) -> f64;
    fn _Py_dg_dtoa(
        arg0: f64,
        arg1: i32,
        arg2: i32,
        arg3: *mut i32,
        arg4: *mut i32,
        arg5: *mut *mut i8,
    ) -> *mut i8;
    fn _Py_dg_freedtoa(arg0: *mut i8);
    fn _PyUnicode_FromASCII(arg0: *const i8, arg1: i64) -> *mut _object;
    fn _PyUnicode_TransformDecimalAndSpaceToASCII(arg0: *mut _object) -> *mut _object;
    fn _PyFloat_FormatAdvancedWriter(
        arg0: *mut __SlateRecord_unicodeobject_h_539,
        arg1: *mut _object,
        arg2: *mut _object,
        arg3: i64,
        arg4: i64,
    ) -> i32;
    fn _Py_string_to_number_with_underscores(
        arg0: *const i8,
        arg1: i64,
        arg2: *const i8,
        arg3: *mut _object,
        arg4: *mut (),
        arg5: Option<unsafe extern "C-unwind" fn(*const i8, i64, *mut ()) -> *mut _object>,
    ) -> *mut _object;
    fn _Py_parse_inf_or_nan(arg0: *const i8, arg1: *mut *mut i8) -> f64;
    fn _PyInterpreterState_Main() -> *mut _is;
    fn _PyArg_NoKwnames(arg0: *const i8, arg1: *mut _object) -> i32;
    fn _PyArg_NoKeywords(arg0: *const i8, arg1: *mut _object) -> i32;
    fn _PyArg_CheckPositional(arg0: *const i8, arg1: i64, arg2: i64, arg3: i64) -> i32;
    fn _PyArg_BadArgument(arg0: *const i8, arg1: *const i8, arg2: *const i8, arg3: *mut _object);
    fn _PyDebugAllocatorStats(
        arg0: *mut slate_support::FILE,
        arg1: *const i8,
        arg2: i32,
        arg3: u64,
    );
    fn _PyStructSequence_InitBuiltinWithFlags(
        arg0: *mut _is,
        arg1: *mut _typeobject,
        arg2: *mut PyStructSequence_Desc,
        arg3: u64,
    ) -> i32;
    fn _PyStructSequence_FiniBuiltin(arg0: *mut _is, arg1: *mut _typeobject);
    fn __builtin_flt_rounds() -> i32;
    fn __slate_fenv_flt_rounds() -> i32;
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
struct __SlateRecord_stdlib_h_67 {
    quot: i32,
    rem: i32,
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

fn Py_SET_TYPE(ob: *mut _object, r#type: *mut _typeobject) {
    unsafe {
        (*ob).ob_type = r#type;
    }
}

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
// Set if the type allows subclassing
// This undocumented flag gives certain built-ins their unique pattern-matching
//
// behavior, which allows a single positional subpattern to match against the
//
// subject itself (rather than a mapped attribute on it):
// Rich comparison opcodes
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
struct __SlateRecord_unicodeobject_h_163_2 {
    __slate_bits_0: __slate_bits::__SlateBits139U0,
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
struct __SlateRecord_bytearrayobject_h_12 {
    ob_base: __SlateRecord_object_h_172,
    ob_alloc: i64,
    ob_bytes: *mut i8,
    ob_start: *mut i8,
    ob_exports: i64,
}

fn PyByteArray_AS_STRING(op: *mut _object) -> *mut i8 {
    let self_: *mut __SlateRecord_bytearrayobject_h_12 = _PyByteArray_CAST!(op);
    if Py_SIZE!(self_) != 0 {
        return unsafe { (*self_).ob_start };
    }
    unsafe { core::ptr::addr_of_mut!(_PyByteArray_empty_string) as *mut i8 }
}

fn PyByteArray_GET_SIZE(op: *mut _object) -> i64 {
    let self_: *mut __SlateRecord_bytearrayobject_h_12 = _PyByteArray_CAST!(op);
    Py_SIZE!(self_)
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
struct _PyExecutorObject {}

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
struct __SlateRecord_unicodeobject_h_539 {
    buffer: *mut _object,
    data: *mut (),
    kind: i32,
    maxchar: u32,
    size: i64,
    pos: i64,
    min_length: i64,
    min_char: u32,
    overallocate: u8,
    readonly: u8,
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

#[unsafe(no_mangle)]
static mut PyFloat_Type: _typeobject = {
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
    __t0.tp_name = (b"float\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_floatobject_h_8>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(float_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = Some(float_repr);
    __t0.tp_as_number = unsafe { core::ptr::addr_of_mut!(float_as_number) };
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = Some(float_hash);
    __t0.tp_call = None;
    __t0.tp_str = None;
    __t0.tp_getattro = unsafe {
        core::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
        >(PyObject_GenericGetAttr as *const ())
    };
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 4195328u64;
    __t0.tp_doc = unsafe { core::ptr::addr_of!(float_new__doc__.0) as *const i8 };
    __t0.tp_traverse = None;
    __t0.tp_clear = None;
    __t0.tp_richcompare = Some(float_richcompare);
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = unsafe { core::ptr::addr_of_mut!(float_methods.0) as *mut PyMethodDef };
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = unsafe { core::ptr::addr_of_mut!(float_getset.0) as *mut PyGetSetDef };
    __t0.tp_base = core::ptr::null_mut::<_typeobject>();
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = 0i64;
    __t0.tp_init = None;
    __t0.tp_alloc = None;
    __t0.tp_new = Some(float_new);
    __t0.tp_version_tag = 2u32;
    __t0.tp_vectorcall = Some(float_vectorcall);
    __t0
};

#[unsafe(no_mangle)]
extern "C-unwind" fn PyFloat_GetInfo() -> *mut _object {
    let mut floatinfo: *mut _object = unsafe { core::mem::zeroed() };
    let mut pos: i32 = 0;
    floatinfo = unsafe { PyStructSequence_New(core::ptr::addr_of_mut!(FloatInfoType)) };
    if floatinfo.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    SetDblFlag!(1.7976931348623157e308f64, floatinfo, pos);
    SetIntFlag!(1024i32, floatinfo, pos);
    SetIntFlag!(308i32, floatinfo, pos);
    SetDblFlag!(2.2250738585072014e-308f64, floatinfo, pos);
    SetIntFlag!(1021i32.wrapping_neg(), floatinfo, pos);
    SetIntFlag!(307i32.wrapping_neg(), floatinfo, pos);
    SetIntFlag!(15i32, floatinfo, pos);
    SetIntFlag!(53i32, floatinfo, pos);
    SetDblFlag!(2.220446049250313e-16f64, floatinfo, pos);
    SetIntFlag!(2i32, floatinfo, pos);
    SetIntFlag!(unsafe { __slate_fenv_flt_rounds() }, floatinfo, pos);
    floatinfo
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
struct __SlateRecord_pystate_h_78 {
    __slate_bits_0: __slate_bits::__SlateBits190U0,
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
struct __SlateRecord_pycore_structs_h_32_2 {
    code: u8,
    arg: u8,
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

fn _Py_ADJUST_ERANGE1(x: f64) {
    if (unsafe { *slate_support::errno_location() }) == 0 {
        if x == (f32::INFINITY as f64) || x == -(f32::INFINITY as f64) {
            unsafe {
                *slate_support::errno_location() = slate_support::ERANGE as i32;
            }
        }
    } else {
        if (unsafe { *slate_support::errno_location() }) == (slate_support::ERANGE as i32)
            && x == 0.0f64
        {
            unsafe {
                *slate_support::errno_location() = 0;
            }
        }
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
struct code_arena_st {}

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
union __SlateRecord_pycore_obmalloc_h_255 {
    _padding: *mut u8,
    count: u32,
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
struct __SlateRecord_pycore_runtime_structs_h_49 {
    fd: i32,
    st_dev: u64,
    st_ino: u64,
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
struct __SlateRecord_pycore_tracemalloc_h_70 {
    mem: __SlateRecord_pymem_h_48,
    raw: __SlateRecord_pymem_h_48,
    obj: __SlateRecord_pymem_h_48,
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

fn _Py_IsMainInterpreter(interp: *mut _is) -> i32 {
    (interp == unsafe { _PyInterpreterState_Main() }) as i32
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

fn _PyThreadState_GET() -> *mut _ts {
    unsafe { _Py_tss_tstate }
}

fn _Py_freelists_GET() -> *mut _Py_freelists {
    let tstate: *mut _ts = _PyThreadState_GET();
    unsafe { core::ptr::addr_of_mut!((*(*tstate).interp).object_state.freelists) }
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

// Pushes `op` to the freelist, calls `freefunc` if the freelist is full
fn _PyFreeList_Push(fl: *mut _Py_freelist, obj: *mut (), maxsize: i64) -> i32 {
    if (unsafe { (*fl).size }) < maxsize && (unsafe { (*fl).size }) >= 0 {
        FT_ATOMIC_STORE_PTR_RELAXED!(*(obj as *mut *mut ()), unsafe { (*fl).freelist });
        unsafe {
            (*fl).freelist = obj;
        }
        let __v1: i64 = unsafe { (*fl).size };
        unsafe {
            (*fl).size = __v1.wrapping_add(1);
        }
        OBJECT_STAT_INC!();
        return 1;
    }
    0
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord_sigaction_h_31 {
    sa_handler: Option<unsafe extern "C-unwind" fn(i32)>,
    sa_sigaction:
        Option<unsafe extern "C-unwind" fn(i32, *mut __SlateRecord_siginfo_t_h_124, *mut ())>,
}

fn _PyFreeList_Free(
    fl: *mut _Py_freelist,
    obj: *mut (),
    maxsize: i64,
    dofree: Option<unsafe extern "C-unwind" fn(*mut ())>,
) {
    if _PyFreeList_Push(fl, obj, maxsize) == 0 {
        unsafe { dofree.unwrap()(obj) };
    }
}

fn _PyFreeList_PopNoStats(fl: *mut _Py_freelist) -> *mut () {
    let obj: *mut () = unsafe { (*fl).freelist };
    if !obj.is_null() {
        unsafe {
            (*fl).freelist = *(obj as *mut *mut ());
        }
        let __v1: i64 = unsafe { (*fl).size };
        unsafe {
            (*fl).size = __v1.wrapping_sub(1);
        }
    }
    obj
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

fn _PyFreeList_Pop(fl: *mut _Py_freelist) -> *mut _object {
    let op: *mut _object = _PyFreeList_PopNoStats(fl) as *mut _object;
    if !op.is_null() {
        OBJECT_STAT_INC!();
        unsafe { _Py_NewReference(op) };
    }
    op
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
struct _obmalloc_state {
    pools: _obmalloc_pools,
    mgmt: _obmalloc_mgmt,
    usage: _obmalloc_usage,
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
    pub struct __SlateBits139U0 {
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
    pub struct __SlateBits190U0 {
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

use slate_support::memcmp;
use slate_support::memcpy;
use slate_support::strcmp;
use slate_support::strlen;

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
fn Py_DECREF_MORTAL_SPECIALIZED(
    op: *mut _object,
    destruct: Option<unsafe extern "C-unwind" fn(*mut _object)>,
) {
    _Py_DECREF_STAT_INC!();
    let __v1: u32 = unsafe { (*op).__slate_anon_0.__slate_anon_1.ob_refcnt };
    let __v2: u32 = __v1.wrapping_sub(1);
    unsafe {
        (*op).__slate_anon_0.__slate_anon_1.ob_refcnt = __v2;
    }
    if __v2 == 0 {
        _PyReftracerTrack!(op, 1i32);
        unsafe { destruct.unwrap()(op) };
    }
}

fn _PyObject_Init(op: *mut _object, typeobj: *mut _typeobject) {
    Py_SET_TYPE!(op, typeobj);
    Py_INCREF(_PyObject_CAST__2!(typeobj));
    unsafe { _Py_NewReference(op) };
}

// Fast inlined version of PyType_IS_GC()
static PyStackRef_NULL: _PyStackRef = _PyStackRef { bits: 1u64 };

fn PyStackRef_RefcountOnObject(r#ref: _PyStackRef) -> i32 {
    ((unsafe { r#ref.bits }) & 1 == 0) as i32
}

fn PyStackRef_FromPyObjectSteal(obj: *mut _object) -> _PyStackRef {
    let mut __slate_compound_0: _PyStackRef = unsafe { core::mem::zeroed() };
    let tag: u32 = (((unsafe { (*obj).__slate_anon_0.__slate_anon_1.ob_flags }) as i32) & 1) as u32;
    let r#ref: _PyStackRef = unsafe {
        *{
            __slate_compound_0 = _PyStackRef {
                bits: (obj as u64) | (tag as u64),
            };
            core::ptr::addr_of_mut!(__slate_compound_0)
        }
    };
    PyStackRef_CheckValid!();
    return r#ref;
}

fn PyStackRef_CLOSE_SPECIALIZED(
    r#ref: _PyStackRef,
    destruct: Option<unsafe extern "C-unwind" fn(*mut _object)>,
) {
    if PyStackRef_RefcountOnObject(r#ref) != 0 {
        Py_DECREF_MORTAL_SPECIALIZED!(BITS_TO_PTR!(r#ref), destruct);
    }
}

fn _PyStructSequence_InitBuiltin(
    interp: *mut _is,
    r#type: *mut _typeobject,
    desc: *mut PyStructSequence_Desc,
) -> i32 {
    unsafe { _PyStructSequence_InitBuiltinWithFlags(interp, r#type, desc, 0u64) }
}

static float_is_integer__doc__: __SlateAlign16<[i8; 65]> = __SlateAlign16([
    105i8, 115i8, 95i8, 105i8, 110i8, 116i8, 101i8, 103i8, 101i8, 114i8, 40i8, 36i8, 115i8, 101i8,
    108i8, 102i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8,
    114i8, 110i8, 32i8, 84i8, 114i8, 117i8, 101i8, 32i8, 105i8, 102i8, 32i8, 116i8, 104i8, 101i8,
    32i8, 102i8, 108i8, 111i8, 97i8, 116i8, 32i8, 105i8, 115i8, 32i8, 97i8, 110i8, 32i8, 105i8,
    110i8, 116i8, 101i8, 103i8, 101i8, 114i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.floatobject.float_is_integer")]
extern "C-unwind" fn float_is_integer(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    float_is_integer_impl(self_)
}

static float___trunc____doc__: __SlateAlign16<[i8; 74]> = __SlateAlign16([
    95i8, 95i8, 116i8, 114i8, 117i8, 110i8, 99i8, 95i8, 95i8, 40i8, 36i8, 115i8, 101i8, 108i8,
    102i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8,
    110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 73i8, 110i8, 116i8, 101i8, 103i8, 114i8, 97i8, 108i8,
    32i8, 99i8, 108i8, 111i8, 115i8, 101i8, 115i8, 116i8, 32i8, 116i8, 111i8, 32i8, 120i8, 32i8,
    98i8, 101i8, 116i8, 119i8, 101i8, 101i8, 110i8, 32i8, 48i8, 32i8, 97i8, 110i8, 100i8, 32i8,
    120i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.floatobject.float___trunc__")]
extern "C-unwind" fn float___trunc__(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    float___trunc___impl(self_)
}

static float___floor____doc__: __SlateAlign16<[i8; 57]> = __SlateAlign16([
    95i8, 95i8, 102i8, 108i8, 111i8, 111i8, 114i8, 95i8, 95i8, 40i8, 36i8, 115i8, 101i8, 108i8,
    102i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8,
    110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 102i8, 108i8, 111i8, 111i8, 114i8, 32i8, 97i8, 115i8,
    32i8, 97i8, 110i8, 32i8, 73i8, 110i8, 116i8, 101i8, 103i8, 114i8, 97i8, 108i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.floatobject.float___floor__")]
extern "C-unwind" fn float___floor__(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    float___floor___impl(self_)
}

static float___ceil____doc__: __SlateAlign16<[i8; 58]> = __SlateAlign16([
    95i8, 95i8, 99i8, 101i8, 105i8, 108i8, 95i8, 95i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8,
    44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8,
    32i8, 116i8, 104i8, 101i8, 32i8, 99i8, 101i8, 105i8, 108i8, 105i8, 110i8, 103i8, 32i8, 97i8,
    115i8, 32i8, 97i8, 110i8, 32i8, 73i8, 110i8, 116i8, 101i8, 103i8, 114i8, 97i8, 108i8, 46i8,
    0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.floatobject.float___ceil__")]
extern "C-unwind" fn float___ceil__(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    float___ceil___impl(self_)
}

static float___round____doc__: __SlateAlign16<[i8; 166]> = __SlateAlign16([
    95i8, 95i8, 114i8, 111i8, 117i8, 110i8, 100i8, 95i8, 95i8, 40i8, 36i8, 115i8, 101i8, 108i8,
    102i8, 44i8, 32i8, 110i8, 100i8, 105i8, 103i8, 105i8, 116i8, 115i8, 61i8, 78i8, 111i8, 110i8,
    101i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8,
    110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 73i8, 110i8, 116i8, 101i8, 103i8, 114i8, 97i8, 108i8,
    32i8, 99i8, 108i8, 111i8, 115i8, 101i8, 115i8, 116i8, 32i8, 116i8, 111i8, 32i8, 120i8, 44i8,
    32i8, 114i8, 111i8, 117i8, 110i8, 100i8, 105i8, 110i8, 103i8, 32i8, 104i8, 97i8, 108i8, 102i8,
    32i8, 116i8, 111i8, 119i8, 97i8, 114i8, 100i8, 32i8, 101i8, 118i8, 101i8, 110i8, 46i8, 10i8,
    10i8, 87i8, 104i8, 101i8, 110i8, 32i8, 97i8, 110i8, 32i8, 97i8, 114i8, 103i8, 117i8, 109i8,
    101i8, 110i8, 116i8, 32i8, 105i8, 115i8, 32i8, 112i8, 97i8, 115i8, 115i8, 101i8, 100i8, 44i8,
    32i8, 119i8, 111i8, 114i8, 107i8, 32i8, 108i8, 105i8, 107i8, 101i8, 32i8, 98i8, 117i8, 105i8,
    108i8, 116i8, 45i8, 105i8, 110i8, 32i8, 114i8, 111i8, 117i8, 110i8, 100i8, 40i8, 120i8, 44i8,
    32i8, 110i8, 100i8, 105i8, 103i8, 105i8, 116i8, 115i8, 41i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.floatobject.float___round__")]
extern "C-unwind" fn float___round__(
    self_: *mut _object,
    args: *const *mut _object,
    nargs: i64,
) -> *mut _object {
    let mut o_ndigits: *mut _object = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        o_ndigits = core::ptr::addr_of_mut!(_Py_NoneStruct);
        if _PyArg_CheckPositional!(c"__round__".as_ptr() as *mut i8, nargs, 0i32, 1i32) {
            if nargs >= 1 {
                o_ndigits = *args.wrapping_offset(0 as isize);
            }
            return_value = float___round___impl(self_, o_ndigits);
        }
        return return_value;
    }
}

static float_conjugate__doc__: __SlateAlign16<[i8; 73]> = __SlateAlign16([
    99i8, 111i8, 110i8, 106i8, 117i8, 103i8, 97i8, 116i8, 101i8, 40i8, 36i8, 115i8, 101i8, 108i8,
    102i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8,
    110i8, 32i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8, 116i8, 104i8, 101i8, 32i8, 99i8, 111i8,
    109i8, 112i8, 108i8, 101i8, 120i8, 32i8, 99i8, 111i8, 110i8, 106i8, 117i8, 103i8, 97i8, 116i8,
    101i8, 32i8, 111i8, 102i8, 32i8, 97i8, 110i8, 121i8, 32i8, 102i8, 108i8, 111i8, 97i8, 116i8,
    46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.floatobject.float_conjugate")]
extern "C-unwind" fn float_conjugate(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    float_conjugate_impl(self_)
}

static float_hex__doc__: __SlateAlign16<[i8; 165]> = __SlateAlign16([
    104i8, 101i8, 120i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8, 47i8, 41i8, 10i8,
    45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 97i8, 32i8, 104i8,
    101i8, 120i8, 97i8, 100i8, 101i8, 99i8, 105i8, 109i8, 97i8, 108i8, 32i8, 114i8, 101i8, 112i8,
    114i8, 101i8, 115i8, 101i8, 110i8, 116i8, 97i8, 116i8, 105i8, 111i8, 110i8, 32i8, 111i8, 102i8,
    32i8, 97i8, 32i8, 102i8, 108i8, 111i8, 97i8, 116i8, 105i8, 110i8, 103i8, 45i8, 112i8, 111i8,
    105i8, 110i8, 116i8, 32i8, 110i8, 117i8, 109i8, 98i8, 101i8, 114i8, 46i8, 10i8, 10i8, 62i8,
    62i8, 62i8, 32i8, 40i8, 45i8, 48i8, 46i8, 49i8, 41i8, 46i8, 104i8, 101i8, 120i8, 40i8, 41i8,
    10i8, 39i8, 45i8, 48i8, 120i8, 49i8, 46i8, 57i8, 57i8, 57i8, 57i8, 57i8, 57i8, 57i8, 57i8,
    57i8, 57i8, 57i8, 57i8, 97i8, 112i8, 45i8, 52i8, 39i8, 10i8, 62i8, 62i8, 62i8, 32i8, 51i8,
    46i8, 49i8, 52i8, 49i8, 53i8, 57i8, 46i8, 104i8, 101i8, 120i8, 40i8, 41i8, 10i8, 39i8, 48i8,
    120i8, 49i8, 46i8, 57i8, 50i8, 49i8, 102i8, 57i8, 102i8, 48i8, 49i8, 98i8, 56i8, 54i8, 54i8,
    101i8, 112i8, 43i8, 49i8, 39i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.floatobject.float_hex")]
extern "C-unwind" fn float_hex(self_: *mut _object, _unused_ignored: *mut _object) -> *mut _object {
    float_hex_impl(self_)
}

static float_fromhex__doc__: __SlateAlign16<[i8; 174]> = __SlateAlign16([
    102i8, 114i8, 111i8, 109i8, 104i8, 101i8, 120i8, 40i8, 36i8, 116i8, 121i8, 112i8, 101i8, 44i8,
    32i8, 115i8, 116i8, 114i8, 105i8, 110i8, 103i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8,
    10i8, 67i8, 114i8, 101i8, 97i8, 116i8, 101i8, 32i8, 97i8, 32i8, 102i8, 108i8, 111i8, 97i8,
    116i8, 105i8, 110i8, 103i8, 45i8, 112i8, 111i8, 105i8, 110i8, 116i8, 32i8, 110i8, 117i8, 109i8,
    98i8, 101i8, 114i8, 32i8, 102i8, 114i8, 111i8, 109i8, 32i8, 97i8, 32i8, 104i8, 101i8, 120i8,
    97i8, 100i8, 101i8, 99i8, 105i8, 109i8, 97i8, 108i8, 32i8, 115i8, 116i8, 114i8, 105i8, 110i8,
    103i8, 46i8, 10i8, 10i8, 62i8, 62i8, 62i8, 32i8, 102i8, 108i8, 111i8, 97i8, 116i8, 46i8, 102i8,
    114i8, 111i8, 109i8, 104i8, 101i8, 120i8, 40i8, 39i8, 48i8, 120i8, 49i8, 46i8, 102i8, 102i8,
    102i8, 102i8, 112i8, 49i8, 48i8, 39i8, 41i8, 10i8, 50i8, 48i8, 52i8, 55i8, 46i8, 57i8, 56i8,
    52i8, 51i8, 55i8, 53i8, 10i8, 62i8, 62i8, 62i8, 32i8, 102i8, 108i8, 111i8, 97i8, 116i8, 46i8,
    102i8, 114i8, 111i8, 109i8, 104i8, 101i8, 120i8, 40i8, 39i8, 45i8, 48i8, 120i8, 49i8, 112i8,
    45i8, 49i8, 48i8, 55i8, 52i8, 39i8, 41i8, 10i8, 45i8, 53i8, 101i8, 45i8, 51i8, 50i8, 52i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.floatobject.float_fromhex")]
extern "C-unwind" fn float_fromhex(r#type: *mut _object, string: *mut _object) -> *mut _object {
    let mut return_value: *mut _object = core::ptr::null_mut::<_object>();
    return_value = float_fromhex_impl(r#type as *mut _typeobject, string);
    return_value
}

static float_as_integer_ratio__doc__: __SlateAlign16<[i8; 346]> = __SlateAlign16([
    97i8, 115i8, 95i8, 105i8, 110i8, 116i8, 101i8, 103i8, 101i8, 114i8, 95i8, 114i8, 97i8, 116i8,
    105i8, 111i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8,
    10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 97i8, 32i8, 112i8, 97i8, 105i8,
    114i8, 32i8, 111i8, 102i8, 32i8, 105i8, 110i8, 116i8, 101i8, 103i8, 101i8, 114i8, 115i8, 44i8,
    32i8, 119i8, 104i8, 111i8, 115i8, 101i8, 32i8, 114i8, 97i8, 116i8, 105i8, 111i8, 32i8, 105i8,
    115i8, 32i8, 101i8, 120i8, 97i8, 99i8, 116i8, 108i8, 121i8, 32i8, 101i8, 113i8, 117i8, 97i8,
    108i8, 32i8, 116i8, 111i8, 32i8, 116i8, 104i8, 101i8, 32i8, 111i8, 114i8, 105i8, 103i8, 105i8,
    110i8, 97i8, 108i8, 32i8, 102i8, 108i8, 111i8, 97i8, 116i8, 46i8, 10i8, 10i8, 84i8, 104i8,
    101i8, 32i8, 114i8, 97i8, 116i8, 105i8, 111i8, 32i8, 105i8, 115i8, 32i8, 105i8, 110i8, 32i8,
    108i8, 111i8, 119i8, 101i8, 115i8, 116i8, 32i8, 116i8, 101i8, 114i8, 109i8, 115i8, 32i8, 97i8,
    110i8, 100i8, 32i8, 104i8, 97i8, 115i8, 32i8, 97i8, 32i8, 112i8, 111i8, 115i8, 105i8, 116i8,
    105i8, 118i8, 101i8, 32i8, 100i8, 101i8, 110i8, 111i8, 109i8, 105i8, 110i8, 97i8, 116i8, 111i8,
    114i8, 46i8, 32i8, 32i8, 82i8, 97i8, 105i8, 115i8, 101i8, 10i8, 79i8, 118i8, 101i8, 114i8,
    102i8, 108i8, 111i8, 119i8, 69i8, 114i8, 114i8, 111i8, 114i8, 32i8, 111i8, 110i8, 32i8, 105i8,
    110i8, 102i8, 105i8, 110i8, 105i8, 116i8, 105i8, 101i8, 115i8, 32i8, 97i8, 110i8, 100i8, 32i8,
    97i8, 32i8, 86i8, 97i8, 108i8, 117i8, 101i8, 69i8, 114i8, 114i8, 111i8, 114i8, 32i8, 111i8,
    110i8, 32i8, 78i8, 97i8, 78i8, 115i8, 46i8, 10i8, 10i8, 62i8, 62i8, 62i8, 32i8, 40i8, 49i8,
    48i8, 46i8, 48i8, 41i8, 46i8, 97i8, 115i8, 95i8, 105i8, 110i8, 116i8, 101i8, 103i8, 101i8,
    114i8, 95i8, 114i8, 97i8, 116i8, 105i8, 111i8, 40i8, 41i8, 10i8, 40i8, 49i8, 48i8, 44i8, 32i8,
    49i8, 41i8, 10i8, 62i8, 62i8, 62i8, 32i8, 40i8, 48i8, 46i8, 48i8, 41i8, 46i8, 97i8, 115i8,
    95i8, 105i8, 110i8, 116i8, 101i8, 103i8, 101i8, 114i8, 95i8, 114i8, 97i8, 116i8, 105i8, 111i8,
    40i8, 41i8, 10i8, 40i8, 48i8, 44i8, 32i8, 49i8, 41i8, 10i8, 62i8, 62i8, 62i8, 32i8, 40i8, 45i8,
    46i8, 50i8, 53i8, 41i8, 46i8, 97i8, 115i8, 95i8, 105i8, 110i8, 116i8, 101i8, 103i8, 101i8,
    114i8, 95i8, 114i8, 97i8, 116i8, 105i8, 111i8, 40i8, 41i8, 10i8, 40i8, 45i8, 49i8, 44i8, 32i8,
    52i8, 41i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.floatobject.float_as_integer_ratio")]
extern "C-unwind" fn float_as_integer_ratio(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    float_as_integer_ratio_impl(self_)
}

static float_new__doc__: __SlateAlign16<[i8; 86]> = __SlateAlign16([
    102i8, 108i8, 111i8, 97i8, 116i8, 40i8, 120i8, 61i8, 48i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8,
    45i8, 10i8, 10i8, 67i8, 111i8, 110i8, 118i8, 101i8, 114i8, 116i8, 32i8, 97i8, 32i8, 115i8,
    116i8, 114i8, 105i8, 110i8, 103i8, 32i8, 111i8, 114i8, 32i8, 110i8, 117i8, 109i8, 98i8, 101i8,
    114i8, 32i8, 116i8, 111i8, 32i8, 97i8, 32i8, 102i8, 108i8, 111i8, 97i8, 116i8, 105i8, 110i8,
    103i8, 45i8, 112i8, 111i8, 105i8, 110i8, 116i8, 32i8, 110i8, 117i8, 109i8, 98i8, 101i8, 114i8,
    44i8, 32i8, 105i8, 102i8, 32i8, 112i8, 111i8, 115i8, 115i8, 105i8, 98i8, 108i8, 101i8, 46i8,
    0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.floatobject.float_new")]
extern "C-unwind" fn float_new(
    r#type: *mut _typeobject,
    args: *mut _object,
    kwargs: *mut _object,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut x: *mut _object = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        let base_tp: *mut _typeobject = core::ptr::addr_of_mut!(PyFloat_Type);
        x = core::ptr::null_mut::<_object>();
        if r#type == base_tp || (*r#type).tp_init == (*base_tp).tp_init {
            __v0 = !_PyArg_NoKeywords!(c"float".as_ptr() as *mut i8, kwargs);
        } else {
            __v0 = false;
        }
        if !__v0 {
            if _PyArg_CheckPositional__2!(
                c"float".as_ptr() as *mut i8,
                PyTuple_GET_SIZE(args),
                0i32,
                1i32
            ) {
                if PyTuple_GET_SIZE!(args) >= 1 {
                    x = PyTuple_GET_ITEM!(args, 0i32);
                }
                return_value = float_new_impl(r#type, x);
            }
        }
        return return_value;
    }
}

static float_from_number__doc__: __SlateAlign16<[i8; 82]> = __SlateAlign16([
    102i8, 114i8, 111i8, 109i8, 95i8, 110i8, 117i8, 109i8, 98i8, 101i8, 114i8, 40i8, 36i8, 116i8,
    121i8, 112i8, 101i8, 44i8, 32i8, 110i8, 117i8, 109i8, 98i8, 101i8, 114i8, 44i8, 32i8, 47i8,
    41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 67i8, 111i8, 110i8, 118i8, 101i8, 114i8, 116i8, 32i8,
    114i8, 101i8, 97i8, 108i8, 32i8, 110i8, 117i8, 109i8, 98i8, 101i8, 114i8, 32i8, 116i8, 111i8,
    32i8, 97i8, 32i8, 102i8, 108i8, 111i8, 97i8, 116i8, 105i8, 110i8, 103i8, 45i8, 112i8, 111i8,
    105i8, 110i8, 116i8, 32i8, 110i8, 117i8, 109i8, 98i8, 101i8, 114i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.floatobject.float_from_number")]
extern "C-unwind" fn float_from_number(r#type: *mut _object, number: *mut _object) -> *mut _object {
    let mut return_value: *mut _object = core::ptr::null_mut::<_object>();
    return_value = float_from_number_impl(r#type as *mut _typeobject, number);
    return_value
}

static float___getnewargs____doc__: __SlateAlign16<[i8; 30]> = __SlateAlign16([
    95i8, 95i8, 103i8, 101i8, 116i8, 110i8, 101i8, 119i8, 97i8, 114i8, 103i8, 115i8, 95i8, 95i8,
    40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8,
    0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.floatobject.float___getnewargs__")]
extern "C-unwind" fn float___getnewargs__(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    float___getnewargs___impl(self_)
}

static float___getformat____doc__: __SlateAlign16<[i8; 360]> = __SlateAlign16([
    95i8, 95i8, 103i8, 101i8, 116i8, 102i8, 111i8, 114i8, 109i8, 97i8, 116i8, 95i8, 95i8, 40i8,
    36i8, 116i8, 121i8, 112i8, 101i8, 44i8, 32i8, 116i8, 121i8, 112i8, 101i8, 115i8, 116i8, 114i8,
    44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 89i8, 111i8, 117i8, 32i8, 112i8, 114i8,
    111i8, 98i8, 97i8, 98i8, 108i8, 121i8, 32i8, 100i8, 111i8, 110i8, 39i8, 116i8, 32i8, 119i8,
    97i8, 110i8, 116i8, 32i8, 116i8, 111i8, 32i8, 117i8, 115i8, 101i8, 32i8, 116i8, 104i8, 105i8,
    115i8, 32i8, 102i8, 117i8, 110i8, 99i8, 116i8, 105i8, 111i8, 110i8, 46i8, 10i8, 10i8, 32i8,
    32i8, 116i8, 121i8, 112i8, 101i8, 115i8, 116i8, 114i8, 10i8, 32i8, 32i8, 32i8, 32i8, 77i8,
    117i8, 115i8, 116i8, 32i8, 98i8, 101i8, 32i8, 39i8, 100i8, 111i8, 117i8, 98i8, 108i8, 101i8,
    39i8, 32i8, 111i8, 114i8, 32i8, 39i8, 102i8, 108i8, 111i8, 97i8, 116i8, 39i8, 46i8, 10i8, 10i8,
    73i8, 116i8, 32i8, 101i8, 120i8, 105i8, 115i8, 116i8, 115i8, 32i8, 109i8, 97i8, 105i8, 110i8,
    108i8, 121i8, 32i8, 116i8, 111i8, 32i8, 98i8, 101i8, 32i8, 117i8, 115i8, 101i8, 100i8, 32i8,
    105i8, 110i8, 32i8, 80i8, 121i8, 116i8, 104i8, 111i8, 110i8, 39i8, 115i8, 32i8, 116i8, 101i8,
    115i8, 116i8, 32i8, 115i8, 117i8, 105i8, 116i8, 101i8, 46i8, 10i8, 10i8, 84i8, 104i8, 105i8,
    115i8, 32i8, 102i8, 117i8, 110i8, 99i8, 116i8, 105i8, 111i8, 110i8, 32i8, 114i8, 101i8, 116i8,
    117i8, 114i8, 110i8, 115i8, 32i8, 119i8, 104i8, 105i8, 99i8, 104i8, 101i8, 118i8, 101i8, 114i8,
    32i8, 111i8, 102i8, 32i8, 39i8, 117i8, 110i8, 107i8, 110i8, 111i8, 119i8, 110i8, 39i8, 44i8,
    32i8, 39i8, 73i8, 69i8, 69i8, 69i8, 44i8, 32i8, 98i8, 105i8, 103i8, 45i8, 101i8, 110i8, 100i8,
    105i8, 97i8, 110i8, 39i8, 32i8, 111i8, 114i8, 10i8, 39i8, 73i8, 69i8, 69i8, 69i8, 44i8, 32i8,
    108i8, 105i8, 116i8, 116i8, 108i8, 101i8, 45i8, 101i8, 110i8, 100i8, 105i8, 97i8, 110i8, 39i8,
    32i8, 98i8, 101i8, 115i8, 116i8, 32i8, 100i8, 101i8, 115i8, 99i8, 114i8, 105i8, 98i8, 101i8,
    115i8, 32i8, 116i8, 104i8, 101i8, 32i8, 102i8, 111i8, 114i8, 109i8, 97i8, 116i8, 32i8, 111i8,
    102i8, 32i8, 102i8, 108i8, 111i8, 97i8, 116i8, 105i8, 110i8, 103i8, 45i8, 112i8, 111i8, 105i8,
    110i8, 116i8, 10i8, 110i8, 117i8, 109i8, 98i8, 101i8, 114i8, 115i8, 32i8, 117i8, 115i8, 101i8,
    100i8, 32i8, 98i8, 121i8, 32i8, 116i8, 104i8, 101i8, 32i8, 67i8, 32i8, 116i8, 121i8, 112i8,
    101i8, 32i8, 110i8, 97i8, 109i8, 101i8, 100i8, 32i8, 98i8, 121i8, 32i8, 116i8, 121i8, 112i8,
    101i8, 115i8, 116i8, 114i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.floatobject.float___getformat__")]
extern "C-unwind" fn float___getformat__(r#type: *mut _object, arg: *mut _object) -> *mut _object {
    let mut typestr_length: i64 = unsafe { core::mem::zeroed() };
    let mut typestr: *const i8 = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        if PyUnicode_Check!(arg) == 0 {
            _PyArg_BadArgument(
                c"__getformat__".as_ptr(),
                c"argument".as_ptr(),
                c"str".as_ptr(),
                arg,
            );
        } else {
            typestr = PyUnicode_AsUTF8AndSize(arg, core::ptr::addr_of_mut!(typestr_length));
            if !typestr.is_null() {
                if (strlen(typestr as *const core::ffi::c_char) as u64) != (typestr_length as u64) {
                    PyErr_SetString(PyExc_ValueError, c"embedded null character".as_ptr());
                } else {
                    return_value = float___getformat___impl(r#type as *mut _typeobject, typestr);
                }
            }
        }
        return return_value;
    }
}

static float___format____doc__: __SlateAlign16<[i8; 82]> = __SlateAlign16([
    95i8, 95i8, 102i8, 111i8, 114i8, 109i8, 97i8, 116i8, 95i8, 95i8, 40i8, 36i8, 115i8, 101i8,
    108i8, 102i8, 44i8, 32i8, 102i8, 111i8, 114i8, 109i8, 97i8, 116i8, 95i8, 115i8, 112i8, 101i8,
    99i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 70i8, 111i8, 114i8, 109i8, 97i8,
    116i8, 115i8, 32i8, 116i8, 104i8, 101i8, 32i8, 102i8, 108i8, 111i8, 97i8, 116i8, 32i8, 97i8,
    99i8, 99i8, 111i8, 114i8, 100i8, 105i8, 110i8, 103i8, 32i8, 116i8, 111i8, 32i8, 102i8, 111i8,
    114i8, 109i8, 97i8, 116i8, 95i8, 115i8, 112i8, 101i8, 99i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.floatobject.float___format__")]
extern "C-unwind" fn float___format__(self_: *mut _object, arg: *mut _object) -> *mut _object {
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        if PyUnicode_Check!(arg) == 0 {
            _PyArg_BadArgument(
                c"__format__".as_ptr(),
                c"argument".as_ptr(),
                c"str".as_ptr(),
                arg,
            );
        } else {
            let format_spec: *mut _object = arg;
            return_value = float___format___impl(self_, format_spec);
        }
        return return_value;
    }
}

// XXX There should be overflow checks here, but it's hard to check
// for any kind of float exception without losing portability.
// _PyNumber_Index()
// _Py_dg_dtoa()
// _PyFloat_FormatAdvancedWriter()
// _Py_FREELIST_FREE(), _Py_FREELIST_POP()
// _PyStatus_OK()
// _PyLong_GetOne()
// _PyArg_NoKwnames()
// _PyObject_Init(), _PyDebugAllocatorStats()
// _PY_SHORT_FLOAT_REPR
// _PyInterpreterState_GET()
// PyStackRef_AsPyObjectBorrow()
// _PyStructSequence_FiniBuiltin()
// DBL_MAX
// strtol()
// [clinic input]
// class float "PyObject *" "&PyFloat_Type"
// [clinic start generated code]
//
// [clinic end generated code: output=da39a3ee5e6b4b0d input=dd0003f68f144284]
#[unsafe(no_mangle)]
extern "C-unwind" fn PyFloat_GetMax() -> f64 {
    1.7976931348623157e308f64
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyFloat_GetMin() -> f64 {
    2.2250738585072014e-308f64
}

static mut FloatInfoType: _typeobject = unsafe { core::mem::zeroed() };

static floatinfo__doc__: __SlateAlign16<[i8; 220]> = __SlateAlign16([
    115i8, 121i8, 115i8, 46i8, 102i8, 108i8, 111i8, 97i8, 116i8, 95i8, 105i8, 110i8, 102i8, 111i8,
    10i8, 10i8, 65i8, 32i8, 110i8, 97i8, 109i8, 101i8, 100i8, 32i8, 116i8, 117i8, 112i8, 108i8,
    101i8, 32i8, 104i8, 111i8, 108i8, 100i8, 105i8, 110i8, 103i8, 32i8, 105i8, 110i8, 102i8, 111i8,
    114i8, 109i8, 97i8, 116i8, 105i8, 111i8, 110i8, 32i8, 97i8, 98i8, 111i8, 117i8, 116i8, 32i8,
    116i8, 104i8, 101i8, 32i8, 102i8, 108i8, 111i8, 97i8, 116i8, 32i8, 116i8, 121i8, 112i8, 101i8,
    46i8, 32i8, 73i8, 116i8, 32i8, 99i8, 111i8, 110i8, 116i8, 97i8, 105i8, 110i8, 115i8, 32i8,
    108i8, 111i8, 119i8, 32i8, 108i8, 101i8, 118i8, 101i8, 108i8, 10i8, 105i8, 110i8, 102i8, 111i8,
    114i8, 109i8, 97i8, 116i8, 105i8, 111i8, 110i8, 32i8, 97i8, 98i8, 111i8, 117i8, 116i8, 32i8,
    116i8, 104i8, 101i8, 32i8, 112i8, 114i8, 101i8, 99i8, 105i8, 115i8, 105i8, 111i8, 110i8, 32i8,
    97i8, 110i8, 100i8, 32i8, 105i8, 110i8, 116i8, 101i8, 114i8, 110i8, 97i8, 108i8, 32i8, 114i8,
    101i8, 112i8, 114i8, 101i8, 115i8, 101i8, 110i8, 116i8, 97i8, 116i8, 105i8, 111i8, 110i8, 46i8,
    32i8, 80i8, 108i8, 101i8, 97i8, 115i8, 101i8, 32i8, 115i8, 116i8, 117i8, 100i8, 121i8, 10i8,
    121i8, 111i8, 117i8, 114i8, 32i8, 115i8, 121i8, 115i8, 116i8, 101i8, 109i8, 39i8, 115i8, 32i8,
    58i8, 102i8, 105i8, 108i8, 101i8, 58i8, 96i8, 102i8, 108i8, 111i8, 97i8, 116i8, 46i8, 104i8,
    96i8, 32i8, 102i8, 111i8, 114i8, 32i8, 109i8, 111i8, 114i8, 101i8, 32i8, 105i8, 110i8, 102i8,
    111i8, 114i8, 109i8, 97i8, 116i8, 105i8, 111i8, 110i8, 46i8, 0i8,
]);

static mut floatinfo_fields: __SlateAlign16<[PyStructSequence_Field; 12]> = __SlateAlign16([PyStructSequence_Field { name: (b"max\0".as_ptr() as *mut i8) as *const i8, doc: (b"DBL_MAX -- maximum representable finite float\0".as_ptr() as *mut i8) as *const i8 }, PyStructSequence_Field { name: (b"max_exp\0".as_ptr() as *mut i8) as *const i8, doc: (b"DBL_MAX_EXP -- maximum int e such that radix**(e-1) is representable\0".as_ptr() as *mut i8) as *const i8 }, PyStructSequence_Field { name: (b"max_10_exp\0".as_ptr() as *mut i8) as *const i8, doc: (b"DBL_MAX_10_EXP -- maximum int e such that 10**e is representable\0".as_ptr() as *mut i8) as *const i8 }, PyStructSequence_Field { name: (b"min\0".as_ptr() as *mut i8) as *const i8, doc: (b"DBL_MIN -- Minimum positive normalized float\0".as_ptr() as *mut i8) as *const i8 }, PyStructSequence_Field { name: (b"min_exp\0".as_ptr() as *mut i8) as *const i8, doc: (b"DBL_MIN_EXP -- minimum int e such that radix**(e-1) is a normalized float\0".as_ptr() as *mut i8) as *const i8 }, PyStructSequence_Field { name: (b"min_10_exp\0".as_ptr() as *mut i8) as *const i8, doc: (b"DBL_MIN_10_EXP -- minimum int e such that 10**e is a normalized float\0".as_ptr() as *mut i8) as *const i8 }, PyStructSequence_Field { name: (b"dig\0".as_ptr() as *mut i8) as *const i8, doc: (b"DBL_DIG -- maximum number of decimal digits that can be faithfully represented in a float\0".as_ptr() as *mut i8) as *const i8 }, PyStructSequence_Field { name: (b"mant_dig\0".as_ptr() as *mut i8) as *const i8, doc: (b"DBL_MANT_DIG -- mantissa digits\0".as_ptr() as *mut i8) as *const i8 }, PyStructSequence_Field { name: (b"epsilon\0".as_ptr() as *mut i8) as *const i8, doc: (b"DBL_EPSILON -- Difference between 1 and the next representable float\0".as_ptr() as *mut i8) as *const i8 }, PyStructSequence_Field { name: (b"radix\0".as_ptr() as *mut i8) as *const i8, doc: (b"FLT_RADIX -- radix of exponent\0".as_ptr() as *mut i8) as *const i8 }, PyStructSequence_Field { name: (b"rounds\0".as_ptr() as *mut i8) as *const i8, doc: (b"FLT_ROUNDS -- rounding mode used for arithmetic operations\0".as_ptr() as *mut i8) as *const i8 }, {
let mut __t0: PyStructSequence_Field = unsafe { core::mem::zeroed() };
__t0.name = core::ptr::null::<i8>();
__t0
}]);

/// name
/// doc
/// fields
static mut floatinfo_desc: PyStructSequence_Desc = {
    let mut __t0: PyStructSequence_Desc = unsafe { core::mem::zeroed() };
    __t0.name = (b"sys.float_info\0".as_ptr() as *mut i8) as *const i8;
    __t0.doc = unsafe { core::ptr::addr_of!(floatinfo__doc__.0) as *const i8 };
    __t0.fields =
        unsafe { core::ptr::addr_of_mut!(floatinfo_fields.0) as *mut PyStructSequence_Field };
    __t0.n_in_sequence = 11i32;
    __t0
};

#[unsafe(no_mangle)]
extern "C-unwind" fn PyFloat_FromDouble(fval: f64) -> *mut _object {
    let mut op: *mut __SlateRecord_floatobject_h_8 =
        _Py_FREELIST_POP!(__SlateRecord_floatobject_h_8, floats);
    if op.is_null() {
        op = (unsafe {
            PyObject_Malloc(core::mem::size_of::<__SlateRecord_floatobject_h_8>() as u64)
        }) as *mut __SlateRecord_floatobject_h_8;
        if op.is_null() {
            return unsafe { PyErr_NoMemory() };
        }
        _PyObject_Init(op as *mut _object, unsafe {
            core::ptr::addr_of_mut!(PyFloat_Type)
        });
    }
    unsafe {
        (*op).ob_fval = fval;
    }
    op as *mut _object
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyFloat_FromDouble_ConsumeInputs(
    left: _PyStackRef,
    right: _PyStackRef,
    value: f64,
) -> _PyStackRef {
    PyStackRef_CLOSE_SPECIALIZED(left, Some(_PyFloat_ExactDealloc));
    PyStackRef_CLOSE_SPECIALIZED(right, Some(_PyFloat_ExactDealloc));
    let obj: *mut _object = PyFloat_FromDouble(value);
    if obj.is_null() {
        return unsafe { PyStackRef_NULL };
    }
    return PyStackRef_FromPyObjectSteal(obj);
}

#[unsafe(link_section = ".text.slate_distinct.floatobject.float_from_string_inner")]
extern "C-unwind" fn float_from_string_inner(
    mut s: *const i8,
    len: i64,
    obj: *mut (),
) -> *mut _object {
    let mut x: f64 = 0 as f64;
    let mut end: *const i8 = unsafe { core::mem::zeroed() };
    let mut last: *const i8 = s.wrapping_offset(len as isize);
    // strip leading whitespace
    while s < last && Py_ISSPACE!(unsafe { *s }) != 0 {
        let __v0: *const i8 = s;
        s = __v0.wrapping_offset(1 as isize);
    }
    if s == last {
        unsafe {
            PyErr_Format(
                PyExc_ValueError,
                c"could not convert string to float: %R".as_ptr(),
                obj,
            )
        };
        return core::ptr::null_mut::<_object>();
    }
    // strip trailing whitespace
    while s < last.wrapping_offset(-(1 as isize))
        && Py_ISSPACE!(unsafe { *last.wrapping_offset(1i32.wrapping_neg() as isize) }) != 0
    {
        let __v3: *const i8 = last;
        last = __v3.wrapping_offset(-(1 as isize));
    }
    // We don't care about overflow or underflow.  If the platform
    // supports them, infinities and signed zeroes (on underflow) are
    // fine.
    x = unsafe {
        PyOS_string_to_double(
            s,
            core::ptr::addr_of_mut!(end) as *mut *mut i8,
            core::ptr::null_mut::<_object>(),
        )
    };
    if end != last {
        unsafe {
            PyErr_Format(
                PyExc_ValueError,
                c"could not convert string to float: %R".as_ptr(),
                obj,
            )
        };
        return core::ptr::null_mut::<_object>();
    } else {
        let __v6: bool = x == -1.0f64 && !unsafe { PyErr_Occurred() }.is_null();
        if __v6 {
            return core::ptr::null_mut::<_object>();
        } else {
            return PyFloat_FromDouble(x);
        }
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyFloat_FromString(v: *mut _object) -> *mut _object {
    let mut s: *const i8 = unsafe { core::mem::zeroed() };
    let mut s_buffer: *mut _object = core::ptr::null_mut::<_object>();
    let mut len: i64 = 0;
    let mut view: __SlateRecord_pybuffer_h_33 = {
        let mut __t0: __SlateRecord_pybuffer_h_33 = unsafe { core::mem::zeroed() };
        __t0.buf = core::ptr::null_mut::<()>();
        __t0.obj = core::ptr::null_mut::<_object>();
        __t0
    };
    let mut result: *mut _object = core::ptr::null_mut::<_object>();
    if PyUnicode_Check!(v) != 0 {
        s_buffer = unsafe { _PyUnicode_TransformDecimalAndSpaceToASCII(v) };
        if s_buffer.is_null() {
            return core::ptr::null_mut::<_object>();
        }
        // Simply get a pointer to existing ASCII characters.
        s = unsafe { PyUnicode_AsUTF8AndSize(s_buffer, core::ptr::addr_of_mut!(len)) };
    } else {
        if PyBytes_Check!(v) != 0 {
            s = PyBytes_AS_STRING!(v) as *const i8;
            len = PyBytes_GET_SIZE!(v);
        } else {
            if PyByteArray_Check!(v) != 0 {
                s = PyByteArray_AS_STRING__2!(v) as *const i8;
                len = PyByteArray_GET_SIZE!(v);
            } else {
                if (unsafe { PyObject_GetBuffer(v, core::ptr::addr_of_mut!(view), 0i32) }) == 0 {
                    s = view.buf as *const i8;
                    len = view.len;
                    // Copy to NUL-terminated buffer.
                    s_buffer = unsafe { PyBytes_FromStringAndSize(s, len) };
                    if s_buffer.is_null() {
                        unsafe { PyBuffer_Release(core::ptr::addr_of_mut!(view)) };
                        return core::ptr::null_mut::<_object>();
                    }
                    s = PyBytes_AS_STRING!(s_buffer) as *const i8;
                } else {
                    unsafe {
                        PyErr_Format(
                            PyExc_TypeError,
                            c"float() argument must be a string or a real number, not '%.200s'"
                                .as_ptr(),
                            (*Py_TYPE!(v)).tp_name,
                        )
                    };
                    return core::ptr::null_mut::<_object>();
                }
            }
        }
    }
    result = unsafe {
        _Py_string_to_number_with_underscores(
            s,
            len,
            c"float".as_ptr(),
            v,
            v as *mut (),
            Some(float_from_string_inner),
        )
    };
    unsafe { PyBuffer_Release(core::ptr::addr_of_mut!(view)) };
    Py_XDECREF!(s_buffer);
    result
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.floatobject._PyFloat_ExactDealloc")]
extern "C-unwind" fn _PyFloat_ExactDealloc(obj: *mut _object) {
    _PyFreeList_Free(
        unsafe { core::ptr::addr_of_mut!((*_Py_freelists_GET()).floats) },
        _PyObject_CAST!(obj) as *mut (),
        100i64,
        unsafe {
            core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
                PyObject_Free as *const (),
            )
        },
    );
}

#[unsafe(link_section = ".text.slate_distinct.floatobject.float_dealloc")]
extern "C-unwind" fn float_dealloc(op: *mut _object) {
    if PyFloat_CheckExact!(op) != 0 {
        _PyFloat_ExactDealloc(op);
    } else {
        unsafe { unsafe { (*Py_TYPE!(op)).tp_free }.unwrap()(op as *mut ()) };
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyFloat_AsDouble(op: *mut _object) -> f64 {
    let mut val: f64 = 0 as f64;
    if op.is_null() {
        unsafe { PyErr_BadArgument() };
        return 1i32.wrapping_neg() as f64;
    }
    if PyFloat_Check!(op) != 0 {
        return PyFloat_AS_DOUBLE!(op);
    }
    let nb: *mut __SlateRecord_object_h_106 = unsafe { (*Py_TYPE!(op)).tp_as_number };
    if nb.is_null() || (unsafe { (*nb).nb_float }) == None {
        if !nb.is_null() && (unsafe { (*nb).nb_index }) != None {
            let res: *mut _object = unsafe { _PyNumber_Index(op) };
            if res.is_null() {
                return 1i32.wrapping_neg() as f64;
            }
            let val: f64 = unsafe { PyLong_AsDouble(res) };
            Py_DECREF!(res);
            return val;
        }
        unsafe {
            PyErr_Format(
                PyExc_TypeError,
                c"must be real number, not %.50s".as_ptr(),
                (*Py_TYPE!(op)).tp_name,
            )
        };
        return 1i32.wrapping_neg() as f64;
    }
    let res: *mut _object = unsafe { unsafe { (*nb).nb_float }.unwrap()(op) };
    if res.is_null() {
        return 1i32.wrapping_neg() as f64;
    }
    if PyFloat_CheckExact!(res) == 0 {
        if PyFloat_Check!(res) == 0 {
            unsafe {
                PyErr_Format(
                    PyExc_TypeError,
                    c"%.50s.__float__ returned non-float (type %.50s)".as_ptr(),
                    (*Py_TYPE!(op)).tp_name,
                    (*Py_TYPE!(res)).tp_name,
                )
            };
            Py_DECREF!(res);
            return 1i32.wrapping_neg() as f64;
        }
        if (unsafe {
            PyErr_WarnFormat(PyExc_DeprecationWarning, 1i64, c"%.50s.__float__ returned non-float (type %.50s).  The ability to return an instance of a strict subclass of float is deprecated, and may be removed in a future version of Python.".as_ptr(), (*Py_TYPE!(op)).tp_name, (*Py_TYPE!(res)).tp_name)
        }) != 0
        {
            Py_DECREF!(res);
            return 1i32.wrapping_neg() as f64;
        }
    }
    val = PyFloat_AS_DOUBLE!(res);
    Py_DECREF!(res);
    val
}

// Methods
#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_convert_int_to_double(v: *mut *mut _object, dbl: *mut f64) -> i32 {
    let obj: *mut _object = unsafe { *v };
    if PyLong_Check!(obj) != 0 {
        unsafe {
            *dbl = PyLong_AsDouble(obj);
        }
        let __v0: bool = (unsafe { *dbl }) == -1.0f64 && !unsafe { PyErr_Occurred() }.is_null();
        if __v0 {
            unsafe {
                *v = core::ptr::null_mut::<_object>();
            }
            return 1i32.wrapping_neg();
        }
    } else {
        unsafe {
            *v = Py_NewRef!(core::ptr::addr_of_mut!(_Py_NotImplementedStruct));
        }
        return 1i32.wrapping_neg();
    }
    0
}

#[unsafe(link_section = ".text.slate_distinct.floatobject.float_repr")]
extern "C-unwind" fn float_repr(op: *mut _object) -> *mut _object {
    let v: *mut __SlateRecord_floatobject_h_8 = _PyFloat_CAST!(op);
    let buf: *mut i8 = unsafe {
        PyOS_double_to_string(
            PyFloat_AS_DOUBLE__2!(v),
            114i8,
            0i32,
            2i32,
            core::ptr::null_mut::<i32>(),
        )
    };
    if buf.is_null() {
        return unsafe { PyErr_NoMemory() };
    }
    let result: *mut _object = unsafe {
        _PyUnicode_FromASCII(
            buf as *const i8,
            (strlen((buf as *const i8) as *const core::ffi::c_char) as u64) as i64,
        )
    };
    unsafe { PyMem_Free(buf as *mut ()) };
    result
}

// Comparison is pretty much a nightmare.  When comparing float to float,
// we do it as straightforwardly (and long-windedly) as conceivable, so
// that, e.g., Python x == y delivers the same result as the platform
// C x == y when x and/or y is a NaN.
// When mixing float with an integer type, there's no good *uniform* approach.
// Converting the double to an integer obviously doesn't work, since we
// may lose info from fractional bits.  Converting the integer to a double
// also has two failure modes:  (1) an int may trigger overflow (too
// large to fit in the dynamic range of a C double); (2) even a C long may have
// more bits than fit in a C double (e.g., on a 64-bit box long may have
// 63 bits of precision, but a C double probably has only 53), and then
// we can falsely claim equality when low-order integer bits are lost by
// coercion to double.  So this part is painful too.
#[unsafe(link_section = ".text.slate_distinct.floatobject.float_richcompare")]
extern "C-unwind" fn float_richcompare(
    v: *mut _object,
    w: *mut _object,
    mut op: i32,
) -> *mut _object {
    let mut vv: *mut _object = unsafe { core::mem::zeroed() };
    let mut result: *mut _object = unsafe { core::mem::zeroed() };
    let mut intpart: f64 = unsafe { core::mem::zeroed() };
    let mut nbits: i32 = unsafe { core::mem::zeroed() };
    // The signs are the same.
    //
    // Convert w to a double if it fits.  In particular, 0 fits.
    let mut nbits64: i64 = unsafe { core::mem::zeroed() };
    let mut exponent: i32 = unsafe { core::mem::zeroed() };
    let mut wsign: i32 = unsafe { core::mem::zeroed() };
    let mut vsign: i32 = unsafe { core::mem::zeroed() };
    let mut r: i32 = unsafe { core::mem::zeroed() };
    let mut j: f64 = unsafe { core::mem::zeroed() };
    let mut i: f64 = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_8: {
            '__join_9: {
                r = 0;
                i = PyFloat_AS_DOUBLE!(v);
                // Switch on the type of w.  Set i and j to doubles to be compared,
                // and op to the richcomp to use.
                if PyFloat_Check!(w) != 0 {
                    j = PyFloat_AS_DOUBLE!(w);
                } else {
                    if !i.is_finite() {
                        if PyLong_Check!(w) != 0 {
                            j = 0.0f64;
                            // If i is an infinity, its magnitude exceeds any
                            // finite integer, so it doesn't matter which int we
                            // compare i with.  If i is a NaN, similarly.
                            break '__join_9;
                        }
                    } else {
                        if PyLong_Check!(w) != 0 {
                            vsign = if i == 0.0f64 {
                                0
                            } else {
                                if i < 0.0f64 { 1i32.wrapping_neg() } else { 1 }
                            };
                            PyLong_GetSign(w, core::ptr::addr_of_mut!(wsign));
                            if vsign != wsign {
                                // Magnitudes are irrelevant -- the signs alone
                                // determine the outcome.
                                i = vsign as f64;
                                j = wsign as f64;
                                break '__join_8;
                            } else {
                                nbits64 = _PyLong_NumBits(w);
                                if nbits64 > 1024 {
                                    // This Python integer is larger than any finite C double.
                                    // Replace with little doubles
                                    // that give the same outcome -- w is so large that
                                    // its magnitude must exceed the magnitude of any
                                    // finite float.
                                    i = vsign as f64;
                                    j = (wsign as f64) * 2.0f64;
                                    break '__join_8;
                                } else {
                                    nbits = nbits64 as i32;
                                    if nbits <= 48 {
                                        j = PyLong_AsDouble(w);
                                        // It's impossible that <= 48 bits overflowed.
                                        break '__join_8;
                                    } else {
                                        // else nbits was 0
                                        // if vsign were 0, then since wsign is
                                        // not 0, we would have taken the
                                        // vsign != wsign branch at the start
                                        frexp(i, core::ptr::addr_of_mut!(exponent));
                                        // exponent is the # of bits in v before the radix point;
                                        // we know that nbits (the # of bits in w) > 48 at this point
                                        if exponent < nbits {
                                            j = i;
                                            i = 0.0f64;
                                            break '__join_8;
                                        } else {
                                            if exponent > nbits {
                                                j = 0.0f64;
                                                break '__join_8;
                                            } else {
                                                '__join_16: {
                                                    result = core::ptr::null_mut::<_object>();
                                                    vv = core::ptr::null_mut::<_object>();
                                                    // v and w have the same number of bits before the radix
                                                    // point.  Construct an int from the integer part of v and
                                                    // update op if necessary, so comparing two ints has the same outcome.
                                                    let fracpart: f64 =
                                                        modf(i, core::ptr::addr_of_mut!(intpart));
                                                    if fracpart != 0.0f64 {
                                                        '__join_18: {
                                                            let __t1: i32 = op;
                                                            if __t1 == 2 {
                                                                return _PyObject_CAST__2!(
                                                                    core::ptr::addr_of_mut!(
                                                                        _Py_FalseStruct
                                                                    )
                                                                );
                                                            } else {
                                                                if __t1 == 3 {
                                                                    return _PyObject_CAST__2!(
                                                                        core::ptr::addr_of_mut!(
                                                                            _Py_TrueStruct
                                                                        )
                                                                    );
                                                                } else {
                                                                    if __t1 == 0 {
                                                                        break '__join_18;
                                                                    } else {
                                                                        if __t1 == 1 {
                                                                            break '__join_18;
                                                                        } else {
                                                                            if __t1 != 4 {
                                                                                if __t1 != 5 {
                                                                                    break '__join_16;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            op = if vsign > 0 { 5 } else { 4 };
                                                            break '__join_16;
                                                        }
                                                        op = if vsign > 0 { 0 } else { 1 };
                                                    }
                                                }
                                                vv = PyLong_FromDouble(intpart);
                                                if !vv.is_null() {
                                                    r = PyObject_RichCompareBool(vv, w, op);
                                                    if r >= 0 {
                                                        result = PyBool_FromLong(r as i64);
                                                    }
                                                }
                                                Py_XDECREF!(vv);
                                                return result;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    return core::ptr::addr_of_mut!(_Py_NotImplementedStruct);
                }
            }
            // else if (PyLong_Check(w))
            // w isn't float or int
        }
        let __t0: i32 = op;
        if __t0 == 2 {
            r = (i == j) as i32;
        } else {
            if __t0 == 3 {
                r = (i != j) as i32;
            } else {
                if __t0 == 1 {
                    r = (i <= j) as i32;
                } else {
                    if __t0 == 5 {
                        r = (i >= j) as i32;
                    } else {
                        if __t0 == 0 {
                            r = (i < j) as i32;
                        } else {
                            if __t0 == 4 {
                                r = (i > j) as i32;
                            }
                        }
                    }
                }
            }
        }
        return PyBool_FromLong(r as i64);
    }
    // Non-integer float never equals to an int.
    // For non-integer float, v <= w <=> v < w.
    // If v > 0: trunc(v) < v < trunc(v) + 1
    //   v < w => trunc(v) < w
    //   trunc(v) < w => trunc(v) + 1 <= w => v < w
    // If v < 0: trunc(v) - 1 < v < trunc(v)
    //   v < w => trunc(v) - 1 < w => trunc(v) <= w
    //   trunc(v) <= w => v < w
    // The same as above, but with opposite directions.
}

#[unsafe(link_section = ".text.slate_distinct.floatobject.float_hash")]
extern "C-unwind" fn float_hash(op: *mut _object) -> i64 {
    let v: *mut __SlateRecord_floatobject_h_8 = _PyFloat_CAST!(op);
    unsafe { _Py_HashDouble(op, (*v).ob_fval) }
}

#[unsafe(link_section = ".text.slate_distinct.floatobject.float_add")]
extern "C-unwind" fn float_add(mut v: *mut _object, mut w: *mut _object) -> *mut _object {
    let mut a: f64 = 0 as f64;
    let mut b: f64 = 0 as f64;
    CONVERT_TO_DOUBLE!(v, a);
    CONVERT_TO_DOUBLE!(w, b);
    a += b;
    PyFloat_FromDouble(a)
}

#[unsafe(link_section = ".text.slate_distinct.floatobject.float_sub")]
extern "C-unwind" fn float_sub(mut v: *mut _object, mut w: *mut _object) -> *mut _object {
    let mut a: f64 = 0 as f64;
    let mut b: f64 = 0 as f64;
    CONVERT_TO_DOUBLE!(v, a);
    CONVERT_TO_DOUBLE!(w, b);
    a -= b;
    PyFloat_FromDouble(a)
}

#[unsafe(link_section = ".text.slate_distinct.floatobject.float_mul")]
extern "C-unwind" fn float_mul(mut v: *mut _object, mut w: *mut _object) -> *mut _object {
    let mut a: f64 = 0 as f64;
    let mut b: f64 = 0 as f64;
    CONVERT_TO_DOUBLE!(v, a);
    CONVERT_TO_DOUBLE!(w, b);
    a *= b;
    PyFloat_FromDouble(a)
}

#[unsafe(link_section = ".text.slate_distinct.floatobject.float_div")]
extern "C-unwind" fn float_div(mut v: *mut _object, mut w: *mut _object) -> *mut _object {
    let mut a: f64 = 0 as f64;
    let mut b: f64 = 0 as f64;
    CONVERT_TO_DOUBLE!(v, a);
    CONVERT_TO_DOUBLE!(w, b);
    if b == 0.0f64 {
        unsafe { PyErr_SetString(PyExc_ZeroDivisionError, c"division by zero".as_ptr()) };
        return core::ptr::null_mut::<_object>();
    }
    a /= b;
    PyFloat_FromDouble(a)
}

#[unsafe(link_section = ".text.slate_distinct.floatobject.float_rem")]
extern "C-unwind" fn float_rem(mut v: *mut _object, mut w: *mut _object) -> *mut _object {
    let mut vx: f64 = 0 as f64;
    let mut wx: f64 = 0 as f64;
    let mut r#mod: f64 = 0 as f64;
    CONVERT_TO_DOUBLE!(v, vx);
    CONVERT_TO_DOUBLE!(w, wx);
    if wx == 0.0f64 {
        unsafe { PyErr_SetString(PyExc_ZeroDivisionError, c"division by zero".as_ptr()) };
        return core::ptr::null_mut::<_object>();
    }
    r#mod = unsafe { fmod(vx, wx) };
    if r#mod != 0.0f64 {
        // ensure the remainder has the same sign as the denominator
        if ((wx < (0i32 as f64)) as i32) != ((r#mod < (0i32 as f64)) as i32) {
            r#mod += wx;
        }
    } else {
        // the remainder is zero, and in the presence of signed zeroes
        // fmod returns different results across platforms; ensure
        // it has the same sign as the denominator.
        r#mod = unsafe { copysign(0.0f64, wx) };
    }
    PyFloat_FromDouble(r#mod)
}

fn _float_div_mod(vx: f64, wx: f64, floordiv: &mut f64, r#mod: &mut f64) {
    let mut div: f64 = 0 as f64;
    unsafe {
        *(r#mod as *mut f64) = fmod(vx, wx);
    }
    // fmod is typically exact, so vx-mod is *mathematically* an
    // exact multiple of wx.  But this is fp arithmetic, and fp
    // vx - mod is an approximation; the result is that div may
    // not be an exact integral value after the division, although
    // it will always be very close to one.
    div = (vx - unsafe { *(r#mod as *mut f64) }) / wx;
    if (unsafe { *(r#mod as *mut f64) }) != 0.0f64 {
        // ensure the remainder has the same sign as the denominator
        if ((wx < (0i32 as f64)) as i32)
            != (((unsafe { *(r#mod as *mut f64) }) < (0i32 as f64)) as i32)
        {
            unsafe {
                *r#mod = *r#mod + wx;
            }
            let __v4: f64 = div - 1.0f64;
            div = __v4;
        }
    } else {
        // the remainder is zero, and in the presence of signed zeroes
        // fmod returns different results across platforms; ensure
        // it has the same sign as the denominator.
        unsafe {
            *(r#mod as *mut f64) = copysign(0.0f64, wx);
        }
    }
    // snap quotient to nearest integral value
    if div != 0.0f64 {
        unsafe {
            *(floordiv as *mut f64) = floor(div);
        }
        if div - unsafe { *(floordiv as *mut f64) } > 0.5f64 {
            let __v7: f64 = (unsafe { *floordiv }) + 1.0f64;
            unsafe {
                *floordiv = __v7;
            }
        }
    } else {
        // div is zero - get the same sign as the true quotient
        unsafe {
            *(floordiv as *mut f64) = copysign(0.0f64, vx / wx);
        }
        // zero w/ sign of vx/wx
    }
}

#[unsafe(link_section = ".text.slate_distinct.floatobject.float_divmod")]
extern "C-unwind" fn float_divmod(mut v: *mut _object, mut w: *mut _object) -> *mut _object {
    let mut vx: f64 = 0 as f64;
    let mut wx: f64 = 0 as f64;
    let mut r#mod: f64 = 0 as f64;
    let mut floordiv: f64 = 0 as f64;
    CONVERT_TO_DOUBLE!(v, vx);
    CONVERT_TO_DOUBLE!(w, wx);
    if wx == 0.0f64 {
        unsafe { PyErr_SetString(PyExc_ZeroDivisionError, c"division by zero".as_ptr()) };
        return core::ptr::null_mut::<_object>();
    }
    _float_div_mod(
        vx,
        wx,
        unsafe { &mut (*core::ptr::addr_of_mut!(floordiv)) },
        unsafe { &mut (*core::ptr::addr_of_mut!(r#mod)) },
    );
    unsafe { Py_BuildValue(c"(dd)".as_ptr(), floordiv, r#mod) }
}

#[unsafe(link_section = ".text.slate_distinct.floatobject.float_floor_div")]
extern "C-unwind" fn float_floor_div(mut v: *mut _object, mut w: *mut _object) -> *mut _object {
    let mut vx: f64 = 0 as f64;
    let mut wx: f64 = 0 as f64;
    let mut r#mod: f64 = 0 as f64;
    let mut floordiv: f64 = 0 as f64;
    CONVERT_TO_DOUBLE!(v, vx);
    CONVERT_TO_DOUBLE!(w, wx);
    if wx == 0.0f64 {
        unsafe { PyErr_SetString(PyExc_ZeroDivisionError, c"division by zero".as_ptr()) };
        return core::ptr::null_mut::<_object>();
    }
    _float_div_mod(
        vx,
        wx,
        unsafe { &mut (*core::ptr::addr_of_mut!(floordiv)) },
        unsafe { &mut (*core::ptr::addr_of_mut!(r#mod)) },
    );
    PyFloat_FromDouble(floordiv)
}

#[unsafe(link_section = ".text.slate_distinct.floatobject.float_pow")]
extern "C-unwind" fn float_pow(
    mut v: *mut _object,
    mut w: *mut _object,
    z: *mut _object,
) -> *mut _object {
    let mut iv: f64 = 0 as f64;
    let mut iw: f64 = 0 as f64;
    let mut ix: f64 = 0 as f64;
    let mut negate_result: i32 = 0;
    if z != unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } {
        unsafe {
            PyErr_SetString(
                PyExc_TypeError,
                c"pow() 3rd argument not allowed unless all arguments are integers".as_ptr(),
            )
        };
        return core::ptr::null_mut::<_object>();
    }
    CONVERT_TO_DOUBLE!(v, iv);
    CONVERT_TO_DOUBLE!(w, iw);
    // Sort out special cases here instead of relying on pow()
    if iw == (0i32 as f64) {
        // v**0 is 1, even 0**0
        return PyFloat_FromDouble(1.0f64);
    }
    if iv.is_nan() {
        // nan**w = nan, unless w == 0
        return PyFloat_FromDouble(iv);
    }
    if iw.is_nan() {
        // v**nan = nan, unless v == 1; 1**nan = 1
        return PyFloat_FromDouble(if iv == 1.0f64 { 1.0f64 } else { iw });
    }
    if (if iw.is_infinite() {
        if iw.is_sign_negative() { -1i32 } else { 1i32 }
    } else {
        0i32
    }) != 0
    {
        // v**inf is: 0.0 if abs(v) < 1; 1.0 if abs(v) == 1; inf if
        //     abs(v) > 1 (including case where v infinite)
        //
        // v**-inf is: inf if abs(v) < 1; 1.0 if abs(v) == 1; 0.0 if
        //     abs(v) > 1 (including case where v infinite)
        iv = unsafe { fabs(iv) };
        if iv == 1.0f64 {
            return PyFloat_FromDouble(1.0f64);
        } else {
            if ((iw > 0.0f64) as i32) == ((iv > 1.0f64) as i32) {
                return PyFloat_FromDouble(unsafe { fabs(iw) });
            } else {
                return PyFloat_FromDouble(0.0f64);
            }
        }
        // return inf
    }
    if (if iv.is_infinite() {
        if iv.is_sign_negative() { -1i32 } else { 1i32 }
    } else {
        0i32
    }) != 0
    {
        // (+-inf)**w is: inf for w positive, 0 for w negative; in
        // both cases, we need to add the appropriate sign if w is
        // an odd integer.
        let iw_is_odd: i32 = DOUBLE_IS_ODD_INTEGER!(iw) as i32;
        if iw > 0.0f64 {
            return PyFloat_FromDouble(if iw_is_odd != 0 {
                iv
            } else {
                unsafe { fabs(iv) }
            });
        } else {
            return PyFloat_FromDouble(if iw_is_odd != 0 {
                unsafe { copysign(0.0f64, iv) }
            } else {
                0.0f64
            });
        }
    }
    if iv == 0.0f64 {
        // 0**w is: 0 for w positive, 1 for w zero
        // (already dealt with above), and an error
        // if w is negative.
        let iw_is_odd: i32 = DOUBLE_IS_ODD_INTEGER!(iw) as i32;
        if iw < 0.0f64 {
            unsafe {
                PyErr_SetString(
                    PyExc_ZeroDivisionError,
                    c"zero to a negative power".as_ptr(),
                )
            };
            return core::ptr::null_mut::<_object>();
        }
        // use correct sign if iw is odd
        return PyFloat_FromDouble(if iw_is_odd != 0 { iv } else { 0.0f64 });
    }
    if iv < 0.0f64 {
        // Whether this is an error is a mess, and bumps into libm
        // bugs so we have to figure it out ourselves.
        if iw != unsafe { floor(iw) } {
            // Negative numbers raised to fractional powers
            // become complex.
            return unsafe { unsafe { (*PyComplex_Type.tp_as_number).nb_power }.unwrap()(v, w, z) };
        }
        // iw is an exact integer, albeit perhaps a very large
        // one.  Replace iv by its absolute value and remember
        // to negate the pow result if iw is odd.
        iv = -iv;
        negate_result = DOUBLE_IS_ODD_INTEGER!(iw) as i32;
    }
    if iv == 1.0f64 {
        // 1**w is 1, even 1**inf and 1**nan
        // (-1) ** large_integer also ends up here.  Here's an
        // extract from the comments for the previous
        // implementation explaining why this special case is
        // necessary:
        //
        // -1 raised to an exact integer should never be exceptional.
        // Alas, some libms (chiefly glibc as of early 2003) return
        // NaN and set EDOM on pow(-1, large_int) if the int doesn't
        // happen to be representable in a *C* integer.  That's a
        // bug.
        return PyFloat_FromDouble(if negate_result != 0 { -1.0f64 } else { 1.0f64 });
    }
    // Now iv and iw are finite, iw is nonzero, and iv is
    // positive and not equal to 1.0.  We finally allow
    // the platform pow to step in and do the rest.
    unsafe {
        *slate_support::errno_location() = 0;
    }
    ix = unsafe { pow(iv, iw) };
    _Py_ADJUST_ERANGE1(ix);
    if negate_result != 0 {
        ix = -ix;
    }
    if (unsafe { *slate_support::errno_location() }) != 0 {
        // We don't expect any errno value other than ERANGE, but
        // the range of libm bugs appears unbounded.
        unsafe {
            PyErr_SetFromErrno(
                if *slate_support::errno_location() == (slate_support::ERANGE as i32) {
                    PyExc_OverflowError
                } else {
                    PyExc_ValueError
                },
            )
        };
        return core::ptr::null_mut::<_object>();
    }
    PyFloat_FromDouble(ix)
}

#[unsafe(link_section = ".text.slate_distinct.floatobject.float_neg")]
extern "C-unwind" fn float_neg(op: *mut _object) -> *mut _object {
    let v: *mut __SlateRecord_floatobject_h_8 = _PyFloat_CAST!(op);
    PyFloat_FromDouble(-unsafe { (*v).ob_fval })
}

#[unsafe(link_section = ".text.slate_distinct.floatobject.float_abs")]
extern "C-unwind" fn float_abs(op: *mut _object) -> *mut _object {
    let v: *mut __SlateRecord_floatobject_h_8 = _PyFloat_CAST!(op);
    PyFloat_FromDouble(unsafe { fabs((*v).ob_fval) })
}

#[unsafe(link_section = ".text.slate_distinct.floatobject.float_bool")]
extern "C-unwind" fn float_bool(op: *mut _object) -> i32 {
    let v: *mut __SlateRecord_floatobject_h_8 = _PyFloat_CAST!(op);
    ((unsafe { (*v).ob_fval }) != 0.0f64) as i32
}

// [clinic input]
// float.is_integer
//
// Return True if the float is an integer.
// [clinic start generated code]
/// [clinic end generated code: output=7112acf95a4d31ea input=311810d3f777e10d]
fn float_is_integer_impl(self_: *mut _object) -> *mut _object {
    let x: f64 = PyFloat_AsDouble(self_);
    let __v0: bool = x == -1.0f64 && !unsafe { PyErr_Occurred() }.is_null();
    if __v0 {
        return core::ptr::null_mut::<_object>();
    }
    if !x.is_finite() {
        return _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_FalseStruct) });
    }
    unsafe {
        *slate_support::errno_location() = 0;
    }
    let o: *mut _object = if (unsafe { floor(x) }) == x {
        _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_TrueStruct) })
    } else {
        _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_FalseStruct) })
    };
    if (unsafe { *slate_support::errno_location() }) != 0 {
        unsafe {
            PyErr_SetFromErrno(
                if *slate_support::errno_location() == (slate_support::ERANGE as i32) {
                    PyExc_OverflowError
                } else {
                    PyExc_ValueError
                },
            )
        };
        return core::ptr::null_mut::<_object>();
    }
    Py_NewRef!(o)
}

// [clinic input]
// float.__trunc__
//
// Return the Integral closest to x between 0 and x.
// [clinic start generated code]
/// [clinic end generated code: output=dd3e289dd4c6b538 input=591b9ba0d650fdff]
#[unsafe(link_section = ".text.slate_distinct.floatobject.float___trunc___impl")]
extern "C-unwind" fn float___trunc___impl(self_: *mut _object) -> *mut _object {
    unsafe { PyLong_FromDouble(PyFloat_AS_DOUBLE!(self_)) }
}

// [clinic input]
// float.__floor__
//
// Return the floor as an Integral.
// [clinic start generated code]
/// [clinic end generated code: output=e0551dbaea8c01d1 input=77bb13eb12e268df]
fn float___floor___impl(self_: *mut _object) -> *mut _object {
    let x: f64 = PyFloat_AS_DOUBLE!(self_);
    unsafe { PyLong_FromDouble(floor(x)) }
}

// [clinic input]
// float.__ceil__
//
// Return the ceiling as an Integral.
// [clinic start generated code]
/// [clinic end generated code: output=a2fd8858f73736f9 input=79e41ae94aa0a516]
fn float___ceil___impl(self_: *mut _object) -> *mut _object {
    let x: f64 = PyFloat_AS_DOUBLE!(self_);
    unsafe { PyLong_FromDouble(ceil(x)) }
}

// double_round: rounds a finite double to the closest multiple of
// 10**-ndigits; here ndigits is within reasonable bounds (typically, -308 <=
// ndigits <= 323).  Returns a Python float, or sets a Python error and
// returns NULL on failure (OverflowError and memory errors are possible).
// version of double_round that uses the correctly-rounded string<->double
// conversions from Python/dtoa.c
fn double_round(x: f64, ndigits: i32) -> *mut _object {
    let mut new_387controlword: u16 = unsafe { core::mem::zeroed() };
    let mut old_387controlword: u16 = unsafe { core::mem::zeroed() };
    let mut result: *mut _object = unsafe { core::mem::zeroed() };
    let mut sign: i32 = unsafe { core::mem::zeroed() };
    let mut decpt: i32 = unsafe { core::mem::zeroed() };
    let mut mybuf: *mut i8 = unsafe { core::mem::zeroed() };
    let mut shortbuf: core::mem::MaybeUninit<__SlateAlign16<[i8; 100]>> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_6: *mut [i8; 100] = core::ptr::addr_of_mut!(shortbuf) as *mut [i8; 100];
    let mut buf_end: *mut i8 = unsafe { core::mem::zeroed() };
    let mut buf: *mut i8 = unsafe { core::mem::zeroed() };
    let mut mybuflen: i64 = unsafe { core::mem::zeroed() };
    let mut buflen: i64 = unsafe { core::mem::zeroed() };
    let mut rounded: f64 = unsafe { core::mem::zeroed() };
    unsafe {
        mybuflen = 100;
        mybuf = (*__slate_slot_6).as_mut_ptr() as *mut i8;
        result = core::ptr::null_mut::<_object>();
        // round to a decimal string
        loop {
            old_387controlword = _Py_get_387controlword();
            new_387controlword = ((old_387controlword as i32) & !3840 | 512) as u16;
            if (new_387controlword as i32) != (old_387controlword as i32) {
                _Py_set_387controlword(new_387controlword);
            }
            if true {
                break;
            }
        }
        buf = _Py_dg_dtoa(
            x,
            3i32,
            ndigits,
            core::ptr::addr_of_mut!(decpt),
            core::ptr::addr_of_mut!(sign),
            core::ptr::addr_of_mut!(buf_end),
        );
        loop {
            if (new_387controlword as i32) != (old_387controlword as i32) {
                _Py_set_387controlword(old_387controlword);
            }
            if true {
                break;
            }
        }
        if buf.is_null() {
            PyErr_NoMemory();
            return core::ptr::null_mut::<_object>();
        } else {
            '__join_0: {
                // Get new buffer if shortbuf is too small.  Space needed <= buf_end -
                // buf + 8: (1 extra for '0', 1 for sign, 5 for exp, 1 for '\0').
                buflen = buf_end.offset_from(buf as *mut i8) as i64;
                if buflen.wrapping_add(8) > mybuflen {
                    mybuflen = buflen.wrapping_add(8);
                    mybuf = PyMem_Malloc(mybuflen as u64) as *mut i8;
                    if mybuf.is_null() {
                        PyErr_NoMemory();
                        break '__join_0;
                    }
                }
                // copy buf to mybuf, adding exponent, sign and leading 0
                PyOS_snprintf(
                    mybuf,
                    mybuflen as u64,
                    c"%s0%se%d".as_ptr(),
                    if sign != 0 {
                        c"-".as_ptr() as *mut i8
                    } else {
                        c"".as_ptr() as *mut i8
                    },
                    buf,
                    decpt.wrapping_sub(buflen as i32),
                );
                // and convert the resulting string back to a double
                *slate_support::errno_location() = 0;
                loop {
                    old_387controlword = _Py_get_387controlword();
                    new_387controlword = ((old_387controlword as i32) & !3840 | 512) as u16;
                    if (new_387controlword as i32) != (old_387controlword as i32) {
                        _Py_set_387controlword(new_387controlword);
                    }
                    if true {
                        break;
                    }
                }
                rounded = _Py_dg_strtod(mybuf as *const i8, core::ptr::null_mut::<*mut i8>());
                loop {
                    if (new_387controlword as i32) != (old_387controlword as i32) {
                        _Py_set_387controlword(old_387controlword);
                    }
                    if true {
                        break;
                    }
                }
                if *slate_support::errno_location() == (slate_support::ERANGE as i32)
                    && fabs(rounded) >= 1.0f64
                {
                    PyErr_SetString(
                        PyExc_OverflowError,
                        c"rounded value too large to represent".as_ptr(),
                    );
                } else {
                    result = PyFloat_FromDouble(rounded);
                }
                // done computing value;  now clean up
                if mybuf != ((*__slate_slot_6).as_mut_ptr() as *mut i8) {
                    PyMem_Free(mybuf as *mut ());
                }
            }
            _Py_dg_freedtoa(buf);
            return result;
        }
    }
}

// round a Python float v to the closest multiple of 10**-ndigits
// [clinic input]
// float.__round__
//
//     ndigits as o_ndigits: object = None
//     /
//
// Return the Integral closest to x, rounding half toward even.
//
// When an argument is passed, work like built-in round(x, ndigits).
// [clinic start generated code]
/// [clinic end generated code: output=374c36aaa0f13980 input=fc0fe25924fbc9ed]
fn float___round___impl(self_: *mut _object, o_ndigits: *mut _object) -> *mut _object {
    let x: f64 = PyFloat_AsDouble(self_);
    let mut rounded: f64 = 0 as f64;
    let mut ndigits: i64 = 0;
    if o_ndigits == unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } {
        // single-argument round or with None ndigits:
        // round to nearest integer
        rounded = unsafe { round(x) };
        if (unsafe { fabs(x - rounded) }) == 0.5f64 {
            rounded = 2.0f64 * unsafe { round(x / 2.0f64) };
        }
        // halfway case: round to even
        return unsafe { PyLong_FromDouble(rounded) };
    }
    // interpret second argument as a Py_ssize_t; clips on overflow
    ndigits = unsafe { PyNumber_AsSsize_t(o_ndigits, core::ptr::null_mut::<_object>()) };
    let __v0: bool =
        ndigits == (1i32.wrapping_neg() as i64) && !unsafe { PyErr_Occurred() }.is_null();
    if __v0 {
        return core::ptr::null_mut::<_object>();
    }
    // nans and infinities round to themselves
    if !x.is_finite() {
        return PyFloat_FromDouble(x);
    }
    // Deal with extreme values for ndigits. For ndigits > NDIGITS_MAX, x
    // always rounds to itself.  For ndigits < NDIGITS_MIN, x always
    // rounds to +-0.0.  Here 0.30103 is an upper bound for log10(2).
    if ndigits
        > ((((53i32.wrapping_sub(1021i32.wrapping_neg()) as f64) * 0.30103f64) as i32) as i64)
    {
        return PyFloat_FromDouble(x);
    } else {
        if ndigits < ((((1025i32 as f64) * 0.30103f64) as i32).wrapping_neg() as i64) {
            return PyFloat_FromDouble(0.0f64 * x);
        } else {
            return double_round(x, ndigits as i32);
        }
    }
    // return x
    // return 0.0, but with sign of x
    // finite x, and ndigits is not unreasonably large
}

#[unsafe(link_section = ".text.slate_distinct.floatobject.float_float")]
extern "C-unwind" fn float_float(v: *mut _object) -> *mut _object {
    if PyFloat_CheckExact!(v) != 0 {
        return Py_NewRef!(v);
    } else {
        return PyFloat_FromDouble(unsafe { (*(v as *mut __SlateRecord_floatobject_h_8)).ob_fval });
    }
}

// [clinic input]
// float.conjugate
//
// Return self, the complex conjugate of any float.
// [clinic start generated code]
/// [clinic end generated code: output=8ca292c2479194af input=82ba6f37a9ff91dd]
fn float_conjugate_impl(self_: *mut _object) -> *mut _object {
    float_float(self_)
}

// turn ASCII hex characters into integer values and vice versa
fn char_from_hex(x: i32) -> i8 {
    unsafe { *unsafe { Py_hexdigits }.wrapping_offset(x as isize) }
}

/// This table maps characters to their hexadecimal values, only
/// works with encodings whose lower half is ASCII (like UTF-8).
/// '0' maps to 0, ..., '9' maps to 9.
/// 'a' and 'A' map to 10, ..., 'f' and 'F' map to 15.
/// All other indices map to -1.
static _CHAR_TO_HEX: __SlateAlign16<[i32; 256]> = __SlateAlign16([
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    0i32,
    1i32,
    2i32,
    3i32,
    4i32,
    5i32,
    6i32,
    7i32,
    8i32,
    9i32,
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    10i32,
    11i32,
    12i32,
    13i32,
    14i32,
    15i32,
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    10i32,
    11i32,
    12i32,
    13i32,
    14i32,
    15i32,
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
    1i32.wrapping_neg(),
]);

/// Convert a character to its hexadecimal value, or -1 if it's not a
/// valid hexadecimal character, only works with encodings whose lower
/// half is ASCII (like UTF-8).
fn hex_from_char(c: u8) -> i32 {
    unsafe { _CHAR_TO_HEX.0[(c as i32) as usize] }
}

// convert a float to a hexadecimal string
// TOHEX_NBITS is DBL_MANT_DIG rounded up to the next integer
// of the form 4k+1.
// [clinic input]
// float.hex
//
// Return a hexadecimal representation of a floating-point number.
//
// >>> (-0.1).hex()
// '-0x1.999999999999ap-4'
// >>> 3.14159.hex()
// '0x1.921f9f01b866ep+1'
// [clinic start generated code]
/// [clinic end generated code: output=0ebc9836e4d302d4 input=bec1271a33d47e67]
fn float_hex_impl(mut self_: *mut _object) -> *mut _object {
    let mut x: f64 = 0 as f64;
    let mut m: f64 = 0 as f64;
    let mut e: i32 = 0;
    let mut shift: i32 = 0;
    let mut i: i32 = 0;
    let mut si: i32 = 0;
    let mut esign: i32 = 0;
    // Space for 1+(TOHEX_NBITS-1)/4 digits, a decimal point, and the
    // trailing NUL byte.
    let mut s: __SlateAlign16<[i8; 16]> = __SlateAlign16([0i8; 16]);
    CONVERT_TO_DOUBLE!(self_, x);
    if x.is_nan()
        || (if x.is_infinite() {
            if x.is_sign_negative() { -1i32 } else { 1i32 }
        } else {
            0i32
        }) != 0
    {
        return float_repr(self_);
    }
    if x == 0.0f64 {
        if (unsafe { copysign(1.0f64, x) }) == -1.0f64 {
            return unsafe { PyUnicode_FromString(c"-0x0.0p+0".as_ptr()) };
        } else {
            return unsafe { PyUnicode_FromString(c"0x0.0p+0".as_ptr()) };
        }
    }
    m = unsafe { frexp(fabs(x), core::ptr::addr_of_mut!(e)) };
    shift = 1i32.wrapping_sub(Py_MAX!(1021i32.wrapping_neg().wrapping_sub(e), 0i32));
    m = unsafe { ldexp(m, shift) };
    e = e.wrapping_sub(shift);
    s.0[si as usize] = char_from_hex(m as i32);
    si = si.wrapping_add(1);
    m -= (m as i32) as f64;
    s.0[si as usize] = 46;
    si = si.wrapping_add(1);
    while i < 13i32 {
        let __v9: f64 = m * 16.0f64;
        m = __v9;
        s.0[si as usize] = char_from_hex(m as i32);
        si = si.wrapping_add(1);
        m -= (m as i32) as f64;
        i = i.wrapping_add(1);
    }
    s.0[si as usize] = 0;
    if e < 0 {
        esign = 45;
        e = e.wrapping_neg();
    } else {
        esign = 43;
    }
    if x < 0.0f64 {
        return unsafe {
            PyUnicode_FromFormat(
                c"-0x%sp%c%d".as_ptr(),
                s.0.as_mut_ptr() as *mut i8,
                esign,
                e,
            )
        };
    } else {
        return unsafe {
            PyUnicode_FromFormat(c"0x%sp%c%d".as_ptr(), s.0.as_mut_ptr() as *mut i8, esign, e)
        };
    }
}

// Convert a hexadecimal string to a float.
// [clinic input]
// @classmethod
// float.fromhex
//
//     string: object
//     /
//
// Create a floating-point number from a hexadecimal string.
//
// >>> float.fromhex('0x1.ffffp10')
// 2047.984375
// >>> float.fromhex('-0x1p-1074')
// -5e-324
// [clinic start generated code]
/// [clinic end generated code: output=c54b4923552e5af5 input=0407bebd354bca89]
fn float_fromhex_impl(r#type: *mut _typeobject, string: *mut _object) -> *mut _object {
    let mut __v2: bool = unsafe { core::mem::zeroed() };
    let mut __v7: bool = unsafe { core::mem::zeroed() };
    let mut __v8: bool = unsafe { core::mem::zeroed() };
    let mut __v17: bool = unsafe { core::mem::zeroed() };
    let mut i: i64 = unsafe { core::mem::zeroed() };
    let mut fdigits: i64 = unsafe { core::mem::zeroed() };
    let mut ndigits: i64 = unsafe { core::mem::zeroed() };
    let mut length: i64 = unsafe { core::mem::zeroed() };
    let mut negate: i32 = unsafe { core::mem::zeroed() };
    let mut round_up: i32 = unsafe { core::mem::zeroed() };
    let mut digit: i32 = unsafe { core::mem::zeroed() };
    let mut half_eps: i32 = unsafe { core::mem::zeroed() };
    let mut s_end: *const i8 = unsafe { core::mem::zeroed() };
    let mut exp_start: *const i8 = unsafe { core::mem::zeroed() };
    let mut coeff_end: *const i8 = unsafe { core::mem::zeroed() };
    let mut s_store: *const i8 = unsafe { core::mem::zeroed() };
    let mut coeff_start: *const i8 = unsafe { core::mem::zeroed() };
    let mut s: *const i8 = unsafe { core::mem::zeroed() };
    let mut key_digit: i64 = unsafe { core::mem::zeroed() };
    let mut lsb: i64 = unsafe { core::mem::zeroed() };
    let mut top_exp: i64 = unsafe { core::mem::zeroed() };
    let mut exp: i64 = unsafe { core::mem::zeroed() };
    let mut x: f64 = unsafe { core::mem::zeroed() };
    let mut result: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        negate = 0;
        // For the sake of simplicity and correctness, we impose an artificial
        // limit on ndigits, the total number of hex digits in the coefficient
        // The limit is chosen to ensure that, writing exp for the exponent,
        //
        //   (1) if exp > LONG_MAX/2 then the value of the hex string is
        //   guaranteed to overflow (provided it's nonzero)
        //
        //   (2) if exp < LONG_MIN/2 then the value of the hex string is
        //   guaranteed to underflow to 0.
        //
        //   (3) if LONG_MIN/2 <= exp <= LONG_MAX/2 then there's no danger of
        //   overflow in the calculation of exp and top_exp below.
        //
        // More specifically, ndigits is assumed to satisfy the following
        // inequalities:
        //
        //   4*ndigits <= DBL_MIN_EXP - DBL_MANT_DIG - LONG_MIN/2
        //   4*ndigits <= LONG_MAX/2 + 1 - DBL_MAX_EXP
        //
        // If either of these inequalities is not satisfied, a ValueError is
        // raised.  Otherwise, write x for the value of the hex string, and
        // assume x is nonzero.  Then
        //
        //   2**(exp-4*ndigits) <= |x| < 2**(exp+4*ndigits).
        //
        // Now if exp > LONG_MAX/2 then:
        //
        //   exp - 4*ndigits >= LONG_MAX/2 + 1 - (LONG_MAX/2 + 1 - DBL_MAX_EXP)
        //                    = DBL_MAX_EXP
        //
        // so |x| >= 2**DBL_MAX_EXP, which is too large to be stored in C
        // double, so overflows.  If exp < LONG_MIN/2, then
        //
        //   exp + 4*ndigits <= LONG_MIN/2 - 1 + (
        //                      DBL_MIN_EXP - DBL_MANT_DIG - LONG_MIN/2)
        //                    = DBL_MIN_EXP - DBL_MANT_DIG - 1
        //
        // and so |x| < 2**(DBL_MIN_EXP-DBL_MANT_DIG-1), hence underflows to 0
        // when converted to a C double.
        //
        // It's easy to show that if LONG_MIN/2 <= exp <= LONG_MAX/2 then both
        // exp+4*ndigits and exp-4*ndigits are within the range of a long.
        s = PyUnicode_AsUTF8AndSize(string, core::ptr::addr_of_mut!(length));
        if s.is_null() {
            return core::ptr::null_mut::<_object>();
        } else {
            s_end = s.wrapping_offset(length as isize);
            // Parse the string *
            // leading whitespace
            while Py_ISSPACE!(unsafe { *s }) != 0 {
                let __v43: *const i8 = s;
                s = __v43.wrapping_offset(1 as isize);
            }
            '__join_1: {
                '__join_7: {
                    // infinities and nans
                    x = _Py_parse_inf_or_nan(s, core::ptr::addr_of_mut!(coeff_end) as *mut *mut i8);
                    if coeff_end != s {
                        s = coeff_end;
                    } else {
                        // optional sign
                        if (*s as i32) == 45 {
                            let __v39: *const i8 = s;
                            s = __v39.wrapping_offset(1 as isize);
                            negate = 1;
                        } else {
                            if (*s as i32) == 43 {
                                let __v41: *const i8 = s;
                                s = __v41.wrapping_offset(1 as isize);
                            }
                        }
                        // [0x]
                        s_store = s;
                        if (*s as i32) == 48 {
                            let __v37: *const i8 = s;
                            s = __v37.wrapping_offset(1 as isize);
                            if (*s as i32) == 120 || (*s as i32) == 88 {
                                let __v35: *const i8 = s;
                                s = __v35.wrapping_offset(1 as isize);
                            } else {
                                s = s_store;
                            }
                        }
                        // coefficient: <integer> [. <fraction>]
                        coeff_start = s;
                        while hex_from_char(*s as u8) >= 0 {
                            let __v33: *const i8 = s;
                            s = __v33.wrapping_offset(1 as isize);
                        }
                        s_store = s;
                        if (*s as i32) == 46 {
                            let __v31: *const i8 = s;
                            s = __v31.wrapping_offset(1 as isize);
                            while hex_from_char(*s as u8) >= 0 {
                                let __v29: *const i8 = s;
                                s = __v29.wrapping_offset(1 as isize);
                            }
                            coeff_end = s.wrapping_offset(-(1 as isize));
                        } else {
                            coeff_end = s;
                        }
                        // ndigits = total # of hex digits; fdigits = # after point
                        ndigits = coeff_end.offset_from(coeff_start as *const i8) as i64;
                        fdigits = coeff_end.offset_from(s_store as *const i8) as i64;
                        if ndigits == 0 {
                            break '__join_1;
                        } else {
                            if ndigits
                                > Py_MIN!(
                                    (1021i32.wrapping_neg().wrapping_sub(53) as i64).wrapping_sub(
                                        9223372036854775807i64.wrapping_neg().wrapping_sub(1) / 2
                                    ),
                                    4611686018427386880i64
                                ) / 4
                            {
                                PyErr_SetString(
                                    PyExc_ValueError,
                                    c"hexadecimal string too long to convert".as_ptr(),
                                );
                                return core::ptr::null_mut::<_object>();
                            } else {
                                // [p <exponent>]
                                if (*s as i32) == 112 || (*s as i32) == 80 {
                                    let __v27: *const i8 = s;
                                    s = __v27.wrapping_offset(1 as isize);
                                    exp_start = s;
                                    if (*s as i32) == 45 || (*s as i32) == 43 {
                                        let __v25: *const i8 = s;
                                        s = __v25.wrapping_offset(1 as isize);
                                    }
                                    if !(48 <= (*s as i32) && (*s as i32) <= 57) {
                                        break '__join_1;
                                    } else {
                                        let __v23: *const i8 = s;
                                        s = __v23.wrapping_offset(1 as isize);
                                        while 48 <= (*s as i32) && (*s as i32) <= 57 {
                                            let __v21: *const i8 = s;
                                            s = __v21.wrapping_offset(1 as isize);
                                        }
                                        exp = strtol(
                                            exp_start,
                                            core::ptr::null_mut::<*mut i8>(),
                                            10i32,
                                        );
                                    }
                                } else {
                                    exp = 0;
                                }
                                // for 0 <= j < ndigits, HEX_DIGIT(j) gives the jth most significant digit
                                // Compute rounded value of the hex string *
                                // Discard leading zeros, and catch extreme overflow and underflow
                                loop {
                                    if ndigits > 0 {
                                        __v17 =
                                            HEX_DIGIT!(ndigits.wrapping_sub(1), fdigits, coeff_end)
                                                == 0;
                                    } else {
                                        __v17 = false;
                                    }
                                    if __v17 {
                                        ndigits = ndigits.wrapping_sub(1);
                                    } else {
                                        break;
                                    }
                                }
                                if ndigits == 0
                                    || exp
                                        < 9223372036854775807i64.wrapping_neg().wrapping_sub(1) / 2
                                {
                                    x = 0.0f64;
                                } else {
                                    '__join_2: {
                                        if exp <= 9223372036854775807 / 2 {
                                            // Adjust exponent for fractional part.
                                            exp = exp.wrapping_sub(4i64.wrapping_mul(fdigits));
                                            // top_exp = 1 more than exponent of most sig. bit of coefficient
                                            top_exp = exp.wrapping_add(
                                                4i64.wrapping_mul(ndigits.wrapping_sub(1)),
                                            );
                                            digit = HEX_DIGIT!(
                                                ndigits.wrapping_sub(1),
                                                fdigits,
                                                coeff_end
                                            );
                                            while digit != 0 {
                                                top_exp = top_exp.wrapping_add(1);
                                                digit /= 2;
                                            }
                                            // catch almost all nonextreme cases of overflow and underflow here
                                            if top_exp
                                                < (1021i32.wrapping_neg().wrapping_sub(53) as i64)
                                            {
                                                x = 0.0f64;
                                                break '__join_7;
                                            } else {
                                                if top_exp <= 1024 {
                                                    // lsb = exponent of least significant bit of the *rounded* value.
                                                    // This is top_exp - DBL_MANT_DIG unless result is subnormal.
                                                    lsb = Py_MAX!(
                                                        top_exp,
                                                        1021i32.wrapping_neg() as i64
                                                    )
                                                    .wrapping_sub(53);
                                                    x = 0.0f64;
                                                    if exp >= lsb {
                                                        // no rounding required
                                                        i = ndigits.wrapping_sub(1);
                                                        while i >= 0 {
                                                            x = 16.0f64 * x
                                                                + (HEX_DIGIT!(i, fdigits, coeff_end)
                                                                    as f64);
                                                            i = i.wrapping_sub(1);
                                                        }
                                                        x = ldexp(x, exp as i32);
                                                        break '__join_7;
                                                    } else {
                                                        // rounding required.  key_digit is the index of the hex digit
                                                        // containing the first bit to be rounded away.
                                                        half_eps = 1i32
                                                            << ((lsb
                                                                .wrapping_sub(exp)
                                                                .wrapping_sub(1)
                                                                % 4)
                                                                as i32);
                                                        key_digit =
                                                            lsb.wrapping_sub(exp).wrapping_sub(1)
                                                                / 4;
                                                        i = ndigits.wrapping_sub(1);
                                                        while i > key_digit {
                                                            x = 16.0f64 * x
                                                                + (HEX_DIGIT!(i, fdigits, coeff_end)
                                                                    as f64);
                                                            i = i.wrapping_sub(1);
                                                        }
                                                        digit = HEX_DIGIT!(
                                                            key_digit, fdigits, coeff_end
                                                        );
                                                        x = 16.0f64 * x
                                                            + ((digit
                                                                & 16i32.wrapping_sub(
                                                                    2i32.wrapping_mul(half_eps),
                                                                ))
                                                                as f64);
                                                        // round-half-even: round up if bit lsb-1 is 1 and at least one of
                                                        // bits lsb, lsb-2, lsb-3, lsb-4, ... is 1.
                                                        if digit & half_eps != 0 {
                                                            round_up = 0;
                                                            if digit
                                                                & 3i32
                                                                    .wrapping_mul(half_eps)
                                                                    .wrapping_sub(1)
                                                                != 0
                                                            {
                                                                __v8 = true;
                                                            } else {
                                                                if half_eps == 8
                                                                    && key_digit.wrapping_add(1)
                                                                        < ndigits
                                                                {
                                                                    __v7 = HEX_DIGIT!(
                                                                        key_digit.wrapping_add(1),
                                                                        fdigits,
                                                                        coeff_end
                                                                    ) & 1
                                                                        != 0;
                                                                } else {
                                                                    __v7 = false;
                                                                }
                                                                __v8 = __v7;
                                                            }
                                                            '__join_15: {
                                                                if __v8 {
                                                                    round_up = 1;
                                                                } else {
                                                                    i = key_digit.wrapping_sub(1);
                                                                    loop {
                                                                        if i >= 0 {
                                                                            if HEX_DIGIT!(
                                                                                i, fdigits,
                                                                                coeff_end
                                                                            ) != 0
                                                                            {
                                                                                break;
                                                                            } else {
                                                                                i = i.wrapping_sub(
                                                                                    1,
                                                                                );
                                                                            }
                                                                        } else {
                                                                            break '__join_15;
                                                                        }
                                                                    }
                                                                    round_up = 1;
                                                                }
                                                            }
                                                            if round_up != 0 {
                                                                x += 2i32.wrapping_mul(half_eps)
                                                                    as f64;
                                                                if top_exp == 1024 {
                                                                    __v2 = x
                                                                        == ldexp(
                                                                            2i32.wrapping_mul(
                                                                                half_eps,
                                                                            )
                                                                                as f64,
                                                                            53i32,
                                                                        );
                                                                } else {
                                                                    __v2 = false;
                                                                }
                                                                if __v2 {
                                                                    break '__join_2;
                                                                }
                                                                // overflow corner case: pre-rounded value <
                                                                // 2**DBL_MAX_EXP; rounded=2**DBL_MAX_EXP.
                                                            }
                                                        }
                                                        x = ldexp(
                                                            x,
                                                            exp.wrapping_add(
                                                                4i64.wrapping_mul(key_digit),
                                                            )
                                                                as i32,
                                                        );
                                                        break '__join_7;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    PyErr_SetString(
                                        PyExc_OverflowError,
                                        c"hexadecimal value too large to represent as a float"
                                            .as_ptr(),
                                    );
                                    return core::ptr::null_mut::<_object>();
                                }
                            }
                        }
                    }
                }
                while Py_ISSPACE!(unsafe { *s }) != 0 {
                    let __v1: *const i8 = s;
                    s = __v1.wrapping_offset(1 as isize);
                }
                // optional trailing whitespace leading to the end of the string
                if s == s_end {
                    result = PyFloat_FromDouble(if negate != 0 { -x } else { x });
                    if r#type != core::ptr::addr_of_mut!(PyFloat_Type) && !result.is_null() {
                        Py_SETREF!(result, PyObject_CallOneArg(r#type as *mut _object, result));
                    }
                    return result;
                }
            }
            PyErr_SetString(
                PyExc_ValueError,
                c"invalid hexadecimal floating-point string".as_ptr(),
            );
            return core::ptr::null_mut::<_object>();
        }
    }
}

// [clinic input]
// float.as_integer_ratio
//
// Return a pair of integers, whose ratio is exactly equal to the original float.
//
// The ratio is in lowest terms and has a positive denominator.  Raise
// OverflowError on infinities and a ValueError on NaNs.
//
// >>> (10.0).as_integer_ratio()
// (10, 1)
// >>> (0.0).as_integer_ratio()
// (0, 1)
// >>> (-.25).as_integer_ratio()
// (-1, 4)
// [clinic start generated code]
/// [clinic end generated code: output=65f25f0d8d30a712 input=d5ba7765655d75bd]
fn float_as_integer_ratio_impl(mut self_: *mut _object) -> *mut _object {
    let mut long_methods: *mut __SlateRecord_object_h_106 = unsafe { core::mem::zeroed() };
    let mut result_pair: *mut _object = unsafe { core::mem::zeroed() };
    let mut denominator: *mut _object = unsafe { core::mem::zeroed() };
    let mut numerator: *mut _object = unsafe { core::mem::zeroed() };
    let mut py_exponent: *mut _object = unsafe { core::mem::zeroed() };
    let mut i: i32 = unsafe { core::mem::zeroed() };
    let mut exponent: i32 = unsafe { core::mem::zeroed() };
    let mut float_part: f64 = unsafe { core::mem::zeroed() };
    let mut self_double: f64 = unsafe { core::mem::zeroed() };
    unsafe {
        py_exponent = core::ptr::null_mut::<_object>();
        numerator = core::ptr::null_mut::<_object>();
        denominator = core::ptr::null_mut::<_object>();
        result_pair = core::ptr::null_mut::<_object>();
        long_methods = PyLong_Type.tp_as_number;
        CONVERT_TO_DOUBLE!(self_, self_double);
        if (if self_double.is_infinite() {
            if self_double.is_sign_negative() {
                -1i32
            } else {
                1i32
            }
        } else {
            0i32
        }) != 0
        {
            PyErr_SetString(
                PyExc_OverflowError,
                c"cannot convert Infinity to integer ratio".as_ptr(),
            );
            return core::ptr::null_mut::<_object>();
        } else {
            if self_double.is_nan() {
                PyErr_SetString(
                    PyExc_ValueError,
                    c"cannot convert NaN to integer ratio".as_ptr(),
                );
                return core::ptr::null_mut::<_object>();
            } else {
                float_part = frexp(self_double, core::ptr::addr_of_mut!(exponent)); // self_double == float_part * 2**exponent exactly
                i = 0;
                while i < 300 && float_part != floor(float_part) {
                    let __v4: f64 = float_part * 2.0f64;
                    float_part = __v4;
                    exponent = exponent.wrapping_sub(1);
                    i = i.wrapping_add(1);
                }
                '__join_0: {
                    // self == float_part * 2**exponent exactly and float_part is integral.
                    // If FLT_RADIX != 2, the 300 steps may leave a tiny fractional part
                    // to be truncated by PyLong_FromDouble().
                    numerator = PyLong_FromDouble(float_part);
                    if !numerator.is_null() {
                        denominator = PyLong_FromLong(1i64);
                        if !denominator.is_null() {
                            py_exponent = PyLong_FromLong(Py_ABS__2!(exponent) as i64);
                            if !py_exponent.is_null() {
                                // fold in 2**exponent
                                if exponent > 0 {
                                    Py_SETREF!(
                                        numerator,
                                        unsafe { (*long_methods).nb_lshift }.unwrap()(
                                            numerator,
                                            py_exponent
                                        )
                                    );
                                    if numerator.is_null() {
                                        break '__join_0;
                                    }
                                } else {
                                    Py_SETREF!(
                                        denominator,
                                        unsafe { (*long_methods).nb_lshift }.unwrap()(
                                            denominator,
                                            py_exponent
                                        )
                                    );
                                    if denominator.is_null() {
                                        break '__join_0;
                                    }
                                }
                                result_pair = PyTuple_Pack(2i64, numerator, denominator);
                            }
                        }
                    }
                }
                Py_XDECREF!(py_exponent);
                Py_XDECREF!(denominator);
                Py_XDECREF!(numerator);
                return result_pair;
            }
        }
    }
}

// [clinic input]
// @classmethod
// float.__new__ as float_new
//     x: object(c_default="NULL") = 0
//     /
//
// Convert a string or number to a floating-point number, if possible.
// [clinic start generated code]
/// [clinic end generated code: output=ccf1e8dc460ba6ba input=55909f888aa0c8a6]
fn float_new_impl(r#type: *mut _typeobject, mut x: *mut _object) -> *mut _object {
    if r#type != unsafe { core::ptr::addr_of_mut!(PyFloat_Type) } {
        if x.is_null() {
            x = _PyLong_GetZero();
        }
        return float_subtype_new(r#type, x); // Wimp out
    }
    if x.is_null() {
        return PyFloat_FromDouble(0.0f64);
    }
    // If it's a string, but not a string subclass, use
    // PyFloat_FromString.
    if PyUnicode_CheckExact!(x) != 0 {
        return PyFloat_FromString(x);
    }
    unsafe { PyNumber_Float(x) }
}

/// Wimpy, slow approach to tp_new calls for subtypes of float:
/// first create a regular float from whatever arguments we got,
/// then allocate a subtype instance and initialize its ob_fval
/// from the regular float.  The regular float is then thrown away.
fn float_subtype_new(r#type: *mut _typeobject, x: *mut _object) -> *mut _object {
    let tmp: *mut _object = float_new_impl(unsafe { core::ptr::addr_of_mut!(PyFloat_Type) }, x);
    if tmp.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let newobj: *mut _object = unsafe { unsafe { (*r#type).tp_alloc }.unwrap()(r#type, 0i64) };
    if newobj.is_null() {
        Py_DECREF!(tmp);
        return core::ptr::null_mut::<_object>();
    }
    unsafe {
        (*(newobj as *mut __SlateRecord_floatobject_h_8)).ob_fval =
            (*(tmp as *mut __SlateRecord_floatobject_h_8)).ob_fval;
    }
    Py_DECREF!(tmp);
    newobj
}

#[unsafe(link_section = ".text.slate_distinct.floatobject.float_vectorcall")]
extern "C-unwind" fn float_vectorcall(
    r#type: *mut _object,
    args: *const *mut _object,
    nargsf: u64,
    kwnames: *mut _object,
) -> *mut _object {
    if !_PyArg_NoKwnames!(c"float".as_ptr() as *mut i8, kwnames) {
        return core::ptr::null_mut::<_object>();
    }
    let nargs: i64 = PyVectorcall_NARGS!(nargsf);
    if !_PyArg_CheckPositional!(c"float".as_ptr() as *mut i8, nargs, 0i32, 1i32) {
        return core::ptr::null_mut::<_object>();
    }
    let x: *mut _object = if nargs >= 1 {
        unsafe { *args.wrapping_offset(0 as isize) }
    } else {
        core::ptr::null_mut::<_object>()
    };
    float_new_impl(_PyType_CAST!(r#type), x)
}

// [clinic input]
// @classmethod
// float.from_number
//
//     number: object
//     /
//
// Convert real number to a floating-point number.
// [clinic start generated code]
/// [clinic end generated code: output=dda7e4466ab7068d input=1f8424d9bc11866a]
fn float_from_number_impl(r#type: *mut _typeobject, number: *mut _object) -> *mut _object {
    if PyFloat_CheckExact!(number) != 0
        && r#type == unsafe { core::ptr::addr_of_mut!(PyFloat_Type) }
    {
        Py_INCREF!(number);
        return number;
    }
    let x: f64 = PyFloat_AsDouble(number);
    let __v0: bool = x == -1.0f64 && !unsafe { PyErr_Occurred() }.is_null();
    if __v0 {
        return core::ptr::null_mut::<_object>();
    }
    let mut result: *mut _object = PyFloat_FromDouble(x);
    if r#type != unsafe { core::ptr::addr_of_mut!(PyFloat_Type) } && !result.is_null() {
        Py_SETREF!(result, unsafe {
            PyObject_CallOneArg(r#type as *mut _object, result)
        });
    }
    result
}

// [clinic input]
// float.__getnewargs__
// [clinic start generated code]
/// [clinic end generated code: output=873258c9d206b088 input=002279d1d77891e6]
fn float___getnewargs___impl(self_: *mut _object) -> *mut _object {
    unsafe {
        Py_BuildValue(
            c"(d)".as_ptr(),
            (*(self_ as *mut __SlateRecord_floatobject_h_8)).ob_fval,
        )
    }
}

// [clinic input]
// @classmethod
// float.__getformat__
//
//     typestr: str
//         Must be 'double' or 'float'.
//     /
//
// You probably don't want to use this function.
//
// It exists mainly to be used in Python's test suite.
//
// This function returns whichever of 'unknown', 'IEEE, big-endian' or
// 'IEEE, little-endian' best describes the format of floating-point
// numbers used by the C type named by typestr.
// [clinic start generated code]
// this is for the benefit of the pack/unpack routines below
/// [clinic end generated code: output=2bfb987228cc9628 input=933cc4bdcf8fa8d3]
fn float___getformat___impl(r#type: *mut _typeobject, typestr: *const i8) -> *mut _object {
    let mut r: u32 = 0;
    if (unsafe {
        strcmp(
            typestr as *const core::ffi::c_char,
            c"double".as_ptr() as *const core::ffi::c_char,
        ) as i32
    }) == 0
    {
        r = unsafe { _PyRuntime.float_state.double_format };
    } else {
        if (unsafe {
            strcmp(
                typestr as *const core::ffi::c_char,
                c"float".as_ptr() as *const core::ffi::c_char,
            ) as i32
        }) == 0
        {
            r = unsafe { _PyRuntime.float_state.float_format };
        } else {
            unsafe {
                PyErr_SetString(
                    PyExc_ValueError,
                    c"__getformat__() argument 1 must be 'double' or 'float'".as_ptr(),
                )
            };
            return core::ptr::null_mut::<_object>();
        }
    }
    match r as u32 {
        0 => {
            return unsafe { PyUnicode_FromString(c"unknown".as_ptr()) };
        }
        2 => {
            return unsafe { PyUnicode_FromString(c"IEEE, little-endian".as_ptr()) };
        }
        1 => {
            return unsafe { PyUnicode_FromString(c"IEEE, big-endian".as_ptr()) };
        }
        _ => {
            unsafe {
                PyErr_SetString(
                    PyExc_RuntimeError,
                    c"insane float_format or double_format".as_ptr(),
                )
            };
            return core::ptr::null_mut::<_object>();
        }
    }
    unsafe { core::mem::zeroed() }
}

#[unsafe(link_section = ".text.slate_distinct.floatobject.float_getreal")]
extern "C-unwind" fn float_getreal(v: *mut _object, _unused_closure: *mut ()) -> *mut _object {
    float_float(v)
}

#[unsafe(link_section = ".text.slate_distinct.floatobject.float_getimag")]
extern "C-unwind" fn float_getimag(
    _unused_v: *mut _object,
    _unused_closure: *mut (),
) -> *mut _object {
    PyFloat_FromDouble(0.0f64)
}

// [clinic input]
// float.__format__
//
//   format_spec: unicode
//   /
//
// Formats the float according to format_spec.
// [clinic start generated code]
/// [clinic end generated code: output=b260e52a47eade56 input=2ece1052211fd0e6]
fn float___format___impl(self_: *mut _object, format_spec: *mut _object) -> *mut _object {
    let mut writer: __SlateRecord_unicodeobject_h_539 = unsafe { core::mem::zeroed() };
    let mut ret: i32 = 0;
    unsafe { _PyUnicodeWriter_Init(core::ptr::addr_of_mut!(writer)) };
    ret = unsafe {
        _PyFloat_FormatAdvancedWriter(
            core::ptr::addr_of_mut!(writer),
            self_,
            format_spec,
            0i64,
            PyUnicode_GET_LENGTH!(format_spec),
        )
    };
    if ret == 1i32.wrapping_neg() {
        unsafe { _PyUnicodeWriter_Dealloc(core::ptr::addr_of_mut!(writer)) };
        return core::ptr::null_mut::<_object>();
    }
    unsafe { _PyUnicodeWriter_Finish(core::ptr::addr_of_mut!(writer)) }
}

/// sentinel
static mut float_methods: __SlateAlign16<[PyMethodDef; 14]> = __SlateAlign16([
    {
        let mut __t0: PyMethodDef = unsafe { core::mem::zeroed() };
        __t0.ml_name = (b"from_number\0".as_ptr() as *mut i8) as *const i8;
        __t0.ml_meth = Some(float_from_number);
        __t0.ml_flags = 24i32;
        __t0.ml_doc = unsafe { core::ptr::addr_of!(float_from_number__doc__.0) as *const i8 };
        __t0
    },
    {
        let mut __t1: PyMethodDef = unsafe { core::mem::zeroed() };
        __t1.ml_name = (b"conjugate\0".as_ptr() as *mut i8) as *const i8;
        __t1.ml_meth = Some(float_conjugate);
        __t1.ml_flags = 4i32;
        __t1.ml_doc = unsafe { core::ptr::addr_of!(float_conjugate__doc__.0) as *const i8 };
        __t1
    },
    {
        let mut __t2: PyMethodDef = unsafe { core::mem::zeroed() };
        __t2.ml_name = (b"__trunc__\0".as_ptr() as *mut i8) as *const i8;
        __t2.ml_meth = Some(float___trunc__);
        __t2.ml_flags = 4i32;
        __t2.ml_doc = unsafe { core::ptr::addr_of!(float___trunc____doc__.0) as *const i8 };
        __t2
    },
    {
        let mut __t3: PyMethodDef = unsafe { core::mem::zeroed() };
        __t3.ml_name = (b"__floor__\0".as_ptr() as *mut i8) as *const i8;
        __t3.ml_meth = Some(float___floor__);
        __t3.ml_flags = 4i32;
        __t3.ml_doc = unsafe { core::ptr::addr_of!(float___floor____doc__.0) as *const i8 };
        __t3
    },
    {
        let mut __t4: PyMethodDef = unsafe { core::mem::zeroed() };
        __t4.ml_name = (b"__ceil__\0".as_ptr() as *mut i8) as *const i8;
        __t4.ml_meth = Some(float___ceil__);
        __t4.ml_flags = 4i32;
        __t4.ml_doc = unsafe { core::ptr::addr_of!(float___ceil____doc__.0) as *const i8 };
        __t4
    },
    {
        let mut __t5: PyMethodDef = unsafe { core::mem::zeroed() };
        __t5.ml_name = (b"__round__\0".as_ptr() as *mut i8) as *const i8;
        __t5.ml_meth = unsafe {
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
                >(Some(float___round__))
            })
        };
        __t5.ml_flags = 128i32;
        __t5.ml_doc = unsafe { core::ptr::addr_of!(float___round____doc__.0) as *const i8 };
        __t5
    },
    {
        let mut __t6: PyMethodDef = unsafe { core::mem::zeroed() };
        __t6.ml_name = (b"as_integer_ratio\0".as_ptr() as *mut i8) as *const i8;
        __t6.ml_meth = Some(float_as_integer_ratio);
        __t6.ml_flags = 4i32;
        __t6.ml_doc = unsafe { core::ptr::addr_of!(float_as_integer_ratio__doc__.0) as *const i8 };
        __t6
    },
    {
        let mut __t7: PyMethodDef = unsafe { core::mem::zeroed() };
        __t7.ml_name = (b"fromhex\0".as_ptr() as *mut i8) as *const i8;
        __t7.ml_meth = Some(float_fromhex);
        __t7.ml_flags = 24i32;
        __t7.ml_doc = unsafe { core::ptr::addr_of!(float_fromhex__doc__.0) as *const i8 };
        __t7
    },
    {
        let mut __t8: PyMethodDef = unsafe { core::mem::zeroed() };
        __t8.ml_name = (b"hex\0".as_ptr() as *mut i8) as *const i8;
        __t8.ml_meth = Some(float_hex);
        __t8.ml_flags = 4i32;
        __t8.ml_doc = unsafe { core::ptr::addr_of!(float_hex__doc__.0) as *const i8 };
        __t8
    },
    {
        let mut __t9: PyMethodDef = unsafe { core::mem::zeroed() };
        __t9.ml_name = (b"is_integer\0".as_ptr() as *mut i8) as *const i8;
        __t9.ml_meth = Some(float_is_integer);
        __t9.ml_flags = 4i32;
        __t9.ml_doc = unsafe { core::ptr::addr_of!(float_is_integer__doc__.0) as *const i8 };
        __t9
    },
    {
        let mut __t10: PyMethodDef = unsafe { core::mem::zeroed() };
        __t10.ml_name = (b"__getnewargs__\0".as_ptr() as *mut i8) as *const i8;
        __t10.ml_meth = Some(float___getnewargs__);
        __t10.ml_flags = 4i32;
        __t10.ml_doc = unsafe { core::ptr::addr_of!(float___getnewargs____doc__.0) as *const i8 };
        __t10
    },
    {
        let mut __t11: PyMethodDef = unsafe { core::mem::zeroed() };
        __t11.ml_name = (b"__getformat__\0".as_ptr() as *mut i8) as *const i8;
        __t11.ml_meth = Some(float___getformat__);
        __t11.ml_flags = 24i32;
        __t11.ml_doc = unsafe { core::ptr::addr_of!(float___getformat____doc__.0) as *const i8 };
        __t11
    },
    {
        let mut __t12: PyMethodDef = unsafe { core::mem::zeroed() };
        __t12.ml_name = (b"__format__\0".as_ptr() as *mut i8) as *const i8;
        __t12.ml_meth = Some(float___format__);
        __t12.ml_flags = 8i32;
        __t12.ml_doc = unsafe { core::ptr::addr_of!(float___format____doc__.0) as *const i8 };
        __t12
    },
    {
        let mut __t13: PyMethodDef = unsafe { core::mem::zeroed() };
        __t13.ml_name = core::ptr::null::<i8>();
        __t13.ml_meth = None;
        __t13
    },
]);

/// Sentinel
static mut float_getset: __SlateAlign16<[PyGetSetDef; 3]> = __SlateAlign16([
    PyGetSetDef {
        name: (b"real\0".as_ptr() as *mut i8) as *const i8,
        get: Some(float_getreal),
        set: None,
        doc: (b"the real part of a complex number\0".as_ptr() as *mut i8) as *const i8,
        closure: core::ptr::null_mut::<()>(),
    },
    PyGetSetDef {
        name: (b"imag\0".as_ptr() as *mut i8) as *const i8,
        get: Some(float_getimag),
        set: None,
        doc: (b"the imaginary part of a complex number\0".as_ptr() as *mut i8) as *const i8,
        closure: core::ptr::null_mut::<()>(),
    },
    {
        let mut __t0: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t0.name = core::ptr::null::<i8>();
        __t0
    },
]);

/// nb_add
/// nb_subtract
/// nb_multiply
/// nb_remainder
/// nb_divmod
/// nb_power
/// nb_negative
/// nb_positive
/// nb_absolute
/// nb_bool
/// nb_invert
/// nb_lshift
/// nb_rshift
/// nb_and
/// nb_xor
/// nb_or
/// nb_int
/// nb_reserved
/// nb_float
/// nb_inplace_add
/// nb_inplace_subtract
/// nb_inplace_multiply
/// nb_inplace_remainder
/// nb_inplace_power
/// nb_inplace_lshift
/// nb_inplace_rshift
/// nb_inplace_and
/// nb_inplace_xor
/// nb_inplace_or
/// nb_floor_divide
/// nb_true_divide
/// nb_inplace_floor_divide
/// nb_inplace_true_divide
static mut float_as_number: __SlateRecord_object_h_106 = {
    let mut __t0: __SlateRecord_object_h_106 = unsafe { core::mem::zeroed() };
    __t0.nb_add = Some(float_add);
    __t0.nb_subtract = Some(float_sub);
    __t0.nb_multiply = Some(float_mul);
    __t0.nb_remainder = Some(float_rem);
    __t0.nb_divmod = Some(float_divmod);
    __t0.nb_power = Some(float_pow);
    __t0.nb_negative = Some(float_neg);
    __t0.nb_positive = Some(float_float);
    __t0.nb_absolute = Some(float_abs);
    __t0.nb_bool = Some(float_bool);
    __t0.nb_invert = None;
    __t0.nb_lshift = None;
    __t0.nb_rshift = None;
    __t0.nb_and = None;
    __t0.nb_xor = None;
    __t0.nb_or = None;
    __t0.nb_int = Some(float___trunc___impl);
    __t0.nb_reserved = core::ptr::null_mut::<()>();
    __t0.nb_float = Some(float_float);
    __t0.nb_inplace_add = None;
    __t0.nb_inplace_subtract = None;
    __t0.nb_inplace_multiply = None;
    __t0.nb_inplace_remainder = None;
    __t0.nb_inplace_power = None;
    __t0.nb_inplace_lshift = None;
    __t0.nb_inplace_rshift = None;
    __t0.nb_inplace_and = None;
    __t0.nb_inplace_xor = None;
    __t0.nb_inplace_or = None;
    __t0.nb_floor_divide = Some(float_floor_div);
    __t0.nb_true_divide = Some(float_div);
    __t0.nb_inplace_floor_divide = None;
    __t0.nb_inplace_true_divide = None;
    __t0
};

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
fn _init_global_state() {
    let mut detected_double_format: u32 = 0;
    let mut detected_float_format: u32 = 0;
    // We attempt to determine if this machine is using IEEE
    // floating-point formats by peering at the bits of some
    // carefully chosen values.  If it looks like we are on an
    // IEEE platform, the float packing/unpacking routines can
    // just copy bits, if not they resort to arithmetic & shifts
    // and masks.  The shifts & masks approach works on all finite
    // values, but what happens to infinities, NaNs and signed
    // zeroes on packing is an accident, and attempting to unpack
    // a NaN or an infinity will raise an exception.
    //
    // Note that if we're on some whacked-out platform which uses
    // IEEE formats but isn't strictly little-endian or big-
    // endian, we will fall back to the portable shifts & masks
    // method.
    let mut x: f64 = 9006104071832581.0f64;
    if (unsafe {
        memcmp(
            (core::ptr::addr_of_mut!(x) as *const ()) as *const core::ffi::c_void,
            (b"C?\xff\x01\x02\x03\x04\x05\0".as_ptr() as *const ()) as *const core::ffi::c_void,
            8u64 as usize,
        ) as i32
    }) == 0
    {
        detected_double_format = 1;
    } else {
        if (unsafe {
            memcmp(
                (core::ptr::addr_of_mut!(x) as *const ()) as *const core::ffi::c_void,
                (b"\x05\x04\x03\x02\x01\xff?C\0".as_ptr() as *const ()) as *const core::ffi::c_void,
                8u64 as usize,
            ) as i32
        }) == 0
        {
            detected_double_format = 2;
        } else {
            detected_double_format = 0;
        }
    }
    let mut y: f32 = 16711938.0f64 as f32;
    if (unsafe {
        memcmp(
            (core::ptr::addr_of_mut!(y) as *const ()) as *const core::ffi::c_void,
            (b"K\x7f\x01\x02\0".as_ptr() as *const ()) as *const core::ffi::c_void,
            4u64 as usize,
        ) as i32
    }) == 0
    {
        detected_float_format = 1;
    } else {
        if (unsafe {
            memcmp(
                (core::ptr::addr_of_mut!(y) as *const ()) as *const core::ffi::c_void,
                (b"\x02\x01\x7fK\0".as_ptr() as *const ()) as *const core::ffi::c_void,
                4u64 as usize,
            ) as i32
        }) == 0
        {
            detected_float_format = 2;
        } else {
            detected_float_format = 0;
        }
    }
    unsafe {
        _PyRuntime.float_state.double_format = detected_double_format;
    }
    unsafe {
        _PyRuntime.float_state.float_format = detected_float_format;
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyFloat_InitState(interp: *mut _is) {
    if _Py_IsMainInterpreter(interp) == 0 {
        return;
    }
    _init_global_state();
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyFloat_InitTypes(interp: *mut _is) -> __SlateRecord_initconfig_h_19 {
    let mut __slate_compound_0: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_1: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    // Init float info
    if _PyStructSequence_InitBuiltin(
        interp,
        unsafe { core::ptr::addr_of_mut!(FloatInfoType) },
        unsafe { core::ptr::addr_of_mut!(floatinfo_desc) },
    ) < 0
    {
        return unsafe {
            *{
                __slate_compound_0 = {
                    let mut __t0: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                    __t0._type = 1u32;
                    __t0.func = _PyStatus_GET_FUNC__61!() as *const i8;
                    __t0.err_msg = c"can't init float info type".as_ptr();
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
extern "C-unwind" fn _PyFloat_FiniType(interp: *mut _is) {
    unsafe { _PyStructSequence_FiniBuiltin(interp, core::ptr::addr_of_mut!(FloatInfoType)) };
}

/// Print summary info about the state of the optimized allocator
#[unsafe(no_mangle)]
extern "C-unwind" fn _PyFloat_DebugMallocStats(out: *mut slate_support::FILE) {
    unsafe {
        _PyDebugAllocatorStats(
            out,
            c"free PyFloatObject".as_ptr(),
            _Py_FREELIST_SIZE!(floats),
            core::mem::size_of::<__SlateRecord_floatobject_h_8>() as u64,
        )
    };
}

// PyFloat_{Pack,Unpack}{2,4,8}.  See floatobject.h.
// To match the NPY_HALF_ROUND_TIES_TO_EVEN behavior in:
// https://github.com/numpy/numpy/blob/master/numpy/core/src/npymath/halffloat.c
// We use:
//       bits = (unsigned short)f;    Note the truncation
//       if ((f - bits > 0.5) || (f - bits == 0.5 && bits % 2)) {
//           bits++;
//       }
#[unsafe(no_mangle)]
extern "C-unwind" fn PyFloat_Pack2(mut x: f64, data: *mut i8, le: i32) -> i32 {
    let mut v: u64 = unsafe { core::mem::zeroed() };
    let mut incr: i32 = unsafe { core::mem::zeroed() };
    let mut bits: u16 = unsafe { core::mem::zeroed() };
    let mut f: f64 = unsafe { core::mem::zeroed() };
    let mut e: i32 = unsafe { core::mem::zeroed() };
    let mut sign: u8 = unsafe { core::mem::zeroed() };
    let mut p: *mut u8 = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_3: {
            p = data as *mut u8;
            incr = 1;
            if x == 0.0f64 {
                sign = (copysign(1.0f64, x) == -1.0f64) as u8;
                e = 0;
                bits = 0;
            } else {
                if (if x.is_infinite() {
                    if x.is_sign_negative() { -1i32 } else { 1i32 }
                } else {
                    0i32
                }) != 0
                {
                    sign = (x < 0.0f64) as u8;
                    e = 31;
                    bits = 0;
                } else {
                    if x.is_nan() {
                        sign = (copysign(1.0f64, x) == -1.0f64) as u8;
                        e = 31;
                        core::ptr::copy_nonoverlapping(
                            ((core::ptr::addr_of_mut!(x) as *const ()) as *const core::ffi::c_void)
                                as *const u8,
                            ((core::ptr::addr_of_mut!(v) as *mut ()) as *mut core::ffi::c_void)
                                as *mut u8,
                            ((core::mem::size_of::<u64>() as u64) as usize) as usize,
                        );
                        v &= 4499201580859392;
                        bits = (v >> 42i32) as u16; // NaN's type & payload
                        // set qNaN if no payload
                        if bits == 0 {
                            bits = ((bits as i32) | 512i32) as u16;
                        }
                    } else {
                        sign = (x < 0.0f64) as u8;
                        if sign != 0 {
                            x = -x;
                        }
                        f = frexp(x, core::ptr::addr_of_mut!(e));
                        if f < 0.5f64 || f >= 1.0f64 {
                            PyErr_SetString(
                                PyExc_SystemError,
                                c"frexp() result out of range".as_ptr(),
                            );
                            return 1i32.wrapping_neg();
                        } else {
                            // Normalize f to be in the range [1.0, 2.0)
                            let __v23: f64 = f * 2.0f64;
                            f = __v23;
                            e = e.wrapping_sub(1);
                            if e < 16 {
                                if e < 25i32.wrapping_neg() {
                                    // |x| < 2**-25. Underflow to zero.
                                    f = 0.0f64;
                                    e = 0;
                                } else {
                                    if e < 14i32.wrapping_neg() {
                                        // |x| < 2**-14. Gradual underflow
                                        f = ldexp(f, 14i32.wrapping_add(e));
                                        e = 0;
                                    } else {
                                        // if (!(e == 0 && f == 0.0))
                                        e = e.wrapping_add(15);
                                        let __v17: f64 = f - 1.0f64;
                                        f = __v17; // Get rid of leading 1
                                    }
                                }
                                let __v15: f64 = f * 1024.0f64;
                                f = __v15; // 2**10
                                // Round to even
                                bits = f as u16; // Note the truncation
                                if f - ((bits as i32) as f64) > 0.5f64
                                    || f - ((bits as i32) as f64) == 0.5f64
                                        && (bits as i32) % 2 == 1
                                {
                                    bits = (bits as i32).wrapping_add(1) as u16;
                                    if (bits as i32) == 1024 {
                                        // The carry propagated out of a string of 10 1 bits.
                                        bits = 0;
                                        e = e.wrapping_add(1);
                                        if e != 31 {
                                            break '__join_3;
                                        }
                                    } else {
                                        break '__join_3;
                                    }
                                } else {
                                    break '__join_3;
                                }
                            }
                            PyErr_SetString(
                                PyExc_OverflowError,
                                c"float too large to pack with e format".as_ptr(),
                            );
                            return 1i32.wrapping_neg();
                        }
                    }
                }
            }
        }
        bits = ((bits as i32) | (e << 10i32 | (sign as i32) << 15i32)) as u16;
        // Write out result.
        if le != 0 {
            let __v3: *mut u8 = p;
            p = __v3.wrapping_offset(1 as isize);
            incr = 1i32.wrapping_neg();
        }
        // First byte
        *p = ((bits as i32) >> 8i32 & 255) as u8;
        let __v1: *mut u8 = p;
        p = __v1.wrapping_offset(incr as isize);
        // Second byte
        *p = ((bits as i32) & 255) as u8;
        return 0;
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyFloat_Pack4(mut x: f64, data: *mut i8, le: i32) -> i32 {
    let mut incr: i32 = unsafe { core::mem::zeroed() };
    let mut fbits: u32 = unsafe { core::mem::zeroed() };
    let mut f: f64 = unsafe { core::mem::zeroed() };
    let mut e: i32 = unsafe { core::mem::zeroed() };
    let mut sign: u8 = unsafe { core::mem::zeroed() };
    let mut s: [u8; 4] = unsafe { core::mem::zeroed() };
    let mut u32: u32 = unsafe { core::mem::zeroed() };
    let mut v: u64 = unsafe { core::mem::zeroed() };
    let mut incr_1: i32 = unsafe { core::mem::zeroed() };
    let mut i: i32 = unsafe { core::mem::zeroed() };
    let mut y: f32 = unsafe { core::mem::zeroed() };
    let mut p: *mut u8 = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_0: {
            p = data as *mut u8;
            if (_PyRuntime.float_state.float_format as u32) == 0 {
                incr = 1;
                if le != 0 {
                    let __v19: *mut u8 = p;
                    p = __v19.wrapping_offset(3 as isize);
                    incr = 1i32.wrapping_neg();
                }
                if x < (0i32 as f64) {
                    sign = 1;
                    x = -x;
                } else {
                    sign = 0;
                }
                f = frexp(x, core::ptr::addr_of_mut!(e));
                // Normalize f to be in the range [1.0, 2.0)
                if 0.5f64 <= f && f < 1.0f64 {
                    let __v16: f64 = f * 2.0f64;
                    f = __v16;
                    e = e.wrapping_sub(1);
                } else {
                    if f == 0.0f64 {
                        e = 0;
                    } else {
                        PyErr_SetString(PyExc_SystemError, c"frexp() result out of range".as_ptr());
                        return 1i32.wrapping_neg();
                    }
                }
                if e < 128 {
                    if e < 126i32.wrapping_neg() {
                        // Gradual underflow
                        f = ldexp(f, 126i32.wrapping_add(e));
                        e = 0;
                    } else {
                        if !(e == 0 && f == 0.0f64) {
                            e = e.wrapping_add(127);
                            let __v10: f64 = f - 1.0f64;
                            f = __v10; // Get rid of leading 1
                        }
                    }
                    let __v8: f64 = f * 8388608.0f64;
                    f = __v8; // 2**23
                    fbits = (f + 0.5f64) as u32; // Round
                    if fbits >> 23i32 != 0 {
                        // The carry propagated out of a string of 23 1 bits.
                        fbits = 0;
                        e = e.wrapping_add(1);
                        if e >= 255 {
                            break '__join_0;
                        }
                    }
                    // First byte
                    *p = ((sign as i32) << 7i32 | e >> 1i32) as u8;
                    let __v5: *mut u8 = p;
                    p = __v5.wrapping_offset(incr as isize);
                    // Second byte
                    *p = (((((e & 1) << 7i32) as u32) | fbits >> 16i32) as i8) as u8;
                    let __v3: *mut u8 = p;
                    p = __v3.wrapping_offset(incr as isize);
                    // Third byte
                    *p = (fbits >> 8i32 & 255) as u8;
                    let __v1: *mut u8 = p;
                    p = __v1.wrapping_offset(incr as isize);
                    // Fourth byte
                    *p = (fbits & 255) as u8;
                    // Done
                    return 0;
                }
            } else {
                y = x as f32;
                incr_1 = 1;
                if !((if y.is_infinite() {
                    if y.is_sign_negative() { -1i32 } else { 1i32 }
                } else {
                    0i32
                }) != 0
                    && (if x.is_infinite() {
                        if x.is_sign_negative() { -1i32 } else { 1i32 }
                    } else {
                        0i32
                    }) == 0)
                {
                    // correct y if x was a sNaN, transformed to qNaN by conversion
                    if x.is_nan() {
                        core::ptr::copy_nonoverlapping(
                            ((core::ptr::addr_of_mut!(x) as *const ()) as *const core::ffi::c_void)
                                as *const u8,
                            ((core::ptr::addr_of_mut!(v) as *mut ()) as *mut core::ffi::c_void)
                                as *mut u8,
                            (8u64 as usize) as usize,
                        );
                        if v & 2251799813685248u64 == 0 {
                            core::ptr::copy_nonoverlapping(
                                ((core::ptr::addr_of_mut!(y) as *const ())
                                    as *const core::ffi::c_void)
                                    as *const u8,
                                ((core::ptr::addr_of_mut!(u32) as *mut ())
                                    as *mut core::ffi::c_void)
                                    as *mut u8,
                                (4u64 as usize) as usize,
                            );
                            // if have payload, make sNaN
                            if u32 & 4194303 != 0 {
                                let __v32: u32 = u32 & 4290772991u32;
                                u32 = __v32;
                            }
                            core::ptr::copy_nonoverlapping(
                                ((core::ptr::addr_of_mut!(u32) as *const ())
                                    as *const core::ffi::c_void)
                                    as *const u8,
                                ((core::ptr::addr_of_mut!(y) as *mut ()) as *mut core::ffi::c_void)
                                    as *mut u8,
                                (4u64 as usize) as usize,
                            );
                        }
                    }
                    core::ptr::copy_nonoverlapping(
                        ((core::ptr::addr_of_mut!(y) as *const ()) as *const core::ffi::c_void)
                            as *const u8,
                        ((s.as_mut_ptr() as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                        ((core::mem::size_of::<f32>() as u64) as usize) as usize,
                    );
                    if (_PyRuntime.float_state.float_format as u32) == 2 && le == 0
                        || (_PyRuntime.float_state.float_format as u32) == 1 && le != 0
                    {
                        let __v30: *mut u8 = p;
                        p = __v30.wrapping_offset(3 as isize);
                        incr_1 = 1i32.wrapping_neg();
                    }
                    i = 0;
                    while i < 4 {
                        *p = s[i as usize];
                        let __v28: *mut u8 = p;
                        p = __v28.wrapping_offset(incr_1 as isize);
                        i = i.wrapping_add(1);
                    }
                    return 0;
                }
            }
        }
        PyErr_SetString(
            PyExc_OverflowError,
            c"float too large to pack with f format".as_ptr(),
        );
        return 1i32.wrapping_neg();
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyFloat_Pack8(mut x: f64, data: *mut i8, le: i32) -> i32 {
    let mut incr: i32 = unsafe { core::mem::zeroed() };
    let mut flo: u32 = unsafe { core::mem::zeroed() };
    let mut fhi: u32 = unsafe { core::mem::zeroed() };
    let mut f: f64 = unsafe { core::mem::zeroed() };
    let mut e: i32 = unsafe { core::mem::zeroed() };
    let mut sign: u8 = unsafe { core::mem::zeroed() };
    let mut incr_1: i32 = unsafe { core::mem::zeroed() };
    let mut i: i32 = unsafe { core::mem::zeroed() };
    let mut s: *const u8 = unsafe { core::mem::zeroed() };
    let mut as_bytes: [u8; 8] = unsafe { core::mem::zeroed() };
    let mut p: *mut u8 = unsafe { core::mem::zeroed() };
    unsafe {
        p = data as *mut u8;
        if (_PyRuntime.float_state.double_format as u32) == 0 {
            incr = 1;
            if le != 0 {
                let __v33: *mut u8 = p;
                p = __v33.wrapping_offset(7 as isize);
                incr = 1i32.wrapping_neg();
            }
            if x < (0i32 as f64) {
                sign = 1;
                x = -x;
            } else {
                sign = 0;
            }
            f = frexp(x, core::ptr::addr_of_mut!(e));
            // Normalize f to be in the range [1.0, 2.0)
            if 0.5f64 <= f && f < 1.0f64 {
                let __v30: f64 = f * 2.0f64;
                f = __v30;
                e = e.wrapping_sub(1);
            } else {
                if f == 0.0f64 {
                    e = 0;
                } else {
                    PyErr_SetString(PyExc_SystemError, c"frexp() result out of range".as_ptr());
                    return 1i32.wrapping_neg();
                }
            }
            '__join_0: {
                if e < 1024 {
                    if e < 1022i32.wrapping_neg() {
                        // Gradual underflow
                        f = ldexp(f, 1022i32.wrapping_add(e));
                        e = 0;
                    } else {
                        if !(e == 0 && f == 0.0f64) {
                            e = e.wrapping_add(1023);
                            let __v24: f64 = f - 1.0f64;
                            f = __v24; // Get rid of leading 1
                        }
                    }
                    // fhi receives the high 28 bits; flo the low 24 bits (== 52 bits)
                    let __v22: f64 = f * 268435456.0f64;
                    f = __v22; // 2**28
                    fhi = f as u32; // Truncate
                    f -= fhi as f64;
                    let __v18: f64 = f * 16777216.0f64;
                    f = __v18; // 2**24
                    flo = (f + 0.5f64) as u32; // Round
                    if flo >> 24i32 != 0 {
                        // The carry propagated out of a string of 24 1 bits.
                        flo = 0;
                        fhi = fhi.wrapping_add(1);
                        if fhi >> 28i32 != 0 {
                            // And it also propagated out of the next 28 bits.
                            fhi = 0;
                            e = e.wrapping_add(1);
                            if e >= 2047 {
                                break '__join_0;
                            }
                        }
                    }
                    // First byte
                    *p = ((sign as i32) << 7i32 | e >> 4i32) as u8;
                    let __v13: *mut u8 = p;
                    p = __v13.wrapping_offset(incr as isize);
                    // Second byte
                    *p = ((((e & 15) << 4i32) as u32) | fhi >> 24i32) as u8;
                    let __v11: *mut u8 = p;
                    p = __v11.wrapping_offset(incr as isize);
                    // Third byte
                    *p = (fhi >> 16i32 & 255) as u8;
                    let __v9: *mut u8 = p;
                    p = __v9.wrapping_offset(incr as isize);
                    // Fourth byte
                    *p = (fhi >> 8i32 & 255) as u8;
                    let __v7: *mut u8 = p;
                    p = __v7.wrapping_offset(incr as isize);
                    // Fifth byte
                    *p = (fhi & 255) as u8;
                    let __v5: *mut u8 = p;
                    p = __v5.wrapping_offset(incr as isize);
                    // Sixth byte
                    *p = (flo >> 16i32 & 255) as u8;
                    let __v3: *mut u8 = p;
                    p = __v3.wrapping_offset(incr as isize);
                    // Seventh byte
                    *p = (flo >> 8i32 & 255) as u8;
                    let __v1: *mut u8 = p;
                    p = __v1.wrapping_offset(incr as isize);
                    // Eighth byte
                    *p = (flo & 255) as u8;
                    // p += incr;
                    // Done
                    return 0;
                }
            }
            PyErr_SetString(
                PyExc_OverflowError,
                c"float too large to pack with d format".as_ptr(),
            );
            return 1i32.wrapping_neg();
        } else {
            core::ptr::copy_nonoverlapping(
                ((core::ptr::addr_of_mut!(x) as *const ()) as *const core::ffi::c_void)
                    as *const u8,
                ((as_bytes.as_mut_ptr() as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                (8u64 as usize) as usize,
            );
            s = as_bytes.as_mut_ptr() as *const u8;
            incr_1 = 1;
            if (_PyRuntime.float_state.double_format as u32) == 2 && le == 0
                || (_PyRuntime.float_state.double_format as u32) == 1 && le != 0
            {
                let __v47: *mut u8 = p;
                p = __v47.wrapping_offset(7 as isize);
                incr_1 = 1i32.wrapping_neg();
            }
            i = 0;
            while i < 8 {
                let __v45: *const u8 = s;
                s = __v45.wrapping_offset(1 as isize);
                *p = *__v45;
                let __v43: *mut u8 = p;
                p = __v43.wrapping_offset(incr_1 as isize);
                i = i.wrapping_add(1);
            }
            return 0;
        }
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyFloat_Unpack2(data: *const i8, le: i32) -> f64 {
    let mut p: *mut u8 = data as *mut u8;
    let mut sign: u8 = 0;
    let mut e: i32 = 0;
    let mut f: u32 = 0;
    let mut x: f64 = 0 as f64;
    let mut incr: i32 = 1;
    if le != 0 {
        let __v0: *mut u8 = p;
        p = __v0.wrapping_offset(1 as isize);
        incr = 1i32.wrapping_neg();
    }
    // First byte
    sign = (((unsafe { *p }) as i32) >> 7i32 & 1) as u8;
    e = (((unsafe { *p }) as i32) & 124) >> 2i32;
    f = ((((unsafe { *p }) as i32) & 3) << 8i32) as u32;
    let __v2: *mut u8 = p;
    p = __v2.wrapping_offset(incr as isize);
    // Second byte
    f |= ((unsafe { *p }) as i32) as u32;
    if e == 31 {
        if f == 0 {
            // Infinity
            return if sign != 0 {
                -(f32::INFINITY as f64)
            } else {
                f32::INFINITY as f64
            };
        } else {
            // NaN
            let mut v: u64 = if sign != 0 {
                18442240474082181120
            } else {
                9218868437227405312
            };
            v = v.wrapping_add((f as u64) << 42i32); // add NaN's type & payload
            unsafe {
                core::ptr::copy_nonoverlapping(
                    ((core::ptr::addr_of_mut!(v) as *const ()) as *const core::ffi::c_void)
                        as *const u8,
                    ((core::ptr::addr_of_mut!(x) as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                    ((core::mem::size_of::<u64>() as u64) as usize) as usize,
                )
            };
            return x;
        }
    }
    x = (f as f64) / 1024.0f64;
    if e == 0 {
        e = 14i32.wrapping_neg();
    } else {
        let __v9: f64 = x + 1.0f64;
        x = __v9;
        e = e.wrapping_sub(15);
    }
    x = unsafe { ldexp(x, e) };
    if sign != 0 {
        x = -x;
    }
    x
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyFloat_Unpack4(data: *const i8, le: i32) -> f64 {
    let mut p: *mut u8 = data as *mut u8;
    if ((unsafe { _PyRuntime.float_state.float_format }) as u32) == 0 {
        let mut sign: u8 = 0;
        let mut e: i32 = 0;
        let mut f: u32 = 0;
        let mut x: f64 = 0 as f64;
        let mut incr: i32 = 1;
        if le != 0 {
            let __v0: *mut u8 = p;
            p = __v0.wrapping_offset(3 as isize);
            incr = 1i32.wrapping_neg();
        }
        // First byte
        sign = (((unsafe { *p }) as i32) >> 7i32 & 1) as u8;
        e = (((unsafe { *p }) as i32) & 127) << 1i32;
        let __v2: *mut u8 = p;
        p = __v2.wrapping_offset(incr as isize);
        // Second byte
        e |= ((unsafe { *p }) as i32) >> 7i32 & 1;
        f = ((((unsafe { *p }) as i32) & 127) << 16i32) as u32;
        let __v6: *mut u8 = p;
        p = __v6.wrapping_offset(incr as isize);
        if e == 255 {
            unsafe {
                PyErr_SetString(
                    PyExc_ValueError,
                    c"can't unpack IEEE 754 special value on non-IEEE platform".as_ptr(),
                )
            };
            return 1i32.wrapping_neg() as f64;
        }
        // Third byte
        f |= (((unsafe { *p }) as i32) << 8i32) as u32;
        let __v10: *mut u8 = p;
        p = __v10.wrapping_offset(incr as isize);
        // Fourth byte
        f |= ((unsafe { *p }) as i32) as u32;
        x = (f as f64) / 8388608.0f64;
        // XXX This sadly ignores Inf/NaN issues
        if e == 0 {
            e = 126i32.wrapping_neg();
        } else {
            let __v15: f64 = x + 1.0f64;
            x = __v15;
            e = e.wrapping_sub(127);
        }
        x = unsafe { ldexp(x, e) };
        if sign != 0 {
            x = -x;
        }
        return x;
    } else {
        let mut x: f32 = 0 as f32;
        if ((unsafe { _PyRuntime.float_state.float_format }) as u32) == 2 && le == 0
            || ((unsafe { _PyRuntime.float_state.float_format }) as u32) == 1 && le != 0
        {
            let mut buf: [i8; 4] = [0i8; 4];
            let mut d: *mut i8 = (buf.as_mut_ptr() as *mut i8).wrapping_offset(3 as isize);
            for i in 0i32..4i32 {
                let __v18: *mut u8 = p;
                p = __v18.wrapping_offset(1 as isize);
                let __v20: *mut i8 = d;
                d = __v20.wrapping_offset(-(1 as isize));
                unsafe {
                    *__v20 = *__v18 as i8;
                }
            }
            unsafe {
                core::ptr::copy_nonoverlapping(
                    ((buf.as_mut_ptr() as *const ()) as *const core::ffi::c_void) as *const u8,
                    ((core::ptr::addr_of_mut!(x) as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                    (4u64 as usize) as usize,
                )
            };
        } else {
            unsafe {
                core::ptr::copy_nonoverlapping(
                    ((p as *const ()) as *const core::ffi::c_void) as *const u8,
                    ((core::ptr::addr_of_mut!(x) as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                    (4u64 as usize) as usize,
                )
            };
        }
        // return sNaN double if x was sNaN float
        if x.is_nan() {
            let mut v: u32 = 0;
            unsafe {
                core::ptr::copy_nonoverlapping(
                    ((core::ptr::addr_of_mut!(x) as *const ()) as *const core::ffi::c_void)
                        as *const u8,
                    ((core::ptr::addr_of_mut!(v) as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                    (4u64 as usize) as usize,
                )
            };
            if v & 4194304u32 == 0 {
                let mut y: f64 = x as f64; // will make qNaN double
                let mut u64: u64 = 0;
                unsafe {
                    core::ptr::copy_nonoverlapping(
                        ((core::ptr::addr_of_mut!(y) as *const ()) as *const core::ffi::c_void)
                            as *const u8,
                        ((core::ptr::addr_of_mut!(u64) as *mut ()) as *mut core::ffi::c_void)
                            as *mut u8,
                        (8u64 as usize) as usize,
                    )
                };
                let __v26: u64 = u64 & 18444492273895866367u64;
                u64 = __v26; // make sNaN
                unsafe {
                    core::ptr::copy_nonoverlapping(
                        ((core::ptr::addr_of_mut!(u64) as *const ()) as *const core::ffi::c_void)
                            as *const u8,
                        ((core::ptr::addr_of_mut!(y) as *mut ()) as *mut core::ffi::c_void)
                            as *mut u8,
                        (8u64 as usize) as usize,
                    )
                };
                return y;
            }
        }
        return x as f64;
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyFloat_Unpack8(data: *const i8, le: i32) -> f64 {
    let mut p: *mut u8 = data as *mut u8;
    if ((unsafe { _PyRuntime.float_state.double_format }) as u32) == 0 {
        let mut sign: u8 = 0;
        let mut e: i32 = 0;
        let mut fhi: u32 = 0;
        let mut flo: u32 = 0;
        let mut x: f64 = 0 as f64;
        let mut incr: i32 = 1;
        if le != 0 {
            let __v0: *mut u8 = p;
            p = __v0.wrapping_offset(7 as isize);
            incr = 1i32.wrapping_neg();
        }
        // First byte
        sign = (((unsafe { *p }) as i32) >> 7i32 & 1) as u8;
        e = (((unsafe { *p }) as i32) & 127) << 4i32;
        let __v2: *mut u8 = p;
        p = __v2.wrapping_offset(incr as isize);
        // Second byte
        e |= ((unsafe { *p }) as i32) >> 4i32 & 15;
        fhi = ((((unsafe { *p }) as i32) & 15) << 24i32) as u32;
        let __v6: *mut u8 = p;
        p = __v6.wrapping_offset(incr as isize);
        if e == 2047 {
            unsafe {
                PyErr_SetString(
                    PyExc_ValueError,
                    c"can't unpack IEEE 754 special value on non-IEEE platform".as_ptr(),
                )
            };
            return -1.0f64;
        }
        // Third byte
        fhi |= (((unsafe { *p }) as i32) << 16i32) as u32;
        let __v10: *mut u8 = p;
        p = __v10.wrapping_offset(incr as isize);
        // Fourth byte
        fhi |= (((unsafe { *p }) as i32) << 8i32) as u32;
        let __v14: *mut u8 = p;
        p = __v14.wrapping_offset(incr as isize);
        // Fifth byte
        fhi |= ((unsafe { *p }) as i32) as u32;
        let __v18: *mut u8 = p;
        p = __v18.wrapping_offset(incr as isize);
        // Sixth byte
        flo = (((unsafe { *p }) as i32) << 16i32) as u32;
        let __v20: *mut u8 = p;
        p = __v20.wrapping_offset(incr as isize);
        // Seventh byte
        flo |= (((unsafe { *p }) as i32) << 8i32) as u32;
        let __v24: *mut u8 = p;
        p = __v24.wrapping_offset(incr as isize);
        // Eighth byte
        flo |= ((unsafe { *p }) as i32) as u32;
        x = (fhi as f64) + (flo as f64) / 16777216.0f64; // 2**24
        let __v29: f64 = x / 268435456.0f64;
        x = __v29; // 2**28
        if e == 0 {
            e = 1022i32.wrapping_neg();
        } else {
            let __v31: f64 = x + 1.0f64;
            x = __v31;
            e = e.wrapping_sub(1023);
        }
        x = unsafe { ldexp(x, e) };
        if sign != 0 {
            x = -x;
        }
        return x;
    } else {
        let mut x: f64 = 0 as f64;
        if ((unsafe { _PyRuntime.float_state.double_format }) as u32) == 2 && le == 0
            || ((unsafe { _PyRuntime.float_state.double_format }) as u32) == 1 && le != 0
        {
            let mut buf: [i8; 8] = [0i8; 8];
            let mut d: *mut i8 = (buf.as_mut_ptr() as *mut i8).wrapping_offset(7 as isize);
            for i in 0i32..8i32 {
                let __v34: *mut u8 = p;
                p = __v34.wrapping_offset(1 as isize);
                let __v36: *mut i8 = d;
                d = __v36.wrapping_offset(-(1 as isize));
                unsafe {
                    *__v36 = *__v34 as i8;
                }
            }
            unsafe {
                core::ptr::copy_nonoverlapping(
                    ((buf.as_mut_ptr() as *const ()) as *const core::ffi::c_void) as *const u8,
                    ((core::ptr::addr_of_mut!(x) as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                    (8u64 as usize) as usize,
                )
            };
        } else {
            unsafe {
                core::ptr::copy_nonoverlapping(
                    ((p as *const ()) as *const core::ffi::c_void) as *const u8,
                    ((core::ptr::addr_of_mut!(x) as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                    (8u64 as usize) as usize,
                )
            };
        }
        return x;
    }
}
