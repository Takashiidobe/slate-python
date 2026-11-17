//! Long (arbitrary precision) integer object implementation
//! The size of 'long long', as computed by sizeof.
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
//! max(e such that 10**e fits in a digit)
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
//! Almost all errors causing Python initialization to fail
//!
//! other fields are set to 0
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
//! Default int base conversion size limitation: Denial of Service prevention.
//!
//! Chosen such that this isn't wildly slow on modern hardware and so that
//! everyone's existing deployed numpy test suite passes before
//! https://github.com/numpy/numpy/issues/22098 is widely available.
//!
//! $ python -m timeit -s 's = "1"*4300' 'int(s)'
//! 2000 loops, best of 5: 125 usec per loop
//! $ python -m timeit -s 's = "1"*4300; v = int(s)' 'str(v)'
//! 1000 loops, best of 5: 311 usec per loop
//! (zen2 cloud VM)
//!
//! 4300 decimal digits fits a ~14284 bit number.
//! Bit flags for _gc_prev
//!
//! Bit 0 is set when tp_finalize is called
// The size of 'void *', as computed by sizeof.
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
// Smallest negative value of type Py_ssize_t.
// Use static_assert() on C11 and newer
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
macro_rules! CHECK_BINOP {
    ($v:expr, $w:expr) => {{
        loop {
            let __v: bool = PyLong_Check!($v) == 0 || PyLong_Check!($w) == 0;
            if __v {
                return unsafe { core::ptr::addr_of_mut!(_Py_NotImplementedStruct) };
            }
            if true {
                break;
            }
        }
    }};
}

macro_rules! FT_ATOMIC_STORE_PTR_RELAXED {
    ($value:expr, $new_value:expr) => {
        unsafe {
            $value = $new_value;
        }
    };
}

macro_rules! IS_SMALL_INT__2 {
    ($ival:expr) => {
        _PY_IS_SMALL_INT__2!($ival)
    };
}

macro_rules! IS_SMALL_INT {
    ($ival:expr) => {
        _PY_IS_SMALL_INT!($ival)
    };
}

macro_rules! IS_SMALL_UINT {
    ($ival:expr) => {
        ($ival) < 257
    };
}

macro_rules! IS_SMALL_UINT__2 {
    ($ival:expr) => {
        ($ival) < 257
    };
}

macro_rules! OBJECT_STAT_INC {
    () => {};
}

/// Handle small and medium cases.
/// Do shift in two steps to avoid possible undefined behavior.
/// Count digits (at least two - smaller cases were handled above).
/// Construct output value.
macro_rules! PYLONG_FROM_INT {
    ($UINT_TYPE:ty, $INT_TYPE:ty, $ival:expr) => {{
        loop {
            if IS_SMALL_INT!($ival) {
                return get_small_int(($ival) as i32);
            }
            if (1073741823u32 as $INT_TYPE).wrapping_neg() <= $ival
                && ($ival) <= (1073741823u32 as $INT_TYPE)
            {
                return _PyLong_FromMedium(($ival) as i32);
            }
            let abs_ival: $UINT_TYPE = if ($ival) < (0i32 as $INT_TYPE) {
                (0u32 as $UINT_TYPE).wrapping_sub(($ival) as $UINT_TYPE)
            } else {
                ($ival) as $UINT_TYPE
            };
            let mut t: $UINT_TYPE = abs_ival >> 30i32 >> 30i32;
            let mut ndigits: $INT_TYPE = 2i32 as $INT_TYPE;
            while t != 0 {
                let mut __v: $INT_TYPE = ndigits;
                ndigits = __v.wrapping_add(1i32 as $INT_TYPE);
                t >>= 30i32;
            }
            let v: *mut _longobject = long_alloc(ndigits);
            if v.is_null() {
                return core::ptr::null_mut::<_object>();
            }
            let mut p: *mut u32 = unsafe { (*v).long_value.ob_digit.as_mut_ptr() as *mut u32 };
            _PyLong_SetSignAndDigitCount(
                v,
                if ($ival) < (0i32 as $INT_TYPE) {
                    1i32.wrapping_neg()
                } else {
                    1i32
                },
                ndigits,
            );
            t = abs_ival;
            while t != 0 {
                let __v5: *mut u32 = p;
                p = __v5.wrapping_offset(1 as isize);
                unsafe {
                    *__v5 = (t & (1073741823u32 as $UINT_TYPE)) as u32;
                }
                t >>= 30i32;
            }
            return v as *mut _object;
        }
    }};
}

/// Handle small and medium cases.
/// Do shift in two steps to avoid possible undefined behavior.
/// Count digits (at least two - smaller cases were handled above).
/// Construct output value.
macro_rules! PYLONG_FROM_INT__2 {
    ($UINT_TYPE:ty, $INT_TYPE:ty, $ival:expr) => {{
        loop {
            if IS_SMALL_INT__2!($ival) {
                return get_small_int(($ival) as $INT_TYPE);
            }
            if (((1i32 as $UINT_TYPE) << 30i32).wrapping_sub(1i32 as $UINT_TYPE) as $INT_TYPE)
                .wrapping_neg()
                <= $ival
                && ($ival)
                    <= (((1i32 as $UINT_TYPE) << 30i32).wrapping_sub(1i32 as $UINT_TYPE)
                        as $INT_TYPE)
            {
                return _PyLong_FromMedium(($ival) as $INT_TYPE);
            }
            let abs_ival: $UINT_TYPE = if ($ival) < 0 {
                0u32.wrapping_sub(($ival) as $UINT_TYPE)
            } else {
                ($ival) as $UINT_TYPE
            };
            let mut t: $UINT_TYPE = abs_ival >> 30i32 >> 30i32;
            let mut ndigits: i64 = 2;
            while t != 0 {
                let __v: i64 = ndigits;
                ndigits = __v.wrapping_add(1);
                t >>= 30i32;
            }
            let v: *mut _longobject = long_alloc(ndigits);
            if v.is_null() {
                return core::ptr::null_mut::<_object>();
            }
            let mut p: *mut $UINT_TYPE =
                unsafe { (*v).long_value.ob_digit.as_mut_ptr() as *mut $UINT_TYPE };
            _PyLong_SetSignAndDigitCount(
                v,
                if ($ival) < 0 {
                    1i32.wrapping_neg()
                } else {
                    1i32
                },
                ndigits,
            );
            t = abs_ival;
            while t != 0 {
                let __v5: *mut $UINT_TYPE = p;
                p = __v5.wrapping_offset(1 as isize);
                unsafe {
                    *__v5 = t & ((1i32 as $UINT_TYPE) << 30i32).wrapping_sub(1i32 as $UINT_TYPE);
                }
                t >>= 30i32;
            }
            return v as *mut _object;
        }
    }};
}

/// Handle small and medium cases.
/// Do shift in two steps to avoid possible undefined behavior.
/// Count digits (at least two - smaller cases were handled above).
/// Construct output value.
macro_rules! PYLONG_FROM_UINT {
    ($INT_TYPE:ty, $ival:expr) => {{
        loop {
            if IS_SMALL_UINT!($ival) {
                return get_small_int(($ival) as i32);
            }
            if ($ival) <= (1073741823u32 as $INT_TYPE) {
                return _PyLong_FromMedium(($ival) as i32);
            }
            let mut t: $INT_TYPE = ($ival) >> 30i32 >> 30i32;
            let mut ndigits: i64 = 2;
            while t != 0 {
                let __v: i64 = ndigits;
                ndigits = __v.wrapping_add(1);
                t >>= 30i32;
            }
            let v: *mut _longobject = long_alloc(ndigits);
            if v.is_null() {
                return core::ptr::null_mut::<_object>();
            }
            let mut p: *mut u32 = unsafe { (*v).long_value.ob_digit.as_mut_ptr() as *mut u32 };
            while ($ival) != 0 {
                let __v5: *mut u32 = p;
                p = __v5.wrapping_offset(1 as isize);
                unsafe {
                    *__v5 = (($ival) & (1073741823u32 as $INT_TYPE)) as u32;
                }
                let __v7: $INT_TYPE = $ival;
                let __v8: $INT_TYPE = __v7 >> 30i32;
                $ival = __v8;
            }
            return v as *mut _object;
        }
    }};
}

/// Handle small and medium cases.
/// Do shift in two steps to avoid possible undefined behavior.
/// Count digits (at least two - smaller cases were handled above).
/// Construct output value.
macro_rules! PYLONG_FROM_UINT__2 {
    ($INT_TYPE:ty, $ival:expr) => {{
        loop {
            if IS_SMALL_UINT__2!($ival) {
                return get_small_int(($ival) as i32);
            }
            if ($ival) <= ((1i32 as $INT_TYPE) << 30i32).wrapping_sub(1i32 as $INT_TYPE) {
                return _PyLong_FromMedium(($ival) as i32);
            }
            let mut t: $INT_TYPE = ($ival) >> 30i32 >> 30i32;
            let mut ndigits: i64 = 2;
            while t != 0 {
                let __v: i64 = ndigits;
                ndigits = __v.wrapping_add(1);
                t >>= 30i32;
            }
            let v: *mut _longobject = long_alloc(ndigits);
            if v.is_null() {
                return core::ptr::null_mut::<_object>();
            }
            let mut p: *mut $INT_TYPE =
                unsafe { (*v).long_value.ob_digit.as_mut_ptr() as *mut $INT_TYPE };
            while ($ival) != 0 {
                let __v5: *mut $INT_TYPE = p;
                p = __v5.wrapping_offset(1 as isize);
                unsafe {
                    *__v5 =
                        ($ival) & ((1i32 as $INT_TYPE) << 30i32).wrapping_sub(1i32 as $INT_TYPE);
                }
                let __v7: $INT_TYPE = $ival;
                let __v8: $INT_TYPE = __v7 >> 30i32;
                $ival = __v8;
            }
            return v as *mut _object;
        }
    }};
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

macro_rules! PyDict_GET_SIZE {
    ($op:expr) => {
        PyDict_GET_SIZE(_PyObject_CAST!($op))
    };
}

/// Mask the old API with a call to the new API for code compiled under
/// Python 2.0:
macro_rules! PyErr_BadInternalCall__2 {
    () => {
        unsafe {
            _PyErr_BadInternalCall(
                c"/home/takashi/c-corpus/cpython/Objects/longobject.c".as_ptr(),
                1557i32,
            )
        };
    };
}

/// Mask the old API with a call to the new API for code compiled under
/// Python 2.0:
macro_rules! PyErr_BadInternalCall__12 {
    () => {
        unsafe {
            _PyErr_BadInternalCall(
                c"/home/takashi/c-corpus/cpython/Objects/longobject.c".as_ptr(),
                3490i32,
            )
        };
    };
}

/// Mask the old API with a call to the new API for code compiled under
/// Python 2.0:
macro_rules! PyErr_BadInternalCall__20 {
    () => {
        unsafe {
            _PyErr_BadInternalCall(
                c"/home/takashi/c-corpus/cpython/Objects/longobject.c".as_ptr(),
                738i32,
            )
        };
    };
}

/// Mask the old API with a call to the new API for code compiled under
/// Python 2.0:
macro_rules! PyErr_BadInternalCall__25 {
    () => {
        unsafe {
            _PyErr_BadInternalCall(
                c"/home/takashi/c-corpus/cpython/Objects/longobject.c".as_ptr(),
                635i32,
            )
        };
    };
}

/// Mask the old API with a call to the new API for code compiled under
/// Python 2.0:
macro_rules! PyErr_BadInternalCall__34 {
    () => {
        unsafe {
            _PyErr_BadInternalCall(
                c"/home/takashi/c-corpus/cpython/Objects/longobject.c".as_ptr(),
                807i32,
            )
        };
    };
}

/// Mask the old API with a call to the new API for code compiled under
/// Python 2.0:
macro_rules! PyErr_BadInternalCall__39 {
    () => {
        unsafe {
            _PyErr_BadInternalCall(
                c"/home/takashi/c-corpus/cpython/Objects/longobject.c".as_ptr(),
                1632i32,
            )
        };
    };
}

/// Mask the old API with a call to the new API for code compiled under
/// Python 2.0:
macro_rules! PyErr_BadInternalCall__42 {
    () => {
        unsafe {
            _PyErr_BadInternalCall(
                c"/home/takashi/c-corpus/cpython/Objects/longobject.c".as_ptr(),
                1215i32,
            )
        };
    };
}

/// Mask the old API with a call to the new API for code compiled under
/// Python 2.0:
macro_rules! PyErr_BadInternalCall__45 {
    () => {
        unsafe {
            _PyErr_BadInternalCall(
                c"/home/takashi/c-corpus/cpython/Objects/longobject.c".as_ptr(),
                524i32,
            )
        };
    };
}

/// Mask the old API with a call to the new API for code compiled under
/// Python 2.0:
macro_rules! PyErr_BadInternalCall__50 {
    () => {
        unsafe {
            _PyErr_BadInternalCall(
                c"/home/takashi/c-corpus/cpython/Objects/longobject.c".as_ptr(),
                1513i32,
            )
        };
    };
}

/// Mask the old API with a call to the new API for code compiled under
/// Python 2.0:
macro_rules! PyErr_BadInternalCall__58 {
    () => {
        unsafe {
            _PyErr_BadInternalCall(
                c"/home/takashi/c-corpus/cpython/Objects/longobject.c".as_ptr(),
                1605i32,
            )
        };
    };
}

/// Mask the old API with a call to the new API for code compiled under
/// Python 2.0:
macro_rules! PyErr_BadInternalCall__61 {
    () => {
        unsafe {
            _PyErr_BadInternalCall(
                c"/home/takashi/c-corpus/cpython/Objects/longobject.c".as_ptr(),
                1414i32,
            )
        };
    };
}

/// Mask the old API with a call to the new API for code compiled under
/// Python 2.0:
macro_rules! PyErr_BadInternalCall__101 {
    () => {
        unsafe {
            _PyErr_BadInternalCall(
                c"/home/takashi/c-corpus/cpython/Objects/longobject.c".as_ptr(),
                684i32,
            )
        };
    };
}

/// Mask the old API with a call to the new API for code compiled under
/// Python 2.0:
macro_rules! PyErr_BadInternalCall__103 {
    () => {
        unsafe {
            _PyErr_BadInternalCall(
                c"/home/takashi/c-corpus/cpython/Objects/longobject.c".as_ptr(),
                1672i32,
            )
        };
    };
}

/// Mask the old API with a call to the new API for code compiled under
/// Python 2.0:
macro_rules! PyErr_BadInternalCall__106 {
    () => {
        unsafe {
            _PyErr_BadInternalCall(
                c"/home/takashi/c-corpus/cpython/Objects/longobject.c".as_ptr(),
                1392i32,
            )
        };
    };
}

/// Mask the old API with a call to the new API for code compiled under
/// Python 2.0:
macro_rules! PyErr_BadInternalCall__116 {
    () => {
        unsafe {
            _PyErr_BadInternalCall(
                c"/home/takashi/c-corpus/cpython/Objects/longobject.c".as_ptr(),
                780i32,
            )
        };
    };
}

/// Mask the old API with a call to the new API for code compiled under
/// Python 2.0:
macro_rules! PyErr_BadInternalCall__133 {
    () => {
        unsafe {
            _PyErr_BadInternalCall(
                c"/home/takashi/c-corpus/cpython/Objects/longobject.c".as_ptr(),
                2038i32,
            )
        };
    };
}

/// Mask the old API with a call to the new API for code compiled under
/// Python 2.0:
macro_rules! PyErr_BadInternalCall__147 {
    () => {
        unsafe {
            _PyErr_BadInternalCall(
                c"/home/takashi/c-corpus/cpython/Objects/longobject.c".as_ptr(),
                2272i32,
            )
        };
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

macro_rules! PyLong_Check__2 {
    ($op:expr) => {
        PyType_FastSubclass!(Py_TYPE__2!($op), 16777216u64)
    };
}

macro_rules! PyObject_TypeCheck {
    ($ob:expr, $type:expr) => {
        PyObject_TypeCheck(_PyObject_CAST!($ob), $type)
    };
}

/// Alias to PyThreadState_Get()
macro_rules! PyThreadState_GET {
    () => {
        unsafe { PyThreadState_Get() }
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

/// Absolute value of the number x
macro_rules! Py_ABS {
    ($x:expr) => {
        if ($x) < 0 { $x.wrapping_neg() } else { $x }
    };
}

macro_rules! Py_ARITHMETIC_RIGHT_SHIFT {
    ($I:expr, $J:expr) => {
        ($I) >> $J
    };
}

/// Argument must be a char or an int in [-128, 127] or [0, 255].
macro_rules! Py_CHARMASK {
    ($c:expr) => {
        ((($c) as i32) & 255) as u8
    };
}

macro_rules! Py_CLEAR__7 {
    ($op:expr) => {
        loop {
            let _tmp_op_ptr: *mut *mut _longobject = unsafe { core::ptr::addr_of_mut!($op) };
            let _tmp_old_op: *mut _longobject = unsafe { *_tmp_op_ptr };
            if !_tmp_old_op.is_null() {
                unsafe {
                    *_tmp_op_ptr = core::ptr::null_mut::<_longobject>();
                }
                Py_DECREF__2!(_tmp_old_op);
            }
            if true {
                break;
            }
        }
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

macro_rules! Py_CLEAR__10 {
    ($op:expr) => {
        loop {
            let _tmp_op_ptr: *mut *mut _longobject = core::ptr::addr_of_mut!($op);
            let _tmp_old_op: *mut _longobject = unsafe { *_tmp_op_ptr };
            if !_tmp_old_op.is_null() {
                unsafe {
                    *_tmp_op_ptr = core::ptr::null_mut::<_longobject>();
                }
                Py_DECREF__2!(_tmp_old_op);
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
macro_rules! Py_MAX__4 {
    ($x:expr, $y:expr) => {{
        let __v: i64 = if ($x) > (($y) as i64) {
            $x
        } else {
            ($y) as i64
        };
        __v
    }};
}

/// Maximum value between x and y
macro_rules! Py_MAX__2 {
    ($x:expr, $y:expr) => {
        if ($x) > (($y) as i64) {
            $x
        } else {
            ($y) as i64
        }
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

/// Minimum value between x and y
macro_rules! Py_MIN__2 {
    ($x:expr, $y:expr) => {
        if ($x) > (($y) as i64) {
            ($y) as i64
        } else {
            $x
        }
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

macro_rules! Py_SAFE_DOWNCAST {
    ($VALUE:expr, $NARROW:ty) => {
        _Py_STATIC_CAST!($NARROW, $VALUE)
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

macro_rules! Py_SETREF__4 {
    ($dst:expr, $src:expr) => {
        loop {
            let _tmp_dst_ptr: *mut *mut _longobject = core::ptr::addr_of_mut!($dst);
            let _tmp_old_dst: *mut _longobject = unsafe { *_tmp_dst_ptr };
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

macro_rules! TAG_FROM_SIGN_AND_SIZE {
    ($sign:expr, $size:expr) => {
        (1i32.wrapping_sub($sign) as u64) | (($size) as u64) << 3i32
    };
}

macro_rules! WRITE_DIGITS {
    ($p:expr, $size_a:expr, $a:expr, $bits:expr, $base:expr, $alternate:expr, $negative:expr) => {
        loop {
            if ($size_a) == 0 {
                let __v: *mut i8 = $p;
                let __v2: *mut i8 = __v.wrapping_offset(-(1 as isize));
                $p = __v2;
                unsafe {
                    *__v2 = 48;
                }
            } else {
                let mut accum: u64 = 0;
                let mut accumbits: i32 = 0;
                let mut i: i64 = 0;
                while i < $size_a {
                    let __v3: u64 = accum;
                    accum = __v3
                        | ((unsafe {
                            *((*$a).long_value.ob_digit.as_mut_ptr() as *mut u32)
                                .wrapping_offset(i as isize)
                        }) as u64)
                            << accumbits;
                    accumbits = accumbits.wrapping_add(30);
                    loop {
                        let mut cdigit: i8 = (accum & ($base.wrapping_sub(1) as u64)) as i8;
                        cdigit = (cdigit as i32).wrapping_add(if (cdigit as i32) < 10 {
                            48
                        } else {
                            87i32
                        }) as i8;
                        let __v9: *mut i8 = $p;
                        let __v10: *mut i8 = __v9.wrapping_offset(-(1 as isize));
                        $p = __v10;
                        unsafe {
                            *__v10 = cdigit;
                        }
                        let __v11: i32 = accumbits;
                        accumbits = __v11.wrapping_sub($bits);
                        let __v13: u64 = accum;
                        accum = __v13 >> $bits;
                        if (if i < $size_a.wrapping_sub(1) {
                            (accumbits >= $bits) as i32
                        } else {
                            (accum > 0) as i32
                        }) == 0
                        {
                            break;
                        }
                    }
                    i = i.wrapping_add(1);
                }
            }
            if ($alternate) != 0 {
                if ($base) == 16 {
                    let __v17: *mut i8 = $p;
                    let __v18: *mut i8 = __v17.wrapping_offset(-(1 as isize));
                    $p = __v18;
                    unsafe {
                        *__v18 = 120;
                    }
                } else {
                    if ($base) == 8 {
                        let __v19: *mut i8 = $p;
                        let __v20: *mut i8 = __v19.wrapping_offset(-(1 as isize));
                        $p = __v20;
                        unsafe {
                            *__v20 = 111;
                        }
                    } else {
                        let __v21: *mut i8 = $p;
                        let __v22: *mut i8 = __v21.wrapping_offset(-(1 as isize));
                        $p = __v22;
                        unsafe {
                            *__v22 = 98;
                        }
                    }
                }
                let __v23: *mut i8 = $p;
                let __v24: *mut i8 = __v23.wrapping_offset(-(1 as isize));
                $p = __v24;
                unsafe {
                    *__v24 = 48;
                }
            }
            if ($negative) != 0 {
                let __v25: *mut i8 = $p;
                let __v26: *mut i8 = __v25.wrapping_offset(-(1 as isize));
                $p = __v26;
                unsafe {
                    *__v26 = 45;
                }
            }
            if true {
                break;
            }
        }
    };
}

macro_rules! WRITE_DIGITS__2 {
    ($p:expr, $i:expr, $size:expr, $rem:expr, $pout:expr, $j:expr, $negative:expr) => {
        loop {
            $i = 0;
            while ($i) < $size.wrapping_sub(1) {
                $rem = unsafe { *$pout.wrapping_offset(($i) as isize) };
                $j = 0;
                while ($j) < 9 {
                    let __v: *mut u32 = $p;
                    let __v2: *mut u32 = __v.wrapping_offset(-(1 as isize));
                    $p = __v2;
                    unsafe {
                        *__v2 = 48u32.wrapping_add(($rem) % 10);
                    }
                    let __v3: u32 = $rem;
                    let __v4: u32 = __v3 / 10;
                    $rem = __v4;
                    let __v5: i64 = $j;
                    let __v6: i64 = __v5.wrapping_add(1);
                    $j = __v6;
                }
                let __v7: i64 = $i;
                let __v8: i64 = __v7.wrapping_add(1);
                $i = __v8;
            }
            $rem = unsafe { *$pout.wrapping_offset(($i) as isize) };
            loop {
                let __v9: *mut u32 = $p;
                let __v10: *mut u32 = __v9.wrapping_offset(-(1 as isize));
                $p = __v10;
                unsafe {
                    *__v10 = 48u32.wrapping_add(($rem) % 10);
                }
                let __v11: u32 = $rem;
                let __v12: u32 = __v11 / 10;
                $rem = __v12;
                if ($rem) == 0 {
                    break;
                }
            }
            if ($negative) != 0 {
                let __v13: *mut u32 = $p;
                let __v14: *mut u32 = __v13.wrapping_offset(-(1 as isize));
                $p = __v14;
                unsafe {
                    *__v14 = 45;
                }
            }
            if true {
                break;
            }
        }
    };
}

macro_rules! WRITE_DIGITS__3 {
    ($p:expr, $size_a:expr, $a:expr, $bits:expr, $base:expr, $alternate:expr, $negative:expr) => {
        loop {
            if ($size_a) == 0 {
                let __v: *mut u8 = $p;
                let __v2: *mut u8 = __v.wrapping_offset(-(1 as isize));
                $p = __v2;
                unsafe {
                    *__v2 = 48;
                }
            } else {
                let mut accum: u64 = 0;
                let mut accumbits: i32 = 0;
                let mut i: i64 = 0;
                while i < $size_a {
                    let __v3: u64 = accum;
                    accum = __v3
                        | ((unsafe {
                            *((*$a).long_value.ob_digit.as_mut_ptr() as *mut u32)
                                .wrapping_offset(i as isize)
                        }) as u64)
                            << accumbits;
                    accumbits = accumbits.wrapping_add(30);
                    loop {
                        let mut cdigit: i8 = (accum & ($base.wrapping_sub(1) as u64)) as i8;
                        cdigit = (cdigit as i32).wrapping_add(if (cdigit as i32) < 10 {
                            48
                        } else {
                            87i32
                        }) as i8;
                        let __v9: *mut u8 = $p;
                        let __v10: *mut u8 = __v9.wrapping_offset(-(1 as isize));
                        $p = __v10;
                        unsafe {
                            *__v10 = cdigit as u8;
                        }
                        let __v11: i32 = accumbits;
                        accumbits = __v11.wrapping_sub($bits);
                        let __v13: u64 = accum;
                        accum = __v13 >> $bits;
                        if (if i < $size_a.wrapping_sub(1) {
                            (accumbits >= $bits) as i32
                        } else {
                            (accum > 0) as i32
                        }) == 0
                        {
                            break;
                        }
                    }
                    i = i.wrapping_add(1);
                }
            }
            if ($alternate) != 0 {
                if ($base) == 16 {
                    let __v17: *mut u8 = $p;
                    let __v18: *mut u8 = __v17.wrapping_offset(-(1 as isize));
                    $p = __v18;
                    unsafe {
                        *__v18 = 120;
                    }
                } else {
                    if ($base) == 8 {
                        let __v19: *mut u8 = $p;
                        let __v20: *mut u8 = __v19.wrapping_offset(-(1 as isize));
                        $p = __v20;
                        unsafe {
                            *__v20 = 111;
                        }
                    } else {
                        let __v21: *mut u8 = $p;
                        let __v22: *mut u8 = __v21.wrapping_offset(-(1 as isize));
                        $p = __v22;
                        unsafe {
                            *__v22 = 98;
                        }
                    }
                }
                let __v23: *mut u8 = $p;
                let __v24: *mut u8 = __v23.wrapping_offset(-(1 as isize));
                $p = __v24;
                unsafe {
                    *__v24 = 48;
                }
            }
            if ($negative) != 0 {
                let __v25: *mut u8 = $p;
                let __v26: *mut u8 = __v25.wrapping_offset(-(1 as isize));
                $p = __v26;
                unsafe {
                    *__v26 = 45;
                }
            }
            if true {
                break;
            }
        }
    };
}

macro_rules! WRITE_DIGITS__4 {
    ($p:expr, $i:expr, $size:expr, $rem:expr, $pout:expr, $j:expr, $negative:expr) => {
        loop {
            $i = 0;
            while ($i) < $size.wrapping_sub(1) {
                $rem = unsafe { *$pout.wrapping_offset(($i) as isize) };
                $j = 0;
                while ($j) < 9 {
                    let __v: *mut i8 = $p;
                    let __v2: *mut i8 = __v.wrapping_offset(-(1 as isize));
                    $p = __v2;
                    unsafe {
                        *__v2 = 48u32.wrapping_add(($rem) % 10) as i8;
                    }
                    let __v3: u32 = $rem;
                    let __v4: u32 = __v3 / 10;
                    $rem = __v4;
                    let __v5: i64 = $j;
                    let __v6: i64 = __v5.wrapping_add(1);
                    $j = __v6;
                }
                let __v7: i64 = $i;
                let __v8: i64 = __v7.wrapping_add(1);
                $i = __v8;
            }
            $rem = unsafe { *$pout.wrapping_offset(($i) as isize) };
            loop {
                let __v9: *mut i8 = $p;
                let __v10: *mut i8 = __v9.wrapping_offset(-(1 as isize));
                $p = __v10;
                unsafe {
                    *__v10 = 48u32.wrapping_add(($rem) % 10) as i8;
                }
                let __v11: u32 = $rem;
                let __v12: u32 = __v11 / 10;
                $rem = __v12;
                if ($rem) == 0 {
                    break;
                }
            }
            if ($negative) != 0 {
                let __v13: *mut i8 = $p;
                let __v14: *mut i8 = __v13.wrapping_offset(-(1 as isize));
                $p = __v14;
                unsafe {
                    *__v14 = 45;
                }
            }
            if true {
                break;
            }
        }
    };
}

macro_rules! WRITE_DIGITS__5 {
    ($p:expr, $size_a:expr, $a:expr, $bits:expr, $base:expr, $alternate:expr, $negative:expr) => {
        loop {
            if ($size_a) == 0 {
                let __v: *mut u32 = $p;
                let __v2: *mut u32 = __v.wrapping_offset(-(1 as isize));
                $p = __v2;
                unsafe {
                    *__v2 = 48;
                }
            } else {
                let mut accum: u64 = 0;
                let mut accumbits: i32 = 0;
                let mut i: i64 = 0;
                while i < $size_a {
                    let __v3: u64 = accum;
                    accum = __v3
                        | ((unsafe {
                            *((*$a).long_value.ob_digit.as_mut_ptr() as *mut u32)
                                .wrapping_offset(i as isize)
                        }) as u64)
                            << accumbits;
                    accumbits = accumbits.wrapping_add(30);
                    loop {
                        let mut cdigit: i8 = (accum & ($base.wrapping_sub(1) as u64)) as i8;
                        cdigit = (cdigit as i32).wrapping_add(if (cdigit as i32) < 10 {
                            48
                        } else {
                            87i32
                        }) as i8;
                        let __v9: *mut u32 = $p;
                        let __v10: *mut u32 = __v9.wrapping_offset(-(1 as isize));
                        $p = __v10;
                        unsafe {
                            *__v10 = cdigit as u32;
                        }
                        let __v11: i32 = accumbits;
                        accumbits = __v11.wrapping_sub($bits);
                        let __v13: u64 = accum;
                        accum = __v13 >> $bits;
                        if (if i < $size_a.wrapping_sub(1) {
                            (accumbits >= $bits) as i32
                        } else {
                            (accum > 0) as i32
                        }) == 0
                        {
                            break;
                        }
                    }
                    i = i.wrapping_add(1);
                }
            }
            if ($alternate) != 0 {
                if ($base) == 16 {
                    let __v17: *mut u32 = $p;
                    let __v18: *mut u32 = __v17.wrapping_offset(-(1 as isize));
                    $p = __v18;
                    unsafe {
                        *__v18 = 120;
                    }
                } else {
                    if ($base) == 8 {
                        let __v19: *mut u32 = $p;
                        let __v20: *mut u32 = __v19.wrapping_offset(-(1 as isize));
                        $p = __v20;
                        unsafe {
                            *__v20 = 111;
                        }
                    } else {
                        let __v21: *mut u32 = $p;
                        let __v22: *mut u32 = __v21.wrapping_offset(-(1 as isize));
                        $p = __v22;
                        unsafe {
                            *__v22 = 98;
                        }
                    }
                }
                let __v23: *mut u32 = $p;
                let __v24: *mut u32 = __v23.wrapping_offset(-(1 as isize));
                $p = __v24;
                unsafe {
                    *__v24 = 48;
                }
            }
            if ($negative) != 0 {
                let __v25: *mut u32 = $p;
                let __v26: *mut u32 = __v25.wrapping_offset(-(1 as isize));
                $p = __v26;
                unsafe {
                    *__v26 = 45;
                }
            }
            if true {
                break;
            }
        }
    };
}

macro_rules! WRITE_DIGITS__6 {
    ($p:expr, $i:expr, $size:expr, $rem:expr, $pout:expr, $j:expr, $negative:expr) => {
        loop {
            $i = 0;
            while ($i) < $size.wrapping_sub(1) {
                $rem = unsafe { *$pout.wrapping_offset(($i) as isize) };
                $j = 0;
                while ($j) < 9 {
                    let __v: *mut u8 = $p;
                    let __v2: *mut u8 = __v.wrapping_offset(-(1 as isize));
                    $p = __v2;
                    unsafe {
                        *__v2 = 48u32.wrapping_add(($rem) % 10) as u8;
                    }
                    let __v3: u32 = $rem;
                    let __v4: u32 = __v3 / 10;
                    $rem = __v4;
                    let __v5: i64 = $j;
                    let __v6: i64 = __v5.wrapping_add(1);
                    $j = __v6;
                }
                let __v7: i64 = $i;
                let __v8: i64 = __v7.wrapping_add(1);
                $i = __v8;
            }
            $rem = unsafe { *$pout.wrapping_offset(($i) as isize) };
            loop {
                let __v9: *mut u8 = $p;
                let __v10: *mut u8 = __v9.wrapping_offset(-(1 as isize));
                $p = __v10;
                unsafe {
                    *__v10 = 48u32.wrapping_add(($rem) % 10) as u8;
                }
                let __v11: u32 = $rem;
                let __v12: u32 = __v11 / 10;
                $rem = __v12;
                if ($rem) == 0 {
                    break;
                }
            }
            if ($negative) != 0 {
                let __v13: *mut u8 = $p;
                let __v14: *mut u8 = __v13.wrapping_offset(-(1 as isize));
                $p = __v14;
                unsafe {
                    *__v14 = 45;
                }
            }
            if true {
                break;
            }
        }
    };
}

macro_rules! WRITE_DIGITS__7 {
    ($p:expr, $size_a:expr, $a:expr, $bits:expr, $base:expr, $alternate:expr, $negative:expr) => {
        loop {
            if ($size_a) == 0 {
                let __v: *mut u16 = $p;
                let __v2: *mut u16 = __v.wrapping_offset(-(1 as isize));
                $p = __v2;
                unsafe {
                    *__v2 = 48;
                }
            } else {
                let mut accum: u64 = 0;
                let mut accumbits: i32 = 0;
                let mut i: i64 = 0;
                while i < $size_a {
                    let __v3: u64 = accum;
                    accum = __v3
                        | ((unsafe {
                            *((*$a).long_value.ob_digit.as_mut_ptr() as *mut u32)
                                .wrapping_offset(i as isize)
                        }) as u64)
                            << accumbits;
                    accumbits = accumbits.wrapping_add(30);
                    loop {
                        let mut cdigit: i8 = (accum & ($base.wrapping_sub(1) as u64)) as i8;
                        cdigit = (cdigit as i32).wrapping_add(if (cdigit as i32) < 10 {
                            48
                        } else {
                            87i32
                        }) as i8;
                        let __v9: *mut u16 = $p;
                        let __v10: *mut u16 = __v9.wrapping_offset(-(1 as isize));
                        $p = __v10;
                        unsafe {
                            *__v10 = cdigit as u16;
                        }
                        let __v11: i32 = accumbits;
                        accumbits = __v11.wrapping_sub($bits);
                        let __v13: u64 = accum;
                        accum = __v13 >> $bits;
                        if (if i < $size_a.wrapping_sub(1) {
                            (accumbits >= $bits) as i32
                        } else {
                            (accum > 0) as i32
                        }) == 0
                        {
                            break;
                        }
                    }
                    i = i.wrapping_add(1);
                }
            }
            if ($alternate) != 0 {
                if ($base) == 16 {
                    let __v17: *mut u16 = $p;
                    let __v18: *mut u16 = __v17.wrapping_offset(-(1 as isize));
                    $p = __v18;
                    unsafe {
                        *__v18 = 120;
                    }
                } else {
                    if ($base) == 8 {
                        let __v19: *mut u16 = $p;
                        let __v20: *mut u16 = __v19.wrapping_offset(-(1 as isize));
                        $p = __v20;
                        unsafe {
                            *__v20 = 111;
                        }
                    } else {
                        let __v21: *mut u16 = $p;
                        let __v22: *mut u16 = __v21.wrapping_offset(-(1 as isize));
                        $p = __v22;
                        unsafe {
                            *__v22 = 98;
                        }
                    }
                }
                let __v23: *mut u16 = $p;
                let __v24: *mut u16 = __v23.wrapping_offset(-(1 as isize));
                $p = __v24;
                unsafe {
                    *__v24 = 48;
                }
            }
            if ($negative) != 0 {
                let __v25: *mut u16 = $p;
                let __v26: *mut u16 = __v25.wrapping_offset(-(1 as isize));
                $p = __v26;
                unsafe {
                    *__v26 = 45;
                }
            }
            if true {
                break;
            }
        }
    };
}

macro_rules! WRITE_DIGITS__8 {
    ($p:expr, $i:expr, $size:expr, $rem:expr, $pout:expr, $j:expr, $negative:expr) => {
        loop {
            $i = 0;
            while ($i) < $size.wrapping_sub(1) {
                $rem = unsafe { *$pout.wrapping_offset(($i) as isize) };
                $j = 0;
                while ($j) < 9 {
                    let __v: *mut u16 = $p;
                    let __v2: *mut u16 = __v.wrapping_offset(-(1 as isize));
                    $p = __v2;
                    unsafe {
                        *__v2 = 48u32.wrapping_add(($rem) % 10) as u16;
                    }
                    let __v3: u32 = $rem;
                    let __v4: u32 = __v3 / 10;
                    $rem = __v4;
                    let __v5: i64 = $j;
                    let __v6: i64 = __v5.wrapping_add(1);
                    $j = __v6;
                }
                let __v7: i64 = $i;
                let __v8: i64 = __v7.wrapping_add(1);
                $i = __v8;
            }
            $rem = unsafe { *$pout.wrapping_offset(($i) as isize) };
            loop {
                let __v9: *mut u16 = $p;
                let __v10: *mut u16 = __v9.wrapping_offset(-(1 as isize));
                $p = __v10;
                unsafe {
                    *__v10 = 48u32.wrapping_add(($rem) % 10) as u16;
                }
                let __v11: u32 = $rem;
                let __v12: u32 = __v11 / 10;
                $rem = __v12;
                if ($rem) == 0 {
                    break;
                }
            }
            if ($negative) != 0 {
                let __v13: *mut u16 = $p;
                let __v14: *mut u16 = __v13.wrapping_offset(-(1 as isize));
                $p = __v14;
                unsafe {
                    *__v14 = 45;
                }
            }
            if true {
                break;
            }
        }
    };
}

macro_rules! WRITE_UNICODE_DIGITS {
    ($TYPE:ty, $writer:expr, $p:expr, $sz:expr, $v:expr, $size_a:expr, $a:expr, $bits:expr, $base:expr, $alternate:expr, $negative:expr) => {
        loop {
            if !$writer.is_null() {
                $p = (PyUnicode_DATA!(unsafe { (*$writer).buffer }) as *mut $TYPE)
                    .wrapping_offset((unsafe { (*$writer).pos }) as isize)
                    .wrapping_offset(($sz) as isize);
            } else {
                $p = (PyUnicode_DATA!($v) as *mut $TYPE).wrapping_offset(($sz) as isize);
            }
            WRITE_DIGITS__7!($p, $size_a, $a, $bits, $base, $alternate, $negative);
            if $writer.is_null() {}
            if true {
                break;
            }
        }
    };
}

macro_rules! WRITE_UNICODE_DIGITS__2 {
    ($TYPE:ty, $writer:expr, $p:expr, $sz:expr, $v:expr, $size_a:expr, $a:expr, $bits:expr, $base:expr, $alternate:expr, $negative:expr) => {
        loop {
            if !$writer.is_null() {
                $p = (PyUnicode_DATA!(unsafe { (*$writer).buffer }) as *mut $TYPE)
                    .wrapping_offset((unsafe { (*$writer).pos }) as isize)
                    .wrapping_offset(($sz) as isize);
            } else {
                $p = (PyUnicode_DATA!($v) as *mut $TYPE).wrapping_offset(($sz) as isize);
            }
            WRITE_DIGITS__5!($p, $size_a, $a, $bits, $base, $alternate, $negative);
            if $writer.is_null() {}
            if true {
                break;
            }
        }
    };
}

macro_rules! WRITE_UNICODE_DIGITS__3 {
    ($TYPE:ty, $writer:expr, $p:expr, $strlen:expr, $str:expr, $i:expr, $size:expr, $rem:expr, $pout:expr, $j:expr, $negative:expr) => {
        loop {
            if !$writer.is_null() {
                $p = (PyUnicode_DATA!(unsafe { (*$writer).buffer }) as *mut $TYPE)
                    .wrapping_offset((unsafe { (*$writer).pos }) as isize)
                    .wrapping_offset(($strlen) as isize);
            } else {
                $p = (PyUnicode_DATA!($str) as *mut $TYPE).wrapping_offset(($strlen) as isize);
            }
            WRITE_DIGITS__6!($p, $i, $size, $rem, $pout, $j, $negative);
            if $writer.is_null() {}
            if true {
                break;
            }
        }
    };
}

macro_rules! WRITE_UNICODE_DIGITS__4 {
    ($TYPE:ty, $writer:expr, $p:expr, $sz:expr, $v:expr, $size_a:expr, $a:expr, $bits:expr, $base:expr, $alternate:expr, $negative:expr) => {
        loop {
            if !$writer.is_null() {
                $p = (PyUnicode_DATA!(unsafe { (*$writer).buffer }) as *mut $TYPE)
                    .wrapping_offset((unsafe { (*$writer).pos }) as isize)
                    .wrapping_offset(($sz) as isize);
            } else {
                $p = (PyUnicode_DATA!($v) as *mut $TYPE).wrapping_offset(($sz) as isize);
            }
            WRITE_DIGITS__3!($p, $size_a, $a, $bits, $base, $alternate, $negative);
            if $writer.is_null() {}
            if true {
                break;
            }
        }
    };
}

macro_rules! WRITE_UNICODE_DIGITS__5 {
    ($TYPE:ty, $writer:expr, $p:expr, $strlen:expr, $str:expr, $i:expr, $size:expr, $rem:expr, $pout:expr, $j:expr, $negative:expr) => {
        loop {
            if !$writer.is_null() {
                $p = (PyUnicode_DATA!(unsafe { (*$writer).buffer }) as *mut $TYPE)
                    .wrapping_offset((unsafe { (*$writer).pos }) as isize)
                    .wrapping_offset(($strlen) as isize);
            } else {
                $p = (PyUnicode_DATA!($str) as *mut $TYPE).wrapping_offset(($strlen) as isize);
            }
            WRITE_DIGITS__8!($p, $i, $size, $rem, $pout, $j, $negative);
            if $writer.is_null() {}
            if true {
                break;
            }
        }
    };
}

macro_rules! WRITE_UNICODE_DIGITS__6 {
    ($TYPE:ty, $writer:expr, $p:expr, $strlen:expr, $str:expr, $i:expr, $size:expr, $rem:expr, $pout:expr, $j:expr, $negative:expr) => {
        loop {
            if !$writer.is_null() {
                $p = (PyUnicode_DATA!(unsafe { (*$writer).buffer }) as *mut $TYPE)
                    .wrapping_offset((unsafe { (*$writer).pos }) as isize)
                    .wrapping_offset(($strlen) as isize);
            } else {
                $p = (PyUnicode_DATA!($str) as *mut $TYPE).wrapping_offset(($strlen) as isize);
            }
            WRITE_DIGITS__2!($p, $i, $size, $rem, $pout, $j, $negative);
            if $writer.is_null() {}
            if true {
                break;
            }
        }
    };
}

macro_rules! _PY_IS_SMALL_INT {
    ($val:expr) => {
        (5i32.wrapping_neg() as i64) <= $val && ($val) < 257
    };
}

macro_rules! _PY_IS_SMALL_INT__2 {
    ($val:expr) => {
        5i32.wrapping_neg() <= $val && ($val) < 257
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

macro_rules! _PyArg_UnpackKeywords__2 {
    ($args:expr, $nargs:expr, $kwargs:expr, $kwnames:expr, $parser:expr, $minpos:expr, $maxpos:expr, $minkw:expr, $varpos:expr, $buf:expr) => {{
        let __v: bool = ($minkw) == 0
            && $kwargs.is_null()
            && $kwnames.is_null()
            && (($minpos) as i64) <= $nargs
            && (($varpos) != 0 || ($nargs) <= (($maxpos) as i64))
            && !$args.is_null();
        let __v2: *const *mut _object = if __v {
            ($args) as *const *mut _object
        } else {
            unsafe {
                _PyArg_UnpackKeywords(
                    ($args) as *const *mut _object,
                    $nargs,
                    $kwargs,
                    core::ptr::null_mut::<_object>(),
                    $parser,
                    $minpos,
                    $maxpos,
                    $minkw,
                    $varpos,
                    $buf,
                )
            }
        };
        __v2
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

macro_rules! _PyCompactUnicodeObject_CAST {
    ($op:expr) => {
        _Py_CAST!(*mut __SlateRecord_unicodeobject_h_173, $op)
    };
}

macro_rules! _PyLong_CAST {
    ($op:expr) => {
        _Py_CAST!(*mut _longobject, $op)
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

macro_rules! _PyStatus_GET_FUNC__46 {
    () => {
        c"_PyLong_InitTypes".as_ptr() as *mut i8
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

/// Prepare the buffer to write 'length' characters
/// with the specified maximum character.
///
/// Return 0 on success, raise an exception and return -1 on error.
macro_rules! _PyUnicodeWriter_Prepare__4 {
    ($WRITER:expr, $LENGTH:expr, $MAXCHAR:expr) => {{
        let __v: i32 = if (($MAXCHAR) as u32) <= unsafe { (*$WRITER).maxchar }
            && ($LENGTH) <= unsafe { (*$WRITER).size }.wrapping_sub(unsafe { (*$WRITER).pos })
        {
            0
        } else {
            let __v2: i32 = if ($LENGTH) == 0 {
                0
            } else {
                unsafe { _PyUnicodeWriter_PrepareInternal($WRITER, $LENGTH, ($MAXCHAR) as u32) }
            };
            __v2
        };
        __v
    }};
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

macro_rules! medium_value {
    ($x:expr) => {
        _PyLong_CompactValue(($x) as *const _longobject)
    };
}

unsafe extern "C" {
    static mut PyType_Type: _typeobject;
    static mut _Py_NoneStruct: _object;
    static mut _Py_NotImplementedStruct: _object;
    static mut PyByteArray_Type: _typeobject;
    static mut _PyByteArray_empty_string: [i8; 0];
    static mut PyExc_OverflowError: *mut _object;
    static mut PyExc_TypeError: *mut _object;
    static mut PyExc_ValueError: *mut _object;
    static mut PyExc_ZeroDivisionError: *mut _object;
    static mut _Py_FalseStruct: _longobject;
    static mut _Py_TrueStruct: _longobject;
    static mut PyFloat_Type: _typeobject;
    static mut PyTuple_Type: _typeobject;
    static mut _Py_ctype_table: [u32; 256];
    #[thread_local]
    static mut _Py_tss_tstate: *mut _ts;
    static mut _PyRuntime: pyruntimestate;
    fn frexp(arg0: f64, arg1: *mut i32) -> f64;
    fn ldexp(arg0: f64, arg1: i32) -> f64;
    fn remainder(arg0: f64, arg1: f64) -> f64;
    fn div(arg0: i32, arg1: i32) -> __SlateRecord_stdlib_h_67;
    fn index(arg0: *const i8, arg1: i32) -> *mut i8;
    fn PyType_IsSubtype(arg0: *mut _typeobject, arg1: *mut _typeobject) -> i32;
    fn PyObject_Bytes(arg0: *mut _object) -> *mut _object;
    fn PyObject_GenericGetAttr(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn PyObject_IsTrue(arg0: *mut _object) -> i32;
    fn _Py_NewReference(arg0: *mut _object);
    fn _Py_Dealloc(arg0: *mut _object);
    fn PyObject_Malloc(arg0: u64) -> *mut ();
    fn PyObject_Free(arg0: *mut ());
    fn PyBytes_FromStringAndSize(arg0: *const i8, arg1: i64) -> *mut _object;
    fn PyUnicode_FromStringAndSize(arg0: *const i8, arg1: i64) -> *mut _object;
    fn PyUnicode_AsUTF8AndSize(arg0: *mut _object, arg1: *mut i64) -> *const i8;
    fn PyUnicode_New(arg0: i64, arg1: u32) -> *mut _object;
    fn _PyUnicodeWriter_Init(arg0: *mut __SlateRecord_unicodeobject_h_539);
    fn _PyUnicodeWriter_PrepareInternal(
        arg0: *mut __SlateRecord_unicodeobject_h_539,
        arg1: i64,
        arg2: u32,
    ) -> i32;
    fn _PyUnicodeWriter_WriteStr(
        arg0: *mut __SlateRecord_unicodeobject_h_539,
        arg1: *mut _object,
    ) -> i32;
    fn _PyUnicodeWriter_Finish(arg0: *mut __SlateRecord_unicodeobject_h_539) -> *mut _object;
    fn _PyUnicodeWriter_Dealloc(arg0: *mut __SlateRecord_unicodeobject_h_539);
    fn PyErr_SetString(arg0: *mut _object, arg1: *const i8);
    fn PyErr_Occurred() -> *mut _object;
    fn PyErr_Clear();
    fn PyErr_ExceptionMatches(arg0: *mut _object) -> i32;
    fn PyErr_NoMemory() -> *mut _object;
    fn PyErr_Format(arg0: *mut _object, arg1: *const i8, ...) -> *mut _object;
    fn _PyErr_BadInternalCall(arg0: *const i8, arg1: i32);
    fn PyErr_CheckSignals() -> i32;
    fn PyFloat_FromDouble(arg0: f64) -> *mut _object;
    fn PyTuple_New(arg0: i64) -> *mut _object;
    fn PyTuple_Pack(arg0: i64, ...) -> *mut _object;
    fn PyThreadState_Get() -> *mut _ts;
    fn PyStructSequence_New(arg0: *mut _typeobject) -> *mut _object;
    fn PyStructSequence_SetItem(arg0: *mut _object, arg1: i64, arg2: *mut _object);
    fn Py_BuildValue(arg0: *const i8, ...) -> *mut _object;
    fn PyImport_ImportModule(arg0: *const i8) -> *mut _object;
    fn PyObject_CallMethod(
        arg0: *mut _object,
        arg1: *const i8,
        arg2: *const i8,
        ...
    ) -> *mut _object;
    fn PyNumber_AsSsize_t(arg0: *mut _object, arg1: *mut _object) -> i64;
    fn PyNumber_Long(arg0: *mut _object) -> *mut _object;
    fn PyObject_CallOneArg(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn _PyObject_MakeTpCall(
        arg0: *mut _ts,
        arg1: *mut _object,
        arg2: *const *mut _object,
        arg3: i64,
        arg4: *mut _object,
    ) -> *mut _object;
    fn _PyBytesWriter_Prepare(
        arg0: *mut __SlateRecord_pycore_bytesobject_h_89,
        arg1: *mut (),
        arg2: i64,
    ) -> *mut ();
    fn _PyLong_FormatAdvancedWriter(
        arg0: *mut __SlateRecord_unicodeobject_h_539,
        arg1: *mut _object,
        arg2: *mut _object,
        arg3: i64,
        arg4: i64,
    ) -> i32;
    fn _Py_SetImmortal(arg0: *mut _object);
    fn _PyStructSequence_InitBuiltinWithFlags(
        arg0: *mut _is,
        arg1: *mut _typeobject,
        arg2: *mut PyStructSequence_Desc,
        arg3: u64,
    ) -> i32;
    fn _PyStructSequence_FiniBuiltin(arg0: *mut _is, arg1: *mut _typeobject);
    fn _PyUnicode_TransformDecimalAndSpaceToASCII(arg0: *mut _object) -> *mut _object;
    fn _PyUnicode_Equal(arg0: *mut _object, arg1: *mut _object) -> i32;
    fn _PyNumber_Index(arg0: *mut _object) -> *mut _object;
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

fn _Py_atomic_load_int(obj: *const i32) -> i32 {
    unsafe {
        core::sync::atomic::AtomicI32::from_ptr(obj as *mut i32)
            .load(core::sync::atomic::Ordering::SeqCst)
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

#[unsafe(no_mangle)]
static mut PyLong_Type: _typeobject = {
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
    __t0.tp_name = (b"int\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::offset_of!(_longobject, long_value.ob_digit) as u64) as i64;
    __t0.tp_itemsize = (core::mem::size_of::<u32>() as u64) as i64;
    __t0.tp_dealloc = Some(long_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = Some(long_to_decimal_string);
    __t0.tp_as_number = unsafe { core::ptr::addr_of_mut!(long_as_number) };
    __t0.tp_as_sequence = core::ptr::null_mut::<__SlateRecord_object_h_120>();
    __t0.tp_as_mapping = core::ptr::null_mut::<__SlateRecord_object_h_126>();
    __t0.tp_hash = Some(long_hash);
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
    __t0.tp_flags = 20972544u64;
    __t0.tp_doc = unsafe { core::ptr::addr_of!(long_doc.0) as *const i8 };
    __t0.tp_traverse = None;
    __t0.tp_clear = None;
    __t0.tp_richcompare = Some(long_richcompare);
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = None;
    __t0.tp_iternext = None;
    __t0.tp_methods = unsafe { core::ptr::addr_of_mut!(long_methods.0) as *mut PyMethodDef };
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = unsafe { core::ptr::addr_of_mut!(long_getset.0) as *mut PyGetSetDef };
    __t0.tp_base = core::ptr::null_mut::<_typeobject>();
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = 0i64;
    __t0.tp_init = None;
    __t0.tp_alloc = None;
    __t0.tp_new = Some(long_new);
    __t0.tp_free = unsafe {
        core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
            PyObject_Free as *const (),
        )
    };
    __t0.tp_version_tag = 1u32;
    __t0.tp_vectorcall = Some(long_vectorcall);
    __t0
};

#[repr(C)]
#[derive(Clone, Copy)]
struct _frame {}

fn Py_SIZE(ob: *mut _object) -> i64 {
    unsafe { (*_PyVarObject_CAST!(ob)).ob_size }
}

fn Py_IS_TYPE(ob: *mut _object, r#type: *mut _typeobject) -> i32 {
    (Py_TYPE!(ob) == r#type) as i32
}

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
#[repr(C)]
#[derive(Clone, Copy)]
struct _dictkeysobject {}

// Track types initialized using _PyStaticType_InitBuiltin().
// Set if the type allows subclassing
// This undocumented flag gives certain built-ins their unique pattern-matching
//
// behavior, which allows a single positional subpattern to match against the
//
// subject itself (rather than a mapped attribute on it):
// These flags are used to determine if a type is a subclass.
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

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_unicodeobject_h_163_2 {
    __slate_bits_0: __slate_bits::__SlateBits148U0,
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

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord_unicodeobject_h_184_2 {
    any: *mut (),
    latin1: *mut u8,
    ucs2: *mut u16,
    ucs4: *mut u32,
}

fn _Py_NewRef(obj: *mut _object) -> *mut _object {
    Py_INCREF!(obj);
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

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_bytesobject_h_15 {
    ob_base: __SlateRecord_object_h_172,
    ob_shash: i64,
    ob_sval: [i8; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct PyLongWriter {}

fn PyBytes_AS_STRING(op: *mut _object) -> *mut i8 {
    unsafe { (*_PyBytes_CAST!(op)).ob_sval.as_mut_ptr() as *mut i8 }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _dictvalues {}

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

#[repr(C)]
#[derive(Clone, Copy)]
struct _PyExecutorObject {}

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

// 10 ** DECIMAL_SHIFT
#[repr(C)]
#[derive(Clone, Copy)]
struct _PyLongValue {
    lv_tag: u64,
    ob_digit: [u32; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pystate_h_78 {
    __slate_bits_0: __slate_bits::__SlateBits215U0,
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
struct PyLongLayout {
    bits_per_digit: u8,
    digit_size: u8,
    digits_order: i8,
    digit_endianness: i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct PyLongExport {
    value: i64,
    negative: u8,
    ndigits: i64,
    digits: *const (),
    _reserved: u64,
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
struct code_arena_st {}

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

fn _PyThreadState_GET() -> *mut _ts {
    unsafe { _Py_tss_tstate }
}

fn _PyInterpreterState_GET() -> *mut _is {
    let tstate: *mut _ts = _PyThreadState_GET();
    unsafe { (*tstate).interp }
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
struct xi_exceptions {
    PyExc_InterpreterError: *mut _object,
    PyExc_InterpreterNotFoundError: *mut _object,
    PyExc_NotShareableError: *mut _object,
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

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_bytesobject_h_89 {
    buffer: *mut _object,
    allocated: i64,
    min_size: i64,
    use_bytearray: i32,
    overallocate: i32,
    use_small_buffer: i32,
    small_buffer: [i8; 512],
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
struct __SlateRecord_longobject_c_h_27 {
    _this_is_not_used: __SlateRecord_pycore_interp_structs_h_170,
    ob_base: __SlateRecord_object_h_172,
    ob_hash: i64,
    ob_item: [*mut _object; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_longobject_c_h_299 {
    _this_is_not_used: __SlateRecord_pycore_interp_structs_h_170,
    ob_base: __SlateRecord_object_h_172,
    ob_hash: i64,
    ob_item: [*mut _object; 3],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_longobject_c_h_415 {
    _this_is_not_used: __SlateRecord_pycore_interp_structs_h_170,
    ob_base: __SlateRecord_object_h_172,
    ob_hash: i64,
    ob_item: [*mut _object; 3],
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord_longobject_c_1210 {
    v: i64,
    b: [u8; 8],
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
    pub struct __SlateBits148U0 {
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
    pub struct __SlateBits215U0 {
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

use slate_support::memcpy;
use slate_support::memset;
use slate_support::strlen;

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

// Threshold for max digits check.  For performance reasons int() and
// int.__str__() don't checks values that are smaller than this
// threshold.  Acts as a guaranteed minimum size limit for bignums that
// applications can expect from CPython.
//
// % python -m timeit -s 's = "1"*640; v = int(s)' 'str(int(s))'
// 20000 loops, best of 5: 12 usec per loop
//
// "640 digits should be enough for anyone." - gps
// fits a ~2126 bit decimal number.
fn _PyLong_GetZero() -> *mut _object {
    unsafe {
        core::ptr::addr_of_mut!(_PyRuntime.static_objects.singletons.small_ints) as *mut _longobject
    }
    .wrapping_offset(5 as isize) as *mut _object
}

fn _PyLong_GetOne() -> *mut _object {
    unsafe {
        core::ptr::addr_of_mut!(_PyRuntime.static_objects.singletons.small_ints) as *mut _longobject
    }
    .wrapping_offset(6i32 as isize) as *mut _object
}

// Long value tag bits:
// 0-1: Sign bits value = (1-sign), ie. negative=2, positive=0, zero=1.
// 2: Set to 1 for the small ints
// 3+ Unsigned digit count
fn _PyLong_IsNonNegativeCompact(op: *const _longobject) -> i32 {
    ((unsafe { (*op).long_value.lv_tag }) & 18446744073709551611u64 <= 8u64) as i32
}

fn _PyLong_BothAreCompact(a: *const _longobject, b: *const _longobject) -> i32 {
    ((unsafe { (*a).long_value.lv_tag }) | unsafe { (*b).long_value.lv_tag } < 16u64) as i32
}

fn _PyLong_IsZero(op: *const _longobject) -> bool {
    (unsafe { (*op).long_value.lv_tag }) & 3 == 1
}

fn _PyLong_IsNegative(op: *const _longobject) -> bool {
    (unsafe { (*op).long_value.lv_tag }) & 3 == 2
}

fn _PyLong_IsPositive(op: *const _longobject) -> bool {
    (unsafe { (*op).long_value.lv_tag }) & 3 == 0
}

fn _PyLong_IsSmallInt(op: *const _longobject) -> bool {
    let is_small_int: bool = (unsafe { (*op).long_value.lv_tag }) & 4u64 != 0;
    is_small_int
}

fn _PyLong_DigitCount(op: *const _longobject) -> i64 {
    ((unsafe { (*op).long_value.lv_tag }) >> 3i32) as i64
}

fn _PyLong_SignedDigitCount(op: *const _longobject) -> i64 {
    let sign: i64 = 1u64.wrapping_sub((unsafe { (*op).long_value.lv_tag }) & 3) as i64;
    sign.wrapping_mul(((unsafe { (*op).long_value.lv_tag }) >> 3i32) as i64)
}

fn _PyLong_CompactSign(op: *const _longobject) -> i32 {
    1u64.wrapping_sub((unsafe { (*op).long_value.lv_tag }) & 3) as i32
}

fn _PyLong_NonCompactSign(op: *const _longobject) -> i32 {
    1u64.wrapping_sub((unsafe { (*op).long_value.lv_tag }) & 3) as i32
}

fn _PyLong_SameSign(a: *const _longobject, b: *const _longobject) -> i32 {
    ((unsafe { (*a).long_value.lv_tag }) & 3 == (unsafe { (*b).long_value.lv_tag }) & 3) as i32
}

fn _PyLong_SetSignAndDigitCount(op: *mut _longobject, sign: i32, size: i64) {
    unsafe {
        (*op).long_value.lv_tag = TAG_FROM_SIGN_AND_SIZE!(sign, size);
    }
}

fn _PyLong_SetDigitCount(op: *mut _longobject, size: i64) {
    unsafe {
        (*op).long_value.lv_tag = (size as u64) << 3i32 | (*op).long_value.lv_tag & 3;
    }
}

fn _PyLong_FlipSign(op: *mut _longobject) {
    let flipped_sign: u32 = 2u64.wrapping_sub((unsafe { (*op).long_value.lv_tag }) & 3) as u32;
    let __v2: u64 = (unsafe { (*op).long_value.lv_tag }) & 18446744073709551608u64;
    unsafe {
        (*op).long_value.lv_tag = __v2;
    }
    unsafe {
        (*op).long_value.lv_tag = (*op).long_value.lv_tag | (flipped_sign as u64);
    }
}

fn _PyObject_IsUniquelyReferenced(ob: *mut _object) -> i32 {
    (Py_REFCNT!(ob) == 1) as i32
}

fn _Py_DECREF_SPECIALIZED(
    op: *mut _object,
    destruct: Option<unsafe extern "C-unwind" fn(*mut _object)>,
) {
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
fn _PyStructSequence_InitBuiltin(
    interp: *mut _is,
    r#type: *mut _typeobject,
    desc: *mut PyStructSequence_Desc,
) -> i32 {
    unsafe { _PyStructSequence_InitBuiltinWithFlags(interp, r#type, desc, 0u64) }
}

#[unsafe(link_section = ".text.slate_distinct.longobject.long_new")]
extern "C-unwind" fn long_new(
    r#type: *mut _typeobject,
    args: *mut _object,
    kwargs: *mut _object,
) -> *mut _object {
    let mut obase: *mut _object = unsafe { core::mem::zeroed() };
    let mut x: *mut _object = unsafe { core::mem::zeroed() };
    let mut __v4: i64 = unsafe { core::mem::zeroed() };
    let mut noptargs: i64 = unsafe { core::mem::zeroed() };
    let mut nargs: i64 = unsafe { core::mem::zeroed() };
    let mut fastargs: *const *mut _object = unsafe { core::mem::zeroed() };
    let mut argsbuf: core::mem::MaybeUninit<__SlateAlign16<[*mut _object; 2]>> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_8: *mut [*mut _object; 2] =
        core::ptr::addr_of_mut!(argsbuf) as *mut [*mut _object; 2];
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        nargs = PyTuple_GET_SIZE!(args);
        if !kwargs.is_null() {
            __v4 = PyDict_GET_SIZE!(kwargs);
        } else {
            __v4 = 0;
        }
        noptargs = nargs.wrapping_add(__v4).wrapping_sub(0);
        x = core::ptr::null_mut::<_object>();
        obase = core::ptr::null_mut::<_object>();
        fastargs = _PyArg_UnpackKeywords__2!(
            (*(args as *mut __SlateRecord_tupleobject_h_13))
                .ob_item
                .as_mut_ptr() as *mut *mut _object,
            nargs,
            kwargs,
            core::ptr::null_mut::<()>(),
            core::ptr::addr_of_mut!(_parser__40),
            0i32,
            2i32,
            0i32,
            0i32,
            (*__slate_slot_8).as_mut_ptr() as *mut *mut _object
        );
        // minpos
        // maxpos
        // minkw
        // varpos
        if !fastargs.is_null() {
            if nargs >= 1 {
                noptargs = noptargs.wrapping_sub(1);
                x = *fastargs.wrapping_offset(0 as isize);
            }
            if noptargs != 0 {
                obase = *fastargs.wrapping_offset(1 as isize);
            }
            return_value = long_new_impl(r#type, x, obase);
        }
        return return_value;
    }
}

static mut _kwtuple__27: __SlateRecord_longobject_c_h_27 = {
    let mut __t0: __SlateRecord_longobject_c_h_27 = unsafe { core::mem::zeroed() };
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
                ._py_base
                ._ascii
                .ob_base
        )
    }];
    __t0
};

static _keywords__39: __SlateAlign16<[*const i8; 3]> = __SlateAlign16([
    (b"\0".as_ptr() as *mut i8) as *const i8,
    (b"base\0".as_ptr() as *mut i8) as *const i8,
    core::ptr::null::<i8>(),
]);

static mut _parser__40: _PyArg_Parser = {
    let mut __t0: _PyArg_Parser = unsafe { core::mem::zeroed() };
    __t0.keywords = unsafe { core::ptr::addr_of!(_keywords__39.0) as *const *const i8 };
    __t0.fname = (b"int\0".as_ptr() as *mut i8) as *const i8;
    __t0.kwtuple = unsafe { core::ptr::addr_of_mut!(_kwtuple__27.ob_base.ob_base) };
    __t0
};

static int___getnewargs____doc__: __SlateAlign16<[i8; 30]> = __SlateAlign16([
    95i8, 95i8, 103i8, 101i8, 116i8, 110i8, 101i8, 119i8, 97i8, 114i8, 103i8, 115i8, 95i8, 95i8,
    40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8,
    0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.longobject.int___getnewargs__")]
extern "C-unwind" fn int___getnewargs__(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    int___getnewargs___impl(self_)
}

static int___format____doc__: __SlateAlign16<[i8; 84]> = __SlateAlign16([
    95i8, 95i8, 102i8, 111i8, 114i8, 109i8, 97i8, 116i8, 95i8, 95i8, 40i8, 36i8, 115i8, 101i8,
    108i8, 102i8, 44i8, 32i8, 102i8, 111i8, 114i8, 109i8, 97i8, 116i8, 95i8, 115i8, 112i8, 101i8,
    99i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 67i8, 111i8, 110i8, 118i8, 101i8,
    114i8, 116i8, 32i8, 116i8, 111i8, 32i8, 97i8, 32i8, 115i8, 116i8, 114i8, 105i8, 110i8, 103i8,
    32i8, 97i8, 99i8, 99i8, 111i8, 114i8, 100i8, 105i8, 110i8, 103i8, 32i8, 116i8, 111i8, 32i8,
    102i8, 111i8, 114i8, 109i8, 97i8, 116i8, 95i8, 115i8, 112i8, 101i8, 99i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.longobject.int___format__")]
extern "C-unwind" fn int___format__(self_: *mut _object, arg: *mut _object) -> *mut _object {
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
            return_value = int___format___impl(self_, format_spec);
        }
        return return_value;
    }
}

static int___round____doc__: __SlateAlign16<[i8; 135]> = __SlateAlign16([
    95i8, 95i8, 114i8, 111i8, 117i8, 110i8, 100i8, 95i8, 95i8, 40i8, 36i8, 115i8, 101i8, 108i8,
    102i8, 44i8, 32i8, 110i8, 100i8, 105i8, 103i8, 105i8, 116i8, 115i8, 61i8, 78i8, 111i8, 110i8,
    101i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 111i8, 117i8, 110i8, 100i8,
    105i8, 110i8, 103i8, 32i8, 97i8, 110i8, 32i8, 73i8, 110i8, 116i8, 101i8, 103i8, 114i8, 97i8,
    108i8, 32i8, 114i8, 101i8, 116i8, 117i8, 114i8, 110i8, 115i8, 32i8, 105i8, 116i8, 115i8, 101i8,
    108i8, 102i8, 46i8, 10i8, 10i8, 82i8, 111i8, 117i8, 110i8, 100i8, 105i8, 110i8, 103i8, 32i8,
    119i8, 105i8, 116i8, 104i8, 32i8, 97i8, 110i8, 32i8, 110i8, 100i8, 105i8, 103i8, 105i8, 116i8,
    115i8, 32i8, 97i8, 114i8, 103i8, 117i8, 109i8, 101i8, 110i8, 116i8, 32i8, 97i8, 108i8, 115i8,
    111i8, 32i8, 114i8, 101i8, 116i8, 117i8, 114i8, 110i8, 115i8, 32i8, 97i8, 110i8, 32i8, 105i8,
    110i8, 116i8, 101i8, 103i8, 101i8, 114i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.longobject.int___round__")]
extern "C-unwind" fn int___round__(
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
            return_value = int___round___impl(self_, o_ndigits);
        }
        return return_value;
    }
}

static int___sizeof____doc__: __SlateAlign16<[i8; 59]> = __SlateAlign16([
    95i8, 95i8, 115i8, 105i8, 122i8, 101i8, 111i8, 102i8, 95i8, 95i8, 40i8, 36i8, 115i8, 101i8,
    108i8, 102i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8,
    114i8, 110i8, 115i8, 32i8, 115i8, 105i8, 122i8, 101i8, 32i8, 105i8, 110i8, 32i8, 109i8, 101i8,
    109i8, 111i8, 114i8, 121i8, 44i8, 32i8, 105i8, 110i8, 32i8, 98i8, 121i8, 116i8, 101i8, 115i8,
    46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.longobject.int___sizeof__")]
extern "C-unwind" fn int___sizeof__(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut _return_value: i64 = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        _return_value = int___sizeof___impl(self_);
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

static int_bit_length__doc__: __SlateAlign16<[i8; 127]> = __SlateAlign16([
    98i8, 105i8, 116i8, 95i8, 108i8, 101i8, 110i8, 103i8, 116i8, 104i8, 40i8, 36i8, 115i8, 101i8,
    108i8, 102i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 78i8, 117i8, 109i8, 98i8,
    101i8, 114i8, 32i8, 111i8, 102i8, 32i8, 98i8, 105i8, 116i8, 115i8, 32i8, 110i8, 101i8, 99i8,
    101i8, 115i8, 115i8, 97i8, 114i8, 121i8, 32i8, 116i8, 111i8, 32i8, 114i8, 101i8, 112i8, 114i8,
    101i8, 115i8, 101i8, 110i8, 116i8, 32i8, 115i8, 101i8, 108i8, 102i8, 32i8, 105i8, 110i8, 32i8,
    98i8, 105i8, 110i8, 97i8, 114i8, 121i8, 46i8, 10i8, 10i8, 62i8, 62i8, 62i8, 32i8, 98i8, 105i8,
    110i8, 40i8, 51i8, 55i8, 41i8, 10i8, 39i8, 48i8, 98i8, 49i8, 48i8, 48i8, 49i8, 48i8, 49i8,
    39i8, 10i8, 62i8, 62i8, 62i8, 32i8, 40i8, 51i8, 55i8, 41i8, 46i8, 98i8, 105i8, 116i8, 95i8,
    108i8, 101i8, 110i8, 103i8, 116i8, 104i8, 40i8, 41i8, 10i8, 54i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.longobject.int_bit_length")]
extern "C-unwind" fn int_bit_length(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    int_bit_length_impl(self_)
}

static int_bit_count__doc__: __SlateAlign16<[i8; 181]> = __SlateAlign16([
    98i8, 105i8, 116i8, 95i8, 99i8, 111i8, 117i8, 110i8, 116i8, 40i8, 36i8, 115i8, 101i8, 108i8,
    102i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 78i8, 117i8, 109i8, 98i8, 101i8,
    114i8, 32i8, 111i8, 102i8, 32i8, 111i8, 110i8, 101i8, 115i8, 32i8, 105i8, 110i8, 32i8, 116i8,
    104i8, 101i8, 32i8, 98i8, 105i8, 110i8, 97i8, 114i8, 121i8, 32i8, 114i8, 101i8, 112i8, 114i8,
    101i8, 115i8, 101i8, 110i8, 116i8, 97i8, 116i8, 105i8, 111i8, 110i8, 32i8, 111i8, 102i8, 32i8,
    116i8, 104i8, 101i8, 32i8, 97i8, 98i8, 115i8, 111i8, 108i8, 117i8, 116i8, 101i8, 32i8, 118i8,
    97i8, 108i8, 117i8, 101i8, 32i8, 111i8, 102i8, 32i8, 115i8, 101i8, 108i8, 102i8, 46i8, 10i8,
    10i8, 65i8, 108i8, 115i8, 111i8, 32i8, 107i8, 110i8, 111i8, 119i8, 110i8, 32i8, 97i8, 115i8,
    32i8, 116i8, 104i8, 101i8, 32i8, 112i8, 111i8, 112i8, 117i8, 108i8, 97i8, 116i8, 105i8, 111i8,
    110i8, 32i8, 99i8, 111i8, 117i8, 110i8, 116i8, 46i8, 10i8, 10i8, 62i8, 62i8, 62i8, 32i8, 98i8,
    105i8, 110i8, 40i8, 49i8, 51i8, 41i8, 10i8, 39i8, 48i8, 98i8, 49i8, 49i8, 48i8, 49i8, 39i8,
    10i8, 62i8, 62i8, 62i8, 32i8, 40i8, 49i8, 51i8, 41i8, 46i8, 98i8, 105i8, 116i8, 95i8, 99i8,
    111i8, 117i8, 110i8, 116i8, 40i8, 41i8, 10i8, 51i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.longobject.int_bit_count")]
extern "C-unwind" fn int_bit_count(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    int_bit_count_impl(self_)
}

static int_as_integer_ratio__doc__: __SlateAlign16<[i8; 271]> = __SlateAlign16([
    97i8, 115i8, 95i8, 105i8, 110i8, 116i8, 101i8, 103i8, 101i8, 114i8, 95i8, 114i8, 97i8, 116i8,
    105i8, 111i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8,
    10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 97i8, 32i8, 112i8, 97i8, 105i8,
    114i8, 32i8, 111i8, 102i8, 32i8, 105i8, 110i8, 116i8, 101i8, 103i8, 101i8, 114i8, 115i8, 44i8,
    32i8, 119i8, 104i8, 111i8, 115i8, 101i8, 32i8, 114i8, 97i8, 116i8, 105i8, 111i8, 32i8, 105i8,
    115i8, 32i8, 101i8, 113i8, 117i8, 97i8, 108i8, 32i8, 116i8, 111i8, 32i8, 116i8, 104i8, 101i8,
    32i8, 111i8, 114i8, 105i8, 103i8, 105i8, 110i8, 97i8, 108i8, 32i8, 105i8, 110i8, 116i8, 46i8,
    10i8, 10i8, 84i8, 104i8, 101i8, 32i8, 114i8, 97i8, 116i8, 105i8, 111i8, 32i8, 105i8, 115i8,
    32i8, 105i8, 110i8, 32i8, 108i8, 111i8, 119i8, 101i8, 115i8, 116i8, 32i8, 116i8, 101i8, 114i8,
    109i8, 115i8, 32i8, 97i8, 110i8, 100i8, 32i8, 104i8, 97i8, 115i8, 32i8, 97i8, 32i8, 112i8,
    111i8, 115i8, 105i8, 116i8, 105i8, 118i8, 101i8, 32i8, 100i8, 101i8, 110i8, 111i8, 109i8,
    105i8, 110i8, 97i8, 116i8, 111i8, 114i8, 46i8, 10i8, 10i8, 62i8, 62i8, 62i8, 32i8, 40i8, 49i8,
    48i8, 41i8, 46i8, 97i8, 115i8, 95i8, 105i8, 110i8, 116i8, 101i8, 103i8, 101i8, 114i8, 95i8,
    114i8, 97i8, 116i8, 105i8, 111i8, 40i8, 41i8, 10i8, 40i8, 49i8, 48i8, 44i8, 32i8, 49i8, 41i8,
    10i8, 62i8, 62i8, 62i8, 32i8, 40i8, 45i8, 49i8, 48i8, 41i8, 46i8, 97i8, 115i8, 95i8, 105i8,
    110i8, 116i8, 101i8, 103i8, 101i8, 114i8, 95i8, 114i8, 97i8, 116i8, 105i8, 111i8, 40i8, 41i8,
    10i8, 40i8, 45i8, 49i8, 48i8, 44i8, 32i8, 49i8, 41i8, 10i8, 62i8, 62i8, 62i8, 32i8, 40i8, 48i8,
    41i8, 46i8, 97i8, 115i8, 95i8, 105i8, 110i8, 116i8, 101i8, 103i8, 101i8, 114i8, 95i8, 114i8,
    97i8, 116i8, 105i8, 111i8, 40i8, 41i8, 10i8, 40i8, 48i8, 44i8, 32i8, 49i8, 41i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.longobject.int_as_integer_ratio")]
extern "C-unwind" fn int_as_integer_ratio(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    int_as_integer_ratio_impl(self_)
}

static int_to_bytes__doc__: __SlateAlign16<[i8; 838]> = __SlateAlign16([
    116i8, 111i8, 95i8, 98i8, 121i8, 116i8, 101i8, 115i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8,
    44i8, 32i8, 47i8, 44i8, 32i8, 108i8, 101i8, 110i8, 103i8, 116i8, 104i8, 61i8, 49i8, 44i8, 32i8,
    98i8, 121i8, 116i8, 101i8, 111i8, 114i8, 100i8, 101i8, 114i8, 61i8, 39i8, 98i8, 105i8, 103i8,
    39i8, 44i8, 32i8, 42i8, 44i8, 32i8, 115i8, 105i8, 103i8, 110i8, 101i8, 100i8, 61i8, 70i8, 97i8,
    108i8, 115i8, 101i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8,
    110i8, 32i8, 97i8, 110i8, 32i8, 97i8, 114i8, 114i8, 97i8, 121i8, 32i8, 111i8, 102i8, 32i8,
    98i8, 121i8, 116i8, 101i8, 115i8, 32i8, 114i8, 101i8, 112i8, 114i8, 101i8, 115i8, 101i8, 110i8,
    116i8, 105i8, 110i8, 103i8, 32i8, 97i8, 110i8, 32i8, 105i8, 110i8, 116i8, 101i8, 103i8, 101i8,
    114i8, 46i8, 10i8, 10i8, 32i8, 32i8, 108i8, 101i8, 110i8, 103i8, 116i8, 104i8, 10i8, 32i8,
    32i8, 32i8, 32i8, 76i8, 101i8, 110i8, 103i8, 116i8, 104i8, 32i8, 111i8, 102i8, 32i8, 98i8,
    121i8, 116i8, 101i8, 115i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 32i8, 116i8, 111i8,
    32i8, 117i8, 115i8, 101i8, 46i8, 32i8, 32i8, 65i8, 110i8, 32i8, 79i8, 118i8, 101i8, 114i8,
    102i8, 108i8, 111i8, 119i8, 69i8, 114i8, 114i8, 111i8, 114i8, 32i8, 105i8, 115i8, 32i8, 114i8,
    97i8, 105i8, 115i8, 101i8, 100i8, 32i8, 105i8, 102i8, 10i8, 32i8, 32i8, 32i8, 32i8, 116i8,
    104i8, 101i8, 32i8, 105i8, 110i8, 116i8, 101i8, 103i8, 101i8, 114i8, 32i8, 105i8, 115i8, 32i8,
    110i8, 111i8, 116i8, 32i8, 114i8, 101i8, 112i8, 114i8, 101i8, 115i8, 101i8, 110i8, 116i8, 97i8,
    98i8, 108i8, 101i8, 32i8, 119i8, 105i8, 116i8, 104i8, 32i8, 116i8, 104i8, 101i8, 32i8, 103i8,
    105i8, 118i8, 101i8, 110i8, 32i8, 110i8, 117i8, 109i8, 98i8, 101i8, 114i8, 32i8, 111i8, 102i8,
    32i8, 98i8, 121i8, 116i8, 101i8, 115i8, 46i8, 10i8, 32i8, 32i8, 32i8, 32i8, 68i8, 101i8, 102i8,
    97i8, 117i8, 108i8, 116i8, 32i8, 105i8, 115i8, 32i8, 108i8, 101i8, 110i8, 103i8, 116i8, 104i8,
    32i8, 49i8, 46i8, 10i8, 32i8, 32i8, 98i8, 121i8, 116i8, 101i8, 111i8, 114i8, 100i8, 101i8,
    114i8, 10i8, 32i8, 32i8, 32i8, 32i8, 84i8, 104i8, 101i8, 32i8, 98i8, 121i8, 116i8, 101i8, 32i8,
    111i8, 114i8, 100i8, 101i8, 114i8, 32i8, 117i8, 115i8, 101i8, 100i8, 32i8, 116i8, 111i8, 32i8,
    114i8, 101i8, 112i8, 114i8, 101i8, 115i8, 101i8, 110i8, 116i8, 32i8, 116i8, 104i8, 101i8, 32i8,
    105i8, 110i8, 116i8, 101i8, 103i8, 101i8, 114i8, 46i8, 32i8, 32i8, 73i8, 102i8, 32i8, 98i8,
    121i8, 116i8, 101i8, 111i8, 114i8, 100i8, 101i8, 114i8, 32i8, 105i8, 115i8, 10i8, 32i8, 32i8,
    32i8, 32i8, 39i8, 98i8, 105i8, 103i8, 39i8, 44i8, 32i8, 116i8, 104i8, 101i8, 32i8, 109i8,
    111i8, 115i8, 116i8, 32i8, 115i8, 105i8, 103i8, 110i8, 105i8, 102i8, 105i8, 99i8, 97i8, 110i8,
    116i8, 32i8, 98i8, 121i8, 116i8, 101i8, 32i8, 105i8, 115i8, 32i8, 97i8, 116i8, 32i8, 116i8,
    104i8, 101i8, 32i8, 98i8, 101i8, 103i8, 105i8, 110i8, 110i8, 105i8, 110i8, 103i8, 32i8, 111i8,
    102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 98i8, 121i8, 116i8, 101i8, 10i8, 32i8, 32i8, 32i8,
    32i8, 97i8, 114i8, 114i8, 97i8, 121i8, 46i8, 32i8, 32i8, 73i8, 102i8, 32i8, 98i8, 121i8, 116i8,
    101i8, 111i8, 114i8, 100i8, 101i8, 114i8, 32i8, 105i8, 115i8, 32i8, 39i8, 108i8, 105i8, 116i8,
    116i8, 108i8, 101i8, 39i8, 44i8, 32i8, 116i8, 104i8, 101i8, 32i8, 109i8, 111i8, 115i8, 116i8,
    32i8, 115i8, 105i8, 103i8, 110i8, 105i8, 102i8, 105i8, 99i8, 97i8, 110i8, 116i8, 32i8, 98i8,
    121i8, 116i8, 101i8, 32i8, 105i8, 115i8, 32i8, 97i8, 116i8, 10i8, 32i8, 32i8, 32i8, 32i8,
    116i8, 104i8, 101i8, 32i8, 101i8, 110i8, 100i8, 32i8, 111i8, 102i8, 32i8, 116i8, 104i8, 101i8,
    32i8, 98i8, 121i8, 116i8, 101i8, 32i8, 97i8, 114i8, 114i8, 97i8, 121i8, 46i8, 32i8, 32i8, 84i8,
    111i8, 32i8, 114i8, 101i8, 113i8, 117i8, 101i8, 115i8, 116i8, 32i8, 116i8, 104i8, 101i8, 32i8,
    110i8, 97i8, 116i8, 105i8, 118i8, 101i8, 32i8, 98i8, 121i8, 116i8, 101i8, 32i8, 111i8, 114i8,
    100i8, 101i8, 114i8, 32i8, 111i8, 102i8, 10i8, 32i8, 32i8, 32i8, 32i8, 116i8, 104i8, 101i8,
    32i8, 104i8, 111i8, 115i8, 116i8, 32i8, 115i8, 121i8, 115i8, 116i8, 101i8, 109i8, 44i8, 32i8,
    117i8, 115i8, 101i8, 32i8, 115i8, 121i8, 115i8, 46i8, 98i8, 121i8, 116i8, 101i8, 111i8, 114i8,
    100i8, 101i8, 114i8, 32i8, 97i8, 115i8, 32i8, 116i8, 104i8, 101i8, 32i8, 98i8, 121i8, 116i8,
    101i8, 32i8, 111i8, 114i8, 100i8, 101i8, 114i8, 32i8, 118i8, 97i8, 108i8, 117i8, 101i8, 46i8,
    10i8, 32i8, 32i8, 32i8, 32i8, 68i8, 101i8, 102i8, 97i8, 117i8, 108i8, 116i8, 32i8, 105i8,
    115i8, 32i8, 116i8, 111i8, 32i8, 117i8, 115i8, 101i8, 32i8, 39i8, 98i8, 105i8, 103i8, 39i8,
    46i8, 10i8, 32i8, 32i8, 115i8, 105i8, 103i8, 110i8, 101i8, 100i8, 10i8, 32i8, 32i8, 32i8, 32i8,
    68i8, 101i8, 116i8, 101i8, 114i8, 109i8, 105i8, 110i8, 101i8, 115i8, 32i8, 119i8, 104i8, 101i8,
    116i8, 104i8, 101i8, 114i8, 32i8, 116i8, 119i8, 111i8, 39i8, 115i8, 32i8, 99i8, 111i8, 109i8,
    112i8, 108i8, 101i8, 109i8, 101i8, 110i8, 116i8, 32i8, 105i8, 115i8, 32i8, 117i8, 115i8, 101i8,
    100i8, 32i8, 116i8, 111i8, 32i8, 114i8, 101i8, 112i8, 114i8, 101i8, 115i8, 101i8, 110i8, 116i8,
    32i8, 116i8, 104i8, 101i8, 10i8, 32i8, 32i8, 32i8, 32i8, 105i8, 110i8, 116i8, 101i8, 103i8,
    101i8, 114i8, 46i8, 32i8, 32i8, 73i8, 102i8, 32i8, 115i8, 105i8, 103i8, 110i8, 101i8, 100i8,
    32i8, 105i8, 115i8, 32i8, 70i8, 97i8, 108i8, 115i8, 101i8, 32i8, 97i8, 110i8, 100i8, 32i8,
    97i8, 32i8, 110i8, 101i8, 103i8, 97i8, 116i8, 105i8, 118i8, 101i8, 32i8, 105i8, 110i8, 116i8,
    101i8, 103i8, 101i8, 114i8, 32i8, 105i8, 115i8, 32i8, 103i8, 105i8, 118i8, 101i8, 110i8, 44i8,
    10i8, 32i8, 32i8, 32i8, 32i8, 97i8, 110i8, 32i8, 79i8, 118i8, 101i8, 114i8, 102i8, 108i8,
    111i8, 119i8, 69i8, 114i8, 114i8, 111i8, 114i8, 32i8, 105i8, 115i8, 32i8, 114i8, 97i8, 105i8,
    115i8, 101i8, 100i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.longobject.int_to_bytes")]
extern "C-unwind" fn int_to_bytes(
    self_: *mut _object,
    mut args: *const *mut _object,
    nargs: i64,
    kwnames: *mut _object,
) -> *mut _object {
    let mut __v4: bool = unsafe { core::mem::zeroed() };
    let mut iobj: *mut _object = unsafe { core::mem::zeroed() };
    let mut ival: i64 = unsafe { core::mem::zeroed() };
    let mut is_signed: i32 = unsafe { core::mem::zeroed() };
    let mut byteorder: *mut _object = unsafe { core::mem::zeroed() };
    let mut length: i64 = unsafe { core::mem::zeroed() };
    let mut __v10: i64 = unsafe { core::mem::zeroed() };
    let mut noptargs: i64 = unsafe { core::mem::zeroed() };
    let mut argsbuf: core::mem::MaybeUninit<__SlateAlign16<[*mut _object; 3]>> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_12: *mut [*mut _object; 3] =
        core::ptr::addr_of_mut!(argsbuf) as *mut [*mut _object; 3];
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        if !kwnames.is_null() {
            __v10 = PyTuple_GET_SIZE!(kwnames);
        } else {
            __v10 = 0;
        }
        '__join_0: {
            noptargs = nargs.wrapping_add(__v10).wrapping_sub(0);
            length = 1;
            byteorder = core::ptr::null_mut::<_object>();
            is_signed = 0;
            args = _PyArg_UnpackKeywords!(
                args,
                nargs,
                core::ptr::null_mut::<()>(),
                kwnames,
                core::ptr::addr_of_mut!(_parser__312),
                0i32,
                2i32,
                0i32,
                0i32,
                (*__slate_slot_12).as_mut_ptr() as *mut *mut _object
            );
            // minpos
            // maxpos
            // minkw
            // varpos
            if !args.is_null() {
                '__join_3: {
                    if noptargs != 0 {
                        if !unsafe { *args.wrapping_offset(0 as isize) }.is_null() {
                            ival = 1i32.wrapping_neg() as i64;
                            iobj = _PyNumber_Index(*args.wrapping_offset(0 as isize));
                            if !iobj.is_null() {
                                ival = PyLong_AsSsize_t(iobj);
                                Py_DECREF!(iobj);
                            }
                            if ival == (1i32.wrapping_neg() as i64) {
                                __v4 = !PyErr_Occurred().is_null();
                            } else {
                                __v4 = false;
                            }
                            if __v4 {
                                break '__join_0;
                            } else {
                                length = ival;
                                let __v2: i64 = noptargs.wrapping_sub(1);
                                noptargs = __v2;
                                if __v2 == 0 {
                                    break '__join_3;
                                }
                            }
                        }
                        if !unsafe { *args.wrapping_offset(1 as isize) }.is_null() {
                            if PyUnicode_Check!(unsafe { *args.wrapping_offset(1 as isize) }) == 0 {
                                _PyArg_BadArgument(
                                    c"to_bytes".as_ptr(),
                                    c"argument 'byteorder'".as_ptr(),
                                    c"str".as_ptr(),
                                    *args.wrapping_offset(1 as isize),
                                );
                                break '__join_0;
                            } else {
                                byteorder = *args.wrapping_offset(1 as isize);
                                noptargs = noptargs.wrapping_sub(1);
                            }
                        }
                    }
                }
                if noptargs != 0 {
                    is_signed = PyObject_IsTrue(*args.wrapping_offset(2 as isize));
                    if is_signed < 0 {
                        break '__join_0;
                    }
                }
                return_value = int_to_bytes_impl(self_, length, byteorder, is_signed);
            }
        }
        return return_value;
    }
}

static mut _kwtuple__299: __SlateRecord_longobject_c_h_299 = {
    let mut __t0: __SlateRecord_longobject_c_h_299 = unsafe { core::mem::zeroed() };
    __t0.ob_base = __SlateRecord_object_h_172 {
        ob_base: _object {
            __slate_anon_0: __SlateRecord_object_h_122 {
                ob_refcnt_full: 1407378104778752i64,
            },
            ob_type: unsafe { core::ptr::addr_of_mut!(PyTuple_Type) },
        },
        ob_size: 3i64,
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
                    ._py_length
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
                    ._py_byteorder
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
                    ._py_signed
                    ._ascii
                    .ob_base
            )
        },
    ];
    __t0
};

static _keywords__311: __SlateAlign16<[*const i8; 4]> = __SlateAlign16([
    (b"length\0".as_ptr() as *mut i8) as *const i8,
    (b"byteorder\0".as_ptr() as *mut i8) as *const i8,
    (b"signed\0".as_ptr() as *mut i8) as *const i8,
    core::ptr::null::<i8>(),
]);

static mut _parser__312: _PyArg_Parser = {
    let mut __t0: _PyArg_Parser = unsafe { core::mem::zeroed() };
    __t0.keywords = unsafe { core::ptr::addr_of!(_keywords__311.0) as *const *const i8 };
    __t0.fname = (b"to_bytes\0".as_ptr() as *mut i8) as *const i8;
    __t0.kwtuple = unsafe { core::ptr::addr_of_mut!(_kwtuple__299.ob_base.ob_base) };
    __t0
};

static int_from_bytes__doc__: __SlateAlign16<[i8; 837]> = __SlateAlign16([
    102i8, 114i8, 111i8, 109i8, 95i8, 98i8, 121i8, 116i8, 101i8, 115i8, 40i8, 36i8, 116i8, 121i8,
    112i8, 101i8, 44i8, 32i8, 47i8, 44i8, 32i8, 98i8, 121i8, 116i8, 101i8, 115i8, 44i8, 32i8, 98i8,
    121i8, 116i8, 101i8, 111i8, 114i8, 100i8, 101i8, 114i8, 61i8, 39i8, 98i8, 105i8, 103i8, 39i8,
    44i8, 32i8, 42i8, 44i8, 32i8, 115i8, 105i8, 103i8, 110i8, 101i8, 100i8, 61i8, 70i8, 97i8,
    108i8, 115i8, 101i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8,
    110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 105i8, 110i8, 116i8, 101i8, 103i8, 101i8, 114i8, 32i8,
    114i8, 101i8, 112i8, 114i8, 101i8, 115i8, 101i8, 110i8, 116i8, 101i8, 100i8, 32i8, 98i8, 121i8,
    32i8, 116i8, 104i8, 101i8, 32i8, 103i8, 105i8, 118i8, 101i8, 110i8, 32i8, 97i8, 114i8, 114i8,
    97i8, 121i8, 32i8, 111i8, 102i8, 32i8, 98i8, 121i8, 116i8, 101i8, 115i8, 46i8, 10i8, 10i8,
    32i8, 32i8, 98i8, 121i8, 116i8, 101i8, 115i8, 10i8, 32i8, 32i8, 32i8, 32i8, 72i8, 111i8, 108i8,
    100i8, 115i8, 32i8, 116i8, 104i8, 101i8, 32i8, 97i8, 114i8, 114i8, 97i8, 121i8, 32i8, 111i8,
    102i8, 32i8, 98i8, 121i8, 116i8, 101i8, 115i8, 32i8, 116i8, 111i8, 32i8, 99i8, 111i8, 110i8,
    118i8, 101i8, 114i8, 116i8, 46i8, 32i8, 32i8, 84i8, 104i8, 101i8, 32i8, 97i8, 114i8, 103i8,
    117i8, 109i8, 101i8, 110i8, 116i8, 32i8, 109i8, 117i8, 115i8, 116i8, 32i8, 101i8, 105i8, 116i8,
    104i8, 101i8, 114i8, 10i8, 32i8, 32i8, 32i8, 32i8, 115i8, 117i8, 112i8, 112i8, 111i8, 114i8,
    116i8, 32i8, 116i8, 104i8, 101i8, 32i8, 98i8, 117i8, 102i8, 102i8, 101i8, 114i8, 32i8, 112i8,
    114i8, 111i8, 116i8, 111i8, 99i8, 111i8, 108i8, 32i8, 111i8, 114i8, 32i8, 98i8, 101i8, 32i8,
    97i8, 110i8, 32i8, 105i8, 116i8, 101i8, 114i8, 97i8, 98i8, 108i8, 101i8, 32i8, 111i8, 98i8,
    106i8, 101i8, 99i8, 116i8, 32i8, 112i8, 114i8, 111i8, 100i8, 117i8, 99i8, 105i8, 110i8, 103i8,
    10i8, 32i8, 32i8, 32i8, 32i8, 98i8, 121i8, 116i8, 101i8, 115i8, 46i8, 32i8, 32i8, 66i8, 121i8,
    116i8, 101i8, 115i8, 32i8, 97i8, 110i8, 100i8, 32i8, 98i8, 121i8, 116i8, 101i8, 97i8, 114i8,
    114i8, 97i8, 121i8, 32i8, 97i8, 114i8, 101i8, 32i8, 101i8, 120i8, 97i8, 109i8, 112i8, 108i8,
    101i8, 115i8, 32i8, 111i8, 102i8, 32i8, 98i8, 117i8, 105i8, 108i8, 116i8, 45i8, 105i8, 110i8,
    32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 115i8, 32i8, 116i8, 104i8, 97i8, 116i8, 10i8,
    32i8, 32i8, 32i8, 32i8, 115i8, 117i8, 112i8, 112i8, 111i8, 114i8, 116i8, 32i8, 116i8, 104i8,
    101i8, 32i8, 98i8, 117i8, 102i8, 102i8, 101i8, 114i8, 32i8, 112i8, 114i8, 111i8, 116i8, 111i8,
    99i8, 111i8, 108i8, 46i8, 10i8, 32i8, 32i8, 98i8, 121i8, 116i8, 101i8, 111i8, 114i8, 100i8,
    101i8, 114i8, 10i8, 32i8, 32i8, 32i8, 32i8, 84i8, 104i8, 101i8, 32i8, 98i8, 121i8, 116i8,
    101i8, 32i8, 111i8, 114i8, 100i8, 101i8, 114i8, 32i8, 117i8, 115i8, 101i8, 100i8, 32i8, 116i8,
    111i8, 32i8, 114i8, 101i8, 112i8, 114i8, 101i8, 115i8, 101i8, 110i8, 116i8, 32i8, 116i8, 104i8,
    101i8, 32i8, 105i8, 110i8, 116i8, 101i8, 103i8, 101i8, 114i8, 46i8, 32i8, 32i8, 73i8, 102i8,
    32i8, 98i8, 121i8, 116i8, 101i8, 111i8, 114i8, 100i8, 101i8, 114i8, 32i8, 105i8, 115i8, 10i8,
    32i8, 32i8, 32i8, 32i8, 39i8, 98i8, 105i8, 103i8, 39i8, 44i8, 32i8, 116i8, 104i8, 101i8, 32i8,
    109i8, 111i8, 115i8, 116i8, 32i8, 115i8, 105i8, 103i8, 110i8, 105i8, 102i8, 105i8, 99i8, 97i8,
    110i8, 116i8, 32i8, 98i8, 121i8, 116i8, 101i8, 32i8, 105i8, 115i8, 32i8, 97i8, 116i8, 32i8,
    116i8, 104i8, 101i8, 32i8, 98i8, 101i8, 103i8, 105i8, 110i8, 110i8, 105i8, 110i8, 103i8, 32i8,
    111i8, 102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 98i8, 121i8, 116i8, 101i8, 10i8, 32i8, 32i8,
    32i8, 32i8, 97i8, 114i8, 114i8, 97i8, 121i8, 46i8, 32i8, 32i8, 73i8, 102i8, 32i8, 98i8, 121i8,
    116i8, 101i8, 111i8, 114i8, 100i8, 101i8, 114i8, 32i8, 105i8, 115i8, 32i8, 39i8, 108i8, 105i8,
    116i8, 116i8, 108i8, 101i8, 39i8, 44i8, 32i8, 116i8, 104i8, 101i8, 32i8, 109i8, 111i8, 115i8,
    116i8, 32i8, 115i8, 105i8, 103i8, 110i8, 105i8, 102i8, 105i8, 99i8, 97i8, 110i8, 116i8, 32i8,
    98i8, 121i8, 116i8, 101i8, 32i8, 105i8, 115i8, 32i8, 97i8, 116i8, 10i8, 32i8, 32i8, 32i8, 32i8,
    116i8, 104i8, 101i8, 32i8, 101i8, 110i8, 100i8, 32i8, 111i8, 102i8, 32i8, 116i8, 104i8, 101i8,
    32i8, 98i8, 121i8, 116i8, 101i8, 32i8, 97i8, 114i8, 114i8, 97i8, 121i8, 46i8, 32i8, 32i8, 84i8,
    111i8, 32i8, 114i8, 101i8, 113i8, 117i8, 101i8, 115i8, 116i8, 32i8, 116i8, 104i8, 101i8, 32i8,
    110i8, 97i8, 116i8, 105i8, 118i8, 101i8, 32i8, 98i8, 121i8, 116i8, 101i8, 32i8, 111i8, 114i8,
    100i8, 101i8, 114i8, 32i8, 111i8, 102i8, 10i8, 32i8, 32i8, 32i8, 32i8, 116i8, 104i8, 101i8,
    32i8, 104i8, 111i8, 115i8, 116i8, 32i8, 115i8, 121i8, 115i8, 116i8, 101i8, 109i8, 44i8, 32i8,
    117i8, 115i8, 101i8, 32i8, 115i8, 121i8, 115i8, 46i8, 98i8, 121i8, 116i8, 101i8, 111i8, 114i8,
    100i8, 101i8, 114i8, 32i8, 97i8, 115i8, 32i8, 116i8, 104i8, 101i8, 32i8, 98i8, 121i8, 116i8,
    101i8, 32i8, 111i8, 114i8, 100i8, 101i8, 114i8, 32i8, 118i8, 97i8, 108i8, 117i8, 101i8, 46i8,
    10i8, 32i8, 32i8, 32i8, 32i8, 68i8, 101i8, 102i8, 97i8, 117i8, 108i8, 116i8, 32i8, 105i8,
    115i8, 32i8, 116i8, 111i8, 32i8, 117i8, 115i8, 101i8, 32i8, 39i8, 98i8, 105i8, 103i8, 39i8,
    46i8, 10i8, 32i8, 32i8, 115i8, 105i8, 103i8, 110i8, 101i8, 100i8, 10i8, 32i8, 32i8, 32i8, 32i8,
    73i8, 110i8, 100i8, 105i8, 99i8, 97i8, 116i8, 101i8, 115i8, 32i8, 119i8, 104i8, 101i8, 116i8,
    104i8, 101i8, 114i8, 32i8, 116i8, 119i8, 111i8, 39i8, 115i8, 32i8, 99i8, 111i8, 109i8, 112i8,
    108i8, 101i8, 109i8, 101i8, 110i8, 116i8, 32i8, 105i8, 115i8, 32i8, 117i8, 115i8, 101i8, 100i8,
    32i8, 116i8, 111i8, 32i8, 114i8, 101i8, 112i8, 114i8, 101i8, 115i8, 101i8, 110i8, 116i8, 32i8,
    116i8, 104i8, 101i8, 10i8, 32i8, 32i8, 32i8, 32i8, 105i8, 110i8, 116i8, 101i8, 103i8, 101i8,
    114i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.longobject.int_from_bytes")]
extern "C-unwind" fn int_from_bytes(
    r#type: *mut _object,
    mut args: *const *mut _object,
    nargs: i64,
    kwnames: *mut _object,
) -> *mut _object {
    let mut is_signed: i32 = unsafe { core::mem::zeroed() };
    let mut byteorder: *mut _object = unsafe { core::mem::zeroed() };
    let mut bytes_obj: *mut _object = unsafe { core::mem::zeroed() };
    let mut __v5: i64 = unsafe { core::mem::zeroed() };
    let mut noptargs: i64 = unsafe { core::mem::zeroed() };
    let mut argsbuf: core::mem::MaybeUninit<__SlateAlign16<[*mut _object; 3]>> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_7: *mut [*mut _object; 3] =
        core::ptr::addr_of_mut!(argsbuf) as *mut [*mut _object; 3];
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        if !kwnames.is_null() {
            __v5 = PyTuple_GET_SIZE!(kwnames);
        } else {
            __v5 = 0;
        }
        '__join_0: {
            noptargs = nargs.wrapping_add(__v5).wrapping_sub(1);
            byteorder = core::ptr::null_mut::<_object>();
            is_signed = 0;
            args = _PyArg_UnpackKeywords!(
                args,
                nargs,
                core::ptr::null_mut::<()>(),
                kwnames,
                core::ptr::addr_of_mut!(_parser__428),
                1i32,
                2i32,
                0i32,
                0i32,
                (*__slate_slot_7).as_mut_ptr() as *mut *mut _object
            );
            // minpos
            // maxpos
            // minkw
            // varpos
            if !args.is_null() {
                bytes_obj = *args.wrapping_offset(0 as isize);
                if noptargs != 0 {
                    if !unsafe { *args.wrapping_offset(1 as isize) }.is_null() {
                        if PyUnicode_Check!(unsafe { *args.wrapping_offset(1 as isize) }) == 0 {
                            _PyArg_BadArgument(
                                c"from_bytes".as_ptr(),
                                c"argument 'byteorder'".as_ptr(),
                                c"str".as_ptr(),
                                *args.wrapping_offset(1 as isize),
                            );
                            break '__join_0;
                        } else {
                            byteorder = *args.wrapping_offset(1 as isize);
                            noptargs = noptargs.wrapping_sub(1);
                        }
                    }
                }
                if noptargs != 0 {
                    is_signed = PyObject_IsTrue(*args.wrapping_offset(2 as isize));
                    if is_signed < 0 {
                        break '__join_0;
                    }
                }
                return_value = int_from_bytes_impl(
                    r#type as *mut _typeobject,
                    bytes_obj,
                    byteorder,
                    is_signed,
                );
            }
        }
        return return_value;
    }
}

static mut _kwtuple__415: __SlateRecord_longobject_c_h_415 = {
    let mut __t0: __SlateRecord_longobject_c_h_415 = unsafe { core::mem::zeroed() };
    __t0.ob_base = __SlateRecord_object_h_172 {
        ob_base: _object {
            __slate_anon_0: __SlateRecord_object_h_122 {
                ob_refcnt_full: 1407378104778752i64,
            },
            ob_type: unsafe { core::ptr::addr_of_mut!(PyTuple_Type) },
        },
        ob_size: 3i64,
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
                    ._py_bytes
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
                    ._py_byteorder
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
                    ._py_signed
                    ._ascii
                    .ob_base
            )
        },
    ];
    __t0
};

static _keywords__427: __SlateAlign16<[*const i8; 4]> = __SlateAlign16([
    (b"bytes\0".as_ptr() as *mut i8) as *const i8,
    (b"byteorder\0".as_ptr() as *mut i8) as *const i8,
    (b"signed\0".as_ptr() as *mut i8) as *const i8,
    core::ptr::null::<i8>(),
]);

static mut _parser__428: _PyArg_Parser = {
    let mut __t0: _PyArg_Parser = unsafe { core::mem::zeroed() };
    __t0.keywords = unsafe { core::ptr::addr_of!(_keywords__427.0) as *const *const i8 };
    __t0.fname = (b"from_bytes\0".as_ptr() as *mut i8) as *const i8;
    __t0.kwtuple = unsafe { core::ptr::addr_of_mut!(_kwtuple__415.ob_base.ob_base) };
    __t0
};

static int_is_integer__doc__: __SlateAlign16<[i8; 97]> = __SlateAlign16([
    105i8, 115i8, 95i8, 105i8, 110i8, 116i8, 101i8, 103i8, 101i8, 114i8, 40i8, 36i8, 115i8, 101i8,
    108i8, 102i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8,
    114i8, 110i8, 115i8, 32i8, 84i8, 114i8, 117i8, 101i8, 46i8, 32i8, 69i8, 120i8, 105i8, 115i8,
    116i8, 115i8, 32i8, 102i8, 111i8, 114i8, 32i8, 100i8, 117i8, 99i8, 107i8, 32i8, 116i8, 121i8,
    112i8, 101i8, 32i8, 99i8, 111i8, 109i8, 112i8, 97i8, 116i8, 105i8, 98i8, 105i8, 108i8, 105i8,
    116i8, 121i8, 32i8, 119i8, 105i8, 116i8, 104i8, 32i8, 102i8, 108i8, 111i8, 97i8, 116i8, 46i8,
    105i8, 115i8, 95i8, 105i8, 110i8, 116i8, 101i8, 103i8, 101i8, 114i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.longobject.int_is_integer")]
extern "C-unwind" fn int_is_integer(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    int_is_integer_impl(self_)
}

// XXX The functional organization of this file is terrible
// _Py_popcount32()
// _PyStatus_OK()
// _PyObject_MakeTpCall
// _Py_FREELIST_FREE, _Py_FREELIST_POP
// _Py_SmallInts
// _PyObject_Init()
// _PY_NSMALLPOSINTS
// _PyStructSequence_FiniBuiltin()
// _PyUnicode_Equal()
// DBL_MANT_DIG
// offsetof
// [clinic input]
// class int "PyObject *" "&PyLong_Type"
// [clinic start generated code]
//
// [clinic end generated code: output=da39a3ee5e6b4b0d input=ec0275e3422a36e3]
// If defined, use algorithms from the _pylong.py module
fn _Py_DECREF_INT(op: *mut _longobject) {
    _Py_DECREF_SPECIALIZED(op as *mut _object, Some(_PyLong_ExactDealloc));
}

fn is_medium_int(x: i64) -> i32 {
    // Take care that we are comparing unsigned values.
    let x_plus_mask: u64 = (x as u64).wrapping_add(1073741823u64);
    (x_plus_mask < 2147483647u64) as i32
}

fn get_small_int(ival: i32) -> *mut _object {
    unsafe {
        core::ptr::addr_of_mut!(_PyRuntime.static_objects.singletons.small_ints) as *mut _longobject
    }
    .wrapping_offset(5i32.wrapping_add(ival) as isize) as *mut _object
}

fn maybe_small_long(v: *mut _longobject) -> *mut _longobject {
    let __v0: bool = !v.is_null() && _PyLong_IsCompact(v as *const _longobject) != 0;
    if __v0 {
        let ival: i64 = medium_value!(v);
        if IS_SMALL_INT!(ival) {
            _Py_DECREF_INT(v);
            return get_small_int(ival as i32) as *mut _longobject;
        }
    }
    v
}

// For int multiplication, use the O(N**2) school algorithm unless
// both operands contain more than KARATSUBA_CUTOFF digits (this
// being an internal Python int digit, in base BASE).
// For exponentiation, use the binary left-to-right algorithm unless the
// ^ exponent contains more than HUGE_EXP_CUTOFF bits.  In that case, do
// * (no more than) EXP_WINDOW_SIZE bits at a time.  The potential drawback is
// * that a table of 2**(EXP_WINDOW_SIZE - 1) intermediate results is
// * precomputed.
// Suppose the exponent has bit length e. All ways of doing this
// need e squarings. The binary method also needs a multiply for
// each bit set. In a k-ary method with window width w, a multiply
// for each non-zero window, so at worst (and likely!)
// ceiling(e/w). The k-ary sliding window method has the same
// worst case, but the window slides so it can sometimes skip
// over an all-zero window that the fixed-window method can't
// exploit. In addition, the windowing methods need multiplies
// to precompute a table of small powers.
//
// For the sliding window method with width 5, 16 precomputation
// multiplies are needed. Assuming about half the exponent bits
// are set, then, the binary method needs about e/2 extra mults
// and the window method about 16 + e/5.
//
// The latter is smaller for e > 53 1/3. We don't have direct
// access to the bit length, though, so call it 60, which is a
// multiple of a long digit's max bit length (15 or 30 so far).
// Normalize (remove leading zeros from) an int object.
// Doesn't attempt to free the storage--in most cases, due to the nature
// of the algorithms used, this could save at most be one word anyway.
fn long_normalize(v: *mut _longobject) -> *mut _longobject {
    let j: i64 = _PyLong_DigitCount(v as *const _longobject);
    let mut i: i64 = j;
    while i > 0
        && (unsafe {
            *((*v).long_value.ob_digit.as_mut_ptr() as *mut u32)
                .wrapping_offset(i.wrapping_sub(1) as isize)
        }) == 0
    {
        i = i.wrapping_sub(1);
    }
    if i != j {
        if i == 0 {
            _PyLong_SetSignAndDigitCount(v, 0i32, 0i64);
        } else {
            _PyLong_SetDigitCount(v, i);
        }
    }
    v
}

// Allocate a new int object with size digits.
// Return NULL and set exception if we run out of memory.
// Guarantee that the number of bits fits in int64_t.
// This is more than an exbibyte, that is more than many of modern
// architectures support in principle.
// -1 is added to avoid overflow in _PyLong_Frexp().
fn long_alloc(size: i64) -> *mut _longobject {
    let mut result: *mut _longobject = core::ptr::null_mut::<_longobject>();
    if size > 307445734561825860i64 {
        unsafe { PyErr_SetString(PyExc_OverflowError, c"too many digits in integer".as_ptr()) };
        return core::ptr::null_mut::<_longobject>();
    }
    // Fast operations for single digit integers (including zero)
    // assume that there is always at least one digit present.
    let ndigits: i64 = if size != 0 { size } else { 1 };
    if ndigits == 1 {
        result = _Py_FREELIST_POP!(_longobject, ints);
    }
    if result.is_null() {
        // Number of bytes needed is: offsetof(PyLongObject, ob_digit) +
        // sizeof(digit)*size.  Previous incarnations of this code used
        // sizeof() instead of the offsetof, but this risks being
        // incorrect in the presence of padding between the header
        // and the digits.
        result = (unsafe {
            PyObject_Malloc(
                (core::mem::offset_of!(_longobject, long_value.ob_digit) as u64).wrapping_add(
                    (ndigits as u64).wrapping_mul(core::mem::size_of::<u32>() as u64),
                ),
            )
        }) as *mut _longobject;
        if result.is_null() {
            unsafe { PyErr_NoMemory() };
            return core::ptr::null_mut::<_longobject>();
        }
        _PyObject_Init(result as *mut _object, unsafe {
            core::ptr::addr_of_mut!(PyLong_Type)
        });
    }
    _PyLong_SetSignAndDigitCount(result, (size != 0) as i32, size);
    // The digit has to be initialized explicitly to avoid
    // use-of-uninitialized-value.
    unsafe {
        *((*result).long_value.ob_digit.as_mut_ptr() as *mut u32).wrapping_offset(0 as isize) = 0;
    }
    result
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyLong_New(size: i64) -> *mut _longobject {
    long_alloc(size)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyLong_FromDigits(
    negative: i32,
    digit_count: i64,
    digits: *mut u32,
) -> *mut _longobject {
    if digit_count == 0 {
        return _PyLong_GetZero() as *mut _longobject;
    }
    let result: *mut _longobject = long_alloc(digit_count);
    if result.is_null() {
        return core::ptr::null_mut::<_longobject>();
    }
    _PyLong_SetSignAndDigitCount(
        result,
        if negative != 0 {
            1i32.wrapping_neg()
        } else {
            1i32
        },
        digit_count,
    );
    unsafe {
        core::ptr::copy_nonoverlapping(
            ((digits as *const ()) as *const core::ffi::c_void) as *const u8,
            ((((*result).long_value.ob_digit.as_mut_ptr() as *mut u32) as *mut ())
                as *mut core::ffi::c_void) as *mut u8,
            ((digit_count as u64).wrapping_mul(core::mem::size_of::<u32>() as u64) as usize)
                as usize,
        )
    };
    result
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyLong_Copy(src: *mut _longobject) -> *mut _object {
    let sign: i32 = if _PyLong_IsCompact(src as *const _longobject) != 0 {
        let ival: i64 = medium_value!(src);
        if IS_SMALL_INT!(ival) {
            return get_small_int(ival as i32);
        }
        _PyLong_CompactSign(src as *const _longobject)
    } else {
        _PyLong_NonCompactSign(src as *const _longobject)
    };
    let size: i64 = _PyLong_DigitCount(src as *const _longobject);
    let result: *mut _longobject = long_alloc(size);
    if result.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    _PyLong_SetSignAndDigitCount(result, sign, size);
    unsafe {
        core::ptr::copy_nonoverlapping(
            ((((*src).long_value.ob_digit.as_mut_ptr() as *mut u32) as *const ())
                as *const core::ffi::c_void) as *const u8,
            ((((*result).long_value.ob_digit.as_mut_ptr() as *mut u32) as *mut ())
                as *mut core::ffi::c_void) as *mut u8,
            ((size as u64).wrapping_mul(core::mem::size_of::<u32>() as u64) as usize) as usize,
        )
    };
    result as *mut _object
}

fn _PyLong_FromMedium(x: i32) -> *mut _object {
    let mut v: *mut _longobject = _Py_FREELIST_POP!(_longobject, ints);
    if v.is_null() {
        v = (unsafe { PyObject_Malloc(core::mem::size_of::<_longobject>() as u64) })
            as *mut _longobject;
        if v.is_null() {
            unsafe { PyErr_NoMemory() };
            return core::ptr::null_mut::<_object>();
        }
        _PyObject_Init(v as *mut _object, unsafe {
            core::ptr::addr_of_mut!(PyLong_Type)
        });
    }
    let abs_x: u32 = (if x < 0 { x.wrapping_neg() } else { x }) as u32;
    _PyLong_SetSignAndDigitCount(v, if x < 0 { 1i32.wrapping_neg() } else { 1i32 }, 1i64);
    unsafe {
        *((*v).long_value.ob_digit.as_mut_ptr() as *mut u32).wrapping_offset(0 as isize) = abs_x;
    }
    v as *mut _object
}

fn _PyLong_FromLarge(ival: i64) -> *mut _object {
    let mut abs_ival: u64 = 0;
    let sign: i32 = if ival < 0 {
        // negate: can't write this as abs_ival = -ival since that
        // invokes undefined behaviour when ival is LONG_MIN
        abs_ival = 0u64.wrapping_sub(ival as u64);
        1i32.wrapping_neg()
    } else {
        abs_ival = ival as u64;
        1
    };
    // Must be at least two digits
    let mut t: u64 = abs_ival >> 60i32;
    let mut ndigits: i64 = 2;
    while t != 0 {
        ndigits = ndigits.wrapping_add(1);
        t >>= 30i32;
    }
    let v: *mut _longobject = long_alloc(ndigits);
    if !v.is_null() {
        let mut p: *mut u32 = unsafe { (*v).long_value.ob_digit.as_mut_ptr() as *mut u32 };
        _PyLong_SetSignAndDigitCount(v, sign, ndigits);
        t = abs_ival;
        while t != 0 {
            let __v5: *mut u32 = p;
            p = __v5.wrapping_offset(1 as isize);
            unsafe {
                *__v5 = Py_SAFE_DOWNCAST!(t & 1073741823u64, u32);
            }
            t >>= 30i32;
        }
    }
    v as *mut _object
}

/// Create a new int object from a C word-sized int
fn _PyLong_FromSTwoDigits(x: i64) -> *mut _longobject {
    if IS_SMALL_INT!(x) {
        return get_small_int(x as i32) as *mut _longobject;
    }
    if is_medium_int(x) != 0 {
        return _PyLong_FromMedium(x as i32) as *mut _longobject;
    }
    _PyLong_FromLarge(x) as *mut _longobject
}

/// If a freshly-allocated int is already shared, it must
/// be a small integer, so negating it must go to PyLong_FromLong
fn _PyLong_Negate(x_p: *mut *mut _longobject) {
    let x: *mut _longobject = unsafe { *x_p };
    if _PyObject_IsUniquelyReferenced(x as *mut _object) != 0 {
        _PyLong_FlipSign(x);
        return;
    }
    unsafe {
        *x_p = _PyLong_FromSTwoDigits(medium_value!(x).wrapping_neg());
    }
    Py_DECREF__2!(x);
}

// Create a new int object from a C long int
#[unsafe(no_mangle)]
extern "C-unwind" fn PyLong_FromLong(ival: i64) -> *mut _object {
    PYLONG_FROM_INT!(u64, i64, ival);
    unsafe { core::mem::zeroed() }
}

// Create a new int object from a C unsigned long int
#[unsafe(no_mangle)]
extern "C-unwind" fn PyLong_FromUnsignedLong(mut ival: u64) -> *mut _object {
    PYLONG_FROM_UINT!(u64, ival);
    unsafe { core::mem::zeroed() }
}

// Create a new int object from a C unsigned long long int.
#[unsafe(no_mangle)]
extern "C-unwind" fn PyLong_FromUnsignedLongLong(mut ival: u64) -> *mut _object {
    PYLONG_FROM_UINT!(u64, ival);
    unsafe { core::mem::zeroed() }
}

// Create a new int object from a C size_t.
#[unsafe(no_mangle)]
extern "C-unwind" fn PyLong_FromSize_t(mut ival: u64) -> *mut _object {
    PYLONG_FROM_UINT!(u64, ival);
    unsafe { core::mem::zeroed() }
}

// Create a new int object from a C double
#[unsafe(no_mangle)]
extern "C-unwind" fn PyLong_FromDouble(mut dval: f64) -> *mut _object {
    // Try to get out cheap if this fits in a long. When a finite value of real
    // floating type is converted to an integer type, the value is truncated
    // toward zero. If the value of the integral part cannot be represented by
    // the integer type, the behavior is undefined. Thus, we must check that
    // value is in range (LONG_MIN - 1, LONG_MAX + 1). If a long has more bits
    // of precision than a double, casting LONG_MIN - 1 to double may yield an
    // approximation, but LONG_MAX + 1 is a power of two and can be represented
    // as double exactly (assuming FLT_RADIX is 2 or 16), so for simplicity
    // check against [-(LONG_MAX + 1), LONG_MAX + 1).
    let int_max: f64 = 9223372036854775808u64 as f64;
    if -int_max < dval && dval < int_max {
        return PyLong_FromLong(dval as i64);
    }
    let mut frac: f64 = 0 as f64;
    let mut i: i32 = 0;
    let mut ndig: i32 = 0;
    let mut expo: i32 = 0;
    let mut neg: i32 = 0;
    if (if dval.is_infinite() {
        if dval.is_sign_negative() { -1i32 } else { 1i32 }
    } else {
        0i32
    }) != 0
    {
        unsafe {
            PyErr_SetString(
                PyExc_OverflowError,
                c"cannot convert float infinity to integer".as_ptr(),
            )
        };
        return core::ptr::null_mut::<_object>();
    }
    if dval.is_nan() {
        unsafe {
            PyErr_SetString(
                PyExc_ValueError,
                c"cannot convert float NaN to integer".as_ptr(),
            )
        };
        return core::ptr::null_mut::<_object>();
    }
    if dval < 0.0f64 {
        neg = 1;
        dval = -dval;
    }
    frac = unsafe { frexp(dval, core::ptr::addr_of_mut!(expo)) }; // dval = frac*2**expo; 0.0 <= frac < 1.0
    ndig = (expo.wrapping_sub(1) / 30).wrapping_add(1); // Number of 'digits' in result
    let v: *mut _longobject = long_alloc(ndig as i64);
    if v.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    frac = unsafe { ldexp(frac, (expo.wrapping_sub(1) % 30).wrapping_add(1)) };
    i = ndig;
    loop {
        let __v1: i32 = i.wrapping_sub(1);
        i = __v1;
        if __v1 < 0 {
            break;
        }
        let bits: u32 = frac as u32;
        unsafe {
            *((*v).long_value.ob_digit.as_mut_ptr() as *mut u32).wrapping_offset(i as isize) = bits;
        }
        frac -= bits as f64;
        frac = unsafe { ldexp(frac, 30i32) };
    }
    if neg != 0 {
        _PyLong_FlipSign(v);
    }
    v as *mut _object
}

// Checking for overflow in PyLong_AsLong is a PITA since C doesn't define
// anything about what happens when a signed integer operation overflows,
// and some compilers think they're doing you a favor by being "clever"
// then.  The bit pattern for the largest positive signed long is
// (unsigned long)LONG_MAX, and for the smallest negative signed long
// it is abs(LONG_MIN), which we could write -(unsigned long)LONG_MIN.
// However, some other compilers warn about applying unary minus to an
// unsigned operand.  Hence the weird "0-".
// Get a C long int from an int object or any object that has an __index__
// method.
//
// On overflow, return -1 and set *overflow to 1 or -1 depending on the sign of
// the result.  Otherwise *overflow is 0.
//
// For other errors (e.g., TypeError), return -1 and set an error condition.
// In this case *overflow will be 0.
#[unsafe(no_mangle)]
extern "C-unwind" fn PyLong_AsLongAndOverflow(vv: *mut _object, overflow: *mut i32) -> i64 {
    // if PyNumber_Index was called
    let mut do_decref: i32 = unsafe { core::mem::zeroed() };
    let mut sign: i32 = unsafe { core::mem::zeroed() };
    let mut i: i64 = unsafe { core::mem::zeroed() };
    let mut res: i64 = unsafe { core::mem::zeroed() };
    let mut x: u64 = unsafe { core::mem::zeroed() };
    // This version by Tim Peters
    let mut v: *mut _longobject = unsafe { core::mem::zeroed() };
    unsafe {
        do_decref = 0;
        *overflow = 0;
        if vv.is_null() {
            PyErr_BadInternalCall__45!();
            return 1i32.wrapping_neg() as i64;
        } else {
            if PyLong_Check!(vv) != 0 {
                v = vv as *mut _longobject;
            } else {
                v = _PyNumber_Index(vv) as *mut _longobject;
                if v.is_null() {
                    return 1i32.wrapping_neg() as i64;
                } else {
                    do_decref = 1;
                }
            }
            '__join_2: {
                if _PyLong_IsCompact(v as *const _longobject) != 0 {
                    res = _PyLong_CompactValue(v as *const _longobject);
                } else {
                    res = 1i32.wrapping_neg() as i64;
                    i = _PyLong_DigitCount(v as *const _longobject);
                    sign = _PyLong_NonCompactSign(v as *const _longobject);
                    x = 0;
                    '__join_10: {
                        loop {
                            let __v0: i64 = i.wrapping_sub(1);
                            i = __v0;
                            if __v0 >= 0 {
                                let prev: u64 = x;
                                x = x << 30i32
                                    | (*((*v).long_value.ob_digit.as_mut_ptr() as *mut u32)
                                        .wrapping_offset(i as isize)
                                        as u64);
                                if x >> 30i32 != prev {
                                    break '__join_10;
                                }
                            } else {
                                break;
                            }
                        }
                        // Haven't lost any bits, but casting to long requires extra
                        // care (see comment above).
                        if x <= 9223372036854775807 {
                            res = (x as i64).wrapping_mul(sign as i64);
                            break '__join_2;
                        } else {
                            if sign < 0
                                && x == 0u64.wrapping_sub(
                                    9223372036854775807i64.wrapping_neg().wrapping_sub(1) as u64,
                                )
                            {
                                res = 9223372036854775807i64.wrapping_neg().wrapping_sub(1);
                                break '__join_2;
                            } else {
                                *overflow = sign;
                                // res is already set to -1
                                break '__join_2;
                            }
                        }
                    }
                    *overflow = sign;
                }
            }
            if do_decref != 0 {
                Py_DECREF__2!(v);
            }
            return res;
        }
    }
}

// Get a C long int from an int object or any object that has an __index__
// method.  Return -1 and set an error if overflow occurs.
#[unsafe(no_mangle)]
extern "C-unwind" fn PyLong_AsLong(obj: *mut _object) -> i64 {
    let mut overflow: i32 = 0;
    let result: i64 = PyLong_AsLongAndOverflow(obj, core::ptr::addr_of_mut!(overflow));
    if overflow != 0 {
        // XXX: could be cute and give a different
        // message for overflow == -1
        unsafe {
            PyErr_SetString(
                PyExc_OverflowError,
                c"Python int too large to convert to C long".as_ptr(),
            )
        };
    }
    result
}

// Get a C int from an int object or any object that has an __index__
// method.  Return -1 and set an error if overflow occurs.
#[unsafe(no_mangle)]
extern "C-unwind" fn PyLong_AsInt(obj: *mut _object) -> i32 {
    let mut overflow: i32 = 0;
    let result: i64 = PyLong_AsLongAndOverflow(obj, core::ptr::addr_of_mut!(overflow));
    if overflow != 0
        || result > 2147483647
        || result < (2147483647i32.wrapping_neg().wrapping_sub(1) as i64)
    {
        // XXX: could be cute and give a different
        // message for overflow == -1
        unsafe {
            PyErr_SetString(
                PyExc_OverflowError,
                c"Python int too large to convert to C int".as_ptr(),
            )
        };
        return 1i32.wrapping_neg();
    }
    result as i32
}

// Get a Py_ssize_t from an int object.
// Returns -1 and sets an error condition if overflow occurs.
#[unsafe(no_mangle)]
extern "C-unwind" fn PyLong_AsSsize_t(vv: *mut _object) -> i64 {
    let mut sign: i32 = unsafe { core::mem::zeroed() };
    let mut i: i64 = unsafe { core::mem::zeroed() };
    let mut x: u64 = unsafe { core::mem::zeroed() };
    let mut v: *mut _longobject = unsafe { core::mem::zeroed() };
    unsafe {
        if vv.is_null() {
            PyErr_BadInternalCall__25!();
            return 1i32.wrapping_neg() as i64;
        } else {
            if PyLong_Check!(vv) == 0 {
                PyErr_SetString(PyExc_TypeError, c"an integer is required".as_ptr());
                return 1i32.wrapping_neg() as i64;
            } else {
                v = vv as *mut _longobject;
                if _PyLong_IsCompact(v as *const _longobject) != 0 {
                    return _PyLong_CompactValue(v as *const _longobject);
                } else {
                    i = _PyLong_DigitCount(v as *const _longobject);
                    sign = _PyLong_NonCompactSign(v as *const _longobject);
                    x = 0;
                    '__join_0: {
                        loop {
                            let __v0: i64 = i.wrapping_sub(1);
                            i = __v0;
                            if __v0 >= 0 {
                                let prev: u64 = x;
                                x = x << 30i32
                                    | (*((*v).long_value.ob_digit.as_mut_ptr() as *mut u32)
                                        .wrapping_offset(i as isize)
                                        as u64);
                                if x >> 30i32 != prev {
                                    break '__join_0;
                                }
                            } else {
                                break;
                            }
                        }
                        // Haven't lost any bits, but casting to a signed type requires
                        // extra care (see comment above).
                        if x <= 9223372036854775807 {
                            return (x as i64).wrapping_mul(sign as i64);
                        } else {
                            if sign < 0
                                && x == 0u64.wrapping_sub(
                                    9223372036854775807i64.wrapping_neg().wrapping_sub(1) as u64,
                                )
                            {
                                return 9223372036854775807i64.wrapping_neg().wrapping_sub(1);
                            }
                            // else overflow
                        }
                    }
                    PyErr_SetString(
                        PyExc_OverflowError,
                        c"Python int too large to convert to C ssize_t".as_ptr(),
                    );
                    return 1i32.wrapping_neg() as i64;
                }
            }
        }
    }
}

// Get a C unsigned long int from an int object.
// Returns -1 and sets an error condition if overflow occurs.
#[unsafe(no_mangle)]
extern "C-unwind" fn PyLong_AsUnsignedLong(vv: *mut _object) -> u64 {
    let mut i: i64 = unsafe { core::mem::zeroed() };
    let mut x: u64 = unsafe { core::mem::zeroed() };
    let mut v: *mut _longobject = unsafe { core::mem::zeroed() };
    unsafe {
        if vv.is_null() {
            PyErr_BadInternalCall__101!();
            return 1i32.wrapping_neg() as u64;
        } else {
            if PyLong_Check!(vv) == 0 {
                PyErr_SetString(PyExc_TypeError, c"an integer is required".as_ptr());
                return 1i32.wrapping_neg() as u64;
            } else {
                v = vv as *mut _longobject;
                if _PyLong_IsNonNegativeCompact(v as *const _longobject) != 0 {
                    return _PyLong_CompactValue(v as *const _longobject) as u64;
                } else {
                    if _PyLong_IsNegative(v as *const _longobject) {
                        PyErr_SetString(
                            PyExc_OverflowError,
                            c"can't convert negative value to unsigned int".as_ptr(),
                        );
                        return 1i32.wrapping_neg() as u64;
                    } else {
                        i = _PyLong_DigitCount(v as *const _longobject);
                        x = 0;
                        loop {
                            let __v0: i64 = i.wrapping_sub(1);
                            i = __v0;
                            if __v0 >= 0 {
                                let prev: u64 = x;
                                x = x << 30i32
                                    | (*((*v).long_value.ob_digit.as_mut_ptr() as *mut u32)
                                        .wrapping_offset(i as isize)
                                        as u64);
                                if x >> 30i32 != prev {
                                    PyErr_SetString(
                                        PyExc_OverflowError,
                                        c"Python int too large to convert to C unsigned long"
                                            .as_ptr(),
                                    );
                                    return 1i32.wrapping_neg() as u64;
                                }
                            } else {
                                break;
                            }
                        }
                        return x;
                    }
                }
            }
        }
    }
}

// Get a C size_t from an int object. Returns (size_t)-1 and sets
// an error condition if overflow occurs.
#[unsafe(no_mangle)]
extern "C-unwind" fn PyLong_AsSize_t(vv: *mut _object) -> u64 {
    let mut x: u64 = 0;
    let mut prev: u64 = 0;
    let mut i: i64 = 0;
    if vv.is_null() {
        PyErr_BadInternalCall__20!();
        return 1i32.wrapping_neg() as u64;
    }
    if PyLong_Check!(vv) == 0 {
        unsafe { PyErr_SetString(PyExc_TypeError, c"an integer is required".as_ptr()) };
        return 1i32.wrapping_neg() as u64;
    }
    let v: *mut _longobject = vv as *mut _longobject;
    if _PyLong_IsNonNegativeCompact(v as *const _longobject) != 0 {
        return _PyLong_CompactValue(v as *const _longobject) as u64;
    }
    if _PyLong_IsNegative(v as *const _longobject) {
        unsafe {
            PyErr_SetString(
                PyExc_OverflowError,
                c"can't convert negative value to size_t".as_ptr(),
            )
        };
        return 1i32.wrapping_neg() as u64;
    }
    i = _PyLong_DigitCount(v as *const _longobject);
    loop {
        let __v1: i64 = i.wrapping_sub(1);
        i = __v1;
        if __v1 < 0 {
            break;
        }
        prev = x;
        x = x << 30i32
            | ((unsafe {
                *((*v).long_value.ob_digit.as_mut_ptr() as *mut u32).wrapping_offset(i as isize)
            }) as u64);
        if x >> 30i32 != prev {
            unsafe {
                PyErr_SetString(
                    PyExc_OverflowError,
                    c"Python int too large to convert to C size_t".as_ptr(),
                )
            };
            return 1i32.wrapping_neg() as u64;
        }
    }
    x
}

// Get a C unsigned long int from an int object, ignoring the high bits.
// Returns -1 and sets an error condition if an error occurs.
fn _PyLong_AsUnsignedLongMask(vv: *mut _object) -> u64 {
    let mut x: u64 = 0;
    let mut i: i64 = 0;
    let __v0: bool = vv.is_null() || PyLong_Check!(vv) == 0;
    if __v0 {
        PyErr_BadInternalCall__116!();
        return 1i32.wrapping_neg() as u64;
    }
    let v: *mut _longobject = vv as *mut _longobject;
    if _PyLong_IsCompact(v as *const _longobject) != 0 {
        return _PyLong_CompactValue(v as *const _longobject) as u64;
    }
    i = _PyLong_DigitCount(v as *const _longobject);
    let sign: i32 = _PyLong_NonCompactSign(v as *const _longobject);
    loop {
        let __v2: i64 = i.wrapping_sub(1);
        i = __v2;
        if __v2 < 0 {
            break;
        }
        x = x << 30i32
            | ((unsafe {
                *((*v).long_value.ob_digit.as_mut_ptr() as *mut u32).wrapping_offset(i as isize)
            }) as u64);
    }
    x.wrapping_mul(sign as u64)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyLong_AsUnsignedLongMask(op: *mut _object) -> u64 {
    let mut val: u64 = 0;
    if op.is_null() {
        PyErr_BadInternalCall__34!();
        return 1i32.wrapping_neg() as u64;
    }
    if PyLong_Check!(op) != 0 {
        return _PyLong_AsUnsignedLongMask(op);
    }
    let lo: *mut _longobject = (unsafe { _PyNumber_Index(op) }) as *mut _longobject;
    if lo.is_null() {
        return 1i32.wrapping_neg() as u64;
    }
    val = _PyLong_AsUnsignedLongMask(lo as *mut _object);
    Py_DECREF__2!(lo);
    val
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyLong_IsPositive(obj: *mut _object) -> i32 {
    if PyLong_Check!(obj) == 0 {
        unsafe { PyErr_Format(PyExc_TypeError, c"expected int, got %T".as_ptr(), obj) };
        return 1i32.wrapping_neg();
    }
    _PyLong_IsPositive(obj as *const _longobject) as i32
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyLong_IsNegative(obj: *mut _object) -> i32 {
    if PyLong_Check!(obj) == 0 {
        unsafe { PyErr_Format(PyExc_TypeError, c"expected int, got %T".as_ptr(), obj) };
        return 1i32.wrapping_neg();
    }
    _PyLong_IsNegative(obj as *const _longobject) as i32
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyLong_IsZero(obj: *mut _object) -> i32 {
    if PyLong_Check!(obj) == 0 {
        unsafe { PyErr_Format(PyExc_TypeError, c"expected int, got %T".as_ptr(), obj) };
        return 1i32.wrapping_neg();
    }
    _PyLong_IsZero(obj as *const _longobject) as i32
}

fn long_sign(vv: *mut _object) -> i32 {
    let v: *mut _longobject = vv as *mut _longobject;
    if _PyLong_IsCompact(v as *const _longobject) != 0 {
        return _PyLong_CompactSign(v as *const _longobject);
    }
    _PyLong_NonCompactSign(v as *const _longobject)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyLong_Sign(vv: *mut _object) -> i32 {
    long_sign(vv)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyLong_GetSign(vv: *mut _object, sign: *mut i32) -> i32 {
    if PyLong_Check!(vv) == 0 {
        unsafe { PyErr_Format(PyExc_TypeError, c"expect int, got %T".as_ptr(), vv) };
        return 1i32.wrapping_neg();
    }
    unsafe {
        *sign = long_sign(vv);
    }
    0
}

fn bit_length_digit(x: u32) -> i32 {
    // digit can be larger than unsigned long, but only PyLong_SHIFT bits
    //
    // of it will be ever used.
    _Py_bit_length(x as u64)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyLong_NumBits(vv: *mut _object) -> i64 {
    let v: *mut _longobject = vv as *mut _longobject;
    let mut result: i64 = 0;
    let mut ndigits: i64 = 0;
    let mut msd_bits: i32 = 0;
    ndigits = _PyLong_DigitCount(v as *const _longobject);
    if ndigits > 0 {
        let msd: u32 = unsafe {
            *((*v).long_value.ob_digit.as_mut_ptr() as *mut u32)
                .wrapping_offset(ndigits.wrapping_sub(1) as isize)
        };
        result = ndigits.wrapping_sub(1).wrapping_mul(30);
        msd_bits = bit_length_digit(msd);
        result = result.wrapping_add(msd_bits as i64);
    }
    result
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyLong_FromByteArray(
    bytes: *const u8,
    n: u64,
    little_endian: i32,
    mut is_signed: i32,
) -> *mut _object {
    let mut pstartbyte: *const u8 = unsafe { core::mem::zeroed() }; // LSB of bytes
    let mut incr: i32 = 0; // direction to move pstartbyte
    let mut pendbyte: *const u8 = unsafe { core::mem::zeroed() }; // MSB of bytes
    let mut numsignificantbytes: u64 = 0; // number of bytes that matter
    let mut ndigits: i64 = 0; // number of Python int digits
    let mut idigit: i64 = 0; // next free index in v->long_value.ob_digit
    if n == 0 {
        return PyLong_FromLong(0i64);
    }
    if little_endian != 0 {
        pstartbyte = bytes;
        pendbyte = bytes
            .wrapping_offset(n as isize)
            .wrapping_offset(-(1 as isize));
        incr = 1;
    } else {
        pstartbyte = bytes
            .wrapping_offset(n as isize)
            .wrapping_offset(-(1 as isize));
        pendbyte = bytes;
        incr = 1i32.wrapping_neg();
    }
    if is_signed != 0 {
        is_signed = (((unsafe { *pendbyte }) as i32) >= 128) as i32;
    }
    // Compute numsignificantbytes.  This consists of finding the most
    // significant byte.  Leading 0 bytes are insignificant if the number
    // is positive, and leading 0xff bytes if negative.
    {
        let mut i: u64 = 0;
        let mut p: *const u8 = pendbyte;
        let pincr: i32 = incr.wrapping_neg(); // search MSB to LSB
        let insignificant: u8 = (if is_signed != 0 { 255i32 } else { 0i32 }) as u8;
        while i < n {
            if ((unsafe { *p }) as i32) != (insignificant as i32) {
                break;
            }
            i = i.wrapping_add(1);
            let __v3: *const u8 = p;
            p = __v3.wrapping_offset(pincr as isize);
        }
        numsignificantbytes = n.wrapping_sub(i);
        // 2's-comp is a bit tricky here, e.g. 0xff00 == -0x0100, so
        // actually has 2 significant bytes.  OTOH, 0xff0001 ==
        // -0x00ffff, so we wouldn't *need* to bump it there; but we
        // do for 0xffff = -0x0001.  To be safe without bothering to
        // check every case, bump it regardless.
        if is_signed != 0 && numsignificantbytes < n {
            numsignificantbytes = numsignificantbytes.wrapping_add(1);
        }
    }
    // avoid integer overflow
    ndigits = (numsignificantbytes / 30).wrapping_mul(8).wrapping_add(
        (numsignificantbytes % 30)
            .wrapping_mul(8)
            .wrapping_add(30)
            .wrapping_sub(1)
            / 30,
    ) as i64;
    // result
    let v: *mut _longobject = long_alloc(ndigits);
    if v.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    // Copy the bits over.  The tricky parts are computing 2's-comp on
    // the fly for signed numbers, and dealing with the mismatch between
    // 8-bit bytes and (probably) 15-bit Python digits.
    let mut i: u64 = 0;
    let mut carry: u64 = 1; // for 2's-comp calculation
    let mut accum: u64 = 0; // sliding register
    let mut accumbits: u32 = 0; // number of bits in accum
    let mut p: *const u8 = pstartbyte;
    while i < numsignificantbytes {
        let mut thisbyte: u64 = (unsafe { *p }) as u64;
        // Compute correction for 2's comp, if needed.
        if is_signed != 0 {
            thisbyte = (255 ^ thisbyte).wrapping_add(carry);
            carry = thisbyte >> 8i32;
            thisbyte &= 255;
        }
        // Because we're going LSB to MSB, thisbyte is
        // more significant than what's already in accum,
        // so needs to be prepended to accum.
        accum |= thisbyte << accumbits;
        accumbits = accumbits.wrapping_add(8);
        if accumbits >= 30 {
            // There's enough to fill a Python digit.
            unsafe {
                *((*v).long_value.ob_digit.as_mut_ptr() as *mut u32)
                    .wrapping_offset(idigit as isize) = (accum & 1073741823u64) as u32;
            }
            idigit = idigit.wrapping_add(1);
            accum >>= 30i32;
            accumbits = accumbits.wrapping_sub(30);
        }
        i = i.wrapping_add(1);
        let __v22: *const u8 = p;
        p = __v22.wrapping_offset(incr as isize);
    }
    if accumbits != 0 {
        unsafe {
            *((*v).long_value.ob_digit.as_mut_ptr() as *mut u32).wrapping_offset(idigit as isize) =
                accum as u32;
        }
        idigit = idigit.wrapping_add(1);
    }
    let mut sign: i32 = if is_signed != 0 {
        1i32.wrapping_neg()
    } else {
        1
    };
    if idigit == 0 {
        sign = 0;
    }
    _PyLong_SetSignAndDigitCount(v, sign, idigit);
    maybe_small_long(long_normalize(v)) as *mut _object
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyLong_AsByteArray(
    v: *mut _longobject,
    bytes: *mut u8,
    n: u64,
    little_endian: i32,
    is_signed: i32,
    with_exceptions: i32,
) -> i32 {
    // Fill remaining bytes with copies of the sign bit.
    let mut signbyte: u8 = unsafe { core::mem::zeroed() };
    // The main loop filled the byte array exactly, so the code
    // just above didn't get to ensure there's a sign bit, and the
    // loop below wouldn't add one either.  Make sure a sign bit
    // exists.
    let mut sign_bit_set: i32 = unsafe { core::mem::zeroed() };
    // Count # of sign bits -- they needn't be stored,
    // although for signed conversion we need later to
    // make sure at least one sign bit gets stored.
    let mut s: u32 = unsafe { core::mem::zeroed() };
    let mut thisdigit: u32 = unsafe { core::mem::zeroed() };
    // direction to move p
    let mut pincr: i32 = unsafe { core::mem::zeroed() };
    // pointer to next byte in bytes
    let mut p: *mut u8 = unsafe { core::mem::zeroed() };
    // # bytes filled
    let mut j: u64 = unsafe { core::mem::zeroed() };
    // for computing 2's-comp
    let mut carry: u32 = unsafe { core::mem::zeroed() };
    // store 2's-comp?  is_signed and v < 0
    let mut do_twos_comp: i32 = unsafe { core::mem::zeroed() };
    // # bits in accum
    let mut accumbits: u32 = unsafe { core::mem::zeroed() };
    // sliding register
    let mut accum: u64 = unsafe { core::mem::zeroed() };
    // number of digits
    let mut ndigits: i64 = unsafe { core::mem::zeroed() };
    // index into v->long_value.ob_digit
    let mut i: i64 = unsafe { core::mem::zeroed() };
    unsafe {
        ndigits = _PyLong_DigitCount(v as *const _longobject);
        if _PyLong_IsNegative(v as *const _longobject) {
            if is_signed == 0 {
                if with_exceptions != 0 {
                    PyErr_SetString(
                        PyExc_OverflowError,
                        c"can't convert negative int to unsigned".as_ptr(),
                    );
                }
                return 1i32.wrapping_neg();
            } else {
                do_twos_comp = 1;
            }
        } else {
            do_twos_comp = 0;
        }
        if little_endian != 0 {
            p = bytes;
            pincr = 1;
        } else {
            p = bytes
                .wrapping_offset(n as isize)
                .wrapping_offset(-(1 as isize));
            pincr = 1i32.wrapping_neg();
        }
        // Copy over all the Python digits.
        // It's crucial that every Python digit except for the MSD contribute
        // exactly PyLong_SHIFT bits to the total, so first assert that the int is
        // normalized.
        // NOTE: PyLong_AsNativeBytes() assumes that this function will fill in 'n'
        // bytes even if it eventually fails to convert the whole number. Make sure
        // you account for that if you are changing this algorithm to return without
        // doing that.
        j = 0;
        accum = 0;
        accumbits = 0;
        carry = (if do_twos_comp != 0 { 1i32 } else { 0i32 }) as u32;
        i = 0;
        '__join_2: {
            while i < ndigits {
                thisdigit = *((*v).long_value.ob_digit.as_mut_ptr() as *mut u32)
                    .wrapping_offset(i as isize);
                if do_twos_comp != 0 {
                    thisdigit = (thisdigit ^ 1073741823u32).wrapping_add(carry);
                    carry = thisdigit >> 30i32;
                    thisdigit &= 1073741823u32;
                }
                // Because we're going LSB to MSB, thisdigit is more
                // significant than what's already in accum, so needs to be
                // prepended to accum.
                accum |= (thisdigit as u64) << accumbits;
                // The most-significant digit may be (probably is) at least
                // partly empty.
                if i == ndigits.wrapping_sub(1) {
                    s = if do_twos_comp != 0 {
                        thisdigit ^ 1073741823u32
                    } else {
                        thisdigit
                    };
                    while s != 0 {
                        s >>= 1i32;
                        accumbits = accumbits.wrapping_add(1);
                    }
                } else {
                    accumbits = accumbits.wrapping_add(30);
                }
                // Store as many bytes as possible.
                while accumbits >= 8 {
                    if j >= n {
                        break '__join_2;
                    } else {
                        j = j.wrapping_add(1);
                        *p = (accum & 255) as u8;
                        let __v20: *mut u8 = p;
                        p = __v20.wrapping_offset(pincr as isize);
                        accumbits = accumbits.wrapping_sub(8);
                        accum >>= 8i32;
                    }
                }
                i = i.wrapping_add(1);
            }
            // Store the straggler (if any).
            // else do_twos_comp and *every* digit was 0
            if accumbits > 0 {
                if j >= n {
                    break '__join_2;
                } else {
                    j = j.wrapping_add(1);
                    if do_twos_comp != 0 {
                        // Fill leading bits of the byte with sign bits
                        // (appropriately pretending that the int had an
                        // infinite supply of sign bits).
                        let __v7: u64 = accum | 18446744073709551615u64 << accumbits;
                        accum = __v7;
                    }
                    *p = (accum & 255) as u8;
                    let __v6: *mut u8 = p;
                    p = __v6.wrapping_offset(pincr as isize);
                }
            } else {
                if j == n && is_signed != 0 {
                    if n > 0 {
                        let msb: u8 = *p.wrapping_offset(-(pincr as isize));
                        sign_bit_set = ((msb as i32) >= 128) as i32;
                    } else {
                        sign_bit_set = 0;
                    }
                    if sign_bit_set == do_twos_comp {
                        return 0;
                    } else {
                        break '__join_2;
                    }
                }
            }
            signbyte = (if do_twos_comp != 0 { 255u32 } else { 0u32 }) as u8;
            while j < n {
                *p = signbyte;
                j = j.wrapping_add(1);
                let __v1: *mut u8 = p;
                p = __v1.wrapping_offset(pincr as isize);
            }
            return 0;
        }
        if with_exceptions != 0 {
            PyErr_SetString(PyExc_OverflowError, c"int too big to convert".as_ptr());
        }
        return 1i32.wrapping_neg();
    }
}

/// Refactored out for readability, not reuse
fn _fits_in_n_bits(v: i64, n: i64) -> i32 {
    if n >= (core::mem::size_of::<i64>() as i64).wrapping_mul(8) {
        return 1;
    }
    // If all bits above n are the same, we fit.
    //
    // (Use n-1 if we require the sign bit to be consistent.)
    let v_extended: i64 = v >> (n as i32).wrapping_sub(1);
    (v_extended == 0 || v_extended == (1i32.wrapping_neg() as i64)) as i32
}

fn _resolve_endianness(endianness: &mut i32) -> i32 {
    if (unsafe { *(endianness as *mut i32) }) == 1i32.wrapping_neg()
        || (unsafe { *(endianness as *mut i32) }) & 2 != 0
    {
        unsafe {
            *(endianness as *mut i32) = 1;
        }
    } else {
        unsafe {
            *endianness = *endianness & 1;
        }
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyLong_AsNativeBytes(
    vv: *mut _object,
    buffer: *mut (),
    n: i64,
    flags: i32,
) -> i64 {
    let mut v: *mut _longobject = unsafe { core::mem::zeroed() };
    let mut cv: __SlateRecord_longobject_c_1210 = unsafe { core::mem::zeroed() };
    let mut do_decref: i32 = 0;
    let mut res: i64 = 0;
    if vv.is_null() || n < 0 {
        PyErr_BadInternalCall__42!();
        return 1i32.wrapping_neg() as i64;
    }
    let mut little_endian: i32 = flags;
    if _resolve_endianness(unsafe { &mut (*core::ptr::addr_of_mut!(little_endian)) }) < 0 {
        return 1i32.wrapping_neg() as i64;
    }
    if PyLong_Check!(vv) != 0 {
        v = vv as *mut _longobject;
    } else {
        if flags != 1i32.wrapping_neg() && flags & 16 != 0 {
            v = (unsafe { _PyNumber_Index(vv) }) as *mut _longobject;
            if v.is_null() {
                return 1i32.wrapping_neg() as i64;
            }
            do_decref = 1;
        } else {
            unsafe { PyErr_Format(PyExc_TypeError, c"expect int, got %T".as_ptr(), vv) };
            return 1i32.wrapping_neg() as i64;
        }
    }
    let __v0: bool = flags != 1i32.wrapping_neg()
        && flags & 8 != 0
        && _PyLong_IsNegative(v as *const _longobject);
    if __v0 {
        unsafe { PyErr_SetString(PyExc_ValueError, c"Cannot convert negative int".as_ptr()) };
        if do_decref != 0 {
            Py_DECREF__2!(v);
        }
        return 1i32.wrapping_neg() as i64;
    }
    if _PyLong_IsCompact(v as *const _longobject) != 0 {
        res = 0;
        unsafe {
            cv.v = _PyLong_CompactValue(v as *const _longobject);
        }
        // Most paths result in res = sizeof(compact value). Only the case
        // where 0 < n < sizeof(compact value) do we need to check and adjust
        // our return value.
        res = core::mem::size_of::<[u8; 8]>() as i64;
        if n <= 0 {
            // nothing to do!
        } else {
            if n <= (core::mem::size_of::<[u8; 8]>() as i64) {
                if little_endian != 0 {
                    unsafe {
                        core::ptr::copy_nonoverlapping(
                            (((cv.b.as_mut_ptr() as *mut u8) as *const ())
                                as *const core::ffi::c_void)
                                as *const u8,
                            (buffer as *mut core::ffi::c_void) as *mut u8,
                            ((n as u64) as usize) as usize,
                        )
                    };
                } else {
                    for i in 0..n {
                        unsafe {
                            *(buffer as *mut u8)
                                .wrapping_offset(n.wrapping_sub(i).wrapping_sub(1) as isize) =
                                cv.b[i as usize];
                        }
                    }
                }
                // If we fit, return the requested number of bytes
                if _fits_in_n_bits(unsafe { cv.v }, n.wrapping_mul(8)) != 0 {
                    res = n;
                } else {
                    let __v4: bool = (unsafe { cv.v }) > 0
                        && _fits_in_n_bits(unsafe { cv.v }, n.wrapping_mul(8).wrapping_add(1)) != 0;
                    if __v4 {
                        // Positive values with the MSB set do not require an
                        // additional bit when the caller's intent is to treat them
                        // as unsigned.
                        if flags == 1i32.wrapping_neg() || flags & 4 != 0 {
                            res = n;
                        } else {
                            res = n.wrapping_add(1);
                        }
                    }
                }
            } else {
                let fill: u8 = (if (unsafe { cv.v }) < 0 { 255i32 } else { 0i32 }) as u8;
                if little_endian != 0 {
                    unsafe {
                        core::ptr::copy_nonoverlapping(
                            (((cv.b.as_mut_ptr() as *mut u8) as *const ())
                                as *const core::ffi::c_void)
                                as *const u8,
                            (buffer as *mut core::ffi::c_void) as *mut u8,
                            ((core::mem::size_of::<[u8; 8]>() as u64) as usize) as usize,
                        )
                    };
                    unsafe {
                        core::ptr::write_bytes(
                            (((buffer as *mut i8)
                                .wrapping_offset(core::mem::size_of::<[u8; 8]>() as isize)
                                as *mut ()) as *mut core::ffi::c_void)
                                as *mut u8,
                            ((fill as i32) as i32) as u8,
                            ((n as u64).wrapping_sub(core::mem::size_of::<[u8; 8]>() as u64)
                                as usize) as usize,
                        )
                    };
                } else {
                    let mut b: *mut u8 = buffer as *mut u8;
                    {
                        let mut i: i64 = 0;
                        while i < n.wrapping_sub(core::mem::size_of::<[u8; 8]>() as i64) {
                            let __v5: *mut u8 = b;
                            b = __v5.wrapping_offset(1 as isize);
                            unsafe {
                                *__v5 = fill;
                            }
                            i = i.wrapping_add(1);
                        }
                    }
                    let mut i: i64 = core::mem::size_of::<[u8; 8]>() as i64;
                    while i > 0 {
                        let __v10: *mut u8 = b;
                        b = __v10.wrapping_offset(1 as isize);
                        unsafe {
                            *__v10 = cv.b[i.wrapping_sub(1) as usize];
                        }
                        i = i.wrapping_sub(1);
                    }
                }
            }
        }
    } else {
        if n > 0 {
            _PyLong_AsByteArray(v, buffer as *mut u8, n as u64, little_endian, 1i32, 0i32);
        }
        // Calculates the number of bits required for the *absolute* value
        // of v. This does not take sign into account, only magnitude.
        let nb: i64 = _PyLong_NumBits(v as *mut _object);
        // Normally this would be ((nb - 1) / 8) + 1 to avoid rounding up
        // multiples of 8 to the next byte, but we add an implied bit for
        // the sign and it cancels out.
        res = (nb / 8).wrapping_add(1);
        // Two edge cases exist that are best handled after extracting the
        // bits. These may result in us reporting overflow when the value
        // actually fits.
        if n > 0 && res == n.wrapping_add(1) && nb % 8 == 0 {
            if _PyLong_IsNegative(v as *const _longobject) {
                // Values of 0x80...00 from negative values that use every
                // available bit in the buffer do not require an additional
                // bit to store the sign.
                let mut is_edge_case: i32 = 1;
                let mut b: *mut u8 = buffer as *mut u8;
                let mut i: i64 = 0;
                while i < n && is_edge_case != 0 {
                    if i == 0 {
                        is_edge_case = (((unsafe { *b }) as i32)
                            == if little_endian != 0 { 0 } else { 128 })
                            as i32;
                    } else {
                        if i < n.wrapping_sub(1) {
                            is_edge_case = (((unsafe { *b }) as i32) == 0) as i32;
                        } else {
                            is_edge_case = (((unsafe { *b }) as i32)
                                == if little_endian != 0 { 128 } else { 0 })
                                as i32;
                        }
                    }
                    i = i.wrapping_add(1);
                    let __v18: *mut u8 = b;
                    b = __v18.wrapping_offset(1 as isize);
                }
                if is_edge_case != 0 {
                    res = n;
                }
            } else {
                // Positive values with the MSB set do not require an
                // additional bit when the caller's intent is to treat them
                // as unsigned.
                let b: *mut u8 = buffer as *mut u8;
                if ((unsafe {
                    *b.wrapping_offset(
                        (if little_endian != 0 {
                            n.wrapping_sub(1)
                        } else {
                            0i64
                        }) as isize,
                    )
                }) as i32)
                    & 128
                    != 0
                {
                    if flags == 1i32.wrapping_neg() || flags & 4 != 0 {
                        res = n;
                    } else {
                        res = n.wrapping_add(1);
                    }
                }
            }
        }
    }
    if do_decref != 0 {
        Py_DECREF__2!(v);
    }
    res
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyLong_FromNativeBytes(buffer: *const (), n: u64, flags: i32) -> *mut _object {
    if buffer.is_null() {
        PyErr_BadInternalCall__106!();
        return core::ptr::null_mut::<_object>();
    }
    let mut little_endian: i32 = flags;
    if _resolve_endianness(unsafe { &mut (*core::ptr::addr_of_mut!(little_endian)) }) < 0 {
        return core::ptr::null_mut::<_object>();
    }
    _PyLong_FromByteArray(
        buffer as *const u8,
        n,
        little_endian,
        if flags == 1i32.wrapping_neg() || flags & 4 == 0 {
            1i32
        } else {
            0i32
        },
    )
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyLong_FromUnsignedNativeBytes(
    buffer: *const (),
    n: u64,
    mut little_endian: i32,
) -> *mut _object {
    if buffer.is_null() {
        PyErr_BadInternalCall__61!();
        return core::ptr::null_mut::<_object>();
    }
    if _resolve_endianness(unsafe { &mut (*core::ptr::addr_of_mut!(little_endian)) }) < 0 {
        return core::ptr::null_mut::<_object>();
    }
    _PyLong_FromByteArray(buffer as *const u8, n, little_endian, 0i32)
}

// Create a new int object from a C pointer
#[unsafe(no_mangle)]
extern "C-unwind" fn PyLong_FromVoidPtr(p: *mut ()) -> *mut _object {
    PyLong_FromUnsignedLong(p as u64)
}

// Get a C pointer from an int object.
#[unsafe(no_mangle)]
extern "C-unwind" fn PyLong_AsVoidPtr(vv: *mut _object) -> *mut () {
    let mut x: i64 = 0;
    let __v0: bool = PyLong_Check!(vv) != 0 && _PyLong_IsNegative(vv as *const _longobject);
    if __v0 {
        x = PyLong_AsLong(vv);
    } else {
        x = PyLong_AsUnsignedLong(vv) as i64;
    }
    let __v1: bool = x == (1i32.wrapping_neg() as i64) && !unsafe { PyErr_Occurred() }.is_null();
    if __v1 {
        return core::ptr::null_mut::<()>();
    }
    x as *mut ()
}

// Initial long long support by Chris Herborth (chrish@qnx.com), later
// rewritten to use the newer PyLong_{As,From}ByteArray API.
// Create a new int object from a C long long int.
#[unsafe(no_mangle)]
extern "C-unwind" fn PyLong_FromLongLong(ival: i64) -> *mut _object {
    PYLONG_FROM_INT!(u64, i64, ival);
    unsafe { core::mem::zeroed() }
}

// Create a new int object from a C Py_ssize_t.
#[unsafe(no_mangle)]
extern "C-unwind" fn PyLong_FromSsize_t(ival: i64) -> *mut _object {
    PYLONG_FROM_INT!(u64, i64, ival);
    unsafe { core::mem::zeroed() }
}

// Get a C long long int from an int object or any object that has an
// __index__ method.  Return -1 and set an error if overflow occurs.
#[unsafe(no_mangle)]
extern "C-unwind" fn PyLong_AsLongLong(vv: *mut _object) -> i64 {
    let mut v: *mut _longobject = unsafe { core::mem::zeroed() };
    let mut bytes: i64 = 0;
    let mut res: i32 = 0;
    let mut do_decref: i32 = 0; // if PyNumber_Index was called
    if vv.is_null() {
        PyErr_BadInternalCall__50!();
        return 1i32.wrapping_neg() as i64;
    }
    if PyLong_Check!(vv) != 0 {
        v = vv as *mut _longobject;
    } else {
        v = (unsafe { _PyNumber_Index(vv) }) as *mut _longobject;
        if v.is_null() {
            return 1i32.wrapping_neg() as i64;
        }
        do_decref = 1;
    }
    if _PyLong_IsCompact(v as *const _longobject) != 0 {
        res = 0;
        bytes = _PyLong_CompactValue(v as *const _longobject);
    } else {
        res = _PyLong_AsByteArray(
            v,
            core::ptr::addr_of_mut!(bytes) as *mut u8,
            8u64,
            1i32,
            1i32,
            1i32,
        );
    }
    if do_decref != 0 {
        Py_DECREF__2!(v);
    }
    // Plan 9 can't handle long long in ? : expressions
    if res < 0 {
        return 1i32.wrapping_neg() as i64;
    } else {
        return bytes;
    }
}

// Get a C unsigned long long int from an int object.
// Return -1 and set an error if overflow occurs.
#[unsafe(no_mangle)]
extern "C-unwind" fn PyLong_AsUnsignedLongLong(vv: *mut _object) -> u64 {
    let mut bytes: u64 = 0;
    let mut res: i32 = 0;
    if vv.is_null() {
        PyErr_BadInternalCall__2!();
        return 1i32.wrapping_neg() as u64;
    }
    if PyLong_Check!(vv) == 0 {
        unsafe { PyErr_SetString(PyExc_TypeError, c"an integer is required".as_ptr()) };
        return 1i32.wrapping_neg() as u64;
    }
    let v: *mut _longobject = vv as *mut _longobject;
    if _PyLong_IsNonNegativeCompact(v as *const _longobject) != 0 {
        res = 0;
        bytes = _PyLong_CompactValue(v as *const _longobject) as u64;
    } else {
        res = _PyLong_AsByteArray(
            vv as *mut _longobject,
            core::ptr::addr_of_mut!(bytes) as *mut u8,
            8u64,
            1i32,
            0i32,
            1i32,
        );
    }
    // Plan 9 can't handle long long in ? : expressions
    if res < 0 {
        return res as u64;
    } else {
        return bytes;
    }
}

// Get a C unsigned long int from an int object, ignoring the high bits.
// Returns -1 and sets an error condition if an error occurs.
fn _PyLong_AsUnsignedLongLongMask(vv: *mut _object) -> u64 {
    let mut x: u64 = 0;
    let mut i: i64 = 0;
    let mut sign: i32 = 0;
    let __v0: bool = vv.is_null() || PyLong_Check!(vv) == 0;
    if __v0 {
        PyErr_BadInternalCall__58!();
        return 1i32.wrapping_neg() as u64;
    }
    let v: *mut _longobject = vv as *mut _longobject;
    if _PyLong_IsCompact(v as *const _longobject) != 0 {
        return _PyLong_CompactValue(v as *const _longobject) as u64;
    }
    i = _PyLong_DigitCount(v as *const _longobject);
    sign = _PyLong_NonCompactSign(v as *const _longobject);
    loop {
        let __v2: i64 = i.wrapping_sub(1);
        i = __v2;
        if __v2 < 0 {
            break;
        }
        x = x << 30i32
            | ((unsafe {
                *((*v).long_value.ob_digit.as_mut_ptr() as *mut u32).wrapping_offset(i as isize)
            }) as u64);
    }
    x.wrapping_mul(sign as u64)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyLong_AsUnsignedLongLongMask(op: *mut _object) -> u64 {
    let mut val: u64 = 0;
    if op.is_null() {
        PyErr_BadInternalCall__39!();
        return 1i32.wrapping_neg() as u64;
    }
    if PyLong_Check!(op) != 0 {
        return _PyLong_AsUnsignedLongLongMask(op);
    }
    let lo: *mut _longobject = (unsafe { _PyNumber_Index(op) }) as *mut _longobject;
    if lo.is_null() {
        return 1i32.wrapping_neg() as u64;
    }
    val = _PyLong_AsUnsignedLongLongMask(lo as *mut _object);
    Py_DECREF__2!(lo);
    val
}

// Get a C long long int from an int object or any object that has an
// __index__ method.
//
// On overflow, return -1 and set *overflow to 1 or -1 depending on the sign of
// the result.  Otherwise *overflow is 0.
//
// For other errors (e.g., TypeError), return -1 and set an error condition.
// In this case *overflow will be 0.
#[unsafe(no_mangle)]
extern "C-unwind" fn PyLong_AsLongLongAndOverflow(vv: *mut _object, overflow: *mut i32) -> i64 {
    // if PyNumber_Index was called
    let mut do_decref: i32 = unsafe { core::mem::zeroed() };
    let mut sign: i32 = unsafe { core::mem::zeroed() };
    let mut i: i64 = unsafe { core::mem::zeroed() };
    let mut res: i64 = unsafe { core::mem::zeroed() };
    let mut x: u64 = unsafe { core::mem::zeroed() };
    // This version by Tim Peters
    let mut v: *mut _longobject = unsafe { core::mem::zeroed() };
    unsafe {
        do_decref = 0;
        *overflow = 0;
        if vv.is_null() {
            PyErr_BadInternalCall__103!();
            return 1i32.wrapping_neg() as i64;
        } else {
            if PyLong_Check!(vv) != 0 {
                v = vv as *mut _longobject;
            } else {
                v = _PyNumber_Index(vv) as *mut _longobject;
                if v.is_null() {
                    return 1i32.wrapping_neg() as i64;
                } else {
                    do_decref = 1;
                }
            }
            '__join_2: {
                if _PyLong_IsCompact(v as *const _longobject) != 0 {
                    res = _PyLong_CompactValue(v as *const _longobject);
                } else {
                    i = _PyLong_DigitCount(v as *const _longobject);
                    sign = _PyLong_NonCompactSign(v as *const _longobject);
                    x = 0;
                    '__join_10: {
                        loop {
                            let __v0: i64 = i.wrapping_sub(1);
                            i = __v0;
                            if __v0 >= 0 {
                                let prev: u64 = x;
                                x = (x << 30i32).wrapping_add(
                                    *((*v).long_value.ob_digit.as_mut_ptr() as *mut u32)
                                        .wrapping_offset(i as isize)
                                        as u64,
                                );
                                if x >> 30i32 != prev {
                                    break '__join_10;
                                }
                            } else {
                                break;
                            }
                        }
                        // Haven't lost any bits, but casting to long requires extra
                        // care (see comment above).
                        if x <= 9223372036854775807 {
                            res = (x as i64).wrapping_mul(sign as i64);
                            break '__join_2;
                        } else {
                            if sign < 0
                                && x == 0u64.wrapping_sub(
                                    9223372036854775807i64.wrapping_neg().wrapping_sub(1) as u64,
                                )
                            {
                                res = 9223372036854775807i64.wrapping_neg().wrapping_sub(1);
                                break '__join_2;
                            } else {
                                *overflow = sign;
                                res = 1i32.wrapping_neg() as i64;
                                break '__join_2;
                            }
                        }
                    }
                    *overflow = sign;
                    res = 1i32.wrapping_neg() as i64;
                }
            }
            if do_decref != 0 {
                Py_DECREF__2!(v);
            }
            return res;
        }
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyLong_UnsignedShort_Converter(obj: *mut _object, ptr: *mut ()) -> i32 {
    let bytes: i64 = PyLong_AsNativeBytes(obj, ptr, core::mem::size_of::<u16>() as i64, 31i32);
    if bytes < 0 {
        return 0;
    }
    if (bytes as u64) > (core::mem::size_of::<u16>() as u64) {
        unsafe {
            PyErr_SetString(
                PyExc_OverflowError,
                c"Python int too large for C unsigned short".as_ptr(),
            )
        };
        return 0;
    }
    1
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyLong_UnsignedInt_Converter(obj: *mut _object, ptr: *mut ()) -> i32 {
    let bytes: i64 = PyLong_AsNativeBytes(obj, ptr, core::mem::size_of::<u32>() as i64, 31i32);
    if bytes < 0 {
        return 0;
    }
    if (bytes as u64) > (core::mem::size_of::<u32>() as u64) {
        unsafe {
            PyErr_SetString(
                PyExc_OverflowError,
                c"Python int too large for C unsigned int".as_ptr(),
            )
        };
        return 0;
    }
    1
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyLong_UnsignedLong_Converter(obj: *mut _object, ptr: *mut ()) -> i32 {
    let bytes: i64 = PyLong_AsNativeBytes(obj, ptr, core::mem::size_of::<u64>() as i64, 31i32);
    if bytes < 0 {
        return 0;
    }
    if (bytes as u64) > (core::mem::size_of::<u64>() as u64) {
        unsafe {
            PyErr_SetString(
                PyExc_OverflowError,
                c"Python int too large for C unsigned long".as_ptr(),
            )
        };
        return 0;
    }
    1
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyLong_UnsignedLongLong_Converter(obj: *mut _object, ptr: *mut ()) -> i32 {
    let bytes: i64 = PyLong_AsNativeBytes(obj, ptr, core::mem::size_of::<u64>() as i64, 31i32);
    if bytes < 0 {
        return 0;
    }
    if (bytes as u64) > (core::mem::size_of::<u64>() as u64) {
        unsafe {
            PyErr_SetString(
                PyExc_OverflowError,
                c"Python int too large for C unsigned long long".as_ptr(),
            )
        };
        return 0;
    }
    1
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyLong_Size_t_Converter(obj: *mut _object, ptr: *mut ()) -> i32 {
    let bytes: i64 = PyLong_AsNativeBytes(obj, ptr, core::mem::size_of::<u64>() as i64, 31i32);
    if bytes < 0 {
        return 0;
    }
    if (bytes as u64) > (core::mem::size_of::<u64>() as u64) {
        unsafe {
            PyErr_SetString(
                PyExc_OverflowError,
                c"Python int too large for C size_t".as_ptr(),
            )
        };
        return 0;
    }
    1
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyLong_UInt8_Converter(obj: *mut _object, ptr: *mut ()) -> i32 {
    let bytes: i64 = PyLong_AsNativeBytes(obj, ptr, core::mem::size_of::<u8>() as i64, 31i32);
    if bytes < 0 {
        return 0;
    }
    if (bytes as u64) > (core::mem::size_of::<u8>() as u64) {
        unsafe {
            PyErr_SetString(
                PyExc_OverflowError,
                c"Python int too large for C uint8_t".as_ptr(),
            )
        };
        return 0;
    }
    1
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyLong_UInt16_Converter(obj: *mut _object, ptr: *mut ()) -> i32 {
    let bytes: i64 = PyLong_AsNativeBytes(obj, ptr, core::mem::size_of::<u16>() as i64, 31i32);
    if bytes < 0 {
        return 0;
    }
    if (bytes as u64) > (core::mem::size_of::<u16>() as u64) {
        unsafe {
            PyErr_SetString(
                PyExc_OverflowError,
                c"Python int too large for C uint16_t".as_ptr(),
            )
        };
        return 0;
    }
    1
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyLong_UInt32_Converter(obj: *mut _object, ptr: *mut ()) -> i32 {
    let bytes: i64 = PyLong_AsNativeBytes(obj, ptr, core::mem::size_of::<u32>() as i64, 31i32);
    if bytes < 0 {
        return 0;
    }
    if (bytes as u64) > (core::mem::size_of::<u32>() as u64) {
        unsafe {
            PyErr_SetString(
                PyExc_OverflowError,
                c"Python int too large for C uint32_t".as_ptr(),
            )
        };
        return 0;
    }
    1
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyLong_UInt64_Converter(obj: *mut _object, ptr: *mut ()) -> i32 {
    let bytes: i64 = PyLong_AsNativeBytes(obj, ptr, core::mem::size_of::<u64>() as i64, 31i32);
    if bytes < 0 {
        return 0;
    }
    if (bytes as u64) > (core::mem::size_of::<u64>() as u64) {
        unsafe {
            PyErr_SetString(
                PyExc_OverflowError,
                c"Python int too large for C uint64_t".as_ptr(),
            )
        };
        return 0;
    }
    1
}

/// x[0:m] and y[0:n] are digit vectors, LSD first, m >= n required.  x[0:n]
/// is modified in place, by adding y to it.  Carries are propagated as far as
/// x[m-1], and the remaining carry (0 or 1) is returned.
fn v_iadd(x: *mut u32, m: i64, y: *mut u32, n: i64) -> u32 {
    let mut i: i64 = 0;
    let mut carry: u32 = 0;
    while i < n {
        carry = carry.wrapping_add(
            unsafe { *x.wrapping_offset(i as isize) }
                .wrapping_add(unsafe { *y.wrapping_offset(i as isize) }),
        );
        unsafe {
            *x.wrapping_offset(i as isize) = carry & 1073741823u32;
        }
        carry >>= 30i32;
        i = i.wrapping_add(1);
    }
    while carry != 0 && i < m {
        carry = carry.wrapping_add(unsafe { *x.wrapping_offset(i as isize) });
        unsafe {
            *x.wrapping_offset(i as isize) = carry & 1073741823u32;
        }
        carry >>= 30i32;
        i = i.wrapping_add(1);
    }
    carry
}

/// x[0:m] and y[0:n] are digit vectors, LSD first, m >= n required.  x[0:n]
/// is modified in place, by subtracting y from it.  Borrows are propagated as
/// far as x[m-1], and the remaining borrow (0 or 1) is returned.
fn v_isub(x: *mut u32, m: i64, y: *mut u32, n: i64) -> u32 {
    let mut i: i64 = 0;
    let mut borrow: u32 = 0;
    while i < n {
        borrow = unsafe { *x.wrapping_offset(i as isize) }
            .wrapping_sub(unsafe { *y.wrapping_offset(i as isize) })
            .wrapping_sub(borrow);
        unsafe {
            *x.wrapping_offset(i as isize) = borrow & 1073741823u32;
        }
        borrow >>= 30i32;
        borrow &= 1; // keep only 1 sign bit
        i = i.wrapping_add(1);
    }
    while borrow != 0 && i < m {
        borrow = unsafe { *x.wrapping_offset(i as isize) }.wrapping_sub(borrow);
        unsafe {
            *x.wrapping_offset(i as isize) = borrow & 1073741823u32;
        }
        borrow >>= 30i32;
        borrow &= 1;
        i = i.wrapping_add(1);
    }
    borrow
}

/// Shift digit vector a[0:m] d bits left, with 0 <= d < PyLong_SHIFT.  Put
/// result in z[0:m], and return the d bits shifted out of the top.
fn v_lshift(z: *mut u32, a: *mut u32, m: i64, d: i32) -> u32 {
    let mut carry: u32 = 0;
    for i in 0..m {
        let acc: u64 = ((unsafe { *a.wrapping_offset(i as isize) }) as u64) << d | (carry as u64);
        unsafe {
            *z.wrapping_offset(i as isize) = (acc as u32) & 1073741823u32;
        }
        carry = (acc >> 30i32) as u32;
    }
    carry
}

/// Shift digit vector a[0:m] d bits right, with 0 <= d < PyLong_SHIFT.  Put
/// result in z[0:m], and return the d bits shifted out of the bottom.
fn v_rshift(z: *mut u32, a: *mut u32, m: i64, d: i32) -> u32 {
    let mut i: i64 = 0;
    let mut carry: u32 = 0;
    let mask: u32 = (1u32 << d).wrapping_sub(1);
    i = m;
    loop {
        let __v0: i64 = i;
        i = __v0.wrapping_sub(1);
        if __v0 <= 0 {
            break;
        }
        let acc: u64 =
            (carry as u64) << 30i32 | ((unsafe { *a.wrapping_offset(i as isize) }) as u64);
        carry = (acc as u32) & mask;
        unsafe {
            *z.wrapping_offset(i as isize) = (acc >> d) as u32;
        }
    }
    carry
}

/// Divide long pin, w/ size digits, by non-zero digit n, storing quotient
/// in pout, and returning the remainder.  pin and pout point at the LSD.
/// It's OK for pin == pout on entry, which saves oodles of mallocs/frees in
/// _PyLong_Format, but that should be done with great care since ints are
/// immutable.
///
/// This version of the code can be 20% faster than the pre-2022 version
/// on todays compilers on architectures like amd64.  It evolved from Mark
/// Dickinson observing that a 128:64 divide instruction was always being
/// generated by the compiler despite us working with 30-bit digit values.
/// See the thread for full context:
///
///   https://mail.python.org/archives/list/python-dev@python.org/thread/ZICIMX5VFCX4IOFH5NUPVHCUJCQ4Q7QM/#NEUNFZU3TQU4CPTYZNF3WCN7DOJBBTK5
///
/// If you ever want to change this code, pay attention to performance using
/// different compilers, optimization levels, and cpu architectures. Beware of
/// PGO/FDO builds doing value specialization such as a fast path for //10. :)
///
/// Verify that 17 isn't specialized and this works as a quick test:
///   python -m timeit -s 'x = 10**1000; r=x//10; assert r == 10**999, r' 'x//17'
fn inplace_divrem1(pout: *mut u32, pin: *mut u32, mut size: i64, n: u32) -> u32 {
    let mut remainder: u32 = 0;
    loop {
        let __v1: i64 = size.wrapping_sub(1);
        size = __v1;
        if __v1 < 0 {
            break;
        }
        let dividend: u64 =
            (remainder as u64) << 30i32 | ((unsafe { *pin.wrapping_offset(size as isize) }) as u64);
        let quotient: u32 = (dividend / (n as u64)) as u32;
        remainder = (dividend % (n as u64)) as u32;
        unsafe {
            *pout.wrapping_offset(size as isize) = quotient;
        }
    }
    remainder
}

// Divide an integer by a digit, returning both the quotient
// (as function result) and the remainder (through *prem).
// The sign of a is ignored; n should not be zero.
fn divrem1(a: *mut _longobject, n: u32, prem: &mut u32) -> *mut _longobject {
    let size: i64 = _PyLong_DigitCount(a as *const _longobject);
    let z: *mut _longobject = long_alloc(size);
    if z.is_null() {
        return core::ptr::null_mut::<_longobject>();
    }
    unsafe {
        *(prem as *mut u32) = inplace_divrem1(
            (*z).long_value.ob_digit.as_mut_ptr() as *mut u32,
            (*a).long_value.ob_digit.as_mut_ptr() as *mut u32,
            size,
            n,
        );
    }
    long_normalize(z)
}

// Remainder of long pin, w/ size digits, by non-zero digit n,
// returning the remainder. pin points at the LSD.
fn inplace_rem1(pin: *mut u32, mut size: i64, n: u32) -> u32 {
    let mut rem: u64 = 0;
    loop {
        let __v1: i64 = size.wrapping_sub(1);
        size = __v1;
        if __v1 < 0 {
            break;
        }
        rem =
            (rem << 30i32 | ((unsafe { *pin.wrapping_offset(size as isize) }) as u64)) % (n as u64);
    }
    rem as u32
}

// Get the remainder of an integer divided by a digit, returning
// the remainder as the result of the function. The sign of a is
// ignored; n should not be zero.
fn rem1(a: *mut _longobject, n: u32) -> *mut _longobject {
    let size: i64 = _PyLong_DigitCount(a as *const _longobject);
    PyLong_FromLong(inplace_rem1(
        unsafe { (*a).long_value.ob_digit.as_mut_ptr() as *mut u32 },
        size,
        n,
    ) as i64) as *mut _longobject
}

/// asymptotically faster long_to_decimal_string, using _pylong.py
fn pylong_int_to_decimal_string(
    aa: *mut _object,
    p_output: *mut *mut _object,
    writer: *mut __SlateRecord_unicodeobject_h_539,
    bytes_writer: *mut __SlateRecord_pycore_bytesobject_h_89,
    bytes_str: *mut *mut i8,
) -> i32 {
    let mut i: i64 = unsafe { core::mem::zeroed() };
    let mut p: *mut i8 = unsafe { core::mem::zeroed() };
    let mut kind: i32 = unsafe { core::mem::zeroed() };
    let mut data: *const () = unsafe { core::mem::zeroed() };
    let mut size_1: i64 = unsafe { core::mem::zeroed() };
    let mut r#mod: *mut _object = unsafe { core::mem::zeroed() };
    let mut s: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        s = core::ptr::null_mut::<_object>();
        r#mod = PyImport_ImportModule(c"_pylong".as_ptr());
        if r#mod.is_null() {
            return 1i32.wrapping_neg();
        } else {
            '__join_1: {
                s = PyObject_CallMethod(
                    r#mod,
                    c"int_to_decimal_string".as_ptr(),
                    c"O".as_ptr(),
                    aa,
                );
                if !s.is_null() {
                    if PyUnicode_Check!(s) == 0 {
                        PyErr_SetString(
                            PyExc_TypeError,
                            c"_pylong.int_to_decimal_string did not return a str".as_ptr(),
                        );
                    } else {
                        if !writer.is_null() {
                            let size: i64 = PyUnicode_GET_LENGTH!(s);
                            if _PyUnicodeWriter_Prepare__4!(writer, size, 57i32)
                                == 1i32.wrapping_neg()
                            {
                                break '__join_1;
                            } else {
                                if _PyUnicodeWriter_WriteStr(writer, s) < 0 {
                                    break '__join_1;
                                }
                            }
                        } else {
                            if !bytes_writer.is_null() {
                                size_1 = PyUnicode_GET_LENGTH!(s);
                                data = PyUnicode_DATA!(s) as *const ();
                                kind = PyUnicode_KIND!(s);
                                *bytes_str = _PyBytesWriter_Prepare(
                                    bytes_writer,
                                    *bytes_str as *mut (),
                                    size_1,
                                ) as *mut i8;
                                if unsafe { *bytes_str }.is_null() {
                                    break '__join_1;
                                } else {
                                    p = *bytes_str;
                                    i = 0;
                                    while i < size_1 {
                                        let ch: u32 = PyUnicode_READ!(kind, data, i);
                                        let __v4: *mut i8 = p;
                                        p = __v4.wrapping_offset(1 as isize);
                                        *__v4 = ch as i8;
                                        i = i.wrapping_add(1);
                                    }
                                    *bytes_str = p;
                                }
                            } else {
                                *p_output = Py_NewRef!(s);
                            }
                        }
                        Py_DECREF!(r#mod);
                        Py_DECREF!(s);
                        return 0;
                    }
                }
            }
            Py_DECREF!(r#mod);
            Py_XDECREF!(s);
            return 1i32.wrapping_neg();
        }
    }
}

// Convert an integer to a base 10 string.  Returns a new non-shared
// string.  (Return value is non-shared so that callers can modify the
// returned value if necessary.)
fn long_to_decimal_string_internal(
    aa: *mut _object,
    p_output: *mut *mut _object,
    writer: *mut __SlateRecord_unicodeobject_h_539,
    bytes_writer: *mut __SlateRecord_pycore_bytesobject_h_89,
    bytes_str: *mut *mut i8,
) -> i32 {
    let mut str: *mut _object = core::ptr::null_mut::<_object>();
    let mut size: i64 = 0;
    let mut strlen: i64 = 0;
    let mut size_a: i64 = 0;
    let mut i: i64 = 0;
    let mut j: i64 = 0;
    let mut pout: *mut u32 = unsafe { core::mem::zeroed() };
    let mut rem: u32 = 0;
    let mut tenpow: u32 = 10;
    let mut negative: i32 = 0;
    let mut d: i32 = 0;
    // writer or bytes_writer can be used, but not both at the same time.
    let a: *mut _longobject = aa as *mut _longobject;
    let __v0: bool = a.is_null() || PyLong_Check__2!(a) == 0;
    if __v0 {
        PyErr_BadInternalCall__133!();
        return 1i32.wrapping_neg();
    }
    size_a = _PyLong_DigitCount(a as *const _longobject);
    negative = _PyLong_IsNegative(a as *const _longobject) as i32;
    // quick and dirty pre-check for overflowing the decimal digit limit,
    // based on the inequality 10/3 >= log2(10)
    //
    // explanation in https://github.com/python/cpython/pull/96537
    if size_a >= 73i64 {
        let interp: *mut _is = _PyInterpreterState_GET();
        let max_str_digits: i32 = _Py_atomic_load_int(
            (unsafe { core::ptr::addr_of_mut!((*interp).long_state.max_str_digits) }) as *const i32,
        );
        if max_str_digits > 0 && ((max_str_digits / 90i32) as i64) <= size_a.wrapping_sub(11) / 10 {
            unsafe {
                PyErr_Format(PyExc_ValueError, c"Exceeds the limit (%d digits) for integer string conversion; use sys.set_int_max_str_digits() to increase the limit".as_ptr(), max_str_digits)
            };
            return 1i32.wrapping_neg();
        }
    }
    if size_a > 1000 {
        // Switch to _pylong.int_to_decimal_string().
        return pylong_int_to_decimal_string(aa, p_output, writer, bytes_writer, bytes_str);
    }
    // quick and dirty upper bound for the number of digits
    // required to express a in base _PyLong_DECIMAL_BASE:
    //
    //   #digits = 1 + floor(log2(a) / log2(_PyLong_DECIMAL_BASE))
    //
    // But log2(a) < size_a * PyLong_SHIFT, and
    // log2(_PyLong_DECIMAL_BASE) = log2(10) * _PyLong_DECIMAL_SHIFT
    //                            > 3.3 * _PyLong_DECIMAL_SHIFT
    //
    //   size_a * PyLong_SHIFT / (3.3 * _PyLong_DECIMAL_SHIFT) =
    //       size_a + size_a / d < size_a + size_a / floor(d),
    // where d = (3.3 * _PyLong_DECIMAL_SHIFT) /
    //           (PyLong_SHIFT - 3.3 * _PyLong_DECIMAL_SHIFT)
    d = 99i32;
    size = 1i64.wrapping_add(size_a).wrapping_add(size_a / (d as i64));
    let scratch: *mut _longobject = long_alloc(size);
    if scratch.is_null() {
        return 1i32.wrapping_neg();
    }
    // convert array of base _PyLong_BASE digits in pin to an array of
    // base _PyLong_DECIMAL_BASE digits in pout, following Knuth (TAOCP,
    // Volume 2 (3rd edn), section 4.4, Method 1b).
    let pin: *mut u32 = unsafe { (*a).long_value.ob_digit.as_mut_ptr() as *mut u32 };
    pout = unsafe { (*scratch).long_value.ob_digit.as_mut_ptr() as *mut u32 };
    size = 0;
    i = size_a;
    loop {
        let __v11: i64 = i.wrapping_sub(1);
        i = __v11;
        if __v11 < 0 {
            break;
        }
        let mut hi: u32 = unsafe { *pin.wrapping_offset(i as isize) };
        j = 0;
        while j < size {
            let z: u64 =
                ((unsafe { *pout.wrapping_offset(j as isize) }) as u64) << 30i32 | (hi as u64);
            hi = (z / 1000000000) as u32;
            unsafe {
                *pout.wrapping_offset(j as isize) =
                    z.wrapping_sub((hi as u64).wrapping_mul(1000000000)) as u32;
            }
            j = j.wrapping_add(1);
        }
        while hi != 0 {
            let __v4: i64 = size;
            size = __v4.wrapping_add(1);
            unsafe {
                *pout.wrapping_offset(__v4 as isize) = hi % 1000000000;
            }
            hi /= 1000000000;
        }
        // check for keyboard interrupt
        loop {
            if (unsafe { PyErr_CheckSignals() }) != 0 {
                Py_DECREF__2!(scratch);
                return 1i32.wrapping_neg();
            }
            if true {
                break;
            }
        }
    }
    // pout should have at least one digit, so that the case when a = 0
    // works correctly
    if size == 0 {
        let __v13: i64 = size;
        size = __v13.wrapping_add(1);
        unsafe {
            *pout.wrapping_offset(__v13 as isize) = 0;
        }
    }
    // calculate exact length of output string, and allocate
    strlen = (negative.wrapping_add(1) as i64).wrapping_add(size.wrapping_sub(1).wrapping_mul(9));
    rem = unsafe { *pout.wrapping_offset(size.wrapping_sub(1) as isize) };
    while rem >= tenpow {
        tenpow = tenpow.wrapping_mul(10);
        strlen = strlen.wrapping_add(1);
    }
    if strlen > 640 {
        let interp: *mut _is = _PyInterpreterState_GET();
        let max_str_digits: i32 = _Py_atomic_load_int(
            (unsafe { core::ptr::addr_of_mut!((*interp).long_state.max_str_digits) }) as *const i32,
        );
        let strlen_nosign: i64 = strlen.wrapping_sub(negative as i64);
        if max_str_digits > 0 && strlen_nosign > (max_str_digits as i64) {
            Py_DECREF__2!(scratch);
            unsafe {
                PyErr_Format(PyExc_ValueError, c"Exceeds the limit (%d digits) for integer string conversion; use sys.set_int_max_str_digits() to increase the limit".as_ptr(), max_str_digits)
            };
            return 1i32.wrapping_neg();
        }
    }
    if !writer.is_null() {
        if _PyUnicodeWriter_Prepare__4!(writer, strlen, 57i32) == 1i32.wrapping_neg() {
            Py_DECREF__2!(scratch);
            return 1i32.wrapping_neg();
        }
    } else {
        if !bytes_writer.is_null() {
            unsafe {
                *bytes_str =
                    _PyBytesWriter_Prepare(bytes_writer, *bytes_str as *mut (), strlen) as *mut i8;
            }
            if unsafe { *bytes_str }.is_null() {
                Py_DECREF__2!(scratch);
                return 1i32.wrapping_neg();
            }
        } else {
            str = unsafe { PyUnicode_New(strlen, 57u32) };
            if str.is_null() {
                Py_DECREF__2!(scratch);
                return 1i32.wrapping_neg();
            }
        }
    }
    // pout[0] through pout[size-2] contribute exactly                       _PyLong_DECIMAL_SHIFT digits each
    // pout[size-1]: always produce at least one decimal digit
    // and sign
    // check we've counted correctly
    // fill the string right-to-left
    if !bytes_writer.is_null() {
        let mut p: *mut i8 = unsafe { *bytes_str }.wrapping_offset(strlen as isize);
        WRITE_DIGITS__4!(p, i, size, rem, pout, j, negative);
    } else {
        let mut kind: i32 = 0;
        let __v40: i32 = if !writer.is_null() {
            unsafe { (*writer).kind }
        } else {
            PyUnicode_KIND!(str)
        };
        kind = __v40;
        if kind == 1 {
            let mut p: *mut u8 = unsafe { core::mem::zeroed() };
            WRITE_UNICODE_DIGITS__3!(u8, writer, p, strlen, str, i, size, rem, pout, j, negative);
        } else {
            if kind == 2 {
                let mut p: *mut u16 = unsafe { core::mem::zeroed() };
                WRITE_UNICODE_DIGITS__5!(
                    u16, writer, p, strlen, str, i, size, rem, pout, j, negative
                );
            } else {
                let mut p: *mut u32 = unsafe { core::mem::zeroed() };
                WRITE_UNICODE_DIGITS__6!(
                    u32, writer, p, strlen, str, i, size, rem, pout, j, negative
                );
            }
        }
    }
    _Py_DECREF_INT(scratch);
    if !writer.is_null() {
        let __v99: i64 = unsafe { (*writer).pos };
        unsafe {
            (*writer).pos = __v99.wrapping_add(strlen);
        }
    } else {
        if !bytes_writer.is_null() {
            let __v102: *mut i8 = unsafe { *bytes_str };
            unsafe {
                *bytes_str = __v102.wrapping_offset(strlen as isize);
            }
        } else {
            unsafe {
                *p_output = str;
            }
        }
    }
    0
}

#[unsafe(link_section = ".text.slate_distinct.longobject.long_to_decimal_string")]
extern "C-unwind" fn long_to_decimal_string(aa: *mut _object) -> *mut _object {
    let mut v: *mut _object = unsafe { core::mem::zeroed() };
    if long_to_decimal_string_internal(
        aa,
        core::ptr::addr_of_mut!(v),
        core::ptr::null_mut::<__SlateRecord_unicodeobject_h_539>(),
        core::ptr::null_mut::<__SlateRecord_pycore_bytesobject_h_89>(),
        core::ptr::null_mut::<*mut i8>(),
    ) == 1i32.wrapping_neg()
    {
        return core::ptr::null_mut::<_object>();
    }
    v
}

// Convert an int object to a string, using a given conversion base,
// which should be one of 2, 8 or 16.  Return a string object.
// If base is 2, 8 or 16, add the proper prefix '0b', '0o' or '0x'
// if alternate is nonzero.
fn long_format_binary(
    aa: *mut _object,
    mut base: i32,
    alternate: i32,
    p_output: *mut *mut _object,
    writer: *mut __SlateRecord_unicodeobject_h_539,
    bytes_writer: *mut __SlateRecord_pycore_bytesobject_h_89,
    bytes_str: *mut *mut i8,
) -> i32 {
    let a: *mut _longobject = aa as *mut _longobject;
    let mut v: *mut _object = core::ptr::null_mut::<_object>();
    let mut sz: i64 = 0;
    let mut size_a: i64 = 0;
    let mut negative: i32 = 0;
    let mut bits: i32 = 0;
    // writer or bytes_writer can be used, but not both at the same time.
    let __v0: bool = a.is_null() || PyLong_Check__2!(a) == 0;
    if __v0 {
        PyErr_BadInternalCall__147!();
        return 1i32.wrapping_neg();
    }
    size_a = _PyLong_DigitCount(a as *const _longobject);
    negative = _PyLong_IsNegative(a as *const _longobject) as i32;
    // Compute a rough upper bound for the length of the string
    match base {
        16 => {
            bits = 4;
        }
        8 => {
            bits = 3;
        }
        2 => {
            bits = 1;
        }
        _ => {
            Py_UNREACHABLE!();
        }
    }
    // Compute exact length 'sz' of output string.
    if size_a == 0 {
        sz = 1;
    } else {
        let mut size_a_in_bits: i64 = 0;
        // Ensure overflow doesn't occur during computation of sz.
        if size_a > 307445734561825860i64 {
            unsafe { PyErr_SetString(PyExc_OverflowError, c"int too large to format".as_ptr()) };
            return 1i32.wrapping_neg();
        }
        size_a_in_bits = size_a
            .wrapping_sub(1)
            .wrapping_mul(30)
            .wrapping_add(bit_length_digit(unsafe {
                *((*a).long_value.ob_digit.as_mut_ptr() as *mut u32)
                    .wrapping_offset(size_a.wrapping_sub(1) as isize)
            }) as i64);
        // Allow 1 character for a '-' sign.
        sz = (negative as i64)
            .wrapping_add(size_a_in_bits.wrapping_add(bits.wrapping_sub(1) as i64) / (bits as i64));
    }
    if alternate != 0 {
        // 2 characters for prefix
        sz = sz.wrapping_add(2);
    }
    if !writer.is_null() {
        if _PyUnicodeWriter_Prepare__4!(writer, sz, 120i32) == 1i32.wrapping_neg() {
            return 1i32.wrapping_neg();
        }
    } else {
        if !bytes_writer.is_null() {
            unsafe {
                *bytes_str =
                    _PyBytesWriter_Prepare(bytes_writer, *bytes_str as *mut (), sz) as *mut i8;
            }
            if unsafe { *bytes_str }.is_null() {
                return 1i32.wrapping_neg();
            }
        } else {
            v = unsafe { PyUnicode_New(sz, 120u32) };
            if v.is_null() {
                return 1i32.wrapping_neg();
            }
        }
    }
    // JRH: special case for power-of-2 bases
    // # of bits in accum
    // (base == 2)
    if !bytes_writer.is_null() {
        let mut p: *mut i8 = unsafe { *bytes_str }.wrapping_offset(sz as isize);
        WRITE_DIGITS!(p, size_a, a, bits, base, alternate, negative);
    } else {
        let mut kind: i32 = 0;
        let __v35: i32 = if !writer.is_null() {
            unsafe { (*writer).kind }
        } else {
            PyUnicode_KIND!(v)
        };
        kind = __v35;
        if kind == 1 {
            let mut p: *mut u8 = unsafe { core::mem::zeroed() };
            WRITE_UNICODE_DIGITS__4!(
                u8, writer, p, sz, v, size_a, a, bits, base, alternate, negative
            );
        } else {
            if kind == 2 {
                let mut p: *mut u16 = unsafe { core::mem::zeroed() };
                WRITE_UNICODE_DIGITS!(
                    u16, writer, p, sz, v, size_a, a, bits, base, alternate, negative
                );
            } else {
                let mut p: *mut u32 = unsafe { core::mem::zeroed() };
                WRITE_UNICODE_DIGITS__2!(
                    u32, writer, p, sz, v, size_a, a, bits, base, alternate, negative
                );
            }
        }
    }
    if !writer.is_null() {
        let __v127: i64 = unsafe { (*writer).pos };
        unsafe {
            (*writer).pos = __v127.wrapping_add(sz);
        }
    } else {
        if !bytes_writer.is_null() {
            let __v130: *mut i8 = unsafe { *bytes_str };
            unsafe {
                *bytes_str = __v130.wrapping_offset(sz as isize);
            }
        } else {
            unsafe {
                *p_output = v;
            }
        }
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyLong_Format(obj: *mut _object, base: i32) -> *mut _object {
    let mut str: *mut _object = unsafe { core::mem::zeroed() };
    let err: i32 = if base == 10 {
        long_to_decimal_string_internal(
            obj,
            core::ptr::addr_of_mut!(str),
            core::ptr::null_mut::<__SlateRecord_unicodeobject_h_539>(),
            core::ptr::null_mut::<__SlateRecord_pycore_bytesobject_h_89>(),
            core::ptr::null_mut::<*mut i8>(),
        )
    } else {
        long_format_binary(
            obj,
            base,
            1i32,
            core::ptr::addr_of_mut!(str),
            core::ptr::null_mut::<__SlateRecord_unicodeobject_h_539>(),
            core::ptr::null_mut::<__SlateRecord_pycore_bytesobject_h_89>(),
            core::ptr::null_mut::<*mut i8>(),
        )
    };
    if err == 1i32.wrapping_neg() {
        return core::ptr::null_mut::<_object>();
    }
    str
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyLong_FormatWriter(
    writer: *mut __SlateRecord_unicodeobject_h_539,
    obj: *mut _object,
    base: i32,
    alternate: i32,
) -> i32 {
    if base == 10 {
        return long_to_decimal_string_internal(
            obj,
            core::ptr::null_mut::<*mut _object>(),
            writer,
            core::ptr::null_mut::<__SlateRecord_pycore_bytesobject_h_89>(),
            core::ptr::null_mut::<*mut i8>(),
        );
    } else {
        return long_format_binary(
            obj,
            base,
            alternate,
            core::ptr::null_mut::<*mut _object>(),
            writer,
            core::ptr::null_mut::<__SlateRecord_pycore_bytesobject_h_89>(),
            core::ptr::null_mut::<*mut i8>(),
        );
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyLong_FormatBytesWriter(
    writer: *mut __SlateRecord_pycore_bytesobject_h_89,
    str: *mut i8,
    obj: *mut _object,
    base: i32,
    alternate: i32,
) -> *mut i8 {
    let mut str2: *mut i8 = unsafe { core::mem::zeroed() };
    let mut res: i32 = 0;
    str2 = str;
    if base == 10 {
        res = long_to_decimal_string_internal(
            obj,
            core::ptr::null_mut::<*mut _object>(),
            core::ptr::null_mut::<__SlateRecord_unicodeobject_h_539>(),
            writer,
            core::ptr::addr_of_mut!(str2),
        );
    } else {
        res = long_format_binary(
            obj,
            base,
            alternate,
            core::ptr::null_mut::<*mut _object>(),
            core::ptr::null_mut::<__SlateRecord_unicodeobject_h_539>(),
            writer,
            core::ptr::addr_of_mut!(str2),
        );
    }
    if res < 0 {
        return core::ptr::null_mut::<i8>();
    }
    str2
}

/// Table of digit values for 8-bit string -> integer conversion.
/// '0' maps to 0, ..., '9' maps to 9.
/// 'a' and 'A' map to 10, ..., 'z' and 'Z' map to 35.
/// All other indices map to 37.
/// Note that when converting a base B string, a char c is a legitimate
/// base B digit iff _PyLong_DigitValue[Py_CHARPyLong_MASK(c)] < B.
#[unsafe(no_mangle)]
static mut _PyLong_DigitValue: __SlateAlign16<[u8; 256]> = __SlateAlign16([
    37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8,
    37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8,
    37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8,
    0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8,
    10u8, 11u8, 12u8, 13u8, 14u8, 15u8, 16u8, 17u8, 18u8, 19u8, 20u8, 21u8, 22u8, 23u8, 24u8, 25u8,
    26u8, 27u8, 28u8, 29u8, 30u8, 31u8, 32u8, 33u8, 34u8, 35u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8,
    10u8, 11u8, 12u8, 13u8, 14u8, 15u8, 16u8, 17u8, 18u8, 19u8, 20u8, 21u8, 22u8, 23u8, 24u8, 25u8,
    26u8, 27u8, 28u8, 29u8, 30u8, 31u8, 32u8, 33u8, 34u8, 35u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8,
    37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8,
    37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8,
    37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8,
    37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8,
    37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8,
    37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8,
    37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8,
    37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8, 37u8,
]);

/// `start` and `end` point to the start and end of a string of base `base`
/// digits.  base is a power of 2 (2, 4, 8, 16, or 32). An unnormalized int is
/// returned in *res. The string should be already validated by the caller and
/// consists only of valid digit characters and underscores. `digits` gives the
/// number of digit characters.
///
/// The point to this routine is that it takes time linear in the
/// number of string characters.
///
/// Return values:
///   -1 on syntax error (exception needs to be set, *res is untouched)
///   0 else (exception may be set, in that case *res is set to NULL)
fn long_from_binary_base(
    start: *const i8,
    end: *const i8,
    digits: i64,
    base: i32,
    res: &mut *mut _longobject,
) -> i32 {
    let mut p: *const i8 = unsafe { core::mem::zeroed() };
    let mut bits_per_char: i32 = 0;
    let mut n: i64 = 0;
    // Read string from right, and fill in int from left; i.e.,
    // from least to most significant in both.
    let mut accum: u64 = 0;
    let mut bits_in_accum: i32 = 0;
    let mut pdigit: *mut u32 = unsafe { core::mem::zeroed() };
    n = base as i64;
    bits_per_char = 1i32.wrapping_neg();
    while n != 0 {
        n >>= 1i32;
        bits_per_char = bits_per_char.wrapping_add(1);
    }
    // n <- the number of Python digits needed,
    // = ceiling((digits * bits_per_char) / PyLong_SHIFT).
    if digits > 9223372036854775778i64 / (bits_per_char as i64) {
        unsafe {
            PyErr_SetString(
                PyExc_ValueError,
                c"int string too large to convert".as_ptr(),
            )
        };
        unsafe {
            *(res as *mut *mut _longobject) = core::ptr::null_mut::<_longobject>();
        }
        return 0;
    }
    n = digits
        .wrapping_mul(bits_per_char as i64)
        .wrapping_add(30)
        .wrapping_sub(1)
        / 30;
    let z: *mut _longobject = long_alloc(n);
    if z.is_null() {
        unsafe {
            *(res as *mut *mut _longobject) = core::ptr::null_mut::<_longobject>();
        }
        return 0;
    }
    pdigit = unsafe { (*z).long_value.ob_digit.as_mut_ptr() as *mut u32 };
    p = end;
    loop {
        let __v16: *const i8 = p;
        let __v17: *const i8 = __v16.wrapping_offset(-(1 as isize));
        p = __v17;
        if !(__v17 >= start) {
            break;
        }
        let mut k: i32 = 0;
        if ((unsafe { *p }) as i32) != 95 {
            k = (unsafe { _PyLong_DigitValue.0[(Py_CHARMASK!(unsafe { *p }) as i32) as usize] })
                as i32;
            accum |= (k as u64) << bits_in_accum;
            bits_in_accum = bits_in_accum.wrapping_add(bits_per_char);
            if bits_in_accum >= 30 {
                let __v10: *mut u32 = pdigit;
                pdigit = __v10.wrapping_offset(1 as isize);
                unsafe {
                    *__v10 = (accum & 1073741823u64) as u32;
                }
                accum >>= 30i32;
                bits_in_accum = bits_in_accum.wrapping_sub(30);
            }
        }
    }
    if bits_in_accum != 0 {
        let __v18: *mut u32 = pdigit;
        pdigit = __v18.wrapping_offset(1 as isize);
        unsafe {
            *__v18 = accum as u32;
        }
    }
    while ((unsafe {
        pdigit.offset_from(((*z).long_value.ob_digit.as_mut_ptr() as *mut u32) as *mut u32)
    }) as i64)
        < n
    {
        let __v20: *mut u32 = pdigit;
        pdigit = __v20.wrapping_offset(1 as isize);
        unsafe {
            *__v20 = 0;
        }
    }
    unsafe {
        *(res as *mut *mut _longobject) = z;
    }
    0
}

/// asymptotically faster str-to-long conversion for base 10, using _pylong.py
fn pylong_int_from_string(start: *const i8, end: *const i8, res: &mut *mut _longobject) -> i32 {
    let mut result: *mut _object = unsafe { core::mem::zeroed() };
    let mut s: *mut _object = unsafe { core::mem::zeroed() };
    let mut r#mod: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        r#mod = PyImport_ImportModule(c"_pylong".as_ptr());
        if !r#mod.is_null() {
            s = PyUnicode_FromStringAndSize(start, end.offset_from(start as *const i8) as i64);
            if s.is_null() {
                Py_DECREF!(r#mod);
            } else {
                result = PyObject_CallMethod(r#mod, c"int_from_string".as_ptr(), c"O".as_ptr(), s);
                Py_DECREF!(s);
                Py_DECREF!(r#mod);
                if !result.is_null() {
                    if PyLong_Check!(result) == 0 {
                        Py_DECREF!(result);
                        PyErr_SetString(
                            PyExc_TypeError,
                            c"_pylong.int_from_string did not return an int".as_ptr(),
                        );
                    } else {
                        *(res as *mut *mut _longobject) = result as *mut _longobject;
                        return 0;
                    }
                }
            }
        }
        *(res as *mut *mut _longobject) = core::ptr::null_mut::<_longobject>();
        return 0;
    }
    // See the long_from_string_base() API comment.
}

// long_from_non_binary_base: parameters and return values are the same as
// long_from_binary_base.
//
// Binary bases can be converted in time linear in the number of digits, because
// Python's representation base is binary.  Other bases (including decimal!) use
// the simple quadratic-time algorithm below, complicated by some speed tricks.
//
// First some math:  the largest integer that can be expressed in N base-B digits
// is B**N-1.  Consequently, if we have an N-digit input in base B, the worst-
// case number of Python digits needed to hold it is the smallest integer n s.t.
//
//     BASE**n-1 >= B**N-1  [or, adding 1 to both sides]
//     BASE**n >= B**N      [taking logs to base BASE]
//     n >= log(B**N)/log(BASE) = N * log(B)/log(BASE)
//
// The static array log_base_BASE[base] == log(base)/log(BASE) so we can compute
// this quickly.  A Python int with that much space is reserved near the start,
// and the result is computed into it.
//
// The input string is actually treated as being in base base**i (i.e., i digits
// are processed at a time), where two more static arrays hold:
//
//     convwidth_base[base] = the largest integer i such that base**i <= BASE
//     convmultmax_base[base] = base ** convwidth_base[base]
//
// The first of these is the largest i such that i consecutive input digits
// must fit in a single Python digit.  The second is effectively the input
// base we're really using.
//
// Viewing the input as a sequence <c0, c1, ..., c_n-1> of digits in base
// convmultmax_base[base], the result is "simply"
//
//    (((c0*B + c1)*B + c2)*B + c3)*B + ... ))) + c_n-1
//
// where B = convmultmax_base[base].
//
// Error analysis:  as above, the number of Python digits `n` needed is worst-
// case
//
//     n >= N * log(B)/log(BASE)
//
// where `N` is the number of input digits in base `B`.  This is computed via
//
//     size_z = (Py_ssize_t)((scan - str) * log_base_BASE[base]) + 1;
//
// below.  Two numeric concerns are how much space this can waste, and whether
// the computed result can be too small.  To be concrete, assume BASE = 2**15,
// which is the default (and it's unlikely anyone changes that).
//
// Waste isn't a problem:  provided the first input digit isn't 0, the difference
// between the worst-case input with N digits and the smallest input with N
// digits is about a factor of B, but B is small compared to BASE so at most
// one allocated Python digit can remain unused on that count.  If
// N*log(B)/log(BASE) is mathematically an exact integer, then truncating that
// and adding 1 returns a result 1 larger than necessary.  However, that can't
// happen:  whenever B is a power of 2, long_from_binary_base() is called
// instead, and it's impossible for B**i to be an integer power of 2**15 when
// B is not a power of 2 (i.e., it's impossible for N*log(B)/log(BASE) to be
// an exact integer when B is not a power of 2, since B**i has a prime factor
// other than 2 in that case, but (2**15)**j's only prime factor is 2).
//
// The computed result can be too small if the true value of N*log(B)/log(BASE)
// is a little bit larger than an exact integer, but due to roundoff errors (in
// computing log(B), log(BASE), their quotient, and/or multiplying that by N)
// yields a numeric result a little less than that integer.  Unfortunately, "how
// close can a transcendental function get to an integer over some range?"
// questions are generally theoretically intractable.  Computer analysis via
// continued fractions is practical:  expand log(B)/log(BASE) via continued
// fractions, giving a sequence i/j of "the best" rational approximations.  Then
// j*log(B)/log(BASE) is approximately equal to (the integer) i.  This shows that
// we can get very close to being in trouble, but very rarely.  For example,
// 76573 is a denominator in one of the continued-fraction approximations to
// log(10)/log(2**15), and indeed:
//
//     >>> log(10)/log(2**15)*76573
//     16958.000000654003
//
// is very close to an integer.  If we were working with IEEE single-precision,
// rounding errors could kill us.  Finding worst cases in IEEE double-precision
// requires better-than-double-precision log() functions, and Tim didn't bother.
// Instead the code checks to see whether the allocated space is enough as each
// new Python digit is added, and copies the whole thing to a larger int if not.
// This should happen extremely rarely, and in fact I don't have a test case
// that triggers it(!).  Instead the code was tested by artificially allocating
// just 1 digit at the start, so that the copying code was exercised for every
// digit beyond the first.
/// Tables are computed by Tools/scripts/long_conv_tables.py
static log_base_BASE: __SlateAlign16<[f64; 37]> = __SlateAlign16([
    0.0f64,
    0.0f64,
    0.0f64,
    0.05283208335737188f64,
    0.0f64,
    0.07739760316291208f64,
    0.08616541669070521f64,
    0.09357849740192013f64,
    0.0f64,
    0.10566416671474375f64,
    0.11073093649624542f64,
    0.11531438728790992f64,
    0.11949875002403855f64,
    0.12334799060470308f64,
    0.12691183073525347f64,
    0.13022968652028397f64,
    0.0f64,
    0.13624876137501132f64,
    0.13899750004807707f64,
    0.14159758378145285f64,
    0.14406426982957873f64,
    0.14641058075929203f64,
    0.14864772062124326f64,
    0.15078539853523376f64,
    0.1528320833573719f64,
    0.15479520632582416f64,
    0.1566813239380364f64,
    0.15849625007211562f64,
    0.1602451640685868f64,
    0.16193269983758574f64,
    0.1635630198536173f64,
    0.16513987701289584f64,
    0.0f64,
    0.1681464706452818f64,
    0.16958209470834465f64,
    0.17097610056483223f64,
    0.17233083338141042f64,
]);

static convwidth_base: __SlateAlign16<[i32; 37]> = __SlateAlign16([
    0i32, 0i32, 0i32, 18i32, 0i32, 12i32, 11i32, 10i32, 0i32, 9i32, 9i32, 8i32, 8i32, 8i32, 7i32,
    7i32, 0i32, 7i32, 7i32, 7i32, 6i32, 6i32, 6i32, 6i32, 6i32, 6i32, 6i32, 6i32, 6i32, 6i32, 6i32,
    6i32, 0i32, 5i32, 5i32, 5i32, 5i32,
]);

static convmultmax_base: __SlateAlign16<[u64; 37]> = __SlateAlign16([
    0u64,
    0u64,
    0u64,
    387420489u64,
    0u64,
    244140625u64,
    362797056u64,
    282475249u64,
    0u64,
    387420489u64,
    1000000000u64,
    214358881u64,
    429981696u64,
    815730721u64,
    105413504u64,
    170859375u64,
    0u64,
    410338673u64,
    612220032u64,
    893871739u64,
    64000000u64,
    85766121u64,
    113379904u64,
    148035889u64,
    191102976u64,
    244140625u64,
    308915776u64,
    387420489u64,
    481890304u64,
    594823321u64,
    729000000u64,
    887503681u64,
    0u64,
    39135393u64,
    45435424u64,
    52521875u64,
    60466176u64,
]);

fn long_from_non_binary_base(
    start: *const i8,
    end: *const i8,
    digits: i64,
    base: i32,
    res: &mut *mut _longobject,
) -> i32 {
    let mut c: u64 = 0; // current input character
    let mut size_z: i64 = 0;
    let mut i: i32 = 0;
    let mut convwidth: i32 = 0;
    let mut convmultmax: u64 = 0;
    let mut convmult: u64 = 0;
    let mut pz: *mut u32 = unsafe { core::mem::zeroed() };
    let mut z: *mut _longobject = unsafe { core::mem::zeroed() };
    let mut p: *const i8 = unsafe { core::mem::zeroed() };
    // Create an int object that can contain the largest possible
    // integer with this base and length.  Note that there's no
    // need to initialize z->long_value.ob_digit -- no slot is read up before
    // being stored into.
    let fsize_z: f64 = (digits as f64) * unsafe { log_base_BASE.0[base as usize] } + 1.0f64;
    if fsize_z > (307445734561825860i64 as f64) {
        // The same exception as in long_alloc().
        unsafe { PyErr_SetString(PyExc_OverflowError, c"too many digits in integer".as_ptr()) };
        unsafe {
            *(res as *mut *mut _longobject) = core::ptr::null_mut::<_longobject>();
        }
        return 0;
    }
    size_z = fsize_z as i64;
    // Uncomment next line to test exceedingly rare copy code
    //
    // size_z = 1;
    z = long_alloc(size_z);
    if z.is_null() {
        unsafe {
            *(res as *mut *mut _longobject) = core::ptr::null_mut::<_longobject>();
        }
        return 0;
    }
    _PyLong_SetSignAndDigitCount(z, 0i32, 0i64);
    // `convwidth` consecutive input digits are treated as a single
    // digit in base `convmultmax`.
    convwidth = unsafe { convwidth_base.0[base as usize] };
    convmultmax = unsafe { convmultmax_base.0[base as usize] };
    // Work ;-)
    p = start;
    while p < end {
        if ((unsafe { *p }) as i32) == 95 {
            let __v0: *const i8 = p;
            p = __v0.wrapping_offset(1 as isize);
        } else {
            // grab up to convwidth digits from the input string
            let __v3: *const i8 = p;
            p = __v3.wrapping_offset(1 as isize);
            c = ((unsafe { _PyLong_DigitValue.0[((((*__v3 as i32) & 255) as u8) as i32) as usize] })
                as u32) as u64;
            i = 1;
            while i < convwidth && p != end {
                if ((unsafe { *p }) as i32) != 95 {
                    i = i.wrapping_add(1);
                    c = c.wrapping_mul(base as u64).wrapping_add(
                        ((unsafe {
                            _PyLong_DigitValue.0[(Py_CHARMASK!(unsafe { *p }) as i32) as usize]
                        }) as i32) as u64,
                    );
                }
                let __v8: *const i8 = p;
                p = __v8.wrapping_offset(1 as isize);
            }
            convmult = convmultmax;
            // Calculate the shift only if we couldn't get
            // convwidth digits.
            if i != convwidth {
                convmult = base as u64;
                while i > 1 {
                    convmult = convmult.wrapping_mul(base as u64);
                    i = i.wrapping_sub(1);
                }
            }
            // Multiply z by convmult, and add c.
            pz = unsafe { (*z).long_value.ob_digit.as_mut_ptr() as *mut u32 };
            let pzstop: *mut u32 =
                pz.wrapping_offset(_PyLong_DigitCount(z as *const _longobject) as isize);
            while pz < pzstop {
                c = c.wrapping_add(((unsafe { *pz }) as u64).wrapping_mul(convmult));
                unsafe {
                    *pz = (c & 1073741823u64) as u32;
                }
                c >>= 30i32;
                let __v20: *mut u32 = pz;
                pz = __v20.wrapping_offset(1 as isize);
            }
            // carry off the current end?
            if c != 0 {
                if _PyLong_DigitCount(z as *const _longobject) < size_z {
                    unsafe {
                        *pz = c as u32;
                    }
                    _PyLong_SetSignAndDigitCount(
                        z,
                        1i32,
                        _PyLong_DigitCount(z as *const _longobject).wrapping_add(1),
                    );
                } else {
                    // Extremely rare.  Get more space.
                    let tmp: *mut _longobject = long_alloc(size_z.wrapping_add(1));
                    if tmp.is_null() {
                        Py_DECREF__2!(z);
                        unsafe {
                            *(res as *mut *mut _longobject) = core::ptr::null_mut::<_longobject>();
                        }
                        return 0;
                    }
                    unsafe {
                        core::ptr::copy_nonoverlapping(
                            ((((*z).long_value.ob_digit.as_mut_ptr() as *mut u32) as *const ())
                                as *const core::ffi::c_void)
                                as *const u8,
                            ((((*tmp).long_value.ob_digit.as_mut_ptr() as *mut u32) as *mut ())
                                as *mut core::ffi::c_void) as *mut u8,
                            ((core::mem::size_of::<u32>() as u64).wrapping_mul(size_z as u64)
                                as usize) as usize,
                        )
                    };
                    Py_SETREF__4!(z, tmp);
                    unsafe {
                        *((*z).long_value.ob_digit.as_mut_ptr() as *mut u32)
                            .wrapping_offset(size_z as isize) = c as u32;
                    }
                    size_z = size_z.wrapping_add(1);
                }
            }
        }
    }
    unsafe {
        *(res as *mut *mut _longobject) = z;
    }
    0
}

/// *str points to the first digit in a string of base `base` digits. base is an
/// integer from 2 to 36 inclusive. Here we don't need to worry about prefixes
/// like 0x or leading +- signs. The string should be null terminated consisting
/// of ASCII digits and separating underscores possibly with trailing whitespace
/// but we have to validate all of those points here.
///
/// If base is a power of 2 then the complexity is linear in the number of
/// characters in the string. Otherwise a quadratic algorithm is used for
/// non-binary bases.
///
/// Return values:
///
///   - Returns -1 on syntax error (exception needs to be set, *res is untouched)
///   - Returns 0 and sets *res to NULL for MemoryError, OverflowError, or
///     _pylong.int_from_string() errors.
///   - Returns 0 and sets *res to an unsigned, unnormalized PyLong (success!).
///
/// Afterwards *str is set to point to the first non-digit (which may be *str!).
fn long_from_string_base(str: &mut *const i8, base: i32, res: &mut *mut _longobject) -> i32 {
    let mut p: *const i8 = unsafe { core::mem::zeroed() };
    let mut prev: i8 = 0;
    let mut digits: i64 = 0;
    let is_binary_base: i32 = (base & base.wrapping_sub(1) == 0) as i32;
    // Here we do four things:
    //
    // - Find the `end` of the string.
    // - Validate the string.
    // - Count the number of `digits` (rather than underscores)
    // - Point *str to the end-of-string or first invalid character.
    let __v0: *const i8 = unsafe { *(str as *mut *const i8) };
    p = __v0;
    let start: *const i8 = __v0;
    // Leading underscore not allowed.
    if ((unsafe { *start }) as i32) == 95 {
        return 1i32.wrapping_neg();
    }
    // Verify all characters are digits and underscores.
    while ((unsafe { _PyLong_DigitValue.0[(Py_CHARMASK!(unsafe { *p }) as i32) as usize] }) as i32)
        < base
        || ((unsafe { *p }) as i32) == 95
    {
        if ((unsafe { *p }) as i32) == 95 {
            // Double underscore not allowed.
            if (prev as i32) == 95 {
                unsafe {
                    *(str as *mut *const i8) = p.wrapping_offset(-(1 as isize));
                }
                return 1i32.wrapping_neg();
            }
        } else {
            digits = digits.wrapping_add(1);
        }
        prev = unsafe { *p };
        let __v3: *const i8 = p;
        p = __v3.wrapping_offset(1 as isize);
    }
    // Trailing underscore not allowed.
    if (prev as i32) == 95 {
        unsafe {
            *(str as *mut *const i8) = p.wrapping_offset(-(1 as isize));
        }
        return 1i32.wrapping_neg();
    }
    let __v6: *const i8 = p;
    let end: *const i8 = __v6;
    unsafe {
        *(str as *mut *const i8) = __v6;
    }
    // Reject empty strings
    if start == end {
        return 1i32.wrapping_neg();
    }
    // Allow only trailing whitespace after `end`
    while (unsafe { *p }) != 0 && Py_ISSPACE!(unsafe { *p }) != 0 {
        let __v7: *const i8 = p;
        p = __v7.wrapping_offset(1 as isize);
    }
    unsafe {
        *(str as *mut *const i8) = p;
    }
    if ((unsafe { *p }) as i32) != 0 {
        return 1i32.wrapping_neg();
    }
    // Pass a validated string consisting of only valid digits and underscores
    // to long_from_xxx_base.
    if is_binary_base != 0 {
        // Use the linear algorithm for binary bases.
        return long_from_binary_base(start, end, digits, base, res);
    } else {
        // Limit the size to avoid excessive computation attacks exploiting the
        // quadratic algorithm.
        if digits > 640 {
            let interp: *mut _is = _PyInterpreterState_GET();
            let max_str_digits: i32 = _Py_atomic_load_int(
                (unsafe { core::ptr::addr_of_mut!((*interp).long_state.max_str_digits) })
                    as *const i32,
            );
            if max_str_digits > 0 && digits > (max_str_digits as i64) {
                unsafe {
                    PyErr_Format(PyExc_ValueError, c"Exceeds the limit (%d digits) for integer string conversion: value has %zd digits; use sys.set_int_max_str_digits() to increase the limit".as_ptr(), max_str_digits, digits)
                };
                unsafe {
                    *(res as *mut *mut _longobject) = core::ptr::null_mut::<_longobject>();
                }
                return 0;
            }
        }
        if digits > 6000 && base == 10 {
            // Switch to _pylong.int_from_string()
            return pylong_int_from_string(start, end, res);
        }
        // Use the quadratic algorithm for non binary bases.
        return long_from_non_binary_base(start, end, digits, base, res);
    }
}

/// Parses an int from a bytestring. Leading and trailing whitespace will be
/// ignored.
///
/// If successful, a PyLong object will be returned and 'pend' will be pointing
/// to the first unused byte unless it's NULL.
///
/// If unsuccessful, NULL will be returned.
#[unsafe(no_mangle)]
extern "C-unwind" fn PyLong_FromString(
    mut str: *const i8,
    pend: *mut *mut i8,
    mut base: i32,
) -> *mut _object {
    let mut strobj: *mut _object = unsafe { core::mem::zeroed() };
    let mut z: *mut _longobject = unsafe { core::mem::zeroed() };
    let mut orig_str: *const i8 = unsafe { core::mem::zeroed() };
    let mut error_if_nonzero: i32 = unsafe { core::mem::zeroed() };
    let mut sign: i32 = unsafe { core::mem::zeroed() };
    unsafe {
        sign = 1;
        error_if_nonzero = 0;
        orig_str = str;
        z = core::ptr::null_mut::<_longobject>();
        if base != 0 && base < 2 || base > 36 {
            PyErr_SetString(
                PyExc_ValueError,
                c"int() arg 2 must be >= 2 and <= 36".as_ptr(),
            );
            return core::ptr::null_mut::<_object>();
        } else {
            while (*str as i32) != 0 && Py_ISSPACE!(unsafe { *str }) != 0 {
                let __v10: *const i8 = str;
                str = __v10.wrapping_offset(1 as isize);
            }
            if (*str as i32) == 43 {
                let __v6: *const i8 = str;
                str = __v6.wrapping_offset(1 as isize);
            } else {
                if (*str as i32) == 45 {
                    let __v8: *const i8 = str;
                    str = __v8.wrapping_offset(1 as isize);
                    sign = 1i32.wrapping_neg();
                }
            }
            if base == 0 {
                if (*str.wrapping_offset(0 as isize) as i32) != 48 {
                    base = 10;
                } else {
                    if (*str.wrapping_offset(1 as isize) as i32) == 120
                        || (*str.wrapping_offset(1 as isize) as i32) == 88
                    {
                        base = 16;
                    } else {
                        if (*str.wrapping_offset(1 as isize) as i32) == 111
                            || (*str.wrapping_offset(1 as isize) as i32) == 79
                        {
                            base = 8;
                        } else {
                            if (*str.wrapping_offset(1 as isize) as i32) == 98
                                || (*str.wrapping_offset(1 as isize) as i32) == 66
                            {
                                base = 2;
                            } else {
                                // "old" (C-style) octal literal, now invalid.
                                // it might still be zero though
                                error_if_nonzero = 1;
                                base = 10;
                            }
                        }
                    }
                }
            }
            if (*str.wrapping_offset(0 as isize) as i32) == 48
                && (base == 16
                    && ((*str.wrapping_offset(1 as isize) as i32) == 120
                        || (*str.wrapping_offset(1 as isize) as i32) == 88)
                    || base == 8
                        && ((*str.wrapping_offset(1 as isize) as i32) == 111
                            || (*str.wrapping_offset(1 as isize) as i32) == 79)
                    || base == 2
                        && ((*str.wrapping_offset(1 as isize) as i32) == 98
                            || (*str.wrapping_offset(1 as isize) as i32) == 66))
            {
                let __v4: *const i8 = str;
                str = __v4.wrapping_offset(2 as isize);
                // One underscore allowed here.
                if (*str as i32) == 95 {
                    let __v2: *const i8 = str;
                    str = __v2.wrapping_offset(1 as isize);
                }
            }
            '__join_4: {
                // long_from_string_base is the main workhorse here.
                let ret: i32 = long_from_string_base(
                    &mut (*core::ptr::addr_of_mut!(str)),
                    base,
                    &mut (*core::ptr::addr_of_mut!(z)),
                );
                if ret == 1i32.wrapping_neg() {
                    // Syntax error.
                } else {
                    if z.is_null() {
                        // Error. exception already set.
                        return core::ptr::null_mut::<_object>();
                    } else {
                        if error_if_nonzero != 0 {
                            // reset the base to 0, else the exception message
                            // doesn't make too much sense
                            base = 0;
                            if !_PyLong_IsZero(z as *const _longobject) {
                                break '__join_4;
                            }
                            // there might still be other problems, therefore base
                            // remains zero here for the same reason
                        }
                        // Set sign and normalize
                        long_normalize(z);
                        z = maybe_small_long(z);
                        if sign < 0 {
                            _PyLong_Negate(core::ptr::addr_of_mut!(z));
                        }
                        if !pend.is_null() {
                            *pend = str as *mut i8;
                        }
                        return z as *mut _object;
                    }
                }
            }
            if !pend.is_null() {
                *pend = str as *mut i8;
            }
            Py_XDECREF__2!(z);
            let slen: i64 = (if (strlen(orig_str as *const core::ffi::c_char) as u64) < 200 {
                strlen(orig_str as *const core::ffi::c_char) as u64
            } else {
                200u64
            }) as i64;
            strobj = PyUnicode_FromStringAndSize(orig_str, slen);
            if strobj.is_null() {
                return core::ptr::null_mut::<_object>();
            } else {
                PyErr_Format(
                    PyExc_ValueError,
                    c"invalid literal for int() with base %d: %.200R".as_ptr(),
                    base,
                    strobj,
                );
                Py_DECREF!(strobj);
                return core::ptr::null_mut::<_object>();
            }
        }
    }
}

/// Since PyLong_FromString doesn't have a length parameter,
/// check here for possible NULs in the string.
///
/// Reports an invalid literal as a bytes object.
#[unsafe(no_mangle)]
extern "C-unwind" fn _PyLong_FromBytes(s: *const i8, len: i64, base: i32) -> *mut _object {
    let mut end: *mut i8 = core::ptr::null_mut::<i8>();
    let result: *mut _object = PyLong_FromString(s, core::ptr::addr_of_mut!(end), base);
    if end.is_null() || !result.is_null() && end == (s.wrapping_offset(len as isize) as *mut i8) {
        return result;
    }
    Py_XDECREF!(result);
    let strobj: *mut _object = unsafe { PyBytes_FromStringAndSize(s, Py_MIN__2!(len, 200i32)) };
    if !strobj.is_null() {
        unsafe {
            PyErr_Format(
                PyExc_ValueError,
                c"invalid literal for int() with base %d: %.200R".as_ptr(),
                base,
                strobj,
            )
        };
        Py_DECREF!(strobj);
    }
    core::ptr::null_mut::<_object>()
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyLong_FromUnicodeObject(u: *mut _object, base: i32) -> *mut _object {
    let mut end: *mut i8 = core::ptr::null_mut::<i8>();
    let mut buflen: i64 = 0;
    let asciidig: *mut _object = unsafe { _PyUnicode_TransformDecimalAndSpaceToASCII(u) };
    if asciidig.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    // Simply get a pointer to existing ASCII characters.
    let buffer: *const i8 =
        unsafe { PyUnicode_AsUTF8AndSize(asciidig, core::ptr::addr_of_mut!(buflen)) };
    let result: *mut _object = PyLong_FromString(buffer, core::ptr::addr_of_mut!(end), base);
    if end.is_null()
        || !result.is_null() && end == (buffer.wrapping_offset(buflen as isize) as *mut i8)
    {
        Py_DECREF!(asciidig);
        return result;
    }
    Py_DECREF!(asciidig);
    Py_XDECREF!(result);
    unsafe {
        PyErr_Format(
            PyExc_ValueError,
            c"invalid literal for int() with base %d: %.200R".as_ptr(),
            base,
            u,
        )
    };
    core::ptr::null_mut::<_object>()
}

// Int division with remainder, top-level routine
fn long_divrem(
    a: *mut _longobject,
    b: *mut _longobject,
    pdiv: &mut *mut _longobject,
    prem: *mut *mut _longobject,
) -> i32 {
    let size_a: i64 = _PyLong_DigitCount(a as *const _longobject);
    let size_b: i64 = _PyLong_DigitCount(b as *const _longobject);
    let mut z: *mut _longobject = unsafe { core::mem::zeroed() };
    if size_b == 0 {
        unsafe { PyErr_SetString(PyExc_ZeroDivisionError, c"division by zero".as_ptr()) };
        return 1i32.wrapping_neg();
    }
    if size_a < size_b
        || size_a == size_b
            && (unsafe {
                *((*a).long_value.ob_digit.as_mut_ptr() as *mut u32)
                    .wrapping_offset(size_a.wrapping_sub(1) as isize)
            }) < unsafe {
                *((*b).long_value.ob_digit.as_mut_ptr() as *mut u32)
                    .wrapping_offset(size_b.wrapping_sub(1) as isize)
            }
    {
        // |a| < |b|.
        unsafe {
            *prem = long_long(a as *mut _object) as *mut _longobject;
        }
        if unsafe { *prem }.is_null() {
            return 1i32.wrapping_neg();
        }
        unsafe {
            *(pdiv as *mut *mut _longobject) = _PyLong_GetZero() as *mut _longobject;
        }
        return 0;
    }
    if size_b == 1 {
        let mut rem: u32 = 0;
        z = divrem1(
            a,
            unsafe {
                *((*b).long_value.ob_digit.as_mut_ptr() as *mut u32).wrapping_offset(0 as isize)
            },
            unsafe { &mut (*core::ptr::addr_of_mut!(rem)) },
        );
        if z.is_null() {
            return 1i32.wrapping_neg();
        }
        unsafe {
            *prem = PyLong_FromLong(rem as i64) as *mut _longobject;
        }
        if unsafe { *prem }.is_null() {
            Py_DECREF__2!(z);
            return 1i32.wrapping_neg();
        }
    } else {
        z = x_divrem(a, b, prem);
        unsafe {
            *prem = maybe_small_long(*prem);
        }
        if z.is_null() {
            return 1i32.wrapping_neg();
        }
    }
    // Set the signs.
    // The quotient z has the sign of a*b;
    // the remainder r has the sign of a,
    // so a = b*z + r.
    if (_PyLong_IsNegative(a as *const _longobject) as i32)
        != (_PyLong_IsNegative(b as *const _longobject) as i32)
    {
        _PyLong_Negate(core::ptr::addr_of_mut!(z));
        if z.is_null() {
            Py_CLEAR__7!(*prem);
            return 1i32.wrapping_neg();
        }
    }
    let __v1: bool = _PyLong_IsNegative(a as *const _longobject)
        && !_PyLong_IsZero((unsafe { *prem }) as *const _longobject);
    if __v1 {
        _PyLong_Negate(prem);
        if unsafe { *prem }.is_null() {
            Py_DECREF__2!(z);
            Py_CLEAR__7!(*prem);
            return 1i32.wrapping_neg();
        }
    }
    unsafe {
        *(pdiv as *mut *mut _longobject) = maybe_small_long(z);
    }
    0
}

// Int remainder, top-level routine
fn long_rem(a: *mut _longobject, b: *mut _longobject, prem: *mut *mut _longobject) -> i32 {
    let size_a: i64 = _PyLong_DigitCount(a as *const _longobject);
    let size_b: i64 = _PyLong_DigitCount(b as *const _longobject);
    if size_b == 0 {
        unsafe { PyErr_SetString(PyExc_ZeroDivisionError, c"division by zero".as_ptr()) };
        return 1i32.wrapping_neg();
    }
    if size_a < size_b
        || size_a == size_b
            && (unsafe {
                *((*a).long_value.ob_digit.as_mut_ptr() as *mut u32)
                    .wrapping_offset(size_a.wrapping_sub(1) as isize)
            }) < unsafe {
                *((*b).long_value.ob_digit.as_mut_ptr() as *mut u32)
                    .wrapping_offset(size_b.wrapping_sub(1) as isize)
            }
    {
        // |a| < |b|.
        unsafe {
            *prem = long_long(a as *mut _object) as *mut _longobject;
        }
        return (unsafe { *prem }.is_null() as i32).wrapping_neg();
    }
    if size_b == 1 {
        unsafe {
            *prem = rem1(
                a,
                *((*b).long_value.ob_digit.as_mut_ptr() as *mut u32).wrapping_offset(0 as isize),
            );
        }
        if unsafe { *prem }.is_null() {
            return 1i32.wrapping_neg();
        }
    } else {
        // Slow path using divrem.
        Py_XDECREF__2!(x_divrem(a, b, prem));
        unsafe {
            *prem = maybe_small_long(*prem);
        }
        if unsafe { *prem }.is_null() {
            return 1i32.wrapping_neg();
        }
    }
    // Set the sign.
    let __v0: bool = _PyLong_IsNegative(a as *const _longobject)
        && !_PyLong_IsZero((unsafe { *prem }) as *const _longobject);
    if __v0 {
        _PyLong_Negate(prem);
        if unsafe { *prem }.is_null() {
            Py_CLEAR__7!(*prem);
            return 1i32.wrapping_neg();
        }
    }
    0
}

// Unsigned int division with remainder -- the algorithm.  The arguments v1
// and w1 should satisfy 2 <= _PyLong_DigitCount(w1) <= _PyLong_DigitCount(v1).
fn x_divrem(
    v1: *mut _longobject,
    w1: *mut _longobject,
    prem: *mut *mut _longobject,
) -> *mut _longobject {
    let mut i: i64 = 0;
    let mut k: i64 = 0;
    let mut size_v: i64 = 0;
    let mut size_w: i64 = 0;
    let mut d: i32 = 0;
    let mut wm1: u32 = 0;
    let mut wm2: u32 = 0;
    let mut carry: u32 = 0;
    let mut q: u32 = 0;
    let mut r: u32 = 0;
    let mut vtop: u32 = 0;
    let mut vk: *mut u32 = unsafe { core::mem::zeroed() };
    let mut ak: *mut u32 = unsafe { core::mem::zeroed() };
    let mut vv: u64 = 0;
    let mut zhi: i32 = 0;
    let mut z: i64 = 0;
    // We follow Knuth [The Art of Computer Programming, Vol. 2 (3rd
    // edn.), section 4.3.1, Algorithm D], except that we don't explicitly
    // handle the special case when the initial estimate q for a quotient
    // digit is >= PyLong_BASE: the max value for q is PyLong_BASE+1, and
    // that won't overflow a digit.
    // allocate space; w will also be used to hold the final remainder
    size_v = _PyLong_DigitCount(v1 as *const _longobject);
    size_w = _PyLong_DigitCount(w1 as *const _longobject);
    // Assert checks by div()
    let v: *mut _longobject = long_alloc(size_v.wrapping_add(1));
    if v.is_null() {
        unsafe {
            *prem = core::ptr::null_mut::<_longobject>();
        }
        return core::ptr::null_mut::<_longobject>();
    }
    let w: *mut _longobject = long_alloc(size_w);
    if w.is_null() {
        Py_DECREF__2!(v);
        unsafe {
            *prem = core::ptr::null_mut::<_longobject>();
        }
        return core::ptr::null_mut::<_longobject>();
    }
    // normalize: shift w1 left so that its top digit is >= PyLong_BASE/2.
    // shift v1 left by the same amount.  Results go into w and v.
    d = 30i32.wrapping_sub(bit_length_digit(unsafe {
        *((*w1).long_value.ob_digit.as_mut_ptr() as *mut u32)
            .wrapping_offset(size_w.wrapping_sub(1) as isize)
    }));
    carry = v_lshift(
        unsafe { (*w).long_value.ob_digit.as_mut_ptr() as *mut u32 },
        unsafe { (*w1).long_value.ob_digit.as_mut_ptr() as *mut u32 },
        size_w,
        d,
    );
    carry = v_lshift(
        unsafe { (*v).long_value.ob_digit.as_mut_ptr() as *mut u32 },
        unsafe { (*v1).long_value.ob_digit.as_mut_ptr() as *mut u32 },
        size_v,
        d,
    );
    if carry != 0
        || (unsafe {
            *((*v).long_value.ob_digit.as_mut_ptr() as *mut u32)
                .wrapping_offset(size_v.wrapping_sub(1) as isize)
        }) >= unsafe {
            *((*w).long_value.ob_digit.as_mut_ptr() as *mut u32)
                .wrapping_offset(size_w.wrapping_sub(1) as isize)
        }
    {
        unsafe {
            *((*v).long_value.ob_digit.as_mut_ptr() as *mut u32).wrapping_offset(size_v as isize) =
                carry;
        }
        size_v = size_v.wrapping_add(1);
    }
    // Now v->long_value.ob_digit[size_v-1] < w->long_value.ob_digit[size_w-1], so quotient has
    // at most (and usually exactly) k = size_v - size_w digits.
    k = size_v.wrapping_sub(size_w);
    let a: *mut _longobject = long_alloc(k);
    if a.is_null() {
        Py_DECREF__2!(w);
        Py_DECREF__2!(v);
        unsafe {
            *prem = core::ptr::null_mut::<_longobject>();
        }
        return core::ptr::null_mut::<_longobject>();
    }
    let v0: *mut u32 = unsafe { (*v).long_value.ob_digit.as_mut_ptr() as *mut u32 };
    let w0: *mut u32 = unsafe { (*w).long_value.ob_digit.as_mut_ptr() as *mut u32 };
    wm1 = unsafe { *w0.wrapping_offset(size_w.wrapping_sub(1) as isize) };
    wm2 = unsafe { *w0.wrapping_offset(size_w.wrapping_sub(2) as isize) };
    vk = v0.wrapping_offset(k as isize);
    ak = unsafe { (*a).long_value.ob_digit.as_mut_ptr() as *mut u32 }.wrapping_offset(k as isize);
    loop {
        let __v23: *mut u32 = vk;
        vk = __v23.wrapping_offset(-(1 as isize));
        if !(__v23 > v0) {
            break;
        }
        // inner loop: divide vk[0:size_w+1] by w0[0:size_w], giving
        // single-digit quotient q, remainder in vk[0:size_w].
        loop {
            if (unsafe { PyErr_CheckSignals() }) != 0 {
                Py_DECREF__2!(a);
                Py_DECREF__2!(w);
                Py_DECREF__2!(v);
                unsafe {
                    *prem = core::ptr::null_mut::<_longobject>();
                }
                return core::ptr::null_mut::<_longobject>();
            }
            if true {
                break;
            }
        }
        // estimate quotient digit q; may overestimate by 1 (rare)
        vtop = unsafe { *vk.wrapping_offset(size_w as isize) };
        vv = (vtop as u64) << 30i32
            | ((unsafe { *vk.wrapping_offset(size_w.wrapping_sub(1) as isize) }) as u64);
        // The code used to compute the remainder via
        //     r = (digit)(vv - (twodigits)wm1 * q);
        // and compilers generally generated code to do the * and -.
        // But modern processors generally compute q and r with a single
        // instruction, and modern optimizing compilers exploit that if we
        // _don't_ try to optimize it.
        q = (vv / (wm1 as u64)) as u32;
        r = (vv % (wm1 as u64)) as u32;
        while (wm2 as u64).wrapping_mul(q as u64)
            > (r as u64) << 30i32
                | ((unsafe { *vk.wrapping_offset(size_w.wrapping_sub(2) as isize) }) as u64)
        {
            q = q.wrapping_sub(1);
            r = r.wrapping_add(wm1);
            if r >= 1073741824u32 {
                break;
            }
        }
        // subtract q*w0[0:size_w] from vk[0:size_w+1]
        zhi = 0;
        i = 0;
        while i < size_w {
            // invariants: -PyLong_BASE <= -q <= zhi <= 0;
            // -PyLong_BASE * q <= z < PyLong_BASE
            z = (((unsafe { *vk.wrapping_offset(i as isize) }) as i32).wrapping_add(zhi) as i64)
                .wrapping_sub(
                    (q as i64).wrapping_mul((unsafe { *w0.wrapping_offset(i as isize) }) as i64),
                );
            unsafe {
                *vk.wrapping_offset(i as isize) = (z as u32) & 1073741823u32;
            }
            zhi = Py_ARITHMETIC_RIGHT_SHIFT!(z, 30i32) as i32;
            i = i.wrapping_add(1);
        }
        // add w back if q was too large (this branch taken rarely)
        if (vtop as i32).wrapping_add(zhi) < 0 {
            carry = 0;
            i = 0;
            while i < size_w {
                carry = carry.wrapping_add(
                    unsafe { *vk.wrapping_offset(i as isize) }
                        .wrapping_add(unsafe { *w0.wrapping_offset(i as isize) }),
                );
                unsafe {
                    *vk.wrapping_offset(i as isize) = carry & 1073741823u32;
                }
                carry >>= 30i32;
                i = i.wrapping_add(1);
            }
            q = q.wrapping_sub(1);
        }
        // store quotient digit
        let __v21: *mut u32 = ak;
        let __v22: *mut u32 = __v21.wrapping_offset(-(1 as isize));
        ak = __v22;
        unsafe {
            *__v22 = q;
        }
    }
    // unshift remainder; we reuse w to store the result
    carry = v_rshift(w0, v0, size_w, d);
    Py_DECREF__2!(v);
    unsafe {
        *prem = long_normalize(w);
    }
    long_normalize(a)
}

// For a nonzero PyLong a, express a in the form x * 2**e, with 0.5 <=
// abs(x) < 1.0 and e >= 0; return x and put e in *e.  Here x is
// rounded to DBL_MANT_DIG significant bits using round-half-to-even.
// If a == 0, return 0.0 and set *e = 0.
// attempt to define 2.0**DBL_MANT_DIG as a compile-time constant
#[unsafe(no_mangle)]
extern "C-unwind" fn _PyLong_Frexp(a: *mut _longobject, e: *mut i64) -> f64 {
    let mut a_size: i64 = 0;
    let mut shift_digits: i64 = 0;
    let mut x_size: i64 = 0;
    let mut shift_bits: i32 = 0;
    let mut a_bits: i64 = 0;
    // See below for why x_digits is always large enough.
    let mut rem: u32 = 0;
    let mut x_digits: [u32; 3] = {
        let mut __t0: [u32; 3] = unsafe { core::mem::zeroed() };
        __t0[0] = 0u32;
        __t0
    };
    let mut dx: f64 = 0 as f64;
    // Correction term for round-half-to-even rounding.  For a digit x,
    // "x + half_even_correction[x & 7]" gives x rounded to the nearest
    // multiple of 4, rounding ties to a multiple of 8.
    a_size = _PyLong_DigitCount(a as *const _longobject);
    if a_size == 0 {
        // Special case for 0: significand 0.0, exponent 0.
        unsafe {
            *e = 0;
        }
        return 0.0f64;
    }
    a_bits = _PyLong_NumBits(a as *mut _object);
    // Shift the first DBL_MANT_DIG + 2 bits of a into x_digits[0:x_size]
    // (shifting left if a_bits <= DBL_MANT_DIG + 2).
    //
    // Number of digits needed for result: write // for floor division.
    // Then if shifting left, we end up using
    //
    //   1 + a_size + (DBL_MANT_DIG + 2 - a_bits) // PyLong_SHIFT
    //
    // digits.  If shifting right, we use
    //
    //   a_size - (a_bits - DBL_MANT_DIG - 2) // PyLong_SHIFT
    //
    // digits.  Using a_size = 1 + (a_bits - 1) // PyLong_SHIFT along with
    // the inequalities
    //
    //   m // PyLong_SHIFT + n // PyLong_SHIFT <= (m + n) // PyLong_SHIFT
    //   m // PyLong_SHIFT - n // PyLong_SHIFT <=
    //                                    1 + (m - n - 1) // PyLong_SHIFT,
    //
    // valid for any integers m and n, we find that x_size satisfies
    //
    //   x_size <= 2 + (DBL_MANT_DIG + 1) // PyLong_SHIFT
    //
    // in both cases.
    if a_bits <= 55i64 {
        shift_digits = 55i64.wrapping_sub(a_bits) / 30;
        shift_bits = 55i32.wrapping_sub(a_bits as i32) % 30;
        x_size = shift_digits;
        rem = v_lshift(
            (x_digits.as_mut_ptr() as *mut u32).wrapping_offset(x_size as isize),
            unsafe { (*a).long_value.ob_digit.as_mut_ptr() as *mut u32 },
            a_size,
            shift_bits,
        );
        x_size = x_size.wrapping_add(a_size);
        let __v2: i64 = x_size;
        x_size = __v2.wrapping_add(1);
        x_digits[__v2 as usize] = rem;
    } else {
        shift_digits = a_bits.wrapping_sub(53).wrapping_sub(2) / 30;
        shift_bits = (a_bits.wrapping_sub(53).wrapping_sub(2) % 30) as i32;
        rem = v_rshift(
            x_digits.as_mut_ptr() as *mut u32,
            unsafe { (*a).long_value.ob_digit.as_mut_ptr() as *mut u32 }
                .wrapping_offset(shift_digits as isize),
            a_size.wrapping_sub(shift_digits),
            shift_bits,
        );
        x_size = a_size.wrapping_sub(shift_digits);
        // For correct rounding below, we need the least significant
        // bit of x to be 'sticky' for this shift: if any of the bits
        // shifted out was nonzero, we set the least significant bit
        // of x.
        if rem != 0 {
            let __v4: *mut u32 = (x_digits.as_mut_ptr() as *mut u32).wrapping_offset(0 as isize);
            unsafe {
                *__v4 = *__v4 | 1;
            }
        } else {
            while shift_digits > 0 {
                let __v8: i64 = shift_digits.wrapping_sub(1);
                shift_digits = __v8;
                if (unsafe {
                    *((*a).long_value.ob_digit.as_mut_ptr() as *mut u32)
                        .wrapping_offset(__v8 as isize)
                }) != 0
                {
                    let __v9: *mut u32 =
                        (x_digits.as_mut_ptr() as *mut u32).wrapping_offset(0 as isize);
                    unsafe {
                        *__v9 = *__v9 | 1;
                    }
                    break;
                }
            }
        }
    }
    // Round, and convert to double.
    let __v13: *mut u32 = (x_digits.as_mut_ptr() as *mut u32).wrapping_offset(0 as isize);
    let __v14: u32 = unsafe { *__v13 };
    unsafe {
        *__v13 = __v14.wrapping_add(half_even_correction.0[(x_digits[0] & 7) as usize] as u32);
    }
    let __v17: i64 = x_size.wrapping_sub(1);
    x_size = __v17;
    dx = x_digits[__v17 as usize] as f64;
    while x_size > 0 {
        let __v19: i64 = x_size.wrapping_sub(1);
        x_size = __v19;
        dx = dx * (1073741824u32 as f64) + (x_digits[__v19 as usize] as f64);
    }
    // Rescale;  make correction if result is 1.0.
    let __v22: f64 = dx / (4.0f64 * 9007199254740992.0f64);
    dx = __v22;
    if dx == 1.0f64 {
        dx = 0.5f64;
        a_bits = a_bits.wrapping_add(1);
    }
    unsafe {
        *e = a_bits;
    }
    if _PyLong_IsNegative(a as *const _longobject) {
        -dx
    } else {
        dx
    }
}

static half_even_correction: __SlateAlign16<[i32; 8]> = __SlateAlign16([
    0i32,
    1i32.wrapping_neg(),
    2i32.wrapping_neg(),
    1i32,
    0i32,
    1i32.wrapping_neg(),
    2i32,
    1i32,
]);

// Get a C double from an int object.  Rounds to the nearest double,
// using the round-half-to-even rule in the case of a tie.
#[unsafe(no_mangle)]
extern "C-unwind" fn PyLong_AsDouble(v: *mut _object) -> f64 {
    let mut exponent: i64 = 0;
    let mut x: f64 = 0 as f64;
    if v.is_null() {
        PyErr_BadInternalCall__12!();
        return -1.0f64;
    }
    if PyLong_Check!(v) == 0 {
        unsafe { PyErr_SetString(PyExc_TypeError, c"an integer is required".as_ptr()) };
        return -1.0f64;
    }
    if _PyLong_IsCompact(v as *const _longobject) != 0 {
        // Fast path; single digit long (31 bits) will cast safely
        // to double.  This improves performance of FP/long operations
        // by 20%.
        return medium_value!(v as *mut _longobject) as f64;
    }
    x = _PyLong_Frexp(v as *mut _longobject, core::ptr::addr_of_mut!(exponent));
    if exponent > 1024 {
        unsafe {
            PyErr_SetString(
                PyExc_OverflowError,
                c"int too large to convert to float".as_ptr(),
            )
        };
        return -1.0f64;
    }
    unsafe { ldexp(x, exponent as i32) }
}

// Methods
// if a < b, return a negative number
// if a == b, return 0
// if a > b, return a positive number
fn long_compare(a: *mut _longobject, b: *mut _longobject) -> i64 {
    if _PyLong_BothAreCompact(a as *const _longobject, b as *const _longobject) != 0 {
        return _PyLong_CompactValue(a as *const _longobject)
            .wrapping_sub(_PyLong_CompactValue(b as *const _longobject));
    }
    let mut sign: i64 = _PyLong_SignedDigitCount(a as *const _longobject)
        .wrapping_sub(_PyLong_SignedDigitCount(b as *const _longobject));
    if sign == 0 {
        let mut i: i64 = _PyLong_DigitCount(a as *const _longobject);
        let mut diff: i32 = 0;
        loop {
            let __v2: i64 = i.wrapping_sub(1);
            i = __v2;
            if __v2 < 0 {
                break;
            }
            diff = ((unsafe {
                *((*a).long_value.ob_digit.as_mut_ptr() as *mut u32).wrapping_offset(i as isize)
            }) as i32)
                .wrapping_sub(
                    (unsafe {
                        *((*b).long_value.ob_digit.as_mut_ptr() as *mut u32)
                            .wrapping_offset(i as isize)
                    }) as i32,
                );
            if diff != 0 {
                break;
            }
        }
        sign = (if _PyLong_IsNegative(a as *const _longobject) {
            diff.wrapping_neg()
        } else {
            diff
        }) as i64;
    }
    sign
}

#[unsafe(link_section = ".text.slate_distinct.longobject.long_richcompare")]
extern "C-unwind" fn long_richcompare(
    self_: *mut _object,
    other: *mut _object,
    op: i32,
) -> *mut _object {
    let mut result: i64 = 0;
    CHECK_BINOP!(self_, other);
    if self_ == other {
        result = 0;
    } else {
        result = long_compare(self_ as *mut _longobject, other as *mut _longobject);
    }
    loop {
        match op {
            2 => {
                if result == 0 {
                    return _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_TrueStruct) });
                }
                return _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_FalseStruct) });
            }
            3 => {
                if result != 0 {
                    return _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_TrueStruct) });
                }
                return _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_FalseStruct) });
            }
            0 => {
                if result < 0 {
                    return _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_TrueStruct) });
                }
                return _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_FalseStruct) });
            }
            4 => {
                if result > 0 {
                    return _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_TrueStruct) });
                }
                return _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_FalseStruct) });
            }
            1 => {
                if result <= 0 {
                    return _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_TrueStruct) });
                }
                return _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_FalseStruct) });
            }
            5 => {
                if result >= 0 {
                    return _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_TrueStruct) });
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
    unsafe { core::mem::zeroed() }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.longobject._PyLong_ExactDealloc")]
extern "C-unwind" fn _PyLong_ExactDealloc(self_: *mut _object) {
    if _PyLong_IsSmallInt(self_ as *const _longobject) {
        // See PEP 683, section Accidental De-Immortalizing for details
        unsafe { _Py_SetImmortal(self_) };
        return;
    }
    if _PyLong_IsCompact(self_ as *const _longobject) != 0 {
        _PyFreeList_Free(
            unsafe { core::ptr::addr_of_mut!((*_Py_freelists_GET()).ints) },
            _PyObject_CAST!(self_) as *mut (),
            100i64,
            unsafe {
                core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
                    PyObject_Free as *const (),
                )
            },
        );
        return;
    }
    unsafe { PyObject_Free(self_ as *mut ()) };
}

#[unsafe(link_section = ".text.slate_distinct.longobject.long_dealloc")]
extern "C-unwind" fn long_dealloc(self_: *mut _object) {
    if _PyLong_IsSmallInt(self_ as *const _longobject) {
        // This should never get called, but we also don't want to SEGV if
        // we accidentally decref small Ints out of existence. Instead,
        // since small Ints are immortal, re-set the reference count.
        //
        // See PEP 683, section Accidental De-Immortalizing for details
        unsafe { _Py_SetImmortal(self_) };
        return;
    }
    let __v0: bool =
        PyLong_CheckExact!(self_) != 0 && _PyLong_IsCompact(self_ as *const _longobject) != 0;
    if __v0 {
        _PyFreeList_Free(
            unsafe { core::ptr::addr_of_mut!((*_Py_freelists_GET()).ints) },
            _PyObject_CAST!(self_) as *mut (),
            100i64,
            unsafe {
                core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
                    PyObject_Free as *const (),
                )
            },
        );
        return;
    }
    unsafe { unsafe { (*Py_TYPE!(self_)).tp_free }.unwrap()(self_ as *mut ()) };
}

#[unsafe(link_section = ".text.slate_distinct.longobject.long_hash")]
extern "C-unwind" fn long_hash(obj: *mut _object) -> i64 {
    let v: *mut _longobject = obj as *mut _longobject;
    let mut x: u64 = 0;
    let mut i: i64 = 0;
    let mut sign: i32 = 0;
    if _PyLong_IsCompact(v as *const _longobject) != 0 {
        x = _PyLong_CompactValue(v as *const _longobject) as u64;
        if x == (1i32.wrapping_neg() as u64) {
            x = 2i32.wrapping_neg() as u64;
        }
        return x as i64;
    }
    i = _PyLong_DigitCount(v as *const _longobject);
    sign = _PyLong_NonCompactSign(v as *const _longobject);
    x = 0;
    loop {
        let __v5: i64 = i.wrapping_sub(1);
        i = __v5;
        if __v5 < 0 {
            break;
        }
        // Here x is a quantity in the range [0, _PyHASH_MODULUS); we
        // want to compute x * 2**PyLong_SHIFT + v->long_value.ob_digit[i] modulo
        // _PyHASH_MODULUS.
        //
        // The computation of x * 2**PyLong_SHIFT % _PyHASH_MODULUS
        // amounts to a rotation of the bits of x.  To see this, write
        //
        //   x * 2**PyLong_SHIFT = y * 2**_PyHASH_BITS + z
        //
        // where y = x >> (_PyHASH_BITS - PyLong_SHIFT) gives the top
        // PyLong_SHIFT bits of x (those that are shifted out of the
        // original _PyHASH_BITS bits, and z = (x << PyLong_SHIFT) &
        // _PyHASH_MODULUS gives the bottom _PyHASH_BITS - PyLong_SHIFT
        // bits of x, shifted up.  Then since 2**_PyHASH_BITS is
        // congruent to 1 modulo _PyHASH_MODULUS, y*2**_PyHASH_BITS is
        // congruent to y modulo _PyHASH_MODULUS.  So
        //
        //   x * 2**PyLong_SHIFT = y + z (mod _PyHASH_MODULUS).
        //
        // The right-hand side is just the result of rotating the
        // _PyHASH_BITS bits of x left by PyLong_SHIFT places; since
        // not all _PyHASH_BITS bits of x are 1s, the same is true
        // after rotation, so 0 <= y+z < _PyHASH_MODULUS and y + z is
        // the reduction of x*2**PyLong_SHIFT modulo
        // _PyHASH_MODULUS.
        x = x << 30i32 & 2305843009213693951u64 | x >> 31i32;
        let __v0: u64 = x;
        x = __v0.wrapping_add(
            (unsafe {
                *((*v).long_value.ob_digit.as_mut_ptr() as *mut u32).wrapping_offset(i as isize)
            }) as u64,
        );
        if x >= 2305843009213693951u64 {
            x = x.wrapping_sub(2305843009213693951u64);
        }
    }
    x = x.wrapping_mul(sign as u64);
    if x == (1i32.wrapping_neg() as u64) {
        x = 2i32.wrapping_neg() as u64;
    }
    x as i64
}

// Add the absolute values of two integers.
fn x_add(mut a: *mut _longobject, mut b: *mut _longobject) -> *mut _longobject {
    let mut size_a: i64 = _PyLong_DigitCount(a as *const _longobject);
    let mut size_b: i64 = _PyLong_DigitCount(b as *const _longobject);
    let mut i: i64 = 0;
    let mut carry: u32 = 0;
    // Ensure a is the larger of the two:
    if size_a < size_b {
        let temp: *mut _longobject = a;
        a = b;
        b = temp;
        let size_temp: i64 = size_a;
        size_a = size_b;
        size_b = size_temp;
    }
    let z: *mut _longobject = long_alloc(size_a.wrapping_add(1));
    if z.is_null() {
        return core::ptr::null_mut::<_longobject>();
    }
    while i < size_b {
        let __v0: u32 = carry;
        carry = __v0.wrapping_add(
            unsafe {
                *((*a).long_value.ob_digit.as_mut_ptr() as *mut u32).wrapping_offset(i as isize)
            }
            .wrapping_add(unsafe {
                *((*b).long_value.ob_digit.as_mut_ptr() as *mut u32).wrapping_offset(i as isize)
            }),
        );
        unsafe {
            *((*z).long_value.ob_digit.as_mut_ptr() as *mut u32).wrapping_offset(i as isize) =
                carry & 1073741823u32;
        }
        carry >>= 30i32;
        i = i.wrapping_add(1);
    }
    while i < size_a {
        let __v7: u32 = carry;
        carry = __v7.wrapping_add(unsafe {
            *((*a).long_value.ob_digit.as_mut_ptr() as *mut u32).wrapping_offset(i as isize)
        });
        unsafe {
            *((*z).long_value.ob_digit.as_mut_ptr() as *mut u32).wrapping_offset(i as isize) =
                carry & 1073741823u32;
        }
        carry >>= 30i32;
        i = i.wrapping_add(1);
    }
    unsafe {
        *((*z).long_value.ob_digit.as_mut_ptr() as *mut u32).wrapping_offset(i as isize) = carry;
    }
    long_normalize(z)
}

// Subtract the absolute values of two integers.
fn x_sub(mut a: *mut _longobject, mut b: *mut _longobject) -> *mut _longobject {
    let mut size_a: i64 = _PyLong_DigitCount(a as *const _longobject);
    let mut size_b: i64 = _PyLong_DigitCount(b as *const _longobject);
    let mut i: i64 = 0;
    let mut sign: i32 = 1;
    let mut borrow: u32 = 0;
    // Ensure a is the larger of the two:
    if size_a < size_b {
        sign = 1i32.wrapping_neg();
        let temp: *mut _longobject = a;
        a = b;
        b = temp;
        let size_temp: i64 = size_a;
        size_a = size_b;
        size_b = size_temp;
    } else {
        if size_a == size_b {
            // Find highest digit where a and b differ:
            i = size_a;
            loop {
                let __v1: i64 = i.wrapping_sub(1);
                i = __v1;
                if !(__v1 >= 0
                    && (unsafe {
                        *((*a).long_value.ob_digit.as_mut_ptr() as *mut u32)
                            .wrapping_offset(i as isize)
                    }) == unsafe {
                        *((*b).long_value.ob_digit.as_mut_ptr() as *mut u32)
                            .wrapping_offset(i as isize)
                    })
                {
                    break;
                }
            }
            if i < 0 {
                return PyLong_FromLong(0i64) as *mut _longobject;
            }
            if (unsafe {
                *((*a).long_value.ob_digit.as_mut_ptr() as *mut u32).wrapping_offset(i as isize)
            }) < unsafe {
                *((*b).long_value.ob_digit.as_mut_ptr() as *mut u32).wrapping_offset(i as isize)
            } {
                sign = 1i32.wrapping_neg();
                let temp: *mut _longobject = a;
                a = b;
                b = temp;
            }
            let __v3: i64 = i.wrapping_add(1);
            size_b = __v3;
            size_a = __v3;
        }
    }
    let z: *mut _longobject = long_alloc(size_a);
    if z.is_null() {
        return core::ptr::null_mut::<_longobject>();
    }
    i = 0;
    while i < size_b {
        // The following assumes unsigned arithmetic
        // works module 2**N for some N>PyLong_SHIFT.
        borrow = unsafe {
            *((*a).long_value.ob_digit.as_mut_ptr() as *mut u32).wrapping_offset(i as isize)
        }
        .wrapping_sub(unsafe {
            *((*b).long_value.ob_digit.as_mut_ptr() as *mut u32).wrapping_offset(i as isize)
        })
        .wrapping_sub(borrow);
        unsafe {
            *((*z).long_value.ob_digit.as_mut_ptr() as *mut u32).wrapping_offset(i as isize) =
                borrow & 1073741823u32;
        }
        borrow >>= 30i32;
        borrow &= 1; // Keep only one sign bit
        i = i.wrapping_add(1);
    }
    while i < size_a {
        borrow = unsafe {
            *((*a).long_value.ob_digit.as_mut_ptr() as *mut u32).wrapping_offset(i as isize)
        }
        .wrapping_sub(borrow);
        unsafe {
            *((*z).long_value.ob_digit.as_mut_ptr() as *mut u32).wrapping_offset(i as isize) =
                borrow & 1073741823u32;
        }
        borrow >>= 30i32;
        borrow &= 1; // Keep only one sign bit
        i = i.wrapping_add(1);
    }
    if sign < 0 {
        _PyLong_FlipSign(z);
    }
    maybe_small_long(long_normalize(z))
}

fn long_add(a: *mut _longobject, b: *mut _longobject) -> *mut _longobject {
    if _PyLong_BothAreCompact(a as *const _longobject, b as *const _longobject) != 0 {
        let z: i64 = medium_value!(a).wrapping_add(medium_value!(b));
        return _PyLong_FromSTwoDigits(z);
    }
    let mut z: *mut _longobject = unsafe { core::mem::zeroed() };
    if _PyLong_IsNegative(a as *const _longobject) {
        if _PyLong_IsNegative(b as *const _longobject) {
            z = x_add(a, b);
            if !z.is_null() {
                // x_add received at least one multiple-digit int,
                // and thus z must be a multiple-digit int.
                // That also means z is not an element of
                // small_ints, so negating it in-place is safe.
                _PyLong_FlipSign(z);
            }
        } else {
            z = x_sub(b, a);
        }
    } else {
        if _PyLong_IsNegative(b as *const _longobject) {
            z = x_sub(a, b);
        } else {
            z = x_add(a, b);
        }
    }
    z
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyLong_Add(a: *mut _longobject, b: *mut _longobject) -> *mut _object {
    long_add(a, b) as *mut _object
}

#[unsafe(link_section = ".text.slate_distinct.longobject.long_add_method")]
extern "C-unwind" fn long_add_method(a: *mut _object, b: *mut _object) -> *mut _object {
    CHECK_BINOP!(a, b);
    long_add(a as *mut _longobject, b as *mut _longobject) as *mut _object
}

fn long_sub(a: *mut _longobject, b: *mut _longobject) -> *mut _longobject {
    if _PyLong_BothAreCompact(a as *const _longobject, b as *const _longobject) != 0 {
        return _PyLong_FromSTwoDigits(medium_value!(a).wrapping_sub(medium_value!(b)));
    }
    let mut z: *mut _longobject = unsafe { core::mem::zeroed() };
    if _PyLong_IsNegative(a as *const _longobject) {
        if _PyLong_IsNegative(b as *const _longobject) {
            z = x_sub(b, a);
        } else {
            z = x_add(a, b);
            if !z.is_null() {
                _PyLong_FlipSign(z);
            }
        }
    } else {
        if _PyLong_IsNegative(b as *const _longobject) {
            z = x_add(a, b);
        } else {
            z = x_sub(a, b);
        }
    }
    z
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyLong_Subtract(a: *mut _longobject, b: *mut _longobject) -> *mut _object {
    long_sub(a, b) as *mut _object
}

#[unsafe(link_section = ".text.slate_distinct.longobject.long_sub_method")]
extern "C-unwind" fn long_sub_method(a: *mut _object, b: *mut _object) -> *mut _object {
    CHECK_BINOP!(a, b);
    long_sub(a as *mut _longobject, b as *mut _longobject) as *mut _object
}

/// Grade school multiplication, ignoring the signs.
/// Returns the absolute value of the product, or NULL if error.
fn x_mul(a: *mut _longobject, b: *mut _longobject) -> *mut _longobject {
    let size_a: i64 = _PyLong_DigitCount(a as *const _longobject);
    let size_b: i64 = _PyLong_DigitCount(b as *const _longobject);
    let mut i: i64 = 0;
    let z: *mut _longobject = long_alloc(size_a.wrapping_add(size_b));
    if z.is_null() {
        return core::ptr::null_mut::<_longobject>();
    }
    unsafe {
        core::ptr::write_bytes(
            ((((*z).long_value.ob_digit.as_mut_ptr() as *mut u32) as *mut ())
                as *mut core::ffi::c_void) as *mut u8,
            (0i32 as i32) as u8,
            ((_PyLong_DigitCount(z as *const _longobject) as u64)
                .wrapping_mul(core::mem::size_of::<u32>() as u64) as usize) as usize,
        )
    };
    if a == b {
        // Efficient squaring per HAC, Algorithm 14.16:
        // https://cacr.uwaterloo.ca/hac/about/chap14.pdf
        // Gives slightly less than a 2x speedup when a == b,
        // via exploiting that each entry in the multiplication
        // pyramid appears twice (except for the size_a squares).
        let paend: *mut u32 = unsafe { (*a).long_value.ob_digit.as_mut_ptr() as *mut u32 }
            .wrapping_offset(size_a as isize);
        i = 0;
        while i < size_a {
            let mut carry: u64 = 0;
            let mut f: u64 = (unsafe {
                *((*a).long_value.ob_digit.as_mut_ptr() as *mut u32).wrapping_offset(i as isize)
            }) as u64;
            let mut pz: *mut u32 = unsafe { (*z).long_value.ob_digit.as_mut_ptr() as *mut u32 }
                .wrapping_offset((i << 1i32) as isize);
            let mut pa: *mut u32 = unsafe { (*a).long_value.ob_digit.as_mut_ptr() as *mut u32 }
                .wrapping_offset(i as isize)
                .wrapping_offset(1 as isize);
            loop {
                if (unsafe { PyErr_CheckSignals() }) != 0 {
                    Py_DECREF__2!(z);
                    return core::ptr::null_mut::<_longobject>();
                }
                if true {
                    break;
                }
            }
            carry = ((unsafe { *pz }) as u64).wrapping_add(f.wrapping_mul(f));
            let __v1: *mut u32 = pz;
            pz = __v1.wrapping_offset(1 as isize);
            unsafe {
                *__v1 = (carry & 1073741823u64) as u32;
            }
            carry >>= 30i32;
            // Now f is added in twice in each column of the
            // pyramid it appears.  Same as adding f<<1 once.
            f <<= 1i32;
            while pa < paend {
                let __v7: *mut u32 = pa;
                pa = __v7.wrapping_offset(1 as isize);
                let __v9: u64 = ((unsafe { *pz }) as u64)
                    .wrapping_add(((unsafe { *__v7 }) as u64).wrapping_mul(f));
                carry = carry.wrapping_add(__v9);
                let __v12: *mut u32 = pz;
                pz = __v12.wrapping_offset(1 as isize);
                unsafe {
                    *__v12 = (carry & 1073741823u64) as u32;
                }
                carry >>= 30i32;
            }
            if carry != 0 {
                // See comment below. pz points at the highest possible
                // carry position from the last outer loop iteration, so
                // *pz is at most 1.
                carry = carry.wrapping_add((unsafe { *pz }) as u64);
                unsafe {
                    *pz = (carry & 1073741823u64) as u32;
                }
                carry >>= 30i32;
                if carry != 0 {
                    // If there's still a carry, it must be into a position
                    // * that still holds a 0. Where the base
                    // ^ B is 1 << PyLong_SHIFT, the last add was of a carry no
                    // * more than 2*B - 2 to a stored digit no more than 1.
                    // * So the sum was no more than 2*B - 1, so the current
                    // * carry no more than floor((2*B - 1)/B) = 1.
                    unsafe {
                        *pz.wrapping_offset(1 as isize) = carry as u32;
                    }
                }
            }
            i = i.wrapping_add(1);
        }
    } else {
        // a is not the same as b -- gradeschool int mult
        i = 0;
        while i < size_a {
            let mut carry: u64 = 0;
            let f: u64 = (unsafe {
                *((*a).long_value.ob_digit.as_mut_ptr() as *mut u32).wrapping_offset(i as isize)
            }) as u64;
            let mut pz: *mut u32 = unsafe { (*z).long_value.ob_digit.as_mut_ptr() as *mut u32 }
                .wrapping_offset(i as isize);
            let mut pb: *mut u32 = unsafe { (*b).long_value.ob_digit.as_mut_ptr() as *mut u32 };
            let pbend: *mut u32 = unsafe { (*b).long_value.ob_digit.as_mut_ptr() as *mut u32 }
                .wrapping_offset(size_b as isize);
            loop {
                if (unsafe { PyErr_CheckSignals() }) != 0 {
                    Py_DECREF__2!(z);
                    return core::ptr::null_mut::<_longobject>();
                }
                if true {
                    break;
                }
            }
            while pb < pbend {
                let __v25: *mut u32 = pb;
                pb = __v25.wrapping_offset(1 as isize);
                let __v27: u64 = ((unsafe { *pz }) as u64)
                    .wrapping_add(((unsafe { *__v25 }) as u64).wrapping_mul(f));
                carry = carry.wrapping_add(__v27);
                let __v30: *mut u32 = pz;
                pz = __v30.wrapping_offset(1 as isize);
                unsafe {
                    *__v30 = (carry & 1073741823u64) as u32;
                }
                carry >>= 30i32;
            }
            if carry != 0 {
                let __v36: u32 = unsafe { *pz };
                unsafe {
                    *pz = __v36.wrapping_add((carry & 1073741823u64) as u32);
                }
            }
            i = i.wrapping_add(1);
        }
    }
    long_normalize(z)
}

/// A helper for Karatsuba multiplication (k_mul).
/// Takes an int "n" and an integer "size" representing the place to
/// split, and sets low and high such that abs(n) == (high << size) + low,
/// viewing the shift as being by digits.  The sign bit is ignored, and
/// the return values are >= 0.
/// Returns 0 on success, -1 on failure.
fn kmul_split(
    n: *mut _longobject,
    size: i64,
    high: &mut *mut _longobject,
    low: &mut *mut _longobject,
) -> i32 {
    let mut size_lo: i64 = 0;
    let mut size_hi: i64 = 0;
    let size_n: i64 = _PyLong_DigitCount(n as *const _longobject);
    size_lo = Py_MIN!(size_n, size);
    size_hi = size_n.wrapping_sub(size_lo);
    let __v0: *mut _longobject = long_alloc(size_hi);
    let hi: *mut _longobject = __v0;
    if __v0.is_null() {
        return 1i32.wrapping_neg();
    }
    let __v1: *mut _longobject = long_alloc(size_lo);
    let lo: *mut _longobject = __v1;
    if __v1.is_null() {
        Py_DECREF__2!(hi);
        return 1i32.wrapping_neg();
    }
    unsafe {
        core::ptr::copy_nonoverlapping(
            ((((*n).long_value.ob_digit.as_mut_ptr() as *mut u32) as *const ())
                as *const core::ffi::c_void) as *const u8,
            ((((*lo).long_value.ob_digit.as_mut_ptr() as *mut u32) as *mut ())
                as *mut core::ffi::c_void) as *mut u8,
            ((size_lo as u64).wrapping_mul(core::mem::size_of::<u32>() as u64) as usize) as usize,
        )
    };
    unsafe {
        core::ptr::copy_nonoverlapping(
            ((((*n).long_value.ob_digit.as_mut_ptr() as *mut u32).wrapping_offset(size_lo as isize)
                as *const ()) as *const core::ffi::c_void) as *const u8,
            ((((*hi).long_value.ob_digit.as_mut_ptr() as *mut u32) as *mut ())
                as *mut core::ffi::c_void) as *mut u8,
            ((size_hi as u64).wrapping_mul(core::mem::size_of::<u32>() as u64) as usize) as usize,
        )
    };
    unsafe {
        *(high as *mut *mut _longobject) = long_normalize(hi);
    }
    unsafe {
        *(low as *mut *mut _longobject) = long_normalize(lo);
    }
    0
}

/// Karatsuba multiplication.  Ignores the input signs, and returns the
/// absolute value of the product (or NULL if error).
/// See Knuth Vol. 2 Chapter 4.3.3 (Pp. 294-295).
fn k_mul(mut a: *mut _longobject, mut b: *mut _longobject) -> *mut _longobject {
    let mut i: i64 = unsafe { core::mem::zeroed() };
    let mut t3: *mut _longobject = unsafe { core::mem::zeroed() };
    let mut t2: *mut _longobject = unsafe { core::mem::zeroed() };
    let mut t1: *mut _longobject = unsafe { core::mem::zeroed() };
    let mut ret: *mut _longobject = unsafe { core::mem::zeroed() };
    let mut bl: *mut _longobject = unsafe { core::mem::zeroed() };
    let mut bh: *mut _longobject = unsafe { core::mem::zeroed() };
    let mut al: *mut _longobject = unsafe { core::mem::zeroed() };
    let mut ah: *mut _longobject = unsafe { core::mem::zeroed() };
    let mut bsize: i64 = unsafe { core::mem::zeroed() };
    let mut asize: i64 = unsafe { core::mem::zeroed() };
    unsafe {
        asize = _PyLong_DigitCount(a as *const _longobject);
        bsize = _PyLong_DigitCount(b as *const _longobject);
        ah = core::ptr::null_mut::<_longobject>();
        al = core::ptr::null_mut::<_longobject>();
        bh = core::ptr::null_mut::<_longobject>();
        bl = core::ptr::null_mut::<_longobject>();
        ret = core::ptr::null_mut::<_longobject>();
        // (ah*X+al)(bh*X+bl) = ah*bh*X*X + (ah*bl + al*bh)*X + al*bl
        // Let k = (ah+al)*(bh+bl) = ah*bl + al*bh  + ah*bh + al*bl
        // Then the original product is
        //     ah*bh*X*X + (k - ah*bh - al*bl)*X + al*bl
        // By picking X to be a power of 2, "*X" is just shifting, and it's
        // been reduced to 3 multiplies on numbers half the size.
        // We want to split based on the larger number; fiddle so that b
        // is largest.
        if asize > bsize {
            t1 = a;
            a = b;
            b = t1;
            i = asize;
            asize = bsize;
            bsize = i;
        }
        // Use gradeschool math when either number is too small.
        i = (if a == b { 140i32 } else { 70i32 }) as i64;
        if asize <= i {
            if asize == 0 {
                return PyLong_FromLong(0i64) as *mut _longobject;
            } else {
                return x_mul(a, b);
            }
        } else {
            // If a is small compared to b, splitting on b gives a degenerate
            // case with ah==0, and Karatsuba may be (even much) less efficient
            // than "grade school" then.  However, we can still win, by viewing
            // b as a string of "big digits", each of the same width as a. That
            // leads to a sequence of balanced calls to k_mul.
            if 2i64.wrapping_mul(asize) <= bsize {
                return k_lopsided_mul(a, b);
            } else {
                '__join_0: {
                    // Split a & b into hi & lo pieces.
                    // the number of digits we split off
                    let shift: i64 = bsize >> 1i32;
                    if kmul_split(
                        a,
                        shift,
                        &mut (*core::ptr::addr_of_mut!(ah)),
                        &mut (*core::ptr::addr_of_mut!(al)),
                    ) >= 0
                    {
                        // the split isn't degenerate
                        if a == b {
                            bh = Py_NewRef__2!(ah) as *mut _longobject;
                            bl = Py_NewRef__2!(al) as *mut _longobject;
                        } else {
                            if kmul_split(
                                b,
                                shift,
                                &mut (*core::ptr::addr_of_mut!(bh)),
                                &mut (*core::ptr::addr_of_mut!(bl)),
                            ) < 0
                            {
                                break '__join_0;
                            }
                        }
                        // The plan:
                        // 1. Allocate result space (asize + bsize digits:  that's always
                        //    enough).
                        // 2. Compute ah*bh, and copy into result at 2*shift.
                        // 3. Compute al*bl, and copy into result at 0.  Note that this
                        //    can't overlap with #2.
                        // 4. Subtract al*bl from the result, starting at shift.  This may
                        //    underflow (borrow out of the high digit), but we don't care:
                        //    we're effectively doing unsigned arithmetic mod
                        //    BASE**(sizea + sizeb), and so long as the *final* result fits,
                        //    borrows and carries out of the high digit can be ignored.
                        // 5. Subtract ah*bh from the result, starting at shift.
                        // 6. Compute (ah+al)*(bh+bl), and add it into the result starting
                        //    at shift.
                        // 1. Allocate result space.
                        ret = long_alloc(asize.wrapping_add(bsize));
                        if !ret.is_null() {
                            // 2. t1 <- ah*bh, and copy into high digits of result.
                            let __v3: *mut _longobject = k_mul(ah, bh);
                            t1 = __v3;
                            if !__v3.is_null() {
                                core::ptr::copy_nonoverlapping(
                                    ((((*t1).long_value.ob_digit.as_mut_ptr() as *mut u32)
                                        as *const ())
                                        as *const core::ffi::c_void)
                                        as *const u8,
                                    ((((*ret).long_value.ob_digit.as_mut_ptr() as *mut u32)
                                        .wrapping_offset(2i64.wrapping_mul(shift) as isize)
                                        as *mut ())
                                        as *mut core::ffi::c_void)
                                        as *mut u8,
                                    ((_PyLong_DigitCount(t1 as *const _longobject) as u64)
                                        .wrapping_mul(core::mem::size_of::<u32>() as u64)
                                        as usize) as usize,
                                );
                                // Zero-out the digits higher than the ah*bh copy.
                                i = _PyLong_DigitCount(ret as *const _longobject)
                                    .wrapping_sub(2i64.wrapping_mul(shift))
                                    .wrapping_sub(_PyLong_DigitCount(t1 as *const _longobject));
                                if i != 0 {
                                    core::ptr::write_bytes(
                                        ((((*ret).long_value.ob_digit.as_mut_ptr() as *mut u32)
                                            .wrapping_offset(2i64.wrapping_mul(shift) as isize)
                                            .wrapping_offset(_PyLong_DigitCount(
                                                t1 as *const _longobject,
                                            )
                                                as isize)
                                            as *mut ())
                                            as *mut core::ffi::c_void)
                                            as *mut u8,
                                        (0i32 as i32) as u8,
                                        ((i as u64).wrapping_mul(core::mem::size_of::<u32>() as u64)
                                            as usize)
                                            as usize,
                                    );
                                }
                                // 3. t2 <- al*bl, and copy into the low digits.
                                let __v2: *mut _longobject = k_mul(al, bl);
                                t2 = __v2;
                                if __v2.is_null() {
                                    Py_DECREF__2!(t1);
                                } else {
                                    // no overlap with high digits
                                    core::ptr::copy_nonoverlapping(
                                        ((((*t2).long_value.ob_digit.as_mut_ptr() as *mut u32)
                                            as *const ())
                                            as *const core::ffi::c_void)
                                            as *const u8,
                                        ((((*ret).long_value.ob_digit.as_mut_ptr() as *mut u32)
                                            as *mut ())
                                            as *mut core::ffi::c_void)
                                            as *mut u8,
                                        ((_PyLong_DigitCount(t2 as *const _longobject) as u64)
                                            .wrapping_mul(core::mem::size_of::<u32>() as u64)
                                            as usize)
                                            as usize,
                                    );
                                    // Zero out remaining digits.
                                    i = 2i64
                                        .wrapping_mul(shift)
                                        .wrapping_sub(_PyLong_DigitCount(t2 as *const _longobject)); // number of uninitialized digits
                                    if i != 0 {
                                        core::ptr::write_bytes(
                                            ((((*ret).long_value.ob_digit.as_mut_ptr() as *mut u32)
                                                .wrapping_offset(_PyLong_DigitCount(
                                                    t2 as *const _longobject,
                                                )
                                                    as isize)
                                                as *mut ())
                                                as *mut core::ffi::c_void)
                                                as *mut u8,
                                            (0i32 as i32) as u8,
                                            ((i as u64)
                                                .wrapping_mul(core::mem::size_of::<u32>() as u64)
                                                as usize)
                                                as usize,
                                        );
                                    }
                                    // 4 & 5. Subtract ah*bh (t1) and al*bl (t2).  We do al*bl first
                                    // because it's fresher in cache.
                                    i = _PyLong_DigitCount(ret as *const _longobject)
                                        .wrapping_sub(shift); // # digits after shift
                                    v_isub(
                                        ((*ret).long_value.ob_digit.as_mut_ptr() as *mut u32)
                                            .wrapping_offset(shift as isize),
                                        i,
                                        (*t2).long_value.ob_digit.as_mut_ptr() as *mut u32,
                                        _PyLong_DigitCount(t2 as *const _longobject),
                                    );
                                    _Py_DECREF_INT(t2);
                                    v_isub(
                                        ((*ret).long_value.ob_digit.as_mut_ptr() as *mut u32)
                                            .wrapping_offset(shift as isize),
                                        i,
                                        (*t1).long_value.ob_digit.as_mut_ptr() as *mut u32,
                                        _PyLong_DigitCount(t1 as *const _longobject),
                                    );
                                    _Py_DECREF_INT(t1);
                                    // 6. t3 <- (ah+al)(bh+bl), and add into result.
                                    let __v1: *mut _longobject = x_add(ah, al);
                                    t1 = __v1;
                                    if !__v1.is_null() {
                                        _Py_DECREF_INT(ah);
                                        _Py_DECREF_INT(al);
                                        al = core::ptr::null_mut::<_longobject>();
                                        ah = core::ptr::null_mut::<_longobject>();
                                        if a == b {
                                            t2 = Py_NewRef__2!(t1) as *mut _longobject;
                                        } else {
                                            let __v0: *mut _longobject = x_add(bh, bl);
                                            t2 = __v0;
                                            if __v0.is_null() {
                                                Py_DECREF__2!(t1);
                                                break '__join_0;
                                            }
                                        }
                                        _Py_DECREF_INT(bh);
                                        _Py_DECREF_INT(bl);
                                        bl = core::ptr::null_mut::<_longobject>();
                                        bh = core::ptr::null_mut::<_longobject>();
                                        t3 = k_mul(t1, t2);
                                        _Py_DECREF_INT(t1);
                                        _Py_DECREF_INT(t2);
                                        if !t3.is_null() {
                                            // Add t3.  It's not obvious why we can't run out of room here.
                                            // See the (*) comment after this function.
                                            v_iadd(
                                                ((*ret).long_value.ob_digit.as_mut_ptr()
                                                    as *mut u32)
                                                    .wrapping_offset(shift as isize),
                                                i,
                                                (*t3).long_value.ob_digit.as_mut_ptr() as *mut u32,
                                                _PyLong_DigitCount(t3 as *const _longobject),
                                            );
                                            _Py_DECREF_INT(t3);
                                            return long_normalize(ret);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                Py_XDECREF__2!(ret);
                Py_XDECREF__2!(ah);
                Py_XDECREF__2!(al);
                Py_XDECREF__2!(bh);
                Py_XDECREF__2!(bl);
                return core::ptr::null_mut::<_longobject>();
            }
        }
    }
}

// (*) Why adding t3 can't "run out of room" above.
//
// Let f(x) mean the floor of x and c(x) mean the ceiling of x.  Some facts
// to start with:
//
// 1. For any integer i, i = c(i/2) + f(i/2).  In particular,
//    bsize = c(bsize/2) + f(bsize/2).
// 2. shift = f(bsize/2)
// 3. asize <= bsize
// 4. Since we call k_lopsided_mul if asize*2 <= bsize, asize*2 > bsize in this
//    routine, so asize > bsize/2 >= f(bsize/2) in this routine.
//
// We allocated asize + bsize result digits, and add t3 into them at an offset
// of shift.  This leaves asize+bsize-shift allocated digit positions for t3
// to fit into, = (by #1 and #2) asize + f(bsize/2) + c(bsize/2) - f(bsize/2) =
// asize + c(bsize/2) available digit positions.
//
// bh has c(bsize/2) digits, and bl at most f(size/2) digits.  So bh+hl has
// at most c(bsize/2) digits + 1 bit.
//
// If asize == bsize, ah has c(bsize/2) digits, else ah has at most f(bsize/2)
// digits, and al has at most f(bsize/2) digits in any case.  So ah+al has at
// most (asize == bsize ? c(bsize/2) : f(bsize/2)) digits + 1 bit.
//
// The product (ah+al)*(bh+bl) therefore has at most
//
//     c(bsize/2) + (asize == bsize ? c(bsize/2) : f(bsize/2)) digits + 2 bits
//
// and we have asize + c(bsize/2) available digit positions.  We need to show
// this is always enough.  An instance of c(bsize/2) cancels out in both, so
// the question reduces to whether asize digits is enough to hold
// (asize == bsize ? c(bsize/2) : f(bsize/2)) digits + 2 bits.  If asize < bsize,
// then we're asking whether asize digits >= f(bsize/2) digits + 2 bits.  By #4,
// asize is at least f(bsize/2)+1 digits, so this in turn reduces to whether 1
// digit is enough to hold 2 bits.  This is so since PyLong_SHIFT=15 >= 2.  If
// asize == bsize, then we're asking whether bsize digits is enough to hold
// c(bsize/2) digits + 2 bits, or equivalently (by #1) whether f(bsize/2) digits
// is enough to hold 2 bits.  This is so if bsize >= 2, which holds because
// bsize >= KARATSUBA_CUTOFF >= 2.
//
// Note that since there's always enough room for (ah+al)*(bh+bl), and that's
// clearly >= each of ah*bh and al*bl, there's always enough room to subtract
// ah*bh and al*bl too.
/// b has at least twice the digits of a, and a is big enough that Karatsuba
/// would pay off *if* the inputs had balanced sizes.  View b as a sequence
/// of slices, each with the same number of digits as a, and multiply the
/// slices by a, one at a time.  This gives k_mul balanced inputs to work with,
/// and is also cache-friendly (we compute one double-width slice of the result
/// at a time, then move on, never backtracking except for the helpful
/// single-width slice overlap between successive partial sums).
fn k_lopsided_mul(a: *mut _longobject, b: *mut _longobject) -> *mut _longobject {
    let mut product: *mut _longobject = unsafe { core::mem::zeroed() };
    let mut bslice: *mut _longobject = unsafe { core::mem::zeroed() };
    // # of b digits already multiplied
    let mut nbdone: i64 = unsafe { core::mem::zeroed() };
    let mut bsize: i64 = unsafe { core::mem::zeroed() };
    let mut asize: i64 = unsafe { core::mem::zeroed() };
    unsafe {
        asize = _PyLong_DigitCount(a as *const _longobject);
        bsize = _PyLong_DigitCount(b as *const _longobject);
        bslice = core::ptr::null_mut::<_longobject>();
        // Allocate result space, and zero it out.
        let ret: *mut _longobject = long_alloc(asize.wrapping_add(bsize));
        if ret.is_null() {
            return core::ptr::null_mut::<_longobject>();
        } else {
            '__join_0: {
                core::ptr::write_bytes(
                    ((((*ret).long_value.ob_digit.as_mut_ptr() as *mut u32) as *mut ())
                        as *mut core::ffi::c_void) as *mut u8,
                    (0i32 as i32) as u8,
                    ((_PyLong_DigitCount(ret as *const _longobject) as u64)
                        .wrapping_mul(core::mem::size_of::<u32>() as u64)
                        as usize) as usize,
                );
                // Successive slices of b are copied into bslice.
                bslice = long_alloc(asize);
                if !bslice.is_null() {
                    nbdone = 0;
                    while bsize > 0 {
                        let nbtouse: i64 = Py_MIN!(bsize, asize);
                        // Multiply the next slice of b by a.
                        core::ptr::copy_nonoverlapping(
                            ((((*b).long_value.ob_digit.as_mut_ptr() as *mut u32)
                                .wrapping_offset(nbdone as isize)
                                as *const ())
                                as *const core::ffi::c_void)
                                as *const u8,
                            ((((*bslice).long_value.ob_digit.as_mut_ptr() as *mut u32) as *mut ())
                                as *mut core::ffi::c_void) as *mut u8,
                            ((nbtouse as u64).wrapping_mul(core::mem::size_of::<u32>() as u64)
                                as usize) as usize,
                        );
                        _PyLong_SetSignAndDigitCount(bslice, 1i32, nbtouse);
                        product = k_mul(a, bslice);
                        if product.is_null() {
                            break '__join_0;
                        } else {
                            // Add into result.
                            v_iadd(
                                ((*ret).long_value.ob_digit.as_mut_ptr() as *mut u32)
                                    .wrapping_offset(nbdone as isize),
                                _PyLong_DigitCount(ret as *const _longobject).wrapping_sub(nbdone),
                                (*product).long_value.ob_digit.as_mut_ptr() as *mut u32,
                                _PyLong_DigitCount(product as *const _longobject),
                            );
                            _Py_DECREF_INT(product);
                            bsize = bsize.wrapping_sub(nbtouse);
                            nbdone = nbdone.wrapping_add(nbtouse);
                        }
                    }
                    _Py_DECREF_INT(bslice);
                    return long_normalize(ret);
                }
            }
            Py_DECREF__2!(ret);
            Py_XDECREF__2!(bslice);
            return core::ptr::null_mut::<_longobject>();
        }
    }
}

fn long_mul(a: *mut _longobject, b: *mut _longobject) -> *mut _longobject {
    // fast path for single-digit multiplication
    if _PyLong_BothAreCompact(a as *const _longobject, b as *const _longobject) != 0 {
        let v: i64 = medium_value!(a).wrapping_mul(medium_value!(b));
        return _PyLong_FromSTwoDigits(v);
    }
    let mut z: *mut _longobject = k_mul(a, b);
    // Negate if exactly one of the inputs is negative.
    if _PyLong_SameSign(a as *const _longobject, b as *const _longobject) == 0 && !z.is_null() {
        _PyLong_Negate(core::ptr::addr_of_mut!(z));
    }
    z
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyLong_Multiply(a: *mut _longobject, b: *mut _longobject) -> *mut _object {
    long_mul(a, b) as *mut _object
}

#[unsafe(link_section = ".text.slate_distinct.longobject.long_mul_method")]
extern "C-unwind" fn long_mul_method(a: *mut _object, b: *mut _object) -> *mut _object {
    CHECK_BINOP!(a, b);
    long_mul(a as *mut _longobject, b as *mut _longobject) as *mut _object
}

/// Fast modulo division for single-digit longs.
fn fast_mod(a: *mut _longobject, b: *mut _longobject) -> *mut _object {
    let left: i32 = (unsafe {
        *((*a).long_value.ob_digit.as_mut_ptr() as *mut u32).wrapping_offset(0 as isize)
    }) as i32;
    let right: i32 = (unsafe {
        *((*b).long_value.ob_digit.as_mut_ptr() as *mut u32).wrapping_offset(0 as isize)
    }) as i32;
    let mut r#mod: i32 = 0;
    let sign: i32 = _PyLong_CompactSign(b as *const _longobject);
    if _PyLong_SameSign(a as *const _longobject, b as *const _longobject) != 0 {
        r#mod = left % right;
    } else {
        // Either 'a' or 'b' is negative.
        r#mod = right
            .wrapping_sub(1)
            .wrapping_sub(left.wrapping_sub(1) % right);
    }
    PyLong_FromLong(r#mod.wrapping_mul(sign) as i64)
}

/// Fast floor division for single-digit longs.
fn fast_floor_div(a: *mut _longobject, b: *mut _longobject) -> *mut _object {
    let left: i32 = (unsafe {
        *((*a).long_value.ob_digit.as_mut_ptr() as *mut u32).wrapping_offset(0 as isize)
    }) as i32;
    let right: i32 = (unsafe {
        *((*b).long_value.ob_digit.as_mut_ptr() as *mut u32).wrapping_offset(0 as isize)
    }) as i32;
    // Either 'a' or 'b' is negative.
    let div: i32 = if _PyLong_SameSign(a as *const _longobject, b as *const _longobject) != 0 {
        left / right
    } else {
        1i32.wrapping_neg()
            .wrapping_sub(left.wrapping_sub(1) / right)
    };
    PyLong_FromLong(div as i64)
}

/// asymptotically faster divmod, using _pylong.py
fn pylong_int_divmod(
    v: *mut _longobject,
    w: *mut _longobject,
    pdiv: *mut *mut _longobject,
    pmod: *mut *mut _longobject,
) -> i32 {
    let r#mod: *mut _object = unsafe { PyImport_ImportModule(c"_pylong".as_ptr()) };
    if r#mod.is_null() {
        return 1i32.wrapping_neg();
    }
    let result: *mut _object =
        unsafe { PyObject_CallMethod(r#mod, c"int_divmod".as_ptr(), c"OO".as_ptr(), v, w) };
    Py_DECREF!(r#mod);
    if result.is_null() {
        return 1i32.wrapping_neg();
    }
    let __v0: bool = PyTuple_Check!(result) == 0 || PyTuple_GET_SIZE!(result) != 2;
    if __v0 {
        Py_DECREF!(result);
        unsafe {
            PyErr_SetString(
                PyExc_ValueError,
                c"tuple of length 2 is required from int_divmod()".as_ptr(),
            )
        };
        return 1i32.wrapping_neg();
    }
    let q: *mut _object = PyTuple_GET_ITEM!(result, 0i32);
    let r: *mut _object = PyTuple_GET_ITEM!(result, 1i32);
    let __v1: bool = PyLong_Check!(q) == 0 || PyLong_Check!(r) == 0;
    if __v1 {
        Py_DECREF!(result);
        unsafe {
            PyErr_SetString(
                PyExc_ValueError,
                c"tuple of int is required from int_divmod()".as_ptr(),
            )
        };
        return 1i32.wrapping_neg();
    }
    if !pdiv.is_null() {
        unsafe {
            *pdiv = Py_NewRef!(q) as *mut _longobject;
        }
    }
    if !pmod.is_null() {
        unsafe {
            *pmod = Py_NewRef!(r) as *mut _longobject;
        }
    }
    Py_DECREF!(result);
    0
}

// The / and % operators are now defined in terms of divmod().
// The expression a mod b has the value a - b*floor(a/b).
// The long_divrem function gives the remainder after division of
// |a| by |b|, with the sign of a.  This is also expressed
// as a - b*trunc(a/b), if trunc truncates towards zero.
// Some examples:
//   a           b      a rem b         a mod b
//   13          10      3               3
//  -13          10     -3               7
//   13         -10      3              -7
//  -13         -10     -3              -3
// So, to get from rem to mod, we have to add b if a and b
// have different signs.  We then subtract one from the 'div'
// part of the outcome to keep the invariant intact.
/// Compute
///     *pdiv, *pmod = divmod(v, w)
/// NULL can be passed for pdiv or pmod, in which case that part of
/// the result is simply thrown away.  The caller owns a reference to
/// each of these it requests (does not pass NULL for).
fn l_divmod(
    v: *mut _longobject,
    w: *mut _longobject,
    pdiv: *mut *mut _longobject,
    pmod: *mut *mut _longobject,
) -> i32 {
    let mut div: *mut _longobject = unsafe { core::mem::zeroed() };
    let mut r#mod: *mut _longobject = unsafe { core::mem::zeroed() };
    let __v0: bool = _PyLong_DigitCount(v as *const _longobject) == 1
        && _PyLong_DigitCount(w as *const _longobject) == 1;
    if __v0 {
        // Fast path for single-digit longs
        div = core::ptr::null_mut::<_longobject>();
        if !pdiv.is_null() {
            div = fast_floor_div(v, w) as *mut _longobject;
            if div.is_null() {
                return 1i32.wrapping_neg();
            }
        }
        if !pmod.is_null() {
            r#mod = fast_mod(v, w) as *mut _longobject;
            if r#mod.is_null() {
                Py_XDECREF__2!(div);
                return 1i32.wrapping_neg();
            }
            unsafe {
                *pmod = r#mod;
            }
        }
        if !pdiv.is_null() {
            // We only want to set `*pdiv` when `*pmod` is
            // set successfully.
            unsafe {
                *pdiv = div;
            }
        }
        return 0;
    }
    let size_v: i64 = _PyLong_DigitCount(v as *const _longobject); // digits in numerator
    let size_w: i64 = _PyLong_DigitCount(w as *const _longobject); // digits in denominator
    if size_w > 300 && size_v.wrapping_sub(size_w) > 150 {
        // Switch to _pylong.int_divmod().  If the quotient is small then
        // "schoolbook" division is linear-time so don't use in that case.
        // These limits are empirically determined and should be slightly
        // conservative so that _pylong is used in cases it is likely
        // to be faster. See Tools/scripts/divmod_threshold.py.
        return pylong_int_divmod(v, w, pdiv, pmod);
    }
    if long_divrem(
        v,
        w,
        unsafe { &mut (*core::ptr::addr_of_mut!(div)) },
        core::ptr::addr_of_mut!(r#mod),
    ) < 0
    {
        return 1i32.wrapping_neg();
    }
    let __v1: bool = _PyLong_IsNegative(r#mod as *const _longobject)
        && _PyLong_IsPositive(w as *const _longobject);
    let __v2: bool = __v1
        || _PyLong_IsPositive(r#mod as *const _longobject)
            && _PyLong_IsNegative(w as *const _longobject);
    if __v2 {
        let mut temp: *mut _longobject = unsafe { core::mem::zeroed() };
        temp = long_add(r#mod, w);
        Py_SETREF__4!(r#mod, temp);
        if r#mod.is_null() {
            Py_DECREF__2!(div);
            return 1i32.wrapping_neg();
        }
        temp = long_sub(div, _PyLong_GetOne() as *mut _longobject);
        if temp.is_null() {
            Py_DECREF__2!(r#mod);
            Py_DECREF__2!(div);
            return 1i32.wrapping_neg();
        }
        Py_SETREF__4!(div, temp);
    }
    if !pdiv.is_null() {
        unsafe {
            *pdiv = div;
        }
    } else {
        Py_DECREF__2!(div);
    }
    if !pmod.is_null() {
        unsafe {
            *pmod = r#mod;
        }
    } else {
        Py_DECREF__2!(r#mod);
    }
    0
}

/// Compute
///     *pmod = v % w
/// pmod cannot be NULL. The caller owns a reference to pmod.
fn l_mod(v: *mut _longobject, w: *mut _longobject, pmod: &mut *mut _longobject) -> i32 {
    let mut r#mod: *mut _longobject = unsafe { core::mem::zeroed() };
    let __v0: bool = _PyLong_DigitCount(v as *const _longobject) == 1
        && _PyLong_DigitCount(w as *const _longobject) == 1;
    if __v0 {
        // Fast path for single-digit longs
        unsafe {
            *(pmod as *mut *mut _longobject) = fast_mod(v, w) as *mut _longobject;
        }
        return (unsafe { *(pmod as *mut *mut _longobject) }.is_null() as i32).wrapping_neg();
    }
    if long_rem(v, w, core::ptr::addr_of_mut!(r#mod)) < 0 {
        return 1i32.wrapping_neg();
    }
    let __v1: bool = _PyLong_IsNegative(r#mod as *const _longobject)
        && _PyLong_IsPositive(w as *const _longobject);
    let __v2: bool = __v1
        || _PyLong_IsPositive(r#mod as *const _longobject)
            && _PyLong_IsNegative(w as *const _longobject);
    if __v2 {
        let temp: *mut _longobject = long_add(r#mod, w);
        Py_SETREF__4!(r#mod, temp);
        if r#mod.is_null() {
            return 1i32.wrapping_neg();
        }
    }
    unsafe {
        *(pmod as *mut *mut _longobject) = r#mod;
    }
    0
}

#[unsafe(link_section = ".text.slate_distinct.longobject.long_div")]
extern "C-unwind" fn long_div(a: *mut _object, b: *mut _object) -> *mut _object {
    let mut div: *mut _longobject = unsafe { core::mem::zeroed() };
    CHECK_BINOP!(a, b);
    let __v2: bool = _PyLong_DigitCount(a as *const _longobject) == 1
        && _PyLong_DigitCount(b as *const _longobject) == 1;
    if __v2 {
        return fast_floor_div(a as *mut _longobject, b as *mut _longobject);
    }
    if l_divmod(
        a as *mut _longobject,
        b as *mut _longobject,
        core::ptr::addr_of_mut!(div),
        core::ptr::null_mut::<*mut _longobject>(),
    ) < 0
    {
        div = core::ptr::null_mut::<_longobject>();
    }
    div as *mut _object
}

// PyLong/PyLong -> float, with correctly rounded result.
#[unsafe(link_section = ".text.slate_distinct.longobject.long_true_divide")]
extern "C-unwind" fn long_true_divide(v: *mut _object, w: *mut _object) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    // Check whether ldexp result will overflow a double.
    let mut __v1: bool = unsafe { core::mem::zeroed() };
    let mut rem_1: *mut _longobject = unsafe { core::mem::zeroed() };
    let mut shift_digits: i64 = unsafe { core::mem::zeroed() };
    let mut i: i64 = unsafe { core::mem::zeroed() };
    let mut shift_digits_1: i64 = unsafe { core::mem::zeroed() };
    let mut db: f64 = unsafe { core::mem::zeroed() };
    let mut da: f64 = unsafe { core::mem::zeroed() };
    let mut result: f64 = unsafe { core::mem::zeroed() };
    let mut dx: f64 = unsafe { core::mem::zeroed() };
    let mut negate: i32 = unsafe { core::mem::zeroed() };
    let mut inexact: i32 = unsafe { core::mem::zeroed() };
    let mut low: u32 = unsafe { core::mem::zeroed() };
    let mut mask: u32 = unsafe { core::mem::zeroed() };
    let mut x_bits: i64 = unsafe { core::mem::zeroed() };
    let mut x_size: i64 = unsafe { core::mem::zeroed() };
    let mut diff: i64 = unsafe { core::mem::zeroed() };
    let mut shift: i64 = unsafe { core::mem::zeroed() };
    let mut b_size: i64 = unsafe { core::mem::zeroed() };
    let mut a_size: i64 = unsafe { core::mem::zeroed() };
    let mut x: *mut _longobject = unsafe { core::mem::zeroed() };
    let mut b: *mut _longobject = unsafe { core::mem::zeroed() };
    let mut a: *mut _longobject = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_0: {
            CHECK_BINOP!(v, w);
            a = v as *mut _longobject;
            b = w as *mut _longobject;
            // Method in a nutshell:
            //
            //   0. reduce to case a, b > 0; filter out obvious underflow/overflow
            //   1. choose a suitable integer 'shift'
            //   2. use integer arithmetic to compute x = floor(2**-shift*a/b)
            //   3. adjust x for correct rounding
            //   4. convert x to a double dx with the same value
            //   5. return ldexp(dx, shift).
            //
            // In more detail:
            //
            // 0. For any a, a/0 raises ZeroDivisionError; for nonzero b, 0/b
            // returns either 0.0 or -0.0, depending on the sign of b.  For a and
            // b both nonzero, ignore signs of a and b, and add the sign back in
            // at the end.  Now write a_bits and b_bits for the bit lengths of a
            // and b respectively (that is, a_bits = 1 + floor(log_2(a)); likewise
            // for b).  Then
            //
            //    2**(a_bits - b_bits - 1) < a/b < 2**(a_bits - b_bits + 1).
            //
            // So if a_bits - b_bits > DBL_MAX_EXP then a/b > 2**DBL_MAX_EXP and
            // so overflows.  Similarly, if a_bits - b_bits < DBL_MIN_EXP -
            // DBL_MANT_DIG - 1 then a/b underflows to 0.  With these cases out of
            // the way, we can assume that
            //
            //    DBL_MIN_EXP - DBL_MANT_DIG - 1 <= a_bits - b_bits <= DBL_MAX_EXP.
            //
            // 1. The integer 'shift' is chosen so that x has the right number of
            // bits for a double, plus two or three extra bits that will be used
            // in the rounding decisions.  Writing a_bits and b_bits for the
            // number of significant bits in a and b respectively, a
            // straightforward formula for shift is:
            //
            //    shift = a_bits - b_bits - DBL_MANT_DIG - 2
            //
            // This is fine in the usual case, but if a/b is smaller than the
            // smallest normal float then it can lead to double rounding on an
            // IEEE 754 platform, giving incorrectly rounded results.  So we
            // adjust the formula slightly.  The actual formula used is:
            //
            //     shift = MAX(a_bits - b_bits, DBL_MIN_EXP) - DBL_MANT_DIG - 2
            //
            // 2. The quantity x is computed by first shifting a (left -shift bits
            // if shift <= 0, right shift bits if shift > 0) and then dividing by
            // b.  For both the shift and the division, we keep track of whether
            // the result is inexact, in a flag 'inexact'; this information is
            // needed at the rounding stage.
            //
            // With the choice of shift above, together with our assumption that
            // a_bits - b_bits >= DBL_MIN_EXP - DBL_MANT_DIG - 1, it follows
            // that x >= 1.
            //
            // 3. Now x * 2**shift <= a/b < (x+1) * 2**shift.  We want to replace
            // this with an exactly representable float of the form
            //
            //    round(x/2**extra_bits) * 2**(extra_bits+shift).
            //
            // For float representability, we need x/2**extra_bits <
            // 2**DBL_MANT_DIG and extra_bits + shift >= DBL_MIN_EXP -
            // DBL_MANT_DIG.  This translates to the condition:
            //
            //    extra_bits >= MAX(x_bits, DBL_MIN_EXP - shift) - DBL_MANT_DIG
            //
            // To round, we just modify the bottom digit of x in-place; this can
            // end up giving a digit with value > PyLONG_MASK, but that's not a
            // problem since digits can hold values up to 2*PyLONG_MASK+1.
            //
            // With the original choices for shift above, extra_bits will always
            // be 2 or 3.  Then rounding under the round-half-to-even rule, we
            // round up iff the most significant of the extra bits is 1, and
            // either: (a) the computation of x in step 2 had an inexact result,
            // or (b) at least one other of the extra bits is 1, or (c) the least
            // significant bit of x (above those to be rounded) is 1.
            //
            // 4. Conversion to a double is straightforward; all floating-point
            // operations involved in the conversion are exact, so there's no
            // danger of rounding errors.
            //
            // 5. Use ldexp(x, shift) to compute x*2**shift, the final result.
            // The result will always be exactly representable as a double, except
            // in the case that it overflows.  To avoid dependence on the exact
            // behaviour of ldexp on overflow, we check for overflow before
            // applying ldexp.  The result of ldexp is adjusted for sign before
            // returning.
            // Reduce to case where a and b are both positive.
            a_size = _PyLong_DigitCount(a as *const _longobject);
            b_size = _PyLong_DigitCount(b as *const _longobject);
            negate = ((_PyLong_IsNegative(a as *const _longobject) as i32)
                != (_PyLong_IsNegative(b as *const _longobject) as i32))
                as i32;
            if b_size == 0 {
                PyErr_SetString(PyExc_ZeroDivisionError, c"division by zero".as_ptr());
            } else {
                '__join_2: {
                    if a_size != 0 {
                        '__join_3: {
                            // Fast path for a and b small (exactly representable in a double).
                            // Relies on floating-point division being correctly rounded; results
                            // may be subject to double rounding on x86 machines that operate with
                            // the x87 FPU set to 64-bit precision.
                            let a_is_small: i32 = (a_size <= 1i64
                                || a_size == 2i64
                                    && *((*a).long_value.ob_digit.as_mut_ptr() as *mut u32)
                                        .wrapping_offset(1i32 as isize)
                                        >> 23i32
                                        == 0)
                                as i32;
                            let b_is_small: i32 = (b_size <= 1i64
                                || b_size == 2i64
                                    && *((*b).long_value.ob_digit.as_mut_ptr() as *mut u32)
                                        .wrapping_offset(1i32 as isize)
                                        >> 23i32
                                        == 0)
                                as i32;
                            if a_is_small != 0 && b_is_small != 0 {
                                let __v26: i64 = a_size.wrapping_sub(1);
                                a_size = __v26;
                                da = *((*a).long_value.ob_digit.as_mut_ptr() as *mut u32)
                                    .wrapping_offset(__v26 as isize)
                                    as f64;
                                while a_size > 0 {
                                    let __v24: i64 = a_size.wrapping_sub(1);
                                    a_size = __v24;
                                    da = da * (1073741824u32 as f64)
                                        + (*((*a).long_value.ob_digit.as_mut_ptr() as *mut u32)
                                            .wrapping_offset(__v24 as isize)
                                            as f64);
                                }
                                let __v22: i64 = b_size.wrapping_sub(1);
                                b_size = __v22;
                                db = *((*b).long_value.ob_digit.as_mut_ptr() as *mut u32)
                                    .wrapping_offset(__v22 as isize)
                                    as f64;
                                while b_size > 0 {
                                    let __v20: i64 = b_size.wrapping_sub(1);
                                    b_size = __v20;
                                    db = db * (1073741824u32 as f64)
                                        + (*((*b).long_value.ob_digit.as_mut_ptr() as *mut u32)
                                            .wrapping_offset(__v20 as isize)
                                            as f64);
                                }
                                result = da / db;
                            } else {
                                // Catch obvious cases of underflow and overflow
                                diff = a_size.wrapping_sub(b_size);
                                if diff <= 307445734561825859i64 {
                                    if diff < -307445734561825859i64 {
                                        break '__join_2;
                                    } else {
                                        // Extreme overflow
                                        // Extreme underflow
                                        // Next line is now safe from overflowing a Py_ssize_t
                                        diff = diff
                                            .wrapping_mul(30)
                                            .wrapping_add(bit_length_digit(
                                                *((*a).long_value.ob_digit.as_mut_ptr()
                                                    as *mut u32)
                                                    .wrapping_offset(
                                                        a_size.wrapping_sub(1) as isize
                                                    ),
                                            )
                                                as i64)
                                            .wrapping_sub(bit_length_digit(
                                                *((*b).long_value.ob_digit.as_mut_ptr()
                                                    as *mut u32)
                                                    .wrapping_offset(
                                                        b_size.wrapping_sub(1) as isize
                                                    ),
                                            )
                                                as i64);
                                        // Now diff = a_bits - b_bits.
                                        if diff <= 1024 {
                                            if diff
                                                < (1021i32
                                                    .wrapping_neg()
                                                    .wrapping_sub(53)
                                                    .wrapping_sub(1)
                                                    as i64)
                                            {
                                                break '__join_2;
                                            } else {
                                                // Choose value for shift; see comments for step 1 above.
                                                shift = Py_MAX__2!(diff, 1021i32.wrapping_neg())
                                                    .wrapping_sub(53)
                                                    .wrapping_sub(2);
                                                inexact = 0;
                                                // x = abs(a * 2**-shift)
                                                if shift <= 0 {
                                                    shift_digits = shift.wrapping_neg() / 30;
                                                    // x = a << -shift
                                                    if a_size
                                                        >= 9223372036854775806i64
                                                            .wrapping_sub(shift_digits)
                                                    {
                                                        // In practice, it's probably impossible to end up
                                                        // here.  Both a and b would have to be enormous,
                                                        // using close to SIZE_T_MAX bytes of memory each.
                                                        PyErr_SetString(PyExc_OverflowError, c"intermediate overflow during division".as_ptr());
                                                        break '__join_0;
                                                    } else {
                                                        x = long_alloc(
                                                            a_size
                                                                .wrapping_add(shift_digits)
                                                                .wrapping_add(1),
                                                        );
                                                        if x.is_null() {
                                                            break '__join_0;
                                                        } else {
                                                            i = 0;
                                                            while i < shift_digits {
                                                                *((*x)
                                                                    .long_value
                                                                    .ob_digit
                                                                    .as_mut_ptr()
                                                                    as *mut u32)
                                                                    .wrapping_offset(i as isize) =
                                                                    0;
                                                                i = i.wrapping_add(1);
                                                            }
                                                            let rem: u32 = v_lshift(
                                                                ((*x)
                                                                    .long_value
                                                                    .ob_digit
                                                                    .as_mut_ptr()
                                                                    as *mut u32)
                                                                    .wrapping_offset(
                                                                        shift_digits as isize,
                                                                    ),
                                                                (*a).long_value
                                                                    .ob_digit
                                                                    .as_mut_ptr()
                                                                    as *mut u32,
                                                                a_size,
                                                                (shift.wrapping_neg() % 30) as i32,
                                                            );
                                                            *((*x).long_value.ob_digit.as_mut_ptr() as *mut u32).wrapping_offset(a_size.wrapping_add(shift_digits) as isize) = rem;
                                                        }
                                                    }
                                                } else {
                                                    shift_digits_1 = shift / 30;
                                                    // x = a >> shift
                                                    x = long_alloc(
                                                        a_size.wrapping_sub(shift_digits_1),
                                                    );
                                                    if x.is_null() {
                                                        break '__join_0;
                                                    } else {
                                                        let rem: u32 = v_rshift(
                                                            (*x).long_value.ob_digit.as_mut_ptr()
                                                                as *mut u32,
                                                            ((*a).long_value.ob_digit.as_mut_ptr()
                                                                as *mut u32)
                                                                .wrapping_offset(
                                                                    shift_digits_1 as isize,
                                                                ),
                                                            a_size.wrapping_sub(shift_digits_1),
                                                            (shift % 30) as i32,
                                                        );
                                                        // set inexact if any of the bits shifted out is nonzero
                                                        if rem != 0 {
                                                            inexact = 1;
                                                        }
                                                        while inexact == 0 && shift_digits_1 > 0 {
                                                            let __v16: i64 =
                                                                shift_digits_1.wrapping_sub(1);
                                                            shift_digits_1 = __v16;
                                                            if *((*a)
                                                                .long_value
                                                                .ob_digit
                                                                .as_mut_ptr()
                                                                as *mut u32)
                                                                .wrapping_offset(__v16 as isize)
                                                                != 0
                                                            {
                                                                inexact = 1;
                                                            }
                                                        }
                                                    }
                                                }
                                                long_normalize(x);
                                                x_size = _PyLong_SignedDigitCount(
                                                    x as *const _longobject,
                                                );
                                                // x //= b. If the remainder is nonzero, set inexact.  We own the only
                                                // reference to x, so it's safe to modify it in-place.
                                                if b_size == 1 {
                                                    let rem: u32 = inplace_divrem1(
                                                        (*x).long_value.ob_digit.as_mut_ptr()
                                                            as *mut u32,
                                                        (*x).long_value.ob_digit.as_mut_ptr()
                                                            as *mut u32,
                                                        x_size,
                                                        *((*b).long_value.ob_digit.as_mut_ptr()
                                                            as *mut u32)
                                                            .wrapping_offset(0 as isize),
                                                    );
                                                    long_normalize(x);
                                                    if rem != 0 {
                                                        inexact = 1;
                                                    }
                                                } else {
                                                    let div: *mut _longobject = x_divrem(
                                                        x,
                                                        b,
                                                        core::ptr::addr_of_mut!(rem_1),
                                                    );
                                                    Py_SETREF__4!(x, div);
                                                    if x.is_null() {
                                                        break '__join_0;
                                                    } else {
                                                        if !_PyLong_IsZero(
                                                            rem_1 as *const _longobject,
                                                        ) {
                                                            inexact = 1;
                                                        }
                                                        Py_DECREF__2!(rem_1);
                                                    }
                                                }
                                                x_size =
                                                    _PyLong_DigitCount(x as *const _longobject);
                                                // result of division is never zero
                                                x_bits = x_size
                                                    .wrapping_sub(1)
                                                    .wrapping_mul(30)
                                                    .wrapping_add(bit_length_digit(
                                                        *((*x).long_value.ob_digit.as_mut_ptr()
                                                            as *mut u32)
                                                            .wrapping_offset(
                                                                x_size.wrapping_sub(1) as isize,
                                                            ),
                                                    )
                                                        as i64);
                                                // The number of extra bits that have to be rounded away.
                                                let extra_bits: i64 = Py_MAX!(
                                                    x_bits,
                                                    (1021i32.wrapping_neg() as i64)
                                                        .wrapping_sub(shift)
                                                )
                                                .wrapping_sub(53);
                                                // Round by directly modifying the low digit of x.
                                                mask = 1u32 << extra_bits.wrapping_sub(1);
                                                low = *((*x).long_value.ob_digit.as_mut_ptr()
                                                    as *mut u32)
                                                    .wrapping_offset(0 as isize)
                                                    | (inexact as u32);
                                                if low & mask != 0
                                                    && low & 3u32.wrapping_mul(mask).wrapping_sub(1)
                                                        != 0
                                                {
                                                    low = low.wrapping_add(mask);
                                                }
                                                *((*x).long_value.ob_digit.as_mut_ptr()
                                                    as *mut u32)
                                                    .wrapping_offset(0 as isize) =
                                                    low & !2u32.wrapping_mul(mask).wrapping_sub(1);
                                                // Convert x to a double dx; the conversion is exact.
                                                let __v4: i64 = x_size.wrapping_sub(1);
                                                x_size = __v4;
                                                dx = *((*x).long_value.ob_digit.as_mut_ptr()
                                                    as *mut u32)
                                                    .wrapping_offset(__v4 as isize)
                                                    as f64;
                                                while x_size > 0 {
                                                    let __v2: i64 = x_size.wrapping_sub(1);
                                                    x_size = __v2;
                                                    dx = dx * (1073741824u32 as f64)
                                                        + (*((*x).long_value.ob_digit.as_mut_ptr()
                                                            as *mut u32)
                                                            .wrapping_offset(__v2 as isize)
                                                            as f64);
                                                }
                                                Py_DECREF__2!(x);
                                                if shift.wrapping_add(x_bits) >= 1024 {
                                                    if shift.wrapping_add(x_bits) > 1024 {
                                                        __v0 = true;
                                                    } else {
                                                        __v0 = dx == ldexp(1.0f64, x_bits as i32);
                                                    }
                                                    __v1 = __v0;
                                                } else {
                                                    __v1 = false;
                                                }
                                                if !__v1 {
                                                    result = ldexp(dx, shift as i32);
                                                    break '__join_3;
                                                }
                                            }
                                        }
                                    }
                                }
                                PyErr_SetString(
                                    PyExc_OverflowError,
                                    c"integer division result too large for a float".as_ptr(),
                                );
                                break '__join_0;
                            }
                        }
                        return PyFloat_FromDouble(if negate != 0 { -result } else { result });
                    }
                }
                return PyFloat_FromDouble(if negate != 0 { -0.0f64 } else { 0.0f64 });
            }
        }
        return core::ptr::null_mut::<_object>();
    }
}

#[unsafe(link_section = ".text.slate_distinct.longobject.long_mod")]
extern "C-unwind" fn long_mod(a: *mut _object, b: *mut _object) -> *mut _object {
    let mut r#mod: *mut _longobject = unsafe { core::mem::zeroed() };
    CHECK_BINOP!(a, b);
    if l_mod(a as *mut _longobject, b as *mut _longobject, unsafe {
        &mut (*core::ptr::addr_of_mut!(r#mod))
    }) < 0
    {
        r#mod = core::ptr::null_mut::<_longobject>();
    }
    r#mod as *mut _object
}

#[unsafe(link_section = ".text.slate_distinct.longobject.long_divmod")]
extern "C-unwind" fn long_divmod(a: *mut _object, b: *mut _object) -> *mut _object {
    let mut div: *mut _longobject = unsafe { core::mem::zeroed() };
    let mut r#mod: *mut _longobject = unsafe { core::mem::zeroed() };
    CHECK_BINOP!(a, b);
    if l_divmod(
        a as *mut _longobject,
        b as *mut _longobject,
        core::ptr::addr_of_mut!(div),
        core::ptr::addr_of_mut!(r#mod),
    ) < 0
    {
        return core::ptr::null_mut::<_object>();
    }
    let z: *mut _object = unsafe { PyTuple_New(2i64) };
    if !z.is_null() {
        PyTuple_SET_ITEM!(z, 0i32, div as *mut _object);
        PyTuple_SET_ITEM!(z, 1i32, r#mod as *mut _object);
    } else {
        Py_DECREF__2!(div);
        Py_DECREF__2!(r#mod);
    }
    z
}

// Compute an inverse to a modulo n, or raise ValueError if a is not
// invertible modulo n. Assumes n is positive. The inverse returned
// is whatever falls out of the extended Euclidean algorithm: it may
// be either positive or negative, but will be smaller than n in
// absolute value.
//
// Pure Python equivalent for long_invmod:
//
//      def invmod(a, n):
//          b, c = 1, 0
//          while n:
//              q, r = divmod(a, n)
//              a, b, c, n = n, c, b - q*c, r
//
//          # at this point a is the gcd of the original inputs
//          if a == 1:
//              return b
//          raise ValueError("Not invertible")
fn long_invmod(mut a: *mut _longobject, mut n: *mut _longobject) -> *mut _longobject {
    let mut t: *mut _longobject = unsafe { core::mem::zeroed() };
    let mut s: *mut _longobject = unsafe { core::mem::zeroed() };
    let mut r: *mut _longobject = unsafe { core::mem::zeroed() };
    let mut q: *mut _longobject = unsafe { core::mem::zeroed() };
    let mut c: *mut _longobject = unsafe { core::mem::zeroed() };
    let mut b: *mut _longobject = unsafe { core::mem::zeroed() };
    unsafe {
        // Should only ever be called for positive n
        Py_INCREF__2!(a);
        b = Py_NewRef!(_PyLong_GetOne()) as *mut _longobject;
        c = Py_NewRef!(_PyLong_GetZero()) as *mut _longobject;
        Py_INCREF__2!(n);
        // references now owned: a, b, c, n
        '__join_0: {
            while !_PyLong_IsZero(n as *const _longobject) {
                if l_divmod(a, n, core::ptr::addr_of_mut!(q), core::ptr::addr_of_mut!(r))
                    == 1i32.wrapping_neg()
                {
                    break '__join_0;
                } else {
                    Py_SETREF__4!(a, n);
                    n = r;
                    t = long_mul(q, c);
                    Py_DECREF__2!(q);
                    if t.is_null() {
                        break '__join_0;
                    } else {
                        s = long_sub(b, t);
                        Py_DECREF__2!(t);
                        if s.is_null() {
                            break '__join_0;
                        } else {
                            Py_SETREF__4!(b, c);
                            c = s;
                        }
                    }
                }
            }
            // references now owned: a, b, c, n
            Py_DECREF__2!(c);
            Py_DECREF__2!(n);
            if long_compare(a, _PyLong_GetOne() as *mut _longobject) != 0 {
                // a != 1; we don't have an inverse.
                Py_DECREF__2!(a);
                Py_DECREF__2!(b);
                PyErr_SetString(
                    PyExc_ValueError,
                    c"base is not invertible for the given modulus".as_ptr(),
                );
                return core::ptr::null_mut::<_longobject>();
            } else {
                // a == 1; b gives an inverse modulo n
                Py_DECREF__2!(a);
                return b;
            }
        }
        Py_DECREF__2!(a);
        Py_DECREF__2!(b);
        Py_DECREF__2!(c);
        Py_DECREF__2!(n);
        return core::ptr::null_mut::<_longobject>();
    }
}

/// pow(v, w, x)
#[unsafe(link_section = ".text.slate_distinct.longobject.long_pow")]
extern "C-unwind" fn long_pow(v: *mut _object, w: *mut _object, x: *mut _object) -> *mut _object {
    let mut __v2: bool = unsafe { core::mem::zeroed() };
    let mut ntz: i32 = unsafe { core::mem::zeroed() };
    let mut ntz_1: i32 = unsafe { core::mem::zeroed() };
    let mut bi: u32 = unsafe { core::mem::zeroed() };
    let mut blen: i32 = unsafe { core::mem::zeroed() };
    // Repeatedly extract the next (no more than) EXP_WINDOW_SIZE bits
    // into `pending`, starting with the next 1 bit.  The current bit
    // length of `pending` is `blen`.
    let mut pending: i32 = unsafe { core::mem::zeroed() };
    let mut bit_1: u32 = unsafe { core::mem::zeroed() };
    let mut bi_1: u32 = unsafe { core::mem::zeroed() };
    // Reduce base by modulus in some cases:
    // 1. If base < 0.  Forcing the base non-negative makes things easier.
    // 2. If base is obviously larger than the modulus.  The "small
    //    exponent" case later can multiply directly by base repeatedly,
    //    while the "large exponent" case multiplies directly by base 31
    //    times.  It can be unboundedly faster to multiply by
    //    base % modulus instead.
    // We could _always_ do this reduction, but l_mod() isn't cheap,
    // so we only do it when it buys something.
    let mut __v53: bool = unsafe { core::mem::zeroed() };
    let mut num_table_entries: i64 = unsafe { core::mem::zeroed() };
    // k-ary values.  If the exponent is large enough, table is
    // precomputed so that table[i] == a**(2*i+1) % c for i in
    // range(EXP_TABLE_LEN).
    // Note: this is uninitialized stack trash: don't pay to set it to known
    // values unless it's needed. Instead ensure that num_table_entries is
    // set to the number of entries actually filled whenever a branch to the
    // Error or Done labels is possible.
    let mut table: core::mem::MaybeUninit<__SlateAlign16<[*mut _longobject; 16]>> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_55: *mut [*mut _longobject; 16] =
        core::ptr::addr_of_mut!(table) as *mut [*mut _longobject; 16]; // may temporarily hold a**2 % c
    let mut a2: *mut _longobject = unsafe { core::mem::zeroed() };
    let mut temp: *mut _longobject = unsafe { core::mem::zeroed() };
    // counters
    let mut j: i64 = unsafe { core::mem::zeroed() };
    let mut i: i64 = unsafe { core::mem::zeroed() };
    // accumulated result
    let mut z: *mut _longobject = unsafe { core::mem::zeroed() };
    // if x<0 return negative output
    let mut negativeOutput: i32 = unsafe { core::mem::zeroed() };
    // a,b,c = v,w,x
    let mut c: *mut _longobject = unsafe { core::mem::zeroed() };
    let mut b: *mut _longobject = unsafe { core::mem::zeroed() };
    let mut a: *mut _longobject = unsafe { core::mem::zeroed() };
    unsafe {
        negativeOutput = 0;
        z = core::ptr::null_mut::<_longobject>();
        temp = core::ptr::null_mut::<_longobject>();
        a2 = core::ptr::null_mut::<_longobject>();
        num_table_entries = 0;
        // a, b, c = v, w, x
        CHECK_BINOP!(v, w);
        a = Py_NewRef!(v) as *mut _longobject;
        b = Py_NewRef!(w) as *mut _longobject;
        if PyLong_Check!(x) != 0 {
            c = Py_NewRef!(x) as *mut _longobject;
        } else {
            if x == core::ptr::addr_of_mut!(_Py_NoneStruct) {
                c = core::ptr::null_mut::<_longobject>();
            } else {
                Py_DECREF__2!(a);
                Py_DECREF__2!(b);
                return core::ptr::addr_of_mut!(_Py_NotImplementedStruct);
            }
        }
        if _PyLong_IsNegative(b as *const _longobject) && c.is_null() {
            // if exponent is negative and there's no modulus:
            // return a float.  This works because we know
            // that this calls float_pow() which converts its
            // arguments to double.
            Py_DECREF__2!(a);
            Py_DECREF__2!(b);
            return unsafe { (*PyFloat_Type.tp_as_number).nb_power }.unwrap()(v, w, x);
        } else {
            '__join_3: {
                '__join_4: {
                    if !c.is_null() {
                        // if modulus == 0:
                        // raise ValueError()
                        if _PyLong_IsZero(c as *const _longobject) {
                            PyErr_SetString(
                                PyExc_ValueError,
                                c"pow() 3rd argument cannot be 0".as_ptr(),
                            );
                            break '__join_4;
                        } else {
                            // if modulus < 0:
                            // negativeOutput = True
                            // modulus = -modulus
                            if _PyLong_IsNegative(c as *const _longobject) {
                                negativeOutput = 1;
                                temp = _PyLong_Copy(c) as *mut _longobject;
                                if temp.is_null() {
                                    break '__join_4;
                                } else {
                                    Py_SETREF__4!(c, temp);
                                    temp = core::ptr::null_mut::<_longobject>();
                                    _PyLong_Negate(core::ptr::addr_of_mut!(c));
                                    if c.is_null() {
                                        break '__join_4;
                                    }
                                }
                            }
                            // if modulus == 1:
                            // return 0
                            if _PyLong_IsNonNegativeCompact(c as *const _longobject) != 0
                                && *((*c).long_value.ob_digit.as_mut_ptr() as *mut u32)
                                    .wrapping_offset(0 as isize)
                                    == 1
                            {
                                z = PyLong_FromLong(0i64) as *mut _longobject;
                                break '__join_3;
                            } else {
                                // if exponent is negative, negate the exponent and
                                // replace the base with a modular inverse
                                if _PyLong_IsNegative(b as *const _longobject) {
                                    temp = _PyLong_Copy(b) as *mut _longobject;
                                    if temp.is_null() {
                                        break '__join_4;
                                    } else {
                                        Py_SETREF__4!(b, temp);
                                        temp = core::ptr::null_mut::<_longobject>();
                                        _PyLong_Negate(core::ptr::addr_of_mut!(b));
                                        if b.is_null() {
                                            break '__join_4;
                                        } else {
                                            temp = long_invmod(a, c);
                                            if temp.is_null() {
                                                break '__join_4;
                                            } else {
                                                Py_SETREF__4!(a, temp);
                                                temp = core::ptr::null_mut::<_longobject>();
                                            }
                                        }
                                    }
                                }
                                if _PyLong_IsNegative(a as *const _longobject) {
                                    __v53 = true;
                                } else {
                                    __v53 = _PyLong_DigitCount(a as *const _longobject)
                                        > _PyLong_DigitCount(c as *const _longobject);
                                }
                                if __v53 {
                                    if l_mod(a, c, &mut (*core::ptr::addr_of_mut!(temp))) < 0 {
                                        break '__join_4;
                                    } else {
                                        Py_SETREF__4!(a, temp);
                                        temp = core::ptr::null_mut::<_longobject>();
                                    }
                                }
                            }
                        }
                    }
                    // At this point a, b, and c are guaranteed non-negative UNLESS
                    // c is NULL, in which case a may be negative.
                    z = PyLong_FromLong(1i64) as *mut _longobject;
                    if !z.is_null() {
                        // Perform a modular reduction, X = X % c, but leave X alone if c
                        // is NULL.
                        // Multiply two values, then reduce the result:
                        // result = X*Y % c.  If c is NULL, skip the mod.
                        i = _PyLong_SignedDigitCount(b as *const _longobject);
                        bi_1 = if i != 0 {
                            *((*b).long_value.ob_digit.as_mut_ptr() as *mut u32)
                                .wrapping_offset(i.wrapping_sub(1) as isize)
                        } else {
                            0
                        };
                        if i <= 1 && bi_1 <= 3 {
                            // aim for minimal overhead
                            if bi_1 >= 2 {
                                loop {
                                    temp = long_mul(a, a);
                                    if temp.is_null() {
                                        break '__join_4;
                                    } else {
                                        Py_XDECREF__2!(z);
                                        z = temp;
                                        temp = core::ptr::null_mut::<_longobject>();
                                        loop {
                                            if !c.is_null() {
                                                if l_mod(
                                                    z,
                                                    c,
                                                    &mut (*core::ptr::addr_of_mut!(temp)),
                                                ) < 0
                                                {
                                                    break '__join_4;
                                                } else {
                                                    Py_XDECREF__2!(z);
                                                    z = temp;
                                                    temp = core::ptr::null_mut::<_longobject>();
                                                }
                                            }
                                            if true {
                                                break;
                                            }
                                        }
                                        if true {
                                            break;
                                        }
                                    }
                                }
                                if bi_1 == 3 {
                                    loop {
                                        temp = long_mul(z, a);
                                        if temp.is_null() {
                                            break '__join_4;
                                        } else {
                                            Py_XDECREF__2!(z);
                                            z = temp;
                                            temp = core::ptr::null_mut::<_longobject>();
                                            loop {
                                                if !c.is_null() {
                                                    if l_mod(
                                                        z,
                                                        c,
                                                        &mut (*core::ptr::addr_of_mut!(temp)),
                                                    ) < 0
                                                    {
                                                        break '__join_4;
                                                    } else {
                                                        Py_XDECREF__2!(z);
                                                        z = temp;
                                                        temp = core::ptr::null_mut::<_longobject>();
                                                    }
                                                }
                                                if true {
                                                    break;
                                                }
                                            }
                                            if true {
                                                break;
                                            }
                                        }
                                    }
                                }
                            } else {
                                if bi_1 == 1 {
                                    // Multiplying by 1 serves two purposes: if `a` is of an int
                                    // subclass, makes the result an int (e.g., pow(False, 1) returns
                                    // 0 instead of False), and potentially reduces `a` by the modulus.
                                    loop {
                                        temp = long_mul(a, z);
                                        if temp.is_null() {
                                            break '__join_4;
                                        } else {
                                            Py_XDECREF__2!(z);
                                            z = temp;
                                            temp = core::ptr::null_mut::<_longobject>();
                                            loop {
                                                if !c.is_null() {
                                                    if l_mod(
                                                        z,
                                                        c,
                                                        &mut (*core::ptr::addr_of_mut!(temp)),
                                                    ) < 0
                                                    {
                                                        break '__join_4;
                                                    } else {
                                                        Py_XDECREF__2!(z);
                                                        z = temp;
                                                        temp = core::ptr::null_mut::<_longobject>();
                                                    }
                                                }
                                                if true {
                                                    break;
                                                }
                                            }
                                            if true {
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                        // else bi is 0, and z==1 is correct
                        } else {
                            if i <= 2i64 {
                                // Left-to-right binary exponentiation (HAC Algorithm 14.79)
                                //
                                // https://cacr.uwaterloo.ca/hac/about/chap14.pdf
                                // Find the first significant exponent bit. Search right to left
                                // because we're primarily trying to cut overhead for small powers.
                                // else there is no significant bit
                                Py_SETREF__4!(z, Py_NewRef__2!(a) as *mut _longobject);
                                bit_1 = 2;
                                loop {
                                    if bit_1 > bi_1 {
                                        break;
                                    } else {
                                        bit_1 <<= 1i32;
                                    }
                                }
                                // found the first bit
                                bit_1 >>= 1i32;
                                i = i.wrapping_sub(1);
                                bit_1 >>= 1i32;
                                loop {
                                    if bit_1 != 0 {
                                        loop {
                                            temp = long_mul(z, z);
                                            if temp.is_null() {
                                                break '__join_4;
                                            } else {
                                                Py_XDECREF__2!(z);
                                                z = temp;
                                                temp = core::ptr::null_mut::<_longobject>();
                                                loop {
                                                    if !c.is_null() {
                                                        if l_mod(
                                                            z,
                                                            c,
                                                            &mut (*core::ptr::addr_of_mut!(temp)),
                                                        ) < 0
                                                        {
                                                            break '__join_4;
                                                        } else {
                                                            Py_XDECREF__2!(z);
                                                            z = temp;
                                                            temp = core::ptr::null_mut::<_longobject>(
                                                            );
                                                        }
                                                    }
                                                    if true {
                                                        break;
                                                    }
                                                }
                                                if true {
                                                    break;
                                                }
                                            }
                                        }
                                        if bi_1 & bit_1 != 0 {
                                            loop {
                                                temp = long_mul(z, a);
                                                if temp.is_null() {
                                                    break '__join_4;
                                                } else {
                                                    Py_XDECREF__2!(z);
                                                    z = temp;
                                                    temp = core::ptr::null_mut::<_longobject>();
                                                    loop {
                                                        if !c.is_null() {
                                                            if l_mod(
                                                                z,
                                                                c,
                                                                &mut (*core::ptr::addr_of_mut!(
                                                                    temp
                                                                )),
                                                            ) < 0
                                                            {
                                                                break '__join_4;
                                                            } else {
                                                                Py_XDECREF__2!(z);
                                                                z = temp;
                                                                temp = core::ptr::null_mut::<
                                                                    _longobject,
                                                                >(
                                                                );
                                                            }
                                                        }
                                                        if true {
                                                            break;
                                                        }
                                                    }
                                                    if true {
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                        bit_1 >>= 1i32;
                                    } else {
                                        let __v3: i64 = i.wrapping_sub(1);
                                        i = __v3;
                                        if __v3 < 0 {
                                            break;
                                        } else {
                                            bi_1 = *((*b).long_value.ob_digit.as_mut_ptr()
                                                as *mut u32)
                                                .wrapping_offset(i as isize);
                                            bit_1 = 536870912u32;
                                        }
                                    }
                                }
                            } else {
                                // Left-to-right k-ary sliding window exponentiation
                                // (Handbook of Applied Cryptography (HAC) Algorithm 14.85)
                                (*__slate_slot_55)[0] = Py_NewRef__2!(a) as *mut _longobject;
                                num_table_entries = 1;
                                loop {
                                    temp = long_mul(a, a);
                                    if temp.is_null() {
                                        break '__join_4;
                                    } else {
                                        Py_XDECREF__2!(a2);
                                        a2 = temp;
                                        temp = core::ptr::null_mut::<_longobject>();
                                        loop {
                                            if !c.is_null() {
                                                if l_mod(
                                                    a2,
                                                    c,
                                                    &mut (*core::ptr::addr_of_mut!(temp)),
                                                ) < 0
                                                {
                                                    break '__join_4;
                                                } else {
                                                    Py_XDECREF__2!(a2);
                                                    a2 = temp;
                                                    temp = core::ptr::null_mut::<_longobject>();
                                                }
                                            }
                                            if true {
                                                break;
                                            }
                                        }
                                        if true {
                                            break;
                                        }
                                    }
                                }
                                // table[i] == a**(2*i + 1) % c
                                i = 1;
                                while i < 16i64 {
                                    (*__slate_slot_55)[i as usize] =
                                        core::ptr::null_mut::<_longobject>(); // must set to known value for MULT
                                    loop {
                                        temp = long_mul(
                                            (*__slate_slot_55)[i.wrapping_sub(1) as usize],
                                            a2,
                                        );
                                        if temp.is_null() {
                                            break '__join_4;
                                        } else {
                                            Py_XDECREF__2!((*__slate_slot_55)[i as usize]);
                                            (*__slate_slot_55)[i as usize] = temp;
                                            temp = core::ptr::null_mut::<_longobject>();
                                            loop {
                                                if !c.is_null() {
                                                    if l_mod(
                                                        (*__slate_slot_55)[i as usize],
                                                        c,
                                                        &mut (*core::ptr::addr_of_mut!(temp)),
                                                    ) < 0
                                                    {
                                                        break '__join_4;
                                                    } else {
                                                        Py_XDECREF__2!(
                                                            (*__slate_slot_55)[i as usize]
                                                        );
                                                        (*__slate_slot_55)[i as usize] = temp;
                                                        temp = core::ptr::null_mut::<_longobject>();
                                                    }
                                                }
                                                if true {
                                                    break;
                                                }
                                            }
                                            if true {
                                                break;
                                            }
                                        }
                                    }
                                    num_table_entries = num_table_entries.wrapping_add(1); // incremented iff MULT succeeded
                                    i = i.wrapping_add(1);
                                }
                                Py_CLEAR__10!(a2);
                                pending = 0;
                                blen = 0;
                                // number of trailing zeroes in `pending`
                                i = _PyLong_SignedDigitCount(b as *const _longobject)
                                    .wrapping_sub(1);
                                while i >= 0 {
                                    bi = *((*b).long_value.ob_digit.as_mut_ptr() as *mut u32)
                                        .wrapping_offset(i as isize);
                                    j = 29i64;
                                    while j >= 0 {
                                        let bit: i32 = (bi >> j & 1) as i32;
                                        pending = pending << 1i32 | bit;
                                        if pending != 0 {
                                            blen = blen.wrapping_add(1);
                                            if blen == 5 {
                                                loop {
                                                    ntz_1 = 0;
                                                    while pending & 1 == 0 {
                                                        ntz_1 = ntz_1.wrapping_add(1);
                                                        pending >>= 1i32;
                                                    }
                                                    blen = blen.wrapping_sub(ntz_1);
                                                    loop {
                                                        temp = long_mul(z, z);
                                                        if temp.is_null() {
                                                            break '__join_4;
                                                        } else {
                                                            Py_XDECREF__2!(z);
                                                            z = temp;
                                                            temp = core::ptr::null_mut::<_longobject>(
                                                            );
                                                            loop {
                                                                if !c.is_null() {
                                                                    if l_mod(
                                                                        z,
                                                                        c,
                                                                        &mut (*core::ptr::addr_of_mut!(
                                                                            temp
                                                                        )),
                                                                    ) < 0
                                                                    {
                                                                        break '__join_4;
                                                                    } else {
                                                                        Py_XDECREF__2!(z);
                                                                        z = temp;
                                                                        temp = core::ptr::null_mut::<
                                                                            _longobject,
                                                                        >(
                                                                        );
                                                                    }
                                                                }
                                                                if true {
                                                                    break;
                                                                }
                                                            }
                                                            if true {
                                                                let __v32: i32 =
                                                                    blen.wrapping_sub(1);
                                                                blen = __v32;
                                                                if __v32 == 0 {
                                                                    break;
                                                                }
                                                            }
                                                        }
                                                    }
                                                    loop {
                                                        temp = long_mul(
                                                            z,
                                                            (*__slate_slot_55)
                                                                [(pending >> 1i32) as usize],
                                                        );
                                                        if temp.is_null() {
                                                            break '__join_4;
                                                        } else {
                                                            Py_XDECREF__2!(z);
                                                            z = temp;
                                                            temp = core::ptr::null_mut::<_longobject>(
                                                            );
                                                            loop {
                                                                if !c.is_null() {
                                                                    if l_mod(
                                                                        z,
                                                                        c,
                                                                        &mut (*core::ptr::addr_of_mut!(
                                                                            temp
                                                                        )),
                                                                    ) < 0
                                                                    {
                                                                        break '__join_4;
                                                                    } else {
                                                                        Py_XDECREF__2!(z);
                                                                        z = temp;
                                                                        temp = core::ptr::null_mut::<
                                                                            _longobject,
                                                                        >(
                                                                        );
                                                                    }
                                                                }
                                                                if true {
                                                                    break;
                                                                }
                                                            }
                                                            if true {
                                                                break;
                                                            }
                                                        }
                                                    }
                                                    '__loop_94: loop {
                                                        let __v31: i32 = ntz_1;
                                                        ntz_1 = __v31.wrapping_sub(1);
                                                        if __v31 > 0 {
                                                            loop {
                                                                temp = long_mul(z, z);
                                                                if temp.is_null() {
                                                                    break '__join_4;
                                                                } else {
                                                                    Py_XDECREF__2!(z);
                                                                    z = temp;
                                                                    temp = core::ptr::null_mut::<
                                                                        _longobject,
                                                                    >(
                                                                    );
                                                                    loop {
                                                                        if !c.is_null() {
                                                                            if l_mod(
                                                                                z,
                                                                                c,
                                                                                &mut (*core::ptr::addr_of_mut!(
                                                                                    temp
                                                                                )),
                                                                            ) < 0
                                                                            {
                                                                                break '__join_4;
                                                                            } else {
                                                                                Py_XDECREF__2!(z);
                                                                                z = temp;
                                                                                temp = core::ptr::null_mut::<_longobject>();
                                                                            }
                                                                        }
                                                                        if true {
                                                                            break;
                                                                        }
                                                                    }
                                                                    if true {
                                                                        continue '__loop_94;
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            break;
                                                        }
                                                    }
                                                    pending = 0;
                                                    if true {
                                                        break;
                                                    }
                                                }
                                            }
                                        } else {
                                            loop {
                                                temp = long_mul(z, z);
                                                if temp.is_null() {
                                                    break '__join_4;
                                                } else {
                                                    Py_XDECREF__2!(z);
                                                    z = temp;
                                                    temp = core::ptr::null_mut::<_longobject>();
                                                    loop {
                                                        if !c.is_null() {
                                                            if l_mod(
                                                                z,
                                                                c,
                                                                &mut (*core::ptr::addr_of_mut!(
                                                                    temp
                                                                )),
                                                            ) < 0
                                                            {
                                                                break '__join_4;
                                                            } else {
                                                                Py_XDECREF__2!(z);
                                                                z = temp;
                                                                temp = core::ptr::null_mut::<
                                                                    _longobject,
                                                                >(
                                                                );
                                                            }
                                                        }
                                                        if true {
                                                            break;
                                                        }
                                                    }
                                                    if true {
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                        // absorb strings of 0 bits
                                        j = j.wrapping_sub(1);
                                    }
                                    i = i.wrapping_sub(1);
                                }
                                if pending != 0 {
                                    loop {
                                        ntz = 0;
                                        while pending & 1 == 0 {
                                            ntz = ntz.wrapping_add(1);
                                            pending >>= 1i32;
                                        }
                                        blen = blen.wrapping_sub(ntz);
                                        loop {
                                            temp = long_mul(z, z);
                                            if temp.is_null() {
                                                break '__join_4;
                                            } else {
                                                Py_XDECREF__2!(z);
                                                z = temp;
                                                temp = core::ptr::null_mut::<_longobject>();
                                                loop {
                                                    if !c.is_null() {
                                                        if l_mod(
                                                            z,
                                                            c,
                                                            &mut (*core::ptr::addr_of_mut!(temp)),
                                                        ) < 0
                                                        {
                                                            break '__join_4;
                                                        } else {
                                                            Py_XDECREF__2!(z);
                                                            z = temp;
                                                            temp = core::ptr::null_mut::<_longobject>(
                                                            );
                                                        }
                                                    }
                                                    if true {
                                                        break;
                                                    }
                                                }
                                                if true {
                                                    let __v17: i32 = blen.wrapping_sub(1);
                                                    blen = __v17;
                                                    if __v17 == 0 {
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                        loop {
                                            temp = long_mul(
                                                z,
                                                (*__slate_slot_55)[(pending >> 1i32) as usize],
                                            );
                                            if temp.is_null() {
                                                break '__join_4;
                                            } else {
                                                Py_XDECREF__2!(z);
                                                z = temp;
                                                temp = core::ptr::null_mut::<_longobject>();
                                                loop {
                                                    if !c.is_null() {
                                                        if l_mod(
                                                            z,
                                                            c,
                                                            &mut (*core::ptr::addr_of_mut!(temp)),
                                                        ) < 0
                                                        {
                                                            break '__join_4;
                                                        } else {
                                                            Py_XDECREF__2!(z);
                                                            z = temp;
                                                            temp = core::ptr::null_mut::<_longobject>(
                                                            );
                                                        }
                                                    }
                                                    if true {
                                                        break;
                                                    }
                                                }
                                                if true {
                                                    break;
                                                }
                                            }
                                        }
                                        '__loop_61: loop {
                                            let __v16: i32 = ntz;
                                            ntz = __v16.wrapping_sub(1);
                                            if __v16 > 0 {
                                                loop {
                                                    temp = long_mul(z, z);
                                                    if temp.is_null() {
                                                        break '__join_4;
                                                    } else {
                                                        Py_XDECREF__2!(z);
                                                        z = temp;
                                                        temp = core::ptr::null_mut::<_longobject>();
                                                        loop {
                                                            if !c.is_null() {
                                                                if l_mod(
                                                                    z,
                                                                    c,
                                                                    &mut (*core::ptr::addr_of_mut!(
                                                                        temp
                                                                    )),
                                                                ) < 0
                                                                {
                                                                    break '__join_4;
                                                                } else {
                                                                    Py_XDECREF__2!(z);
                                                                    z = temp;
                                                                    temp = core::ptr::null_mut::<
                                                                        _longobject,
                                                                    >(
                                                                    );
                                                                }
                                                            }
                                                            if true {
                                                                break;
                                                            }
                                                        }
                                                        if true {
                                                            continue '__loop_61;
                                                        }
                                                    }
                                                }
                                            } else {
                                                break;
                                            }
                                        }
                                        pending = 0;
                                        if true {
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                        if negativeOutput != 0 {
                            __v2 = !_PyLong_IsZero(z as *const _longobject);
                        } else {
                            __v2 = false;
                        }
                        if __v2 {
                            temp = long_sub(z, c);
                            if !temp.is_null() {
                                Py_SETREF__4!(z, temp);
                                temp = core::ptr::null_mut::<_longobject>();
                                break '__join_3;
                            }
                        } else {
                            break '__join_3;
                        }
                    }
                }
                Py_CLEAR__10!(z);
                // fall through
            }
            i = 0;
            while i < num_table_entries {
                Py_DECREF__2!((*__slate_slot_55)[i as usize]);
                i = i.wrapping_add(1);
            }
            Py_DECREF__2!(a);
            Py_DECREF__2!(b);
            Py_XDECREF__2!(c);
            Py_XDECREF__2!(a2);
            Py_XDECREF__2!(temp);
            return z as *mut _object;
        }
    }
}

#[unsafe(link_section = ".text.slate_distinct.longobject.long_invert")]
extern "C-unwind" fn long_invert(self_: *mut _object) -> *mut _object {
    let v: *mut _longobject = _PyLong_CAST!(self_);
    // Implement ~x as -(x+1)
    if _PyLong_IsCompact(v as *const _longobject) != 0 {
        return _PyLong_FromSTwoDigits(!medium_value!(v)) as *mut _object;
    }
    let mut x: *mut _longobject = long_add(v, _PyLong_GetOne() as *mut _longobject);
    if x.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    _PyLong_Negate(core::ptr::addr_of_mut!(x));
    // No need for maybe_small_long here, since any small longs
    // will have been caught in the _PyLong_IsCompact() fast path.
    x as *mut _object
}

/// Forward declarations
fn long_neg(v: *mut _longobject) -> *mut _longobject {
    if _PyLong_IsCompact(v as *const _longobject) != 0 {
        return _PyLong_FromSTwoDigits(medium_value!(v).wrapping_neg());
    }
    let z: *mut _longobject = _PyLong_Copy(v) as *mut _longobject;
    if !z.is_null() {
        _PyLong_FlipSign(z);
    }
    z
}

#[unsafe(link_section = ".text.slate_distinct.longobject.long_neg_method")]
extern "C-unwind" fn long_neg_method(v: *mut _object) -> *mut _object {
    long_neg(_PyLong_CAST!(v)) as *mut _object
}

fn long_abs(v: *mut _longobject) -> *mut _longobject {
    if _PyLong_IsNegative(v as *const _longobject) {
        return long_neg(v);
    } else {
        return long_long(v as *mut _object) as *mut _longobject;
    }
}

#[unsafe(link_section = ".text.slate_distinct.longobject.long_abs_method")]
extern "C-unwind" fn long_abs_method(v: *mut _object) -> *mut _object {
    long_abs(_PyLong_CAST!(v)) as *mut _object
}

#[unsafe(link_section = ".text.slate_distinct.longobject.long_bool")]
extern "C-unwind" fn long_bool(v: *mut _object) -> i32 {
    !_PyLong_IsZero(_PyLong_CAST!(v) as *const _longobject) as i32
}

// Inner function for both long_rshift and _PyLong_Rshift, shifting an
// integer right by PyLong_SHIFT*wordshift + remshift bits.
// wordshift should be nonnegative.
fn long_rshift1(a: *mut _longobject, mut wordshift: i64, mut remshift: u32) -> *mut _object {
    let mut z: *mut _longobject = core::ptr::null_mut::<_longobject>();
    let mut newsize: i64 = 0;
    let mut hishift: i64 = 0;
    let mut size_a: i64 = 0;
    let mut accum: u64 = 0;
    let mut a_negative: i32 = 0;
    // Total number of bits shifted must be nonnegative.
    // Fast path for small a.
    if _PyLong_IsCompact(a as *const _longobject) != 0 {
        let m: i64 = medium_value!(a);
        let mut x: i64 = 0;
        let shift: u32 = if wordshift == 0 { remshift } else { 30 };
        x = if m < 0 { !(!m >> shift) } else { m >> shift };
        return _PyLong_FromSTwoDigits(x) as *mut _object;
    }
    a_negative = _PyLong_IsNegative(a as *const _longobject) as i32;
    size_a = _PyLong_DigitCount(a as *const _longobject);
    if a_negative != 0 {
        // For negative 'a', adjust so that 0 < remshift <= PyLong_SHIFT,
        // while keeping PyLong_SHIFT*wordshift + remshift the same. This
        // ensures that 'newsize' is computed correctly below.
        if remshift == 0 {
            if wordshift == 0 {
                // Can only happen if the original shift was 0.
                return long_long(a as *mut _object);
            }
            remshift = 30;
            wordshift = wordshift.wrapping_sub(1);
        }
    }
    newsize = size_a.wrapping_sub(wordshift);
    if newsize <= 0 {
        // Shifting all the bits of 'a' out gives either -1 or 0.
        return PyLong_FromLong(a_negative.wrapping_neg() as i64);
    }
    z = long_alloc(newsize);
    if z.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    hishift = 30u32.wrapping_sub(remshift) as i64;
    accum = (unsafe {
        *((*a).long_value.ob_digit.as_mut_ptr() as *mut u32).wrapping_offset(wordshift as isize)
    }) as u64;
    if a_negative != 0 {
        // For a positive integer a and nonnegative shift, we have:
        //
        //     (-a) >> shift == -((a + 2**shift - 1) >> shift).
        //
        // In the addition `a + (2**shift - 1)`, the low `wordshift` digits of
        // `2**shift - 1` all have value `PyLong_MASK`, so we get a carry out
        // from the bottom `wordshift` digits when at least one of the least
        // significant `wordshift` digits of `a` is nonzero. Digit `wordshift`
        // of `2**shift - 1` has value `PyLong_MASK >> hishift`.
        _PyLong_SetSignAndDigitCount(z, 1i32.wrapping_neg(), newsize);
        let mut sticky: u32 = 0;
        for j in 0..wordshift {
            let __v2: u32 = sticky;
            sticky = __v2
                | unsafe {
                    *((*a).long_value.ob_digit.as_mut_ptr() as *mut u32).wrapping_offset(j as isize)
                };
        }
        accum = accum
            .wrapping_add((1073741823u32 >> hishift).wrapping_add((sticky != 0) as u32) as u64);
    }
    accum >>= remshift;
    let mut i: i64 = 0;
    let mut j: i64 = wordshift.wrapping_add(1);
    while j < size_a {
        let __v11: u64 = accum;
        accum = __v11.wrapping_add(
            ((unsafe {
                *((*a).long_value.ob_digit.as_mut_ptr() as *mut u32).wrapping_offset(j as isize)
            }) as u64)
                << hishift,
        );
        unsafe {
            *((*z).long_value.ob_digit.as_mut_ptr() as *mut u32).wrapping_offset(i as isize) =
                (accum & 1073741823u64) as u32;
        }
        accum >>= 30i32;
        i = i.wrapping_add(1);
        j = j.wrapping_add(1);
    }
    unsafe {
        *((*z).long_value.ob_digit.as_mut_ptr() as *mut u32)
            .wrapping_offset(newsize.wrapping_sub(1) as isize) = accum as u32;
    }
    z = maybe_small_long(long_normalize(z));
    z as *mut _object
}

#[unsafe(link_section = ".text.slate_distinct.longobject.long_rshift")]
extern "C-unwind" fn long_rshift(a: *mut _object, b: *mut _object) -> *mut _object {
    let mut shiftby: i64 = 0;
    CHECK_BINOP!(a, b);
    if _PyLong_IsNegative(b as *const _longobject) {
        unsafe { PyErr_SetString(PyExc_ValueError, c"negative shift count".as_ptr()) };
        return core::ptr::null_mut::<_object>();
    }
    if _PyLong_IsZero(a as *const _longobject) {
        return PyLong_FromLong(0i64);
    }
    if PyLong_AsInt64(b, core::ptr::addr_of_mut!(shiftby)) < 0 {
        if (unsafe { PyErr_ExceptionMatches(PyExc_OverflowError) }) == 0 {
            return core::ptr::null_mut::<_object>();
        }
        unsafe { PyErr_Clear() };
        if _PyLong_IsNegative(a as *const _longobject) {
            return PyLong_FromLong(1i32.wrapping_neg() as i64);
        } else {
            return PyLong_FromLong(0i64);
        }
    }
    _PyLong_Rshift(a, shiftby)
}

/// Return a >> shiftby.
#[unsafe(no_mangle)]
extern "C-unwind" fn _PyLong_Rshift(a: *mut _object, shiftby: i64) -> *mut _object {
    let mut wordshift: i64 = 0;
    let mut remshift: u32 = 0;
    if _PyLong_IsZero(a as *const _longobject) {
        return PyLong_FromLong(0i64);
    }
    wordshift = shiftby / 30;
    remshift = (shiftby % 30) as u32;
    long_rshift1(a as *mut _longobject, wordshift, remshift)
}

fn long_lshift1(a: *mut _longobject, wordshift: i64, remshift: u32) -> *mut _object {
    let mut z: *mut _longobject = core::ptr::null_mut::<_longobject>();
    let mut oldsize: i64 = 0;
    let mut newsize: i64 = 0;
    let mut i: i64 = 0;
    let mut accum: u64 = 0;
    let __v0: bool = wordshift == 0 && _PyLong_IsCompact(a as *const _longobject) != 0;
    if __v0 {
        let m: i64 = medium_value!(a);
        // bypass undefined shift operator behavior
        let x: i64 = if m < 0 {
            (m.wrapping_neg() << remshift).wrapping_neg()
        } else {
            m << remshift
        };
        return _PyLong_FromSTwoDigits(x) as *mut _object;
    }
    oldsize = _PyLong_DigitCount(a as *const _longobject);
    newsize = oldsize.wrapping_add(wordshift);
    if remshift != 0 {
        newsize = newsize.wrapping_add(1);
    }
    z = long_alloc(newsize);
    if z.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    if _PyLong_IsNegative(a as *const _longobject) {
        _PyLong_FlipSign(z);
    }
    while i < wordshift {
        unsafe {
            *((*z).long_value.ob_digit.as_mut_ptr() as *mut u32).wrapping_offset(i as isize) = 0;
        }
        i = i.wrapping_add(1);
    }
    for j in 0..oldsize {
        let __v6: u64 = accum;
        accum = __v6
            | ((unsafe {
                *((*a).long_value.ob_digit.as_mut_ptr() as *mut u32).wrapping_offset(j as isize)
            }) as u64)
                << remshift;
        unsafe {
            *((*z).long_value.ob_digit.as_mut_ptr() as *mut u32).wrapping_offset(i as isize) =
                (accum & 1073741823u64) as u32;
        }
        accum >>= 30i32;
        i = i.wrapping_add(1);
    }
    if remshift != 0 {
        unsafe {
            *((*z).long_value.ob_digit.as_mut_ptr() as *mut u32)
                .wrapping_offset(newsize.wrapping_sub(1) as isize) = accum as u32;
        }
    }
    z = long_normalize(z);
    maybe_small_long(z) as *mut _object
}

#[unsafe(link_section = ".text.slate_distinct.longobject.long_lshift_method")]
extern "C-unwind" fn long_lshift_method(aa: *mut _object, bb: *mut _object) -> *mut _object {
    CHECK_BINOP!(aa, bb);
    let a: *mut _longobject = aa as *mut _longobject;
    let b: *mut _longobject = bb as *mut _longobject;
    if _PyLong_IsNegative(b as *const _longobject) {
        unsafe { PyErr_SetString(PyExc_ValueError, c"negative shift count".as_ptr()) };
        return core::ptr::null_mut::<_object>();
    }
    if _PyLong_IsZero(a as *const _longobject) {
        return PyLong_FromLong(0i64);
    }
    let mut shiftby: i64 = 0;
    if PyLong_AsInt64(bb, core::ptr::addr_of_mut!(shiftby)) < 0 {
        if (unsafe { PyErr_ExceptionMatches(PyExc_OverflowError) }) != 0 {
            unsafe { PyErr_SetString(PyExc_OverflowError, c"too many digits in integer".as_ptr()) };
        }
        return core::ptr::null_mut::<_object>();
    }
    long_lshift_int64(a, shiftby)
}

/// Return a << shiftby.
fn long_lshift_int64(a: *mut _longobject, shiftby: i64) -> *mut _object {
    if _PyLong_IsZero(a as *const _longobject) {
        return PyLong_FromLong(0i64);
    }
    let wordshift: i64 = shiftby / 30;
    let remshift: u32 = (shiftby % 30) as u32;
    long_lshift1(a, wordshift, remshift)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyLong_Lshift(a: *mut _object, shiftby: i64) -> *mut _object {
    long_lshift_int64(_PyLong_CAST!(a), shiftby)
}

// Compute two's complement of digit vector a[0:m], writing result to
// z[0:m].  The digit vector a need not be normalized, but should not
// be entirely zero.  a and z may point to the same digit vector.
fn v_complement(z: *mut u32, a: *mut u32, m: i64) {
    let mut carry: u32 = 1;
    for i in 0..m {
        carry = carry.wrapping_add((unsafe { *a.wrapping_offset(i as isize) }) ^ 1073741823u32);
        unsafe {
            *z.wrapping_offset(i as isize) = carry & 1073741823u32;
        }
        carry >>= 30i32;
    }
}

// Bitwise and/xor/or operations
/// # Arguments
///
/// * `op` - '&', '|', '^'
fn long_bitwise(mut a: *mut _longobject, op: i8, mut b: *mut _longobject) -> *mut _object {
    let mut nega: i32 = 0;
    let mut negb: i32 = 0;
    let mut negz: i32 = 0;
    let mut size_a: i64 = 0;
    let mut size_b: i64 = 0;
    let mut size_z: i64 = 0;
    let mut i: i64 = 0;
    let mut z: *mut _longobject = unsafe { core::mem::zeroed() };
    // Bitwise operations for negative numbers operate as though
    // on a two's complement representation.  So convert arguments
    // from sign-magnitude to two's complement, and convert the
    // result back to sign-magnitude at the end.
    // If a is negative, replace it by its two's complement.
    size_a = _PyLong_DigitCount(a as *const _longobject);
    nega = _PyLong_IsNegative(a as *const _longobject) as i32;
    if nega != 0 {
        z = long_alloc(size_a);
        if z.is_null() {
            return core::ptr::null_mut::<_object>();
        }
        v_complement(
            unsafe { (*z).long_value.ob_digit.as_mut_ptr() as *mut u32 },
            unsafe { (*a).long_value.ob_digit.as_mut_ptr() as *mut u32 },
            size_a,
        );
        a = z;
    } else {
        Py_INCREF__2!(a);
    }
    // Keep reference count consistent.
    // Same for b.
    size_b = _PyLong_DigitCount(b as *const _longobject);
    negb = _PyLong_IsNegative(b as *const _longobject) as i32;
    if negb != 0 {
        z = long_alloc(size_b);
        if z.is_null() {
            Py_DECREF__2!(a);
            return core::ptr::null_mut::<_object>();
        }
        v_complement(
            unsafe { (*z).long_value.ob_digit.as_mut_ptr() as *mut u32 },
            unsafe { (*b).long_value.ob_digit.as_mut_ptr() as *mut u32 },
            size_b,
        );
        b = z;
    } else {
        Py_INCREF__2!(b);
    }
    // Swap a and b if necessary to ensure size_a >= size_b.
    if size_a < size_b {
        z = a;
        a = b;
        b = z;
        size_z = size_a;
        size_a = size_b;
        size_b = size_z;
        negz = nega;
        nega = negb;
        negb = negz;
    }
    // JRH: The original logic here was to allocate the result value (z)
    // as the longer of the two operands.  However, there are some cases
    // where the result is guaranteed to be shorter than that: AND of two
    // positives, OR of two negatives: use the shorter number.  AND with
    // mixed signs: use the positive number.  OR with mixed signs: use the
    // negative number.
    match op as i32 {
        94 => {
            negz = nega ^ negb;
            size_z = size_a;
        }
        38 => {
            negz = nega & negb;
            size_z = if negb != 0 { size_a } else { size_b };
        }
        124 => {
            negz = nega | negb;
            size_z = if negb != 0 { size_b } else { size_a };
        }
        _ => {
            Py_UNREACHABLE!();
        }
    }
    // We allow an extra digit if z is negative, to make sure that
    // the final two's complement of z doesn't overflow.
    z = long_alloc(size_z.wrapping_add(negz as i64));
    if z.is_null() {
        Py_DECREF__2!(a);
        Py_DECREF__2!(b);
        return core::ptr::null_mut::<_object>();
    }
    // Compute digits for overlap of a and b.
    match op as i32 {
        38 => {
            i = 0;
            while i < size_b {
                unsafe {
                    *((*z).long_value.ob_digit.as_mut_ptr() as *mut u32)
                        .wrapping_offset(i as isize) = *((*a).long_value.ob_digit.as_mut_ptr()
                        as *mut u32)
                        .wrapping_offset(i as isize)
                        & *((*b).long_value.ob_digit.as_mut_ptr() as *mut u32)
                            .wrapping_offset(i as isize);
                }
                i = i.wrapping_add(1);
            }
        }
        124 => {
            i = 0;
            while i < size_b {
                unsafe {
                    *((*z).long_value.ob_digit.as_mut_ptr() as *mut u32)
                        .wrapping_offset(i as isize) = *((*a).long_value.ob_digit.as_mut_ptr()
                        as *mut u32)
                        .wrapping_offset(i as isize)
                        | *((*b).long_value.ob_digit.as_mut_ptr() as *mut u32)
                            .wrapping_offset(i as isize);
                }
                i = i.wrapping_add(1);
            }
        }
        94 => {
            i = 0;
            while i < size_b {
                unsafe {
                    *((*z).long_value.ob_digit.as_mut_ptr() as *mut u32)
                        .wrapping_offset(i as isize) = *((*a).long_value.ob_digit.as_mut_ptr()
                        as *mut u32)
                        .wrapping_offset(i as isize)
                        ^ *((*b).long_value.ob_digit.as_mut_ptr() as *mut u32)
                            .wrapping_offset(i as isize);
                }
                i = i.wrapping_add(1);
            }
        }
        _ => {
            Py_UNREACHABLE!();
        }
    }
    // Copy any remaining digits of a, inverting if necessary.
    if (op as i32) == 94 && negb != 0 {
        while i < size_z {
            unsafe {
                *((*z).long_value.ob_digit.as_mut_ptr() as *mut u32).wrapping_offset(i as isize) =
                    *((*a).long_value.ob_digit.as_mut_ptr() as *mut u32)
                        .wrapping_offset(i as isize)
                        ^ 1073741823u32;
            }
            i = i.wrapping_add(1);
        }
    } else {
        if i < size_z {
            unsafe {
                core::ptr::copy_nonoverlapping(
                    ((((*a).long_value.ob_digit.as_mut_ptr() as *mut u32)
                        .wrapping_offset(i as isize) as *const ())
                        as *const core::ffi::c_void) as *const u8,
                    ((((*z).long_value.ob_digit.as_mut_ptr() as *mut u32)
                        .wrapping_offset(i as isize) as *mut ())
                        as *mut core::ffi::c_void) as *mut u8,
                    ((size_z.wrapping_sub(i) as u64)
                        .wrapping_mul(core::mem::size_of::<u32>() as u64)
                        as usize) as usize,
                )
            };
        }
    }
    // Complement result if negative.
    if negz != 0 {
        _PyLong_FlipSign(z);
        unsafe {
            *((*z).long_value.ob_digit.as_mut_ptr() as *mut u32).wrapping_offset(size_z as isize) =
                1073741823u32;
        }
        v_complement(
            unsafe { (*z).long_value.ob_digit.as_mut_ptr() as *mut u32 },
            unsafe { (*z).long_value.ob_digit.as_mut_ptr() as *mut u32 },
            size_z.wrapping_add(1),
        );
    }
    Py_DECREF__2!(a);
    Py_DECREF__2!(b);
    maybe_small_long(long_normalize(z)) as *mut _object
}

#[unsafe(link_section = ".text.slate_distinct.longobject.long_and")]
extern "C-unwind" fn long_and(a: *mut _object, b: *mut _object) -> *mut _object {
    CHECK_BINOP!(a, b);
    let x: *mut _longobject = a as *mut _longobject;
    let y: *mut _longobject = b as *mut _longobject;
    let __v2: bool = _PyLong_IsCompact(x as *const _longobject) != 0
        && _PyLong_IsCompact(y as *const _longobject) != 0;
    if __v2 {
        return _PyLong_FromSTwoDigits(medium_value!(x) & medium_value!(y)) as *mut _object;
    }
    long_bitwise(x, 38i8, y)
}

#[unsafe(link_section = ".text.slate_distinct.longobject.long_xor")]
extern "C-unwind" fn long_xor(a: *mut _object, b: *mut _object) -> *mut _object {
    CHECK_BINOP!(a, b);
    let x: *mut _longobject = a as *mut _longobject;
    let y: *mut _longobject = b as *mut _longobject;
    let __v2: bool = _PyLong_IsCompact(x as *const _longobject) != 0
        && _PyLong_IsCompact(y as *const _longobject) != 0;
    if __v2 {
        return _PyLong_FromSTwoDigits(medium_value!(x) ^ medium_value!(y)) as *mut _object;
    }
    long_bitwise(x, 94i8, y)
}

#[unsafe(link_section = ".text.slate_distinct.longobject.long_or")]
extern "C-unwind" fn long_or(a: *mut _object, b: *mut _object) -> *mut _object {
    CHECK_BINOP!(a, b);
    let x: *mut _longobject = a as *mut _longobject;
    let y: *mut _longobject = b as *mut _longobject;
    let __v2: bool = _PyLong_IsCompact(x as *const _longobject) != 0
        && _PyLong_IsCompact(y as *const _longobject) != 0;
    if __v2 {
        return _PyLong_FromSTwoDigits(medium_value!(x) | medium_value!(y)) as *mut _object;
    }
    long_bitwise(x, 124i8, y)
}

#[unsafe(link_section = ".text.slate_distinct.longobject.long_long")]
extern "C-unwind" fn long_long(v: *mut _object) -> *mut _object {
    if PyLong_CheckExact!(v) != 0 {
        return Py_NewRef!(v);
    } else {
        return _PyLong_Copy(v as *mut _longobject);
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyLong_GCD(aarg: *mut _object, barg: *mut _object) -> *mut _object {
    let mut alloc_b: i64 = unsafe { core::mem::zeroed() };
    let mut alloc_a: i64 = unsafe { core::mem::zeroed() };
    let mut size_b: i64 = unsafe { core::mem::zeroed() };
    let mut size_a: i64 = unsafe { core::mem::zeroed() };
    let mut __v39: bool = unsafe { core::mem::zeroed() };
    let mut b_end: *mut u32 = unsafe { core::mem::zeroed() };
    let mut a_end: *mut u32 = unsafe { core::mem::zeroed() };
    let mut d_digit: *mut u32 = unsafe { core::mem::zeroed() };
    let mut c_digit: *mut u32 = unsafe { core::mem::zeroed() };
    let mut b_digit: *mut u32 = unsafe { core::mem::zeroed() };
    let mut a_digit: *mut u32 = unsafe { core::mem::zeroed() };
    let mut k: i32 = unsafe { core::mem::zeroed() };
    let mut nbits: i32 = unsafe { core::mem::zeroed() };
    let mut T: i64 = unsafe { core::mem::zeroed() };
    let mut D: i64 = unsafe { core::mem::zeroed() };
    let mut C: i64 = unsafe { core::mem::zeroed() };
    let mut B: i64 = unsafe { core::mem::zeroed() };
    let mut A: i64 = unsafe { core::mem::zeroed() };
    let mut d_carry: i64 = unsafe { core::mem::zeroed() };
    let mut c_carry: i64 = unsafe { core::mem::zeroed() };
    let mut t: i64 = unsafe { core::mem::zeroed() };
    let mut s: i64 = unsafe { core::mem::zeroed() };
    let mut q: i64 = unsafe { core::mem::zeroed() };
    let mut y: i64 = unsafe { core::mem::zeroed() };
    let mut x: i64 = unsafe { core::mem::zeroed() };
    let mut r: *mut _longobject = unsafe { core::mem::zeroed() };
    let mut d: *mut _longobject = unsafe { core::mem::zeroed() };
    let mut c: *mut _longobject = unsafe { core::mem::zeroed() };
    let mut b: *mut _longobject = unsafe { core::mem::zeroed() };
    let mut a: *mut _longobject = unsafe { core::mem::zeroed() };
    unsafe {
        c = core::ptr::null_mut::<_longobject>();
        d = core::ptr::null_mut::<_longobject>();
        a = aarg as *mut _longobject;
        b = barg as *mut _longobject;
        if _PyLong_DigitCount(a as *const _longobject) <= 2 {
            __v39 = _PyLong_DigitCount(b as *const _longobject) <= 2;
        } else {
            __v39 = false;
        }
        '__join_4: {
            if __v39 {
                Py_INCREF__2!(a);
                Py_INCREF__2!(b);
            } else {
                // Initial reduction: make sure that 0 <= b <= a.
                a = long_abs(a);
                if a.is_null() {
                    return core::ptr::null_mut::<_object>();
                } else {
                    b = long_abs(b);
                    if b.is_null() {
                        Py_DECREF__2!(a);
                        return core::ptr::null_mut::<_object>();
                    } else {
                        if long_compare(a, b) < 0 {
                            r = a;
                            a = b;
                            b = r;
                        }
                        // We now own references to a and b
                        alloc_a = _PyLong_DigitCount(a as *const _longobject);
                        alloc_b = _PyLong_DigitCount(b as *const _longobject);
                        // reduce until a fits into 2 digits
                        '__join_35: {
                            '__join_0: {
                                loop {
                                    let __v0: i64 = _PyLong_DigitCount(a as *const _longobject);
                                    size_a = __v0;
                                    if __v0 > 2 {
                                        nbits = bit_length_digit(
                                            *((*a).long_value.ob_digit.as_mut_ptr() as *mut u32)
                                                .wrapping_offset(size_a.wrapping_sub(1) as isize),
                                        );
                                        // extract top 2*PyLong_SHIFT bits of a into x, along with
                                        // corresponding bits of b into y
                                        size_b = _PyLong_DigitCount(b as *const _longobject);
                                        if size_b == 0 {
                                            break '__join_35;
                                        } else {
                                            x = ((*((*a).long_value.ob_digit.as_mut_ptr() as *mut u32).wrapping_offset(size_a.wrapping_sub(1) as isize) as u64) << 60i32.wrapping_sub(nbits) | (*((*a).long_value.ob_digit.as_mut_ptr() as *mut u32).wrapping_offset(size_a.wrapping_sub(2) as isize) as u64) << 30i32.wrapping_sub(nbits) | ((*((*a).long_value.ob_digit.as_mut_ptr() as *mut u32).wrapping_offset(size_a.wrapping_sub(3) as isize) >> nbits) as u64)) as i64;
                                            y = (((if size_b >= size_a.wrapping_sub(2) {
                                                *((*b).long_value.ob_digit.as_mut_ptr() as *mut u32).wrapping_offset(size_a.wrapping_sub(3) as isize) >> nbits
                                            } else {
                                                0u32
                                            })
                                                as u64)
                                                | if size_b >= size_a.wrapping_sub(1) {
                                                    (*((*b).long_value.ob_digit.as_mut_ptr()
                                                        as *mut u32)
                                                        .wrapping_offset(
                                                            size_a.wrapping_sub(2) as isize
                                                        )
                                                        as u64)
                                                        << 30i32.wrapping_sub(nbits)
                                                } else {
                                                    0
                                                }
                                                | if size_b >= size_a {
                                                    (*((*b).long_value.ob_digit.as_mut_ptr()
                                                        as *mut u32)
                                                        .wrapping_offset(
                                                            size_a.wrapping_sub(1) as isize
                                                        )
                                                        as u64)
                                                        << 60i32.wrapping_sub(nbits)
                                                } else {
                                                    0
                                                })
                                                as i64;
                                            // inner loop of Lehmer's algorithm; A, B, C, D never grow
                                            // larger than PyLong_MASK during the algorithm.
                                            A = 1;
                                            B = 0;
                                            C = 0;
                                            D = 1;
                                            k = 0;
                                            loop {
                                                if y.wrapping_sub(C) == 0 {
                                                    break;
                                                } else {
                                                    q = x.wrapping_add(A.wrapping_sub(1))
                                                        / y.wrapping_sub(C);
                                                    s = B.wrapping_add(q.wrapping_mul(D));
                                                    t = x.wrapping_sub(q.wrapping_mul(y));
                                                    if s > t {
                                                        break;
                                                    } else {
                                                        x = y;
                                                        y = t;
                                                        t = A.wrapping_add(q.wrapping_mul(C));
                                                        A = D;
                                                        B = C;
                                                        C = s;
                                                        D = t;
                                                        k = k.wrapping_add(1);
                                                    }
                                                }
                                            }
                                            if k == 0 {
                                                // no progress; do a Euclidean step
                                                if l_mod(a, b, &mut (*core::ptr::addr_of_mut!(r)))
                                                    < 0
                                                {
                                                    break '__join_0;
                                                } else {
                                                    Py_SETREF__4!(a, b);
                                                    b = r;
                                                    alloc_a = alloc_b;
                                                    alloc_b =
                                                        _PyLong_DigitCount(b as *const _longobject);
                                                }
                                            } else {
                                                // a, b = A*b-B*a, D*a-C*b if k is odd
                                                // a, b = A*a-B*b, D*b-C*a if k is even
                                                if k & 1 != 0 {
                                                    T = A.wrapping_neg();
                                                    A = B.wrapping_neg();
                                                    B = T;
                                                    T = C.wrapping_neg();
                                                    C = D.wrapping_neg();
                                                    D = T;
                                                }
                                                if !c.is_null() {
                                                    _PyLong_SetSignAndDigitCount(c, 1i32, size_a);
                                                } else {
                                                    if _PyObject_IsUniquelyReferenced(
                                                        a as *mut _object,
                                                    ) != 0
                                                    {
                                                        c = Py_NewRef__2!(a) as *mut _longobject;
                                                    } else {
                                                        alloc_a = size_a;
                                                        c = long_alloc(size_a);
                                                        if c.is_null() {
                                                            break '__join_0;
                                                        }
                                                    }
                                                }
                                                if !d.is_null() {
                                                    _PyLong_SetSignAndDigitCount(d, 1i32, size_a);
                                                } else {
                                                    if _PyObject_IsUniquelyReferenced(
                                                        b as *mut _object,
                                                    ) != 0
                                                        && size_a <= alloc_b
                                                    {
                                                        d = Py_NewRef__2!(b) as *mut _longobject;
                                                        _PyLong_SetSignAndDigitCount(
                                                            d, 1i32, size_a,
                                                        );
                                                    } else {
                                                        alloc_b = size_a;
                                                        d = long_alloc(size_a);
                                                        if d.is_null() {
                                                            break '__join_0;
                                                        }
                                                    }
                                                }
                                                a_end = ((*a).long_value.ob_digit.as_mut_ptr()
                                                    as *mut u32)
                                                    .wrapping_offset(size_a as isize);
                                                b_end = ((*b).long_value.ob_digit.as_mut_ptr()
                                                    as *mut u32)
                                                    .wrapping_offset(size_b as isize);
                                                // compute new a and new b in parallel
                                                a_digit = (*a).long_value.ob_digit.as_mut_ptr()
                                                    as *mut u32;
                                                b_digit = (*b).long_value.ob_digit.as_mut_ptr()
                                                    as *mut u32;
                                                c_digit = (*c).long_value.ob_digit.as_mut_ptr()
                                                    as *mut u32;
                                                d_digit = (*d).long_value.ob_digit.as_mut_ptr()
                                                    as *mut u32;
                                                c_carry = 0;
                                                d_carry = 0;
                                                while b_digit < b_end {
                                                    c_carry = c_carry.wrapping_add(
                                                        A.wrapping_mul(*a_digit as i64)
                                                            .wrapping_sub(
                                                                B.wrapping_mul(*b_digit as i64),
                                                            ),
                                                    );
                                                    let __v30: *mut u32 = b_digit;
                                                    b_digit = __v30.wrapping_offset(1 as isize);
                                                    let __v28: *mut u32 = a_digit;
                                                    a_digit = __v28.wrapping_offset(1 as isize);
                                                    d_carry = d_carry.wrapping_add(
                                                        D.wrapping_mul(*__v30 as i64).wrapping_sub(
                                                            C.wrapping_mul(*__v28 as i64),
                                                        ),
                                                    );
                                                    let __v23: *mut u32 = c_digit;
                                                    c_digit = __v23.wrapping_offset(1 as isize);
                                                    *__v23 = (c_carry & 1073741823i64) as u32;
                                                    let __v21: *mut u32 = d_digit;
                                                    d_digit = __v21.wrapping_offset(1 as isize);
                                                    *__v21 = (d_carry & 1073741823i64) as u32;
                                                    c_carry >>= 30i32;
                                                    d_carry >>= 30i32;
                                                }
                                                while a_digit < a_end {
                                                    c_carry = c_carry.wrapping_add(
                                                        A.wrapping_mul(*a_digit as i64),
                                                    );
                                                    let __v13: *mut u32 = a_digit;
                                                    a_digit = __v13.wrapping_offset(1 as isize);
                                                    d_carry = d_carry.wrapping_sub(
                                                        C.wrapping_mul(*__v13 as i64),
                                                    );
                                                    let __v8: *mut u32 = c_digit;
                                                    c_digit = __v8.wrapping_offset(1 as isize);
                                                    *__v8 = (c_carry & 1073741823i64) as u32;
                                                    let __v6: *mut u32 = d_digit;
                                                    d_digit = __v6.wrapping_offset(1 as isize);
                                                    *__v6 = (d_carry & 1073741823i64) as u32;
                                                    c_carry >>= 30i32;
                                                    d_carry >>= 30i32;
                                                }
                                                Py_INCREF__2!(c);
                                                Py_INCREF__2!(d);
                                                Py_DECREF__2!(a);
                                                Py_DECREF__2!(b);
                                                a = long_normalize(c);
                                                b = long_normalize(d);
                                            }
                                        }
                                    } else {
                                        break;
                                    }
                                }
                                Py_XDECREF__2!(c);
                                Py_XDECREF__2!(d);
                                break '__join_4;
                            }
                            Py_DECREF__2!(a);
                            Py_DECREF__2!(b);
                            Py_XDECREF__2!(c);
                            Py_XDECREF__2!(d);
                            return core::ptr::null_mut::<_object>();
                        }
                        if size_a < alloc_a {
                            r = _PyLong_Copy(a) as *mut _longobject;
                            Py_DECREF__2!(a);
                        } else {
                            r = a;
                        }
                        Py_DECREF__2!(b);
                        Py_XDECREF__2!(c);
                        Py_XDECREF__2!(d);
                        return r as *mut _object;
                    }
                }
            }
        }
        // Issue #24999: use two shifts instead of ">> 2*PyLong_SHIFT" to avoid
        // undefined behaviour when LONG_MAX type is smaller than 60 bits
        //
        // a fits into a long, so b must too
        x = PyLong_AsLong(a as *mut _object);
        y = PyLong_AsLong(b as *mut _object);
        x = Py_ABS!(x);
        y = Py_ABS!(y);
        Py_DECREF__2!(a);
        Py_DECREF__2!(b);
        // usual Euclidean algorithm for longs
        while y != 0 {
            t = y;
            y = x % y;
            x = t;
        }
        return PyLong_FromLong(x);
    }
}

#[unsafe(link_section = ".text.slate_distinct.longobject.long_float")]
extern "C-unwind" fn long_float(v: *mut _object) -> *mut _object {
    let result: f64 = PyLong_AsDouble(v);
    let __v0: bool = result == -1.0f64 && !unsafe { PyErr_Occurred() }.is_null();
    if __v0 {
        return core::ptr::null_mut::<_object>();
    }
    unsafe { PyFloat_FromDouble(result) }
}

// [clinic input]
// @classmethod
// int.__new__ as long_new
//     x: object(c_default="NULL") = 0
//     /
//     base as obase: object(c_default="NULL") = 10
// [clinic start generated code]
/// [clinic end generated code: output=e47cfe777ab0f24c input=81c98f418af9eb6f]
fn long_new_impl(r#type: *mut _typeobject, x: *mut _object, obase: *mut _object) -> *mut _object {
    let mut base: i64 = 0;
    if r#type != unsafe { core::ptr::addr_of_mut!(PyLong_Type) } {
        return long_subtype_new(r#type, x, obase);
    }
    // Wimp out
    if x.is_null() {
        if !obase.is_null() {
            unsafe { PyErr_SetString(PyExc_TypeError, c"int() missing string argument".as_ptr()) };
            return core::ptr::null_mut::<_object>();
        }
        return PyLong_FromLong(0i64);
    }
    // default base and limit, forward to standard implementation
    if obase.is_null() {
        return unsafe { PyNumber_Long(x) };
    }
    base = unsafe { PyNumber_AsSsize_t(obase, core::ptr::null_mut::<_object>()) };
    let __v0: bool = base == (1i32.wrapping_neg() as i64) && !unsafe { PyErr_Occurred() }.is_null();
    if __v0 {
        return core::ptr::null_mut::<_object>();
    }
    if base != 0 && base < 2 || base > 36 {
        unsafe {
            PyErr_SetString(
                PyExc_ValueError,
                c"int() base must be >= 2 and <= 36, or 0".as_ptr(),
            )
        };
        return core::ptr::null_mut::<_object>();
    }
    if PyUnicode_Check!(x) != 0 {
        return PyLong_FromUnicodeObject(x, base as i32);
    } else {
        let __v1: bool = PyByteArray_Check!(x) != 0 || PyBytes_Check!(x) != 0;
        if __v1 {
            let mut string: *const i8 = unsafe { core::mem::zeroed() };
            if PyByteArray_Check!(x) != 0 {
                string = PyByteArray_AS_STRING__2!(x) as *const i8;
            } else {
                string = PyBytes_AS_STRING!(x) as *const i8;
            }
            return _PyLong_FromBytes(string, Py_SIZE__2!(x), base as i32);
        } else {
            unsafe {
                PyErr_SetString(
                    PyExc_TypeError,
                    c"int() can't convert non-string with explicit base".as_ptr(),
                )
            };
            return core::ptr::null_mut::<_object>();
        }
    }
}

/// Wimpy, slow approach to tp_new calls for subtypes of int:
/// first create a regular int from whatever arguments we got,
/// then allocate a subtype instance and initialize it from
/// the regular int.  The regular int is then thrown away.
fn long_subtype_new(
    r#type: *mut _typeobject,
    x: *mut _object,
    obase: *mut _object,
) -> *mut _object {
    let mut n: i64 = 0;
    let tmp: *mut _longobject =
        long_new_impl(unsafe { core::ptr::addr_of_mut!(PyLong_Type) }, x, obase)
            as *mut _longobject;
    if tmp.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    n = _PyLong_DigitCount(tmp as *const _longobject);
    // Fast operations for single digit integers (including zero)
    // assume that there is always at least one digit present.
    if n == 0 {
        n = 1;
    }
    let newobj: *mut _longobject =
        (unsafe { unsafe { (*r#type).tp_alloc }.unwrap()(r#type, n) }) as *mut _longobject;
    if newobj.is_null() {
        Py_DECREF__2!(tmp);
        return core::ptr::null_mut::<_object>();
    }
    unsafe {
        (*newobj).long_value.lv_tag = (*tmp).long_value.lv_tag & 18446744073709551611u64;
    }
    for i in 0..n {
        unsafe {
            *((*newobj).long_value.ob_digit.as_mut_ptr() as *mut u32).wrapping_offset(i as isize) =
                *((*tmp).long_value.ob_digit.as_mut_ptr() as *mut u32).wrapping_offset(i as isize);
        }
    }
    Py_DECREF__2!(tmp);
    newobj as *mut _object
}

// [clinic input]
// int.__getnewargs__
// [clinic start generated code]
/// [clinic end generated code: output=839a49de3f00b61b input=5904770ab1fb8c75]
fn int___getnewargs___impl(self_: *mut _object) -> *mut _object {
    unsafe { Py_BuildValue(c"(N)".as_ptr(), _PyLong_Copy(self_ as *mut _longobject)) }
}

#[unsafe(link_section = ".text.slate_distinct.longobject.long_get0")]
extern "C-unwind" fn long_get0(
    _unused_self: *mut _object,
    _unused_context: *mut (),
) -> *mut _object {
    PyLong_FromLong(0i64)
}

#[unsafe(link_section = ".text.slate_distinct.longobject.long_get1")]
extern "C-unwind" fn long_get1(
    _unused_self: *mut _object,
    _unused_ignored: *mut (),
) -> *mut _object {
    PyLong_FromLong(1i64)
}

// [clinic input]
// int.__format__
//
//     format_spec: unicode
//     /
//
// Convert to a string according to format_spec.
// [clinic start generated code]
/// [clinic end generated code: output=b4929dee9ae18689 input=d5e1254a47e8d1dc]
fn int___format___impl(self_: *mut _object, format_spec: *mut _object) -> *mut _object {
    let mut writer: __SlateRecord_unicodeobject_h_539 = unsafe { core::mem::zeroed() };
    let mut ret: i32 = 0;
    unsafe { _PyUnicodeWriter_Init(core::ptr::addr_of_mut!(writer)) };
    ret = unsafe {
        _PyLong_FormatAdvancedWriter(
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

// Return a pair (q, r) such that a = b * q + r, and
// abs(r) <= abs(b)/2, with equality possible only if q is even.
// In other words, q == a / b, rounded to the nearest integer using
// round-half-to-even.
#[unsafe(no_mangle)]
extern "C-unwind" fn _PyLong_DivmodNear(a: *mut _object, b: *mut _object) -> *mut _object {
    // fix up quotient
    // borrowed reference
    let mut one: *mut _object = unsafe { core::mem::zeroed() };
    // Equivalent Python code:
    //
    // def divmod_near(a, b):
    //     q, r = divmod(a, b)
    //     # round up if either r / b > 0.5, or r / b == 0.5 and q is odd.
    //     # The expression r / b > 0.5 is equivalent to 2 * r > b if b is
    //     # positive, 2 * r < b if b negative.
    //     greater_than_half = 2*r > b if b > 0 else 2*r < b
    //     exactly_half = 2*r == b
    //     if greater_than_half or exactly_half and q % 2 == 1:
    //         q += 1
    //         r -= b
    //     return q, r
    let mut __v1: bool = unsafe { core::mem::zeroed() };
    let mut quo_is_neg: i32 = unsafe { core::mem::zeroed() };
    let mut temp: *mut _object = unsafe { core::mem::zeroed() };
    let mut result: *mut _object = unsafe { core::mem::zeroed() };
    let mut twice_rem: *mut _object = unsafe { core::mem::zeroed() };
    let mut rem: *mut _longobject = unsafe { core::mem::zeroed() };
    let mut quo: *mut _longobject = unsafe { core::mem::zeroed() };
    unsafe {
        quo = core::ptr::null_mut::<_longobject>();
        rem = core::ptr::null_mut::<_longobject>();
        if PyLong_Check!(a) == 0 {
            __v1 = true;
        } else {
            __v1 = PyLong_Check!(b) == 0;
        }
        if __v1 {
            PyErr_SetString(
                PyExc_TypeError,
                c"non-integer arguments in division".as_ptr(),
            );
            return core::ptr::null_mut::<_object>();
        } else {
            '__join_0: {
                // Do a and b have different signs?  If so, quotient is negative.
                quo_is_neg = ((_PyLong_IsNegative(a as *const _longobject) as i32)
                    != (_PyLong_IsNegative(b as *const _longobject) as i32))
                    as i32;
                if long_divrem(
                    a as *mut _longobject,
                    b as *mut _longobject,
                    &mut (*core::ptr::addr_of_mut!(quo)),
                    core::ptr::addr_of_mut!(rem),
                ) >= 0
                {
                    // compare twice the remainder with the divisor, to see
                    // if we need to adjust the quotient and remainder
                    twice_rem = long_lshift_int64(rem, 1i64);
                    if !twice_rem.is_null() {
                        if quo_is_neg != 0 {
                            temp = long_neg(twice_rem as *mut _longobject) as *mut _object;
                            Py_SETREF!(twice_rem, temp);
                            if twice_rem.is_null() {
                                break '__join_0;
                            }
                        }
                        let cmp: i64 =
                            long_compare(twice_rem as *mut _longobject, b as *mut _longobject);
                        Py_DECREF!(twice_rem);
                        let quo_is_odd: i32 = (*((*quo).long_value.ob_digit.as_mut_ptr()
                            as *mut u32)
                            .wrapping_offset(0 as isize)
                            & 1
                            != 0) as i32;
                        if (if _PyLong_IsNegative(b as *const _longobject) {
                            (cmp < 0) as i32
                        } else {
                            (cmp > 0) as i32
                        }) != 0
                            || cmp == 0 && quo_is_odd != 0
                        {
                            one = _PyLong_GetOne();
                            if quo_is_neg != 0 {
                                temp = long_sub(quo, one as *mut _longobject) as *mut _object;
                            } else {
                                temp = long_add(quo, one as *mut _longobject) as *mut _object;
                            }
                            Py_SETREF__4!(quo, temp as *mut _longobject);
                            if quo.is_null() {
                                break '__join_0;
                            } else {
                                // and remainder
                                if quo_is_neg != 0 {
                                    temp = long_add(rem, b as *mut _longobject) as *mut _object;
                                } else {
                                    temp = long_sub(rem, b as *mut _longobject) as *mut _object;
                                }
                                Py_SETREF__4!(rem, temp as *mut _longobject);
                                if rem.is_null() {
                                    break '__join_0;
                                }
                            }
                        }
                        result = PyTuple_New(2i64);
                        if !result.is_null() {
                            // PyTuple_SET_ITEM steals references
                            PyTuple_SET_ITEM!(result, 0i32, quo as *mut _object);
                            PyTuple_SET_ITEM!(result, 1i32, rem as *mut _object);
                            return result;
                        }
                    }
                }
            }
            Py_XDECREF__2!(quo);
            Py_XDECREF__2!(rem);
            return core::ptr::null_mut::<_object>();
        }
    }
}

// [clinic input]
// int.__round__
//
//     ndigits as o_ndigits: object = None
//     /
//
// Rounding an Integral returns itself.
//
// Rounding with an ndigits argument also returns an integer.
// [clinic start generated code]
/// [clinic end generated code: output=954fda6b18875998 input=30c2aec788263144]
fn int___round___impl(self_: *mut _object, o_ndigits: *mut _object) -> *mut _object {
    // To round an integer m to the nearest 10**n (n positive), we make use of
    // the divmod_near operation, defined by:
    //
    //   divmod_near(a, b) = (q, r)
    //
    // where q is the nearest integer to the quotient a / b (the
    // nearest even integer in the case of a tie) and r == a - q * b.
    // Hence q * b = a - r is the nearest multiple of b to a,
    // preferring even multiples in the case of a tie.
    //
    // So the nearest multiple of 10**n to m is:
    //
    //   m - divmod_near(m, 10**n)[1].
    if o_ndigits == unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } {
        return long_long(self_);
    }
    let mut ndigits: *mut _object = unsafe { _PyNumber_Index(o_ndigits) };
    if ndigits.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    // if ndigits >= 0 then no rounding is necessary; return self unchanged
    if !_PyLong_IsNegative(ndigits as *const _longobject) {
        Py_DECREF!(ndigits);
        return long_long(self_);
    }
    // result = self - divmod_near(self, 10 ** -ndigits)[1]
    let mut temp: *mut _object = long_neg(ndigits as *mut _longobject) as *mut _object;
    Py_SETREF!(ndigits, temp);
    if ndigits.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let mut result: *mut _object = PyLong_FromLong(10i64);
    if result.is_null() {
        Py_DECREF!(ndigits);
        return core::ptr::null_mut::<_object>();
    }
    temp = long_pow(result, ndigits, unsafe {
        core::ptr::addr_of_mut!(_Py_NoneStruct)
    });
    Py_DECREF!(ndigits);
    Py_SETREF!(result, temp);
    if result.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    temp = _PyLong_DivmodNear(self_, result);
    Py_SETREF!(result, temp);
    if result.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    temp = long_sub(
        self_ as *mut _longobject,
        PyTuple_GET_ITEM!(result, 1i32) as *mut _longobject,
    ) as *mut _object;
    Py_SETREF!(result, temp);
    result
}

// [clinic input]
// int.__sizeof__ -> Py_ssize_t
//
// Returns size in memory, in bytes.
// [clinic start generated code]
/// [clinic end generated code: output=3303f008eaa6a0a5 input=9b51620c76fc4507]
fn int___sizeof___impl(self_: *mut _object) -> i64 {
    // using Py_MAX(..., 1) because we always allocate space for at least
    // one digit, even though the integer zero has a digit count of 0
    let ndigits: i64 = Py_MAX__4!(_PyLong_DigitCount(self_ as *const _longobject), 1i32);
    unsafe { (*Py_TYPE!(self_)).tp_basicsize }
        .wrapping_add(unsafe { (*Py_TYPE!(self_)).tp_itemsize }.wrapping_mul(ndigits))
}

// [clinic input]
// int.bit_length
//
// Number of bits necessary to represent self in binary.
//
// >>> bin(37)
// '0b100101'
// >>> (37).bit_length()
// 6
// [clinic start generated code]
/// [clinic end generated code: output=fc1977c9353d6a59 input=e4eb7a587e849a32]
fn int_bit_length_impl(self_: *mut _object) -> *mut _object {
    let nbits: i64 = _PyLong_NumBits(self_);
    PyLong_FromInt64(nbits)
}

fn popcount_digit(d: u32) -> i32 {
    // digit can be larger than uint32_t, but only PyLong_SHIFT bits
    //
    // of it will be ever used.
    _Py_popcount32(d)
}

// [clinic input]
// int.bit_count
//
// Number of ones in the binary representation of the absolute value of self.
//
// Also known as the population count.
//
// >>> bin(13)
// '0b1101'
// >>> (13).bit_count()
// 3
// [clinic start generated code]
/// [clinic end generated code: output=2e571970daf1e5c3 input=7e0adef8e8ccdf2e]
fn int_bit_count_impl(self_: *mut _object) -> *mut _object {
    let z: *mut _longobject = self_ as *mut _longobject;
    let ndigits: i64 = _PyLong_DigitCount(z as *const _longobject);
    let mut bit_count: i64 = 0;
    for i in 0..ndigits {
        let __v0: i32 = popcount_digit(unsafe {
            *((*z).long_value.ob_digit.as_mut_ptr() as *mut u32).wrapping_offset(i as isize)
        });
        bit_count = bit_count.wrapping_add(__v0 as i64);
    }
    PyLong_FromInt64(bit_count)
}

// [clinic input]
// int.as_integer_ratio
//
// Return a pair of integers, whose ratio is equal to the original int.
//
// The ratio is in lowest terms and has a positive denominator.
//
// >>> (10).as_integer_ratio()
// (10, 1)
// >>> (-10).as_integer_ratio()
// (-10, 1)
// >>> (0).as_integer_ratio()
// (0, 1)
// [clinic start generated code]
/// [clinic end generated code: output=e60803ae1cc8621a input=384ff1766634bec2]
fn int_as_integer_ratio_impl(self_: *mut _object) -> *mut _object {
    let numerator: *mut _object = long_long(self_);
    if numerator.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let ratio_tuple: *mut _object = unsafe { PyTuple_Pack(2i64, numerator, _PyLong_GetOne()) };
    Py_DECREF!(numerator);
    ratio_tuple
}

// [clinic input]
// int.to_bytes
//
//     length: Py_ssize_t = 1
//         Length of bytes object to use.  An OverflowError is raised if
//         the integer is not representable with the given number of bytes.
//         Default is length 1.
//     byteorder: unicode(c_default="NULL") = "big"
//         The byte order used to represent the integer.  If byteorder is
//         'big', the most significant byte is at the beginning of the byte
//         array.  If byteorder is 'little', the most significant byte is at
//         the end of the byte array.  To request the native byte order of
//         the host system, use sys.byteorder as the byte order value.
//         Default is to use 'big'.
//     *
//     signed as is_signed: bool = False
//         Determines whether two's complement is used to represent the
//         integer.  If signed is False and a negative integer is given,
//         an OverflowError is raised.
//
// Return an array of bytes representing an integer.
// [clinic start generated code]
/// [clinic end generated code: output=89c801df114050a3 input=661d7e615a4f132d]
fn int_to_bytes_impl(
    self_: *mut _object,
    length: i64,
    byteorder: *mut _object,
    is_signed: i32,
) -> *mut _object {
    let mut little_endian: i32 = 0;
    if byteorder.is_null() {
        little_endian = 0;
    } else {
        if (unsafe {
            _PyUnicode_Equal(
                byteorder,
                core::ptr::addr_of_mut!(
                    _PyRuntime
                        .static_objects
                        .singletons
                        .strings
                        .identifiers
                        ._py_little
                        ._ascii
                        .ob_base
                ),
            )
        }) != 0
        {
            little_endian = 1;
        } else {
            if (unsafe {
                _PyUnicode_Equal(
                    byteorder,
                    core::ptr::addr_of_mut!(
                        _PyRuntime
                            .static_objects
                            .singletons
                            .strings
                            .identifiers
                            ._py_big
                            ._ascii
                            .ob_base
                    ),
                )
            }) != 0
            {
                little_endian = 0;
            } else {
                unsafe {
                    PyErr_SetString(
                        PyExc_ValueError,
                        c"byteorder must be either 'little' or 'big'".as_ptr(),
                    )
                };
                return core::ptr::null_mut::<_object>();
            }
        }
    }
    if length < 0 {
        unsafe {
            PyErr_SetString(
                PyExc_ValueError,
                c"length argument must be non-negative".as_ptr(),
            )
        };
        return core::ptr::null_mut::<_object>();
    }
    let bytes: *mut _object = unsafe { PyBytes_FromStringAndSize(core::ptr::null::<i8>(), length) };
    if bytes.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    if _PyLong_AsByteArray(
        self_ as *mut _longobject,
        PyBytes_AS_STRING!(bytes) as *mut u8,
        length as u64,
        little_endian,
        is_signed,
        1i32,
    ) < 0
    {
        Py_DECREF!(bytes);
        return core::ptr::null_mut::<_object>();
    }
    bytes
}

// [clinic input]
// @classmethod
// int.from_bytes
//
//     bytes as bytes_obj: object
//         Holds the array of bytes to convert.  The argument must either
//         support the buffer protocol or be an iterable object producing
//         bytes.  Bytes and bytearray are examples of built-in objects that
//         support the buffer protocol.
//     byteorder: unicode(c_default="NULL") = "big"
//         The byte order used to represent the integer.  If byteorder is
//         'big', the most significant byte is at the beginning of the byte
//         array.  If byteorder is 'little', the most significant byte is at
//         the end of the byte array.  To request the native byte order of
//         the host system, use sys.byteorder as the byte order value.
//         Default is to use 'big'.
//     *
//     signed as is_signed: bool = False
//         Indicates whether two's complement is used to represent the
//         integer.
//
// Return the integer represented by the given array of bytes.
// [clinic start generated code]
/// [clinic end generated code: output=efc5d68e31f9314f input=95801e50b942e164]
fn int_from_bytes_impl(
    r#type: *mut _typeobject,
    bytes_obj: *mut _object,
    byteorder: *mut _object,
    is_signed: i32,
) -> *mut _object {
    let mut little_endian: i32 = 0;
    let mut long_obj: *mut _object = unsafe { core::mem::zeroed() };
    if byteorder.is_null() {
        little_endian = 0;
    } else {
        if (unsafe {
            _PyUnicode_Equal(
                byteorder,
                core::ptr::addr_of_mut!(
                    _PyRuntime
                        .static_objects
                        .singletons
                        .strings
                        .identifiers
                        ._py_little
                        ._ascii
                        .ob_base
                ),
            )
        }) != 0
        {
            little_endian = 1;
        } else {
            if (unsafe {
                _PyUnicode_Equal(
                    byteorder,
                    core::ptr::addr_of_mut!(
                        _PyRuntime
                            .static_objects
                            .singletons
                            .strings
                            .identifiers
                            ._py_big
                            ._ascii
                            .ob_base
                    ),
                )
            }) != 0
            {
                little_endian = 0;
            } else {
                unsafe {
                    PyErr_SetString(
                        PyExc_ValueError,
                        c"byteorder must be either 'little' or 'big'".as_ptr(),
                    )
                };
                return core::ptr::null_mut::<_object>();
            }
        }
    }
    let bytes: *mut _object = unsafe { PyObject_Bytes(bytes_obj) };
    if bytes.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    long_obj = _PyLong_FromByteArray(
        (PyBytes_AS_STRING!(bytes) as *mut u8) as *const u8,
        Py_SIZE__2!(bytes) as u64,
        little_endian,
        is_signed,
    );
    Py_DECREF!(bytes);
    if !long_obj.is_null() && r#type != unsafe { core::ptr::addr_of_mut!(PyLong_Type) } {
        Py_SETREF!(long_obj, unsafe {
            PyObject_CallOneArg(r#type as *mut _object, long_obj)
        });
    }
    long_obj
}

#[unsafe(link_section = ".text.slate_distinct.longobject.long_long_meth")]
extern "C-unwind" fn long_long_meth(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    long_long(self_)
}

#[unsafe(link_section = ".text.slate_distinct.longobject.long_long_getter")]
extern "C-unwind" fn long_long_getter(
    self_: *mut _object,
    _unused_ignored: *mut (),
) -> *mut _object {
    long_long(self_)
}

// [clinic input]
// int.is_integer
//
// Returns True. Exists for duck type compatibility with float.is_integer.
// [clinic start generated code]
/// [clinic end generated code: output=90f8e794ce5430ef input=7e41c4d4416e05f2]
fn int_is_integer_impl(self_: *mut _object) -> *mut _object {
    _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_TrueStruct) })
}

#[unsafe(link_section = ".text.slate_distinct.longobject.long_vectorcall")]
extern "C-unwind" fn long_vectorcall(
    r#type: *mut _object,
    args: *const *mut _object,
    nargsf: u64,
    kwnames: *mut _object,
) -> *mut _object {
    let nargs: i64 = PyVectorcall_NARGS!(nargsf);
    if !kwnames.is_null() {
        let tstate: *mut _ts = PyThreadState_GET!();
        return unsafe { _PyObject_MakeTpCall(tstate, r#type, args, nargs, kwnames) };
    }
    match nargs {
        0 => {
            return _PyLong_GetZero();
        }
        1 => {
            return unsafe { PyNumber_Long(*args.wrapping_offset(0 as isize)) };
        }
        2 => {
            return long_new_impl(
                _PyType_CAST!(r#type),
                unsafe { *args.wrapping_offset(0 as isize) },
                unsafe { *args.wrapping_offset(1 as isize) },
            );
        }
        _ => {
            return unsafe {
                PyErr_Format(
                    PyExc_TypeError,
                    c"int expected at most 2 arguments, got %zd".as_ptr(),
                    nargs,
                )
            };
        }
    }
    unsafe { core::mem::zeroed() }
}

/// sentinel
static mut long_methods: __SlateAlign16<[PyMethodDef; 15]> = __SlateAlign16([
    {
        let mut __t0: PyMethodDef = unsafe { core::mem::zeroed() };
        __t0.ml_name = (b"conjugate\0".as_ptr() as *mut i8) as *const i8;
        __t0.ml_meth = Some(long_long_meth);
        __t0.ml_flags = 4i32;
        __t0.ml_doc =
            (b"Returns self, the complex conjugate of any int.\0".as_ptr() as *mut i8) as *const i8;
        __t0
    },
    {
        let mut __t1: PyMethodDef = unsafe { core::mem::zeroed() };
        __t1.ml_name = (b"bit_length\0".as_ptr() as *mut i8) as *const i8;
        __t1.ml_meth = Some(int_bit_length);
        __t1.ml_flags = 4i32;
        __t1.ml_doc = unsafe { core::ptr::addr_of!(int_bit_length__doc__.0) as *const i8 };
        __t1
    },
    {
        let mut __t2: PyMethodDef = unsafe { core::mem::zeroed() };
        __t2.ml_name = (b"bit_count\0".as_ptr() as *mut i8) as *const i8;
        __t2.ml_meth = Some(int_bit_count);
        __t2.ml_flags = 4i32;
        __t2.ml_doc = unsafe { core::ptr::addr_of!(int_bit_count__doc__.0) as *const i8 };
        __t2
    },
    {
        let mut __t3: PyMethodDef = unsafe { core::mem::zeroed() };
        __t3.ml_name = (b"to_bytes\0".as_ptr() as *mut i8) as *const i8;
        __t3.ml_meth = unsafe {
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
                >(Some(int_to_bytes))
            })
        };
        __t3.ml_flags = 130i32;
        __t3.ml_doc = unsafe { core::ptr::addr_of!(int_to_bytes__doc__.0) as *const i8 };
        __t3
    },
    {
        let mut __t4: PyMethodDef = unsafe { core::mem::zeroed() };
        __t4.ml_name = (b"from_bytes\0".as_ptr() as *mut i8) as *const i8;
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
                >(Some(int_from_bytes))
            })
        };
        __t4.ml_flags = 146i32;
        __t4.ml_doc = unsafe { core::ptr::addr_of!(int_from_bytes__doc__.0) as *const i8 };
        __t4
    },
    {
        let mut __t5: PyMethodDef = unsafe { core::mem::zeroed() };
        __t5.ml_name = (b"as_integer_ratio\0".as_ptr() as *mut i8) as *const i8;
        __t5.ml_meth = Some(int_as_integer_ratio);
        __t5.ml_flags = 4i32;
        __t5.ml_doc = unsafe { core::ptr::addr_of!(int_as_integer_ratio__doc__.0) as *const i8 };
        __t5
    },
    {
        let mut __t6: PyMethodDef = unsafe { core::mem::zeroed() };
        __t6.ml_name = (b"__trunc__\0".as_ptr() as *mut i8) as *const i8;
        __t6.ml_meth = Some(long_long_meth);
        __t6.ml_flags = 4i32;
        __t6.ml_doc =
            (b"Truncating an Integral returns itself.\0".as_ptr() as *mut i8) as *const i8;
        __t6
    },
    {
        let mut __t7: PyMethodDef = unsafe { core::mem::zeroed() };
        __t7.ml_name = (b"__floor__\0".as_ptr() as *mut i8) as *const i8;
        __t7.ml_meth = Some(long_long_meth);
        __t7.ml_flags = 4i32;
        __t7.ml_doc = (b"Flooring an Integral returns itself.\0".as_ptr() as *mut i8) as *const i8;
        __t7
    },
    {
        let mut __t8: PyMethodDef = unsafe { core::mem::zeroed() };
        __t8.ml_name = (b"__ceil__\0".as_ptr() as *mut i8) as *const i8;
        __t8.ml_meth = Some(long_long_meth);
        __t8.ml_flags = 4i32;
        __t8.ml_doc =
            (b"Ceiling of an Integral returns itself.\0".as_ptr() as *mut i8) as *const i8;
        __t8
    },
    {
        let mut __t9: PyMethodDef = unsafe { core::mem::zeroed() };
        __t9.ml_name = (b"__round__\0".as_ptr() as *mut i8) as *const i8;
        __t9.ml_meth = unsafe {
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
                >(Some(int___round__))
            })
        };
        __t9.ml_flags = 128i32;
        __t9.ml_doc = unsafe { core::ptr::addr_of!(int___round____doc__.0) as *const i8 };
        __t9
    },
    {
        let mut __t10: PyMethodDef = unsafe { core::mem::zeroed() };
        __t10.ml_name = (b"__getnewargs__\0".as_ptr() as *mut i8) as *const i8;
        __t10.ml_meth = Some(int___getnewargs__);
        __t10.ml_flags = 4i32;
        __t10.ml_doc = unsafe { core::ptr::addr_of!(int___getnewargs____doc__.0) as *const i8 };
        __t10
    },
    {
        let mut __t11: PyMethodDef = unsafe { core::mem::zeroed() };
        __t11.ml_name = (b"__format__\0".as_ptr() as *mut i8) as *const i8;
        __t11.ml_meth = Some(int___format__);
        __t11.ml_flags = 8i32;
        __t11.ml_doc = unsafe { core::ptr::addr_of!(int___format____doc__.0) as *const i8 };
        __t11
    },
    {
        let mut __t12: PyMethodDef = unsafe { core::mem::zeroed() };
        __t12.ml_name = (b"__sizeof__\0".as_ptr() as *mut i8) as *const i8;
        __t12.ml_meth = Some(int___sizeof__);
        __t12.ml_flags = 4i32;
        __t12.ml_doc = unsafe { core::ptr::addr_of!(int___sizeof____doc__.0) as *const i8 };
        __t12
    },
    {
        let mut __t13: PyMethodDef = unsafe { core::mem::zeroed() };
        __t13.ml_name = (b"is_integer\0".as_ptr() as *mut i8) as *const i8;
        __t13.ml_meth = Some(int_is_integer);
        __t13.ml_flags = 4i32;
        __t13.ml_doc = unsafe { core::ptr::addr_of!(int_is_integer__doc__.0) as *const i8 };
        __t13
    },
    {
        let mut __t14: PyMethodDef = unsafe { core::mem::zeroed() };
        __t14.ml_name = core::ptr::null::<i8>();
        __t14.ml_meth = None;
        __t14
    },
]);

/// Sentinel
static mut long_getset: __SlateAlign16<[PyGetSetDef; 5]> = __SlateAlign16([
    PyGetSetDef {
        name: (b"real\0".as_ptr() as *mut i8) as *const i8,
        get: Some(long_long_getter),
        set: None,
        doc: (b"the real part of a complex number\0".as_ptr() as *mut i8) as *const i8,
        closure: core::ptr::null_mut::<()>(),
    },
    PyGetSetDef {
        name: (b"imag\0".as_ptr() as *mut i8) as *const i8,
        get: Some(long_get0),
        set: None,
        doc: (b"the imaginary part of a complex number\0".as_ptr() as *mut i8) as *const i8,
        closure: core::ptr::null_mut::<()>(),
    },
    PyGetSetDef {
        name: (b"numerator\0".as_ptr() as *mut i8) as *const i8,
        get: Some(long_long_getter),
        set: None,
        doc: (b"the numerator of a rational number in lowest terms\0".as_ptr() as *mut i8)
            as *const i8,
        closure: core::ptr::null_mut::<()>(),
    },
    PyGetSetDef {
        name: (b"denominator\0".as_ptr() as *mut i8) as *const i8,
        get: Some(long_get1),
        set: None,
        doc: (b"the denominator of a rational number in lowest terms\0".as_ptr() as *mut i8)
            as *const i8,
        closure: core::ptr::null_mut::<()>(),
    },
    {
        let mut __t0: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t0.name = core::ptr::null::<i8>();
        __t0
    },
]);

static long_doc: __SlateAlign16<[i8; 605]> = __SlateAlign16([
    105i8, 110i8, 116i8, 40i8, 91i8, 120i8, 93i8, 41i8, 32i8, 45i8, 62i8, 32i8, 105i8, 110i8,
    116i8, 101i8, 103i8, 101i8, 114i8, 10i8, 105i8, 110i8, 116i8, 40i8, 120i8, 44i8, 32i8, 98i8,
    97i8, 115i8, 101i8, 61i8, 49i8, 48i8, 41i8, 32i8, 45i8, 62i8, 32i8, 105i8, 110i8, 116i8, 101i8,
    103i8, 101i8, 114i8, 10i8, 10i8, 67i8, 111i8, 110i8, 118i8, 101i8, 114i8, 116i8, 32i8, 97i8,
    32i8, 110i8, 117i8, 109i8, 98i8, 101i8, 114i8, 32i8, 111i8, 114i8, 32i8, 115i8, 116i8, 114i8,
    105i8, 110i8, 103i8, 32i8, 116i8, 111i8, 32i8, 97i8, 110i8, 32i8, 105i8, 110i8, 116i8, 101i8,
    103i8, 101i8, 114i8, 44i8, 32i8, 111i8, 114i8, 32i8, 114i8, 101i8, 116i8, 117i8, 114i8, 110i8,
    32i8, 48i8, 32i8, 105i8, 102i8, 32i8, 110i8, 111i8, 32i8, 97i8, 114i8, 103i8, 117i8, 109i8,
    101i8, 110i8, 116i8, 115i8, 10i8, 97i8, 114i8, 101i8, 32i8, 103i8, 105i8, 118i8, 101i8, 110i8,
    46i8, 32i8, 32i8, 73i8, 102i8, 32i8, 120i8, 32i8, 105i8, 115i8, 32i8, 97i8, 32i8, 110i8, 117i8,
    109i8, 98i8, 101i8, 114i8, 44i8, 32i8, 114i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 120i8,
    46i8, 95i8, 95i8, 105i8, 110i8, 116i8, 95i8, 95i8, 40i8, 41i8, 46i8, 32i8, 32i8, 70i8, 111i8,
    114i8, 32i8, 102i8, 108i8, 111i8, 97i8, 116i8, 105i8, 110i8, 103i8, 45i8, 112i8, 111i8, 105i8,
    110i8, 116i8, 10i8, 110i8, 117i8, 109i8, 98i8, 101i8, 114i8, 115i8, 44i8, 32i8, 116i8, 104i8,
    105i8, 115i8, 32i8, 116i8, 114i8, 117i8, 110i8, 99i8, 97i8, 116i8, 101i8, 115i8, 32i8, 116i8,
    111i8, 119i8, 97i8, 114i8, 100i8, 115i8, 32i8, 122i8, 101i8, 114i8, 111i8, 46i8, 10i8, 10i8,
    73i8, 102i8, 32i8, 120i8, 32i8, 105i8, 115i8, 32i8, 110i8, 111i8, 116i8, 32i8, 97i8, 32i8,
    110i8, 117i8, 109i8, 98i8, 101i8, 114i8, 32i8, 111i8, 114i8, 32i8, 105i8, 102i8, 32i8, 98i8,
    97i8, 115i8, 101i8, 32i8, 105i8, 115i8, 32i8, 103i8, 105i8, 118i8, 101i8, 110i8, 44i8, 32i8,
    116i8, 104i8, 101i8, 110i8, 32i8, 120i8, 32i8, 109i8, 117i8, 115i8, 116i8, 32i8, 98i8, 101i8,
    32i8, 97i8, 32i8, 115i8, 116i8, 114i8, 105i8, 110i8, 103i8, 44i8, 10i8, 98i8, 121i8, 116i8,
    101i8, 115i8, 44i8, 32i8, 111i8, 114i8, 32i8, 98i8, 121i8, 116i8, 101i8, 97i8, 114i8, 114i8,
    97i8, 121i8, 32i8, 105i8, 110i8, 115i8, 116i8, 97i8, 110i8, 99i8, 101i8, 32i8, 114i8, 101i8,
    112i8, 114i8, 101i8, 115i8, 101i8, 110i8, 116i8, 105i8, 110i8, 103i8, 32i8, 97i8, 110i8, 32i8,
    105i8, 110i8, 116i8, 101i8, 103i8, 101i8, 114i8, 32i8, 108i8, 105i8, 116i8, 101i8, 114i8, 97i8,
    108i8, 32i8, 105i8, 110i8, 32i8, 116i8, 104i8, 101i8, 10i8, 103i8, 105i8, 118i8, 101i8, 110i8,
    32i8, 98i8, 97i8, 115i8, 101i8, 46i8, 32i8, 32i8, 84i8, 104i8, 101i8, 32i8, 108i8, 105i8,
    116i8, 101i8, 114i8, 97i8, 108i8, 32i8, 99i8, 97i8, 110i8, 32i8, 98i8, 101i8, 32i8, 112i8,
    114i8, 101i8, 99i8, 101i8, 100i8, 101i8, 100i8, 32i8, 98i8, 121i8, 32i8, 39i8, 43i8, 39i8,
    32i8, 111i8, 114i8, 32i8, 39i8, 45i8, 39i8, 32i8, 97i8, 110i8, 100i8, 32i8, 98i8, 101i8, 32i8,
    115i8, 117i8, 114i8, 114i8, 111i8, 117i8, 110i8, 100i8, 101i8, 100i8, 10i8, 98i8, 121i8, 32i8,
    119i8, 104i8, 105i8, 116i8, 101i8, 115i8, 112i8, 97i8, 99i8, 101i8, 46i8, 32i8, 32i8, 84i8,
    104i8, 101i8, 32i8, 98i8, 97i8, 115i8, 101i8, 32i8, 100i8, 101i8, 102i8, 97i8, 117i8, 108i8,
    116i8, 115i8, 32i8, 116i8, 111i8, 32i8, 49i8, 48i8, 46i8, 32i8, 32i8, 86i8, 97i8, 108i8, 105i8,
    100i8, 32i8, 98i8, 97i8, 115i8, 101i8, 115i8, 32i8, 97i8, 114i8, 101i8, 32i8, 48i8, 32i8, 97i8,
    110i8, 100i8, 32i8, 50i8, 45i8, 51i8, 54i8, 46i8, 10i8, 66i8, 97i8, 115i8, 101i8, 32i8, 48i8,
    32i8, 109i8, 101i8, 97i8, 110i8, 115i8, 32i8, 116i8, 111i8, 32i8, 105i8, 110i8, 116i8, 101i8,
    114i8, 112i8, 114i8, 101i8, 116i8, 32i8, 116i8, 104i8, 101i8, 32i8, 98i8, 97i8, 115i8, 101i8,
    32i8, 102i8, 114i8, 111i8, 109i8, 32i8, 116i8, 104i8, 101i8, 32i8, 115i8, 116i8, 114i8, 105i8,
    110i8, 103i8, 32i8, 97i8, 115i8, 32i8, 97i8, 110i8, 32i8, 105i8, 110i8, 116i8, 101i8, 103i8,
    101i8, 114i8, 10i8, 105i8, 116i8, 101i8, 114i8, 97i8, 108i8, 46i8, 10i8, 62i8, 62i8, 62i8,
    32i8, 105i8, 110i8, 116i8, 40i8, 39i8, 48i8, 98i8, 49i8, 48i8, 48i8, 39i8, 44i8, 32i8, 98i8,
    97i8, 115i8, 101i8, 61i8, 48i8, 41i8, 10i8, 52i8, 0i8,
]);

/// nb_add
/// nb_subtract
/// nb_multiply
/// nb_remainder
/// nb_divmod
/// nb_power
/// nb_negative
/// tp_positive
/// tp_absolute
/// tp_bool
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
/// nb_index
static mut long_as_number: __SlateRecord_object_h_106 = {
    let mut __t0: __SlateRecord_object_h_106 = unsafe { core::mem::zeroed() };
    __t0.nb_add = Some(long_add_method);
    __t0.nb_subtract = Some(long_sub_method);
    __t0.nb_multiply = Some(long_mul_method);
    __t0.nb_remainder = Some(long_mod);
    __t0.nb_divmod = Some(long_divmod);
    __t0.nb_power = Some(long_pow);
    __t0.nb_negative = Some(long_neg_method);
    __t0.nb_positive = Some(long_long);
    __t0.nb_absolute = Some(long_abs_method);
    __t0.nb_bool = Some(long_bool);
    __t0.nb_invert = Some(long_invert);
    __t0.nb_lshift = Some(long_lshift_method);
    __t0.nb_rshift = Some(long_rshift);
    __t0.nb_and = Some(long_and);
    __t0.nb_xor = Some(long_xor);
    __t0.nb_or = Some(long_or);
    __t0.nb_int = Some(long_long);
    __t0.nb_reserved = core::ptr::null_mut::<()>();
    __t0.nb_float = Some(long_float);
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
    __t0.nb_floor_divide = Some(long_div);
    __t0.nb_true_divide = Some(long_true_divide);
    __t0.nb_inplace_floor_divide = None;
    __t0.nb_inplace_true_divide = None;
    __t0.nb_index = Some(long_long);
    __t0
};

// tp_name
// tp_basicsize
// tp_itemsize
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
static mut Int_InfoType: _typeobject = unsafe { core::mem::zeroed() };

static int_info__doc__: __SlateAlign16<[i8; 134]> = __SlateAlign16([
    115i8, 121i8, 115i8, 46i8, 105i8, 110i8, 116i8, 95i8, 105i8, 110i8, 102i8, 111i8, 10i8, 10i8,
    65i8, 32i8, 110i8, 97i8, 109i8, 101i8, 100i8, 32i8, 116i8, 117i8, 112i8, 108i8, 101i8, 32i8,
    116i8, 104i8, 97i8, 116i8, 32i8, 104i8, 111i8, 108i8, 100i8, 115i8, 32i8, 105i8, 110i8, 102i8,
    111i8, 114i8, 109i8, 97i8, 116i8, 105i8, 111i8, 110i8, 32i8, 97i8, 98i8, 111i8, 117i8, 116i8,
    32i8, 80i8, 121i8, 116i8, 104i8, 111i8, 110i8, 39i8, 115i8, 10i8, 105i8, 110i8, 116i8, 101i8,
    114i8, 110i8, 97i8, 108i8, 32i8, 114i8, 101i8, 112i8, 114i8, 101i8, 115i8, 101i8, 110i8, 116i8,
    97i8, 116i8, 105i8, 111i8, 110i8, 32i8, 111i8, 102i8, 32i8, 105i8, 110i8, 116i8, 101i8, 103i8,
    101i8, 114i8, 115i8, 46i8, 32i8, 32i8, 84i8, 104i8, 101i8, 32i8, 97i8, 116i8, 116i8, 114i8,
    105i8, 98i8, 117i8, 116i8, 101i8, 115i8, 32i8, 97i8, 114i8, 101i8, 32i8, 114i8, 101i8, 97i8,
    100i8, 32i8, 111i8, 110i8, 108i8, 121i8, 46i8, 0i8,
]);

static mut int_info_fields: __SlateAlign16<[PyStructSequence_Field; 5]> = __SlateAlign16([
    PyStructSequence_Field {
        name: (b"bits_per_digit\0".as_ptr() as *mut i8) as *const i8,
        doc: (b"size of a digit in bits\0".as_ptr() as *mut i8) as *const i8,
    },
    PyStructSequence_Field {
        name: (b"sizeof_digit\0".as_ptr() as *mut i8) as *const i8,
        doc: (b"size in bytes of the C type used to represent a digit\0".as_ptr() as *mut i8)
            as *const i8,
    },
    PyStructSequence_Field {
        name: (b"default_max_str_digits\0".as_ptr() as *mut i8) as *const i8,
        doc: (b"maximum string conversion digits limitation\0".as_ptr() as *mut i8) as *const i8,
    },
    PyStructSequence_Field {
        name: (b"str_digits_check_threshold\0".as_ptr() as *mut i8) as *const i8,
        doc: (b"minimum positive value for int_max_str_digits\0".as_ptr() as *mut i8) as *const i8,
    },
    PyStructSequence_Field {
        name: core::ptr::null::<i8>(),
        doc: core::ptr::null::<i8>(),
    },
]);

/// name
/// doc
/// fields
/// number of fields
static mut int_info_desc: PyStructSequence_Desc = {
    let mut __t0: PyStructSequence_Desc = unsafe { core::mem::zeroed() };
    __t0.name = (b"sys.int_info\0".as_ptr() as *mut i8) as *const i8;
    __t0.doc = unsafe { core::ptr::addr_of!(int_info__doc__.0) as *const i8 };
    __t0.fields =
        unsafe { core::ptr::addr_of_mut!(int_info_fields.0) as *mut PyStructSequence_Field };
    __t0.n_in_sequence = 4i32;
    __t0
};

#[unsafe(no_mangle)]
extern "C-unwind" fn PyLong_GetInfo() -> *mut _object {
    let mut int_info: *mut _object = unsafe { core::mem::zeroed() };
    let mut field: i32 = 0;
    int_info = unsafe { PyStructSequence_New(core::ptr::addr_of_mut!(Int_InfoType)) };
    if int_info.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let __v0: i32 = field;
    field = __v0.wrapping_add(1);
    unsafe { PyStructSequence_SetItem(int_info, __v0 as i64, PyLong_FromLong(30i64)) };
    let __v2: i32 = field;
    field = __v2.wrapping_add(1);
    unsafe {
        PyStructSequence_SetItem(
            int_info,
            __v2 as i64,
            PyLong_FromLong(core::mem::size_of::<u32>() as i64),
        )
    };
    // The following two fields were added after investigating uses of
    // sys.int_info in the wild: Exceedingly rarely used. The ONLY use found was
    // numba using sys.int_info.bits_per_digit as attribute access rather than
    // sequence unpacking. Cython and sympy also refer to sys.int_info but only
    // as info for debugging. No concern about adding these in a backport.
    let __v4: i32 = field;
    field = __v4.wrapping_add(1);
    unsafe { PyStructSequence_SetItem(int_info, __v4 as i64, PyLong_FromLong(4300i64)) };
    let __v6: i32 = field;
    field = __v6.wrapping_add(1);
    unsafe { PyStructSequence_SetItem(int_info, __v6 as i64, PyLong_FromLong(640i64)) };
    if !unsafe { PyErr_Occurred() }.is_null() {
        Py_CLEAR__2!(int_info);
        return core::ptr::null_mut::<_object>();
    }
    int_info
}

// runtime lifecycle
#[unsafe(no_mangle)]
extern "C-unwind" fn _PyLong_InitTypes(interp: *mut _is) -> __SlateRecord_initconfig_h_19 {
    let mut __slate_compound_0: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    let mut __slate_compound_1: __SlateRecord_initconfig_h_19 = unsafe { core::mem::zeroed() };
    // initialize int_info
    if _PyStructSequence_InitBuiltin(
        interp,
        unsafe { core::ptr::addr_of_mut!(Int_InfoType) },
        unsafe { core::ptr::addr_of_mut!(int_info_desc) },
    ) < 0
    {
        return unsafe {
            *{
                __slate_compound_0 = {
                    let mut __t0: __SlateRecord_initconfig_h_19 = core::mem::zeroed();
                    __t0._type = 1u32;
                    __t0.func = _PyStatus_GET_FUNC__46!() as *const i8;
                    __t0.err_msg = c"can't init int info type".as_ptr();
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
extern "C-unwind" fn _PyLong_FiniTypes(interp: *mut _is) {
    unsafe { _PyStructSequence_FiniBuiltin(interp, core::ptr::addr_of_mut!(Int_InfoType)) };
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyUnstable_Long_IsCompact(op: *const _longobject) -> i32 {
    _PyLong_IsCompact(op as *const _longobject)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyUnstable_Long_CompactValue(op: *const _longobject) -> i64 {
    _PyLong_CompactValue(op as *const _longobject)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyLong_FromInt32(value: i32) -> *mut _object {
    PYLONG_FROM_INT__2!(u32, i32, value);
    unsafe { core::mem::zeroed() }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyLong_FromUInt32(mut value: u32) -> *mut _object {
    PYLONG_FROM_UINT__2!(u32, value);
    unsafe { core::mem::zeroed() }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyLong_FromInt64(value: i64) -> *mut _object {
    PYLONG_FROM_INT!(u64, i64, value);
    unsafe { core::mem::zeroed() }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyLong_FromUInt64(mut value: u64) -> *mut _object {
    PYLONG_FROM_UINT!(u64, value);
    unsafe { core::mem::zeroed() }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyLong_AsInt32(obj: *mut _object, result: *mut i32) -> i32 {
    loop {
        let mut value: i32 = 0;
        let flags: i32 = 3 | 16;
        let bytes: i64 = PyLong_AsNativeBytes(
            obj,
            core::ptr::addr_of_mut!(value) as *mut (),
            core::mem::size_of::<i32>() as i64,
            flags,
        );
        if bytes < 0 {
            return 1i32.wrapping_neg();
        }
        if (bytes as u64) > (core::mem::size_of::<i32>() as u64) {
            unsafe {
                PyErr_SetString(
                    PyExc_OverflowError,
                    c"Python int too large to convert to C int32_t".as_ptr(),
                )
            };
            return 1i32.wrapping_neg();
        }
        unsafe {
            *result = value;
        }
        return 0;
    }
    unsafe { core::mem::zeroed() }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyLong_AsInt64(obj: *mut _object, result: *mut i64) -> i32 {
    loop {
        let mut value: i64 = 0;
        let flags: i32 = 3 | 16;
        let bytes: i64 = PyLong_AsNativeBytes(
            obj,
            core::ptr::addr_of_mut!(value) as *mut (),
            core::mem::size_of::<i64>() as i64,
            flags,
        );
        if bytes < 0 {
            return 1i32.wrapping_neg();
        }
        if (bytes as u64) > (core::mem::size_of::<i64>() as u64) {
            unsafe {
                PyErr_SetString(
                    PyExc_OverflowError,
                    c"Python int too large to convert to C int64_t".as_ptr(),
                )
            };
            return 1i32.wrapping_neg();
        }
        unsafe {
            *result = value;
        }
        return 0;
    }
    unsafe { core::mem::zeroed() }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyLong_AsUInt32(obj: *mut _object, result: *mut u32) -> i32 {
    loop {
        let mut value: u32 = 0;
        let flags: i32 = 3 | 4 | 8 | 16;
        let bytes: i64 = PyLong_AsNativeBytes(
            obj,
            core::ptr::addr_of_mut!(value) as *mut (),
            core::mem::size_of::<u32>() as i64,
            flags,
        );
        if bytes < 0 {
            return 1i32.wrapping_neg();
        }
        if (bytes as u64) > (core::mem::size_of::<u32>() as u64) {
            unsafe {
                PyErr_SetString(
                    PyExc_OverflowError,
                    c"Python int too large to convert to C uint32_t".as_ptr(),
                )
            };
            return 1i32.wrapping_neg();
        }
        unsafe {
            *result = value;
        }
        return 0;
    }
    unsafe { core::mem::zeroed() }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyLong_AsUInt64(obj: *mut _object, result: *mut u64) -> i32 {
    loop {
        let mut value: u64 = 0;
        let flags: i32 = 3 | 4 | 8 | 16;
        let bytes: i64 = PyLong_AsNativeBytes(
            obj,
            core::ptr::addr_of_mut!(value) as *mut (),
            core::mem::size_of::<u64>() as i64,
            flags,
        );
        if bytes < 0 {
            return 1i32.wrapping_neg();
        }
        if (bytes as u64) > (core::mem::size_of::<u64>() as u64) {
            unsafe {
                PyErr_SetString(
                    PyExc_OverflowError,
                    c"Python int too large to convert to C uint64_t".as_ptr(),
                )
            };
            return 1i32.wrapping_neg();
        }
        unsafe {
            *result = value;
        }
        return 0;
    }
    unsafe { core::mem::zeroed() }
}

/// least significant first
static PyLong_LAYOUT: PyLongLayout = PyLongLayout {
    bits_per_digit: 30u8,
    digit_size: (core::mem::size_of::<u32>() as u64) as u8,
    digits_order: 1i32.wrapping_neg() as i8,
    digit_endianness: 1i32.wrapping_neg() as i8,
};

#[unsafe(no_mangle)]
extern "C-unwind" fn PyLong_GetNativeLayout() -> *const PyLongLayout {
    unsafe { core::ptr::addr_of!(PyLong_LAYOUT) }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyLong_Export(obj: *mut _object, export_long: *mut PyLongExport) -> i32 {
    if PyLong_Check!(obj) == 0 {
        unsafe {
            core::ptr::write_bytes(
                ((export_long as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                (0i32 as i32) as u8,
                ((core::mem::size_of::<PyLongExport>() as u64) as usize) as usize,
            )
        };
        unsafe { PyErr_Format(PyExc_TypeError, c"expect int, got %T".as_ptr(), obj) };
        return 1i32.wrapping_neg();
    }
    // Fast-path: try to convert to a int64_t
    let mut overflow: i32 = 0;
    let value: i64 = PyLong_AsLongAndOverflow(obj, core::ptr::addr_of_mut!(overflow));
    while false {}
    // the function cannot fail since obj is a PyLongObject
    if overflow == 0 {
        unsafe {
            (*export_long).value = value;
        }
        unsafe {
            (*export_long).negative = 0;
        }
        unsafe {
            (*export_long).ndigits = 0;
        }
        unsafe {
            (*export_long).digits = core::ptr::null::<()>();
        }
        unsafe {
            (*export_long)._reserved = 0;
        }
    } else {
        let self_: *mut _longobject = obj as *mut _longobject;
        unsafe {
            (*export_long).value = 0;
        }
        unsafe {
            (*export_long).negative = _PyLong_IsNegative(self_ as *const _longobject) as u8;
        }
        unsafe {
            (*export_long).ndigits = _PyLong_DigitCount(self_ as *const _longobject);
        }
        if (unsafe { (*export_long).ndigits }) == 0 {
            unsafe {
                (*export_long).ndigits = 1;
            }
        }
        unsafe {
            (*export_long).digits =
                ((*self_).long_value.ob_digit.as_mut_ptr() as *mut u32) as *const ();
        }
        unsafe {
            (*export_long)._reserved = Py_NewRef!(obj) as u64;
        }
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyLong_FreeExport(export_long: *mut PyLongExport) {
    let obj: *mut _object = (unsafe { (*export_long)._reserved }) as *mut _object;
    if !obj.is_null() {
        unsafe {
            (*export_long)._reserved = 0;
        }
        Py_DECREF!(obj);
    }
}

// --- PyLongWriter API ---------------------------------------------------
#[unsafe(no_mangle)]
extern "C-unwind" fn PyLongWriter_Create(
    negative: i32,
    ndigits: i64,
    digits: *mut *mut (),
) -> *mut PyLongWriter {
    let mut obj: *mut _longobject = unsafe { core::mem::zeroed() };
    unsafe {
        if ndigits <= 0 {
            PyErr_SetString(PyExc_ValueError, c"ndigits must be positive".as_ptr());
        } else {
            obj = long_alloc(ndigits);
            if !obj.is_null() {
                if negative != 0 {
                    _PyLong_FlipSign(obj);
                }
                *digits = ((*obj).long_value.ob_digit.as_mut_ptr() as *mut u32) as *mut ();
                return obj as *mut PyLongWriter;
            }
        }
        *digits = core::ptr::null_mut::<()>();
        return core::ptr::null_mut::<PyLongWriter>();
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyLongWriter_Discard(writer: *mut PyLongWriter) {
    if writer.is_null() {
        return;
    }
    let obj: *mut _longobject = writer as *mut _longobject;
    Py_DECREF__2!(obj);
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyLongWriter_Finish(writer: *mut PyLongWriter) -> *mut _object {
    let mut obj: *mut _longobject = writer as *mut _longobject;
    // Normalize and get singleton if possible
    obj = maybe_small_long(long_normalize(obj));
    obj as *mut _object
}
