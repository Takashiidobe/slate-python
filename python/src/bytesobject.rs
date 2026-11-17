//! bytes object implementation
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
//! Format codes
//! F_LJUST      '-'
//! F_SIGN       '+'
//! F_BLANK      ' '
//! F_ALT        '#'
//! F_ZERO       '0'
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
// "alternate" formatting. it's format_code
// specific
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

/// Always force the list to the expected size.
macro_rules! FIX_PREALLOC_SIZE {
    ($list:expr, $count:expr) => {
        Py_SET_SIZE!($list, $count);
    };
}

/// 5 splits gives 6 elements
macro_rules! PREALLOC_SIZE {
    ($maxsplit:expr) => {
        if ($maxsplit) >= 12 {
            12i64
        } else {
            $maxsplit.wrapping_add(1)
        }
    };
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

macro_rules! PyBytes_AS_STRING__2 {
    ($op:expr) => {
        PyBytes_AS_STRING(_PyObject_CAST__2!($op))
    };
}

macro_rules! PyBytes_AS_STRING {
    ($op:expr) => {
        PyBytes_AS_STRING(_PyObject_CAST!($op))
    };
}

macro_rules! PyBytes_CheckExact__2 {
    ($op:expr) => {
        Py_IS_TYPE__2!($op, unsafe { core::ptr::addr_of_mut!(PyBytes_Type) })
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

macro_rules! PyBytes_GET_SIZE__2 {
    ($self:expr) => {
        PyBytes_GET_SIZE(_PyObject_CAST__2!($self))
    };
}

macro_rules! PyDict_GET_SIZE {
    ($op:expr) => {
        PyDict_GET_SIZE(_PyObject_CAST!($op))
    };
}

/// Mask the old API with a call to the new API for code compiled under
/// Python 2.0:
macro_rules! PyErr_BadInternalCall__5 {
    () => {
        unsafe {
            _PyErr_BadInternalCall(
                c"/home/takashi/c-corpus/cpython/Objects/bytesobject.c".as_ptr(),
                3227i32,
            )
        };
    };
}

/// Mask the old API with a call to the new API for code compiled under
/// Python 2.0:
macro_rules! PyErr_BadInternalCall__41 {
    () => {
        unsafe {
            _PyErr_BadInternalCall(
                c"/home/takashi/c-corpus/cpython/Objects/bytesobject.c".as_ptr(),
                1267i32,
            )
        };
    };
}

/// Mask the old API with a call to the new API for code compiled under
/// Python 2.0:
macro_rules! PyErr_BadInternalCall__60 {
    () => {
        unsafe {
            _PyErr_BadInternalCall(
                c"/home/takashi/c-corpus/cpython/Objects/bytesobject.c".as_ptr(),
                3425i32,
            )
        };
    };
}

/// Mask the old API with a call to the new API for code compiled under
/// Python 2.0:
macro_rules! PyErr_BadInternalCall__90 {
    () => {
        unsafe {
            _PyErr_BadInternalCall(
                c"/home/takashi/c-corpus/cpython/Objects/bytesobject.c".as_ptr(),
                1905i32,
            )
        };
    };
}

/// Mask the old API with a call to the new API for code compiled under
/// Python 2.0:
macro_rules! PyErr_BadInternalCall__119 {
    () => {
        unsafe {
            _PyErr_BadInternalCall(
                c"/home/takashi/c-corpus/cpython/Objects/bytesobject.c".as_ptr(),
                611i32,
            )
        };
    };
}

/// Mask the old API with a call to the new API for code compiled under
/// Python 2.0:
macro_rules! PyErr_BadInternalCall__137 {
    () => {
        unsafe {
            _PyErr_BadInternalCall(
                c"/home/takashi/c-corpus/cpython/Objects/bytesobject.c".as_ptr(),
                3018i32,
            )
        };
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

macro_rules! PyList_SET_ITEM__2 {
    ($op:expr, $index:expr, $value:expr) => {
        PyList_SET_ITEM(
            _PyObject_CAST!($op),
            ($index) as i64,
            _PyObject_CAST!($value),
        );
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

macro_rules! PyMem_NEW__3 {
    ($type:ty, $n:expr) => {
        PyMem_New__4!($type, $n)
    };
}

macro_rules! PyMem_New__4 {
    ($type:ty, $n:expr) => {{
        let __v: *mut $type =
            if (($n) as u64) > 9223372036854775807 / (core::mem::size_of::<$type>() as u64) {
                core::ptr::null_mut::<__SlateRecord_pybuffer_h_33>()
            } else {
                (unsafe {
                    PyMem_Malloc((($n) as u64).wrapping_mul(core::mem::size_of::<$type>() as u64))
                }) as *mut $type
            };
        __v
    }};
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

macro_rules! PyUnicode_1BYTE_DATA {
    ($op:expr) => {
        _Py_STATIC_CAST!(*mut u8, PyUnicode_DATA!($op))
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

/// Argument must be a char or an int in [-128, 127] or [0, 255].
macro_rules! Py_CHARMASK__2 {
    ($c:expr) => {
        (($c) & 255) as u8
    };
}

/// Argument must be a char or an int in [-128, 127] or [0, 255].
macro_rules! Py_CHARMASK {
    ($c:expr) => {
        ((($c) as i32) & 255) as u8
    };
}

/// Argument must be a char or an int in [-128, 127] or [0, 255].
macro_rules! Py_CHARMASK__4 {
    ($c:expr) => {
        (($c) & 255) as u8
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

macro_rules! Py_ISALPHA {
    ($c:expr) => {
        (unsafe { _Py_ctype_table[(Py_CHARMASK!($c) as i32) as usize] }) & 3u32
    };
}

macro_rules! Py_ISDIGIT__3 {
    ($c:expr) => {
        (unsafe { _Py_ctype_table[(Py_CHARMASK!($c) as i32) as usize] }) & 4
    };
}

macro_rules! Py_ISDIGIT {
    ($c:expr) => {
        (unsafe { _Py_ctype_table[(Py_CHARMASK__2!($c) as i32) as usize] }) & 4
    };
}

macro_rules! Py_ISSPACE {
    ($c:expr) => {
        (unsafe { _Py_ctype_table[(Py_CHARMASK!($c) as i32) as usize] }) & 8
    };
}

macro_rules! Py_ISXDIGIT__2 {
    ($c:expr) => {
        (unsafe { _Py_ctype_table[(Py_CHARMASK!($c) as i32) as usize] }) & 16
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

macro_rules! Py_SET_SIZE {
    ($ob:expr, $size:expr) => {
        Py_SET_SIZE(_PyVarObject_CAST!($ob), $size);
    };
}

macro_rules! Py_SET_SIZE__3 {
    ($ob:expr, $size:expr) => {
        Py_SET_SIZE(_PyVarObject_CAST__2!($ob), $size);
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
macro_rules! Py_VISIT__22 {
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

macro_rules! STRINGLIB_BLOOM_ADD {
    ($mask:expr, $ch:expr) => {
        let __v: u64 = $mask;
        let __v2: u64 = __v | 1u64 << ((($ch) as i32) & 63i32);
        $mask = __v2;
    };
}

macro_rules! STRINGLIB_BLOOM {
    ($mask:expr, $ch:expr) => {
        ($mask) & 1u64 << ((($ch) as i32) & 63i32)
    };
}

macro_rules! STRINGLIB_GET_EMPTY__2 {
    () => {
        bytes_get_empty()
    };
}

macro_rules! STRINGLIB_ISLINEBREAK {
    ($x:expr) => {
        (($x) as i32) == 10 || (($x) as i32) == 13
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
macro_rules! _PyVarObject_CAST__2 {
    ($op:expr) => {
        _Py_CAST__2!($op)
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

macro_rules! _striterobject_CAST {
    ($op:expr) => {
        ($op) as *mut __SlateRecord_bytesobject_c_3286
    };
}

macro_rules! findchar {
    ($target:expr, $target_len:expr, $c:expr) => {
        ((unsafe {
            memchr(
                (($target) as *const ()) as *const core::ffi::c_void,
                (($c) as i32) as i32,
                (($target_len) as u64) as usize,
            ) as *mut ()
        }) as *const ()) as *mut i8
    };
}

unsafe extern "C" {
    static mut PyLong_Type: _typeobject;
    static mut PyType_Type: _typeobject;
    static mut _Py_NoneStruct: _object;
    static mut _Py_NotImplementedStruct: _object;
    static mut PyByteArray_Type: _typeobject;
    static mut _PyByteArray_empty_string: [i8; 0];
    static mut PyExc_IndexError: *mut _object;
    static mut PyExc_OverflowError: *mut _object;
    static mut PyExc_RuntimeError: *mut _object;
    static mut PyExc_SystemError: *mut _object;
    static mut PyExc_TypeError: *mut _object;
    static mut PyExc_ValueError: *mut _object;
    static mut PyExc_DeprecationWarning: *mut _object;
    static mut PyExc_BytesWarning: *mut _object;
    static mut _Py_FalseStruct: _longobject;
    static mut _Py_TrueStruct: _longobject;
    static mut PyTuple_Type: _typeobject;
    static mut PyList_Type: _typeobject;
    static mut PySlice_Type: _typeobject;
    static mut Py_hexdigits: *const i8;
    static mut _Py_ctype_table: [u32; 256];
    static mut _Py_isspace__doc__: [i8; 0];
    static mut _Py_isalpha__doc__: [i8; 0];
    static mut _Py_isalnum__doc__: [i8; 0];
    static mut _Py_isascii__doc__: [i8; 0];
    static mut _Py_isdigit__doc__: [i8; 0];
    static mut _Py_islower__doc__: [i8; 0];
    static mut _Py_isupper__doc__: [i8; 0];
    static mut _Py_istitle__doc__: [i8; 0];
    static mut _Py_lower__doc__: [i8; 0];
    static mut _Py_upper__doc__: [i8; 0];
    static mut _Py_title__doc__: [i8; 0];
    static mut _Py_capitalize__doc__: [i8; 0];
    static mut _Py_swapcase__doc__: [i8; 0];
    #[thread_local]
    static mut _Py_tss_tstate: *mut _ts;
    static mut _PyRuntime: pyruntimestate;
    static mut _PyLong_DigitValue: [u8; 256];
    fn memrchr(arg0: *const (), arg1: i32, arg2: u64) -> *mut ();
    fn index(arg0: *const i8, arg1: i32) -> *mut i8;
    fn PyMem_Malloc(arg0: u64) -> *mut ();
    fn PyMem_Free(arg0: *mut ());
    fn PyObject_CheckBuffer(arg0: *mut _object) -> i32;
    fn PyObject_GetBuffer(
        arg0: *mut _object,
        arg1: *mut __SlateRecord_pybuffer_h_33,
        arg2: i32,
    ) -> i32;
    fn PyBuffer_ToContiguous(
        arg0: *mut (),
        arg1: *const __SlateRecord_pybuffer_h_33,
        arg2: i64,
        arg3: i8,
    ) -> i32;
    fn PyBuffer_FillInfo(
        arg0: *mut __SlateRecord_pybuffer_h_33,
        arg1: *mut _object,
        arg2: *mut (),
        arg3: i64,
        arg4: i32,
        arg5: i32,
    ) -> i32;
    fn PyBuffer_Release(arg0: *mut __SlateRecord_pybuffer_h_33);
    fn PyType_IsSubtype(arg0: *mut _typeobject, arg1: *mut _typeobject) -> i32;
    fn PyType_GenericAlloc(arg0: *mut _typeobject, arg1: i64) -> *mut _object;
    fn PyObject_ASCII(arg0: *mut _object) -> *mut _object;
    fn PyObject_SelfIter(arg0: *mut _object) -> *mut _object;
    fn PyObject_GenericGetAttr(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn PyObject_IsTrue(arg0: *mut _object) -> i32;
    fn Py_GetConstant(arg0: u32) -> *mut _object;
    fn _Py_NewReference(arg0: *mut _object);
    fn _Py_NewReferenceNoTotal(arg0: *mut _object);
    fn _Py_Dealloc(arg0: *mut _object);
    fn PyObject_Malloc(arg0: u64) -> *mut ();
    fn PyObject_Calloc(arg0: u64, arg1: u64) -> *mut ();
    fn PyObject_Realloc(arg0: *mut (), arg1: u64) -> *mut ();
    fn PyObject_Free(arg0: *mut ());
    fn _PyObject_GC_New(arg0: *mut _typeobject) -> *mut _object;
    fn PyObject_GC_Del(arg0: *mut ());
    fn Py_HashBuffer(arg0: *const (), arg1: i64) -> i64;
    fn PyByteArray_FromStringAndSize(arg0: *const i8, arg1: i64) -> *mut _object;
    fn PyByteArray_Resize(arg0: *mut _object, arg1: i64) -> i32;
    fn PyUnicode_FromEncodedObject(
        arg0: *mut _object,
        arg1: *const i8,
        arg2: *const i8,
    ) -> *mut _object;
    fn PyUnicode_AsEncodedString(
        arg0: *mut _object,
        arg1: *const i8,
        arg2: *const i8,
    ) -> *mut _object;
    fn PyUnicode_AsUTF8AndSize(arg0: *mut _object, arg1: *mut i64) -> *const i8;
    fn PyUnicode_New(arg0: i64, arg1: u32) -> *mut _object;
    fn PyErr_SetString(arg0: *mut _object, arg1: *const i8);
    fn PyErr_Occurred() -> *mut _object;
    fn PyErr_Clear();
    fn PyErr_ExceptionMatches(arg0: *mut _object) -> i32;
    fn PyErr_BadArgument() -> i32;
    fn PyErr_NoMemory() -> *mut _object;
    fn PyErr_Format(arg0: *mut _object, arg1: *const i8, ...) -> *mut _object;
    fn _PyErr_BadInternalCall(arg0: *const i8, arg1: i32);
    fn PyLong_FromSsize_t(arg0: i64) -> *mut _object;
    fn PyLong_AsLongAndOverflow(arg0: *mut _object, arg1: *mut i32) -> i64;
    fn PyLong_AsSsize_t(arg0: *mut _object) -> i64;
    fn PyLong_AsInt(arg0: *mut _object) -> i32;
    fn PyBool_FromLong(arg0: i64) -> *mut _object;
    fn PyFloat_AsDouble(arg0: *mut _object) -> f64;
    fn PyTuple_New(arg0: i64) -> *mut _object;
    fn PyTuple_GetItem(arg0: *mut _object, arg1: i64) -> *mut _object;
    fn PyList_New(arg0: i64) -> *mut _object;
    fn PyList_Append(arg0: *mut _object, arg1: *mut _object) -> i32;
    fn PyList_Reverse(arg0: *mut _object) -> i32;
    fn PySlice_Unpack(arg0: *mut _object, arg1: *mut i64, arg2: *mut i64, arg3: *mut i64) -> i32;
    fn PySlice_AdjustIndices(arg0: i64, arg1: *mut i64, arg2: *mut i64, arg3: i64) -> i64;
    fn PyErr_WarnEx(arg0: *mut _object, arg1: *const i8, arg2: i64) -> i32;
    fn PyErr_WarnFormat(arg0: *mut _object, arg1: i64, arg2: *const i8, ...) -> i32;
    fn Py_BuildValue(arg0: *const i8, ...) -> *mut _object;
    fn PyEval_SaveThread() -> *mut _ts;
    fn PyEval_RestoreThread(arg0: *mut _ts);
    fn _PyEval_SliceIndex(arg0: *mut _object, arg1: *mut i64) -> i32;
    fn PyObject_GetItem(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn PyObject_GetIter(arg0: *mut _object) -> *mut _object;
    fn PyIter_Next(arg0: *mut _object) -> *mut _object;
    fn PyNumber_Check(arg0: *mut _object) -> i32;
    fn PyIndex_Check(arg0: *mut _object) -> i32;
    fn PyNumber_AsSsize_t(arg0: *mut _object, arg1: *mut _object) -> i64;
    fn PyNumber_Long(arg0: *mut _object) -> *mut _object;
    fn PySequence_Fast(arg0: *mut _object, arg1: *const i8) -> *mut _object;
    fn PyObject_CallOneArg(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn PyObject_LengthHint(arg0: *mut _object, arg1: i64) -> i64;
    fn PyOS_double_to_string(arg0: f64, arg1: i8, arg2: i32, arg3: i32, arg4: *mut i32) -> *mut i8;
    fn _PyNumber_Index(arg0: *mut _object) -> *mut _object;
    fn _Py_bytes_isspace(arg0: *const i8, arg1: i64) -> *mut _object;
    fn _Py_bytes_isalpha(arg0: *const i8, arg1: i64) -> *mut _object;
    fn _Py_bytes_isalnum(arg0: *const i8, arg1: i64) -> *mut _object;
    fn _Py_bytes_isascii(arg0: *const i8, arg1: i64) -> *mut _object;
    fn _Py_bytes_isdigit(arg0: *const i8, arg1: i64) -> *mut _object;
    fn _Py_bytes_islower(arg0: *const i8, arg1: i64) -> *mut _object;
    fn _Py_bytes_isupper(arg0: *const i8, arg1: i64) -> *mut _object;
    fn _Py_bytes_istitle(arg0: *const i8, arg1: i64) -> *mut _object;
    fn _Py_bytes_lower(arg0: *mut i8, arg1: *const i8, arg2: i64);
    fn _Py_bytes_upper(arg0: *mut i8, arg1: *const i8, arg2: i64);
    fn _Py_bytes_title(arg0: *mut i8, arg1: *const i8, arg2: i64);
    fn _Py_bytes_capitalize(arg0: *mut i8, arg1: *const i8, arg2: i64);
    fn _Py_bytes_swapcase(arg0: *mut i8, arg1: *const i8, arg2: i64);
    fn _Py_bytes_find(
        arg0: *const i8,
        arg1: i64,
        arg2: *mut _object,
        arg3: i64,
        arg4: i64,
    ) -> *mut _object;
    fn _Py_bytes_index(
        arg0: *const i8,
        arg1: i64,
        arg2: *mut _object,
        arg3: i64,
        arg4: i64,
    ) -> *mut _object;
    fn _Py_bytes_rfind(
        arg0: *const i8,
        arg1: i64,
        arg2: *mut _object,
        arg3: i64,
        arg4: i64,
    ) -> *mut _object;
    fn _Py_bytes_rindex(
        arg0: *const i8,
        arg1: i64,
        arg2: *mut _object,
        arg3: i64,
        arg4: i64,
    ) -> *mut _object;
    fn _Py_bytes_count(
        arg0: *const i8,
        arg1: i64,
        arg2: *mut _object,
        arg3: i64,
        arg4: i64,
    ) -> *mut _object;
    fn _Py_bytes_contains(arg0: *const i8, arg1: i64, arg2: *mut _object) -> i32;
    fn _Py_bytes_startswith(
        arg0: *const i8,
        arg1: i64,
        arg2: *mut _object,
        arg3: i64,
        arg4: i64,
    ) -> *mut _object;
    fn _Py_bytes_endswith(
        arg0: *const i8,
        arg1: i64,
        arg2: *mut _object,
        arg3: i64,
        arg4: i64,
    ) -> *mut _object;
    fn _Py_bytes_maketrans(
        arg0: *mut __SlateRecord_pybuffer_h_33,
        arg1: *mut __SlateRecord_pybuffer_h_33,
    ) -> *mut _object;
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
    fn _PyEval_GetBuiltin(arg0: *mut _object) -> *mut _object;
    fn _PyLong_FormatBytesWriter(
        arg0: *mut __SlateRecord_pycore_bytesobject_h_89,
        arg1: *mut i8,
        arg2: *mut _object,
        arg3: i32,
        arg4: i32,
    ) -> *mut i8;
    fn _PyList_GetItemRef(arg0: *mut __SlateRecord_listobject_h_22, arg1: i64) -> *mut _object;
    fn _PyObject_LookupSpecial(arg0: *mut _object, arg1: *mut _object) -> *mut _object;
    fn _Py_strhex_with_sep(
        arg0: *const i8,
        arg1: i64,
        arg2: *mut _object,
        arg3: i32,
    ) -> *mut _object;
    fn _PyUnicode_FormatLong(arg0: *mut _object, arg1: i32, arg2: i32, arg3: i32) -> *mut _object;
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

fn Py_SET_SIZE(ob: *mut __SlateRecord_object_h_172, size: i64) {
    unsafe {
        (*ob).ob_size = size;
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

// Track types initialized using _PyStaticType_InitBuiltin().
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

#[repr(C)]
#[derive(Clone, Copy)]
struct _dictkeysobject {}

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
    __slate_bits_0: __slate_bits::__SlateBits146U0,
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
struct _dictvalues {}

#[unsafe(no_mangle)]
static mut PyBytes_Type: _typeobject = {
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
    __t0.tp_name = (b"bytes\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::offset_of!(__SlateRecord_bytesobject_h_15, ob_sval) as u64)
        .wrapping_add(1u64) as i64;
    __t0.tp_itemsize = (core::mem::size_of::<i8>() as u64) as i64;
    __t0.tp_dealloc = None;
    __t0.tp_vectorcall_offset = 0i64;
    __t0.tp_getattr = None;
    __t0.tp_setattr = None;
    __t0.tp_as_async = core::ptr::null_mut::<__SlateRecord_object_h_135>();
    __t0.tp_repr = Some(bytes_repr);
    __t0.tp_as_number = unsafe { core::ptr::addr_of_mut!(bytes_as_number) };
    __t0.tp_as_sequence = unsafe { core::ptr::addr_of_mut!(bytes_as_sequence) };
    __t0.tp_as_mapping = unsafe { core::ptr::addr_of_mut!(bytes_as_mapping) };
    __t0.tp_hash = Some(bytes_hash);
    __t0.tp_call = None;
    __t0.tp_str = Some(bytes_str);
    __t0.tp_getattro = unsafe {
        core::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut _object, *mut _object) -> *mut _object>,
        >(PyObject_GenericGetAttr as *const ())
    };
    __t0.tp_setattro = None;
    __t0.tp_as_buffer = unsafe { core::ptr::addr_of_mut!(bytes_as_buffer) };
    __t0.tp_flags = 138413056u64;
    __t0.tp_doc = unsafe { core::ptr::addr_of!(bytes_doc.0) as *const i8 };
    __t0.tp_traverse = None;
    __t0.tp_clear = None;
    __t0.tp_richcompare = Some(bytes_richcompare);
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = Some(bytes_iter);
    __t0.tp_iternext = None;
    __t0.tp_methods = unsafe { core::ptr::addr_of_mut!(bytes_methods.0) as *mut PyMethodDef };
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0.tp_getset = core::ptr::null_mut::<PyGetSetDef>();
    __t0.tp_base = core::ptr::null_mut::<_typeobject>();
    __t0.tp_dict = core::ptr::null_mut::<_object>();
    __t0.tp_descr_get = None;
    __t0.tp_descr_set = None;
    __t0.tp_dictoffset = 0i64;
    __t0.tp_init = None;
    __t0.tp_alloc = Some(bytes_alloc);
    __t0.tp_new = Some(bytes_new);
    __t0.tp_free = unsafe {
        core::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
            PyObject_Free as *const (),
        )
    };
    __t0.tp_version_tag = 10u32;
    __t0
};

#[unsafe(no_mangle)]
static mut PyBytesIter_Type: _typeobject = {
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
    __t0.tp_name = (b"bytes_iterator\0".as_ptr() as *mut i8) as *const i8;
    __t0.tp_basicsize = (core::mem::size_of::<__SlateRecord_bytesobject_c_3286>() as u64) as i64;
    __t0.tp_itemsize = 0i64;
    __t0.tp_dealloc = Some(striter_dealloc);
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
    __t0.tp_traverse = Some(striter_traverse);
    __t0.tp_clear = None;
    __t0.tp_richcompare = None;
    __t0.tp_weaklistoffset = 0i64;
    __t0.tp_iter = unsafe {
        core::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut _object) -> *mut _object>,
        >(PyObject_SelfIter as *const ())
    };
    __t0.tp_iternext = Some(striter_next);
    __t0.tp_methods = unsafe { core::ptr::addr_of_mut!(striter_methods.0) as *mut PyMethodDef };
    __t0.tp_members = core::ptr::null_mut::<PyMemberDef>();
    __t0
};

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
struct _PyExecutorObject {}

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

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pystate_h_78 {
    __slate_bits_0: __slate_bits::__SlateBits201U0,
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
struct __SlateRecord_pycore_structs_h_32_2 {
    code: u8,
    arg: u8,
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
struct code_arena_st {}

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

fn _PyIndex_Check(obj: *mut _object) -> i32 {
    let tp_as_number: *mut __SlateRecord_object_h_106 = unsafe { (*Py_TYPE!(obj)).tp_as_number };
    (!tp_as_number.is_null() && (unsafe { (*tp_as_number).nb_index }) != None) as i32
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyBytes_FormatEx(
    format: *const i8,
    format_len: i64,
    mut args: *mut _object,
    use_bytearray: i32,
) -> *mut _object {
    let mut pos: *mut i8 = unsafe { core::mem::zeroed() };
    let mut len: i64 = unsafe { core::mem::zeroed() };
    let mut base: i32 = unsafe { core::mem::zeroed() };
    // Fast path
    let mut alternate: i32 = unsafe { core::mem::zeroed() };
    let mut __v64: bool = unsafe { core::mem::zeroed() };
    let mut __v79: bool = unsafe { core::mem::zeroed() };
    let mut __v101: bool = unsafe { core::mem::zeroed() };
    let mut pcount: i32 = unsafe { core::mem::zeroed() };
    let mut key: *mut _object = unsafe { core::mem::zeroed() };
    let mut keylen: i64 = unsafe { core::mem::zeroed() };
    let mut keystart: *const i8 = unsafe { core::mem::zeroed() };
    let mut alloc: i64 = unsafe { core::mem::zeroed() };
    // For byte_converter()
    let mut onechar: i8 = unsafe { core::mem::zeroed() };
    let mut len_1: i64 = unsafe { core::mem::zeroed() };
    let mut sign: i32 = unsafe { core::mem::zeroed() };
    let mut pbuf: *const i8 = unsafe { core::mem::zeroed() };
    let mut temp: *mut _object = unsafe { core::mem::zeroed() };
    let mut v: *mut _object = unsafe { core::mem::zeroed() };
    let mut fill: i32 = unsafe { core::mem::zeroed() };
    let mut c: i32 = unsafe { core::mem::zeroed() };
    let mut prec: i32 = unsafe { core::mem::zeroed() };
    let mut width: i64 = unsafe { core::mem::zeroed() };
    // Got a format specifier
    let mut flags: i32 = unsafe { core::mem::zeroed() };
    let mut __v136: bool = unsafe { core::mem::zeroed() };
    let mut __v137: bool = unsafe { core::mem::zeroed() };
    let mut __v138: bool = unsafe { core::mem::zeroed() };
    let mut __v139: bool = unsafe { core::mem::zeroed() };
    let mut __v140: bool = unsafe { core::mem::zeroed() };
    let mut writer: core::mem::MaybeUninit<__SlateRecord_pycore_bytesobject_h_89> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_141: *mut __SlateRecord_pycore_bytesobject_h_89 =
        core::ptr::addr_of_mut!(writer) as *mut __SlateRecord_pycore_bytesobject_h_89;
    let mut dict: *mut _object = unsafe { core::mem::zeroed() };
    let mut args_owned: i32 = unsafe { core::mem::zeroed() };
    let mut fmtcnt: i64 = unsafe { core::mem::zeroed() };
    let mut argidx: i64 = unsafe { core::mem::zeroed() };
    let mut arglen: i64 = unsafe { core::mem::zeroed() };
    let mut res: *mut i8 = unsafe { core::mem::zeroed() };
    let mut fmt: *const i8 = unsafe { core::mem::zeroed() };
    unsafe {
        args_owned = 0;
        dict = core::ptr::null_mut::<_object>();
        if args.is_null() {
            PyErr_BadInternalCall__119!();
            return core::ptr::null_mut::<_object>();
        } else {
            fmt = format;
            fmtcnt = format_len;
            _PyBytesWriter_Init(core::ptr::addr_of_mut!(*__slate_slot_141));
            (*__slate_slot_141).use_bytearray = use_bytearray;
            res =
                _PyBytesWriter_Alloc(core::ptr::addr_of_mut!(*__slate_slot_141), fmtcnt) as *mut i8;
            if res.is_null() {
                return core::ptr::null_mut::<_object>();
            } else {
                if use_bytearray == 0 {
                    (*__slate_slot_141).overallocate = 1;
                }
                if PyTuple_Check!(args) != 0 {
                    arglen = PyTuple_GET_SIZE!(args);
                    argidx = 0;
                } else {
                    arglen = 1i32.wrapping_neg() as i64;
                    argidx = 2i32.wrapping_neg() as i64;
                }
                if !unsafe { (*Py_TYPE!(args)).tp_as_mapping }.is_null() {
                    __v140 = (*(*Py_TYPE!(args)).tp_as_mapping).mp_subscript != None;
                } else {
                    __v140 = false;
                }
                if __v140 {
                    __v139 = PyTuple_Check!(args) == 0;
                } else {
                    __v139 = false;
                }
                if __v139 {
                    __v138 = PyBytes_Check!(args) == 0;
                } else {
                    __v138 = false;
                }
                if __v138 {
                    __v137 = PyUnicode_Check!(args) == 0;
                } else {
                    __v137 = false;
                }
                if __v137 {
                    __v136 = PyByteArray_Check!(args) == 0;
                } else {
                    __v136 = false;
                }
                if __v136 {
                    dict = args;
                }
                '__join_2: {
                    '__join_153: {
                        '__join_141: {
                            '__join_117: {
                                '__join_95: {
                                    '__join_81: {
                                        '__join_37: {
                                            '__join_15: {
                                                '__join_52: {
                                                    '__join_100: {
                                                        '__join_122: {
                                                            '__loop_8: loop {
                                                                let __v0: i64 =
                                                                    fmtcnt.wrapping_sub(1);
                                                                fmtcnt = __v0;
                                                                if __v0 >= 0 {
                                                                    if (*fmt as i32) != 37 {
                                                                        pos = ((memchr((fmt.wrapping_offset(1 as isize) as *const ()) as *const core::ffi::c_void, 37i32 as i32, (fmtcnt as u64) as usize) as *mut ()) as *const ()) as *mut i8;
                                                                        if !pos.is_null() {
                                                                            len = pos.offset_from(
                                                                                fmt as *mut i8,
                                                                            )
                                                                                as i64;
                                                                        } else {
                                                                            len = fmtcnt
                                                                                .wrapping_add(1);
                                                                        }
                                                                        core::ptr::copy_nonoverlapping(((fmt as *const ()) as *const core::ffi::c_void) as *const u8, ((res as *mut ()) as *mut core::ffi::c_void) as *mut u8, ((len as u64) as usize) as usize);
                                                                        let __v7: *mut i8 = res;
                                                                        res = __v7.wrapping_offset(
                                                                            len as isize,
                                                                        );
                                                                        let __v5: *const i8 = fmt;
                                                                        fmt = __v5.wrapping_offset(
                                                                            len as isize,
                                                                        );
                                                                        fmtcnt = fmtcnt
                                                                            .wrapping_sub(
                                                                                len.wrapping_sub(1),
                                                                            );
                                                                    } else {
                                                                        flags = 0;
                                                                        width = 1i32.wrapping_neg()
                                                                            as i64;
                                                                        prec = 1i32.wrapping_neg();
                                                                        c = 0;
                                                                        v = core::ptr::null_mut::<
                                                                            _object,
                                                                        >(
                                                                        );
                                                                        temp = core::ptr::null_mut::<
                                                                            _object,
                                                                        >(
                                                                        );
                                                                        pbuf =
                                                                            core::ptr::null::<i8>();
                                                                        len_1 = 0;
                                                                        let __v123: *const i8 = fmt;
                                                                        fmt = __v123
                                                                            .wrapping_offset(
                                                                                1 as isize,
                                                                            );
                                                                        if (*fmt as i32) == 37 {
                                                                            let __v121: *mut i8 =
                                                                                res;
                                                                            res = __v121
                                                                                .wrapping_offset(
                                                                                    1 as isize,
                                                                                );
                                                                            *__v121 = 37;
                                                                            let __v119: *const i8 =
                                                                                fmt;
                                                                            fmt = __v119
                                                                                .wrapping_offset(
                                                                                    1 as isize,
                                                                                );
                                                                            fmtcnt = fmtcnt
                                                                                .wrapping_sub(1);
                                                                            continue '__loop_8;
                                                                        } else {
                                                                            if (*fmt as i32) == 40 {
                                                                                pcount = 1;
                                                                                if dict.is_null() {
                                                                                    break '__join_153;
                                                                                } else {
                                                                                    let __v111: *const i8 = fmt;
                                                                                    fmt = __v111.wrapping_offset(1 as isize);
                                                                                    fmtcnt = fmtcnt.wrapping_sub(1);
                                                                                    keystart = fmt;
                                                                                    // Skip over balanced parentheses
                                                                                    loop {
                                                                                        if pcount
                                                                                            > 0
                                                                                        {
                                                                                            let __v99: i64 = fmtcnt.wrapping_sub(1);
                                                                                            fmtcnt = __v99;
                                                                                            __v101 = __v99 >= 0;
                                                                                        } else {
                                                                                            __v101 = false;
                                                                                        }
                                                                                        if __v101 {
                                                                                            if (*fmt as i32) == 41 {
pcount = pcount.wrapping_sub(1);
} else {
if (*fmt as i32) == 40 {
pcount = pcount.wrapping_add(1);
}
}
                                                                                            let __v103: *const i8 = fmt;
                                                                                            fmt = __v103.wrapping_offset(1 as isize);
                                                                                        } else {
                                                                                            break;
                                                                                        }
                                                                                    }
                                                                                    keylen = (fmt.offset_from(keystart as *const i8) as i64).wrapping_sub(1);
                                                                                    if fmtcnt < 0
                                                                                        || pcount
                                                                                            > 0
                                                                                    {
                                                                                        break '__join_141;
                                                                                    } else {
                                                                                        key = PyBytes_FromStringAndSize(keystart, keylen);
                                                                                        if key.is_null() {
break '__join_2;
} else {
if args_owned != 0 {
Py_DECREF!(args);
args_owned = 0;
}
args = PyObject_GetItem(dict, key);
Py_DECREF!(key);
if args.is_null() {
break '__join_2;
} else {
args_owned = 1;
arglen = 1i32.wrapping_neg() as i64;
argidx = 2i32.wrapping_neg() as i64;
}
}
                                                                                    }
                                                                                }
                                                                            }
                                                                            // Parse flags. Example: "%+i" => flags=F_SIGN.
                                                                            loop {
                                                                                let __v84: i64 = fmtcnt.wrapping_sub(1);
                                                                                fmtcnt = __v84;
                                                                                if __v84 >= 0 {
                                                                                    let __v98: *const i8 = fmt;
                                                                                    fmt = __v98.wrapping_offset(1 as isize);
                                                                                    let __v96: i32 =
                                                                                        *__v98
                                                                                            as i32;
                                                                                    c = __v96;
                                                                                    let __t2: i32 =
                                                                                        __v96;
                                                                                    if __t2 == 45 {
                                                                                        flags |=
                                                                                            1i32;
                                                                                    } else {
                                                                                        if __t2
                                                                                            == 43
                                                                                        {
                                                                                            flags |= 2i32;
                                                                                        } else {
                                                                                            if __t2 == 32 {
flags |= 4i32;
} else {
if __t2 == 35 {
flags |= 8i32;
} else {
if __t2 == 48 {
flags |= 16i32;
} else {
break;
}
}
}
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    break;
                                                                                }
                                                                            }
                                                                            // Parse width. Example: "%10s" => width=10
                                                                            if c == 42 {
                                                                                v = getnextarg(
                                                                                    args,
                                                                                    arglen,
                                                                                    &mut (*core::ptr::addr_of_mut!(
                                                                                        argidx
                                                                                    )),
                                                                                );
                                                                                if v.is_null() {
                                                                                    break '__join_2;
                                                                                } else {
                                                                                    if PyLong_Check!(
                                                                                        v
                                                                                    ) == 0
                                                                                    {
                                                                                        break '__join_117;
                                                                                    } else {
                                                                                        width = PyLong_AsSsize_t(v);
                                                                                        if width == (1i32.wrapping_neg() as i64) {
__v79 = !PyErr_Occurred().is_null();
} else {
__v79 = false;
}
                                                                                        if __v79 {
                                                                                            break '__join_2;
                                                                                        } else {
                                                                                            if width
                                                                                                < 0
                                                                                            {
                                                                                                flags |= 1i32;
                                                                                                width = width.wrapping_neg();
                                                                                            }
                                                                                            let __v75: i64 = fmtcnt.wrapping_sub(1);
                                                                                            fmtcnt = __v75;
                                                                                            if __v75
                                                                                                >= 0
                                                                                            {
                                                                                                let __v74: *const i8 = fmt;
                                                                                                fmt = __v74.wrapping_offset(1 as isize);
                                                                                                c = *__v74 as i32;
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                if c >= 0
                                                                                    && Py_ISDIGIT!(
                                                                                        c
                                                                                    ) != 0
                                                                                {
                                                                                    width = c.wrapping_sub(48) as i64;
                                                                                    loop {
                                                                                        let __v80: i64 = fmtcnt.wrapping_sub(1);
                                                                                        fmtcnt =
                                                                                            __v80;
                                                                                        if __v80
                                                                                            >= 0
                                                                                        {
                                                                                            let __v83: *const i8 = fmt;
                                                                                            fmt = __v83.wrapping_offset(1 as isize);
                                                                                            c = (((*__v83 as i32) & 255) as u8) as i32;
                                                                                            if Py_ISDIGIT!(
                                                                                                c
                                                                                            )
                                                                                                == 0
                                                                                            {
                                                                                                break;
                                                                                            } else {
                                                                                                if width > 9223372036854775807i64.wrapping_sub(c.wrapping_sub(48) as i64) / 10 {
break '__join_122;
} else {
width = width.wrapping_mul(10).wrapping_add(c.wrapping_sub(48) as i64);
}
                                                                                            }
                                                                                        } else {
                                                                                            break;
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                            // Parse precision. Example: "%.3f" => prec=3
                                                                            if c == 46 {
                                                                                prec = 0;
                                                                                let __v71: i64 = fmtcnt.wrapping_sub(1);
                                                                                fmtcnt = __v71;
                                                                                if __v71 >= 0 {
                                                                                    let __v70: *const i8 = fmt;
                                                                                    fmt = __v70.wrapping_offset(1 as isize);
                                                                                    c = *__v70
                                                                                        as i32;
                                                                                }
                                                                                if c == 42 {
                                                                                    v = getnextarg(
                                                                                        args,
                                                                                        arglen,
                                                                                        &mut (*core::ptr::addr_of_mut!(
                                                                                            argidx
                                                                                        )),
                                                                                    );
                                                                                    if v.is_null() {
                                                                                        break '__join_2;
                                                                                    } else {
                                                                                        if PyLong_Check!(
                                                                                            v
                                                                                        ) == 0
                                                                                        {
                                                                                            break '__join_95;
                                                                                        } else {
                                                                                            prec = PyLong_AsInt(v);
                                                                                            if prec == 1i32.wrapping_neg() {
__v64 = !PyErr_Occurred().is_null();
} else {
__v64 = false;
}
                                                                                            if __v64
                                                                                            {
                                                                                                break '__join_2;
                                                                                            } else {
                                                                                                if prec < 0 {
prec = 0;
}
                                                                                                let __v62: i64 = fmtcnt.wrapping_sub(1);
                                                                                                fmtcnt = __v62;
                                                                                                if __v62 >= 0 {
let __v61: *const i8 = fmt;
fmt = __v61.wrapping_offset(1 as isize);
c = *__v61 as i32;
}
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    if c >= 0
                                                                                        && Py_ISDIGIT!(
                                                                                            c
                                                                                        ) != 0
                                                                                    {
                                                                                        prec = c.wrapping_sub(48);
                                                                                        loop {
                                                                                            let __v65: i64 = fmtcnt.wrapping_sub(1);
                                                                                            fmtcnt = __v65;
                                                                                            if __v65
                                                                                                >= 0
                                                                                            {
                                                                                                let __v68: *const i8 = fmt;
                                                                                                fmt = __v68.wrapping_offset(1 as isize);
                                                                                                c = (((*__v68 as i32) & 255) as u8) as i32;
                                                                                                if Py_ISDIGIT!(c) == 0 {
break;
} else {
if prec > 2147483647i32.wrapping_sub(c.wrapping_sub(48)) / 10 {
break '__join_100;
} else {
prec = prec.wrapping_mul(10).wrapping_add(c.wrapping_sub(48));
}
}
                                                                                            } else {
                                                                                                break;
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                            // prec
                                                                            if fmtcnt >= 0 {
                                                                                if c == 104
                                                                                    || c == 108
                                                                                    || c == 76
                                                                                {
                                                                                    let __v58: i64 = fmtcnt.wrapping_sub(1);
                                                                                    fmtcnt = __v58;
                                                                                    if __v58 >= 0 {
                                                                                        let __v57: *const i8 = fmt;
                                                                                        fmt = __v57.wrapping_offset(1 as isize);
                                                                                        c = *__v57
                                                                                            as i32;
                                                                                    }
                                                                                }
                                                                            }
                                                                            if fmtcnt < 0 {
                                                                                break '__join_81;
                                                                            } else {
                                                                                v = getnextarg(
                                                                                    args,
                                                                                    arglen,
                                                                                    &mut (*core::ptr::addr_of_mut!(
                                                                                        argidx
                                                                                    )),
                                                                                );
                                                                                if v.is_null() {
                                                                                    break '__join_2;
                                                                                } else {
                                                                                    if fmtcnt == 0 {
                                                                                        // last write: disable writer overallocation
                                                                                        (*__slate_slot_141).overallocate = 0;
                                                                                    }
                                                                                    '__join_51: {
                                                                                        '__join_76: {
                                                                                            '__join_73: {
                                                                                                '__join_70: {
                                                                                                    sign = 0;
                                                                                                    fill = 32;
                                                                                                    let __t1: i32 = c;
                                                                                                    if __t1 == 114 {
break '__join_76;
} else {
if __t1 == 97 {
break '__join_76;
} else {
if __t1 == 115 {
break '__join_73;
} else {
if __t1 == 98 {
break '__join_73;
} else {
if __t1 == 105 {
break '__join_70;
} else {
if __t1 == 100 {
break '__join_70;
} else {
if __t1 == 117 {
break '__join_70;
} else {
if __t1 == 111 {
break '__join_70;
} else {
if __t1 == 120 {
break '__join_70;
} else {
if __t1 == 88 {
break '__join_70;
} else {
if __t1 != 101 {
if __t1 != 69 {
if __t1 != 102 {
if __t1 != 70 {
if __t1 != 103 {
if __t1 != 71 {
if __t1 == 99 {
pbuf = core::ptr::addr_of_mut!(onechar) as *const i8;
len_1 = byte_converter(v, &mut (*core::ptr::addr_of_mut!(onechar))) as i64;
if len_1 == 0 {
break '__join_2;
} else {
if width == (1i32.wrapping_neg() as i64) {
// Fast path
let __v49: *mut i8 = res;
res = __v49.wrapping_offset(1 as isize);
*__v49 = onechar;
continue '__loop_8;
} else {
break '__join_51;
}
}
} else {
break '__join_52;
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
                                                                                                    if width == (1i32.wrapping_neg() as i64) && prec == 1i32.wrapping_neg() && flags & 6i32 == 0 {
// Fast path
let __v51: i64 = (*__slate_slot_141).min_size;
(*__slate_slot_141).min_size = __v51.wrapping_sub(2); // size preallocated for "%f"
res = formatfloat(v, flags, prec, c, core::ptr::null_mut::<*mut _object>(), core::ptr::addr_of_mut!(*__slate_slot_141), res);
if res.is_null() {
break '__join_2;
} else {
continue '__loop_8;
}
} else {
if formatfloat(v, flags, prec, c, core::ptr::addr_of_mut!(temp), core::ptr::null_mut::<__SlateRecord_pycore_bytesobject_h_89>(), res).is_null() {
break '__join_2;
} else {
pbuf = PyBytes_AS_STRING!(temp) as *const i8;
len_1 = PyBytes_GET_SIZE!(temp);
sign = 1;
if flags & 16i32 != 0 {
fill = 48;
break '__join_51;
} else {
break '__join_51;
}
}
}
                                                                                                }
                                                                                                if PyLong_CheckExact!(v) != 0 && width == (1i32.wrapping_neg() as i64) && prec == 1i32.wrapping_neg() && flags & 6i32 == 0 && c != 88 {
'__join_64: {
'__join_67: {
alternate = flags & 8i32;
let __t0: i32 = c;
if __t0 == 100 {
break '__join_67;
} else {
if __t0 == 105 {
break '__join_67;
} else {
if __t0 == 117 {
break '__join_67;
} else {
if __t0 == 111 {
base = 8;
break '__join_64;
} else {
if __t0 != 120 {
if __t0 != 88 {
Py_UNREACHABLE!();
break '__join_67;
}
}
}
}
}
}
base = 16;
break '__join_64;
}
base = 10;
}
// Fast path
let __v53: i64 = (*__slate_slot_141).min_size;
(*__slate_slot_141).min_size = __v53.wrapping_sub(2); // size preallocated for "%d"
res = _PyLong_FormatBytesWriter(core::ptr::addr_of_mut!(*__slate_slot_141), res, v, base, alternate);
if res.is_null() {
break '__join_2;
} else {
continue '__loop_8;
}
} else {
temp = formatlong(v, flags, prec, c);
if temp.is_null() {
break '__join_2;
} else {
pbuf = PyUnicode_1BYTE_DATA!(temp) as *const i8;
len_1 = PyUnicode_GET_LENGTH!(temp);
sign = 1;
if flags & 16i32 != 0 {
fill = 48;
break '__join_51;
} else {
break '__join_51;
}
}
}
                                                                                            }
                                                                                            temp = format_obj(v, &mut (*core::ptr::addr_of_mut!(pbuf)), &mut (*core::ptr::addr_of_mut!(len_1))); // %s is only for 2/3 code; 3 only code should use %b
                                                                                            if temp.is_null() {
break '__join_2;
} else {
if prec >= 0 && len_1 > (prec as i64) {
len_1 = prec as i64;
break '__join_51;
} else {
break '__join_51;
}
}
                                                                                        }
                                                                                        temp = PyObject_ASCII(v); // %r is only for 2/3 code; 3 only code should use %a
                                                                                        if temp.is_null() {
break '__join_2;
} else {
pbuf = PyUnicode_1BYTE_DATA!(temp) as *const i8;
len_1 = PyUnicode_GET_LENGTH!(temp);
if prec >= 0 && len_1 > (prec as i64) {
len_1 = prec as i64;
}
}
                                                                                    }
                                                                                    if sign != 0 {
                                                                                        if (*pbuf as i32) == 45 || (*pbuf as i32) == 43 {
let __v47: *const i8 = pbuf;
pbuf = __v47.wrapping_offset(1 as isize);
sign = *__v47 as i32;
len_1 = len_1.wrapping_sub(1);
} else {
if flags & 2i32 != 0 {
sign = 43;
} else {
if flags & 4i32 != 0 {
sign = 32;
} else {
sign = 0;
}
}
}
                                                                                    }
                                                                                    if width < len_1
                                                                                    {
                                                                                        width =
                                                                                            len_1;
                                                                                    }
                                                                                    alloc = width;
                                                                                    if sign != 0
                                                                                        && len_1
                                                                                            == width
                                                                                    {
                                                                                        alloc = alloc.wrapping_add(1);
                                                                                    }
                                                                                    // 2: size preallocated for %s
                                                                                    if alloc > 2 {
                                                                                        res = _PyBytesWriter_Prepare(core::ptr::addr_of_mut!(*__slate_slot_141), res as *mut (), alloc.wrapping_sub(2)) as *mut i8;
                                                                                        if res.is_null() {
break '__join_37;
}
                                                                                    }
                                                                                    // Write the sign if needed
                                                                                    if sign != 0 {
                                                                                        if fill
                                                                                            != 32
                                                                                        {
                                                                                            let __v41: *mut i8 = res;
                                                                                            res = __v41.wrapping_offset(1 as isize);
                                                                                            *__v41 = sign as i8;
                                                                                        }
                                                                                        if width
                                                                                            > len_1
                                                                                        {
                                                                                            width = width.wrapping_sub(1);
                                                                                        }
                                                                                    }
                                                                                    // Write the numeric prefix for "x", "X" and "o" formats
                                                                                    // if the alternate form is used.
                                                                                    // For example, write "0x" for the "%#x" format.
                                                                                    if flags & 8i32 != 0 && (c == 111 || c == 120 || c == 88) {
if fill != 32 {
let __v37: *const i8 = pbuf;
pbuf = __v37.wrapping_offset(1 as isize);
let __v35: *mut i8 = res;
res = __v35.wrapping_offset(1 as isize);
*__v35 = *__v37;
let __v33: *const i8 = pbuf;
pbuf = __v33.wrapping_offset(1 as isize);
let __v31: *mut i8 = res;
res = __v31.wrapping_offset(1 as isize);
*__v31 = *__v33;
}
width = width.wrapping_sub(2);
if width < 0 {
width = 0;
}
len_1 = len_1.wrapping_sub(2);
}
                                                                                    // Pad left with the fill character if needed
                                                                                    if width > len_1
                                                                                        && flags
                                                                                            & 1i32
                                                                                            == 0
                                                                                    {
                                                                                        core::ptr::write_bytes(((res as *mut ()) as *mut core::ffi::c_void) as *mut u8, (fill as i32) as u8, ((width.wrapping_sub(len_1) as u64) as usize) as usize);
                                                                                        let __v25: *mut i8 = res;
                                                                                        res = __v25.wrapping_offset(width.wrapping_sub(len_1) as isize);
                                                                                        width =
                                                                                            len_1;
                                                                                    }
                                                                                    // If padding with spaces: write sign if needed and/or numeric
                                                                                    // prefix if the alternate form is used
                                                                                    if fill == 32 {
                                                                                        if sign != 0
                                                                                        {
                                                                                            let __v23: *mut i8 = res;
                                                                                            res = __v23.wrapping_offset(1 as isize);
                                                                                            *__v23 = sign as i8;
                                                                                        }
                                                                                        if flags & 8i32 != 0 && (c == 111 || c == 120 || c == 88) {
let __v21: *const i8 = pbuf;
pbuf = __v21.wrapping_offset(1 as isize);
let __v19: *mut i8 = res;
res = __v19.wrapping_offset(1 as isize);
*__v19 = *__v21;
let __v17: *const i8 = pbuf;
pbuf = __v17.wrapping_offset(1 as isize);
let __v15: *mut i8 = res;
res = __v15.wrapping_offset(1 as isize);
*__v15 = *__v17;
}
                                                                                    }
                                                                                    // Copy bytes
                                                                                    core::ptr::copy_nonoverlapping(((pbuf as *const ()) as *const core::ffi::c_void) as *const u8, ((res as *mut ()) as *mut core::ffi::c_void) as *mut u8, ((len_1 as u64) as usize) as usize);
                                                                                    let __v13: *mut i8 = res;
                                                                                    res = __v13.wrapping_offset(len_1 as isize);
                                                                                    // Pad right with the fill character if needed
                                                                                    if width > len_1
                                                                                    {
                                                                                        core::ptr::write_bytes(((res as *mut ()) as *mut core::ffi::c_void) as *mut u8, (32i32 as i32) as u8, ((width.wrapping_sub(len_1) as u64) as usize) as usize);
                                                                                        let __v11: *mut i8 = res;
                                                                                        res = __v11.wrapping_offset(width.wrapping_sub(len_1) as isize);
                                                                                    }
                                                                                    if !dict
                                                                                        .is_null()
                                                                                        && argidx
                                                                                            < arglen
                                                                                    {
                                                                                        break '__join_15;
                                                                                    } else {
                                                                                        Py_XDECREF!(
                                                                                            temp
                                                                                        );
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                // '%'
                                                                // If overallocation was disabled, ensure that it was the last
                                                                // write. Otherwise, we missed an optimization
                                                                } else {
                                                                    break;
                                                                }
                                                            }
                                                            // until end
                                                            if argidx < arglen && dict.is_null() {
                                                                PyErr_SetString(PyExc_TypeError, c"not all arguments converted during bytes formatting".as_ptr());
                                                                break '__join_2;
                                                            } else {
                                                                if args_owned != 0 {
                                                                    Py_DECREF!(args);
                                                                }
                                                                return _PyBytesWriter_Finish(
                                                                    core::ptr::addr_of_mut!(
                                                                        *__slate_slot_141
                                                                    ),
                                                                    res as *mut (),
                                                                );
                                                            }
                                                        }
                                                        PyErr_SetString(
                                                            PyExc_ValueError,
                                                            c"width too big".as_ptr(),
                                                        );
                                                        break '__join_2;
                                                    }
                                                    PyErr_SetString(
                                                        PyExc_ValueError,
                                                        c"prec too big".as_ptr(),
                                                    );
                                                    break '__join_2;
                                                }
                                                PyErr_Format(PyExc_ValueError, c"unsupported format character '%c' (0x%x) at index %zd".as_ptr(), c, c, fmt.wrapping_offset(-(1 as isize)).offset_from(format as *const i8) as i64);
                                                break '__join_2;
                                            }
                                            PyErr_SetString(PyExc_TypeError, c"not all arguments converted during bytes formatting".as_ptr());
                                            Py_XDECREF!(temp);
                                            break '__join_2;
                                        }
                                        Py_XDECREF!(temp);
                                        break '__join_2;
                                    }
                                    PyErr_SetString(
                                        PyExc_ValueError,
                                        c"incomplete format".as_ptr(),
                                    );
                                    break '__join_2;
                                }
                                PyErr_SetString(PyExc_TypeError, c"* wants int".as_ptr());
                                break '__join_2;
                            }
                            PyErr_SetString(PyExc_TypeError, c"* wants int".as_ptr());
                            break '__join_2;
                        }
                        PyErr_SetString(PyExc_ValueError, c"incomplete format key".as_ptr());
                        break '__join_2;
                    }
                    PyErr_SetString(PyExc_TypeError, c"format requires a mapping".as_ptr());
                }
                _PyBytesWriter_Dealloc(core::ptr::addr_of_mut!(*__slate_slot_141));
                if args_owned != 0 {
                    Py_DECREF!(args);
                }
                return core::ptr::null_mut::<_object>();
            }
        }
    }
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
struct __SlateRecord_pycore_pythread_h_64 {
    ptr: *mut __SlateRecord_pthreadtypes_h_45,
    val: __SlateRecord_pthreadtypes_h_45,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_pycore_structs_h_38 {
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
struct __SlateRecord_pycore_interp_structs_h_348 {
    import_level: i32,
    accumulated: i64,
    header: i32,
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
struct __SlateRecord_pycore_interp_structs_h_718 {
    _not_used: i32,
    _hamt_empty_gc_not_used: __SlateRecord_pycore_interp_structs_h_170,
    hamt_empty: __SlateRecord_pycore_structs_h_47,
    last_resort_memory_error: __SlateRecord_pyerrors_h_15,
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
union __SlateRecord_pycore_obmalloc_h_255 {
    _padding: *mut u8,
    count: u32,
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
struct __SlateRecord_pycore_runtime_structs_h_49 {
    fd: i32,
    st_dev: u64,
    st_ino: u64,
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
struct __SlateRecord_pycore_tracemalloc_h_70 {
    mem: __SlateRecord_pymem_h_48,
    raw: __SlateRecord_pymem_h_48,
    obj: __SlateRecord_pymem_h_48,
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
struct _Py_unicode_runtime_ids {
    mutex: PyMutex,
    next_index: i64,
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
struct arena_map_bot {
    arenas: [__SlateRecord_pycore_obmalloc_h_622; 16384],
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
struct __SlateRecord_bytesobject_c_h_62 {
    _this_is_not_used: __SlateRecord_pycore_interp_structs_h_170,
    ob_base: __SlateRecord_object_h_172,
    ob_hash: i64,
    ob_item: [*mut _object; 2],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_bytesobject_c_h_233 {
    _this_is_not_used: __SlateRecord_pycore_interp_structs_h_170,
    ob_base: __SlateRecord_object_h_172,
    ob_hash: i64,
    ob_item: [*mut _object; 2],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_bytesobject_c_h_706 {
    _this_is_not_used: __SlateRecord_pycore_interp_structs_h_170,
    ob_base: __SlateRecord_object_h_172,
    ob_hash: i64,
    ob_item: [*mut _object; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_bytesobject_c_h_1071 {
    _this_is_not_used: __SlateRecord_pycore_interp_structs_h_170,
    ob_base: __SlateRecord_object_h_172,
    ob_hash: i64,
    ob_item: [*mut _object; 2],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_bytesobject_c_h_1168 {
    _this_is_not_used: __SlateRecord_pycore_interp_structs_h_170,
    ob_base: __SlateRecord_object_h_172,
    ob_hash: i64,
    ob_item: [*mut _object; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_bytesobject_c_h_1276 {
    _this_is_not_used: __SlateRecord_pycore_interp_structs_h_170,
    ob_base: __SlateRecord_object_h_172,
    ob_hash: i64,
    ob_item: [*mut _object; 2],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_bytesobject_c_h_1341 {
    _this_is_not_used: __SlateRecord_pycore_interp_structs_h_170,
    ob_base: __SlateRecord_object_h_172,
    ob_hash: i64,
    ob_item: [*mut _object; 3],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct arena_map_mid {
    ptrs: [*mut arena_map_bot; 32768],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_transmogrify_h_h_38 {
    _this_is_not_used: __SlateRecord_pycore_interp_structs_h_170,
    ob_base: __SlateRecord_object_h_172,
    ob_hash: i64,
    ob_item: [*mut _object; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct arena_map_top {
    ptrs: [*mut arena_map_mid; 32768],
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
    pub struct __SlateBits146U0 {
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
    pub struct __SlateBits201U0 {
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

fn _PyLong_FromUnsignedChar(i: u8) -> *mut _object {
    unsafe {
        core::ptr::addr_of_mut!(_PyRuntime.static_objects.singletons.small_ints) as *mut _longobject
    }
    .wrapping_offset(5i32.wrapping_add(i as i32) as isize) as *mut _object
}

// Long value tag bits:
// 0-1: Sign bits value = (1-sign), ie. negative=2, positive=0, zero=1.
// 2: Set to 1 for the small ints
// 3+ Unsigned digit count
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

fn _PyObject_IsUniquelyReferenced(ob: *mut _object) -> i32 {
    (Py_REFCNT!(ob) == 1) as i32
}

fn _PyObject_Init(op: *mut _object, typeobj: *mut _typeobject) {
    Py_SET_TYPE!(op, typeobj);
    Py_INCREF(_PyObject_CAST__2!(typeobj));
    unsafe { _Py_NewReference(op) };
}

fn _PyObject_InitVar(op: *mut __SlateRecord_object_h_172, typeobj: *mut _typeobject, size: i64) {
    _PyObject_Init(op as *mut _object, typeobj);
    Py_SET_SIZE__3!(op, size);
}

// Fast inlined version of PyType_IS_GC()
static bytes___bytes____doc__: __SlateAlign16<[i8; 64]> = __SlateAlign16([
    95i8, 95i8, 98i8, 121i8, 116i8, 101i8, 115i8, 95i8, 95i8, 40i8, 36i8, 115i8, 101i8, 108i8,
    102i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 67i8, 111i8, 110i8, 118i8, 101i8,
    114i8, 116i8, 32i8, 116i8, 104i8, 105i8, 115i8, 32i8, 118i8, 97i8, 108i8, 117i8, 101i8, 32i8,
    116i8, 111i8, 32i8, 101i8, 120i8, 97i8, 99i8, 116i8, 32i8, 116i8, 121i8, 112i8, 101i8, 32i8,
    98i8, 121i8, 116i8, 101i8, 115i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bytesobject.bytes___bytes__")]
extern "C-unwind" fn bytes___bytes__(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    bytes___bytes___impl(self_ as *mut __SlateRecord_bytesobject_h_15)
}

static bytes_split__doc__: __SlateAlign16<[i8; 397]> = __SlateAlign16([
    115i8, 112i8, 108i8, 105i8, 116i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8, 47i8,
    44i8, 32i8, 115i8, 101i8, 112i8, 61i8, 78i8, 111i8, 110i8, 101i8, 44i8, 32i8, 109i8, 97i8,
    120i8, 115i8, 112i8, 108i8, 105i8, 116i8, 61i8, 45i8, 49i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8,
    82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 97i8, 32i8, 108i8, 105i8, 115i8, 116i8, 32i8,
    111i8, 102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 115i8, 101i8, 99i8, 116i8, 105i8, 111i8, 110i8,
    115i8, 32i8, 105i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 98i8, 121i8, 116i8, 101i8, 115i8,
    44i8, 32i8, 117i8, 115i8, 105i8, 110i8, 103i8, 32i8, 115i8, 101i8, 112i8, 32i8, 97i8, 115i8,
    32i8, 116i8, 104i8, 101i8, 32i8, 100i8, 101i8, 108i8, 105i8, 109i8, 105i8, 116i8, 101i8, 114i8,
    46i8, 10i8, 10i8, 32i8, 32i8, 115i8, 101i8, 112i8, 10i8, 32i8, 32i8, 32i8, 32i8, 84i8, 104i8,
    101i8, 32i8, 100i8, 101i8, 108i8, 105i8, 109i8, 105i8, 116i8, 101i8, 114i8, 32i8, 97i8, 99i8,
    99i8, 111i8, 114i8, 100i8, 105i8, 110i8, 103i8, 32i8, 119i8, 104i8, 105i8, 99i8, 104i8, 32i8,
    116i8, 111i8, 32i8, 115i8, 112i8, 108i8, 105i8, 116i8, 32i8, 116i8, 104i8, 101i8, 32i8, 98i8,
    121i8, 116i8, 101i8, 115i8, 46i8, 10i8, 32i8, 32i8, 32i8, 32i8, 78i8, 111i8, 110i8, 101i8,
    32i8, 40i8, 116i8, 104i8, 101i8, 32i8, 100i8, 101i8, 102i8, 97i8, 117i8, 108i8, 116i8, 32i8,
    118i8, 97i8, 108i8, 117i8, 101i8, 41i8, 32i8, 109i8, 101i8, 97i8, 110i8, 115i8, 32i8, 115i8,
    112i8, 108i8, 105i8, 116i8, 32i8, 111i8, 110i8, 32i8, 65i8, 83i8, 67i8, 73i8, 73i8, 32i8,
    119i8, 104i8, 105i8, 116i8, 101i8, 115i8, 112i8, 97i8, 99i8, 101i8, 10i8, 32i8, 32i8, 32i8,
    32i8, 99i8, 104i8, 97i8, 114i8, 97i8, 99i8, 116i8, 101i8, 114i8, 115i8, 32i8, 40i8, 115i8,
    112i8, 97i8, 99i8, 101i8, 44i8, 32i8, 116i8, 97i8, 98i8, 44i8, 32i8, 114i8, 101i8, 116i8,
    117i8, 114i8, 110i8, 44i8, 32i8, 110i8, 101i8, 119i8, 108i8, 105i8, 110i8, 101i8, 44i8, 32i8,
    102i8, 111i8, 114i8, 109i8, 102i8, 101i8, 101i8, 100i8, 44i8, 32i8, 118i8, 101i8, 114i8, 116i8,
    105i8, 99i8, 97i8, 108i8, 32i8, 116i8, 97i8, 98i8, 41i8, 46i8, 10i8, 32i8, 32i8, 109i8, 97i8,
    120i8, 115i8, 112i8, 108i8, 105i8, 116i8, 10i8, 32i8, 32i8, 32i8, 32i8, 77i8, 97i8, 120i8,
    105i8, 109i8, 117i8, 109i8, 32i8, 110i8, 117i8, 109i8, 98i8, 101i8, 114i8, 32i8, 111i8, 102i8,
    32i8, 115i8, 112i8, 108i8, 105i8, 116i8, 115i8, 32i8, 116i8, 111i8, 32i8, 100i8, 111i8, 46i8,
    10i8, 32i8, 32i8, 32i8, 32i8, 45i8, 49i8, 32i8, 40i8, 116i8, 104i8, 101i8, 32i8, 100i8, 101i8,
    102i8, 97i8, 117i8, 108i8, 116i8, 32i8, 118i8, 97i8, 108i8, 117i8, 101i8, 41i8, 32i8, 109i8,
    101i8, 97i8, 110i8, 115i8, 32i8, 110i8, 111i8, 32i8, 108i8, 105i8, 109i8, 105i8, 116i8, 46i8,
    0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bytesobject.bytes_split")]
extern "C-unwind" fn bytes_split(
    self_: *mut _object,
    mut args: *const *mut _object,
    nargs: i64,
    kwnames: *mut _object,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut iobj: *mut _object = unsafe { core::mem::zeroed() };
    let mut ival: i64 = unsafe { core::mem::zeroed() };
    let mut maxsplit: i64 = unsafe { core::mem::zeroed() };
    let mut sep: *mut _object = unsafe { core::mem::zeroed() };
    let mut __v7: i64 = unsafe { core::mem::zeroed() };
    let mut noptargs: i64 = unsafe { core::mem::zeroed() };
    let mut argsbuf: core::mem::MaybeUninit<__SlateAlign16<[*mut _object; 2]>> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_9: *mut [*mut _object; 2] =
        core::ptr::addr_of_mut!(argsbuf) as *mut [*mut _object; 2];
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        if !kwnames.is_null() {
            __v7 = PyTuple_GET_SIZE!(kwnames);
        } else {
            __v7 = 0;
        }
        '__join_0: {
            noptargs = nargs.wrapping_add(__v7).wrapping_sub(0);
            sep = core::ptr::addr_of_mut!(_Py_NoneStruct);
            maxsplit = 1i32.wrapping_neg() as i64;
            args = _PyArg_UnpackKeywords!(
                args,
                nargs,
                core::ptr::null_mut::<()>(),
                kwnames,
                core::ptr::addr_of_mut!(_parser__75),
                0i32,
                2i32,
                0i32,
                0i32,
                (*__slate_slot_9).as_mut_ptr() as *mut *mut _object
            );
            // minpos
            // maxpos
            // minkw
            // varpos
            if !args.is_null() {
                '__join_1: {
                    if noptargs != 0 {
                        if !unsafe { *args.wrapping_offset(0 as isize) }.is_null() {
                            sep = *args.wrapping_offset(0 as isize);
                            let __v3: i64 = noptargs.wrapping_sub(1);
                            noptargs = __v3;
                            if __v3 == 0 {
                                break '__join_1;
                            }
                        }
                        ival = 1i32.wrapping_neg() as i64;
                        iobj = _PyNumber_Index(*args.wrapping_offset(1 as isize));
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
                            maxsplit = ival;
                        }
                    }
                }
                return_value =
                    bytes_split_impl(self_ as *mut __SlateRecord_bytesobject_h_15, sep, maxsplit);
            }
        }
        return return_value;
    }
}

static mut _kwtuple__62: __SlateRecord_bytesobject_c_h_62 = {
    let mut __t0: __SlateRecord_bytesobject_c_h_62 = unsafe { core::mem::zeroed() };
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
                    ._py_sep
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
                    ._py_maxsplit
                    ._ascii
                    .ob_base
            )
        },
    ];
    __t0
};

static _keywords__74: __SlateAlign16<[*const i8; 3]> = __SlateAlign16([
    (b"sep\0".as_ptr() as *mut i8) as *const i8,
    (b"maxsplit\0".as_ptr() as *mut i8) as *const i8,
    core::ptr::null::<i8>(),
]);

static mut _parser__75: _PyArg_Parser = {
    let mut __t0: _PyArg_Parser = unsafe { core::mem::zeroed() };
    __t0.keywords = unsafe { core::ptr::addr_of!(_keywords__74.0) as *const *const i8 };
    __t0.fname = (b"split\0".as_ptr() as *mut i8) as *const i8;
    __t0.kwtuple = unsafe { core::ptr::addr_of_mut!(_kwtuple__62.ob_base.ob_base) };
    __t0
};

static bytes_partition__doc__: __SlateAlign16<[i8; 396]> = __SlateAlign16([
    112i8, 97i8, 114i8, 116i8, 105i8, 116i8, 105i8, 111i8, 110i8, 40i8, 36i8, 115i8, 101i8, 108i8,
    102i8, 44i8, 32i8, 115i8, 101i8, 112i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8,
    80i8, 97i8, 114i8, 116i8, 105i8, 116i8, 105i8, 111i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8,
    98i8, 121i8, 116i8, 101i8, 115i8, 32i8, 105i8, 110i8, 116i8, 111i8, 32i8, 116i8, 104i8, 114i8,
    101i8, 101i8, 32i8, 112i8, 97i8, 114i8, 116i8, 115i8, 32i8, 117i8, 115i8, 105i8, 110i8, 103i8,
    32i8, 116i8, 104i8, 101i8, 32i8, 103i8, 105i8, 118i8, 101i8, 110i8, 32i8, 115i8, 101i8, 112i8,
    97i8, 114i8, 97i8, 116i8, 111i8, 114i8, 46i8, 10i8, 10i8, 84i8, 104i8, 105i8, 115i8, 32i8,
    119i8, 105i8, 108i8, 108i8, 32i8, 115i8, 101i8, 97i8, 114i8, 99i8, 104i8, 32i8, 102i8, 111i8,
    114i8, 32i8, 116i8, 104i8, 101i8, 32i8, 115i8, 101i8, 112i8, 97i8, 114i8, 97i8, 116i8, 111i8,
    114i8, 32i8, 115i8, 101i8, 112i8, 32i8, 105i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 98i8,
    121i8, 116i8, 101i8, 115i8, 46i8, 32i8, 32i8, 73i8, 102i8, 32i8, 116i8, 104i8, 101i8, 10i8,
    115i8, 101i8, 112i8, 97i8, 114i8, 97i8, 116i8, 111i8, 114i8, 32i8, 105i8, 115i8, 32i8, 102i8,
    111i8, 117i8, 110i8, 100i8, 44i8, 32i8, 114i8, 101i8, 116i8, 117i8, 114i8, 110i8, 115i8, 32i8,
    97i8, 32i8, 51i8, 45i8, 116i8, 117i8, 112i8, 108i8, 101i8, 32i8, 99i8, 111i8, 110i8, 116i8,
    97i8, 105i8, 110i8, 105i8, 110i8, 103i8, 32i8, 116i8, 104i8, 101i8, 32i8, 112i8, 97i8, 114i8,
    116i8, 32i8, 98i8, 101i8, 102i8, 111i8, 114i8, 101i8, 32i8, 116i8, 104i8, 101i8, 10i8, 115i8,
    101i8, 112i8, 97i8, 114i8, 97i8, 116i8, 111i8, 114i8, 44i8, 32i8, 116i8, 104i8, 101i8, 32i8,
    115i8, 101i8, 112i8, 97i8, 114i8, 97i8, 116i8, 111i8, 114i8, 32i8, 105i8, 116i8, 115i8, 101i8,
    108i8, 102i8, 44i8, 32i8, 97i8, 110i8, 100i8, 32i8, 116i8, 104i8, 101i8, 32i8, 112i8, 97i8,
    114i8, 116i8, 32i8, 97i8, 102i8, 116i8, 101i8, 114i8, 32i8, 105i8, 116i8, 46i8, 10i8, 10i8,
    73i8, 102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 115i8, 101i8, 112i8, 97i8, 114i8, 97i8, 116i8,
    111i8, 114i8, 32i8, 105i8, 115i8, 32i8, 110i8, 111i8, 116i8, 32i8, 102i8, 111i8, 117i8, 110i8,
    100i8, 44i8, 32i8, 114i8, 101i8, 116i8, 117i8, 114i8, 110i8, 115i8, 32i8, 97i8, 32i8, 51i8,
    45i8, 116i8, 117i8, 112i8, 108i8, 101i8, 32i8, 99i8, 111i8, 110i8, 116i8, 97i8, 105i8, 110i8,
    105i8, 110i8, 103i8, 32i8, 116i8, 104i8, 101i8, 10i8, 111i8, 114i8, 105i8, 103i8, 105i8, 110i8,
    97i8, 108i8, 32i8, 98i8, 121i8, 116i8, 101i8, 115i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8,
    116i8, 32i8, 97i8, 110i8, 100i8, 32i8, 116i8, 119i8, 111i8, 32i8, 101i8, 109i8, 112i8, 116i8,
    121i8, 32i8, 98i8, 121i8, 116i8, 101i8, 115i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8,
    115i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bytesobject.bytes_partition")]
extern "C-unwind" fn bytes_partition(self_: *mut _object, arg: *mut _object) -> *mut _object {
    let mut sep: core::mem::MaybeUninit<__SlateRecord_pybuffer_h_33> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_0: *mut __SlateRecord_pybuffer_h_33 =
        core::ptr::addr_of_mut!(sep) as *mut __SlateRecord_pybuffer_h_33;
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        core::ptr::write(__slate_slot_0, {
            let mut __t0: __SlateRecord_pybuffer_h_33 = core::mem::zeroed();
            __t0.buf = core::ptr::null_mut::<()>();
            __t0.obj = core::ptr::null_mut::<_object>();
            __t0
        });
        if PyObject_GetBuffer(arg, core::ptr::addr_of_mut!(*__slate_slot_0), 0i32) == 0 {
            return_value = bytes_partition_impl(
                self_ as *mut __SlateRecord_bytesobject_h_15,
                &(*core::ptr::addr_of_mut!(*__slate_slot_0)),
            );
        }
        if !(*__slate_slot_0).obj.is_null() {
            PyBuffer_Release(core::ptr::addr_of_mut!(*__slate_slot_0));
        }
        // Cleanup for sep
        return return_value;
    }
}

static bytes_rpartition__doc__: __SlateAlign16<[i8; 418]> = __SlateAlign16([
    114i8, 112i8, 97i8, 114i8, 116i8, 105i8, 116i8, 105i8, 111i8, 110i8, 40i8, 36i8, 115i8, 101i8,
    108i8, 102i8, 44i8, 32i8, 115i8, 101i8, 112i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8,
    10i8, 80i8, 97i8, 114i8, 116i8, 105i8, 116i8, 105i8, 111i8, 110i8, 32i8, 116i8, 104i8, 101i8,
    32i8, 98i8, 121i8, 116i8, 101i8, 115i8, 32i8, 105i8, 110i8, 116i8, 111i8, 32i8, 116i8, 104i8,
    114i8, 101i8, 101i8, 32i8, 112i8, 97i8, 114i8, 116i8, 115i8, 32i8, 117i8, 115i8, 105i8, 110i8,
    103i8, 32i8, 116i8, 104i8, 101i8, 32i8, 103i8, 105i8, 118i8, 101i8, 110i8, 32i8, 115i8, 101i8,
    112i8, 97i8, 114i8, 97i8, 116i8, 111i8, 114i8, 46i8, 10i8, 10i8, 84i8, 104i8, 105i8, 115i8,
    32i8, 119i8, 105i8, 108i8, 108i8, 32i8, 115i8, 101i8, 97i8, 114i8, 99i8, 104i8, 32i8, 102i8,
    111i8, 114i8, 32i8, 116i8, 104i8, 101i8, 32i8, 115i8, 101i8, 112i8, 97i8, 114i8, 97i8, 116i8,
    111i8, 114i8, 32i8, 115i8, 101i8, 112i8, 32i8, 105i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8,
    98i8, 121i8, 116i8, 101i8, 115i8, 44i8, 32i8, 115i8, 116i8, 97i8, 114i8, 116i8, 105i8, 110i8,
    103i8, 32i8, 97i8, 116i8, 32i8, 116i8, 104i8, 101i8, 10i8, 101i8, 110i8, 100i8, 46i8, 32i8,
    32i8, 73i8, 102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 115i8, 101i8, 112i8, 97i8, 114i8, 97i8,
    116i8, 111i8, 114i8, 32i8, 105i8, 115i8, 32i8, 102i8, 111i8, 117i8, 110i8, 100i8, 44i8, 32i8,
    114i8, 101i8, 116i8, 117i8, 114i8, 110i8, 115i8, 32i8, 97i8, 32i8, 51i8, 45i8, 116i8, 117i8,
    112i8, 108i8, 101i8, 32i8, 99i8, 111i8, 110i8, 116i8, 97i8, 105i8, 110i8, 105i8, 110i8, 103i8,
    32i8, 116i8, 104i8, 101i8, 10i8, 112i8, 97i8, 114i8, 116i8, 32i8, 98i8, 101i8, 102i8, 111i8,
    114i8, 101i8, 32i8, 116i8, 104i8, 101i8, 32i8, 115i8, 101i8, 112i8, 97i8, 114i8, 97i8, 116i8,
    111i8, 114i8, 44i8, 32i8, 116i8, 104i8, 101i8, 32i8, 115i8, 101i8, 112i8, 97i8, 114i8, 97i8,
    116i8, 111i8, 114i8, 32i8, 105i8, 116i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8, 97i8, 110i8,
    100i8, 32i8, 116i8, 104i8, 101i8, 32i8, 112i8, 97i8, 114i8, 116i8, 32i8, 97i8, 102i8, 116i8,
    101i8, 114i8, 10i8, 105i8, 116i8, 46i8, 10i8, 10i8, 73i8, 102i8, 32i8, 116i8, 104i8, 101i8,
    32i8, 115i8, 101i8, 112i8, 97i8, 114i8, 97i8, 116i8, 111i8, 114i8, 32i8, 105i8, 115i8, 32i8,
    110i8, 111i8, 116i8, 32i8, 102i8, 111i8, 117i8, 110i8, 100i8, 44i8, 32i8, 114i8, 101i8, 116i8,
    117i8, 114i8, 110i8, 115i8, 32i8, 97i8, 32i8, 51i8, 45i8, 116i8, 117i8, 112i8, 108i8, 101i8,
    32i8, 99i8, 111i8, 110i8, 116i8, 97i8, 105i8, 110i8, 105i8, 110i8, 103i8, 32i8, 116i8, 119i8,
    111i8, 10i8, 101i8, 109i8, 112i8, 116i8, 121i8, 32i8, 98i8, 121i8, 116i8, 101i8, 115i8, 32i8,
    111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 115i8, 32i8, 97i8, 110i8, 100i8, 32i8, 116i8, 104i8,
    101i8, 32i8, 111i8, 114i8, 105i8, 103i8, 105i8, 110i8, 97i8, 108i8, 32i8, 98i8, 121i8, 116i8,
    101i8, 115i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bytesobject.bytes_rpartition")]
extern "C-unwind" fn bytes_rpartition(self_: *mut _object, arg: *mut _object) -> *mut _object {
    let mut sep: core::mem::MaybeUninit<__SlateRecord_pybuffer_h_33> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_0: *mut __SlateRecord_pybuffer_h_33 =
        core::ptr::addr_of_mut!(sep) as *mut __SlateRecord_pybuffer_h_33;
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        core::ptr::write(__slate_slot_0, {
            let mut __t0: __SlateRecord_pybuffer_h_33 = core::mem::zeroed();
            __t0.buf = core::ptr::null_mut::<()>();
            __t0.obj = core::ptr::null_mut::<_object>();
            __t0
        });
        if PyObject_GetBuffer(arg, core::ptr::addr_of_mut!(*__slate_slot_0), 0i32) == 0 {
            return_value = bytes_rpartition_impl(
                self_ as *mut __SlateRecord_bytesobject_h_15,
                &(*core::ptr::addr_of_mut!(*__slate_slot_0)),
            );
        }
        if !(*__slate_slot_0).obj.is_null() {
            PyBuffer_Release(core::ptr::addr_of_mut!(*__slate_slot_0));
        }
        // Cleanup for sep
        return return_value;
    }
}

static bytes_rsplit__doc__: __SlateAlign16<[i8; 476]> = __SlateAlign16([
    114i8, 115i8, 112i8, 108i8, 105i8, 116i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8,
    47i8, 44i8, 32i8, 115i8, 101i8, 112i8, 61i8, 78i8, 111i8, 110i8, 101i8, 44i8, 32i8, 109i8,
    97i8, 120i8, 115i8, 112i8, 108i8, 105i8, 116i8, 61i8, 45i8, 49i8, 41i8, 10i8, 45i8, 45i8, 10i8,
    10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 97i8, 32i8, 108i8, 105i8, 115i8, 116i8,
    32i8, 111i8, 102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 115i8, 101i8, 99i8, 116i8, 105i8, 111i8,
    110i8, 115i8, 32i8, 105i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 98i8, 121i8, 116i8, 101i8,
    115i8, 44i8, 32i8, 117i8, 115i8, 105i8, 110i8, 103i8, 32i8, 115i8, 101i8, 112i8, 32i8, 97i8,
    115i8, 32i8, 116i8, 104i8, 101i8, 32i8, 100i8, 101i8, 108i8, 105i8, 109i8, 105i8, 116i8, 101i8,
    114i8, 46i8, 10i8, 10i8, 32i8, 32i8, 115i8, 101i8, 112i8, 10i8, 32i8, 32i8, 32i8, 32i8, 84i8,
    104i8, 101i8, 32i8, 100i8, 101i8, 108i8, 105i8, 109i8, 105i8, 116i8, 101i8, 114i8, 32i8, 97i8,
    99i8, 99i8, 111i8, 114i8, 100i8, 105i8, 110i8, 103i8, 32i8, 119i8, 104i8, 105i8, 99i8, 104i8,
    32i8, 116i8, 111i8, 32i8, 115i8, 112i8, 108i8, 105i8, 116i8, 32i8, 116i8, 104i8, 101i8, 32i8,
    98i8, 121i8, 116i8, 101i8, 115i8, 46i8, 10i8, 32i8, 32i8, 32i8, 32i8, 78i8, 111i8, 110i8,
    101i8, 32i8, 40i8, 116i8, 104i8, 101i8, 32i8, 100i8, 101i8, 102i8, 97i8, 117i8, 108i8, 116i8,
    32i8, 118i8, 97i8, 108i8, 117i8, 101i8, 41i8, 32i8, 109i8, 101i8, 97i8, 110i8, 115i8, 32i8,
    115i8, 112i8, 108i8, 105i8, 116i8, 32i8, 111i8, 110i8, 32i8, 65i8, 83i8, 67i8, 73i8, 73i8,
    32i8, 119i8, 104i8, 105i8, 116i8, 101i8, 115i8, 112i8, 97i8, 99i8, 101i8, 10i8, 32i8, 32i8,
    32i8, 32i8, 99i8, 104i8, 97i8, 114i8, 97i8, 99i8, 116i8, 101i8, 114i8, 115i8, 32i8, 40i8,
    115i8, 112i8, 97i8, 99i8, 101i8, 44i8, 32i8, 116i8, 97i8, 98i8, 44i8, 32i8, 114i8, 101i8,
    116i8, 117i8, 114i8, 110i8, 44i8, 32i8, 110i8, 101i8, 119i8, 108i8, 105i8, 110i8, 101i8, 44i8,
    32i8, 102i8, 111i8, 114i8, 109i8, 102i8, 101i8, 101i8, 100i8, 44i8, 32i8, 118i8, 101i8, 114i8,
    116i8, 105i8, 99i8, 97i8, 108i8, 32i8, 116i8, 97i8, 98i8, 41i8, 46i8, 10i8, 32i8, 32i8, 109i8,
    97i8, 120i8, 115i8, 112i8, 108i8, 105i8, 116i8, 10i8, 32i8, 32i8, 32i8, 32i8, 77i8, 97i8,
    120i8, 105i8, 109i8, 117i8, 109i8, 32i8, 110i8, 117i8, 109i8, 98i8, 101i8, 114i8, 32i8, 111i8,
    102i8, 32i8, 115i8, 112i8, 108i8, 105i8, 116i8, 115i8, 32i8, 116i8, 111i8, 32i8, 100i8, 111i8,
    46i8, 10i8, 32i8, 32i8, 32i8, 32i8, 45i8, 49i8, 32i8, 40i8, 116i8, 104i8, 101i8, 32i8, 100i8,
    101i8, 102i8, 97i8, 117i8, 108i8, 116i8, 32i8, 118i8, 97i8, 108i8, 117i8, 101i8, 41i8, 32i8,
    109i8, 101i8, 97i8, 110i8, 115i8, 32i8, 110i8, 111i8, 32i8, 108i8, 105i8, 109i8, 105i8, 116i8,
    46i8, 10i8, 10i8, 83i8, 112i8, 108i8, 105i8, 116i8, 116i8, 105i8, 110i8, 103i8, 32i8, 105i8,
    115i8, 32i8, 100i8, 111i8, 110i8, 101i8, 32i8, 115i8, 116i8, 97i8, 114i8, 116i8, 105i8, 110i8,
    103i8, 32i8, 97i8, 116i8, 32i8, 116i8, 104i8, 101i8, 32i8, 101i8, 110i8, 100i8, 32i8, 111i8,
    102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 98i8, 121i8, 116i8, 101i8, 115i8, 32i8, 97i8, 110i8,
    100i8, 32i8, 119i8, 111i8, 114i8, 107i8, 105i8, 110i8, 103i8, 32i8, 116i8, 111i8, 10i8, 116i8,
    104i8, 101i8, 32i8, 102i8, 114i8, 111i8, 110i8, 116i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bytesobject.bytes_rsplit")]
extern "C-unwind" fn bytes_rsplit(
    self_: *mut _object,
    mut args: *const *mut _object,
    nargs: i64,
    kwnames: *mut _object,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut iobj: *mut _object = unsafe { core::mem::zeroed() };
    let mut ival: i64 = unsafe { core::mem::zeroed() };
    let mut maxsplit: i64 = unsafe { core::mem::zeroed() };
    let mut sep: *mut _object = unsafe { core::mem::zeroed() };
    let mut __v7: i64 = unsafe { core::mem::zeroed() };
    let mut noptargs: i64 = unsafe { core::mem::zeroed() };
    let mut argsbuf: core::mem::MaybeUninit<__SlateAlign16<[*mut _object; 2]>> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_9: *mut [*mut _object; 2] =
        core::ptr::addr_of_mut!(argsbuf) as *mut [*mut _object; 2];
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        if !kwnames.is_null() {
            __v7 = PyTuple_GET_SIZE!(kwnames);
        } else {
            __v7 = 0;
        }
        '__join_0: {
            noptargs = nargs.wrapping_add(__v7).wrapping_sub(0);
            sep = core::ptr::addr_of_mut!(_Py_NoneStruct);
            maxsplit = 1i32.wrapping_neg() as i64;
            args = _PyArg_UnpackKeywords!(
                args,
                nargs,
                core::ptr::null_mut::<()>(),
                kwnames,
                core::ptr::addr_of_mut!(_parser__246),
                0i32,
                2i32,
                0i32,
                0i32,
                (*__slate_slot_9).as_mut_ptr() as *mut *mut _object
            );
            // minpos
            // maxpos
            // minkw
            // varpos
            if !args.is_null() {
                '__join_1: {
                    if noptargs != 0 {
                        if !unsafe { *args.wrapping_offset(0 as isize) }.is_null() {
                            sep = *args.wrapping_offset(0 as isize);
                            let __v3: i64 = noptargs.wrapping_sub(1);
                            noptargs = __v3;
                            if __v3 == 0 {
                                break '__join_1;
                            }
                        }
                        ival = 1i32.wrapping_neg() as i64;
                        iobj = _PyNumber_Index(*args.wrapping_offset(1 as isize));
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
                            maxsplit = ival;
                        }
                    }
                }
                return_value =
                    bytes_rsplit_impl(self_ as *mut __SlateRecord_bytesobject_h_15, sep, maxsplit);
            }
        }
        return return_value;
    }
}

static mut _kwtuple__233: __SlateRecord_bytesobject_c_h_233 = {
    let mut __t0: __SlateRecord_bytesobject_c_h_233 = unsafe { core::mem::zeroed() };
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
                    ._py_sep
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
                    ._py_maxsplit
                    ._ascii
                    .ob_base
            )
        },
    ];
    __t0
};

static _keywords__245: __SlateAlign16<[*const i8; 3]> = __SlateAlign16([
    (b"sep\0".as_ptr() as *mut i8) as *const i8,
    (b"maxsplit\0".as_ptr() as *mut i8) as *const i8,
    core::ptr::null::<i8>(),
]);

static mut _parser__246: _PyArg_Parser = {
    let mut __t0: _PyArg_Parser = unsafe { core::mem::zeroed() };
    __t0.keywords = unsafe { core::ptr::addr_of!(_keywords__245.0) as *const *const i8 };
    __t0.fname = (b"rsplit\0".as_ptr() as *mut i8) as *const i8;
    __t0.kwtuple = unsafe { core::ptr::addr_of_mut!(_kwtuple__233.ob_base.ob_base) };
    __t0
};

static bytes_join__doc__: __SlateAlign16<[i8; 253]> = __SlateAlign16([
    106i8, 111i8, 105i8, 110i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8, 105i8, 116i8,
    101i8, 114i8, 97i8, 98i8, 108i8, 101i8, 95i8, 111i8, 102i8, 95i8, 98i8, 121i8, 116i8, 101i8,
    115i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 67i8, 111i8, 110i8, 99i8, 97i8,
    116i8, 101i8, 110i8, 97i8, 116i8, 101i8, 32i8, 97i8, 110i8, 121i8, 32i8, 110i8, 117i8, 109i8,
    98i8, 101i8, 114i8, 32i8, 111i8, 102i8, 32i8, 98i8, 121i8, 116i8, 101i8, 115i8, 32i8, 111i8,
    98i8, 106i8, 101i8, 99i8, 116i8, 115i8, 46i8, 10i8, 10i8, 84i8, 104i8, 101i8, 32i8, 98i8,
    121i8, 116i8, 101i8, 115i8, 32i8, 119i8, 104i8, 111i8, 115i8, 101i8, 32i8, 109i8, 101i8, 116i8,
    104i8, 111i8, 100i8, 32i8, 105i8, 115i8, 32i8, 99i8, 97i8, 108i8, 108i8, 101i8, 100i8, 32i8,
    105i8, 115i8, 32i8, 105i8, 110i8, 115i8, 101i8, 114i8, 116i8, 101i8, 100i8, 32i8, 105i8, 110i8,
    32i8, 98i8, 101i8, 116i8, 119i8, 101i8, 101i8, 110i8, 32i8, 101i8, 97i8, 99i8, 104i8, 32i8,
    112i8, 97i8, 105i8, 114i8, 46i8, 10i8, 10i8, 84i8, 104i8, 101i8, 32i8, 114i8, 101i8, 115i8,
    117i8, 108i8, 116i8, 32i8, 105i8, 115i8, 32i8, 114i8, 101i8, 116i8, 117i8, 114i8, 110i8, 101i8,
    100i8, 32i8, 97i8, 115i8, 32i8, 97i8, 32i8, 110i8, 101i8, 119i8, 32i8, 98i8, 121i8, 116i8,
    101i8, 115i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 46i8, 10i8, 10i8, 69i8, 120i8,
    97i8, 109i8, 112i8, 108i8, 101i8, 58i8, 32i8, 98i8, 39i8, 46i8, 39i8, 46i8, 106i8, 111i8,
    105i8, 110i8, 40i8, 91i8, 98i8, 39i8, 97i8, 98i8, 39i8, 44i8, 32i8, 98i8, 39i8, 112i8, 113i8,
    39i8, 44i8, 32i8, 98i8, 39i8, 114i8, 115i8, 39i8, 93i8, 41i8, 32i8, 45i8, 62i8, 32i8, 98i8,
    39i8, 97i8, 98i8, 46i8, 112i8, 113i8, 46i8, 114i8, 115i8, 39i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bytesobject.bytes_join")]
extern "C-unwind" fn bytes_join(
    self_: *mut _object,
    iterable_of_bytes: *mut _object,
) -> *mut _object {
    let mut return_value: *mut _object = core::ptr::null_mut::<_object>();
    return_value = bytes_join_impl(
        self_ as *mut __SlateRecord_bytesobject_h_15,
        iterable_of_bytes,
    );
    return_value
}

use slate_support::memchr;
use slate_support::memcmp;
use slate_support::memcpy;
use slate_support::memmove;
use slate_support::memset;
use slate_support::sprintf;
use slate_support::strcmp;
use slate_support::strlen;

static bytes_find__doc__: __SlateAlign16<[i8; 303]> = __SlateAlign16([
    102i8, 105i8, 110i8, 100i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8, 115i8, 117i8,
    98i8, 91i8, 44i8, 32i8, 115i8, 116i8, 97i8, 114i8, 116i8, 91i8, 44i8, 32i8, 101i8, 110i8,
    100i8, 93i8, 93i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8,
    117i8, 114i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 108i8, 111i8, 119i8, 101i8, 115i8, 116i8,
    32i8, 105i8, 110i8, 100i8, 101i8, 120i8, 32i8, 105i8, 110i8, 32i8, 66i8, 32i8, 119i8, 104i8,
    101i8, 114i8, 101i8, 32i8, 115i8, 117i8, 98i8, 115i8, 101i8, 99i8, 116i8, 105i8, 111i8, 110i8,
    32i8, 39i8, 115i8, 117i8, 98i8, 39i8, 32i8, 105i8, 115i8, 32i8, 102i8, 111i8, 117i8, 110i8,
    100i8, 44i8, 32i8, 115i8, 117i8, 99i8, 104i8, 32i8, 116i8, 104i8, 97i8, 116i8, 32i8, 39i8,
    115i8, 117i8, 98i8, 39i8, 32i8, 105i8, 115i8, 32i8, 99i8, 111i8, 110i8, 116i8, 97i8, 105i8,
    110i8, 101i8, 100i8, 32i8, 119i8, 105i8, 116i8, 104i8, 105i8, 110i8, 32i8, 66i8, 91i8, 115i8,
    116i8, 97i8, 114i8, 116i8, 44i8, 101i8, 110i8, 100i8, 93i8, 46i8, 10i8, 10i8, 32i8, 32i8,
    115i8, 116i8, 97i8, 114i8, 116i8, 10i8, 32i8, 32i8, 32i8, 32i8, 79i8, 112i8, 116i8, 105i8,
    111i8, 110i8, 97i8, 108i8, 32i8, 115i8, 116i8, 97i8, 114i8, 116i8, 32i8, 112i8, 111i8, 115i8,
    105i8, 116i8, 105i8, 111i8, 110i8, 46i8, 32i8, 68i8, 101i8, 102i8, 97i8, 117i8, 108i8, 116i8,
    58i8, 32i8, 115i8, 116i8, 97i8, 114i8, 116i8, 32i8, 111i8, 102i8, 32i8, 116i8, 104i8, 101i8,
    32i8, 98i8, 121i8, 116i8, 101i8, 115i8, 46i8, 10i8, 32i8, 32i8, 101i8, 110i8, 100i8, 10i8,
    32i8, 32i8, 32i8, 32i8, 79i8, 112i8, 116i8, 105i8, 111i8, 110i8, 97i8, 108i8, 32i8, 115i8,
    116i8, 111i8, 112i8, 32i8, 112i8, 111i8, 115i8, 105i8, 116i8, 105i8, 111i8, 110i8, 46i8, 32i8,
    68i8, 101i8, 102i8, 97i8, 117i8, 108i8, 116i8, 58i8, 32i8, 101i8, 110i8, 100i8, 32i8, 111i8,
    102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 98i8, 121i8, 116i8, 101i8, 115i8, 46i8, 10i8, 10i8,
    82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 45i8, 49i8, 32i8, 111i8, 110i8, 32i8, 102i8,
    97i8, 105i8, 108i8, 117i8, 114i8, 101i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bytesobject.bytes_find")]
extern "C-unwind" fn bytes_find(
    self_: *mut _object,
    args: *const *mut _object,
    nargs: i64,
) -> *mut _object {
    let mut end: i64 = unsafe { core::mem::zeroed() };
    let mut start: i64 = unsafe { core::mem::zeroed() };
    let mut sub: *mut _object = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_0: {
            return_value = core::ptr::null_mut::<_object>();
            start = 0;
            end = 9223372036854775807;
            if _PyArg_CheckPositional!(c"find".as_ptr() as *mut i8, nargs, 1i32, 3i32) {
                sub = *args.wrapping_offset(0 as isize);
                if nargs >= 2 {
                    if _PyEval_SliceIndex(
                        *args.wrapping_offset(1 as isize),
                        core::ptr::addr_of_mut!(start),
                    ) == 0
                    {
                        break '__join_0;
                    } else {
                        if nargs >= 3 {
                            if _PyEval_SliceIndex(
                                *args.wrapping_offset(2 as isize),
                                core::ptr::addr_of_mut!(end),
                            ) == 0
                            {
                                break '__join_0;
                            }
                        }
                    }
                }
                return_value = bytes_find_impl(
                    self_ as *mut __SlateRecord_bytesobject_h_15,
                    sub,
                    start,
                    end,
                );
            }
        }
        return return_value;
    }
}

static bytes_index__doc__: __SlateAlign16<[i8; 331]> = __SlateAlign16([
    105i8, 110i8, 100i8, 101i8, 120i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8, 115i8,
    117i8, 98i8, 91i8, 44i8, 32i8, 115i8, 116i8, 97i8, 114i8, 116i8, 91i8, 44i8, 32i8, 101i8,
    110i8, 100i8, 93i8, 93i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8,
    116i8, 117i8, 114i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 108i8, 111i8, 119i8, 101i8, 115i8,
    116i8, 32i8, 105i8, 110i8, 100i8, 101i8, 120i8, 32i8, 105i8, 110i8, 32i8, 66i8, 32i8, 119i8,
    104i8, 101i8, 114i8, 101i8, 32i8, 115i8, 117i8, 98i8, 115i8, 101i8, 99i8, 116i8, 105i8, 111i8,
    110i8, 32i8, 39i8, 115i8, 117i8, 98i8, 39i8, 32i8, 105i8, 115i8, 32i8, 102i8, 111i8, 117i8,
    110i8, 100i8, 44i8, 32i8, 115i8, 117i8, 99i8, 104i8, 32i8, 116i8, 104i8, 97i8, 116i8, 32i8,
    39i8, 115i8, 117i8, 98i8, 39i8, 32i8, 105i8, 115i8, 32i8, 99i8, 111i8, 110i8, 116i8, 97i8,
    105i8, 110i8, 101i8, 100i8, 32i8, 119i8, 105i8, 116i8, 104i8, 105i8, 110i8, 32i8, 66i8, 91i8,
    115i8, 116i8, 97i8, 114i8, 116i8, 44i8, 101i8, 110i8, 100i8, 93i8, 46i8, 10i8, 10i8, 32i8,
    32i8, 115i8, 116i8, 97i8, 114i8, 116i8, 10i8, 32i8, 32i8, 32i8, 32i8, 79i8, 112i8, 116i8,
    105i8, 111i8, 110i8, 97i8, 108i8, 32i8, 115i8, 116i8, 97i8, 114i8, 116i8, 32i8, 112i8, 111i8,
    115i8, 105i8, 116i8, 105i8, 111i8, 110i8, 46i8, 32i8, 68i8, 101i8, 102i8, 97i8, 117i8, 108i8,
    116i8, 58i8, 32i8, 115i8, 116i8, 97i8, 114i8, 116i8, 32i8, 111i8, 102i8, 32i8, 116i8, 104i8,
    101i8, 32i8, 98i8, 121i8, 116i8, 101i8, 115i8, 46i8, 10i8, 32i8, 32i8, 101i8, 110i8, 100i8,
    10i8, 32i8, 32i8, 32i8, 32i8, 79i8, 112i8, 116i8, 105i8, 111i8, 110i8, 97i8, 108i8, 32i8,
    115i8, 116i8, 111i8, 112i8, 32i8, 112i8, 111i8, 115i8, 105i8, 116i8, 105i8, 111i8, 110i8, 46i8,
    32i8, 68i8, 101i8, 102i8, 97i8, 117i8, 108i8, 116i8, 58i8, 32i8, 101i8, 110i8, 100i8, 32i8,
    111i8, 102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 98i8, 121i8, 116i8, 101i8, 115i8, 46i8, 10i8,
    10i8, 82i8, 97i8, 105i8, 115i8, 101i8, 32i8, 86i8, 97i8, 108i8, 117i8, 101i8, 69i8, 114i8,
    114i8, 111i8, 114i8, 32i8, 105i8, 102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 115i8, 117i8, 98i8,
    115i8, 101i8, 99i8, 116i8, 105i8, 111i8, 110i8, 32i8, 105i8, 115i8, 32i8, 110i8, 111i8, 116i8,
    32i8, 102i8, 111i8, 117i8, 110i8, 100i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bytesobject.bytes_index")]
extern "C-unwind" fn bytes_index(
    self_: *mut _object,
    args: *const *mut _object,
    nargs: i64,
) -> *mut _object {
    let mut end: i64 = unsafe { core::mem::zeroed() };
    let mut start: i64 = unsafe { core::mem::zeroed() };
    let mut sub: *mut _object = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_0: {
            return_value = core::ptr::null_mut::<_object>();
            start = 0;
            end = 9223372036854775807;
            if _PyArg_CheckPositional!(c"index".as_ptr() as *mut i8, nargs, 1i32, 3i32) {
                sub = *args.wrapping_offset(0 as isize);
                if nargs >= 2 {
                    if _PyEval_SliceIndex(
                        *args.wrapping_offset(1 as isize),
                        core::ptr::addr_of_mut!(start),
                    ) == 0
                    {
                        break '__join_0;
                    } else {
                        if nargs >= 3 {
                            if _PyEval_SliceIndex(
                                *args.wrapping_offset(2 as isize),
                                core::ptr::addr_of_mut!(end),
                            ) == 0
                            {
                                break '__join_0;
                            }
                        }
                    }
                }
                return_value = bytes_index_impl(
                    self_ as *mut __SlateRecord_bytesobject_h_15,
                    sub,
                    start,
                    end,
                );
            }
        }
        return return_value;
    }
}

static bytes_rfind__doc__: __SlateAlign16<[i8; 305]> = __SlateAlign16([
    114i8, 102i8, 105i8, 110i8, 100i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8, 115i8,
    117i8, 98i8, 91i8, 44i8, 32i8, 115i8, 116i8, 97i8, 114i8, 116i8, 91i8, 44i8, 32i8, 101i8,
    110i8, 100i8, 93i8, 93i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8,
    116i8, 117i8, 114i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 104i8, 105i8, 103i8, 104i8, 101i8,
    115i8, 116i8, 32i8, 105i8, 110i8, 100i8, 101i8, 120i8, 32i8, 105i8, 110i8, 32i8, 66i8, 32i8,
    119i8, 104i8, 101i8, 114i8, 101i8, 32i8, 115i8, 117i8, 98i8, 115i8, 101i8, 99i8, 116i8, 105i8,
    111i8, 110i8, 32i8, 39i8, 115i8, 117i8, 98i8, 39i8, 32i8, 105i8, 115i8, 32i8, 102i8, 111i8,
    117i8, 110i8, 100i8, 44i8, 32i8, 115i8, 117i8, 99i8, 104i8, 32i8, 116i8, 104i8, 97i8, 116i8,
    32i8, 39i8, 115i8, 117i8, 98i8, 39i8, 32i8, 105i8, 115i8, 32i8, 99i8, 111i8, 110i8, 116i8,
    97i8, 105i8, 110i8, 101i8, 100i8, 32i8, 119i8, 105i8, 116i8, 104i8, 105i8, 110i8, 32i8, 66i8,
    91i8, 115i8, 116i8, 97i8, 114i8, 116i8, 44i8, 101i8, 110i8, 100i8, 93i8, 46i8, 10i8, 10i8,
    32i8, 32i8, 115i8, 116i8, 97i8, 114i8, 116i8, 10i8, 32i8, 32i8, 32i8, 32i8, 79i8, 112i8, 116i8,
    105i8, 111i8, 110i8, 97i8, 108i8, 32i8, 115i8, 116i8, 97i8, 114i8, 116i8, 32i8, 112i8, 111i8,
    115i8, 105i8, 116i8, 105i8, 111i8, 110i8, 46i8, 32i8, 68i8, 101i8, 102i8, 97i8, 117i8, 108i8,
    116i8, 58i8, 32i8, 115i8, 116i8, 97i8, 114i8, 116i8, 32i8, 111i8, 102i8, 32i8, 116i8, 104i8,
    101i8, 32i8, 98i8, 121i8, 116i8, 101i8, 115i8, 46i8, 10i8, 32i8, 32i8, 101i8, 110i8, 100i8,
    10i8, 32i8, 32i8, 32i8, 32i8, 79i8, 112i8, 116i8, 105i8, 111i8, 110i8, 97i8, 108i8, 32i8,
    115i8, 116i8, 111i8, 112i8, 32i8, 112i8, 111i8, 115i8, 105i8, 116i8, 105i8, 111i8, 110i8, 46i8,
    32i8, 68i8, 101i8, 102i8, 97i8, 117i8, 108i8, 116i8, 58i8, 32i8, 101i8, 110i8, 100i8, 32i8,
    111i8, 102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 98i8, 121i8, 116i8, 101i8, 115i8, 46i8, 10i8,
    10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 45i8, 49i8, 32i8, 111i8, 110i8, 32i8,
    102i8, 97i8, 105i8, 108i8, 117i8, 114i8, 101i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bytesobject.bytes_rfind")]
extern "C-unwind" fn bytes_rfind(
    self_: *mut _object,
    args: *const *mut _object,
    nargs: i64,
) -> *mut _object {
    let mut end: i64 = unsafe { core::mem::zeroed() };
    let mut start: i64 = unsafe { core::mem::zeroed() };
    let mut sub: *mut _object = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_0: {
            return_value = core::ptr::null_mut::<_object>();
            start = 0;
            end = 9223372036854775807;
            if _PyArg_CheckPositional!(c"rfind".as_ptr() as *mut i8, nargs, 1i32, 3i32) {
                sub = *args.wrapping_offset(0 as isize);
                if nargs >= 2 {
                    if _PyEval_SliceIndex(
                        *args.wrapping_offset(1 as isize),
                        core::ptr::addr_of_mut!(start),
                    ) == 0
                    {
                        break '__join_0;
                    } else {
                        if nargs >= 3 {
                            if _PyEval_SliceIndex(
                                *args.wrapping_offset(2 as isize),
                                core::ptr::addr_of_mut!(end),
                            ) == 0
                            {
                                break '__join_0;
                            }
                        }
                    }
                }
                return_value = bytes_rfind_impl(
                    self_ as *mut __SlateRecord_bytesobject_h_15,
                    sub,
                    start,
                    end,
                );
            }
        }
        return return_value;
    }
}

static bytes_rindex__doc__: __SlateAlign16<[i8; 333]> = __SlateAlign16([
    114i8, 105i8, 110i8, 100i8, 101i8, 120i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8,
    115i8, 117i8, 98i8, 91i8, 44i8, 32i8, 115i8, 116i8, 97i8, 114i8, 116i8, 91i8, 44i8, 32i8,
    101i8, 110i8, 100i8, 93i8, 93i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8,
    101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 104i8, 105i8, 103i8, 104i8,
    101i8, 115i8, 116i8, 32i8, 105i8, 110i8, 100i8, 101i8, 120i8, 32i8, 105i8, 110i8, 32i8, 66i8,
    32i8, 119i8, 104i8, 101i8, 114i8, 101i8, 32i8, 115i8, 117i8, 98i8, 115i8, 101i8, 99i8, 116i8,
    105i8, 111i8, 110i8, 32i8, 39i8, 115i8, 117i8, 98i8, 39i8, 32i8, 105i8, 115i8, 32i8, 102i8,
    111i8, 117i8, 110i8, 100i8, 44i8, 32i8, 115i8, 117i8, 99i8, 104i8, 32i8, 116i8, 104i8, 97i8,
    116i8, 32i8, 39i8, 115i8, 117i8, 98i8, 39i8, 32i8, 105i8, 115i8, 32i8, 99i8, 111i8, 110i8,
    116i8, 97i8, 105i8, 110i8, 101i8, 100i8, 32i8, 119i8, 105i8, 116i8, 104i8, 105i8, 110i8, 32i8,
    66i8, 91i8, 115i8, 116i8, 97i8, 114i8, 116i8, 44i8, 101i8, 110i8, 100i8, 93i8, 46i8, 10i8,
    10i8, 32i8, 32i8, 115i8, 116i8, 97i8, 114i8, 116i8, 10i8, 32i8, 32i8, 32i8, 32i8, 79i8, 112i8,
    116i8, 105i8, 111i8, 110i8, 97i8, 108i8, 32i8, 115i8, 116i8, 97i8, 114i8, 116i8, 32i8, 112i8,
    111i8, 115i8, 105i8, 116i8, 105i8, 111i8, 110i8, 46i8, 32i8, 68i8, 101i8, 102i8, 97i8, 117i8,
    108i8, 116i8, 58i8, 32i8, 115i8, 116i8, 97i8, 114i8, 116i8, 32i8, 111i8, 102i8, 32i8, 116i8,
    104i8, 101i8, 32i8, 98i8, 121i8, 116i8, 101i8, 115i8, 46i8, 10i8, 32i8, 32i8, 101i8, 110i8,
    100i8, 10i8, 32i8, 32i8, 32i8, 32i8, 79i8, 112i8, 116i8, 105i8, 111i8, 110i8, 97i8, 108i8,
    32i8, 115i8, 116i8, 111i8, 112i8, 32i8, 112i8, 111i8, 115i8, 105i8, 116i8, 105i8, 111i8, 110i8,
    46i8, 32i8, 68i8, 101i8, 102i8, 97i8, 117i8, 108i8, 116i8, 58i8, 32i8, 101i8, 110i8, 100i8,
    32i8, 111i8, 102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 98i8, 121i8, 116i8, 101i8, 115i8, 46i8,
    10i8, 10i8, 82i8, 97i8, 105i8, 115i8, 101i8, 32i8, 86i8, 97i8, 108i8, 117i8, 101i8, 69i8,
    114i8, 114i8, 111i8, 114i8, 32i8, 105i8, 102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 115i8, 117i8,
    98i8, 115i8, 101i8, 99i8, 116i8, 105i8, 111i8, 110i8, 32i8, 105i8, 115i8, 32i8, 110i8, 111i8,
    116i8, 32i8, 102i8, 111i8, 117i8, 110i8, 100i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bytesobject.bytes_rindex")]
extern "C-unwind" fn bytes_rindex(
    self_: *mut _object,
    args: *const *mut _object,
    nargs: i64,
) -> *mut _object {
    let mut end: i64 = unsafe { core::mem::zeroed() };
    let mut start: i64 = unsafe { core::mem::zeroed() };
    let mut sub: *mut _object = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_0: {
            return_value = core::ptr::null_mut::<_object>();
            start = 0;
            end = 9223372036854775807;
            if _PyArg_CheckPositional!(c"rindex".as_ptr() as *mut i8, nargs, 1i32, 3i32) {
                sub = *args.wrapping_offset(0 as isize);
                if nargs >= 2 {
                    if _PyEval_SliceIndex(
                        *args.wrapping_offset(1 as isize),
                        core::ptr::addr_of_mut!(start),
                    ) == 0
                    {
                        break '__join_0;
                    } else {
                        if nargs >= 3 {
                            if _PyEval_SliceIndex(
                                *args.wrapping_offset(2 as isize),
                                core::ptr::addr_of_mut!(end),
                            ) == 0
                            {
                                break '__join_0;
                            }
                        }
                    }
                }
                return_value = bytes_rindex_impl(
                    self_ as *mut __SlateRecord_bytesobject_h_15,
                    sub,
                    start,
                    end,
                );
            }
        }
        return return_value;
    }
}

static bytes_strip__doc__: __SlateAlign16<[i8; 174]> = __SlateAlign16([
    115i8, 116i8, 114i8, 105i8, 112i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8, 98i8,
    121i8, 116i8, 101i8, 115i8, 61i8, 78i8, 111i8, 110i8, 101i8, 44i8, 32i8, 47i8, 41i8, 10i8,
    45i8, 45i8, 10i8, 10i8, 83i8, 116i8, 114i8, 105i8, 112i8, 32i8, 108i8, 101i8, 97i8, 100i8,
    105i8, 110i8, 103i8, 32i8, 97i8, 110i8, 100i8, 32i8, 116i8, 114i8, 97i8, 105i8, 108i8, 105i8,
    110i8, 103i8, 32i8, 98i8, 121i8, 116i8, 101i8, 115i8, 32i8, 99i8, 111i8, 110i8, 116i8, 97i8,
    105i8, 110i8, 101i8, 100i8, 32i8, 105i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 97i8, 114i8,
    103i8, 117i8, 109i8, 101i8, 110i8, 116i8, 46i8, 10i8, 10i8, 73i8, 102i8, 32i8, 116i8, 104i8,
    101i8, 32i8, 97i8, 114i8, 103i8, 117i8, 109i8, 101i8, 110i8, 116i8, 32i8, 105i8, 115i8, 32i8,
    111i8, 109i8, 105i8, 116i8, 116i8, 101i8, 100i8, 32i8, 111i8, 114i8, 32i8, 78i8, 111i8, 110i8,
    101i8, 44i8, 32i8, 115i8, 116i8, 114i8, 105i8, 112i8, 32i8, 108i8, 101i8, 97i8, 100i8, 105i8,
    110i8, 103i8, 32i8, 97i8, 110i8, 100i8, 32i8, 116i8, 114i8, 97i8, 105i8, 108i8, 105i8, 110i8,
    103i8, 32i8, 65i8, 83i8, 67i8, 73i8, 73i8, 10i8, 119i8, 104i8, 105i8, 116i8, 101i8, 115i8,
    112i8, 97i8, 99i8, 101i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bytesobject.bytes_strip")]
extern "C-unwind" fn bytes_strip(
    self_: *mut _object,
    args: *const *mut _object,
    nargs: i64,
) -> *mut _object {
    let mut bytes: *mut _object = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        bytes = core::ptr::addr_of_mut!(_Py_NoneStruct);
        if _PyArg_CheckPositional!(c"strip".as_ptr() as *mut i8, nargs, 0i32, 1i32) {
            if nargs >= 1 {
                bytes = *args.wrapping_offset(0 as isize);
            }
            return_value = bytes_strip_impl(self_ as *mut __SlateRecord_bytesobject_h_15, bytes);
        }
        return return_value;
    }
}

static bytes_lstrip__doc__: __SlateAlign16<[i8; 150]> = __SlateAlign16([
    108i8, 115i8, 116i8, 114i8, 105i8, 112i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8,
    98i8, 121i8, 116i8, 101i8, 115i8, 61i8, 78i8, 111i8, 110i8, 101i8, 44i8, 32i8, 47i8, 41i8,
    10i8, 45i8, 45i8, 10i8, 10i8, 83i8, 116i8, 114i8, 105i8, 112i8, 32i8, 108i8, 101i8, 97i8,
    100i8, 105i8, 110i8, 103i8, 32i8, 98i8, 121i8, 116i8, 101i8, 115i8, 32i8, 99i8, 111i8, 110i8,
    116i8, 97i8, 105i8, 110i8, 101i8, 100i8, 32i8, 105i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8,
    97i8, 114i8, 103i8, 117i8, 109i8, 101i8, 110i8, 116i8, 46i8, 10i8, 10i8, 73i8, 102i8, 32i8,
    116i8, 104i8, 101i8, 32i8, 97i8, 114i8, 103i8, 117i8, 109i8, 101i8, 110i8, 116i8, 32i8, 105i8,
    115i8, 32i8, 111i8, 109i8, 105i8, 116i8, 116i8, 101i8, 100i8, 32i8, 111i8, 114i8, 32i8, 78i8,
    111i8, 110i8, 101i8, 44i8, 32i8, 115i8, 116i8, 114i8, 105i8, 112i8, 32i8, 108i8, 101i8, 97i8,
    100i8, 105i8, 110i8, 103i8, 32i8, 32i8, 65i8, 83i8, 67i8, 73i8, 73i8, 32i8, 119i8, 104i8,
    105i8, 116i8, 101i8, 115i8, 112i8, 97i8, 99i8, 101i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bytesobject.bytes_lstrip")]
extern "C-unwind" fn bytes_lstrip(
    self_: *mut _object,
    args: *const *mut _object,
    nargs: i64,
) -> *mut _object {
    let mut bytes: *mut _object = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        bytes = core::ptr::addr_of_mut!(_Py_NoneStruct);
        if _PyArg_CheckPositional!(c"lstrip".as_ptr() as *mut i8, nargs, 0i32, 1i32) {
            if nargs >= 1 {
                bytes = *args.wrapping_offset(0 as isize);
            }
            return_value = bytes_lstrip_impl(self_ as *mut __SlateRecord_bytesobject_h_15, bytes);
        }
        return return_value;
    }
}

static bytes_rstrip__doc__: __SlateAlign16<[i8; 151]> = __SlateAlign16([
    114i8, 115i8, 116i8, 114i8, 105i8, 112i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8,
    98i8, 121i8, 116i8, 101i8, 115i8, 61i8, 78i8, 111i8, 110i8, 101i8, 44i8, 32i8, 47i8, 41i8,
    10i8, 45i8, 45i8, 10i8, 10i8, 83i8, 116i8, 114i8, 105i8, 112i8, 32i8, 116i8, 114i8, 97i8,
    105i8, 108i8, 105i8, 110i8, 103i8, 32i8, 98i8, 121i8, 116i8, 101i8, 115i8, 32i8, 99i8, 111i8,
    110i8, 116i8, 97i8, 105i8, 110i8, 101i8, 100i8, 32i8, 105i8, 110i8, 32i8, 116i8, 104i8, 101i8,
    32i8, 97i8, 114i8, 103i8, 117i8, 109i8, 101i8, 110i8, 116i8, 46i8, 10i8, 10i8, 73i8, 102i8,
    32i8, 116i8, 104i8, 101i8, 32i8, 97i8, 114i8, 103i8, 117i8, 109i8, 101i8, 110i8, 116i8, 32i8,
    105i8, 115i8, 32i8, 111i8, 109i8, 105i8, 116i8, 116i8, 101i8, 100i8, 32i8, 111i8, 114i8, 32i8,
    78i8, 111i8, 110i8, 101i8, 44i8, 32i8, 115i8, 116i8, 114i8, 105i8, 112i8, 32i8, 116i8, 114i8,
    97i8, 105i8, 108i8, 105i8, 110i8, 103i8, 32i8, 65i8, 83i8, 67i8, 73i8, 73i8, 32i8, 119i8,
    104i8, 105i8, 116i8, 101i8, 115i8, 112i8, 97i8, 99i8, 101i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bytesobject.bytes_rstrip")]
extern "C-unwind" fn bytes_rstrip(
    self_: *mut _object,
    args: *const *mut _object,
    nargs: i64,
) -> *mut _object {
    let mut bytes: *mut _object = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        bytes = core::ptr::addr_of_mut!(_Py_NoneStruct);
        if _PyArg_CheckPositional!(c"rstrip".as_ptr() as *mut i8, nargs, 0i32, 1i32) {
            if nargs >= 1 {
                bytes = *args.wrapping_offset(0 as isize);
            }
            return_value = bytes_rstrip_impl(self_ as *mut __SlateRecord_bytesobject_h_15, bytes);
        }
        return return_value;
    }
}

static bytes_count__doc__: __SlateAlign16<[i8; 261]> = __SlateAlign16([
    99i8, 111i8, 117i8, 110i8, 116i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8, 115i8,
    117i8, 98i8, 91i8, 44i8, 32i8, 115i8, 116i8, 97i8, 114i8, 116i8, 91i8, 44i8, 32i8, 101i8,
    110i8, 100i8, 93i8, 93i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8,
    116i8, 117i8, 114i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 110i8, 117i8, 109i8, 98i8, 101i8,
    114i8, 32i8, 111i8, 102i8, 32i8, 110i8, 111i8, 110i8, 45i8, 111i8, 118i8, 101i8, 114i8, 108i8,
    97i8, 112i8, 112i8, 105i8, 110i8, 103i8, 32i8, 111i8, 99i8, 99i8, 117i8, 114i8, 114i8, 101i8,
    110i8, 99i8, 101i8, 115i8, 32i8, 111i8, 102i8, 32i8, 115i8, 117i8, 98i8, 115i8, 101i8, 99i8,
    116i8, 105i8, 111i8, 110i8, 32i8, 39i8, 115i8, 117i8, 98i8, 39i8, 32i8, 105i8, 110i8, 32i8,
    98i8, 121i8, 116i8, 101i8, 115i8, 32i8, 66i8, 91i8, 115i8, 116i8, 97i8, 114i8, 116i8, 58i8,
    101i8, 110i8, 100i8, 93i8, 46i8, 10i8, 10i8, 32i8, 32i8, 115i8, 116i8, 97i8, 114i8, 116i8,
    10i8, 32i8, 32i8, 32i8, 32i8, 79i8, 112i8, 116i8, 105i8, 111i8, 110i8, 97i8, 108i8, 32i8,
    115i8, 116i8, 97i8, 114i8, 116i8, 32i8, 112i8, 111i8, 115i8, 105i8, 116i8, 105i8, 111i8, 110i8,
    46i8, 32i8, 68i8, 101i8, 102i8, 97i8, 117i8, 108i8, 116i8, 58i8, 32i8, 115i8, 116i8, 97i8,
    114i8, 116i8, 32i8, 111i8, 102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 98i8, 121i8, 116i8, 101i8,
    115i8, 46i8, 10i8, 32i8, 32i8, 101i8, 110i8, 100i8, 10i8, 32i8, 32i8, 32i8, 32i8, 79i8, 112i8,
    116i8, 105i8, 111i8, 110i8, 97i8, 108i8, 32i8, 115i8, 116i8, 111i8, 112i8, 32i8, 112i8, 111i8,
    115i8, 105i8, 116i8, 105i8, 111i8, 110i8, 46i8, 32i8, 68i8, 101i8, 102i8, 97i8, 117i8, 108i8,
    116i8, 58i8, 32i8, 101i8, 110i8, 100i8, 32i8, 111i8, 102i8, 32i8, 116i8, 104i8, 101i8, 32i8,
    98i8, 121i8, 116i8, 101i8, 115i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bytesobject.bytes_count")]
extern "C-unwind" fn bytes_count(
    self_: *mut _object,
    args: *const *mut _object,
    nargs: i64,
) -> *mut _object {
    let mut end: i64 = unsafe { core::mem::zeroed() };
    let mut start: i64 = unsafe { core::mem::zeroed() };
    let mut sub: *mut _object = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_0: {
            return_value = core::ptr::null_mut::<_object>();
            start = 0;
            end = 9223372036854775807;
            if _PyArg_CheckPositional!(c"count".as_ptr() as *mut i8, nargs, 1i32, 3i32) {
                sub = *args.wrapping_offset(0 as isize);
                if nargs >= 2 {
                    if _PyEval_SliceIndex(
                        *args.wrapping_offset(1 as isize),
                        core::ptr::addr_of_mut!(start),
                    ) == 0
                    {
                        break '__join_0;
                    } else {
                        if nargs >= 3 {
                            if _PyEval_SliceIndex(
                                *args.wrapping_offset(2 as isize),
                                core::ptr::addr_of_mut!(end),
                            ) == 0
                            {
                                break '__join_0;
                            }
                        }
                    }
                }
                return_value = bytes_count_impl(
                    self_ as *mut __SlateRecord_bytesobject_h_15,
                    sub,
                    start,
                    end,
                );
            }
        }
        return return_value;
    }
}

static bytes_translate__doc__: __SlateAlign16<[i8; 337]> = __SlateAlign16([
    116i8, 114i8, 97i8, 110i8, 115i8, 108i8, 97i8, 116i8, 101i8, 40i8, 36i8, 115i8, 101i8, 108i8,
    102i8, 44i8, 32i8, 116i8, 97i8, 98i8, 108i8, 101i8, 44i8, 32i8, 47i8, 44i8, 32i8, 100i8, 101i8,
    108i8, 101i8, 116i8, 101i8, 61i8, 98i8, 39i8, 39i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8,
    101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 97i8, 32i8, 99i8, 111i8, 112i8, 121i8, 32i8, 119i8,
    105i8, 116i8, 104i8, 32i8, 101i8, 97i8, 99i8, 104i8, 32i8, 99i8, 104i8, 97i8, 114i8, 97i8,
    99i8, 116i8, 101i8, 114i8, 32i8, 109i8, 97i8, 112i8, 112i8, 101i8, 100i8, 32i8, 98i8, 121i8,
    32i8, 116i8, 104i8, 101i8, 32i8, 103i8, 105i8, 118i8, 101i8, 110i8, 32i8, 116i8, 114i8, 97i8,
    110i8, 115i8, 108i8, 97i8, 116i8, 105i8, 111i8, 110i8, 32i8, 116i8, 97i8, 98i8, 108i8, 101i8,
    46i8, 10i8, 10i8, 32i8, 32i8, 116i8, 97i8, 98i8, 108i8, 101i8, 10i8, 32i8, 32i8, 32i8, 32i8,
    84i8, 114i8, 97i8, 110i8, 115i8, 108i8, 97i8, 116i8, 105i8, 111i8, 110i8, 32i8, 116i8, 97i8,
    98i8, 108i8, 101i8, 44i8, 32i8, 119i8, 104i8, 105i8, 99i8, 104i8, 32i8, 109i8, 117i8, 115i8,
    116i8, 32i8, 98i8, 101i8, 32i8, 97i8, 32i8, 98i8, 121i8, 116i8, 101i8, 115i8, 32i8, 111i8,
    98i8, 106i8, 101i8, 99i8, 116i8, 32i8, 111i8, 102i8, 32i8, 108i8, 101i8, 110i8, 103i8, 116i8,
    104i8, 32i8, 50i8, 53i8, 54i8, 46i8, 10i8, 10i8, 65i8, 108i8, 108i8, 32i8, 99i8, 104i8, 97i8,
    114i8, 97i8, 99i8, 116i8, 101i8, 114i8, 115i8, 32i8, 111i8, 99i8, 99i8, 117i8, 114i8, 114i8,
    105i8, 110i8, 103i8, 32i8, 105i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 111i8, 112i8, 116i8,
    105i8, 111i8, 110i8, 97i8, 108i8, 32i8, 97i8, 114i8, 103i8, 117i8, 109i8, 101i8, 110i8, 116i8,
    32i8, 100i8, 101i8, 108i8, 101i8, 116i8, 101i8, 32i8, 97i8, 114i8, 101i8, 10i8, 114i8, 101i8,
    109i8, 111i8, 118i8, 101i8, 100i8, 46i8, 32i8, 32i8, 84i8, 104i8, 101i8, 32i8, 114i8, 101i8,
    109i8, 97i8, 105i8, 110i8, 105i8, 110i8, 103i8, 32i8, 99i8, 104i8, 97i8, 114i8, 97i8, 99i8,
    116i8, 101i8, 114i8, 115i8, 32i8, 97i8, 114i8, 101i8, 32i8, 109i8, 97i8, 112i8, 112i8, 101i8,
    100i8, 32i8, 116i8, 104i8, 114i8, 111i8, 117i8, 103i8, 104i8, 32i8, 116i8, 104i8, 101i8, 32i8,
    103i8, 105i8, 118i8, 101i8, 110i8, 10i8, 116i8, 114i8, 97i8, 110i8, 115i8, 108i8, 97i8, 116i8,
    105i8, 111i8, 110i8, 32i8, 116i8, 97i8, 98i8, 108i8, 101i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bytesobject.bytes_translate")]
extern "C-unwind" fn bytes_translate(
    self_: *mut _object,
    mut args: *const *mut _object,
    nargs: i64,
    kwnames: *mut _object,
) -> *mut _object {
    let mut deletechars: *mut _object = unsafe { core::mem::zeroed() };
    let mut table: *mut _object = unsafe { core::mem::zeroed() };
    let mut __v2: i64 = unsafe { core::mem::zeroed() };
    let mut noptargs: i64 = unsafe { core::mem::zeroed() };
    let mut argsbuf: core::mem::MaybeUninit<__SlateAlign16<[*mut _object; 2]>> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_4: *mut [*mut _object; 2] =
        core::ptr::addr_of_mut!(argsbuf) as *mut [*mut _object; 2];
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        if !kwnames.is_null() {
            __v2 = PyTuple_GET_SIZE!(kwnames);
        } else {
            __v2 = 0;
        }
        noptargs = nargs.wrapping_add(__v2).wrapping_sub(1);
        deletechars = core::ptr::null_mut::<_object>();
        args = _PyArg_UnpackKeywords!(
            args,
            nargs,
            core::ptr::null_mut::<()>(),
            kwnames,
            core::ptr::addr_of_mut!(_parser__719),
            1i32,
            2i32,
            0i32,
            0i32,
            (*__slate_slot_4).as_mut_ptr() as *mut *mut _object
        );
        // minpos
        // maxpos
        // minkw
        // varpos
        if !args.is_null() {
            table = *args.wrapping_offset(0 as isize);
            if noptargs != 0 {
                deletechars = *args.wrapping_offset(1 as isize);
            }
            return_value = bytes_translate_impl(
                self_ as *mut __SlateRecord_bytesobject_h_15,
                table,
                deletechars,
            );
        }
        return return_value;
    }
}

static mut _kwtuple__706: __SlateRecord_bytesobject_c_h_706 = {
    let mut __t0: __SlateRecord_bytesobject_c_h_706 = unsafe { core::mem::zeroed() };
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
                ._py_delete
                ._ascii
                .ob_base
        )
    }];
    __t0
};

static _keywords__718: __SlateAlign16<[*const i8; 3]> = __SlateAlign16([
    (b"\0".as_ptr() as *mut i8) as *const i8,
    (b"delete\0".as_ptr() as *mut i8) as *const i8,
    core::ptr::null::<i8>(),
]);

static mut _parser__719: _PyArg_Parser = {
    let mut __t0: _PyArg_Parser = unsafe { core::mem::zeroed() };
    __t0.keywords = unsafe { core::ptr::addr_of!(_keywords__718.0) as *const *const i8 };
    __t0.fname = (b"translate\0".as_ptr() as *mut i8) as *const i8;
    __t0.kwtuple = unsafe { core::ptr::addr_of_mut!(_kwtuple__706.ob_base.ob_base) };
    __t0
};

static bytes_maketrans__doc__: __SlateAlign16<[i8; 268]> = __SlateAlign16([
    109i8, 97i8, 107i8, 101i8, 116i8, 114i8, 97i8, 110i8, 115i8, 40i8, 102i8, 114i8, 109i8, 44i8,
    32i8, 116i8, 111i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8,
    117i8, 114i8, 110i8, 32i8, 97i8, 32i8, 116i8, 114i8, 97i8, 110i8, 115i8, 108i8, 97i8, 116i8,
    105i8, 111i8, 110i8, 32i8, 116i8, 97i8, 98i8, 108i8, 101i8, 32i8, 117i8, 115i8, 97i8, 98i8,
    108i8, 101i8, 32i8, 102i8, 111i8, 114i8, 32i8, 116i8, 104i8, 101i8, 32i8, 98i8, 121i8, 116i8,
    101i8, 115i8, 32i8, 111i8, 114i8, 32i8, 98i8, 121i8, 116i8, 101i8, 97i8, 114i8, 114i8, 97i8,
    121i8, 32i8, 116i8, 114i8, 97i8, 110i8, 115i8, 108i8, 97i8, 116i8, 101i8, 32i8, 109i8, 101i8,
    116i8, 104i8, 111i8, 100i8, 46i8, 10i8, 10i8, 84i8, 104i8, 101i8, 32i8, 114i8, 101i8, 116i8,
    117i8, 114i8, 110i8, 101i8, 100i8, 32i8, 116i8, 97i8, 98i8, 108i8, 101i8, 32i8, 119i8, 105i8,
    108i8, 108i8, 32i8, 98i8, 101i8, 32i8, 111i8, 110i8, 101i8, 32i8, 119i8, 104i8, 101i8, 114i8,
    101i8, 32i8, 101i8, 97i8, 99i8, 104i8, 32i8, 98i8, 121i8, 116i8, 101i8, 32i8, 105i8, 110i8,
    32i8, 102i8, 114i8, 109i8, 32i8, 105i8, 115i8, 32i8, 109i8, 97i8, 112i8, 112i8, 101i8, 100i8,
    32i8, 116i8, 111i8, 10i8, 116i8, 104i8, 101i8, 32i8, 98i8, 121i8, 116i8, 101i8, 32i8, 97i8,
    116i8, 32i8, 116i8, 104i8, 101i8, 32i8, 115i8, 97i8, 109i8, 101i8, 32i8, 112i8, 111i8, 115i8,
    105i8, 116i8, 105i8, 111i8, 110i8, 32i8, 105i8, 110i8, 32i8, 116i8, 111i8, 46i8, 10i8, 10i8,
    84i8, 104i8, 101i8, 32i8, 98i8, 121i8, 116i8, 101i8, 115i8, 32i8, 111i8, 98i8, 106i8, 101i8,
    99i8, 116i8, 115i8, 32i8, 102i8, 114i8, 109i8, 32i8, 97i8, 110i8, 100i8, 32i8, 116i8, 111i8,
    32i8, 109i8, 117i8, 115i8, 116i8, 32i8, 98i8, 101i8, 32i8, 111i8, 102i8, 32i8, 116i8, 104i8,
    101i8, 32i8, 115i8, 97i8, 109i8, 101i8, 32i8, 108i8, 101i8, 110i8, 103i8, 116i8, 104i8, 46i8,
    0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bytesobject.bytes_maketrans")]
extern "C-unwind" fn bytes_maketrans(
    null: *mut _object,
    args: *const *mut _object,
    nargs: i64,
) -> *mut _object {
    let mut to: core::mem::MaybeUninit<__SlateRecord_pybuffer_h_33> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_0: *mut __SlateRecord_pybuffer_h_33 =
        core::ptr::addr_of_mut!(to) as *mut __SlateRecord_pybuffer_h_33;
    let mut frm: core::mem::MaybeUninit<__SlateRecord_pybuffer_h_33> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_1: *mut __SlateRecord_pybuffer_h_33 =
        core::ptr::addr_of_mut!(frm) as *mut __SlateRecord_pybuffer_h_33;
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        core::ptr::write(__slate_slot_1, {
            let mut __t0: __SlateRecord_pybuffer_h_33 = core::mem::zeroed();
            __t0.buf = core::ptr::null_mut::<()>();
            __t0.obj = core::ptr::null_mut::<_object>();
            __t0
        });
        core::ptr::write(__slate_slot_0, {
            let mut __t1: __SlateRecord_pybuffer_h_33 = core::mem::zeroed();
            __t1.buf = core::ptr::null_mut::<()>();
            __t1.obj = core::ptr::null_mut::<_object>();
            __t1
        });
        if _PyArg_CheckPositional!(c"maketrans".as_ptr() as *mut i8, nargs, 2i32, 2i32) {
            if PyObject_GetBuffer(
                *args.wrapping_offset(0 as isize),
                core::ptr::addr_of_mut!(*__slate_slot_1),
                0i32,
            ) == 0
            {
                if PyObject_GetBuffer(
                    *args.wrapping_offset(1 as isize),
                    core::ptr::addr_of_mut!(*__slate_slot_0),
                    0i32,
                ) == 0
                {
                    return_value = bytes_maketrans_impl(
                        core::ptr::addr_of_mut!(*__slate_slot_1),
                        core::ptr::addr_of_mut!(*__slate_slot_0),
                    );
                }
            }
        }
        if !(*__slate_slot_1).obj.is_null() {
            PyBuffer_Release(core::ptr::addr_of_mut!(*__slate_slot_1));
        }
        // Cleanup for frm
        // Cleanup for to
        if !(*__slate_slot_0).obj.is_null() {
            PyBuffer_Release(core::ptr::addr_of_mut!(*__slate_slot_0));
        }
        return return_value;
    }
}

static bytes_replace__doc__: __SlateAlign16<[i8; 313]> = __SlateAlign16([
    114i8, 101i8, 112i8, 108i8, 97i8, 99i8, 101i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8,
    32i8, 111i8, 108i8, 100i8, 44i8, 32i8, 110i8, 101i8, 119i8, 44i8, 32i8, 99i8, 111i8, 117i8,
    110i8, 116i8, 61i8, 45i8, 49i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8,
    101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 97i8, 32i8, 99i8, 111i8, 112i8, 121i8, 32i8, 119i8,
    105i8, 116i8, 104i8, 32i8, 97i8, 108i8, 108i8, 32i8, 111i8, 99i8, 99i8, 117i8, 114i8, 114i8,
    101i8, 110i8, 99i8, 101i8, 115i8, 32i8, 111i8, 102i8, 32i8, 115i8, 117i8, 98i8, 115i8, 116i8,
    114i8, 105i8, 110i8, 103i8, 32i8, 111i8, 108i8, 100i8, 32i8, 114i8, 101i8, 112i8, 108i8, 97i8,
    99i8, 101i8, 100i8, 32i8, 98i8, 121i8, 32i8, 110i8, 101i8, 119i8, 46i8, 10i8, 10i8, 32i8, 32i8,
    99i8, 111i8, 117i8, 110i8, 116i8, 10i8, 32i8, 32i8, 32i8, 32i8, 77i8, 97i8, 120i8, 105i8,
    109i8, 117i8, 109i8, 32i8, 110i8, 117i8, 109i8, 98i8, 101i8, 114i8, 32i8, 111i8, 102i8, 32i8,
    111i8, 99i8, 99i8, 117i8, 114i8, 114i8, 101i8, 110i8, 99i8, 101i8, 115i8, 32i8, 116i8, 111i8,
    32i8, 114i8, 101i8, 112i8, 108i8, 97i8, 99i8, 101i8, 46i8, 10i8, 32i8, 32i8, 32i8, 32i8, 45i8,
    49i8, 32i8, 40i8, 116i8, 104i8, 101i8, 32i8, 100i8, 101i8, 102i8, 97i8, 117i8, 108i8, 116i8,
    32i8, 118i8, 97i8, 108i8, 117i8, 101i8, 41i8, 32i8, 109i8, 101i8, 97i8, 110i8, 115i8, 32i8,
    114i8, 101i8, 112i8, 108i8, 97i8, 99i8, 101i8, 32i8, 97i8, 108i8, 108i8, 32i8, 111i8, 99i8,
    99i8, 117i8, 114i8, 114i8, 101i8, 110i8, 99i8, 101i8, 115i8, 46i8, 10i8, 10i8, 73i8, 102i8,
    32i8, 116i8, 104i8, 101i8, 32i8, 111i8, 112i8, 116i8, 105i8, 111i8, 110i8, 97i8, 108i8, 32i8,
    97i8, 114i8, 103i8, 117i8, 109i8, 101i8, 110i8, 116i8, 32i8, 99i8, 111i8, 117i8, 110i8, 116i8,
    32i8, 105i8, 115i8, 32i8, 103i8, 105i8, 118i8, 101i8, 110i8, 44i8, 32i8, 111i8, 110i8, 108i8,
    121i8, 32i8, 116i8, 104i8, 101i8, 32i8, 102i8, 105i8, 114i8, 115i8, 116i8, 32i8, 99i8, 111i8,
    117i8, 110i8, 116i8, 32i8, 111i8, 99i8, 99i8, 117i8, 114i8, 114i8, 101i8, 110i8, 99i8, 101i8,
    115i8, 32i8, 97i8, 114i8, 101i8, 10i8, 114i8, 101i8, 112i8, 108i8, 97i8, 99i8, 101i8, 100i8,
    46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bytesobject.bytes_replace")]
extern "C-unwind" fn bytes_replace(
    self_: *mut _object,
    args: *const *mut _object,
    nargs: i64,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut iobj: *mut _object = unsafe { core::mem::zeroed() };
    let mut ival: i64 = unsafe { core::mem::zeroed() };
    let mut count: i64 = unsafe { core::mem::zeroed() };
    let mut new: core::mem::MaybeUninit<__SlateRecord_pybuffer_h_33> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_4: *mut __SlateRecord_pybuffer_h_33 =
        core::ptr::addr_of_mut!(new) as *mut __SlateRecord_pybuffer_h_33;
    let mut old: core::mem::MaybeUninit<__SlateRecord_pybuffer_h_33> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_5: *mut __SlateRecord_pybuffer_h_33 =
        core::ptr::addr_of_mut!(old) as *mut __SlateRecord_pybuffer_h_33;
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_4: {
            return_value = core::ptr::null_mut::<_object>();
            core::ptr::write(__slate_slot_5, {
                let mut __t0: __SlateRecord_pybuffer_h_33 = core::mem::zeroed();
                __t0.buf = core::ptr::null_mut::<()>();
                __t0.obj = core::ptr::null_mut::<_object>();
                __t0
            });
            core::ptr::write(__slate_slot_4, {
                let mut __t1: __SlateRecord_pybuffer_h_33 = core::mem::zeroed();
                __t1.buf = core::ptr::null_mut::<()>();
                __t1.obj = core::ptr::null_mut::<_object>();
                __t1
            });
            count = 1i32.wrapping_neg() as i64;
            if _PyArg_CheckPositional!(c"replace".as_ptr() as *mut i8, nargs, 2i32, 3i32) {
                if PyObject_GetBuffer(
                    *args.wrapping_offset(0 as isize),
                    core::ptr::addr_of_mut!(*__slate_slot_5),
                    0i32,
                ) == 0
                {
                    if PyObject_GetBuffer(
                        *args.wrapping_offset(1 as isize),
                        core::ptr::addr_of_mut!(*__slate_slot_4),
                        0i32,
                    ) == 0
                    {
                        if nargs >= 3 {
                            ival = 1i32.wrapping_neg() as i64;
                            iobj = _PyNumber_Index(*args.wrapping_offset(2 as isize));
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
                                break '__join_4;
                            } else {
                                count = ival;
                            }
                        }
                        return_value = bytes_replace_impl(
                            self_ as *mut __SlateRecord_bytesobject_h_15,
                            &(*core::ptr::addr_of_mut!(*__slate_slot_5)),
                            &(*core::ptr::addr_of_mut!(*__slate_slot_4)),
                            count,
                        );
                    }
                }
            }
        }
        if !(*__slate_slot_5).obj.is_null() {
            PyBuffer_Release(core::ptr::addr_of_mut!(*__slate_slot_5));
        }
        // Cleanup for old
        // Cleanup for new
        if !(*__slate_slot_4).obj.is_null() {
            PyBuffer_Release(core::ptr::addr_of_mut!(*__slate_slot_4));
        }
        return return_value;
    }
}

static bytes_removeprefix__doc__: __SlateAlign16<[i8; 228]> = __SlateAlign16([
    114i8, 101i8, 109i8, 111i8, 118i8, 101i8, 112i8, 114i8, 101i8, 102i8, 105i8, 120i8, 40i8, 36i8,
    115i8, 101i8, 108i8, 102i8, 44i8, 32i8, 112i8, 114i8, 101i8, 102i8, 105i8, 120i8, 44i8, 32i8,
    47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 97i8,
    32i8, 98i8, 121i8, 116i8, 101i8, 115i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 32i8,
    119i8, 105i8, 116i8, 104i8, 32i8, 116i8, 104i8, 101i8, 32i8, 103i8, 105i8, 118i8, 101i8, 110i8,
    32i8, 112i8, 114i8, 101i8, 102i8, 105i8, 120i8, 32i8, 115i8, 116i8, 114i8, 105i8, 110i8, 103i8,
    32i8, 114i8, 101i8, 109i8, 111i8, 118i8, 101i8, 100i8, 32i8, 105i8, 102i8, 32i8, 112i8, 114i8,
    101i8, 115i8, 101i8, 110i8, 116i8, 46i8, 10i8, 10i8, 73i8, 102i8, 32i8, 116i8, 104i8, 101i8,
    32i8, 98i8, 121i8, 116i8, 101i8, 115i8, 32i8, 115i8, 116i8, 97i8, 114i8, 116i8, 115i8, 32i8,
    119i8, 105i8, 116i8, 104i8, 32i8, 116i8, 104i8, 101i8, 32i8, 112i8, 114i8, 101i8, 102i8, 105i8,
    120i8, 32i8, 115i8, 116i8, 114i8, 105i8, 110i8, 103i8, 44i8, 32i8, 114i8, 101i8, 116i8, 117i8,
    114i8, 110i8, 10i8, 98i8, 121i8, 116i8, 101i8, 115i8, 91i8, 108i8, 101i8, 110i8, 40i8, 112i8,
    114i8, 101i8, 102i8, 105i8, 120i8, 41i8, 58i8, 93i8, 46i8, 32i8, 32i8, 79i8, 116i8, 104i8,
    101i8, 114i8, 119i8, 105i8, 115i8, 101i8, 44i8, 32i8, 114i8, 101i8, 116i8, 117i8, 114i8, 110i8,
    32i8, 97i8, 32i8, 99i8, 111i8, 112i8, 121i8, 32i8, 111i8, 102i8, 32i8, 116i8, 104i8, 101i8,
    32i8, 111i8, 114i8, 105i8, 103i8, 105i8, 110i8, 97i8, 108i8, 10i8, 98i8, 121i8, 116i8, 101i8,
    115i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bytesobject.bytes_removeprefix")]
extern "C-unwind" fn bytes_removeprefix(self_: *mut _object, arg: *mut _object) -> *mut _object {
    let mut prefix: core::mem::MaybeUninit<__SlateRecord_pybuffer_h_33> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_0: *mut __SlateRecord_pybuffer_h_33 =
        core::ptr::addr_of_mut!(prefix) as *mut __SlateRecord_pybuffer_h_33;
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        core::ptr::write(__slate_slot_0, {
            let mut __t0: __SlateRecord_pybuffer_h_33 = core::mem::zeroed();
            __t0.buf = core::ptr::null_mut::<()>();
            __t0.obj = core::ptr::null_mut::<_object>();
            __t0
        });
        if PyObject_GetBuffer(arg, core::ptr::addr_of_mut!(*__slate_slot_0), 0i32) == 0 {
            return_value = bytes_removeprefix_impl(
                self_ as *mut __SlateRecord_bytesobject_h_15,
                &(*core::ptr::addr_of_mut!(*__slate_slot_0)),
            );
        }
        if !(*__slate_slot_0).obj.is_null() {
            PyBuffer_Release(core::ptr::addr_of_mut!(*__slate_slot_0));
        }
        // Cleanup for prefix
        return return_value;
    }
}

static bytes_removesuffix__doc__: __SlateAlign16<[i8; 256]> = __SlateAlign16([
    114i8, 101i8, 109i8, 111i8, 118i8, 101i8, 115i8, 117i8, 102i8, 102i8, 105i8, 120i8, 40i8, 36i8,
    115i8, 101i8, 108i8, 102i8, 44i8, 32i8, 115i8, 117i8, 102i8, 102i8, 105i8, 120i8, 44i8, 32i8,
    47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 97i8,
    32i8, 98i8, 121i8, 116i8, 101i8, 115i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 32i8,
    119i8, 105i8, 116i8, 104i8, 32i8, 116i8, 104i8, 101i8, 32i8, 103i8, 105i8, 118i8, 101i8, 110i8,
    32i8, 115i8, 117i8, 102i8, 102i8, 105i8, 120i8, 32i8, 115i8, 116i8, 114i8, 105i8, 110i8, 103i8,
    32i8, 114i8, 101i8, 109i8, 111i8, 118i8, 101i8, 100i8, 32i8, 105i8, 102i8, 32i8, 112i8, 114i8,
    101i8, 115i8, 101i8, 110i8, 116i8, 46i8, 10i8, 10i8, 73i8, 102i8, 32i8, 116i8, 104i8, 101i8,
    32i8, 98i8, 121i8, 116i8, 101i8, 115i8, 32i8, 101i8, 110i8, 100i8, 115i8, 32i8, 119i8, 105i8,
    116i8, 104i8, 32i8, 116i8, 104i8, 101i8, 32i8, 115i8, 117i8, 102i8, 102i8, 105i8, 120i8, 32i8,
    115i8, 116i8, 114i8, 105i8, 110i8, 103i8, 32i8, 97i8, 110i8, 100i8, 32i8, 116i8, 104i8, 97i8,
    116i8, 32i8, 115i8, 117i8, 102i8, 102i8, 105i8, 120i8, 32i8, 105i8, 115i8, 32i8, 110i8, 111i8,
    116i8, 10i8, 101i8, 109i8, 112i8, 116i8, 121i8, 44i8, 32i8, 114i8, 101i8, 116i8, 117i8, 114i8,
    110i8, 32i8, 98i8, 121i8, 116i8, 101i8, 115i8, 91i8, 58i8, 45i8, 108i8, 101i8, 110i8, 40i8,
    112i8, 114i8, 101i8, 102i8, 105i8, 120i8, 41i8, 93i8, 46i8, 32i8, 32i8, 79i8, 116i8, 104i8,
    101i8, 114i8, 119i8, 105i8, 115i8, 101i8, 44i8, 32i8, 114i8, 101i8, 116i8, 117i8, 114i8, 110i8,
    32i8, 97i8, 32i8, 99i8, 111i8, 112i8, 121i8, 32i8, 111i8, 102i8, 32i8, 116i8, 104i8, 101i8,
    10i8, 111i8, 114i8, 105i8, 103i8, 105i8, 110i8, 97i8, 108i8, 32i8, 98i8, 121i8, 116i8, 101i8,
    115i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bytesobject.bytes_removesuffix")]
extern "C-unwind" fn bytes_removesuffix(self_: *mut _object, arg: *mut _object) -> *mut _object {
    let mut suffix: core::mem::MaybeUninit<__SlateRecord_pybuffer_h_33> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_0: *mut __SlateRecord_pybuffer_h_33 =
        core::ptr::addr_of_mut!(suffix) as *mut __SlateRecord_pybuffer_h_33;
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        core::ptr::write(__slate_slot_0, {
            let mut __t0: __SlateRecord_pybuffer_h_33 = core::mem::zeroed();
            __t0.buf = core::ptr::null_mut::<()>();
            __t0.obj = core::ptr::null_mut::<_object>();
            __t0
        });
        if PyObject_GetBuffer(arg, core::ptr::addr_of_mut!(*__slate_slot_0), 0i32) == 0 {
            return_value = bytes_removesuffix_impl(
                self_ as *mut __SlateRecord_bytesobject_h_15,
                &(*core::ptr::addr_of_mut!(*__slate_slot_0)),
            );
        }
        if !(*__slate_slot_0).obj.is_null() {
            PyBuffer_Release(core::ptr::addr_of_mut!(*__slate_slot_0));
        }
        // Cleanup for suffix
        return return_value;
    }
}

static bytes_startswith__doc__: __SlateAlign16<[i8; 302]> = __SlateAlign16([
    115i8, 116i8, 97i8, 114i8, 116i8, 115i8, 119i8, 105i8, 116i8, 104i8, 40i8, 36i8, 115i8, 101i8,
    108i8, 102i8, 44i8, 32i8, 112i8, 114i8, 101i8, 102i8, 105i8, 120i8, 91i8, 44i8, 32i8, 115i8,
    116i8, 97i8, 114i8, 116i8, 91i8, 44i8, 32i8, 101i8, 110i8, 100i8, 93i8, 93i8, 44i8, 32i8, 47i8,
    41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 84i8, 114i8,
    117i8, 101i8, 32i8, 105i8, 102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 98i8, 121i8, 116i8, 101i8,
    115i8, 32i8, 115i8, 116i8, 97i8, 114i8, 116i8, 115i8, 32i8, 119i8, 105i8, 116i8, 104i8, 32i8,
    116i8, 104i8, 101i8, 32i8, 115i8, 112i8, 101i8, 99i8, 105i8, 102i8, 105i8, 101i8, 100i8, 32i8,
    112i8, 114i8, 101i8, 102i8, 105i8, 120i8, 44i8, 32i8, 70i8, 97i8, 108i8, 115i8, 101i8, 32i8,
    111i8, 116i8, 104i8, 101i8, 114i8, 119i8, 105i8, 115i8, 101i8, 46i8, 10i8, 10i8, 32i8, 32i8,
    112i8, 114i8, 101i8, 102i8, 105i8, 120i8, 10i8, 32i8, 32i8, 32i8, 32i8, 65i8, 32i8, 98i8,
    121i8, 116i8, 101i8, 115i8, 32i8, 111i8, 114i8, 32i8, 97i8, 32i8, 116i8, 117i8, 112i8, 108i8,
    101i8, 32i8, 111i8, 102i8, 32i8, 98i8, 121i8, 116i8, 101i8, 115i8, 32i8, 116i8, 111i8, 32i8,
    116i8, 114i8, 121i8, 46i8, 10i8, 32i8, 32i8, 115i8, 116i8, 97i8, 114i8, 116i8, 10i8, 32i8,
    32i8, 32i8, 32i8, 79i8, 112i8, 116i8, 105i8, 111i8, 110i8, 97i8, 108i8, 32i8, 115i8, 116i8,
    97i8, 114i8, 116i8, 32i8, 112i8, 111i8, 115i8, 105i8, 116i8, 105i8, 111i8, 110i8, 46i8, 32i8,
    68i8, 101i8, 102i8, 97i8, 117i8, 108i8, 116i8, 58i8, 32i8, 115i8, 116i8, 97i8, 114i8, 116i8,
    32i8, 111i8, 102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 98i8, 121i8, 116i8, 101i8, 115i8, 46i8,
    10i8, 32i8, 32i8, 101i8, 110i8, 100i8, 10i8, 32i8, 32i8, 32i8, 32i8, 79i8, 112i8, 116i8, 105i8,
    111i8, 110i8, 97i8, 108i8, 32i8, 115i8, 116i8, 111i8, 112i8, 32i8, 112i8, 111i8, 115i8, 105i8,
    116i8, 105i8, 111i8, 110i8, 46i8, 32i8, 68i8, 101i8, 102i8, 97i8, 117i8, 108i8, 116i8, 58i8,
    32i8, 101i8, 110i8, 100i8, 32i8, 111i8, 102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 98i8, 121i8,
    116i8, 101i8, 115i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bytesobject.bytes_startswith")]
extern "C-unwind" fn bytes_startswith(
    self_: *mut _object,
    args: *const *mut _object,
    nargs: i64,
) -> *mut _object {
    let mut end: i64 = unsafe { core::mem::zeroed() };
    let mut start: i64 = unsafe { core::mem::zeroed() };
    let mut subobj: *mut _object = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_0: {
            return_value = core::ptr::null_mut::<_object>();
            start = 0;
            end = 9223372036854775807;
            if _PyArg_CheckPositional!(c"startswith".as_ptr() as *mut i8, nargs, 1i32, 3i32) {
                subobj = *args.wrapping_offset(0 as isize);
                if nargs >= 2 {
                    if _PyEval_SliceIndex(
                        *args.wrapping_offset(1 as isize),
                        core::ptr::addr_of_mut!(start),
                    ) == 0
                    {
                        break '__join_0;
                    } else {
                        if nargs >= 3 {
                            if _PyEval_SliceIndex(
                                *args.wrapping_offset(2 as isize),
                                core::ptr::addr_of_mut!(end),
                            ) == 0
                            {
                                break '__join_0;
                            }
                        }
                    }
                }
                return_value = bytes_startswith_impl(
                    self_ as *mut __SlateRecord_bytesobject_h_15,
                    subobj,
                    start,
                    end,
                );
            }
        }
        return return_value;
    }
}

static bytes_endswith__doc__: __SlateAlign16<[i8; 298]> = __SlateAlign16([
    101i8, 110i8, 100i8, 115i8, 119i8, 105i8, 116i8, 104i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8,
    44i8, 32i8, 115i8, 117i8, 102i8, 102i8, 105i8, 120i8, 91i8, 44i8, 32i8, 115i8, 116i8, 97i8,
    114i8, 116i8, 91i8, 44i8, 32i8, 101i8, 110i8, 100i8, 93i8, 93i8, 44i8, 32i8, 47i8, 41i8, 10i8,
    45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 84i8, 114i8, 117i8,
    101i8, 32i8, 105i8, 102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 98i8, 121i8, 116i8, 101i8, 115i8,
    32i8, 101i8, 110i8, 100i8, 115i8, 32i8, 119i8, 105i8, 116i8, 104i8, 32i8, 116i8, 104i8, 101i8,
    32i8, 115i8, 112i8, 101i8, 99i8, 105i8, 102i8, 105i8, 101i8, 100i8, 32i8, 115i8, 117i8, 102i8,
    102i8, 105i8, 120i8, 44i8, 32i8, 70i8, 97i8, 108i8, 115i8, 101i8, 32i8, 111i8, 116i8, 104i8,
    101i8, 114i8, 119i8, 105i8, 115i8, 101i8, 46i8, 10i8, 10i8, 32i8, 32i8, 115i8, 117i8, 102i8,
    102i8, 105i8, 120i8, 10i8, 32i8, 32i8, 32i8, 32i8, 65i8, 32i8, 98i8, 121i8, 116i8, 101i8,
    115i8, 32i8, 111i8, 114i8, 32i8, 97i8, 32i8, 116i8, 117i8, 112i8, 108i8, 101i8, 32i8, 111i8,
    102i8, 32i8, 98i8, 121i8, 116i8, 101i8, 115i8, 32i8, 116i8, 111i8, 32i8, 116i8, 114i8, 121i8,
    46i8, 10i8, 32i8, 32i8, 115i8, 116i8, 97i8, 114i8, 116i8, 10i8, 32i8, 32i8, 32i8, 32i8, 79i8,
    112i8, 116i8, 105i8, 111i8, 110i8, 97i8, 108i8, 32i8, 115i8, 116i8, 97i8, 114i8, 116i8, 32i8,
    112i8, 111i8, 115i8, 105i8, 116i8, 105i8, 111i8, 110i8, 46i8, 32i8, 68i8, 101i8, 102i8, 97i8,
    117i8, 108i8, 116i8, 58i8, 32i8, 115i8, 116i8, 97i8, 114i8, 116i8, 32i8, 111i8, 102i8, 32i8,
    116i8, 104i8, 101i8, 32i8, 98i8, 121i8, 116i8, 101i8, 115i8, 46i8, 10i8, 32i8, 32i8, 101i8,
    110i8, 100i8, 10i8, 32i8, 32i8, 32i8, 32i8, 79i8, 112i8, 116i8, 105i8, 111i8, 110i8, 97i8,
    108i8, 32i8, 115i8, 116i8, 111i8, 112i8, 32i8, 112i8, 111i8, 115i8, 105i8, 116i8, 105i8, 111i8,
    110i8, 46i8, 32i8, 68i8, 101i8, 102i8, 97i8, 117i8, 108i8, 116i8, 58i8, 32i8, 101i8, 110i8,
    100i8, 32i8, 111i8, 102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 98i8, 121i8, 116i8, 101i8, 115i8,
    46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bytesobject.bytes_endswith")]
extern "C-unwind" fn bytes_endswith(
    self_: *mut _object,
    args: *const *mut _object,
    nargs: i64,
) -> *mut _object {
    let mut end: i64 = unsafe { core::mem::zeroed() };
    let mut start: i64 = unsafe { core::mem::zeroed() };
    let mut subobj: *mut _object = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_0: {
            return_value = core::ptr::null_mut::<_object>();
            start = 0;
            end = 9223372036854775807;
            if _PyArg_CheckPositional!(c"endswith".as_ptr() as *mut i8, nargs, 1i32, 3i32) {
                subobj = *args.wrapping_offset(0 as isize);
                if nargs >= 2 {
                    if _PyEval_SliceIndex(
                        *args.wrapping_offset(1 as isize),
                        core::ptr::addr_of_mut!(start),
                    ) == 0
                    {
                        break '__join_0;
                    } else {
                        if nargs >= 3 {
                            if _PyEval_SliceIndex(
                                *args.wrapping_offset(2 as isize),
                                core::ptr::addr_of_mut!(end),
                            ) == 0
                            {
                                break '__join_0;
                            }
                        }
                    }
                }
                return_value = bytes_endswith_impl(
                    self_ as *mut __SlateRecord_bytesobject_h_15,
                    subobj,
                    start,
                    end,
                );
            }
        }
        return return_value;
    }
}

static bytes_decode__doc__: __SlateAlign16<[i8; 507]> = __SlateAlign16([
    100i8, 101i8, 99i8, 111i8, 100i8, 101i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8,
    47i8, 44i8, 32i8, 101i8, 110i8, 99i8, 111i8, 100i8, 105i8, 110i8, 103i8, 61i8, 39i8, 117i8,
    116i8, 102i8, 45i8, 56i8, 39i8, 44i8, 32i8, 101i8, 114i8, 114i8, 111i8, 114i8, 115i8, 61i8,
    39i8, 115i8, 116i8, 114i8, 105i8, 99i8, 116i8, 39i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 68i8,
    101i8, 99i8, 111i8, 100i8, 101i8, 32i8, 116i8, 104i8, 101i8, 32i8, 98i8, 121i8, 116i8, 101i8,
    115i8, 32i8, 117i8, 115i8, 105i8, 110i8, 103i8, 32i8, 116i8, 104i8, 101i8, 32i8, 99i8, 111i8,
    100i8, 101i8, 99i8, 32i8, 114i8, 101i8, 103i8, 105i8, 115i8, 116i8, 101i8, 114i8, 101i8, 100i8,
    32i8, 102i8, 111i8, 114i8, 32i8, 101i8, 110i8, 99i8, 111i8, 100i8, 105i8, 110i8, 103i8, 46i8,
    10i8, 10i8, 32i8, 32i8, 101i8, 110i8, 99i8, 111i8, 100i8, 105i8, 110i8, 103i8, 10i8, 32i8,
    32i8, 32i8, 32i8, 84i8, 104i8, 101i8, 32i8, 101i8, 110i8, 99i8, 111i8, 100i8, 105i8, 110i8,
    103i8, 32i8, 119i8, 105i8, 116i8, 104i8, 32i8, 119i8, 104i8, 105i8, 99i8, 104i8, 32i8, 116i8,
    111i8, 32i8, 100i8, 101i8, 99i8, 111i8, 100i8, 101i8, 32i8, 116i8, 104i8, 101i8, 32i8, 98i8,
    121i8, 116i8, 101i8, 115i8, 46i8, 10i8, 32i8, 32i8, 101i8, 114i8, 114i8, 111i8, 114i8, 115i8,
    10i8, 32i8, 32i8, 32i8, 32i8, 84i8, 104i8, 101i8, 32i8, 101i8, 114i8, 114i8, 111i8, 114i8,
    32i8, 104i8, 97i8, 110i8, 100i8, 108i8, 105i8, 110i8, 103i8, 32i8, 115i8, 99i8, 104i8, 101i8,
    109i8, 101i8, 32i8, 116i8, 111i8, 32i8, 117i8, 115i8, 101i8, 32i8, 102i8, 111i8, 114i8, 32i8,
    116i8, 104i8, 101i8, 32i8, 104i8, 97i8, 110i8, 100i8, 108i8, 105i8, 110i8, 103i8, 32i8, 111i8,
    102i8, 32i8, 100i8, 101i8, 99i8, 111i8, 100i8, 105i8, 110i8, 103i8, 10i8, 32i8, 32i8, 32i8,
    32i8, 101i8, 114i8, 114i8, 111i8, 114i8, 115i8, 46i8, 32i8, 32i8, 84i8, 104i8, 101i8, 32i8,
    100i8, 101i8, 102i8, 97i8, 117i8, 108i8, 116i8, 32i8, 105i8, 115i8, 32i8, 39i8, 115i8, 116i8,
    114i8, 105i8, 99i8, 116i8, 39i8, 32i8, 109i8, 101i8, 97i8, 110i8, 105i8, 110i8, 103i8, 32i8,
    116i8, 104i8, 97i8, 116i8, 32i8, 100i8, 101i8, 99i8, 111i8, 100i8, 105i8, 110i8, 103i8, 32i8,
    101i8, 114i8, 114i8, 111i8, 114i8, 115i8, 10i8, 32i8, 32i8, 32i8, 32i8, 114i8, 97i8, 105i8,
    115i8, 101i8, 32i8, 97i8, 32i8, 85i8, 110i8, 105i8, 99i8, 111i8, 100i8, 101i8, 68i8, 101i8,
    99i8, 111i8, 100i8, 101i8, 69i8, 114i8, 114i8, 111i8, 114i8, 46i8, 32i8, 32i8, 79i8, 116i8,
    104i8, 101i8, 114i8, 32i8, 112i8, 111i8, 115i8, 115i8, 105i8, 98i8, 108i8, 101i8, 32i8, 118i8,
    97i8, 108i8, 117i8, 101i8, 115i8, 32i8, 97i8, 114i8, 101i8, 32i8, 39i8, 105i8, 103i8, 110i8,
    111i8, 114i8, 101i8, 39i8, 10i8, 32i8, 32i8, 32i8, 32i8, 97i8, 110i8, 100i8, 32i8, 39i8, 114i8,
    101i8, 112i8, 108i8, 97i8, 99i8, 101i8, 39i8, 32i8, 97i8, 115i8, 32i8, 119i8, 101i8, 108i8,
    108i8, 32i8, 97i8, 115i8, 32i8, 97i8, 110i8, 121i8, 32i8, 111i8, 116i8, 104i8, 101i8, 114i8,
    32i8, 110i8, 97i8, 109i8, 101i8, 32i8, 114i8, 101i8, 103i8, 105i8, 115i8, 116i8, 101i8, 114i8,
    101i8, 100i8, 32i8, 119i8, 105i8, 116i8, 104i8, 10i8, 32i8, 32i8, 32i8, 32i8, 99i8, 111i8,
    100i8, 101i8, 99i8, 115i8, 46i8, 114i8, 101i8, 103i8, 105i8, 115i8, 116i8, 101i8, 114i8, 95i8,
    101i8, 114i8, 114i8, 111i8, 114i8, 32i8, 116i8, 104i8, 97i8, 116i8, 32i8, 99i8, 97i8, 110i8,
    32i8, 104i8, 97i8, 110i8, 100i8, 108i8, 101i8, 32i8, 85i8, 110i8, 105i8, 99i8, 111i8, 100i8,
    101i8, 68i8, 101i8, 99i8, 111i8, 100i8, 101i8, 69i8, 114i8, 114i8, 111i8, 114i8, 115i8, 46i8,
    0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bytesobject.bytes_decode")]
extern "C-unwind" fn bytes_decode(
    self_: *mut _object,
    mut args: *const *mut _object,
    nargs: i64,
    kwnames: *mut _object,
) -> *mut _object {
    let mut errors_length: i64 = unsafe { core::mem::zeroed() };
    let mut encoding_length: i64 = unsafe { core::mem::zeroed() };
    let mut errors: *const i8 = unsafe { core::mem::zeroed() };
    let mut encoding: *const i8 = unsafe { core::mem::zeroed() };
    let mut __v6: i64 = unsafe { core::mem::zeroed() };
    let mut noptargs: i64 = unsafe { core::mem::zeroed() };
    let mut argsbuf: core::mem::MaybeUninit<__SlateAlign16<[*mut _object; 2]>> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_8: *mut [*mut _object; 2] =
        core::ptr::addr_of_mut!(argsbuf) as *mut [*mut _object; 2];
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        if !kwnames.is_null() {
            __v6 = PyTuple_GET_SIZE!(kwnames);
        } else {
            __v6 = 0;
        }
        '__join_0: {
            noptargs = nargs.wrapping_add(__v6).wrapping_sub(0);
            encoding = core::ptr::null::<i8>();
            errors = core::ptr::null::<i8>();
            args = _PyArg_UnpackKeywords!(
                args,
                nargs,
                core::ptr::null_mut::<()>(),
                kwnames,
                core::ptr::addr_of_mut!(_parser__1084),
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
            if !args.is_null() {
                '__join_1: {
                    if noptargs != 0 {
                        if !unsafe { *args.wrapping_offset(0 as isize) }.is_null() {
                            if PyUnicode_Check!(unsafe { *args.wrapping_offset(0 as isize) }) == 0 {
                                _PyArg_BadArgument(
                                    c"decode".as_ptr(),
                                    c"argument 'encoding'".as_ptr(),
                                    c"str".as_ptr(),
                                    *args.wrapping_offset(0 as isize),
                                );
                                break '__join_0;
                            } else {
                                encoding = PyUnicode_AsUTF8AndSize(
                                    *args.wrapping_offset(0 as isize),
                                    core::ptr::addr_of_mut!(encoding_length),
                                );
                                if encoding.is_null() {
                                    break '__join_0;
                                } else {
                                    if (strlen(encoding as *const core::ffi::c_char) as u64)
                                        != (encoding_length as u64)
                                    {
                                        PyErr_SetString(
                                            PyExc_ValueError,
                                            c"embedded null character".as_ptr(),
                                        );
                                        break '__join_0;
                                    } else {
                                        let __v1: i64 = noptargs.wrapping_sub(1);
                                        noptargs = __v1;
                                        if __v1 == 0 {
                                            break '__join_1;
                                        }
                                    }
                                }
                            }
                        }
                        if PyUnicode_Check!(unsafe { *args.wrapping_offset(1 as isize) }) == 0 {
                            _PyArg_BadArgument(
                                c"decode".as_ptr(),
                                c"argument 'errors'".as_ptr(),
                                c"str".as_ptr(),
                                *args.wrapping_offset(1 as isize),
                            );
                            break '__join_0;
                        } else {
                            errors = PyUnicode_AsUTF8AndSize(
                                *args.wrapping_offset(1 as isize),
                                core::ptr::addr_of_mut!(errors_length),
                            );
                            if errors.is_null() {
                                break '__join_0;
                            } else {
                                if (strlen(errors as *const core::ffi::c_char) as u64)
                                    != (errors_length as u64)
                                {
                                    PyErr_SetString(
                                        PyExc_ValueError,
                                        c"embedded null character".as_ptr(),
                                    );
                                    break '__join_0;
                                }
                            }
                        }
                    }
                }
                return_value = bytes_decode_impl(
                    self_ as *mut __SlateRecord_bytesobject_h_15,
                    encoding,
                    errors,
                );
            }
        }
        return return_value;
    }
}

static mut _kwtuple__1071: __SlateRecord_bytesobject_c_h_1071 = {
    let mut __t0: __SlateRecord_bytesobject_c_h_1071 = unsafe { core::mem::zeroed() };
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
                    ._py_encoding
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
                    ._py_errors
                    ._ascii
                    .ob_base
            )
        },
    ];
    __t0
};

static _keywords__1083: __SlateAlign16<[*const i8; 3]> = __SlateAlign16([
    (b"encoding\0".as_ptr() as *mut i8) as *const i8,
    (b"errors\0".as_ptr() as *mut i8) as *const i8,
    core::ptr::null::<i8>(),
]);

static mut _parser__1084: _PyArg_Parser = {
    let mut __t0: _PyArg_Parser = unsafe { core::mem::zeroed() };
    __t0.keywords = unsafe { core::ptr::addr_of!(_keywords__1083.0) as *const *const i8 };
    __t0.fname = (b"decode\0".as_ptr() as *mut i8) as *const i8;
    __t0.kwtuple = unsafe { core::ptr::addr_of_mut!(_kwtuple__1071.ob_base.ob_base) };
    __t0
};

static bytes_splitlines__doc__: __SlateAlign16<[i8; 198]> = __SlateAlign16([
    115i8, 112i8, 108i8, 105i8, 116i8, 108i8, 105i8, 110i8, 101i8, 115i8, 40i8, 36i8, 115i8, 101i8,
    108i8, 102i8, 44i8, 32i8, 47i8, 44i8, 32i8, 107i8, 101i8, 101i8, 112i8, 101i8, 110i8, 100i8,
    115i8, 61i8, 70i8, 97i8, 108i8, 115i8, 101i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8,
    116i8, 117i8, 114i8, 110i8, 32i8, 97i8, 32i8, 108i8, 105i8, 115i8, 116i8, 32i8, 111i8, 102i8,
    32i8, 116i8, 104i8, 101i8, 32i8, 108i8, 105i8, 110i8, 101i8, 115i8, 32i8, 105i8, 110i8, 32i8,
    116i8, 104i8, 101i8, 32i8, 98i8, 121i8, 116i8, 101i8, 115i8, 44i8, 32i8, 98i8, 114i8, 101i8,
    97i8, 107i8, 105i8, 110i8, 103i8, 32i8, 97i8, 116i8, 32i8, 108i8, 105i8, 110i8, 101i8, 32i8,
    98i8, 111i8, 117i8, 110i8, 100i8, 97i8, 114i8, 105i8, 101i8, 115i8, 46i8, 10i8, 10i8, 76i8,
    105i8, 110i8, 101i8, 32i8, 98i8, 114i8, 101i8, 97i8, 107i8, 115i8, 32i8, 97i8, 114i8, 101i8,
    32i8, 110i8, 111i8, 116i8, 32i8, 105i8, 110i8, 99i8, 108i8, 117i8, 100i8, 101i8, 100i8, 32i8,
    105i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 114i8, 101i8, 115i8, 117i8, 108i8, 116i8, 105i8,
    110i8, 103i8, 32i8, 108i8, 105i8, 115i8, 116i8, 32i8, 117i8, 110i8, 108i8, 101i8, 115i8, 115i8,
    32i8, 107i8, 101i8, 101i8, 112i8, 101i8, 110i8, 100i8, 115i8, 10i8, 105i8, 115i8, 32i8, 103i8,
    105i8, 118i8, 101i8, 110i8, 32i8, 97i8, 110i8, 100i8, 32i8, 116i8, 114i8, 117i8, 101i8, 46i8,
    0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bytesobject.bytes_splitlines")]
extern "C-unwind" fn bytes_splitlines(
    self_: *mut _object,
    mut args: *const *mut _object,
    nargs: i64,
    kwnames: *mut _object,
) -> *mut _object {
    let mut keepends: i32 = unsafe { core::mem::zeroed() };
    let mut __v1: i64 = unsafe { core::mem::zeroed() };
    let mut noptargs: i64 = unsafe { core::mem::zeroed() };
    let mut argsbuf: [*mut _object; 1] = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        if !kwnames.is_null() {
            __v1 = PyTuple_GET_SIZE!(kwnames);
        } else {
            __v1 = 0;
        }
        '__join_0: {
            noptargs = nargs.wrapping_add(__v1).wrapping_sub(0);
            keepends = 0;
            args = _PyArg_UnpackKeywords!(
                args,
                nargs,
                core::ptr::null_mut::<()>(),
                kwnames,
                core::ptr::addr_of_mut!(_parser__1181),
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
                    keepends = PyObject_IsTrue(*args.wrapping_offset(0 as isize));
                    if keepends < 0 {
                        break '__join_0;
                    }
                }
                return_value =
                    bytes_splitlines_impl(self_ as *mut __SlateRecord_bytesobject_h_15, keepends);
            }
        }
        return return_value;
    }
}

static mut _kwtuple__1168: __SlateRecord_bytesobject_c_h_1168 = {
    let mut __t0: __SlateRecord_bytesobject_c_h_1168 = unsafe { core::mem::zeroed() };
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
                ._py_keepends
                ._ascii
                .ob_base
        )
    }];
    __t0
};

static _keywords__1180: __SlateAlign16<[*const i8; 2]> = __SlateAlign16([
    (b"keepends\0".as_ptr() as *mut i8) as *const i8,
    core::ptr::null::<i8>(),
]);

static mut _parser__1181: _PyArg_Parser = {
    let mut __t0: _PyArg_Parser = unsafe { core::mem::zeroed() };
    __t0.keywords = unsafe { core::ptr::addr_of!(_keywords__1180.0) as *const *const i8 };
    __t0.fname = (b"splitlines\0".as_ptr() as *mut i8) as *const i8;
    __t0.kwtuple = unsafe { core::ptr::addr_of_mut!(_kwtuple__1168.ob_base.ob_base) };
    __t0
};

static bytes_fromhex__doc__: __SlateAlign16<[i8; 189]> = __SlateAlign16([
    102i8, 114i8, 111i8, 109i8, 104i8, 101i8, 120i8, 40i8, 36i8, 116i8, 121i8, 112i8, 101i8, 44i8,
    32i8, 115i8, 116i8, 114i8, 105i8, 110i8, 103i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8,
    10i8, 67i8, 114i8, 101i8, 97i8, 116i8, 101i8, 32i8, 97i8, 32i8, 98i8, 121i8, 116i8, 101i8,
    115i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 32i8, 102i8, 114i8, 111i8, 109i8, 32i8,
    97i8, 32i8, 115i8, 116i8, 114i8, 105i8, 110i8, 103i8, 32i8, 111i8, 102i8, 32i8, 104i8, 101i8,
    120i8, 97i8, 100i8, 101i8, 99i8, 105i8, 109i8, 97i8, 108i8, 32i8, 110i8, 117i8, 109i8, 98i8,
    101i8, 114i8, 115i8, 46i8, 10i8, 10i8, 83i8, 112i8, 97i8, 99i8, 101i8, 115i8, 32i8, 98i8,
    101i8, 116i8, 119i8, 101i8, 101i8, 110i8, 32i8, 116i8, 119i8, 111i8, 32i8, 110i8, 117i8, 109i8,
    98i8, 101i8, 114i8, 115i8, 32i8, 97i8, 114i8, 101i8, 32i8, 97i8, 99i8, 99i8, 101i8, 112i8,
    116i8, 101i8, 100i8, 46i8, 10i8, 69i8, 120i8, 97i8, 109i8, 112i8, 108i8, 101i8, 58i8, 32i8,
    98i8, 121i8, 116i8, 101i8, 115i8, 46i8, 102i8, 114i8, 111i8, 109i8, 104i8, 101i8, 120i8, 40i8,
    39i8, 66i8, 57i8, 32i8, 48i8, 49i8, 69i8, 70i8, 39i8, 41i8, 32i8, 45i8, 62i8, 32i8, 98i8, 39i8,
    92i8, 92i8, 120i8, 98i8, 57i8, 92i8, 92i8, 120i8, 48i8, 49i8, 92i8, 92i8, 120i8, 101i8, 102i8,
    39i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bytesobject.bytes_fromhex")]
extern "C-unwind" fn bytes_fromhex(r#type: *mut _object, string: *mut _object) -> *mut _object {
    let mut return_value: *mut _object = core::ptr::null_mut::<_object>();
    return_value = bytes_fromhex_impl(r#type as *mut _typeobject, string);
    return_value
}

static bytes_hex__doc__: __SlateAlign16<[i8; 482]> = __SlateAlign16([
    104i8, 101i8, 120i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8, 47i8, 44i8, 32i8,
    115i8, 101i8, 112i8, 61i8, 60i8, 117i8, 110i8, 114i8, 101i8, 112i8, 114i8, 101i8, 115i8, 101i8,
    110i8, 116i8, 97i8, 98i8, 108i8, 101i8, 62i8, 44i8, 32i8, 98i8, 121i8, 116i8, 101i8, 115i8,
    95i8, 112i8, 101i8, 114i8, 95i8, 115i8, 101i8, 112i8, 61i8, 49i8, 41i8, 10i8, 45i8, 45i8, 10i8,
    10i8, 67i8, 114i8, 101i8, 97i8, 116i8, 101i8, 32i8, 97i8, 32i8, 115i8, 116i8, 114i8, 105i8,
    110i8, 103i8, 32i8, 111i8, 102i8, 32i8, 104i8, 101i8, 120i8, 97i8, 100i8, 101i8, 99i8, 105i8,
    109i8, 97i8, 108i8, 32i8, 110i8, 117i8, 109i8, 98i8, 101i8, 114i8, 115i8, 32i8, 102i8, 114i8,
    111i8, 109i8, 32i8, 97i8, 32i8, 98i8, 121i8, 116i8, 101i8, 115i8, 32i8, 111i8, 98i8, 106i8,
    101i8, 99i8, 116i8, 46i8, 10i8, 10i8, 32i8, 32i8, 115i8, 101i8, 112i8, 10i8, 32i8, 32i8, 32i8,
    32i8, 65i8, 110i8, 32i8, 111i8, 112i8, 116i8, 105i8, 111i8, 110i8, 97i8, 108i8, 32i8, 115i8,
    105i8, 110i8, 103i8, 108i8, 101i8, 32i8, 99i8, 104i8, 97i8, 114i8, 97i8, 99i8, 116i8, 101i8,
    114i8, 32i8, 111i8, 114i8, 32i8, 98i8, 121i8, 116i8, 101i8, 32i8, 116i8, 111i8, 32i8, 115i8,
    101i8, 112i8, 97i8, 114i8, 97i8, 116i8, 101i8, 32i8, 104i8, 101i8, 120i8, 32i8, 98i8, 121i8,
    116i8, 101i8, 115i8, 46i8, 10i8, 32i8, 32i8, 98i8, 121i8, 116i8, 101i8, 115i8, 95i8, 112i8,
    101i8, 114i8, 95i8, 115i8, 101i8, 112i8, 10i8, 32i8, 32i8, 32i8, 32i8, 72i8, 111i8, 119i8,
    32i8, 109i8, 97i8, 110i8, 121i8, 32i8, 98i8, 121i8, 116i8, 101i8, 115i8, 32i8, 98i8, 101i8,
    116i8, 119i8, 101i8, 101i8, 110i8, 32i8, 115i8, 101i8, 112i8, 97i8, 114i8, 97i8, 116i8, 111i8,
    114i8, 115i8, 46i8, 32i8, 32i8, 80i8, 111i8, 115i8, 105i8, 116i8, 105i8, 118i8, 101i8, 32i8,
    118i8, 97i8, 108i8, 117i8, 101i8, 115i8, 32i8, 99i8, 111i8, 117i8, 110i8, 116i8, 32i8, 102i8,
    114i8, 111i8, 109i8, 10i8, 32i8, 32i8, 32i8, 32i8, 116i8, 104i8, 101i8, 32i8, 114i8, 105i8,
    103i8, 104i8, 116i8, 44i8, 32i8, 110i8, 101i8, 103i8, 97i8, 116i8, 105i8, 118i8, 101i8, 32i8,
    118i8, 97i8, 108i8, 117i8, 101i8, 115i8, 32i8, 99i8, 111i8, 117i8, 110i8, 116i8, 32i8, 102i8,
    114i8, 111i8, 109i8, 32i8, 116i8, 104i8, 101i8, 32i8, 108i8, 101i8, 102i8, 116i8, 46i8, 10i8,
    10i8, 69i8, 120i8, 97i8, 109i8, 112i8, 108i8, 101i8, 58i8, 10i8, 62i8, 62i8, 62i8, 32i8, 118i8,
    97i8, 108i8, 117i8, 101i8, 32i8, 61i8, 32i8, 98i8, 39i8, 92i8, 120i8, 98i8, 57i8, 92i8, 120i8,
    48i8, 49i8, 92i8, 120i8, 101i8, 102i8, 39i8, 10i8, 62i8, 62i8, 62i8, 32i8, 118i8, 97i8, 108i8,
    117i8, 101i8, 46i8, 104i8, 101i8, 120i8, 40i8, 41i8, 10i8, 39i8, 98i8, 57i8, 48i8, 49i8, 101i8,
    102i8, 39i8, 10i8, 62i8, 62i8, 62i8, 32i8, 118i8, 97i8, 108i8, 117i8, 101i8, 46i8, 104i8,
    101i8, 120i8, 40i8, 39i8, 58i8, 39i8, 41i8, 10i8, 39i8, 98i8, 57i8, 58i8, 48i8, 49i8, 58i8,
    101i8, 102i8, 39i8, 10i8, 62i8, 62i8, 62i8, 32i8, 118i8, 97i8, 108i8, 117i8, 101i8, 46i8,
    104i8, 101i8, 120i8, 40i8, 39i8, 58i8, 39i8, 44i8, 32i8, 50i8, 41i8, 10i8, 39i8, 98i8, 57i8,
    58i8, 48i8, 49i8, 101i8, 102i8, 39i8, 10i8, 62i8, 62i8, 62i8, 32i8, 118i8, 97i8, 108i8, 117i8,
    101i8, 46i8, 104i8, 101i8, 120i8, 40i8, 39i8, 58i8, 39i8, 44i8, 32i8, 45i8, 50i8, 41i8, 10i8,
    39i8, 98i8, 57i8, 48i8, 49i8, 58i8, 101i8, 102i8, 39i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bytesobject.bytes_hex")]
extern "C-unwind" fn bytes_hex(
    self_: *mut _object,
    mut args: *const *mut _object,
    nargs: i64,
    kwnames: *mut _object,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut bytes_per_sep: i32 = unsafe { core::mem::zeroed() };
    let mut sep: *mut _object = unsafe { core::mem::zeroed() };
    let mut __v5: i64 = unsafe { core::mem::zeroed() };
    let mut noptargs: i64 = unsafe { core::mem::zeroed() };
    let mut argsbuf: core::mem::MaybeUninit<__SlateAlign16<[*mut _object; 2]>> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_7: *mut [*mut _object; 2] =
        core::ptr::addr_of_mut!(argsbuf) as *mut [*mut _object; 2];
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        if !kwnames.is_null() {
            __v5 = PyTuple_GET_SIZE!(kwnames);
        } else {
            __v5 = 0;
        }
        '__join_0: {
            noptargs = nargs.wrapping_add(__v5).wrapping_sub(0);
            sep = core::ptr::null_mut::<_object>();
            bytes_per_sep = 1;
            args = _PyArg_UnpackKeywords!(
                args,
                nargs,
                core::ptr::null_mut::<()>(),
                kwnames,
                core::ptr::addr_of_mut!(_parser__1289),
                0i32,
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
                '__join_1: {
                    if noptargs != 0 {
                        if !unsafe { *args.wrapping_offset(0 as isize) }.is_null() {
                            sep = *args.wrapping_offset(0 as isize);
                            let __v1: i64 = noptargs.wrapping_sub(1);
                            noptargs = __v1;
                            if __v1 == 0 {
                                break '__join_1;
                            }
                        }
                        bytes_per_sep = PyLong_AsInt(*args.wrapping_offset(1 as isize));
                        if bytes_per_sep == 1i32.wrapping_neg() {
                            __v0 = !PyErr_Occurred().is_null();
                        } else {
                            __v0 = false;
                        }
                        if __v0 {
                            break '__join_0;
                        }
                    }
                }
                return_value = bytes_hex_impl(
                    self_ as *mut __SlateRecord_bytesobject_h_15,
                    sep,
                    bytes_per_sep,
                );
            }
        }
        return return_value;
    }
}

static mut _kwtuple__1276: __SlateRecord_bytesobject_c_h_1276 = {
    let mut __t0: __SlateRecord_bytesobject_c_h_1276 = unsafe { core::mem::zeroed() };
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
                    ._py_sep
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
                    ._py_bytes_per_sep
                    ._ascii
                    .ob_base
            )
        },
    ];
    __t0
};

static _keywords__1288: __SlateAlign16<[*const i8; 3]> = __SlateAlign16([
    (b"sep\0".as_ptr() as *mut i8) as *const i8,
    (b"bytes_per_sep\0".as_ptr() as *mut i8) as *const i8,
    core::ptr::null::<i8>(),
]);

static mut _parser__1289: _PyArg_Parser = {
    let mut __t0: _PyArg_Parser = unsafe { core::mem::zeroed() };
    __t0.keywords = unsafe { core::ptr::addr_of!(_keywords__1288.0) as *const *const i8 };
    __t0.fname = (b"hex\0".as_ptr() as *mut i8) as *const i8;
    __t0.kwtuple = unsafe { core::ptr::addr_of_mut!(_kwtuple__1276.ob_base.ob_base) };
    __t0
};

#[unsafe(link_section = ".text.slate_distinct.bytesobject.bytes_new")]
extern "C-unwind" fn bytes_new(
    r#type: *mut _typeobject,
    args: *mut _object,
    kwargs: *mut _object,
) -> *mut _object {
    let mut errors_length: i64 = unsafe { core::mem::zeroed() };
    let mut encoding_length: i64 = unsafe { core::mem::zeroed() };
    let mut errors: *const i8 = unsafe { core::mem::zeroed() };
    let mut encoding: *const i8 = unsafe { core::mem::zeroed() };
    let mut x: *mut _object = unsafe { core::mem::zeroed() };
    let mut __v9: i64 = unsafe { core::mem::zeroed() };
    let mut noptargs: i64 = unsafe { core::mem::zeroed() };
    let mut nargs: i64 = unsafe { core::mem::zeroed() };
    let mut fastargs: *const *mut _object = unsafe { core::mem::zeroed() };
    let mut argsbuf: core::mem::MaybeUninit<__SlateAlign16<[*mut _object; 3]>> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_13: *mut [*mut _object; 3] =
        core::ptr::addr_of_mut!(argsbuf) as *mut [*mut _object; 3];
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        nargs = PyTuple_GET_SIZE!(args);
        if !kwargs.is_null() {
            __v9 = PyDict_GET_SIZE!(kwargs);
        } else {
            __v9 = 0;
        }
        '__join_0: {
            noptargs = nargs.wrapping_add(__v9).wrapping_sub(0);
            x = core::ptr::null_mut::<_object>();
            encoding = core::ptr::null::<i8>();
            errors = core::ptr::null::<i8>();
            fastargs = _PyArg_UnpackKeywords__2!(
                (*(args as *mut __SlateRecord_tupleobject_h_13))
                    .ob_item
                    .as_mut_ptr() as *mut *mut _object,
                nargs,
                kwargs,
                core::ptr::null_mut::<()>(),
                core::ptr::addr_of_mut!(_parser__1354),
                0i32,
                3i32,
                0i32,
                0i32,
                (*__slate_slot_13).as_mut_ptr() as *mut *mut _object
            );
            // minpos
            // maxpos
            // minkw
            // varpos
            if !fastargs.is_null() {
                '__join_1: {
                    if noptargs != 0 {
                        if !unsafe { *fastargs.wrapping_offset(0 as isize) }.is_null() {
                            x = *fastargs.wrapping_offset(0 as isize);
                            let __v4: i64 = noptargs.wrapping_sub(1);
                            noptargs = __v4;
                            if __v4 == 0 {
                                break '__join_1;
                            }
                        }
                        if !unsafe { *fastargs.wrapping_offset(1 as isize) }.is_null() {
                            if PyUnicode_Check!(unsafe { *fastargs.wrapping_offset(1 as isize) })
                                == 0
                            {
                                _PyArg_BadArgument(
                                    c"bytes".as_ptr(),
                                    c"argument 'encoding'".as_ptr(),
                                    c"str".as_ptr(),
                                    *fastargs.wrapping_offset(1 as isize),
                                );
                                break '__join_0;
                            } else {
                                encoding = PyUnicode_AsUTF8AndSize(
                                    *fastargs.wrapping_offset(1 as isize),
                                    core::ptr::addr_of_mut!(encoding_length),
                                );
                                if encoding.is_null() {
                                    break '__join_0;
                                } else {
                                    if (strlen(encoding as *const core::ffi::c_char) as u64)
                                        != (encoding_length as u64)
                                    {
                                        PyErr_SetString(
                                            PyExc_ValueError,
                                            c"embedded null character".as_ptr(),
                                        );
                                        break '__join_0;
                                    } else {
                                        let __v1: i64 = noptargs.wrapping_sub(1);
                                        noptargs = __v1;
                                        if __v1 == 0 {
                                            break '__join_1;
                                        }
                                    }
                                }
                            }
                        }
                        if PyUnicode_Check!(unsafe { *fastargs.wrapping_offset(2 as isize) }) == 0 {
                            _PyArg_BadArgument(
                                c"bytes".as_ptr(),
                                c"argument 'errors'".as_ptr(),
                                c"str".as_ptr(),
                                *fastargs.wrapping_offset(2 as isize),
                            );
                            break '__join_0;
                        } else {
                            errors = PyUnicode_AsUTF8AndSize(
                                *fastargs.wrapping_offset(2 as isize),
                                core::ptr::addr_of_mut!(errors_length),
                            );
                            if errors.is_null() {
                                break '__join_0;
                            } else {
                                if (strlen(errors as *const core::ffi::c_char) as u64)
                                    != (errors_length as u64)
                                {
                                    PyErr_SetString(
                                        PyExc_ValueError,
                                        c"embedded null character".as_ptr(),
                                    );
                                    break '__join_0;
                                }
                            }
                        }
                    }
                }
                return_value = bytes_new_impl(r#type, x, encoding, errors);
            }
        }
        return return_value;
    }
}

static mut _kwtuple__1341: __SlateRecord_bytesobject_c_h_1341 = {
    let mut __t0: __SlateRecord_bytesobject_c_h_1341 = unsafe { core::mem::zeroed() };
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
                    ._py_source
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
                    ._py_encoding
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
                    ._py_errors
                    ._ascii
                    .ob_base
            )
        },
    ];
    __t0
};

static _keywords__1353: __SlateAlign16<[*const i8; 4]> = __SlateAlign16([
    (b"source\0".as_ptr() as *mut i8) as *const i8,
    (b"encoding\0".as_ptr() as *mut i8) as *const i8,
    (b"errors\0".as_ptr() as *mut i8) as *const i8,
    core::ptr::null::<i8>(),
]);

static mut _parser__1354: _PyArg_Parser = {
    let mut __t0: _PyArg_Parser = unsafe { core::mem::zeroed() };
    __t0.keywords = unsafe { core::ptr::addr_of!(_keywords__1353.0) as *const *const i8 };
    __t0.fname = (b"bytes\0".as_ptr() as *mut i8) as *const i8;
    __t0.kwtuple = unsafe { core::ptr::addr_of_mut!(_kwtuple__1341.ob_base.ob_base) };
    __t0
};

// _PyIndex_Check()
// _Py_bytes_startswith()
// _PyBytes_Find(), _PyBytes_Repeat()
// _PyObject_CallNoArgs()
// _PyEval_GetBuiltin()
// F_LJUST
// _Py_GET_GLOBAL_OBJECT()
// _PyStatus_OK()
// _PyLong_DigitValue
// _PyList_GetItemRef
// _PyObject_GC_TRACK
// PYMEM_CLEANBYTE
// _Py_strhex_with_sep()
// _PyUnicode_FormatLong()
// [clinic input]
// class bytes "PyBytesObject *" "&PyBytes_Type"
// [clinic start generated code]
//
// [clinic end generated code: output=da39a3ee5e6b4b0d input=7a238f965d64892b]
// PyBytesObject_SIZE gives the basic size of a bytes object; any memory allocation
// for a bytes object of length n should request PyBytesObject_SIZE + n bytes.
//
// Using PyBytesObject_SIZE instead of sizeof(PyBytesObject) saves
// 3 or 7 bytes per bytes object allocation on a typical system.
/// Return a reference to the immortal empty bytes string singleton.
fn bytes_get_empty() -> *mut _object {
    let empty: *mut _object = unsafe {
        core::ptr::addr_of_mut!(
            (*core::ptr::addr_of_mut!(_PyRuntime.static_objects.singletons.bytes_empty))
                .ob_base
                .ob_base
        )
    };
    empty
}

fn set_ob_shash(a: *mut __SlateRecord_bytesobject_h_15, hash: i64) {
    unsafe {
        (*a).ob_shash = hash;
    }
}

fn get_ob_shash(a: *mut __SlateRecord_bytesobject_h_15) -> i64 {
    unsafe { (*a).ob_shash }
}

/// For PyBytes_FromString(), the parameter 'str' points to a null-terminated
/// string containing exactly 'size' bytes.
///
/// For PyBytes_FromStringAndSize(), the parameter 'str' is
/// either NULL or else points to a string containing at least 'size' bytes.
/// For PyBytes_FromStringAndSize(), the string in the 'str' parameter does
/// not have to be null-terminated.  (Therefore it is safe to construct a
/// substring by calling 'PyBytes_FromStringAndSize(origstring, substrlen)'.)
/// If 'str' is NULL then PyBytes_FromStringAndSize() will allocate 'size+1'
/// bytes (setting the last byte to the null terminating character) and you can
/// fill in the data yourself.  If 'str' is non-NULL then the resulting
/// PyBytes object must be treated as immutable and you must not fill in nor
/// alter the data yourself, since the strings may be shared.
///
/// The PyObject member 'op->ob_size', which denotes the number of "extra
/// items" in a variable-size object, will contain the number of bytes
/// allocated for string data, not counting the null terminating character.
/// It is therefore equal to the 'size' parameter (for
/// PyBytes_FromStringAndSize()) or the length of the string in the 'str'
/// parameter (for PyBytes_FromString()).
fn _PyBytes_FromSize(size: i64, use_calloc: i32) -> *mut _object {
    let mut op: *mut __SlateRecord_bytesobject_h_15 = unsafe { core::mem::zeroed() };
    if size == 0 {
        return bytes_get_empty();
    }
    if (size as u64)
        > 9223372036854775807u64.wrapping_sub(
            (core::mem::offset_of!(__SlateRecord_bytesobject_h_15, ob_sval) as u64).wrapping_add(1),
        )
    {
        unsafe { PyErr_SetString(PyExc_OverflowError, c"byte string is too large".as_ptr()) };
        return core::ptr::null_mut::<_object>();
    }
    // Inline PyObject_NewVar
    if use_calloc != 0 {
        op = (unsafe {
            PyObject_Calloc(
                1u64,
                (core::mem::offset_of!(__SlateRecord_bytesobject_h_15, ob_sval) as u64)
                    .wrapping_add(1)
                    .wrapping_add(size as u64),
            )
        }) as *mut __SlateRecord_bytesobject_h_15;
    } else {
        op = (unsafe {
            PyObject_Malloc(
                (core::mem::offset_of!(__SlateRecord_bytesobject_h_15, ob_sval) as u64)
                    .wrapping_add(1)
                    .wrapping_add(size as u64),
            )
        }) as *mut __SlateRecord_bytesobject_h_15;
    }
    if op.is_null() {
        return unsafe { PyErr_NoMemory() };
    }
    _PyObject_InitVar(
        op as *mut __SlateRecord_object_h_172,
        unsafe { core::ptr::addr_of_mut!(PyBytes_Type) },
        size,
    );
    set_ob_shash(op, 1i32.wrapping_neg() as i64);
    if use_calloc == 0 {
        unsafe {
            *((*op).ob_sval.as_mut_ptr() as *mut i8).wrapping_offset(size as isize) = 0;
        }
    }
    op as *mut _object
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyBytes_FromStringAndSize(str: *const i8, size: i64) -> *mut _object {
    let mut op: *mut __SlateRecord_bytesobject_h_15 = unsafe { core::mem::zeroed() };
    if size < 0 {
        unsafe {
            PyErr_SetString(
                PyExc_SystemError,
                c"Negative size passed to PyBytes_FromStringAndSize".as_ptr(),
            )
        };
        return core::ptr::null_mut::<_object>();
    }
    if size == 1 && !str.is_null() {
        op = unsafe {
            core::ptr::addr_of_mut!(_PyRuntime.static_objects.singletons.bytes_characters)
                as *mut __SlateRecord_pycore_runtime_structs_h_124
        }
        .wrapping_offset((((unsafe { *str }) as i32) & 255) as isize)
            as *mut __SlateRecord_bytesobject_h_15;
        return op as *mut _object;
    }
    if size == 0 {
        return bytes_get_empty();
    }
    op = _PyBytes_FromSize(size, 0i32) as *mut __SlateRecord_bytesobject_h_15;
    if op.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    if str.is_null() {
        return op as *mut _object;
    }
    unsafe {
        core::ptr::copy_nonoverlapping(
            ((str as *const ()) as *const core::ffi::c_void) as *const u8,
            ((((*op).ob_sval.as_mut_ptr() as *mut i8) as *mut ()) as *mut core::ffi::c_void)
                as *mut u8,
            ((size as u64) as usize) as usize,
        )
    };
    op as *mut _object
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyBytes_FromString(str: *const i8) -> *mut _object {
    let mut size: u64 = 0;
    let mut op: *mut __SlateRecord_bytesobject_h_15 = unsafe { core::mem::zeroed() };
    size = unsafe { strlen(str as *const core::ffi::c_char) as u64 };
    if size
        > 9223372036854775807u64.wrapping_sub(
            (core::mem::offset_of!(__SlateRecord_bytesobject_h_15, ob_sval) as u64).wrapping_add(1),
        )
    {
        unsafe { PyErr_SetString(PyExc_OverflowError, c"byte string is too long".as_ptr()) };
        return core::ptr::null_mut::<_object>();
    }
    if size == 0 {
        return bytes_get_empty();
    } else {
        if size == 1 {
            op = unsafe {
                core::ptr::addr_of_mut!(_PyRuntime.static_objects.singletons.bytes_characters)
                    as *mut __SlateRecord_pycore_runtime_structs_h_124
            }
            .wrapping_offset((((unsafe { *str }) as i32) & 255) as isize)
                as *mut __SlateRecord_bytesobject_h_15;
            return op as *mut _object;
        }
    }
    // Inline PyObject_NewVar
    op = (unsafe {
        PyObject_Malloc(
            (core::mem::offset_of!(__SlateRecord_bytesobject_h_15, ob_sval) as u64)
                .wrapping_add(1)
                .wrapping_add(size),
        )
    }) as *mut __SlateRecord_bytesobject_h_15;
    if op.is_null() {
        return unsafe { PyErr_NoMemory() };
    }
    _PyObject_InitVar(
        op as *mut __SlateRecord_object_h_172,
        unsafe { core::ptr::addr_of_mut!(PyBytes_Type) },
        size as i64,
    );
    set_ob_shash(op, 1i32.wrapping_neg() as i64);
    unsafe {
        core::ptr::copy_nonoverlapping(
            ((str as *const ()) as *const core::ffi::c_void) as *const u8,
            ((((*op).ob_sval.as_mut_ptr() as *mut i8) as *mut ()) as *mut core::ffi::c_void)
                as *mut u8,
            (size.wrapping_add(1) as usize) as usize,
        )
    };
    op as *mut _object
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyBytes_FromFormatV(
    format: *const i8,
    mut vargs: core::ffi::VaList<'static>,
) -> *mut _object {
    let mut i: i64 = unsafe { core::mem::zeroed() };
    let mut c: i32 = unsafe { core::mem::zeroed() };
    let mut writer: core::mem::MaybeUninit<__SlateRecord_pycore_bytesobject_h_89> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_34: *mut __SlateRecord_pycore_bytesobject_h_89 =
        core::ptr::addr_of_mut!(writer) as *mut __SlateRecord_pycore_bytesobject_h_89;
    // Longest 64-bit formatted numbers:
    // - "18446744073709551615\0" (21 bytes)
    // - "-9223372036854775808\0" (21 bytes)
    // Decimal takes the most space (it isn't enough for octal.)
    //
    // Longest 64-bit pointer representation:
    // "0xffffffffffffffff\0" (19 bytes).
    let mut buffer: core::mem::MaybeUninit<__SlateAlign16<[i8; 21]>> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_35: *mut [i8; 21] = core::ptr::addr_of_mut!(buffer) as *mut [i8; 21];
    let mut size_tflag: i32 = unsafe { core::mem::zeroed() };
    let mut longflag: i32 = unsafe { core::mem::zeroed() };
    let mut prec: i64 = unsafe { core::mem::zeroed() };
    let mut p: *const i8 = unsafe { core::mem::zeroed() };
    let mut f: *const i8 = unsafe { core::mem::zeroed() };
    let mut s: *mut i8 = unsafe { core::mem::zeroed() };
    unsafe {
        _PyBytesWriter_Init(core::ptr::addr_of_mut!(*__slate_slot_34));
        s = _PyBytesWriter_Alloc(
            core::ptr::addr_of_mut!(*__slate_slot_34),
            (strlen(format as *const core::ffi::c_char) as u64) as i64,
        ) as *mut i8;
        if s.is_null() {
            return core::ptr::null_mut::<_object>();
        } else {
            (*__slate_slot_34).overallocate = 1;
            f = format;
            '__join_0: {
                '__join_46: {
                    '__join_9: {
                        while *f != 0 {
                            if (*f as i32) != 37 {
                                let __v33: *mut i8 = s;
                                s = __v33.wrapping_offset(1 as isize);
                                *__v33 = *f;
                            } else {
                                let __v31: *const i8 = f;
                                f = __v31.wrapping_offset(1 as isize);
                                p = __v31;
                                // ignore the width (ex: 10 in "%10s")
                                while Py_ISDIGIT__3!(unsafe { *f }) != 0 {
                                    let __v29: *const i8 = f;
                                    f = __v29.wrapping_offset(1 as isize);
                                }
                                // parse the precision (ex: 10 in "%.10s")
                                prec = 0;
                                if (*f as i32) == 46 {
                                    let __v27: *const i8 = f;
                                    f = __v27.wrapping_offset(1 as isize);
                                    while Py_ISDIGIT__3!(unsafe { *f }) != 0 {
                                        prec = prec
                                            .wrapping_mul(10)
                                            .wrapping_add((*f as i32).wrapping_sub(48) as i64);
                                        let __v25: *const i8 = f;
                                        f = __v25.wrapping_offset(1 as isize);
                                    }
                                }
                                while *f != 0
                                    && (*f as i32) != 37
                                    && Py_ISALPHA!(unsafe { *f }) == 0
                                {
                                    let __v23: *const i8 = f;
                                    f = __v23.wrapping_offset(1 as isize);
                                }
                                // handle the long flag ('l'), but only for %ld and %lu.
                                // others can be added when necessary.
                                longflag = 0;
                                if (*f as i32) == 108
                                    && ((*f.wrapping_offset(1 as isize) as i32) == 100
                                        || (*f.wrapping_offset(1 as isize) as i32) == 117)
                                {
                                    longflag = 1;
                                    let __v21: *const i8 = f;
                                    f = __v21.wrapping_offset(1 as isize);
                                }
                                // handle the size_t flag ('z').
                                size_tflag = 0;
                                if (*f as i32) == 122
                                    && ((*f.wrapping_offset(1 as isize) as i32) == 100
                                        || (*f.wrapping_offset(1 as isize) as i32) == 117)
                                {
                                    size_tflag = 1;
                                    let __v19: *const i8 = f;
                                    f = __v19.wrapping_offset(1 as isize);
                                }
                                // subtract bytes preallocated for the format string
                                // (ex: 2 for "%s")
                                let __v17: i64 = (*__slate_slot_34).min_size;
                                let __v16: i64 = __v17.wrapping_sub(
                                    (f.offset_from(p as *const i8) as i64).wrapping_add(1),
                                );
                                (*__slate_slot_34).min_size = __v16;
                                let __t0: i32 = *f as i32;
                                if __t0 == 99 {
                                    c = vargs.next_arg::<i32>();
                                    if c < 0 || c > 255 {
                                        break '__join_46;
                                    } else {
                                        let __v14: i64 = (*__slate_slot_34).min_size;
                                        (*__slate_slot_34).min_size = __v14.wrapping_add(1);
                                        let __v12: *mut i8 = s;
                                        s = __v12.wrapping_offset(1 as isize);
                                        *__v12 = (c as u8) as i8;
                                    }
                                } else {
                                    if __t0 == 100 {
                                        if longflag != 0 {
                                            sprintf(
                                                ((*__slate_slot_35).as_mut_ptr() as *mut i8)
                                                    as *mut core::ffi::c_char,
                                                c"%ld".as_ptr() as *const core::ffi::c_char,
                                                vargs.next_arg::<i64>(),
                                            ) as i32;
                                        } else {
                                            if size_tflag != 0 {
                                                sprintf(
                                                    ((*__slate_slot_35).as_mut_ptr() as *mut i8)
                                                        as *mut core::ffi::c_char,
                                                    c"%zd".as_ptr() as *const core::ffi::c_char,
                                                    vargs.next_arg::<i64>(),
                                                )
                                                    as i32;
                                            } else {
                                                sprintf(
                                                    ((*__slate_slot_35).as_mut_ptr() as *mut i8)
                                                        as *mut core::ffi::c_char,
                                                    c"%d".as_ptr() as *const core::ffi::c_char,
                                                    vargs.next_arg::<i32>(),
                                                )
                                                    as i32;
                                            }
                                        }
                                        loop {
                                            s = _PyBytesWriter_WriteBytes(
                                                core::ptr::addr_of_mut!(*__slate_slot_34),
                                                s as *mut (),
                                                (*__slate_slot_35).as_mut_ptr() as *const (),
                                                (strlen(
                                                    ((*__slate_slot_35).as_mut_ptr() as *const i8)
                                                        as *const core::ffi::c_char,
                                                )
                                                    as u64)
                                                    as i64,
                                            )
                                                as *mut i8;
                                            if s.is_null() {
                                                break '__join_0;
                                            } else {
                                                if true {
                                                    break;
                                                }
                                            }
                                        }
                                    } else {
                                        if __t0 == 117 {
                                            if longflag != 0 {
                                                sprintf(
                                                    ((*__slate_slot_35).as_mut_ptr() as *mut i8)
                                                        as *mut core::ffi::c_char,
                                                    c"%lu".as_ptr() as *const core::ffi::c_char,
                                                    vargs.next_arg::<u64>(),
                                                )
                                                    as i32;
                                            } else {
                                                if size_tflag != 0 {
                                                    sprintf(
                                                        ((*__slate_slot_35).as_mut_ptr() as *mut i8)
                                                            as *mut core::ffi::c_char,
                                                        c"%zu".as_ptr() as *const core::ffi::c_char,
                                                        vargs.next_arg::<u64>(),
                                                    )
                                                        as i32;
                                                } else {
                                                    sprintf(
                                                        ((*__slate_slot_35).as_mut_ptr() as *mut i8)
                                                            as *mut core::ffi::c_char,
                                                        c"%u".as_ptr() as *const core::ffi::c_char,
                                                        vargs.next_arg::<u32>(),
                                                    )
                                                        as i32;
                                                }
                                            }
                                            loop {
                                                s = _PyBytesWriter_WriteBytes(
                                                    core::ptr::addr_of_mut!(*__slate_slot_34),
                                                    s as *mut (),
                                                    (*__slate_slot_35).as_mut_ptr() as *const (),
                                                    (strlen(
                                                        ((*__slate_slot_35).as_mut_ptr()
                                                            as *const i8)
                                                            as *const core::ffi::c_char,
                                                    )
                                                        as u64)
                                                        as i64,
                                                )
                                                    as *mut i8;
                                                if s.is_null() {
                                                    break '__join_0;
                                                } else {
                                                    if true {
                                                        break;
                                                    }
                                                }
                                            }
                                        } else {
                                            if __t0 == 105 {
                                                sprintf(
                                                    ((*__slate_slot_35).as_mut_ptr() as *mut i8)
                                                        as *mut core::ffi::c_char,
                                                    c"%i".as_ptr() as *const core::ffi::c_char,
                                                    vargs.next_arg::<i32>(),
                                                )
                                                    as i32;
                                                loop {
                                                    s = _PyBytesWriter_WriteBytes(
                                                        core::ptr::addr_of_mut!(*__slate_slot_34),
                                                        s as *mut (),
                                                        (*__slate_slot_35).as_mut_ptr()
                                                            as *const (),
                                                        (strlen(
                                                            ((*__slate_slot_35).as_mut_ptr()
                                                                as *const i8)
                                                                as *const core::ffi::c_char,
                                                        )
                                                            as u64)
                                                            as i64,
                                                    )
                                                        as *mut i8;
                                                    if s.is_null() {
                                                        break '__join_0;
                                                    } else {
                                                        if true {
                                                            break;
                                                        }
                                                    }
                                                }
                                            } else {
                                                if __t0 == 120 {
                                                    sprintf(
                                                        ((*__slate_slot_35).as_mut_ptr() as *mut i8)
                                                            as *mut core::ffi::c_char,
                                                        c"%x".as_ptr() as *const core::ffi::c_char,
                                                        vargs.next_arg::<i32>(),
                                                    )
                                                        as i32;
                                                    loop {
                                                        s = _PyBytesWriter_WriteBytes(
                                                            core::ptr::addr_of_mut!(
                                                                *__slate_slot_34
                                                            ),
                                                            s as *mut (),
                                                            (*__slate_slot_35).as_mut_ptr()
                                                                as *const (),
                                                            (strlen(
                                                                ((*__slate_slot_35).as_mut_ptr()
                                                                    as *const i8)
                                                                    as *const core::ffi::c_char,
                                                            )
                                                                as u64)
                                                                as i64,
                                                        )
                                                            as *mut i8;
                                                        if s.is_null() {
                                                            break '__join_0;
                                                        } else {
                                                            if true {
                                                                break;
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if __t0 == 115 {
                                                        p = vargs.next_arg::<*const i8>();
                                                        if prec <= 0 {
                                                            i = (strlen(
                                                                p as *const core::ffi::c_char,
                                                            )
                                                                as u64)
                                                                as i64;
                                                        } else {
                                                            i = 0;
                                                            while i < prec
                                                                && *p.wrapping_offset(i as isize)
                                                                    != 0
                                                            {
                                                                i = i.wrapping_add(1);
                                                            }
                                                        }
                                                        s = _PyBytesWriter_WriteBytes(
                                                            core::ptr::addr_of_mut!(
                                                                *__slate_slot_34
                                                            ),
                                                            s as *mut (),
                                                            p as *const (),
                                                            i,
                                                        )
                                                            as *mut i8;
                                                        if s.is_null() {
                                                            break '__join_0;
                                                        }
                                                    } else {
                                                        if __t0 == 112 {
                                                            sprintf(
                                                                ((*__slate_slot_35).as_mut_ptr()
                                                                    as *mut i8)
                                                                    as *mut core::ffi::c_char,
                                                                c"%p".as_ptr()
                                                                    as *const core::ffi::c_char,
                                                                vargs.next_arg::<*mut ()>(),
                                                            )
                                                                as i32;
                                                            // %p is ill-defined:  ensure leading 0x.
                                                            if ((*__slate_slot_35)[1] as i32) == 88
                                                            {
                                                                (*__slate_slot_35)[1] = 120;
                                                            } else {
                                                                if ((*__slate_slot_35)[1] as i32)
                                                                    != 120
                                                                {
                                                                    core::ptr::copy((((*__slate_slot_35).as_mut_ptr() as *const ()) as *const core::ffi::c_void) as *const u8, ((((*__slate_slot_35).as_mut_ptr() as *mut i8).wrapping_offset(2 as isize) as *mut ()) as *mut core::ffi::c_void) as *mut u8, ((strlen(((*__slate_slot_35).as_mut_ptr() as *const i8) as *const core::ffi::c_char) as u64).wrapping_add(1) as usize) as usize);
                                                                    (*__slate_slot_35)[0] = 48;
                                                                    (*__slate_slot_35)[1] = 120;
                                                                }
                                                            }
                                                            loop {
                                                                s = _PyBytesWriter_WriteBytes(core::ptr::addr_of_mut!(*__slate_slot_34), s as *mut (), (*__slate_slot_35).as_mut_ptr() as *const (), (strlen(((*__slate_slot_35).as_mut_ptr() as *const i8) as *const core::ffi::c_char) as u64) as i64) as *mut i8;
                                                                if s.is_null() {
                                                                    break '__join_0;
                                                                } else {
                                                                    if true {
                                                                        break;
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            if __t0 == 37 {
                                                                let __v7: i64 =
                                                                    (*__slate_slot_34).min_size;
                                                                (*__slate_slot_34).min_size =
                                                                    __v7.wrapping_add(1);
                                                                let __v5: *mut i8 = s;
                                                                s = __v5
                                                                    .wrapping_offset(1 as isize);
                                                                *__v5 = 37;
                                                            } else {
                                                                break '__join_9;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            let __v1: *const i8 = f;
                            f = __v1.wrapping_offset(1 as isize);
                        }
                        return _PyBytesWriter_Finish(
                            core::ptr::addr_of_mut!(*__slate_slot_34),
                            s as *mut (),
                        );
                    }
                    if (*f as i32) == 0 {
                        // fix min_size if we reached the end of the format string
                        let __v3: i64 = (*__slate_slot_34).min_size;
                        (*__slate_slot_34).min_size = __v3.wrapping_add(1);
                    }
                    // invalid format string: copy unformatted string and exit
                    loop {
                        s = _PyBytesWriter_WriteBytes(
                            core::ptr::addr_of_mut!(*__slate_slot_34),
                            s as *mut (),
                            p as *const (),
                            (strlen(p as *const core::ffi::c_char) as u64) as i64,
                        ) as *mut i8;
                        if s.is_null() {
                            break '__join_0;
                        } else {
                            if true {
                                break;
                            }
                        }
                    }
                    return _PyBytesWriter_Finish(
                        core::ptr::addr_of_mut!(*__slate_slot_34),
                        s as *mut (),
                    );
                }
                PyErr_SetString(
                    PyExc_OverflowError,
                    c"PyBytes_FromFormatV(): %c format expects an integer in range [0; 255]"
                        .as_ptr(),
                );
            }
            _PyBytesWriter_Dealloc(core::ptr::addr_of_mut!(*__slate_slot_34));
            return core::ptr::null_mut::<_object>();
        }
    }
}

#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn PyBytes_FromFormat(
    format: *const i8,
    mut __va_args: ...
) -> *mut _object {
    let mut vargs: core::ffi::VaList<'static> = unsafe { core::mem::zeroed() };
    vargs = unsafe {
        core::mem::transmute::<core::ffi::VaList<'_>, core::ffi::VaList<'static>>(__va_args.clone())
    };
    let ret: *mut _object = PyBytes_FromFormatV(format, vargs.clone());
    ret
}

// Helpers for formatstring
fn getnextarg(args: *mut _object, arglen: i64, p_argidx: &mut i64) -> *mut _object {
    let argidx: i64 = unsafe { *(p_argidx as *mut i64) };
    if argidx < arglen {
        let __v1: i64 = unsafe { *p_argidx };
        unsafe {
            *p_argidx = __v1.wrapping_add(1);
        }
        if arglen < 0 {
            return args;
        } else {
            return unsafe { PyTuple_GetItem(args, argidx) };
        }
    }
    unsafe {
        PyErr_SetString(
            PyExc_TypeError,
            c"not enough arguments for format string".as_ptr(),
        )
    };
    core::ptr::null_mut::<_object>()
}

// Returns a new reference to a PyBytes object, or NULL on failure.
fn formatfloat(
    v: *mut _object,
    flags: i32,
    mut prec: i32,
    r#type: i32,
    p_result: *mut *mut _object,
    writer: *mut __SlateRecord_pycore_bytesobject_h_89,
    mut str: *mut i8,
) -> *mut i8 {
    let x: f64 = unsafe { PyFloat_AsDouble(v) };
    let mut len: u64 = 0;
    let mut dtoa_flags: i32 = 0;
    let __v0: bool = x == -1.0f64 && !unsafe { PyErr_Occurred() }.is_null();
    if __v0 {
        unsafe {
            PyErr_Format(
                PyExc_TypeError,
                c"float argument required, not %.200s".as_ptr(),
                (*Py_TYPE!(v)).tp_name,
            )
        };
        return core::ptr::null_mut::<i8>();
    }
    if prec < 0 {
        prec = 6;
    }
    if flags & 8i32 != 0 {
        dtoa_flags |= 4;
    }
    let p: *mut i8 = unsafe {
        PyOS_double_to_string(
            x,
            r#type as i8,
            prec,
            dtoa_flags,
            core::ptr::null_mut::<i32>(),
        )
    };
    if p.is_null() {
        return core::ptr::null_mut::<i8>();
    }
    len = unsafe { strlen((p as *const i8) as *const core::ffi::c_char) as u64 };
    if !writer.is_null() {
        str = _PyBytesWriter_Prepare(writer, str as *mut (), len as i64) as *mut i8;
        if str.is_null() {
            unsafe { PyMem_Free(p as *mut ()) };
            return core::ptr::null_mut::<i8>();
        }
        unsafe {
            core::ptr::copy_nonoverlapping(
                ((p as *const ()) as *const core::ffi::c_void) as *const u8,
                ((str as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                (len as usize) as usize,
            )
        };
        unsafe { PyMem_Free(p as *mut ()) };
        let __v3: *mut i8 = str;
        str = __v3.wrapping_offset(len as isize);
        return str;
    }
    let result: *mut _object = PyBytes_FromStringAndSize(p as *const i8, len as i64);
    unsafe { PyMem_Free(p as *mut ()) };
    unsafe {
        *p_result = result;
    }
    if !result.is_null() {
        str
    } else {
        core::ptr::null_mut::<i8>()
    }
}

fn formatlong(v: *mut _object, flags: i32, prec: i32, r#type: i32) -> *mut _object {
    let mut iobj: *mut _object = unsafe { core::mem::zeroed() };
    if PyLong_Check!(v) != 0 {
        return unsafe { _PyUnicode_FormatLong(v, flags & 8i32, prec, r#type) };
    }
    if (unsafe { PyNumber_Check(v) }) != 0 {
        // make sure number is a type of integer for o, x, and X
        if r#type == 111 || r#type == 120 || r#type == 88 {
            iobj = unsafe { _PyNumber_Index(v) };
        } else {
            iobj = unsafe { PyNumber_Long(v) };
        }
        if !iobj.is_null() {
            let result: *mut _object =
                unsafe { _PyUnicode_FormatLong(iobj, flags & 8i32, prec, r#type) };
            Py_DECREF!(iobj);
            return result;
        }
        if (unsafe { PyErr_ExceptionMatches(PyExc_TypeError) }) == 0 {
            return core::ptr::null_mut::<_object>();
        }
    }
    unsafe {
        PyErr_Format(
            PyExc_TypeError,
            c"%%%c format: %s is required, not %.200s".as_ptr(),
            r#type,
            if r#type == 111 || r#type == 120 || r#type == 88 {
                c"an integer".as_ptr() as *mut i8
            } else {
                c"a real number".as_ptr() as *mut i8
            },
            (*Py_TYPE!(v)).tp_name,
        )
    };
    core::ptr::null_mut::<_object>()
}

fn byte_converter(arg: *mut _object, p: &mut i8) -> i32 {
    if PyBytes_Check!(arg) != 0 {
        if PyBytes_GET_SIZE!(arg) != 1 {
            unsafe {
                PyErr_Format(PyExc_TypeError, c"%%c requires an integer in range(256) or a single byte, not a bytes object of length %zd".as_ptr(), PyBytes_GET_SIZE!(arg))
            };
            return 0;
        }
        unsafe {
            *(p as *mut i8) = *PyBytes_AS_STRING!(arg).wrapping_offset(0 as isize);
        }
        return 1;
    } else {
        if PyByteArray_Check!(arg) != 0 {
            if PyByteArray_GET_SIZE!(arg) != 1 {
                unsafe {
                    PyErr_Format(PyExc_TypeError, c"%%c requires an integer in range(256) or a single byte, not a bytearray object of length %zd".as_ptr(), PyByteArray_GET_SIZE!(arg))
                };
                return 0;
            }
            unsafe {
                *(p as *mut i8) = *PyByteArray_AS_STRING__2!(arg).wrapping_offset(0 as isize);
            }
            return 1;
        } else {
            if (unsafe { PyIndex_Check(arg) }) != 0 {
                let mut overflow: i32 = 0;
                let ival: i64 =
                    unsafe { PyLong_AsLongAndOverflow(arg, core::ptr::addr_of_mut!(overflow)) };
                let __v0: bool =
                    ival == (1i32.wrapping_neg() as i64) && !unsafe { PyErr_Occurred() }.is_null();
                if __v0 {
                    return 0;
                }
                if !(0 <= ival && ival <= 255) {
                    // this includes an overflow in converting to C long
                    unsafe {
                        PyErr_SetString(PyExc_OverflowError, c"%c arg not in range(256)".as_ptr())
                    };
                    return 0;
                }
                unsafe {
                    *(p as *mut i8) = ival as i8;
                }
                return 1;
            }
        }
    }
    unsafe {
        PyErr_Format(
            PyExc_TypeError,
            c"%%c requires an integer in range(256) or a single byte, not %T".as_ptr(),
            arg,
        )
    };
    0
}

fn format_obj(v: *mut _object, pbuf: &mut *const i8, plen: &mut i64) -> *mut _object {
    let mut result: *mut _object = unsafe { core::mem::zeroed() };
    // is it a bytes object?
    if PyBytes_Check!(v) != 0 {
        unsafe {
            *(pbuf as *mut *const i8) = PyBytes_AS_STRING!(v) as *const i8;
        }
        unsafe {
            *(plen as *mut i64) = PyBytes_GET_SIZE!(v);
        }
        return Py_NewRef!(v);
    }
    if PyByteArray_Check!(v) != 0 {
        unsafe {
            *(pbuf as *mut *const i8) = PyByteArray_AS_STRING__2!(v) as *const i8;
        }
        unsafe {
            *(plen as *mut i64) = PyByteArray_GET_SIZE!(v);
        }
        return Py_NewRef!(v);
    }
    // does it support __bytes__?
    let func: *mut _object = unsafe {
        _PyObject_LookupSpecial(
            v,
            core::ptr::addr_of_mut!(
                _PyRuntime
                    .static_objects
                    .singletons
                    .strings
                    .identifiers
                    ._py___bytes__
                    ._ascii
                    .ob_base
            ),
        )
    };
    if !func.is_null() {
        result = _PyObject_CallNoArgs(func);
        Py_DECREF!(func);
        if result.is_null() {
            return core::ptr::null_mut::<_object>();
        }
        if PyBytes_Check!(result) == 0 {
            unsafe {
                PyErr_Format(
                    PyExc_TypeError,
                    c"__bytes__ returned non-bytes (type %.200s)".as_ptr(),
                    (*Py_TYPE!(result)).tp_name,
                )
            };
            Py_DECREF!(result);
            return core::ptr::null_mut::<_object>();
        }
        unsafe {
            *(pbuf as *mut *const i8) = PyBytes_AS_STRING!(result) as *const i8;
        }
        unsafe {
            *(plen as *mut i64) = PyBytes_GET_SIZE!(result);
        }
        return result;
    }
    // does it support buffer protocol?
    if (unsafe { PyObject_CheckBuffer(v) }) != 0 {
        // maybe we can avoid making a copy of the buffer object here?
        result = _PyBytes_FromBuffer(v);
        if result.is_null() {
            return core::ptr::null_mut::<_object>();
        }
        unsafe {
            *(pbuf as *mut *const i8) = PyBytes_AS_STRING!(result) as *const i8;
        }
        unsafe {
            *(plen as *mut i64) = PyBytes_GET_SIZE!(result);
        }
        return result;
    }
    unsafe {
        PyErr_Format(PyExc_TypeError, c"%%b requires a bytes-like object, or an object that implements __bytes__, not '%.100s'".as_ptr(), (*Py_TYPE!(v)).tp_name)
    };
    core::ptr::null_mut::<_object>()
}

// fmt%(v1,v2,...) is roughly equivalent to sprintf(fmt, v1, v2, ...)
/// Unescape a backslash-escaped string.
#[unsafe(no_mangle)]
extern "C-unwind" fn _PyBytes_DecodeEscape2(
    mut s: *const i8,
    len: i64,
    errors: *const i8,
    first_invalid_escape_char: *mut i32,
    first_invalid_escape_ptr: *mut *const i8,
) -> *mut _object {
    let mut digit2: i32 = unsafe { core::mem::zeroed() };
    let mut digit1: i32 = unsafe { core::mem::zeroed() };
    let mut writer: core::mem::MaybeUninit<__SlateRecord_pycore_bytesobject_h_89> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_48: *mut __SlateRecord_pycore_bytesobject_h_89 =
        core::ptr::addr_of_mut!(writer) as *mut __SlateRecord_pycore_bytesobject_h_89;
    let mut end: *const i8 = unsafe { core::mem::zeroed() };
    let mut p: *mut i8 = unsafe { core::mem::zeroed() };
    let mut c: i32 = unsafe { core::mem::zeroed() };
    unsafe {
        _PyBytesWriter_Init(core::ptr::addr_of_mut!(*__slate_slot_48));
        p = _PyBytesWriter_Alloc(core::ptr::addr_of_mut!(*__slate_slot_48), len) as *mut i8;
        if p.is_null() {
            return core::ptr::null_mut::<_object>();
        } else {
            (*__slate_slot_48).overallocate = 1;
            *first_invalid_escape_char = 1i32.wrapping_neg();
            *first_invalid_escape_ptr = core::ptr::null::<i8>();
            end = s.wrapping_offset(len as isize);
            '__join_0: {
                '__join_37: {
                    '__join_14: {
                        '__join_11: {
                            while s < end {
                                if (*s as i32) != 92 {
                                    let __v47: *const i8 = s;
                                    s = __v47.wrapping_offset(1 as isize);
                                    let __v45: *mut i8 = p;
                                    p = __v45.wrapping_offset(1 as isize);
                                    *__v45 = *__v47;
                                } else {
                                    let __v43: *const i8 = s;
                                    s = __v43.wrapping_offset(1 as isize);
                                    if s == end {
                                        break '__join_37;
                                    } else {
                                        let __v41: *const i8 = s;
                                        s = __v41.wrapping_offset(1 as isize);
                                        let __t0: i32 = *__v41 as i32;
                                        if __t0 == 10 {
                                            continue;
                                        } else {
                                            if __t0 == 92 {
                                                let __v39: *mut i8 = p;
                                                p = __v39.wrapping_offset(1 as isize);
                                                *__v39 = 92;
                                                continue;
                                            } else {
                                                if __t0 == 39 {
                                                    let __v37: *mut i8 = p;
                                                    p = __v37.wrapping_offset(1 as isize);
                                                    *__v37 = 39;
                                                    continue;
                                                } else {
                                                    if __t0 == 34 {
                                                        let __v35: *mut i8 = p;
                                                        p = __v35.wrapping_offset(1 as isize);
                                                        *__v35 = 34;
                                                        continue;
                                                    } else {
                                                        if __t0 == 98 {
                                                            let __v33: *mut i8 = p;
                                                            p = __v33.wrapping_offset(1 as isize);
                                                            *__v33 = 8;
                                                            continue;
                                                        } else {
                                                            if __t0 == 102 {
                                                                let __v31: *mut i8 = p;
                                                                p = __v31
                                                                    .wrapping_offset(1 as isize);
                                                                *__v31 = 12;
                                                                continue;
                                                            } else {
                                                                if __t0 == 116 {
                                                                    let __v29: *mut i8 = p;
                                                                    p = __v29.wrapping_offset(
                                                                        1 as isize,
                                                                    );
                                                                    *__v29 = 9;
                                                                    continue;
                                                                } else {
                                                                    if __t0 == 110 {
                                                                        let __v27: *mut i8 = p;
                                                                        p = __v27.wrapping_offset(
                                                                            1 as isize,
                                                                        );
                                                                        *__v27 = 10;
                                                                        continue;
                                                                    } else {
                                                                        if __t0 == 114 {
                                                                            let __v25: *mut i8 = p;
                                                                            p = __v25
                                                                                .wrapping_offset(
                                                                                    1 as isize,
                                                                                );
                                                                            *__v25 = 13;
                                                                            continue;
                                                                        } else {
                                                                            if __t0 == 118 {
                                                                                let __v23: *mut i8 =
                                                                                    p;
                                                                                p = __v23.wrapping_offset(1 as isize);
                                                                                *__v23 = 11;
                                                                                continue;
                                                                            } else {
                                                                                if __t0 == 97 {
                                                                                    let __v21: *mut i8 = p;
                                                                                    p = __v21.wrapping_offset(1 as isize);
                                                                                    *__v21 = 7;
                                                                                    continue;
                                                                                } else {
                                                                                    if __t0 != 48 {
                                                                                        if __t0
                                                                                            != 49
                                                                                        {
                                                                                            if __t0 != 50 {
if __t0 != 51 {
if __t0 != 52 {
if __t0 != 53 {
if __t0 != 54 {
if __t0 != 55 {
if __t0 == 120 {
if s.wrapping_offset(1 as isize) < end {
digit1 = _PyLong_DigitValue[(Py_CHARMASK!(unsafe { *s.wrapping_offset(0 as isize) }) as i32) as usize] as i32;
digit2 = _PyLong_DigitValue[(Py_CHARMASK!(unsafe { *s.wrapping_offset(1 as isize) }) as i32) as usize] as i32;
if digit1 < 16 && digit2 < 16 {
let __v11: *mut i8 = p;
p = __v11.wrapping_offset(1 as isize);
*__v11 = ((digit1 << 4i32).wrapping_add(digit2) as u8) as i8;
let __v9: *const i8 = s;
s = __v9.wrapping_offset(2 as isize);
continue;
}
}
// invalid hexadecimal digits
if errors.is_null() || (strcmp(errors as *const core::ffi::c_char, c"strict".as_ptr() as *const core::ffi::c_char) as i32) == 0 {
break '__join_14;
} else {
if (strcmp(errors as *const core::ffi::c_char, c"replace".as_ptr() as *const core::ffi::c_char) as i32) == 0 {
let __v7: *mut i8 = p;
p = __v7.wrapping_offset(1 as isize);
*__v7 = 63;
} else {
if (strcmp(errors as *const core::ffi::c_char, c"ignore".as_ptr() as *const core::ffi::c_char) as i32) != 0 {
break '__join_11;
}
}
// do nothing
// skip \x
if s < end && Py_ISXDIGIT__2!(unsafe { *s.wrapping_offset(0 as isize) }) != 0 {
let __v5: *const i8 = s;
s = __v5.wrapping_offset(1 as isize);
}
// and a hexdigit
continue;
}
} else {
if *first_invalid_escape_char == 1i32.wrapping_neg() {
*first_invalid_escape_char = (*s.wrapping_offset(1i32.wrapping_neg() as isize) as u8) as i32;
// Back up one char, since we've already incremented s.
*first_invalid_escape_ptr = s.wrapping_offset(-(1 as isize));
}
let __v3: *mut i8 = p;
p = __v3.wrapping_offset(1 as isize);
*__v3 = 92;
let __v1: *const i8 = s;
s = __v1.wrapping_offset(-(1 as isize));
continue;
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
                                        }
                                        c = (*s.wrapping_offset(1i32.wrapping_neg() as isize)
                                            as i32)
                                            .wrapping_sub(48);
                                        if s < end && 48 <= (*s as i32) && (*s as i32) <= 55 {
                                            let __v19: *const i8 = s;
                                            s = __v19.wrapping_offset(1 as isize);
                                            c = (c << 3i32)
                                                .wrapping_add(*__v19 as i32)
                                                .wrapping_sub(48);
                                            if s < end && 48 <= (*s as i32) && (*s as i32) <= 55 {
                                                let __v17: *const i8 = s;
                                                s = __v17.wrapping_offset(1 as isize);
                                                c = (c << 3i32)
                                                    .wrapping_add(*__v17 as i32)
                                                    .wrapping_sub(48);
                                            }
                                        }
                                        if c > 255 {
                                            if *first_invalid_escape_char == 1i32.wrapping_neg() {
                                                *first_invalid_escape_char = c;
                                                // Back up 3 chars, since we've already incremented s.
                                                *first_invalid_escape_ptr =
                                                    s.wrapping_offset(-(3 as isize));
                                            }
                                        }
                                        let __v15: *mut i8 = p;
                                        p = __v15.wrapping_offset(1 as isize);
                                        *__v15 = c as i8;
                                    }
                                }
                            }
                            return _PyBytesWriter_Finish(
                                core::ptr::addr_of_mut!(*__slate_slot_48),
                                p as *mut (),
                            );
                        }
                        PyErr_Format(
                            PyExc_ValueError,
                            c"decoding error; unknown error handling code: %.400s".as_ptr(),
                            errors,
                        );
                        break '__join_0;
                    }
                    PyErr_Format(
                        PyExc_ValueError,
                        c"invalid \\x escape at position %zd".as_ptr(),
                        s.wrapping_offset(-(2 as isize))
                            .offset_from(end.wrapping_offset(-(len as isize)) as *const i8)
                            as i64,
                    );
                    break '__join_0;
                }
                PyErr_SetString(PyExc_ValueError, c"Trailing \\ in string".as_ptr());
            }
            _PyBytesWriter_Dealloc(core::ptr::addr_of_mut!(*__slate_slot_48));
            return core::ptr::null_mut::<_object>();
        }
    }
    // XXX This assumes ASCII!
    // FF
    // VT
    // BEL, not classic C
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyBytes_DecodeEscape(
    s: *const i8,
    len: i64,
    errors: *const i8,
    _unused_unicode: i64,
    _unused_recode_encoding: *const i8,
) -> *mut _object {
    let mut first_invalid_escape_char: i32 = 0;
    let mut first_invalid_escape_ptr: *const i8 = unsafe { core::mem::zeroed() };
    let result: *mut _object = _PyBytes_DecodeEscape2(
        s,
        len,
        errors,
        core::ptr::addr_of_mut!(first_invalid_escape_char),
        core::ptr::addr_of_mut!(first_invalid_escape_ptr),
    );
    if result.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    if first_invalid_escape_char != 1i32.wrapping_neg() {
        if first_invalid_escape_char > 255 {
            if (unsafe {
                PyErr_WarnFormat(PyExc_DeprecationWarning, 1i64, c"b\"\\%o\" is an invalid octal escape sequence. Such sequences will not work in the future. ".as_ptr(), first_invalid_escape_char)
            }) < 0
            {
                Py_DECREF!(result);
                return core::ptr::null_mut::<_object>();
            }
        } else {
            if (unsafe {
                PyErr_WarnFormat(PyExc_DeprecationWarning, 1i64, c"b\"\\%c\" is an invalid escape sequence. Such sequences will not work in the future. ".as_ptr(), first_invalid_escape_char)
            }) < 0
            {
                Py_DECREF!(result);
                return core::ptr::null_mut::<_object>();
            }
        }
    }
    result
}

// object api
#[unsafe(no_mangle)]
extern "C-unwind" fn PyBytes_Size(op: *mut _object) -> i64 {
    if PyBytes_Check!(op) == 0 {
        unsafe {
            PyErr_Format(
                PyExc_TypeError,
                c"expected bytes, %.200s found".as_ptr(),
                (*Py_TYPE!(op)).tp_name,
            )
        };
        return 1i32.wrapping_neg() as i64;
    }
    Py_SIZE__2!(op)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyBytes_AsString(op: *mut _object) -> *mut i8 {
    if PyBytes_Check!(op) == 0 {
        unsafe {
            PyErr_Format(
                PyExc_TypeError,
                c"expected bytes, %.200s found".as_ptr(),
                (*Py_TYPE!(op)).tp_name,
            )
        };
        return core::ptr::null_mut::<i8>();
    }
    unsafe {
        (*(op as *mut __SlateRecord_bytesobject_h_15))
            .ob_sval
            .as_mut_ptr() as *mut i8
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyBytes_AsStringAndSize(
    obj: *mut _object,
    s: *mut *mut i8,
    len: *mut i64,
) -> i32 {
    if s.is_null() {
        PyErr_BadInternalCall__41!();
        return 1i32.wrapping_neg();
    }
    if PyBytes_Check!(obj) == 0 {
        unsafe {
            PyErr_Format(
                PyExc_TypeError,
                c"expected bytes, %.200s found".as_ptr(),
                (*Py_TYPE!(obj)).tp_name,
            )
        };
        return 1i32.wrapping_neg();
    }
    unsafe {
        *s = PyBytes_AS_STRING!(obj);
    }
    if !len.is_null() {
        unsafe {
            *len = PyBytes_GET_SIZE!(obj);
        }
    } else {
        if (unsafe { strlen((*s as *const i8) as *const core::ffi::c_char) as u64 })
            != (PyBytes_GET_SIZE!(obj) as u64)
        {
            unsafe { PyErr_SetString(PyExc_ValueError, c"embedded null byte".as_ptr()) };
            return 1i32.wrapping_neg();
        }
    }
    0
}

fn stringlib_find_char(s: *const i8, n: i64, ch: i8) -> i64 {
    let mut p: *const i8 = unsafe { core::mem::zeroed() };
    p = s;
    let e: *const i8 = s.wrapping_offset(n as isize);
    if n > 15 {
        p = ((unsafe {
            memchr(
                (s as *const ()) as *const core::ffi::c_void,
                (ch as i32) as i32,
                (n as u64) as usize,
            ) as *mut ()
        }) as *const ()) as *const i8;
        if !p.is_null() {
            return (unsafe { p.offset_from(s as *const i8) }) as i64;
        }
        return 1i32.wrapping_neg() as i64;
    }
    while p < e {
        if ((unsafe { *p }) as i32) == (ch as i32) {
            return (unsafe { p.offset_from(s as *const i8) }) as i64;
        }
        let __v0: *const i8 = p;
        p = __v0.wrapping_offset(1 as isize);
    }
    1i32.wrapping_neg() as i64
}

fn stringlib_rfind_char(s: *const i8, n: i64, ch: i8) -> i64 {
    let mut p: *const i8 = unsafe { core::mem::zeroed() };
    // memrchr() is a GNU extension, available since glibc 2.1.91.  it
    // doesn't seem as optimized as memchr(), but is still quite
    // faster than our hand-written loop below. There is no wmemrchr
    // for 4-byte chars.
    if n > 15 {
        p = (unsafe { memrchr(s as *const (), ch as i32, n as u64) }) as *const i8;
        if !p.is_null() {
            return (unsafe { p.offset_from(s as *const i8) }) as i64;
        }
        return 1i32.wrapping_neg() as i64;
    }
    p = s.wrapping_offset(n as isize);
    while p > s {
        let __v0: *const i8 = p;
        p = __v0.wrapping_offset(-(1 as isize));
        if ((unsafe { *p }) as i32) == (ch as i32) {
            return (unsafe { p.offset_from(s as *const i8) }) as i64;
        }
    }
    1i32.wrapping_neg() as i64
}

fn stringlib__lex_search(
    needle: *const i8,
    len_needle: i64,
    return_period: &mut i64,
    invert_alphabet: i32,
) -> i64 {
    // Do a lexicographic search. Essentially this:
    //     >>> max(needle[i:] for i in range(len(needle)+1))
    // Also find the period of the right half.
    let mut max_suffix: i64 = 0;
    let mut candidate: i64 = 1;
    let mut k: i64 = 0;
    // The period of the right half.
    let mut period: i64 = 1;
    while candidate.wrapping_add(k) < len_needle {
        // each loop increases candidate + k + max_suffix
        let a: i8 = unsafe { *needle.wrapping_offset(candidate.wrapping_add(k) as isize) };
        let b: i8 = unsafe { *needle.wrapping_offset(max_suffix.wrapping_add(k) as isize) };
        // check if the suffix at candidate is better than max_suffix
        if (if invert_alphabet != 0 {
            ((b as i32) < (a as i32)) as i32
        } else {
            ((a as i32) < (b as i32)) as i32
        }) != 0
        {
            // Fell short of max_suffix.
            //
            // The next k + 1 characters are non-increasing
            //
            // from candidate, so they won't start a maximal suffix.
            candidate = candidate.wrapping_add(k.wrapping_add(1));
            k = 0;
            // We've ruled out any period smaller than what's
            //
            // been scanned since max_suffix.
            period = candidate.wrapping_sub(max_suffix);
        } else {
            if (a as i32) == (b as i32) {
                if k.wrapping_add(1) != period {
                    // Keep scanning the equal strings
                    k = k.wrapping_add(1);
                } else {
                    // Matched a whole period.
                    //
                    // Start matching the next period.
                    candidate = candidate.wrapping_add(period);
                    k = 0;
                }
            } else {
                // Did better than max_suffix, so replace it.
                max_suffix = candidate;
                candidate = candidate.wrapping_add(1);
                k = 0;
                period = 1;
            }
        }
    }
    unsafe {
        *(return_period as *mut i64) = period;
    }
    max_suffix
}

fn stringlib__factorize(needle: *const i8, len_needle: i64, return_period: *mut i64) -> i64 {
    // Do a "critical factorization", making it so that:
    // >>> needle = (left := needle[:cut]) + (right := needle[cut:])
    // where the "local period" of the cut is maximal.
    //
    // The local period of the cut is the minimal length of a string w
    // such that (left endswith w or w endswith left)
    // and (right startswith w or w startswith right).
    //
    // The Critical Factorization Theorem says that this maximal local
    // period is the global period of the string.
    //
    // Crochemore and Perrin (1991) show that this cut can be computed
    // as the later of two cuts: one that gives a lexicographically
    // maximal right half, and one that gives the same with the
    // with respect to a reversed alphabet-ordering.
    //
    // This is what we want to happen:
    //     >>> x = "GCAGAGAG"
    //     >>> cut, period = factorize(x)
    //     >>> x[:cut], (right := x[cut:])
    //     ('GC', 'AGAGAG')
    //     >>> period  # right half period
    //     2
    //     >>> right[period:] == right[:-period]
    //     True
    //
    // This is how the local period lines up in the above example:
    //          GC | AGAGAG
    //     AGAGAGC = AGAGAGC
    // The length of this minimal repetition is 7, which is indeed the
    // period of the original string.
    let mut cut1: i64 = 0;
    let mut period1: i64 = 0;
    let mut cut2: i64 = 0;
    let mut period2: i64 = 0;
    let mut cut: i64 = 0;
    let mut period: i64 = 0;
    cut1 = stringlib__lex_search(
        needle,
        len_needle,
        unsafe { &mut (*core::ptr::addr_of_mut!(period1)) },
        0i32,
    );
    cut2 = stringlib__lex_search(
        needle,
        len_needle,
        unsafe { &mut (*core::ptr::addr_of_mut!(period2)) },
        1i32,
    );
    // Take the later cut.
    if cut1 > cut2 {
        period = period1;
        cut = cut1;
    } else {
        period = period2;
        cut = cut2;
    }
    unsafe {
        *return_period = period;
    }
    cut
}

#[repr(C)]
#[derive(Clone, Copy)]
struct stringlib__pre {
    needle: *const i8,
    len_needle: i64,
    cut: i64,
    period: i64,
    gap: i64,
    is_periodic: i32,
    table: [u8; 64],
}

fn stringlib__preprocess(needle: *const i8, len_needle: i64, p: *mut stringlib__pre) {
    unsafe {
        (*p).needle = needle;
    }
    unsafe {
        (*p).len_needle = len_needle;
    }
    unsafe {
        (*p).cut = stringlib__factorize(needle, len_needle, core::ptr::addr_of_mut!((*p).period));
    }
    unsafe {
        (*p).is_periodic = (0
            == (memcmp(
                (needle as *const ()) as *const core::ffi::c_void,
                (needle.wrapping_offset((*p).period as isize) as *const ())
                    as *const core::ffi::c_void,
                (unsafe { (*p).cut }.wrapping_mul(1) as u64) as usize,
            ) as i32)) as i32;
    }
    if (unsafe { (*p).is_periodic }) == 0 {
        // A lower bound on the period
        unsafe {
            (*p).period =
                Py_MAX!(unsafe { (*p).cut }, len_needle.wrapping_sub((*p).cut)).wrapping_add(1);
        }
    }
    // The gap between the last character and the previous
    //
    // occurrence of an equivalent character (modulo TABLE_SIZE)
    unsafe {
        (*p).gap = len_needle;
    }
    let last: i8 = (((unsafe { *needle.wrapping_offset(len_needle.wrapping_sub(1) as isize) })
        as u32)
        & 63u32) as i8;
    {
        let mut i: i64 = len_needle.wrapping_sub(2);
        while i >= 0 {
            let x: i8 = (((unsafe { *needle.wrapping_offset(i as isize) }) as u32) & 63u32) as i8;
            if (x as i32) == (last as i32) {
                unsafe {
                    (*p).gap = len_needle.wrapping_sub(1).wrapping_sub(i);
                }
                break;
            }
            i = i.wrapping_sub(1);
        }
    }
    // Fill up a compressed Boyer-Moore "Bad Character" table
    let not_found_shift: i64 = Py_MIN__2!(len_needle, 255i32);
    for i in 0..64i64 {
        unsafe {
            *((*p).table.as_mut_ptr() as *mut u8).wrapping_offset(i as isize) =
                Py_SAFE_DOWNCAST!(not_found_shift, u8);
        }
    }
    let mut i: i64 = len_needle.wrapping_sub(not_found_shift);
    while i < len_needle {
        let shift: u8 = Py_SAFE_DOWNCAST!(len_needle.wrapping_sub(1).wrapping_sub(i), u8);
        unsafe {
            *((*p).table.as_mut_ptr() as *mut u8)
                .wrapping_offset(((*needle.wrapping_offset(i as isize) as u32) & 63u32) as isize) =
                shift;
        }
        i = i.wrapping_add(1);
    }
}

fn stringlib__two_way(haystack: *const i8, len_haystack: i64, p: &mut stringlib__pre) -> i64 {
    let mut shift: i64 = unsafe { core::mem::zeroed() };
    let mut i: i64 = unsafe { core::mem::zeroed() };
    let mut memory: i64 = unsafe { core::mem::zeroed() };
    let mut i_1: i64 = unsafe { core::mem::zeroed() };
    let mut i_2: i64 = unsafe { core::mem::zeroed() };
    let mut gap_jump_end: i64 = unsafe { core::mem::zeroed() };
    let mut gap: i64 = unsafe { core::mem::zeroed() };
    let mut window: *const i8 = unsafe { core::mem::zeroed() };
    let mut table: *mut u8 = unsafe { core::mem::zeroed() };
    let mut haystack_end: *const i8 = unsafe { core::mem::zeroed() };
    let mut window_last: *const i8 = unsafe { core::mem::zeroed() };
    let mut needle: *const i8 = unsafe { core::mem::zeroed() };
    let mut period: i64 = unsafe { core::mem::zeroed() };
    let mut cut: i64 = unsafe { core::mem::zeroed() };
    // Crochemore and Perrin's (1991) Two-Way algorithm.
    //
    // See http://www-igm.univ-mlv.fr/~lecroq/string/node26.html#SECTION00260
    let mut len_needle: i64 = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_0: {
            len_needle = (*(p as *mut stringlib__pre)).len_needle;
            cut = (*(p as *mut stringlib__pre)).cut;
            period = (*(p as *mut stringlib__pre)).period;
            needle = (*(p as *mut stringlib__pre)).needle;
            window_last = haystack
                .wrapping_offset(len_needle as isize)
                .wrapping_offset(-(1 as isize));
            haystack_end = haystack.wrapping_offset(len_haystack as isize);
            table = (*(p as *mut stringlib__pre)).table.as_mut_ptr() as *mut u8;
            gap = (*(p as *mut stringlib__pre)).gap;
            gap_jump_end = Py_MIN!(len_needle, cut.wrapping_add(gap));
            if (*(p as *mut stringlib__pre)).is_periodic != 0 {
                memory = 0;
                '__loop_1: loop {
                    if window_last < haystack_end {
                        loop {
                            let shift: i64 = *table
                                .wrapping_offset(((*window_last as u32) & 63u32) as isize)
                                as i64;
                            let __v16: *const i8 = window_last;
                            window_last = __v16.wrapping_offset(shift as isize);
                            if shift == 0 {
                                break;
                            } else {
                                if window_last >= haystack_end {
                                    break '__loop_1;
                                }
                            }
                        }
                        '__join_16: {
                            loop {
                                window = window_last
                                    .wrapping_offset(-(len_needle as isize))
                                    .wrapping_offset(1 as isize);
                                i = Py_MAX!(cut, memory);
                                while i < len_needle {
                                    if (*needle.wrapping_offset(i as isize) as i32)
                                        != (*window.wrapping_offset(i as isize) as i32)
                                    {
                                        break '__join_16;
                                    } else {
                                        i = i.wrapping_add(1);
                                    }
                                }
                                i = memory;
                                loop {
                                    if i < cut {
                                        if (*needle.wrapping_offset(i as isize) as i32)
                                            != (*window.wrapping_offset(i as isize) as i32)
                                        {
                                            break;
                                        } else {
                                            i = i.wrapping_add(1);
                                        }
                                    } else {
                                        return window.offset_from(haystack as *const i8) as i64;
                                    }
                                }
                                let __v7: *const i8 = window_last;
                                window_last = __v7.wrapping_offset(period as isize);
                                memory = len_needle.wrapping_sub(period);
                                if window_last >= haystack_end {
                                    return 1i32.wrapping_neg() as i64;
                                } else {
                                    shift = *table
                                        .wrapping_offset(((*window_last as u32) & 63u32) as isize)
                                        as i64;
                                    if shift != 0 {
                                        break;
                                    }
                                }
                            }
                            // A mismatch has been identified to the right
                            //
                            // of where i will next start, so we can jump
                            //
                            // at least as far as if the mismatch occurred
                            //
                            // on the first comparison.
                            let mem_jump: i64 =
                                Py_MAX!(cut, memory).wrapping_sub(cut).wrapping_add(1);
                            memory = 0;
                            let __v3: *const i8 = window_last;
                            window_last = __v3.wrapping_offset(Py_MAX!(shift, mem_jump) as isize);
                            continue '__loop_1;
                        }
                        if i < gap_jump_end {
                            let __v11: *const i8 = window_last;
                            window_last = __v11.wrapping_offset(gap as isize);
                        } else {
                            let __v13: *const i8 = window_last;
                            window_last =
                                __v13.wrapping_offset(i.wrapping_sub(cut).wrapping_add(1) as isize);
                        }
                        memory = 0;
                    } else {
                        break '__join_0;
                    }
                }
                return 1i32.wrapping_neg() as i64;
            } else {
                period = Py_MAX!(gap, period);
                '__loop_25: loop {
                    if window_last < haystack_end {
                        loop {
                            let shift: i64 = *table
                                .wrapping_offset(((*window_last as u32) & 63u32) as isize)
                                as i64;
                            let __v32: *const i8 = window_last;
                            window_last = __v32.wrapping_offset(shift as isize);
                            if shift == 0 {
                                break;
                            } else {
                                if window_last >= haystack_end {
                                    break '__loop_25;
                                }
                            }
                        }
                        window = window_last
                            .wrapping_offset(-(len_needle as isize))
                            .wrapping_offset(1 as isize);
                        i_2 = cut;
                        '__join_36: {
                            while i_2 < len_needle {
                                if (*needle.wrapping_offset(i_2 as isize) as i32)
                                    != (*window.wrapping_offset(i_2 as isize) as i32)
                                {
                                    break '__join_36;
                                } else {
                                    i_2 = i_2.wrapping_add(1);
                                }
                            }
                            i_1 = 0;
                            loop {
                                if i_1 < cut {
                                    if (*needle.wrapping_offset(i_1 as isize) as i32)
                                        != (*window.wrapping_offset(i_1 as isize) as i32)
                                    {
                                        break;
                                    } else {
                                        i_1 = i_1.wrapping_add(1);
                                    }
                                } else {
                                    return window.offset_from(haystack as *const i8) as i64;
                                }
                            }
                            let __v22: *const i8 = window_last;
                            window_last = __v22.wrapping_offset(period as isize);
                            continue '__loop_25;
                        }
                        if i_2 < gap_jump_end {
                            let __v27: *const i8 = window_last;
                            window_last = __v27.wrapping_offset(gap as isize);
                        } else {
                            let __v29: *const i8 = window_last;
                            window_last = __v29
                                .wrapping_offset(i_2.wrapping_sub(cut).wrapping_add(1) as isize);
                        }
                    } else {
                        break '__join_0;
                    }
                }
                return 1i32.wrapping_neg() as i64;
            }
        }
        return 1i32.wrapping_neg() as i64;
    }
}

fn stringlib__two_way_find(
    haystack: *const i8,
    len_haystack: i64,
    needle: *const i8,
    len_needle: i64,
) -> i64 {
    let mut p: stringlib__pre = unsafe { core::mem::zeroed() };
    stringlib__preprocess(needle, len_needle, core::ptr::addr_of_mut!(p));
    stringlib__two_way(haystack, len_haystack, unsafe {
        &mut (*core::ptr::addr_of_mut!(p))
    })
}

fn stringlib__two_way_count(
    haystack: *const i8,
    len_haystack: i64,
    needle: *const i8,
    len_needle: i64,
    maxcount: i64,
) -> i64 {
    let mut p: stringlib__pre = unsafe { core::mem::zeroed() };
    stringlib__preprocess(needle, len_needle, core::ptr::addr_of_mut!(p));
    let mut index: i64 = 0;
    let mut count: i64 = 0;
    while true {
        let result: i64 = stringlib__two_way(
            haystack.wrapping_offset(index as isize),
            len_haystack.wrapping_sub(index),
            unsafe { &mut (*core::ptr::addr_of_mut!(p)) },
        );
        if result == (1i32.wrapping_neg() as i64) {
            return count;
        }
        count = count.wrapping_add(1);
        if count == maxcount {
            return maxcount;
        }
        index = index.wrapping_add(result.wrapping_add(len_needle));
    }
    count
}

fn stringlib_default_find(
    s: *const i8,
    n: i64,
    p: *const i8,
    m: i64,
    maxcount: i64,
    mode: i32,
) -> i64 {
    let w: i64 = n.wrapping_sub(m);
    let mlast: i64 = m.wrapping_sub(1);
    let mut count: i64 = 0;
    let mut gap: i64 = mlast;
    let last: i8 = unsafe { *p.wrapping_offset(mlast as isize) };
    let ss: *const i8 = s.wrapping_offset(mlast as isize);
    let mut mask: u64 = 0;
    for i in 0..mlast {
        STRINGLIB_BLOOM_ADD!(mask, unsafe { *p.wrapping_offset(i as isize) });
        if ((unsafe { *p.wrapping_offset(i as isize) }) as i32) == (last as i32) {
            gap = mlast.wrapping_sub(i).wrapping_sub(1);
        }
    }
    STRINGLIB_BLOOM_ADD!(mask, last);
    let mut i: i64 = 0;
    while i <= w {
        if ((unsafe { *ss.wrapping_offset(i as isize) }) as i32) == (last as i32) {
            // candidate match
            let mut j: i64 = 0;
            while j < mlast {
                if ((unsafe { *s.wrapping_offset(i.wrapping_add(j) as isize) }) as i32)
                    != ((unsafe { *p.wrapping_offset(j as isize) }) as i32)
                {
                    break;
                }
                j = j.wrapping_add(1);
            }
            if j == mlast {
                // got a match!
                if mode != 0 {
                    return i;
                }
                count = count.wrapping_add(1);
                if count == maxcount {
                    return maxcount;
                }
                i = i.wrapping_add(mlast);
            } else {
                // miss: check if next character is part of pattern
                if i.wrapping_add(1) <= w
                    && STRINGLIB_BLOOM!(mask, unsafe {
                        *ss.wrapping_offset(i.wrapping_add(1) as isize)
                    }) == 0
                {
                    i = i.wrapping_add(m);
                } else {
                    i = i.wrapping_add(gap);
                }
            }
        } else {
            // skip: check if next character is part of pattern
            if i.wrapping_add(1) <= w
                && STRINGLIB_BLOOM!(mask, unsafe {
                    *ss.wrapping_offset(i.wrapping_add(1) as isize)
                }) == 0
            {
                i = i.wrapping_add(m);
            }
        }
        i = i.wrapping_add(1);
    }
    if mode == 0 {
        count
    } else {
        1i32.wrapping_neg() as i64
    }
}

fn stringlib_adaptive_find(
    s: *const i8,
    n: i64,
    p: *const i8,
    m: i64,
    maxcount: i64,
    mode: i32,
) -> i64 {
    let w: i64 = n.wrapping_sub(m);
    let mlast: i64 = m.wrapping_sub(1);
    let mut count: i64 = 0;
    let mut gap: i64 = mlast;
    let mut hits: i64 = 0;
    let mut res: i64 = 0;
    let last: i8 = unsafe { *p.wrapping_offset(mlast as isize) };
    let ss: *const i8 = s.wrapping_offset(mlast as isize);
    let mut mask: u64 = 0;
    for i in 0..mlast {
        STRINGLIB_BLOOM_ADD!(mask, unsafe { *p.wrapping_offset(i as isize) });
        if ((unsafe { *p.wrapping_offset(i as isize) }) as i32) == (last as i32) {
            gap = mlast.wrapping_sub(i).wrapping_sub(1);
        }
    }
    STRINGLIB_BLOOM_ADD!(mask, last);
    let mut i: i64 = 0;
    while i <= w {
        if ((unsafe { *ss.wrapping_offset(i as isize) }) as i32) == (last as i32) {
            // candidate match
            let mut j: i64 = 0;
            while j < mlast {
                if ((unsafe { *s.wrapping_offset(i.wrapping_add(j) as isize) }) as i32)
                    != ((unsafe { *p.wrapping_offset(j as isize) }) as i32)
                {
                    break;
                }
                j = j.wrapping_add(1);
            }
            if j == mlast {
                // got a match!
                if mode != 0 {
                    return i;
                }
                count = count.wrapping_add(1);
                if count == maxcount {
                    return maxcount;
                }
                i = i.wrapping_add(mlast);
            } else {
                hits = hits.wrapping_add(j.wrapping_add(1));
                if hits > m / 4 && w.wrapping_sub(i) > 2000 {
                    if mode == 1 {
                        res = stringlib__two_way_find(
                            s.wrapping_offset(i as isize),
                            n.wrapping_sub(i),
                            p,
                            m,
                        );
                        return if res == (1i32.wrapping_neg() as i64) {
                            1i32.wrapping_neg() as i64
                        } else {
                            res.wrapping_add(i)
                        };
                    } else {
                        res = stringlib__two_way_count(
                            s.wrapping_offset(i as isize),
                            n.wrapping_sub(i),
                            p,
                            m,
                            maxcount.wrapping_sub(count),
                        );
                        return res.wrapping_add(count);
                    }
                }
                // miss: check if next character is part of pattern
                if i.wrapping_add(1) <= w
                    && STRINGLIB_BLOOM!(mask, unsafe {
                        *ss.wrapping_offset(i.wrapping_add(1) as isize)
                    }) == 0
                {
                    i = i.wrapping_add(m);
                } else {
                    i = i.wrapping_add(gap);
                }
            }
        } else {
            // skip: check if next character is part of pattern
            if i.wrapping_add(1) <= w
                && STRINGLIB_BLOOM!(mask, unsafe {
                    *ss.wrapping_offset(i.wrapping_add(1) as isize)
                }) == 0
            {
                i = i.wrapping_add(m);
            }
        }
        i = i.wrapping_add(1);
    }
    if mode == 0 {
        count
    } else {
        1i32.wrapping_neg() as i64
    }
}

fn stringlib_default_rfind(
    s: *const i8,
    n: i64,
    p: *const i8,
    m: i64,
    maxcount: i64,
    mode: i32,
) -> i64 {
    // create compressed boyer-moore delta 1 table
    let mut mask: u64 = 0;
    let mut i: i64 = 0;
    let mut j: i64 = 0;
    let mlast: i64 = m.wrapping_sub(1);
    let mut skip: i64 = m.wrapping_sub(1);
    let w: i64 = n.wrapping_sub(m);
    // process pattern[0] outside the loop
    STRINGLIB_BLOOM_ADD!(mask, unsafe { *p.wrapping_offset(0 as isize) });
    // process pattern[:0:-1]
    i = mlast;
    while i > 0 {
        STRINGLIB_BLOOM_ADD!(mask, unsafe { *p.wrapping_offset(i as isize) });
        if ((unsafe { *p.wrapping_offset(i as isize) }) as i32)
            == ((unsafe { *p.wrapping_offset(0 as isize) }) as i32)
        {
            skip = i.wrapping_sub(1);
        }
        i = i.wrapping_sub(1);
    }
    i = w;
    while i >= 0 {
        if ((unsafe { *s.wrapping_offset(i as isize) }) as i32)
            == ((unsafe { *p.wrapping_offset(0 as isize) }) as i32)
        {
            // candidate match
            j = mlast;
            while j > 0 {
                if ((unsafe { *s.wrapping_offset(i.wrapping_add(j) as isize) }) as i32)
                    != ((unsafe { *p.wrapping_offset(j as isize) }) as i32)
                {
                    break;
                }
                j = j.wrapping_sub(1);
            }
            if j == 0 {
                // got a match!
                return i;
            }
            // miss: check if previous character is part of pattern
            if i > 0
                && STRINGLIB_BLOOM!(mask, unsafe {
                    *s.wrapping_offset(i.wrapping_sub(1) as isize)
                }) == 0
            {
                i = i.wrapping_sub(m);
            } else {
                i = i.wrapping_sub(skip);
            }
        } else {
            // skip: check if previous character is part of pattern
            if i > 0
                && STRINGLIB_BLOOM!(mask, unsafe {
                    *s.wrapping_offset(i.wrapping_sub(1) as isize)
                }) == 0
            {
                i = i.wrapping_sub(m);
            }
        }
        i = i.wrapping_sub(1);
    }
    1i32.wrapping_neg() as i64
}

fn stringlib_count_char(s: *const i8, n: i64, p0: i8, maxcount: i64) -> i64 {
    let mut count: i64 = 0;
    for i in 0..n {
        if ((unsafe { *s.wrapping_offset(i as isize) }) as i32) == (p0 as i32) {
            count = count.wrapping_add(1);
            if count == maxcount {
                return maxcount;
            }
        }
    }
    count
}

fn stringlib_count_char_no_maxcount(s: *const i8, n: i64, p0: i8) -> i64 {
    let mut count: i64 = 0;
    for i in 0..n {
        if ((unsafe { *s.wrapping_offset(i as isize) }) as i32) == (p0 as i32) {
            count = count.wrapping_add(1);
        }
    }
    count
}

fn fastsearch(s: *const i8, n: i64, p: *const i8, m: i64, maxcount: i64, mode: i32) -> i64 {
    if n < m || mode == 0 && maxcount == 0 {
        return 1i32.wrapping_neg() as i64;
    }
    // look for special cases
    if m <= 1 {
        if m <= 0 {
            return 1i32.wrapping_neg() as i64;
        }
        // use special case for 1-character strings
        if mode == 1 {
            return stringlib_find_char(s, n, unsafe { *p.wrapping_offset(0 as isize) });
        } else {
            if mode == 2 {
                return stringlib_rfind_char(s, n, unsafe { *p.wrapping_offset(0 as isize) });
            } else {
                if maxcount == 9223372036854775807 {
                    return stringlib_count_char_no_maxcount(s, n, unsafe {
                        *p.wrapping_offset(0 as isize)
                    });
                }
                return stringlib_count_char(
                    s,
                    n,
                    unsafe { *p.wrapping_offset(0 as isize) },
                    maxcount,
                );
            }
        }
    }
    if mode != 2 {
        if n < 2500 || m < 100 && n < 30000 || m < 6 {
            return stringlib_default_find(s, n, p, m, maxcount, mode);
        } else {
            if (m >> 2i32).wrapping_mul(3) < n >> 2i32 {
                // 33% threshold, but don't overflow.
                //
                // For larger problems where the needle isn't a huge
                // percentage of the size of the haystack, the relatively
                // expensive O(m) startup cost of the two-way algorithm
                // will surely pay off.
                if mode == 1 {
                    return stringlib__two_way_find(s, n, p, m);
                } else {
                    return stringlib__two_way_count(s, n, p, m, maxcount);
                }
            } else {
                // To ensure that we have good worst-case behavior,
                // here's an adaptive version of the algorithm, where if
                // we match O(m) characters without any matches of the
                // entire needle, then we predict that the startup cost of
                // the two-way algorithm will probably be worth it.
                return stringlib_adaptive_find(s, n, p, m, maxcount, mode);
            }
        }
    } else {
        // FAST_RSEARCH
        return stringlib_default_rfind(s, n, p, m, maxcount, mode);
    }
}

fn stringlib_count(
    str: *const i8,
    str_len: i64,
    sub: *const i8,
    sub_len: i64,
    maxcount: i64,
) -> i64 {
    let mut count: i64 = 0;
    if str_len < 0 {
        return 0;
    }
    // start > len(str)
    if sub_len == 0 {
        return if str_len < maxcount {
            str_len.wrapping_add(1)
        } else {
            maxcount
        };
    }
    count = fastsearch(str, str_len, sub, sub_len, maxcount, 0i32);
    if count < 0 {
        return 0;
    }
    // no match
    count
}

fn stringlib_find(str: *const i8, str_len: i64, sub: *const i8, sub_len: i64, offset: i64) -> i64 {
    let mut pos: i64 = 0;
    if sub_len == 0 {
        return offset;
    }
    pos = fastsearch(str, str_len, sub, sub_len, 1i32.wrapping_neg() as i64, 1i32);
    if pos >= 0 {
        pos = pos.wrapping_add(offset);
    }
    pos
}

fn stringlib_rfind(str: *const i8, str_len: i64, sub: *const i8, sub_len: i64, offset: i64) -> i64 {
    let mut pos: i64 = 0;
    if sub_len == 0 {
        return str_len.wrapping_add(offset);
    }
    pos = fastsearch(str, str_len, sub, sub_len, 1i32.wrapping_neg() as i64, 2i32);
    if pos >= 0 {
        pos = pos.wrapping_add(offset);
    }
    pos
}

fn stringlib_bytes_join(sep: *mut _object, iterable: *mut _object) -> *mut _object {
    let mut itemlen: i64 = unsafe { core::mem::zeroed() };
    let mut save: *mut _ts = unsafe { core::mem::zeroed() };
    let mut drop_gil: i32 = unsafe { core::mem::zeroed() };
    let mut static_buffers: core::mem::MaybeUninit<
        __SlateAlign16<[__SlateRecord_pybuffer_h_33; 10]>,
    > = core::mem::MaybeUninit::uninit();
    let __slate_slot_25: *mut [__SlateRecord_pybuffer_h_33; 10] =
        core::ptr::addr_of_mut!(static_buffers) as *mut [__SlateRecord_pybuffer_h_33; 10];
    let mut buffers: *mut __SlateRecord_pybuffer_h_33 = unsafe { core::mem::zeroed() };
    let mut item: *mut _object = unsafe { core::mem::zeroed() };
    let mut seq: *mut _object = unsafe { core::mem::zeroed() };
    let mut nbufs: i64 = unsafe { core::mem::zeroed() };
    let mut i: i64 = unsafe { core::mem::zeroed() };
    let mut sz: i64 = unsafe { core::mem::zeroed() };
    let mut seqlen: i64 = unsafe { core::mem::zeroed() };
    let mut p: *mut i8 = unsafe { core::mem::zeroed() };
    let mut res: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        let sepstr: *const i8 = PyBytes_AS_STRING(_PyObject_CAST!(sep)) as *const i8;
        let seplen: i64 = PyBytes_GET_SIZE(_PyObject_CAST!(sep));
        res = core::ptr::null_mut::<_object>();
        seqlen = 0;
        sz = 0;
        buffers = core::ptr::null_mut::<__SlateRecord_pybuffer_h_33>();
        drop_gil = 1;
        save = core::ptr::null_mut::<_ts>();
        seq = PySequence_Fast(iterable, c"can only join an iterable".as_ptr());
        if seq.is_null() {
            return core::ptr::null_mut::<_object>();
        } else {
            seqlen = PySequence_Fast_GET_SIZE!(seq);
            if seqlen == 0 {
                Py_DECREF!(seq);
                return PyBytes_FromStringAndSize(core::ptr::null::<i8>(), 0i64);
            } else {
                if seqlen == 1 {
                    item = PySequence_Fast_GET_ITEM!(seq, 0i32);
                    if Py_IS_TYPE!(item, core::ptr::addr_of_mut!(PyBytes_Type)) != 0 {
                        Py_INCREF!(item);
                        Py_DECREF!(seq);
                        return item;
                    }
                }
                if seqlen > 10 {
                    buffers = PyMem_NEW__3!(__SlateRecord_pybuffer_h_33, seqlen);
                    if buffers.is_null() {
                        Py_DECREF!(seq);
                        PyErr_NoMemory();
                        return core::ptr::null_mut::<_object>();
                    }
                } else {
                    buffers = (*__slate_slot_25).as_mut_ptr() as *mut __SlateRecord_pybuffer_h_33;
                }
                // Here is the general case.  Do a pre-pass to figure out the total
                // amount of space we'll need (sz), and see whether all arguments are
                // bytes-like.
                i = 0;
                nbufs = 0;
                '__join_5: {
                    '__join_6: {
                        '__join_31: {
                            '__join_28: {
                                '__join_25: {
                                    '__join_35: {
                                        while i < seqlen {
                                            item = PySequence_Fast_GET_ITEM!(seq, i);
                                            if PyBytes_CheckExact!(item) != 0 {
                                                // Fast path.
                                                (*buffers.wrapping_offset(i as isize)).obj =
                                                    Py_NewRef!(item);
                                                (*buffers.wrapping_offset(i as isize)).buf =
                                                    PyBytes_AS_STRING!(item) as *mut ();
                                                (*buffers.wrapping_offset(i as isize)).len =
                                                    PyBytes_GET_SIZE!(item);
                                            } else {
                                                // item is only borrowed; its __buffer__() may run Python that
                                                // drops the sequence's last reference to it.
                                                Py_INCREF!(item);
                                                if PyObject_GetBuffer(
                                                    item,
                                                    buffers.wrapping_offset(i as isize),
                                                    0i32,
                                                ) != 0
                                                {
                                                    break '__join_35;
                                                } else {
                                                    Py_DECREF!(item);
                                                    // If the backing objects are mutable, then dropping the GIL
                                                    // opens up race conditions where another thread tries to modify
                                                    // the object which we hold a buffer on it. Such code has data
                                                    // races anyway, but this is a conservative approach that avoids
                                                    // changing the behaviour of that data race.
                                                    drop_gil = 0;
                                                }
                                            }
                                            nbufs = i.wrapping_add(1); // for error cleanup
                                            itemlen = (*buffers.wrapping_offset(i as isize)).len;
                                            if itemlen > 9223372036854775807i64.wrapping_sub(sz) {
                                                break '__join_31;
                                            } else {
                                                sz = sz.wrapping_add(itemlen);
                                                if i != 0 {
                                                    if seplen
                                                        > 9223372036854775807i64.wrapping_sub(sz)
                                                    {
                                                        break '__join_28;
                                                    } else {
                                                        sz = sz.wrapping_add(seplen);
                                                    }
                                                }
                                                if seqlen != PySequence_Fast_GET_SIZE!(seq) {
                                                    break '__join_25;
                                                } else {
                                                    i = i.wrapping_add(1);
                                                }
                                            }
                                        }
                                        // Allocate result space.
                                        res =
                                            PyBytes_FromStringAndSize(core::ptr::null::<i8>(), sz);
                                        if res.is_null() {
                                            break '__join_6;
                                        } else {
                                            // Catenate everything.
                                            p = PyBytes_AS_STRING(_PyObject_CAST!(res));
                                            if sz < 1048576 {
                                                drop_gil = 0; // Benefits are likely outweighed by the overheads
                                            }
                                            if drop_gil != 0 {
                                                save = PyEval_SaveThread();
                                            }
                                            if seplen == 0 {
                                                // fast path
                                                i = 0;
                                                while i < nbufs {
                                                    let n: i64 =
                                                        (*buffers.wrapping_offset(i as isize)).len;
                                                    let q: *mut i8 =
                                                        (*buffers.wrapping_offset(i as isize)).buf
                                                            as *mut i8;
                                                    core::ptr::copy_nonoverlapping(
                                                        ((q as *const ())
                                                            as *const core::ffi::c_void)
                                                            as *const u8,
                                                        ((p as *mut ()) as *mut core::ffi::c_void)
                                                            as *mut u8,
                                                        ((n as u64) as usize) as usize,
                                                    );
                                                    let __v5: *mut i8 = p;
                                                    p = __v5.wrapping_offset(n as isize);
                                                    i = i.wrapping_add(1);
                                                }
                                            } else {
                                                i = 0;
                                                while i < nbufs {
                                                    if i != 0 {
                                                        core::ptr::copy_nonoverlapping(
                                                            ((sepstr as *const ())
                                                                as *const core::ffi::c_void)
                                                                as *const u8,
                                                            ((p as *mut ())
                                                                as *mut core::ffi::c_void)
                                                                as *mut u8,
                                                            ((seplen as u64) as usize) as usize,
                                                        );
                                                        let __v13: *mut i8 = p;
                                                        p = __v13.wrapping_offset(seplen as isize);
                                                    }
                                                    let n: i64 =
                                                        (*buffers.wrapping_offset(i as isize)).len;
                                                    let q: *mut i8 =
                                                        (*buffers.wrapping_offset(i as isize)).buf
                                                            as *mut i8;
                                                    core::ptr::copy_nonoverlapping(
                                                        ((q as *const ())
                                                            as *const core::ffi::c_void)
                                                            as *const u8,
                                                        ((p as *mut ()) as *mut core::ffi::c_void)
                                                            as *mut u8,
                                                        ((n as u64) as usize) as usize,
                                                    );
                                                    let __v11: *mut i8 = p;
                                                    p = __v11.wrapping_offset(n as isize);
                                                    i = i.wrapping_add(1);
                                                }
                                            }
                                            if drop_gil != 0 {
                                                PyEval_RestoreThread(save);
                                                break '__join_5;
                                            } else {
                                                break '__join_5;
                                            }
                                        }
                                    }
                                    Py_DECREF!(item);
                                    PyErr_Format(PyExc_TypeError, c"sequence item %zd: expected a bytes-like object, %.80s found".as_ptr(), i, (*Py_TYPE!(item)).tp_name);
                                    break '__join_6;
                                }
                                PyErr_SetString(
                                    PyExc_RuntimeError,
                                    c"sequence changed size during iteration".as_ptr(),
                                );
                                break '__join_6;
                            }
                            PyErr_SetString(
                                PyExc_OverflowError,
                                c"join() result is too long".as_ptr(),
                            );
                            break '__join_6;
                        }
                        PyErr_SetString(PyExc_OverflowError, c"join() result is too long".as_ptr());
                    }
                    res = core::ptr::null_mut::<_object>();
                }
                Py_DECREF!(seq);
                i = 0;
                while i < nbufs {
                    PyBuffer_Release(buffers.wrapping_offset(i as isize));
                    i = i.wrapping_add(1);
                }
                if buffers != ((*__slate_slot_25).as_mut_ptr() as *mut __SlateRecord_pybuffer_h_33)
                {
                    PyMem_Free(buffers as *mut ());
                }
                return res;
            }
        }
    }
}

fn stringlib_partition(
    str_obj: *mut _object,
    str: *const i8,
    str_len: i64,
    sep_obj: *mut _object,
    sep: *const i8,
    sep_len: i64,
) -> *mut _object {
    let mut pos: i64 = 0;
    if sep_len == 0 {
        unsafe { PyErr_SetString(PyExc_ValueError, c"empty separator".as_ptr()) };
        return core::ptr::null_mut::<_object>();
    }
    let out: *mut _object = unsafe { PyTuple_New(3i64) };
    if out.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    pos = fastsearch(str, str_len, sep, sep_len, 1i32.wrapping_neg() as i64, 1i32);
    if pos < 0 {
        Py_INCREF!(str_obj);
        PyTuple_SET_ITEM!(out, 0i32, str_obj);
        let empty: *mut _object = STRINGLIB_GET_EMPTY__2!();
        Py_INCREF!(empty);
        PyTuple_SET_ITEM!(out, 1i32, empty);
        Py_INCREF!(empty);
        PyTuple_SET_ITEM!(out, 2i32, empty);
        return out;
    }
    PyTuple_SET_ITEM!(out, 0i32, PyBytes_FromStringAndSize(str, pos));
    Py_INCREF!(sep_obj);
    PyTuple_SET_ITEM!(out, 1i32, sep_obj);
    pos = pos.wrapping_add(sep_len);
    PyTuple_SET_ITEM!(
        out,
        2i32,
        PyBytes_FromStringAndSize(str.wrapping_offset(pos as isize), str_len.wrapping_sub(pos))
    );
    if !unsafe { PyErr_Occurred() }.is_null() {
        Py_DECREF!(out);
        return core::ptr::null_mut::<_object>();
    }
    out
}

fn stringlib_rpartition(
    str_obj: *mut _object,
    str: *const i8,
    str_len: i64,
    sep_obj: *mut _object,
    sep: *const i8,
    sep_len: i64,
) -> *mut _object {
    let mut pos: i64 = 0;
    if sep_len == 0 {
        unsafe { PyErr_SetString(PyExc_ValueError, c"empty separator".as_ptr()) };
        return core::ptr::null_mut::<_object>();
    }
    let out: *mut _object = unsafe { PyTuple_New(3i64) };
    if out.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    pos = fastsearch(str, str_len, sep, sep_len, 1i32.wrapping_neg() as i64, 2i32);
    if pos < 0 {
        let empty: *mut _object = STRINGLIB_GET_EMPTY__2!();
        Py_INCREF!(empty);
        PyTuple_SET_ITEM!(out, 0i32, empty);
        Py_INCREF!(empty);
        PyTuple_SET_ITEM!(out, 1i32, empty);
        Py_INCREF!(str_obj);
        PyTuple_SET_ITEM!(out, 2i32, str_obj);
        return out;
    }
    PyTuple_SET_ITEM!(out, 0i32, PyBytes_FromStringAndSize(str, pos));
    Py_INCREF!(sep_obj);
    PyTuple_SET_ITEM!(out, 1i32, sep_obj);
    pos = pos.wrapping_add(sep_len);
    PyTuple_SET_ITEM!(
        out,
        2i32,
        PyBytes_FromStringAndSize(str.wrapping_offset(pos as isize), str_len.wrapping_sub(pos))
    );
    if !unsafe { PyErr_Occurred() }.is_null() {
        Py_DECREF!(out);
        return core::ptr::null_mut::<_object>();
    }
    out
}

fn stringlib_split_whitespace(
    str_obj: *mut _object,
    str: *const i8,
    str_len: i64,
    mut maxcount: i64,
) -> *mut _object {
    let mut __v10: bool = unsafe { core::mem::zeroed() };
    let mut sub: *mut _object = unsafe { core::mem::zeroed() };
    let mut list: *mut _object = unsafe { core::mem::zeroed() };
    let mut count: i64 = unsafe { core::mem::zeroed() };
    let mut j: i64 = unsafe { core::mem::zeroed() };
    let mut i: i64 = unsafe { core::mem::zeroed() };
    unsafe {
        count = 0;
        list = PyList_New(PREALLOC_SIZE!(maxcount));
        if list.is_null() {
            return core::ptr::null_mut::<_object>();
        } else {
            j = 0;
            i = 0;
            '__join_0: {
                '__join_14: {
                    '__join_23: {
                        loop {
                            let __v5: i64 = maxcount;
                            maxcount = __v5.wrapping_sub(1);
                            if __v5 > 0 {
                                while i < str_len
                                    && _Py_ctype_table[(Py_CHARMASK!(unsafe {
                                        *str.wrapping_offset(i as isize)
                                    })
                                        as i32)
                                        as usize]
                                        & 8
                                        != 0
                                {
                                    i = i.wrapping_add(1);
                                }
                                if i == str_len {
                                    break '__join_14;
                                } else {
                                    j = i;
                                    i = i.wrapping_add(1);
                                    while i < str_len
                                        && _Py_ctype_table[(Py_CHARMASK!(unsafe {
                                            *str.wrapping_offset(i as isize)
                                        })
                                            as i32)
                                            as usize]
                                            & 8
                                            == 0
                                    {
                                        i = i.wrapping_add(1);
                                    }
                                    if j == 0 && i == str_len {
                                        __v10 = Py_IS_TYPE!(
                                            str_obj,
                                            core::ptr::addr_of_mut!(PyBytes_Type)
                                        ) != 0;
                                    } else {
                                        __v10 = false;
                                    }
                                    if __v10 {
                                        break '__join_23;
                                    } else {
                                        sub = PyBytes_FromStringAndSize(
                                            str.wrapping_offset(j as isize),
                                            i.wrapping_sub(j),
                                        );
                                        if sub.is_null() {
                                            break '__join_0;
                                        } else {
                                            if count < 12 {
                                                PyList_SET_ITEM!(list, count, sub);
                                            } else {
                                                if PyList_Append(list, sub) != 0 {
                                                    break;
                                                } else {
                                                    Py_DECREF!(sub);
                                                }
                                            }
                                            count = count.wrapping_add(1);
                                        }
                                    }
                                }
                            } else {
                                break '__join_14;
                            }
                        }
                        Py_DECREF!(sub);
                        break '__join_0;
                    }
                    // No whitespace in str_obj, so just use it as list[0]
                    Py_INCREF!(str_obj);
                    PyList_SET_ITEM__2!(list, 0i32, str_obj);
                    count = count.wrapping_add(1);
                }
                if i < str_len {
                    // Only occurs when maxcount was reached
                    //
                    // Skip any remaining whitespace and copy to end of string
                    while i < str_len
                        && _Py_ctype_table[(Py_CHARMASK!(unsafe {
                            *str.wrapping_offset(i as isize)
                        }) as i32) as usize]
                            & 8
                            != 0
                    {
                        i = i.wrapping_add(1);
                    }
                    if i != str_len {
                        sub = PyBytes_FromStringAndSize(
                            str.wrapping_offset(i as isize),
                            str_len.wrapping_sub(i),
                        );
                        if sub.is_null() {
                            break '__join_0;
                        } else {
                            if count < 12 {
                                PyList_SET_ITEM!(list, count, sub);
                            } else {
                                if PyList_Append(list, sub) != 0 {
                                    Py_DECREF!(sub);
                                    break '__join_0;
                                } else {
                                    Py_DECREF!(sub);
                                }
                            }
                            count = count.wrapping_add(1);
                        }
                    }
                }
                FIX_PREALLOC_SIZE!(list, count);
                return list;
            }
            Py_DECREF!(list);
            return core::ptr::null_mut::<_object>();
        }
    }
}

fn stringlib_split_char(
    str_obj: *mut _object,
    str: *const i8,
    str_len: i64,
    ch: i8,
    mut maxcount: i64,
) -> *mut _object {
    let mut __v4: bool = unsafe { core::mem::zeroed() };
    let mut __v7: bool = unsafe { core::mem::zeroed() };
    let mut sub: *mut _object = unsafe { core::mem::zeroed() };
    let mut list: *mut _object = unsafe { core::mem::zeroed() };
    let mut count: i64 = unsafe { core::mem::zeroed() };
    let mut j: i64 = unsafe { core::mem::zeroed() };
    let mut i: i64 = unsafe { core::mem::zeroed() };
    unsafe {
        count = 0;
        list = PyList_New(PREALLOC_SIZE!(maxcount));
        if list.is_null() {
            return core::ptr::null_mut::<_object>();
        } else {
            j = 0;
            i = 0;
            '__join_0: {
                '__join_23: {
                    '__loop_18: loop {
                        if j < str_len {
                            let __v6: i64 = maxcount;
                            maxcount = __v6.wrapping_sub(1);
                            __v7 = __v6 > 0;
                        } else {
                            __v7 = false;
                        }
                        if __v7 {
                            loop {
                                if j < str_len {
                                    // I found that using memchr makes no difference
                                    if (*str.wrapping_offset(j as isize) as i32) == (ch as i32) {
                                        break;
                                    } else {
                                        j = j.wrapping_add(1);
                                    }
                                } else {
                                    continue '__loop_18;
                                }
                            }
                            sub = PyBytes_FromStringAndSize(
                                str.wrapping_offset(i as isize),
                                j.wrapping_sub(i),
                            );
                            if sub.is_null() {
                                break '__join_0;
                            } else {
                                if count < 12 {
                                    PyList_SET_ITEM!(list, count, sub);
                                } else {
                                    if PyList_Append(list, sub) != 0 {
                                        break '__join_23;
                                    } else {
                                        Py_DECREF!(sub);
                                    }
                                }
                                count = count.wrapping_add(1);
                                let __v10: i64 = j.wrapping_add(1);
                                j = __v10;
                                i = __v10;
                            }
                        } else {
                            break;
                        }
                    }
                    if count == 0 {
                        __v4 = Py_IS_TYPE!(str_obj, core::ptr::addr_of_mut!(PyBytes_Type)) != 0;
                    } else {
                        __v4 = false;
                    }
                    if __v4 {
                        // ch not in str_obj, so just use str_obj as list[0]
                        Py_INCREF!(str_obj);
                        PyList_SET_ITEM__2!(list, 0i32, str_obj);
                        count = count.wrapping_add(1);
                    } else {
                        if i <= str_len {
                            sub = PyBytes_FromStringAndSize(
                                str.wrapping_offset(i as isize),
                                str_len.wrapping_sub(i),
                            );
                            if sub.is_null() {
                                break '__join_0;
                            } else {
                                if count < 12 {
                                    PyList_SET_ITEM!(list, count, sub);
                                } else {
                                    if PyList_Append(list, sub) != 0 {
                                        Py_DECREF!(sub);
                                        break '__join_0;
                                    } else {
                                        Py_DECREF!(sub);
                                    }
                                }
                                count = count.wrapping_add(1);
                            }
                        }
                    }
                    FIX_PREALLOC_SIZE!(list, count);
                    return list;
                }
                Py_DECREF!(sub);
            }
            Py_DECREF!(list);
            return core::ptr::null_mut::<_object>();
        }
    }
}

fn stringlib_split(
    str_obj: *mut _object,
    str: *const i8,
    str_len: i64,
    sep: *const i8,
    sep_len: i64,
    mut maxcount: i64,
) -> *mut _object {
    let mut __v4: bool = unsafe { core::mem::zeroed() };
    let mut sub: *mut _object = unsafe { core::mem::zeroed() };
    let mut list: *mut _object = unsafe { core::mem::zeroed() };
    let mut count: i64 = unsafe { core::mem::zeroed() };
    let mut pos: i64 = unsafe { core::mem::zeroed() };
    let mut j: i64 = unsafe { core::mem::zeroed() };
    let mut i: i64 = unsafe { core::mem::zeroed() };
    unsafe {
        count = 0;
        if sep_len == 0 {
            PyErr_SetString(PyExc_ValueError, c"empty separator".as_ptr());
            return core::ptr::null_mut::<_object>();
        } else {
            if sep_len == 1 {
                return stringlib_split_char(
                    str_obj,
                    str,
                    str_len,
                    *sep.wrapping_offset(0 as isize),
                    maxcount,
                );
            } else {
                list = PyList_New(PREALLOC_SIZE!(maxcount));
                if list.is_null() {
                    return core::ptr::null_mut::<_object>();
                } else {
                    j = 0;
                    i = 0;
                    '__join_0: {
                        '__join_13: {
                            loop {
                                let __v6: i64 = maxcount;
                                maxcount = __v6.wrapping_sub(1);
                                if __v6 > 0 {
                                    pos = fastsearch(
                                        str.wrapping_offset(i as isize),
                                        str_len.wrapping_sub(i),
                                        sep,
                                        sep_len,
                                        1i32.wrapping_neg() as i64,
                                        1i32,
                                    );
                                    if pos < 0 {
                                        break '__join_13;
                                    } else {
                                        j = i.wrapping_add(pos);
                                        sub = PyBytes_FromStringAndSize(
                                            str.wrapping_offset(i as isize),
                                            j.wrapping_sub(i),
                                        );
                                        if sub.is_null() {
                                            break '__join_0;
                                        } else {
                                            if count < 12 {
                                                PyList_SET_ITEM!(list, count, sub);
                                            } else {
                                                if PyList_Append(list, sub) != 0 {
                                                    break;
                                                } else {
                                                    Py_DECREF!(sub);
                                                }
                                            }
                                            count = count.wrapping_add(1);
                                            i = j.wrapping_add(sep_len);
                                        }
                                    }
                                } else {
                                    break '__join_13;
                                }
                            }
                            Py_DECREF!(sub);
                            break '__join_0;
                        }
                        if count == 0 {
                            __v4 = Py_IS_TYPE!(str_obj, core::ptr::addr_of_mut!(PyBytes_Type)) != 0;
                        } else {
                            __v4 = false;
                        }
                        if __v4 {
                            // No match in str_obj, so just use it as list[0]
                            Py_INCREF!(str_obj);
                            PyList_SET_ITEM__2!(list, 0i32, str_obj);
                            count = count.wrapping_add(1);
                        } else {
                            sub = PyBytes_FromStringAndSize(
                                str.wrapping_offset(i as isize),
                                str_len.wrapping_sub(i),
                            );
                            if sub.is_null() {
                                break '__join_0;
                            } else {
                                if count < 12 {
                                    PyList_SET_ITEM!(list, count, sub);
                                } else {
                                    if PyList_Append(list, sub) != 0 {
                                        Py_DECREF!(sub);
                                        break '__join_0;
                                    } else {
                                        Py_DECREF!(sub);
                                    }
                                }
                                count = count.wrapping_add(1);
                            }
                        }
                        FIX_PREALLOC_SIZE!(list, count);
                        return list;
                    }
                    Py_DECREF!(list);
                    return core::ptr::null_mut::<_object>();
                }
            }
        }
    }
}

fn stringlib_rsplit_whitespace(
    str_obj: *mut _object,
    str: *const i8,
    str_len: i64,
    mut maxcount: i64,
) -> *mut _object {
    let mut __v10: bool = unsafe { core::mem::zeroed() };
    let mut sub: *mut _object = unsafe { core::mem::zeroed() };
    let mut list: *mut _object = unsafe { core::mem::zeroed() };
    let mut count: i64 = unsafe { core::mem::zeroed() };
    let mut j: i64 = unsafe { core::mem::zeroed() };
    let mut i: i64 = unsafe { core::mem::zeroed() };
    unsafe {
        count = 0;
        list = PyList_New(PREALLOC_SIZE!(maxcount));
        if list.is_null() {
            return core::ptr::null_mut::<_object>();
        } else {
            let __v17: i64 = str_len.wrapping_sub(1);
            j = __v17;
            i = __v17;
            '__join_0: {
                '__join_15: {
                    '__join_24: {
                        loop {
                            let __v5: i64 = maxcount;
                            maxcount = __v5.wrapping_sub(1);
                            if __v5 > 0 {
                                while i >= 0
                                    && _Py_ctype_table[(Py_CHARMASK!(unsafe {
                                        *str.wrapping_offset(i as isize)
                                    })
                                        as i32)
                                        as usize]
                                        & 8
                                        != 0
                                {
                                    i = i.wrapping_sub(1);
                                }
                                if i < 0 {
                                    break '__join_15;
                                } else {
                                    j = i;
                                    i = i.wrapping_sub(1);
                                    while i >= 0
                                        && _Py_ctype_table[(Py_CHARMASK!(unsafe {
                                            *str.wrapping_offset(i as isize)
                                        })
                                            as i32)
                                            as usize]
                                            & 8
                                            == 0
                                    {
                                        i = i.wrapping_sub(1);
                                    }
                                    if j == str_len.wrapping_sub(1) && i < 0 {
                                        __v10 = Py_IS_TYPE!(
                                            str_obj,
                                            core::ptr::addr_of_mut!(PyBytes_Type)
                                        ) != 0;
                                    } else {
                                        __v10 = false;
                                    }
                                    if __v10 {
                                        break '__join_24;
                                    } else {
                                        sub = PyBytes_FromStringAndSize(
                                            str.wrapping_offset(i.wrapping_add(1) as isize),
                                            j.wrapping_add(1).wrapping_sub(i.wrapping_add(1)),
                                        );
                                        if sub.is_null() {
                                            break '__join_0;
                                        } else {
                                            if count < 12 {
                                                PyList_SET_ITEM!(list, count, sub);
                                            } else {
                                                if PyList_Append(list, sub) != 0 {
                                                    break;
                                                } else {
                                                    Py_DECREF!(sub);
                                                }
                                            }
                                            count = count.wrapping_add(1);
                                        }
                                    }
                                }
                            } else {
                                break '__join_15;
                            }
                        }
                        Py_DECREF!(sub);
                        break '__join_0;
                    }
                    // No whitespace in str_obj, so just use it as list[0]
                    Py_INCREF!(str_obj);
                    PyList_SET_ITEM__2!(list, 0i32, str_obj);
                    count = count.wrapping_add(1);
                }
                if i >= 0 {
                    // Only occurs when maxcount was reached
                    //
                    // Skip any remaining whitespace and copy to beginning of string
                    while i >= 0
                        && _Py_ctype_table[(Py_CHARMASK!(unsafe {
                            *str.wrapping_offset(i as isize)
                        }) as i32) as usize]
                            & 8
                            != 0
                    {
                        i = i.wrapping_sub(1);
                    }
                    if i >= 0 {
                        sub = PyBytes_FromStringAndSize(
                            str.wrapping_offset(0 as isize),
                            i.wrapping_add(1).wrapping_sub(0),
                        );
                        if sub.is_null() {
                            break '__join_0;
                        } else {
                            if count < 12 {
                                PyList_SET_ITEM!(list, count, sub);
                            } else {
                                if PyList_Append(list, sub) != 0 {
                                    Py_DECREF!(sub);
                                    break '__join_0;
                                } else {
                                    Py_DECREF!(sub);
                                }
                            }
                            count = count.wrapping_add(1);
                        }
                    }
                }
                FIX_PREALLOC_SIZE!(list, count);
                if PyList_Reverse(list) >= 0 {
                    return list;
                }
            }
            Py_DECREF!(list);
            return core::ptr::null_mut::<_object>();
        }
    }
}

fn stringlib_rsplit_char(
    str_obj: *mut _object,
    str: *const i8,
    str_len: i64,
    ch: i8,
    mut maxcount: i64,
) -> *mut _object {
    let mut __v4: bool = unsafe { core::mem::zeroed() };
    let mut __v7: bool = unsafe { core::mem::zeroed() };
    let mut sub: *mut _object = unsafe { core::mem::zeroed() };
    let mut list: *mut _object = unsafe { core::mem::zeroed() };
    let mut count: i64 = unsafe { core::mem::zeroed() };
    let mut j: i64 = unsafe { core::mem::zeroed() };
    let mut i: i64 = unsafe { core::mem::zeroed() };
    unsafe {
        count = 0;
        list = PyList_New(PREALLOC_SIZE!(maxcount));
        if list.is_null() {
            return core::ptr::null_mut::<_object>();
        } else {
            let __v13: i64 = str_len.wrapping_sub(1);
            j = __v13;
            i = __v13;
            '__join_0: {
                '__join_24: {
                    '__loop_19: loop {
                        if i >= 0 {
                            let __v6: i64 = maxcount;
                            maxcount = __v6.wrapping_sub(1);
                            __v7 = __v6 > 0;
                        } else {
                            __v7 = false;
                        }
                        if __v7 {
                            loop {
                                if i >= 0 {
                                    if (*str.wrapping_offset(i as isize) as i32) == (ch as i32) {
                                        break;
                                    } else {
                                        i = i.wrapping_sub(1);
                                    }
                                } else {
                                    continue '__loop_19;
                                }
                            }
                            sub = PyBytes_FromStringAndSize(
                                str.wrapping_offset(i.wrapping_add(1) as isize),
                                j.wrapping_add(1).wrapping_sub(i.wrapping_add(1)),
                            );
                            if sub.is_null() {
                                break '__join_0;
                            } else {
                                if count < 12 {
                                    PyList_SET_ITEM!(list, count, sub);
                                } else {
                                    if PyList_Append(list, sub) != 0 {
                                        break '__join_24;
                                    } else {
                                        Py_DECREF!(sub);
                                    }
                                }
                                count = count.wrapping_add(1);
                                let __v10: i64 = i.wrapping_sub(1);
                                i = __v10;
                                j = __v10;
                            }
                        } else {
                            break;
                        }
                    }
                    if count == 0 {
                        __v4 = Py_IS_TYPE!(str_obj, core::ptr::addr_of_mut!(PyBytes_Type)) != 0;
                    } else {
                        __v4 = false;
                    }
                    if __v4 {
                        // ch not in str_obj, so just use str_obj as list[0]
                        Py_INCREF!(str_obj);
                        PyList_SET_ITEM__2!(list, 0i32, str_obj);
                        count = count.wrapping_add(1);
                    } else {
                        if j >= (1i32.wrapping_neg() as i64) {
                            sub = PyBytes_FromStringAndSize(
                                str.wrapping_offset(0 as isize),
                                j.wrapping_add(1).wrapping_sub(0),
                            );
                            if sub.is_null() {
                                break '__join_0;
                            } else {
                                if count < 12 {
                                    PyList_SET_ITEM!(list, count, sub);
                                } else {
                                    if PyList_Append(list, sub) != 0 {
                                        Py_DECREF!(sub);
                                        break '__join_0;
                                    } else {
                                        Py_DECREF!(sub);
                                    }
                                }
                                count = count.wrapping_add(1);
                            }
                        }
                    }
                    FIX_PREALLOC_SIZE!(list, count);
                    if PyList_Reverse(list) < 0 {
                        break '__join_0;
                    } else {
                        return list;
                    }
                }
                Py_DECREF!(sub);
            }
            Py_DECREF!(list);
            return core::ptr::null_mut::<_object>();
        }
    }
}

fn stringlib_rsplit(
    str_obj: *mut _object,
    str: *const i8,
    str_len: i64,
    sep: *const i8,
    sep_len: i64,
    mut maxcount: i64,
) -> *mut _object {
    let mut __v4: bool = unsafe { core::mem::zeroed() };
    let mut sub: *mut _object = unsafe { core::mem::zeroed() };
    let mut list: *mut _object = unsafe { core::mem::zeroed() };
    let mut count: i64 = unsafe { core::mem::zeroed() };
    let mut pos: i64 = unsafe { core::mem::zeroed() };
    let mut j: i64 = unsafe { core::mem::zeroed() };
    unsafe {
        count = 0;
        if sep_len == 0 {
            PyErr_SetString(PyExc_ValueError, c"empty separator".as_ptr());
            return core::ptr::null_mut::<_object>();
        } else {
            if sep_len == 1 {
                return stringlib_rsplit_char(
                    str_obj,
                    str,
                    str_len,
                    *sep.wrapping_offset(0 as isize),
                    maxcount,
                );
            } else {
                list = PyList_New(PREALLOC_SIZE!(maxcount));
                if list.is_null() {
                    return core::ptr::null_mut::<_object>();
                } else {
                    j = str_len;
                    '__join_0: {
                        '__join_14: {
                            loop {
                                let __v6: i64 = maxcount;
                                maxcount = __v6.wrapping_sub(1);
                                if __v6 > 0 {
                                    pos = fastsearch(
                                        str,
                                        j,
                                        sep,
                                        sep_len,
                                        1i32.wrapping_neg() as i64,
                                        2i32,
                                    );
                                    if pos < 0 {
                                        break '__join_14;
                                    } else {
                                        sub = PyBytes_FromStringAndSize(
                                            str.wrapping_offset(pos.wrapping_add(sep_len) as isize),
                                            j.wrapping_sub(pos.wrapping_add(sep_len)),
                                        );
                                        if sub.is_null() {
                                            break '__join_0;
                                        } else {
                                            if count < 12 {
                                                PyList_SET_ITEM!(list, count, sub);
                                            } else {
                                                if PyList_Append(list, sub) != 0 {
                                                    break;
                                                } else {
                                                    Py_DECREF!(sub);
                                                }
                                            }
                                            count = count.wrapping_add(1);
                                            j = pos;
                                        }
                                    }
                                } else {
                                    break '__join_14;
                                }
                            }
                            Py_DECREF!(sub);
                            break '__join_0;
                        }
                        if count == 0 {
                            __v4 = Py_IS_TYPE!(str_obj, core::ptr::addr_of_mut!(PyBytes_Type)) != 0;
                        } else {
                            __v4 = false;
                        }
                        if __v4 {
                            // No match in str_obj, so just use it as list[0]
                            Py_INCREF!(str_obj);
                            PyList_SET_ITEM__2!(list, 0i32, str_obj);
                            count = count.wrapping_add(1);
                        } else {
                            sub = PyBytes_FromStringAndSize(
                                str.wrapping_offset(0 as isize),
                                j.wrapping_sub(0),
                            );
                            if sub.is_null() {
                                break '__join_0;
                            } else {
                                if count < 12 {
                                    PyList_SET_ITEM!(list, count, sub);
                                } else {
                                    if PyList_Append(list, sub) != 0 {
                                        Py_DECREF!(sub);
                                        break '__join_0;
                                    } else {
                                        Py_DECREF!(sub);
                                    }
                                }
                                count = count.wrapping_add(1);
                            }
                        }
                        FIX_PREALLOC_SIZE!(list, count);
                        if PyList_Reverse(list) >= 0 {
                            return list;
                        }
                    }
                    Py_DECREF!(list);
                    return core::ptr::null_mut::<_object>();
                }
            }
        }
    }
}

fn stringlib_splitlines(
    str_obj: *mut _object,
    str: *const i8,
    str_len: i64,
    keepends: i32,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut eol: i64 = unsafe { core::mem::zeroed() };
    let mut sub: *mut _object = unsafe { core::mem::zeroed() };
    let mut list: *mut _object = unsafe { core::mem::zeroed() };
    let mut j: i64 = unsafe { core::mem::zeroed() };
    let mut i: i64 = unsafe { core::mem::zeroed() };
    unsafe {
        // This does not use the preallocated list because splitlines is
        // usually run with hundreds of newlines.  The overhead of
        // switching between PyList_SET_ITEM and append causes about a
        // 2-3% slowdown for that common case.  A smarter implementation
        // could move the if check out, so the SET_ITEMs are done first
        // and the appends only done when the prealloc buffer is full.
        // That's too much work for little gain.
        list = PyList_New(0i64);
        if list.is_null() {
            return core::ptr::null_mut::<_object>();
        } else {
            j = 0;
            i = 0;
            '__join_0: {
                '__join_1: {
                    '__join_7: {
                        loop {
                            if i < str_len {
                                // Find a line and append it
                                while i < str_len
                                    && !STRINGLIB_ISLINEBREAK!(unsafe {
                                        *str.wrapping_offset(i as isize)
                                    })
                                {
                                    i = i.wrapping_add(1);
                                }
                                // Skip the line break reading CRLF as one line break
                                eol = i;
                                if i < str_len {
                                    if (*str.wrapping_offset(i as isize) as i32) == 13
                                        && i.wrapping_add(1) < str_len
                                        && (*str.wrapping_offset(i.wrapping_add(1) as isize) as i32)
                                            == 10
                                    {
                                        i = i.wrapping_add(2);
                                    } else {
                                        i = i.wrapping_add(1);
                                    }
                                    if keepends != 0 {
                                        eol = i;
                                    }
                                }
                                if j == 0 && eol == str_len {
                                    __v0 =
                                        Py_IS_TYPE!(str_obj, core::ptr::addr_of_mut!(PyBytes_Type))
                                            != 0;
                                } else {
                                    __v0 = false;
                                }
                                if __v0 {
                                    break '__join_7;
                                } else {
                                    sub = PyBytes_FromStringAndSize(
                                        str.wrapping_offset(j as isize),
                                        eol.wrapping_sub(j),
                                    );
                                    if sub.is_null() {
                                        break '__join_0;
                                    } else {
                                        if PyList_Append(list, sub) != 0 {
                                            break;
                                        } else {
                                            Py_DECREF!(sub);
                                            j = i;
                                        }
                                    }
                                }
                            } else {
                                break '__join_1;
                            }
                        }
                        Py_DECREF!(sub);
                        break '__join_0;
                    }
                    // No linebreak in str_obj, so just use it as list[0]
                    if PyList_Append(list, str_obj) != 0 {
                        break '__join_0;
                    }
                }
                return list;
            }
            Py_DECREF!(list);
            return core::ptr::null_mut::<_object>();
        }
    }
}

#[unsafe(link_section = ".text.slate_distinct.bytesobject.stringlib_isspace")]
extern "C-unwind" fn stringlib_isspace(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    unsafe {
        _Py_bytes_isspace(
            PyBytes_AS_STRING(_PyObject_CAST!(self_)) as *const i8,
            PyBytes_GET_SIZE(_PyObject_CAST!(self_)),
        )
    }
}

#[unsafe(link_section = ".text.slate_distinct.bytesobject.stringlib_isalpha")]
extern "C-unwind" fn stringlib_isalpha(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    unsafe {
        _Py_bytes_isalpha(
            PyBytes_AS_STRING(_PyObject_CAST!(self_)) as *const i8,
            PyBytes_GET_SIZE(_PyObject_CAST!(self_)),
        )
    }
}

#[unsafe(link_section = ".text.slate_distinct.bytesobject.stringlib_isalnum")]
extern "C-unwind" fn stringlib_isalnum(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    unsafe {
        _Py_bytes_isalnum(
            PyBytes_AS_STRING(_PyObject_CAST!(self_)) as *const i8,
            PyBytes_GET_SIZE(_PyObject_CAST!(self_)),
        )
    }
}

#[unsafe(link_section = ".text.slate_distinct.bytesobject.stringlib_isascii")]
extern "C-unwind" fn stringlib_isascii(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    unsafe {
        _Py_bytes_isascii(
            PyBytes_AS_STRING(_PyObject_CAST!(self_)) as *const i8,
            PyBytes_GET_SIZE(_PyObject_CAST!(self_)),
        )
    }
}

#[unsafe(link_section = ".text.slate_distinct.bytesobject.stringlib_isdigit")]
extern "C-unwind" fn stringlib_isdigit(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    unsafe {
        _Py_bytes_isdigit(
            PyBytes_AS_STRING(_PyObject_CAST!(self_)) as *const i8,
            PyBytes_GET_SIZE(_PyObject_CAST!(self_)),
        )
    }
}

#[unsafe(link_section = ".text.slate_distinct.bytesobject.stringlib_islower")]
extern "C-unwind" fn stringlib_islower(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    unsafe {
        _Py_bytes_islower(
            PyBytes_AS_STRING(_PyObject_CAST!(self_)) as *const i8,
            PyBytes_GET_SIZE(_PyObject_CAST!(self_)),
        )
    }
}

#[unsafe(link_section = ".text.slate_distinct.bytesobject.stringlib_isupper")]
extern "C-unwind" fn stringlib_isupper(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    unsafe {
        _Py_bytes_isupper(
            PyBytes_AS_STRING(_PyObject_CAST!(self_)) as *const i8,
            PyBytes_GET_SIZE(_PyObject_CAST!(self_)),
        )
    }
}

#[unsafe(link_section = ".text.slate_distinct.bytesobject.stringlib_istitle")]
extern "C-unwind" fn stringlib_istitle(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    unsafe {
        _Py_bytes_istitle(
            PyBytes_AS_STRING(_PyObject_CAST!(self_)) as *const i8,
            PyBytes_GET_SIZE(_PyObject_CAST!(self_)),
        )
    }
}

#[unsafe(link_section = ".text.slate_distinct.bytesobject.stringlib_lower")]
extern "C-unwind" fn stringlib_lower(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    let newobj: *mut _object = PyBytes_FromStringAndSize(
        core::ptr::null::<i8>(),
        PyBytes_GET_SIZE(_PyObject_CAST!(self_)),
    );
    if newobj.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    unsafe {
        _Py_bytes_lower(
            PyBytes_AS_STRING(_PyObject_CAST!(newobj)),
            PyBytes_AS_STRING(_PyObject_CAST!(self_)) as *const i8,
            PyBytes_GET_SIZE(_PyObject_CAST!(self_)),
        )
    };
    newobj
}

#[unsafe(link_section = ".text.slate_distinct.bytesobject.stringlib_upper")]
extern "C-unwind" fn stringlib_upper(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    let newobj: *mut _object = PyBytes_FromStringAndSize(
        core::ptr::null::<i8>(),
        PyBytes_GET_SIZE(_PyObject_CAST!(self_)),
    );
    if newobj.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    unsafe {
        _Py_bytes_upper(
            PyBytes_AS_STRING(_PyObject_CAST!(newobj)),
            PyBytes_AS_STRING(_PyObject_CAST!(self_)) as *const i8,
            PyBytes_GET_SIZE(_PyObject_CAST!(self_)),
        )
    };
    newobj
}

#[unsafe(link_section = ".text.slate_distinct.bytesobject.stringlib_title")]
extern "C-unwind" fn stringlib_title(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    let newobj: *mut _object = PyBytes_FromStringAndSize(
        core::ptr::null::<i8>(),
        PyBytes_GET_SIZE(_PyObject_CAST!(self_)),
    );
    if newobj.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    unsafe {
        _Py_bytes_title(
            PyBytes_AS_STRING(_PyObject_CAST!(newobj)),
            PyBytes_AS_STRING(_PyObject_CAST!(self_)) as *const i8,
            PyBytes_GET_SIZE(_PyObject_CAST!(self_)),
        )
    };
    newobj
}

#[unsafe(link_section = ".text.slate_distinct.bytesobject.stringlib_capitalize")]
extern "C-unwind" fn stringlib_capitalize(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    let newobj: *mut _object = PyBytes_FromStringAndSize(
        core::ptr::null::<i8>(),
        PyBytes_GET_SIZE(_PyObject_CAST!(self_)),
    );
    if newobj.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    unsafe {
        _Py_bytes_capitalize(
            PyBytes_AS_STRING(_PyObject_CAST!(newobj)),
            PyBytes_AS_STRING(_PyObject_CAST!(self_)) as *const i8,
            PyBytes_GET_SIZE(_PyObject_CAST!(self_)),
        )
    };
    newobj
}

#[unsafe(link_section = ".text.slate_distinct.bytesobject.stringlib_swapcase")]
extern "C-unwind" fn stringlib_swapcase(
    self_: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    let newobj: *mut _object = PyBytes_FromStringAndSize(
        core::ptr::null::<i8>(),
        PyBytes_GET_SIZE(_PyObject_CAST!(self_)),
    );
    if newobj.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    unsafe {
        _Py_bytes_swapcase(
            PyBytes_AS_STRING(_PyObject_CAST!(newobj)),
            PyBytes_AS_STRING(_PyObject_CAST!(self_)) as *const i8,
            PyBytes_GET_SIZE(_PyObject_CAST!(self_)),
        )
    };
    newobj
}

static stringlib_expandtabs__doc__: __SlateAlign16<[i8; 167]> = __SlateAlign16([
    101i8, 120i8, 112i8, 97i8, 110i8, 100i8, 116i8, 97i8, 98i8, 115i8, 40i8, 36i8, 115i8, 101i8,
    108i8, 102i8, 44i8, 32i8, 47i8, 44i8, 32i8, 116i8, 97i8, 98i8, 115i8, 105i8, 122i8, 101i8,
    61i8, 56i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8,
    97i8, 32i8, 99i8, 111i8, 112i8, 121i8, 32i8, 119i8, 104i8, 101i8, 114i8, 101i8, 32i8, 97i8,
    108i8, 108i8, 32i8, 116i8, 97i8, 98i8, 32i8, 99i8, 104i8, 97i8, 114i8, 97i8, 99i8, 116i8,
    101i8, 114i8, 115i8, 32i8, 97i8, 114i8, 101i8, 32i8, 101i8, 120i8, 112i8, 97i8, 110i8, 100i8,
    101i8, 100i8, 32i8, 117i8, 115i8, 105i8, 110i8, 103i8, 32i8, 115i8, 112i8, 97i8, 99i8, 101i8,
    115i8, 46i8, 10i8, 10i8, 73i8, 102i8, 32i8, 116i8, 97i8, 98i8, 115i8, 105i8, 122i8, 101i8,
    32i8, 105i8, 115i8, 32i8, 110i8, 111i8, 116i8, 32i8, 103i8, 105i8, 118i8, 101i8, 110i8, 44i8,
    32i8, 97i8, 32i8, 116i8, 97i8, 98i8, 32i8, 115i8, 105i8, 122i8, 101i8, 32i8, 111i8, 102i8,
    32i8, 56i8, 32i8, 99i8, 104i8, 97i8, 114i8, 97i8, 99i8, 116i8, 101i8, 114i8, 115i8, 32i8,
    105i8, 115i8, 32i8, 97i8, 115i8, 115i8, 117i8, 109i8, 101i8, 100i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bytesobject.stringlib_expandtabs")]
extern "C-unwind" fn stringlib_expandtabs(
    self_: *mut _object,
    mut args: *const *mut _object,
    nargs: i64,
    kwnames: *mut _object,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut tabsize: i32 = unsafe { core::mem::zeroed() };
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
            tabsize = 8;
            args = _PyArg_UnpackKeywords!(
                args,
                nargs,
                core::ptr::null_mut::<()>(),
                kwnames,
                core::ptr::addr_of_mut!(_parser__51),
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
                    tabsize = PyLong_AsInt(*args.wrapping_offset(0 as isize));
                    if tabsize == 1i32.wrapping_neg() {
                        __v0 = !PyErr_Occurred().is_null();
                    } else {
                        __v0 = false;
                    }
                    if __v0 {
                        break '__join_0;
                    }
                }
                return_value = stringlib_expandtabs_impl(self_, tabsize);
            }
        }
        return return_value;
    }
}

static mut _kwtuple__38: __SlateRecord_transmogrify_h_h_38 = {
    let mut __t0: __SlateRecord_transmogrify_h_h_38 = unsafe { core::mem::zeroed() };
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
                ._py_tabsize
                ._ascii
                .ob_base
        )
    }];
    __t0
};

static _keywords__50: __SlateAlign16<[*const i8; 2]> = __SlateAlign16([
    (b"tabsize\0".as_ptr() as *mut i8) as *const i8,
    core::ptr::null::<i8>(),
]);

static mut _parser__51: _PyArg_Parser = {
    let mut __t0: _PyArg_Parser = unsafe { core::mem::zeroed() };
    __t0.keywords = unsafe { core::ptr::addr_of!(_keywords__50.0) as *const *const i8 };
    __t0.fname = (b"expandtabs\0".as_ptr() as *mut i8) as *const i8;
    __t0.kwtuple = unsafe { core::ptr::addr_of_mut!(_kwtuple__38.ob_base.ob_base) };
    __t0
};

static stringlib_ljust__doc__: __SlateAlign16<[i8; 143]> = __SlateAlign16([
    108i8, 106i8, 117i8, 115i8, 116i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8, 119i8,
    105i8, 100i8, 116i8, 104i8, 44i8, 32i8, 102i8, 105i8, 108i8, 108i8, 99i8, 104i8, 97i8, 114i8,
    61i8, 98i8, 39i8, 32i8, 39i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8,
    101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 97i8, 32i8, 108i8, 101i8, 102i8, 116i8, 45i8, 106i8,
    117i8, 115i8, 116i8, 105i8, 102i8, 105i8, 101i8, 100i8, 32i8, 115i8, 116i8, 114i8, 105i8,
    110i8, 103i8, 32i8, 111i8, 102i8, 32i8, 108i8, 101i8, 110i8, 103i8, 116i8, 104i8, 32i8, 119i8,
    105i8, 100i8, 116i8, 104i8, 46i8, 10i8, 10i8, 80i8, 97i8, 100i8, 100i8, 105i8, 110i8, 103i8,
    32i8, 105i8, 115i8, 32i8, 100i8, 111i8, 110i8, 101i8, 32i8, 117i8, 115i8, 105i8, 110i8, 103i8,
    32i8, 116i8, 104i8, 101i8, 32i8, 115i8, 112i8, 101i8, 99i8, 105i8, 102i8, 105i8, 101i8, 100i8,
    32i8, 102i8, 105i8, 108i8, 108i8, 32i8, 99i8, 104i8, 97i8, 114i8, 97i8, 99i8, 116i8, 101i8,
    114i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bytesobject.stringlib_ljust")]
extern "C-unwind" fn stringlib_ljust(
    self_: *mut _object,
    args: *const *mut _object,
    nargs: i64,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut iobj: *mut _object = unsafe { core::mem::zeroed() };
    let mut ival: i64 = unsafe { core::mem::zeroed() };
    let mut fillchar: i8 = unsafe { core::mem::zeroed() };
    let mut width: i64 = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_0: {
            return_value = core::ptr::null_mut::<_object>();
            fillchar = 32;
            if _PyArg_CheckPositional!(c"ljust".as_ptr() as *mut i8, nargs, 1i32, 2i32) {
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
                    width = ival;
                    if nargs >= 2 {
                        if PyBytes_Check!(unsafe { *args.wrapping_offset(1 as isize) }) != 0 {
                            if PyBytes_GET_SIZE!(unsafe { *args.wrapping_offset(1 as isize) }) != 1
                            {
                                PyErr_Format(PyExc_TypeError, c"ljust(): argument 2 must be a byte string of length 1, not a bytes object of length %zd".as_ptr(), PyBytes_GET_SIZE!(unsafe { *args.wrapping_offset(1 as isize) }));
                                break '__join_0;
                            } else {
                                fillchar = *PyBytes_AS_STRING!(unsafe {
                                    *args.wrapping_offset(1 as isize)
                                })
                                .wrapping_offset(0 as isize);
                            }
                        } else {
                            if PyByteArray_Check!(unsafe { *args.wrapping_offset(1 as isize) }) != 0
                            {
                                if PyByteArray_GET_SIZE!(unsafe {
                                    *args.wrapping_offset(1 as isize)
                                }) != 1
                                {
                                    PyErr_Format(PyExc_TypeError, c"ljust(): argument 2 must be a byte string of length 1, not a bytearray object of length %zd".as_ptr(), PyByteArray_GET_SIZE!(unsafe { *args.wrapping_offset(1 as isize) }));
                                    break '__join_0;
                                } else {
                                    fillchar = *PyByteArray_AS_STRING__2!(unsafe {
                                        *args.wrapping_offset(1 as isize)
                                    })
                                    .wrapping_offset(0 as isize);
                                }
                            } else {
                                _PyArg_BadArgument(
                                    c"ljust".as_ptr(),
                                    c"argument 2".as_ptr(),
                                    c"a byte string of length 1".as_ptr(),
                                    *args.wrapping_offset(1 as isize),
                                );
                                break '__join_0;
                            }
                        }
                    }
                    return_value = stringlib_ljust_impl(self_, width, fillchar);
                }
            }
        }
        return return_value;
    }
}

static stringlib_rjust__doc__: __SlateAlign16<[i8; 144]> = __SlateAlign16([
    114i8, 106i8, 117i8, 115i8, 116i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8, 119i8,
    105i8, 100i8, 116i8, 104i8, 44i8, 32i8, 102i8, 105i8, 108i8, 108i8, 99i8, 104i8, 97i8, 114i8,
    61i8, 98i8, 39i8, 32i8, 39i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 82i8,
    101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 97i8, 32i8, 114i8, 105i8, 103i8, 104i8, 116i8, 45i8,
    106i8, 117i8, 115i8, 116i8, 105i8, 102i8, 105i8, 101i8, 100i8, 32i8, 115i8, 116i8, 114i8,
    105i8, 110i8, 103i8, 32i8, 111i8, 102i8, 32i8, 108i8, 101i8, 110i8, 103i8, 116i8, 104i8, 32i8,
    119i8, 105i8, 100i8, 116i8, 104i8, 46i8, 10i8, 10i8, 80i8, 97i8, 100i8, 100i8, 105i8, 110i8,
    103i8, 32i8, 105i8, 115i8, 32i8, 100i8, 111i8, 110i8, 101i8, 32i8, 117i8, 115i8, 105i8, 110i8,
    103i8, 32i8, 116i8, 104i8, 101i8, 32i8, 115i8, 112i8, 101i8, 99i8, 105i8, 102i8, 105i8, 101i8,
    100i8, 32i8, 102i8, 105i8, 108i8, 108i8, 32i8, 99i8, 104i8, 97i8, 114i8, 97i8, 99i8, 116i8,
    101i8, 114i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bytesobject.stringlib_rjust")]
extern "C-unwind" fn stringlib_rjust(
    self_: *mut _object,
    args: *const *mut _object,
    nargs: i64,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut iobj: *mut _object = unsafe { core::mem::zeroed() };
    let mut ival: i64 = unsafe { core::mem::zeroed() };
    let mut fillchar: i8 = unsafe { core::mem::zeroed() };
    let mut width: i64 = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_0: {
            return_value = core::ptr::null_mut::<_object>();
            fillchar = 32;
            if _PyArg_CheckPositional!(c"rjust".as_ptr() as *mut i8, nargs, 1i32, 2i32) {
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
                    width = ival;
                    if nargs >= 2 {
                        if PyBytes_Check!(unsafe { *args.wrapping_offset(1 as isize) }) != 0 {
                            if PyBytes_GET_SIZE!(unsafe { *args.wrapping_offset(1 as isize) }) != 1
                            {
                                PyErr_Format(PyExc_TypeError, c"rjust(): argument 2 must be a byte string of length 1, not a bytes object of length %zd".as_ptr(), PyBytes_GET_SIZE!(unsafe { *args.wrapping_offset(1 as isize) }));
                                break '__join_0;
                            } else {
                                fillchar = *PyBytes_AS_STRING!(unsafe {
                                    *args.wrapping_offset(1 as isize)
                                })
                                .wrapping_offset(0 as isize);
                            }
                        } else {
                            if PyByteArray_Check!(unsafe { *args.wrapping_offset(1 as isize) }) != 0
                            {
                                if PyByteArray_GET_SIZE!(unsafe {
                                    *args.wrapping_offset(1 as isize)
                                }) != 1
                                {
                                    PyErr_Format(PyExc_TypeError, c"rjust(): argument 2 must be a byte string of length 1, not a bytearray object of length %zd".as_ptr(), PyByteArray_GET_SIZE!(unsafe { *args.wrapping_offset(1 as isize) }));
                                    break '__join_0;
                                } else {
                                    fillchar = *PyByteArray_AS_STRING__2!(unsafe {
                                        *args.wrapping_offset(1 as isize)
                                    })
                                    .wrapping_offset(0 as isize);
                                }
                            } else {
                                _PyArg_BadArgument(
                                    c"rjust".as_ptr(),
                                    c"argument 2".as_ptr(),
                                    c"a byte string of length 1".as_ptr(),
                                    *args.wrapping_offset(1 as isize),
                                );
                                break '__join_0;
                            }
                        }
                    }
                    return_value = stringlib_rjust_impl(self_, width, fillchar);
                }
            }
        }
        return return_value;
    }
}

static stringlib_center__doc__: __SlateAlign16<[i8; 138]> = __SlateAlign16([
    99i8, 101i8, 110i8, 116i8, 101i8, 114i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8,
    119i8, 105i8, 100i8, 116i8, 104i8, 44i8, 32i8, 102i8, 105i8, 108i8, 108i8, 99i8, 104i8, 97i8,
    114i8, 61i8, 98i8, 39i8, 32i8, 39i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8,
    82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 97i8, 32i8, 99i8, 101i8, 110i8, 116i8, 101i8,
    114i8, 101i8, 100i8, 32i8, 115i8, 116i8, 114i8, 105i8, 110i8, 103i8, 32i8, 111i8, 102i8, 32i8,
    108i8, 101i8, 110i8, 103i8, 116i8, 104i8, 32i8, 119i8, 105i8, 100i8, 116i8, 104i8, 46i8, 10i8,
    10i8, 80i8, 97i8, 100i8, 100i8, 105i8, 110i8, 103i8, 32i8, 105i8, 115i8, 32i8, 100i8, 111i8,
    110i8, 101i8, 32i8, 117i8, 115i8, 105i8, 110i8, 103i8, 32i8, 116i8, 104i8, 101i8, 32i8, 115i8,
    112i8, 101i8, 99i8, 105i8, 102i8, 105i8, 101i8, 100i8, 32i8, 102i8, 105i8, 108i8, 108i8, 32i8,
    99i8, 104i8, 97i8, 114i8, 97i8, 99i8, 116i8, 101i8, 114i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bytesobject.stringlib_center")]
extern "C-unwind" fn stringlib_center(
    self_: *mut _object,
    args: *const *mut _object,
    nargs: i64,
) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut iobj: *mut _object = unsafe { core::mem::zeroed() };
    let mut ival: i64 = unsafe { core::mem::zeroed() };
    let mut fillchar: i8 = unsafe { core::mem::zeroed() };
    let mut width: i64 = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_0: {
            return_value = core::ptr::null_mut::<_object>();
            fillchar = 32;
            if _PyArg_CheckPositional!(c"center".as_ptr() as *mut i8, nargs, 1i32, 2i32) {
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
                    width = ival;
                    if nargs >= 2 {
                        if PyBytes_Check!(unsafe { *args.wrapping_offset(1 as isize) }) != 0 {
                            if PyBytes_GET_SIZE!(unsafe { *args.wrapping_offset(1 as isize) }) != 1
                            {
                                PyErr_Format(PyExc_TypeError, c"center(): argument 2 must be a byte string of length 1, not a bytes object of length %zd".as_ptr(), PyBytes_GET_SIZE!(unsafe { *args.wrapping_offset(1 as isize) }));
                                break '__join_0;
                            } else {
                                fillchar = *PyBytes_AS_STRING!(unsafe {
                                    *args.wrapping_offset(1 as isize)
                                })
                                .wrapping_offset(0 as isize);
                            }
                        } else {
                            if PyByteArray_Check!(unsafe { *args.wrapping_offset(1 as isize) }) != 0
                            {
                                if PyByteArray_GET_SIZE!(unsafe {
                                    *args.wrapping_offset(1 as isize)
                                }) != 1
                                {
                                    PyErr_Format(PyExc_TypeError, c"center(): argument 2 must be a byte string of length 1, not a bytearray object of length %zd".as_ptr(), PyByteArray_GET_SIZE!(unsafe { *args.wrapping_offset(1 as isize) }));
                                    break '__join_0;
                                } else {
                                    fillchar = *PyByteArray_AS_STRING__2!(unsafe {
                                        *args.wrapping_offset(1 as isize)
                                    })
                                    .wrapping_offset(0 as isize);
                                }
                            } else {
                                _PyArg_BadArgument(
                                    c"center".as_ptr(),
                                    c"argument 2".as_ptr(),
                                    c"a byte string of length 1".as_ptr(),
                                    *args.wrapping_offset(1 as isize),
                                );
                                break '__join_0;
                            }
                        }
                    }
                    return_value = stringlib_center_impl(self_, width, fillchar);
                }
            }
        }
        return return_value;
    }
}

static stringlib_zfill__doc__: __SlateAlign16<[i8; 149]> = __SlateAlign16([
    122i8, 102i8, 105i8, 108i8, 108i8, 40i8, 36i8, 115i8, 101i8, 108i8, 102i8, 44i8, 32i8, 119i8,
    105i8, 100i8, 116i8, 104i8, 44i8, 32i8, 47i8, 41i8, 10i8, 45i8, 45i8, 10i8, 10i8, 80i8, 97i8,
    100i8, 32i8, 97i8, 32i8, 110i8, 117i8, 109i8, 101i8, 114i8, 105i8, 99i8, 32i8, 115i8, 116i8,
    114i8, 105i8, 110i8, 103i8, 32i8, 119i8, 105i8, 116i8, 104i8, 32i8, 122i8, 101i8, 114i8, 111i8,
    115i8, 32i8, 111i8, 110i8, 32i8, 116i8, 104i8, 101i8, 32i8, 108i8, 101i8, 102i8, 116i8, 44i8,
    32i8, 116i8, 111i8, 32i8, 102i8, 105i8, 108i8, 108i8, 32i8, 97i8, 32i8, 102i8, 105i8, 101i8,
    108i8, 100i8, 32i8, 111i8, 102i8, 32i8, 116i8, 104i8, 101i8, 32i8, 103i8, 105i8, 118i8, 101i8,
    110i8, 32i8, 119i8, 105i8, 100i8, 116i8, 104i8, 46i8, 10i8, 10i8, 84i8, 104i8, 101i8, 32i8,
    111i8, 114i8, 105i8, 103i8, 105i8, 110i8, 97i8, 108i8, 32i8, 115i8, 116i8, 114i8, 105i8, 110i8,
    103i8, 32i8, 105i8, 115i8, 32i8, 110i8, 101i8, 118i8, 101i8, 114i8, 32i8, 116i8, 114i8, 117i8,
    110i8, 99i8, 97i8, 116i8, 101i8, 100i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bytesobject.stringlib_zfill")]
extern "C-unwind" fn stringlib_zfill(self_: *mut _object, arg: *mut _object) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    let mut iobj: *mut _object = unsafe { core::mem::zeroed() };
    let mut ival: i64 = unsafe { core::mem::zeroed() };
    let mut return_value: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        return_value = core::ptr::null_mut::<_object>();
        ival = 1i32.wrapping_neg() as i64;
        iobj = _PyNumber_Index(arg);
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
            let width: i64 = ival;
            return_value = stringlib_zfill_impl(self_, width);
        }
        return return_value;
    }
}

fn return_self(self_: *mut _object) -> *mut _object {
    if Py_IS_TYPE!(self_, unsafe { core::ptr::addr_of_mut!(PyBytes_Type) }) != 0 {
        return Py_NewRef!(self_);
    }
    PyBytes_FromStringAndSize(
        PyBytes_AS_STRING(_PyObject_CAST!(self_)) as *const i8,
        PyBytes_GET_SIZE(_PyObject_CAST!(self_)),
    )
}

fn stringlib_expandtabs_impl(self_: *mut _object, tabsize: i32) -> *mut _object {
    let mut incr: i64 = unsafe { core::mem::zeroed() };
    let mut u: *mut _object = unsafe { core::mem::zeroed() };
    let mut j: i64 = unsafe { core::mem::zeroed() };
    let mut i: i64 = unsafe { core::mem::zeroed() };
    let mut q: *mut i8 = unsafe { core::mem::zeroed() };
    let mut p: *const i8 = unsafe { core::mem::zeroed() };
    let mut e: *const i8 = unsafe { core::mem::zeroed() };
    unsafe {
        // First pass: determine size of output string
        j = 0;
        i = 0;
        e = PyBytes_AS_STRING(_PyObject_CAST!(self_))
            .wrapping_offset(PyBytes_GET_SIZE(_PyObject_CAST!(self_)) as isize)
            as *const i8;
        p = PyBytes_AS_STRING(_PyObject_CAST!(self_)) as *const i8;
        '__join_0: {
            while p < e {
                if (*p as i32) == 9 {
                    if tabsize > 0 {
                        incr = (tabsize as i64).wrapping_sub(j % (tabsize as i64));
                        if j > 9223372036854775807i64.wrapping_sub(incr) {
                            break '__join_0;
                        } else {
                            j = j.wrapping_add(incr);
                        }
                    }
                } else {
                    if j > 9223372036854775806i64 {
                        break '__join_0;
                    } else {
                        j = j.wrapping_add(1);
                        if (*p as i32) == 10 || (*p as i32) == 13 {
                            if i > 9223372036854775807i64.wrapping_sub(j) {
                                break '__join_0;
                            } else {
                                i = i.wrapping_add(j);
                                j = 0;
                            }
                        }
                    }
                }
                let __v13: *const i8 = p;
                p = __v13.wrapping_offset(1 as isize);
            }
            if i <= 9223372036854775807i64.wrapping_sub(j) {
                // Second pass: create output string and fill it
                u = PyBytes_FromStringAndSize(core::ptr::null::<i8>(), i.wrapping_add(j));
                if u.is_null() {
                    return core::ptr::null_mut::<_object>();
                } else {
                    j = 0;
                    q = PyBytes_AS_STRING(_PyObject_CAST!(u));
                    p = PyBytes_AS_STRING(_PyObject_CAST!(self_)) as *const i8;
                    while p < e {
                        if (*p as i32) == 9 {
                            if tabsize > 0 {
                                i = (tabsize as i64).wrapping_sub(j % (tabsize as i64));
                                j = j.wrapping_add(i);
                                loop {
                                    let __v3: i64 = i;
                                    i = __v3.wrapping_sub(1);
                                    if __v3 != 0 {
                                        let __v5: *mut i8 = q;
                                        q = __v5.wrapping_offset(1 as isize);
                                        *__v5 = 32;
                                    } else {
                                        break;
                                    }
                                }
                            }
                        } else {
                            j = j.wrapping_add(1);
                            let __v9: *mut i8 = q;
                            q = __v9.wrapping_offset(1 as isize);
                            *__v9 = *p;
                            if (*p as i32) == 10 || (*p as i32) == 13 {
                                j = 0;
                            }
                        }
                        let __v1: *const i8 = p;
                        p = __v1.wrapping_offset(1 as isize);
                    }
                    return u;
                }
            }
        }
        PyErr_SetString(PyExc_OverflowError, c"result too long".as_ptr());
        return core::ptr::null_mut::<_object>();
    }
}

fn pad(self_: *mut _object, mut left: i64, mut right: i64, fill: i8) -> *mut _object {
    if left < 0 {
        left = 0;
    }
    if right < 0 {
        right = 0;
    }
    if left == 0 && right == 0 {
        return return_self(self_);
    }
    let u: *mut _object = PyBytes_FromStringAndSize(
        core::ptr::null::<i8>(),
        left.wrapping_add(PyBytes_GET_SIZE(_PyObject_CAST!(self_)))
            .wrapping_add(right),
    );
    if !u.is_null() {
        if left != 0 {
            unsafe {
                core::ptr::write_bytes(
                    ((PyBytes_AS_STRING(_PyObject_CAST!(u)) as *mut ()) as *mut core::ffi::c_void)
                        as *mut u8,
                    ((fill as i32) as i32) as u8,
                    ((left as u64) as usize) as usize,
                )
            };
        }
        unsafe {
            core::ptr::copy_nonoverlapping(
                ((PyBytes_AS_STRING(_PyObject_CAST!(self_)) as *const ())
                    as *const core::ffi::c_void) as *const u8,
                ((PyBytes_AS_STRING(_PyObject_CAST!(u)).wrapping_offset(left as isize) as *mut ())
                    as *mut core::ffi::c_void) as *mut u8,
                ((PyBytes_GET_SIZE(_PyObject_CAST!(self_)) as u64) as usize) as usize,
            )
        };
        if right != 0 {
            unsafe {
                core::ptr::write_bytes(
                    ((PyBytes_AS_STRING(_PyObject_CAST!(u))
                        .wrapping_offset(left as isize)
                        .wrapping_offset(PyBytes_GET_SIZE(_PyObject_CAST!(self_)) as isize)
                        as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                    ((fill as i32) as i32) as u8,
                    ((right as u64) as usize) as usize,
                )
            };
        }
    }
    u
}

fn stringlib_ljust_impl(self_: *mut _object, width: i64, fillchar: i8) -> *mut _object {
    if PyBytes_GET_SIZE(_PyObject_CAST!(self_)) >= width {
        return return_self(self_);
    }
    pad(
        self_,
        0i64,
        width.wrapping_sub(PyBytes_GET_SIZE(_PyObject_CAST!(self_))),
        fillchar,
    )
}

fn stringlib_rjust_impl(self_: *mut _object, width: i64, fillchar: i8) -> *mut _object {
    if PyBytes_GET_SIZE(_PyObject_CAST!(self_)) >= width {
        return return_self(self_);
    }
    pad(
        self_,
        width.wrapping_sub(PyBytes_GET_SIZE(_PyObject_CAST!(self_))),
        0i64,
        fillchar,
    )
}

fn stringlib_center_impl(self_: *mut _object, width: i64, fillchar: i8) -> *mut _object {
    let mut marg: i64 = 0;
    let mut left: i64 = 0;
    if PyBytes_GET_SIZE(_PyObject_CAST!(self_)) >= width {
        return return_self(self_);
    }
    marg = width.wrapping_sub(PyBytes_GET_SIZE(_PyObject_CAST!(self_)));
    left = (marg / 2).wrapping_add(marg & width & 1);
    pad(self_, left, marg.wrapping_sub(left), fillchar)
}

fn stringlib_zfill_impl(self_: *mut _object, width: i64) -> *mut _object {
    let mut fill: i64 = 0;
    if PyBytes_GET_SIZE(_PyObject_CAST!(self_)) >= width {
        return return_self(self_);
    }
    fill = width.wrapping_sub(PyBytes_GET_SIZE(_PyObject_CAST!(self_)));
    let s: *mut _object = pad(self_, fill, 0i64, 48i8);
    if s.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let p: *mut i8 = PyBytes_AS_STRING(_PyObject_CAST!(s));
    if ((unsafe { *p.wrapping_offset(fill as isize) }) as i32) == 43
        || ((unsafe { *p.wrapping_offset(fill as isize) }) as i32) == 45
    {
        // move sign to beginning of string
        unsafe {
            *p.wrapping_offset(0 as isize) = *p.wrapping_offset(fill as isize);
        }
        unsafe {
            *p.wrapping_offset(fill as isize) = 48;
        }
    }
    s
}

fn countchar(target: *const i8, target_len: i64, c: i8, maxcount: i64) -> i64 {
    let mut count: i64 = 0;
    let mut start: *const i8 = target;
    let end: *const i8 = target.wrapping_offset(target_len as isize);
    loop {
        let __v5: *const i8 = findchar!(
            start,
            (unsafe { end.offset_from(start as *const i8) }) as i64,
            c
        ) as *const i8;
        start = __v5;
        if __v5.is_null() {
            break;
        }
        count = count.wrapping_add(1);
        if count >= maxcount {
            break;
        }
        let __v3: *const i8 = start;
        start = __v3.wrapping_offset(1 as isize);
    }
    count
}

fn stringlib_replace_interleave(
    self_: *mut _object,
    to_s: *const i8,
    to_len: i64,
    maxcount: i64,
) -> *mut _object {
    let mut self_s: *const i8 = unsafe { core::mem::zeroed() };
    let mut result_s: *mut i8 = unsafe { core::mem::zeroed() };
    let self_len: i64 = PyBytes_GET_SIZE(_PyObject_CAST!(self_));
    let mut result_len: i64 = 0;
    let mut count: i64 = 0;
    let mut i: i64 = 0;
    // 1 at the end plus 1 after every character;
    // count = min(maxcount, self_len + 1)
    if maxcount <= self_len {
        count = maxcount;
    } else {
        // Can't overflow: self_len + 1 <= maxcount <= PY_SSIZE_T_MAX.
        count = self_len.wrapping_add(1);
    }
    // Check for overflow
    //
    //   result_len = count * to_len + self_len;
    if to_len > 9223372036854775807i64.wrapping_sub(self_len) / count {
        unsafe { PyErr_SetString(PyExc_OverflowError, c"replace bytes is too long".as_ptr()) };
        return core::ptr::null_mut::<_object>();
    }
    result_len = count.wrapping_mul(to_len).wrapping_add(self_len);
    let result: *mut _object = PyBytes_FromStringAndSize(core::ptr::null::<i8>(), result_len);
    if result.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    self_s = PyBytes_AS_STRING(_PyObject_CAST!(self_)) as *const i8;
    result_s = PyBytes_AS_STRING(_PyObject_CAST!(result));
    if to_len > 1 {
        // Lay the first one down (guaranteed this will occur)
        unsafe {
            core::ptr::copy_nonoverlapping(
                ((to_s as *const ()) as *const core::ffi::c_void) as *const u8,
                ((result_s as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                ((to_len as u64) as usize) as usize,
            )
        };
        let __v0: *mut i8 = result_s;
        result_s = __v0.wrapping_offset(to_len as isize);
        count = count.wrapping_sub(1);
        i = 0;
        while i < count {
            let __v4: *const i8 = self_s;
            self_s = __v4.wrapping_offset(1 as isize);
            let __v6: *mut i8 = result_s;
            result_s = __v6.wrapping_offset(1 as isize);
            unsafe {
                *__v6 = *__v4;
            }
            unsafe {
                core::ptr::copy_nonoverlapping(
                    ((to_s as *const ()) as *const core::ffi::c_void) as *const u8,
                    ((result_s as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                    ((to_len as u64) as usize) as usize,
                )
            };
            let __v8: *mut i8 = result_s;
            result_s = __v8.wrapping_offset(to_len as isize);
            i = i.wrapping_add(1);
        }
    } else {
        unsafe {
            *result_s.wrapping_offset(0 as isize) = *to_s.wrapping_offset(0 as isize);
        }
        let __v13: *mut i8 = result_s;
        result_s = __v13.wrapping_offset(to_len as isize);
        count = count.wrapping_sub(1);
        i = 0;
        while i < count {
            let __v17: *const i8 = self_s;
            self_s = __v17.wrapping_offset(1 as isize);
            let __v19: *mut i8 = result_s;
            result_s = __v19.wrapping_offset(1 as isize);
            unsafe {
                *__v19 = *__v17;
            }
            unsafe {
                *result_s.wrapping_offset(0 as isize) = *to_s.wrapping_offset(0 as isize);
            }
            let __v21: *mut i8 = result_s;
            result_s = __v21.wrapping_offset(to_len as isize);
            i = i.wrapping_add(1);
        }
    }
    // Copy the rest of the original string
    unsafe {
        core::ptr::copy_nonoverlapping(
            ((self_s as *const ()) as *const core::ffi::c_void) as *const u8,
            ((result_s as *mut ()) as *mut core::ffi::c_void) as *mut u8,
            ((self_len.wrapping_sub(i) as u64) as usize) as usize,
        )
    };
    result
}

fn stringlib_replace_delete_single_character(
    self_: *mut _object,
    from_c: i8,
    maxcount: i64,
) -> *mut _object {
    let mut start: *const i8 = unsafe { core::mem::zeroed() };
    let mut result_s: *mut i8 = unsafe { core::mem::zeroed() };
    let mut self_len: i64 = 0;
    let mut result_len: i64 = 0;
    let mut count: i64 = 0;
    self_len = PyBytes_GET_SIZE(_PyObject_CAST!(self_));
    let self_s: *const i8 = PyBytes_AS_STRING(_PyObject_CAST!(self_)) as *const i8;
    count = countchar(self_s, self_len, from_c, maxcount);
    if count == 0 {
        return return_self(self_);
    }
    result_len = self_len.wrapping_sub(count); // from_len == 1
    let result: *mut _object = PyBytes_FromStringAndSize(core::ptr::null::<i8>(), result_len);
    if result.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    result_s = PyBytes_AS_STRING(_PyObject_CAST!(result));
    start = self_s;
    let end: *const i8 = self_s.wrapping_offset(self_len as isize);
    loop {
        let __v3: i64 = count;
        count = __v3.wrapping_sub(1);
        if __v3 <= 0 {
            break;
        }
        let next: *const i8 = findchar!(
            start,
            (unsafe { end.offset_from(start as *const i8) }) as i64,
            from_c
        ) as *const i8;
        if next.is_null() {
            break;
        }
        unsafe {
            core::ptr::copy_nonoverlapping(
                ((start as *const ()) as *const core::ffi::c_void) as *const u8,
                ((result_s as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                (((next.offset_from(start as *const i8) as i64) as u64) as usize) as usize,
            )
        };
        let __v1: *mut i8 = result_s;
        result_s = __v1
            .wrapping_offset(((unsafe { next.offset_from(start as *const i8) }) as i64) as isize);
        start = next.wrapping_offset(1 as isize);
    }
    unsafe {
        core::ptr::copy_nonoverlapping(
            ((start as *const ()) as *const core::ffi::c_void) as *const u8,
            ((result_s as *mut ()) as *mut core::ffi::c_void) as *mut u8,
            (((end.offset_from(start as *const i8) as i64) as u64) as usize) as usize,
        )
    };
    result
}

fn stringlib_replace_delete_substring(
    self_: *mut _object,
    from_s: *const i8,
    from_len: i64,
    maxcount: i64,
) -> *mut _object {
    let mut start: *const i8 = unsafe { core::mem::zeroed() };
    let mut result_s: *mut i8 = unsafe { core::mem::zeroed() };
    let mut self_len: i64 = 0;
    let mut result_len: i64 = 0;
    let mut count: i64 = 0;
    let mut offset: i64 = 0;
    self_len = PyBytes_GET_SIZE(_PyObject_CAST!(self_));
    let self_s: *const i8 = PyBytes_AS_STRING(_PyObject_CAST!(self_)) as *const i8;
    count = stringlib_count(self_s, self_len, from_s, from_len, maxcount);
    if count == 0 {
        // no matches
        return return_self(self_);
    }
    result_len = self_len.wrapping_sub(count.wrapping_mul(from_len));
    let result: *mut _object = PyBytes_FromStringAndSize(core::ptr::null::<i8>(), result_len);
    if result.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    result_s = PyBytes_AS_STRING(_PyObject_CAST!(result));
    start = self_s;
    let end: *const i8 = self_s.wrapping_offset(self_len as isize);
    loop {
        let __v3: i64 = count;
        count = __v3.wrapping_sub(1);
        if __v3 <= 0 {
            break;
        }
        offset = stringlib_find(
            start,
            (unsafe { end.offset_from(start as *const i8) }) as i64,
            from_s,
            from_len,
            0i64,
        );
        if offset == (1i32.wrapping_neg() as i64) {
            break;
        }
        let next: *const i8 = start.wrapping_offset(offset as isize);
        unsafe {
            core::ptr::copy_nonoverlapping(
                ((start as *const ()) as *const core::ffi::c_void) as *const u8,
                ((result_s as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                (((next.offset_from(start as *const i8) as i64) as u64) as usize) as usize,
            )
        };
        let __v1: *mut i8 = result_s;
        result_s = __v1
            .wrapping_offset(((unsafe { next.offset_from(start as *const i8) }) as i64) as isize);
        start = next.wrapping_offset(from_len as isize);
    }
    unsafe {
        core::ptr::copy_nonoverlapping(
            ((start as *const ()) as *const core::ffi::c_void) as *const u8,
            ((result_s as *mut ()) as *mut core::ffi::c_void) as *mut u8,
            (((end.offset_from(start as *const i8) as i64) as u64) as usize) as usize,
        )
    };
    result
}

fn stringlib_replace_single_character_in_place(
    self_: *mut _object,
    from_c: i8,
    to_c: i8,
    mut maxcount: i64,
) -> *mut _object {
    let mut start: *mut i8 = unsafe { core::mem::zeroed() };
    let mut next: *mut i8 = unsafe { core::mem::zeroed() };
    let mut self_len: i64 = 0;
    // The result string will be the same size
    let self_s: *const i8 = PyBytes_AS_STRING(_PyObject_CAST!(self_)) as *const i8;
    self_len = PyBytes_GET_SIZE(_PyObject_CAST!(self_));
    next = findchar!(self_s, self_len, from_c);
    if next.is_null() {
        // No matches; return the original bytes
        return return_self(self_);
    }
    // Need to make a new bytes
    let result: *mut _object = PyBytes_FromStringAndSize(core::ptr::null::<i8>(), self_len);
    if result.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let result_s: *mut i8 = PyBytes_AS_STRING(_PyObject_CAST!(result));
    unsafe {
        core::ptr::copy_nonoverlapping(
            ((self_s as *const ()) as *const core::ffi::c_void) as *const u8,
            ((result_s as *mut ()) as *mut core::ffi::c_void) as *mut u8,
            ((self_len as u64) as usize) as usize,
        )
    };
    // change everything in-place, starting with this one
    start = result_s
        .wrapping_offset(((unsafe { next.offset_from(self_s as *mut i8) }) as i64) as isize);
    unsafe {
        *start = to_c;
    }
    let __v0: *mut i8 = start;
    start = __v0.wrapping_offset(1 as isize);
    let end: *const i8 = result_s.wrapping_offset(self_len as isize) as *const i8;
    loop {
        let __v4: i64 = maxcount.wrapping_sub(1);
        maxcount = __v4;
        if __v4 <= 0 {
            break;
        }
        next = findchar!(
            start,
            (unsafe { end.offset_from(start as *const i8) }) as i64,
            from_c
        );
        if next.is_null() {
            break;
        }
        unsafe {
            *next = to_c;
        }
        start = next.wrapping_offset(1 as isize);
    }
    result
}

fn stringlib_replace_substring_in_place(
    self_: *mut _object,
    from_s: *const i8,
    from_len: i64,
    to_s: *const i8,
    to_len: i64,
    mut maxcount: i64,
) -> *mut _object {
    let mut start: *mut i8 = unsafe { core::mem::zeroed() };
    let mut self_len: i64 = 0;
    let mut offset: i64 = 0;
    // The result bytes will be the same size
    let self_s: *const i8 = PyBytes_AS_STRING(_PyObject_CAST!(self_)) as *const i8;
    self_len = PyBytes_GET_SIZE(_PyObject_CAST!(self_));
    offset = stringlib_find(self_s, self_len, from_s, from_len, 0i64);
    if offset == (1i32.wrapping_neg() as i64) {
        // No matches; return the original bytes
        return return_self(self_);
    }
    // Need to make a new bytes
    let result: *mut _object = PyBytes_FromStringAndSize(core::ptr::null::<i8>(), self_len);
    if result.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    let result_s: *mut i8 = PyBytes_AS_STRING(_PyObject_CAST!(result));
    unsafe {
        core::ptr::copy_nonoverlapping(
            ((self_s as *const ()) as *const core::ffi::c_void) as *const u8,
            ((result_s as *mut ()) as *mut core::ffi::c_void) as *mut u8,
            ((self_len as u64) as usize) as usize,
        )
    };
    // change everything in-place, starting with this one
    start = result_s.wrapping_offset(offset as isize);
    unsafe {
        core::ptr::copy_nonoverlapping(
            ((to_s as *const ()) as *const core::ffi::c_void) as *const u8,
            ((start as *mut ()) as *mut core::ffi::c_void) as *mut u8,
            ((from_len as u64) as usize) as usize,
        )
    };
    let __v0: *mut i8 = start;
    start = __v0.wrapping_offset(from_len as isize);
    let end: *const i8 = result_s.wrapping_offset(self_len as isize) as *const i8;
    loop {
        let __v6: i64 = maxcount.wrapping_sub(1);
        maxcount = __v6;
        if __v6 <= 0 {
            break;
        }
        offset = stringlib_find(
            start as *const i8,
            (unsafe { end.offset_from(start as *const i8) }) as i64,
            from_s,
            from_len,
            0i64,
        );
        if offset == (1i32.wrapping_neg() as i64) {
            break;
        }
        unsafe {
            core::ptr::copy_nonoverlapping(
                ((to_s as *const ()) as *const core::ffi::c_void) as *const u8,
                ((start.wrapping_offset(offset as isize) as *mut ()) as *mut core::ffi::c_void)
                    as *mut u8,
                ((from_len as u64) as usize) as usize,
            )
        };
        let __v3: *mut i8 = start;
        start = __v3.wrapping_offset(offset.wrapping_add(from_len) as isize);
    }
    result
}

fn stringlib_replace_single_character(
    self_: *mut _object,
    from_c: i8,
    to_s: *const i8,
    to_len: i64,
    maxcount: i64,
) -> *mut _object {
    let mut start: *const i8 = unsafe { core::mem::zeroed() };
    let mut result_s: *mut i8 = unsafe { core::mem::zeroed() };
    let mut self_len: i64 = 0;
    let mut result_len: i64 = 0;
    let mut count: i64 = 0;
    let self_s: *const i8 = PyBytes_AS_STRING(_PyObject_CAST!(self_)) as *const i8;
    self_len = PyBytes_GET_SIZE(_PyObject_CAST!(self_));
    count = countchar(self_s, self_len, from_c, maxcount);
    if count == 0 {
        // no matches, return unchanged
        return return_self(self_);
    }
    // use the difference between current and new, hence the "-1"
    //
    //   result_len = self_len + count * (to_len-1)
    if to_len.wrapping_sub(1) > 9223372036854775807i64.wrapping_sub(self_len) / count {
        unsafe { PyErr_SetString(PyExc_OverflowError, c"replace bytes is too long".as_ptr()) };
        return core::ptr::null_mut::<_object>();
    }
    result_len = self_len.wrapping_add(count.wrapping_mul(to_len.wrapping_sub(1)));
    let result: *mut _object = PyBytes_FromStringAndSize(core::ptr::null::<i8>(), result_len);
    if result.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    result_s = PyBytes_AS_STRING(_PyObject_CAST!(result));
    start = self_s;
    let end: *const i8 = self_s.wrapping_offset(self_len as isize);
    loop {
        let __v9: i64 = count;
        count = __v9.wrapping_sub(1);
        if __v9 <= 0 {
            break;
        }
        let next: *const i8 = findchar!(
            start,
            (unsafe { end.offset_from(start as *const i8) }) as i64,
            from_c
        ) as *const i8;
        if next.is_null() {
            break;
        }
        if next == start {
            // replace with the 'to'
            unsafe {
                core::ptr::copy_nonoverlapping(
                    ((to_s as *const ()) as *const core::ffi::c_void) as *const u8,
                    ((result_s as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                    ((to_len as u64) as usize) as usize,
                )
            };
            let __v1: *mut i8 = result_s;
            result_s = __v1.wrapping_offset(to_len as isize);
            let __v3: *const i8 = start;
            start = __v3.wrapping_offset(1 as isize);
        } else {
            // copy the unchanged old then the 'to'
            unsafe {
                core::ptr::copy_nonoverlapping(
                    ((start as *const ()) as *const core::ffi::c_void) as *const u8,
                    ((result_s as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                    (((next.offset_from(start as *const i8) as i64) as u64) as usize) as usize,
                )
            };
            let __v5: *mut i8 = result_s;
            result_s = __v5.wrapping_offset(
                ((unsafe { next.offset_from(start as *const i8) }) as i64) as isize,
            );
            unsafe {
                core::ptr::copy_nonoverlapping(
                    ((to_s as *const ()) as *const core::ffi::c_void) as *const u8,
                    ((result_s as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                    ((to_len as u64) as usize) as usize,
                )
            };
            let __v7: *mut i8 = result_s;
            result_s = __v7.wrapping_offset(to_len as isize);
            start = next.wrapping_offset(1 as isize);
        }
    }
    // Copy the remainder of the remaining bytes
    unsafe {
        core::ptr::copy_nonoverlapping(
            ((start as *const ()) as *const core::ffi::c_void) as *const u8,
            ((result_s as *mut ()) as *mut core::ffi::c_void) as *mut u8,
            (((end.offset_from(start as *const i8) as i64) as u64) as usize) as usize,
        )
    };
    result
}

fn stringlib_replace_substring(
    self_: *mut _object,
    from_s: *const i8,
    from_len: i64,
    to_s: *const i8,
    to_len: i64,
    maxcount: i64,
) -> *mut _object {
    let mut start: *const i8 = unsafe { core::mem::zeroed() };
    let mut result_s: *mut i8 = unsafe { core::mem::zeroed() };
    let mut self_len: i64 = 0;
    let mut result_len: i64 = 0;
    let mut count: i64 = 0;
    let mut offset: i64 = 0;
    let self_s: *const i8 = PyBytes_AS_STRING(_PyObject_CAST!(self_)) as *const i8;
    self_len = PyBytes_GET_SIZE(_PyObject_CAST!(self_));
    count = stringlib_count(self_s, self_len, from_s, from_len, maxcount);
    if count == 0 {
        // no matches, return unchanged
        return return_self(self_);
    }
    // Check for overflow
    //
    //    result_len = self_len + count * (to_len-from_len)
    if to_len.wrapping_sub(from_len) > 9223372036854775807i64.wrapping_sub(self_len) / count {
        unsafe { PyErr_SetString(PyExc_OverflowError, c"replace bytes is too long".as_ptr()) };
        return core::ptr::null_mut::<_object>();
    }
    result_len = self_len.wrapping_add(count.wrapping_mul(to_len.wrapping_sub(from_len)));
    let result: *mut _object = PyBytes_FromStringAndSize(core::ptr::null::<i8>(), result_len);
    if result.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    result_s = PyBytes_AS_STRING(_PyObject_CAST!(result));
    start = self_s;
    let end: *const i8 = self_s.wrapping_offset(self_len as isize);
    loop {
        let __v9: i64 = count;
        count = __v9.wrapping_sub(1);
        if __v9 <= 0 {
            break;
        }
        offset = stringlib_find(
            start,
            (unsafe { end.offset_from(start as *const i8) }) as i64,
            from_s,
            from_len,
            0i64,
        );
        if offset == (1i32.wrapping_neg() as i64) {
            break;
        }
        let next: *const i8 = start.wrapping_offset(offset as isize);
        if next == start {
            // replace with the 'to'
            unsafe {
                core::ptr::copy_nonoverlapping(
                    ((to_s as *const ()) as *const core::ffi::c_void) as *const u8,
                    ((result_s as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                    ((to_len as u64) as usize) as usize,
                )
            };
            let __v1: *mut i8 = result_s;
            result_s = __v1.wrapping_offset(to_len as isize);
            let __v3: *const i8 = start;
            start = __v3.wrapping_offset(from_len as isize);
        } else {
            // copy the unchanged old then the 'to'
            unsafe {
                core::ptr::copy_nonoverlapping(
                    ((start as *const ()) as *const core::ffi::c_void) as *const u8,
                    ((result_s as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                    (((next.offset_from(start as *const i8) as i64) as u64) as usize) as usize,
                )
            };
            let __v5: *mut i8 = result_s;
            result_s = __v5.wrapping_offset(
                ((unsafe { next.offset_from(start as *const i8) }) as i64) as isize,
            );
            unsafe {
                core::ptr::copy_nonoverlapping(
                    ((to_s as *const ()) as *const core::ffi::c_void) as *const u8,
                    ((result_s as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                    ((to_len as u64) as usize) as usize,
                )
            };
            let __v7: *mut i8 = result_s;
            result_s = __v7.wrapping_offset(to_len as isize);
            start = next.wrapping_offset(from_len as isize);
        }
    }
    // Copy the remainder of the remaining bytes
    unsafe {
        core::ptr::copy_nonoverlapping(
            ((start as *const ()) as *const core::ffi::c_void) as *const u8,
            ((result_s as *mut ()) as *mut core::ffi::c_void) as *mut u8,
            (((end.offset_from(start as *const i8) as i64) as u64) as usize) as usize,
        )
    };
    result
}

fn stringlib_replace(
    self_: *mut _object,
    from_s: *const i8,
    from_len: i64,
    to_s: *const i8,
    to_len: i64,
    mut maxcount: i64,
) -> *mut _object {
    if PyBytes_GET_SIZE(_PyObject_CAST!(self_)) < from_len {
        // nothing to do; return the original bytes
        return return_self(self_);
    }
    if maxcount < 0 {
        maxcount = 9223372036854775807;
    } else {
        if maxcount == 0 {
            // nothing to do; return the original bytes
            return return_self(self_);
        }
    }
    // Handle zero-length special cases
    if from_len == 0 {
        if to_len == 0 {
            // nothing to do; return the original bytes
            return return_self(self_);
        }
        // insert the 'to' bytes everywhere.
        //
        //    >>> b"Python".replace(b"", b".")
        //
        //    b'.P.y.t.h.o.n.'
        return stringlib_replace_interleave(self_, to_s, to_len, maxcount);
    }
    if to_len == 0 {
        // delete all occurrences of 'from' bytes
        if from_len == 1 {
            return stringlib_replace_delete_single_character(
                self_,
                unsafe { *from_s.wrapping_offset(0 as isize) },
                maxcount,
            );
        } else {
            return stringlib_replace_delete_substring(self_, from_s, from_len, maxcount);
        }
    }
    // Handle special case where both bytes have the same length
    if from_len == to_len {
        if from_len == 1 {
            return stringlib_replace_single_character_in_place(
                self_,
                unsafe { *from_s.wrapping_offset(0 as isize) },
                unsafe { *to_s.wrapping_offset(0 as isize) },
                maxcount,
            );
        } else {
            return stringlib_replace_substring_in_place(
                self_, from_s, from_len, to_s, to_len, maxcount,
            );
        }
    }
    // Otherwise use the more generic algorithms
    if from_len == 1 {
        return stringlib_replace_single_character(
            self_,
            unsafe { *from_s.wrapping_offset(0 as isize) },
            to_s,
            to_len,
            maxcount,
        );
    } else {
        // len('from')>=2, len('to')>=1
        return stringlib_replace_substring(self_, from_s, from_len, to_s, to_len, maxcount);
    }
}

// Methods
#[unsafe(no_mangle)]
extern "C-unwind" fn _PyBytes_Find(
    haystack: *const i8,
    len_haystack: i64,
    needle: *const i8,
    len_needle: i64,
    offset: i64,
) -> i64 {
    // Extra checks because stringlib_find accesses haystack[len_haystack].
    if len_needle == 0 {
        return offset;
    }
    if len_needle > len_haystack {
        return 1i32.wrapping_neg() as i64;
    }
    let res: i64 = stringlib_find(
        haystack,
        len_haystack.wrapping_sub(1),
        needle,
        len_needle,
        offset,
    );
    if res == (1i32.wrapping_neg() as i64) {
        let last_align: i64 = len_haystack.wrapping_sub(len_needle);
        if (unsafe {
            memcmp(
                (haystack.wrapping_offset(last_align as isize) as *const ())
                    as *const core::ffi::c_void,
                (needle as *const ()) as *const core::ffi::c_void,
                (len_needle as u64) as usize,
            ) as i32
        }) == 0
        {
            return offset.wrapping_add(last_align);
        }
    }
    res
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyBytes_ReverseFind(
    haystack: *const i8,
    len_haystack: i64,
    needle: *const i8,
    len_needle: i64,
    offset: i64,
) -> i64 {
    stringlib_rfind(haystack, len_haystack, needle, len_needle, offset)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyBytes_Repr(obj: *mut _object, smartquotes: i32) -> *mut _object {
    let mut c: u8 = unsafe { core::mem::zeroed() };
    let mut incr: i64 = unsafe { core::mem::zeroed() };
    let mut p: *mut u8 = unsafe { core::mem::zeroed() };
    let mut s: *const u8 = unsafe { core::mem::zeroed() };
    let mut quote: u8 = unsafe { core::mem::zeroed() };
    let mut v: *mut _object = unsafe { core::mem::zeroed() };
    let mut dquotes: i64 = unsafe { core::mem::zeroed() };
    let mut squotes: i64 = unsafe { core::mem::zeroed() };
    let mut newsize: i64 = unsafe { core::mem::zeroed() };
    let mut length: i64 = unsafe { core::mem::zeroed() };
    let mut i: i64 = unsafe { core::mem::zeroed() };
    let mut op: *mut __SlateRecord_bytesobject_h_15 = unsafe { core::mem::zeroed() };
    unsafe {
        op = obj as *mut __SlateRecord_bytesobject_h_15;
        length = Py_SIZE!(op);
        // Compute size of output string
        dquotes = 0;
        squotes = 0;
        newsize = 3; // b''
        s = ((*op).ob_sval.as_mut_ptr() as *mut i8) as *const u8;
        i = 0;
        '__join_0: {
            while i < length {
                '__join_25: {
                    incr = 1;
                    let __t0: i32 = *s.wrapping_offset(i as isize) as i32;
                    if __t0 == 39 {
                        squotes = squotes.wrapping_add(1);
                        break '__join_25;
                    } else {
                        if __t0 == 34 {
                            dquotes = dquotes.wrapping_add(1);
                            break '__join_25;
                        } else {
                            if __t0 != 92 {
                                if __t0 != 9 {
                                    if __t0 != 10 {
                                        if __t0 != 13 {
                                            if (*s.wrapping_offset(i as isize) as i32) < 32
                                                || (*s.wrapping_offset(i as isize) as i32) >= 127
                                            {
                                                incr = 4;
                                            }
                                            // \xHH
                                            break '__join_25;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    incr = 2;
                }
                if newsize > 9223372036854775807i64.wrapping_sub(incr) {
                    break '__join_0;
                } else {
                    newsize = newsize.wrapping_add(incr);
                    i = i.wrapping_add(1);
                }
            }
            quote = 39;
            if smartquotes != 0 && squotes != 0 && dquotes == 0 {
                quote = 34;
            }
            if squotes != 0 && (quote as i32) == 39 {
                if newsize > 9223372036854775807i64.wrapping_sub(squotes) {
                    break '__join_0;
                } else {
                    newsize = newsize.wrapping_add(squotes);
                }
            }
            v = PyUnicode_New(newsize, 127u32);
            if v.is_null() {
                return core::ptr::null_mut::<_object>();
            } else {
                p = PyUnicode_1BYTE_DATA!(v);
                let __v36: *mut u8 = p;
                p = __v36.wrapping_offset(1 as isize);
                *__v36 = 98;
                let __v34: *mut u8 = p;
                p = __v34.wrapping_offset(1 as isize);
                *__v34 = quote;
                i = 0;
                while i < length {
                    c = *((*op).ob_sval.as_mut_ptr() as *mut i8).wrapping_offset(i as isize) as u8;
                    if (c as i32) == (quote as i32) || (c as i32) == 92 {
                        let __v8: *mut u8 = p;
                        p = __v8.wrapping_offset(1 as isize);
                        *__v8 = 92;
                        let __v6: *mut u8 = p;
                        p = __v6.wrapping_offset(1 as isize);
                        *__v6 = c;
                    } else {
                        if (c as i32) == 9 {
                            let __v12: *mut u8 = p;
                            p = __v12.wrapping_offset(1 as isize);
                            *__v12 = 92;
                            let __v10: *mut u8 = p;
                            p = __v10.wrapping_offset(1 as isize);
                            *__v10 = 116;
                        } else {
                            if (c as i32) == 10 {
                                let __v16: *mut u8 = p;
                                p = __v16.wrapping_offset(1 as isize);
                                *__v16 = 92;
                                let __v14: *mut u8 = p;
                                p = __v14.wrapping_offset(1 as isize);
                                *__v14 = 110;
                            } else {
                                if (c as i32) == 13 {
                                    let __v20: *mut u8 = p;
                                    p = __v20.wrapping_offset(1 as isize);
                                    *__v20 = 92;
                                    let __v18: *mut u8 = p;
                                    p = __v18.wrapping_offset(1 as isize);
                                    *__v18 = 114;
                                } else {
                                    if (c as i32) < 32 || (c as i32) >= 127 {
                                        let __v28: *mut u8 = p;
                                        p = __v28.wrapping_offset(1 as isize);
                                        *__v28 = 92;
                                        let __v26: *mut u8 = p;
                                        p = __v26.wrapping_offset(1 as isize);
                                        *__v26 = 120;
                                        let __v24: *mut u8 = p;
                                        p = __v24.wrapping_offset(1 as isize);
                                        *__v24 = *unsafe { Py_hexdigits }
                                            .wrapping_offset((((c as i32) & 240) >> 4i32) as isize)
                                            as u8;
                                        let __v22: *mut u8 = p;
                                        p = __v22.wrapping_offset(1 as isize);
                                        *__v22 = *unsafe { Py_hexdigits }
                                            .wrapping_offset(((c as i32) & 15) as isize)
                                            as u8;
                                    } else {
                                        let __v30: *mut u8 = p;
                                        p = __v30.wrapping_offset(1 as isize);
                                        *__v30 = c;
                                    }
                                }
                            }
                        }
                    }
                    i = i.wrapping_add(1);
                }
                let __v1: *mut u8 = p;
                p = __v1.wrapping_offset(1 as isize);
                *__v1 = quote;
                return v;
            }
        }
        PyErr_SetString(
            PyExc_OverflowError,
            c"bytes object is too large to make repr".as_ptr(),
        );
        return core::ptr::null_mut::<_object>();
    }
    // \C
}

#[unsafe(link_section = ".text.slate_distinct.bytesobject.bytes_repr")]
extern "C-unwind" fn bytes_repr(op: *mut _object) -> *mut _object {
    PyBytes_Repr(op, 1i32)
}

#[unsafe(link_section = ".text.slate_distinct.bytesobject.bytes_str")]
extern "C-unwind" fn bytes_str(op: *mut _object) -> *mut _object {
    if (unsafe { (*_Py_GetConfig()).bytes_warning }) != 0 {
        if (unsafe {
            PyErr_WarnEx(
                PyExc_BytesWarning,
                c"str() on a bytes instance".as_ptr(),
                1i64,
            )
        }) != 0
        {
            return core::ptr::null_mut::<_object>();
        }
    }
    bytes_repr(op)
}

#[unsafe(link_section = ".text.slate_distinct.bytesobject.bytes_length")]
extern "C-unwind" fn bytes_length(self_: *mut _object) -> i64 {
    let a: *mut __SlateRecord_bytesobject_h_15 = _PyBytes_CAST!(self_);
    Py_SIZE!(a)
}

/// This is also used by PyBytes_Concat()
#[unsafe(link_section = ".text.slate_distinct.bytesobject.bytes_concat")]
extern "C-unwind" fn bytes_concat(a: *mut _object, b: *mut _object) -> *mut _object {
    let mut __v0: bool = unsafe { core::mem::zeroed() };
    // Optimize end cases
    let mut __v1: bool = unsafe { core::mem::zeroed() };
    let mut __v2: bool = unsafe { core::mem::zeroed() };
    let mut result: *mut _object = unsafe { core::mem::zeroed() };
    let mut vb: core::mem::MaybeUninit<__SlateRecord_pybuffer_h_33> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_4: *mut __SlateRecord_pybuffer_h_33 =
        core::ptr::addr_of_mut!(vb) as *mut __SlateRecord_pybuffer_h_33;
    let mut va: core::mem::MaybeUninit<__SlateRecord_pybuffer_h_33> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_5: *mut __SlateRecord_pybuffer_h_33 =
        core::ptr::addr_of_mut!(va) as *mut __SlateRecord_pybuffer_h_33;
    unsafe {
        result = core::ptr::null_mut::<_object>();
        (*__slate_slot_5).len = 1i32.wrapping_neg() as i64;
        (*__slate_slot_4).len = 1i32.wrapping_neg() as i64;
        if PyObject_GetBuffer(a, core::ptr::addr_of_mut!(*__slate_slot_5), 0i32) != 0 {
            __v2 = true;
        } else {
            __v2 = PyObject_GetBuffer(b, core::ptr::addr_of_mut!(*__slate_slot_4), 0i32) != 0;
        }
        if __v2 {
            PyErr_Format(
                PyExc_TypeError,
                c"can't concat %.100s to %.100s".as_ptr(),
                (*Py_TYPE!(b)).tp_name,
                (*Py_TYPE!(a)).tp_name,
            );
        } else {
            if (*__slate_slot_5).len == 0 {
                __v1 = PyBytes_CheckExact!(b) != 0;
            } else {
                __v1 = false;
            }
            if __v1 {
                result = Py_NewRef!(b);
            } else {
                if (*__slate_slot_4).len == 0 {
                    __v0 = PyBytes_CheckExact!(a) != 0;
                } else {
                    __v0 = false;
                }
                if __v0 {
                    result = Py_NewRef!(a);
                } else {
                    if (*__slate_slot_5).len
                        > 9223372036854775807i64.wrapping_sub((*__slate_slot_4).len)
                    {
                        PyErr_NoMemory();
                    } else {
                        result = PyBytes_FromStringAndSize(
                            core::ptr::null::<i8>(),
                            (*__slate_slot_5).len.wrapping_add((*__slate_slot_4).len),
                        );
                        if !result.is_null() {
                            core::ptr::copy_nonoverlapping(
                                (((*__slate_slot_5).buf as *const ()) as *const core::ffi::c_void)
                                    as *const u8,
                                ((PyBytes_AS_STRING!(result) as *mut ()) as *mut core::ffi::c_void)
                                    as *mut u8,
                                (((*__slate_slot_5).len as u64) as usize) as usize,
                            );
                            core::ptr::copy_nonoverlapping(
                                (((*__slate_slot_4).buf as *const ()) as *const core::ffi::c_void)
                                    as *const u8,
                                ((PyBytes_AS_STRING!(result)
                                    .wrapping_offset((*__slate_slot_5).len as isize)
                                    as *mut ())
                                    as *mut core::ffi::c_void)
                                    as *mut u8,
                                (((*__slate_slot_4).len as u64) as usize) as usize,
                            );
                        }
                    }
                }
            }
        }
        if (*__slate_slot_5).len != (1i32.wrapping_neg() as i64) {
            PyBuffer_Release(core::ptr::addr_of_mut!(*__slate_slot_5));
        }
        if (*__slate_slot_4).len != (1i32.wrapping_neg() as i64) {
            PyBuffer_Release(core::ptr::addr_of_mut!(*__slate_slot_4));
        }
        return result;
    }
}

#[unsafe(link_section = ".text.slate_distinct.bytesobject.bytes_repeat")]
extern "C-unwind" fn bytes_repeat(self_: *mut _object, mut n: i64) -> *mut _object {
    let a: *mut __SlateRecord_bytesobject_h_15 = _PyBytes_CAST!(self_);
    if n < 0 {
        n = 0;
    }
    // watch out for overflows:  the size can overflow int,
    // and the # of bytes needed can overflow size_t
    let __v0: bool = n > 0 && Py_SIZE!(a) > 9223372036854775807 / n;
    if __v0 {
        unsafe { PyErr_SetString(PyExc_OverflowError, c"repeated bytes are too long".as_ptr()) };
        return core::ptr::null_mut::<_object>();
    }
    let size: i64 = Py_SIZE!(a).wrapping_mul(n);
    let __v1: bool = size == Py_SIZE!(a) && PyBytes_CheckExact__2!(a) != 0;
    if __v1 {
        return Py_NewRef__2!(a);
    }
    let nbytes: u64 = size as u64;
    if nbytes.wrapping_add(
        (core::mem::offset_of!(__SlateRecord_bytesobject_h_15, ob_sval) as u64).wrapping_add(1),
    ) <= nbytes
    {
        unsafe { PyErr_SetString(PyExc_OverflowError, c"repeated bytes are too long".as_ptr()) };
        return core::ptr::null_mut::<_object>();
    }
    let op: *mut __SlateRecord_bytesobject_h_15 = (unsafe {
        PyObject_Malloc(
            (core::mem::offset_of!(__SlateRecord_bytesobject_h_15, ob_sval) as u64)
                .wrapping_add(1)
                .wrapping_add(nbytes),
        )
    }) as *mut __SlateRecord_bytesobject_h_15;
    if op.is_null() {
        return unsafe { PyErr_NoMemory() };
    }
    _PyObject_InitVar(
        op as *mut __SlateRecord_object_h_172,
        unsafe { core::ptr::addr_of_mut!(PyBytes_Type) },
        size,
    );
    set_ob_shash(op, 1i32.wrapping_neg() as i64);
    unsafe {
        *((*op).ob_sval.as_mut_ptr() as *mut i8).wrapping_offset(size as isize) = 0;
    }
    _PyBytes_Repeat(
        unsafe { (*op).ob_sval.as_mut_ptr() as *mut i8 },
        size,
        (unsafe { (*a).ob_sval.as_mut_ptr() as *mut i8 }) as *const i8,
        Py_SIZE!(a),
    );
    op as *mut _object
}

#[unsafe(link_section = ".text.slate_distinct.bytesobject.bytes_contains")]
extern "C-unwind" fn bytes_contains(self_: *mut _object, arg: *mut _object) -> i32 {
    unsafe {
        _Py_bytes_contains(
            PyBytes_AS_STRING!(self_) as *const i8,
            PyBytes_GET_SIZE!(self_),
            arg,
        )
    }
}

#[unsafe(link_section = ".text.slate_distinct.bytesobject.bytes_item")]
extern "C-unwind" fn bytes_item(self_: *mut _object, i: i64) -> *mut _object {
    let a: *mut __SlateRecord_bytesobject_h_15 = _PyBytes_CAST!(self_);
    let __v0: bool = i < 0 || i >= Py_SIZE!(a);
    if __v0 {
        unsafe { PyErr_SetString(PyExc_IndexError, c"index out of range".as_ptr()) };
        return core::ptr::null_mut::<_object>();
    }
    _PyLong_FromUnsignedChar(
        (unsafe { *((*a).ob_sval.as_mut_ptr() as *mut i8).wrapping_offset(i as isize) }) as u8,
    )
}

fn bytes_compare_eq(
    a: *mut __SlateRecord_bytesobject_h_15,
    b: *mut __SlateRecord_bytesobject_h_15,
) -> i32 {
    let mut cmp: i32 = 0;
    let len: i64 = Py_SIZE!(a);
    if Py_SIZE!(b) != len {
        return 0;
    }
    if ((unsafe { *((*a).ob_sval.as_mut_ptr() as *mut i8).wrapping_offset(0 as isize) }) as i32)
        != ((unsafe { *((*b).ob_sval.as_mut_ptr() as *mut i8).wrapping_offset(0 as isize) }) as i32)
    {
        return 0;
    }
    cmp = unsafe {
        memcmp(
            (((*a).ob_sval.as_mut_ptr() as *mut i8) as *const ()) as *const core::ffi::c_void,
            (((*b).ob_sval.as_mut_ptr() as *mut i8) as *const ()) as *const core::ffi::c_void,
            (len as u64) as usize,
        ) as i32
    };
    (cmp == 0) as i32
}

#[unsafe(link_section = ".text.slate_distinct.bytesobject.bytes_richcompare")]
extern "C-unwind" fn bytes_richcompare(
    aa: *mut _object,
    bb: *mut _object,
    op: i32,
) -> *mut _object {
    // Make sure both arguments are strings.
    let __v0: bool = PyBytes_Check!(aa) != 0 && PyBytes_Check!(bb) != 0;
    if !__v0 {
        if (unsafe { (*_Py_GetConfig()).bytes_warning }) != 0 && (op == 2 || op == 3) {
            let __v1: bool = PyUnicode_Check!(aa) != 0 || PyUnicode_Check!(bb) != 0;
            if __v1 {
                if (unsafe {
                    PyErr_WarnEx(
                        PyExc_BytesWarning,
                        c"Comparison between bytes and string".as_ptr(),
                        1i64,
                    )
                }) != 0
                {
                    return core::ptr::null_mut::<_object>();
                }
            }
            let __v2: bool = PyLong_Check!(aa) != 0 || PyLong_Check!(bb) != 0;
            if __v2 {
                if (unsafe {
                    PyErr_WarnEx(
                        PyExc_BytesWarning,
                        c"Comparison between bytes and int".as_ptr(),
                        1i64,
                    )
                }) != 0
                {
                    return core::ptr::null_mut::<_object>();
                }
            }
        }
        return unsafe { core::ptr::addr_of_mut!(_Py_NotImplementedStruct) };
    }
    let a: *mut __SlateRecord_bytesobject_h_15 = _PyBytes_CAST!(aa);
    let b: *mut __SlateRecord_bytesobject_h_15 = _PyBytes_CAST!(bb);
    if a == b {
        match op {
            1 | 2 | 5 => {
                return _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_TrueStruct) }); // a byte string is equal to itself
            }
            0 | 3 | 4 => {
                return _PyObject_CAST__2!(unsafe { core::ptr::addr_of_mut!(_Py_FalseStruct) });
            }
            _ => {
                unsafe { PyErr_BadArgument() };
                return core::ptr::null_mut::<_object>();
            }
        }
    } else {
        if op == 2 || op == 3 {
            let mut eq: i32 = bytes_compare_eq(a, b);
            eq ^= (op == 3) as i32;
            return unsafe { PyBool_FromLong(eq as i64) };
        } else {
            let len_a: i64 = Py_SIZE!(a);
            let len_b: i64 = Py_SIZE!(b);
            let min_len: i64 = Py_MIN!(len_a, len_b);
            let mut c: i32 = 0;
            if min_len > 0 {
                c = (Py_CHARMASK!(unsafe { *((*a).ob_sval.as_mut_ptr() as *mut i8) }) as i32)
                    .wrapping_sub(
                        Py_CHARMASK!(unsafe { *((*b).ob_sval.as_mut_ptr() as *mut i8) }) as i32,
                    );
                if c == 0 {
                    c = unsafe {
                        memcmp(
                            (((*a).ob_sval.as_mut_ptr() as *mut i8) as *const ())
                                as *const core::ffi::c_void,
                            (((*b).ob_sval.as_mut_ptr() as *mut i8) as *const ())
                                as *const core::ffi::c_void,
                            (min_len as u64) as usize,
                        ) as i32
                    };
                }
            } else {
                c = 0;
            }
            if c != 0 {
                loop {
                    match op {
                        2 => {
                            if c == 0 {
                                return _PyObject_CAST__2!(unsafe {
                                    core::ptr::addr_of_mut!(_Py_TrueStruct)
                                });
                            }
                            return _PyObject_CAST__2!(unsafe {
                                core::ptr::addr_of_mut!(_Py_FalseStruct)
                            });
                        }
                        3 => {
                            if c != 0 {
                                return _PyObject_CAST__2!(unsafe {
                                    core::ptr::addr_of_mut!(_Py_TrueStruct)
                                });
                            }
                            return _PyObject_CAST__2!(unsafe {
                                core::ptr::addr_of_mut!(_Py_FalseStruct)
                            });
                        }
                        0 => {
                            if c < 0 {
                                return _PyObject_CAST__2!(unsafe {
                                    core::ptr::addr_of_mut!(_Py_TrueStruct)
                                });
                            }
                            return _PyObject_CAST__2!(unsafe {
                                core::ptr::addr_of_mut!(_Py_FalseStruct)
                            });
                        }
                        4 => {
                            if c > 0 {
                                return _PyObject_CAST__2!(unsafe {
                                    core::ptr::addr_of_mut!(_Py_TrueStruct)
                                });
                            }
                            return _PyObject_CAST__2!(unsafe {
                                core::ptr::addr_of_mut!(_Py_FalseStruct)
                            });
                        }
                        1 => {
                            if c <= 0 {
                                return _PyObject_CAST__2!(unsafe {
                                    core::ptr::addr_of_mut!(_Py_TrueStruct)
                                });
                            }
                            return _PyObject_CAST__2!(unsafe {
                                core::ptr::addr_of_mut!(_Py_FalseStruct)
                            });
                        }
                        5 => {
                            if c >= 0 {
                                return _PyObject_CAST__2!(unsafe {
                                    core::ptr::addr_of_mut!(_Py_TrueStruct)
                                });
                            }
                            return _PyObject_CAST__2!(unsafe {
                                core::ptr::addr_of_mut!(_Py_FalseStruct)
                            });
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
            loop {
                match op {
                    2 => {
                        if len_a == len_b {
                            return _PyObject_CAST__2!(unsafe {
                                core::ptr::addr_of_mut!(_Py_TrueStruct)
                            });
                        }
                        return _PyObject_CAST__2!(unsafe {
                            core::ptr::addr_of_mut!(_Py_FalseStruct)
                        });
                    }
                    3 => {
                        if len_a != len_b {
                            return _PyObject_CAST__2!(unsafe {
                                core::ptr::addr_of_mut!(_Py_TrueStruct)
                            });
                        }
                        return _PyObject_CAST__2!(unsafe {
                            core::ptr::addr_of_mut!(_Py_FalseStruct)
                        });
                    }
                    0 => {
                        if len_a < len_b {
                            return _PyObject_CAST__2!(unsafe {
                                core::ptr::addr_of_mut!(_Py_TrueStruct)
                            });
                        }
                        return _PyObject_CAST__2!(unsafe {
                            core::ptr::addr_of_mut!(_Py_FalseStruct)
                        });
                    }
                    4 => {
                        if len_a > len_b {
                            return _PyObject_CAST__2!(unsafe {
                                core::ptr::addr_of_mut!(_Py_TrueStruct)
                            });
                        }
                        return _PyObject_CAST__2!(unsafe {
                            core::ptr::addr_of_mut!(_Py_FalseStruct)
                        });
                    }
                    1 => {
                        if len_a <= len_b {
                            return _PyObject_CAST__2!(unsafe {
                                core::ptr::addr_of_mut!(_Py_TrueStruct)
                            });
                        }
                        return _PyObject_CAST__2!(unsafe {
                            core::ptr::addr_of_mut!(_Py_FalseStruct)
                        });
                    }
                    5 => {
                        if len_a >= len_b {
                            return _PyObject_CAST__2!(unsafe {
                                core::ptr::addr_of_mut!(_Py_TrueStruct)
                            });
                        }
                        return _PyObject_CAST__2!(unsafe {
                            core::ptr::addr_of_mut!(_Py_FalseStruct)
                        });
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
    }
    unsafe { core::mem::zeroed() }
}

#[unsafe(link_section = ".text.slate_distinct.bytesobject.bytes_hash")]
extern "C-unwind" fn bytes_hash(self_: *mut _object) -> i64 {
    let a: *mut __SlateRecord_bytesobject_h_15 = _PyBytes_CAST!(self_);
    let mut hash: i64 = get_ob_shash(a);
    if hash == (1i32.wrapping_neg() as i64) {
        // Can't fail
        hash = unsafe {
            Py_HashBuffer(
                ((*a).ob_sval.as_mut_ptr() as *mut i8) as *const (),
                Py_SIZE!(a),
            )
        };
        set_ob_shash(a, hash);
    }
    hash
}

#[unsafe(link_section = ".text.slate_distinct.bytesobject.bytes_subscript")]
extern "C-unwind" fn bytes_subscript(op: *mut _object, item: *mut _object) -> *mut _object {
    let self_: *mut __SlateRecord_bytesobject_h_15 = _PyBytes_CAST!(op);
    if _PyIndex_Check(item) != 0 {
        let mut i: i64 = unsafe { PyNumber_AsSsize_t(item, PyExc_IndexError) };
        let __v0: bool =
            i == (1i32.wrapping_neg() as i64) && !unsafe { PyErr_Occurred() }.is_null();
        if __v0 {
            return core::ptr::null_mut::<_object>();
        }
        if i < 0 {
            let __v1: i64 = PyBytes_GET_SIZE__2!(self_);
            i = i.wrapping_add(__v1);
        }
        let __v4: bool = i < 0 || i >= PyBytes_GET_SIZE__2!(self_);
        if __v4 {
            unsafe { PyErr_SetString(PyExc_IndexError, c"index out of range".as_ptr()) };
            return core::ptr::null_mut::<_object>();
        }
        return _PyLong_FromUnsignedChar(
            (unsafe { *((*self_).ob_sval.as_mut_ptr() as *mut i8).wrapping_offset(i as isize) })
                as u8,
        );
    } else {
        if PySlice_Check!(item) != 0 {
            let mut start: i64 = 0;
            let mut stop: i64 = 0;
            let mut step: i64 = 0;
            let mut slicelength: i64 = 0;
            let mut cur: u64 = 0;
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
            slicelength = unsafe {
                PySlice_AdjustIndices(
                    PyBytes_GET_SIZE__2!(self_),
                    core::ptr::addr_of_mut!(start),
                    core::ptr::addr_of_mut!(stop),
                    step,
                )
            };
            if slicelength <= 0 {
                return unsafe { Py_GetConstant(8u32) };
            } else {
                let __v5: bool =
                    start == 0 && step == 1 && slicelength == PyBytes_GET_SIZE__2!(self_);
                let __v6: bool = __v5 && PyBytes_CheckExact__2!(self_) != 0;
                if __v6 {
                    return Py_NewRef__2!(self_);
                } else {
                    if step == 1 {
                        return PyBytes_FromStringAndSize(
                            PyBytes_AS_STRING__2!(self_).wrapping_offset(start as isize)
                                as *const i8,
                            slicelength,
                        );
                    } else {
                        let source_buf: *const i8 = PyBytes_AS_STRING__2!(self_) as *const i8;
                        let result: *mut _object =
                            PyBytes_FromStringAndSize(core::ptr::null::<i8>(), slicelength);
                        if result.is_null() {
                            return core::ptr::null_mut::<_object>();
                        }
                        let result_buf: *mut i8 = PyBytes_AS_STRING!(result);
                        cur = start as u64;
                        for i in 0..slicelength {
                            unsafe {
                                *result_buf.wrapping_offset(i as isize) =
                                    *source_buf.wrapping_offset(cur as isize);
                            }
                            cur = cur.wrapping_add(step as u64);
                        }
                        return result;
                    }
                }
            }
        } else {
            unsafe {
                PyErr_Format(
                    PyExc_TypeError,
                    c"byte indices must be integers or slices, not %.200s".as_ptr(),
                    (*Py_TYPE!(item)).tp_name,
                )
            };
            return core::ptr::null_mut::<_object>();
        }
    }
}

#[unsafe(link_section = ".text.slate_distinct.bytesobject.bytes_buffer_getbuffer")]
extern "C-unwind" fn bytes_buffer_getbuffer(
    op: *mut _object,
    view: *mut __SlateRecord_pybuffer_h_33,
    flags: i32,
) -> i32 {
    let self_: *mut __SlateRecord_bytesobject_h_15 = _PyBytes_CAST!(op);
    unsafe {
        PyBuffer_FillInfo(
            view,
            self_ as *mut _object,
            ((*self_).ob_sval.as_mut_ptr() as *mut i8) as *mut (),
            Py_SIZE!(self_),
            1i32,
            flags,
        )
    }
}

/// sq_length
/// sq_concat
/// sq_repeat
/// sq_item
/// sq_slice
/// sq_ass_item
/// sq_ass_slice
/// sq_contains
static mut bytes_as_sequence: __SlateRecord_object_h_120 = {
    let mut __t0: __SlateRecord_object_h_120 = unsafe { core::mem::zeroed() };
    __t0.sq_length = Some(bytes_length);
    __t0.sq_concat = Some(bytes_concat);
    __t0.sq_repeat = Some(bytes_repeat);
    __t0.sq_item = Some(bytes_item);
    __t0.was_sq_slice = core::ptr::null_mut::<()>();
    __t0.sq_ass_item = None;
    __t0.was_sq_ass_slice = core::ptr::null_mut::<()>();
    __t0.sq_contains = Some(bytes_contains);
    __t0
};

static mut bytes_as_mapping: __SlateRecord_object_h_126 = __SlateRecord_object_h_126 {
    mp_length: Some(bytes_length),
    mp_subscript: Some(bytes_subscript),
    mp_ass_subscript: None,
};

static mut bytes_as_buffer: __SlateRecord_object_h_140 = __SlateRecord_object_h_140 {
    bf_getbuffer: Some(bytes_buffer_getbuffer),
    bf_releasebuffer: None,
};

// [clinic input]
// bytes.__bytes__
// Convert this value to exact type bytes.
// [clinic start generated code]
/// [clinic end generated code: output=63a306a9bc0caac5 input=34ec5ddba98bd6bb]
fn bytes___bytes___impl(self_: *mut __SlateRecord_bytesobject_h_15) -> *mut _object {
    if PyBytes_CheckExact__2!(self_) != 0 {
        return Py_NewRef__2!(self_);
    } else {
        return PyBytes_FromStringAndSize(
            (unsafe { (*self_).ob_sval.as_mut_ptr() as *mut i8 }) as *const i8,
            Py_SIZE!(self_),
        );
    }
}

// [clinic input]
// bytes.split
//
//     sep: object = None
//         The delimiter according which to split the bytes.
//         None (the default value) means split on ASCII whitespace
//         characters (space, tab, return, newline, formfeed, vertical tab).
//     maxsplit: Py_ssize_t = -1
//         Maximum number of splits to do.
//         -1 (the default value) means no limit.
//
// Return a list of the sections in the bytes, using sep as the delimiter.
// [clinic start generated code]
/// [clinic end generated code: output=52126b5844c1d8ef input=9c0faeffc870f672]
fn bytes_split_impl(
    self_: *mut __SlateRecord_bytesobject_h_15,
    sep: *mut _object,
    mut maxsplit: i64,
) -> *mut _object {
    let len: i64 = PyBytes_GET_SIZE__2!(self_);
    let mut n: i64 = 0;
    let s: *const i8 = PyBytes_AS_STRING__2!(self_) as *const i8;
    let mut vsub: __SlateRecord_pybuffer_h_33 = unsafe { core::mem::zeroed() };
    if maxsplit < 0 {
        maxsplit = 9223372036854775807;
    }
    if sep == unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } {
        return stringlib_split_whitespace(self_ as *mut _object, s, len, maxsplit);
    }
    if (unsafe { PyObject_GetBuffer(sep, core::ptr::addr_of_mut!(vsub), 0i32) }) != 0 {
        return core::ptr::null_mut::<_object>();
    }
    let sub: *const i8 = vsub.buf as *const i8;
    n = vsub.len;
    let list: *mut _object = stringlib_split(self_ as *mut _object, s, len, sub, n, maxsplit);
    unsafe { PyBuffer_Release(core::ptr::addr_of_mut!(vsub)) };
    list
}

// [clinic input]
// bytes.partition
//
//     sep: Py_buffer
//     /
//
// Partition the bytes into three parts using the given separator.
//
// This will search for the separator sep in the bytes.  If the
// separator is found, returns a 3-tuple containing the part before the
// separator, the separator itself, and the part after it.
//
// If the separator is not found, returns a 3-tuple containing the
// original bytes object and two empty bytes objects.
// [clinic start generated code]
/// [clinic end generated code: output=f532b392a17ff695 input=2e6e551ea4f8b95a]
fn bytes_partition_impl(
    self_: *mut __SlateRecord_bytesobject_h_15,
    sep: &__SlateRecord_pybuffer_h_33,
) -> *mut _object {
    stringlib_partition(
        self_ as *mut _object,
        PyBytes_AS_STRING__2!(self_) as *const i8,
        PyBytes_GET_SIZE__2!(self_),
        unsafe { (*(sep as *const __SlateRecord_pybuffer_h_33)).obj },
        (unsafe { (*(sep as *const __SlateRecord_pybuffer_h_33)).buf }) as *const i8,
        unsafe { (*(sep as *const __SlateRecord_pybuffer_h_33)).len },
    )
}

// [clinic input]
// bytes.rpartition
//
//     sep: Py_buffer
//     /
//
// Partition the bytes into three parts using the given separator.
//
// This will search for the separator sep in the bytes, starting at the
// end.  If the separator is found, returns a 3-tuple containing the
// part before the separator, the separator itself, and the part after
// it.
//
// If the separator is not found, returns a 3-tuple containing two
// empty bytes objects and the original bytes object.
// [clinic start generated code]
/// [clinic end generated code: output=191b114cbb028e50 input=f7d24f722a5470a4]
fn bytes_rpartition_impl(
    self_: *mut __SlateRecord_bytesobject_h_15,
    sep: &__SlateRecord_pybuffer_h_33,
) -> *mut _object {
    stringlib_rpartition(
        self_ as *mut _object,
        PyBytes_AS_STRING__2!(self_) as *const i8,
        PyBytes_GET_SIZE__2!(self_),
        unsafe { (*(sep as *const __SlateRecord_pybuffer_h_33)).obj },
        (unsafe { (*(sep as *const __SlateRecord_pybuffer_h_33)).buf }) as *const i8,
        unsafe { (*(sep as *const __SlateRecord_pybuffer_h_33)).len },
    )
}

// [clinic input]
// bytes.rsplit = bytes.split
//
// Return a list of the sections in the bytes, using sep as the delimiter.
//
// Splitting is done starting at the end of the bytes and working to
// the front.
// [clinic start generated code]
/// [clinic end generated code: output=ba698d9ea01e1c8f input=7fd643d4337b6a9b]
fn bytes_rsplit_impl(
    self_: *mut __SlateRecord_bytesobject_h_15,
    sep: *mut _object,
    mut maxsplit: i64,
) -> *mut _object {
    let len: i64 = PyBytes_GET_SIZE__2!(self_);
    let mut n: i64 = 0;
    let s: *const i8 = PyBytes_AS_STRING__2!(self_) as *const i8;
    let mut vsub: __SlateRecord_pybuffer_h_33 = unsafe { core::mem::zeroed() };
    if maxsplit < 0 {
        maxsplit = 9223372036854775807;
    }
    if sep == unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } {
        return stringlib_rsplit_whitespace(self_ as *mut _object, s, len, maxsplit);
    }
    if (unsafe { PyObject_GetBuffer(sep, core::ptr::addr_of_mut!(vsub), 0i32) }) != 0 {
        return core::ptr::null_mut::<_object>();
    }
    let sub: *const i8 = vsub.buf as *const i8;
    n = vsub.len;
    let list: *mut _object = stringlib_rsplit(self_ as *mut _object, s, len, sub, n, maxsplit);
    unsafe { PyBuffer_Release(core::ptr::addr_of_mut!(vsub)) };
    list
}

// [clinic input]
// bytes.join
//
//     iterable_of_bytes: object
//     /
//
// Concatenate any number of bytes objects.
//
// The bytes whose method is called is inserted in between each pair.
//
// The result is returned as a new bytes object.
//
// Example: b'.'.join([b'ab', b'pq', b'rs']) -> b'ab.pq.rs'.
// [clinic start generated code]
/// [clinic end generated code: output=0687abb94d7d438e input=7fe377b95bd549d2]
fn bytes_join_impl(
    self_: *mut __SlateRecord_bytesobject_h_15,
    iterable_of_bytes: *mut _object,
) -> *mut _object {
    stringlib_bytes_join(self_ as *mut _object, iterable_of_bytes)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyBytes_Join(sep: *mut _object, iterable: *mut _object) -> *mut _object {
    if sep.is_null() {
        PyErr_BadInternalCall__90!();
        return core::ptr::null_mut::<_object>();
    }
    if PyBytes_Check!(sep) == 0 {
        unsafe {
            PyErr_Format(
                PyExc_TypeError,
                c"sep: expected bytes, got %T".as_ptr(),
                sep,
            )
        };
        return core::ptr::null_mut::<_object>();
    }
    stringlib_bytes_join(sep, iterable)
}

// [clinic input]
// @text_signature "($self, sub[, start[, end]], /)"
// bytes.find
//
//     sub: object
//     start: slice_index(accept={int, NoneType}, c_default='0') = None
//          Optional start position. Default: start of the bytes.
//     end: slice_index(accept={int, NoneType}, c_default='PY_SSIZE_T_MAX') = None
//          Optional stop position. Default: end of the bytes.
//     /
//
// Return the lowest index in B where subsection 'sub' is found, such that 'sub' is contained within B[start,end].
//
// Return -1 on failure.
// [clinic start generated code]
/// [clinic end generated code: output=d5961a1c77b472a1 input=3171e62a8ae7f240]
fn bytes_find_impl(
    self_: *mut __SlateRecord_bytesobject_h_15,
    sub: *mut _object,
    start: i64,
    end: i64,
) -> *mut _object {
    unsafe {
        _Py_bytes_find(
            PyBytes_AS_STRING__2!(self_) as *const i8,
            PyBytes_GET_SIZE__2!(self_),
            sub,
            start,
            end,
        )
    }
}

// [clinic input]
// bytes.index = bytes.find
//
// Return the lowest index in B where subsection 'sub' is found, such that 'sub' is contained within B[start,end].
//
// Raise ValueError if the subsection is not found.
// [clinic start generated code]
/// [clinic end generated code: output=0da25cc74683ba42 input=aa34ad71ba0bafe3]
fn bytes_index_impl(
    self_: *mut __SlateRecord_bytesobject_h_15,
    sub: *mut _object,
    start: i64,
    end: i64,
) -> *mut _object {
    unsafe {
        _Py_bytes_index(
            PyBytes_AS_STRING__2!(self_) as *const i8,
            PyBytes_GET_SIZE__2!(self_),
            sub,
            start,
            end,
        )
    }
}

// [clinic input]
// bytes.rfind = bytes.find
//
// Return the highest index in B where subsection 'sub' is found, such that 'sub' is contained within B[start,end].
//
// Return -1 on failure.
// [clinic start generated code]
/// [clinic end generated code: output=51b60fa4ad011c09 input=864c3e7f3010b33c]
fn bytes_rfind_impl(
    self_: *mut __SlateRecord_bytesobject_h_15,
    sub: *mut _object,
    start: i64,
    end: i64,
) -> *mut _object {
    unsafe {
        _Py_bytes_rfind(
            PyBytes_AS_STRING__2!(self_) as *const i8,
            PyBytes_GET_SIZE__2!(self_),
            sub,
            start,
            end,
        )
    }
}

// [clinic input]
// bytes.rindex = bytes.find
//
// Return the highest index in B where subsection 'sub' is found, such that 'sub' is contained within B[start,end].
//
// Raise ValueError if the subsection is not found.
// [clinic start generated code]
/// [clinic end generated code: output=42bf674e0a0aabf6 input=21051fc5cfeacf2c]
fn bytes_rindex_impl(
    self_: *mut __SlateRecord_bytesobject_h_15,
    sub: *mut _object,
    start: i64,
    end: i64,
) -> *mut _object {
    unsafe {
        _Py_bytes_rindex(
            PyBytes_AS_STRING__2!(self_) as *const i8,
            PyBytes_GET_SIZE__2!(self_),
            sub,
            start,
            end,
        )
    }
}

fn do_xstrip(
    self_: *mut __SlateRecord_bytesobject_h_15,
    striptype: i32,
    sepobj: *mut _object,
) -> *mut _object {
    let mut vsep: __SlateRecord_pybuffer_h_33 = unsafe { core::mem::zeroed() };
    let s: *const i8 = PyBytes_AS_STRING__2!(self_) as *const i8;
    let len: i64 = PyBytes_GET_SIZE__2!(self_);
    let mut seplen: i64 = 0;
    let mut i: i64 = 0;
    let mut j: i64 = 0;
    if (unsafe { PyObject_GetBuffer(sepobj, core::ptr::addr_of_mut!(vsep), 0i32) }) != 0 {
        return core::ptr::null_mut::<_object>();
    }
    let sep: *mut i8 = vsep.buf as *mut i8;
    seplen = vsep.len;
    if striptype != 1 {
        while i < len
            && !unsafe {
                memchr(
                    (sep as *const ()) as *const core::ffi::c_void,
                    ((((*s.wrapping_offset(i as isize) as i32) & 255) as u8) as i32) as i32,
                    (seplen as u64) as usize,
                ) as *mut ()
            }
            .is_null()
        {
            i = i.wrapping_add(1);
        }
    }
    j = len;
    if striptype != 0 {
        loop {
            j = j.wrapping_sub(1);
            if !(j >= i
                && !unsafe {
                    memchr(
                        (sep as *const ()) as *const core::ffi::c_void,
                        ((((*s.wrapping_offset(j as isize) as i32) & 255) as u8) as i32) as i32,
                        (seplen as u64) as usize,
                    ) as *mut ()
                }
                .is_null())
            {
                break;
            }
        }
        j = j.wrapping_add(1);
    }
    unsafe { PyBuffer_Release(core::ptr::addr_of_mut!(vsep)) };
    let __v8: bool = i == 0 && j == len && PyBytes_CheckExact__2!(self_) != 0;
    if __v8 {
        return Py_NewRef__2!(self_);
    } else {
        return PyBytes_FromStringAndSize(s.wrapping_offset(i as isize), j.wrapping_sub(i));
    }
}

fn do_strip(self_: *mut __SlateRecord_bytesobject_h_15, striptype: i32) -> *mut _object {
    let s: *const i8 = PyBytes_AS_STRING__2!(self_) as *const i8;
    let len: i64 = PyBytes_GET_SIZE__2!(self_);
    let mut i: i64 = 0;
    let mut j: i64 = 0;
    if striptype != 1 {
        while i < len && Py_ISSPACE!(unsafe { *s.wrapping_offset(i as isize) }) != 0 {
            i = i.wrapping_add(1);
        }
    }
    j = len;
    if striptype != 0 {
        loop {
            j = j.wrapping_sub(1);
            if !(j >= i && Py_ISSPACE!(unsafe { *s.wrapping_offset(j as isize) }) != 0) {
                break;
            }
        }
        j = j.wrapping_add(1);
    }
    let __v8: bool = i == 0 && j == len && PyBytes_CheckExact__2!(self_) != 0;
    if __v8 {
        return Py_NewRef__2!(self_);
    } else {
        return PyBytes_FromStringAndSize(s.wrapping_offset(i as isize), j.wrapping_sub(i));
    }
}

fn do_argstrip(
    self_: *mut __SlateRecord_bytesobject_h_15,
    striptype: i32,
    bytes: *mut _object,
) -> *mut _object {
    if bytes != unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } {
        return do_xstrip(self_, striptype, bytes);
    }
    do_strip(self_, striptype)
}

// [clinic input]
// bytes.strip
//
//     bytes: object = None
//     /
//
// Strip leading and trailing bytes contained in the argument.
//
// If the argument is omitted or None, strip leading and trailing ASCII
// whitespace.
// [clinic start generated code]
/// [clinic end generated code: output=c7c228d3bd104a1b input=9ffea5f752032bd0]
fn bytes_strip_impl(
    self_: *mut __SlateRecord_bytesobject_h_15,
    bytes: *mut _object,
) -> *mut _object {
    do_argstrip(self_, 2i32, bytes)
}

// [clinic input]
// bytes.lstrip
//
//     bytes: object = None
//     /
//
// Strip leading bytes contained in the argument.
//
// If the argument is omitted or None, strip leading  ASCII whitespace.
// [clinic start generated code]
/// [clinic end generated code: output=28602e586f524e82 input=9baff4398c3f6857]
fn bytes_lstrip_impl(
    self_: *mut __SlateRecord_bytesobject_h_15,
    bytes: *mut _object,
) -> *mut _object {
    do_argstrip(self_, 0i32, bytes)
}

// [clinic input]
// bytes.rstrip
//
//     bytes: object = None
//     /
//
// Strip trailing bytes contained in the argument.
//
// If the argument is omitted or None, strip trailing ASCII whitespace.
// [clinic start generated code]
/// [clinic end generated code: output=547e3815c95447da input=b78af445c727e32b]
fn bytes_rstrip_impl(
    self_: *mut __SlateRecord_bytesobject_h_15,
    bytes: *mut _object,
) -> *mut _object {
    do_argstrip(self_, 1i32, bytes)
}

// [clinic input]
// bytes.count = bytes.find
//
// Return the number of non-overlapping occurrences of subsection 'sub' in bytes B[start:end].
// [clinic start generated code]
/// [clinic end generated code: output=9848140b9be17d0f input=b6e4a5ed515e1e59]
fn bytes_count_impl(
    self_: *mut __SlateRecord_bytesobject_h_15,
    sub: *mut _object,
    start: i64,
    end: i64,
) -> *mut _object {
    unsafe {
        _Py_bytes_count(
            PyBytes_AS_STRING__2!(self_) as *const i8,
            PyBytes_GET_SIZE__2!(self_),
            sub,
            start,
            end,
        )
    }
}

// [clinic input]
// bytes.translate
//
//     table: object
//         Translation table, which must be a bytes object of length 256.
//     /
//     delete as deletechars: object(c_default="NULL") = b''
//
// Return a copy with each character mapped by the given translation table.
//
// All characters occurring in the optional argument delete are
// removed.  The remaining characters are mapped through the given
// translation table.
// [clinic start generated code]
/// [clinic end generated code: output=43be3437f1956211 input=4e4460a981d768c5]
fn bytes_translate_impl(
    self_: *mut __SlateRecord_bytesobject_h_15,
    table: *mut _object,
    deletechars: *mut _object,
) -> *mut _object {
    let mut input: *const i8 = unsafe { core::mem::zeroed() };
    let mut output: *mut i8 = unsafe { core::mem::zeroed() };
    let mut table_view: __SlateRecord_pybuffer_h_33 = {
        let mut __t0: __SlateRecord_pybuffer_h_33 = unsafe { core::mem::zeroed() };
        __t0.buf = core::ptr::null_mut::<()>();
        __t0.obj = core::ptr::null_mut::<_object>();
        __t0
    };
    let mut del_table_view: __SlateRecord_pybuffer_h_33 = {
        let mut __t1: __SlateRecord_pybuffer_h_33 = unsafe { core::mem::zeroed() };
        __t1.buf = core::ptr::null_mut::<()>();
        __t1.obj = core::ptr::null_mut::<_object>();
        __t1
    };
    let mut table_chars: *const i8 = unsafe { core::mem::zeroed() };
    let mut i: i64 = 0;
    let mut c: i64 = 0;
    let mut changed: i64 = 0;
    let input_obj: *mut _object = self_ as *mut _object;
    let mut del_table_chars: *const i8 = core::ptr::null::<i8>();
    let mut inlen: i64 = 0;
    let mut tablen: i64 = 0;
    let mut dellen: i64 = 0;
    let mut result: *mut _object = unsafe { core::mem::zeroed() };
    let mut trans_table: __SlateAlign16<[i32; 256]> = __SlateAlign16([0i32; 256]);
    if PyBytes_Check!(table) != 0 {
        table_chars = PyBytes_AS_STRING!(table) as *const i8;
        tablen = PyBytes_GET_SIZE!(table);
    } else {
        if table == unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) } {
            table_chars = core::ptr::null::<i8>();
            tablen = 256;
        } else {
            if (unsafe { PyObject_GetBuffer(table, core::ptr::addr_of_mut!(table_view), 0i32) })
                != 0
            {
                return core::ptr::null_mut::<_object>();
            }
            table_chars = table_view.buf as *const i8;
            tablen = table_view.len;
        }
    }
    if tablen != 256 {
        unsafe {
            PyErr_SetString(
                PyExc_ValueError,
                c"translation table must be 256 characters long".as_ptr(),
            )
        };
        unsafe { PyBuffer_Release(core::ptr::addr_of_mut!(table_view)) };
        return core::ptr::null_mut::<_object>();
    }
    if !deletechars.is_null() {
        if PyBytes_Check!(deletechars) != 0 {
            del_table_chars = PyBytes_AS_STRING!(deletechars) as *const i8;
            dellen = PyBytes_GET_SIZE!(deletechars);
        } else {
            if (unsafe {
                PyObject_GetBuffer(deletechars, core::ptr::addr_of_mut!(del_table_view), 0i32)
            }) != 0
            {
                unsafe { PyBuffer_Release(core::ptr::addr_of_mut!(table_view)) };
                return core::ptr::null_mut::<_object>();
            }
            del_table_chars = del_table_view.buf as *const i8;
            dellen = del_table_view.len;
        }
    } else {
        del_table_chars = core::ptr::null::<i8>();
        dellen = 0;
    }
    inlen = PyBytes_GET_SIZE!(input_obj);
    result = PyBytes_FromStringAndSize(core::ptr::null_mut::<i8>() as *const i8, inlen);
    if result.is_null() {
        unsafe { PyBuffer_Release(core::ptr::addr_of_mut!(del_table_view)) };
        unsafe { PyBuffer_Release(core::ptr::addr_of_mut!(table_view)) };
        return core::ptr::null_mut::<_object>();
    }
    let __v0: *mut i8 = PyBytes_AS_STRING!(result);
    output = __v0;
    let output_start: *const i8 = __v0 as *const i8;
    input = PyBytes_AS_STRING!(input_obj) as *const i8;
    if dellen == 0 && !table_chars.is_null() {
        // If no deletions are required, use faster code
        i = inlen;
        loop {
            let __v7: i64 = i.wrapping_sub(1);
            i = __v7;
            if __v7 < 0 {
                break;
            }
            let __v1: *const i8 = input;
            input = __v1.wrapping_offset(1 as isize);
            c = ((((unsafe { *__v1 }) as i32) & 255) as u8) as i64;
            let __v3: *mut i8 = output;
            output = __v3.wrapping_offset(1 as isize);
            let __v5: i8 = unsafe { *table_chars.wrapping_offset(c as isize) };
            unsafe {
                *__v3 = __v5;
            }
            if (((((__v5 as i32) & 255) as u8) as i32) as i64) != c {
                changed = 1;
            }
        }
        let __v9: bool = changed == 0 && PyBytes_CheckExact!(input_obj) != 0;
        if __v9 {
            Py_SETREF!(result, Py_NewRef!(input_obj));
        }
        unsafe { PyBuffer_Release(core::ptr::addr_of_mut!(del_table_view)) };
        unsafe { PyBuffer_Release(core::ptr::addr_of_mut!(table_view)) };
        return result;
    }
    if table_chars.is_null() {
        i = 0;
        while i < 256 {
            trans_table.0[i as usize] = Py_CHARMASK__4!(i) as i32;
            i = i.wrapping_add(1);
        }
    } else {
        i = 0;
        while i < 256 {
            trans_table.0[i as usize] =
                Py_CHARMASK!(unsafe { *table_chars.wrapping_offset(i as isize) }) as i32;
            i = i.wrapping_add(1);
        }
    }
    unsafe { PyBuffer_Release(core::ptr::addr_of_mut!(table_view)) };
    i = 0;
    while i < dellen {
        trans_table.0[(Py_CHARMASK!(unsafe { *del_table_chars.wrapping_offset(i as isize) }) as i32)
            as usize] = 1i32.wrapping_neg();
        i = i.wrapping_add(1);
    }
    unsafe { PyBuffer_Release(core::ptr::addr_of_mut!(del_table_view)) };
    i = inlen;
    loop {
        let __v27: i64 = i.wrapping_sub(1);
        i = __v27;
        if __v27 < 0 {
            break;
        }
        '__slate_continue_25: {
            let __v20: *const i8 = input;
            input = __v20.wrapping_offset(1 as isize);
            c = ((((unsafe { *__v20 }) as i32) & 255) as u8) as i64;
            if trans_table.0[c as usize] != 1i32.wrapping_neg() {
                let __v22: *mut i8 = output;
                output = __v22.wrapping_offset(1 as isize);
                let __v24: i8 = trans_table.0[c as usize] as i8;
                unsafe {
                    *__v22 = __v24;
                }
                if (((((__v24 as i32) & 255) as u8) as i32) as i64) == c {
                    break '__slate_continue_25;
                }
            }
            changed = 1;
        }
    }
    let __v28: bool = changed == 0 && PyBytes_CheckExact!(input_obj) != 0;
    if __v28 {
        Py_DECREF!(result);
        return Py_NewRef!(input_obj);
    }
    // Fix the size of the resulting byte string
    if inlen > 0 {
        _PyBytes_Resize(
            core::ptr::addr_of_mut!(result),
            (unsafe { output.offset_from(output_start as *mut i8) }) as i64,
        );
    }
    result
}

// [clinic input]
//
// @staticmethod
// bytes.maketrans
//
//     frm: Py_buffer
//     to: Py_buffer
//     /
//
// Return a translation table usable for the bytes or bytearray translate method.
//
// The returned table will be one where each byte in frm is mapped to
// the byte at the same position in to.
//
// The bytes objects frm and to must be of the same length.
// [clinic start generated code]
/// [clinic end generated code: output=a36f6399d4b77f6f input=c88bcc17621f785d]
fn bytes_maketrans_impl(
    frm: *mut __SlateRecord_pybuffer_h_33,
    to: *mut __SlateRecord_pybuffer_h_33,
) -> *mut _object {
    unsafe { _Py_bytes_maketrans(frm, to) }
}

// [clinic input]
// bytes.replace
//
//     old: Py_buffer
//     new: Py_buffer
//     count: Py_ssize_t = -1
//         Maximum number of occurrences to replace.
//         -1 (the default value) means replace all occurrences.
//     /
//
// Return a copy with all occurrences of substring old replaced by new.
//
// If the optional argument count is given, only the first count occurrences are
// replaced.
// [clinic start generated code]
/// [clinic end generated code: output=994fa588b6b9c104 input=b2fbbf0bf04de8e5]
fn bytes_replace_impl(
    self_: *mut __SlateRecord_bytesobject_h_15,
    old: &__SlateRecord_pybuffer_h_33,
    new: &__SlateRecord_pybuffer_h_33,
    count: i64,
) -> *mut _object {
    stringlib_replace(
        self_ as *mut _object,
        (unsafe { (*(old as *const __SlateRecord_pybuffer_h_33)).buf }) as *const i8,
        unsafe { (*(old as *const __SlateRecord_pybuffer_h_33)).len },
        (unsafe { (*(new as *const __SlateRecord_pybuffer_h_33)).buf }) as *const i8,
        unsafe { (*(new as *const __SlateRecord_pybuffer_h_33)).len },
        count,
    )
}

// End DALKE
// [clinic input]
// bytes.removeprefix as bytes_removeprefix
//
//     prefix: Py_buffer
//     /
//
// Return a bytes object with the given prefix string removed if present.
//
// If the bytes starts with the prefix string, return
// bytes[len(prefix):].  Otherwise, return a copy of the original
// bytes.
// [clinic start generated code]
/// [clinic end generated code: output=f006865331a06ab6 input=8f371f9421b8addd]
fn bytes_removeprefix_impl(
    self_: *mut __SlateRecord_bytesobject_h_15,
    prefix: &__SlateRecord_pybuffer_h_33,
) -> *mut _object {
    let self_start: *const i8 = PyBytes_AS_STRING__2!(self_) as *const i8;
    let self_len: i64 = PyBytes_GET_SIZE__2!(self_);
    let prefix_start: *const i8 =
        (unsafe { (*(prefix as *const __SlateRecord_pybuffer_h_33)).buf }) as *const i8;
    let prefix_len: i64 = unsafe { (*(prefix as *const __SlateRecord_pybuffer_h_33)).len };
    if self_len >= prefix_len
        && prefix_len > 0
        && (unsafe {
            memcmp(
                (self_start as *const ()) as *const core::ffi::c_void,
                (prefix_start as *const ()) as *const core::ffi::c_void,
                (prefix_len as u64) as usize,
            ) as i32
        }) == 0
    {
        return PyBytes_FromStringAndSize(
            self_start.wrapping_offset(prefix_len as isize),
            self_len.wrapping_sub(prefix_len),
        );
    }
    if PyBytes_CheckExact__2!(self_) != 0 {
        return Py_NewRef__2!(self_);
    }
    PyBytes_FromStringAndSize(self_start, self_len)
}

// [clinic input]
// bytes.removesuffix as bytes_removesuffix
//
//     suffix: Py_buffer
//     /
//
// Return a bytes object with the given suffix string removed if present.
//
// If the bytes ends with the suffix string and that suffix is not
// empty, return bytes[:-len(prefix)].  Otherwise, return a copy of the
// original bytes.
// [clinic start generated code]
/// [clinic end generated code: output=d887d308e3242eeb input=35eada0260d1171b]
fn bytes_removesuffix_impl(
    self_: *mut __SlateRecord_bytesobject_h_15,
    suffix: &__SlateRecord_pybuffer_h_33,
) -> *mut _object {
    let self_start: *const i8 = PyBytes_AS_STRING__2!(self_) as *const i8;
    let self_len: i64 = PyBytes_GET_SIZE__2!(self_);
    let suffix_start: *const i8 =
        (unsafe { (*(suffix as *const __SlateRecord_pybuffer_h_33)).buf }) as *const i8;
    let suffix_len: i64 = unsafe { (*(suffix as *const __SlateRecord_pybuffer_h_33)).len };
    if self_len >= suffix_len
        && suffix_len > 0
        && (unsafe {
            memcmp(
                (self_start
                    .wrapping_offset(self_len as isize)
                    .wrapping_offset(-(suffix_len as isize)) as *const ())
                    as *const core::ffi::c_void,
                (suffix_start as *const ()) as *const core::ffi::c_void,
                (suffix_len as u64) as usize,
            ) as i32
        }) == 0
    {
        return PyBytes_FromStringAndSize(self_start, self_len.wrapping_sub(suffix_len));
    }
    if PyBytes_CheckExact__2!(self_) != 0 {
        return Py_NewRef__2!(self_);
    }
    PyBytes_FromStringAndSize(self_start, self_len)
}

// [clinic input]
// @text_signature "($self, prefix[, start[, end]], /)"
// bytes.startswith
//
//     prefix as subobj: object
//         A bytes or a tuple of bytes to try.
//     start: slice_index(accept={int, NoneType}, c_default='0') = None
//         Optional start position. Default: start of the bytes.
//     end: slice_index(accept={int, NoneType}, c_default='PY_SSIZE_T_MAX') = None
//         Optional stop position. Default: end of the bytes.
//     /
//
// Return True if the bytes starts with the specified prefix, False otherwise.
// [clinic start generated code]
/// [clinic end generated code: output=b1e8da1cbd528e8c input=8a4165df8adfa6c9]
fn bytes_startswith_impl(
    self_: *mut __SlateRecord_bytesobject_h_15,
    subobj: *mut _object,
    start: i64,
    end: i64,
) -> *mut _object {
    unsafe {
        _Py_bytes_startswith(
            PyBytes_AS_STRING__2!(self_) as *const i8,
            PyBytes_GET_SIZE__2!(self_),
            subobj,
            start,
            end,
        )
    }
}

// [clinic input]
// @text_signature "($self, suffix[, start[, end]], /)"
// bytes.endswith
//
//     suffix as subobj: object
//         A bytes or a tuple of bytes to try.
//     start: slice_index(accept={int, NoneType}, c_default='0') = None
//          Optional start position. Default: start of the bytes.
//     end: slice_index(accept={int, NoneType}, c_default='PY_SSIZE_T_MAX') = None
//          Optional stop position. Default: end of the bytes.
//     /
//
// Return True if the bytes ends with the specified suffix, False otherwise.
// [clinic start generated code]
/// [clinic end generated code: output=038b633111f3629d input=b5c3407a2a5c9aac]
fn bytes_endswith_impl(
    self_: *mut __SlateRecord_bytesobject_h_15,
    subobj: *mut _object,
    start: i64,
    end: i64,
) -> *mut _object {
    unsafe {
        _Py_bytes_endswith(
            PyBytes_AS_STRING__2!(self_) as *const i8,
            PyBytes_GET_SIZE__2!(self_),
            subobj,
            start,
            end,
        )
    }
}

// [clinic input]
// bytes.decode
//
//     encoding: str(c_default="NULL") = 'utf-8'
//         The encoding with which to decode the bytes.
//     errors: str(c_default="NULL") = 'strict'
//         The error handling scheme to use for the handling of decoding
//         errors.  The default is 'strict' meaning that decoding errors
//         raise a UnicodeDecodeError.  Other possible values are 'ignore'
//         and 'replace' as well as any other name registered with
//         codecs.register_error that can handle UnicodeDecodeErrors.
//
// Decode the bytes using the codec registered for encoding.
// [clinic start generated code]
/// [clinic end generated code: output=5649a53dde27b314 input=94e9b8524f1d7f37]
fn bytes_decode_impl(
    self_: *mut __SlateRecord_bytesobject_h_15,
    encoding: *const i8,
    errors: *const i8,
) -> *mut _object {
    unsafe { PyUnicode_FromEncodedObject(self_ as *mut _object, encoding, errors) }
}

// [clinic input]
// bytes.splitlines
//
//     keepends: bool = False
//
// Return a list of the lines in the bytes, breaking at line boundaries.
//
// Line breaks are not included in the resulting list unless keepends
// is given and true.
// [clinic start generated code]
/// [clinic end generated code: output=3484149a5d880ffb input=8b7b6915be775bcf]
fn bytes_splitlines_impl(
    self_: *mut __SlateRecord_bytesobject_h_15,
    keepends: i32,
) -> *mut _object {
    stringlib_splitlines(
        self_ as *mut _object,
        PyBytes_AS_STRING__2!(self_) as *const i8,
        PyBytes_GET_SIZE__2!(self_),
        keepends,
    )
}

// [clinic input]
// @classmethod
// bytes.fromhex
//
//     string: object
//     /
//
// Create a bytes object from a string of hexadecimal numbers.
//
// Spaces between two numbers are accepted.
// Example: bytes.fromhex('B9 01EF') -> b'\\xb9\\x01\\xef'.
// [clinic start generated code]
/// [clinic end generated code: output=0973acc63661bb2e input=f37d98ed51088a21]
fn bytes_fromhex_impl(r#type: *mut _typeobject, string: *mut _object) -> *mut _object {
    let mut result: *mut _object = _PyBytes_FromHex(string, 0i32);
    if r#type != unsafe { core::ptr::addr_of_mut!(PyBytes_Type) } && !result.is_null() {
        Py_SETREF!(result, unsafe {
            PyObject_CallOneArg(r#type as *mut _object, result)
        });
    }
    result
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyBytes_FromHex(string: *mut _object, use_bytearray: i32) -> *mut _object {
    let mut i: i64 = unsafe { core::mem::zeroed() };
    let mut kind: i32 = unsafe { core::mem::zeroed() };
    let mut data: *const () = unsafe { core::mem::zeroed() };
    let mut view: core::mem::MaybeUninit<__SlateRecord_pybuffer_h_33> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_13: *mut __SlateRecord_pybuffer_h_33 =
        core::ptr::addr_of_mut!(view) as *mut __SlateRecord_pybuffer_h_33;
    let mut writer: core::mem::MaybeUninit<__SlateRecord_pycore_bytesobject_h_89> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_14: *mut __SlateRecord_pycore_bytesobject_h_89 =
        core::ptr::addr_of_mut!(writer) as *mut __SlateRecord_pycore_bytesobject_h_89;
    let mut end: *const u8 = unsafe { core::mem::zeroed() };
    let mut start: *const u8 = unsafe { core::mem::zeroed() };
    let mut str: *const u8 = unsafe { core::mem::zeroed() };
    let mut bot: u32 = unsafe { core::mem::zeroed() };
    let mut top: u32 = unsafe { core::mem::zeroed() };
    let mut invalid_char: i64 = unsafe { core::mem::zeroed() };
    let mut hexlen: i64 = unsafe { core::mem::zeroed() };
    let mut buf: *mut i8 = unsafe { core::mem::zeroed() };
    unsafe {
        '__join_2: {
            '__join_6: {
                (*__slate_slot_13).obj = core::ptr::null_mut::<_object>();
                _PyBytesWriter_Init(core::ptr::addr_of_mut!(*__slate_slot_14));
                (*__slate_slot_14).use_bytearray = use_bytearray;
                if PyUnicode_Check!(string) != 0 {
                    hexlen = PyUnicode_GET_LENGTH!(string);
                    if PyUnicode_IS_ASCII!(string) == 0 {
                        data = PyUnicode_DATA!(string) as *const ();
                        kind = PyUnicode_KIND!(string);
                        // search for the first non-ASCII character
                        i = 0;
                        while i < hexlen {
                            if PyUnicode_READ!(kind, data, i) >= 128 {
                                break;
                            } else {
                                i = i.wrapping_add(1);
                            }
                        }
                        invalid_char = i;
                        break '__join_6;
                    } else {
                        str = PyUnicode_1BYTE_DATA!(string) as *const u8;
                    }
                } else {
                    if PyObject_CheckBuffer(string) != 0 {
                        if PyObject_GetBuffer(
                            string,
                            core::ptr::addr_of_mut!(*__slate_slot_13),
                            0i32,
                        ) != 0
                        {
                            return core::ptr::null_mut::<_object>();
                        } else {
                            hexlen = (*__slate_slot_13).len;
                            str = (*__slate_slot_13).buf as *const u8;
                        }
                    } else {
                        PyErr_Format(
                            PyExc_TypeError,
                            c"fromhex() argument must be str or bytes-like, not %T".as_ptr(),
                            string,
                        );
                        return core::ptr::null_mut::<_object>();
                    }
                }
                // This overestimates if there are spaces
                buf = _PyBytesWriter_Alloc(core::ptr::addr_of_mut!(*__slate_slot_14), hexlen / 2)
                    as *mut i8;
                if buf.is_null() {
                    break '__join_2;
                } else {
                    start = str;
                    end = str.wrapping_offset(hexlen as isize);
                    '__join_9: {
                        '__join_16: {
                            loop {
                                if str < end {
                                    // skip over spaces in the input
                                    if Py_ISSPACE!(unsafe { *str }) != 0 {
                                        loop {
                                            let __v7: *const u8 = str;
                                            str = __v7.wrapping_offset(1 as isize);
                                            if Py_ISSPACE!(unsafe { *str }) == 0 {
                                                break;
                                            }
                                        }
                                        if str >= end {
                                            break '__join_9;
                                        }
                                    }
                                    top = _PyLong_DigitValue[(*str as i32) as usize] as u32;
                                    if top >= 16 {
                                        break '__join_16;
                                    } else {
                                        let __v5: *const u8 = str;
                                        str = __v5.wrapping_offset(1 as isize);
                                        bot = _PyLong_DigitValue[(*str as i32) as usize] as u32;
                                        if bot >= 16 {
                                            break;
                                        } else {
                                            let __v3: *const u8 = str;
                                            str = __v3.wrapping_offset(1 as isize);
                                            let __v1: *mut i8 = buf;
                                            buf = __v1.wrapping_offset(1 as isize);
                                            *__v1 = (top << 4i32).wrapping_add(bot) as i8;
                                        }
                                    }
                                } else {
                                    break '__join_9;
                                }
                            }
                            // Check if we had a second digit
                            if str >= end {
                                invalid_char = 1i32.wrapping_neg() as i64;
                                break '__join_6;
                            } else {
                                invalid_char = str.offset_from(start as *const u8) as i64;
                                break '__join_6;
                            }
                        }
                        invalid_char = str.offset_from(start as *const u8) as i64;
                        break '__join_6;
                    }
                    if !(*__slate_slot_13).obj.is_null() {
                        PyBuffer_Release(core::ptr::addr_of_mut!(*__slate_slot_13));
                    }
                    return _PyBytesWriter_Finish(
                        core::ptr::addr_of_mut!(*__slate_slot_14),
                        buf as *mut (),
                    );
                }
            }
            if invalid_char == (1i32.wrapping_neg() as i64) {
                PyErr_SetString(
                    PyExc_ValueError,
                    c"fromhex() arg must contain an even number of hexadecimal digits".as_ptr(),
                );
            } else {
                PyErr_Format(
                    PyExc_ValueError,
                    c"non-hexadecimal number found in fromhex() arg at position %zd".as_ptr(),
                    invalid_char,
                );
            }
            _PyBytesWriter_Dealloc(core::ptr::addr_of_mut!(*__slate_slot_14));
        }
        if !(*__slate_slot_13).obj.is_null() {
            PyBuffer_Release(core::ptr::addr_of_mut!(*__slate_slot_13));
        }
        return core::ptr::null_mut::<_object>();
    }
}

// [clinic input]
// bytes.hex
//
//     sep: object = NULL
//         An optional single character or byte to separate hex bytes.
//
//     bytes_per_sep: int = 1
//         How many bytes between separators.  Positive values count from
//         the right, negative values count from the left.
//
// Create a string of hexadecimal numbers from a bytes object.
//
// Example:
// >>> value = b'\xb9\x01\xef'
// >>> value.hex()
// 'b901ef'
// >>> value.hex(':')
// 'b9:01:ef'
// >>> value.hex(':', 2)
// 'b9:01ef'
// >>> value.hex(':', -2)
// 'b901:ef'
// [clinic start generated code]
/// [clinic end generated code: output=1f134da504064139 input=67d7bb5f70f0d6f2]
fn bytes_hex_impl(
    self_: *mut __SlateRecord_bytesobject_h_15,
    sep: *mut _object,
    bytes_per_sep: i32,
) -> *mut _object {
    let argbuf: *const i8 = PyBytes_AS_STRING__2!(self_) as *const i8;
    let arglen: i64 = PyBytes_GET_SIZE__2!(self_);
    unsafe { _Py_strhex_with_sep(argbuf, arglen, sep, bytes_per_sep) }
}

#[unsafe(link_section = ".text.slate_distinct.bytesobject.bytes_getnewargs")]
extern "C-unwind" fn bytes_getnewargs(
    op: *mut _object,
    _unused_dummy: *mut _object,
) -> *mut _object {
    let v: *mut __SlateRecord_bytesobject_h_15 = _PyBytes_CAST!(op);
    unsafe {
        Py_BuildValue(
            c"(y#)".as_ptr(),
            (*v).ob_sval.as_mut_ptr() as *mut i8,
            Py_SIZE!(v),
        )
    }
}

/// sentinel
static mut bytes_methods: __SlateAlign16<[PyMethodDef; 45]> = __SlateAlign16([
    {
        let mut __t0: PyMethodDef = unsafe { core::mem::zeroed() };
        __t0.ml_name = (b"__getnewargs__\0".as_ptr() as *mut i8) as *const i8;
        __t0.ml_meth = Some(bytes_getnewargs);
        __t0.ml_flags = 4i32;
        __t0
    },
    {
        let mut __t1: PyMethodDef = unsafe { core::mem::zeroed() };
        __t1.ml_name = (b"__bytes__\0".as_ptr() as *mut i8) as *const i8;
        __t1.ml_meth = Some(bytes___bytes__);
        __t1.ml_flags = 4i32;
        __t1.ml_doc = unsafe { core::ptr::addr_of!(bytes___bytes____doc__.0) as *const i8 };
        __t1
    },
    {
        let mut __t2: PyMethodDef = unsafe { core::mem::zeroed() };
        __t2.ml_name = (b"capitalize\0".as_ptr() as *mut i8) as *const i8;
        __t2.ml_meth = Some(stringlib_capitalize);
        __t2.ml_flags = 4i32;
        __t2.ml_doc = unsafe { core::ptr::addr_of!(_Py_capitalize__doc__) as *const i8 };
        __t2
    },
    {
        let mut __t3: PyMethodDef = unsafe { core::mem::zeroed() };
        __t3.ml_name = (b"center\0".as_ptr() as *mut i8) as *const i8;
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
                >(Some(stringlib_center))
            })
        };
        __t3.ml_flags = 128i32;
        __t3.ml_doc = unsafe { core::ptr::addr_of!(stringlib_center__doc__.0) as *const i8 };
        __t3
    },
    {
        let mut __t4: PyMethodDef = unsafe { core::mem::zeroed() };
        __t4.ml_name = (b"count\0".as_ptr() as *mut i8) as *const i8;
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
                >(Some(bytes_count))
            })
        };
        __t4.ml_flags = 128i32;
        __t4.ml_doc = unsafe { core::ptr::addr_of!(bytes_count__doc__.0) as *const i8 };
        __t4
    },
    {
        let mut __t5: PyMethodDef = unsafe { core::mem::zeroed() };
        __t5.ml_name = (b"decode\0".as_ptr() as *mut i8) as *const i8;
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
                >(Some(bytes_decode))
            })
        };
        __t5.ml_flags = 130i32;
        __t5.ml_doc = unsafe { core::ptr::addr_of!(bytes_decode__doc__.0) as *const i8 };
        __t5
    },
    {
        let mut __t6: PyMethodDef = unsafe { core::mem::zeroed() };
        __t6.ml_name = (b"endswith\0".as_ptr() as *mut i8) as *const i8;
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
                >(Some(bytes_endswith))
            })
        };
        __t6.ml_flags = 128i32;
        __t6.ml_doc = unsafe { core::ptr::addr_of!(bytes_endswith__doc__.0) as *const i8 };
        __t6
    },
    {
        let mut __t7: PyMethodDef = unsafe { core::mem::zeroed() };
        __t7.ml_name = (b"expandtabs\0".as_ptr() as *mut i8) as *const i8;
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
                            *mut _object,
                        ) -> *mut _object,
                    >,
                    Option<unsafe extern "C-unwind" fn()>,
                >(Some(stringlib_expandtabs))
            })
        };
        __t7.ml_flags = 130i32;
        __t7.ml_doc = unsafe { core::ptr::addr_of!(stringlib_expandtabs__doc__.0) as *const i8 };
        __t7
    },
    {
        let mut __t8: PyMethodDef = unsafe { core::mem::zeroed() };
        __t8.ml_name = (b"find\0".as_ptr() as *mut i8) as *const i8;
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
                >(Some(bytes_find))
            })
        };
        __t8.ml_flags = 128i32;
        __t8.ml_doc = unsafe { core::ptr::addr_of!(bytes_find__doc__.0) as *const i8 };
        __t8
    },
    {
        let mut __t9: PyMethodDef = unsafe { core::mem::zeroed() };
        __t9.ml_name = (b"fromhex\0".as_ptr() as *mut i8) as *const i8;
        __t9.ml_meth = Some(bytes_fromhex);
        __t9.ml_flags = 24i32;
        __t9.ml_doc = unsafe { core::ptr::addr_of!(bytes_fromhex__doc__.0) as *const i8 };
        __t9
    },
    {
        let mut __t10: PyMethodDef = unsafe { core::mem::zeroed() };
        __t10.ml_name = (b"hex\0".as_ptr() as *mut i8) as *const i8;
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
                            *mut _object,
                        ) -> *mut _object,
                    >,
                    Option<unsafe extern "C-unwind" fn()>,
                >(Some(bytes_hex))
            })
        };
        __t10.ml_flags = 130i32;
        __t10.ml_doc = unsafe { core::ptr::addr_of!(bytes_hex__doc__.0) as *const i8 };
        __t10
    },
    {
        let mut __t11: PyMethodDef = unsafe { core::mem::zeroed() };
        __t11.ml_name = (b"index\0".as_ptr() as *mut i8) as *const i8;
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
                >(Some(bytes_index))
            })
        };
        __t11.ml_flags = 128i32;
        __t11.ml_doc = unsafe { core::ptr::addr_of!(bytes_index__doc__.0) as *const i8 };
        __t11
    },
    {
        let mut __t12: PyMethodDef = unsafe { core::mem::zeroed() };
        __t12.ml_name = (b"isalnum\0".as_ptr() as *mut i8) as *const i8;
        __t12.ml_meth = Some(stringlib_isalnum);
        __t12.ml_flags = 4i32;
        __t12.ml_doc = unsafe { core::ptr::addr_of!(_Py_isalnum__doc__) as *const i8 };
        __t12
    },
    {
        let mut __t13: PyMethodDef = unsafe { core::mem::zeroed() };
        __t13.ml_name = (b"isalpha\0".as_ptr() as *mut i8) as *const i8;
        __t13.ml_meth = Some(stringlib_isalpha);
        __t13.ml_flags = 4i32;
        __t13.ml_doc = unsafe { core::ptr::addr_of!(_Py_isalpha__doc__) as *const i8 };
        __t13
    },
    {
        let mut __t14: PyMethodDef = unsafe { core::mem::zeroed() };
        __t14.ml_name = (b"isascii\0".as_ptr() as *mut i8) as *const i8;
        __t14.ml_meth = Some(stringlib_isascii);
        __t14.ml_flags = 4i32;
        __t14.ml_doc = unsafe { core::ptr::addr_of!(_Py_isascii__doc__) as *const i8 };
        __t14
    },
    {
        let mut __t15: PyMethodDef = unsafe { core::mem::zeroed() };
        __t15.ml_name = (b"isdigit\0".as_ptr() as *mut i8) as *const i8;
        __t15.ml_meth = Some(stringlib_isdigit);
        __t15.ml_flags = 4i32;
        __t15.ml_doc = unsafe { core::ptr::addr_of!(_Py_isdigit__doc__) as *const i8 };
        __t15
    },
    {
        let mut __t16: PyMethodDef = unsafe { core::mem::zeroed() };
        __t16.ml_name = (b"islower\0".as_ptr() as *mut i8) as *const i8;
        __t16.ml_meth = Some(stringlib_islower);
        __t16.ml_flags = 4i32;
        __t16.ml_doc = unsafe { core::ptr::addr_of!(_Py_islower__doc__) as *const i8 };
        __t16
    },
    {
        let mut __t17: PyMethodDef = unsafe { core::mem::zeroed() };
        __t17.ml_name = (b"isspace\0".as_ptr() as *mut i8) as *const i8;
        __t17.ml_meth = Some(stringlib_isspace);
        __t17.ml_flags = 4i32;
        __t17.ml_doc = unsafe { core::ptr::addr_of!(_Py_isspace__doc__) as *const i8 };
        __t17
    },
    {
        let mut __t18: PyMethodDef = unsafe { core::mem::zeroed() };
        __t18.ml_name = (b"istitle\0".as_ptr() as *mut i8) as *const i8;
        __t18.ml_meth = Some(stringlib_istitle);
        __t18.ml_flags = 4i32;
        __t18.ml_doc = unsafe { core::ptr::addr_of!(_Py_istitle__doc__) as *const i8 };
        __t18
    },
    {
        let mut __t19: PyMethodDef = unsafe { core::mem::zeroed() };
        __t19.ml_name = (b"isupper\0".as_ptr() as *mut i8) as *const i8;
        __t19.ml_meth = Some(stringlib_isupper);
        __t19.ml_flags = 4i32;
        __t19.ml_doc = unsafe { core::ptr::addr_of!(_Py_isupper__doc__) as *const i8 };
        __t19
    },
    {
        let mut __t20: PyMethodDef = unsafe { core::mem::zeroed() };
        __t20.ml_name = (b"join\0".as_ptr() as *mut i8) as *const i8;
        __t20.ml_meth = Some(bytes_join);
        __t20.ml_flags = 8i32;
        __t20.ml_doc = unsafe { core::ptr::addr_of!(bytes_join__doc__.0) as *const i8 };
        __t20
    },
    {
        let mut __t21: PyMethodDef = unsafe { core::mem::zeroed() };
        __t21.ml_name = (b"ljust\0".as_ptr() as *mut i8) as *const i8;
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
                >(Some(stringlib_ljust))
            })
        };
        __t21.ml_flags = 128i32;
        __t21.ml_doc = unsafe { core::ptr::addr_of!(stringlib_ljust__doc__.0) as *const i8 };
        __t21
    },
    {
        let mut __t22: PyMethodDef = unsafe { core::mem::zeroed() };
        __t22.ml_name = (b"lower\0".as_ptr() as *mut i8) as *const i8;
        __t22.ml_meth = Some(stringlib_lower);
        __t22.ml_flags = 4i32;
        __t22.ml_doc = unsafe { core::ptr::addr_of!(_Py_lower__doc__) as *const i8 };
        __t22
    },
    {
        let mut __t23: PyMethodDef = unsafe { core::mem::zeroed() };
        __t23.ml_name = (b"lstrip\0".as_ptr() as *mut i8) as *const i8;
        __t23.ml_meth = unsafe {
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
                >(Some(bytes_lstrip))
            })
        };
        __t23.ml_flags = 128i32;
        __t23.ml_doc = unsafe { core::ptr::addr_of!(bytes_lstrip__doc__.0) as *const i8 };
        __t23
    },
    {
        let mut __t24: PyMethodDef = unsafe { core::mem::zeroed() };
        __t24.ml_name = (b"maketrans\0".as_ptr() as *mut i8) as *const i8;
        __t24.ml_meth = unsafe {
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
                >(Some(bytes_maketrans))
            })
        };
        __t24.ml_flags = 160i32;
        __t24.ml_doc = unsafe { core::ptr::addr_of!(bytes_maketrans__doc__.0) as *const i8 };
        __t24
    },
    {
        let mut __t25: PyMethodDef = unsafe { core::mem::zeroed() };
        __t25.ml_name = (b"partition\0".as_ptr() as *mut i8) as *const i8;
        __t25.ml_meth = Some(bytes_partition);
        __t25.ml_flags = 8i32;
        __t25.ml_doc = unsafe { core::ptr::addr_of!(bytes_partition__doc__.0) as *const i8 };
        __t25
    },
    {
        let mut __t26: PyMethodDef = unsafe { core::mem::zeroed() };
        __t26.ml_name = (b"replace\0".as_ptr() as *mut i8) as *const i8;
        __t26.ml_meth = unsafe {
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
                >(Some(bytes_replace))
            })
        };
        __t26.ml_flags = 128i32;
        __t26.ml_doc = unsafe { core::ptr::addr_of!(bytes_replace__doc__.0) as *const i8 };
        __t26
    },
    {
        let mut __t27: PyMethodDef = unsafe { core::mem::zeroed() };
        __t27.ml_name = (b"removeprefix\0".as_ptr() as *mut i8) as *const i8;
        __t27.ml_meth = Some(bytes_removeprefix);
        __t27.ml_flags = 8i32;
        __t27.ml_doc = unsafe { core::ptr::addr_of!(bytes_removeprefix__doc__.0) as *const i8 };
        __t27
    },
    {
        let mut __t28: PyMethodDef = unsafe { core::mem::zeroed() };
        __t28.ml_name = (b"removesuffix\0".as_ptr() as *mut i8) as *const i8;
        __t28.ml_meth = Some(bytes_removesuffix);
        __t28.ml_flags = 8i32;
        __t28.ml_doc = unsafe { core::ptr::addr_of!(bytes_removesuffix__doc__.0) as *const i8 };
        __t28
    },
    {
        let mut __t29: PyMethodDef = unsafe { core::mem::zeroed() };
        __t29.ml_name = (b"rfind\0".as_ptr() as *mut i8) as *const i8;
        __t29.ml_meth = unsafe {
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
                >(Some(bytes_rfind))
            })
        };
        __t29.ml_flags = 128i32;
        __t29.ml_doc = unsafe { core::ptr::addr_of!(bytes_rfind__doc__.0) as *const i8 };
        __t29
    },
    {
        let mut __t30: PyMethodDef = unsafe { core::mem::zeroed() };
        __t30.ml_name = (b"rindex\0".as_ptr() as *mut i8) as *const i8;
        __t30.ml_meth = unsafe {
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
                >(Some(bytes_rindex))
            })
        };
        __t30.ml_flags = 128i32;
        __t30.ml_doc = unsafe { core::ptr::addr_of!(bytes_rindex__doc__.0) as *const i8 };
        __t30
    },
    {
        let mut __t31: PyMethodDef = unsafe { core::mem::zeroed() };
        __t31.ml_name = (b"rjust\0".as_ptr() as *mut i8) as *const i8;
        __t31.ml_meth = unsafe {
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
                >(Some(stringlib_rjust))
            })
        };
        __t31.ml_flags = 128i32;
        __t31.ml_doc = unsafe { core::ptr::addr_of!(stringlib_rjust__doc__.0) as *const i8 };
        __t31
    },
    {
        let mut __t32: PyMethodDef = unsafe { core::mem::zeroed() };
        __t32.ml_name = (b"rpartition\0".as_ptr() as *mut i8) as *const i8;
        __t32.ml_meth = Some(bytes_rpartition);
        __t32.ml_flags = 8i32;
        __t32.ml_doc = unsafe { core::ptr::addr_of!(bytes_rpartition__doc__.0) as *const i8 };
        __t32
    },
    {
        let mut __t33: PyMethodDef = unsafe { core::mem::zeroed() };
        __t33.ml_name = (b"rsplit\0".as_ptr() as *mut i8) as *const i8;
        __t33.ml_meth = unsafe {
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
                >(Some(bytes_rsplit))
            })
        };
        __t33.ml_flags = 130i32;
        __t33.ml_doc = unsafe { core::ptr::addr_of!(bytes_rsplit__doc__.0) as *const i8 };
        __t33
    },
    {
        let mut __t34: PyMethodDef = unsafe { core::mem::zeroed() };
        __t34.ml_name = (b"rstrip\0".as_ptr() as *mut i8) as *const i8;
        __t34.ml_meth = unsafe {
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
                >(Some(bytes_rstrip))
            })
        };
        __t34.ml_flags = 128i32;
        __t34.ml_doc = unsafe { core::ptr::addr_of!(bytes_rstrip__doc__.0) as *const i8 };
        __t34
    },
    {
        let mut __t35: PyMethodDef = unsafe { core::mem::zeroed() };
        __t35.ml_name = (b"split\0".as_ptr() as *mut i8) as *const i8;
        __t35.ml_meth = unsafe {
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
                >(Some(bytes_split))
            })
        };
        __t35.ml_flags = 130i32;
        __t35.ml_doc = unsafe { core::ptr::addr_of!(bytes_split__doc__.0) as *const i8 };
        __t35
    },
    {
        let mut __t36: PyMethodDef = unsafe { core::mem::zeroed() };
        __t36.ml_name = (b"splitlines\0".as_ptr() as *mut i8) as *const i8;
        __t36.ml_meth = unsafe {
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
                >(Some(bytes_splitlines))
            })
        };
        __t36.ml_flags = 130i32;
        __t36.ml_doc = unsafe { core::ptr::addr_of!(bytes_splitlines__doc__.0) as *const i8 };
        __t36
    },
    {
        let mut __t37: PyMethodDef = unsafe { core::mem::zeroed() };
        __t37.ml_name = (b"startswith\0".as_ptr() as *mut i8) as *const i8;
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
                >(Some(bytes_startswith))
            })
        };
        __t37.ml_flags = 128i32;
        __t37.ml_doc = unsafe { core::ptr::addr_of!(bytes_startswith__doc__.0) as *const i8 };
        __t37
    },
    {
        let mut __t38: PyMethodDef = unsafe { core::mem::zeroed() };
        __t38.ml_name = (b"strip\0".as_ptr() as *mut i8) as *const i8;
        __t38.ml_meth = unsafe {
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
                >(Some(bytes_strip))
            })
        };
        __t38.ml_flags = 128i32;
        __t38.ml_doc = unsafe { core::ptr::addr_of!(bytes_strip__doc__.0) as *const i8 };
        __t38
    },
    {
        let mut __t39: PyMethodDef = unsafe { core::mem::zeroed() };
        __t39.ml_name = (b"swapcase\0".as_ptr() as *mut i8) as *const i8;
        __t39.ml_meth = Some(stringlib_swapcase);
        __t39.ml_flags = 4i32;
        __t39.ml_doc = unsafe { core::ptr::addr_of!(_Py_swapcase__doc__) as *const i8 };
        __t39
    },
    {
        let mut __t40: PyMethodDef = unsafe { core::mem::zeroed() };
        __t40.ml_name = (b"title\0".as_ptr() as *mut i8) as *const i8;
        __t40.ml_meth = Some(stringlib_title);
        __t40.ml_flags = 4i32;
        __t40.ml_doc = unsafe { core::ptr::addr_of!(_Py_title__doc__) as *const i8 };
        __t40
    },
    {
        let mut __t41: PyMethodDef = unsafe { core::mem::zeroed() };
        __t41.ml_name = (b"translate\0".as_ptr() as *mut i8) as *const i8;
        __t41.ml_meth = unsafe {
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
                >(Some(bytes_translate))
            })
        };
        __t41.ml_flags = 130i32;
        __t41.ml_doc = unsafe { core::ptr::addr_of!(bytes_translate__doc__.0) as *const i8 };
        __t41
    },
    {
        let mut __t42: PyMethodDef = unsafe { core::mem::zeroed() };
        __t42.ml_name = (b"upper\0".as_ptr() as *mut i8) as *const i8;
        __t42.ml_meth = Some(stringlib_upper);
        __t42.ml_flags = 4i32;
        __t42.ml_doc = unsafe { core::ptr::addr_of!(_Py_upper__doc__) as *const i8 };
        __t42
    },
    {
        let mut __t43: PyMethodDef = unsafe { core::mem::zeroed() };
        __t43.ml_name = (b"zfill\0".as_ptr() as *mut i8) as *const i8;
        __t43.ml_meth = Some(stringlib_zfill);
        __t43.ml_flags = 8i32;
        __t43.ml_doc = unsafe { core::ptr::addr_of!(stringlib_zfill__doc__.0) as *const i8 };
        __t43
    },
    {
        let mut __t44: PyMethodDef = unsafe { core::mem::zeroed() };
        __t44.ml_name = core::ptr::null::<i8>();
        __t44.ml_meth = None;
        __t44
    },
]);

#[unsafe(link_section = ".text.slate_distinct.bytesobject.bytes_mod")]
extern "C-unwind" fn bytes_mod(self_: *mut _object, arg: *mut _object) -> *mut _object {
    if PyBytes_Check!(self_) == 0 {
        return unsafe { core::ptr::addr_of_mut!(_Py_NotImplementedStruct) };
    }
    _PyBytes_FormatEx(
        PyBytes_AS_STRING!(self_) as *const i8,
        PyBytes_GET_SIZE!(self_),
        arg,
        0i32,
    )
}

/// nb_add
/// nb_subtract
/// nb_multiply
/// nb_remainder
static mut bytes_as_number: __SlateRecord_object_h_106 = {
    let mut __t0: __SlateRecord_object_h_106 = unsafe { core::mem::zeroed() };
    __t0.nb_add = None;
    __t0.nb_subtract = None;
    __t0.nb_multiply = None;
    __t0.nb_remainder = Some(bytes_mod);
    __t0
};

// [clinic input]
// @classmethod
// bytes.__new__ as bytes_new
//
//     source as x: object = NULL
//     encoding: str = NULL
//     errors: str = NULL
//
// [clinic start generated code]
/// [clinic end generated code: output=1e0c471be311a425 input=f0a966d19b7262b4]
fn bytes_new_impl(
    r#type: *mut _typeobject,
    x: *mut _object,
    encoding: *const i8,
    errors: *const i8,
) -> *mut _object {
    let mut bytes: *mut _object = unsafe { core::mem::zeroed() };
    let mut size: i64 = 0;
    if x.is_null() {
        if !encoding.is_null() || !errors.is_null() {
            unsafe {
                PyErr_SetString(
                    PyExc_TypeError,
                    (if !encoding.is_null() {
                        c"encoding without a string argument".as_ptr() as *mut i8
                    } else {
                        c"errors without a string argument".as_ptr() as *mut i8
                    }) as *const i8,
                )
            };
            return core::ptr::null_mut::<_object>();
        }
        bytes = PyBytes_FromStringAndSize(core::ptr::null::<i8>(), 0i64);
    } else {
        if !encoding.is_null() {
            // Encode via the codec registry
            if PyUnicode_Check!(x) == 0 {
                unsafe {
                    PyErr_SetString(
                        PyExc_TypeError,
                        c"encoding without a string argument".as_ptr(),
                    )
                };
                return core::ptr::null_mut::<_object>();
            }
            bytes = unsafe { PyUnicode_AsEncodedString(x, encoding, errors) };
        } else {
            if !errors.is_null() {
                unsafe {
                    PyErr_SetString(
                        PyExc_TypeError,
                        (if PyUnicode_Check!(x) != 0 {
                            c"string argument without an encoding".as_ptr() as *mut i8
                        } else {
                            c"errors without a string argument".as_ptr() as *mut i8
                        }) as *const i8,
                    )
                };
                return core::ptr::null_mut::<_object>();
            } else {
                let __v0: *mut _object = unsafe {
                    _PyObject_LookupSpecial(
                        x,
                        core::ptr::addr_of_mut!(
                            _PyRuntime
                                .static_objects
                                .singletons
                                .strings
                                .identifiers
                                ._py___bytes__
                                ._ascii
                                .ob_base
                        ),
                    )
                };
                let func: *mut _object = __v0;
                if !__v0.is_null() {
                    bytes = _PyObject_CallNoArgs(func);
                    Py_DECREF!(func);
                    if bytes.is_null() {
                        return core::ptr::null_mut::<_object>();
                    }
                    if PyBytes_Check!(bytes) == 0 {
                        unsafe {
                            PyErr_Format(
                                PyExc_TypeError,
                                c"__bytes__ returned non-bytes (type %.200s)".as_ptr(),
                                (*Py_TYPE!(bytes)).tp_name,
                            )
                        };
                        Py_DECREF!(bytes);
                        return core::ptr::null_mut::<_object>();
                    }
                } else {
                    if !unsafe { PyErr_Occurred() }.is_null() {
                        return core::ptr::null_mut::<_object>();
                    } else {
                        if PyUnicode_Check!(x) != 0 {
                            unsafe {
                                PyErr_SetString(
                                    PyExc_TypeError,
                                    c"string argument without an encoding".as_ptr(),
                                )
                            };
                            return core::ptr::null_mut::<_object>();
                        } else {
                            if _PyIndex_Check(x) != 0 {
                                size = unsafe { PyNumber_AsSsize_t(x, PyExc_OverflowError) };
                                let __v1: bool = size == (1i32.wrapping_neg() as i64)
                                    && !unsafe { PyErr_Occurred() }.is_null();
                                if __v1 {
                                    if (unsafe { PyErr_ExceptionMatches(PyExc_TypeError) }) == 0 {
                                        return core::ptr::null_mut::<_object>();
                                    }
                                    unsafe { PyErr_Clear() }; // fall through
                                    bytes = PyBytes_FromObject(x);
                                } else {
                                    if size < 0 {
                                        unsafe {
                                            PyErr_SetString(
                                                PyExc_ValueError,
                                                c"negative count".as_ptr(),
                                            )
                                        };
                                        return core::ptr::null_mut::<_object>();
                                    }
                                    bytes = _PyBytes_FromSize(size, 1i32);
                                }
                            } else {
                                bytes = PyBytes_FromObject(x);
                            }
                        }
                    }
                }
            }
        }
    }
    // We'd like to call PyObject_Bytes here, but we need to check for an
    // integer argument before deferring to PyBytes_FromObject, something
    // PyObject_Bytes doesn't do.
    // Is it an integer?
    if !bytes.is_null() && r#type != unsafe { core::ptr::addr_of_mut!(PyBytes_Type) } {
        Py_SETREF!(bytes, bytes_subtype_new(r#type, bytes));
    }
    bytes
}

fn _PyBytes_FromBuffer(x: *mut _object) -> *mut _object {
    let mut view: core::mem::MaybeUninit<__SlateRecord_pybuffer_h_33> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_0: *mut __SlateRecord_pybuffer_h_33 =
        core::ptr::addr_of_mut!(view) as *mut __SlateRecord_pybuffer_h_33;
    let mut new: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        if PyObject_GetBuffer(x, core::ptr::addr_of_mut!(*__slate_slot_0), 284i32) < 0 {
            return core::ptr::null_mut::<_object>();
        } else {
            new = PyBytes_FromStringAndSize(core::ptr::null::<i8>(), (*__slate_slot_0).len);
            if !new.is_null() {
                if PyBuffer_ToContiguous(
                    ((*(new as *mut __SlateRecord_bytesobject_h_15))
                        .ob_sval
                        .as_mut_ptr() as *mut i8) as *mut (),
                    core::ptr::addr_of_mut!(*__slate_slot_0) as *const __SlateRecord_pybuffer_h_33,
                    (*__slate_slot_0).len,
                    67i8,
                ) >= 0
                {
                    PyBuffer_Release(core::ptr::addr_of_mut!(*__slate_slot_0));
                    return new;
                }
            }
            Py_XDECREF!(new);
            PyBuffer_Release(core::ptr::addr_of_mut!(*__slate_slot_0));
            return core::ptr::null_mut::<_object>();
        }
    }
}

fn _PyBytes_FromList(x: *mut _object) -> *mut _object {
    let mut __v4: bool = unsafe { core::mem::zeroed() };
    let mut item: *mut _object = unsafe { core::mem::zeroed() };
    let mut writer: core::mem::MaybeUninit<__SlateRecord_pycore_bytesobject_h_89> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_6: *mut __SlateRecord_pycore_bytesobject_h_89 =
        core::ptr::addr_of_mut!(writer) as *mut __SlateRecord_pycore_bytesobject_h_89;
    let mut str: *mut i8 = unsafe { core::mem::zeroed() };
    let mut value: i64 = unsafe { core::mem::zeroed() };
    let mut size: i64 = unsafe { core::mem::zeroed() };
    let mut i: i64 = unsafe { core::mem::zeroed() };
    unsafe {
        size = PyList_GET_SIZE!(x);
        _PyBytesWriter_Init(core::ptr::addr_of_mut!(*__slate_slot_6));
        str = _PyBytesWriter_Alloc(core::ptr::addr_of_mut!(*__slate_slot_6), size) as *mut i8;
        if str.is_null() {
            return core::ptr::null_mut::<_object>();
        } else {
            (*__slate_slot_6).overallocate = 1;
            size = (*__slate_slot_6).allocated;
            i = 0;
            '__join_0: {
                '__join_8: {
                    while i < PyList_GET_SIZE!(x) {
                        item = _PyList_GetItemRef(x as *mut __SlateRecord_listobject_h_22, i);
                        if item.is_null() {
                            break '__join_0;
                        } else {
                            value = PyNumber_AsSsize_t(item, core::ptr::null_mut::<_object>());
                            Py_DECREF!(item);
                            if value == (1i32.wrapping_neg() as i64) {
                                __v4 = !PyErr_Occurred().is_null();
                            } else {
                                __v4 = false;
                            }
                            if __v4 {
                                break '__join_0;
                            } else {
                                if value < 0 || value >= 256 {
                                    break '__join_8;
                                } else {
                                    if i >= size {
                                        str = _PyBytesWriter_Resize(
                                            core::ptr::addr_of_mut!(*__slate_slot_6),
                                            str as *mut (),
                                            size.wrapping_add(1),
                                        ) as *mut i8;
                                        if str.is_null() {
                                            return core::ptr::null_mut::<_object>();
                                        } else {
                                            size = (*__slate_slot_6).allocated;
                                        }
                                    }
                                    let __v3: *mut i8 = str;
                                    str = __v3.wrapping_offset(1 as isize);
                                    *__v3 = value as i8;
                                    i = i.wrapping_add(1);
                                }
                            }
                        }
                    }
                    return _PyBytesWriter_Finish(
                        core::ptr::addr_of_mut!(*__slate_slot_6),
                        str as *mut (),
                    );
                }
                PyErr_SetString(PyExc_ValueError, c"bytes must be in range(0, 256)".as_ptr());
            }
            _PyBytesWriter_Dealloc(core::ptr::addr_of_mut!(*__slate_slot_6));
            return core::ptr::null_mut::<_object>();
        }
    }
}

fn _PyBytes_FromTuple(x: *mut _object) -> *mut _object {
    let mut __v4: bool = unsafe { core::mem::zeroed() };
    let mut str: *mut i8 = unsafe { core::mem::zeroed() };
    let mut value: i64 = unsafe { core::mem::zeroed() };
    let mut size: i64 = unsafe { core::mem::zeroed() };
    let mut i: i64 = unsafe { core::mem::zeroed() };
    let mut bytes: *mut _object = unsafe { core::mem::zeroed() };
    unsafe {
        size = PyTuple_GET_SIZE!(x);
        bytes = PyBytes_FromStringAndSize(core::ptr::null::<i8>(), size);
        if bytes.is_null() {
            return core::ptr::null_mut::<_object>();
        } else {
            str = (*(bytes as *mut __SlateRecord_bytesobject_h_15))
                .ob_sval
                .as_mut_ptr() as *mut i8;
            i = 0;
            '__join_0: {
                '__join_4: {
                    while i < size {
                        let item: *mut _object = PyTuple_GET_ITEM!(x, i);
                        value = PyNumber_AsSsize_t(item, core::ptr::null_mut::<_object>());
                        if value == (1i32.wrapping_neg() as i64) {
                            __v4 = !PyErr_Occurred().is_null();
                        } else {
                            __v4 = false;
                        }
                        if __v4 {
                            break '__join_0;
                        } else {
                            if value < 0 || value >= 256 {
                                break '__join_4;
                            } else {
                                let __v3: *mut i8 = str;
                                str = __v3.wrapping_offset(1 as isize);
                                *__v3 = value as i8;
                                i = i.wrapping_add(1);
                            }
                        }
                    }
                    return bytes;
                }
                PyErr_SetString(PyExc_ValueError, c"bytes must be in range(0, 256)".as_ptr());
            }
            Py_DECREF!(bytes);
            return core::ptr::null_mut::<_object>();
        }
    }
}

fn _PyBytes_FromIterator(it: *mut _object, x: *mut _object) -> *mut _object {
    let mut __v4: bool = unsafe { core::mem::zeroed() };
    let mut value: i64 = unsafe { core::mem::zeroed() };
    let mut item: *mut _object = unsafe { core::mem::zeroed() };
    let mut __v7: bool = unsafe { core::mem::zeroed() };
    let mut writer: core::mem::MaybeUninit<__SlateRecord_pycore_bytesobject_h_89> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_8: *mut __SlateRecord_pycore_bytesobject_h_89 =
        core::ptr::addr_of_mut!(writer) as *mut __SlateRecord_pycore_bytesobject_h_89;
    let mut size: i64 = unsafe { core::mem::zeroed() };
    let mut i: i64 = unsafe { core::mem::zeroed() };
    let mut str: *mut i8 = unsafe { core::mem::zeroed() };
    unsafe {
        // For iterator version, create a bytes object and resize as needed
        size = PyObject_LengthHint(x, 64i64);
        if size == (1i32.wrapping_neg() as i64) {
            __v7 = !PyErr_Occurred().is_null();
        } else {
            __v7 = false;
        }
        if __v7 {
            return core::ptr::null_mut::<_object>();
        } else {
            _PyBytesWriter_Init(core::ptr::addr_of_mut!(*__slate_slot_8));
            str = _PyBytesWriter_Alloc(core::ptr::addr_of_mut!(*__slate_slot_8), size) as *mut i8;
            if str.is_null() {
                return core::ptr::null_mut::<_object>();
            } else {
                (*__slate_slot_8).overallocate = 1;
                size = (*__slate_slot_8).allocated;
                // Run the iterator to exhaustion
                i = 0;
                '__join_0: {
                    '__join_13: {
                        '__join_7: {
                            loop {
                                // Get the next item
                                item = PyIter_Next(it);
                                if item.is_null() {
                                    break '__join_13;
                                } else {
                                    // Interpret it as an int (__index__)
                                    value =
                                        PyNumber_AsSsize_t(item, core::ptr::null_mut::<_object>());
                                    Py_DECREF!(item);
                                    if value == (1i32.wrapping_neg() as i64) {
                                        __v4 = !PyErr_Occurred().is_null();
                                    } else {
                                        __v4 = false;
                                    }
                                    if __v4 {
                                        break '__join_0;
                                    } else {
                                        // Range check
                                        if value < 0 || value >= 256 {
                                            break '__join_7;
                                        } else {
                                            // Append the byte
                                            if i >= size {
                                                str = _PyBytesWriter_Resize(
                                                    core::ptr::addr_of_mut!(*__slate_slot_8),
                                                    str as *mut (),
                                                    size.wrapping_add(1),
                                                )
                                                    as *mut i8;
                                                if str.is_null() {
                                                    break;
                                                } else {
                                                    size = (*__slate_slot_8).allocated;
                                                }
                                            }
                                            let __v3: *mut i8 = str;
                                            str = __v3.wrapping_offset(1 as isize);
                                            *__v3 = value as i8;
                                            i = i.wrapping_add(1);
                                        }
                                    }
                                }
                            }
                            return core::ptr::null_mut::<_object>();
                        }
                        PyErr_SetString(
                            PyExc_ValueError,
                            c"bytes must be in range(0, 256)".as_ptr(),
                        );
                        break '__join_0;
                    }
                    if PyErr_Occurred().is_null() {
                        return _PyBytesWriter_Finish(
                            core::ptr::addr_of_mut!(*__slate_slot_8),
                            str as *mut (),
                        );
                    }
                }
                _PyBytesWriter_Dealloc(core::ptr::addr_of_mut!(*__slate_slot_8));
                return core::ptr::null_mut::<_object>();
            }
        }
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyBytes_FromObject(x: *mut _object) -> *mut _object {
    if x.is_null() {
        PyErr_BadInternalCall__137!();
        return core::ptr::null_mut::<_object>();
    }
    if PyBytes_CheckExact!(x) != 0 {
        return Py_NewRef!(x);
    }
    // Use the modern buffer interface
    if (unsafe { PyObject_CheckBuffer(x) }) != 0 {
        return _PyBytes_FromBuffer(x);
    }
    if PyList_CheckExact!(x) != 0 {
        return _PyBytes_FromList(x);
    }
    if PyTuple_CheckExact!(x) != 0 {
        return _PyBytes_FromTuple(x);
    }
    if PyUnicode_Check!(x) == 0 {
        let it: *mut _object = unsafe { PyObject_GetIter(x) };
        if !it.is_null() {
            let result: *mut _object = _PyBytes_FromIterator(it, x);
            Py_DECREF!(it);
            return result;
        }
        if (unsafe { PyErr_ExceptionMatches(PyExc_TypeError) }) == 0 {
            return core::ptr::null_mut::<_object>();
        }
    }
    unsafe {
        PyErr_Format(
            PyExc_TypeError,
            c"cannot convert '%.200s' object to bytes".as_ptr(),
            (*Py_TYPE!(x)).tp_name,
        )
    };
    core::ptr::null_mut::<_object>()
}

/// This allocator is needed for subclasses don't want to use __new__.
/// See https://github.com/python/cpython/issues/91020#issuecomment-1096793239
///
/// This allocator will be removed when ob_shash is removed.
#[unsafe(link_section = ".text.slate_distinct.bytesobject.bytes_alloc")]
extern "C-unwind" fn bytes_alloc(self_: *mut _typeobject, nitems: i64) -> *mut _object {
    let obj: *mut __SlateRecord_bytesobject_h_15 =
        (unsafe { PyType_GenericAlloc(self_, nitems) }) as *mut __SlateRecord_bytesobject_h_15;
    if obj.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    set_ob_shash(obj, 1i32.wrapping_neg() as i64);
    obj as *mut _object
}

fn bytes_subtype_new(r#type: *mut _typeobject, tmp: *mut _object) -> *mut _object {
    let n: i64 = PyBytes_GET_SIZE!(tmp);
    let pnew: *mut _object = unsafe { unsafe { (*r#type).tp_alloc }.unwrap()(r#type, n) };
    if !pnew.is_null() {
        unsafe {
            core::ptr::copy_nonoverlapping(
                ((PyBytes_AS_STRING!(tmp) as *const ()) as *const core::ffi::c_void) as *const u8,
                ((PyBytes_AS_STRING!(pnew) as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                ((n.wrapping_add(1) as u64) as usize) as usize,
            )
        };
        set_ob_shash(
            pnew as *mut __SlateRecord_bytesobject_h_15,
            get_ob_shash(tmp as *mut __SlateRecord_bytesobject_h_15),
        );
    }
    pnew
}

static bytes_doc: __SlateAlign16<[i8; 459]> = __SlateAlign16([
    98i8, 121i8, 116i8, 101i8, 115i8, 40i8, 105i8, 116i8, 101i8, 114i8, 97i8, 98i8, 108i8, 101i8,
    95i8, 111i8, 102i8, 95i8, 105i8, 110i8, 116i8, 115i8, 41i8, 32i8, 45i8, 62i8, 32i8, 98i8,
    121i8, 116i8, 101i8, 115i8, 10i8, 98i8, 121i8, 116i8, 101i8, 115i8, 40i8, 115i8, 116i8, 114i8,
    105i8, 110i8, 103i8, 44i8, 32i8, 101i8, 110i8, 99i8, 111i8, 100i8, 105i8, 110i8, 103i8, 91i8,
    44i8, 32i8, 101i8, 114i8, 114i8, 111i8, 114i8, 115i8, 93i8, 41i8, 32i8, 45i8, 62i8, 32i8, 98i8,
    121i8, 116i8, 101i8, 115i8, 10i8, 98i8, 121i8, 116i8, 101i8, 115i8, 40i8, 98i8, 121i8, 116i8,
    101i8, 115i8, 95i8, 111i8, 114i8, 95i8, 98i8, 117i8, 102i8, 102i8, 101i8, 114i8, 41i8, 32i8,
    45i8, 62i8, 32i8, 105i8, 109i8, 109i8, 117i8, 116i8, 97i8, 98i8, 108i8, 101i8, 32i8, 99i8,
    111i8, 112i8, 121i8, 32i8, 111i8, 102i8, 32i8, 98i8, 121i8, 116i8, 101i8, 115i8, 95i8, 111i8,
    114i8, 95i8, 98i8, 117i8, 102i8, 102i8, 101i8, 114i8, 10i8, 98i8, 121i8, 116i8, 101i8, 115i8,
    40i8, 105i8, 110i8, 116i8, 41i8, 32i8, 45i8, 62i8, 32i8, 98i8, 121i8, 116i8, 101i8, 115i8,
    32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 32i8, 111i8, 102i8, 32i8, 115i8, 105i8, 122i8,
    101i8, 32i8, 103i8, 105i8, 118i8, 101i8, 110i8, 32i8, 98i8, 121i8, 32i8, 116i8, 104i8, 101i8,
    32i8, 112i8, 97i8, 114i8, 97i8, 109i8, 101i8, 116i8, 101i8, 114i8, 32i8, 105i8, 110i8, 105i8,
    116i8, 105i8, 97i8, 108i8, 105i8, 122i8, 101i8, 100i8, 32i8, 119i8, 105i8, 116i8, 104i8, 32i8,
    110i8, 117i8, 108i8, 108i8, 32i8, 98i8, 121i8, 116i8, 101i8, 115i8, 10i8, 98i8, 121i8, 116i8,
    101i8, 115i8, 40i8, 41i8, 32i8, 45i8, 62i8, 32i8, 101i8, 109i8, 112i8, 116i8, 121i8, 32i8,
    98i8, 121i8, 116i8, 101i8, 115i8, 32i8, 111i8, 98i8, 106i8, 101i8, 99i8, 116i8, 10i8, 10i8,
    67i8, 111i8, 110i8, 115i8, 116i8, 114i8, 117i8, 99i8, 116i8, 32i8, 97i8, 110i8, 32i8, 105i8,
    109i8, 109i8, 117i8, 116i8, 97i8, 98i8, 108i8, 101i8, 32i8, 97i8, 114i8, 114i8, 97i8, 121i8,
    32i8, 111i8, 102i8, 32i8, 98i8, 121i8, 116i8, 101i8, 115i8, 32i8, 102i8, 114i8, 111i8, 109i8,
    58i8, 10i8, 32i8, 32i8, 45i8, 32i8, 97i8, 110i8, 32i8, 105i8, 116i8, 101i8, 114i8, 97i8, 98i8,
    108i8, 101i8, 32i8, 121i8, 105i8, 101i8, 108i8, 100i8, 105i8, 110i8, 103i8, 32i8, 105i8, 110i8,
    116i8, 101i8, 103i8, 101i8, 114i8, 115i8, 32i8, 105i8, 110i8, 32i8, 114i8, 97i8, 110i8, 103i8,
    101i8, 40i8, 50i8, 53i8, 54i8, 41i8, 10i8, 32i8, 32i8, 45i8, 32i8, 97i8, 32i8, 116i8, 101i8,
    120i8, 116i8, 32i8, 115i8, 116i8, 114i8, 105i8, 110i8, 103i8, 32i8, 101i8, 110i8, 99i8, 111i8,
    100i8, 101i8, 100i8, 32i8, 117i8, 115i8, 105i8, 110i8, 103i8, 32i8, 116i8, 104i8, 101i8, 32i8,
    115i8, 112i8, 101i8, 99i8, 105i8, 102i8, 105i8, 101i8, 100i8, 32i8, 101i8, 110i8, 99i8, 111i8,
    100i8, 105i8, 110i8, 103i8, 10i8, 32i8, 32i8, 45i8, 32i8, 97i8, 110i8, 121i8, 32i8, 111i8,
    98i8, 106i8, 101i8, 99i8, 116i8, 32i8, 105i8, 109i8, 112i8, 108i8, 101i8, 109i8, 101i8, 110i8,
    116i8, 105i8, 110i8, 103i8, 32i8, 116i8, 104i8, 101i8, 32i8, 98i8, 117i8, 102i8, 102i8, 101i8,
    114i8, 32i8, 65i8, 80i8, 73i8, 46i8, 10i8, 32i8, 32i8, 45i8, 32i8, 97i8, 110i8, 32i8, 105i8,
    110i8, 116i8, 101i8, 103i8, 101i8, 114i8, 0i8,
]);

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
extern "C-unwind" fn PyBytes_Concat(pv: *mut *mut _object, w: *mut _object) {
    let mut wb: core::mem::MaybeUninit<__SlateRecord_pybuffer_h_33> =
        core::mem::MaybeUninit::uninit();
    let __slate_slot_0: *mut __SlateRecord_pybuffer_h_33 =
        core::ptr::addr_of_mut!(wb) as *mut __SlateRecord_pybuffer_h_33;
    let mut __v3: bool = unsafe { core::mem::zeroed() };
    unsafe {
        if unsafe { *pv }.is_null() {
            return;
        } else {
            if w.is_null() {
                Py_CLEAR!(*pv);
                return;
            } else {
                if _PyObject_IsUniquelyReferenced(*pv) != 0 {
                    __v3 = PyBytes_CheckExact!(unsafe { *pv }) != 0;
                } else {
                    __v3 = false;
                }
                if __v3 {
                    if PyObject_GetBuffer(w, core::ptr::addr_of_mut!(*__slate_slot_0), 0i32) != 0 {
                        PyErr_Format(
                            PyExc_TypeError,
                            c"can't concat %.100s to %.100s".as_ptr(),
                            (*Py_TYPE!(w)).tp_name,
                            (*Py_TYPE!(unsafe { *pv })).tp_name,
                        );
                        Py_CLEAR!(*pv);
                        return;
                    } else {
                        // Only one reference, so we can resize in place
                        let oldsize: i64 = PyBytes_GET_SIZE!(unsafe { *pv });
                        if oldsize > 9223372036854775807i64.wrapping_sub((*__slate_slot_0).len) {
                            PyErr_NoMemory();
                        } else {
                            if _PyBytes_Resize(pv, oldsize.wrapping_add((*__slate_slot_0).len)) >= 0
                            {
                                core::ptr::copy_nonoverlapping(
                                    (((*__slate_slot_0).buf as *const ())
                                        as *const core::ffi::c_void)
                                        as *const u8,
                                    ((PyBytes_AS_STRING!(unsafe { *pv })
                                        .wrapping_offset(oldsize as isize)
                                        as *mut ())
                                        as *mut core::ffi::c_void)
                                        as *mut u8,
                                    (((*__slate_slot_0).len as u64) as usize) as usize,
                                );
                                PyBuffer_Release(core::ptr::addr_of_mut!(*__slate_slot_0));
                                return;
                            }
                        }
                        PyBuffer_Release(core::ptr::addr_of_mut!(*__slate_slot_0));
                        Py_CLEAR!(*pv);
                        return;
                    }
                } else {
                    // Multiple references, need to create new object
                    let v: *mut _object = bytes_concat(*pv, w);
                    Py_SETREF__2!(*pv, v);
                }
            }
        }
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn PyBytes_ConcatAndDel(pv: *mut *mut _object, w: *mut _object) {
    PyBytes_Concat(pv, w);
    Py_XDECREF!(w);
}

// The following function breaks the notion that bytes are immutable:
// it changes the size of a bytes object.  You can think of it
// as creating a new bytes object and destroying the old one, only
// more efficiently.
// Note that if there's not enough memory to resize the bytes object, the
// original bytes object at *pv is deallocated, *pv is set to NULL, an "out of
// memory" exception is set, and -1 is returned.  Else (on success) 0 is
// returned, and the value in *pv may or may not be the same as on input.
// As always, an extra byte is allocated for a trailing \0 byte (newsize
// does *not* include that), and a trailing \0 byte is stored.
#[unsafe(no_mangle)]
extern "C-unwind" fn _PyBytes_Resize(pv: *mut *mut _object, newsize: i64) -> i32 {
    let v: *mut _object = unsafe { *pv };
    if PyBytes_Check!(v) == 0 || newsize < 0 {
        unsafe {
            *pv = core::ptr::null_mut::<_object>();
        }
        Py_DECREF!(v);
        PyErr_BadInternalCall__5!();
        return 1i32.wrapping_neg();
    }
    let oldsize: i64 = PyBytes_GET_SIZE!(v);
    if oldsize == newsize {
        // return early if newsize equals to v->ob_size
        return 0;
    }
    if oldsize == 0 {
        unsafe {
            *pv = _PyBytes_FromSize(newsize, 0i32);
        }
        Py_DECREF!(v);
        return if unsafe { *pv }.is_null() {
            1i32.wrapping_neg()
        } else {
            0
        };
    }
    if newsize == 0 {
        unsafe {
            *pv = bytes_get_empty();
        }
        Py_DECREF!(v);
        return 0;
    }
    if _PyObject_IsUniquelyReferenced(v) == 0 {
        // Allocate and then copy so we don't get a shared immortal
        //
        // one-character singleton!
        unsafe {
            *pv = _PyBytes_FromSize(newsize, 0i32);
        }
        if !unsafe { *pv }.is_null() {
            unsafe {
                core::ptr::copy_nonoverlapping(
                    ((PyBytes_AS_STRING!(v) as *const ()) as *const core::ffi::c_void) as *const u8,
                    ((PyBytes_AS_STRING!(unsafe { *pv }) as *mut ()) as *mut core::ffi::c_void)
                        as *mut u8,
                    ((Py_MIN!(oldsize, newsize) as u64) as usize) as usize,
                )
            };
        }
        Py_DECREF!(v);
        return if unsafe { *pv }.is_null() {
            1i32.wrapping_neg()
        } else {
            0
        };
    }
    _PyReftracerTrack!(v, 1i32);
    unsafe {
        *pv = PyObject_Realloc(
            v as *mut (),
            (core::mem::offset_of!(__SlateRecord_bytesobject_h_15, ob_sval) as u64)
                .wrapping_add(1)
                .wrapping_add(newsize as u64),
        ) as *mut _object;
    }
    if unsafe { *pv }.is_null() {
        unsafe { PyObject_Free(v as *mut ()) };
        unsafe { PyErr_NoMemory() };
        return 1i32.wrapping_neg();
    }
    unsafe { _Py_NewReferenceNoTotal(*pv) };
    let sv: *mut __SlateRecord_bytesobject_h_15 =
        (unsafe { *pv }) as *mut __SlateRecord_bytesobject_h_15;
    Py_SET_SIZE!(sv, newsize);
    unsafe {
        *((*sv).ob_sval.as_mut_ptr() as *mut i8).wrapping_offset(newsize as isize) = 0;
    }
    set_ob_shash(sv, 1i32.wrapping_neg() as i64); // invalidate cached hash value
    0
}

// Bytes Iterator
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_bytesobject_c_3286 {
    ob_base: _object,
    it_index: i64,
    /// Set to NULL when iterator is exhausted
    it_seq: *mut __SlateRecord_bytesobject_h_15,
}

#[unsafe(link_section = ".text.slate_distinct.bytesobject.striter_dealloc")]
extern "C-unwind" fn striter_dealloc(op: *mut _object) {
    let it: *mut __SlateRecord_bytesobject_c_3286 = _striterobject_CAST!(op);
    _PyObject_GC_UNTRACK__2!(it);
    Py_XDECREF__2!(unsafe { (*it).it_seq });
    unsafe { PyObject_GC_Del(it as *mut ()) };
}

#[unsafe(link_section = ".text.slate_distinct.bytesobject.striter_traverse")]
extern "C-unwind" fn striter_traverse(
    op: *mut _object,
    mut visit: Option<unsafe extern "C-unwind" fn(*mut _object, *mut ()) -> i32>,
    arg: *mut (),
) -> i32 {
    let it: *mut __SlateRecord_bytesobject_c_3286 = _striterobject_CAST!(op);
    Py_VISIT__22!(unsafe { (*it).it_seq }, visit, arg);
    0
}

#[unsafe(link_section = ".text.slate_distinct.bytesobject.striter_next")]
extern "C-unwind" fn striter_next(op: *mut _object) -> *mut _object {
    let it: *mut __SlateRecord_bytesobject_c_3286 = _striterobject_CAST!(op);
    let seq: *mut __SlateRecord_bytesobject_h_15 = unsafe { (*it).it_seq };
    if seq.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    if (unsafe { (*it).it_index }) < PyBytes_GET_SIZE__2!(seq) {
        let __v1: i64 = unsafe { (*it).it_index };
        unsafe {
            (*it).it_index = __v1.wrapping_add(1);
        }
        return _PyLong_FromUnsignedChar(
            (unsafe { *((*seq).ob_sval.as_mut_ptr() as *mut i8).wrapping_offset(__v1 as isize) })
                as u8,
        );
    }
    unsafe {
        (*it).it_seq = core::ptr::null_mut::<__SlateRecord_bytesobject_h_15>();
    }
    Py_DECREF__2!(seq);
    core::ptr::null_mut::<_object>()
}

#[unsafe(link_section = ".text.slate_distinct.bytesobject.striter_len")]
extern "C-unwind" fn striter_len(op: *mut _object, _unused_ignored: *mut _object) -> *mut _object {
    let it: *mut __SlateRecord_bytesobject_c_3286 = _striterobject_CAST!(op);
    let mut len: i64 = 0;
    if !unsafe { (*it).it_seq }.is_null() {
        len = PyBytes_GET_SIZE__2!(unsafe { (*it).it_seq }).wrapping_sub(unsafe { (*it).it_index });
    }
    unsafe { PyLong_FromSsize_t(len) }
}

static length_hint_doc: __SlateAlign16<[i8; 55]> = __SlateAlign16([
    80i8, 114i8, 105i8, 118i8, 97i8, 116i8, 101i8, 32i8, 109i8, 101i8, 116i8, 104i8, 111i8, 100i8,
    32i8, 114i8, 101i8, 116i8, 117i8, 114i8, 110i8, 105i8, 110i8, 103i8, 32i8, 97i8, 110i8, 32i8,
    101i8, 115i8, 116i8, 105i8, 109i8, 97i8, 116i8, 101i8, 32i8, 111i8, 102i8, 32i8, 108i8, 101i8,
    110i8, 40i8, 108i8, 105i8, 115i8, 116i8, 40i8, 105i8, 116i8, 41i8, 41i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bytesobject.striter_reduce")]
extern "C-unwind" fn striter_reduce(
    op: *mut _object,
    _unused_ignored: *mut _object,
) -> *mut _object {
    let iter: *mut _object = unsafe {
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
    // _PyEval_GetBuiltin can invoke arbitrary code,
    // call must be before access of iterator pointers.
    // see issue #101765
    let it: *mut __SlateRecord_bytesobject_c_3286 = _striterobject_CAST!(op);
    if !unsafe { (*it).it_seq }.is_null() {
        return unsafe { Py_BuildValue(c"N(O)n".as_ptr(), iter, (*it).it_seq, (*it).it_index) };
    } else {
        return unsafe { Py_BuildValue(c"N(())".as_ptr(), iter) };
    }
}

static reduce_doc: __SlateAlign16<[i8; 39]> = __SlateAlign16([
    82i8, 101i8, 116i8, 117i8, 114i8, 110i8, 32i8, 115i8, 116i8, 97i8, 116i8, 101i8, 32i8, 105i8,
    110i8, 102i8, 111i8, 114i8, 109i8, 97i8, 116i8, 105i8, 111i8, 110i8, 32i8, 102i8, 111i8, 114i8,
    32i8, 112i8, 105i8, 99i8, 107i8, 108i8, 105i8, 110i8, 103i8, 46i8, 0i8,
]);

#[unsafe(link_section = ".text.slate_distinct.bytesobject.striter_setstate")]
extern "C-unwind" fn striter_setstate(op: *mut _object, state: *mut _object) -> *mut _object {
    let mut index: i64 = unsafe { PyLong_AsSsize_t(state) };
    let __v0: bool =
        index == (1i32.wrapping_neg() as i64) && !unsafe { PyErr_Occurred() }.is_null();
    if __v0 {
        return core::ptr::null_mut::<_object>();
    }
    let it: *mut __SlateRecord_bytesobject_c_3286 = _striterobject_CAST!(op);
    if !unsafe { (*it).it_seq }.is_null() {
        if index < 0 {
            index = 0;
        } else {
            if index > PyBytes_GET_SIZE__2!(unsafe { (*it).it_seq }) {
                index = PyBytes_GET_SIZE__2!(unsafe { (*it).it_seq });
            }
        }
        // iterator exhausted
        unsafe {
            (*it).it_index = index;
        }
    }
    unsafe { core::ptr::addr_of_mut!(_Py_NoneStruct) }
}

static setstate_doc: __SlateAlign16<[i8; 38]> = __SlateAlign16([
    83i8, 101i8, 116i8, 32i8, 115i8, 116i8, 97i8, 116i8, 101i8, 32i8, 105i8, 110i8, 102i8, 111i8,
    114i8, 109i8, 97i8, 116i8, 105i8, 111i8, 110i8, 32i8, 102i8, 111i8, 114i8, 32i8, 117i8, 110i8,
    112i8, 105i8, 99i8, 107i8, 108i8, 105i8, 110i8, 103i8, 46i8, 0i8,
]);

/// sentinel
static mut striter_methods: __SlateAlign16<[PyMethodDef; 4]> = __SlateAlign16([
    {
        let mut __t0: PyMethodDef = unsafe { core::mem::zeroed() };
        __t0.ml_name = (b"__length_hint__\0".as_ptr() as *mut i8) as *const i8;
        __t0.ml_meth = Some(striter_len);
        __t0.ml_flags = 4i32;
        __t0.ml_doc = unsafe { core::ptr::addr_of!(length_hint_doc.0) as *const i8 };
        __t0
    },
    {
        let mut __t1: PyMethodDef = unsafe { core::mem::zeroed() };
        __t1.ml_name = (b"__reduce__\0".as_ptr() as *mut i8) as *const i8;
        __t1.ml_meth = Some(striter_reduce);
        __t1.ml_flags = 4i32;
        __t1.ml_doc = unsafe { core::ptr::addr_of!(reduce_doc.0) as *const i8 };
        __t1
    },
    {
        let mut __t2: PyMethodDef = unsafe { core::mem::zeroed() };
        __t2.ml_name = (b"__setstate__\0".as_ptr() as *mut i8) as *const i8;
        __t2.ml_meth = Some(striter_setstate);
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
#[unsafe(link_section = ".text.slate_distinct.bytesobject.bytes_iter")]
extern "C-unwind" fn bytes_iter(seq: *mut _object) -> *mut _object {
    if PyBytes_Check!(seq) == 0 {
        PyErr_BadInternalCall__60!();
        return core::ptr::null_mut::<_object>();
    }
    let it: *mut __SlateRecord_bytesobject_c_3286 =
        PyObject_GC_New!(__SlateRecord_bytesobject_c_3286, unsafe {
            core::ptr::addr_of_mut!(PyBytesIter_Type)
        });
    if it.is_null() {
        return core::ptr::null_mut::<_object>();
    }
    unsafe {
        (*it).it_index = 0;
    }
    unsafe {
        (*it).it_seq = Py_NewRef!(seq) as *mut __SlateRecord_bytesobject_h_15;
    }
    _PyObject_GC_TRACK!(it);
    it as *mut _object
}

// _PyBytesWriter API
// On Linux, overallocate by 25% is the best factor
#[unsafe(no_mangle)]
extern "C-unwind" fn _PyBytesWriter_Init(writer: *mut __SlateRecord_pycore_bytesobject_h_89) {
    // Set all attributes before small_buffer to 0
    unsafe {
        core::ptr::write_bytes(
            ((writer as *mut ()) as *mut core::ffi::c_void) as *mut u8,
            (0i32 as i32) as u8,
            ((core::mem::offset_of!(__SlateRecord_pycore_bytesobject_h_89, small_buffer) as u64)
                as usize) as usize,
        )
    };
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyBytesWriter_Dealloc(writer: *mut __SlateRecord_pycore_bytesobject_h_89) {
    Py_CLEAR!((*writer).buffer);
}

fn _PyBytesWriter_AsString(writer: *mut __SlateRecord_pycore_bytesobject_h_89) -> *mut i8 {
    if (unsafe { (*writer).use_small_buffer }) != 0 {
        return unsafe { (*writer).small_buffer.as_mut_ptr() as *mut i8 };
    } else {
        if (unsafe { (*writer).use_bytearray }) != 0 {
            return PyByteArray_AS_STRING__2!(unsafe { (*writer).buffer });
        } else {
            return PyBytes_AS_STRING!(unsafe { (*writer).buffer });
        }
    }
}

/// Forward declaration
fn _PyBytesWriter_GetSize(writer: *mut __SlateRecord_pycore_bytesobject_h_89, str: *mut i8) -> i64 {
    let start: *const i8 = _PyBytesWriter_AsString(writer) as *const i8;
    (unsafe { str.offset_from(start as *mut i8) }) as i64
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyBytesWriter_Resize(
    writer: *mut __SlateRecord_pycore_bytesobject_h_89,
    mut str: *mut (),
    size: i64,
) -> *mut () {
    let mut dest: *mut i8 = unsafe { core::mem::zeroed() };
    let mut pos: i64 = unsafe { core::mem::zeroed() };
    let mut allocated: i64 = unsafe { core::mem::zeroed() };
    unsafe {
        allocated = size;
        if (*writer).overallocate != 0
            && allocated <= 9223372036854775807i64.wrapping_sub(allocated / 4)
        {
            // overallocate to limit the number of realloc()
            allocated = allocated.wrapping_add(allocated / 4);
        }
        '__join_0: {
            pos = _PyBytesWriter_GetSize(writer, str as *mut i8);
            if (*writer).use_small_buffer == 0 {
                if (*writer).use_bytearray != 0 {
                    if PyByteArray_Resize((*writer).buffer, allocated) != 0 {
                        break '__join_0;
                    }
                // writer->allocated can be smaller than writer->buffer->ob_alloc,
                // but we cannot use ob_alloc because bytes may need to be moved
                // to use the whole buffer. bytearray uses an internal optimization
                // to avoid moving or copying bytes when bytes are removed at the
                // beginning (ex: del bytearray[:1]).
                } else {
                    if _PyBytes_Resize(core::ptr::addr_of_mut!((*writer).buffer), allocated) != 0 {
                        break '__join_0;
                    }
                }
            } else {
                // convert from stack buffer to bytes object buffer
                if (*writer).use_bytearray != 0 {
                    (*writer).buffer =
                        PyByteArray_FromStringAndSize(core::ptr::null::<i8>(), allocated);
                } else {
                    (*writer).buffer =
                        PyBytes_FromStringAndSize(core::ptr::null::<i8>(), allocated);
                }
                if unsafe { (*writer).buffer }.is_null() {
                    break '__join_0;
                } else {
                    if pos != 0 {
                        if (*writer).use_bytearray != 0 {
                            dest = PyByteArray_AS_STRING__2!(unsafe { (*writer).buffer });
                        } else {
                            dest = PyBytes_AS_STRING!(unsafe { (*writer).buffer });
                        }
                        core::ptr::copy_nonoverlapping(
                            ((((*writer).small_buffer.as_mut_ptr() as *mut i8) as *const ())
                                as *const core::ffi::c_void)
                                as *const u8,
                            ((dest as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                            ((pos as u64) as usize) as usize,
                        );
                    }
                    (*writer).use_small_buffer = 0;
                }
            }
            (*writer).allocated = allocated;
            str = _PyBytesWriter_AsString(writer).wrapping_offset(pos as isize) as *mut ();
            return str;
        }
        _PyBytesWriter_Dealloc(writer);
        return core::ptr::null_mut::<()>();
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyBytesWriter_Prepare(
    writer: *mut __SlateRecord_pycore_bytesobject_h_89,
    mut str: *mut (),
    size: i64,
) -> *mut () {
    let mut new_min_size: i64 = 0;
    if size == 0 {
        // nothing to do
        return str;
    }
    if (unsafe { (*writer).min_size }) > 9223372036854775807i64.wrapping_sub(size) {
        unsafe { PyErr_NoMemory() };
        _PyBytesWriter_Dealloc(writer);
        return core::ptr::null_mut::<()>();
    }
    new_min_size = unsafe { (*writer).min_size }.wrapping_add(size);
    if new_min_size > unsafe { (*writer).allocated } {
        str = _PyBytesWriter_Resize(writer, str, new_min_size);
    }
    unsafe {
        (*writer).min_size = new_min_size;
    }
    str
}

/// Allocate the buffer to write size bytes.
/// Return the pointer to the beginning of buffer data.
/// Raise an exception and return NULL on error.
#[unsafe(no_mangle)]
extern "C-unwind" fn _PyBytesWriter_Alloc(
    writer: *mut __SlateRecord_pycore_bytesobject_h_89,
    size: i64,
) -> *mut () {
    // ensure that _PyBytesWriter_Alloc() is only called once
    unsafe {
        (*writer).use_small_buffer = 1;
    }
    unsafe {
        (*writer).allocated = core::mem::size_of::<[i8; 512]>() as i64;
    }
    _PyBytesWriter_Prepare(
        writer,
        (unsafe { (*writer).small_buffer.as_mut_ptr() as *mut i8 }) as *mut (),
        size,
    )
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyBytesWriter_Finish(
    writer: *mut __SlateRecord_pycore_bytesobject_h_89,
    str: *mut (),
) -> *mut _object {
    let mut size: i64 = 0;
    let mut result: *mut _object = unsafe { core::mem::zeroed() };
    size = _PyBytesWriter_GetSize(writer, str as *mut i8);
    if size == 0 && (unsafe { (*writer).use_bytearray }) == 0 {
        Py_CLEAR!((*writer).buffer);
        // Get the empty byte string singleton
        result = PyBytes_FromStringAndSize(core::ptr::null::<i8>(), 0i64);
    } else {
        if (unsafe { (*writer).use_small_buffer }) != 0 {
            if (unsafe { (*writer).use_bytearray }) != 0 {
                result = unsafe {
                    PyByteArray_FromStringAndSize(
                        ((*writer).small_buffer.as_mut_ptr() as *mut i8) as *const i8,
                        size,
                    )
                };
            } else {
                result = PyBytes_FromStringAndSize(
                    (unsafe { (*writer).small_buffer.as_mut_ptr() as *mut i8 }) as *const i8,
                    size,
                );
            }
        } else {
            result = unsafe { (*writer).buffer };
            unsafe {
                (*writer).buffer = core::ptr::null_mut::<_object>();
            }
            if size != unsafe { (*writer).allocated } {
                if (unsafe { (*writer).use_bytearray }) != 0 {
                    if (unsafe { PyByteArray_Resize(result, size) }) != 0 {
                        Py_DECREF!(result);
                        return core::ptr::null_mut::<_object>();
                    }
                } else {
                    if _PyBytes_Resize(core::ptr::addr_of_mut!(result), size) != 0 {
                        return core::ptr::null_mut::<_object>();
                    }
                }
            }
        }
    }
    result
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyBytesWriter_WriteBytes(
    writer: *mut __SlateRecord_pycore_bytesobject_h_89,
    ptr: *mut (),
    bytes: *const (),
    size: i64,
) -> *mut () {
    let mut str: *mut i8 = ptr as *mut i8;
    str = _PyBytesWriter_Prepare(writer, str as *mut (), size) as *mut i8;
    if str.is_null() {
        return core::ptr::null_mut::<()>();
    }
    unsafe {
        core::ptr::copy_nonoverlapping(
            (bytes as *const core::ffi::c_void) as *const u8,
            ((str as *mut ()) as *mut core::ffi::c_void) as *mut u8,
            ((size as u64) as usize) as usize,
        )
    };
    let __v0: *mut i8 = str;
    str = __v0.wrapping_offset(size as isize);
    str as *mut ()
}

#[unsafe(no_mangle)]
extern "C-unwind" fn _PyBytes_Repeat(dest: *mut i8, len_dest: i64, src: *const i8, len_src: i64) {
    if len_dest == 0 {
        return;
    }
    if len_src == 1 {
        unsafe {
            core::ptr::write_bytes(
                ((dest as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                ((*src.wrapping_offset(0 as isize) as i32) as i32) as u8,
                ((len_dest as u64) as usize) as usize,
            )
        };
    } else {
        if src != (dest as *const i8) {
            unsafe {
                core::ptr::copy_nonoverlapping(
                    ((src as *const ()) as *const core::ffi::c_void) as *const u8,
                    ((dest as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                    ((len_src as u64) as usize) as usize,
                )
            };
        }
        let mut copied: i64 = len_src;
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
}
