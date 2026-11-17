//! Dictionary object implementation using a hash table
//! The size of 'size_t', as computed by sizeof.
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
// Use static_assert() on C11 and newer
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
macro_rules! ASSERT_CONSISTENT {
    () => {};
}

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

macro_rules! CACHED_KEYS {
    ($tp:expr) => {
        unsafe { (*(($tp) as *mut _heaptypeobject)).ht_cached_keys }
    };
}

macro_rules! CHECK__3 {
    ($expr:expr, $op:expr) => {
        loop {
            if !$expr {
                _PyObject_ASSERT_FAILED_MSG__7!($op, Py_STRINGIFY__50!());
            }
            if true {
                break;
            }
        }
    };
}

macro_rules! CHECK__7 {
    ($expr:expr, $op:expr) => {
        loop {
            if !$expr {
                _PyObject_ASSERT_FAILED_MSG!($op, Py_STRINGIFY!());
            }
            if true {
                break;
            }
        }
    };
}

macro_rules! CHECK__8 {
    ($expr:expr, $op:expr) => {
        loop {
            if !$expr {
                _PyObject_ASSERT_FAILED_MSG__20!($op, Py_STRINGIFY__30!());
            }
            if true {
                break;
            }
        }
    };
}

macro_rules! CHECK__12 {
    ($expr:expr, $op:expr) => {
        loop {
            if !$expr {
                _PyObject_ASSERT_FAILED_MSG__9!($op, Py_STRINGIFY!());
            }
            if true {
                break;
            }
        }
    };
}

macro_rules! CHECK__14 {
    ($expr:expr, $op:expr) => {
        loop {
            if !$expr {
                _PyObject_ASSERT_FAILED_MSG__37!($op, Py_STRINGIFY__10!());
            }
            if true {
                break;
            }
        }
    };
}

macro_rules! CHECK__16 {
    ($expr:expr, $op:expr) => {
        loop {
            if !$expr {
                _PyObject_ASSERT_FAILED_MSG__54!($op, Py_STRINGIFY__15!());
            }
            if true {
                break;
            }
        }
    };
}

macro_rules! CHECK__18 {
    ($expr:expr, $op:expr) => {
        loop {
            if !$expr {
                _PyObject_ASSERT_FAILED_MSG__44!($op, Py_STRINGIFY__32!());
            }
            if true {
                break;
            }
        }
    };
}

macro_rules! CHECK__20 {
    ($expr:expr, $op:expr) => {
        loop {
            if !$expr {
                _PyObject_ASSERT_FAILED_MSG__21!($op, Py_STRINGIFY__18!());
            }
            if true {
                break;
            }
        }
    };
}

macro_rules! CHECK__23 {
    ($expr:expr, $op:expr) => {
        loop {
            if ($expr) == 0 {
                _PyObject_ASSERT_FAILED_MSG__4!($op, Py_STRINGIFY__17!());
            }
            if true {
                break;
            }
        }
    };
}

macro_rules! CHECK__24 {
    ($expr:expr, $op:expr) => {
        loop {
            if !$expr {
                _PyObject_ASSERT_FAILED_MSG__50!($op, Py_STRINGIFY__36!());
            }
            if true {
                break;
            }
        }
    };
}

macro_rules! CHECK__26 {
    ($expr:expr, $op:expr) => {
        loop {
            if !$expr {
                _PyObject_ASSERT_FAILED_MSG__15!($op, Py_STRINGIFY__12!());
            }
            if true {
                break;
            }
        }
    };
}

macro_rules! CHECK__28 {
    ($expr:expr, $op:expr) => {
        loop {
            if !$expr {
                _PyObject_ASSERT_FAILED_MSG__30!($op, Py_STRINGIFY__24!());
            }
            if true {
                break;
            }
        }
    };
}

macro_rules! CHECK__31 {
    ($expr:expr, $op:expr) => {
        loop {
            if !$expr {
                _PyObject_ASSERT_FAILED_MSG__10!($op, Py_STRINGIFY__51!());
            }
            if true {
                break;
            }
        }
    };
}

macro_rules! CHECK__33 {
    ($expr:expr, $op:expr) => {
        loop {
            if !$expr {
                _PyObject_ASSERT_FAILED_MSG__12!($op, Py_STRINGIFY__41!());
            }
            if true {
                break;
            }
        }
    };
}

macro_rules! CHECK__34 {
    ($expr:expr, $op:expr) => {
        loop {
            if !$expr {
                _PyObject_ASSERT_FAILED_MSG__19!($op, Py_STRINGIFY__48!());
            }
            if true {
                break;
            }
        }
    };
}

macro_rules! CHECK__44 {
    ($expr:expr, $op:expr) => {
        loop {
            if !$expr {
                _PyObject_ASSERT_FAILED_MSG__42!($op, Py_STRINGIFY__6!());
            }
            if true {
                break;
            }
        }
    };
}

macro_rules! CHECK__45 {
    ($expr:expr, $op:expr) => {
        loop {
            if !$expr {
                _PyObject_ASSERT_FAILED_MSG__29!($op, Py_STRINGIFY__37!());
            }
            if true {
                break;
            }
        }
    };
}

macro_rules! CHECK__46 {
    ($expr:expr, $op:expr) => {
        loop {
            if !$expr {
                _PyObject_ASSERT_FAILED_MSG__51!($op, Py_STRINGIFY__46!());
            }
            if true {
                break;
            }
        }
    };
}

macro_rules! CHECK__47 {
    ($expr:expr, $op:expr) => {
        loop {
            if !$expr {
                _PyObject_ASSERT_FAILED_MSG__35!($op, Py_STRINGIFY__39!());
            }
            if true {
                break;
            }
        }
    };
}

macro_rules! CHECK__49 {
    ($expr:expr, $op:expr) => {
        loop {
            if ($expr) == 0 {
                _PyObject_ASSERT_FAILED_MSG__25!($op, Py_STRINGIFY__25!());
            }
            if true {
                break;
            }
        }
    };
}

macro_rules! CHECK__51 {
    ($expr:expr, $op:expr) => {
        loop {
            if !$expr {
                _PyObject_ASSERT_FAILED_MSG__39!($op, Py_STRINGIFY__3!());
            }
            if true {
                break;
            }
        }
    };
}

macro_rules! CHECK__54 {
    ($expr:expr, $op:expr) => {
        loop {
            if !$expr {
                _PyObject_ASSERT_FAILED_MSG__45!($op, Py_STRINGIFY__3!());
            }
            if true {
                break;
            }
        }
    };
}

macro_rules! DECREF_KEYS {
    ($dk:expr) => {{
        let __v: *mut _dictkeysobject = $dk;
        let __v2: i64 = unsafe { (*__v).dk_refcnt };
        unsafe {
            (*__v).dk_refcnt = __v2.wrapping_sub(1);
        }
        __v2
    }};
}

macro_rules! DK_IS_UNICODE {
    ($dk:expr) => {
        ((unsafe { (*$dk).dk_kind }) as i32) != 0
    };
}

macro_rules! DK_LOG_SIZE {
    ($dk:expr) => {
        _Py_RVALUE!(unsafe { (*$dk).dk_log2_size })
    };
}

macro_rules! DK_MASK {
    ($dk:expr) => {
        DK_SIZE!($dk).wrapping_sub(1)
    };
}

macro_rules! DK_SIZE {
    ($dk:expr) => {
        1i64 << (DK_LOG_SIZE!($dk) as i32)
    };
}

macro_rules! EVAL_CALL_STAT_INC_IF_FUNCTION {
    () => {};
}

macro_rules! FT_ATOMIC_STORE_PTR_RELAXED {
    ($value:expr, $new_value:expr) => {
        unsafe {
            $value = $new_value;
        }
    };
}

macro_rules! FT_ATOMIC_STORE_PTR_RELEASE__4 {
    ($value:expr) => {
        unsafe {
            $value = None;
        }
    };
}

macro_rules! FT_ATOMIC_STORE_PTR_RELEASE__2 {
    ($value:expr) => {
        unsafe {
            $value = core::ptr::null_mut::<_object>();
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

macro_rules! FT_ATOMIC_STORE_PTR {
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

macro_rules! FT_ATOMIC_STORE_UINT32_RELAXED__2 {
    ($value:expr, $new_value:expr) => {
        unsafe {
            $value = $new_value;
        }
    };
}

macro_rules! FT_ATOMIC_STORE_UINT32_RELAXED {
    ($value:expr, $new_value:expr) => {
        unsafe {
            $value = ($new_value) as u32;
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

macro_rules! FT_MUTEX_LOCK_FLAGS {
    () => {
        while false {}
    };
}

macro_rules! FT_MUTEX_UNLOCK {
    () => {
        while false {}
    };
}

/// GROWTH_RATE. Growth rate upon hitting maximum load.
/// Currently set to used*3.
/// This means that dicts double in size when growing without deletions,
/// but have more head room when the number of deletions is on a par with the
/// number of insertions.  See also bpo-17563 and bpo-33205.
///
/// GROWTH_RATE was set to used*4 up to version 3.2.
/// GROWTH_RATE was set to used*2 in version 3.3.0
/// GROWTH_RATE was set to used*2 + capacity/2 in 3.4.0-3.6.0.
macro_rules! GROWTH_RATE {
    ($d:expr) => {
        unsafe { (*$d).ma_used }.wrapping_mul(3)
    };
}

macro_rules! INCREF_KEYS {
    ($dk:expr) => {
        let __v: *mut _dictkeysobject = $dk;
        let __v2: i64 = unsafe { (*__v).dk_refcnt };
        unsafe {
            (*__v).dk_refcnt = __v2.wrapping_add(1);
        }
    };
}

macro_rules! IS_DICT_SHARED {
    () => {
        0i32
    };
}

macro_rules! LOAD_KEYS_NENTRIES {
    ($keys:expr) => {
        unsafe { (*$keys).dk_nentries }
    };
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

macro_rules! PyAnySet_Check {
    ($ob:expr) => {{
        let __v: bool = Py_IS_TYPE!($ob, unsafe { core::ptr::addr_of_mut!(PySet_Type) }) != 0
            || Py_IS_TYPE!($ob, unsafe { core::ptr::addr_of_mut!(PyFrozenSet_Type) }) != 0;
        let __v2: bool = __v
            || (unsafe { PyType_IsSubtype(Py_TYPE!($ob), core::ptr::addr_of_mut!(PySet_Type)) })
                != 0;
        __v2 || (unsafe {
            PyType_IsSubtype(Py_TYPE!($ob), core::ptr::addr_of_mut!(PyFrozenSet_Type))
        }) != 0
    }};
}

macro_rules! PyDictItems_Check {
    ($op:expr) => {
        PyObject_TypeCheck!($op, unsafe { core::ptr::addr_of_mut!(PyDictItems_Type) })
    };
}

macro_rules! PyDictKeys_Check {
    ($op:expr) => {
        PyObject_TypeCheck!($op, unsafe { core::ptr::addr_of_mut!(PyDictKeys_Type) })
    };
}

/// This excludes Values, since they are not sets.
macro_rules! PyDictViewSet_Check {
    ($op:expr) => {{
        let __v: bool = PyDictKeys_Check!($op) != 0 || PyDictItems_Check!($op) != 0;
        __v
    }};
}

macro_rules! PyDict_CheckExact__3 {
    ($op:expr) => {
        Py_IS_TYPE__2!($op, unsafe { core::ptr::addr_of_mut!(PyDict_Type) })
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

/// Mask the old API with a call to the new API for code compiled under
/// Python 2.0:
macro_rules! PyErr_BadInternalCall {
    () => {
        unsafe {
            _PyErr_BadInternalCall(
                c"/home/takashi/c-corpus/cpython/Objects/dictobject.c".as_ptr(),
                6002i32,
            )
        };
    };
}

/// Mask the old API with a call to the new API for code compiled under
/// Python 2.0:
macro_rules! PyErr_BadInternalCall__11 {
    () => {
        unsafe {
            _PyErr_BadInternalCall(
                c"/home/takashi/c-corpus/cpython/Objects/dictobject.c".as_ptr(),
                2722i32,
            )
        };
    };
}

/// Mask the old API with a call to the new API for code compiled under
/// Python 2.0:
macro_rules! PyErr_BadInternalCall__18 {
    () => {
        unsafe {
            _PyErr_BadInternalCall(
                c"/home/takashi/c-corpus/cpython/Objects/dictobject.c".as_ptr(),
                3561i32,
            )
        };
    };
}

/// Mask the old API with a call to the new API for code compiled under
/// Python 2.0:
macro_rules! PyErr_BadInternalCall__26 {
    () => {
        unsafe {
            _PyErr_BadInternalCall(
                c"/home/takashi/c-corpus/cpython/Objects/dictobject.c".as_ptr(),
                2397i32,
            )
        };
    };
}

/// Mask the old API with a call to the new API for code compiled under
/// Python 2.0:
macro_rules! PyErr_BadInternalCall__29 {
    () => {
        unsafe {
            _PyErr_BadInternalCall(
                c"/home/takashi/c-corpus/cpython/Objects/dictobject.c".as_ptr(),
                2854i32,
            )
        };
    };
}

/// Mask the old API with a call to the new API for code compiled under
/// Python 2.0:
macro_rules! PyErr_BadInternalCall__36 {
    () => {
        unsafe {
            _PyErr_BadInternalCall(
                c"/home/takashi/c-corpus/cpython/Objects/dictobject.c".as_ptr(),
                2468i32,
            )
        };
    };
}

/// Mask the old API with a call to the new API for code compiled under
/// Python 2.0:
macro_rules! PyErr_BadInternalCall__54 {
    () => {
        unsafe {
            _PyErr_BadInternalCall(
                c"/home/takashi/c-corpus/cpython/Objects/dictobject.c".as_ptr(),
                2524i32,
            )
        };
    };
}

/// Mask the old API with a call to the new API for code compiled under
/// Python 2.0:
macro_rules! PyErr_BadInternalCall__81 {
    () => {
        unsafe {
            _PyErr_BadInternalCall(
                c"/home/takashi/c-corpus/cpython/Objects/dictobject.c".as_ptr(),
                3610i32,
            )
        };
    };
}

/// Mask the old API with a call to the new API for code compiled under
/// Python 2.0:
macro_rules! PyErr_BadInternalCall__100 {
    () => {
        unsafe {
            _PyErr_BadInternalCall(
                c"/home/takashi/c-corpus/cpython/Objects/dictobject.c".as_ptr(),
                4003i32,
            )
        };
    };
}

/// Mask the old API with a call to the new API for code compiled under
/// Python 2.0:
macro_rules! PyErr_BadInternalCall__110 {
    () => {
        unsafe {
            _PyErr_BadInternalCall(
                c"/home/takashi/c-corpus/cpython/Objects/dictobject.c".as_ptr(),
                4248i32,
            )
        };
    };
}

/// Mask the old API with a call to the new API for code compiled under
/// Python 2.0:
macro_rules! PyErr_BadInternalCall__111 {
    () => {
        unsafe {
            _PyErr_BadInternalCall(
                c"/home/takashi/c-corpus/cpython/Objects/dictobject.c".as_ptr(),
                3658i32,
            )
        };
    };
}

/// Mask the old API with a call to the new API for code compiled under
/// Python 2.0:
macro_rules! PyErr_BadInternalCall__112 {
    () => {
        unsafe {
            _PyErr_BadInternalCall(
                c"/home/takashi/c-corpus/cpython/Objects/dictobject.c".as_ptr(),
                4231i32,
            )
        };
    };
}

/// Mask the old API with a call to the new API for code compiled under
/// Python 2.0:
macro_rules! PyErr_BadInternalCall__125 {
    () => {
        unsafe {
            _PyErr_BadInternalCall(
                c"/home/takashi/c-corpus/cpython/Objects/dictobject.c".as_ptr(),
                4421i32,
            )
        };
    };
}

/// Mask the old API with a call to the new API for code compiled under
/// Python 2.0:
macro_rules! PyErr_BadInternalCall__148 {
    () => {
        unsafe {
            _PyErr_BadInternalCall(
                c"/home/takashi/c-corpus/cpython/Objects/dictobject.c".as_ptr(),
                2758i32,
            )
        };
    };
}

/// Mask the old API with a call to the new API for code compiled under
/// Python 2.0:
macro_rules! PyErr_BadInternalCall__150 {
    () => {
        unsafe {
            _PyErr_BadInternalCall(
                c"/home/takashi/c-corpus/cpython/Objects/dictobject.c".as_ptr(),
                3166i32,
            )
        };
    };
}

/// Mask the old API with a call to the new API for code compiled under
/// Python 2.0:
macro_rules! PyErr_BadInternalCall__151 {
    () => {
        unsafe {
            _PyErr_BadInternalCall(
                c"/home/takashi/c-corpus/cpython/Objects/dictobject.c".as_ptr(),
                2646i32,
            )
        };
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

macro_rules! PyList_SET_ITEM {
    ($op:expr, $index:expr, $value:expr) => {
        PyList_SET_ITEM(_PyObject_CAST!($op), $index, _PyObject_CAST!($value));
    };
}

macro_rules! PyModule_Check {
    ($op:expr) => {
        PyObject_TypeCheck!($op, unsafe { core::ptr::addr_of_mut!(PyModule_Type) })
    };
}

macro_rules! PyObject_GC_New {
    ($type:ty, $typeobj:expr) => {
        _Py_CAST!(*mut $type, unsafe { _PyObject_GC_New($typeobj) })
    };
}

macro_rules! PyObject_TypeCheck {
    ($ob:expr, $type:expr) => {
        PyObject_TypeCheck(_PyObject_CAST!($ob), $type)
    };
}

/// Return the 'i'-th element of the sequence 'o', assuming that o was returned
/// by PySequence_Fast, and that i is within bounds.
macro_rules! PySequence_Fast_GET_ITEM {
    ($o:expr, $i:expr) => {{
        let __v: *mut _object = if PyList_Check!($o) != 0 {
            PyList_GET_ITEM!($o, $i)
        } else {
            PyTuple_GET_ITEM!($o, $i)
        };
        __v
    }};
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

macro_rules! PySet_CheckExact {
    ($op:expr) => {
        Py_IS_TYPE!($op, unsafe { core::ptr::addr_of_mut!(PySet_Type) })
    };
}

macro_rules! PyStackRef_CLEAR {
    ($REF:expr) => {
        loop {
            let _tmp_op_ptr: *mut _PyStackRef = unsafe { core::ptr::addr_of_mut!($REF) };
            let _tmp_old_op: _PyStackRef = unsafe { *_tmp_op_ptr };
            unsafe {
                *_tmp_op_ptr = PyStackRef_NULL;
            }
            PyStackRef_XCLOSE(_tmp_old_op);
            if true {
                break;
            }
        }
    };
}

macro_rules! PyStackRef_CheckValid {
    () => {};
}

macro_rules! PyStackRef_FromPyObjectNew {
    ($obj:expr) => {
        _PyStackRef_FromPyObjectNew(_PyObject_CAST!($obj))
    };
}

macro_rules! PyStackRef_IsNull {
    ($ref:expr) => {
        (unsafe { $ref.bits }) == 1
    };
}

macro_rules! PyStackRef_XSETREF {
    ($dst:expr, $src:expr) => {
        loop {
            let _tmp_dst_ref: _PyStackRef = unsafe { $dst };
            unsafe {
                $dst = $src;
            }
            PyStackRef_XCLOSE(_tmp_dst_ref);
            if true {
                break;
            }
        }
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

macro_rules! Py_CLEAR__27 {
    ($op:expr) => {
        loop {
            let _tmp_op_ptr: *mut *mut __SlateRecord_dictobject_h_33 =
                unsafe { core::ptr::addr_of_mut!($op) };
            let _tmp_old_op: *mut __SlateRecord_dictobject_h_33 = unsafe { *_tmp_op_ptr };
            if !_tmp_old_op.is_null() {
                unsafe {
                    *_tmp_op_ptr = core::ptr::null_mut::<__SlateRecord_dictobject_h_33>();
                }
                Py_DECREF__2!(_tmp_old_op);
            }
            if true {
                break;
            }
        }
    };
}

macro_rules! Py_DECREF_MORTAL {
    ($op:expr) => {
        Py_DECREF_MORTAL(_PyObject_CAST!($op));
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
macro_rules! Py_MAX__5 {
    ($x:expr, $y:expr) => {{
        let __v: i32 = if (($x) as i32) > (($y) as i32) {
            ($x) as i32
        } else {
            ($y) as i32
        };
        __v
    }};
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

/// Convert the argument to a string. For example, Py_STRINGIFY(123) is replaced
/// with "123" by the preprocessor. Defines are also replaced by their value.
/// For example Py_STRINGIFY(__LINE__) is replaced by the line number, not
/// by "__LINE__".
macro_rules! Py_STRINGIFY__6 {
    () => {
        c"entry->me_hash != -1".as_ptr() as *mut i8
    };
}

/// Convert the argument to a string. For example, Py_STRINGIFY(123) is replaced
/// with "123" by the preprocessor. Defines are also replaced by their value.
/// For example Py_STRINGIFY(__LINE__) is replaced by the line number, not
/// by "__LINE__".
macro_rules! Py_STRINGIFY__10 {
    () => {
        c"hash != -1".as_ptr() as *mut i8
    };
}

/// Convert the argument to a string. For example, Py_STRINGIFY(123) is replaced
/// with "123" by the preprocessor. Defines are also replaced by their value.
/// For example Py_STRINGIFY(__LINE__) is replaced by the line number, not
/// by "__LINE__".
macro_rules! Py_STRINGIFY__12 {
    () => {
        c"keys->dk_refcnt == 1 || keys == &empty_keys_struct".as_ptr() as *mut i8
    };
}

/// Convert the argument to a string. For example, Py_STRINGIFY(123) is replaced
/// with "123" by the preprocessor. Defines are also replaced by their value.
/// For example Py_STRINGIFY(__LINE__) is replaced by the line number, not
/// by "__LINE__".
macro_rules! Py_STRINGIFY {
    () => {
        c"mp->ma_used <= 30".as_ptr() as *mut i8
    };
}

/// Convert the argument to a string. For example, Py_STRINGIFY(123) is replaced
/// with "123" by the preprocessor. Defines are also replaced by their value.
/// For example Py_STRINGIFY(__LINE__) is replaced by the line number, not
/// by "__LINE__".
macro_rules! Py_STRINGIFY__15 {
    () => {
        c"mp->ma_values->values[index] != ((void*)0)".as_ptr() as *mut i8
    };
}

/// Convert the argument to a string. For example, Py_STRINGIFY(123) is replaced
/// with "123" by the preprocessor. Defines are also replaced by their value.
/// For example Py_STRINGIFY(__LINE__) is replaced by the line number, not
/// by "__LINE__".
macro_rules! Py_STRINGIFY__17 {
    () => {
        c"PyType_HasFeature((_Py_TYPE(((PyObject*)((op))))), ((1UL << 29)))".as_ptr() as *mut i8
    };
}

/// Convert the argument to a string. For example, Py_STRINGIFY(123) is replaced
/// with "123" by the preprocessor. Defines are also replaced by their value.
/// For example Py_STRINGIFY(__LINE__) is replaced by the line number, not
/// by "__LINE__".
macro_rules! Py_STRINGIFY__18 {
    () => {
        c"mp->ma_values->embedded == 1".as_ptr() as *mut i8
    };
}

/// Convert the argument to a string. For example, Py_STRINGIFY(123) is replaced
/// with "123" by the preprocessor. Defines are also replaced by their value.
/// For example Py_STRINGIFY(__LINE__) is replaced by the line number, not
/// by "__LINE__".
macro_rules! Py_STRINGIFY__24 {
    () => {
        c"entry->me_value == ((void*)0)".as_ptr() as *mut i8
    };
}

/// Convert the argument to a string. For example, Py_STRINGIFY(123) is replaced
/// with "123" by the preprocessor. Defines are also replaced by their value.
/// For example Py_STRINGIFY(__LINE__) is replaced by the line number, not
/// by "__LINE__".
macro_rules! Py_STRINGIFY__25 {
    () => {
        c"Py_IS_TYPE(((PyObject*)(((key)))), (&PyUnicode_Type))".as_ptr() as *mut i8
    };
}

/// Convert the argument to a string. For example, Py_STRINGIFY(123) is replaced
/// with "123" by the preprocessor. Defines are also replaced by their value.
/// For example Py_STRINGIFY(__LINE__) is replaced by the line number, not
/// by "__LINE__".
macro_rules! Py_STRINGIFY__3 {
    () => {
        c"entry->me_value != ((void*)0)".as_ptr() as *mut i8
    };
}

/// Convert the argument to a string. For example, Py_STRINGIFY(123) is replaced
/// with "123" by the preprocessor. Defines are also replaced by their value.
/// For example Py_STRINGIFY(__LINE__) is replaced by the line number, not
/// by "__LINE__".
macro_rules! Py_STRINGIFY__30 {
    () => {
        c"keys->dk_kind != DICT_KEYS_SPLIT".as_ptr() as *mut i8
    };
}

/// Convert the argument to a string. For example, Py_STRINGIFY(123) is replaced
/// with "123" by the preprocessor. Defines are also replaced by their value.
/// For example Py_STRINGIFY(__LINE__) is replaced by the line number, not
/// by "__LINE__".
macro_rules! Py_STRINGIFY__32 {
    () => {
        c"(-2) <= ix && ix <= usable".as_ptr() as *mut i8
    };
}

/// Convert the argument to a string. For example, Py_STRINGIFY(123) is replaced
/// with "123" by the preprocessor. Defines are also replaced by their value.
/// For example Py_STRINGIFY(__LINE__) is replaced by the line number, not
/// by "__LINE__".
macro_rules! Py_STRINGIFY__36 {
    () => {
        c"mp->ma_values->valid == 1".as_ptr() as *mut i8
    };
}

/// Convert the argument to a string. For example, Py_STRINGIFY(123) is replaced
/// with "123" by the preprocessor. Defines are also replaced by their value.
/// For example Py_STRINGIFY(__LINE__) is replaced by the line number, not
/// by "__LINE__".
macro_rules! Py_STRINGIFY__37 {
    () => {
        c"dk_usable + dk_nentries <= usable".as_ptr() as *mut i8
    };
}

/// Convert the argument to a string. For example, Py_STRINGIFY(123) is replaced
/// with "123" by the preprocessor. Defines are also replaced by their value.
/// For example Py_STRINGIFY(__LINE__) is replaced by the line number, not
/// by "__LINE__".
macro_rules! Py_STRINGIFY__39 {
    () => {
        c"entry->me_hash == hash".as_ptr() as *mut i8
    };
}

/// Convert the argument to a string. For example, Py_STRINGIFY(123) is replaced
/// with "123" by the preprocessor. Defines are also replaced by their value.
/// For example Py_STRINGIFY(__LINE__) is replaced by the line number, not
/// by "__LINE__".
macro_rules! Py_STRINGIFY__41 {
    () => {
        c"(duplicate_check & (1<<index)) == 0".as_ptr() as *mut i8
    };
}

/// Convert the argument to a string. For example, Py_STRINGIFY(123) is replaced
/// with "123" by the preprocessor. Defines are also replaced by their value.
/// For example Py_STRINGIFY(__LINE__) is replaced by the line number, not
/// by "__LINE__".
macro_rules! Py_STRINGIFY__46 {
    () => {
        c"keys->dk_kind == DICT_KEYS_SPLIT".as_ptr() as *mut i8
    };
}

/// Convert the argument to a string. For example, Py_STRINGIFY(123) is replaced
/// with "123" by the preprocessor. Defines are also replaced by their value.
/// For example Py_STRINGIFY(__LINE__) is replaced by the line number, not
/// by "__LINE__".
macro_rules! Py_STRINGIFY__48 {
    () => {
        c"0 <= mp->ma_used && mp->ma_used <= usable".as_ptr() as *mut i8
    };
}

/// Convert the argument to a string. For example, Py_STRINGIFY(123) is replaced
/// with "123" by the preprocessor. Defines are also replaced by their value.
/// For example Py_STRINGIFY(__LINE__) is replaced by the line number, not
/// by "__LINE__".
macro_rules! Py_STRINGIFY__50 {
    () => {
        c"0 <= dk_usable && dk_usable <= usable".as_ptr() as *mut i8
    };
}

/// Convert the argument to a string. For example, Py_STRINGIFY(123) is replaced
/// with "123" by the preprocessor. Defines are also replaced by their value.
/// For example Py_STRINGIFY(__LINE__) is replaced by the line number, not
/// by "__LINE__".
macro_rules! Py_STRINGIFY__51 {
    () => {
        c"0 <= dk_nentries && dk_nentries <= usable".as_ptr() as *mut i8
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
macro_rules! Py_VISIT__4 {
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

macro_rules! Py_XINCREF__2 {
    ($op:expr) => {
        Py_XINCREF(_PyObject_CAST__2!($op));
    };
}

macro_rules! Py_XNewRef__3 {
    ($obj:expr) => {
        _Py_XNewRef(_PyObject_CAST!($obj));
    };
}

macro_rules! Py_XNewRef {
    ($obj:expr) => {
        _Py_XNewRef(_PyObject_CAST!($obj))
    };
}

macro_rules! RARE_EVENT_STAT_INC {
    () => {};
}

macro_rules! STORE_HASH__2 {
    ($ep:expr, $hash:expr) => {
        FT_ATOMIC_STORE_SSIZE_RELAXED!((*$ep).me_hash, $hash);
    };
}

macro_rules! STORE_HASH {
    ($ep:expr, $hash:expr) => {
        FT_ATOMIC_STORE_SSIZE_RELAXED__2!((*$ep).me_hash, $hash);
    };
}

macro_rules! STORE_KEYS_NENTRIES {
    ($keys:expr, $nentries:expr) => {
        FT_ATOMIC_STORE_SSIZE_RELAXED!((*$keys).dk_nentries, $nentries);
    };
}

macro_rules! STORE_KEYS_USABLE {
    ($keys:expr, $usable:expr) => {
        FT_ATOMIC_STORE_SSIZE_RELAXED!((*$keys).dk_usable, $usable);
    };
}

macro_rules! STORE_KEY {
    ($ep:expr) => {
        FT_ATOMIC_STORE_PTR_RELEASE__2!((*$ep).me_key);
    };
}

macro_rules! STORE_KEY__2 {
    ($ep:expr, $key:expr) => {
        FT_ATOMIC_STORE_PTR_RELEASE!((*$ep).me_key, $key);
    };
}

macro_rules! STORE_SHARED_KEY {
    ($key:expr, $value:expr) => {
        unsafe {
            $key = $value;
        }
    };
}

macro_rules! STORE_SPLIT_VALUE {
    ($mp:expr, $idx:expr, $value:expr) => {
        FT_ATOMIC_STORE_PTR_RELEASE!(
            *unsafe { (*(*$mp).ma_values).values.as_mut_ptr() as *mut *mut _object }
                .wrapping_offset(($idx) as isize),
            $value
        );
    };
}

macro_rules! STORE_SPLIT_VALUE__2 {
    ($mp:expr, $idx:expr) => {
        FT_ATOMIC_STORE_PTR_RELEASE__2!(
            *unsafe { (*(*$mp).ma_values).values.as_mut_ptr() as *mut *mut _object }
                .wrapping_offset(($idx) as isize)
        );
    };
}

macro_rules! STORE_USED {
    ($mp:expr, $used:expr) => {
        FT_ATOMIC_STORE_SSIZE_RELAXED!((*$mp).ma_used, $used);
    };
}

macro_rules! STORE_USED__2 {
    ($mp:expr, $used:expr) => {
        FT_ATOMIC_STORE_SSIZE_RELAXED__2!((*$mp).ma_used, $used);
    };
}

macro_rules! STORE_VALUE__2 {
    ($ep:expr) => {
        FT_ATOMIC_STORE_PTR_RELEASE__2!((*$ep).me_value);
    };
}

macro_rules! STORE_VALUE {
    ($ep:expr, $value:expr) => {
        FT_ATOMIC_STORE_PTR_RELEASE!((*$ep).me_value, $value);
    };
}

/// USABLE_FRACTION is the maximum dictionary load.
/// Increasing this ratio makes dictionaries more dense resulting in more
/// collisions.  Decreasing it improves sparseness at the expense of spreading
/// indices over more cache lines and at the cost of total memory consumed.
///
/// USABLE_FRACTION must obey the following:
///     (0 < USABLE_FRACTION(n) < n) for all n >= 2
///
/// USABLE_FRACTION should be quick to calculate.
/// Fractions around 1/2 to 2/3 seem to work well in practice.
macro_rules! USABLE_FRACTION {
    ($n:expr) => {
        (($n) << 1i32) / 3
    };
}

/// USABLE_FRACTION is the maximum dictionary load.
/// Increasing this ratio makes dictionaries more dense resulting in more
/// collisions.  Decreasing it improves sparseness at the expense of spreading
/// indices over more cache lines and at the cost of total memory consumed.
///
/// USABLE_FRACTION must obey the following:
///     (0 < USABLE_FRACTION(n) < n) for all n >= 2
///
/// USABLE_FRACTION should be quick to calculate.
/// Fractions around 1/2 to 2/3 seem to work well in practice.
macro_rules! USABLE_FRACTION__2 {
    ($n:expr) => {
        (($n) << 1i32) / 3
    };
}

/// USABLE_FRACTION is the maximum dictionary load.
/// Increasing this ratio makes dictionaries more dense resulting in more
/// collisions.  Decreasing it improves sparseness at the expense of spreading
/// indices over more cache lines and at the cost of total memory consumed.
///
/// USABLE_FRACTION must obey the following:
///     (0 < USABLE_FRACTION(n) < n) for all n >= 2
///
/// USABLE_FRACTION should be quick to calculate.
/// Fractions around 1/2 to 2/3 seem to work well in practice.
macro_rules! USABLE_FRACTION__3 {
    ($n:expr) => {
        (($n) << 1i32) / 3
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

macro_rules! _PyCompactUnicodeObject_CAST {
    ($op:expr) => {
        _Py_CAST!(*mut __SlateRecord_unicodeobject_h_173, $op)
    };
}

macro_rules! _PyDict_HasSplitTable {
    ($d:expr) => {
        !unsafe { (*$d).ma_values }.is_null()
    };
}

/// Cast argument to PyListObject* type.
macro_rules! _PyList_CAST {
    ($op:expr) => {
        _Py_CAST!(*mut __SlateRecord_listobject_h_22, $op)
    };
}

macro_rules! _PyObject_ASSERT_FAILED_MSG {
    ($obj:expr, $msg:expr) => {
        unsafe {
            _PyObject_AssertFailed(
                $obj,
                core::ptr::null::<i8>(),
                ($msg) as *const i8,
                c"/home/takashi/c-corpus/cpython/Objects/dictobject.c".as_ptr(),
                737i32,
                c"_PyDict_CheckConsistency".as_ptr(),
            )
        };
        unsafe { core::hint::unreachable_unchecked() };
    };
}

macro_rules! _PyObject_ASSERT_FAILED_MSG__4 {
    ($obj:expr, $msg:expr) => {
        unsafe {
            _PyObject_AssertFailed(
                $obj,
                core::ptr::null::<i8>(),
                ($msg) as *const i8,
                c"/home/takashi/c-corpus/cpython/Objects/dictobject.c".as_ptr(),
                659i32,
                c"_PyDict_CheckConsistency".as_ptr(),
            )
        };
        unsafe { core::hint::unreachable_unchecked() };
    };
}

macro_rules! _PyObject_ASSERT_FAILED_MSG__7 {
    ($obj:expr, $msg:expr) => {
        unsafe {
            _PyObject_AssertFailed(
                $obj,
                core::ptr::null::<i8>(),
                ($msg) as *const i8,
                c"/home/takashi/c-corpus/cpython/Objects/dictobject.c".as_ptr(),
                672i32,
                c"_PyDict_CheckConsistency".as_ptr(),
            )
        };
        unsafe { core::hint::unreachable_unchecked() };
    };
}

macro_rules! _PyObject_ASSERT_FAILED_MSG__9 {
    ($obj:expr, $msg:expr) => {
        unsafe {
            _PyObject_AssertFailed(
                $obj,
                core::ptr::null::<i8>(),
                ($msg) as *const i8,
                c"/home/takashi/c-corpus/cpython/Objects/dictobject.c".as_ptr(),
                683i32,
                c"_PyDict_CheckConsistency".as_ptr(),
            )
        };
        unsafe { core::hint::unreachable_unchecked() };
    };
}

macro_rules! _PyObject_ASSERT_FAILED_MSG__10 {
    ($obj:expr, $msg:expr) => {
        unsafe {
            _PyObject_AssertFailed(
                $obj,
                core::ptr::null::<i8>(),
                ($msg) as *const i8,
                c"/home/takashi/c-corpus/cpython/Objects/dictobject.c".as_ptr(),
                673i32,
                c"_PyDict_CheckConsistency".as_ptr(),
            )
        };
        unsafe { core::hint::unreachable_unchecked() };
    };
}

macro_rules! _PyObject_ASSERT_FAILED_MSG__12 {
    ($obj:expr, $msg:expr) => {
        unsafe {
            _PyObject_AssertFailed(
                $obj,
                core::ptr::null::<i8>(),
                ($msg) as *const i8,
                c"/home/takashi/c-corpus/cpython/Objects/dictobject.c".as_ptr(),
                742i32,
                c"_PyDict_CheckConsistency".as_ptr(),
            )
        };
        unsafe { core::hint::unreachable_unchecked() };
    };
}

macro_rules! _PyObject_ASSERT_FAILED_MSG__15 {
    ($obj:expr, $msg:expr) => {
        unsafe {
            _PyObject_AssertFailed(
                $obj,
                core::ptr::null::<i8>(),
                ($msg) as *const i8,
                c"/home/takashi/c-corpus/cpython/Objects/dictobject.c".as_ptr(),
                679i32,
                c"_PyDict_CheckConsistency".as_ptr(),
            )
        };
        unsafe { core::hint::unreachable_unchecked() };
    };
}

macro_rules! _PyObject_ASSERT_FAILED_MSG__19 {
    ($obj:expr, $msg:expr) => {
        unsafe {
            _PyObject_AssertFailed(
                $obj,
                core::ptr::null::<i8>(),
                ($msg) as *const i8,
                c"/home/takashi/c-corpus/cpython/Objects/dictobject.c".as_ptr(),
                671i32,
                c"_PyDict_CheckConsistency".as_ptr(),
            )
        };
        unsafe { core::hint::unreachable_unchecked() };
    };
}

macro_rules! _PyObject_ASSERT_FAILED_MSG__20 {
    ($obj:expr, $msg:expr) => {
        unsafe {
            _PyObject_AssertFailed(
                $obj,
                core::ptr::null::<i8>(),
                ($msg) as *const i8,
                c"/home/takashi/c-corpus/cpython/Objects/dictobject.c".as_ptr(),
                678i32,
                c"_PyDict_CheckConsistency".as_ptr(),
            )
        };
        unsafe { core::hint::unreachable_unchecked() };
    };
}

macro_rules! _PyObject_ASSERT_FAILED_MSG__21 {
    ($obj:expr, $msg:expr) => {
        unsafe {
            _PyObject_AssertFailed(
                $obj,
                core::ptr::null::<i8>(),
                ($msg) as *const i8,
                c"/home/takashi/c-corpus/cpython/Objects/dictobject.c".as_ptr(),
                685i32,
                c"_PyDict_CheckConsistency".as_ptr(),
            )
        };
        unsafe { core::hint::unreachable_unchecked() };
    };
}

macro_rules! _PyObject_ASSERT_FAILED_MSG__25 {
    ($obj:expr, $msg:expr) => {
        unsafe {
            _PyObject_AssertFailed(
                $obj,
                core::ptr::null::<i8>(),
                ($msg) as *const i8,
                c"/home/takashi/c-corpus/cpython/Objects/dictobject.c".as_ptr(),
                722i32,
                c"_PyDict_CheckConsistency".as_ptr(),
            )
        };
        unsafe { core::hint::unreachable_unchecked() };
    };
}

macro_rules! _PyObject_ASSERT_FAILED_MSG__29 {
    ($obj:expr, $msg:expr) => {
        unsafe {
            _PyObject_AssertFailed(
                $obj,
                core::ptr::null::<i8>(),
                ($msg) as *const i8,
                c"/home/takashi/c-corpus/cpython/Objects/dictobject.c".as_ptr(),
                674i32,
                c"_PyDict_CheckConsistency".as_ptr(),
            )
        };
        unsafe { core::hint::unreachable_unchecked() };
    };
}

macro_rules! _PyObject_ASSERT_FAILED_MSG__30 {
    ($obj:expr, $msg:expr) => {
        unsafe {
            _PyObject_AssertFailed(
                $obj,
                core::ptr::null::<i8>(),
                ($msg) as *const i8,
                c"/home/takashi/c-corpus/cpython/Objects/dictobject.c".as_ptr(),
                731i32,
                c"_PyDict_CheckConsistency".as_ptr(),
            )
        };
        unsafe { core::hint::unreachable_unchecked() };
    };
}

macro_rules! _PyObject_ASSERT_FAILED_MSG__35 {
    ($obj:expr, $msg:expr) => {
        unsafe {
            _PyObject_AssertFailed(
                $obj,
                core::ptr::null::<i8>(),
                ($msg) as *const i8,
                c"/home/takashi/c-corpus/cpython/Objects/dictobject.c".as_ptr(),
                710i32,
                c"_PyDict_CheckConsistency".as_ptr(),
            )
        };
        unsafe { core::hint::unreachable_unchecked() };
    };
}

macro_rules! _PyObject_ASSERT_FAILED_MSG__37 {
    ($obj:expr, $msg:expr) => {
        unsafe {
            _PyObject_AssertFailed(
                $obj,
                core::ptr::null::<i8>(),
                ($msg) as *const i8,
                c"/home/takashi/c-corpus/cpython/Objects/dictobject.c".as_ptr(),
                724i32,
                c"_PyDict_CheckConsistency".as_ptr(),
            )
        };
        unsafe { core::hint::unreachable_unchecked() };
    };
}

macro_rules! _PyObject_ASSERT_FAILED_MSG__39 {
    ($obj:expr, $msg:expr) => {
        unsafe {
            _PyObject_AssertFailed(
                $obj,
                core::ptr::null::<i8>(),
                ($msg) as *const i8,
                c"/home/takashi/c-corpus/cpython/Objects/dictobject.c".as_ptr(),
                726i32,
                c"_PyDict_CheckConsistency".as_ptr(),
            )
        };
        unsafe { core::hint::unreachable_unchecked() };
    };
}

macro_rules! _PyObject_ASSERT_FAILED_MSG__42 {
    ($obj:expr, $msg:expr) => {
        unsafe {
            _PyObject_AssertFailed(
                $obj,
                core::ptr::null::<i8>(),
                ($msg) as *const i8,
                c"/home/takashi/c-corpus/cpython/Objects/dictobject.c".as_ptr(),
                705i32,
                c"_PyDict_CheckConsistency".as_ptr(),
            )
        };
        unsafe { core::hint::unreachable_unchecked() };
    };
}

macro_rules! _PyObject_ASSERT_FAILED_MSG__44 {
    ($obj:expr, $msg:expr) => {
        unsafe {
            _PyObject_AssertFailed(
                $obj,
                core::ptr::null::<i8>(),
                ($msg) as *const i8,
                c"/home/takashi/c-corpus/cpython/Objects/dictobject.c".as_ptr(),
                694i32,
                c"_PyDict_CheckConsistency".as_ptr(),
            )
        };
        unsafe { core::hint::unreachable_unchecked() };
    };
}

macro_rules! _PyObject_ASSERT_FAILED_MSG__45 {
    ($obj:expr, $msg:expr) => {
        unsafe {
            _PyObject_AssertFailed(
                $obj,
                core::ptr::null::<i8>(),
                ($msg) as *const i8,
                c"/home/takashi/c-corpus/cpython/Objects/dictobject.c".as_ptr(),
                706i32,
                c"_PyDict_CheckConsistency".as_ptr(),
            )
        };
        unsafe { core::hint::unreachable_unchecked() };
    };
}

macro_rules! _PyObject_ASSERT_FAILED_MSG__50 {
    ($obj:expr, $msg:expr) => {
        unsafe {
            _PyObject_AssertFailed(
                $obj,
                core::ptr::null::<i8>(),
                ($msg) as *const i8,
                c"/home/takashi/c-corpus/cpython/Objects/dictobject.c".as_ptr(),
                686i32,
                c"_PyDict_CheckConsistency".as_ptr(),
            )
        };
        unsafe { core::hint::unreachable_unchecked() };
    };
}

macro_rules! _PyObject_ASSERT_FAILED_MSG__51 {
    ($obj:expr, $msg:expr) => {
        unsafe {
            _PyObject_AssertFailed(
                $obj,
                core::ptr::null::<i8>(),
                ($msg) as *const i8,
                c"/home/takashi/c-corpus/cpython/Objects/dictobject.c".as_ptr(),
                682i32,
                c"_PyDict_CheckConsistency".as_ptr(),
            )
        };
        unsafe { core::hint::unreachable_unchecked() };
    };
}

macro_rules! _PyObject_ASSERT_FAILED_MSG__54 {
    ($obj:expr, $msg:expr) => {
        unsafe {
            _PyObject_AssertFailed(
                $obj,
                core::ptr::null::<i8>(),
                ($msg) as *const i8,
                c"/home/takashi/c-corpus/cpython/Objects/dictobject.c".as_ptr(),
                744i32,
                c"_PyDict_CheckConsistency".as_ptr(),
            )
        };
        unsafe { core::hint::unreachable_unchecked() };
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

macro_rules! _PyObject_GC_IS_TRACKED {
    ($op:expr) => {
        _PyObject_GC_IS_TRACKED(_Py_CAST__2!($op))
    };
}

macro_rules! _PyObject_GC_IS_TRACKED__2 {
    ($op:expr) => {
        _PyObject_GC_IS_TRACKED(_Py_CAST!(*mut _object, $op))
    };
}

macro_rules! _PyObject_GC_TRACK__2 {
    ($op:expr) => {
        _PyObject_GC_TRACK(_PyObject_CAST!($op));
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

/// Pops a non-PyObject data structure from the freelist, returns NULL if the
///
/// freelist is empty.
macro_rules! _Py_FREELIST_POP_MEM {
    ($NAME:ident) => {
        _PyFreeList_PopMem(unsafe { core::ptr::addr_of_mut!((*_Py_freelists_GET()).$NAME) })
    };
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

/// Round up size "n" to be a multiple of "a".
macro_rules! _Py_SIZE_ROUND_UP__2 {
    ($n:expr, $a:expr) => {
        (($n) as u64).wrapping_add($a.wrapping_sub(1)) & !$a.wrapping_sub(1)
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
    static mut _Py_NotImplementedStruct: _object;
    static mut PyUnicode_Type: _typeobject;
    static mut PyExc_AttributeError: *mut _object;
    static mut PyExc_KeyError: *mut _object;
    static mut PyExc_RuntimeError: *mut _object;
    static mut PyExc_TypeError: *mut _object;
    static mut PyExc_ValueError: *mut _object;
    static mut _Py_FalseStruct: _longobject;
    static mut _Py_TrueStruct: _longobject;
    static mut PySet_Type: _typeobject;
    static mut PyFrozenSet_Type: _typeobject;
    static mut PyModule_Type: _typeobject;
    #[thread_local]
    static mut _Py_tss_tstate: *mut _ts;
    static mut _PyRuntime: pyruntimestate;
    fn index(arg0: *const i8, arg1: i32) -> *mut i8;
    fn PyMem_Malloc(arg0: u64) -> *mut ();
    fn PyMem_Free(arg0: *mut ());
    fn PyType_IsSubtype(arg0: *mut _typeobject, arg1: *mut _typeobject) -> i32;
    fn PyObject_RichCompareBool(arg0: *mut _object, arg1: *mut _object, arg2: i32) -> i32;
    fn PyObject_HasAttrWithError(arg0: *mut _object, arg1: *mut _object) -> i32;
    fn PyObject_SelfIter(arg0: *mut _object) -> *mut _object;
    fn PyObject_GenericGetAttr(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn PyObject_Hash(arg0: *mut _object) -> i64;
    fn PyObject_HashNotImplemented(arg0: *mut _object) -> i64;
    fn Py_ReprEnter(arg0: *mut _object) -> i32;
    fn Py_ReprLeave(arg0: *mut _object);
    fn _Py_NewReference(arg0: *mut _object);
    fn _PyObject_AssertFailed(
        arg0: *mut _object,
        arg1: *const i8,
        arg2: *const i8,
        arg3: *const i8,
        arg4: i32,
        arg5: *const i8,
    );
    fn _Py_Dealloc(arg0: *mut _object);
    fn _PyObject_GC_New(arg0: *mut _typeobject) -> *mut _object;
    fn PyObject_GC_UnTrack(arg0: *mut ());
    fn PyObject_GC_Del(arg0: *mut ());
    fn PyUnicode_FromString(arg0: *const i8) -> *mut _object;
    fn PyUnicode_FromFormat(arg0: *const i8, ...) -> *mut _object;
    fn PyUnicodeWriter_Create(arg0: i64) -> *mut PyUnicodeWriter;
    fn PyUnicodeWriter_Discard(arg0: *mut PyUnicodeWriter);
    fn PyUnicodeWriter_Finish(arg0: *mut PyUnicodeWriter) -> *mut _object;
    fn PyUnicodeWriter_WriteChar(arg0: *mut PyUnicodeWriter, arg1: u32) -> i32;
    fn PyUnicodeWriter_WriteRepr(arg0: *mut PyUnicodeWriter, arg1: *mut _object) -> i32;
    fn _PyUnicode_FromId(arg0: *mut _Py_Identifier) -> *mut _object;
    fn PyErr_SetString(arg0: *mut _object, arg1: *const i8);
    fn PyErr_Occurred() -> *mut _object;
    fn PyErr_Clear();
    fn PyErr_GetRaisedException() -> *mut _object;
    fn PyErr_SetRaisedException(arg0: *mut _object);
    fn PyErr_GivenExceptionMatches(arg0: *mut _object, arg1: *mut _object) -> i32;
    fn PyErr_ExceptionMatches(arg0: *mut _object) -> i32;
    fn PyErr_NoMemory() -> *mut _object;
    fn PyErr_Format(arg0: *mut _object, arg1: *const i8, ...) -> *mut _object;
    fn _PyErr_BadInternalCall(arg0: *const i8, arg1: i32);
    fn PyErr_FormatUnraisable(arg0: *const i8, ...);
    fn PyLong_FromSize_t(arg0: u64) -> *mut _object;
    fn PyLong_FromSsize_t(arg0: i64) -> *mut _object;
    fn PyTuple_New(arg0: i64) -> *mut _object;
    fn PyTuple_Pack(arg0: i64, ...) -> *mut _object;
    fn PyList_New(arg0: i64) -> *mut _object;
    fn PySet_New(arg0: *mut _object) -> *mut _object;
    fn PySet_Add(arg0: *mut _object, arg1: *mut _object) -> i32;
    fn PyInterpreterState_Get() -> *mut _is;
    fn PyDictProxy_New(arg0: *mut _object) -> *mut _object;
    fn Py_GenericAlias(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn PyArg_ValidateKeywordArguments(arg0: *mut _object) -> i32;
    fn PyArg_UnpackTuple(arg0: *mut _object, arg1: *const i8, arg2: i64, arg3: i64, ...) -> i32;
    fn Py_BuildValue(arg0: *const i8, ...) -> *mut _object;
    fn PyEval_GetBuiltins() -> *mut _object;
    fn PyObject_CallMethodObjArgs(arg0: *mut _object, arg1: *mut _object, ...) -> *mut _object;
    fn PyObject_VectorcallMethod(
        arg0: *mut _object,
        arg1: *const *mut _object,
        arg2: u64,
        arg3: *mut _object,
    ) -> *mut _object;
    fn PyObject_Size(arg0: *mut _object) -> i64;
    fn PyObject_GetItem(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn PyObject_SetItem(arg0: *mut _object, arg1: *mut _object, arg2: *mut _object) -> i32;
    fn PyObject_GetIter(arg0: *mut _object) -> *mut _object;
    fn PyIter_Next(arg0: *mut _object) -> *mut _object;
    fn PySequence_List(arg0: *mut _object) -> *mut _object;
    fn PySequence_Fast(arg0: *mut _object, arg1: *const i8) -> *mut _object;
    fn PySequence_Contains(arg0: *mut _object, arg1: *mut _object) -> i32;
    fn PyMapping_Keys(arg0: *mut _object) -> *mut _object;
    fn PyObject_CallOneArg(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
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
    fn _PyErr_SetKeyError(arg0: *mut _object);
    fn _PyErr_GetRaisedException(arg0: *mut _ts) -> *mut _object;
    fn _PyErr_SetRaisedException(arg0: *mut _ts, arg1: *mut _object);
    fn _PyErr_FormatNote(arg0: *const i8, ...);
    fn _PyEval_GetBuiltin(arg0: *mut _object) -> *mut _object;
    fn _PyDebugAllocatorStats(
        arg0: *mut slate_support::FILE,
        arg1: *const i8,
        arg2: i32,
        arg3: u64,
    );
    fn _PyType_AllocNoTrack(arg0: *mut _typeobject, arg1: i64) -> *mut _object;
    fn _PyObject_ComputedDictPointer(arg0: *mut _object) -> *mut *mut _object;
    fn _PyObject_LookupSpecial(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn _PySet_NextEntryRef(
        arg0: *mut _object,
        arg1: *mut i64,
        arg2: *mut *mut _object,
        arg3: *mut i64,
    ) -> i32;
    fn _PySet_Update(arg0: *mut _object, arg1: *mut _object) -> i32;
    fn _PyUnicode_InternImmortal(arg0: *mut _is, arg1: *mut *mut _object);
    fn _PyArg_CheckPositional(arg0: *const i8, arg1: i64, arg2: i64, arg3: i64) -> i32;
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

fn PyObject_TypeCheck(ob: *mut _object, r#type: *mut _typeobject) -> i32 {
    let __v0: bool =
        Py_IS_TYPE!(ob, r#type) != 0 || (unsafe { PyType_IsSubtype(Py_TYPE!(ob), r#type) }) != 0;
    __v0 as i32
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _Py_Identifier {
    string: *const i8,
    index: i64,
    mutex: __SlateRecord_object_h_48,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _frame {}

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

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_object_h_48 {
    v: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _specialization_cache {
    getitem: *mut _object,
    getitem_version: u32,
    init: *mut _object,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _heaptypeobject {
    ht_type: _typeobject,
    as_async: __SlateRecord_object_h_135,
    as_number: __SlateRecord_object_h_106,
    as_mapping: __SlateRecord_object_h_126,
    as_sequence: __SlateRecord_object_h_120,
    as_buffer: __SlateRecord_object_h_140,
    ht_name: *mut _object,
    ht_slots: *mut _object,
    ht_qualname: *mut _object,
    ht_cached_keys: *mut _dictkeysobject,
    ht_module: *mut _object,
    _ht_tpname: *mut i8,
    ht_token: *mut (),
    _spec_cache: _specialization_cache,
}

// Track types initialized using _PyStaticType_InitBuiltin().
// The values array is placed inline directly after the rest of
// the object. Implies Py_TPFLAGS_HAVE_GC.
// Placement of dict (and values) pointers are managed by the VM, not by the type.
// The VM will automatically set tp_dictoffset. Implies Py_TPFLAGS_HAVE_GC.
// Set if instances of the type object are treated as mappings for pattern matching
// Set if the type object is dynamically allocated
// Set if the type allows subclassing
// Objects support garbage collection (see objimpl.h)
// This undocumented flag gives certain built-ins their unique pattern-matching
//
// behavior, which allows a single positional subpattern to match against the
//
// subject itself (rather than a mapped attribute on it):
// Rich comparison opcodes
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

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_unicodeobject_h_163_2 {
    __slate_bits_0: __slate_bits::__SlateBits152U0,
}

fn _Py_XNewRef(obj: *mut _object) -> *mut _object {
    Py_XINCREF!(obj);
    obj
}

fn _PyObject_SIZE(r#type: *mut _typeobject) -> u64 {
    _Py_STATIC_CAST!(u64, unsafe { (*r#type).tp_basicsize })
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

fn PyList_SET_ITEM(op: *mut _object, index: i64, value: *mut _object) {
    let list: *mut __SlateRecord_listobject_h_22 = _PyList_CAST!(op);
    unsafe {
        *unsafe { (*list).ob_item }.wrapping_offset(index as isize) = value;
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pystate_h_78 {
    __slate_bits_0: __slate_bits::__SlateBits214U0,
}

#[unsafe(no_mangle)]
static mut PyDict_Type: _typeobject = {
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
    __t0.tp_name = (b"dict\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_dictobject_h_33>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(dict_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = Some(dict_repr);
    __t0.tp_as_number = unsafe { core::ptr::addr_of_mut!(dict_as_number) };
    __t0.tp_as_sequence = unsafe { core::ptr::addr_of_mut!(dict_as_sequence) };
    __t0.tp_as_mapping = unsafe { core::ptr::addr_of_mut!(dict_as_mapping) };
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
    __t0.tp_flags = 541082688u64;
    __t0.tp_doc = unsafe { core::ptr::addr_of!(dictionary_doc.0) as *const i8 };
    __t0.tp_traverse = Some(dict_traverse);
    __t0.tp_clear = Some(dict_tp_clear);
    __t0.tp_richcompare = Some(dict_richcompare);
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = Some(dict_iter);
    __t0.tp_iternext = None;
    __t0.tp_methods = unsafe { core::ptr::addr_of_mut!(mapp_methods.0) as *mut PyMethodDef };
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = core::ptr::null_mut::<_typeobject>();
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = 0i64;
    __t0.tp_init = Some(dict_init);
    __t0.tp_alloc = unsafe {
        core::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut _typeobject, i64) -> *mut _object>,
        >(_PyType_AllocNoTrack as *const ())
    };
    __t0.tp_new = Some(dict_new);
    __t0.tp_free = unsafe {
        core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
            PyObject_GC_Del as *const (),
        )
    };
    __t0.tp_version_tag = 8u32;
    __t0.tp_vectorcall = Some(dict_vectorcall);
    __t0
};

#[unsafe(no_mangle)]
static mut PyDictKeys_Type: _typeobject = {
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
    __t0.tp_name = (b"dict_keys\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pycore_dict_h_73>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(dictview_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = Some(dictview_repr);
    __t0.tp_as_number = unsafe { core::ptr::addr_of_mut!(dictviews_as_number) };
    __t0.tp_as_sequence = unsafe { core::ptr::addr_of_mut!(dictkeys_as_sequence) };
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
    __t0.tp_traverse = Some(dictview_traverse);
    __t0.tp_clear = None;
    __t0.tp_richcompare = Some(dictview_richcompare);
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = Some(dictkeys_iter);
    __t0.tp_iternext = None;
    __t0.tp_methods = unsafe { core::ptr::addr_of_mut!(dictkeys_methods.0) as *mut PyMethodDef };
    __t0.tp_getset = unsafe { core::ptr::addr_of_mut!(dictview_getset.0) as *mut PyGetSetDef };
    __t0
};

#[unsafe(no_mangle)]
static mut PyDictValues_Type: _typeobject = {
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
    __t0.tp_name = (b"dict_values\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pycore_dict_h_73>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(dictview_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = Some(dictview_repr);
    __t0.tp_as_number = core::ptr::null_mut::<__SlateRecord_object_h_106>();
    __t0.tp_as_sequence = unsafe { core::ptr::addr_of_mut!(dictvalues_as_sequence) };
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
    __t0.tp_traverse = Some(dictview_traverse);
    __t0.tp_clear = None;
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = Some(dictvalues_iter);
    __t0.tp_iternext = None;
    __t0.tp_methods = unsafe { core::ptr::addr_of_mut!(dictvalues_methods.0) as *mut PyMethodDef };
    __t0.tp_getset = unsafe { core::ptr::addr_of_mut!(dictview_getset.0) as *mut PyGetSetDef };
    __t0
};

#[unsafe(no_mangle)]
static mut PyDictItems_Type: _typeobject = {
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
    __t0.tp_name = (b"dict_items\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_pycore_dict_h_73>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(dictview_dealloc);
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = Some(dictview_repr);
    __t0.tp_as_number = unsafe { core::ptr::addr_of_mut!(dictviews_as_number) };
    __t0.tp_as_sequence = unsafe { core::ptr::addr_of_mut!(dictitems_as_sequence) };
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
    __t0.tp_traverse = Some(dictview_traverse);
    __t0.tp_clear = None;
    __t0.tp_richcompare = Some(dictview_richcompare);
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = Some(dictitems_iter);
    __t0.tp_iternext = None;
    __t0.tp_methods = unsafe { core::ptr::addr_of_mut!(dictitems_methods.0) as *mut PyMethodDef };
    __t0.tp_getset = unsafe { core::ptr::addr_of_mut!(dictview_getset.0) as *mut PyGetSetDef };
    __t0
};

#[unsafe(no_mangle)]
static mut PyDictIterKey_Type: _typeobject = {
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
    __t0.tp_name = (b"dict_keyiterator\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_dictobject_c_5130>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(dictiter_dealloc);
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
    __t0.tp_traverse = Some(dictiter_traverse);
    __t0.tp_clear = None;
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = unsafe {
        core::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
        >(PyObject_SelfIter as *const ())
    };
    __t0.tp_iternext = Some(dictiter_iternextkey);
    __t0.tp_methods = unsafe { core::ptr::addr_of_mut!(dictiter_methods.0) as *mut PyMethodDef };
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0
};

#[unsafe(no_mangle)]
static mut PyDictIterValue_Type: _typeobject = {
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
    __t0.tp_name = (b"dict_valueiterator\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_dictobject_c_5130>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(dictiter_dealloc);
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
    __t0.tp_traverse = Some(dictiter_traverse);
    __t0.tp_clear = None;
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = unsafe {
        core::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
        >(PyObject_SelfIter as *const ())
    };
    __t0.tp_iternext = Some(dictiter_iternextvalue);
    __t0.tp_methods = unsafe { core::ptr::addr_of_mut!(dictiter_methods.0) as *mut PyMethodDef };
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0
};

#[unsafe(no_mangle)]
static mut PyDictIterItem_Type: _typeobject = {
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
    __t0.tp_name = (b"dict_itemiterator\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_dictobject_c_5130>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(dictiter_dealloc);
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
    __t0.tp_traverse = Some(dictiter_traverse);
    __t0.tp_clear = None;
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = unsafe {
        core::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
        >(PyObject_SelfIter as *const ())
    };
    __t0.tp_iternext = Some(dictiter_iternextitem);
    __t0.tp_methods = unsafe { core::ptr::addr_of_mut!(dictiter_methods.0) as *mut PyMethodDef };
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0
};

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_structs_h_32_2 {
    code: u8,
    arg: u8,
}

#[unsafe(no_mangle)]
static mut PyDictRevIterKey_Type: _typeobject = {
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
    __t0.tp_name = (b"dict_reversekeyiterator\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_dictobject_c_5130>() as u64) as i64;
    __t0.tp_dealloc = Some(dictiter_dealloc);
    __t0.tp_flags = 16384u64;
    __t0.tp_traverse = Some(dictiter_traverse);
    __t0.tp_iter = unsafe {
        core::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
        >(PyObject_SelfIter as *const ())
    };
    __t0.tp_iternext = Some(dictreviter_iternext);
    __t0.tp_methods = unsafe { core::ptr::addr_of_mut!(dictiter_methods.0) as *mut PyMethodDef };
    __t0
};

#[unsafe(no_mangle)]
static mut PyDictRevIterItem_Type: _typeobject = {
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
    __t0.tp_name = (b"dict_reverseitemiterator\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_dictobject_c_5130>() as u64) as i64;
    __t0.tp_dealloc = Some(dictiter_dealloc);
    __t0.tp_flags = 16384u64;
    __t0.tp_traverse = Some(dictiter_traverse);
    __t0.tp_iter = unsafe {
        core::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
        >(PyObject_SelfIter as *const ())
    };
    __t0.tp_iternext = Some(dictreviter_iternext);
    __t0.tp_methods = unsafe { core::ptr::addr_of_mut!(dictiter_methods.0) as *mut PyMethodDef };
    __t0
};

#[unsafe(no_mangle)]
static mut PyDictRevIterValue_Type: _typeobject = {
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
    __t0.tp_name = (b"dict_reversevalueiterator\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_dictobject_c_5130>() as u64) as i64;
    __t0.tp_dealloc = Some(dictiter_dealloc);
    __t0.tp_flags = 16384u64;
    __t0.tp_traverse = Some(dictiter_traverse);
    __t0.tp_iter = unsafe {
        core::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
        >(PyObject_SelfIter as *const ())
    };
    __t0.tp_iternext = Some(dictreviter_iternext);
    __t0.tp_methods = unsafe { core::ptr::addr_of_mut!(dictiter_methods.0) as *mut PyMethodDef };
    __t0
};

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
struct code_arena_st {}

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

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_pythread_h_64 {
    ptr: *mut __SlateRecord_pthreadtypes_h_45,
    val: __SlateRecord_pthreadtypes_h_45,
}

fn PyObject_CallMethodNoArgs(mut self_: *mut _object, name: *mut _object) -> *mut _object {
    let nargsf: u64 = 1 | _Py_STATIC_CAST!(u64, 1i32)
        << 8u64
            .wrapping_mul(core::mem::size_of::<u64>() as u64)
            .wrapping_sub(1);
    unsafe {
        PyObject_VectorcallMethod(
            name,
            core::ptr::addr_of_mut!(self_) as *const *mut _object,
            nargsf,
            core::ptr::null_mut::<_object>(),
        )
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_interp_structs_h_297 {
    mutex: PyMutex,
    hashtable: *mut _Py_hashtable_t,
}

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
struct __SlateRecord_pycore_interp_structs_h_348 {
    import_level: i32,
    accumulated: i64,
    header: i32,
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

fn _PyObject_XDecRefDelayed(obj: *mut _object) {
    Py_XDECREF!(obj);
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
struct xi_exceptions {
    PyExc_InterpreterError: *mut _object,
    PyExc_InterpreterNotFoundError: *mut _object,
    PyExc_NotShareableError: *mut _object,
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
struct _Py_slist_item_s {
    next: *mut _Py_slist_item_s,
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
union __SlateRecord_pycore_obmalloc_h_255 {
    _padding: *mut u8,
    count: u32,
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
struct __SlateRecord_pycore_runtime_structs_h_49 {
    fd: i32,
    st_dev: u64,
    st_ino: u64,
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

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_tracemalloc_h_70 {
    mem: __SlateRecord_pymem_h_48,
    raw: __SlateRecord_pymem_h_48,
    obj: __SlateRecord_pymem_h_48,
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
union __SlateRecord_sigaction_h_31 {
    sa_handler: Option<unsafe extern "C-unwind" fn(i32)>,
    sa_sigaction:
        Option<unsafe extern "C-unwind" fn(i32, *mut __SlateRecord_siginfo_t_h_124, *mut ())>,
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
struct __SlateRecord_pycore_instruments_h_94 {
    bytes_per_entry: u8,
    data: [u8; 1],
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

fn _PyObject_GC_IS_TRACKED(op: *mut _object) -> i32 {
    let gc: *mut __SlateRecord_pycore_interp_structs_h_170 = _Py_AS_GC(op);
    ((unsafe { (*gc)._gc_next }) != 0) as i32
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

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateStorage_empty_keys_struct {
    dk_refcnt: i64,
    dk_log2_size: u8,
    dk_log2_index_bytes: u8,
    dk_kind: u8,
    dk_version: u32,
    dk_usable: i64,
    dk_nentries: i64,
    dk_indices: [i8; 8],
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
    pub struct __SlateBits152U0 {
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
    pub struct __SlateBits214U0 {
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

use slate_support::memcmp;
use slate_support::memcpy;
use slate_support::memset;

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

fn _PyObject_IsUniquelyReferenced(ob: *mut _object) -> i32 {
    (Py_REFCNT!(ob) == 1) as i32
}

fn _PyType_HasFeature(r#type: *mut _typeobject, feature: u64) -> i32 {
    ((unsafe { (*r#type).tp_flags }) & feature != 0) as i32
}

fn Py_DECREF_MORTAL(op: *mut _object) {
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
fn _PyObject_HashFast(op: *mut _object) -> i64 {
    if PyUnicode_CheckExact!(op) != 0 {
        let hash: i64 = unsafe { (*_PyASCIIObject_CAST!(op)).hash };
        if hash != (1i32.wrapping_neg() as i64) {
            return hash;
        }
    }
    unsafe { PyObject_Hash(op) }
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord_pycore_object_h_926 {
    dict: *mut __SlateRecord_dictobject_h_33,
}

fn _PyObject_ManagedDictPointer(obj: *mut _object) -> *mut __SlateRecord_pycore_object_h_926 {
    (obj as *mut i8).wrapping_offset(
        (core::mem::size_of::<*mut _object>() as i64).wrapping_mul(3i32.wrapping_neg() as i64)
            as isize,
    ) as *mut __SlateRecord_pycore_object_h_926
}

fn _PyObject_GetManagedDict(obj: *mut _object) -> *mut __SlateRecord_dictobject_h_33 {
    let dorv: *mut __SlateRecord_pycore_object_h_926 = _PyObject_ManagedDictPointer(obj);
    unsafe { (*dorv).dict }
}

fn _PyObject_InlineValues(obj: *mut _object) -> *mut _dictvalues {
    let tp: *mut _typeobject = Py_TYPE!(obj);
    (obj as *mut i8).wrapping_offset((unsafe { (*tp).tp_basicsize }) as isize) as *mut _dictvalues
}

#[inline(always)]
fn _Py_INCREF_MORTAL(__v0: *mut _object) {
    let __v1: u32 = unsafe { (*__v0).__slate_anon_0.__slate_anon_1.ob_refcnt };
    unsafe {
        (*__v0).__slate_anon_0.__slate_anon_1.ob_refcnt = __v1.wrapping_add(1);
    }
    _Py_INCREF_STAT_INC!();
}

static PyStackRef_NULL: _PyStackRef = _PyStackRef { bits: 1u64 };

fn PyStackRef_RefcountOnObject(r#ref: _PyStackRef) -> i32 {
    ((unsafe { r#ref.bits }) & 1 == 0) as i32
}

fn PyStackRef_AsPyObjectBorrow(r#ref: _PyStackRef) -> *mut _object {
    BITS_TO_PTR_MASKED!(r#ref)
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

fn _PyStackRef_FromPyObjectNew(obj: *mut _object) -> _PyStackRef {
    let mut __slate_compound_0: _PyStackRef = unsafe { core::mem::zeroed() };
    let mut __slate_compound_1: _PyStackRef = unsafe { core::mem::zeroed() };
    if _Py_IsImmortal!(obj) != 0 {
        return unsafe {
            *{
                __slate_compound_0 = _PyStackRef {
                    bits: (obj as u64) | 1,
                };
                core::ptr::addr_of_mut!(__slate_compound_0)
            }
        };
    }
    _Py_INCREF_MORTAL(obj);
    let r#ref: _PyStackRef = unsafe {
        *{
            __slate_compound_1 = _PyStackRef { bits: obj as u64 };
            core::ptr::addr_of_mut!(__slate_compound_1)
        }
    };
    PyStackRef_CheckValid!();
    return r#ref;
}

fn PyStackRef_CLOSE(r#ref: _PyStackRef) {
    if PyStackRef_RefcountOnObject(r#ref) != 0 {
        Py_DECREF_MORTAL!(BITS_TO_PTR!(r#ref));
    }
}

fn PyStackRef_XCLOSE(r#ref: _PyStackRef) {
    if PyStackRef_RefcountOnObject(r#ref) != 0 {
        Py_DECREF_MORTAL!(BITS_TO_PTR!(r#ref));
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
struct __SlateRecord_pycore_dict_h_85 {
    me_hash: i64,
    me_key: *mut _object,
    me_value: *mut _object,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_dict_h_90 {
    me_key: *mut _object,
    me_value: *mut _object,
}

// Used internally
// Used internally
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

// This must be no more than 250, for the prefix size to fit in one byte.
#[repr(C)]
#[derive(Clone, Copy)]
struct _dictvalues {
    capacity: u8,
    size: u8,
    embedded: u8,
    valid: u8,
    values: [*mut _object; 1],
}

fn _DK_ENTRIES(dk: *mut _dictkeysobject) -> *mut () {
    let indices: *mut i8 =
        (unsafe { core::ptr::addr_of_mut!((*dk).dk_indices) as *mut i8 }) as *mut i8;
    let index: u64 = 1u64 << ((unsafe { (*dk).dk_log2_index_bytes }) as i32);
    indices.wrapping_offset(index as isize) as *mut ()
}

fn DK_ENTRIES(dk: *mut _dictkeysobject) -> *mut __SlateRecord_pycore_dict_h_85 {
    _DK_ENTRIES(dk) as *mut __SlateRecord_pycore_dict_h_85
}

fn DK_UNICODE_ENTRIES(dk: *mut _dictkeysobject) -> *mut __SlateRecord_pycore_dict_h_90 {
    _DK_ENTRIES(dk) as *mut __SlateRecord_pycore_dict_h_90
}

fn _PyDict_NotifyEvent(
    interp: *mut _is,
    event: u32,
    mp: *mut __SlateRecord_dictobject_h_33,
    key: *mut _object,
    value: *mut _object,
) {
    let watcher_bits: i32 = ((unsafe { (*mp)._ma_watcher_tag }) & 255u64) as i32;
    if watcher_bits != 0 {
        RARE_EVENT_STAT_INC!();
        _PyDict_SendEvent(watcher_bits, event, mp, key, value);
    }
}

fn get_insertion_order_array(values: *mut _dictvalues) -> *mut u8 {
    unsafe { (*values).values.as_mut_ptr() as *mut *mut _object }
        .wrapping_offset(((unsafe { (*values).capacity }) as i32) as isize) as *mut u8
}

fn _PyDictValues_AddToInsertionOrder(values: *mut _dictvalues, ix: i64) {
    let size: i32 = (unsafe { (*values).size }) as i32;
    let array: *mut u8 = get_insertion_order_array(values);
    unsafe {
        *array.wrapping_offset(size as isize) = ix as u8;
    }
    unsafe {
        (*values).size = size.wrapping_add(1) as u8;
    }
}

fn shared_keys_usable_size(keys: *mut _dictkeysobject) -> u64 {
    // dk_usable will decrease for each instance that is created and each
    //
    // value that is added.  dk_nentries will increase for each value that
    //
    // is added.  We want to always return the right value or larger.
    //
    // We therefore increase dk_nentries first and we decrease dk_usable
    //
    // second, and conversely here we read dk_usable first and dk_entries
    //
    // second (to avoid the case where we read entries before the increment
    //
    // and read usable after the decrement)
    let dk_usable: i64 = unsafe { (*keys).dk_usable };
    let dk_nentries: i64 = unsafe { (*keys).dk_nentries };
    dk_nentries.wrapping_add(dk_usable) as u64
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

fn _PyFreeList_PopMem(fl: *mut _Py_freelist) -> *mut () {
    let op: *mut () = _PyFreeList_PopNoStats(fl);
    if !op.is_null() {
        OBJECT_STAT_INC!();
    }
    op
}

fn _PyTuple_Recycle(op: *mut _object) {
    _PyTuple_RESET_HASH_CACHE!(op);
    if _PyObject_GC_IS_TRACKED!(op) == 0 {
        _PyObject_GC_TRACK__2!(op);
    }
}

fn unicode_eq(str1: *mut _object, str2: *mut _object) -> i32 {
    let len: i64 = PyUnicode_GET_LENGTH!(str1);
    if PyUnicode_GET_LENGTH!(str2) != len {
        return 0;
    }
    let kind: i32 = PyUnicode_KIND!(str1);
    if PyUnicode_KIND!(str2) != kind {
        return 0;
    }
    let data1: *const () = PyUnicode_DATA!(str1) as *const ();
    let data2: *const () = PyUnicode_DATA!(str2) as *const ();
    ((unsafe {
        memcmp(
            data1 as *const core::ffi::c_void,
            data2 as *const core::ffi::c_void,
            (len.wrapping_mul(kind as i64) as u64) as usize,
        ) as i32
    }) == 0) as i32
}

// The distribution includes a separate file, Objects/dictnotes.txt,
// describing explorations into dictionary design and optimization.
// It covers typical dictionary use patterns, the parameters for
// tuning dictionaries, and several ideas for possible optimizations.
// PyDictKeysObject
//
// This implements the dictionary's hashtable.
//
// As of Python 3.6, this is compact and ordered. Basic idea is described here:
// * https://mail.python.org/pipermail/python-dev/2012-December/123028.html
// * https://morepypy.blogspot.com/2015/01/faster-more-memory-efficient-and-more.html
//
// layout:
//
// | dk_refcnt           |
// | dk_log2_size        |
// | dk_log2_index_bytes |
// | dk_kind             |
// | dk_version          |
// | dk_usable           |
// | dk_nentries         |
// | dk_indices[]        |
// |                     |
// | dk_entries[]        |
// |                     |
//
// dk_indices is actual hashtable.  It holds index in entries, or DKIX_EMPTY(-1)
// or DKIX_DUMMY(-2).
// Size of indices is dk_size.  Type of each index in indices varies with dk_size:
//
// * int8  for          dk_size <= 128
// * int16 for 256   <= dk_size <= 2**15
// * int32 for 2**16 <= dk_size <= 2**31
// * int64 for 2**32 <= dk_size
//
// dk_entries is array of PyDictKeyEntry when dk_kind == DICT_KEYS_GENERAL or
// PyDictUnicodeEntry otherwise. Its length is USABLE_FRACTION(dk_size).
//
// NOTE: Since negative value is used for DKIX_EMPTY and DKIX_DUMMY, type of
// dk_indices entry is signed integer and int16 is used for table which
// dk_size == 256.
// The DictObject can be in one of two forms.
//
// Either:
//   A combined table:
//     ma_values == NULL, dk_refcnt == 1.
//     Values are stored in the me_value field of the PyDictKeyEntry.
// Or:
//   A split table:
//     ma_values != NULL, dk_refcnt >= 1
//     Values are stored in the ma_values array.
//     Only string (unicode) keys are allowed.
//
// There are four kinds of slots in the table (slot is index, and
// DK_ENTRIES(keys)[index] if index >= 0):
//
// 1. Unused.  index == DKIX_EMPTY
//    Does not hold an active (key, value) pair now and never did.  Unused can
//    transition to Active upon key insertion.  This is each slot's initial state.
//
// 2. Active.  index >= 0, me_key != NULL and me_value != NULL
//    Holds an active (key, value) pair.  Active can transition to Dummy or
//    Pending upon key deletion (for combined and split tables respectively).
//    This is the only case in which me_value != NULL.
//
// 3. Dummy.  index == DKIX_DUMMY  (combined only)
//    Previously held an active (key, value) pair, but that was deleted and an
//    active pair has not yet overwritten the slot.  Dummy can transition to
//    Active upon key insertion.  Dummy slots cannot be made Unused again
//    else the probe sequence in case of collision would have no way to know
//    they were once active.
//    In free-threaded builds dummy slots are not re-used to allow lock-free
//    lookups to proceed safely.
//
// 4. Pending. index >= 0, key != NULL, and value == NULL  (split only)
//    Not yet inserted in split-table.
// Preserving insertion order
//
// It's simple for combined table.  Since dk_entries is mostly append only, we can
// get insertion order by just iterating dk_entries.
//
// One exception is .popitem().  It removes last item in dk_entries and decrement
// dk_nentries to achieve amortized O(1).  Since there are DKIX_DUMMY remains in
// dk_indices, we can't increment dk_usable even though dk_nentries is
// decremented.
//
// To preserve the order in a split table, a bit vector is used  to record the
// insertion order. When a key is inserted the bit vector is shifted up by 4 bits
// and the index of the key is stored in the low 4 bits.
// As a consequence of this, split keys have a maximum size of 16.
// PyDict_MINSIZE is the starting size for any new dict.
// 8 allows dicts with no more than 5 active entries; experiments suggested
// this suffices for the majority of dicts (consisting mostly of usually-small
// dicts created to pass keyword arguments).
// Making this 8, rather than 4 reduces the number of resizes for most
// dictionaries, without any significant extra memory use.
// _Py_bit_length
// _PyObject_CallNoArgs()
// _PyEval_GetBuiltin()
// stats
// Py_BEGIN_CRITICAL_SECTION, Py_END_CRITICAL_SECTION
// export _PyDict_SizeOf()
// _PyFreeListState_GET()
// _PyObject_GC_IS_TRACKED()
// _PyObject_GC_TRACK(), _PyDebugAllocatorStats()
// FT_ATOMIC_LOAD_SSIZE_RELAXED
// _PyErr_GetRaisedException()
// _PyThreadState_GET()
// _PySet_NextEntry()
// _PyTuple_Recycle()
// _PyUnicode_InternImmortal()
// unicode_eq()
// [clinic input]
// class dict "PyDictObject *" "&PyDict_Type"
// [clinic start generated code]
//
// [clinic end generated code: output=da39a3ee5e6b4b0d input=f157a5a0ce9589d6]
// To ensure the lookup algorithm terminates, there must be at least one Unused
// slot (NULL key) in the table.
// To avoid slowing down lookups on a near-full table, we resize the table when
// it's USABLE_FRACTION (currently two-thirds) full.
fn split_keys_entry_added(keys: *mut _dictkeysobject) {
    let __v1: i64 = unsafe { (*keys).dk_usable };
    unsafe {
        (*keys).dk_usable = __v1.wrapping_sub(1);
    }
    let __v4: i64 = unsafe { (*keys).dk_nentries };
    unsafe {
        (*keys).dk_nentries = __v4.wrapping_add(1);
    }
}

fn set_keys(mp: *mut __SlateRecord_dictobject_h_33, keys: *mut _dictkeysobject) {
    unsafe {
        (*mp).ma_keys = keys;
    }
}

fn set_values(mp: *mut __SlateRecord_dictobject_h_33, values: *mut _dictvalues) {
    unsafe {
        (*mp).ma_values = values;
    }
}

fn load_keys_nentries(mp: *mut __SlateRecord_dictobject_h_33) -> i64 {
    unsafe { (*(*mp).ma_keys).dk_nentries }
}

static dict_fromkeys__doc__: __SlateAlign16<[i8; 118]> = __SlateAlign16([
    102i8, 114i8, 111i8, 109i8, 107i8, 101i8, 121i8, 115i8, 40i8, 36i8, 116i8, 121i8, 112i8, 101i8,
    44i8, 32i8, 105i8, 116i8, 101i8, 114i8, 97i8, 98i8, 108i8, 101i8, 44i8, 32i8, 118i8, 97i8,
    108i8, 117i8, 101i8, 61i8, 78i8, 111i8, 110i8, 101i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8,
    10i8, 10i8, 67i8, 114i8, 101i8, 97i8, 116i8, 101i8, 32i8, 97i8, 32i8, 110i8, 101i8, 119i8,
    32i8, 100i8, 105i8, 99i8, 116i8, 105i8, 111i8, 110i8, 97i8, 114i8, 121i8, 32i8, 119i8, 105i8,
    116i8, 104i8, 32i8, 107i8, 101i8, 121i8, 115i8, 32i8, 102i8, 114i8, 111i8, 109i8, 32i8, 105i8,
    116i8, 101i8, 114i8, 97i8, 98i8, 108i8, 101i8, 32i8, 97i8, 110i8, 100i8, 32i8, 118i8, 97i8,
    108i8, 117i8, 101i8, 115i8, 32i8, 115i8, 101i8, 116i8, 32i8, 116i8, 111i8, 32i8, 118i8, 97i8,
    108i8, 117i8, 101i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.dictobject.dict_fromkeys")]
extern "C-unwind" fn dict_fromkeys(
    r#type: *mut _object,
    args: *const *mut _object,
    nargs: i64,
) -> *mut _object {
    let mut value: *mut _object = unsafe { core::mem::zeroed() };
    let mut iterable: *mut _object = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        value = core::ptr::addr_of_mut!(_Py_NoneStruct);
        if _PyArg_CheckPositional!(c"fromkeys".as_ptr() as *mut i8, nargs, 1i32, 2i32) {
            iterable = *args.wrapping_offset(0 as isize);
            if nargs >= 2 {
                value = *args.wrapping_offset(1 as isize);
            }
            return_value = dict_fromkeys_impl(r#type as *mut _typeobject, iterable, value);
        }
        return return_value;
    }
}

static dict_copy__doc__: __SlateAlign16<[i8; 54]> = __SlateAlign16([
    99i8, 111i8, 112i8, 121i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8, 47i8, 41i8,
    10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 97i8, 32i8, 115i8,
    104i8, 97i8, 108i8, 108i8, 111i8, 119i8, 32i8, 99i8, 111i8, 112i8, 121i8, 32i8, 111i8, 102i8,
    32i8, 116i8, 104i8, 101i8, 32i8, 100i8, 105i8, 99i8, 116i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.dictobject.dict_copy")]
extern "C-unwind" fn dict_copy(self_: *mut _object, _unused_ignored: *mut _object) -> *mut _object {
    dict_copy_impl(self_ as *mut __SlateRecord_dictobject_h_33)
}

static dict___contains____doc__: __SlateAlign16<[i8; 90]> = __SlateAlign16([
    95i8, 95i8, 99i8, 111i8, 110i8, 116i8, 97i8, 105i8, 110i8, 115i8, 95i8, 95i8, 40i8, 36i8,
    115i8, 101i8, 108i8, 102i8, 44i8, 32i8, 107i8, 101i8, 121i8, 44i8, 32i8, 47i8, 41i8, 10i8,
    45i8, 45i8, 10i8, 10i8, 84i8, 114i8, 117i8, 101i8, 32i8, 105i8, 102i8, 32i8, 116i8, 104i8,
    101i8, 32i8, 100i8, 105i8, 99i8, 116i8, 105i8, 111i8, 110i8, 97i8, 114i8, 121i8, 32i8, 104i8,
    97i8, 115i8, 32i8, 116i8, 104i8, 101i8, 32i8, 115i8, 112i8, 101i8, 99i8, 105i8, 102i8, 105i8,
    101i8, 100i8, 32i8, 107i8, 101i8, 121i8, 44i8, 32i8, 101i8, 108i8, 115i8, 101i8, 32i8, 70i8,
    97i8, 108i8, 115i8, 101i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.dictobject.dict___contains__")]
extern "C-unwind" fn dict___contains__(self_: *mut _object, key: *mut _object) -> *mut _object {
    let mut return_value: *mut _object = core::ptr::null_mut::<_object>();
    return_value = dict___contains___impl(self_ as *mut __SlateRecord_dictobject_h_33, key);
    return_value
}

static dict_get__doc__: __SlateAlign16<[i8; 105]> = __SlateAlign16([
    103i8, 101i8, 116i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8, 107i8, 101i8, 121i8,
    44i8, 32i8, 100i8, 101i8, 102i8, 97i8, 117i8, 108i8, 116i8, 61i8, 78i8, 111i8, 110i8, 101i8,
    44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8,
    32i8, 116i8, 104i8, 101i8, 32i8, 118i8, 97i8, 108i8, 117i8, 101i8, 32i8, 102i8, 111i8, 114i8,
    32i8, 107i8, 101i8, 121i8, 32i8, 105i8, 102i8, 32i8, 107i8, 101i8, 121i8, 32i8, 105i8, 115i8,
    32i8, 105i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 100i8, 105i8, 99i8, 116i8, 105i8, 111i8,
    110i8, 97i8, 114i8, 121i8, 44i8, 32i8, 101i8, 108i8, 115i8, 101i8, 32i8, 100i8, 101i8, 102i8,
    97i8, 117i8, 108i8, 116i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.dictobject.dict_get")]
extern "C-unwind" fn dict_get(
    self_: *mut _object,
    args: *const *mut _object,
    nargs: i64,
) -> *mut _object {
    let mut default_value: *mut _object = unsafe { core::mem::zeroed() };
    let mut key: *mut _object = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        default_value = core::ptr::addr_of_mut!(_Py_NoneStruct);
        if _PyArg_CheckPositional!(c"get".as_ptr() as *mut i8, nargs, 1i32, 2i32) {
            key = *args.wrapping_offset(0 as isize);
            if nargs >= 2 {
                default_value = *args.wrapping_offset(1 as isize);
            }
            return_value = dict_get_impl(
                self_ as *mut __SlateRecord_dictobject_h_33,
                key,
                default_value,
            );
        }
        return return_value;
    }
}

static dict_setdefault__doc__: __SlateAlign16<[i8; 181]> = __SlateAlign16([
    115i8, 101i8, 116i8, 100i8, 101i8, 102i8, 97i8, 117i8, 108i8, 116i8, 40i8, 36i8, 115i8, 101i8,
    108i8, 102i8, 44i8, 32i8, 107i8, 101i8, 121i8, 44i8, 32i8, 100i8, 101i8, 102i8, 97i8, 117i8,
    108i8, 116i8, 61i8, 78i8, 111i8, 110i8, 101i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8,
    10i8, 73i8, 110i8, 115i8, 101i8, 114i8, 116i8, 32i8, 107i8, 101i8, 121i8, 32i8, 119i8, 105i8,
    116i8, 104i8, 32i8, 97i8, 32i8, 118i8, 97i8, 108i8, 117i8, 101i8, 32i8, 111i8, 102i8, 32i8,
    100i8, 101i8, 102i8, 97i8, 117i8, 108i8, 116i8, 32i8, 105i8, 102i8, 32i8, 107i8, 101i8, 121i8,
    32i8, 105i8, 115i8, 32i8, 110i8, 111i8, 116i8, 32i8, 105i8, 110i8, 32i8, 116i8, 104i8, 101i8,
    32i8, 100i8, 105i8, 99i8, 116i8, 105i8, 111i8, 110i8, 97i8, 114i8, 121i8, 46i8, 10i8, 10i8,
    82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 118i8, 97i8, 108i8,
    117i8, 101i8, 32i8, 102i8, 111i8, 114i8, 32i8, 107i8, 101i8, 121i8, 32i8, 105i8, 102i8, 32i8,
    107i8, 101i8, 121i8, 32i8, 105i8, 115i8, 32i8, 105i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8,
    100i8, 105i8, 99i8, 116i8, 105i8, 111i8, 110i8, 97i8, 114i8, 121i8, 44i8, 32i8, 101i8, 108i8,
    115i8, 101i8, 32i8, 100i8, 101i8, 102i8, 97i8, 117i8, 108i8, 116i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.dictobject.dict_setdefault")]
extern "C-unwind" fn dict_setdefault(
    self_: *mut _object,
    args: *const *mut _object,
    nargs: i64,
) -> *mut _object {
    let mut default_value: *mut _object = unsafe { core::mem::zeroed() };
    let mut key: *mut _object = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        default_value = core::ptr::addr_of_mut!(_Py_NoneStruct);
        if _PyArg_CheckPositional!(c"setdefault".as_ptr() as *mut i8, nargs, 1i32, 2i32) {
            key = *args.wrapping_offset(0 as isize);
            if nargs >= 2 {
                default_value = *args.wrapping_offset(1 as isize);
            }
            return_value = dict_setdefault_impl(
                self_ as *mut __SlateRecord_dictobject_h_33,
                key,
                default_value,
            );
        }
        return return_value;
    }
}

static dict_clear__doc__: __SlateAlign16<[i8; 52]> = __SlateAlign16([
    99i8, 108i8, 101i8, 97i8, 114i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8, 47i8,
    41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 109i8, 111i8, 118i8, 101i8, 32i8, 97i8, 108i8,
    108i8, 32i8, 105i8, 116i8, 101i8, 109i8, 115i8, 32i8, 102i8, 114i8, 111i8, 109i8, 32i8, 116i8,
    104i8, 101i8, 32i8, 100i8, 105i8, 99i8, 116i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.dictobject.dict_clear")]
extern "C-unwind" fn dict_clear(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    dict_clear_impl(self_ as *mut __SlateRecord_dictobject_h_33)
}

static dict_pop__doc__: __SlateAlign16<[i8; 210]> = __SlateAlign16([
    112i8, 111i8, 112i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8, 107i8, 101i8, 121i8,
    44i8, 32i8, 100i8, 101i8, 102i8, 97i8, 117i8, 108i8, 116i8, 61i8, 60i8, 117i8, 110i8, 114i8,
    101i8, 112i8, 114i8, 101i8, 115i8, 101i8, 110i8, 116i8, 97i8, 98i8, 108i8, 101i8, 62i8, 44i8,
    32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 68i8, 46i8, 112i8, 111i8, 112i8, 40i8, 107i8,
    91i8, 44i8, 100i8, 93i8, 41i8, 32i8, 45i8, 62i8, 32i8, 118i8, 44i8, 32i8, 114i8, 101i8, 109i8,
    111i8, 118i8, 101i8, 32i8, 115i8, 112i8, 101i8, 99i8, 105i8, 102i8, 105i8, 101i8, 100i8, 32i8,
    107i8, 101i8, 121i8, 32i8, 97i8, 110i8, 100i8, 32i8, 114i8, 101i8, 116i8, 117i8, 114i8, 110i8,
    32i8, 116i8, 104i8, 101i8, 32i8, 99i8, 111i8, 114i8, 114i8, 101i8, 115i8, 112i8, 111i8, 110i8,
    100i8, 105i8, 110i8, 103i8, 32i8, 118i8, 97i8, 108i8, 117i8, 101i8, 46i8, 10i8, 10i8, 73i8,
    102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 107i8, 101i8, 121i8, 32i8, 105i8, 115i8, 32i8, 110i8,
    111i8, 116i8, 32i8, 102i8, 111i8, 117i8, 110i8, 100i8, 44i8, 32i8, 114i8, 101i8, 116i8, 117i8,
    114i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 100i8, 101i8, 102i8, 97i8, 117i8, 108i8, 116i8,
    32i8, 105i8, 102i8, 32i8, 103i8, 105i8, 118i8, 101i8, 110i8, 59i8, 32i8, 111i8, 116i8, 104i8,
    101i8, 114i8, 119i8, 105i8, 115i8, 101i8, 44i8, 10i8, 114i8, 97i8, 105i8, 115i8, 101i8, 32i8,
    97i8, 32i8, 75i8, 101i8, 121i8, 69i8, 114i8, 114i8, 111i8, 114i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.dictobject.dict_pop")]
extern "C-unwind" fn dict_pop(
    self_: *mut _object,
    args: *const *mut _object,
    nargs: i64,
) -> *mut _object {
    let mut default_value: *mut _object = unsafe { core::mem::zeroed() };
    let mut key: *mut _object = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        default_value = core::ptr::null_mut::<_object>();
        if _PyArg_CheckPositional!(c"pop".as_ptr() as *mut i8, nargs, 1i32, 2i32) {
            key = *args.wrapping_offset(0 as isize);
            if nargs >= 2 {
                default_value = *args.wrapping_offset(1 as isize);
            }
            return_value = dict_pop_impl(
                self_ as *mut __SlateRecord_dictobject_h_33,
                key,
                default_value,
            );
        }
        return return_value;
    }
}

static dict_popitem__doc__: __SlateAlign16<[i8; 168]> = __SlateAlign16([
    112i8, 111i8, 112i8, 105i8, 116i8, 101i8, 109i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8,
    32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 109i8, 111i8, 118i8, 101i8, 32i8,
    97i8, 110i8, 100i8, 32i8, 114i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 97i8, 32i8, 40i8,
    107i8, 101i8, 121i8, 44i8, 32i8, 118i8, 97i8, 108i8, 117i8, 101i8, 41i8, 32i8, 112i8, 97i8,
    105i8, 114i8, 32i8, 97i8, 115i8, 32i8, 97i8, 32i8, 50i8, 45i8, 116i8, 117i8, 112i8, 108i8,
    101i8, 46i8, 10i8, 10i8, 80i8, 97i8, 105i8, 114i8, 115i8, 32i8, 97i8, 114i8, 101i8, 32i8,
    114i8, 101i8, 116i8, 117i8, 114i8, 110i8, 101i8, 100i8, 32i8, 105i8, 110i8, 32i8, 76i8, 73i8,
    70i8, 79i8, 32i8, 40i8, 108i8, 97i8, 115i8, 116i8, 45i8, 105i8, 110i8, 44i8, 32i8, 102i8,
    105i8, 114i8, 115i8, 116i8, 45i8, 111i8, 117i8, 116i8, 41i8, 32i8, 111i8, 114i8, 100i8, 101i8,
    114i8, 46i8, 10i8, 82i8, 97i8, 105i8, 115i8, 101i8, 115i8, 32i8, 75i8, 101i8, 121i8, 69i8,
    114i8, 114i8, 111i8, 114i8, 32i8, 105i8, 102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 100i8, 105i8,
    99i8, 116i8, 32i8, 105i8, 115i8, 32i8, 101i8, 109i8, 112i8, 116i8, 121i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.dictobject.dict_popitem")]
extern "C-unwind" fn dict_popitem(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    let mut return_value: *mut _object = core::ptr::null_mut::<_object>();
    return_value = dict_popitem_impl(self_ as *mut __SlateRecord_dictobject_h_33);
    return_value
}

static dict___sizeof____doc__: __SlateAlign16<[i8; 74]> = __SlateAlign16([
    95i8, 95i8, 115i8, 105i8, 122i8, 101i8, 111i8, 102i8, 95i8, 95i8, 40i8, 36i8, 115i8, 101i8,
    108i8, 102i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8,
    114i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 115i8, 105i8, 122i8, 101i8, 32i8, 111i8, 102i8,
    32i8, 116i8, 104i8, 101i8, 32i8, 100i8, 105i8, 99i8, 116i8, 32i8, 105i8, 110i8, 32i8, 109i8,
    101i8, 109i8, 111i8, 114i8, 121i8, 44i8, 32i8, 105i8, 110i8, 32i8, 98i8, 121i8, 116i8, 101i8,
    115i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.dictobject.dict___sizeof__")]
extern "C-unwind" fn dict___sizeof__(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    dict___sizeof___impl(self_ as *mut __SlateRecord_dictobject_h_33)
}

static dict___reversed____doc__: __SlateAlign16<[i8; 73]> = __SlateAlign16([
    95i8, 95i8, 114i8, 101i8, 118i8, 101i8, 114i8, 115i8, 101i8, 100i8, 95i8, 95i8, 40i8, 36i8,
    115i8, 101i8, 108i8, 102i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8,
    116i8, 117i8, 114i8, 110i8, 32i8, 97i8, 32i8, 114i8, 101i8, 118i8, 101i8, 114i8, 115i8, 101i8,
    32i8, 105i8, 116i8, 101i8, 114i8, 97i8, 116i8, 111i8, 114i8, 32i8, 111i8, 118i8, 101i8, 114i8,
    32i8, 116i8, 104i8, 101i8, 32i8, 100i8, 105i8, 99i8, 116i8, 32i8, 107i8, 101i8, 121i8, 115i8,
    46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.dictobject.dict___reversed__")]
extern "C-unwind" fn dict___reversed__(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    dict___reversed___impl(self_ as *mut __SlateRecord_dictobject_h_33)
}

static dict_keys__doc__: __SlateAlign16<[i8; 81]> = __SlateAlign16([
    107i8, 101i8, 121i8, 115i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8, 47i8, 41i8,
    10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 97i8, 32i8, 115i8,
    101i8, 116i8, 45i8, 108i8, 105i8, 107i8, 101i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8,
    32i8, 112i8, 114i8, 111i8, 118i8, 105i8, 100i8, 105i8, 110i8, 103i8, 32i8, 97i8, 32i8, 118i8,
    105i8, 101i8, 119i8, 32i8, 111i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 100i8, 105i8, 99i8,
    116i8, 39i8, 115i8, 32i8, 107i8, 101i8, 121i8, 115i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.dictobject.dict_keys")]
extern "C-unwind" fn dict_keys(self_: *mut _object, _unused_ignored: *mut _object) -> *mut _object {
    dict_keys_impl(self_ as *mut __SlateRecord_dictobject_h_33)
}

static dict_items__doc__: __SlateAlign16<[i8; 83]> = __SlateAlign16([
    105i8, 116i8, 101i8, 109i8, 115i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8, 47i8,
    41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 97i8, 32i8,
    115i8, 101i8, 116i8, 45i8, 108i8, 105i8, 107i8, 101i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8,
    116i8, 32i8, 112i8, 114i8, 111i8, 118i8, 105i8, 100i8, 105i8, 110i8, 103i8, 32i8, 97i8, 32i8,
    118i8, 105i8, 101i8, 119i8, 32i8, 111i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 100i8, 105i8,
    99i8, 116i8, 39i8, 115i8, 32i8, 105i8, 116i8, 101i8, 109i8, 115i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.dictobject.dict_items")]
extern "C-unwind" fn dict_items(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    dict_items_impl(self_ as *mut __SlateRecord_dictobject_h_33)
}

static dict_values__doc__: __SlateAlign16<[i8; 77]> = __SlateAlign16([
    118i8, 97i8, 108i8, 117i8, 101i8, 115i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8,
    47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 97i8,
    110i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 32i8, 112i8, 114i8, 111i8, 118i8, 105i8,
    100i8, 105i8, 110i8, 103i8, 32i8, 97i8, 32i8, 118i8, 105i8, 101i8, 119i8, 32i8, 111i8, 110i8,
    32i8, 116i8, 104i8, 101i8, 32i8, 100i8, 105i8, 99i8, 116i8, 39i8, 115i8, 32i8, 118i8, 97i8,
    108i8, 117i8, 101i8, 115i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.dictobject.dict_values")]
extern "C-unwind" fn dict_values(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    dict_values_impl(self_ as *mut __SlateRecord_dictobject_h_33)
}

// Major subtleties ahead:  Most hash schemes depend on having a "good" hash
// function, in the sense of simulating randomness.  Python doesn't:  its most
// important hash functions (for ints) are very regular in common
// cases:
//
//   >>>[hash(i) for i in range(4)]
//   [0, 1, 2, 3]
//
// This isn't necessarily bad!  To the contrary, in a table of size 2**i, taking
// the low-order i bits as the initial table index is extremely fast, and there
// are no collisions at all for dicts indexed by a contiguous range of ints. So
// this gives better-than-random behavior in common cases, and that's very
// desirable.
//
// OTOH, when collisions occur, the tendency to fill contiguous slices of the
// hash table makes a good collision resolution strategy crucial.  Taking only
// the last i bits of the hash code is also vulnerable:  for example, consider
// the list [i << 16 for i in range(20000)] as a set of keys.  Since ints are
// their own hash codes, and this fits in a dict of size 2**15, the last 15 bits
//  of every hash code are all 0:  they *all* map to the same table index.
//
// But catering to unusual cases should not slow the usual ones, so we just take
// the last i bits anyway.  It's up to collision resolution to do the rest.  If
// we *usually* find the key we're looking for on the first try (and, it turns
// out, we usually do -- the table load factor is kept under 2/3, so the odds
// are solidly in our favor), then it makes best sense to keep the initial index
// computation dirt cheap.
//
// The first half of collision resolution is to visit table indices via this
// recurrence:
//
//     j = ((5*j) + 1) mod 2**i
//
// For any initial j in range(2**i), repeating that 2**i times generates each
// int in range(2**i) exactly once (see any text on random-number generation for
// proof).  By itself, this doesn't help much:  like linear probing (setting
// j += 1, or j -= 1, on each loop trip), it scans the table entries in a fixed
// order.  This would be bad, except that's not the only thing we do, and it's
// actually *good* in the common cases where hash keys are consecutive.  In an
// example that's really too small to make this entirely clear, for a table of
// size 2**3 the order of indices is:
//
//     0 -> 1 -> 6 -> 7 -> 4 -> 5 -> 2 -> 3 -> 0 [and here it's repeating]
//
// If two things come in at index 5, the first place we look after is index 2,
// not 6, so if another comes in at index 6 the collision at 5 didn't hurt it.
// Linear probing is deadly in this case because there the fixed probe order
// is the *same* as the order consecutive keys are likely to arrive.  But it's
// extremely unlikely hash codes will follow a 5*j+1 recurrence by accident,
// and certain that consecutive hash codes do not.
//
// The other half of the strategy is to get the other bits of the hash code
// into play.  This is done by initializing a (unsigned) vrbl "perturb" to the
// full hash code, and changing the recurrence to:
//
//     perturb >>= PERTURB_SHIFT;
//     j = (5*j) + 1 + perturb;
//     use j % 2**i as the next table index;
//
// Now the probe sequence depends (eventually) on every bit in the hash code,
// and the pseudo-scrambling property of recurring on 5*j+1 is more valuable,
// because it quickly magnifies small differences in the bits that didn't affect
// the initial index.  Note that because perturb is unsigned, if the recurrence
// is executed often enough perturb eventually becomes and remains 0.  At that
// point (very rarely reached) the recurrence is on (just) 5*j+1 again, and
// that's certain to find an empty slot eventually (since it generates every int
// in range(2**i), and we make sure there's always at least one empty slot).
//
// Selecting a good value for PERTURB_SHIFT is a balancing act.  You want it
// small so that the high bits of the hash code continue to affect the probe
// sequence across iterations; but you want it large so that in really bad cases
// the high-order hash bits have an effect on early iterations.  5 was "the
// best" in minimizing total collisions across experiments Tim Peters ran (on
// both normal and pathological cases), but 4 and 6 weren't significantly worse.
//
// Historical: Reimer Behrends contributed the idea of using a polynomial-based
// approach, using repeated multiplication by x in GF(2**n) where an irreducible
// polynomial for each table size was chosen such that x was a primitive root.
// Christian Tismer later extended that to use division by x instead, as an
// efficient way to get the high bits of the hash code into play.  This scheme
// also gave excellent collision statistics, but was more expensive:  two
// if-tests were required inside the loop; computing "the next" index took about
// the same number of operations but without as much potential parallelism
// (e.g., computing 5*j can go on at the same time as computing 1+perturb in the
// above, and then shifting perturb can be done while the table index is being
// masked); and the PyDictObject struct required a member to hold the table's
// polynomial.  In Tim's experiments the current scheme ran faster, produced
// equally good collision statistics, needed less code & used less memory.
fn unicode_get_hash(o: *mut _object) -> i64 {
    unsafe { (*_PyASCIIObject_CAST!(o)).hash }
}

/// Print summary info about the state of the optimized allocator
#[unsafe(no_mangle)]
extern "C-unwind" fn _PyDict_DebugMallocStats(out: *mut slate_support::FILE) {
    unsafe {
        _PyDebugAllocatorStats(
            out,
            c"free PyDictObject".as_ptr(),
            _Py_FREELIST_SIZE!(dicts),
            core::mem::size_of::<__SlateRecord_dictobject_h_33>() as u64,
        )
    };
    unsafe {
        _PyDebugAllocatorStats(
            out,
            c"free PyDictKeysObject".as_ptr(),
            _Py_FREELIST_SIZE!(dictkeys),
            32u64,
        )
    };
}

// PyDictKeysObject has refcounts like PyObject does, so we have the
// following two functions to mirror what Py_INCREF() and Py_DECREF() do.
// (Keep in mind that PyDictKeysObject isn't actually a PyObject.)
// Likewise a PyDictKeysObject can be immortal (e.g. Py_EMPTY_KEYS),
// so we apply a naive version of what Py_INCREF() and Py_DECREF() do
// for immortal objects.
fn dictkeys_incref(dk: *mut _dictkeysobject) {
    if (unsafe { (*dk).dk_refcnt }) < 0 {
        return;
    }
    INCREF_KEYS!(dk);
}

fn dictkeys_decref(interp: *mut _is, dk: *mut _dictkeysobject, use_qsbr: bool) {
    if (unsafe { (*dk).dk_refcnt }) < 0 {
        return;
    }
    if DECREF_KEYS!(dk) == 1 {
        if DK_IS_UNICODE!(dk) {
            let entries: *mut __SlateRecord_pycore_dict_h_90 = DK_UNICODE_ENTRIES(dk);
            let n: i64 = unsafe { (*dk).dk_nentries };
            for i in 0..n {
                Py_XDECREF!(unsafe { (*entries.wrapping_offset(i as isize)).me_key });
                Py_XDECREF!(unsafe { (*entries.wrapping_offset(i as isize)).me_value });
            }
        } else {
            let entries: *mut __SlateRecord_pycore_dict_h_85 = DK_ENTRIES(dk);
            let mut n: i64 = 0;
            n = unsafe { (*dk).dk_nentries };
            for i in 0..n {
                Py_XDECREF!(unsafe { (*entries.wrapping_offset(i as isize)).me_key });
                Py_XDECREF!(unsafe { (*entries.wrapping_offset(i as isize)).me_value });
            }
        }
        free_keys_object(dk, use_qsbr);
    }
}

/// lookup indices.  returns DKIX_EMPTY, DKIX_DUMMY, or ix >=0
fn dictkeys_get_index(keys: *const _dictkeysobject, i: i64) -> i64 {
    let log2size: i32 = DK_LOG_SIZE!(keys) as i32;
    let mut ix: i64 = 0;
    if log2size < 8 {
        ix = (unsafe {
            *((core::ptr::addr_of!((*keys).dk_indices) as *const i8) as *const i8)
                .wrapping_offset(i as isize)
        }) as i64;
    } else {
        if log2size < 16 {
            ix = (unsafe {
                *((core::ptr::addr_of!((*keys).dk_indices) as *const i8) as *const i16)
                    .wrapping_offset(i as isize)
            }) as i64;
        } else {
            if log2size >= 32 {
                ix = unsafe {
                    *((core::ptr::addr_of!((*keys).dk_indices) as *const i8) as *const i64)
                        .wrapping_offset(i as isize)
                };
            } else {
                ix = (unsafe {
                    *((core::ptr::addr_of!((*keys).dk_indices) as *const i8) as *const i32)
                        .wrapping_offset(i as isize)
                }) as i64;
            }
        }
    }
    ix
}

/// write to indices.
fn dictkeys_set_index(keys: *mut _dictkeysobject, i: i64, ix: i64) {
    let log2size: i32 = DK_LOG_SIZE!(keys) as i32;
    if log2size < 8 {
        unsafe {
            *((core::ptr::addr_of_mut!((*keys).dk_indices) as *mut i8) as *mut i8)
                .wrapping_offset(i as isize) = ix as i8;
        }
    } else {
        if log2size < 16 {
            unsafe {
                *((core::ptr::addr_of_mut!((*keys).dk_indices) as *mut i8) as *mut i16)
                    .wrapping_offset(i as isize) = ix as i16;
            }
        } else {
            if log2size >= 32 {
                unsafe {
                    *((core::ptr::addr_of_mut!((*keys).dk_indices) as *mut i8) as *mut i64)
                        .wrapping_offset(i as isize) = ix as i64;
                }
            } else {
                unsafe {
                    *((core::ptr::addr_of_mut!((*keys).dk_indices) as *mut i8) as *mut i32)
                        .wrapping_offset(i as isize) = ix as i32;
                }
            }
        }
    }
}

/// Find the smallest dk_size >= minsize.
fn calculate_log2_keysize(mut minsize: i64) -> u8 {
    minsize = Py_MAX__2!(minsize, 8i32);
    _Py_bit_length(minsize.wrapping_sub(1) as u64) as u8
}

/// estimate_keysize is reverse function of USABLE_FRACTION.
///
/// This can be used to reserve enough size to insert n entries without
/// resizing.
fn estimate_log2_keysize(n: i64) -> u8 {
    calculate_log2_keysize(n.wrapping_mul(3).wrapping_add(1) / 2)
}

/// This immutable, empty PyDictKeysObject is used for PyDict_Clear()
/// (which cannot fail and thus can do no allocation).
///
/// See https://github.com/python/cpython/pull/127568#discussion_r1868070614
/// for the rationale of using dk_log2_index_bytes=3 instead of 0.
/// dk_refcnt
/// dk_log2_size
/// dk_log2_index_bytes
/// dk_kind
/// dk_version
/// dk_usable (immutable)
/// dk_nentries
/// dk_indices
static mut empty_keys_struct: __SlateStorage_empty_keys_struct = {
    let mut __t0: __SlateStorage_empty_keys_struct = unsafe { core::mem::zeroed() };
    __t0.dk_refcnt = 9223372036854775807i64.wrapping_neg().wrapping_sub(1i64);
    __t0.dk_log2_size = 0u8;
    __t0.dk_log2_index_bytes = 3u8;
    __t0.dk_kind = 1u8;
    __t0.dk_version = 1u32;
    __t0.dk_usable = 0i64;
    __t0.dk_nentries = 0i64;
    __t0.dk_indices = [
        1i32.wrapping_neg() as i8,
        1i32.wrapping_neg() as i8,
        1i32.wrapping_neg() as i8,
        1i32.wrapping_neg() as i8,
        1i32.wrapping_neg() as i8,
        1i32.wrapping_neg() as i8,
        1i32.wrapping_neg() as i8,
        1i32.wrapping_neg() as i8,
    ];
    __t0
};

// Uncomment to check the dict content in _PyDict_CheckConsistency()
//
// #define DEBUG_PYDICT
fn get_index_from_order(mp: *mut __SlateRecord_dictobject_h_33, i: i64) -> i32 {
    let array: *mut u8 = get_insertion_order_array(unsafe { (*mp).ma_values });
    (unsafe { *array.wrapping_offset(i as isize) }) as i32
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyDict_CheckConsistency(op: *mut _object, check_content: i32) -> i32 {
    CHECK__23!(PyDict_Check!(op), op);
    let mp: *mut __SlateRecord_dictobject_h_33 = op as *mut __SlateRecord_dictobject_h_33;
    let keys: *mut _dictkeysobject = unsafe { (*mp).ma_keys };
    let splitted: i32 = _PyDict_HasSplitTable!(mp) as i32;
    let usable: i64 = USABLE_FRACTION!(DK_SIZE!(keys));
    // In the free-threaded build, shared keys may be concurrently modified,
    //
    // so use atomic loads.
    let dk_usable: i64 = unsafe { (*keys).dk_usable };
    let dk_nentries: i64 = unsafe { (*keys).dk_nentries };
    CHECK__34!(
        0 <= unsafe { (*mp).ma_used } && (unsafe { (*mp).ma_used }) <= usable,
        op
    );
    CHECK__3!(0 <= dk_usable && dk_usable <= usable, op);
    CHECK__31!(0 <= dk_nentries && dk_nentries <= usable, op);
    CHECK__45!(dk_usable.wrapping_add(dk_nentries) <= usable, op);
    if splitted == 0 {
        // combined table
        CHECK__8!(((unsafe { (*keys).dk_kind }) as i32) != 2, op);
        CHECK__26!(
            (unsafe { (*keys).dk_refcnt }) == 1
                || keys
                    == unsafe {
                        core::ptr::addr_of_mut!(
                            *(core::ptr::addr_of_mut!(empty_keys_struct) as *mut _dictkeysobject)
                        )
                    },
            op
        );
    } else {
        CHECK__46!(((unsafe { (*keys).dk_kind }) as i32) == 2, op);
        CHECK__12!((unsafe { (*mp).ma_used }) <= 30, op);
        if (unsafe { (*(*mp).ma_values).embedded }) != 0 {
            CHECK__20!(((unsafe { (*(*mp).ma_values).embedded }) as i32) == 1, op);
            CHECK__24!(((unsafe { (*(*mp).ma_values).valid }) as i32) == 1, op);
        }
    }
    if check_content != 0 {
        {
            let mut i: i64 = 0;
            while i < DK_SIZE!(keys) {
                let ix: i64 = dictkeys_get_index(keys as *const _dictkeysobject, i);
                CHECK__18!((2i32.wrapping_neg() as i64) <= ix && ix <= usable, op);
                i = i.wrapping_add(1);
            }
        }
        if ((unsafe { (*keys).dk_kind }) as i32) == 0 {
            let entries: *mut __SlateRecord_pycore_dict_h_85 = DK_ENTRIES(keys);
            for i in 0..usable {
                let entry: *mut __SlateRecord_pycore_dict_h_85 =
                    entries.wrapping_offset(i as isize);
                let key: *mut _object = unsafe { (*entry).me_key };
                if !key.is_null() {
                    // test_dict fails if PyObject_Hash() is called again
                    CHECK__44!(
                        (unsafe { (*entry).me_hash }) != (1i32.wrapping_neg() as i64),
                        op
                    );
                    CHECK__54!(!unsafe { (*entry).me_value }.is_null(), op);
                    if PyUnicode_CheckExact!(key) != 0 {
                        let hash: i64 = unicode_get_hash(key);
                        CHECK__47!((unsafe { (*entry).me_hash }) == hash, op);
                    }
                }
            }
        } else {
            let entries: *mut __SlateRecord_pycore_dict_h_90 = DK_UNICODE_ENTRIES(keys);
            for i in 0..usable {
                let entry: *mut __SlateRecord_pycore_dict_h_90 =
                    entries.wrapping_offset(i as isize);
                let key: *mut _object = unsafe { (*entry).me_key };
                if !key.is_null() {
                    CHECK__49!(PyUnicode_CheckExact!(key), op);
                    let hash: i64 = unicode_get_hash(key);
                    CHECK__14!(hash != (1i32.wrapping_neg() as i64), op);
                    if splitted == 0 {
                        CHECK__51!(!unsafe { (*entry).me_value }.is_null(), op);
                    }
                }
                if splitted != 0 {
                    CHECK__28!(unsafe { (*entry).me_value }.is_null(), op);
                }
            }
        }
        if splitted != 0 {
            CHECK__7!((unsafe { (*mp).ma_used }) <= 30, op);
            // splitted table
            let mut duplicate_check: i32 = 0;
            let mut i: i64 = 0;
            while i < unsafe { (*mp).ma_used } {
                let index: i32 = get_index_from_order(mp, i);
                CHECK__33!(duplicate_check & 1i32 << index == 0, op);
                duplicate_check |= 1i32 << index;
                CHECK__16!(
                    !unsafe {
                        *((*(*mp).ma_values).values.as_mut_ptr() as *mut *mut _object)
                            .wrapping_offset(index as isize)
                    }
                    .is_null(),
                    op
                );
                i = i.wrapping_add(1);
            }
        }
    }
    1
}

fn new_keys_object(interp: *mut _is, log2_size: u8, unicode: bool) -> *mut _dictkeysobject {
    let mut usable: i64 = 0;
    let mut log2_bytes: i32 = 0;
    let entry_size: u64 = if unicode {
        core::mem::size_of::<__SlateRecord_pycore_dict_h_90>() as u64
    } else {
        core::mem::size_of::<__SlateRecord_pycore_dict_h_85>() as u64
    };
    usable = USABLE_FRACTION__2!(1u64 << (log2_size as i32)) as i64;
    if (log2_size as i32) < 8 {
        log2_bytes = log2_size as i32;
    } else {
        if (log2_size as i32) < 16 {
            log2_bytes = (log2_size as i32).wrapping_add(1);
        } else {
            if (log2_size as i32) >= 32 {
                log2_bytes = (log2_size as i32).wrapping_add(3);
            } else {
                log2_bytes = (log2_size as i32).wrapping_add(2);
            }
        }
    }
    let mut dk: *mut _dictkeysobject = core::ptr::null_mut::<_dictkeysobject>();
    if (log2_size as i32) == 3 && unicode {
        dk = _Py_FREELIST_POP_MEM!(dictkeys) as *mut _dictkeysobject;
    }
    if dk.is_null() {
        dk = (unsafe {
            PyMem_Malloc(
                32u64
                    .wrapping_add(1u64 << log2_bytes)
                    .wrapping_add(entry_size.wrapping_mul(usable as u64)),
            )
        }) as *mut _dictkeysobject;
        if dk.is_null() {
            unsafe { PyErr_NoMemory() };
            return core::ptr::null_mut::<_dictkeysobject>();
        }
    }
    unsafe {
        (*dk).dk_refcnt = 1;
    }
    unsafe {
        (*dk).dk_log2_size = log2_size;
    }
    unsafe {
        (*dk).dk_log2_index_bytes = log2_bytes as u8;
    }
    unsafe {
        (*dk).dk_kind = (if unicode { 1i32 } else { 0i32 }) as u8;
    }
    unsafe {
        (*dk).dk_nentries = 0;
    }
    unsafe {
        (*dk).dk_usable = usable;
    }
    unsafe {
        (*dk).dk_version = 0;
    }
    unsafe {
        core::ptr::write_bytes(
            (((core::ptr::addr_of_mut!((*dk).dk_indices) as *mut i8).wrapping_offset(0 as isize)
                as *mut ()) as *mut core::ffi::c_void) as *mut u8,
            (255i32 as i32) as u8,
            ((1u64 << log2_bytes) as usize) as usize,
        )
    };
    unsafe {
        core::ptr::write_bytes(
            (((core::ptr::addr_of_mut!((*dk).dk_indices) as *mut i8)
                .wrapping_offset((1u64 << log2_bytes) as isize) as *mut ())
                as *mut core::ffi::c_void) as *mut u8,
            (0i32 as i32) as u8,
            (entry_size.wrapping_mul(usable as u64) as usize) as usize,
        )
    };
    dk
}

fn free_keys_object(keys: *mut _dictkeysobject, use_qsbr: bool) {
    if (DK_LOG_SIZE!(keys) as i32) == 3 && ((unsafe { (*keys).dk_kind }) as i32) == 1 {
        _PyFreeList_Free(
            unsafe { core::ptr::addr_of_mut!((*_Py_freelists_GET()).dictkeys) },
            _PyObject_CAST__2!(keys) as *mut (),
            80i64,
            unsafe {
                core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
                    PyMem_Free as *const (),
                )
            },
        );
    } else {
        unsafe { PyMem_Free(keys as *mut ()) };
    }
}

fn values_size_from_count(count: u64) -> u64 {
    let suffix_size: u64 =
        _Py_SIZE_ROUND_UP__2!(count, core::mem::size_of::<*mut _object>() as u64);
    count
        .wrapping_add(1)
        .wrapping_mul(core::mem::size_of::<*mut _object>() as u64)
        .wrapping_add(suffix_size)
}

fn new_values(size: u64) -> *mut _dictvalues {
    let n: u64 = values_size_from_count(size);
    let res: *mut _dictvalues = (unsafe { PyMem_Malloc(n) }) as *mut _dictvalues;
    if res.is_null() {
        return core::ptr::null_mut::<_dictvalues>();
    }
    unsafe {
        (*res).embedded = 0;
    }
    unsafe {
        (*res).size = 0;
    }
    unsafe {
        (*res).capacity = size as u8;
    }
    res
}

fn free_values(values: *mut _dictvalues, use_qsbr: bool) {
    unsafe { PyMem_Free(values as *mut ()) };
}

/// Consumes a reference to the keys object
fn new_dict(
    interp: *mut _is,
    keys: *mut _dictkeysobject,
    values: *mut _dictvalues,
    used: i64,
    free_values_on_failure: i32,
) -> *mut _object {
    let mut mp: *mut __SlateRecord_dictobject_h_33 =
        _Py_FREELIST_POP!(__SlateRecord_dictobject_h_33, dicts);
    if mp.is_null() {
        mp = PyObject_GC_New!(__SlateRecord_dictobject_h_33, unsafe {
            core::ptr::addr_of_mut!(PyDict_Type)
        });
        if mp.is_null() {
            dictkeys_decref(interp, keys, false);
            if free_values_on_failure != 0 {
                free_values(values, false);
            }
            return core::ptr::null_mut::<_object>();
        }
    }
    unsafe {
        (*mp).ma_keys = keys;
    }
    unsafe {
        (*mp).ma_values = values;
    }
    unsafe {
        (*mp).ma_used = used;
    }
    unsafe {
        (*mp)._ma_watcher_tag = 0;
    }
    ASSERT_CONSISTENT!();
    _PyObject_GC_TRACK!(mp);
    mp as *mut _object
}

fn new_dict_with_shared_keys(interp: *mut _is, keys: *mut _dictkeysobject) -> *mut _object {
    let size: u64 = shared_keys_usable_size(keys);
    let values: *mut _dictvalues = new_values(size);
    if values.is_null() {
        return unsafe { PyErr_NoMemory() };
    }
    dictkeys_incref(keys);
    for i in 0..size {
        unsafe {
            *((*values).values.as_mut_ptr() as *mut *mut _object).wrapping_offset(i as isize) =
                core::ptr::null_mut::<_object>();
        }
    }
    new_dict(interp, keys, values, 0i64, 1i32)
}

fn clone_combined_dict_keys(orig: *mut __SlateRecord_dictobject_h_33) -> *mut _dictkeysobject {
    let keys_size: u64 = _PyDict_KeysSize(unsafe { (*orig).ma_keys });
    let keys: *mut _dictkeysobject = (unsafe { PyMem_Malloc(keys_size) }) as *mut _dictkeysobject;
    if keys.is_null() {
        unsafe { PyErr_NoMemory() };
        return core::ptr::null_mut::<_dictkeysobject>();
    }
    unsafe {
        core::ptr::copy_nonoverlapping(
            (((*orig).ma_keys as *const ()) as *const core::ffi::c_void) as *const u8,
            ((keys as *mut ()) as *mut core::ffi::c_void) as *mut u8,
            (keys_size as usize) as usize,
        )
    };
    // After copying key/value pairs, we need to incref all
    // keys and values and they are about to be co-owned by a
    // new dict object.
    let mut pkey: *mut *mut _object = unsafe { core::mem::zeroed() };
    let mut pvalue: *mut *mut _object = unsafe { core::mem::zeroed() };
    let offs: u64 = if DK_IS_UNICODE!(unsafe { (*orig).ma_keys }) {
        let ep0: *mut __SlateRecord_pycore_dict_h_90 = DK_UNICODE_ENTRIES(keys);
        pkey = unsafe { core::ptr::addr_of_mut!((*ep0).me_key) };
        pvalue = unsafe { core::ptr::addr_of_mut!((*ep0).me_value) };
        (core::mem::size_of::<__SlateRecord_pycore_dict_h_90>() as u64)
            / (core::mem::size_of::<*mut _object>() as u64)
    } else {
        let ep0: *mut __SlateRecord_pycore_dict_h_85 = DK_ENTRIES(keys);
        pkey = unsafe { core::ptr::addr_of_mut!((*ep0).me_key) };
        pvalue = unsafe { core::ptr::addr_of_mut!((*ep0).me_value) };
        (core::mem::size_of::<__SlateRecord_pycore_dict_h_85>() as u64)
            / (core::mem::size_of::<*mut _object>() as u64)
    };
    let n: i64 = unsafe { (*keys).dk_nentries };
    for i in 0..n {
        let value: *mut _object = unsafe { *pvalue };
        if !value.is_null() {
            Py_INCREF!(value);
            Py_INCREF!(unsafe { *pkey });
        }
        let __v0: *mut *mut _object = pvalue;
        pvalue = __v0.wrapping_offset(offs as isize);
        let __v2: *mut *mut _object = pkey;
        pkey = __v2.wrapping_offset(offs as isize);
    }
    // Since we copied the keys table we now have an extra reference
    // in the system.  Manually call increment _Py_RefTotal to signal that
    // we have it now; calling dictkeys_incref would be an error as
    // keys->dk_refcnt is already set to 1 (after memcpy).
    keys
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyDict_New() -> *mut _object {
    let interp: *mut _is = _PyInterpreterState_GET();
    // We don't incref Py_EMPTY_KEYS here because it is immortal.
    new_dict(
        interp,
        unsafe {
            core::ptr::addr_of_mut!(
                *(core::ptr::addr_of_mut!(empty_keys_struct) as *mut _dictkeysobject)
            )
        },
        core::ptr::null_mut::<_dictvalues>(),
        0i64,
        0i32,
    )
}

// Search index of hash table from offset of entry table
fn lookdict_index(k: *mut _dictkeysobject, hash: i64, index: i64) -> i64 {
    let mask: u64 = DK_MASK!(k) as u64;
    let mut perturb: u64 = hash as u64;
    let mut i: u64 = (hash as u64) & mask;
    loop {
        let ix: i64 = dictkeys_get_index(k as *const _dictkeysobject, i as i64);
        if ix == index {
            return i as i64;
        }
        if ix == (1i32.wrapping_neg() as i64) {
            return 1i32.wrapping_neg() as i64;
        }
        perturb >>= 5i32;
        i = mask & i.wrapping_mul(5).wrapping_add(perturb).wrapping_add(1);
    }
    Py_UNREACHABLE!();
    unsafe { core::mem::zeroed() }
}

#[inline(always)]
fn do_lookup(
    mp: *mut __SlateRecord_dictobject_h_33,
    dk: *mut _dictkeysobject,
    key: *mut _object,
    hash: i64,
    check_lookup: Option<
        unsafe extern "C-unwind" fn(
            *mut __SlateRecord_dictobject_h_33,
            *mut _dictkeysobject,
            *mut (),
            i64,
            *mut _object,
            i64,
        ) -> i32,
    >,
) -> i64 {
    let ep0: *mut () = _DK_ENTRIES(dk);
    let mask: u64 = DK_MASK!(dk) as u64;
    let mut perturb: u64 = hash as u64;
    let mut i: u64 = (hash as u64) & mask;
    let mut ix: i64 = 0;
    loop {
        ix = dictkeys_get_index(dk as *const _dictkeysobject, i as i64);
        if ix >= 0 {
            let cmp: i32 = unsafe { check_lookup.unwrap()(mp, dk, ep0, ix, key, hash) };
            if cmp < 0 {
                return cmp as i64;
            } else {
                if cmp != 0 {
                    return ix;
                }
            }
        } else {
            if ix == (1i32.wrapping_neg() as i64) {
                return 1i32.wrapping_neg() as i64;
            }
        }
        perturb >>= 5i32;
        i = mask & i.wrapping_mul(5).wrapping_add(perturb).wrapping_add(1);
        // Manual loop unrolling
        ix = dictkeys_get_index(dk as *const _dictkeysobject, i as i64);
        if ix >= 0 {
            let cmp: i32 = unsafe { check_lookup.unwrap()(mp, dk, ep0, ix, key, hash) };
            if cmp < 0 {
                return cmp as i64;
            } else {
                if cmp != 0 {
                    return ix;
                }
            }
        } else {
            if ix == (1i32.wrapping_neg() as i64) {
                return 1i32.wrapping_neg() as i64;
            }
        }
        perturb >>= 5i32;
        i = mask & i.wrapping_mul(5).wrapping_add(perturb).wrapping_add(1);
    }
    Py_UNREACHABLE!();
    unsafe { core::mem::zeroed() }
}

#[unsafe(link_section = ".text.slate_distinct.dictobject.compare_unicode_generic")]
extern "C-unwind" fn compare_unicode_generic(
    mp: *mut __SlateRecord_dictobject_h_33,
    dk: *mut _dictkeysobject,
    ep0: *mut (),
    ix: i64,
    key: *mut _object,
    hash: i64,
) -> i32 {
    let ep: *mut __SlateRecord_pycore_dict_h_90 =
        (ep0 as *mut __SlateRecord_pycore_dict_h_90).wrapping_offset(ix as isize);
    if unicode_get_hash(unsafe { (*ep).me_key }) == hash {
        let startkey: *mut _object = unsafe { (*ep).me_key };
        Py_INCREF!(startkey);
        let cmp: i32 = unsafe { PyObject_RichCompareBool(startkey, key, 2i32) };
        Py_DECREF!(startkey);
        if cmp < 0 {
            return 3i32.wrapping_neg();
        }
        if dk == unsafe { (*mp).ma_keys } && (unsafe { (*ep).me_key }) == startkey {
            return cmp;
        } else {
            // The dict was mutated, restart
            return 4i32.wrapping_neg();
        }
    }
    0
}

/// Search non-Unicode key from Unicode table
fn unicodekeys_lookup_generic(
    mp: *mut __SlateRecord_dictobject_h_33,
    dk: *mut _dictkeysobject,
    key: *mut _object,
    hash: i64,
) -> i64 {
    do_lookup(mp, dk, key, hash, Some(compare_unicode_generic))
}

#[unsafe(link_section = ".text.slate_distinct.dictobject.compare_unicode_unicode")]
extern "C-unwind" fn compare_unicode_unicode(
    mp: *mut __SlateRecord_dictobject_h_33,
    dk: *mut _dictkeysobject,
    ep0: *mut (),
    ix: i64,
    key: *mut _object,
    hash: i64,
) -> i32 {
    let ep: *mut __SlateRecord_pycore_dict_h_90 =
        (ep0 as *mut __SlateRecord_pycore_dict_h_90).wrapping_offset(ix as isize);
    let ep_key: *mut _object = unsafe { (*ep).me_key };
    let __v0: bool =
        ep_key == key || unicode_get_hash(ep_key) == hash && unicode_eq(ep_key, key) != 0;
    if __v0 {
        return 1;
    }
    0
}

fn unicodekeys_lookup_unicode(dk: *mut _dictkeysobject, key: *mut _object, hash: i64) -> i64 {
    do_lookup(
        core::ptr::null_mut::<__SlateRecord_dictobject_h_33>(),
        dk,
        key,
        hash,
        Some(compare_unicode_unicode),
    )
}

#[unsafe(link_section = ".text.slate_distinct.dictobject.compare_generic")]
extern "C-unwind" fn compare_generic(
    mp: *mut __SlateRecord_dictobject_h_33,
    dk: *mut _dictkeysobject,
    ep0: *mut (),
    ix: i64,
    key: *mut _object,
    hash: i64,
) -> i32 {
    let ep: *mut __SlateRecord_pycore_dict_h_85 =
        (ep0 as *mut __SlateRecord_pycore_dict_h_85).wrapping_offset(ix as isize);
    if (unsafe { (*ep).me_key }) == key {
        return 1;
    }
    if (unsafe { (*ep).me_hash }) == hash {
        let startkey: *mut _object = unsafe { (*ep).me_key };
        Py_INCREF!(startkey);
        let cmp: i32 = unsafe { PyObject_RichCompareBool(startkey, key, 2i32) };
        Py_DECREF!(startkey);
        if cmp < 0 {
            return 3i32.wrapping_neg();
        }
        if dk == unsafe { (*mp).ma_keys } && (unsafe { (*ep).me_key }) == startkey {
            return cmp;
        } else {
            // The dict was mutated, restart
            return 4i32.wrapping_neg();
        }
    }
    0
}

fn dictkeys_generic_lookup(
    mp: *mut __SlateRecord_dictobject_h_33,
    dk: *mut _dictkeysobject,
    key: *mut _object,
    hash: i64,
) -> i64 {
    do_lookup(mp, dk, key, hash, Some(compare_generic))
}

fn check_keys_unicode(dk: *mut _dictkeysobject, key: *mut _object) -> bool {
    PyUnicode_CheckExact!(key) != 0 && ((unsafe { (*dk).dk_kind }) as i32) != 0
}

fn hash_unicode_key(key: *mut _object) -> i64 {
    let mut hash: i64 = unicode_get_hash(key);
    if hash == (1i32.wrapping_neg() as i64) {
        hash = unsafe { unsafe { PyUnicode_Type.tp_hash }.unwrap()(key) };
    }
    hash
}

fn unicodekeys_lookup_split(dk: *mut _dictkeysobject, key: *mut _object, hash: i64) -> i64 {
    let mut ix: i64 = 0;
    ix = unicodekeys_lookup_unicode(dk, key, hash);
    ix
}

/// Lookup a string in a (all unicode) dict keys.
/// Returns DKIX_ERROR if key is not a string,
/// or if the dict keys is not all strings.
/// If the keys is present then return the index of key.
/// If the key is not present then return DKIX_EMPTY.
#[unsafe(no_mangle)]
extern "C-unwind" fn _PyDictKeys_StringLookup(dk: *mut _dictkeysobject, key: *mut _object) -> i64 {
    if !check_keys_unicode(dk, key) {
        return 3i32.wrapping_neg() as i64;
    }
    let hash: i64 = hash_unicode_key(key);
    unicodekeys_lookup_unicode(dk, key, hash)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyDictKeys_StringLookupAndVersion(
    dk: *mut _dictkeysobject,
    key: *mut _object,
    version: *mut u32,
) -> i64 {
    if !check_keys_unicode(dk, key) {
        return 3i32.wrapping_neg() as i64;
    }
    let mut ix: i64 = 0;
    let hash: i64 = hash_unicode_key(key);
    ix = unicodekeys_lookup_unicode(dk, key, hash);
    unsafe {
        *version = _PyDictKeys_GetVersionForCurrentState(_PyInterpreterState_GET(), dk);
    }
    ix
}

/// Like _PyDictKeys_StringLookup() but only works on split keys.  Note
/// that in free-threaded builds this locks the keys object as required.
#[unsafe(no_mangle)]
extern "C-unwind" fn _PyDictKeys_StringLookupSplit(
    dk: *mut _dictkeysobject,
    key: *mut _object,
) -> i64 {
    let mut hash: i64 = unicode_get_hash(key);
    if hash == (1i32.wrapping_neg() as i64) {
        hash = unsafe { unsafe { PyUnicode_Type.tp_hash }.unwrap()(key) };
        if hash == (1i32.wrapping_neg() as i64) {
            unsafe { PyErr_Clear() };
            return 3i32.wrapping_neg() as i64;
        }
    }
    unicodekeys_lookup_split(dk, key, hash)
}

/// The basic lookup function used by all operations.
/// This is based on Algorithm D from Knuth Vol. 3, Sec. 6.4.
/// Open addressing is preferred over chaining since the link overhead for
/// chaining would be substantial (100% with typical malloc overhead).
///
/// The initial probe index is computed as hash mod the table size. Subsequent
/// probe indices are computed as explained earlier.
///
/// All arithmetic on hash should ignore overflow.
///
/// _Py_dict_lookup() is general-purpose, and may return DKIX_ERROR if (and only if) a
/// comparison raises an exception.
/// When the key isn't found a DKIX_EMPTY is returned.
#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_dict_lookup(
    mp: *mut __SlateRecord_dictobject_h_33,
    key: *mut _object,
    hash: i64,
    value_addr: *mut *mut _object,
) -> i64 {
    let mut ix: i64 = unsafe { core::mem::zeroed() };
    let mut kind: u32 = unsafe { core::mem::zeroed() };
    let mut dk: *mut _dictkeysobject = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_0: {
            '__join_5: {
                '__join_6: {
                    loop {
                        dk = (*mp).ma_keys;
                        kind = (*dk).dk_kind as u32;
                        if (kind as u32) != 0 {
                            if PyUnicode_CheckExact!(key) != 0 {
                                break '__join_6;
                            } else {
                                ix = unicodekeys_lookup_generic(mp, dk, key, hash);
                                if ix != (4i32.wrapping_neg() as i64) {
                                    break '__join_5;
                                }
                            }
                        } else {
                            ix = dictkeys_generic_lookup(mp, dk, key, hash);
                            if ix != (4i32.wrapping_neg() as i64) {
                                break;
                            }
                        }
                    }
                    if ix >= 0 {
                        *value_addr = (*DK_ENTRIES(dk).wrapping_offset(ix as isize)).me_value;
                        break '__join_0;
                    } else {
                        *value_addr = core::ptr::null_mut::<_object>();
                        break '__join_0;
                    }
                }
                ix = unicodekeys_lookup_unicode(dk, key, hash);
            }
            if ix >= 0 {
                if (kind as u32) == 2 {
                    *value_addr = *((*(*mp).ma_values).values.as_mut_ptr() as *mut *mut _object)
                        .wrapping_offset(ix as isize);
                } else {
                    *value_addr = (*DK_UNICODE_ENTRIES(dk).wrapping_offset(ix as isize)).me_value;
                }
            } else {
                *value_addr = core::ptr::null_mut::<_object>();
            }
        }
        return ix;
    }
}

fn ensure_shared_on_resize(mp: *mut __SlateRecord_dictobject_h_33) {}

fn ensure_shared_on_keys_version_assignment(mp: *mut __SlateRecord_dictobject_h_33) {}

#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_dict_lookup_threadsafe(
    mp: *mut __SlateRecord_dictobject_h_33,
    key: *mut _object,
    hash: i64,
    value_addr: *mut *mut _object,
) -> i64 {
    let ix: i64 = _Py_dict_lookup(mp, key, hash, value_addr);
    Py_XNewRef__3!(unsafe { *value_addr });
    ix
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _Py_dict_lookup_threadsafe_stackref(
    mp: *mut __SlateRecord_dictobject_h_33,
    key: *mut _object,
    hash: i64,
    value_addr: *mut _PyStackRef,
) -> i64 {
    let mut val: *mut _object = unsafe { core::mem::zeroed() };
    let ix: i64 = _Py_dict_lookup(mp, key, hash, core::ptr::addr_of_mut!(val));
    if val.is_null() {
        unsafe {
            *value_addr = PyStackRef_NULL;
        }
    } else {
        unsafe {
            *value_addr = PyStackRef_FromPyObjectNew!(val);
        }
    }
    ix
}

/// Looks up the unicode key `key` in the dictionary. Note that `*method` may
///
/// already contain a valid value! See _PyObject_GetMethodStackRef().
#[unsafe(no_mangle)]
extern "C-unwind" fn _PyDict_GetMethodStackRef(
    mp: *mut __SlateRecord_dictobject_h_33,
    key: *mut _object,
    method: *mut _PyStackRef,
) -> i32 {
    let hash: i64 = hash_unicode_key(key);
    let mut obj: *mut _object = unsafe { core::mem::zeroed() };
    Py_INCREF__2!(mp);
    let ix: i64 = _Py_dict_lookup_threadsafe(mp, key, hash, core::ptr::addr_of_mut!(obj));
    Py_DECREF__2!(mp);
    if ix == (3i32.wrapping_neg() as i64) {
        PyStackRef_CLEAR!(*method);
        return 1i32.wrapping_neg();
    } else {
        if ix >= 0 && !obj.is_null() {
            PyStackRef_XSETREF!(*method, PyStackRef_FromPyObjectSteal(obj));
            return 1;
        }
    }
    return 0; // not found
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyDict_HasOnlyStringKeys(dict: *mut _object) -> i32 {
    let mut pos: i64 = 0;
    let mut key: *mut _object = unsafe { core::mem::zeroed() };
    let mut value: *mut _object = unsafe { core::mem::zeroed() };
    // Shortcut
    if ((unsafe { (*(*(dict as *mut __SlateRecord_dictobject_h_33)).ma_keys).dk_kind }) as i32) != 0
    {
        return 1;
    }
    while PyDict_Next(
        dict,
        core::ptr::addr_of_mut!(pos),
        core::ptr::addr_of_mut!(key),
        core::ptr::addr_of_mut!(value),
    ) != 0
    {
        if PyUnicode_Check!(key) == 0 {
            return 0;
        }
    }
    1
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyDict_EnablePerThreadRefcounting(op: *mut _object) {}

fn is_unusable_slot(ix: i64) -> i32 {
    (ix >= 0) as i32
}

/// Internal function to find slot for an item from its hash
/// when it is known that the key is not present in the dict.
fn find_empty_slot(keys: *mut _dictkeysobject, hash: i64) -> i64 {
    let mask: u64 = DK_MASK!(keys) as u64;
    let mut i: u64 = (hash as u64) & mask;
    let mut ix: i64 = dictkeys_get_index(keys as *const _dictkeysobject, i as i64);
    let mut perturb: u64 = hash as u64;
    while is_unusable_slot(ix) != 0 {
        perturb >>= 5i32;
        i = i.wrapping_mul(5).wrapping_add(perturb).wrapping_add(1) & mask;
        ix = dictkeys_get_index(keys as *const _dictkeysobject, i as i64);
    }
    i as i64
}

fn insertion_resize(interp: *mut _is, mp: *mut __SlateRecord_dictobject_h_33, unicode: i32) -> i32 {
    dictresize(
        interp,
        mp,
        calculate_log2_keysize(GROWTH_RATE!(mp)),
        unicode,
    )
}

fn insert_combined_dict(
    interp: *mut _is,
    mp: *mut __SlateRecord_dictobject_h_33,
    hash: i64,
    key: *mut _object,
    value: *mut _object,
) -> i32 {
    // gh-140551: If dict was cleared in _Py_dict_lookup,
    //
    // we have to resize one more time to force general key kind.
    let __v0: bool = DK_IS_UNICODE!(unsafe { (*mp).ma_keys }) && PyUnicode_CheckExact!(key) == 0;
    if __v0 {
        if insertion_resize(interp, mp, 0i32) < 0 {
            return 1i32.wrapping_neg();
        }
    }
    if (unsafe { (*(*mp).ma_keys).dk_usable }) <= 0 {
        // Need to resize.
        if insertion_resize(interp, mp, 1i32) < 0 {
            return 1i32.wrapping_neg();
        }
    }
    _PyDict_NotifyEvent(interp, 0u32, mp, key, value);
    FT_ATOMIC_STORE_UINT32_RELAXED!((*unsafe { (*mp).ma_keys }).dk_version, 0i32);
    let hashpos: i64 = find_empty_slot(unsafe { (*mp).ma_keys }, hash);
    dictkeys_set_index(unsafe { (*mp).ma_keys }, hashpos, unsafe {
        (*(*mp).ma_keys).dk_nentries
    });
    if DK_IS_UNICODE!(unsafe { (*mp).ma_keys }) {
        let ep: *mut __SlateRecord_pycore_dict_h_90 = DK_UNICODE_ENTRIES(unsafe { (*mp).ma_keys })
            .wrapping_offset((unsafe { (*(*mp).ma_keys).dk_nentries }) as isize);
        STORE_KEY__2!(ep, key);
        STORE_VALUE!(ep, value);
    } else {
        let ep: *mut __SlateRecord_pycore_dict_h_85 = DK_ENTRIES(unsafe { (*mp).ma_keys })
            .wrapping_offset((unsafe { (*(*mp).ma_keys).dk_nentries }) as isize);
        STORE_KEY__2!(ep, key);
        STORE_VALUE!(ep, value);
        STORE_HASH__2!(ep, hash);
    }
    STORE_KEYS_USABLE!(
        unsafe { (*mp).ma_keys },
        unsafe { (*(*mp).ma_keys).dk_usable }.wrapping_sub(1)
    );
    STORE_KEYS_NENTRIES!(
        unsafe { (*mp).ma_keys },
        unsafe { (*(*mp).ma_keys).dk_nentries }.wrapping_add(1)
    );
    0
}

fn insert_split_key(keys: *mut _dictkeysobject, key: *mut _object, hash: i64) -> i64 {
    let mut ix: i64 = 0;
    ix = unicodekeys_lookup_unicode(keys, key, hash);
    if ix == (1i32.wrapping_neg() as i64) && (unsafe { (*keys).dk_usable }) > 0 {
        // Insert into new slot
        FT_ATOMIC_STORE_UINT32_RELAXED!((*keys).dk_version, 0i32);
        let hashpos: i64 = find_empty_slot(keys, hash);
        ix = unsafe { (*keys).dk_nentries };
        dictkeys_set_index(keys, hashpos, ix);
        let ep: *mut __SlateRecord_pycore_dict_h_90 =
            DK_UNICODE_ENTRIES(keys).wrapping_offset(ix as isize);
        STORE_SHARED_KEY!((*ep).me_key, Py_NewRef!(key));
        split_keys_entry_added(keys);
    }
    ix
}

fn insert_split_value(
    interp: *mut _is,
    mp: *mut __SlateRecord_dictobject_h_33,
    key: *mut _object,
    value: *mut _object,
    ix: i64,
) {
    let old_value: *mut _object = unsafe {
        *((*(*mp).ma_values).values.as_mut_ptr() as *mut *mut _object).wrapping_offset(ix as isize)
    };
    if old_value.is_null() {
        _PyDict_NotifyEvent(interp, 0u32, mp, key, value);
        STORE_SPLIT_VALUE!(mp, ix, Py_NewRef!(value));
        _PyDictValues_AddToInsertionOrder(unsafe { (*mp).ma_values }, ix);
        STORE_USED!(mp, unsafe { (*mp).ma_used }.wrapping_add(1));
    } else {
        _PyDict_NotifyEvent(interp, 1u32, mp, key, value);
        STORE_SPLIT_VALUE!(mp, ix, Py_NewRef!(value));
        // old_value should be DECREFed after GC track checking is done, if not, it could raise a segmentation fault,
        //
        // when dict only holds the strong reference to value in ep->me_value.
        Py_DECREF!(old_value);
    }
    ASSERT_CONSISTENT!();
}

/// Internal routine to insert a new item into the table.
/// Used both by the internal resize routine and by the public insert routine.
/// Returns -1 if an error occurred, or 0 on success.
/// Consumes key and value references.
fn insertdict(
    interp: *mut _is,
    mp: *mut __SlateRecord_dictobject_h_33,
    key: *mut _object,
    hash: i64,
    value: *mut _object,
) -> i32 {
    let mut __v2: bool = unsafe { core::mem::zeroed() };
    let mut ix: i64 = unsafe { core::mem::zeroed() };
    let mut old_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        old_value = core::ptr::null_mut::<_object>();
        if _PyDict_HasSplitTable!(mp) {
            __v2 = PyUnicode_CheckExact!(key) != 0;
        } else {
            __v2 = false;
        }
        '__join_0: {
            if __v2 {
                ix = insert_split_key((*mp).ma_keys, key, hash);
                if ix != (1i32.wrapping_neg() as i64) {
                    insert_split_value(interp, mp, key, value, ix);
                    Py_DECREF!(key);
                    Py_DECREF!(value);
                    return 0;
                }
            // No space in shared keys. Go to insert_combined_dict() below.
            } else {
                ix = _Py_dict_lookup(mp, key, hash, core::ptr::addr_of_mut!(old_value));
                if ix == (3i32.wrapping_neg() as i64) {
                    break '__join_0;
                }
            }
            if old_value.is_null() {
                // insert_combined_dict() will convert from non DICT_KEYS_GENERAL table
                //
                // into DICT_KEYS_GENERAL table if key is not Unicode.
                //
                // We don't convert it before _Py_dict_lookup because non-Unicode key
                //
                // may change generic table into Unicode table.
                //
                //
                //
                // NOTE: ix may not be DKIX_EMPTY because split table may have key
                //
                // without value.
                if insert_combined_dict(interp, mp, hash, key, value) >= 0 {
                    STORE_USED!(mp, unsafe { (*mp).ma_used }.wrapping_add(1));
                    ASSERT_CONSISTENT!();
                    return 0;
                }
            } else {
                if old_value != value {
                    _PyDict_NotifyEvent(interp, 1u32, mp, key, value);
                    if DK_IS_UNICODE!(unsafe { (*mp).ma_keys }) {
                        if _PyDict_HasSplitTable!(mp) {
                            STORE_SPLIT_VALUE!(mp, ix, value);
                        } else {
                            let ep: *mut __SlateRecord_pycore_dict_h_90 =
                                DK_UNICODE_ENTRIES((*mp).ma_keys).wrapping_offset(ix as isize);
                            STORE_VALUE!(ep, value);
                        }
                    } else {
                        let ep: *mut __SlateRecord_pycore_dict_h_85 =
                            DK_ENTRIES((*mp).ma_keys).wrapping_offset(ix as isize);
                        STORE_VALUE!(ep, value);
                    }
                }
                Py_XDECREF!(old_value); // which **CAN** re-enter (see issue #22653)
                ASSERT_CONSISTENT!();
                Py_DECREF!(key);
                return 0;
            }
        }
        Py_DECREF!(value);
        Py_DECREF!(key);
        return 1i32.wrapping_neg();
    }
}

/// Same as insertdict but specialized for ma_keys == Py_EMPTY_KEYS.
///
/// Consumes key and value references.
fn insert_to_emptydict(
    interp: *mut _is,
    mp: *mut __SlateRecord_dictobject_h_33,
    key: *mut _object,
    hash: i64,
    value: *mut _object,
) -> i32 {
    let unicode: i32 = PyUnicode_CheckExact!(key);
    let newkeys: *mut _dictkeysobject = new_keys_object(interp, 3u8, unicode != 0);
    if newkeys.is_null() {
        Py_DECREF!(key);
        Py_DECREF!(value);
        return 1i32.wrapping_neg();
    }
    _PyDict_NotifyEvent(interp, 0u32, mp, key, value);
    // We don't decref Py_EMPTY_KEYS here because it is immortal.
    let hashpos: u64 = (hash as u64) & 7u64;
    dictkeys_set_index(newkeys, hashpos as i64, 0i64);
    if unicode != 0 {
        let ep: *mut __SlateRecord_pycore_dict_h_90 = DK_UNICODE_ENTRIES(newkeys);
        unsafe {
            (*ep).me_key = key;
        }
        STORE_VALUE!(ep, value);
    } else {
        let ep: *mut __SlateRecord_pycore_dict_h_85 = DK_ENTRIES(newkeys);
        unsafe {
            (*ep).me_key = key;
        }
        unsafe {
            (*ep).me_hash = hash;
        }
        STORE_VALUE!(ep, value);
    }
    STORE_USED!(mp, unsafe { (*mp).ma_used }.wrapping_add(1));
    let __v1: i64 = unsafe { (*newkeys).dk_usable };
    unsafe {
        (*newkeys).dk_usable = __v1.wrapping_sub(1);
    }
    let __v4: i64 = unsafe { (*newkeys).dk_nentries };
    unsafe {
        (*newkeys).dk_nentries = __v4.wrapping_add(1);
    }
    // We store the keys last so no one can see them in a partially inconsistent
    //
    // state so that we don't need to switch the keys to being shared yet for
    //
    // the case where we're inserting from the non-owner thread.  We don't use
    //
    // set_keys here because the transition from empty to non-empty is safe
    //
    // as the empty keys will never be freed.
    FT_ATOMIC_STORE_PTR_RELEASE!((*mp).ma_keys, newkeys);
    0
}

/// Internal routine used by dictresize() to build a hashtable of entries.
fn build_indices_generic(
    keys: *mut _dictkeysobject,
    mut ep: *mut __SlateRecord_pycore_dict_h_85,
    n: i64,
) {
    let mask: u64 = DK_MASK!(keys) as u64;
    let mut ix: i64 = 0;
    while ix != n {
        let hash: i64 = unsafe { (*ep).me_hash };
        let mut i: u64 = (hash as u64) & mask;
        let mut perturb: u64 = hash as u64;
        while dictkeys_get_index(keys as *const _dictkeysobject, i as i64)
            != (1i32.wrapping_neg() as i64)
        {
            perturb >>= 5i32;
            i = mask & i.wrapping_mul(5).wrapping_add(perturb).wrapping_add(1);
        }
        dictkeys_set_index(keys, i as i64, ix);
        ix = ix.wrapping_add(1);
        let __v6: *mut __SlateRecord_pycore_dict_h_85 = ep;
        ep = __v6.wrapping_offset(1 as isize);
    }
}

fn build_indices_unicode(
    keys: *mut _dictkeysobject,
    mut ep: *mut __SlateRecord_pycore_dict_h_90,
    n: i64,
) {
    let mask: u64 = DK_MASK!(keys) as u64;
    let mut ix: i64 = 0;
    while ix != n {
        let hash: i64 = unicode_get_hash(unsafe { (*ep).me_key });
        let mut i: u64 = (hash as u64) & mask;
        let mut perturb: u64 = hash as u64;
        while dictkeys_get_index(keys as *const _dictkeysobject, i as i64)
            != (1i32.wrapping_neg() as i64)
        {
            perturb >>= 5i32;
            i = mask & i.wrapping_mul(5).wrapping_add(perturb).wrapping_add(1);
        }
        dictkeys_set_index(keys, i as i64, ix);
        ix = ix.wrapping_add(1);
        let __v6: *mut __SlateRecord_pycore_dict_h_90 = ep;
        ep = __v6.wrapping_offset(1 as isize);
    }
}

fn invalidate_and_clear_inline_values(values: *mut _dictvalues) {
    FT_ATOMIC_STORE_UINT8!((*values).valid, 0i32);
    let mut i: i32 = 0;
    while i < ((unsafe { (*values).capacity }) as i32) {
        FT_ATOMIC_STORE_PTR_RELEASE__2!(
            *unsafe { (*values).values.as_mut_ptr() as *mut *mut _object }
                .wrapping_offset(i as isize)
        );
        i = i.wrapping_add(1);
    }
}

/// Restructure the table by allocating a new table and reinserting all
/// items again.  When entries have been deleted, the new table may
/// actually be smaller than the old one.
/// If a table is split (its keys and hashes are shared, its values are not),
/// then the values are temporarily copied into the table, it is resized as
/// a combined table, then the me_value slots in the old table are NULLed out.
/// After resizing, a table is always combined.
///
/// This function supports:
///  - Unicode split -> Unicode combined or Generic
///  - Unicode combined -> Unicode combined or Generic
///  - Generic -> Generic
fn dictresize(
    interp: *mut _is,
    mp: *mut __SlateRecord_dictobject_h_33,
    log2_newsize: u8,
    mut unicode: i32,
) -> i32 {
    if (log2_newsize as i32) >= 64i32 {
        unsafe { PyErr_NoMemory() };
        return 1i32.wrapping_neg();
    }
    let oldkeys: *mut _dictkeysobject = unsafe { (*mp).ma_keys };
    let oldvalues: *mut _dictvalues = unsafe { (*mp).ma_values };
    if !DK_IS_UNICODE!(oldkeys) {
        unicode = 0;
    }
    ensure_shared_on_resize(mp);
    // NOTE: Current odict checks mp->ma_keys to detect resize happen.
    // So we can't reuse oldkeys even if oldkeys->dk_size == newsize.
    // TODO: Try reusing oldkeys when reimplement odict.
    // Allocate a new table.
    let newkeys: *mut _dictkeysobject = new_keys_object(interp, log2_newsize, unicode != 0);
    if newkeys.is_null() {
        return 1i32.wrapping_neg();
    }
    // New table must be large enough.
    let numentries: i64 = unsafe { (*mp).ma_used };
    if !oldvalues.is_null() {
        let oldentries: *mut __SlateRecord_pycore_dict_h_90 = DK_UNICODE_ENTRIES(oldkeys);
        // Convert split table into new combined table.
        // We must incref keys; we can transfer values.
        if ((unsafe { (*newkeys).dk_kind }) as i32) == 0 {
            // split -> generic
            let newentries: *mut __SlateRecord_pycore_dict_h_85 = DK_ENTRIES(newkeys);
            for i in 0..numentries {
                let index: i32 = get_index_from_order(mp, i);
                let ep: *mut __SlateRecord_pycore_dict_h_90 =
                    oldentries.wrapping_offset(index as isize);
                unsafe {
                    (*newentries.wrapping_offset(i as isize)).me_key =
                        Py_NewRef!(unsafe { (*ep).me_key });
                }
                unsafe {
                    (*newentries.wrapping_offset(i as isize)).me_hash =
                        unicode_get_hash((*ep).me_key);
                }
                unsafe {
                    (*newentries.wrapping_offset(i as isize)).me_value =
                        *((*oldvalues).values.as_mut_ptr() as *mut *mut _object)
                            .wrapping_offset(index as isize);
                }
            }
            build_indices_generic(newkeys, newentries, numentries);
        } else {
            // split -> combined unicode
            let newentries: *mut __SlateRecord_pycore_dict_h_90 = DK_UNICODE_ENTRIES(newkeys);
            for i in 0..numentries {
                let index: i32 = get_index_from_order(mp, i);
                let ep: *mut __SlateRecord_pycore_dict_h_90 =
                    oldentries.wrapping_offset(index as isize);
                unsafe {
                    (*newentries.wrapping_offset(i as isize)).me_key =
                        Py_NewRef!(unsafe { (*ep).me_key });
                }
                unsafe {
                    (*newentries.wrapping_offset(i as isize)).me_value =
                        *((*oldvalues).values.as_mut_ptr() as *mut *mut _object)
                            .wrapping_offset(index as isize);
                }
            }
            build_indices_unicode(newkeys, newentries, numentries);
        }
        set_keys(mp, newkeys);
        dictkeys_decref(interp, oldkeys, IS_DICT_SHARED!() != 0);
        set_values(mp, core::ptr::null_mut::<_dictvalues>());
        if (unsafe { (*oldvalues).embedded }) != 0 {
            invalidate_and_clear_inline_values(oldvalues);
        } else {
            free_values(oldvalues, IS_DICT_SHARED!() != 0);
        }
    } else {
        // oldkeys is combined.
        if ((unsafe { (*oldkeys).dk_kind }) as i32) == 0 {
            // generic -> generic
            let oldentries: *mut __SlateRecord_pycore_dict_h_85 = DK_ENTRIES(oldkeys);
            let newentries: *mut __SlateRecord_pycore_dict_h_85 = DK_ENTRIES(newkeys);
            if (unsafe { (*oldkeys).dk_nentries }) == numentries {
                unsafe {
                    core::ptr::copy_nonoverlapping(
                        ((oldentries as *const ()) as *const core::ffi::c_void) as *const u8,
                        ((newentries as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                        ((numentries as u64).wrapping_mul(core::mem::size_of::<
                            __SlateRecord_pycore_dict_h_85,
                        >() as u64) as usize) as usize,
                    )
                };
            } else {
                let mut ep: *mut __SlateRecord_pycore_dict_h_85 = oldentries;
                for i in 0..numentries {
                    while unsafe { (*ep).me_value }.is_null() {
                        let __v2: *mut __SlateRecord_pycore_dict_h_85 = ep;
                        ep = __v2.wrapping_offset(1 as isize);
                    }
                    let __v5: *mut __SlateRecord_pycore_dict_h_85 = ep;
                    ep = __v5.wrapping_offset(1 as isize);
                    unsafe {
                        *newentries.wrapping_offset(i as isize) = *__v5;
                    }
                }
            }
            build_indices_generic(newkeys, newentries, numentries);
        } else {
            // oldkeys is combined unicode
            let oldentries: *mut __SlateRecord_pycore_dict_h_90 = DK_UNICODE_ENTRIES(oldkeys);
            if unicode != 0 {
                // combined unicode -> combined unicode
                let newentries: *mut __SlateRecord_pycore_dict_h_90 = DK_UNICODE_ENTRIES(newkeys);
                if (unsafe { (*oldkeys).dk_nentries }) == numentries
                    && ((unsafe { (*(*mp).ma_keys).dk_kind }) as i32) == 1
                {
                    unsafe {
                        core::ptr::copy_nonoverlapping(
                            ((oldentries as *const ()) as *const core::ffi::c_void) as *const u8,
                            ((newentries as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                            ((numentries as u64)
                                .wrapping_mul(
                                    core::mem::size_of::<__SlateRecord_pycore_dict_h_90>() as u64,
                                ) as usize) as usize,
                        )
                    };
                } else {
                    let mut ep: *mut __SlateRecord_pycore_dict_h_90 = oldentries;
                    for i in 0..numentries {
                        while unsafe { (*ep).me_value }.is_null() {
                            let __v8: *mut __SlateRecord_pycore_dict_h_90 = ep;
                            ep = __v8.wrapping_offset(1 as isize);
                        }
                        let __v11: *mut __SlateRecord_pycore_dict_h_90 = ep;
                        ep = __v11.wrapping_offset(1 as isize);
                        unsafe {
                            *newentries.wrapping_offset(i as isize) = *__v11;
                        }
                    }
                }
                build_indices_unicode(newkeys, newentries, numentries);
            } else {
                // combined unicode -> generic
                let newentries: *mut __SlateRecord_pycore_dict_h_85 = DK_ENTRIES(newkeys);
                let mut ep: *mut __SlateRecord_pycore_dict_h_90 = oldentries;
                for i in 0..numentries {
                    while unsafe { (*ep).me_value }.is_null() {
                        let __v14: *mut __SlateRecord_pycore_dict_h_90 = ep;
                        ep = __v14.wrapping_offset(1 as isize);
                    }
                    unsafe {
                        (*newentries.wrapping_offset(i as isize)).me_key = (*ep).me_key;
                    }
                    unsafe {
                        (*newentries.wrapping_offset(i as isize)).me_hash =
                            unicode_get_hash((*ep).me_key);
                    }
                    unsafe {
                        (*newentries.wrapping_offset(i as isize)).me_value = (*ep).me_value;
                    }
                    let __v17: *mut __SlateRecord_pycore_dict_h_90 = ep;
                    ep = __v17.wrapping_offset(1 as isize);
                }
                build_indices_generic(newkeys, newentries, numentries);
            }
        }
        set_keys(mp, newkeys);
        if oldkeys
            != unsafe {
                core::ptr::addr_of_mut!(
                    *(core::ptr::addr_of_mut!(empty_keys_struct) as *mut _dictkeysobject)
                )
            }
        {
            free_keys_object(oldkeys, IS_DICT_SHARED!() != 0);
        }
    }
    STORE_KEYS_USABLE!(
        unsafe { (*mp).ma_keys },
        unsafe { (*(*mp).ma_keys).dk_usable }.wrapping_sub(numentries)
    );
    STORE_KEYS_NENTRIES!(unsafe { (*mp).ma_keys }, numentries);
    ASSERT_CONSISTENT!();
    0
}

fn dict_new_presized(interp: *mut _is, minused: i64, unicode: bool) -> *mut _object {
    let log2_max_presize: u8 = 17;
    let max_presize: i64 = 1i64 << (log2_max_presize as i32);
    let mut log2_newsize: u8 = 0;
    if minused <= (USABLE_FRACTION__3!(8i32) as i64) {
        return PyDict_New();
    }
    // There are no strict guarantee that returned dict can contain minused
    // items without resize.  So we create medium size dict instead of very
    // large dict or MemoryError.
    if minused > USABLE_FRACTION!(max_presize) {
        log2_newsize = log2_max_presize;
    } else {
        log2_newsize = estimate_log2_keysize(minused);
    }
    let new_keys: *mut _dictkeysobject = new_keys_object(interp, log2_newsize, unicode);
    if new_keys.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    new_dict(
        interp,
        new_keys,
        core::ptr::null_mut::<_dictvalues>(),
        0i64,
        0i32,
    )
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyDict_NewPresized(minused: i64) -> *mut _object {
    let interp: *mut _is = _PyInterpreterState_GET();
    dict_new_presized(interp, minused, false)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyDict_FromItems(
    keys: *const *mut _object,
    keys_offset: i64,
    mut vs: *const *mut _object,
    values_offset: i64,
    length: i64,
) -> *mut _object {
    let mut unicode: bool = true;
    let mut ks: *const *mut _object = keys;
    let interp: *mut _is = _PyInterpreterState_GET();
    for i in 0..length {
        if PyUnicode_CheckExact!(unsafe { *ks }) == 0 {
            unicode = false;
            break;
        }
        let __v1: *const *mut _object = ks;
        ks = __v1.wrapping_offset(keys_offset as isize);
    }
    let dict: *mut _object = dict_new_presized(interp, length, unicode);
    if dict.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    ks = keys;
    for i in 0..length {
        let key: *mut _object = unsafe { *ks };
        let value: *mut _object = unsafe { *vs };
        if setitem_lock_held(dict as *mut __SlateRecord_dictobject_h_33, key, value) < 0 {
            Py_DECREF!(dict);
            return core::ptr::null_mut::<_object>();
        }
        let __v3: *const *mut _object = ks;
        ks = __v3.wrapping_offset(keys_offset as isize);
        let __v5: *const *mut _object = vs;
        vs = __v5.wrapping_offset(values_offset as isize);
    }
    dict
}

/// Note that, for historical reasons, PyDict_GetItem() suppresses all errors
/// that may occur (originally dicts supported only string keys, and exceptions
/// weren't possible).  So, while the original intent was that a NULL return
/// meant the key wasn't present, in reality it can mean that, or that an error
/// (suppressed) occurred while computing the key's hash, or that some error
/// (suppressed) occurred when comparing keys in the dict's internal probe
/// sequence.  A nasty example of the latter is when a Python-coded comparison
/// function hits a stack-depth error, which can cause this to return NULL
/// even if the key is present.
fn dict_getitem(op: *mut _object, key: *mut _object, warnmsg: *const i8) -> *mut _object {
    if PyDict_Check!(op) == 0 {
        return core::ptr::null_mut::<_object>();
    }
    let mp: *mut __SlateRecord_dictobject_h_33 = op as *mut __SlateRecord_dictobject_h_33;
    let hash: i64 = _PyObject_HashFast(key);
    if hash == (1i32.wrapping_neg() as i64) {
        unsafe { PyErr_FormatUnraisable(warnmsg) };
        return core::ptr::null_mut::<_object>();
    }
    let tstate: *mut _ts = _PyThreadState_GET();
    // Preserve the existing exception
    let mut value: *mut _object = unsafe { core::mem::zeroed() };
    let mut ix: i64 = 0;
    let exc: *mut _object = unsafe { _PyErr_GetRaisedException(tstate) };
    ix = _Py_dict_lookup(mp, key, hash, core::ptr::addr_of_mut!(value));
    // Ignore any exception raised by the lookup
    let exc2: *mut _object = _PyErr_Occurred(tstate);
    let __v0: bool =
        !exc2.is_null() && (unsafe { PyErr_GivenExceptionMatches(exc2, PyExc_KeyError) }) == 0;
    if __v0 {
        unsafe { PyErr_FormatUnraisable(warnmsg) };
    }
    unsafe { _PyErr_SetRaisedException(tstate, exc) };
    return value; // borrowed reference
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyDict_GetItem(op: *mut _object, key: *mut _object) -> *mut _object {
    dict_getitem(op, key, c"Exception ignored in PyDict_GetItem(); consider using PyDict_GetItemRef() or PyDict_GetItemWithError()".as_ptr())
}

fn dict_unhashable_type(key: *mut _object) {
    let exc: *mut _object = unsafe { PyErr_GetRaisedException() };
    if Py_IS_TYPE!(exc, (unsafe { PyExc_TypeError }) as *mut _typeobject) == 0 {
        unsafe { PyErr_SetRaisedException(exc) };
        return;
    }
    unsafe {
        PyErr_Format(
            PyExc_TypeError,
            c"cannot use '%T' as a dict key (%S)".as_ptr(),
            key,
            exc,
        )
    };
    Py_DECREF!(exc);
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyDict_LookupIndex(
    mp: *mut __SlateRecord_dictobject_h_33,
    key: *mut _object,
) -> i64 {
    // TODO: Thread safety
    let mut value: *mut _object = unsafe { core::mem::zeroed() };
    let hash: i64 = _PyObject_HashFast(key);
    if hash == (1i32.wrapping_neg() as i64) {
        dict_unhashable_type(key);
        return 1i32.wrapping_neg() as i64;
    }
    _Py_dict_lookup(mp, key, hash, core::ptr::addr_of_mut!(value))
}

/// Same as PyDict_GetItemWithError() but with hash supplied by caller.
/// This returns NULL *with* an exception set if an exception occurred.
/// It returns NULL *without* an exception set if the key wasn't present.
#[unsafe(no_mangle)]
extern "C-unwind" fn _PyDict_GetItem_KnownHash(
    op: *mut _object,
    key: *mut _object,
    hash: i64,
) -> *mut _object {
    let mut ix: i64 = 0;
    let mp: *mut __SlateRecord_dictobject_h_33 = op as *mut __SlateRecord_dictobject_h_33;
    let mut value: *mut _object = unsafe { core::mem::zeroed() };
    if PyDict_Check!(op) == 0 {
        PyErr_BadInternalCall__26!();
        return core::ptr::null_mut::<_object>();
    }
    ix = _Py_dict_lookup(mp, key, hash, core::ptr::addr_of_mut!(value));
    return value; // borrowed reference
}

/// Gets an item and provides a new reference if the value is present.
/// Returns 1 if the key is present, 0 if the key is missing, and -1 if an
/// exception occurred.
#[unsafe(no_mangle)]
extern "C-unwind" fn _PyDict_GetItemRef_KnownHash_LockHeld(
    op: *mut __SlateRecord_dictobject_h_33,
    key: *mut _object,
    hash: i64,
    result: *mut *mut _object,
) -> i32 {
    let mut value: *mut _object = unsafe { core::mem::zeroed() };
    let ix: i64 = _Py_dict_lookup(op, key, hash, core::ptr::addr_of_mut!(value));
    if ix == (3i32.wrapping_neg() as i64) {
        unsafe {
            *result = core::ptr::null_mut::<_object>();
        }
        return 1i32.wrapping_neg();
    }
    if value.is_null() {
        unsafe {
            *result = core::ptr::null_mut::<_object>();
        }
        return 0; // missing key
    }
    unsafe {
        *result = Py_NewRef!(value);
    }
    return 1; // key is present
}

/// Gets an item and provides a new reference if the value is present.
/// Returns 1 if the key is present, 0 if the key is missing, and -1 if an
/// exception occurred.
#[unsafe(no_mangle)]
extern "C-unwind" fn _PyDict_GetItemRef_KnownHash(
    op: *mut __SlateRecord_dictobject_h_33,
    key: *mut _object,
    hash: i64,
    result: *mut *mut _object,
) -> i32 {
    let mut value: *mut _object = unsafe { core::mem::zeroed() };
    let ix: i64 = _Py_dict_lookup(op, key, hash, core::ptr::addr_of_mut!(value));
    if ix == (3i32.wrapping_neg() as i64) {
        unsafe {
            *result = core::ptr::null_mut::<_object>();
        }
        return 1i32.wrapping_neg();
    }
    if value.is_null() {
        unsafe {
            *result = core::ptr::null_mut::<_object>();
        }
        return 0; // missing key
    }
    unsafe {
        *result = Py_NewRef!(value);
    }
    return 1; // key is present
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyDict_GetItemRef(
    op: *mut _object,
    key: *mut _object,
    result: *mut *mut _object,
) -> i32 {
    if PyDict_Check!(op) == 0 {
        PyErr_BadInternalCall__36!();
        unsafe {
            *result = core::ptr::null_mut::<_object>();
        }
        return 1i32.wrapping_neg();
    }
    let hash: i64 = _PyObject_HashFast(key);
    if hash == (1i32.wrapping_neg() as i64) {
        dict_unhashable_type(key);
        unsafe {
            *result = core::ptr::null_mut::<_object>();
        }
        return 1i32.wrapping_neg();
    }
    _PyDict_GetItemRef_KnownHash(op as *mut __SlateRecord_dictobject_h_33, key, hash, result)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyDict_GetItemRef_Unicode_LockHeld(
    op: *mut __SlateRecord_dictobject_h_33,
    key: *mut _object,
    result: *mut *mut _object,
) -> i32 {
    let hash: i64 = _PyObject_HashFast(key);
    if hash == (1i32.wrapping_neg() as i64) {
        dict_unhashable_type(key);
        unsafe {
            *result = core::ptr::null_mut::<_object>();
        }
        return 1i32.wrapping_neg();
    }
    let mut value: *mut _object = unsafe { core::mem::zeroed() };
    let ix: i64 = _Py_dict_lookup(op, key, hash, core::ptr::addr_of_mut!(value));
    if ix == (3i32.wrapping_neg() as i64) {
        unsafe {
            *result = core::ptr::null_mut::<_object>();
        }
        return 1i32.wrapping_neg();
    }
    if value.is_null() {
        unsafe {
            *result = core::ptr::null_mut::<_object>();
        }
        return 0; // missing key
    }
    unsafe {
        *result = Py_NewRef!(value);
    }
    return 1; // key is present
}

/// Variant of PyDict_GetItem() that doesn't suppress exceptions.
/// This returns NULL *with* an exception set if an exception occurred.
/// It returns NULL *without* an exception set if the key wasn't present.
#[unsafe(no_mangle)]
extern "C-unwind" fn PyDict_GetItemWithError(op: *mut _object, key: *mut _object) -> *mut _object {
    let mut ix: i64 = 0;
    let mut hash: i64 = 0;
    let mp: *mut __SlateRecord_dictobject_h_33 = op as *mut __SlateRecord_dictobject_h_33;
    let mut value: *mut _object = unsafe { core::mem::zeroed() };
    if PyDict_Check!(op) == 0 {
        PyErr_BadInternalCall__54!();
        return core::ptr::null_mut::<_object>();
    }
    hash = _PyObject_HashFast(key);
    if hash == (1i32.wrapping_neg() as i64) {
        dict_unhashable_type(key);
        return core::ptr::null_mut::<_object>();
    }
    ix = _Py_dict_lookup(mp, key, hash, core::ptr::addr_of_mut!(value));
    return value; // borrowed reference
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyDict_GetItemWithError(dp: *mut _object, kv: *mut _object) -> *mut _object {
    let hash: i64 = unsafe { unsafe { (*Py_TYPE!(kv)).tp_hash }.unwrap()(kv) };
    if hash == (1i32.wrapping_neg() as i64) {
        return core::ptr::null_mut::<_object>();
    }
    return _PyDict_GetItem_KnownHash(dp, kv, hash); // borrowed reference
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyDict_GetItemIdWithError(
    dp: *mut _object,
    key: *mut _Py_Identifier,
) -> *mut _object {
    let kv: *mut _object = unsafe { _PyUnicode_FromId(key) }; // borrowed
    if kv.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let hash: i64 = unicode_get_hash(kv);
    // interned strings have their hash value initialised
    return _PyDict_GetItem_KnownHash(dp, kv, hash); // borrowed reference
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyDict_GetItemStringWithError(
    v: *mut _object,
    key: *const i8,
) -> *mut _object {
    let kv: *mut _object = unsafe { PyUnicode_FromString(key) };
    if kv.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let rv: *mut _object = PyDict_GetItemWithError(v, kv);
    Py_DECREF!(kv);
    rv
}

/// Fast version of global value lookup (LOAD_GLOBAL).
/// Lookup in globals, then builtins.
///
///
///
///
/// Raise an exception and return NULL if an error occurred (ex: computing the
/// key hash failed, key comparison failed, ...). Return NULL if the key doesn't
/// exist. Return the value if the key exists.
///
/// Returns a new reference.
#[unsafe(no_mangle)]
extern "C-unwind" fn _PyDict_LoadGlobal(
    globals: *mut __SlateRecord_dictobject_h_33,
    builtins: *mut __SlateRecord_dictobject_h_33,
    key: *mut _object,
) -> *mut _object {
    let mut ix: i64 = 0;
    let mut hash: i64 = 0;
    let mut value: *mut _object = unsafe { core::mem::zeroed() };
    hash = _PyObject_HashFast(key);
    if hash == (1i32.wrapping_neg() as i64) {
        return core::ptr::null_mut::<_object>();
    }
    // namespace 1: globals
    ix = _Py_dict_lookup_threadsafe(globals, key, hash, core::ptr::addr_of_mut!(value));
    if ix == (3i32.wrapping_neg() as i64) {
        return core::ptr::null_mut::<_object>();
    }
    if ix != (1i32.wrapping_neg() as i64) && !value.is_null() {
        return value;
    }
    // namespace 2: builtins
    ix = _Py_dict_lookup_threadsafe(builtins, key, hash, core::ptr::addr_of_mut!(value));
    value
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyDict_LoadGlobalStackRef(
    globals: *mut __SlateRecord_dictobject_h_33,
    builtins: *mut __SlateRecord_dictobject_h_33,
    key: *mut _object,
    res: *mut _PyStackRef,
) {
    let mut ix: i64 = 0;
    let hash: i64 = _PyObject_HashFast(key);
    if hash == (1i32.wrapping_neg() as i64) {
        unsafe {
            *res = PyStackRef_NULL;
        }
        return;
    }
    // namespace 1: globals
    ix = _Py_dict_lookup_threadsafe_stackref(globals, key, hash, res);
    if ix == (3i32.wrapping_neg() as i64) {
        return;
    }
    if ix != (1i32.wrapping_neg() as i64) && !PyStackRef_IsNull!(*res) {
        return;
    }
    // namespace 2: builtins
    ix = _Py_dict_lookup_threadsafe_stackref(builtins, key, hash, res);
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyDict_LoadBuiltinsFromGlobals(globals: *mut _object) -> *mut _object {
    if PyDict_Check!(globals) == 0 {
        PyErr_BadInternalCall__151!();
        return core::ptr::null_mut::<_object>();
    }
    let mp: *mut __SlateRecord_dictobject_h_33 = globals as *mut __SlateRecord_dictobject_h_33;
    let key: *mut _object = unsafe {
        core::ptr::addr_of_mut!(
            _PyRuntime
                .static_objects
                .singletons
                .strings
                .identifiers
                ._py___builtins__
                ._ascii
                .ob_base
        )
    };
    let hash: i64 = unicode_get_hash(key);
    // Use the stackref variant to avoid reference count contention on the
    //
    // builtins module in the free threading build. It's important not to
    //
    // make any escaping calls between the lookup and the `PyStackRef_CLOSE()`
    //
    // because the `ref` is not visible to the GC.
    let mut r#ref: _PyStackRef = unsafe { core::mem::zeroed() };
    let ix: i64 =
        _Py_dict_lookup_threadsafe_stackref(mp, key, hash, core::ptr::addr_of_mut!(r#ref));
    if ix == (3i32.wrapping_neg() as i64) {
        return core::ptr::null_mut::<_object>();
    }
    if PyStackRef_IsNull!(r#ref) {
        return Py_NewRef!(unsafe { PyEval_GetBuiltins() });
    }
    let mut builtins: *mut _object = PyStackRef_AsPyObjectBorrow(r#ref);
    if PyModule_Check!(builtins) != 0 {
        builtins = _PyModule_GetDict(builtins);
    }
    Py_INCREF(_PyObject_CAST!(builtins));
    PyStackRef_CLOSE(r#ref);
    builtins
}

/// Consumes references to key and value
fn setitem_take2_lock_held(
    mp: *mut __SlateRecord_dictobject_h_33,
    key: *mut _object,
    value: *mut _object,
) -> i32 {
    let hash: i64 = _PyObject_HashFast(key);
    if hash == (1i32.wrapping_neg() as i64) {
        dict_unhashable_type(key);
        Py_DECREF!(key);
        Py_DECREF!(value);
        return 1i32.wrapping_neg();
    }
    let interp: *mut _is = _PyInterpreterState_GET();
    if (unsafe { (*mp).ma_keys })
        == unsafe {
            core::ptr::addr_of_mut!(
                *(core::ptr::addr_of_mut!(empty_keys_struct) as *mut _dictkeysobject)
            )
        }
    {
        return insert_to_emptydict(interp, mp, key, hash, value);
    }
    // insertdict() handles any resizing that might be necessary
    insertdict(interp, mp, key, hash, value)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyDict_SetItem_Take2(
    mp: *mut __SlateRecord_dictobject_h_33,
    key: *mut _object,
    value: *mut _object,
) -> i32 {
    let mut res: i32 = 0;
    res = setitem_take2_lock_held(mp, key, value);
    res
}

/// CAUTION: PyDict_SetItem() must guarantee that it won't resize the
/// dictionary if it's merely replacing the value for an existing key.
/// This means that it's safe to loop over a dictionary with PyDict_Next()
/// and occasionally replace a value -- but you can't insert new keys or
/// remove them.
#[unsafe(no_mangle)]
extern "C-unwind" fn PyDict_SetItem(
    op: *mut _object,
    key: *mut _object,
    value: *mut _object,
) -> i32 {
    if PyDict_Check!(op) == 0 {
        PyErr_BadInternalCall__11!();
        return 1i32.wrapping_neg();
    }
    _PyDict_SetItem_Take2(
        op as *mut __SlateRecord_dictobject_h_33,
        Py_NewRef!(key),
        Py_NewRef!(value),
    )
}

fn setitem_lock_held(
    mp: *mut __SlateRecord_dictobject_h_33,
    key: *mut _object,
    value: *mut _object,
) -> i32 {
    setitem_take2_lock_held(mp, Py_NewRef!(key), Py_NewRef!(value))
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyDict_SetItem_KnownHash_LockHeld(
    mp: *mut __SlateRecord_dictobject_h_33,
    key: *mut _object,
    value: *mut _object,
    hash: i64,
) -> i32 {
    let interp: *mut _is = _PyInterpreterState_GET();
    if (unsafe { (*mp).ma_keys })
        == unsafe {
            core::ptr::addr_of_mut!(
                *(core::ptr::addr_of_mut!(empty_keys_struct) as *mut _dictkeysobject)
            )
        }
    {
        return insert_to_emptydict(interp, mp, Py_NewRef!(key), hash, Py_NewRef!(value));
    }
    // insertdict() handles any resizing that might be necessary
    insertdict(interp, mp, Py_NewRef!(key), hash, Py_NewRef!(value))
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyDict_SetItem_KnownHash(
    op: *mut _object,
    key: *mut _object,
    value: *mut _object,
    hash: i64,
) -> i32 {
    if PyDict_Check!(op) == 0 {
        PyErr_BadInternalCall__148!();
        return 1i32.wrapping_neg();
    }
    let mut res: i32 = 0;
    res = _PyDict_SetItem_KnownHash_LockHeld(
        op as *mut __SlateRecord_dictobject_h_33,
        key,
        value,
        hash,
    );
    res
}

fn delete_index_from_values(values: *mut _dictvalues, ix: i64) {
    let array: *mut u8 = get_insertion_order_array(values);
    let mut size: i32 = (unsafe { (*values).size }) as i32;
    let mut i: i32 = 0;
    while (((unsafe { *array.wrapping_offset(i as isize) }) as i32) as i64) != ix {
        i = i.wrapping_add(1);
    }
    size = size.wrapping_sub(1);
    while i < size {
        unsafe {
            *array.wrapping_offset(i as isize) = *array.wrapping_offset(i.wrapping_add(1) as isize);
        }
        i = i.wrapping_add(1);
    }
    unsafe {
        (*values).size = size as u8;
    }
}

fn delitem_common(
    mp: *mut __SlateRecord_dictobject_h_33,
    hash: i64,
    ix: i64,
    old_value: *mut _object,
) {
    let mut old_key: *mut _object = unsafe { core::mem::zeroed() };
    let hashpos: i64 = lookdict_index(unsafe { (*mp).ma_keys }, hash, ix);
    STORE_USED!(mp, unsafe { (*mp).ma_used }.wrapping_sub(1));
    if _PyDict_HasSplitTable!(mp) {
        STORE_SPLIT_VALUE__2!(mp, ix);
        // Update order
        delete_index_from_values(unsafe { (*mp).ma_values }, ix);
        ASSERT_CONSISTENT!();
    } else {
        FT_ATOMIC_STORE_UINT32_RELAXED!((*unsafe { (*mp).ma_keys }).dk_version, 0i32);
        dictkeys_set_index(
            unsafe { (*mp).ma_keys },
            hashpos,
            2i32.wrapping_neg() as i64,
        );
        if DK_IS_UNICODE!(unsafe { (*mp).ma_keys }) {
            let ep: *mut __SlateRecord_pycore_dict_h_90 =
                DK_UNICODE_ENTRIES(unsafe { (*mp).ma_keys }).wrapping_offset(ix as isize);
            old_key = unsafe { (*ep).me_key };
            STORE_KEY!(ep);
            STORE_VALUE__2!(ep);
        } else {
            let ep: *mut __SlateRecord_pycore_dict_h_85 =
                DK_ENTRIES(unsafe { (*mp).ma_keys }).wrapping_offset(ix as isize);
            old_key = unsafe { (*ep).me_key };
            STORE_KEY!(ep);
            STORE_VALUE__2!(ep);
            STORE_HASH!(ep, 0i32);
        }
        Py_DECREF!(old_key);
    }
    Py_DECREF!(old_value);
    ASSERT_CONSISTENT!();
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyDict_DelItem(op: *mut _object, key: *mut _object) -> i32 {
    let hash: i64 = _PyObject_HashFast(key);
    if hash == (1i32.wrapping_neg() as i64) {
        dict_unhashable_type(key);
        return 1i32.wrapping_neg();
    }
    _PyDict_DelItem_KnownHash(op, key, hash)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyDict_DelItem_KnownHash_LockHeld(
    op: *mut _object,
    key: *mut _object,
    hash: i64,
) -> i32 {
    let mut ix: i64 = 0;
    let mut old_value: *mut _object = unsafe { core::mem::zeroed() };
    if PyDict_Check!(op) == 0 {
        PyErr_BadInternalCall__29!();
        return 1i32.wrapping_neg();
    }
    let mp: *mut __SlateRecord_dictobject_h_33 = op as *mut __SlateRecord_dictobject_h_33;
    ix = _Py_dict_lookup(mp, key, hash, core::ptr::addr_of_mut!(old_value));
    if ix == (3i32.wrapping_neg() as i64) {
        return 1i32.wrapping_neg();
    }
    if ix == (1i32.wrapping_neg() as i64) || old_value.is_null() {
        unsafe { _PyErr_SetKeyError(key) };
        return 1i32.wrapping_neg();
    }
    let interp: *mut _is = _PyInterpreterState_GET();
    _PyDict_NotifyEvent(interp, 2u32, mp, key, core::ptr::null_mut::<_object>());
    delitem_common(mp, hash, ix, old_value);
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyDict_DelItem_KnownHash(
    op: *mut _object,
    key: *mut _object,
    hash: i64,
) -> i32 {
    let mut res: i32 = 0;
    res = _PyDict_DelItem_KnownHash_LockHeld(op, key, hash);
    res
}

fn delitemif_lock_held(
    op: *mut _object,
    key: *mut _object,
    predicate: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
    arg: *mut (),
) -> i32 {
    let mut ix: i64 = 0;
    let mut hash: i64 = 0;
    let mut old_value: *mut _object = unsafe { core::mem::zeroed() };
    let mut res: i32 = 0;
    hash = unsafe { PyObject_Hash(key) };
    if hash == (1i32.wrapping_neg() as i64) {
        return 1i32.wrapping_neg();
    }
    let mp: *mut __SlateRecord_dictobject_h_33 = op as *mut __SlateRecord_dictobject_h_33;
    ix = _Py_dict_lookup(mp, key, hash, core::ptr::addr_of_mut!(old_value));
    if ix == (3i32.wrapping_neg() as i64) {
        return 1i32.wrapping_neg();
    }
    if ix == (1i32.wrapping_neg() as i64) || old_value.is_null() {
        return 0;
    }
    res = unsafe { predicate.unwrap()(old_value, arg) };
    if res == 1i32.wrapping_neg() {
        return 1i32.wrapping_neg();
    }
    if res > 0 {
        let interp: *mut _is = _PyInterpreterState_GET();
        _PyDict_NotifyEvent(interp, 2u32, mp, key, core::ptr::null_mut::<_object>());
        delitem_common(mp, hash, ix, old_value);
        return 1;
    } else {
        return 0;
    }
}

/// This function promises that the predicate -> deletion sequence is atomic
/// (i.e. protected by the GIL or the per-dict mutex in free threaded builds),
/// assuming the predicate itself doesn't release the GIL (or cause re-entrancy
/// which would release the per-dict mutex)
#[unsafe(no_mangle)]
extern "C-unwind" fn _PyDict_DelItemIf(
    op: *mut _object,
    key: *mut _object,
    predicate: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
    arg: *mut (),
) -> i32 {
    let mut res: i32 = 0;
    res = delitemif_lock_held(op, key, predicate, arg);
    res
}

fn clear_embedded_values(values: *mut _dictvalues, nentries: i64) {
    let mut refs: __SlateAlign16<[*mut _object; 30]> = __SlateAlign16([0 as *mut _object; 30]);
    for i in 0..nentries {
        refs.0[i as usize] = unsafe {
            *((*values).values.as_mut_ptr() as *mut *mut _object).wrapping_offset(i as isize)
        };
        unsafe {
            *((*values).values.as_mut_ptr() as *mut *mut _object).wrapping_offset(i as isize) =
                core::ptr::null_mut::<_object>();
        }
    }
    unsafe {
        (*values).size = 0;
    }
    for i in 0..nentries {
        Py_XDECREF!(refs.0[i as usize]);
    }
}

fn clear_lock_held(op: *mut _object) {
    let mut oldvalues: *mut _dictvalues = unsafe { core::mem::zeroed() };
    let mut n: i64 = 0;
    if PyDict_Check!(op) == 0 {
        return;
    }
    let mp: *mut __SlateRecord_dictobject_h_33 = op as *mut __SlateRecord_dictobject_h_33;
    let oldkeys: *mut _dictkeysobject = unsafe { (*mp).ma_keys };
    oldvalues = unsafe { (*mp).ma_values };
    if oldkeys
        == unsafe {
            core::ptr::addr_of_mut!(
                *(core::ptr::addr_of_mut!(empty_keys_struct) as *mut _dictkeysobject)
            )
        }
    {
        return;
    }
    // Empty the dict...
    let interp: *mut _is = _PyInterpreterState_GET();
    _PyDict_NotifyEvent(
        interp,
        4u32,
        mp,
        core::ptr::null_mut::<_object>(),
        core::ptr::null_mut::<_object>(),
    );
    // We don't inc ref empty keys because they're immortal
    ensure_shared_on_resize(mp);
    STORE_USED__2!(mp, 0i32);
    if oldvalues.is_null() {
        set_keys(mp, unsafe {
            core::ptr::addr_of_mut!(
                *(core::ptr::addr_of_mut!(empty_keys_struct) as *mut _dictkeysobject)
            )
        });
        dictkeys_decref(interp, oldkeys, IS_DICT_SHARED!() != 0);
    } else {
        if (unsafe { (*oldvalues).embedded }) != 0 {
            clear_embedded_values(oldvalues, unsafe { (*oldkeys).dk_nentries });
        } else {
            set_values(mp, core::ptr::null_mut::<_dictvalues>());
            set_keys(mp, unsafe {
                core::ptr::addr_of_mut!(
                    *(core::ptr::addr_of_mut!(empty_keys_struct) as *mut _dictkeysobject)
                )
            });
            n = unsafe { (*oldkeys).dk_nentries };
            for i in 0..n {
                Py_CLEAR!(
                    *unsafe { (*oldvalues).values.as_mut_ptr() as *mut *mut _object }
                        .wrapping_offset(i as isize)
                );
            }
            free_values(oldvalues, IS_DICT_SHARED!() != 0);
            dictkeys_decref(interp, oldkeys, false);
        }
    }
    ASSERT_CONSISTENT!();
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyDict_Clear_LockHeld(op: *mut _object) {
    clear_lock_held(op);
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyDict_Clear(op: *mut _object) {
    clear_lock_held(op);
}

/// Internal version of PyDict_Next that returns a hash value in addition
/// to the key and value.
/// Return 1 on success, return 0 when the reached the end of the dictionary
/// (or if op is not a dictionary)
#[unsafe(no_mangle)]
extern "C-unwind" fn _PyDict_Next(
    op: *mut _object,
    ppos: *mut i64,
    pkey: *mut *mut _object,
    pvalue: *mut *mut _object,
    phash: *mut i64,
) -> i32 {
    let mut i: i64 = 0;
    let mut key: *mut _object = unsafe { core::mem::zeroed() };
    let mut value: *mut _object = unsafe { core::mem::zeroed() };
    let mut hash: i64 = 0;
    if PyDict_Check!(op) == 0 {
        return 0;
    }
    let mp: *mut __SlateRecord_dictobject_h_33 = op as *mut __SlateRecord_dictobject_h_33;
    i = unsafe { *ppos };
    if _PyDict_HasSplitTable!(mp) {
        if i < 0 || i >= unsafe { (*mp).ma_used } {
            return 0;
        }
        let index: i32 = get_index_from_order(mp, i);
        value = unsafe {
            *((*(*mp).ma_values).values.as_mut_ptr() as *mut *mut _object)
                .wrapping_offset(index as isize)
        };
        key =
            unsafe { (*DK_UNICODE_ENTRIES((*mp).ma_keys).wrapping_offset(index as isize)).me_key };
        hash = unicode_get_hash(key);
    } else {
        let n: i64 = unsafe { (*(*mp).ma_keys).dk_nentries };
        if i < 0 || i >= n {
            return 0;
        }
        if DK_IS_UNICODE!(unsafe { (*mp).ma_keys }) {
            let mut entry_ptr: *mut __SlateRecord_pycore_dict_h_90 =
                DK_UNICODE_ENTRIES(unsafe { (*mp).ma_keys }).wrapping_offset(i as isize);
            while i < n && unsafe { (*entry_ptr).me_value }.is_null() {
                let __v0: *mut __SlateRecord_pycore_dict_h_90 = entry_ptr;
                entry_ptr = __v0.wrapping_offset(1 as isize);
                i = i.wrapping_add(1);
            }
            if i >= n {
                return 0;
            }
            key = unsafe { (*entry_ptr).me_key };
            hash = unicode_get_hash(unsafe { (*entry_ptr).me_key });
            value = unsafe { (*entry_ptr).me_value };
        } else {
            let mut entry_ptr: *mut __SlateRecord_pycore_dict_h_85 =
                DK_ENTRIES(unsafe { (*mp).ma_keys }).wrapping_offset(i as isize);
            while i < n && unsafe { (*entry_ptr).me_value }.is_null() {
                let __v5: *mut __SlateRecord_pycore_dict_h_85 = entry_ptr;
                entry_ptr = __v5.wrapping_offset(1 as isize);
                i = i.wrapping_add(1);
            }
            if i >= n {
                return 0;
            }
            key = unsafe { (*entry_ptr).me_key };
            hash = unsafe { (*entry_ptr).me_hash };
            value = unsafe { (*entry_ptr).me_value };
        }
    }
    unsafe {
        *ppos = i.wrapping_add(1);
    }
    if !pkey.is_null() {
        unsafe {
            *pkey = key;
        }
    }
    if !pvalue.is_null() {
        unsafe {
            *pvalue = value;
        }
    }
    if !phash.is_null() {
        unsafe {
            *phash = hash;
        }
    }
    1
}

/// Iterate over a dict.  Use like so:
///
///     Py_ssize_t i;
///     PyObject *key, *value;
///     i = 0;   # important!  i should not otherwise be changed by you
///     while (PyDict_Next(yourdict, &i, &key, &value)) {
///         Refer to borrowed references in key and value.
///     }
///
/// Return 1 on success, return 0 when the reached the end of the dictionary
/// (or if op is not a dictionary)
///
/// CAUTION:  In general, it isn't safe to use PyDict_Next in a loop that
/// mutates the dict.  One exception:  it is safe if the loop merely changes
/// the values associated with the keys (but doesn't insert new keys or
/// delete keys), via PyDict_SetItem().
#[unsafe(no_mangle)]
extern "C-unwind" fn PyDict_Next(
    op: *mut _object,
    ppos: *mut i64,
    pkey: *mut *mut _object,
    pvalue: *mut *mut _object,
) -> i32 {
    _PyDict_Next(op, ppos, pkey, pvalue, core::ptr::null_mut::<i64>())
}

/// Internal version of dict.pop().
#[unsafe(no_mangle)]
extern "C-unwind" fn _PyDict_Pop_KnownHash(
    mp: *mut __SlateRecord_dictobject_h_33,
    key: *mut _object,
    hash: i64,
    result: *mut *mut _object,
) -> i32 {
    if (unsafe { (*mp).ma_used }) == 0 {
        if !result.is_null() {
            unsafe {
                *result = core::ptr::null_mut::<_object>();
            }
        }
        return 0;
    }
    let mut old_value: *mut _object = unsafe { core::mem::zeroed() };
    let ix: i64 = _Py_dict_lookup(mp, key, hash, core::ptr::addr_of_mut!(old_value));
    if ix == (3i32.wrapping_neg() as i64) {
        if !result.is_null() {
            unsafe {
                *result = core::ptr::null_mut::<_object>();
            }
        }
        return 1i32.wrapping_neg();
    }
    if ix == (1i32.wrapping_neg() as i64) || old_value.is_null() {
        if !result.is_null() {
            unsafe {
                *result = core::ptr::null_mut::<_object>();
            }
        }
        return 0;
    }
    let interp: *mut _is = _PyInterpreterState_GET();
    _PyDict_NotifyEvent(interp, 2u32, mp, key, core::ptr::null_mut::<_object>());
    delitem_common(mp, hash, ix, Py_NewRef!(old_value));
    ASSERT_CONSISTENT!();
    if !result.is_null() {
        unsafe {
            *result = old_value;
        }
    } else {
        Py_DECREF!(old_value);
    }
    1
}

fn pop_lock_held(op: *mut _object, key: *mut _object, result: *mut *mut _object) -> i32 {
    if PyDict_Check!(op) == 0 {
        if !result.is_null() {
            unsafe {
                *result = core::ptr::null_mut::<_object>();
            }
        }
        PyErr_BadInternalCall__150!();
        return 1i32.wrapping_neg();
    }
    let dict: *mut __SlateRecord_dictobject_h_33 = op as *mut __SlateRecord_dictobject_h_33;
    if (unsafe { (*dict).ma_used }) == 0 {
        if !result.is_null() {
            unsafe {
                *result = core::ptr::null_mut::<_object>();
            }
        }
        return 0;
    }
    let hash: i64 = _PyObject_HashFast(key);
    if hash == (1i32.wrapping_neg() as i64) {
        dict_unhashable_type(key);
        if !result.is_null() {
            unsafe {
                *result = core::ptr::null_mut::<_object>();
            }
        }
        return 1i32.wrapping_neg();
    }
    _PyDict_Pop_KnownHash(dict, key, hash, result)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyDict_Pop(
    op: *mut _object,
    key: *mut _object,
    result: *mut *mut _object,
) -> i32 {
    let mut err: i32 = 0;
    err = pop_lock_held(op, key, result);
    err
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyDict_PopString(
    op: *mut _object,
    key: *const i8,
    result: *mut *mut _object,
) -> i32 {
    let key_obj: *mut _object = unsafe { PyUnicode_FromString(key) };
    if key_obj.is_null() {
        if !result.is_null() {
            unsafe {
                *result = core::ptr::null_mut::<_object>();
            }
        }
        return 1i32.wrapping_neg();
    }
    let res: i32 = PyDict_Pop(op, key_obj, result);
    Py_DECREF!(key_obj);
    res
}

fn dict_pop_default(
    dict: *mut _object,
    key: *mut _object,
    default_value: *mut _object,
) -> *mut _object {
    let mut result: *mut _object = unsafe { core::mem::zeroed() };
    if PyDict_Pop(dict, key, core::ptr::addr_of_mut!(result)) == 0 {
        if !default_value.is_null() {
            return Py_NewRef!(default_value);
        }
        unsafe { _PyErr_SetKeyError(key) };
        return core::ptr::null_mut::<_object>();
    }
    result
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyDict_Pop(
    dict: *mut _object,
    key: *mut _object,
    default_value: *mut _object,
) -> *mut _object {
    dict_pop_default(dict, key, default_value)
}

fn dict_dict_fromkeys(
    interp: *mut _is,
    mp: *mut __SlateRecord_dictobject_h_33,
    iterable: *mut _object,
    value: *mut _object,
) -> *mut __SlateRecord_dictobject_h_33 {
    let mut oldvalue: *mut _object = unsafe { core::mem::zeroed() };
    let mut pos: i64 = 0;
    let mut key: *mut _object = unsafe { core::mem::zeroed() };
    let mut hash: i64 = 0;
    let unicode: i32 =
        DK_IS_UNICODE!(unsafe { (*(iterable as *mut __SlateRecord_dictobject_h_33)).ma_keys })
            as i32;
    let new_size: u8 = Py_MAX__5!(estimate_log2_keysize(PyDict_GET_SIZE(iterable)), unsafe {
        (*(*mp).ma_keys).dk_log2_size
    }) as u8;
    if dictresize(interp, mp, new_size, unicode) != 0 {
        Py_DECREF__2!(mp);
        return core::ptr::null_mut::<__SlateRecord_dictobject_h_33>();
    }
    while _PyDict_Next(
        iterable,
        core::ptr::addr_of_mut!(pos),
        core::ptr::addr_of_mut!(key),
        core::ptr::addr_of_mut!(oldvalue),
        core::ptr::addr_of_mut!(hash),
    ) != 0
    {
        if insertdict(interp, mp, Py_NewRef!(key), hash, Py_NewRef!(value)) != 0 {
            Py_DECREF__2!(mp);
            return core::ptr::null_mut::<__SlateRecord_dictobject_h_33>();
        }
    }
    mp
}

fn dict_set_fromkeys(
    interp: *mut _is,
    mp: *mut __SlateRecord_dictobject_h_33,
    iterable: *mut _object,
    value: *mut _object,
) -> *mut __SlateRecord_dictobject_h_33 {
    let mut pos: i64 = 0;
    let mut key: *mut _object = unsafe { core::mem::zeroed() };
    let mut hash: i64 = 0;
    let new_size: u8 = Py_MAX__5!(estimate_log2_keysize(PySet_GET_SIZE(iterable)), unsafe {
        (*(*mp).ma_keys).dk_log2_size
    }) as u8;
    if dictresize(interp, mp, new_size, 0i32) != 0 {
        Py_DECREF__2!(mp);
        return core::ptr::null_mut::<__SlateRecord_dictobject_h_33>();
    }
    while (unsafe {
        _PySet_NextEntryRef(
            iterable,
            core::ptr::addr_of_mut!(pos),
            core::ptr::addr_of_mut!(key),
            core::ptr::addr_of_mut!(hash),
        )
    }) != 0
    {
        if insertdict(interp, mp, key, hash, Py_NewRef!(value)) != 0 {
            Py_DECREF__2!(mp);
            return core::ptr::null_mut::<__SlateRecord_dictobject_h_33>();
        }
    }
    mp
}

/// Internal version of dict.from_keys().  It is subclass-friendly.
#[unsafe(no_mangle)]
extern "C-unwind" fn _PyDict_FromKeys(
    cls: *mut _object,
    iterable: *mut _object,
    value: *mut _object,
) -> *mut _object {
    let mut interp: *mut _is = unsafe { core::mem::zeroed() };
    let mut status: i32 = unsafe { core::mem::zeroed() };
    let mut d: *mut _object = unsafe { core::mem::zeroed() };
    let mut key: *mut _object = unsafe { core::mem::zeroed() };
    // iter(iterable)
    let mut it: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        interp = _PyInterpreterState_GET();
        d = _PyObject_CallNoArgs(cls);
        if d.is_null() {
            return core::ptr::null_mut::<_object>();
        } else {
            if PyDict_CheckExact!(d) != 0 {
                if PyDict_CheckExact!(iterable) != 0 {
                    let mp: *mut __SlateRecord_dictobject_h_33 =
                        d as *mut __SlateRecord_dictobject_h_33;
                    d = dict_dict_fromkeys(interp, mp, iterable, value) as *mut _object;
                    return d;
                } else {
                    if PyAnySet_CheckExact!(iterable) {
                        let mp: *mut __SlateRecord_dictobject_h_33 =
                            d as *mut __SlateRecord_dictobject_h_33;
                        d = dict_set_fromkeys(interp, mp, iterable, value) as *mut _object;
                        return d;
                    }
                }
            }
            it = PyObject_GetIter(iterable);
            if it.is_null() {
                Py_DECREF!(d);
                return core::ptr::null_mut::<_object>();
            } else {
                '__join_0: {
                    if PyDict_CheckExact!(d) != 0 {
                        loop {
                            let __v0: *mut _object = PyIter_Next(it);
                            key = __v0;
                            if !__v0.is_null() {
                                status = setitem_lock_held(
                                    d as *mut __SlateRecord_dictobject_h_33,
                                    key,
                                    value,
                                );
                                Py_DECREF!(key);
                                if status < 0 {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }
                    } else {
                        loop {
                            let __v1: *mut _object = PyIter_Next(it);
                            key = __v1;
                            if !__v1.is_null() {
                                status = PyObject_SetItem(d, key, value);
                                Py_DECREF!(key);
                                if status < 0 {
                                    break '__join_0;
                                }
                            } else {
                                break;
                            }
                        }
                    }
                    if PyErr_Occurred().is_null() {
                        Py_DECREF!(it);
                        return d;
                    }
                }
                Py_DECREF!(it);
                Py_DECREF!(d);
                return core::ptr::null_mut::<_object>();
            }
        }
    }
}

// Methods
#[unsafe(link_section = ".text.slate_distinct.dictobject.dict_dealloc")]
extern "C-unwind" fn dict_dealloc(self_: *mut _object) {
    let mp: *mut __SlateRecord_dictobject_h_33 = self_ as *mut __SlateRecord_dictobject_h_33;
    let interp: *mut _is = _PyInterpreterState_GET();
    _PyObject_ResurrectStart(self_);
    _PyDict_NotifyEvent(
        interp,
        5u32,
        mp,
        core::ptr::null_mut::<_object>(),
        core::ptr::null_mut::<_object>(),
    );
    if _PyObject_ResurrectEnd(self_) != 0 {
        return;
    }
    let values: *mut _dictvalues = unsafe { (*mp).ma_values };
    let keys: *mut _dictkeysobject = unsafe { (*mp).ma_keys };
    let mut i: i64 = 0;
    let mut n: i64 = 0;
    // bpo-31095: UnTrack is needed before calling any callbacks
    unsafe { PyObject_GC_UnTrack(mp as *mut ()) };
    if !values.is_null() {
        if ((unsafe { (*values).embedded }) as i32) == 0 {
            i = 0;
            n = (unsafe { (*values).capacity }) as i64;
            while i < n {
                Py_XDECREF!(unsafe {
                    *((*values).values.as_mut_ptr() as *mut *mut _object)
                        .wrapping_offset(i as isize)
                });
                i = i.wrapping_add(1);
            }
            free_values(values, false);
        }
        dictkeys_decref(interp, keys, false);
    } else {
        if !keys.is_null() {
            dictkeys_decref(interp, keys, false);
        }
    }
    if Py_IS_TYPE__2!(mp, unsafe { core::ptr::addr_of_mut!(PyDict_Type) }) != 0 {
        _PyFreeList_Free(
            unsafe { core::ptr::addr_of_mut!((*_Py_freelists_GET()).dicts) },
            _PyObject_CAST__2!(mp) as *mut (),
            80i64,
            unsafe { (*Py_TYPE__2!(mp)).tp_free },
        );
    } else {
        unsafe { unsafe { (*Py_TYPE__2!(mp)).tp_free }.unwrap()(mp as *mut ()) };
    }
}

fn dict_repr_lock_held(self_: *mut _object) -> *mut _object {
    let mut first: i32 = unsafe { core::mem::zeroed() };
    // Do repr() on each key+value pair, and insert ": " between them.
    // Note that repr may mutate the dict.
    let mut i: i64 = unsafe { core::mem::zeroed() };
    let mut writer: *mut PyUnicodeWriter = unsafe { core::mem::zeroed() };
    let mut __v4: *mut _object = unsafe { core::mem::zeroed() };
    let mut res: i32 = unsafe { core::mem::zeroed() };
    let mut value: *mut _object = unsafe { core::mem::zeroed() };
    let mut key: *mut _object = unsafe { core::mem::zeroed() };
    let mut mp: *mut __SlateRecord_dictobject_h_33 = unsafe { core::mem::zeroed() };
    unsafe {
        mp = self_ as *mut __SlateRecord_dictobject_h_33;
        key = core::ptr::null_mut::<_object>();
        value = core::ptr::null_mut::<_object>();
        res = Py_ReprEnter(mp as *mut _object);
        if res != 0 {
            if res > 0 {
                __v4 = PyUnicode_FromString(c"{...}".as_ptr());
            } else {
                __v4 = core::ptr::null_mut::<_object>();
            }
            return __v4;
        } else {
            if (*mp).ma_used == 0 {
                Py_ReprLeave(mp as *mut _object);
                return PyUnicode_FromString(c"{}".as_ptr());
            } else {
                '__join_0: {
                    // "{" + "1: 2" + ", 3: 4" * (len - 1) + "}"
                    let prealloc: i64 = 5i64
                        .wrapping_add(6i64.wrapping_mul(unsafe { (*mp).ma_used }.wrapping_sub(1)))
                        .wrapping_add(1);
                    writer = PyUnicodeWriter_Create(prealloc);
                    if !writer.is_null() {
                        if PyUnicodeWriter_WriteChar(writer, 123u32) >= 0 {
                            i = 0;
                            first = 1;
                            while _PyDict_Next(
                                mp as *mut _object,
                                core::ptr::addr_of_mut!(i),
                                core::ptr::addr_of_mut!(key),
                                core::ptr::addr_of_mut!(value),
                                core::ptr::null_mut::<i64>(),
                            ) != 0
                            {
                                // Prevent repr from deleting key or value during key format.
                                Py_INCREF!(key);
                                Py_INCREF!(value);
                                if first == 0 {
                                    // Write ", "
                                    if PyUnicodeWriter_WriteChar(writer, 44u32) < 0 {
                                        break '__join_0;
                                    } else {
                                        if PyUnicodeWriter_WriteChar(writer, 32u32) < 0 {
                                            break '__join_0;
                                        }
                                    }
                                }
                                first = 0;
                                // Write repr(key)
                                if PyUnicodeWriter_WriteRepr(writer, key) < 0 {
                                    break '__join_0;
                                } else {
                                    // Write ": "
                                    if PyUnicodeWriter_WriteChar(writer, 58u32) < 0 {
                                        break '__join_0;
                                    } else {
                                        if PyUnicodeWriter_WriteChar(writer, 32u32) < 0 {
                                            break '__join_0;
                                        } else {
                                            // Write repr(value)
                                            if PyUnicodeWriter_WriteRepr(writer, value) < 0 {
                                                break '__join_0;
                                            } else {
                                                Py_CLEAR__2!(key);
                                                Py_CLEAR__2!(value);
                                            }
                                        }
                                    }
                                }
                            }
                            if PyUnicodeWriter_WriteChar(writer, 125u32) >= 0 {
                                Py_ReprLeave(mp as *mut _object);
                                return PyUnicodeWriter_Finish(writer);
                            }
                        }
                    }
                }
                Py_ReprLeave(mp as *mut _object);
                PyUnicodeWriter_Discard(writer);
                Py_XDECREF!(key);
                Py_XDECREF!(value);
                return core::ptr::null_mut::<_object>();
            }
        }
    }
}

#[unsafe(link_section = ".text.slate_distinct.dictobject.dict_repr")]
extern "C-unwind" fn dict_repr(self_: *mut _object) -> *mut _object {
    let res: *mut _object = dict_repr_lock_held(self_);
    res
}

#[unsafe(link_section = ".text.slate_distinct.dictobject.dict_length")]
extern "C-unwind" fn dict_length(self_: *mut _object) -> i64 {
    unsafe { (*(self_ as *mut __SlateRecord_dictobject_h_33)).ma_used }
}

#[unsafe(link_section = ".text.slate_distinct.dictobject.dict_subscript")]
extern "C-unwind" fn dict_subscript(self_: *mut _object, key: *mut _object) -> *mut _object {
    let mp: *mut __SlateRecord_dictobject_h_33 = self_ as *mut __SlateRecord_dictobject_h_33;
    let mut ix: i64 = 0;
    let mut hash: i64 = 0;
    let mut value: *mut _object = unsafe { core::mem::zeroed() };
    hash = _PyObject_HashFast(key);
    if hash == (1i32.wrapping_neg() as i64) {
        dict_unhashable_type(key);
        return core::ptr::null_mut::<_object>();
    }
    ix = _Py_dict_lookup_threadsafe(mp, key, hash, core::ptr::addr_of_mut!(value));
    if ix == (3i32.wrapping_neg() as i64) {
        return core::ptr::null_mut::<_object>();
    }
    if ix == (1i32.wrapping_neg() as i64) || value.is_null() {
        if PyDict_CheckExact__3!(mp) == 0 {
            // Look up __missing__ method if we're a subclass.
            let missing: *mut _object = unsafe {
                _PyObject_LookupSpecial(
                    mp as *mut _object,
                    core::ptr::addr_of_mut!(
                        _PyRuntime
                            .static_objects
                            .singletons
                            .strings
                            .identifiers
                            ._py___missing__
                            ._ascii
                            .ob_base
                    ),
                )
            };
            if !missing.is_null() {
                let res: *mut _object = unsafe { PyObject_CallOneArg(missing, key) };
                Py_DECREF!(missing);
                return res;
            } else {
                if !unsafe { PyErr_Occurred() }.is_null() {
                    return core::ptr::null_mut::<_object>();
                }
            }
        }
        unsafe { _PyErr_SetKeyError(key) };
        return core::ptr::null_mut::<_object>();
    }
    value
}

#[unsafe(link_section = ".text.slate_distinct.dictobject.dict_ass_sub")]
extern "C-unwind" fn dict_ass_sub(mp: *mut _object, v: *mut _object, w: *mut _object) -> i32 {
    if w.is_null() {
        return PyDict_DelItem(mp, v);
    } else {
        return PyDict_SetItem(mp, v, w);
    }
}

/// mp_length
/// mp_subscript
/// mp_ass_subscript
static mut dict_as_mapping: __SlateRecord_object_h_126 = __SlateRecord_object_h_126 {
    mp_length: Some(dict_length),
    mp_subscript: Some(dict_subscript),
    mp_ass_subscript: Some(dict_ass_sub),
};

fn keys_lock_held(dict: *mut _object) -> *mut _object {
    let mut key: *mut _object = unsafe { core::mem::zeroed() };
    let mut pos: i64 = unsafe { core::mem::zeroed() };
    // Nothing we do below makes any function calls.
    let mut j: i64 = unsafe { core::mem::zeroed() };
    let mut n: i64 = unsafe { core::mem::zeroed() };
    let mut v: *mut _object = unsafe { core::mem::zeroed() };
    let mut mp: *mut __SlateRecord_dictobject_h_33 = unsafe { core::mem::zeroed() };
    let mut __v8: bool = unsafe { core::mem::zeroed() };
    unsafe {
        if dict.is_null() {
            __v8 = true;
        } else {
            __v8 = PyDict_Check!(dict) == 0;
        }
        if __v8 {
            PyErr_BadInternalCall__18!();
            return core::ptr::null_mut::<_object>();
        } else {
            mp = dict as *mut __SlateRecord_dictobject_h_33;
            loop {
                n = (*mp).ma_used;
                v = PyList_New(n);
                if v.is_null() {
                    return core::ptr::null_mut::<_object>();
                } else {
                    if n != (*mp).ma_used {
                        // Durnit.  The allocations caused the dict to resize.
                        // Just start over, this shouldn't normally happen.
                        Py_DECREF!(v);
                    } else {
                        break;
                    }
                }
            }
            j = 0;
            pos = 0;
            while _PyDict_Next(
                mp as *mut _object,
                core::ptr::addr_of_mut!(pos),
                core::ptr::addr_of_mut!(key),
                core::ptr::null_mut::<*mut _object>(),
                core::ptr::null_mut::<i64>(),
            ) != 0
            {
                PyList_SET_ITEM!(v, j, Py_NewRef!(key));
                j = j.wrapping_add(1);
            }
            return v;
        }
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyDict_Keys(dict: *mut _object) -> *mut _object {
    let res: *mut _object = keys_lock_held(dict);
    res
}

fn values_lock_held(dict: *mut _object) -> *mut _object {
    let mut value: *mut _object = unsafe { core::mem::zeroed() };
    let mut pos: i64 = unsafe { core::mem::zeroed() };
    // Nothing we do below makes any function calls.
    let mut j: i64 = unsafe { core::mem::zeroed() };
    let mut n: i64 = unsafe { core::mem::zeroed() };
    let mut v: *mut _object = unsafe { core::mem::zeroed() };
    let mut mp: *mut __SlateRecord_dictobject_h_33 = unsafe { core::mem::zeroed() };
    let mut __v8: bool = unsafe { core::mem::zeroed() };
    unsafe {
        if dict.is_null() {
            __v8 = true;
        } else {
            __v8 = PyDict_Check!(dict) == 0;
        }
        if __v8 {
            PyErr_BadInternalCall__81!();
            return core::ptr::null_mut::<_object>();
        } else {
            mp = dict as *mut __SlateRecord_dictobject_h_33;
            loop {
                n = (*mp).ma_used;
                v = PyList_New(n);
                if v.is_null() {
                    return core::ptr::null_mut::<_object>();
                } else {
                    if n != (*mp).ma_used {
                        // Durnit.  The allocations caused the dict to resize.
                        // Just start over, this shouldn't normally happen.
                        Py_DECREF!(v);
                    } else {
                        break;
                    }
                }
            }
            j = 0;
            pos = 0;
            while _PyDict_Next(
                mp as *mut _object,
                core::ptr::addr_of_mut!(pos),
                core::ptr::null_mut::<*mut _object>(),
                core::ptr::addr_of_mut!(value),
                core::ptr::null_mut::<i64>(),
            ) != 0
            {
                PyList_SET_ITEM!(v, j, Py_NewRef!(value));
                j = j.wrapping_add(1);
            }
            return v;
        }
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyDict_Values(dict: *mut _object) -> *mut _object {
    let res: *mut _object = values_lock_held(dict);
    res
}

fn items_lock_held(dict: *mut _object) -> *mut _object {
    let mut value: *mut _object = unsafe { core::mem::zeroed() };
    let mut key: *mut _object = unsafe { core::mem::zeroed() };
    let mut pos: i64 = unsafe { core::mem::zeroed() };
    // Nothing we do below makes any function calls.
    let mut j: i64 = unsafe { core::mem::zeroed() };
    let mut item_1: *mut _object = unsafe { core::mem::zeroed() };
    let mut n: i64 = unsafe { core::mem::zeroed() };
    let mut i: i64 = unsafe { core::mem::zeroed() };
    let mut v: *mut _object = unsafe { core::mem::zeroed() };
    let mut mp: *mut __SlateRecord_dictobject_h_33 = unsafe { core::mem::zeroed() };
    let mut __v14: bool = unsafe { core::mem::zeroed() };
    unsafe {
        if dict.is_null() {
            __v14 = true;
        } else {
            __v14 = PyDict_Check!(dict) == 0;
        }
        if __v14 {
            PyErr_BadInternalCall__111!();
            return core::ptr::null_mut::<_object>();
        } else {
            mp = dict as *mut __SlateRecord_dictobject_h_33;
            // Preallocate the list of tuples, to avoid allocations during
            // the loop over the items, which could trigger GC, which
            // could resize the dict. :-(
            loop {
                n = (*mp).ma_used;
                v = PyList_New(n);
                if v.is_null() {
                    return core::ptr::null_mut::<_object>();
                } else {
                    i = 0;
                    while i < n {
                        item_1 = PyTuple_New(2i64);
                        if item_1.is_null() {
                            Py_DECREF!(v);
                            return core::ptr::null_mut::<_object>();
                        } else {
                            PyList_SET_ITEM!(v, i, item_1);
                            i = i.wrapping_add(1);
                        }
                    }
                    if n != (*mp).ma_used {
                        // Durnit.  The allocations caused the dict to resize.
                        // Just start over, this shouldn't normally happen.
                        Py_DECREF!(v);
                    } else {
                        break;
                    }
                }
            }
            j = 0;
            pos = 0;
            while _PyDict_Next(
                mp as *mut _object,
                core::ptr::addr_of_mut!(pos),
                core::ptr::addr_of_mut!(key),
                core::ptr::addr_of_mut!(value),
                core::ptr::null_mut::<i64>(),
            ) != 0
            {
                let item: *mut _object = PyList_GET_ITEM!(v, j);
                PyTuple_SET_ITEM!(item, 0i32, Py_NewRef!(key));
                PyTuple_SET_ITEM!(item, 1i32, Py_NewRef!(value));
                j = j.wrapping_add(1);
            }
            return v;
        }
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyDict_Items(dict: *mut _object) -> *mut _object {
    let res: *mut _object = items_lock_held(dict);
    res
}

// [clinic input]
// @classmethod
// dict.fromkeys
//     iterable: object
//     value: object=None
//     /
//
// Create a new dictionary with keys from iterable and values set to value.
// [clinic start generated code]
/// [clinic end generated code: output=8fb98e4b10384999 input=382ba4855d0f74c3]
fn dict_fromkeys_impl(
    r#type: *mut _typeobject,
    iterable: *mut _object,
    value: *mut _object,
) -> *mut _object {
    _PyDict_FromKeys(r#type as *mut _object, iterable, value)
}

/// Single-arg dict update; used by dict_update_common and operators.
fn dict_update_arg(self_: *mut _object, arg: *mut _object) -> i32 {
    if PyDict_CheckExact!(arg) != 0 {
        return PyDict_Merge(self_, arg, 1i32);
    }
    let has_keys: i32 = unsafe {
        PyObject_HasAttrWithError(
            arg,
            core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py_keys
                    ._ascii
                    .ob_base
            ),
        )
    };
    if has_keys < 0 {
        return 1i32.wrapping_neg();
    }
    if has_keys != 0 {
        return PyDict_Merge(self_, arg, 1i32);
    }
    PyDict_MergeFromSeq2(self_, arg, 1i32)
}

fn dict_update_common(
    self_: *mut _object,
    args: *mut _object,
    kwds: *mut _object,
    methname: *const i8,
) -> i32 {
    let mut arg: *mut _object = core::ptr::null_mut::<_object>();
    let mut result: i32 = 0;
    if (unsafe { PyArg_UnpackTuple(args, methname, 0i64, 1i64, core::ptr::addr_of_mut!(arg)) }) == 0
    {
        result = 1i32.wrapping_neg();
    } else {
        if !arg.is_null() {
            result = dict_update_arg(self_, arg);
        }
    }
    if result == 0 && !kwds.is_null() {
        if (unsafe { PyArg_ValidateKeywordArguments(kwds) }) != 0 {
            result = PyDict_Merge(self_, kwds, 1i32);
        } else {
            result = 1i32.wrapping_neg();
        }
    }
    result
}

/// Note: dict.update() uses the METH_VARARGS|METH_KEYWORDS calling convention.
/// Using METH_FASTCALL|METH_KEYWORDS would make dict.update(**dict2) calls
/// slower, see the issue #29312.
#[unsafe(link_section = ".text.slate_distinct.dictobject.dict_update")]
extern "C-unwind" fn dict_update(
    self_: *mut _object,
    args: *mut _object,
    kwds: *mut _object,
) -> *mut _object {
    if dict_update_common(self_, args, kwds, c"update".as_ptr()) != 1i32.wrapping_neg() {
        return unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) };
    }
    core::ptr::null_mut::<_object>()
}

// Update unconditionally replaces existing items.
// Merge has a 3rd argument 'override'; if set, it acts like Update,
// otherwise it leaves existing items unchanged.
//
// PyDict_{Update,Merge} update/merge from a mapping object.
//
// PyDict_MergeFromSeq2 updates/merges from any iterable object
// producing iterable objects of length 2.
fn merge_from_seq2_lock_held(d: *mut _object, seq2: *mut _object, r#override: i32) -> i32 {
    let mut n: i64 = unsafe { core::mem::zeroed() };
    let mut value: *mut _object = unsafe { core::mem::zeroed() };
    let mut key: *mut _object = unsafe { core::mem::zeroed() };
    // item as a 2-tuple or 2-list
    let mut fast: *mut _object = unsafe { core::mem::zeroed() };
    // seq2[i]
    let mut item: *mut _object = unsafe { core::mem::zeroed() };
    // index into seq2 of current element
    let mut i: i64 = unsafe { core::mem::zeroed() };
    // iter(seq2)
    let mut it: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        it = PyObject_GetIter(seq2);
        if it.is_null() {
            return 1i32.wrapping_neg();
        } else {
            i = 0;
            '__join_0: {
                '__join_1: {
                    '__join_14: {
                        '__join_12: {
                            '__join_9: {
                                '__join_4: {
                                    loop {
                                        fast = core::ptr::null_mut::<_object>();
                                        item = PyIter_Next(it);
                                        if item.is_null() {
                                            break '__join_14;
                                        } else {
                                            // Convert item to sequence, and verify length 2.
                                            fast = PySequence_Fast(
                                                item,
                                                c"object is not iterable".as_ptr(),
                                            );
                                            if fast.is_null() {
                                                break '__join_12;
                                            } else {
                                                n = PySequence_Fast_GET_SIZE!(fast);
                                                if n != 2 {
                                                    break '__join_9;
                                                } else {
                                                    // Update/merge with this (key, value) pair.
                                                    key = PySequence_Fast_GET_ITEM!(fast, 0i32);
                                                    value = PySequence_Fast_GET_ITEM!(fast, 1i32);
                                                    Py_INCREF!(key);
                                                    Py_INCREF!(value);
                                                    if r#override != 0 {
                                                        if setitem_lock_held(
                                                            d as *mut __SlateRecord_dictobject_h_33,
                                                            key,
                                                            value,
                                                        ) < 0
                                                        {
                                                            break '__join_4;
                                                        }
                                                    } else {
                                                        if dict_setdefault_ref_lock_held(
                                                            d,
                                                            key,
                                                            value,
                                                            core::ptr::null_mut::<*mut _object>(),
                                                            0i32,
                                                        ) < 0
                                                        {
                                                            break;
                                                        }
                                                    }
                                                    Py_DECREF!(key);
                                                    Py_DECREF!(value);
                                                    Py_DECREF!(fast);
                                                    Py_DECREF!(item);
                                                    i = i.wrapping_add(1);
                                                }
                                            }
                                        }
                                    }
                                    Py_DECREF!(key);
                                    Py_DECREF!(value);
                                    break '__join_1;
                                }
                                Py_DECREF!(key);
                                Py_DECREF!(value);
                                break '__join_1;
                            }
                            PyErr_Format(PyExc_ValueError, c"dictionary update sequence element #%zd has length %zd; 2 is required".as_ptr(), i, n);
                            break '__join_1;
                        }
                        if PyErr_ExceptionMatches(PyExc_TypeError) != 0 {
                            _PyErr_FormatNote(c"Cannot convert dictionary update sequence element #%zd to a sequence".as_ptr(), i);
                            break '__join_1;
                        } else {
                            break '__join_1;
                        }
                    }
                    if PyErr_Occurred().is_null() {
                        i = 0;
                        ASSERT_CONSISTENT!();
                        break '__join_0;
                    }
                }
                Py_XDECREF!(item);
                Py_XDECREF!(fast);
                i = 1i32.wrapping_neg() as i64;
            }
            Py_DECREF!(it);
            return Py_SAFE_DOWNCAST!(i, i32);
        }
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyDict_MergeFromSeq2(
    d: *mut _object,
    seq2: *mut _object,
    r#override: i32,
) -> i32 {
    let mut res: i32 = 0;
    res = merge_from_seq2_lock_held(d, seq2, r#override);
    res
}

fn dict_dict_merge(
    interp: *mut _is,
    mp: *mut __SlateRecord_dictobject_h_33,
    other: *mut __SlateRecord_dictobject_h_33,
    mut r#override: i32,
) -> i32 {
    if other == mp || (unsafe { (*other).ma_used }) == 0 {
        return 0;
    }
    // a.update(a) or a.update({}); nothing to do
    if (unsafe { (*mp).ma_used }) == 0 {
        // Since the target dict is empty, PyDict_GetItem()
        // always returns NULL.  Setting override to 1
        // skips the unnecessary test.
        r#override = 1;
        let okeys: *mut _dictkeysobject = unsafe { (*other).ma_keys };
        // If other is clean, combined, and just allocated, just clone it.
        let __v0: bool = unsafe { (*mp).ma_values }.is_null()
            && unsafe { (*other).ma_values }.is_null()
            && (unsafe { (*other).ma_used }) == unsafe { (*okeys).dk_nentries }
            && ((DK_LOG_SIZE!(okeys) as i32) == 3
                || USABLE_FRACTION!(DK_SIZE!(okeys) / 2) < unsafe { (*other).ma_used });
        if __v0 {
            _PyDict_NotifyEvent(
                interp,
                3u32,
                mp,
                other as *mut _object,
                core::ptr::null_mut::<_object>(),
            );
            let keys: *mut _dictkeysobject = clone_combined_dict_keys(other);
            if keys.is_null() {
                return 1i32.wrapping_neg();
            }
            ensure_shared_on_resize(mp);
            dictkeys_decref(interp, unsafe { (*mp).ma_keys }, IS_DICT_SHARED!() != 0);
            set_keys(mp, keys);
            STORE_USED!(mp, unsafe { (*other).ma_used });
            ASSERT_CONSISTENT!();
            let __v2: bool =
                _PyObject_GC_IS_TRACKED__2!(other) != 0 && _PyObject_GC_IS_TRACKED__2!(mp) == 0;
            if __v2 {
                // Maintain tracking.
                _PyObject_GC_TRACK!(mp);
            }
            return 0;
        }
    }
    // Do one big resize at the start, rather than
    // incrementally resizing as we insert new items.  Expect
    // that there will be no (or few) overlapping keys.
    if USABLE_FRACTION!(DK_SIZE!(unsafe { (*mp).ma_keys })) < unsafe { (*other).ma_used } {
        let unicode: i32 = DK_IS_UNICODE!(unsafe { (*other).ma_keys }) as i32;
        if dictresize(
            interp,
            mp,
            estimate_log2_keysize(
                unsafe { (*mp).ma_used }.wrapping_add(unsafe { (*other).ma_used }),
            ),
            unicode,
        ) != 0
        {
            return 1i32.wrapping_neg();
        }
    }
    let orig_size: i64 = unsafe { (*other).ma_used };
    let mut pos: i64 = 0;
    let mut hash: i64 = 0;
    let mut key: *mut _object = unsafe { core::mem::zeroed() };
    let mut value: *mut _object = unsafe { core::mem::zeroed() };
    while _PyDict_Next(
        other as *mut _object,
        core::ptr::addr_of_mut!(pos),
        core::ptr::addr_of_mut!(key),
        core::ptr::addr_of_mut!(value),
        core::ptr::addr_of_mut!(hash),
    ) != 0
    {
        let mut err: i32 = 0;
        Py_INCREF!(key);
        Py_INCREF!(value);
        if r#override == 1 {
            err = insertdict(interp, mp, Py_NewRef!(key), hash, Py_NewRef!(value));
        } else {
            err = _PyDict_Contains_KnownHash(mp as *mut _object, key, hash);
            if err == 0 {
                err = insertdict(interp, mp, Py_NewRef!(key), hash, Py_NewRef!(value));
            } else {
                if err > 0 {
                    if r#override != 0 {
                        unsafe { _PyErr_SetKeyError(key) };
                        Py_DECREF!(value);
                        Py_DECREF!(key);
                        return 1i32.wrapping_neg();
                    }
                    err = 0;
                }
            }
        }
        Py_DECREF!(value);
        Py_DECREF!(key);
        if err != 0 {
            return 1i32.wrapping_neg();
        }
        if orig_size != unsafe { (*other).ma_used } {
            unsafe { PyErr_SetString(PyExc_RuntimeError, c"dict mutated during update".as_ptr()) };
            return 1i32.wrapping_neg();
        }
    }
    0
}

fn dict_merge(interp: *mut _is, a: *mut _object, b: *mut _object, r#override: i32) -> i32 {
    let mut status: i32 = unsafe { core::mem::zeroed() };
    let mut value: *mut _object = unsafe { core::mem::zeroed() };
    let mut key: *mut _object = unsafe { core::mem::zeroed() };
    let mut iter: *mut _object = unsafe { core::mem::zeroed() };
    let mut keys: *mut _object = unsafe { core::mem::zeroed() };
    let mut __v6: bool = unsafe { core::mem::zeroed() };
    let mut res_1: i32 = unsafe { core::mem::zeroed() };
    // We accept for the argument either a concrete dictionary object,
    // or an abstract "mapping" object.  For the former, we can do
    // things quite efficiently.  For the latter, we only require that
    // PyMapping_Keys() and PyObject_GetItem() be supported.
    let mut __v8: bool = unsafe { core::mem::zeroed() };
    let mut mp: *mut __SlateRecord_dictobject_h_33 = unsafe { core::mem::zeroed() };
    unsafe {
        if a.is_null() {
            __v8 = true;
        } else {
            __v8 = PyDict_Check!(a) == 0;
        }
        if __v8 || b.is_null() {
            PyErr_BadInternalCall__100!();
            return 1i32.wrapping_neg();
        } else {
            mp = a as *mut __SlateRecord_dictobject_h_33;
            res_1 = 0;
            if PyDict_Check!(b) != 0 {
                __v6 = (*Py_TYPE!(b)).tp_iter == Some(dict_iter);
            } else {
                __v6 = false;
            }
            if __v6 {
                let other: *mut __SlateRecord_dictobject_h_33 =
                    b as *mut __SlateRecord_dictobject_h_33;
                let res: i32 = dict_dict_merge(
                    interp,
                    a as *mut __SlateRecord_dictobject_h_33,
                    other,
                    r#override,
                );
                ASSERT_CONSISTENT!();
                return res;
            } else {
                '__join_1: {
                    // Do it the generic, slower way
                    keys = PyMapping_Keys(b);
                    if keys.is_null() {
                        // Docstring says this is equivalent to E.keys() so
                        // if E doesn't have a .keys() method we want
                        // AttributeError to percolate up.  Might as well
                        // do the same for any other error.
                        res_1 = 1i32.wrapping_neg();
                    } else {
                        iter = PyObject_GetIter(keys);
                        Py_DECREF!(keys);
                        if iter.is_null() {
                            res_1 = 1i32.wrapping_neg();
                        } else {
                            key = PyIter_Next(iter);
                            '__join_10: {
                                '__join_11: {
                                    '__join_8: {
                                        '__join_6: {
                                            while !key.is_null() {
                                                '__join_5: {
                                                    if r#override != 1 {
                                                        status = PyDict_Contains(a, key);
                                                        if status != 0 {
                                                            if status > 0 {
                                                                if r#override == 0 {
                                                                    Py_DECREF!(key);
                                                                    break '__join_5;
                                                                } else {
                                                                    break '__join_11;
                                                                }
                                                            } else {
                                                                break '__join_10;
                                                            }
                                                        }
                                                    }
                                                    value = PyObject_GetItem(b, key);
                                                    if value.is_null() {
                                                        break '__join_8;
                                                    } else {
                                                        status = setitem_lock_held(mp, key, value);
                                                        Py_DECREF!(key);
                                                        Py_DECREF!(value);
                                                        if status < 0 {
                                                            break '__join_6;
                                                        }
                                                    }
                                                }
                                                key = PyIter_Next(iter);
                                            }
                                            Py_DECREF!(iter);
                                            if !PyErr_Occurred().is_null() {
                                                // Iterator completed, via error
                                                res_1 = 1i32.wrapping_neg();
                                                break '__join_1;
                                            } else {
                                                break '__join_1;
                                            }
                                        }
                                        Py_DECREF!(iter);
                                        res_1 = 1i32.wrapping_neg();
                                        break '__join_1;
                                    }
                                    Py_DECREF!(iter);
                                    Py_DECREF!(key);
                                    res_1 = 1i32.wrapping_neg();
                                    break '__join_1;
                                }
                                _PyErr_SetKeyError(key);
                            }
                            Py_DECREF!(key);
                            Py_DECREF!(iter);
                            res_1 = 1i32.wrapping_neg();
                        }
                    }
                }
                ASSERT_CONSISTENT!();
                return res_1;
            }
        }
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyDict_Update(a: *mut _object, b: *mut _object) -> i32 {
    let interp: *mut _is = _PyInterpreterState_GET();
    dict_merge(interp, a, b, 1i32)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyDict_Merge(a: *mut _object, b: *mut _object, r#override: i32) -> i32 {
    let interp: *mut _is = _PyInterpreterState_GET();
    // XXX Deprecate override not in (0, 1).
    dict_merge(interp, a, b, (r#override != 0) as i32)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyDict_MergeEx(a: *mut _object, b: *mut _object, r#override: i32) -> i32 {
    let interp: *mut _is = _PyInterpreterState_GET();
    dict_merge(interp, a, b, r#override)
}

// [clinic input]
// dict.copy
//
// Return a shallow copy of the dict.
// [clinic start generated code]
/// [clinic end generated code: output=ffb782cf970a5c39 input=73935f042b639de4]
fn dict_copy_impl(self_: *mut __SlateRecord_dictobject_h_33) -> *mut _object {
    PyDict_Copy(self_ as *mut _object)
}

/// Copies the values, but does not change the reference
/// counts of the objects in the array.
/// Return NULL, but does *not* set an exception on failure
fn copy_values(values: *mut _dictvalues) -> *mut _dictvalues {
    let newvalues: *mut _dictvalues = new_values((unsafe { (*values).capacity }) as u64);
    if newvalues.is_null() {
        return core::ptr::null_mut::<_dictvalues>();
    }
    unsafe {
        (*newvalues).size = (*values).size;
    }
    let values_order: *mut u8 = get_insertion_order_array(values);
    let new_values_order: *mut u8 = get_insertion_order_array(newvalues);
    unsafe {
        core::ptr::copy_nonoverlapping(
            ((values_order as *const ()) as *const core::ffi::c_void) as *const u8,
            ((new_values_order as *mut ()) as *mut core::ffi::c_void) as *mut u8,
            (((*values).capacity as u64) as usize) as usize,
        )
    };
    let mut i: i32 = 0;
    while i < ((unsafe { (*values).capacity }) as i32) {
        unsafe {
            *((*newvalues).values.as_mut_ptr() as *mut *mut _object).wrapping_offset(i as isize) =
                *((*values).values.as_mut_ptr() as *mut *mut _object).wrapping_offset(i as isize);
        }
        i = i.wrapping_add(1);
    }
    newvalues
}

fn copy_lock_held(o: *mut _object) -> *mut _object {
    let interp: *mut _is = _PyInterpreterState_GET();
    let mp: *mut __SlateRecord_dictobject_h_33 = o as *mut __SlateRecord_dictobject_h_33;
    if (unsafe { (*mp).ma_used }) == 0 {
        // The dict is empty; just return a new dict.
        return PyDict_New();
    }
    if _PyDict_HasSplitTable!(mp) {
        let newvalues: *mut _dictvalues = copy_values(unsafe { (*mp).ma_values });
        if newvalues.is_null() {
            return unsafe { PyErr_NoMemory() };
        }
        let split_copy: *mut __SlateRecord_dictobject_h_33 =
            PyObject_GC_New!(__SlateRecord_dictobject_h_33, unsafe {
                core::ptr::addr_of_mut!(PyDict_Type)
            });
        if split_copy.is_null() {
            free_values(newvalues, false);
            return core::ptr::null_mut::<_object>();
        }
        let mut i: u64 = 0;
        while i < (((unsafe { (*newvalues).capacity }) as i32) as u64) {
            Py_XINCREF!(unsafe {
                *((*newvalues).values.as_mut_ptr() as *mut *mut _object).wrapping_offset(i as isize)
            });
            i = i.wrapping_add(1);
        }
        unsafe {
            (*split_copy).ma_values = newvalues;
        }
        unsafe {
            (*split_copy).ma_keys = (*mp).ma_keys;
        }
        unsafe {
            (*split_copy).ma_used = (*mp).ma_used;
        }
        unsafe {
            (*split_copy)._ma_watcher_tag = 0;
        }
        dictkeys_incref(unsafe { (*mp).ma_keys });
        _PyObject_GC_TRACK!(split_copy);
        return split_copy as *mut _object;
    }
    if (unsafe { (*Py_TYPE__2!(mp)).tp_iter }) == Some(dict_iter)
        && unsafe { (*mp).ma_values }.is_null()
        && (unsafe { (*mp).ma_used }) >= unsafe { (*(*mp).ma_keys).dk_nentries }.wrapping_mul(2) / 3
    {
        // Use fast-copy if:
        //
        // (1) type(mp) doesn't override tp_iter; and
        //
        // (2) 'mp' is not a split-dict; and
        //
        // (3) if 'mp' is non-compact ('del' operation does not resize dicts),
        //     do fast-copy only if it has at most 1/3 non-used keys.
        //
        // The last condition (3) is important to guard against a pathological
        // case when a large dict is almost emptied with multiple del/pop
        // operations and copied after that.  In cases like this, we defer to
        // PyDict_Merge, which produces a compacted copy.
        let keys: *mut _dictkeysobject = clone_combined_dict_keys(mp);
        if keys.is_null() {
            return core::ptr::null_mut::<_object>();
        }
        let new: *mut __SlateRecord_dictobject_h_33 = new_dict(
            interp,
            keys,
            core::ptr::null_mut::<_dictvalues>(),
            0i64,
            0i32,
        )
            as *mut __SlateRecord_dictobject_h_33;
        if new.is_null() {
            // In case of an error, `new_dict()` takes care of
            // cleaning up `keys`.
            return core::ptr::null_mut::<_object>();
        }
        unsafe {
            (*new).ma_used = (*mp).ma_used;
        }
        ASSERT_CONSISTENT!();
        return new as *mut _object;
    }
    let copy: *mut _object = PyDict_New();
    if copy.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    if dict_merge(interp, copy, o, 1i32) == 0 {
        return copy;
    }
    Py_DECREF!(copy);
    core::ptr::null_mut::<_object>()
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyDict_Copy(o: *mut _object) -> *mut _object {
    let __v0: bool = o.is_null() || PyDict_Check!(o) == 0;
    if __v0 {
        PyErr_BadInternalCall__112!();
        return core::ptr::null_mut::<_object>();
    }
    let res: *mut _object = copy_lock_held(o);
    res
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyDict_Size(mp: *mut _object) -> i64 {
    let __v0: bool = mp.is_null() || PyDict_Check!(mp) == 0;
    if __v0 {
        PyErr_BadInternalCall__110!();
        return 1i32.wrapping_neg() as i64;
    }
    unsafe { (*(mp as *mut __SlateRecord_dictobject_h_33)).ma_used }
}

/// Return 1 if dicts equal, 0 if not, -1 if error.
/// Gets out as soon as any difference is detected.
/// Uses only Py_EQ comparison.
fn dict_equal_lock_held(
    a: *mut __SlateRecord_dictobject_h_33,
    b: *mut __SlateRecord_dictobject_h_33,
) -> i32 {
    // Same # of entries -- check all of 'em.  Exit early on any diff.
    let mut i: i64 = 0;
    if (unsafe { (*a).ma_used }) != unsafe { (*b).ma_used } {
        return 0;
    }
    // can't be equal if # of entries differ
    while i < LOAD_KEYS_NENTRIES!(unsafe { (*a).ma_keys }) {
        '__slate_continue_0: {
            let mut key: *mut _object = unsafe { core::mem::zeroed() };
            let mut aval: *mut _object = unsafe { core::mem::zeroed() };
            let mut hash: i64 = 0;
            if DK_IS_UNICODE!(unsafe { (*a).ma_keys }) {
                let ep: *mut __SlateRecord_pycore_dict_h_90 =
                    DK_UNICODE_ENTRIES(unsafe { (*a).ma_keys }).wrapping_offset(i as isize);
                key = unsafe { (*ep).me_key };
                if key.is_null() {
                    break '__slate_continue_0;
                }
                hash = unicode_get_hash(key);
                if _PyDict_HasSplitTable!(a) {
                    aval = unsafe {
                        *((*(*a).ma_values).values.as_mut_ptr() as *mut *mut _object)
                            .wrapping_offset(i as isize)
                    };
                } else {
                    aval = unsafe { (*ep).me_value };
                }
            } else {
                let ep: *mut __SlateRecord_pycore_dict_h_85 =
                    DK_ENTRIES(unsafe { (*a).ma_keys }).wrapping_offset(i as isize);
                key = unsafe { (*ep).me_key };
                aval = unsafe { (*ep).me_value };
                hash = unsafe { (*ep).me_hash };
            }
            if !aval.is_null() {
                let mut cmp: i32 = 0;
                let mut bval: *mut _object = unsafe { core::mem::zeroed() };
                // temporarily bump aval's refcount to ensure it stays
                // alive until we're done with it
                Py_INCREF!(aval);
                // ditto for key
                Py_INCREF!(key);
                // reuse the known hash value
                _Py_dict_lookup(b, key, hash, core::ptr::addr_of_mut!(bval));
                if bval.is_null() {
                    Py_DECREF!(key);
                    Py_DECREF!(aval);
                    if !unsafe { PyErr_Occurred() }.is_null() {
                        return 1i32.wrapping_neg();
                    }
                    return 0;
                }
                Py_INCREF!(bval);
                cmp = unsafe { PyObject_RichCompareBool(aval, bval, 2i32) };
                Py_DECREF!(key);
                Py_DECREF!(aval);
                Py_DECREF!(bval);
                if cmp <= 0 {
                    return cmp;
                }
                // error or not equal
            }
        }
        i = i.wrapping_add(1);
    }
    1
}

fn dict_equal(a: *mut __SlateRecord_dictobject_h_33, b: *mut __SlateRecord_dictobject_h_33) -> i32 {
    let mut res: i32 = 0;
    res = dict_equal_lock_held(a, b);
    res
}

#[unsafe(link_section = ".text.slate_distinct.dictobject.dict_richcompare")]
extern "C-unwind" fn dict_richcompare(v: *mut _object, w: *mut _object, op: i32) -> *mut _object {
    let mut cmp: i32 = 0;
    let mut res: *mut _object = unsafe { core::mem::zeroed() };
    let __v0: bool = PyDict_Check!(v) == 0 || PyDict_Check!(w) == 0;
    if __v0 {
        res = unsafe { core::ptr::addr_of_mut!(_Py_NotImplementedStruct) };
    } else {
        if op == 2 || op == 3 {
            cmp = dict_equal(
                v as *mut __SlateRecord_dictobject_h_33,
                w as *mut __SlateRecord_dictobject_h_33,
            );
            if cmp < 0 {
                return core::ptr::null_mut::<_object>();
            }
            res = if cmp == ((op == 2) as i32) {
                _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_TrueStruct) })
            } else {
                _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_FalseStruct) })
            };
        } else {
            res = unsafe { core::ptr::addr_of_mut!(_Py_NotImplementedStruct) };
        }
    }
    Py_NewRef!(res)
}

// [clinic input]
//
// @coexist
// dict.__contains__
//
//   key: object
//   /
//
// True if the dictionary has the specified key, else False.
// [clinic start generated code]
/// [clinic end generated code: output=1b314e6da7687dae input=fe1cb42ad831e820]
fn dict___contains___impl(
    self_: *mut __SlateRecord_dictobject_h_33,
    key: *mut _object,
) -> *mut _object {
    let contains: i32 = PyDict_Contains(self_ as *mut _object, key);
    if contains < 0 {
        return core::ptr::null_mut::<_object>();
    }
    if contains != 0 {
        return _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_TrueStruct) });
    }
    _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_FalseStruct) })
}

// [clinic input]
// dict.get
//
//     key: object
//     default: object = None
//     /
//
// Return the value for key if key is in the dictionary, else default.
// [clinic start generated code]
/// [clinic end generated code: output=bba707729dee05bf input=279ddb5790b6b107]
fn dict_get_impl(
    self_: *mut __SlateRecord_dictobject_h_33,
    key: *mut _object,
    default_value: *mut _object,
) -> *mut _object {
    let mut val: *mut _object = core::ptr::null_mut::<_object>();
    let hash: i64 = _PyObject_HashFast(key);
    let mut ix: i64 = 0;
    if hash == (1i32.wrapping_neg() as i64) {
        dict_unhashable_type(key);
        return core::ptr::null_mut::<_object>();
    }
    ix = _Py_dict_lookup_threadsafe(self_, key, hash, core::ptr::addr_of_mut!(val));
    if ix == (3i32.wrapping_neg() as i64) {
        return core::ptr::null_mut::<_object>();
    }
    if ix == (1i32.wrapping_neg() as i64) || val.is_null() {
        val = Py_NewRef!(default_value);
    }
    val
}

fn dict_setdefault_ref_lock_held(
    d: *mut _object,
    key: *mut _object,
    default_value: *mut _object,
    result: *mut *mut _object,
    incref_result: i32,
) -> i32 {
    let mp: *mut __SlateRecord_dictobject_h_33 = d as *mut __SlateRecord_dictobject_h_33;
    let mut value: *mut _object = unsafe { core::mem::zeroed() };
    let mut hash: i64 = 0;
    let mut ix: i64 = 0;
    let interp: *mut _is = _PyInterpreterState_GET();
    if PyDict_Check!(d) == 0 {
        PyErr_BadInternalCall__125!();
        if !result.is_null() {
            unsafe {
                *result = core::ptr::null_mut::<_object>();
            }
        }
        return 1i32.wrapping_neg();
    }
    hash = _PyObject_HashFast(key);
    if hash == (1i32.wrapping_neg() as i64) {
        dict_unhashable_type(key);
        if !result.is_null() {
            unsafe {
                *result = core::ptr::null_mut::<_object>();
            }
        }
        return 1i32.wrapping_neg();
    }
    if (unsafe { (*mp).ma_keys })
        == unsafe {
            core::ptr::addr_of_mut!(
                *(core::ptr::addr_of_mut!(empty_keys_struct) as *mut _dictkeysobject)
            )
        }
    {
        if insert_to_emptydict(interp, mp, Py_NewRef!(key), hash, Py_NewRef!(default_value)) < 0 {
            if !result.is_null() {
                unsafe {
                    *result = core::ptr::null_mut::<_object>();
                }
            }
            return 1i32.wrapping_neg();
        }
        if !result.is_null() {
            let __v0: *mut _object = if incref_result != 0 {
                Py_NewRef!(default_value)
            } else {
                default_value
            };
            unsafe {
                *result = __v0;
            }
        }
        return 0;
    }
    let __v1: bool = _PyDict_HasSplitTable!(mp) && PyUnicode_CheckExact!(key) != 0;
    if __v1 {
        ix = insert_split_key(unsafe { (*mp).ma_keys }, key, hash);
        if ix != (1i32.wrapping_neg() as i64) {
            let mut value: *mut _object = unsafe {
                *((*(*mp).ma_values).values.as_mut_ptr() as *mut *mut _object)
                    .wrapping_offset(ix as isize)
            };
            let already_present: i32 = !value.is_null() as i32;
            if already_present == 0 {
                insert_split_value(interp, mp, key, default_value, ix);
                value = default_value;
            }
            if !result.is_null() {
                let __v2: *mut _object = if incref_result != 0 {
                    Py_NewRef!(value)
                } else {
                    value
                };
                unsafe {
                    *result = __v2;
                }
            }
            return already_present;
        }
    // No space in shared keys. Go to insert_combined_dict() below.
    } else {
        ix = _Py_dict_lookup(mp, key, hash, core::ptr::addr_of_mut!(value));
        if ix == (3i32.wrapping_neg() as i64) {
            if !result.is_null() {
                unsafe {
                    *result = core::ptr::null_mut::<_object>();
                }
            }
            return 1i32.wrapping_neg();
        }
    }
    if ix == (1i32.wrapping_neg() as i64) {
        value = default_value;
        // See comment to this function in insertdict.
        if insert_combined_dict(interp, mp, hash, Py_NewRef!(key), Py_NewRef!(value)) < 0 {
            Py_DECREF!(key);
            Py_DECREF!(value);
            if !result.is_null() {
                unsafe {
                    *result = core::ptr::null_mut::<_object>();
                }
            }
            return 1i32.wrapping_neg();
        }
        STORE_USED!(mp, unsafe { (*mp).ma_used }.wrapping_add(1));
        ASSERT_CONSISTENT!();
        if !result.is_null() {
            let __v3: *mut _object = if incref_result != 0 {
                Py_NewRef!(value)
            } else {
                value
            };
            unsafe {
                *result = __v3;
            }
        }
        return 0;
    }
    ASSERT_CONSISTENT!();
    if !result.is_null() {
        let __v4: *mut _object = if incref_result != 0 {
            Py_NewRef!(value)
        } else {
            value
        };
        unsafe {
            *result = __v4;
        }
    }
    1
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyDict_SetDefaultRef(
    d: *mut _object,
    key: *mut _object,
    default_value: *mut _object,
    result: *mut *mut _object,
) -> i32 {
    let mut res: i32 = 0;
    res = dict_setdefault_ref_lock_held(d, key, default_value, result, 1i32);
    res
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyDict_SetDefault(
    d: *mut _object,
    key: *mut _object,
    defaultobj: *mut _object,
) -> *mut _object {
    let mut result: *mut _object = unsafe { core::mem::zeroed() };
    dict_setdefault_ref_lock_held(d, key, defaultobj, core::ptr::addr_of_mut!(result), 0i32);
    result
}

// [clinic input]
// @critical_section
// dict.setdefault
//
//     key: object
//     default: object = None
//     /
//
// Insert key with a value of default if key is not in the dictionary.
//
// Return the value for key if key is in the dictionary, else default.
// [clinic start generated code]
/// [clinic end generated code: output=f8c1101ebf69e220 input=9237af9a0a224302]
fn dict_setdefault_impl(
    self_: *mut __SlateRecord_dictobject_h_33,
    key: *mut _object,
    default_value: *mut _object,
) -> *mut _object {
    let mut val: *mut _object = unsafe { core::mem::zeroed() };
    dict_setdefault_ref_lock_held(
        self_ as *mut _object,
        key,
        default_value,
        core::ptr::addr_of_mut!(val),
        1i32,
    );
    val
}

// [clinic input]
// dict.clear
//
// Remove all items from the dict.
// [clinic start generated code]
/// [clinic end generated code: output=5139a830df00830a input=0bf729baba97a4c2]
fn dict_clear_impl(self_: *mut __SlateRecord_dictobject_h_33) -> *mut _object {
    PyDict_Clear(self_ as *mut _object);
    unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
}

// [clinic input]
// dict.pop
//
//     key: object
//     default: object = NULL
//     /
//
// D.pop(k[,d]) -> v, remove specified key and return the corresponding value.
//
// If the key is not found, return the default if given; otherwise,
// raise a KeyError.
// [clinic start generated code]
/// [clinic end generated code: output=3abb47b89f24c21c input=e221baa01044c44c]
fn dict_pop_impl(
    self_: *mut __SlateRecord_dictobject_h_33,
    key: *mut _object,
    default_value: *mut _object,
) -> *mut _object {
    dict_pop_default(self_ as *mut _object, key, default_value)
}

// [clinic input]
// @critical_section
// dict.popitem
//
// Remove and return a (key, value) pair as a 2-tuple.
//
// Pairs are returned in LIFO (last-in, first-out) order.
// Raises KeyError if the dict is empty.
// [clinic start generated code]
/// [clinic end generated code: output=e65fcb04420d230d input=ef28b4da5f0f762e]
fn dict_popitem_impl(self_: *mut __SlateRecord_dictobject_h_33) -> *mut _object {
    let mut i: i64 = 0;
    let mut j: i64 = 0;
    let interp: *mut _is = _PyInterpreterState_GET();
    // Allocate the result tuple before checking the size.  Believe it
    // or not, this allocation could trigger a garbage collection which
    // could empty the dict, so if we checked the size first and that
    // happened, the result would be an infinite loop (searching for an
    // entry that no longer exists).  Note that the usual popitem()
    // idiom is "while d: k, v = d.popitem()". so needing to throw the
    // tuple away if the dict *is* empty isn't a significant
    // inefficiency -- possible, but unlikely in practice.
    let res: *mut _object = unsafe { PyTuple_New(2i64) };
    if res.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    if (unsafe { (*self_).ma_used }) == 0 {
        Py_DECREF!(res);
        unsafe { PyErr_SetString(PyExc_KeyError, c"popitem(): dictionary is empty".as_ptr()) };
        return core::ptr::null_mut::<_object>();
    }
    // Convert split table to combined table
    if _PyDict_HasSplitTable!(self_) {
        if dictresize(
            interp,
            self_,
            DK_LOG_SIZE!(unsafe { (*self_).ma_keys }),
            1i32,
        ) < 0
        {
            Py_DECREF!(res);
            return core::ptr::null_mut::<_object>();
        }
    }
    FT_ATOMIC_STORE_UINT32_RELAXED!((*unsafe { (*self_).ma_keys }).dk_version, 0i32);
    // Pop last item
    let mut key: *mut _object = unsafe { core::mem::zeroed() };
    let mut value: *mut _object = unsafe { core::mem::zeroed() };
    let mut hash: i64 = 0;
    if DK_IS_UNICODE!(unsafe { (*self_).ma_keys }) {
        let ep0: *mut __SlateRecord_pycore_dict_h_90 =
            DK_UNICODE_ENTRIES(unsafe { (*self_).ma_keys });
        i = unsafe { (*(*self_).ma_keys).dk_nentries }.wrapping_sub(1);
        while i >= 0 && unsafe { (*ep0.wrapping_offset(i as isize)).me_value }.is_null() {
            i = i.wrapping_sub(1);
        }
        key = unsafe { (*ep0.wrapping_offset(i as isize)).me_key };
        _PyDict_NotifyEvent(interp, 2u32, self_, key, core::ptr::null_mut::<_object>());
        hash = unicode_get_hash(key);
        value = unsafe { (*ep0.wrapping_offset(i as isize)).me_value };
        STORE_KEY!(ep0.wrapping_offset(i as isize));
        STORE_VALUE__2!(ep0.wrapping_offset(i as isize));
    } else {
        let ep0: *mut __SlateRecord_pycore_dict_h_85 = DK_ENTRIES(unsafe { (*self_).ma_keys });
        i = unsafe { (*(*self_).ma_keys).dk_nentries }.wrapping_sub(1);
        while i >= 0 && unsafe { (*ep0.wrapping_offset(i as isize)).me_value }.is_null() {
            i = i.wrapping_sub(1);
        }
        key = unsafe { (*ep0.wrapping_offset(i as isize)).me_key };
        _PyDict_NotifyEvent(interp, 2u32, self_, key, core::ptr::null_mut::<_object>());
        hash = unsafe { (*ep0.wrapping_offset(i as isize)).me_hash };
        value = unsafe { (*ep0.wrapping_offset(i as isize)).me_value };
        STORE_KEY!(ep0.wrapping_offset(i as isize));
        STORE_HASH!(ep0.wrapping_offset(i as isize), 1i32.wrapping_neg());
        STORE_VALUE__2!(ep0.wrapping_offset(i as isize));
    }
    j = lookdict_index(unsafe { (*self_).ma_keys }, hash, i);
    dictkeys_set_index(unsafe { (*self_).ma_keys }, j, 2i32.wrapping_neg() as i64);
    PyTuple_SET_ITEM!(res, 0i32, key);
    PyTuple_SET_ITEM!(res, 1i32, value);
    // We can't dk_usable++ since there is DKIX_DUMMY in indices
    STORE_KEYS_NENTRIES!(unsafe { (*self_).ma_keys }, i);
    STORE_USED!(self_, unsafe { (*self_).ma_used }.wrapping_sub(1));
    ASSERT_CONSISTENT!();
    res
}

#[unsafe(link_section = ".text.slate_distinct.dictobject.dict_traverse")]
extern "C-unwind" fn dict_traverse(
    op: *mut _object,
    mut visit: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
    arg: *mut (),
) -> i32 {
    let mp: *mut __SlateRecord_dictobject_h_33 = op as *mut __SlateRecord_dictobject_h_33;
    let keys: *mut _dictkeysobject = unsafe { (*mp).ma_keys };
    let mut i: i64 = 0;
    let n: i64 = unsafe { (*keys).dk_nentries };
    if DK_IS_UNICODE!(keys) {
        if _PyDict_HasSplitTable!(mp) {
            i = 0;
            while i < n {
                Py_VISIT!(
                    unsafe {
                        *((*(*mp).ma_values).values.as_mut_ptr() as *mut *mut _object)
                            .wrapping_offset(i as isize)
                    },
                    visit,
                    arg
                );
                i = i.wrapping_add(1);
            }
        } else {
            let entries: *mut __SlateRecord_pycore_dict_h_90 = DK_UNICODE_ENTRIES(keys);
            i = 0;
            while i < n {
                Py_VISIT!(
                    unsafe { (*entries.wrapping_offset(i as isize)).me_value },
                    visit,
                    arg
                );
                i = i.wrapping_add(1);
            }
        }
    } else {
        let entries: *mut __SlateRecord_pycore_dict_h_85 = DK_ENTRIES(keys);
        i = 0;
        while i < n {
            if !unsafe { (*entries.wrapping_offset(i as isize)).me_value }.is_null() {
                Py_VISIT!(
                    unsafe { (*entries.wrapping_offset(i as isize)).me_value },
                    visit,
                    arg
                );
                Py_VISIT!(
                    unsafe { (*entries.wrapping_offset(i as isize)).me_key },
                    visit,
                    arg
                );
            }
            i = i.wrapping_add(1);
        }
    }
    0
}

#[unsafe(link_section = ".text.slate_distinct.dictobject.dict_tp_clear")]
extern "C-unwind" fn dict_tp_clear(op: *mut _object) -> i32 {
    PyDict_Clear(op);
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyDict_SizeOf_LockHeld(mp: *mut __SlateRecord_dictobject_h_33) -> i64 {
    let mut res: u64 = _PyObject_SIZE(Py_TYPE__2!(mp));
    if _PyDict_HasSplitTable!(mp) {
        let __v0: u64 = shared_keys_usable_size(unsafe { (*mp).ma_keys })
            .wrapping_mul(core::mem::size_of::<*mut _object>() as u64);
        res = res.wrapping_add(__v0);
    }
    // If the dictionary is split, the keys portion is accounted-for
    // in the type object.
    if (unsafe { (*(*mp).ma_keys).dk_refcnt }) == 1 {
        let __v3: u64 = _PyDict_KeysSize(unsafe { (*mp).ma_keys });
        res = res.wrapping_add(__v3);
    }
    res as i64
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyDict_SizeOf(mp: *mut __SlateRecord_dictobject_h_33) -> i64 {
    let mut res: i64 = 0;
    res = _PyDict_SizeOf_LockHeld(mp);
    res
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyDict_KeysSize(keys: *mut _dictkeysobject) -> u64 {
    let es: u64 = if ((unsafe { (*keys).dk_kind }) as i32) == 0 {
        core::mem::size_of::<__SlateRecord_pycore_dict_h_85>() as u64
    } else {
        core::mem::size_of::<__SlateRecord_pycore_dict_h_90>() as u64
    };
    let mut size: u64 = 32;
    size = size.wrapping_add(1u64 << ((unsafe { (*keys).dk_log2_index_bytes }) as i32));
    let __v2: u64 = USABLE_FRACTION__2!(DK_SIZE!(keys) as u64).wrapping_mul(es);
    size = size.wrapping_add(__v2);
    size
}

// [clinic input]
// dict.__sizeof__
//
// Return the size of the dict in memory, in bytes.
// [clinic start generated code]
/// [clinic end generated code: output=44279379b3824bda input=4fec4ddfc44a4d1a]
fn dict___sizeof___impl(self_: *mut __SlateRecord_dictobject_h_33) -> *mut _object {
    unsafe { PyLong_FromSsize_t(_PyDict_SizeOf(self_)) }
}

#[unsafe(link_section = ".text.slate_distinct.dictobject.dict_or")]
extern "C-unwind" fn dict_or(self_: *mut _object, other: *mut _object) -> *mut _object {
    let __v0: bool = PyDict_Check!(self_) == 0 || PyDict_Check!(other) == 0;
    if __v0 {
        return unsafe { core::ptr::addr_of_mut!(_Py_NotImplementedStruct) };
    }
    let new: *mut _object = PyDict_Copy(self_);
    if new.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    if dict_update_arg(new, other) != 0 {
        Py_DECREF!(new);
        return core::ptr::null_mut::<_object>();
    }
    new
}

#[unsafe(link_section = ".text.slate_distinct.dictobject.dict_ior")]
extern "C-unwind" fn dict_ior(self_: *mut _object, other: *mut _object) -> *mut _object {
    if dict_update_arg(self_, other) != 0 {
        return core::ptr::null_mut::<_object>();
    }
    Py_NewRef!(self_)
}

static getitem__doc__: __SlateAlign16<[i8; 49]> = __SlateAlign16([
    95i8, 95i8, 103i8, 101i8, 116i8, 105i8, 116i8, 101i8, 109i8, 95i8, 95i8, 40i8, 36i8, 115i8,
    101i8, 108i8, 102i8, 44i8, 32i8, 107i8, 101i8, 121i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8,
    10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 115i8, 101i8, 108i8, 102i8, 91i8,
    107i8, 101i8, 121i8, 93i8, 46i8, 0i8,
]);

static update__doc__: __SlateAlign16<[i8; 296]> = __SlateAlign16([
    68i8, 46i8, 117i8, 112i8, 100i8, 97i8, 116i8, 101i8, 40i8, 91i8, 69i8, 44i8, 32i8, 93i8, 42i8,
    42i8, 70i8, 41i8, 32i8, 45i8, 62i8, 32i8, 78i8, 111i8, 110i8, 101i8, 46i8, 32i8, 32i8, 85i8,
    112i8, 100i8, 97i8, 116i8, 101i8, 32i8, 68i8, 32i8, 102i8, 114i8, 111i8, 109i8, 32i8, 109i8,
    97i8, 112i8, 112i8, 105i8, 110i8, 103i8, 47i8, 105i8, 116i8, 101i8, 114i8, 97i8, 98i8, 108i8,
    101i8, 32i8, 69i8, 32i8, 97i8, 110i8, 100i8, 32i8, 70i8, 46i8, 10i8, 73i8, 102i8, 32i8, 69i8,
    32i8, 105i8, 115i8, 32i8, 112i8, 114i8, 101i8, 115i8, 101i8, 110i8, 116i8, 32i8, 97i8, 110i8,
    100i8, 32i8, 104i8, 97i8, 115i8, 32i8, 97i8, 32i8, 46i8, 107i8, 101i8, 121i8, 115i8, 40i8,
    41i8, 32i8, 109i8, 101i8, 116i8, 104i8, 111i8, 100i8, 44i8, 32i8, 116i8, 104i8, 101i8, 110i8,
    32i8, 100i8, 111i8, 101i8, 115i8, 58i8, 32i8, 32i8, 102i8, 111i8, 114i8, 32i8, 107i8, 32i8,
    105i8, 110i8, 32i8, 69i8, 46i8, 107i8, 101i8, 121i8, 115i8, 40i8, 41i8, 58i8, 32i8, 68i8, 91i8,
    107i8, 93i8, 32i8, 61i8, 32i8, 69i8, 91i8, 107i8, 93i8, 10i8, 73i8, 102i8, 32i8, 69i8, 32i8,
    105i8, 115i8, 32i8, 112i8, 114i8, 101i8, 115i8, 101i8, 110i8, 116i8, 32i8, 97i8, 110i8, 100i8,
    32i8, 108i8, 97i8, 99i8, 107i8, 115i8, 32i8, 97i8, 32i8, 46i8, 107i8, 101i8, 121i8, 115i8,
    40i8, 41i8, 32i8, 109i8, 101i8, 116i8, 104i8, 111i8, 100i8, 44i8, 32i8, 116i8, 104i8, 101i8,
    110i8, 32i8, 100i8, 111i8, 101i8, 115i8, 58i8, 32i8, 32i8, 102i8, 111i8, 114i8, 32i8, 107i8,
    44i8, 32i8, 118i8, 32i8, 105i8, 110i8, 32i8, 69i8, 58i8, 32i8, 68i8, 91i8, 107i8, 93i8, 32i8,
    61i8, 32i8, 118i8, 10i8, 73i8, 110i8, 32i8, 101i8, 105i8, 116i8, 104i8, 101i8, 114i8, 32i8,
    99i8, 97i8, 115i8, 101i8, 44i8, 32i8, 116i8, 104i8, 105i8, 115i8, 32i8, 105i8, 115i8, 32i8,
    102i8, 111i8, 108i8, 108i8, 111i8, 119i8, 101i8, 100i8, 32i8, 98i8, 121i8, 58i8, 32i8, 102i8,
    111i8, 114i8, 32i8, 107i8, 32i8, 105i8, 110i8, 32i8, 70i8, 58i8, 32i8, 32i8, 68i8, 91i8, 107i8,
    93i8, 32i8, 61i8, 32i8, 70i8, 91i8, 107i8, 93i8, 0i8,
]);

// Forward
/// sentinel
static mut mapp_methods: __SlateAlign16<[PyMethodDef; 17]> = __SlateAlign16([
    {
        let mut __t0: PyMethodDef = unsafe { core::mem::zeroed() };
        __t0.ml_name = (b"__contains__\0".as_ptr() as *mut i8) as *const i8;
        __t0.ml_meth = Some(dict___contains__);
        __t0.ml_flags = 72i32;
        __t0.ml_doc = unsafe { core::ptr::addr_of!(dict___contains____doc__.0) as *const i8 };
        __t0
    },
    {
        let mut __t1: PyMethodDef = unsafe { core::mem::zeroed() };
        __t1.ml_name = (b"__getitem__\0".as_ptr() as *mut i8) as *const i8;
        __t1.ml_meth = Some(dict_subscript);
        __t1.ml_flags = 72i32;
        __t1.ml_doc = unsafe { core::ptr::addr_of!(getitem__doc__.0) as *const i8 };
        __t1
    },
    {
        let mut __t2: PyMethodDef = unsafe { core::mem::zeroed() };
        __t2.ml_name = (b"__sizeof__\0".as_ptr() as *mut i8) as *const i8;
        __t2.ml_meth = Some(dict___sizeof__);
        __t2.ml_flags = 4i32;
        __t2.ml_doc = unsafe { core::ptr::addr_of!(dict___sizeof____doc__.0) as *const i8 };
        __t2
    },
    {
        let mut __t3: PyMethodDef = unsafe { core::mem::zeroed() };
        __t3.ml_name = (b"get\0".as_ptr() as *mut i8) as *const i8;
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
                        ) -> *mut _object,
                    >,
                    Option<unsafe extern "C-unwind" fn()>,
                >(Some(dict_get))
            })
        };
        __t3.ml_flags = 128i32;
        __t3.ml_doc = unsafe { core::ptr::addr_of!(dict_get__doc__.0) as *const i8 };
        __t3
    },
    {
        let mut __t4: PyMethodDef = unsafe { core::mem::zeroed() };
        __t4.ml_name = (b"setdefault\0".as_ptr() as *mut i8) as *const i8;
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
                >(Some(dict_setdefault))
            })
        };
        __t4.ml_flags = 128i32;
        __t4.ml_doc = unsafe { core::ptr::addr_of!(dict_setdefault__doc__.0) as *const i8 };
        __t4
    },
    {
        let mut __t5: PyMethodDef = unsafe { core::mem::zeroed() };
        __t5.ml_name = (b"pop\0".as_ptr() as *mut i8) as *const i8;
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
                >(Some(dict_pop))
            })
        };
        __t5.ml_flags = 128i32;
        __t5.ml_doc = unsafe { core::ptr::addr_of!(dict_pop__doc__.0) as *const i8 };
        __t5
    },
    {
        let mut __t6: PyMethodDef = unsafe { core::mem::zeroed() };
        __t6.ml_name = (b"popitem\0".as_ptr() as *mut i8) as *const i8;
        __t6.ml_meth = Some(dict_popitem);
        __t6.ml_flags = 4i32;
        __t6.ml_doc = unsafe { core::ptr::addr_of!(dict_popitem__doc__.0) as *const i8 };
        __t6
    },
    {
        let mut __t7: PyMethodDef = unsafe { core::mem::zeroed() };
        __t7.ml_name = (b"keys\0".as_ptr() as *mut i8) as *const i8;
        __t7.ml_meth = Some(dict_keys);
        __t7.ml_flags = 4i32;
        __t7.ml_doc = unsafe { core::ptr::addr_of!(dict_keys__doc__.0) as *const i8 };
        __t7
    },
    {
        let mut __t8: PyMethodDef = unsafe { core::mem::zeroed() };
        __t8.ml_name = (b"items\0".as_ptr() as *mut i8) as *const i8;
        __t8.ml_meth = Some(dict_items);
        __t8.ml_flags = 4i32;
        __t8.ml_doc = unsafe { core::ptr::addr_of!(dict_items__doc__.0) as *const i8 };
        __t8
    },
    {
        let mut __t9: PyMethodDef = unsafe { core::mem::zeroed() };
        __t9.ml_name = (b"values\0".as_ptr() as *mut i8) as *const i8;
        __t9.ml_meth = Some(dict_values);
        __t9.ml_flags = 4i32;
        __t9.ml_doc = unsafe { core::ptr::addr_of!(dict_values__doc__.0) as *const i8 };
        __t9
    },
    {
        let mut __t10: PyMethodDef = unsafe { core::mem::zeroed() };
        __t10.ml_name = (b"update\0".as_ptr() as *mut i8) as *const i8;
        __t10.ml_meth = unsafe {
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
                >(Some(dict_update))
            })
        };
        __t10.ml_flags = 3i32;
        __t10.ml_doc = unsafe { core::ptr::addr_of!(update__doc__.0) as *const i8 };
        __t10
    },
    {
        let mut __t11: PyMethodDef = unsafe { core::mem::zeroed() };
        __t11.ml_name = (b"fromkeys\0".as_ptr() as *mut i8) as *const i8;
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
                >(Some(dict_fromkeys))
            })
        };
        __t11.ml_flags = 144i32;
        __t11.ml_doc = unsafe { core::ptr::addr_of!(dict_fromkeys__doc__.0) as *const i8 };
        __t11
    },
    {
        let mut __t12: PyMethodDef = unsafe { core::mem::zeroed() };
        __t12.ml_name = (b"clear\0".as_ptr() as *mut i8) as *const i8;
        __t12.ml_meth = Some(dict_clear);
        __t12.ml_flags = 4i32;
        __t12.ml_doc = unsafe { core::ptr::addr_of!(dict_clear__doc__.0) as *const i8 };
        __t12
    },
    {
        let mut __t13: PyMethodDef = unsafe { core::mem::zeroed() };
        __t13.ml_name = (b"copy\0".as_ptr() as *mut i8) as *const i8;
        __t13.ml_meth = Some(dict_copy);
        __t13.ml_flags = 4i32;
        __t13.ml_doc = unsafe { core::ptr::addr_of!(dict_copy__doc__.0) as *const i8 };
        __t13
    },
    {
        let mut __t14: PyMethodDef = unsafe { core::mem::zeroed() };
        __t14.ml_name = (b"__reversed__\0".as_ptr() as *mut i8) as *const i8;
        __t14.ml_meth = Some(dict___reversed__);
        __t14.ml_flags = 4i32;
        __t14.ml_doc = unsafe { core::ptr::addr_of!(dict___reversed____doc__.0) as *const i8 };
        __t14
    },
    {
        let mut __t15: PyMethodDef = unsafe { core::mem::zeroed() };
        __t15.ml_name = (b"__class_getitem__\0".as_ptr() as *mut i8) as *const i8;
        __t15.ml_meth = unsafe {
            core::mem::transmute::<
                *const (),
                Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
            >(Py_GenericAlias as *const ())
        };
        __t15.ml_flags = 24i32;
        __t15.ml_doc = (b"dicts are generic over two types, signifying (respectively) the types of their keys and values\0".as_ptr() as *mut i8) as *const i8;
        __t15
    },
    {
        let mut __t16: PyMethodDef = unsafe { core::mem::zeroed() };
        __t16.ml_name = core::ptr::null::<i8>();
        __t16.ml_meth = None;
        __t16
    },
]);

/// Return 1 if `key` is in dict `op`, 0 if not, and -1 on error.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.dictobject.PyDict_Contains")]
extern "C-unwind" fn PyDict_Contains(op: *mut _object, key: *mut _object) -> i32 {
    let hash: i64 = _PyObject_HashFast(key);
    if hash == (1i32.wrapping_neg() as i64) {
        dict_unhashable_type(key);
        return 1i32.wrapping_neg();
    }
    _PyDict_Contains_KnownHash(op, key, hash)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyDict_ContainsString(op: *mut _object, key: *const i8) -> i32 {
    let key_obj: *mut _object = unsafe { PyUnicode_FromString(key) };
    if key_obj.is_null() {
        return 1i32.wrapping_neg();
    }
    let res: i32 = PyDict_Contains(op, key_obj);
    Py_DECREF!(key_obj);
    res
}

/// Internal version of PyDict_Contains used when the hash value is already known
#[unsafe(no_mangle)]
extern "C-unwind" fn _PyDict_Contains_KnownHash(
    op: *mut _object,
    key: *mut _object,
    hash: i64,
) -> i32 {
    let mp: *mut __SlateRecord_dictobject_h_33 = op as *mut __SlateRecord_dictobject_h_33;
    let mut value: *mut _object = unsafe { core::mem::zeroed() };
    let ix: i64 = _Py_dict_lookup(mp, key, hash, core::ptr::addr_of_mut!(value));
    if ix == (3i32.wrapping_neg() as i64) {
        return 1i32.wrapping_neg();
    }
    if ix != (1i32.wrapping_neg() as i64) && !value.is_null() {
        return 1;
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyDict_ContainsId(op: *mut _object, key: *mut _Py_Identifier) -> i32 {
    let kv: *mut _object = unsafe { _PyUnicode_FromId(key) }; // borrowed
    if kv.is_null() {
        return 1i32.wrapping_neg();
    }
    PyDict_Contains(op, kv)
}

/// Hack to implement "key in dict"
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
static mut dict_as_sequence: __SlateRecord_object_h_120 = __SlateRecord_object_h_120 {
    sq_length: None,
    sq_concat: None,
    sq_repeat: None,
    sq_item: None,
    was_sq_slice: core::ptr::null_mut::<()>(),
    sq_ass_item: None,
    was_sq_ass_slice: core::ptr::null_mut::<()>(),
    sq_contains: Some(PyDict_Contains),
    sq_inplace_concat: None,
    sq_inplace_repeat: None,
};

static mut dict_as_number: __SlateRecord_object_h_106 = {
    let mut __t0: __SlateRecord_object_h_106 = unsafe { core::mem::zeroed() };
    __t0.nb_or = Some(dict_or);
    __t0.nb_inplace_or = Some(dict_ior);
    __t0
};

#[unsafe(link_section = ".text.slate_distinct.dictobject.dict_new")]
extern "C-unwind" fn dict_new(
    r#type: *mut _typeobject,
    args: *mut _object,
    kwds: *mut _object,
) -> *mut _object {
    // dict subclasses must implement the GC protocol
    let self_: *mut _object = unsafe { unsafe { (*r#type).tp_alloc }.unwrap()(r#type, 0i64) };
    if self_.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let d: *mut __SlateRecord_dictobject_h_33 = self_ as *mut __SlateRecord_dictobject_h_33;
    unsafe {
        (*d).ma_used = 0;
    }
    unsafe {
        (*d)._ma_watcher_tag = 0;
    }
    // We don't inc ref empty keys because they're immortal
    unsafe {
        (*d).ma_keys = core::ptr::addr_of_mut!(
            *(core::ptr::addr_of_mut!(empty_keys_struct) as *mut _dictkeysobject)
        );
    }
    unsafe {
        (*d).ma_values = core::ptr::null_mut::<_dictvalues>();
    }
    ASSERT_CONSISTENT!();
    if _PyObject_GC_IS_TRACKED__2!(d) == 0 {
        _PyObject_GC_TRACK!(d);
    }
    self_
}

#[unsafe(link_section = ".text.slate_distinct.dictobject.dict_init")]
extern "C-unwind" fn dict_init(self_: *mut _object, args: *mut _object, kwds: *mut _object) -> i32 {
    dict_update_common(self_, args, kwds, c"dict".as_ptr())
}

#[unsafe(link_section = ".text.slate_distinct.dictobject.dict_vectorcall")]
extern "C-unwind" fn dict_vectorcall(
    r#type: *mut _object,
    mut args: *const *mut _object,
    nargsf: u64,
    kwnames: *mut _object,
) -> *mut _object {
    let nargs: i64 = PyVectorcall_NARGS!(nargsf);
    if !_PyArg_CheckPositional!(c"dict".as_ptr() as *mut i8, nargs, 0i32, 1i32) {
        return core::ptr::null_mut::<_object>();
    }
    let self_: *mut _object = dict_new(
        _PyType_CAST!(r#type),
        core::ptr::null_mut::<_object>(),
        core::ptr::null_mut::<_object>(),
    );
    if self_.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    if nargs == 1 {
        if dict_update_arg(self_, unsafe { *args.wrapping_offset(0 as isize) }) < 0 {
            Py_DECREF!(self_);
            return core::ptr::null_mut::<_object>();
        }
        let __v1: *const *mut _object = args;
        args = __v1.wrapping_offset(1 as isize);
    }
    if !kwnames.is_null() {
        let mut i: i64 = 0;
        while i < PyTuple_GET_SIZE!(kwnames) {
            if PyDict_SetItem(self_, PyTuple_GET_ITEM!(kwnames, i), unsafe {
                *args.wrapping_offset(i as isize)
            }) < 0
            {
                Py_DECREF!(self_);
                return core::ptr::null_mut::<_object>();
            }
            i = i.wrapping_add(1);
        }
    }
    self_
}

#[unsafe(link_section = ".text.slate_distinct.dictobject.dict_iter")]
extern "C-unwind" fn dict_iter(self_: *mut _object) -> *mut _object {
    let dict: *mut __SlateRecord_dictobject_h_33 = self_ as *mut __SlateRecord_dictobject_h_33;
    dictiter_new(dict, unsafe { core::ptr::addr_of_mut!(PyDictIterKey_Type) })
}

static dictionary_doc: __SlateAlign16<[i8; 371]> = __SlateAlign16([
    100i8, 105i8, 99i8, 116i8, 40i8, 41i8, 32i8, 45i8, 62i8, 32i8, 110i8, 101i8, 119i8, 32i8,
    101i8, 109i8, 112i8, 116i8, 121i8, 32i8, 100i8, 105i8, 99i8, 116i8, 105i8, 111i8, 110i8, 97i8,
    114i8, 121i8, 10i8, 100i8, 105i8, 99i8, 116i8, 40i8, 109i8, 97i8, 112i8, 112i8, 105i8, 110i8,
    103i8, 41i8, 32i8, 45i8, 62i8, 32i8, 110i8, 101i8, 119i8, 32i8, 100i8, 105i8, 99i8, 116i8,
    105i8, 111i8, 110i8, 97i8, 114i8, 121i8, 32i8, 105i8, 110i8, 105i8, 116i8, 105i8, 97i8, 108i8,
    105i8, 122i8, 101i8, 100i8, 32i8, 102i8, 114i8, 111i8, 109i8, 32i8, 97i8, 32i8, 109i8, 97i8,
    112i8, 112i8, 105i8, 110i8, 103i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 39i8, 115i8,
    10i8, 32i8, 32i8, 32i8, 32i8, 40i8, 107i8, 101i8, 121i8, 44i8, 32i8, 118i8, 97i8, 108i8, 117i8,
    101i8, 41i8, 32i8, 112i8, 97i8, 105i8, 114i8, 115i8, 10i8, 100i8, 105i8, 99i8, 116i8, 40i8,
    105i8, 116i8, 101i8, 114i8, 97i8, 98i8, 108i8, 101i8, 41i8, 32i8, 45i8, 62i8, 32i8, 110i8,
    101i8, 119i8, 32i8, 100i8, 105i8, 99i8, 116i8, 105i8, 111i8, 110i8, 97i8, 114i8, 121i8, 32i8,
    105i8, 110i8, 105i8, 116i8, 105i8, 97i8, 108i8, 105i8, 122i8, 101i8, 100i8, 32i8, 97i8, 115i8,
    32i8, 105i8, 102i8, 32i8, 118i8, 105i8, 97i8, 58i8, 10i8, 32i8, 32i8, 32i8, 32i8, 100i8, 32i8,
    61i8, 32i8, 123i8, 125i8, 10i8, 32i8, 32i8, 32i8, 32i8, 102i8, 111i8, 114i8, 32i8, 107i8, 44i8,
    32i8, 118i8, 32i8, 105i8, 110i8, 32i8, 105i8, 116i8, 101i8, 114i8, 97i8, 98i8, 108i8, 101i8,
    58i8, 10i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 32i8, 100i8, 91i8, 107i8, 93i8, 32i8,
    61i8, 32i8, 118i8, 10i8, 100i8, 105i8, 99i8, 116i8, 40i8, 42i8, 42i8, 107i8, 119i8, 97i8,
    114i8, 103i8, 115i8, 41i8, 32i8, 45i8, 62i8, 32i8, 110i8, 101i8, 119i8, 32i8, 100i8, 105i8,
    99i8, 116i8, 105i8, 111i8, 110i8, 97i8, 114i8, 121i8, 32i8, 105i8, 110i8, 105i8, 116i8, 105i8,
    97i8, 108i8, 105i8, 122i8, 101i8, 100i8, 32i8, 119i8, 105i8, 116i8, 104i8, 32i8, 116i8, 104i8,
    101i8, 32i8, 110i8, 97i8, 109i8, 101i8, 61i8, 118i8, 97i8, 108i8, 117i8, 101i8, 32i8, 112i8,
    97i8, 105i8, 114i8, 115i8, 10i8, 32i8, 32i8, 32i8, 32i8, 105i8, 110i8, 32i8, 116i8, 104i8,
    101i8, 32i8, 107i8, 101i8, 121i8, 119i8, 111i8, 114i8, 100i8, 32i8, 97i8, 114i8, 103i8, 117i8,
    109i8, 101i8, 110i8, 116i8, 32i8, 108i8, 105i8, 115i8, 116i8, 46i8, 32i8, 32i8, 70i8, 111i8,
    114i8, 32i8, 101i8, 120i8, 97i8, 109i8, 112i8, 108i8, 101i8, 58i8, 32i8, 32i8, 100i8, 105i8,
    99i8, 116i8, 40i8, 111i8, 110i8, 101i8, 61i8, 49i8, 44i8, 32i8, 116i8, 119i8, 111i8, 61i8,
    50i8, 41i8, 0i8,
]);

// For backward compatibility with old dictionary interface
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
#[unsafe(no_mangle)]
extern "C-unwind" fn PyDict_GetItemString(v: *mut _object, key: *const i8) -> *mut _object {
    let kv: *mut _object = unsafe { PyUnicode_FromString(key) };
    if kv.is_null() {
        unsafe {
            PyErr_FormatUnraisable(c"Exception ignored in PyDict_GetItemString(); consider using PyDict_GetItemStringRef()".as_ptr())
        };
        return core::ptr::null_mut::<_object>();
    }
    let rv: *mut _object = dict_getitem(
        v,
        kv,
        c"Exception ignored in PyDict_GetItemString(); consider using PyDict_GetItemStringRef()"
            .as_ptr(),
    );
    Py_DECREF!(kv);
    return rv; // borrowed reference
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyDict_GetItemStringRef(
    v: *mut _object,
    key: *const i8,
    result: *mut *mut _object,
) -> i32 {
    let key_obj: *mut _object = unsafe { PyUnicode_FromString(key) };
    if key_obj.is_null() {
        unsafe {
            *result = core::ptr::null_mut::<_object>();
        }
        return 1i32.wrapping_neg();
    }
    let res: i32 = PyDict_GetItemRef(v, key_obj, result);
    Py_DECREF!(key_obj);
    res
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyDict_SetItemId(
    v: *mut _object,
    key: *mut _Py_Identifier,
    item: *mut _object,
) -> i32 {
    let kv: *mut _object = unsafe { _PyUnicode_FromId(key) }; // borrowed
    if kv.is_null() {
        return 1i32.wrapping_neg();
    }
    PyDict_SetItem(v, kv, item)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyDict_SetItemString(
    v: *mut _object,
    key: *const i8,
    item: *mut _object,
) -> i32 {
    let mut kv: *mut _object = unsafe { core::mem::zeroed() };
    let mut err: i32 = 0;
    kv = unsafe { PyUnicode_FromString(key) };
    if kv.is_null() {
        return 1i32.wrapping_neg();
    }
    let interp: *mut _is = _PyInterpreterState_GET();
    unsafe { _PyUnicode_InternImmortal(interp, core::ptr::addr_of_mut!(kv)) }; // XXX Should we really?
    err = PyDict_SetItem(v, kv, item);
    Py_DECREF!(kv);
    err
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyDict_DelItemId(v: *mut _object, key: *mut _Py_Identifier) -> i32 {
    let kv: *mut _object = unsafe { _PyUnicode_FromId(key) }; // borrowed
    if kv.is_null() {
        return 1i32.wrapping_neg();
    }
    PyDict_DelItem(v, kv)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyDict_DelItemString(v: *mut _object, key: *const i8) -> i32 {
    let mut err: i32 = 0;
    let kv: *mut _object = unsafe { PyUnicode_FromString(key) };
    if kv.is_null() {
        return 1i32.wrapping_neg();
    }
    err = PyDict_DelItem(v, kv);
    Py_DECREF!(kv);
    err
}

// Dictionary iterator types
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_dictobject_c_5130 {
    ob_base: _object,
    /// Set to NULL when iterator is exhausted
    di_dict: *mut __SlateRecord_dictobject_h_33,
    di_used: i64,
    di_pos: i64,
    /// reusable result tuple for iteritems
    di_result: *mut _object,
    len: i64,
}

fn dictiter_new(
    dict: *mut __SlateRecord_dictobject_h_33,
    itertype: *mut _typeobject,
) -> *mut _object {
    let mut used: i64 = 0;
    let di: *mut __SlateRecord_dictobject_c_5130 =
        PyObject_GC_New!(__SlateRecord_dictobject_c_5130, itertype);
    if di.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    unsafe {
        (*di).di_dict = Py_NewRef__2!(dict) as *mut __SlateRecord_dictobject_h_33;
    }
    used = unsafe { (*dict).ma_used };
    unsafe {
        (*di).di_used = used;
    }
    unsafe {
        (*di).len = used;
    }
    if itertype == unsafe { core::ptr::addr_of_mut!(PyDictRevIterKey_Type) }
        || itertype == unsafe { core::ptr::addr_of_mut!(PyDictRevIterItem_Type) }
        || itertype == unsafe { core::ptr::addr_of_mut!(PyDictRevIterValue_Type) }
    {
        if _PyDict_HasSplitTable!(dict) {
            unsafe {
                (*di).di_pos = used.wrapping_sub(1);
            }
        } else {
            unsafe {
                (*di).di_pos = load_keys_nentries(dict).wrapping_sub(1);
            }
        }
    } else {
        unsafe {
            (*di).di_pos = 0;
        }
    }
    if itertype == unsafe { core::ptr::addr_of_mut!(PyDictIterItem_Type) }
        || itertype == unsafe { core::ptr::addr_of_mut!(PyDictRevIterItem_Type) }
    {
        unsafe {
            (*di).di_result = PyTuple_Pack(
                2i64,
                core::ptr::addr_of_mut!(_Py_NoneStruct),
                core::ptr::addr_of_mut!(_Py_NoneStruct),
            );
        }
        if unsafe { (*di).di_result }.is_null() {
            Py_DECREF__2!(di);
            return core::ptr::null_mut::<_object>();
        }
    } else {
        unsafe {
            (*di).di_result = core::ptr::null_mut::<_object>();
        }
    }
    _PyObject_GC_TRACK!(di);
    di as *mut _object
}

#[unsafe(link_section = ".text.slate_distinct.dictobject.dictiter_dealloc")]
extern "C-unwind" fn dictiter_dealloc(self_: *mut _object) {
    let di: *mut __SlateRecord_dictobject_c_5130 = self_ as *mut __SlateRecord_dictobject_c_5130;
    // bpo-31095: UnTrack is needed before calling any callbacks
    _PyObject_GC_UNTRACK__2!(di);
    Py_XDECREF__2!(unsafe { (*di).di_dict });
    Py_XDECREF!(unsafe { (*di).di_result });
    unsafe { PyObject_GC_Del(di as *mut ()) };
}

#[unsafe(link_section = ".text.slate_distinct.dictobject.dictiter_traverse")]
extern "C-unwind" fn dictiter_traverse(
    self_: *mut _object,
    mut visit: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
    arg: *mut (),
) -> i32 {
    let di: *mut __SlateRecord_dictobject_c_5130 = self_ as *mut __SlateRecord_dictobject_c_5130;
    Py_VISIT__4!(unsafe { (*di).di_dict }, visit, arg);
    Py_VISIT!(unsafe { (*di).di_result }, visit, arg);
    0
}

#[unsafe(link_section = ".text.slate_distinct.dictobject.dictiter_len")]
extern "C-unwind" fn dictiter_len(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    let mut len: i64 = 0;
    if !unsafe { (*(self_ as *mut __SlateRecord_dictobject_c_5130)).di_dict }.is_null()
        && (unsafe { (*(self_ as *mut __SlateRecord_dictobject_c_5130)).di_used })
            == unsafe { (*(*(self_ as *mut __SlateRecord_dictobject_c_5130)).di_dict).ma_used }
    {
        len = unsafe { (*(self_ as *mut __SlateRecord_dictobject_c_5130)).len };
    }
    unsafe { PyLong_FromSize_t(len as u64) }
}

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

/// sentinel
static mut dictiter_methods: __SlateAlign16<[PyMethodDef; 3]> = __SlateAlign16([
    {
        let mut __t0: PyMethodDef = unsafe { core::mem::zeroed() };
        __t0.ml_name = (b"__length_hint__\0".as_ptr() as *mut i8) as *const i8;
        __t0.ml_meth = Some(dictiter_len);
        __t0.ml_flags = 4i32;
        __t0.ml_doc = unsafe { core::ptr::addr_of!(length_hint_doc.0) as *const i8 };
        __t0
    },
    {
        let mut __t1: PyMethodDef = unsafe { core::mem::zeroed() };
        __t1.ml_name = (b"__reduce__\0".as_ptr() as *mut i8) as *const i8;
        __t1.ml_meth = Some(dictiter_reduce);
        __t1.ml_flags = 4i32;
        __t1.ml_doc = unsafe { core::ptr::addr_of!(reduce_doc.0) as *const i8 };
        __t1
    },
    {
        let mut __t2: PyMethodDef = unsafe { core::mem::zeroed() };
        __t2.ml_name = core::ptr::null::<i8>();
        __t2.ml_meth = None;
        __t2
    },
]);

fn dictiter_iternextkey_lock_held(
    d: *mut __SlateRecord_dictobject_h_33,
    self_: *mut _object,
) -> *mut _object {
    let mut entry_ptr: *mut __SlateRecord_pycore_dict_h_90 = unsafe { core::mem::zeroed() };
    let mut entry_ptr_1: *mut __SlateRecord_pycore_dict_h_85 = unsafe { core::mem::zeroed() };
    let mut n: i64 = unsafe { core::mem::zeroed() };
    let mut k: *mut _dictkeysobject = unsafe { core::mem::zeroed() };
    let mut i: i64 = unsafe { core::mem::zeroed() };
    let mut key: *mut _object = unsafe { core::mem::zeroed() };
    let mut di: *mut __SlateRecord_dictobject_c_5130 = unsafe { core::mem::zeroed() };
    unsafe {
        di = self_ as *mut __SlateRecord_dictobject_c_5130;
        if (*di).di_used != (*d).ma_used {
            PyErr_SetString(
                PyExc_RuntimeError,
                c"dictionary changed size during iteration".as_ptr(),
            );
            (*di).di_used = 1i32.wrapping_neg() as i64;
            // Make this state sticky
            return core::ptr::null_mut::<_object>();
        } else {
            '__join_0: {
                i = (*di).di_pos;
                k = (*d).ma_keys;
                if _PyDict_HasSplitTable!(d) {
                    if i >= (*d).ma_used {
                        break '__join_0;
                    } else {
                        let index: i32 = get_index_from_order(d, i);
                        key = (*DK_UNICODE_ENTRIES(k).wrapping_offset(index as isize)).me_key;
                    }
                } else {
                    n = (*k).dk_nentries;
                    if DK_IS_UNICODE!(k) {
                        entry_ptr = DK_UNICODE_ENTRIES(k).wrapping_offset(i as isize);
                        while i < n && unsafe { (*entry_ptr).me_value }.is_null() {
                            let __v7: *mut __SlateRecord_pycore_dict_h_90 = entry_ptr;
                            entry_ptr = __v7.wrapping_offset(1 as isize);
                            i = i.wrapping_add(1);
                        }
                        if i >= n {
                            break '__join_0;
                        } else {
                            key = (*entry_ptr).me_key;
                        }
                    } else {
                        entry_ptr_1 = DK_ENTRIES(k).wrapping_offset(i as isize);
                        while i < n && unsafe { (*entry_ptr_1).me_value }.is_null() {
                            let __v12: *mut __SlateRecord_pycore_dict_h_85 = entry_ptr_1;
                            entry_ptr_1 = __v12.wrapping_offset(1 as isize);
                            i = i.wrapping_add(1);
                        }
                        if i >= n {
                            break '__join_0;
                        } else {
                            key = (*entry_ptr_1).me_key;
                        }
                    }
                }
                // We found an element (key), but did not expect it
                if (*di).len == 0 {
                    PyErr_SetString(
                        PyExc_RuntimeError,
                        c"dictionary keys changed during iteration".as_ptr(),
                    );
                } else {
                    (*di).di_pos = i.wrapping_add(1);
                    let __v1: i64 = (*di).len;
                    (*di).len = __v1.wrapping_sub(1);
                    return Py_NewRef!(key);
                }
            }
            (*di).di_dict = core::ptr::null_mut::<__SlateRecord_dictobject_h_33>();
            Py_DECREF__2!(d);
            return core::ptr::null_mut::<_object>();
        }
    }
}

#[unsafe(link_section = ".text.slate_distinct.dictobject.dictiter_iternextkey")]
extern "C-unwind" fn dictiter_iternextkey(self_: *mut _object) -> *mut _object {
    let d: *mut __SlateRecord_dictobject_h_33 =
        unsafe { (*(self_ as *mut __SlateRecord_dictobject_c_5130)).di_dict };
    if d.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let value: *mut _object = dictiter_iternextkey_lock_held(d, self_);
    value
}

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
fn dictiter_iternextvalue_lock_held(
    d: *mut __SlateRecord_dictobject_h_33,
    self_: *mut _object,
) -> *mut _object {
    let mut entry_ptr: *mut __SlateRecord_pycore_dict_h_90 = unsafe { core::mem::zeroed() };
    let mut entry_ptr_1: *mut __SlateRecord_pycore_dict_h_85 = unsafe { core::mem::zeroed() };
    let mut n: i64 = unsafe { core::mem::zeroed() };
    let mut i: i64 = unsafe { core::mem::zeroed() };
    let mut value: *mut _object = unsafe { core::mem::zeroed() };
    let mut di: *mut __SlateRecord_dictobject_c_5130 = unsafe { core::mem::zeroed() };
    unsafe {
        di = self_ as *mut __SlateRecord_dictobject_c_5130;
        if (*di).di_used != (*d).ma_used {
            PyErr_SetString(
                PyExc_RuntimeError,
                c"dictionary changed size during iteration".as_ptr(),
            );
            (*di).di_used = 1i32.wrapping_neg() as i64;
            // Make this state sticky
            return core::ptr::null_mut::<_object>();
        } else {
            '__join_0: {
                i = (*di).di_pos;
                if _PyDict_HasSplitTable!(d) {
                    if i >= (*d).ma_used {
                        break '__join_0;
                    } else {
                        let index: i32 = get_index_from_order(d, i);
                        value = *((*(*d).ma_values).values.as_mut_ptr() as *mut *mut _object)
                            .wrapping_offset(index as isize);
                    }
                } else {
                    n = (*(*d).ma_keys).dk_nentries;
                    if DK_IS_UNICODE!(unsafe { (*d).ma_keys }) {
                        entry_ptr = DK_UNICODE_ENTRIES((*d).ma_keys).wrapping_offset(i as isize);
                        while i < n && unsafe { (*entry_ptr).me_value }.is_null() {
                            let __v7: *mut __SlateRecord_pycore_dict_h_90 = entry_ptr;
                            entry_ptr = __v7.wrapping_offset(1 as isize);
                            i = i.wrapping_add(1);
                        }
                        if i >= n {
                            break '__join_0;
                        } else {
                            value = (*entry_ptr).me_value;
                        }
                    } else {
                        entry_ptr_1 = DK_ENTRIES((*d).ma_keys).wrapping_offset(i as isize);
                        while i < n && unsafe { (*entry_ptr_1).me_value }.is_null() {
                            let __v12: *mut __SlateRecord_pycore_dict_h_85 = entry_ptr_1;
                            entry_ptr_1 = __v12.wrapping_offset(1 as isize);
                            i = i.wrapping_add(1);
                        }
                        if i >= n {
                            break '__join_0;
                        } else {
                            value = (*entry_ptr_1).me_value;
                        }
                    }
                }
                // We found an element, but did not expect it
                if (*di).len == 0 {
                    PyErr_SetString(
                        PyExc_RuntimeError,
                        c"dictionary keys changed during iteration".as_ptr(),
                    );
                } else {
                    (*di).di_pos = i.wrapping_add(1);
                    let __v1: i64 = (*di).len;
                    (*di).len = __v1.wrapping_sub(1);
                    return Py_NewRef!(value);
                }
            }
            (*di).di_dict = core::ptr::null_mut::<__SlateRecord_dictobject_h_33>();
            Py_DECREF__2!(d);
            return core::ptr::null_mut::<_object>();
        }
    }
}

#[unsafe(link_section = ".text.slate_distinct.dictobject.dictiter_iternextvalue")]
extern "C-unwind" fn dictiter_iternextvalue(self_: *mut _object) -> *mut _object {
    let d: *mut __SlateRecord_dictobject_h_33 =
        unsafe { (*(self_ as *mut __SlateRecord_dictobject_c_5130)).di_dict };
    if d.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let value: *mut _object = dictiter_iternextvalue_lock_held(d, self_);
    value
}

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
fn dictiter_iternextitem_lock_held(
    d: *mut __SlateRecord_dictobject_h_33,
    self_: *mut _object,
    out_key: *mut *mut _object,
    out_value: *mut *mut _object,
) -> i32 {
    let mut entry_ptr: *mut __SlateRecord_pycore_dict_h_90 = unsafe { core::mem::zeroed() };
    let mut entry_ptr_1: *mut __SlateRecord_pycore_dict_h_85 = unsafe { core::mem::zeroed() };
    let mut n: i64 = unsafe { core::mem::zeroed() };
    let mut i: i64 = unsafe { core::mem::zeroed() };
    let mut value: *mut _object = unsafe { core::mem::zeroed() };
    let mut key: *mut _object = unsafe { core::mem::zeroed() };
    let mut di: *mut __SlateRecord_dictobject_c_5130 = unsafe { core::mem::zeroed() };
    unsafe {
        di = self_ as *mut __SlateRecord_dictobject_c_5130;
        if (*di).di_used != (*d).ma_used {
            PyErr_SetString(
                PyExc_RuntimeError,
                c"dictionary changed size during iteration".as_ptr(),
            );
            (*di).di_used = 1i32.wrapping_neg() as i64;
            // Make this state sticky
            return 1i32.wrapping_neg();
        } else {
            '__join_0: {
                i = (*di).di_pos;
                if _PyDict_HasSplitTable!(d) {
                    if i >= (*d).ma_used {
                        break '__join_0;
                    } else {
                        let index: i32 = get_index_from_order(d, i);
                        key = (*DK_UNICODE_ENTRIES((*d).ma_keys).wrapping_offset(index as isize))
                            .me_key;
                        value = *((*(*d).ma_values).values.as_mut_ptr() as *mut *mut _object)
                            .wrapping_offset(index as isize);
                    }
                } else {
                    n = (*(*d).ma_keys).dk_nentries;
                    if DK_IS_UNICODE!(unsafe { (*d).ma_keys }) {
                        entry_ptr = DK_UNICODE_ENTRIES((*d).ma_keys).wrapping_offset(i as isize);
                        while i < n && unsafe { (*entry_ptr).me_value }.is_null() {
                            let __v7: *mut __SlateRecord_pycore_dict_h_90 = entry_ptr;
                            entry_ptr = __v7.wrapping_offset(1 as isize);
                            i = i.wrapping_add(1);
                        }
                        if i >= n {
                            break '__join_0;
                        } else {
                            key = (*entry_ptr).me_key;
                            value = (*entry_ptr).me_value;
                        }
                    } else {
                        entry_ptr_1 = DK_ENTRIES((*d).ma_keys).wrapping_offset(i as isize);
                        while i < n && unsafe { (*entry_ptr_1).me_value }.is_null() {
                            let __v12: *mut __SlateRecord_pycore_dict_h_85 = entry_ptr_1;
                            entry_ptr_1 = __v12.wrapping_offset(1 as isize);
                            i = i.wrapping_add(1);
                        }
                        if i >= n {
                            break '__join_0;
                        } else {
                            key = (*entry_ptr_1).me_key;
                            value = (*entry_ptr_1).me_value;
                        }
                    }
                }
                // We found an element, but did not expect it
                if (*di).len == 0 {
                    PyErr_SetString(
                        PyExc_RuntimeError,
                        c"dictionary keys changed during iteration".as_ptr(),
                    );
                } else {
                    (*di).di_pos = i.wrapping_add(1);
                    let __v1: i64 = (*di).len;
                    (*di).len = __v1.wrapping_sub(1);
                    if !out_key.is_null() {
                        *out_key = Py_NewRef!(key);
                    }
                    if !out_value.is_null() {
                        *out_value = Py_NewRef!(value);
                    }
                    return 0;
                }
            }
            (*di).di_dict = core::ptr::null_mut::<__SlateRecord_dictobject_h_33>();
            Py_DECREF__2!(d);
            return 1i32.wrapping_neg();
        }
    }
}

fn acquire_iter_result(result: *mut _object) -> bool {
    if _PyObject_IsUniquelyReferenced(result) != 0 {
        Py_INCREF!(result);
        return true;
    }
    false
}

#[unsafe(link_section = ".text.slate_distinct.dictobject.dictiter_iternextitem")]
extern "C-unwind" fn dictiter_iternextitem(self_: *mut _object) -> *mut _object {
    let d: *mut __SlateRecord_dictobject_h_33 =
        unsafe { (*(self_ as *mut __SlateRecord_dictobject_c_5130)).di_dict };
    if d.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let mut key: *mut _object = unsafe { core::mem::zeroed() };
    let mut value: *mut _object = unsafe { core::mem::zeroed() };
    if dictiter_iternextitem_lock_held(
        d,
        self_,
        core::ptr::addr_of_mut!(key),
        core::ptr::addr_of_mut!(value),
    ) == 0
    {
        let mut result: *mut _object =
            unsafe { (*(self_ as *mut __SlateRecord_dictobject_c_5130)).di_result };
        if acquire_iter_result(result) {
            let oldkey: *mut _object = PyTuple_GET_ITEM!(result, 0i32);
            let oldvalue: *mut _object = PyTuple_GET_ITEM!(result, 1i32);
            PyTuple_SET_ITEM!(result, 0i32, key);
            PyTuple_SET_ITEM!(result, 1i32, value);
            Py_DECREF!(oldkey);
            Py_DECREF!(oldvalue);
            // bpo-42536: The GC may have untracked this result tuple. Since we're
            //
            // recycling it, make sure it's tracked again:
            _PyTuple_Recycle(result);
        } else {
            result = unsafe { PyTuple_New(2i64) };
            if result.is_null() {
                Py_DECREF!(key);
                Py_DECREF!(value);
                return core::ptr::null_mut::<_object>();
            }
            PyTuple_SET_ITEM!(result, 0i32, key);
            PyTuple_SET_ITEM!(result, 1i32, value);
        }
        return result;
    }
    core::ptr::null_mut::<_object>()
}

// dictreviter
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
fn dictreviter_iter_lock_held(
    d: *mut __SlateRecord_dictobject_h_33,
    self_: *mut _object,
) -> *mut _object {
    let mut entry_ptr: *mut __SlateRecord_pycore_dict_h_90 = unsafe { core::mem::zeroed() };
    let mut entry_ptr_1: *mut __SlateRecord_pycore_dict_h_85 = unsafe { core::mem::zeroed() };
    let mut result: *mut _object = unsafe { core::mem::zeroed() };
    let mut value: *mut _object = unsafe { core::mem::zeroed() };
    let mut key: *mut _object = unsafe { core::mem::zeroed() };
    let mut k: *mut _dictkeysobject = unsafe { core::mem::zeroed() };
    let mut i: i64 = unsafe { core::mem::zeroed() };
    let mut di: *mut __SlateRecord_dictobject_c_5130 = unsafe { core::mem::zeroed() };
    unsafe {
        di = self_ as *mut __SlateRecord_dictobject_c_5130;
        if (*di).di_used != (*d).ma_used {
            PyErr_SetString(
                PyExc_RuntimeError,
                c"dictionary changed size during iteration".as_ptr(),
            );
            (*di).di_used = 1i32.wrapping_neg() as i64;
            // Make this state sticky
            return core::ptr::null_mut::<_object>();
        } else {
            '__join_0: {
                i = (*di).di_pos;
                k = (*d).ma_keys;
                if i >= 0 {
                    if _PyDict_HasSplitTable!(d) {
                        let index: i32 = get_index_from_order(d, i);
                        key = (*DK_UNICODE_ENTRIES(k).wrapping_offset(index as isize)).me_key;
                        value = *((*(*d).ma_values).values.as_mut_ptr() as *mut *mut _object)
                            .wrapping_offset(index as isize);
                    } else {
                        if i >= (*k).dk_nentries {
                            break '__join_0;
                        } else {
                            if DK_IS_UNICODE!(k) {
                                entry_ptr = DK_UNICODE_ENTRIES(k).wrapping_offset(i as isize);
                                while unsafe { (*entry_ptr).me_value }.is_null() {
                                    let __v8: i64 = i.wrapping_sub(1);
                                    i = __v8;
                                    if __v8 < 0 {
                                        break '__join_0;
                                    } else {
                                        let __v7: *mut __SlateRecord_pycore_dict_h_90 = entry_ptr;
                                        entry_ptr = __v7.wrapping_offset(-(1 as isize));
                                    }
                                }
                                key = (*entry_ptr).me_key;
                                value = (*entry_ptr).me_value;
                            } else {
                                entry_ptr_1 = DK_ENTRIES(k).wrapping_offset(i as isize);
                                while unsafe { (*entry_ptr_1).me_value }.is_null() {
                                    let __v13: i64 = i.wrapping_sub(1);
                                    i = __v13;
                                    if __v13 < 0 {
                                        break '__join_0;
                                    } else {
                                        let __v12: *mut __SlateRecord_pycore_dict_h_85 =
                                            entry_ptr_1;
                                        entry_ptr_1 = __v12.wrapping_offset(-(1 as isize));
                                    }
                                }
                                key = (*entry_ptr_1).me_key;
                                value = (*entry_ptr_1).me_value;
                            }
                        }
                    }
                    // We found an element, but did not expect it
                    if (*di).len == 0 {
                        PyErr_SetString(
                            PyExc_RuntimeError,
                            c"dictionary keys changed during iteration".as_ptr(),
                        );
                    } else {
                        (*di).di_pos = i.wrapping_sub(1);
                        let __v3: i64 = (*di).len;
                        (*di).len = __v3.wrapping_sub(1);
                        if Py_IS_TYPE__2!(di, core::ptr::addr_of_mut!(PyDictRevIterKey_Type)) != 0 {
                            return Py_NewRef!(key);
                        } else {
                            if Py_IS_TYPE__2!(di, core::ptr::addr_of_mut!(PyDictRevIterValue_Type))
                                != 0
                            {
                                return Py_NewRef!(value);
                            } else {
                                if Py_IS_TYPE__2!(
                                    di,
                                    core::ptr::addr_of_mut!(PyDictRevIterItem_Type)
                                ) != 0
                                {
                                    result = (*di).di_result;
                                    if _PyObject_IsUniquelyReferenced(result) != 0 {
                                        let oldkey: *mut _object = PyTuple_GET_ITEM!(result, 0i32);
                                        let oldvalue: *mut _object =
                                            PyTuple_GET_ITEM!(result, 1i32);
                                        PyTuple_SET_ITEM!(result, 0i32, Py_NewRef!(key));
                                        PyTuple_SET_ITEM!(result, 1i32, Py_NewRef!(value));
                                        Py_INCREF!(result);
                                        Py_DECREF!(oldkey);
                                        Py_DECREF!(oldvalue);
                                        // bpo-42536: The GC may have untracked this result tuple. Since
                                        //
                                        // we're recycling it, make sure it's tracked again:
                                        _PyTuple_Recycle(result);
                                    } else {
                                        result = PyTuple_New(2i64);
                                        if result.is_null() {
                                            return core::ptr::null_mut::<_object>();
                                        } else {
                                            PyTuple_SET_ITEM!(result, 0i32, Py_NewRef!(key));
                                            PyTuple_SET_ITEM!(result, 1i32, Py_NewRef!(value));
                                        }
                                    }
                                    return result;
                                } else {
                                    Py_UNREACHABLE!();
                                }
                            }
                        }
                    }
                }
            }
            (*di).di_dict = core::ptr::null_mut::<__SlateRecord_dictobject_h_33>();
            Py_DECREF__2!(d);
            return core::ptr::null_mut::<_object>();
        }
    }
}

#[unsafe(link_section = ".text.slate_distinct.dictobject.dictreviter_iternext")]
extern "C-unwind" fn dictreviter_iternext(self_: *mut _object) -> *mut _object {
    let d: *mut __SlateRecord_dictobject_h_33 =
        unsafe { (*(self_ as *mut __SlateRecord_dictobject_c_5130)).di_dict };
    if d.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let value: *mut _object = dictreviter_iter_lock_held(d, self_);
    value
}

// [clinic input]
// dict.__reversed__
//
// Return a reverse iterator over the dict keys.
// [clinic start generated code]
/// [clinic end generated code: output=e674483336d1ed51 input=23210ef3477d8c4d]
fn dict___reversed___impl(self_: *mut __SlateRecord_dictobject_h_33) -> *mut _object {
    dictiter_new(self_, unsafe {
        core::ptr::addr_of_mut!(PyDictRevIterKey_Type)
    })
}

#[unsafe(link_section = ".text.slate_distinct.dictobject.dictiter_reduce")]
extern "C-unwind" fn dictiter_reduce(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    // copy the iterator state
    let mut tmp: __SlateRecord_dictobject_c_5130 =
        unsafe { *(self_ as *mut __SlateRecord_dictobject_c_5130) };
    Py_XINCREF__2!(tmp.di_dict);
    let list: *mut _object =
        unsafe { PySequence_List(core::ptr::addr_of_mut!(tmp) as *mut _object) };
    Py_XDECREF__2!(tmp.di_dict);
    if list.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    unsafe {
        Py_BuildValue(
            c"N(N)".as_ptr(),
            _PyEval_GetBuiltin(core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py_iter
                    ._ascii
                    .ob_base
            )),
            list,
        )
    }
}

// View objects for keys(), items(), values().
// The instance lay-out is the same for all three; but the type differs.
#[unsafe(link_section = ".text.slate_distinct.dictobject.dictview_dealloc")]
extern "C-unwind" fn dictview_dealloc(self_: *mut _object) {
    let dv: *mut __SlateRecord_pycore_dict_h_73 = self_ as *mut __SlateRecord_pycore_dict_h_73;
    // bpo-31095: UnTrack is needed before calling any callbacks
    _PyObject_GC_UNTRACK__2!(dv);
    Py_XDECREF__2!(unsafe { (*dv).dv_dict });
    unsafe { PyObject_GC_Del(dv as *mut ()) };
}

#[unsafe(link_section = ".text.slate_distinct.dictobject.dictview_traverse")]
extern "C-unwind" fn dictview_traverse(
    self_: *mut _object,
    mut visit: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
    arg: *mut (),
) -> i32 {
    let dv: *mut __SlateRecord_pycore_dict_h_73 = self_ as *mut __SlateRecord_pycore_dict_h_73;
    Py_VISIT__4!(unsafe { (*dv).dv_dict }, visit, arg);
    0
}

#[unsafe(link_section = ".text.slate_distinct.dictobject.dictview_len")]
extern "C-unwind" fn dictview_len(self_: *mut _object) -> i64 {
    let mut len: i64 = 0;
    if !unsafe { (*(self_ as *mut __SlateRecord_pycore_dict_h_73)).dv_dict }.is_null() {
        len = unsafe { (*(*(self_ as *mut __SlateRecord_pycore_dict_h_73)).dv_dict).ma_used };
    }
    len
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyDictView_New(dict: *mut _object, r#type: *mut _typeobject) -> *mut _object {
    if dict.is_null() {
        PyErr_BadInternalCall!();
        return core::ptr::null_mut::<_object>();
    }
    if PyDict_Check!(dict) == 0 {
        // XXX Get rid of this restriction later
        unsafe {
            PyErr_Format(
                PyExc_TypeError,
                c"%s() requires a dict argument, not '%s'".as_ptr(),
                (*r#type).tp_name,
                (*Py_TYPE!(dict)).tp_name,
            )
        };
        return core::ptr::null_mut::<_object>();
    }
    let dv: *mut __SlateRecord_pycore_dict_h_73 =
        PyObject_GC_New!(__SlateRecord_pycore_dict_h_73, r#type);
    if dv.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    unsafe {
        (*dv).dv_dict = Py_NewRef!(dict) as *mut __SlateRecord_dictobject_h_33;
    }
    _PyObject_GC_TRACK!(dv);
    dv as *mut _object
}

#[unsafe(link_section = ".text.slate_distinct.dictobject.dictview_mapping")]
extern "C-unwind" fn dictview_mapping(
    view: *mut _object,
    _unused_ignored: *mut (),
) -> *mut _object {
    let mapping: *mut _object =
        (unsafe { (*(view as *mut __SlateRecord_pycore_dict_h_73)).dv_dict }) as *mut _object;
    unsafe { PyDictProxy_New(mapping) }
}

static mut dictview_getset: __SlateAlign16<[PyGetSetDef; 2]> = __SlateAlign16([
    PyGetSetDef {
        name: (b"mapping\0".as_ptr() as *mut i8) as *const i8,
        get: Some(dictview_mapping),
        set: None,
        doc: (b"dictionary that this view refers to\0".as_ptr() as *mut i8) as *const i8,
        closure: core::ptr::null_mut::<()>(),
    },
    {
        let mut __t0: PyGetSetDef = unsafe { core::mem::zeroed() };
        __t0.name = core::ptr::null::<i8>();
        __t0
    },
]);

// TODO(guido): The views objects are not complete:
//
// * support more set operations
// * support arbitrary mappings?
//   - either these should be static or exported in dictobject.h
//   - if public then they should probably be in builtins
/// Return 1 if self is a subset of other, iterating over self;
/// 0 if not; -1 if an error occurred.
fn all_contained_in(self_: *mut _object, other: *mut _object) -> i32 {
    let iter: *mut _object = unsafe { PyObject_GetIter(self_) };
    let mut ok: i32 = 1;
    if iter.is_null() {
        return 1i32.wrapping_neg();
    }
    loop {
        let next: *mut _object = unsafe { PyIter_Next(iter) };
        if next.is_null() {
            if !unsafe { PyErr_Occurred() }.is_null() {
                ok = 1i32.wrapping_neg();
            }
            break;
        }
        ok = unsafe { PySequence_Contains(other, next) };
        Py_DECREF!(next);
        if ok <= 0 {
            break;
        }
    }
    Py_DECREF!(iter);
    ok
}

#[unsafe(link_section = ".text.slate_distinct.dictobject.dictview_richcompare")]
extern "C-unwind" fn dictview_richcompare(
    self_: *mut _object,
    other: *mut _object,
    op: i32,
) -> *mut _object {
    let mut len_self: i64 = 0;
    let mut len_other: i64 = 0;
    let mut ok: i32 = 0;
    let __v0: bool = !PyAnySet_Check!(other) && !PyDictViewSet_Check!(other);
    if __v0 {
        return unsafe { core::ptr::addr_of_mut!(_Py_NotImplementedStruct) };
    }
    len_self = unsafe { PyObject_Size(self_) };
    if len_self < 0 {
        return core::ptr::null_mut::<_object>();
    }
    len_other = unsafe { PyObject_Size(other) };
    if len_other < 0 {
        return core::ptr::null_mut::<_object>();
    }
    match op {
        2 | 3 => {
            if len_self == len_other {
                ok = all_contained_in(self_, other);
            }
            if op == 3 && ok >= 0 {
                ok = (ok == 0) as i32;
            }
        }
        0 => {
            if len_self < len_other {
                ok = all_contained_in(self_, other);
            }
        }
        1 => {
            if len_self <= len_other {
                ok = all_contained_in(self_, other);
            }
        }
        4 => {
            if len_self > len_other {
                ok = all_contained_in(other, self_);
            }
        }
        5 => {
            if len_self >= len_other {
                ok = all_contained_in(other, self_);
            }
        }
        _ => {}
    }
    if ok < 0 {
        return core::ptr::null_mut::<_object>();
    }
    let result: *mut _object = if ok != 0 {
        _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_TrueStruct) })
    } else {
        _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_FalseStruct) })
    };
    Py_NewRef!(result)
}

#[unsafe(link_section = ".text.slate_distinct.dictobject.dictview_repr")]
extern "C-unwind" fn dictview_repr(self_: *mut _object) -> *mut _object {
    let mut __v0: *mut _object = unsafe { core::mem::zeroed() };
    let mut rc: i64 = unsafe { core::mem::zeroed() };
    let mut result: *mut _object = unsafe { core::mem::zeroed() };
    let mut seq: *mut _object = unsafe { core::mem::zeroed() };
    let mut dv: *mut __SlateRecord_pycore_dict_h_73 = unsafe { core::mem::zeroed() };
    unsafe {
        dv = self_ as *mut __SlateRecord_pycore_dict_h_73;
        result = core::ptr::null_mut::<_object>();
        rc = Py_ReprEnter(dv as *mut _object) as i64;
        if rc != 0 {
            if rc > 0 {
                __v0 = PyUnicode_FromString(c"...".as_ptr());
            } else {
                __v0 = core::ptr::null_mut::<_object>();
            }
            return __v0;
        } else {
            seq = PySequence_List(dv as *mut _object);
            if !seq.is_null() {
                result = PyUnicode_FromFormat(c"%s(%R)".as_ptr(), (*Py_TYPE__2!(dv)).tp_name, seq);
                Py_DECREF!(seq);
            }
            Py_ReprLeave(dv as *mut _object);
            return result;
        }
    }
}

// dict_keys
#[unsafe(link_section = ".text.slate_distinct.dictobject.dictkeys_iter")]
extern "C-unwind" fn dictkeys_iter(self_: *mut _object) -> *mut _object {
    if unsafe { (*(self_ as *mut __SlateRecord_pycore_dict_h_73)).dv_dict }.is_null() {
        return unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) };
    }
    dictiter_new(
        unsafe { (*(self_ as *mut __SlateRecord_pycore_dict_h_73)).dv_dict },
        unsafe { core::ptr::addr_of_mut!(PyDictIterKey_Type) },
    )
}

#[unsafe(link_section = ".text.slate_distinct.dictobject.dictkeys_contains")]
extern "C-unwind" fn dictkeys_contains(self_: *mut _object, obj: *mut _object) -> i32 {
    if unsafe { (*(self_ as *mut __SlateRecord_pycore_dict_h_73)).dv_dict }.is_null() {
        return 0;
    }
    PyDict_Contains(
        (unsafe { (*(self_ as *mut __SlateRecord_pycore_dict_h_73)).dv_dict }) as *mut _object,
        obj,
    )
}

/// sq_length
/// sq_concat
/// sq_repeat
/// sq_item
/// sq_slice
/// sq_ass_item
/// sq_ass_slice
/// sq_contains
static mut dictkeys_as_sequence: __SlateRecord_object_h_120 = {
    let mut __t0: __SlateRecord_object_h_120 = unsafe { core::mem::zeroed() };
    __t0.sq_length = Some(dictview_len);
    __t0.sq_concat = None;
    __t0.sq_repeat = None;
    __t0.sq_item = None;
    __t0.was_sq_slice = core::ptr::null_mut::<()>();
    __t0.sq_ass_item = None;
    __t0.was_sq_ass_slice = core::ptr::null_mut::<()>();
    __t0.sq_contains = Some(dictkeys_contains);
    __t0
};

/// Create a set object from dictviews object.
///
/// Returns a new reference.
///
/// This utility function is used by set operations.
fn dictviews_to_set(self_: *mut _object) -> *mut _object {
    let mut left: *mut _object = self_;
    if PyDictKeys_Check!(self_) != 0 {
        // PySet_New() has fast path for the dict object.
        let dict: *mut _object =
            (unsafe { (*(self_ as *mut __SlateRecord_pycore_dict_h_73)).dv_dict }) as *mut _object;
        if PyDict_CheckExact!(dict) != 0 {
            left = dict;
        }
    }
    unsafe { PySet_New(left) }
}

#[unsafe(link_section = ".text.slate_distinct.dictobject.dictviews_sub")]
extern "C-unwind" fn dictviews_sub(self_: *mut _object, other: *mut _object) -> *mut _object {
    let result: *mut _object = dictviews_to_set(self_);
    if result.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let tmp: *mut _object = PyObject_CallMethodOneArg(
        result,
        unsafe {
            core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py_difference_update
                    ._ascii
                    .ob_base
            )
        },
        other,
    );
    if tmp.is_null() {
        Py_DECREF!(result);
        return core::ptr::null_mut::<_object>();
    }
    Py_DECREF!(tmp);
    result
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.dictobject._PyDictView_Intersect")]
extern "C-unwind" fn _PyDictView_Intersect(
    mut self_: *mut _object,
    mut other: *mut _object,
) -> *mut _object {
    // if other is a set and self is smaller than other,
    // reuse set intersection logic
    let mut __v3: bool = unsafe { core::mem::zeroed() };
    let mut dict_contains: core::mem::MaybeUninit<
        Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> i32>,
    > = core::mem::MaybeUninit::uninit();
    let __slate_slot_5: *mut Option<
        unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> i32,
    > = core::ptr::addr_of_mut!(dict_contains)
        as *mut Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> i32>;
    let mut rv: i32 = unsafe { core::mem::zeroed() };
    let mut len_self: i64 = unsafe { core::mem::zeroed() };
    let mut key: *mut _object = unsafe { core::mem::zeroed() };
    let mut it: *mut _object = unsafe { core::mem::zeroed() };
    let mut result: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        // Python interpreter swaps parameters when dict view
        // is on right side of &
        if !PyDictViewSet_Check!(self_) {
            let tmp: *mut _object = other;
            other = self_;
            self_ = tmp;
        }
        len_self = dictview_len(self_);
        if PySet_CheckExact!(other) != 0 {
            __v3 = len_self <= PyObject_Size(other);
        } else {
            __v3 = false;
        }
        if __v3 {
            return PyObject_CallMethodObjArgs(
                other,
                core::ptr::addr_of_mut!(
                    _PyRuntime
                        .static_objects
                        .singletons
                        .strings
                        .identifiers
                        ._py_intersection
                        ._ascii
                        .ob_base
                ),
                self_,
                core::ptr::null_mut::<()>(),
            );
        } else {
            // if other is another dict view, and it is bigger than self,
            // swap them
            if PyDictViewSet_Check!(other) {
                let len_other: i64 = dictview_len(other);
                if len_other > len_self {
                    let tmp: *mut _object = other;
                    other = self_;
                    self_ = tmp;
                }
            }
            // at this point, two things should be true
            // 1. self is a dictview
            // 2. if other is a dictview then it is smaller than self
            result = PySet_New(core::ptr::null_mut::<_object>());
            if result.is_null() {
                return core::ptr::null_mut::<_object>();
            } else {
                it = PyObject_GetIter(other);
                if it.is_null() {
                    Py_DECREF!(result);
                    return core::ptr::null_mut::<_object>();
                } else {
                    if PyDictKeys_Check!(self_) != 0 {
                        *__slate_slot_5 = Some(dictkeys_contains);
                    } else {
                        *__slate_slot_5 = Some(dictitems_contains);
                    }
                    // else PyDictItems_Check(self)
                    '__join_0: {
                        loop {
                            let __v0: *mut _object = PyIter_Next(it);
                            key = __v0;
                            if !__v0.is_null() {
                                rv = (*__slate_slot_5).unwrap()(self_, key);
                                if rv < 0 {
                                    break '__join_0;
                                } else {
                                    if rv != 0 {
                                        if PySet_Add(result, key) != 0 {
                                            break '__join_0;
                                        }
                                    }
                                    Py_DECREF!(key);
                                }
                            } else {
                                break;
                            }
                        }
                        Py_DECREF!(it);
                        if !PyErr_Occurred().is_null() {
                            Py_DECREF!(result);
                            return core::ptr::null_mut::<_object>();
                        } else {
                            return result;
                        }
                    }
                    Py_DECREF!(it);
                    Py_DECREF!(result);
                    Py_DECREF!(key);
                    return core::ptr::null_mut::<_object>();
                }
            }
        }
    }
}

#[unsafe(link_section = ".text.slate_distinct.dictobject.dictviews_or")]
extern "C-unwind" fn dictviews_or(self_: *mut _object, other: *mut _object) -> *mut _object {
    let result: *mut _object = dictviews_to_set(self_);
    if result.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    if (unsafe { _PySet_Update(result, other) }) < 0 {
        Py_DECREF!(result);
        return core::ptr::null_mut::<_object>();
    }
    result
}

fn dictitems_xor_lock_held(d1: *mut _object, d2: *mut _object) -> *mut _object {
    let mut remaining_pairs: *mut _object = unsafe { core::mem::zeroed() };
    let mut pair: *mut _object = unsafe { core::mem::zeroed() };
    let mut to_delete: i32 = unsafe { core::mem::zeroed() };
    let mut hash: i64 = unsafe { core::mem::zeroed() };
    let mut pos: i64 = unsafe { core::mem::zeroed() };
    let mut val2: *mut _object = unsafe { core::mem::zeroed() };
    let mut val1: *mut _object = unsafe { core::mem::zeroed() };
    let mut key: *mut _object = unsafe { core::mem::zeroed() };
    let mut result_set: *mut _object = unsafe { core::mem::zeroed() };
    let mut temp_dict: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        temp_dict = copy_lock_held(d1);
        if temp_dict.is_null() {
            return core::ptr::null_mut::<_object>();
        } else {
            result_set = PySet_New(core::ptr::null_mut::<_object>());
            if result_set.is_null() {
                Py_CLEAR__2!(temp_dict);
                return core::ptr::null_mut::<_object>();
            } else {
                key = core::ptr::null_mut::<_object>();
                val1 = core::ptr::null_mut::<_object>();
                val2 = core::ptr::null_mut::<_object>();
                pos = 0;
                '__join_0: {
                    '__join_9: {
                        while _PyDict_Next(
                            d2,
                            core::ptr::addr_of_mut!(pos),
                            core::ptr::addr_of_mut!(key),
                            core::ptr::addr_of_mut!(val2),
                            core::ptr::addr_of_mut!(hash),
                        ) != 0
                        {
                            Py_INCREF!(key);
                            Py_INCREF!(val2);
                            val1 = _PyDict_GetItem_KnownHash(temp_dict, key, hash);
                            if val1.is_null() {
                                if !PyErr_Occurred().is_null() {
                                    break '__join_0;
                                } else {
                                    to_delete = 0;
                                }
                            } else {
                                Py_INCREF!(val1);
                                to_delete = PyObject_RichCompareBool(val1, val2, 2i32);
                                if to_delete < 0 {
                                    break '__join_0;
                                }
                            }
                            if to_delete != 0 {
                                if _PyDict_DelItem_KnownHash(temp_dict, key, hash) < 0 {
                                    break '__join_0;
                                }
                            } else {
                                pair = PyTuple_Pack(2i64, key, val2);
                                if pair.is_null() {
                                    break '__join_0;
                                } else {
                                    if PySet_Add(result_set, pair) < 0 {
                                        break '__join_9;
                                    } else {
                                        Py_DECREF!(pair);
                                    }
                                }
                            }
                            Py_DECREF!(key);
                            Py_XDECREF!(val1);
                            Py_DECREF!(val2);
                        }
                        val2 = core::ptr::null_mut::<_object>();
                        val1 = core::ptr::null_mut::<_object>();
                        key = core::ptr::null_mut::<_object>();
                        remaining_pairs = PyObject_CallMethodNoArgs(
                            temp_dict,
                            core::ptr::addr_of_mut!(
                                _PyRuntime
                                    .static_objects
                                    .singletons
                                    .strings
                                    .identifiers
                                    ._py_items
                                    ._ascii
                                    .ob_base
                            ),
                        );
                        if remaining_pairs.is_null() {
                            break '__join_0;
                        } else {
                            if _PySet_Update(result_set, remaining_pairs) < 0 {
                                Py_DECREF!(remaining_pairs);
                                break '__join_0;
                            } else {
                                Py_DECREF!(temp_dict);
                                Py_DECREF!(remaining_pairs);
                                return result_set;
                            }
                        }
                    }
                    Py_DECREF!(pair);
                }
                Py_XDECREF!(temp_dict);
                Py_XDECREF!(result_set);
                Py_XDECREF!(key);
                Py_XDECREF!(val1);
                Py_XDECREF!(val2);
                return core::ptr::null_mut::<_object>();
            }
        }
    }
}

fn dictitems_xor(self_: *mut _object, other: *mut _object) -> *mut _object {
    let d1: *mut _object =
        (unsafe { (*(self_ as *mut __SlateRecord_pycore_dict_h_73)).dv_dict }) as *mut _object;
    let d2: *mut _object =
        (unsafe { (*(other as *mut __SlateRecord_pycore_dict_h_73)).dv_dict }) as *mut _object;
    let res: *mut _object = dictitems_xor_lock_held(d1, d2);
    res
}

#[unsafe(link_section = ".text.slate_distinct.dictobject.dictviews_xor")]
extern "C-unwind" fn dictviews_xor(self_: *mut _object, other: *mut _object) -> *mut _object {
    let __v0: bool = PyDictItems_Check!(self_) != 0 && PyDictItems_Check!(other) != 0;
    if __v0 {
        return dictitems_xor(self_, other);
    }
    let result: *mut _object = dictviews_to_set(self_);
    if result.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let tmp: *mut _object = PyObject_CallMethodOneArg(
        result,
        unsafe {
            core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py_symmetric_difference_update
                    ._ascii
                    .ob_base
            )
        },
        other,
    );
    if tmp.is_null() {
        Py_DECREF!(result);
        return core::ptr::null_mut::<_object>();
    }
    Py_DECREF!(tmp);
    result
}

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
static mut dictviews_as_number: __SlateRecord_object_h_106 = {
    let mut __t0: __SlateRecord_object_h_106 = unsafe { core::mem::zeroed() };
    __t0.nb_add = None;
    __t0.nb_subtract = Some(dictviews_sub);
    __t0.nb_multiply = None;
    __t0.nb_remainder = None;
    __t0.nb_divmod = None;
    __t0.nb_power = None;
    __t0.nb_negative = None;
    __t0.nb_positive = None;
    __t0.nb_absolute = None;
    __t0.nb_bool = None;
    __t0.nb_invert = None;
    __t0.nb_lshift = None;
    __t0.nb_rshift = None;
    __t0.nb_and = Some(_PyDictView_Intersect);
    __t0.nb_xor = Some(dictviews_xor);
    __t0.nb_or = Some(dictviews_or);
    __t0
};

#[unsafe(link_section = ".text.slate_distinct.dictobject.dictviews_isdisjoint")]
extern "C-unwind" fn dictviews_isdisjoint(
    mut self_: *mut _object,
    mut other: *mut _object,
) -> *mut _object {
    let mut item: *mut _object = core::ptr::null_mut::<_object>();
    if self_ == other {
        if dictview_len(self_) == 0 {
            return _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_TrueStruct) });
        } else {
            return _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_FalseStruct) });
        }
    }
    // Iterate over the shorter object (only if other is a set,
    // because PySequence_Contains may be expensive otherwise):
    let __v0: bool = PyAnySet_Check!(other) || PyDictViewSet_Check!(other);
    if __v0 {
        let len_self: i64 = dictview_len(self_);
        let len_other: i64 = unsafe { PyObject_Size(other) };
        if len_other == (1i32.wrapping_neg() as i64) {
            return core::ptr::null_mut::<_object>();
        }
        if len_other > len_self {
            let tmp: *mut _object = other;
            other = self_;
            self_ = tmp;
        }
    }
    let it: *mut _object = unsafe { PyObject_GetIter(other) };
    if it.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    loop {
        let __v5: *mut _object = unsafe { PyIter_Next(it) };
        item = __v5;
        if __v5.is_null() {
            break;
        }
        let contains: i32 = unsafe { PySequence_Contains(self_, item) };
        Py_DECREF!(item);
        if contains == 1i32.wrapping_neg() {
            Py_DECREF!(it);
            return core::ptr::null_mut::<_object>();
        }
        if contains != 0 {
            Py_DECREF!(it);
            return _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_FalseStruct) });
        }
    }
    Py_DECREF!(it);
    if !unsafe { PyErr_Occurred() }.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    // PyIter_Next raised an exception.
    _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_TrueStruct) })
}

static isdisjoint_doc: __SlateAlign16<[i8; 73]> = __SlateAlign16([
    82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 84i8, 114i8, 117i8, 101i8, 32i8, 105i8, 102i8,
    32i8, 116i8, 104i8, 101i8, 32i8, 118i8, 105i8, 101i8, 119i8, 32i8, 97i8, 110i8, 100i8, 32i8,
    116i8, 104i8, 101i8, 32i8, 103i8, 105i8, 118i8, 101i8, 110i8, 32i8, 105i8, 116i8, 101i8, 114i8,
    97i8, 98i8, 108i8, 101i8, 32i8, 104i8, 97i8, 118i8, 101i8, 32i8, 97i8, 32i8, 110i8, 117i8,
    108i8, 108i8, 32i8, 105i8, 110i8, 116i8, 101i8, 114i8, 115i8, 101i8, 99i8, 116i8, 105i8, 111i8,
    110i8, 46i8, 0i8,
]);

static reversed_keys_doc: __SlateAlign16<[i8; 46]> = __SlateAlign16([
    82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 97i8, 32i8, 114i8, 101i8, 118i8, 101i8, 114i8,
    115i8, 101i8, 32i8, 105i8, 116i8, 101i8, 114i8, 97i8, 116i8, 111i8, 114i8, 32i8, 111i8, 118i8,
    101i8, 114i8, 32i8, 116i8, 104i8, 101i8, 32i8, 100i8, 105i8, 99i8, 116i8, 32i8, 107i8, 101i8,
    121i8, 115i8, 46i8, 0i8,
]);

/// sentinel
static mut dictkeys_methods: __SlateAlign16<[PyMethodDef; 3]> = __SlateAlign16([
    {
        let mut __t0: PyMethodDef = unsafe { core::mem::zeroed() };
        __t0.ml_name = (b"isdisjoint\0".as_ptr() as *mut i8) as *const i8;
        __t0.ml_meth = Some(dictviews_isdisjoint);
        __t0.ml_flags = 8i32;
        __t0.ml_doc = unsafe { core::ptr::addr_of!(isdisjoint_doc.0) as *const i8 };
        __t0
    },
    {
        let mut __t1: PyMethodDef = unsafe { core::mem::zeroed() };
        __t1.ml_name = (b"__reversed__\0".as_ptr() as *mut i8) as *const i8;
        __t1.ml_meth = Some(dictkeys_reversed);
        __t1.ml_flags = 4i32;
        __t1.ml_doc = unsafe { core::ptr::addr_of!(reversed_keys_doc.0) as *const i8 };
        __t1
    },
    {
        let mut __t2: PyMethodDef = unsafe { core::mem::zeroed() };
        __t2.ml_name = core::ptr::null::<i8>();
        __t2.ml_meth = None;
        __t2
    },
]);

// [clinic input]
// dict.keys
//
// Return a set-like object providing a view on the dict's keys.
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
/// [clinic end generated code: output=aac2830c62990358 input=42f48a7a771212a7]
fn dict_keys_impl(self_: *mut __SlateRecord_dictobject_h_33) -> *mut _object {
    _PyDictView_New(self_ as *mut _object, unsafe {
        core::ptr::addr_of_mut!(PyDictKeys_Type)
    })
}

#[unsafe(link_section = ".text.slate_distinct.dictobject.dictkeys_reversed")]
extern "C-unwind" fn dictkeys_reversed(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    if unsafe { (*(self_ as *mut __SlateRecord_pycore_dict_h_73)).dv_dict }.is_null() {
        return unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) };
    }
    dictiter_new(
        unsafe { (*(self_ as *mut __SlateRecord_pycore_dict_h_73)).dv_dict },
        unsafe { core::ptr::addr_of_mut!(PyDictRevIterKey_Type) },
    )
}

// dict_items
#[unsafe(link_section = ".text.slate_distinct.dictobject.dictitems_iter")]
extern "C-unwind" fn dictitems_iter(self_: *mut _object) -> *mut _object {
    if unsafe { (*(self_ as *mut __SlateRecord_pycore_dict_h_73)).dv_dict }.is_null() {
        return unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) };
    }
    dictiter_new(
        unsafe { (*(self_ as *mut __SlateRecord_pycore_dict_h_73)).dv_dict },
        unsafe { core::ptr::addr_of_mut!(PyDictIterItem_Type) },
    )
}

#[unsafe(link_section = ".text.slate_distinct.dictobject.dictitems_contains")]
extern "C-unwind" fn dictitems_contains(self_: *mut _object, obj: *mut _object) -> i32 {
    let mut result: i32 = 0;
    let mut found: *mut _object = unsafe { core::mem::zeroed() };
    if unsafe { (*(self_ as *mut __SlateRecord_pycore_dict_h_73)).dv_dict }.is_null() {
        return 0;
    }
    let __v0: bool = PyTuple_Check!(obj) == 0 || PyTuple_GET_SIZE!(obj) != 2;
    if __v0 {
        return 0;
    }
    let key: *mut _object = PyTuple_GET_ITEM!(obj, 0i32);
    let value: *mut _object = PyTuple_GET_ITEM!(obj, 1i32);
    result = PyDict_GetItemRef(
        (unsafe { (*(self_ as *mut __SlateRecord_pycore_dict_h_73)).dv_dict }) as *mut _object,
        key,
        core::ptr::addr_of_mut!(found),
    );
    if result == 1 {
        result = unsafe { PyObject_RichCompareBool(found, value, 2i32) };
        Py_DECREF!(found);
    }
    result
}

/// sq_length
/// sq_concat
/// sq_repeat
/// sq_item
/// sq_slice
/// sq_ass_item
/// sq_ass_slice
/// sq_contains
static mut dictitems_as_sequence: __SlateRecord_object_h_120 = {
    let mut __t0: __SlateRecord_object_h_120 = unsafe { core::mem::zeroed() };
    __t0.sq_length = Some(dictview_len);
    __t0.sq_concat = None;
    __t0.sq_repeat = None;
    __t0.sq_item = None;
    __t0.was_sq_slice = core::ptr::null_mut::<()>();
    __t0.sq_ass_item = None;
    __t0.was_sq_ass_slice = core::ptr::null_mut::<()>();
    __t0.sq_contains = Some(dictitems_contains);
    __t0
};

static reversed_items_doc: __SlateAlign16<[i8; 47]> = __SlateAlign16([
    82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 97i8, 32i8, 114i8, 101i8, 118i8, 101i8, 114i8,
    115i8, 101i8, 32i8, 105i8, 116i8, 101i8, 114i8, 97i8, 116i8, 111i8, 114i8, 32i8, 111i8, 118i8,
    101i8, 114i8, 32i8, 116i8, 104i8, 101i8, 32i8, 100i8, 105i8, 99i8, 116i8, 32i8, 105i8, 116i8,
    101i8, 109i8, 115i8, 46i8, 0i8,
]);

/// sentinel
static mut dictitems_methods: __SlateAlign16<[PyMethodDef; 3]> = __SlateAlign16([
    {
        let mut __t0: PyMethodDef = unsafe { core::mem::zeroed() };
        __t0.ml_name = (b"isdisjoint\0".as_ptr() as *mut i8) as *const i8;
        __t0.ml_meth = Some(dictviews_isdisjoint);
        __t0.ml_flags = 8i32;
        __t0.ml_doc = unsafe { core::ptr::addr_of!(isdisjoint_doc.0) as *const i8 };
        __t0
    },
    {
        let mut __t1: PyMethodDef = unsafe { core::mem::zeroed() };
        __t1.ml_name = (b"__reversed__\0".as_ptr() as *mut i8) as *const i8;
        __t1.ml_meth = Some(dictitems_reversed);
        __t1.ml_flags = 4i32;
        __t1.ml_doc = unsafe { core::ptr::addr_of!(reversed_items_doc.0) as *const i8 };
        __t1
    },
    {
        let mut __t2: PyMethodDef = unsafe { core::mem::zeroed() };
        __t2.ml_name = core::ptr::null::<i8>();
        __t2.ml_meth = None;
        __t2
    },
]);

// [clinic input]
// dict.items
//
// Return a set-like object providing a view on the dict's items.
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
/// [clinic end generated code: output=88c7db7150c7909a input=87c822872eb71f5a]
fn dict_items_impl(self_: *mut __SlateRecord_dictobject_h_33) -> *mut _object {
    _PyDictView_New(self_ as *mut _object, unsafe {
        core::ptr::addr_of_mut!(PyDictItems_Type)
    })
}

#[unsafe(link_section = ".text.slate_distinct.dictobject.dictitems_reversed")]
extern "C-unwind" fn dictitems_reversed(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    if unsafe { (*(self_ as *mut __SlateRecord_pycore_dict_h_73)).dv_dict }.is_null() {
        return unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) };
    }
    dictiter_new(
        unsafe { (*(self_ as *mut __SlateRecord_pycore_dict_h_73)).dv_dict },
        unsafe { core::ptr::addr_of_mut!(PyDictRevIterItem_Type) },
    )
}

// dict_values
#[unsafe(link_section = ".text.slate_distinct.dictobject.dictvalues_iter")]
extern "C-unwind" fn dictvalues_iter(self_: *mut _object) -> *mut _object {
    if unsafe { (*(self_ as *mut __SlateRecord_pycore_dict_h_73)).dv_dict }.is_null() {
        return unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) };
    }
    dictiter_new(
        unsafe { (*(self_ as *mut __SlateRecord_pycore_dict_h_73)).dv_dict },
        unsafe { core::ptr::addr_of_mut!(PyDictIterValue_Type) },
    )
}

/// sq_length
/// sq_concat
/// sq_repeat
/// sq_item
/// sq_slice
/// sq_ass_item
/// sq_ass_slice
/// sq_contains
static mut dictvalues_as_sequence: __SlateRecord_object_h_120 = {
    let mut __t0: __SlateRecord_object_h_120 = unsafe { core::mem::zeroed() };
    __t0.sq_length = Some(dictview_len);
    __t0.sq_concat = None;
    __t0.sq_repeat = None;
    __t0.sq_item = None;
    __t0.was_sq_slice = core::ptr::null_mut::<()>();
    __t0.sq_ass_item = None;
    __t0.was_sq_ass_slice = core::ptr::null_mut::<()>();
    __t0.sq_contains = None;
    __t0
};

static reversed_values_doc: __SlateAlign16<[i8; 48]> = __SlateAlign16([
    82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 97i8, 32i8, 114i8, 101i8, 118i8, 101i8, 114i8,
    115i8, 101i8, 32i8, 105i8, 116i8, 101i8, 114i8, 97i8, 116i8, 111i8, 114i8, 32i8, 111i8, 118i8,
    101i8, 114i8, 32i8, 116i8, 104i8, 101i8, 32i8, 100i8, 105i8, 99i8, 116i8, 32i8, 118i8, 97i8,
    108i8, 117i8, 101i8, 115i8, 46i8, 0i8,
]);

/// sentinel
static mut dictvalues_methods: __SlateAlign16<[PyMethodDef; 2]> = __SlateAlign16([
    {
        let mut __t0: PyMethodDef = unsafe { core::mem::zeroed() };
        __t0.ml_name = (b"__reversed__\0".as_ptr() as *mut i8) as *const i8;
        __t0.ml_meth = Some(dictvalues_reversed);
        __t0.ml_flags = 4i32;
        __t0.ml_doc = unsafe { core::ptr::addr_of!(reversed_values_doc.0) as *const i8 };
        __t0
    },
    {
        let mut __t1: PyMethodDef = unsafe { core::mem::zeroed() };
        __t1.ml_name = core::ptr::null::<i8>();
        __t1.ml_meth = None;
        __t1
    },
]);

// [clinic input]
// dict.values
//
// Return an object providing a view on the dict's values.
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
/// [clinic end generated code: output=ce9f2e9e8a959dd4 input=b46944f85493b230]
fn dict_values_impl(self_: *mut __SlateRecord_dictobject_h_33) -> *mut _object {
    _PyDictView_New(self_ as *mut _object, unsafe {
        core::ptr::addr_of_mut!(PyDictValues_Type)
    })
}

#[unsafe(link_section = ".text.slate_distinct.dictobject.dictvalues_reversed")]
extern "C-unwind" fn dictvalues_reversed(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    if unsafe { (*(self_ as *mut __SlateRecord_pycore_dict_h_73)).dv_dict }.is_null() {
        return unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) };
    }
    dictiter_new(
        unsafe { (*(self_ as *mut __SlateRecord_pycore_dict_h_73)).dv_dict },
        unsafe { core::ptr::addr_of_mut!(PyDictRevIterValue_Type) },
    )
}

/// Returns NULL if cannot allocate a new PyDictKeysObject,
/// but does not set an error
#[unsafe(no_mangle)]
extern "C-unwind" fn _PyDict_NewKeysForClass(cls: *mut _heaptypeobject) -> *mut _dictkeysobject {
    let interp: *mut _is = _PyInterpreterState_GET();
    let keys: *mut _dictkeysobject = new_keys_object(interp, 6u8, true);
    if keys.is_null() {
        unsafe { PyErr_Clear() };
    } else {
        // Set to max size+1 as it will shrink by one before each new object
        unsafe {
            (*keys).dk_usable = 30;
        }
        unsafe {
            (*keys).dk_kind = 2;
        }
    }
    if !unsafe { (*cls).ht_type.tp_dict }.is_null() {
        let attrs: *mut _object = PyDict_GetItem(unsafe { (*cls).ht_type.tp_dict }, unsafe {
            core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py___static_attributes__
                    ._ascii
                    .ob_base
            )
        });
        let __v0: bool = !attrs.is_null() && PyTuple_Check!(attrs) != 0;
        if __v0 {
            let mut i: i64 = 0;
            while i < PyTuple_GET_SIZE!(attrs) {
                let key: *mut _object = PyTuple_GET_ITEM!(attrs, i);
                let mut hash: i64 = 0;
                let __v1: bool = if PyUnicode_CheckExact!(key) != 0 {
                    let __v2: i64 = unicode_get_hash(key);
                    hash = __v2;
                    __v2 != (1i32.wrapping_neg() as i64)
                } else {
                    false
                };
                if __v1 {
                    if insert_split_key(keys, key, hash) == (1i32.wrapping_neg() as i64) {
                        break;
                    }
                }
                i = i.wrapping_add(1);
            }
        }
    }
    keys
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyObject_InitInlineValues(obj: *mut _object, tp: *mut _typeobject) {
    let keys: *mut _dictkeysobject = CACHED_KEYS!(tp);
    OBJECT_STAT_INC!();
    if (unsafe { (*keys).dk_usable }) > 1 {
        let __v1: i64 = unsafe { (*keys).dk_usable };
        unsafe {
            (*keys).dk_usable = __v1.wrapping_sub(1);
        }
    }
    let size: u64 = shared_keys_usable_size(keys);
    let values: *mut _dictvalues = _PyObject_InlineValues(obj);
    unsafe {
        (*values).capacity = size as u8;
    }
    unsafe {
        (*values).size = 0;
    }
    unsafe {
        (*values).embedded = 1;
    }
    unsafe {
        (*values).valid = 1;
    }
    for i in 0..size {
        unsafe {
            *((*values).values.as_mut_ptr() as *mut *mut _object).wrapping_offset(i as isize) =
                core::ptr::null_mut::<_object>();
        }
    }
    unsafe {
        (*_PyObject_ManagedDictPointer(obj)).dict =
            core::ptr::null_mut::<__SlateRecord_dictobject_h_33>();
    }
}

fn make_dict_from_instance_attributes(
    interp: *mut _is,
    keys: *mut _dictkeysobject,
    values: *mut _dictvalues,
) -> *mut __SlateRecord_dictobject_h_33 {
    dictkeys_incref(keys);
    let mut used: i64 = 0;
    let size: u64 = shared_keys_usable_size(keys);
    for i in 0..size {
        let val: *mut _object = unsafe {
            *((*values).values.as_mut_ptr() as *mut *mut _object).wrapping_offset(i as isize)
        };
        if !val.is_null() {
            used = used.wrapping_add(1);
        }
    }
    let res: *mut __SlateRecord_dictobject_h_33 =
        new_dict(interp, keys, values, used, 0i32) as *mut __SlateRecord_dictobject_h_33;
    res
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyObject_MaterializeManagedDict_LockHeld(
    obj: *mut _object,
) -> *mut __SlateRecord_dictobject_h_33 {
    OBJECT_STAT_INC!();
    let values: *mut _dictvalues = _PyObject_InlineValues(obj);
    let mut dict: *mut __SlateRecord_dictobject_h_33 = unsafe { core::mem::zeroed() };
    if (unsafe { (*values).valid }) != 0 {
        let interp: *mut _is = _PyInterpreterState_GET();
        let keys: *mut _dictkeysobject = CACHED_KEYS!(Py_TYPE!(obj));
        dict = make_dict_from_instance_attributes(interp, keys, values);
    } else {
        dict = PyDict_New() as *mut __SlateRecord_dictobject_h_33;
    }
    FT_ATOMIC_STORE_PTR_RELEASE!((*_PyObject_ManagedDictPointer(obj)).dict, dict);
    dict
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyObject_MaterializeManagedDict(
    obj: *mut _object,
) -> *mut __SlateRecord_dictobject_h_33 {
    let mut dict: *mut __SlateRecord_dictobject_h_33 = _PyObject_GetManagedDict(obj);
    if !dict.is_null() {
        return dict;
    }
    dict = _PyObject_MaterializeManagedDict_LockHeld(obj);
    dict
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyDict_SetItem_LockHeld(
    dict: *mut __SlateRecord_dictobject_h_33,
    name: *mut _object,
    value: *mut _object,
) -> i32 {
    if value.is_null() {
        let hash: i64 = _PyObject_HashFast(name);
        if hash == (1i32.wrapping_neg() as i64) {
            dict_unhashable_type(name);
            return 1i32.wrapping_neg();
        }
        return _PyDict_DelItem_KnownHash_LockHeld(dict as *mut _object, name, hash);
    } else {
        return setitem_lock_held(dict, name, value);
    }
}

/// Called with either the object's lock or the dict's lock held
///
/// depending on whether or not a dict has been materialized for
///
/// the object.
fn store_instance_attr_lock_held(
    obj: *mut _object,
    values: *mut _dictvalues,
    name: *mut _object,
    value: *mut _object,
) -> i32 {
    let keys: *mut _dictkeysobject = CACHED_KEYS!(Py_TYPE!(obj));
    let mut ix: i64 = 1i32.wrapping_neg() as i64;
    let mut dict: *mut __SlateRecord_dictobject_h_33 = _PyObject_GetManagedDict(obj);
    if PyUnicode_CheckExact!(name) != 0 {
        let mut hash: i64 = unicode_get_hash(name);
        if hash == (1i32.wrapping_neg() as i64) {
            hash = unsafe { unsafe { PyUnicode_Type.tp_hash }.unwrap()(name) };
        }
        ix = insert_split_key(keys, name, hash);
    }
    if ix == (1i32.wrapping_neg() as i64) {
        let mut res: i32 = 0;
        if dict.is_null() {
            // Make the dict but don't publish it in the object
            //
            // so that no one else will see it.
            dict = make_dict_from_instance_attributes(
                unsafe { PyInterpreterState_Get() },
                keys,
                values,
            );
            let __v0: bool = dict.is_null() || _PyDict_SetItem_LockHeld(dict, name, value) < 0;
            if __v0 {
                Py_XDECREF__2!(dict);
                return 1i32.wrapping_neg();
            }
            FT_ATOMIC_STORE_PTR_RELEASE!((*_PyObject_ManagedDictPointer(obj)).dict, dict);
            return 0;
        }
        res = _PyDict_SetItem_LockHeld(dict, name, value);
        return res;
    }
    let old_value: *mut _object = unsafe {
        *((*values).values.as_mut_ptr() as *mut *mut _object).wrapping_offset(ix as isize)
    };
    if old_value.is_null() && value.is_null() {
        unsafe {
            PyErr_Format(
                PyExc_AttributeError,
                c"'%.100s' object has no attribute '%U'".as_ptr(),
                (*Py_TYPE!(obj)).tp_name,
                name,
            )
        };
        return 1i32.wrapping_neg();
    }
    if !dict.is_null() {
        let interp: *mut _is = _PyInterpreterState_GET();
        let event: u32 = (if old_value.is_null() {
            0i32
        } else {
            if value.is_null() { 2i32 } else { 1i32 }
        }) as u32;
        _PyDict_NotifyEvent(interp, event, dict, name, value);
    }
    FT_ATOMIC_STORE_PTR_RELEASE!(
        *unsafe { (*values).values.as_mut_ptr() as *mut *mut _object }.wrapping_offset(ix as isize),
        Py_XNewRef!(value)
    );
    if old_value.is_null() {
        _PyDictValues_AddToInsertionOrder(values, ix);
        if !dict.is_null() {
            STORE_USED!(dict, unsafe { (*dict).ma_used }.wrapping_add(1));
        }
    } else {
        if value.is_null() {
            delete_index_from_values(values, ix);
            if !dict.is_null() {
                STORE_USED!(dict, unsafe { (*dict).ma_used }.wrapping_sub(1));
            }
        }
        Py_DECREF!(old_value);
    }
    0
}

fn store_instance_attr_dict(
    obj: *mut _object,
    dict: *mut __SlateRecord_dictobject_h_33,
    name: *mut _object,
    value: *mut _object,
) -> i32 {
    let values: *mut _dictvalues = _PyObject_InlineValues(obj);
    let res: i32 = if (unsafe { (*dict).ma_values }) == values {
        store_instance_attr_lock_held(obj, values, name, value)
    } else {
        _PyDict_SetItem_LockHeld(dict, name, value)
    };
    res
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyObject_StoreInstanceAttribute(
    obj: *mut _object,
    name: *mut _object,
    value: *mut _object,
) -> i32 {
    let values: *mut _dictvalues = _PyObject_InlineValues(obj);
    if (unsafe { (*values).valid }) == 0 {
        let mut dict: *mut __SlateRecord_dictobject_h_33 = _PyObject_GetManagedDict(obj);
        if dict.is_null() {
            dict = PyObject_GenericGetDict(obj, core::ptr::null_mut::<()>())
                as *mut __SlateRecord_dictobject_h_33;
            if dict.is_null() {
                return 1i32.wrapping_neg();
            }
            let res: i32 = store_instance_attr_dict(obj, dict, name, value);
            Py_DECREF__2!(dict);
            return res;
        }
        return store_instance_attr_dict(obj, dict, name, value);
    }
    store_instance_attr_lock_held(obj, values, name, value)
}

// Sanity check for managed dicts
/// Attempts to get an instance attribute from the inline values. Returns true
///
/// if successful, or false if the caller needs to lookup in the dictionary.
#[unsafe(no_mangle)]
extern "C-unwind" fn _PyObject_TryGetInstanceAttribute(
    obj: *mut _object,
    name: *mut _object,
    attr: *mut *mut _object,
) -> bool {
    let values: *mut _dictvalues = _PyObject_InlineValues(obj);
    if (unsafe { (*values).valid }) == 0 {
        return false;
    }
    let keys: *mut _dictkeysobject = CACHED_KEYS!(Py_TYPE!(obj));
    let ix: i64 = _PyDictKeys_StringLookupSplit(keys, name);
    if ix == (1i32.wrapping_neg() as i64) {
        unsafe {
            *attr = core::ptr::null_mut::<_object>();
        }
        return true;
    }
    let value: *mut _object = unsafe {
        *((*values).values.as_mut_ptr() as *mut *mut _object).wrapping_offset(ix as isize)
    };
    unsafe {
        *attr = Py_XNewRef!(value);
    }
    true
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyObject_IsInstanceDictEmpty(obj: *mut _object) -> i32 {
    let tp: *mut _typeobject = Py_TYPE!(obj);
    if (unsafe { (*tp).tp_dictoffset }) == 0 {
        return 1;
    }
    let mut dict: *mut __SlateRecord_dictobject_h_33 = unsafe { core::mem::zeroed() };
    if (unsafe { (*tp).tp_flags }) & 4u64 != 0 {
        let values: *mut _dictvalues = _PyObject_InlineValues(obj);
        if (unsafe { (*values).valid }) != 0 {
            let keys: *mut _dictkeysobject = CACHED_KEYS!(tp);
            let mut i: i64 = 0;
            while i < unsafe { (*keys).dk_nentries } {
                if !unsafe {
                    *((*values).values.as_mut_ptr() as *mut *mut _object)
                        .wrapping_offset(i as isize)
                }
                .is_null()
                {
                    return 0;
                }
                i = i.wrapping_add(1);
            }
            return 1;
        }
        dict = _PyObject_GetManagedDict(obj);
    } else {
        if (unsafe { (*tp).tp_flags }) & 16u64 != 0 {
            dict = _PyObject_GetManagedDict(obj);
        } else {
            let dictptr: *mut *mut _object = unsafe { _PyObject_ComputedDictPointer(obj) };
            dict = (unsafe { *dictptr }) as *mut __SlateRecord_dictobject_h_33;
        }
    }
    if dict.is_null() {
        return 1;
    }
    ((unsafe { (*dict).ma_used }) == 0) as i32
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyObject_VisitManagedDict(
    obj: *mut _object,
    mut visit: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
    arg: *mut (),
) -> i32 {
    let tp: *mut _typeobject = Py_TYPE!(obj);
    if (unsafe { (*tp).tp_flags }) & 16u64 == 0 {
        return 0;
    }
    let dict: *mut __SlateRecord_dictobject_h_33 =
        unsafe { (*_PyObject_ManagedDictPointer(obj)).dict };
    if !dict.is_null() {
        // GH-130327: If there's a managed dictionary available, we should
        //
        // *always* traverse it. The dict is responsible for traversing the
        //
        // inline values if it points to them.
        Py_VISIT__4!(dict, visit, arg);
    } else {
        if (unsafe { (*tp).tp_flags }) & 4u64 != 0 {
            let values: *mut _dictvalues = _PyObject_InlineValues(obj);
            if (unsafe { (*values).valid }) != 0 {
                let mut i: i64 = 0;
                while i < (((unsafe { (*values).capacity }) as i32) as i64) {
                    Py_VISIT!(
                        unsafe {
                            *((*values).values.as_mut_ptr() as *mut *mut _object)
                                .wrapping_offset(i as isize)
                        },
                        visit,
                        arg
                    );
                    i = i.wrapping_add(1);
                }
            }
        }
    }
    0
}

fn clear_inline_values(values: *mut _dictvalues) {
    if (unsafe { (*values).valid }) != 0 {
        FT_ATOMIC_STORE_UINT8!((*values).valid, 0i32);
        let mut i: i64 = 0;
        while i < (((unsafe { (*values).capacity }) as i32) as i64) {
            Py_CLEAR!(
                *unsafe { (*values).values.as_mut_ptr() as *mut *mut _object }
                    .wrapping_offset(i as isize)
            );
            i = i.wrapping_add(1);
        }
    }
}

fn set_dict_inline_values(obj: *mut _object, new_dict: *mut __SlateRecord_dictobject_h_33) {
    let values: *mut _dictvalues = _PyObject_InlineValues(obj);
    Py_XINCREF__2!(new_dict);
    FT_ATOMIC_STORE_PTR!((*_PyObject_ManagedDictPointer(obj)).dict, new_dict);
    clear_inline_values(values);
}

fn decref_maybe_delay(obj: *mut _object, delay: bool) {
    if delay {
        _PyObject_XDecRefDelayed(obj);
    } else {
        Py_XDECREF!(obj);
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyObject_SetManagedDict(obj: *mut _object, new_dict: *mut _object) -> i32 {
    let err: i32 = 0;
    let tp: *mut _typeobject = Py_TYPE!(obj);
    if (unsafe { (*tp).tp_flags }) & 4u64 != 0 {
        let dict: *mut __SlateRecord_dictobject_h_33 = _PyObject_GetManagedDict(obj);
        if dict.is_null() {
            set_dict_inline_values(obj, new_dict as *mut __SlateRecord_dictobject_h_33);
            return 0;
        }
        if _PyDict_DetachFromObject(dict, obj) == 0 {
            unsafe {
                (*_PyObject_ManagedDictPointer(obj)).dict =
                    Py_XNewRef!(new_dict) as *mut __SlateRecord_dictobject_h_33;
            }
            Py_DECREF__2!(dict);
            return 0;
        }
        return 1i32.wrapping_neg();
    } else {
        let dict: *mut __SlateRecord_dictobject_h_33 =
            unsafe { (*_PyObject_ManagedDictPointer(obj)).dict };
        FT_ATOMIC_STORE_PTR!(
            (*_PyObject_ManagedDictPointer(obj)).dict,
            Py_XNewRef!(new_dict) as *mut __SlateRecord_dictobject_h_33
        );
        decref_maybe_delay(dict as *mut _object, true);
    }
    err
}

fn detach_dict_from_object(mp: *mut __SlateRecord_dictobject_h_33, obj: *mut _object) -> i32 {
    if (unsafe { (*mp).ma_values }) != _PyObject_InlineValues(obj) {
        return 0;
    }
    // We could be called with an unlocked dict when the caller knows the
    //
    // values are already detached, so we assert after inline values check.
    let values: *mut _dictvalues = copy_values(unsafe { (*mp).ma_values });
    if values.is_null() {
        unsafe { PyErr_NoMemory() };
        return 1i32.wrapping_neg();
    }
    unsafe {
        (*mp).ma_values = values;
    }
    invalidate_and_clear_inline_values(_PyObject_InlineValues(obj));
    ASSERT_CONSISTENT!();
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyObject_ClearManagedDict(obj: *mut _object) {
    // This is called when the object is being freed or cleared
    //
    // by the GC and therefore known to have no references.
    if (unsafe { (*Py_TYPE!(obj)).tp_flags }) & 4u64 != 0 {
        let dict: *mut __SlateRecord_dictobject_h_33 = _PyObject_GetManagedDict(obj);
        if dict.is_null() {
            // We have no materialized dictionary and inline values
            //
            // that just need to be cleared.
            //
            // No dict to clear, we're done
            clear_inline_values(_PyObject_InlineValues(obj));
            return;
        } else {
            if (unsafe { (*dict).ma_values }) == _PyObject_InlineValues(obj) {
                // We have a materialized object which points at the inline
                //
                // values. We need to materialize the keys. Nothing can modify
                //
                // this object, but we need to lock the dictionary.
                let mut err: i32 = 0;
                err = detach_dict_from_object(dict, obj);
                if err != 0 {
                    // Must be out of memory
                    unsafe {
                        PyErr_FormatUnraisable(
                            c"Exception ignored while clearing an object managed dict".as_ptr(),
                        )
                    };
                    // Clear the dict
                    let interp: *mut _is = _PyInterpreterState_GET();
                    let oldkeys: *mut _dictkeysobject = unsafe { (*dict).ma_keys };
                    set_keys(dict, unsafe {
                        core::ptr::addr_of_mut!(
                            *(core::ptr::addr_of_mut!(empty_keys_struct) as *mut _dictkeysobject)
                        )
                    });
                    unsafe {
                        (*dict).ma_values = core::ptr::null_mut::<_dictvalues>();
                    }
                    dictkeys_decref(interp, oldkeys, IS_DICT_SHARED!() != 0);
                    STORE_USED__2!(dict, 0i32);
                    clear_inline_values(_PyObject_InlineValues(obj));
                }
            }
        }
    }
    Py_CLEAR__27!((*_PyObject_ManagedDictPointer(obj)).dict);
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyDict_DetachFromObject(
    mp: *mut __SlateRecord_dictobject_h_33,
    obj: *mut _object,
) -> i32 {
    detach_dict_from_object(mp, obj)
}

fn ensure_managed_dict(obj: *mut _object) -> *mut _object {
    let mut dict: *mut __SlateRecord_dictobject_h_33 = _PyObject_GetManagedDict(obj);
    if dict.is_null() {
        let tp: *mut _typeobject = Py_TYPE!(obj);
        let __v0: bool = (unsafe { (*tp).tp_flags }) & 4u64 != 0
            && (unsafe { (*_PyObject_InlineValues(obj)).valid }) != 0;
        if __v0 {
            dict = _PyObject_MaterializeManagedDict(obj);
        } else {
            dict = new_dict_with_shared_keys(_PyInterpreterState_GET(), CACHED_KEYS!(tp))
                as *mut __SlateRecord_dictobject_h_33;
            FT_ATOMIC_STORE_PTR_RELEASE!((*_PyObject_ManagedDictPointer(obj)).dict, dict);
        }
    }
    dict as *mut _object
}

fn ensure_nonmanaged_dict(obj: *mut _object, dictptr: *mut *mut _object) -> *mut _object {
    let mut cached: *mut _dictkeysobject = unsafe { core::mem::zeroed() };
    let mut dict: *mut _object = unsafe { *dictptr };
    if dict.is_null() {
        let tp: *mut _typeobject = Py_TYPE!(obj);
        let __v0: bool = if _PyType_HasFeature(tp, 512u64) != 0 {
            let __v1: *mut _dictkeysobject = CACHED_KEYS!(tp);
            cached = __v1;
            !__v1.is_null()
        } else {
            false
        };
        if __v0 {
            let interp: *mut _is = _PyInterpreterState_GET();
            dict = new_dict_with_shared_keys(interp, cached);
        } else {
            dict = PyDict_New();
        }
        FT_ATOMIC_STORE_PTR_RELEASE!(*dictptr, dict);
    }
    dict
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.dictobject.PyObject_GenericGetDict")]
extern "C-unwind" fn PyObject_GenericGetDict(obj: *mut _object, context: *mut ()) -> *mut _object {
    let tp: *mut _typeobject = Py_TYPE!(obj);
    if _PyType_HasFeature(tp, 16u64) != 0 {
        return Py_XNewRef!(ensure_managed_dict(obj));
    } else {
        let dictptr: *mut *mut _object = unsafe { _PyObject_ComputedDictPointer(obj) };
        if dictptr.is_null() {
            unsafe {
                PyErr_SetString(
                    PyExc_AttributeError,
                    c"This object has no __dict__".as_ptr(),
                )
            };
            return core::ptr::null_mut::<_object>();
        }
        return Py_XNewRef!(ensure_nonmanaged_dict(obj, dictptr));
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyObjectDict_SetItem(
    tp: *mut _typeobject,
    obj: *mut _object,
    dictptr: *mut *mut _object,
    key: *mut _object,
    value: *mut _object,
) -> i32 {
    let mut res: i32 = 0;
    let dict: *mut _object = ensure_nonmanaged_dict(obj, dictptr);
    if dict.is_null() {
        return 1i32.wrapping_neg();
    }
    res = _PyDict_SetItem_LockHeld(dict as *mut __SlateRecord_dictobject_h_33, key, value);
    ASSERT_CONSISTENT!();
    res
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyDictKeys_DecRef(keys: *mut _dictkeysobject) {
    let interp: *mut _is = _PyInterpreterState_GET();
    dictkeys_decref(interp, keys, false);
}

fn get_next_dict_keys_version(interp: *mut _is) -> u32 {
    if (unsafe { (*interp).dict_state.next_keys_version }) == 0 {
        return 0;
    }
    let mut v: u32 = 0;
    let __v1: u32 = unsafe { (*interp).dict_state.next_keys_version };
    unsafe {
        (*interp).dict_state.next_keys_version = __v1.wrapping_add(1);
    }
    v = __v1;
    v
}

/// In free-threaded builds the caller must ensure that the keys object is not
///
/// being mutated concurrently by another thread.
#[unsafe(no_mangle)]
extern "C-unwind" fn _PyDictKeys_GetVersionForCurrentState(
    interp: *mut _is,
    dictkeys: *mut _dictkeysobject,
) -> u32 {
    let mut dk_version: u32 = unsafe { (*dictkeys).dk_version };
    if dk_version != 0 {
        return dk_version;
    }
    dk_version = get_next_dict_keys_version(interp);
    FT_ATOMIC_STORE_UINT32_RELAXED__2!((*dictkeys).dk_version, dk_version);
    dk_version
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyDict_GetKeysVersionForCurrentState(
    interp: *mut _is,
    dict: *mut __SlateRecord_dictobject_h_33,
) -> u32 {
    let dk_version: u32 = _PyDictKeys_GetVersionForCurrentState(interp, unsafe { (*dict).ma_keys });
    ensure_shared_on_keys_version_assignment(dict);
    dk_version
}

fn validate_watcher_id(interp: *mut _is, watcher_id: i32) -> i32 {
    if watcher_id < 0 || watcher_id >= 8 {
        unsafe {
            PyErr_Format(
                PyExc_ValueError,
                c"Invalid dict watcher ID %d".as_ptr(),
                watcher_id,
            )
        };
        return 1i32.wrapping_neg();
    }
    let cb: Option<
        unsafe extern "C-unwind" fn(u32, *mut _object, *mut _object, *mut _object) -> i32,
    > = unsafe {
        *((*interp).dict_state.watchers.as_mut_ptr()
            as *mut Option<
                unsafe extern "C-unwind" fn(u32, *mut _object, *mut _object, *mut _object) -> i32,
            >)
            .wrapping_offset(watcher_id as isize)
    };
    if cb == None {
        unsafe {
            PyErr_Format(
                PyExc_ValueError,
                c"No dict watcher set for ID %d".as_ptr(),
                watcher_id,
            )
        };
        return 1i32.wrapping_neg();
    }
    0
}

// In free-threaded builds, Add/Clear serialize on watcher_mutex and publish
//
// callbacks with release stores. SendEvent reads them lock-free using
//
// acquire loads.
#[unsafe(no_mangle)]
extern "C-unwind" fn PyDict_Watch(watcher_id: i32, dict: *mut _object) -> i32 {
    if PyDict_Check!(dict) == 0 {
        unsafe { PyErr_SetString(PyExc_ValueError, c"Cannot watch non-dictionary".as_ptr()) };
        return 1i32.wrapping_neg();
    }
    let interp: *mut _is = _PyInterpreterState_GET();
    if validate_watcher_id(interp, watcher_id) != 0 {
        return 1i32.wrapping_neg();
    }
    unsafe {
        (*(dict as *mut __SlateRecord_dictobject_h_33))._ma_watcher_tag =
            (*(dict as *mut __SlateRecord_dictobject_h_33))._ma_watcher_tag | 1u64 << watcher_id;
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyDict_Unwatch(watcher_id: i32, dict: *mut _object) -> i32 {
    if PyDict_Check!(dict) == 0 {
        unsafe { PyErr_SetString(PyExc_ValueError, c"Cannot watch non-dictionary".as_ptr()) };
        return 1i32.wrapping_neg();
    }
    let interp: *mut _is = _PyInterpreterState_GET();
    if validate_watcher_id(interp, watcher_id) != 0 {
        return 1i32.wrapping_neg();
    }
    let __v2: u64 = (unsafe { (*(dict as *mut __SlateRecord_dictobject_h_33))._ma_watcher_tag })
        & !(1u64 << watcher_id);
    unsafe {
        (*(dict as *mut __SlateRecord_dictobject_h_33))._ma_watcher_tag = __v2;
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyDict_AddWatcher(
    callback: Option<
        unsafe extern "C-unwind" fn(u32, *mut _object, *mut _object, *mut _object) -> i32,
    >,
) -> i32 {
    // Start at 2, as 0 and 1 are reserved for CPython
    let mut i: i32 = unsafe { core::mem::zeroed() };
    let mut interp: *mut _is = unsafe { core::mem::zeroed() };
    let mut watcher_id: i32 = unsafe { core::mem::zeroed() };
    unsafe {
        watcher_id = 1i32.wrapping_neg();
        interp = _PyInterpreterState_GET();
        FT_MUTEX_LOCK_FLAGS!();
        i = 2;
        '__join_0: {
            '__join_4: {
                while i < 8 {
                    if *((*interp).dict_state.watchers.as_mut_ptr()
                        as *mut Option<
                            unsafe extern "C-unwind" fn(
                                u32,
                                *mut _object,
                                *mut _object,
                                *mut _object,
                            ) -> i32,
                        >)
                        .wrapping_offset(i as isize)
                        == None
                    {
                        break '__join_4;
                    } else {
                        i = i.wrapping_add(1);
                    }
                }
                PyErr_SetString(
                    PyExc_RuntimeError,
                    c"no more dict watcher IDs available".as_ptr(),
                );
                break '__join_0;
            }
            FT_ATOMIC_STORE_PTR_RELEASE!(
                *((*interp).dict_state.watchers.as_mut_ptr()
                    as *mut Option<
                        unsafe extern "C-unwind" fn(
                            u32,
                            *mut _object,
                            *mut _object,
                            *mut _object,
                        ) -> i32,
                    >)
                    .wrapping_offset(i as isize),
                callback
            );
            watcher_id = i;
        }
        FT_MUTEX_UNLOCK!();
        return watcher_id;
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyDict_ClearWatcher(watcher_id: i32) -> i32 {
    let mut interp: *mut _is = unsafe { core::mem::zeroed() };
    let mut res: i32 = unsafe { core::mem::zeroed() };
    unsafe {
        res = 0;
        interp = _PyInterpreterState_GET();
        FT_MUTEX_LOCK_FLAGS!();
        if validate_watcher_id(interp, watcher_id) != 0 {
            res = 1i32.wrapping_neg();
        } else {
            FT_ATOMIC_STORE_PTR_RELEASE__4!(
                *((*interp).dict_state.watchers.as_mut_ptr()
                    as *mut Option<
                        unsafe extern "C-unwind" fn(
                            u32,
                            *mut _object,
                            *mut _object,
                            *mut _object,
                        ) -> i32,
                    >)
                    .wrapping_offset(watcher_id as isize)
            );
        }
        FT_MUTEX_UNLOCK!();
        return res;
    }
}

fn dict_event_name(event: u32) -> *const i8 {
    match event as u32 {
        0 => {
            return c"PyDict_EVENT_ADDED".as_ptr();
        }
        1 => {
            return c"PyDict_EVENT_MODIFIED".as_ptr();
        }
        2 => {
            return c"PyDict_EVENT_DELETED".as_ptr();
        }
        3 => {
            return c"PyDict_EVENT_CLONED".as_ptr();
        }
        4 => {
            return c"PyDict_EVENT_CLEARED".as_ptr();
        }
        5 => {
            return c"PyDict_EVENT_DEALLOCATED".as_ptr();
        }
        _ => {}
    }
    Py_UNREACHABLE!();
    unsafe { core::mem::zeroed() }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyDict_SendEvent(
    mut watcher_bits: i32,
    event: u32,
    mp: *mut __SlateRecord_dictobject_h_33,
    key: *mut _object,
    value: *mut _object,
) {
    let interp: *mut _is = _PyInterpreterState_GET();
    for i in 0i32..8i32 {
        if watcher_bits & 1 != 0 {
            let cb: Option<
                unsafe extern "C-unwind" fn(u32, *mut _object, *mut _object, *mut _object) -> i32,
            > = unsafe {
                *((*interp).dict_state.watchers.as_mut_ptr()
                    as *mut Option<
                        unsafe extern "C-unwind" fn(
                            u32,
                            *mut _object,
                            *mut _object,
                            *mut _object,
                        ) -> i32,
                    >)
                    .wrapping_offset(i as isize)
            };
            let __v0: bool =
                cb != None && (unsafe { cb.unwrap()(event, mp as *mut _object, key, value) }) < 0;
            if __v0 {
                // We don't want to resurrect the dict by potentially having an
                //
                // unraisablehook keep a reference to it, so we don't pass the
                //
                // dict as context, just an informative string message.  Dict
                //
                // repr can call arbitrary code, so we invent a simpler version.
                unsafe {
                    PyErr_FormatUnraisable(
                        c"Exception ignored in %s watcher callback for <dict at %p>".as_ptr(),
                        dict_event_name(event),
                        mp,
                    )
                };
            }
        }
        watcher_bits >>= 1i32;
    }
}
