//! List object implementation
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
//! Long value tag bits:
//! 0-1: Sign bits value = (1-sign), ie. negative=2, positive=0, zero=1.
//! 2: Set to 1 for the small ints
//! 3+ Unsigned digit count
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
macro_rules! BITS_TO_PTR_MASKED {
    ($REF:expr) => {
        ((unsafe { $REF.bits }) & 18446744073709551614u64) as *mut _object
    };
}

macro_rules! BITS_TO_PTR {
    ($REF:expr) => {
        (unsafe { $REF.bits }) as *mut _object
    };
}

macro_rules! FT_ATOMIC_STORE_PTR_RELAXED {
    ($value:expr, $new_value:expr) => {
        unsafe {
            $value = $new_value;
        }
    };
}

macro_rules! FT_ATOMIC_STORE_PTR_RELEASE__3 {
    ($value:expr) => {
        unsafe {
            $value = core::ptr::null_mut::<*mut _object>();
        }
    };
}

macro_rules! FT_ATOMIC_STORE_PTR_RELEASE {
    ($value:expr, $new_value:expr) => {
        unsafe {
            $value = $new_value;
        }
    };
}

macro_rules! FT_ATOMIC_STORE_SSIZE_RELAXED__2 {
    ($value:expr, $new_value:expr) => {
        unsafe {
            $value = ($new_value) as i64;
        }
    };
}

macro_rules! FT_ATOMIC_STORE_SSIZE_RELAXED {
    ($value:expr, $new_value:expr) => {
        unsafe {
            $value = $new_value;
        }
    };
}

macro_rules! ISLT {
    ($X:expr, $Y:expr, $ms:expr) => {
        unsafe { unsafe { (*$ms).key_compare }.unwrap()($X, $Y, $ms) }
    };
}

macro_rules! MERGE_GETMEM {
    ($MS:expr, $NEED:expr) => {{
        let __v: i32 = if ($NEED) <= unsafe { (*$MS).alloced } {
            0
        } else {
            merge_getmem($MS, $NEED)
        };
        __v
    }};
}

macro_rules! OBJECT_STAT_INC {
    () => {};
}

macro_rules! PyAnySet_CheckExact {
    ($ob:expr) => {{
        let __v: bool = Py_IS_TYPE!($ob, unsafe { core::ptr::addr_of_mut!(PySet_Type) }) != 0
            || Py_IS_TYPE!($ob, unsafe { core::ptr::addr_of_mut!(PyFrozenSet_Type) }) != 0;
        __v
    }};
}

macro_rules! PyBool_Check {
    ($x:expr) => {
        Py_IS_TYPE!($x, unsafe { core::ptr::addr_of_mut!(PyBool_Type) })
    };
}

macro_rules! PyDict_CheckExact {
    ($op:expr) => {
        Py_IS_TYPE!($op, unsafe { core::ptr::addr_of_mut!(PyDict_Type) })
    };
}

macro_rules! PyDict_GET_SIZE__2 {
    ($op:expr) => {
        PyDict_GET_SIZE(_PyObject_CAST__2!($op))
    };
}

/// Mask the old API with a call to the new API for code compiled under
/// Python 2.0:
macro_rules! PyErr_BadInternalCall__4 {
    () => {
        unsafe {
            _PyErr_BadInternalCall(
                c"/home/takashi/c-corpus/cpython/Objects/listobject.c".as_ptr(),
                3204i32,
            )
        };
    };
}

/// Mask the old API with a call to the new API for code compiled under
/// Python 2.0:
macro_rules! PyErr_BadInternalCall__6 {
    () => {
        unsafe {
            _PyErr_BadInternalCall(
                c"/home/takashi/c-corpus/cpython/Objects/listobject.c".as_ptr(),
                1026i32,
            )
        };
    };
}

/// Mask the old API with a call to the new API for code compiled under
/// Python 2.0:
macro_rules! PyErr_BadInternalCall__15 {
    () => {
        unsafe {
            _PyErr_BadInternalCall(
                c"/home/takashi/c-corpus/cpython/Objects/listobject.c".as_ptr(),
                717i32,
            )
        };
    };
}

/// Mask the old API with a call to the new API for code compiled under
/// Python 2.0:
macro_rules! PyErr_BadInternalCall__28 {
    () => {
        unsafe {
            _PyErr_BadInternalCall(
                c"/home/takashi/c-corpus/cpython/Objects/listobject.c".as_ptr(),
                3170i32,
            )
        };
    };
}

/// Mask the old API with a call to the new API for code compiled under
/// Python 2.0:
macro_rules! PyErr_BadInternalCall__31 {
    () => {
        unsafe {
            _PyErr_BadInternalCall(
                c"/home/takashi/c-corpus/cpython/Objects/listobject.c".as_ptr(),
                238i32,
            )
        };
    };
}

/// Mask the old API with a call to the new API for code compiled under
/// Python 2.0:
macro_rules! PyErr_BadInternalCall__40 {
    () => {
        unsafe {
            _PyErr_BadInternalCall(
                c"/home/takashi/c-corpus/cpython/Objects/listobject.c".as_ptr(),
                481i32,
            )
        };
    };
}

/// Mask the old API with a call to the new API for code compiled under
/// Python 2.0:
macro_rules! PyErr_BadInternalCall__52 {
    () => {
        unsafe {
            _PyErr_BadInternalCall(
                c"/home/takashi/c-corpus/cpython/Objects/listobject.c".as_ptr(),
                453i32,
            )
        };
    };
}

/// Mask the old API with a call to the new API for code compiled under
/// Python 2.0:
macro_rules! PyErr_BadInternalCall__65 {
    () => {
        unsafe {
            _PyErr_BadInternalCall(
                c"/home/takashi/c-corpus/cpython/Objects/listobject.c".as_ptr(),
                1487i32,
            )
        };
    };
}

/// Mask the old API with a call to the new API for code compiled under
/// Python 2.0:
macro_rules! PyErr_BadInternalCall__75 {
    () => {
        unsafe {
            _PyErr_BadInternalCall(
                c"/home/takashi/c-corpus/cpython/Objects/listobject.c".as_ptr(),
                3985i32,
            )
        };
    };
}

/// Mask the old API with a call to the new API for code compiled under
/// Python 2.0:
macro_rules! PyErr_BadInternalCall__86 {
    () => {
        unsafe {
            _PyErr_BadInternalCall(
                c"/home/takashi/c-corpus/cpython/Objects/listobject.c".as_ptr(),
                3219i32,
            )
        };
    };
}

/// Mask the old API with a call to the new API for code compiled under
/// Python 2.0:
macro_rules! PyErr_BadInternalCall__91 {
    () => {
        unsafe {
            _PyErr_BadInternalCall(
                c"/home/takashi/c-corpus/cpython/Objects/listobject.c".as_ptr(),
                1498i32,
            )
        };
    };
}

/// Mask the old API with a call to the new API for code compiled under
/// Python 2.0:
macro_rules! PyErr_BadInternalCall__98 {
    () => {
        unsafe {
            _PyErr_BadInternalCall(
                c"/home/takashi/c-corpus/cpython/Objects/listobject.c".as_ptr(),
                388i32,
            )
        };
    };
}

/// Mask the old API with a call to the new API for code compiled under
/// Python 2.0:
macro_rules! PyErr_BadInternalCall__114 {
    () => {
        unsafe {
            _PyErr_BadInternalCall(
                c"/home/takashi/c-corpus/cpython/Objects/listobject.c".as_ptr(),
                507i32,
            )
        };
    };
}

/// Mask the old API with a call to the new API for code compiled under
/// Python 2.0:
macro_rules! PyErr_BadInternalCall__115 {
    () => {
        unsafe {
            _PyErr_BadInternalCall(
                c"/home/takashi/c-corpus/cpython/Objects/listobject.c".as_ptr(),
                542i32,
            )
        };
    };
}

/// Mask the old API with a call to the new API for code compiled under
/// Python 2.0:
macro_rules! PyErr_BadInternalCall__143 {
    () => {
        unsafe {
            _PyErr_BadInternalCall(
                c"/home/takashi/c-corpus/cpython/Objects/listobject.c".as_ptr(),
                306i32,
            )
        };
    };
}

macro_rules! PyFloat_AS_DOUBLE {
    ($op:expr) => {
        PyFloat_AS_DOUBLE(_PyObject_CAST!($op))
    };
}

macro_rules! PyList_CheckExact__3 {
    ($op:expr) => {
        Py_IS_TYPE__2!($op, unsafe { core::ptr::addr_of_mut!(PyList_Type) })
    };
}

macro_rules! PyList_CheckExact {
    ($op:expr) => {
        Py_IS_TYPE!($op, unsafe { core::ptr::addr_of_mut!(PyList_Type) })
    };
}

macro_rules! PyList_Check {
    ($op:expr) => {
        PyType_FastSubclass!(Py_TYPE!($op), 33554432u64)
    };
}

macro_rules! PyList_GET_ITEM__2 {
    ($op:expr, $index:expr) => {
        unsafe { *unsafe { (*_PyList_CAST__2!($op)).ob_item }.wrapping_offset(($index) as isize) }
    };
}

macro_rules! PyList_GET_SIZE {
    ($op:expr) => {
        PyList_GET_SIZE(_PyObject_CAST!($op))
    };
}

macro_rules! PyList_GET_SIZE__2 {
    ($op:expr) => {
        PyList_GET_SIZE(_PyObject_CAST__2!($op))
    };
}

macro_rules! PyList_SET_ITEM__3 {
    ($op:expr, $index:expr, $value:expr) => {
        PyList_SET_ITEM(_PyObject_CAST__2!($op), $index, _PyObject_CAST!($value));
    };
}

macro_rules! PyObject_GC_New {
    ($type:ty, $typeobj:expr) => {
        _Py_CAST!(*mut $type, unsafe { _PyObject_GC_New($typeobj) })
    };
}

/// Return the size of the sequence 'o', assuming that 'o' was returned by
/// PySequence_Fast and is not NULL.
macro_rules! PySequence_Fast_GET_SIZE {
    ($o:expr) => {{
        let __v: i64 = if PyList_Check!($o) != 0 {
            PyList_GET_SIZE!($o)
        } else {
            PyTuple_GET_SIZE!($o)
        };
        __v
    }};
}

/// Return a pointer to the underlying item array for
/// an object returned by PySequence_Fast
macro_rules! PySequence_Fast_ITEMS {
    ($sf:expr) => {
        if PyList_Check!($sf) != 0 {
            unsafe { (*(($sf) as *mut __SlateRecord_listobject_h_22)).ob_item }
        } else {
            unsafe {
                (*(($sf) as *mut __SlateRecord_tupleobject_h_13))
                    .ob_item
                    .as_mut_ptr() as *mut *mut _object
            }
        }
    };
}

macro_rules! PySet_GET_SIZE__2 {
    ($so:expr) => {
        PySet_GET_SIZE(_PyObject_CAST__2!($so))
    };
}

macro_rules! PySlice_Check {
    ($op:expr) => {
        Py_IS_TYPE!($op, unsafe { core::ptr::addr_of_mut!(PySlice_Type) })
    };
}

