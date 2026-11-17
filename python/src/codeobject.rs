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
//! _PyCodeConstructor
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
//! Note that these all fit within a byte, as do combinations.
//! Values used as the oparg for LOAD_COMMON_CONSTANT
//! Total tool ids available
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
// Initial exit temperature.
// Must be larger than ADAPTIVE_COOLDOWN_VALUE,
// otherwise when a side exit warms up we may construct
// a new trace before the Tier 1 code has properly re-specialized.
// 64KiB  (32KiB on 32-bit)
// 1024
// Used as a special value to encode block sizes in 32 bits.
// 35 == mi_segment_bin(MI_SLICES_PER_SEGMENT)
// initialize a local variable to zero; use memset as compilers optimize constant sized memset's
// Values used in the oparg for RESUME
macro_rules! ASSERT_VALID_BOUNDS {
    () => {};
}

macro_rules! FT_ATOMIC_STORE_PTR_RELEASE {
    ($value:expr, $new_value:expr) => {
        unsafe {
            $value = $new_value;
        }
    };
}

macro_rules! FT_ATOMIC_STORE_PTR {
    ($value:expr, $new_value:expr) => {
        unsafe {
            $value = $new_value;
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

macro_rules! GETITEM {
    ($v:expr, $i:expr) => {
        PyTuple_GET_ITEM!($v, $i)
    };
}

macro_rules! GET_OPARG {
    ($initial:expr) => {
        $initial
    };
}

macro_rules! IS_RETURN_OPCODE {
    ($opcode:expr) => {
        (($opcode) as i32) == 35
    };
}

macro_rules! LOAD_ATTR_NAME_INDEX {
    ($oparg:expr) => {
        ($oparg) >> 1i32
    };
}

macro_rules! PyBool_Check {
    ($x:expr) => {
        Py_IS_TYPE!($x, unsafe { core::ptr::addr_of_mut!(PyBool_Type) })
    };
}

macro_rules! PyBytes_AS_STRING {
    ($op:expr) => {
        PyBytes_AS_STRING(_PyObject_CAST!($op))
    };
}

macro_rules! PyBytes_CheckExact {
    ($op:expr) => {
        Py_IS_TYPE!($op, unsafe { core::ptr::addr_of_mut!(PyBytes_Type) })
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

macro_rules! PyCode_Check {
    ($op:expr) => {
        Py_IS_TYPE!($op, unsafe { core::ptr::addr_of_mut!(PyCode_Type) })
    };
}

macro_rules! PyComplex_CheckExact {
    ($op:expr) => {
        Py_IS_TYPE!($op, unsafe { core::ptr::addr_of_mut!(PyComplex_Type) })
    };
}

/// Mask the old API with a call to the new API for code compiled under
/// Python 2.0:
macro_rules! PyErr_BadInternalCall__19 {
    () => {
        unsafe {
            _PyErr_BadInternalCall(
                c"/home/takashi/c-corpus/cpython/Objects/codeobject.c".as_ptr(),
                789i32,
            )
        };
    };
}

/// Mask the old API with a call to the new API for code compiled under
/// Python 2.0:
macro_rules! PyErr_BadInternalCall__21 {
    () => {
        unsafe {
            _PyErr_BadInternalCall(
                c"/home/takashi/c-corpus/cpython/Objects/codeobject.c".as_ptr(),
                1648i32,
            )
        };
    };
}

/// Mask the old API with a call to the new API for code compiled under
/// Python 2.0:
macro_rules! PyErr_BadInternalCall__66 {
    () => {
        unsafe {
            _PyErr_BadInternalCall(
                c"/home/takashi/c-corpus/cpython/Objects/codeobject.c".as_ptr(),
                1681i32,
            )
        };
    };
}

/// Mask the old API with a call to the new API for code compiled under
/// Python 2.0:
macro_rules! PyErr_BadInternalCall__131 {
    () => {
        unsafe {
            _PyErr_BadInternalCall(
                c"/home/takashi/c-corpus/cpython/Objects/codeobject.c".as_ptr(),
                465i32,
            )
        };
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

macro_rules! PyFrozenSet_CheckExact {
    ($ob:expr) => {
        Py_IS_TYPE!($ob, unsafe { core::ptr::addr_of_mut!(PyFrozenSet_Type) })
    };
}

macro_rules! PyLong_CheckExact {
    ($op:expr) => {
        Py_IS_TYPE!($op, unsafe { core::ptr::addr_of_mut!(PyLong_Type) })
    };
}

macro_rules! PyMem_New__13 {
    ($type:ty, $n:expr) => {{
        let __v: *mut $type =
            if (($n) as u64) > 9223372036854775807 / (core::mem::size_of::<$type>() as u64) {
                core::ptr::null_mut::<__SlateRecord_code_h_16>()
            } else {
                (unsafe {
                    PyMem_Malloc((($n) as u64).wrapping_mul(core::mem::size_of::<$type>() as u64))
                }) as *mut $type
            };
        __v
    }};
}

macro_rules! PyObject_NewVar {
    ($type:ty, $typeobj:expr, $n:expr) => {
        (unsafe { _PyObject_NewVar($typeobj, $n) }) as *mut $type
    };
}

macro_rules! PySet_Check {
    ($ob:expr) => {{
        let __v: bool = Py_IS_TYPE!($ob, unsafe { core::ptr::addr_of_mut!(PySet_Type) }) != 0
            || (unsafe { PyType_IsSubtype(Py_TYPE!($ob), core::ptr::addr_of_mut!(PySet_Type)) })
                != 0;
        __v
    }};
}

macro_rules! PySet_GET_SIZE {
    ($so:expr) => {
        PySet_GET_SIZE(_PyObject_CAST!($so))
    };
}

macro_rules! PySlice_Check {
    ($op:expr) => {
        Py_IS_TYPE!($op, unsafe { core::ptr::addr_of_mut!(PySlice_Type) })
    };
}

/// Alias to PyThreadState_Get()
macro_rules! PyThreadState_GET {
    () => {
        unsafe { PyThreadState_Get() }
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

macro_rules! PyUnicode_1BYTE_DATA {
    ($op:expr) => {
        _Py_STATIC_CAST!(*mut u8, PyUnicode_DATA!($op))
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

/// Argument must be a char or an int in [-128, 127] or [0, 255].
macro_rules! Py_CHARMASK {
    ($c:expr) => {
        ((($c) as i32) & 255) as u8
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

macro_rules! Py_ISALNUM {
    ($c:expr) => {
        (unsafe { _Py_ctype_table[(Py_CHARMASK!($c) as i32) as usize] }) & 7u32
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

macro_rules! Py_SET_REFCNT {
    ($ob:expr, $refcnt:expr) => {
        Py_SET_REFCNT(_PyObject_CAST!($ob), ($refcnt) as i64);
    };
}

macro_rules! Py_SET_REFCNT__2 {
    ($ob:expr, $refcnt:expr) => {
        Py_SET_REFCNT(_PyObject_CAST!($ob), $refcnt);
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

macro_rules! Py_UNREACHABLE {
    () => {
        unsafe { core::hint::unreachable_unchecked() };
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

macro_rules! SCRAMBLE_IN_HASH {
    ($EXPR:expr, $uhash:expr) => {{
        loop {
            let h: i64 = unsafe { PyObject_Hash($EXPR) };
            if h == (1i32.wrapping_neg() as i64) {
                return 1i32.wrapping_neg() as i64;
            }
            SCRAMBLE_IN!(h, $uhash);
            if true {
                break;
            }
        }
    }};
}

macro_rules! SCRAMBLE_IN {
    ($H:expr, $uhash:expr) => {
        loop {
            let __v: u64 = $uhash;
            let __v2: u64 = __v ^ (($H) as u64);
            $uhash = __v2;
            let __v3: u64 = $uhash;
            let __v4: u64 = __v3.wrapping_mul(1000003);
            $uhash = __v4;
            if true {
                break;
            }
        }
    };
}

macro_rules! SCRAMBLE_IN__2 {
    ($H:expr, $uhash:expr) => {
        loop {
            let __v: u64 = ($H) as u64;
            let __v2: u64 = $uhash;
            let __v3: u64 = __v2 ^ __v;
            $uhash = __v3;
            let __v4: u64 = $uhash;
            let __v5: u64 = __v4.wrapping_mul(1000003);
            $uhash = __v5;
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

macro_rules! _PyCodeObject_CAST {
    ($op:expr) => {
        ($op) as *mut PyCodeObject
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

macro_rules! _PyCode_NBYTES {
    ($CO:expr) => {
        Py_SIZE!($CO).wrapping_mul(core::mem::size_of::<__SlateRecord_pycore_structs_h_32>() as i64)
    };
}

macro_rules! _PyCompactUnicodeObject_CAST {
    ($op:expr) => {
        _Py_CAST!(*mut __SlateRecord_unicodeobject_h_173, $op)
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

macro_rules! _PySet_CAST {
    ($so:expr) => {
        _Py_CAST!(*mut __SlateRecord_setobject_h_59, $so)
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

macro_rules! _Py_INCREF_IMMORTAL_STAT_INC {
    () => {};
}

macro_rules! _Py_INCREF_STAT_INC {
    () => {};
}

/// Check if pointer "p" is aligned to "a"-bytes boundary.
macro_rules! _Py_IS_ALIGNED__2 {
    ($p:expr, $a:expr) => {
        (($p) as u64) & $a.wrapping_sub(1) == 0
    };
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

/// Round up size "n" to be a multiple of "a".
macro_rules! _Py_SIZE_ROUND_UP {
    ($n:expr, $a:expr) => {
        (($n) as u64).wrapping_add($a.wrapping_sub(1) as u64) & !($a.wrapping_sub(1) as u64)
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
    static mut PyType_Type: _typeobject;
    static mut _Py_NoneStruct: _object;
    static mut _Py_NotImplementedStruct: _object;
    static mut PyBytes_Type: _typeobject;
    static mut PyUnicode_Type: _typeobject;
    static mut PyExc_OverflowError: *mut _object;
    static mut PyExc_RuntimeError: *mut _object;
    static mut PyExc_SystemError: *mut _object;
    static mut PyExc_TypeError: *mut _object;
    static mut PyExc_ValueError: *mut _object;
    static mut PyExc_DeprecationWarning: *mut _object;
    static mut _Py_FalseStruct: _longobject;
    static mut _Py_TrueStruct: _longobject;
    static mut PyFloat_Type: _typeobject;
    static mut PyComplex_Type: _typeobject;
    static mut PyTuple_Type: _typeobject;
    static mut PySet_Type: _typeobject;
    static mut PyFrozenSet_Type: _typeobject;
    static mut _Py_EllipsisObject: _object;
    static mut PySlice_Type: _typeobject;
    static mut _Py_ctype_table: [u32; 256];
    #[thread_local]
    static mut _Py_tss_tstate: *mut _ts;
    static mut _PyRuntime: pyruntimestate;
    static mut _PyOpcode_Caches: [u8; 256];
    fn copysign(arg0: f64, arg1: f64) -> f64;
    fn index(arg0: *const i8, arg1: i32) -> *mut i8;
    fn PyMem_Malloc(arg0: u64) -> *mut ();
    fn PyMem_Realloc(arg0: *mut (), arg1: u64) -> *mut ();
    fn PyMem_Free(arg0: *mut ());
    fn PyType_IsSubtype(arg0: *mut _typeobject, arg1: *mut _typeobject) -> i32;
    fn PyType_GenericAlloc(arg0: *mut _typeobject, arg1: i64) -> *mut _object;
    fn PyObject_RichCompareBool(arg0: *mut _object, arg1: *mut _object, arg2: i32) -> i32;
    fn PyObject_SelfIter(arg0: *mut _object) -> *mut _object;
    fn PyObject_GenericGetAttr(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn PyObject_Hash(arg0: *mut _object) -> i64;
    fn PyObject_ClearWeakRefs(arg0: *mut _object);
    fn _Py_Dealloc(arg0: *mut _object);
    fn PyObject_Free(arg0: *mut ());
    fn _PyObject_NewVar(arg0: *mut _typeobject, arg1: i64) -> *mut __SlateRecord_object_h_172;
    fn PyBytes_FromStringAndSize(arg0: *const i8, arg1: i64) -> *mut _object;
    fn _PyBytes_Resize(arg0: *mut *mut _object, arg1: i64) -> i32;
    fn PyUnicode_FromString(arg0: *const i8) -> *mut _object;
    fn PyUnicode_FromFormat(arg0: *const i8, ...) -> *mut _object;
    fn PyUnicode_DecodeFSDefault(arg0: *const i8) -> *mut _object;
    fn PyUnicode_Compare(arg0: *mut _object, arg1: *mut _object) -> i32;
    fn PyErr_SetString(arg0: *mut _object, arg1: *const i8);
    fn PyErr_Occurred() -> *mut _object;
    fn PyErr_NoMemory() -> *mut _object;
    fn PyErr_Format(arg0: *mut _object, arg1: *const i8, ...) -> *mut _object;
    fn _PyErr_BadInternalCall(arg0: *const i8, arg1: i32);
    fn PyErr_FormatUnraisable(arg0: *const i8, ...);
    fn PyLong_FromLong(arg0: i64) -> *mut _object;
    fn PyLong_FromSize_t(arg0: u64) -> *mut _object;
    fn PyLong_AsInt(arg0: *mut _object) -> i32;
    fn PyLong_FromVoidPtr(arg0: *mut ()) -> *mut _object;
    fn PyComplex_AsCComplex(arg0: *mut _object) -> __SlateRecord_complexobject_h_8;
    fn PyTuple_New(arg0: i64) -> *mut _object;
    fn PyTuple_Size(arg0: *mut _object) -> i64;
    fn PyTuple_GetItem(arg0: *mut _object, arg1: i64) -> *mut _object;
    fn PyTuple_Pack(arg0: i64, ...) -> *mut _object;
    fn _PyTuple_Resize(arg0: *mut *mut _object, arg1: i64) -> i32;
    fn PyDict_Contains(arg0: *mut _object, arg1: *mut _object) -> i32;
    fn PySet_New(arg0: *mut _object) -> *mut _object;
    fn PyFrozenSet_New(arg0: *mut _object) -> *mut _object;
    fn PySet_Add(arg0: *mut _object, arg1: *mut _object) -> i32;
    fn PySet_Contains(arg0: *mut _object, arg1: *mut _object) -> i32;
    fn PySlice_New(arg0: *mut _object, arg1: *mut _object, arg2: *mut _object) -> *mut _object;
    fn PyThreadState_Get() -> *mut _ts;
    fn PyErr_WarnEx(arg0: *mut _object, arg1: *const i8, arg2: i64) -> i32;
    fn Py_BuildValue(arg0: *const i8, ...) -> *mut _object;
    fn PySys_Audit(arg0: *const i8, arg1: *const i8, ...) -> i32;
    fn PySequence_Tuple(arg0: *mut _object) -> *mut _object;
    fn _Py_GetBaseCodeUnit(arg0: *mut PyCodeObject, arg1: i32)
    -> __SlateRecord_pycore_structs_h_32;
    fn _PyInstruction_GetLength(arg0: *mut PyCodeObject, arg1: i32) -> i32;
    fn _PyInstrumentation_BranchesIterator(arg0: *mut PyCodeObject) -> *mut _object;
    fn _PyFunction_ClearCodeByVersion(arg0: u32);
    fn _PyErr_SetString(arg0: *mut _ts, arg1: *mut _object, arg2: *const i8);
    fn _PyErr_Format(arg0: *mut _ts, arg1: *mut _object, arg2: *const i8, ...) -> *mut _object;
    fn _Py_Instrumentation_GetLine(
        arg0: *mut PyCodeObject,
        arg1: *mut __SlateRecord_pycore_instruments_h_94,
        arg2: i32,
    ) -> i32;
    fn _Py_GetConfig() -> *const PyConfig;
    fn _PySet_NextEntry(
        arg0: *mut _object,
        arg1: *mut i64,
        arg2: *mut *mut _object,
        arg3: *mut i64,
    ) -> i32;
    fn _PyUnicode_Copy(arg0: *mut _object) -> *mut _object;
    fn _PyUnicode_InternMortal(arg0: *mut _is, arg1: *mut *mut _object);
    fn _PyUnicode_InternImmortal(arg0: *mut _is, arg1: *mut *mut _object);
    fn _PyArg_NoKeywords(arg0: *const i8, arg1: *mut _object) -> i32;
    fn _PyArg_CheckPositional(arg0: *const i8, arg1: i64, arg2: i64, arg3: i64) -> i32;
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
    fn __builtin_unreachable();
    fn _PyCode_Quicken(arg0: *mut __SlateRecord_pycore_structs_h_32, arg1: i64, arg2: i32);
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

fn _Py_atomic_add_uint64(obj: *mut u64, value: u64) -> u64 {
    unsafe {
        let __t1 = value;
        let __t0 = core::sync::atomic::AtomicU64::from_ptr(obj as *mut u64)
            .fetch_add(__t1, core::sync::atomic::Ordering::SeqCst);
        __t0
    }
}

fn _Py_atomic_load_ptr_acquire(obj: *const ()) -> *mut () {
    unsafe {
        core::sync::atomic::AtomicPtr::from_ptr(obj as *mut *mut ())
            .load(core::sync::atomic::Ordering::Acquire) as *mut ()
    }
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

#[repr(C)]
#[derive(Clone, Copy)]
struct _frame {}

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

fn _Py_REFCNT(ob: *mut _object) -> i64 {
    (unsafe { (*ob).__slate_anon_0.__slate_anon_1.ob_refcnt }) as i64
}

#[inline(always)]
fn _Py_IsImmortal(op: *mut _object) -> i32 {
    (_Py_CAST!(i32, unsafe {
        (*op).__slate_anon_0.__slate_anon_1.ob_refcnt
    }) < 0) as i32
}

fn Py_SET_REFCNT(ob: *mut _object, refcnt: i64) {
    // This immortal check is for code that is unaware of immortal objects.
    //
    // The runtime tracks these objects and we should avoid as much
    //
    // as possible having extensions inadvertently change the refcnt
    //
    // of an immortalized object.
    if _Py_IsImmortal!(ob) != 0 {
        return;
    }
    unsafe {
        (*ob).__slate_anon_0.__slate_anon_1.ob_refcnt = refcnt as u32;
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

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_unicodeobject_h_163_2 {
    __slate_bits_0: __slate_bits::__SlateBits137U0,
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
union __SlateRecord_unicodeobject_h_184_2 {
    any: *mut (),
    latin1: *mut u8,
    ucs2: *mut u16,
    ucs4: *mut u32,
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

fn _PyObject_VAR_SIZE(r#type: *mut _typeobject, nitems: i64) -> u64 {
    let mut size: u64 = _Py_STATIC_CAST!(u64, unsafe { (*r#type).tp_basicsize });
    let __v0: u64 = size;
    size = __v0.wrapping_add(
        _Py_STATIC_CAST!(u64, nitems)
            .wrapping_mul(_Py_STATIC_CAST!(u64, unsafe { (*r#type).tp_itemsize })),
    );
    _Py_SIZE_ROUND_UP!(size, 8i32)
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
struct __SlateRecord_floatobject_h_8 {
    ob_base: _object,
    ob_fval: f64,
}

fn PyFloat_AS_DOUBLE(op: *mut _object) -> f64 {
    unsafe { (*_PyFloat_CAST!(op)).ob_fval }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_complexobject_h_8 {
    real: f64,
    imag: f64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pystate_h_78 {
    __slate_bits_0: __slate_bits::__SlateBits202U0,
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
struct __SlateRecord_setobject_h_23 {
    key: *mut _object,
    hash: i64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_setobject_h_59 {
    ob_base: _object,
    fill: i64,
    used: i64,
    mask: i64,
    table: *mut __SlateRecord_setobject_h_23,
    hash: i64,
    finger: i64,
    smalltable: [__SlateRecord_setobject_h_23; 8],
    weakreflist: *mut _object,
}

fn PySet_GET_SIZE(so: *mut _object) -> i64 {
    unsafe { (*_PySet_CAST!(so)).used }
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
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_structs_h_32_2 {
    code: u8,
    arg: u8,
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

// Masks for co_flags above
// The CO_COROUTINE flag is set for coroutine functions (defined with
// ``async def`` keywords)
#[unsafe(no_mangle)]
static mut PyCode_Type: _typeobject = {
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
    __t0.tp_name = (b"code\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::offset_of!(PyCodeObject, co_code_adaptive) as u64) as i64;
    __t0.tp_itemsize = (core::mem::size_of::<__SlateRecord_pycore_structs_h_32>() as u64) as i64;
    __t0.tp_dealloc = Some(code_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = Some(code_repr);
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = Some(code_hash);
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
    __t0.tp_flags = 0u64;
    __t0.tp_doc = unsafe { core::ptr::addr_of!(code_new__doc__.0) as *const i8 };
    __t0.tp_traverse = None;
    __t0.tp_clear = None;
    __t0.tp_richcompare = Some(code_richcompare);
    __t0.tp_weaklistoffset = (core::mem::offset_of!(PyCodeObject, co_weakreflist) as u64) as i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = unsafe { core::ptr::addr_of_mut!(code_methods.0) as *mut PyMethodDef };
    __t0.tp_members = unsafe { core::ptr::addr_of_mut!(code_memberlist.0) as *mut PyMemberDef };
    __t0.tp_getset = unsafe { core::ptr::addr_of_mut!(code_getsetlist.0) as *mut PyGetSetDef };
    __t0.tp_base = core::ptr::null_mut::<_typeobject>();
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = 0i64;
    __t0.tp_init = None;
    __t0.tp_alloc = None;
    __t0.tp_new = Some(code_new);
    __t0
};

fn PyCode_NewWithPosOnlyArgs(
    a: i32,
    poac: i32,
    b: i32,
    c: i32,
    d: i32,
    e: i32,
    f: *mut _object,
    g: *mut _object,
    h: *mut _object,
    i: *mut _object,
    j: *mut _object,
    k: *mut _object,
    l: *mut _object,
    m: *mut _object,
    n: *mut _object,
    o: i32,
    p: *mut _object,
    q: *mut _object,
) -> *mut PyCodeObject {
    PyUnstable_Code_NewWithPosOnlyArgs(a, poac, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q)
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
struct __SlateRecord_sliceobject_h_29 {
    ob_base: _object,
    start: *mut _object,
    stop: *mut _object,
    step: *mut _object,
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
struct co_locals_counts {
    total: i32,
    args: __SlateRecord_pycore_code_h_610_2,
    numpure: i32,
    cells: __SlateRecord_pycore_code_h_610_3,
    hidden: __SlateRecord_pycore_code_h_610_4,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_code_h_610_2 {
    total: i32,
    numposonly: i32,
    numposorkw: i32,
    numkwonly: i32,
    varargs: i32,
    varkwargs: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_code_h_610_3 {
    total: i32,
    numargs: i32,
    numothers: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_code_h_610_4 {
    total: i32,
    numpure: i32,
    numcells: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct co_unbound_counts {
    total: i32,
    globals: __SlateRecord_pycore_code_h_610_5,
    numattrs: i32,
    numunknown: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_code_h_610_5 {
    total: i32,
    numglobal: i32,
    numbuiltin: i32,
    numunknown: i32,
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
struct code_arena_st {}

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
struct __SlateRecord_pycore_pythread_h_64 {
    ptr: *mut __SlateRecord_pthreadtypes_h_45,
    val: __SlateRecord_pthreadtypes_h_45,
}

// pos-only, pos-or-kw, varargs
// kw-only, pos-or-kw, varkwargs
// varargs, varkwargs
fn _PyLocals_GetKind(kinds: *mut _object, i: i32) -> u8 {
    let ptr: *mut i8 = PyBytes_AS_STRING!(kinds);
    (unsafe { *ptr.wrapping_offset(i as isize) }) as u8
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_interp_structs_h_297 {
    mutex: PyMutex,
    hashtable: *mut _Py_hashtable_t,
}

fn _PyLocals_SetKind(kinds: *mut _object, i: i32, kind: u8) {
    let ptr: *mut i8 = PyBytes_AS_STRING!(kinds);
    unsafe {
        *ptr.wrapping_offset(i as isize) = kind as i8;
    }
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
struct _PyCodeConstructor {
    filename: *mut _object,
    name: *mut _object,
    qualname: *mut _object,
    flags: i32,
    code: *mut _object,
    firstlineno: i32,
    linetable: *mut _object,
    consts: *mut _object,
    names: *mut _object,
    localsplusnames: *mut _object,
    localspluskinds: *mut _object,
    argcount: i32,
    posonlyargcount: i32,
    kwonlyargcount: i32,
    stacksize: i32,
    exceptiontable: *mut _object,
}

fn write_varint(mut ptr: *mut u8, mut val: u32) -> i32 {
    let mut written: i32 = 1;
    while val >= 64 {
        let __v0: *mut u8 = ptr;
        ptr = __v0.wrapping_offset(1 as isize);
        unsafe {
            *__v0 = (64 | val & 63) as u8;
        }
        val >>= 6i32;
        written = written.wrapping_add(1);
    }
    unsafe {
        *ptr = val as u8;
    }
    written
}

fn write_signed_varint(ptr: *mut u8, val: i32) -> i32 {
    // (unsigned int)(-val) has an undefined behavior for INT_MIN
    let uval: u32 = if val < 0 {
        0u32.wrapping_sub(val as u32) << 1i32 | 1
    } else {
        (val as u32) << 1i32
    };
    write_varint(ptr, uval)
}

fn write_location_entry_start(ptr: *mut u8, code: i32, length: i32) -> i32 {
    unsafe {
        *ptr =
            (128 | (((code << 3i32) as u8) as i32) | ((length.wrapping_sub(1) as u8) as i32)) as u8;
    }
    1
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
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_code_h_610 {
    total: i32,
    locals: co_locals_counts,
    numfree: i32,
    unbound: co_unbound_counts,
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
struct xi_exceptions {
    PyExc_InterpreterError: *mut _object,
    PyExc_InterpreterNotFoundError: *mut _object,
    PyExc_NotShareableError: *mut _object,
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
struct __SlateRecord_pycore_interp_structs_h_718 {
    _not_used: i32,
    _hamt_empty_gc_not_used: __SlateRecord_pycore_interp_structs_h_170,
    hamt_empty: __SlateRecord_pycore_structs_h_47,
    last_resort_memory_error: __SlateRecord_pyerrors_h_15,
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
union __SlateRecord_pycore_obmalloc_h_255 {
    _padding: *mut u8,
    count: u32,
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
struct __SlateRecord_pycore_runtime_structs_h_49 {
    fd: i32,
    st_dev: u64,
    st_ino: u64,
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
struct __SlateRecord_pycore_tracemalloc_h_70 {
    mem: __SlateRecord_pymem_h_48,
    raw: __SlateRecord_pymem_h_48,
    obj: __SlateRecord_pymem_h_48,
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
struct __SlateRecord_pycore_ucnhash_h_29 {
    getname: Option<unsafe extern "C-unwind" fn(u32, *mut i8, i32, i32) -> i32>,
    getcode: Option<unsafe extern "C-unwind" fn(*const i8, i32, *mut u32, i32) -> i32>,
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
fn _PyThreadState_GET() -> *mut _ts {
    unsafe { _Py_tss_tstate }
}

fn _PyInterpreterState_GET() -> *mut _is {
    let tstate: *mut _ts = _PyThreadState_GET();
    unsafe { (*tstate).interp }
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
struct arena_map_mid {
    ptrs: [*mut arena_map_bot; 32768],
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
struct __SlateRecord_pycore_runtime_structs_h_19 {
    api_id: i8,
    alloc: __SlateRecord_pymem_h_48,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_codeobject_c_h_188 {
    _this_is_not_used: __SlateRecord_pycore_interp_structs_h_170,
    ob_base: __SlateRecord_object_h_172,
    ob_hash: i64,
    ob_item: [*mut _object; 18],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_codeobject_c_h_438 {
    _this_is_not_used: __SlateRecord_pycore_interp_structs_h_170,
    ob_base: __SlateRecord_object_h_172,
    ob_hash: i64,
    ob_item: [*mut _object; 1],
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
    pub struct __SlateBits1321U0 {
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
    pub struct __SlateBits1319U0 {
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
    pub struct __SlateBits202U0 {
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

use slate_support::memcpy;
use slate_support::read;

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

fn _PyObject_ResurrectStart(op: *mut _object) {
    Py_SET_REFCNT!(op, 1i32);
}

fn _PyObject_ResurrectEnd(op: *mut _object) -> i32 {
    Py_SET_REFCNT__2!(op, Py_REFCNT!(op).wrapping_sub(1));
    if Py_REFCNT!(op) == 0 {
        return 0;
    }
    1
}

// Fast inlined version of PyType_IS_GC()
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
    __slate_bits_0: __slate_bits::__SlateBits1319U0,
    warm: bool,
    index: i32,
    bloom: __SlateRecord_pycore_optimizer_h_28,
    links: _PyExecutorLinkListNode,
    code: *mut PyCodeObject,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_optimizer_h_67 {
    __slate_bits_0: __slate_bits::__SlateBits1321U0,
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
static code_new__doc__: __SlateAlign16<[i8; 280]> = __SlateAlign16([
    99i8, 111i8, 100i8, 101i8, 40i8, 97i8, 114i8, 103i8, 99i8, 111i8, 117i8, 110i8, 116i8, 44i8,
    32i8, 112i8, 111i8, 115i8, 111i8, 110i8, 108i8, 121i8, 97i8, 114i8, 103i8, 99i8, 111i8, 117i8,
    110i8, 116i8, 44i8, 32i8, 107i8, 119i8, 111i8, 110i8, 108i8, 121i8, 97i8, 114i8, 103i8, 99i8,
    111i8, 117i8, 110i8, 116i8, 44i8, 32i8, 110i8, 108i8, 111i8, 99i8, 97i8, 108i8, 115i8, 44i8,
    32i8, 115i8, 116i8, 97i8, 99i8, 107i8, 115i8, 105i8, 122i8, 101i8, 44i8, 10i8, 32i8, 32i8,
    32i8, 32i8, 32i8, 102i8, 108i8, 97i8, 103i8, 115i8, 44i8, 32i8, 99i8, 111i8, 100i8, 101i8,
    115i8, 116i8, 114i8, 105i8, 110i8, 103i8, 44i8, 32i8, 99i8, 111i8, 110i8, 115i8, 116i8, 97i8,
    110i8, 116i8, 115i8, 44i8, 32i8, 110i8, 97i8, 109i8, 101i8, 115i8, 44i8, 32i8, 118i8, 97i8,
    114i8, 110i8, 97i8, 109i8, 101i8, 115i8, 44i8, 32i8, 102i8, 105i8, 108i8, 101i8, 110i8, 97i8,
    109i8, 101i8, 44i8, 32i8, 110i8, 97i8, 109i8, 101i8, 44i8, 10i8, 32i8, 32i8, 32i8, 32i8, 32i8,
    113i8, 117i8, 97i8, 108i8, 110i8, 97i8, 109i8, 101i8, 44i8, 32i8, 102i8, 105i8, 114i8, 115i8,
    116i8, 108i8, 105i8, 110i8, 101i8, 110i8, 111i8, 44i8, 32i8, 108i8, 105i8, 110i8, 101i8, 116i8,
    97i8, 98i8, 108i8, 101i8, 44i8, 32i8, 101i8, 120i8, 99i8, 101i8, 112i8, 116i8, 105i8, 111i8,
    110i8, 116i8, 97i8, 98i8, 108i8, 101i8, 44i8, 32i8, 102i8, 114i8, 101i8, 101i8, 118i8, 97i8,
    114i8, 115i8, 61i8, 40i8, 41i8, 44i8, 10i8, 32i8, 32i8, 32i8, 32i8, 32i8, 99i8, 101i8, 108i8,
    108i8, 118i8, 97i8, 114i8, 115i8, 61i8, 40i8, 41i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8,
    10i8, 10i8, 67i8, 114i8, 101i8, 97i8, 116i8, 101i8, 32i8, 97i8, 32i8, 99i8, 111i8, 100i8,
    101i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 46i8, 32i8, 32i8, 78i8, 111i8, 116i8,
    32i8, 102i8, 111i8, 114i8, 32i8, 116i8, 104i8, 101i8, 32i8, 102i8, 97i8, 105i8, 110i8, 116i8,
    32i8, 111i8, 102i8, 32i8, 104i8, 101i8, 97i8, 114i8, 116i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.codeobject.code_new")]
extern "C-unwind" fn code_new(
    r#type: *mut _typeobject,
    args: *mut _object,
    kwargs: *mut _object,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut __v1: bool = unsafe { core::mem::zeroed() };
    let mut __v2: bool = unsafe { core::mem::zeroed() };
    let mut __v3: bool = unsafe { core::mem::zeroed() };
    let mut __v4: bool = unsafe { core::mem::zeroed() };
    let mut __v5: bool = unsafe { core::mem::zeroed() };
    let mut __v6: bool = unsafe { core::mem::zeroed() };
    let mut __v7: bool = unsafe { core::mem::zeroed() };
    let mut cellvars: *mut _object = unsafe { core::mem::zeroed() };
    let mut freevars: *mut _object = unsafe { core::mem::zeroed() };
    let mut exceptiontable: *mut _object = unsafe { core::mem::zeroed() };
    let mut linetable__0: *mut _object = unsafe { core::mem::zeroed() };
    let mut firstlineno: i32 = unsafe { core::mem::zeroed() };
    let mut qualname: *mut _object = unsafe { core::mem::zeroed() };
    let mut name: *mut _object = unsafe { core::mem::zeroed() };
    let mut filename: *mut _object = unsafe { core::mem::zeroed() };
    let mut varnames: *mut _object = unsafe { core::mem::zeroed() };
    let mut names: *mut _object = unsafe { core::mem::zeroed() };
    let mut consts: *mut _object = unsafe { core::mem::zeroed() };
    let mut code: *mut _object = unsafe { core::mem::zeroed() };
    let mut flags: i32 = unsafe { core::mem::zeroed() };
    let mut stacksize: i32 = unsafe { core::mem::zeroed() };
    let mut nlocals: i32 = unsafe { core::mem::zeroed() };
    let mut kwonlyargcount: i32 = unsafe { core::mem::zeroed() };
    let mut posonlyargcount: i32 = unsafe { core::mem::zeroed() };
    let mut argcount: i32 = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        let base_tp: *mut _typeobject = core::ptr::addr_of_mut!(PyCode_Type);
        freevars = core::ptr::null_mut::<_object>();
        cellvars = core::ptr::null_mut::<_object>();
        if r#type == base_tp || (*r#type).tp_init == (*base_tp).tp_init {
            __v7 = !_PyArg_NoKeywords!(c"code".as_ptr() as *mut i8, kwargs);
        } else {
            __v7 = false;
        }
        '__join_0: {
            if !__v7 {
                if _PyArg_CheckPositional__2!(
                    c"code".as_ptr() as *mut i8,
                    PyTuple_GET_SIZE(args),
                    16i32,
                    18i32
                ) {
                    argcount = PyLong_AsInt(PyTuple_GET_ITEM!(args, 0i32));
                    if argcount == 1i32.wrapping_neg() {
                        __v6 = !PyErr_Occurred().is_null();
                    } else {
                        __v6 = false;
                    }
                    if !__v6 {
                        posonlyargcount = PyLong_AsInt(PyTuple_GET_ITEM!(args, 1i32));
                        if posonlyargcount == 1i32.wrapping_neg() {
                            __v5 = !PyErr_Occurred().is_null();
                        } else {
                            __v5 = false;
                        }
                        if !__v5 {
                            kwonlyargcount = PyLong_AsInt(PyTuple_GET_ITEM!(args, 2i32));
                            if kwonlyargcount == 1i32.wrapping_neg() {
                                __v4 = !PyErr_Occurred().is_null();
                            } else {
                                __v4 = false;
                            }
                            if !__v4 {
                                nlocals = PyLong_AsInt(PyTuple_GET_ITEM!(args, 3i32));
                                if nlocals == 1i32.wrapping_neg() {
                                    __v3 = !PyErr_Occurred().is_null();
                                } else {
                                    __v3 = false;
                                }
                                if !__v3 {
                                    stacksize = PyLong_AsInt(PyTuple_GET_ITEM!(args, 4i32));
                                    if stacksize == 1i32.wrapping_neg() {
                                        __v2 = !PyErr_Occurred().is_null();
                                    } else {
                                        __v2 = false;
                                    }
                                    if !__v2 {
                                        flags = PyLong_AsInt(PyTuple_GET_ITEM!(args, 5i32));
                                        if flags == 1i32.wrapping_neg() {
                                            __v1 = !PyErr_Occurred().is_null();
                                        } else {
                                            __v1 = false;
                                        }
                                        if !__v1 {
                                            if PyBytes_Check!(PyTuple_GET_ITEM!(args, 6i32)) == 0 {
                                                _PyArg_BadArgument(
                                                    c"code".as_ptr(),
                                                    c"argument 7".as_ptr(),
                                                    c"bytes".as_ptr(),
                                                    PyTuple_GET_ITEM!(args, 6i32),
                                                );
                                            } else {
                                                code = PyTuple_GET_ITEM!(args, 6i32);
                                                if PyTuple_Check!(PyTuple_GET_ITEM!(args, 7i32))
                                                    == 0
                                                {
                                                    _PyArg_BadArgument(
                                                        c"code".as_ptr(),
                                                        c"argument 8".as_ptr(),
                                                        c"tuple".as_ptr(),
                                                        PyTuple_GET_ITEM!(args, 7i32),
                                                    );
                                                } else {
                                                    consts = PyTuple_GET_ITEM!(args, 7i32);
                                                    if PyTuple_Check!(PyTuple_GET_ITEM!(args, 8i32))
                                                        == 0
                                                    {
                                                        _PyArg_BadArgument(
                                                            c"code".as_ptr(),
                                                            c"argument 9".as_ptr(),
                                                            c"tuple".as_ptr(),
                                                            PyTuple_GET_ITEM!(args, 8i32),
                                                        );
                                                    } else {
                                                        names = PyTuple_GET_ITEM!(args, 8i32);
                                                        if PyTuple_Check!(PyTuple_GET_ITEM!(
                                                            args, 9i32
                                                        )) == 0
                                                        {
                                                            _PyArg_BadArgument(
                                                                c"code".as_ptr(),
                                                                c"argument 10".as_ptr(),
                                                                c"tuple".as_ptr(),
                                                                PyTuple_GET_ITEM!(args, 9i32),
                                                            );
                                                        } else {
                                                            varnames =
                                                                PyTuple_GET_ITEM!(args, 9i32);
                                                            if PyUnicode_Check!(PyTuple_GET_ITEM!(
                                                                args, 10i32
                                                            )) == 0
                                                            {
                                                                _PyArg_BadArgument(
                                                                    c"code".as_ptr(),
                                                                    c"argument 11".as_ptr(),
                                                                    c"str".as_ptr(),
                                                                    PyTuple_GET_ITEM!(args, 10i32),
                                                                );
                                                            } else {
                                                                filename =
                                                                    PyTuple_GET_ITEM!(args, 10i32);
                                                                if PyUnicode_Check!(
                                                                    PyTuple_GET_ITEM!(args, 11i32)
                                                                ) == 0
                                                                {
                                                                    _PyArg_BadArgument(
                                                                        c"code".as_ptr(),
                                                                        c"argument 12".as_ptr(),
                                                                        c"str".as_ptr(),
                                                                        PyTuple_GET_ITEM!(
                                                                            args, 11i32
                                                                        ),
                                                                    );
                                                                } else {
                                                                    name = PyTuple_GET_ITEM!(
                                                                        args, 11i32
                                                                    );
                                                                    if PyUnicode_Check!(
                                                                        PyTuple_GET_ITEM!(
                                                                            args, 12i32
                                                                        )
                                                                    ) == 0
                                                                    {
                                                                        _PyArg_BadArgument(
                                                                            c"code".as_ptr(),
                                                                            c"argument 13".as_ptr(),
                                                                            c"str".as_ptr(),
                                                                            PyTuple_GET_ITEM!(
                                                                                args, 12i32
                                                                            ),
                                                                        );
                                                                    } else {
                                                                        qualname = PyTuple_GET_ITEM!(
                                                                            args, 12i32
                                                                        );
                                                                        firstlineno = PyLong_AsInt(
                                                                            PyTuple_GET_ITEM!(
                                                                                args, 13i32
                                                                            ),
                                                                        );
                                                                        if firstlineno
                                                                            == 1i32.wrapping_neg()
                                                                        {
                                                                            __v0 =
                                                                                !PyErr_Occurred()
                                                                                    .is_null();
                                                                        } else {
                                                                            __v0 = false;
                                                                        }
                                                                        if !__v0 {
                                                                            if PyBytes_Check!(
                                                                                PyTuple_GET_ITEM!(
                                                                                    args, 14i32
                                                                                )
                                                                            ) == 0
                                                                            {
                                                                                _PyArg_BadArgument(
                                                                                    c"code"
                                                                                        .as_ptr(),
                                                                                    c"argument 15"
                                                                                        .as_ptr(),
                                                                                    c"bytes"
                                                                                        .as_ptr(),
                                                                                    PyTuple_GET_ITEM!(
                                                                                        args, 14i32
                                                                                    ),
                                                                                );
                                                                            } else {
                                                                                linetable__0 = PyTuple_GET_ITEM!(
                                                                                    args, 14i32
                                                                                );
                                                                                if PyBytes_Check!(
                                                                                    PyTuple_GET_ITEM!(
                                                                                        args, 15i32
                                                                                    )
                                                                                ) == 0
                                                                                {
                                                                                    _PyArg_BadArgument(c"code".as_ptr(), c"argument 16".as_ptr(), c"bytes".as_ptr(), PyTuple_GET_ITEM!(args, 15i32));
                                                                                } else {
                                                                                    exceptiontable = PyTuple_GET_ITEM!(args, 15i32);
                                                                                    if PyTuple_GET_SIZE!(
                                                                                        args
                                                                                    ) >= 17
                                                                                    {
                                                                                        if PyTuple_Check!(PyTuple_GET_ITEM!(args, 16i32)) == 0 {
_PyArg_BadArgument(c"code".as_ptr(), c"argument 17".as_ptr(), c"tuple".as_ptr(), PyTuple_GET_ITEM!(args, 16i32));
break '__join_0;
} else {
freevars = PyTuple_GET_ITEM!(args, 16i32);
if PyTuple_GET_SIZE!(args) >= 18 {
if PyTuple_Check!(PyTuple_GET_ITEM!(args, 17i32)) == 0 {
_PyArg_BadArgument(c"code".as_ptr(), c"argument 18".as_ptr(), c"tuple".as_ptr(), PyTuple_GET_ITEM!(args, 17i32));
break '__join_0;
} else {
cellvars = PyTuple_GET_ITEM!(args, 17i32);
}
}
}
                                                                                    }
                                                                                    return_value = code_new_impl(r#type, argcount, posonlyargcount, kwonlyargcount, nlocals, stacksize, flags, code, consts, names, varnames, filename, name, qualname, firstlineno, linetable__0, exceptiontable, freevars, cellvars);
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
                                }
                            }
                        }
                    }
                }
            }
        }
        return return_value;
    }
}

static code_replace__doc__: __SlateAlign16<[i8; 108]> = __SlateAlign16([
    114i8, 101i8, 112i8, 108i8, 97i8, 99i8, 101i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8,
    32i8, 47i8, 44i8, 32i8, 42i8, 42i8, 99i8, 104i8, 97i8, 110i8, 103i8, 101i8, 115i8, 41i8, 10i8,
    45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 97i8, 32i8, 99i8, 111i8,
    112i8, 121i8, 32i8, 111i8, 102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 99i8, 111i8, 100i8, 101i8,
    32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 32i8, 119i8, 105i8, 116i8, 104i8, 32i8, 110i8,
    101i8, 119i8, 32i8, 118i8, 97i8, 108i8, 117i8, 101i8, 115i8, 32i8, 102i8, 111i8, 114i8, 32i8,
    116i8, 104i8, 101i8, 32i8, 115i8, 112i8, 101i8, 99i8, 105i8, 102i8, 105i8, 101i8, 100i8, 32i8,
    102i8, 105i8, 101i8, 108i8, 100i8, 115i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.codeobject.code_replace")]
extern "C-unwind" fn code_replace(
    self_: *mut _object,
    mut args: *const *mut _object,
    nargs: i64,
    kwnames: *mut _object,
) -> *mut _object {
    let mut __v22: bool = unsafe { core::mem::zeroed() };
    let mut __v25: bool = unsafe { core::mem::zeroed() };
    let mut __v28: bool = unsafe { core::mem::zeroed() };
    let mut __v31: bool = unsafe { core::mem::zeroed() };
    let mut __v34: bool = unsafe { core::mem::zeroed() };
    let mut __v37: bool = unsafe { core::mem::zeroed() };
    let mut __v40: bool = unsafe { core::mem::zeroed() };
    let mut co_exceptiontable: *mut _object = unsafe { core::mem::zeroed() };
    let mut co_linetable: *mut _object = unsafe { core::mem::zeroed() };
    let mut co_qualname: *mut _object = unsafe { core::mem::zeroed() };
    let mut co_name: *mut _object = unsafe { core::mem::zeroed() };
    let mut co_filename: *mut _object = unsafe { core::mem::zeroed() };
    let mut co_cellvars: *mut _object = unsafe { core::mem::zeroed() };
    let mut co_freevars: *mut _object = unsafe { core::mem::zeroed() };
    let mut co_varnames: *mut _object = unsafe { core::mem::zeroed() };
    let mut co_names: *mut _object = unsafe { core::mem::zeroed() };
    let mut co_consts: *mut _object = unsafe { core::mem::zeroed() };
    let mut co_code: *mut _object = unsafe { core::mem::zeroed() };
    let mut co_firstlineno: i32 = unsafe { core::mem::zeroed() };
    let mut co_flags: i32 = unsafe { core::mem::zeroed() };
    let mut co_stacksize: i32 = unsafe { core::mem::zeroed() };
    let mut co_nlocals: i32 = unsafe { core::mem::zeroed() };
    let mut co_kwonlyargcount: i32 = unsafe { core::mem::zeroed() };
    let mut co_posonlyargcount: i32 = unsafe { core::mem::zeroed() };
    let mut co_argcount: i32 = unsafe { core::mem::zeroed() };
    let mut __v59: i64 = unsafe { core::mem::zeroed() };
    let mut noptargs: i64 = unsafe { core::mem::zeroed() };
    let mut argsbuf: core::mem::MaybeUninit<__SlateAlign16<[*mut _object; 18]>> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_61: *mut [*mut _object; 18] =
        core::ptr::addr_of_mut!(argsbuf) as *mut [*mut _object; 18];
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        if !kwnames.is_null() {
            __v59 = PyTuple_GET_SIZE!(kwnames);
        } else {
            __v59 = 0;
        }
        '__join_0: {
            noptargs = nargs.wrapping_add(__v59).wrapping_sub(0);
            co_argcount = (*(self_ as *mut PyCodeObject)).co_argcount;
            co_posonlyargcount = (*(self_ as *mut PyCodeObject)).co_posonlyargcount;
            co_kwonlyargcount = (*(self_ as *mut PyCodeObject)).co_kwonlyargcount;
            co_nlocals = (*(self_ as *mut PyCodeObject)).co_nlocals;
            co_stacksize = (*(self_ as *mut PyCodeObject)).co_stacksize;
            co_flags = (*(self_ as *mut PyCodeObject)).co_flags;
            co_firstlineno = (*(self_ as *mut PyCodeObject)).co_firstlineno;
            co_code = core::ptr::null_mut::<_object>();
            co_consts = (*(self_ as *mut PyCodeObject)).co_consts;
            co_names = (*(self_ as *mut PyCodeObject)).co_names;
            co_varnames = core::ptr::null_mut::<_object>();
            co_freevars = core::ptr::null_mut::<_object>();
            co_cellvars = core::ptr::null_mut::<_object>();
            co_filename = (*(self_ as *mut PyCodeObject)).co_filename;
            co_name = (*(self_ as *mut PyCodeObject)).co_name;
            co_qualname = (*(self_ as *mut PyCodeObject)).co_qualname;
            co_linetable = (*(self_ as *mut PyCodeObject)).co_linetable;
            co_exceptiontable = (*(self_ as *mut PyCodeObject)).co_exceptiontable;
            args = _PyArg_UnpackKeywords!(
                args,
                nargs,
                core::ptr::null_mut::<()>(),
                kwnames,
                core::ptr::addr_of_mut!(_parser__201),
                0i32,
                0i32,
                0i32,
                0i32,
                (*__slate_slot_61).as_mut_ptr() as *mut *mut _object
            );
            // minpos
            // maxpos
            // minkw
            // varpos
            if !args.is_null() {
                '__join_1: {
                    if noptargs != 0 {
                        if !unsafe { *args.wrapping_offset(0 as isize) }.is_null() {
                            co_argcount = PyLong_AsInt(*args.wrapping_offset(0 as isize));
                            if co_argcount == 1i32.wrapping_neg() {
                                __v40 = !PyErr_Occurred().is_null();
                            } else {
                                __v40 = false;
                            }
                            if __v40 {
                                break '__join_0;
                            } else {
                                let __v38: i64 = noptargs.wrapping_sub(1);
                                noptargs = __v38;
                                if __v38 == 0 {
                                    break '__join_1;
                                }
                            }
                        }
                        if !unsafe { *args.wrapping_offset(1 as isize) }.is_null() {
                            co_posonlyargcount = PyLong_AsInt(*args.wrapping_offset(1 as isize));
                            if co_posonlyargcount == 1i32.wrapping_neg() {
                                __v37 = !PyErr_Occurred().is_null();
                            } else {
                                __v37 = false;
                            }
                            if __v37 {
                                break '__join_0;
                            } else {
                                let __v35: i64 = noptargs.wrapping_sub(1);
                                noptargs = __v35;
                                if __v35 == 0 {
                                    break '__join_1;
                                }
                            }
                        }
                        if !unsafe { *args.wrapping_offset(2 as isize) }.is_null() {
                            co_kwonlyargcount = PyLong_AsInt(*args.wrapping_offset(2 as isize));
                            if co_kwonlyargcount == 1i32.wrapping_neg() {
                                __v34 = !PyErr_Occurred().is_null();
                            } else {
                                __v34 = false;
                            }
                            if __v34 {
                                break '__join_0;
                            } else {
                                let __v32: i64 = noptargs.wrapping_sub(1);
                                noptargs = __v32;
                                if __v32 == 0 {
                                    break '__join_1;
                                }
                            }
                        }
                        if !unsafe { *args.wrapping_offset(3 as isize) }.is_null() {
                            co_nlocals = PyLong_AsInt(*args.wrapping_offset(3 as isize));
                            if co_nlocals == 1i32.wrapping_neg() {
                                __v31 = !PyErr_Occurred().is_null();
                            } else {
                                __v31 = false;
                            }
                            if __v31 {
                                break '__join_0;
                            } else {
                                let __v29: i64 = noptargs.wrapping_sub(1);
                                noptargs = __v29;
                                if __v29 == 0 {
                                    break '__join_1;
                                }
                            }
                        }
                        if !unsafe { *args.wrapping_offset(4 as isize) }.is_null() {
                            co_stacksize = PyLong_AsInt(*args.wrapping_offset(4 as isize));
                            if co_stacksize == 1i32.wrapping_neg() {
                                __v28 = !PyErr_Occurred().is_null();
                            } else {
                                __v28 = false;
                            }
                            if __v28 {
                                break '__join_0;
                            } else {
                                let __v26: i64 = noptargs.wrapping_sub(1);
                                noptargs = __v26;
                                if __v26 == 0 {
                                    break '__join_1;
                                }
                            }
                        }
                        if !unsafe { *args.wrapping_offset(5 as isize) }.is_null() {
                            co_flags = PyLong_AsInt(*args.wrapping_offset(5 as isize));
                            if co_flags == 1i32.wrapping_neg() {
                                __v25 = !PyErr_Occurred().is_null();
                            } else {
                                __v25 = false;
                            }
                            if __v25 {
                                break '__join_0;
                            } else {
                                let __v23: i64 = noptargs.wrapping_sub(1);
                                noptargs = __v23;
                                if __v23 == 0 {
                                    break '__join_1;
                                }
                            }
                        }
                        if !unsafe { *args.wrapping_offset(6 as isize) }.is_null() {
                            co_firstlineno = PyLong_AsInt(*args.wrapping_offset(6 as isize));
                            if co_firstlineno == 1i32.wrapping_neg() {
                                __v22 = !PyErr_Occurred().is_null();
                            } else {
                                __v22 = false;
                            }
                            if __v22 {
                                break '__join_0;
                            } else {
                                let __v20: i64 = noptargs.wrapping_sub(1);
                                noptargs = __v20;
                                if __v20 == 0 {
                                    break '__join_1;
                                }
                            }
                        }
                        if !unsafe { *args.wrapping_offset(7 as isize) }.is_null() {
                            if PyBytes_Check!(unsafe { *args.wrapping_offset(7 as isize) }) == 0 {
                                _PyArg_BadArgument(
                                    c"replace".as_ptr(),
                                    c"argument 'co_code'".as_ptr(),
                                    c"bytes".as_ptr(),
                                    *args.wrapping_offset(7 as isize),
                                );
                                break '__join_0;
                            } else {
                                co_code = *args.wrapping_offset(7 as isize);
                                let __v18: i64 = noptargs.wrapping_sub(1);
                                noptargs = __v18;
                                if __v18 == 0 {
                                    break '__join_1;
                                }
                            }
                        }
                        if !unsafe { *args.wrapping_offset(8 as isize) }.is_null() {
                            if PyTuple_Check!(unsafe { *args.wrapping_offset(8 as isize) }) == 0 {
                                _PyArg_BadArgument(
                                    c"replace".as_ptr(),
                                    c"argument 'co_consts'".as_ptr(),
                                    c"tuple".as_ptr(),
                                    *args.wrapping_offset(8 as isize),
                                );
                                break '__join_0;
                            } else {
                                co_consts = *args.wrapping_offset(8 as isize);
                                let __v16: i64 = noptargs.wrapping_sub(1);
                                noptargs = __v16;
                                if __v16 == 0 {
                                    break '__join_1;
                                }
                            }
                        }
                        if !unsafe { *args.wrapping_offset(9 as isize) }.is_null() {
                            if PyTuple_Check!(unsafe { *args.wrapping_offset(9 as isize) }) == 0 {
                                _PyArg_BadArgument(
                                    c"replace".as_ptr(),
                                    c"argument 'co_names'".as_ptr(),
                                    c"tuple".as_ptr(),
                                    *args.wrapping_offset(9 as isize),
                                );
                                break '__join_0;
                            } else {
                                co_names = *args.wrapping_offset(9 as isize);
                                let __v14: i64 = noptargs.wrapping_sub(1);
                                noptargs = __v14;
                                if __v14 == 0 {
                                    break '__join_1;
                                }
                            }
                        }
                        if !unsafe { *args.wrapping_offset(10 as isize) }.is_null() {
                            if PyTuple_Check!(unsafe { *args.wrapping_offset(10 as isize) }) == 0 {
                                _PyArg_BadArgument(
                                    c"replace".as_ptr(),
                                    c"argument 'co_varnames'".as_ptr(),
                                    c"tuple".as_ptr(),
                                    *args.wrapping_offset(10 as isize),
                                );
                                break '__join_0;
                            } else {
                                co_varnames = *args.wrapping_offset(10 as isize);
                                let __v12: i64 = noptargs.wrapping_sub(1);
                                noptargs = __v12;
                                if __v12 == 0 {
                                    break '__join_1;
                                }
                            }
                        }
                        if !unsafe { *args.wrapping_offset(11 as isize) }.is_null() {
                            if PyTuple_Check!(unsafe { *args.wrapping_offset(11 as isize) }) == 0 {
                                _PyArg_BadArgument(
                                    c"replace".as_ptr(),
                                    c"argument 'co_freevars'".as_ptr(),
                                    c"tuple".as_ptr(),
                                    *args.wrapping_offset(11 as isize),
                                );
                                break '__join_0;
                            } else {
                                co_freevars = *args.wrapping_offset(11 as isize);
                                let __v10: i64 = noptargs.wrapping_sub(1);
                                noptargs = __v10;
                                if __v10 == 0 {
                                    break '__join_1;
                                }
                            }
                        }
                        if !unsafe { *args.wrapping_offset(12 as isize) }.is_null() {
                            if PyTuple_Check!(unsafe { *args.wrapping_offset(12 as isize) }) == 0 {
                                _PyArg_BadArgument(
                                    c"replace".as_ptr(),
                                    c"argument 'co_cellvars'".as_ptr(),
                                    c"tuple".as_ptr(),
                                    *args.wrapping_offset(12 as isize),
                                );
                                break '__join_0;
                            } else {
                                co_cellvars = *args.wrapping_offset(12 as isize);
                                let __v8: i64 = noptargs.wrapping_sub(1);
                                noptargs = __v8;
                                if __v8 == 0 {
                                    break '__join_1;
                                }
                            }
                        }
                        if !unsafe { *args.wrapping_offset(13 as isize) }.is_null() {
                            if PyUnicode_Check!(unsafe { *args.wrapping_offset(13 as isize) }) == 0
                            {
                                _PyArg_BadArgument(
                                    c"replace".as_ptr(),
                                    c"argument 'co_filename'".as_ptr(),
                                    c"str".as_ptr(),
                                    *args.wrapping_offset(13 as isize),
                                );
                                break '__join_0;
                            } else {
                                co_filename = *args.wrapping_offset(13 as isize);
                                let __v6: i64 = noptargs.wrapping_sub(1);
                                noptargs = __v6;
                                if __v6 == 0 {
                                    break '__join_1;
                                }
                            }
                        }
                        if !unsafe { *args.wrapping_offset(14 as isize) }.is_null() {
                            if PyUnicode_Check!(unsafe { *args.wrapping_offset(14 as isize) }) == 0
                            {
                                _PyArg_BadArgument(
                                    c"replace".as_ptr(),
                                    c"argument 'co_name'".as_ptr(),
                                    c"str".as_ptr(),
                                    *args.wrapping_offset(14 as isize),
                                );
                                break '__join_0;
                            } else {
                                co_name = *args.wrapping_offset(14 as isize);
                                let __v4: i64 = noptargs.wrapping_sub(1);
                                noptargs = __v4;
                                if __v4 == 0 {
                                    break '__join_1;
                                }
                            }
                        }
                        if !unsafe { *args.wrapping_offset(15 as isize) }.is_null() {
                            if PyUnicode_Check!(unsafe { *args.wrapping_offset(15 as isize) }) == 0
                            {
                                _PyArg_BadArgument(
                                    c"replace".as_ptr(),
                                    c"argument 'co_qualname'".as_ptr(),
                                    c"str".as_ptr(),
                                    *args.wrapping_offset(15 as isize),
                                );
                                break '__join_0;
                            } else {
                                co_qualname = *args.wrapping_offset(15 as isize);
                                let __v2: i64 = noptargs.wrapping_sub(1);
                                noptargs = __v2;
                                if __v2 == 0 {
                                    break '__join_1;
                                }
                            }
                        }
                        if !unsafe { *args.wrapping_offset(16 as isize) }.is_null() {
                            if PyBytes_Check!(unsafe { *args.wrapping_offset(16 as isize) }) == 0 {
                                _PyArg_BadArgument(
                                    c"replace".as_ptr(),
                                    c"argument 'co_linetable'".as_ptr(),
                                    c"bytes".as_ptr(),
                                    *args.wrapping_offset(16 as isize),
                                );
                                break '__join_0;
                            } else {
                                co_linetable = *args.wrapping_offset(16 as isize);
                                let __v0: i64 = noptargs.wrapping_sub(1);
                                noptargs = __v0;
                                if __v0 == 0 {
                                    break '__join_1;
                                }
                            }
                        }
                        if PyBytes_Check!(unsafe { *args.wrapping_offset(17 as isize) }) == 0 {
                            _PyArg_BadArgument(
                                c"replace".as_ptr(),
                                c"argument 'co_exceptiontable'".as_ptr(),
                                c"bytes".as_ptr(),
                                *args.wrapping_offset(17 as isize),
                            );
                            break '__join_0;
                        } else {
                            co_exceptiontable = *args.wrapping_offset(17 as isize);
                        }
                    }
                }
                return_value = code_replace_impl(
                    self_ as *mut PyCodeObject,
                    co_argcount,
                    co_posonlyargcount,
                    co_kwonlyargcount,
                    co_nlocals,
                    co_stacksize,
                    co_flags,
                    co_firstlineno,
                    co_code,
                    co_consts,
                    co_names,
                    co_varnames,
                    co_freevars,
                    co_cellvars,
                    co_filename,
                    co_name,
                    co_qualname,
                    co_linetable,
                    co_exceptiontable,
                );
            }
        }
        return return_value;
    }
}

static mut _kwtuple__188: __SlateRecord_codeobject_c_h_188 = {
    let mut __t0: __SlateRecord_codeobject_c_h_188 = unsafe { core::mem::zeroed() };
    __t0.ob_base = __SlateRecord_object_h_172 {
        ob_base: _object {
            __slate_anon_0: __SlateRecord_object_h_122 {
                ob_refcnt_full: 1407378104778752i64,
            },
            ob_type: unsafe { core::ptr::addr_of_mut!(PyTuple_Type) },
        },
        ob_size: 18i64,
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
                    ._py_co_argcount
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
                    ._py_co_posonlyargcount
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
                    ._py_co_kwonlyargcount
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
                    ._py_co_nlocals
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
                    ._py_co_stacksize
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
                    ._py_co_flags
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
                    ._py_co_firstlineno
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
                    ._py_co_code
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
                    ._py_co_consts
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
                    ._py_co_names
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
                    ._py_co_varnames
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
                    ._py_co_freevars
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
                    ._py_co_cellvars
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
                    ._py_co_filename
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
                    ._py_co_name
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
                    ._py_co_qualname
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
                    ._py_co_linetable
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
                    ._py_co_exceptiontable
                    ._ascii
                    .ob_base
            )
        },
    ];
    __t0
};

static _keywords__200: __SlateAlign16<[*const i8; 19]> = __SlateAlign16([
    (b"co_argcount\0".as_ptr() as *mut i8) as *const i8,
    (b"co_posonlyargcount\0".as_ptr() as *mut i8) as *const i8,
    (b"co_kwonlyargcount\0".as_ptr() as *mut i8) as *const i8,
    (b"co_nlocals\0".as_ptr() as *mut i8) as *const i8,
    (b"co_stacksize\0".as_ptr() as *mut i8) as *const i8,
    (b"co_flags\0".as_ptr() as *mut i8) as *const i8,
    (b"co_firstlineno\0".as_ptr() as *mut i8) as *const i8,
    (b"co_code\0".as_ptr() as *mut i8) as *const i8,
    (b"co_consts\0".as_ptr() as *mut i8) as *const i8,
    (b"co_names\0".as_ptr() as *mut i8) as *const i8,
    (b"co_varnames\0".as_ptr() as *mut i8) as *const i8,
    (b"co_freevars\0".as_ptr() as *mut i8) as *const i8,
    (b"co_cellvars\0".as_ptr() as *mut i8) as *const i8,
    (b"co_filename\0".as_ptr() as *mut i8) as *const i8,
    (b"co_name\0".as_ptr() as *mut i8) as *const i8,
    (b"co_qualname\0".as_ptr() as *mut i8) as *const i8,
    (b"co_linetable\0".as_ptr() as *mut i8) as *const i8,
    (b"co_exceptiontable\0".as_ptr() as *mut i8) as *const i8,
    core::ptr::null::<i8>(),
]);

static mut _parser__201: _PyArg_Parser = {
    let mut __t0: _PyArg_Parser = unsafe { core::mem::zeroed() };
    __t0.keywords = unsafe { core::ptr::addr_of!(_keywords__200.0) as *const *const i8 };
    __t0.fname = (b"replace\0".as_ptr() as *mut i8) as *const i8;
    __t0.kwtuple = unsafe { core::ptr::addr_of_mut!(_kwtuple__188.ob_base.ob_base) };
    __t0
};

static code__varname_from_oparg__doc__: __SlateAlign16<[i8; 183]> = __SlateAlign16([
    95i8, 118i8, 97i8, 114i8, 110i8, 97i8, 109i8, 101i8, 95i8, 102i8, 114i8, 111i8, 109i8, 95i8,
    111i8, 112i8, 97i8, 114i8, 103i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8, 47i8,
    44i8, 32i8, 111i8, 112i8, 97i8, 114i8, 103i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 40i8, 105i8,
    110i8, 116i8, 101i8, 114i8, 110i8, 97i8, 108i8, 45i8, 111i8, 110i8, 108i8, 121i8, 41i8, 32i8,
    82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 108i8, 111i8, 99i8,
    97i8, 108i8, 32i8, 118i8, 97i8, 114i8, 105i8, 97i8, 98i8, 108i8, 101i8, 32i8, 110i8, 97i8,
    109i8, 101i8, 32i8, 102i8, 111i8, 114i8, 32i8, 116i8, 104i8, 101i8, 32i8, 103i8, 105i8, 118i8,
    101i8, 110i8, 32i8, 111i8, 112i8, 97i8, 114i8, 103i8, 46i8, 10i8, 10i8, 87i8, 65i8, 82i8, 78i8,
    73i8, 78i8, 71i8, 58i8, 32i8, 116i8, 104i8, 105i8, 115i8, 32i8, 109i8, 101i8, 116i8, 104i8,
    111i8, 100i8, 32i8, 105i8, 115i8, 32i8, 102i8, 111i8, 114i8, 32i8, 105i8, 110i8, 116i8, 101i8,
    114i8, 110i8, 97i8, 108i8, 32i8, 117i8, 115i8, 101i8, 32i8, 111i8, 110i8, 108i8, 121i8, 32i8,
    97i8, 110i8, 100i8, 32i8, 109i8, 97i8, 121i8, 32i8, 99i8, 104i8, 97i8, 110i8, 103i8, 101i8,
    32i8, 111i8, 114i8, 32i8, 103i8, 111i8, 10i8, 97i8, 119i8, 97i8, 121i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.codeobject.code__varname_from_oparg")]
extern "C-unwind" fn code__varname_from_oparg(
    self_: *mut _object,
    mut args: *const *mut _object,
    nargs: i64,
    kwnames: *mut _object,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut oparg: i32 = unsafe { core::mem::zeroed() };
    let mut argsbuf: [*mut _object; 1] = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        args = _PyArg_UnpackKeywords!(
            args,
            nargs,
            core::ptr::null_mut::<()>(),
            kwnames,
            core::ptr::addr_of_mut!(_parser__451),
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
            oparg = PyLong_AsInt(*args.wrapping_offset(0 as isize));
            if oparg == 1i32.wrapping_neg() {
                __v0 = !PyErr_Occurred().is_null();
            } else {
                __v0 = false;
            }
            if !__v0 {
                return_value = code__varname_from_oparg_impl(self_ as *mut PyCodeObject, oparg);
            }
        }
        return return_value;
    }
}

static mut _kwtuple__438: __SlateRecord_codeobject_c_h_438 = {
    let mut __t0: __SlateRecord_codeobject_c_h_438 = unsafe { core::mem::zeroed() };
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
                ._py_oparg
                ._ascii
                .ob_base
        )
    }];
    __t0
};

static _keywords__450: __SlateAlign16<[*const i8; 2]> = __SlateAlign16([
    (b"oparg\0".as_ptr() as *mut i8) as *const i8,
    core::ptr::null::<i8>(),
]);

static mut _parser__451: _PyArg_Parser = {
    let mut __t0: _PyArg_Parser = unsafe { core::mem::zeroed() };
    __t0.keywords = unsafe { core::ptr::addr_of!(_keywords__450.0) as *const *const i8 };
    __t0.fname = (b"_varname_from_oparg\0".as_ptr() as *mut i8) as *const i8;
    __t0.kwtuple = unsafe { core::ptr::addr_of_mut!(_kwtuple__438.ob_base.ob_base) };
    __t0
};

// _PyFunction_ClearCodeByVersion()
// _Py_hashtable_t
// _PyIndexPool_Fini()
// _PyStatus_OK()
// PyInterpreterState.co_extra_freefuncs
// FRAME_SPECIALS_SIZE
// _PyOpcode_Caches
// RESUME_AT_FUNC_START
// _Py_ExecutorDetach
// _PyMem_FreeDelayed()
// _PyInterpreterState_GET()
// _PySet_NextEntry()
// _PyTuple_ITEMS()
// _PyUnicode_InternImmortal()
// _PyObject_AssignUniqueId()
// FT_CLEAR_WEAKREFS()
fn code_event_name(event: u32) -> *const i8 {
    match event as u32 {
        0 => {
            return c"PY_CODE_EVENT_CREATE".as_ptr();
        }
        1 => {
            return c"PY_CODE_EVENT_DESTROY".as_ptr();
        }
        _ => {}
    }
    Py_UNREACHABLE!();
    unsafe { core::mem::zeroed() }
}

fn notify_code_watchers(event: u32, co: *mut PyCodeObject) {
    let interp: *mut _is = _PyInterpreterState_GET();
    let mut bits: u8 = unsafe { (*interp).active_code_watchers };
    let mut i: i32 = 0;
    while bits != 0 {
        if (bits as i32) & 1 != 0 {
            let cb: Option<unsafe extern "C-unwind" fn(u32, *mut PyCodeObject) -> i32> = unsafe {
                *((*interp).code_watchers.as_mut_ptr()
                    as *mut Option<unsafe extern "C-unwind" fn(u32, *mut PyCodeObject) -> i32>)
                    .wrapping_offset(i as isize)
            };
            // callback must be non-null if the watcher bit is set
            if (unsafe { cb.unwrap()(event, co) }) < 0 {
                unsafe {
                    PyErr_FormatUnraisable(
                        c"Exception ignored in %s watcher callback for %R".as_ptr(),
                        code_event_name(event),
                        co,
                    )
                };
            }
        }
        i = i.wrapping_add(1);
        bits = ((bits as i32) >> 1i32) as u8;
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyCode_AddWatcher(
    callback: Option<unsafe extern "C-unwind" fn(u32, *mut PyCodeObject) -> i32>,
) -> i32 {
    let interp: *mut _is = _PyInterpreterState_GET();
    for i in 0i32..8i32 {
        if (unsafe {
            *((*interp).code_watchers.as_mut_ptr()
                as *mut Option<unsafe extern "C-unwind" fn(u32, *mut PyCodeObject) -> i32>)
                .wrapping_offset(i as isize)
        }) == None
        {
            unsafe {
                *((*interp).code_watchers.as_mut_ptr()
                    as *mut Option<unsafe extern "C-unwind" fn(u32, *mut PyCodeObject) -> i32>)
                    .wrapping_offset(i as isize) = callback;
            }
            unsafe {
                (*interp).active_code_watchers =
                    (((*interp).active_code_watchers as i32) | 1i32 << i) as u8;
            }
            return i;
        }
    }
    unsafe {
        PyErr_SetString(
            PyExc_RuntimeError,
            c"no more code watcher IDs available".as_ptr(),
        )
    };
    1i32.wrapping_neg()
}

fn validate_watcher_id(interp: *mut _is, watcher_id: i32) -> i32 {
    if watcher_id < 0 || watcher_id >= 8 {
        unsafe {
            PyErr_Format(
                PyExc_ValueError,
                c"Invalid code watcher ID %d".as_ptr(),
                watcher_id,
            )
        };
        return 1i32.wrapping_neg();
    }
    if (unsafe {
        *((*interp).code_watchers.as_mut_ptr()
            as *mut Option<unsafe extern "C-unwind" fn(u32, *mut PyCodeObject) -> i32>)
            .wrapping_offset(watcher_id as isize)
    }) == None
    {
        unsafe {
            PyErr_Format(
                PyExc_ValueError,
                c"No code watcher set for ID %d".as_ptr(),
                watcher_id,
            )
        };
        return 1i32.wrapping_neg();
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyCode_ClearWatcher(watcher_id: i32) -> i32 {
    let interp: *mut _is = _PyInterpreterState_GET();
    if validate_watcher_id(interp, watcher_id) < 0 {
        return 1i32.wrapping_neg();
    }
    unsafe {
        *((*interp).code_watchers.as_mut_ptr()
            as *mut Option<unsafe extern "C-unwind" fn(u32, *mut PyCodeObject) -> i32>)
            .wrapping_offset(watcher_id as isize) = None;
    }
    let __v2: u8 =
        (((unsafe { (*interp).active_code_watchers }) as i32) & !(1i32 << watcher_id)) as u8;
    unsafe {
        (*interp).active_code_watchers = __v2;
    }
    0
}

// generic helpers
fn should_intern_string(o: *mut _object) -> i32 {
    // compute if s matches [a-zA-Z0-9_]
    let mut s: *const u8 = unsafe { core::mem::zeroed() };
    if PyUnicode_IS_ASCII!(o) == 0 {
        return 0;
    }
    s = PyUnicode_1BYTE_DATA!(o) as *const u8;
    let e: *const u8 = s.wrapping_offset(PyUnicode_GET_LENGTH!(o) as isize);
    while s != e {
        if Py_ISALNUM!(unsafe { *s }) == 0 && ((unsafe { *s }) as i32) != 95 {
            return 0;
        }
        let __v1: *const u8 = s;
        s = __v1.wrapping_offset(1 as isize);
    }
    1
}

fn intern_strings(tuple: *mut _object) -> i32 {
    let interp: *mut _is = _PyInterpreterState_GET();
    let mut i: i64 = PyTuple_GET_SIZE!(tuple);
    loop {
        let __v2: i64 = i.wrapping_sub(1);
        i = __v2;
        if __v2 < 0 {
            break;
        }
        let v: *mut _object = PyTuple_GET_ITEM!(tuple, i);
        let __v0: bool = v.is_null() || PyUnicode_CheckExact!(v) == 0;
        if __v0 {
            unsafe {
                PyErr_SetString(PyExc_SystemError, c"non-string found in code slot".as_ptr())
            };
            return 1i32.wrapping_neg();
        }
        unsafe {
            _PyUnicode_InternImmortal(interp, _PyTuple_ITEMS!(tuple).wrapping_offset(i as isize))
        };
    }
    0
}

/// Intern constants. In the default build, this interns selected string
/// constants. In the free-threaded build, this also interns non-string
/// constants.
fn intern_constants(tuple: *mut _object, modified: *mut i32) -> i32 {
    let interp: *mut _is = _PyInterpreterState_GET();
    let mut i: i64 = PyTuple_GET_SIZE!(tuple);
    loop {
        let __v1: i64 = i.wrapping_sub(1);
        i = __v1;
        if __v1 < 0 {
            break;
        }
        let mut v: *mut _object = PyTuple_GET_ITEM!(tuple, i);
        if PyUnicode_CheckExact!(v) != 0 {
            if should_intern_string(v) != 0 {
                let w: *mut _object = v;
                unsafe { _PyUnicode_InternMortal(interp, core::ptr::addr_of_mut!(v)) };
                if w != v {
                    PyTuple_SET_ITEM__2!(tuple, i, v);
                    if !modified.is_null() {
                        unsafe {
                            *modified = 1;
                        }
                    }
                }
            }
        } else {
            if PyTuple_CheckExact!(v) != 0 {
                if intern_constants(v, core::ptr::null_mut::<i32>()) < 0 {
                    return 1i32.wrapping_neg();
                }
            } else {
                if PyFrozenSet_CheckExact!(v) != 0 {
                    let w: *mut _object = v;
                    let tmp: *mut _object = unsafe { PySequence_Tuple(v) };
                    if tmp.is_null() {
                        return 1i32.wrapping_neg();
                    }
                    let mut tmp_modified: i32 = 0;
                    if intern_constants(tmp, core::ptr::addr_of_mut!(tmp_modified)) < 0 {
                        Py_DECREF!(tmp);
                        return 1i32.wrapping_neg();
                    }
                    if tmp_modified != 0 {
                        v = unsafe { PyFrozenSet_New(tmp) };
                        if v.is_null() {
                            Py_DECREF!(tmp);
                            return 1i32.wrapping_neg();
                        }
                        PyTuple_SET_ITEM__2!(tuple, i, v);
                        Py_DECREF!(w);
                        if !modified.is_null() {
                            unsafe {
                                *modified = 1;
                            }
                        }
                    }
                    Py_DECREF!(tmp);
                }
            }
        }
    }
    0
}

/// Return a shallow copy of a tuple that is
/// guaranteed to contain exact strings, by converting string subclasses
/// to exact strings and complaining if a non-string is found.
fn validate_and_copy_tuple(tup: *mut _object) -> *mut _object {
    let mut item: *mut _object = unsafe { core::mem::zeroed() };
    let len: i64 = PyTuple_GET_SIZE!(tup);
    let newtuple: *mut _object = unsafe { PyTuple_New(len) };
    if newtuple.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    for i in 0..len {
        item = PyTuple_GET_ITEM!(tup, i);
        if PyUnicode_CheckExact!(item) != 0 {
            Py_INCREF!(item);
        } else {
            if PyUnicode_Check!(item) == 0 {
                unsafe {
                    PyErr_Format(
                        PyExc_TypeError,
                        c"name tuples must contain only strings, not '%.500s'".as_ptr(),
                        (*Py_TYPE!(item)).tp_name,
                    )
                };
                Py_DECREF!(newtuple);
                return core::ptr::null_mut::<_object>();
            } else {
                item = unsafe { _PyUnicode_Copy(item) };
                if item.is_null() {
                    Py_DECREF!(newtuple);
                    return core::ptr::null_mut::<_object>();
                }
            }
        }
        PyTuple_SET_ITEM__2!(newtuple, i, item);
    }
    newtuple
}

fn init_co_cached(self_: *mut PyCodeObject) -> i32 {
    let mut cached: *mut __SlateRecord_code_h_16 = unsafe { (*self_)._co_cached };
    if !cached.is_null() {
        return 0;
    }
    cached = unsafe { (*self_)._co_cached };
    if cached.is_null() {
        cached = PyMem_New__13!(__SlateRecord_code_h_16, 1i32);
        if cached.is_null() {
            unsafe { PyErr_NoMemory() };
        } else {
            unsafe {
                (*cached)._co_code = core::ptr::null_mut::<_object>();
            }
            unsafe {
                (*cached)._co_cellvars = core::ptr::null_mut::<_object>();
            }
            unsafe {
                (*cached)._co_freevars = core::ptr::null_mut::<_object>();
            }
            unsafe {
                (*cached)._co_varnames = core::ptr::null_mut::<_object>();
            }
            FT_ATOMIC_STORE_PTR!((*self_)._co_cached, cached);
        }
    }
    if !cached.is_null() {
        0
    } else {
        1i32.wrapping_neg()
    }
}

// _PyCode_New()
/// This is also used in compile.c.
#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_set_localsplus_info(
    offset: i32,
    name: *mut _object,
    kind: u8,
    names: *mut _object,
    kinds: *mut _object,
) {
    PyTuple_SET_ITEM!(names, offset, Py_NewRef!(name));
    _PyLocals_SetKind(kinds, offset, kind);
}

fn get_localsplus_counts(
    names: *mut _object,
    kinds: *mut _object,
    pnlocals: *mut i32,
    pncellvars: *mut i32,
    pnfreevars: *mut i32,
) {
    let mut nlocals: i32 = 0;
    let mut ncellvars: i32 = 0;
    let mut nfreevars: i32 = 0;
    let nlocalsplus: i64 = PyTuple_GET_SIZE!(names);
    let mut i: i32 = 0;
    while (i as i64) < nlocalsplus {
        let kind: u8 = _PyLocals_GetKind(kinds, i);
        if (kind as i32) & 32 != 0 {
            nlocals = nlocals.wrapping_add(1);
            if (kind as i32) & 64 != 0 {
                ncellvars = ncellvars.wrapping_add(1);
            }
        } else {
            if (kind as i32) & 64 != 0 {
                ncellvars = ncellvars.wrapping_add(1);
            } else {
                if (kind as i32) & 128 != 0 {
                    nfreevars = nfreevars.wrapping_add(1);
                }
            }
        }
        i = i.wrapping_add(1);
    }
    if !pnlocals.is_null() {
        unsafe {
            *pnlocals = nlocals;
        }
    }
    if !pncellvars.is_null() {
        unsafe {
            *pncellvars = ncellvars;
        }
    }
    if !pnfreevars.is_null() {
        unsafe {
            *pnfreevars = nfreevars;
        }
    }
}

fn get_localsplus_names(co: *mut PyCodeObject, kind: u8, num: i32) -> *mut _object {
    let names: *mut _object = unsafe { PyTuple_New(num as i64) };
    if names.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let mut index: i32 = 0;
    let mut offset: i32 = 0;
    while offset < unsafe { (*co).co_nlocalsplus } {
        let k: u8 = _PyLocals_GetKind(unsafe { (*co).co_localspluskinds }, offset);
        if (k as i32) & (kind as i32) != 0 {
            let name: *mut _object = PyTuple_GET_ITEM!(unsafe { (*co).co_localsplusnames }, offset);
            PyTuple_SET_ITEM!(names, index, Py_NewRef!(name));
            index = index.wrapping_add(1);
        }
        offset = offset.wrapping_add(1);
    }
    names
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyCode_Validate(con: *mut _PyCodeConstructor) -> i32 {
    // Check argument types
    let __v0: bool = (unsafe { (*con).argcount }) < unsafe { (*con).posonlyargcount }
        || (unsafe { (*con).posonlyargcount }) < 0
        || (unsafe { (*con).kwonlyargcount }) < 0
        || (unsafe { (*con).stacksize }) < 0
        || (unsafe { (*con).flags }) < 0
        || unsafe { (*con).code }.is_null()
        || PyBytes_Check!(unsafe { (*con).code }) == 0;
    let __v1: bool =
        __v0 || unsafe { (*con).consts }.is_null() || PyTuple_Check!(unsafe { (*con).consts }) == 0;
    let __v2: bool =
        __v1 || unsafe { (*con).names }.is_null() || PyTuple_Check!(unsafe { (*con).names }) == 0;
    let __v3: bool = __v2
        || unsafe { (*con).localsplusnames }.is_null()
        || PyTuple_Check!(unsafe { (*con).localsplusnames }) == 0;
    let __v4: bool = __v3
        || unsafe { (*con).localspluskinds }.is_null()
        || PyBytes_Check!(unsafe { (*con).localspluskinds }) == 0;
    let __v5: bool = __v4
        || PyTuple_GET_SIZE!(unsafe { (*con).localsplusnames })
            != PyBytes_GET_SIZE!(unsafe { (*con).localspluskinds });
    let __v6: bool =
        __v5 || unsafe { (*con).name }.is_null() || PyUnicode_Check!(unsafe { (*con).name }) == 0;
    let __v7: bool = __v6
        || unsafe { (*con).qualname }.is_null()
        || PyUnicode_Check!(unsafe { (*con).qualname }) == 0;
    let __v8: bool = __v7
        || unsafe { (*con).filename }.is_null()
        || PyUnicode_Check!(unsafe { (*con).filename }) == 0;
    let __v9: bool = __v8
        || unsafe { (*con).linetable }.is_null()
        || PyBytes_Check!(unsafe { (*con).linetable }) == 0;
    let __v10: bool = __v9
        || unsafe { (*con).exceptiontable }.is_null()
        || PyBytes_Check!(unsafe { (*con).exceptiontable }) == 0;
    if __v10 {
        PyErr_BadInternalCall__131!();
        return 1i32.wrapping_neg();
    }
    // Make sure that code is indexable with an int, this is
    // a long running assumption in ceval.c and many parts of
    // the interpreter.
    if PyBytes_GET_SIZE!(unsafe { (*con).code }) > 2147483647 {
        unsafe {
            PyErr_SetString(
                PyExc_OverflowError,
                c"code: co_code larger than INT_MAX".as_ptr(),
            )
        };
        return 1i32.wrapping_neg();
    }
    let __v11: bool = (PyBytes_GET_SIZE!(unsafe { (*con).code }) as u64)
        % (core::mem::size_of::<__SlateRecord_pycore_structs_h_32>() as u64)
        != 0
        || !_Py_IS_ALIGNED__2!(
            PyBytes_AS_STRING!(unsafe { (*con).code }),
            core::mem::size_of::<__SlateRecord_pycore_structs_h_32>() as u64
        );
    if __v11 {
        unsafe { PyErr_SetString(PyExc_ValueError, c"code: co_code is malformed".as_ptr()) };
        return 1i32.wrapping_neg();
    }
    // Ensure that the co_varnames has enough names to cover the arg counts.
    // Note that totalargs = nlocals - nplainlocals.  We check nplainlocals
    // here to avoid the possibility of overflow (however remote).
    let mut nlocals: i32 = 0;
    get_localsplus_counts(
        unsafe { (*con).localsplusnames },
        unsafe { (*con).localspluskinds },
        core::ptr::addr_of_mut!(nlocals),
        core::ptr::null_mut::<i32>(),
        core::ptr::null_mut::<i32>(),
    );
    let nplainlocals: i32 = nlocals
        .wrapping_sub(unsafe { (*con).argcount })
        .wrapping_sub(unsafe { (*con).kwonlyargcount })
        .wrapping_sub(((unsafe { (*con).flags }) & 4 != 0) as i32)
        .wrapping_sub(((unsafe { (*con).flags }) & 8 != 0) as i32);
    if nplainlocals < 0 {
        unsafe { PyErr_SetString(PyExc_ValueError, c"code: co_varnames is too small".as_ptr()) };
        return 1i32.wrapping_neg();
    }
    0
}

fn init_code(co: *mut PyCodeObject, con: *mut _PyCodeConstructor) -> i32 {
    let nlocalsplus: i32 = PyTuple_GET_SIZE!(unsafe { (*con).localsplusnames }) as i32;
    let mut nlocals: i32 = 0;
    let mut ncellvars: i32 = 0;
    let mut nfreevars: i32 = 0;
    get_localsplus_counts(
        unsafe { (*con).localsplusnames },
        unsafe { (*con).localspluskinds },
        core::ptr::addr_of_mut!(nlocals),
        core::ptr::addr_of_mut!(ncellvars),
        core::ptr::addr_of_mut!(nfreevars),
    );
    if (unsafe { (*con).stacksize }) == 0 {
        unsafe {
            (*con).stacksize = 1;
        }
    }
    let interp: *mut _is = _PyInterpreterState_GET();
    unsafe {
        (*co).co_filename = Py_NewRef!(unsafe { (*con).filename });
    }
    unsafe {
        (*co).co_name = Py_NewRef!(unsafe { (*con).name });
    }
    unsafe {
        (*co).co_qualname = Py_NewRef!(unsafe { (*con).qualname });
    }
    unsafe { _PyUnicode_InternMortal(interp, core::ptr::addr_of_mut!((*co).co_filename)) };
    unsafe { _PyUnicode_InternMortal(interp, core::ptr::addr_of_mut!((*co).co_name)) };
    unsafe { _PyUnicode_InternMortal(interp, core::ptr::addr_of_mut!((*co).co_qualname)) };
    unsafe {
        (*co).co_flags = (*con).flags;
    }
    unsafe {
        (*co).co_firstlineno = (*con).firstlineno;
    }
    unsafe {
        (*co).co_linetable = Py_NewRef!(unsafe { (*con).linetable });
    }
    unsafe {
        (*co).co_consts = Py_NewRef!(unsafe { (*con).consts });
    }
    unsafe {
        (*co).co_names = Py_NewRef!(unsafe { (*con).names });
    }
    unsafe {
        (*co).co_localsplusnames = Py_NewRef!(unsafe { (*con).localsplusnames });
    }
    unsafe {
        (*co).co_localspluskinds = Py_NewRef!(unsafe { (*con).localspluskinds });
    }
    unsafe {
        (*co).co_argcount = (*con).argcount;
    }
    unsafe {
        (*co).co_posonlyargcount = (*con).posonlyargcount;
    }
    unsafe {
        (*co).co_kwonlyargcount = (*con).kwonlyargcount;
    }
    unsafe {
        (*co).co_stacksize = (*con).stacksize;
    }
    unsafe {
        (*co).co_exceptiontable = Py_NewRef!(unsafe { (*con).exceptiontable });
    }
    // derived values
    unsafe {
        (*co).co_nlocalsplus = nlocalsplus;
    }
    unsafe {
        (*co).co_nlocals = nlocals;
    }
    unsafe {
        (*co).co_framesize = nlocalsplus.wrapping_add((*con).stacksize).wrapping_add(
            ((core::mem::size_of::<_PyInterpreterFrame>() as u64).wrapping_sub(1)
                / (core::mem::size_of::<*mut _object>() as u64)) as i32,
        );
    }
    unsafe {
        (*co).co_ncellvars = ncellvars;
    }
    unsafe {
        (*co).co_nfreevars = nfreevars;
    }
    unsafe {
        (*co).co_version = (*interp).func_state.next_version;
    }
    if (unsafe { (*interp).func_state.next_version }) != 0 {
        let __v1: u32 = unsafe { (*interp).func_state.next_version };
        unsafe {
            (*interp).func_state.next_version = __v1.wrapping_add(1);
        }
    }
    unsafe {
        (*co)._co_monitoring = core::ptr::null_mut::<_PyCoMonitoringData>();
    }
    unsafe {
        (*co)._co_instrumentation_version = 0;
    }
    // not set
    unsafe {
        (*co).co_weakreflist = core::ptr::null_mut::<_object>();
    }
    unsafe {
        (*co).co_extra = core::ptr::null_mut::<()>();
    }
    unsafe {
        (*co)._co_cached = core::ptr::null_mut::<__SlateRecord_code_h_16>();
    }
    unsafe {
        (*co).co_executors = core::ptr::null_mut::<__SlateRecord_code_h_22>();
    }
    unsafe {
        core::ptr::copy_nonoverlapping(
            ((PyBytes_AS_STRING!(unsafe { (*con).code }) as *const ()) as *const core::ffi::c_void)
                as *const u8,
            ((_PyCode_CODE!(co) as *mut ()) as *mut core::ffi::c_void) as *mut u8,
            ((PyBytes_GET_SIZE!(unsafe { (*con).code }) as u64) as usize) as usize,
        )
    };
    let mut entry_point: i32 = 0;
    loop {
        let __v5: bool = (entry_point as i64) < Py_SIZE!(co)
            && ((unsafe {
                (*_PyCode_CODE!(co).wrapping_offset(entry_point as isize))
                    .op
                    .code
            }) as i32)
                != 128;
        if !__v5 {
            break;
        }
        entry_point = entry_point.wrapping_add(1);
    }
    unsafe {
        (*co)._co_firsttraceable = entry_point;
    }
    unsafe { _PyCode_Quicken(_PyCode_CODE!(co), Py_SIZE!(co), 1i32) };
    notify_code_watchers(0u32, co);
    0
}

fn scan_varint(mut ptr: *const u8) -> i32 {
    let mut read: u32 = 0;
    let __v0: *const u8 = ptr;
    ptr = __v0.wrapping_offset(1 as isize);
    read = (unsafe { *__v0 }) as u32;
    let mut val: u32 = read & 63;
    let mut shift: u32 = 0;
    while read & 64 != 0 {
        let __v2: *const u8 = ptr;
        ptr = __v2.wrapping_offset(1 as isize);
        read = (unsafe { *__v2 }) as u32;
        shift = shift.wrapping_add(6);
        val |= (read & 63) << shift;
    }
    val as i32
}

fn scan_signed_varint(ptr: *const u8) -> i32 {
    let uval: u32 = scan_varint(ptr) as u32;
    if uval & 1 != 0 {
        return ((uval >> 1i32) as i32).wrapping_neg();
    } else {
        return (uval >> 1i32) as i32;
    }
}

fn get_line_delta(ptr: *const u8) -> i32 {
    let code: i32 = ((unsafe { *ptr }) as i32) >> 3i32 & 15;
    match code {
        15 => {
            return 0;
        }
        13 | 14 => {
            return scan_signed_varint(ptr.wrapping_offset(1 as isize));
        }
        10 => {
            return 0;
        }
        11 => {
            return 1;
        }
        12 => {
            return 2;
        }
        _ => {
            return 0; // Same line
        }
    }
    unsafe { core::mem::zeroed() }
}

fn remove_column_info(locations: *mut _object) -> *mut _object {
    let mut offset: i64 = 0;
    let data: *const u8 = PyBytes_AS_STRING!(locations) as *const u8;
    let mut res: *mut _object =
        unsafe { PyBytes_FromStringAndSize(core::ptr::null::<i8>(), 32i64) };
    if res.is_null() {
        unsafe { PyErr_NoMemory() };
        return core::ptr::null_mut::<_object>();
    }
    let mut output: *mut u8 = PyBytes_AS_STRING!(res) as *mut u8;
    while offset < PyBytes_GET_SIZE!(locations) {
        let write_offset: i64 =
            (unsafe { output.offset_from(PyBytes_AS_STRING!(res) as *mut u8) }) as i64;
        if write_offset.wrapping_add(16) >= PyBytes_GET_SIZE!(res) {
            if (unsafe {
                _PyBytes_Resize(
                    core::ptr::addr_of_mut!(res),
                    PyBytes_GET_SIZE!(res).wrapping_mul(2),
                )
            }) < 0
            {
                return core::ptr::null_mut::<_object>();
            }
            output = (PyBytes_AS_STRING!(res) as *mut u8).wrapping_offset(write_offset as isize);
        }
        let code: i32 = ((unsafe { *data.wrapping_offset(offset as isize) }) as i32) >> 3i32 & 15;
        if code == 15 {
            let __v0: *mut u8 = output;
            output = __v0.wrapping_offset(1 as isize);
            unsafe {
                *__v0 = *data.wrapping_offset(offset as isize);
            }
        } else {
            let blength: i32 =
                (((unsafe { *data.wrapping_offset(offset as isize) }) as i32) & 7).wrapping_add(1);
            let __v2: i32 = write_location_entry_start(output, 13i32, blength);
            let __v3: *mut u8 = output;
            output = __v3.wrapping_offset(__v2 as isize);
            let ldelta: i32 = get_line_delta(data.wrapping_offset(offset as isize));
            let __v5: i32 = write_signed_varint(output, ldelta);
            let __v6: *mut u8 = output;
            output = __v6.wrapping_offset(__v5 as isize);
        }
        offset = offset.wrapping_add(1);
        while offset < PyBytes_GET_SIZE!(locations)
            && ((unsafe { *data.wrapping_offset(offset as isize) }) as i32) & 128 == 0
        {
            offset = offset.wrapping_add(1);
        }
    }
    let write_offset: i64 =
        (unsafe { output.offset_from(PyBytes_AS_STRING!(res) as *mut u8) }) as i64;
    if (unsafe { _PyBytes_Resize(core::ptr::addr_of_mut!(res), write_offset) }) != 0 {
        return core::ptr::null_mut::<_object>();
    }
    res
}

fn intern_code_constants(con: *mut _PyCodeConstructor) -> i32 {
    unsafe {
        if intern_strings((*con).names) >= 0 {
            if intern_constants((*con).consts, core::ptr::null_mut::<i32>()) >= 0 {
                if intern_strings((*con).localsplusnames) >= 0 {
                    return 0;
                }
            }
        }
        return 1i32.wrapping_neg();
    }
}

// The caller is responsible for ensuring that the given data is valid.
#[unsafe(no_mangle)]
extern "C-unwind" fn _PyCode_New(con: *mut _PyCodeConstructor) -> *mut PyCodeObject {
    if intern_code_constants(con) < 0 {
        return core::ptr::null_mut::<PyCodeObject>();
    }
    let mut replacement_locations: *mut _object = core::ptr::null_mut::<_object>();
    // Compact the linetable if we are opted out of debug
    //
    // ranges.
    if (unsafe { (*_Py_GetConfig()).code_debug_ranges }) == 0 {
        replacement_locations = remove_column_info(unsafe { (*con).linetable });
        if replacement_locations.is_null() {
            return core::ptr::null_mut::<PyCodeObject>();
        }
        unsafe {
            (*con).linetable = replacement_locations;
        }
    }
    let size: i64 = ((PyBytes_GET_SIZE!(unsafe { (*con).code }) as u64)
        / (core::mem::size_of::<__SlateRecord_pycore_structs_h_32>() as u64))
        as i64;
    let co: *mut PyCodeObject = PyObject_NewVar!(
        PyCodeObject,
        unsafe { core::ptr::addr_of_mut!(PyCode_Type) },
        size
    );
    if co.is_null() {
        Py_XDECREF!(replacement_locations);
        unsafe { PyErr_NoMemory() };
        return core::ptr::null_mut::<PyCodeObject>();
    }
    if init_code(co, con) < 0 {
        Py_DECREF__2!(co);
        return core::ptr::null_mut::<PyCodeObject>();
    }
    Py_XDECREF!(replacement_locations);
    co
}

// the legacy "constructors"
#[unsafe(no_mangle)]
extern "C-unwind" fn PyUnstable_Code_NewWithPosOnlyArgs(
    argcount: i32,
    posonlyargcount: i32,
    kwonlyargcount: i32,
    nlocals: i32,
    stacksize: i32,
    flags: i32,
    code: *mut _object,
    consts: *mut _object,
    names: *mut _object,
    varnames: *mut _object,
    freevars: *mut _object,
    cellvars: *mut _object,
    filename: *mut _object,
    name: *mut _object,
    qualname: *mut _object,
    firstlineno: i32,
    linetable__0: *mut _object,
    exceptiontable: *mut _object,
) -> *mut PyCodeObject {
    let mut con: core::mem::MaybeUninit<_PyCodeConstructor> = core::mem::MaybeUninit::uninit();
    let __slate_slot_0: *mut _PyCodeConstructor =
        core::ptr::addr_of_mut!(con) as *mut _PyCodeConstructor;
    let mut __v1: bool = unsafe { core::mem::zeroed() };
    let mut oparg: i32 = unsafe { core::mem::zeroed() };
    let mut opcode: u8 = unsafe { core::mem::zeroed() };
    let mut instr: *mut __SlateRecord_pycore_structs_h_32 = unsafe { core::mem::zeroed() };
    let mut i: i32 = unsafe { core::mem::zeroed() };
    let mut extended_arg: i32 = unsafe { core::mem::zeroed() };
    let mut num_code_units: i64 = unsafe { core::mem::zeroed() };
    let mut code_data: *mut __SlateRecord_pycore_structs_h_32 = unsafe { core::mem::zeroed() };
    let mut i_1: i32 = unsafe { core::mem::zeroed() };
    let mut j: i32 = unsafe { core::mem::zeroed() };
    let mut argoffset: i32 = unsafe { core::mem::zeroed() };
    let mut name__1_1: *mut _object = unsafe { core::mem::zeroed() };
    let mut i_2: i32 = unsafe { core::mem::zeroed() };
    let mut i_3: i32 = unsafe { core::mem::zeroed() };
    let mut offset: i32 = unsafe { core::mem::zeroed() };
    let mut nlocalsplus: i32 = unsafe { core::mem::zeroed() };
    let mut nfreevars: i32 = unsafe { core::mem::zeroed() };
    let mut ncellvars: i32 = unsafe { core::mem::zeroed() };
    // Set the "fast locals plus" info.
    let mut nvarnames: i32 = unsafe { core::mem::zeroed() };
    let mut __v46: bool = unsafe { core::mem::zeroed() };
    let mut __v47: bool = unsafe { core::mem::zeroed() };
    let mut __v48: bool = unsafe { core::mem::zeroed() };
    let mut localspluskinds: *mut _object = unsafe { core::mem::zeroed() };
    let mut localsplusnames: *mut _object = unsafe { core::mem::zeroed() };
    let mut co: *mut PyCodeObject = unsafe { core::mem::zeroed() };
    unsafe {
        co = core::ptr::null_mut::<PyCodeObject>();
        localsplusnames = core::ptr::null_mut::<_object>();
        localspluskinds = core::ptr::null_mut::<_object>();
        if varnames.is_null() {
            __v48 = true;
        } else {
            __v48 = PyTuple_Check!(varnames) == 0;
        }
        if __v48 || cellvars.is_null() {
            __v47 = true;
        } else {
            __v47 = PyTuple_Check!(cellvars) == 0;
        }
        if __v47 || freevars.is_null() {
            __v46 = true;
        } else {
            __v46 = PyTuple_Check!(freevars) == 0;
        }
        if __v46 {
            PyErr_BadInternalCall__19!();
            return core::ptr::null_mut::<PyCodeObject>();
        } else {
            '__join_0: {
                nvarnames = PyTuple_GET_SIZE!(varnames) as i32;
                ncellvars = PyTuple_GET_SIZE!(cellvars) as i32;
                nfreevars = PyTuple_GET_SIZE!(freevars) as i32;
                nlocalsplus = nvarnames.wrapping_add(ncellvars).wrapping_add(nfreevars);
                localsplusnames = PyTuple_New(nlocalsplus as i64);
                if !localsplusnames.is_null() {
                    localspluskinds =
                        PyBytes_FromStringAndSize(core::ptr::null::<i8>(), nlocalsplus as i64);
                    if !localspluskinds.is_null() {
                        offset = 0;
                        i_3 = 0;
                        while i_3 < nvarnames {
                            let name__1: *mut _object = PyTuple_GET_ITEM!(varnames, i_3);
                            _Py_set_localsplus_info(
                                offset,
                                name__1,
                                32u8,
                                localsplusnames,
                                localspluskinds,
                            );
                            i_3 = i_3.wrapping_add(1);
                            offset = offset.wrapping_add(1);
                        }
                        i_2 = 0;
                        while i_2 < ncellvars {
                            name__1_1 = PyTuple_GET_ITEM!(cellvars, i_2);
                            argoffset = 1i32.wrapping_neg();
                            j = 0;
                            '__join_28: {
                                loop {
                                    if j < nvarnames {
                                        let cmp: i32 = PyUnicode_Compare(
                                            PyTuple_GET_ITEM!(varnames, j),
                                            name__1_1,
                                        );
                                        if cmp == 0 {
                                            break;
                                        } else {
                                            j = j.wrapping_add(1);
                                        }
                                    } else {
                                        break '__join_28;
                                    }
                                }
                                argoffset = j;
                            }
                            if argoffset >= 0 {
                                // Merge the localsplus indices.
                                nlocalsplus = nlocalsplus.wrapping_sub(1);
                                offset = offset.wrapping_sub(1);
                                let kind: u8 = _PyLocals_GetKind(localspluskinds, argoffset);
                                _PyLocals_SetKind(
                                    localspluskinds,
                                    argoffset,
                                    ((kind as i32) | 64) as u8,
                                );
                            } else {
                                _Py_set_localsplus_info(
                                    offset,
                                    name__1_1,
                                    64u8,
                                    localsplusnames,
                                    localspluskinds,
                                );
                            }
                            i_2 = i_2.wrapping_add(1);
                            offset = offset.wrapping_add(1);
                        }
                        i_1 = 0;
                        while i_1 < nfreevars {
                            let name__1: *mut _object = PyTuple_GET_ITEM!(freevars, i_1);
                            _Py_set_localsplus_info(
                                offset,
                                name__1,
                                128u8,
                                localsplusnames,
                                localspluskinds,
                            );
                            i_1 = i_1.wrapping_add(1);
                            offset = offset.wrapping_add(1);
                        }
                        '__join_9: {
                            // gh-110543: Make sure the CO_FAST_HIDDEN flag is set correctly.
                            if flags & 1 == 0 {
                                let code_len: i64 = PyBytes_GET_SIZE!(code);
                                code_data = PyBytes_AS_STRING!(code)
                                    as *mut __SlateRecord_pycore_structs_h_32;
                                num_code_units = ((code_len as u64)
                                    / (core::mem::size_of::<__SlateRecord_pycore_structs_h_32>()
                                        as u64))
                                    as i64;
                                extended_arg = 0;
                                i = 0;
                                '__loop_10: loop {
                                    if (i as i64) < num_code_units {
                                        instr = code_data.wrapping_offset(i as isize);
                                        opcode = (*instr).op.code;
                                        if (opcode as i32) == 69 {
                                            extended_arg =
                                                extended_arg << 8i32 | ((*instr).op.arg as i32);
                                        } else {
                                            '__join_12: {
                                                if (opcode as i32) == 85 {
                                                    oparg = extended_arg << 8i32
                                                        | ((*instr).op.arg as i32);
                                                    if oparg >= nlocalsplus {
                                                        break '__loop_10;
                                                    } else {
                                                        let kind: u8 = _PyLocals_GetKind(
                                                            localspluskinds,
                                                            oparg,
                                                        );
                                                        _PyLocals_SetKind(
                                                            localspluskinds,
                                                            oparg,
                                                            ((kind as i32) | 16) as u8,
                                                        );
                                                        break '__join_12;
                                                    }
                                                }
                                            }
                                            extended_arg = 0;
                                        }
                                        i = i.wrapping_add(1i32.wrapping_add(
                                            _PyOpcode_Caches
                                                [((*code_data.wrapping_offset(i as isize)).op.code
                                                    as i32)
                                                    as usize]
                                                as i32,
                                        ));
                                    } else {
                                        break '__join_9;
                                    }
                                }
                                PyErr_Format(
                                    PyExc_ValueError,
                                    c"code: LOAD_FAST_AND_CLEAR oparg %d out of range".as_ptr(),
                                    oparg,
                                );
                                break '__join_0;
                            }
                        }
                        // If any cells were args then nlocalsplus will have shrunk.
                        if (nlocalsplus as i64) != PyTuple_GET_SIZE!(localsplusnames) {
                            if _PyTuple_Resize(
                                core::ptr::addr_of_mut!(localsplusnames),
                                nlocalsplus as i64,
                            ) < 0
                            {
                                __v1 = true;
                            } else {
                                __v1 = _PyBytes_Resize(
                                    core::ptr::addr_of_mut!(localspluskinds),
                                    nlocalsplus as i64,
                                ) < 0;
                            }
                            if __v1 {
                                break '__join_0;
                            }
                        }
                        core::ptr::write(__slate_slot_0, {
                            let mut __t0: _PyCodeConstructor = core::mem::zeroed();
                            __t0.filename = filename;
                            __t0.name = name;
                            __t0.qualname = qualname;
                            __t0.flags = flags;
                            __t0.code = code;
                            __t0.firstlineno = firstlineno;
                            __t0.linetable = linetable__0;
                            __t0.consts = consts;
                            __t0.names = names;
                            __t0.localsplusnames = localsplusnames;
                            __t0.localspluskinds = localspluskinds;
                            __t0.argcount = argcount;
                            __t0.posonlyargcount = posonlyargcount;
                            __t0.kwonlyargcount = kwonlyargcount;
                            __t0.stacksize = stacksize;
                            __t0.exceptiontable = exceptiontable;
                            __t0
                        });
                        if _PyCode_Validate(core::ptr::addr_of_mut!(*__slate_slot_0)) >= 0 {
                            if (nlocals as i64) != PyTuple_GET_SIZE!(varnames) {
                                PyErr_SetString(
                                    PyExc_ValueError,
                                    c"code: co_nlocals != len(co_varnames)".as_ptr(),
                                );
                            } else {
                                co = _PyCode_New(core::ptr::addr_of_mut!(*__slate_slot_0));
                                if co.is_null() {}
                            }
                        }
                    }
                }
            }
            Py_XDECREF!(localsplusnames);
            Py_XDECREF!(localspluskinds);
            return co;
        }
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyUnstable_Code_New(
    argcount: i32,
    kwonlyargcount: i32,
    nlocals: i32,
    stacksize: i32,
    flags: i32,
    code: *mut _object,
    consts: *mut _object,
    names: *mut _object,
    varnames: *mut _object,
    freevars: *mut _object,
    cellvars: *mut _object,
    filename: *mut _object,
    name: *mut _object,
    qualname: *mut _object,
    firstlineno: i32,
    linetable__0: *mut _object,
    exceptiontable: *mut _object,
) -> *mut PyCodeObject {
    PyCode_NewWithPosOnlyArgs(
        argcount,
        0i32,
        kwonlyargcount,
        nlocals,
        stacksize,
        flags,
        code,
        consts,
        names,
        varnames,
        freevars,
        cellvars,
        filename,
        name,
        qualname,
        firstlineno,
        linetable__0,
        exceptiontable,
    )
}

// NOTE: When modifying the construction of PyCode_NewEmpty, please also change
//
// test.test_code.CodeLocationTest.test_code_new_empty to keep it in sync!
static assert0: [u8; 6] = [128, 0, 81, 0, 104, 1];

/// New entry.
/// Three code units.
/// Offset from co_firstlineno.
static linetable: [u8; 2] = [234, 0];

#[unsafe(no_mangle)]
extern "C-unwind" fn PyCode_NewEmpty(
    filename: *const i8,
    funcname: *const i8,
    firstlineno: i32,
) -> *mut PyCodeObject {
    let mut con: core::mem::MaybeUninit<_PyCodeConstructor> = core::mem::MaybeUninit::uninit();
    let __slate_slot_0: *mut _PyCodeConstructor =
        core::ptr::addr_of_mut!(con) as *mut _PyCodeConstructor;
    let mut result: *mut PyCodeObject = unsafe { core::mem::zeroed() };
    let mut linetable_ob: *mut _object = unsafe { core::mem::zeroed() };
    let mut code_ob: *mut _object = unsafe { core::mem::zeroed() };
    let mut funcname_ob: *mut _object = unsafe { core::mem::zeroed() };
    let mut filename_ob: *mut _object = unsafe { core::mem::zeroed() };
    let mut nulltuple: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        nulltuple = core::ptr::null_mut::<_object>();
        filename_ob = core::ptr::null_mut::<_object>();
        funcname_ob = core::ptr::null_mut::<_object>();
        code_ob = core::ptr::null_mut::<_object>();
        linetable_ob = core::ptr::null_mut::<_object>();
        result = core::ptr::null_mut::<PyCodeObject>();
        nulltuple = PyTuple_New(0i64);
        if !nulltuple.is_null() {
            funcname_ob = PyUnicode_FromString(funcname);
            if !funcname_ob.is_null() {
                filename_ob = PyUnicode_DecodeFSDefault(filename);
                if !filename_ob.is_null() {
                    code_ob = PyBytes_FromStringAndSize(
                        (core::ptr::addr_of!(assert0) as *const u8) as *const i8,
                        6i64,
                    );
                    if !code_ob.is_null() {
                        linetable_ob = PyBytes_FromStringAndSize(
                            (core::ptr::addr_of!(linetable) as *const u8) as *const i8,
                            2i64,
                        );
                        if !linetable_ob.is_null() {
                            core::ptr::write(__slate_slot_0, {
                                let mut __t0: _PyCodeConstructor = core::mem::zeroed();
                                __t0.filename = filename_ob;
                                __t0.name = funcname_ob;
                                __t0.qualname = funcname_ob;
                                __t0.code = code_ob;
                                __t0.firstlineno = firstlineno;
                                __t0.linetable = linetable_ob;
                                __t0.consts = nulltuple;
                                __t0.names = nulltuple;
                                __t0.localsplusnames = nulltuple;
                                __t0.localspluskinds = core::ptr::addr_of_mut!(
                                    _PyRuntime.static_objects.singletons.bytes_empty
                                )
                                    as *mut _object;
                                __t0.stacksize = 1i32;
                                __t0.exceptiontable = core::ptr::addr_of_mut!(
                                    _PyRuntime.static_objects.singletons.bytes_empty
                                )
                                    as *mut _object;
                                __t0
                            });
                            result = _PyCode_New(core::ptr::addr_of_mut!(*__slate_slot_0));
                        }
                    }
                }
            }
        }
        Py_XDECREF!(nulltuple);
        Py_XDECREF!(funcname_ob);
        Py_XDECREF!(filename_ob);
        Py_XDECREF!(code_ob);
        Py_XDECREF!(linetable_ob);
        return result;
    }
}

// source location tracking (co_lines/co_positions)
#[unsafe(no_mangle)]
extern "C-unwind" fn PyCode_Addr2Line(co: *mut PyCodeObject, addrq: i32) -> i32 {
    if addrq < 0 {
        return unsafe { (*co).co_firstlineno };
    }
    let data: *mut _PyCoMonitoringData = _Py_atomic_load_ptr_acquire(
        (unsafe { core::ptr::addr_of_mut!((*co)._co_monitoring) }) as *const (),
    ) as *mut _PyCoMonitoringData;
    if !data.is_null() {
        let lines: *mut __SlateRecord_pycore_instruments_h_94 = _Py_atomic_load_ptr_acquire(
            (unsafe { core::ptr::addr_of_mut!((*data).lines) }) as *const (),
        )
            as *mut __SlateRecord_pycore_instruments_h_94;
        if !lines.is_null() {
            return unsafe {
                _Py_Instrumentation_GetLine(
                    co,
                    lines,
                    ((addrq as u64)
                        / (core::mem::size_of::<__SlateRecord_pycore_structs_h_32>() as u64))
                        as i32,
                )
            };
        }
    }
    let mut bounds: _line_offsets = unsafe { core::mem::zeroed() };
    _PyCode_InitAddressRange(co, core::ptr::addr_of_mut!(bounds));
    _PyCode_CheckLineNumber(addrq, core::ptr::addr_of_mut!(bounds))
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyCode_SafeAddr2Line(co: *mut PyCodeObject, addrq: i32) -> i32 {
    if addrq < 0 {
        return unsafe { (*co).co_firstlineno };
    }
    if !unsafe { (*co)._co_monitoring }.is_null()
        && !unsafe { (*(*co)._co_monitoring).lines }.is_null()
    {
        return unsafe {
            _Py_Instrumentation_GetLine(
                co,
                (*(*co)._co_monitoring).lines,
                ((addrq as u64)
                    / (core::mem::size_of::<__SlateRecord_pycore_structs_h_32>() as u64))
                    as i32,
            )
        };
    }
    let __v0: bool = addrq >= 0 && (addrq as i64) < _PyCode_NBYTES!(co);
    if !__v0 {
        return 1i32.wrapping_neg();
    }
    let mut bounds: _line_offsets = unsafe { core::mem::zeroed() };
    _PyCode_InitAddressRange(co, core::ptr::addr_of_mut!(bounds));
    _PyCode_CheckLineNumber(addrq, core::ptr::addr_of_mut!(bounds))
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyLineTable_InitAddressRange(
    linetable__0: *const i8,
    length: i64,
    firstlineno: i32,
    range: *mut _line_offsets,
) {
    unsafe {
        (*range).opaque.lo_next = linetable__0 as *const u8;
    }
    unsafe {
        (*range).opaque.limit = unsafe { (*range).opaque.lo_next }.wrapping_offset(length as isize);
    }
    unsafe {
        (*range).ar_start = 1i32.wrapping_neg();
    }
    unsafe {
        (*range).ar_end = 0;
    }
    unsafe {
        (*range).opaque.computed_line = firstlineno;
    }
    unsafe {
        (*range).ar_line = 1i32.wrapping_neg();
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyCode_InitAddressRange(
    co: *mut PyCodeObject,
    bounds: *mut _line_offsets,
) -> i32 {
    let linetable__0: *const i8 = PyBytes_AS_STRING!(unsafe { (*co).co_linetable }) as *const i8;
    let length: i64 = PyBytes_GET_SIZE!(unsafe { (*co).co_linetable });
    _PyLineTable_InitAddressRange(
        linetable__0,
        length,
        unsafe { (*co).co_firstlineno },
        bounds,
    );
    unsafe { (*bounds).ar_line }
}

/// Update *bounds to describe the first and one-past-the-last instructions in
/// the same line as lasti.  Return the number of that line, or -1 if lasti is out of bounds.
#[unsafe(no_mangle)]
extern "C-unwind" fn _PyCode_CheckLineNumber(lasti: i32, bounds: *mut _line_offsets) -> i32 {
    while (unsafe { (*bounds).ar_end }) <= lasti {
        if _PyLineTable_NextAddressRange(bounds) == 0 {
            return 1i32.wrapping_neg();
        }
    }
    while (unsafe { (*bounds).ar_start }) > lasti {
        if _PyLineTable_PreviousAddressRange(bounds) == 0 {
            return 1i32.wrapping_neg();
        }
    }
    unsafe { (*bounds).ar_line }
}

fn is_no_line_marker(b: u8) -> i32 {
    ((b as i32) >> 3i32 == 31) as i32
}

fn next_code_delta(bounds: *mut _line_offsets) -> i32 {
    ((((unsafe { *(*bounds).opaque.lo_next }) as i32) & 7).wrapping_add(1) as u64)
        .wrapping_mul(core::mem::size_of::<__SlateRecord_pycore_structs_h_32>() as u64) as i32
}

fn previous_code_delta(bounds: *mut _line_offsets) -> i32 {
    if (unsafe { (*bounds).ar_start }) == 0 {
        // If we looking at the first entry, the
        //
        // "previous" entry has an implicit length of 1.
        return 1;
    }
    let mut ptr: *const u8 = unsafe { (*bounds).opaque.lo_next }.wrapping_offset(-(1 as isize));
    while ((unsafe { *ptr }) as i32) & 128 == 0 {
        let __v0: *const u8 = ptr;
        ptr = __v0.wrapping_offset(-(1 as isize));
    }
    ((((unsafe { *ptr }) as i32) & 7).wrapping_add(1) as u64)
        .wrapping_mul(core::mem::size_of::<__SlateRecord_pycore_structs_h_32>() as u64) as i32
}

fn read_byte(__v0: *mut _line_offsets) -> i32 {
    let __v1: *const u8 = unsafe { (*__v0).opaque.lo_next };
    unsafe {
        (*__v0).opaque.lo_next = __v1.wrapping_offset(1 as isize);
    }
    (unsafe { *__v1 }) as i32
}

fn read_varint(bounds: *mut _line_offsets) -> i32 {
    let mut read: u32 = read_byte(bounds) as u32;
    let mut val: u32 = read & 63;
    let mut shift: u32 = 0;
    while read & 64 != 0 {
        read = read_byte(bounds) as u32;
        shift = shift.wrapping_add(6);
        val |= (read & 63) << shift;
    }
    val as i32
}

fn read_signed_varint(bounds: *mut _line_offsets) -> i32 {
    let uval: u32 = read_varint(bounds) as u32;
    if uval & 1 != 0 {
        return ((uval >> 1i32) as i32).wrapping_neg();
    } else {
        return (uval >> 1i32) as i32;
    }
}

fn retreat(bounds: *mut _line_offsets) {
    ASSERT_VALID_BOUNDS!();
    loop {
        let __v1: *const u8 = unsafe { (*bounds).opaque.lo_next };
        unsafe {
            (*bounds).opaque.lo_next = __v1.wrapping_offset(-(1 as isize));
        }
        if ((unsafe { *(*bounds).opaque.lo_next }) as i32) & 128 != 0 {
            break;
        }
    }
    let __v4: i32 = get_line_delta(unsafe { (*bounds).opaque.lo_next });
    let __v6: i32 = unsafe { (*bounds).opaque.computed_line };
    unsafe {
        (*bounds).opaque.computed_line = __v6.wrapping_sub(__v4);
    }
    unsafe {
        (*bounds).ar_end = (*bounds).ar_start;
    }
    let __v8: i32 = previous_code_delta(bounds);
    let __v10: i32 = unsafe { (*bounds).ar_start };
    unsafe {
        (*bounds).ar_start = __v10.wrapping_sub(__v8);
    }
    if is_no_line_marker(unsafe {
        *unsafe { (*bounds).opaque.lo_next }.wrapping_offset(1i32.wrapping_neg() as isize)
    }) != 0
    {
        unsafe {
            (*bounds).ar_line = 1i32.wrapping_neg();
        }
    } else {
        unsafe {
            (*bounds).ar_line = (*bounds).opaque.computed_line;
        }
    }
    ASSERT_VALID_BOUNDS!();
}

fn advance(bounds: *mut _line_offsets) {
    ASSERT_VALID_BOUNDS!();
    let __v0: i32 = get_line_delta(unsafe { (*bounds).opaque.lo_next });
    let __v2: i32 = unsafe { (*bounds).opaque.computed_line };
    unsafe {
        (*bounds).opaque.computed_line = __v2.wrapping_add(__v0);
    }
    if is_no_line_marker(unsafe { *(*bounds).opaque.lo_next }) != 0 {
        unsafe {
            (*bounds).ar_line = 1i32.wrapping_neg();
        }
    } else {
        unsafe {
            (*bounds).ar_line = (*bounds).opaque.computed_line;
        }
    }
    unsafe {
        (*bounds).ar_start = (*bounds).ar_end;
    }
    let __v4: i32 = next_code_delta(bounds);
    let __v6: i32 = unsafe { (*bounds).ar_end };
    unsafe {
        (*bounds).ar_end = __v6.wrapping_add(__v4);
    }
    loop {
        let __v9: *const u8 = unsafe { (*bounds).opaque.lo_next };
        unsafe {
            (*bounds).opaque.lo_next = __v9.wrapping_offset(1 as isize);
        }
        if !((unsafe { (*bounds).opaque.lo_next }) < unsafe { (*bounds).opaque.limit }
            && ((unsafe { *(*bounds).opaque.lo_next }) as i32) & 128 == 0)
        {
            break;
        }
    }
    ASSERT_VALID_BOUNDS!();
}

fn advance_with_locations(
    bounds: *mut _line_offsets,
    endline: *mut i32,
    column: *mut i32,
    endcolumn: *mut i32,
) {
    ASSERT_VALID_BOUNDS!();
    let first_byte: i32 = read_byte(bounds);
    let code: i32 = first_byte >> 3i32 & 15;
    unsafe {
        (*bounds).ar_start = (*bounds).ar_end;
    }
    unsafe {
        (*bounds).ar_end = ((*bounds).ar_start as u64).wrapping_add(
            ((first_byte & 7).wrapping_add(1) as u64)
                .wrapping_mul(core::mem::size_of::<__SlateRecord_pycore_structs_h_32>() as u64),
        ) as i32;
    }
    match code {
        15 => {
            let __v0: i32 = 1i32.wrapping_neg();
            unsafe {
                *endline = __v0;
            }
            unsafe {
                (*bounds).ar_line = __v0;
            }
            let __v1: i32 = 1i32.wrapping_neg();
            unsafe {
                *endcolumn = __v1;
            }
            unsafe {
                *column = __v1;
            }
        }
        14 => {
            let __v3: i32 = read_signed_varint(bounds);
            let __v5: i32 = unsafe { (*bounds).opaque.computed_line };
            unsafe {
                (*bounds).opaque.computed_line = __v5.wrapping_add(__v3);
            }
            unsafe {
                (*bounds).ar_line = (*bounds).opaque.computed_line;
            }
            unsafe {
                *endline = unsafe { (*bounds).ar_line }.wrapping_add(read_varint(bounds));
            }
            unsafe {
                *column = read_varint(bounds).wrapping_sub(1);
            }
            unsafe {
                *endcolumn = read_varint(bounds).wrapping_sub(1);
            }
        }
        13 => {
            // No column
            let __v7: i32 = read_signed_varint(bounds);
            let __v9: i32 = unsafe { (*bounds).opaque.computed_line };
            unsafe {
                (*bounds).opaque.computed_line = __v9.wrapping_add(__v7);
            }
            let __v11: i32 = unsafe { (*bounds).opaque.computed_line };
            unsafe {
                (*bounds).ar_line = __v11;
            }
            unsafe {
                *endline = __v11;
            }
            let __v12: i32 = 1i32.wrapping_neg();
            unsafe {
                *endcolumn = __v12;
            }
            unsafe {
                *column = __v12;
            }
        }
        10..=12 => {
            // one line form
            let line_delta: i32 = code.wrapping_sub(10);
            let __v14: i32 = unsafe { (*bounds).opaque.computed_line };
            unsafe {
                (*bounds).opaque.computed_line = __v14.wrapping_add(line_delta);
            }
            let __v16: i32 = unsafe { (*bounds).opaque.computed_line };
            unsafe {
                (*bounds).ar_line = __v16;
            }
            unsafe {
                *endline = __v16;
            }
            unsafe {
                *column = read_byte(bounds);
            }
            unsafe {
                *endcolumn = read_byte(bounds);
            }
        }
        _ => {
            // Short forms
            let second_byte: i32 = read_byte(bounds);
            let __v17: i32 = unsafe { (*bounds).opaque.computed_line };
            unsafe {
                (*bounds).ar_line = __v17;
            }
            unsafe {
                *endline = __v17;
            }
            unsafe {
                *column = code << 3i32 | second_byte >> 4i32;
            }
            unsafe {
                *endcolumn = unsafe { *column }.wrapping_add(second_byte & 15);
            }
        }
    }
    ASSERT_VALID_BOUNDS!();
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyCode_Addr2Location(
    co: *mut PyCodeObject,
    addrq: i32,
    start_line: *mut i32,
    start_column: *mut i32,
    end_line: *mut i32,
    end_column: *mut i32,
) -> i32 {
    if addrq < 0 {
        let __v0: i32 = unsafe { (*co).co_firstlineno };
        unsafe {
            *end_line = __v0;
        }
        unsafe {
            *start_line = __v0;
        }
        unsafe {
            *end_column = 0;
        }
        unsafe {
            *start_column = 0;
        }
        return 1;
    }
    let mut bounds: _line_offsets = unsafe { core::mem::zeroed() };
    _PyCode_InitAddressRange(co, core::ptr::addr_of_mut!(bounds));
    _PyCode_CheckLineNumber(addrq, core::ptr::addr_of_mut!(bounds));
    retreat(core::ptr::addr_of_mut!(bounds));
    advance_with_locations(
        core::ptr::addr_of_mut!(bounds),
        end_line,
        start_column,
        end_column,
    );
    unsafe {
        *start_line = bounds.ar_line;
    }
    1
}

fn at_end(bounds: *mut _line_offsets) -> i32 {
    ((unsafe { (*bounds).opaque.lo_next }) >= unsafe { (*bounds).opaque.limit }) as i32
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyLineTable_PreviousAddressRange(range: *mut _line_offsets) -> i32 {
    if (unsafe { (*range).ar_start }) <= 0 {
        return 0;
    }
    retreat(range);
    1
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyLineTable_NextAddressRange(range: *mut _line_offsets) -> i32 {
    if at_end(range) != 0 {
        return 0;
    }
    advance(range);
    1
}

fn emit_pair(bytes: *mut *mut _object, offset: &mut i32, a: i32, b: i32) -> i32 {
    let len: i64 = PyBytes_GET_SIZE!(unsafe { *bytes });
    if (unsafe { *(offset as *mut i32) }.wrapping_add(2) as i64) >= len {
        if (unsafe { _PyBytes_Resize(bytes, len.wrapping_mul(2)) }) < 0 {
            return 0;
        }
    }
    let mut lnotab: *mut u8 = PyBytes_AS_STRING!(unsafe { *bytes }) as *mut u8;
    let __v0: *mut u8 = lnotab;
    lnotab = __v0.wrapping_offset((unsafe { *(offset as *mut i32) }) as isize);
    let __v2: *mut u8 = lnotab;
    lnotab = __v2.wrapping_offset(1 as isize);
    unsafe {
        *__v2 = a as u8;
    }
    let __v4: *mut u8 = lnotab;
    lnotab = __v4.wrapping_offset(1 as isize);
    unsafe {
        *__v4 = b as u8;
    }
    let __v7: i32 = unsafe { *offset };
    unsafe {
        *offset = __v7.wrapping_add(2);
    }
    1
}

fn emit_delta(bytes: *mut *mut _object, mut bdelta: i32, mut ldelta: i32, offset: &mut i32) -> i32 {
    while bdelta > 255 {
        if emit_pair(bytes, offset, 255i32, 0i32) == 0 {
            return 0;
        }
        bdelta = bdelta.wrapping_sub(255);
    }
    while ldelta > 127 {
        if emit_pair(bytes, offset, bdelta, 127i32) == 0 {
            return 0;
        }
        bdelta = 0;
        ldelta = ldelta.wrapping_sub(127);
    }
    while ldelta < 128i32.wrapping_neg() {
        if emit_pair(bytes, offset, bdelta, 128i32.wrapping_neg()) == 0 {
            return 0;
        }
        bdelta = 0;
        ldelta = ldelta.wrapping_add(128);
    }
    emit_pair(bytes, offset, bdelta, ldelta)
}

fn decode_linetable(code: *mut PyCodeObject) -> *mut _object {
    let mut bounds: _line_offsets = unsafe { core::mem::zeroed() };
    let mut bytes: *mut _object = unsafe { core::mem::zeroed() };
    let mut table_offset: i32 = 0;
    let mut code_offset: i32 = 0;
    let mut line: i32 = unsafe { (*code).co_firstlineno };
    bytes = unsafe { PyBytes_FromStringAndSize(core::ptr::null::<i8>(), 64i64) };
    if bytes.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    _PyCode_InitAddressRange(code, core::ptr::addr_of_mut!(bounds));
    while _PyLineTable_NextAddressRange(core::ptr::addr_of_mut!(bounds)) != 0 {
        if bounds.opaque.computed_line != line {
            let bdelta: i32 = bounds.ar_start.wrapping_sub(code_offset);
            let ldelta: i32 = bounds.opaque.computed_line.wrapping_sub(line);
            if emit_delta(core::ptr::addr_of_mut!(bytes), bdelta, ldelta, unsafe {
                &mut (*core::ptr::addr_of_mut!(table_offset))
            }) == 0
            {
                Py_DECREF!(bytes);
                return core::ptr::null_mut::<_object>();
            }
            code_offset = bounds.ar_start;
            line = bounds.opaque.computed_line;
        }
    }
    unsafe { _PyBytes_Resize(core::ptr::addr_of_mut!(bytes), table_offset as i64) };
    bytes
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_codeobject_c_1380 {
    ob_base: _object,
    li_code: *mut PyCodeObject,
    li_line: _line_offsets,
}

#[unsafe(link_section = ".text.slate_distinct.codeobject.lineiter_dealloc")]
extern "C-unwind" fn lineiter_dealloc(self_: *mut _object) {
    let li: *mut __SlateRecord_codeobject_c_1380 = self_ as *mut __SlateRecord_codeobject_c_1380;
    Py_DECREF__2!(unsafe { (*li).li_code });
    unsafe { unsafe { (*Py_TYPE__2!(li)).tp_free }.unwrap()(li as *mut ()) };
}

#[unsafe(link_section = ".text.slate_distinct.codeobject._source_offset_converter")]
extern "C-unwind" fn _source_offset_converter(arg: *mut ()) -> *mut _object {
    if (unsafe { *(arg as *mut i32) }) == 1i32.wrapping_neg() {
        return unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) };
    }
    unsafe { PyLong_FromLong(*(arg as *mut i32) as i64) }
}

#[unsafe(link_section = ".text.slate_distinct.codeobject.lineiter_next")]
extern "C-unwind" fn lineiter_next(self_: *mut _object) -> *mut _object {
    let bounds: *mut _line_offsets = unsafe {
        core::ptr::addr_of_mut!((*(self_ as *mut __SlateRecord_codeobject_c_1380)).li_line)
    };
    if _PyLineTable_NextAddressRange(bounds) == 0 {
        return core::ptr::null_mut::<_object>();
    }
    let start: i32 = unsafe { (*bounds).ar_start };
    let mut line: i32 = unsafe { (*bounds).ar_line };
    // Merge overlapping entries:
    while _PyLineTable_NextAddressRange(bounds) != 0 {
        if (unsafe { (*bounds).ar_line }) != line {
            _PyLineTable_PreviousAddressRange(bounds);
            break;
        }
    }
    unsafe {
        Py_BuildValue(
            c"iiO&".as_ptr(),
            start,
            (*bounds).ar_end,
            unsafe {
                core::mem::transmute::<
                    Option<unsafe extern "C-unwind" fn(*mut ()) -> *mut _object>,
                    *const u8,
                >(Some(_source_offset_converter))
            },
            core::ptr::addr_of_mut!(line),
        )
    }
}

/// tp_name
/// tp_basicsize
/// tp_itemsize
/// methods
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
/// tp_free
#[unsafe(no_mangle)]
static mut _PyLineIterator: _typeobject = {
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
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_codeobject_c_1380>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(lineiter_dealloc);
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
    __t0.tp_flags = 1024u64;
    __t0.tp_doc = core::ptr::null::<i8>();
    __t0.tp_traverse = None;
    __t0.tp_clear = None;
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = unsafe {
        core::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
        >(PyObject_SelfIter as *const ())
    };
    __t0.tp_iternext = Some(lineiter_next);
    __t0.tp_methods = core::ptr::null_mut::<PyMethodDef>();
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = core::ptr::null_mut::<_typeobject>();
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = 0i64;
    __t0.tp_init = None;
    __t0.tp_alloc = None;
    __t0.tp_new = None;
    __t0.tp_free = unsafe {
        core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
            PyObject_Free as *const (),
        )
    };
    __t0
};

fn new_linesiterator(code: *mut PyCodeObject) -> *mut __SlateRecord_codeobject_c_1380 {
    let li: *mut __SlateRecord_codeobject_c_1380 =
        (unsafe { PyType_GenericAlloc(core::ptr::addr_of_mut!(_PyLineIterator), 0i64) })
            as *mut __SlateRecord_codeobject_c_1380;
    if li.is_null() {
        return core::ptr::null_mut::<__SlateRecord_codeobject_c_1380>();
    }
    unsafe {
        (*li).li_code = Py_NewRef__2!(code) as *mut PyCodeObject;
    }
    _PyCode_InitAddressRange(code, unsafe { core::ptr::addr_of_mut!((*li).li_line) });
    li
}

/// co_positions iterator object.
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_codeobject_c_1485 {
    ob_base: _object,
    pi_code: *mut PyCodeObject,
    pi_range: _line_offsets,
    pi_offset: i32,
    pi_endline: i32,
    pi_column: i32,
    pi_endcolumn: i32,
}

#[unsafe(link_section = ".text.slate_distinct.codeobject.positionsiter_dealloc")]
extern "C-unwind" fn positionsiter_dealloc(self_: *mut _object) {
    let pi: *mut __SlateRecord_codeobject_c_1485 = self_ as *mut __SlateRecord_codeobject_c_1485;
    Py_DECREF__2!(unsafe { (*pi).pi_code });
    unsafe { unsafe { (*Py_TYPE__2!(pi)).tp_free }.unwrap()(pi as *mut ()) };
}

#[unsafe(link_section = ".text.slate_distinct.codeobject.positionsiter_next")]
extern "C-unwind" fn positionsiter_next(self_: *mut _object) -> *mut _object {
    if (unsafe { (*(self_ as *mut __SlateRecord_codeobject_c_1485)).pi_offset })
        >= unsafe {
            (*(self_ as *mut __SlateRecord_codeobject_c_1485))
                .pi_range
                .ar_end
        }
    {
        if at_end(unsafe {
            core::ptr::addr_of_mut!((*(self_ as *mut __SlateRecord_codeobject_c_1485)).pi_range)
        }) != 0
        {
            return core::ptr::null_mut::<_object>();
        }
        advance_with_locations(
            unsafe {
                core::ptr::addr_of_mut!((*(self_ as *mut __SlateRecord_codeobject_c_1485)).pi_range)
            },
            unsafe {
                core::ptr::addr_of_mut!(
                    (*(self_ as *mut __SlateRecord_codeobject_c_1485)).pi_endline
                )
            },
            unsafe {
                core::ptr::addr_of_mut!(
                    (*(self_ as *mut __SlateRecord_codeobject_c_1485)).pi_column
                )
            },
            unsafe {
                core::ptr::addr_of_mut!(
                    (*(self_ as *mut __SlateRecord_codeobject_c_1485)).pi_endcolumn
                )
            },
        );
    }
    let __v1: i32 = unsafe { (*(self_ as *mut __SlateRecord_codeobject_c_1485)).pi_offset };
    unsafe {
        (*(self_ as *mut __SlateRecord_codeobject_c_1485)).pi_offset = __v1.wrapping_add(2);
    }
    unsafe {
        Py_BuildValue(
            c"(O&O&O&O&)".as_ptr(),
            unsafe {
                core::mem::transmute::<
                    Option<unsafe extern "C-unwind" fn(*mut ()) -> *mut _object>,
                    *const u8,
                >(Some(_source_offset_converter))
            },
            core::ptr::addr_of_mut!(
                (*(self_ as *mut __SlateRecord_codeobject_c_1485))
                    .pi_range
                    .ar_line
            ),
            unsafe {
                core::mem::transmute::<
                    Option<unsafe extern "C-unwind" fn(*mut ()) -> *mut _object>,
                    *const u8,
                >(Some(_source_offset_converter))
            },
            core::ptr::addr_of_mut!((*(self_ as *mut __SlateRecord_codeobject_c_1485)).pi_endline),
            unsafe {
                core::mem::transmute::<
                    Option<unsafe extern "C-unwind" fn(*mut ()) -> *mut _object>,
                    *const u8,
                >(Some(_source_offset_converter))
            },
            core::ptr::addr_of_mut!((*(self_ as *mut __SlateRecord_codeobject_c_1485)).pi_column),
            unsafe {
                core::mem::transmute::<
                    Option<unsafe extern "C-unwind" fn(*mut ()) -> *mut _object>,
                    *const u8,
                >(Some(_source_offset_converter))
            },
            core::ptr::addr_of_mut!(
                (*(self_ as *mut __SlateRecord_codeobject_c_1485)).pi_endcolumn
            ),
        )
    }
}

/// tp_name
/// tp_basicsize
/// tp_itemsize
/// methods
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
/// tp_free
#[unsafe(no_mangle)]
static mut _PyPositionsIterator: _typeobject = {
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
    __t0.tp_name = (b"positions_iterator\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_codeobject_c_1485>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(positionsiter_dealloc);
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
    __t0.tp_flags = 1024u64;
    __t0.tp_doc = core::ptr::null::<i8>();
    __t0.tp_traverse = None;
    __t0.tp_clear = None;
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = unsafe {
        core::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
        >(PyObject_SelfIter as *const ())
    };
    __t0.tp_iternext = Some(positionsiter_next);
    __t0.tp_methods = core::ptr::null_mut::<PyMethodDef>();
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = core::ptr::null_mut::<_typeobject>();
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = 0i64;
    __t0.tp_init = None;
    __t0.tp_alloc = None;
    __t0.tp_new = None;
    __t0.tp_free = unsafe {
        core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
            PyObject_Free as *const (),
        )
    };
    __t0
};

#[unsafe(link_section = ".text.slate_distinct.codeobject.code_positionsiterator")]
extern "C-unwind" fn code_positionsiterator(
    self_: *mut _object,
    _unused_args: *mut _object,
) -> *mut _object {
    let code: *mut PyCodeObject = self_ as *mut PyCodeObject;
    let pi: *mut __SlateRecord_codeobject_c_1485 =
        (unsafe { PyType_GenericAlloc(core::ptr::addr_of_mut!(_PyPositionsIterator), 0i64) })
            as *mut __SlateRecord_codeobject_c_1485;
    if pi.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    unsafe {
        (*pi).pi_code = Py_NewRef__2!(code) as *mut PyCodeObject;
    }
    _PyCode_InitAddressRange(code, unsafe { core::ptr::addr_of_mut!((*pi).pi_range) });
    unsafe {
        (*pi).pi_offset = (*pi).pi_range.ar_end;
    }
    pi as *mut _object
}

// "extra" frame eval info (see PEP 523)
/// Holder for co_extra information
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_codeobject_c_1580 {
    ce_size: i64,
    ce_extras: [*mut (); 1],
}

fn code_extra_size(n: i64) -> u64 {
    (core::mem::size_of::<__SlateRecord_codeobject_c_1580>() as u64).wrapping_add(
        (n.wrapping_sub(1) as u64).wrapping_mul(core::mem::size_of::<*mut ()>() as u64),
    )
}

fn code_extra_grow_gil(
    co: *mut PyCodeObject,
    old_co_extra: *mut __SlateRecord_codeobject_c_1580,
    old_ce_size: i64,
    new_ce_size: i64,
    index: i64,
    extra: *mut (),
) -> i32 {
    let new_co_extra: *mut __SlateRecord_codeobject_c_1580 =
        (unsafe { PyMem_Realloc(old_co_extra as *mut (), code_extra_size(new_ce_size)) })
            as *mut __SlateRecord_codeobject_c_1580;
    if new_co_extra.is_null() {
        unsafe { PyErr_NoMemory() };
        return 1i32.wrapping_neg();
    }
    for i in old_ce_size..new_ce_size {
        unsafe {
            *((*new_co_extra).ce_extras.as_mut_ptr() as *mut *mut ()).wrapping_offset(i as isize) =
                core::ptr::null_mut::<()>();
        }
    }
    unsafe {
        (*new_co_extra).ce_size = new_ce_size;
    }
    unsafe {
        *((*new_co_extra).ce_extras.as_mut_ptr() as *mut *mut ()).wrapping_offset(index as isize) =
            extra;
    }
    unsafe {
        (*co).co_extra = new_co_extra as *mut ();
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyUnstable_Code_GetExtra(
    code: *mut _object,
    index: i64,
    extra: *mut *mut (),
) -> i32 {
    if PyCode_Check!(code) == 0 {
        PyErr_BadInternalCall__21!();
        return 1i32.wrapping_neg();
    }
    unsafe {
        *extra = core::ptr::null_mut::<()>();
    }
    if index < 0 {
        return 0;
    }
    // Lock-free read; pairs with release stores in SetExtra.
    let co_extra: *mut __SlateRecord_codeobject_c_1580 =
        (unsafe { (*(code as *mut PyCodeObject)).co_extra })
            as *mut __SlateRecord_codeobject_c_1580;
    if !co_extra.is_null() && index < unsafe { (*co_extra).ce_size } {
        unsafe {
            *extra = *((*co_extra).ce_extras.as_mut_ptr() as *mut *mut ())
                .wrapping_offset(index as isize);
        }
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyUnstable_Code_SetExtra(
    code: *mut _object,
    index: i64,
    extra: *mut (),
) -> i32 {
    // Free the old slot value if a free function was registered.
    //
    // The caller must ensure no other thread can still access the old
    //
    // value after this overwrite.
    let mut free_extra: core::mem::MaybeUninit<Option<unsafe extern "C-unwind" fn(*mut ())>> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_0: *mut Option<unsafe extern "C-unwind" fn(*mut ())> =
        core::ptr::addr_of_mut!(free_extra) as *mut Option<unsafe extern "C-unwind" fn(*mut ())>;
    let mut old_ce_size: i64 = unsafe { core::mem::zeroed() };
    let mut old_co_extra: *mut __SlateRecord_codeobject_c_1580 = unsafe { core::mem::zeroed() };
    let mut old_slot_value: *mut () = unsafe { core::mem::zeroed() };
    let mut result: i32 = unsafe { core::mem::zeroed() };
    let mut co: *mut PyCodeObject = unsafe { core::mem::zeroed() };
    // co_extra_user_count is monotonically increasing and published with
    //
    // release store in RequestCodeExtraIndex, so once an index is valid
    //
    // it stays valid.
    let mut user_count: i64 = unsafe { core::mem::zeroed() };
    let mut interp: *mut _is = unsafe { core::mem::zeroed() };
    unsafe {
        interp = _PyInterpreterState_GET();
        user_count = (*interp).co_extra_user_count;
        if PyCode_Check!(code) == 0 || index < 0 || index >= user_count {
            PyErr_BadInternalCall__66!();
            return 1i32.wrapping_neg();
        } else {
            co = code as *mut PyCodeObject;
            result = 0;
            old_slot_value = core::ptr::null_mut::<()>();
            old_co_extra = (*co).co_extra as *mut __SlateRecord_codeobject_c_1580;
            old_ce_size = if old_co_extra.is_null() {
                0
            } else {
                (*old_co_extra).ce_size
            };
            // Fast path: slot already exists, update in place.
            if index < old_ce_size {
                old_slot_value = *((*old_co_extra).ce_extras.as_mut_ptr() as *mut *mut ())
                    .wrapping_offset(index as isize);
                FT_ATOMIC_STORE_PTR_RELEASE!(
                    *((*old_co_extra).ce_extras.as_mut_ptr() as *mut *mut ())
                        .wrapping_offset(index as isize),
                    extra
                );
            } else {
                // Slow path: buffer needs to grow.
                let new_ce_size: i64 = user_count;
                // GIL build: grow with realloc.
                result =
                    code_extra_grow_gil(co, old_co_extra, old_ce_size, new_ce_size, index, extra);
            }
            if !old_slot_value.is_null() {
                core::ptr::write(
                    __slate_slot_0,
                    *((*interp).co_extra_freefuncs.as_mut_ptr()
                        as *mut Option<unsafe extern "C-unwind" fn(*mut ())>)
                        .wrapping_offset(index as isize),
                );
                if *__slate_slot_0 != None {
                    (*__slate_slot_0).unwrap()(old_slot_value);
                }
            }
            return result;
        }
    }
}

// other PyCodeObject accessor functions
fn get_cached_locals(
    co: *mut PyCodeObject,
    cached_field: *mut *mut _object,
    kind: u8,
    num: i32,
) -> *mut _object {
    let mut varnames: *mut _object = unsafe { *cached_field };
    if !varnames.is_null() {
        return Py_NewRef!(varnames);
    }
    varnames = unsafe { *cached_field };
    if varnames.is_null() {
        varnames = get_localsplus_names(co, kind, num);
        if !varnames.is_null() {
            FT_ATOMIC_STORE_PTR!(*cached_field, varnames);
        }
    }
    Py_XNewRef!(varnames)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyCode_GetVarnames(co: *mut PyCodeObject) -> *mut _object {
    if init_co_cached(co) != 0 {
        return core::ptr::null_mut::<_object>();
    }
    get_cached_locals(
        co,
        unsafe { core::ptr::addr_of_mut!((*(*co)._co_cached)._co_varnames) },
        32u8,
        unsafe { (*co).co_nlocals },
    )
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyCode_GetVarnames(code: *mut PyCodeObject) -> *mut _object {
    _PyCode_GetVarnames(code)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyCode_GetCellvars(co: *mut PyCodeObject) -> *mut _object {
    if init_co_cached(co) != 0 {
        return core::ptr::null_mut::<_object>();
    }
    get_cached_locals(
        co,
        unsafe { core::ptr::addr_of_mut!((*(*co)._co_cached)._co_cellvars) },
        64u8,
        unsafe { (*co).co_ncellvars },
    )
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyCode_GetCellvars(code: *mut PyCodeObject) -> *mut _object {
    _PyCode_GetCellvars(code)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyCode_GetFreevars(co: *mut PyCodeObject) -> *mut _object {
    if init_co_cached(co) != 0 {
        return core::ptr::null_mut::<_object>();
    }
    get_cached_locals(
        co,
        unsafe { core::ptr::addr_of_mut!((*(*co)._co_cached)._co_freevars) },
        128u8,
        unsafe { (*co).co_nfreevars },
    )
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyCode_GetFreevars(code: *mut PyCodeObject) -> *mut _object {
    _PyCode_GetFreevars(code)
}

// We may want to move these macros to pycore_opcode_utils.h
//
// and use them in Python/bytecodes.c.
fn identify_unbound_names(
    tstate: *mut _ts,
    co: *mut PyCodeObject,
    globalnames: *mut _object,
    attrnames: *mut _object,
    globalsns: *mut _object,
    builtinsns: *mut _object,
    counts: *mut co_unbound_counts,
    p_numdupes: *mut i32,
) -> i32 {
    // This function is inspired by inspect.getclosurevars().
    //
    // It would be nicer if we had something similar to co_localspluskinds,
    //
    // but for co_names.
    let mut unbound: co_unbound_counts = {
        let mut __t0: co_unbound_counts = unsafe { core::mem::zeroed() };
        __t0.total = 0i32;
        __t0
    };
    let mut numdupes: i32 = 0;
    let len: i64 = Py_SIZE!(co);
    let mut i: i32 = 0;
    while (i as i64) < len {
        let inst: __SlateRecord_pycore_structs_h_32 = unsafe { _Py_GetBaseCodeUnit(co, i) };
        if ((unsafe { inst.op.code }) as i32) == 80 {
            let oparg: i32 = GET_OPARG!(unsafe { inst.op.arg }) as i32;
            let index: i32 = LOAD_ATTR_NAME_INDEX!(oparg);
            let name: *mut _object = GETITEM!(unsafe { (*co).co_names }, index);
            if (unsafe { PySet_Contains(attrnames, name) }) != 0 {
                if !_PyErr_Occurred(tstate).is_null() {
                    return 1i32.wrapping_neg();
                }
            } else {
                let __v1: i32 = unbound.total;
                unbound.total = __v1.wrapping_add(1);
                let __v3: i32 = unbound.numattrs;
                unbound.numattrs = __v3.wrapping_add(1);
                if (unsafe { PySet_Add(attrnames, name) }) < 0 {
                    return 1i32.wrapping_neg();
                }
                if (unsafe { PySet_Contains(globalnames, name) }) != 0 {
                    if !_PyErr_Occurred(tstate).is_null() {
                        return 1i32.wrapping_neg();
                    }
                    numdupes = numdupes.wrapping_add(1);
                }
            }
        } else {
            if ((unsafe { inst.op.code }) as i32) == 92 {
                let oparg: i32 = GET_OPARG!(unsafe { inst.op.arg }) as i32;
                let index: i32 = LOAD_ATTR_NAME_INDEX!(oparg);
                let name: *mut _object = GETITEM!(unsafe { (*co).co_names }, index);
                if (unsafe { PySet_Contains(globalnames, name) }) != 0 {
                    if !_PyErr_Occurred(tstate).is_null() {
                        return 1i32.wrapping_neg();
                    }
                } else {
                    let __v7: i32 = unbound.total;
                    unbound.total = __v7.wrapping_add(1);
                    let __v9: i32 = unbound.globals.total;
                    unbound.globals.total = __v9.wrapping_add(1);
                    let __v11: bool =
                        !globalsns.is_null() && (unsafe { PyDict_Contains(globalsns, name) }) != 0;
                    if __v11 {
                        if !_PyErr_Occurred(tstate).is_null() {
                            return 1i32.wrapping_neg();
                        }
                        let __v12: i32 = unbound.globals.numglobal;
                        unbound.globals.numglobal = __v12.wrapping_add(1);
                    } else {
                        let __v14: bool = !builtinsns.is_null()
                            && (unsafe { PyDict_Contains(builtinsns, name) }) != 0;
                        if __v14 {
                            if !_PyErr_Occurred(tstate).is_null() {
                                return 1i32.wrapping_neg();
                            }
                            let __v15: i32 = unbound.globals.numbuiltin;
                            unbound.globals.numbuiltin = __v15.wrapping_add(1);
                        } else {
                            let __v17: i32 = unbound.globals.numunknown;
                            unbound.globals.numunknown = __v17.wrapping_add(1);
                        }
                    }
                    if (unsafe { PySet_Add(globalnames, name) }) < 0 {
                        return 1i32.wrapping_neg();
                    }
                    if (unsafe { PySet_Contains(attrnames, name) }) != 0 {
                        if !_PyErr_Occurred(tstate).is_null() {
                            return 1i32.wrapping_neg();
                        }
                        numdupes = numdupes.wrapping_add(1);
                    }
                }
            }
        }
        let __v21: i32 = unsafe { _PyInstruction_GetLength(co, i) };
        i = i.wrapping_add(__v21);
    }
    if !counts.is_null() {
        unsafe {
            *counts = unbound;
        }
    }
    if !p_numdupes.is_null() {
        unsafe {
            *p_numdupes = numdupes;
        }
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyCode_GetVarCounts(
    co: *mut PyCodeObject,
    counts: *mut __SlateRecord_pycore_code_h_610,
) {
    let mut __slate_compound_33: __SlateRecord_pycore_code_h_610 = unsafe { core::mem::zeroed() };
    // Count the locals, cells, and free vars.
    let mut locals: co_locals_counts = {
        let mut __t0: co_locals_counts = unsafe { core::mem::zeroed() };
        __t0.total = 0i32;
        __t0
    };
    let mut numfree: i32 = 0;
    let kinds: *mut _object = unsafe { (*co).co_localspluskinds };
    let numlocalplusfree: i64 = PyBytes_GET_SIZE!(kinds);
    let mut i: i32 = 0;
    while (i as i64) < numlocalplusfree {
        let kind: u8 = _PyLocals_GetKind(unsafe { (*co).co_localspluskinds }, i);
        if (kind as i32) & 128 != 0 {
            numfree = numfree.wrapping_add(1);
        } else {
            // Apparently not all non-free vars a CO_FAST_LOCAL.
            let __v2: i32 = locals.total;
            locals.total = __v2.wrapping_add(1);
            if (kind as i32) & (2 | 4 | 8) != 0 {
                let __v4: i32 = locals.args.total;
                locals.args.total = __v4.wrapping_add(1);
                if (kind as i32) & 8 != 0 {
                    if (kind as i32) & 2 != 0 {
                        locals.args.varargs = 1;
                    } else {
                        locals.args.varkwargs = 1;
                    }
                } else {
                    if (kind as i32) & 2 != 0 {
                        if (kind as i32) & 4 != 0 {
                            let __v6: i32 = locals.args.numposorkw;
                            locals.args.numposorkw = __v6.wrapping_add(1);
                        } else {
                            let __v8: i32 = locals.args.numposonly;
                            locals.args.numposonly = __v8.wrapping_add(1);
                        }
                    } else {
                        let __v10: i32 = locals.args.numkwonly;
                        locals.args.numkwonly = __v10.wrapping_add(1);
                    }
                }
                if (kind as i32) & 64 != 0 {
                    let __v12: i32 = locals.cells.total;
                    locals.cells.total = __v12.wrapping_add(1);
                    let __v14: i32 = locals.cells.numargs;
                    locals.cells.numargs = __v14.wrapping_add(1);
                }
            // Args are never hidden currently.
            } else {
                if (kind as i32) & 64 != 0 {
                    let __v16: i32 = locals.cells.total;
                    locals.cells.total = __v16.wrapping_add(1);
                    let __v18: i32 = locals.cells.numothers;
                    locals.cells.numothers = __v18.wrapping_add(1);
                    if (kind as i32) & 16 != 0 {
                        let __v20: i32 = locals.hidden.total;
                        locals.hidden.total = __v20.wrapping_add(1);
                        let __v22: i32 = locals.hidden.numcells;
                        locals.hidden.numcells = __v22.wrapping_add(1);
                    }
                } else {
                    let __v24: i32 = locals.numpure;
                    locals.numpure = __v24.wrapping_add(1);
                    if (kind as i32) & 16 != 0 {
                        let __v26: i32 = locals.hidden.total;
                        locals.hidden.total = __v26.wrapping_add(1);
                        let __v28: i32 = locals.hidden.numpure;
                        locals.hidden.numpure = __v28.wrapping_add(1);
                    }
                }
            }
        }
        i = i.wrapping_add(1);
    }
    // Get the unbound counts.
    let numunbound: i32 = PyTuple_GET_SIZE!(unsafe { (*co).co_names }) as i32;
    let unbound: co_unbound_counts = {
        let mut __t1: co_unbound_counts = unsafe { core::mem::zeroed() };
        __t1.total = numunbound;
        __t1.numunknown = numunbound;
        __t1
    };
    // numglobal and numattrs can be set later
    //
    // with _PyCode_SetUnboundVarCounts().
    // "Return" the result.
    unsafe {
        *counts = *{
            __slate_compound_33 = __SlateRecord_pycore_code_h_610 {
                total: locals
                    .total
                    .wrapping_add(numfree)
                    .wrapping_add(unbound.total),
                locals: locals,
                numfree: numfree,
                unbound: unbound,
            };
            core::ptr::addr_of_mut!(__slate_compound_33)
        };
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyCode_SetUnboundVarCounts(
    tstate: *mut _ts,
    co: *mut PyCodeObject,
    counts: *mut __SlateRecord_pycore_code_h_610,
    mut globalnames: *mut _object,
    mut attrnames: *mut _object,
    globalsns: *mut _object,
    builtinsns: *mut _object,
) -> i32 {
    let mut numdupes: i32 = unsafe { core::mem::zeroed() };
    // Fill in unbound.globals and unbound.numattrs.
    let mut unbound: core::mem::MaybeUninit<co_unbound_counts> = core::mem::MaybeUninit::uninit();
    let __slate_slot_5: *mut co_unbound_counts =
        core::ptr::addr_of_mut!(unbound) as *mut co_unbound_counts;
    let mut attrnames_owned: *mut _object = unsafe { core::mem::zeroed() };
    let mut globalnames_owned: *mut _object = unsafe { core::mem::zeroed() };
    let mut res: i32 = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_0: {
            res = 1i32.wrapping_neg();
            globalnames_owned = core::ptr::null_mut::<_object>();
            attrnames_owned = core::ptr::null_mut::<_object>();
            // Prep the name sets.
            if globalnames.is_null() {
                globalnames_owned = PySet_New(core::ptr::null_mut::<_object>());
                if globalnames_owned.is_null() {
                    break '__join_0;
                } else {
                    globalnames = globalnames_owned;
                }
            } else {
                if !PySet_Check!(globalnames) {
                    _PyErr_Format(
                        tstate,
                        PyExc_TypeError,
                        c"expected a set for \"globalnames\", got %R".as_ptr(),
                        globalnames,
                    );
                    break '__join_0;
                }
            }
            if attrnames.is_null() {
                attrnames_owned = PySet_New(core::ptr::null_mut::<_object>());
                if attrnames_owned.is_null() {
                    break '__join_0;
                } else {
                    attrnames = attrnames_owned;
                }
            } else {
                if !PySet_Check!(attrnames) {
                    _PyErr_Format(
                        tstate,
                        PyExc_TypeError,
                        c"expected a set for \"attrnames\", got %R".as_ptr(),
                        attrnames,
                    );
                    break '__join_0;
                }
            }
            core::ptr::write(__slate_slot_5, {
                let mut __t0: co_unbound_counts = core::mem::zeroed();
                __t0.total = 0i32;
                __t0
            });
            numdupes = 0;
            res = identify_unbound_names(
                tstate,
                co,
                globalnames,
                attrnames,
                globalsns,
                builtinsns,
                core::ptr::addr_of_mut!(*__slate_slot_5),
                core::ptr::addr_of_mut!(numdupes),
            );
            if res >= 0 {
                // There may be a name that is both a global and an attr.
                let totalunbound: i32 = unsafe { (*counts).unbound.total }.wrapping_add(numdupes);
                (*__slate_slot_5).numunknown = totalunbound.wrapping_sub((*__slate_slot_5).total);
                (*__slate_slot_5).total = totalunbound;
                (*counts).unbound = *__slate_slot_5;
                let __v1: i32 = (*counts).total;
                (*counts).total = __v1.wrapping_add(numdupes);
                res = 0;
            }
        }
        Py_XDECREF!(globalnames_owned);
        Py_XDECREF!(attrnames_owned);
        return res;
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyCode_CheckNoInternalState(
    co: *mut PyCodeObject,
    p_errmsg: *mut *const i8,
) -> i32 {
    let mut errmsg: *const i8 = core::ptr::null::<i8>();
    // We don't worry about co_executors, co_instrumentation,
    //
    // or co_monitoring.  They are essentially ephemeral.
    if !unsafe { (*co).co_extra }.is_null() {
        errmsg = c"only basic code objects are supported".as_ptr();
    }
    if !errmsg.is_null() {
        if !p_errmsg.is_null() {
            unsafe {
                *p_errmsg = errmsg;
            }
        }
        return 0;
    }
    1
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyCode_CheckNoExternalState(
    co: *mut PyCodeObject,
    counts: *mut __SlateRecord_pycore_code_h_610,
    p_errmsg: *mut *const i8,
) -> i32 {
    let mut errmsg: *const i8 = core::ptr::null::<i8>();
    if (unsafe { (*counts).numfree }) > 0 {
        // It's a closure.
        errmsg = c"closures not supported".as_ptr();
    } else {
        if (unsafe { (*counts).unbound.globals.numglobal }) > 0 {
            errmsg = c"globals not supported".as_ptr();
        } else {
            if (unsafe { (*counts).unbound.globals.numbuiltin }) > 0
                && (unsafe { (*counts).unbound.globals.numunknown }) > 0
            {
                errmsg = c"globals not supported".as_ptr();
            }
        }
    }
    // Otherwise we don't check counts.unbound.globals.numunknown since we can't
    //
    // distinguish beween globals and builtins here.
    if !errmsg.is_null() {
        if !p_errmsg.is_null() {
            unsafe {
                *p_errmsg = errmsg;
            }
        }
        return 0;
    }
    1
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyCode_VerifyStateless(
    tstate: *mut _ts,
    co: *mut PyCodeObject,
    globalnames: *mut _object,
    globalsns: *mut _object,
    builtinsns: *mut _object,
) -> i32 {
    let mut errmsg: *const i8 = unsafe { core::mem::zeroed() };
    let mut counts: __SlateRecord_pycore_code_h_610 = {
        let mut __t0: __SlateRecord_pycore_code_h_610 = unsafe { core::mem::zeroed() };
        __t0.total = 0i32;
        __t0
    };
    _PyCode_GetVarCounts(co, core::ptr::addr_of_mut!(counts));
    if _PyCode_SetUnboundVarCounts(
        tstate,
        co,
        core::ptr::addr_of_mut!(counts),
        globalnames,
        core::ptr::null_mut::<_object>(),
        globalsns,
        builtinsns,
    ) < 0
    {
        return 1i32.wrapping_neg();
    }
    // We may consider relaxing the internal state constraints
    //
    // if it becomes a problem.
    if _PyCode_CheckNoInternalState(co, core::ptr::addr_of_mut!(errmsg)) == 0 {
        unsafe { _PyErr_SetString(tstate, PyExc_ValueError, errmsg) };
        return 1i32.wrapping_neg();
    }
    if !builtinsns.is_null() {
        // Make sure the next check will fail for globals,
        //
        // even if there aren't any builtins.
        let __v0: i32 = counts.unbound.globals.numbuiltin;
        counts.unbound.globals.numbuiltin = __v0.wrapping_add(1);
    }
    if _PyCode_CheckNoExternalState(
        co,
        core::ptr::addr_of_mut!(counts),
        core::ptr::addr_of_mut!(errmsg),
    ) == 0
    {
        unsafe { _PyErr_SetString(tstate, PyExc_ValueError, errmsg) };
        return 1i32.wrapping_neg();
    }
    // Note that we don't check co->co_flags & CO_NESTED for anything here.
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyCode_CheckPureFunction(
    co: *mut PyCodeObject,
    p_errmsg: *mut *const i8,
) -> i32 {
    let mut errmsg: *const i8 = core::ptr::null::<i8>();
    if (unsafe { (*co).co_flags }) & 32 != 0 {
        errmsg = c"generators not supported".as_ptr();
    } else {
        if (unsafe { (*co).co_flags }) & 128 != 0 {
            errmsg = c"coroutines not supported".as_ptr();
        } else {
            if (unsafe { (*co).co_flags }) & 256 != 0 {
                errmsg = c"coroutines not supported".as_ptr();
            } else {
                if (unsafe { (*co).co_flags }) & 512 != 0 {
                    errmsg = c"generators not supported".as_ptr();
                }
            }
        }
    }
    if !errmsg.is_null() {
        if !p_errmsg.is_null() {
            unsafe {
                *p_errmsg = errmsg;
            }
        }
        return 0;
    }
    1
}

/// Here "value" means a non-None value, since a bare return is identical
/// to returning None explicitly.  Likewise a missing return statement
/// at the end of the function is turned into "return None".
fn code_returns_only_none(co: *mut PyCodeObject) -> i32 {
    if _PyCode_CheckPureFunction(co, core::ptr::null_mut::<*const i8>()) == 0 {
        return 0;
    }
    let len: i32 = Py_SIZE!(co) as i32;
    // The last instruction either returns or raises.  We can take advantage
    //
    // of that for a quick exit.
    let r#final: __SlateRecord_pycore_structs_h_32 =
        unsafe { _Py_GetBaseCodeUnit(co, len.wrapping_sub(1)) };
    // Look up None in co_consts.
    let nconsts: i64 = unsafe { PyTuple_Size((*co).co_consts) };
    let mut none_index: i32 = 0;
    while (none_index as i64) < nconsts {
        if PyTuple_GET_ITEM!(unsafe { (*co).co_consts }, none_index)
            == unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
        {
            break;
        }
        none_index = none_index.wrapping_add(1);
    }
    if (none_index as i64) == nconsts {
        // None wasn't there, which means there was no implicit return,
        //
        // "return", or "return None".
        // That means there must be
        //
        // an explicit return (non-None), or it only raises.
        if IS_RETURN_OPCODE!(unsafe { r#final.op.code }) {
            // It was an explicit return (non-None).
            return 0;
        }
        // It must end with a raise then.  We still have to walk the
        //
        // bytecode to see if there's any explicit return (non-None).
        let mut i: i32 = 0;
        while i < len {
            let inst: __SlateRecord_pycore_structs_h_32 = unsafe { _Py_GetBaseCodeUnit(co, i) };
            if IS_RETURN_OPCODE!(unsafe { inst.op.code }) {
                // We alraedy know it isn't returning None.
                return 0;
            }
            let __v4: i32 = unsafe { _PyInstruction_GetLength(co, i) };
            i = i.wrapping_add(__v4);
        }
    // It must only raise.
    } else {
        // Walk the bytecode, looking for RETURN_VALUE.
        let mut i: i32 = 0;
        while i < len {
            '__slate_continue_7: {
                let inst: __SlateRecord_pycore_structs_h_32 = unsafe { _Py_GetBaseCodeUnit(co, i) };
                if IS_RETURN_OPCODE!(unsafe { inst.op.code }) {
                    // Ignore it if it returns None.
                    let prev: __SlateRecord_pycore_structs_h_32 =
                        unsafe { _Py_GetBaseCodeUnit(co, i.wrapping_sub(1)) };
                    if ((unsafe { prev.op.code }) as i32) == 82 {
                        // We don't worry about EXTENDED_ARG for now.
                        if ((unsafe { prev.op.arg }) as i32) == none_index {
                            break '__slate_continue_7;
                        }
                    }
                    return 0;
                }
            }
            let __v8: i32 = unsafe { _PyInstruction_GetLength(co, i) };
            i = i.wrapping_add(__v8);
        }
    }
    1
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyCode_ReturnsOnlyNone(co: *mut PyCodeObject) -> i32 {
    let mut res: i32 = 0;
    res = code_returns_only_none(co);
    res
}

fn deopt_code(code: *mut PyCodeObject, instructions: *mut __SlateRecord_pycore_structs_h_32) {
    let len: i64 = Py_SIZE!(code);
    let mut i: i32 = 0;
    while (i as i64) < len {
        let inst: __SlateRecord_pycore_structs_h_32 = unsafe { _Py_GetBaseCodeUnit(code, i) };
        let caches: i32 = (unsafe { _PyOpcode_Caches[(inst.op.code as i32) as usize] }) as i32;
        unsafe {
            *instructions.wrapping_offset(i as isize) = inst;
        }
        let mut j: i32 = 1;
        while j <= caches {
            unsafe {
                (*instructions.wrapping_offset(i.wrapping_add(j) as isize)).cache = 0;
            }
            j = j.wrapping_add(1);
        }
        i = i.wrapping_add(caches);
        i = i.wrapping_add(1);
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyCode_GetCode(co: *mut PyCodeObject) -> *mut _object {
    if init_co_cached(co) != 0 {
        return core::ptr::null_mut::<_object>();
    }
    let cached: *mut __SlateRecord_code_h_16 = unsafe { (*co)._co_cached };
    let mut code: *mut _object = unsafe { (*cached)._co_code };
    if !code.is_null() {
        return Py_NewRef!(code);
    }
    code = unsafe { (*cached)._co_code };
    if code.is_null() {
        code = unsafe {
            PyBytes_FromStringAndSize(_PyCode_CODE!(co) as *const i8, _PyCode_NBYTES!(co))
        };
        if !code.is_null() {
            deopt_code(
                co,
                PyBytes_AS_STRING!(code) as *mut __SlateRecord_pycore_structs_h_32,
            );
            FT_ATOMIC_STORE_PTR!((*cached)._co_code, code);
        }
    }
    Py_XNewRef!(code)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyCode_GetCode(co: *mut PyCodeObject) -> *mut _object {
    _PyCode_GetCode(co)
}

// PyCode_Type
// [clinic input]
// class code "PyCodeObject *" "&PyCode_Type"
// [clinic start generated code]
//
// [clinic end generated code: output=da39a3ee5e6b4b0d input=78aa5d576683bb4b]
// [clinic input]
// @classmethod
// code.__new__ as code_new
//
//     argcount: int
//     posonlyargcount: int
//     kwonlyargcount: int
//     nlocals: int
//     stacksize: int
//     flags: int
//     codestring as code: object(subclass_of="&PyBytes_Type")
//     constants as consts: object(subclass_of="&PyTuple_Type")
//     names: object(subclass_of="&PyTuple_Type")
//     varnames: object(subclass_of="&PyTuple_Type")
//     filename: unicode
//     name: unicode
//     qualname: unicode
//     firstlineno: int
//     linetable: object(subclass_of="&PyBytes_Type")
//     exceptiontable: object(subclass_of="&PyBytes_Type")
//     freevars: object(subclass_of="&PyTuple_Type", c_default="NULL") = ()
//     cellvars: object(subclass_of="&PyTuple_Type", c_default="NULL") = ()
//     /
//
// Create a code object.  Not for the faint of heart.
// [clinic start generated code]
/// [clinic end generated code: output=069fa20d299f9dda input=e31da3c41ad8064a]
fn code_new_impl(
    r#type: *mut _typeobject,
    argcount: i32,
    posonlyargcount: i32,
    kwonlyargcount: i32,
    nlocals: i32,
    stacksize: i32,
    flags: i32,
    code: *mut _object,
    consts: *mut _object,
    names: *mut _object,
    varnames: *mut _object,
    filename: *mut _object,
    name: *mut _object,
    qualname: *mut _object,
    firstlineno: i32,
    linetable__0: *mut _object,
    exceptiontable: *mut _object,
    freevars: *mut _object,
    cellvars: *mut _object,
) -> *mut _object {
    let mut ourcellvars: *mut _object = unsafe { core::mem::zeroed() };
    let mut ourfreevars: *mut _object = unsafe { core::mem::zeroed() };
    let mut ourvarnames: *mut _object = unsafe { core::mem::zeroed() };
    let mut ournames: *mut _object = unsafe { core::mem::zeroed() };
    let mut co: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        co = core::ptr::null_mut::<_object>();
        ournames = core::ptr::null_mut::<_object>();
        ourvarnames = core::ptr::null_mut::<_object>();
        ourfreevars = core::ptr::null_mut::<_object>();
        ourcellvars = core::ptr::null_mut::<_object>();
        if PySys_Audit(
            c"code.__new__".as_ptr(),
            c"OOOiiiiii".as_ptr(),
            code,
            filename,
            name,
            argcount,
            posonlyargcount,
            kwonlyargcount,
            nlocals,
            stacksize,
            flags,
        ) >= 0
        {
            if argcount < 0 {
                PyErr_SetString(
                    PyExc_ValueError,
                    c"code: argcount must not be negative".as_ptr(),
                );
            } else {
                if posonlyargcount < 0 {
                    PyErr_SetString(
                        PyExc_ValueError,
                        c"code: posonlyargcount must not be negative".as_ptr(),
                    );
                } else {
                    if kwonlyargcount < 0 {
                        PyErr_SetString(
                            PyExc_ValueError,
                            c"code: kwonlyargcount must not be negative".as_ptr(),
                        );
                    } else {
                        if nlocals < 0 {
                            PyErr_SetString(
                                PyExc_ValueError,
                                c"code: nlocals must not be negative".as_ptr(),
                            );
                        } else {
                            ournames = validate_and_copy_tuple(names);
                            if !ournames.is_null() {
                                ourvarnames = validate_and_copy_tuple(varnames);
                                if !ourvarnames.is_null() {
                                    if !freevars.is_null() {
                                        ourfreevars = validate_and_copy_tuple(freevars);
                                    } else {
                                        ourfreevars = PyTuple_New(0i64);
                                    }
                                    if !ourfreevars.is_null() {
                                        if !cellvars.is_null() {
                                            ourcellvars = validate_and_copy_tuple(cellvars);
                                        } else {
                                            ourcellvars = PyTuple_New(0i64);
                                        }
                                        if !ourcellvars.is_null() {
                                            co = PyCode_NewWithPosOnlyArgs(
                                                argcount,
                                                posonlyargcount,
                                                kwonlyargcount,
                                                nlocals,
                                                stacksize,
                                                flags,
                                                code,
                                                consts,
                                                ournames,
                                                ourvarnames,
                                                ourfreevars,
                                                ourcellvars,
                                                filename,
                                                name,
                                                qualname,
                                                firstlineno,
                                                linetable__0,
                                                exceptiontable,
                                            )
                                                as *mut _object;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Py_XDECREF!(ournames);
        Py_XDECREF!(ourvarnames);
        Py_XDECREF!(ourfreevars);
        Py_XDECREF!(ourcellvars);
        return co;
    }
}

fn free_monitoring_data(data: *mut _PyCoMonitoringData) {
    if data.is_null() {
        return;
    }
    if !unsafe { (*data).tools }.is_null() {
        unsafe { PyMem_Free((*data).tools as *mut ()) };
    }
    if !unsafe { (*data).lines }.is_null() {
        unsafe { PyMem_Free((*data).lines as *mut ()) };
    }
    if !unsafe { (*data).line_tools }.is_null() {
        unsafe { PyMem_Free((*data).line_tools as *mut ()) };
    }
    if !unsafe { (*data).per_instruction_opcodes }.is_null() {
        unsafe { PyMem_Free((*data).per_instruction_opcodes as *mut ()) };
    }
    if !unsafe { (*data).per_instruction_tools }.is_null() {
        unsafe { PyMem_Free((*data).per_instruction_tools as *mut ()) };
    }
    unsafe { PyMem_Free(data as *mut ()) };
}

#[unsafe(link_section = ".text.slate_distinct.codeobject.code_dealloc")]
extern "C-unwind" fn code_dealloc(self_: *mut _object) {
    let tstate: *mut _ts = PyThreadState_GET!();
    _Py_atomic_add_uint64(
        unsafe { core::ptr::addr_of_mut!((*(*tstate).interp)._code_object_generation) },
        1u64,
    );
    let co: *mut PyCodeObject = _PyCodeObject_CAST!(self_);
    _PyObject_ResurrectStart(self_);
    notify_code_watchers(1u32, co);
    if _PyObject_ResurrectEnd(self_) != 0 {
        return;
    }
    unsafe { _PyFunction_ClearCodeByVersion((*co).co_version) };
    if !unsafe { (*co).co_extra }.is_null() {
        let interp: *mut _is = _PyInterpreterState_GET();
        let co_extra: *mut __SlateRecord_codeobject_c_1580 =
            (unsafe { (*co).co_extra }) as *mut __SlateRecord_codeobject_c_1580;
        let mut i: i64 = 0;
        while i < unsafe { (*co_extra).ce_size } {
            let free_extra: Option<unsafe extern "C-unwind" fn(*mut ())> = unsafe {
                *((*interp).co_extra_freefuncs.as_mut_ptr()
                    as *mut Option<unsafe extern "C-unwind" fn(*mut ())>)
                    .wrapping_offset(i as isize)
            };
            if free_extra != None {
                unsafe {
                    free_extra.unwrap()(
                        *((*co_extra).ce_extras.as_mut_ptr() as *mut *mut ())
                            .wrapping_offset(i as isize),
                    )
                };
            }
            i = i.wrapping_add(1);
        }
        unsafe { PyMem_Free(co_extra as *mut ()) };
    }
    Py_XDECREF!(unsafe { (*co).co_consts });
    Py_XDECREF!(unsafe { (*co).co_names });
    Py_XDECREF!(unsafe { (*co).co_localsplusnames });
    Py_XDECREF!(unsafe { (*co).co_localspluskinds });
    Py_XDECREF!(unsafe { (*co).co_filename });
    Py_XDECREF!(unsafe { (*co).co_name });
    Py_XDECREF!(unsafe { (*co).co_qualname });
    Py_XDECREF!(unsafe { (*co).co_linetable });
    Py_XDECREF!(unsafe { (*co).co_exceptiontable });
    if !unsafe { (*co)._co_cached }.is_null() {
        Py_XDECREF!(unsafe { (*(*co)._co_cached)._co_code });
        Py_XDECREF!(unsafe { (*(*co)._co_cached)._co_cellvars });
        Py_XDECREF!(unsafe { (*(*co)._co_cached)._co_freevars });
        Py_XDECREF!(unsafe { (*(*co)._co_cached)._co_varnames });
        unsafe { PyMem_Free((*co)._co_cached as *mut ()) };
    }
    FT_CLEAR_WEAKREFS!(self_, unsafe { (*co).co_weakreflist });
    free_monitoring_data(unsafe { (*co)._co_monitoring });
    unsafe { PyObject_Free(co as *mut ()) };
}

#[unsafe(link_section = ".text.slate_distinct.codeobject.code_repr")]
extern "C-unwind" fn code_repr(self_: *mut _object) -> *mut _object {
    let co: *mut PyCodeObject = _PyCodeObject_CAST!(self_);
    let lineno: i32 = if (unsafe { (*co).co_firstlineno }) != 0 {
        unsafe { (*co).co_firstlineno }
    } else {
        1i32.wrapping_neg()
    };
    let __v0: bool = !unsafe { (*co).co_filename }.is_null()
        && PyUnicode_Check!(unsafe { (*co).co_filename }) != 0;
    if __v0 {
        return unsafe {
            PyUnicode_FromFormat(
                c"<code object %U at %p, file \"%U\", line %d>".as_ptr(),
                (*co).co_name,
                co,
                (*co).co_filename,
                lineno,
            )
        };
    } else {
        return unsafe {
            PyUnicode_FromFormat(
                c"<code object %U at %p, file ???, line %d>".as_ptr(),
                (*co).co_name,
                co,
                lineno,
            )
        };
    }
}

#[unsafe(link_section = ".text.slate_distinct.codeobject.code_richcompare")]
extern "C-unwind" fn code_richcompare(
    self_: *mut _object,
    other: *mut _object,
    op: i32,
) -> *mut _object {
    let mut cp_instr: core::mem::MaybeUninit<__SlateRecord_pycore_structs_h_32> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_4: *mut __SlateRecord_pycore_structs_h_32 =
        core::ptr::addr_of_mut!(cp_instr) as *mut __SlateRecord_pycore_structs_h_32;
    let mut co_instr: core::mem::MaybeUninit<__SlateRecord_pycore_structs_h_32> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_5: *mut __SlateRecord_pycore_structs_h_32 =
        core::ptr::addr_of_mut!(co_instr) as *mut __SlateRecord_pycore_structs_h_32;
    let mut i: i32 = unsafe { core::mem::zeroed() };
    let mut __v7: bool = unsafe { core::mem::zeroed() };
    let mut __v8: bool = unsafe { core::mem::zeroed() };
    let mut res: *mut _object = unsafe { core::mem::zeroed() };
    let mut consts2: *mut _object = unsafe { core::mem::zeroed() };
    let mut consts1: *mut _object = unsafe { core::mem::zeroed() };
    let mut eq: i32 = unsafe { core::mem::zeroed() };
    let mut cp: *mut PyCodeObject = unsafe { core::mem::zeroed() };
    let mut co: *mut PyCodeObject = unsafe { core::mem::zeroed() };
    unsafe {
        if op != 2 && op != 3 {
            __v8 = true;
        } else {
            __v8 = PyCode_Check!(self_) == 0;
        }
        if __v8 {
            __v7 = true;
        } else {
            __v7 = PyCode_Check!(other) == 0;
        }
        if __v7 {
            return core::ptr::addr_of_mut!(_Py_NotImplementedStruct);
        } else {
            '__join_0: {
                '__join_5: {
                    co = self_ as *mut PyCodeObject;
                    cp = other as *mut PyCodeObject;
                    eq = PyObject_RichCompareBool((*co).co_name, (*cp).co_name, 2i32);
                    if eq != 0 {
                        eq = ((*co).co_argcount == (*cp).co_argcount) as i32;
                        if eq != 0 {
                            eq = ((*co).co_posonlyargcount == (*cp).co_posonlyargcount) as i32;
                            if eq != 0 {
                                eq = ((*co).co_kwonlyargcount == (*cp).co_kwonlyargcount) as i32;
                                if eq != 0 {
                                    eq = ((*co).co_flags == (*cp).co_flags) as i32;
                                    if eq != 0 {
                                        eq = ((*co).co_firstlineno == (*cp).co_firstlineno) as i32;
                                        if eq != 0 {
                                            eq = (Py_SIZE!(co) == Py_SIZE!(cp)) as i32;
                                            if eq != 0 {
                                                i = 0;
                                                while (i as i64) < Py_SIZE!(co) {
                                                    core::ptr::write(
                                                        __slate_slot_5,
                                                        _Py_GetBaseCodeUnit(co, i),
                                                    );
                                                    core::ptr::write(
                                                        __slate_slot_4,
                                                        _Py_GetBaseCodeUnit(cp, i),
                                                    );
                                                    if ((*__slate_slot_5).cache as i32)
                                                        != ((*__slate_slot_4).cache as i32)
                                                    {
                                                        break '__join_5;
                                                    } else {
                                                        i = i.wrapping_add(
                                                            _PyOpcode_Caches[((*__slate_slot_5)
                                                                .op
                                                                .code
                                                                as i32)
                                                                as usize]
                                                                as i32,
                                                        );
                                                        i = i.wrapping_add(1);
                                                    }
                                                }
                                                // compare constants
                                                consts1 = _PyCode_ConstantKey((*co).co_consts);
                                                if consts1.is_null() {
                                                    return core::ptr::null_mut::<_object>();
                                                } else {
                                                    consts2 = _PyCode_ConstantKey((*cp).co_consts);
                                                    if consts2.is_null() {
                                                        Py_DECREF!(consts1);
                                                        return core::ptr::null_mut::<_object>();
                                                    } else {
                                                        eq = PyObject_RichCompareBool(
                                                            consts1, consts2, 2i32,
                                                        );
                                                        Py_DECREF!(consts1);
                                                        Py_DECREF!(consts2);
                                                        if eq > 0 {
                                                            eq = PyObject_RichCompareBool(
                                                                (*co).co_names,
                                                                (*cp).co_names,
                                                                2i32,
                                                            );
                                                            if eq > 0 {
                                                                eq = PyObject_RichCompareBool(
                                                                    (*co).co_localsplusnames,
                                                                    (*cp).co_localsplusnames,
                                                                    2i32,
                                                                );
                                                                if eq > 0 {
                                                                    eq = PyObject_RichCompareBool(
                                                                        (*co).co_linetable,
                                                                        (*cp).co_linetable,
                                                                        2i32,
                                                                    );
                                                                    if eq > 0 {
                                                                        eq = PyObject_RichCompareBool((*co).co_exceptiontable, (*cp).co_exceptiontable, 2i32);
                                                                        if eq > 0 {
                                                                            if op == 2 {
                                                                                res = _PyObject_CAST__2!(core::ptr::addr_of_mut!(_Py_TrueStruct));
                                                                                break '__join_0;
                                                                            } else {
                                                                                res = _PyObject_CAST__2!(core::ptr::addr_of_mut!(_Py_FalseStruct));
                                                                                break '__join_0;
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
                            }
                        }
                    }
                }
                if eq < 0 {
                    return core::ptr::null_mut::<_object>();
                } else {
                    if op == 3 {
                        res = _PyObject_CAST__2!(core::ptr::addr_of_mut!(_Py_TrueStruct));
                    } else {
                        res = _PyObject_CAST__2!(core::ptr::addr_of_mut!(_Py_FalseStruct));
                    }
                }
            }
            return Py_NewRef!(res);
        }
    }
}

#[unsafe(link_section = ".text.slate_distinct.codeobject.code_hash")]
extern "C-unwind" fn code_hash(self_: *mut _object) -> i64 {
    let co: *mut PyCodeObject = _PyCodeObject_CAST!(self_);
    let mut uhash: u64 = 20221211;
    SCRAMBLE_IN_HASH!(unsafe { (*co).co_name }, uhash);
    SCRAMBLE_IN_HASH!(unsafe { (*co).co_consts }, uhash);
    SCRAMBLE_IN_HASH!(unsafe { (*co).co_names }, uhash);
    SCRAMBLE_IN_HASH!(unsafe { (*co).co_localsplusnames }, uhash);
    SCRAMBLE_IN_HASH!(unsafe { (*co).co_linetable }, uhash);
    SCRAMBLE_IN_HASH!(unsafe { (*co).co_exceptiontable }, uhash);
    SCRAMBLE_IN!(unsafe { (*co).co_argcount }, uhash);
    SCRAMBLE_IN!(unsafe { (*co).co_posonlyargcount }, uhash);
    SCRAMBLE_IN!(unsafe { (*co).co_kwonlyargcount }, uhash);
    SCRAMBLE_IN!(unsafe { (*co).co_flags }, uhash);
    SCRAMBLE_IN!(unsafe { (*co).co_firstlineno }, uhash);
    SCRAMBLE_IN__2!(Py_SIZE!(co), uhash);
    let mut i: i32 = 0;
    while (i as i64) < Py_SIZE!(co) {
        let co_instr: __SlateRecord_pycore_structs_h_32 = unsafe { _Py_GetBaseCodeUnit(co, i) };
        SCRAMBLE_IN!(unsafe { co_instr.op.code }, uhash);
        SCRAMBLE_IN!(unsafe { co_instr.op.arg }, uhash);
        i = i
            .wrapping_add((unsafe { _PyOpcode_Caches[(co_instr.op.code as i32) as usize] }) as i32);
        i = i.wrapping_add(1);
    }
    if (uhash as i64) == (1i32.wrapping_neg() as i64) {
        return 2i32.wrapping_neg() as i64;
    }
    uhash as i64
}

/// Sentinel
static mut code_memberlist: __SlateAlign16<[PyMemberDef; 15]> = __SlateAlign16([
    {
        let mut __t0: PyMemberDef = unsafe { core::mem::zeroed() };
        __t0.name = (b"co_argcount\0".as_ptr() as *mut i8) as *const i8;
        __t0.r#type = 1i32;
        __t0.offset = (core::mem::offset_of!(PyCodeObject, co_argcount) as u64) as i64;
        __t0.flags = 1i32;
        __t0
    },
    {
        let mut __t1: PyMemberDef = unsafe { core::mem::zeroed() };
        __t1.name = (b"co_posonlyargcount\0".as_ptr() as *mut i8) as *const i8;
        __t1.r#type = 1i32;
        __t1.offset = (core::mem::offset_of!(PyCodeObject, co_posonlyargcount) as u64) as i64;
        __t1.flags = 1i32;
        __t1
    },
    {
        let mut __t2: PyMemberDef = unsafe { core::mem::zeroed() };
        __t2.name = (b"co_kwonlyargcount\0".as_ptr() as *mut i8) as *const i8;
        __t2.r#type = 1i32;
        __t2.offset = (core::mem::offset_of!(PyCodeObject, co_kwonlyargcount) as u64) as i64;
        __t2.flags = 1i32;
        __t2
    },
    {
        let mut __t3: PyMemberDef = unsafe { core::mem::zeroed() };
        __t3.name = (b"co_stacksize\0".as_ptr() as *mut i8) as *const i8;
        __t3.r#type = 1i32;
        __t3.offset = (core::mem::offset_of!(PyCodeObject, co_stacksize) as u64) as i64;
        __t3.flags = 1i32;
        __t3
    },
    {
        let mut __t4: PyMemberDef = unsafe { core::mem::zeroed() };
        __t4.name = (b"co_flags\0".as_ptr() as *mut i8) as *const i8;
        __t4.r#type = 1i32;
        __t4.offset = (core::mem::offset_of!(PyCodeObject, co_flags) as u64) as i64;
        __t4.flags = 1i32;
        __t4
    },
    {
        let mut __t5: PyMemberDef = unsafe { core::mem::zeroed() };
        __t5.name = (b"co_nlocals\0".as_ptr() as *mut i8) as *const i8;
        __t5.r#type = 1i32;
        __t5.offset = (core::mem::offset_of!(PyCodeObject, co_nlocals) as u64) as i64;
        __t5.flags = 1i32;
        __t5
    },
    {
        let mut __t6: PyMemberDef = unsafe { core::mem::zeroed() };
        __t6.name = (b"co_consts\0".as_ptr() as *mut i8) as *const i8;
        __t6.r#type = 6i32;
        __t6.offset = (core::mem::offset_of!(PyCodeObject, co_consts) as u64) as i64;
        __t6.flags = 1i32;
        __t6
    },
    {
        let mut __t7: PyMemberDef = unsafe { core::mem::zeroed() };
        __t7.name = (b"co_names\0".as_ptr() as *mut i8) as *const i8;
        __t7.r#type = 6i32;
        __t7.offset = (core::mem::offset_of!(PyCodeObject, co_names) as u64) as i64;
        __t7.flags = 1i32;
        __t7
    },
    {
        let mut __t8: PyMemberDef = unsafe { core::mem::zeroed() };
        __t8.name = (b"co_filename\0".as_ptr() as *mut i8) as *const i8;
        __t8.r#type = 6i32;
        __t8.offset = (core::mem::offset_of!(PyCodeObject, co_filename) as u64) as i64;
        __t8.flags = 1i32;
        __t8
    },
    {
        let mut __t9: PyMemberDef = unsafe { core::mem::zeroed() };
        __t9.name = (b"co_name\0".as_ptr() as *mut i8) as *const i8;
        __t9.r#type = 6i32;
        __t9.offset = (core::mem::offset_of!(PyCodeObject, co_name) as u64) as i64;
        __t9.flags = 1i32;
        __t9
    },
    {
        let mut __t10: PyMemberDef = unsafe { core::mem::zeroed() };
        __t10.name = (b"co_qualname\0".as_ptr() as *mut i8) as *const i8;
        __t10.r#type = 6i32;
        __t10.offset = (core::mem::offset_of!(PyCodeObject, co_qualname) as u64) as i64;
        __t10.flags = 1i32;
        __t10
    },
    {
        let mut __t11: PyMemberDef = unsafe { core::mem::zeroed() };
        __t11.name = (b"co_firstlineno\0".as_ptr() as *mut i8) as *const i8;
        __t11.r#type = 1i32;
        __t11.offset = (core::mem::offset_of!(PyCodeObject, co_firstlineno) as u64) as i64;
        __t11.flags = 1i32;
        __t11
    },
    {
        let mut __t12: PyMemberDef = unsafe { core::mem::zeroed() };
        __t12.name = (b"co_linetable\0".as_ptr() as *mut i8) as *const i8;
        __t12.r#type = 6i32;
        __t12.offset = (core::mem::offset_of!(PyCodeObject, co_linetable) as u64) as i64;
        __t12.flags = 1i32;
        __t12
    },
    {
        let mut __t13: PyMemberDef = unsafe { core::mem::zeroed() };
        __t13.name = (b"co_exceptiontable\0".as_ptr() as *mut i8) as *const i8;
        __t13.r#type = 6i32;
        __t13.offset = (core::mem::offset_of!(PyCodeObject, co_exceptiontable) as u64) as i64;
        __t13.flags = 1i32;
        __t13
    },
    {
        let mut __t14: PyMemberDef = unsafe { core::mem::zeroed() };
        __t14.name = core::ptr::null::<i8>();
        __t14
    },
]);

#[unsafe(link_section = ".text.slate_distinct.codeobject.code_getlnotab")]
extern "C-unwind" fn code_getlnotab(self_: *mut _object, closure: *mut ()) -> *mut _object {
    let code: *mut PyCodeObject = _PyCodeObject_CAST!(self_);
    if (unsafe {
        PyErr_WarnEx(
            PyExc_DeprecationWarning,
            c"co_lnotab is deprecated, use co_lines instead.".as_ptr(),
            1i64,
        )
    }) < 0
    {
        return core::ptr::null_mut::<_object>();
    }
    decode_linetable(code)
}

#[unsafe(link_section = ".text.slate_distinct.codeobject.code_getvarnames")]
extern "C-unwind" fn code_getvarnames(self_: *mut _object, closure: *mut ()) -> *mut _object {
    let code: *mut PyCodeObject = _PyCodeObject_CAST!(self_);
    _PyCode_GetVarnames(code)
}

#[unsafe(link_section = ".text.slate_distinct.codeobject.code_getcellvars")]
extern "C-unwind" fn code_getcellvars(self_: *mut _object, closure: *mut ()) -> *mut _object {
    let code: *mut PyCodeObject = _PyCodeObject_CAST!(self_);
    _PyCode_GetCellvars(code)
}

#[unsafe(link_section = ".text.slate_distinct.codeobject.code_getfreevars")]
extern "C-unwind" fn code_getfreevars(self_: *mut _object, closure: *mut ()) -> *mut _object {
    let code: *mut PyCodeObject = _PyCodeObject_CAST!(self_);
    _PyCode_GetFreevars(code)
}

#[unsafe(link_section = ".text.slate_distinct.codeobject.code_getcodeadaptive")]
extern "C-unwind" fn code_getcodeadaptive(self_: *mut _object, closure: *mut ()) -> *mut _object {
    let code: *mut PyCodeObject = _PyCodeObject_CAST!(self_);
    unsafe {
        PyBytes_FromStringAndSize(
            ((*code).co_code_adaptive.as_mut_ptr() as *mut i8) as *const i8,
            _PyCode_NBYTES!(code),
        )
    }
}

#[unsafe(link_section = ".text.slate_distinct.codeobject.code_getcode")]
extern "C-unwind" fn code_getcode(self_: *mut _object, closure: *mut ()) -> *mut _object {
    let code: *mut PyCodeObject = _PyCodeObject_CAST!(self_);
    _PyCode_GetCode(code)
}

/// The following old names are kept for backward compatibility.
static mut code_getsetlist: __SlateAlign16<[PyGetSetDef; 7]> = __SlateAlign16([
    {
        let mut __t0: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t0.name = (b"co_lnotab\0".as_ptr() as *mut i8) as *const i8;
        __t0.get = Some(code_getlnotab);
        __t0.set = None;
        __t0.doc = core::ptr::null::<i8>();
        __t0
    },
    {
        let mut __t1: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t1.name = (b"_co_code_adaptive\0".as_ptr() as *mut i8) as *const i8;
        __t1.get = Some(code_getcodeadaptive);
        __t1.set = None;
        __t1.doc = core::ptr::null::<i8>();
        __t1
    },
    {
        let mut __t2: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t2.name = (b"co_varnames\0".as_ptr() as *mut i8) as *const i8;
        __t2.get = Some(code_getvarnames);
        __t2.set = None;
        __t2.doc = core::ptr::null::<i8>();
        __t2
    },
    {
        let mut __t3: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t3.name = (b"co_cellvars\0".as_ptr() as *mut i8) as *const i8;
        __t3.get = Some(code_getcellvars);
        __t3.set = None;
        __t3.doc = core::ptr::null::<i8>();
        __t3
    },
    {
        let mut __t4: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t4.name = (b"co_freevars\0".as_ptr() as *mut i8) as *const i8;
        __t4.get = Some(code_getfreevars);
        __t4.set = None;
        __t4.doc = core::ptr::null::<i8>();
        __t4
    },
    {
        let mut __t5: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t5.name = (b"co_code\0".as_ptr() as *mut i8) as *const i8;
        __t5.get = Some(code_getcode);
        __t5.set = None;
        __t5.doc = core::ptr::null::<i8>();
        __t5
    },
    {
        let mut __t6: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t6.name = core::ptr::null::<i8>();
        __t6
    },
]);

#[unsafe(link_section = ".text.slate_distinct.codeobject.code_sizeof")]
extern "C-unwind" fn code_sizeof(self_: *mut _object, _unused_args: *mut _object) -> *mut _object {
    let co: *mut PyCodeObject = _PyCodeObject_CAST!(self_);
    let mut res: u64 = _PyObject_VAR_SIZE(Py_TYPE__2!(co), Py_SIZE!(co));
    let co_extra: *mut __SlateRecord_codeobject_c_1580 =
        (unsafe { (*co).co_extra }) as *mut __SlateRecord_codeobject_c_1580;
    if !co_extra.is_null() {
        res = res.wrapping_add(core::mem::size_of::<__SlateRecord_codeobject_c_1580>() as u64);
        res = res.wrapping_add(
            ((unsafe { (*co_extra).ce_size }) as u64)
                .wrapping_sub(1)
                .wrapping_mul(core::mem::size_of::<*mut ()>() as u64),
        );
    }
    unsafe { PyLong_FromSize_t(res) }
}

#[unsafe(link_section = ".text.slate_distinct.codeobject.code_linesiterator")]
extern "C-unwind" fn code_linesiterator(
    self_: *mut _object,
    _unused_args: *mut _object,
) -> *mut _object {
    let code: *mut PyCodeObject = _PyCodeObject_CAST!(self_);
    new_linesiterator(code) as *mut _object
}

#[unsafe(link_section = ".text.slate_distinct.codeobject.code_branchesiterator")]
extern "C-unwind" fn code_branchesiterator(
    self_: *mut _object,
    _unused_args: *mut _object,
) -> *mut _object {
    let code: *mut PyCodeObject = _PyCodeObject_CAST!(self_);
    unsafe { _PyInstrumentation_BranchesIterator(code) }
}

// [clinic input]
// @text_signature "($self, /, **changes)"
// code.replace
//
//     *
//     co_argcount: int(c_default="((PyCodeObject *)self)->co_argcount") = unchanged
//     co_posonlyargcount: int(c_default="((PyCodeObject *)self)->co_posonlyargcount") = unchanged
//     co_kwonlyargcount: int(c_default="((PyCodeObject *)self)->co_kwonlyargcount") = unchanged
//     co_nlocals: int(c_default="((PyCodeObject *)self)->co_nlocals") = unchanged
//     co_stacksize: int(c_default="((PyCodeObject *)self)->co_stacksize") = unchanged
//     co_flags: int(c_default="((PyCodeObject *)self)->co_flags") = unchanged
//     co_firstlineno: int(c_default="((PyCodeObject *)self)->co_firstlineno") = unchanged
//     co_code: object(subclass_of="&PyBytes_Type", c_default="NULL") = unchanged
//     co_consts: object(subclass_of="&PyTuple_Type", c_default="((PyCodeObject *)self)->co_consts") = unchanged
//     co_names: object(subclass_of="&PyTuple_Type", c_default="((PyCodeObject *)self)->co_names") = unchanged
//     co_varnames: object(subclass_of="&PyTuple_Type", c_default="NULL") = unchanged
//     co_freevars: object(subclass_of="&PyTuple_Type", c_default="NULL") = unchanged
//     co_cellvars: object(subclass_of="&PyTuple_Type", c_default="NULL") = unchanged
//     co_filename: unicode(c_default="((PyCodeObject *)self)->co_filename") = unchanged
//     co_name: unicode(c_default="((PyCodeObject *)self)->co_name") = unchanged
//     co_qualname: unicode(c_default="((PyCodeObject *)self)->co_qualname") = unchanged
//     co_linetable: object(subclass_of="&PyBytes_Type", c_default="((PyCodeObject *)self)->co_linetable") = unchanged
//     co_exceptiontable: object(subclass_of="&PyBytes_Type", c_default="((PyCodeObject *)self)->co_exceptiontable") = unchanged
//
// Return a copy of the code object with new values for the specified fields.
// [clinic start generated code]
/// [clinic end generated code: output=e75c48a15def18b9 input=a455a89c57ac9d42]
fn code_replace_impl(
    self_: *mut PyCodeObject,
    co_argcount: i32,
    co_posonlyargcount: i32,
    co_kwonlyargcount: i32,
    co_nlocals: i32,
    co_stacksize: i32,
    co_flags: i32,
    co_firstlineno: i32,
    mut co_code: *mut _object,
    co_consts: *mut _object,
    co_names: *mut _object,
    mut co_varnames: *mut _object,
    mut co_freevars: *mut _object,
    mut co_cellvars: *mut _object,
    co_filename: *mut _object,
    co_name: *mut _object,
    co_qualname: *mut _object,
    co_linetable: *mut _object,
    co_exceptiontable: *mut _object,
) -> *mut _object {
    let mut freevars: *mut _object = unsafe { core::mem::zeroed() };
    let mut cellvars: *mut _object = unsafe { core::mem::zeroed() };
    let mut varnames: *mut _object = unsafe { core::mem::zeroed() };
    let mut co: *mut PyCodeObject = unsafe { core::mem::zeroed() };
    let mut code: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        if co_argcount < 0 {
            PyErr_SetString(
                PyExc_ValueError,
                c"co_argcount must be a positive integer".as_ptr(),
            );
            return core::ptr::null_mut::<_object>();
        } else {
            if co_posonlyargcount < 0 {
                PyErr_SetString(
                    PyExc_ValueError,
                    c"co_posonlyargcount must be a positive integer".as_ptr(),
                );
                return core::ptr::null_mut::<_object>();
            } else {
                if co_kwonlyargcount < 0 {
                    PyErr_SetString(
                        PyExc_ValueError,
                        c"co_kwonlyargcount must be a positive integer".as_ptr(),
                    );
                    return core::ptr::null_mut::<_object>();
                } else {
                    if co_nlocals < 0 {
                        PyErr_SetString(
                            PyExc_ValueError,
                            c"co_nlocals must be a positive integer".as_ptr(),
                        );
                        return core::ptr::null_mut::<_object>();
                    } else {
                        if co_stacksize < 0 {
                            PyErr_SetString(
                                PyExc_ValueError,
                                c"co_stacksize must be a positive integer".as_ptr(),
                            );
                            return core::ptr::null_mut::<_object>();
                        } else {
                            if co_flags < 0 {
                                PyErr_SetString(
                                    PyExc_ValueError,
                                    c"co_flags must be a positive integer".as_ptr(),
                                );
                                return core::ptr::null_mut::<_object>();
                            } else {
                                if co_firstlineno < 0 {
                                    PyErr_SetString(
                                        PyExc_ValueError,
                                        c"co_firstlineno must be a positive integer".as_ptr(),
                                    );
                                    return core::ptr::null_mut::<_object>();
                                } else {
                                    code = core::ptr::null_mut::<_object>();
                                    if co_code.is_null() {
                                        code = _PyCode_GetCode(self_);
                                        if code.is_null() {
                                            return core::ptr::null_mut::<_object>();
                                        } else {
                                            co_code = code;
                                        }
                                    }
                                    if PySys_Audit(
                                        c"code.__new__".as_ptr(),
                                        c"OOOiiiiii".as_ptr(),
                                        co_code,
                                        co_filename,
                                        co_name,
                                        co_argcount,
                                        co_posonlyargcount,
                                        co_kwonlyargcount,
                                        co_nlocals,
                                        co_stacksize,
                                        co_flags,
                                    ) < 0
                                    {
                                        Py_XDECREF!(code);
                                        return core::ptr::null_mut::<_object>();
                                    } else {
                                        '__join_0: {
                                            co = core::ptr::null_mut::<PyCodeObject>();
                                            varnames = core::ptr::null_mut::<_object>();
                                            cellvars = core::ptr::null_mut::<_object>();
                                            freevars = core::ptr::null_mut::<_object>();
                                            if co_varnames.is_null() {
                                                varnames = get_localsplus_names(
                                                    self_,
                                                    32u8,
                                                    (*self_).co_nlocals,
                                                );
                                                if varnames.is_null() {
                                                    break '__join_0;
                                                } else {
                                                    co_varnames = varnames;
                                                }
                                            }
                                            if co_cellvars.is_null() {
                                                cellvars = get_localsplus_names(
                                                    self_,
                                                    64u8,
                                                    (*self_).co_ncellvars,
                                                );
                                                if cellvars.is_null() {
                                                    break '__join_0;
                                                } else {
                                                    co_cellvars = cellvars;
                                                }
                                            }
                                            if co_freevars.is_null() {
                                                freevars = get_localsplus_names(
                                                    self_,
                                                    128u8,
                                                    (*self_).co_nfreevars,
                                                );
                                                if freevars.is_null() {
                                                    break '__join_0;
                                                } else {
                                                    co_freevars = freevars;
                                                }
                                            }
                                            co = PyCode_NewWithPosOnlyArgs(
                                                co_argcount,
                                                co_posonlyargcount,
                                                co_kwonlyargcount,
                                                co_nlocals,
                                                co_stacksize,
                                                co_flags,
                                                co_code,
                                                co_consts,
                                                co_names,
                                                co_varnames,
                                                co_freevars,
                                                co_cellvars,
                                                co_filename,
                                                co_name,
                                                co_qualname,
                                                co_firstlineno,
                                                co_linetable,
                                                co_exceptiontable,
                                            );
                                        }
                                        Py_XDECREF!(code);
                                        Py_XDECREF!(varnames);
                                        Py_XDECREF!(cellvars);
                                        Py_XDECREF!(freevars);
                                        return co as *mut _object;
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

// [clinic input]
// code._varname_from_oparg
//
//     oparg: int
//
// (internal-only) Return the local variable name for the given oparg.
//
// WARNING: this method is for internal use only and may change or go
// away.
// [clinic start generated code]
/// [clinic end generated code: output=1fd1130413184206 input=6ba7d6df0d566463]
fn code__varname_from_oparg_impl(self_: *mut PyCodeObject, oparg: i32) -> *mut _object {
    let name: *mut _object = unsafe { PyTuple_GetItem((*self_).co_localsplusnames, oparg as i64) };
    if name.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    Py_NewRef!(name)
}

// XXX code objects need to participate in GC?
/// sentinel
static mut code_methods: __SlateAlign16<[PyMethodDef; 8]> = __SlateAlign16([
    {
        let mut __t0: PyMethodDef = unsafe { core::mem::zeroed() };
        __t0.ml_name = (b"__sizeof__\0".as_ptr() as *mut i8) as *const i8;
        __t0.ml_meth = Some(code_sizeof);
        __t0.ml_flags = 4i32;
        __t0
    },
    {
        let mut __t1: PyMethodDef = unsafe { core::mem::zeroed() };
        __t1.ml_name = (b"co_lines\0".as_ptr() as *mut i8) as *const i8;
        __t1.ml_meth = Some(code_linesiterator);
        __t1.ml_flags = 4i32;
        __t1
    },
    {
        let mut __t2: PyMethodDef = unsafe { core::mem::zeroed() };
        __t2.ml_name = (b"co_branches\0".as_ptr() as *mut i8) as *const i8;
        __t2.ml_meth = Some(code_branchesiterator);
        __t2.ml_flags = 4i32;
        __t2
    },
    {
        let mut __t3: PyMethodDef = unsafe { core::mem::zeroed() };
        __t3.ml_name = (b"co_positions\0".as_ptr() as *mut i8) as *const i8;
        __t3.ml_meth = Some(code_positionsiterator);
        __t3.ml_flags = 4i32;
        __t3
    },
    {
        let mut __t4: PyMethodDef = unsafe { core::mem::zeroed() };
        __t4.ml_name = (b"replace\0".as_ptr() as *mut i8) as *const i8;
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
                            *mut _object,
                        ) -> *mut _object,
                    >,
                    Option<unsafe extern "C-unwind" fn()>,
                >(Some(code_replace))
            })
        };
        __t4.ml_flags = 130i32;
        __t4.ml_doc = unsafe { core::ptr::addr_of!(code_replace__doc__.0) as *const i8 };
        __t4
    },
    {
        let mut __t5: PyMethodDef = unsafe { core::mem::zeroed() };
        __t5.ml_name = (b"_varname_from_oparg\0".as_ptr() as *mut i8) as *const i8;
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
                            *mut _object,
                        ) -> *mut _object,
                    >,
                    Option<unsafe extern "C-unwind" fn()>,
                >(Some(code__varname_from_oparg))
            })
        };
        __t5.ml_flags = 130i32;
        __t5.ml_doc =
            unsafe { core::ptr::addr_of!(code__varname_from_oparg__doc__.0) as *const i8 };
        __t5
    },
    {
        let mut __t6: PyMethodDef = unsafe { core::mem::zeroed() };
        __t6.ml_name = (b"__replace__\0".as_ptr() as *mut i8) as *const i8;
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
                            *mut _object,
                        ) -> *mut _object,
                    >,
                    Option<unsafe extern "C-unwind" fn()>,
                >(Some(code_replace))
            })
        };
        __t6.ml_flags = 130i32;
        __t6.ml_doc = (b"__replace__($self, /, **changes)\n--\n\nThe same as replace().\0".as_ptr()
            as *mut i8) as *const i8;
        __t6
    },
    {
        let mut __t7: PyMethodDef = unsafe { core::mem::zeroed() };
        __t7.ml_name = core::ptr::null::<i8>();
        __t7.ml_meth = None;
        __t7
    },
]);

// other API
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
#[unsafe(no_mangle)]
extern "C-unwind" fn _PyCode_ConstantKey(op: *mut _object) -> *mut _object {
    let mut imag_negzero: i32 = unsafe { core::mem::zeroed() };
    let mut real_negzero: i32 = unsafe { core::mem::zeroed() };
    let mut z: core::mem::MaybeUninit<__SlateRecord_complexobject_h_8> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_3: *mut __SlateRecord_complexobject_h_8 =
        core::ptr::addr_of_mut!(z) as *mut __SlateRecord_complexobject_h_8;
    let mut item_key: *mut _object = unsafe { core::mem::zeroed() };
    let mut tuple: *mut _object = unsafe { core::mem::zeroed() };
    let mut len: i64 = unsafe { core::mem::zeroed() };
    let mut i: i64 = unsafe { core::mem::zeroed() };
    let mut item_key_1: *mut _object = unsafe { core::mem::zeroed() };
    let mut set: *mut _object = unsafe { core::mem::zeroed() };
    let mut tuple_1: *mut _object = unsafe { core::mem::zeroed() };
    let mut i_1: i64 = unsafe { core::mem::zeroed() };
    let mut hash: i64 = unsafe { core::mem::zeroed() };
    let mut item_1: *mut _object = unsafe { core::mem::zeroed() };
    let mut pos: i64 = unsafe { core::mem::zeroed() };
    let mut slice_key: *mut _object = unsafe { core::mem::zeroed() };
    let mut step_key: *mut _object = unsafe { core::mem::zeroed() };
    let mut stop_key: *mut _object = unsafe { core::mem::zeroed() };
    let mut start_key: *mut _object = unsafe { core::mem::zeroed() };
    let mut slice: *mut __SlateRecord_sliceobject_h_29 = unsafe { core::mem::zeroed() };
    // for other types, use the object identifier as a unique identifier
    // to ensure that they are seen as unequal.
    let mut obj_id: *mut _object = unsafe { core::mem::zeroed() };
    let mut __v27: bool = unsafe { core::mem::zeroed() };
    let mut __v28: bool = unsafe { core::mem::zeroed() };
    let mut __v29: bool = unsafe { core::mem::zeroed() };
    // Py_None and Py_Ellipsis are singletons.
    let mut __v30: bool = unsafe { core::mem::zeroed() };
    let mut key: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        if op == core::ptr::addr_of_mut!(_Py_NoneStruct)
            || op == core::ptr::addr_of_mut!(_Py_EllipsisObject)
        {
            __v30 = true;
        } else {
            __v30 = PyLong_CheckExact!(op) != 0;
        }
        if __v30 {
            __v29 = true;
        } else {
            __v29 = PyUnicode_CheckExact!(op) != 0;
        }
        if __v29 {
            __v28 = true;
        } else {
            __v28 = PyCode_Check!(op) != 0;
        }
        if __v28 {
            // Objects of these types are always different from object of other
            // type and from tuples.
            key = Py_NewRef!(op);
        } else {
            if PyBool_Check!(op) != 0 {
                __v27 = true;
            } else {
                __v27 = PyBytes_CheckExact!(op) != 0;
            }
            if __v27 {
                // Make booleans different from integers 0 and 1.
                // Avoid BytesWarning from comparing bytes with strings.
                key = PyTuple_Pack(2i64, Py_TYPE!(op), op);
            } else {
                if PyFloat_CheckExact!(op) != 0 {
                    let d: f64 = PyFloat_AS_DOUBLE!(op);
                    // all we need is to make the tuple different in either the 0.0
                    // or -0.0 case from all others, just to avoid the "coercion".
                    if d == 0.0f64 && copysign(1.0f64, d) < 0.0f64 {
                        key = PyTuple_Pack(
                            3i64,
                            Py_TYPE!(op),
                            op,
                            core::ptr::addr_of_mut!(_Py_NoneStruct),
                        );
                    } else {
                        key = PyTuple_Pack(2i64, Py_TYPE!(op), op);
                    }
                } else {
                    if PyComplex_CheckExact!(op) != 0 {
                        // For the complex case we must make complex(x, 0.)
                        // different from complex(x, -0.) and complex(0., y)
                        // different from complex(-0., y), for any x and y.
                        // All four complex zeros must be distinguished.
                        *__slate_slot_3 = PyComplex_AsCComplex(op);
                        real_negzero = ((*__slate_slot_3).real == 0.0f64
                            && copysign(1.0f64, (*__slate_slot_3).real) < 0.0f64)
                            as i32;
                        imag_negzero = ((*__slate_slot_3).imag == 0.0f64
                            && copysign(1.0f64, (*__slate_slot_3).imag) < 0.0f64)
                            as i32;
                        // use True, False and None singleton as tags for the real and imag
                        // sign, to make tuples different
                        if real_negzero != 0 && imag_negzero != 0 {
                            key = PyTuple_Pack(
                                3i64,
                                Py_TYPE!(op),
                                op,
                                _PyObject_CAST__2!(core::ptr::addr_of_mut!(_Py_TrueStruct)),
                            );
                        } else {
                            if imag_negzero != 0 {
                                key = PyTuple_Pack(
                                    3i64,
                                    Py_TYPE!(op),
                                    op,
                                    _PyObject_CAST__2!(core::ptr::addr_of_mut!(_Py_FalseStruct)),
                                );
                            } else {
                                if real_negzero != 0 {
                                    key = PyTuple_Pack(
                                        3i64,
                                        Py_TYPE!(op),
                                        op,
                                        core::ptr::addr_of_mut!(_Py_NoneStruct),
                                    );
                                } else {
                                    key = PyTuple_Pack(2i64, Py_TYPE!(op), op);
                                }
                            }
                        }
                    } else {
                        if PyTuple_CheckExact!(op) != 0 {
                            len = PyTuple_GET_SIZE!(op);
                            tuple = PyTuple_New(len);
                            if tuple.is_null() {
                                return core::ptr::null_mut::<_object>();
                            } else {
                                i = 0;
                                while i < len {
                                    let item: *mut _object = PyTuple_GET_ITEM!(op, i);
                                    item_key = _PyCode_ConstantKey(item);
                                    if item_key.is_null() {
                                        Py_DECREF!(tuple);
                                        return core::ptr::null_mut::<_object>();
                                    } else {
                                        PyTuple_SET_ITEM__2!(tuple, i, item_key);
                                        i = i.wrapping_add(1);
                                    }
                                }
                                key = PyTuple_Pack(2i64, tuple, op);
                                Py_DECREF!(tuple);
                            }
                        } else {
                            if PyFrozenSet_CheckExact!(op) != 0 {
                                pos = 0;
                                let len: i64 = PySet_GET_SIZE!(op);
                                tuple_1 = PyTuple_New(len);
                                if tuple_1.is_null() {
                                    return core::ptr::null_mut::<_object>();
                                } else {
                                    i_1 = 0;
                                    while _PySet_NextEntry(
                                        op,
                                        core::ptr::addr_of_mut!(pos),
                                        core::ptr::addr_of_mut!(item_1),
                                        core::ptr::addr_of_mut!(hash),
                                    ) != 0
                                    {
                                        item_key_1 = _PyCode_ConstantKey(item_1);
                                        if item_key_1.is_null() {
                                            Py_DECREF!(tuple_1);
                                            return core::ptr::null_mut::<_object>();
                                        } else {
                                            PyTuple_SET_ITEM__2!(tuple_1, i_1, item_key_1);
                                            i_1 = i_1.wrapping_add(1);
                                        }
                                    }
                                    set = PyFrozenSet_New(tuple_1);
                                    Py_DECREF!(tuple_1);
                                    if set.is_null() {
                                        return core::ptr::null_mut::<_object>();
                                    } else {
                                        key = PyTuple_Pack(2i64, set, op);
                                        Py_DECREF!(set);
                                        return key;
                                    }
                                }
                            } else {
                                if PySlice_Check!(op) != 0 {
                                    slice = op as *mut __SlateRecord_sliceobject_h_29;
                                    start_key = core::ptr::null_mut::<_object>();
                                    stop_key = core::ptr::null_mut::<_object>();
                                    step_key = core::ptr::null_mut::<_object>();
                                    key = core::ptr::null_mut::<_object>();
                                    start_key = _PyCode_ConstantKey((*slice).start);
                                    if !start_key.is_null() {
                                        stop_key = _PyCode_ConstantKey((*slice).stop);
                                        if !stop_key.is_null() {
                                            step_key = _PyCode_ConstantKey((*slice).step);
                                            if !step_key.is_null() {
                                                slice_key =
                                                    PySlice_New(start_key, stop_key, step_key);
                                                if !slice_key.is_null() {
                                                    key = PyTuple_Pack(2i64, slice_key, op);
                                                    Py_DECREF!(slice_key);
                                                }
                                            }
                                        }
                                    }
                                    Py_XDECREF!(start_key);
                                    Py_XDECREF!(stop_key);
                                    Py_XDECREF!(step_key);
                                } else {
                                    obj_id = PyLong_FromVoidPtr(op as *mut ());
                                    if obj_id.is_null() {
                                        return core::ptr::null_mut::<_object>();
                                    } else {
                                        key = PyTuple_Pack(2i64, obj_id, op);
                                        Py_DECREF!(obj_id);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        // code_richcompare() uses _PyCode_ConstantKey() internally
        return key;
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyCode_Init(interp: *mut _is) -> __SlateRecord_initconfig_h_19 {
    let mut __slate_compound_0: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
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
extern "C-unwind" fn _PyCode_Fini(interp: *mut _is) {}