macro_rules! PyTuple_CheckExact {
    ($op:expr) => {
        Py_IS_TYPE!($op, unsafe { core::ptr::addr_of_mut!(PyTuple_Type) })
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

macro_rules! PyVectorcall_NARGS {
    ($n:expr) => {
        _PyVectorcall_NARGS($n)
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

/// Minimum value between x and y
macro_rules! Py_MIN__3 {
    ($x:expr, $y:expr) => {{
        let __v: i64 = if ($x) > $y { $y } else { $x };
        __v
    }};
}

/// Minimum value between x and y
macro_rules! Py_MIN {
    ($x:expr, $y:expr) => {
        if ($x) > $y { $y } else { $x }
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

macro_rules! Py_SET_SIZE {
    ($ob:expr, $size:expr) => {
        Py_SET_SIZE(_PyVarObject_CAST!($ob), $size);
    };
}

macro_rules! Py_SET_SIZE__2 {
    ($ob:expr, $size:expr) => {
        Py_SET_SIZE(_PyVarObject_CAST!($ob), ($size) as i64);
    };
}

macro_rules! Py_SIZE {
    ($ob:expr) => {
        Py_SIZE(_PyObject_CAST__2!($ob))
    };
}

macro_rules! Py_SIZE__2 {
    ($ob:expr) => {
        Py_SIZE(_PyObject_CAST!($ob))
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
macro_rules! Py_VISIT__11 {
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

/// Cast argument to PyListObject* type.
macro_rules! _PyList_CAST {
    ($op:expr) => {
        _Py_CAST!(*mut __SlateRecord_listobject_h_22, $op)
    };
}

/// Cast argument to PyListObject* type.
macro_rules! _PyList_CAST__2 {
    ($op:expr) => {
        _Py_CAST__2!($op)
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

macro_rules! _PyObject_GC_UNTRACK__2 {
    ($op:expr) => {
        _PyObject_GC_UNTRACK(_PyObject_CAST__2!($op));
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

/// Prevent using an expression as a l-value.
///
/// For example, "int x; _Py_RVALUE(x) = 1;" fails with a compiler error.
macro_rules! _Py_RVALUE {
    ($EXPR:expr) => {
        $EXPR
    };
}

macro_rules! _Py_RefcntAdd {
    ($op:expr, $n:expr) => {
        _Py_RefcntAdd(_PyObject_CAST!($op), $n);
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
    static mut PyUnicode_Type: _typeobject;
    static mut PyExc_StopIteration: *mut _object;
    static mut PyExc_IndexError: *mut _object;
    static mut PyExc_TypeError: *mut _object;
    static mut PyExc_ValueError: *mut _object;
    static mut _Py_FalseStruct: _longobject;
    static mut _Py_TrueStruct: _longobject;
    static mut PyFloat_Type: _typeobject;
    static mut PyTuple_Type: _typeobject;
    static mut PyDict_Type: _typeobject;
    static mut PyDictKeys_Type: _typeobject;
    static mut PyDictValues_Type: _typeobject;
    static mut PyDictItems_Type: _typeobject;
    static mut PySet_Type: _typeobject;
    static mut PyFrozenSet_Type: _typeobject;
    static mut PySlice_Type: _typeobject;
    #[thread_local]
    static mut _Py_tss_tstate: *mut _ts;
    static mut _PyRuntime: pyruntimestate;
    fn index(arg0: *const i8, arg1: i32) -> *mut i8;
    fn PyMem_Malloc(arg0: u64) -> *mut ();
    fn PyMem_Calloc(arg0: u64, arg1: u64) -> *mut ();
    fn PyMem_Realloc(arg0: *mut (), arg1: u64) -> *mut ();
    fn PyMem_Free(arg0: *mut ());
    fn PyType_GenericAlloc(arg0: *mut _typeobject, arg1: i64) -> *mut _object;
    fn PyType_GenericNew(
        arg0: *mut _typeobject,
        arg1: *mut _object,
        arg2: *mut _object,
    ) -> *mut _object;
    fn PyObject_RichCompare(arg0: *mut _object, arg1: *mut _object, arg2: i32) -> *mut _object;
    fn PyObject_RichCompareBool(arg0: *mut _object, arg1: *mut _object, arg2: i32) -> i32;
    fn PyObject_SelfIter(arg0: *mut _object) -> *mut _object;
    fn PyObject_GenericGetAttr(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn PyObject_HashNotImplemented(arg0: *mut _object) -> i64;
    fn PyObject_IsTrue(arg0: *mut _object) -> i32;
    fn Py_ReprEnter(arg0: *mut _object) -> i32;
    fn Py_ReprLeave(arg0: *mut _object);
    fn _Py_NewReference(arg0: *mut _object);
    fn _Py_Dealloc(arg0: *mut _object);
    fn _PyObject_GC_New(arg0: *mut _typeobject) -> *mut _object;
    fn PyObject_GC_Track(arg0: *mut ());
    fn PyObject_GC_UnTrack(arg0: *mut ());
    fn PyObject_GC_Del(arg0: *mut ());
    fn PyUnicode_FromString(arg0: *const i8) -> *mut _object;
    fn PyUnicodeWriter_Create(arg0: i64) -> *mut PyUnicodeWriter;
    fn PyUnicodeWriter_Discard(arg0: *mut PyUnicodeWriter);
    fn PyUnicodeWriter_Finish(arg0: *mut PyUnicodeWriter) -> *mut _object;
    fn PyUnicodeWriter_WriteChar(arg0: *mut PyUnicodeWriter, arg1: u32) -> i32;
    fn PyUnicodeWriter_WriteRepr(arg0: *mut PyUnicodeWriter, arg1: *mut _object) -> i32;
    fn PyErr_SetObject(arg0: *mut _object, arg1: *mut _object);
    fn PyErr_SetString(arg0: *mut _object, arg1: *const i8);
    fn PyErr_Occurred() -> *mut _object;
    fn PyErr_Clear();
    fn PyErr_ExceptionMatches(arg0: *mut _object) -> i32;
    fn PyErr_NoMemory() -> *mut _object;
    fn PyErr_Format(arg0: *mut _object, arg1: *const i8, ...) -> *mut _object;
    fn _PyErr_BadInternalCall(arg0: *const i8, arg1: i32);
    fn PyLong_FromLong(arg0: i64) -> *mut _object;
    fn PyLong_FromSize_t(arg0: u64) -> *mut _object;
    fn PyLong_FromSsize_t(arg0: i64) -> *mut _object;
    fn PyLong_AsSsize_t(arg0: *mut _object) -> i64;
    fn PyTuple_New(arg0: i64) -> *mut _object;
    fn PyTuple_Pack(arg0: i64, ...) -> *mut _object;
    fn PySlice_Unpack(arg0: *mut _object, arg1: *mut i64, arg2: *mut i64, arg3: *mut i64) -> i32;
    fn PySlice_AdjustIndices(arg0: i64, arg1: *mut i64, arg2: *mut i64, arg3: i64) -> i64;
    fn Py_GenericAlias(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn Py_BuildValue(arg0: *const i8, ...) -> *mut _object;
    fn _PyEval_SliceIndexNotNone(arg0: *mut _object, arg1: *mut i64) -> i32;
    fn PyObject_GetIter(arg0: *mut _object) -> *mut _object;
    fn PyNumber_AsSsize_t(arg0: *mut _object, arg1: *mut _object) -> i64;
    fn PySequence_Fast(arg0: *mut _object, arg1: *const i8) -> *mut _object;
    fn PyObject_CallOneArg(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn PyObject_LengthHint(arg0: *mut _object, arg1: i64) -> i64;
    fn _PyNumber_Index(arg0: *mut _object) -> *mut _object;
    fn _PyEval_GetBuiltin(arg0: *mut _object) -> *mut _object;
    fn _PyDebugAllocatorStats(
        arg0: *mut slate_support::FILE,
        arg1: *const i8,
        arg2: i32,
        arg3: u64,
    );
    fn _PyDict_Next(
        arg0: *mut _object,
        arg1: *mut i64,
        arg2: *mut *mut _object,
        arg3: *mut *mut _object,
        arg4: *mut i64,
    ) -> i32;
    fn _PyArg_NoKwnames(arg0: *const i8, arg1: *mut _object) -> i32;
    fn _PyArg_NoKeywords(arg0: *const i8, arg1: *mut _object) -> i32;
    fn _PyArg_CheckPositional(arg0: *const i8, arg1: i64, arg2: i64, arg3: i64) -> i32;
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
    fn _PyTuple_FromArray(arg0: *const *mut _object, arg1: i64) -> *mut _object;
    fn _PyTuple_FromArraySteal(arg0: *const *mut _object, arg1: i64) -> *mut _object;
    fn _PySet_NextEntryRef(
        arg0: *mut _object,
        arg1: *mut i64,
        arg2: *mut *mut _object,
        arg3: *mut i64,
    ) -> i32;
    fn __builtin_unreachable();
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
struct _frame {}

fn Py_SET_SIZE(ob: *mut __SlateRecord_object_h_172, size: i64) {
    unsafe {
        (*ob).ob_size = size;
    }
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
// Set if instances of the type object are treated as sequences for pattern matching
// Set if the type allows subclassing
// Objects support garbage collection (see objimpl.h)
// This undocumented flag gives certain built-ins their unique pattern-matching
//
// behavior, which allows a single positional subpattern to match against the
//
// subject itself (rather than a mapped attribute on it):
// Rich comparison opcodes
// Macro for implementing rich comparisons
//
// Needs to be a macro because any C-comparable type can be used.
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

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_unicodeobject_h_163_2 {
    __slate_bits_0: __slate_bits::__SlateBits142U0,
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
union __SlateRecord_unicodeobject_h_184_2 {
    any: *mut (),
    latin1: *mut u8,
    ucs2: *mut u16,
    ucs4: *mut u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct PyUnicodeWriter {}

fn _PyObject_SIZE(r#type: *mut _typeobject) -> u64 {
    _Py_STATIC_CAST!(u64, unsafe { (*r#type).tp_basicsize })
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
struct _PyExecutorObject {}

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

fn _PyLong_IsCompact(op: *const _longobject) -> i32 {
    ((unsafe { (*op).long_value.lv_tag }) < 16u64) as i32
}

fn _PyLong_CompactValue(op: *const _longobject) -> i64 {
    let mut sign: i64 = 0;
    sign = 1u64.wrapping_sub((unsafe { (*op).long_value.lv_tag }) & 3) as i64;
    sign.wrapping_mul(
        (unsafe { *((*op).long_value.ob_digit.as_ptr() as *const u32).wrapping_offset(0 as isize) })
            as i64,
    )
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

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pystate_h_78 {
    __slate_bits_0: __slate_bits::__SlateBits205U0,
}

fn PyTuple_GET_SIZE(op: *mut _object) -> i64 {
    let tuple: *mut __SlateRecord_tupleobject_h_13 = _PyTuple_CAST!(op);
    Py_SIZE!(tuple)
}

#[unsafe(no_mangle)]
static mut PyList_Type: _typeobject = {
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
    __t0.tp_name = (b"list\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_listobject_h_22>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(list_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = Some(list_repr);
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = unsafe { core::ptr::addr_of_mut!(list_as_sequence) };
    __t0.tp_as_mapping = unsafe { core::ptr::addr_of_mut!(list_as_mapping) };
    __t0.tp_hash = unsafe {
        core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut _object) -> i64>>(
            PyObject_HashNotImplemented as *const (),
        )
    };
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
    __t0.tp_flags = 37766176u64;
    __t0.tp_doc = unsafe { core::ptr::addr_of!(list___init____doc__.0) as *const i8 };
    __t0.tp_traverse = Some(list_traverse);
    __t0.tp_clear = Some(list_clear_slot);
    __t0.tp_richcompare = Some(list_richcompare);
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = Some(list_iter);
    __t0.tp_iternext = None;
    __t0.tp_methods = unsafe { core::ptr::addr_of_mut!(list_methods.0) as *mut PyMethodDef };
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = core::ptr::null_mut::<_typeobject>();
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = 0i64;
    __t0.tp_init = Some(list___init__);
    __t0.tp_alloc = unsafe {
        core::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut _typeobject, i64) -> *mut _object>,
        >(PyType_GenericAlloc as *const ())
    };
    __t0.tp_new = unsafe {
        core::mem::transmute::<
            *const (),
            Option<
                unsafe extern "C-unwind" fn(
                    *mut _typeobject,
                    *mut _object,
                    *mut _object,
                ) -> *mut _object,
            >,
        >(PyType_GenericNew as *const ())
    };
    __t0.tp_free = unsafe {
        core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
            PyObject_GC_Del as *const (),
        )
    };
    __t0.tp_version_tag = 3u32;
    __t0.tp_vectorcall = Some(list_vectorcall);
    __t0
};

#[unsafe(no_mangle)]
static mut PyListIter_Type: _typeobject = {
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
    __t0.tp_name = (b"list_iterator\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pycore_list_h_71>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(listiter_dealloc);
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
    __t0.tp_getattro = unsafe {
        core::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
        >(PyObject_GenericGetAttr as *const ())
    };
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 16384u64;
    __t0.tp_doc = core::ptr::null::<i8>();
    __t0.tp_traverse = Some(listiter_traverse);
    __t0.tp_clear = None;
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = unsafe {
        core::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
        >(PyObject_SelfIter as *const ())
    };
    __t0.tp_iternext = Some(listiter_next);
    __t0.tp_methods = unsafe { core::ptr::addr_of_mut!(listiter_methods.0) as *mut PyMethodDef };
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0
};

#[unsafe(no_mangle)]
static mut PyListRevIter_Type: _typeobject = {
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
    __t0.tp_name = (b"list_reverseiterator\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_listobject_c_4085>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(listreviter_dealloc);
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
    __t0.tp_getattro = unsafe {
        core::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
        >(PyObject_GenericGetAttr as *const ())
    };
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = core::ptr::null_mut::<__SlateRecord_object_h_140>();
    __t0.tp_flags = 16384u64;
    __t0.tp_doc = core::ptr::null::<i8>();
    __t0.tp_traverse = Some(listreviter_traverse);
    __t0.tp_clear = None;
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = unsafe {
        core::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
        >(PyObject_SelfIter as *const ())
    };
    __t0.tp_iternext = Some(listreviter_next);
    __t0.tp_methods = unsafe { core::ptr::addr_of_mut!(listreviter_methods.0) as *mut PyMethodDef };
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0
};

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
struct __SlateRecord_pycore_structs_h_32_2 {
    code: u8,
    arg: u8,
}

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
struct code_arena_st {}

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

fn _PyIndex_Check(obj: *mut _object) -> i32 {
    let tp_as_number: *mut __SlateRecord_object_h_106 = unsafe { (*Py_TYPE!(obj)).tp_as_number };
    (!tp_as_number.is_null() && (unsafe { (*tp_as_number).nb_index }) != None) as i32
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
struct __SlateRecord_pycore_pythread_h_64 {
    ptr: *mut __SlateRecord_pthreadtypes_h_45,
    val: __SlateRecord_pthreadtypes_h_45,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_context_h_21 {
    ob_base: _object,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_interp_structs_h_297 {
    mutex: PyMutex,
    hashtable: *mut _Py_hashtable_t,
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
struct __SlateRecord_pycore_interp_structs_h_348 {
    import_level: i32,
    accumulated: i64,
    header: i32,
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
struct xi_exceptions {
    PyExc_InterpreterError: *mut _object,
    PyExc_InterpreterNotFoundError: *mut _object,
    PyExc_NotShareableError: *mut _object,
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
struct __SlateRecord_pycore_interp_structs_h_718 {
    _not_used: i32,
    _hamt_empty_gc_not_used: __SlateRecord_pycore_interp_structs_h_170,
    hamt_empty: __SlateRecord_pycore_structs_h_47,
    last_resort_memory_error: __SlateRecord_pyerrors_h_15,
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

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord_pycore_obmalloc_h_255 {
    _padding: *mut u8,
    count: u32,
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
struct __SlateRecord_pycore_runtime_structs_h_49 {
    fd: i32,
    st_dev: u64,
    st_ino: u64,
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
struct __SlateRecord_pycore_tracemalloc_h_70 {
    mem: __SlateRecord_pymem_h_48,
    raw: __SlateRecord_pymem_h_48,
    obj: __SlateRecord_pymem_h_48,
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
struct _Py_interp_static_objects {
    singletons: __SlateRecord_pycore_interp_structs_h_718,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord_sigaction_h_31 {
    sa_handler: Option<unsafe extern "C-unwind" fn(i32)>,
    sa_sigaction:
        Option<unsafe extern "C-unwind" fn(i32, *mut __SlateRecord_siginfo_t_h_124, *mut ())>,
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

fn _PyInterpreterState_GET() -> *mut _is {
    let tstate: *mut _ts = _PyThreadState_GET();
    unsafe { (*tstate).interp }
}

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
struct __SlateRecord_pycore_obmalloc_h_622 {
    tail_hi: i32,
    tail_lo: i32,
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
struct __SlateRecord_listobject_c_h_229 {
    _this_is_not_used: __SlateRecord_pycore_interp_structs_h_170,
    ob_base: __SlateRecord_object_h_172,
    ob_hash: i64,
    ob_item: [*mut _object; 2],
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
    pub struct __SlateBits142U0 {
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
    pub struct __SlateBits205U0 {
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

use slate_support::memcmp;
use slate_support::memcpy;
use slate_support::memmove;

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

fn _Py_RefcntAdd(op: *mut _object, n: i64) {
    if _Py_IsImmortal!(op) != 0 {
        _Py_INCREF_IMMORTAL_STAT_INC!();
        return;
    }
    let refcnt: i64 = _Py_REFCNT(op);
    let mut new_refcnt: i64 = refcnt.wrapping_add(n);
    if new_refcnt >= 2147483648i64 {
        new_refcnt = 3221225472i64;
    }
    unsafe {
        (*op).__slate_anon_0.__slate_anon_1.ob_refcnt = new_refcnt as u32;
    }
    // Although the ref count was increased by `n` (which may be greater than 1)
    //
    // it is only a single increment (i.e. addition) operation, so only 1 refcnt
    //
    // increment operation is counted.
    _Py_INCREF_STAT_INC!();
}

// Fast inlined version of PyType_IS_GC()
fn PyStackRef_RefcountOnObject(r#ref: _PyStackRef) -> i32 {
    ((unsafe { r#ref.bits }) & 1 == 0) as i32
}

fn PyStackRef_AsPyObjectSteal(r#ref: _PyStackRef) -> *mut _object {
    if PyStackRef_RefcountOnObject(r#ref) != 0 {
        return BITS_TO_PTR!(r#ref);
    } else {
        return Py_NewRef!(BITS_TO_PTR_MASKED!(r#ref));
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_dict_h_73 {
    ob_base: _object,
    dv_dict: *mut __SlateRecord_dictobject_h_33,
}

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

fn _Py_freelists_GET() -> *mut _Py_freelists {
    let tstate: *mut _ts = _PyThreadState_GET();
    unsafe { core::ptr::addr_of_mut!((*(*tstate).interp).object_state.freelists) }
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

fn _PyFreeList_Pop(fl: *mut _Py_freelist) -> *mut _object {
    let op: *mut _object = _PyFreeList_PopNoStats(fl) as *mut _object;
    if !op.is_null() {
        OBJECT_STAT_INC!();
        unsafe { _Py_NewReference(op) };
    }
    op
}

fn _PyList_AppendTakeRef(self_: *mut __SlateRecord_listobject_h_22, newitem: *mut _object) -> i32 {
    let len: i64 = Py_SIZE!(self_);
    let allocated: i64 = unsafe { (*self_).allocated };
    if allocated > len {
        PyList_SET_ITEM__3!(self_, len, newitem);
        Py_SET_SIZE!(self_, len.wrapping_add(1));
        return 0;
    }
    _PyList_AppendTakeRefListResize(self_, newitem)
}

fn _Py_memory_repeat(dest: *mut i8, len_dest: i64, mut copied: i64) {
    while copied < len_dest {
        let bytes_to_copy: i64 = Py_MIN!(copied, len_dest.wrapping_sub(copied));
        unsafe {
            core::ptr::copy_nonoverlapping(
                ((dest as *const ()) as *const core::ffi::c_void) as *const u8,
                ((dest.wrapping_offset(copied as isize) as *mut ()) as *mut core::ffi::c_void)
                    as *mut u8,
                ((bytes_to_copy as u64) as usize) as usize,
            )
        };
        copied = copied.wrapping_add(bytes_to_copy);
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_list_h_71 {
    ob_base: _object,
    it_index: i64,
    it_seq: *mut __SlateRecord_listobject_h_22,
}

static list_insert__doc__: __SlateAlign16<[i8; 64]> = __SlateAlign16([
    105i8, 110i8, 115i8, 101i8, 114i8, 116i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8,
    105i8, 110i8, 100i8, 101i8, 120i8, 44i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 44i8,
    32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 73i8, 110i8, 115i8, 101i8, 114i8, 116i8, 32i8,
    111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 32i8, 98i8, 101i8, 102i8, 111i8, 114i8, 101i8, 32i8,
    105i8, 110i8, 100i8, 101i8, 120i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.listobject.list_insert")]
extern "C-unwind" fn list_insert(
    self_: *mut _object,
    args: *const *mut _object,
    nargs: i64,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut iobj: *mut _object = unsafe { core::mem::zeroed() };
    let mut ival: i64 = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        if _PyArg_CheckPositional!(c"insert".as_ptr() as *mut i8, nargs, 2i32, 2i32) {
            ival = 1i32.wrapping_neg() as i64;
            iobj = _PyNumber_Index(*args.wrapping_offset(0 as isize));
            if !iobj.is_null() {
                ival = PyLong_AsSsize_t(iobj);
                Py_DECREF!(iobj);
            }
            if ival == (1i32.wrapping_neg() as i64) {
                __v0 = !PyErr_Occurred().is_null();
            } else {
                __v0 = false;
            }
            if !__v0 {
                let index: i64 = ival;
                let object: *mut _object = *args.wrapping_offset(1 as isize);
                return_value =
                    list_insert_impl(self_ as *mut __SlateRecord_listobject_h_22, index, object);
            }
        }
        return return_value;
    }
}

static py_list_clear__doc__: __SlateAlign16<[i8; 48]> = __SlateAlign16([
    99i8, 108i8, 101i8, 97i8, 114i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8, 47i8,
    41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 109i8, 111i8, 118i8, 101i8, 32i8, 97i8, 108i8,
    108i8, 32i8, 105i8, 116i8, 101i8, 109i8, 115i8, 32i8, 102i8, 114i8, 111i8, 109i8, 32i8, 108i8,
    105i8, 115i8, 116i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.listobject.py_list_clear")]
extern "C-unwind" fn py_list_clear(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    let mut return_value: *mut _object = core::ptr::null_mut::<_object>();
    return_value = py_list_clear_impl(self_ as *mut __SlateRecord_listobject_h_22);
    return_value
}

static list_copy__doc__: __SlateAlign16<[i8; 54]> = __SlateAlign16([
    99i8, 111i8, 112i8, 121i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8, 47i8, 41i8,
    10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 97i8, 32i8, 115i8,
    104i8, 97i8, 108i8, 108i8, 111i8, 119i8, 32i8, 99i8, 111i8, 112i8, 121i8, 32i8, 111i8, 102i8,
    32i8, 116i8, 104i8, 101i8, 32i8, 108i8, 105i8, 115i8, 116i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.listobject.list_copy")]
extern "C-unwind" fn list_copy(self_: *mut _object, _unused_ignored: *mut _object) -> *mut _object {
    let mut return_value: *mut _object = core::ptr::null_mut::<_object>();
    return_value = list_copy_impl(self_ as *mut __SlateRecord_listobject_h_22);
    return_value
}

static list_append__doc__: __SlateAlign16<[i8; 67]> = __SlateAlign16([
    97i8, 112i8, 112i8, 101i8, 110i8, 100i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8,
    111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8,
    65i8, 112i8, 112i8, 101i8, 110i8, 100i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 32i8,
    116i8, 111i8, 32i8, 116i8, 104i8, 101i8, 32i8, 101i8, 110i8, 100i8, 32i8, 111i8, 102i8, 32i8,
    116i8, 104i8, 101i8, 32i8, 108i8, 105i8, 115i8, 116i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.listobject.list_append")]
extern "C-unwind" fn list_append(self_: *mut _object, object: *mut _object) -> *mut _object {
    let mut return_value: *mut _object = core::ptr::null_mut::<_object>();
    return_value = list_append_impl(self_ as *mut __SlateRecord_listobject_h_22, object);
    return_value
}

static list_extend__doc__: __SlateAlign16<[i8; 84]> = __SlateAlign16([
    101i8, 120i8, 116i8, 101i8, 110i8, 100i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8,
    105i8, 116i8, 101i8, 114i8, 97i8, 98i8, 108i8, 101i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8,
    10i8, 10i8, 69i8, 120i8, 116i8, 101i8, 110i8, 100i8, 32i8, 108i8, 105i8, 115i8, 116i8, 32i8,
    98i8, 121i8, 32i8, 97i8, 112i8, 112i8, 101i8, 110i8, 100i8, 105i8, 110i8, 103i8, 32i8, 101i8,
    108i8, 101i8, 109i8, 101i8, 110i8, 116i8, 115i8, 32i8, 102i8, 114i8, 111i8, 109i8, 32i8, 116i8,
    104i8, 101i8, 32i8, 105i8, 116i8, 101i8, 114i8, 97i8, 98i8, 108i8, 101i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.listobject.list_extend")]
extern "C-unwind" fn list_extend(self_: *mut _object, iterable: *mut _object) -> *mut _object {
    let mut return_value: *mut _object = core::ptr::null_mut::<_object>();
    return_value = list_extend_impl(self_ as *mut __SlateRecord_listobject_h_22, iterable);
    return_value
}

static list_pop__doc__: __SlateAlign16<[i8; 138]> = __SlateAlign16([
    112i8, 111i8, 112i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8, 105i8, 110i8, 100i8,
    101i8, 120i8, 61i8, 45i8, 49i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8,
    101i8, 109i8, 111i8, 118i8, 101i8, 32i8, 97i8, 110i8, 100i8, 32i8, 114i8, 101i8, 116i8, 117i8,
    114i8, 110i8, 32i8, 105i8, 116i8, 101i8, 109i8, 32i8, 97i8, 116i8, 32i8, 105i8, 110i8, 100i8,
    101i8, 120i8, 32i8, 40i8, 100i8, 101i8, 102i8, 97i8, 117i8, 108i8, 116i8, 32i8, 108i8, 97i8,
    115i8, 116i8, 41i8, 46i8, 10i8, 10i8, 82i8, 97i8, 105i8, 115i8, 101i8, 115i8, 32i8, 73i8,
    110i8, 100i8, 101i8, 120i8, 69i8, 114i8, 114i8, 111i8, 114i8, 32i8, 105i8, 102i8, 32i8, 108i8,
    105i8, 115i8, 116i8, 32i8, 105i8, 115i8, 32i8, 101i8, 109i8, 112i8, 116i8, 121i8, 32i8, 111i8,
    114i8, 32i8, 105i8, 110i8, 100i8, 101i8, 120i8, 32i8, 105i8, 115i8, 32i8, 111i8, 117i8, 116i8,
    32i8, 111i8, 102i8, 32i8, 114i8, 97i8, 110i8, 103i8, 101i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.listobject.list_pop")]
extern "C-unwind" fn list_pop(
    self_: *mut _object,
    args: *const *mut _object,
    nargs: i64,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut iobj: *mut _object = unsafe { core::mem::zeroed() };
    let mut ival: i64 = unsafe { core::mem::zeroed() };
    let mut index: i64 = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_0: {
            return_value = core::ptr::null_mut::<_object>();
            index = 1i32.wrapping_neg() as i64;
            if _PyArg_CheckPositional!(c"pop".as_ptr() as *mut i8, nargs, 0i32, 1i32) {
                if nargs >= 1 {
                    ival = 1i32.wrapping_neg() as i64;
                    iobj = _PyNumber_Index(*args.wrapping_offset(0 as isize));
                    if !iobj.is_null() {
                        ival = PyLong_AsSsize_t(iobj);
                        Py_DECREF!(iobj);
                    }
                    if ival == (1i32.wrapping_neg() as i64) {
                        __v0 = !PyErr_Occurred().is_null();
                    } else {
                        __v0 = false;
                    }
                    if __v0 {
                        break '__join_0;
                    } else {
                        index = ival;
                    }
                }
                return_value = list_pop_impl(self_ as *mut __SlateRecord_listobject_h_22, index);
            }
        }
        return return_value;
    }
}

static list_sort__doc__: __SlateAlign16<[i8; 414]> = __SlateAlign16([
    115i8, 111i8, 114i8, 116i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8, 47i8, 44i8,
    32i8, 42i8, 44i8, 32i8, 107i8, 101i8, 121i8, 61i8, 78i8, 111i8, 110i8, 101i8, 44i8, 32i8,
    114i8, 101i8, 118i8, 101i8, 114i8, 115i8, 101i8, 61i8, 70i8, 97i8, 108i8, 115i8, 101i8, 41i8,
    10i8, 45i8, 45i8, 10i8, 10i8, 83i8, 111i8, 114i8, 116i8, 32i8, 116i8, 104i8, 101i8, 32i8,
    108i8, 105i8, 115i8, 116i8, 32i8, 105i8, 110i8, 32i8, 97i8, 115i8, 99i8, 101i8, 110i8, 100i8,
    105i8, 110i8, 103i8, 32i8, 111i8, 114i8, 100i8, 101i8, 114i8, 32i8, 97i8, 110i8, 100i8, 32i8,
    114i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 78i8, 111i8, 110i8, 101i8, 46i8, 10i8, 10i8,
    84i8, 104i8, 101i8, 32i8, 115i8, 111i8, 114i8, 116i8, 32i8, 105i8, 115i8, 32i8, 105i8, 110i8,
    45i8, 112i8, 108i8, 97i8, 99i8, 101i8, 32i8, 40i8, 105i8, 46i8, 101i8, 46i8, 32i8, 116i8,
    104i8, 101i8, 32i8, 108i8, 105i8, 115i8, 116i8, 32i8, 105i8, 116i8, 115i8, 101i8, 108i8, 102i8,
    32i8, 105i8, 115i8, 32i8, 109i8, 111i8, 100i8, 105i8, 102i8, 105i8, 101i8, 100i8, 41i8, 32i8,
    97i8, 110i8, 100i8, 32i8, 115i8, 116i8, 97i8, 98i8, 108i8, 101i8, 10i8, 40i8, 105i8, 46i8,
    101i8, 46i8, 32i8, 116i8, 104i8, 101i8, 32i8, 111i8, 114i8, 100i8, 101i8, 114i8, 32i8, 111i8,
    102i8, 32i8, 116i8, 119i8, 111i8, 32i8, 101i8, 113i8, 117i8, 97i8, 108i8, 32i8, 101i8, 108i8,
    101i8, 109i8, 101i8, 110i8, 116i8, 115i8, 32i8, 105i8, 115i8, 32i8, 109i8, 97i8, 105i8, 110i8,
    116i8, 97i8, 105i8, 110i8, 101i8, 100i8, 41i8, 46i8, 10i8, 10i8, 73i8, 102i8, 32i8, 97i8, 32i8,
    107i8, 101i8, 121i8, 32i8, 102i8, 117i8, 110i8, 99i8, 116i8, 105i8, 111i8, 110i8, 32i8, 105i8,
    115i8, 32i8, 103i8, 105i8, 118i8, 101i8, 110i8, 44i8, 32i8, 97i8, 112i8, 112i8, 108i8, 121i8,
    32i8, 105i8, 116i8, 32i8, 111i8, 110i8, 99i8, 101i8, 32i8, 116i8, 111i8, 32i8, 101i8, 97i8,
    99i8, 104i8, 32i8, 108i8, 105i8, 115i8, 116i8, 32i8, 105i8, 116i8, 101i8, 109i8, 32i8, 97i8,
    110i8, 100i8, 32i8, 115i8, 111i8, 114i8, 116i8, 10i8, 116i8, 104i8, 101i8, 109i8, 44i8, 32i8,
    97i8, 115i8, 99i8, 101i8, 110i8, 100i8, 105i8, 110i8, 103i8, 32i8, 111i8, 114i8, 32i8, 100i8,
    101i8, 115i8, 99i8, 101i8, 110i8, 100i8, 105i8, 110i8, 103i8, 44i8, 32i8, 97i8, 99i8, 99i8,
    111i8, 114i8, 100i8, 105i8, 110i8, 103i8, 32i8, 116i8, 111i8, 32i8, 116i8, 104i8, 101i8, 105i8,
    114i8, 32i8, 102i8, 117i8, 110i8, 99i8, 116i8, 105i8, 111i8, 110i8, 32i8, 118i8, 97i8, 108i8,
    117i8, 101i8, 115i8, 46i8, 10i8, 10i8, 84i8, 104i8, 101i8, 32i8, 114i8, 101i8, 118i8, 101i8,
    114i8, 115i8, 101i8, 32i8, 102i8, 108i8, 97i8, 103i8, 32i8, 99i8, 97i8, 110i8, 32i8, 98i8,
    101i8, 32i8, 115i8, 101i8, 116i8, 32i8, 116i8, 111i8, 32i8, 115i8, 111i8, 114i8, 116i8, 32i8,
    105i8, 110i8, 32i8, 100i8, 101i8, 115i8, 99i8, 101i8, 110i8, 100i8, 105i8, 110i8, 103i8, 32i8,
    111i8, 114i8, 100i8, 101i8, 114i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.listobject.list_sort")]
extern "C-unwind" fn list_sort(
    self_: *mut _object,
    mut args: *const *mut _object,
    nargs: i64,
    kwnames: *mut _object,
) -> *mut _object {
    let mut reverse: i32 = unsafe { core::mem::zeroed() };
    let mut keyfunc: *mut _object = unsafe { core::mem::zeroed() };
    let mut __v4: i64 = unsafe { core::mem::zeroed() };
    let mut noptargs: i64 = unsafe { core::mem::zeroed() };
    let mut argsbuf: core::mem::MaybeUninit<__SlateAlign16<[*mut _object; 2]>> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_6: *mut [*mut _object; 2] =
        core::ptr::addr_of_mut!(argsbuf) as *mut [*mut _object; 2];
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        if !kwnames.is_null() {
            __v4 = PyTuple_GET_SIZE!(kwnames);
        } else {
            __v4 = 0;
        }
        '__join_0: {
            noptargs = nargs.wrapping_add(__v4).wrapping_sub(0);
            keyfunc = core::ptr::addr_of_mut!(_Py_NoneStruct);
            reverse = 0;
            args = _PyArg_UnpackKeywords!(
                args,
                nargs,
                core::ptr::null_mut::<()>(),
                kwnames,
                core::ptr::addr_of_mut!(_parser),
                0i32,
                0i32,
                0i32,
                0i32,
                (*__slate_slot_6).as_mut_ptr() as *mut *mut _object
            );
            // minpos
            // maxpos
            // minkw
            // varpos
            if !args.is_null() {
                '__join_1: {
                    if noptargs != 0 {
                        if !unsafe { *args.wrapping_offset(0 as isize) }.is_null() {
                            keyfunc = *args.wrapping_offset(0 as isize);
                            let __v0: i64 = noptargs.wrapping_sub(1);
                            noptargs = __v0;
                            if __v0 == 0 {
                                break '__join_1;
                            }
                        }
                        reverse = PyObject_IsTrue(*args.wrapping_offset(1 as isize));
                        if reverse < 0 {
                            break '__join_0;
                        }
                    }
                }
                return_value = list_sort_impl(
                    self_ as *mut __SlateRecord_listobject_h_22,
                    keyfunc,
                    reverse,
                );
            }
        }
        return return_value;
    }
}

static mut _kwtuple: __SlateRecord_listobject_c_h_229 = {
    let mut __t0: __SlateRecord_listobject_c_h_229 = unsafe { core::mem::zeroed() };
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
                    ._py_key
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
                    ._py_reverse
                    ._ascii
                    .ob_base
            )
        },
    ];
    __t0
};

static _keywords: __SlateAlign16<[*const i8; 3]> = __SlateAlign16([
    (b"key\0".as_ptr() as *mut i8) as *const i8,
    (b"reverse\0".as_ptr() as *mut i8) as *const i8,
    core::ptr::null::<i8>(),
]);

static mut _parser: _PyArg_Parser = {
    let mut __t0: _PyArg_Parser = unsafe { core::mem::zeroed() };
    __t0.keywords = unsafe { core::ptr::addr_of!(_keywords.0) as *const *const i8 };
    __t0.fname = (b"sort\0".as_ptr() as *mut i8) as *const i8;
    __t0.kwtuple = unsafe { core::ptr::addr_of_mut!(_kwtuple.ob_base.ob_base) };
    __t0
};

static list_reverse__doc__: __SlateAlign16<[i8; 42]> = __SlateAlign16([
    114i8, 101i8, 118i8, 101i8, 114i8, 115i8, 101i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8,
    32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 118i8, 101i8, 114i8, 115i8, 101i8,
    32i8, 42i8, 73i8, 78i8, 32i8, 80i8, 76i8, 65i8, 67i8, 69i8, 42i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.listobject.list_reverse")]
extern "C-unwind" fn list_reverse(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    let mut return_value: *mut _object = core::ptr::null_mut::<_object>();
    return_value = list_reverse_impl(self_ as *mut __SlateRecord_listobject_h_22);
    return_value
}

static list_index__doc__: __SlateAlign16<[i8; 131]> = __SlateAlign16([
    105i8, 110i8, 100i8, 101i8, 120i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8, 118i8,
    97i8, 108i8, 117i8, 101i8, 44i8, 32i8, 115i8, 116i8, 97i8, 114i8, 116i8, 61i8, 48i8, 44i8,
    32i8, 115i8, 116i8, 111i8, 112i8, 61i8, 115i8, 121i8, 115i8, 46i8, 109i8, 97i8, 120i8, 115i8,
    105i8, 122i8, 101i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8,
    117i8, 114i8, 110i8, 32i8, 102i8, 105i8, 114i8, 115i8, 116i8, 32i8, 105i8, 110i8, 100i8, 101i8,
    120i8, 32i8, 111i8, 102i8, 32i8, 118i8, 97i8, 108i8, 117i8, 101i8, 46i8, 10i8, 10i8, 82i8,
    97i8, 105i8, 115i8, 101i8, 115i8, 32i8, 86i8, 97i8, 108i8, 117i8, 101i8, 69i8, 114i8, 114i8,
    111i8, 114i8, 32i8, 105i8, 102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 118i8, 97i8, 108i8, 117i8,
    101i8, 32i8, 105i8, 115i8, 32i8, 110i8, 111i8, 116i8, 32i8, 112i8, 114i8, 101i8, 115i8, 101i8,
    110i8, 116i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.listobject.list_index")]
extern "C-unwind" fn list_index(
    self_: *mut _object,
    args: *const *mut _object,
    nargs: i64,
) -> *mut _object {
    let mut stop: i64 = unsafe { core::mem::zeroed() };
    let mut start: i64 = unsafe { core::mem::zeroed() };
    let mut value: *mut _object = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_0: {
            return_value = core::ptr::null_mut::<_object>();
            start = 0;
            stop = 9223372036854775807;
            if _PyArg_CheckPositional!(c"index".as_ptr() as *mut i8, nargs, 1i32, 3i32) {
                value = *args.wrapping_offset(0 as isize);
                if nargs >= 2 {
                    if _PyEval_SliceIndexNotNone(
                        *args.wrapping_offset(1 as isize),
                        core::ptr::addr_of_mut!(start),
                    ) == 0
                    {
                        break '__join_0;
                    } else {
                        if nargs >= 3 {
                            if _PyEval_SliceIndexNotNone(
                                *args.wrapping_offset(2 as isize),
                                core::ptr::addr_of_mut!(stop),
                            ) == 0
                            {
                                break '__join_0;
                            }
                        }
                    }
                }
                return_value = list_index_impl(
                    self_ as *mut __SlateRecord_listobject_h_22,
                    value,
                    start,
                    stop,
                );
            }
        }
        return return_value;
    }
}

static list_count__doc__: __SlateAlign16<[i8; 66]> = __SlateAlign16([
    99i8, 111i8, 117i8, 110i8, 116i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8, 118i8,
    97i8, 108i8, 117i8, 101i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8,
    116i8, 117i8, 114i8, 110i8, 32i8, 110i8, 117i8, 109i8, 98i8, 101i8, 114i8, 32i8, 111i8, 102i8,
    32i8, 111i8, 99i8, 99i8, 117i8, 114i8, 114i8, 101i8, 110i8, 99i8, 101i8, 115i8, 32i8, 111i8,
    102i8, 32i8, 118i8, 97i8, 108i8, 117i8, 101i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.listobject.list_count")]
extern "C-unwind" fn list_count(self_: *mut _object, value: *mut _object) -> *mut _object {
    let mut return_value: *mut _object = core::ptr::null_mut::<_object>();
    return_value = list_count_impl(self_ as *mut __SlateRecord_listobject_h_22, value);
    return_value
}

static list_remove__doc__: __SlateAlign16<[i8; 110]> = __SlateAlign16([
    114i8, 101i8, 109i8, 111i8, 118i8, 101i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8,
    118i8, 97i8, 108i8, 117i8, 101i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8,
    101i8, 109i8, 111i8, 118i8, 101i8, 32i8, 102i8, 105i8, 114i8, 115i8, 116i8, 32i8, 111i8, 99i8,
    99i8, 117i8, 114i8, 114i8, 101i8, 110i8, 99i8, 101i8, 32i8, 111i8, 102i8, 32i8, 118i8, 97i8,
    108i8, 117i8, 101i8, 46i8, 10i8, 10i8, 82i8, 97i8, 105i8, 115i8, 101i8, 115i8, 32i8, 86i8,
    97i8, 108i8, 117i8, 101i8, 69i8, 114i8, 114i8, 111i8, 114i8, 32i8, 105i8, 102i8, 32i8, 116i8,
    104i8, 101i8, 32i8, 118i8, 97i8, 108i8, 117i8, 101i8, 32i8, 105i8, 115i8, 32i8, 110i8, 111i8,
    116i8, 32i8, 112i8, 114i8, 101i8, 115i8, 101i8, 110i8, 116i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.listobject.list_remove")]
extern "C-unwind" fn list_remove(self_: *mut _object, value: *mut _object) -> *mut _object {
    let mut return_value: *mut _object = core::ptr::null_mut::<_object>();
    return_value = list_remove_impl(self_ as *mut __SlateRecord_listobject_h_22, value);
    return_value
}

static list___init____doc__: __SlateAlign16<[i8; 167]> = __SlateAlign16([
    108i8, 105i8, 115i8, 116i8, 40i8, 105i8, 116i8, 101i8, 114i8, 97i8, 98i8, 108i8, 101i8, 61i8,
    40i8, 41i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 66i8, 117i8, 105i8, 108i8,
    116i8, 45i8, 105i8, 110i8, 32i8, 109i8, 117i8, 116i8, 97i8, 98i8, 108i8, 101i8, 32i8, 115i8,
    101i8, 113i8, 117i8, 101i8, 110i8, 99i8, 101i8, 46i8, 10i8, 10i8, 73i8, 102i8, 32i8, 110i8,
    111i8, 32i8, 97i8, 114i8, 103i8, 117i8, 109i8, 101i8, 110i8, 116i8, 32i8, 105i8, 115i8, 32i8,
    103i8, 105i8, 118i8, 101i8, 110i8, 44i8, 32i8, 116i8, 104i8, 101i8, 32i8, 99i8, 111i8, 110i8,
    115i8, 116i8, 114i8, 117i8, 99i8, 116i8, 111i8, 114i8, 32i8, 99i8, 114i8, 101i8, 97i8, 116i8,
    101i8, 115i8, 32i8, 97i8, 32i8, 110i8, 101i8, 119i8, 32i8, 101i8, 109i8, 112i8, 116i8, 121i8,
    32i8, 108i8, 105i8, 115i8, 116i8, 46i8, 10i8, 84i8, 104i8, 101i8, 32i8, 97i8, 114i8, 103i8,
    117i8, 109i8, 101i8, 110i8, 116i8, 32i8, 109i8, 117i8, 115i8, 116i8, 32i8, 98i8, 101i8, 32i8,
    97i8, 110i8, 32i8, 105i8, 116i8, 101i8, 114i8, 97i8, 98i8, 108i8, 101i8, 32i8, 105i8, 102i8,
    32i8, 115i8, 112i8, 101i8, 99i8, 105i8, 102i8, 105i8, 101i8, 100i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.listobject.list___init__")]
extern "C-unwind" fn list___init__(
    self_: *mut _object,
    args: *mut _object,
    kwargs: *mut _object,
) -> i32 {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut __v1: bool = unsafe { core::mem::zeroed() };
    let mut iterable: *mut _object = unsafe { core::mem::zeroed() };
    let mut base_tp: *mut _typeobject = unsafe { core::mem::zeroed() };
    let mut return_value: i32 = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = 1i32.wrapping_neg();
        base_tp = core::ptr::addr_of_mut!(PyList_Type);
        iterable = core::ptr::null_mut::<_object>();
        if Py_IS_TYPE!(self_, base_tp) != 0 {
            __v1 = true;
        } else {
            __v1 = (*Py_TYPE!(self_)).tp_new == (*base_tp).tp_new;
        }
        if __v1 {
            __v0 = !_PyArg_NoKeywords!(c"list".as_ptr() as *mut i8, kwargs);
        } else {
            __v0 = false;
        }
        if !__v0 {
            if _PyArg_CheckPositional__2!(
                c"list".as_ptr() as *mut i8,
                PyTuple_GET_SIZE(args),
                0i32,
                1i32
            ) {
                if PyTuple_GET_SIZE!(args) >= 1 {
                    iterable = PyTuple_GET_ITEM!(args, 0i32);
                }
                return_value =
                    list___init___impl(self_ as *mut __SlateRecord_listobject_h_22, iterable);
            }
        }
        return return_value;
    }
}

static list___sizeof____doc__: __SlateAlign16<[i8; 74]> = __SlateAlign16([
    95i8, 95i8, 115i8, 105i8, 122i8, 101i8, 111i8, 102i8, 95i8, 95i8, 40i8, 36i8, 115i8, 101i8,
    108i8, 102i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8,
    114i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 115i8, 105i8, 122i8, 101i8, 32i8, 111i8, 102i8,
    32i8, 116i8, 104i8, 101i8, 32i8, 108i8, 105i8, 115i8, 116i8, 32i8, 105i8, 110i8, 32i8, 109i8,
    101i8, 109i8, 111i8, 114i8, 121i8, 44i8, 32i8, 105i8, 110i8, 32i8, 98i8, 121i8, 116i8, 101i8,
    115i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.listobject.list___sizeof__")]
extern "C-unwind" fn list___sizeof__(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    list___sizeof___impl(self_ as *mut __SlateRecord_listobject_h_22)
}

static list___reversed____doc__: __SlateAlign16<[i8; 68]> = __SlateAlign16([
    95i8, 95i8, 114i8, 101i8, 118i8, 101i8, 114i8, 115i8, 101i8, 100i8, 95i8, 95i8, 40i8, 36i8,
    115i8, 101i8, 108i8, 102i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8,
    116i8, 117i8, 114i8, 110i8, 32i8, 97i8, 32i8, 114i8, 101i8, 118i8, 101i8, 114i8, 115i8, 101i8,
    32i8, 105i8, 116i8, 101i8, 114i8, 97i8, 116i8, 111i8, 114i8, 32i8, 111i8, 118i8, 101i8, 114i8,
    32i8, 116i8, 104i8, 101i8, 32i8, 108i8, 105i8, 115i8, 116i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.listobject.list___reversed__")]
extern "C-unwind" fn list___reversed__(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    list___reversed___impl(self_ as *mut __SlateRecord_listobject_h_22)
}

// _PyIndex_Check()
// _PyEval_GetBuiltin()
// _Py_CRITICAL_SECTION_ASSERT_OBJECT_LOCKED()
// _PyDictViewObject
// _Py_FREELIST_FREE(), _Py_FREELIST_POP()
// PyInterpreterState.list
// struct _Py_list_freelist, _PyListIterObject
// _PyLong_DigitCount
// _PyArg_NoKwnames()
// _PyObject_GC_TRACK(), _PyDebugAllocatorStats()
// _Py_TryIncrefCompareStackRef()
// _PyTuple_FromArray()
// _Py_TYPE_VERSION_LIST
// _PySet_NextEntry()
// [clinic input]
// class list "PyListObject *" "&PyList_Type"
// [clinic start generated code]
//
// [clinic end generated code: output=da39a3ee5e6b4b0d input=f9b222678f9f71e0]
fn free_list_items(items: *mut *mut _object, use_qsbr: bool) {
    unsafe { PyMem_Free(items as *mut ()) };
}

fn ensure_shared_on_resize(self_: *mut __SlateRecord_listobject_h_22) {}

/// Ensure ob_item has room for at least newsize elements, and set
/// ob_size to newsize.  If newsize > ob_size on entry, the content
/// of the new slots at exit is undefined heap trash; it's the caller's
/// responsibility to overwrite them with sane values.
/// The number of allocated elements may grow, shrink, or stay the same.
/// Failure is impossible if newsize <= self.allocated on entry, although
/// that partly relies on an assumption that the system realloc() never
/// fails when passed a number of bytes <= the number of bytes last
/// allocated (the C standard doesn't guarantee this, but it's hard to
/// imagine a realloc implementation where it wouldn't be true).
/// Note that self->ob_item may change, and even if newsize is less
/// than ob_size on entry.
fn list_resize(self_: *mut __SlateRecord_listobject_h_22, newsize: i64) -> i32 {
    let mut new_allocated: u64 = 0;
    let mut target_bytes: u64 = 0;
    let allocated: i64 = unsafe { (*self_).allocated };
    // Bypass realloc() when a previous overallocation is large enough
    // to accommodate the newsize.  If the newsize falls lower than half
    // the allocated size, then proceed with the realloc() to shrink the list.
    if allocated >= newsize && newsize >= allocated >> 1i32 {
        Py_SET_SIZE!(self_, newsize);
        return 0;
    }
    // This over-allocates proportional to the list size, making room
    // for additional growth.  The over-allocation is mild, but is
    // enough to give linear-time amortized behavior over a long
    // sequence of appends() in the presence of a poorly-performing
    // system realloc().
    // Add padding to make the allocated size multiple of 4.
    // The growth pattern is:  0, 4, 8, 16, 24, 32, 40, 52, 64, 76, ...
    // Note: new_allocated won't overflow because the largest possible value
    //       is PY_SSIZE_T_MAX * (9 / 8) + 6 which always fits in a size_t.
    new_allocated = (newsize as u64)
        .wrapping_add((newsize >> 3i32) as u64)
        .wrapping_add(6)
        & !3;
    // Do not overallocate if the new size is closer to overallocated size
    // than to the old size.
    if newsize.wrapping_sub(Py_SIZE!(self_)) > (new_allocated.wrapping_sub(newsize as u64) as i64) {
        new_allocated = (newsize as u64).wrapping_add(3) & !3;
    }
    if newsize == 0 {
        new_allocated = 0;
    }
    ensure_shared_on_resize(self_);
    let mut items: *mut *mut _object = unsafe { core::mem::zeroed() };
    if new_allocated <= 9223372036854775807 / (core::mem::size_of::<*mut _object>() as u64) {
        target_bytes = new_allocated.wrapping_mul(core::mem::size_of::<*mut _object>() as u64);
        items = (unsafe { PyMem_Realloc((*self_).ob_item as *mut (), target_bytes) })
            as *mut *mut _object;
    } else {
        // integer overflow
        items = core::ptr::null_mut::<*mut _object>();
    }
    if items.is_null() {
        unsafe { PyErr_NoMemory() };
        return 1i32.wrapping_neg();
    }
    unsafe {
        (*self_).ob_item = items;
    }
    Py_SET_SIZE!(self_, newsize);
    unsafe {
        (*self_).allocated = new_allocated as i64;
    }
    0
}

fn list_preallocate_exact(self_: *mut __SlateRecord_listobject_h_22, mut size: i64) -> i32 {
    // Since the Python memory allocator has granularity of 16 bytes on 64-bit
    // platforms (8 on 32-bit), there is no benefit of allocating space for
    // the odd number of items, and there is no drawback of rounding the
    // allocated size up to the nearest even number.
    size = ((size.wrapping_add(1) as u64) & !1) as i64;
    let __v0: *mut *mut _object = if (size as u64)
        > 9223372036854775807 / (core::mem::size_of::<*mut _object>() as u64)
    {
        core::ptr::null_mut::<*mut _object>()
    } else {
        (unsafe {
            PyMem_Malloc((size as u64).wrapping_mul(core::mem::size_of::<*mut _object>() as u64))
        }) as *mut *mut _object
    };
    let items: *mut *mut _object = __v0;
    if items.is_null() {
        unsafe { PyErr_NoMemory() };
        return 1i32.wrapping_neg();
    }
    FT_ATOMIC_STORE_PTR_RELEASE!((*self_).ob_item, items);
    unsafe {
        (*self_).allocated = size;
    }
    0
}

/// Print summary info about the state of the optimized allocator
#[unsafe(no_mangle)]
extern "C-unwind" fn _PyList_DebugMallocStats(out: *mut slate_support::FILE) {
    unsafe {
        _PyDebugAllocatorStats(
            out,
            c"free PyListObject".as_ptr(),
            _Py_FREELIST_SIZE!(lists),
            core::mem::size_of::<__SlateRecord_listobject_h_22>() as u64,
        )
    };
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyList_New(size: i64) -> *mut _object {
    if size < 0 {
        PyErr_BadInternalCall__31!();
        return core::ptr::null_mut::<_object>();
    }
    let mut op: *mut __SlateRecord_listobject_h_22 =
        _Py_FREELIST_POP!(__SlateRecord_listobject_h_22, lists);
    if op.is_null() {
        op = PyObject_GC_New!(__SlateRecord_listobject_h_22, unsafe {
            core::ptr::addr_of_mut!(PyList_Type)
        });
        if op.is_null() {
            return core::ptr::null_mut::<_object>();
        }
    }
    if size <= 0 {
        unsafe {
            (*op).ob_item = core::ptr::null_mut::<*mut _object>();
        }
    } else {
        unsafe {
            (*op).ob_item = PyMem_Calloc(size as u64, core::mem::size_of::<*mut _object>() as u64)
                as *mut *mut _object;
        }
        if unsafe { (*op).ob_item }.is_null() {
            Py_DECREF__2!(op);
            return unsafe { PyErr_NoMemory() };
        }
    }
    Py_SET_SIZE!(op, size);
    unsafe {
        (*op).allocated = size;
    }
    _PyObject_GC_TRACK!(op);
    op as *mut _object
}

fn list_new_prealloc(size: i64) -> *mut _object {
    let op: *mut __SlateRecord_listobject_h_22 =
        PyList_New(0i64) as *mut __SlateRecord_listobject_h_22;
    if op.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let __v0: *mut *mut _object = if (size as u64)
        > 9223372036854775807 / (core::mem::size_of::<*mut _object>() as u64)
    {
        core::ptr::null_mut::<*mut _object>()
    } else {
        (unsafe {
            PyMem_Malloc((size as u64).wrapping_mul(core::mem::size_of::<*mut _object>() as u64))
        }) as *mut *mut _object
    };
    unsafe {
        (*op).ob_item = __v0;
    }
    if unsafe { (*op).ob_item }.is_null() {
        Py_DECREF__2!(op);
        return unsafe { PyErr_NoMemory() };
    }
    unsafe {
        (*op).allocated = size;
    }
    op as *mut _object
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyList_Size(op: *mut _object) -> i64 {
    if PyList_Check!(op) == 0 {
        PyErr_BadInternalCall__143!();
        return 1i32.wrapping_neg() as i64;
    } else {
        return PyList_GET_SIZE!(op);
    }
}

fn valid_index(i: i64, limit: i64) -> i32 {
    // The cast to size_t lets us use just a single comparison
    // to check whether i is in the range: 0 <= i < limit.
    //
    // See:  Section 14.2 "Bounds Checking" in the Agner Fog
    // optimization manual found at:
    // https://www.agner.org/optimize/optimizing_cpp.pdf
    ((i as u64) < (limit as u64)) as i32
}

fn list_get_item_ref(op: *mut __SlateRecord_listobject_h_22, i: i64) -> *mut _object {
    if valid_index(i, Py_SIZE!(op)) == 0 {
        return core::ptr::null_mut::<_object>();
    }
    Py_NewRef!(PyList_GET_ITEM__2!(op, i))
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyList_GetItem(op: *mut _object, i: i64) -> *mut _object {
    if PyList_Check!(op) == 0 {
        PyErr_BadInternalCall__98!();
        return core::ptr::null_mut::<_object>();
    }
    if valid_index(i, Py_SIZE__2!(op)) == 0 {
        unsafe {
            PyErr_SetObject(
                PyExc_IndexError,
                core::ptr::addr_of_mut!(
                    _PyRuntime
                        .static_objects
                        .singletons
                        .strings
                        .literals
                        ._py_list_err
                        ._ascii
                        .ob_base
                ),
            )
        };
        return core::ptr::null_mut::<_object>();
    }
    unsafe {
        *unsafe { (*(op as *mut __SlateRecord_listobject_h_22)).ob_item }
            .wrapping_offset(i as isize)
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyList_GetItemRef(op: *mut _object, i: i64) -> *mut _object {
    if PyList_Check!(op) == 0 {
        unsafe { PyErr_SetString(PyExc_TypeError, c"expected a list".as_ptr()) };
        return core::ptr::null_mut::<_object>();
    }
    let item: *mut _object = list_get_item_ref(op as *mut __SlateRecord_listobject_h_22, i);
    if item.is_null() {
        unsafe {
            PyErr_SetObject(
                PyExc_IndexError,
                core::ptr::addr_of_mut!(
                    _PyRuntime
                        .static_objects
                        .singletons
                        .strings
                        .literals
                        ._py_list_err
                        ._ascii
                        .ob_base
                ),
            )
        };
        return core::ptr::null_mut::<_object>();
    }
    item
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyList_GetItemRef(
    list: *mut __SlateRecord_listobject_h_22,
    i: i64,
) -> *mut _object {
    list_get_item_ref(list, i)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyList_SetItem(op: *mut _object, i: i64, newitem: *mut _object) -> i32 {
    let mut self_: *mut __SlateRecord_listobject_h_22 = unsafe { core::mem::zeroed() };
    let mut ret: i32 = unsafe { core::mem::zeroed() };
    unsafe {
        if PyList_Check!(op) == 0 {
            Py_XDECREF!(newitem);
            PyErr_BadInternalCall__52!();
            return 1i32.wrapping_neg();
        } else {
            self_ = op as *mut __SlateRecord_listobject_h_22;
            if valid_index(i, Py_SIZE!(self_)) == 0 {
                Py_XDECREF!(newitem);
                PyErr_SetString(
                    PyExc_IndexError,
                    c"list assignment index out of range".as_ptr(),
                );
                ret = 1i32.wrapping_neg();
            } else {
                let tmp: *mut _object = *unsafe { (*self_).ob_item }.wrapping_offset(i as isize);
                FT_ATOMIC_STORE_PTR_RELEASE!(
                    *unsafe { (*self_).ob_item }.wrapping_offset(i as isize),
                    newitem
                );
                Py_XDECREF!(tmp);
                ret = 0;
            }
            return ret;
        }
    }
}

fn ins1(self_: *mut __SlateRecord_listobject_h_22, mut r#where: i64, v: *mut _object) -> i32 {
    let mut i: i64 = 0;
    let n: i64 = Py_SIZE!(self_);
    let mut items: *mut *mut _object = unsafe { core::mem::zeroed() };
    if v.is_null() {
        PyErr_BadInternalCall__40!();
        return 1i32.wrapping_neg();
    }
    if list_resize(self_, n.wrapping_add(1)) < 0 {
        return 1i32.wrapping_neg();
    }
    if r#where < 0 {
        r#where = r#where.wrapping_add(n);
        if r#where < 0 {
            r#where = 0;
        }
    }
    if r#where > n {
        r#where = n;
    }
    items = unsafe { (*self_).ob_item };
    i = n;
    loop {
        let __v3: i64 = i.wrapping_sub(1);
        i = __v3;
        if __v3 < r#where {
            break;
        }
        FT_ATOMIC_STORE_PTR_RELAXED!(*items.wrapping_offset(i.wrapping_add(1) as isize), unsafe {
            *items.wrapping_offset(i as isize)
        });
    }
    FT_ATOMIC_STORE_PTR_RELEASE!(*items.wrapping_offset(r#where as isize), Py_NewRef!(v));
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyList_Insert(op: *mut _object, r#where: i64, newitem: *mut _object) -> i32 {
    if PyList_Check!(op) == 0 {
        PyErr_BadInternalCall__114!();
        return 1i32.wrapping_neg();
    }
    let self_: *mut __SlateRecord_listobject_h_22 = op as *mut __SlateRecord_listobject_h_22;
    let mut err: i32 = 0;
    err = ins1(self_, r#where, newitem);
    err
}

/// internal, used by _PyList_AppendTakeRef
#[unsafe(no_mangle)]
extern "C-unwind" fn _PyList_AppendTakeRefListResize(
    self_: *mut __SlateRecord_listobject_h_22,
    newitem: *mut _object,
) -> i32 {
    let len: i64 = Py_SIZE!(self_);
    if list_resize(self_, len.wrapping_add(1)) < 0 {
        Py_DECREF!(newitem);
        return 1i32.wrapping_neg();
    }
    FT_ATOMIC_STORE_PTR_RELEASE!(
        *unsafe { (*self_).ob_item }.wrapping_offset(len as isize),
        newitem
    );
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyList_Append(op: *mut _object, newitem: *mut _object) -> i32 {
    if PyList_Check!(op) != 0 && !newitem.is_null() {
        let mut ret: i32 = 0;
        ret = _PyList_AppendTakeRef(
            op as *mut __SlateRecord_listobject_h_22,
            Py_NewRef!(newitem),
        );
        return ret;
    }
    PyErr_BadInternalCall__115!();
    1i32.wrapping_neg()
}

// Methods
#[unsafe(link_section = ".text.slate_distinct.listobject.list_dealloc")]
extern "C-unwind" fn list_dealloc(self_: *mut _object) {
    let op: *mut __SlateRecord_listobject_h_22 = self_ as *mut __SlateRecord_listobject_h_22;
    let mut i: i64 = 0;
    unsafe { PyObject_GC_UnTrack(op as *mut ()) };
    if !unsafe { (*op).ob_item }.is_null() {
        // Do it backwards, for Christian Tismer.
        // There's a simple test case where somehow this reduces
        // thrashing when a *very* large list is created and
        // immediately deleted.
        i = Py_SIZE!(op);
        loop {
            let __v1: i64 = i.wrapping_sub(1);
            i = __v1;
            if __v1 < 0 {
                break;
            }
            Py_XDECREF!(unsafe { *unsafe { (*op).ob_item }.wrapping_offset(i as isize) });
        }
        free_list_items(unsafe { (*op).ob_item }, false);
        unsafe {
            (*op).ob_item = core::ptr::null_mut::<*mut _object>();
        }
    }
    if PyList_CheckExact__3!(op) != 0 {
        _PyFreeList_Free(
            unsafe { core::ptr::addr_of_mut!((*_Py_freelists_GET()).lists) },
            _PyObject_CAST__2!(op) as *mut (),
            80i64,
            unsafe {
                core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
                    PyObject_GC_Del as *const (),
                )
            },
        );
    } else {
        unsafe { PyObject_GC_Del(op as *mut ()) };
    }
}

fn list_repr_impl(v: *mut __SlateRecord_listobject_h_22) -> *mut _object {
    // Do repr() on each element.  Note that this may mutate the list,
    // so must refetch the list size on each iteration.
    let mut i: i64 = unsafe { core::mem::zeroed() };
    let mut item: *mut _object = unsafe { core::mem::zeroed() };
    let mut writer: *mut PyUnicodeWriter = unsafe { core::mem::zeroed() };
    let mut __v6: *mut _object = unsafe { core::mem::zeroed() };
    let mut res: i32 = unsafe { core::mem::zeroed() };
    unsafe {
        res = Py_ReprEnter(v as *mut _object);
        if res != 0 {
            if res > 0 {
                __v6 = PyUnicode_FromString(c"[...]".as_ptr());
            } else {
                __v6 = core::ptr::null_mut::<_object>();
            }
            return __v6;
        } else {
            '__join_0: {
                // "[" + "1" + ", 2" * (len - 1) + "]"
                let prealloc: i64 = 2i64
                    .wrapping_add(3i64.wrapping_mul(Py_SIZE!(v).wrapping_sub(1)))
                    .wrapping_add(1);
                writer = PyUnicodeWriter_Create(prealloc);
                item = core::ptr::null_mut::<_object>();
                if !writer.is_null() {
                    if PyUnicodeWriter_WriteChar(writer, 91u32) >= 0 {
                        i = 0;
                        while i < Py_SIZE!(v) {
                            // Hold a strong reference since repr(item) can mutate the list
                            item = Py_XNewRef!(unsafe {
                                *unsafe { (*v).ob_item }.wrapping_offset(i as isize)
                            });
                            if i > 0 {
                                if PyUnicodeWriter_WriteChar(writer, 44u32) < 0 {
                                    break '__join_0;
                                } else {
                                    if PyUnicodeWriter_WriteChar(writer, 32u32) < 0 {
                                        break '__join_0;
                                    }
                                }
                            }
                            if PyUnicodeWriter_WriteRepr(writer, item) < 0 {
                                break '__join_0;
                            } else {
                                Py_CLEAR__2!(item);
                                i = i.wrapping_add(1);
                            }
                        }
                        if PyUnicodeWriter_WriteChar(writer, 93u32) >= 0 {
                            Py_ReprLeave(v as *mut _object);
                            return PyUnicodeWriter_Finish(writer);
                        }
                    }
                }
            }
            Py_XDECREF!(item);
            PyUnicodeWriter_Discard(writer);
            Py_ReprLeave(v as *mut _object);
            return core::ptr::null_mut::<_object>();
        }
    }
}

#[unsafe(link_section = ".text.slate_distinct.listobject.list_repr")]
extern "C-unwind" fn list_repr(self_: *mut _object) -> *mut _object {
    if PyList_GET_SIZE!(self_) == 0 {
        return unsafe { PyUnicode_FromString(c"[]".as_ptr()) };
    }
    let v: *mut __SlateRecord_listobject_h_22 = self_ as *mut __SlateRecord_listobject_h_22;
    let mut ret: *mut _object = core::ptr::null_mut::<_object>();
    ret = list_repr_impl(v);
    ret
}

#[unsafe(link_section = ".text.slate_distinct.listobject.list_length")]
extern "C-unwind" fn list_length(a: *mut _object) -> i64 {
    PyList_GET_SIZE!(a)
}

#[unsafe(link_section = ".text.slate_distinct.listobject.list_contains")]
extern "C-unwind" fn list_contains(aa: *mut _object, el: *mut _object) -> i32 {
    let mut i: i64 = 0;
    loop {
        let item: *mut _object = list_get_item_ref(aa as *mut __SlateRecord_listobject_h_22, i);
        if item.is_null() {
            // out-of-bounds
            return 0;
        }
        let cmp: i32 = unsafe { PyObject_RichCompareBool(item, el, 2i32) };
        Py_DECREF!(item);
        if cmp != 0 {
            return cmp;
        }
        i = i.wrapping_add(1);
    }
    0
}

#[unsafe(link_section = ".text.slate_distinct.listobject.list_item")]
extern "C-unwind" fn list_item(aa: *mut _object, i: i64) -> *mut _object {
    let a: *mut __SlateRecord_listobject_h_22 = aa as *mut __SlateRecord_listobject_h_22;
    if valid_index(i, PyList_GET_SIZE__2!(a)) == 0 {
        unsafe {
            PyErr_SetObject(
                PyExc_IndexError,
                core::ptr::addr_of_mut!(
                    _PyRuntime
                        .static_objects
                        .singletons
                        .strings
                        .literals
                        ._py_list_err
                        ._ascii
                        .ob_base
                ),
            )
        };
        return core::ptr::null_mut::<_object>();
    }
    let item: *mut _object =
        Py_NewRef!(unsafe { *unsafe { (*a).ob_item }.wrapping_offset(i as isize) });
    item
}

fn list_slice_lock_held(
    a: *mut __SlateRecord_listobject_h_22,
    ilow: i64,
    ihigh: i64,
) -> *mut _object {
    let len: i64 = ihigh.wrapping_sub(ilow);
    if len <= 0 {
        return PyList_New(0i64);
    }
    let np: *mut __SlateRecord_listobject_h_22 =
        list_new_prealloc(len) as *mut __SlateRecord_listobject_h_22;
    if np.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let src: *mut *mut _object = unsafe { (*a).ob_item }.wrapping_offset(ilow as isize);
    let dest: *mut *mut _object = unsafe { (*np).ob_item };
    for i in 0..len {
        let v: *mut _object = unsafe { *src.wrapping_offset(i as isize) };
        unsafe {
            *dest.wrapping_offset(i as isize) = Py_NewRef!(v);
        }
    }
    Py_SET_SIZE!(np, len);
    np as *mut _object
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyList_GetSlice(
    a: *mut _object,
    mut ilow: i64,
    mut ihigh: i64,
) -> *mut _object {
    if PyList_Check!(a) == 0 {
        PyErr_BadInternalCall__15!();
        return core::ptr::null_mut::<_object>();
    }
    if ilow < 0 {
        ilow = 0;
    } else {
        if ilow > Py_SIZE__2!(a) {
            ilow = Py_SIZE__2!(a);
        }
    }
    if ihigh < ilow {
        ihigh = ilow;
    } else {
        if ihigh > Py_SIZE__2!(a) {
            ihigh = Py_SIZE__2!(a);
        }
    }
    let ret: *mut _object =
        list_slice_lock_held(a as *mut __SlateRecord_listobject_h_22, ilow, ihigh);
    ret
}

fn list_concat_lock_held(
    a: *mut __SlateRecord_listobject_h_22,
    b: *mut __SlateRecord_listobject_h_22,
) -> *mut _object {
    let mut size: i64 = 0;
    let mut i: i64 = 0;
    let mut src: *mut *mut _object = unsafe { core::mem::zeroed() };
    let mut dest: *mut *mut _object = unsafe { core::mem::zeroed() };
    size = Py_SIZE!(a).wrapping_add(Py_SIZE!(b));
    if size == 0 {
        return PyList_New(0i64);
    }
    let np: *mut __SlateRecord_listobject_h_22 =
        list_new_prealloc(size) as *mut __SlateRecord_listobject_h_22;
    if np.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    src = unsafe { (*a).ob_item };
    dest = unsafe { (*np).ob_item };
    while i < Py_SIZE!(a) {
        let v: *mut _object = unsafe { *src.wrapping_offset(i as isize) };
        unsafe {
            *dest.wrapping_offset(i as isize) = Py_NewRef!(v);
        }
        i = i.wrapping_add(1);
    }
    src = unsafe { (*b).ob_item };
    dest = unsafe { (*np).ob_item }.wrapping_offset(Py_SIZE!(a) as isize);
    i = 0;
    while i < Py_SIZE!(b) {
        let v: *mut _object = unsafe { *src.wrapping_offset(i as isize) };
        unsafe {
            *dest.wrapping_offset(i as isize) = Py_NewRef!(v);
        }
        i = i.wrapping_add(1);
    }
    Py_SET_SIZE!(np, size);
    np as *mut _object
}

#[unsafe(link_section = ".text.slate_distinct.listobject.list_concat")]
extern "C-unwind" fn list_concat(aa: *mut _object, bb: *mut _object) -> *mut _object {
    if PyList_Check!(bb) == 0 {
        unsafe {
            PyErr_Format(
                PyExc_TypeError,
                c"can only concatenate list (not \"%.200s\") to list".as_ptr(),
                (*Py_TYPE!(bb)).tp_name,
            )
        };
        return core::ptr::null_mut::<_object>();
    }
    let a: *mut __SlateRecord_listobject_h_22 = aa as *mut __SlateRecord_listobject_h_22;
    let b: *mut __SlateRecord_listobject_h_22 = bb as *mut __SlateRecord_listobject_h_22;
    let ret: *mut _object = list_concat_lock_held(a, b);
    ret
}

fn list_repeat_lock_held(a: *mut __SlateRecord_listobject_h_22, n: i64) -> *mut _object {
    let input_size: i64 = Py_SIZE!(a);
    if input_size == 0 || n <= 0 {
        return PyList_New(0i64);
    }
    if input_size > 9223372036854775807 / n {
        return unsafe { PyErr_NoMemory() };
    }
    let output_size: i64 = input_size.wrapping_mul(n);
    let np: *mut __SlateRecord_listobject_h_22 =
        list_new_prealloc(output_size) as *mut __SlateRecord_listobject_h_22;
    if np.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let mut dest: *mut *mut _object = unsafe { (*np).ob_item };
    if input_size == 1 {
        let elem: *mut _object = unsafe { *unsafe { (*a).ob_item }.wrapping_offset(0 as isize) };
        _Py_RefcntAdd!(elem, n);
        let dest_end: *mut *mut _object = dest.wrapping_offset(output_size as isize);
        while dest < dest_end {
            let __v0: *mut *mut _object = dest;
            dest = __v0.wrapping_offset(1 as isize);
            unsafe {
                *__v0 = elem;
            }
        }
    } else {
        let mut src: *mut *mut _object = unsafe { (*a).ob_item };
        let src_end: *mut *mut _object = src.wrapping_offset(input_size as isize);
        while src < src_end {
            _Py_RefcntAdd!(unsafe { *src }, n);
            let __v3: *mut *mut _object = src;
            src = __v3.wrapping_offset(1 as isize);
            let __v5: *mut *mut _object = dest;
            dest = __v5.wrapping_offset(1 as isize);
            unsafe {
                *__v5 = *__v3;
            }
        }
        // TODO: _Py_memory_repeat calls are not safe for shared lists in
        //
        // GIL_DISABLED builds. (See issue #129069)
        _Py_memory_repeat(
            (unsafe { (*np).ob_item }) as *mut i8,
            (core::mem::size_of::<*mut _object>() as u64).wrapping_mul(output_size as u64) as i64,
            (core::mem::size_of::<*mut _object>() as u64).wrapping_mul(input_size as u64) as i64,
        );
    }
    Py_SET_SIZE!(np, output_size);
    np as *mut _object
}

#[unsafe(link_section = ".text.slate_distinct.listobject.list_repeat")]
extern "C-unwind" fn list_repeat(aa: *mut _object, n: i64) -> *mut _object {
    let a: *mut __SlateRecord_listobject_h_22 = aa as *mut __SlateRecord_listobject_h_22;
    let ret: *mut _object = list_repeat_lock_held(a, n);
    ret
}

fn list_clear_impl(a: *mut __SlateRecord_listobject_h_22, is_resize: bool) {
    let items: *mut *mut _object = unsafe { (*a).ob_item };
    if items.is_null() {
        return;
    }
    // Because XDECREF can recursively invoke operations on
    // this list, we make it empty first.
    let mut i: i64 = Py_SIZE!(a);
    Py_SET_SIZE__2!(a, 0i32);
    FT_ATOMIC_STORE_PTR_RELEASE__3!((*a).ob_item);
    unsafe {
        (*a).allocated = 0;
    }
    loop {
        let __v1: i64 = i.wrapping_sub(1);
        i = __v1;
        if __v1 < 0 {
            break;
        }
        Py_XDECREF!(unsafe { *items.wrapping_offset(i as isize) });
    }
    let use_qsbr: bool = false;
    free_list_items(items, use_qsbr);
    // Note that there is no guarantee that the list is actually empty
    //
    // at this point, because XDECREF may have populated it indirectly again!
}

fn list_clear(a: *mut __SlateRecord_listobject_h_22) {
    list_clear_impl(a, true);
}

#[unsafe(link_section = ".text.slate_distinct.listobject.list_clear_slot")]
extern "C-unwind" fn list_clear_slot(self_: *mut _object) -> i32 {
    list_clear_impl(self_ as *mut __SlateRecord_listobject_h_22, false);
    0
}

/// a[ilow:ihigh] = v if v != NULL.
/// del a[ilow:ihigh] if v == NULL.
///
/// Special speed gimmick:  when v is NULL and ihigh - ilow <= 8, it's
/// guaranteed the call cannot fail.
fn list_ass_slice_lock_held(
    a: *mut __SlateRecord_listobject_h_22,
    mut ilow: i64,
    mut ihigh: i64,
    v: *mut _object,
) -> i32 {
    // guilty until proved innocent
    let mut result: i32 = unsafe { core::mem::zeroed() };
    let mut s: u64 = unsafe { core::mem::zeroed() };
    let mut k: i64 = unsafe { core::mem::zeroed() };
    // # of elements in list getting replaced
    let mut norig: i64 = unsafe { core::mem::zeroed() };
    // # of elements in replacement list
    let mut n: i64 = unsafe { core::mem::zeroed() };
    // PySequence_Fast(v)
    let mut v_as_SF: *mut _object = unsafe { core::mem::zeroed() };
    let mut vitem: *mut *mut _object = unsafe { core::mem::zeroed() };
    let mut item: *mut *mut _object = unsafe { core::mem::zeroed() };
    // will allocate more if needed
    let mut recycle: *mut *mut _object = unsafe { core::mem::zeroed() };
    // Because [X]DECREF can recursively invoke list operations on
    // this list, we must postpone all [X]DECREF activity until
    // after the list is back in its canonical shape.  Therefore
    // we must allocate an additional array, 'recycle', into which
    // we temporarily copy the items that are deleted from the
    // list. :-(
    let mut recycle_on_stack: core::mem::MaybeUninit<__SlateAlign16<[*mut _object; 8]>> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_18: *mut [*mut _object; 8] =
        core::ptr::addr_of_mut!(recycle_on_stack) as *mut [*mut _object; 8];
    unsafe {
        '__join_2: {
            recycle = (*__slate_slot_18).as_mut_ptr() as *mut *mut _object;
            vitem = core::ptr::null_mut::<*mut _object>();
            v_as_SF = core::ptr::null_mut::<_object>();
            result = 1i32.wrapping_neg();
            if v.is_null() {
                n = 0;
            } else {
                v_as_SF = PySequence_Fast(v, c"can only assign an iterable".as_ptr());
                if v_as_SF.is_null() {
                    break '__join_2;
                } else {
                    n = PySequence_Fast_GET_SIZE!(v_as_SF);
                    vitem = PySequence_Fast_ITEMS!(v_as_SF);
                }
            }
            if ilow < 0 {
                ilow = 0;
            } else {
                if ilow > Py_SIZE!(a) {
                    ilow = Py_SIZE!(a);
                }
            }
            if ihigh < ilow {
                ihigh = ilow;
            } else {
                if ihigh > Py_SIZE!(a) {
                    ihigh = Py_SIZE!(a);
                }
            }
            norig = ihigh.wrapping_sub(ilow);
            // Change in size
            let d: i64 = n.wrapping_sub(norig);
            if Py_SIZE!(a).wrapping_add(d) == 0 {
                Py_XDECREF!(v_as_SF);
                list_clear(a);
                return 0;
            } else {
                item = (*a).ob_item;
                // recycle the items that we are about to remove
                s = (norig as u64).wrapping_mul(core::mem::size_of::<*mut _object>() as u64);
                // If norig == 0, item might be NULL, in which case we may not memcpy from it.
                if s != 0 {
                    if s > (core::mem::size_of::<[*mut _object; 8]>() as u64) {
                        recycle = PyMem_Malloc(s) as *mut *mut _object;
                        if recycle.is_null() {
                            PyErr_NoMemory();
                            break '__join_2;
                        }
                    }
                    core::ptr::copy_nonoverlapping(
                        ((item.wrapping_offset(ilow as isize) as *const ())
                            as *const core::ffi::c_void) as *const u8,
                        ((recycle as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                        (s as usize) as usize,
                    );
                }
                if d < 0 {
                    // Delete -d items
                    let tail: i64 = (Py_SIZE!(a).wrapping_sub(ihigh) as u64)
                        .wrapping_mul(core::mem::size_of::<*mut _object>() as u64)
                        as i64;
                    // TODO: these memmove/memcpy calls are not safe for shared lists in
                    //
                    // GIL_DISABLED builds. (See issue #129069)
                    core::ptr::copy(
                        ((item.wrapping_offset(ihigh as isize) as *const ())
                            as *const core::ffi::c_void) as *const u8,
                        ((item.wrapping_offset(ihigh.wrapping_add(d) as isize) as *mut ())
                            as *mut core::ffi::c_void) as *mut u8,
                        ((tail as u64) as usize) as usize,
                    );
                    if list_resize(a, Py_SIZE!(a).wrapping_add(d)) < 0 {
                        core::ptr::copy(
                            ((item.wrapping_offset(ihigh.wrapping_add(d) as isize) as *const ())
                                as *const core::ffi::c_void)
                                as *const u8,
                            ((item.wrapping_offset(ihigh as isize) as *mut ())
                                as *mut core::ffi::c_void) as *mut u8,
                            ((tail as u64) as usize) as usize,
                        );
                        core::ptr::copy_nonoverlapping(
                            ((recycle as *const ()) as *const core::ffi::c_void) as *const u8,
                            ((item.wrapping_offset(ilow as isize) as *mut ())
                                as *mut core::ffi::c_void) as *mut u8,
                            (s as usize) as usize,
                        );
                        break '__join_2;
                    } else {
                        item = (*a).ob_item;
                    }
                } else {
                    if d > 0 {
                        // Insert d items
                        k = Py_SIZE!(a);
                        if list_resize(a, k.wrapping_add(d)) < 0 {
                            break '__join_2;
                        } else {
                            item = (*a).ob_item;
                            // TODO: these memmove/memcpy calls are not safe for shared lists in
                            //
                            // GIL_DISABLED builds. (See issue #129069)
                            core::ptr::copy(
                                ((item.wrapping_offset(ihigh as isize) as *const ())
                                    as *const core::ffi::c_void)
                                    as *const u8,
                                ((item.wrapping_offset(ihigh.wrapping_add(d) as isize) as *mut ())
                                    as *mut core::ffi::c_void)
                                    as *mut u8,
                                ((k.wrapping_sub(ihigh) as u64).wrapping_mul(core::mem::size_of::<
                                    *mut _object,
                                >(
                                )
                                    as u64) as usize) as usize,
                            );
                        }
                    }
                }
                k = 0;
                while k < n {
                    let w: *mut _object = *vitem.wrapping_offset(k as isize);
                    FT_ATOMIC_STORE_PTR_RELEASE!(
                        *item.wrapping_offset(ilow as isize),
                        Py_XNewRef!(w)
                    );
                    k = k.wrapping_add(1);
                    ilow = ilow.wrapping_add(1);
                }
                k = norig.wrapping_sub(1);
                while k >= 0 {
                    Py_XDECREF!(unsafe { *recycle.wrapping_offset(k as isize) });
                    k = k.wrapping_sub(1);
                }
                result = 0;
            }
        }
        if recycle != ((*__slate_slot_18).as_mut_ptr() as *mut *mut _object) {
            PyMem_Free(recycle as *mut ());
        }
        Py_XDECREF!(v_as_SF);
        return result;
    }
}

fn list_ass_slice(
    a: *mut __SlateRecord_listobject_h_22,
    ilow: i64,
    ihigh: i64,
    v: *mut _object,
) -> i32 {
    let mut ret: i32 = 0;
    if a == (v as *mut __SlateRecord_listobject_h_22) {
        let n: i64 = PyList_GET_SIZE__2!(a);
        let copy: *mut _object = list_slice_lock_held(a, 0i64, n);
        if copy.is_null() {
            ret = 1i32.wrapping_neg();
        } else {
            ret = list_ass_slice_lock_held(a, ilow, ihigh, copy);
            Py_DECREF!(copy);
        }
    } else {
        let __v0: bool = !v.is_null() && PyList_CheckExact!(v) != 0;
        if __v0 {
            ret = list_ass_slice_lock_held(a, ilow, ihigh, v);
        } else {
            ret = list_ass_slice_lock_held(a, ilow, ihigh, v);
        }
    }
    ret
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyList_SetSlice(
    a: *mut _object,
    ilow: i64,
    ihigh: i64,
    v: *mut _object,
) -> i32 {
    if PyList_Check!(a) == 0 {
        PyErr_BadInternalCall__6!();
        return 1i32.wrapping_neg();
    }
    list_ass_slice(a as *mut __SlateRecord_listobject_h_22, ilow, ihigh, v)
}

fn list_inplace_repeat_lock_held(self_: *mut __SlateRecord_listobject_h_22, n: i64) -> i32 {
    let input_size: i64 = PyList_GET_SIZE__2!(self_);
    if input_size == 0 || n == 1 {
        return 0;
    }
    if n < 1 {
        list_clear(self_);
        return 0;
    }
    if input_size > 9223372036854775807 / n {
        unsafe { PyErr_NoMemory() };
        return 1i32.wrapping_neg();
    }
    let output_size: i64 = input_size.wrapping_mul(n);
    if list_resize(self_, output_size) < 0 {
        return 1i32.wrapping_neg();
    }
    let items: *mut *mut _object = unsafe { (*self_).ob_item };
    for j in 0..input_size {
        _Py_RefcntAdd!(
            unsafe { *items.wrapping_offset(j as isize) },
            n.wrapping_sub(1)
        );
    }
    // TODO: _Py_memory_repeat calls are not safe for shared lists in
    //
    // GIL_DISABLED builds. (See issue #129069)
    _Py_memory_repeat(
        items as *mut i8,
        (core::mem::size_of::<*mut _object>() as u64).wrapping_mul(output_size as u64) as i64,
        (core::mem::size_of::<*mut _object>() as u64).wrapping_mul(input_size as u64) as i64,
    );
    0
}

#[unsafe(link_section = ".text.slate_distinct.listobject.list_inplace_repeat")]
extern "C-unwind" fn list_inplace_repeat(_self: *mut _object, n: i64) -> *mut _object {
    let mut ret: *mut _object = unsafe { core::mem::zeroed() };
    let self_: *mut __SlateRecord_listobject_h_22 = _self as *mut __SlateRecord_listobject_h_22;
    if list_inplace_repeat_lock_held(self_, n) < 0 {
        ret = core::ptr::null_mut::<_object>();
    } else {
        ret = Py_NewRef__2!(self_);
    }
    ret
}

fn list_ass_item_lock_held(a: *mut __SlateRecord_listobject_h_22, i: i64, v: *mut _object) -> i32 {
    if valid_index(i, Py_SIZE!(a)) == 0 {
        unsafe {
            PyErr_SetString(
                PyExc_IndexError,
                c"list assignment index out of range".as_ptr(),
            )
        };
        return 1i32.wrapping_neg();
    }
    let tmp: *mut _object = unsafe { *unsafe { (*a).ob_item }.wrapping_offset(i as isize) };
    if v.is_null() {
        let size: i64 = Py_SIZE!(a);
        for idx in i..size.wrapping_sub(1) {
            FT_ATOMIC_STORE_PTR_RELAXED!(
                *unsafe { (*a).ob_item }.wrapping_offset(idx as isize),
                unsafe { *unsafe { (*a).ob_item }.wrapping_offset(idx.wrapping_add(1) as isize) }
            );
        }
        Py_SET_SIZE!(a, size.wrapping_sub(1));
    } else {
        FT_ATOMIC_STORE_PTR_RELEASE!(
            *unsafe { (*a).ob_item }.wrapping_offset(i as isize),
            Py_NewRef!(v)
        );
    }
    Py_DECREF!(tmp);
    0
}

#[unsafe(link_section = ".text.slate_distinct.listobject.list_ass_item")]
extern "C-unwind" fn list_ass_item(aa: *mut _object, i: i64, v: *mut _object) -> i32 {
    let mut ret: i32 = 0;
    let a: *mut __SlateRecord_listobject_h_22 = aa as *mut __SlateRecord_listobject_h_22;
    ret = list_ass_item_lock_held(a, i, v);
    ret
}

// [clinic input]
// @critical_section
// list.insert
//
//     index: Py_ssize_t
//     object: object
//     /
//
// Insert object before index.
// [clinic start generated code]
/// [clinic end generated code: output=7f35e32f60c8cb78 input=b1987ca998a4ae2d]
fn list_insert_impl(
    self_: *mut __SlateRecord_listobject_h_22,
    index: i64,
    object: *mut _object,
) -> *mut _object {
    if ins1(self_, index, object) == 0 {
        return unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) };
    }
    core::ptr::null_mut::<_object>()
}

// [clinic input]
// @critical_section
// list.clear as py_list_clear
//
// Remove all items from list.
// [clinic start generated code]
/// [clinic end generated code: output=83726743807e3518 input=e285b7f09051a9ba]
fn py_list_clear_impl(self_: *mut __SlateRecord_listobject_h_22) -> *mut _object {
    list_clear(self_);
    unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
}

// [clinic input]
// @critical_section
// list.copy
//
// Return a shallow copy of the list.
// [clinic start generated code]
/// [clinic end generated code: output=ec6b72d6209d418e input=81c54b0c7bb4f73d]
fn list_copy_impl(self_: *mut __SlateRecord_listobject_h_22) -> *mut _object {
    list_slice_lock_held(self_, 0i64, Py_SIZE!(self_))
}

// [clinic input]
// @critical_section
// list.append
//
//      object: object
//      /
//
// Append object to the end of the list.
// [clinic start generated code]
/// [clinic end generated code: output=78423561d92ed405 input=122b0853de54004f]
fn list_append_impl(
    self_: *mut __SlateRecord_listobject_h_22,
    object: *mut _object,
) -> *mut _object {
    if _PyList_AppendTakeRef(self_, Py_NewRef!(object)) < 0 {
        return core::ptr::null_mut::<_object>();
    }
    unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
}

fn list_extend_fast(self_: *mut __SlateRecord_listobject_h_22, iterable: *mut _object) -> i32 {
    let n: i64 = PySequence_Fast_GET_SIZE!(iterable);
    if n == 0 {
        // short circuit when iterable is empty
        return 0;
    }
    let m: i64 = Py_SIZE!(self_);
    // It should not be possible to allocate a list large enough to cause
    //
    // an overflow on any relevant platform.
    if unsafe { (*self_).ob_item }.is_null() {
        if list_preallocate_exact(self_, n) < 0 {
            return 1i32.wrapping_neg();
        }
        Py_SET_SIZE!(self_, n);
    } else {
        if list_resize(self_, m.wrapping_add(n)) < 0 {
            return 1i32.wrapping_neg();
        }
    }
    // note that we may still have self == iterable here for the
    //
    // situation a.extend(a), but the following code works
    //
    // in that case too.  Just make sure to resize self
    //
    // before calling PySequence_Fast_ITEMS.
    //
    //
    //
    // populate the end of self with iterable's items.
    let src: *mut *mut _object = PySequence_Fast_ITEMS!(iterable);
    let dest: *mut *mut _object = unsafe { (*self_).ob_item }.wrapping_offset(m as isize);
    for i in 0..n {
        let o: *mut _object = unsafe { *src.wrapping_offset(i as isize) };
        FT_ATOMIC_STORE_PTR_RELEASE!(*dest.wrapping_offset(i as isize), Py_NewRef!(o));
    }
    0
}

fn list_extend_iter_lock_held(
    self_: *mut __SlateRecord_listobject_h_22,
    iterable: *mut _object,
) -> i32 {
    let mut len: i64 = unsafe { core::mem::zeroed() };
    let mut item: *mut _object = unsafe { core::mem::zeroed() };
    let mut __v2: bool = unsafe { core::mem::zeroed() };
    let mut m: i64 = unsafe { core::mem::zeroed() };
    // Guess a result list size.
    let mut n: i64 = unsafe { core::mem::zeroed() };
    let mut iternext: core::mem::MaybeUninit<
        Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
    > = core::mem::MaybeUninit::uninit();
    let __slate_slot_5: *mut Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object> =
        core::ptr::addr_of_mut!(iternext)
            as *mut Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>;
    let mut it: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        it = PyObject_GetIter(iterable);
        if it.is_null() {
            return 1i32.wrapping_neg();
        } else {
            core::ptr::write(__slate_slot_5, (*Py_TYPE!(it)).tp_iternext);
            n = PyObject_LengthHint(iterable, 8i64);
            if n < 0 {
                Py_DECREF!(it);
                return 1i32.wrapping_neg();
            } else {
                '__join_0: {
                    m = Py_SIZE!(self_);
                    if m > 9223372036854775807i64.wrapping_sub(n) {
                        // m + n overflowed; on the chance that n lied, and there really
                        // is enough room, ignore it.  If n was telling the truth, we'll
                        // eventually run out of memory during the loop.
                    } else {
                        if unsafe { (*self_).ob_item }.is_null() {
                            if n != 0 {
                                __v2 = list_preallocate_exact(self_, n) < 0;
                            } else {
                                __v2 = false;
                            }
                            if __v2 {
                                break '__join_0;
                            }
                        } else {
                            // Make room.
                            if list_resize(self_, m.wrapping_add(n)) < 0 {
                                break '__join_0;
                            } else {
                                // Make the list sane again.
                                Py_SET_SIZE!(self_, m);
                            }
                        }
                    }
                    // Run iterator to exhaustion.
                    loop {
                        item = (*__slate_slot_5).unwrap()(it);
                        if item.is_null() {
                            break;
                        } else {
                            if Py_SIZE!(self_) < (*self_).allocated {
                                len = Py_SIZE!(self_);
                                FT_ATOMIC_STORE_PTR_RELEASE!(
                                    *unsafe { (*self_).ob_item }.wrapping_offset(len as isize),
                                    item
                                ); // steals item ref
                                Py_SET_SIZE!(self_, len.wrapping_add(1));
                            } else {
                                if _PyList_AppendTakeRef(self_, item) < 0 {
                                    break '__join_0;
                                }
                            }
                        }
                    }
                    if !PyErr_Occurred().is_null() {
                        if PyErr_ExceptionMatches(PyExc_StopIteration) != 0 {
                            PyErr_Clear();
                        } else {
                            break '__join_0;
                        }
                    }
                    // Cut back result list if initial guess was too large.
                    if Py_SIZE!(self_) < (*self_).allocated {
                        if list_resize(self_, Py_SIZE!(self_)) < 0 {
                            break '__join_0;
                        }
                    }
                    Py_DECREF!(it);
                    return 0;
                }
                Py_DECREF!(it);
                return 1i32.wrapping_neg();
            }
        }
    }
}

fn list_extend_lock_held(self_: *mut __SlateRecord_listobject_h_22, iterable: *mut _object) -> i32 {
    let seq: *mut _object =
        unsafe { PySequence_Fast(iterable, c"argument must be iterable".as_ptr()) };
    if seq.is_null() {
        return 1i32.wrapping_neg();
    }
    let res: i32 = list_extend_fast(self_, seq);
    Py_DECREF!(seq);
    res
}

fn list_extend_set(
    self_: *mut __SlateRecord_listobject_h_22,
    other: *mut __SlateRecord_setobject_h_59,
) -> i32 {
    let m: i64 = Py_SIZE!(self_);
    let n: i64 = PySet_GET_SIZE__2!(other);
    let r: i64 = m.wrapping_add(n);
    if r == 0 {
        return 0;
    }
    if list_resize(self_, r) < 0 {
        return 1i32.wrapping_neg();
    }
    // populate the end of self with iterable's items
    let mut setpos: i64 = 0;
    let mut hash: i64 = 0;
    let mut key: *mut _object = unsafe { core::mem::zeroed() };
    let mut dest: *mut *mut _object = unsafe { (*self_).ob_item }.wrapping_offset(m as isize);
    while (unsafe {
        _PySet_NextEntryRef(
            other as *mut _object,
            core::ptr::addr_of_mut!(setpos),
            core::ptr::addr_of_mut!(key),
            core::ptr::addr_of_mut!(hash),
        )
    }) != 0
    {
        FT_ATOMIC_STORE_PTR_RELEASE!(*dest, key);
        let __v0: *mut *mut _object = dest;
        dest = __v0.wrapping_offset(1 as isize);
    }
    Py_SET_SIZE!(self_, r);
    0
}

fn list_extend_dict(
    self_: *mut __SlateRecord_listobject_h_22,
    dict: *mut __SlateRecord_dictobject_h_33,
    which_item: i32,
) -> i32 {
    // which_item: 0 for keys and 1 for values
    let m: i64 = Py_SIZE!(self_);
    let n: i64 = PyDict_GET_SIZE__2!(dict);
    let r: i64 = m.wrapping_add(n);
    if r == 0 {
        return 0;
    }
    if list_resize(self_, r) < 0 {
        return 1i32.wrapping_neg();
    }
    let mut dest: *mut *mut _object = unsafe { (*self_).ob_item }.wrapping_offset(m as isize);
    let mut pos: i64 = 0;
    let mut keyvalue: __SlateAlign16<[*mut _object; 2]> = __SlateAlign16([0 as *mut _object; 2]);
    while (unsafe {
        _PyDict_Next(
            dict as *mut _object,
            core::ptr::addr_of_mut!(pos),
            (keyvalue.0.as_mut_ptr() as *mut *mut _object).wrapping_offset(0 as isize),
            (keyvalue.0.as_mut_ptr() as *mut *mut _object).wrapping_offset(1 as isize),
            core::ptr::null_mut::<i64>(),
        )
    }) != 0
    {
        let obj: *mut _object = keyvalue.0[which_item as usize];
        Py_INCREF!(obj);
        FT_ATOMIC_STORE_PTR_RELEASE!(*dest, obj);
        let __v0: *mut *mut _object = dest;
        dest = __v0.wrapping_offset(1 as isize);
    }
    Py_SET_SIZE!(self_, r);
    0
}

fn list_extend_dictitems(
    self_: *mut __SlateRecord_listobject_h_22,
    dict: *mut __SlateRecord_dictobject_h_33,
) -> i32 {
    let m: i64 = Py_SIZE!(self_);
    let n: i64 = PyDict_GET_SIZE__2!(dict);
    let r: i64 = m.wrapping_add(n);
    if r == 0 {
        return 0;
    }
    if list_resize(self_, r) < 0 {
        return 1i32.wrapping_neg();
    }
    let mut dest: *mut *mut _object = unsafe { (*self_).ob_item }.wrapping_offset(m as isize);
    let mut pos: i64 = 0;
    let mut i: i64 = 0;
    let mut key: *mut _object = unsafe { core::mem::zeroed() };
    let mut value: *mut _object = unsafe { core::mem::zeroed() };
    while (unsafe {
        _PyDict_Next(
            dict as *mut _object,
            core::ptr::addr_of_mut!(pos),
            core::ptr::addr_of_mut!(key),
            core::ptr::addr_of_mut!(value),
            core::ptr::null_mut::<i64>(),
        )
    }) != 0
    {
        let item: *mut _object = unsafe { PyTuple_Pack(2i64, key, value) };
        if item.is_null() {
            Py_SET_SIZE!(self_, m.wrapping_add(i));
            return 1i32.wrapping_neg();
        }
        FT_ATOMIC_STORE_PTR_RELEASE!(*dest, item);
        let __v0: *mut *mut _object = dest;
        dest = __v0.wrapping_offset(1 as isize);
        i = i.wrapping_add(1);
    }
    Py_SET_SIZE!(self_, r);
    0
}

fn _list_extend(self_: *mut __SlateRecord_listobject_h_22, iterable: *mut _object) -> i32 {
    // Special case:
    //
    // lists and tuples which can use PySequence_Fast ops
    let mut res: i32 = 1i32.wrapping_neg();
    if (self_ as *mut _object) == iterable {
        res = list_inplace_repeat_lock_held(self_, 2i64);
    } else {
        if PyList_CheckExact!(iterable) != 0 {
            res = list_extend_lock_held(self_, iterable);
        } else {
            if PyTuple_CheckExact!(iterable) != 0 {
                res = list_extend_lock_held(self_, iterable);
            } else {
                if PyAnySet_CheckExact!(iterable) {
                    res = list_extend_set(self_, iterable as *mut __SlateRecord_setobject_h_59);
                } else {
                    if PyDict_CheckExact!(iterable) != 0 {
                        res = list_extend_dict(
                            self_,
                            iterable as *mut __SlateRecord_dictobject_h_33,
                            0i32,
                        ); // keys
                    } else {
                        if Py_IS_TYPE!(iterable, unsafe {
                            core::ptr::addr_of_mut!(PyDictKeys_Type)
                        }) != 0
                        {
                            let dict: *mut __SlateRecord_dictobject_h_33 = unsafe {
                                (*(iterable as *mut __SlateRecord_pycore_dict_h_73)).dv_dict
                            };
                            res = list_extend_dict(self_, dict, 0i32); // keys
                        } else {
                            if Py_IS_TYPE!(iterable, unsafe {
                                core::ptr::addr_of_mut!(PyDictValues_Type)
                            }) != 0
                            {
                                let dict: *mut __SlateRecord_dictobject_h_33 = unsafe {
                                    (*(iterable as *mut __SlateRecord_pycore_dict_h_73)).dv_dict
                                };
                                res = list_extend_dict(self_, dict, 1i32); // values
                            } else {
                                if Py_IS_TYPE!(iterable, unsafe {
                                    core::ptr::addr_of_mut!(PyDictItems_Type)
                                }) != 0
                                {
                                    let dict: *mut __SlateRecord_dictobject_h_33 = unsafe {
                                        (*(iterable as *mut __SlateRecord_pycore_dict_h_73)).dv_dict
                                    };
                                    res = list_extend_dictitems(self_, dict);
                                } else {
                                    res = list_extend_iter_lock_held(self_, iterable);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    res
}

// [clinic input]
// list.extend as list_extend
//
//      iterable: object
//      /
//
// Extend list by appending elements from the iterable.
// [clinic start generated code]
/// [clinic end generated code: output=b0eba9e0b186d5ce input=979da7597a515791]
fn list_extend_impl(
    self_: *mut __SlateRecord_listobject_h_22,
    iterable: *mut _object,
) -> *mut _object {
    if _list_extend(self_, iterable) < 0 {
        return core::ptr::null_mut::<_object>();
    }
    unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyList_Extend(
    self_: *mut __SlateRecord_listobject_h_22,
    iterable: *mut _object,
) -> *mut _object {
    list_extend(self_ as *mut _object, iterable)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyList_Extend(self_: *mut _object, iterable: *mut _object) -> i32 {
    if PyList_Check!(self_) == 0 {
        PyErr_BadInternalCall__65!();
        return 1i32.wrapping_neg();
    }
    _list_extend(self_ as *mut __SlateRecord_listobject_h_22, iterable)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyList_Clear(self_: *mut _object) -> i32 {
    if PyList_Check!(self_) == 0 {
        PyErr_BadInternalCall__91!();
        return 1i32.wrapping_neg();
    }
    list_clear(self_ as *mut __SlateRecord_listobject_h_22);
    0
}

#[unsafe(link_section = ".text.slate_distinct.listobject.list_inplace_concat")]
extern "C-unwind" fn list_inplace_concat(_self: *mut _object, other: *mut _object) -> *mut _object {
    let self_: *mut __SlateRecord_listobject_h_22 = _self as *mut __SlateRecord_listobject_h_22;
    if _list_extend(self_, other) < 0 {
        return core::ptr::null_mut::<_object>();
    }
    Py_NewRef__2!(self_)
}

// [clinic input]
// @critical_section
// list.pop
//
//     index: Py_ssize_t = -1
//     /
//
// Remove and return item at index (default last).
//
// Raises IndexError if list is empty or index is out of range.
// [clinic start generated code]
/// [clinic end generated code: output=6bd69dcb3f17eca8 input=c269141068ae4b8f]
fn list_pop_impl(self_: *mut __SlateRecord_listobject_h_22, mut index: i64) -> *mut _object {
    let mut status: i32 = 0;
    if Py_SIZE!(self_) == 0 {
        // Special-case most common failure cause
        unsafe { PyErr_SetString(PyExc_IndexError, c"pop from empty list".as_ptr()) };
        return core::ptr::null_mut::<_object>();
    }
    if index < 0 {
        let __v0: i64 = Py_SIZE!(self_);
        index = index.wrapping_add(__v0);
    }
    if valid_index(index, Py_SIZE!(self_)) == 0 {
        unsafe { PyErr_SetString(PyExc_IndexError, c"pop index out of range".as_ptr()) };
        return core::ptr::null_mut::<_object>();
    }
    let items: *mut *mut _object = unsafe { (*self_).ob_item };
    let v: *mut _object = unsafe { *items.wrapping_offset(index as isize) };
    let size_after_pop: i64 = Py_SIZE!(self_).wrapping_sub(1);
    if size_after_pop == 0 {
        Py_INCREF!(v);
        list_clear(self_);
        status = 0;
    } else {
        if size_after_pop.wrapping_sub(index) > 0 {
            unsafe {
                core::ptr::copy(
                    ((items.wrapping_offset(index.wrapping_add(1) as isize) as *const ())
                        as *const core::ffi::c_void) as *const u8,
                    ((items.wrapping_offset(index as isize) as *mut ()) as *mut core::ffi::c_void)
                        as *mut u8,
                    ((size_after_pop.wrapping_sub(index) as u64).wrapping_mul(core::mem::size_of::<
                        *mut _object,
                    >(
                    )
                        as u64) as usize) as usize,
                )
            };
        }
        status = list_resize(self_, size_after_pop);
    }
    if status >= 0 {
        return v; // and v now owns the reference the list had
    } else {
        // list resize failed, need to restore
        unsafe {
            core::ptr::copy(
                ((items.wrapping_offset(index as isize) as *const ()) as *const core::ffi::c_void)
                    as *const u8,
                ((items.wrapping_offset(index.wrapping_add(1) as isize) as *mut ())
                    as *mut core::ffi::c_void) as *mut u8,
                ((size_after_pop.wrapping_sub(index) as u64)
                    .wrapping_mul(core::mem::size_of::<*mut _object>() as u64)
                    as usize) as usize,
            )
        };
        unsafe {
            *items.wrapping_offset(index as isize) = v;
        }
        return core::ptr::null_mut::<_object>();
    }
}

/// Reverse a slice of a list in place, from lo up to (exclusive) hi.
fn reverse_slice(mut lo: *mut *mut _object, mut hi: *mut *mut _object) {
    let __v0: *mut *mut _object = hi;
    hi = __v0.wrapping_offset(-(1 as isize));
    while lo < hi {
        let t: *mut _object = unsafe { *lo };
        unsafe {
            *lo = *hi;
        }
        unsafe {
            *hi = t;
        }
        let __v2: *mut *mut _object = lo;
        lo = __v2.wrapping_offset(1 as isize);
        let __v4: *mut *mut _object = hi;
        hi = __v4.wrapping_offset(-(1 as isize));
    }
}

// Lots of code for an adaptive, stable, natural mergesort.  There are many
// pieces to this algorithm; read listsort.txt for overviews and details.
// A sortslice contains a pointer to an array of keys and a pointer to
// an array of corresponding values.  In other words, keys[i]
// corresponds with values[i].  If values == NULL, then the keys are
// also the values.
//
// Several convenience routines are provided here, so that keys and
// values are always moved in sync.
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_listobject_c_1606 {
    keys: *mut *mut _object,
    values: *mut *mut _object,
}

fn sortslice_copy(
    s1: &__SlateRecord_listobject_c_1606,
    i: i64,
    s2: &__SlateRecord_listobject_c_1606,
    j: i64,
) {
    unsafe {
        *unsafe { (*(s1 as *const __SlateRecord_listobject_c_1606)).keys }
            .wrapping_offset(i as isize) =
            *unsafe { (*(s2 as *const __SlateRecord_listobject_c_1606)).keys }
                .wrapping_offset(j as isize);
    }
    if !unsafe { (*(s1 as *const __SlateRecord_listobject_c_1606)).values }.is_null() {
        unsafe {
            *unsafe { (*(s1 as *const __SlateRecord_listobject_c_1606)).values }
                .wrapping_offset(i as isize) =
                *unsafe { (*(s2 as *const __SlateRecord_listobject_c_1606)).values }
                    .wrapping_offset(j as isize);
        }
    }
}

fn sortslice_copy_incr(
    dst: &mut __SlateRecord_listobject_c_1606,
    src: &mut __SlateRecord_listobject_c_1606,
) {
    let __v1: *mut *mut _object = unsafe { (*src).keys };
    unsafe {
        (*src).keys = __v1.wrapping_offset(1 as isize);
    }
    let __v4: *mut *mut _object = unsafe { (*dst).keys };
    unsafe {
        (*dst).keys = __v4.wrapping_offset(1 as isize);
    }
    unsafe {
        *__v4 = *__v1;
    }
    if !unsafe { (*(dst as *mut __SlateRecord_listobject_c_1606)).values }.is_null() {
        let __v7: *mut *mut _object = unsafe { (*src).values };
        unsafe {
            (*src).values = __v7.wrapping_offset(1 as isize);
        }
        let __v10: *mut *mut _object = unsafe { (*dst).values };
        unsafe {
            (*dst).values = __v10.wrapping_offset(1 as isize);
        }
        unsafe {
            *__v10 = *__v7;
        }
    }
}

fn sortslice_copy_decr(
    dst: &mut __SlateRecord_listobject_c_1606,
    src: &mut __SlateRecord_listobject_c_1606,
) {
    let __v1: *mut *mut _object = unsafe { (*src).keys };
    unsafe {
        (*src).keys = __v1.wrapping_offset(-(1 as isize));
    }
    let __v4: *mut *mut _object = unsafe { (*dst).keys };
    unsafe {
        (*dst).keys = __v4.wrapping_offset(-(1 as isize));
    }
    unsafe {
        *__v4 = *__v1;
    }
    if !unsafe { (*(dst as *mut __SlateRecord_listobject_c_1606)).values }.is_null() {
        let __v7: *mut *mut _object = unsafe { (*src).values };
        unsafe {
            (*src).values = __v7.wrapping_offset(-(1 as isize));
        }
        let __v10: *mut *mut _object = unsafe { (*dst).values };
        unsafe {
            (*dst).values = __v10.wrapping_offset(-(1 as isize));
        }
        unsafe {
            *__v10 = *__v7;
        }
    }
}

fn sortslice_memcpy(
    s1: *mut __SlateRecord_listobject_c_1606,
    i: i64,
    s2: &__SlateRecord_listobject_c_1606,
    j: i64,
    n: i64,
) {
    unsafe {
        core::ptr::copy_nonoverlapping(
            ((unsafe { (*(s2 as *const __SlateRecord_listobject_c_1606)).keys }
                .wrapping_offset(j as isize) as *const ()) as *const core::ffi::c_void)
                as *const u8,
            ((unsafe { (*s1).keys }.wrapping_offset(i as isize) as *mut ())
                as *mut core::ffi::c_void) as *mut u8,
            ((core::mem::size_of::<*mut _object>() as u64).wrapping_mul(n as u64) as usize)
                as usize,
        )
    };
    if !unsafe { (*s1).values }.is_null() {
        unsafe {
            core::ptr::copy_nonoverlapping(
                ((unsafe { (*(s2 as *const __SlateRecord_listobject_c_1606)).values }
                    .wrapping_offset(j as isize) as *const ())
                    as *const core::ffi::c_void) as *const u8,
                ((unsafe { (*s1).values }.wrapping_offset(i as isize) as *mut ())
                    as *mut core::ffi::c_void) as *mut u8,
                ((core::mem::size_of::<*mut _object>() as u64).wrapping_mul(n as u64) as usize)
                    as usize,
            )
        };
    }
}

fn sortslice_memmove(
    s1: &__SlateRecord_listobject_c_1606,
    i: i64,
    s2: &__SlateRecord_listobject_c_1606,
    j: i64,
    n: i64,
) {
    unsafe {
        core::ptr::copy(
            ((unsafe { (*(s2 as *const __SlateRecord_listobject_c_1606)).keys }
                .wrapping_offset(j as isize) as *const ()) as *const core::ffi::c_void)
                as *const u8,
            ((unsafe { (*(s1 as *const __SlateRecord_listobject_c_1606)).keys }
                .wrapping_offset(i as isize) as *mut ()) as *mut core::ffi::c_void)
                as *mut u8,
            ((core::mem::size_of::<*mut _object>() as u64).wrapping_mul(n as u64) as usize)
                as usize,
        )
    };
    if !unsafe { (*(s1 as *const __SlateRecord_listobject_c_1606)).values }.is_null() {
        unsafe {
            core::ptr::copy(
                ((unsafe { (*(s2 as *const __SlateRecord_listobject_c_1606)).values }
                    .wrapping_offset(j as isize) as *const ())
                    as *const core::ffi::c_void) as *const u8,
                ((unsafe { (*(s1 as *const __SlateRecord_listobject_c_1606)).values }
                    .wrapping_offset(i as isize) as *mut ())
                    as *mut core::ffi::c_void) as *mut u8,
                ((core::mem::size_of::<*mut _object>() as u64).wrapping_mul(n as u64) as usize)
                    as usize,
            )
        };
    }
}

fn sortslice_advance(slice: &mut __SlateRecord_listobject_c_1606, n: i64) {
    let __v1: *mut *mut _object = unsafe { (*slice).keys };
    unsafe {
        (*slice).keys = __v1.wrapping_offset(n as isize);
    }
    if !unsafe { (*(slice as *mut __SlateRecord_listobject_c_1606)).values }.is_null() {
        let __v4: *mut *mut _object = unsafe { (*slice).values };
        unsafe {
            (*slice).values = __v4.wrapping_offset(n as isize);
        }
    }
}

// Comparison function: ms->key_compare, which is set at run-time in
// listsort_impl to optimize for various special cases.
// Returns -1 on error, 1 if x < y, 0 if x >= y.
// Compare X to Y via "<".  Goto "fail" if the comparison raises an
// error.  Else "k" is set to true iff X<Y, and an "if (k)" block is
// started.  It makes more sense in context <wink>.  X and Y are PyObject*s.
// The maximum number of entries in a MergeState's pending-runs stack.
// For a list with n elements, this needs at most floor(log2(n)) + 1 entries
// even if we didn't force runs to a minimal length.  So the number of bits
// in a Py_ssize_t is plenty large enough for all cases.
// When we get into galloping mode, we stay there until both runs win less
// often than MIN_GALLOP consecutive times.  See listsort.txt for more info.
// Avoid malloc for small temp arrays.
// The largest value of minrun. This must be a power of 2, and >= 1, so that
// the compute_minrun() algorithm guarantees to return a result no larger than
// this,
/// One MergeState exists on the stack per invocation of mergesort.  It's just
/// a convenient way to pass state around among the helper functions.
#[repr(C)]
#[derive(Clone, Copy)]
struct s_slice {
    base: __SlateRecord_listobject_c_1606,
    /// length of run
    len: i64,
    /// node "level" for powersort merge strategy
    power: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct s_MergeState {
    /// This controls when we get *into* galloping mode.  It's initialized
    /// to MIN_GALLOP.  merge_lo and merge_hi tend to nudge it higher for
    /// random data, and lower for highly structured data.
    min_gallop: i64,
    /// len(input_list) - read only
    listlen: i64,
    /// base address of keys array - read only
    basekeys: *mut *mut _object,
    /// 'a' is temp storage to help with merges.  It contains room for
    /// alloced entries.
    /// may point to temparray below
    a: __SlateRecord_listobject_c_1606,
    alloced: i64,
    /// A stack of n pending runs yet to be merged.  Run #i starts at
    /// address base[i] and extends for len[i] elements.  It's always
    /// true (so long as the indices are in bounds) that
    ///
    ///     pending[i].base + pending[i].len == pending[i+1].base
    ///
    /// so we could cut the storage for this, but it's a minor amount,
    /// and keeping all the info explicit simplifies the code.
    n: i32,
    pending: [s_slice; 64],
    /// 'a' points to this when possible, rather than muck with malloc.
    temparray: [*mut _object; 256],
    /// This is the function we will use to compare two keys,
    /// even when none of our special cases apply and we have to use
    /// safe_object_compare.
    key_compare:
        Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object, *mut s_MergeState) -> i32>,
    /// This function is used by unsafe_object_compare to optimize comparisons
    /// when we know our list is type-homogeneous but we can't assume anything else.
    /// In the pre-sort check it is set equal to Py_TYPE(key)->tp_richcompare
    key_richcompare:
        Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object, i32) -> *mut _object>,
    /// This function is used by unsafe_tuple_compare to compare the first elements
    /// of tuples. It may be set to safe_object_compare, but the idea is that hopefully
    /// we can assume more, and use one of the special-case compares.
    tuple_elem_compare:
        Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object, *mut s_MergeState) -> i32>,
}

/// binarysort is the best method for sorting small arrays: it does few
/// compares, but can do data movement quadratic in the number of elements.
/// ss->keys is viewed as an array of n kays, a[:n]. a[:ok] is already sorted.
/// Pass ok = 0 (or 1) if you don't know.
/// It's sorted in-place, by a stable binary insertion sort. If ss->values
/// isn't NULL, it's permuted in lockstap with ss->keys.
/// On entry, must have n >= 1, and 0 <= ok <= n <= MAX_MINRUN.
/// Return -1 if comparison raises an exception, else 0.
/// Even in case of error, the output slice will be some permutation of
/// the input (nothing is lost or duplicated).
fn binarysort(
    ms: *mut s_MergeState,
    ss: &__SlateRecord_listobject_c_1606,
    n: i64,
    mut ok: i64,
) -> i32 {
    let mut R: i64 = unsafe { core::mem::zeroed() };
    // Regular insertion sort has average- and worst-case O(n**2) cost
    // for both # of comparisons and number of bytes moved. But its branches
    // are highly predictable, and it loves sorted input (n-1 compares and no
    // data movement). This is significant in cases like sortperf.py's %sort,
    // where an out-of-order element near the start of a run is moved into
    // place slowly but then the remaining elements up to length minrun are
    // generally at worst one slot away from their correct position (so only
    // need 1 or 2 commpares to resolve). If comparisons are very fast (such
    // as for a list of Python floats), the simple inner loop leaves it
    // very competitive with binary insertion, despite that it does
    // significantly more compares overall on random data.
    //
    // Binary insertion sort has worst, average, and best case O(n log n)
    // cost for # of comparisons, but worst and average case O(n**2) cost
    // for data movement. The more expensive comparisons, the more important
    // the comparison advantage. But its branches are less predictable the
    // more "randomish" the data, and that's so significant its worst case
    // in real life is random input rather than reverse-ordered (which does
    // about twice the data movement than random input does).
    //
    // Note that the number of bytes moved doesn't seem to matter. MAX_MINRUN
    // of 64 is so small that the key and value pointers all fit in a corner
    // of L1 cache, and moving things around in that is very fast.
    let mut L: i64 = unsafe { core::mem::zeroed() };
    let mut M: i64 = unsafe { core::mem::zeroed() };
    let mut pivot: *mut _object = unsafe { core::mem::zeroed() };
    let mut has_values: bool = unsafe { core::mem::zeroed() };
    let mut v: *mut *mut _object = unsafe { core::mem::zeroed() };
    let mut a: *mut *mut _object = unsafe { core::mem::zeroed() };
    // for IFLT macro expansion
    let mut k: i64 = unsafe { core::mem::zeroed() };
    unsafe {
        a = (*(ss as *const __SlateRecord_listobject_c_1606)).keys;
        v = (*(ss as *const __SlateRecord_listobject_c_1606)).values;
        has_values = !v.is_null();
        // assert a[:ok] is sorted
        if ok == 0 {
            ok = ok.wrapping_add(1);
        }
        while ok < n {
            // set L to where a[ok] belongs
            L = 0;
            R = ok;
            pivot = *a.wrapping_offset(ok as isize);
            // Slice invariants. vacuously true at the start:
            // all a[0:L]  <= pivot
            // all a[L:R]     unknown
            // all a[R:ok]  > pivot
            loop {
                // don't do silly ;-) things to prevent overflow when finding
                // the midpoint; L and R are very far from filling a Py_ssize_t
                M = L.wrapping_add(R) >> 1i32;
                let __v6: i64 = ISLT!(pivot, unsafe { *a.wrapping_offset(M as isize) }, ms) as i64;
                k = __v6;
                if __v6 < 0 {
                    return 1i32.wrapping_neg();
                } else {
                    if k != 0 {
                        R = M;
                    } else {
                        L = M.wrapping_add(1);
                    }
                    if L >= R {
                        break;
                    }
                }
            }
            // a[:L] holds all elements from a[:ok] <= pivot now, so pivot belongs
            // at index L. Slide a[L:ok] to the right a slot to make room for it.
            // Caution: using memmove is much slower under MSVC 5; we're not
            // usually moving many slots. Years later: under Visual Studio 2022,
            // memmove seems just slightly slower than doing it "by hand".
            M = ok;
            while M > L {
                *a.wrapping_offset(M as isize) = *a.wrapping_offset(M.wrapping_sub(1) as isize);
                M = M.wrapping_sub(1);
            }
            *a.wrapping_offset(L as isize) = pivot;
            if has_values {
                pivot = *v.wrapping_offset(ok as isize);
                M = ok;
                while M > L {
                    *v.wrapping_offset(M as isize) = *v.wrapping_offset(M.wrapping_sub(1) as isize);
                    M = M.wrapping_sub(1);
                }
                *v.wrapping_offset(L as isize) = pivot;
            }
            ok = ok.wrapping_add(1);
        }
        return 0;
    }
}

fn sortslice_reverse(s: &__SlateRecord_listobject_c_1606, n: i64) {
    reverse_slice(
        unsafe { (*(s as *const __SlateRecord_listobject_c_1606)).keys },
        unsafe { (*(s as *const __SlateRecord_listobject_c_1606)).keys }
            .wrapping_offset(n as isize),
    );
    if !unsafe { (*(s as *const __SlateRecord_listobject_c_1606)).values }.is_null() {
        reverse_slice(
            unsafe { (*(s as *const __SlateRecord_listobject_c_1606)).values },
            unsafe { (*(s as *const __SlateRecord_listobject_c_1606)).values }
                .wrapping_offset(n as isize),
        );
    }
}

/// Return the length of the run beginning at slo->keys, spanning no more than
/// nremaining elements. The run beginning there may be ascending or descending,
/// but the function permutes it in place, if needed, so that it's always ascending
/// upon return.
///
/// Returns -1 in case of error.
fn count_run(ms: *mut s_MergeState, slo: &__SlateRecord_listobject_c_1606, nremaining: i64) -> i64 {
    let mut slice: core::mem::MaybeUninit<__SlateRecord_listobject_c_1606> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_5: *mut __SlateRecord_listobject_c_1606 =
        core::ptr::addr_of_mut!(slice) as *mut __SlateRecord_listobject_c_1606;
    let mut slice: core::mem::MaybeUninit<__SlateRecord_listobject_c_1606> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_10: *mut __SlateRecord_listobject_c_1606 =
        core::ptr::addr_of_mut!(slice) as *mut __SlateRecord_listobject_c_1606;
    // Finish descending run. All-squal subruns are reversed in-place on the
    // fly. Their original order will be restored at the end by the whole-slice
    // reversal.
    let mut neq: i64 = unsafe { core::mem::zeroed() };
    let mut lo: *mut *mut _object = unsafe { core::mem::zeroed() };
    let mut n: i64 = unsafe { core::mem::zeroed() };
    // used by IFLT macro expansion
    let mut k: i64 = unsafe { core::mem::zeroed() };
    unsafe {
        lo = (*(slo as *const __SlateRecord_listobject_c_1606)).keys;
        // In general, as things go on we've established that the slice starts
        // with a monotone run of n elements, starting at lo.
        // We're n elements into the slice, and the most recent neq+1 elements are
        // all equal. This reverses them in-place, and resets neq for reuse.
        // Sticking to only __lt__ compares is confusing and error-prone. But in
        // this routine, almost all uses of IFLT can be captured by tiny macros
        // giving mnemonic names to the intent. Note that inline functions don't
        // work for this (IFLT expands to code including `goto fail`).
        // try ascending run first
        n = 1;
        '__join_0: {
            while n < nremaining {
                let __v21: i64 = ISLT!(
                    unsafe { *lo.wrapping_offset(n as isize) },
                    unsafe { *lo.wrapping_offset(n.wrapping_sub(1) as isize) },
                    ms
                ) as i64;
                k = __v21;
                if __v21 < 0 {
                    break '__join_0;
                } else {
                    if k != 0 {
                        break;
                    } else {
                        n = n.wrapping_add(1);
                    }
                }
            }
            if n == nremaining {
                return n;
            } else {
                // lo[n] is strictly less
                //
                // If n is 1 now, then the first compare established it's a descending
                // run, so fall through to the descending case. But if n > 1, there are
                // n elements in an ascending run terminated by the strictly less lo[n].
                // If the first key < lo[n-1], *somewhere* along the way the sequence
                // increased, so we're done (there is no descending run).
                // Else first key >= lo[n-1], which implies that the entire ascending run
                // consists of equal elements. In that case, this is a descending run,
                // and we reverse the all-equal prefix in-place.
                if n > 1 {
                    let __v18: i64 = ISLT!(
                        unsafe { *lo.wrapping_offset(0 as isize) },
                        unsafe { *lo.wrapping_offset(n.wrapping_sub(1) as isize) },
                        ms
                    ) as i64;
                    k = __v18;
                    if __v18 < 0 {
                        break '__join_0;
                    } else {
                        if k != 0 {
                            return n;
                        } else {
                            sortslice_reverse(slo, n);
                        }
                    }
                }
                n = n.wrapping_add(1); // in all cases it's been established that lo[n] has been resolved
                neq = 0;
                while n < nremaining {
                    let __v14: i64 = ISLT!(
                        unsafe { *lo.wrapping_offset(n as isize) },
                        unsafe { *lo.wrapping_offset(n.wrapping_sub(1) as isize) },
                        ms
                    ) as i64;
                    k = __v14;
                    if __v14 < 0 {
                        break '__join_0;
                    } else {
                        if k != 0 {
                            // This ends the most recent run of equal elements, but still in
                            // the "descending" direction.
                            if neq != 0 {
                                core::ptr::write(
                                    __slate_slot_10,
                                    *(slo as *const __SlateRecord_listobject_c_1606),
                                );
                                neq = neq.wrapping_add(1);
                                sortslice_advance(
                                    &mut (*core::ptr::addr_of_mut!(*__slate_slot_10)),
                                    n.wrapping_sub(neq),
                                );
                                sortslice_reverse(
                                    &(*core::ptr::addr_of_mut!(*__slate_slot_10)),
                                    neq,
                                );
                                neq = 0;
                            }
                        } else {
                            let __v13: i64 = ISLT!(
                                unsafe { *lo.wrapping_offset(n.wrapping_sub(1) as isize) },
                                unsafe { *lo.wrapping_offset(n as isize) },
                                ms
                            ) as i64;
                            k = __v13;
                            if __v13 < 0 {
                                break '__join_0;
                            } else {
                                if k != 0 {
                                    break;
                                } else {
                                    neq = neq.wrapping_add(1); // descending run is over
                                    // not x < y and not y < x implies x == y
                                }
                            }
                        }
                        n = n.wrapping_add(1);
                    }
                }
                if neq != 0 {
                    core::ptr::write(
                        __slate_slot_5,
                        *(slo as *const __SlateRecord_listobject_c_1606),
                    );
                    neq = neq.wrapping_add(1);
                    sortslice_advance(
                        &mut (*core::ptr::addr_of_mut!(*__slate_slot_5)),
                        n.wrapping_sub(neq),
                    );
                    sortslice_reverse(&(*core::ptr::addr_of_mut!(*__slate_slot_5)), neq);
                    neq = 0;
                }
                sortslice_reverse(slo, n); // transform to ascending run
                // And after reversing, it's possible this can be extended by a
                // naturally increasing suffix; e.g., [3, 2, 3, 4, 1] makes an
                // ascending run from the first 4 elements.
                while n < nremaining {
                    let __v2: i64 = ISLT!(
                        unsafe { *lo.wrapping_offset(n as isize) },
                        unsafe { *lo.wrapping_offset(n.wrapping_sub(1) as isize) },
                        ms
                    ) as i64;
                    k = __v2;
                    if __v2 < 0 {
                        break '__join_0;
                    } else {
                        if k != 0 {
                            break;
                        } else {
                            n = n.wrapping_add(1);
                        }
                    }
                }
                return n;
            }
        }
        return 1i32.wrapping_neg() as i64;
    }
}

/// Locate the proper position of key in a sorted vector; if the vector contains
/// an element equal to key, return the position immediately to the left of
/// the leftmost equal element.  [gallop_right() does the same except returns
/// the position to the right of the rightmost equal element (if any).]
///
/// "a" is a sorted vector with n elements, starting at a[0].  n must be > 0.
///
/// "hint" is an index at which to begin the search, 0 <= hint < n.  The closer
/// hint is to the final result, the faster this runs.
///
/// The return value is the int k in 0..n such that
///
///     a[k-1] < key <= a[k]
///
/// pretending that *(a-1) is minus infinity and a[n] is plus infinity.  IOW,
/// key belongs at index k; or, IOW, the first k elements of a should precede
/// key, and the last n-k should follow key.
///
/// Returns -1 on error.  See listsort.txt for info on the method.
fn gallop_left(
    ms: *mut s_MergeState,
    key: *mut _object,
    mut a: *mut *mut _object,
    n: i64,
    hint: i64,
) -> i64 {
    let mut m: i64 = unsafe { core::mem::zeroed() };
    // a[hint] < key -- gallop right, until
    // a[hint + lastofs] < key <= a[hint + ofs]
    // &a[n-1] is highest
    let mut maxofs: i64 = unsafe { core::mem::zeroed() };
    // key <= a[hint] -- gallop left, until
    // a[hint - ofs] < key <= a[hint - lastofs]
    // &a[0] is lowest
    let mut maxofs_1: i64 = unsafe { core::mem::zeroed() };
    let mut k: i64 = unsafe { core::mem::zeroed() };
    let mut lastofs: i64 = unsafe { core::mem::zeroed() };
    let mut ofs: i64 = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_0: {
            let __v16: *mut *mut _object = a;
            a = __v16.wrapping_offset(hint as isize);
            lastofs = 0;
            ofs = 1;
            let __v14: i64 = ISLT!(unsafe { *a }, key, ms) as i64;
            k = __v14;
            if __v14 >= 0 {
                if k != 0 {
                    maxofs = n.wrapping_sub(hint);
                    while ofs < maxofs {
                        let __v10: i64 =
                            ISLT!(unsafe { *a.wrapping_offset(ofs as isize) }, key, ms) as i64;
                        k = __v10;
                        if __v10 < 0 {
                            break '__join_0;
                        } else {
                            if k != 0 {
                                lastofs = ofs;
                                ofs = (ofs << 1i32).wrapping_add(1); // key <= a[hint + ofs]
                            } else {
                                break;
                            }
                        }
                    }
                    if ofs > maxofs {
                        ofs = maxofs;
                    }
                    // Translate back to offsets relative to &a[0].
                    lastofs = lastofs.wrapping_add(hint);
                    ofs = ofs.wrapping_add(hint);
                } else {
                    maxofs_1 = hint.wrapping_add(1);
                    while ofs < maxofs_1 {
                        let __v12: i64 =
                            ISLT!(unsafe { *a.wrapping_offset(-(ofs as isize)) }, key, ms) as i64;
                        k = __v12;
                        if __v12 < 0 {
                            break '__join_0;
                        } else {
                            if k != 0 {
                                break;
                            } else {
                                // key <= a[hint - ofs]
                                lastofs = ofs;
                                ofs = (ofs << 1i32).wrapping_add(1);
                            }
                        }
                    }
                    if ofs > maxofs_1 {
                        ofs = maxofs_1;
                    }
                    // Translate back to positive offsets relative to &a[0].
                    k = lastofs;
                    lastofs = hint.wrapping_sub(ofs);
                    ofs = hint.wrapping_sub(k);
                }
                let __v5: *mut *mut _object = a;
                a = __v5.wrapping_offset(-(hint as isize));
                // Now a[lastofs] < key <= a[ofs], so key belongs somewhere to the
                // right of lastofs but no farther right than ofs.  Do a binary
                // search, with invariant a[lastofs-1] < key <= a[ofs].
                lastofs = lastofs.wrapping_add(1);
                while lastofs < ofs {
                    m = lastofs.wrapping_add(ofs.wrapping_sub(lastofs) >> 1i32);
                    let __v0: i64 =
                        ISLT!(unsafe { *a.wrapping_offset(m as isize) }, key, ms) as i64;
                    k = __v0;
                    if __v0 < 0 {
                        break '__join_0;
                    } else {
                        if k != 0 {
                            lastofs = m.wrapping_add(1);
                        } else {
                            ofs = m;
                        }
                        // a[m] < key
                        // key <= a[m]
                    }
                }
                // so a[ofs-1] < key <= a[ofs]
                return ofs;
            }
        }
        return 1i32.wrapping_neg() as i64;
    }
}

/// Exactly like gallop_left(), except that if key already exists in a[0:n],
/// finds the position immediately to the right of the rightmost equal value.
///
/// The return value is the int k in 0..n such that
///
///     a[k-1] <= key < a[k]
///
/// or -1 if error.
///
/// The code duplication is massive, but this is enough different given that
/// we're sticking to "<" comparisons that it's much harder to follow if
/// written as one routine with yet another "left or right?" flag.
fn gallop_right(
    ms: *mut s_MergeState,
    key: *mut _object,
    mut a: *mut *mut _object,
    n: i64,
    hint: i64,
) -> i64 {
    let mut m: i64 = unsafe { core::mem::zeroed() };
    // key < a[hint] -- gallop left, until
    // a[hint - ofs] <= key < a[hint - lastofs]
    // &a[0] is lowest
    let mut maxofs: i64 = unsafe { core::mem::zeroed() };
    // a[hint] <= key -- gallop right, until
    // a[hint + lastofs] <= key < a[hint + ofs]
    // &a[n-1] is highest
    let mut maxofs_1: i64 = unsafe { core::mem::zeroed() };
    let mut k: i64 = unsafe { core::mem::zeroed() };
    let mut lastofs: i64 = unsafe { core::mem::zeroed() };
    let mut ofs: i64 = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_0: {
            let __v16: *mut *mut _object = a;
            a = __v16.wrapping_offset(hint as isize);
            lastofs = 0;
            ofs = 1;
            let __v14: i64 = ISLT!(key, unsafe { *a }, ms) as i64;
            k = __v14;
            if __v14 >= 0 {
                if k != 0 {
                    maxofs = hint.wrapping_add(1);
                    while ofs < maxofs {
                        let __v6: i64 =
                            ISLT!(key, unsafe { *a.wrapping_offset(-(ofs as isize)) }, ms) as i64;
                        k = __v6;
                        if __v6 < 0 {
                            break '__join_0;
                        } else {
                            if k != 0 {
                                lastofs = ofs;
                                ofs = (ofs << 1i32).wrapping_add(1); // a[hint - ofs] <= key
                            } else {
                                break;
                            }
                        }
                    }
                    if ofs > maxofs {
                        ofs = maxofs;
                    }
                    // Translate back to positive offsets relative to &a[0].
                    k = lastofs;
                    lastofs = hint.wrapping_sub(ofs);
                    ofs = hint.wrapping_sub(k);
                } else {
                    maxofs_1 = n.wrapping_sub(hint);
                    while ofs < maxofs_1 {
                        let __v12: i64 =
                            ISLT!(key, unsafe { *a.wrapping_offset(ofs as isize) }, ms) as i64;
                        k = __v12;
                        if __v12 < 0 {
                            break '__join_0;
                        } else {
                            if k != 0 {
                                break;
                            } else {
                                // a[hint + ofs] <= key
                                lastofs = ofs;
                                ofs = (ofs << 1i32).wrapping_add(1);
                            }
                        }
                    }
                    if ofs > maxofs_1 {
                        ofs = maxofs_1;
                    }
                    // Translate back to offsets relative to &a[0].
                    lastofs = lastofs.wrapping_add(hint);
                    ofs = ofs.wrapping_add(hint);
                }
                let __v5: *mut *mut _object = a;
                a = __v5.wrapping_offset(-(hint as isize));
                // Now a[lastofs] <= key < a[ofs], so key belongs somewhere to the
                // right of lastofs but no farther right than ofs.  Do a binary
                // search, with invariant a[lastofs-1] <= key < a[ofs].
                lastofs = lastofs.wrapping_add(1);
                while lastofs < ofs {
                    m = lastofs.wrapping_add(ofs.wrapping_sub(lastofs) >> 1i32);
                    let __v0: i64 =
                        ISLT!(key, unsafe { *a.wrapping_offset(m as isize) }, ms) as i64;
                    k = __v0;
                    if __v0 < 0 {
                        break '__join_0;
                    } else {
                        if k != 0 {
                            ofs = m;
                        } else {
                            lastofs = m.wrapping_add(1);
                        }
                        // key < a[m]
                        // a[m] <= key
                    }
                }
                // so a[ofs-1] <= key < a[ofs]
                return ofs;
            }
        }
        return 1i32.wrapping_neg() as i64;
    }
}

/// Conceptually a MergeState's constructor.
fn merge_init(
    ms: &mut s_MergeState,
    list_size: i64,
    has_keyfunc: i32,
    lo: &__SlateRecord_listobject_c_1606,
) {
    if has_keyfunc != 0 {
        // The temporary space for merging will need at most half the list
        // size rounded up.  Use the minimum possible space so we can use the
        // rest of temparray for other things.  In particular, if there is
        // enough extra space, listsort() will use it to store the keys.
        unsafe {
            (*(ms as *mut s_MergeState)).alloced = list_size.wrapping_add(1) / 2;
        }
        // ms->alloced describes how many keys will be stored at
        // ms->temparray, but we also need to store the values.  Hence,
        // ms->alloced is capped at half of MERGESTATE_TEMP_SIZE.
        if 128i64 < unsafe { (*(ms as *mut s_MergeState)).alloced } {
            unsafe {
                (*(ms as *mut s_MergeState)).alloced = 128i64;
            }
        }
        unsafe {
            (*(ms as *mut s_MergeState)).a.values =
                ((*(ms as *mut s_MergeState)).temparray.as_mut_ptr() as *mut *mut _object)
                    .wrapping_offset((*(ms as *mut s_MergeState)).alloced as isize);
        }
    } else {
        unsafe {
            (*(ms as *mut s_MergeState)).alloced = 256;
        }
        unsafe {
            (*(ms as *mut s_MergeState)).a.values = core::ptr::null_mut::<*mut _object>();
        }
    }
    unsafe {
        (*(ms as *mut s_MergeState)).a.keys =
            (*(ms as *mut s_MergeState)).temparray.as_mut_ptr() as *mut *mut _object;
    }
    unsafe {
        (*(ms as *mut s_MergeState)).n = 0;
    }
    unsafe {
        (*(ms as *mut s_MergeState)).min_gallop = 7;
    }
    unsafe {
        (*(ms as *mut s_MergeState)).listlen = list_size;
    }
    unsafe {
        (*(ms as *mut s_MergeState)).basekeys =
            (*(lo as *const __SlateRecord_listobject_c_1606)).keys;
    }
}

/// Free all the temp memory owned by the MergeState.  This must be called
/// when you're done with a MergeState, and may be called before then if
/// you want to free the temp memory early.
fn merge_freemem(ms: *mut s_MergeState) {
    if (unsafe { (*ms).a.keys }) != unsafe { (*ms).temparray.as_mut_ptr() as *mut *mut _object } {
        unsafe { PyMem_Free((*ms).a.keys as *mut ()) };
        unsafe {
            (*ms).a.keys = core::ptr::null_mut::<*mut _object>();
        }
    }
}

/// Ensure enough temp memory for 'need' array slots is available.
/// Returns 0 on success and -1 if the memory can't be gotten.
fn merge_getmem(ms: *mut s_MergeState, need: i64) -> i32 {
    let mut multiplier: i32 = 0;
    if need <= unsafe { (*ms).alloced } {
        return 0;
    }
    multiplier = if !unsafe { (*ms).a.values }.is_null() {
        2
    } else {
        1
    };
    // Don't realloc!  That can cost cycles to copy the old data, but
    // we don't care what's in the block.
    merge_freemem(ms);
    if (need as u64)
        > 9223372036854775807 / (core::mem::size_of::<*mut _object>() as u64) / (multiplier as u64)
    {
        unsafe { PyErr_NoMemory() };
        return 1i32.wrapping_neg();
    }
    unsafe {
        (*ms).a.keys = PyMem_Malloc(
            ((multiplier as i64).wrapping_mul(need) as u64)
                .wrapping_mul(core::mem::size_of::<*mut _object>() as u64),
        ) as *mut *mut _object;
    }
    if !unsafe { (*ms).a.keys }.is_null() {
        unsafe {
            (*ms).alloced = need;
        }
        if !unsafe { (*ms).a.values }.is_null() {
            unsafe {
                (*ms).a.values = unsafe { (*ms).a.keys }.wrapping_offset(need as isize);
            }
        }
        return 0;
    }
    unsafe { PyErr_NoMemory() };
    1i32.wrapping_neg()
}

/// Merge the na elements starting at ssa with the nb elements starting at
/// ssb.keys = ssa.keys + na in a stable way, in-place.  na and nb must be > 0.
/// Must also have that ssa.keys[na-1] belongs at the end of the merge, and
/// should have na <= nb.  See listsort.txt for more info.  Return 0 if
/// successful, -1 if error.
fn merge_lo(
    ms: *mut s_MergeState,
    mut ssa: __SlateRecord_listobject_c_1606,
    mut na: i64,
    mut ssb: __SlateRecord_listobject_c_1606,
    mut nb: i64,
) -> i64 {
    // # of times B won in a row
    let mut bcount: i64 = unsafe { core::mem::zeroed() };
    // # of times A won in a row
    let mut acount: i64 = unsafe { core::mem::zeroed() };
    let mut min_gallop: i64 = unsafe { core::mem::zeroed() };
    // guilty until proved innocent
    let mut result: i32 = unsafe { core::mem::zeroed() };
    let mut dest: core::mem::MaybeUninit<__SlateRecord_listobject_c_1606> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_28: *mut __SlateRecord_listobject_c_1606 =
        core::ptr::addr_of_mut!(dest) as *mut __SlateRecord_listobject_c_1606;
    let mut k: i64 = unsafe { core::mem::zeroed() };
    unsafe {
        result = 1i32.wrapping_neg();
        if MERGE_GETMEM!(ms, na) < 0 {
            return 1i32.wrapping_neg() as i64;
        } else {
            '__join_3: {
                '__join_4: {
                    sortslice_memcpy(
                        core::ptr::addr_of_mut!((*ms).a),
                        0i64,
                        &(*core::ptr::addr_of_mut!(ssa)),
                        0i64,
                        na,
                    );
                    *__slate_slot_28 = ssa;
                    ssa = (*ms).a;
                    sortslice_copy_incr(
                        &mut (*core::ptr::addr_of_mut!(*__slate_slot_28)),
                        &mut (*core::ptr::addr_of_mut!(ssb)),
                    );
                    nb = nb.wrapping_sub(1);
                    if nb != 0 {
                        '__join_0: {
                            if na != 1 {
                                min_gallop = (*ms).min_gallop;
                                loop {
                                    acount = 0;
                                    bcount = 0;
                                    // Do the straightforward thing until (if ever) one run
                                    // appears to win consistently.
                                    loop {
                                        k = ISLT!(
                                            unsafe { *ssb.keys.wrapping_offset(0 as isize) },
                                            unsafe { *ssa.keys.wrapping_offset(0 as isize) },
                                            ms
                                        ) as i64;
                                        if k != 0 {
                                            if k < 0 {
                                                break '__join_3;
                                            } else {
                                                sortslice_copy_incr(
                                                    &mut (*core::ptr::addr_of_mut!(
                                                        *__slate_slot_28
                                                    )),
                                                    &mut (*core::ptr::addr_of_mut!(ssb)),
                                                );
                                                bcount = bcount.wrapping_add(1);
                                                acount = 0;
                                                nb = nb.wrapping_sub(1);
                                                if nb == 0 {
                                                    break '__join_4;
                                                } else {
                                                    if bcount >= min_gallop {
                                                        break;
                                                    }
                                                }
                                            }
                                        } else {
                                            sortslice_copy_incr(
                                                &mut (*core::ptr::addr_of_mut!(*__slate_slot_28)),
                                                &mut (*core::ptr::addr_of_mut!(ssa)),
                                            );
                                            acount = acount.wrapping_add(1);
                                            bcount = 0;
                                            na = na.wrapping_sub(1);
                                            if na == 1 {
                                                break '__join_0;
                                            } else {
                                                if acount >= min_gallop {
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                    // One run is winning so consistently that galloping may
                                    // be a huge win.  So try that, and continue galloping until
                                    // (if ever) neither run appears to be winning consistently
                                    // anymore.
                                    min_gallop = min_gallop.wrapping_add(1);
                                    loop {
                                        min_gallop = min_gallop
                                            .wrapping_sub(((min_gallop > 1) as i32) as i64);
                                        (*ms).min_gallop = min_gallop;
                                        k = gallop_right(
                                            ms,
                                            *ssb.keys.wrapping_offset(0 as isize),
                                            ssa.keys,
                                            na,
                                            0i64,
                                        );
                                        acount = k;
                                        if k != 0 {
                                            if k < 0 {
                                                break '__join_3;
                                            } else {
                                                sortslice_memcpy(
                                                    core::ptr::addr_of_mut!(*__slate_slot_28),
                                                    0i64,
                                                    &(*core::ptr::addr_of_mut!(ssa)),
                                                    0i64,
                                                    k,
                                                );
                                                sortslice_advance(
                                                    &mut (*core::ptr::addr_of_mut!(
                                                        *__slate_slot_28
                                                    )),
                                                    k,
                                                );
                                                sortslice_advance(
                                                    &mut (*core::ptr::addr_of_mut!(ssa)),
                                                    k,
                                                );
                                                na = na.wrapping_sub(k);
                                                if na == 1 {
                                                    break '__join_0;
                                                } else {
                                                    // na==0 is impossible now if the comparison
                                                    // function is consistent, but we can't assume
                                                    // that it is.
                                                    if na == 0 {
                                                        break '__join_4;
                                                    }
                                                }
                                            }
                                        }
                                        sortslice_copy_incr(
                                            &mut (*core::ptr::addr_of_mut!(*__slate_slot_28)),
                                            &mut (*core::ptr::addr_of_mut!(ssb)),
                                        );
                                        nb = nb.wrapping_sub(1);
                                        if nb == 0 {
                                            break '__join_4;
                                        } else {
                                            k = gallop_left(
                                                ms,
                                                *ssa.keys.wrapping_offset(0 as isize),
                                                ssb.keys,
                                                nb,
                                                0i64,
                                            );
                                            bcount = k;
                                            if k != 0 {
                                                if k < 0 {
                                                    break '__join_3;
                                                } else {
                                                    sortslice_memmove(
                                                        &(*core::ptr::addr_of_mut!(
                                                            *__slate_slot_28
                                                        )),
                                                        0i64,
                                                        &(*core::ptr::addr_of_mut!(ssb)),
                                                        0i64,
                                                        k,
                                                    );
                                                    sortslice_advance(
                                                        &mut (*core::ptr::addr_of_mut!(
                                                            *__slate_slot_28
                                                        )),
                                                        k,
                                                    );
                                                    sortslice_advance(
                                                        &mut (*core::ptr::addr_of_mut!(ssb)),
                                                        k,
                                                    );
                                                    nb = nb.wrapping_sub(k);
                                                    if nb == 0 {
                                                        break '__join_4;
                                                    }
                                                }
                                            }
                                            sortslice_copy_incr(
                                                &mut (*core::ptr::addr_of_mut!(*__slate_slot_28)),
                                                &mut (*core::ptr::addr_of_mut!(ssa)),
                                            );
                                            na = na.wrapping_sub(1);
                                            if na == 1 {
                                                break '__join_0;
                                            } else {
                                                if !(acount >= 7 || bcount >= 7) {
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                    min_gallop = min_gallop.wrapping_add(1); // penalize it for leaving galloping mode
                                    (*ms).min_gallop = min_gallop;
                                }
                            }
                        }
                        // The last element of ssa belongs at the end of the merge.
                        sortslice_memmove(
                            &(*core::ptr::addr_of_mut!(*__slate_slot_28)),
                            0i64,
                            &(*core::ptr::addr_of_mut!(ssb)),
                            0i64,
                            nb,
                        );
                        sortslice_copy(
                            &(*core::ptr::addr_of_mut!(*__slate_slot_28)),
                            nb,
                            &(*core::ptr::addr_of_mut!(ssa)),
                            0i64,
                        );
                        return 0;
                    }
                }
                result = 0;
            }
            if na != 0 {
                sortslice_memcpy(
                    core::ptr::addr_of_mut!(*__slate_slot_28),
                    0i64,
                    &(*core::ptr::addr_of_mut!(ssa)),
                    0i64,
                    na,
                );
            }
            return result as i64;
        }
    }
}

/// Merge the na elements starting at pa with the nb elements starting at
/// ssb.keys = ssa.keys + na in a stable way, in-place.  na and nb must be > 0.
/// Must also have that ssa.keys[na-1] belongs at the end of the merge, and
/// should have na >= nb.  See listsort.txt for more info.  Return 0 if
/// successful, -1 if error.
fn merge_hi(
    ms: *mut s_MergeState,
    mut ssa: __SlateRecord_listobject_c_1606,
    mut na: i64,
    mut ssb: __SlateRecord_listobject_c_1606,
    mut nb: i64,
) -> i64 {
    // # of times B won in a row
    let mut bcount: i64 = unsafe { core::mem::zeroed() };
    // # of times A won in a row
    let mut acount: i64 = unsafe { core::mem::zeroed() };
    let mut min_gallop: i64 = unsafe { core::mem::zeroed() };
    // guilty until proved innocent
    let mut result: i32 = unsafe { core::mem::zeroed() };
    let mut baseb: core::mem::MaybeUninit<__SlateRecord_listobject_c_1606> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_28: *mut __SlateRecord_listobject_c_1606 =
        core::ptr::addr_of_mut!(baseb) as *mut __SlateRecord_listobject_c_1606;
    let mut basea: core::mem::MaybeUninit<__SlateRecord_listobject_c_1606> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_29: *mut __SlateRecord_listobject_c_1606 =
        core::ptr::addr_of_mut!(basea) as *mut __SlateRecord_listobject_c_1606;
    let mut dest: core::mem::MaybeUninit<__SlateRecord_listobject_c_1606> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_30: *mut __SlateRecord_listobject_c_1606 =
        core::ptr::addr_of_mut!(dest) as *mut __SlateRecord_listobject_c_1606;
    let mut k: i64 = unsafe { core::mem::zeroed() };
    unsafe {
        result = 1i32.wrapping_neg();
        if MERGE_GETMEM!(ms, nb) < 0 {
            return 1i32.wrapping_neg() as i64;
        } else {
            *__slate_slot_30 = ssb;
            sortslice_advance(
                &mut (*core::ptr::addr_of_mut!(*__slate_slot_30)),
                nb.wrapping_sub(1),
            );
            sortslice_memcpy(
                core::ptr::addr_of_mut!((*ms).a),
                0i64,
                &(*core::ptr::addr_of_mut!(ssb)),
                0i64,
                nb,
            );
            *__slate_slot_29 = ssa;
            *__slate_slot_28 = (*ms).a;
            ssb.keys = unsafe { (*ms).a.keys }
                .wrapping_offset(nb as isize)
                .wrapping_offset(-(1 as isize));
            if !ssb.values.is_null() {
                ssb.values = unsafe { (*ms).a.values }
                    .wrapping_offset(nb as isize)
                    .wrapping_offset(-(1 as isize));
            }
            '__join_3: {
                '__join_4: {
                    sortslice_advance(&mut (*core::ptr::addr_of_mut!(ssa)), na.wrapping_sub(1));
                    sortslice_copy_decr(
                        &mut (*core::ptr::addr_of_mut!(*__slate_slot_30)),
                        &mut (*core::ptr::addr_of_mut!(ssa)),
                    );
                    na = na.wrapping_sub(1);
                    if na != 0 {
                        '__join_0: {
                            if nb != 1 {
                                min_gallop = (*ms).min_gallop;
                                loop {
                                    acount = 0;
                                    bcount = 0;
                                    // Do the straightforward thing until (if ever) one run
                                    // appears to win consistently.
                                    loop {
                                        k = ISLT!(
                                            unsafe { *ssb.keys.wrapping_offset(0 as isize) },
                                            unsafe { *ssa.keys.wrapping_offset(0 as isize) },
                                            ms
                                        ) as i64;
                                        if k != 0 {
                                            if k < 0 {
                                                break '__join_3;
                                            } else {
                                                sortslice_copy_decr(
                                                    &mut (*core::ptr::addr_of_mut!(
                                                        *__slate_slot_30
                                                    )),
                                                    &mut (*core::ptr::addr_of_mut!(ssa)),
                                                );
                                                acount = acount.wrapping_add(1);
                                                bcount = 0;
                                                na = na.wrapping_sub(1);
                                                if na == 0 {
                                                    break '__join_4;
                                                } else {
                                                    if acount >= min_gallop {
                                                        break;
                                                    }
                                                }
                                            }
                                        } else {
                                            sortslice_copy_decr(
                                                &mut (*core::ptr::addr_of_mut!(*__slate_slot_30)),
                                                &mut (*core::ptr::addr_of_mut!(ssb)),
                                            );
                                            bcount = bcount.wrapping_add(1);
                                            acount = 0;
                                            nb = nb.wrapping_sub(1);
                                            if nb == 1 {
                                                break '__join_0;
                                            } else {
                                                if bcount >= min_gallop {
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                    // One run is winning so consistently that galloping may
                                    // be a huge win.  So try that, and continue galloping until
                                    // (if ever) neither run appears to be winning consistently
                                    // anymore.
                                    min_gallop = min_gallop.wrapping_add(1);
                                    loop {
                                        min_gallop = min_gallop
                                            .wrapping_sub(((min_gallop > 1) as i32) as i64);
                                        (*ms).min_gallop = min_gallop;
                                        k = gallop_right(
                                            ms,
                                            *ssb.keys.wrapping_offset(0 as isize),
                                            (*__slate_slot_29).keys,
                                            na,
                                            na.wrapping_sub(1),
                                        );
                                        if k < 0 {
                                            break '__join_3;
                                        } else {
                                            k = na.wrapping_sub(k);
                                            acount = k;
                                            if k != 0 {
                                                sortslice_advance(
                                                    &mut (*core::ptr::addr_of_mut!(
                                                        *__slate_slot_30
                                                    )),
                                                    k.wrapping_neg(),
                                                );
                                                sortslice_advance(
                                                    &mut (*core::ptr::addr_of_mut!(ssa)),
                                                    k.wrapping_neg(),
                                                );
                                                sortslice_memmove(
                                                    &(*core::ptr::addr_of_mut!(*__slate_slot_30)),
                                                    1i64,
                                                    &(*core::ptr::addr_of_mut!(ssa)),
                                                    1i64,
                                                    k,
                                                );
                                                na = na.wrapping_sub(k);
                                                if na == 0 {
                                                    break '__join_4;
                                                }
                                            }
                                            sortslice_copy_decr(
                                                &mut (*core::ptr::addr_of_mut!(*__slate_slot_30)),
                                                &mut (*core::ptr::addr_of_mut!(ssb)),
                                            );
                                            nb = nb.wrapping_sub(1);
                                            if nb == 1 {
                                                break '__join_0;
                                            } else {
                                                k = gallop_left(
                                                    ms,
                                                    *ssa.keys.wrapping_offset(0 as isize),
                                                    (*__slate_slot_28).keys,
                                                    nb,
                                                    nb.wrapping_sub(1),
                                                );
                                                if k < 0 {
                                                    break '__join_3;
                                                } else {
                                                    k = nb.wrapping_sub(k);
                                                    bcount = k;
                                                    if k != 0 {
                                                        sortslice_advance(
                                                            &mut (*core::ptr::addr_of_mut!(
                                                                *__slate_slot_30
                                                            )),
                                                            k.wrapping_neg(),
                                                        );
                                                        sortslice_advance(
                                                            &mut (*core::ptr::addr_of_mut!(ssb)),
                                                            k.wrapping_neg(),
                                                        );
                                                        sortslice_memcpy(
                                                            core::ptr::addr_of_mut!(
                                                                *__slate_slot_30
                                                            ),
                                                            1i64,
                                                            &(*core::ptr::addr_of_mut!(ssb)),
                                                            1i64,
                                                            k,
                                                        );
                                                        nb = nb.wrapping_sub(k);
                                                        if nb == 1 {
                                                            break '__join_0;
                                                        } else {
                                                            // nb==0 is impossible now if the comparison
                                                            // function is consistent, but we can't assume
                                                            // that it is.
                                                            if nb == 0 {
                                                                break '__join_4;
                                                            }
                                                        }
                                                    }
                                                    sortslice_copy_decr(
                                                        &mut (*core::ptr::addr_of_mut!(
                                                            *__slate_slot_30
                                                        )),
                                                        &mut (*core::ptr::addr_of_mut!(ssa)),
                                                    );
                                                    na = na.wrapping_sub(1);
                                                    if na == 0 {
                                                        break '__join_4;
                                                    } else {
                                                        if !(acount >= 7 || bcount >= 7) {
                                                            break;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    min_gallop = min_gallop.wrapping_add(1); // penalize it for leaving galloping mode
                                    (*ms).min_gallop = min_gallop;
                                }
                            }
                        }
                        // The first element of ssb belongs at the front of the merge.
                        sortslice_memmove(
                            &(*core::ptr::addr_of_mut!(*__slate_slot_30)),
                            1i64.wrapping_sub(na),
                            &(*core::ptr::addr_of_mut!(ssa)),
                            1i64.wrapping_sub(na),
                            na,
                        );
                        sortslice_advance(
                            &mut (*core::ptr::addr_of_mut!(*__slate_slot_30)),
                            na.wrapping_neg(),
                        );
                        sortslice_advance(&mut (*core::ptr::addr_of_mut!(ssa)), na.wrapping_neg());
                        sortslice_copy(
                            &(*core::ptr::addr_of_mut!(*__slate_slot_30)),
                            0i64,
                            &(*core::ptr::addr_of_mut!(ssb)),
                            0i64,
                        );
                        return 0;
                    }
                }
                result = 0;
            }
            if nb != 0 {
                sortslice_memcpy(
                    core::ptr::addr_of_mut!(*__slate_slot_30),
                    nb.wrapping_sub(1).wrapping_neg(),
                    &(*core::ptr::addr_of_mut!(*__slate_slot_28)),
                    0i64,
                    nb,
                );
            }
            return result as i64;
        }
    }
}

/// Merge the two runs at stack indices i and i+1.
/// Returns 0 on success, -1 on error.
fn merge_at(ms: *mut s_MergeState, i: i64) -> i64 {
    let mut ssa: __SlateRecord_listobject_c_1606 = unsafe { core::mem::zeroed() };
    let mut na: i64 = 0;
    let mut nb: i64 = 0;
    let mut k: i64 = 0;
    ssa =
        unsafe { (*((*ms).pending.as_mut_ptr() as *mut s_slice).wrapping_offset(i as isize)).base };
    na = unsafe { (*((*ms).pending.as_mut_ptr() as *mut s_slice).wrapping_offset(i as isize)).len };
    let ssb: __SlateRecord_listobject_c_1606 = unsafe {
        (*((*ms).pending.as_mut_ptr() as *mut s_slice).wrapping_offset(i.wrapping_add(1) as isize))
            .base
    };
    nb = unsafe {
        (*((*ms).pending.as_mut_ptr() as *mut s_slice).wrapping_offset(i.wrapping_add(1) as isize))
            .len
    };
    // Record the length of the combined runs; if i is the 3rd-last
    // run now, also slide over the last run (which isn't involved
    // in this merge).  The current run i+1 goes away in any case.
    unsafe {
        (*((*ms).pending.as_mut_ptr() as *mut s_slice).wrapping_offset(i as isize)).len =
            na.wrapping_add(nb);
    }
    if i == (unsafe { (*ms).n }.wrapping_sub(3) as i64) {
        unsafe {
            *((*ms).pending.as_mut_ptr() as *mut s_slice)
                .wrapping_offset(i.wrapping_add(1) as isize) = *((*ms).pending.as_mut_ptr()
                as *mut s_slice)
                .wrapping_offset(i.wrapping_add(2) as isize);
        }
    }
    let __v1: i32 = unsafe { (*ms).n };
    unsafe {
        (*ms).n = __v1.wrapping_sub(1);
    }
    // Where does b start in a?  Elements in a before that can be
    // ignored (already in place).
    k = gallop_right(ms, unsafe { *ssb.keys }, ssa.keys, na, 0i64);
    if k < 0 {
        return 1i32.wrapping_neg() as i64;
    }
    sortslice_advance(unsafe { &mut (*core::ptr::addr_of_mut!(ssa)) }, k);
    na = na.wrapping_sub(k);
    if na == 0 {
        return 0;
    }
    // Where does a end in b?  Elements in b after that can be
    // ignored (already in place).
    nb = gallop_left(
        ms,
        unsafe { *ssa.keys.wrapping_offset(na.wrapping_sub(1) as isize) },
        ssb.keys,
        nb,
        nb.wrapping_sub(1),
    );
    if nb <= 0 {
        return nb;
    }
    // Merge what remains of the runs, using a temp array with
    // min(na, nb) elements.
    if na <= nb {
        return merge_lo(ms, ssa, na, ssb, nb);
    } else {
        return merge_hi(ms, ssa, na, ssb, nb);
    }
}

/// Two adjacent runs begin at index s1. The first run has length n1, and
/// the second run (starting at index s1+n1) has length n2. The list has total
/// length n.
/// Compute the "power" of the first run. See listsort.txt for details.
fn powerloop(s1: i64, n1: i64, n2: i64, n: i64) -> i32 {
    let mut result: i32 = 0;
    // midpoints a and b:
    // a = s1 + n1/2
    // b = s1 + n1 + n2/2 = a + (n1 + n2)/2
    //
    // Those may not be integers, though, because of the "/2". So we work with
    // 2*a and 2*b instead, which are necessarily integers. It makes no
    // difference to the outcome, since the bits in the expansion of (2*i)/n
    // are merely shifted one position from those of i/n.
    let mut a: i64 = 2i64.wrapping_mul(s1).wrapping_add(n1); // 2*a
    let mut b: i64 = a.wrapping_add(n1).wrapping_add(n2); // 2*b
    // Emulate a/n and b/n one bit a time, until bits differ.
    loop {
        result = result.wrapping_add(1);
        if a >= n {
            // both quotient bits are 1
            a = a.wrapping_sub(n);
            b = b.wrapping_sub(n);
        } else {
            if b >= n {
                // a/n bit is 0, b/n bit is 1
                break;
            }
        }
        // else both quotient bits are 0
        a <<= 1i32;
        b <<= 1i32;
    }
    result
}

/// The next run has been identified, of length n2.
/// If there's already a run on the stack, apply the "powersort" merge strategy:
/// compute the topmost run's "power" (depth in a conceptual binary merge tree)
/// and merge adjacent runs on the stack with greater power. See listsort.txt
/// for more info.
///
/// It's the caller's responsibility to push the new run on the stack when this
/// returns.
///
/// Returns 0 on success, -1 on error.
fn found_new_run(ms: *mut s_MergeState, n2: i64) -> i32 {
    if (unsafe { (*ms).n }) != 0 {
        let p: *mut s_slice = unsafe { (*ms).pending.as_mut_ptr() as *mut s_slice };
        let s1: i64 = (unsafe {
            unsafe {
                (*p.wrapping_offset(unsafe { (*ms).n }.wrapping_sub(1) as isize))
                    .base
                    .keys
            }
            .offset_from((*ms).basekeys as *mut *mut _object)
        }) as i64; // start index
        let n1: i64 =
            unsafe { (*p.wrapping_offset(unsafe { (*ms).n }.wrapping_sub(1) as isize)).len };
        let power: i32 = powerloop(s1, n1, n2, unsafe { (*ms).listlen });
        while (unsafe { (*ms).n }) > 1
            && (unsafe { (*p.wrapping_offset(unsafe { (*ms).n }.wrapping_sub(2) as isize)).power })
                > power
        {
            if merge_at(ms, unsafe { (*ms).n }.wrapping_sub(2) as i64) < 0 {
                return 1i32.wrapping_neg();
            }
        }
        unsafe {
            (*p.wrapping_offset(unsafe { (*ms).n }.wrapping_sub(1) as isize)).power = power;
        }
    }
    0
}

/// Regardless of invariants, merge all runs on the stack until only one
/// remains.  This is used at the end of the mergesort.
///
/// Returns 0 on success, -1 on error.
fn merge_force_collapse(ms: *mut s_MergeState) -> i32 {
    let p: *mut s_slice = unsafe { (*ms).pending.as_mut_ptr() as *mut s_slice };
    while (unsafe { (*ms).n }) > 1 {
        let mut n: i64 = unsafe { (*ms).n }.wrapping_sub(2) as i64;
        if n > 0
            && (unsafe { (*p.wrapping_offset(n.wrapping_sub(1) as isize)).len })
                < unsafe { (*p.wrapping_offset(n.wrapping_add(1) as isize)).len }
        {
            n = n.wrapping_sub(1);
        }
        if merge_at(ms, n) < 0 {
            return 1i32.wrapping_neg();
        }
    }
    0
}

/// Compute a good value for the minimum run length; natural runs shorter
/// than this are boosted artificially via binary insertion.
///
/// If n < MAX_MINRUN return n (it's too small to bother with fancy stuff).
/// Else if n is an exact power of 2, return MAX_MINRUN / 2.
/// Else return an int k, MAX_MINRUN / 2 <= k <= MAX_MINRUN, such that n/k is
/// close to, but strictly less than, an exact power of 2.
///
/// See listsort.txt for more info.
fn merge_compute_minrun(mut n: i64) -> i64 {
    let mut r: i64 = 0; // becomes 1 if any 1 bits are shifted off
    while n >= 64 {
        r |= n & 1;
        n >>= 1i32;
    }
    n.wrapping_add(r)
}

// Here we define custom comparison functions to optimize for the cases one commonly
// encounters in practice: homogeneous lists, often of one of the basic types.
// This struct holds the comparison function and helper functions
// selected in the pre-sort check.
// These are the special case compare functions.
// ms->key_compare will always point to one of these:
/// Heterogeneous compare: default, always safe to fall back on.
#[unsafe(link_section = ".text.slate_distinct.listobject.safe_object_compare")]
extern "C-unwind" fn safe_object_compare(
    v: *mut _object,
    w: *mut _object,
    ms: *mut s_MergeState,
) -> i32 {
    // No assumptions necessary!
    unsafe { PyObject_RichCompareBool(v, w, 0i32) }
}

/// Homogeneous compare: safe for any two comparable objects of the same type.
/// (ms->key_richcompare is set to ob_type->tp_richcompare in the
///  pre-sort check.)
#[unsafe(link_section = ".text.slate_distinct.listobject.unsafe_object_compare")]
extern "C-unwind" fn unsafe_object_compare(
    v: *mut _object,
    w: *mut _object,
    ms: *mut s_MergeState,
) -> i32 {
    let mut res: i32 = 0;
    // No assumptions, because we check first:
    if (unsafe { (*Py_TYPE!(v)).tp_richcompare }) != unsafe { (*ms).key_richcompare } {
        return unsafe { PyObject_RichCompareBool(v, w, 0i32) };
    }
    let res_obj: *mut _object = unsafe { unsafe { (*ms).key_richcompare }.unwrap()(v, w, 0i32) };
    if res_obj == unsafe { core::ptr::addr_of_mut!(_Py_NotImplementedStruct) } {
        Py_DECREF!(res_obj);
        return unsafe { PyObject_RichCompareBool(v, w, 0i32) };
    }
    if res_obj.is_null() {
        return 1i32.wrapping_neg();
    }
    if PyBool_Check!(res_obj) != 0 {
        res = (res_obj == _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_TrueStruct) }))
            as i32;
    } else {
        res = unsafe { PyObject_IsTrue(res_obj) };
    }
    Py_DECREF!(res_obj);
    // Note that we can't assert
    //     res == PyObject_RichCompareBool(v, w, Py_LT);
    // because of evil compare functions like this:
    //     lambda a, b:  int(random.random() * 3) - 1)
    // (which is actually in test_sort.py)
    res
}

/// Latin string compare: safe for any two latin (one byte per char) strings.
#[unsafe(link_section = ".text.slate_distinct.listobject.unsafe_latin_compare")]
extern "C-unwind" fn unsafe_latin_compare(
    v: *mut _object,
    w: *mut _object,
    ms: *mut s_MergeState,
) -> i32 {
    let mut len: i64 = 0;
    let mut res: i32 = 0;
    // Modified from Objects/unicodeobject.c:unicode_compare, assuming:
    len = Py_MIN__3!(PyUnicode_GET_LENGTH(v), PyUnicode_GET_LENGTH(w));
    res = unsafe {
        memcmp(
            (PyUnicode_DATA!(v) as *const ()) as *const core::ffi::c_void,
            (PyUnicode_DATA!(w) as *const ()) as *const core::ffi::c_void,
            (len as u64) as usize,
        ) as i32
    };
    let __v1: i32 = if res != 0 {
        (res < 0) as i32
    } else {
        (PyUnicode_GET_LENGTH!(v) < PyUnicode_GET_LENGTH!(w)) as i32
    };
    res = __v1;
    res
}

/// Bounded int compare: compare any two longs that fit in a single machine word.
#[unsafe(link_section = ".text.slate_distinct.listobject.unsafe_long_compare")]
extern "C-unwind" fn unsafe_long_compare(
    v: *mut _object,
    w: *mut _object,
    ms: *mut s_MergeState,
) -> i32 {
    let mut v0: i64 = 0;
    let mut w0: i64 = 0;
    let mut res: i32 = 0;
    // Modified from Objects/longobject.c:long_compare, assuming:
    let vl: *mut _longobject = v as *mut _longobject;
    let wl: *mut _longobject = w as *mut _longobject;
    v0 = _PyLong_CompactValue(vl as *const _longobject);
    w0 = _PyLong_CompactValue(wl as *const _longobject);
    res = (v0 < w0) as i32;
    res
}

/// Float compare: compare any two floats.
#[unsafe(link_section = ".text.slate_distinct.listobject.unsafe_float_compare")]
extern "C-unwind" fn unsafe_float_compare(
    v: *mut _object,
    w: *mut _object,
    ms: *mut s_MergeState,
) -> i32 {
    let mut res: i32 = 0;
    // Modified from Objects/floatobject.c:float_richcompare, assuming:
    res = (PyFloat_AS_DOUBLE!(v) < PyFloat_AS_DOUBLE!(w)) as i32;
    res
}

/// Tuple compare: compare *any* two tuples, using
/// ms->tuple_elem_compare to compare the first elements, which is set
/// using the same pre-sort check as we use for ms->key_compare,
/// but run on the list [x[0] for x in L]. This allows us to optimize compares
/// on two levels (as long as [x[0] for x in L] is type-homogeneous.) The idea is
/// that most tuple compares don't involve x[1:].
#[unsafe(link_section = ".text.slate_distinct.listobject.unsafe_tuple_compare")]
extern "C-unwind" fn unsafe_tuple_compare(
    v: *mut _object,
    w: *mut _object,
    ms: *mut s_MergeState,
) -> i32 {
    let mut i: i64 = 0;
    let mut vlen: i64 = 0;
    let mut wlen: i64 = 0;
    let mut k: i32 = 0;
    // Modified from Objects/tupleobject.c:tuplerichcompare, assuming:
    let vt: *mut __SlateRecord_tupleobject_h_13 = v as *mut __SlateRecord_tupleobject_h_13;
    let wt: *mut __SlateRecord_tupleobject_h_13 = w as *mut __SlateRecord_tupleobject_h_13;
    vlen = Py_SIZE!(vt);
    wlen = Py_SIZE!(wt);
    while i < vlen && i < wlen {
        k = unsafe {
            PyObject_RichCompareBool(
                *((*vt).ob_item.as_mut_ptr() as *mut *mut _object).wrapping_offset(i as isize),
                *((*wt).ob_item.as_mut_ptr() as *mut *mut _object).wrapping_offset(i as isize),
                2i32,
            )
        };
        if k < 0 {
            return 1i32.wrapping_neg();
        }
        if k == 0 {
            break;
        }
        i = i.wrapping_add(1);
    }
    if i >= vlen || i >= wlen {
        return (vlen < wlen) as i32;
    }
    if i == 0 {
        return unsafe {
            unsafe { (*ms).tuple_elem_compare }.unwrap()(
                *((*vt).ob_item.as_mut_ptr() as *mut *mut _object).wrapping_offset(i as isize),
                *((*wt).ob_item.as_mut_ptr() as *mut *mut _object).wrapping_offset(i as isize),
                ms,
            )
        };
    } else {
        return unsafe {
            PyObject_RichCompareBool(
                *((*vt).ob_item.as_mut_ptr() as *mut *mut _object).wrapping_offset(i as isize),
                *((*wt).ob_item.as_mut_ptr() as *mut *mut _object).wrapping_offset(i as isize),
                0i32,
            )
        };
    }
}

// An adaptive, stable, natural mergesort.  See listsort.txt.
// Returns Py_None on success, NULL on error.  Even in case of error, the
// list will be some permutation of its input state (nothing is lost or
// duplicated).
//
// [clinic input]
// @critical_section
// list.sort
//
//     *
//     key as keyfunc: object = None
//     reverse: bool = False
//
// Sort the list in ascending order and return None.
//
// The sort is in-place (i.e. the list itself is modified) and stable
// (i.e. the order of two equal elements is maintained).
//
// If a key function is given, apply it once to each list item and sort
// them, ascending or descending, according to their function values.
//
// The reverse flag can be set to sort in descending order.
// [clinic start generated code]
/// [clinic end generated code: output=57b9f9c5e23fbe42 input=c145526281e1fb9f]
fn list_sort_impl(
    self_: *mut __SlateRecord_listobject_h_22,
    mut keyfunc: *mut _object,
    reverse: i32,
) -> *mut _object {
    let mut force: i64 = unsafe { core::mem::zeroed() };
    let mut n: i64 = unsafe { core::mem::zeroed() };
    let mut __v11: core::mem::MaybeUninit<
        Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object, i32) -> *mut _object>,
    > = core::mem::MaybeUninit::uninit();
    let __slate_slot_11: *mut Option<
        unsafe extern "C-unwind" fn(*mut _object, *mut _object, i32) -> *mut _object,
    > = core::ptr::addr_of_mut!(__v11)
        as *mut Option<
            unsafe extern "C-unwind" fn(*mut _object, *mut _object, i32) -> *mut _object,
        >;
    let mut __v14: bool = unsafe { core::mem::zeroed() };
    let mut __v15: bool = unsafe { core::mem::zeroed() };
    let mut __v16: *mut _object = unsafe { core::mem::zeroed() };
    // Note: for lists of tuples, key is the first element of the tuple
    // lo.keys[i], not lo.keys[i] itself! We verify type-homogeneity
    // for lists of tuples in the if-statement directly above.
    let mut key: *mut _object = unsafe { core::mem::zeroed() };
    let mut __v18: bool = unsafe { core::mem::zeroed() };
    let mut __v19: bool = unsafe { core::mem::zeroed() };
    let mut ints_are_bounded: i32 = unsafe { core::mem::zeroed() };
    let mut strings_are_latin: i32 = unsafe { core::mem::zeroed() };
    let mut keys_are_all_same_type: i32 = unsafe { core::mem::zeroed() };
    let mut __v23: *mut _typeobject = unsafe { core::mem::zeroed() };
    let mut key_type: *mut _typeobject = unsafe { core::mem::zeroed() };
    let mut __v25: bool = unsafe { core::mem::zeroed() };
    // Assume the first element is representative of the whole list.
    let mut keys_are_in_tuples: i32 = unsafe { core::mem::zeroed() };
    let mut keys: *mut *mut _object = unsafe { core::mem::zeroed() };
    let mut i: i64 = unsafe { core::mem::zeroed() };
    // guilty until proved innocent
    let mut result: *mut _object = unsafe { core::mem::zeroed() };
    let mut final_ob_item: *mut *mut _object = unsafe { core::mem::zeroed() };
    let mut saved_ob_item: *mut *mut _object = unsafe { core::mem::zeroed() };
    let mut saved_allocated: i64 = unsafe { core::mem::zeroed() };
    let mut saved_ob_size: i64 = unsafe { core::mem::zeroed() };
    let mut lo: core::mem::MaybeUninit<__SlateRecord_listobject_c_1606> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_38: *mut __SlateRecord_listobject_c_1606 =
        core::ptr::addr_of_mut!(lo) as *mut __SlateRecord_listobject_c_1606;
    let mut minrun: i64 = unsafe { core::mem::zeroed() };
    let mut nremaining: i64 = unsafe { core::mem::zeroed() };
    let mut ms: core::mem::MaybeUninit<s_MergeState> = core::mem::MaybeUninit::uninit();
    let __slate_slot_41: *mut s_MergeState = core::ptr::addr_of_mut!(ms) as *mut s_MergeState;
    unsafe {
        result = core::ptr::null_mut::<_object>();
        if keyfunc == core::ptr::addr_of_mut!(_Py_NoneStruct) {
            keyfunc = core::ptr::null_mut::<_object>();
        }
        '__join_5: {
            '__join_81: {
                // The list is temporarily made empty, so that mutations performed
                // by comparison functions can't affect the slice of memory we're
                // sorting (allowing mutations during sorting is a core-dump
                // factory, since ob_item may change).
                saved_ob_size = Py_SIZE!(self_);
                saved_ob_item = (*self_).ob_item;
                saved_allocated = (*self_).allocated;
                Py_SET_SIZE__2!(self_, 0i32);
                FT_ATOMIC_STORE_PTR_RELEASE__3!((*self_).ob_item);
                (*self_).allocated = 1i32.wrapping_neg() as i64;
                // any operation will reset it to >= 0
                if keyfunc.is_null() {
                    keys = core::ptr::null_mut::<*mut _object>();
                    (*__slate_slot_38).keys = saved_ob_item;
                    (*__slate_slot_38).values = core::ptr::null_mut::<*mut _object>();
                } else {
                    if saved_ob_size < 128i64 {
                        keys = ((*__slate_slot_41).temparray.as_mut_ptr() as *mut *mut _object)
                            .wrapping_offset(saved_ob_size.wrapping_add(1) as isize);
                    } else {
                        keys = PyMem_Malloc(
                            (core::mem::size_of::<*mut _object>() as u64)
                                .wrapping_mul(saved_ob_size as u64),
                        ) as *mut *mut _object;
                        if keys.is_null() {
                            PyErr_NoMemory();
                            break '__join_5;
                        }
                    }
                    // Leverage stack space we allocated but won't otherwise use
                    i = 0;
                    '__join_90: {
                        while i < saved_ob_size {
                            *keys.wrapping_offset(i as isize) = PyObject_CallOneArg(
                                keyfunc,
                                *saved_ob_item.wrapping_offset(i as isize),
                            );
                            if unsafe { *keys.wrapping_offset(i as isize) }.is_null() {
                                break '__join_90;
                            } else {
                                i = i.wrapping_add(1);
                            }
                        }
                        (*__slate_slot_38).keys = keys;
                        (*__slate_slot_38).values = saved_ob_item;
                        break '__join_81;
                    }
                    i = i.wrapping_sub(1);
                    while i >= 0 {
                        Py_DECREF!(unsafe { *keys.wrapping_offset(i as isize) });
                        i = i.wrapping_sub(1);
                    }
                    if saved_ob_size >= 128i64 {
                        PyMem_Free(keys as *mut ());
                        break '__join_5;
                    } else {
                        break '__join_5;
                    }
                }
            }
            // The pre-sort check: here's where we decide which compare function to use.
            // How much optimization is safe? We test for homogeneity with respect to
            // several properties that are expensive to check at compare-time, and
            // set ms appropriately.
            if saved_ob_size > 1 {
                if Py_IS_TYPE!(
                    unsafe { *(*__slate_slot_38).keys.wrapping_offset(0 as isize) },
                    core::ptr::addr_of_mut!(PyTuple_Type)
                ) != 0
                {
                    __v25 = Py_SIZE__2!(unsafe {
                        *(*__slate_slot_38).keys.wrapping_offset(0 as isize)
                    }) > 0;
                } else {
                    __v25 = false;
                }
                keys_are_in_tuples = __v25 as i32;
                if keys_are_in_tuples != 0 {
                    __v23 = Py_TYPE!(PyTuple_GET_ITEM!(
                        unsafe { *(*__slate_slot_38).keys.wrapping_offset(0 as isize) },
                        0i32
                    ));
                } else {
                    __v23 =
                        Py_TYPE!(unsafe { *(*__slate_slot_38).keys.wrapping_offset(0 as isize) });
                }
                key_type = __v23;
                keys_are_all_same_type = 1;
                strings_are_latin = 1;
                ints_are_bounded = 1;
                // Prove that assumption by checking every key.
                i = 0;
                '__join_47: {
                    loop {
                        if i < saved_ob_size {
                            if keys_are_in_tuples != 0 {
                                if Py_IS_TYPE!(
                                    unsafe { *(*__slate_slot_38).keys.wrapping_offset(i as isize) },
                                    core::ptr::addr_of_mut!(PyTuple_Type)
                                ) != 0
                                {
                                    __v18 = Py_SIZE__2!(unsafe {
                                        *(*__slate_slot_38).keys.wrapping_offset(i as isize)
                                    }) != 0;
                                } else {
                                    __v18 = false;
                                }
                                __v19 = !__v18;
                            } else {
                                __v19 = false;
                            }
                            if __v19 {
                                break;
                            } else {
                                if keys_are_in_tuples != 0 {
                                    __v16 = PyTuple_GET_ITEM!(
                                        unsafe {
                                            *(*__slate_slot_38).keys.wrapping_offset(i as isize)
                                        },
                                        0i32
                                    );
                                } else {
                                    __v16 = *(*__slate_slot_38).keys.wrapping_offset(i as isize);
                                }
                                key = __v16;
                                if Py_IS_TYPE!(key, key_type) == 0 {
                                    keys_are_all_same_type = 0;
                                    // If keys are in tuple we must loop over the whole list to make
                                    // sure all items are tuples
                                    if keys_are_in_tuples == 0 {
                                        break '__join_47;
                                    }
                                }
                                if keys_are_all_same_type != 0 {
                                    if key_type == core::ptr::addr_of_mut!(PyLong_Type)
                                        && ints_are_bounded != 0
                                    {
                                        __v15 = _PyLong_IsCompact(key as *const _longobject) == 0;
                                    } else {
                                        __v15 = false;
                                    }
                                    if __v15 {
                                        ints_are_bounded = 0;
                                    } else {
                                        if key_type == core::ptr::addr_of_mut!(PyUnicode_Type)
                                            && strings_are_latin != 0
                                        {
                                            __v14 = PyUnicode_KIND!(key) != 1;
                                        } else {
                                            __v14 = false;
                                        }
                                        if __v14 {
                                            strings_are_latin = 0;
                                        }
                                    }
                                }
                                i = i.wrapping_add(1);
                            }
                        } else {
                            break '__join_47;
                        }
                    }
                    keys_are_in_tuples = 0;
                    keys_are_all_same_type = 0;
                }
                // Choose the best compare, given what we now know about the keys.
                if keys_are_all_same_type != 0 {
                    if key_type == core::ptr::addr_of_mut!(PyUnicode_Type) && strings_are_latin != 0
                    {
                        (*__slate_slot_41).key_compare = Some(unsafe_latin_compare);
                    } else {
                        if key_type == core::ptr::addr_of_mut!(PyLong_Type) && ints_are_bounded != 0
                        {
                            (*__slate_slot_41).key_compare = Some(unsafe_long_compare);
                        } else {
                            if key_type == core::ptr::addr_of_mut!(PyFloat_Type) {
                                (*__slate_slot_41).key_compare = Some(unsafe_float_compare);
                            } else {
                                core::ptr::write(__slate_slot_11, (*key_type).tp_richcompare);
                                (*__slate_slot_41).key_richcompare = *__slate_slot_11;
                                if *__slate_slot_11 != None {
                                    (*__slate_slot_41).key_compare = Some(unsafe_object_compare);
                                } else {
                                    (*__slate_slot_41).key_compare = Some(safe_object_compare);
                                }
                            }
                        }
                    }
                } else {
                    (*__slate_slot_41).key_compare = Some(safe_object_compare);
                }
                if keys_are_in_tuples != 0 {
                    // Make sure we're not dealing with tuples of tuples
                    // (remember: here, key_type refers list [key[0] for key in keys])
                    if key_type == core::ptr::addr_of_mut!(PyTuple_Type) {
                        (*__slate_slot_41).tuple_elem_compare = Some(safe_object_compare);
                    } else {
                        (*__slate_slot_41).tuple_elem_compare = (*__slate_slot_41).key_compare;
                    }
                    (*__slate_slot_41).key_compare = Some(unsafe_tuple_compare);
                }
            }
            '__join_16: {
                // End of pre-sort check: ms is now set properly!
                merge_init(
                    &mut (*core::ptr::addr_of_mut!(*__slate_slot_41)),
                    saved_ob_size,
                    !keys.is_null() as i32,
                    &(*core::ptr::addr_of_mut!(*__slate_slot_38)),
                );
                nremaining = saved_ob_size;
                if nremaining >= 2 {
                    // Reverse sort stability achieved by initially reversing the list,
                    // applying a stable forward sort, then reversing the final result.
                    if reverse != 0 {
                        if !keys.is_null() {
                            reverse_slice(
                                keys.wrapping_offset(0 as isize),
                                keys.wrapping_offset(saved_ob_size as isize),
                            );
                        }
                        reverse_slice(
                            saved_ob_item.wrapping_offset(0 as isize),
                            saved_ob_item.wrapping_offset(saved_ob_size as isize),
                        );
                    }
                    // March over the array once, left to right, finding natural runs,
                    // and extending short natural runs to minrun elements.
                    minrun = merge_compute_minrun(nremaining);
                    loop {
                        // Identify next run.
                        n = count_run(
                            core::ptr::addr_of_mut!(*__slate_slot_41),
                            &(*core::ptr::addr_of_mut!(*__slate_slot_38)),
                            nremaining,
                        );
                        if n < 0 {
                            break '__join_16;
                        } else {
                            // If short, extend to min(minrun, nremaining).
                            if n < minrun {
                                force = if nremaining <= minrun {
                                    nremaining
                                } else {
                                    minrun
                                };
                                if binarysort(
                                    core::ptr::addr_of_mut!(*__slate_slot_41),
                                    &(*(core::ptr::addr_of_mut!(*__slate_slot_38)
                                        as *const __SlateRecord_listobject_c_1606)),
                                    force,
                                    n,
                                ) < 0
                                {
                                    break '__join_16;
                                } else {
                                    n = force;
                                }
                            }
                            // Maybe merge pending runs.
                            if found_new_run(core::ptr::addr_of_mut!(*__slate_slot_41), n) < 0 {
                                break '__join_16;
                            } else {
                                // Push new run on stack.
                                (*__slate_slot_41).pending[(*__slate_slot_41).n as usize].base =
                                    *__slate_slot_38;
                                (*__slate_slot_41).pending[(*__slate_slot_41).n as usize].len = n;
                                let __v8: i32 = (*__slate_slot_41).n;
                                (*__slate_slot_41).n = __v8.wrapping_add(1);
                                // Advance to find next run.
                                sortslice_advance(
                                    &mut (*core::ptr::addr_of_mut!(*__slate_slot_38)),
                                    n,
                                );
                                nremaining = nremaining.wrapping_sub(n);
                                if nremaining == 0 {
                                    break;
                                }
                            }
                        }
                    }
                    if merge_force_collapse(core::ptr::addr_of_mut!(*__slate_slot_41)) < 0 {
                        break '__join_16;
                    } else {
                        *__slate_slot_38 = (*__slate_slot_41).pending[0].base;
                    }
                }
                result = core::ptr::addr_of_mut!(_Py_NoneStruct);
            }
            if !keys.is_null() {
                i = 0;
                while i < saved_ob_size {
                    Py_DECREF!(unsafe { *keys.wrapping_offset(i as isize) });
                    i = i.wrapping_add(1);
                }
                if saved_ob_size >= 128i64 {
                    PyMem_Free(keys as *mut ());
                }
            }
            if (*self_).allocated != (1i32.wrapping_neg() as i64) && !result.is_null() {
                // The user mucked with the list during the sort,
                // and we don't already have another error to report.
                PyErr_SetString(PyExc_ValueError, c"list modified during sort".as_ptr());
                result = core::ptr::null_mut::<_object>();
            }
            if reverse != 0 && saved_ob_size > 1 {
                reverse_slice(
                    saved_ob_item,
                    saved_ob_item.wrapping_offset(saved_ob_size as isize),
                );
            }
            merge_freemem(core::ptr::addr_of_mut!(*__slate_slot_41));
        }
        final_ob_item = (*self_).ob_item;
        i = Py_SIZE!(self_);
        Py_SET_SIZE!(self_, saved_ob_size);
        FT_ATOMIC_STORE_PTR_RELEASE!((*self_).ob_item, saved_ob_item);
        FT_ATOMIC_STORE_SSIZE_RELAXED!((*self_).allocated, saved_allocated);
        if !final_ob_item.is_null() {
            // we cannot use list_clear() for this because it does not
            // guarantee that the list is really empty when it returns
            loop {
                let __v1: i64 = i.wrapping_sub(1);
                i = __v1;
                if __v1 >= 0 {
                    Py_XDECREF!(unsafe { *final_ob_item.wrapping_offset(i as isize) });
                } else {
                    break;
                }
            }
            let use_qsbr: bool = false;
            free_list_items(final_ob_item, use_qsbr);
        }
        return Py_XNewRef!(result);
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyList_Sort(mut v: *mut _object) -> i32 {
    let __v0: bool = v.is_null() || PyList_Check!(v) == 0;
    if __v0 {
        PyErr_BadInternalCall__28!();
        return 1i32.wrapping_neg();
    }
    v = list_sort_impl(
        v as *mut __SlateRecord_listobject_h_22,
        core::ptr::null_mut::<_object>(),
        0i32,
    );
    if v.is_null() {
        return 1i32.wrapping_neg();
    }
    Py_DECREF!(v);
    0
}

// [clinic input]
// @critical_section
// list.reverse
//
// Reverse *IN PLACE*.
// [clinic start generated code]
/// [clinic end generated code: output=482544fc451abea9 input=04ac8e0c6a66e4d9]
fn list_reverse_impl(self_: *mut __SlateRecord_listobject_h_22) -> *mut _object {
    if Py_SIZE!(self_) > 1 {
        reverse_slice(
            unsafe { (*self_).ob_item },
            unsafe { (*self_).ob_item }.wrapping_offset(Py_SIZE!(self_) as isize),
        );
    }
    unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyList_Reverse(v: *mut _object) -> i32 {
    let self_: *mut __SlateRecord_listobject_h_22 = v as *mut __SlateRecord_listobject_h_22;
    let __v0: bool = v.is_null() || PyList_Check!(v) == 0;
    if __v0 {
        PyErr_BadInternalCall__4!();
        return 1i32.wrapping_neg();
    }
    if Py_SIZE!(self_) > 1 {
        reverse_slice(
            unsafe { (*self_).ob_item },
            unsafe { (*self_).ob_item }.wrapping_offset(Py_SIZE!(self_) as isize),
        );
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyList_AsTuple(v: *mut _object) -> *mut _object {
    let __v0: bool = v.is_null() || PyList_Check!(v) == 0;
    if __v0 {
        PyErr_BadInternalCall__86!();
        return core::ptr::null_mut::<_object>();
    }
    let ret: *mut _object = unsafe {
        _PyTuple_FromArray(
            (*(v as *mut __SlateRecord_listobject_h_22)).ob_item as *const *mut _object,
            Py_SIZE__2!(v),
        )
    };
    ret
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyList_AsTupleAndClear(
    self_: *mut __SlateRecord_listobject_h_22,
) -> *mut _object {
    if unsafe { (*self_).ob_item }.is_null() {
        return unsafe { PyTuple_New(0i64) };
    }
    let items: *mut *mut _object = unsafe { (*self_).ob_item };
    let size: i64 = Py_SIZE!(self_);
    unsafe {
        (*self_).ob_item = core::ptr::null_mut::<*mut _object>();
    }
    Py_SET_SIZE__2!(self_, 0i32);
    let ret: *mut _object = unsafe { _PyTuple_FromArraySteal(items as *const *mut _object, size) };
    free_list_items(items, false);
    ret
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyList_FromStackRefStealOnSuccess(
    src: *const _PyStackRef,
    n: i64,
) -> *mut _object {
    if n == 0 {
        return PyList_New(0i64);
    }
    let list: *mut __SlateRecord_listobject_h_22 =
        PyList_New(n) as *mut __SlateRecord_listobject_h_22;
    if list.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let dst: *mut *mut _object = unsafe { (*list).ob_item };
    for i in 0..n {
        unsafe {
            *dst.wrapping_offset(i as isize) =
                PyStackRef_AsPyObjectSteal(*src.wrapping_offset(i as isize));
        }
    }
    list as *mut _object
}

// [clinic input]
// list.index
//
//     value: object
//     start: slice_index(accept={int}) = 0
//     stop: slice_index(accept={int}, c_default="PY_SSIZE_T_MAX") = sys.maxsize
//     /
//
// Return first index of value.
//
// Raises ValueError if the value is not present.
// [clinic start generated code]
/// [clinic end generated code: output=ec51b88787e4e481 input=40ec5826303a0eb1]
fn list_index_impl(
    self_: *mut __SlateRecord_listobject_h_22,
    value: *mut _object,
    mut start: i64,
    mut stop: i64,
) -> *mut _object {
    if start < 0 {
        let __v0: i64 = Py_SIZE!(self_);
        start = start.wrapping_add(__v0);
        if start < 0 {
            start = 0;
        }
    }
    if stop < 0 {
        let __v3: i64 = Py_SIZE!(self_);
        stop = stop.wrapping_add(__v3);
        if stop < 0 {
            stop = 0;
        }
    }
    for i in start..stop {
        let obj: *mut _object = list_get_item_ref(self_, i);
        if obj.is_null() {
            // out-of-bounds
            break;
        }
        let cmp: i32 = unsafe { PyObject_RichCompareBool(obj, value, 2i32) };
        Py_DECREF!(obj);
        if cmp > 0 {
            return unsafe { PyLong_FromSsize_t(i) };
        } else {
            if cmp < 0 {
                return core::ptr::null_mut::<_object>();
            }
        }
    }
    unsafe { PyErr_SetString(PyExc_ValueError, c"list.index(x): x not in list".as_ptr()) };
    core::ptr::null_mut::<_object>()
}

// [clinic input]
// list.count
//
//      value: object
//      /
//
// Return number of occurrences of value.
// [clinic start generated code]
/// [clinic end generated code: output=eff66f14aef2df86 input=3bdc3a5e6f749565]
fn list_count_impl(self_: *mut __SlateRecord_listobject_h_22, value: *mut _object) -> *mut _object {
    let mut count: i64 = 0;
    let mut i: i64 = 0;
    loop {
        let obj: *mut _object = list_get_item_ref(self_, i);
        if obj.is_null() {
            // out-of-bounds
            break;
        }
        if obj == value {
            count = count.wrapping_add(1);
            Py_DECREF!(obj);
        } else {
            let cmp: i32 = unsafe { PyObject_RichCompareBool(obj, value, 2i32) };
            Py_DECREF!(obj);
            if cmp > 0 {
                count = count.wrapping_add(1);
            } else {
                if cmp < 0 {
                    return core::ptr::null_mut::<_object>();
                }
            }
        }
        i = i.wrapping_add(1);
    }
    unsafe { PyLong_FromSsize_t(count) }
}

// [clinic input]
// @critical_section
// list.remove
//
//      value: object
//      /
//
// Remove first occurrence of value.
//
// Raises ValueError if the value is not present.
// [clinic start generated code]
/// [clinic end generated code: output=b9b76a6633b18778 input=26c813dbb95aa93b]
fn list_remove_impl(
    self_: *mut __SlateRecord_listobject_h_22,
    value: *mut _object,
) -> *mut _object {
    let mut i: i64 = 0;
    while i < Py_SIZE!(self_) {
        let obj: *mut _object = unsafe { *unsafe { (*self_).ob_item }.wrapping_offset(i as isize) };
        Py_INCREF!(obj);
        let cmp: i32 = unsafe { PyObject_RichCompareBool(obj, value, 2i32) };
        Py_DECREF!(obj);
        if cmp > 0 {
            if list_ass_slice_lock_held(
                self_,
                i,
                i.wrapping_add(1),
                core::ptr::null_mut::<_object>(),
            ) == 0
            {
                return unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) };
            }
            return core::ptr::null_mut::<_object>();
        } else {
            if cmp < 0 {
                return core::ptr::null_mut::<_object>();
            }
        }
        i = i.wrapping_add(1);
    }
    unsafe { PyErr_SetString(PyExc_ValueError, c"list.remove(x): x not in list".as_ptr()) };
    core::ptr::null_mut::<_object>()
}

#[unsafe(link_section = ".text.slate_distinct.listobject.list_traverse")]
extern "C-unwind" fn list_traverse(
    self_: *mut _object,
    mut visit: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
    arg: *mut (),
) -> i32 {
    let o: *mut __SlateRecord_listobject_h_22 = self_ as *mut __SlateRecord_listobject_h_22;
    let mut i: i64 = Py_SIZE!(o);
    loop {
        let __v2: i64 = i.wrapping_sub(1);
        i = __v2;
        if __v2 < 0 {
            break;
        }
        Py_VISIT!(
            unsafe { *unsafe { (*o).ob_item }.wrapping_offset(i as isize) },
            visit,
            arg
        );
    }
    0
}

fn list_richcompare_impl(v: *mut _object, w: *mut _object, op: i32) -> *mut _object {
    // Search for the first index where items are different
    let mut i: i64 = 0;
    let __v0: bool = PyList_Check!(v) == 0 || PyList_Check!(w) == 0;
    if __v0 {
        return unsafe { core::ptr::addr_of_mut!(_Py_NotImplementedStruct) };
    }
    let vl: *mut __SlateRecord_listobject_h_22 = v as *mut __SlateRecord_listobject_h_22;
    let wl: *mut __SlateRecord_listobject_h_22 = w as *mut __SlateRecord_listobject_h_22;
    if Py_SIZE!(vl) != Py_SIZE!(wl) && (op == 2 || op == 3) {
        // Shortcut: if the lengths differ, the lists differ
        if op == 2 {
            return _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_FalseStruct) });
        } else {
            return _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_TrueStruct) });
        }
    }
    loop {
        let __v2: bool = i < Py_SIZE!(vl) && i < Py_SIZE!(wl);
        if !__v2 {
            break;
        }
        let vitem: *mut _object = unsafe { *unsafe { (*vl).ob_item }.wrapping_offset(i as isize) };
        let witem: *mut _object = unsafe { *unsafe { (*wl).ob_item }.wrapping_offset(i as isize) };
        if vitem != witem {
            Py_INCREF!(vitem);
            Py_INCREF!(witem);
            let k: i32 = unsafe { PyObject_RichCompareBool(vitem, witem, 2i32) };
            Py_DECREF!(vitem);
            Py_DECREF!(witem);
            if k < 0 {
                return core::ptr::null_mut::<_object>();
            }
            if k == 0 {
                break;
            }
        }
        i = i.wrapping_add(1);
    }
    let __v5: bool = i >= Py_SIZE!(vl) || i >= Py_SIZE!(wl);
    if __v5 {
        // No more items to compare -- compare sizes
        loop {
            match op {
                2 => {
                    if Py_SIZE(vl as *mut _object) == Py_SIZE(wl as *mut _object) {
                        return _PyObject_CAST__2!(unsafe {
                            core::ptr::addr_of_mut!(_Py_TrueStruct)
                        });
                    }
                    return _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_FalseStruct) });
                }
                3 => {
                    if Py_SIZE(vl as *mut _object) != Py_SIZE(wl as *mut _object) {
                        return _PyObject_CAST__2!(unsafe {
                            core::ptr::addr_of_mut!(_Py_TrueStruct)
                        });
                    }
                    return _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_FalseStruct) });
                }
                0 => {
                    if Py_SIZE(vl as *mut _object) < Py_SIZE(wl as *mut _object) {
                        return _PyObject_CAST__2!(unsafe {
                            core::ptr::addr_of_mut!(_Py_TrueStruct)
                        });
                    }
                    return _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_FalseStruct) });
                }
                4 => {
                    if Py_SIZE(vl as *mut _object) > Py_SIZE(wl as *mut _object) {
                        return _PyObject_CAST__2!(unsafe {
                            core::ptr::addr_of_mut!(_Py_TrueStruct)
                        });
                    }
                    return _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_FalseStruct) });
                }
                1 => {
                    if Py_SIZE(vl as *mut _object) <= Py_SIZE(wl as *mut _object) {
                        return _PyObject_CAST__2!(unsafe {
                            core::ptr::addr_of_mut!(_Py_TrueStruct)
                        });
                    }
                    return _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_FalseStruct) });
                }
                5 => {
                    if Py_SIZE(vl as *mut _object) >= Py_SIZE(wl as *mut _object) {
                        return _PyObject_CAST__2!(unsafe {
                            core::ptr::addr_of_mut!(_Py_TrueStruct)
                        });
                    }
                    return _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_FalseStruct) });
                }
                _ => {
                    Py_UNREACHABLE!();
                }
            }
            if true {
                break;
            }
        }
    }
    // We have an item that differs -- shortcuts for EQ/NE
    if op == 2 {
        return _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_FalseStruct) });
    }
    if op == 3 {
        return _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_TrueStruct) });
    }
    // Compare the final item again using the proper operator
    let vitem: *mut _object = unsafe { *unsafe { (*vl).ob_item }.wrapping_offset(i as isize) };
    let witem: *mut _object = unsafe { *unsafe { (*wl).ob_item }.wrapping_offset(i as isize) };
    Py_INCREF!(vitem);
    Py_INCREF!(witem);
    let result: *mut _object = unsafe {
        PyObject_RichCompare(
            *unsafe { (*vl).ob_item }.wrapping_offset(i as isize),
            *unsafe { (*wl).ob_item }.wrapping_offset(i as isize),
            op,
        )
    };
    Py_DECREF!(vitem);
    Py_DECREF!(witem);
    result
}

#[unsafe(link_section = ".text.slate_distinct.listobject.list_richcompare")]
extern "C-unwind" fn list_richcompare(v: *mut _object, w: *mut _object, op: i32) -> *mut _object {
    let ret: *mut _object = list_richcompare_impl(v, w, op);
    ret
}

// [clinic input]
// list.__init__
//
//     iterable: object(c_default="NULL") = ()
//     /
//
// Built-in mutable sequence.
//
// If no argument is given, the constructor creates a new empty list.
// The argument must be an iterable if specified.
// [clinic start generated code]
/// [clinic end generated code: output=0f3c21379d01de48 input=b3f3fe7206af8f6b]
fn list___init___impl(self_: *mut __SlateRecord_listobject_h_22, iterable: *mut _object) -> i32 {
    // Verify list invariants established by PyType_GenericAlloc()
    // Empty previous contents
    if !unsafe { (*self_).ob_item }.is_null() {
        list_clear(self_);
    }
    if !iterable.is_null() {
        if _list_extend(self_, iterable) < 0 {
            return 1i32.wrapping_neg();
        }
    }
    0
}

#[unsafe(link_section = ".text.slate_distinct.listobject.list_vectorcall")]
extern "C-unwind" fn list_vectorcall(
    r#type: *mut _object,
    args: *const *mut _object,
    nargsf: u64,
    kwnames: *mut _object,
) -> *mut _object {
    if !_PyArg_NoKwnames!(c"list".as_ptr() as *mut i8, kwnames) {
        return core::ptr::null_mut::<_object>();
    }
    let nargs: i64 = PyVectorcall_NARGS!(nargsf);
    if !_PyArg_CheckPositional!(c"list".as_ptr() as *mut i8, nargs, 0i32, 1i32) {
        return core::ptr::null_mut::<_object>();
    }
    let list: *mut _object = unsafe { PyType_GenericAlloc(_PyType_CAST!(r#type), 0i64) };
    if list.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    if nargs != 0 {
        if list___init___impl(list as *mut __SlateRecord_listobject_h_22, unsafe {
            *args.wrapping_offset(0 as isize)
        }) != 0
        {
            Py_DECREF!(list);
            return core::ptr::null_mut::<_object>();
        }
    }
    list
}

// [clinic input]
// list.__sizeof__
//
// Return the size of the list in memory, in bytes.
// [clinic start generated code]
/// [clinic end generated code: output=3417541f95f9a53e input=b8030a5d5ce8a187]
fn list___sizeof___impl(self_: *mut __SlateRecord_listobject_h_22) -> *mut _object {
    let mut res: u64 = _PyObject_SIZE(Py_TYPE__2!(self_));
    res = res.wrapping_add(
        ((unsafe { (*self_).allocated }) as u64)
            .wrapping_mul(core::mem::size_of::<*mut _object>() as u64),
    );
    unsafe { PyLong_FromSize_t(res) }
}

/// sentinel
static mut list_methods: __SlateAlign16<[PyMethodDef; 16]> = __SlateAlign16([
    {
        let mut __t0: PyMethodDef = unsafe { core::mem::zeroed() };
        __t0.ml_name = (b"__getitem__\0".as_ptr() as *mut i8) as *const i8;
        __t0.ml_meth = Some(list_subscript);
        __t0.ml_flags = 72i32;
        __t0.ml_doc = (b"__getitem__($self, index, /)\n--\n\nReturn self[index].\0".as_ptr()
            as *mut i8) as *const i8;
        __t0
    },
    {
        let mut __t1: PyMethodDef = unsafe { core::mem::zeroed() };
        __t1.ml_name = (b"__reversed__\0".as_ptr() as *mut i8) as *const i8;
        __t1.ml_meth = Some(list___reversed__);
        __t1.ml_flags = 4i32;
        __t1.ml_doc = unsafe { core::ptr::addr_of!(list___reversed____doc__.0) as *const i8 };
        __t1
    },
    {
        let mut __t2: PyMethodDef = unsafe { core::mem::zeroed() };
        __t2.ml_name = (b"__sizeof__\0".as_ptr() as *mut i8) as *const i8;
        __t2.ml_meth = Some(list___sizeof__);
        __t2.ml_flags = 4i32;
        __t2.ml_doc = unsafe { core::ptr::addr_of!(list___sizeof____doc__.0) as *const i8 };
        __t2
    },
    {
        let mut __t3: PyMethodDef = unsafe { core::mem::zeroed() };
        __t3.ml_name = (b"clear\0".as_ptr() as *mut i8) as *const i8;
        __t3.ml_meth = Some(py_list_clear);
        __t3.ml_flags = 4i32;
        __t3.ml_doc = unsafe { core::ptr::addr_of!(py_list_clear__doc__.0) as *const i8 };
        __t3
    },
    {
        let mut __t4: PyMethodDef = unsafe { core::mem::zeroed() };
        __t4.ml_name = (b"copy\0".as_ptr() as *mut i8) as *const i8;
        __t4.ml_meth = Some(list_copy);
        __t4.ml_flags = 4i32;
        __t4.ml_doc = unsafe { core::ptr::addr_of!(list_copy__doc__.0) as *const i8 };
        __t4
    },
    {
        let mut __t5: PyMethodDef = unsafe { core::mem::zeroed() };
        __t5.ml_name = (b"append\0".as_ptr() as *mut i8) as *const i8;
        __t5.ml_meth = Some(list_append);
        __t5.ml_flags = 8i32;
        __t5.ml_doc = unsafe { core::ptr::addr_of!(list_append__doc__.0) as *const i8 };
        __t5
    },
    {
        let mut __t6: PyMethodDef = unsafe { core::mem::zeroed() };
        __t6.ml_name = (b"insert\0".as_ptr() as *mut i8) as *const i8;
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
                >(Some(list_insert))
            })
        };
        __t6.ml_flags = 128i32;
        __t6.ml_doc = unsafe { core::ptr::addr_of!(list_insert__doc__.0) as *const i8 };
        __t6
    },
    {
        let mut __t7: PyMethodDef = unsafe { core::mem::zeroed() };
        __t7.ml_name = (b"extend\0".as_ptr() as *mut i8) as *const i8;
        __t7.ml_meth = Some(list_extend);
        __t7.ml_flags = 8i32;
        __t7.ml_doc = unsafe { core::ptr::addr_of!(list_extend__doc__.0) as *const i8 };
        __t7
    },
    {
        let mut __t8: PyMethodDef = unsafe { core::mem::zeroed() };
        __t8.ml_name = (b"pop\0".as_ptr() as *mut i8) as *const i8;
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
                >(Some(list_pop))
            })
        };
        __t8.ml_flags = 128i32;
        __t8.ml_doc = unsafe { core::ptr::addr_of!(list_pop__doc__.0) as *const i8 };
        __t8
    },
    {
        let mut __t9: PyMethodDef = unsafe { core::mem::zeroed() };
        __t9.ml_name = (b"remove\0".as_ptr() as *mut i8) as *const i8;
        __t9.ml_meth = Some(list_remove);
        __t9.ml_flags = 8i32;
        __t9.ml_doc = unsafe { core::ptr::addr_of!(list_remove__doc__.0) as *const i8 };
        __t9
    },
    {
        let mut __t10: PyMethodDef = unsafe { core::mem::zeroed() };
        __t10.ml_name = (b"index\0".as_ptr() as *mut i8) as *const i8;
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
                >(Some(list_index))
            })
        };
        __t10.ml_flags = 128i32;
        __t10.ml_doc = unsafe { core::ptr::addr_of!(list_index__doc__.0) as *const i8 };
        __t10
    },
    {
        let mut __t11: PyMethodDef = unsafe { core::mem::zeroed() };
        __t11.ml_name = (b"count\0".as_ptr() as *mut i8) as *const i8;
        __t11.ml_meth = Some(list_count);
        __t11.ml_flags = 8i32;
        __t11.ml_doc = unsafe { core::ptr::addr_of!(list_count__doc__.0) as *const i8 };
        __t11
    },
    {
        let mut __t12: PyMethodDef = unsafe { core::mem::zeroed() };
        __t12.ml_name = (b"reverse\0".as_ptr() as *mut i8) as *const i8;
        __t12.ml_meth = Some(list_reverse);
        __t12.ml_flags = 4i32;
        __t12.ml_doc = unsafe { core::ptr::addr_of!(list_reverse__doc__.0) as *const i8 };
        __t12
    },
    {
        let mut __t13: PyMethodDef = unsafe { core::mem::zeroed() };
        __t13.ml_name = (b"sort\0".as_ptr() as *mut i8) as *const i8;
        __t13.ml_meth = unsafe {
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
                >(Some(list_sort))
            })
        };
        __t13.ml_flags = 130i32;
        __t13.ml_doc = unsafe { core::ptr::addr_of!(list_sort__doc__.0) as *const i8 };
        __t13
    },
    {
        let mut __t14: PyMethodDef = unsafe { core::mem::zeroed() };
        __t14.ml_name = (b"__class_getitem__\0".as_ptr() as *mut i8) as *const i8;
        __t14.ml_meth = unsafe {
            core::mem::transmute::<
                *const (),
                Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
            >(Py_GenericAlias as *const ())
        };
        __t14.ml_flags = 24i32;
        __t14.ml_doc = (b"lists are generic over the type of their contents\0".as_ptr() as *mut i8)
            as *const i8;
        __t14
    },
    {
        let mut __t15: PyMethodDef = unsafe { core::mem::zeroed() };
        __t15.ml_name = core::ptr::null::<i8>();
        __t15.ml_meth = None;
        __t15
    },
]);

/// sq_length
/// sq_concat
/// sq_repeat
/// sq_item
/// sq_slice
/// sq_ass_item
/// sq_ass_slice
/// sq_contains
/// sq_inplace_concat
/// sq_inplace_repeat
static mut list_as_sequence: __SlateRecord_object_h_120 = __SlateRecord_object_h_120 {
    sq_length: Some(list_length),
    sq_concat: Some(list_concat),
    sq_repeat: Some(list_repeat),
    sq_item: Some(list_item),
    was_sq_slice: core::ptr::null_mut::<()>(),
    sq_ass_item: Some(list_ass_item),
    was_sq_ass_slice: core::ptr::null_mut::<()>(),
    sq_contains: Some(list_contains),
    sq_inplace_concat: Some(list_inplace_concat),
    sq_inplace_repeat: Some(list_inplace_repeat),
};

fn list_slice_step_lock_held(
    a: *mut __SlateRecord_listobject_h_22,
    start: i64,
    step: i64,
    len: i64,
) -> *mut _object {
    let np: *mut __SlateRecord_listobject_h_22 =
        list_new_prealloc(len) as *mut __SlateRecord_listobject_h_22;
    if np.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let mut cur: u64 = 0;
    let src: *mut *mut _object = unsafe { (*a).ob_item };
    let dest: *mut *mut _object = unsafe { (*np).ob_item };
    cur = start as u64;
    for i in 0..len {
        let v: *mut _object = unsafe { *src.wrapping_offset(cur as isize) };
        unsafe {
            *dest.wrapping_offset(i as isize) = Py_NewRef!(v);
        }
        cur = cur.wrapping_add(step as u64);
    }
    Py_SET_SIZE!(np, len);
    np as *mut _object
}

fn list_slice_wrap(
    aa: *mut __SlateRecord_listobject_h_22,
    mut start: i64,
    mut stop: i64,
    step: i64,
) -> *mut _object {
    let mut res: *mut _object = core::ptr::null_mut::<_object>();
    let len: i64 = unsafe {
        PySlice_AdjustIndices(
            Py_SIZE!(aa),
            core::ptr::addr_of_mut!(start),
            core::ptr::addr_of_mut!(stop),
            step,
        )
    };
    if len <= 0 {
        res = PyList_New(0i64);
    } else {
        if step == 1 {
            res = list_slice_lock_held(aa, start, stop);
        } else {
            res = list_slice_step_lock_held(aa, start, step, len);
        }
    }
    res
}

fn list_slice_subscript(self_: *mut _object, item: *mut _object) -> *mut _object {
    let mut start: i64 = 0;
    let mut stop: i64 = 0;
    let mut step: i64 = 0;
    if (unsafe {
        PySlice_Unpack(
            item,
            core::ptr::addr_of_mut!(start),
            core::ptr::addr_of_mut!(stop),
            core::ptr::addr_of_mut!(step),
        )
    }) < 0
    {
        return core::ptr::null_mut::<_object>();
    }
    list_slice_wrap(
        self_ as *mut __SlateRecord_listobject_h_22,
        start,
        stop,
        step,
    )
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyList_SliceSubscript(
    _self: *mut _object,
    item: *mut _object,
) -> *mut _object {
    list_slice_subscript(_self, item)
}

#[unsafe(link_section = ".text.slate_distinct.listobject.list_subscript")]
extern "C-unwind" fn list_subscript(_self: *mut _object, item: *mut _object) -> *mut _object {
    let self_: *mut __SlateRecord_listobject_h_22 = _self as *mut __SlateRecord_listobject_h_22;
    if _PyIndex_Check(item) != 0 {
        let mut i: i64 = unsafe { PyNumber_AsSsize_t(item, PyExc_IndexError) };
        let __v0: bool =
            i == (1i32.wrapping_neg() as i64) && !unsafe { PyErr_Occurred() }.is_null();
        if __v0 {
            return core::ptr::null_mut::<_object>();
        }
        if i < 0 {
            let __v1: i64 = PyList_GET_SIZE__2!(self_);
            i = i.wrapping_add(__v1);
        }
        return list_item(self_ as *mut _object, i);
    } else {
        if PySlice_Check!(item) != 0 {
            return list_slice_subscript(_self, item);
        } else {
            unsafe {
                PyErr_Format(
                    PyExc_TypeError,
                    c"list indices must be integers or slices, not %.200s".as_ptr(),
                    (*Py_TYPE!(item)).tp_name,
                )
            };
            return core::ptr::null_mut::<_object>();
        }
    }
}

fn adjust_slice_indexes(
    lst: *mut __SlateRecord_listobject_h_22,
    start: *mut i64,
    stop: *mut i64,
    step: i64,
) -> i64 {
    let slicelength: i64 = unsafe { PySlice_AdjustIndices(Py_SIZE!(lst), start, stop, step) };
    // Make sure s[5:2] = [..] inserts at the right place:
    // before 5, not before 2.
    if step < 0 && (unsafe { *start }) < unsafe { *stop }
        || step > 0 && (unsafe { *start }) > unsafe { *stop }
    {
        unsafe {
            *stop = *start;
        }
    }
    slicelength
}

fn list_ass_subscript_lock_held(
    _self: *mut _object,
    item: *mut _object,
    value: *mut _object,
) -> i32 {
    let self_: *mut __SlateRecord_listobject_h_22 = _self as *mut __SlateRecord_listobject_h_22;
    if _PyIndex_Check(item) != 0 {
        let mut i: i64 = unsafe { PyNumber_AsSsize_t(item, PyExc_IndexError) };
        let __v0: bool =
            i == (1i32.wrapping_neg() as i64) && !unsafe { PyErr_Occurred() }.is_null();
        if __v0 {
            return 1i32.wrapping_neg();
        }
        if i < 0 {
            let __v1: i64 = PyList_GET_SIZE__2!(self_);
            i = i.wrapping_add(__v1);
        }
        return list_ass_item_lock_held(self_, i, value);
    } else {
        if PySlice_Check!(item) != 0 {
            let mut start: i64 = 0;
            let mut stop: i64 = 0;
            let mut step: i64 = 0;
            if (unsafe {
                PySlice_Unpack(
                    item,
                    core::ptr::addr_of_mut!(start),
                    core::ptr::addr_of_mut!(stop),
                    core::ptr::addr_of_mut!(step),
                )
            }) < 0
            {
                return 1i32.wrapping_neg();
            }
            if value.is_null() {
                let mut cur: u64 = 0;
                let mut i: i64 = 0;
                let mut res: i32 = 0;
                let slicelength: i64 = adjust_slice_indexes(
                    self_,
                    core::ptr::addr_of_mut!(start),
                    core::ptr::addr_of_mut!(stop),
                    step,
                );
                if step == 1 {
                    return list_ass_slice_lock_held(self_, start, stop, value);
                }
                if slicelength <= 0 {
                    return 0;
                }
                if step < 0 {
                    stop = start.wrapping_add(1);
                    start = stop
                        .wrapping_add(step.wrapping_mul(slicelength.wrapping_sub(1)))
                        .wrapping_sub(1);
                    step = step.wrapping_neg();
                }
                // delete slice
                let garbage: *mut *mut _object = (unsafe {
                    PyMem_Malloc(
                        (slicelength as u64)
                            .wrapping_mul(core::mem::size_of::<*mut _object>() as u64),
                    )
                }) as *mut *mut _object;
                if garbage.is_null() {
                    unsafe { PyErr_NoMemory() };
                    return 1i32.wrapping_neg();
                }
                // drawing pictures might help understand these for
                // loops. Basically, we memmove the parts of the
                // list that are *not* part of the slice: step-1
                // items for each item that is part of the slice,
                // and then tail end of the list that was not
                // covered by the slice
                cur = start as u64;
                while cur < (stop as u64) {
                    let mut lim: i64 = step.wrapping_sub(1);
                    unsafe {
                        *garbage.wrapping_offset(i as isize) = PyList_GET_ITEM__2!(self_, cur);
                    }
                    if cur.wrapping_add(step as u64) >= (Py_SIZE!(self_) as u64) {
                        lim = (Py_SIZE!(self_) as u64).wrapping_sub(cur).wrapping_sub(1) as i64;
                    }
                    unsafe {
                        core::ptr::copy(
                            ((unsafe { (*self_).ob_item }
                                .wrapping_offset(cur as isize)
                                .wrapping_offset(1 as isize)
                                as *const ())
                                as *const core::ffi::c_void)
                                as *const u8,
                            ((unsafe { (*self_).ob_item }
                                .wrapping_offset(cur as isize)
                                .wrapping_offset(-(i as isize))
                                as *mut ()) as *mut core::ffi::c_void)
                                as *mut u8,
                            ((lim as u64).wrapping_mul(core::mem::size_of::<*mut _object>() as u64)
                                as usize) as usize,
                        )
                    };
                    cur = cur.wrapping_add(step as u64);
                    i = i.wrapping_add(1);
                }
                cur = (start as u64).wrapping_add((slicelength as u64).wrapping_mul(step as u64));
                if cur < (Py_SIZE!(self_) as u64) {
                    unsafe {
                        core::ptr::copy(
                            ((unsafe { (*self_).ob_item }.wrapping_offset(cur as isize)
                                as *const ())
                                as *const core::ffi::c_void)
                                as *const u8,
                            ((unsafe { (*self_).ob_item }
                                .wrapping_offset(cur as isize)
                                .wrapping_offset(-(slicelength as isize))
                                as *mut ()) as *mut core::ffi::c_void)
                                as *mut u8,
                            ((Py_SIZE!(self_) as u64)
                                .wrapping_sub(cur)
                                .wrapping_mul(core::mem::size_of::<*mut _object>() as u64)
                                as usize) as usize,
                        )
                    };
                }
                Py_SET_SIZE!(self_, Py_SIZE!(self_).wrapping_sub(slicelength));
                res = list_resize(self_, Py_SIZE!(self_));
                i = 0;
                while i < slicelength {
                    Py_DECREF!(unsafe { *garbage.wrapping_offset(i as isize) });
                    i = i.wrapping_add(1);
                }
                unsafe { PyMem_Free(garbage as *mut ()) };
                return res;
            } else {
                let mut seq: *mut _object = unsafe { core::mem::zeroed() };
                let mut i: i64 = 0;
                let mut cur: u64 = 0;
                // protect against a[::-1] = a
                if self_ == (value as *mut __SlateRecord_listobject_h_22) {
                    seq = list_slice_lock_held(
                        value as *mut __SlateRecord_listobject_h_22,
                        0i64,
                        Py_SIZE__2!(value),
                    );
                } else {
                    seq = unsafe {
                        PySequence_Fast(value, c"must assign iterable to extended slice".as_ptr())
                    };
                }
                if seq.is_null() {
                    return 1i32.wrapping_neg();
                }
                let slicelength: i64 = adjust_slice_indexes(
                    self_,
                    core::ptr::addr_of_mut!(start),
                    core::ptr::addr_of_mut!(stop),
                    step,
                );
                if step == 1 {
                    let res: i32 = list_ass_slice_lock_held(self_, start, stop, seq);
                    Py_DECREF!(seq);
                    return res;
                }
                if PySequence_Fast_GET_SIZE!(seq) != slicelength {
                    unsafe {
                        PyErr_Format(
                            PyExc_ValueError,
                            c"attempt to assign sequence of size %zd to extended slice of size %zd"
                                .as_ptr(),
                            PySequence_Fast_GET_SIZE!(seq),
                            slicelength,
                        )
                    };
                    Py_DECREF!(seq);
                    return 1i32.wrapping_neg();
                }
                if slicelength == 0 {
                    Py_DECREF!(seq);
                    return 0;
                }
                let garbage: *mut *mut _object = (unsafe {
                    PyMem_Malloc(
                        (slicelength as u64)
                            .wrapping_mul(core::mem::size_of::<*mut _object>() as u64),
                    )
                }) as *mut *mut _object;
                if garbage.is_null() {
                    Py_DECREF!(seq);
                    unsafe { PyErr_NoMemory() };
                    return 1i32.wrapping_neg();
                }
                let selfitems: *mut *mut _object = unsafe { (*self_).ob_item };
                let seqitems: *mut *mut _object = PySequence_Fast_ITEMS!(seq);
                cur = start as u64;
                while i < slicelength {
                    unsafe {
                        *garbage.wrapping_offset(i as isize) =
                            *selfitems.wrapping_offset(cur as isize);
                    }
                    // assign slice
                    let ins: *mut _object =
                        Py_NewRef!(unsafe { *seqitems.wrapping_offset(i as isize) });
                    unsafe {
                        *selfitems.wrapping_offset(cur as isize) = ins;
                    }
                    cur = cur.wrapping_add(step as u64);
                    i = i.wrapping_add(1);
                }
                i = 0;
                while i < slicelength {
                    Py_DECREF!(unsafe { *garbage.wrapping_offset(i as isize) });
                    i = i.wrapping_add(1);
                }
                unsafe { PyMem_Free(garbage as *mut ()) };
                Py_DECREF!(seq);
                return 0;
            }
        } else {
            unsafe {
                PyErr_Format(
                    PyExc_TypeError,
                    c"list indices must be integers or slices, not %.200s".as_ptr(),
                    (*Py_TYPE!(item)).tp_name,
                )
            };
            return 1i32.wrapping_neg();
        }
    }
}

#[unsafe(link_section = ".text.slate_distinct.listobject.list_ass_subscript")]
extern "C-unwind" fn list_ass_subscript(
    self_: *mut _object,
    item: *mut _object,
    value: *mut _object,
) -> i32 {
    let mut res: i32 = 0;
    res = list_ass_subscript_lock_held(self_, item, value);
    res
}

static mut list_as_mapping: __SlateRecord_object_h_126 = __SlateRecord_object_h_126 {
    mp_length: Some(list_length),
    mp_subscript: Some(list_subscript),
    mp_ass_subscript: Some(list_ass_subscript),
};

// List Iterator
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
static length_hint_doc: __SlateAlign16<[i8; 55]> = __SlateAlign16([
    80i8, 114i8, 105i8, 118i8, 97i8, 116i8, 101i8, 32i8, 109i8, 101i8, 116i8, 104i8, 111i8, 100i8,
    32i8, 114i8, 101i8, 116i8, 117i8, 114i8, 110i8, 105i8, 110i8, 103i8, 32i8, 97i8, 110i8, 32i8,
    101i8, 115i8, 116i8, 105i8, 109i8, 97i8, 116i8, 101i8, 32i8, 111i8, 102i8, 32i8, 108i8, 101i8,
    110i8, 40i8, 108i8, 105i8, 115i8, 116i8, 40i8, 105i8, 116i8, 41i8, 41i8, 46i8, 0i8,
]);

static reduce_doc: __SlateAlign16<[i8; 39]> = __SlateAlign16([
    82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 115i8, 116i8, 97i8, 116i8, 101i8, 32i8, 105i8,
    110i8, 102i8, 111i8, 114i8, 109i8, 97i8, 116i8, 105i8, 111i8, 110i8, 32i8, 102i8, 111i8, 114i8,
    32i8, 112i8, 105i8, 99i8, 107i8, 108i8, 105i8, 110i8, 103i8, 46i8, 0i8,
]);

static setstate_doc: __SlateAlign16<[i8; 38]> = __SlateAlign16([
    83i8, 101i8, 116i8, 32i8, 115i8, 116i8, 97i8, 116i8, 101i8, 32i8, 105i8, 110i8, 102i8, 111i8,
    114i8, 109i8, 97i8, 116i8, 105i8, 111i8, 110i8, 32i8, 102i8, 111i8, 114i8, 32i8, 117i8, 110i8,
    112i8, 105i8, 99i8, 107i8, 108i8, 105i8, 110i8, 103i8, 46i8, 0i8,
]);

/// sentinel
static mut listiter_methods: __SlateAlign16<[PyMethodDef; 4]> = __SlateAlign16([
    {
        let mut __t0: PyMethodDef = unsafe { core::mem::zeroed() };
        __t0.ml_name = (b"__length_hint__\0".as_ptr() as *mut i8) as *const i8;
        __t0.ml_meth = Some(listiter_len);
        __t0.ml_flags = 4i32;
        __t0.ml_doc = unsafe { core::ptr::addr_of!(length_hint_doc.0) as *const i8 };
        __t0
    },
    {
        let mut __t1: PyMethodDef = unsafe { core::mem::zeroed() };
        __t1.ml_name = (b"__reduce__\0".as_ptr() as *mut i8) as *const i8;
        __t1.ml_meth = Some(listiter_reduce);
        __t1.ml_flags = 4i32;
        __t1.ml_doc = unsafe { core::ptr::addr_of!(reduce_doc.0) as *const i8 };
        __t1
    },
    {
        let mut __t2: PyMethodDef = unsafe { core::mem::zeroed() };
        __t2.ml_name = (b"__setstate__\0".as_ptr() as *mut i8) as *const i8;
        __t2.ml_meth = Some(listiter_setstate);
        __t2.ml_flags = 8i32;
        __t2.ml_doc = unsafe { core::ptr::addr_of!(setstate_doc.0) as *const i8 };
        __t2
    },
    {
        let mut __t3: PyMethodDef = unsafe { core::mem::zeroed() };
        __t3.ml_name = core::ptr::null::<i8>();
        __t3.ml_meth = None;
        __t3
    },
]);

// tp_name
// tp_basicsize
// tp_itemsize
// methods
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
#[unsafe(link_section = ".text.slate_distinct.listobject.list_iter")]
extern "C-unwind" fn list_iter(seq: *mut _object) -> *mut _object {
    if PyList_Check!(seq) == 0 {
        PyErr_BadInternalCall__75!();
        return core::ptr::null_mut::<_object>();
    }
    let mut it: *mut __SlateRecord_pycore_list_h_71 =
        _Py_FREELIST_POP!(__SlateRecord_pycore_list_h_71, list_iters);
    if it.is_null() {
        it = PyObject_GC_New!(__SlateRecord_pycore_list_h_71, unsafe {
            core::ptr::addr_of_mut!(PyListIter_Type)
        });
        if it.is_null() {
            return core::ptr::null_mut::<_object>();
        }
    }
    unsafe {
        (*it).it_index = 0;
    }
    unsafe {
        (*it).it_seq = Py_NewRef!(seq) as *mut __SlateRecord_listobject_h_22;
    }
    _PyObject_GC_TRACK!(it);
    it as *mut _object
}

#[unsafe(link_section = ".text.slate_distinct.listobject.listiter_dealloc")]
extern "C-unwind" fn listiter_dealloc(self_: *mut _object) {
    let it: *mut __SlateRecord_pycore_list_h_71 = self_ as *mut __SlateRecord_pycore_list_h_71;
    _PyObject_GC_UNTRACK__2!(it);
    Py_XDECREF__2!(unsafe { (*it).it_seq });
    _PyFreeList_Free(
        unsafe { core::ptr::addr_of_mut!((*_Py_freelists_GET()).list_iters) },
        _PyObject_CAST__2!(it) as *mut (),
        10i64,
        unsafe {
            core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
                PyObject_GC_Del as *const (),
            )
        },
    );
}

#[unsafe(link_section = ".text.slate_distinct.listobject.listiter_traverse")]
extern "C-unwind" fn listiter_traverse(
    it: *mut _object,
    mut visit: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
    arg: *mut (),
) -> i32 {
    Py_VISIT__11!(
        unsafe { (*(it as *mut __SlateRecord_pycore_list_h_71)).it_seq },
        visit,
        arg
    );
    0
}

#[unsafe(link_section = ".text.slate_distinct.listobject.listiter_next")]
extern "C-unwind" fn listiter_next(self_: *mut _object) -> *mut _object {
    let it: *mut __SlateRecord_pycore_list_h_71 = self_ as *mut __SlateRecord_pycore_list_h_71;
    let index: i64 = unsafe { (*it).it_index };
    if index < 0 {
        return core::ptr::null_mut::<_object>();
    }
    let item: *mut _object = list_get_item_ref(unsafe { (*it).it_seq }, index);
    if item.is_null() {
        // out-of-bounds
        FT_ATOMIC_STORE_SSIZE_RELAXED__2!((*it).it_index, 1i32.wrapping_neg());
        let seq: *mut __SlateRecord_listobject_h_22 = unsafe { (*it).it_seq };
        unsafe {
            (*it).it_seq = core::ptr::null_mut::<__SlateRecord_listobject_h_22>();
        }
        Py_DECREF__2!(seq);
        return core::ptr::null_mut::<_object>();
    }
    FT_ATOMIC_STORE_SSIZE_RELAXED!((*it).it_index, index.wrapping_add(1));
    item
}

#[unsafe(link_section = ".text.slate_distinct.listobject.listiter_len")]
extern "C-unwind" fn listiter_len(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    let it: *mut __SlateRecord_pycore_list_h_71 = self_ as *mut __SlateRecord_pycore_list_h_71;
    let index: i64 = unsafe { (*it).it_index };
    if index >= 0 {
        let len: i64 = PyList_GET_SIZE__2!(unsafe { (*it).it_seq }).wrapping_sub(index);
        if len >= 0 {
            return unsafe { PyLong_FromSsize_t(len) };
        }
    }
    unsafe { PyLong_FromLong(0i64) }
}

#[unsafe(link_section = ".text.slate_distinct.listobject.listiter_reduce")]
extern "C-unwind" fn listiter_reduce(
    it: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    listiter_reduce_general(it as *mut (), 1i32)
}

#[unsafe(link_section = ".text.slate_distinct.listobject.listiter_setstate")]
extern "C-unwind" fn listiter_setstate(self_: *mut _object, state: *mut _object) -> *mut _object {
    let it: *mut __SlateRecord_pycore_list_h_71 = self_ as *mut __SlateRecord_pycore_list_h_71;
    let mut index: i64 = unsafe { PyLong_AsSsize_t(state) };
    let __v0: bool =
        index == (1i32.wrapping_neg() as i64) && !unsafe { PyErr_Occurred() }.is_null();
    if __v0 {
        return core::ptr::null_mut::<_object>();
    }
    if !unsafe { (*it).it_seq }.is_null() {
        if index < (1i32.wrapping_neg() as i64) {
            index = 1i32.wrapping_neg() as i64;
        } else {
            if index > PyList_GET_SIZE__2!(unsafe { (*it).it_seq }) {
                index = PyList_GET_SIZE__2!(unsafe { (*it).it_seq });
            }
        }
        // iterator exhausted
        FT_ATOMIC_STORE_SSIZE_RELAXED!((*it).it_index, index);
    }
    unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
}

// List Reverse Iterator
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_listobject_c_4085 {
    ob_base: _object,
    it_index: i64,
    /// Set to NULL when iterator is exhausted
    it_seq: *mut __SlateRecord_listobject_h_22,
}

/// sentinel
static mut listreviter_methods: __SlateAlign16<[PyMethodDef; 4]> = __SlateAlign16([
    {
        let mut __t0: PyMethodDef = unsafe { core::mem::zeroed() };
        __t0.ml_name = (b"__length_hint__\0".as_ptr() as *mut i8) as *const i8;
        __t0.ml_meth = Some(listreviter_len);
        __t0.ml_flags = 4i32;
        __t0.ml_doc = unsafe { core::ptr::addr_of!(length_hint_doc.0) as *const i8 };
        __t0
    },
    {
        let mut __t1: PyMethodDef = unsafe { core::mem::zeroed() };
        __t1.ml_name = (b"__reduce__\0".as_ptr() as *mut i8) as *const i8;
        __t1.ml_meth = Some(listreviter_reduce);
        __t1.ml_flags = 4i32;
        __t1.ml_doc = unsafe { core::ptr::addr_of!(reduce_doc.0) as *const i8 };
        __t1
    },
    {
        let mut __t2: PyMethodDef = unsafe { core::mem::zeroed() };
        __t2.ml_name = (b"__setstate__\0".as_ptr() as *mut i8) as *const i8;
        __t2.ml_meth = Some(listreviter_setstate);
        __t2.ml_flags = 8i32;
        __t2.ml_doc = unsafe { core::ptr::addr_of!(setstate_doc.0) as *const i8 };
        __t2
    },
    {
        let mut __t3: PyMethodDef = unsafe { core::mem::zeroed() };
        __t3.ml_name = core::ptr::null::<i8>();
        __t3.ml_meth = None;
        __t3
    },
]);

// [clinic input]
// list.__reversed__
//
// Return a reverse iterator over the list.
// [clinic start generated code]
// tp_name
// tp_basicsize
// tp_itemsize
// methods
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
/// [clinic end generated code: output=b166f073208c888c input=eadb6e17f8a6a280]
fn list___reversed___impl(self_: *mut __SlateRecord_listobject_h_22) -> *mut _object {
    let it: *mut __SlateRecord_listobject_c_4085 =
        PyObject_GC_New!(__SlateRecord_listobject_c_4085, unsafe {
            core::ptr::addr_of_mut!(PyListRevIter_Type)
        });
    if it.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    unsafe {
        (*it).it_index = PyList_GET_SIZE__2!(self_).wrapping_sub(1);
    }
    unsafe {
        (*it).it_seq = Py_NewRef__2!(self_) as *mut __SlateRecord_listobject_h_22;
    }
    unsafe { PyObject_GC_Track(it as *mut ()) };
    it as *mut _object
}

#[unsafe(link_section = ".text.slate_distinct.listobject.listreviter_dealloc")]
extern "C-unwind" fn listreviter_dealloc(self_: *mut _object) {
    let it: *mut __SlateRecord_listobject_c_4085 = self_ as *mut __SlateRecord_listobject_c_4085;
    unsafe { PyObject_GC_UnTrack(it as *mut ()) };
    Py_XDECREF__2!(unsafe { (*it).it_seq });
    unsafe { PyObject_GC_Del(it as *mut ()) };
}

#[unsafe(link_section = ".text.slate_distinct.listobject.listreviter_traverse")]
extern "C-unwind" fn listreviter_traverse(
    it: *mut _object,
    mut visit: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
    arg: *mut (),
) -> i32 {
    Py_VISIT__11!(
        unsafe { (*(it as *mut __SlateRecord_listobject_c_4085)).it_seq },
        visit,
        arg
    );
    0
}

#[unsafe(link_section = ".text.slate_distinct.listobject.listreviter_next")]
extern "C-unwind" fn listreviter_next(self_: *mut _object) -> *mut _object {
    let it: *mut __SlateRecord_listobject_c_4085 = self_ as *mut __SlateRecord_listobject_c_4085;
    let index: i64 = unsafe { (*it).it_index };
    if index < 0 {
        return core::ptr::null_mut::<_object>();
    }
    let seq: *mut __SlateRecord_listobject_h_22 = unsafe { (*it).it_seq };
    let item: *mut _object = list_get_item_ref(seq, index);
    if !item.is_null() {
        FT_ATOMIC_STORE_SSIZE_RELAXED!((*it).it_index, index.wrapping_sub(1));
        return item;
    }
    FT_ATOMIC_STORE_SSIZE_RELAXED__2!((*it).it_index, 1i32.wrapping_neg());
    unsafe {
        (*it).it_seq = core::ptr::null_mut::<__SlateRecord_listobject_h_22>();
    }
    Py_DECREF__2!(seq);
    core::ptr::null_mut::<_object>()
}

#[unsafe(link_section = ".text.slate_distinct.listobject.listreviter_len")]
extern "C-unwind" fn listreviter_len(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    let it: *mut __SlateRecord_listobject_c_4085 = self_ as *mut __SlateRecord_listobject_c_4085;
    let index: i64 = unsafe { (*it).it_index };
    let mut len: i64 = index.wrapping_add(1);
    let __v0: bool =
        unsafe { (*it).it_seq }.is_null() || PyList_GET_SIZE__2!(unsafe { (*it).it_seq }) < len;
    if __v0 {
        len = 0;
    }
    unsafe { PyLong_FromSsize_t(len) }
}

#[unsafe(link_section = ".text.slate_distinct.listobject.listreviter_reduce")]
extern "C-unwind" fn listreviter_reduce(
    it: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    listiter_reduce_general(it as *mut (), 0i32)
}

#[unsafe(link_section = ".text.slate_distinct.listobject.listreviter_setstate")]
extern "C-unwind" fn listreviter_setstate(
    self_: *mut _object,
    state: *mut _object,
) -> *mut _object {
    let it: *mut __SlateRecord_listobject_c_4085 = self_ as *mut __SlateRecord_listobject_c_4085;
    let mut index: i64 = unsafe { PyLong_AsSsize_t(state) };
    let __v0: bool =
        index == (1i32.wrapping_neg() as i64) && !unsafe { PyErr_Occurred() }.is_null();
    if __v0 {
        return core::ptr::null_mut::<_object>();
    }
    if !unsafe { (*it).it_seq }.is_null() {
        if index < (1i32.wrapping_neg() as i64) {
            index = 1i32.wrapping_neg() as i64;
        } else {
            if index > PyList_GET_SIZE__2!(unsafe { (*it).it_seq }).wrapping_sub(1) {
                index = PyList_GET_SIZE__2!(unsafe { (*it).it_seq }).wrapping_sub(1);
            }
        }
        FT_ATOMIC_STORE_SSIZE_RELAXED!((*it).it_index, index);
    }
    unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
}

// common pickling support
fn listiter_reduce_general(_it: *mut (), forward: i32) -> *mut _object {
    let mut iter: *mut _object = unsafe { core::mem::zeroed() };
    // _PyEval_GetBuiltin can invoke arbitrary code,
    // call must be before access of iterator pointers.
    // see issue #101765
    if forward != 0 {
        iter = unsafe {
            _PyEval_GetBuiltin(core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py_iter
                    ._ascii
                    .ob_base
            ))
        };
        let idx: i64 = unsafe { (*(_it as *mut __SlateRecord_pycore_list_h_71)).it_index };
        if idx >= 0 {
            return unsafe {
                Py_BuildValue(
                    c"N(O)n".as_ptr(),
                    iter,
                    (*(_it as *mut __SlateRecord_pycore_list_h_71)).it_seq,
                    idx,
                )
            };
        }
    } else {
        iter = unsafe {
            _PyEval_GetBuiltin(core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py_reversed
                    ._ascii
                    .ob_base
            ))
        };
        let idx: i64 = unsafe { (*(_it as *mut __SlateRecord_listobject_c_4085)).it_index };
        if idx >= 0 {
            return unsafe {
                Py_BuildValue(
                    c"N(O)n".as_ptr(),
                    iter,
                    (*(_it as *mut __SlateRecord_listobject_c_4085)).it_seq,
                    idx,
                )
            };
        }
    }
    // empty iterator, create an empty list
    let list: *mut _object = PyList_New(0i64);
    if list.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    unsafe { Py_BuildValue(c"N(N)".as_ptr(), iter, list) }
}
